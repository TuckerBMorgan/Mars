use crate::scene::{Hitable, HitRecord};
use crate::math::Ray;

pub struct HitableList {
    list: Vec<Box<Hitable>>
}

impl HitableList {
    pub fn new(list: Vec<Box<Hitable>>) -> HitableList {
        Hitable {
            list
        }
    }
}

impl Hitable for HitableList {
    fn hit(ray_in: &Ray, t_min: f32, t_mac: f32, record: &HitRecord) -> bool {
        let mut temp_rec = HitRecord;
    }
}