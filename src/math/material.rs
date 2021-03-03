use rand::Rng;
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
}

impl Material for Metal {
    fn scatter(&self, ray: Ray, record: HitRecord) -> Option<(Vec3f<Color>, Ray)> {
        let reflected = ray.direction().unit().reflect(record.normal);
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

pub struct Dielectric {
    refraction_index: f32,
}

impl Dielectric {
    #[allow(dead_code)]
    pub fn new(refraction_index: f32) -> Self {
        Self { refraction_index }
    }

    pub fn boxed(refraction_index: f32) -> Box<Self> {
        Box::new(Self { refraction_index })
    }

    fn schlick(cosine: f32, refraction_index: f32) -> f32 {
        let r0 = ((1.0 - refraction_index) / (1.0 + refraction_index)).powf(2.0);
        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: Ray, record: HitRecord) -> Option<(Vec3f<Color>, Ray)> {
        let mut rng = rand::thread_rng();
        let reflected = ray.direction().reflect(record.normal);
        // Attenuation is 1 because glass absorbs nothing
        // Kill the blue (z) channel
        let attenuation = Vec3f::new(1.0, 1.0, 0.0);
        let (outward_normal, ni_over_nt, cosine) = if ray.direction().dot(record.normal) > 0.0 {
            (
                -record.normal,
                self.refraction_index,
                self.refraction_index * ray.direction().dot(record.normal)
                    / ray.direction().magnitude(),
            )
        } else {
            (
                record.normal,
                1.0 / self.refraction_index,
                -ray.direction().dot(record.normal) / ray.direction().magnitude(),
            )
        };
        let (refracted, reflect_probability) =  if let Some(refracted) = ray.direction().refract(outward_normal, ni_over_nt) {
            (refracted, Self::schlick(cosine, self.refraction_index))
        } else {
            (Default::default(), 1.0)
        };
        Some((attenuation, Ray {
            a: record.p,
            b: if reflect_probability > rng.gen()  {
                reflected
            } else { refracted }
        }))
    }
}
