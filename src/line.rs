use crate::eq_f64;
use crate::point::*;
use std::fmt;

#[derive(Debug, Copy, Clone)]
pub struct Line{
    pub a: Point,
    pub b: Point,
}

impl fmt::Display for Line {
    /// 支持以两点坐标形式输出直线
    ///
    ///     use rust_geometry::point::Point;
    ///     use rust_geometry::line::Line;
    ///     
    ///     let l = Line::new(Point::new(0.0, 0.0), Point::new(1.0, 0.0));
    ///     println!("{}", l); // (0.00000,0.00000)-(1.00000,0.00000)
    ///
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.a, self.b)
    }
}

impl Line {
    /// 通过两点构造直线。
    pub fn new(a: Point, b: Point) -> Line {
        Line { a, b }
    }

    /// 判断是否为合法直线（两点间距离是否过小）。
    pub fn valid(&self) -> bool {
        !eq_f64(self.vec().dis(), 0.0)
    }

    /// 计算两点构成向量。
    pub fn vec(&self) -> Point {
        self.b - self.a
    }

    /// 计算线段长度。
    pub fn len(&self) -> f64 {
        self.vec().dis()
    }

    /// 计算线段长度的平方。
    pub fn sqrlen(&self) -> f64 {
        self.vec().sqrdis()
    }

    /// 计算点到直线的投影。
    ///
    ///     use rust_geometry::point::Point;
    ///     use rust_geometry::line::Line;
    ///     let l = Line::new(Point::new(0.0, 0.0), Point::new(1.0, 1.0));
    ///     let p = Point::new(1.0, 0.0);
    ///     let proj = l.proj(p);
    ///     assert_eq!(proj, Point::new(0.5, 0.5));
    ///
    pub fn proj(&self, p: Point) -> Point {
        self.a + self.vec() * (((p - self.a) * self.vec()) / self.sqrlen())
    }

    /// 计算两条直线的交点，如果两直线平行或重合则返回 `None`。
    ///
    ///     use rust_geometry::point::Point;
    ///     use rust_geometry::line::Line;
    ///     
    ///     let la = Line::new(Point::new(0.0, 0.0), Point::new(1.0, 1.0));
    ///     let lb = Line::new(Point::new(1.0, 0.0), Point::new(0.0, 1.0));
    ///     let p = la.inter(lb);
    ///     assert_eq!(p, Some(Point::new(0.5, 0.5)));
    ///     
    ///     let la = Line::new(Point::new(0.0, 0.0), Point::new(0.0, 1.0));
    ///     let lb = Line::new(Point::new(1.0, 0.0), Point::new(1.0, 1.0));
    ///     let p = la.inter(lb);
    ///     assert_eq!(p, None);
    ///
    pub fn inter(&self, l: Line) -> Option<Point> {
        if eq_f64(self.vec() ^ l.vec(), 0.0) {
            None
        }
        else {
            let s1 = (l.a - self.a) ^ (l.b - self.a);
            let s2 = (l.b - self.b) ^ (l.a - self.b);
            Some(self.a + self.vec() * (s1 / (s1 + s2)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copy_type_test() {
        let la = Line::new(Point::new(0.0, 0.0), Point::new(1.0, 1.0));
        let lb = la;

        assert_eq!(la.a, lb.a);
        assert_eq!(la.b, lb.b);
    }

    #[test]
    fn dis_test() {
        let l = Line::new(Point::new(0.0, 0.0), Point::new(1.0, 1.0));
        let d = l.len();
        
        assert!(eq_f64(d, 2.0_f64.sqrt()))
    }
}