use super::{Color, Perlin, Position, Vec3f};
use std::path::Path;
use std::sync::Arc;

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, point: Vec3f<Position>) -> Vec3f<Color>;
}

impl Texture for Vec3f<Color> {
    fn value(&self, _: f64, _: f64, _: Vec3f<Position>) -> Vec3f<Color> {
        *self
    }
}

pub struct CheckerTexture {
    pub odd: Arc<dyn Texture>,
    pub even: Arc<dyn Texture>,
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, point: Vec3f<Position>) -> Vec3f<Color> {
        let sines =
            f64::sin(10.0 * point.x()) * f64::sin(10.0 * point.y()) * f64::sin(10.0 * point.z());
        if sines < 0.0 {
            self.odd.value(u, v, point)
        } else {
            self.even.value(u, v, point)
        }
    }
}

pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            noise: Perlin::new(),
            scale: 1.0,
        }
    }
    pub fn scaled(scale: f64) -> Self {
        Self {
            noise: Perlin::new(),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _: f64, _: f64, point: Vec3f<Position>) -> Vec3f<Color> {
        Vec3f::repeat(1.0)
            * 0.5
            * (1. + f64::sin(self.scale * point.z() + 10. * self.noise.turbulence(point, 7)))
    }
}

use image::io::Reader;
use image::RgbaImage;
pub struct ImageTexture {
    data: Option<RgbaImage>,
}

impl ImageTexture {
    pub fn new<T: AsRef<Path>>(file: T) -> Self {
        let data = Reader::open(file)
            .ok()
            .and_then(|x| x.decode().map(|x| x.to_rgba8()).ok());
        Self { data }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _: Vec3f<Position>) -> Vec3f<Color> {
        if let Some(data) = &self.data {
            let u = u.clamp(0., 1.);
            let v = 1. - v.clamp(0., 1.);

            let (i, j) = {
                let mut i = (u * data.width() as f64) as u32;
                let mut j = (v * data.height() as f64) as u32;

                // Clamp integer mapping. The actual coordinates should be < 1.0
                if i >= data.width() {
                    i = data.width() - 1;
                }
                if j >= data.height() {
                    j = data.height() - 1;
                }

                (i, j)
            };

            let color_scale = 1.0 / 255.0;
            let pixel = data.get_pixel(i, j).0;

            Vec3f::scaled(&pixel, color_scale)
        } else {
            // If an image does not load return cyan
            Vec3f::new(0., 1., 1.)
        }
    }
}
