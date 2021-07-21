use super::line2d::EPS;
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
    pub fn is_point_on_circle(&self, p: &Point2d) -> bool {
        self.distance_by_point(p) < EPS
    }
    pub fn distance_by_point(&self, p: &Point2d) -> f64 {
        ((self.center - p).l2_norm() - self.radius()).abs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn on_circle() {
        let circle = Circle2d::new(&Point2d::new(1.0, 2.0), 1.0);
        assert!(circle.is_point_on_circle(&Point2d::new(0.0, 2.0)));
        assert!(circle.is_point_on_circle(&Point2d::new(1.0, 3.0)));
        assert!(!circle.is_point_on_circle(&Point2d::new(1.0, 2.0)));
        assert!(!circle.is_point_on_circle(&Point2d::new(1.0, 3.1)));
    }
}
