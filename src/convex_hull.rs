use crate::point::*;
use std::cmp::Ordering::*;
use std::fmt;

pub struct ConvexHull {
    u_hull: Vec<Point>,
    d_hull: Vec<Point>,
}

impl fmt::Display for ConvexHull {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "u[{:?}] d[{:?}]", self.u_hull, self.d_hull)
    }
}

impl ConvexHull {

    pub fn new(u_hull: Vec<Point>, d_hull: Vec<Point>) -> Self {
        ConvexHull { u_hull, d_hull }
    }

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

    fn pt_cmp(a: &Point, b: &Point) -> std::cmp::Ordering {
        if (a.x - b.x).abs() > EPS {
            if a.x < b.x { Less } else { Greater }
        }
        else if (a.y - b.y).abs() > EPS {
            if a.y < b.y { Less } else { Greater }
        }
        else {
            Equal
        }
    }

    /// todo
    pub fn get_points(&self) -> Vec<Point> {
        vec![Point::new(0.0, 0.0)]
    }
    
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

    /// todo
    pub fn inside(_p: Point) -> bool {
        true
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
    }
}