use super::{Vec3f, Position, Ray};

pub struct Camera {
    pub origin: Vec3f<Position>,
    pub lower_left_corner: Vec3f<Position>,
    pub horizontal: Vec3f<Position>,
    pub vertical: Vec3f<Position>,
}

impl Camera {
    pub fn ray(&self, u: f32, v: f32) -> Ray {
        Ray {
            a: self.origin,
            b: self.lower_left_corner + u*self.horizontal + v*self.vertical - self.origin,
        }
    }
}