//! Port of netbuild/NBPTStop.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

// C++ MAX2/MIN2 (utils/common/StdDefs.h): same comparison order as the macros
fn max2(a: f64, b: f64) -> f64 {
    if a > b { a } else { b }
}
fn min2(a: f64, b: f64) -> f64 {
    if a < b { a } else { b }
}

/// The representation of a single pt stop
/// C++ class `NBPTStop` (netbuild/NBPTStop.h)
pub struct NBPTStop {
    /// C++ base class `Parameterised`
    pub base: Parameterised,
    pub myTag: SumoXMLTag,
    pub myPTStopId: String,
    pub myPosition: Position,
    pub myEdgeId: String,
    pub myAdditionalEdgeCandidates: BTreeMap<String, String>,
    pub myOrigEdgeId: String,
    pub myPTStopLength: f64,
    pub myName: String,
    pub myParkingLength: f64,
    pub myColor: RGBColor,
    pub myLaneId: String,
    pub myPermissions: SVCPermissions,
    pub myStartPos: f64,
    pub myEndPos: f64,
    /// laneId, lanePos, accessLength
    pub myAccesses: Vec<(String, f64, f64)>,
    /// list of public transport lines (for displaying)
    pub myLines: Vec<String>,
    pub myBidiStop: std::rc::Weak<NBPTStop>,
    /// whether the stop was not part of the road network and must be mapped
    pub myIsLoose: bool,
    /// whether this stop was build from a platform position
    pub myIsPlatform: bool,
    pub myPlatformCands: Vec<NBPTPlatform>,
    pub myIsMultipleStopPositions: bool,
    pub myAreaID: i64,
    pub myGivenStartPos: f64,
}

impl std::ops::Deref for NBPTStop {
    type Target = Parameterised;
    fn deref(&self) -> &Parameterised {
        &self.base
    }
}
impl std::ops::DerefMut for NBPTStop {
    fn deref_mut(&mut self) -> &mut Parameterised {
        &mut self.base
    }
}

impl NBPTStop {
    /// C++: `void NBPTStop::NBPTStop(SumoXMLTag, std::string, Position, std::string, std::string, double, std::string, SVCPermissions, double, const RGBColor, double)`
    /// Constructor
    // NBPTStop::NBPTStop(SumoXMLTag tag, std::string ptStopId, Position position, std::string edgeId, std::string origEdgeId, double length,
    //                    std::string name, SVCPermissions svcPermissions, double parkingLength, const RGBColor color, double givenStartPos) :
    //     myTag(tag),
    //     myPTStopId(ptStopId),
    //     myPosition(position),
    //     myEdgeId(edgeId),
    //     myOrigEdgeId(origEdgeId),
    //     myPTStopLength(length),
    //     myName(name),
    //     myParkingLength(parkingLength),
    //     myColor(color),
    //     myPermissions(svcPermissions),
    //     myStartPos(0),
    //     myEndPos(0),
    //     myBidiStop(std::weak_ptr<NBPTStop>()),
    //     myIsLoose(origEdgeId == ""),
    //     myIsPlatform(false),
    //     myIsMultipleStopPositions(false),
    //     myAreaID(-1),
    //     myGivenStartPos(givenStartPos) {
    // }
    #[allow(unused_variables)]
    pub fn new(tag: SumoXMLTag, ptStopId: String, position: Position, edgeId: String, origEdgeId: String, length: f64, name: String, svcPermissions: SVCPermissions, parkingLength: f64, color: RGBColor, givenStartPos: f64) -> Self {
                let is_loose = origEdgeId.is_empty();
        Self {
            base: Parameterised::new(),
            myTag: tag,
            myPTStopId: ptStopId,
            myPosition: position,
            myEdgeId: edgeId,
            myAdditionalEdgeCandidates: BTreeMap::new(),
            myOrigEdgeId: origEdgeId,
            myPTStopLength: length,
            myName: name,
            myParkingLength: parkingLength,
            myColor: color,
            myLaneId: String::new(),
            myPermissions: svcPermissions,
            myStartPos: 0.0,
            myEndPos: 0.0,
            myAccesses: Vec::new(),
            myLines: Vec::new(),
            myBidiStop: std::rc::Weak::new(),
            myIsLoose: is_loose,
            myIsPlatform: false,
            myPlatformCands: Vec::new(),
            myIsMultipleStopPositions: false,
            myAreaID: -1,
            myGivenStartPos: givenStartPos,
        }
    }

