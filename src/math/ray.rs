use crate::euclid::Vector3D;

pub struct Ray {
   pub origin:Vector3D<f32>,
   pub direction:Vector3D<f32>
}

impl Ray {
    pub fn new(origin:Vector3D<f32>, direction:Vector3D<f32>) -> Ray {
        Ray {
            origin,
            direction
        }
    }

    #[inline]
    pub fn get_origin(&self) ->Vector3D<f32> {
        self.origin
    }

    #[inline]
    pub fn get_direction(&self) ->Vector3D<f32> {
        self.direction
    }

    #[inline]
    pub fn point_at_paramater(&self, t: f32) ->Vector3D<f32> {
        self.origin + self.direction * t
    } 
}