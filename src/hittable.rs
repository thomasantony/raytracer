use crate::{Ray, Point3, Vec3};
use std::ops::{Deref, DerefMut};

pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub distance: f64,
    pub is_front_face: bool
}

impl HitRecord {
    pub fn new(outward_normal: Vec3, ray: &Ray, distance: f64, point: Point3) -> Self
    {
        let is_front_face = ray.direction.dot(&outward_normal) < 0.;
        let normal = if is_front_face { outward_normal } else {-outward_normal};
        Self {
            point,
            distance,
            normal,
            is_front_face,
        }
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
    pub fn add<T: Hittable + 'static>(&mut self, object: T) {
        self.0.push(Box::new(object));
    }
}
impl Hittable for HittableList
{
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>
    {
        let mut closest_so_far = t_max;
        let mut output: Option<HitRecord> = None;
        for item in self.0.iter()
        {
            if let Some(rec) = item.hit(r, t_min, closest_so_far)
            {
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

