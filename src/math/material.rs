use crate::math::{Color, HitRecord, Position, Ray, Texture, Vec3f};
use rand::Rng;
use std::sync::Arc;

/// Different materials scatter light differently
pub trait Material: Send + Sync {
    fn scatter(&self, ray: Ray, record: HitRecord) -> Option<(Vec3f<Color>, Ray)>;

    #[allow(unused_variables)]
    fn emitted(&self, u: f64, v: f64, point: Vec3f<Position>) -> Vec3f<Color> {
        Vec3f::repeat(0.)
    }
}

/// Solid material
#[derive(Clone)]
pub struct Lambertian<T: Texture> {
    albedo: Arc<T>,
}

impl<T> From<T> for Lambertian<T>
where
    T: 'static + Texture,
{
    fn from(albedo: T) -> Self {
        Self {
            albedo: Arc::new(albedo),
        }
    }
}

#[allow(dead_code)]
impl<T: Texture> Lambertian<T> {
    pub fn new<A>(albedo: A) -> Self
    where
        T: 'static,
        A: Into<Arc<T>>,
    {
        let albedo = albedo.into();
        Self { albedo }
    }
    /// Create a Lambertian wrapped in a box
    /// TODO: Is there a way to help the compiler infer the type T? Example:
    /// ```
    /// pub fn two_spheres() -> List {
    ///     let mut world = List::new();
    ///     let checker = Arc::new(CheckerTexture {
    ///         odd: Arc::new(Vec3f::new(0.2, 0.3, 0.1)),
    ///         even: Arc::new(Vec3f::new(0.9, 0.9, 0.9)),
    ///     });
    ///     world.push(Sphere {
    ///         center: Vec3f::new(0.0, -10.0, 0.0),
    ///         radius: 10.0,
    ///         material: Lambertian::<CheckerTexture>::boxed(checker.clone()),
    ///     });
    ///     world.push(Sphere {
    ///         center: Vec3f::new(0.0, 10.0, 0.0),
    ///         radius: 10.0,
    ///         material: Lambertian::<CheckerTexture>::boxed(checker),
    ///     });
    ///
    ///     world
    /// }
    /// ```
    pub fn boxed<A>(albedo: A) -> Box<Self>
    where
        T: 'static,
        A: Into<Arc<T>>,
    {
        Box::new(Self::new(albedo))
    }
    pub fn arc<A>(albedo: A) -> Arc<Self>
    where
        T: 'static,
        A: Into<Arc<T>>,
    {
        Arc::new(Self::new(albedo))
    }
}

impl<T: Texture> Material for Lambertian<T> {
    fn scatter(&self, ray: Ray, record: HitRecord) -> Option<(Vec3f<Color>, Ray)> {
        let target = record.p + record.normal + Vec3f::random_in_unit_space();
        let scattered = Ray {
            a: record.p,
            b: target - record.p,
            time: ray.time,
        };
        let attenuation = self.albedo.value(record.u, record.v, record.p);
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
#[derive(Copy, Clone)]
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
        let mut rng = rand::thread_rng();
        let reflected = ray.direction().reflect(record.normal);
        // Attenuation is 1 because glass absorbs nothing
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

#[derive(Clone)]
pub struct DiffuseLight {
    emit: Arc<dyn Texture>,
}

impl<T> From<T> for DiffuseLight
where
    T: 'static + Texture,
{
    fn from(albedo: T) -> Self {
        Self {
            emit: Arc::new(albedo),
        }
    }
}

impl DiffuseLight {
    #[allow(dead_code)]
    pub fn new<T: 'static + Texture>(albedo: &Arc<T>) -> Self {
        Self {
            emit: albedo.clone(),
        }
    }
    #[allow(dead_code)]
    pub fn boxed<T: 'static + Texture>(albedo: T) -> Box<Self> {
        Box::new(Self {
            emit: Arc::new(albedo),
        })
    }
    pub fn arc<T: 'static + Texture>(albedo: T) -> Arc<Self> {
        Arc::new(Self {
            emit: Arc::new(albedo),
        })
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _: Ray, _: HitRecord) -> Option<(Vec3f<Color>, Ray)> {
        None
    }

    fn emitted(&self, u: f64, v: f64, point: Vec3f<Position>) -> Vec3f<Color> {
        self.emit.value(u, v, point)
    }
}
