use super::{Position, Ray, Vec3f};
use rand::Rng;

pub struct Camera {
    pub origin: Vec3f<Position>,
    pub lower_left_corner: Vec3f<Position>,
    pub horizontal: Vec3f<Position>,
    pub vertical: Vec3f<Position>,
    pub u: Vec3f<Position>,
    pub v: Vec3f<Position>,
    pub w: Vec3f<Position>,
    pub lens_radius: f32,
}

impl Camera {
    pub fn new(
        lookfrom: Vec3f<Position>,
        lookat: Vec3f<Position>,
        view_up: Vec3f<Position>,
        vertical_fov: f32,
        aspect_ratio: f32,
        aperture: f32,
        focus_distance: f32,
    ) -> Self {
        let lens_radius = aperture / 2.0;
        let theta = vertical_fov * std::f32::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect_ratio * half_height;
        let origin = lookfrom;
        let w = (lookfrom - lookat).unit();
        let u = view_up.cross(w).unit();
        let v = w.cross(u);
        let lower_left_corner = origin
            - half_width * focus_distance * u
            - half_height * focus_distance * v
            - focus_distance * w;
        let horizontal = 2.0 * half_width * focus_distance * u;
        let vertical = 2.0 * half_height * focus_distance * v;
        Self {
            lower_left_corner,
            horizontal,
            vertical,
            origin,
            v,
            u,
            w,
            lens_radius,
        }
    }
    /// Generate a point around 
    fn random_in_unit_disk() -> Vec3f<Position> {
        let mut rng = rand::thread_rng();
        loop {
            let p = 2.0 * Vec3f::new(rng.gen(), rng.gen(), 0.0) - Vec3f::new(1.0, 1.0, 0.0);
            if p.dot(p) >= 1.0 {
                return p;
            }
        }
    }
    /// The resulting ray pointing from the camera to the (u, v) coordinates.
    pub fn ray(&self, s: f32, t: f32) -> Ray {
        let rd = self.lens_radius * Self::random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();
        Ray {
            a: self.origin + offset,
            b: self.lower_left_corner + s * self.horizontal + t * self.vertical
                - self.origin
                - offset,
        }
    }
}
