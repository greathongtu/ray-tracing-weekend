use std::rc::Rc;

use crate::hittable::{HitRecord, Hittable};

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
    fn hit(&self, r: &crate::ray::Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = ray_tmax;

        for object in &self.objects {
            if object.hit(r, ray_tmin, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                // *rec = temp_rec.clone();
                rec.t = temp_rec.t;
                rec.p = temp_rec.p;
                rec.normal = temp_rec.normal;
                rec.front_face = temp_rec.front_face;
            }
        }

        return hit_anything;
    }
}
