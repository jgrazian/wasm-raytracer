// https://users.rust-lang.org/t/random-number-without-using-the-external-crate/17260/11

const KX: u32 = 123456789;
const KY: u32 = 362436069;
const KZ: u32 = 521288629;
const KW: u32 = 88675123;

pub struct Rand {
    x: u32,
    y: u32,
    z: u32,
    w: u32,
}

impl Rand {
    pub fn new(seed: u32) -> Rand {
        Rand {
            x: KX ^ seed,
            y: KY ^ seed,
            z: KZ,
            w: KW,
        }
    }

    pub fn set_seed(&mut self, seed: u32) {
        self.x = KX ^ seed;
        self.y = KY ^ seed;
    }

    // Xorshift 128, taken from German Wikipedia
    pub fn rand(&mut self) -> u32 {
        let t = self.x ^ self.x.wrapping_shl(11);
        self.x = self.y;
        self.y = self.z;
        self.z = self.w;
        self.w ^= self.w.wrapping_shr(19) ^ t ^ t.wrapping_shr(8);
        return self.w;
    }

    pub fn rand_range(&mut self, a: i32, b: i32) -> i32 {
        let m = (b - a + 1) as u32;
        return a + (self.rand() % m) as i32;
    }

    #[inline]
    pub fn rand_float(&mut self) -> f32 {
        (self.rand() as f32) / (<u32>::max_value() as f32)
    }

    #[inline]
    pub fn rand_float_range(&mut self, a: f32, b: f32) -> f32 {
        a + (b - a) * self.rand_float()
    }
}
