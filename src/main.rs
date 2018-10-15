#[macro_use]
extern crate minifb;
extern crate nalgebra as na;
extern crate rand;
extern crate num_cpus;

use minifb::{Key, WindowOptions, Window};


use crate::rand::{thread_rng, Rng};

pub mod math;
pub mod controls;
pub mod scene;


use std::f32;
use std::time::{Duration, Instant};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc::channel;
use std::sync::mpsc;
use std::sync::Mutex;
use std::thread;
use std::sync::Arc;

use self::na::Vector3;

use self::math::Ray;
use self::controls::Camera;
use self::scene::*;

const WIDTH: usize = 320;
const HEIGHT: usize = 288;


pub struct RayTracePixelConfig {
    width: usize,
    height: usize,
    x: u32,
    y: u32,
    number_of_samples: u32,
    index: usize
}


#[derive(Clone, Copy)]
pub struct PixelColor {
    pub color: u32,
    pub index: usize
}
unsafe impl Send for PixelColor {}
//unsafe impl Sync for PixelColor {}

pub fn render_thread(world: Arc<Hitable>, rx:Receiver<RayTracePixelConfig>, tx: Sender<PixelColor>) {
    let mut rng = thread_rng();

    let mut material_library = MaterialLibrary::new();
    let lambert_1_id = material_library.add_new(Box::new(Lambertian::new(Vector3::new(0.8, 0.3, 0.3))));
    let lambert_2_id = material_library.add_new(Box::new(Lambertian::new(Vector3::new(0.8, 0.8, 0.0))));
    let metal_1_id = material_library.add_new(Box::new(Metal::new(Vector3::new(0.8, 0.6, 0.2), 0.3)));
    let dielectric_1_id = material_library.add_new(Box::new(Deilectric::new(1.5)));

    let camera = Camera::new(90.0, WIDTH as f32 / HEIGHT as f32);

    loop {
        let recv = rx.recv();
        match recv {
            Ok(val) => {

                let mut return_color = Vector3::new(0.0, 0.0, 0.0);

                for _ in 0..val.number_of_samples {
                    let x = val.x as f32;
                    let y = val.y as f32;

                    let u = (x + rng.gen_range(0.0, 1.0)) / val.width as f32;
                    let v = (y + rng.gen_range(0.0, 1.0)) / val.height  as f32;
                    let r = camera.get_ray(u, v);
                    return_color += color(&r, &world, &material_library, 0);
                }

                let red   = (255.0 * return_color.x).min(255.0).max(0.0);
                let green = (255.0 * return_color.y).min(255.0).max(0.0);
                let blue  = (255.0 * return_color.z).min(255.0).max(0.0);

                let final_color = ((red as u32) << 16 | (green as u32) << 8 | (blue as u32)).into();
                tx.send(PixelColor{color: final_color, index: val.index});
            },
            Err(er) => {
                break;
            }
        }
    }

}

#[inline]
pub fn color(ray: &Ray, world: &Hitable, material_library: &MaterialLibrary, depth: i32) -> Vector3<f32> {

    let mut record : HitRecord = HitRecord::empty();
    if depth > 50 {
        return Vector3::new(0.0, 0.0, 0.0);
    }

    if world.hit(ray, 0.001, f32::MAX, &mut record) == true {
        let material = material_library.checkout_material(record.material);
        match &material {
            Some(mat) => {
                let scatter_hit = mat.scatter(ray, &record);
                if scatter_hit.result == true {
                    let col = color(&scatter_hit.scattered, world, material_library, depth + 1);
                    return Vector3::new(col.x*scatter_hit.attenuation.x, col.y * scatter_hit.attenuation.y, col.z * scatter_hit.attenuation.z);
                }
                return Vector3::new(0.0, 0.0, 0.0);
            },
            None => {
                panic!("Tried to unwrap a nonesistant material");
            }
        }
    }
    let unit_direction = ray.get_direction().normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    return (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0)
}

fn start_up_threads(number_of_threads: usize) -> (Vec<(thread::JoinHandle<()>, Sender<RayTracePixelConfig>)>, Receiver<PixelColor>) {

    let mut material_library = MaterialLibrary::new();
    let lambert_1_id = material_library.add_new(Box::new(Lambertian::new(Vector3::new(0.8, 0.3, 0.3))));
    let lambert_2_id = material_library.add_new(Box::new(Lambertian::new(Vector3::new(0.8, 0.8, 0.0))));
    let metal_1_id = material_library.add_new(Box::new(Metal::new(Vector3::new(0.8, 0.6, 0.2), 0.3)));
    let dielectric_1_id = material_library.add_new(Box::new(Deilectric::new(1.5)));

    let world_list : Vec<Box<Hitable  + Send>> = vec![
        Box::new(Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5, lambert_1_id)),
        Box::new(Sphere::new(Vector3::new(0.0, -100.5, -1.0), 100.0, lambert_2_id)),
        Box::new(Sphere::new(Vector3::new(1.0, 0.0, -1.0), 0.5, metal_1_id)),
        Box::new(Sphere::new(Vector3::new(-1.0, 0.0,-1.0), -0.45, dielectric_1_id)),
    ];
    let world = HitableList::new(world_list);
    let arc_world = Arc::new(world);

    let mut join_handles = vec![];
    let (pixel_tx, pixel_rx) : (Sender<PixelColor>, Receiver<PixelColor>) = mpsc::channel();
    for _ in 0..number_of_threads {
        let (tx, rx) = channel();
        let pixel_tx_clone = pixel_tx.clone();
        let world_arc_clone = Arc::clone(&arc_world);
        let jh = thread::spawn(move ||{render_thread(world_arc_clone, rx, pixel_tx_clone)});
        join_handles.push((jh, tx));
    }
    return (join_handles, pixel_rx);
}

fn main() {

    let(thread_comms, final_results) = start_up_threads(num_cpus::get());
    
    
    let mut buffer: Vec<u32> = vec![0;WIDTH * HEIGHT];
    let mut window = Window::new("Test - ESC to exit", WIDTH, HEIGHT, WindowOptions::default()).unwrap_or_else(|e|{
        panic!("{}", e);
    });
//    window.
    let mut frame_count = 0;

    while window.is_open() && !window.is_key_down(Key::Escape) {

        let mut ray_trace_pixel_configs = vec![];
        let mut count = 0;
        for y in (0..HEIGHT).rev() {
            for x in 0..WIDTH {
                let rtpc = RayTracePixelConfig {
                    width: WIDTH,
                    height: HEIGHT,
                    x: x as u32,
                    y: y as u32,
                    number_of_samples: 1,
                    index: count
                };

                ray_trace_pixel_configs.push(rtpc);
                count += 1;
            }
        }
/*
        while ray_trace_pixel_configs.is_empty() == false {
            for thread in thread_comms.0.iter() {

            }
        }
*/

        
        //buffer[count] = final_color; 
        //frame_count+=1;
        window.set_title(frame_count.to_string().as_str());
        window.update_with_buffer(&buffer).unwrap();
    }
}