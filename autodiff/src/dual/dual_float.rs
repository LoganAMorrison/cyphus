use super::Dual;
use num::Float;
use std::num::FpCategory;

impl<T: Float> Float for Dual<T> {
    fn nan() -> Self {
        Dual {
            val: T::nan(),
            eps: T::zero(),
        }
    }
    /// Returns the infinite value.
    fn infinity() -> Self {
        Dual {
            val: T::infinity(),
            eps: T::zero(),
        }
    }
    /// Returns the negative infinite value.
    fn neg_infinity() -> Self {
        Dual {
            val: T::neg_infinity(),
            eps: T::zero(),
        }
    }
    /// Returns `-0.0`.
    fn neg_zero() -> Self {
        Dual {
            val: T::neg_zero(),
            eps: T::zero(),
        }
    }

    /// Returns the smallest finite value that this type can represent.
    fn min_value() -> Self {
        Dual {
            val: T::min_value(),
            eps: T::zero(),
        }
    }

    /// Returns the smallest positive, normalized value that this type can represent.
    fn min_positive_value() -> Self {
        Dual {
            val: T::min_positive_value(),
            eps: T::zero(),
        }
    }

    /// Returns epsilon, a small positive value.
    fn epsilon() -> Self {
        Dual {
            val: T::epsilon(),
            eps: T::zero(),
        }
    }

    /// Returns the largest finite value that this type can represent.
    fn max_value() -> Self {
        Dual {
            val: T::max_value(),
            eps: T::zero(),
        }
    }

    /// Returns `true` if this value is `NaN` and false otherwise.
    fn is_nan(self) -> bool {
        self.val.is_nan()
    }

    /// Returns `true` if this value is positive infinity or negative infinity and
    fn is_infinite(self) -> bool {
        self.val.is_infinite()
    }

    /// Returns `true` if this number is neither infinite nor `NaN`.
    fn is_finite(self) -> bool {
        self.val.is_finite()
    }

    /// Returns `true` if the number is neither zero, infinite,
    fn is_normal(self) -> bool {
        self.val.is_normal()
    }

    /// Returns the floating point category of the number. If only one property
    /// is going to be tested, it is generally faster to use the specific
    /// predicate instead.
    fn classify(self) -> FpCategory {
        self.val.classify()
    }

    /// Returns the largest integer less than or equal to a number.
    fn floor(self) -> Self {
        Dual {
            val: self.val.floor(),
            eps: T::zero(),
        }
    }

    /// Returns the smallest integer greater than or equal to a number.
    fn ceil(self) -> Self {
        Dual {
            val: self.val.ceil(),
            eps: T::zero(),
        }
    }

    /// Returns the nearest integer to a number. Round half-way cases away from
    /// `0.0`.
    fn round(self) -> Self {
        Dual {
            val: self.val.round(),
            eps: self.eps.round(),
        }
    }

    /// Return the integer part of a number.
    fn trunc(self) -> Self {
        Dual {
            val: self.val.trunc(),
            eps: self.eps.trunc(),
        }
    }

    /// Returns the fractional part of a number.
    fn fract(self) -> Self {
        Dual {
            val: self.val.fract(),
            eps: self.eps.fract(),
        }
    }

    /// Computes the absolute value of `self`.
    fn abs(self) -> Self {
        let eps = if self.val > T::zero() {
            T::one()
        } else if self.val < T::zero() {
            -T::one()
        } else {
            T::nan()
        };
        Dual {
            val: self.val.abs(),
            eps,
        }
    }

    /// Returns a number that represents the sign of `self`.
    fn signum(self) -> Self {
        Dual {
            val: self.val.signum(),
            eps: if self.val == T::zero() {
                T::nan()
            } else {
                T::zero()
            },
        }
    }

    /// Returns `true` if `self` is positive, including `+0.0`,
    /// `Float::infinity()`, and since Rust 1.20 also `Float::nan()`.
    fn is_sign_positive(self) -> bool {
        self.val.is_sign_positive()
    }

    /// Returns `true` if `self` is negative, including `-0.0`,
    fn is_sign_negative(self) -> bool {
        self.val.is_sign_negative()
    }

    /// Fused multiply-add. Computes `(self * a) + b` with only one rounding
    /// error, yielding a more accurate result than an unfused multiply-add.
    fn mul_add(self, a: Self, b: Self) -> Self {
        Dual {
            val: self.val.mul_add(a.val, b.val),
            eps: self.val.mul_add(a.eps, a.val * self.eps + b.eps),
        }
    }
    /// Take the reciprocal (inverse) of a number, `1/x`.
    fn recip(self) -> Self {
        Dual {
            val: self.val.recip(),
            eps: -self.val.recip() * self.val.recip(),
        }
    }

