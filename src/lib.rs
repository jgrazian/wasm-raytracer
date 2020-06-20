mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod vec3;

use std::f64::consts::PI;
use std::f64::INFINITY;
use std::rc::Rc;

use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use ray::Ray;
use sphere::Sphere;
use vec3::{Color, Point3, Vec3};

use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

#[wasm_bindgen]
pub struct Image {
    width: usize,
    height: usize,
    data: Vec<u8>,
    camera: Camera,
    world: HittableList,
}

struct Camera {
    viewport_height: f64,
    viewport_width: f64,
    focal_length: f64,
    origin: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
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
        }
    }

    pub fn render(&mut self) {
        for j in (0..self.height).rev() {
            for i in 0..self.width {
                let index = ((self.height - j - 1) * self.width + i) * 4;

                let u = i as f64 / (self.width - 1) as f64;
                let v = j as f64 / (self.height - 1) as f64;

                let r = Ray::new(
                    self.camera.origin,
                    self.camera.lower_left_corner
                        + u * self.camera.horizontal
                        + v * self.camera.vertical
                        - self.camera.origin,
                );

                let pixel_color = ray_color(r, &self.world);

                self.write_color(pixel_color, index);
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
    fn write_color(&mut self, color: Color, index: usize) {
        self.data[index + 0] = (255.999 * color.x()) as u8;
        self.data[index + 1] = (255.999 * color.y()) as u8;
        self.data[index + 2] = (255.999 * color.z()) as u8;
        self.data[index + 3] = 255;
    }
}

impl Camera {
    fn new() -> Camera {
        let vh = 2.0;
        let vw = 2.0 * (16.0 / 9.0);
        let fl = 1.0;
        let orig = Vec3::zero();
        let hor = Vec3::new(vw, 0.0, 0.0);
        let vert = Vec3::new(0.0, vh, 0.0);
        let ll = orig - hor / 2.0 - vert / 2.0 - Vec3::new(0.0, 0.0, fl);
        Camera {
            viewport_height: vh,
            viewport_width: vw,
            focal_length: fl,
            origin: orig,
            horizontal: hor,
            vertical: vert,
            lower_left_corner: ll,
        }
    }
}

fn ray_color(r: Ray, world: &HittableList) -> Color {
    let mut rec: HitRecord = Default::default();

    if world.hit(r, 0.0, INFINITY, &mut rec) {
        return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
    }

    let unit_dir = Vec3::unit_vector(r.direction());
    let t = 0.5 * (unit_dir.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}
