extern crate minifb;
extern crate rand;
extern crate num_cpus;
extern crate rayon;
extern crate glam;


pub mod math;
pub mod controls;
pub mod scene;

use std::time::{SystemTime, Duration};
use std::sync::mpsc::*;
use std::sync::mpsc::channel;
use std::slice::ChunksMut;
use std::f32;


use rand::{thread_rng, Rng};

use minifb::{Key, WindowOptions, Window};
use rayon::prelude::*;
use glam::Vec3;


use self::math::{Ray};
use self::controls::Camera;
use self::scene::*;

const WIDTH: usize = 480;
const HEIGHT: usize = 320;

pub struct RayTraceThreadConfig<'a> {
    rtpc: Vec<RayTracePixelConfig<'a>>,
    sender: Sender<PixelColor>,
    pixel_subset: &'a mut [u32],
    number_of_samples: u32
}
unsafe impl<'a> Send for RayTraceThreadConfig<'a>{}
unsafe impl<'a> Sync for RayTraceThreadConfig<'a>{}

pub struct RayTracePixelConfig<'a> {
    world: &'a Hitable,
    material_library: &'a MaterialLibrary,
    width: usize,
    height: usize,
    x: u32,
    y: u32,
    index: usize
}
unsafe impl<'a> Send for RayTracePixelConfig<'a>{}
unsafe impl<'a> Sync for RayTracePixelConfig<'a>{}

#[derive(Clone, Copy)]
pub struct PixelColor {
    pub color: u32,
    pub index: usize
}
unsafe impl Send for PixelColor {}

pub fn render_thread(thread_config: &mut RayTraceThreadConfig) {
    let mut count = 0;
    let camera = Camera::new(90.0, WIDTH as f32 / HEIGHT as f32);
    let mut rng = thread_rng();


    for rtpc in &thread_config.rtpc {
        let mut return_color : Vec3 = Vec3::new(0.0, 0.0, 0.0);
        let mut raycastresult = [RayCastResult::new(); 4];//I would like to lift this out eventually, have all of them be allocated for an entire row

        for i in 0..thread_config.number_of_samples {
            let x = rtpc.x as f32;
            let y = rtpc.y as f32;

            let u = (x + rng.gen_range(0.0, 1.0)) / rtpc.width as f32;
            let v = (y + rng.gen_range(0.0, 1.0)) / rtpc.height  as f32;
            let r = camera.get_ray(u, v);
            cast_ray(&r, rtpc.world, &rtpc.material_library, 0, &mut raycastresult[i as usize]);
        }

        for raycast_result in raycastresult.iter() {
            let mut ray_color = Vec3::new(1.0, 1.0, 1.0);
            for i in 0..raycast_result.number_of_hits {
                let hitresult = raycast_result.hits[i];
                let material = rtpc.material_library.checkout_material(hitresult.material);//hitresult.material);

                match material {
                    Some(mat) => {
                        ray_color *= mat.color(&hitresult);                
                    },
                    None => {
                        panic!("{}", "Checked out bad material");
                    }
                }
            }
            return_color += ray_color;
        }

        return_color.set_x(return_color.x() / thread_config.number_of_samples as f32);
        return_color.set_y(return_color.y() / thread_config.number_of_samples as f32);
        return_color.set_z(return_color.z() / thread_config.number_of_samples as f32);

        let red   = (255.0 * return_color.x()).min(255.0).max(0.0);
        let green = (255.0 * return_color.y()).min(255.0).max(0.0);
        let blue  = (255.0 * return_color.z()).min(255.0).max(0.0);

        let final_color = ((red as u32) << 16 | (green as u32) << 8 | (blue as u32)).into();
        thread_config.pixel_subset[count] = final_color;
        count += 1;
    }
}

#[inline]
pub fn cast_ray(ray: &Ray, world: &Hitable, material_library: &MaterialLibrary, depth: i32, raycastresult: &mut RayCastResult) {
    //it is not important in what order I solve my child rays, just that I solve them
    if depth >= 10 {
        return;
    }

    let mut record : &mut HitRecord = &mut raycastresult.hits[raycastresult.number_of_hits];
    if world.hit(ray, 0.001, f32::MAX, record) == true {
        let material = material_library.checkout_material(record.material);
        match &material {
            Some(mat) => {
                let scatter_hit = mat.scatter(ray, &record);
                if scatter_hit.result == true {
                    raycastresult.number_of_hits += 1;
                    cast_ray(&scatter_hit.scattered, world, material_library, depth + 1, raycastresult);
                }
                return;
            },
            None => {
                panic!("Tried to unwrap a nonesistant material");
            }
        }
    }

    //A REALLY HACKY WAY TO DOING THE SKY
    record.material = 6;
    raycastresult.number_of_hits += 1;
}

