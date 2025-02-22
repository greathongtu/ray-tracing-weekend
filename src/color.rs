use crate::{interval::Interval, vec3::Vec3};

pub struct Color(pub Vec3);

impl Default for Color {
    fn default() -> Self {
        Color(Vec3::default())
    }
}

impl std::ops::AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.0.e[0] += rhs.0.e[0];
        self.0.e[1] += rhs.0.e[1];
        self.0.e[2] += rhs.0.e[2];
    }
}

impl std::ops::Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self(Vec3 {
            e: [self.0.e[0] * rhs, self.0.e[1] * rhs, self.0.e[2] * rhs],
        })
    }
}

impl std::ops::Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color(Vec3 {
            e: [self * rhs.0.e[0], self * rhs.0.e[1], self * rhs.0.e[2]],
        })
    }
}

impl From<Vec3> for Color {
    fn from(value: Vec3) -> Self {
        Self(value)
    }
}

pub fn write_color(pixel_color: &Color) {
    let r = pixel_color.0.x();
    let g = pixel_color.0.y();
    let b = pixel_color.0.z();

    const INTENSITY: Interval = Interval::new(0.0, 0.999);
    let rbyte = (255.999 * INTENSITY.clamp(r)) as u8;
    let gbyte = (255.999 * INTENSITY.clamp(g)) as u8;
    let bbyte = (255.999 * INTENSITY.clamp(b)) as u8;

    println!("{} {} {}", rbyte, gbyte, bbyte);
}
