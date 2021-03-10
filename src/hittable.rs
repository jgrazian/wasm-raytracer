use std::rc::Rc;

use enum_dispatch::enum_dispatch;

use crate::material::Mat;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[enum_dispatch]
pub trait Hittable {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRec>;
}

//#[enum_dispatch(Hittable)]
pub enum Object<'a> {
    Sphere(Sphere<'a>),
}

impl Hittable for Object<'_> {
    #[inline(always)]
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRec> {
        match self {
            Object::Sphere(s) => s.hit(r, t_min, t_max),
        }
    }
}

impl<'a> From<Sphere<'a>> for Object<'a> {
    fn from(sphere: Sphere<'a>) -> Self {
        Object::Sphere(sphere)
    }
}

pub struct HittableList<'a> {
    objects: Vec<Object<'a>>,
}

impl<'a> HittableList<'a> {
    pub fn new() -> Self {
        Self {
            objects: Vec::with_capacity(16),
        }
    }

    pub fn push(&mut self, obj: Object<'a>) {
        self.objects.push(obj);
    }
}

impl Hittable for HittableList<'_> {
    #[inline(always)]
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRec> {
        let mut rec: Option<HitRec> = None;
        let mut closest_so_far = t_max;

        for obj in &self.objects {
            match obj.hit(r, t_min, closest_so_far) {
                Some(h) => {
                    closest_so_far = h.t;
                    rec = Some(h);
                }
                None => {}
            }
        }
        rec
    }
}

#[derive(Clone, Debug, Default)]
pub struct HitRec<'mat> {
    pub p: Vec3,
    pub n: Vec3,
    pub t: f64,
    pub mat: Option<&'mat Mat>,
    pub front_face: bool,
}

impl<'a> HitRec<'a> {
    fn new(p: Vec3, n: Vec3, t: f64, mat: Option<&'a Mat>) -> Self {
        Self {
            p,
            n,
            t,
            front_face: false,
            mat,
        }
    }

    fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3) {
        self.front_face = Vec3::dot(r.d, outward_normal) < 0.0;
        if self.front_face {
            self.n = outward_normal;
        } else {
            self.n = -outward_normal;
        }
    }
}

#[derive(Clone, Debug)]
pub struct Sphere<'mat> {
    pub c: Vec3,
    pub r: f64,
    pub mat: &'mat Mat,
}

impl<'a> Sphere<'a> {
    pub fn new(c: Vec3, r: f64, mat: &'a Mat) -> Self {
        Self { c, r, mat }
    }
}

impl<'a> Hittable for Sphere<'a> {
    #[inline(always)]
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRec> {
        let oc = r.o - self.c;
        let a = r.d.len_sq();
        let half_b = Vec3::dot(oc, r.d);
        let c = oc.len_sq() - self.r * self.r;

        let disc = half_b * half_b - a * c;
        if disc < 0.0 {
            return None;
        }
        let sqrtd = disc.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let t = root;
        let p = r.at(root);
        let n = (p - self.c) / self.r;

        let mut rec = HitRec::new(p, n, t, Some(self.mat));
        rec.set_face_normal(r, n);
        Some(rec)
    }
}
