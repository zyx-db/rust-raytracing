use std::ops::AddAssign;

#[derive(Copy, Clone)]
pub struct Color {
    values: [f64; 3],
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color { values: [r, g, b] }
    }

    pub fn format_color(self, samples_per_pixel: u64) -> String {
        let ir = (256.0 * (self.values[0] / (samples_per_pixel as f64)).clamp(0.0, 0.999)) as u64;
        let ig = (256.0 * (self.values[1] / (samples_per_pixel as f64)).clamp(0.0, 0.999)) as u64;
        let ib = (256.0 * (self.values[2] / (samples_per_pixel as f64)).clamp(0.0, 0.999)) as u64;

        format!("{} {} {}", ir, ig, ib)
    }
}

impl AddAssign for Color{
    fn add_assign(&mut self, other: Self) {
        *self = Color {
            values: [
                self.values[0] + other.values[0],
                self.values[1] + other.values[1],
                self.values[2] + other.values[2]
            ],
        }
    }
}
