//! Port of utils/common/SUMOTime.{h,cpp}
use super::std_defs::{g_human_readable_time, g_precision};
use super::string_tokenizer::StringTokenizer;
use super::string_utils;
use super::util_exceptions::ProcessError;
use std::sync::atomic::{AtomicI64, Ordering};

pub type SUMOTime = i64;

pub const SUMOTIME_MAX: SUMOTime = i64::MAX - 1000;
pub const SUMOTIME_MIN: SUMOTime = i64::MIN;
pub const SUMOTIME_DAY: SUMOTime = 86_400_000;
pub const SUMOTIME_HOUR: SUMOTime = 3_600_000;
pub const SUMOTIME_MINUTE: SUMOTime = 60_000;

static DELTA_T: AtomicI64 = AtomicI64::new(1000);

#[inline]
pub fn delta_t() -> SUMOTime {
    DELTA_T.load(Ordering::Relaxed)
}
pub fn set_delta_t(v: SUMOTime) {
    DELTA_T.store(v, Ordering::Relaxed)
}
/// TS: step length in seconds
#[inline]
pub fn ts() -> f64 {
    delta_t() as f64 / 1000.
}
#[inline]
pub fn speed2dist(x: f64) -> f64 {
    x * ts()
}
#[inline]
pub fn dist2speed(x: f64) -> f64 {
    x / ts()
}
#[inline]
pub fn accel2dist(x: f64) -> f64 {
    x * ts() * ts()
}
#[inline]
pub fn accel2speed(x: f64) -> f64 {
    x * ts()
}
#[inline]
pub fn speed2accel(x: f64) -> f64 {
    x / ts()
}
#[inline]
pub fn steps2time(x: SUMOTime) -> f64 {
    x as f64 / 1000.
}
#[inline]
pub fn time2steps(x: f64) -> SUMOTime {
    (x * 1000. + if x >= 0. { 0.5 } else { -0.5 }) as SUMOTime
}
#[inline]
pub fn stepfloor(x: SUMOTime) -> SUMOTime {
    (x / delta_t()) * delta_t()
}
pub fn sumotime_max_period() -> SUMOTime {
    SUMOTIME_MAX - SUMOTIME_MAX % delta_t()
}

/// throws NumberFormatException (via toDouble) or TimeFormatException like the C++ code
pub fn string2time(r: &str) -> Result<SUMOTime, ProcessError> {
    if !r.contains(':') {
        let time = string_utils::to_double(r)?;
        if time > steps2time(SUMOTIME_MAX) {
            return Err(ProcessError::TimeFormat(format!("Input string '{r}' exceeds the time value range.")));
        }
        return Ok(time2steps(time));
    }
    let hrt = StringTokenizer::with_token(r, ":", false).get_vector();
    let t = |s: &str| string2time(s);
    match hrt.len() {
        3 => Ok(3600 * t(&hrt[0])? + 60 * t(&hrt[1])? + t(&hrt[2])?),
        4 => Ok(24 * 3600 * t(&hrt[0])? + 3600 * t(&hrt[1])? + 60 * t(&hrt[2])? + t(&hrt[3])?),
        _ => Err(ProcessError::TimeFormat(format!(
            "Input string '{r}' is not a valid time format (jj:HH:MM:SS.S)."
        ))),
    }
}

pub fn is_time(r: &str) -> bool {
    if !r.contains(':') {
        return string_utils::is_double(r) && string_utils::to_double(r).is_ok_and(|v| v <= steps2time(SUMOTIME_MAX));
    }
    let hrt = StringTokenizer::with_token(r, ":", false).get_vector();
    let int = |s: &str| string_utils::is_int(s);
    let dbl = |s: &str| string_utils::is_double(s);
    match hrt.len() {
        3 => int(&hrt[0]) && int(&hrt[1]) && int(&hrt[2]),
        4 => int(&hrt[0]) && int(&hrt[1]) && int(&hrt[2]) && dbl(&hrt[3]),
        _ => false,
    }
}

fn append_padded(result: &mut String, value: SUMOTime, width: usize) {
    let digits = value.to_string();
    for _ in digits.len()..width {
        result.push('0');
    }
    result.push_str(&digits);
}

pub fn time2string_hr(mut t: SUMOTime, human_readable: bool) -> String {
    let negative = t < 0;
    t = if t == SUMOTIME_MIN { SUMOTIME_MAX } else { t.abs() };
    let prec = g_precision();
    let mut scale = 10i64.pow((3 - prec).max(0) as u32);
    if scale > 1 {
        if t != SUMOTIME_MAX {
            t = (t + scale / 2) / scale;
        } else {
            scale = 1;
        }
    }
    let second = time2steps(1.) / scale;
    let frac_w = prec.min(3).max(0) as usize;
    let mut result = String::from(if negative { "-" } else { "" });
    if human_readable {
        let minute = 60 * second;
        let hour = 60 * minute;
        let day = 24 * hour;
        if t > day {
            append_padded(&mut result, t / day, 0);
            result.push(':');
            t %= day;
        }
        append_padded(&mut result, t / hour, 2);
        result.push(':');
        t %= hour;
        append_padded(&mut result, t / minute, 2);
        result.push(':');
        t %= minute;
        append_padded(&mut result, t / second, 2);
        t %= second;
        if t != 0 || ts() < 1. {
            result.push('.');
            append_padded(&mut result, t, frac_w);
        }
        return result;
    }
    append_padded(&mut result, t / second, 0);
    result.push('.');
    append_padded(&mut result, t % second, frac_w);
    result
}

pub fn time2string(t: SUMOTime) -> String {
    time2string_hr(t, g_human_readable_time())
}

pub fn elapsed_ms2string(t: i64) -> String {
    if g_human_readable_time() {
        if steps2time(t) > 60. {
            time2string((t / 1000) * 1000)
        } else {
            format!("{}s", super::to_string::f64_to_string(t as f64 / 1000.0))
        }
    } else {
        time2string(t) + "s"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn roundtrip() {
        assert_eq!(string2time("1:00:00").unwrap(), 3_600_000);
        assert_eq!(string2time("12.5").unwrap(), 12_500);
        assert_eq!(time2string_hr(12_500, false), "12.50");
        assert_eq!(time2string_hr(3_725_000, true), "01:02:05");
        assert_eq!(time2string_hr(-1500, false), "-1.50");
        assert!(string2time("1::2").is_err()); // empty field -> EmptyData, as in C++
        assert!(!is_time("1::2"));
    }
}
