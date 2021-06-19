use rand::Rng;

use crate::math::{
    BvhTree, CheckerTexture, Color, ConstantMedium, Cube, Dielectric, DiffuseLight, Hittable,
    ImageTexture, Lambertian, List, Material, Metal, MovingSphere, NoiseTexture, Plane, Position,
    Sphere, Vec3f, Xy, Xz, Yz,
};
use std::sync::Arc;

#[derive(Default)]
pub struct Scene {
    pub background: Vec3f<Color>,
    pub world: List,
    pub lookfrom: Vec3f<Position>,
    pub lookat: Vec3f<Position>,
    pub vertical_fov: f64,
    pub aperture: f64,
    pub width: u32,
    pub height: u32,
    pub aspect_ratio: f64,
    pub samples: u32,
}

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
        material: Lambertian::arc(checker),
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
                        material: Metal::arc(
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
                        material: Dielectric::arc(1.5),
                    })
                }
            }
        }
    }
    list.push(Sphere {
        center: (0.0, 1.0, 0.0).into(),
        radius: 1.0,
        material: Dielectric::arc(1.5),
    });
    list.push(Sphere {
        center: (-4.0, 1.0, 0.0).into(),
        radius: 1.0,
        material: Lambertian::arc(Vec3f::new(0.4, 0.2, 0.1)),
    });
    list.push(Sphere {
        center: (4.0, 1.0, 0.0).into(),
        radius: 1.0,
        material: Metal::arc((0.7, 0.6, 0.5).into(), 0.0),
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
        material: Lambertian::<CheckerTexture>::arc(checker.clone()),
    });
    world.push(Sphere {
        center: Vec3f::new(0.0, 10.0, 0.0),
        radius: 10.0,
        material: Lambertian::<CheckerTexture>::arc(checker),
    });

    world
}

pub fn two_perlin_spheres() -> List {
    let mut world = List::new();
    let perlin = Arc::new(NoiseTexture::scaled(4.));
    world.push(Sphere {
        center: Vec3f::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Lambertian::<NoiseTexture>::arc(perlin.clone()),
    });
    world.push(Sphere {
        center: Vec3f::new(0.0, 2.0, 0.0),
        radius: 2.0,
        material: Lambertian::<NoiseTexture>::arc(perlin),
    });

    world
}

pub fn earth() -> List {
    let mut world = List::new();
    let earth = ImageTexture::new("assets/earth.png");
    world.push(Sphere {
        center: Vec3f::repeat(0.0),
        radius: 2.,
        material: Lambertian::arc(earth),
    });
    world
}

pub fn simple_light() -> List {
    let mut world = List::new();
    let perlin = Arc::new(NoiseTexture::scaled(4.));
    world.push(Sphere {
        center: Vec3f::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Lambertian::<NoiseTexture>::arc(perlin.clone()),
    });
    world.push(Sphere {
        center: Vec3f::new(0.0, 2.0, 0.0),
        radius: 2.0,
        material: Lambertian::<NoiseTexture>::arc(perlin),
    });
    let light = DiffuseLight::arc(Vec3f::repeat(4.));
    world.push(Xy::rectangle(light, 3. ..5., 1. ..3., -2.0));

    world
}

