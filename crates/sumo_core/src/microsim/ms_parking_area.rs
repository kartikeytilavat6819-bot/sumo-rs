//! Port of microsim/MSParkingArea.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// Representation of a single lot space
/// C++ class `MSParkingArea::LotSpaceDefinition` (microsim/MSParkingArea.h)
pub struct MSParkingArea_LotSpaceDefinition {
    /// the running index
    pub index: i32,
    /// The last parked vehicle or 0
    pub vehicle: Ptr<SUMOVehicle>,
    /// The position of the vehicle when parking in this space
    pub position: Position,
    /// The rotation
    pub rotation: f64,
    /// The slope
    pub slope: f64,
    /// The width
    pub width: f64,
    /// The length
    pub length: f64,
    /// The position along the lane that the vehicle needs to reach for entering this lot
    pub endPos: f64,
    /// The angle between lane and lot through which a vehicle must manoeuver to enter the lot
    pub manoeuverAngle: f64,
    /// Whether the lot is on the LHS of the lane relative to the lane direction
    pub sideIsLHS: bool,
}

impl MSParkingArea_LotSpaceDefinition {
    /// C++: `void LotSpaceDefinition::LotSpaceDefinition()`
    /// default constructor
    // MSParkingArea::LotSpaceDefinition::LotSpaceDefinition() :
    //     index(-1),
    //     vehicle(nullptr),
    //     rotation(0),
    //     slope(0),
    //     width(0),
    //     length(0),
    //     endPos(0),
    //     manoeuverAngle(0),
    //     sideIsLHS(false) {
    // }
    #[allow(unused_variables)]
    pub fn new() -> Self {
        todo!("port LotSpaceDefinition::LotSpaceDefinition")
    }

    /// C++: `void LotSpaceDefinition::LotSpaceDefinition(int, SUMOVehicle *, double, double, double, double, double, double, double)`
    /// parameter constructor
    // MSParkingArea::LotSpaceDefinition::LotSpaceDefinition(int index_, SUMOVehicle* vehicle_, double x, double y, double z, double rotation_, double slope_, double width_, double length_) :
    //     index(index_),
    //     vehicle(vehicle_),
    //     position(Position(x, y, z)),
    //     rotation(rotation_),
    //     slope(slope_),
    //     width(width_),
    //     length(length_),
    //     endPos(0),
    //     manoeuverAngle(0),
    //     sideIsLHS(false) {
    // }
    #[allow(unused_variables)]
    pub fn new_2(index: i32, vehicle: Ptr<SUMOVehicle>, x: f64, y: f64, z: f64, rotation: f64, slope: f64, width: f64, length: f64) -> Self {
        todo!("port LotSpaceDefinition::LotSpaceDefinition")
    }

}

/// A lane area vehicles can halt at
/// C++ class `MSParkingArea` (microsim/MSParkingArea.h)
pub struct MSParkingArea {
    /// C++ base class `MSStoppingPlace`
    pub base: MSStoppingPlace,
    /// Last free lot number (-1 no free lot)
    pub myLastFreeLot: i32,
    /// Stop area capacity configured via roadsideCapacity
    pub myRoadSideCapacity: i32,
    /// Stop area total capacity
    pub myCapacity: i32,
    /// Whether vehicles stay on the road
    pub myOnRoad: bool,
    /// Whether this parkingarea may receive reservations by vehicles that are on their way
    pub myReservable: bool,
    /// The default width of each parking space
    pub myWidth: f64,
    /// The default length of each parking space
    pub myLength: f64,
    /// The default angle of each parking space
    pub myAngle: f64,
    /// All the spaces in this parking area
    pub mySpaceOccupancies: Vec<MSParkingArea_LotSpaceDefinition>,
    /// The roadside shape of this parkingArea
    pub myShape: PositionVector,
    /// The parking badges to grant access
    pub myAcceptedBadges: BTreeSet<String>,
    /// whether a vehicle wants to exit but is blocked
    pub myEgressBlocked: bool,
    /// track parking reservations from the lane for the current time step
    pub myReservationTime: SUMOTime,
    pub myLastReservationTime: SUMOTime,
    /// number of reservations
    pub myReservations: i32,
    pub myLastReservations: i32,
    /// reservation max length
    pub myReservationMaxLength: f64,
    pub myLastReservationMaxLength: f64,
    /// the set of vehicles that performed a local reservation in this step
    pub myReservedVehicles: BTreeSet<Ptr<SUMOVehicle>>,
    /// the set of vehicles that performed a remote reservation
    pub myRemoteReservedVehicles: BTreeSet<Ptr<SUMOVehicle>>,
    /// a copy from the last step is needed to achieve thread/lane ordering independence
    pub myLastRemoteReservedVehicles: BTreeSet<Ptr<SUMOVehicle>>,
    /// maximum length of all parked vehicles
    pub myMaxVehLength: f64,
    /// the number of alternative parkingAreas that are assigned to parkingAreaRerouter
    pub myNumAlternatives: i32,
    /// Changes to the occupancy in the current time step
    pub myLastStepOccupancy: i32,
    /// custom departPos
    pub myDepartPos: f64,
    pub myDepartPosDefinition: DepartPosDefinition,
    /// Event for updating the occupancy
    pub myUpdateEvent: Ptr<Unmapped /* Command */>,
}

impl std::ops::Deref for MSParkingArea {
    type Target = MSStoppingPlace;
    fn deref(&self) -> &MSStoppingPlace {
        &self.base
    }
}
impl std::ops::DerefMut for MSParkingArea {
    fn deref_mut(&mut self) -> &mut MSStoppingPlace {
        &mut self.base
    }
}

