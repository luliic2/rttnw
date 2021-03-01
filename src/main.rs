use std::fs::File;
use std::io::{BufWriter, Write};

fn print_result(nx: isize, ny: isize) {
    let output = File::create("image.ppm").unwrap();
    let mut output = BufWriter::new(output);

    output
        .write(format!("P3\n{} {}\n255\n", nx, ny).as_bytes())
        .unwrap();
    for j in (0..ny).rev() {
        for i in 0..nx {
            let r = i as f32 / nx as f32;
            let g = j as f32 / ny as f32;
            let b = 0.2f32;
            let ir = (255.99 * r) as u8;
            let ig = (255.99 * g) as u8;
            let ib = (255.99 * b) as u8;
            output
                .write(format!("{} {} {}\n", ir, ig, ib).as_bytes())
                .unwrap();
        }
    }
}

fn main() {
    print_result(200, 100);
}
