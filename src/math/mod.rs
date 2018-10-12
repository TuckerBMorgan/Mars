pub mod ray;

pub use self::ray::Ray;

use crate::na::Vector3;
use crate::rand::{thread_rng, Rng};

#[inline]
pub fn random_in_unit_sphere() -> Vector3<f32>{
    let mut rng = thread_rng();
    Vector3::new(rand::random::<f32>(), rand::random::<f32>(), rand::random::<f32>()).normalize() * rng.gen_range(0.0, 1.0)
}

#[inline]
pub fn reflect(a: &Vector3<f32>, b: &Vector3<f32>) -> Vector3<f32> {
    return a - 2.0 * a.dot(&b) * b;
}

#[inline]
pub fn refract(v: &Vector3<f32>, n: &Vector3<f32>, ni_over_nt: f32) -> Option<Vector3<f32>> {
    let uv = v.normalize();
    let dt = uv.dot(&n);
    let det = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if det > 0.0 {
        return Some(ni_over_nt * (uv - dt * n) - det.sqrt() * n);
    }
    return None;
}