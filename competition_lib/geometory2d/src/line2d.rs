use super::point2d::Point2d;
use super::vec2d::Vec2d;

const EPS: f64 = 1.0e-5;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Line2d<'a> {
    start: &'a Point2d,
    end: &'a Point2d,
}

impl<'a> Line2d<'a> {
    pub fn new(start: &'a Point2d, end: &'a Point2d) -> Self {
        Line2d { start, end }
    }
    pub fn start(&self) -> &'a Point2d {
        self.start
    }
    pub fn end(&self) -> &'a Point2d {
        self.end
    }
    pub fn vector(&self) -> Vec2d {
        self.end - self.start
    }
    pub fn project(&self, p: &Point2d) -> Point2d {
        let a = p - self.start;
        let v = self.vector().unit_vector();
        // |a| cos = (v, a) / |v|
        let a_cos = v.dot(&a);
        self.start + v * a_cos
    }
    pub fn reflect(&self, p: &Point2d) -> Point2d {
        let mid = self.project(p);
        let v = (mid - p) * 2.0;
        p + v
    }
    pub fn is_on_line(&self, p: &Point2d) -> bool {
        (p - self.start).det(&(self.end - p)).abs() < EPS
    }
    /// check whether p is on line and between start point and end point
    /// including point start and ponit end.
    pub fn is_on_segment(&self, p: &Point2d) -> bool {
        if !self.is_on_line(p) {
            false
        } else {
            (p - self.start).dot(&(self.end - p)) > -EPS
        }
    }
    pub fn intersection_point(&self, line: &Line2d) -> Option<Point2d> {
        let vec1 = self.vector();
        let vec2 = line.vector();

        let b = vec1.det(&(self.start - line.start));
        let a = vec1.det(&vec2);

        let t = b / a;

        if !t.is_normal() {
            if self.is_on_segment(&line.start) {
                Some(*line.start)
            } else if self.is_on_segment(&line.end) {
                Some(*line.end)
            } else if line.is_on_segment(&self.start) {
                Some(*self.start)
            } else if line.is_on_segment(&self.end) {
                Some(*self.end)
            } else {
                None
            }
        } else {
            Some(line.start + t * vec2)
        }
    }
    pub fn is_intersection_point_on_both_segment(&self, line: &Line2d) -> bool {
        if let Some(p) = self.intersection_point(line) {
            self.is_on_segment(&p) && line.is_on_segment(&p)
        } else {
            false
        }
    }
}

pub struct Lines2d {
    pub points: Vec<Point2d>,
}

