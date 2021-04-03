use super::Ray;
use super::Vec3;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

impl AABB {
    pub fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> bool {
        let mut loc_t_min = t_min;
        let mut loc_t_max = t_max;

        for a in 0..3 {
            let inv_d = 1.0 / r.d[a];
            let mut t0 = (self.min[a] - r.o[a]) * inv_d;
            let mut t1 = (self.max[a] - r.o[a]) * inv_d;

            if inv_d < 1.0 {
                std::mem::swap(&mut t0, &mut t1);
            }

            loc_t_min = if t0 > t_min { t0 } else { loc_t_min };
            loc_t_max = if t1 < t_max { t1 } else { loc_t_max };

            if loc_t_max <= loc_t_min {
                return false;
            }
        }
        true
    }

    pub fn union(a: Self, b: Self) -> Self {
        let small = Vec3 {
            x: a.min.x.min(b.min.x),
            y: a.min.y.min(b.min.y),
            z: a.min.z.min(b.min.z),
        };

        let big = Vec3 {
            x: a.max.x.max(b.max.x),
            y: a.max.y.max(b.max.y),
            z: a.max.z.max(b.max.z),
        };

        Self {
            min: small,
            max: big,
        }
    }
}
