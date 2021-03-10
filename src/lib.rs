mod camera;
mod common;
mod hittable;
mod material;
mod ray;
mod vec3;

use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use std::rc::Rc;

use camera::Camera;
use common::{clamp, Rng};
use hittable::{Hittable, HittableList, Object, Sphere};
use material::{Dielectric, Lambertian, Material, MaterialTrait, Metal};
use ray::Ray;
use vec3::Vec3;

struct Renderer {
    pub width: usize,
    pub height: usize,
}

impl Renderer {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }

    pub fn render(&self, n_samples: usize) {
        let mut rng = Rng::new(1234);
        let max_depth = 50;

        // PNG
        let path = Path::new(r"out.png");
        let file = File::create(path).unwrap();
        let ref mut w = BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, self.width as u32, self.height as u32);
        encoder.set_color(png::ColorType::RGB);
        encoder.set_depth(png::BitDepth::Eight);

        let look_from = Vec3::new(13.0, 2.0, 3.0);
        let look_at = Vec3::new(0.0, 0.0, 0.0);
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

        let mut buf: Vec<u8> = Vec::with_capacity(self.width * self.height * 3);
        for j in 0..self.height {
            for i in 0..self.width {
                let mut pixel_color = Vec3::zero();
                for _n in 0..n_samples {
                    let u = (i as f64 + rng.gen()) / ((self.width - 1) as f64);
                    let v = (j as f64 + rng.gen()) / ((self.height - 1) as f64);

                    let r = cam.get_ray(u, v, &mut rng);
                    pixel_color += Self::ray_color(r, &world, &mut rng, max_depth);
                }
                Self::write_color(&mut buf, pixel_color, n_samples);
            }
            println!("Done line {} of {}.", j, self.height);
        }

        let mut writer = encoder.write_header().unwrap();
        writer.write_image_data(&buf).unwrap();
    }

    fn ray_color(r: Ray, world: &HittableList, rng: &mut Rng, depth: usize) -> Vec3 {
        if depth <= 0 {
            return Vec3::zero();
        }

        match world.hit(r, 0.001, f64::INFINITY) {
            Some(rec) => match &rec.mat {
                Some(mat) => match mat.scatter(r, &rec, rng) {
                    Some((r, c)) => c * Self::ray_color(r, world, rng, depth - 1),
                    None => Vec3::zero(),
                },
                None => Vec3::zero(),
            },
            None => {
                let t = 0.5 * (r.d.unit().y + 1.0);
                (1.0 - t) * Vec3::splat(1.0) + t * Vec3::new(0.5, 0.7, 1.0)
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

    let ground_mat = Rc::new(Material::from(Lambertian::new(Vec3::new(0.5, 0.5, 0.5))));

    world.push(Object::from(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Rc::clone(&ground_mat),
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen();
            let center = Vec3::new(a as f64 + 0.9 * rng.gen(), 0.2, b as f64 + 0.9 * rng.gen());

            if (center - Vec3::new(4.0, 0.2, 0.0)).len() > 0.9 {
                let mut mat;

                if choose_mat < 0.8 {
                    let albedo = Vec3::random(rng) * Vec3::random(rng);
                    mat = Rc::new(Material::from(Lambertian::new(albedo)));
                } else if choose_mat < 0.95 {
                    let albedo = Vec3::random_range(rng, 0.5, 1.0);
                    let fuzz = rng.range(0.0, 0.5);
                    mat = Rc::new(Material::from(Metal::new(albedo, fuzz)));
                } else {
                    mat = Rc::new(Material::from(Dielectric::new(1.5)));
                }

                world.push(Object::from(Sphere::new(center, 0.2, Rc::clone(&mat))));
            }
        }
    }

    let mat_1 = Rc::new(Material::from(Dielectric::new(1.5)));
    world.push(Object::from(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Rc::clone(&mat_1),
    )));

    let mat_2 = Rc::new(Material::from(Lambertian::new(Vec3::new(0.4, 0.2, 0.1))));
    world.push(Object::from(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Rc::clone(&mat_2),
    )));

    let mat_3 = Rc::new(Material::from(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0)));
    world.push(Object::from(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Rc::clone(&mat_3),
    )));

    world
}

#[cfg(test)]
mod tests {
    use crate::Renderer;

    #[test]
    fn main() {
        // 960 540 / 426 240
        let r = Renderer::new(1200, 800);
        r.render(500);
    }
}
