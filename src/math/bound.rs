use super::{Position, Ray, Vec3f};

/// Axis-aligned bounding box
#[derive(Copy, Clone, Default)]
pub struct Bound {
    pub min: Vec3f<Position>,
    pub max: Vec3f<Position>,
}

impl Bound {
    /// Andrew Kensler's version of the hit detection.
    /// http://psgraphics.blogspot.com/2016/02/new-simple-ray-box-test-from-andrew.html
    pub fn hit(&self, ray: Ray, mut min: f64, mut max: f64) -> bool {
        for dimension in 0..3 {
            let inverse_direction = 1.0 / ray.direction().at(dimension);
            let (t0, t1) = {
                let t0 = (self.min.at(dimension) - ray.origin().at(dimension)) * inverse_direction;
                let t1 = (self.max.at(dimension) - ray.origin().at(dimension)) * inverse_direction;
                if inverse_direction < 0.0 {
                    (t1, t0)
                } else {
                    (t0, t1)
                }
            };
            min = t0.max(min);
            max = t1.max(max);
            if max <= min {
                return false;
            }
        }
        true
    }

    pub fn surrounding(&self, other: Self) -> Self {
        let min = Vec3f::new(
            self.min.x().min(other.min.x()),
            self.min.y().min(other.min.y()),
            self.min.z().min(other.min.z()),
        );
        let max = Vec3f::new(
            self.max.x().min(other.max.x()),
            self.max.y().min(other.max.y()),
            self.max.z().min(other.max.z()),
        );
        Self { min, max }
    }
}
