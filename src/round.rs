use crate::EPS;
use crate::point::*;
use crate::line::*;
use std::fmt;

#[derive(Debug, Copy, Clone)]
pub struct Round {
    o: Point,
    r: f64,
}

impl fmt::Display for Round {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{} {}]", self.o, self.r)
    }
}

impl Round {
    /// 通过圆心和半径构造圆对象。
    pub fn new(o: Point, r: f64) -> Self {
        Round { o, r }
    }

    /// todo
    pub fn inter_line(&self, _l: Line) -> Option<(Point, Point)> {
        None
    }

    /// todo
    pub fn inter_round(&self, _r: Round) -> Option<(Point, Point)> {
        None
    }

}

/// 计算三个点构成三角形的内心
///
///     use rust_geometry::eq_f64;
///     use rust_geometry::point::Point;
///     use rust_geometry::line::Line;
///     use rust_geometry::round::incentre;
///
///     let a = Point::new(0.0, 0.0);
///     let b = Point::new(1.0, 0.0);
///     let c = Point::new(1.0, 1.0);
///     let o = incentre(a, b, c);
///     
///     let lena = (o - Line::new(b, c).proj(o)).dis();
///     let lenb = (o - Line::new(a, c).proj(o)).dis();
///     let lenc = (o - Line::new(a, b).proj(o)).dis();
///     assert!(eq_f64(lena, lenb));
///     assert!(eq_f64(lenb, lenc));
///
pub fn incentre(a: Point, b: Point, c: Point) -> Point {
    if a == b && b == c {
        return a
    }
    let la = (b - c).dis();
    let lb = (a - c).dis();
    let lc = (a - b).dis();
    (a * la + b * lb + c * lc) / (la + lb + lc)
}

/// 计算三个点构成三角形的外心
///
///     use rust_geometry::point::Point;
///     use rust_geometry::round::circum;
///
///     let a = Point::new(0.0, 0.0);
///     let b = Point::new(1.0, 0.0);
///     let c = Point::new(1.0, 1.0);
///     let o = circum(a, b, c);
///     assert_eq!(o, Some(Point::new(0.5, 0.5)));
///
pub fn circum(a: Point, b: Point, c: Point) -> Option<Point> {
    if a == b && b == c {
        return Some(a)
    }
    if ((b - a) ^ (c - a)).abs() < EPS {
        return None
    }
    let v1 = (b - a) * 2.0;
    let v2 = (c - b) * 2.0;
    let c1 = b.sqrdis() - a.sqrdis();
    let c2 = c.sqrdis() - b.sqrdis();

    let x = (c1 * v2.y - c2 * v1.y) / (v1 ^ v2);
    let y = (c2 * v1.x - c1 * v2.x) / (v1 ^ v2);

    Some(Point { x, y })
}