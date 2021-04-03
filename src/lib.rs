mod camera;
mod common;
mod geometry;
mod hittable;
mod material;

use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use std::sync::Arc;

use indicatif::{ParallelProgressIterator, ProgressBar};
use rayon::prelude::*;

use camera::Camera;
use common::{clamp, Rng};
use geometry::{Ray, Vec3};
use hittable::{Hittable, HittableList, Primative, Sphere};
use material::{Material, MaterialTrait};

/// Holds info about an image. Handles rendering.
struct Renderer {
    pub width: usize,
    pub height: usize,
}

impl Renderer {
    pub fn render(&self, n_samples: usize) {
        let mut rng = Rng::new(1232);
        let max_depth = 50;

        // PNG
        let path = Path::new(r"out.png");
        let file = File::create(path).unwrap();
        let ref mut w = BufWriter::new(file);
        let mut encoder = png::Encoder::new(w, self.width as u32, self.height as u32);
        encoder.set_color(png::ColorType::RGB);
        encoder.set_depth(png::BitDepth::Eight);

        let prog_bar = ProgressBar::new(self.height as u64);
        prog_bar.set_style(
            indicatif::ProgressStyle::default_bar()
                .template(
                    "  {msg:.bright.cyan} [{bar:25}] {pos:>4}/{len:4} {elapsed_precise} Eta: {eta_precise}",
                )
                .progress_chars("=> "),
        );
        prog_bar.set_message("Rendering");

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
        let cam = Camera::new(
            look_from,
            look_at,
            20.0,
            self.width as f64 / self.height as f64,
            0.1,
            10.0,
        );

        // World
        let world = random_scene(&mut rng);

        let img_buf: Vec<u8> = (0..self.height)
            .into_par_iter()
            .progress_with(prog_bar)
            .map(|j| {
                let mut row_buf: Vec<u8> = Vec::with_capacity(self.width * 3);
                let mut rng = Rng::new(123 + j as u32);
                for i in 0..self.width {
                    let mut pixel_color = Vec3::zero();
                    for _n in 0..n_samples {
                        let u = (i as f64 + rng.gen()) / ((self.width - 1) as f64);
                        let v = (j as f64 + rng.gen()) / ((self.height - 1) as f64);

                        let r = cam.get_ray(u, v, &mut rng);
                        pixel_color += Self::ray_color(r, &world, &mut rng, max_depth);
                    }
                    Self::write_color(&mut row_buf, pixel_color, n_samples);
                }
                row_buf
            })
            .flatten()
            .collect();

        let mut writer = encoder.write_header().unwrap();
        writer.write_image_data(&img_buf).unwrap();
    }

    fn ray_color(r: Ray, world: &HittableList, rng: &mut Rng, depth: usize) -> Vec3 {
        if depth <= 0 {
            return Vec3::zero();
        }

        match world.hit(r, 0.001, f64::INFINITY) {
            Some((rec, mat)) => match mat.scatter(r, &rec, rng) {
                Some((r, c)) => c * Self::ray_color(r, world, rng, depth - 1),
                None => Vec3::zero(),
            },
            None => {
                let t = 0.5 * (r.d.unit().y + 1.0);
                (1.0 - t) * Vec3::splat(1.0)
                    + t * Vec3 {
                        x: 0.5,
                        y: 0.7,
                        z: 1.0,
                    }
            }
        }
    }

    fn write_color(buf: &mut Vec<u8>, v: Vec3, n_samples: usize) {
        let scale = 1.0 / (n_samples as f64);
        let r = (v.x * scale).sqrt();
        let g = (v.y * scale).sqrt();
        let b = (v.z * scale).sqrt();

        buf.push((256.0 * clamp(r, 0.0, 0.9999)) as u8);
        buf.push((256.0 * clamp(g, 0.0, 0.9999)) as u8);
        buf.push((256.0 * clamp(b, 0.0, 0.9999)) as u8);
    }
}

fn random_scene(rng: &mut Rng) -> HittableList {
    let mut world = HittableList::new();

    let ground_mat = Arc::new(Material::Lambertian {
        albedo: Vec3 {
            x: 0.5,
            y: 0.5,
            z: 0.5,
        },
    });

    world.push(Primative::Sphere {
        c: Vec3 {
            x: 0.0,
            y: -1000.0,
            z: 0.0,
        },
        r: 1000.0,
        mat: Arc::clone(&ground_mat),
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
                let mat;

                if choose_mat < 0.8 {
                    let albedo = Vec3::random(rng) * Vec3::random(rng);
                    mat = Arc::new(Material::Lambertian { albedo });
                } else if choose_mat < 0.95 {
                    let albedo = Vec3::random_range(rng, 0.5, 1.0);
                    let fuzz = rng.range(0.0, 0.5);
                    mat = Arc::new(Material::Metal { albedo, fuzz });
                } else {
                    mat = Arc::new(Material::Dielectric { ir: 1.5 });
                }

                world.push(Primative::Sphere {
                    c: center,
                    r: 0.2,
                    mat: Arc::clone(&mat),
                });
            }
        }
    }

    let mat_1 = Arc::new(Material::Dielectric { ir: 1.5 });
    world.push(Primative::Sphere {
        c: Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        r: 1.0,
        mat: Arc::clone(&mat_1),
    });

    let mat_2 = Arc::new(Material::Lambertian {
        albedo: Vec3 {
            x: 0.4,
            y: 0.2,
            z: 0.1,
        },
    });
    world.push(Primative::Sphere {
        c: Vec3 {
            x: -4.0,
            y: 1.0,
            z: 0.0,
        },
        r: 1.0,
        mat: Arc::clone(&mat_2),
    });

    let mat_3 = Arc::new(Material::Metal {
        albedo: Vec3 {
            x: 0.7,
            y: 0.6,
            z: 0.5,
        },
        fuzz: 0.0,
    });
    world.push(Primative::Sphere {
        c: Vec3 {
            x: 4.0,
            y: 1.0,
            z: 0.0,
        },
        r: 1.0,
        mat: Arc::clone(&mat_3),
    });

    world
}

#[cfg(test)]
mod tests {
    use crate::Renderer;

    #[test]
    fn main() {
        // 960 540 / 426 240
        let r = Renderer {
            width: 300,
            height: 200,
        };
        r.render(10);
    }
}
