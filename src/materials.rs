use crate::{Ray, HitRecord, Color, random_unit_vector};
use crate::utils::{reflect, refract, schlick, clamp, fmin};
use rand::prelude::*;

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

pub struct Dielectric {
    ref_idx: f64,
}
impl Dielectric {
    pub fn new(ref_idx: f64) -> Self {
        Self {
            ref_idx
        }
    }
}
impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)>
    {
        let attenuation = Color::new(1.0, 1.0, 1.0);

        let etai_over_etat = if rec.is_front_face {
            1.0 / self.ref_idx
        }else {
            self.ref_idx
        };
        let ray_unit = r_in.direction.unit();
        let cos_theta = fmin(-ray_unit.dot(rec.normal), 1.0);
        let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();
        // Total Internal Reflection
        if etai_over_etat * sin_theta > 1.0 
        {
            let reflected = reflect(ray_unit, rec.normal);
            let scattered = Ray::new(rec.point, reflected);
            return Some((scattered, attenuation))
        }
        // Glancing Reflection
        let reflect_prob = schlick(cos_theta, etai_over_etat);
        let mut rng = thread_rng();
        if rng.gen::<f64>() < reflect_prob
        {
            let reflected = reflect(ray_unit, rec.normal);
            let scattered = Ray::new(rec.point, reflected);
            return Some((scattered, attenuation))
        }
        // Refraction
        let refracted = refract(ray_unit, rec.normal, etai_over_etat);
        let scattered = Ray::new(rec.point, refracted);
        Some((scattered, attenuation))
    }
}