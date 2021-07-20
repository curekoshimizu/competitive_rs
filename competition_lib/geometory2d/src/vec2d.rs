use std::ops::{Add, Div, Mul, Neg, Sub};
use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Vec2d {
    pub x: f64,
    pub y: f64,
}

impl Vec2d {
    pub fn new(x: f64, y: f64) -> Self {
        Vec2d { x, y }
    }
    pub fn origin() -> Self {
        Vec2d { x: 0.0, y: 0.0 }
    }
    pub fn is_almost_zero(&self) -> bool {
        self.l2_norm() < 1.0e-5
    }
    pub fn rotate90(&self) -> Vec2d {
        Vec2d::new(-self.y, self.x)
    }
    pub fn rotate_by_deg(&self, deg: f64) -> Vec2d {
        self.rotate_by_rad(deg.to_radians())
    }
    pub fn rotate_by_rad(&self, rad: f64) -> Vec2d {
        let x = self.x;
        let y = self.y;
        let cos = f64::cos(rad);
        let sin = f64::sin(rad);
        Vec2d::new(x * cos - y * sin, x * sin + y * cos)
    }
    pub fn angle(&self, v: &Vec2d) -> f64 {
        let dot = self.dot(v);
        let cos_theta = dot / self.l2_norm() / v.l2_norm();

        // to avoid NaN
        let cos_theta = if cos_theta > 1.0 {
            1.0
        } else if cos_theta < -1.0 {
            -1.0
        } else {
            cos_theta
        };

        if self.det(v) > 0.0 {
            cos_theta.acos()
        } else {
            -cos_theta.acos()
        }
    }
    pub fn dot(&self, rhs: &Vec2d) -> f64 {
        let v = self * rhs;
        v.x + v.y
    }
    /// v1 = (x1, y1), v2 = (x2, y2)
    /// vec1.det(vec2) = x1 y2 - x2 y1 = |v1||v2| sin(theta)
    pub fn det(&self, rhs: &Vec2d) -> f64 {
        self.x * rhs.y - self.y * rhs.x
    }
    pub fn l2_norm(&self) -> f64 {
        self.dot(self).sqrt()
    }
    pub fn l2_norm_squared(&self) -> f64 {
        self.dot(self)
    }
    pub fn unit_vector(&self) -> Vec2d {
        let len = self.l2_norm();
        self / &Vec2d::new(len, len)
    }
    pub fn add(&self, rhs: &Vec2d) -> Vec2d {
        Vec2d {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
    pub fn sub(&self, rhs: &Vec2d) -> Vec2d {
        Vec2d {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
    fn mul(&self, rhs: &Vec2d) -> Vec2d {
        Vec2d {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
    fn div(&self, rhs: &Vec2d) -> Vec2d {
        Vec2d {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
    pub fn to_xy(&self) -> [f64; 2] {
        [self.x, self.y]
    }
}

macro_rules! implement_binop {
    ($imp:ident, $method:ident) => {
        impl $imp<Vec2d> for Vec2d {
            type Output = Vec2d;
            fn $method(self, rhs: Vec2d) -> Self::Output {
                <Vec2d>::$method(&self, &rhs)
            }
        }
        impl<'a> $imp<&'a Vec2d> for Vec2d {
            type Output = Vec2d;
            fn $method(self, rhs: &Vec2d) -> Self::Output {
                <Vec2d>::$method(&self, rhs)
            }
        }
        impl<'a> $imp<Vec2d> for &'a Vec2d {
            type Output = Vec2d;
            fn $method(self, rhs: Vec2d) -> Self::Output {
                <Vec2d>::$method(self, &rhs)
            }
        }
        impl<'a, 'b> $imp<&'b Vec2d> for &'a Vec2d {
            type Output = Vec2d;
            fn $method(self, rhs: &'b Vec2d) -> Vec2d {
                <Vec2d>::$method(self, rhs)
            }
        }

        impl $imp<f64> for Vec2d {
            type Output = Vec2d;
            fn $method(self, rhs: f64) -> Self::Output {
                <Vec2d>::$method(&self, &Vec2d::new(rhs, rhs))
            }
        }
        impl<'a> $imp<f64> for &'a Vec2d {
            type Output = Vec2d;
            fn $method(self, rhs: f64) -> Self::Output {
                <Vec2d>::$method(self, &Vec2d::new(rhs, rhs))
            }
        }
        impl $imp<Vec2d> for f64 {
            type Output = Vec2d;
            fn $method(self, rhs: Vec2d) -> Self::Output {
                <Vec2d>::$method(&Vec2d::new(self, self), &rhs)
            }
        }
        impl<'a> $imp<&'a Vec2d> for f64 {
            type Output = Vec2d;
            fn $method(self, rhs: &Vec2d) -> Self::Output {
                <Vec2d>::$method(&Vec2d::new(self, self), rhs)
            }
        }
    };
}

macro_rules! implement_assignop {
    ($imp:ident, $method:ident, $term:tt) => {
        impl $imp<Vec2d> for Vec2d {
            fn $method(&mut self, rhs: Self) {
                *self = Self {
                    x: self.x $term rhs.x,
                    y: self.y $term rhs.y,
                };
            }
        }
        impl<'a> $imp<&'a Vec2d> for Vec2d {
            fn $method(&mut self, rhs: &'a Vec2d) {
                *self = Self {
                    x: self.x $term rhs.x,
                    y: self.y $term rhs.y,
                };
            }
        }
        impl $imp<f64> for Vec2d {
            fn $method(&mut self, rhs: f64) {
                *self = Self {
                    x: self.x $term rhs,
                    y: self.y $term rhs,
                };
            }
        }
    };
}

implement_binop! {Add, add}
implement_binop! {Sub, sub}
implement_binop! {Mul, mul}
implement_binop! {Div, div}

implement_assignop! {AddAssign, add_assign, +}
implement_assignop! {SubAssign, sub_assign, -}
implement_assignop! {MulAssign, mul_assign, *}
implement_assignop! {DivAssign, div_assign, /}

impl Neg for Vec2d {
    type Output = Vec2d;
    fn neg(self) -> Self::Output {
        Vec2d {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl<'a> Neg for &'a Vec2d {
    type Output = Vec2d;
    fn neg(self) -> Vec2d {
        Vec2d {
            x: -self.x,
            y: -self.y,
        }
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
        let a = Vec2d::new(1.0, 1.0);
        let b = Vec2d::new(1.0, 2.0);
        assert_eq!(&a + &b, Vec2d::new(2.0, 3.0));
        assert_eq!(a + b, Vec2d::new(2.0, 3.0));

        assert_eq!(
            Vec2d::new(0.0, 1.0) + Vec2d::new(2.0, 3.0),
            Vec2d::new(2.0, 4.0)
        );
        assert_eq!(
            Vec2d::new(0.0, 1.0) + &Vec2d::new(2.0, 3.0),
            Vec2d::new(2.0, 4.0)
        );
        assert_eq!(
            &Vec2d::new(0.0, 1.0) + Vec2d::new(2.0, 3.0),
            Vec2d::new(2.0, 4.0)
        );
        assert_eq!(
            &Vec2d::new(0.0, 1.0) + &Vec2d::new(2.0, 3.0),
            Vec2d::new(2.0, 4.0)
        );

        assert_eq!(Vec2d::new(0.0, 1.0) + 1.0, Vec2d::new(1.0, 2.0));
        assert_eq!(&Vec2d::new(0.0, 1.0) + 1.0, Vec2d::new(1.0, 2.0));
        assert_eq!(1.0 + Vec2d::new(0.0, 1.0), Vec2d::new(1.0, 2.0));
        assert_eq!(1.0 + &Vec2d::new(0.0, 1.0), Vec2d::new(1.0, 2.0));
    }
    #[test]
    fn sub() {
        let a = Vec2d::new(1.0, 1.0);
        let b = Vec2d::new(1.0, 2.0);
        assert_eq!(&a - &b, Vec2d::new(0.0, -1.0));
        assert_eq!(a - b, Vec2d::new(0.0, -1.0));
    }
    #[test]
    fn mul() {
        let a = Vec2d::new(1.0, 1.0);
        let b = Vec2d::new(1.0, 2.0);
        assert_eq!(&a * &b, Vec2d::new(1.0, 2.0));
        assert_eq!(a * b, Vec2d::new(1.0, 2.0));
    }
    #[test]
    fn div() {
        let a = Vec2d::new(1.0, 1.0);
        let b = Vec2d::new(1.0, 2.0);
        assert_eq!(&a / &b, Vec2d::new(1.0, 0.5));
        assert_eq!(a / b, Vec2d::new(1.0, 0.5));
    }
    #[test]
    fn add_assign() {
        let mut a = Vec2d::new(1.0, 2.0);
        a += Vec2d::new(-1.0, -1.0);
        let mut b = a.clone();
        assert_eq!(a, Vec2d::new(0.0, 1.0));
        b += 3.0;
        assert_eq!(b, Vec2d::new(3.0, 4.0));
    }
    #[test]
    fn sub_assign() {
        let mut a = Vec2d::new(1.0, 2.0);
        a -= Vec2d::new(-1.0, -1.0);
        let mut b = a.clone();
        assert_eq!(a, Vec2d::new(2.0, 3.0));
        b -= 3.0;
        assert_eq!(b, Vec2d::new(-1.0, 0.0));
    }
    #[test]
    fn mul_assign() {
        let mut a = Vec2d::new(1.0, 2.0);
        a *= Vec2d::new(-1.0, -1.0);
        let mut b = a.clone();
        assert_eq!(a, Vec2d::new(-1.0, -2.0));
        b *= 3.0;
        assert_eq!(b, Vec2d::new(-3.0, -6.0));
    }
    #[test]
    fn div_assign() {
        let mut a = Vec2d::new(1.0, 2.0);
        a /= Vec2d::new(-1.0, -1.0);
        let mut b = a.clone();
        assert_eq!(a, Vec2d::new(-1.0, -2.0));
        b /= 2.0;
        assert_eq!(b, Vec2d::new(-0.5, -1.0));
    }
    #[test]
    fn neg() {
        let a = Vec2d::new(1.0, 2.0);
        assert_eq!(-&a, Vec2d::new(-1.0, -2.0));
        assert_eq!(-a, Vec2d::new(-1.0, -2.0));
    }
    #[test]
    fn dot() {
        assert_eq!(Vec2d::new(1.0, 2.0).dot(&Vec2d::new(2.0, -3.0)), -4.0);
    }
    #[test]
    fn det() {
        assert_eq!(Vec2d::new(2.0, -3.0).det(&Vec2d::new(1.0, 2.0)), 7.0);
        assert_eq!(Vec2d::new(1.0, 2.0).det(&Vec2d::new(2.0, -3.0)), -7.0);
    }
    #[test]
    fn norm() {
        let a = Vec2d::new(1.0, 2.0);
        assert_eq!(a.l2_norm(), 5.0f64.sqrt());
        assert_eq!(a.l2_norm_squared(), 5.0);
    }
    #[test]
    fn unit() {
        let a = Vec2d::new(3.0, 4.0);
        assert_eq!(a.unit_vector(), Vec2d::new(3.0 / 5.0, 4.0 / 5.0));
    }
    #[test]
    fn rotate() {
        assert!((Vec2d::new(1.0, 0.0).rotate_by_deg(90.0) - Vec2d::new(0.0, 1.0)).is_almost_zero());
        assert!((Vec2d::new(1.0, 0.0).rotate90() - Vec2d::new(0.0, 1.0)).is_almost_zero());
        assert!(
            (Vec2d::new(1.0, 0.0).rotate_by_deg(180.0) - Vec2d::new(-1.0, 0.0)).is_almost_zero()
        );
        assert!(
            (Vec2d::new(1.0, 0.0).rotate_by_deg(270.0) - Vec2d::new(0.0, -1.0)).is_almost_zero()
        );
        assert!(
            (Vec2d::new(1.0, 1.0).rotate_by_rad(-std::f64::consts::FRAC_PI_4)
                - Vec2d::new(std::f64::consts::SQRT_2, 0.0))
            .is_almost_zero()
        );
    }
    #[test]
    fn angle() {
        let v1 = Vec2d::new(1.0, 0.1);
        for deg in vec![
            0.0, 10.0, -10.0, 30.0, -30.0, 80.0, -80.0, 90.0, -90.0, 110.0, -110.0, 180.0, -180.0,
            240.0, -240.0,
        ] {
            let v2 = v1.rotate_by_deg(deg);
            let angle = v1.angle(&v2);
            assert!((v1.rotate_by_rad(angle) - v2).is_almost_zero());
        }
    }
}
