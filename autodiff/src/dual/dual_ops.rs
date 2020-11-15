use super::Dual;
use num::Float;
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

impl<T: Float> Neg for Dual<T> {
    type Output = Self;
    fn neg(self) -> Self {
        Dual {
            val: -self.val,
            eps: -self.eps,
        }
    }
}

impl<T: Float> Add for Dual<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            val: self.val + other.val,
            eps: self.eps + other.eps,
        }
    }
}

impl<T: Float> Sub for Dual<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            val: self.val - other.val,
            eps: self.eps - other.eps,
        }
    }
}

impl<T: Float> Mul for Dual<T> {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            val: self.val * other.val,
            eps: self.eps * other.val + self.val * other.eps,
        }
    }
}

impl<T: Float> Div for Dual<T> {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self {
            val: self.val / other.val,
            eps: (self.eps * other.val - self.val * other.eps) / (other.val * other.val),
        }
    }
}

impl<T: Float> Rem for Dual<T> {
    type Output = Self;

    fn rem(self, other: Self) -> Self {
        Self {
            val: self.val.rem(other.val),
            eps: self.eps - other.eps * (self.val / other.val).trunc(),
        }
    }
}
