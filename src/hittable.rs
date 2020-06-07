use crate::{Point3, Ray, Vec3, Material, Color};
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub distance: f64,
    pub is_front_face: bool,
    pub material: Option<Rc<dyn Material>>,
}

impl HitRecord {
    pub fn new(outward_normal: Vec3, ray: &Ray, distance: f64, point: Point3) -> Self {
        let is_front_face = ray.direction.dot(outward_normal) < 0.;
        let normal = if is_front_face {
            outward_normal
        } else {
            -outward_normal
        };
        Self {
            point,
            distance,
            normal,
            is_front_face,
            material: None,
        }
    }
    pub fn new_with_material(outward_normal: Vec3, 
                             ray: &Ray, 
                             distance: f64, 
                             point: Point3, 
                             material: Rc<dyn Material>) -> Self
    {
        let mut rec = Self::new(outward_normal, ray, distance, point);
        rec.material = Some(material);
        rec
    }

    pub fn scatter(&self, r: &Ray) -> Option<(Ray, Color)>
    {
        self.material.as_ref().map(|m|m.scatter(r, self)).flatten()
    }
}
pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct HittableList(Vec<Box<dyn Hittable>>);
impl HittableList {
    pub fn new() -> Self {
        Self(Vec::new())
    }
    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.0.push(object);
    }
}
impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut output: Option<HitRecord> = None;
        for item in self.0.iter() {
            if let Some(rec) = item.hit(r, t_min, closest_so_far) {
                closest_so_far = rec.distance;
                output = Some(rec);
            }
        }
        output
    }
}

// Allow use of HittableList like a vector
impl Deref for HittableList {
    type Target = Vec<Box<dyn Hittable>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for HittableList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
