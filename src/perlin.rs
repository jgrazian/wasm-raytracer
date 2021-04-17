use crate::geometry::Vec3;
use crate::rng::Rng;

pub struct Perlin {
    ranf: [f64; Self::POINT_COUNT],
    perm_x: [usize; Self::POINT_COUNT],
    perm_y: [usize; Self::POINT_COUNT],
    perm_z: [usize; Self::POINT_COUNT],
}

impl Perlin {
    const POINT_COUNT: usize = 256;

    pub fn new() -> Self {
        let mut rng = Rng::new(1234);

        let mut ranf = [0.0; Self::POINT_COUNT];
        ranf.iter_mut().for_each(|v| *v = rng.gen());

        let perm_x = Self::generate_perm(&mut rng);
        let perm_y = Self::generate_perm(&mut rng);
        let perm_z = Self::generate_perm(&mut rng);

        Self {
            ranf,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise(&self, p: Vec3) -> f64 {
        let i = (4.0 * p.x) as isize & 255;
        let j = (4.0 * p.y) as isize & 255;
        let k = (4.0 * p.z) as isize & 255;

        self.ranf[self.perm_x[i as usize] ^ self.perm_y[j as usize] ^ self.perm_z[k as usize]]
    }

    fn generate_perm(rng: &mut Rng) -> [usize; Self::POINT_COUNT] {
        let mut p = [0; Self::POINT_COUNT];
        p.iter_mut().enumerate().for_each(|(i, v)| *v = i);

        Self::permute(&mut p, Self::POINT_COUNT, rng);
        p
    }

    fn permute(p: &mut [usize], n: usize, rng: &mut Rng) {
        for i in (0..n).rev() {
            let target = rng.int(0, i as i64) as usize;
            let tmp = p[i];
            p[i] = p[target];
            p[target] = tmp;
        }
    }
}
