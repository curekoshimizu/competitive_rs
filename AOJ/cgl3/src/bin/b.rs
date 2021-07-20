// keywords :

use geometory2d::point2d::Point2d;
use geometory2d::polygon2d::Polygon2d;
use io_lib::*;

fn main() {
    input! {
        n: usize,
        p: [f64; 2*n],
    }

    let polygon = Polygon2d::new(
        (0..n)
            .map(|i| Point2d::new(p[2 * i], p[2 * i + 1]))
            .collect::<Vec<_>>(),
    );
    if polygon.is_convex() {
        println!("1");
    } else {
        println!("0");
    }
}
