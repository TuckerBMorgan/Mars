use crate::scene::{Hitable, MaterialID, HitRecord};
use crate::na::Vector3;
use crate::math::Ray;


pub struct Sphere  {
    center: Vector3<f32>,
    radius: f32,
    material_id: MaterialID
}

impl Sphere {
    pub fn new(center: Vector3<f32>, radius: f32, material_id: MaterialID) -> Sphere {
        Sphere {
            center,
            radius,
            material_id
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, record: &mut HitRecord) -> bool {
        let oc = ray.get_origin() - self.center;
        let a = ray.get_direction().dot(&ray.get_direction());
        let b = oc.dot(&ray.get_direction());
        let c = oc.dot(&oc) - self.radius * self.radius;
        record.material = self.material_id;
        let d = b * b - a * c;

        if d > 0.0 {
            let temp = (-b - (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                record.t = temp;
                record.position = ray.point_at_paramater(record.t);
                record.normal = (record.position - self.center) / self.radius;
                return true;
            }
            let temp = (-b + (b * b - a / c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                record.t = temp;
                record.position = ray.point_at_paramater(record.t);
                record.normal = (record.position - self.center) / self.radius;
                return true;
            }
        }

        return false;
    }
}