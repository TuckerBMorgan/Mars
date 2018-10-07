use crate::na::Vector3;
use crate::math::Ray;

pub struct Camera {
    origin: Vector3<f32>,
    lower_left_corner: Vector3<f32>,
    vertical: Vector3<f32>,
    horizontal: Vector3<f32>
}

impl Camera {
    pub fn new(/*vfov: f32, aspect: f32*/) -> Camera {
        Camera{
            lower_left_corner: Vector3::new(-2.0, -1.0, -1.0),
            horizontal: Vector3::new(2.0 * 2.0, 0.0, 0.0),
            vertical: Vector3::new(0.0, 2.0 * 1.0, 0.0),
            origin: Vector3::new(0.0, 0.0, 0.0)
        }
    }

    pub fn get_ray(&self, u:f32, v:f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin
        )
    }
}