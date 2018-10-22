extern crate minifb;
extern crate rand;
extern crate num_cpus;
extern crate rayon;
extern crate euclid;

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
use euclid::Vector3D;

use self::math::{Ray};
use self::controls::Camera;
use self::scene::*;

const WIDTH: usize = 320;
const HEIGHT: usize = 240;

pub struct RayTraceThreadConfig<'a> {
    rtpc: Vec<RayTracePixelConfig<'a>>,
    sender: Sender<PixelColor>
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
    number_of_samples: u32,
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
//unsafe impl Sync for PixelColor {}

pub fn render_simgle_pixel_thread(rtpc: &RayTracePixelConfig) -> PixelColor {

    let mut rng = thread_rng();

    let camera = Camera::new(90.0, WIDTH as f32 / HEIGHT as f32);

    let mut return_color :Vector3D<f32> =Vector3D::new(0.0, 0.0, 0.0);

    for _ in 0..rtpc.number_of_samples {
        let x = rtpc.x as f32;
        let y = rtpc.y as f32;

        let u = (x + rng.gen_range(0.0, 1.0)) / rtpc.width as f32;
        let v = (y + rng.gen_range(0.0, 1.0)) / rtpc.height  as f32;
        let r = camera.get_ray(u, v);
        return_color += color(&r, rtpc.world, &rtpc.material_library, 0);
    }

    return_color.x /= rtpc.number_of_samples as f32;
    return_color.y /= rtpc.number_of_samples as f32;
    return_color.z /= rtpc.number_of_samples as f32;

    let red   = (255.0 * return_color.x).min(255.0).max(0.0);
    let green = (255.0 * return_color.y).min(255.0).max(0.0);
    let blue  = (255.0 * return_color.z).min(255.0).max(0.0);

    let final_color = ((red as u32) << 16 | (green as u32) << 8 | (blue as u32)).into();
    return PixelColor{color: final_color, index: rtpc.index};
}

pub fn render_thread(thread_config: &RayTraceThreadConfig) {

    for rtpc in &thread_config.rtpc {
        let mut rng = thread_rng();

        let camera = Camera::new(90.0, WIDTH as f32 / HEIGHT as f32);

        let mut return_color :Vector3D<f32> =Vector3D::new(0.0, 0.0, 0.0);

        for _ in 0..rtpc.number_of_samples {
            let x = rtpc.x as f32;
            let y = rtpc.y as f32;

            let u = (x + rng.gen_range(0.0, 1.0)) / rtpc.width as f32;
            let v = (y + rng.gen_range(0.0, 1.0)) / rtpc.height  as f32;
            let r = camera.get_ray(u, v);
            return_color += color(&r, rtpc.world, &rtpc.material_library, 0);
        }

        return_color.x /= rtpc.number_of_samples as f32;
        return_color.y /= rtpc.number_of_samples as f32;
        return_color.z /= rtpc.number_of_samples as f32;

        let red   = (255.0 * return_color.x).min(255.0).max(0.0);
        let green = (255.0 * return_color.y).min(255.0).max(0.0);
        let blue  = (255.0 * return_color.z).min(255.0).max(0.0);

        let final_color = ((red as u32) << 16 | (green as u32) << 8 | (blue as u32)).into();
        let _ = thread_config.sender.send(PixelColor{color: final_color, index: rtpc.index});
    }
}

#[inline]
pub fn color(ray: &Ray, world: &Hitable, material_library: &MaterialLibrary, depth: i32) ->Vector3D<f32> {

    let mut record : HitRecord = HitRecord::empty();
    if depth > 10 {
        return Vector3D::new(0.0, 0.0, 0.0);
    }

    if world.hit(ray, 0.001, f32::MAX, &mut record) == true {
        let material = material_library.checkout_material(record.material);
        match &material {
            Some(mat) => {
                let scatter_hit = mat.scatter(ray, &record);
                if scatter_hit.result == true {
                    let col = color(&scatter_hit.scattered, world, material_library, depth + 1);
                    return Vector3D::new(col.x*scatter_hit.attenuation.x, col.y * scatter_hit.attenuation.y, col.z * scatter_hit.attenuation.z);
                }
                return Vector3D::new(0.0, 0.0, 0.0);
            },
            None => {
                panic!("Tried to unwrap a nonesistant material");
            }
        }
    }
    let unit_direction = ray.get_direction().normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    return Vector3D::new(1.0f32, 1.0f32, 1.0f32) * (1.0 - t) + Vector3D::new(0.5, 0.7, 1.0) * t;
}

fn main() {
    let test = Vector3D::new(0.0, 0.0, 0.0) * 1.0;
    let mut material_library = MaterialLibrary::new();
    let lambert_1_id = material_library.add_new(Box::new(Lambertian::new(Vector3D::new(0.8, 0.3, 0.3))));
    let lambert_2_id = material_library.add_new(Box::new(Lambertian::new(Vector3D::new(0.8, 0.8, 0.0))));
    let metal_1_id = material_library.add_new(Box::new(Metal::new(Vector3D::new(0.8, 0.6, 0.2), 0.3)));
    let dielectric_1_id = material_library.add_new(Box::new(Deilectric::new(1.5)));

    let world_list : Vec<Box<Hitable  + Send>> = vec![
        Box::new(Sphere::new(Vector3D::new(0.0, 0.0, -1.0), 0.5, lambert_1_id)),
        Box::new(Sphere::new(Vector3D::new(0.0, -100.5, -1.0), 100.0, lambert_2_id)),
        Box::new(Sphere::new(Vector3D::new(1.0, 0.0, -1.0), 0.5, metal_1_id)),
        Box::new(Sphere::new(Vector3D::new(-1.0, 0.0,-1.0), -0.45, dielectric_1_id)),
    ];
    let world = HitableList::new(world_list);


    let mut buffer: Vec<u32> = vec![0;WIDTH * HEIGHT];
    let mut chunks = buffer.chunks_mut(WIDTH);
    
    let mut window = Window::new("Test - ESC to exit", WIDTH, HEIGHT, WindowOptions::default()).unwrap_or_else(|e|{
        panic!("{}", e);
    });
//    window.
    let mut ray_trace_pixel_configs = vec![];
    let mut count = 0;

    let (tx, rx) : (Sender<PixelColor>, Receiver<PixelColor>) = channel();
    let mut frame_count = 0;
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
                number_of_samples: 4,
                index: count
            };
            row.push(rtpc);
            count += 1;
        }

        ray_trace_pixel_configs.push(
            RayTraceThreadConfig{
                rtpc: row,
                sender: tx.clone()
            }
        );
    }
    let mut returned;
    while window.is_open() && !window.is_key_down(Key::Escape){
        let start = SystemTime::now();
        returned = 0;
        
        let _ : Vec<_> = ray_trace_pixel_configs.par_iter().map(|rtpc|render_thread(&rtpc)).collect();
        while returned != count {
            let new_pixel = rx.recv().unwrap();
            buffer[new_pixel.index] = new_pixel.color;
            returned+=1;
        }
        /*
        par_iter.sort_by(|a, b|{return a.index.cmp(&b.index)});
        let mut buffer_as_iter = buffer.iter_mut();

        for p in par_iter.iter() {
           *buffer_as_iter.next().unwrap() = p.color;  
        }
        */

        frame_count+=1;
        window.update_with_buffer(&buffer).unwrap();
        let elasped = start.duration_since(start).expect("Dealing with result of duration_since()");
//        window.set_title().to_string().as_str());
  //      println!("{:?}", elasped.subsec_nanos());
    }
}