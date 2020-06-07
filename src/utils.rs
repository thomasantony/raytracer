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

pub fn rand_unit_vector() -> Vec3 {
    let mut rng = thread_rng();
    let a: f64 = rng.gen_range(0., 2. * std::f64::consts::PI);
    let z: f64 = rng.gen_range(-1., 1.);
    let r = (1. - z * z).sqrt();
    return Vec3::new(r * a.cos(), r * a.sin(), z);
}


pub fn rand_in_hemisphere(normal: &Vec3) -> Vec3 {
    let in_unit_sphere = rand_unit_vector();
    if in_unit_sphere.dot(normal) > 0.0
    { // In the same hemisphere as the normal
        return in_unit_sphere;
    }else{
        return -in_unit_sphere;
    }
}
