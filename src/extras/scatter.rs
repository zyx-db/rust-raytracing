use super::color::Color;
use super::hit_record::HitRecord;
use super::ray::Ray;

pub trait Scatter : Send + Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
}
