use super::{color::Color, ray::Ray, scatter::Scatter, vec3::Vec3};

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(a: Color) -> Lambertian {
        Lambertian { albedo: a }
    }
}

impl Scatter for Lambertian {
    fn scatter(
        &self,
        r_in: &super::ray::Ray,
        rec: &super::hit_record::HitRecord,
    ) -> Option<(Color, super::ray::Ray)> {
        let mut scatter_dir = &rec.normal + &Vec3::random_in_unit_sphere().unit_vector();
        if scatter_dir.near_zero() {
            scatter_dir = rec.normal;
        }
        let scattered = Ray::new(rec.p, scatter_dir);

        Some((self.albedo, scattered))
    }
}
