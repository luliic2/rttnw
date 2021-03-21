use rand::Rng;

use crate::math::{CheckerTexture, Dielectric, Lambertian, List, Metal, MovingSphere, NoiseTexture, Position, Sphere, Vec3f, ImageTexture, DiffuseLight, XYRectangle, YZRectangle, XZRectangle, Color};
use std::sync::Arc;

/// Generate the cover of the book
pub fn random_scene() -> List {
    let mut rng = rand::thread_rng();
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

pub fn simple_light() -> List {
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
    let light = DiffuseLight::arc(Vec3f::repeat(4.));
    world.push(XYRectangle {
        material: light,
        x_min: 3.0,
        x_max: 5.0,
        y_min: 1.0,
        y_max: 3.0,
        k: -2.0
    });

    world
}

pub fn cornell_box() -> List {
    let mut world = List::new();

    let red = Lambertian::arc(Vec3f::new(0.65, 0.05, 0.05));
    let white = Lambertian::arc(Vec3f::repeat(0.73));
    let green = Lambertian::arc(Vec3f::new(0.12, 0.45, 0.15));
    let light = DiffuseLight::arc(Vec3f::<Color>::repeat(15.));
    world.push(YZRectangle {
        material: green,
        y_min: 0.0,
        y_max: 555.,
        z_min: 0.0,
        z_max: 555.,
        k: 555.
    });
    world.push(YZRectangle {
        material: red,
        y_min: 0.0,
        y_max: 555.,
        z_min: 0.0,
        z_max: 555.,
        k: 0.
    });
    world.push(XZRectangle {
        material: light,
        x_min: 213.0,
        x_max: 343.,
        z_min: 227.0,
        z_max: 332.,
        k: 554.
    });
    world.push(XZRectangle {
        material: white.clone(),
        x_min: 0.0,
        x_max: 555.,
        z_min: 0.0,
        z_max: 555.,
        k: 555.
    });
    world.push(XZRectangle {
        material: white.clone(),
        x_min: 0.0,
        x_max: 555.,
        z_min: 0.0,
        z_max: 555.,
        k: 0.
    });
    world.push(XYRectangle {
        material: white,
        x_min: 0.0,
        x_max: 555.,
        y_min: 0.0,
        y_max: 555.,
        k: 555.
    });

    world
}
