//! Port of netbuild/NBHelpers.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

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
        todo!("port NBHelpers::relAngle")
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
        todo!("port NBHelpers::normRelAngle")
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
        todo!("port NBHelpers::normalIDRepresentation")
    }

    /// C++: `double NBHelpers::distance(NBNode *, NBNode *)` [static]
    /// returns the distance between both nodes
    // double
    // NBHelpers::distance(NBNode* node1, NBNode* node2) {
    //     return node1->getPosition().distanceTo(node2->getPosition());
    // }
    #[allow(unused_variables)]
    pub fn distance(node1: Ptr<NBNode>, node2: Ptr<NBNode>) -> f64 {
        todo!("port NBHelpers::distance")
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
        todo!("port NBHelpers::loadEdgesFromFile")
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
        todo!("port NBHelpers::loadPrefixedIDsFomFile")
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
        todo!("port NBHelpers::interpretLaneID")
    }

}

