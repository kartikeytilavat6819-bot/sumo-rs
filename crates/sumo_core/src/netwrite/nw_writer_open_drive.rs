//! Port of netwrite/NWWriter_OpenDrive.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::sumo_vehicle_class::{is_railway, SVCPermissions, SVC_ALL, SVC_BICYCLE, SVC_PASSENGER, SVC_PEDESTRIAN, SVC_RAIL, SVC_RAIL_ELECTRIC, SVC_RAIL_FAST, SVC_RAIL_URBAN, SVC_TRAM, SVC_UNSPECIFIED};
use utils::common::to_string::to_string;

/// C++ typedef `std::map<std::string, std::pair<std::set<int>, std::set<LinkDirection>>> NWWriter_OpenDrive::SignalLanes`
pub type NWWriter_OpenDrive_SignalLanes = BTreeMap<String, (BTreeSet<i32>, BTreeSet<LinkDirection>)>;

/// Exporter writing networks using the openDRIVE format
/// C++ class `NWWriter_OpenDrive` (netwrite/NWWriter_OpenDrive.h)
pub struct NWWriter_OpenDrive {
}

// C++ static members of NWWriter_OpenDrive (port as module-level statics, see AGENTS.md):
//   static bool NWWriter_OpenDrive::lefthand;
//   static bool NWWriter_OpenDrive::LHLL;
//   static bool NWWriter_OpenDrive::LHRL;
thread_local! {
    static LEFTHAND: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    static LHLL: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    static LHRL: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

/// NWWriter_OpenDrive::lefthand
fn lefthand() -> bool {
    LEFTHAND.with(|c| c.get())
}

/// NWWriter_OpenDrive::lefthand = v
fn set_lefthand(v: bool) {
    LEFTHAND.with(|c| c.set(v));
}

/// NWWriter_OpenDrive::LHLL
fn lhll() -> bool {
    LHLL.with(|c| c.get())
}

/// NWWriter_OpenDrive::LHLL = v
fn set_lhll(v: bool) {
    LHLL.with(|c| c.set(v));
}

/// NWWriter_OpenDrive::LHRL
fn lhrl() -> bool {
    LHRL.with(|c| c.get())
}

/// NWWriter_OpenDrive::LHRL = v
fn set_lhrl(v: bool) {
    LHRL.with(|c| c.set(v));
}

/// C++ `#define INVALID_ID -1` (NWWriter_OpenDrive.cpp)
const INVALID_ID: i32 = -1;

/// C++ OptionsCont getters throw ProcessError; the options used here are always registered
/// Hand-written equivalent of the std::regex in NWWriter_OpenDrive::parseTrafficSignId:
/// `([A-Z]{2}):([0-9.]{1,6}|[A-Z]{1}[0-9.]{2,6})(?:\[(.*)\])?(?:-(.*))?` (full match).
/// Returns (country, id, bracket value, hyphen value); None when the regex does not match.
fn match_traffic_sign_id(s: &str) -> Option<(String, String, Option<String>, Option<String>)> {
    let b: Vec<char> = s.chars().collect();
    if b.len() < 3 || !(b[0].is_ascii_uppercase() && b[1].is_ascii_uppercase() && b[2] == ':') {
        return None;
    }
    let country: String = b[0..2].iter().collect();
    let rest: &[char] = &b[3..];
    let is_num = |c: char| c.is_ascii_digit() || c == '.';
    // candidate id lengths in alternation order, greedy within each alternative
    let mut lengths: Vec<usize> = Vec::new();
    let run = rest.iter().take_while(|c| is_num(**c)).count();
    for l in (1..=run.min(6)).rev() {
        lengths.push(l);
    }
    if !rest.is_empty() && rest[0].is_ascii_uppercase() {
        let run2 = rest[1..].iter().take_while(|c| is_num(**c)).count();
        for l in (2..=run2.min(6)).rev() {
            lengths.push(1 + l);
        }
    }
    for l in lengths {
        let id: String = rest[..l].iter().collect();
        if let Some((bracket, hyphen)) = match_traffic_sign_tail(&rest[l..]) {
            return Some((country, id, bracket, hyphen));
        }
    }
    None
}

/// Tail of the traffic sign regex: `(?:\[(.*)\])?(?:-(.*))?`
fn match_traffic_sign_tail(tail: &[char]) -> Option<(Option<String>, Option<String>)> {
    if tail.is_empty() {
        return Some((None, None));
    }
    if tail[0] == '[' {
        // greedy `.*` inside the brackets: prefer the last closing bracket that lets the rest match
        for j in (1..tail.len()).rev() {
            if tail[j] != ']' {
                continue;
            }
            let after = &tail[j + 1..];
            if after.is_empty() {
                return Some((Some(tail[1..j].iter().collect()), None));
            }
            if after[0] == '-' {
                return Some((Some(tail[1..j].iter().collect()), Some(after[1..].iter().collect())));
            }
        }
        return None;
    }
    if tail[0] == '-' {
        return Some((None, Some(tail[1..].iter().collect())));
    }
    None
}

fn or_panic<T, E: std::fmt::Display>(r: Result<T, E>) -> T {
    r.unwrap_or_else(|e| panic!("{e}"))
}

/// GeoConvHelper::getFinal() (copied; the closure form cannot be held across the writers)
fn get_final_geo() -> GeoConvHelper {
    GeoConvHelper::with_final(|g| g.clone())
}

/// C library ctime(&now): "Www Mmm dd hh:mm:ss yyyy\n" for a time in seconds since the epoch
// shortcut: formats in UTC (no timezone database), C ctime() uses local time; add chrono if local time matters
fn ctime_string(secs: i64) -> String {
    const WDAYS: [&str; 7] = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
    const MONTHS: [&str; 12] = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
    let days = secs.div_euclid(86400);
    let rem = secs.rem_euclid(86400);
    let (h, mi, s) = (rem / 3600, (rem % 3600) / 60, rem % 60);
    let wday = (days + 4).rem_euclid(7) as usize; // 1970-01-01 was a Thursday
    // civil date from day count (proleptic Gregorian)
    let z = days + 719468;
    let era = z.div_euclid(146097);
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = yoe + era * 400 + if m <= 2 { 1 } else { 0 };
    format!("{} {} {:>2} {:02}:{:02}:{:02} {}\n", WDAYS[wday], MONTHS[(m - 1) as usize], d, h, mi, s, y)
}

impl NWWriter_OpenDrive {
    /// C++: `void NWWriter_OpenDrive::writeNetwork(const OptionsCont &, NBNetBuilder &)` [static]
    /// Writes the network into a openDRIVE-file
    // void
    // NWWriter_OpenDrive::writeNetwork(const OptionsCont& oc, NBNetBuilder& nb) {
    //     // check whether an opendrive-file shall be generated
    //     if (!oc.isSet("opendrive-output")) {
    //         return;
    //     }
    //     const NBNodeCont& nc = nb.getNodeCont();
    //     const NBEdgeCont& ec = nb.getEdgeCont();
    //     const bool origNames = oc.getBool("output.original-names");
    //     lefthand = oc.getBool("lefthand");
    //     LHLL = lefthand && oc.getBool("opendrive-output.lefthand-left");
    //     LHRL = lefthand && !LHLL;
    //     const double straightThresh = DEG2RAD(oc.getFloat("opendrive-output.straight-threshold"));
    //     // some internal mapping containers
    //     int nodeID = 1;
    //     int edgeID = nc.size() * 10; // distinct from node ids
    //     StringBijection<int> edgeMap;
    //     StringBijection<int> nodeMap;
    //     //
    //     OutputDevice& device = OutputDevice::getDevice(oc.getString("opendrive-output"));
    //     OutputDevice::createDeviceByOption("opendrive-output", "OpenDRIVE");
    //     time_t now = time(nullptr);
    //     std::string dstr(ctime(&now));
    //     const Boundary& b = GeoConvHelper::getFinal().getConvBoundary();
    //     // write header
    //     device.openTag("header");
    //     device.writeAttr("revMajor", "1");
    //     device.writeAttr("revMinor", "4");
    //     device.writeAttr("name", "");
    //     device.writeAttr("version", "1.00");
    //     device.writeAttr("date", dstr.substr(0, dstr.length() - 1));
    //     device.writeAttr("north", b.ymax());
    //     device.writeAttr("south", b.ymin());
    //     device.writeAttr("east", b.xmax());
    //     device.writeAttr("west", b.xmin());
    //     /* @note obsolete in 1.4
    //     device.writeAttr("maxRoad", ec.size());
    //     device.writeAttr("maxJunc", nc.size());
    //     device.writeAttr("maxPrg", 0);
    //     */
    //     // write optional geo reference
    //     const GeoConvHelper& gch = GeoConvHelper::getFinal();
    //     if (gch.usingGeoProjection()) {
    //         device.openTag("geoReference");
    //         device.writePreformattedTag(" <![CDATA[\n "
    //                                     + gch.getProjString()
    //                                     + "\n]]>\n");
    //         device.closeTag();
    //         if (gch.getOffsetBase() != Position(0, 0)) {
    //             device.openTag("offset");
    //             device.writeAttr("x", -gch.getOffsetBase().x());
    //             device.writeAttr("y", -gch.getOffsetBase().y());
    //             device.writeAttr("z", -gch.getOffsetBase().z());
    //             device.writeAttr("hdg", 0);
    //             device.closeTag();
    //         }
    //     }
    //     device.closeTag();
    //
    //     SignalLanes signalLanes;
    //
    //     mapmatchRoadObjects(nb.getShapeCont(), ec);
    //
    //     PositionVector crosswalk_shape;
    //     std::map<std::string, std::vector<std::string>> crosswalksByEdge;
    //     for (auto it = nc.begin(); it != nc.end(); ++it) {
    //         NBNode* n = it->second;
    //         auto crosswalks = n->getCrossings();
    //         for (const auto& cw : n->getCrossings()) {
    //             // getting from crosswalk line to a full shape
    //             crosswalk_shape = cw->shape;
    //             PositionVector rightside = cw->shape;
    //             try {
    //                 crosswalk_shape.move2side(cw->width / 2);
    //                 rightside.move2side(cw->width / -2);
    //             } catch (InvalidArgument&) { }
    //             rightside = rightside.reverse();
    //             crosswalk_shape.append(rightside);
    //             auto crosswalkId = cw->id;
    //             nb.getShapeCont().addPolygon(crosswalkId, "crosswalk", RGBColor::DEFAULT_COLOR, 0,
    //                                          Shape::DEFAULT_ANGLE, Shape::DEFAULT_IMG_FILE,
    //                                          crosswalk_shape, false, true, 1, false, crosswalkId);
    //             SUMOPolygon* cwp = nb.getShapeCont().getPolygons().get(crosswalkId);
    //             cwp->setParameter("length", toString(cw->width));
    //             cwp->setParameter("width", toString(cw->shape.length2D()));
    //             cwp->setParameter("hdg", "0");
    //             crosswalksByEdge[cw->edges[0]->getID()].push_back(crosswalkId);
    //         }
    //     }
    //
    //     // write normal edges (road)
    //     for (std::map<std::string, NBEdge*>::const_iterator i = ec.begin(); i != ec.end(); ++i) {
    //         const NBEdge* e = (*i).second;
    //         const int fromNodeID = e->getIncomingEdges().size() > 0 ? getID(e->getFromNode()->getID(), nodeMap, nodeID) : INVALID_ID;
    //         const int toNodeID = e->getConnections().size() > 0 ? getID(e->getToNode()->getID(), nodeMap, nodeID) : INVALID_ID;
    //         writeNormalEdge(device, e,
    //                         getID(e->getID(), edgeMap, edgeID),
    //                         fromNodeID, toNodeID,
    //                         origNames, straightThresh,
    //                         nb.getShapeCont(),
    //                         signalLanes,
    //                         crosswalksByEdge[e->getID()]);
    //     }
    //     device.lf();
    //
    //     // write junction-internal edges (road). In OpenDRIVE these are called 'paths' or 'connecting roads'
    //     OutputDevice_String junctionOSS(3);
    //     for (std::map<std::string, NBNode*>::const_iterator i = nc.begin(); i != nc.end(); ++i) {
    //         NBNode* n = (*i).second;
    //         int connectionID = 0; // unique within a junction
    //         const int nID = getID(n->getID(), nodeMap, nodeID);
    //         if (n->numNormalConnections() > 0) {
    //             junctionOSS << "    <junction name=\"" << n->getID() << "\" id=\"" << nID << "\">\n";
    //         }
    //         std::vector<NBEdge*> incoming = (*i).second->getIncomingEdges();
    //         if (lefthand) {
    //             std::reverse(incoming.begin(), incoming.end());
    //         }
    //         for (NBEdge* inEdge : incoming) {
    //             std::string centerMark = "none";
    //             const int inEdgeID = getID(inEdge->getID(), edgeMap, edgeID);
    //             // group parallel edges
    //             const NBEdge* outEdge = nullptr;
    //             bool isOuterEdge = true; // determine where a solid outer border should be drawn
    //             int lastFromLane = -1;
    //             std::vector<NBEdge::Connection> parallel;
    //             std::vector<NBEdge::Connection> connections = inEdge->getConnections();
    //             for (const NBEdge::Connection& c : connections) {
    //                 assert(c.toEdge != 0);
    //                 if (outEdge != c.toEdge || c.fromLane == lastFromLane) {
    //                     if (outEdge != nullptr) {
    //                         if (isOuterEdge) {
    //                             addPedestrianConnection(inEdge, outEdge, parallel);
    //                         }
    //                         connectionID = writeInternalEdge(device, junctionOSS, inEdge, nID,
    //                                                          getID(parallel.back().getInternalLaneID(), edgeMap, edgeID),
    //                                                          inEdgeID,
    //                                                          getID(outEdge->getID(), edgeMap, edgeID),
    //                                                          connectionID,
    //                                                          parallel, isOuterEdge, straightThresh, centerMark,
    //                                                          signalLanes);
    //                         parallel.clear();
    //                         isOuterEdge = false;
    //                     }
    //                     outEdge = c.toEdge;
    //                 }
    //                 lastFromLane = c.fromLane;
    //                 parallel.push_back(c);
    //             }
    //             if (isOuterEdge) {
    //                 addPedestrianConnection(inEdge, outEdge, parallel);
    //             }
    //             if (!parallel.empty()) {
    //                 if (!lefthand && (n->geometryLike() || inEdge->isTurningDirectionAt(outEdge))) {
    //                     centerMark = "solid";
    //                 }
    //                 connectionID = writeInternalEdge(device, junctionOSS, inEdge, nID,
    //                                                  getID(parallel.back().getInternalLaneID(), edgeMap, edgeID),
    //                                                  inEdgeID,
    //                                                  getID(outEdge->getID(), edgeMap, edgeID),
    //                                                  connectionID,
    //                                                  parallel, isOuterEdge, straightThresh, centerMark,
    //                                                  signalLanes);
    //                 parallel.clear();
    //             }
    //         }
    //         if (n->numNormalConnections() > 0) {
    //             junctionOSS << "    </junction>\n";
    //         }
    //     }
    //     device.lf();
    //     // write controllers
    //     for (std::map<std::string, NBNode*>::const_iterator i = nc.begin(); i != nc.end(); ++i) {
    //         NBNode* n = (*i).second;
    //         if (n->isTLControlled()) {
    //             NBTrafficLightDefinition* tl = *n->getControllingTLS().begin();
    //             std::set<std::string> ids;
    //             device.openTag("controller");
    //             device.writeAttr("id", tl->getID());
    //             for (const NBConnection& c : tl->getControlledLinks()) {
    //                 const std::string id = tl->getID() + "_" + toString(c.getTLIndex());
    //                 if (ids.count(id) == 0) {
    //                     ids.insert(id);
    //                     device.openTag("control");
    //                     device.writeAttr("signalId", id);
    //                     device.closeTag();
    //                 }
    //             }
    //             device.closeTag();
    //         }
    //     }
    //     // write junctions (junction)
    //     device << junctionOSS.getString();
    //
    //     device.closeTag();
    //     device.close();
    // }
    #[allow(unused_variables)]
    pub fn write_network(oc: &OptionsCont, nb: &mut NBNetBuilder) {
        // check whether an opendrive-file shall be generated
        if !or_panic(oc.is_set("opendrive-output", true)) {
            return;
        }
        let nc = &nb.myNodeCont;
        let ec = &nb.myEdgeCont;
        let orig_names = or_panic(oc.get_bool("output.original-names"));
        set_lefthand(or_panic(oc.get_bool("lefthand")));
        set_lhll(lefthand() && or_panic(oc.get_bool("opendrive-output.lefthand-left")));
        set_lhrl(lefthand() && !lhll());
        let straight_thresh = deg2rad(or_panic(oc.get_float("opendrive-output.straight-threshold")));
        // some internal mapping containers
        let mut node_id: i32 = 1;
        let mut edge_id: i32 = nc.size() * 10; // distinct from node ids
        let mut edge_map: StringBijection<i32> = StringBijection::default();
        let mut node_map: StringBijection<i32> = StringBijection::default();
        //
        let dev_name = or_panic(oc.get_string("opendrive-output"));
        let device_ref = or_panic(OutputDevice::get_device(&dev_name, true));
        or_panic(OutputDevice::create_device_by_option("opendrive-output", "OpenDRIVE", "", 2));
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs() as i64).unwrap_or(0);
        let dstr = ctime_string(now);
        let gch = get_final_geo();
        let b = gch.get_conv_boundary();
        let mut device = device_ref.borrow_mut();
        // write header
        device.open_tag("header");
        device.write_attr("revMajor", "1");
        device.write_attr("revMinor", "4");
        device.write_attr("name", "");
        device.write_attr("version", "1.00");
        device.write_attr("date", &dstr[..dstr.len() - 1]);
        device.write_attr("north", &b.ymax());
        device.write_attr("south", &b.ymin());
        device.write_attr("east", &b.xmax());
        device.write_attr("west", &b.xmin());
        // write optional geo reference
        if gch.using_geo_projection() {
            device.open_tag("geoReference");
            device.write_preformatted_tag(&format!(" <![CDATA[\n {}\n]]>\n", gch.get_proj_string()));
            device.close_tag("");
            if gch.get_offset_base() != Position::new(0.0, 0.0) {
                let ob = gch.get_offset_base();
                device.open_tag("offset");
                device.write_attr("x", &-ob.x());
                device.write_attr("y", &-ob.y());
                device.write_attr("z", &-ob.z());
                device.write_attr("hdg", &0i32);
                device.close_tag("");
            }
        }
        device.close_tag("");

        let mut signal_lanes: NWWriter_OpenDrive_SignalLanes = BTreeMap::new();

        Self::mapmatch_road_objects(&nb.myShapeCont, ec);

        let mut crosswalk_shape: PositionVector;
        let mut crosswalks_by_edge: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for n in nc.myNodes.values() {
            let n: Ptr<NBNode> = *n;
            let _crosswalks = n.get_crossings();
            for cw in n.get_crossings() {
                // getting from crosswalk line to a full shape
                crosswalk_shape = cw.shape.clone();
                let mut rightside: PositionVector = cw.shape.clone();
                // C++ try { move2side(...) } catch (InvalidArgument&) { }: move2side throws only in move2sideCustom,
                // so the catch never fires here and is dropped
                crosswalk_shape.move2side(cw.width / 2.0, 100.0);
                rightside.move2side(cw.width / -2.0, 100.0);
                rightside = rightside.reverse();
                crosswalk_shape.append(&rightside, 2.0);
                let crosswalk_id: String = cw.id.clone();
                nb.myShapeCont.add_polygon(&crosswalk_id, "crosswalk", &RGBColor::DEFAULT_COLOR, 0.0, DEFAULT_ANGLE, DEFAULT_IMG_FILE, &crosswalk_shape, false, true, 1.0, false, &crosswalk_id);
                let cwp: Ptr<SUMOPolygon> = nb.myShapeCont.get_polygons()[&crosswalk_id];
                cwp.m().base2.set_parameter("length", &to_string(&cw.width));
                cwp.m().base2.set_parameter("width", &to_string(&cw.shape.length2d()));
                cwp.m().base2.set_parameter("hdg", "0");
                crosswalks_by_edge.entry(cw.edges[0].get_id().to_string()).or_default().push(crosswalk_id.clone());
            }
        }

        // write normal edges (road)
        for e in ec.myEdges.values() {
            let e: Ptr<NBEdge> = *e;
            let from_node_id = if e.get_incoming_edges().len() > 0 { Self::get_id(e.get_from_node().get_id(), &mut node_map, &mut node_id) } else { INVALID_ID };
            let to_node_id = if e.get_connections().len() > 0 { Self::get_id(e.get_to_node().get_id(), &mut node_map, &mut node_id) } else { INVALID_ID };
            let edge_num = Self::get_id(e.get_id(), &mut edge_map, &mut edge_id);
            let crosswalks = crosswalks_by_edge.entry(e.get_id().to_string()).or_default().clone();
            Self::write_normal_edge(&mut device, e, edge_num, from_node_id, to_node_id, orig_names, straight_thresh, &nb.myShapeCont, &mut signal_lanes, &crosswalks);
        }
        device.lf();

        // write junction-internal edges (road). In OpenDRIVE these are called 'paths' or 'connecting roads'
        let mut junction_oss = OutputDevice::new_string(3);
        for n in nc.myNodes.values() {
            let n: Ptr<NBNode> = *n;
            let mut connection_id: i32 = 0; // unique within a junction
            let n_id = Self::get_id(n.get_id(), &mut node_map, &mut node_id);
            if n.num_normal_connections() > 0 {
                junction_oss.write(format!("    <junction name=\"{}\" id=\"{}\">\n", n.get_id(), n_id).as_str());
            }
            let mut incoming = n.get_incoming_edges().clone();
            if lefthand() {
                incoming.reverse();
            }
            for in_edge in incoming {
                let mut center_mark: String = "none".to_string();
                let in_edge_id = Self::get_id(in_edge.get_id(), &mut edge_map, &mut edge_id);
                // group parallel edges
                let mut out_edge: Ptr<NBEdge> = Ptr::null();
                let mut is_outer_edge = true; // determine where a solid outer border should be drawn
                let mut last_from_lane: i32 = -1;
                let mut parallel: Vec<NBEdge_Connection> = Vec::new();
                let connections: Vec<NBEdge_Connection> = in_edge.get_connections().clone();
                for c in connections {
                    assert!(!c.toEdge.is_null());
                    if out_edge != c.toEdge || c.fromLane == last_from_lane {
                        if !out_edge.is_null() {
                            if is_outer_edge {
                                Self::add_pedestrian_connection(in_edge, out_edge, &mut parallel);
                            }
                            // determinism: C++ argument evaluation order is unspecified; the id lookups run left to right here
                            let lane_id = Self::get_id(&parallel.last().unwrap().get_internal_lane_id(), &mut edge_map, &mut edge_id);
                            let out_edge_id = Self::get_id(out_edge.get_id(), &mut edge_map, &mut edge_id);
                            connection_id = Self::write_internal_edge(&mut device, &mut junction_oss, in_edge, n_id, lane_id, in_edge_id, out_edge_id, connection_id, &parallel, is_outer_edge, straight_thresh, &center_mark, &mut signal_lanes);
                            parallel.clear();
                            is_outer_edge = false;
                        }
                        out_edge = c.toEdge;
                    }
                    last_from_lane = c.fromLane;
                    parallel.push(c);
                }
                if is_outer_edge {
                    Self::add_pedestrian_connection(in_edge, out_edge, &mut parallel);
                }
                if !parallel.is_empty() {
                    if !lefthand() && (n.geometry_like() || in_edge.is_turning_direction_at(out_edge)) {
                        center_mark = "solid".to_string();
                    }
                    let lane_id = Self::get_id(&parallel.last().unwrap().get_internal_lane_id(), &mut edge_map, &mut edge_id);
                    let out_edge_id = Self::get_id(out_edge.get_id(), &mut edge_map, &mut edge_id);
                    connection_id = Self::write_internal_edge(&mut device, &mut junction_oss, in_edge, n_id, lane_id, in_edge_id, out_edge_id, connection_id, &parallel, is_outer_edge, straight_thresh, &center_mark, &mut signal_lanes);
                    parallel.clear();
                }
            }
            if n.num_normal_connections() > 0 {
                junction_oss.write("    </junction>\n");
            }
        }
        device.lf();
        // write controllers
        for n in nc.myNodes.values() {
            let n: Ptr<NBNode> = *n;
            if n.is_tl_controlled() {
                let tl: Ptr<NBTrafficLightDefinition> = *n.get_controlling_tls().iter().next().unwrap();
                let mut ids: BTreeSet<String> = BTreeSet::new();
                device.open_tag("controller");
                device.write_attr("id", tl.get_id());
                for c in tl.get_controlled_links().iter() {
                    let id = format!("{}_{}", tl.get_id(), c.get_tl_index());
                    if !ids.contains(&id) {
                        ids.insert(id.clone());
                        device.open_tag("control");
                        device.write_attr("signalId", id.as_str());
                        device.close_tag("");
                    }
                }
                device.close_tag("");
            }
        }
        // write junctions (junction)
        device.write(&junction_oss.get_string());

        device.close_tag("");
        or_panic(device.close());
    }

    /// C++: `std::string NWWriter_OpenDrive::getDividerType(const NBEdge *)` [static, protected]
    /// retrieve divider type
    // std::string
    // NWWriter_OpenDrive::getDividerType(const NBEdge* e) {
    //     std::map<std::string, std::string> dividerTypeMapping;
    //     dividerTypeMapping["solid_line"] = "solid";
    //     dividerTypeMapping["dashed_line"] = "broken";
    //     dividerTypeMapping["double_solid_line"] = "solid solid";
    //     dividerTypeMapping["no"] = "none";
    //
    //     // defaulting to solid as in the original code
    //     std::string dividerType = "solid";
    //
    //     if (e->getParametersMap().count("divider") > 0) {
    //         std::string divider = e->getParametersMap().find("divider")->second;
    //         if (dividerTypeMapping.count(divider) > 0) {
    //             dividerType = dividerTypeMapping.find(divider)->second;
    //         }
    //     }
    //     return dividerType;
    // }
    #[allow(unused_variables)]
    pub fn get_divider_type(e: Ptr<NBEdge>) -> String {
        let mut divider_type_mapping: BTreeMap<&str, &str> = BTreeMap::new();
        divider_type_mapping.insert("solid_line", "solid");
        divider_type_mapping.insert("dashed_line", "broken");
        divider_type_mapping.insert("double_solid_line", "solid solid");
        divider_type_mapping.insert("no", "none");

        // defaulting to solid as in the original code
        let mut divider_type: String = "solid".to_string();

        if let Some(divider) = e.base2.get_parameters_map().get("divider") {
            if let Some(mapped) = divider_type_mapping.get(divider.as_str()) {
                divider_type = mapped.to_string();
            }
        }
        divider_type
    }

    /// C++: `void NWWriter_OpenDrive::writeNormalEdge(OutputDevice &, const NBEdge *, int, int, int, const bool, const double, const ShapeContainer &, SignalLanes &, const std::vector<std::string> &)` [static, protected]
    /// write normal edge to device
    // void
    // NWWriter_OpenDrive::writeNormalEdge(OutputDevice& device, const NBEdge* e,
    //                                     int edgeID, int fromNodeID, int toNodeID,
    //                                     const bool origNames,
    //                                     const double straightThresh,
    //                                     const ShapeContainer& shc,
    //                                     SignalLanes& signalLanes,
    //                                     const std::vector<std::string>& crossings) {
    //     // buffer output because some fields are computed out of order
    //     OutputDevice_String elevationOSS(3);
    //     elevationOSS.setPrecision(8);
    //     OutputDevice_String planViewOSS(2);
    //     planViewOSS.setPrecision(8);
    //     double length = 0;
    //
    //     planViewOSS.openTag("planView");
    //     // for the shape we need to use the leftmost border of the leftmost lane
    //     const std::vector<NBEdge::Lane>& lanes = e->getLanes();
    //     PositionVector ls = getInnerLaneBorder(e);
    // #ifdef DEBUG_SMOOTH_GEOM
    //     if (DEBUGCOND) {
    //         std::cout << "write planview for edge " << e->getID() << "\n";
    //     }
    // #endif
    //
    //     if (ls.size() == 2 || e->getPermissions() == SVC_PEDESTRIAN) {
    //         // foot paths may contain sharp angles
    //         length = writeGeomLines(ls, planViewOSS, elevationOSS);
    //     } else {
    //         bool ok = writeGeomSmooth(ls, e->getSpeed(), planViewOSS, elevationOSS, straightThresh, length);
    //         if (!ok) {
    //             WRITE_WARNINGF(TL("Could not compute smooth shape for edge '%'."), e->getID());
    //         }
    //     }
    //     planViewOSS.closeTag();
    //
    //     device.openTag("road");
    //     device.writeAttr("name", StringUtils::escapeXML(e->getStreetName()));
    //     device.setPrecision(8); // length requires higher precision
    //     device.writeAttr("length", MAX2(POSITION_EPS, length));
    //     device.setPrecision(gPrecision);
    //     device.writeAttr("id", edgeID);
    //     device.writeAttr("junction", -1);
    //     if (fromNodeID != INVALID_ID || toNodeID != INVALID_ID) {
    //         device.openTag("link");
    //         if (fromNodeID != INVALID_ID) {
    //             device.openTag("predecessor");
    //             device.writeAttr("elementType", "junction");
    //             device.writeAttr("elementId", fromNodeID);
    //             device.closeTag();
    //         }
    //         if (toNodeID != INVALID_ID) {
    //             device.openTag("successor");
    //             device.writeAttr("elementType", "junction");
    //             device.writeAttr("elementId", toNodeID);
    //             device.closeTag();
    //         }
    //         device.closeTag();
    //     }
    //     device.openTag("type").writeAttr("s", 0).writeAttr("type", "town").closeTag();
    //     device << planViewOSS.getString();
    //     writeElevationProfile(ls, device, elevationOSS);
    //     device << "        <lateralProfile/>\n";
    //     device << "        <lanes>\n";
    //     device << "            <laneSection s=\"0\">\n";
    //     const std::string centerMark = e->getPermissions(e->getNumLanes() - 1) == 0 ? "none" : getDividerType(e);
    //     if (!LHLL) {
    //         writeEmptyCenterLane(device, centerMark, 0.13);
    //     }
    //     const std::string side = LHLL ? "left" : "right";
    //     device << "                <" << side << ">\n";
    //     const int numLanes = e->getNumLanes();
    //     for (int jRH = numLanes; --jRH >= 0;) {
    //         // XODR always has the lanes left to right (by default this is
    //         // inner-to-outer but in LH networks its outer-to-inner)
    //         const int j = lefthand ? numLanes - 1 - jRH : jRH;
    //         std::string laneType = e->getLaneStruct(j).type;
    //         if (laneType == "") {
    //             laneType = getLaneType(e->getPermissions(j));
    //         }
    //         device << "                    <lane id=\"" << s2x(j, numLanes) << "\" type=\"" << laneType << "\" level=\"true\">\n";
    //         device << "                        <link/>\n";
    //         // this could be used for geometry-link junctions without u-turn,
    //         // predecessor and successors would be lane indices,
    //         // road predecessor / successors would be of type 'road' rather than
    //         // 'junction'
    //         //device << "                            <predecessor id=\"-1\"/>\n";
    //         //device << "                            <successor id=\"-1\"/>\n";
    //         //device << "                        </link>\n";
    //         device << "                        <width sOffset=\"0\" a=\"" << e->getLaneWidth(j) << "\" b=\"0\" c=\"0\" d=\"0\"/>\n";
    //         std::string markType = "broken";
    //         if (LHRL) {
    //             if (j == numLanes - 1) {
    //                 // solid road mark in the middle of the road
    //                 markType = "solid";
    //             } else if ((e->getPermissions(j) & ~(SVC_PEDESTRIAN | SVC_BICYCLE)) == 0) {
    //                 // solid road mark to the right of sidewalk or bicycle lane
    //                 markType = "solid";
    //             }
    //         } else {
    //             if (j == 0) {
    //                 markType = "solid";
    //             } else if (j > 0
    //                        && (e->getPermissions(j - 1) & ~(SVC_PEDESTRIAN | SVC_BICYCLE)) == 0) {
    //                 // solid road mark to the left of sidewalk or bicycle lane
    //                 markType = "solid";
    //             } else if (e->getPermissions(j) == 0) {
    //                 // solid road mark to the right of a forbidden lane
    //                 markType = "solid";
    //             }
    //         }
    //         device << "                        <roadMark sOffset=\"0\" type=\"" << markType << "\" weight=\"standard\" color=\"standard\" width=\"0.13\"/>\n";
    //         device << "                        <speed sOffset=\"0\" max=\"" << lanes[j].speed << "\"/>\n";
    //         device << "                    </lane>\n";
    //     }
    //     device << "                 </" << side << ">\n";
    //     if (LHLL) {
    //         writeEmptyCenterLane(device, centerMark, 0.13);
    //     }
    //     device << "            </laneSection>\n";
    //     device << "        </lanes>\n";
    //     writeRoadObjects(device, e, shc, crossings);
    //     writeSignals(device, e, length, signalLanes, shc);
    //     if (origNames) {
    //         device << "        <userData code=\"sumoId\" value=\"" << e->getID() << "\"/>\n";
    //     }
    //     device.closeTag();
    //     checkLaneGeometries(e);
    // }
    #[allow(unused_variables)]
    pub fn write_normal_edge(device: &mut OutputDevice, e: Ptr<NBEdge>, edgeID: i32, fromNodeID: i32, toNodeID: i32, origNames: bool, straightThresh: f64, shc: &ShapeContainer, signalLanes: &mut NWWriter_OpenDrive_SignalLanes, crossings: &Vec<String>) {
        // buffer output because some fields are computed out of order
        let mut elevationOSS = OutputDevice::new_string(3);
        elevationOSS.set_precision(8);
        let mut planViewOSS = OutputDevice::new_string(2);
        planViewOSS.set_precision(8);
        let mut length: f64 = 0.0;

        planViewOSS.open_tag("planView");
        // for the shape we need to use the leftmost border of the leftmost lane
        let lanes = e.get_lanes();
        let ls = Self::get_inner_lane_border(e, -1, 0.0);

        if ls.len() == 2 || e.get_permissions(-1) == SVC_PEDESTRIAN {
            // foot paths may contain sharp angles
            length = Self::write_geom_lines(&ls, &mut planViewOSS, &mut elevationOSS, 0.0);
        } else {
            let ok = Self::write_geom_smooth(&ls, e.get_speed(), &mut planViewOSS, &mut elevationOSS, straightThresh, &mut length);
            if !ok {
                write_warning(&format!("Could not compute smooth shape for edge '{}'.", e.get_id()));
            }
        }
        planViewOSS.close_tag("");

        device.open_tag("road");
        device.write_attr("name", &escape_xml(e.get_street_name(), false));
        device.set_precision(8); // length requires higher precision
        device.write_attr("length", &max2_local(POSITION_EPS, length));
        device.set_precision(g_precision());
        device.write_attr("id", &edgeID);
        device.write_attr("junction", &-1i32);
        if fromNodeID != INVALID_ID || toNodeID != INVALID_ID {
            device.open_tag("link");
            if fromNodeID != INVALID_ID {
                device.open_tag("predecessor");
                device.write_attr("elementType", "junction");
                device.write_attr("elementId", &fromNodeID);
                device.close_tag("");
            }
            if toNodeID != INVALID_ID {
                device.open_tag("successor");
                device.write_attr("elementType", "junction");
                device.write_attr("elementId", &toNodeID);
                device.close_tag("");
            }
            device.close_tag("");
        }
        device.open_tag("type").write_attr("s", &0i32).write_attr("type", "town").close_tag("");
        device.write(&planViewOSS.get_string());
        Self::write_elevation_profile(&ls, device, &elevationOSS);
        device.write("        <lateralProfile/>\n");
        device.write("        <lanes>\n");
        device.write("            <laneSection s=\"0\">\n");
        let centerMark: String = if e.get_permissions(e.get_num_lanes() - 1) == 0 { "none".to_string() } else { Self::get_divider_type(e) };
        if !lhll() {
            Self::write_empty_center_lane(device, &centerMark, 0.13);
        }
        let side = if lhll() { "left" } else { "right" };
        device.write(&format!("                <{}>\n", side));
        let numLanes = e.get_num_lanes();
        for jRH in (0..numLanes).rev() {
            // XODR always has the lanes left to right (by default this is
            // inner-to-outer but in LH networks its outer-to-inner)
            let j = if lefthand() { numLanes - 1 - jRH } else { jRH };
            let mut laneType: String = e.get_lane_struct_2(j).type_.clone();
            if laneType == "" {
                laneType = Self::get_lane_type(e.get_permissions(j));
            }
            device.write("                    <lane id=\"").write(&Self::s2x(j, numLanes)).write("\" type=\"").write(&laneType).write("\" level=\"true\">\n");
            device.write("                        <link/>\n");
            // this could be used for geometry-link junctions without u-turn,
            // predecessor and successors would be lane indices,
            // road predecessor / successors would be of type 'road' rather than
            // 'junction'
            //device << "                            <predecessor id=\"-1\"/>\n";
            //device << "                            <successor id=\"-1\"/>\n";
            //device << "                        </link>\n";
            device.write("                        <width sOffset=\"0\" a=\"").write(&e.get_lane_width_2(j)).write("\" b=\"0\" c=\"0\" d=\"0\"/>\n");
            let mut markType: String = "broken".to_string();
            if lhrl() {
                if j == numLanes - 1 {
                    // solid road mark in the middle of the road
                    markType = "solid".to_string();
                } else if (e.get_permissions(j) & !(SVC_PEDESTRIAN | SVC_BICYCLE)) == 0 {
                    // solid road mark to the right of sidewalk or bicycle lane
                    markType = "solid".to_string();
                }
            } else {
                if j == 0 {
                    markType = "solid".to_string();
                } else if j > 0 && (e.get_permissions(j - 1) & !(SVC_PEDESTRIAN | SVC_BICYCLE)) == 0 {
                    // solid road mark to the left of sidewalk or bicycle lane
                    markType = "solid".to_string();
                } else if e.get_permissions(j) == 0 {
                    // solid road mark to the right of a forbidden lane
                    markType = "solid".to_string();
                }
            }
            device.write("                        <roadMark sOffset=\"0\" type=\"").write(&markType).write("\" weight=\"standard\" color=\"standard\" width=\"0.13\"/>\n");
            device.write("                        <speed sOffset=\"0\" max=\"").write(&lanes[j as usize].speed).write("\"/>\n");
            device.write("                    </lane>\n");
        }
        device.write(&format!("                 </{}>\n", side));
        if lhll() {
            Self::write_empty_center_lane(device, &centerMark, 0.13);
        }
        device.write("            </laneSection>\n");
        device.write("        </lanes>\n");
        Self::write_road_objects(device, e, shc, crossings);
        Self::write_signals(device, e, length, signalLanes, shc);
        if origNames {
            device.write("        <userData code=\"sumoId\" value=\"").write(e.get_id()).write("\"/>\n");
        }
        device.close_tag("");
        Self::check_lane_geometries(e);
    }

    /// C++: `int NWWriter_OpenDrive::writeInternalEdge(OutputDevice &, OutputDevice &, const NBEdge *, int, int, int, int, int, const std::vector<NBEdge::Connection> &, const bool, const double, const std::string &, SignalLanes &)` [static, protected]
    /// write internal edge to device, return next connectionID
    // int
    // NWWriter_OpenDrive::writeInternalEdge(OutputDevice& device, OutputDevice& junctionDevice, const NBEdge* inEdge, int nodeID,
    //                                       int edgeID, int inEdgeID, int outEdgeID,
    //                                       int connectionID,
    //                                       const std::vector<NBEdge::Connection>& parallel,
    //                                       const bool isOuterEdge,
    //                                       const double straightThresh,
    //                                       const std::string& centerMark,
    //                                       SignalLanes& signalLanes) {
    //     assert(parallel.size() != 0);
    //     const NBEdge::Connection& cLeft = LHRL ? parallel.front() : parallel.back();
    //     const NBEdge* outEdge = cLeft.toEdge;
    //     PositionVector begShape = getInnerLaneBorder(inEdge, cLeft.fromLane);
    //     PositionVector endShape = getInnerLaneBorder(outEdge, cLeft.toLane);
    //     //std::cout << "computing reference line for internal lane " << cLeft.getInternalLaneID() << " begLane=" << inEdge->getLaneShape(cLeft.fromLane) << " endLane=" << outEdge->getLaneShape(cLeft.toLane) << "\n";
    //
    //     double length;
    //     double laneOffset = 0;
    //     PositionVector fallBackShape;
    //     fallBackShape.push_back(begShape.back());
    //     fallBackShape.push_back(endShape.front());
    //     const bool turnaround = inEdge->isTurningDirectionAt(outEdge);
    //     bool ok = true;
    //     PositionVector init = NBNode::bezierControlPoints(begShape, endShape, turnaround, 25, 25, ok, nullptr, straightThresh);
    //     if (init.size() == 0) {
    //         length = fallBackShape.length2D();
    //         // problem with turnarounds is known, method currently returns 'ok' (#2539)
    //         if (!ok) {
    //             WRITE_WARNINGF(TL("Could not compute smooth shape from lane '%' to lane '%'. Use option 'junctions.scurve-stretch' or increase radius of junction '%' to fix this."), inEdge->getLaneID(cLeft.fromLane), outEdge->getLaneID(cLeft.toLane), inEdge->getToNode()->getID());
    //         } else if (length <= NUMERICAL_EPS) {
    //             // left-curving geometry-like edges must use the right
    //             // side as reference line and shift
    //             begShape = getOuterLaneBorder(inEdge, cLeft.fromLane);
    //             endShape = getOuterLaneBorder(outEdge, cLeft.toLane);
    //             init = NBNode::bezierControlPoints(begShape, endShape, turnaround, 25, 25, ok, nullptr, straightThresh);
    //             if (init.size() != 0) {
    //                 length = init.bezier(12).length2D();
    //                 laneOffset = outEdge->getLaneWidth(cLeft.toLane);
    //                 //std::cout << " internalLane=" << cLeft.getInternalLaneID() << " length=" << length << "\n";
    //             }
    //         }
    //     } else {
    //         length = init.bezier(12).length2D();
    //     }
    //     double roadLength = MAX2(POSITION_EPS, length);
    //
    //     junctionDevice << "        <connection id=\"" << connectionID << "\" incomingRoad=\"" << inEdgeID << "\" connectingRoad=\"" << edgeID << "\" contactPoint=\"start\">\n";
    //     device.openTag("road");
    //     device.writeAttr("name", cLeft.id);
    //     device.setPrecision(8); // length requires higher precision
    //     device.writeAttr("length", roadLength);
    //     device.setPrecision(gPrecision);
    //     device.writeAttr("id", edgeID);
    //     device.writeAttr("junction", nodeID);
    //     device.openTag("link");
    //     device.openTag("predecessor");
    //     device.writeAttr("elementType", "road");
    //     device.writeAttr("elementId", inEdgeID);
    //     device.writeAttr("contactPoint", "end");
    //     device.closeTag();
    //     device.openTag("successor");
    //     device.writeAttr("elementType", "road");
    //     device.writeAttr("elementId", outEdgeID);
    //     device.writeAttr("contactPoint", "start");
    //     device.closeTag();
    //     device.closeTag();
    //     device.openTag("type").writeAttr("s", 0).writeAttr("type", "town").closeTag();
    //     device.openTag("planView");
    //     device.setPrecision(8); // geometry hdg requires higher precision
    //     OutputDevice_String elevationOSS(3);
    //     elevationOSS.setPrecision(8);
    // #ifdef DEBUG_SMOOTH_GEOM
    //     if (DEBUGCOND) {
    //         std::cout << "write planview for internal edge " << cLeft.id << " init=" << init << " fallback=" << fallBackShape
    //                   << " begShape=" << begShape << " endShape=" << endShape
    //                   << "\n";
    //     }
    // #endif
    //     if (init.size() == 0) {
    //         writeGeomLines(fallBackShape, device, elevationOSS);
    //     } else {
    //         writeGeomPP3(device, elevationOSS, init, length);
    //     }
    //     device.setPrecision(gPrecision);
    //     device.closeTag();
    //     writeElevationProfile(fallBackShape, device, elevationOSS);
    //     device << "        <lateralProfile/>\n";
    //     device << "        <lanes>\n";
    //     if (laneOffset != 0) {
    //         device << "            <laneOffset s=\"0\" a=\"" << laneOffset << "\" b=\"0\" c=\"0\" d=\"0\"/>\n";
    //     }
    //     device << "            <laneSection s=\"0\">\n";
    //     if (!lefthand) {
    //         writeEmptyCenterLane(device, centerMark, 0);
    //     }
    //     const std::string side = lefthand ? "left" : "right";
    //     device << "                <" << side << ">\n";
    //     const int numLanes = (int)parallel.size();
    //     for (int jRH = numLanes; --jRH >= 0;) {
    //         const int j = lefthand ? numLanes - 1 - jRH : jRH;
    //         const int xJ = s2x(j, numLanes);
    //         const NBEdge::Connection& c = parallel[j];
    //         const int fromIndex = s2x(c.fromLane, inEdge->getNumLanes());
    //         const int toIndex = s2x(c.toLane, outEdge->getNumLanes());
    //
    //         double inEdgeWidth = inEdge->getLaneWidth(c.fromLane);
    //         double outEdgeWidth = outEdge->getLaneWidth(c.toLane);
    //         // Ideally a polynomial function of third order would be needed for more precision.
    //         // This is obtained by specifying c and d coefficients, keeping it simple and linear
    //         // as we only know final and initial width.
    //         double bCoefficient = (outEdgeWidth - inEdgeWidth) / roadLength;
    //         double cCoefficient = 0;
    //         double dCoefficient = 0;
    //         device << "                    <lane id=\"" << xJ << "\" type=\"" << getLaneType(outEdge->getPermissions(c.toLane)) << "\" level=\"true\">\n";
    //         device << "                        <link>\n";
    //         device << "                            <predecessor id=\"" << fromIndex << "\"/>\n";
    //         device << "                            <successor id=\"" << toIndex << "\"/>\n";
    //         device << "                        </link>\n";
    //         device << "                        <width sOffset=\"0\" a=\"" << inEdgeWidth << "\" b=\"" << bCoefficient << "\" c=\"" << cCoefficient << "\" d=\"" << dCoefficient << "\"/>\n";
    //         std::string markType = "broken";
    //         if (inEdge->isTurningDirectionAt(outEdge)) {
    //             markType = "none";
    //         } else if (c.fromLane == 0 && c.toLane == 0 && isOuterEdge) {
    //             // solid road mark at the outer border
    //             markType = "solid";
    //         } else if (isOuterEdge && j > 0
    //                    && (outEdge->getPermissions(parallel[j - 1].toLane) & ~(SVC_PEDESTRIAN | SVC_BICYCLE)) == 0) {
    //             // solid road mark to the left of sidewalk or bicycle lane
    //             markType = "solid";
    //         } else if (!inEdge->getToNode()->geometryLike()) {
    //             // draw shorter road marks to indicate turning paths
    //             LinkDirection dir = inEdge->getToNode()->getDirection(inEdge, outEdge, lefthand);
    //             if (dir == LinkDirection::LEFT || dir == LinkDirection::RIGHT || dir == LinkDirection::PARTLEFT || dir == LinkDirection::PARTRIGHT) {
    //                 // XXX <type><line/><type> is not rendered by odrViewer so cannot be validated
    //                 // device << "                             <type name=\"broken\" width=\"0.13\">\n";
    //                 // device << "                                  <line length=\"0.5\" space=\"0.5\" tOffset=\"0\" sOffset=\"0\" rule=\"none\"/>\n";
    //                 // device << "                             </type>\n";
    //                 markType = "none";
    //             }
    //         }
    //         device << "                        <roadMark sOffset=\"0\" type=\"" << markType << "\" weight=\"standard\" color=\"standard\" width=\"0.13\"/>\n";
    //         device << "                        <speed sOffset=\"0\" max=\"" << c.vmax << "\"/>\n";
    //         device << "                    </lane>\n";
    //
    //         junctionDevice << "            <laneLink from=\"" << fromIndex << "\" to=\"" << xJ << "\"/>\n";
    //         connectionID++;
    //     }
    //     device << "                 </" << side << ">\n";
    //     if (lefthand) {
    //         writeEmptyCenterLane(device, centerMark, 0);
    //     }
    //     device << "            </laneSection>\n";
    //     device << "        </lanes>\n";
    //     device << "        <objects/>\n";
    //     UNUSED_PARAMETER(signalLanes);
    //     device << "        <signals/>\n";
    //     device.closeTag();
    //     junctionDevice << "        </connection>\n";
    //
    //     return connectionID;
    // }
    #[allow(unused_variables)]
    pub fn write_internal_edge(device: &mut OutputDevice, junctionDevice: &mut OutputDevice, inEdge: Ptr<NBEdge>, nodeID: i32, edgeID: i32, inEdgeID: i32, outEdgeID: i32, connectionID: i32, parallel: &Vec<NBEdge_Connection>, isOuterEdge: bool, straightThresh: f64, centerMark: &str, signalLanes: &mut NWWriter_OpenDrive_SignalLanes) -> i32 {
        let mut connectionID = connectionID;
        assert!(!parallel.is_empty());
        let cLeft: &NBEdge_Connection = if lhrl() { parallel.first().unwrap() } else { parallel.last().unwrap() };
        let outEdge: Ptr<NBEdge> = cLeft.toEdge;
        let mut begShape = Self::get_inner_lane_border(inEdge, cLeft.fromLane, 0.0);
        let mut endShape = Self::get_inner_lane_border(outEdge, cLeft.toLane, 0.0);
        //std::cout << "computing reference line for internal lane " << cLeft.getInternalLaneID() << " begLane=" << inEdge->getLaneShape(cLeft.fromLane) << " endLane=" << outEdge->getLaneShape(cLeft.toLane) << "\n";

        let mut length: f64 = 0.0;
        let mut laneOffset: f64 = 0.0;
        let mut fallBackShape = PositionVector::new();
        fallBackShape.push(*begShape.last().unwrap());
        fallBackShape.push(*endShape.first().unwrap());
        let turnaround = inEdge.is_turning_direction_at(outEdge);
        let mut ok = true;
        let mut init = NBNode::bezier_control_points(&begShape, &endShape, turnaround, 25.0, 25.0, &mut ok, Ptr::null(), straightThresh, 0);
        if init.len() == 0 {
            length = fallBackShape.length2d();
            // problem with turnarounds is known, method currently returns 'ok' (#2539)
            if !ok {
                write_warning(&format!("Could not compute smooth shape from lane '{}' to lane '{}'. Use option 'junctions.scurve-stretch' or increase radius of junction '{}' to fix this.", inEdge.get_lane_id(cLeft.fromLane), outEdge.get_lane_id(cLeft.toLane), inEdge.get_to_node().get_id()));
            } else if length <= NUMERICAL_EPS {
                // left-curving geometry-like edges must use the right
                // side as reference line and shift
                begShape = Self::get_outer_lane_border(inEdge, cLeft.fromLane);
                endShape = Self::get_outer_lane_border(outEdge, cLeft.toLane);
                init = NBNode::bezier_control_points(&begShape, &endShape, turnaround, 25.0, 25.0, &mut ok, Ptr::null(), straightThresh, 0);
                if init.len() != 0 {
                    length = init.bezier(12).length2d();
                    laneOffset = outEdge.get_lane_width_2(cLeft.toLane);
                    //std::cout << " internalLane=" << cLeft.getInternalLaneID() << " length=" << length << "\n";
                }
            }
        } else {
            length = init.bezier(12).length2d();
        }
        let roadLength = max2_local(POSITION_EPS, length);

        junctionDevice.write(&format!("        <connection id=\"{}\" incomingRoad=\"{}\" connectingRoad=\"{}\" contactPoint=\"start\">\n", connectionID, inEdgeID, edgeID));
        device.open_tag("road");
        device.write_attr("name", cLeft.id.as_str());
        device.set_precision(8); // length requires higher precision
        device.write_attr("length", &roadLength);
        device.set_precision(g_precision());
        device.write_attr("id", &edgeID);
        device.write_attr("junction", &nodeID);
        device.open_tag("link");
        device.open_tag("predecessor");
        device.write_attr("elementType", "road");
        device.write_attr("elementId", &inEdgeID);
        device.write_attr("contactPoint", "end");
        device.close_tag("");
        device.open_tag("successor");
        device.write_attr("elementType", "road");
        device.write_attr("elementId", &outEdgeID);
        device.write_attr("contactPoint", "start");
        device.close_tag("");
        device.close_tag("");
        device.open_tag("type").write_attr("s", &0i32).write_attr("type", "town").close_tag("");
        device.open_tag("planView");
        device.set_precision(8); // geometry hdg requires higher precision
        let mut elevationOSS = OutputDevice::new_string(3);
        elevationOSS.set_precision(8);
        if init.len() == 0 {
            Self::write_geom_lines(&fallBackShape, device, &mut elevationOSS, 0.0);
        } else {
            Self::write_geom_pp3(device, &mut elevationOSS, init, length, 0.0);
        }
        device.set_precision(g_precision());
        device.close_tag("");
        Self::write_elevation_profile(&fallBackShape, device, &elevationOSS);
        device.write("        <lateralProfile/>\n");
        device.write("        <lanes>\n");
        if laneOffset != 0.0 {
            device.write("            <laneOffset s=\"0\" a=\"").write(&laneOffset).write("\" b=\"0\" c=\"0\" d=\"0\"/>\n");
        }
        device.write("            <laneSection s=\"0\">\n");
        if !lefthand() {
            Self::write_empty_center_lane(device, centerMark, 0.0);
        }
        let side = if lefthand() { "left" } else { "right" };
        device.write(&format!("                <{}>\n", side));
        let numLanes = parallel.len() as i32;
        for jRH in (0..numLanes).rev() {
            let j = if lefthand() { numLanes - 1 - jRH } else { jRH };
            let xJ = Self::s2x(j, numLanes);
            let c: &NBEdge_Connection = &parallel[j as usize];
            let fromIndex = Self::s2x(c.fromLane, inEdge.get_num_lanes());
            let toIndex = Self::s2x(c.toLane, outEdge.get_num_lanes());

            let inEdgeWidth = inEdge.get_lane_width_2(c.fromLane);
            let outEdgeWidth = outEdge.get_lane_width_2(c.toLane);
            // Ideally a polynomial function of third order would be needed for more precision.
            // This is obtained by specifying c and d coefficients, keeping it simple and linear
            // as we only know final and initial width.
            let bCoefficient = (outEdgeWidth - inEdgeWidth) / roadLength;
            let cCoefficient: f64 = 0.0;
            let dCoefficient: f64 = 0.0;
            device.write("                    <lane id=\"").write(&xJ).write("\" type=\"").write(&Self::get_lane_type(outEdge.get_permissions(c.toLane))).write("\" level=\"true\">\n");
            device.write("                        <link>\n");
            device.write("                            <predecessor id=\"").write(&fromIndex).write("\"/>\n");
            device.write("                            <successor id=\"").write(&toIndex).write("\"/>\n");
            device.write("                        </link>\n");
            device.write("                        <width sOffset=\"0\" a=\"").write(&inEdgeWidth).write("\" b=\"").write(&bCoefficient).write("\" c=\"").write(&cCoefficient).write("\" d=\"").write(&dCoefficient).write("\"/>\n");
            let mut markType: String = "broken".to_string();
            if inEdge.is_turning_direction_at(outEdge) {
                markType = "none".to_string();
            } else if c.fromLane == 0 && c.toLane == 0 && isOuterEdge {
                // solid road mark at the outer border
                markType = "solid".to_string();
            } else if isOuterEdge && j > 0 && (outEdge.get_permissions(parallel[(j - 1) as usize].toLane) & !(SVC_PEDESTRIAN | SVC_BICYCLE)) == 0 {
                // solid road mark to the left of sidewalk or bicycle lane
                markType = "solid".to_string();
            } else if !inEdge.get_to_node().geometry_like() {
                // draw shorter road marks to indicate turning paths
                let dir = inEdge.get_to_node().get_direction(inEdge, outEdge, lefthand());
                if dir == LinkDirection::LEFT || dir == LinkDirection::RIGHT || dir == LinkDirection::PARTLEFT || dir == LinkDirection::PARTRIGHT {
                    // XXX <type><line/><type> is not rendered by odrViewer so cannot be validated
                    // device << "                             <type name=\"broken\" width=\"0.13\">\n";
                    // device << "                                  <line length=\"0.5\" space=\"0.5\" tOffset=\"0\" sOffset=\"0\" rule=\"none\"/>\n";
                    // device << "                             </type>\n";
                    markType = "none".to_string();
                }
            }
            device.write("                        <roadMark sOffset=\"0\" type=\"").write(&markType).write("\" weight=\"standard\" color=\"standard\" width=\"0.13\"/>\n");
            device.write("                        <speed sOffset=\"0\" max=\"").write(&c.vmax).write("\"/>\n");
            device.write("                    </lane>\n");

            junctionDevice.write("            <laneLink from=\"").write(&fromIndex).write("\" to=\"").write(&xJ).write("\"/>\n");
            connectionID += 1;
        }
        device.write(&format!("                 </{}>\n", side));
        if lefthand() {
            Self::write_empty_center_lane(device, centerMark, 0.0);
        }
        device.write("            </laneSection>\n");
        device.write("        </lanes>\n");
        device.write("        <objects/>\n");
        // UNUSED_PARAMETER(signalLanes);
        device.write("        <signals/>\n");
        device.close_tag("");
        junctionDevice.write("        </connection>\n");

        connectionID
    }

    /// C++: `void NWWriter_OpenDrive::addPedestrianConnection(const NBEdge *, const NBEdge *, std::vector<NBEdge::Connection> &)` [static, protected]
    // void
    // NWWriter_OpenDrive::addPedestrianConnection(const NBEdge* inEdge, const NBEdge* outEdge, std::vector<NBEdge::Connection>& parallel) {
    //     // by default there are no internal lanes for pedestrians. Determine if
    //     // one is feasible and does not exist yet.
    //     if (outEdge != nullptr
    //             && inEdge->getPermissions(0) == SVC_PEDESTRIAN
    //             && outEdge->getPermissions(0) == SVC_PEDESTRIAN
    //             && (parallel.empty()
    //                 || parallel.front().fromLane != 0
    //                 || parallel.front().toLane != 0)) {
    //         parallel.insert(parallel.begin(), NBEdge::Connection(0, const_cast<NBEdge*>(outEdge), 0));
    //         parallel.front().vmax = (inEdge->getLanes()[0].speed + outEdge->getLanes()[0].speed) / (double) 2.0;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_pedestrian_connection(inEdge: Ptr<NBEdge>, outEdge: Ptr<NBEdge>, parallel: &mut Vec<NBEdge_Connection>) {
        // by default there are no internal lanes for pedestrians. Determine if
        // one is feasible and does not exist yet.
        if !outEdge.is_null()
            && inEdge.get_permissions(0) == SVC_PEDESTRIAN
            && outEdge.get_permissions(0) == SVC_PEDESTRIAN
            && (parallel.is_empty() || parallel.first().unwrap().fromLane != 0 || parallel.first().unwrap().toLane != 0)
        {
            parallel.insert(0, NBEdge_Connection::new(0, outEdge, 0, false));
            parallel.first_mut().unwrap().vmax = (inEdge.get_lanes()[0].speed + outEdge.get_lanes()[0].speed) / 2.0;
        }
    }

    /// C++: `double NWWriter_OpenDrive::writeGeomLines(const PositionVector &, OutputDevice &, OutputDevice &, double)` [static, protected]
    /// write geometry as sequence of lines (sumo style)
    // double
    // NWWriter_OpenDrive::writeGeomLines(const PositionVector& shape, OutputDevice& device, OutputDevice& elevationDevice, double offset) {
    //     for (int j = 0; j < (int)shape.size() - 1; ++j) {
    //         const Position& p = shape[j];
    //         const Position& p2 = shape[j + 1];
    //         const double hdg = shape.angleAt2D(j);
    //         const double length = p.distanceTo2D(p2);
    //         device.openTag("geometry");
    //         device.writeAttr("s", offset);
    //         device.writeAttr("x", p.x());
    //         device.writeAttr("y", p.y());
    //         device.writeAttr("hdg", hdg);
    //         device.writeAttr("length", length < 1e-8 ? 1e-8 : length);
    //         device.openTag("line").closeTag();
    //         device.closeTag();
    //         elevationDevice << "            <elevation s=\"" << offset << "\" a=\"" << p.z() << "\" b=\"" << (p2.z() - p.z()) / MAX2(POSITION_EPS, length) << "\" c=\"0\" d=\"0\"/>\n";
    //         offset += length;
    //     }
    //     return offset;
    // }
    #[allow(unused_variables)]
    pub fn write_geom_lines(shape: &PositionVector, device: &mut OutputDevice, elevationDevice: &mut OutputDevice, offset: f64) -> f64 {
        let mut offset = offset;
        for j in 0..(shape.len() as i32 - 1) {
            let p: Position = shape[j as usize];
            let p2: Position = shape[j as usize + 1];
            let hdg = shape.angle_at2d(j);
            let length = p.distance_to2d(&p2);
            device.open_tag("geometry");
            device.write_attr("s", &offset);
            device.write_attr("x", &p.x());
            device.write_attr("y", &p.y());
            device.write_attr("hdg", &hdg);
            device.write_attr("length", &(if length < 1e-8 { 1e-8 } else { length }));
            device.open_tag("line").close_tag("");
            device.close_tag("");
            elevationDevice
                .write("            <elevation s=\"")
                .write(&offset)
                .write("\" a=\"")
                .write(&p.z())
                .write("\" b=\"")
                .write(&((p2.z() - p.z()) / max2_local(POSITION_EPS, length)))
                .write("\" c=\"0\" d=\"0\"/>\n");
            offset += length;
        }
        offset
    }

    /// C++: `bool NWWriter_OpenDrive::writeGeomSmooth(const PositionVector &, double, OutputDevice &, OutputDevice &, double, double &)` [static, protected]
    // bool
    // NWWriter_OpenDrive::writeGeomSmooth(const PositionVector& shape, double speed, OutputDevice& device, OutputDevice& elevationDevice, double straightThresh, double& length) {
    // #ifdef DEBUG_SMOOTH_GEOM
    //     if (DEBUGCOND) {
    //         std::cout << "writeGeomSmooth\n  n=" << shape.size() << " shape=" << toString(shape) << "\n";
    //     }
    // #endif
    //     bool ok = true;
    //     const double longThresh = speed; //  16.0; // make user-configurable (should match the sampling rate of the source data)
    //     const double curveCutout = longThresh / 2; // 8.0; // make user-configurable (related to the maximum turning rate)
    //     // the length of the segment that is added for cutting a corner can be bounded by 2*curveCutout (prevent the segment to be classified as 'long')
    //     assert(longThresh >= 2 * curveCutout);
    //     assert(shape.size() > 2);
    //     // add intermediate points wherever there is a strong angular change between long segments
    //     // assume the geometry is simplified so as not to contain consecutive colinear points
    //     PositionVector shape2 = shape;
    //     double maxAngleDiff = 0;
    //     double offset = 0;
    //     for (int j = 1; j < (int)shape.size() - 1; ++j) {
    //         //const double hdg = shape.angleAt2D(j);
    //         const Position& p0 = shape[j - 1];
    //         const Position& p1 = shape[j];
    //         const Position& p2 = shape[j + 1];
    //         const double dAngle = fabs(GeomHelper::angleDiff(p0.angleTo2D(p1), p1.angleTo2D(p2)));
    //         const double length1 = p0.distanceTo2D(p1);
    //         const double length2 = p1.distanceTo2D(p2);
    //         maxAngleDiff = MAX2(maxAngleDiff, dAngle);
    // #ifdef DEBUG_SMOOTH_GEOM
    //         if (DEBUGCOND) {
    //             std::cout << "   j=" << j << " dAngle=" << RAD2DEG(dAngle) << " length1=" << length1 << " length2=" << length2 << "\n";
    //         }
    // #endif
    //         if (dAngle > straightThresh
    //                 && (length1 > longThresh || j == 1)
    //                 && (length2 > longThresh || j == (int)shape.size() - 2)) {
    //             // NBNode::bezierControlPoints checks for minimum length of POSITION_EPS so we make sure there is no instability
    //             shape2.insertAtClosest(shape.positionAtOffset2D(offset + length1 - MIN2(length1 - 2 * POSITION_EPS, curveCutout)), false);
    //             shape2.insertAtClosest(shape.positionAtOffset2D(offset + length1 + MIN2(length2 - 2 * POSITION_EPS, curveCutout)), false);
    //             shape2.removeClosest(p1);
    //         }
    //         offset += length1;
    //     }
    //     const int numPoints = (int)shape2.size();
    // #ifdef DEBUG_SMOOTH_GEOM
    //     if (DEBUGCOND) {
    //         std::cout << " n=" << numPoints << " shape2=" << toString(shape2) << "\n";
    //     }
    // #endif
    //
    //     if (maxAngleDiff < straightThresh) {
    //         length = writeGeomLines(shape2, device, elevationDevice, 0);
    // #ifdef DEBUG_SMOOTH_GEOM
    //         if (DEBUGCOND) {
    //             std::cout << "   special case: all lines. maxAngleDiff=" << maxAngleDiff << "\n";
    //         }
    // #endif
    //         return ok;
    //     }
    //
    //     // write the long segments as lines, short segments as curves
    //     offset = 0;
    //     for (int j = 0; j < numPoints - 1; ++j) {
    //         const Position& p0 = shape2[j];
    //         const Position& p1 = shape2[j + 1];
    //         PositionVector line;
    //         line.push_back(p0);
    //         line.push_back(p1);
    //         const double lineLength = line.length2D();
    //         if (lineLength >= longThresh) {
    //             offset = writeGeomLines(line, device, elevationDevice, offset);
    // #ifdef DEBUG_SMOOTH_GEOM
    //             if (DEBUGCOND) {
    //                 std::cout << "      writeLine=" << toString(line) << "\n";
    //             }
    // #endif
    //         } else {
    //             // find control points
    //             PositionVector begShape;
    //             PositionVector endShape;
    //             if (j == 0 || j == numPoints - 2) {
    //                 // keep the angle of the first/last segment but end at the front of the shape
    //                 begShape = line;
    //                 begShape.add(p0 - begShape.back());
    //             } else if (j == 1 || p0.distanceTo2D(shape2[j - 1]) > longThresh) {
    //                 // use the previous segment if it is long or the first one
    //                 begShape.push_back(shape2[j - 1]);
    //                 begShape.push_back(p0);
    //             } else {
    //                 // end at p0 with mean angle of the previous and current segment
    //                 begShape.push_back(shape2[j - 1]);
    //                 begShape.push_back(p1);
    //                 begShape.add(p0 - begShape.back());
    //             }
    //
    //             if (j == 0 || j == numPoints - 2) {
    //                 // keep the angle of the first/last segment but start at the end of the shape
    //                 endShape = line;
    //                 endShape.add(p1 - endShape.front());
    //             } else if (j == numPoints - 3 || p1.distanceTo2D(shape2[j + 2]) > longThresh) {
    //                 // use the next segment if it is long or the final one
    //                 endShape.push_back(p1);
    //                 endShape.push_back(shape2[j + 2]);
    //             } else {
    //                 // start at p1 with mean angle of the current and next segment
    //                 endShape.push_back(p0);
    //                 endShape.push_back(shape2[j + 2]);
    //                 endShape.add(p1 - endShape.front());
    //             }
    //             const double extrapolateLength = MIN2((double)25, lineLength / 4);
    //             PositionVector init = NBNode::bezierControlPoints(begShape, endShape, false, extrapolateLength, extrapolateLength, ok, nullptr, straightThresh);
    //             if (init.size() == 0) {
    //                 // could not compute control points, write line
    //                 offset = writeGeomLines(line, device, elevationDevice, offset);
    // #ifdef DEBUG_SMOOTH_GEOM
    //                 if (DEBUGCOND) {
    //                     std::cout << "      writeLine lineLength=" << lineLength << " begShape" << j << "=" << toString(begShape) << " endShape" << j << "=" << toString(endShape) << " init" << j << "=" << toString(init) << "\n";
    //                 }
    // #endif
    //             } else {
    //                 // write bezier
    //                 const double curveLength = init.bezier(12).length2D();
    //                 offset = writeGeomPP3(device, elevationDevice, init, curveLength, offset);
    // #ifdef DEBUG_SMOOTH_GEOM
    //                 if (DEBUGCOND) {
    //                     std::cout << "      writeCurve lineLength=" << lineLength << " curveLength=" << curveLength << " begShape" << j << "=" << toString(begShape) << " endShape" << j << "=" << toString(endShape) << " init" << j << "=" << toString(init) << "\n";
    //                 }
    // #endif
    //             }
    //         }
    //     }
    //     length = offset;
    //     return ok;
    // }
    #[allow(unused_variables)]
    pub fn write_geom_smooth(shape: &PositionVector, speed: f64, device: &mut OutputDevice, elevationDevice: &mut OutputDevice, straightThresh: f64, length: &mut f64) -> bool {
        let mut ok = true;
        let longThresh = speed; //  16.0; // make user-configurable (should match the sampling rate of the source data)
        let curveCutout = longThresh / 2.0; // 8.0; // make user-configurable (related to the maximum turning rate)
        // the length of the segment that is added for cutting a corner can be bounded by 2*curveCutout (prevent the segment to be classified as 'long')
        assert!(longThresh >= 2.0 * curveCutout);
        assert!(shape.len() > 2);
        // add intermediate points wherever there is a strong angular change between long segments
        // assume the geometry is simplified so as not to contain consecutive colinear points
        let mut shape2 = shape.clone();
        let mut maxAngleDiff: f64 = 0.0;
        let mut offset: f64 = 0.0;
        for j in 1..(shape.len() as i32 - 1) {
            let p0: Position = shape[(j - 1) as usize];
            let p1: Position = shape[j as usize];
            let p2: Position = shape[(j + 1) as usize];
            let dAngle = angle_diff(p0.angle_to2d(&p1), p1.angle_to2d(&p2)).abs();
            let length1 = p0.distance_to2d(&p1);
            let length2 = p1.distance_to2d(&p2);
            maxAngleDiff = max2_local(maxAngleDiff, dAngle);
            if dAngle > straightThresh && (length1 > longThresh || j == 1) && (length2 > longThresh || j == shape.len() as i32 - 2) {
                // NBNode::bezierControlPoints checks for minimum length of POSITION_EPS so we make sure there is no instability
                shape2.insert_at_closest(&shape.position_at_offset2d(offset + length1 - min2_local(length1 - 2.0 * POSITION_EPS, curveCutout), 0.0, false), false);
                shape2.insert_at_closest(&shape.position_at_offset2d(offset + length1 + min2_local(length2 - 2.0 * POSITION_EPS, curveCutout), 0.0, false), false);
                shape2.remove_closest(&p1);
            }
            offset += length1;
        }
        let numPoints = shape2.len() as i32;
        if maxAngleDiff < straightThresh {
            *length = Self::write_geom_lines(&shape2, device, elevationDevice, 0.0);
            return ok;
        }
        // write the long segments as lines, short segments as curves
        offset = 0.0;
        for j in 0..(numPoints - 1) {
            let p0: Position = shape2[j as usize];
            let p1: Position = shape2[(j + 1) as usize];
            let mut line = PositionVector::new();
            line.push(p0);
            line.push(p1);
            let lineLength = line.length2d();
            if lineLength >= longThresh {
                offset = Self::write_geom_lines(&line, device, elevationDevice, offset);
            } else {
                // find control points
                let mut begShape = PositionVector::new();
                let mut endShape = PositionVector::new();
                if j == 0 || j == numPoints - 2 {
                    // keep the angle of the first/last segment but end at the front of the shape
                    begShape = line.clone();
                    let d = p0 - *begShape.back();
                    begShape.add(&d);
                } else if j == 1 || p0.distance_to2d(&shape2[(j - 1) as usize]) > longThresh {
                    // use the previous segment if it is long or the first one
                    begShape.push(shape2[(j - 1) as usize]);
                    begShape.push(p0);
                } else {
                    // end at p0 with mean angle of the previous and current segment
                    begShape.push(shape2[(j - 1) as usize]);
                    begShape.push(p1);
                    let d = p0 - *begShape.back();
                    begShape.add(&d);
                }
                if j == 0 || j == numPoints - 2 {
                    // keep the angle of the first/last segment but start at the end of the shape
                    endShape = line.clone();
                    let d = p1 - *endShape.front();
                    endShape.add(&d);
                } else if j == numPoints - 3 || p1.distance_to2d(&shape2[(j + 2) as usize]) > longThresh {
                    // use the next segment if it is long or the final one
                    endShape.push(p1);
                    endShape.push(shape2[(j + 2) as usize]);
                } else {
                    // start at p1 with mean angle of the current and next segment
                    endShape.push(p0);
                    endShape.push(shape2[(j + 2) as usize]);
                    let d = p1 - *endShape.front();
                    endShape.add(&d);
                }
                let extrapolateLength = min2_local(25.0, lineLength / 4.0);
                let init = NBNode::bezier_control_points(&begShape, &endShape, false, extrapolateLength, extrapolateLength, &mut ok, Ptr::null(), straightThresh, 0);
                if init.len() == 0 {
                    // could not compute control points, write line
                    offset = Self::write_geom_lines(&line, device, elevationDevice, offset);
                } else {
                    // write bezier
                    let curveLength = init.bezier(12).length2d();
                    offset = Self::write_geom_pp3(device, elevationDevice, init, curveLength, offset);
                }
            }
        }
        *length = offset;
        ok
    }

    /// C++: `double NWWriter_OpenDrive::writeGeomPP3(OutputDevice &, OutputDevice &, PositionVector, double, double)` [static, protected]
    /// write geometry as a single bezier curve (paramPoly3)
    // double
    // NWWriter_OpenDrive::writeGeomPP3(
    //     OutputDevice& device,
    //     OutputDevice& elevationDevice,
    //     PositionVector init,
    //     double length,
    //     double offset) {
    //     assert(init.size() == 3 || init.size() == 4);
    //
    //     // avoid division by 0
    //     length = MAX2(POSITION_EPS, length);
    //
    //     const Position p = init.front();
    //     const double hdg = init.angleAt2D(0);
    //
    //     // backup elevation values
    //     const PositionVector initZ = init;
    //     // translate to u,v coordinates
    //     init.add(-p.x(), -p.y(), -p.z());
    //     init.rotate2D(-hdg);
    //
    //     // parametric coefficients
    //     double aU, bU, cU, dU;
    //     double aV, bV, cV, dV;
    //     double aZ, bZ, cZ, dZ;
    //
    //     // unfactor the Bernstein polynomials of degree 2 (or 3) and collect the coefficients
    //     if (init.size() == 3) {
    //         //f(x, a, b ,c) = a + (2*b - 2*a)*x + (a - 2*b + c)*x*x
    //         aU = init[0].x();
    //         bU = 2 * init[1].x() - 2 * init[0].x();
    //         cU = init[0].x() - 2 * init[1].x() + init[2].x();
    //         dU = 0;
    //
    //         aV = init[0].y();
    //         bV = 2 * init[1].y() - 2 * init[0].y();
    //         cV = init[0].y() - 2 * init[1].y() + init[2].y();
    //         dV = 0;
    //
    //         // elevation is not parameteric on [0:1] but on [0:length]
    //         aZ = initZ[0].z();
    //         bZ = (2 * initZ[1].z() - 2 * initZ[0].z()) / length;
    //         cZ = (initZ[0].z() - 2 * initZ[1].z() + initZ[2].z()) / (length * length);
    //         dZ = 0;
    //
    //     } else {
    //         // f(x, a, b, c, d) = a + (x*((3*b) - (3*a))) + ((x*x)*((3*a) + (3*c) - (6*b))) + ((x*x*x)*((3*b) - (3*c) - a + d))
    //         aU = init[0].x();
    //         bU = 3 * init[1].x() - 3 * init[0].x();
    //         cU = 3 * init[0].x() - 6 * init[1].x() + 3 * init[2].x();
    //         dU = -init[0].x() + 3 * init[1].x() - 3 * init[2].x() + init[3].x();
    //
    //         aV = init[0].y();
    //         bV = 3 * init[1].y() - 3 * init[0].y();
    //         cV = 3 * init[0].y() - 6 * init[1].y() + 3 * init[2].y();
    //         dV = -init[0].y() + 3 * init[1].y() - 3 * init[2].y() + init[3].y();
    //
    //         // elevation is not parameteric on [0:1] but on [0:length]
    //         aZ = initZ[0].z();
    //         bZ = (3 * initZ[1].z() - 3 * initZ[0].z()) / length;
    //         cZ = (3 * initZ[0].z() - 6 * initZ[1].z() + 3 * initZ[2].z()) / (length * length);
    //         dZ = (-initZ[0].z() + 3 * initZ[1].z() - 3 * initZ[2].z() + initZ[3].z()) / (length * length * length);
    //     }
    //
    //     device.openTag("geometry");
    //     device.writeAttr("s", offset);
    //     device.writeAttr("x", p.x());
    //     device.writeAttr("y", p.y());
    //     device.writeAttr("hdg", hdg);
    //     device.writeAttr("length", length);
    //
    //     device.openTag("paramPoly3");
    //     device.writeAttr("aU", aU);
    //     device.writeAttr("bU", bU);
    //     device.writeAttr("cU", cU);
    //     device.writeAttr("dU", dU);
    //     device.writeAttr("aV", aV);
    //     device.writeAttr("bV", bV);
    //     device.writeAttr("cV", cV);
    //     device.writeAttr("dV", dV);
    //     device.writeAttr("pRange", "normalized");
    //     device.closeTag();
    //     device.closeTag();
    //
    //     // write elevation
    //     elevationDevice.openTag("elevation");
    //     elevationDevice.writeAttr("s", offset);
    //     elevationDevice.writeAttr("a", aZ);
    //     elevationDevice.writeAttr("b", bZ);
    //     elevationDevice.writeAttr("c", cZ);
    //     elevationDevice.writeAttr("d", dZ);
    //     elevationDevice.closeTag();
    //
    //     return offset + length;
    // }
    #[allow(unused_variables)]
    pub fn write_geom_pp3(device: &mut OutputDevice, elevationDevice: &mut OutputDevice, init: PositionVector, length: f64, offset: f64) -> f64 {
        let mut init = init;
        assert!(init.len() == 3 || init.len() == 4);

        // avoid division by 0
        let length = if utils::geom::POSITION_EPS > length { utils::geom::POSITION_EPS } else { length };

        let p: Position = *init.front();
        let hdg = init.angle_at2d(0);

        // backup elevation values
        let initZ = init.clone();
        // translate to u,v coordinates
        init.add_xyz(-p.x(), -p.y(), -p.z());
        init.rotate2d(-hdg);

        // unfactor the Bernstein polynomials of degree 2 (or 3) and collect the coefficients
        let (aU, bU, cU, dU, aV, bV, cV, dV, aZ, bZ, cZ, dZ) = if init.len() == 3 {
            //f(x, a, b ,c) = a + (2*b - 2*a)*x + (a - 2*b + c)*x*x
            let aU = init[0].x();
            let bU = 2.0 * init[1].x() - 2.0 * init[0].x();
            let cU = init[0].x() - 2.0 * init[1].x() + init[2].x();
            let dU = 0.0;

            let aV = init[0].y();
            let bV = 2.0 * init[1].y() - 2.0 * init[0].y();
            let cV = init[0].y() - 2.0 * init[1].y() + init[2].y();
            let dV = 0.0;

            // elevation is not parameteric on [0:1] but on [0:length]
            let aZ = initZ[0].z();
            let bZ = (2.0 * initZ[1].z() - 2.0 * initZ[0].z()) / length;
            let cZ = (initZ[0].z() - 2.0 * initZ[1].z() + initZ[2].z()) / (length * length);
            let dZ = 0.0;
            (aU, bU, cU, dU, aV, bV, cV, dV, aZ, bZ, cZ, dZ)
        } else {
            // f(x, a, b, c, d) = a + (x*((3*b) - (3*a))) + ((x*x)*((3*a) + (3*c) - (6*b))) + ((x*x*x)*((3*b) - (3*c) - a + d))
            let aU = init[0].x();
            let bU = 3.0 * init[1].x() - 3.0 * init[0].x();
            let cU = 3.0 * init[0].x() - 6.0 * init[1].x() + 3.0 * init[2].x();
            let dU = -init[0].x() + 3.0 * init[1].x() - 3.0 * init[2].x() + init[3].x();

            let aV = init[0].y();
            let bV = 3.0 * init[1].y() - 3.0 * init[0].y();
            let cV = 3.0 * init[0].y() - 6.0 * init[1].y() + 3.0 * init[2].y();
            let dV = -init[0].y() + 3.0 * init[1].y() - 3.0 * init[2].y() + init[3].y();

            // elevation is not parameteric on [0:1] but on [0:length]
            let aZ = initZ[0].z();
            let bZ = (3.0 * initZ[1].z() - 3.0 * initZ[0].z()) / length;
            let cZ = (3.0 * initZ[0].z() - 6.0 * initZ[1].z() + 3.0 * initZ[2].z()) / (length * length);
            let dZ = (-initZ[0].z() + 3.0 * initZ[1].z() - 3.0 * initZ[2].z() + initZ[3].z()) / (length * length * length);
            (aU, bU, cU, dU, aV, bV, cV, dV, aZ, bZ, cZ, dZ)
        };

        device.open_tag("geometry");
        device.write_attr("s", &offset);
        device.write_attr("x", &p.x());
        device.write_attr("y", &p.y());
        device.write_attr("hdg", &hdg);
        device.write_attr("length", &length);

        device.open_tag("paramPoly3");
        device.write_attr("aU", &aU);
        device.write_attr("bU", &bU);
        device.write_attr("cU", &cU);
        device.write_attr("dU", &dU);
        device.write_attr("aV", &aV);
        device.write_attr("bV", &bV);
        device.write_attr("cV", &cV);
        device.write_attr("dV", &dV);
        device.write_attr("pRange", "normalized");
        device.close_tag("");
        device.close_tag("");

        // write elevation
        elevationDevice.open_tag("elevation");
        elevationDevice.write_attr("s", &offset);
        elevationDevice.write_attr("a", &aZ);
        elevationDevice.write_attr("b", &bZ);
        elevationDevice.write_attr("c", &cZ);
        elevationDevice.write_attr("d", &dZ);
        elevationDevice.close_tag("");

        offset + length
    }

    /// C++: `void NWWriter_OpenDrive::writeElevationProfile(const PositionVector &, OutputDevice &, const OutputDevice_String &)` [static, protected]
    // void
    // NWWriter_OpenDrive::writeElevationProfile(const PositionVector& shape, OutputDevice& device, const OutputDevice_String& elevationDevice) {
    //     // check if the shape is flat
    //     bool flat = true;
    //     double z = shape.size() == 0 ? 0 : shape[0].z();
    //     for (int i = 1; i < (int)shape.size(); ++i) {
    //         if (fabs(shape[i].z() - z) > NUMERICAL_EPS) {
    //             flat = false;
    //             break;
    //         }
    //     }
    //     device << "        <elevationProfile>\n";
    //     if (flat) {
    //         device << "            <elevation s=\"0\" a=\"" << z << "\" b=\"0\" c=\"0\" d=\"0\"/>\n";
    //     } else {
    //         device << elevationDevice.getString();
    //     }
    //     device << "        </elevationProfile>\n";
    //
    // }
    #[allow(unused_variables)]
    pub fn write_elevation_profile(shape: &PositionVector, device: &mut OutputDevice, elevationDevice: &OutputDevice /* OutputDevice_String */) {
        // check if the shape is flat
        let mut flat = true;
        let z: f64 = if shape.len() == 0 { 0.0 } else { shape[0].z() };
        for i in 1..shape.len() {
            if (shape[i].z() - z).abs() > utils::geom::NUMERICAL_EPS {
                flat = false;
                break;
            }
        }
        device.write("        <elevationProfile>\n");
        if flat {
            device.write("            <elevation s=\"0\" a=\"");
            device.write(&z);
            device.write("\" b=\"0\" c=\"0\" d=\"0\"/>\n");
        } else {
            device.write(&elevationDevice.get_string());
        }
        device.write("        </elevationProfile>\n");
    }

    /// C++: `void NWWriter_OpenDrive::writeEmptyCenterLane(OutputDevice &, const std::string &, double)` [static, protected]
    // void
    // NWWriter_OpenDrive::writeEmptyCenterLane(OutputDevice& device, const std::string& mark, double markWidth) {
    //     device << "                <center>\n";
    //     device << "                    <lane id=\"0\" type=\"none\" level=\"true\">\n";
    //     device << "                        <link/>\n";
    //     device << "                        <roadMark sOffset=\"0\" type=\"" << mark << "\" weight=\"standard\" color=\"standard\" width=\"" << markWidth << "\"/>\n";
    //     device << "                    </lane>\n";
    //     device << "                </center>\n";
    // }
    #[allow(unused_variables)]
    pub fn write_empty_center_lane(device: &mut OutputDevice, mark: &str, markWidth: f64) {
        device.write("                <center>\n");
        device.write("                    <lane id=\"0\" type=\"none\" level=\"true\">\n");
        device.write("                        <link/>\n");
        device.write("                        <roadMark sOffset=\"0\" type=\"");
        device.write(mark);
        device.write("\" weight=\"standard\" color=\"standard\" width=\"");
        device.write(&markWidth);
        device.write("\"/>\n");
        device.write("                    </lane>\n");
        device.write("                </center>\n");
    }

    /// C++: `int NWWriter_OpenDrive::getID(const std::string &, StringBijection<int> &, int &)` [static, protected]
    // int
    // NWWriter_OpenDrive::getID(const std::string& origID, StringBijection<int>& map, int& lastID) {
    //     if (map.hasString(origID)) {
    //         return map.get(origID);
    //     }
    //     map.insert(origID, lastID++);
    //     return lastID - 1;
    // }
    #[allow(unused_variables)]
    pub fn get_id(origID: &str, map: &mut StringBijection<i32>, lastID: &mut i32) -> i32 {
        if map.has_string(origID) {
            return or_panic(map.get(origID));
        }
        let key = *lastID;
        *lastID += 1;
        or_panic(map.insert(origID, key, true));
        *lastID - 1
    }

    /// C++: `std::string NWWriter_OpenDrive::getLaneType(SVCPermissions)` [static, protected]
    // std::string
    // NWWriter_OpenDrive::getLaneType(SVCPermissions permissions) {
    //     switch (permissions) {
    //         case SVC_PEDESTRIAN:
    //             return "sidewalk";
    //         //case (SVC_BICYCLE | SVC_PEDESTRIAN):
    //         //    WRITE_WARNING("Ambiguous lane type (biking+driving) for road '" + roadID + "'");
    //         //    return "sidewalk";
    //         case SVC_BICYCLE:
    //             return "biking";
    //         case 0:
    //             // ambiguous
    //             return "none";
    //         case SVC_RAIL:
    //         case SVC_RAIL_URBAN:
    //         case SVC_RAIL_ELECTRIC:
    //         case SVC_RAIL_FAST:
    //             return "rail";
    //         case SVC_TRAM:
    //             return "tram";
    //         default: {
    //             // complex permissions
    //             if (permissions == SVCAll) {
    //                 return "driving";
    //             } else if (isRailway(permissions)) {
    //                 return "rail";
    //             } else if ((permissions & SVC_PASSENGER) != 0) {
    //                 return "driving";
    //             } else {
    //                 return "restricted";
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_lane_type(permissions: SVCPermissions) -> String {
        match permissions {
            SVC_PEDESTRIAN => "sidewalk".to_string(),
            //case (SVC_BICYCLE | SVC_PEDESTRIAN): (C++ disabled warning branch)
            SVC_BICYCLE => "biking".to_string(),
            // ambiguous
            0 => "none".to_string(),
            SVC_RAIL | SVC_RAIL_URBAN | SVC_RAIL_ELECTRIC | SVC_RAIL_FAST => "rail".to_string(),
            SVC_TRAM => "tram".to_string(),
            // complex permissions
            _ => {
                if permissions == SVC_ALL {
                    "driving".to_string()
                } else if utils::common::sumo_vehicle_class::is_railway(permissions) {
                    "rail".to_string()
                } else if (permissions & SVC_PASSENGER) != 0 {
                    "driving".to_string()
                } else {
                    "restricted".to_string()
                }
            }
        }
    }

    /// C++: `PositionVector NWWriter_OpenDrive::getInnerLaneBorder(const NBEdge *, int, double)` [static, protected]
    /// get the lane border that is closer to the reference line (center line of the edge)
    // PositionVector
    // NWWriter_OpenDrive::getInnerLaneBorder(const NBEdge* edge, int laneIndex, double widthOffset) {
    //     if (laneIndex == -1) {
    //         // innermost lane
    //         laneIndex = (int)edge->getNumLanes() - 1;
    //         if (LHRL) {
    //             laneIndex = 0;
    //         }
    //     }
    //     PositionVector result = edge->getLaneShape(laneIndex);
    //     widthOffset -= (LHLL ? -1 : 1) * edge->getLaneWidth(laneIndex) / 2;
    //     try {
    //         result.move2side(widthOffset);
    //     } catch (InvalidArgument&) { }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_inner_lane_border(edge: Ptr<NBEdge>, laneIndex: i32, widthOffset: f64) -> PositionVector {
        let mut laneIndex = laneIndex;
        let mut widthOffset = widthOffset;
        if laneIndex == -1 {
            // innermost lane
            laneIndex = edge.get_num_lanes() - 1;
            if lhrl() {
                laneIndex = 0;
            }
        }
        let mut result: PositionVector = edge.get_lane_shape(laneIndex).clone();
        widthOffset -= (if lhll() { -1.0 } else { 1.0 }) * edge.get_lane_width_2(laneIndex) / 2.0;
        // C++ catches InvalidArgument from move2side here; the Rust move2side does not fail
        result.move2side(widthOffset, 100.0);
        result
    }

    /// C++: `PositionVector NWWriter_OpenDrive::getOuterLaneBorder(const NBEdge *, int)` [static, protected]
    /// get the lane border that is further away from the reference line (center line of the edge)
    // PositionVector
    // NWWriter_OpenDrive::getOuterLaneBorder(const NBEdge* edge, int laneIndex) {
    //     return getInnerLaneBorder(edge, laneIndex, edge->getLaneWidth(laneIndex));
    // }
    #[allow(unused_variables)]
    pub fn get_outer_lane_border(edge: Ptr<NBEdge>, laneIndex: i32) -> PositionVector {
        Self::get_inner_lane_border(edge, laneIndex, edge.get_lane_width_2(laneIndex))
    }

    /// C++: `void NWWriter_OpenDrive::checkLaneGeometries(const NBEdge *)` [static, protected]
    /// check if the lane geometries are compatible with OpenDRIVE assumptions (colinear stop line)
    // void
    // NWWriter_OpenDrive::checkLaneGeometries(const NBEdge* e) {
    //     if (e->getNumLanes() > 1) {
    //         // compute 'stop line' of rightmost lane
    //         const PositionVector shape0 = e->getLaneShape(0);
    //         assert(shape0.size() >= 2);
    //         const Position& from = shape0[-2];
    //         const Position& to = shape0[-1];
    //         PositionVector stopLine;
    //         stopLine.push_back(to);
    //         stopLine.push_back(to - PositionVector::sideOffset(from, to, lefthand ? 1000 : -1000));
    //         // endpoints of all other lanes should be on the stop line
    //         for (int lane = 1; lane < e->getNumLanes(); ++lane) {
    //             const double dist = stopLine.distance2D(e->getLaneShape(lane)[-1]);
    //             if (dist > NUMERICAL_EPS) {
    //                 WRITE_WARNINGF(TL("Uneven stop line at lane '%' (dist=%) cannot be represented in OpenDRIVE."), e->getLaneID(lane), toString(dist));
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn check_lane_geometries(e: Ptr<NBEdge>) {
        if e.get_num_lanes() > 1 {
            // compute 'stop line' of rightmost lane
            let shape0: PositionVector = e.get_lane_shape(0).clone();
            assert!(shape0.len() >= 2);
            let from: Position = *shape0.at(-2);
            let to: Position = *shape0.at(-1);
            let mut stopLine = PositionVector::default();
            stopLine.0.push(to);
            stopLine.0.push(to - PositionVector::side_offset(&from, &to, if lefthand() { 1000.0 } else { -1000.0 }));
            // endpoints of all other lanes should be on the stop line
            for lane in 1..e.get_num_lanes() {
                let dist = stopLine.distance2d(e.get_lane_shape(lane).at(-1), false);
                if dist > utils::geom::NUMERICAL_EPS {
                    utils::common::msg_handler::write_warning(&format!(
                        "Uneven stop line at lane '{}' (dist={}) cannot be represented in OpenDRIVE.",
                        e.get_lane_id(lane),
                        to_string(&dist)
                    ));
                }
            }
        }
    }

    /// C++: `void NWWriter_OpenDrive::writeRoadObjects(OutputDevice &, const NBEdge *, const ShapeContainer &, const std::vector<std::string> &)` [static, protected]
    /// write road objects referenced as edge parameters
    // void
    // NWWriter_OpenDrive::writeRoadObjects(OutputDevice& device, const NBEdge* e, const ShapeContainer& shc, const std::vector<std::string>& crossings) {
    //     device.openTag("objects");
    //     if (e->hasParameter(ROAD_OBJECTS)) {
    //         device.setPrecision(8); // geometry hdg requires higher precision
    //         PositionVector road = getInnerLaneBorder(e);
    //         for (std::string id : StringTokenizer(e->getParameter(ROAD_OBJECTS, "")).getVector()) {
    //             SUMOPolygon* p = shc.getPolygons().get(id);
    //             if (p == nullptr) {
    //                 PointOfInterest* poi = shc.getPOIs().get(id);
    //                 if (poi == nullptr) {
    //                     WRITE_WARNINGF("Road object polygon or POI '%' not found for edge '%'", id, e->getID());
    //                 } else {
    //                     writeRoadObjectPOI(device, e, road, poi);
    //                 }
    //             } else {
    //                 writeRoadObjectPoly(device, e, road, p);
    //             }
    //         }
    //         device.setPrecision(gPrecision);
    //     }
    //     if (crossings.size() > 0) {
    //         device.setPrecision(8); // geometry hdg requires higher precision
    //         PositionVector road = getInnerLaneBorder(e);
    //         for (size_t ic = 0; ic < crossings.size(); ic++) {
    //             SUMOPolygon* p = shc.getPolygons().get(crossings[ic]);
    //             if (p != 0) {
    //                 writeRoadObjectPoly(device, e, road, p);
    //             }
    //         }
    //         device.setPrecision(gPrecision);
    //     }
    //     device.closeTag();
    // }
    #[allow(unused_variables)]
    pub fn write_road_objects(device: &mut OutputDevice, e: Ptr<NBEdge>, shc: &ShapeContainer, crossings: &Vec<String>) {
        device.open_tag("objects");
        if e.base2.has_parameter("roadObjects") {
            device.set_precision(8); // geometry hdg requires higher precision
            let road = Self::get_inner_lane_border(e, -1, 0.0);
            for id in utils::common::string_tokenizer::StringTokenizer::new(&e.base2.get_parameter("roadObjects", "")).get_vector() {
                let p: Option<Ptr<SUMOPolygon>> = shc.get_polygons().get(&id).cloned();
                match p {
                    None => {
                        let poi: Option<Ptr<PointOfInterest>> = shc.myPOIs.get(&id).cloned();
                        match poi {
                            None => utils::common::msg_handler::write_warning(&format!(
                                "Road object polygon or POI '{}' not found for edge '{}'",
                                id,
                                e.get_id()
                            )),
                            Some(poi) => Self::write_road_object_poi(device, e, &road, poi),
                        }
                    }
                    Some(p) => Self::write_road_object_poly(device, e, &road, p),
                }
            }
            device.set_precision(utils::common::std_defs::g_precision());
        }
        if crossings.len() > 0 {
            device.set_precision(8); // geometry hdg requires higher precision
            let road = Self::get_inner_lane_border(e, -1, 0.0);
            for ic in 0..crossings.len() {
                let p: Option<Ptr<SUMOPolygon>> = shc.get_polygons().get(&crossings[ic]).cloned();
                if let Some(p) = p {
                    Self::write_road_object_poly(device, e, &road, p);
                }
            }
            device.set_precision(utils::common::std_defs::g_precision());
        }
        device.close_tag("");
    }

    /// C++: `void NWWriter_OpenDrive::writeSignals(OutputDevice &, const NBEdge *, double, SignalLanes &, const ShapeContainer &)` [static, protected]
    /// write signal record for traffic light
    // void
    // NWWriter_OpenDrive::writeSignals(OutputDevice& device, const NBEdge* e, double length,
    //                                  SignalLanes& signalLanes, const ShapeContainer& shc) {
    //     device.openTag("signals");
    //     if (e->getToNode()->isTLControlled()) {
    //         // try to faithfully represent the SUMO signal layout
    //         // (if a realistic number of signals is needed, the user should set
    //         // option --tls.group-signals)
    //         NBTrafficLightDefinition* tl = *e->getToNode()->getControllingTLS().begin();
    //         std::map<std::string, bool> toWrite;
    //         for (const NBConnection& c : tl->getControlledLinks()) {
    //             if (c.getFrom() == e) {
    //                 const std::string id = tl->getID() + "_" + toString(c.getTLIndex());
    //                 if (toWrite.count(id) == 0) {
    //                     toWrite[id] = signalLanes.count(id) == 0;
    //                 }
    //                 signalLanes[id].first.insert(c.getFromLane());
    //                 signalLanes[id].second.insert(e->getToNode()->getDirection(e, c.getTo()));
    //             }
    //         }
    //         for (auto item : toWrite) {
    //             const std::string id = item.first;
    //             const bool isNew = item.second;
    //             const std::set<LinkDirection>& dirs = signalLanes[id].second;
    //             const bool l = dirs.count(LinkDirection::LEFT) != 0 || dirs.count(LinkDirection::PARTLEFT) != 0;
    //             const bool r = dirs.count(LinkDirection::RIGHT) != 0 || dirs.count(LinkDirection::PARTRIGHT) != 0;
    //             const bool s = dirs.count(LinkDirection::STRAIGHT) != 0;
    //             const std::string tag = isNew ? "signal" : "signalReference";
    //             int firstLane = *signalLanes[id].first.begin();
    //             double t = e->getLaneWidth(firstLane) * 0.5;
    //             for (int i = firstLane + 1; i < e->getNumLanes(); i++) {
    //                 t += e->getLaneWidth(i);
    //             }
    //             device.openTag(tag);
    //             device.writeAttr("id", id);
    //             device.setPrecision(8);
    //             device.writeAttr("s", length);
    //             device.writeAttr("t", -t);
    //             device.writeAttr("orientation", "+");
    //             if (isNew) {
    //                 int type = 1000001;
    //                 int subType = -1;
    //                 if (l && !s && !r) {
    //                     type = 1000011;
    //                     subType = 10;
    //                 } else if (!l && !s && r) {
    //                     type = 1000011;
    //                     subType = 20;
    //                 } else if (!l && s && !r) {
    //                     type = 1000011;
    //                     subType = 30;
    //                 } else if (l && s && !r) {
    //                     type = 1000011;
    //                     subType = 40;
    //                 } else if (!l && s && r) {
    //                     type = 1000011;
    //                     subType = 50;
    //                 }
    //                 device.writeAttr("dynamic", "yes");
    //                 device.writeAttr("zOffset", 5);
    //                 device.writeAttr("country", "OpenDRIVE");
    //                 device.writeAttr("type", type);
    //                 device.writeAttr("subtype", subType);
    //                 device.writeAttr("height", 0.78);
    //                 device.writeAttr("width", 0.26);
    //             }
    //             for (int lane : signalLanes[id].first) {
    //                 device.openTag("validity");
    //                 device.writeAttr("fromLane", s2x(lane, e->getNumLanes()));
    //                 device.writeAttr("toLane", s2x(lane, e->getNumLanes()));
    //                 device.closeTag();
    //             }
    //             device.closeTag();
    //         }
    //     } else if (e->hasParameter(ROAD_OBJECTS)) {
    //         PositionVector roadShape = getInnerLaneBorder(e);
    //         for (std::string id : StringTokenizer(e->getParameter(ROAD_OBJECTS, "")).getVector()) {
    //             PointOfInterest* poi = shc.getPOIs().get(id);
    //             if (poi != nullptr) {
    //                 if (poi->getShapeType() == "traffic_sign" && poi->hasParameter("traffic_sign")) {
    //                     std::string traffic_sign_type = poi->getParameter("traffic_sign");
    //
    //                     std::vector<TrafficSign> trafficSigns = parseTrafficSign(traffic_sign_type, poi);
    //
    //                     auto distance = roadShape.nearest_offset_to_point2D(*poi, true);
    //                     double t = getRoadSideOffset(e);
    //                     double calculatedZOffset = 3.0;
    //                     for (auto it = trafficSigns.rbegin(); it != trafficSigns.rend(); ++it) {
    //                         TrafficSign trafficSign = *it;
    //                         device.openTag("signal");
    //                         device.writeAttr("id", id);
    //                         device.writeAttr("s", distance);
    //                         device.writeAttr("t", t);
    //                         device.writeAttr("orientation", "-");
    //                         device.writeAttr("dynamic", "no");
    //                         device.writeAttr("zOffset", calculatedZOffset);
    //                         device.writeAttr("country", trafficSign.country);
    //                         device.writeAttr("type", trafficSign.type);
    //                         device.writeAttr("subtype", trafficSign.subtype);
    //                         device.writeAttr("value", trafficSign.value);
    //                         device.writeAttr("height", 0.78);
    //                         device.writeAttr("width", 0.78);
    //                         device.closeTag();
    //                         calculatedZOffset += 0.78;
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     device.closeTag();
    // }
    #[allow(unused_variables)]
    pub fn write_signals(device: &mut OutputDevice, e: Ptr<NBEdge>, length: f64, signalLanes: &mut NWWriter_OpenDrive_SignalLanes, shc: &ShapeContainer) {
        device.open_tag("signals");
        if e.get_to_node().is_tl_controlled() {
            // try to faithfully represent the SUMO signal layout
            // (if a realistic number of signals is needed, the user should set
            // option --tls.group-signals)
            // determinism: C++ std::set<NBTrafficLightDefinition*> is pointer-ordered; Rust BTreeSet<Ptr> is too
            let tl: Ptr<NBTrafficLightDefinition> = *e.get_to_node().get_controlling_tls().iter().next().unwrap();
            let mut toWrite: BTreeMap<String, bool> = BTreeMap::new();
            for c in tl.get_controlled_links() {
                if c.get_from() == e {
                    let id = format!("{}_{}", tl.get_id(), c.get_tl_index());
                    if !toWrite.contains_key(&id) {
                        toWrite.insert(id.clone(), !signalLanes.contains_key(&id));
                    }
                    let entry = signalLanes.entry(id.clone()).or_insert_with(|| (BTreeSet::new(), BTreeSet::new()));
                    entry.0.insert(c.get_from_lane());
                    entry.1.insert(e.get_to_node().get_direction(e, c.get_to(), false));
                }
            }
            for (id, isNew) in toWrite.iter() {
                let isNew = *isNew;
                let dirs: BTreeSet<LinkDirection> = signalLanes[id].1.clone();
                let l = dirs.contains(&LinkDirection::LEFT) || dirs.contains(&LinkDirection::PARTLEFT);
                let r = dirs.contains(&LinkDirection::RIGHT) || dirs.contains(&LinkDirection::PARTRIGHT);
                let s = dirs.contains(&LinkDirection::STRAIGHT);
                let tag = if isNew { "signal" } else { "signalReference" };
                let firstLane: i32 = *signalLanes[id].0.iter().next().unwrap();
                let mut t: f64 = e.get_lane_width_2(firstLane) * 0.5;
                for i in (firstLane + 1)..e.get_num_lanes() {
                    t += e.get_lane_width_2(i);
                }
                device.open_tag(tag);
                device.write_attr("id", id.as_str());
                device.set_precision(8);
                device.write_attr("s", &length);
                device.write_attr("t", &(-t));
                device.write_attr("orientation", "+");
                if isNew {
                    let mut type_: i32 = 1000001;
                    let mut subType: i32 = -1;
                    if l && !s && !r {
                        type_ = 1000011;
                        subType = 10;
                    } else if !l && !s && r {
                        type_ = 1000011;
                        subType = 20;
                    } else if !l && s && !r {
                        type_ = 1000011;
                        subType = 30;
                    } else if l && s && !r {
                        type_ = 1000011;
                        subType = 40;
                    } else if !l && s && r {
                        type_ = 1000011;
                        subType = 50;
                    }
                    device.write_attr("dynamic", "yes");
                    device.write_attr("zOffset", &5i32);
                    device.write_attr("country", "OpenDRIVE");
                    device.write_attr("type", &type_);
                    device.write_attr("subtype", &subType);
                    device.write_attr("height", &0.78f64);
                    device.write_attr("width", &0.26f64);
                }
                let lanes: Vec<i32> = signalLanes[id].0.iter().cloned().collect();
                for lane in lanes {
                    device.open_tag("validity");
                    device.write_attr("fromLane", &s2x(lane, e.get_num_lanes()));
                    device.write_attr("toLane", &s2x(lane, e.get_num_lanes()));
                    device.close_tag("");
                }
                device.close_tag("");
            }
        } else if e.base2.has_parameter("roadObjects") {
            let roadShape = Self::get_inner_lane_border(e, -1, 0.0);
            for id in utils::common::string_tokenizer::StringTokenizer::new(&e.base2.get_parameter("roadObjects", "")).get_vector() {
                let poi: Option<Ptr<PointOfInterest>> = shc.myPOIs.get(&id).cloned();
                if let Some(poi) = poi {
                    if poi.base.get_shape_type() == "traffic_sign" && poi.base3.has_parameter("traffic_sign") {
                        let traffic_sign_type = poi.base3.get_parameter("traffic_sign", "");

                        let trafficSigns = Self::parse_traffic_sign(&traffic_sign_type, poi);

                        let distance = roadShape.nearest_offset_to_point2d(&poi.base2, true);
                        let t = Self::get_road_side_offset(e);
                        let mut calculatedZOffset: f64 = 3.0;
                        for trafficSign in trafficSigns.iter().rev() {
                            device.open_tag("signal");
                            device.write_attr("id", id.as_str());
                            device.write_attr("s", &distance);
                            device.write_attr("t", &t);
                            device.write_attr("orientation", "-");
                            device.write_attr("dynamic", "no");
                            device.write_attr("zOffset", &calculatedZOffset);
                            device.write_attr("country", trafficSign.country.as_str());
                            device.write_attr("type", trafficSign.type_.as_str());
                            device.write_attr("subtype", trafficSign.subtype.as_str());
                            device.write_attr("value", trafficSign.value.as_str());
                            device.write_attr("height", &0.78f64);
                            device.write_attr("width", &0.78f64);
                            device.close_tag("");
                            calculatedZOffset += 0.78;
                        }
                    }
                }
            }
        }
        device.close_tag("");
    }

    /// C++: `int NWWriter_OpenDrive::s2x(int, int)` [static, protected]
    /// convert sumo lane index to xodr lane index
    // int
    // NWWriter_OpenDrive::s2x(int sumoIndex, int numLanes) {
    //     // sumo lanes:     0, 1, 2  (0 being the outermost lane)
    //     // XODR:          -3,-2,-1  (written in reverse order)
    //     // LHLL:           3, 2, 1  (written in reverse order)
    //     // lefthand (old):-1,-2,-3
    //     return (lefthand
    //             ? (LHLL
    //                ? numLanes - sumoIndex
    //                : - sumoIndex - 1)
    //             : sumoIndex - numLanes);
    // }
    #[allow(unused_variables)]
    pub fn s2x(sumoIndex: i32, numLanes: i32) -> i32 {
        if lefthand() {
            if lhll() {
                numLanes - sumoIndex
            } else {
                -sumoIndex - 1
            }
        } else {
            sumoIndex - numLanes
        }
    }

    /// C++: `void NWWriter_OpenDrive::mapmatchRoadObjects(const ShapeContainer &, const NBEdgeCont &)` [static, protected]
    /// map pois and polygons to the closes edge
    // void
    // NWWriter_OpenDrive::mapmatchRoadObjects(const ShapeContainer& shc,  const NBEdgeCont& ec) {
    //     if (shc.getPolygons().size() == 0 && shc.getPOIs().size() == 0) {
    //         return;
    //     }
    //     const double maxDist = OptionsCont::getOptions().getFloat("opendrive-output.shape-match-dist");
    //     if (maxDist < 0) {
    //         return;
    //     }
    //     // register custom assignements
    //     std::set<std::string> assigned;
    //     for (auto it = ec.begin(); it != ec.end(); ++it) {
    //         NBEdge* e = it->second;
    //         if (e->hasParameter(ROAD_OBJECTS)) {
    //             for (std::string id : StringTokenizer(e->getParameter(ROAD_OBJECTS, "")).getVector()) {
    //                 assigned.insert(id);
    //             }
    //         }
    //     }
    //     // build rtree for edges
    //     NamedRTree r;
    //     for (auto it = ec.begin(); it != ec.end(); ++it) {
    //         NBEdge* edge = it->second;
    //         Boundary bound = edge->getGeometry().getBoxBoundary();
    //         bound.grow(maxDist);
    //         float min[2] = { static_cast<float>(bound.xmin()), static_cast<float>(bound.ymin()) };
    //         float max[2] = { static_cast<float>(bound.xmax()), static_cast<float>(bound.ymax()) };
    //         r.Insert(min, max, edge);
    //     }
    //
    //     for (auto itPoly = shc.getPolygons().begin(); itPoly != shc.getPolygons().end(); itPoly++) {
    //         SUMOPolygon* p = itPoly->second;
    //         Boundary bound = p->getShape().getBoxBoundary();
    //         float min[2] = { static_cast<float>(bound.xmin()), static_cast<float>(bound.ymin()) };
    //         float max[2] = { static_cast<float>(bound.xmax()), static_cast<float>(bound.ymax()) };
    //         std::set<const Named*> edges;
    //         Named::StoringVisitor visitor(edges);
    //         r.Search(min, max, visitor);
    //         std::vector<std::pair<double, std::string> > nearby;
    //         for (const Named* namedEdge : edges) {
    //             NBEdge* e = const_cast<NBEdge*>(dynamic_cast<const NBEdge*>(namedEdge));
    //             const double distance = VectorHelper<double>::minValue(p->getShape().distances(e->getLaneShape(0), true));
    //             if (distance <= maxDist) {
    //                 // sort by distance and ID to stabilize results
    //                 nearby.push_back(std::make_pair(distance, e->getID()));
    //                 //std::cout << " poly=" << p->getID() << " e=" << e->getID() << " dist=" << distance << "\n";
    //             }
    //         }
    //         if (nearby.size() > 0) {
    //             std::sort(nearby.begin(), nearby.end());
    //             NBEdge* closest = ec.retrieve(nearby.front().second);
    //             std::string objects = closest->getParameter(ROAD_OBJECTS, "");
    //             if (objects != "") {
    //                 objects += " ";
    //             }
    //             objects += p->getID();
    //             closest->setParameter(ROAD_OBJECTS, objects);
    //             //std::cout << "poly=" << p->getID() << " closest=" << closest->getID() << "\n";
    //         }
    //     }
    //     for (auto itPoi = shc.getPOIs().begin(); itPoi != shc.getPOIs().end(); itPoi++) {
    //         PointOfInterest* p = itPoi->second;
    //         float min[2] = { static_cast<float>(p->x()), static_cast<float>(p->y()) };
    //         float max[2] = { static_cast<float>(p->x()), static_cast<float>(p->y()) };
    //         std::set<const Named*> edges;
    //         Named::StoringVisitor visitor(edges);
    //         r.Search(min, max, visitor);
    //         std::vector<std::pair<double, std::string> > nearby;
    //         for (const Named* namedEdge : edges) {
    //             NBEdge* e = const_cast<NBEdge*>(dynamic_cast<const NBEdge*>(namedEdge));
    //             const double distance = e->getLaneShape(0).distance2D(*p, true);
    //             if (distance != GeomHelper::INVALID_OFFSET && distance <= maxDist) {
    //                 // sort by distance and ID to stabilize results
    //                 nearby.push_back(std::make_pair(distance, e->getID()));
    //                 //if (p->getID() == "1275468911") {
    //                 //    std::cout << " poly=" << p->getID() << " e=" << e->getID() << " dist=" << distance << "\n";
    //                 //}
    //             }
    //         }
    //         if (nearby.size() > 0) {
    //             std::sort(nearby.begin(), nearby.end());
    //             NBEdge* closest = ec.retrieve(nearby.front().second);
    //             std::string objects = closest->getParameter(ROAD_OBJECTS, "");
    //             if (objects != "") {
    //                 objects += " ";
    //             }
    //             objects += p->getID();
    //             closest->setParameter(ROAD_OBJECTS, objects);
    //             //std::cout << "poly=" << p->getID() << " closest=" << closest->getID() << "\n";
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn mapmatch_road_objects(shc: &ShapeContainer, ec: &NBEdgeCont) {
        if shc.get_polygons().len() == 0 && shc.myPOIs.len() == 0 {
            return;
        }
        let maxDist = utils::options::options_cont::get_float("opendrive-output.shape-match-dist");
        if maxDist < 0.0 {
            return;
        }
        // shortcut: the C++ NamedRTree is replaced by a linear bounding-box scan with the same overlap test;
        // the candidates are then sorted by (distance, id) as in C++, so results match. Upgrade to an R-tree for large nets.
        let mut edge_boxes: Vec<(Ptr<NBEdge>, [f32; 2], [f32; 2])> = Vec::new();
        for edge in ec.myEdges.values() {
            let mut bound = edge.get_geometry().get_box_boundary();
            bound.grow(maxDist);
            let min = [bound.xmin() as f32, bound.ymin() as f32];
            let max = [bound.xmax() as f32, bound.ymax() as f32];
            edge_boxes.push((*edge, min, max));
        }
        let search = |min: [f32; 2], max: [f32; 2]| -> Vec<Ptr<NBEdge>> {
            edge_boxes
                .iter()
                .filter(|(_, emin, emax)| emin[0] <= max[0] && emax[0] >= min[0] && emin[1] <= max[1] && emax[1] >= min[1])
                .map(|(e, _, _)| *e)
                .collect()
        };
        // C++ minValue(): starts at numeric max and keeps the smallest entry
        let min_value = |v: &Vec<f64>| -> f64 {
            let mut m = f64::MAX;
            for x in v {
                if *x < m {
                    m = *x;
                }
            }
            m
        };

        for p in shc.get_polygons().values() {
            let bound = p.get_shape().get_box_boundary();
            let min = [bound.xmin() as f32, bound.ymin() as f32];
            let max = [bound.xmax() as f32, bound.ymax() as f32];
            let mut nearby: Vec<(f64, String)> = Vec::new();
            for e in search(min, max) {
                let distance = min_value(&p.get_shape().distances(e.get_lane_shape(0), true));
                if distance <= maxDist {
                    // sort by distance and ID to stabilize results
                    nearby.push((distance, e.get_id().to_string()));
                }
            }
            if nearby.len() > 0 {
                nearby.sort_by(|a, b| a.partial_cmp(b).unwrap());
                let closest = ec.retrieve(&nearby[0].1, false);
                let mut objects = closest.base2.get_parameter("roadObjects", "");
                if objects != "" {
                    objects += " ";
                }
                objects += p.get_id();
                closest.m().base2.set_parameter("roadObjects", &objects);
            }
        }
        for p in shc.myPOIs.values() {
            let min = [p.base2.x() as f32, p.base2.y() as f32];
            let max = [p.base2.x() as f32, p.base2.y() as f32];
            let mut nearby: Vec<(f64, String)> = Vec::new();
            for e in search(min, max) {
                let distance = e.get_lane_shape(0).distance2d(&p.base2, true);
                if distance != utils::geom::geom_helper::INVALID_OFFSET && distance <= maxDist {
                    // sort by distance and ID to stabilize results
                    nearby.push((distance, e.get_id().to_string()));
                }
            }
            if nearby.len() > 0 {
                nearby.sort_by(|a, b| a.partial_cmp(b).unwrap());
                let closest = ec.retrieve(&nearby[0].1, false);
                let mut objects = closest.base2.get_parameter("roadObjects", "");
                if objects != "" {
                    objects += " ";
                }
                objects += p.get_id();
                closest.m().base2.set_parameter("roadObjects", &objects);
            }
        }
    }

    /// C++: `void NWWriter_OpenDrive::writeRoadObjectPOI(OutputDevice &, const NBEdge *, const PositionVector &, const PointOfInterest *)` [static, protected]
    // void
    // NWWriter_OpenDrive::writeRoadObjectPOI(OutputDevice& device, const NBEdge* e, const PositionVector& roadShape, const PointOfInterest* poi) {
    //     Position center = *poi;
    //     const double edgeOffset = roadShape.nearest_offset_to_point2D(center, false);
    //     if (edgeOffset == GeomHelper::INVALID_OFFSET) {
    //         WRITE_WARNINGF("Cannot map road object POI '%' with center % onto edge '%'", poi->getID(), center, e->getID());
    //         return;
    //     }
    //
    //     Position edgePos = roadShape.positionAtOffset2D(edgeOffset);
    //     double sideOffset = center.distanceTo2D(edgePos);
    //     // determine sign of sideOffset
    //     PositionVector tmp = roadShape.getSubpart2D(MAX2(0.0, edgeOffset - 1), MIN2(roadShape.length2D(), edgeOffset + 1));
    //     tmp.move2side(sideOffset);
    //     if (tmp.distance2D(center) < sideOffset) {
    //         sideOffset *= -1;
    //     }
    //     // place traffic signs on appropriate side of the road
    //     std::string type = poi->getShapeType();
    //     std::string name = StringUtils::escapeXML(poi->getParameter("name", ""), true);
    //     if (poi->getShapeType() == "traffic_sign") {
    //         sideOffset = getRoadSideOffset(e);
    //         type = "pole";
    //         name = "pole";
    //     }
    //
    //     device.openTag("object");
    //     device.writeAttr("id", poi->getID());
    //     device.writeAttr("type", type);
    //     device.writeAttr("name", name);
    //     device.writeAttr("s", edgeOffset);
    //     device.writeAttr("t", sideOffset);
    //     device.writeAttr("hdg", 0);
    //     device.closeTag();
    // }
    #[allow(unused_variables)]
    pub fn write_road_object_poi(device: &mut OutputDevice, e: Ptr<NBEdge>, roadShape: &PositionVector, poi: Ptr<PointOfInterest>) {
        let center: Position = poi.base2;
        let edgeOffset = roadShape.nearest_offset_to_point2d(&center, false);
        if edgeOffset == utils::geom::geom_helper::INVALID_OFFSET {
            utils::common::msg_handler::write_warning(&format!(
                "Cannot map road object POI '{}' with center {} onto edge '{}'",
                poi.get_id(),
                center,
                e.get_id()
            ));
            return;
        }

        let edgePos = roadShape.position_at_offset2d(edgeOffset, 0.0, false);
        let mut sideOffset = center.distance_to2d(&edgePos);
        // determine sign of sideOffset
        let lo = edgeOffset - 1.0;
        let hi = roadShape.length2d();
        let hi2 = edgeOffset + 1.0;
        let tmp = roadShape.get_subpart2d(if 0.0 > lo { 0.0 } else { lo }, if hi > hi2 { hi2 } else { hi });
        let mut tmp = tmp;
        tmp.move2side(sideOffset, 100.0);
        if tmp.distance2d(&center, false) < sideOffset {
            sideOffset *= -1.0;
        }
        // place traffic signs on appropriate side of the road
        let mut type_: String = poi.base.get_shape_type().to_string();
        let mut name: String = utils::common::string_utils::escape_xml(&poi.base3.get_parameter("name", ""), true);
        if poi.base.get_shape_type() == "traffic_sign" {
            sideOffset = Self::get_road_side_offset(e);
            type_ = "pole".to_string();
            name = "pole".to_string();
        }

        device.open_tag("object");
        device.write_attr("id", poi.get_id());
        device.write_attr("type", type_.as_str());
        device.write_attr("name", name.as_str());
        device.write_attr("s", &edgeOffset);
        device.write_attr("t", &sideOffset);
        device.write_attr("hdg", &0i32);
        device.close_tag("");
    }

    /// C++: `void NWWriter_OpenDrive::writeRoadObjectPoly(OutputDevice &, const NBEdge *, const PositionVector &, const SUMOPolygon *)` [static, protected]
    // void
    // NWWriter_OpenDrive::writeRoadObjectPoly(OutputDevice& device, const NBEdge* e, const PositionVector& roadShape, const SUMOPolygon* p) {
    //     PositionVector shape = p->getShape();
    //     Position center = shape.getPolygonCenter();
    //
    //     const double edgeOffset = roadShape.nearest_offset_to_point2D(center, false);
    //     if (edgeOffset == GeomHelper::INVALID_OFFSET) {
    //         WRITE_WARNINGF("Cannot map road object polygon '%' with center % onto edge '%'", p->getID(), center, e->getID());
    //         return;
    //     }
    //     Position edgePos = roadShape.positionAtOffset2D(edgeOffset);
    //     const double edgeAngle = roadShape.rotationAtOffset(edgeOffset);
    //     double sideOffset = center.distanceTo2D(edgePos);
    //     // determine sign of sideOffset
    //     PositionVector tmp = roadShape.getSubpart2D(MAX2(0.0, edgeOffset - 1), MIN2(roadShape.length2D(), edgeOffset + 1));
    //     tmp.move2side(sideOffset);
    //     if (tmp.distance2D(center) < sideOffset) {
    //         sideOffset *= -1;
    //     }
    //     //std::cout << " id=" << id
    //     //    << " shape=" << shape
    //     //    << " center=" << center
    //     //    << " edgeOffset=" << edgeOffset
    //     //    << "\n";
    //     auto shapeType = p->getShapeType();
    //     device.openTag("object");
    //     device.writeAttr("id", p->getID());
    //     device.writeAttr("type", shapeType);
    //     if (p->hasParameter("name")) {
    //         device.writeAttr("name", StringUtils::escapeXML(p->getParameter("name", ""), true));
    //     }
    //     device.writeAttr("s", edgeOffset);
    //     device.writeAttr("t", shapeType == "crosswalk" && !lefthand ? 0 : sideOffset);
    //     double hdg = -edgeAngle;
    //     if (p->hasParameter("hdg")) {
    //         try {
    //             hdg = StringUtils::toDoubleSecure(p->getParameter("hdg", ""), 0);
    //         } catch (NumberFormatException&) {}
    //     }
    //     device.writeAttr("hdg", hdg);
    //     if (p->hasParameter("length")) {
    //         try {
    //             device.writeAttr("length", StringUtils::toDoubleSecure(p->getParameter("length", ""), 0));
    //         } catch (NumberFormatException&) {}
    //     }
    //     if (p->hasParameter("width")) {
    //         try {
    //             device.writeAttr("width", StringUtils::toDoubleSecure(p->getParameter("width", ""), 0));
    //         } catch (NumberFormatException&) {}
    //     }
    //     double height = 0;
    //     if (p->hasParameter("height")) {
    //         try {
    //             height = StringUtils::toDoubleSecure(p->getParameter("height", ""), 0);
    //             device.writeAttr("height", height);
    //         } catch (NumberFormatException&) {}
    //     }
    //     //device.openTag("outlines");
    //     device.openTag("outline");
    //     device.writeAttr("id", 0);
    //     device.writeAttr("fillType", "concrete");
    //     device.writeAttr("outer", "true");
    //     device.writeAttr("closed", p->getShape().isClosed() ? "true" : "false");
    //     device.writeAttr("laneType", "border");
    //
    //     shape.sub(center);
    //     if (hdg != -edgeAngle) {
    //         shape.rotate2D(-edgeAngle - hdg);
    //     }
    //     int i = 0;
    //     for (Position pos : shape) {
    //         device.openTag("cornerLocal");
    //         device.writeAttr("u", pos.x());
    //         device.writeAttr("v", pos.y());
    //         device.writeAttr("z", MAX2(0.0, p->getShapeLayer()));
    //         device.writeAttr("height", height);
    //         device.writeAttr("id", i++);
    //         device.closeTag();
    //     }
    //
    //
    //     //device.closeTag();
    //     device.closeTag();
    //     device.closeTag();
    // }
    #[allow(unused_variables)]
    pub fn write_road_object_poly(device: &mut OutputDevice, e: Ptr<NBEdge>, roadShape: &PositionVector, p: Ptr<SUMOPolygon>) {
        let mut shape: PositionVector = p.get_shape().clone();
        let center: Position = shape.get_polygon_center();

        let edgeOffset = roadShape.nearest_offset_to_point2d(&center, false);
        if edgeOffset == utils::geom::geom_helper::INVALID_OFFSET {
            utils::common::msg_handler::write_warning(&format!(
                "Cannot map road object polygon '{}' with center {} onto edge '{}'",
                p.get_id(),
                center,
                e.get_id()
            ));
            return;
        }
        let edgePos = roadShape.position_at_offset2d(edgeOffset, 0.0, false);
        let edgeAngle = roadShape.rotation_at_offset(edgeOffset);
        let mut sideOffset = center.distance_to2d(&edgePos);
        // determine sign of sideOffset
        let lo = edgeOffset - 1.0;
        let lo = if 0.0 > lo { 0.0 } else { lo };
        let len = roadShape.length2d();
        let hi2 = edgeOffset + 1.0;
        let hi = if len < hi2 { len } else { hi2 };
        let mut tmp = roadShape.get_subpart2d(lo, hi);
        tmp.move2side(sideOffset, 100.0);
        if tmp.distance2d(&center, false) < sideOffset {
            sideOffset *= -1.0;
        }
        let shapeType: String = p.base.get_shape_type().to_string();
        device.open_tag("object");
        device.write_attr("id", p.get_id());
        device.write_attr("type", shapeType.as_str());
        if p.base2.has_parameter("name") {
            device.write_attr("name", utils::common::string_utils::escape_xml(&p.base2.get_parameter("name", ""), true).as_str());
        }
        device.write_attr("s", &edgeOffset);
        if shapeType == "crosswalk" && !lefthand() {
            device.write_attr("t", &0.0f64);
        } else {
            device.write_attr("t", &sideOffset);
        }
        let mut hdg = -edgeAngle;
        if p.base2.has_parameter("hdg") {
            if let Ok(v) = utils::common::string_utils::to_double_secure(&p.base2.get_parameter("hdg", ""), 0.0) {
                hdg = v;
            }
        }
        device.write_attr("hdg", &hdg);
        if p.base2.has_parameter("length") {
            if let Ok(v) = utils::common::string_utils::to_double_secure(&p.base2.get_parameter("length", ""), 0.0) {
                device.write_attr("length", &v);
            }
        }
        if p.base2.has_parameter("width") {
            if let Ok(v) = utils::common::string_utils::to_double_secure(&p.base2.get_parameter("width", ""), 0.0) {
                device.write_attr("width", &v);
            }
        }
        let mut height: f64 = 0.0;
        if p.base2.has_parameter("height") {
            if let Ok(v) = utils::common::string_utils::to_double_secure(&p.base2.get_parameter("height", ""), 0.0) {
                height = v;
                device.write_attr("height", &height);
            }
        }
        //device.openTag("outlines");
        device.open_tag("outline");
        device.write_attr("id", &0i32);
        device.write_attr("fillType", "concrete");
        device.write_attr("outer", "true");
        device.write_attr("closed", if p.get_shape().is_closed() { "true" } else { "false" });
        device.write_attr("laneType", "border");

        shape.sub(&center);
        if hdg != -edgeAngle {
            shape.rotate2d(-edgeAngle - hdg);
        }
        let mut i: i32 = 0;
        for pos in shape.iter() {
            device.open_tag("cornerLocal");
            device.write_attr("u", &pos.x());
            device.write_attr("v", &pos.y());
            let layer = p.base.get_shape_layer();
            device.write_attr("z", &(if 0.0 > layer { 0.0 } else { layer }));
            device.write_attr("height", &height);
            device.write_attr("id", &i);
            i += 1;
            device.close_tag("");
        }

        //device.closeTag();
        device.close_tag("");
        device.close_tag("");
    }

    /// C++: `std::vector<TrafficSign> NWWriter_OpenDrive::parseTrafficSign(const std::string &, PointOfInterest *)` [static, protected]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn parse_traffic_sign(trafficSign: &str, poi: Ptr<PointOfInterest>) -> Vec<NWWriter_OpenDrive_TrafficSign> {
        let mut result: Vec<NWWriter_OpenDrive_TrafficSign> = Vec::new();
        let sign = |country: &str, type_: &str, subtype: &str, value: &str| NWWriter_OpenDrive_TrafficSign {
            country: country.to_string(),
            type_: type_.to_string(),
            subtype: subtype.to_string(),
            value: value.to_string(),
        };
        // check for maxspeed, stop, give_way and hazard
        if trafficSign == "maxspeed" && poi.base3.has_parameter("maxspeed") {
            result.push(sign("OpenDrive", "maxspeed", "", &poi.base3.get_parameter("maxspeed", "")));
        } else if trafficSign == "stop" {
            result.push(sign("OpenDrive", trafficSign, "", ""));
        } else if trafficSign == "give_way" {
            result.push(sign("OpenDrive", trafficSign, "", ""));
        } else if trafficSign == "hazard" && poi.base3.has_parameter("hazard") {
            result.push(sign("OpenDrive", trafficSign, &poi.base3.get_parameter("hazard", ""), ""));
        } else {
            if trafficSign.find(|c| c == ',' || c == ';').is_some() {
                // C++ substr with npos: without a colon the whole string is both country and remainder
                let (country, remaining): (String, String) = match trafficSign.find(':') {
                    Some(colon) => (trafficSign[..colon].to_string(), trafficSign[colon + 1..].to_string()),
                    None => (trafficSign.to_string(), trafficSign.to_string()),
                };
                let mut tokens: Vec<String> = Vec::new();
                let mut lastPos: usize = 0;
                let mut pos = remaining.find(|c| c == ',' || c == ';');
                while let Some(p) = pos {
                    // add country and colon before pushing the token
                    tokens.push(format!("{}:{}", country, &remaining[lastPos..p]));
                    lastPos = p + 1;
                    pos = remaining[lastPos..].find(|c| c == ',' || c == ';').map(|x| x + lastPos);
                }
                tokens.push(format!("{}:{}", country, &remaining[lastPos..]));
                // call for each token the parseTrafficSignId function and fill the result vector
                for token in tokens {
                    result.push(Self::parse_traffic_sign_id(&token));
                }
            } else {
                result.push(Self::parse_traffic_sign_id(trafficSign));
            }
        }
        result
    }

    /// C++: `TrafficSign NWWriter_OpenDrive::parseTrafficSignId(const std::string &)` [static, protected]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn parse_traffic_sign_id(trafficSign: &str) -> NWWriter_OpenDrive_TrafficSign {
        // for OpenDrive 1.4 the country code is specified as ISO 3166-1, alpha-3,
        // but OSM uses ISO 3166-1, alpha-2. Newer OpenDrive versions support alpha-2, so that is used.
        // std::regex: ([A-Z]{2}):([0-9.]{1,6}|[A-Z]{1}[0-9.]{2,6})(?:\[(.*)\])?(?:-(.*))?
        match match_traffic_sign_id(trafficSign) {
            Some((country, id, bracket, hyphen)) => {
                // value in brackets (group 3) takes precedence over the value after the hyphen (group 4)
                let value = if let Some(v) = bracket {
                    v
                } else if let Some(v) = hyphen {
                    v
                } else {
                    String::new()
                };
                NWWriter_OpenDrive_TrafficSign { country, type_: id, subtype: String::new(), value }
            }
            None => NWWriter_OpenDrive_TrafficSign {
                country: "OpenDrive".to_string(),
                type_: trafficSign.to_string(),
                subtype: String::new(),
                value: String::new(),
            },
        }
    }

    /// C++: `double NWWriter_OpenDrive::getRoadSideOffset(const NBEdge *)` [static, protected]
    // double
    // NWWriter_OpenDrive::getRoadSideOffset(const NBEdge* e) {
    //     double t = 0.30;
    //     if (!lefthand || LHLL) {
    //         for (int i = 0; i < e->getNumLanes(); i++) {
    //             t += e->getPermissions(i) == SVC_PEDESTRIAN ? e->getLaneWidth(i) * 0.2 : e->getLaneWidth(i);
    //         }
    //     }
    //     t = LHRL ? t : -t;
    //     return t;
    // }
    #[allow(unused_variables)]
    pub fn get_road_side_offset(e: Ptr<NBEdge>) -> f64 {
        let mut t: f64 = 0.30;
        if !lefthand() || lhll() {
            for i in 0..e.get_num_lanes() {
                t += if e.get_permissions(i) == SVC_PEDESTRIAN { e.get_lane_width_2(i) * 0.2 } else { e.get_lane_width_2(i) };
            }
        }
        if lhrl() {
            t
        } else {
            -t
        }
    }

}

/// C++ class `NWWriter_OpenDrive::TrafficSign` (netwrite/NWWriter_OpenDrive.h)
pub struct NWWriter_OpenDrive_TrafficSign {
    pub country: String,
    pub type_: String,
    pub subtype: String,
    pub value: String,
}

impl NWWriter_OpenDrive_TrafficSign {
}

