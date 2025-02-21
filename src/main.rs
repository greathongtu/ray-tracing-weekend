mod color;
mod hittable;
mod ray;
mod sphere;
mod vec3;

use color::{Color, write_color};
use ray::Ray;
use vec3::{Point3, Vec3, dot, unit_vector};

fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> f64 {
    let oc = *center - *r.origin();
    let a = r.direction().length_squared();
    let h = dot(r.direction(), &oc);
    let c = oc.length_squared() - radius * radius;
    let discriminant = h * h - a * c;
    if discriminant < 0 as f64 {
        return -1.0;
    }
    return (h - discriminant.sqrt()) / a;
}

pub fn ray_color(r: Ray) -> Color {
    let t = hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, &r);
    if t > 0.0 {
        let N = unit_vector(&(r.at(t) - Vec3::new(0.0, 0.0, -1.0)));
        return Color(0.5 * Vec3::new(N.x() + 1.0, N.y() + 1.0, N.z() + 1.0));
    }
    let unit_direction = unit_vector(r.direction());
    let a = 0.5 * (unit_direction.y() + 1.0) as f64;

    // blend white and blue
    Color((1.0 - a) * Vec3::new(1.0, 1.0, 1.0) + a * Vec3::new(0.5, 0.7, 1.0))
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    let image_height: usize = (image_width as f64 / aspect_ratio) as usize;
    let image_height = if image_height < 1 { 1 } else { image_height };

    let real_aspet_ratio = image_width as f64 / image_height as f64;

    // Camera

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_wideth = viewport_height * real_aspet_ratio;
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    // vectors

    let viewport_u = Vec3::new(viewport_wideth, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    // location of upper left pixel

    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // Render

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for j in 0..image_height {
        eprintln!("\rScanlines remaining: {} ", image_height - j);
        for i in 0..image_width {
            let pixel_center = pixel00_loc + i as f64 * pixel_delta_u + j as f64 * pixel_delta_v;
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(&camera_center, &ray_direction);
            let pixel_color = ray_color(r);
            write_color(&pixel_color);
        }
    }
    eprintln!("\rDone.");
}
