use super::hit::Hit;
use super::hit_record::HitRecord;
use super::ray::Ray;
use super::scatter::Scatter;
use super::vec3::Vec3;
use std::sync::Arc;

#[derive(Clone)]
pub struct Sphere {
    center: Vec3,
    radius: f64,
    mat: Arc<dyn Scatter>,
}

impl Sphere {
    pub fn new(c: Vec3, r: f64, m: Arc<dyn Scatter>) -> Sphere {
        Sphere {
            center: c,
            radius: r,
            mat: m,
        }
    }
}

impl Hit for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // Do we get hit?
        let oc = r.origin() - self.center;
        let a = r.direction().length().powi(2);
        let half_b = oc.dot(r.direction());
        let c = oc.length().powi(2) - self.radius * self.radius;

        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 {
            return None;
        }

        // Find the nearest root in the range
        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let point = r.at(root);
        let mut rec = HitRecord {
            t: root,
            p: point,
            mat: self.mat.clone(),
            normal: Vec3::new(0.0, 0.0, 0.0),
            front_face: false,
        };

        let outward_normal = (point - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);

        Some(rec)
    }
}