impl MSParkingArea {
    /// C++: `void MSParkingArea::MSParkingArea(const std::string &, const std::vector<std::string> &, const std::vector<std::string> &, MSLane &, double, double, int, double, double, double, const std::string &, bool, const std::string &, bool, bool)`
    /// Constructor
    // MSParkingArea::MSParkingArea(const std::string& id, const std::vector<std::string>& lines,
    //                              const std::vector<std::string>& badges, MSLane& lane,
    //                              double begPos, double endPos, int capacity, double width, double length,
    //                              double angle, const std::string& name, bool onRoad,
    //                              const std::string& departPos, bool lefthand, bool reservable) :
    //     MSStoppingPlace(id, SUMO_TAG_PARKING_AREA, lines, lane, begPos, endPos, name),
    //     myRoadSideCapacity(capacity),
    //     myCapacity(0),
    //     myOnRoad(onRoad),
    //     myReservable(reservable),
    //     myWidth(width),
    //     myLength(length),
    //     myAngle(lefthand ? -angle : angle),
    //     myAcceptedBadges(badges.begin(), badges.end()),
    //     myEgressBlocked(false),
    //     myReservationTime(-1),
    //     myLastReservationTime(-1),
    //     myReservations(0),
    //     myLastReservations(0),
    //     myReservationMaxLength(0),
    //     myLastReservationMaxLength(0),
    //     myMaxVehLength(0),
    //     myNumAlternatives(0),
    //     myLastStepOccupancy(0),
    //     myDepartPos(-1),
    //     myDepartPosDefinition(DepartPosDefinition::DEFAULT),
    //     myUpdateEvent(nullptr) {
    //     // initialize unspecified defaults
    //     if (myWidth == 0) {
    //         myWidth = SUMO_const_laneWidth;
    //     }
    //
    //     if (departPos != "") {
    //         std::string error;
    //         if (!SUMOVehicleParameter::parseDepartPos(departPos, toString(myElement), getID(), myDepartPos, myDepartPosDefinition, error)) {
    //             throw ProcessError(error);
    //         }
    //         if (myDepartPosDefinition != DepartPosDefinition::GIVEN) {
    //             // maybe allow other methods at a later time
    //             throw ProcessError(TLF("Only a numerical departPos is supported for % '%'", toString(myElement), getID()));
    //         } else if (myDepartPos < 0 || myDepartPos > lane.getLength()) {
    //             throw ProcessError(TLF("Invalid departPos for % '%'", toString(myElement), getID()));
    //         }
    //     }
    //
    //     const double offset = (MSGlobals::gLefthand != lefthand) ? -1 : 1;
    //     myShape = lane.getShape().getSubpart(
    //                   lane.interpolateLanePosToGeometryPos(begPos),
    //                   lane.interpolateLanePosToGeometryPos(endPos));
    //     if (!myOnRoad) {
    //         myShape.move2side((lane.getWidth() / 2. + myWidth / 2.) * offset);
    //     }
    //     setRoadsideCapacity(capacity);
    // }
    #[allow(unused_variables)]
    pub fn new(id: &str, lines: &Vec<String>, badges: &Vec<String>, lane: &mut MSLane, begPos: f64, endPos: f64, capacity: i32, width: f64, length: f64, angle: f64, name: &str, onRoad: bool, departPos: &str, lefthand: bool, reservable: bool) -> Self {
        todo!("port MSParkingArea::MSParkingArea")
    }

