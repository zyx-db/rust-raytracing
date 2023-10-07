use super::ray::Ray;
use super::vec3::Vec3;

pub struct Camera {
    origin: Vec3,
    corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    cu: Vec3,
    cv: Vec3,
    lens_radius: f64
}

impl Camera {
    pub fn new(lookfrom: Vec3,
               lookat: Vec3,
               vup: Vec3,
               vfov: f64,
               aspect_ratio: f64,
               aperture: f64,
               focus_dist: f64) -> Camera {
        let theta = std::f64::consts::PI / 180.0 * vfov;
        let viewport_height: f64 = 2.0 * (theta / 2.0).tan();
        let viewport_width: f64 = aspect_ratio * viewport_height;

        let cw = (lookfrom - lookat).unit_vector();
        let cu = vup.cross(cw).unit_vector();
        let cv = cw.cross(cu);

        let h = cu * viewport_width * focus_dist;
        let v = cv * viewport_height * focus_dist;

        let llc = lookfrom - h / 2.0 - v / 2.0 - cw * focus_dist;

        Camera {
            origin: lookfrom,
            corner: llc,
            horizontal: h,
            vertical: v,
            cu,
            cv,
            lens_radius: aperture / 2.0
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let rd = Vec3::random_in_unit_sphere() * self.lens_radius;
        let offset = self.cu * rd.x() + self.cv * rd.y();

        return Ray::new(
            self.origin + offset,
            (self.corner + (self.horizontal * u)) + (self.vertical * v) - self.origin,
        );
    }
}
