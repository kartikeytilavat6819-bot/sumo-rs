//! Port of netbuild/NBHelpers.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::string_utils::{starts_with, to_int};
use utils::common::util_exceptions::ProcessError;
use utils::geom::NUMERICAL_EPS;
use utils::{tlf, write_errorf};
/// C isspace() set: the whitespace that `strm >> token` skips
fn is_stream_space(c: char) -> bool {
    c.is_ascii_whitespace() || c as u32 == 11
}
/// reads a whole file like `ifstream` (ProcessError when it cannot be opened)
fn read_stream_or_throw(file: &str, err: String) -> String {
    match std::fs::read(file) {
        Ok(d) => String::from_utf8_lossy(&d).into_owned(),
        Err(_) => panic!("{}", ProcessError::new(err)),
    }
}
/// whitespace-separated tokens as `strm >> name` yields them; an empty stream still gives one empty name
fn stream_tokens(text: &str) -> Vec<&str> {
    let mut v: Vec<&str> = text.split(is_stream_space).filter(|t| !t.is_empty()).collect();
    if v.is_empty() {
        v.push("");
    }
    v
}

/// Some mathmatical methods for the computation of angles
/// C++ class `NBHelpers` (netbuild/NBHelpers.h)
pub struct NBHelpers {
}

impl NBHelpers {
    /// C++: `double NBHelpers::relAngle(double, double)` [static]
    /// computes the relative angle between the two angles
    // double
    // NBHelpers::relAngle(double angle1, double angle2) {
    //     angle2 -= angle1;
    //     while (angle2 > 180.) {
    //         angle2 -= 360.;
    //     }
    //     while (angle2 < -180.) {
    //         angle2 += 360.;
    //     }
    //     return angle2;
    // }
    #[allow(unused_variables)]
    pub fn rel_angle(angle1: f64, angle2: f64) -> f64 {
        let mut angle2 = angle2;
        angle2 -= angle1;
        while angle2 > 180. {
            angle2 -= 360.;
        }
        while angle2 < -180. {
            angle2 += 360.;
        }
        angle2
    }

