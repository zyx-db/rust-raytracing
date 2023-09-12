use super::hit::Hit;
use super::hit_record::HitRecord;
use super::ray::Ray;

pub type World = Vec<Box<dyn Hit>>;

impl Hit for World {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut tmp_rec = None;
        let mut current_closest = t_max;

        for object in self {
            if let Some(rec) = object.hit(r, t_min, current_closest) {
                current_closest = rec.t;
                tmp_rec = Some(rec);
            }
        }

        tmp_rec
    }
}
