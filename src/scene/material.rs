use crate::math::Ray;
use crate::scene::{Hitable, HitRecord};
use crate::na::Vector3;

pub struct ScatterHit {
    result: bool,
    attenuation: Vector3<f32>,
    scattered: Ray
}
impl ScatterHit {
    pub fn new(result: bool, attenuation: Vector3<f32>, scattered: Ray) ->  ScatterHit {
        ScatterHit {
            result,
            attenuation,
            scattered
        }
    }
}

pub trait Material {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> ScatterHit;
}

struct Lambertian {
    pub albedo: Vector3<f32>,
}

impl Lambertian {
    pub fn new(albedo: Vector3<f32>) -> Lambertian {
        Lambertian {
            albedo
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> ScatterHit {
        let target = record.position + record.normal;//RANDOM_IN_UNIT_SPHERE()
        ScatterHit::new(
            true,
            self.albedo.clone(),
            Ray::new(record.position, target - record.position)
        )
    }
}