use super::{HitRec, Hittable, Primative, Ray, AABB};

#[derive(Debug, Clone)]
pub struct BVH {
    objects: Vec<Primative>,
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
