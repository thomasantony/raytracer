
use std::ops::*;

#[derive(Debug, Clone, Copy)]
pub struct Vec3
{
    e: [f64; 3]
}
pub type Point = Vec3;
pub type Color = Vec3;

impl Vec3 {
    pub fn default() -> Self 
    {
        Self {
            e: [0., 0., 0.]
        }
    }
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self 
    {
        Self {
            e: [e0, e1, e2]
        }
    }
    pub fn x(&self) -> f64 
    {
        self.e[0]
    }
    pub fn y(&self) -> f64 
    {
        self.e[1]
    }
    pub fn z(&self) -> f64 
    {
        self.e[2]
    }
    pub fn length(&self) -> f64
    {
        self.length_squared().sqrt()
    }
    pub fn length_squared(&self) -> f64
    {
        self.e[0]*self.e[0] + self.e[1]*self.e[1] + self.e[2]*self.e[2]
    }
    pub fn dot(&self, rhs: &Vec3) -> f64 
    {
        self.e[0] * rhs.e[0] +
        self.e[1] * rhs.e[1] +
        self.e[2] * rhs.e[2]
    }
    pub fn cross(&self, rhs: &Vec3) -> Vec3 
    {
        Vec3::new(self.e[1] * rhs.e[2] - self.e[2] * rhs.e[1],
                  self.e[2] * rhs.e[0] - self.e[0] * rhs.e[2],
                  self.e[0] * rhs.e[1] - self.e[1] * rhs.e[0])
    }
    pub fn unit(&self) -> Vec3 
    {
        self / self.length()
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

impl IndexMut<usize> for Vec3 
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}

impl Add<&Self> for &Vec3 
{
    type Output = Vec3;
    fn add(self, rhs: &Self) -> Vec3
    {
        Vec3::new(self.e[0] + rhs.e[0], self.e[1] + rhs.e[1], self.e[2] + rhs.e[2])
    }
}


impl Sub<&Self> for &Vec3 
{
    type Output = Vec3;
    fn sub(self, rhs: &Self) -> Vec3
    {
        Vec3::new(self.e[0] - rhs.e[0], self.e[1] - rhs.e[1], self.e[2] - rhs.e[2])
    }
}

impl Mul<&Self> for &Vec3 
{
    type Output = Vec3;
    fn mul(self, rhs: &Self) -> Vec3
    {
        Vec3::new(self.e[0] * rhs.e[0], self.e[1] * rhs.e[1], self.e[2] * rhs.e[2])
    }
}

impl Mul<f64> for &Vec3 
{
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Vec3
    {
        Vec3::new(self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs)
    }
}

impl Mul<&Vec3> for f64
{
    type Output = Vec3;
    fn mul(self, rhs: &Vec3) -> Vec3
    {
        rhs * self
    }
}

impl Div<f64> for &Vec3 
{
    type Output = Vec3;
    fn div(self, rhs: f64) -> Vec3
    {
        self * (1./rhs)
    }
}

impl AddAssign<&Self> for Vec3 
{
    fn add_assign(&mut self, rhs: &Self)
    {
        self.e[0] += rhs.e[0];
        self.e[1] += rhs.e[1];
        self.e[2] += rhs.e[2];
    }
}

impl MulAssign<f64> for Vec3 
{
    fn mul_assign(&mut self, rhs: f64)
    {
        self.e[0] *= rhs;
        self.e[1] *= rhs;
        self.e[2] *= rhs;
    }
}

impl DivAssign<f64> for Vec3 
{
    fn div_assign(&mut self, rhs: f64)
    {
        *self *= 1./rhs;
    }
}