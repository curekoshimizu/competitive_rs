use super::vec2d::Vec2d;
use std::ops::{Add, Div, Mul, Neg, Sub};
use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Point2d(Vec2d);

impl Point2d {
    pub fn new(x: f64, y: f64) -> Self {
        Point2d(Vec2d::new(x, y))
    }
    pub fn x(&self) -> f64 {
        self.0.x
    }
    pub fn y(&self) -> f64 {
        self.0.y
    }
    pub fn origin() -> Self {
        Point2d(Vec2d::origin())
    }
    pub fn is_almost_zero(&self) -> bool {
        self.0.is_almost_zero()
    }
    pub fn rotate90(&self) -> Point2d {
        Point2d(self.0.rotate90())
    }
    pub fn rotate_by_deg(&self, deg: f64) -> Point2d {
        Point2d(self.0.rotate_by_deg(deg))
    }
    pub fn rotate_by_rad(&self, rad: f64) -> Point2d {
        Point2d(self.0.rotate_by_rad(rad))
    }
    pub fn l2_norm(&self) -> f64 {
        self.0.l2_norm()
    }
    pub fn l2_norm_squared(&self) -> f64 {
        self.0.l2_norm_squared()
    }
    pub fn add(&self, rhs: &Vec2d) -> Point2d {
        Point2d(self.0 + rhs)
    }
    pub fn sub(&self, rhs: &Vec2d) -> Point2d {
        Point2d(self.0 - rhs)
    }
    pub fn mul(&self, rhs: &Vec2d) -> Point2d {
        Point2d(self.0 * rhs)
    }
    pub fn div(&self, rhs: &Vec2d) -> Point2d {
        Point2d(self.0 / rhs)
    }
    pub fn to_xy(&self) -> [f64; 2] {
        [self.0.x, self.0.y]
    }
}

macro_rules! implement_binop {
    ($imp:ident, $method:ident, $term:tt) => {
        impl $imp<Vec2d> for Point2d {
            type Output = Point2d;
            fn $method(self, rhs: Vec2d) -> Self::Output {
                <Point2d>::$method(&self, &rhs)
            }
        }
        impl<'a> $imp<&'a Vec2d> for Point2d {
            type Output = Point2d;
            fn $method(self, rhs: &Vec2d) -> Self::Output {
                <Point2d>::$method(&self, rhs)
            }
        }
        impl<'a> $imp<Vec2d> for &'a Point2d {
            type Output = Point2d;
            fn $method(self, rhs: Vec2d) -> Self::Output {
                <Point2d>::$method(self, &rhs)
            }
        }
        impl<'a, 'b> $imp<&'b Vec2d> for &'a Point2d {
            type Output = Point2d;
            fn $method(self, rhs: &'b Vec2d) -> Point2d {
                <Point2d>::$method(self, rhs)
            }
        }

        impl $imp<f64> for Point2d {
            type Output = Point2d;
            fn $method(self, rhs: f64) -> Self::Output {
                Point2d(self.0 $term rhs)
            }
        }
        impl<'a> $imp<f64> for &'a Point2d {
            type Output = Point2d;
            fn $method(self, rhs: f64) -> Self::Output {
                Point2d(self.0 $term rhs)
            }
        }
        impl $imp<Point2d> for f64 {
            type Output = Point2d;
            fn $method(self, rhs: Point2d) -> Self::Output {
                Point2d(self $term rhs.0)
            }
        }
        impl<'a> $imp<&'a Point2d> for f64 {
            type Output = Point2d;
            fn $method(self, rhs: &Point2d) -> Self::Output {
                Point2d(self $term rhs.0)
            }
        }
    };
}

macro_rules! implement_assignop {
    ($imp:ident, $method:ident, $term:tt) => {
        impl $imp<Vec2d> for Point2d {
            fn $method(&mut self, rhs: Vec2d) {
                *self = Point2d(self.0 $term rhs);
            }
        }
        impl<'a> $imp<&'a Vec2d> for Point2d {
            fn $method(&mut self, rhs: &'a Vec2d) {
                *self = Point2d(self.0 $term rhs);
            }
        }
        impl $imp<f64> for Point2d {
            fn $method(&mut self, rhs: f64) {
                *self = Point2d(self.0 $term rhs);
            }
        }
    };
}

implement_binop! {Add, add, +}
implement_binop! {Sub, sub, -}
implement_binop! {Mul, mul, *}
implement_binop! {Div, div, /}

