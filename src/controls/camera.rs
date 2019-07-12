use crate::math::Ray;
use crate::glam::Vec3;

pub struct Camera {
    origin:Vec3,
    lower_left_corner:Vec3,
    vertical:Vec3,
    horizontal:Vec3
}

impl Camera {
    pub fn new(_vfov: f32, _aspect: f32) -> Camera {
        Camera{
            lower_left_corner:Vec3::new(-2.0, -1.0, -1.0),
            horizontal:Vec3::new(2.0 * 2.0, 0.0, 0.0),
            vertical:Vec3::new(0.0, 2.0 * 1.0, 0.0),
            origin:Vec3::new(0.0, 0.0, 0.0)
        }
    }

    #[inline]
    pub fn get_ray(&self, u:f32, v:f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin
        )
    }
}