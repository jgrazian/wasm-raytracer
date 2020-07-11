use std::f32::consts::PI;

#[inline]
pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}

#[inline]
pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    x
}

// Returns a random value in the range [0.0, 1.0)
#[inline]
pub fn random_float(seed: &mut u32) -> f32 {
    let mut x = *seed;
    x ^= x >> 13;
    x ^= x << 17;
    x ^= x >> 5;
    *seed = x;
    let float_bits = (x & 0x007FFFFF) | 0x3F800000;
    let float: f32 = unsafe { ::core::mem::transmute(float_bits) };
    float - 1.0
}

// Returns a random value in given range
#[inline]
pub fn random_range(seed: &mut u32, min: f32, max: f32) -> f32 {
    min + (max - min) * random_float(seed)
}
