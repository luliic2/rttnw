use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};

mod math;
use crate::math::Position;
use math::{
    Camera, CameraDescriptor, Checker, Color, Dielectric, Hittable, Lambertian, List, Metal,
    MovingSphere, Ray, Sphere, Vec3f,
};
use std::error::Error;
use std::sync::Arc;

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

/// Generate the cover of the book
fn random_scene() -> List {
    let mut rng = SmallRng::from_entropy();
    let mut list = List::new();
    let checker = Checker {
        odd: Arc::new(Vec3f::new(0.2, 0.3, 0.1)),
        even: Arc::new(Vec3f::new(0.9, 0.9, 0.9)),
    };
    list.push(Sphere {
        center: (0.0, -1000.0, 0.0).into(),
        radius: 1000.0,
        material: Lambertian::boxed(checker),
    });
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rng.gen();
            let center = Vec3f::<Position>::new(
                a as f64 + 0.9 + rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 + rng.gen::<f64>(),
            );
            if (center - Vec3f::new(4.0, 0.2, 0.0)).magnitude() > 0.9 {
                if choose_mat < 0.8 {
                    let final_center = center + Vec3f::new(0.0, rng.gen_range(0.0..0.5), 0.0);
                    // diffuse
                    list.push(MovingSphere {
                        initial_center: center,
                        final_center,
                        initial_time: 0.0,
                        final_time: 1.0,
                        radius: 0.2,
                        material: Lambertian::boxed(Vec3f::new(
                            rng.gen::<f64>() * rng.gen::<f64>(),
                            rng.gen::<f64>() * rng.gen::<f64>(),
                            rng.gen::<f64>() * rng.gen::<f64>(),
                        )),
                    });
                } else if choose_mat < 0.95 {
                    // metal
                    list.push(Sphere {
                        center,
                        radius: 0.2,
                        material: Metal::boxed(
                            (
                                0.5 * (1.0 - rng.gen::<f64>()),
                                0.5 * (1.0 - rng.gen::<f64>()),
                                0.5 * (1.0 - rng.gen::<f64>()),
                            )
                                .into(),
                            0.5 * rng.gen::<f64>(),
                        ),
                    });
                } else {
                    // glass
                    list.push(Sphere {
                        center,
                        radius: 0.2,
                        material: Dielectric::boxed(1.5),
                    })
                }
            }
        }
    }
    list.push(Sphere {
        center: (0.0, 1.0, 0.0).into(),
        radius: 1.0,
        material: Dielectric::boxed(1.5),
    });
    list.push(Sphere {
        center: (-4.0, 1.0, 0.0).into(),
        radius: 1.0,
        material: Lambertian::boxed(Vec3f::new(0.4, 0.2, 0.1)),
    });
    list.push(Sphere {
        center: (4.0, 1.0, 0.0).into(),
        radius: 1.0,
        material: Metal::boxed((0.7, 0.6, 0.5).into(), 0.0),
    });

    list
}

fn two_spheres() -> List {
    let mut world = List::new();
    let checker = Arc::new(Checker {
        odd: Arc::new(Vec3f::new(0.2, 0.3, 0.1)),
        even: Arc::new(Vec3f::new(0.9, 0.9, 0.9)),
    });
    world.push(Sphere {
        center: Vec3f::new(0.0, -10.0, 0.0),
        radius: 10.0,
        material: Box::new(Lambertian::from(&checker)),
    });
    world.push(Sphere {
        center: Vec3f::new(0.0, 10.0, 0.0),
        radius: 10.0,
        material: Box::new(Lambertian::from(&checker)),
    });

    world
}

/// Saves the scene to a .png image of size `nx*ny`
fn print_result(width: usize, aspect_ratio: f64, samples: usize, scene: usize) {
    let height = (width as f64 / aspect_ratio) as usize;
    let (world, lookfrom, lookat, vertical_fov, aperture) = match scene {
        1 => {
            println!("Running scene random_scene");
            (
                random_scene(),
                Vec3f::new(13.0, 2.0, 3.0),
                Vec3f::repeat(0.0),
                20.0,
                0.1,
            )
        },
        2 => {
            println!("Running scene two_spheres");
            (
            two_spheres(),
            Vec3f::new(13.0, 2.0, 3.0),
            Vec3f::repeat(0.0),
            20.0,
            Default::default(),
            )
        },
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

    let progress = ProgressBar::new(height as u64).with_style(ProgressStyle::default_spinner()
                                                                  // .tick_chars("/|\\- ")
                                                                  .template("{pos}/{len} {spinner:.dim.bold}"));
    // let mut image = Vec::new();
    // let world = random_scene();
    // For each pixel
    let image: Vec<lodepng::RGBA> = (0..height)
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
                    // let mut col = Vec3f::<Color>::repeat(0.0);
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
                                // unimplemented!()
                            },
                        )
                        .sum::<Vec3f<Color>>()
                        / samples as f64;
                    // col = col / ns as f64;
                    // Gamma correction
                    let col = col.map(|x| x.sqrt() * 255.99);
                    lodepng::RGBA {
                        r: col.x() as u8,
                        g: col.y() as u8,
                        b: col.z() as u8,
                        a: 255,
                    }
                    // lodepng::RGBA {
                    //     r: 0,
                    //     g: 0,
                    //     b: 0,
                    //     a: 255
                    // }
                })
                .collect::<Vec<_>>()
        })
        .collect();
    lodepng::encode32_file("image.png", &image, width, height).unwrap();
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
