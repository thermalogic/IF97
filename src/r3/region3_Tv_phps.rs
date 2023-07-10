//! Backward Equation for Region 3:
//!   Supp-Tv(ph,ps)3-2014.pdf
//!        (p,h) -T,v    (p,s)->T,v
 
use crate::common::constant::*;
use crate::algo::fast_ipower::sac_pow;
use crate::r3::region3_v_subregion_pT::*;

/// the boundary /between subregions 3a and 3b
fn p2h_3ab(p:f64)->f64
{
  //Page 6 Table 2. the coefficients of the equation h3ab(p) inits dimensionless form, Eq. (1)
  //  for defining the boundary /between subregions 3a and 3b
  const n:[f64;4] = [0.201464004206875E+04,
                        0.374696550136983E+01,
                        -0.219921901054187E-01,
                        0.875131686009950E-04];
  let pi:f64 = p / 1.0;
  let eta:f64 = n[0] + pi * (n[1] + pi * (n[2] + n[3] * pi));
  return eta * 1.0;
}

/// (p,h)->T for region 3a 
pub fn ph2T_3a_reg3(p:f64, h:f64)->f64
{
  // Page7 Table 3.  Coefficients and exponents of the equation T3a(p,h) for subregion 3a in its dimensionless form, Eq. (2)
  const IJn:[(i32,i32,f64);31] = [
      (-12, 0, -1.33645667811215e-7),
      (-12, 1, 4.55912656802978e-6),
      (-12, 2, -1.46294640700979e-5),
      (-12, 6, 6.39341312970080e-3),
      (-12, 14, 3.72783927268847e2),

      (-12, 16, -7.18654377460447e3),
      (-12, 20, 5.73494752103400e5),
      (-12, 22, -2.67569329111439e6),
      (-10, 1, -3.34066283302614e-5),
      (-10, 5, -2.45479214069597e-2),

      (-10, 12, 4.78087847764996e1),
      (-8, 0, 7.64664131818904e-6),
      (-8, 2, 1.28350627676972e-3),
      (-8, 4, 1.71219081377331e-2),
      (-8, 10, -8.51007304583213),

      (-5, 2, -1.36513461629781e-2),
      (-3, 0, -3.84460997596657e-6),
      (-2, 1, 3.37423807911655e-3),
      (-2, 3, -5.51624873066791e-1),
      (-2, 4, 7.29202277107470e-1),

      (-1, 0, -9.92522757376041e-3),
      (-1, 2, -1.19308831407288e-1),
      (0, 0, 7.93929190615421e-1),
      (0, 1, 4.54270731799386e-1),
      (1, 1, 2.09998591259910e-1),

      (3, 0, -6.42109823904738e-3),
      (3, 1, -2.35155868604540e-2),
      (4, 0, 2.52233108341612e-3),
      (4, 3, -7.64885133368119e-3),
      (10, 4, 1.36176427574291e-2),

      (12, 5, -1.33027883575669e-2)];
  
  let pi:f64 = p / 100.0+0.240;
  let eta:f64 = h / 2300.0-0.615;
  let mut theta:f64 = 0.0;
  for k in 0..31
  {
    theta += IJn[k].2 as f64 * sac_pow(pi, IJn[k].0) * sac_pow(eta, IJn[k].1);
  }
  return 760.0 * theta;

}

