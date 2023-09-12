use std::fmt;

#[derive(Copy, Clone)]
pub struct Color {
    values: [f64; 3],
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ir = (255.99 * self.values[0]) as i32;
        let ig = (255.99 * self.values[1]) as i32;
        let ib = (255.99 * self.values[2]) as i32;
        write!(f, "{} {} {}", ir, ig, ib)
    }
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color { values: [r, g, b] }
    }
}
