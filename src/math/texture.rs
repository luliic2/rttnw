use super::{Color, Position, Vec3f};
use std::sync::Arc;

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, point: Vec3f<Position>) -> Vec3f<Color>;
}

impl Texture for Vec3f<Color> {
    fn value(&self, _: f64, _: f64, _: Vec3f<Position>) -> Vec3f<Color> {
        *self
    }
}

pub struct Checker {
    pub odd: Arc<dyn Texture>,
    pub even: Arc<dyn Texture>,
}

impl Texture for Checker {
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
