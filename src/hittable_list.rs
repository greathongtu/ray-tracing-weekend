use std::rc::Rc;

use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
    ray::Ray,
};

#[derive(Default)]
pub struct HittableList<T>
where
    T: Hittable,
{
    objects: Vec<Rc<T>>,
}

impl<T> HittableList<T>
where
    T: Hittable,
{
    pub fn new(mut self, object: Rc<T>) -> Self {
        self.add(object);
        self
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Rc<T>) {
        self.objects.push(object);
    }
}

impl<T> Hittable for HittableList<T>
where
    T: Hittable,
{
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
            if let Some(temp_rec) = object.hit(r, Interval::new(ray_t.min, closest_so_far)) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                rec = temp_rec;
                // rec.t = temp_rec.t;
                // rec.p = temp_rec.p;
                // rec.normal = temp_rec.normal;
                // rec.front_face = temp_rec.front_face;
            }
        }
        if hit_anything {
            return Some(rec);
        }
        None
    }
}