// Point(a) + Point(b) -> Point
impl Add<Point2d> for Point2d {
    type Output = Point2d;
    fn add(self, rhs: Point2d) -> Self::Output {
        self + rhs.0
    }
}
impl<'a> Add<&'a Point2d> for Point2d {
    type Output = Point2d;
    fn add(self, rhs: &'a Point2d) -> Self::Output {
        self + rhs.0
    }
}
impl<'a> Add<Point2d> for &'a Point2d {
    type Output = Point2d;
    fn add(self, rhs: Point2d) -> Self::Output {
        self + rhs.0
    }
}
impl<'a, 'b> Add<&'b Point2d> for &'a Point2d {
    type Output = Point2d;
    fn add(self, rhs: &'b Point2d) -> Self::Output {
        self + rhs.0
    }
}

// Point(end) - Point(start) -> Vec
impl Sub<Point2d> for Point2d {
    type Output = Vec2d;
    fn sub(self, rhs: Point2d) -> Self::Output {
        (self - rhs.0).0
    }
}
impl<'a> Sub<&'a Point2d> for Point2d {
    type Output = Vec2d;
    fn sub(self, rhs: &'a Point2d) -> Self::Output {
        (self - rhs.0).0
    }
}
impl<'a> Sub<Point2d> for &'a Point2d {
    type Output = Vec2d;
    fn sub(self, rhs: Point2d) -> Self::Output {
        (self - rhs.0).0
    }
}

impl<'a, 'b> Sub<&'b Point2d> for &'a Point2d {
    type Output = Vec2d;
    fn sub(self, rhs: &'b Point2d) -> Self::Output {
        (self - rhs.0).0
    }
}

implement_assignop! {AddAssign, add_assign, +}
implement_assignop! {SubAssign, sub_assign, -}
implement_assignop! {MulAssign, mul_assign, *}
implement_assignop! {DivAssign, div_assign, /}

impl Neg for Point2d {
    type Output = Point2d;
    fn neg(self) -> Self::Output {
        Point2d(-self.0)
    }
}

