//! Port of utils/common/StringUtils.{h,cpp}
use super::std_defs::g_precision;
use super::string_tokenizer::StringTokenizer;
use super::time::{is_time, steps2time, string2time};
use super::to_string::{to_string_prec, ToStringPrec};
use super::util_exceptions::ProcessError;
use std::time::{SystemTime, UNIX_EPOCH};

const KM_PER_MILE: f64 = 1.609344;

pub fn prune(s: &str) -> String {
    s.trim_matches([' ', '\t', '\n', '\r']).to_string()
}

pub fn prune_zeros(s: &str, max: i32) -> String {
    if let Some(endpos) = s.rfind(|c| c != '0') {
        if s.ends_with('0') {
            let keep = (s.len() as i32 - max).max(endpos as i32 + 1);
            return s[..keep as usize].to_string();
        }
    }
    s.to_string()
}

pub fn to_lower_case(s: &str) -> String {
    s.to_ascii_lowercase()
}

pub fn to_upper_case(s: &str) -> String {
    s.to_ascii_uppercase()
}

/// every latin1 byte becomes the unicode char with the same code point
pub fn latin1_to_utf8(s: &[u8]) -> String {
    s.iter().map(|&b| b as char).collect()
}

/// Replacement for strings decoded from Latin-1 bytes (e.g. VISSIM): C++ sees the raw
/// Latin-1 bytes and transliterates them, which is equivalent to replacing the code points.
pub fn convert_umlaute_latin1(s: &str) -> String {
    let mut s = replace(s, "\u{E4}", "ae");
    s = replace(&s, "\u{C4}", "Ae");
    s = replace(&s, "\u{F6}", "oe");
    s = replace(&s, "\u{D6}", "Oe");
    s = replace(&s, "\u{FC}", "ue");
    s = replace(&s, "\u{DC}", "Ue");
    s = replace(&s, "\u{DF}", "ss");
    s = replace(&s, "\u{C9}", "E");
    s = replace(&s, "\u{E9}", "e");
    s = replace(&s, "\u{C8}", "E");
    replace(&s, "\u{E8}", "e")
}

/// C++ StringUtils::convertUmlaute replaces single Latin-1 *bytes* in a byte string.
/// XML input is decoded to UTF-8 first (e.g. 'ä' is C3 A4), so those bytes do not match
/// and C++ leaves the characters intact. Operate on the UTF-8 bytes, not on code points.
pub fn convert_umlaute(s: &str) -> String {
    const REPL: [(u8, &[u8]); 11] = [
        (0xE4, b"ae"), (0xC4, b"Ae"), (0xF6, b"oe"), (0xD6, b"Oe"),
        (0xFC, b"ue"), (0xDC, b"Ue"), (0xDF, b"ss"), (0xC9, b"E"),
        (0xE9, b"e"), (0xC8, b"E"), (0xE8, b"e"),
    ];
    let mut bytes = s.as_bytes().to_vec();
    for (needle, repl) in REPL {
        if !bytes.contains(&needle) {
            continue;
        }
        let mut out = Vec::with_capacity(bytes.len());
        for &b in &bytes {
            if b == needle {
                out.extend_from_slice(repl);
            } else {
                out.push(b);
            }
        }
        bytes = out;
    }
    String::from_utf8_lossy(&bytes).into_owned()
}

pub fn replace(s: &str, what: &str, by: &str) -> String {
    if what.is_empty() {
        return s.to_string();
    }
    s.replace(what, by)
}

/// date/time parts (y, m, d, H, M, S) of a unix timestamp in UTC
fn civil_from_unix(secs: i64) -> (i64, u32, u32, u32, u32, u32) {
    let days = secs.div_euclid(86400);
    let rem = secs.rem_euclid(86400);
    // Howard Hinnant's civil_from_days
    let z = days + 719468;
    let era = z.div_euclid(146097);
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    let y = yoe + era * 400 + i64::from(m <= 2);
    (y, m, d, (rem / 3600) as u32, (rem % 3600 / 60) as u32, (rem % 60) as u32)
}

