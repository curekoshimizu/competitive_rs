// keywords :

use geometory2d::point2d::Point2d;
use geometory2d::polygon2d::Polygon2d;
use io_lib::*;

fn main() {
    input! {
        p: [f64; 6],
    }

    let p1 = Point2d::new(p[0], p[1]);
    let p2 = Point2d::new(p[2], p[3]);
    let p3 = Point2d::new(p[4], p[5]);

    let poly = Polygon2d::new(vec![p1, p2, p3]);
    let circle = poly.circumscribed_circle_of_triangle();

    println!(
        "{} {} {}",
        &circle.center().x(),
        &circle.center().y(),
        &circle.radius()
    );
}
