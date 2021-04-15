mod aabb;
mod bvh;
mod sphere;

use std::fmt::Debug;

use crate::geometry::{Ray, Vec3};
use crate::material::Material;

pub use aabb::AABB;
pub use bvh::BVH;
pub use sphere::Sphere;

pub trait Hittable: Debug + Send + Sync {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> HitRec;
    fn aabb(&self, t0: f64, t1: f64) -> Option<AABB>;
}

#[derive(Default, Debug)]
pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::with_capacity(32),
        }
    }

    pub fn push(&mut self, obj: Box<dyn Hittable>) {
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
                (Some(b0), Some(b1)) => out = Some(AABB::grow(b0, b1)),
                (None, _) => return None,
            }
        }

        out
    }
}

#[derive(Debug, Clone, Copy)]
pub enum HitRec<'mat> {
    Hit(Rec, Option<&'mat Box<dyn Material>>),
    Miss,
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Rec {
    pub p: Vec3,
    pub n: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl<'mat> HitRec<'mat> {
    fn hit(
        p: Vec3,
        t: f64,
        r: Ray,
        outward_normal: Vec3,
        mat: Option<&'mat Box<dyn Material>>,
    ) -> Self {
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
            mat,
        )
    }
}
