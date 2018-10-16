use crate::math::*;
use crate::scene::{HitRecord};
use crate::na::Vector3;

use crate::rand::{thread_rng, Rng};

use std::collections::HashMap;

pub type MaterialID = u32;

pub struct MaterialLibrary {
    material_id_counter: MaterialID,
    library: HashMap<MaterialID, Box<Material + Send>>
}

impl MaterialLibrary {
    pub fn new() -> MaterialLibrary {
        MaterialLibrary {
            material_id_counter: 1,
            library: HashMap::new()
        }
    }

    pub fn add_new(&mut self, material: Box<Material + Send>) -> MaterialID {
        self.material_id_counter += 1;
        self.library.insert(self.material_id_counter, material);
        return self.material_id_counter;
    }

    #[inline]
    pub fn checkout_material(&self, material_id: MaterialID) -> Option<&Box<Material + Send>> {
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
    fn scatter(&self, _ray_in: &Ray, record: &HitRecord) -> ScatterHit {
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

#[inline]
fn schlick(consine: f32, ref_index: f32) -> f32 {
    let r0 = (1.0 - ref_index) / (1.0 + ref_index);
    let r02 = r0 * r0;
    return r02 + (1.0 - r02) * (1.0 - consine).powf(5.0);
}

impl Material for Deilectric {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> ScatterHit {  
        let reflected = reflect(&ray_in.get_direction(), &record.normal);
        let outward_normal;
        let ni_over_nt;
        let consine;
        let mut refracted = Vector3::new(0.0, 0.0, 0.0);
        let relfect_prob;
        let mut scattered;
        let attenuation = Vector3::new(1.0, 1.0, 1.0);

        if ray_in.get_direction().dot(&record.normal) > 0.0 {
            outward_normal = -record.normal;
            ni_over_nt =  self.ref_index;
            consine = self.ref_index * ray_in.get_direction().dot(&record.normal) / ray_in.get_direction().magnitude();
        }
        else {
            outward_normal = record.normal;
            ni_over_nt = 1.0 / self.ref_index;
            consine = -ray_in.get_direction().dot(&record.normal) / ray_in.get_direction().magnitude();
        }

        let refraction_test = refract(&ray_in.get_direction(), &outward_normal, ni_over_nt);
        match refraction_test {
            Some(val) => {
                relfect_prob = schlick(consine, self.ref_index);
                refracted = val;
            },
            None => {
                scattered = Ray::new(record.position, reflected);
                relfect_prob = 1.0;
            }
        }


        let mut rng = thread_rng();

        if rng.gen_range(0.0, 1.0) < relfect_prob {
            scattered = Ray::new(record.position, reflected);
        }
        else {
            scattered = Ray::new(record.position, refracted);
        }

        ScatterHit::new(
            true,
            attenuation,
            scattered
        )
        
    }
}
