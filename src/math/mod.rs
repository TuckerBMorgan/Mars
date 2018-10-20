pub mod ray;

pub use self::ray::Ray;
use crate::euclid::Vector3D;
use crate::rand::{thread_rng, Rng};

#[inline]
pub fn random_in_unit_sphere() ->Vector3D<f32>{
    let mut rng = thread_rng();
   Vector3D::new(rand::random::<f32>(), rand::random::<f32>(), rand::random::<f32>()).normalize() * rng.gen_range(0.0, 1.0)
}

#[inline]
pub fn reflect(a: Vector3D<f32>, b: Vector3D<f32>) ->Vector3D<f32> {
    return a - (b * a.dot(b) * 2.0);
}

#[inline]
pub fn refract(v: Vector3D<f32>, n: Vector3D<f32>, ni_over_nt: f32) -> Option<Vector3D<f32>> {
    let uv = v.normalize();
    let dt = uv.dot(n);
    let det = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if det > 0.0 {
        return Some((uv - n * dt) * ni_over_nt - n * det.sqrt());
    }
    return None;
}