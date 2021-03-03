use std::fs::File;
use std::io::{BufWriter, Write};

use rand::Rng;

mod math;
use math::{
    Camera, Color, Dielectric, Hittable, Lambertian, List, Metal, Position, Ray, Sphere, Vec3f,
};

/// The resulting color of a ray pointing to a direction
fn color<T: Hittable>(ray: Ray, world: &List<T>, depth: i32) -> Vec3f<Color> {
    // If the ray hits something
    // `t_min` is not 0.0 to avoid the shadow acne problem
    if let Some(record) = world.hit(ray, 0.001, f32::MAX) {
        // New random point at a random direction. Where the ray is reflected.
        return if depth >= 50 {
            Vec3f::repeat(0.0)
        } else if let Some((attenuation, scattered)) = record.material.scatter(ray, record) {
            attenuation * color(scattered, world, depth + 1)
        } else {
            Vec3f::repeat(0.0)
        };
    }
    // Else return the horizont, blue -> white gradient
    let direction = ray.direction().unit();
    // Scale it between `0.0 < t < 1.0`
    let t = (direction.y() + 1.0) * 0.5;
    // (1.0 - t)*WHITE + t*BLUE
    (1.0 - t) * Vec3f::repeat(1.0) + t * Vec3f::new(0.5, 0.7, 1.0)
}

/// Saves the scene to a .ppm image of size `nx*ny`
fn print_result(nx: isize, ny: isize, ns: isize) {
    let output = File::create("image.ppm").unwrap();
    let mut output = BufWriter::new(output);
    let lower_left_corner: Vec3f<Position> = (-2.0, -1.0, -1.0).into();
    // Canvas width
    let horizontal: Vec3f<Position> = (4.0, 0.0, 0.0).into();
    // Canvas height
    let vertical: Vec3f<Position> = (0.0, 2.0, 0.0).into();
    // Camera eye
    let origin: Vec3f<Position> = (0.0, 0.0, 0.0).into();
    let camera = Camera {
        origin,
        horizontal,
        vertical,
        lower_left_corner,
    };

    let world = List {
        list: vec![
            Sphere {
                center: (0.0, 0.0, -1.0).into(),
                radius: 0.5,
                material: Lambertian::boxed((0.8, 0.3, 0.3).into()),
            },
            Sphere {
                center: (0.0, -100.5, -1.0).into(),
                radius: 100.0,
                material: Lambertian::boxed((0.8, 0.8, 0.0).into()),
            },
            Sphere {
                center: (1.0, 0.0, -1.0).into(),
                radius: 0.5,
                material: Metal::boxed((0.8, 0.6, 0.2).into(), 1.0),
            },
            Sphere {
                center: (-1.0, 0.0, -1.0).into(),
                radius: 0.5,
                material: Dielectric::boxed(1.5),
            },
            Sphere {
                center: (-1.0, 0.0, -1.0).into(),
                radius: -0.45,
                material: Dielectric::boxed(1.5),
            },
        ],
    };
    let mut rng = rand::thread_rng();
    output
        .write_all(format!("P3\n{} {}\n255\n", nx, ny).as_bytes())
        .unwrap();
    // For each pixel
    for j in (0..ny).rev() {
        for i in 0..nx {
            // Calculate the color `ns` times and average the result
            let mut col = Vec3f::repeat(0.0);
            for _ in 0..ns {
                let u = (i as f32 + rng.gen::<f32>()) / nx as f32;
                let v = (j as f32 + rng.gen::<f32>()) / ny as f32;
                let ray = camera.ray(u, v);
                col = col + color(ray, &world, 0);
            }
            col = col / ns as f32;
            // Gamma correction
            let col = col.map(|x| x.sqrt());
            // Scale it from 0..1 to 0..255
            let ir = (255.99 * col.x()) as u8;
            let ig = (255.99 * col.y()) as u8;
            let ib = (255.99 * col.z()) as u8;
            output
                .write_all(format!("{} {} {}\n", ir, ig, ib).as_bytes())
                .unwrap();
        }
    }
}

fn main() {
    print_result(200, 100, 100);
}
