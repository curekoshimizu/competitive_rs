use super::circle2d::Circle2d;
use super::line2d::{Line2d, Segment2d};
use super::point2d::Point2d;

pub enum Contains {
    IN,
    OUT,
    ON,
}

const EPS: f64 = 1.0e-5;

pub struct Polygon2d {
    pub vertices: Vec<Point2d>,
}

impl Polygon2d {
    pub fn new(vertices: Vec<Point2d>) -> Self {
        assert!(vertices.len() >= 3);
        Polygon2d { vertices }
    }
    pub fn n_gon(&self) -> usize {
        self.vertices.len()
    }
    pub fn signed_area(&self) -> f64 {
        self.edges()
            .map(|segment| segment.start().to_vector().det(&segment.end().to_vector()))
            .sum::<f64>()
            / 2.0
    }
    pub fn area(&self) -> f64 {
        self.signed_area().abs()
    }
    pub fn is_convex(&self) -> bool {
        let score = self
            .edge_pair_iter()
            .map(|(edge1, edge2)| {
                if edge1.vector().det(&edge2.vector()) >= 0.0 {
                    1
                } else {
                    -1
                }
            })
            .sum::<isize>()
            .abs();

        score == self.n_gon() as isize
    }
    /// O(n): Winding Number Algorithm
    pub fn contains(&self, p: &Point2d) -> Contains {
        let mut angle = 0.0;

        for edge in self.edges() {
            if edge.is_point_on_segment(p) {
                return Contains::ON;
            }

            let v1 = p - edge.start();
            let v2 = p - edge.end();
            angle += v1.angle(&v2);
        }

        if angle.abs() < EPS {
            Contains::OUT
        } else {
            Contains::IN
        }
    }
    pub fn edges(&self) -> impl Iterator<Item = Segment2d> {
        self.vertices
            .iter()
            .zip(
                self.vertices
                    .iter()
                    .skip(1)
                    .chain(self.vertices[0..1].iter()),
            )
            .map(|(start, end)| Segment2d::new(start, end))
    }
    pub fn edge_pair_iter(&self) -> impl Iterator<Item = (Segment2d, Segment2d)> {
        self.edges()
            .zip(self.edges().skip(1).chain(self.edges().take(1)))
    }
    pub fn incircle_of_triangle(&self) -> Circle2d {
        assert_eq!(self.n_gon(), 3);

        let a = (self.vertices[2] - self.vertices[1]).l2_norm();
        let b = (self.vertices[0] - self.vertices[2]).l2_norm();
        let c = (self.vertices[1] - self.vertices[0]).l2_norm();
        let sum = a + b + c;

        let center =
            a * &self.vertices[0] / sum + b * &self.vertices[1] / sum + c * &self.vertices[2] / sum;

        let line = Line2d::new(&self.vertices[0], &self.vertices[1]);
        let radius = line.distance_by_point(&center);

        Circle2d::new(&center, radius)
    }
    pub fn circumscribed_circle_of_triangle(&self) -> Circle2d {
        assert_eq!(self.n_gon(), 3);

        let line = Line2d::new(&self.vertices[0], &self.vertices[1]);
        let start = (self.vertices[0] + self.vertices[1]) / 2.0;
        let end = start + line.vector().rotate90();
        let bisector1 = Line2d::new(&start, &end);

        let line = Line2d::new(&self.vertices[1], &self.vertices[2]);
        let start = (self.vertices[1] + self.vertices[2]) / 2.0;
        let end = start + line.vector().rotate90();
        let bisector2 = Line2d::new(&start, &end);

        let center = bisector1.intersection_point_with_line(&bisector2).unwrap();
        let radius = (center - self.vertices[0]).l2_norm();

        Circle2d::new(&center, radius)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn triangle() {
        let triangle = Polygon2d::new(vec![
            Point2d::new(0.0, 0.0),
            Point2d::new(1.0, 0.0),
            Point2d::new(1.0, 1.0),
        ]);
        assert_eq!(triangle.n_gon(), 3);
        assert_eq!(triangle.area(), 0.5);

        let mut edges = triangle.edges();
        let edge = edges.next().unwrap();
        assert_eq!(edge.start(), &Point2d::new(0.0, 0.0));
        assert_eq!(edge.end(), &Point2d::new(1.0, 0.0));

        let edge = edges.next().unwrap();
        assert_eq!(edge.start(), &Point2d::new(1.0, 0.0));
        assert_eq!(edge.end(), &Point2d::new(1.0, 1.0));

        let edge = edges.next().unwrap();
        assert_eq!(edge.start(), &Point2d::new(1.0, 1.0));
        assert_eq!(edge.end(), &Point2d::new(0.0, 0.0));

        assert_eq!(edges.next(), None);
        assert!(triangle.is_convex());
    }

    #[test]
    fn square() {
        let p1 = Point2d::new(0.0, 0.0);
        let p2 = Point2d::new(1.0, 0.0);
        let p3 = Point2d::new(1.0, 1.0);
        let p4 = Point2d::new(0.0, 1.0);
        let square = Polygon2d::new(vec![p1, p2, p3, p4]);
        assert_eq!(square.n_gon(), 4);
        assert_eq!(square.signed_area(), 1.0);
        assert_eq!(square.area(), 1.0);
        assert!(square.is_convex());

        let s = Point2d::new(10.0, 20.0);
        let square = Polygon2d::new(vec![p1 + s, p2 + s, p3 + s, p4 + s]);
        assert_eq!(square.n_gon(), 4);
        assert_eq!(square.signed_area(), 1.0);
        assert_eq!(square.area(), 1.0);
        assert!(square.is_convex());

        let square = Polygon2d::new(vec![p1, p4, p3, p2]);
        assert_eq!(square.n_gon(), 4);
        assert_eq!(square.signed_area(), -1.0);
        assert_eq!(square.area(), 1.0);
        assert!(square.is_convex());
    }

    #[test]
    fn convex_pentagon() {
        let p1 = Point2d::new(0.0, 0.0);
        let p2 = Point2d::new(1.0, 0.0);
        let p3 = Point2d::new(1.0, 1.0);
        let p4 = Point2d::new(0.5, 2.0);
        let p5 = Point2d::new(0.0, 1.0);
        let pentagon = Polygon2d::new(vec![p1, p2, p3, p4, p5]);
        assert_eq!(pentagon.n_gon(), 5);
        assert_eq!(pentagon.area(), 1.5);
        assert!(pentagon.is_convex());

        let s = Point2d::new(10.0, 20.0);
        let pentagon = Polygon2d::new(vec![p1 + s, p2 + s, p3 + s, p4 + s, p5 + s]);
        assert_eq!(pentagon.n_gon(), 5);
        assert_eq!(pentagon.signed_area(), 1.5);
        assert_eq!(pentagon.area(), 1.5);
        assert!(pentagon.is_convex());

        let pentagon = Polygon2d::new(vec![p1, p5, p4, p3, p2]);
        assert_eq!(pentagon.n_gon(), 5);
        assert_eq!(pentagon.signed_area(), -1.5);
        assert_eq!(pentagon.area(), 1.5);
        assert!(pentagon.is_convex());
    }
    #[test]
    fn non_convex_pentagon() {
        let p1 = Point2d::new(0.0, 0.0);
        let p2 = Point2d::new(1.0, 0.0);
        let p3 = Point2d::new(1.0, 1.0);
        let p4 = Point2d::new(0.5, 0.5);
        let p5 = Point2d::new(0.0, 1.0);
        let pentagon = Polygon2d::new(vec![p1, p2, p3, p4, p5]);
        assert_eq!(pentagon.n_gon(), 5);
        assert_eq!(pentagon.area(), 0.75);
        assert!(!pentagon.is_convex());
        assert!(matches!(
            pentagon.contains(&Point2d::new(0.1, 0.0)),
            Contains::ON
        ));
        assert!(matches!(
            pentagon.contains(&Point2d::new(1.0, 0.9)),
            Contains::ON
        ));
        assert!(matches!(
            pentagon.contains(&Point2d::new(1.0, 1.1)),
            Contains::OUT
        ));
        assert!(matches!(
            pentagon.contains(&Point2d::new(0.5, 0.4)),
            Contains::IN
        ));
        assert!(matches!(
            pentagon.contains(&Point2d::new(0.5, 0.6)),
            Contains::OUT
        ));

        let s = Point2d::new(10.0, 20.0);
        let pentagon = Polygon2d::new(vec![p1 + s, p2 + s, p3 + s, p4 + s, p5 + s]);
        assert_eq!(pentagon.n_gon(), 5);
        assert_eq!(pentagon.signed_area(), 0.75);
        assert_eq!(pentagon.area(), 0.75);
        assert!(!pentagon.is_convex());

        assert!(matches!(
            pentagon.contains(&(Point2d::new(0.1, 0.0) + s)),
            Contains::ON
        ));
        assert!(matches!(
            pentagon.contains(&(Point2d::new(1.0, 0.9) + s)),
            Contains::ON
        ));
        assert!(matches!(
            pentagon.contains(&(Point2d::new(1.0, 1.1) + s)),
            Contains::OUT
        ));
        assert!(matches!(
            pentagon.contains(&(Point2d::new(0.5, 0.4) + s)),
            Contains::IN
        ));
        assert!(matches!(
            pentagon.contains(&(Point2d::new(0.5, 0.6) + s)),
            Contains::OUT
        ));

        let pentagon = Polygon2d::new(vec![p1, p5, p4, p3, p2]);
        assert_eq!(pentagon.n_gon(), 5);
        assert_eq!(pentagon.signed_area(), -0.75);
        assert_eq!(pentagon.area(), 0.75);
        assert!(!pentagon.is_convex());
        assert!(matches!(
            pentagon.contains(&Point2d::new(0.1, 0.0)),
            Contains::ON
        ));
        assert!(matches!(
            pentagon.contains(&Point2d::new(1.0, 0.9)),
            Contains::ON
        ));
        assert!(matches!(
            pentagon.contains(&Point2d::new(1.0, 1.1)),
            Contains::OUT
        ));
        assert!(matches!(
            pentagon.contains(&Point2d::new(0.5, 0.4)),
            Contains::IN
        ));
        assert!(matches!(
            pentagon.contains(&Point2d::new(0.5, 0.6)),
            Contains::OUT
        ));
    }
    #[test]
    fn contains() {
        let pentagon = Polygon2d::new(vec![
            Point2d::new(0.0, 0.0),
            Point2d::new(3.0, 1.0),
            Point2d::new(2.0, 3.0),
            Point2d::new(0.0, 3.0),
        ]);
        assert!(matches!(
            pentagon.contains(&Point2d::new(2.0, 1.0)),
            Contains::IN
        ));
        assert!(matches!(
            pentagon.contains(&Point2d::new(0.0, 2.0)),
            Contains::ON
        ));
        assert!(matches!(
            pentagon.contains(&Point2d::new(3.0, 2.0)),
            Contains::OUT
        ));
    }
    #[test]
    fn incircle_of_triangle() {
        const EPS: f64 = 1.0e-5;

        let a = Point2d::new(1.0, 2.0);
        let b = Point2d::new(10.0, -2.0);
        let c = Point2d::new(7.0, 4.0);
        let triangle = Polygon2d::new(vec![a, b, c]);
        let circle = triangle.incircle_of_triangle();
        assert!(
            (Line2d::new(&a, &b).distance_by_point(&circle.center()) - circle.radius()).abs() < EPS
        );
        assert!(
            (Line2d::new(&b, &c).distance_by_point(&circle.center()) - circle.radius()).abs() < EPS
        );
        assert!(
            (Line2d::new(&c, &a).distance_by_point(&circle.center()) - circle.radius()).abs() < EPS
        );

        let v1_unit = (b - a).unit_vector();
        let v2_unit = (c - a).unit_vector();
        let end = a + v1_unit + v2_unit;
        let line = Line2d::new(&a, &end); // bisector of an angle
        assert!(line.is_point_on_line(&circle.center()));

        let v1_unit = (c - b).unit_vector();
        let v2_unit = (a - b).unit_vector();
        let end = b + v1_unit + v2_unit;
        let line = Line2d::new(&b, &end); // bisector of an angle
        assert!(line.is_point_on_line(&circle.center()));

        let v1_unit = (a - c).unit_vector();
        let v2_unit = (b - c).unit_vector();
        let end = c + v1_unit + v2_unit;
        let line = Line2d::new(&c, &end); // bisector of an angle
        assert!(line.is_point_on_line(&circle.center()));

        let a = Point2d::new(1.0, 2.0);
        let b = Point2d::new(-3.0, 5.0);
        let c = Point2d::new(0.0, -2.0);
        let triangle = Polygon2d::new(vec![a, b, c]);
        let circle = triangle.incircle_of_triangle();
        assert!(
            (Line2d::new(&a, &b).distance_by_point(&circle.center()) - circle.radius()).abs() < EPS
        );
        assert!(
            (Line2d::new(&b, &c).distance_by_point(&circle.center()) - circle.radius()).abs() < EPS
        );
        assert!(
            (Line2d::new(&c, &a).distance_by_point(&circle.center()) - circle.radius()).abs() < EPS
        );
    }
    #[test]
    fn circumscribed_circle_of_triangle() {
        let a = Point2d::new(1.0, 2.0);
        let b = Point2d::new(10.0, -2.0);
        let c = Point2d::new(7.0, 4.0);
        let triangle = Polygon2d::new(vec![a, b, c]);
        let circle = triangle.circumscribed_circle_of_triangle();
        assert!(matches!(
            circle.contains(&a),
            super::super::circle2d::Contains::ON
        ));
        assert!(matches!(
            circle.contains(&b),
            super::super::circle2d::Contains::ON
        ));
        assert!(matches!(
            circle.contains(&c),
            super::super::circle2d::Contains::ON
        ));

        let a = Point2d::new(1.0, 2.0);
        let b = Point2d::new(-3.0, 5.0);
        let c = Point2d::new(0.0, -2.0);
        let triangle = Polygon2d::new(vec![a, b, c]);
        let circle = triangle.circumscribed_circle_of_triangle();
        assert!(matches!(
            circle.contains(&a),
            super::super::circle2d::Contains::ON
        ));
        assert!(matches!(
            circle.contains(&b),
            super::super::circle2d::Contains::ON
        ));
        assert!(matches!(
            circle.contains(&c),
            super::super::circle2d::Contains::ON
        ));
    }
}