fn split_time(t: SystemTime) -> (i64, i64) {
    match t.duration_since(UNIX_EPOCH) {
        Ok(d) => (d.as_secs() as i64, d.subsec_micros() as i64),
        Err(e) => (-(e.duration().as_secs() as i64), 0),
    }
}

pub fn substitute_environment(s: &str, time_ref: Option<SystemTime>) -> String {
    let mut res = s.to_string();
    if let Some(time_ref) = time_ref {
        let local_time_index = s.find("${LOCALTIME}");
        let utc_index = s.find("${UTC}");
        if local_time_index.is_some() || utc_index.is_some() {
            // shortcut: LOCALTIME is written in UTC (no timezone database), add chrono if local time matters
            let (secs, micros) = split_time(time_ref);
            let (y, mo, d, h, mi, se) = civil_from_unix(secs);
            let micro = format!("{y:04}-{mo:02}-{d:02}-{h:02}-{mi:02}-{se:02}.{micros}");
            if let Some(i) = utc_index {
                res.replace_range(i..i + 6, &micro);
            } else if let Some(i) = local_time_index {
                res.replace_range(i..i + 12, &micro);
            }
        }
    }
    if let Some(pid_index) = s.find("${PID}") {
        // C++ applies the index into the original string to the (maybe already changed) result
        let end = (pid_index + 6).min(res.len());
        if pid_index <= res.len() && res.is_char_boundary(pid_index) && res.is_char_boundary(end) {
            res.replace_range(pid_index..end, &std::process::id().to_string());
        }
    }
    if std::env::var_os("SUMO_LOGO").is_none() {
        res = replace(&res, "${SUMO_LOGO}", "${SUMO_HOME}/data/logo/sumo-128x138.png");
    }
    if s.starts_with('~') {
        res.replace_range(0..1, "${HOME}");
    }
    res = replace(&res, ",~", ",${HOME}");
    #[cfg(windows)]
    if std::env::var_os("HOME").is_none() {
        res = replace(&res, "${HOME}", "${USERPROFILE}");
    }
    // regex \$\{(.+?)\}: each variable name found is replaced everywhere in the string
    let mut names = Vec::new();
    let mut rest = res.as_str();
    while let Some(start) = rest.find("${") {
        let after = &rest[start + 2..];
        let first = after.chars().next();
        match first {
            Some(c) if c != '\n' => {
                let tail = &after[c.len_utf8()..];
                let line_end = tail.find('\n').unwrap_or(tail.len());
                if let Some(end) = tail[..line_end].find('}') {
                    let name_len = c.len_utf8() + end;
                    names.push(after[..name_len].to_string());
                    rest = &after[name_len + 1..];
                    continue;
                }
            }
            _ => {}
        }
        rest = &rest[start + 2..];
    }
    for name in names {
        let value = std::env::var(&name).unwrap_or_default();
        res = res.replace(&format!("${{{name}}}"), &value);
    }
    res
}

/// shortcut: always writes UTC with offset +00:00 (no timezone database), add chrono if local time matters
pub fn iso_time_string(time_ref: Option<SystemTime>) -> String {
    let now = time_ref.unwrap_or_else(SystemTime::now);
    let (secs, micros) = split_time(now);
    let (y, mo, d, h, mi, se) = civil_from_unix(secs);
    format!("{y:04}-{mo:02}-{d:02}T{h:02}:{mi:02}:{se:02}.{micros:06}+00:00")
}

pub fn starts_with(s: &str, prefix: &str) -> bool {
    s.starts_with(prefix)
}

pub fn ends_with(s: &str, suffix: &str) -> bool {
    s.ends_with(suffix)
}

pub fn pad_front(s: &str, length: i32, padding: char) -> String {
    let n = (length - s.len() as i32).max(0) as usize;
    std::iter::repeat_n(padding, n).collect::<String>() + s
}

