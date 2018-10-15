use crate::na::Vector3;


pub struct Ray {
   pub origin: Vector3<f32>,
   pub direction: Vector3<f32>
}

impl Ray {
    pub fn new(origin: Vector3<f32>, direction: Vector3<f32>) -> Ray {
        Ray {
            origin,
            direction
        }
    }

    #[inline]
    pub fn get_origin(&self) -> Vector3<f32> {
        self.origin
    }

    #[inline]
    pub fn get_direction(&self) -> Vector3<f32> {
        self.direction
    }

    #[inline]
    pub fn point_at_paramater(&self, t: f32) -> Vector3<f32> {
        self.origin + t * self.direction
    } 
}