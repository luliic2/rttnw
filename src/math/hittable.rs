#![allow(dead_code)]
#![allow(clippy::many_single_char_names)]

use rand::Rng;

use std::cmp::Ordering;
use std::sync::Arc;

use super::{Bound, Material, Position, Ray, Vec3f};

/// The result after a ray hits an object.
#[derive(Clone, Copy)]
pub struct HitRecord<'a> {
    /// Ray's `t` parameter for which the ray hits
    pub t: f64,
    /// Point that the ray hit
    pub p: Vec3f<Position>,
    pub normal: Vec3f<Position>,
    pub material: &'a dyn Material,
    /// Texture coordinates
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}

impl HitRecord<'_> {
    pub fn face_normal(ray: Ray, outward_normal: Vec3f<Position>) -> ( /* normal */ Vec3f<Position>, /* front_face */ bool) {
        let front_face = ray.direction().dot(outward_normal) < 0.;
        let normal = if front_face { outward_normal } else { -outward_normal };
        (normal, front_face)
    }
}

/// Trait for objects that a ray can hit.
pub trait Hittable: Send + Sync {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, initial_time: f64, final_time: f64) -> Option<Bound>;
}

/// A sphere that can be hit by a ray.
pub struct Sphere {
    pub center: Vec3f<Position>,
    pub radius: f64,
    pub material: Box<dyn Material>,
}

impl Sphere {
    fn uv(p: Vec3f<Position>) -> (f64, f64) {
        let theta = f64::acos(-p.y());
        let phi = f64::atan2(-p.z(), p.x()) + std::f64::consts::PI;
        let u = phi / (2.0 * std::f64::consts::PI);
        let v = theta / std::f64::consts::PI;
        (u, v)
    }
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
        let (u, v) = Self::uv(normal);
        let (normal, front_face) = HitRecord::face_normal(ray, normal);
        Some(HitRecord {
            t,
            p,
            normal,
            material: self.material.as_ref(),
            u,
            v,
            front_face
        })
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<Bound> {
        Some(Bound {
            min: self.center - Vec3f::repeat(self.radius),
            max: self.center + Vec3f::repeat(self.radius),
        })
    }
}

/// List of items that can be hit by a ray
#[derive(Default)]
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

    fn bounding_box(&self, initial_time: f64, final_time: f64) -> Option<Bound> {
        // TODO: use https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fold_first when stabilized
        let mut iter = self.list.iter();

        if let Some(item) = iter.next()?.bounding_box(initial_time, final_time) {
            // if let Some(bound) = item.bounding_box(initial_time, final_time) {
            return iter.try_fold(item, |accumulator, next| {
                next.bounding_box(initial_time, final_time)
                    .map(|x| x.surrounding(accumulator))
            });
            // }
        }
        None
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
        let outward_normal = (p - self.center(ray.time)) / self.radius;
        let (u, v) = Default::default();
        let (normal, front_face) = HitRecord::face_normal(ray, outward_normal);
        Some(HitRecord {
            t,
            p,
            normal,
            material: self.material.as_ref(),
            u,
            v,
            front_face
        })
    }

    fn bounding_box(&self, initial_time: f64, final_time: f64) -> Option<Bound> {
        let initial_box = Bound {
            min: self.center(initial_time) - Vec3f::repeat(self.radius),
            max: self.center(initial_time) + Vec3f::repeat(self.radius),
        };
        let final_box = Bound {
            min: self.center(final_time) - Vec3f::repeat(self.radius),
            max: self.center(final_time) + Vec3f::repeat(self.radius),
        };

        Some(initial_box.surrounding(final_box))
    }
}

/// Bounding Volume Hierarchy
pub struct BvhTree {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bound: Bound,
}