pub fn escape_xml(orig: &str, mask_double_hyphen: bool) -> String {
    let mut result = replace(orig, "&", "&amp;");
    result = replace(&result, ">", "&gt;");
    result = replace(&result, "<", "&lt;");
    result = replace(&result, "\"", "&quot;");
    if mask_double_hyphen {
        result = replace(&result, "--", "&#45;&#45;");
    }
    result.retain(|c| !('\u{1}'..' ').contains(&c));
    replace(&result, "'", "&apos;")
}

pub fn escape_shell(orig: &str) -> String {
    replace(orig, "\"", "\\\"")
}

pub fn escape_csv(orig: &str, separator: char, quote: char) -> String {
    if !orig.contains([separator, quote]) {
        return orig.to_string();
    }
    let quote_str = quote.to_string();
    format!("{quote_str}{}{quote_str}", replace(orig, &quote_str, &format!("\\{quote}")))
}

pub fn url_encode(to_encode: &str, encode_which: &str) -> String {
    let mut out = String::new();
    for &t in to_encode.as_bytes() {
        let keep = if !encode_which.is_empty() {
            !encode_which.as_bytes().contains(&t)
        } else {
            (45..=57).contains(&t) || (65..=90).contains(&t) || t == 95 || (97..=122).contains(&t) || t == 126
        };
        if keep {
            out.push(t as char);
        } else {
            out += &char_to_hex(t);
        }
    }
    out
}

pub fn url_decode(to_decode: &str) -> Result<String, ProcessError> {
    let b = to_decode.as_bytes();
    let mut out = Vec::with_capacity(b.len());
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'%' {
            let end = (i + 3).min(b.len());
            out.push(hex_to_char(&String::from_utf8_lossy(&b[i + 1..end]))?);
            i += 2;
        } else {
            out.push(b[i]);
        }
        i += 1;
    }
    Ok(String::from_utf8_lossy(&out).into_owned())
}

pub fn char_to_hex(c: u8) -> String {
    format!("%{c:02x}")
}

/// `istream >> std::hex >> short`
pub fn hex_to_char(s: &str) -> Result<u8, ProcessError> {
    if s.is_empty() {
        return Ok(0);
    }
    let t = lead_trim(s);
    let (neg, t) = match t.as_bytes().first() {
        Some(b'-') => (true, &t[1..]),
        Some(b'+') => (false, &t[1..]),
        _ => (false, t),
    };
    let t = t.strip_prefix("0x").or_else(|| t.strip_prefix("0X")).filter(|r| r.starts_with(|c: char| c.is_ascii_hexdigit())).unwrap_or(t);
    let digits: String = t.chars().take_while(|c| c.is_ascii_hexdigit()).collect();
    let fail = || ProcessError::NumberFormat(format!("{s} could not be interpreted as hex"));
    if digits.is_empty() {
        return Err(fail());
    }
    let v = i64::from_str_radix(&digits, 16).map_err(|_| fail())?;
    let v = if neg { -v } else { v };
    if v > i16::MAX as i64 || v < i16::MIN as i64 {
        return Err(fail());
    }
    Ok(v as i16 as u8)
}

pub fn to_int(s: &str) -> Result<i32, ProcessError> {
    let result = to_long(s)?;
    if result > i32::MAX as i64 || result < i32::MIN as i64 {
        return Err(ProcessError::NumberFormat(format!("{result} int overflow")));
    }
    Ok(result as i32)
}

pub fn is_int(s: &str) -> bool {
    is_long(s) && to_long(s).is_ok_and(|r| r <= i32::MAX as i64 && r >= i32::MIN as i64)
}

pub fn to_int_secure(s: &str, def: i32) -> Result<i32, ProcessError> {
    if s.is_empty() {
        return Ok(def);
    }
    to_int(s)
}

/// strtoll/strtod skip leading whitespace (isspace)
fn lead_trim(s: &str) -> &str {
    s.trim_start_matches([' ', '\t', '\n', '\r', '\x0b', '\x0c'])
}

