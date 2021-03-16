use super::{Color, Perlin, Position, Vec3f};
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
            * (1.
                + f64::sin(
                    self.scale * point.z()
                        + 10.
                            * self
                                .noise
                                .turbulence(point, 7),
                ))
    }
}
