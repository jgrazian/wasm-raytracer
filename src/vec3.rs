use auto_ops::{impl_op_ex, impl_op_ex_commutative};

use crate::common::Rng;

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    /// Creates a new Vec2 with the same value for x and y
    pub fn splat(v: f64) -> Self {
        Self { x: v, y: v, z: v }
    }

    /// Length^2
    #[inline(always)]
    pub fn len_sq(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Length
    #[inline(always)]
    pub fn len(&self) -> f64 {
        self.len_sq().sqrt()
    }

    /// Normalize
    #[inline(always)]
    pub fn unit(&self) -> Self {
        let l = 1.0 / self.len();
        Self {
            x: self.x * l,
            y: self.y * l,
            z: self.z * l,
        }
    }

    #[inline(always)]
    pub fn dot(u: Self, v: Self) -> f64 {
        u.x * v.x + u.y * v.y + u.z * v.z
    }

    #[inline(always)]
    pub fn cross(u: Self, v: Self) -> Self {
        Self {
            x: u.y * v.z - u.z * v.y,
            y: u.z * v.x - u.x * v.z,
            z: u.x * v.y - u.y * v.x,
        }
    }

    #[inline(always)]
    pub fn reflect(v: Self, n: Self) -> Self {
        v - 2.0 * Vec3::dot(v, n) * n
    }

    #[inline(always)]
    pub fn refract(v: Self, n: Self, eta: f64) -> Self {
        let cos_theta = Vec3::dot(-v, n).min(1.0);
        let perp = eta * (v + cos_theta * n);
        let parallel = -((1.0 - perp.len_sq()).abs().sqrt()) * n;
        perp + parallel
    }

    #[inline(always)]
    pub fn near_zero(&self) -> bool {
        const ETA: f64 = 1e-8;
        (self.x.abs() < ETA) && (self.y.abs() < ETA) && (self.z.abs() < ETA)
    }
}

impl Vec3 {
    #[inline(always)]
    pub fn random(rng: &mut Rng) -> Self {
        Self {
            x: rng.gen(),
            y: rng.gen(),
            z: rng.gen(),
        }
    }

    #[inline(always)]
    pub fn random_range(rng: &mut Rng, min: f64, max: f64) -> Self {
        Self {
            x: rng.range(min, max),
            y: rng.range(min, max),
            z: rng.range(min, max),
        }
    }

    #[inline(always)]
    pub fn random_unit_sphere(rng: &mut Rng) -> Self {
        Self::random_range(rng, -1.0, 1.0).unit()
    }

    #[inline(always)]
    pub fn random_unit_disk(rng: &mut Rng) -> Self {
        loop {
            let p = Vec3 {
                x: rng.range(-1.0, 1.0),
                y: rng.range(-1.0, 1.0),
                z: 0.0,
            };
            if p.len_sq() >= 1.0 {
                continue;
            }
            return p;
        }
    }
}

// Add
impl_op_ex!(+|a: &Vec3, b: &Vec3| -> Vec3 { Vec3{x: a.x + b.x, y: a.y + b.y, z: a.z + b.z} });
impl_op_ex_commutative!(+|a: &Vec3, b: &f64| -> Vec3 { Vec3{x: a.x + b, y: a.y + b, z: a.z + b} });

// Assign Add
impl_op_ex!(+=|a: &mut Vec3, b: &Vec3| { a.x += b.x; a.y += b.y; a.z += b.z });
impl_op_ex!(+=|a: &mut Vec3, b: &f64| { a.x += b; a.y += b; a.z += b });

// Sub
impl_op_ex!(-|a: &Vec3, b: &Vec3| -> Vec3 {
    Vec3 {
        x: a.x - b.x,
        y: a.y - b.y,
        z: a.z - b.z,
    }
});
impl_op_ex!(-|a: &Vec3, b: &f64| -> Vec3 {
    Vec3 {
        x: a.x - b,
        y: a.y - b,
        z: a.z - b,
    }
});
impl_op_ex!(-|a: &f64, b: &Vec3| -> Vec3 {
    Vec3 {
        x: a - b.x,
        y: a - b.y,
        z: a - b.z,
    }
});

// Assign Sub
impl_op_ex!(-=|a: &mut Vec3, b: &Vec3| { a.x -= b.x; a.y -= b.y; a.z -= b.z });
impl_op_ex!(-=|a: &mut Vec3, b: &f64| { a.x -= b; a.y -= b; a.z -= b });

// Mul
impl_op_ex!(*|a: &Vec3, b: &Vec3| -> Vec3 {
    Vec3 {
        x: a.x * b.x,
        y: a.y * b.y,
        z: a.z * b.z,
    }
});
impl_op_ex_commutative!(*|a: &Vec3, b: &f64| -> Vec3 {
    Vec3 {
        x: a.x * b,
        y: a.y * b,
        z: a.z * b,
    }
});

// Assign Mul
impl_op_ex!(*=|a: &mut Vec3, b: &Vec3| { a.x *= b.x; a.y *= b.y; a.z *= b.z });
impl_op_ex!(*=|a: &mut Vec3, b: &f64| { a.x *= b; a.y *= b; a.z *= b });

// Div
impl_op_ex!(/|a: &Vec3, b: &Vec3| -> Vec3 { Vec3{x: a.x / b.x, y: a.y / b.y, z: a.z / b.z} });
impl_op_ex!(/|a: &Vec3, b: &f64| -> Vec3 { Vec3{x: a.x / b, y: a.y / b, z: a.z / b} });
impl_op_ex!(/|a: &f64, b: &Vec3| -> Vec3 { Vec3{x: a / b.x, y: a / b.y, z: a / b.z} });

// Assign Div
impl_op_ex!(/=|a: &mut Vec3, b: &Vec3| { a.x /= b.x; a.y /= b.y; a.z /= b.z });
impl_op_ex!(/=|a: &mut Vec3, b: &f64| { a.x /= b; a.y /= b; a.z /= b });

// Uniary Neg
impl_op_ex!(-|a: Vec3| -> Vec3 {
    Vec3 {
        x: -a.x,
        y: -a.y,
        z: -a.z,
    }
});