/// strtoll(data, &end, 10): (value, consumed bytes, out of range)
fn strtoll(s: &str) -> (i64, usize, bool) {
    let t = lead_trim(s);
    let skipped = s.len() - t.len();
    let b = t.as_bytes();
    let mut i = 0;
    let neg = match b.first() {
        Some(b'-') => {
            i += 1;
            true
        }
        Some(b'+') => {
            i += 1;
            false
        }
        _ => false,
    };
    let dstart = i;
    let mut v: i128 = 0;
    let mut overflow = false;
    while i < b.len() && b[i].is_ascii_digit() {
        v = v * 10 + (b[i] - b'0') as i128;
        if v > i64::MAX as i128 + 1 {
            overflow = true;
            v = i64::MAX as i128 + 1;
        }
        i += 1;
    }
    if i == dstart {
        return (0, 0, false);
    }
    let v = if neg { -v } else { v };
    if v > i64::MAX as i128 || v < i64::MIN as i128 || overflow {
        return (if neg { i64::MIN } else { i64::MAX }, skipped + i, true);
    }
    (v as i64, skipped + i, false)
}

pub fn to_long(s: &str) -> Result<i64, ProcessError> {
    if s.is_empty() {
        return Err(ProcessError::EmptyData);
    }
    let (ret, end, range) = strtoll(s);
    if range {
        return Err(ProcessError::NumberFormat(format!("(long long integer range) {s}")));
    }
    if end != s.len() {
        return Err(ProcessError::NumberFormat(format!("(long long integer format) {s}")));
    }
    Ok(ret)
}

pub fn is_long(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let (_, end, range) = strtoll(s);
    !range && end == s.len()
}

/// strtol(s, &end, 16) incl. optional sign and 0x prefix: (value, consumed bytes, out of i64 range)
fn strtol_hex(s: &str) -> (i64, usize, bool) {
    let t = lead_trim(s);
    let skipped = s.len() - t.len();
    let b = t.as_bytes();
    let mut i = 0;
    let neg = match b.first() {
        Some(b'-') => {
            i += 1;
            true
        }
        Some(b'+') => {
            i += 1;
            false
        }
        _ => false,
    };
    if i + 2 < b.len() + 1 && b.get(i) == Some(&b'0') && matches!(b.get(i + 1), Some(b'x' | b'X')) && b.get(i + 2).is_some_and(|c| c.is_ascii_hexdigit()) {
        i += 2;
    }
    let dstart = i;
    let mut v: i128 = 0;
    let mut overflow = false;
    while i < b.len() && b[i].is_ascii_hexdigit() {
        v = v * 16 + (b[i] as char).to_digit(16).unwrap() as i128;
        if v > i64::MAX as i128 + 1 {
            overflow = true;
            v = i64::MAX as i128 + 1;
        }
        i += 1;
    }
    if i == dstart {
        return (0, 0, false);
    }
    let v = if neg { -v } else { v };
    if overflow || v > i64::MAX as i128 || v < i64::MIN as i128 {
        return (0, skipped + i, true);
    }
    (v as i64, skipped + i, false)
}

pub fn hex_to_int(s: &str) -> Result<i32, ProcessError> {
    if s.is_empty() {
        return Err(ProcessError::EmptyData);
    }
    let fail = || ProcessError::NumberFormat(format!("(hex integer format) {s}"));
    let (body, extra) = if let Some(stripped) = s.strip_prefix('#') { (stripped, 1) } else { (s, 0) };
    // std::stoi: invalid_argument without digits, out_of_range beyond int
    let (v, end, range) = strtol_hex(body);
    if end == 0 || range || v > i32::MAX as i64 || v < i32::MIN as i64 {
        return Err(fail());
    }
    if end + extra != s.len() {
        return Err(fail());
    }
    Ok(v as i32)
}

pub fn is_hex(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let s = s.strip_prefix('#').unwrap_or(s);
    let (_, end, range) = strtol_hex(s);
    !range && end != 0 && end == s.len()
}

