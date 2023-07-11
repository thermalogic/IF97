//！
//！ common data for tests
//！
pub struct propD {
    pub p: f64,
    pub T: f64,
    pub v: f64,
    pub h: f64,
    pub s: f64,
    pub u: f64,
    pub cp: f64,
    pub w: f64,
}

// ------ region 1
// Table 5，Page 9: p,T,v,h,u,s,cp,w
pub const r1_pT_data: [propD; 3] = [
    propD {
        p: 3.0,
        T: 300.0,
        v: 0.100215168e-2,
        h: 0.115331273e+3,
        u: 0.112324818e3,
        s: 0.392294792,
        cp: 0.417301218e1,
        w: 0.150773921e4,
    },
    propD {
        p: 80.0,
        T: 300.0,
        v: 0.971180894e-3,
        h: 0.184142828e+3,
        u: 0.106448356e3,
        s: 0.368563852,
        cp: 0.401008987e1,
        w: 0.163469054e4,
    },
    propD {
        p: 3.0,
        T: 500.0,
        v: 0.120241800e-2,
        h: 0.975542239e+3,
        u: 0.971934985e3,
        s: 0.258041912e1,
        cp: 0.465580682e1,
        w: 0.124071337e4,
    },
];

pub const r1_phT: [[f64; 3]; 3] = [
    [3.0, 0.115331273e+3, 300.0],
    [80.0, 0.184142828e+3, 300.0],
    [3.0, 0.975542239e+3, 500.0],
];

pub const r1_psT: [[f64; 3]; 3] = [
    [3.0, 0.392294792, 300.0],
    [80.0, 0.368563852, 300.0],
    [3.0, 0.258041912e1, 500.0],
];

pub const r1_hspT: [[f64; 4]; 3] = [
    [0.115331273e+3, 0.392294792, 3.0, 300.0],
    [0.184142828e+3, 0.368563852, 80.0, 300.0],
    [0.975542239e+3, 0.258041912e1, 3.0, 500.0],
];

//
// ------ region 2--------------------------------------
//
//  Table15，Page17: p,T,v,h,u,s,cp,w
pub const r2_pT_data: [propD; 3] = [
    propD {
        p: 0.0035,
        T: 300.0,
        v: 0.394913866E+02,
        h: 0.254991145E+04,
        u: 0.241169160E+04,
        s: 0.852238967E+01,
        cp: 0.191300162E+01,
        w: 0.427920172E+03,
    },
    propD {
        p: 0.0035,
        T: 700.0,
        v: 0.923015898E+02,
        h: 0.333568375E+04,
        u: 0.301262819E+04,
        s: 0.101749996E+02,
        cp: 0.208141274E+01,
        w: 0.644289068E+03,
    },
    propD {
        p: 30.0,
        T: 700.0,
        v: 0.542946619E-02,
        h: 0.263149474E+04,
        u: 0.246861076E+04,
        s: 0.517540298E+01,
        cp: 0.103505092E+02,
        w: 0.480386523E+03,
    },
];

// Table24 a，Page25: p,h,T
pub const r2_ph_2a: [[f64; 3]; 3] = [
    [0.001, 3000.0, 0.534433241E+03],
    [3.0, 3000.0, 0.575373370E+03],
    [3.0, 4000.0, 0.101077577E+04],
];

// table24 b，Page25: p,h,T
pub const r2_ph_2b: [[f64; 3]; 3] = [
    [5.0, 3500.0, 0.801299102E+03],
    [5.0, 4000.0, 0.101531583E+04],
    [25.0, 3500.0, 0.875279054E+03],
];

// table24 c，Page25: p,h,T
pub const r2_ph_2c: [[f64; 3]; 3] = [
    [40.0, 2700.0, 0.743056411E+03],
    [60.0, 2700.0, 0.791137067E+03],
    [60.0, 3200.0, 0.882756860E+03],
];

// table29 a，Page29: p,s,T
pub const r2_ps_2a: [[f64; 3]; 3] = [
    [0.1, 7.5, 0.399517097E+03],
    [0.1, 8.0, 0.514127081E+03],
    [2.5, 8.0, 0.103984917E+04],
];

// table29 b，Page29: p,s,T
pub const r2_ps_2b: [[f64; 3]; 3] = [
    [8.0, 6.0, 0.600484040E+03],
    [8.0, 7.5, 0.106495556E+04],
    [90.0, 6.0, 0.103801126E+04],
];

