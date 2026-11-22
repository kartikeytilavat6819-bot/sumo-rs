//! Port of netbuild/NBPTLine.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::msg_handler::write_warning;
use utils::common::string_utils::escape_xml;
use utils::common::std_defs::SUMO_CONST_LANE_WIDTH;

/// C++ class `NBPTLine` (netbuild/NBPTLine.h)
pub struct NBPTLine {
    pub myName: String,
    pub myType: String,
    pub myPTStops: Vec<Ptr<NBPTStop>>,
    pub myStopsRevised: Vec<bool>,
    pub myWayNodes: BTreeMap<String, Vec<i64>>,
    pub myWays: Vec<String>,
    pub myCurrentWay: String,
    pub myPTLineId: String,
    pub myRef: String,
    pub myColor: RGBColor,
    pub myInterval: i32,
    pub myNightService: String,
    pub myVClass: SUMOVehicleClass,
    pub myRoute: Vec<Ptr<NBEdge>>,
    pub myNumOfStops: i32,
    pub myMissingStopsBefore: i32,
    pub myMissingStopsAfter: i32,
}

impl NBPTLine {
    /// C++: `void NBPTLine::NBPTLine(const std::string &, const std::string &, const std::string &, const std::string &, int, const std::string &, SUMOVehicleClass, RGBColor)`
    // NBPTLine::NBPTLine(const std::string& id, const std::string& name, const std::string& type, const std::string& ref, int interval, const std::string& nightService,
    //                    SUMOVehicleClass vClass, RGBColor color) :
    //     myName(name),
    //     myType(type),
    //     myPTLineId(id),
    //     myRef(ref != "" ? ref : name),
    //     myColor(color),
    //     myInterval(interval),
    //     myNightService(nightService),
    //     myVClass(vClass),
    //     myNumOfStops(0),
    //     myMissingStopsBefore(0),
    //     myMissingStopsAfter(0)
    // { }
    #[allow(unused_variables)]
    pub fn new(id: &str, name: &str, type_: &str, ref_: &str, interval: i32, nightService: &str, vClass: SUMOVehicleClass, color: RGBColor) -> Self {
        NBPTLine {
            myName: name.to_string(),
            myType: type_.to_string(),
            myPTStops: Vec::new(),
            myStopsRevised: Vec::new(),
            myWayNodes: BTreeMap::new(),
            myWays: Vec::new(),
            myCurrentWay: String::new(),
            myPTLineId: id.to_string(),
            myRef: if ref_ != "" { ref_.to_string() } else { name.to_string() },
            myColor: color,
            myInterval: interval,
            myNightService: nightService.to_string(),
            myVClass: vClass,
            myRoute: Vec::new(),
            myNumOfStops: 0,
            myMissingStopsBefore: 0,
            myMissingStopsAfter: 0,
        }
    }

    /// C++: `void NBPTLine::addPTStop(std::shared_ptr<NBPTStop>)`
    // void
    // NBPTLine::addPTStop(std::shared_ptr<NBPTStop> pStop) {
    //     if (!myPTStops.empty() && pStop->getName() != "" && myPTStops.back()->getName() == pStop->getName()) {
    //         // avoid duplicate stop when both platform and stop_position are given as nodes
    //         if (myPTStops.back()->isPlatform() && !pStop->isPlatform()) {
    //             myPTStops.pop_back();
    //         } else if (pStop->isPlatform()) {
    //             return;
    //         }
    //     }
    //     myPTStops.push_back(pStop);
    // }
    #[allow(unused_variables)]
    pub fn add_pt_stop(&mut self, pStop: Ptr<NBPTStop>) {
        if !self.myPTStops.is_empty() && pStop.get_name() != "" && self.myPTStops[self.myPTStops.len() - 1].get_name() == pStop.get_name() {
            // avoid duplicate stop when both platform and stop_position are given as nodes
            if self.myPTStops[self.myPTStops.len() - 1].is_platform() && !pStop.is_platform() {
                self.myPTStops.pop();
            } else if pStop.is_platform() {
                return;
            }
        }
        self.myPTStops.push(pStop);
    }

    /// C++: `const std::string & NBPTLine::getLineID() const` [const]
    // const std::string& getLineID() const {
    //         return myPTLineId;
    //     }
    #[allow(unused_variables)]
    pub fn get_line_id(&self) -> &str {
        &self.myPTLineId
    }

