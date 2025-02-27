use crate::{interval::Interval, vec3::Vec3};

#[derive(Clone, Copy)]
pub struct Color(pub Vec3);

impl Default for Color {
    fn default() -> Self {
        Color(Vec3::default())
    }
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color(Vec3::new(r, g, b))
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

impl std::ops::Mul<Color> for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color(Vec3 {
            e: [
                self.0.e[0] * rhs.0.e[0],
                self.0.e[1] * rhs.0.e[1],
                self.0.e[2] * rhs.0.e[2],
            ],
        })
    }
}

impl From<Vec3> for Color {
    fn from(value: Vec3) -> Self {
        Self(value)
    }
}

#[inline]
pub fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        return linear_component.sqrt();
    }
    0.0
}

pub fn write_color(pixel_color: &Color) {
    let r = pixel_color.0.x();
    let g = pixel_color.0.y();
    let b = pixel_color.0.z();

    let r = linear_to_gamma(r);
    let g = linear_to_gamma(g);
    let b = linear_to_gamma(b);

    const INTENSITY: Interval = Interval::new(0.0, 0.999);
    let rbyte = (255.999 * INTENSITY.clamp(r)) as u8;
    let gbyte = (255.999 * INTENSITY.clamp(g)) as u8;
    let bbyte = (255.999 * INTENSITY.clamp(b)) as u8;

    println!("{} {} {}", rbyte, gbyte, bbyte);
}
