use crate::{Ray, HitRecord, Color, random_unit_vector, reflect, clamp};
pub trait Material {
    /**
     * Returns a ray if there is a scattered ray
     */
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)>;
}

pub struct Lambertian {
    albedo: Color
}
impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self{ albedo }
    }
}
impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)>
    {
        let scatter_direction = rec.normal + random_unit_vector();
        let scattered = Ray::new(rec.point, scatter_direction);
        Some((scattered, self.albedo))
    }
}
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}
impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        let fuzz = clamp(fuzz, 0., 1.);
        Self {
            albedo,
            fuzz,
        }
    }
}
impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)>
    {
        let reflected = reflect(r_in.direction.unit(), rec.normal);
        let scattered = Ray::new(rec.point, reflected + self.fuzz*random_unit_vector());
        let is_scattered = scattered.direction.dot(rec.normal) > 0.;
        if is_scattered
        {
            Some((scattered, self.albedo))
        }else{
            None
        }
    }
}