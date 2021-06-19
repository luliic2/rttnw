//! Raytracing: The Next Week implementation in Rust.
//! Original book by Peter Shirley [here](https://raytracing.github.io/books/RayTracingTheNextWeek.html)

use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use rand::Rng;
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};

mod math;
mod scenes;

use crate::math::Position;
#[allow(unused_imports)]
use math::{BvhTree, Camera, CameraDescriptor, Color, Hittable, List, Ray, Vec3f};
use std::error::Error;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct Rgba {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

/// The resulting color of a ray pointing to a direction
fn color<T: Hittable>(ray: Ray, background: Vec3f<Color>, world: &T, depth: i32) -> Vec3f<Color> {
    // If the ray bounce limit is reached, no more light is gathered.
    if depth <= 0 {
        return Vec3f::repeat(0.);
    }
    // If the ray hits something
    // `t_min` is not 0.0 to avoid the shadow acne problem
    if let Some(record) = world.hit(ray, 0.001, f64::MAX) {
        let emitted = record.material.emitted(record.u, record.v, record.p);

        // New random point at a random direction. Where the ray is reflected.
        if let Some((attenuation, scattered)) = record.material.scatter(ray, record) {
            emitted + attenuation * color(scattered, background, world, depth - 1)
        } else {
            emitted
        }
    } else {
        background
    }
}

#[derive(Default)]
struct Scene {
    background: Vec3f<Color>,
    world: List,
    lookfrom: Vec3f<Position>,
    lookat: Vec3f<Position>,
    vertical_fov: f64,
    aperture: f64,
}

/// Saves the scene to a .png image of size `nx*ny`
fn render(mut width: u32, mut aspect_ratio: f64, mut samples: usize, scene: usize) -> Option<()> {
    let Scene {
        background,
        world,
        lookfrom,
        lookat,
        vertical_fov,
        aperture,
    } = match scene {
        1 => {
            println!("Running scene random_scene");
            Scene {
                background: Vec3f::new(0.7, 0.8, 1.),
                world: scenes::random_scene(),
                lookfrom: Vec3f::new(13.0, 2.0, 3.0),
                lookat: Vec3f::repeat(0.0),
                vertical_fov: 20.0,
                aperture: 0.1,
            }
        }
        2 => {
            println!("Running scene two_spheres");
            Scene {
                background: Vec3f::new(0.7, 0.8, 1.),
                world: scenes::two_spheres(),
                lookfrom: Vec3f::new(13.0, 2.0, 3.0),
                lookat: Vec3f::repeat(0.0),
                vertical_fov: 20.0,
                ..Default::default()
            }
        }
        3 => {
            println!("Running scene two_perlin_spheres");
            Scene {
                background: Vec3f::new(0.7, 0.8, 1.),
                world: scenes::two_perlin_spheres(),
                lookfrom: Vec3f::new(13.0, 2.0, 3.0),
                lookat: Vec3f::repeat(0.0),
                vertical_fov: 20.0,
                ..Default::default()
            }
        }
        4 => {
            println!("Running scene earth");
            Scene {
                background: Vec3f::new(0.7, 0.8, 1.),
                world: scenes::earth(),
                lookfrom: Vec3f::new(13.0, 2.0, 3.0),
                lookat: Vec3f::repeat(0.0),
                vertical_fov: 20.0,
                ..Default::default()
            }
        }
        5 => {
            println!("Running scene simple_light");
            samples = 400;
            Scene {
                background: Vec3f::new(0.0, 0.0, 0.0),
                world: scenes::simple_light(),
                lookfrom: Vec3f::new(26.0, 3.0, 6.0),
                lookat: Vec3f::new(0., 2., 0.),
                vertical_fov: 20.,
                ..Default::default()
            }
        }
        6 => {
            println!("Running scene empty_cornell_box");
            samples = 200;
            aspect_ratio = 1.0;
            width = 600;
            Scene {
                background: Vec3f::new(0.0, 0.0, 0.0),
                world: scenes::empty_cornell_box(),
                lookfrom: Vec3f::new(278.0, 278.0, -800.0),
                lookat: Vec3f::new(278., 278., 0.),
                vertical_fov: 40.,
                ..Default::default()
            }
        }
        7 => {
            println!("Running scene cornell_box");
            samples = 200;
            aspect_ratio = 1.0;
            width = 600;
            Scene {
                background: Vec3f::new(0.0, 0.0, 0.0),
                world: scenes::cornell_box(),
                lookfrom: Vec3f::new(278.0, 278.0, -800.0),
                lookat: Vec3f::new(278., 278., 0.),
                vertical_fov: 40.,
                ..Default::default()
            }
        }
        8 => {
            println!("Running scene smoke_cornell_box");
            samples = 200;
            aspect_ratio = 1.0;
            width = 600;
            Scene {
                background: Vec3f::new(0.0, 0.0, 0.0),
                world: scenes::smoke_cornell_box(),
                lookfrom: Vec3f::new(278.0, 278.0, -800.0),
                lookat: Vec3f::new(278., 278., 0.),
                vertical_fov: 40.,
                ..Default::default()
            }
        }
        9 => {
            println!("Running scene final_scene");
            samples = 10000;
            aspect_ratio = 1.0;
            width = 800;
            Scene {
                background: Vec3f::new(0.0, 0.0, 0.0),
                world: scenes::final_scene(),
                lookfrom: Vec3f::new(478.0, 278.0, -600.0),
                lookat: Vec3f::new(278., 278., 0.),
                vertical_fov: 40.,
                ..Default::default()
            }
        }
        _ => {
            eprintln!("There is no scene {}", scene);
            return None;
        },
    };
    let height = (width as f64 / aspect_ratio) as u32;
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
    let image: Vec<Rgba> = (0..height)
        .into_par_iter()
        .rev()
        .progress_with(progress)
        .flat_map(|x| rayon::iter::repeat(x).zip(0..width))
        .map(|(j, i)| {
                    // Calculate the color `ns` times and average the result
                    let col = (0..samples).fold(Vec3f::<Color>::repeat(0.0), |acc, _| {
                        let mut rng = rand::thread_rng();
                        let u = (i as f64 + rng.gen::<f64>()) / width as f64;
                        let v = (j as f64 + rng.gen::<f64>()) / height as f64;
                        let ray = camera.ray(u, v);
                        acc + color(ray, background, &world, 50)
                    }) / samples as f64;
                    // Gamma correction
                    let col = col.map(|x| x.sqrt().clamp(0.0, 0.999) * 256.);
                    Rgba {
                        r: col.x() as u8,
                        g: col.y() as u8,
                        b: col.z() as u8,
                        a: 255,
                    }
                })
        .collect::<Vec<_>>();
    let buffer: &[u8] = bytemuck::cast_slice(&image);
    image::save_buffer("image.png", buffer, width, height, image::ColorType::Rgba8).unwrap();
    Some(())
}


fn main() -> Result<(), DummyError> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <scene>", args[0]);
        eprintln!("Possible scenes:");
        eprintln!("\t- 1: random_scene");
        eprintln!("\t- 2: two_spheres");
        eprintln!("\t- 3: two_perlin_spheres");
        eprintln!("\t- 4: earth");
        eprintln!("\t- 5: simple_light");
        eprintln!("\t- 6: empty_cornell_box");
        eprintln!("\t- 7: cornell_box");
        eprintln!("\t- 8: smoke_cornell_box");
        eprintln!("\t- 9: final_scene");
        return Err(ERROR);
    }
    let scene = args.get(1).unwrap_or(&String::from("1")).parse().map_err(|_| ERROR)?;
    println!("Scene number: {}", scene);
    let instant = std::time::Instant::now();
    render(400, 16.0 / 9.0, 100, scene).ok_or(ERROR)?;
    println!("{:?}", instant.elapsed());
    Ok(())
}

use std::fmt::{Display, Formatter, self};
#[derive(Debug)]
struct DummyError;
impl Display for DummyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("There was an error")
    }
}
impl Error for DummyError {}

const ERROR: DummyError = DummyError{};