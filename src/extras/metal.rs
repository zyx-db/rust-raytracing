use super::{color::Color, ray::Ray, scatter::Scatter, vec3::Vec3};

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(a: Color, f: f64) -> Metal {
        Metal { albedo: a , fuzz: f}
    }
}

impl Scatter for Metal {
    fn scatter(&self, r_in: &Ray, rec: &super::hit_record::HitRecord) -> Option<(Color, Ray)> {
        let reflected = r_in.direction().reflect(rec.normal).unit_vector();
        let scattered = Ray::new(rec.p, &(Vec3::random_in_unit_sphere() * self.fuzz) + &reflected);
        if scattered.direction().dot(rec.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}
