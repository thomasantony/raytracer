use std::sync::Arc;
use crate::{HitRecord, Hittable, Point3, Ray};
use crate::Material;
pub struct SimpleSphere {
    center: Point3,
    radius: f64,
}
impl SimpleSphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Self { center, radius }
    }
}

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Arc<dyn Material + Sync + Send>,
}
impl Sphere
{
    pub fn new(center: Point3, radius: f64, material: Arc<dyn Material + Sync + Send>) -> Self {
        Self { center, radius, material: material }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = oc.dot(r.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        // Has valid intersection
        if discriminant > 0. {
            let d_root = discriminant.sqrt();
            let temp = (-half_b - d_root) / a;
            if temp < t_max && temp > t_min {
                // Ray hitting outside sphere
                let point = r.at(temp);
                let outward_normal = (point - self.center) / self.radius;
                let hr = HitRecord::new_with_material(outward_normal, &r, temp, point, self.material.clone());
                return Some(hr);
            }
            let temp = (-half_b + d_root) / a;
            if temp < t_max && temp > t_min {
                // Ray hitting inside sphere
                let point = r.at(temp);
                let outward_normal = (point - self.center) / self.radius;
                let hr = HitRecord::new_with_material(outward_normal, &r, temp, point, self.material.clone());
                return Some(hr);
            }
        }
        return None;
    }
}


impl Hittable for SimpleSphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = oc.dot(r.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        // Has valid intersection
        if discriminant > 0. {
            let d_root = discriminant.sqrt();
            let temp = (-half_b - d_root) / a;
            if temp < t_max && temp > t_min {
                // Ray hitting outside sphere
                let point = r.at(temp);
                let outward_normal = (point - self.center) / self.radius;
                let hr = HitRecord::new(outward_normal, &r, temp, point);
                return Some(hr);
            }
            let temp = (-half_b + d_root) / a;
            if temp < t_max && temp > t_min {
                // Ray hitting inside sphere
                let point = r.at(temp);
                let outward_normal = (point - self.center) / self.radius;
                let hr = HitRecord::new(outward_normal, &r, temp, point);
                return Some(hr);
            }
        }
        return None;
    }
}
