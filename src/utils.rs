use crate::Vec3;
use rand::prelude::*;

pub fn clamp(n: f64, min: f64, max: f64) -> f64 {
    if n > max {
        max
    } else if n < min {
        min
    } else {
        n
    }
}

pub fn fmin(a: f64, b: f64) -> f64 {
    if a < b 
    {
        a
    } else {
        b
    }
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    return v - 2.0 * n * v.dot(n);
}

pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3
{
    let cos_theta = -uv.dot(n);
    let r_out_parallel =  etai_over_etat * (uv + cos_theta*n);
    let r_out_perp = -((1.0 - r_out_parallel.length_squared()).sqrt()) * n;
    return r_out_parallel + r_out_perp;
}
pub fn schlick(cosine: f64, ref_idx: f64) -> f64
{
    let r0 = (1.-ref_idx) / (1.+ref_idx);
    let r0 = r0*r0;
    return r0 + (1.-r0)*((1. - cosine).powf(5.0));
}

pub fn random_unit_vector() -> Vec3 {
    let mut rng = thread_rng();
    let a: f64 = rng.gen_range(0., 2. * std::f64::consts::PI);
    let z: f64 = rng.gen_range(-1., 1.);
    let r = (1. - z * z).sqrt();
    return Vec3::new(r * a.cos(), r * a.sin(), z);
}


pub fn rand_in_hemisphere(normal: &Vec3) -> Vec3 {
    let in_unit_sphere = random_unit_vector();
    if in_unit_sphere.dot(*normal) > 0.0
    { // In the same hemisphere as the normal
        return in_unit_sphere;
    }else{
        return -in_unit_sphere;
    }
}
