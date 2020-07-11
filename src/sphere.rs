use crate::hitable::{Hit, HitRecord};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

#[derive(Debug, PartialEq, Default, Clone, Copy)]
pub struct Sphere {
    center: Point3,
    radius: f32,
    material: Material,
}

impl Sphere {
    pub fn new(c: Point3, r: f32, mat: Material) -> Sphere {
        Sphere {
            center: c,
            radius: r,
            material: mat,
        }
    }
}

impl Hit for Sphere {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = Vec3::dot(oc, r.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let mut tmp = (-half_b - root) / a;
            if tmp < t_max && tmp > t_min {
                rec.t = tmp;
                rec.p = r.at(rec.t);
                let outward_normal = (rec.p - self.center) / self.radius;
                rec.set_face_normal(r, outward_normal);
                rec.material = self.material;
                return true;
            }
            tmp = (-half_b + root) / a;
            if tmp < t_max && tmp > t_min {
                rec.t = tmp;
                rec.p = r.at(rec.t);
                let outward_normal = (rec.p - self.center) / self.radius;
                rec.set_face_normal(r, outward_normal);
                rec.material = self.material;
                return true;
            }
        }
        false
    }
}
