#![allow(dead_code)]
#![allow(clippy::many_single_char_names)]

use rand::Rng;

use std::cmp::Ordering;
use std::marker::PhantomData;
use std::ops::Range;
use std::sync::Arc;

use super::{Bound, Coordinate, Material, Position, Ray, Vec3f, Isotropic};
use crate::math::Texture;

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
    pub fn face_normal(
        ray: Ray,
        outward_normal: Vec3f<Position>,
    ) -> (
        /* normal */ Vec3f<Position>,
        /* front_face */ bool,
    ) {
        let front_face = ray.direction().dot(outward_normal) < 0.;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        (normal, front_face)
    }
}

/// Trait for objects that a ray can hit.
pub trait Hittable: Send + Sync {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, initial_time: f64, final_time: f64) -> Option<Bound>;
    fn translate(self, offset: Vec3f<Position>) -> Translate
    where
        Self: 'static + Sized,
    {
        Translate {
            item: Box::new(self),
            offset,
        }
    }
    fn rotate_y(self, angle: f64) -> YRotate
    where
        Self: 'static + Sized,
    {
        YRotate::new(Box::new(self), angle)
    }
}

/// A sphere that can be hit by a ray.
#[derive(Clone)]
pub struct Sphere {
    pub center: Vec3f<Position>,
    pub radius: f64,
    pub material: Arc<dyn Material>,
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
            front_face,
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
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            list: Vec::with_capacity(capacity),
        }
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
            return iter.try_fold(item, |accumulator, next| {
                next.bounding_box(initial_time, final_time)
                    .map(|x| x.surrounding(accumulator))
            });
        }
        None
    }
}

pub struct MovingSphere {
    pub center: Range<Vec3f<Position>>,
    pub time: Range<f64>,
    pub radius: f64,
    pub material: Box<dyn Material>,
}

