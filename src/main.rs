mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod ray;
mod rtweekend;
mod sphere;
mod vec3;

use camera::CameraConfig;
use hittable_list::HittableList;
use sphere::Sphere;
use std::rc::Rc;
use vec3::Point3;

fn main() {
    let mut world = HittableList::default();
    world.add(Rc::new(Sphere::new(&Point3::new(0.0, -100.5, -1.0), 100.0)));
    world.add(Rc::new(Sphere::new(&Point3::new(0.0, 0.0, -1.0), 0.5)));

    let camera_config = CameraConfig {
        aspect_ratio: 16.0 / 9.0,
        image_width: 400,
        samples_per_pixel: 100,
    };
    camera_config.get_camera().render(&world);
}
