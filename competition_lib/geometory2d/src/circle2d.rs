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
    /// common tangents
    /// 4: Lying outside each other
    /// 3: Touching externally
    /// 2: Intersecting at two points
    /// 1: Touching internally
    /// 0: One lying inside other
    pub fn num_common_tangents(&self, circle: &Circle2d) -> usize {
        let r_small = f64::min(self.radius(), circle.radius());
        let r_big = f64::max(self.radius(), circle.radius());
        let dist = (self.center() - circle.center()).l2_norm();

        let score = r_small + r_big - dist;
        if score.abs() < EPS {
            // r + R = d
            3
        } else if score < 0.0 {
            // r + R < d
            4
        } else {
            // r + R > d
            let score = dist + r_small - r_big;
            if score.abs() < EPS {
                // r + d = R
                1
            } else if score < 0.0 {
                // r + d < R
                0
            } else {
                2
            }
        }
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
    fn num_common_tangents() {
        let circle1 = Circle2d::new(&Point2d::new(0.0, 0.0), 1.0);
        let circle2 = Circle2d::new(&Point2d::new(3.0, 0.0), 1.0);
        assert_eq!(circle1.num_common_tangents(&circle2), 4);
        assert_eq!(circle2.num_common_tangents(&circle1), 4);

        let circle1 = Circle2d::new(&Point2d::new(0.0, 0.0), 1.0);
        let circle2 = Circle2d::new(&Point2d::new(2.0, 0.0), 1.0);
        assert_eq!(circle1.num_common_tangents(&circle2), 3);
        assert_eq!(circle2.num_common_tangents(&circle1), 3);

        let circle1 = Circle2d::new(&Point2d::new(0.0, 0.0), 1.0);
        let circle2 = Circle2d::new(&Point2d::new(1.5, 0.0), 1.0);
        assert_eq!(circle1.num_common_tangents(&circle2), 2);
        assert_eq!(circle2.num_common_tangents(&circle1), 2);

        let circle1 = Circle2d::new(&Point2d::new(0.0, 0.0), 1.0);
        let circle2 = Circle2d::new(&Point2d::new(0.5, 0.0), 0.5);
        assert_eq!(circle1.num_common_tangents(&circle2), 1);
        assert_eq!(circle2.num_common_tangents(&circle1), 1);

        let circle1 = Circle2d::new(&Point2d::new(0.0, 0.0), 1.0);
        let circle2 = Circle2d::new(&Point2d::new(0.0, 0.0), 0.5);
        assert_eq!(circle1.num_common_tangents(&circle2), 0);
        assert_eq!(circle2.num_common_tangents(&circle1), 0);
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