impl MovingSphere {
    pub fn center(&self, time: f64) -> Vec3f<Position> {
        self.center.start
            + ((time - self.time.start) / (self.time.end - self.time.start))
                * (self.center.end - self.center.start)
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
            front_face,
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

impl From<List> for BvhTree {
    fn from(list: List) -> Self {
        Self::from_time(list, 0., 1.)
    }
}

impl BvhTree {
    pub fn from_time(mut list: List, initial_time: f64, final_time: f64) -> Self {
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

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PlaneCoordinates {
    pub axis0: Coordinate,
    pub axis1: Coordinate,
    pub k: Coordinate,
}

/// Trait representing a 2D Plane
pub trait Plane: Send + Sync {
    /// The coordinates of the 2 rectangles created by both points.
    /// The `f64`s correspond to the each rectangles's `k` side, one for each `Vec3f`.
    fn points(p0: Vec3f<Position>, p1: Vec3f<Position>) -> (Range<f64>, Range<f64>, f64, f64);

    /// The axes of the plane.
    /// ```
    /// let a = XY::axis();
    /// let b = PlaneCoordinates {
    ///     axis0: Coordinate::X,
    ///     axis1: Coordinate::Y,
    ///     k:  Coordinate::Z,
    /// };
    /// assert_eq!(a, b);
    fn axes() -> PlaneCoordinates;

    /// Create a rectangle in the plane. It's supposed to be used
    /// as a aid type inference when creating a new rectangle.
    fn rectangle<M: Material>(
        material: Arc<M>,
        p0: Range<f64>,
        p1: Range<f64>,
        k: f64,
    ) -> Rectangle<M, Self>
    where
        Self: std::marker::Sized,
    {
        Rectangle::new(material, p0, p1, k)
    }

    /// Create 2 rectangles in the plane, with different `k`.
    fn rectangles<M: Material>(
        p0: Vec3f<Position>,
        p1: Vec3f<Position>,
        material: &Arc<M>,
    ) -> (Rectangle<M, Self>, Rectangle<M, Self>)
    where
        Self: std::marker::Sized,
    {
        let (p0, p1, k0, k1) = Self::points(p0, p1);
        let r0 = Rectangle::new(material.clone(), p0.clone(), p1.clone(), k0);
        let r1 = Rectangle::new(material.clone(), p0, p1, k1);
        (r0, r1)
    }
}

// The whole plane thing is probably better implemented via an enum field.
// However, I like that the plane information is part of the type.
// Also, Peter Shirley creates a different rectangle class for each plane,
// making this solution quite similar.
/// A rectangle in a plane P
pub struct Rectangle<M: Material, P: Plane> {
    pub material: Arc<M>,
    pub p0: Range<f64>,
    pub p1: Range<f64>,
    // Must be non-zero, or else problems appear when creating the bounding box in the BVH.
    pub k: f64,
    _phantom: PhantomData<P>,
}

/// The XY-Plane.
pub struct Xy(());
/// The XZ-Plane.
pub struct Xz(());
/// The YZ-Plane.
pub struct Yz(());

impl Plane for Xy {
    fn points(p0: Vec3f<Position>, p1: Vec3f<Position>) -> (Range<f64>, Range<f64>, f64, f64) {
        (p0.x()..p1.x(), p0.y()..p1.y(), p0.z(), p1.z())
    }

    fn axes() -> PlaneCoordinates {
        PlaneCoordinates {
            axis0: Coordinate::X,
            axis1: Coordinate::Y,
            k: Coordinate::Z,
        }
    }
}
impl Plane for Xz {
    fn points(p0: Vec3f<Position>, p1: Vec3f<Position>) -> (Range<f64>, Range<f64>, f64, f64) {
        (p0.x()..p1.x(), p0.z()..p1.z(), p0.y(), p1.y())
    }

    fn axes() -> PlaneCoordinates {
        PlaneCoordinates {
            axis0: Coordinate::X,
            axis1: Coordinate::Z,
            k: Coordinate::Y,
        }
    }
}
impl Plane for Yz {
    fn points(p0: Vec3f<Position>, p1: Vec3f<Position>) -> (Range<f64>, Range<f64>, f64, f64) {
        (p0.y()..p1.y(), p0.z()..p1.z(), p0.x(), p1.x())
    }

    fn axes() -> PlaneCoordinates {
        PlaneCoordinates {
            axis0: Coordinate::Y,
            axis1: Coordinate::Z,
            k: Coordinate::X,
        }
    }
}

impl<M: Material, P: Plane> Rectangle<M, P> {
    pub fn new(material: Arc<M>, p0: Range<f64>, p1: Range<f64>, k: f64) -> Self {
        Self {
            material,
            p0,
            p1,
            k,
            _phantom: PhantomData::<P>,
        }
    }
}

impl<M: Material, P: Plane> Hittable for Rectangle<M, P> {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let PlaneCoordinates { axis0, axis1, k } = P::axes();
        let t = (self.k - ray.origin().at(k)) / ray.direction().at(k);
        if t < t_min || t > t_max {
            return None;
        }
        let p0 = ray.origin().at(axis0) + t * ray.direction().at(axis0);
        let p1 = ray.origin().at(axis1) + t * ray.direction().at(axis1);
        if !self.p0.contains(&p0) || !self.p1.contains(&p1) {
            return None;
        }

        let u = (p0 - self.p0.start) / (self.p0.end - self.p0.start);
        let v = (p1 - self.p1.start) / (self.p1.end - self.p1.start);
        let outward_normal = Vec3f::default().with_dimension(k, 1.);
        let (normal, front_face) = HitRecord::face_normal(ray, outward_normal);
        let p = ray.point_at_parameter(t);
        Some(HitRecord {
            t,
            p,
            normal,
            material: &*self.material,
            u,
            v,
            front_face,
        })
    }

    #[allow(unused_variables)]
    fn bounding_box(&self, initial_time: f64, final_time: f64) -> Option<Bound> {
        let PlaneCoordinates { axis0, axis1, k } = P::axes();
        let bound = Bound {
            min: Vec3f::default()
                .with_dimension(axis0, self.p0.start)
                .with_dimension(axis1, self.p1.start)
                // Must have non-zero value
                .with_dimension(k, self.k - 0.0001),
            max: Vec3f::default()
                .with_dimension(axis0, self.p0.end)
                .with_dimension(axis1, self.p1.end)
                .with_dimension(k, self.k + 0.0001),
        };
        Some(bound)
    }
}

pub struct Cube {
    box_min: Vec3f<Position>,
    box_max: Vec3f<Position>,
    sides: List,
}

impl Cube {
    pub fn new<T>(box_min: Vec3f<Position>, box_max: Vec3f<Position>, material: Arc<T>) -> Self
    where
        T: 'static + Material,
    {
        let mut sides = List::with_capacity(6);
        let xy = Xy::rectangles(box_min, box_max, &material);
        sides.push(xy.0);
        sides.push(xy.1);
        let xz = Xz::rectangles(box_min, box_max, &material);
        sides.push(xz.0);
        sides.push(xz.1);
        let yz = Yz::rectangles(box_min, box_max, &material);
        sides.push(yz.0);
        sides.push(yz.1);

        Self {
            box_min,
            box_max,
            sides,
        }
    }
}

impl Hittable for Cube {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.sides.hit(ray, t_min, t_max)
    }

    #[allow(unused_variables)]
    fn bounding_box(&self, initial_time: f64, final_time: f64) -> Option<Bound> {
        let bound = Bound {
            min: self.box_min,
            max: self.box_max,
        };
        Some(bound)
    }
}

pub struct Translate {
    pub item: Box<dyn Hittable>,
    pub offset: Vec3f<Position>,
}
impl Hittable for Translate {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let moved_ray = Ray {
            a: ray.origin() - self.offset,
            b: ray.direction(),
            time: ray.time,
        };

        if let Some(record) = self.item.hit(moved_ray, t_min, t_max) {
            let (normal, front_face) = HitRecord::face_normal(moved_ray, record.normal);
            Some(HitRecord {
                normal,
                front_face,
                p: record.p + self.offset,
                ..record
            })
        } else {
            None
        }
    }

    fn bounding_box(&self, initial_time: f64, final_time: f64) -> Option<Bound> {
        if let Some(bound) = self.item.bounding_box(initial_time, final_time) {
            Some(Bound {
                min: bound.min + self.offset,
                max: bound.max + self.offset,
            })
        } else {
            None
        }
    }
}

pub struct YRotate {
    item: Box<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    has_bound: bool,
    bound: Bound,
}

impl YRotate {
    pub fn new(item: Box<dyn Hittable>, angle: f64) -> Self {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();

        let (bound, has_bound) = if let Some(bound) = item.bounding_box(0., 1.) {
            (bound, true)
        } else {
            (Default::default(), false)
        };

        let mut min = Vec3f::repeat(f64::INFINITY);
        let mut max = Vec3f::repeat(f64::NEG_INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bound.max.x() + (1 - i) as f64 * bound.min.x();
                    let y = j as f64 * bound.max.y() + (1 - j) as f64 * bound.min.y();
                    let z = k as f64 * bound.max.z() + (1 - k) as f64 * bound.min.z();

                    let x = cos_theta * x + sin_theta * z;
                    let z = -sin_theta * x + cos_theta * z;

                    let tmp = Vec3f::<Position>::new(x, y, z);

                    for coord in 0..3 {
                        min[coord] = min[coord].min(tmp[coord]);
                        max[coord] = max[coord].max(tmp[coord]);
                    }
                }
            }
        }

