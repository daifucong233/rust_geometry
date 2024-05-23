use std::fmt;
use std::cmp::PartialEq;
use std::ops::{Add, Sub, Mul, Div, Neg, BitXor};

pub const EPS: f64 = 1e-9;

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl PartialEq for Point {
    fn eq(&self, p: &Self) -> bool {
        (self.x - p.x).abs() < EPS && (self.y - p.y).abs() < EPS
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:.5},{:.5})", self.x, self.y)
    }
}

impl Add for Point {
    type Output = Self;
    fn add(self, p: Self) -> Self {
        Self {
            x: self.x + p.x,
            y: self.y + p.y,
        } 
    }
}

impl Sub for Point {
    type Output = Self;
    fn sub(self, p: Self) -> Self {
        Self {
            x: self.x - p.x,
            y: self.y - p.y,
        } 
    }
}

impl Neg for Point {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        } 
    }
}

impl Mul<f64> for Point {
    type Output = Self;
    fn mul(self, a: f64) -> Self {
        Self {
            x: self.x * a,
            y: self.y * a,
        } 
    }
}

impl Mul for Point {
    type Output = f64;
    fn mul(self, p: Self) -> Self::Output {
        self.x * p.x + self.y * p.y
    }
}

impl Div<f64> for Point {
    type Output = Self;
    fn div(self, a: f64) -> Self {
        Self {
            x: self.x / a,
            y: self.y / a,
        } 
    }
}

impl BitXor for Point {
    type Output = f64;
    fn bitxor(self, p: Self) -> Self::Output {
        self.x * p.y - self.y * p.x
    }
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }

    pub fn dis(&self) -> f64 {
        ((*self) * (*self)).sqrt()
    }

    pub fn sqrdis(&self) -> f64 {
        (*self) * (*self)
    }

    pub fn rot(&self, theta: f64) -> Point {
        Point {
            x: self.x * theta.cos() - self.y * theta.sin(),
            y: self.x * theta.sin() + self.y * theta.cos(),
        }
    }
    
    pub fn rad(&self, p: Point) -> f64 {
        (*self ^ p).atan2(*self * p)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn copy_type_test() {
        let p1 = Point::new(1.0, 2.0);
        let p2 = p1;
        assert!((p1.x - p2.x).abs() < EPS);
        assert!((p1.y - p2.y).abs() < EPS);
    }

    #[test]
    fn ops_test() {
        let p1 = Point::new(1.0, 2.0);
        let p2 = Point::new(4.0, 6.0);
        let addp = p1 + p2;
        let subp = p1 - p2;
        let negp = -p1;
        let p_mul_2 = p1 * 2.0;
        let mulp = p1 * p2;
        let p_div_2 = p1 / 2.0;
        let cross = p1 ^ p2;
        assert_eq!(addp, Point::new(5.0, 8.0));
        assert_eq!(subp, Point::new(-3.0, -4.0));
        assert_eq!(negp, Point::new(-1.0, -2.0));
        assert_eq!(p_mul_2, Point::new(2.0, 4.0));
        assert!((mulp - 16.0).abs() < EPS);
        assert_eq!(p_div_2, Point::new(0.5, 1.0));
        assert!((cross + 2.0).abs() < EPS);
    }

    #[test]
    fn dis_test() {
        let p = Point::new(3.0, 4.0);
        let d = p.dis();
        let sqrd = p.sqrdis();
        assert!((d - 5.0).abs() < EPS);
        assert!((sqrd - 25.0).abs() < EPS);
    }

    #[test]
    fn rot_test() {
        let p = Point::new(3.0, 4.0);
        let rotp = p.rot(PI / 2.0);
        assert_eq!(rotp, Point::new(-4.0, 3.0));
    }

    #[test]
    fn rad_test() {
        let pa = Point::new(1.0, 0.0);
        let pb = Point::new(1.0, 3.0_f64.sqrt());
        let theta = PI / 3.0;
        assert!((pa.rad(pb) - theta).abs() < EPS);
    }
}
