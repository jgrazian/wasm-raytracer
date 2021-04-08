use super::{HitRec, Hittable, Ray, Vec3, AABB};

#[derive(Clone, Debug, Default, Copy)]
pub struct Sphere {
    pub c: Vec3,
    pub r: f64,
}

impl Hittable for Sphere {
    #[inline(always)]
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> HitRec {
        let oc = r.o - self.c;
        let a = r.d.len_sq();
        let half_b = Vec3::dot(oc, r.d);
        let c = oc.len_sq() - self.r * self.r;

        let disc = half_b * half_b - a * c;
        if disc < 0.0 {
            return HitRec::Miss;
        }
        let sqrtd = disc.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return HitRec::Miss;
            }
        }

        let t = root;
        let p = r.at(root);
        let n = (p - self.c) / self.r;

        HitRec::hit(p, t, r, n)
    }

    fn aabb(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        Some(AABB {
            min: self.c - Vec3::splat(self.r),
            max: self.c + Vec3::splat(self.r),
        })
    }
}