pub fn empty_cornell_box() -> List {
    let mut world = List::new();

    let red = Lambertian::arc(Vec3f::new(0.65, 0.05, 0.05));
    let white = Lambertian::arc(Vec3f::repeat(0.73));
    let green = Lambertian::arc(Vec3f::new(0.12, 0.45, 0.15));
    let light = DiffuseLight::arc(Vec3f::<Color>::repeat(15.));

    world.push(Yz::rectangle(green, 0. ..555., 0. ..555., 555.));
    world.push(Yz::rectangle(red, 0. ..555., 0. ..555., 0.));
    world.push(Xz::rectangle(light, 213. ..343., 227. ..332., 554.));
    world.push(Xz::rectangle(white.clone(), 0. ..555., 0.0..555., 555.));
    world.push(Xz::rectangle(white.clone(), 0. ..555., 0. ..555., 0.));
    world.push(Xy::rectangle(white, 0. ..555., 0. ..555., 555.));

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

    world.push(Yz::rectangle(green, 0. ..555., 0. ..555., 555.));
    world.push(Yz::rectangle(red, 0. ..555., 0. ..555., 0.));
    world.push(Xz::rectangle(light, 113. ..443., 127. ..432., 554.));
    world.push(Xz::rectangle(white.clone(), 0. ..555., 0.0..555., 555.));
    world.push(Xz::rectangle(white.clone(), 0. ..555., 0. ..555., 0.));
    world.push(Xy::rectangle(white.clone(), 0. ..555., 0. ..555., 555.));

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

pub fn final_scene() -> List {
    let mut boxes = List::new();
    let ground = Lambertian::arc(Vec3f::new(0.48, 0.83, 0.53));

    let mut rng = rand::thread_rng();
    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.;
            let v0 = Vec3f::new(-1000. + i as f64 * w, 0., -1000. + j as f64 * w);
            let v1 = Vec3f::new(v0.x() + w, rng.gen_range(1. ..101.), v0.z() + w);

            let cube = Cube::new(v0, v1, ground.clone());
            boxes.push(cube);
        }
    }

    let mut world = List::new();

    world.push(BvhTree::from(boxes));

    let light = DiffuseLight::arc(Vec3f::repeat(7.));
    world.push(Xz::rectangle(light, 123. ..423., 147. ..412., 554.));

    let center1 = Vec3f::repeat(400.);
    let center2 = center1 + Vec3f::new(30., 0., 0.);
    world.push(MovingSphere {
        center: center1..center2,
        time: 0. ..1.,
        radius: 50.,
        material: Lambertian::boxed(Vec3f::new(0.7, 0.3, 0.1)),
    });

    world.push(Sphere {
        center: Vec3f::new(260., 150., 45.),
        radius: 50.0,
        material: Dielectric::arc(1.5),
    });
    world.push(Sphere {
        center: Vec3f::new(0., 150., 45.),
        radius: 50.0,
        material: Metal::arc(Vec3f::new(0.8, 0.8, 0.9), 1.),
    });

    let boundary = Sphere {
        center: Vec3f::new(360., 150., 145.),
        radius: 70.,
        material: Dielectric::arc(1.5),
    };
    world.push(boundary.clone());
    world.push(ConstantMedium::new(
        Arc::new(boundary),
        0.2,
        Arc::new(Vec3f::new(0.2, 0.4, 0.9)),
    ));
    world.push(ConstantMedium::new(
        Arc::new(Sphere {
            center: Vec3f::repeat(0.),
            radius: 5000.,
            material: Dielectric::arc(1.5),
        }),
        0.0001,
        Arc::new(Vec3f::repeat(1.)),
    ));

    let earth = ImageTexture::new("assets/earth.png");
    world.push(Sphere {
        center: Vec3f::new(400., 200., 400.),
        radius: 100.,
        material: Lambertian::arc(earth),
    });
    let noise = NoiseTexture::scaled(0.1);
    world.push(Sphere {
        center: Vec3f::new(220., 280., 300.),
        radius: 80.0,
        material: Lambertian::arc(noise),
    });

    let mut boxes = List::new();
    let white = Lambertian::arc(Vec3f::repeat(0.73));
    let ns = 1000;
    for _ in 0..ns {
        boxes.push(Sphere {
            center: Vec3f::random(0. ..165.),
            radius: 10.,
            material: white.clone(),
        });
    }

    world.push(
        BvhTree::from(boxes)
            .rotate_y(15.)
            .translate(Vec3f::new(-100., 270., 395.)),
    );

    world
}

// 2280x1080
pub fn galaxy_s10e() -> Scene {
    let mut world = List::new();
    let width = 1080;
    let height = 2280;
    let aspect_ratio = (width / height) as f64;

    let light = Sphere {
        center: Vec3f::default(),
        radius: (width / 2) as f64 * 0.5,
        material: Lambertian::from(Vec3f::repeat(7.5)).arc(),
    };

    world.push(light);

    Scene {
        background: Vec3f::new(0.5, 0.5, 0.5),
        world,
        lookfrom: Default::default(),
        lookat: Vec3f::new(0., 0., 0.),
        vertical_fov: 40.,
        aperture: 0.1,
        width,
        height,
        aspect_ratio,
        samples: 200,
    }
}