    /// C++: `void MSParkingArea::~MSParkingArea() noexcept` [virtual]
    /// Destructor
    // MSParkingArea::~MSParkingArea() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MSParkingArea::~MSParkingArea")
    }

    /// C++: `int MSParkingArea::getCapacity() const` [const]
    /// Returns the area capacity
    // int
    // MSParkingArea::getCapacity() const {
    //     return myCapacity;
    // }
    #[allow(unused_variables)]
    pub fn get_capacity(&self) -> i32 {
        todo!("port MSParkingArea::getCapacity")
    }

    /// C++: `bool MSParkingArea::parkOnRoad() const` [const]
    /// whether vehicles park on the road
    // bool
    // MSParkingArea::parkOnRoad() const {
    //     return myOnRoad;
    // }
    #[allow(unused_variables)]
    pub fn park_on_road(&self) -> bool {
        todo!("port MSParkingArea::parkOnRoad")
    }

    /// C++: `bool MSParkingArea::mustAdvance(SUMOVehicleClass) const` [const]
    /// whether parked vehicles must advance in a queue
    // bool
    // MSParkingArea::mustAdvance(SUMOVehicleClass svc) const {
    //     return myOnRoad && cannotChange(svc);
    // }
    #[allow(unused_variables)]
    pub fn must_advance(&self, svc: SUMOVehicleClass) -> bool {
        todo!("port MSParkingArea::mustAdvance")
    }

    /// C++: `bool MSParkingArea::isReservable() const` [const]
    /// whether vehicles may reserve a slot for this parkingArea
    // inline bool isReservable() const {
    //         return myReservable;
    //     }
    #[allow(unused_variables)]
    pub fn is_reservable(&self) -> bool {
        todo!("port MSParkingArea::isReservable")
    }

    /// C++: `int MSParkingArea::getLotIndex(const SUMOVehicle *) const` [const]
    /// compute lot for this vehicle
    // int
    // MSParkingArea::getLotIndex(const SUMOVehicle* veh) const {
    //     if (veh->getPositionOnLane() > myLastFreePos) {
    //         // vehicle has gone past myLastFreePos and we need to find the actual lot
    //         int closestLot = -1;
    //         for (int i = 0; i < (int)mySpaceOccupancies.size(); i++) {
    //             const LotSpaceDefinition lsd = mySpaceOccupancies[i];
    //             if (lsd.vehicle == nullptr) {
    //                 closestLot = i;
    //                 if (lsd.endPos >= veh->getPositionOnLane()) {
    //                     return i;
    //                 }
    //             }
    //         }
    //         return closestLot;
    //     }
    //     if (myOnRoad && myLastFreePos - veh->getPositionOnLane() > POSITION_EPS) {
    //         // for on-road parking we need to be precise
    //         return -1;
    //     }
    //     return myLastFreeLot;
    // }
    #[allow(unused_variables)]
    pub fn get_lot_index(&self, veh: Ptr<SUMOVehicle>) -> i32 {
        todo!("port MSParkingArea::getLotIndex")
    }

    /// C++: `int MSParkingArea::getOccupancy() const` [const]
    /// Returns the area occupancy
    // int
    // MSParkingArea::getOccupancy() const {
    //     return (int)myEndPositions.size() - (myEgressBlocked ? 1 : 0);
    // }
    #[allow(unused_variables)]
    pub fn get_occupancy(&self) -> i32 {
        todo!("port MSParkingArea::getOccupancy")
    }

    /// C++: `int MSParkingArea::getOccupancyIncludingBlocked() const` [const]
    /// Returns the area occupancy
    // int
    // MSParkingArea::getOccupancyIncludingBlocked() const {
    //     return (int)myEndPositions.size();
    // }
    #[allow(unused_variables)]
    pub fn get_occupancy_including_blocked(&self) -> i32 {
        todo!("port MSParkingArea::getOccupancyIncludingBlocked")
    }

    /// C++: `int MSParkingArea::getOccupancyIncludingReservations(const SUMOVehicle *) const` [const]
    // int
    // MSParkingArea::getOccupancyIncludingReservations(const SUMOVehicle* forVehicle) const {
    //     const bool reservedLocal = myReservedVehicles.count(forVehicle) != 0;
    //     const bool reservedRemote = myRemoteReservedVehicles.count(forVehicle) != 0;
    //     return ((int)myEndPositions.size()
    //         + (reservedLocal ? 0 : myReservations)
    //         + (reservedRemote ? 0 : (int)myRemoteReservedVehicles.size()));
    // }
    #[allow(unused_variables)]
    pub fn get_occupancy_including_reservations(&self, forVehicle: Ptr<SUMOVehicle>) -> i32 {
        todo!("port MSParkingArea::getOccupancyIncludingReservations")
    }

    /// C++: `int MSParkingArea::getOccupancyIncludingRemoteReservations(const SUMOVehicle *) const` [const]
    // int
    // MSParkingArea::getOccupancyIncludingRemoteReservations(const SUMOVehicle* forVehicle) const {
    //     const int reservedRemote = myRemoteReservedVehicles.count(forVehicle) != 0;
    //     return getOccupancy() + (int)myRemoteReservedVehicles.size() - reservedRemote;
    // }
    #[allow(unused_variables)]
    pub fn get_occupancy_including_remote_reservations(&self, forVehicle: Ptr<SUMOVehicle>) -> i32 {
        todo!("port MSParkingArea::getOccupancyIncludingRemoteReservations")
    }

    /// C++: `int MSParkingArea::getLastStepOccupancy() const` [const]
    /// Returns the area occupancy at the end of the last simulation step
    // int
    // MSParkingArea::getLastStepOccupancy() const {
    //     return myLastStepOccupancy;
    // }
    #[allow(unused_variables)]
    pub fn get_last_step_occupancy(&self) -> i32 {
        todo!("port MSParkingArea::getLastStepOccupancy")
    }

    /// C++: `int MSParkingArea::getLastStepOccupancyIncludingRemoteReservations(const SUMOVehicle *) const` [const]
    // int
    // MSParkingArea::getLastStepOccupancyIncludingRemoteReservations(const SUMOVehicle* forVehicle) const {
    //     const int reservedRemote = myLastRemoteReservedVehicles.count(forVehicle) != 0;
    //     return myLastStepOccupancy + (int)myLastRemoteReservedVehicles.size() - reservedRemote;
    // }
    #[allow(unused_variables)]
    pub fn get_last_step_occupancy_including_remote_reservations(&self, forVehicle: Ptr<SUMOVehicle>) -> i32 {
        todo!("port MSParkingArea::getLastStepOccupancyIncludingRemoteReservations")
    }

    /// C++: `void MSParkingArea::accept(std::string)`
    /// Add a badge to the accepted set
    // void
    // MSParkingArea::accept(std::string badge) {
    //     myAcceptedBadges.insert(badge);
    // }
    #[allow(unused_variables)]
    pub fn accept(&mut self, badge: String) {
        todo!("port MSParkingArea::accept")
    }

    /// C++: `void MSParkingArea::accept(std::vector<std::string>)`
    /// Add badges to the accepted set
    // void
    // MSParkingArea::accept(std::vector<std::string> badges) {
    //     myAcceptedBadges.insert(badges.begin(), badges.end());
    // }
    #[allow(unused_variables)]
    pub fn accept_2(&mut self, badges: Vec<String>) {
        todo!("port MSParkingArea::accept")
    }

    /// C++: `void MSParkingArea::refuse(std::string)`
    /// Remove the access right for the given badge
    // void
    // MSParkingArea::refuse(std::string badge) {
    //     myAcceptedBadges.erase(badge);
    // }
    #[allow(unused_variables)]
    pub fn refuse(&mut self, badge: String) {
        todo!("port MSParkingArea::refuse")
    }

    /// C++: `bool MSParkingArea::accepts(SUMOVehicle *) const` [const, virtual]
    /// Return the parking accepts the vehicle (due to its given badges)
    // bool
    // MSParkingArea::accepts(SUMOVehicle* veh) const {
    //     if (myAcceptedBadges.size() == 0) {
    //         return true;
    //     } else {
    //         std::vector<std::string> vehicleBadges = veh->getParkingBadges();
    //         for (auto badge : vehicleBadges) {
    //             if (myAcceptedBadges.count(badge) != 0) {
    //                 return true;
    //             }
    //         }
    //         return false;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn accepts(&self, veh: Ptr<SUMOVehicle>) -> bool {
        todo!("port MSParkingArea::accepts")
    }

    /// C++: `void MSParkingArea::enter(SUMOVehicle *, const bool)` [virtual]
    /// Called if a vehicle enters this stop
    // void
    // MSParkingArea::enter(SUMOVehicle* veh, const bool /* parking */) {
    //     removeSpaceReservation(veh);
    //     double beg = veh->getPositionOnLane() + veh->getVehicleType().getMinGap();
    //     double end = veh->getPositionOnLane() - veh->getVehicleType().getLength();
    //     if (myUpdateEvent == nullptr) {
    //         myUpdateEvent = new WrappingCommand<MSParkingArea>(this, &MSParkingArea::updateOccupancy);
    //         MSNet::getInstance()->getEndOfTimestepEvents()->addEvent(myUpdateEvent);
    //     }
    //     int lotIndex = getLotIndex(veh);
    //     if (lotIndex < 0) {
    //         WRITE_WARNING("Unsuitable parking position for vehicle '" + veh->getID() + "' at parkingArea '" + getID() + "' time=" + time2string(SIMSTEP));
    //         lotIndex = myLastFreeLot;
    //     }
    // #ifdef DEBUG_GET_LAST_FREE_POS
    //     ((SUMOVehicleParameter&)veh->getParameter()).setParameter("lotIndex", toString(lotIndex));
    // #endif
    //     assert(myLastFreePos >= 0);
    //     assert(lotIndex < (int)mySpaceOccupancies.size());
    //     mySpaceOccupancies[lotIndex].vehicle = veh;
    //     myEndPositions[veh] = std::pair<double, double>(beg, end);
    //     computeLastFreePos();
    //     myMaxVehLength = MAX2(myMaxVehLength, veh->getLength());
    //     // current search ends here
    //     veh->setNumberParkingReroutes(0);
    // }
    #[allow(unused_variables)]
    pub fn enter(&mut self, veh: Ptr<SUMOVehicle>, parking: bool) {
        todo!("port MSParkingArea::enter")
    }

    /// C++: `void MSParkingArea::leaveFrom(SUMOVehicle *)` [virtual]
    /// Called if a vehicle leaves this stop
    // void
    // MSParkingArea::leaveFrom(SUMOVehicle* what) {
    //     assert(myEndPositions.find(what) != myEndPositions.end());
    //     if (myUpdateEvent == nullptr) {
    //         myUpdateEvent = new WrappingCommand<MSParkingArea>(this, &MSParkingArea::updateOccupancy);
    //         MSNet::getInstance()->getEndOfTimestepEvents()->addEvent(myUpdateEvent);
    //     }
    //     for (auto& lsd : mySpaceOccupancies) {
    //         if (lsd.vehicle == what) {
    //             lsd.vehicle = nullptr;
    //             break;
    //         }
    //     }
    //     if (what->getLength() == myMaxVehLength) {
    //         myMaxVehLength = 0;
    //         for (auto item : myEndPositions) {
    //             myMaxVehLength = MAX2(myMaxVehLength, item.first->getLength());
    //         }
    //     }
    //     myEndPositions.erase(myEndPositions.find(what));
    //     computeLastFreePos();
    // }
    #[allow(unused_variables)]
    pub fn leave_from(&mut self, what: Ptr<SUMOVehicle>) {
        todo!("port MSParkingArea::leaveFrom")
    }

    /// C++: `void MSParkingArea::addSpaceReservation(const SUMOVehicle *)`
    /// api for reserving spaces at this parkingArea @{
    // void
    // MSParkingArea::addSpaceReservation(const SUMOVehicle* veh) {
    //     myRemoteReservedVehicles.insert(veh);
    //     if (myUpdateEvent == nullptr) {
    //         myUpdateEvent = new WrappingCommand<MSParkingArea>(this, &MSParkingArea::updateOccupancy);
    //         MSNet::getInstance()->getEndOfTimestepEvents()->addEvent(myUpdateEvent);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_space_reservation(&mut self, veh: Ptr<SUMOVehicle>) {
        todo!("port MSParkingArea::addSpaceReservation")
    }

    /// C++: `void MSParkingArea::removeSpaceReservation(const SUMOVehicle *)`
    // void
    // MSParkingArea::removeSpaceReservation(const SUMOVehicle* veh) {
    //     myRemoteReservedVehicles.erase(veh);
    //     if (myUpdateEvent == nullptr) {
    //         myUpdateEvent = new WrappingCommand<MSParkingArea>(this, &MSParkingArea::updateOccupancy);
    //         MSNet::getInstance()->getEndOfTimestepEvents()->addEvent(myUpdateEvent);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn remove_space_reservation(&mut self, veh: Ptr<SUMOVehicle>) {
        todo!("port MSParkingArea::removeSpaceReservation")
    }

    /// C++: `const std::set<const SUMOVehicle *> & MSParkingArea::getRemoteReservedVehicles() const` [const]
    // const std::set<const SUMOVehicle*>& getRemoteReservedVehicles() const {
    //         return myRemoteReservedVehicles;
    //     }
    #[allow(unused_variables)]
    pub fn get_remote_reserved_vehicles(&self) -> &BTreeSet<Ptr<SUMOVehicle>> {
        todo!("port MSParkingArea::getRemoteReservedVehicles")
    }

    /// C++: `SUMOTime MSParkingArea::updateOccupancy(SUMOTime)`
    /// Called at the end of the time step
    // SUMOTime
    // MSParkingArea::updateOccupancy(SUMOTime /* currentTime */) {
    //     myLastStepOccupancy = getOccupancy();
    //     myLastRemoteReservedVehicles = myRemoteReservedVehicles;
    //     myUpdateEvent = nullptr;
    //     return 0;
    // }
    #[allow(unused_variables)]
    pub fn update_occupancy(&mut self, currentTime: SUMOTime) -> SUMOTime {
        todo!("port MSParkingArea::updateOccupancy")
    }

    /// C++: `double MSParkingArea::getLastFreePos(const SUMOVehicle &, double) const` [const, virtual]
    /// Returns the last free position on this stop
    // double
    // MSParkingArea::getLastFreePos(const SUMOVehicle& forVehicle, double brakePos) const {
    //     if (myCapacity == (int)myEndPositions.size()) {
    //         // keep enough space so that  parking vehicles can leave
    // #ifdef DEBUG_GET_LAST_FREE_POS
    //         if (DEBUG_COND2(forVehicle)) {
    //             std::cout << SIMTIME << " getLastFreePos veh=" << forVehicle.getID() << " allOccupied\n";
    //         }
    // #endif
    //         return myLastFreePos - forVehicle.getVehicleType().getMinGap() - POSITION_EPS;
    //     } else if (mustAdvance(forVehicle.getVClass())) {
    //         // vehicles cannot overtake so we must fill from the downstream end
    //         int skipN = SIMSTEP == myReservationTime ? myReservations - 1 : 0;
    //         //std::cout << SIMTIME << " v=" << forVehicle.getID() << " t=" << SIMTIME << " resTime=" << STEPS2TIME(myReservationTime) << " myR=" << myReservations << " skip=" << skipN << " rV=" << toString(myReservedVehicles) << "\n";
    //         for (auto it_lsd = mySpaceOccupancies.rbegin(); it_lsd != mySpaceOccupancies.rend(); it_lsd++) {
    //             if (it_lsd->vehicle == nullptr) {
    //                 if (skipN > 0) {
    //                     // skip reservations
    //                     skipN--;
    //                     continue;
    //                 }
    // #ifdef DEBUG_GET_LAST_FREE_POS
    //                 if (DEBUG_COND2(forVehicle)) {
    //                     std::cout << SIMTIME << " getLastFreePos (onRoad-upstream) veh=" << forVehicle.getID() << " brakePos=" << brakePos << " myEndPos=" << myEndPos << " nextFreePos=" << it_lsd->endPos << "\n";
    //                 }
    // #endif
    //                 return it_lsd->endPos;
    //             }
    //         }
    //         // should not happen
    //         return brakePos;
    //     } else {
    //         const double minPos = MIN2(myEndPos, brakePos);
    //         if (myLastFreePos >= minPos) {
    // #ifdef DEBUG_GET_LAST_FREE_POS
    //             if (DEBUG_COND2(forVehicle)) {
    //                 std::cout << SIMTIME << " getLastFreePos veh=" << forVehicle.getID() << " brakePos=" << brakePos << " myEndPos=" << myEndPos << " using myLastFreePos=" << myLastFreePos << "\n";
    //             }
    // #endif
    //             return myLastFreePos;
    //         } else {
    //             // find free pos after minPos
    //             for (const auto& lsd : mySpaceOccupancies) {
    //                 if (lsd.vehicle == nullptr && lsd.endPos >= minPos) {
    // #ifdef DEBUG_GET_LAST_FREE_POS
    //                     if (DEBUG_COND2(forVehicle)) {
    //                         std::cout << SIMTIME << " getLastFreePos veh=" << forVehicle.getID() << " brakePos=" << brakePos << " myEndPos=" << myEndPos << " nextFreePos=" << lsd.endPos << "\n";
    //                     }
    // #endif
    //                     return lsd.endPos;
    //                 }
    //             }
    //             // shouldn't happen. No good solution seems possible
    // #ifdef DEBUG_GET_LAST_FREE_POS
    //             if (DEBUG_COND2(forVehicle)) {
    //                 std::cout << SIMTIME << " getLastFreePos veh=" << forVehicle.getID() << " brakePos=" << brakePos << " myEndPos=" << myEndPos << " noGoodFreePos blockedAt=" << brakePos << "\n";
    //             }
    // #endif
    //             return brakePos;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_last_free_pos(&self, forVehicle: &SUMOVehicle, brakePos: f64) -> f64 {
        todo!("port MSParkingArea::getLastFreePos")
    }

    /// C++: `double MSParkingArea::getLastFreePosWithReservation(SUMOTime, const SUMOVehicle &, double)`
    /// Returns the last free position on this stop including reservations from the current lane and time step
    // double
    // MSParkingArea::getLastFreePosWithReservation(SUMOTime t, const SUMOVehicle& forVehicle, double brakePos) {
    //     // do not perform reservation when far away
    //     if (forVehicle.getLane() != &myLane || forVehicle.getPositionOnLane() < (myBegPos - myMaxVehLength - forVehicle.getVehicleType().getMinGap())) {
    //         // for different lanes, do not consider reservations to avoid lane-order
    //         // dependency in parallel simulation
    // #ifdef DEBUG_RESERVATIONS
    //         if (DEBUG_COND2(forVehicle)) {
    //             std::cout << SIMTIME << " pa=" << getID() << " freePosRes veh=" << forVehicle.getID() << " other lane\n";
    //         }
    // #endif
    //         if (myNumAlternatives > 0 && getOccupancy() == getCapacity()) {
    //             // ensure that the vehicle reaches the rerouter lane
    //             if (mySpaceOccupancies.size() > 0) {
    //                 assert(mySpaceOccupancies[0].vehicle != nullptr);
    //                 const double waitPos = (mySpaceOccupancies[0].endPos
    //                                         - mySpaceOccupancies[0].vehicle->getLength()
    //                                         - forVehicle.getVehicleType().getMinGap()
    //                                         - NUMERICAL_EPS);
    //                 return MAX2(waitPos, MIN2(POSITION_EPS, myEndPos));
    //             } else {
    //                 return MAX2(myBegPos, MIN2(POSITION_EPS, myEndPos));
    //             }
    //         } else {
    //             // check if there is a reservation from the last time step
    //             // (this could also be in myReserations, if myLane wasn't processed before the forVehicle-lane)
    //             const SUMOTime last = t - DELTA_T;
    // #ifdef DEBUG_RESERVATIONS
    //             if (DEBUG_COND2(forVehicle)) {
    //                 std::cout << SIMTIME << " last=" << time2string(last) << " lastRes=" << time2string(myLastReservationTime) << " resTime=" << toString(myReservationTime) << "\n";
    //             }
    // #endif
    //             if (myLastReservationTime == last || myReservationTime == last) {
    //                 int res = myLastReservationTime == last ? myLastReservations : myReservations;
    //                 if (myCapacity <= getOccupancy() + res) {
    //                     double maxLen = myLastReservationTime == last ? myLastReservationMaxLength : myReservationMaxLength;
    // #ifdef DEBUG_RESERVATIONS
    //                     if (DEBUG_COND2(forVehicle)) std::cout << SIMTIME << " pa=" << getID() << " freePosRes veh=" << forVehicle.getID()
    //                                                                << " lastRes=" << res << " resTime=" << myReservationTime << " reserved full, maxLen=" << maxLen << " endPos=" << mySpaceOccupancies[0].endPos << "\n";
    // #endif
    //                     return (mySpaceOccupancies[0].endPos
    //                             - maxLen
    //                             - forVehicle.getVehicleType().getMinGap()
    //                             - NUMERICAL_EPS);
    //                 }
    //             }
    //             return getLastFreePos(forVehicle, brakePos);
    //         }
    //     }
    //     if (t > myReservationTime) {
    // #ifdef DEBUG_RESERVATIONS
    //         if (DEBUG_COND2(forVehicle)) {
    //             std::cout << SIMTIME << " pa=" << getID() << " freePosRes veh=" << forVehicle.getID() << " first reservation\n";
    //         }
    // #endif
    //         myLastReservationTime = myReservationTime;
    //         myLastReservations = myReservations;
    //         myLastReservationMaxLength = myReservationMaxLength;
    //         myReservationTime = t;
    //         myReservations = 1;
    //         myReservationMaxLength = forVehicle.getVehicleType().getLength();
    //         myReservedVehicles.clear();
    //         myReservedVehicles.insert(&forVehicle);
    //         for (const auto& lsd : mySpaceOccupancies) {
    //             if (lsd.vehicle != nullptr) {
    //                 myReservationMaxLength = MAX2(myReservationMaxLength, lsd.vehicle->getVehicleType().getLength());
    //             }
    //         }
    //         return getLastFreePos(forVehicle, brakePos);
    //     } else {
    //         if (myCapacity > getOccupancy() + myReservations) {
    // #ifdef DEBUG_RESERVATIONS
    //             if (DEBUG_COND2(forVehicle)) {
    //                 std::cout << SIMTIME << " pa=" << getID() << " freePosRes veh=" << forVehicle.getID() << " res=" << myReservations << " enough space\n";
    //             }
    // #endif
    //             myReservations++;
    //             myReservationMaxLength = MAX2(myReservationMaxLength, forVehicle.getVehicleType().getLength());
    //             myReservedVehicles.insert(&forVehicle);
    //             return getLastFreePos(forVehicle, brakePos);
    //         } else {
    //             if (myCapacity == 0) {
    //                 return getLastFreePos(forVehicle, brakePos);
    //             } else {
    // #ifdef DEBUG_RESERVATIONS
    //                 if (DEBUG_COND2(forVehicle)) std::cout << SIMTIME << " pa=" << getID() << " freePosRes veh=" << forVehicle.getID()
    //                                                            << " res=" << myReservations << " resTime=" << myReservationTime << " reserved full, maxLen=" << myReservationMaxLength << " endPos=" << mySpaceOccupancies[0].endPos << "\n";
    // #endif
    //                 return (mySpaceOccupancies[0].endPos
    //                         - myReservationMaxLength
    //                         - forVehicle.getVehicleType().getMinGap()
    //                         - NUMERICAL_EPS);
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_last_free_pos_with_reservation(&mut self, t: SUMOTime, forVehicle: &SUMOVehicle, brakePos: f64) -> f64 {
        todo!("port MSParkingArea::getLastFreePosWithReservation")
    }

    /// C++: `Position MSParkingArea::getVehiclePosition(const SUMOVehicle &) const` [const]
    /// Returns the position of parked vehicle
    // Position
    // MSParkingArea::getVehiclePosition(const SUMOVehicle& forVehicle) const {
    //     for (const auto& lsd : mySpaceOccupancies) {
    //         if (lsd.vehicle == &forVehicle) {
    //             return lsd.position;
    //         }
    //     }
    //     return Position::INVALID;
    // }
    #[allow(unused_variables)]
    pub fn get_vehicle_position(&self, forVehicle: &SUMOVehicle) -> Position {
        todo!("port MSParkingArea::getVehiclePosition")
    }

    /// C++: `double MSParkingArea::getInsertionPosition(const SUMOVehicle &) const` [const]
    /// Returns the insertion position of a parked vehicle
    // double
    // MSParkingArea::getInsertionPosition(const SUMOVehicle& forVehicle) const {
    //     if (myDepartPosDefinition == DepartPosDefinition::GIVEN) {
    //         return myDepartPos;
    //     }
    //     for (const auto& lsd : mySpaceOccupancies) {
    //         if (lsd.vehicle == &forVehicle) {
    //             return lsd.endPos;
    //         }
    //     }
    //     return -1;
    // }
    #[allow(unused_variables)]
    pub fn get_insertion_position(&self, forVehicle: &SUMOVehicle) -> f64 {
        todo!("port MSParkingArea::getInsertionPosition")
    }

    /// C++: `double MSParkingArea::getVehicleAngle(const SUMOVehicle &) const` [const]
    /// Returns the angle of parked vehicle
    // double
    // MSParkingArea::getVehicleAngle(const SUMOVehicle& forVehicle) const {
    //     for (const auto& lsd : mySpaceOccupancies) {
    //         if (lsd.vehicle == &forVehicle) {
    //             return (lsd.rotation - 90.) * (double) M_PI / (double) 180.0;
    //         }
    //     }
    //     return 0;
    // }
    #[allow(unused_variables)]
    pub fn get_vehicle_angle(&self, forVehicle: &SUMOVehicle) -> f64 {
        todo!("port MSParkingArea::getVehicleAngle")
    }

    /// C++: `double MSParkingArea::getVehicleSlope(const SUMOVehicle &) const` [const]
    /// Returns the slope of parked vehicle
    // double
    // MSParkingArea::getVehicleSlope(const SUMOVehicle& forVehicle) const {
    //     for (const auto& lsd : mySpaceOccupancies) {
    //         if (lsd.vehicle == &forVehicle) {
    //             return lsd.slope;
    //         }
    //     }
    //     return 0;
    // }
    #[allow(unused_variables)]
    pub fn get_vehicle_slope(&self, forVehicle: &SUMOVehicle) -> f64 {
        todo!("port MSParkingArea::getVehicleSlope")
    }

    /// C++: `int MSParkingArea::getLastFreeLotAngle() const` [const]
    /// Return the angle of myLastFreeLot - the next parking lot only expected to be called after we have established there is space in the parking area
    // int
    // MSParkingArea::getLastFreeLotAngle() const {
    //     assert(myLastFreeLot >= 0);
    //     assert(myLastFreeLot < (int)mySpaceOccupancies.size());
    //
    //     const LotSpaceDefinition& lsd = mySpaceOccupancies[myLastFreeLot];
    //     if (lsd.sideIsLHS) {
    //         return abs(int(lsd.manoeuverAngle)) % 180;
    //     } else {
    //         return abs(abs(int(lsd.manoeuverAngle)) % 180 - 180) % 180;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_last_free_lot_angle(&self) -> i32 {
        todo!("port MSParkingArea::getLastFreeLotAngle")
    }

    /// C++: `double MSParkingArea::getLastFreeLotGUIAngle() const` [const]
    /// Return the GUI angle of myLastFreeLot - the angle the GUI uses to rotate into the next parking lot as above, only expected to be called after we have established there is space in the parking area
    // double
    // MSParkingArea::getLastFreeLotGUIAngle() const {
    //     assert(myLastFreeLot >= 0);
    //     assert(myLastFreeLot < (int)mySpaceOccupancies.size());
    //
    //     const LotSpaceDefinition& lsd = mySpaceOccupancies[myLastFreeLot];
    //     if (lsd.manoeuverAngle > 180.) {
    //         return DEG2RAD(lsd.manoeuverAngle - 360.);
    //     } else {
    //         return DEG2RAD(lsd.manoeuverAngle);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_last_free_lot_gui_angle(&self) -> f64 {
        todo!("port MSParkingArea::getLastFreeLotGUIAngle")
    }

    /// C++: `int MSParkingArea::getManoeuverAngle(const SUMOVehicle &) const` [const]
    /// Return the manoeuver angle of the lot where the vehicle is parked
    // int
    // MSParkingArea::getManoeuverAngle(const SUMOVehicle& forVehicle) const {
    //     for (const auto& lsd : mySpaceOccupancies) {
    //         if (lsd.vehicle == &forVehicle) {
    //             if (lsd.sideIsLHS) {
    //                 return abs(int(lsd.manoeuverAngle)) % 180;
    //             } else {
    //                 return abs(abs(int(lsd.manoeuverAngle)) % 180 - 180) % 180;
    //             }
    //         }
    //     }
    //     return 0;
    // }
    #[allow(unused_variables)]
    pub fn get_manoeuver_angle(&self, forVehicle: &SUMOVehicle) -> i32 {
        todo!("port MSParkingArea::getManoeuverAngle")
    }

    /// C++: `double MSParkingArea::getGUIAngle(const SUMOVehicle &) const` [const]
    /// Return the GUI angle of the lot where the vehicle is parked
    // double
    // MSParkingArea::getGUIAngle(const SUMOVehicle& forVehicle) const {
    //     for (const auto& lsd : mySpaceOccupancies) {
    //         if (lsd.vehicle == &forVehicle) {
    //             if (lsd.manoeuverAngle > 180.) {
    //                 return DEG2RAD(lsd.manoeuverAngle - 360.);
    //             } else {
    //                 return DEG2RAD(lsd.manoeuverAngle);
    //             }
    //         }
    //     }
    //     return 0.;
    // }
    #[allow(unused_variables)]
    pub fn get_gui_angle(&self, forVehicle: &SUMOVehicle) -> f64 {
        todo!("port MSParkingArea::getGUIAngle")
    }

    /// C++: `void MSParkingArea::addLotEntry(double, double, double, double, double, double, double)` [virtual]
    /// Add a lot entry to parking area
    // void
    // MSParkingArea::addLotEntry(double x, double y, double z, double width, double length, double angle, double slope) {
    //     // create LotSpaceDefinition
    //     LotSpaceDefinition lsd((int)mySpaceOccupancies.size(), nullptr, x, y, z, angle, slope, width, length);
    //     // If we are modelling parking set the end position to the lot position relative to the lane
    //     // rather than the end of the parking area - this results in vehicles stopping nearer the space
    //     // and re-entering the lane nearer the space. (If we are not modelling parking the vehicle will usually
    //     // enter the space and re-enter at the end of the parking area.)
    //     if (MSGlobals::gModelParkingManoeuver) {
    //         const double offset = this->getLane().getShape().nearest_offset_to_point2D(lsd.position);
    //         if (offset <  getBeginLanePosition()) {
    //             lsd.endPos =  getBeginLanePosition() + POSITION_EPS;
    //         } else {
    //             if (this->getLane().getLength() > offset) {
    //                 lsd.endPos = offset;
    //             } else {
    //                 lsd.endPos = this->getLane().getLength() - POSITION_EPS;
    //             }
    //         }
    //         // Work out the angle of the lot relative to the lane  (-90 adjusts for the way the bay is drawn )
    //         double relativeAngle = fmod(lsd.rotation - 90., 360) - fmod(RAD2DEG(this->getLane().getShape().rotationAtOffset(lsd.endPos)), 360) + 0.5;
    //         if (relativeAngle < 0.) {
    //             relativeAngle += 360.;
    //         }
    //         lsd.manoeuverAngle = relativeAngle;
    //
    //         // if p2.y is -ve the lot is on LHS of lane relative to lane direction
    //         // we need to know this because it inverts the complexity of the parking manoeuver
    //         Position p2 = this->getLane().getShape().transformToVectorCoordinates(lsd.position);
    //         if (p2.y() < (0. + POSITION_EPS)) {
    //             lsd.sideIsLHS = true;
    //         } else {
    //             lsd.sideIsLHS = false;
    //         }
    //     } else {
    //         lsd.endPos = myEndPos;
    //         lsd.manoeuverAngle = int(angle); // unused unless gModelParkingManoeuver is true
    //         lsd.sideIsLHS = true;
    //     }
    //     mySpaceOccupancies.push_back(lsd);
    //     myCapacity++;
    //     computeLastFreePos();
    // }
    #[allow(unused_variables)]
    pub fn add_lot_entry(&mut self, x: f64, y: f64, z: f64, width: f64, length: f64, angle: f64, slope: f64) {
        todo!("port MSParkingArea::addLotEntry")
    }

    /// C++: `double MSParkingArea::getWidth() const` [const]
    /// Returns the lot rectangle width
    // double
    // MSParkingArea::getWidth() const {
    //     return myWidth;
    // }
    #[allow(unused_variables)]
    pub fn get_width(&self) -> f64 {
        todo!("port MSParkingArea::getWidth")
    }

    /// C++: `double MSParkingArea::getLength() const` [const]
    /// Returns the lot rectangle length
    // double
    // MSParkingArea::getLength() const {
    //     return myLength;
    // }
    #[allow(unused_variables)]
    pub fn get_length(&self) -> f64 {
        todo!("port MSParkingArea::getLength")
    }

    /// C++: `double MSParkingArea::getAngle() const` [const]
    /// Returns the lot rectangle angle
    // double
    // MSParkingArea::getAngle() const {
    //     return myAngle;
    // }
    #[allow(unused_variables)]
    pub fn get_angle(&self) -> f64 {
        todo!("port MSParkingArea::getAngle")
    }

    /// C++: `void MSParkingArea::notifyEgressBlocked()`
    /// update state so that vehicles wishing to enter cooperate with exiting vehicles
    // void
    // MSParkingArea::notifyEgressBlocked() {
    //     computeLastFreePos();
    // }
    #[allow(unused_variables)]
    pub fn notify_egress_blocked(&mut self) {
        todo!("port MSParkingArea::notifyEgressBlocked")
    }

    /// C++: `int MSParkingArea::getNumAlternatives() const` [const]
    /// get number alternatives
    // int
    // MSParkingArea::getNumAlternatives() const {
    //     return myNumAlternatives;
    // }
    #[allow(unused_variables)]
    pub fn get_num_alternatives(&self) -> i32 {
        todo!("port MSParkingArea::getNumAlternatives")
    }

    /// C++: `void MSParkingArea::setNumAlternatives(int)`
    /// set number alternatives
    // void
    // MSParkingArea::setNumAlternatives(int alternatives) {
    //     myNumAlternatives = MAX2(myNumAlternatives, alternatives);
    // }
    #[allow(unused_variables)]
    pub fn set_num_alternatives(&mut self, alternatives: i32) {
        todo!("port MSParkingArea::setNumAlternatives")
    }

    /// C++: `std::vector<std::string> MSParkingArea::getAcceptedBadges() const` [const]
    /// get the accepted badges
    // std::vector<std::string>
    // MSParkingArea::getAcceptedBadges() const {
    //     std::vector<std::string> result(myAcceptedBadges.begin(), myAcceptedBadges.end());
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_accepted_badges(&self) -> Vec<String> {
        todo!("port MSParkingArea::getAcceptedBadges")
    }

    /// C++: `void MSParkingArea::setAcceptedBadges(const std::vector<std::string> &)`
    /// set the accepted badges
    // void
    // MSParkingArea::setAcceptedBadges(const std::vector<std::string>& badges) {
    //     myAcceptedBadges.clear();
    //     myAcceptedBadges.insert(badges.begin(), badges.end());
    // }
    #[allow(unused_variables)]
    pub fn set_accepted_badges(&mut self, badges: &Vec<String>) {
        todo!("port MSParkingArea::setAcceptedBadges")
    }

    /// C++: `const std::vector<LotSpaceDefinition> & MSParkingArea::getSpaceOccupancies() const` [const]
    /// get the parking lots (with occupancy)
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_space_occupancies(&self) -> &Vec<MSParkingArea_LotSpaceDefinition> {
        todo!("port MSParkingArea::getSpaceOccupancies")
    }

    /// C++: `const PositionVector & MSParkingArea::getShape() const` [const]
    /// get the parking shape
    // const PositionVector&
    // MSParkingArea::getShape() const {
    //     return myShape;
    // }
    #[allow(unused_variables)]
    pub fn get_shape(&self) -> &PositionVector {
        todo!("port MSParkingArea::getShape")
    }

    /// C++: `void MSParkingArea::setRoadsideCapacity(int)` [protected]
    /// overwrite the capacity (caution: will delete ANY previous parking space definitions)
    // void
    // MSParkingArea::setRoadsideCapacity(int capacity) {
    //     // reinit parking lot generation process
    //     myRoadSideCapacity = capacity;
    //
    //     // Initialize space occupancies if there is a road-side capacity
    //     // The overall number of lots is fixed and each lot accepts one vehicle regardless of size
    //     const double spaceDim = myRoadSideCapacity > 0 ? myLane.interpolateLanePosToGeometryPos((myEndPos - myBegPos) / myRoadSideCapacity) : 7.5;
    //     const double spaceOffset = myRoadSideCapacity > 0 ? (myEndPos - myBegPos) / myRoadSideCapacity : 7.5;
    //     if (myLength == 0) {
    //         myLength = spaceDim;
    //     }
    //     mySpaceOccupancies.clear();
    //     myCapacity = 0;
    //     for (int i = 0; i < myRoadSideCapacity; ++i) {
    //         // calculate pos, angle and slope of parking lot space
    //         const Position pos = GeomHelper::calculateLotSpacePosition(myShape, i, spaceDim, myAngle, myWidth, myLength);
    //         double spaceAngle = GeomHelper::calculateLotSpaceAngle(myShape, i, spaceDim, myAngle);
    //         double spaceSlope = GeomHelper::calculateLotSpaceSlope(myShape, i, spaceDim);
    //         // add lotEntry
    //         addLotEntry(pos.x(), pos.y(), pos.z(), myWidth, myLength, spaceAngle, spaceSlope);
    //         // update endPos
    //         mySpaceOccupancies.back().endPos = MIN2(myEndPos, myBegPos + MAX2(POSITION_EPS, spaceOffset * (i + 1)));
    //     }
    //     // recompute after modifying the last endPos
    //     computeLastFreePos();
    // }
    #[allow(unused_variables)]
    pub fn set_roadside_capacity(&mut self, capactity: i32) {
        todo!("port MSParkingArea::setRoadsideCapacity")
    }

    /// C++: `bool MSParkingArea::cannotChange(SUMOVehicleClass) const` [const, protected]
    /// whether overtaking on this lane is impossible for the given vehicle class
    // bool
    // MSParkingArea::cannotChange(SUMOVehicleClass svc) const {
    //     const MSLane* left = myLane.getParallelLane(1, false);
    //     const MSLane* right = myLane.getParallelLane(-1, false);
    //     return ((left == nullptr || !left->allowsVehicleClass(svc) || !myLane.allowsChangingLeft(svc))
    //         && (right == nullptr || !right->allowsVehicleClass(svc) || !myLane.allowsChangingRight(svc)));
    // }
    #[allow(unused_variables)]
    pub fn cannot_change(&self, svc: SUMOVehicleClass) -> bool {
        todo!("port MSParkingArea::cannotChange")
    }

    /// C++: `void MSParkingArea::computeLastFreePos()` [protected]
    /// Computes the last free position on this stop
    // void
    // MSParkingArea::computeLastFreePos() {
    //     myLastFreeLot = -1;
    //     myLastFreePos = myBegPos;
    //     myEgressBlocked = false;
    //     for (auto& lsd : mySpaceOccupancies) {
    //         if (lsd.vehicle == nullptr
    //                 || (getOccupancy() == getCapacity()
    //                     && lsd.vehicle->remainingStopDuration() <= 0
    //                     && !lsd.vehicle->isStoppedTriggered())) {
    //             if (lsd.vehicle == nullptr) {
    //                 myLastFreeLot = lsd.index;
    //                 myLastFreePos = lsd.endPos;
    //             } else {
    //                 // vehicle wants to exit the parking area
    //                 myLastFreeLot = lsd.index;
    //                 myLastFreePos = lsd.endPos - lsd.vehicle->getVehicleType().getLength() - POSITION_EPS;
    //                 myEgressBlocked = true;
    //             }
    //             break;
    //         } else {
    //             myLastFreePos = MIN2(myLastFreePos,
    //                                  lsd.endPos - lsd.vehicle->getVehicleType().getLength() - NUMERICAL_EPS);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn compute_last_free_pos(&mut self) {
        todo!("port MSParkingArea::computeLastFreePos")
    }

    /// C++: `void MSParkingArea::MSParkingArea(const MSParkingArea &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSParkingArea) -> Self {
        todo!("port MSParkingArea::MSParkingArea")
    }

    /// C++: `MSParkingArea & MSParkingArea::operator=(const MSParkingArea &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSParkingArea) -> &mut MSParkingArea {
        todo!("port MSParkingArea::operator=")
    }

}