    /// C++: `void NBPTStop::~NBPTStop() noexcept` [virtual]
    /// Destructor
    // virtual ~NBPTStop() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
                // C++ virtual destructor with an empty body; nothing to release here.
    }

    /// C++: `std::string NBPTStop::getID() const` [const]
    // std::string
    // NBPTStop::getID() const {
    //     return myPTStopId;
    // }
    #[allow(unused_variables)]
    pub fn get_id(&self) -> String {
                self.myPTStopId.clone()
    }

    /// C++: `SumoXMLTag NBPTStop::getElement() const` [const]
    // SumoXMLTag getElement() const {
    //         return myTag;
    //     }
    #[allow(unused_variables)]
    pub fn get_element(&self) -> SumoXMLTag {
                self.myTag
    }

    /// C++: `const std::string & NBPTStop::getEdgeId() const` [const]
    // const std::string&
    // NBPTStop::getEdgeId() const {
    //     return myEdgeId;
    // }
    #[allow(unused_variables)]
    pub fn get_edge_id(&self) -> &str {
                &self.myEdgeId
    }

    /// C++: `const std::string & NBPTStop::getLaneId() const` [const]
    // const std::string& getLaneId() const {
    //         return myLaneId;
    //     }
    #[allow(unused_variables)]
    pub fn get_lane_id(&self) -> &str {
                &self.myLaneId
    }

    /// C++: `const std::string NBPTStop::getOrigEdgeId() const` [const]
    // const std::string
    // NBPTStop::getOrigEdgeId() const {
    //     return myOrigEdgeId;
    // }
    #[allow(unused_variables)]
    pub fn get_orig_edge_id(&self) -> String {
                self.myOrigEdgeId.clone()
    }

    /// C++: `const std::string NBPTStop::getName() const` [const]
    // const std::string
    // NBPTStop::getName() const {
    //     return myName;
    // }
    #[allow(unused_variables)]
    pub fn get_name(&self) -> String {
                self.myName.clone()
    }

    /// C++: `const Position & NBPTStop::getPosition() const` [const]
    // const Position&
    // NBPTStop::getPosition() const {
    //     return myPosition;
    // }
    #[allow(unused_variables)]
    pub fn get_position(&self) -> &Position {
                &self.myPosition
    }

    /// C++: `SVCPermissions NBPTStop::getPermissions() const` [const]
    // SVCPermissions
    // NBPTStop::getPermissions() const {
    //     return myPermissions;
    // }
    #[allow(unused_variables)]
    pub fn get_permissions(&self) -> SVCPermissions {
                self.myPermissions
    }

    /// C++: `long long NBPTStop::getAreaID() const` [const]
    // long long int getAreaID() const {
    //         return myAreaID;
    //     }
    #[allow(unused_variables)]
    pub fn get_area_id(&self) -> i64 {
                self.myAreaID
    }

    /// C++: `void NBPTStop::write(OutputDevice &)`
    // void
    // NBPTStop::write(OutputDevice& device) {
    //     device.openTag(myTag);
    //     device.writeAttr(SUMO_ATTR_ID, myPTStopId);
    //     if (!myName.empty()) {
    //         device.writeAttr(SUMO_ATTR_NAME, StringUtils::escapeXML(myName));
    //     }
    //     device.writeAttr(SUMO_ATTR_LANE, myLaneId);
    //     device.writeAttr(SUMO_ATTR_STARTPOS, myStartPos);
    //     device.writeAttr(SUMO_ATTR_ENDPOS, myEndPos);
    //     device.writeAttr(SUMO_ATTR_FRIENDLY_POS, "true");
    //     if (myLines.size() > 0) {
    //         device.writeAttr(SUMO_ATTR_LINES, toString(myLines));
    //     }
    //     if (myParkingLength > 0) {
    //         device.writeAttr(SUMO_ATTR_PARKING_LENGTH, myParkingLength);
    //     }
    //     if (myColor.isValid()) {
    //         device.writeAttr(SUMO_ATTR_COLOR, myColor);
    //     }
    //     if (!myAccesses.empty()) {
    //         std::sort(myAccesses.begin(), myAccesses.end());
    //         for (auto tuple : myAccesses) {
    //             device.openTag(SUMO_TAG_ACCESS);
    //             device.writeAttr(SUMO_ATTR_LANE, std::get<0>(tuple));
    //             device.writeAttr(SUMO_ATTR_POSITION, std::get<1>(tuple));
    //             device.writeAttr(SUMO_ATTR_LENGTH, std::get<2>(tuple));
    //             device.writeAttr(SUMO_ATTR_FRIENDLY_POS, true);
    //             device.closeTag();
    //         }
    //     }
    //     writeParams(device);
    //     device.closeTag();
    // }
    #[allow(unused_variables)]
    pub fn write(&mut self, device: &mut OutputDevice) {
                device.open_tag_id(self.myTag);
        device.write_attr(SUMO_ATTR_ID, &self.myPTStopId);
        if !self.myName.is_empty() {
            device.write_attr(SUMO_ATTR_NAME, &utils::common::string_utils::escape_xml(&self.myName, false));
        }
        device.write_attr(SUMO_ATTR_LANE, &self.myLaneId);
        device.write_attr(SUMO_ATTR_STARTPOS, &self.myStartPos);
        device.write_attr(SUMO_ATTR_ENDPOS, &self.myEndPos);
        device.write_attr(SUMO_ATTR_FRIENDLY_POS, "true");
        if self.myLines.len() > 0 {
            // toString(std::vector<std::string>) joins with spaces
            device.write_attr(SUMO_ATTR_LINES, &self.myLines.join(" "));
        }
        if self.myParkingLength > 0.0 {
            device.write_attr(SUMO_ATTR_PARKING_LENGTH, &self.myParkingLength);
        }
        if self.myColor.is_valid() {
            device.write_attr(SUMO_ATTR_COLOR, &self.myColor.to_string());
        }
        if !self.myAccesses.is_empty() {
            self.myAccesses.sort_by(|a, b| {
                a.0.cmp(&b.0)
                    .then(a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
                    .then(a.2.partial_cmp(&b.2).unwrap_or(std::cmp::Ordering::Equal))
            });
            for (lane, pos, len) in &self.myAccesses {
                device.open_tag_id(SUMO_TAG_ACCESS);
                device.write_attr(SUMO_ATTR_LANE, lane);
                device.write_attr(SUMO_ATTR_POSITION, pos);
                device.write_attr(SUMO_ATTR_LENGTH, len);
                device.write_attr(SUMO_ATTR_FRIENDLY_POS, &true);
                device.close_tag("");
            }
        }
        // Parameterised::writeParams(device), inlined (not ported as a Rust method yet)
        for (key, value) in self.base.get_parameters_map() {
            device.open_tag_id(SUMO_TAG_PARAM);
            device.write_attr_escaped(SUMO_ATTR_KEY, key, true);
            device.write_attr_escaped(SUMO_ATTR_VALUE, value, true);
            device.close_tag("");
        }
        device.close_tag("");
    }

    /// C++: `void NBPTStop::reshiftPosition(const double, const double)`
    // void
    // NBPTStop::reshiftPosition(const double offsetX, const double offsetY) {
    //     myPosition.add(offsetX, offsetY, 0);
    //     for (NBPTPlatform& platformCand : myPlatformCands) {
    //         platformCand.reshiftPosition(offsetX, offsetY);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn reshift_position(&mut self, offsetX: f64, offsetY: f64) {
                self.myPosition.add_xyz(offsetX, offsetY, 0.0);
        for platformCand in self.myPlatformCands.iter_mut() {
            platformCand.reshift_position(offsetX, offsetY);
        }
    }

    /// C++: `const std::vector<NBPTPlatform> & NBPTStop::getPlatformCands()`
    // const std::vector<NBPTPlatform>&
    // NBPTStop::getPlatformCands() {
    //     return myPlatformCands;
    // }
    #[allow(unused_variables)]
    pub fn get_platform_cands(&mut self) -> &Vec<NBPTPlatform> {
                &self.myPlatformCands
    }

    /// C++: `bool NBPTStop::getIsMultipleStopPositions() const` [const]
    // bool
    // NBPTStop::getIsMultipleStopPositions() const {
    //     return myIsMultipleStopPositions;
    // }
    #[allow(unused_variables)]
    pub fn get_is_multiple_stop_positions(&self) -> bool {
                self.myIsMultipleStopPositions
    }

    /// C++: `void NBPTStop::setIsMultipleStopPositions(bool, long long)`
    // void
    // NBPTStop::setIsMultipleStopPositions(bool multipleStopPositions, long long int areaID) {
    //     myIsMultipleStopPositions = multipleStopPositions;
    //     myAreaID = areaID;
    // }
    #[allow(unused_variables)]
    pub fn set_is_multiple_stop_positions(&mut self, multipleStopPositions: bool, areaID: i64) {
                self.myIsMultipleStopPositions = multipleStopPositions;
        self.myAreaID = areaID;
    }

    /// C++: `double NBPTStop::getLength() const` [const]
    // double
    // NBPTStop::getLength() const {
    //     return myPTStopLength;
    // }
    #[allow(unused_variables)]
    pub fn get_length(&self) -> f64 {
                self.myPTStopLength
    }

    /// C++: `bool NBPTStop::setEdgeId(std::string, const NBEdgeCont &)`
    // bool
    // NBPTStop::setEdgeId(std::string edgeId, const NBEdgeCont& ec) {
    //     myEdgeId = edgeId;
    //     return findLaneAndComputeBusStopExtent(ec);
    // }
    #[allow(unused_variables)]
    pub fn set_edge_id(&mut self, edgeId: String, ec: &NBEdgeCont) -> bool {
                self.myEdgeId = edgeId;
        self.find_lane_and_compute_bus_stop_extent(ec)
    }

    /// C++: `void NBPTStop::registerAdditionalEdge(std::string, std::string)`
    // void
    // NBPTStop::registerAdditionalEdge(std::string wayId, std::string edgeId) {
    //     myAdditionalEdgeCandidates[wayId] = edgeId;
    // }
    #[allow(unused_variables)]
    pub fn register_additional_edge(&mut self, wayId: String, edgeId: String) {
                self.myAdditionalEdgeCandidates.insert(wayId, edgeId);
    }

    /// C++: `void NBPTStop::addPlatformCand(NBPTPlatform)`
    // void
    // NBPTStop::addPlatformCand(NBPTPlatform platform) {
    //     myPlatformCands.push_back(platform);
    // }
    #[allow(unused_variables)]
    pub fn add_platform_cand(&mut self, platform: NBPTPlatform) {
                self.myPlatformCands.push(platform);
    }

    /// C++: `bool NBPTStop::findLaneAndComputeBusStopExtent(const NBEdgeCont &)`
    // bool
    // NBPTStop::findLaneAndComputeBusStopExtent(const NBEdgeCont& ec) {
    //     NBEdge* edge = ec.getByID(myEdgeId);
    //     return findLaneAndComputeBusStopExtent(edge);
    // }
    #[allow(unused_variables)]
    pub fn find_lane_and_compute_bus_stop_extent(&mut self, ec: &NBEdgeCont) -> bool {
                let edge = ec.get_by_id(&self.myEdgeId);
        self.find_lane_and_compute_bus_stop_extent_2(edge)
    }

    /// C++: `bool NBPTStop::findLaneAndComputeBusStopExtent(const NBEdge *)`
    // bool
    // NBPTStop::findLaneAndComputeBusStopExtent(const NBEdge* edge) {
    //     if (edge != nullptr) {
    //         myEdgeId = edge->getID();
    //         int laneNr = -1;
    //         for (const auto& it : edge->getLanes()) {
    //             if ((it.permissions & getPermissions()) == getPermissions()) {
    //                 ++laneNr;
    //                 break;
    //             }
    //             laneNr++;
    //         }
    //         if (laneNr != -1) {
    //             myLaneId = edge->getLaneID(laneNr);
    //             const PositionVector& shape = edge->getLaneShape(laneNr);
    //             double offset = shape.nearest_offset_to_point2D(getPosition(), false);
    //             const double edgeLength = edge->getFinalLength();
    //             offset *= edgeLength / shape.length2D();
    //             if (wasLoaded()) {
    //                 myStartPos = myGivenStartPos;
    //                 myEndPos = myStartPos + myPTStopLength;
    //             } else {
    //                 myStartPos = MAX2(0.0, offset - myPTStopLength / 2.);
    //                 myEndPos = MIN2(myStartPos + myPTStopLength, edgeLength);
    //                 double missing = myPTStopLength - (myEndPos - myStartPos);
    //                 if (missing > 0) {
    //                     myStartPos = MAX2(0.0, myStartPos - missing);
    //                 }
    //             }
    //             return true;
    //         }
    //     }
    //     return myEdgeId == ""; // loose stop. Try later when processing lines
    // }
    #[allow(unused_variables)]
    pub fn find_lane_and_compute_bus_stop_extent_2(&mut self, edge: Ptr<NBEdge>) -> bool {
                if !edge.is_null() {
            self.myEdgeId = edge.get_id().to_string();
            // same loop as C++: laneNr ends at the last lane when none matches (kept on purpose)
            let mut laneNr: i32 = -1;
            for it in edge.get_lanes() {
                if (it.permissions & self.get_permissions()) == self.get_permissions() {
                    laneNr += 1;
                    break;
                }
                laneNr += 1;
            }
            if laneNr != -1 {
                self.myLaneId = edge.get_lane_id(laneNr);
                let shape = edge.get_lane_shape(laneNr);
                let mut offset = shape.nearest_offset_to_point2d(self.get_position(), false);
                let edgeLength = edge.get_final_length();
                offset *= edgeLength / shape.length2d();
                if self.was_loaded() {
                    self.myStartPos = self.myGivenStartPos;
                    self.myEndPos = self.myStartPos + self.myPTStopLength;
                } else {
                    self.myStartPos = max2(0.0, offset - self.myPTStopLength / 2.0);
                    self.myEndPos = min2(self.myStartPos + self.myPTStopLength, edgeLength);
                    let missing = self.myPTStopLength - (self.myEndPos - self.myStartPos);
                    if missing > 0.0 {
                        self.myStartPos = max2(0.0, self.myStartPos - missing);
                    }
                }
                return true;
            }
        }
        // loose stop: try again later when processing lines
        self.myEdgeId.is_empty()
    }

    /// C++: `void NBPTStop::setPTStopId(std::string)`
    // void setPTStopId(std::string id) {
    //         myPTStopId = id;
    //     }
    #[allow(unused_variables)]
    pub fn set_pt_stop_id(&mut self, id: String) {
                self.myPTStopId = id;
    }

    /// C++: `bool NBPTStop::wasLoaded() const` [const]
    // bool wasLoaded() const {
    //         return myGivenStartPos >= 0;
    //     }
    #[allow(unused_variables)]
    pub fn was_loaded(&self) -> bool {
                self.myGivenStartPos >= 0.0
    }

    /// C++: `void NBPTStop::resetLoaded()`
    // void resetLoaded() {
    //         myGivenStartPos = -1;
    //     }
    #[allow(unused_variables)]
    pub fn reset_loaded(&mut self) {
                self.myGivenStartPos = -1.0;
    }

    /// C++: `void NBPTStop::setIsPlatform()`
    // void setIsPlatform() {
    //         myIsPlatform = true;
    //     }
    #[allow(unused_variables)]
    pub fn set_is_platform(&mut self) {
                self.myIsPlatform = true;
    }

    /// C++: `bool NBPTStop::isPlatform() const` [const]
    // bool isPlatform() const {
    //         return myIsPlatform;
    //     }
    #[allow(unused_variables)]
    pub fn is_platform(&self) -> bool {
                self.myIsPlatform
    }

    /// C++: `void NBPTStop::addAccess(std::string, double, double)`
    // void
    // NBPTStop::addAccess(std::string laneID, double offset, double length) {
    //     const std::string newEdgeID = SUMOXMLDefinitions::getEdgeIDFromLane(laneID);
    //     // avoid duplicate access
    //     for (auto it = myAccesses.begin(); it != myAccesses.end();) {
    //         if (SUMOXMLDefinitions::getEdgeIDFromLane(std::get<0>(*it)) == newEdgeID) {
    //             it = myAccesses.erase(it);
    //         } else {
    //             it++;
    //         }
    //     }
    //     myAccesses.push_back(std::make_tuple(laneID, offset, length));
    // }
    #[allow(unused_variables)]
    pub fn add_access(&mut self, laneID: String, offset: f64, length: f64) {
                let newEdgeID = utils::xml::sumo_xml_definitions::get_edge_id_from_lane(&laneID);
        // avoid duplicate access
        self.myAccesses.retain(|it| utils::xml::sumo_xml_definitions::get_edge_id_from_lane(&it.0) != newEdgeID);
        self.myAccesses.push((laneID, offset, length));
    }

    /// C++: `void NBPTStop::clearAccess()`
    /// remove all access definitions
    // void
    // NBPTStop::clearAccess() {
    //     myAccesses.clear();
    // }
    #[allow(unused_variables)]
    pub fn clear_access(&mut self) {
                self.myAccesses.clear();
    }

    /// C++: `void NBPTStop::addLine(const std::string &)`
    /// register line that services this stop (for displaying)
    // void
    // NBPTStop::addLine(const std::string& line) {
    //     const std::string l = StringUtils::escapeXML(line);
    //     if (std::find(myLines.begin(), myLines.end(), l) == myLines.end()) {
    //         myLines.push_back(l);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_line(&mut self, line: &str) {
                let l = utils::common::string_utils::escape_xml(line, false);
        if !self.myLines.contains(&l) {
            self.myLines.push(l);
        }
    }

    /// C++: `void NBPTStop::setBidiStop(std::shared_ptr<NBPTStop>)`
    // void setBidiStop(std::shared_ptr<NBPTStop> bidiStop) {
    //         myBidiStop = bidiStop;
    //     }
    #[allow(unused_variables)]
    pub fn set_bidi_stop(&mut self, bidiStop: Rc<NBPTStop>) {
                self.myBidiStop = Rc::downgrade(&bidiStop);
    }

    /// C++: `std::shared_ptr<NBPTStop> NBPTStop::getBidiStop() const` [const]
    // std::shared_ptr<NBPTStop> getBidiStop() const {
    //         return myBidiStop.lock();
    //     }
    #[allow(unused_variables)]
    pub fn get_bidi_stop(&self) -> Option<Rc<NBPTStop>> {
                self.myBidiStop.upgrade()
    }

    /// C++: `bool NBPTStop::isLoose() const` [const]
    // bool isLoose() const {
    //         return myIsLoose;
    //     }
    #[allow(unused_variables)]
    pub fn is_loose(&self) -> bool {
                self.myIsLoose
    }

    /// C++: `double NBPTStop::getStartPos() const` [const]
    // double getStartPos() const {
    //         return myStartPos;
    //     }
    #[allow(unused_variables)]
    pub fn get_start_pos(&self) -> f64 {
                self.myStartPos
    }

    /// C++: `double NBPTStop::getEndPos() const` [const]
    // double getEndPos() const {
    //         return myEndPos;
    //     }
    #[allow(unused_variables)]
    pub fn get_end_pos(&self) -> f64 {
                self.myEndPos
    }

    /// C++: `const std::vector<std::string> & NBPTStop::getLines() const` [const]
    // const std::vector<std::string>& getLines() const {
    //         return myLines;
    //     }
    #[allow(unused_variables)]
    pub fn get_lines(&self) -> &Vec<String> {
                &self.myLines
    }

    /// C++: `void NBPTStop::mirrorX()`
    /// mirror coordinates along the x-axis
    // void
    // NBPTStop::mirrorX() {
    //     myPosition.mul(1, -1);
    // }
    #[allow(unused_variables)]
    pub fn mirror_x(&mut self) {
                self.myPosition.mul_xy(1.0, -1.0);
    }

    /// C++: `bool NBPTStop::replaceEdge(const std::string &, const std::vector<NBEdge *> &)`
    /// replace the stop edge with the closest edge on the given edge list in all stops
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn replace_edge(&mut self, edgeID: &str, replacement: &Vec<Ptr<NBEdge>>) -> bool {
                if self.myEdgeId == edgeID {
            // find best edge among replacement edges
            let mut bestDist = f64::MAX;
            let mut bestEdge: Ptr<NBEdge> = Ptr::null();
            for cand in replacement {
                if self.myPermissions == 0 || (cand.get_permissions(-1) & self.myPermissions) != 0 {
                    let dist = cand.get_geometry().distance2d(&self.myPosition, false) + max2(0.0, self.myPTStopLength - cand.get_loaded_length());
                    if dist < bestDist {
                        bestDist = dist;
                        bestEdge = *cand;
                    }
                }
            }
            if !bestEdge.is_null() {
                if (bestEdge.get_permissions(-1) & SVC_PEDESTRIAN) != 0 {
                    // no need for access
                    self.clear_access();
                }
                return self.find_lane_and_compute_bus_stop_extent_2(bestEdge);
            }
            return false;
        }
        true
    }

    /// C++: `const std::map<std::string, std::string> & NBPTStop::getAdditionalEdgeCandidates() const` [const]
    // const std::map<std::string, std::string>& getAdditionalEdgeCandidates() const {
    //         return myAdditionalEdgeCandidates;
    //     }
    #[allow(unused_variables)]
    pub fn get_additional_edge_candidates(&self) -> &BTreeMap<String, String> {
                &self.myAdditionalEdgeCandidates
    }

    /// C++: `void NBPTStop::setOrigEdgeId(const std::string &)`
    // void setOrigEdgeId(const std::string& origEdgeId) {
    //         myOrigEdgeId = origEdgeId;
    //     }
    #[allow(unused_variables)]
    pub fn set_orig_edge_id(&mut self, origEdgeId: &str) {
                self.myOrigEdgeId = origEdgeId.to_string();
    }

    /// C++: `void NBPTStop::setPTStopLength(double)`
    // void setPTStopLength(double ptStopLength) {
    //         myPTStopLength = ptStopLength;
    //     }
    #[allow(unused_variables)]
    pub fn set_pt_stop_length(&mut self, ptStopLength: f64) {
                self.myPTStopLength = ptStopLength;
    }

    /// C++: `NBPTStop & NBPTStop::operator=(const NBPTStop &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &NBPTStop) -> &mut NBPTStop {
                // C++ declares the assignment operator private and never defines it, so no caller can use it.
        panic!("NBPTStop::operator= is private and undefined in C++ (NBPTStop.h)")
    }

}

