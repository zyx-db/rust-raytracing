use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub};

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

#[derive(Copy, Clone)]
pub struct Vec3 {
    pub values: [f64; 3],
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { values: [x, y, z] }
    }

    pub fn x(&self) -> f64 {
        self.values[0]
    }
    pub fn y(&self) -> f64 {
        self.values[1]
    }
    pub fn z(&self) -> f64 {
        self.values[2]
    }

    pub fn negative(&self) -> Vec3 {
        Vec3 {
            values: [-self.x(), -self.y(), -self.z()],
        }
    }

    pub fn unit_vector(&self) -> Vec3 {
        self / self.length()
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(&self, other: Vec3) -> f64 {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }

    fn length_squared(&self) -> f64 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }
}

impl Add for &Vec3 {
    type Output = Vec3;

    fn add(self, other: &Vec3) -> Vec3 {
        Vec3 {
            values: [
                self.x() + other.x(),
                self.y() + other.y(),
                self.z() + other.z(),
            ],
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Vec3 {
            values: [
                self.x() + other.x(),
                self.y() + other.y(),
                self.z() + other.z(),
            ],
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f64) -> Vec3 {
        Vec3 {
            values: [self.x() * other, self.y() * other, self.z() * other],
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        *self = Vec3 {
            values: [self.x() * other, self.y() * other, self.z() * other],
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            values: [
                self.x() - other.x(),
                self.y() - other.y(),
                self.z() - other.z(),
            ],
        }
    }
}

impl Sub<&Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: &Vec3) -> Self::Output {
        Vec3 {
            values: [
                self.x() - other.x(),
                self.y() - other.y(),
                self.z() - other.z(),
            ],
        }
    }
}

impl Div<f64> for &Vec3 {
    type Output = Vec3;

    fn div(self, other: f64) -> Self::Output {
        Vec3 {
            values: [self.x() / other, self.y() / other, self.z() / other],
        }
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, other: f64) {
        *self = Vec3 {
            values: [self.x() / other, self.y() / other, self.z() / other],
        }
    }
}

#[derive(Copy, Clone)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn new(o: Vec3, dir: Vec3) -> Ray {
        Ray {
            origin: o,
            direction: dir,
        }
    }
    pub fn origin(self) -> Vec3 {
        self.origin
    }
    pub fn direction(self) -> Vec3 {
        self.direction
    }
    pub fn at(self, t: f64) -> Vec3 {
        return &self.origin + &(self.direction * t);
    }
}
