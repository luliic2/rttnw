#![allow(dead_code)]

use super::{Position, Vec3f};

/// A ray is defined as the function
/// `p(t) = A + tB`, where `A` is the origin of the ray
/// and `B` it's direction.
#[derive(Clone, Copy)]
pub struct Ray {
    pub a: Vec3f<Position>,
    pub b: Vec3f<Position>,
}

impl Ray {
    /// Origin of the ray
    pub fn origin(&self) -> Vec3f<Position> {
        self.a
    }
    /// Direction of the ray
    pub fn direction(&self) -> Vec3f<Position> {
        self.b
    }
    /// Ray as the function `p(t) = A + tB`
    pub fn point_at_parameter(&self, t: f32) -> Vec3f<Position> {
        self.a + t * self.b
    }
}
