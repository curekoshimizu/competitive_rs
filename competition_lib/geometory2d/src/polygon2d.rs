use super::line2d::Segment2d;
use super::point2d::Point2d;

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
    pub fn area(&self) -> f64 {
        let signed_area = self
            .edges()
            .map(|segment| segment.start().to_vector().det(&segment.end().to_vector()))
            .sum::<f64>()
            / 2.0;

        signed_area.abs()
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
    }

    #[test]
    fn square() {
        let p1 = Point2d::new(0.0, 0.0);
        let p2 = Point2d::new(1.0, 0.0);
        let p3 = Point2d::new(1.0, 1.0);
        let p4 = Point2d::new(0.0, 1.0);
        let square = Polygon2d::new(vec![p1, p2, p3, p4]);
        assert_eq!(square.n_gon(), 4);
        assert_eq!(square.area(), 1.0);

        let s = Point2d::new(10.0, 20.0);
        let square = Polygon2d::new(vec![p1 + s, p2 + s, p3 + s, p4 + s]);
        assert_eq!(square.n_gon(), 4);
        assert_eq!(square.area(), 1.0);

        let square = Polygon2d::new(vec![p1, p4, p3, p2]);
        assert_eq!(square.n_gon(), 4);
        assert_eq!(square.area(), 1.0);
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

        let s = Point2d::new(10.0, 20.0);
        let square = Polygon2d::new(vec![p1 + s, p2 + s, p3 + s, p4 + s, p5 + s]);
        assert_eq!(square.n_gon(), 5);
        assert_eq!(square.area(), 1.5);

        let pentagon = Polygon2d::new(vec![p1, p5, p4, p3, p2]);
        assert_eq!(pentagon.n_gon(), 5);
        assert_eq!(pentagon.area(), 1.5);
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

        let s = Point2d::new(10.0, 20.0);
        let square = Polygon2d::new(vec![p1 + s, p2 + s, p3 + s, p4 + s, p5 + s]);
        assert_eq!(square.n_gon(), 5);
        assert_eq!(pentagon.area(), 0.75);

        let pentagon = Polygon2d::new(vec![p1, p5, p4, p3, p2]);
        assert_eq!(pentagon.n_gon(), 5);
        assert_eq!(pentagon.area(), 0.75);
    }
}
