use crate::glam::Vec3;

pub struct Ray {
   pub origin:Vec3,
   pub direction:Vec3
}

impl Ray {
    pub fn new(origin:Vec3, direction:Vec3) -> Ray {
        Ray {
            origin,
            direction
        }
    }

    #[inline]
    pub fn get_origin(&self) ->Vec3 {
        self.origin
    }

    #[inline]
    pub fn get_direction(&self) ->Vec3 {
        self.direction
    }

    #[inline]
    pub fn point_at_paramater(&self, t: f32) ->Vec3 {
        self.origin + self.direction * t
    } 
}