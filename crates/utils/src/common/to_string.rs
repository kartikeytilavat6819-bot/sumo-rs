//! Port of utils/common/ToString.h: toString with `std::fixed` + accuracy (default gPrecision)
// shortcut: the SumoXML enum / Named-container / Distribution overloads are not ported yet,
// add them next to the XML definitions when those are ported
use super::std_defs::g_precision;

pub fn f64_to_string_prec(v: f64, accuracy: i32) -> String {
    format!("{:.*}", accuracy.max(0) as usize, v)
}
pub fn f64_to_string(v: f64) -> String {
    f64_to_string_prec(v, g_precision())
}

/// What `oss << std::fixed << std::setprecision(accuracy) << value` prints.
/// Also the argument type of `StringUtils::format` (see the `tlf!` macro).
pub trait ToStringPrec {
    fn to_string_prec(&self, accuracy: i32) -> String;
}

macro_rules! display_impl {
    ($($t:ty),*) => {$(
        impl ToStringPrec for $t {
            fn to_string_prec(&self, _accuracy: i32) -> String {
                self.to_string()
            }
        }
    )*};
}
display_impl!(i8, i16, i32, i64, u8, u16, u32, u64, usize, isize, char, str, String);

impl ToStringPrec for f64 {
    fn to_string_prec(&self, accuracy: i32) -> String {
        f64_to_string_prec(*self, accuracy)
    }
}
impl ToStringPrec for f32 {
    fn to_string_prec(&self, accuracy: i32) -> String {
        f64_to_string_prec(*self as f64, accuracy)
    }
}
/// ostream prints bools as 1/0 (no std::boolalpha)
impl ToStringPrec for bool {
    fn to_string_prec(&self, _accuracy: i32) -> String {
        if *self { "1" } else { "0" }.to_string()
    }
}
impl<T: ToStringPrec + ?Sized> ToStringPrec for &T {
    fn to_string_prec(&self, accuracy: i32) -> String {
        (**self).to_string_prec(accuracy)
    }
}
/// toString(std::vector<...>) joins with " "
impl<T: ToStringPrec> ToStringPrec for Vec<T> {
    fn to_string_prec(&self, accuracy: i32) -> String {
        join_to_string_prec(self, " ", accuracy)
    }
}
impl<T: ToStringPrec> ToStringPrec for [T] {
    fn to_string_prec(&self, accuracy: i32) -> String {
        join_to_string_prec(self, " ", accuracy)
    }
}
impl<T: ToStringPrec> ToStringPrec for std::collections::BTreeSet<T> {
    fn to_string_prec(&self, accuracy: i32) -> String {
        join_to_string_prec(self, " ", accuracy)
    }
}

/// toString(t) with gPrecision
pub fn to_string<T: ToStringPrec + ?Sized>(t: &T) -> String {
    t.to_string_prec(g_precision())
}
pub fn to_string_prec<T: ToStringPrec + ?Sized>(t: &T, accuracy: i32) -> String {
    t.to_string_prec(accuracy)
}

pub fn to_hex(i: u64, num_digits: usize, size_of: usize) -> String {
    let w = if num_digits == 0 { size_of * 2 } else { num_digits };
    format!("0x{i:0w$x}")
}

pub fn join_to_string_prec<'a, T: ToStringPrec + 'a>(v: impl IntoIterator<Item = &'a T>, between: &str, accuracy: i32) -> String {
    let mut oss = String::new();
    let mut connect = false;
    for it in v {
        if connect {
            oss += between;
        } else {
            connect = true;
        }
        oss += &it.to_string_prec(accuracy);
    }
    oss
}

/// joinToString(v, between) for vectors and sets
pub fn join_to_string<'a, T: ToStringPrec + 'a>(v: impl IntoIterator<Item = &'a T>, between: &str) -> String {
    join_to_string_prec(v, between, g_precision())
}

pub fn join_to_string_sorting<T: ToStringPrec + Ord + Clone>(v: &[T], between: &str) -> String {
    let mut sorted = v.to_vec();
    sorted.sort();
    join_to_string(&sorted, between)
}

/// joinToString(map, between, between_keyval)
pub fn join_map_to_string<K: ToStringPrec, V: ToStringPrec>(
    s: &std::collections::BTreeMap<K, V>,
    between: &str,
    between_keyval: &str,
) -> String {
    let acc = g_precision();
    let mut oss = String::new();
    let mut connect = false;
    for (k, v) in s {
        if connect {
            oss += between;
        } else {
            connect = true;
        }
        oss += &k.to_string_prec(acc);
        oss += between_keyval;
        oss += &v.to_string_prec(acc);
    }
    oss
}

/// toString(MMVersion)
pub fn mm_version_to_string(v: (i32, f64)) -> String {
    to_string(&v.0) + "." + &to_string_prec(&v.1, 0)
}

/// What a plain `std::ostream << double` prints (%g with precision 6).
pub fn f64_to_string_default(v: f64) -> String {
    format_g(v, 6)
}

/// C printf "%.{precision}g"
pub fn format_g(v: f64, precision: usize) -> String {
    if v.is_nan() {
        return if v.is_sign_negative() { "-nan" } else { "nan" }.into();
    }
    if v.is_infinite() {
        return if v < 0. { "-inf" } else { "inf" }.into();
    }
    let p = precision.max(1);
    if v == 0. {
        return if v.is_sign_negative() { "-0" } else { "0" }.into();
    }
    // exponent after rounding to p significant digits
    let sci = format!("{:.*e}", p - 1, v);
    let (mant, exp) = sci.split_once('e').unwrap();
    let exp: i32 = exp.parse().unwrap();
    let strip = |s: &str| -> String {
        if s.contains('.') {
            s.trim_end_matches('0').trim_end_matches('.').to_string()
        } else {
            s.to_string()
        }
    };
    if exp < -4 || exp >= p as i32 {
        let sign = if exp < 0 { '-' } else { '+' };
        format!("{}e{}{:02}", strip(mant), sign, exp.abs())
    } else {
        strip(&format!("{:.*}", (p as i32 - 1 - exp) as usize, v))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ties_round_like_fmt() {
        // fmt / printf round the exact binary value, ties to even: same as Rust's formatter
        assert_eq!(f64_to_string_prec(0.125, 2), "0.12");
        assert_eq!(f64_to_string_prec(0.375, 2), "0.38");
        assert_eq!(f64_to_string_prec(2.675, 2), "2.67"); // 2.675 is stored as 2.67499999...
        assert_eq!(f64_to_string_prec(-0.0, 2), "-0.00");
    }
    #[test]
    fn conversions() {
        assert_eq!(to_string_prec(&1.5f64, 2), "1.50");
        assert_eq!(to_string(&true), "1");
        assert_eq!(to_string(&vec![1, 2, 3]), "1 2 3");
        assert_eq!(join_to_string(&["a".to_string(), "b".to_string()], ","), "a,b");
        assert_eq!(to_hex(255, 0, 4), "0x000000ff");
    }
    #[test]
    fn g_format() {
        assert_eq!(format_g(0.1, 6), "0.1");
        assert_eq!(format_g(100., 6), "100");
        assert_eq!(format_g(1e6, 6), "1e+06");
        assert_eq!(format_g(123456.7, 6), "123457");
        assert_eq!(format_g(1e-5, 6), "1e-05");
        assert_eq!(format_g(-2.5, 6), "-2.5");
        assert_eq!(format_g(0.0001, 6), "0.0001");
        assert_eq!(format_g(999999.5, 6), "1e+06");
    }
}
