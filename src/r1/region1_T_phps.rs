//! Region 1 - Backward Equation (p,h)->T (p,s)->T
//！* R7-97(2012) August 2007 : http://www.iapws.org/relguide/IF97-Rev.html
//！  * Page 10, Eq(11) (p,h)->T
//!   * Page 11, Eq(13) (p,s)->T

use crate::algo::*;
use crate::common::constant::*;
use crate::r1::region1_pT::*;

/// Backward equation T(p,h) for region 1
pub fn ph2T_reg1(p: f64, h: f64) -> f64 {
    // Page 11, Table6 :Initialize coefficients and exponents (P,H)->T for region 1
    const IJn: [(i32, i32, f64); 20] = [
        (0, 0, -238.72489924521),
        (0, 1, 404.21188637945),
        (0, 2, 113.49746881718),
        (0, 6, -5.8457616048039),
        (0, 22, -1.528548241314E-04),
        (0, 32, -1.0866707695377E-06),
        (1, 0, -13.391744872602),
        (1, 1, 43.211039183559),
        (1, 2, -54.010067170506),
        (1, 3, 30.535892203916),
        (1, 4, -6.5964749423638),
        (1, 10, 9.3965400878363E-03),
        (1, 32, 1.157364750534E-07),
        (2, 10, -2.5858641282073E-05),
        (2, 32, -4.0644363084799E-09),
        (3, 10, 6.6456186191635E-08),
        (3, 32, 8.0670734103027E-11),
        (4, 32, -9.3477771213947E-13),
        (5, 32, 5.8265442020601E-15),
        (6, 32, -1.5020185953503E-17),
    ];

    let pi: f64 = p / 1.0;
    let eta: f64 = h / 2500.0 + 1.0;
    let mut theta: f64 = sum_power(pi, eta, &IJn);
    // return 1.0 *theta;

    // iteration: refine
    let T1: f64 = 1.0 * theta;
    let f1: f64 = h - pT2h_reg1(p, T1);
    let mut T2: f64;
    let mut T: f64;
    if f1.abs() > ESP {
        if f1 > 0.0 {
            T2 = (1.0 + f1 / h) * T1;
        }
        else {
            T2 = (1.0 - f1 / h) * T1;
        }

        let f2: f64 = h - pT2h_reg1(p, T2);
        T = rtsec2(pT2h_reg1, p, h, T1, T2, f1, f2, ESP, I_MAX);
    } else {
        T = T1;
    };

    return T;
}

///  Backward equation T(p,s) for region 1
pub fn ps2T_reg1(p: f64, s: f64) -> f64 {
    // Page 12, Table 8 : Initialize coefficients and exponents (P,S)->T for region 1
    const IJn: [(i32, i32, f64); 20] = [
        (0, 0, 0.17478268058307e3),
        (0, 1, 0.34806930892873e2),
        (0, 2, 0.65292584978455e1),
        (0, 3, 0.33039981775489),
        (0, 11, -0.19281382923196e-6),
        (0, 31, -0.24909197244573e-22),
        (1, 0, -0.26107636489332),
        (1, 1, 0.22592965981586),
        (1, 2, -0.64256463395226e-1),
        (1, 3, 0.78876289270526e-2),
        (1, 12, 0.35672110607366e-9),
        (1, 31, 0.17332496994895e-23),
        (2, 0, 0.56608900654837e-3),
        (2, 1, -0.32635483139717e-3),
        (2, 2, 0.44778286690632e-4),
        (2, 9, -0.51322156908507e-9),
        (2, 31, -0.42522657042207e-25),
        (3, 10, 0.26400441360689e-12),
        (3, 32, 0.78124600459723e-28),
        (4, 32, -0.30732199903668e-30),
    ];

    let pi: f64 = p / 1.0;
    let sigma: f64 = s / 1.0 + 2.0;
    let mut theta: f64 = sum_power(pi, sigma, &IJn);
    //return 1.0*theta;

    // iteration: refine
    let T1: f64 = 1.0 * theta;
    let f1: f64 = s - pT2s_reg1(p, T1);
    let T2: f64;
    let mut T: f64;
    if f1.abs() > ESP {
        if f1 > 0.0
        // pT2s_reg1(p,T1)< s ,the T1< expt T，so， T2=1.05*T1 T（T1,T2)
        {
            T2 = (1.0 + f1 / s) * T1;
        } else {
            T2 = (1.0 - f1 / s) * T1;
        }

        let f2: f64 = s - pT2s_reg1(p, T2);
        T = rtsec2(pT2s_reg1, p, s, T1, T2, f1, f2, ESP, I_MAX);
    } else {
        T = T1;
    }
    T
}
