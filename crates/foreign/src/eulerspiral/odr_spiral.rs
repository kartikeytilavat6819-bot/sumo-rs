//! Port of foreign/eulerspiral/odrSpiral.{h,cpp}
//!
//! Copyright 2010 VIRES Simulationstechnologie GmbH, Apache License 2.0 (see LICENSE.TXT
//! next to the C++ sources). The Fresnel integrals use CEPHES coefficients (netlib.org/cephes).
//! The C++ out-parameters x, y, t are returned as a tuple.

const M_PI: f64 = 3.1415926535897932384626433832795;

/// S(x) for small x
const SN: [f64; 6] = [
    -2.99181919401019853726E3,
    7.08840045257738576863E5,
    -6.29741486205862506537E7,
    2.54890880573376359104E9,
    -4.42979518059697779103E10,
    3.18016297876567817986E11,
];
const SD: [f64; 6] = [
    2.81376268889994315696E2,
    4.55847810806532581675E4,
    5.17343888770096400730E6,
    4.19320245898111231129E8,
    2.24411795645340920940E10,
    6.07366389490084639049E11,
];

/// C(x) for small x
const CN: [f64; 6] = [
    -4.98843114573573548651E-8,
    9.50428062829859605134E-6,
    -6.45191435683965050962E-4,
    1.88843319396703850064E-2,
    -2.05525900955013891793E-1,
    9.99999999999999998822E-1,
];
const CD: [f64; 7] = [
    3.99982968972495980367E-12,
    9.15439215774657478799E-10,
    1.25001862479598821474E-7,
    1.22262789024179030997E-5,
    8.68029542941784300606E-4,
    4.12142090722199792936E-2,
    1.00000000000000000118E0,
];

/// Auxiliary function f(x)
const FN: [f64; 10] = [
    4.21543555043677546506E-1,
    1.43407919780758885261E-1,
    1.15220955073585758835E-2,
    3.45017939782574027900E-4,
    4.63613749287867322088E-6,
    3.05568983790257605827E-8,
    1.02304514164907233465E-10,
    1.72010743268161828879E-13,
    1.34283276233062758925E-16,
    3.76329711269987889006E-20,
];
const FD: [f64; 10] = [
    7.51586398353378947175E-1,
    1.16888925859191382142E-1,
    6.44051526508858611005E-3,
    1.55934409164153020873E-4,
    1.84627567348930545870E-6,
    1.12699224763999035261E-8,
    3.60140029589371370404E-11,
    5.88754533621578410010E-14,
    4.52001434074129701496E-17,
    1.25443237090011264384E-20,
];

/// Auxiliary function g(x)
const GN: [f64; 11] = [
    5.04442073643383265887E-1,
    1.97102833525523411709E-1,
    1.87648584092575249293E-2,
    6.84079380915393090172E-4,
    1.15138826111884280931E-5,
    9.82852443688422223854E-8,
    4.45344415861750144738E-10,
    1.08268041139020870318E-12,
    1.37555460633261799868E-15,
    8.36354435630677421531E-19,
    1.86958710162783235106E-22,
];
const GD: [f64; 11] = [
    1.47495759925128324529E0,
    3.37748989120019970451E-1,
    2.53603741420338795122E-2,
    8.14679107184306179049E-4,
    1.27545075667729118702E-5,
    1.04314589657571990585E-7,
    4.60680728146520428211E-10,
    1.10273215066240270757E-12,
    1.38796531259578871258E-15,
    8.39158816283118707363E-19,
    1.86958710162783236342E-22,
];

/// Evaluates the polynomial with leading coefficient coef[0] and n further coefficients
fn polevl(x: f64, coef: &[f64], n: usize) -> f64 {
    let mut ans = coef[0];
    for &c in &coef[1..=n] {
        ans = ans * x + c;
    }
    ans
}

/// Like polevl, but with an implicit leading coefficient of 1 (coef[0] is the constant term)
fn p1evl(x: f64, coef: &[f64], n: usize) -> f64 {
    let mut ans = x + coef[0];
    for &c in &coef[1..n] {
        ans = ans * x + c;
    }
    ans
}

/// Fresnel integrals; returns (ss, cc) for xxa
fn fresnel(xxa: f64) -> (f64, f64) {
    let x = xxa.abs();
    let mut x2 = x * x;
    let mut cc;
    let mut ss;

    if x2 < 2.5625 {
        let t = x2 * x2;
        ss = x * x2 * polevl(t, &SN, 5) / p1evl(t, &SD, 6);
        cc = x * polevl(t, &CN, 5) / polevl(t, &CD, 6);
    } else if x > 36974.0 {
        cc = 0.5;
        ss = 0.5;
    } else {
        x2 = x * x;
        let mut t = M_PI * x2;
        let u = 1.0 / (t * t);
        t = 1.0 / t;
        let f = 1.0 - u * polevl(u, &FN, 9) / p1evl(u, &FD, 10);
        let g = t * polevl(u, &GN, 10) / p1evl(u, &GD, 11);

        t = M_PI * 0.5 * x2;
        let c = t.cos();
        let s = t.sin();
        t = M_PI * x;
        cc = 0.5 + (f * s - g * c) / t;
        ss = 0.5 - (f * c + g * s) / t;
    }

    if xxa < 0.0 {
        cc = -cc;
        ss = -ss;
    }

    (ss, cc)
}

/// Computes the actual "standard" spiral, starting with curvature 0.
/// `s` is the run-length along the spiral [m], `c_dot` the first derivative of the curvature [1/m2].
/// Returns (x, y, t): the coordinates in the spiral's local system [m] and the tangent direction [rad].
pub fn odr_spiral(s: f64, c_dot: f64) -> (f64, f64, f64) {
    let mut a = 1.0 / c_dot.abs().sqrt();
    a *= M_PI.sqrt();

    let (ss, cc) = fresnel(s / a);
    let mut x = cc;
    let mut y = ss;

    x *= a;
    y *= a;

    if c_dot < 0.0 {
        y *= -1.0;
    }

    let t = s * s * c_dot * 0.5;
    (x, y, t)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spiral_starts_at_origin() {
        assert_eq!(odr_spiral(0., 1.), (0., 0., 0.));
    }

    #[test]
    fn spiral_matches_series() {
        // For c' = 1: x ~ s - s^5/40, y ~ s^3/6 - s^7/336, t = s^2/2
        let (x, y, t) = odr_spiral(1., 1.);
        assert!((x - 0.9753).abs() < 1e-3, "x = {x}");
        assert!((y - 0.16369).abs() < 1e-3, "y = {y}");
        assert_eq!(t, 0.5);
    }

    #[test]
    fn negative_curvature_mirrors_y() {
        let (x, y, t) = odr_spiral(1., -1.);
        let (xp, yp, _) = odr_spiral(1., 1.);
        assert_eq!(x, xp);
        assert_eq!(y, -yp);
        assert_eq!(t, -0.5);
    }
}
