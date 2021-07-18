// keywords :

use geometory2d::point2d::Point2d;
use io_lib::*;

fn main() {
    input! {
        p: [f64; 4],
        q: usize,
        points: [f64; q*2],
    }

    let p0 = Point2d::new(p[0], p[1]);
    let p1 = Point2d::new(p[2], p[3]);
    let v1 = p1 - p0;

    for i in 0..q {
        let x = points[2 * i];
        let y = points[2 * i + 1];
        let p2 = Point2d::new(x, y);

        let v2 = p2 - p0;

        let det = v1.det(&v2);

        let eps = 1.0e-5;

        if det.abs() > eps {
            if det > 0.0 {
                println!("COUNTER_CLOCKWISE");
            } else {
                println!("CLOCKWISE");
            }
        } else {
            if v1.dot(&v2) >= -eps {
                if v1.l2_norm() >= v2.l2_norm() {
                    println!("ON_SEGMENT");
                } else {
                    println!("ONLINE_FRONT");
                }
            } else {
                println!("ONLINE_BACK");
            }
        }
    }
}