// table29 c，Page29: p,s,T
pub const r2_ps_2c: [[f64; 3]; 3] = [
    [20.0, 5.75, 0.697992849E+03],
    [80.0, 5.25, 0.854011484E+03],
    [80.0, 5.75, 0.949017998E+03],
];

// Supp-PHS12-2014 ,table9 a，Page10: h,s,p
pub const r2_hs_reg2a: [[f64; 3]; 3] = [
    [2800.0, 6.5, 1.371012767],
    [2800.0, 9.5, 1.879743844e-03],
    [4100.0, 9.5, 1.024788997e-01],
];

// Supp-PHS12-2014 ,table9 b，Page10: h,s,p
pub const r2_hs_reg2b: [[f64; 3]; 3] = [
    [2800.0, 6.0, 4.793911442],
    [3600.0, 6.0, 8.395519209e+01], // `75.59971677713756?????
    [3600.0, 7.0, 7.527161441],
];

// Supp-PHS12-2014 ,table9 c Page10: h,s,p
pub const r2_hs_reg2c: [[f64; 3]; 3] = [
    [2800.0, 5.1, 9.439202060E+01],
    [2800.0, 5.8, 8.414574124],
    [3400.0, 5.8, 8.376903879e+01],
];

// --------------------------------------------------------- region 3
//Table 33. Thermodynamic property values calculated from Eq. (28) for selected values of T and  a
// T,d,p,h,u,s,cp,w
pub const r3_Td: [[f64; 8]; 3] = [
    [
        650.,
        500.,
        0.255837018E2,
        0.186343019E4,
        0.181226279E4,
        0.405427273E1,
        0.138935717E2,
        0.502005554E3,
    ],
    [
        650.,
        200.,
        0.222930643E2,
        0.237512401E4,
        0.226365868E4,
        0.485438792E1,
        0.446579342E2,
        0.383444594E3,
    ],
    [
        750.,
        500.,
        0.783095639E2,
        0.225868845E4,
        0.210206932E4,
        0.446971906E1,
        0.634165359E1,
        0.760696041E3,
    ],
];

//_Backward3_PT v， p,T
pub const r3_v_pT: [[f64; 3]; 52] = [
    [1.470853100e-3, 50.0, 630.0], // a
    [1.503831359e-3, 80.0, 670.0],
    [2.204728587e-3, 50.0, 710.0], // b
    [1.973692940e-3, 80.0, 750.0],
    [1.761696406e-3, 20.0, 630.0], // c
    [1.819560617e-3, 30.0, 650.0],
    [2.245587720e-3, 26.0, 656.0], // d
    [2.506897702e-3, 30.0, 670.0],
    [2.970225962e-3, 26.0, 661.0], // e
    [3.004627086e-3, 30.0, 675.0],
    [5.019029401e-3, 26.0, 671.0], // f
    [4.656470142e-3, 30.0, 690.0],
    [2.163198378e-3, 23.6, 649.0], // g
    [2.166044161e-3, 24.0, 650.0],
    [2.651081407e-3, 23.6, 652.0], // h
    [2.967802335e-3, 24.0, 654.0],
    [3.273916816e-3, 23.6, 653.0], // i
    [3.550329864e-3, 24.0, 655.0],
    [4.545001142e-3, 23.5, 655.0], // j
    [5.100267704e-3, 24.0, 660.0],
    [6.109525997e-3, 23.0, 660.0], // k
    [6.427325645e-3, 24.0, 670.0],
    [2.117860851e-3, 22.6, 646.0], // l
    [2.062374674e-3, 23.0, 646.0],
    [2.533063780e-3, 22.6, 648.6], // m
    [2.572971781e-3, 22.8, 649.3],
    [2.923432711e-3, 22.6, 649.0], // n
    [2.913311494e-3, 22.8, 649.7],
    [3.131208996e-3, 22.6, 649.1], // o
    [3.221160278e-3, 22.8, 649.9],
    [3.715596186e-3, 22.6, 649.4], // p
    [3.664754790e-3, 22.8, 650.2],
    [1.970999272e-3, 21.1, 640.0], // q
    [2.043919161e-3, 21.8, 643.0],
    [5.251009921e-3, 21.1, 644.0], // r
    [5.256844741e-3, 21.8, 648.0],
    [1.932829079e-3, 19.1, 635.0], // s
    [1.985387227e-3, 20.0, 638.0],
    [8.483262001e-3, 17.0, 626.0], // t
    [6.227528101e-3, 20.0, 640.0],
    [2.268366647e-3, 21.5, 644.6], // u
    [2.296350553e-3, 22.0, 646.1],
    [2.832373260e-3, 22.5, 648.6], // v
    [2.811424405e-3, 22.3, 647.9],
    [3.694032281e-3, 22.15, 647.5], // w
    [3.622226305e-3, 22.3, 648.1],
    [4.528072649e-3, 22.11, 648.0], // x
    [4.556905799e-3, 22.3, 649.0],
    [2.698354719e-3, 22.0, 646.84], // y
    [2.717655648e-3, 22.064, 647.05],
    [3.798732962e-3, 22.0, 646.89], // z
    [3.701940010e-3, 22.064, 647.15],
];