    /// C++: `const std::string & NBPTLine::getName() const` [const]
    // const std::string& getName() const {
    //         return myName;
    //     }
    #[allow(unused_variables)]
    pub fn get_name(&self) -> &str {
        &self.myName
    }

    /// C++: `const std::string & NBPTLine::getType() const` [const]
    // const std::string& getType() const {
    //         return myType;
    //     }
    #[allow(unused_variables)]
    pub fn get_type(&self) -> &str {
        &self.myType
    }

    /// C++: `const std::vector<std::shared_ptr<NBPTStop>> & NBPTLine::getStops()`
    // const std::vector<std::shared_ptr<NBPTStop> >&
    // NBPTLine::getStops() {
    //     return myPTStops;
    // }
    #[allow(unused_variables)]
    pub fn get_stops(&mut self) -> &Vec<Ptr<NBPTStop>> {
        &self.myPTStops
    }

    /// C++: `void NBPTLine::write(OutputDevice &)`
    // void
    // NBPTLine::write(OutputDevice& device) {
    //     device.openTag(SUMO_TAG_PT_LINE);
    //     device.writeAttr(SUMO_ATTR_ID, myPTLineId);
    //     if (!myName.empty()) {
    //         device.writeAttr(SUMO_ATTR_NAME, StringUtils::escapeXML(myName));
    //     }
    //
    //     device.writeAttr(SUMO_ATTR_LINE, StringUtils::escapeXML(myRef));
    //     device.writeAttr(SUMO_ATTR_TYPE, myType);
    //     device.writeAttr(SUMO_ATTR_VCLASS, toString(myVClass));
    //     if (myInterval > 0) {
    //         // write seconds
    //         device.writeAttr(SUMO_ATTR_PERIOD, 60 * myInterval);
    //     }
    //     if (myNightService != "") {
    //         device.writeAttr("nightService", myNightService);
    //     }
    //
    //     if (myColor.isValid()) {
    //         device.writeAttr(SUMO_ATTR_COLOR, myColor);
    //     }
    //     device.writeAttr("completeness", (double)myPTStops.size() / myNumOfStops);
    //     if (myMissingStopsBefore != 0) {
    //         device.writeAttr("missingBefore", myMissingStopsBefore);
    //     }
    //     if (myMissingStopsAfter != 0) {
    //         device.writeAttr("missingAfter", myMissingStopsAfter);
    //     }
    //
    //     if (!myRoute.empty()) {
    //         device.openTag(SUMO_TAG_ROUTE);
    //         device.writeAttr(SUMO_ATTR_EDGES, myRoute);
    //         device.closeTag();
    //     }
    //
    //     for (auto& myPTStop : myPTStops) {
    //         device.openTag(SUMO_TAG_BUS_STOP);
    //         device.writeAttr(SUMO_ATTR_ID, myPTStop->getID());
    //         device.writeAttr(SUMO_ATTR_NAME, StringUtils::escapeXML(myPTStop->getName()));
    //         device.closeTag();
    //     }
    //     device.closeTag();
    //
    // }
    #[allow(unused_variables)]
    pub fn write(&mut self, device: &mut OutputDevice) {
        device.open_tag_id(SUMO_TAG_PT_LINE);
        device.write_attr(SUMO_ATTR_ID, &self.myPTLineId);
        if !self.myName.is_empty() {
            device.write_attr(SUMO_ATTR_NAME, &escape_xml(&self.myName, false));
        }

        device.write_attr(SUMO_ATTR_LINE, &escape_xml(&self.myRef, false));
        device.write_attr(SUMO_ATTR_TYPE, &self.myType);
        device.write_attr(SUMO_ATTR_VCLASS, SUMO_VEHICLE_CLASS_STRINGS.get_string(self.myVClass).unwrap());
        if self.myInterval > 0 {
            // write seconds
            device.write_attr(SUMO_ATTR_PERIOD, &(60 * self.myInterval));
        }
        if self.myNightService != "" {
            device.write_attr("nightService", &self.myNightService);
        }

        if self.myColor.is_valid() {
            device.write_attr(SUMO_ATTR_COLOR, &self.myColor.to_string());
        }
        device.write_attr("completeness", &(self.myPTStops.len() as f64 / self.myNumOfStops as f64));
        if self.myMissingStopsBefore != 0 {
            device.write_attr("missingBefore", &self.myMissingStopsBefore);
        }
        if self.myMissingStopsAfter != 0 {
            device.write_attr("missingAfter", &self.myMissingStopsAfter);
        }

        if !self.myRoute.is_empty() {
            device.open_tag_id(SUMO_TAG_ROUTE);
            let edgeIDs: Vec<String> = self.myRoute.iter().map(|e| e.get_id().to_string()).collect();
            device.write_attr(SUMO_ATTR_EDGES, &edgeIDs.join(" "));
            device.close_tag("");
        }

        for myPTStop in self.myPTStops.iter() {
            device.open_tag_id(SUMO_TAG_BUS_STOP);
            device.write_attr(SUMO_ATTR_ID, &myPTStop.get_id());
            device.write_attr(SUMO_ATTR_NAME, &escape_xml(&myPTStop.get_name(), false));
            device.close_tag("");
        }
        device.close_tag("");
    }

