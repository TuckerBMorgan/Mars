use crate::math::Ray;
use crate::euclid::Vector3D;

pub struct Camera {
    origin:Vector3D<f32>,
    lower_left_corner:Vector3D<f32>,
    vertical:Vector3D<f32>,
    horizontal:Vector3D<f32>
}

impl Camera {
    pub fn new(_vfov: f32, _aspect: f32) -> Camera {
        Camera{
            lower_left_corner:Vector3D::new(-2.0, -1.0, -1.0),
            horizontal:Vector3D::new(2.0 * 2.0, 0.0, 0.0),
            vertical:Vector3D::new(0.0, 2.0 * 1.0, 0.0),
            origin:Vector3D::new(0.0, 0.0, 0.0)
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