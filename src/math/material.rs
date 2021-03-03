use crate::math::vec3::Phantom;
use crate::math::{Color, HitRecord, Ray, Vec3f};

pub trait Material {
    // fn scatter(ray: &Ray, record: &HitRecord, attenuation: &mut Vec3f<Position>, scattered: &mut Ray) -> bool;
    fn scatter(&self, ray: Ray, record: HitRecord) -> Option<(Vec3f<Color>, Ray)>;
}

#[derive(Copy, Clone)]
pub struct Lambertian {
    albedo: Vec3f<Color>,
}

impl Lambertian {
    #[allow(dead_code)]
    pub fn new(albedo: Vec3f<Color>) -> Self {
        Self { albedo }
    }
    pub fn boxed(albedo: Vec3f<Color>) -> Box<Self> {
        Box::new(Self { albedo })
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: Ray, record: HitRecord) -> Option<(Vec3f<Color>, Ray)> {
        let target = record.p + record.normal + Vec3f::random_in_unit_space();
        let scattered = Ray {
            a: record.p,
            b: target - record.p,
        };
        let attenuation = self.albedo;
        Some((attenuation, scattered))
    }
}

#[derive(Copy, Clone)]
pub struct Metal {
    albedo: Vec3f<Color>,
    fuzz: f32,
}

impl Metal {
    #[allow(dead_code)]
    pub fn new(albedo: Vec3f<Color>, fuzz: f32) -> Self {
        Self {
            albedo,
            fuzz: fuzz.min(1.0),
        }
    }

    pub fn boxed(albedo: Vec3f<Color>, fuzz: f32) -> Box<Self> {
        Box::new(Self {
            albedo,
            fuzz: fuzz.min(1.0),
        })
    }
    pub fn reflect<T: Phantom>(v: Vec3f<T>, n: Vec3f<T>) -> Vec3f<T> {
        v - 2.0 * v.dot(n) * n
    }
}

impl Material for Metal {
    fn scatter(&self, ray: Ray, record: HitRecord) -> Option<(Vec3f<Color>, Ray)> {
        let reflected = Self::reflect(ray.direction().unit(), record.normal);
        let scattered = Ray {
            a: record.p,
            b: reflected + self.fuzz * Vec3f::random_in_unit_space(),
        };
        let attenuation = self.albedo;
        if scattered.direction().dot(record.normal) > 0.0 {
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}
