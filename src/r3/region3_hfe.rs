//! Region 3 :the dimensionless Helmholtz free energy
//!  *  tau :dimensionless temperature K
//1  *  delta: dimensionless density kg/m3

use crate::algo::*;
use crate::common::constant::*;

//  Initialize coefphicients and exponents for region 3(Table 30, page 30)
pub const n1: f64 = 0.10658070028513e1;

pub const IJn: [(i32, i32, f64); 39] = [
    (0, 0, -0.15732845290239E+02),
    (0, 1, 0.20944396974307E+02),
    (0, 2, -0.76867707878716E+01),
    (0, 7, 0.26185947787954E+01),
    (0, 10, -0.28080781148620E+01),
    (0, 12, 0.12053369696517E+01),
    (0, 23, -0.84566812812502E-02),
    (1, 2, -0.12654315477714E+01),
    (1, 6, -0.11524407806681E+01),
    (1, 15, 0.88521043984318E+00),
    (1, 17, -0.64207765181607E+00),
    (2, 0, 0.38493460186671E+00),
    (2, 2, -0.85214708824206E+00),
    (2, 6, 0.48972281541877E+01),
    (2, 7, -0.30502617256965E+01),
    (2, 22, 0.39420536879154E-01),
    (2, 26, 0.12558408424308E+00),
    (3, 0, -0.27999329698710E+00),
    (3, 2, 0.13899799569460E+01),
    (3, 4, -0.20189915023570E+01),
    (3, 16, -0.82147637173963E-02),
    (3, 26, -0.47596035734923E+00),
    (4, 0, 0.43984074473500E-01),
    (4, 2, -0.44476435428739E+00),
    (4, 4, 0.90572070719733E+00),
    (4, 26, 0.70522450087967E+00),
    (5, 1, 0.10770512626332E+00),
    (5, 3, -0.32913623258954E+00),
    (5, 26, -0.50871062041158E+00),
    (6, 0, -0.22175400873096E-01),
    (6, 2, 0.94260751665092E-01),
    (6, 26, 0.16436278447961E+00),
    (7, 2, -0.13503372241348E-01),
    (8, 26, -0.14834345352472E-01),
    (9, 2, 0.57922953628084E-03),
    (9, 26, 0.32308904703711E-02),
    (10, 0, 0.80964802996215E-04),
    (10, 1, -0.16557679795037E-03),
    (11, 26, -0.44923899061815E-04),
];

/// Fundamental equation for region 3
pub fn phi_reg3(tau: f64, delta: f64) -> f64 {
    let mut result: f64 = n1 * delta.ln();
    result + sum_power(delta, tau, &IJn)
}

/// First derivative in delta of fundamental equation for region 3
pub fn phi_delta_reg3(tau: f64, delta: f64) -> f64 {
    let mut result: f64 = n1 / delta;
    result + sum_di_power(delta, tau, &IJn)
}

/// Second derivative in delta of fundamental equation for region 3
pub fn phi_deltadelta_reg3(tau: f64, delta: f64) -> f64 {
    let mut result: f64 = -n1 / delta / delta;
    result + sum_dii_power(delta, tau, &IJn)
}

/// First derivative in tau of fundamental equation for region 3
pub fn phi_tau_reg3(tau: f64, delta: f64) -> f64 {
    let mut result: f64 = 0.0;
    sum_dj_power(delta, tau, &IJn)
}

/// Second derivative in tau of fundamental equation for region 3
pub fn phi_tautau_reg3(tau: f64, delta: f64) -> f64 {
    let mut result: f64 = 0.0;
    sum_djj_power(delta, tau, &IJn)
}

/// Second derivative in delta and tau of fundamental equation for region 3
pub fn phi_deltatau_reg3(tau: f64, delta: f64) -> f64 {
    let mut result: f64 = 0.0;
    sum_dij_power(delta, tau, &IJn)
}
