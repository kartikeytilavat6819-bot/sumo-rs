//! Port of microsim/MSStoppingPlace.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ enum `MSStoppingPlace::AccessExit`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum MSStoppingPlace_AccessExit {
    PLATFORM = 0,
    DOORS = 1,
    CARRIAGE = 2,
}

/// C++ class `MSStoppingPlace::Access` (microsim/MSStoppingPlace.h)
pub struct MSStoppingPlace_Access {
    pub lane: Ptr<MSLane>,
    pub startPos: f64,
    pub endPos: f64,
    pub length: f64,
    pub exit: MSStoppingPlace_AccessExit,
}

impl MSStoppingPlace_Access {
}

/// A lane area vehicles can halt at
/// C++ class `MSStoppingPlace` (microsim/MSStoppingPlace.h)
pub struct MSStoppingPlace {
    /// C++ base class `Named`
    pub base: Named,
    /// C++ base class `Parameterised`
    pub base2: Parameterised,
    /// the type of stopping place
    pub myElement: SumoXMLTag,
    /// The list of lines that are assigned to this stop
    pub myLines: Vec<String>,
    /// A map from objects (vehicles) to the areas they acquire after entering the stop
    pub myEndPositions: BTreeMap<Ptr<SUMOVehicle>, (f64, f64)>, // C++: std::map<const SUMOVehicle *, std::pair<double, double>, ComparatorNumericalIdLess>
    /// The lane this bus stop is located at
    pub myLane: Ptr<MSLane>,
    /// The begin position this bus stop is located at
    pub myBegPos: f64,
    /// The end position this bus stop is located at
    pub myEndPos: f64,
    /// The last free position at this stop (variable)
    pub myLastFreePos: f64,
    /// The length of the last parking vehicle (or 0 if there is none)
    pub myLastParking: Ptr<SUMOVehicle>,
    /// The name of the stopping place
    pub myName: String,
    /// The number of transportables that can wait here
    pub myTransportableCapacity: i32,
    /// the scaled space capacity for parking vehicles
    pub myParkingFactor: f64,
    /// The color of the stopping place
    pub myColor: RGBColor,
    /// The angle offset for waiting transportables
    pub myAngle: f64,
    /// row depth of waiting transportables
    pub myTransportableDepth: f64,
    /// the with of waiting transportables
    pub myTransportableWidth: f64,
    /// Persons waiting at this stop (mapped to waiting position)
    pub myWaitingTransportables: BTreeMap<Ptr<MSTransportable>, i32>,
    pub myWaitingSpots: BTreeSet<i32>,
    /// lanes and positions connected to this stop
    pub myAccessPos: Vec<MSStoppingPlace_Access>,
}

impl std::ops::Deref for MSStoppingPlace {
    type Target = Named;
    fn deref(&self) -> &Named {
        &self.base
    }
}
impl std::ops::DerefMut for MSStoppingPlace {
    fn deref_mut(&mut self) -> &mut Named {
        &mut self.base
    }
}

