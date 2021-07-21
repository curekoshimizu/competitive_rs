// keywords :

use geometory2d::circle2d::Circle2d;
use geometory2d::point2d::Point2d;
use io_lib::*;

fn main() {
    input! {
        p: [f64; 6],
    }

    let c1 = Circle2d::new(&Point2d::new(p[0], p[1]), p[2]);
    let c2 = Circle2d::new(&Point2d::new(p[3], p[4]), p[5]);

    let result = c1.num_common_tangents(&c2);
    println!("{}", result);
}