    /// Raise a number to an integer power.
    fn powi(self, n: i32) -> Self {
        Dual {
            val: self.val.powi(n),
            eps: T::from(n).unwrap() * self.val.powi(n - 1) * self.eps,
        }
    }

    /// Raise a number to a floating point power.
    ///
    /// ```
    /// use num_traits::Float;
    ///
    /// let x = 2.0;
    /// let abs_difference = (x.powf(2.0) - x*x).abs();
    ///
    /// assert!(abs_difference < 1e-10);
    /// ```
    fn powf(self, n: Self) -> Self {
        Dual {
            val: self.val.powf(n.val),
            eps: self.val * n.val * ((n.val * self.eps) / self.val + n.eps * self.val.ln()),
        }
    }

    /// Take the square root of a number.
    fn sqrt(self) -> Self {
        Dual {
            val: self.val.sqrt(),
            eps: self.eps * self.val.sqrt().recip() / T::from(2).unwrap(),
        }
    }

    /// Returns `e^(self)`, (the exponential function).
    fn exp(self) -> Self {
        Dual {
            val: self.val.exp(),
            eps: self.eps * self.val.exp(),
        }
    }

    /// Returns `2^(self)`.
    fn exp2(self) -> Self {
        Dual {
            val: self.val.exp2(),
            eps: self.val.exp2() * self.eps * T::from(2).unwrap().ln(),
        }
    }

    /// Returns the natural logarithm of the number.
    fn ln(self) -> Self {
        Dual {
            val: self.val.ln(),
            eps: self.val.recip() * self.eps,
        }
    }

    /// Returns the logarithm of the number with respect to an arbitrary base.
    fn log(self, base: Self) -> Self {
        Dual {
            val: self.val.log(base.val),
            eps: ((-((self.eps * self.val.log(base.val)) / self.val) + base.eps / base.val)
                / self.val.ln()),
        }
    }

    /// Returns the base 2 logarithm of the number.
    fn log2(self) -> Self {
        Dual {
            val: self.val.log2(),
            eps: (-((self.eps * self.val.log2()) / self.val) / self.val.ln()),
        }
    }

    /// Returns the base 10 logarithm of the number.
    fn log10(self) -> Self {
        Dual {
            val: self.val.log10(),
            eps: ((-((self.eps * self.val.log10()) / self.val)) / self.val.ln()),
        }
    }

    /// Converts radians to degrees.
    #[inline]
    fn to_degrees(self) -> Self {
        Dual {
            val: self.val.to_degrees(),
            eps: self.eps.to_degrees(),
        }
    }

    /// Converts degrees to radians.
    #[inline]
    fn to_radians(self) -> Self {
        Dual {
            val: self.val.to_radians(),
            eps: self.eps.to_radians(),
        }
    }

    /// Returns the maximum of the two numbers.
    fn max(self, other: Self) -> Self {
        if self.val >= other.val {
            Dual {
                val: self.val,
                eps: self.eps,
            }
        } else {
            Dual {
                val: other.val,
                eps: other.eps,
            }
        }
    }

    /// Returns the minimum of the two numbers.
    fn min(self, other: Self) -> Self {
        if self.val <= other.val {
            Dual {
                val: self.val,
                eps: self.eps,
            }
        } else {
            Dual {
                val: other.val,
                eps: other.eps,
            }
        }
    }

    /// The positive difference of two numbers.
    fn abs_sub(self, other: Self) -> Self {
        Dual {
            val: self.val.abs_sub(other.val),
            eps: self.eps.abs_sub(other.eps),
        }
    }

    /// Take the cubic root of a number.
    fn cbrt(self) -> Self {
        Dual {
            val: self.val.cbrt(),
            eps: self.val.cbrt().recip().powi(2) * self.eps / T::from(3).unwrap(),
        }
    }

    /// Calculate the length of the hypotenuse of a right-angle triangle given
    /// legs of length `x` and `y`.
    fn hypot(self, other: Self) -> Self {
        Dual {
            val: self.val.hypot(other.val),
            eps: (other.eps * self.val + other.val * self.eps) * self.val.hypot(other.val).recip(),
        }
    }

    /// Computes the sine of a number (in radians).
    fn sin(self) -> Self {
        Dual {
            val: self.val.sin(),
            eps: self.val.cos() * self.eps,
        }
    }

