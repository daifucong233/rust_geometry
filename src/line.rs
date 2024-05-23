use std::fmt;
use crate::point::*;

#[derive(Debug, Copy, Clone)]
pub struct Line{
    a: Point,
    b: Point,
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.a, self.b)
    }
}

impl Line {
    pub fn new(a: Point, b: Point) -> Line {
        Line { a, b }
    }

    pub fn valid(&self) -> bool {
        self.vec().dis() > EPS
    }

    pub fn vec(&self) -> Point {
        self.b - self.a
    }

    pub fn len(&self) -> f64 {
        self.vec().dis()
    }

    pub fn sqrlen(&self) -> f64 {
        self.vec().sqrdis()
    }

    pub fn proj(&self, p: Point) -> Point {
        self.a + self.vec() * (((p - self.a) * self.vec()) / self.sqrlen())
    }

    pub fn inter(&self, l: Line) -> Option<Point> {
        if (self.vec() ^ l.vec()).abs() < EPS {
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
        
        assert!((d - 2.0_f64.sqrt()).abs() < EPS)
    }

    #[test]
    fn proj_test() {
        let l = Line::new(Point::new(0.0, 0.0), Point::new(1.0, 1.0));
        let p = Point::new(1.0, 0.0);
        let proj = l.proj(p);
        assert_eq!(proj, Point::new(0.5, 0.5));
    }

    #[test]
    fn inter_test() {
        let la = Line::new(Point::new(0.0, 0.0), Point::new(1.0, 1.0));
        let lb = Line::new(Point::new(1.0, 0.0), Point::new(0.0, 1.0));
        let p = la.inter(lb);
        assert_eq!(p, Some(Point::new(0.5, 0.5)));
        
        let la = Line::new(Point::new(0.0, 0.0), Point::new(0.0, 1.0));
        let lb = Line::new(Point::new(1.0, 0.0), Point::new(1.0, 1.0));
        let p = la.inter(lb);
        assert_eq!(p, None);
    }
}