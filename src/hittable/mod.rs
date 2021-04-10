mod bvh;
mod primative;
mod sphere;

use std::sync::Arc;

use crate::geometry::{Ray, Vec3, AABB};
use crate::material::Material;

pub use bvh::BVH;
pub use primative::Primative;
pub use sphere::Sphere;

pub trait Hittable {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> HitRec;
    fn aabb(&self, t0: f64, t1: f64) -> Option<AABB>;
}

#[derive(Debug, Default)]
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
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> HitRec {
        let mut out_rec: HitRec = HitRec::Miss;
        let mut closest_so_far = t_max;

        for obj in &self.objects {
            match obj.hit(r, t_min, closest_so_far) {
                HitRec::Hit(rec, mat) => {
                    closest_so_far = rec.t;
                    out_rec = HitRec::Hit(rec, mat);
                }
                HitRec::Miss => {}
            };
        }
        out_rec
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

pub enum HitRec {
    Hit(Rec, Option<Arc<Material>>),
    Miss,
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Rec {
    pub p: Vec3,
    pub n: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRec {
    fn hit(p: Vec3, t: f64, r: Ray, outward_normal: Vec3) -> Self {
        // Determine if inside or outside shape. Needed for glass
        let front_face = Vec3::dot(r.d, outward_normal) < 0.0;
        let n = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        Self::Hit(
            Rec {
                p,
                n,
                t,
                front_face,
            },
            None,
        )
    }

    fn mat(&self, material: Arc<Material>) -> Self {
        match self {
            Self::Hit(rec, _) => Self::Hit(*rec, Some(material)),
            _ => Self::Miss,
        }
    }
}
