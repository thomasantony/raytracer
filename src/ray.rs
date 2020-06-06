use crate::{Point3, Vec3};
#[derive(Debug, Default)]
pub struct Ray {
    origin: Point3,
    dir: Vec3,
}
impl Ray 
{
    pub fn new(origin: &Point3, dir: &Vec3) -> Self {
        Self {
            origin: origin.clone(),
            dir: dir.clone(),
        }
    }
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.dir * t
    }
}

#[cfg(test)]
mod tests 
{
    use super::*;
    #[test]
    fn test_at()
    {
        let origin = Point3::new(0., 0., 0.);
        let dir = Vec3::new(1., 1., 1.);
        let ray = Ray::new(&origin, &dir);
        assert_eq!(ray.at(5.), Point3::new(5., 5., 5.));
    }
}