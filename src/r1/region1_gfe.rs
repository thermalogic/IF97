//！IAPWS-IF97 Basic Equation for Region 1
//! * The basic equation of the dimensionless Gibbs free energy(p,T) and its derivatives
//！   Eq.(7), P6 http://www.iapws.org/relguide/IF97-Rev.html

use crate::algo::*;
use crate::common::constant::*;

pub const r1pstar: f64 = 16.53; // MPa
pub const r1Tstar: f64 = 1386.0; // K

//  Initialize coefficients and exponents for region 1
pub const IJn: [(i32, i32, f64); 34] = [
    (0, -2, 0.14632971213167E+00),
    (0, -1, -0.84548187169114E+00),
    (0, 0, -0.37563603672040E+01),
    (0, 1, 0.33855169168385E+01),
    (0, 2, -0.95791963387872E+00),
    (0, 3, 0.15772038513228E+00),
    (0, 4, -0.16616417199501E-01),
    (0, 5, 0.81214629983568E-03),
    (1, -9, 0.28319080123804E-03),
    (1, -7, -0.60706301565874E-03),
    (1, -1, -0.18990068218419E-01),
    (1, 0, -0.32529748770505E-01),
    (1, 1, -0.21841717175414E-01),
    (1, 3, -0.52838357969930E-04),
    (2, -3, -0.47184321073267E-03),
    (2, 0, -0.30001780793026E-03),
    (2, 1, 0.47661393906987E-04),
    (2, 3, -0.44141845330846E-05),
    (2, 17, -0.72694996297594E-15),
    (3, -4, -0.31679644845054E-04),
    (3, 0, -0.28270797985312E-05),
    (3, 6, -0.85205128120103E-09),
    (4, -5, -0.22425281908000E-05),
    (4, -2, -0.65171222895601E-06),
    (4, 10, -0.14341729937924E-12),
    (5, -8, -0.40516996860117E-06),
    (8, -11, -0.12734301741641E-08),
    (8, -6, -0.17424871230634E-09),
    (21, -29, -0.68762131295531E-18),
    (23, -31, 0.14478307828521E-19),
    (29, -38, 0.26335781662795E-22),
    (30, -39, -0.11947622640071E-22),
    (31, -40, 0.18228094581404E-23),
    (32, -41, -0.93537087292458E-25),
];

/// Fundamental equation for region 1
pub fn gamma_reg1(mut tau: f64, mut pi: f64) -> f64 {
    sum_power(7.1 - pi, tau - 1.222, &IJn)
}

/// First derivative of fundamental equation in pi for region 1
pub fn gamma_pi_reg1(mut tau: f64, mut pi: f64) -> f64 {
     -sum_di_power(7.1 - pi, tau - 1.222, &IJn)
}

/// Second derivative of fundamental equation in pi for region 1
pub fn gamma_pipi_reg1(mut tau: f64, mut pi: f64) -> f64 {
    sum_dii_power(7.1 - pi, tau - 1.222, &IJn)
}

/// First derivative of fundamental equation in tau for region 1
pub fn gamma_tau_reg1(mut tau: f64, mut pi: f64) -> f64 {
    sum_dj_power(7.1 - pi, tau - 1.222, &IJn)
}

/// Second derivative of fundamental equation in tau for region 1
pub fn gamma_tautau_reg1(mut tau: f64, mut pi: f64) -> f64 {
    sum_djj_power(7.1 - pi, tau - 1.222, &IJn)
}

/// Second derivative of fundamental equation in pi and tau for region 1
pub fn gamma_pitau_reg1(mut tau: f64, mut pi: f64) -> f64 {
    -sum_dij_power(7.1 - pi, tau - 1.222, &IJn)
}

