use super::{Position, Ray, Vec3f};
use crate::math::material::Material;

/// The result after a ray hits an object.
#[derive(Clone, Copy)]
pub struct HitRecord<'a> {
    /// Ray's `t` parameter for which the ray hits
    pub t: f32,
    /// Point that the ray hit
    pub p: Vec3f<Position>,
    pub normal: Vec3f<Position>,
    pub material: &'a dyn Material,
}

/// Trait for objects that a ray can hit.
pub trait Hittable {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

/// A sphere that can be hit by a ray.
pub struct Sphere {
    pub center: Vec3f<Position>,
    pub radius: f32,
    pub material: Box<dyn Material>,
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        // Solve quadratic equation
        let a = ray.direction().dot(ray.direction());
        let b = oc.dot(ray.direction());
        let c = oc.dot(oc) - self.radius.powf(2.0);
        let discriminant = b.powf(2.0) - a * c;
        // A ray hits
        if discriminant > 0.0 {
            let temp = (-b - (b.powf(2.0) - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                let p = ray.point_at_parameter(temp);
                let normal = (p - self.center) / self.radius;
                return Some(HitRecord {
                    t: temp,
                    p,
                    normal,
                    material: self.material.as_ref(),
                });
            }
            let temp = (-b + (b.powf(2.0) - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                let p = ray.point_at_parameter(temp);
                let normal = (p - self.center) / self.radius;
                return Some(HitRecord {
                    t: temp,
                    p,
                    normal,
                    material: self.material.as_ref(),
                });
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
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut record = None;
        let mut closest = t_max;
        for i in &self.list {
            if let Some(new_record) = i.hit(ray, t_min, closest) {
                closest = new_record.t;
                record = Some(new_record);
            }
        }
        record
    }
}
