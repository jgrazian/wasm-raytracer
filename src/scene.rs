use crate::camera::Camera;
use crate::common::Rng;
use crate::geometry::Vec3;
use crate::hittable::{HittableList, Sphere};
use crate::material::*;

pub trait SceneTrait {
    fn scene(&self, rng: &mut Rng) -> Scene;
}

#[derive(Default, Debug)]
pub struct Scene {
    pub world: HittableList,
    pub camera: Camera,
}

pub struct RandomScene {}

impl SceneTrait for RandomScene {
    fn scene(&self, rng: &mut Rng) -> Scene {
        let mut world = HittableList::new();

        let look_from = Vec3 {
            x: 13.0,
            y: 2.0,
            z: 3.0,
        };
        let look_at = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let camera = Camera::new(look_from, look_at, 20.0, 1.5, 0.1, 10.0);

        let ground_mat = Box::new(Lambertian {
            albedo: Vec3 {
                x: 0.5,
                y: 0.5,
                z: 0.5,
            },
        });

        world.push(Box::new(Sphere {
            c: Vec3 {
                x: 0.0,
                y: -1000.0,
                z: 0.0,
            },
            r: 1000.0,
            mat: Some(ground_mat),
        }));

        for a in -11..11 {
            for b in -11..11 {
                let choose_mat = rng.gen();
                let center = Vec3 {
                    x: a as f64 + 0.9 * rng.gen(),
                    y: 0.2,
                    z: b as f64 + 0.9 * rng.gen(),
                };

                if (center
                    - Vec3 {
                        x: 4.0,
                        y: 0.2,
                        z: 0.0,
                    })
                .len()
                    > 0.9
                {
                    let mat: Box<dyn Material>;

                    if choose_mat < 0.8 {
                        let albedo = Vec3::random(rng) * Vec3::random(rng);
                        mat = Box::new(Lambertian { albedo });
                    } else if choose_mat < 0.95 {
                        let albedo = Vec3::random_range(rng, 0.5, 1.0);
                        let fuzz = rng.range(0.0, 0.5);
                        mat = Box::new(Metal { albedo, fuzz });
                    } else {
                        mat = Box::new(Dielectric { ir: 1.5 });
                    }

                    world.push(Box::new(Sphere {
                        c: center,
                        r: 0.2,
                        mat: Some(mat),
                    }));
                }
            }
        }

        let mat_1 = Box::new(Dielectric { ir: 1.5 });
        world.push(Box::new(Sphere {
            c: Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            r: 1.0,
            mat: Some(mat_1),
        }));

        let mat_2 = Box::new(Lambertian {
            albedo: Vec3 {
                x: 0.4,
                y: 0.2,
                z: 0.1,
            },
        });
        world.push(Box::new(Sphere {
            c: Vec3 {
                x: -4.0,
                y: 1.0,
                z: 0.0,
            },
            r: 1.0,
            mat: Some(mat_2),
        }));

        let mat_3 = Box::new(Metal {
            albedo: Vec3 {
                x: 0.7,
                y: 0.6,
                z: 0.5,
            },
            fuzz: 0.0,
        });
        world.push(Box::new(Sphere {
            c: Vec3 {
                x: 4.0,
                y: 1.0,
                z: 0.0,
            },
            r: 1.0,
            mat: Some(mat_3),
        }));

        Scene { world, camera }
    }
}
