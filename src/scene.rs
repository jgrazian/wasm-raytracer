use std::sync::Arc;

use crate::camera::Camera;
use crate::geometry::Vec3;
use crate::hittable::{Hittable, HittableList, Sphere};
use crate::material::*;
use crate::perlin::Perlin;
use crate::rng::Rng;
use crate::texture::*;

pub trait SceneTrait {
    fn scene(&self, width: usize, rng: &mut Rng) -> (usize, Scene);
}

#[derive(Default)]
pub struct Scene {
    pub world: HittableList,
    pub camera: Camera,
    pub background: Vec3,
}

pub struct RandomScene {}

impl SceneTrait for RandomScene {
    fn scene(&self, width: usize, rng: &mut Rng) -> (usize, Scene) {
        let mut world = HittableList::new();
        let background = Vec3 {
            x: 0.70,
            y: 0.80,
            z: 1.00,
        };

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

        let ground_texture = CheckerTexture {
            odd: Box::new(SolidColor {
                color_value: Vec3 {
                    x: 0.2,
                    y: 0.3,
                    z: 0.1,
                },
            }),
            even: Box::new(SolidColor {
                color_value: Vec3 {
                    x: 0.9,
                    y: 0.9,
                    z: 0.9,
                },
            }),
        };
        let ground_mat = Arc::new(Lambertian {
            albedo: Arc::new(ground_texture) as Arc<dyn Texture>,
        });

        world.push(Sphere {
            c: Vec3 {
                x: 0.0,
                y: -1000.0,
                z: 0.0,
            },
            r: 1000.0,
            mat: Some(ground_mat),
        });

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
                    let mat: Arc<dyn Material> = if choose_mat < 0.8 {
                        let albedo = Vec3::random(rng) * Vec3::random(rng);
                        Arc::new(Lambertian::from(albedo))
                    } else if choose_mat < 0.95 {
                        let albedo = Vec3::random_range(rng, 0.5, 1.0);
                        let fuzz = rng.range(0.0, 0.5);
                        Arc::new(Metal { albedo, fuzz })
                    } else {
                        Arc::new(Dielectric { ir: 1.5 })
                    };

                    world.push(Sphere {
                        c: center,
                        r: 0.2,
                        mat: Some(mat),
                    });
                }
            }
        }

        let mat_1 = Arc::new(Dielectric { ir: 1.5 });
        world.push(Sphere {
            c: Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            r: 1.0,
            mat: Some(mat_1),
        });

        let mat_2 = Arc::new(Lambertian::from(Vec3 {
            x: 0.4,
            y: 0.2,
            z: 0.1,
        }));
        world.push(Sphere {
            c: Vec3 {
                x: -4.0,
                y: 1.0,
                z: 0.0,
            },
            r: 1.0,
            mat: Some(mat_2),
        });

        let mat_3 = Arc::new(Metal {
            albedo: Vec3 {
                x: 0.7,
                y: 0.6,
                z: 0.5,
            },
            fuzz: 0.0,
        });
        world.push(Sphere {
            c: Vec3 {
                x: 4.0,
                y: 1.0,
                z: 0.0,
            },
            r: 1.0,
            mat: Some(mat_3),
        });

        world.into_bvh(rng);

        (
            (width as f64 / ar) as usize,
            Scene {
                world,
                camera,
                background,
            },
        )
    }
}

pub struct SimpleScene {}

impl SceneTrait for SimpleScene {
    fn scene(&self, width: usize, rng: &mut Rng) -> (usize, Scene) {
        let mut world = HittableList::new();
        let background = Vec3 {
            x: 0.70,
            y: 0.80,
            z: 1.00,
        };

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

        let ground_mat = Arc::new(Lambertian::from(Vec3 {
            x: 0.5,
            y: 0.5,
            z: 0.5,
        }));
        world.push(Sphere {
            c: Vec3 {
                x: 0.0,
                y: -1000.0,
                z: 0.0,
            },
            r: 1000.0,
            mat: Some(ground_mat),
        });

        let sphere_mat = Arc::new(Lambertian::from(Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.8,
        }));
        world.push(Sphere {
            c: Vec3 {
                x: 0.0,
                y: 0.5,
                z: 0.0,
            },
            r: 0.5,
            mat: Some(sphere_mat),
        });

        world.into_bvh(rng);

        (
            (width as f64 / ar) as usize,
            Scene {
                world,
                camera,
                background,
            },
        )
    }
}

pub struct Sphereflake {}

