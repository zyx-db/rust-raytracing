use std::fmt;
use std::ops::{Add, Sub, Mul, AddAssign, MulAssign, DivAssign};

pub struct Color{
    pub values: [f64; 3],
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ir = (255.99 * self.values[0]) as i32;
        let ig = (255.99 * self.values[1]) as i32;
        let ib = (255.99 * self.values[2]) as i32;
        write!(f, "{} {} {}",ir, ig, ib)
    }
}

impl Color{
    pub fn new(r: f64, g: f64, b: f64) -> Self{
        Color { values: [r, g, b] }
    }
}

pub struct Vec3{
    pub values: [f64; 3],
}

impl Vec3 { 
    pub fn new(x: f64, y: f64, z: f64) -> Self{
        Vec3 { values: [x, y, z] }
    }

    pub fn x(&self) -> f64{
        self.values[0]
    }
    pub fn y(&self) -> f64{
        self.values[1]
    }
    pub fn z(&self) -> f64{
        self.values[2]
    }
}

impl Add for Vec3{
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3{
        Vec3 {values : 
            [self.x() + other.x(),
            self.y() + other.y(),
            self.z() + other.z()]
        }
    }
}

impl AddAssign for Vec3{
    fn add_assign(&mut self, other: Self) {
        *self = Vec3 {values : 
            [self.x() + other.x(),
            self.y() + other.y(),
            self.z() + other.z()]
        }
    }
}

impl Mul<f64> for Vec3{
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Vec3{
        Vec3 { values:
            [self.x() * rhs,
            self.y() * rhs,
            self.z() * rhs]
        }
    }
}