impl<'a> Neg for &'a Point2d {
    type Output = Point2d;
    fn neg(self) -> Point2d {
        Point2d(-self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new() {
        let v = Vec2d::new(1.0, 2.0);
        assert_eq!(v, Vec2d { x: 1.0, y: 2.0 });
    }
    #[test]
    fn debug_origin() {
        assert_eq!(format!("{:?}", Vec2d::origin()), "Vec2d { x: 0.0, y: 0.0 }");
    }
    #[test]
    fn add() {
        let a = Point2d::new(1.0, 1.0);
        let b = Vec2d::new(1.0, 2.0);
        assert_eq!(&a + &b, Point2d::new(2.0, 3.0));
        assert_eq!(a + b, Point2d::new(2.0, 3.0));

        assert_eq!(
            Point2d::new(0.0, 1.0) + Vec2d::new(2.0, 3.0),
            Point2d::new(2.0, 4.0)
        );
        assert_eq!(
            Point2d::new(0.0, 1.0) + &Vec2d::new(2.0, 3.0),
            Point2d::new(2.0, 4.0)
        );
        assert_eq!(
            &Point2d::new(0.0, 1.0) + Vec2d::new(2.0, 3.0),
            Point2d::new(2.0, 4.0)
        );
        assert_eq!(
            &Point2d::new(0.0, 1.0) + &Vec2d::new(2.0, 3.0),
            Point2d::new(2.0, 4.0)
        );

        assert_eq!(Point2d::new(0.0, 1.0) + 1.0, Point2d::new(1.0, 2.0));
        assert_eq!(&Point2d::new(0.0, 1.0) + 1.0, Point2d::new(1.0, 2.0));
        assert_eq!(1.0 + Point2d::new(0.0, 1.0), Point2d::new(1.0, 2.0));
        assert_eq!(1.0 + &Point2d::new(0.0, 1.0), Point2d::new(1.0, 2.0));
    }
    #[test]
    fn sub() {
        let a = Point2d::new(1.0, 1.0);
        let b = Vec2d::new(1.0, 2.0);
        assert_eq!(&a - &b, Point2d::new(0.0, -1.0));
        assert_eq!(a - b, Point2d::new(0.0, -1.0));
    }
    #[test]
    fn mul() {
        let a = Point2d::new(1.0, 1.0);
        let b = Vec2d::new(1.0, 2.0);
        assert_eq!(&a * 3.0, Point2d::new(3.0, 3.0));
        assert_eq!(&a * &b, Point2d::new(1.0, 2.0));
        assert_eq!(a * b, Point2d::new(1.0, 2.0));
    }
    #[test]
    fn div() {
        let a = Point2d::new(1.0, 1.0);
        let b = Vec2d::new(1.0, 2.0);
        assert_eq!(&a / 2.0, Point2d::new(0.5, 0.5));
        assert_eq!(&a / &b, Point2d::new(1.0, 0.5));
        assert_eq!(a / b, Point2d::new(1.0, 0.5));
    }
    #[test]
    fn add_point() {
        let a = Point2d::new(1.0, 1.0);
        let b = Point2d::new(1.0, 2.0);
        assert_eq!(&a + &b, Point2d::new(2.0, 3.0));
        assert_eq!(a + b, Point2d::new(2.0, 3.0));
    }
    #[test]
    fn sub_point() {
        let a = Point2d::new(1.0, 1.0);
        let b = Point2d::new(2.0, 3.0);
        assert_eq!(&a - &b, Vec2d::new(-1.0, -2.0));
        assert_eq!(b - a, Vec2d::new(1.0, 2.0));
    }
    #[test]
    fn add_assign() {
        let mut a = Point2d::new(1.0, 2.0);
        a += Vec2d::new(-1.0, -1.0);
        let mut b = a.clone();
        assert_eq!(a, Point2d::new(0.0, 1.0));
        b += 3.0;
        assert_eq!(b, Point2d::new(3.0, 4.0));
    }
    #[test]
    fn sub_assign() {
        let mut a = Point2d::new(1.0, 2.0);
        a -= Vec2d::new(-1.0, -1.0);
        let mut b = a.clone();
        assert_eq!(a, Point2d::new(2.0, 3.0));
        b -= 3.0;
        assert_eq!(b, Point2d::new(-1.0, 0.0));
    }
    #[test]
    fn mul_assign() {
        let mut a = Point2d::new(1.0, 2.0);
        a *= Vec2d::new(-1.0, -1.0);
        let mut b = a.clone();
        assert_eq!(a, Point2d::new(-1.0, -2.0));
        b *= 3.0;
        assert_eq!(b, Point2d::new(-3.0, -6.0));
    }
    #[test]
    fn div_assign() {
        let mut a = Point2d::new(1.0, 2.0);
        a /= Vec2d::new(-1.0, -1.0);
        let mut b = a.clone();
        assert_eq!(a, Point2d::new(-1.0, -2.0));
        b /= 2.0;
        assert_eq!(b, Point2d::new(-0.5, -1.0));
    }
    #[test]
    fn neg() {
        let a = Point2d::new(1.0, 2.0);
        assert_eq!(-&a, Point2d::new(-1.0, -2.0));
        assert_eq!(-a, Point2d::new(-1.0, -2.0));
    }
    #[test]
    fn norm() {
        let a = Vec2d::new(1.0, 2.0);
        assert_eq!(a.l2_norm(), 5.0f64.sqrt());
        assert_eq!(a.l2_norm_squared(), 5.0);
    }
    #[test]
    fn rotate() {
        assert!(
            (Point2d::new(1.0, 0.0).rotate_by_deg(90.0) - Vec2d::new(0.0, 1.0)).is_almost_zero()
        );
        assert!((Point2d::new(1.0, 0.0).rotate90() - Vec2d::new(0.0, 1.0)).is_almost_zero());
        assert!(
            (Point2d::new(1.0, 0.0).rotate_by_deg(180.0) - Vec2d::new(-1.0, 0.0)).is_almost_zero()
        );
        assert!(
            (Point2d::new(1.0, 0.0).rotate_by_deg(270.0) - Vec2d::new(0.0, -1.0)).is_almost_zero()
        );
        assert!(
            (Point2d::new(1.0, 1.0).rotate_by_rad(-std::f64::consts::FRAC_PI_4)
                - Vec2d::new(std::f64::consts::SQRT_2, 0.0))
            .is_almost_zero()
        );
    }

    #[test]
    fn gen_vec() {
        let a = Vec2d::new(1.0, 2.0);
        let b = Vec2d::new(5.0, 10.0);

        assert!(((&b - &a) - Vec2d::new(4.0, 8.0)).is_almost_zero());
        assert!(((a - b) - Vec2d::new(-4.0, -8.0)).is_almost_zero());
    }
}
