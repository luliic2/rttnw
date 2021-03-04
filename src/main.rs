use rand::Rng;

mod math;
use crate::math::Position;
use math::{Camera, Color, Dielectric, Hittable, Lambertian, List, Metal, Ray, Sphere, Vec3f};

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

/// Generate the cover of the book
fn random_scene() -> List<Sphere> {
    let mut rng = rand::thread_rng();
    let mut list = List {
        list: vec![Sphere {
            center: (0.0, -1000.0, 0.0).into(),
            radius: 1000.0,
            material: Lambertian::boxed((0.5, 0.5, 0.5).into()),
        }],
    };
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f32 = rng.gen();
            let center = Vec3f::<Position>::new(
                a as f32 + 0.9 + rng.gen::<f32>(),
                0.2,
                b as f32 + 0.9 + rng.gen::<f32>(),
            );
            if (center - Vec3f::new(4.0, 0.2, 0.0)).magnitude() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    list.push(Sphere {
                        center,
                        radius: 0.2,
                        material: Lambertian::boxed(
                            (
                                rng.gen::<f32>() * rng.gen::<f32>(),
                                rng.gen::<f32>() * rng.gen::<f32>(),
                                rng.gen::<f32>() * rng.gen::<f32>(),
                            )
                                .into(),
                        ),
                    });
                } else if choose_mat < 0.95 {
                    // metal
                    list.push(Sphere {
                        center,
                        radius: 0.2,
                        material: Metal::boxed(
                            (
                                0.5 * (1.0 - rng.gen::<f32>()),
                                0.5 * (1.0 - rng.gen::<f32>()),
                                0.5 * (1.0 - rng.gen::<f32>()),
                            )
                                .into(),
                            0.5 * rng.gen::<f32>(),
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
        material: Dielectric::boxed(1.5)
    });
    list.push(Sphere {
        center: (-4.0, 1.0, 0.0).into(),
        radius: 1.0,
        material: Lambertian::boxed((0.4, 0.2, 0.1).into())
    });
    list.push(Sphere {
        center: (4.0, 1.0, 0.0).into(),
        radius: 1.0,
        material: Metal::boxed((0.7, 0.6, 0.5).into(), 0.0)
    });

    list
}

/// Saves the scene to a .ppm image of size `nx*ny`
fn print_result(nx: usize, ny: usize, ns: usize) {
    let lookfrom = (13.0, 2.0, 3.0).into();
    let lookat = (0.0, 0.0, 0.0).into();
    let camera = Camera::new(
        lookfrom,
        lookat,
        (0.0, 1.0, 0.0).into(),
        20.0,
        nx as f32 / ny as f32,
        0.1,
        10.0,
    );
    let mut image = Vec::new();
    let world = random_scene();
    let mut rng = rand::thread_rng();
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
            let col = col.map(|x| x.sqrt() * 255.99);
            let rgba = lodepng::RGBA {
                r: col.x() as u8,
                g: col.y() as u8,
                b: col.z() as u8,
                a: 255
            };
            image.push(rgba);
        }
    }
    lodepng::encode32_file("image.png", &image, nx, ny).unwrap();
}

fn main() {
    print_result(3072, 1920, 50);
}
