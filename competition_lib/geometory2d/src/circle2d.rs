use super::line2d::EPS;
use super::point2d::Point2d;

pub enum CONTAINS {
    IN,
    OUT,
    ON,
}

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
    pub fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius() * self.radius()
    }
    pub fn contains(&self, p: &Point2d) -> CONTAINS {
        let signed_dist = (self.center - p).l2_norm() - self.radius();
        if signed_dist.abs() < EPS {
            CONTAINS::ON
        } else if signed_dist > 0.0 {
            CONTAINS::OUT
        } else {
            CONTAINS::IN
        }
    }
    pub fn distance_by_point(&self, p: &Point2d) -> f64 {
        ((self.center - p).l2_norm() - self.radius()).abs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn area() {
        let circle = Circle2d::new(&Point2d::new(1.0, 2.0), 2.0);
        assert_eq!(circle.area(), std::f64::consts::PI * 4.0);
    }
    #[test]
    fn contains() {
        let circle = Circle2d::new(&Point2d::new(1.0, 2.0), 1.0);
        assert!(matches!(
            circle.contains(&Point2d::new(0.0, 2.0)),
            CONTAINS::ON
        ));
        assert!(matches!(
            circle.contains(&Point2d::new(1.0, 3.0)),
            CONTAINS::ON
        ));
        assert!(matches!(
            circle.contains(&Point2d::new(1.0, 2.0)),
            CONTAINS::IN
        ));
        assert!(matches!(
            circle.contains(&Point2d::new(1.0, 3.1)),
            CONTAINS::OUT
        ));
    }
}
