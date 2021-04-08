use super::{Arc, HitRec, Hittable, Material, Ray, Sphere, AABB};

#[derive(Debug)]
pub enum Primative {
    Sphere(Sphere, Arc<Material>),
}

impl Hittable for Primative {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> HitRec {
        match self {
            Self::Sphere(s, mat) => s.hit(ray, t_min, t_max).mat(mat.clone()),
        }
    }

    fn aabb(&self, t0: f64, t1: f64) -> Option<AABB> {
        match self {
            Self::Sphere(s, _) => s.aabb(t0, t1),
        }
    }
}
