use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};

mod math;
mod scenes;

use math::{Camera, CameraDescriptor, Color, Hittable, List, Ray, Vec3f};
use std::error::Error;

// use image::Rgba;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct Rgba {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

/// The resulting color of a ray pointing to a direction
fn color(ray: Ray, world: &List, depth: i32) -> Vec3f<Color> {
    // If the ray hits something
    // `t_min` is not 0.0 to avoid the shadow acne problem
    if let Some(record) = world.hit(ray, 0.001, f64::MAX) {
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

/// Saves the scene to a .png image of size `nx*ny`
fn print_result(width: u32, aspect_ratio: f64, samples: usize, scene: usize) {
    let height = (width as f64 / aspect_ratio) as u32;
    let (world, lookfrom, lookat, vertical_fov, aperture) = match scene {
        1 => {
            println!("Running scene random_scene");
            (
                scenes::random_scene(),
                Vec3f::new(13.0, 2.0, 3.0),
                Vec3f::repeat(0.0),
                20.0,
                0.1,
            )
        }
        2 => {
            println!("Running scene two_spheres");
            (
                scenes::two_spheres(),
                Vec3f::new(13.0, 2.0, 3.0),
                Vec3f::repeat(0.0),
                20.0,
                Default::default(),
            )
        }
        3 => {
            println!("Running scene two_perlin_spheres");
            (
                scenes::two_perlin_spheres(),
                Vec3f::new(13.0, 2.0, 3.0),
                Vec3f::repeat(0.0),
                20.0,
                Default::default(),
            )
        }
        4 => {
            println!("Running scene earth");
            (
                scenes::earth(),
                Vec3f::new(13.0, 2.0, 3.0),
                Vec3f::repeat(0.0),
                20.0,
                Default::default(),
            )
        }
        _ => panic!("Wrong scene"),
    };
    let view_up = Vec3f::new(0.0, 1.0, 0.0);
    let focus_distance = 10.0;
    let camera = Camera::new(&CameraDescriptor {
        lookfrom,
        lookat,
        view_up,
        vertical_fov,
        aspect_ratio,
        aperture,
        focus_distance,
        open_time: 0.0,
        close_time: 1.0,
    });

    let progress = ProgressBar::new(height as u64)
        .with_style(ProgressStyle::default_spinner().template("{pos}/{len} {spinner:.dim.bold}"));
    // For each pixel
    // let image: Vec<u8> =
    let image: Vec<Rgba> = /*Vec::with_capacity(4 * width * height);*/
        (0..height)
        // .into_iter()
        .into_par_iter()
        .rev()
        // .progress()
        .progress_with(progress)
        // .progress_count(height as u64)
        .flat_map(|j| {
            (0..width)
                .into_par_iter()
                .map(|i| {
                    // Calculate the color `ns` times and average the result
                    let col = (0..samples)
                        .into_par_iter()
                        .fold(
                            || Vec3f::<Color>::repeat(0.0),
                            |acc, _| {
                                let mut rng = SmallRng::from_entropy();
                                let u = (i as f64 + rng.gen::<f64>()) / width as f64;
                                let v = (j as f64 + rng.gen::<f64>()) / height as f64;
                                let ray = camera.ray(u, v);
                                acc + color(ray, &world, 0)
                            },
                        )
                        .sum::<Vec3f<Color>>()
                        / samples as f64;
                    // Gamma correction
                    let col = col.map(|x| x.sqrt() * 255.99);
                    Rgba {
                        r: col.x() as u8,
                        g: col.y() as u8,
                        b: col.z() as u8,
                        a: 255,
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect();
    let buffer: &[u8] = bytemuck::cast_slice(&image);
    image::save_buffer("image.png", buffer, width, height, image::ColorType::Rgba8).unwrap()
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    let scene = args.get(1).unwrap_or(&String::from("1")).parse()?;
    println!("Scene number: {}", scene);
    let instant = std::time::Instant::now();
    print_result(400, 16.0 / 9.0, 100, scene);
    println!("{:?}", instant.elapsed());
    Ok(())
}
