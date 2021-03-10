use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Camera {
    pub ar: f64,
    pub viewport_height: f64,
    pub viewport_width: f64,
    pub focal_length: f64,

    origin: Vec3,
    top_right: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.top_right + (u * self.horizontal) - (v * self.vertical) - self.origin,
        )
    }
}

impl Default for Camera {
    fn default() -> Self {
        let ar = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = ar * viewport_height;
        let focal_length = 1.0;

        let origin = Vec3::zero();
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);

        Self {
            ar,
            viewport_height,
            viewport_width,
            focal_length,
            origin,
            horizontal,
            vertical,
            top_right: origin - (0.5 * horizontal) + (0.5 * vertical)
                - Vec3::new(0.0, 0.0, focal_length),
        }
    }
}
