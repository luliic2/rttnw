use super::{Position, Ray, Vec3f};
use rand::Rng;

#[derive(Default)]
pub struct CameraDescriptor {
    pub lookfrom: Vec3f<Position>,
    pub lookat: Vec3f<Position>,
    pub view_up: Vec3f<Position>,
    pub vertical_fov: f64, // Degrees
    pub aspect_ratio: f64,
    pub aperture: f64,
    pub focus_distance: f64,
    pub open_time: f64,
    pub close_time: f64,
}

#[derive(Default)]
pub struct Camera {
    pub origin: Vec3f<Position>,
    pub lower_left_corner: Vec3f<Position>,
    pub horizontal: Vec3f<Position>,
    pub vertical: Vec3f<Position>,
    pub u: Vec3f<Position>,
    pub v: Vec3f<Position>,
    pub w: Vec3f<Position>,
    pub lens_radius: f64,
    pub open_time: f64,
    pub close_time: f64,
}

impl Camera {
    pub fn new(descriptor: &CameraDescriptor) -> Self {
        let lens_radius = descriptor.aperture / 2.0;
        let theta = descriptor.vertical_fov * std::f64::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = descriptor.aspect_ratio * half_height;
        let origin = descriptor.lookfrom;
        let w = (descriptor.lookfrom - descriptor.lookat).unit();
        let u = descriptor.view_up.cross(w).unit();
        let v = w.cross(u);
        let lower_left_corner = origin
            - half_width * descriptor.focus_distance * u
            - half_height * descriptor.focus_distance * v
            - descriptor.focus_distance * w;
        let horizontal = 2.0 * half_width * descriptor.focus_distance * u;
        let vertical = 2.0 * half_height * descriptor.focus_distance * v;
        let open_time = descriptor.open_time;
        let close_time = descriptor.close_time;
        Self {
            lower_left_corner,
            horizontal,
            vertical,
            origin,
            v,
            u,
            w,
            lens_radius,
            open_time,
            close_time,
        }
    }
    /// Generate a point around
    fn random_in_unit_disk() -> Vec3f<Position> {
        let mut rng = rand::thread_rng();
        loop {
            let p = 2.0 * Vec3f::new(rng.gen(), rng.gen(), 0.0) - Vec3f::new(1.0, 1.0, 0.0);
            if p.dot(p) < 1.0 {
                return p;
            }
        }
    }
    /// The resulting ray pointing from the camera to the (u, v) coordinates.
    pub fn ray(&self, s: f64, t: f64) -> Ray {
        let mut rng = rand::thread_rng();
        let rd = self.lens_radius * Self::random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();
        Ray {
            a: self.origin + offset,
            b: self.lower_left_corner + s * self.horizontal + t * self.vertical
                - self.origin
                - offset,
            time: rng.gen_range(self.open_time..self.close_time),
        }
    }
}
