use rand::Rng;

use crate::math::{
    CheckerTexture, Color, ConstantMedium, Cube, Dielectric, DiffuseLight, Hittable, ImageTexture,
    Lambertian, List, Metal, MovingSphere, NoiseTexture, Plane, Position, Sphere, Vec3f, XY, XZ,
    YZ,
};
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
                        center: center..final_center,
                        time: 0. ..1.,
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
        material: Lambertian::<CheckerTexture>::boxed(checker.clone()),
    });
    world.push(Sphere {
        center: Vec3f::new(0.0, 10.0, 0.0),
        radius: 10.0,
        material: Lambertian::<CheckerTexture>::boxed(checker),
    });

    world
}

pub fn two_perlin_spheres() -> List {
    let mut world = List::new();
    let perlin = Arc::new(NoiseTexture::scaled(4.));
    world.push(Sphere {
        center: Vec3f::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Lambertian::<NoiseTexture>::boxed(perlin.clone()),
    });
    world.push(Sphere {
        center: Vec3f::new(0.0, 2.0, 0.0),
        radius: 2.0,
        material: Lambertian::<NoiseTexture>::boxed(perlin),
    });

    world
}

pub fn earth() -> List {
    let mut world = List::new();
    let earth = ImageTexture::new("assets/earth.png");
    world.push(Sphere {
        center: Vec3f::repeat(0.0),
        radius: 2.,
        material: Box::new(Lambertian::from(earth)),
    });
    world
}

pub fn simple_light() -> List {
    let mut world = List::new();
    let perlin = Arc::new(NoiseTexture::scaled(4.));
    world.push(Sphere {
        center: Vec3f::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Lambertian::<NoiseTexture>::boxed(perlin.clone()),
    });
    world.push(Sphere {
        center: Vec3f::new(0.0, 2.0, 0.0),
        radius: 2.0,
        material: Lambertian::<NoiseTexture>::boxed(perlin),
    });
    let light = DiffuseLight::arc(Vec3f::repeat(4.));
    world.push(XY::rectangle(light, 3. ..5., 1. ..3., -2.0));

    world
}

pub fn empty_cornell_box() -> List {
    let mut world = List::new();

    let red = Lambertian::arc(Vec3f::new(0.65, 0.05, 0.05));
    let white = Lambertian::arc(Vec3f::repeat(0.73));
    let green = Lambertian::arc(Vec3f::new(0.12, 0.45, 0.15));
    let light = DiffuseLight::arc(Vec3f::<Color>::repeat(15.));

    world.push(YZ::rectangle(green, 0. ..555., 0. ..555., 555.));
    world.push(YZ::rectangle(red, 0. ..555., 0. ..555., 0.));
    world.push(XZ::rectangle(light, 213. ..343., 227. ..332., 554.));
    world.push(XZ::rectangle(white.clone(), 0. ..555., 0.0..555., 555.));
    world.push(XZ::rectangle(white.clone(), 0. ..555., 0. ..555., 0.));
    world.push(XY::rectangle(white, 0. ..555., 0. ..555., 555.));

    world
}

pub fn cornell_box() -> List {
    let mut world = empty_cornell_box();

    let white = Lambertian::arc(Vec3f::repeat(0.73));

    world.push(
        Cube::new(
            Vec3f::new(0., 0., 0.),
            Vec3f::new(165., 330., 165.),
            white.clone(),
        )
        .rotate_y(15.)
        .translate(Vec3f::new(265., 0., 295.)),
    );
    world.push(
        Cube::new(Vec3f::new(0., 0., 0.), Vec3f::repeat(165.), white)
            .rotate_y(-18.)
            .translate(Vec3f::new(130., 0., 65.)),
    );

    world
}

pub fn smoke_cornell_box() -> List {
    let mut world = List::new();

    let red = Lambertian::arc(Vec3f::new(0.65, 0.05, 0.05));
    let white = Lambertian::arc(Vec3f::repeat(0.73));
    let green = Lambertian::arc(Vec3f::new(0.12, 0.45, 0.15));
    let light = DiffuseLight::arc(Vec3f::<Color>::repeat(7.));

    world.push(YZ::rectangle(green, 0. ..555., 0. ..555., 555.));
    world.push(YZ::rectangle(red, 0. ..555., 0. ..555., 0.));
    world.push(XZ::rectangle(light, 113. ..443., 127. ..432., 554.));
    world.push(XZ::rectangle(white.clone(), 0. ..555., 0.0..555., 555.));
    world.push(XZ::rectangle(white.clone(), 0. ..555., 0. ..555., 0.));
    world.push(XY::rectangle(white.clone(), 0. ..555., 0. ..555., 555.));

    let c1 = Cube::new(
        Vec3f::new(0., 0., 0.),
        Vec3f::new(165., 330., 165.),
        white.clone(),
    )
    .rotate_y(15.)
    .translate(Vec3f::new(265., 0., 295.));
    let c2 = Cube::new(Vec3f::new(0., 0., 0.), Vec3f::repeat(165.), white)
        .rotate_y(-18.)
        .translate(Vec3f::new(130., 0., 65.));

    world.push(ConstantMedium::new(
        Arc::new(c1),
        0.01,
        Arc::new(Vec3f::repeat(0.)),
    ));
    world.push(ConstantMedium::new(
        Arc::new(c2),
        0.01,
        Arc::new(Vec3f::repeat(1.)),
    ));

    world
}
