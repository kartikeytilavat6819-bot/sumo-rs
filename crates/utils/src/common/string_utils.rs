//! Port of the parsing part of utils/common/StringUtils.cpp
// shortcut: only number/bool parsing is ported so far; port the rest of StringUtils as callers need it

#[derive(Debug, Clone)]
pub struct NumberFormatException(pub String);

/// strtoll/strtod skip leading whitespace; Rust's parse does not
fn lead_trim(s: &str) -> &str {
    s.trim_start_matches([' ', '\t', '\n', '\r', '\x0b', '\x0c'])
}

pub fn to_long(s: &str) -> Result<i64, NumberFormatException> {
    if s.is_empty() {
        return Err(NumberFormatException("Empty Data".into()));
    }
    let t = lead_trim(s);
    let t = t.strip_prefix('+').filter(|r| !r.starts_with('-')).unwrap_or(t);
    t.parse::<i64>().map_err(|_| NumberFormatException(format!("(long long integer format) {s}")))
}
pub fn to_int(s: &str) -> Result<i32, NumberFormatException> {
    let r = to_long(s)?;
    i32::try_from(r).map_err(|_| NumberFormatException(format!("{r} int overflow")))
}
pub fn to_double(s: &str) -> Result<f64, NumberFormatException> {
    if s.is_empty() {
        return Err(NumberFormatException("Empty Data".into()));
    }
    let v: f64 = lead_trim(s).parse().map_err(|_| NumberFormatException(format!("(double) {s}")))?;
    if v.is_infinite() && !s.to_ascii_lowercase().contains("inf") {
        return Err(NumberFormatException(format!("(double) {s}"))); // stod out_of_range
    }
    Ok(v)
}
pub fn to_bool(s: &str) -> Result<bool, NumberFormatException> {
    match s.to_ascii_lowercase().as_str() {
        "1" | "yes" | "true" | "on" | "x" | "t" => Ok(true),
        "0" | "no" | "false" | "off" | "-" | "f" => Ok(false),
        _ => Err(NumberFormatException(format!("{s} is not a valid bool"))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse() {
        assert_eq!(to_int(" 42").unwrap(), 42);
        assert!(to_int("4x").is_err());
        assert_eq!(to_double("1e3").unwrap(), 1000.);
        assert!(to_bool("yes").unwrap());
    }
}
