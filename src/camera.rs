use crate::degrees_to_radians;
use crate::rand::Rand;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

#[derive(Debug)]
pub struct Camera {
    pub origin: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lower_left_corner: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperature: f64,
        focus_dist: f64,
    ) -> Camera {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = Vec3::unit_vector(lookfrom - lookat);
        let u = Vec3::unit_vector(Vec3::cross(vup, w));
        let v = Vec3::cross(w, u);

        let orig = lookfrom;
        let hor = focus_dist * viewport_width * u;
        let vert = focus_dist * viewport_height * v;
        let lower_left_corner = orig - hor / 2.0 - vert / 2.0 - focus_dist * w;

        let lens_radius = aperature / 2.0;
        Camera {
            origin: orig,
            horizontal: hor,
            vertical: vert,
            lower_left_corner,
            u,
            v,
            w,
            lens_radius,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64, rng: &mut Rand) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk(rng);
        let offset = self.u * rd.x() + self.v * rd.y();

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }
}