impl Lines2d {
    pub fn new(points: Vec<Point2d>) -> Self {
        Lines2d { points }
    }
    pub fn num_lines(&self) -> usize {
        let len = self.points.len();
        match len {
            0 => 0,
            1 => 0,
            i => i - 1,
        }
    }
    // TODO: understand fully why we need "impl"
    pub fn iter(&self) -> impl Iterator<Item = Line2d> {
        self.points
            .iter()
            .zip(self.points.iter().skip(1))
            .map(|(start, end)| Line2d::new(start, end))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn lines_new() {
        let lines = Lines2d::new(vec![
            Point2d::new(0.0, 0.0),
            Point2d::new(1.0, 1.1),
            Point2d::new(2.0, 2.2),
        ]);
        assert_eq!(lines.num_lines(), 2);

        let lines = Lines2d::new(vec![Point2d::new(0.0, 0.0), Point2d::new(1.0, 1.1)]);
        assert_eq!(lines.num_lines(), 1);

        let lines = Lines2d::new(vec![Point2d::new(0.0, 0.0)]);
        assert_eq!(lines.num_lines(), 0);

        let lines = Lines2d::new(vec![]);
        assert_eq!(lines.num_lines(), 0);
    }
    #[test]
    fn point_projection() {
        let a = Point2d::new(0.0, 0.0);
        let b = Point2d::new(3.0, 4.0);
        let line = Line2d::new(&a, &b);
        assert_eq!(
            line.project(&Point2d::new(2.0, 5.0)),
            Point2d::new(3.12, 4.16)
        );
        let line = Line2d::new(&b, &a);
        assert_eq!(
            line.project(&Point2d::new(2.0, 5.0)),
            Point2d::new(3.12, 4.16)
        );

        let v = Vec2d::new(1.0, 3.0);
        let a = a + v;
        let b = b + v;

        let line = Line2d::new(&a, &b);
        assert_eq!(
            line.project(&(Point2d::new(2.0, 5.0) + v)),
            Point2d::new(3.12, 4.16) + v
        );
        let line = Line2d::new(&b, &a);
        assert_eq!(
            line.project(&(Point2d::new(2.0, 5.0) + v)),
            Point2d::new(3.12, 4.16) + v
        );
    }
    #[test]
    fn point_reflection() {
        let a = Point2d::new(0.0, 0.0);
        let b = Point2d::new(3.0, 4.0);
        let line = Line2d::new(&a, &b);
        assert!(
            (line.reflect(&Point2d::new(2.0, 5.0)) - Point2d::new(4.24, 3.32)).is_almost_zero()
        );
        let line = Line2d::new(&b, &a);
        assert!(
            (line.reflect(&Point2d::new(2.0, 5.0)) - Point2d::new(4.24, 3.32)).is_almost_zero()
        );

        let v = Vec2d::new(1.0, 3.0);
        let a = a + v;
        let b = b + v;

        let line = Line2d::new(&a, &b);
        assert!(
            (line.reflect(&(Point2d::new(2.0, 5.0) + v)) - Point2d::new(4.24, 3.32) - v)
                .is_almost_zero()
        );

        let line = Line2d::new(&b, &a);
        assert!(
            (line.reflect(&(Point2d::new(2.0, 5.0) + v)) - Point2d::new(4.24, 3.32) - v)
                .is_almost_zero()
        );
    }
    #[test]
    fn on_line() {
        let a = Point2d::new(0.0, 0.0);
        let b = Point2d::new(1.0, 0.0);
        let line = Line2d::new(&a, &b);
        assert!(line.is_on_line(&Point2d::new(2.0, 0.0)));
        assert!(line.is_on_line(&Point2d::new(1.0, 0.0)));
        assert!(line.is_on_line(&Point2d::new(0.5, 0.0)));
        assert!(line.is_on_line(&Point2d::new(0.0, 0.0)));
        assert!(line.is_on_line(&Point2d::new(-1.0, 0.0)));
        assert!(!line.is_on_line(&Point2d::new(1.0, 0.1)));
        assert!(!line.is_on_line(&Point2d::new(0.5, 0.1)));
        assert!(!line.is_on_line(&Point2d::new(0.0, 0.1)));
        assert!(!line.is_on_line(&Point2d::new(1.0, -0.1)));
        assert!(!line.is_on_line(&Point2d::new(0.5, -0.1)));
        assert!(!line.is_on_line(&Point2d::new(0.0, -0.1)));
    }
    #[test]
    fn on_segment() {
        let a = Point2d::new(0.0, 0.0);
        let b = Point2d::new(1.0, 0.0);
        let line = Line2d::new(&a, &b);
        assert!(!line.is_on_segment(&Point2d::new(2.0, 0.0)));
        assert!(line.is_on_segment(&Point2d::new(1.0, 0.0)));
        assert!(line.is_on_segment(&Point2d::new(0.5, 0.0)));
        assert!(line.is_on_segment(&Point2d::new(0.0, 0.0)));
        assert!(!line.is_on_segment(&Point2d::new(-1.0, 0.0)));
        assert!(!line.is_on_segment(&Point2d::new(1.0, 0.1)));
        assert!(!line.is_on_segment(&Point2d::new(0.5, 0.1)));
        assert!(!line.is_on_segment(&Point2d::new(0.0, 0.1)));
        assert!(!line.is_on_segment(&Point2d::new(1.0, -0.1)));
        assert!(!line.is_on_segment(&Point2d::new(0.5, -0.1)));
        assert!(!line.is_on_segment(&Point2d::new(0.0, -0.1)));
    }
    #[test]
    fn intersection_point() {
        let a = Point2d::new(0.0, 0.0);
        let b = Point2d::new(1.0, 0.0);
        let line = Line2d::new(&a, &b);

        for x in vec![-1.0, 0.0, 0.5, 1.0, 2.0] {
            let c = Point2d::new(x, 1.0);
            let d = Point2d::new(x, -1.0);
            let line2 = Line2d::new(&c, &d);

            let p = line.intersection_point(&line2).unwrap();
            assert!((x - p.x()).abs() < 1.0e-5);
            assert!(p.y().abs() < 1.0e-5);
            assert!(line.is_on_line(&p));
            assert!(line2.is_on_line(&p));
            if 0.0 <= x && x <= 1.0 {
                assert!(line.is_on_segment(&p));
                assert!(line2.is_on_segment(&p));
            }

            let p = line2.intersection_point(&line).unwrap();
            assert!((x - p.x()).abs() < 1.0e-5);
            assert!(p.y().abs() < 1.0e-5);
            assert!(line.is_on_line(&p));
            assert!(line2.is_on_line(&p));
            if 0.0 <= x && x <= 1.0 {
                assert!(line.is_on_segment(&p));
                assert!(line2.is_on_segment(&p));
            }
        }
    }
    #[test]
    fn intersection_parallel_case() {
        let a = Point2d::new(0.0, 0.0);
        let b = Point2d::new(1.0, 0.0);
        let c = Point2d::new(0.0, 0.2);
        let d = Point2d::new(1.0, 0.2);
        let line = Line2d::new(&a, &b);
        let line2 = Line2d::new(&c, &d);
        assert_eq!(line.intersection_point(&line2), None);
        assert_eq!(line2.intersection_point(&line), None);

        let a = Point2d::new(0.0, 0.0);
        let b = Point2d::new(0.0, 1.0);
        let c = Point2d::new(0.0, 2.0);
        let line = Line2d::new(&a, &b);
        let line2 = Line2d::new(&b, &c);
        assert_eq!(line.intersection_point(&line2), Some(b));
        assert_eq!(line.intersection_point(&line2), Some(b));

        let a = Point2d::new(0.0, 0.0);
        let b = Point2d::new(1.0, 0.0);
        let c = Point2d::new(-1.0, 0.0);
        let d = Point2d::new(10.0, 0.0);
        let line = Line2d::new(&a, &b);
        let line2 = Line2d::new(&c, &d);
        let p = line.intersection_point(&line2).unwrap();
        assert!(line.is_on_segment(&p));
        assert!(line2.is_on_segment(&p));

        let p = line2.intersection_point(&line).unwrap();
        assert!(line.is_on_segment(&p));
        assert!(line2.is_on_segment(&p));
    }
    #[test]
    fn intersect_on_segment() {
        let a = Point2d::new(0.0, 0.0);
        let b = Point2d::new(1.0, 0.0);
        let line = Line2d::new(&a, &b);

        for x in vec![-1.0, 0.0, 0.5, 1.0, 2.0] {
            let c = Point2d::new(x, 1.0);
            let d = Point2d::new(x, -1.0);
            let line2 = Line2d::new(&c, &d);

            assert_eq!(
                line.is_intersection_point_on_both_segment(&line2),
                0.0 <= x && x <= 1.0
            );
            assert_eq!(
                line2.is_intersection_point_on_both_segment(&line),
                0.0 <= x && x <= 1.0
            );
        }
    }
    #[test]
    fn lines_iter() {
        for line in Lines2d::new(vec![
            Point2d::new(0.0, 0.0),
            Point2d::new(1.0, 1.1),
            Point2d::new(2.0, 2.2),
        ])
        .iter()
        {
            let vec = line.vector();
            let [x, y] = vec.to_xy();
            assert_eq!(x, 1.0);
            assert_eq!(y, 1.1);
        }
    }
}
