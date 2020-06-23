use crate::material::Material;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::{Point3, Vec3};

use std::fmt::Debug;

pub trait Hit {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

#[derive(Debug, PartialEq, Default, Clone, Copy)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub material: Material,
    pub t: f64,
    pub front_face: bool,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Hittable {
    Sphere(Sphere),
}

impl Hit for Hittable {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        match self {
            Hittable::Sphere(s) => s.hit(r, t_min, t_max, rec),
        }
    }
}

impl HitRecord {
    #[inline]
    pub fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3) {
        self.front_face = Vec3::dot(r.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}
