use super::{HitRec, Hittable, Ray, AABB};

#[derive(Debug, Default)]
pub struct BVH {
    objects: Vec<Box<dyn Hittable>>,
}

impl BVH {}

impl Hittable for BVH {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> HitRec {
        HitRec::Miss
    }

    fn aabb(&self, t0: f64, t1: f64) -> Option<AABB> {
        None
    }
}