impl BvhTree {
    pub fn from(mut list: List, initial_time: f64, final_time: f64) -> Self {
        let length = list.list.len(); // Must due to borrow checker
        Self::new(&mut list.list, 0, length, initial_time, final_time)
    }
    pub fn new(
        objects: &mut Vec<Box<dyn Hittable>>,
        start: usize,
        end: usize,
        initial_time: f64,
        final_time: f64,
    ) -> Self {
        let mut rng = rand::thread_rng();
        let axis = rng.gen_range(0..3);
        let comparator = match axis {
            0 => Self::x_comparator,
            1 => Self::y_comparator,
            2 => Self::z_comparator,
            _ => unreachable!("Random int in range [0, 2] must be in range"),
        };
        let object_span = end - start;
        let (left, right) = match object_span {
            1 => {
                let first: Arc<dyn Hittable> = objects.remove(0).into();
                // let first = Arc::new(first);
                (first.clone(), first)
            }
            2 => {
                let first: Arc<dyn Hittable> = objects.remove(0).into();
                let second: Arc<dyn Hittable> = objects.remove(0).into();
                match comparator(&*first, &*second) {
                    Ordering::Less => (first, second),
                    _ => (second, first),
                }
            }
            _ => {
                objects.sort_by(|x, y| comparator(&**x, &**y));
                let mid = start + object_span / 2;
                let left: Arc<dyn Hittable> =
                    Arc::new(Self::new(objects, start, mid, initial_time, final_time));
                let right: Arc<dyn Hittable> =
                    Arc::new(Self::new(objects, mid, end, initial_time, final_time));

                (left, right)
            }
        };

        let box_left = left
            .bounding_box(initial_time, final_time)
            .unwrap_or_else(|| {
                eprintln!("No bounding box in BvhTree constructor");
                Default::default()
            });
        let box_right = right
            .bounding_box(initial_time, final_time)
            .unwrap_or_else(|| {
                eprintln!("No bounding box in BvhTree constructor");
                Default::default()
            });

        let bound = box_left.surrounding(box_right);
        Self { bound, left, right }
    }

    fn comparator(x: &dyn Hittable, y: &dyn Hittable, axis: usize) -> Ordering {
        let box_x = x.bounding_box(0.0, 0.0).unwrap_or_else(|| {
            eprintln!("No bounding box in BvhTree constructor");
            Default::default()
        });
        let box_y = y.bounding_box(0.0, 0.0).unwrap_or_else(|| {
            eprintln!("No bounding box in BvhTree constructor");
            Default::default()
        });
        let x = box_x.min.at(axis);
        let y = box_y.min.at(axis);
        // Poor man https://doc.rust-lang.org/std/primitive.f64.html#method.total_cmp
        // Nightly only so far
        if x < y {
            Ordering::Less
        } else if x > y {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
    pub fn x_comparator(x: &dyn Hittable, y: &dyn Hittable) -> Ordering {
        Self::comparator(x, y, 0)
    }
    pub fn y_comparator(x: &dyn Hittable, y: &dyn Hittable) -> Ordering {
        Self::comparator(x, y, 1)
    }
    pub fn z_comparator(x: &dyn Hittable, y: &dyn Hittable) -> Ordering {
        Self::comparator(x, y, 2)
    }
}

impl Hittable for BvhTree {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.bound.hit(ray, t_min, t_max) {
            return None;
        }
        let left_record = self.left.hit(ray, t_min, t_max);
        let t = if let Some(record) = left_record {
            record.t
        } else {
            t_max
        };
        let right_record = self.right.hit(ray, t_min, t);
        right_record.or(left_record)
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<Bound> {
        Some(self.bound)
    }
}

/// A 2D Rectangle on the x-y plane.
pub struct XYRectangle {
    pub material: Arc<dyn Material>,
    pub x_min: f64,
    pub x_max: f64,
    pub y_min: f64,
    pub y_max: f64,
    // `Width` of the plane. Must be non-zero.
    pub k: f64,
}

impl Hittable for XYRectangle {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - ray.origin().z()) / ray.direction().z();
        if t < t_min || t > t_max {
            return None;
        }
        let x = ray.origin().x() + t*ray.direction().x();
        let y = ray.origin().y() + t*ray.direction().y();
        if x < self.x_min || x > self.x_max || y < self.y_min || y > self.y_max {
            return None;
        }

        let u = (x-self.x_min) / (self.x_max-self.x_min);
        let v = (y-self.y_min) / (self.y_max-self.y_min);
        let outward_normal = Vec3f::new(0., 0., 1.);
        let (normal, front_face) = HitRecord::face_normal(ray, outward_normal);
        let p = ray.point_at_parameter(t);
        Some(HitRecord {
            t,
            p,
            normal,
            material: &*self.material,
            u,
            v,
            front_face
        })
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<Bound> {
        // Must have non-zero width in each dimension. Pad the Z dimension a bit.
        let bound = Bound {
            min: Vec3f::new(self.x_min, self.y_min, self.k - 0.0001),
            max: Vec3f::new(self.x_max, self.y_max, self.k + 0.0001)
        };
        Some(bound)
    }
}

/// A 2D Rectangle on the x-z plane.
pub struct XZRectangle {
    pub material: Arc<dyn Material>,
    pub x_min: f64,
    pub x_max: f64,
    pub z_min: f64,
    pub z_max: f64,
    // `Width` of the plane. Must be non-zero.
    pub k: f64,
}

impl Hittable for XZRectangle {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - ray.origin().y()) / ray.direction().y();
        if t < t_min || t > t_max {
            return None;
        }
        let x = ray.origin().x() + t*ray.direction().x();
        let z = ray.origin().z() + t*ray.direction().z();
        if x < self.x_min || x > self.x_max || z < self.z_min || z > self.z_max {
            return None;
        }

