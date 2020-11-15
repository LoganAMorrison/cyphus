pub mod dual_cmp;
pub mod dual_float;
pub mod dual_num;
pub mod dual_ops;

/// Core type for automatic differentiation of floating point functions
#[derive(Clone, Copy, Debug)]
pub struct Dual<T> {
    /// The real par dual.
    pub val: T,
    /// The infinitesimal part of the dual.
    pub eps: T,
}

#[cfg(test)]
mod test {
    use super::*;
    use num::Float;

    #[test]
    fn test_trig() {
        let x = Dual {
            val: 0f64,
            eps: 1f64,
        };
        let y = Dual {
            val: std::f64::consts::PI / 2.0,
            eps: 1f64,
        };
        assert!((x.sin().eps - x.val.cos()).abs() < 1e-12);
        assert!((y.cos().eps + y.val.sin()).abs() < 1e-12);
    }
}