#[inline]
pub fn color(ray: &Ray, world: &Hitable, material_library: &MaterialLibrary, depth: i32) -> Vec3 {

    if depth > 10 {
        return Vec3::new(0.0, 0.0, 0.0);
    }

    let mut record : HitRecord = HitRecord::empty();
    if world.hit(ray, 0.001, f32::MAX, &mut record) == true {
        let material = material_library.checkout_material(record.material);
        match &material {
            Some(mat) => {
                let scatter_hit = mat.scatter(ray, &record);
                if scatter_hit.result == true {
                    let col = color(&scatter_hit.scattered, world, material_library, depth + 1);
                    return Vec3::new(col.x() * scatter_hit.attenuation.x(), col.y() * scatter_hit.attenuation.y(), col.z() * scatter_hit.attenuation.z());
                }
                return Vec3::new(0.0, 0.0, 0.0);
            },
            None => {
                panic!("Tried to unwrap a nonesistant material");
            }
        }
    }

    let unit_direction = ray.get_direction().normalize();
    let t = 0.5 * (unit_direction.y() + 1.0);
    return Vec3::new(1.0f32, 1.0f32, 1.0f32) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t;//background color, I Think?
}

fn main() {
    let test = Vec3::new(0.0, 0.0, 0.0) * 1.0;
    let mut material_library = MaterialLibrary::new();
    let lambert_1_id = material_library.add_new(Box::new(Lambertian::new(Vec3::new(0.1, 0.7, 0.3))));
    let lambert_2_id = material_library.add_new(Box::new(Lambertian::new(Vec3::new(0.8, 0.1, 0.0))));
    let metal_1_id = material_library.add_new(Box::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.3)));
    let dielectric_1_id = material_library.add_new(Box::new(Deilectric::new(1.5)));
    let sky_material = material_library.add_new(Box::new(Sky::new()));

    let world_list : Vec<Box<Hitable  + Send>> = vec![
        Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, lambert_1_id)),
        Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, lambert_2_id)),
        Box::new(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, metal_1_id)),
        Box::new(Sphere::new(Vec3::new(-1.0, 0.0,-1.0), -0.45, dielectric_1_id)),
    ];
    let world = HitableList::new(world_list);


    let mut buffer: Vec<u32> = vec![0;WIDTH * HEIGHT];
    
    let mut window = Window::new("Test - ESC to exit", WIDTH, HEIGHT, WindowOptions::default()).unwrap_or_else(|e|{
        panic!("{}", e);
    });

    let (tx, _rx) : (Sender<PixelColor>, Receiver<PixelColor>) = channel();
    let mut frame_count = 0;
    while window.is_open() && !window.is_key_down(Key::Escape) {

        {
            let mut chunks = buffer.chunks_mut(WIDTH);
            let mut frame_count = 0;
        
            let mut ray_trace_pixel_configs = vec![];
            let mut count = 0;

            for y in (0..HEIGHT).rev() {
                let mut row = vec![];
                for x in 0..WIDTH {
                    let rtpc = RayTracePixelConfig {
                        world: &world,
                        material_library: &material_library,
                        width: WIDTH,
                        height: HEIGHT,
                        x: x as u32,
                        y: y as u32,
                        index: count
                    };
                    row.push(rtpc);
                    count += 1;
                }

                ray_trace_pixel_configs.push(
                    RayTraceThreadConfig{
                        rtpc: row,
                        number_of_samples: 4,
                        sender: tx.clone(),
                        pixel_subset: chunks.next().unwrap()
                    }
                );
            }
            let _ : Vec<_> = ray_trace_pixel_configs.par_iter_mut().map(|rtpc|render_thread(rtpc)).collect();
        }
        frame_count += 1;
        window.update_with_buffer(&buffer).unwrap();
      window.set_title(frame_count.to_string().as_str());
  //      println!("{:?}", elasped.subsec_nanos());
    }
}