    /// C++: `double NBHelpers::normRelAngle(double, double)` [static]
    /// ensure that reverse relAngles (>=179.999) always count as turnarounds (-180)
    // double
    // NBHelpers::normRelAngle(double angle1, double angle2) {
    //     double rel = relAngle(angle1, angle2);
    //     if (rel + NUMERICAL_EPS >= 180) {
    //         return -180;
    //     } else {
    //         return rel;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn norm_rel_angle(angle1: f64, angle2: f64) -> f64 {
        let rel = NBHelpers::rel_angle(angle1, angle2);
        if rel + NUMERICAL_EPS >= 180. {
            -180.
        } else {
            rel
        }
    }

    /// C++: `std::string NBHelpers::normalIDRepresentation(const std::string &)` [static]
    /// converts the numerical id to its "normal" string representation
    // std::string
    // NBHelpers::normalIDRepresentation(const std::string& id) {
    //     std::stringstream strm1(id);
    //     long numid;
    //     strm1 >> numid;
    //     std::stringstream strm2;
    //     strm2 << numid;
    //     return strm2.str();
    // }
    #[allow(unused_variables)]
    pub fn normal_id_representation(id: &str) -> String {
        // shortcut: C++ `long` is 64-bit here (MSVC long is 32-bit), switch to i32 if Windows output must match exactly
        let s = id.trim_start_matches(is_stream_space);
        let bytes = s.as_bytes();
        let mut end = 0;
        if end < bytes.len() && (bytes[end] == b'+' || bytes[end] == b'-') {
            end += 1;
        }
        let digits_start = end;
        while end < bytes.len() && bytes[end].is_ascii_digit() {
            end += 1;
        }
        // `strm >> numid`: no digits gives 0, overflow saturates to LONG_MAX / LONG_MIN
        let numid: i64 = if end == digits_start {
            0
        } else {
            s[..end].parse::<i64>().unwrap_or(if s.starts_with('-') { i64::MIN } else { i64::MAX })
        };
        numid.to_string()
    }

    /// C++: `double NBHelpers::distance(NBNode *, NBNode *)` [static]
    /// returns the distance between both nodes
    // double
    // NBHelpers::distance(NBNode* node1, NBNode* node2) {
    //     return node1->getPosition().distanceTo(node2->getPosition());
    // }
    #[allow(unused_variables)]
    pub fn distance(node1: Ptr<NBNode>, node2: Ptr<NBNode>) -> f64 {
        node1.get_position().distance_to(node2.get_position())
    }

    /// C++: `void NBHelpers::loadEdgesFromFile(const std::string &, std::set<std::string> &)` [static]
    /// Add edge ids defined in file (either ID or edge:ID per line) into the given set
    // void
    // NBHelpers::loadEdgesFromFile(const std::string& file, std::set<std::string>& into) {
    //     std::ifstream strm(file.c_str());
    //     if (!strm.good()) {
    //         throw ProcessError(TLF("Could not load names of edges to keep from '%'.", file));
    //     }
    //     while (strm.good()) {
    //         std::string name;
    //         strm >> name;
    //         into.insert(name);
    //         // maybe we're loading an edge-selection
    //         if (StringUtils::startsWith(name, "edge:")) {
    //             into.insert(name.substr(5));
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn load_edges_from_file(file: &str, into: &mut BTreeSet<String>) {
        let data = read_stream_or_throw(file, tlf!("Could not load names of edges to keep from '%'.", file));
        for name in stream_tokens(&data) {
            into.insert(name.to_string());
            // maybe we're loading an edge-selection
            if starts_with(name, "edge:") {
                into.insert(name["edge:".len()..].to_string());
            }
        }
    }

    /// C++: `void NBHelpers::loadPrefixedIDsFomFile(const std::string &, const std::string, std::set<std::string> &)` [static]
    /// Add prefixed ids defined in file
    // void
    // NBHelpers::loadPrefixedIDsFomFile(const std::string& file, const std::string prefix, std::set<std::string>& into) {
    //     std::ifstream strm(file.c_str());
    //     if (!strm.good()) {
    //         throw ProcessError(TLF("Could not load IDs from '%'.", file));
    //     }
    //     while (strm.good()) {
    //         std::string prefixedID;
    //         strm >> prefixedID;
    //         if (StringUtils::startsWith(prefixedID, prefix)) {
    //             into.insert(prefixedID.substr(prefix.size()));
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn load_prefixed_i_ds_fom_file(file: &str, prefix: String, into: &mut BTreeSet<String>) {
        let data = read_stream_or_throw(file, tlf!("Could not load IDs from '%'.", file));
        for prefixed_id in stream_tokens(&data) {
            if starts_with(prefixed_id, &prefix) {
                into.insert(prefixed_id[prefix.len()..].to_string());
            }
        }
    }

    /// C++: `void NBHelpers::interpretLaneID(const std::string &, std::string &, int &)` [static]
    /// parses edge-id and index from lane-id
    // void
    // NBHelpers::interpretLaneID(const std::string& lane_id, std::string& edge_id, int& index) {
    //     // assume lane_id = edge_id + '_' + index
    //     const std::string::size_type sep_index = lane_id.rfind('_');
    //     if (sep_index == std::string::npos) {
    //         WRITE_ERRORF(TL("Invalid lane id '%' (missing '_')."), lane_id);
    //     }
    //     edge_id = lane_id.substr(0, sep_index);
    //     std::string index_string = lane_id.substr(sep_index + 1);
    //     try {
    //         index = StringUtils::toInt(index_string);
    //     } catch (NumberFormatException&) {
    //         WRITE_ERRORF(TL("Invalid lane index '%' for lane '%'."), index_string, lane_id);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn interpret_lane_id(lane_id: &str, edge_id: &mut String, index: &mut i32) {
        // assume lane_id = edge_id + '_' + index
        let sep_index = lane_id.rfind('_');
        if sep_index.is_none() {
            write_errorf!("Invalid lane id '%' (missing '_').", lane_id);
        }
        // substr(npos) and substr(npos + 1) both cover the whole string
        let (edge_part, index_part) = match sep_index {
            Some(i) => (&lane_id[..i], &lane_id[i + 1..]),
            None => (lane_id, lane_id),
        };
        *edge_id = edge_part.to_string();
        let index_string = index_part.to_string();
        match to_int(&index_string) {
            Ok(v) => *index = v,
            Err(_) => write_errorf!("Invalid lane index '%' for lane '%'.", index_string, lane_id),
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rel_angle_wraps_into_minus_180_180() {
        assert_eq!(NBHelpers::rel_angle(0., 270.), -90.);
        assert_eq!(NBHelpers::rel_angle(350., 10.), 20.);
    }

    #[test]
    fn norm_rel_angle_maps_turnaround_to_minus_180() {
        assert_eq!(NBHelpers::norm_rel_angle(0., 180.), -180.);
        assert_eq!(NBHelpers::norm_rel_angle(0., 90.), 90.);
    }

    #[test]
    fn normal_id_representation_strips_zeros_and_trailing_text() {
        assert_eq!(NBHelpers::normal_id_representation("007"), "7");
        assert_eq!(NBHelpers::normal_id_representation("-5x"), "-5");
        assert_eq!(NBHelpers::normal_id_representation("abc"), "0");
    }

    #[test]
    fn interpret_lane_id_splits_at_last_underscore() {
        let mut edge = String::new();
        let mut index = -1;
        NBHelpers::interpret_lane_id("e_1_2", &mut edge, &mut index);
        assert_eq!(edge, "e_1");
        assert_eq!(index, 2);
    }

    #[test]
    fn stream_tokens_gives_one_empty_name_for_empty_input() {
        assert_eq!(stream_tokens(""), vec![""]);
        assert_eq!(stream_tokens(" a  b "), vec!["a", "b"]);
    }
}
