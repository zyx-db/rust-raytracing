use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub};

use rand::Rng;

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

    pub fn length_squared(&self) -> f64 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }

    pub fn random(lower: f64, upper: f64)-> Vec3 {
        let mut rng = rand::thread_rng();
        Vec3 {
            values: [rng.gen_range(lower..upper), rng.gen_range(lower..upper), rng.gen_range(lower..upper)]
        }
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let v = Vec3::random(-1.0, 1.0);
            if v.length() < 1.0 {
                return v;
            }
        }
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
