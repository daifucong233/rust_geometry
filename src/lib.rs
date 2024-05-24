/// 浮点比较误差常数
pub const EPS: f64 = 1e-9;

/// 浮点比较函数，当两数之差绝对值不超过 `EPS` 时则认为它们相等，用以防止浮点型精度误差。
pub fn eq_f64(a: f64, b: f64) -> bool {
    (a - b).abs() < EPS
}

pub mod point;
pub mod line;
pub mod round;
pub mod convex_hull;