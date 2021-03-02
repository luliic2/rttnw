use super::{Position, Ray, Vec3f};

/// Simple axis-aligned camera.
pub struct Camera {
    pub origin: Vec3f<Position>,
    pub lower_left_corner: Vec3f<Position>,
    pub horizontal: Vec3f<Position>,
    pub vertical: Vec3f<Position>,
}

impl Camera {
    /// The resulting ray pointing from the camera to the (u, v) coordinates.
    pub fn ray(&self, u: f32, v: f32) -> Ray {
        Ray {
            a: self.origin,
            b: self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        }
    }
}
