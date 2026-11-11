//! Port of netbuild/NBPTStop.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

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
    pub myAccesses: Vec<Unmapped /* std::tuple<std::string, double, double> */>,
    /// list of public transport lines (for displaying)
    pub myLines: Vec<String>,
    pub myBidiStop: Unmapped /* std::weak_ptr<NBPTStop> */,
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
        todo!("port NBPTStop::NBPTStop")
    }

    /// C++: `void NBPTStop::~NBPTStop() noexcept` [virtual]
    /// Destructor
    // virtual ~NBPTStop() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port NBPTStop::~NBPTStop")
    }

    /// C++: `std::string NBPTStop::getID() const` [const]
    // std::string
    // NBPTStop::getID() const {
    //     return myPTStopId;
    // }
    #[allow(unused_variables)]
    pub fn get_id(&self) -> String {
        todo!("port NBPTStop::getID")
    }

    /// C++: `SumoXMLTag NBPTStop::getElement() const` [const]
    // SumoXMLTag getElement() const {
    //         return myTag;
    //     }
    #[allow(unused_variables)]
    pub fn get_element(&self) -> SumoXMLTag {
        todo!("port NBPTStop::getElement")
    }

    /// C++: `const std::string & NBPTStop::getEdgeId() const` [const]
    // const std::string&
    // NBPTStop::getEdgeId() const {
    //     return myEdgeId;
    // }
    #[allow(unused_variables)]
    pub fn get_edge_id(&self) -> &str {
        todo!("port NBPTStop::getEdgeId")
    }

    /// C++: `const std::string & NBPTStop::getLaneId() const` [const]
    // const std::string& getLaneId() const {
    //         return myLaneId;
    //     }
    #[allow(unused_variables)]
    pub fn get_lane_id(&self) -> &str {
        todo!("port NBPTStop::getLaneId")
    }

    /// C++: `const std::string NBPTStop::getOrigEdgeId() const` [const]
    // const std::string
    // NBPTStop::getOrigEdgeId() const {
    //     return myOrigEdgeId;
    // }
    #[allow(unused_variables)]
    pub fn get_orig_edge_id(&self) -> String {
        todo!("port NBPTStop::getOrigEdgeId")
    }

    /// C++: `const std::string NBPTStop::getName() const` [const]
    // const std::string
    // NBPTStop::getName() const {
    //     return myName;
    // }
    #[allow(unused_variables)]
    pub fn get_name(&self) -> String {
        todo!("port NBPTStop::getName")
    }

    /// C++: `const Position & NBPTStop::getPosition() const` [const]
    // const Position&
    // NBPTStop::getPosition() const {
    //     return myPosition;
    // }
    #[allow(unused_variables)]
    pub fn get_position(&self) -> &Position {
        todo!("port NBPTStop::getPosition")
    }

    /// C++: `SVCPermissions NBPTStop::getPermissions() const` [const]
    // SVCPermissions
    // NBPTStop::getPermissions() const {
    //     return myPermissions;
    // }
    #[allow(unused_variables)]
    pub fn get_permissions(&self) -> SVCPermissions {
        todo!("port NBPTStop::getPermissions")
    }

    /// C++: `long long NBPTStop::getAreaID() const` [const]
    // long long int getAreaID() const {
    //         return myAreaID;
    //     }
    #[allow(unused_variables)]
    pub fn get_area_id(&self) -> i64 {
        todo!("port NBPTStop::getAreaID")
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
        todo!("port NBPTStop::write")
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
        todo!("port NBPTStop::reshiftPosition")
    }

    /// C++: `const std::vector<NBPTPlatform> & NBPTStop::getPlatformCands()`
    // const std::vector<NBPTPlatform>&
    // NBPTStop::getPlatformCands() {
    //     return myPlatformCands;
    // }
    #[allow(unused_variables)]
    pub fn get_platform_cands(&mut self) -> &Vec<NBPTPlatform> {
        todo!("port NBPTStop::getPlatformCands")
    }

    /// C++: `bool NBPTStop::getIsMultipleStopPositions() const` [const]
    // bool
    // NBPTStop::getIsMultipleStopPositions() const {
    //     return myIsMultipleStopPositions;
    // }
    #[allow(unused_variables)]
    pub fn get_is_multiple_stop_positions(&self) -> bool {
        todo!("port NBPTStop::getIsMultipleStopPositions")
    }

    /// C++: `void NBPTStop::setIsMultipleStopPositions(bool, long long)`
    // void
    // NBPTStop::setIsMultipleStopPositions(bool multipleStopPositions, long long int areaID) {
    //     myIsMultipleStopPositions = multipleStopPositions;
    //     myAreaID = areaID;
    // }
    #[allow(unused_variables)]
    pub fn set_is_multiple_stop_positions(&mut self, multipleStopPositions: bool, areaID: i64) {
        todo!("port NBPTStop::setIsMultipleStopPositions")
    }

    /// C++: `double NBPTStop::getLength() const` [const]
    // double
    // NBPTStop::getLength() const {
    //     return myPTStopLength;
    // }
    #[allow(unused_variables)]
    pub fn get_length(&self) -> f64 {
        todo!("port NBPTStop::getLength")
    }

    /// C++: `bool NBPTStop::setEdgeId(std::string, const NBEdgeCont &)`
    // bool
    // NBPTStop::setEdgeId(std::string edgeId, const NBEdgeCont& ec) {
    //     myEdgeId = edgeId;
    //     return findLaneAndComputeBusStopExtent(ec);
    // }
    #[allow(unused_variables)]
    pub fn set_edge_id(&mut self, edgeId: String, ec: &NBEdgeCont) -> bool {
        todo!("port NBPTStop::setEdgeId")
    }

    /// C++: `void NBPTStop::registerAdditionalEdge(std::string, std::string)`
    // void
    // NBPTStop::registerAdditionalEdge(std::string wayId, std::string edgeId) {
    //     myAdditionalEdgeCandidates[wayId] = edgeId;
    // }
    #[allow(unused_variables)]
    pub fn register_additional_edge(&mut self, wayId: String, edgeId: String) {
        todo!("port NBPTStop::registerAdditionalEdge")
    }

    /// C++: `void NBPTStop::addPlatformCand(NBPTPlatform)`
    // void
    // NBPTStop::addPlatformCand(NBPTPlatform platform) {
    //     myPlatformCands.push_back(platform);
    // }
    #[allow(unused_variables)]
    pub fn add_platform_cand(&mut self, platform: NBPTPlatform) {
        todo!("port NBPTStop::addPlatformCand")
    }

    /// C++: `bool NBPTStop::findLaneAndComputeBusStopExtent(const NBEdgeCont &)`
    // bool
    // NBPTStop::findLaneAndComputeBusStopExtent(const NBEdgeCont& ec) {
    //     NBEdge* edge = ec.getByID(myEdgeId);
    //     return findLaneAndComputeBusStopExtent(edge);
    // }
    #[allow(unused_variables)]
    pub fn find_lane_and_compute_bus_stop_extent(&mut self, ec: &NBEdgeCont) -> bool {
        todo!("port NBPTStop::findLaneAndComputeBusStopExtent")
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
        todo!("port NBPTStop::findLaneAndComputeBusStopExtent")
    }

    /// C++: `void NBPTStop::setPTStopId(std::string)`
    // void setPTStopId(std::string id) {
    //         myPTStopId = id;
    //     }
    #[allow(unused_variables)]
    pub fn set_pt_stop_id(&mut self, id: String) {
        todo!("port NBPTStop::setPTStopId")
    }

    /// C++: `bool NBPTStop::wasLoaded() const` [const]
    // bool wasLoaded() const {
    //         return myGivenStartPos >= 0;
    //     }
    #[allow(unused_variables)]
    pub fn was_loaded(&self) -> bool {
        todo!("port NBPTStop::wasLoaded")
    }

    /// C++: `void NBPTStop::resetLoaded()`
    // void resetLoaded() {
    //         myGivenStartPos = -1;
    //     }
    #[allow(unused_variables)]
    pub fn reset_loaded(&mut self) {
        todo!("port NBPTStop::resetLoaded")
    }

    /// C++: `void NBPTStop::setIsPlatform()`
    // void setIsPlatform() {
    //         myIsPlatform = true;
    //     }
    #[allow(unused_variables)]
    pub fn set_is_platform(&mut self) {
        todo!("port NBPTStop::setIsPlatform")
    }

    /// C++: `bool NBPTStop::isPlatform() const` [const]
    // bool isPlatform() const {
    //         return myIsPlatform;
    //     }
    #[allow(unused_variables)]
    pub fn is_platform(&self) -> bool {
        todo!("port NBPTStop::isPlatform")
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
        todo!("port NBPTStop::addAccess")
    }

    /// C++: `void NBPTStop::clearAccess()`
    /// remove all access definitions
    // void
    // NBPTStop::clearAccess() {
    //     myAccesses.clear();
    // }
    #[allow(unused_variables)]
    pub fn clear_access(&mut self) {
        todo!("port NBPTStop::clearAccess")
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
        todo!("port NBPTStop::addLine")
    }

    /// C++: `void NBPTStop::setBidiStop(std::shared_ptr<NBPTStop>)`
    // void setBidiStop(std::shared_ptr<NBPTStop> bidiStop) {
    //         myBidiStop = bidiStop;
    //     }
    #[allow(unused_variables)]
    pub fn set_bidi_stop(&mut self, bidiStop: Rc<NBPTStop>) {
        todo!("port NBPTStop::setBidiStop")
    }

    /// C++: `std::shared_ptr<NBPTStop> NBPTStop::getBidiStop() const` [const]
    // std::shared_ptr<NBPTStop> getBidiStop() const {
    //         return myBidiStop.lock();
    //     }
    #[allow(unused_variables)]
    pub fn get_bidi_stop(&self) -> Rc<NBPTStop> {
        todo!("port NBPTStop::getBidiStop")
    }

    /// C++: `bool NBPTStop::isLoose() const` [const]
    // bool isLoose() const {
    //         return myIsLoose;
    //     }
    #[allow(unused_variables)]
    pub fn is_loose(&self) -> bool {
        todo!("port NBPTStop::isLoose")
    }

    /// C++: `double NBPTStop::getStartPos() const` [const]
    // double getStartPos() const {
    //         return myStartPos;
    //     }
    #[allow(unused_variables)]
    pub fn get_start_pos(&self) -> f64 {
        todo!("port NBPTStop::getStartPos")
    }

    /// C++: `double NBPTStop::getEndPos() const` [const]
    // double getEndPos() const {
    //         return myEndPos;
    //     }
    #[allow(unused_variables)]
    pub fn get_end_pos(&self) -> f64 {
        todo!("port NBPTStop::getEndPos")
    }

    /// C++: `const std::vector<std::string> & NBPTStop::getLines() const` [const]
    // const std::vector<std::string>& getLines() const {
    //         return myLines;
    //     }
    #[allow(unused_variables)]
    pub fn get_lines(&self) -> &Vec<String> {
        todo!("port NBPTStop::getLines")
    }

    /// C++: `void NBPTStop::mirrorX()`
    /// mirror coordinates along the x-axis
    // void
    // NBPTStop::mirrorX() {
    //     myPosition.mul(1, -1);
    // }
    #[allow(unused_variables)]
    pub fn mirror_x(&mut self) {
        todo!("port NBPTStop::mirrorX")
    }

    /// C++: `bool NBPTStop::replaceEdge(const std::string &, const std::vector<NBEdge *> &)`
    /// replace the stop edge with the closest edge on the given edge list in all stops
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn replace_edge(&mut self, edgeID: &str, replacement: &Vec<Ptr<NBEdge>>) -> bool {
        todo!("port NBPTStop::replaceEdge")
    }

    /// C++: `const std::map<std::string, std::string> & NBPTStop::getAdditionalEdgeCandidates() const` [const]
    // const std::map<std::string, std::string>& getAdditionalEdgeCandidates() const {
    //         return myAdditionalEdgeCandidates;
    //     }
    #[allow(unused_variables)]
    pub fn get_additional_edge_candidates(&self) -> &BTreeMap<String, String> {
        todo!("port NBPTStop::getAdditionalEdgeCandidates")
    }

    /// C++: `void NBPTStop::setOrigEdgeId(const std::string &)`
    // void setOrigEdgeId(const std::string& origEdgeId) {
    //         myOrigEdgeId = origEdgeId;
    //     }
    #[allow(unused_variables)]
    pub fn set_orig_edge_id(&mut self, origEdgeId: &str) {
        todo!("port NBPTStop::setOrigEdgeId")
    }

    /// C++: `void NBPTStop::setPTStopLength(double)`
    // void setPTStopLength(double ptStopLength) {
    //         myPTStopLength = ptStopLength;
    //     }
    #[allow(unused_variables)]
    pub fn set_pt_stop_length(&mut self, ptStopLength: f64) {
        todo!("port NBPTStop::setPTStopLength")
    }

    /// C++: `NBPTStop & NBPTStop::operator=(const NBPTStop &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &NBPTStop) -> &mut NBPTStop {
        todo!("port NBPTStop::operator=")
    }

}

