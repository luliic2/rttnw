use super::{Position, Ray, Vec3f};

/// Simple axis-aligned camera.
pub struct Camera {
    pub origin: Vec3f<Position>,
    pub lower_left_corner: Vec3f<Position>,
    pub horizontal: Vec3f<Position>,
    pub vertical: Vec3f<Position>,
}

impl Camera {
    pub fn new(
        lookfrom: Vec3f<Position>,
        lookat: Vec3f<Position>,
        view_up: Vec3f<Position>,
        vertical_fov: f32,
        aspect_ratio: f32,
    ) -> Self {
        let theta = vertical_fov * std::f32::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect_ratio * half_height;
        let origin = lookfrom;
        let w = (lookfrom - lookat).unit();
        let u = view_up.cross(w).unit();
        let v = w.cross(u);
        let lower_left_corner = origin - half_width * u - half_height * v - w;
        let horizontal = 2.0 * half_width * u;
        let vertical = 2.0 * half_height * v;
        Self {
            lower_left_corner,
            horizontal,
            vertical,
            origin,
        }
    }
    /// The resulting ray pointing from the camera to the (u, v) coordinates.
    pub fn ray(&self, u: f32, v: f32) -> Ray {
        Ray {
            a: self.origin,
            b: self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        }
    }
}
