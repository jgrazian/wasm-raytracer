mod camera;
mod common;
mod geometry;
mod hittable;
mod material;
mod scene;

use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use indicatif::{ParallelProgressIterator, ProgressBar};
use rayon::prelude::*;

use common::{clamp, Rng};
use geometry::{Ray, Vec3};
use hittable::{HitRec, Hittable, HittableList};
pub use scene::*;

/// Holds info about an image. Handles rendering.
pub struct Renderer {
    pub width: usize,
    pub height: usize,
    scene: Scene,
    image: Option<Vec<u8>>,
}

impl Renderer {
    pub fn new(width: usize) -> Self {
        Self {
            width,
            height: width,
            scene: Scene::default(),
            image: None,
        }
    }

    pub fn render(&mut self, n_samples: usize) {
        let max_depth = 50;

        let prog_bar = ProgressBar::new(self.height as u64);
        prog_bar.set_style(
            indicatif::ProgressStyle::default_bar()
                .template(
                    "  {msg:.bright.cyan} [{bar:25}] {pos:>4}/{len:4} {elapsed_precise} Eta: {eta_precise}",
                )
                .progress_chars("=> "),
        );
        prog_bar.set_message("Rendering");

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

                        let r = self.scene.camera.get_ray(u, v, &mut rng);
                        pixel_color += Self::ray_color(r, &self.scene.world, &mut rng, max_depth);
                    }
                    Self::write_color(&mut row_buf, pixel_color, n_samples);
                }
                row_buf
            })
            .flatten()
            .collect();

        self.image = Some(img_buf);
    }

    fn ray_color(r: Ray, world: &HittableList, rng: &mut Rng, depth: usize) -> Vec3 {
        if depth <= 0 {
            return Vec3::zero();
        }

        match world.hit(r, 0.001, f64::INFINITY) {
            HitRec::Hit(rec, mat) => match mat {
                Some(mat) => match mat.scatter(r, &rec, rng) {
                    Some((r, c)) => c * Self::ray_color(r, world, rng, depth - 1),
                    None => Vec3::zero(),
                },
                None => rec.n, // No material found, default to color by normal
            },
            HitRec::Miss => {
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

    pub fn scene<T: SceneTrait>(&mut self, scene_gen: T) {
        let mut rng = Rng::new(1234);
        let (height, scene) = scene_gen.scene(self.width, &mut rng);
        self.height = height;
        self.scene = scene;
    }

    pub fn write_image(&self, path: &Path) {
        let file = File::create(path).unwrap();
        let ref mut w = BufWriter::new(file);
        let mut encoder = png::Encoder::new(w, self.width as u32, self.height as u32);
        encoder.set_color(png::ColorType::RGB);
        encoder.set_depth(png::BitDepth::Eight);

        let mut writer = encoder.write_header().unwrap();
        writer
            .write_image_data(self.image.as_ref().unwrap())
            .unwrap();
    }
}
