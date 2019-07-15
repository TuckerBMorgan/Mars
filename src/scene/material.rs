use crate::math::*;
use crate::glam::Vec3;
use crate::scene::{HitRecord, Hitable};

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
    pub attenuation:Vec3,
    pub scattered: Ray
}

impl ScatterHit {
    pub fn new(result: bool, attenuation:Vec3, scattered: Ray) ->  ScatterHit {
        ScatterHit {
            result,
            attenuation,
            scattered
        }
    }
}

pub trait Material {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> ScatterHit;
    fn color(&self, record: &HitRecord, hitable: &Hitable) -> Vec3;
}

pub struct Lambertian {
    pub albedo:Vec3,
}

impl Lambertian {
    pub fn new(albedo:Vec3) -> Lambertian {
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

    fn color(&self, record: &HitRecord, hitable: &Hitable) -> Vec3 {
        return self.albedo;
    }
}


pub struct CheckerBoard {
    pub albedo:Vec3,
    pub metal_material: Metal
}

impl CheckerBoard {
    pub fn new(albedo:Vec3) -> CheckerBoard {
        CheckerBoard {
            albedo,
            metal_material: Metal::new(Vec3::new(0.3f32, 0.432, 0.7f32), 0.0f32)
        }
    }
}

impl Material for CheckerBoard {
    fn scatter(&self, _ray_in: &Ray, record: &HitRecord) -> ScatterHit {
        let target = record.position + record.normal + random_in_unit_sphere();
        ScatterHit::new(
            true,
            self.albedo.clone(),
            Ray::new(record.position, target - record.position)
        )
    }

    fn color(&self, record: &HitRecord, hitable: &Hitable) -> Vec3 {

        let d = (hitable.get_center() - record.position).normalize();
        let u = 0.5f32 + (d.z().atan2(d.x()) / 2.0f32 * std::f64::consts::PI as f32);
        let v = 0.5f32 + (d.y().asin() / std::f64::consts::PI as f32);

        if (u * 10.0f32) as i32 % 2 == 0 {
            if (v * 10.0f32) as i32 % 2 == 0 {
                return Vec3::new(1.0, 0.0, 0.0);
            }
            else {
                return self.metal_material.color(record, hitable);
            }
        }
        else {
            if (v * 10.0f32) as i32 % 2 == 0 {
                return self.metal_material.color(record, hitable);
            }
            else {
                return Vec3::new(1.0, 0.0, 0.0);        
            }
        }
    }
}

pub struct Metal {
    pub albedo:Vec3,
    pub fuzz: f32,
}

impl Metal {
    pub fn new(albedo:Vec3, fuzz: f32) -> Metal {
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
        let reflected = reflect(ray_in.get_direction().normalize(), record.normal);
        let scattered = Ray::new(record.position, reflected + random_in_unit_sphere() * self.fuzz);
        let attenuation = self.albedo;
        let result = scattered.get_direction().dot(record.normal) > 0.0;
        return ScatterHit::new(result, attenuation, scattered);
    }

    fn color(&self, record: &HitRecord, hitable: &Hitable) -> Vec3 {
        return self.albedo;
    }
}

pub struct Sky {
}

impl Sky {
    pub fn new() -> Sky {
        Sky {}
    }
}

impl Material for Sky {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> ScatterHit {
        return ScatterHit::new(false, Vec3::new(1.0, 1.0, 1.0), Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0)));
    }

    fn color(&self, record: &HitRecord, hitable: &Hitable) -> Vec3 {
        let t = 0.5 * (record.normal.y() + 1.0);
        return Vec3::new(1.0f32, 1.0f32, 1.0f32) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t;//background color, I Think?    
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
        let reflected = reflect(ray_in.get_direction(), record.normal);
        let outward_normal;
        let ni_over_nt;
        let consine;
        let mut refracted =Vec3::new(0.0, 0.0, 0.0);
        let relfect_prob;
        let scattered;
        let attenuation = Vec3::new(1.0, 1.0, 1.0);

        if ray_in.get_direction().dot(record.normal) > 0.0 {
            outward_normal = -record.normal;
            ni_over_nt =  self.ref_index;
            consine = self.ref_index * ray_in.get_direction().dot(record.normal) / ray_in.get_direction().length();
        }
        else {
            outward_normal = record.normal;
            ni_over_nt = 1.0 / self.ref_index;
            consine = -ray_in.get_direction().dot(record.normal) / ray_in.get_direction().length();
        }

        let refraction_test = refract(ray_in.get_direction(), outward_normal, ni_over_nt);
        match refraction_test {
            Some(val) => {
                relfect_prob = schlick(consine, self.ref_index);
                refracted = val;
            },
            None => {
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

    fn color(&self, record: &HitRecord, hitable: &Hitable) -> Vec3 {
        return Vec3::new(1.0, 1.0, 1.0);
    }
}
