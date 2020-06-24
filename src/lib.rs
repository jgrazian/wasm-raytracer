mod camera;
mod hittable;
mod hittable_list;
mod material;
mod rand;
mod ray;
mod sphere;
mod vec3;

use std::f64::consts::PI;
use std::f64::INFINITY;

use camera::Camera;
use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use material::*;
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
        let R = (PI / 4.0).cos();
        let sphere_1 = Hittable::Sphere(Sphere::new(
            Point3::new(0.0, 0.0, -1.0),
            0.5,
            Material::Lambertian(Lambertian::new(Color::new(0.0, 0.0, 1.0))),
        ));
        let sphere_2 = Hittable::Sphere(Sphere::new(
            Point3::new(0.0, -100.5, -1.0),
            100.0,
            Material::Lambertian(Lambertian::new(Color::new(0.8, 0.8, 0.0))),
        ));
        let sphere_3 = Hittable::Sphere(Sphere::new(
            Point3::new(1.0, 0.0, -1.0),
            0.5,
            Material::Metal(Metal::new(Color::new(0.8, 0.6, 0.2), 0.2)),
        ));
        let sphere_4 = Hittable::Sphere(Sphere::new(
            Point3::new(-1.0, 0.0, -1.0),
            0.5,
            Material::Dielectric(Dielectric::new(1.5)),
        ));

        let mut world = HittableList::new();
        world.add(sphere_1);
        world.add(sphere_2);
        world.add(sphere_3);
        world.add(sphere_4);

        let lookfrom = Point3::new(3.0, 0.0, 2.0);
        let lookat = Point3::new(0.0, 0.0, -1.0);

        let cam = Camera::new(
            lookfrom,
            lookat,
            Vec3::new(0.0, 1.0, 0.0),
            20.0,
            w as f64 / h as f64,
            0.1,
            (lookfrom - lookat).length(),
        );

        Image {
            width: w,
            height: h,
            data: vec![0; w * h * 4],
            camera: cam,
            world: world,
            rng: Rand::new(0),
        }
    }

    pub fn render(&mut self) {
        const samples_per_pixel: usize = 100;
        const max_depth: usize = 50;
        for j in (0..self.height).rev() {
            for i in 0..self.width {
                let index = ((self.height - j - 1) * self.width + i) * 4;

                let mut pixel_color = Color::zero();

                for s in 0..samples_per_pixel {
                    let u = (i as f64 + self.rng.rand_float()) / (self.width - 1) as f64;
                    let v = (j as f64 + self.rng.rand_float()) / (self.height - 1) as f64;

                    let r = self.camera.get_ray(u, v, &mut self.rng);

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

fn ray_color(r: Ray, world: &HittableList, mut rng: &mut Rand, depth: usize) -> Color {
    let mut rec: HitRecord = Default::default();
    if depth <= 0 {
        return Color::zero();
    }

    if world.hit(r, 0.001, INFINITY, &mut rec) {
        let mut scattered: Ray = Default::default();
        let mut attenuation: Color = Default::default();
        let mut rec_copy = rec;
        if rec
            .material
            .scatter(r, &mut rec_copy, &mut attenuation, &mut scattered, &mut rng)
        {
            return attenuation * ray_color(scattered, world, rng, depth - 1);
        }
        return Color::new(0.0, 0.0, 0.0);
    }
    let unit_dir = Vec3::unit_vector(r.direction());
    let t = 0.5 * (unit_dir.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}
