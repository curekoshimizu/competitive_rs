// keywords :

use geometory2d::point2d::Point2d;
use io_lib::*;

fn main() {
    input! {
        q: usize,
        p: [f64; 8* q],
    }

    for i in 0..q {
        let base = 8 * i;
        let p0 = Point2d::new(p[0 + base], p[1 + base]);
        let p1 = Point2d::new(p[2 + base], p[3 + base]);
        let p2 = Point2d::new(p[4 + base], p[5 + base]);
        let p3 = Point2d::new(p[6 + base], p[7 + base]);

        let v1 = p1 - p0;
        let v2 = p3 - p2;

        let eps = 1.0e-5;

        if v1.dot(&v2).abs() < eps {
            println!("1");
        } else if v1.det(&v2).abs() < eps {
            println!("2");
        } else {
            println!("0");
        }
    }
}
