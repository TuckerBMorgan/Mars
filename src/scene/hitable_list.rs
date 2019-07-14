use crate::scene::{Hitable, HitRecord, HitableID, HitableLibrary};
use crate::math::Ray;

pub struct HitableList {
    id_list: Vec<HitableID>
}

impl HitableList {
    pub fn new_with_hitable_id_list(list: Vec<HitableID>) -> HitableList {
        HitableList {
            id_list: list
        }
    }

    pub fn cast_ray_into_world(&self, ray_in: &Ray, t_min: f32, t_max: f32, record: &mut HitRecord, hitable_library: &HitableLibrary) -> bool {
        let mut temp_rec = HitRecord::empty();

        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for id in &self.id_list {
            let object = hitable_library.checkout_hitable(*id);
            match object {
                Some(unwraped_object) => {
                    if unwraped_object.hit(ray_in, t_min, closest_so_far, &mut temp_rec) {
                        hit_anything = true;
                        closest_so_far = temp_rec.t;
                        record.copy_over(&temp_rec);
                    }
                },
                None => {panic!("Object {} does not exist", id);}
            }
        }
        /*
        for hitable in &self.list {            
            if hitable.hit(ray_in, t_min, closest_so_far, &mut temp_rec) == true {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                record.copy_over(&temp_rec);
            }
        }
        */
        return hit_anything;
    }
}