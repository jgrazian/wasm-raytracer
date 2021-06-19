use std::clone::Clone;
use std::fmt::Debug;
use std::sync::Arc;

use crate::geometry::{Ray, Vec3};
use crate::hittable::Rec;
use crate::rng::Rng;
use crate::texture::{SolidColor, Texture};

pub trait Material: Sync + Send + Debug {
    fn scatter(&self, r_in: Ray, rec: &Rec, rng: &mut Rng) -> Option<(Ray, Vec3)>;
    fn emitted(&self, _u: f64, _v: f64, _p: Vec3) -> Vec3 {
        Vec3::zero()
    }
}

#[derive(Debug, Clone)]
pub struct Lambertian {
    pub albedo: Arc<dyn Texture>,
}

impl From<Vec3> for Lambertian {
    fn from(color: Vec3) -> Self {
        Self {
            albedo: Arc::new(SolidColor { color_value: color }),
        }
    }
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
            self.albedo.value(rec.u, rec.v, rec.p),
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

#[derive(Clone, Debug)]
pub struct DiffuseLight {
    pub emit: Arc<dyn Texture>,
}

impl Material for DiffuseLight {
    fn scatter(&self, _r_in: Ray, _rec: &Rec, _rng: &mut Rng) -> Option<(Ray, Vec3)> {
        None
    }

    fn emitted(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        self.emit.value(u, v, p)
    }
}
