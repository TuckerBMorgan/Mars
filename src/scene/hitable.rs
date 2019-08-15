use crate::math::{Ray};
use crate::scene::{MaterialID};
use crate::glam::Vec3;

pub type HitableID = u32;

#[derive(Clone, Copy)]
pub struct RayCastResult {
    pub hits: [HitRecord; 10],
    pub number_of_hits: usize
}

impl RayCastResult {
    pub fn new() -> RayCastResult {
        RayCastResult {
            hits: [HitRecord::empty(); 10],
            number_of_hits: 0
        }
    }

    pub fn reset(&mut self) {
        self.number_of_hits = 0;
    }
}

#[derive(Clone, Copy)]
pub struct HitRecord {
    pub t: f32,
    pub position:Vec3,
    pub normal:Vec3,
    pub material: MaterialID,
    pub hitable: HitableID
}

impl HitRecord {
    pub fn empty() -> HitRecord {
        HitRecord {
            t:0.0,
            position:Vec3::new(0.0, 0.0, 0.0),
            normal:Vec3::new(0.0, 0.0, 0.0),
            material: 0,
            hitable: 0
        }
    }

    #[inline]
    pub fn copy_over(&mut self, other: &HitRecord) {
        self.t = other.t;
        self.position = other.position;
        self.normal = other.normal;
        self.material = other.material;
        self.hitable = other.hitable;
    }
}

pub trait Hitable: Send {
    fn hit(&self, ray: &Ray, t_min: f32, t_mac: f32, record: &mut HitRecord) -> bool;
    fn quick_hit(&self, ray: &Ray) -> bool;
    fn set_hitable_id(&mut self, id: HitableID);
    fn get_center(&self) -> Vec3;
    fn get_radius(&self) -> f32;
}