        let u = (x-self.x_min) / (self.x_max-self.x_min);
        let v = (z -self.z_min) / (self.z_max-self.z_min);
        let outward_normal = Vec3f::new(0., 0., 1.);
        let (normal, front_face) = HitRecord::face_normal(ray, outward_normal);
        let p = ray.point_at_parameter(t);
        Some(HitRecord {
            t,
            p,
            normal,
            material: &*self.material,
            u,
            v,
            front_face
        })
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<Bound> {
        // Must have non-zero width in each dimension. Pad the Y dimension a bit.
        let bound = Bound {
            min: Vec3f::new(self.x_min, self.k - 0.0001, self.z_min),
            max: Vec3f::new(self.x_max, self.k + 0.0001, self.z_max)
        };
        Some(bound)
    }
}

/// A 2D Rectangle on the y-z plane.
pub struct YZRectangle {
    pub material: Arc<dyn Material>,
    pub y_min: f64,
    pub y_max: f64,
    pub z_min: f64,
    pub z_max: f64,
    // `Width` of the plane. Must be non-zero.
    pub k: f64,
}

impl Hittable for YZRectangle {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - ray.origin().x()) / ray.direction().x();
        if t < t_min || t > t_max {
            return None;
        }
        let y = ray.origin().x() + t*ray.direction().x();
        let z = ray.origin().z() + t*ray.direction().z();
        if y < self.y_min || y > self.y_max || z < self.z_min || z > self.z_max {
            return None;
        }

        let u = (y -self.y_min) / (self.y_max-self.y_min);
        let v = (z -self.z_min) / (self.z_max-self.z_min);
        let outward_normal = Vec3f::new(0., 0., 1.);
        let (normal, front_face) = HitRecord::face_normal(ray, outward_normal);
        let p = ray.point_at_parameter(t);
        Some(HitRecord {
            t,
            p,
            normal,
            material: &*self.material,
            u,
            v,
            front_face
        })
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<Bound> {
        // Must have non-zero width in each dimension. Pad the X dimension a bit.
        let bound = Bound {
            min: Vec3f::new(self.k - 0.0001, self.y_min, self.z_min),
            max: Vec3f::new(self.k + 0.0001, self.y_max, self.z_max)
        };
        Some(bound)
    }
}