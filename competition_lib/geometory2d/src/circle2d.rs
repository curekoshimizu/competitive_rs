use super::point2d::Point2d;

pub struct Circle2d {
    center: Point2d,
    rad: f64,
}

impl Circle2d {
    pub fn new(center: &Point2d, rad: f64) -> Self {
        Circle2d {
            center: *center,
            rad,
        }
    }
    pub fn center(&self) -> Point2d {
        self.center
    }
    pub fn radius(&self) -> f64 {
        self.rad
    }
}
