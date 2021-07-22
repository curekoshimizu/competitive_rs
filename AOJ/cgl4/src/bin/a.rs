// keywords :

use geometory2d::point2d::Point2d;
use geometory2d::polygon2d::Polygon2d;
use io_lib::*;

fn main() {
    input! {
        n: usize,
        p: [(f64, f64); n],
    }
    let vec = p
        .into_iter()
        .map(|(x, y)| Point2d::new(x, y))
        .collect::<Vec<Point2d>>();

    let mut polygon2d = Polygon2d::new(vec);

    polygon2d.convex_full();

    let (base, _) = polygon2d
        .vertices
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| (a.y(), a.x()).partial_cmp(&(b.y(), b.x())).unwrap())
        .unwrap();

    let n = polygon2d.n_gon();
    println!("{}", n);
    for i in 0..n {
        let index = (i + base) % n;
        let p = polygon2d.vertices[index];
        println!("{} {}", p.x(), p.y());
    }
}
