use crate::geometry::{Ray, Vec3};
use crate::rng::Rng;

#[derive(Debug)]
pub struct Camera {
    origin: Vec3,
    top_right: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lens_radius: f64,

    u: Vec3,
    v: Vec3,
    w: Vec3,
}

impl Camera {
    pub fn new(from: Vec3, at: Vec3, vfov: f64, ar: f64, aperture: f64, focus_dist: f64) -> Self {
        let theta = vfov.to_radians();
        let h = (theta * 0.5).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = ar * viewport_height;

        let w = (from - at).unit();
        let u = Vec3::cross(
            Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            w,
        )
        .unit();
        let v = Vec3::cross(w, u);

        let origin = from;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let top_right = origin - (0.5 * horizontal) + (0.5 * vertical) - focus_dist * w;

        Self {
            origin,
            horizontal,
            vertical,
            top_right,
            lens_radius: 0.5 * aperture,
            u,
            v,
            w,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64, rng: &mut Rng) -> Ray {
        let rd = self.lens_radius * Vec3::random_unit_disk(rng);
        let offset = self.u * rd.x + self.v * rd.y;

        Ray {
            o: self.origin + offset,
            d: self.top_right + (s * self.horizontal) - (t * self.vertical) - self.origin - offset,
        }
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(
            Vec3::zero(),
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            },
            90.0,
            16.0 / 9.0,
            0.1,
            100.0,
        )
    }
}
