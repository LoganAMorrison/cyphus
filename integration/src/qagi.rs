// GSL License:
//
// Copyright (C) 1996, 1997, 1998, 1999, 2000, 2001, 2007 Brian Gough
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 3 of the License, or (at
// your option) any later version.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301, USA.

use crate::qags::qags;
use crate::result::IntegrationResult;

/// Integrate a function over an infinite, semi-infinit or finite interval by
/// transforming the integrand to a new function defined over a finite interval
/// (if nesseccary) and using `qags`.
#[allow(clippy::too_many_arguments)]
pub fn qagi<F>(
    f: F,
    a: f64,
    b: f64,
    epsabs: f64,
    epsrel: f64,
    limit: usize,
    key: u8,
) -> IntegrationResult
where
    F: Fn(f64) -> f64,
{
    let sign = if a < b { 1.0 } else { -1.0 };
    let (aa, bb) = if a < b { (a, b) } else { (b, a) };

    if bb >= f64::INFINITY && aa <= f64::NEG_INFINITY {
        // Integrate from -inf to inf by transforming the integrand using:
        // f(t) -> [f( (1 - t) / t ) + f( -(1 - t) / t)] / t^2
        let transformed = |t: f64| -> f64 {
            let x = (1.0 - t) / t;
            sign * (f(x) + f(-x)) / (t * t)
        };
        qags(transformed, 0.0, 1.0, epsabs, epsrel, limit, key)
    } else if bb >= f64::INFINITY && aa > f64::NEG_INFINITY {
        // Integrate from a to inf by transforming the integrand using:
        // f(t) -> f( a + (1 - t) / t ) / t^2
        let transformed = |t: f64| -> f64 {
            let x = aa + (1.0 - t) / t;
            sign * f(x) / (t * t)
        };
        qags(transformed, 0.0, 1.0, epsabs, epsrel, limit, key)
    } else if aa <= f64::NEG_INFINITY && bb < f64::INFINITY {
        // Integrate from -inf to a by transforming the integrand using:
        // f(t) -> f( a + (1 - t) / t ) / t^2
        let transformed = |t: f64| -> f64 {
            let x = bb - (1.0 - t) * t.recip();
            sign * f(x) * t.powi(2).recip()
        };
        qags(transformed, 0.0, 1.0, epsabs, epsrel, limit, key)
    } else {
        let transformed = |t: f64| -> f64 {
            let x = t * bb + aa * (1.0 - t);
            sign * f(x) * (bb - aa)
        };
        qags(transformed, 0.0, 1.0, epsabs, epsrel, limit, key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;

    #[test]
    fn test_0_inf_f455() {
        let exp_result = -3.616892186127022568E-01;
        let exp_abserr = 3.016716913328831851E-06;

        let result = qagi(f455, 0.0, f64::INFINITY, 0.0, 1e-3, 1000, 1);

        test_rel(result.val, exp_result, 1e-14);
        test_rel(result.err, exp_abserr, 1e-5);
    }

    #[test]
    fn test_0_inf_f15() {
        let exp_result = 6.553600000000024738E+04;
        let exp_abserr = 7.121667111456009280E-04;

        let alpha = 5.0;
        let f = |x| f15(x, alpha);
        let result = qagi(f, 0.0, f64::INFINITY, 0.0, 1e-7, 1000, 1);

        test_rel(result.val, exp_result, 1e-14);
        test_rel(result.err, exp_abserr, 1e-5);
    }

    #[test]
    fn test_0_inf_f16() {
        let exp_result = 1.000000000006713292E-04;
        let exp_abserr = 3.084062020905636316E-09;

        let alpha = 1.0;
        let f = |x| f16(x, alpha);
        let result = qagi(f, 99.9, f64::INFINITY, 1e-7, 0.0, 1000, 1);

        test_rel(result.val, exp_result, 1e-14);
        test_rel(result.err, exp_abserr, 1e-5);
    }

    #[test]
    fn test_inf_inf() {
        let exp_result = 2.275875794468747770E+00;
        let exp_abserr = 7.436490118267390744E-09;

        let f = |x: f64| (-x - x * x).exp();

        let result = qagi(f, f64::NEG_INFINITY, f64::INFINITY, 1e-7, 0.0, 1000, 2);

        test_rel(result.val, exp_result, 1e-14);
        test_rel(result.err, exp_abserr, 1e-5);
    }

    #[test]
    fn test_inf_0() {
        let exp_result = 2.718281828459044647E+00;
        let exp_abserr = 1.588185109253204805E-10;

        let alpha = 1.0;
        let f = |x: f64| (alpha * x).exp();

        let result = qagi(f, f64::NEG_INFINITY, 1.0, 1e-7, 0.0, 1000, 1);

        test_rel(result.val, exp_result, 1e-14);
        test_rel(result.err, exp_abserr, 1e-5);
    }
}
