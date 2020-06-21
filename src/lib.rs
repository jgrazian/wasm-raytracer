mod camera;
mod hittable;
mod hittable_list;
mod rand;
mod ray;
mod sphere;
mod vec3;

use std::f64::consts::PI;
use std::f64::INFINITY;
use std::rc::Rc;

use camera::Camera;
use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use rand::Rand;
use ray::Ray;
use sphere::Sphere;
use vec3::{Color, Point3, Vec3};

use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[inline]
fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

#[inline]
fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    x
}

#[wasm_bindgen]
pub struct Image {
    width: usize,
    height: usize,
    data: Vec<u8>,
    camera: Camera,
    world: HittableList,
    rng: Rand,
}

#[wasm_bindgen]
impl Image {
    pub fn new(w: usize, h: usize) -> Image {
        let sphere_1 = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5);
        let sphere_2 = Sphere::new(Point3::new(0.0, -100.5, 1.0), 100.0);
        let mut world = HittableList::new();
        world.add(Rc::new(sphere_1));
        world.add(Rc::new(sphere_2));

        Image {
            width: w,
            height: h,
            data: vec![0; w * h * 4],
            camera: Camera::new(),
            world: world,
            rng: Rand::new(0),
        }
    }

    pub fn render(&mut self) {
        const samples_per_pixel: usize = 25;
        const max_depth: usize = 50;
        for j in (0..self.height).rev() {
            for i in 0..self.width {
                let index = ((self.height - j - 1) * self.width + i) * 4;

                let mut pixel_color = Color::zero();

                for s in 0..samples_per_pixel {
                    let u = (i as f64 + self.rng.rand_float()) / (self.width - 1) as f64;
                    let v = (j as f64 + self.rng.rand_float()) / (self.height - 1) as f64;

                    let r = self.camera.get_ray(u, v);

                    pixel_color += ray_color(r, &self.world, &mut self.rng, max_depth);
                }

                self.write_color(pixel_color, index, samples_per_pixel);
            }
        }
    }

    pub fn get_image_data_ptr(&self) -> *const u8 {
        self.data.as_ptr()
    }

    pub fn get_image_data_len(&self) -> usize {
        self.width * self.height * 4
    }
}

impl Image {
    fn write_color(&mut self, pixel_color: Color, index: usize, samples_per_pixel: usize) {
        let mut r = pixel_color.x();
        let mut g = pixel_color.y();
        let mut b = pixel_color.z();

        let scale = 1.0 / samples_per_pixel as f64;
        r = (scale * r).sqrt();
        g = (scale * g).sqrt();
        b = (scale * b).sqrt();

        self.data[index + 0] = (256.0 * clamp(r, 0.0, 0.999)) as u8;
        self.data[index + 1] = (256.0 * clamp(g, 0.0, 0.999)) as u8;
        self.data[index + 2] = (256.0 * clamp(b, 0.0, 0.999)) as u8;
        self.data[index + 3] = 255;
    }
}

fn ray_color(r: Ray, world: &HittableList, rng: &mut Rand, depth: usize) -> Color {
    let mut rec: HitRecord = Default::default();
    if depth <= 0 {
        return Color::zero();
    }

    if world.hit(r, 0.001, INFINITY, &mut rec) {
        let target = rec.p + rec.normal + Vec3::random_unit_vector(rng);
        return 0.5 * ray_color(Ray::new(rec.p, target - rec.p), world, rng, depth - 1);
    }

    let unit_dir = Vec3::unit_vector(r.direction());
    let t = 0.5 * (unit_dir.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}
