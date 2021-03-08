use crate::math::{Color, HitRecord, Ray, Vec3f};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

/// Different materials scatter light differently
pub trait Material: Send + Sync {
    fn scatter(&self, ray: Ray, record: HitRecord) -> Option<(Vec3f<Color>, Ray)>;
}

/// Solid material
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
    fn scatter(&self, ray: Ray, record: HitRecord) -> Option<(Vec3f<Color>, Ray)> {
        let target = record.p + record.normal + Vec3f::random_in_unit_space();
        let scattered = Ray {
            a: record.p,
            b: target - record.p,
            time: ray.time,
        };
        let attenuation = self.albedo;
        Some((attenuation, scattered))
    }
}

/// Metalic material
#[derive(Copy, Clone)]
pub struct Metal {
    albedo: Vec3f<Color>,
    fuzz: f64,
}

impl Metal {
    #[allow(dead_code)]
    pub fn new(albedo: Vec3f<Color>, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: fuzz.min(1.0),
        }
    }

    pub fn boxed(albedo: Vec3f<Color>, fuzz: f64) -> Box<Self> {
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
            time: ray.time,
        };
        let attenuation = self.albedo;
        if scattered.direction().dot(record.normal) > 0.0 {
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}

/// Glass material
pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    #[allow(dead_code)]
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }

    pub fn boxed(refraction_index: f64) -> Box<Self> {
        Box::new(Self { refraction_index })
    }

    fn schlick(cosine: f64, refraction_index: f64) -> f64 {
        let r0 = ((1.0 - refraction_index) / (1.0 + refraction_index)).powf(2.0);
        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: Ray, record: HitRecord) -> Option<(Vec3f<Color>, Ray)> {
        let mut rng = SmallRng::from_entropy();
        let reflected = ray.direction().reflect(record.normal);
        // Attenuation is 1 because glass absorbs nothing
        // Kill the blue (z) channel
        let attenuation = Vec3f::new(1.0, 1.0, 1.0);
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
        let (refracted, reflect_probability) =
            if let Some(refracted) = ray.direction().refract(outward_normal, ni_over_nt) {
                (refracted, Self::schlick(cosine, self.refraction_index))
            } else {
                (Default::default(), 1.0)
            };
        Some((
            attenuation,
            Ray {
                a: record.p,
                b: if reflect_probability > rng.gen() {
                    reflected
                } else {
                    refracted
                },
                time: ray.time,
            },
        ))
    }
}
