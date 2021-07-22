// keywords :

use geometory2d::circle2d::{Circle2d, IntersectionPoint};
use geometory2d::point2d::Point2d;
use io_lib::*;

fn main() {
    input! {
        p: [f64; 6],
    }

    let circle1 = Circle2d::new(&Point2d::new(p[0], p[1]), p[2]);
    let circle2 = Circle2d::new(&Point2d::new(p[3], p[4]), p[5]);

    match circle1.intersection_point_with_circle(&circle2) {
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
