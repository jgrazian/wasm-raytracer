use std::fmt::Debug;

use crate::geometry::Vec3;
use crate::perlin::Perlin;

pub trait Texture: Send + Sync + Debug {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Vec3;
}

#[derive(Debug)]
pub struct SolidColor {
    pub color_value: Vec3,
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: Vec3) -> Vec3 {
        self.color_value
    }
}

#[derive(Debug)]
pub struct CheckerTexture {
    pub odd: Box<dyn Texture>,
    pub even: Box<dyn Texture>,
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}

#[derive(Debug)]
pub struct NoiseTexture {
    pub noise: Perlin,
    pub scale: f64,
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: Vec3) -> Vec3 {
        // Vec3::splat(1.0) * 0.5 * (1.0 + self.noise.noise(self.scale * p))
        // Vec3::splat(1.0) * self.noise.turb(self.scale * p)
        Vec3::splat(1.0) * 0.5 * (1.0 + (self.scale * p.z + 10.0 * self.noise.turb(p, 7)).sin())
    }
}
