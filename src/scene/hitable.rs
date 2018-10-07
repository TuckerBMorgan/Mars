use crate::na::Vector3;
use crate::math::Ray;
use crate::scene::Material;

pub struct HitRecord {
    pub t: f32,
    pub position: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub material: Material
}

impl HitRecord {
    pub fn empty() -> HitRecord {
        HitRecord {
            t:0.0,
            position: Vector3::new(),
            normal: Vector3
        }
    }
}

pub trait Hitable {
    fn hit(ray: &Ray, t_min: f32, t_mac: f32, record: &HitRecord) -> bool;
}