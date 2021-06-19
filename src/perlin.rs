use crate::geometry::Vec3;
use crate::rng::Rng;

#[derive(Debug)]
pub struct Perlin {
    ranvec: [Vec3; Self::POINT_COUNT],
    perm_x: [usize; Self::POINT_COUNT],
    perm_y: [usize; Self::POINT_COUNT],
    perm_z: [usize; Self::POINT_COUNT],
}

impl Perlin {
    const POINT_COUNT: usize = 256;

    pub fn new() -> Self {
        let mut rng = Rng::new(1234);

        let mut ranvec = [Vec3::zero(); Self::POINT_COUNT];
        ranvec
            .iter_mut()
            .for_each(|v| *v = Vec3::random_range(&mut rng, -1.0, 1.0).unit());

        let perm_x = Self::generate_perm(&mut rng);
        let perm_y = Self::generate_perm(&mut rng);
        let perm_z = Self::generate_perm(&mut rng);

        Self {
            ranvec,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise(&self, p: Vec3) -> f64 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();

        let i = p.x.floor() as isize;
        let j = p.y.floor() as isize;
        let k = p.z.floor() as isize;

        let mut c = [[[Vec3::zero(); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.ranvec[(self.perm_x[((i + di as isize) & 255) as usize]
                        ^ self.perm_y[((j + dj as isize) & 255) as usize]
                        ^ self.perm_z[((k + dk as isize) & 255) as usize])
                        as usize];
                }
            }
        }

        Self::perlin_interp(&c, u, v, w)
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

    fn perlin_interp(c: &[[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);

        let mut accum = 0.0;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = Vec3 {
                        x: u - i as f64,
                        y: v - j as f64,
                        z: w - k as f64,
                    };
                    accum += (i as f64 * uu + (1 - i) as f64 * (1.0 - uu))
                        * (j as f64 * vv + (1 - j) as f64 * (1.0 - vv))
                        * (k as f64 * ww + (1 - k) as f64 * (1.0 - ww))
                        * Vec3::dot(c[i][j][k], weight_v);
                }
            }
        }

        accum
    }

    pub fn turb(&self, p: Vec3, depth: usize) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = p;
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }

        accum.abs()
    }
}
