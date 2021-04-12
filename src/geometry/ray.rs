use super::Vec3;

#[derive(Debug, PartialEq, Default, Clone, Copy)]
pub struct Ray {
    pub o: Vec3,
    pub d: Vec3,
}

impl Ray {
    #[inline(always)]
    pub fn at(&self, t: f64) -> Vec3 {
        self.o + t * self.d
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at() {
        let r = Ray {
            o: Vec3::splat(0.0),
            d: Vec3::splat(1.0),
        };
        assert_eq!(r.at(2.0), Vec3::splat(2.0))
    }
}
