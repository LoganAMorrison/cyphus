use super::Dual;
use num::traits::ToPrimitive;
use num::Float;
use num::{Num, NumCast, One, Zero};

impl<T: Float> Zero for Dual<T> {
    fn zero() -> Self {
        Self {
            val: T::zero(),
            eps: T::zero(),
        }
    }
    fn is_zero(&self) -> bool {
        self.val.is_zero()
    }
}

impl<T: Float> One for Dual<T> {
    fn one() -> Self {
        Self {
            val: T::one(),
            eps: T::zero(),
        }
    }
}

impl<T: Float + ToPrimitive> ToPrimitive for Dual<T> {
    fn to_i64(&self) -> Option<i64> {
        self.val.to_i64()
    }
    fn to_u64(&self) -> Option<u64> {
        self.val.to_u64()
    }
}

impl<T: Float> NumCast for Dual<T> {
    fn from<S: ToPrimitive>(n: S) -> Option<Self> {
        let val = T::from(n);
        let eps = T::zero();
        match val {
            Some(x) => Some(Self { val: x, eps }),
            None => None,
        }
    }
}

impl<T: Float> Num for Dual<T> {
    type FromStrRadixErr = T::FromStrRadixErr;
    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        let res = T::from_str_radix(str, radix);
        match res {
            Ok(val) => Ok(Self {
                val,
                eps: T::zero(),
            }),
            Err(e) => Err(e),
        }
    }
}
