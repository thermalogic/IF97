//！ Backward Equation for Region 2: (P,H)->T
//！ * 6.3.1 The Backward Equations T( p, h ) for Subregions 2a, 2b, and 2c.
//!     *  ph2T_reg2(p,h)
use crate::algo::*;
use crate::common::constant::*;
use crate::r2::region2_pT::*;

pub fn ph2T_reg2a(p: f64, h: f64) -> f64 {
    // Table 20 the coefficients and exponents of  the backward equation T ( p,h ) for subregion 2a, Eq.(22)
    const IJn: [(i32, i32, f64); 34] = [
        (0, 0, 0.10898952318288E+04),
        (0, 1, 0.84951654495535E+03),
        (0, 2, -0.10781748091826E+03),
        (0, 3, 0.33153654801263E+02),
        (0, 7, -0.74232016790248E+01),
        (0, 20, 0.11765048724356E+02),
        (1, 0, 0.18445749355790E+01),
        (1, 1, -0.41792700549624E+01),
        (1, 2, 0.62478196935812E+01),
        (1, 3, -0.17344563108114E+02),
        (1, 7, -0.20058176862096E+03),
        (1, 9, 0.27196065473796E+03),
        (1, 11, -0.45511318285818E+03),
        (1, 18, 0.30919688604755E+04),
        (1, 44, 0.25226640357872E+06),
        (2, 0, -0.61707422868339E-02),
        (2, 2, -0.31078046629583E+00),
        (2, 7, 0.11670873077107E+02),
        (2, 36, 0.12812798404046E+09),
        (2, 38, -0.98554909623276E+09),
        //
        (2, 40, 0.28224546973002E+10),
        (2, 42, -0.35948971410703E+10),
        (2, 44, 0.17227349913197E+10),
        (3, 24, -0.13551334240775E+05),
        (3, 44, 0.12848734664650E+08),
        (4, 12, 0.13865724283226E+01),
        (4, 32, 0.23598832556514E+06),
        (4, 44, -0.13105236545054E+08),
        (5, 32, 0.73999835474766E+04),
        (5, 36, -0.55196697030060E+06),
        (5, 42, 0.37154085996233E+07),
        (6, 34, 0.19127729239660E+05),
        (6, 44, -0.41535164835634E+06),
        (7, 28, -0.62459855192507E+02),
    ];

    let pi: f64 = p / 1.0;
    let eta: f64 = h / 2000.0 - 2.1;
    1.0 * sum_power(pi, eta, &IJn)
}

pub fn ph2T_reg2b(p: f64, h: f64) -> f64 {
    // Table 21. the coefficients and exponents of the backward equation T ( p,h ) for  subregion 2b Eq.(23)
    const IJn: [(i32, i32, f64); 38] = [
        (0, 0, 1.4895041079516e3),
        (0, 1, 7.4307798314034e2),
        (0, 2, -9.7708318797837e1),
        (0, 12, 2.4742464705674),
        (0, 18, -6.3281320016026e-1),
        (0, 24, 1.1385952129658),
        (0, 28, -4.7811863648625e-1),
        (0, 40, 8.5208123431544e-3),
        (1, 0, 9.3747147377932e-1),
        (1, 2, 3.3593118604916),
        (1, 6, 3.3809355601454),
        (1, 12, 1.6844539671904e-1),
        (1, 18, 7.3875745236695e-1),
        (1, 24, -4.7128737436186e-1),
        (1, 28, 1.5020273139707e-1),
        (1, 40, -2.1764114219750e-3),
        (2, 2, -2.1810755324761e-2),
        (2, 8, -1.0829784403677e-1),
        (2, 18, -4.6333324635812e-2),
        (2, 40, 7.1280351959551e-5),
        (3, 1, 1.1032831789999e-4),
        (3, 2, 1.8955248387902e-4),
        (3, 12, 3.0891541160537e-3),
        (3, 24, 1.3555504554949e-3),
        (4, 2, 2.8640237477456e-7),
        (4, 12, -1.0779857357512e-5),
        (4, 18, -7.6462712454814e-5),
        (4, 24, 1.4052392818316e-5),
        (4, 28, -3.1083814331434e-5),
        (4, 40, -1.0302738212103e-6),
        (5, 18, 2.8217281635040e-7),
        (5, 24, 1.2704902271945e-6),
        (5, 40, 7.3803353468292e-8),
        (6, 28, -1.1030139238909e-8),
        (7, 2, -8.1456365207833e-14),
        (7, 28, -2.5180545682962e-11),
        (9, 1, -1.7565233969407e-18),
        (9, 40, 8.6934156344163e-15),
    ];

    let pi: f64 = p / 1.0 - 2.0;
    let eta: f64 = h / 2000.0 - 2.6;
    1.0 * sum_power(pi, eta, &IJn)
}

pub fn ph2T_reg2c(p: f64, h: f64) -> f64 {
    // Table 22. the coefficients and exponents of the backward  equation T(p,h) for subregion 2c, Eq.(24)
    const IJn: [(i32, i32, f64); 23] = [
        (-7, 0, -3236839855524.2),
        (-7, 4, 7326335090218.1),
        (-6, 0, 358250899454.47),
        (-6, 2, -583401318515.9),
        (-5, 0, -10783068217.47),
        //
        (-5, 2, 20825544563.171),
        (-2, 0, 610747.83564516),
        (-2, 1, 859777.2253558),
        (-1, 0, -25745.72360417),
        (-1, 2, 31081.088422714),
        (0, 0, 1208.2315865936),
        (0, 1, 482.19755109255),
        (1, 4, 3.7966001272486),
        (1, 8, -10.842984880077),
        (2, 4, -0.04536417267666),
        //
        (6, 0, 1.4559115658698E-13),
        (6, 1, 1.126159740723E-12),
        (6, 4, -1.7804982240686E-11),
        (6, 10, 1.2324579690832E-07),
        (6, 12, -1.1606921130984E-06),
        (6, 16, 2.7846367088554E-05),
        (6, 20, -5.9270038474176E-04),
        (6, 22, 1.2918582991878E-03),
    ];

    let pi: f64 = p / 1.0 + 25.0;
    let eta: f64 = h / 2000.0 - 1.8;
    1.0 * sum_power(pi, eta, &IJn)
}

/// Eq21 in <http://www.iapws.org/relguide/IF97-Rev.html>
pub fn enthalpy_2bc(p: f64) -> f64 {
    const n: [f64; 3] = [0.12809002730136e-3, 0.26526571908428e4, 0.45257578905948e1];
    return n[1] + ((p - n[2]) / n[0]).sqrt();
}

pub fn ph2T_reg2(p: f64, h: f64) -> f64 {
    let mut T: f64 = 0.0;

    if p > 4.0 {
        if h < enthalpy_2bc(p) {
            T = ph2T_reg2c(p, h);
        } else {
            T = ph2T_reg2b(p, h);
        }
    } else {
        T = ph2T_reg2a(p, h);
    }
    // T

    let T1: f64 = T;
    let f1: f64 = h - pT2h_reg2(p, T1);
    if f1.abs() > ESP {
        let mut T2: f64 = 0.0;
        if f1 > 0.0 {
            T2 = (1.0 + f1 / h) * T1;
        }
        else {
            T2 = (1.0 - f1 / h) * T1;
        }
        let f2: f64 = h - pT2h_reg2(p, T2);
        T = rtsec2(pT2h_reg2, p, h, T1, T2, f1, f2, ESP, I_MAX);
    };
    T
}
