use std::fmt::Debug;

use crate::hittable::HitRecord;
use crate::rand::Rand;
use crate::ray::Ray;
use crate::vec3::{Color, Vec3};

pub trait Mat {
    fn scatter(
        &self,
        r_in: Ray,
        rec: &mut HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
        rng: &mut Rand,
    ) -> bool {
        true
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
}

impl Material {
    pub fn scatter(
        &self,
        r_in: Ray,
        mut rec: &mut HitRecord,
        mut attenuation: &mut Color,
        mut scattered: &mut Ray,
        mut rng: &mut Rand,
    ) -> bool {
        match self {
            Material::Lambertian(l) => {
                l.scatter(r_in, &mut rec, &mut attenuation, &mut scattered, &mut rng)
            }
            Material::Metal(m) => {
                m.scatter(r_in, &mut rec, &mut attenuation, &mut scattered, &mut rng)
            }
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
        r_in: Ray,
        rec: &mut HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
        rng: &mut Rand,
    ) -> bool {
        let scatter_direction = rec.normal + Vec3::random_in_unit_sphere(rng);
        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}

#[derive(Debug, PartialEq, Default, Clone, Copy)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(a: Color, f: f64) -> Metal {
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
        rng: &mut Rand,
    ) -> bool {
        let reflected = Vec3::reflect(Vec3::unit_vector(r_in.direction()), rec.normal);
        *scattered = Ray::new(
            rec.p,
            reflected + self.fuzz * Vec3::random_in_unit_sphere(rng),
        );
        *attenuation = self.albedo;
        Vec3::dot(scattered.direction(), rec.normal) > 0.0
    }
}
