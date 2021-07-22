// keywords :

use geometory2d::circle2d::{Circle2d, IntersectionPoint};
use geometory2d::line2d::Line2d;
use geometory2d::point2d::Point2d;
use io_lib::*;

fn main() {
    input! {
        p: [f64; 3],
        n: usize,
        q: [f64; 4*n],
    }

    let circle = Circle2d::new(&Point2d::new(p[0], p[1]), p[2]);

    for i in 0..n {
        let p1 = Point2d::new(q[4 * i + 0], q[4 * i + 1]);
        let p2 = Point2d::new(q[4 * i + 2], q[4 * i + 3]);
        let line = Line2d::new(&p1, &p2);

        match circle.intersection_point_with_line(&line) {
            IntersectionPoint::TWO(a, b) => {
                if (a.x() - b.x()).abs() < 1.0e-5 {
                    if a.y() < b.y() {
                        println!("{} {} {} {}", a.x(), a.y(), b.x(), b.y());
                    } else {
                        println!("{} {} {} {}", b.x(), b.y(), a.x(), a.y());
                    }
                } else if a.x() < b.x() {
                    println!("{} {} {} {}", a.x(), a.y(), b.x(), b.y());
                } else {
                    println!("{} {} {} {}", b.x(), b.y(), a.x(), a.y());
                }
            }
            IntersectionPoint::ONE(a) => {
                println!("{} {} {} {}", a.x(), a.y(), a.x(), a.y());
            }
            IntersectionPoint::EMPTY => {
                panic!("!!!!")
            }
        }
    }
}
