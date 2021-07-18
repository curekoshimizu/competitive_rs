// keywords :

use geometory2d::line2d::Line2d;
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

        let line1 = Line2d::new(&p0, &p1);
        let line2 = Line2d::new(&p2, &p3);

        if line1.is_intersection_point_on_both_segment(&line2) {
            println!("1");
        } else {
            println!("0");
        }
    }
}
