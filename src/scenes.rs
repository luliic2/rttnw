use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

use crate::math::{
    CheckerTexture, Dielectric, Lambertian, List, Metal, MovingSphere, NoiseTexture, Position,
    Sphere, Vec3f, ImageTexture
};
use std::sync::Arc;

/// Generate the cover of the book
pub fn random_scene() -> List {
    let mut rng = SmallRng::from_entropy();
    let mut list = List::new();
    let checker = CheckerTexture {
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

pub fn two_spheres() -> List {
    let mut world = List::new();
    let checker = Arc::new(CheckerTexture {
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

pub fn two_perlin_spheres() -> List {
    let mut world = List::new();
    let perlin = Arc::new(NoiseTexture::scaled(4.));
    world.push(Sphere {
        center: Vec3f::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Box::new(Lambertian::from(&perlin)),
    });
    world.push(Sphere {
        center: Vec3f::new(0.0, 2.0, 0.0),
        radius: 2.0,
        material: Box::new(Lambertian::from(&perlin)),
    });

    world
}

pub fn earth() -> List {
    let mut world = List::new();
    let earth = ImageTexture::new("assets/earth.png");
    world.push(Sphere {
        center: Vec3f::repeat(0.0),
        radius: 2.,
        material: Box::new(Lambertian::new(earth))
    });
    world
}
