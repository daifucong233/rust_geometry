use crate::*;
use crate::point::*;
use std::cmp::Ordering::*;
use std::fmt;

pub struct ConvexHull {
    pub u_hull: Vec<Point>,
    pub d_hull: Vec<Point>,
}

impl fmt::Display for ConvexHull {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "u {:?} d {:?}", self.u_hull, self.d_hull)
    }
}

impl ConvexHull {
    /// 直接用上凸壳和下凸壳创建一个凸包，注意输入的所有权会被转移。
    pub fn new(u_hull: Vec<Point>, d_hull: Vec<Point>) -> Self {
        ConvexHull { u_hull, d_hull }
    }

    /// 判断凸包对象中的上凸壳和下凸壳是否合法。
    pub fn valid(&self) -> bool {
        let ulen = self.u_hull.len();
        let dlen = self.d_hull.len();


        if self.u_hull[0] != self.d_hull[0] {
            return false
        }
        if self.u_hull[ulen - 1] != self.d_hull[dlen - 1] {
            return false
        }
        
        for i in 0 .. ulen - 2 {
            if ((self.u_hull[i + 1] - self.u_hull[i]) ^ (self.u_hull[i + 2] - self.u_hull[i])) > -EPS {
                return false
            }
        }
        for i in 0 .. dlen - 2 {
            if ((self.d_hull[i + 1] - self.d_hull[i]) ^ (self.d_hull[i + 2] - self.d_hull[i])) < EPS {
                return false
            }
        }

        true
    }

    /// 使用叉积法计算凸包面积。
    /// 
    ///     use rust_geometry::EPS;
    ///     use rust_geometry::point::Point;
    ///     use rust_geometry::convex_hull::ConvexHull;
    ///
    ///     let vec = vec![Point::new(0.0, 0.0), Point::new(1.0, 0.0), Point::new(0.0, 1.0),
    ///                    Point::new(1.0, 1.0), Point::new(0.5, 0.5)];
    ///     let convex_hull = ConvexHull::get_convex_hull(vec);
    ///     let s = convex_hull.area();
    ///     assert!((s - 1.0).abs() < EPS);
    ///
    pub fn area(&self) -> f64 {
        let mut ans: f64 = 0.0;

        let ulen = self.u_hull.len();
        let dlen = self.d_hull.len();

        for i in 1 .. ulen - 1 {
            ans += (self.u_hull[i + 1] - self.u_hull[0]) ^ (self.u_hull[i] - self.u_hull[0]);
        }
        for i in 1 .. dlen - 1 {
            ans -= (self.d_hull[i + 1] - self.d_hull[0]) ^ (self.d_hull[i] - self.d_hull[0]);
        }
        ans / 2.0
    }

    /// 返回凸包上的点按照逆时针顺序的排列，不会破坏原凸包对象的所有权。
    /// 
    ///     use rust_geometry::point::Point;
    ///     use rust_geometry::convex_hull::ConvexHull;
    ///
    ///     let vec = vec![Point::new(0.0, 0.0), Point::new(1.0, 0.0), Point::new(0.0, 1.0),
    ///                    Point::new(1.0, 1.0), Point::new(0.5, 0.5)];
    ///     let convex_hull = ConvexHull::get_convex_hull(vec);
    ///     let v = convex_hull.get_points();
    ///     let ans = vec![Point::new(0.0, 0.0), Point::new(1.0, 0.0), Point::new(1.0, 1.0), Point::new(0.0, 1.0)];
    ///     assert_eq!(v, ans);
    ///
    pub fn get_points(&self) -> Vec<Point> {
        let mut u_copy = self.u_hull.clone();
        let mut d_copy = self.d_hull.clone();

        u_copy.reverse();

        u_copy.pop();
        d_copy.pop();

        d_copy.extend(u_copy);
        d_copy
    }
    
    /// 凸包求解函数中将点排序用到的比较函数。
    fn pt_cmp(a: &Point, b: &Point) -> std::cmp::Ordering {
        if !eq_f64(a.x, b.x) {
            if a.x < b.x { Less } else { Greater }
        }
        else if !eq_f64(a.y, b.y) {
            if a.y < b.y { Less } else { Greater }
        }
        else {
            Equal
        }
    }

    /// 根据给定点集计算它的凸包，输入的所有权被转移。
    /// 
    ///     use rust_geometry::point::Point;
    ///     use rust_geometry::convex_hull::ConvexHull;
    ///
    ///     let vec = vec![Point::new(0.0, 0.0), Point::new(1.0, 0.0), Point::new(0.0, 1.0),
    ///                    Point::new(1.0, 1.0), Point::new(0.5, 0.5)];
    ///     let convex_hull = ConvexHull::get_convex_hull(vec);
    ///     let ans_u = vec![Point::new(0.0, 0.0), Point::new(0.0, 1.0), Point::new(1.0, 1.0)];
    ///     let ans_d = vec![Point::new(0.0, 0.0), Point::new(1.0, 0.0), Point::new(1.0, 1.0)];
    ///     assert_eq!(ans_u, convex_hull.u_hull);
    ///     assert_eq!(ans_d, convex_hull.d_hull);
    ///
    pub fn get_convex_hull(mut pts: Vec<Point>) -> ConvexHull {
        let mut ucnt: usize = 0;
        let mut dcnt: usize = 0;
        let mut u_hull: Vec<Point> = Vec::new();
        let mut d_hull: Vec<Point> = Vec::new();
    
        pts.sort_by(|a, b| Self::pt_cmp(a, b));
        for p in pts {
            while ucnt >= 2 && ((u_hull[ucnt - 1] - u_hull[ucnt - 2]) ^ (p - u_hull[ucnt - 2])) > -EPS {
                u_hull.pop();
                ucnt -= 1;
            }
            while dcnt >= 2 && ((d_hull[dcnt - 1] - d_hull[dcnt - 2]) ^ (p - d_hull[dcnt - 2])) < EPS {
                d_hull.pop();
                dcnt -= 1;
            }
            u_hull.push(p);
            ucnt += 1;
            d_hull.push(p);
            dcnt += 1;
        }
    
        ConvexHull {u_hull, d_hull}
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convex_hull_test() {
        let vec = vec![Point::new(0.0, 0.0), Point::new(1.0, 0.0), Point::new(0.0, 1.0),
                       Point::new(1.0, 1.0), Point::new(0.5, 0.5)];
        let convex_hull = ConvexHull::get_convex_hull(vec);
        let area = convex_hull.area();

        assert!(convex_hull.valid());
        assert!((area - 1.0).abs() < EPS);

        let pts = convex_hull.get_points();
        let stdans = vec![Point::new(0.0, 0.0), Point::new(1.0, 0.0), Point::new(1.0, 1.0), Point::new(0.0, 1.0)];
        assert_eq!(pts, stdans);
    }
}