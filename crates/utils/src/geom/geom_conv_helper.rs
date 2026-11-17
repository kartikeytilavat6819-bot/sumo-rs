//! Port of utils/geom/GeomConvHelper.{h,cpp}
use super::{tokenize, write_error, Boundary, Position, PositionVector};
use crate::common::string_utils::to_double;

/// C++ StringUtils::toDouble error kinds as caught by GeomConvHelper
enum ParseErr {
    Empty,
    NotNumeric,
}

fn parse_double(s: &str) -> Result<f64, ParseErr> {
    if s.is_empty() {
        return Err(ParseErr::Empty);
    }
    to_double(s).map_err(|_| ParseErr::NotNumeric)
}

/// Builds a PositionVector from "x,y[,z][ x,y[,z]]*", reporting occurred errors.
/// Returns None where C++ sets ok = false (C++ default report = true).
pub fn parse_shape_reporting(
    shpdef: &str,
    objecttype: &str,
    objectid: Option<&str>,
    allow_empty: bool,
    report: bool,
) -> Option<PositionVector> {
    if shpdef.is_empty() {
        if !allow_empty {
            emit_error(report, "Shape", objecttype, objectid, "the shape is empty");
            return None;
        }
        return Some(PositionVector::new());
    }
    let mut shape = PositionVector::new();
    for st in tokenize(shpdef, " ") {
        let pos = tokenize(&st, ",");
        if pos.len() != 2 && pos.len() != 3 {
            emit_error(report, "Shape", objecttype, objectid, "the position is neither x,y nor x,y,z");
            return None;
        }
        let parsed: Result<Vec<f64>, ParseErr> = pos.iter().map(|s| parse_double(s)).collect();
        match parsed {
            Ok(v) if v.len() == 2 => shape.push(Position::new(v[0], v[1])),
            Ok(v) => shape.push(Position::new3(v[0], v[1], v[2])),
            Err(ParseErr::NotNumeric) => {
                emit_error(report, "Shape", objecttype, objectid, "not numeric position entry");
                return None;
            }
            Err(ParseErr::Empty) => {
                emit_error(report, "Shape", objecttype, objectid, "empty position entry");
                return None;
            }
        }
    }
    Some(shape)
}

/// Builds a boundary from "xmin,ymin,xmax,ymax", reporting occurred errors.
/// `offsets`: keep inverted values (xmin > xmax) rather than correcting them.
/// Returns None where C++ sets ok = false (C++ defaults report = true, offsets = false).
pub fn parse_boundary_reporting(def: &str, objecttype: &str, objectid: Option<&str>, report: bool, offsets: bool) -> Option<Boundary> {
    let st = tokenize(def, ",");
    if st.len() != 4 {
        emit_error(report, "Bounding box", objecttype, objectid, "mismatching entry number");
        return None;
    }
    let parsed: Result<Vec<f64>, ParseErr> = st.iter().map(|s| parse_double(s)).collect();
    match parsed {
        Ok(v) => {
            let (xmin, ymin, xmax, ymax) = (v[0], v[1], v[2], v[3]);
            if offsets {
                let mut res = Boundary::new();
                res.set_offsets(xmin, ymin, xmax, ymax);
                Some(res)
            } else {
                Some(Boundary::from_xy(xmin, ymin, xmax, ymax))
            }
        }
        Err(e) => {
            // "Shape" (not "Bounding box") as in C++
            let desc = match e {
                ParseErr::NotNumeric => "not numeric entry",
                ParseErr::Empty => "empty entry",
            };
            emit_error(report, "Shape", objecttype, objectid, desc);
            None
        }
    }
}

fn emit_error(report: bool, what: &str, objecttype: &str, objectid: Option<&str>, desc: &str) {
    if !report {
        return;
    }
    let msg = match objectid {
        None => format!("{what} of a(n) {objecttype} is broken: {desc}."),
        Some(id) => format!("{what} of {objecttype} '{id}' is broken: {desc}."),
    };
    write_error(&msg);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse() {
        let s = parse_shape_reporting("0,0 10.5,-2 1,2,3", "edge", Some("e"), false, false).unwrap();
        assert_eq!(s.to_string_prec(2), "0.00,0.00 10.50,-2.00 1.00,2.00,3.00");
        assert!(parse_shape_reporting("", "edge", None, false, false).is_none());
        assert_eq!(parse_shape_reporting("", "edge", None, true, false).unwrap().len(), 0);
        assert!(parse_shape_reporting("0,0  1,1", "edge", None, false, false).is_none());
        assert!(parse_shape_reporting("0,a", "edge", None, false, false).is_none());
        assert!(parse_shape_reporting("0,", "edge", None, false, false).is_none());
        let b = parse_boundary_reporting("10,0,0,5", "net", None, false, false).unwrap();
        assert_eq!((b.xmin(), b.xmax()), (0., 10.));
        let b = parse_boundary_reporting("10,0,0,5", "net", None, false, true).unwrap();
        assert_eq!((b.xmin(), b.xmax()), (10., 0.));
        assert!(parse_boundary_reporting("1,2,3", "net", None, false, false).is_none());
    }
}
