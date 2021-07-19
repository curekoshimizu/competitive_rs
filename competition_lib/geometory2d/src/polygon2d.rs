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
}
