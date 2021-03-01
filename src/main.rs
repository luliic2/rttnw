use std::fs::File;
use std::io::{BufWriter, Write};

mod math;
use math::{Color, Vec3f, Ray, Position};

fn color(ray: &Ray) -> Vec3f<Color> {
    let direction = ray.direction().unit();
    // Scale it between `0.0 < t < 1.0`
    let t = (direction.y() + 1.0) * 0.5;
    // (1.0 - t)*WHITE + t*BLUE
    (1.0 - t) * Vec3f::repeat(1.0) + t * Vec3f::new([0.5, 0.7, 1.0])
}

fn print_result(nx: isize, ny: isize) {
    let output = File::create("image.ppm").unwrap();
    let mut output = BufWriter::new(output);
    let lower_left_corner: Vec3f<Position> = (-2.0, -1.0, -1.0).into();
    // Canvas width
    let horizontal: Vec3f<Position> = (4.0, 0.0, 0.0).into();
    // Canvas height
    let vertical: Vec3f<Position> = (0.0, 2.0, 0.0).into();
    // Camera eye
    let origin: Vec3f<Position> = (0.0, 0.0, 0.0).into();

    output
        .write_all(format!("P3\n{} {}\n255\n", nx, ny).as_bytes())
        .unwrap();
    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;
            let ray = Ray {
                a: origin,
                b: lower_left_corner + u*horizontal + v*vertical
            };
            let color = color(&ray);
            let ir = (255.99 * color.x()) as u8;
            let ig = (255.99 * color.y()) as u8;
            let ib = (255.99 * color.z()) as u8;
            output
                .write_all(format!("{} {} {}\n", ir, ig, ib).as_bytes())
                .unwrap();
        }
    }
}

fn main() {
    print_result(200, 100);
}
