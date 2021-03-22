use super::{Position, Vec3f};
use rand::prelude::SliceRandom;
use rand::Rng;

pub struct Perlin {
    random_points: Vec<Vec3f<Position>>,
    x: Vec<usize>,
    y: Vec<usize>,
    z: Vec<usize>,
}

impl Perlin {
    const POINT_COUNT: usize = 256;

    fn perlin_generate() -> Vec<Vec3f<Position>> {
        (0..Self::POINT_COUNT)
            .map(|_| Vec3f::random(-1. ..1.))
            .collect()
    }

    fn generate_permutation() -> Vec<usize> {
        let mut rng = rand::thread_rng();
        let mut points: Vec<usize> = (0..Self::POINT_COUNT).collect();

        points.shuffle(&mut rng);
        // Self::permute(&mut points);

        points
    }

    #[allow(dead_code)]
    fn permute(points: &mut [usize]) {
        let mut rng = rand::thread_rng();
        for i in (0..points.len()).rev() {
            let target = rng.gen_range(0..i + 1);
            points.swap(i, target);
        }
    }

    pub fn new() -> Self {
        Self {
            random_points: Self::perlin_generate(),
            x: Self::generate_permutation(),
            y: Self::generate_permutation(),
            z: Self::generate_permutation(),
        }
    }

    #[allow(clippy::many_single_char_names)]
    pub fn noise(&self, point: Vec3f<Position>) -> f64 {
        let u = point.x() - point.x().floor();
        let v = point.y() - point.y().floor();
        let w = point.z() - point.z().floor();

        // Ideally, i, j and k should be of type usize, making indexing simpler later.
        // However, in the resulting noise appear strange artifacts
        // (check `bad_images/perlin.png`) and I don't understand why.
        // Update: Because there can be negative coordinates, duh.
        let i = point.x().floor() as i32;
        let j = point.y().floor() as i32;
        let k = point.z().floor() as i32;
        let mut c = [[[Vec3f::default(); 2]; 2]; 2]; // Vec3f c[2][2][2];
        for (di, item) in c.iter_mut().enumerate() {
            for (dj, item) in item.iter_mut().enumerate() {
                for (dk, item) in item.iter_mut().enumerate() {
                    let x = self.x[((i + di as i32) & 255) as usize];
                    let y = self.y[((j + dj as i32) & 255) as usize];
                    let z = self.z[((k + dk as i32) & 255) as usize];
                    *item = self.random_points[x ^ y ^ z];
                }
            }
        }

        Self::perlin_interpolation(c, u, v, w)
    }

    fn perlin_interpolation(c: [[[Vec3f<Position>; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3. - 2. * u);
        let vv = v * v * (3. - 2. * v);
        let ww = w * w * (3. - 2. * w);
        let mut accumulator = 0.0;
        for (i, item) in c.iter().enumerate() {
            for (j, item) in item.iter().enumerate() {
                for (k, item) in item.iter().enumerate() {
                    let weight = Vec3f::new(u - i as f64, v - j as f64, w - k as f64);
                    accumulator += (i as f64 * uu + (1 - i) as f64 * (1. - uu))
                        * (j as f64 * vv + (1 - j) as f64 * (1. - vv))
                        * (k as f64 * ww + (1 - k) as f64 * (1. - ww))
                        * item.dot(weight);
                }
            }
        }
        accumulator
    }

    pub fn turbulence(&self, point: Vec3f<Position>, depth: u32) -> f64 {
        if depth == 0 {
            return 0.;
        }
        let (accumulator, _temp, _weight) = (0..depth).fold((0.0, point, 1.0), |accumulator, _| {
            let accum = accumulator.0 + accumulator.2 * self.noise(accumulator.1);
            let weight = accumulator.2 * 0.5;
            let temp = accumulator.1 * 2.0;

            (accum, temp, weight)
        });
        accumulator
    }
}
