use super::{Position, Ray, Vec3f};

/// The result after a ray hits an object.
#[derive(Clone, Copy)]
pub struct HitRecord {
    /// Ray's `t` parameter for which the ray hits
    pub t: f32,
    /// Point that the ray hit
    pub p: Vec3f<Position>,
    pub normal: Vec3f<Position>,
}
impl Default for HitRecord {
    fn default() -> Self {
        Self {
            t: 0.0,
            p: (0, 0, 0).into(),
            normal: (0, 0, 0).into(),
        }
    }
}

/// Trait for objects that a ray can hit.
pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

/// A sphere that can be hit by a ray.
pub struct Sphere {
    pub center: Vec3f<Position>,
    pub radius: f32,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        // Solve quadratic equation
        let a = ray.direction().dot(&ray.direction());
        let b = oc.dot(&ray.direction());
        let c = oc.dot(&oc) - self.radius.powf(2.0);
        let discriminant = b.powf(2.0) - a * c;
        // A ray hits
        if discriminant > 0.0 {
            let temp = (-b - (b.powf(2.0) - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                let p = ray.point_at_parameter(temp);
                let normal = (p - self.center) / self.radius;
                return Some(HitRecord{ t: temp, p, normal});
            }
            let temp = (-b + (b.powf(2.0) - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                let p = ray.point_at_parameter(temp);
                let normal = (p - self.center) / self.radius;
                return Some(HitRecord{ t: temp, p, normal});
            }
        }
        None
    }
}

/// List of items that can be hit by a ray
pub struct List<T> {
    pub list: Vec<T>,
}
impl<T: Hittable> Hittable for List<T> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32 ) -> Option<HitRecord> {
        let mut record = None;
        let mut closest = t_max;
        for i in &self.list {
            if let Some(new_record) = i.hit(&ray, t_min, closest) {
                closest = new_record.t;
                record = Some(new_record);
            }
        }
        record
    }
}
