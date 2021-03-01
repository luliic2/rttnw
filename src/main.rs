use std::fs::File;
use std::io::{BufWriter, Write};

mod math;
use math::{Color, Vec3f};

fn print_result(nx: isize, ny: isize) {
    let output = File::create("image.ppm").unwrap();
    let mut output = BufWriter::new(output);

    output
        .write_all(format!("P3\n{} {}\n255\n", nx, ny).as_bytes())
        .unwrap();
    for j in (0..ny).rev() {
        for i in 0..nx {
            let color: Vec3f<Color> = (i as f32 / nx as f32, j as f32 / ny as f32, 0.2).into();
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
