use super::Dual;
use std::cmp::{Ordering, PartialEq, PartialOrd};

impl<T: PartialEq> PartialEq for Dual<T> {
    fn eq(&self, other: &Self) -> bool {
        return self.val == other.val;
    }
}

impl PartialEq<f64> for Dual<f64> {
    fn eq(&self, other: &f64) -> bool {
        return self.val == *other;
    }
}

impl PartialEq<f32> for Dual<f32> {
    fn eq(&self, other: &f32) -> bool {
        return self.val == *other;
    }
}

impl PartialEq<i32> for Dual<f64> {
    fn eq(&self, other: &i32) -> bool {
        return self.val == f64::from(*other);
    }
}

impl<T: PartialOrd> PartialOrd for Dual<T> {
    fn partial_cmp(&self, other: &Dual<T>) -> Option<Ordering> {
        return self.val.partial_cmp(&other.val);
    }
}

impl PartialOrd<f64> for Dual<f64> {
    fn partial_cmp(&self, other: &f64) -> Option<Ordering> {
        return self.val.partial_cmp(&other);
    }
}

impl PartialOrd<f32> for Dual<f32> {
    fn partial_cmp(&self, other: &f32) -> Option<Ordering> {
        return self.val.partial_cmp(&other);
    }
}