impl SceneTrait for Sphereflake {
    fn scene(&self, width: usize, rng: &mut Rng) -> (usize, Scene) {
        let mut world = HittableList::new();
        let background = Vec3 {
            x: 0.70,
            y: 0.80,
            z: 1.00,
        };

        let ar = 16.0 / 9.0;
        let look_from = Vec3 {
            x: -2.5,
            y: 2.5,
            z: 0.0,
        };
        let look_at = Vec3 {
            x: 0.0,
            y: 1.0,
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

        let ground_mat = Arc::new(Lambertian::from(Vec3 {
            x: 0.5,
            y: 0.5,
            z: 0.5,
        }));
        world.push(Sphere {
            c: Vec3 {
                x: 0.0,
                y: -1000.0,
                z: 0.0,
            },
            r: 1000.0,
            mat: Some(ground_mat),
        });

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
        world.objects.extend(Self::make(pos, axis, 1.0, 0));

        world.into_bvh(rng);

        (
            (width as f64 / ar) as usize,
            Scene {
                world,
                camera,
                background,
            },
        )
    }
}

impl Sphereflake {
    fn make(pos: Vec3, axis: Vec3, r: f64, depth: usize) -> Vec<Box<dyn Hittable>> {
        const MAX_DEPTH: usize = 4;

        let sphere_mat = Arc::new(Metal {
            albedo: Vec3 {
                x: 0.7,
                y: 0.7,
                z: 0.7,
            },
            fuzz: 0.01,
        });
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
                let new_axis = a1.rotate_axis_angle(axis, angle * j as f64 + offset).unit();
                let new_pos = pos + new_axis * r * 1.33;
                s.extend(Sphereflake::make(new_pos, new_axis, 0.33 * r, depth + 1));
            }
        }

        s
    }
}

pub struct PerlinSpheres {}

impl SceneTrait for PerlinSpheres {
    fn scene(&self, width: usize, rng: &mut Rng) -> (usize, Scene) {
        let mut world = HittableList::new();
        let background = Vec3 {
            x: 0.70,
            y: 0.80,
            z: 1.00,
        };

        let ar = 16.0 / 9.0;
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
        let camera = Camera::new(
            look_from,
            look_at,
            20.0,
            ar,
            0.01,
            (look_from - look_at).len(),
        );

        let mat = Arc::new(Lambertian {
            albedo: Arc::new(NoiseTexture {
                noise: Perlin::new(),
                scale: 4.0,
            }),
        });
        world.push(Sphere {
            c: Vec3 {
                x: 0.0,
                y: -1000.0,
                z: 0.0,
            },
            r: 1000.0,
            mat: Some(mat.clone()),
        });

        world.push(Sphere {
            c: Vec3 {
                x: 0.0,
                y: 2.0,
                z: 0.0,
            },
            r: 2.0,
            mat: Some(mat.clone()),
        });

        world.into_bvh(rng);

        (
            (width as f64 / ar) as usize,
            Scene {
                world,
                camera,
                background,
            },
        )
    }
}

pub struct CheckeredSpheres {}

impl SceneTrait for CheckeredSpheres {
    fn scene(&self, width: usize, rng: &mut Rng) -> (usize, Scene) {
        let mut world = HittableList::new();
        let background = Vec3 {
            x: 0.70,
            y: 0.80,
            z: 1.00,
        };

        let ar = 16.0 / 9.0;
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

        let checker_text = CheckerTexture {
            odd: Box::new(SolidColor {
                color_value: Vec3 {
                    x: 0.2,
                    y: 0.3,
                    z: 0.1,
                },
            }),
            even: Box::new(SolidColor {
                color_value: Vec3 {
                    x: 0.9,
                    y: 0.9,
                    z: 0.9,
                },
            }),
        };

        let mat = Arc::new(Lambertian {
            albedo: Arc::new(checker_text),
        });
        world.push(Sphere {
            c: Vec3 {
                x: 0.0,
                y: -10.0,
                z: 0.0,
            },
            r: 10.0,
            mat: Some(mat.clone()),
        });

        world.push(Sphere {
            c: Vec3 {
                x: 0.0,
                y: 10.0,
                z: 0.0,
            },
            r: 10.0,
            mat: Some(mat.clone()),
        });

        world.into_bvh(rng);

        (
            (width as f64 / ar) as usize,
            Scene {
                world,
                camera,
                background,
            },
        )
    }
}

pub struct LightSpheres {}

impl SceneTrait for LightSpheres {
    fn scene(&self, width: usize, rng: &mut Rng) -> (usize, Scene) {
        let mut world = HittableList::new();
        let background = Vec3 {
            x: 0.00,
            y: 0.00,
            z: 0.00,
        };

        let ar = 16.0 / 9.0;
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
        let camera = Camera::new(
            look_from,
            look_at,
            20.0,
            ar,
            0.01,
            (look_from - look_at).len(),
        );

        let mat_ground = Arc::new(Lambertian {
            albedo: Arc::new(NoiseTexture {
                noise: Perlin::new(),
                scale: 4.0,
            }),
        });
        let mat_light = Arc::new(DiffuseLight {
            emit: Arc::new(SolidColor {
                color_value: Vec3::splat(2.0),
            }),
        });

        world.push(Sphere {
            c: Vec3 {
                x: 0.0,
                y: -1000.0,
                z: 0.0,
            },
            r: 1000.0,
            mat: Some(mat_ground),
        });
        world.push(Sphere {
            c: Vec3 {
                x: 0.0,
                y: 2.0,
                z: 0.0,
            },
            r: 2.0,
            mat: Some(mat_light),
        });

        world.into_bvh(rng);

        (
            (width as f64 / ar) as usize,
            Scene {
                world,
                camera,
                background,
            },
        )
    }
}
