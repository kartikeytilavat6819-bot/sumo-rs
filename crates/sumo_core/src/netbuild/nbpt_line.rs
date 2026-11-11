//! Port of netbuild/NBPTLine.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `NBPTLine` (netbuild/NBPTLine.h)
pub struct NBPTLine {
    pub myName: String,
    pub myType: String,
    pub myPTStops: Vec<Rc<NBPTStop>>,
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
        todo!("port NBPTLine::NBPTLine")
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
    pub fn add_pt_stop(&mut self, pStop: Rc<NBPTStop>) {
        todo!("port NBPTLine::addPTStop")
    }

    /// C++: `const std::string & NBPTLine::getLineID() const` [const]
    // const std::string& getLineID() const {
    //         return myPTLineId;
    //     }
    #[allow(unused_variables)]
    pub fn get_line_id(&self) -> &str {
        todo!("port NBPTLine::getLineID")
    }

    /// C++: `const std::string & NBPTLine::getName() const` [const]
    // const std::string& getName() const {
    //         return myName;
    //     }
    #[allow(unused_variables)]
    pub fn get_name(&self) -> &str {
        todo!("port NBPTLine::getName")
    }

    /// C++: `const std::string & NBPTLine::getType() const` [const]
    // const std::string& getType() const {
    //         return myType;
    //     }
    #[allow(unused_variables)]
    pub fn get_type(&self) -> &str {
        todo!("port NBPTLine::getType")
    }

    /// C++: `const std::vector<std::shared_ptr<NBPTStop>> & NBPTLine::getStops()`
    // const std::vector<std::shared_ptr<NBPTStop> >&
    // NBPTLine::getStops() {
    //     return myPTStops;
    // }
    #[allow(unused_variables)]
    pub fn get_stops(&mut self) -> &Vec<Rc<NBPTStop>> {
        todo!("port NBPTLine::getStops")
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
        todo!("port NBPTLine::write")
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
        todo!("port NBPTLine::addWayNode")
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
        todo!("port NBPTLine::setNumOfStops")
    }

    /// C++: `const std::string & NBPTLine::getRef() const` [const]
    /// get line reference (not unique)
    // const std::string& getRef() const {
    //         return myRef;
    //     }
    #[allow(unused_variables)]
    pub fn get_ref(&self) -> &str {
        todo!("port NBPTLine::getRef")
    }

    /// C++: `void NBPTLine::replaceStops(std::vector<std::shared_ptr<NBPTStop>>)`
    // void replaceStops(std::vector<std::shared_ptr<NBPTStop> > stops) {
    //         myPTStops = stops;
    //     }
    #[allow(unused_variables)]
    pub fn replace_stops(&mut self, stops: Vec<Rc<NBPTStop>>) {
        todo!("port NBPTLine::replaceStops")
    }

    /// C++: `void NBPTLine::setRevised(std::vector<bool>)`
    // void setRevised(std::vector<bool> stopsRevised) {
    //         myStopsRevised = stopsRevised;
    //     }
    #[allow(unused_variables)]
    pub fn set_revised(&mut self, stopsRevised: Vec<bool>) {
        todo!("port NBPTLine::setRevised")
    }

    /// C++: `std::vector<PTStopInfo> NBPTLine::getStopEdges(const NBEdgeCont &) const` [const]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_stop_edges(&self, ec: &NBEdgeCont) -> Vec<NBPTLine_PTStopInfo> {
        todo!("port NBPTLine::getStopEdges")
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
        todo!("port NBPTLine::getRouteStart")
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
        todo!("port NBPTLine::getRouteEnd")
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
        todo!("port NBPTLine::isConsistent")
    }

    /// C++: `SUMOVehicleClass NBPTLine::getVClass() const` [const]
    // SUMOVehicleClass getVClass() const {
    //         return myVClass;
    //     }
    #[allow(unused_variables)]
    pub fn get_v_class(&self) -> SUMOVehicleClass {
        todo!("port NBPTLine::getVClass")
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
    pub fn replace_stop(&mut self, oldStop: Rc<NBPTStop>, newStop: Rc<NBPTStop>) {
        todo!("port NBPTLine::replaceStop")
    }

    /// C++: `void NBPTLine::replaceEdge(const std::string &, const std::vector<NBEdge *> &)`
    /// replace the edge with the given edge list
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn replace_edge(&mut self, edgeID: &str, replacement: &Vec<Ptr<NBEdge>>) {
        todo!("port NBPTLine::replaceEdge")
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
        todo!("port NBPTLine::deleteInvalidStops")
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
        todo!("port NBPTLine::deleteDuplicateStops")
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
        todo!("port NBPTLine::removeInvalidEdges")
    }

    /// C++: `void NBPTLine::setName(const std::string &)`
    // void setName(const std::string& name) {
    //         myName = name;
    //     }
    #[allow(unused_variables)]
    pub fn set_name(&mut self, name: &str) {
        todo!("port NBPTLine::setName")
    }

    /// C++: `void NBPTLine::setRef(const std::string &)`
    // void setRef(const std::string& line) {
    //         myRef = line;
    //     }
    #[allow(unused_variables)]
    pub fn set_ref(&mut self, line: &str) {
        todo!("port NBPTLine::setRef")
    }

    /// C++: `void NBPTLine::setPeriod(int)`
    // void setPeriod(int intervalS) {
    //         myInterval = intervalS / 60;
    //     }
    #[allow(unused_variables)]
    pub fn set_period(&mut self, intervalS: i32) {
        todo!("port NBPTLine::setPeriod")
    }

    /// C++: `const std::vector<std::string> & NBPTLine::getWays() const` [const]
    // inline const std::vector<std::string>& getWays() const {
    //         return myWays;
    //     }
    #[allow(unused_variables)]
    pub fn get_ways(&self) -> &Vec<String> {
        todo!("port NBPTLine::getWays")
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
        todo!("port NBPTLine::getWayNodes")
    }

    /// C++: `const EdgeVector & NBPTLine::getEdges() const` [const]
    // const EdgeVector& getEdges() const {
    //         return myRoute;
    //     }
    #[allow(unused_variables)]
    pub fn get_edges(&self) -> &EdgeVector {
        todo!("port NBPTLine::getEdges")
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
        todo!("port NBPTLine::setEdges")
    }

    /// C++: `const std::vector<NBEdge *> & NBPTLine::getRoute() const` [const]
    // const std::vector<NBEdge*>&
    // NBPTLine::getRoute() const {
    //     return myRoute;
    // }
    #[allow(unused_variables)]
    pub fn get_route(&self) -> &Vec<Ptr<NBEdge>> {
        todo!("port NBPTLine::getRoute")
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
        todo!("port PTStopInfo::PTStopInfo")
    }

}