/// (p,h)->T for region 3b 
pub fn ph2T_3b_reg3(p:f64, h:f64)->f64
{
  // Table 4. Coefficients and exponents of the equation T3b=(p,h) for subregion 3b in its dimensionless form, Eq. (3)
  const IJn:[(i32,i32,f64);33] = [
      (-12, 0, 3.23254573644920e-5),
      (-12, 1, -1.27575556587181e-4),
      (-10, 0, -4.75851877356068e-4),
      (-10, 1, 1.56183014181602e-3),
      (-10, 5, 1.05724860113781e-1),

      (-10, 10, -8.58514221132534e1),
      (-10, 12, 7.24140095480911e2),
      (-8, 0, 2.96475810273257e-3),
      (-8, 1, -5.92721983365988e-3),
      (-8, 2, -1.26305422818666e-2),

      (-8, 4, -1.15716196364853e-1),
      (-8, 10, 8.49000969739595e1),
      (-6, 0, -1.08602260086615e-2),
      (-6, 1, 1.54304475328851e-2),
      (-6, 2, 7.50455441524466e-2),

      (-4, 0, 2.52520973612982e-2),
      (-4, 1, -6.02507901232996e-2),
      (-3, 5, -3.07622221350501),
      (-2, 0, -5.74011959864879e-2),
      (-2, 4, 5.03471360939849),

      (-1, 2, -9.25081888584834e-1),
      (-1, 4, 3.91733882917546),
      (-1, 6, -7.73146007130190e1),
      (-1, 10, 9.49308762098587e3),
      (-1, 14, -1.41043719679409e6),

      (-1, 16, 8.49166230819026e6),
      (0, 0, 8.61095729446704e-1),
      (0, 2, 3.23346442811720e-1),
      (1, 1, 8.73281936020439e-1),
      (3, 1, -4.36653048526683e-1),

      (5, 1, 2.86596714529479e-1),
      (6, 1, -1.31778331276228e-1),
      (8, 1, 6.76682064330275e-3)];
  
  let pi:f64 = p / 100.0+0.298;
  let eta:f64 = h / 2800.0-0.720;
  let mut theta:f64 = 0.0;
  for k in 0..33 {
    theta += IJn[k].2 * sac_pow(pi, IJn[k].0) * sac_pow(eta, IJn[k].1);
  }
  return 860.0*theta
}

/// Region 3a (p,h)->v
fn ph2v_3a_reg3(p:f64, h:f64)->f64
{
  //Page9 Table 6. Coefficients and exponents of the  equation v3a(p,h) for subregion 3a in its dimensionless form, Eq. (4)
  const IJn:[(i32,i32,f64);32] = [
      (-12, 6, 5.29944062966028e-3),
      (-12, 8, -1.70099690234461e-1),
      (-12, 12, 1.11323814312927e1),
      (-12, 18, -2.17898123145125e3),
      (-10, 4, -5.06061827980875e-4),
      (-10, 7, 5.56495239685324e-1),
      (-10, 10, -9.43672726094016),
      (-8, 5, -2.97856807561527e-1),
      (-8, 12, 9.39353943717186e1),
      (-6, 3, 1.92944939465981e-2),
      (-6, 4, 4.21740664704763e-1),
      (-6, 22, -3.68914126282330e6),
      (-4, 2, -7.37566847600639e-3),
      (-4, 3, -3.54753242424366e-1),
      (-3, 7, -1.99768169338727),
      (-2, 3, 1.15456297059049),
      (-2, 16, 5.68366875815960e3),
      (-1, 0, 8.08169540124668e-3),
      (-1, 1, 1.72416341519307e-1),
      (-1, 2, 1.04270175292927),
      (-1, 3, -2.97691372792847e-1),
      (0, 0, 5.60394465163593e-1),
      (0, 1, 2.75234661176914e-1),
      (1, 0, -1.48347894866012e-1),
      (1, 1, -6.51142513478515e-2),
      (1, 2, -2.92468715386302),
      (2, 0, 6.64876096952665e-2),
      (2, 2, 3.52335014263844),
      (3, 0, -1.46340792313332e-2),
      (4, 2, -2.24503486668184),
      (5, 2, 1.10533464706142),
      (8, 2, -4.08757344495612e-2)];

  let pi:f64 = p / 100.0+0.128;
  let eta:f64 = h / 2100.0-0.727;
  let mut omega:f64 = 0.0;
  for k in 0..32
   { omega += IJn[k].2 * sac_pow(pi, IJn[k].0) * sac_pow(eta, IJn[k].1);}
  return 0.0028 *omega;
}

