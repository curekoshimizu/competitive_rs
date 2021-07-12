use std::ops::{Add, Div, Mul, Neg, Sub};
use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

#[derive(PartialEq, Debug, Clone)]
pub struct Vec3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3d {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3d { x, y, z }
    }
    pub fn origin() -> Self {
        Vec3d {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
    pub fn is_almost_zero(&self) -> bool {
        self.l2_norm() < 1.0e-5
    }
    pub fn dot(&self, rhs: &Vec3d) -> f64 {
        let v = self * rhs;
        v.x + v.y + v.z
    }
    pub fn l2_norm(&self) -> f64 {
        self.dot(self).sqrt()
    }
    pub fn l2_norm_squared(&self) -> f64 {
        self.dot(self)
    }
    pub fn unit_vector(&self) -> Vec3d {
        let len = self.l2_norm();
        self / &Vec3d::new(len, len, len)
    }
    pub fn add(&self, rhs: &Vec3d) -> Vec3d {
        Vec3d {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
    pub fn sub(&self, rhs: &Vec3d) -> Vec3d {
        Vec3d {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
    fn mul(&self, rhs: &Vec3d) -> Vec3d {
        Vec3d {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
    fn div(&self, rhs: &Vec3d) -> Vec3d {
        Vec3d {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
    pub fn to_xyz(&self) -> [f64; 3] {
        [self.x, self.y, self.z]
    }
}

macro_rules! implement_binop {
    ($imp:ident, $method:ident) => {
        impl $imp<Vec3d> for Vec3d {
            type Output = Vec3d;
            fn $method(self, rhs: Vec3d) -> Self::Output {
                <Vec3d>::$method(&self, &rhs)
            }
        }
        impl<'a> $imp<&'a Vec3d> for Vec3d {
            type Output = Vec3d;
            fn $method(self, rhs: &Vec3d) -> Self::Output {
                <Vec3d>::$method(&self, rhs)
            }
        }
        impl<'a> $imp<Vec3d> for &'a Vec3d {
            type Output = Vec3d;
            fn $method(self, rhs: Vec3d) -> Self::Output {
                <Vec3d>::$method(self, &rhs)
            }
        }
        impl<'a, 'b> $imp<&'b Vec3d> for &'a Vec3d {
            type Output = Vec3d;
            fn $method(self, rhs: &'b Vec3d) -> Vec3d {
                <Vec3d>::$method(self, rhs)
            }
        }

        impl $imp<f64> for Vec3d {
            type Output = Vec3d;
            fn $method(self, rhs: f64) -> Self::Output {
                <Vec3d>::$method(&self, &Vec3d::new(rhs, rhs, rhs))
            }
        }
        impl<'a> $imp<f64> for &'a Vec3d {
            type Output = Vec3d;
            fn $method(self, rhs: f64) -> Self::Output {
                <Vec3d>::$method(self, &Vec3d::new(rhs, rhs, rhs))
            }
        }
        impl $imp<Vec3d> for f64 {
            type Output = Vec3d;
            fn $method(self, rhs: Vec3d) -> Self::Output {
                <Vec3d>::$method(&Vec3d::new(self, self, self), &rhs)
            }
        }
        impl<'a> $imp<&'a Vec3d> for f64 {
            type Output = Vec3d;
            fn $method(self, rhs: &Vec3d) -> Self::Output {
                <Vec3d>::$method(&Vec3d::new(self, self, self), rhs)
            }
        }
    };
}

macro_rules! implement_assignop {
    ($imp:ident, $method:ident, $term:tt) => {
        impl $imp<Vec3d> for Vec3d {
            fn $method(&mut self, rhs: Self) {
                *self = Self {
                    x: self.x $term rhs.x,
                    y: self.y $term rhs.y,
                    z: self.z $term rhs.z,
                };
            }
        }
        impl<'a> $imp<&'a Vec3d> for Vec3d {
            fn $method(&mut self, rhs: &'a Vec3d) {
                *self = Self {
                    x: self.x $term rhs.x,
                    y: self.y $term rhs.y,
                    z: self.z $term rhs.z,
                };
            }
        }
        impl $imp<f64> for Vec3d {
            fn $method(&mut self, rhs: f64) {
                *self = Self {
                    x: self.x $term rhs,
                    y: self.y $term rhs,
                    z: self.z $term rhs,
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

impl Neg for Vec3d {
    type Output = Vec3d;
    fn neg(self) -> Self::Output {
        Vec3d {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl<'a> Neg for &'a Vec3d {
    type Output = Vec3d;
    fn neg(self) -> Vec3d {
        Vec3d {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new() {
        let v = Vec3d::new(1.0, 2.0, 3.0);
        assert_eq!(
            v,
            Vec3d {
                x: 1.0,
                y: 2.0,
                z: 3.0
            }
        );
    }
    #[test]
    fn debug_origin() {
        assert_eq!(
            format!("{:?}", Vec3d::origin()),
            "Vec3d { x: 0.0, y: 0.0, z: 0.0 }"
        );
    }
    #[test]
    fn add() {
        let a = Vec3d::new(1.0, 1.0, 1.0);
        let b = Vec3d::new(1.0, 2.0, 3.0);
        assert_eq!(&a + &b, Vec3d::new(2.0, 3.0, 4.0));
        assert_eq!(a + b, Vec3d::new(2.0, 3.0, 4.0));

        assert_eq!(
            Vec3d::new(0.0, 1.0, 2.0) + Vec3d::new(2.0, 3.0, 4.0),
            Vec3d::new(2.0, 4.0, 6.0)
        );
        assert_eq!(
            Vec3d::new(0.0, 1.0, 2.0) + &Vec3d::new(2.0, 3.0, 4.0),
            Vec3d::new(2.0, 4.0, 6.0)
        );
        assert_eq!(
            &Vec3d::new(0.0, 1.0, 2.0) + Vec3d::new(2.0, 3.0, 4.0),
            Vec3d::new(2.0, 4.0, 6.0)
        );
        assert_eq!(
            &Vec3d::new(0.0, 1.0, 2.0) + &Vec3d::new(2.0, 3.0, 4.0),
            Vec3d::new(2.0, 4.0, 6.0)
        );

        assert_eq!(Vec3d::new(0.0, 1.0, 2.0) + 1.0, Vec3d::new(1.0, 2.0, 3.0));
        assert_eq!(&Vec3d::new(0.0, 1.0, 2.0) + 1.0, Vec3d::new(1.0, 2.0, 3.0));
        assert_eq!(1.0 + Vec3d::new(0.0, 1.0, 2.0), Vec3d::new(1.0, 2.0, 3.0));
        assert_eq!(1.0 + &Vec3d::new(0.0, 1.0, 2.0), Vec3d::new(1.0, 2.0, 3.0));
    }
    #[test]
    fn sub() {
        let a = Vec3d::new(1.0, 1.0, 1.0);
        let b = Vec3d::new(1.0, 2.0, 3.0);
        assert_eq!(&a - &b, Vec3d::new(0.0, -1.0, -2.0));
        assert_eq!(a - b, Vec3d::new(0.0, -1.0, -2.0));
    }
    #[test]
    fn mul() {
        let a = Vec3d::new(1.0, 1.0, 2.0);
        let b = Vec3d::new(1.0, 2.0, 3.0);
        assert_eq!(&a * &b, Vec3d::new(1.0, 2.0, 6.0));
        assert_eq!(a * b, Vec3d::new(1.0, 2.0, 6.0));
    }
    #[test]
    fn div() {
        let a = Vec3d::new(1.0, 1.0, 2.0);
        let b = Vec3d::new(1.0, 2.0, 2.0);
        assert_eq!(&a / &b, Vec3d::new(1.0, 0.5, 1.0));
        assert_eq!(a / b, Vec3d::new(1.0, 0.5, 1.0));
    }
    #[test]
    fn add_assign() {
        let mut a = Vec3d::new(1.0, 2.0, 3.0);
        a += Vec3d::new(-1.0, -1.0, -1.0);
        let mut b = a.clone();
        assert_eq!(a, Vec3d::new(0.0, 1.0, 2.0));
        b += 3.0;
        assert_eq!(b, Vec3d::new(3.0, 4.0, 5.0));
    }
    #[test]
    fn sub_assign() {
        let mut a = Vec3d::new(1.0, 2.0, 3.0);
        a -= Vec3d::new(-1.0, -1.0, -1.0);
        let mut b = a.clone();
        assert_eq!(a, Vec3d::new(2.0, 3.0, 4.0));
        b -= 3.0;
        assert_eq!(b, Vec3d::new(-1.0, 0.0, 1.0));
    }
    #[test]
    fn mul_assign() {
        let mut a = Vec3d::new(1.0, 2.0, 3.0);
        a *= Vec3d::new(-1.0, -1.0, -1.0);
        let mut b = a.clone();
        assert_eq!(a, Vec3d::new(-1.0, -2.0, -3.0));
        b *= 3.0;
        assert_eq!(b, Vec3d::new(-3.0, -6.0, -9.0));
    }
    #[test]
    fn div_assign() {
        let mut a = Vec3d::new(1.0, 2.0, 3.0);
        a /= Vec3d::new(-1.0, -1.0, -1.0);
        let mut b = a.clone();
        assert_eq!(a, Vec3d::new(-1.0, -2.0, -3.0));
        b /= 2.0;
        assert_eq!(b, Vec3d::new(-0.5, -1.0, -1.5));
    }
    #[test]
    fn neg() {
        let a = Vec3d::new(1.0, 2.0, 4.0);
        assert_eq!(-&a, Vec3d::new(-1.0, -2.0, -4.0));
        assert_eq!(-a, Vec3d::new(-1.0, -2.0, -4.0));
    }
    #[test]
    fn dot() {
        assert_eq!(
            Vec3d::new(1.0, 2.0, 3.0).dot(&Vec3d::new(2.0, -3.0, 1.0)),
            -1.0
        );
    }
    #[test]
    fn norm() {
        let a = Vec3d::new(1.0, 2.0, 3.0);
        assert_eq!(a.l2_norm(), 14.0f64.sqrt());
        assert_eq!(a.l2_norm_squared(), 14.0);
    }
    #[test]
    fn unit() {
        let a = Vec3d::new(3.0, 0.0, 4.0);
        assert_eq!(a.unit_vector(), Vec3d::new(3.0 / 5.0, 0.0, 4.0 / 5.0));
    }
}
