use std::cmp::Ordering;

use super::{Ray, Vec3};

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

            if inv_d < 0.0 {
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

    pub fn grow(a: Self, b: Self) -> Self {
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

    pub fn compare(a: Self, b: Self, axis: usize) -> Ordering {
        if a.min[axis] < b.min[axis] {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hit() {
        let aabb = AABB {
            min: Vec3::splat(0.0),
            max: Vec3::splat(1.0),
        };
        let ray = Ray {
            o: Vec3 {
                x: -1.0,
                y: 0.5,
                z: 0.5,
            },
            d: Vec3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
        };
        let ray2 = Ray {
            o: Vec3 {
                x: 0.5,
                y: 0.5,
                z: 0.5,
            },
            d: Vec3 {
                x: 0.1,
                y: 0.1,
                z: 0.1,
            },
        };
        assert_eq!(aabb.hit(ray, 0.0, 10.0), true);
        assert_eq!(aabb.hit(ray, 5.0, 10.0), false);
        assert_eq!(aabb.hit(ray2, 0.0, 10.0), true);

        let ray3 = Ray {
            o: Vec3 {
                x: -5.0,
                y: 1.0,
                z: 0.0,
            },
            d: Vec3 {
                x: 4.5,
                y: -5.6,
                z: 7.5,
            },
        };
        let aabb2 = AABB {
            min: Vec3 {
                x: -1000.0,
                y: -2000.0,
                z: -1000.0,
            },
            max: Vec3 {
                x: 1000.0,
                y: 1.0,
                z: 1000.0,
            },
        };
        assert_eq!(aabb2.hit(ray3, 0.001, f64::INFINITY), true);
    }

    #[test]
    fn grow() {
        let aabb1 = AABB {
            min: Vec3::splat(0.0),
            max: Vec3::splat(1.0),
        };
        let aabb2 = AABB {
            min: Vec3::splat(1.0),
            max: Vec3::splat(2.0),
        };
        assert_eq!(
            AABB::grow(aabb1, aabb2),
            AABB {
                min: Vec3::splat(0.0),
                max: Vec3::splat(2.0),
            }
        );
    }

    #[test]
    fn compare() {
        let box1 = AABB {
            min: Vec3::splat(0.0),
            max: Vec3::splat(1.0),
        };
        let box2 = AABB {
            min: Vec3::splat(3.0),
            max: Vec3::splat(5.0),
        };
        assert_eq!(AABB::compare(box2, box1, 0), Ordering::Greater);
        assert_eq!(AABB::compare(box1, box2, 1), Ordering::Less);
        assert_eq!(AABB::compare(box1, box2, 2), Ordering::Less);
    }
}