/// (p,h)->v 3b
pub fn ph2v_3b_reg3(p:f64, h:f64)->f64
{
  // Page 9 Table 7. Coefficients and exponents of the equation v3b (p,h) for subregion 3b in its/dimensionless form, Eq. (5)
  const IJn:[(i32,i32,f64);30] = [
      (-12, 0, -2.25196934336318e-9),
      (-12, 1, 1.40674363313486e-8),
      (-8, 0, 2.33784085280560e-6),
      (-8, 1, -3.31833715229001e-5),
      (-8, 3, 1.07956778514318e-3),
      (-8, 6, -2.71382067378863e-1),
      (-8, 7, 1.07202262490333),
      (-8, 8, -8.53821329075382e-1),
      (-6, 0, -2.15214194340526e-5),
      (-6, 1, 7.69656088222730e-4),
      (-6, 2, -4.31136580433864e-3),
      (-6, 5, 4.53342167309331e-1),
      (-6, 6, -5.07749535873652e-1),
      (-6, 10, -1.00475154528389e2),
      (-4, 3, -2.19201924648793e-1),
      (-4, 6, -3.21087965668917),
      (-4, 10, 6.07567815637771e2),
      (-3, 0, 5.57686450685932e-4),
      (-3, 2, 1.87499040029550e-1),
      (-2, 1, 9.05368030448107e-3),
      (-2, 2, 2.85417173048685e-1),
      (-1, 0, 3.29924030996098e-2),
      (-1, 1, 2.39897419685483e-1),
      (-1, 4, 4.82754995951394),
      (-1, 5, -1.18035753702231e1),
      (0, 0, 1.69490044091791e-1),
      (1, 0, -1.79967222507787e-2),
      (1, 1, 3.71810116332674e-2),
      (2, 2, -5.36288335065096e-2),
      (2, 6, 1.60697101092520)];

  let pi:f64 = p / 100.0+ 0.0661;
  let eta:f64 = h / 2800.0- 0.720;

  let mut omega:f64 = 0.0;
  for k in 0..30
  {  omega += IJn[k].2 * sac_pow(pi, IJn[k].0) * sac_pow(eta, IJn[k].1);}
  return 0.0088 *omega;
}

/// Region 3(3a,3b)  (p,h)->T
pub fn ph2T_reg3(p:f64, h:f64)->f64
{
  let h3ab:f64 = p2h_3ab(p);
  if h <= h3ab
  {
    return ph2T_3a_reg3(p, h);
  }
  else
  {
    return  ph2T_3b_reg3(p, h);
  }
}

pub fn ph2v_reg3(p:f64, h:f64)->f64
{
  let h3ab:f64 = p2h_3ab(p);
  if h <= h3ab
  {
    return ph2v_3a_reg3(p, h);
  }
  else
  {
    return ph2v_3b_reg3(p, h);
  } 
}

