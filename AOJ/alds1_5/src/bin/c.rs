// keywords :

use geometory2d::line2d::SegmentPath2d;
use geometory2d::point2d::Point2d;
use io_lib::*;

fn main() {
    input! {
        n: usize,
    }

    let x = Point2d::origin();
    let y = Point2d::new(100.0, 0.0);

    let mut lines = SegmentPath2d::new(vec![x, y]);

    for _ in 0..n {
        lines = gen(lines);
    }

    for point in lines.points {
        let [x, y] = point.to_xy();
        println!("{} {}", x, y);
    }
}

fn gen(lines: SegmentPath2d) -> SegmentPath2d {
    let mut points: Vec<Point2d> = vec![];

    for line in lines.iter() {
        let start = line.start();
        let end = line.end();

        let p1 = (2.0 * start + end) / 3.0;
        let p2 = (start + 2.0 * end) / 3.0;

        let v = (p2 - p1).rotate_by_deg(60.0);
        let p = p1 + v;

        points.push(*start);
        points.push(p1);
        points.push(p);
        points.push(p2);
    }
    points.push(Point2d::new(100.0, 0.0));

    SegmentPath2d::new(points.into())
}
