use crate::*;
use std::fmt;
use std::cmp::PartialEq;
use std::ops::{Add, Sub, Mul, Div, Neg, BitXor};

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl PartialEq for Point {
    /// 基于 `EPS` 常量，定义了两个点（向量）的相等关系。
    fn eq(&self, p: &Self) -> bool {
        eq_f64(self.x, p.x) && eq_f64(self.y, p.y)
    }
}

impl fmt::Display for Point {
    /// 支持点的输出。
    ///
    ///     use rust_geometry::point::Point;
    ///     
    ///     let p = Point::new(0.0, 0.0);
    ///     println!("{}", p); // (0.00000,0.00000)
    ///
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:.5},{:.5})", self.x, self.y)
    }
}

impl Add for Point {
    type Output = Self;

    /// 实现向量的加法。
    fn add(self, p: Self) -> Self {
        Self {
            x: self.x + p.x,
            y: self.y + p.y,
        } 
    }
}

impl Sub for Point {
    type Output = Self;

    /// 实现向量的加法。
    fn sub(self, p: Self) -> Self {
        Self {
            x: self.x - p.x,
            y: self.y - p.y,
        } 
    }
}

impl Neg for Point {
    type Output = Self;

    /// 实现向量的取负。
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        } 
    }
}

impl Mul<f64> for Point {
    type Output = Self;

    /// 实现向量与常数的乘法。
    fn mul(self, a: f64) -> Self {
        Self {
            x: self.x * a,
            y: self.y * a,
        } 
    }
}

impl Mul for Point {
    type Output = f64;

    /// 实现向量与向量的点乘，结果为浮点数。
    ///
    ///     use rust_geometry::eq_f64;
    ///     use rust_geometry::point::Point;
    ///
    ///     let a = Point::new(1.0, 2.0);
    ///     let b = Point::new(3.0, 4.0);
    ///     let c = a * b;
    ///     assert!(eq_f64(c, 11.0));
    ///
    fn mul(self, p: Self) -> Self::Output {
        self.x * p.x + self.y * p.y
    }
}

impl Div<f64> for Point {
    type Output = Self;

    /// 实现向量和常数的除法。
    fn div(self, a: f64) -> Self {
        Self {
            x: self.x / a,
            y: self.y / a,
        } 
    }
}

impl BitXor for Point {
    type Output = f64;

    /// 实现向量与向量的叉乘，结果为浮点数。
    ///
    ///     use rust_geometry::eq_f64;
    ///     use rust_geometry::point::Point;
    ///
    ///     let a = Point::new(1.0, 2.0);
    ///     let b = Point::new(3.0, 4.0);
    ///     let c = a ^ b;
    ///     assert!(eq_f64(c, -2.0));
    ///
    fn bitxor(self, p: Self) -> Self::Output {
        self.x * p.y - self.y * p.x
    }
}

impl Point {
    /// 根据坐标初始话点（向量）对象。
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }

    /// 点到原点的距离，向量的模长。
    pub fn dis(&self) -> f64 {
        ((*self) * (*self)).sqrt()
    }

    /// 点到原点的距离的平方，向量模长的平方。
    pub fn sqrdis(&self) -> f64 {
        (*self) * (*self)
    }

    /// 将向量逆时针旋转 `theta` 的角度，其中 `theta` 为弧度制。
    pub fn rot(&self, theta: f64) -> Point {
        Point {
            x: self.x * theta.cos() - self.y * theta.sin(),
            y: self.x * theta.sin() + self.y * theta.cos(),
        }
    }
    
    /// 计算两个向量之间相差的角度，即第一个向量逆时针旋转多少度能与第二个向量方向重合。返回角度为弧度制，在 (-pi, pi] 之间。
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
        assert!(eq_f64(p1.x, p2.x));
        assert!(eq_f64(p1.y, p2.y));
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
        assert!(eq_f64(mulp, 16.0));
        assert_eq!(p_div_2, Point::new(0.5, 1.0));
        assert!(eq_f64(cross, -2.0));
    }

    #[test]
    fn dis_test() {
        let p = Point::new(3.0, 4.0);
        let d = p.dis();
        let sqrd = p.sqrdis();
        assert!(eq_f64(d, 5.0));
        assert!(eq_f64(sqrd, 25.0));
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
        assert!(eq_f64(pa.rad(pb), theta));
    }
}
