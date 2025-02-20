use crate::vec3::Vec3;

pub struct Color(pub Vec3);

pub fn write_color(pixel_color: &Color) {
    let r = pixel_color.0.x();
    let g = pixel_color.0.y();
    let b = pixel_color.0.z();

    let rbyte = (255.999 * r) as u8;
    let gbyte = (255.999 * g) as u8;
    let bbyte = (255.999 * b) as u8;

    println!("{} {} {}", rbyte, gbyte, bbyte);
}