impl MSStoppingPlace {
    /// C++: `void MSStoppingPlace::MSStoppingPlace(const std::string &, SumoXMLTag, const std::vector<std::string> &, MSLane &, double, double, const std::string, int, double, const RGBColor &, double)`
    /// Constructor
    // MSStoppingPlace::MSStoppingPlace(const std::string& id,
    //                                  SumoXMLTag element,
    //                                  const std::vector<std::string>& lines,
    //                                  MSLane& lane,
    //                                  double begPos, double endPos, const std::string name,
    //                                  int capacity,
    //                                  double parkingLength,
    //                                  const RGBColor& color,
    //                                  double angle) :
    //     Named(id),
    //     myElement(element),
    //     myLines(lines), myLane(lane),
    //     myBegPos(begPos), myEndPos(endPos),
    //     myLastFreePos(endPos),
    //     myLastParking(nullptr),
    //     myName(name),
    //     myTransportableCapacity(capacity),
    //     myParkingFactor(parkingLength <= 0 ? 1 : (endPos - begPos) / parkingLength),
    //     myColor(color),
    //     myAngle(DEG2RAD(angle)),
    //     // see MSVehicleControl defContainerType
    //     myTransportableDepth(element == SUMO_TAG_CONTAINER_STOP ? SUMO_const_waitingContainerDepth : SUMO_const_waitingPersonDepth),
    //     myTransportableWidth(getDefaultTransportableWidth(myElement)) {
    //     computeLastFreePos();
    //     for (int i = 0; i < capacity; i++) {
    //         myWaitingSpots.insert(i);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn new(id: &str, element: SumoXMLTag, lines: &Vec<String>, lane: &mut MSLane, begPos: f64, endPos: f64, name: String, capacity: i32, parkingLength: f64, color: &RGBColor, angle: f64) -> Self {
        let mut sp = MSStoppingPlace {
            base: Named::new(id),
            base2: Parameterised::new(),
            myElement: element,
            myLines: lines.clone(),
            myEndPositions: BTreeMap::new(),
            myLane: Ptr::from_mut(lane),
            myBegPos: begPos,
            myEndPos: endPos,
            myLastFreePos: endPos,
            myLastParking: Ptr::null(),
            myName: name,
            myTransportableCapacity: capacity,
            myParkingFactor: if parkingLength <= 0.0 { 1.0 } else { (endPos - begPos) / parkingLength },
            myColor: color.clone(),
            myAngle: angle * std::f64::consts::PI / 180.0,
            // see MSVehicleControl defContainerType
            myTransportableDepth: if element == SUMO_TAG_CONTAINER_STOP {
                utils::common::std_defs::SUMO_CONST_WAITING_CONTAINER_DEPTH
            } else {
                utils::common::std_defs::SUMO_CONST_WAITING_PERSON_DEPTH
            },
            myTransportableWidth: MSStoppingPlace::get_default_transportable_width(element),
            myWaitingTransportables: BTreeMap::new(),
            myWaitingSpots: BTreeSet::new(),
            myAccessPos: Vec::new(),
        };
        sp.compute_last_free_pos();
        for i in 0..capacity {
            sp.myWaitingSpots.insert(i);
        }
        sp
    }

    /// C++: `void MSStoppingPlace::~MSStoppingPlace() noexcept` [virtual]
    /// Destructor
    // MSStoppingPlace::~MSStoppingPlace() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor is empty
    }

    /// C++: `const MSLane & MSStoppingPlace::getLane() const` [const]
    /// Returns the lane this stop is located at
    // const MSLane&
    // MSStoppingPlace::getLane() const {
    //     return myLane;
    // }
    #[allow(unused_variables)]
    pub fn get_lane(&self) -> &MSLane {
        self.myLane.get().unwrap()
    }

    /// C++: `double MSStoppingPlace::getBeginLanePosition() const` [const]
    /// Returns the begin position of this stop
    // double
    // MSStoppingPlace::getBeginLanePosition() const {
    //     return myBegPos;
    // }
    #[allow(unused_variables)]
    pub fn get_begin_lane_position(&self) -> f64 {
        self.myBegPos
    }

    /// C++: `double MSStoppingPlace::getEndLanePosition() const` [const]
    /// Returns the end position of this stop
    // double
    // MSStoppingPlace::getEndLanePosition() const {
    //     return myEndPos;
    // }
    #[allow(unused_variables)]
    pub fn get_end_lane_position(&self) -> f64 {
        self.myEndPos
    }

    /// C++: `double MSStoppingPlace::getAngle() const` [const]
    // double getAngle() const {
    //         return myAngle;
    //     }
    #[allow(unused_variables)]
    pub fn get_angle(&self) -> f64 {
        self.myAngle
    }

    /// C++: `Position MSStoppingPlace::getCenterPos() const` [const]
    /// the position in the middle of the stop shape
    // Position
    // MSStoppingPlace::getCenterPos() const {
    //     return myLane.getShape().positionAtOffset(myLane.interpolateLanePosToGeometryPos((myBegPos + myEndPos) / 2),
    //             myLane.getWidth() / 2 + 0.5);
    // }
    #[allow(unused_variables)]
    pub fn get_center_pos(&self) -> Position {
        let lane = self.myLane;
        lane.get_shape().position_at_offset(
            lane.interpolate_lane_pos_to_geometry_pos((self.myBegPos + self.myEndPos) / 2.0),
            lane.get_width() / 2.0 + 0.5,
        )
    }

    /// C++: `void MSStoppingPlace::enter(SUMOVehicle *, const bool)` [virtual]
    /// Called if a vehicle enters this stop
    // void
    // MSStoppingPlace::enter(SUMOVehicle* veh, const bool parking) {
    //     const double beg = veh->getPositionOnLane() + veh->getVehicleType().getMinGap() * (parking ? myParkingFactor : 1);
    //     const double end = beg - veh->getVehicleType().getLengthWithGap() * (parking ? myParkingFactor : 1);
    //     myEndPositions[veh] = std::make_pair(beg, end);
    //     computeLastFreePos();
    // }
    #[allow(unused_variables)]
    pub fn enter(&mut self, veh: Ptr<SUMOVehicle>, parking: bool) {
        let factor = if parking { self.myParkingFactor } else { 1.0 };
        let beg = veh.get_position_on_lane() + veh.get_vehicle_type().get_min_gap() * factor;
        let end = beg - veh.get_vehicle_type().get_length_with_gap() * factor;
        self.myEndPositions.insert(veh, (beg, end));
        self.compute_last_free_pos();
    }

    /// C++: `void MSStoppingPlace::leaveFrom(SUMOVehicle *)` [virtual]
    /// Called if a vehicle leaves this stop
    // void
    // MSStoppingPlace::leaveFrom(SUMOVehicle* what) {
    //     assert(myEndPositions.find(what) != myEndPositions.end());
    //     myEndPositions.erase(myEndPositions.find(what));
    //     computeLastFreePos();
    // }
    #[allow(unused_variables)]
    pub fn leave_from(&mut self, what: Ptr<SUMOVehicle>) {
        self.myEndPositions.remove(&what);
        self.compute_last_free_pos();
    }

    /// C++: `double MSStoppingPlace::getLastFreePos(const SUMOVehicle &, double) const` [const, virtual]
    /// Returns the last free position on this stop
    // double
    // MSStoppingPlace::getLastFreePos(const SUMOVehicle& forVehicle, double /*brakePos*/) const {
    //     if (getStoppedVehicleNumber() > 0) {
    //         const double vehGap = forVehicle.getVehicleType().getMinGap();
    //         double pos = myLastFreePos - vehGap - NUMERICAL_EPS;
    //         if (myParkingFactor < 1 && myLastParking != nullptr && forVehicle.hasStops() && (forVehicle.getStops().front().pars.parking == ParkingType::ONROAD)
    //                 && myLastParking->remainingStopDuration() < forVehicle.getStops().front().getMinDuration(SIMSTEP)) {
    //             // stop far back enough so that the previous parking vehicle can leave (even if this vehicle fits, it will
    //             // be a blocker because it stops on the road)
    //             pos = MIN2(pos, myLastParking->getPositionOnLane() - myLastParking->getLength() - vehGap - NUMERICAL_EPS);
    //         }
    //         if (!fits(pos, forVehicle)) {
    //             // try to find a place ahead of the waiting vehicles
    //             const double vehLength = forVehicle.getVehicleType().getLength() * myParkingFactor;
    //             std::vector<std::pair<double, std::pair<double, const SUMOVehicle*> > > spaces;
    //             for (auto it : myEndPositions) {
    //                 spaces.push_back(std::make_pair(it.second.first, std::make_pair(it.second.second, it.first)));
    //             }
    //             // sorted from myEndPos towars myBegPos
    //             std::sort(spaces.begin(), spaces.end());
    //             std::reverse(spaces.begin(), spaces.end());
    //             double prev = myEndPos;
    //             for (auto it : spaces) {
    //                 //if (forVehicle.isSelected()) {
    //                 //    std::cout << SIMTIME << " fitPosFor " << forVehicle.getID() << " l=" << vehLength << " prev=" << prev << " vehBeg=" << it.first << " vehEnd=" << it.second.first << " found=" << (prev - it.first >= vehLength) << "\n";
    //                 //}
    //                 if (prev - it.first + NUMERICAL_EPS >= vehLength && (
    //                             it.second.second->isParking()
    //                             || it.second.second->remainingStopDuration() > TIME2STEPS(10))) {
    //                     return prev;
    //                 }
    //                 prev = it.second.first - vehGap;
    //             }
    //             if (myParkingFactor < 1 && myLastParking != nullptr) {
    //                 // stop far back enough so that the previous vehicle can leave
    //                 pos = MIN2(pos, myLastParking->getPositionOnLane() - myLastParking->getLength() - vehGap - NUMERICAL_EPS);
    //             }
    //         }
    //         return pos;
    //     }
    //     return myLastFreePos;
    // }
    #[allow(unused_variables)]
    pub fn get_last_free_pos(&self, forVehicle: &SUMOVehicle, brakePos: f64) -> f64 {
        if self.get_stopped_vehicle_number() > 0 {
            let vehGap = forVehicle.get_vehicle_type().get_min_gap();
            let mut pos = self.myLastFreePos - vehGap - utils::geom::NUMERICAL_EPS;
            if self.myParkingFactor < 1.0
                && !self.myLastParking.is_null()
                && forVehicle.has_stops()
                && forVehicle.get_stops().front().unwrap().pars.parking == ParkingType::ONROAD
                && self.myLastParking.remaining_stop_duration()
                    < forVehicle.get_stops().front().unwrap().get_min_duration(MSNet::get_instance().m().get_current_time_step())
            {
                // stop far back enough so that the previous parking vehicle can leave (even if this vehicle fits, it will
                // be a blocker because it stops on the road)
                pos = f64::min(
                    pos,
                    self.myLastParking.get_position_on_lane() - self.myLastParking.get_length() - vehGap - utils::geom::NUMERICAL_EPS,
                );
            }
            if !self.fits(pos, forVehicle) {
                // try to find a place ahead of the waiting vehicles
                let vehLength = forVehicle.get_vehicle_type().get_length() * self.myParkingFactor;
                let mut spaces: Vec<(f64, (f64, Ptr<SUMOVehicle>))> = self
                    .myEndPositions
                    .iter()
                    .map(|(veh, (beg, end))| (*beg, (*end, *veh)))
                    .collect();
                // sorted from myEndPos towars myBegPos
                spaces.sort_by(|a, b| a.partial_cmp(b).unwrap());
                spaces.reverse();
                let mut prev = self.myEndPos;
                for it in spaces.iter() {
                    let (begin, (end, veh)) = (it.0, it.1);
                    if prev - begin + utils::geom::NUMERICAL_EPS >= vehLength
                        && (veh.is_parking() || veh.remaining_stop_duration() > utils::common::time::time2steps(10.0))
                    {
                        return prev;
                    }
                    prev = end - vehGap;
                }
                if self.myParkingFactor < 1.0 && !self.myLastParking.is_null() {
                    // stop far back enough so that the previous vehicle can leave
                    pos = f64::min(
                        pos,
                        self.myLastParking.get_position_on_lane() - self.myLastParking.get_length() - vehGap - utils::geom::NUMERICAL_EPS,
                    );
                }
            }
            return pos;
        }
        self.myLastFreePos
    }

    /// C++: `bool MSStoppingPlace::accepts(SUMOVehicle *) const` [const, virtual]
    // virtual bool accepts(SUMOVehicle* /*veh*/) const {
    //         return true;
    //     }
    #[allow(unused_variables)]
    pub fn accepts(&self, arg0: Ptr<SUMOVehicle>) -> bool {
        let _ = arg0;
        true
    }

    /// C++: `bool MSStoppingPlace::fits(double, const SUMOVehicle &) const` [const]
    /// return whether the given vehicle fits at the given position
    // bool
    // MSStoppingPlace::fits(double pos, const SUMOVehicle& veh) const {
    //     // always fit at the default position or if at least half the vehicle length
    //     // is within the stop range (debatable)
    //     return pos + POSITION_EPS >= myEndPos || (pos - myBegPos >= veh.getVehicleType().getLength() * myParkingFactor / 2);
    // }
    #[allow(unused_variables)]
    pub fn fits(&self, pos: f64, veh: &SUMOVehicle) -> bool {
        // always fit at the default position or if at least half the vehicle length
        // is within the stop range (debatable)
        pos + utils::geom::POSITION_EPS >= self.myEndPos
            || (pos - self.myBegPos >= veh.get_vehicle_type().get_length() * self.myParkingFactor / 2.0)
    }

    /// C++: `Position MSStoppingPlace::getWaitPosition(MSTransportable *) const` [const]
    /// Returns the next free waiting place for pedestrians / containers
    // Position
    // MSStoppingPlace::getWaitPosition(MSTransportable* t) const {
    //     double lanePos = getWaitingPositionOnLane(t);
    //     int row = 0;
    //     auto it = myWaitingTransportables.find(t);
    //     if (it != myWaitingTransportables.end()) {
    //         if (it->second >= 0) {
    //             row = int(it->second / getTransportablesAbreast());
    //         } else {
    //             // invalid position, draw outside bounds
    //             row = 1 + myTransportableCapacity / getTransportablesAbreast();
    //         }
    //     }
    //     const double lefthandSign = (MSGlobals::gLefthand ? -1 : 1);
    //     return myLane.getShape().positionAtOffset(myLane.interpolateLanePosToGeometryPos(lanePos),
    //             lefthandSign * (myLane.getWidth() / 2 + row * myTransportableDepth + fabs(cos(myAngle)) * myTransportableWidth / 2));
    // }
    #[allow(unused_variables)]
    pub fn get_wait_position(&self, person: Ptr<MSTransportable>) -> Position {
        let lanePos = self.get_waiting_position_on_lane(person);
        let mut row: i32 = 0;
        if let Some(&spot) = self.myWaitingTransportables.get(&person) {
            if spot >= 0 {
                row = spot / self.get_transportables_abreast();
            } else {
                // invalid position, draw outside bounds
                row = 1 + self.myTransportableCapacity / self.get_transportables_abreast();
            }
        }
        let lefthandSign = if MSGlobals::gLefthand { -1.0 } else { 1.0 };
        let lane = self.myLane;
        lane.get_shape().position_at_offset(
            lane.interpolate_lane_pos_to_geometry_pos(lanePos),
            lefthandSign
                * (lane.get_width() / 2.0 + row as f64 * self.myTransportableDepth + self.myAngle.cos().abs() * self.myTransportableWidth / 2.0),
        )
    }

    /// C++: `double MSStoppingPlace::getWaitingPositionOnLane(MSTransportable *) const` [const]
    /// Returns the lane position corresponding to getWaitPosition()
    // double
    // MSStoppingPlace::getWaitingPositionOnLane(MSTransportable* t) const {
    //     auto it = myWaitingTransportables.find(t);
    //     if (it != myWaitingTransportables.end() && it->second >= 0) {
    //         return myEndPos - (0.5 + (it->second) % getTransportablesAbreast()) * myTransportableWidth;
    //     } else {
    //         return (myEndPos + myBegPos) / 2;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_waiting_position_on_lane(&self, t: Ptr<MSTransportable>) -> f64 {
        if let Some(&spot) = self.myWaitingTransportables.get(&t) {
            if spot >= 0 {
                return self.myEndPos - (0.5 + (spot % self.get_transportables_abreast()) as f64) * self.myTransportableWidth;
            }
        }
        (self.myEndPos + self.myBegPos) / 2.0
    }

    /// C++: `double MSStoppingPlace::getStoppingPosition(const SUMOVehicle *) const` [const]
    /// For vehicles at the stop this gives the actual stopping position of the vehicle. For all others the last free stopping position
    // double
    // MSStoppingPlace::getStoppingPosition(const SUMOVehicle* veh) const {
    //     auto i = myEndPositions.find(veh);
    //     if (i != myEndPositions.end()) {
    //         return i->second.second;
    //     } else {
    //         return getLastFreePos(*veh);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_stopping_position(&self, veh: Ptr<SUMOVehicle>) -> f64 {
        if let Some((_, end)) = self.myEndPositions.get(&veh) {
            return *end;
        }
        self.get_last_free_pos(&veh, 0.0)
    }

    /// C++: `int MSStoppingPlace::getTransportableNumber() const` [const]
    /// Returns the number of transportables waiting on this stop
    // int getTransportableNumber() const {
    //         return (int)myWaitingTransportables.size();
    //     }
    #[allow(unused_variables)]
    pub fn get_transportable_number(&self) -> i32 {
        self.myWaitingTransportables.len() as i32
    }

    /// C++: `std::vector<const MSTransportable *> MSStoppingPlace::getTransportables() const` [const]
    /// Returns the transportables waiting on this stop
    // std::vector<const MSTransportable*>
    // MSStoppingPlace::getTransportables() const {
    //     std::vector<const MSTransportable*> result;
    //     for (auto item : myWaitingTransportables) {
    //         result.push_back(item.first);
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_transportables(&self) -> Vec<Ptr<MSTransportable>> {
        self.myWaitingTransportables.keys().copied().collect()
    }

    /// C++: `int MSStoppingPlace::getStoppedVehicleNumber() const` [const]
    /// Returns the number of stopped vehicles waiting on this stop
    // int getStoppedVehicleNumber() const {
    //         return (int)myEndPositions.size();
    //     }
    #[allow(unused_variables)]
    pub fn get_stopped_vehicle_number(&self) -> i32 {
        self.myEndPositions.len() as i32
    }

    /// C++: `double MSStoppingPlace::getLastFreePos() const` [const]
    // double getLastFreePos() const {
    //         return myLastFreePos;
    //     }
    #[allow(unused_variables)]
    pub fn get_last_free_pos_2(&self) -> f64 {
        self.myLastFreePos
    }

    /// C++: `bool MSStoppingPlace::hasSpaceForTransportable() const` [const]
    /// whether there is still capacity for more transportables
    // bool
    // MSStoppingPlace::hasSpaceForTransportable() const {
    //     return myWaitingSpots.size() > 0;
    // }
    #[allow(unused_variables)]
    pub fn has_space_for_transportable(&self) -> bool {
        !self.myWaitingSpots.is_empty()
    }

    /// C++: `bool MSStoppingPlace::addTransportable(const MSTransportable *, int)`
    /// adds a transportable to this stop
    // bool
    // MSStoppingPlace::addTransportable(const MSTransportable* p, int spot) {
    //     if (spot < 0 && !hasSpaceForTransportable()) {
    //         return false;
    //     }
    //     if (spot < 0) {
    //         spot = *myWaitingSpots.begin();
    //     }
    //     myWaitingSpots.erase(spot);
    //     myWaitingTransportables[p] = spot;
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn add_transportable(&mut self, p: Ptr<MSTransportable>, spot: i32) -> bool {
        let mut spot = spot;
        if spot < 0 && !self.has_space_for_transportable() {
            return false;
        }
        if spot < 0 {
            spot = *self.myWaitingSpots.iter().next().unwrap();
        }
        self.myWaitingSpots.remove(&spot);
        self.myWaitingTransportables.insert(p, spot);
        true
    }

    /// C++: `void MSStoppingPlace::removeTransportable(const MSTransportable *)`
    /// Removes a transportable from this stop
    // void
    // MSStoppingPlace::removeTransportable(const MSTransportable* p) {
    //     auto i = myWaitingTransportables.find(p);
    //     if (i != myWaitingTransportables.end()) {
    //         if (i->second >= 0) {
    //             myWaitingSpots.insert(i->second);
    //         }
    //         myWaitingTransportables.erase(i);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn remove_transportable(&mut self, p: Ptr<MSTransportable>) {
        if let Some(&spot) = self.myWaitingTransportables.get(&p) {
            if spot >= 0 {
                self.myWaitingSpots.insert(spot);
            }
            self.myWaitingTransportables.remove(&p);
        }
    }

    /// C++: `int MSStoppingPlace::checkWaitingSpot(const MSTransportable *) const` [const]
    // int
    // MSStoppingPlace::checkWaitingSpot(const MSTransportable* p) const {
    //     auto i = myWaitingTransportables.find(p);
    //     if (i != myWaitingTransportables.end()) {
    //         return i->second;
    //     } else {
    //         return -1;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn check_waiting_spot(&self, p: Ptr<MSTransportable>) -> i32 {
        match self.myWaitingTransportables.get(&p) {
            Some(&spot) => spot,
            None => -1,
        }
    }

    /// C++: `bool MSStoppingPlace::addAccess(MSLane *const, const double, const double, double, const MSStoppingPlace::AccessExit)` [virtual]
    /// adds an access point to this stop
    // bool
    // MSStoppingPlace::addAccess(MSLane* const lane, const double startPos, const double endPos, double length, const MSStoppingPlace::AccessExit exit) {
    //     // prevent multiple accesses on the same lane
    //     for (const auto& access : myAccessPos) {
    //         if (lane == access.lane) {
    //             return false;
    //         }
    //     }
    //     if (length < 0.) {
    //         const Position accPos = lane->geometryPositionAtOffset((startPos + endPos) / 2.);
    //         const Position stopPos = myLane.geometryPositionAtOffset((myBegPos + myEndPos) / 2.);
    //         length = accPos.distanceTo(stopPos);
    //     }
    //     myAccessPos.push_back({lane, startPos, endPos, length, exit});
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn add_access(&mut self, lane: Ptr<MSLane>, startPos: f64, endPos: f64, length: f64, exit: MSStoppingPlace_AccessExit) -> bool {
        // prevent multiple accesses on the same lane
        for access in self.myAccessPos.iter() {
            if lane == access.lane {
                return false;
            }
        }
        let mut length = length;
        if length < 0.0 {
            let accPos = lane.geometry_position_at_offset((startPos + endPos) / 2.0);
            let stopPos = self.myLane.geometry_position_at_offset((self.myBegPos + self.myEndPos) / 2.0);
            length = accPos.distance_to(&stopPos);
        }
        self.myAccessPos.push(MSStoppingPlace_Access {
            lane,
            startPos,
            endPos,
            length,
            exit,
        });
        true
    }

    /// C++: `const std::vector<Access> & MSStoppingPlace::getAllAccessPos() const` [const]
    /// lanes and positions connected to this stop
    // const std::vector<Access>& getAllAccessPos() const {
    //         return myAccessPos;
    //     }
    #[allow(unused_variables)]
    pub fn get_all_access_pos(&self) -> &Vec<MSStoppingPlace_Access> {
        &self.myAccessPos
    }

    /// C++: `double MSStoppingPlace::getAccessPos(const MSEdge *, SumoRNG *) const` [const]
    /// the position on the given edge which is connected to this stop, -1 on failure
    // double
    // MSStoppingPlace::getAccessPos(const MSEdge* edge, SumoRNG* rng) const {
    //     if (edge == &myLane.getEdge()) {
    //         return (myBegPos + myEndPos) / 2.;
    //     }
    //     for (const auto& access : myAccessPos) {
    //         if (edge == &access.lane->getEdge()) {
    //             if (rng == nullptr || access.startPos == access.endPos) {
    //                 return access.endPos;
    //             }
    //             return RandHelper::rand(access.startPos, access.endPos, rng);
    //         }
    //     }
    //     return -1.;
    // }
    #[allow(unused_variables)]
    pub fn get_access_pos(&self, edge: Ptr<MSEdge>, rng: Ptr<SumoRNG>) -> f64 {
        if edge == Ptr::from_mut(self.myLane.m().get_edge()) {
            return (self.myBegPos + self.myEndPos) / 2.0;
        }
        for access in self.myAccessPos.iter() {
            if edge == Ptr::from_mut(access.lane.m().get_edge()) {
                if rng.is_null() || access.startPos == access.endPos {
                    return access.endPos;
                }
                return rng.m().rand_range(access.startPos, access.endPos);
            }
        }
        -1.0
    }

    /// C++: `const Access * MSStoppingPlace::getAccess(const MSEdge *) const` [const]
    /// the access on the given edge to the stop, nullptr if there is none
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_access(&self, edge: Ptr<MSEdge>) -> Ptr<MSStoppingPlace_Access> {
        // C++ getAccess is declared in the header only (no .cpp body); same lookup as getAccessPos
        for access in self.myAccessPos.iter() {
            if edge == Ptr::from_mut(access.lane.m().get_edge()) {
                return Ptr::new(access);
            }
        }
        Ptr::null()
    }

    /// C++: `const std::string & MSStoppingPlace::getMyName() const` [const]
    // const std::string&
    // MSStoppingPlace::getMyName() const {
    //     return myName;
    // }
    #[allow(unused_variables)]
    pub fn get_my_name(&self) -> &str {
        &self.myName
    }

    /// C++: `SumoXMLTag MSStoppingPlace::getElement() const` [const]
    /// return the type of this stopping place
    // SumoXMLTag getElement() const {
    //         return myElement;
    //     }
    #[allow(unused_variables)]
    pub fn get_element(&self) -> SumoXMLTag {
        self.myElement
    }

    /// C++: `const RGBColor & MSStoppingPlace::getColor() const` [const]
    // const RGBColor&
    // MSStoppingPlace::getColor() const {
    //     return myColor;
    // }
    #[allow(unused_variables)]
    pub fn get_color(&self) -> &RGBColor {
        &self.myColor
    }

    /// C++: `int MSStoppingPlace::getDefaultTransportablesAbreast(double, SumoXMLTag)` [static]
    // int
    // MSStoppingPlace::getDefaultTransportablesAbreast(double length, SumoXMLTag element) {
    //     return MAX2(1, (int)floor(length / getDefaultTransportableWidth(element)));
    // }
    #[allow(unused_variables)]
    pub fn get_default_transportables_abreast(length: f64, element: SumoXMLTag) -> i32 {
        std::cmp::max(1, (length / MSStoppingPlace::get_default_transportable_width(element)).floor() as i32)
    }

    /// C++: `std::vector<const SUMOVehicle *> MSStoppingPlace::getStoppedVehicles() const` [const]
    /// get list of vehicles waiting at this stop
    // std::vector<const SUMOVehicle*>
    // MSStoppingPlace::getStoppedVehicles() const {
    //     std::vector<const SUMOVehicle*> result;
    //     for (auto item : myEndPositions) {
    //         result.push_back(item.first);
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_stopped_vehicles(&self) -> Vec<Ptr<SUMOVehicle>> {
        self.myEndPositions.keys().copied().collect()
    }

    /// C++: `int MSStoppingPlace::getNumWaitingPersons() const` [const]
    /// get number of persons waiting at this stop
    // inline int getNumWaitingPersons() const {
    //         return (int)myWaitingTransportables.size();
    //     }
    #[allow(unused_variables)]
    pub fn get_num_waiting_persons(&self) -> i32 {
        self.myWaitingTransportables.len() as i32
    }

    /// C++: `int MSStoppingPlace::getWaitingCapacity() const` [const]
    /// get number of persons that can wait at this stop
    // inline int getWaitingCapacity() const {
    //         return myTransportableCapacity;
    //     }
    #[allow(unused_variables)]
    pub fn get_waiting_capacity(&self) -> i32 {
        self.myTransportableCapacity
    }

    /// C++: `double MSStoppingPlace::getParkingLength() const` [const]
    // inline double getParkingLength() const {
    //         return (myEndPos - myBegPos) / myParkingFactor;
    //     }
    #[allow(unused_variables)]
    pub fn get_parking_length(&self) -> f64 {
        (self.myEndPos - self.myBegPos) / self.myParkingFactor
    }

    /// C++: `void MSStoppingPlace::getWaitingPersonIDs(std::vector<std::string> &) const` [const]
    /// get IDs of persons waiting at this stop
    // void
    // MSStoppingPlace::getWaitingPersonIDs(std::vector<std::string>& into) const {
    //     for (auto item : myWaitingTransportables) {
    //         into.push_back(item.first->getID());
    //     }
    //     std::sort(into.begin(), into.end());
    // }
    #[allow(unused_variables)]
    pub fn get_waiting_person_i_ds(&self, into: &mut Vec<String>) {
        for item in self.myWaitingTransportables.keys() {
            into.push(item.get_id().to_string());
        }
        into.sort();
    }

    /// C++: `bool MSStoppingPlace::checkPersonCapacity() const` [const]
    // bool checkPersonCapacity() const {
    //         return myElement == SUMO_TAG_BUS_STOP || myElement == SUMO_TAG_TRAIN_STOP;;
    //     }
    #[allow(unused_variables)]
    pub fn check_person_capacity(&self) -> bool {
        self.myElement == SUMO_TAG_BUS_STOP || self.myElement == SUMO_TAG_TRAIN_STOP
    }

    /// C++: `void MSStoppingPlace::finishedLoading()` [virtual]
    /// perform extra processing after element has been loaded
    // void
    // MSStoppingPlace::finishedLoading() {
    //     const std::string waitingWidth = getParameter("waitingWidth");
    //     if (waitingWidth != "") {
    //         try {
    //             myTransportableWidth = StringUtils::toDouble(waitingWidth);
    //         } catch (ProcessError& e) {
    //             WRITE_WARNINGF("Could not load waitingWidth (m) '%' (%)", waitingWidth, e.what());
    //         }
    //     }
    //     const std::string waitingDepth = getParameter("waitingDepth");
    //     if (waitingDepth != "") {
    //         try {
    //             myTransportableDepth = StringUtils::toDouble(waitingDepth);
    //         } catch (ProcessError& e) {
    //             WRITE_WARNINGF("Could not load waitingDepth (m) '%' (%)", waitingWidth, e.what());
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn finished_loading(&mut self) {
        let waitingWidth = self.base2.get_parameter("waitingWidth", "");
        if waitingWidth != "" {
            match utils::common::string_utils::to_double(&waitingWidth) {
                Ok(v) => self.myTransportableWidth = v,
                Err(e) => write_warningf!("Could not load waitingWidth (m) '%' (%)", waitingWidth, e.to_string()),
            }
        }
        let waitingDepth = self.base2.get_parameter("waitingDepth", "");
        if waitingDepth != "" {
            match utils::common::string_utils::to_double(&waitingDepth) {
                Ok(v) => self.myTransportableDepth = v,
                // the C++ message prints waitingWidth here, kept identical
                Err(e) => write_warningf!("Could not load waitingDepth (m) '%' (%)", waitingWidth, e.to_string()),
            }
        }
    }

    /// C++: `void MSStoppingPlace::clearState()`
    /// Remove all vehicles before quick-loading state
    // void
    // MSStoppingPlace::clearState() {
    //     myEndPositions.clear();
    //     myWaitingTransportables.clear();
    //     computeLastFreePos();
    // }
    #[allow(unused_variables)]
    pub fn clear_state(&mut self) {
        self.myEndPositions.clear();
        self.myWaitingTransportables.clear();
        self.compute_last_free_pos();
    }

    /// C++: `void MSStoppingPlace::computeLastFreePos()` [protected]
    /// Computes the last free position on this stop
    // void
    // MSStoppingPlace::computeLastFreePos() {
    //     myLastFreePos = myEndPos;
    //     myLastParking = nullptr;
    //     for (auto item : myEndPositions) {
    //         // vehicle might be stopped beyond myEndPos
    //         if (myLastFreePos >= item.second.second || myLastFreePos == myEndPos) {
    //             myLastFreePos = item.second.second;
    //             if (item.first->isStoppedParking()) {
    //                 myLastParking = item.first;
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn compute_last_free_pos(&mut self) {
        self.myLastFreePos = self.myEndPos;
        self.myLastParking = Ptr::null();
        for (veh, (_beg, end)) in self.myEndPositions.iter() {
            // vehicle might be stopped beyond myEndPos
            if self.myLastFreePos >= *end || self.myLastFreePos == self.myEndPos {
                self.myLastFreePos = *end;
                if veh.is_stopped_parking() {
                    self.myLastParking = *veh;
                }
            }
        }
    }

    /// C++: `int MSStoppingPlace::getTransportablesAbreast() const` [const, protected]
    // int
    // MSStoppingPlace::getTransportablesAbreast() const {
    //     return MAX2(1, (int)floor((myEndPos - myBegPos) / myTransportableWidth));
    // }
    #[allow(unused_variables)]
    pub fn get_transportables_abreast(&self) -> i32 {
        std::cmp::max(1, ((self.myEndPos - self.myBegPos) / self.myTransportableWidth).floor() as i32)
    }

    /// C++: `double MSStoppingPlace::getDefaultTransportableWidth(SumoXMLTag)` [static, protected]
    // double
    // MSStoppingPlace::getDefaultTransportableWidth(SumoXMLTag element) {
    //     return element == SUMO_TAG_CONTAINER_STOP
    //            ? SUMO_const_waitingContainerWidth
    //            : SUMO_const_waitingPersonWidth;
    //
    // }
    #[allow(unused_variables)]
    pub fn get_default_transportable_width(element: SumoXMLTag) -> f64 {
        if element == SUMO_TAG_CONTAINER_STOP {
            utils::common::std_defs::SUMO_CONST_WAITING_CONTAINER_WIDTH
        } else {
            utils::common::std_defs::SUMO_CONST_WAITING_PERSON_WIDTH
        }
    }

    /// C++: `void MSStoppingPlace::MSStoppingPlace(const MSStoppingPlace &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSStoppingPlace) -> Self {
        // C++ copy constructor is deleted ("Invalidated"), copying is a programming error
        panic!("MSStoppingPlace copy constructor is invalidated in C++ and must not be used");
    }

    /// C++: `MSStoppingPlace & MSStoppingPlace::operator=(const MSStoppingPlace &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSStoppingPlace) -> &mut MSStoppingPlace {
        // C++ assignment operator is deleted ("Invalidated"), assignment is a programming error
        panic!("MSStoppingPlace assignment operator is invalidated in C++ and must not be used");
    }

}

