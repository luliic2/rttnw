use super::{Position, Ray, Vec3f};
use crate::math::material::Material;

/// The result after a ray hits an object.
#[derive(Clone, Copy)]
pub struct HitRecord<'a> {
    /// Ray's `t` parameter for which the ray hits
    pub t: f64,
    /// Point that the ray hit
    pub p: Vec3f<Position>,
    pub normal: Vec3f<Position>,
    pub material: &'a dyn Material,
}

/// Trait for objects that a ray can hit.
pub trait Hittable: Send + Sync {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

/// A sphere that can be hit by a ray.
pub struct Sphere {
    pub center: Vec3f<Position>,
    pub radius: f64,
    pub material: Box<dyn Material>,
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        // Solve quadratic equation
        let a = ray.direction().dot(ray.direction());
        let half_b = oc.dot(ray.direction());
        let c = oc.dot(oc) - self.radius.powf(2.0);
        let discriminant = half_b.powf(2.0) - a * c;
        // If the ray hits nothing
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        // If the root is out of range
        if root < t_min || t_max < root {
            // Choose the other root
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }
        let t = root;
        let p = ray.point_at_parameter(root);
        let normal = (p - self.center) / self.radius;
        Some(HitRecord {
            t,
            p,
            normal,
            material: self.material.as_ref(),
        })
    }
}

/// List of items that can be hit by a ray
pub struct List {
    // The vector owns the items.
    pub list: Vec<Box<dyn 'static + Hittable>>,
}
impl List {
    pub fn push<T: 'static + Hittable>(&mut self, item: T) {
        self.list.push(Box::new(item))
    }
    pub fn new() -> Self {
        Self { list: Vec::new() }
    }
}
impl Hittable for List {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
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

pub struct MovingSphere {
    pub initial_center: Vec3f<Position>,
    pub final_center: Vec3f<Position>,
    pub initial_time: f64,
    pub final_time: f64,
    pub radius: f64,
    pub material: Box<dyn Material>,
}

impl MovingSphere {
    pub fn center(&self, time: f64) -> Vec3f<Position> {
        self.initial_center
            + ((time - self.initial_time) / (self.final_time - self.initial_time))
                * (self.final_center - self.initial_center)
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin() - self.center(ray.time);
        // Solve quadratic equation
        let a = ray.direction().dot(ray.direction());
        let half_b = oc.dot(ray.direction());
        let c = oc.dot(oc) - self.radius.powf(2.0);
        let discriminant = half_b.powf(2.0) - a * c;
        // If the ray hits nothing
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        // If the root is out of range
        if root < t_min || t_max < root {
            // Choose the other root
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }
        let t = root;
        let p = ray.point_at_parameter(root);
        let normal = (p - self.center(ray.time)) / self.radius;
        Some(HitRecord {
            t,
            p,
            normal,
            material: self.material.as_ref(),
        })
    }
}
