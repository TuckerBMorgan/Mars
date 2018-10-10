use crate::math::Ray;
use crate::scene::{Hitable, HitRecord};
use crate::na::Vector3;
use std::collections::HashMap;

pub type MaterialID = u32;

pub struct MaterialLibrary {
    material_id_counter: MaterialID,
    library: HashMap<MaterialID, Box<Material>>
}

impl MaterialLibrary {
    pub fn new() -> MaterialLibrary {
        MaterialLibrary {
            material_id_counter: 1,
            library: HashMap::new()
        }
    }

    pub fn add_new(&mut self, material: Box<Material>) -> MaterialID {
        self.material_id_counter += 1;
        self.library.insert(self.material_id_counter, material);
        return self.material_id_counter;
    }

    
    pub fn checkout_material(&self, material_id: MaterialID) -> Option<&Box<Material>> {
        return self.library.get(&material_id);
    }
    
}

pub struct ScatterHit {
    pub result: bool,
    pub attenuation: Vector3<f32>,
    pub scattered: Ray
}

impl ScatterHit {
    pub fn new(result: bool, attenuation: Vector3<f32>, scattered: Ray) ->  ScatterHit {
        ScatterHit {
            result,
            attenuation,
            scattered
        }
    }
}

pub trait Material {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> ScatterHit;
}

struct Lambertian {
    pub albedo: Vector3<f32>,
}

impl Lambertian {
    pub fn new(albedo: Vector3<f32>) -> Lambertian {
        Lambertian {
            albedo
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> ScatterHit {
        let target = record.position + record.normal;//RANDOM_IN_UNIT_SPHERE()
        ScatterHit::new(
            true,
            self.albedo.clone(),
            Ray::new(record.position, target - record.position)
        )
    }
}