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
pub fn incentre(a: Point, b: Point, c: Point) -> Point {
    if a == b && b == c {
        return a
    }
    let la = (b - c).dis();
    let lb = (a - c).dis();
    let lc = (a - b).dis();
    (a * la + b * lb + c * lc) / (la + lb + lc)
}

/// todo
pub fn circumcentre(a: Point, b: Point, c: Point) -> Option<Point> {
    if a == b && b == c {
        return Some(a)
    }
    if ((b - a) ^ (c - a)).abs() < EPS {
        return None
    }
    Some(Point::new(0.0, 0.0))
}