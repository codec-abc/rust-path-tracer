use std::ops::Add;
use std::ops::Sub;
use std::ops::Rem;
use std::ops::Mul;

#[derive(Copy, Clone)]
pub struct Vector
{
    pub x : f64,
    pub y : f64,
    pub z : f64
}

impl Vector
{
    #[allow(dead_code)]
    pub fn new_zero() -> Vector
    {
        Vector { x : 0.0, y : 0.0, z : 0.0}
    }

    pub fn new(x : f64 , y : f64 , z : f64) -> Vector
    {
        Vector { x : 0.0, y : 0.0, z : 0.0}
    }

    #[allow(dead_code)]
    pub fn mult(&self, b : &Vector) -> Vector
    {
        Vector
        {
            x : self.x * b.x,
            y : self.y * b.y,
            z : self.z * b.z
        }
    }

    #[allow(dead_code)]
    pub fn norm(&self) -> Vector
    {
        let factor = (1.0)/((self.x*self.x + self.y*self.y + self.z*self.z).sqrt());
        Vector
        {
            x : self.x*factor,
            y : self.y*factor,
            z : self.z*factor
        }
    }

    #[allow(dead_code)]
    pub fn dot(&self, b : &Vector) -> f64
    {
        self.x*b.x + self.y*b.y + self.z*b.z
    }
}
impl Add for Vector
{
    type Output = Vector;
    fn add(self, other: Vector) -> Vector
    {
        Vector
        {
            x : self.x + other.x,
            y : self.y + other.y,
            z : self.z + other.z
        }
    }
}
impl Sub for Vector
{
    type Output = Vector;
    fn sub(self, other: Vector) -> Vector
    {
        Vector
        {
            x : self.x - other.x,
            y : self.y - other.y,
            z : self.z - other.z
        }
    }
}
impl Rem for Vector
{
    type Output = Vector;
    fn rem(self, b: Vector) -> Vector
    {
        Vector
        {
            x : self.y*b.z - self.z*b.y,
            y : self.z*b.x - self.x*b.z,
            z : self.x*b.y - self.y*b.x
        }
    }
}
impl Mul<f64> for Vector
{
    type Output = Vector;
    fn mul(self, b: f64) -> Vector
    {
        Vector
        {
            x : self.x * b,
            y : self.y * b,
            z : self.z * b
        }
    }
}
