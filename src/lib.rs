mod vec3;
mod ray;
mod sphere;
mod hittable;
mod camera;

pub use vec3::Vec3;
pub type Point3 = Vec3;
pub type Color = Vec3;


pub fn color(r: f64, g: f64, b: f64) -> Color {
    Color::new(r, g, b)
}
pub fn point3(x: f64, y: f64, z: f64) -> Point3 {
    Point3::new(x, y, z)
}

pub use ray::*;
pub use sphere::*;
pub use hittable::*;
pub use camera::*;

pub fn clamp(n: f64, min: f64, max: f64) -> f64 {
    if n > max {
        max
    }else if n < min {
        min
    }else{
        n
    }
}