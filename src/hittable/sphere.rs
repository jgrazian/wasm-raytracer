use std::f64::consts::PI;
use std::sync::Arc;

use super::{HitRec, Hittable, Ray, Vec3, AABB};
use crate::material::Material;

#[derive(Default, Debug)]
pub struct Sphere {
    pub c: Vec3,
    pub r: f64,
    pub mat: Option<Arc<dyn Material>>,
}

impl Sphere {
    fn uv(p: Vec3) -> (f64, f64) {
        let theta = -p.y.acos();
        let phi = -p.z.atan2(p.x) + PI;

        (phi / (2.0 * PI), theta / PI)
    }
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
        let (u, v) = Self::uv(n);
        if let Some(ref mat) = self.mat {
            HitRec::hit(p, t, u, v, r, n, Some(mat))
        } else {
            HitRec::hit(p, t, u, v, r, n, None)
        }
    }

    fn aabb(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        Some(AABB {
            min: self.c - Vec3::splat(self.r),
            max: self.c + Vec3::splat(self.r),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hit() {
        let s = Sphere {
            c: Vec3::zero(),
            r: 1.0,
            mat: None,
        };
        let r1 = Ray {
            o: Vec3::splat(2.0),
            d: Vec3::splat(1.0),
        };
        let r2 = Ray {
            o: Vec3::splat(-2.0),
            d: Vec3::splat(1.0),
        };

        assert!(match s.hit(r1, 0.0, 10.0) {
            HitRec::Miss => true,
            _ => false,
        });
        assert!(match s.hit(r2, 0.0, 10.0) {
            HitRec::Hit(_, _) => true,
            _ => false,
        });
    }

    #[test]
    fn aabb() {
        let s = Sphere {
            c: Vec3::zero(),
            r: 1.0,
            mat: None,
        };

        assert_eq!(
            s.aabb(0.0, 0.0),
            Some(AABB {
                min: Vec3::splat(-1.0),
                max: Vec3::splat(1.0)
            })
        );
    }
}
