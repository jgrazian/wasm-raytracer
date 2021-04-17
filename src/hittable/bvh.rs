use std::cmp::Ordering;

use super::{HitRec, Hittable, Ray, AABB};
use crate::rng::Rng;

pub struct BVH {
    bounds: AABB,
    left: Option<Box<dyn Hittable>>,
    right: Option<Box<dyn Hittable>>,
}

impl BVH {
    pub fn build(objects: &mut Vec<Box<dyn Hittable>>, t0: f64, t1: f64, rng: &mut Rng) -> Self {
        let axis = rng.int(0, 2) as usize;

        let (left, right) = match objects.len() {
            1 => (objects.pop(), None),
            2 => {
                let a = objects.pop().unwrap();
                let b = objects.pop().unwrap();
                if box_compare(&a, &b, axis) == Ordering::Less {
                    (Some(a), Some(b))
                } else {
                    (Some(b), Some(a))
                }
            }
            _ => {
                objects.sort_by(|a, b| box_compare(a, b, axis));

                let mut obj_right = objects.split_off(objects.len() / 2);
                let left = Self::build(objects, t0, t1, rng);
                let right = Self::build(&mut obj_right, t0, t1, rng);

                (
                    Some(Box::new(left) as Box<dyn Hittable>),
                    Some(Box::new(right) as Box<dyn Hittable>),
                )
            }
        };

        let bounds = match (&left, &right) {
            (Some(l), None) => l.aabb(t0, t1).unwrap(),
            (None, Some(r)) => r.aabb(t0, t1).unwrap(),

            (Some(l), Some(r)) => match (l.aabb(t0, t1), r.aabb(t0, t1)) {
                (Some(aa), Some(bb)) => AABB::grow(aa, bb),
                _ => panic!("No bounding box."),
            },

            _ => panic!("asdas"),
        };

        Self {
            bounds,
            left,
            right,
        }
    }
}

impl Hittable for BVH {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> HitRec {
        if !self.bounds.hit(ray, t_min, t_max) {
            return HitRec::Miss;
        }

        let mut out = match &self.left {
            Some(l) => l.hit(ray, t_min, t_max),
            None => HitRec::Miss,
        };
        // If we hit the left node the right must be closer
        out = match (&self.right, out) {
            (Some(r), HitRec::Hit(rec, _)) => match r.hit(ray, t_min, rec.t) {
                HitRec::Hit(new_rec, new_mat) => HitRec::Hit(new_rec, new_mat),
                HitRec::Miss => out,
            },
            (Some(r), HitRec::Miss) => r.hit(ray, t_min, t_max),
            (None, _) => out,
        };

        out
    }

    fn aabb(&self, _: f64, _: f64) -> Option<AABB> {
        Some(self.bounds)
    }
}

fn box_compare(a: &Box<dyn Hittable>, b: &Box<dyn Hittable>, axis: usize) -> Ordering {
    let box_a = a.aabb(0.0, 0.0);
    let box_b = b.aabb(0.0, 0.0);

    match (box_a, box_b) {
        (Some(aa), Some(bb)) => AABB::compare(aa, bb, axis),
        _ => panic!("No bounding box."),
    }
}

#[cfg(test)]
mod tests {
    use crate::{geometry::Vec3, hittable::Sphere};

    use super::*;

    #[test]
    fn build() {
        let mut rng = Rng::new(1234);

        let mut objects: Vec<Box<dyn Hittable>> = (0..8)
            .into_iter()
            .map(|_| {
                Box::new(Sphere {
                    c: Vec3::random_range(&mut rng, -5.0, 5.0),
                    r: 0.5,
                    mat: None,
                }) as Box<dyn Hittable>
            })
            .collect();

        let bvh = BVH::build(&mut objects, 0.0, 0.0, &mut rng);
    }

    #[test]
    fn hit() {
        let mut rng = Rng::new(1234);

        let obj1 = Box::new(Sphere {
            c: Vec3::zero(),
            r: 1.0,
            mat: None,
        }) as Box<dyn Hittable>;

        let obj2 = Box::new(Sphere {
            c: Vec3::splat(2.0),
            r: 1.0,
            mat: None,
        }) as Box<dyn Hittable>;

        let obj3 = Box::new(Sphere {
            c: Vec3::splat(-2.0),
            r: 1.0,
            mat: None,
        }) as Box<dyn Hittable>;

        let mut objects: Vec<Box<dyn Hittable>> = vec![obj1, obj2, obj3];

        let bvh = BVH::build(&mut objects, 0.0, 0.0, &mut rng);

        let ray1 = Ray {
            o: Vec3 {
                x: -3.0,
                y: 0.0,
                z: 0.0,
            },
            d: Vec3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
        };

        match bvh.hit(ray1, 0.0, f64::INFINITY) {
            HitRec::Hit(rec, _) => assert_eq!(
                rec.p,
                Vec3 {
                    x: -1.0,
                    y: 0.0,
                    z: 0.0
                }
            ),
            HitRec::Miss => assert!(false),
        }

        let ray2 = Ray {
            o: Vec3 {
                x: 2.0,
                y: 5.0,
                z: 2.0,
            },
            d: Vec3 {
                x: 0.0,
                y: -1.0,
                z: 0.0,
            },
        };

        match bvh.hit(ray2, 0.0, f64::INFINITY) {
            HitRec::Hit(rec, _) => assert_eq!(
                rec.p,
                Vec3 {
                    x: 2.0,
                    y: 3.0,
                    z: 2.0
                }
            ),
            HitRec::Miss => assert!(false),
        }
    }
}
