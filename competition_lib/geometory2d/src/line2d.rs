use super::point2d::Point2d;

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
    pub fn iter(&self) -> impl Iterator<Item = (&Point2d, &Point2d)> {
        self.points.iter().zip(self.points.iter().skip(1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new() {
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
    fn iter() {
        for (start, end) in Lines2d::new(vec![
            Point2d::new(0.0, 0.0),
            Point2d::new(1.0, 1.1),
            Point2d::new(2.0, 2.2),
        ])
        .iter()
        {
            let vec = end - start;
            let [x, y] = vec.to_xy();
            assert_eq!(x, 1.0);
            assert_eq!(y, 1.1);
        }
    }
}
