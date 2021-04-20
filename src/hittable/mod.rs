mod aabb;
mod bvh;
mod hitrec;
mod sphere;

use std::fmt::Debug;

use crate::geometry::{Ray, Vec3};
use crate::rng::Rng;

pub use aabb::AABB;
pub use bvh::BVH;
pub use hitrec::{HitRec, Rec};
pub use sphere::Sphere;

pub trait Hittable: Send + Sync + Debug {
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

    pub fn push<T: Hittable + 'static>(&mut self, obj: T) {
        self.objects.push(Box::new(obj));
    }

    pub fn into_bvh(&mut self, rng: &mut Rng) {
        let bvh = BVH::build(&mut self.objects, 0.0, 0.0, rng);
        self.push(bvh);
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