/// strtod(data, &end): (value, consumed bytes, ERANGE)
pub fn strtod(s: &str) -> (f64, usize, bool) {
    let t = lead_trim(s);
    let skipped = s.len() - t.len();
    let b = t.as_bytes();
    let mut i = 0;
    if matches!(b.first(), Some(b'+' | b'-')) {
        i += 1;
    }
    let lower = t[i..].to_ascii_lowercase();
    for word in ["infinity", "inf", "nan"] {
        if lower.starts_with(word) {
            let mut end = i + word.len();
            if word == "nan" && t[end..].starts_with('(') {
                if let Some(close) = t[end..].find(')') {
                    if t[end + 1..end + close].bytes().all(|c| c.is_ascii_alphanumeric() || c == b'_') {
                        end += close + 1;
                    }
                }
            }
            let v: f64 = if word == "nan" { f64::NAN } else { f64::INFINITY };
            let v = if b[0] == b'-' { -v } else { v };
            return (v, skipped + end, false);
        }
    }
    // hexadecimal floats: "0x10", "0x1.8p1"; without hex digits strtod stops after the "0"
    if b.len() > i + 2 && b[i] == b'0' && (b[i + 1] == b'x' || b[i + 1] == b'X') {
        let hex = |c: u8| (c as char).to_digit(16);
        let (mut j, mut mant, mut hex_digits, mut frac_digits, mut nonzero) = (i + 2, 0f64, 0, 0i32, false);
        while let Some(d) = b.get(j).and_then(|&c| hex(c)) {
            nonzero |= d != 0;
            mant = mant * 16. + d as f64;
            hex_digits += 1;
            j += 1;
        }
        if b.get(j) == Some(&b'.') {
            j += 1;
            while let Some(d) = b.get(j).and_then(|&c| hex(c)) {
                nonzero |= d != 0;
                mant = mant * 16. + d as f64;
                hex_digits += 1;
                frac_digits += 1;
                j += 1;
            }
        }
        if hex_digits > 0 {
            let mut exp: i32 = 0;
            if matches!(b.get(j), Some(b'p' | b'P')) {
                let mut k = j + 1;
                let neg = b.get(k) == Some(&b'-');
                if matches!(b.get(k), Some(b'+' | b'-')) {
                    k += 1;
                }
                if b.get(k).is_some_and(|c| c.is_ascii_digit()) {
                    let mut e: i32 = 0;
                    while let Some(c) = b.get(k).filter(|c| c.is_ascii_digit()) {
                        e = e.saturating_mul(10).saturating_add((c - b'0') as i32);
                        k += 1;
                    }
                    exp = if neg { -e } else { e };
                    j = k;
                }
            }
            let v = mant * 2f64.powi(exp - 4 * frac_digits);
            let v = if b[0] == b'-' { -v } else { v };
            let range = v.is_infinite() || (nonzero && (v == 0. || v.is_subnormal()));
            return (v, skipped + j, range);
        }
    }
    let mut digits = 0;
    let mut nonzero = false;
    while i < b.len() && b[i].is_ascii_digit() {
        nonzero |= b[i] != b'0';
        i += 1;
        digits += 1;
    }
    if i < b.len() && b[i] == b'.' {
        i += 1;
        while i < b.len() && b[i].is_ascii_digit() {
            nonzero |= b[i] != b'0';
            i += 1;
            digits += 1;
        }
    }
    if digits == 0 {
        return (0., 0, false);
    }
    if i < b.len() && (b[i] == b'e' || b[i] == b'E') {
        let mut j = i + 1;
        if matches!(b.get(j), Some(b'+' | b'-')) {
            j += 1;
        }
        if b.get(j).is_some_and(|c| c.is_ascii_digit()) {
            while j < b.len() && b[j].is_ascii_digit() {
                j += 1;
            }
            i = j;
        }
    }
    let num = &t[..i];
    let v: f64 = num.parse().unwrap_or(0.);
    let range = v.is_infinite() || (nonzero && (v == 0. || v.is_subnormal()));
    (v, skipped + i, range)
}

