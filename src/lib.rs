mod camera;
mod hittable;
pub mod materials;
mod ray;
mod sphere;
mod utils;
mod vec3;


pub use vec3::Vec3;
pub type Point3 = Vec3;
pub type Color = Vec3;

pub fn color(r: f64, g: f64, b: f64) -> Color {
    Color::new(r, g, b)
}
pub fn point3(x: f64, y: f64, z: f64) -> Point3 {
    Point3::new(x, y, z)
}

pub use camera::*;
pub use hittable::*;
pub use materials::*;
pub use ray::*;
pub use sphere::*;
pub use utils::*;
