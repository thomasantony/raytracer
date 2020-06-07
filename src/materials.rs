use crate::{Ray, HitRecord, Color, random_unit_vector, reflect};
pub trait Material {
    /**
     * Returns a ray if there is a scattered ray
     */
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)>;
}

pub struct Lambertian {
    pub albedo: Color
}
impl Material for Lambertian {
    fn scatter(&self, r_in_: &Ray, rec: &HitRecord) -> Option<(Ray, Color)>
    {
        let scatter_direction = rec.normal + random_unit_vector();
        let scattered = Ray::new(rec.point, scatter_direction);
        Some((scattered, self.albedo))
    }
}
pub struct Metal {
    pub albedo: Color
}
impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)>
    {
        let reflected = reflect(r_in.direction.unit(), rec.normal);
        let scattered = Ray::new(rec.point, reflected);
        let is_scattered = scattered.direction.dot(rec.normal) > 0.;
        if is_scattered
        {
            Some((scattered, self.albedo))
        }else{
            None
        }
    }
}