/// std::stod(s, &idx): Err on invalid_argument / out_of_range
fn stod(s: &str) -> Option<(f64, usize)> {
    let (v, end, range) = strtod(s);
    if end == 0 || range {
        return None;
    }
    Some((v, end))
}

pub fn to_double(s: &str) -> Result<f64, ProcessError> {
    if s.is_empty() {
        return Err(ProcessError::EmptyData);
    }
    match stod(s) {
        Some((v, idx)) if idx == s.len() => Ok(v),
        _ => Err(ProcessError::NumberFormat(format!("(double) {s}"))),
    }
}

pub fn is_double(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let (_, end, range) = strtod(s);
    !range && end != 0 && end == s.len()
}

pub fn to_double_secure(s: &str, def: f64) -> Result<f64, ProcessError> {
    if s.is_empty() {
        return Ok(def);
    }
    to_double(s)
}

pub fn to_double_allow_time(s: &str) -> Result<f64, ProcessError> {
    if !s.contains(':') {
        return to_double(s);
    } else if is_time(s) {
        return string2time(s).map(steps2time);
    }
    Err(ProcessError::NumberFormat(format!("(double) {s}")))
}

pub fn to_bool(s: &str) -> Result<bool, ProcessError> {
    if s.is_empty() {
        return Err(ProcessError::EmptyData);
    }
    let s = to_lower_case(s);
    match s.as_str() {
        "1" | "yes" | "true" | "on" | "x" | "t" => Ok(true),
        "0" | "no" | "false" | "off" | "-" | "f" => Ok(false),
        _ => Err(ProcessError::BoolFormat(s)),
    }
}

pub fn is_bool(s: &str) -> bool {
    !s.is_empty()
        && matches!(
            to_lower_case(s).as_str(),
            "1" | "yes" | "true" | "on" | "x" | "t" | "0" | "no" | "false" | "off" | "-" | "f"
        )
}

pub fn to_version(s: &str) -> Result<(i32, f64), ProcessError> {
    let parts = StringTokenizer::with_token(s, ".", false).get_vector();
    let front = parts.first().map(String::as_str).unwrap_or("");
    let back = parts.last().map(String::as_str).unwrap_or("");
    Ok((to_int(front)?, to_double(back)?))
}

pub fn parse_dist(s: &str) -> Result<f64, ProcessError> {
    if s.is_empty() {
        return Err(ProcessError::EmptyData);
    }
    // every failure inside the C++ try block is rethrown as "(double) ..."
    let fail = || ProcessError::NumberFormat(format!("(double) {s}"));
    let (result, idx) = stod(s).ok_or_else(fail)?;
    if idx == s.len() {
        return Ok(result);
    }
    let unit = prune(&s[idx..]);
    match unit.as_str() {
        "m" | "metre" | "meter" | "metres" | "meters" => return Ok(result),
        "km" | "kilometre" | "kilometer" | "kilometres" | "kilometers" => return Ok(result * 1000.),
        "mi" | "mile" | "miles" => return Ok(result * 1000. * KM_PER_MILE),
        "nmi" => return Ok(result * 1852.),
        "ft" | "foot" | "feet" => return Ok(result * 12. * 0.0254),
        "\"" | "in" | "inch" | "inches" => return Ok(result * 0.0254),
        _ => {}
    }
    if unit.starts_with('\'') {
        let mut inches = 12. * result;
        if unit.len() > 1 {
            let (v, idx2) = stod(&unit[1..]).ok_or_else(fail)?;
            inches += v;
            // C++ applies the index of the substring to the full unit string
            if unit.get(idx2..) == Some("\"") {
                return Ok(inches * 0.0254);
            }
        }
    }
    Err(fail())
}

