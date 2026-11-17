//! Minimal stand-ins for pieces of utils/common and utils/geom that xml and iodevices need.
// shortcut: Position/PositionVector/Boundary below are xml-local copies, because sumo_sax_attributes reads
// PositionVector.0 directly; switch them to utils::geom once that code uses the geom types
use crate::common::msg_handler::{with_instance, MsgType};

pub use crate::common::msg_handler::{write_error, write_message, write_warning};
pub use crate::common::util_exceptions::ProcessError;

/// MsgHandler::getErrorInstance()->wasInformed()
pub fn error_was_informed() -> bool {
    with_instance(MsgType::Error, |h| h.was_informed())
}
/// MsgHandler::getErrorInstance()->clear()
pub fn clear_errors() {
    with_instance(MsgType::Error, |h| h.clear(true));
}

/// StringTokenizer(s).getVector(): split at chars <= ' ', no empty tokens
pub fn tokenize_white(s: &str) -> Vec<&str> {
    s.split(|c: char| c <= ' ').filter(|t| !t.is_empty()).collect()
}
/// StringTokenizer(s, sep).getVector(): keeps empty tokens, but "" gives no token
pub fn tokenize<'a>(s: &'a str, sep: &str) -> Vec<&'a str> {
    if s.is_empty() { Vec::new() } else { s.split(sep).collect() }
}

/// utils/geom/Position
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
impl Position {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Position { x, y, z }
    }
}

/// utils/geom/PositionVector
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PositionVector(pub Vec<Position>);

/// utils/geom/Boundary (only the 2D constructor)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Boundary {
    pub xmin: f64,
    pub xmax: f64,
    pub ymin: f64,
    pub ymax: f64,
    pub zmin: f64,
    pub zmax: f64,
    pub was_initialised: bool,
}
impl Default for Boundary {
    fn default() -> Self {
        Boundary { xmin: 10000000000.0, xmax: -10000000000.0, ymin: 10000000000.0, ymax: -10000000000.0, zmin: 10000000000.0, zmax: -10000000000.0, was_initialised: false }
    }
}
impl Boundary {
    pub fn new(x1: f64, y1: f64, x2: f64, y2: f64) -> Self {
        let mut b = Boundary::default();
        b.add(x1, y1, 0.);
        b.add(x2, y2, 0.);
        b
    }
    pub fn add(&mut self, x: f64, y: f64, z: f64) {
        if !self.was_initialised {
            (self.xmin, self.xmax, self.ymin, self.ymax, self.zmin, self.zmax) = (x, x, y, y, z, z);
        } else {
            self.xmin = if self.xmin < x { self.xmin } else { x };
            self.xmax = if self.xmax > x { self.xmax } else { x };
            self.ymin = if self.ymin < y { self.ymin } else { y };
            self.ymax = if self.ymax > y { self.ymax } else { y };
            self.zmin = if self.zmin < z { self.zmin } else { z };
            self.zmax = if self.zmax > z { self.zmax } else { z };
        }
        self.was_initialised = true;
    }
}

/// FileHelpers::isSocket
pub fn is_socket(name: &str) -> bool {
    match name.find(':') {
        Some(p) => p > 1 || name.starts_with('['),
        None => false,
    }
}
/// FileHelpers::isAbsolute
pub fn is_absolute(path: &str) -> bool {
    let b = path.as_bytes();
    is_socket(path) || b.first() == Some(&b'/') || b.first() == Some(&b'\\') || b.get(1) == Some(&b':') || path == "nul" || path == "NUL"
}
/// FileHelpers::getConfigurationRelative
pub fn get_configuration_relative(config_path: &str, path: &str) -> String {
    let dir = config_path.rfind(['\\', '/']).map_or("", |i| &config_path[..=i]);
    format!("{dir}{path}")
}
/// FileHelpers::isReadable
pub fn is_readable(path: &str) -> bool {
    let p = path.trim_end_matches(['/', '\\']);
    !p.is_empty() && std::fs::metadata(p).is_ok()
}
/// FileHelpers::prependToLastPathComponent
pub fn prepend_to_last_path_component(prefix: &str, path: &str) -> String {
    match path.rfind(['\\', '/']) {
        None => format!("{prefix}{path}"),
        Some(i) => format!("{}{prefix}{}", &path[..=i], &path[i + 1..]),
    }
}
/// FileHelpers::appendBeforeExtension (checkSep = true)
pub fn append_before_extension(path: &str, suffix: &str) -> String {
    let (dir, file) = match path.rfind(['\\', '/']) {
        None => ("", path),
        Some(i) => (&path[..=i], &path[i + 1..]),
    };
    let mut comps: Vec<String> = tokenize(file, ".").into_iter().map(String::from).collect();
    for i in (0..comps.len()).rev() {
        // assume anything after a dot with less then 5 letters is part of the extension
        if i == 0 || comps[i].len() > 4 {
            comps[i] += suffix;
            break;
        }
    }
    format!("{dir}{}", comps.join("."))
}

/// StringUtils::escapeXML(orig, maskDoubleHyphen = false)
pub fn escape_xml(orig: &str) -> String {
    let r = orig.replace('&', "&amp;").replace('>', "&gt;").replace('<', "&lt;").replace('"', "&quot;");
    let r: String = r.chars().filter(|&c| !('\x01'..' ').contains(&c)).collect();
    r.replace('\'', "&apos;")
}

/// StringUtils::isoTimeString()
// shortcut: prints UTC with a +00:00 offset (std has no local time zone); XML comments are not compared by the tests
pub fn iso_time_string() -> String {
    let d = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default();
    let secs = d.as_secs() as i64;
    let (days, rem) = (secs.div_euclid(86400), secs.rem_euclid(86400));
    // civil_from_days (Howard Hinnant)
    let z = days + 719468;
    let era = z.div_euclid(146097);
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = yoe + era * 400 + i64::from(month <= 2);
    format!("{year:04}-{month:02}-{day:02}T{:02}:{:02}:{:02}.{:06}+00:00", rem / 3600, rem % 3600 / 60, rem % 60, d.subsec_micros())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn helpers() {
        assert_eq!(tokenize("1,,2,", ","), vec!["1", "", "2", ""]);
        assert!(tokenize("", ",").is_empty());
        assert_eq!(tokenize_white("  a \t b\n"), vec!["a", "b"]);
        assert_eq!(append_before_extension("out/a.xml", "_x"), "out/a_x.xml");
        assert_eq!(append_before_extension("tripinfo.out.xml", "_x"), "tripinfo_x.out.xml");
        assert_eq!(prepend_to_last_path_component("p_", "d/a.xml"), "d/p_a.xml");
        assert_eq!(escape_xml("a<'b'>&\"\x01"), "a&lt;&apos;b&apos;&gt;&amp;&quot;");
        assert_eq!(iso_time_string().len(), "2026-11-11T00:00:00.000000+00:00".len());
    }
}
