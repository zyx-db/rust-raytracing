use super::vec3::Vec3;
use super::ray::Ray;

pub struct Camera {
    origin: Vec3,
    corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3
}

impl Camera {
    pub fn new() -> Camera {
        const ASPECT_RATIO: f64 = 16.0 / 9.0;
        const VIEWPORT_HEIGHT: f64 = 2.0;
        const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
        const FOCAL_LENGTH: f64 = 1.0;

        let orig = Vec3::new(0.0, 0.0, 0.0);
        let h = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
        let v = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
        let llc = orig - (h * 0.5) - (v * 0.5) - Vec3::new(0.0, 0.0, FOCAL_LENGTH);

        Camera { 
            origin: orig, 
            corner: llc, 
            horizontal: h, 
            vertical: v 
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        return Ray::new(
            self.origin,
            &(&self.corner + &(self.horizontal * u)) + &(self.vertical * v) - &self.origin         
        );
    }
}