pub fn parse_speed(s: &str, default_kmph: bool) -> Result<f64, ProcessError> {
    if s.is_empty() {
        return Err(ProcessError::EmptyData);
    }
    let fail = || ProcessError::NumberFormat(format!("(double) {s}"));
    let (result, idx) = stod(s).ok_or_else(fail)?;
    if idx == s.len() {
        return Ok(if default_kmph { result / 3.6 } else { result });
    }
    match prune(&s[idx..]).as_str() {
        "km/h" | "kph" | "kmh" | "kmph" => Ok(result / 3.6),
        "m/s" => Ok(result),
        "mph" => Ok(result * KM_PER_MILE / 3.6),
        "knots" => Ok(result * 1.852 / 3.6),
        _ => Err(fail()),
    }
}

/// default t is " \t\n"
pub fn trim_left(s: &str, t: &str) -> String {
    s.trim_start_matches(|c| t.contains(c)).to_string()
}

pub fn trim_right(s: &str, t: &str) -> String {
    s.trim_end_matches(|c| t.contains(c)).to_string()
}

pub fn trim(s: &str, t: &str) -> String {
    trim_right(&trim_left(s, t), t)
}

pub fn wrap_text(s: &str, width: i32) -> String {
    let parts = StringTokenizer::new(s).get_vector();
    let mut result = String::new();
    let mut line = String::new();
    let mut first_line = true;
    let mut first_word = true;
    for p in parts {
        if ((line.len() + p.len()) as i32) < width || first_word {
            if first_word {
                first_word = false;
            } else {
                line += " ";
            }
            line += &p;
        } else {
            if first_line {
                first_line = false;
            } else {
                result += "\n";
            }
            result += &line;
            line.clear();
            line += &p;
        }
    }
    if !line.is_empty() {
        if !first_line {
            result += "\n";
        }
        result += &line;
    }
    result
}

pub fn adjust_decimal_value(value: f64, precision: i32) -> String {
    let mut value_str = to_string_prec(&value, precision);
    while value_str.len() > 1 {
        if value_str.ends_with('0') {
            value_str.pop();
        } else if value_str.ends_with('.') {
            value_str.pop();
            return value_str;
        } else {
            return value_str;
        }
    }
    value_str
}

