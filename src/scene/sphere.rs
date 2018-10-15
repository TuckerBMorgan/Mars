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

    pub fn other(&self, ray: &Ray, t_min: f32, t_max: f32, oc: &Vector3<f32>, a: f32, b: f32, c: f32, d: f32, record: &mut HitRecord) -> bool {
        let temp = (-b - d) / a;
        if d > 0.0 && temp < t_max && temp > t_min {
            record.t = temp;
            record.position = ray.point_at_paramater(record.t);
            record.normal = (record.position - self.center) / self.radius;
            return true;
        }
        let temp = (-b + d) / a;
        if d > 0.0 && temp < t_max && temp > t_min {

            record.t = temp;
            record.position = ray.point_at_paramater(record.t);
            record.normal = (record.position - self.center) / self.radius;
            return true;
        }
        return false;
    }
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, record: &mut HitRecord) -> bool {
        
//        let oc = ray.get_origin() - self.center;
        let oc = Vector3::new(ray.origin.x - self.center.x, ray.origin.y - self.center.y, ray.origin.z - self.center.z);
        let a = ray.direction.dot(&ray.direction);
        let b = oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        record.material = self.material_id;
        let d = (b * b - a * c).sqrt();
        self.other(&ray, t_min, t_max, &oc, a, b, c, d, record)
    }
}
