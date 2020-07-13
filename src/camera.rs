use crate::degrees_to_radians;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

#[derive(Debug)]
pub struct Camera {
    origin: Vec3,
    target: Vec3,
    vertical_fov: f32,
    aspect_ratio: f32,
    focus_distance: f32,
    aperature: f32,

    _horizontal: Vec3,
    _vertical: Vec3,
    _lower_left_corner: Vec3,
    _u: Vec3,
    _v: Vec3,
    _w: Vec3,
    _lens_radius: f32,
}

impl Camera {
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        vfov: f32,
        aspect_ratio: f32,
        aperature: f32,
        focus_dist: f32,
    ) -> Camera {
        let mut camera = Camera {
            origin: lookfrom,
            target: lookat,
            vertical_fov: vfov,
            aspect_ratio: aspect_ratio,
            focus_distance: focus_dist,
            aperature: aperature,

            _horizontal: Vec3::zero(),
            _vertical: Vec3::zero(),
            _lower_left_corner: Vec3::zero(),
            _u: Vec3::zero(),
            _v: Vec3::zero(),
            _w: Vec3::zero(),
            _lens_radius: 0.0,
        };
        camera.recalculate();
        camera
    }

    fn recalculate(&mut self) {
        let theta = degrees_to_radians(self.vertical_fov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = self.aspect_ratio * viewport_height;

        self._w = Vec3::unit_vector(self.origin - self.target);
        self._u = Vec3::unit_vector(Vec3::cross(Vec3::new(0.0, 1.0, 0.0), self._w));
        self._v = Vec3::cross(self._w, self._u);

        self._horizontal = self.focus_distance * viewport_width * self._u;
        self._vertical = self.focus_distance * viewport_height * self._v;
        self._lower_left_corner = self.origin
            - self._horizontal / 2.0
            - self._vertical / 2.0
            - self.focus_distance * self._w;

        self._lens_radius = self.aperature / 2.0;
    }

    pub fn get_ray(&self, s: f32, t: f32, seed: &mut u32) -> Ray {
        let rd = self._lens_radius * Vec3::random_in_unit_disk(seed);
        let offset = self._u * rd.x() + self._v * rd.y();

        Ray::new(
            self.origin + offset,
            self._lower_left_corner + s * self._horizontal + t * self._vertical
                - self.origin
                - offset,
        )
    }

    pub fn set_origin(&mut self, origin: Vec3) {
        self.origin = origin;
        self.recalculate();
    }

    pub fn set_target(&mut self, target: Vec3) {
        self.target = target;
        self.recalculate();
    }

    pub fn set_focus(&mut self, d: f32) {
        self.focus_distance = d;
        self.recalculate();
    }
}
