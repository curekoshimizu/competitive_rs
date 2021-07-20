// keywords :

use geometory2d::point2d::Point2d;
use geometory2d::polygon2d::{Polygon2d, CONTAINS};
use io_lib::*;

fn main() {
    input! {
        n: usize,
        p: [f64; 2*n],
        m: usize,
        q: [f64; 2*m],
    }

    let polygon = Polygon2d::new(
        (0..n)
            .map(|i| Point2d::new(p[2 * i], p[2 * i + 1]))
            .collect::<Vec<_>>(),
    );

    for i in 0..m {
        let p = Point2d::new(q[2 * i], q[2 * i + 1]);
        let ret = match polygon.contains(&p) {
            CONTAINS::IN => 2,
            CONTAINS::ON => 1,
            CONTAINS::OUT => 0,
        };
        println!("{}", ret);
    }
}
