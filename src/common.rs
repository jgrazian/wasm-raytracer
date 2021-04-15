#[inline]
pub fn clamp(v: f64, min: f64, max: f64) -> f64 {
    if v < min {
        return min;
    }
    if v > max {
        return max;
    }
    v
}

pub struct Rng {
    seed: u32,
}

impl Rng {
    pub fn new(seed: u32) -> Self {
        Self { seed }
    }

    #[inline(always)]
    pub fn gen(&mut self) -> f64 {
        let mut x = self.seed;
        x ^= x >> 13;
        x ^= x << 17;
        x ^= x >> 5;
        self.seed = x;
        let float_bits = (x & 0x007FFFFF) | 0x3F800000;
        let float: f32 = unsafe { ::core::mem::transmute(float_bits) };
        (float - 1.0) as f64
    }

    #[inline(always)]
    pub fn range(&mut self, min: f64, max: f64) -> f64 {
        min + (max - min) * self.gen()
    }

    #[inline(always)]
    pub fn int(&mut self, min: i64, max: i64) -> i64 {
        self.range(min as f64, (max + 1) as f64) as i64
    }
}
