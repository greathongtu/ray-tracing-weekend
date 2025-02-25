mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod ray;
mod rtweekend;
mod sphere;
mod vec3;

use camera::Camera;
use color::Color;
use hittable_list::HittableList;
use material::{Dielectric, Lambertian, Material, Metal};
use sphere::Sphere;
use std::rc::Rc;
use vec3::{Point3, Vec3};
use std::f64::consts::PI;

fn main() {
    let mut world = HittableList::default();

    let R = (PI / 4.0).cos();
    let material_left: Option<Rc<dyn Material>> = Some(Rc::new(Lambertian::new(&Color::new(0.0, 0.0, 1.0))));
    let material_right: Option<Rc<dyn Material>> = Some(Rc::new(Lambertian::new(&Color::new(1.0, 0.0, 0.0))));

    world.add(Rc::new(Sphere::new(
        &Point3::new(-R, 0.0, -1.0),
        R,
        material_left,
    )));
    world.add(Rc::new(Sphere::new(
        &Point3::new(R, 0.0, -1.0),
        R,
        material_right,
    )));

    let material_ground: Option<Rc<dyn Material>> =
        Some(Rc::new(Lambertian::new(&Color(Vec3::new(0.8, 0.8, 0.0)))));
    let material_center: Option<Rc<dyn Material>> =
        Some(Rc::new(Lambertian::new(&Color(Vec3::new(0.1, 0.2, 0.5)))));
    let material_left: Option<Rc<dyn Material>> = Some(Rc::new(Dielectric::new(1.50)));
    let material_bubble: Option<Rc<dyn Material>> = Some(Rc::new(Dielectric::new(1.00 / 1.50)));
    let material_right: Option<Rc<dyn Material>> =
        Some(Rc::new(Metal::new(&Color(Vec3::new(0.8, 0.6, 0.2)), 1.0)));

    world.add(Rc::new(Sphere::new(
        &Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Rc::new(Sphere::new(
        &Point3::new(0.0, 0.0, -1.2),
        0.5,
        material_center,
    )));
    world.add(Rc::new(Sphere::new(
        &Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.add(Rc::new(Sphere::new(
        &Point3::new(-1.0, 0.0, -1.0),
        0.4,
        material_bubble,
    )));
    world.add(Rc::new(Sphere::new(
        &Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    let mut camera = Camera::builder()
        .aspect_ration(16.0 / 9.0)
        .image_width(400)
        .samples_per_pixel(100)
        .max_depth(50)
        .vfov(90.0)
        .build();

    camera.render(&world);
}
