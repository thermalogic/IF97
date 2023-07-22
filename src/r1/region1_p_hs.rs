//!  Region 1 - Backward Equation(h,s)->p
//！ * http://www.iapws.org/relguide/Supp-PHS12-2014.pdf

use crate::algo::*;
use crate::common::constant::*;
use crate::r1::region1_T_phps::*;
use crate::r1::region1_pT::*;

/// helper for iter (h,s)->p
fn ph2s_reg1(p: f64, h: f64) -> f64 {
    let T: f64 = ph2T_reg1(p, h);
    pT2s_reg1(p, T)
}

///----------------------------------------------------------------
///  Backward equation p(h,s) for region 1
///----------------------------------------------------------------
pub fn hs2p_reg1(h: f64, s: f64) -> f64 {
    // Page 5, Table 2 :
    // Initialize coefficients and exponents (H,S)->P for region 1
    const IJn: [(i32, i32, f64); 19] = [
        (0, 0, -0.691997014660582),
        (0, 1, -0.183612548787560e2),
        (0, 2, -0.928332409297335e1),
        (0, 4, 0.659639569909906e2),
        (0, 5, -0.162060388912024e2),
        (0, 6, 0.450620017338667e3),
        (0, 8, 0.854680678224170e3),
        (0, 14, 0.607523214001162e4),
        (1, 0, 0.326487682621856e2),
        (1, 1, -0.269408844582931e2),
        (1, 4, -0.319947848334300e3),
        (1, 6, -0.928354307043320e3),
        (2, 0, 0.303634537455249e2),
        (2, 1, -0.650540422444146e2),
        (2, 10, -0.430991316516130e4),
        (3, 4, -0.747512324096068e3),
        (4, 1, 0.730000345529245e3),
        (4, 4, 0.114284032569021e4),
        (5, 0, -0.436407041874559e3),
    ];

    let eta: f64 = h / 3400.0 + 0.05;
    let sigma: f64 = s / 7.6 + 0.05;
    let mut pi: f64 = sum_power(eta, sigma, &IJn);
    // 100.0*pi

    // iteration: refine

    let mut p: f64;
    let p1 = 100.0 * pi;
    let f1 = s - ph2s_reg1(p1, h);
    let mut p2: f64;
    if f1.abs() > ESP {
        if f1 > 0.0
        // pT2sreg1(p,h)< s ,the p1< expt p，so， p2=1.05*p1 p（p1,p2)
        {
            p2 = (1.0 + f1 / s) * p1;
        } else {
            p2 = (1.0 - f1 / s) * p1;
        }

        let f2: f64 = s - ph2s_reg1(p2, h);
        p = rtsec1(ph2s_reg1, h, s, p1, p2, f1, f2, ESP, I_MAX);
    } else {
        p = p1;
    };
    p
}
