#[macro_use]
extern crate minifb;
extern crate nalgebra as na;
extern crate rand;

use minifb::{Key, WindowOptions, Window};
use crate::rand::{thread_rng, Rng};

pub mod math;
pub mod controls;
pub mod scene;

use std::f32;


use self::na::Vector3;

use self::math::Ray;
use self::controls::Camera;
use self::scene::*;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

pub fn color(ray: &Ray, world: &Hitable, depth: i32, material_library: &MaterialLibrary) -> Vector3<f32> {
    let mut record : HitRecord = HitRecord::empty();
    if depth > 5 {
        return Vector3::new(0.0, 0.0, 0.0);
    }

    if world.hit(ray, 0.001, f32::MAX, &mut record) == true {
        
        if depth > 50 {
            return Vector3::new(0.0, 0.0, 0.0);
        }

        let material = material_library.checkout_material(record.material);
        match &material {
            Some(mat) => {
                let scatter_hit = mat.scatter(ray, &record);
                if scatter_hit.result == true {
                    return /*scatter_hit.attenuation */ color(&scatter_hit.scattered, world, depth + 1, material_library);
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

fn main() {
    let mut rng = thread_rng();

    let mut material_library = MaterialLibrary::new();
    let lambert_1_id = material_library.add_new(Box::new(Lambertian::new(Vector3::new(0.8, 0.3, 0.3))));
    let lambert_2_id = material_library.add_new(Box::new(Lambertian::new(Vector3::new(0.8, 0.8, 0.0))));
    let metal_1_id = material_library.add_new(Box::new(Metal::new(Vector3::new(0.8, 0.6, 0.2), 0.3)));
    let dielectric_1_id = material_library.add_new(Box::new(Deilectric::new(1.5)));

    let mut world = vec![
        Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5, lambert_1_id),
        Sphere::new(Vector3::new(0.0, -100.5, -1.0), 100.0, lambert_2_id),
        Sphere::new(Vector3::new(1.0, 0.0, -1.0), 0.5, metal_1_id),
        Sphere::new(Vector3::new(-1.0, 0.0,-1.0), -0.45, dielectric_1_id),
//        Sphere::new(Vector3::new(-1.0, 0.0, -))
    ];
    let c = Camera::new(90.0, WIDTH as f32 / HEIGHT as f32);
    let samples = 1;
    for y in 0..WIDTH {
        for x in 0..HEIGHT {
            for s in 0..samples {
                let x = x as f32;
                let y = y as f32;

                let u = x + rng.gen_range(0.0, 1.0) / WIDTH as f32;
                let v = y + rng.gen_range(0.0, 1.0) / HEIGHT  as f32;
                let r = c.get_ray(u, v);
                let p = r.point_at_paramater(2.0);
            }
        }
    }
    let mut buffer: Vec<u32> = vec![0;WIDTH * HEIGHT];
    let mut window = Window::new("Test - ESC to exit", WIDTH, HEIGHT, WindowOptions::default()).unwrap_or_else(|e|{
        panic!("{}", e);
    });

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for i in buffer.iter_mut() {
            *i = 1000;
        }

        window.update_with_buffer(&buffer).unwrap();
    }
}