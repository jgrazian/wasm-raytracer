use crate::vec3::{Point3, Vec3};

#[derive(Debug, PartialEq, Default, Clone, Copy)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(orig: Point3, dir: Vec3) -> Ray {
        Ray {
            orig: orig,
            dir: dir,
        }
    }

    pub fn origin(&self) -> Point3 {
        self.orig
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    pub fn at(&self, t: f32) -> Point3 {
        self.orig + t * self.dir
    }
}
