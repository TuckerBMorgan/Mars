use crate::scene::{Hitable, MaterialID, HitRecord};
use crate::math::Ray;
use crate::euclid::Vector3D;

pub struct Sphere  {
    center:Vector3D<f32>,
    radius: f32,
    material_id: MaterialID
}

impl Sphere {
    pub fn new(center:Vector3D<f32>, radius: f32, material_id: MaterialID) -> Sphere {
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
        let a = ray.direction.dot(ray.direction);
        let b = oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        record.material = self.material_id;
        let d = b * b - a * c;
        if d > 0.0 {
            let d = d.sqrt();
            let temp = (-b - d) / a;
            if temp < t_max && temp > t_min {
                record.t = temp;
                record.position = ray.point_at_paramater(record.t);
                record.normal = (record.position - self.center) / self.radius;
                return true;
            }

            let temp = (-b + d) / a;
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
