use crate::*;
use crate::point::*;
use crate::line::*;
use std::fmt;

#[derive(Debug, Copy, Clone)]
pub struct Round {
    pub o: Point,
    pub r: f64,
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

    /// 计算圆与直线的交点，若直线与圆相离则返回 `None`。
    pub fn inter_line(&self, l: Line) -> Option<(Point, Point)> {
        let proj = l.proj(self.o);
        let dis = (self.o - proj).dis();
        if eq_f64(dis, self.r) {
            return Some((proj, proj))
        }
        else if dis > self.r {
            return None
        }

        let delt = (self.r * self.r - dis * dis).sqrt();
        let delt_v = l.vec().normalize();
        Some((proj + delt_v * delt, proj - delt_v * delt))
    }

    /// 计算圆与圆的交点，如果两圆包含或相离则返回 `None`。
    pub fn inter_round(&self, rd: Round) -> Option<(Point, Point)> {
        if self.o == rd.o {
            return None
        }
        let odis = (self.o - rd.o).dis();
        if eq_f64(odis, (self.r - rd.r).abs()) {
            let ans = if self.r > rd.r {
                self.o + (rd.o - self.o).normalize() * self.r
            }
            else {
                rd.o + (self.o - rd.o).normalize() * rd.r
            };
            return Some((ans, ans))
        }
        if eq_f64(odis, self.r + rd.r) {
            let ans = self.o + (rd.o - self.o).normalize() * self.r;
            return Some((ans, ans))
        }
        if odis < (self.r - rd.r).abs() || odis > self.r + rd.r {
            return None
        }
        
        let theta = ((self.r * self.r + odis * odis - rd.r * rd.r) / (2.0 * self.r * odis)).acos();
        Some((self.o + ((rd.o - self.o).normalize() * self.r).rot(theta),
              self.o + ((rd.o - self.o).normalize() * self.r).rot(-theta)))
    }

    /// 计算点到圆的切点。
    pub fn tangent_point(&self, p: Point) -> Option<(Point, Point)> {
        let dis = (self.o - p).dis();
        if eq_f64(dis, self.r) {
            return Some((p, p))
        }
        else if dis < self.r {
            return None
        }

        let theta = (self.r / dis).acos();
        Some((self.o + ((p - self.o).normalize() * self.r).rot(theta),
              self.o + ((p - self.o).normalize() * self.r).rot(-theta)))
    }

    /// 计算圆到圆的外切线，当两圆包含、内切时返回 `None`。
    pub fn tangent_round_exterior(&self, rd: Round) -> Option<(Line, Line)> {
        let dis = (self.o - rd.o).dis();
        if dis < (self.r - rd.r).abs() + EPS {
            return None
        }
        
        let theta = ((self.r - rd.r) / dis).acos();
        let alpha = (rd.o - self.o).normalize();
        Some((
            Line::new(self.o + alpha.rot(theta) * self.r, rd.o + alpha.rot(theta) * rd.r),
            Line::new(self.o + alpha.rot(-theta) * self.r, rd.o + alpha.rot(-theta) * rd.r)
        ))
    }

    /// 计算圆到圆的内切线，当两圆包含、相交、相切时返回 `None`。
    pub fn tangent_round_interior(&self, rd: Round) -> Option<(Line, Line)> {
        let dis = (self.o - rd.o).dis();
        if dis < self.r + rd.r + EPS {
            return None
        }
        let theta = ((self.r + rd.r) / dis).acos();
        let alpha = (rd.o - self.o).normalize();
        let beta = (self.o - rd.o).normalize();
        Some((
            Line::new(self.o + alpha.rot(theta) * self.r, rd.o + beta.rot(theta) * rd.r),
            Line::new(self.o + alpha.rot(-theta) * self.r, rd.o + beta.rot(-theta) * rd.r),
        ))
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
    if eq_f64((b - a) ^ (c - a), 0.0) {
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inter_test() {
        let rd = Round::new(Point::new(0.0, 1.0), 2.0_f64.sqrt());
        let rd2 = Round::new(Point::new(0.0, -1.0), 2.0_f64.sqrt());

        let l = Line::new(Point::new(0.0, 0.0), Point::new(1.0, 0.0));

        let inter_pt = rd.inter_line(l);
        let inter_rd = rd.inter_round(rd2);

        let ans1 = Some((Point::new(1.0, 0.0), Point::new(-1.0, 0.0)));
        let ans2 = Some((Point::new(-1.0, 0.0), Point::new(1.0, 0.0)));

        assert!(inter_pt == ans1 || inter_pt == ans2);
        assert!(inter_rd == ans1 || inter_rd == ans2);
    }

    #[test]
    fn tangent_points_and_lines_test() {
        let rd1 = Round::new(Point::new(15.0, 73.0), 7.0);
        let rd2 = Round::new(Point::new(40.0, 19.0), 4.0);
        let pt = Point::new(-54.0, 96.0);

        let tang_pt = rd1.tangent_point(pt);

        match tang_pt {
            Some(p) => {
                assert!(eq_f64((pt - p.0) * (rd1.o - p.0), 0.0));
                assert!(eq_f64((pt - p.1) * (rd1.o - p.1), 0.0));
            }
            None => panic!("unexpected result")
        }

        let tang_exterior = rd1.tangent_round_exterior(rd2);

        match tang_exterior {
            Some(l) => {
                let d1 = (l.0.proj(rd1.o) - rd1.o).dis();
                let d2 = (l.0.proj(rd2.o) - rd2.o).dis();
                assert!(eq_f64(d1, rd1.r));
                assert!(eq_f64(d2, rd2.r));
                assert!((l.0.vec() ^ (rd1.o - l.0.a) > 0.0) == (l.0.vec() ^ (rd2.o - l.0.a) > 0.0));
                let d1 = (l.1.proj(rd1.o) - rd1.o).dis();
                let d2 = (l.1.proj(rd2.o) - rd2.o).dis();
                assert!(eq_f64(d1, rd1.r));
                assert!(eq_f64(d2, rd2.r));
                assert!((l.1.vec() ^ (rd1.o - l.1.a) > 0.0) == (l.1.vec() ^ (rd2.o - l.1.a) > 0.0));
            }
            None => panic!("unexpected result")
        }
    
        let tang_interior = rd1.tangent_round_interior(rd2);
        match tang_interior {
            Some(l) => {
                let d1 = (l.0.proj(rd1.o) - rd1.o).dis();
                let d2 = (l.0.proj(rd2.o) - rd2.o).dis();
                assert!(eq_f64(d1, rd1.r));
                assert!(eq_f64(d2, rd2.r));
                assert!((l.0.vec() ^ (rd1.o - l.0.a) > 0.0) != (l.0.vec() ^ (rd2.o - l.0.a) > 0.0));
                let d1 = (l.1.proj(rd1.o) - rd1.o).dis();
                let d2 = (l.1.proj(rd2.o) - rd2.o).dis();
                assert!(eq_f64(d1, rd1.r));
                assert!(eq_f64(d2, rd2.r));
                assert!((l.1.vec() ^ (rd1.o - l.1.a) > 0.0) != (l.1.vec() ^ (rd2.o - l.1.a) > 0.0));
            }
            None => panic!("unexpected result")
        }
    }
}