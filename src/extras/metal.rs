use super::{color::Color, ray::Ray, scatter::Scatter};

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(a: Color) -> Metal {
        Metal { albedo: a }
    }
}

impl Scatter for Metal {
    fn scatter(&self, r_in: &Ray, rec: &super::hit_record::HitRecord) -> Option<(Color, Ray)> {
        let reflected = r_in.direction().reflect(rec.normal).unit_vector();
        let scattered = Ray::new(rec.p, reflected);
        if scattered.direction().dot(rec.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}
