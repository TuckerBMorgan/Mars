use crate::math::*;
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

pub struct Lambertian {
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
        let target = record.position + record.normal + random_in_unit_sphere();
        ScatterHit::new(
            true,
            self.albedo.clone(),
            Ray::new(record.position, target - record.position)
        )
    }
}

pub struct Metal {
    pub albedo: Vector3<f32>,
    pub fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vector3<f32>, fuzz: f32) -> Metal {
        let mut f = fuzz;
        if fuzz > 1.0 {
            f = 1.0;
        }

        Metal {
            albedo,
            fuzz:f
        }
    }
}

impl Material for Metal {
 fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> ScatterHit {
        let reflected = reflect(&ray_in.get_direction().normalize(), &record.normal);
        let scattered = Ray::new(record.position, reflected + self.fuzz * random_in_unit_sphere());
        let attenuation = self.albedo;
        let result = scattered.get_direction().dot(&record.normal) > 0.0;
        return ScatterHit::new(result, attenuation, scattered);
    }
}

pub struct Deilectric {
    ref_index: f32
}

impl Deilectric {
    pub fn new(ref_index: f32) -> Deilectric {
        Deilectric {
            ref_index
        }
    }
}

impl Material for Deilectric {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> ScatterHit {  
        let reflected = reflect(&ray_in.get_direction(), &record.normal);
        let outward_normal;
        let ni_over_nt;
        let consine;
        let refracted;

        if ray_in.get_direction().dot(&record.normal) > 0.0 {
            outward_normal = -record.normal;
            ni_over_nt =  self.ref_index;
            consine = self.ref_index * ray_in.get_direction().dot(&record.normal) / ray_in.get_direction().length();
        }
        else {
            outward_normal = record.normal;
            ni_over_nt = 1.0 / self.ref_index;
            consine = -ray_in.get_direction().dot(&record.normal) / ray_in.get_direction().length();
        }

        let refraction_test = refract(&ray_in.get_direction(), outward_normal, ni_over_nt, refracted);


    }
}
