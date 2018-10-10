use crate::na::Vector3;
use crate::math::Ray;
use crate::scene::{Material, MaterialID};


pub struct HitRecord {
    pub t: f32,
    pub position: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub material: MaterialID
}

impl HitRecord {
    pub fn empty() -> HitRecord {
        HitRecord {
            t:0.0,
            position: Vector3::new(0.0, 0.0, 0.0),
            normal: Vector3::new(0.0, 0.0, 0.0),
            material: 0
        }
    }

    #[inline]
    pub fn copy_over(&mut self, other: &HitRecord) {
        self.t = other.t;
        self.position = other.position;
        self.normal = other.normal;
        self.material = other.material;
    }
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, t_min: f32, t_mac: f32, record: &mut HitRecord) -> bool;
}