    /// C++: `void NBPTLine::addWayNode(long long, long long)`
    // void
    // NBPTLine::addWayNode(long long int way, long long int node) {
    //     std::string wayStr = toString(way);
    //     if (wayStr != myCurrentWay) {
    //         myCurrentWay = wayStr;
    //         myWays.push_back(wayStr);
    //     }
    //     myWayNodes[wayStr].push_back(node);
    // }
    #[allow(unused_variables)]
    pub fn add_way_node(&mut self, way: i64, node: i64) {
        let wayStr = way.to_string();
        if wayStr != self.myCurrentWay {
            self.myCurrentWay = wayStr.clone();
            self.myWays.push(wayStr.clone());
        }
        self.myWayNodes.entry(wayStr).or_default().push(node);
    }

    /// C++: `void NBPTLine::setNumOfStops(int, int, int)`
    // void
    // NBPTLine::setNumOfStops(int numStops, int missingBefore, int missingAfter) {
    //     myNumOfStops = numStops;
    //     myMissingStopsBefore = missingBefore;
    //     myMissingStopsAfter = missingAfter;
    // }
    #[allow(unused_variables)]
    pub fn set_num_of_stops(&mut self, numStops: i32, missingBefore: i32, missingAfter: i32) {
        self.myNumOfStops = numStops;
        self.myMissingStopsBefore = missingBefore;
        self.myMissingStopsAfter = missingAfter;
    }

    /// C++: `const std::string & NBPTLine::getRef() const` [const]
    /// get line reference (not unique)
    // const std::string& getRef() const {
    //         return myRef;
    //     }
    #[allow(unused_variables)]
    pub fn get_ref(&self) -> &str {
        &self.myRef
    }

    /// C++: `void NBPTLine::replaceStops(std::vector<std::shared_ptr<NBPTStop>>)`
    // void replaceStops(std::vector<std::shared_ptr<NBPTStop> > stops) {
    //         myPTStops = stops;
    //     }
    #[allow(unused_variables)]
    pub fn replace_stops(&mut self, stops: Vec<Ptr<NBPTStop>>) {
        self.myPTStops = stops;
    }

    /// C++: `void NBPTLine::setRevised(std::vector<bool>)`
    // void setRevised(std::vector<bool> stopsRevised) {
    //         myStopsRevised = stopsRevised;
    //     }
    #[allow(unused_variables)]
    pub fn set_revised(&mut self, stopsRevised: Vec<bool>) {
        self.myStopsRevised = stopsRevised;
    }

