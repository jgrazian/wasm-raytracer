use std::sync::Arc;

use crate::geometry::{Ray, Vec3};
use crate::material::Material;

#[derive(Clone, Copy)]
pub enum HitRec<'mat> {
    Hit(Rec, Option<&'mat Arc<dyn Material>>),
    Miss,
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Rec {
    pub p: Vec3,
    pub n: Vec3,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}

impl<'mat> HitRec<'mat> {
    pub fn hit(
        p: Vec3,
        t: f64,
        u: f64,
        v: f64,
        r: Ray,
        outward_normal: Vec3,
        mat: Option<&'mat Arc<dyn Material>>,
    ) -> Self {
        // Determine if inside or outside shape. Needed for glass
        let front_face = Vec3::dot(r.d, outward_normal) < 0.0;
        let n = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        Self::Hit(
            Rec {
                p,
                n,
                t,
                u,
                v,
                front_face,
            },
            mat,
        )
    }
}
