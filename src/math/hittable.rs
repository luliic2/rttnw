use super::{Position, Ray, Vec3f};

#[derive(Clone, Copy)]
pub struct HitRecord {
    pub t: f32,
    pub p: Vec3f<Position>,
    pub normal: Vec3f<Position>,
}
impl HitRecord {
    pub fn new() -> Self {
        Self {
            t: 0.0,
            p: (0, 0, 0).into(),
            normal: (0, 0, 0).into(),
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, record: &mut HitRecord) -> bool;
}

pub struct Sphere {
    pub center: Vec3f<Position>,
    pub radius: f32,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, record: &mut HitRecord) -> bool {
        let oc = ray.origin() - self.center;
        let a = ray.direction().dot(&ray.direction());
        let b = oc.dot(&ray.direction());
        let c = oc.dot(&oc) - self.radius.powf(2.0);
        let discriminant = b.powf(2.0) - a * c;
        if discriminant > 0.0 {
            let temp = (-b - (b.powf(2.0) - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                record.t = temp;
                record.p = ray.point_at_parameter(temp);
                record.normal = (record.p - self.center) / self.radius;
                return true;
            }
            let temp = (-b + (b.powf(2.0) - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                record.t = temp;
                record.p = ray.point_at_parameter(temp);
                record.normal = (record.p - self.center) / self.radius;
                return true;
            }
        }
        false
    }
}

pub struct List<T> {
    pub list: Vec<T>,
}

impl<T: Hittable> Hittable for List<T> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, record: &mut HitRecord) -> bool {
        let mut tmp_record = HitRecord::new();
        let mut did_hit = false;
        let mut closest = t_max;
        for i in &self.list {
            if i.hit(&ray, t_min, closest, &mut tmp_record) {
                did_hit = true;
                closest = tmp_record.t;
                *record = tmp_record;
            }
        }
        did_hit
    }
}