/// (p,s)->T   3a Page 13, Table 11  Page 12 Eq(7)
pub fn ps2T_3a_reg3(p:f64,s:f64)->f64
{
  const I:[i32;33] =[-12, -12, -10, -10, -10,
                      -10, -8, -8, -8, -8,
                      -6, -6, -6, -5, -5,
                      -5, -4, -4, -4, -2,
                      -2, -1, -1, 0, 0,
                      0, 1, 2, 2, 3,
                      8, 8, 10];

   const J:[i32;33] = [28, 32, 4, 10, 12,
                      14, 5, 7, 8, 28,
                      2, 6, 32, 0, 14,
                      32, 6, 10, 36, 1,
                      4, 1, 6, 0, 1,
                      4, 0, 0, 3, 2,
                      0, 1, 2];

   const n:[f64;33] = [0.150042008263875E+10,
                         -0.159397258480424E+12,
                         0.502181140217975E-03,
                         -0.672057767855466E+02,
                         0.145058545404456E+04,

                         -0.823889534888890E+04,
                         -0.154852214233853,
                         0.112305046746695E+02,
                         -0.297000213482822E+02,
                         0.438565132635495E+11,

                         0.137837838635464E-02,
                         -0.297478527157462E+01,
                         0.971777947349413E+13,
                         -0.571527767052398E-04,
                         0.288307949778420E+05,

                         -0.744428289262703E+14,
                         0.128017324848921E+02,
                         -0.368275545889071E+03,
                         0.664768904779177E+16,
                         0.449359251958880E-01,

                         -0.422897836099655E+01,
                         -0.240614376434179,
                         -0.474341365254924E+01,
                         0.724093999126110,
                         0.923874349695897,

                         0.399043655281015E+01,
                         0.384066651868009E-01,
                         -0.359344365571848E-02,
                         -0.735196448821653,
                         0.188367048396131,

                         0.141064266818704E-03,
                         -0.257418501496337E-02,
                         0.123220024851555E-02];
  let pi:f64 = p / 100.0+ 0.240;
  let sigma:f64 = s / 4.4- 0.703;
  let mut theta:f64=0.0;
  for k in 0..33
   { theta += n[k] * sac_pow(pi,I[k]) * sac_pow(sigma, J[k]);}
  return 760.0 * theta;
}

/// (p,s)->T   3b  Page 13, Table 12, Page 12 Eq(7)
fn ps2T_3b_reg3(p:f64,s:f64)->f64
{
    const I:[i32;28] =[-12, -12, -12, -12, -8,
                      -8, -8, -6, -6, -6,
                      -5, -5, -5, -5, -5,
                      -4, -3, -3, -2, 0,
                      2, 3, 4, 5, 6,
                      8, 12, 14];

    const J:[i32;28] = [1, 3, 4, 7, 0,
                      1, 3, 0, 2, 4,
                      0, 1, 2, 4, 6,
                      12, 1, 6, 2, 0,
                      1, 1, 0, 24, 0,
                      3, 1, 2];

    const n:[f64;28]= [0.527111701601660,
                         -0.401317830052742E+02,
                         0.153020073134484E+03,
                         -0.224799398218827E+04,
                         -0.193993484669048,

                         -0.140467557893768E+01,
                         0.426799878114024E+02,
                         0.752810643416743,
                         0.226657238616417E+02,
                         -0.622873556909932E+03,

                         -0.660823667935396,
                         0.841267087271658,
                         -0.253717501764397E+02,
                         0.485708963532948E+03,
                         0.880531517490555E+03,

                         0.265015592794626E+07,
                         -0.359287150025783,
                         -0.656991567673753E+03,
                         0.241768149185367E+01,
                         0.856873461222588,

                         0.655143675313458,
                         -0.213535213206406,
                         0.562974957606348E-02,
                         -0.316955725450471E+15,
                         -0.699997000152457E-03,

                         0.119845803210767E-01,
                         0.193848122022095E-04,
                         -0.215095749182309E-04];
  let pi:f64=p / 100.0+0.760;
  let sigma :f64=s / 5.3 - 0.818;
  let mut theta:f64=0.0;
  for k in 0..28 {
    theta += n[k] * sac_pow(pi, I[k]) * sac_pow(sigma, J[k]);}
  return 860.0 *theta
}

