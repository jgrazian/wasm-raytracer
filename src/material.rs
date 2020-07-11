use std::fmt::Debug;

use crate::common::random_float;
use crate::hitable::HitRecord;
use crate::ray::Ray;
use crate::vec3::{Color, Vec3};

pub trait Mat {
    fn scatter(
        &self,
        r_in: Ray,
        rec: &mut HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
        seed: &mut u32,
    ) -> bool;
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
}

impl Material {
    pub fn scatter(
        &self,
        r_in: Ray,
        rec: &mut HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
        seed: &mut u32,
    ) -> bool {
        match self {
            Material::Lambertian(l) => l.scatter(r_in, rec, attenuation, scattered, seed),
            Material::Metal(m) => m.scatter(r_in, rec, attenuation, scattered, seed),
            Material::Dielectric(d) => d.scatter(r_in, rec, attenuation, scattered, seed),
        }
    }
}

impl Default for Material {
    fn default() -> Material {
        Material::Lambertian(Default::default())
    }
}

#[derive(Debug, PartialEq, Default, Clone, Copy)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(a: Color) -> Lambertian {
        Lambertian { albedo: a }
    }
}

impl Mat for Lambertian {
    fn scatter(
        &self,
        _r_in: Ray,
        rec: &mut HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
        seed: &mut u32,
    ) -> bool {
        let scatter_direction = rec.normal + Vec3::random_in_unit_sphere(seed);
        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}

#[derive(Debug, PartialEq, Default, Clone, Copy)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f32,
}

impl Metal {
    pub fn new(a: Color, f: f32) -> Metal {
        Metal {
            albedo: a,
            fuzz: if f < 1.0 { f } else { 1.0 },
        }
    }
}

impl Mat for Metal {
    fn scatter(
        &self,
        r_in: Ray,
        rec: &mut HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
        seed: &mut u32,
    ) -> bool {
        let reflected = Vec3::reflect(Vec3::unit_vector(r_in.direction()), rec.normal);
        *scattered = Ray::new(
            rec.p,
            reflected + self.fuzz * Vec3::random_in_unit_sphere(seed),
        );
        *attenuation = self.albedo;
        Vec3::dot(scattered.direction(), rec.normal) > 0.0
    }
}

#[derive(Debug, PartialEq, Default, Clone, Copy)]
pub struct Dielectric {
    pub ref_idx: f32,
}

impl Dielectric {
    pub fn new(ref_idx: f32) -> Dielectric {
        Dielectric { ref_idx }
    }
}

impl Mat for Dielectric {
    fn scatter(
        &self,
        r_in: Ray,
        rec: &mut HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
        seed: &mut u32,
    ) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);

        let etai_over_etat: f32 = if rec.front_face {
            1.0 / self.ref_idx
        } else {
            self.ref_idx
        };

        let unit_direction = Vec3::unit_vector(r_in.direction());

        let cos_theta = if Vec3::dot(-unit_direction, rec.normal) < 1.0 {
            Vec3::dot(-unit_direction, rec.normal)
        } else {
            1.0
        };
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        if etai_over_etat * sin_theta > 1.0 {
            let reflected = Vec3::reflect(unit_direction, rec.normal);
            *scattered = Ray::new(rec.p, reflected);
            return true;
        }

        let reflect_prob = schlick(cos_theta, self.ref_idx);
        if random_float(seed) < reflect_prob {
            let reflected = Vec3::reflect(unit_direction, rec.normal);
            *scattered = Ray::new(rec.p, reflected);
            return true;
        }

        let refracted = Vec3::refract(unit_direction, rec.normal, etai_over_etat);
        *scattered = Ray::new(rec.p, refracted);

        true
    }
}

fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
