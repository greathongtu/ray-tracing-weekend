use crate::{
    color::{Color, write_color},
    hittable::Hittable,
    interval::Interval,
    ray::Ray,
    rtweekend::{degrees_to_radians, random_f64},
    vec3::{Point3, Vec3, random_in_unit_disk, unit_vector},
};

pub struct CameraBuilder {
    pub aspect_ratio: f64,
    pub image_width: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    pub vfov: f64,
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub vup: Vec3,
    pub defocus_angle: f64,
    pub focus_dist: f64,
}

impl Default for CameraBuilder {
    fn default() -> Self {
        Self {
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
            max_depth: 10,
            vfov: 90.0,
            lookfrom: Point3::new(0.0, 0.0, 0.0),
            lookat: Point3::new(0.0, 0.0, -1.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            defocus_angle: 0.0,
            focus_dist: 10.0,
        }
    }
}

impl CameraBuilder {
    pub fn aspect_ration(mut self, aspect_ratio: f64) -> CameraBuilder {
        self.aspect_ratio = aspect_ratio;
        self
    }

    pub fn image_width(mut self, image_width: u32) -> CameraBuilder {
        self.image_width = image_width;
        self
    }

    pub fn samples_per_pixel(mut self, samples_per_pixel: u32) -> CameraBuilder {
        self.samples_per_pixel = samples_per_pixel;
        self
    }

    pub fn max_depth(mut self, max_depth: u32) -> CameraBuilder {
        self.max_depth = max_depth;
        self
    }

    pub fn vfov(mut self, vfov: f64) -> CameraBuilder {
        self.vfov = vfov;
        self
    }

    pub fn lookfrom(mut self, lookfrom: &Point3) -> CameraBuilder {
        self.lookfrom = *lookfrom;
        self
    }

    pub fn lookat(mut self, lookat: &Point3) -> CameraBuilder {
        self.lookat = *lookat;
        self
    }

    pub fn vup(mut self, vup: &Vec3) -> CameraBuilder {
        self.vup = *vup;
        self
    }

    pub fn defocus_angle(mut self, defocus_angle: f64) -> CameraBuilder {
        self.defocus_angle = defocus_angle;
        self
    }

    pub fn focus_dist(mut self, focus_dist: f64) -> CameraBuilder {
        self.focus_dist = focus_dist;
        self
    }

    pub fn build(&self) -> Camera {
        let aspect_ratio = self.aspect_ratio;
        let image_width = self.image_width;
        let samples_per_pixel = self.samples_per_pixel;
        let max_depth = self.max_depth;
        let vfov = self.vfov;
        let lookfrom = self.lookfrom;
        let lookat = self.lookat;
        let vup = self.vup;
        let defocus_angle = self.defocus_angle;
        let focus_dist = self.focus_dist;

        // image
        let image_height = (image_width as f64 / aspect_ratio) as u32;
        let image_height = if image_height < 1 { 1 } else { image_height };

        let center = lookfrom;

        // Determine viewport dimensions
        // let focal_length = (lookfrom - lookat).length();
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focus_dist;
        let real_aspet_ratio = image_width as f64 / image_height as f64;
        let viewport_wideth = viewport_height * real_aspet_ratio;

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        let w = unit_vector(&(lookfrom - lookat));
        let u = unit_vector(&vup.cross(&w));
        let v = w.cross(&u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = viewport_wideth * u;
        let viewport_v = viewport_height * -v;

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // location of upper left pixel
        let viewport_upper_left = center - (focus_dist * w) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        // Calculate the camera defocus disk basis vectors.
        let defocus_radius = focus_dist * degrees_to_radians(defocus_angle / 2.0).tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Camera {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            max_depth,
            vfov,
            lookfrom,
            lookat,
            vup,
            defocus_angle,
            focus_dist,
            image_height,
            pixel_samples_scale: 1.0 / samples_per_pixel as f64,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            u,
            v,
            w,
            defocus_disk_u,
            defocus_disk_v,
        }
    }
}

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    pub vfov: f64,
    lookfrom: Point3,
    lookat: Point3,
    vup: Vec3,
    defocus_angle: f64,
    focus_dist: f64,
    image_height: u32,
    pixel_samples_scale: f64,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn builder() -> CameraBuilder {
        CameraBuilder::default()
    }

    pub fn render(&mut self, world: &impl Hittable) {
        println!("P3");
        println!("{} {}", self.image_width, self.image_height);
        println!("255");

        for j in 0..self.image_height {
            eprintln!("\rScanlines remaining: {} ", self.image_height - j);
            for i in 0..self.image_width {
                let mut pixel_color = Color::default();
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += self.ray_color(&r, self.max_depth, world);
                }
                write_color(&(self.pixel_samples_scale * pixel_color));
            }
        }
        eprintln!("\rDone.");
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = Self::sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x()) * self.pixel_delta_u)
            + ((j as f64 + offset.y()) * self.pixel_delta_v);

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_dir = pixel_sample - ray_origin;

        Ray::new(&ray_origin, &ray_dir)
    }

    fn sample_square() -> Vec3 {
        Vec3::new(random_f64() - 0.5, random_f64() - 0.5, 0.0)
    }

    fn defocus_disk_sample(&self) -> Point3 {
        let p = random_in_unit_disk();
        self.center + (p.x() * self.defocus_disk_u) + (p.y() * self.defocus_disk_v)
    }

    fn ray_color(&self, r: &Ray, depth: u32, world: &impl Hittable) -> Color {
        if depth <= 0 {
            return Color(Vec3::new(0.0, 0.0, 0.0));
        }

        if let Some(rec) = world.hit(r, Interval::new(0.001, std::f64::INFINITY)) {
            // let direction = rec.normal + random_unit_vector();
            // return 0.5 * self.ray_color(&Ray::new(&rec.p, &direction), depth - 1, world);
            if let Some(ref mat) = rec.mat {
                if let Some((attenuation, scattered)) = mat.scatter(r, &rec) {
                    return attenuation * self.ray_color(&scattered, depth - 1, world);
                }
            }
            return Color(Vec3::new(0.0, 0.0, 0.0));
        }

        let unit_direction = unit_vector(r.direction());
        let a = 0.5 * (unit_direction.y() + 1.0) as f64;

        // background: blend white and blue
        Color((1.0 - a) * Vec3::new(1.0, 1.0, 1.0) + a * Vec3::new(0.5, 0.7, 1.0))
    }
}
