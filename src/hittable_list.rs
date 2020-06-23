use crate::hittable::{Hit, HitRecord, Hittable};
use crate::ray::Ray;

#[derive(Debug, Default, Clone)]
pub struct HittableList {
    objects: Vec<Hittable>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList { objects: vec![] }
    }

    pub fn add(&mut self, object: Hittable) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut tmp_rec: HitRecord = Default::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for obj in self.objects.iter() {
            if obj.hit(r, t_min, closest_so_far, &mut tmp_rec) {
                hit_anything = true;
                closest_so_far = tmp_rec.t;
                *rec = tmp_rec;
            }
        }
        hit_anything
    }
}
