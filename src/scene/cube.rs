use crate::scene::{Hitable, MaterialID, HitRecord, HitableID};
use crate::math::Ray;
use crate::glam::Vec3;

pub struct Cube  {
    center:Vec3,
    radius: f32,
    material_id: MaterialID,
    radius_sqrd: f32,
    id: HitableID
}

impl Cube {
    pub fn new(center:Vec3, radius: f32, material_id: MaterialID) -> Cube {
        Cube {
            center,
            radius,
            material_id,
            radius_sqrd: radius * radius,
            id: 0
        }
    }
}

impl Hitable for Cube {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, record: &mut HitRecord) -> bool {
        
        let oc = ray.get_origin() - self.center;
        let a = ray.direction.dot(ray.direction);
        let b = oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius_sqrd;
        let d = b * b - a * c;
        if d > 0.0 {
            let d = d.sqrt();
            let temp = (-b - d) / a;
            if temp < t_max && temp > t_min {
                record.material = self.material_id;
                record.t = temp;
                record.position = ray.point_at_paramater(record.t);
                record.normal = (record.position - self.center) / self.radius;
                record.hitable = self.id;
                return true;
            }

            let temp = (-b + d) / a;
            if temp < t_max && temp > t_min {
                record.material = self.material_id;
                record.t = temp;
                record.position = ray.point_at_paramater(record.t);
                record.normal = (record.position - self.center) / self.radius;
                record.hitable = self.id;
                return true;
            }
        }
        return false;
    }

    fn set_hitable_id(&mut self, id: HitableID) {
        self.id = id;
    }

    fn get_center(&self) -> Vec3 {
        return self.center;
    }

    fn get_radius(&self) -> f32 {
        return self.radius;
    }

}
