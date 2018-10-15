use crate::scene::{Hitable, HitRecord};
use crate::math::Ray;

pub struct HitableList {
    list: Vec<Box<Hitable>>
}

impl HitableList {
    pub fn new(list: Vec<Box<Hitable>>) -> HitableList {
        HitableList {
            list
        }
    }
}

impl Hitable for HitableList {
    fn hit(&self, ray_in: &Ray, t_min: f32, t_max: f32, record: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::empty();

        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        
        for hitable in &self.list {
            if hitable.hit(ray_in, t_min, closest_so_far, &mut temp_rec) == true {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                record.copy_over(&temp_rec);
            }
        }
        
        return hit_anything;
    }
}