/// (p,s)->v  3a Page 15, Table 14  Page 14, Eq(8), Ok! 2003.12.18
fn  ps2v_3a_reg3(p:f64,s:f64)->f64
{
    const I:[i32;28] = [-12, -12, -12, -10, -10,
                      -10, -10, -8, -8, -8,
                      -8, -6, -5, -4, -3,
                      -3, -2, -2, -1, -1,
                      0, 0, 0, 1, 2,
                      4, 5, 6];

    const J:[i32;28]  = [10, 12, 14, 4, 8,
                      10, 20, 5, 6, 14,
                      16, 28, 1, 5, 2,
                      4, 3, 8, 1, 2,
                      0, 1, 3, 0, 0,
                      2, 2, 0];

    const n:[f64;28] = [0.795544074093975E+02,
                         -0.238261242984590E+04,
                         0.176813100617787E+05,
                         -0.110524727080379E-02,
                         -0.153213833655326E+02,

                         0.297544599376982E+03,
                         -0.350315206871242E+08,
                         0.277513761062119,
                         -0.523964271036888,
                         -0.148011182995403E+06,

                         0.160014899374266E+07,
                         0.170802322663427E+13,
                         0.246866996006494E-03,
                         0.165326084797980E+01,
                         -0.118008384666987,

                         0.253798642355900E+01,
                         0.965127704669424,
                         -0.282172420532826E+02,
                         0.203224612353823,
                         0.110648186063513E+01,

                         0.526127948451280,
                         0.277000018736321,
                         0.108153340501132E+01,
                         -0.744127885357893E-01,
                         0.164094443541384E-01,

                         -0.680468275301065E-01,
                         0.257988576101640E-01,
                         -0.145749861944416E-03];
  
  let pi:f64=p / 100.0+0.187;
  let sigma :f64=s / 4.4 - 0.755;
  let mut omega:f64 = 0.0;
  for k in 0..28 {
    omega += n[k] * sac_pow(pi, I[k]) * sac_pow(sigma, J[k]);}
  return 0.0028 * omega;
}

/// (p,s)->v  3b Page 15, Table 15  Page 14, Eq(9)   OK! 2003.12.18
fn ps2v_3b_reg3(p:f64,s:f64)->f64
{
  const I:[i32;31] = [-12, -12, -12, -12, -12,
                      -12, -10, -10, -10, -10,
                      -8, -5, -5, -5, -4,
                      -4, -4, -4, -3, -2,
                      -2, -2, -2, -2, -2,
                      0, 0, 0, 1, 1,
                      2];

  const J:[i32;31] = [0, 1, 2, 3, 5,
                      6, 0, 1, 2, 4,
                      0, 1, 2, 3, 0,
                      1, 2, 3, 1, 0,
                      1, 2, 3, 4, 12,
                      0, 1, 2, 0, 2,
                      2];

  const n:[f64;31] = [0.591599780322238E-04,
                         -0.185465997137856E-02,
                         0.104190510480013E-01,
                         0.598647302038590E-02,
                         -0.771391189901699,

                         0.172549765557036E+01,
                         -0.467076079846526E-03,
                         0.134533823384439E-01,
                         -0.808094336805495E-01,
                         0.508139374365767,

                         0.128584643361683E-02,
                         -0.163899353915435E+01,
                         0.586938199318063E+01,
                         -0.292466667918613E+01,
                         -0.614076301499537E-02,

                         0.576199014049172E+01,
                         -0.121613320606788E+02,
                         0.167637540957944E+01,
                         -0.744135838773463E+01,
                         0.378168091437659E-01,

                         0.401432203027688E+01,
                         0.160279837479185E+02,
                         0.317848779347728E+01,
                         -0.358362310304853E+01,
                         -0.115995260446827E+07,

                         0.199256573577909,
                         -0.122270624794624,
                         -0.191449143716586E+02,
                         -0.150448002905284E-01,
                         0.146407900162154E+02,

                         -0.327477787188230E+01];

  let pi:f64=p / 100.0 + 0.298;
  let sigma :f64=s / 5.3 - 0.816;
  let mut omega:f64 = 0.0;
  for k in 0..31 {
    omega += n[k] * sac_pow(pi, I[k]) * sac_pow(sigma, J[k]);}
  return 0.0088 *omega;
}


pub fn ps2T_reg3(p:f64,s:f64)->f64
{
  if (s <= SC_WATER)
  {
    return ps2T_3a_reg3(p, s);
  }
  else
  {
    return ps2T_3b_reg3(p, s);
  }
  
}

pub fn ps2v_reg3(p:f64,s:f64)->f64
{
  if (s <= SC_WATER)
  {
    return ps2v_3a_reg3(p, s);
  }
  else
  {
    return ps2v_3b_reg3(p, s);
  }

}
