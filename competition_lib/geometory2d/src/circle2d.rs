use super::line2d::Line2d;
use super::line2d::EPS;
use super::point2d::Point2d;

#[derive(PartialEq, Debug)]
pub enum Contains {
    IN,
    OUT,
    ON,
}

#[derive(PartialEq, Debug)]
pub enum IntersectionPoint {
    EMPTY,
    ONE(Point2d),
    TWO(Point2d, Point2d),
}

#[derive(PartialEq, Debug, Clone, Copy)]
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
    pub fn contains(&self, p: &Point2d) -> Contains {
        let signed_dist = (self.center - p).l2_norm() - self.radius();
        if signed_dist.abs() < EPS {
            Contains::ON
        } else if signed_dist > 0.0 {
            Contains::OUT
        } else {
            Contains::IN
        }
    }
    pub fn distance_by_point(&self, p: &Point2d) -> f64 {
        ((self.center - p).l2_norm() - self.radius()).abs()
    }
    pub fn intersection_point_with_line(&self, line: &Line2d) -> IntersectionPoint {
        let p = line.project(&self.center());
        if self.contains(&p) == Contains::ON {
            IntersectionPoint::ONE(p)
        } else {
            let d = (p - self.center()).l2_norm();
            let ret = self.radius() - d;
            if ret > 0.0 {
                let len = (self.rad * self.rad - d * d).sqrt();
                let v = line.vector().unit_vector();
                IntersectionPoint::TWO(p + &v * len, p - &v * len)
            } else {
                IntersectionPoint::EMPTY
            }
        }
    }
    pub fn distance_by_line(&self, line: &Line2d) -> f64 {
        match self.intersection_point_with_line(line) {
            IntersectionPoint::EMPTY => line.distance_by_point(&self.center()) - self.radius(),
            IntersectionPoint::ONE(_) => 0.0,
            IntersectionPoint::TWO(_, _) => 0.0,
        }
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
            Contains::ON
        ));
        assert!(matches!(
            circle.contains(&Point2d::new(1.0, 3.0)),
            Contains::ON
        ));
        assert!(matches!(
            circle.contains(&Point2d::new(1.0, 2.0)),
            Contains::IN
        ));
        assert!(matches!(
            circle.contains(&Point2d::new(1.0, 3.1)),
            Contains::OUT
        ));
    }
    #[test]
    fn intersection_point_with_line() {
        let circle = Circle2d::new(&Point2d::new(1.0, 2.0), 1.0);
        let a = Point2d::new(0.0, 0.0);
        let b = Point2d::new(10.0, 0.0);
        assert!(matches!(
            circle.intersection_point_with_line(&Line2d::new(&a, &b)),
            IntersectionPoint::EMPTY
        ));
        let circle = Circle2d::new(&Point2d::new(1.0, 2.0), 1.0);
        let a = Point2d::new(0.0, 1.0);
        let b = Point2d::new(10.0, 1.0);
        assert!(matches!(
            circle.intersection_point_with_line(&Line2d::new(&a, &b)),
            IntersectionPoint::ONE(_)
        ));

        let a = Point2d::new(0.0, 1.1);
        let b = Point2d::new(10.0, 1.1);
        if let IntersectionPoint::TWO(p, q) =
            circle.intersection_point_with_line(&Line2d::new(&a, &b))
        {
            assert!(matches!(circle.contains(&p), Contains::ON));
            assert!(matches!(circle.contains(&q), Contains::ON));
            assert!((p - q).l2_norm() > 1.0e-3);
        } else {
            assert!(false);
        }

        let a = Point2d::new(0.0, 3.0);
        let b = Point2d::new(10.0, 3.0);
        assert!(matches!(
            circle.intersection_point_with_line(&Line2d::new(&a, &b)),
            IntersectionPoint::ONE(_)
        ));
        let a = Point2d::new(0.0, 4.0);
        let b = Point2d::new(10.0, 4.0);
        assert!(matches!(
            circle.intersection_point_with_line(&Line2d::new(&a, &b)),
            IntersectionPoint::EMPTY
        ));
    }
    #[test]
    fn distance_by_line() {
        let circle = Circle2d::new(&Point2d::new(1.0, 2.0), 1.0);
        let a = Point2d::new(0.0, 0.0);
        let b = Point2d::new(10.0, 0.0);
        assert_eq!(circle.distance_by_line(&Line2d::new(&a, &b)), 1.0);
        let a = Point2d::new(0.0, 1.0);
        let b = Point2d::new(10.0, 1.0);
        assert_eq!(circle.distance_by_line(&Line2d::new(&a, &b)), 0.0);
        let a = Point2d::new(0.0, 1.1);
        let b = Point2d::new(10.0, 1.1);
        assert_eq!(circle.distance_by_line(&Line2d::new(&a, &b)), 0.0);
        let a = Point2d::new(0.0, 3.0);
        let b = Point2d::new(10.0, 3.0);
        assert_eq!(circle.distance_by_line(&Line2d::new(&a, &b)), 0.0);
        let a = Point2d::new(0.0, 4.0);
        let b = Point2d::new(10.0, 4.0);
        assert_eq!(circle.distance_by_line(&Line2d::new(&a, &b)), 1.0);
    }
}
