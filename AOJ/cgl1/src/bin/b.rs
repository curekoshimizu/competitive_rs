// keywords :

use geometory2d::line2d::Line2d;
use geometory2d::point2d::Point2d;
use io_lib::*;

fn main() {
    input! {
        p1: [f64;2],
        p2: [f64;2],
        q: usize,
        p: [[f64; 2]; q],
    }

    let p1 = Point2d::new(p1[0], p1[1]);
    let p2 = Point2d::new(p2[0], p2[1]);

    let line = Line2d::new(&p1, &p2);

    for point in p {
        let x = point[0];
        let y = point[1];
        let p = Point2d::new(x, y);
        let ret = line.reflect(&p);
        println!("{} {}", ret.x(), ret.y());
    }
}