// Supp-Tv(ph,ps)3-2014.pdf
//      Page 8 Table 5. Selected temperature values calculated from Eqs. (2) and (3) a
//                  p,h->T,
//       Page 10Table 8. Selected specific volume values calculated from Eqs. (4) and (5) a
//                 p,h,->v     p,h,T,v
pub const r3_phTv_3a: [[f64; 4]; 3] = [
    [20.0, 1700.0, 6.293083892e2, 1.749903962e-3],
    [50.0, 2000.0, 6.905718338e2, 1.908139035e-3],
    [100.0, 2100.0, 7.336163014e2, 1.676229776e-3],
];
pub const r3_phTv_3b: [[f64; 4]; 3] = [
    [20.0, 2500.0, 6.418418053e2, 6.670547043e-3],
    [50.0, 2400.0, 7.351848618e2, 2.801244590e-3],
    [100.0, 2700.0, 8.420460876e2, 2.404234998e-3],
];

// Page13 Table 12. Selected temperature values calculated from Eqs. (6) and (7)a
// Page15 Table 15. Selected specific volume values calculated from Eqs. (8) and (9) a
// p,s,T,v
pub const r3_psTv_3a: [[f64; 4]; 3] = [
    [20.0, 3.8, 6.282959869e2, 1.733791463e-3],
    [50.0, 3.6, 6.297158726e2, 1.469680170e-3],
    [100.0, 4.0, 7.056880237e2, 1.555893131e-3],
];

pub const r3_psTv_3b: [[f64; 4]; 3] = [
    [20.0, 5.0, 6.401176443e2, 6.262101987e-3],
    [50.0, 4.5, 7.163687517e2, 2.332634294e-3],
    [100.0, 5.0, 8.474332825e2, 2.449610757e-3],
];

//_Backward3_P_hs Table 5 pag 10
// h,s,p
pub const r3_hsp_3a: [[f64; 3]; 3] = [
    [1700.0, 3.8, 25.5570324620],
    [2000.0, 4.2, 45.40873468],
    [2100.0, 4.3, 60.78123340100],
];

pub const r3_hsp_3b: [[f64; 3]; 3] = [
    [2600.0, 5.1, 34.34999263],
    [2400.0, 4.7, 63.6392488750],
    [2700.0, 5.0, 88.3904328],
];

//
// --- Region 4
//
// saturation T,p Table 35
pub const r4_sat_Tp: [[f64; 2]; 3] = [
    [300.0, 0.00353658941],
    [500.0, 2.63889776],
    [600.0, 12.3443146],
];

// saturation p,T Table 36
pub const r4_sat_pT: [[f64; 2]; 3] = [[0.1, 372.755919], [1.0, 453.035632], [10.0, 584.149488]];

//
// --- Region 5
//

//  Region 5: Table 42, Page 40 T,p  v,h,u,s,cp,w
pub const r5_pT_data: [[f64; 8]; 3] = [
    [
        1500.0,
        0.5,
        1.38455090,
        0.521976855e+4,
        4527.49310,
        9.65408875,
        2.61609445,
        917.068690,
    ],
    [
        1500.0,
        30.0,
        0.0230761299,
        5167.23514,
        4474.95124,
        7.72970133,
        2.72724317,
        928.548002,
    ],
    [
        2000.0,
        30.0,
        0.0311385219,
        6571.22604,
        5637.07038,
        8.53640523,
        2.88569882,
        1067.36948,
    ],
];