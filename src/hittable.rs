use std::sync::Arc;

use crate::geometry::{Ray, Vec3, AABB};
use crate::material::Material;

type MatRec = Option<(HitRec, Arc<Material>)>;

pub trait Hittable {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> MatRec;
    fn aabb(&self, t0: f64, t1: f64) -> Option<AABB>;
}

pub enum Primative {
    Sphere { c: Vec3, r: f64, mat: Arc<Material> },
}

impl Hittable for Primative {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> MatRec {
        match self {
            Self::Sphere { c, r, mat } => {
                Self::with_mat(Sphere { c: *c, r: *r }.hit(ray, t_min, t_max), mat)
            }
        }
    }

    fn aabb(&self, t0: f64, t1: f64) -> Option<AABB> {
        match self {
            Self::Sphere { c, r, .. } => Sphere { c: *c, r: *r }.aabb(t0, t1),
        }
    }
}

impl Primative {
    fn with_mat(rec: Option<HitRec>, mat: &Arc<Material>) -> MatRec {
        match rec {
            Some(r) => Some((r, mat.clone())),
            None => None,
        }
    }
}

pub struct HittableList {
    objects: Vec<Primative>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::with_capacity(32),
        }
    }

    pub fn push(&mut self, obj: Primative) {
        self.objects.push(obj);
    }
}

impl Hittable for HittableList {
    #[inline(always)]
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> MatRec {
        let mut rec: MatRec = None;
        let mut closest_so_far = t_max;

        for obj in &self.objects {
            match obj.hit(r, t_min, closest_so_far) {
                Some(h) => {
                    closest_so_far = h.0.t;
                    rec = Some(h);
                }
                None => {}
            }
        }
        rec
    }

    fn aabb(&self, t0: f64, t1: f64) -> Option<AABB> {
        if self.objects.is_empty() {
            return None;
        }

        let mut out: Option<AABB> = None;

        for obj in &self.objects {
            match (obj.aabb(t0, t1), out) {
                (Some(aabb), None) => out = Some(aabb),
                (Some(b0), Some(b1)) => out = Some(AABB::union(b0, b1)),
                (None, _) => return None,
            }
        }

        out
    }
}

#[derive(Clone, Debug, Default)]
pub struct HitRec {
    pub p: Vec3,
    pub n: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRec {
    fn new(p: Vec3, t: f64, r: Ray, outward_normal: Vec3) -> Self {
        // Determine if inside or outside shape. Needed for glass
        let front_face = Vec3::dot(r.d, outward_normal) < 0.0;
        let n = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        Self {
            p,
            n,
            t,
            front_face,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Sphere {
    pub c: Vec3,
    pub r: f64,
}

impl Sphere {
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

        let mut rec = HitRec::new(p, t, r, n);
        Some(rec)
    }

    fn aabb(&self, t0: f64, t1: f64) -> Option<AABB> {
        Some(AABB {
            min: self.c - Vec3::splat(self.r),
            max: self.c + Vec3::splat(self.r),
        })
    }
}
