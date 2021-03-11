use crate::vec3::Vec3;

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