        let bound = Bound { min, max };
        Self {
            bound,
            has_bound,
            item,
            sin_theta,
            cos_theta,
        }
    }
}

impl Hittable for YRotate {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut origin = ray.origin();
        let mut direction = ray.direction();
        origin[0] = self.cos_theta * ray.origin()[0] - self.sin_theta * ray.origin()[2];
        origin[2] = self.sin_theta * ray.origin()[0] + self.cos_theta * ray.origin()[2];
        direction[0] = self.cos_theta * ray.direction()[0] - self.sin_theta * ray.direction()[2];
        direction[2] = self.sin_theta * ray.direction()[0] + self.cos_theta * ray.direction()[2];
        let ray = Ray {
            a: origin,
            b: direction,
            time: ray.time,
        };

        if let Some(mut record) = self.item.hit(ray, t_min, t_max) {
            record.p[0] = self.cos_theta * record.p[0] + self.sin_theta * record.p[2];
            record.p[2] = -self.sin_theta * record.p[0] + self.cos_theta * record.p[2];
            record.normal[0] =
                self.cos_theta * record.normal[0] + self.sin_theta * record.normal[2];
            record.normal[2] =
                -self.sin_theta * record.normal[0] + self.cos_theta * record.normal[2];
            let (normal, front_face) = HitRecord::face_normal(ray, record.normal);

            Some(HitRecord {
                normal,
                front_face,
                ..record
            })
        } else {
            None
        }
    }

    #[allow(unused_variables)]
    fn bounding_box(&self, initial_time: f64, final_time: f64) -> Option<Bound> {
        Some(self.bound)
    }
}

