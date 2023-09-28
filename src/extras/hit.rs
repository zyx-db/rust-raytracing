use super::hit_record::HitRecord;
use super::ray::Ray;

pub trait Hit : Send + Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
