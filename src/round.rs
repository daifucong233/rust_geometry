use crate::point::*;
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
    pub fn new(o: Point, r: f64) -> Self {
        Round { o, r }
    }
}