use super::hit_record::HitRecord;
use super::ray::Ray;

pub trait Hit {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
