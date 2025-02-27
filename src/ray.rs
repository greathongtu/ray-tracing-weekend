use crate::vec3::{Point3, Vec3};

pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(orig: &Point3, dir: &Vec3) -> Self {
        Self {
            orig: orig.clone(),
            dir: dir.clone(),
        }
    }

    pub fn origin(&self) -> &Point3 {
        &self.orig
    }

    pub fn direction(&self) -> &Vec3 {
        &self.dir
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig.clone() + t * self.dir.clone()
    }
}