    /// Computes the cosine of a number (in radians).
    fn cos(self) -> Self {
        Dual {
            val: self.val.cos(),
            eps: -self.val.sin() * self.eps,
        }
    }

    /// Computes the tangent of a number (in radians).
    fn tan(self) -> Self {
        Dual {
            val: self.val.tan(),
            eps: self.eps * self.val.cos().recip().powi(2),
        }
    }

    /// Computes the arcsine of a number. Return value is in radians in
    /// the range [-pi/2, pi/2] or NaN if the number is outside the range
    /// [-1, 1].
    fn asin(self) -> Self {
        Dual {
            val: self.val.asin(),
            eps: self.eps * (T::from(1).unwrap() - self.val.powi(2)).sqrt(),
        }
    }

    /// Computes the arccosine of a number. Return value is in radians in
    /// the range [0, pi] or NaN if the number is outside the range
    fn acos(self) -> Self {
        Dual {
            val: self.val.acos(),
            eps: -self.eps * (T::from(1).unwrap() - self.val.powi(2)).sqrt(),
        }
    }

    /// Computes the arctangent of a number. Return value is in radians in the
    /// range [-pi/2, pi/2];
    fn atan(self) -> Self {
        Dual {
            val: self.val.atan(),
            eps: self.eps * (T::from(1).unwrap() + self.val.powi(2)),
        }
    }

    /// Computes the four quadrant arctangent of `self` (`y`) and `other` (`x`).
    ///
    /// * `x = 0`, `y = 0`: `0`
    /// * `x >= 0`: `arctan(y/x)` -> `[-pi/2, pi/2]`
    /// * `y >= 0`: `arctan(y/x) + pi` -> `(pi/2, pi]`
    /// * `y < 0`: `arctan(y/x) - pi` -> `(-pi, -pi/2)`
    fn atan2(self, other: Self) -> Self {
        Dual {
            val: self.val.atan2(other.val),
            eps: (other.eps * self.val - other.val * self.eps)
                / (other.val.powi(2) + self.val.powi(2)),
        }
    }

    /// Simultaneously computes the sine and cosine of the number, `x`. Returns
    /// `(sin(x), cos(x))`.
    fn sin_cos(self) -> (Self, Self) {
        (
            Dual {
                val: self.val.sin(),
                eps: self.val.cos() * self.eps,
            },
            Dual {
                val: self.val.cos(),
                eps: -self.val.sin() * self.eps,
            },
        )
    }

    /// Returns `e^(self) - 1` in a way that is accurate even if the
    /// number is close to zero.
    fn exp_m1(self) -> Self {
        Dual {
            val: self.val.exp_m1(),
            eps: self.val.exp() * self.eps,
        }
    }

    /// Returns `ln(1+n)` (natural logarithm) more accurately than if
    /// the operations were performed separately.
    fn ln_1p(self) -> Self {
        Dual {
            val: self.val.ln_1p(),
            eps: self.eps * (T::from(1).unwrap() + self.val).recip(),
        }
    }

    /// Hyperbolic sine function.
    fn sinh(self) -> Self {
        Dual {
            val: self.val.sinh(),
            eps: self.val.cosh() * self.eps,
        }
    }

    /// Hyperbolic cosine function.
    fn cosh(self) -> Self {
        Dual {
            val: self.val.cosh(),
            eps: self.val.sinh() * self.eps,
        }
    }

    /// Hyperbolic tangent function.
    fn tanh(self) -> Self {
        Dual {
            val: self.val.tanh(),
            eps: self.val.cosh().powi(2).recip() * self.eps,
        }
    }

    /// Inverse hyperbolic sine function.
    fn asinh(self) -> Self {
        Dual {
            val: self.val.asinh(),
            eps: self.eps * (T::from(1).unwrap() + self.val.powi(2)).sqrt().recip(),
        }
    }

    /// Inverse hyperbolic cosine function.
    fn acosh(self) -> Self {
        Dual {
            val: self.val.acosh(),
            eps: self.eps * (self.val.powi(2) - T::from(1).unwrap()).sqrt().recip(),
        }
    }

    /// Inverse hyperbolic tangent function.
    fn atanh(self) -> Self {
        Dual {
            val: self.val.atanh(),
            eps: -self.eps * (self.val.powi(2) - T::from(1).unwrap()).recip(),
        }
    }

    /// Returns the mantissa, base 2 exponent, and sign as integers, respectively.
    /// The original number can be recovered by `sign * mantissa * 2 ^ exponent`.
    fn integer_decode(self) -> (u64, i16, i8) {
        self.val.integer_decode()
    }
}