    /// C++: `std::vector<PTStopInfo> NBPTLine::getStopEdges(const NBEdgeCont &) const` [const]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_stop_edges(&self, ec: &NBEdgeCont) -> Vec<NBPTLine_PTStopInfo> {
        let mut result: Vec<NBPTLine_PTStopInfo> = Vec::new();
        let mut i: usize = 0;
        for stop in self.myPTStops.iter() {
            let e = ec.retrieve(stop.get_edge_id(), false);
            if !e.is_null() {
                let revised = if self.myStopsRevised.len() > i { self.myStopsRevised[i] } else { false };
                result.push(NBPTLine_PTStopInfo::new(e, &stop.get_id(), stop.get_end_pos(), revised));
            }
            i += 1;
        }
        result
    }

    /// C++: `NBEdge * NBPTLine::getRouteStart(const NBEdgeCont &) const` [const]
    /// return first valid edge of myRoute (if it doest not lie after the first stop)
    // NBEdge*
    // NBPTLine::getRouteStart(const NBEdgeCont& ec) const {
    //     std::vector<NBEdge*> validEdges;
    //     // filter out edges that have been removed due to joining junctions
    //     for (NBEdge* e : myRoute) {
    //         if (ec.retrieve(e->getID())) {
    //             validEdges.push_back(e);
    //         }
    //     }
    //     if (validEdges.size() == 0) {
    //         return nullptr;
    //     }
    //     // filter out edges after the first stop
    //     if (myPTStops.size() > 0) {
    //         NBEdge* firstStopEdge = ec.retrieve(myPTStops.front()->getEdgeId());
    //         if (firstStopEdge == nullptr) {
    //             WRITE_WARNINGF(TL("Could not retrieve edge '%' for first stop of line '%'."), myPTStops.front()->getEdgeId(), myPTLineId);
    //             return nullptr;
    //
    //         }
    //         auto it = std::find(validEdges.begin(), validEdges.end(), firstStopEdge);
    //         if (it == validEdges.end()) {
    //             WRITE_WARNINGF(TL("First stop edge '%' is not part of the route of line '%'."), firstStopEdge->getID(), myPTLineId);
    //             return nullptr;
    //         }
    //     }
    //     return validEdges.front();
    // }
    #[allow(unused_variables)]
    pub fn get_route_start(&self, ec: &NBEdgeCont) -> Ptr<NBEdge> {
        let mut validEdges: Vec<Ptr<NBEdge>> = Vec::new();
        // filter out edges that have been removed due to joining junctions
        for e in self.myRoute.iter() {
            if !ec.retrieve(e.get_id(), false).is_null() {
                validEdges.push(*e);
            }
        }
        if validEdges.is_empty() {
            return Ptr::null();
        }
        // filter out edges after the first stop
        if !self.myPTStops.is_empty() {
            let firstStopEdge = ec.retrieve(self.myPTStops[0].get_edge_id(), false);
            if firstStopEdge.is_null() {
                write_warning(&format!("Could not retrieve edge '{}' for first stop of line '{}'.", self.myPTStops[0].get_edge_id(), self.myPTLineId));
                return Ptr::null();
            }
            if !validEdges.contains(&firstStopEdge) {
                write_warning(&format!("First stop edge '{}' is not part of the route of line '{}'.", firstStopEdge.get_id(), self.myPTLineId));
                return Ptr::null();
            }
        }
        validEdges[0]
    }

    /// C++: `NBEdge * NBPTLine::getRouteEnd(const NBEdgeCont &) const` [const]
    /// return last valid edge of myRoute (if it doest not lie before the last stop)
    // NBEdge*
    // NBPTLine::getRouteEnd(const NBEdgeCont& ec) const {
    //     std::vector<NBEdge*> validEdges;
    //     // filter out edges that have been removed due to joining junctions
    //     for (NBEdge* e : myRoute) {
    //         if (ec.retrieve(e->getID())) {
    //             validEdges.push_back(e);
    //         }
    //     }
    //     if (validEdges.size() == 0) {
    //         return nullptr;
    //     }
    //     // filter out edges after the last stop
    //     if (myPTStops.size() > 0) {
    //         NBEdge* lastStopEdge = ec.retrieve(myPTStops.back()->getEdgeId());
    //         if (lastStopEdge == nullptr) {
    //             WRITE_WARNINGF(TL("Could not retrieve edge '%' for last stop of line '%'."), myPTStops.back()->getEdgeId(), myPTLineId);
    //             return nullptr;
    //
    //         }
    //         auto it = std::find(validEdges.begin(), validEdges.end(), lastStopEdge);
    //         if (it == validEdges.end()) {
    //             WRITE_WARNINGF(TL("Last stop edge '%' is not part of the route of line '%'."), lastStopEdge->getID(), myPTLineId);
    //             return nullptr;
    //         }
    //     }
    //     return validEdges.back();
    // }
    #[allow(unused_variables)]
    pub fn get_route_end(&self, ec: &NBEdgeCont) -> Ptr<NBEdge> {
        let mut validEdges: Vec<Ptr<NBEdge>> = Vec::new();
        // filter out edges that have been removed due to joining junctions
        for e in self.myRoute.iter() {
            if !ec.retrieve(e.get_id(), false).is_null() {
                validEdges.push(*e);
            }
        }
        if validEdges.is_empty() {
            return Ptr::null();
        }
        // filter out edges after the last stop
        if !self.myPTStops.is_empty() {
            let lastStop = self.myPTStops[self.myPTStops.len() - 1].clone();
            let lastStopEdge = ec.retrieve(lastStop.get_edge_id(), false);
            if lastStopEdge.is_null() {
                write_warning(&format!("Could not retrieve edge '{}' for last stop of line '{}'.", lastStop.get_edge_id(), self.myPTLineId));
                return Ptr::null();
            }
            if !validEdges.contains(&lastStopEdge) {
                write_warning(&format!("Last stop edge '{}' is not part of the route of line '{}'.", lastStopEdge.get_id(), self.myPTLineId));
                return Ptr::null();
            }
        }
        validEdges[validEdges.len() - 1]
    }

    /// C++: `bool NBPTLine::isConsistent(std::vector<NBEdge *>) const` [const]
    /// return whether the mentioned edges appear in that order in the route
    // bool
    // NBPTLine::isConsistent(std::vector<NBEdge*> stops) const {
    //     if (myRoute.empty() || stops.empty()) {
    //         return true;
    //     }
    //     if (stops.size() > 1 && stops.front() == stops.back()) {
    //         // circular route where we don't expect the route edges to occur twice
    //         if (myRoute.front() == stops.front()) {
    //             stops.pop_back();
    //         } else if (myRoute.back() == stops.back()) {
    //             stops.erase(stops.begin());
    //         }
    //     }
    //     std::vector<NBEdge*>::const_iterator stopIt = stops.begin();
    //     for (const NBEdge* const e : myRoute) {
    //         while (stopIt != stops.end() && e == *stopIt) {
    //             ++stopIt;
    //         }
    //         if (stopIt == stops.end()) {
    //             return true;
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn is_consistent(&self, stops: Vec<Ptr<NBEdge>>) -> bool {
        let mut stops = stops;
        if self.myRoute.is_empty() || stops.is_empty() {
            return true;
        }
        if stops.len() > 1 && stops[0] == stops[stops.len() - 1] {
            // circular route where we don't expect the route edges to occur twice
            if self.myRoute[0] == stops[0] {
                stops.pop();
            } else if self.myRoute[self.myRoute.len() - 1] == stops[stops.len() - 1] {
                stops.remove(0);
            }
        }
        let mut stopIt: usize = 0;
        for e in self.myRoute.iter() {
            while stopIt < stops.len() && *e == stops[stopIt] {
                stopIt += 1;
            }
            if stopIt == stops.len() {
                return true;
            }
        }
        false
    }

    /// C++: `SUMOVehicleClass NBPTLine::getVClass() const` [const]
    // SUMOVehicleClass getVClass() const {
    //         return myVClass;
    //     }
    #[allow(unused_variables)]
    pub fn get_v_class(&self) -> SUMOVehicleClass {
        self.myVClass
    }

    /// C++: `void NBPTLine::replaceStop(std::shared_ptr<NBPTStop>, std::shared_ptr<NBPTStop>)`
    /// replace the given stop
    // void
    // NBPTLine::replaceStop(std::shared_ptr<NBPTStop> oldStop, std::shared_ptr<NBPTStop> newStop) {
    //     for (int i = 0; i < (int)myPTStops.size(); i++) {
    //         if (myPTStops[i] == oldStop) {
    //             myPTStops[i] = newStop;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn replace_stop(&mut self, oldStop: Ptr<NBPTStop>, newStop: Ptr<NBPTStop>) {
        for i in 0..self.myPTStops.len() {
            if self.myPTStops[i] == oldStop {
                self.myPTStops[i] = newStop.clone();
            }
        }
    }

    /// C++: `void NBPTLine::replaceEdge(const std::string &, const std::vector<NBEdge *> &)`
    /// replace the edge with the given edge list
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn replace_edge(&mut self, edgeID: &str, replacement: &Vec<Ptr<NBEdge>>) {
        let oldRoute = self.myRoute.clone();
        self.myRoute.clear();
        for e in oldRoute {
            if e.get_id() == edgeID {
                for e2 in replacement.iter() {
                    if self.myRoute.is_empty() || self.myRoute[self.myRoute.len() - 1] != *e2 {
                        self.myRoute.push(*e2);
                    }
                }
            } else {
                self.myRoute.push(e);
            }
        }
    }

    /// C++: `void NBPTLine::deleteInvalidStops(const NBEdgeCont &, const NBPTStopCont &)`
    /// remove invalid stops from the line
    // void
    // NBPTLine::deleteInvalidStops(const NBEdgeCont& ec, const NBPTStopCont& sc) {
    //     // delete stops that are missing or have no edge
    //     for (auto it = myPTStops.begin(); it != myPTStops.end();) {
    //         std::shared_ptr<NBPTStop> stop = *it;
    //         if (sc.get(stop->getID()) == nullptr ||
    //                 ec.getByID(stop->getEdgeId()) == nullptr) {
    //             WRITE_WARNINGF(TL("Removed invalid stop '%' from line '%'."), stop->getID(), getLineID());
    //             it = myPTStops.erase(it);
    //         } else {
    //             it++;
    //         }
    //
    //     }
    // }
    #[allow(unused_variables)]
    pub fn delete_invalid_stops(&mut self, ec: &NBEdgeCont, sc: &NBPTStopCont) {
        // delete stops that are missing or have no edge
        let mut it: usize = 0;
        while it < self.myPTStops.len() {
            let stop = self.myPTStops[it].clone();
            // same lookup as NBPTStopCont::get(id) == nullptr
            if sc.myPTStops.get(&stop.get_id()).is_none() || ec.get_by_id(stop.get_edge_id()).is_null() {
                write_warning(&format!("Removed invalid stop '{}' from line '{}'.", stop.get_id(), self.get_line_id()));
                self.myPTStops.remove(it);
            } else {
                it += 1;
            }
        }
    }

    /// C++: `void NBPTLine::deleteDuplicateStops()`
    // void
    // NBPTLine::deleteDuplicateStops() {
    //     // delete subsequent stops that belong to the same stopArea
    //     long long int lastAreaID = -1;
    //     std::string lastName = "";
    //     for (auto it = myPTStops.begin(); it != myPTStops.end();) {
    //         std::shared_ptr<NBPTStop> stop = *it;
    //         if (lastAreaID != -1 && stop->getAreaID() == lastAreaID) {
    //             WRITE_WARNINGF(TL("Removed duplicate stop '%' at area '%' from line '%'."), stop->getID(), toString(lastAreaID), getLineID());
    //             it = myPTStops.erase(it);
    //         } else if (lastName != "" && stop->getName() == lastName) {
    //             WRITE_WARNINGF(TL("Removed duplicate stop '%' named '%' from line '%'."), stop->getID(), lastName, getLineID());
    //             it = myPTStops.erase(it);
    //         } else {
    //             it++;
    //         }
    //         lastAreaID = stop->getAreaID();
    //         lastName = stop->getName();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn delete_duplicate_stops(&mut self) {
        // delete subsequent stops that belong to the same stopArea
        let mut lastAreaID: i64 = -1;
        let mut lastName: String = String::new();
        let mut it: usize = 0;
        while it < self.myPTStops.len() {
            let stop = self.myPTStops[it].clone();
            if lastAreaID != -1 && stop.get_area_id() == lastAreaID {
                write_warning(&format!("Removed duplicate stop '{}' at area '{}' from line '{}'.", stop.get_id(), lastAreaID.to_string(), self.get_line_id()));
                self.myPTStops.remove(it);
            } else if lastName != "" && stop.get_name() == lastName {
                write_warning(&format!("Removed duplicate stop '{}' named '{}' from line '{}'.", stop.get_id(), lastName, self.get_line_id()));
                self.myPTStops.remove(it);
            } else {
                it += 1;
            }
            lastAreaID = stop.get_area_id();
            lastName = stop.get_name();
        }
    }

    /// C++: `void NBPTLine::removeInvalidEdges(const NBEdgeCont &)`
    /// remove invalid edges from the line
    // void
    // NBPTLine::removeInvalidEdges(const NBEdgeCont& ec) {
    //     for (int i = 0; i < (int)myRoute.size();) {
    //         const std::pair<NBEdge*, NBEdge*>* split = ec.getSplit(myRoute[i]);
    //         if (split != nullptr) {
    //             myRoute[i] = split->first;
    //             myRoute.insert(myRoute.begin() + i + 1, split->second);
    //         } else if (ec.retrieve(myRoute[i]->getID()) == nullptr) {
    //             myRoute.erase(myRoute.begin() + i);
    //         } else {
    //             i++;
    //         }
    //     }
    //     // validate that all stops are actually on the route
    //     auto ri = myRoute.begin();
    //     for (auto it = myPTStops.begin(); it != myPTStops.end();) {
    //         std::shared_ptr<NBPTStop> stop = *it;
    //         NBEdge* e = ec.retrieve(stop->getEdgeId());
    //         if (e == nullptr) {
    //             WRITE_WARNINGF(TL("Removed stop '%' named '%' from line '%' because edge '%' is missing."),
    //                     stop->getID(), stop->getName(), getLineID(), stop->getEdgeId());
    //             it = myPTStops.erase(it);
    //         } else {
    //             auto riPrev = ri;
    //             ri = std::find(ri, myRoute.end(), e);
    //             if (ri == myRoute.end()) {
    //                 if (std::find(myRoute.begin(), myRoute.end(), e) == myRoute.end()) {
    //                     WRITE_WARNINGF(TL("Stop '%' named '%' from line '%' on edge '%' is not part of the route."),
    //                         stop->getID(), stop->getName(), getLineID(), stop->getEdgeId());
    //                 } else {
    //                     WRITE_WARNINGF(TL("Stop '%' named '%' from line '%' on edge '%' is not part of the route downstream of the previous stop."),
    //                         stop->getID(), stop->getName(), getLineID(), stop->getEdgeId());
    //                 }
    //                 ri = riPrev;
    //             }
    //             it++;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn remove_invalid_edges(&mut self, ec: &NBEdgeCont) {
        let mut i: usize = 0;
        while i < self.myRoute.len() {
            let split = ec.get_split(self.myRoute[i]);
            if !split.is_null() {
                self.myRoute[i] = split.0;
                self.myRoute.insert(i + 1, split.1);
            } else if ec.retrieve(self.myRoute[i].get_id(), false).is_null() {
                self.myRoute.remove(i);
            } else {
                i += 1;
            }
        }
        // validate that all stops are actually on the route
        let mut ri: usize = 0;
        let mut it: usize = 0;
        while it < self.myPTStops.len() {
            let stop = self.myPTStops[it].clone();
            let e = ec.retrieve(stop.get_edge_id(), false);
            if e.is_null() {
                write_warning(&format!("Removed stop '{}' named '{}' from line '{}' because edge '{}' is missing.",
                    stop.get_id(), stop.get_name(), self.get_line_id(), stop.get_edge_id()));
                self.myPTStops.remove(it);
            } else {
                let riPrev = ri;
                match (ri..self.myRoute.len()).find(|&k| self.myRoute[k] == e) {
                    Some(k) => ri = k,
                    None => {
                        if !self.myRoute.contains(&e) {
                            write_warning(&format!("Stop '{}' named '{}' from line '{}' on edge '{}' is not part of the route.",
                                stop.get_id(), stop.get_name(), self.get_line_id(), stop.get_edge_id()));
                        } else {
                            write_warning(&format!("Stop '{}' named '{}' from line '{}' on edge '{}' is not part of the route downstream of the previous stop.",
                                stop.get_id(), stop.get_name(), self.get_line_id(), stop.get_edge_id()));
                        }
                        ri = riPrev;
                    }
                }
                it += 1;
            }
        }
    }

    /// C++: `void NBPTLine::setName(const std::string &)`
    // void setName(const std::string& name) {
    //         myName = name;
    //     }
    #[allow(unused_variables)]
    pub fn set_name(&mut self, name: &str) {
        self.myName = name.to_string();
    }

    /// C++: `void NBPTLine::setRef(const std::string &)`
    // void setRef(const std::string& line) {
    //         myRef = line;
    //     }
    #[allow(unused_variables)]
    pub fn set_ref(&mut self, line: &str) {
        self.myRef = line.to_string();
    }

    /// C++: `void NBPTLine::setPeriod(int)`
    // void setPeriod(int intervalS) {
    //         myInterval = intervalS / 60;
    //     }
    #[allow(unused_variables)]
    pub fn set_period(&mut self, intervalS: i32) {
        self.myInterval = intervalS / 60;
    }

    /// C++: `const std::vector<std::string> & NBPTLine::getWays() const` [const]
    // inline const std::vector<std::string>& getWays() const {
    //         return myWays;
    //     }
    #[allow(unused_variables)]
    pub fn get_ways(&self) -> &Vec<String> {
        &self.myWays
    }

    /// C++: `const std::vector<long long> * NBPTLine::getWayNodes(std::string)`
    // const std::vector<long long int>*
    // NBPTLine::getWayNodes(std::string wayId) {
    //     if (myWayNodes.find(wayId) != myWayNodes.end()) {
    //         return &myWayNodes[wayId];
    //     }
    //     return nullptr;
    // }
    #[allow(unused_variables)]
    pub fn get_way_nodes(&mut self, wayId: String) -> Ptr<Vec<i64>> {
        if let Some(v) = self.myWayNodes.get_mut(&wayId) {
            return Ptr::from_mut(v);
        }
        Ptr::null()
    }

    /// C++: `const EdgeVector & NBPTLine::getEdges() const` [const]
    // const EdgeVector& getEdges() const {
    //         return myRoute;
    //     }
    #[allow(unused_variables)]
    pub fn get_edges(&self) -> &EdgeVector {
        &self.myRoute
    }

    /// C++: `void NBPTLine::setEdges(const std::vector<NBEdge *> &)`
    // void
    // NBPTLine::setEdges(const std::vector<NBEdge*>& edges) {
    //     myRoute = edges;
    //     // ensure permissions
    //     for (NBEdge* e : edges) {
    //         SVCPermissions permissions = e->getPermissions();
    //         if ((permissions & myVClass) != myVClass) {
    //             SVCPermissions nVuln = ~(SVC_PEDESTRIAN | SVC_BICYCLE);
    //             if (permissions != 0 && (permissions & nVuln) == 0) {
    //                 // this is a footpath or sidewalk. Add another lane
    //                 e->addRestrictedLane(SUMO_const_laneWidth, myVClass);
    //             } else {
    //                 // add permissions to the rightmost lane that is not exclusively used for pedestrians / bicycles
    //                 for (int i = 0; i < (int)e->getNumLanes(); i++) {
    //                     if ((e->getPermissions(i) & nVuln) != 0) {
    //                         e->allowVehicleClass(i, myVClass);
    //                         break;
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_edges(&mut self, edges: &Vec<Ptr<NBEdge>>) {
        self.myRoute = edges.clone();
        // ensure permissions
        for e in edges.iter() {
            let permissions = e.get_permissions(-1);
            if (permissions & self.myVClass) != self.myVClass {
                let nVuln: SVCPermissions = !(SVC_PEDESTRIAN | SVC_BICYCLE);
                if permissions != 0 && (permissions & nVuln) == 0 {
                    // this is a footpath or sidewalk. Add another lane
                    e.m().add_restricted_lane(SUMO_CONST_LANE_WIDTH, self.myVClass);
                } else {
                    // add permissions to the rightmost lane that is not exclusively used for pedestrians / bicycles
                    for i in 0..e.get_num_lanes() {
                        if (e.get_permissions(i) & nVuln) != 0 {
                            e.m().allow_vehicle_class(i, self.myVClass);
                            break;
                        }
                    }
                }
            }
        }
    }

    /// C++: `const std::vector<NBEdge *> & NBPTLine::getRoute() const` [const]
    // const std::vector<NBEdge*>&
    // NBPTLine::getRoute() const {
    //     return myRoute;
    // }
    #[allow(unused_variables)]
    pub fn get_route(&self) -> &Vec<Ptr<NBEdge>> {
        &self.myRoute
    }

}

/// get stop edges and stop ids and directional validity
/// C++ class `NBPTLine::PTStopInfo` (netbuild/NBPTLine.h)
pub struct NBPTLine_PTStopInfo {
    pub edge: Ptr<NBEdge>,
    pub stopID: String,
    pub pos: f64,
    pub revised: bool,
}

impl NBPTLine_PTStopInfo {
    /// C++: `void PTStopInfo::PTStopInfo(NBEdge *, const std::string &, double, bool)`
    // PTStopInfo(NBEdge* _edge, const std::string& _stopID, double _pos, bool _revised):
    //             edge(_edge), stopID(_stopID), pos(_pos), revised(_revised) {}
    #[allow(unused_variables)]
    pub fn new(_edge: Ptr<NBEdge>, _stopID: &str, _pos: f64, _revised: bool) -> Self {
        NBPTLine_PTStopInfo {
            edge: _edge,
            stopID: _stopID.to_string(),
            pos: _pos,
            revised: _revised,
        }
    }

}

