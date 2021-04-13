use std::fmt::Debug;

use crate::common::Rng;
use crate::geometry::{Ray, Vec3};
use crate::hittable::Rec;

pub trait Material: Debug + Send + Sync {
    fn scatter(&self, r_in: Ray, rec: &Rec, rng: &mut Rng) -> Option<(Ray, Vec3)>;
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Lambertian {
    pub albedo: Vec3,
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: Ray, rec: &Rec, rng: &mut Rng) -> Option<(Ray, Vec3)> {
        let mut scatter_dir = rec.n + Vec3::random_unit_sphere(rng);

        if scatter_dir.near_zero() {
            scatter_dir = rec.n;
        }

        Some((
            Ray {
                o: rec.p,
                d: scatter_dir,
            },
            self.albedo,
        ))
    }
}

#[derive(Clone, Debug, Default)]
pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, r_in: Ray, rec: &Rec, rng: &mut Rng) -> Option<(Ray, Vec3)> {
        let reflected = Vec3::reflect(r_in.d.unit(), rec.n);

        let scattered = Ray {
            o: rec.p,
            d: reflected + self.fuzz * Vec3::random_unit_sphere(rng),
        };

        return if Vec3::dot(scattered.d, rec.n) > 0.0 {
            Some((scattered, self.albedo))
        } else {
            None
        };
    }
}

#[derive(Clone, Debug, Default)]
pub struct Dielectric {
    pub ir: f64,
}

impl Material for Dielectric {
    fn scatter(&self, r_in: Ray, rec: &Rec, rng: &mut Rng) -> Option<(Ray, Vec3)> {
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_dir = r_in.d.unit();
        let cos_theta = Vec3::dot(-unit_dir, rec.n).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let angle_criteria = reflectance(cos_theta, refraction_ratio) > rng.gen();
        let dir = if cannot_refract || angle_criteria {
            Vec3::reflect(unit_dir, rec.n)
        } else {
            Vec3::refract(unit_dir, rec.n, refraction_ratio)
        };

        Some((Ray { o: rec.p, d: dir }, Vec3::splat(1.0)))
    }
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
