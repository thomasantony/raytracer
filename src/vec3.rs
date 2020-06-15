use crate::clamp;
use std::ops::*;
use rand::prelude::*;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub e: [f64; 3],
}

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Self { e: [e0, e1, e2] }
    }
    pub fn x(&self) -> f64 {
        self.e[0]
    }
    pub fn y(&self) -> f64 {
        self.e[1]
    }
    pub fn z(&self) -> f64 {
        self.e[2]
    }
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
    pub fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }
    pub fn dot(&self, rhs: Vec3) -> f64 {
        self.e[0] * rhs.e[0] + self.e[1] * rhs.e[1] + self.e[2] * rhs.e[2]
    }
    pub fn cross(&self, rhs: Vec3) -> Vec3 {
        Vec3::new(
            self.e[1] * rhs.e[2] - self.e[2] * rhs.e[1],
            self.e[2] * rhs.e[0] - self.e[0] * rhs.e[2],
            self.e[0] * rhs.e[1] - self.e[1] * rhs.e[0],
        )
    }
    pub fn unit(&self) -> Vec3 {
        self / self.length()
    }
    pub fn to_rgb(&self) -> [u8; 3] {
        let ir = (255.999 * self.e[0]) as u8;
        let ig = (255.999 * self.e[1]) as u8;
        let ib = (255.999 * self.e[2]) as u8;
        [ir, ig, ib]
    }
    pub fn to_rgb_scaled(&self, samples_per_pixel: i32) -> [u8; 3] {
        // Divide the color total by the number of samples.
        let scale = 1.0f64 / samples_per_pixel as f64;
        let r = self.e[0] * scale;
        let g = self.e[1] * scale;
        let b = self.e[2] * scale;
        let ir = (256f64 * clamp(r, 0., 0.999)) as u8;
        let ig = (256f64 * clamp(g, 0., 0.999)) as u8;
        let ib = (256f64 * clamp(b, 0., 0.999)) as u8;
        [ir, ig, ib]
    }
    pub fn to_rgb_scaled_gamma2(&self, samples_per_pixel: i32) -> [u8; 3] {
        // Divide the color total by the number of samples.
        let scale = 1.0f64 / samples_per_pixel as f64;
        let r = (self.e[0] * scale).sqrt();
        let g = (self.e[1] * scale).sqrt();
        let b = (self.e[2] * scale).sqrt();
        let ir = (256f64 * clamp(r, 0., 0.999)) as u8;
        let ig = (256f64 * clamp(g, 0., 0.999)) as u8;
        let ib = (256f64 * clamp(b, 0., 0.999)) as u8;
        [ir, ig, ib]
    }
    pub fn random() -> Self {
        let mut rng = thread_rng();
        Self
        {
            e: [rng.gen(), rng.gen(), rng.gen()]
        }
    }
    pub fn random_range(min: f64, max: f64) -> Self {
        let mut rng = thread_rng();
        Self
        {
            e: [rng.gen_range(min, max),
                rng.gen_range(min, max),
                rng.gen_range(min, max)]
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3::new(-self.x(), -self.y(), -self.z())
    }
}

impl Neg for &Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3::new(-self.x(), -self.y(), -self.z())
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, index: usize) -> &f64 {
        &self.e[index]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Self) -> Vec3 {
        Vec3::new(
            self.e[0] + rhs.e[0],
            self.e[1] + rhs.e[1],
            self.e[2] + rhs.e[2],
        )
    }
}

impl Add<&Self> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: &Self) -> Vec3 {
        Vec3::new(
            self.e[0] + rhs.e[0],
            self.e[1] + rhs.e[1],
            self.e[2] + rhs.e[2],
        )
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Self) -> Vec3 {
        Vec3::new(
            self.e[0] - rhs.e[0],
            self.e[1] - rhs.e[1],
            self.e[2] - rhs.e[2],
        )
    }
}
impl Sub<&Self> for &Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: &Self) -> Vec3 {
        Vec3::new(
            self.e[0] - rhs.e[0],
            self.e[1] - rhs.e[1],
            self.e[2] - rhs.e[2],
        )
    }
}

impl Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Self) -> Vec3 {
        Vec3::new(
            self.e[0] * rhs.e[0],
            self.e[1] * rhs.e[1],
            self.e[2] * rhs.e[2],
        )
    }
}
impl Mul<&Self> for &Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: &Self) -> Vec3 {
        Vec3::new(
            self.e[0] * rhs.e[0],
            self.e[1] * rhs.e[1],
            self.e[2] * rhs.e[2],
        )
    }
}
impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Vec3 {
        Vec3::new(self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs)
    }
}
impl Mul<f64> for &Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Vec3 {
        Vec3::new(self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        rhs * self
    }
}
impl Mul<&Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: &Vec3) -> Vec3 {
        rhs * self
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Vec3 {
        self * (1. / rhs)
    }
}
impl Div<f64> for &Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Vec3 {
        self * (1. / rhs)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.e[0] += rhs.e[0];
        self.e[1] += rhs.e[1];
        self.e[2] += rhs.e[2];
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.e[0] -= rhs.e[0];
        self.e[1] -= rhs.e[1];
        self.e[2] -= rhs.e[2];
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.e[0] *= rhs;
        self.e[1] *= rhs;
        self.e[2] *= rhs;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1. / rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_vec3_default() {
        let u = Vec3::default();
        assert_eq!(u.x(), 0.);
        assert_eq!(u.y(), 0.);
        assert_eq!(u.z(), 0.);
    }
    #[test]
    fn test_vec3_new() {
        let u = Vec3::new(1., 2., 3.);
        assert_eq!(u.x(), 1.);
        assert_eq!(u.y(), 2.);
        assert_eq!(u.z(), 3.);
        assert_eq!(u.length_squared(), 14.0);
        assert_eq!(u.length(), (14.0f64).sqrt());
    }
    #[test]
    fn test_vec3_dot() {
        let u = Vec3::new(1., 2., 3.);
        let v = Vec3::new(4., 5., 6.);
        assert_eq!(u.dot(v), 32.0);
    }
    #[test]
    fn test_vec3_cross() {
        let u = Vec3::new(1., 2., 3.);
        let v = Vec3::new(4., 5., 6.);
        let result = Vec3::new(-3.0, 6.0, -3.0);
        assert_eq!(u.cross(&v), result);
    }

    #[test]
    fn test_vec3_ops() {
        let u = Vec3::new(1., 2., 3.);
        let v = Vec3::new(4., 5., 6.);
        let t = 2.0f64;

        let sum = Vec3::new(5.0, 7.0, 9.0);
        assert_eq!(u + v, sum);

        let diff = Vec3::new(-3.0, -3.0, -3.0);
        assert_eq!(u - v, diff);

        let scaled_u = Vec3::new(2.0, 4.0, 6.0);
        assert_eq!(t * u, scaled_u);
        assert_eq!(u * t, scaled_u);

        let scaled_u = Vec3::new(2.0, 4.0, 6.0);
        assert_eq!(scaled_u / t, u);

        let product = Vec3::new(4., 10., 18.);
        assert_eq!(u * v, product);

        assert_eq!(-u, Vec3::new(-1., -2., -3.));
    }
}
