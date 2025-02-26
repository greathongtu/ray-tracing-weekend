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
use rtweekend::{random_f64, random_f64_within};
use sphere::Sphere;
use std::rc::Rc;
use vec3::{Point3, Vec3};

fn main() {
    let mut world = HittableList::default();

    let material_ground: Option<Rc<dyn Material>> =
        Some(Rc::new(Lambertian::new(&Color(Vec3::new(0.5, 0.5, 0.5)))));
    world.add(Rc::new(Sphere::new(
        &Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        material_ground,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_f64();
            let center = Point3::new(
                a as f64 + 0.9 * random_f64(),
                0.2,
                b as f64 + 0.9 * random_f64(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Option<Rc<dyn Material>> = {
                    if choose_mat < 0.8 {
                        let albedo = Color(Vec3::random()) * Color(Vec3::random());
                        Some(Rc::new(Lambertian::new(&albedo)))
                    } else if choose_mat < 0.95 {
                        let albedo = Color(Vec3::random_within(0.5, 1.0));
                        let fuzz = random_f64_within(0.0, 0.5);
                        Some(Rc::new(Metal::new(&albedo, fuzz)))
                    } else {
                        Some(Rc::new(Dielectric::new(1.5)))
                    }
                };

                world.add(Rc::new(Sphere::new(&center, 0.2, sphere_material)));
            }
        }
    }

    let material1: Option<Rc<dyn Material>> = Some(Rc::new(Dielectric::new(1.50)));
    world.add(Rc::new(Sphere::new(
        &Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2: Option<Rc<dyn Material>> =
        Some(Rc::new(Lambertian::new(&Color(Vec3::new(0.4, 0.2, 0.1)))));
    world.add(Rc::new(Sphere::new(
        &Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3: Option<Rc<dyn Material>> =
        Some(Rc::new(Metal::new(&Color(Vec3::new(0.7, 0.6, 0.5)), 0.0)));
    world.add(Rc::new(Sphere::new(
        &Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    let mut camera = Camera::builder()
        .aspect_ration(16.0 / 9.0)
        .image_width(1200)
        .samples_per_pixel(500)
        .max_depth(50)
        .vfov(20.0)
        .lookfrom(&Point3::new(13.0, 2.0, 3.0))
        .lookat(&Point3::new(0.0, 0.0, 0.0))
        .vup(&Vec3::new(0.0, 1.0, 0.0))
        .defocus_angle(0.6)
        .focus_dist(10.0)
        .build();

    camera.render(&world);
}