/// StringUtils::format: every '%' is replaced by the next argument (doubles fixed with gPrecision);
/// once the arguments are used up the rest is copied verbatim. See the `tlf!` macro.
pub fn format(fmt: &str, args: &[&dyn ToStringPrec]) -> String {
    let prec = g_precision();
    let mut os = String::new();
    let mut rest = fmt;
    for arg in args {
        match rest.find('%') {
            Some(i) => {
                os += &rest[..i];
                os += &arg.to_string_prec(prec);
                rest = &rest[i + 1..];
            }
            None => {
                // C++ drops the remaining arguments
                os += rest;
                return os;
            }
        }
    }
    os += rest;
    os
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_floats_like_strtod() {
        // glibc strtod: "0x10" = 16, "0x1.8p1" = 3, "-0x1p-2" = -0.25, "0x" stops after the "0"
        assert_eq!(to_double("0x10").unwrap(), 16.);
        assert_eq!(to_double("0x1.8p1").unwrap(), 3.);
        assert_eq!(to_double("-0x1p-2").unwrap(), -0.25);
        assert_eq!(strtod("0x"), (0., 1, false));
        assert!(to_double("0x").is_err());
    }

    #[test]
    fn parse() {
        assert_eq!(to_int(" 42").unwrap(), 42);
        assert_eq!(to_int("4x").unwrap_err().to_string(), "Invalid Number Format (long long integer format) 4x");
        assert_eq!(to_int("3000000000").unwrap_err().to_string(), "Invalid Number Format 3000000000 int overflow");
        assert_eq!(to_long("99999999999999999999").unwrap_err().to_string(), "Invalid Number Format (long long integer range) 99999999999999999999");
        assert_eq!(to_int("").unwrap_err(), ProcessError::EmptyData);
        assert_eq!(to_double("1e3").unwrap(), 1000.);
        assert_eq!(to_double(" .5").unwrap(), 0.5);
        assert_eq!(to_double("1e").unwrap_err().to_string(), "Invalid Number Format (double) 1e");
        assert!(to_double("1e999").is_err());
        assert!(is_double("-inf") && !is_double("1x") && !is_double(""));
        assert!(to_bool("yes").unwrap());
        assert_eq!(to_bool("Maybe").unwrap_err().to_string(), "Invalid Bool Format maybe");
        assert_eq!(hex_to_int("#ff0000").unwrap(), 0xff0000);
        assert!(hex_to_int("#ff0000ff").is_err()); // stoi range
        assert!(is_hex("#ff0000ff") && !is_hex("#xyz"));
        assert_eq!(to_version("1.20").unwrap(), (1, 20.));
    }

    #[test]
    fn units() {
        assert_eq!(parse_dist("2km").unwrap(), 2000.);
        assert_eq!(parse_dist("3 m").unwrap(), 3.);
        assert_eq!(parse_dist("3 x").unwrap_err().to_string(), "Invalid Number Format (double) 3 x");
        assert!((parse_speed("36", true).unwrap() - 10.).abs() < 1e-12);
        assert_eq!(parse_speed("5m/s", true).unwrap(), 5.);
    }

    #[test]
    fn strings() {
        assert_eq!(prune(" \tab c\n"), "ab c");
        assert_eq!(prune("   "), "");
        assert_eq!(prune_zeros("1.500", 2), "1.5");
        assert_eq!(prune_zeros("1.000", 2), "1.0");
        assert_eq!(replace("aaa", "a", "ba"), "bababa");
        assert_eq!(escape_xml("<a & 'b'>\t--", true), "&lt;a &amp; &apos;b&apos;&gt;&#45;&#45;");
        assert_eq!(escape_csv("a,b", ',', '"'), "\"a,b\"");
        assert_eq!(url_encode("a b;%", " ;%"), "a%20b%3b%25");
        assert_eq!(url_decode("a%20b").unwrap(), "a b");
        assert!(url_decode("%zz").is_err());
        assert_eq!(pad_front("7", 3, '0'), "007");
        assert_eq!(trim("  x \n", " \t\n"), "x");
        assert_eq!(wrap_text("aa bb cc", 6), "aa bb\ncc");
        assert_eq!(adjust_decimal_value(1.2500, 4), "1.25");
        assert_eq!(convert_umlaute("Grüße"), "Grüße"); // UTF-8 bytes are untouched by C++
        assert_eq!(convert_umlaute_latin1("Grüße"), "Gruesse"); // Latin-1 decoded bytes
        assert_eq!(latin1_to_utf8(b"\xe4"), "ä");
    }

    #[test]
    fn formatting() {
        assert_eq!(format("a % b %", &[&1, &"x"]), "a 1 b x");
        assert_eq!(format("100% sure", &[]), "100% sure");
        assert_eq!(format("v=% w=%", &[&1.5f64]), "v=1.50 w=%");
        assert_eq!(crate::tlf!("% and %", "a", 2), "a and 2");
    }

    #[test]
    fn environment() {
        // SAFETY: test-only, variable names are unique to this test
        unsafe { std::env::set_var("SUMO_RS_TEST_VAR", "val") };
        assert_eq!(substitute_environment("x${SUMO_RS_TEST_VAR}y${SUMO_RS_TEST_VAR}", None), "xvalyval");
        assert_eq!(substitute_environment("${SUMO_RS_UNSET_VAR}z", None), "z");
        assert_eq!(substitute_environment("p${PID}", None), format!("p{}", std::process::id()));
        let t = UNIX_EPOCH + std::time::Duration::from_micros(86400_000_123);
        assert_eq!(substitute_environment("${UTC}", Some(t)), "1970-01-02-00-00-00.123");
        assert_eq!(iso_time_string(Some(t)), "1970-01-02T00:00:00.000123+00:00");
    }
}