pub struct ConstantMedium {
    boundary: Arc<dyn Hittable>,
    phase_function: Isotropic,
    neg_inv_density: f64,
}

impl ConstantMedium {
    pub fn new(boundary: Arc<dyn Hittable>, density: f64, phase_function: Arc<dyn Texture>) -> Self {
        Self {
            boundary, phase_function: Isotropic { albedo: phase_function }, neg_inv_density: -1./density
        }
    }
}

impl Hittable for ConstantMedium {
    // Current implementation assumes the shape is convex
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut rng = rand::thread_rng();
        // Print occasional samples when debugging. To enable, set enableDebug true.
        let _enable_debug = false;
        let debugging = _enable_debug && rng.gen::<f64>() < 0.00001;
        if let Some(mut record1) =
            self.boundary
                .hit(ray, f64::NEG_INFINITY, f64::INFINITY)
        {
            if let Some(mut record2) =
                self.boundary
                    .hit(ray, record1.t + 0.0001, f64::INFINITY)
            {
                if debugging {
                    eprintln!("\nt_min = {}, t_max = {}", record1.t, record2.t);
                }
                record1.t = record1.t.max(t_min);
                record2.t = record2.t.min(t_max);
                if record1.t >= record2.t {
                    return None;
                }
                record1.t = record1.t.max(0.);

                let ray_length = ray.direction().magnitude();
                let distance_inside_boundary = (record2.t - record1.t) * ray_length;
                let hit_distance = self.neg_inv_density * rng.gen::<f64>().ln();

                if hit_distance > distance_inside_boundary {
                    return None;
                }
                let t = record1.t + hit_distance / ray_length;
                let p = ray.point_at_parameter(t);
                if debugging {
                    eprintln!(
                        "hit_distance = {}\nrec.t = {}\nrec.p {}",
                        hit_distance, t, p
                    );
                }
                let normal = Vec3f::new(1., 0., 0.); // Arbitrary
                let front_face = true; // Arbitrary

                Some(HitRecord {
                    t,
                    p,
                    normal,
                    front_face,
                    material: &self.phase_function,
                    u: 0.0,
                    v: 0.0,
                })
            } else {
                None
            }
        } else {
            None
        }
    }

    fn bounding_box(&self, initial_time: f64, final_time: f64) -> Option<Bound> {
        self.boundary.bounding_box(initial_time, final_time)
    }
}
