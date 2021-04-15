use crate::camera::Camera;
use crate::common::Rng;
use crate::geometry::Vec3;
use crate::hittable::{Hittable, HittableList, Sphere, BVH};
use crate::material::*;

pub trait SceneTrait {
    fn scene(&self, width: usize, rng: &mut Rng) -> (usize, Scene);
}

#[derive(Default, Debug)]
pub struct Scene {
    pub world: HittableList,
    pub camera: Camera,
}

pub struct RandomScene {}

impl SceneTrait for RandomScene {
    fn scene(&self, width: usize, rng: &mut Rng) -> (usize, Scene) {
        let mut world = HittableList::new();

        let ar = 1.5;
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
        let camera = Camera::new(look_from, look_at, 20.0, ar, 0.1, 10.0);

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

        let bvh = BVH::build(&mut world.objects, 0.0, 0.0, rng);
        world.push(Box::new(bvh));

        ((width as f64 / ar) as usize, Scene { world, camera })
    }
}

pub struct SimpleScene {}

impl SceneTrait for SimpleScene {
    fn scene(&self, width: usize, rng: &mut Rng) -> (usize, Scene) {
        let mut world = HittableList::new();

        let ar = 16.0 / 9.0;
        let look_from = Vec3 {
            x: -5.0,
            y: 1.0,
            z: 0.0,
        };
        let look_at = Vec3 {
            x: 0.0,
            y: 0.5,
            z: 0.0,
        };
        let camera = Camera::new(
            look_from,
            look_at,
            90.0,
            ar,
            0.01,
            (look_from - look_at).len(),
        );

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

        let sphere_mat = Box::new(Lambertian {
            albedo: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.8,
            },
        });
        world.push(Box::new(Sphere {
            c: Vec3 {
                x: 0.0,
                y: 0.5,
                z: 0.0,
            },
            r: 0.5,
            mat: Some(sphere_mat),
        }));

        let bvh = BVH::build(&mut world.objects, 0.0, 0.0, rng);
        world.push(Box::new(bvh));

        ((width as f64 / ar) as usize, Scene { world, camera })
    }
}

pub struct Sphereflake {}

impl SceneTrait for Sphereflake {
    fn scene(&self, width: usize, rng: &mut Rng) -> (usize, Scene) {
        let mut objects: Vec<Box<dyn Hittable>> = vec![];

        let ar = 16.0 / 9.0;
        let look_from = Vec3 {
            x: -3.0,
            y: 1.0,
            z: 0.0,
        };
        let look_at = Vec3 {
            x: 0.0,
            y: 0.5,
            z: 0.0,
        };
        let camera = Camera::new(
            look_from,
            look_at,
            90.0,
            ar,
            0.01,
            (look_from - look_at).len(),
        );

        let ground_mat = Box::new(Lambertian {
            albedo: Vec3 {
                x: 0.5,
                y: 0.5,
                z: 0.5,
            },
        });
        objects.push(Box::new(Sphere {
            c: Vec3 {
                x: 0.0,
                y: -1000.0,
                z: 0.0,
            },
            r: 1000.0,
            mat: Some(ground_mat),
        }));

        let pos = Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };
        let axis = Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };
        objects.extend(Self::make(pos, axis, 1.0, 0));

        let bvh = BVH::build(&mut objects, 0.0, 0.0, rng);
        let mut world = HittableList::new();
        world.push(Box::new(bvh));

        ((width as f64 / ar) as usize, Scene { world, camera })
    }
}

impl Sphereflake {
    fn make(pos: Vec3, axis: Vec3, r: f64, depth: usize) -> Vec<Box<dyn Hittable>> {
        const MAX_DEPTH: usize = 3;

        let sphere_mat = Box::new(Dielectric { ir: 1.5 });
        let sphere = Box::new(Sphere {
            c: pos,
            r,
            mat: Some(sphere_mat),
        }) as Box<dyn Hittable>;
        let mut s = vec![sphere];

        if depth == MAX_DEPTH {
            return s;
        }

        let perp: Vec3;
        if axis.x != 0.0 {
            perp = Vec3 {
                x: -axis.y,
                y: axis.x,
                z: 0.0,
            }
            .unit();
        } else if axis.y != 0.0 {
            perp = Vec3 {
                x: axis.y,
                y: -axis.x,
                z: 0.0,
            }
            .unit();
        } else {
            perp = Vec3 {
                x: axis.z,
                y: 0.0,
                z: -axis.x,
            }
            .unit();
        };

        // Vertical
        for i in 1..3 {
            //let mat = glam::Mat3::from_axis_angle(perp, 0.785398 * i as f32);
            //let a1 = mat * axis.unit();
            let a1 = axis.unit().rotate_axis_angle(perp, 0.785398 * i as f64);
            let n_spheres = match i % 2 {
                1 => 3,
                _ => 6,
            };
            let angle = 2.0 * 3.1415926 / (n_spheres) as f64;
            // Around
            for j in 0..n_spheres {
                let offset = match i % 2 {
                    1 => 0.0,
                    _ => 0.523599,
                };
                //let mat = glam::Mat3::from_axis_angle(axis, angle * j as f32 + offset);
                //let new_axis = (mat * a1).normalize();
                let new_axis = a1.rotate_axis_angle(axis, angle * j as f64 + offset).unit();
                let new_pos = pos + new_axis * (r) * 1.33;
                s.extend(Sphereflake::make(new_pos, new_axis, 0.33 * r, depth + 1));
            }
        }

        s
    }
}
