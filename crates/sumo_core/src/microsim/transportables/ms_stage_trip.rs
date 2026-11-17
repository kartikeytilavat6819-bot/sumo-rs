//! Port of microsim/transportables/MSStageTrip.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

use utils::common::msg_handler::write_warning;
use utils::options::options_cont;

const VEHPARS_VTYPE_SET: i64 = 2;
const VEHPARS_DEPARTPOS_SET: i64 = 2 << 2;
const VEHPARS_ARRIVALPOS_SET: i64 = 2 << 12;
const VEHPARS_ARRIVALSPEED_SET: i64 = 2 << 13;


/// A "placeholder" stage storing routing info which will result in real stages when routed
/// C++ class `MSStageTrip` (microsim/transportables/MSStageTrip.h)
#[repr(C)]
pub struct MSStageTrip {
    /// C++ base class `MSStage`
    pub base: MSStage,
    /// the origin edge
    pub myOrigin: Ptr<MSEdge>,
    /// the origin edge
    pub myOriginStop: Ptr<MSStoppingPlace>,
    /// the time the trip should take (applies to only walking)
    pub myDuration: SUMOTime,
    /// The allowed modes of transportation
    pub myModeSet: SVCPermissions,
    /// The possible vehicles to use
    pub myVTypes: String,
    /// The walking speed
    pub mySpeed: f64,
    /// The factor to apply to walking durations
    pub myWalkFactor: f64,
    /// The depart position
    pub myDepartPos: f64,
    /// The lateral depart position
    pub myDepartPosLat: f64,
    /// whether an arrivalPos was in the input
    pub myHaveArrivalPos: bool,
    /// stages pre-computed in proceed() (serial) or by TripRoutingTask (parallel)
    pub myComputedStages: Vec<Ptr<MSStage>>,
    /// routing error stored alongside myComputedStages
    pub myRoutingError: String,
    /// personal vehicle to add to simulation after parallel routing completes
    pub myVehicleToAdd: Ptr<SUMOVehicle>,
}

impl std::ops::Deref for MSStageTrip {
    type Target = MSStage;
    fn deref(&self) -> &MSStage {
        &self.base
    }
}
impl std::ops::DerefMut for MSStageTrip {
    fn deref_mut(&mut self) -> &mut MSStage {
        &mut self.base
    }
}

impl MSStageTrip {
    /// C++: `void MSStageTrip::MSStageTrip(const MSEdge *, MSStoppingPlace *, const MSEdge *, MSStoppingPlace *, const SUMOTime, const SVCPermissions, const std::string &, const double, const double, const std::string &, const double, const bool, const double)`
    /// constructor
    // MSStageTrip::MSStageTrip(const MSEdge* origin, MSStoppingPlace* fromStop,
    //                          const MSEdge* destination, MSStoppingPlace* toStop,
    //                          const SUMOTime duration, const SVCPermissions modeSet,
    //                          const std::string& vTypes, const double speed, const double walkFactor,
    //                          const std::string& group,
    //                          const double departPosLat, const bool hasArrivalPos, const double arrivalPos):
    //     MSStage(MSStageType::TRIP, destination, toStop, arrivalPos, 0.0, group),
    //     myOrigin(origin),
    //     myOriginStop(fromStop),
    //     myDuration(duration),
    //     myModeSet(modeSet),
    //     myVTypes(vTypes),
    //     mySpeed(speed),
    //     myWalkFactor(walkFactor),
    //     myDepartPosLat(departPosLat),
    //     myHaveArrivalPos(hasArrivalPos) {
    // }
    #[allow(unused_variables)]
    pub fn new(origin: Ptr<MSEdge>, fromStop: Ptr<MSStoppingPlace>, destination: Ptr<MSEdge>, toStop: Ptr<MSStoppingPlace>, duration: SUMOTime, modeSet: SVCPermissions, vTypes: &str, speed: f64, walkFactor: f64, group: &str, departPosLat: f64, hasArrivalPos: bool, arrivalPos: f64) -> Self {
        MSStageTrip {
            base: MSStage::new(MSStageType::TRIP, destination, toStop, arrivalPos, 0.0, group),
            myOrigin: origin,
            myOriginStop: fromStop,
            myDuration: duration,
            myModeSet: modeSet,
            myVTypes: vTypes.to_string(),
            mySpeed: speed,
            myWalkFactor: walkFactor,
            myDepartPos: 0.,
            myDepartPosLat: departPosLat,
            myHaveArrivalPos: hasArrivalPos,
            myComputedStages: Vec::new(),
            myRoutingError: String::new(),
            myVehicleToAdd: Ptr::null(),
        }
    }

    /// C++: `void MSStageTrip::~MSStageTrip() noexcept` [virtual]
    /// destructor
    // MSStageTrip::~MSStageTrip() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // empty C++ destructor body
    }

    /// C++: `MSStage * MSStageTrip::clone() const` [const, virtual]
    // MSStage*
    // MSStageTrip::clone() const {
    //     MSStage* const clon = new MSStageTrip(myOrigin, const_cast<MSStoppingPlace*>(myOriginStop),
    //                                           myDestination, myDestinationStop, myDuration,
    //                                           myModeSet, myVTypes, mySpeed, myWalkFactor, myGroup,
    //                                           myDepartPosLat, myHaveArrivalPos, myArrivalPos);
    //     clon->setParameters(*this);
    //     return clon;
    // }
    #[allow(unused_variables)]
    pub fn clone(&self) -> Ptr<MSStage> {
        let clon = MSStageTrip::new(self.myOrigin, self.myOriginStop,
            self.myDestination, self.myDestinationStop, self.myDuration,
            self.myModeSet, &self.myVTypes, self.mySpeed, self.myWalkFactor, &self.myGroup,
            self.myDepartPosLat, self.myHaveArrivalPos, self.myArrivalPos);
        let clonPtr = Ptr::from_mut(Box::leak(Box::new(clon))).cast::<MSStage>();
        clonPtr.m().set_parameters(self);
        clonPtr
    }

    /// C++: `const MSEdge * MSStageTrip::getEdge() const` [const, virtual]
    // const MSEdge*
    // MSStageTrip::getEdge() const {
    //     return myOrigin;
    // }
    #[allow(unused_variables)]
    pub fn get_edge(&self) -> Ptr<MSEdge> {
        self.myOrigin
    }

    /// C++: `MSStoppingPlace * MSStageTrip::getOriginStop() const` [const, virtual]
    // MSStoppingPlace* getOriginStop() const {
    //         return myOriginStop;
    //     }
    #[allow(unused_variables)]
    pub fn get_origin_stop(&self) -> Ptr<MSStoppingPlace> {
        self.myOriginStop
    }

    /// C++: `double MSStageTrip::getEdgePos(SUMOTime) const` [const, virtual]
    // double
    // MSStageTrip::getEdgePos(SUMOTime /* now */) const {
    //     return myDepartPos;
    // }
    #[allow(unused_variables)]
    pub fn get_edge_pos(&self, now: SUMOTime) -> f64 {
        self.myDepartPos
    }

    /// C++: `Position MSStageTrip::getPosition(SUMOTime) const` [const, virtual]
    // Position
    // MSStageTrip::getPosition(SUMOTime /* now */) const {
    //     // may be called concurrently while the trip is still being routed
    //     return getEdgePosition(myOrigin, myDepartPos, ROADSIDE_OFFSET * (MSGlobals::gLefthand ? -1 : 1));
    // }
    #[allow(unused_variables)]
    pub fn get_position(&self, now: SUMOTime) -> Position {
        // may be called concurrently while the trip is still being routed
        self.get_edge_position(self.myOrigin, self.myDepartPos, ROADSIDE_OFFSET * (if crate::microsim::ms_globals::lefthand() { -1. } else { 1. }))
    }

    /// C++: `double MSStageTrip::getAngle(SUMOTime) const` [const, virtual]
    // double
    // MSStageTrip::getAngle(SUMOTime /* now */) const {
    //     // may be called concurrently while the trip is still being routed
    //     return getEdgeAngle(myOrigin, myDepartPos) + M_PI / 2 * (MSGlobals::gLefthand ? -1 : 1);
    // }
    #[allow(unused_variables)]
    pub fn get_angle(&self, now: SUMOTime) -> f64 {
        // may be called concurrently while the trip is still being routed
        self.get_edge_angle(self.myOrigin, self.myDepartPos) + std::f64::consts::PI / 2. * (if crate::microsim::ms_globals::lefthand() { -1. } else { 1. })
    }

    /// C++: `double MSStageTrip::getDistance() const` [const, virtual]
    // double getDistance() const {
    //         // invalid
    //         return -1;
    //     }
    #[allow(unused_variables)]
    pub fn get_distance(&self) -> f64 {
        // invalid
        -1.
    }

    /// C++: `std::string MSStageTrip::getStageDescription(const bool) const` [const, virtual]
    // std::string getStageDescription(const bool isPerson) const {
    //         UNUSED_PARAMETER(isPerson);
    //         return "trip";
    //     }
    #[allow(unused_variables)]
    pub fn get_stage_description(&self, isPerson: bool) -> String {
        "trip".to_string()
    }

    /// C++: `std::string MSStageTrip::getOriginDescription() const` [const]
    // std::string
    // MSStageTrip::getOriginDescription() const {
    //     return (myOriginStop != nullptr
    //             ? toString(myOriginStop->getElement()) + " '" + myOriginStop->getID()
    //             : "edge '" + myOrigin->getID()) + "'";
    // }
    #[allow(unused_variables)]
    pub fn get_origin_description(&self) -> String {
        if !self.myOriginStop.is_null() {
            format!("{} '{}'", utils::xml::sumo_xml_definitions::Tags.get_string(self.myOriginStop.get_element()), self.myOriginStop.get_id())
        } else {
            format!("edge '{}'", self.myOrigin.get_id())
        }
    }

    /// C++: `std::string MSStageTrip::getDestinationDescription() const` [const]
    // std::string
    // MSStageTrip::getDestinationDescription() const {
    //     return (myDestinationStop != nullptr
    //             ? toString(myDestinationStop->getElement()) + " '" + myDestinationStop->getID()
    //             : "edge '" + myDestination->getID()) + "'";
    // }
    #[allow(unused_variables)]
    pub fn get_destination_description(&self) -> String {
        if !self.myDestinationStop.is_null() {
            format!("{} '{}'", utils::xml::sumo_xml_definitions::Tags.get_string(self.myDestinationStop.get_element()), self.myDestinationStop.get_id())
        } else {
            format!("edge '{}'", self.myDestination.get_id())
        }
    }

    /// C++: `std::string MSStageTrip::getStageSummary(const bool) const` [const, virtual]
    // std::string
    // MSStageTrip::getStageSummary(const bool) const {
    //     return "trip from " + getOriginDescription() + " to " + getDestinationDescription();
    // }
    #[allow(unused_variables)]
    pub fn get_stage_summary(&self, isPerson: bool) -> String {
        format!("trip from {} to {}", self.get_origin_description(), self.get_destination_description())
    }

    /// C++: `std::vector<SUMOVehicle *> MSStageTrip::getVehicles(MSVehicleControl &, MSTransportable *, const MSEdge *)`
    // std::vector<SUMOVehicle*>
    // MSStageTrip::getVehicles(MSVehicleControl& vehControl, MSTransportable* transportable, const MSEdge* origin) {
    //     std::vector<SUMOVehicleParameter*> pars;
    //     for (StringTokenizer st(myVTypes); st.hasNext();) {
    //         pars.push_back(new SUMOVehicleParameter());
    //         pars.back()->vtypeid = st.next();
    //         pars.back()->parametersSet |= VEHPARS_VTYPE_SET;
    //         pars.back()->departProcedure = DepartDefinition::TRIGGERED;
    //         pars.back()->id = transportable->getID() + "_" + toString(pars.size() - 1);
    //     }
    //     if (pars.empty()) {
    //         if ((myModeSet & SVC_PASSENGER) != 0) {
    //             pars.push_back(new SUMOVehicleParameter());
    //             pars.back()->id = transportable->getID() + "_0";
    //             pars.back()->departProcedure = DepartDefinition::TRIGGERED;
    //         }
    //         if ((myModeSet & SVC_TAXI) != 0) {
    //             for (const auto& item : MSDevice_Taxi::getTaxiTypes()) {
    //                 pars.push_back(new SUMOVehicleParameter());
    //                 pars.back()->vtypeid = item.second;
    //                 pars.back()->id = transportable->getID() + "_taxi_" + item.second;
    //                 pars.back()->line = "taxi";
    //             }
    //         }
    //         if ((myModeSet & SVC_BICYCLE) != 0) {
    //             pars.push_back(new SUMOVehicleParameter());
    //             pars.back()->vtypeid = DEFAULT_BIKETYPE_ID;
    //             pars.back()->id = transportable->getID() + "_b0";
    //             pars.back()->departProcedure = DepartDefinition::TRIGGERED;
    //         }
    //     }
    //     ConstMSRoutePtr const routeDummy = std::make_shared<MSRoute>(transportable->getID() + "_0", ConstMSEdgeVector({ origin }), false, nullptr, StopParVector());
    //     std::vector<SUMOVehicle*> result;
    //     for (SUMOVehicleParameter* vehPar : pars) {
    //         const bool isTaxi = vehPar->line == "taxi" && (myModeSet & SVC_TAXI) != 0;
    //         if (myDepartPos != 0) {
    //             vehPar->departPosProcedure = DepartPosDefinition::GIVEN;
    //             vehPar->departPos = myDepartPos;
    //             vehPar->parametersSet |= VEHPARS_DEPARTPOS_SET;
    //         }
    //         vehPar->parametersSet |= VEHPARS_ARRIVALPOS_SET;
    //         vehPar->arrivalPosProcedure = ArrivalPosDefinition::GIVEN;
    //         vehPar->parametersSet |= VEHPARS_ARRIVALSPEED_SET;
    //         vehPar->arrivalSpeedProcedure = ArrivalSpeedDefinition::GIVEN;
    //         vehPar->arrivalSpeed = 0;
    //         MSVehicleType* type = vehControl.getVType(vehPar->vtypeid);
    //         if (type->getVehicleClass() != SVC_IGNORING && (origin->getPermissions() & type->getVehicleClass()) == 0 && !isTaxi) {
    //             WRITE_WARNINGF(TL("Ignoring vehicle type '%' when routing person '%' because it is not allowed on the start edge."), type->getID(), transportable->getID());
    //             delete vehPar;
    //         } else {
    //             result.push_back(vehControl.buildVehicle(vehPar, routeDummy, type, !MSGlobals::gCheckRoutes));
    //         }
    //     }
    //     if (result.empty()) {
    //         // walking only but may use access infrastructure
    //         result.push_back(nullptr);
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_vehicles(&mut self, vehControl: &mut MSVehicleControl, transportable: Ptr<MSTransportable>, origin: Ptr<MSEdge>) -> Vec<Ptr<SUMOVehicle>> {
        let tid = transportable.get_id().to_string();
        let mut pars: Vec<SUMOVehicleParameter> = Vec::new();
        for vtype in self.myVTypes.split_whitespace() {
            let idx = pars.len();
            let mut p = SUMOVehicleParameter::new();
            p.vtypeid = vtype.to_string();
            p.parametersSet |= VEHPARS_VTYPE_SET;
            p.departProcedure = DepartDefinition::TRIGGERED;
            p.id = format!("{}_{}", tid, idx);
            pars.push(p);
        }
        if pars.is_empty() {
            if (self.myModeSet & SVC_PASSENGER) != 0 {
                let mut p = SUMOVehicleParameter::new();
                p.id = format!("{}_0", tid);
                p.departProcedure = DepartDefinition::TRIGGERED;
                pars.push(p);
            }
            if (self.myModeSet & SVC_TAXI) != 0 {
                for (_, vt) in MSDevice_Taxi::get_taxi_types().iter() {
                    let mut p = SUMOVehicleParameter::new();
                    p.vtypeid = vt.clone();
                    p.id = format!("{}_taxi_{}", tid, vt);
                    p.line = "taxi".to_string();
                    pars.push(p);
                }
            }
            if (self.myModeSet & SVC_BICYCLE) != 0 {
                let mut p = SUMOVehicleParameter::new();
                p.vtypeid = DEFAULT_BIKETYPE_ID.to_string();
                p.id = format!("{}_b0", tid);
                p.departProcedure = DepartDefinition::TRIGGERED;
                pars.push(p);
            }
        }
        let routeDummy: std::rc::Rc<MSRoute> = std::rc::Rc::new(MSRoute::new(&format!("{}_0", tid), &vec![origin], false, Ptr::null(), &Vec::new(), -1, -1));
        let mut result: Vec<Ptr<SUMOVehicle>> = Vec::new();
        for mut vehPar in pars {
            let isTaxi = vehPar.line == "taxi" && (self.myModeSet & SVC_TAXI) != 0;
            if self.myDepartPos != 0. {
                vehPar.departPosProcedure = DepartPosDefinition::GIVEN;
                vehPar.departPos = self.myDepartPos;
                vehPar.parametersSet |= VEHPARS_DEPARTPOS_SET;
            }
            vehPar.parametersSet |= VEHPARS_ARRIVALPOS_SET;
            vehPar.arrivalPosProcedure = ArrivalPosDefinition::GIVEN;
            vehPar.parametersSet |= VEHPARS_ARRIVALSPEED_SET;
            vehPar.arrivalSpeedProcedure = ArrivalSpeedDefinition::GIVEN;
            vehPar.arrivalSpeed = 0.;
            let vtype = vehControl.get_v_type(&vehPar.vtypeid, Ptr::null(), false);
            if vtype.get_vehicle_class() != SVC_IGNORING && (origin.get_permissions() & vtype.get_vehicle_class()) == 0 && !isTaxi {
                write_warning(&format!("Ignoring vehicle type '{}' when routing person '{}' because it is not allowed on the start edge.", vtype.get_id(), tid));
            } else {
                let parPtr = Ptr::from_mut(Box::leak(Box::new(vehPar)));
                result.push(vehControl.build_vehicle(parPtr, routeDummy.clone(), vtype, !crate::microsim::ms_globals::check_routes(), MSVehicleControl_VehicleDefinitionSource::ROUTEFILE, true));
            }
        }
        if result.is_empty() {
            // walking only but may use access infrastructure
            result.push(Ptr::null());
        }
        result
    }

    /// C++: `const std::string MSStageTrip::reroute(const SUMOTime, MSTransportableRouter &, MSTransportable *const, MSStage *, const MSEdge *, const MSEdge *, std::vector<MSStage *> &)`
    // const std::string
    // MSStageTrip::reroute(const SUMOTime time, MSTransportableRouter& router, MSTransportable* const transportable,
    //                      MSStage* previous, const MSEdge* origin, const MSEdge* destination, std::vector<MSStage*>& stages) {
    //     if (origin->isTazConnector() && origin->getSuccessors().size() == 0) {
    //         // previous stage ended at a taz sink-edge
    //         origin = transportable->getNextStage(-1)->getDestination();
    //     }
    //     MSVehicleControl& vehControl = MSNet::getInstance()->getVehicleControl();
    //     double minCost = std::numeric_limits<double>::max();
    //     std::vector<MSTransportableRouter::TripItem> minResult;
    //     SUMOVehicle* minVehicle = nullptr;
    //     // pre-collect candidate vehicles under lock (buildVehicle touches shared counters)
    // #ifndef THREAD_POOL
    // #ifdef HAVE_FOX
    //     myVehCtrlMutex.lock();
    // #endif
    // #endif
    //     std::vector<SUMOVehicle*> vehicles = getVehicles(vehControl, transportable, origin);
    // #ifndef THREAD_POOL
    // #ifdef HAVE_FOX
    //     myVehCtrlMutex.unlock();
    // #endif
    // #endif
    //     for (SUMOVehicle* vehicle : vehicles) {
    //         std::vector<MSTransportableRouter::TripItem> result;
    //         double departPos = previous->getArrivalPos();
    //         MSStoppingPlace* const prevStop = previous->getDestinationStop();
    //         if (MSGlobals::gUseMesoSim && prevStop != nullptr) {
    //             departPos = (prevStop->getBeginLanePosition() + prevStop->getEndLanePosition()) / 2.;
    //         }
    //         if (router.compute(origin, destination, departPos, prevStop == nullptr ? "" : prevStop->getID(),
    //                            myArrivalPos, myDestinationStop == nullptr ? "" : myDestinationStop->getID(),
    //                            transportable->getMaxSpeed() * myWalkFactor, vehicle, transportable->getVTypeParameter(), myModeSet, time, result)) {
    //             double totalCost = 0;
    //             for (const MSTransportableRouter::TripItem& item : result) {
    //                 totalCost += item.cost;
    //             }
    //             if (totalCost < minCost) {
    //                 minCost = totalCost;
    //                 minResult.swap(result);
    //                 std::swap(minVehicle, vehicle);
    //             }
    //         }
    //         if (vehicle != nullptr) {
    // #ifndef THREAD_POOL
    // #ifdef HAVE_FOX
    //             myVehCtrlMutex.lock();
    // #endif
    // #endif
    //             vehControl.deleteVehicle(vehicle, true);
    //             vehControl.discountRoutingVehicle();
    // #ifndef THREAD_POOL
    // #ifdef HAVE_FOX
    //             myVehCtrlMutex.unlock();
    // #endif
    // #endif
    //         }
    //     }
    //     if (minCost != std::numeric_limits<double>::max()) {
    //         const bool isTaxi = minVehicle != nullptr && (myModeSet & SVC_TAXI) != 0 && minVehicle->getParameter().line == "taxi";
    //         bool carUsed = false;
    //         for (std::vector<MSTransportableRouter::TripItem>::iterator it = minResult.begin(); it != minResult.end(); ++it) {
    //             if (!it->edges.empty()) {
    //                 MSStoppingPlace* bs = MSNet::getInstance()->getStoppingPlace(it->destStop);
    //                 double localArrivalPos = bs != nullptr ? bs->getAccessPos(it->edges.back()) : it->arrivalPos;
    //                 const MSEdge* const first = it->edges.front();
    //                 const MSEdge* const rideOrigin = origin->isTazConnector() && stages.empty() ? first : nullptr;
    //                 if (it + 1 == minResult.end() && myHaveArrivalPos) {
    //                     localArrivalPos = myArrivalPos;
    //                 }
    //                 if (it->line == "") {
    //                     // determine walk departPos
    //                     double depPos = previous->getArrivalPos();
    //                     if (previous->getDestinationStop() != nullptr) {
    //                         depPos = previous->getDestinationStop()->getAccessPos(first, first->getLanes()[0]->getRNG());
    //                     } else if (origin->isTazConnector()) {
    //                         // walk the whole length of the first edge
    //                         if (std::find(first->getPredecessors().begin(), first->getPredecessors().end(), origin) != first->getPredecessors().end()) {
    //                             depPos = 0;
    //                         } else {
    //                             depPos = first->getLength();
    //                         }
    //                     } else if (previous->getDestination() != first) {
    //                         if ((previous->getDestination()->getToJunction() == first->getToJunction())
    //                                 || (previous->getDestination()->getFromJunction() == first->getToJunction())) {
    //                             depPos = first->getLength();
    //                         } else {
    //                             depPos = 0.;
    //                         }
    //                     }
    //                     if (destination->isTazConnector()) {
    //                         // walk the whole length of the last edge
    //                         const MSEdge* last = it->edges.back();
    //                         if (std::find(last->getSuccessors().begin(), last->getSuccessors().end(), destination) != last->getSuccessors().end()) {
    //                             localArrivalPos = last->getLength();
    //                         } else {
    //                             localArrivalPos = 0;
    //                         }
    //                     }
    //                     previous = new MSStageWalking(transportable->getID(), it->edges, bs, myDuration, mySpeed, depPos, localArrivalPos, myDepartPosLat);
    //                     previous->setParameters(*this);
    //                     previous->setCosts(it->cost);
    //                     previous->setTrip(this);
    //                     stages.push_back(previous);
    //                 } else if (isTaxi && it->line == minVehicle->getID()) {
    //                     const ConstMSEdgeVector& prevEdges = previous->getEdges();
    //                     if (prevEdges.size() >= 2 && previous->getDestinationStop() == nullptr) {
    //                         // determine walking direction and let the previous
    //                         // stage end after entering its final edge
    //                         const MSEdge* last = prevEdges.back();
    //                         const MSEdge* prev = prevEdges[prevEdges.size() - 2];
    //                         if (last->getFromJunction() == prev->getToJunction() || prev->getFromJunction() == last->getFromJunction()) {
    //                             previous->setArrivalPos(MIN2(last->getLength(), 10.0));
    //                         } else {
    //                             previous->setArrivalPos(MAX2(0.0, last->getLength() - 10));
    //                         }
    //                     }
    //                     previous = new MSStageDriving(rideOrigin, it->edges.back(), bs, localArrivalPos, 0.0, std::vector<std::string>({ "taxi" }), myGroup);
    //                     previous->setParameters(*this);
    //                     previous->setCosts(it->cost);
    //                     previous->setTrip(this);
    //                     stages.push_back(previous);
    //                 } else if (minVehicle != nullptr && it->line == minVehicle->getID()) {
    //                     previous = new MSStageDriving(rideOrigin, it->edges.back(), bs, localArrivalPos, 0.0, std::vector<std::string>({ it->line }));
    //                     previous->setParameters(*this);
    //                     previous->setCosts(it->cost);
    //                     previous->setTrip(this);
    //                     stages.push_back(previous);
    //                     minVehicle->replaceRouteEdges(it->edges, -1, 0, "person:" + transportable->getID(), true);
    //                     minVehicle->setArrivalPos(localArrivalPos);
    //                     const_cast<SUMOVehicleParameter&>(minVehicle->getParameter()).arrivalPos = localArrivalPos;
    //                     // defer addVehicle to main thread (called from setArrived after parallel routing completes)
    //                     myVehicleToAdd = minVehicle;
    //                     carUsed = true;
    //                 } else {
    //                     const std::string line = OptionsCont::getOptions().getBool("persontrip.ride-public-line") ? it->line : LINE_ANY;
    //                     previous = new MSStageDriving(rideOrigin, it->edges.back(), bs, localArrivalPos, 0.0, std::vector<std::string>({ line }), myGroup, it->intended, TIME2STEPS(it->depart));
    //                     previous->setParameters(*this);
    //                     previous->setCosts(it->cost);
    //                     previous->setTrip(this);
    //                     stages.push_back(previous);
    //                 }
    //             }
    //         }
    //         if (wasSet(VEHPARS_ARRIVALPOS_SET) && !stages.empty()) {
    //             // mark the last stage
    //             stages.back()->markSet(VEHPARS_ARRIVALPOS_SET);
    //         }
    //         setCosts(minCost);
    //         if (minVehicle != nullptr && (isTaxi || !carUsed)) {
    // #ifndef THREAD_POOL
    // #ifdef HAVE_FOX
    //             myVehCtrlMutex.lock();
    // #endif
    // #endif
    //             vehControl.deleteVehicle(minVehicle, true);
    //             vehControl.discountRoutingVehicle();
    // #ifndef THREAD_POOL
    // #ifdef HAVE_FOX
    //             myVehCtrlMutex.unlock();
    // #endif
    // #endif
    //         }
    //     } else {
    //         // append stage so the GUI won't crash due to inconsistent state
    //         previous = new MSStageWalking(transportable->getID(), ConstMSEdgeVector({ origin, destination }), myDestinationStop, myDuration, mySpeed, previous->getArrivalPos(), myArrivalPos, myDepartPosLat);
    //         previous->setParameters(*this);
    //         stages.push_back(previous);
    //         if (MSGlobals::gCheckRoutes) {  // if not pedestrians will teleport
    //             return "No connection found between " + getOriginDescription() + " and " + getDestinationDescription() + " for person '" + transportable->getID() + "'.";
    //         }
    //     }
    //     if (stages.empty()) {
    //         // append stage so the GUI won't crash due to inconsistent state
    //         if (myOriginStop != nullptr && myOriginStop == myDestinationStop) {
    //             stages.push_back(new MSStageWaiting(destination, myDestinationStop, 0, -1, previous->getArrivalPos(), "sameStop", false));
    //         } else {
    //             stages.push_back(new MSStageWalking(transportable->getID(), ConstMSEdgeVector({ origin, destination }), myDestinationStop, myDuration, mySpeed, previous->getArrivalPos(), myArrivalPos, myDepartPosLat));
    //             if (MSGlobals::gCheckRoutes) {  // if not pedestrians will teleport
    //                 return "Empty route between " + getOriginDescription() + " and " + getDestinationDescription() + " for person '" + transportable->getID() + "'.";
    //             }
    //         }
    //     }
    //     return "";
    // }
    #[allow(unused_variables)]
    pub fn reroute(&mut self, time: SUMOTime, router: &mut MSTransportableRouter, transportable: Ptr<MSTransportable>, previous: Ptr<MSStage>, origin: Ptr<MSEdge>, destination: Ptr<MSEdge>, stages: &mut Vec<Ptr<MSStage>>) -> String {
        let mut origin = origin;
        let mut previous = previous;
        if origin.is_taz_connector() && origin.get_successors(SVC_IGNORING).is_empty() {
            // previous stage ended at a taz sink-edge
            origin = transportable.get_next_stage(-1).get_destination();
        }
        let netPtr = MSNet::get_instance();
        let vehControl = netPtr.m().get_vehicle_control();
        let mut minCost = f64::MAX;
        let mut minResult: Vec<IntermodalRouter_TripItem> = Vec::new();
        let mut minVehicle: Ptr<SUMOVehicle> = Ptr::null();
        // shortcut: the parallel path (pre-collecting vehicles under a lock) is not ported, routing is always serial
        let vehicles = self.get_vehicles(vehControl, transportable, origin);
        for mut vehicle in vehicles {
            let mut result: Vec<IntermodalRouter_TripItem> = Vec::new();
            let mut departPos = previous.get_arrival_pos();
            let prevStop = previous.get_destination_stop();
            if MSGlobals::use_meso_sim() && !prevStop.is_null() {
                departPos = (prevStop.get_begin_lane_position() + prevStop.get_end_lane_position()) / 2.;
            }
            let prevStopID = if prevStop.is_null() { String::new() } else { prevStop.get_id().to_string() };
            let destStopID = if self.myDestinationStop.is_null() { String::new() } else { self.myDestinationStop.get_id().to_string() };
            let ok = router.compute(origin, destination, departPos, &prevStopID, self.myArrivalPos, &destStopID,
                transportable.get_max_speed() * self.myWalkFactor, vehicle, transportable.get_v_type_parameter(), self.myModeSet, time, &mut result, 0.);
            if ok {
                let mut totalCost = 0.;
                for item in result.iter() {
                    totalCost += item.cost;
                }
                if totalCost < minCost {
                    minCost = totalCost;
                    minResult = result;
                    std::mem::swap(&mut minVehicle, &mut vehicle);
                }
            }
            if !vehicle.is_null() {
                vehControl.delete_vehicle(vehicle, true, false);
                vehControl.discount_routing_vehicle();
            }
        }
        if minCost != f64::MAX {
            let isTaxi = !minVehicle.is_null() && (self.myModeSet & SVC_TAXI) != 0 && minVehicle.get_parameter().line == "taxi";
            let mut carUsed = false;
            let n = minResult.len();
            for (idx, it) in minResult.iter().enumerate() {
                if !it.edges.is_empty() {
                    let edges: ConstMSEdgeVector = it.edges.iter().map(|e| e.cast::<MSEdge>()).collect();
                    // shortcut: MSNet::getStoppingPlace(id) without a category is only ported for the bus-stop category
                    let bs = MSNet::get_instance().m().get_stopping_place(&it.destStop, SUMO_TAG_BUS_STOP);
                    let first = edges[0];
                    let last = edges[edges.len() - 1];
                    let mut localArrivalPos = if !bs.is_null() { bs.get_access_pos(last, last.get_lanes()[0].get_rng()) } else { it.arrivalPos };
                    let rideOrigin = if origin.is_taz_connector() && stages.is_empty() { first } else { Ptr::null() };
                    if idx + 1 == n && self.myHaveArrivalPos {
                        localArrivalPos = self.myArrivalPos;
                    }
                    if it.line == "" {
                        // determine walk departPos
                        let mut depPos = previous.get_arrival_pos();
                        if !previous.get_destination_stop().is_null() {
                            depPos = previous.get_destination_stop().get_access_pos(first, first.get_lanes()[0].get_rng());
                        } else if origin.is_taz_connector() {
                            // walk the whole length of the first edge
                            if first.get_predecessors().contains(&origin) {
                                depPos = 0.;
                            } else {
                                depPos = first.get_length();
                            }
                        } else if previous.get_destination() != first {
                            if previous.get_destination().get_to_junction() == first.get_to_junction()
                                || previous.get_destination().get_from_junction() == first.get_to_junction() {
                                depPos = first.get_length();
                            } else {
                                depPos = 0.;
                            }
                        }
                        if destination.is_taz_connector() {
                            // walk the whole length of the last edge
                            if last.get_successors(SVC_IGNORING).contains(&destination) {
                                localArrivalPos = last.get_length();
                            } else {
                                localArrivalPos = 0.;
                            }
                        }
                        let mut walk = MSStageWalking::new(transportable.get_id(), &edges, bs, self.myDuration, self.mySpeed, depPos, localArrivalPos, self.myDepartPosLat, -1, "");
                        walk.set_parameters(&*self);
                        walk.set_costs(it.cost);
                        walk.set_trip(Ptr::from_mut(&mut *self));
                        previous = Ptr::from_mut(Box::leak(Box::new(walk))).cast::<MSStage>();
                        stages.push(previous);
                    } else if isTaxi && it.line == minVehicle.get_id() {
                        let prevEdges = previous.get_edges();
                        if prevEdges.len() >= 2 && previous.get_destination_stop().is_null() {
                            // determine walking direction and let the previous
                            // stage end after entering its final edge
                            let pLast = prevEdges[prevEdges.len() - 1];
                            let prev = prevEdges[prevEdges.len() - 2];
                            if pLast.get_from_junction() == prev.get_to_junction() || prev.get_from_junction() == pLast.get_from_junction() {
                                previous.m().set_arrival_pos(pLast.get_length().min(10.0));
                            } else {
                                previous.m().set_arrival_pos((0.0f64).max(pLast.get_length() - 10.));
                            }
                        }
                        let mut drv = MSStageDriving::new(rideOrigin, edges[edges.len() - 1], bs, localArrivalPos, 0.0, &vec!["taxi".to_string()], &self.myGroup, "", -1);
                        drv.set_parameters(&*self);
                        drv.set_costs(it.cost);
                        drv.set_trip(Ptr::from_mut(&mut *self));
                        previous = Ptr::from_mut(Box::leak(Box::new(drv))).cast::<MSStage>();
                        stages.push(previous);
                    } else if !minVehicle.is_null() && it.line == minVehicle.get_id() {
                        let mut drv = MSStageDriving::new(rideOrigin, edges[edges.len() - 1], bs, localArrivalPos, 0.0, &vec![it.line.clone()], "", "", -1);
                        drv.set_parameters(&*self);
                        drv.set_costs(it.cost);
                        drv.set_trip(Ptr::from_mut(&mut *self));
                        previous = Ptr::from_mut(Box::leak(Box::new(drv))).cast::<MSStage>();
                        stages.push(previous);
                        let mut edgesMut = edges.clone();
                        minVehicle.m().replace_route_edges(&mut edgesMut, -1., 0., &format!("person:{}", transportable.get_id()), true, false, true, Ptr::null());
                        minVehicle.m().set_arrival_pos(localArrivalPos);
                        // shortcut: SUMOVehicle has no mutable parameter accessor yet, so the vehicle's parameter arrivalPos is not updated
                        // defer addVehicle to main thread (called from setArrived after parallel routing completes)
                        self.myVehicleToAdd = minVehicle;
                        carUsed = true;
                    } else {
                        let line = if options_cont::get_bool("persontrip.ride-public-line") { it.line.clone() } else { "ANY".to_string() };
                        let mut drv = MSStageDriving::new(rideOrigin, edges[edges.len() - 1], bs, localArrivalPos, 0.0, &vec![line], &self.myGroup, &it.intended, time2steps(it.depart));
                        drv.set_parameters(&*self);
                        drv.set_costs(it.cost);
                        drv.set_trip(Ptr::from_mut(&mut *self));
                        previous = Ptr::from_mut(Box::leak(Box::new(drv))).cast::<MSStage>();
                        stages.push(previous);
                    }
                }
            }
            if self.was_set(VEHPARS_ARRIVALPOS_SET as i32) && !stages.is_empty() {
                // mark the last stage
                stages[stages.len() - 1].m().mark_set(VEHPARS_ARRIVALPOS_SET as i32);
            }
            self.set_costs(minCost);
            if !minVehicle.is_null() && (isTaxi || !carUsed) {
                vehControl.delete_vehicle(minVehicle, true, false);
                vehControl.discount_routing_vehicle();
            }
        } else {
            // append stage so the GUI won't crash due to inconsistent state
            let mut walk = MSStageWalking::new(transportable.get_id(), &vec![origin, destination], self.myDestinationStop, self.myDuration, self.mySpeed, previous.get_arrival_pos(), self.myArrivalPos, self.myDepartPosLat, -1, "");
            walk.set_parameters(&*self);
            previous = Ptr::from_mut(Box::leak(Box::new(walk))).cast::<MSStage>();
            stages.push(previous);
            if crate::microsim::ms_globals::check_routes() {
                // if not pedestrians will teleport
                return format!("No connection found between {} and {} for person '{}'.", self.get_origin_description(), self.get_destination_description(), transportable.get_id());
            }
        }
        if stages.is_empty() {
            // append stage so the GUI won't crash due to inconsistent state
            if !self.myOriginStop.is_null() && self.myOriginStop == self.myDestinationStop {
                let w = MSStageWaiting::new(destination, self.myDestinationStop, 0, -1, previous.get_arrival_pos(), "sameStop", false, -1);
                stages.push(Ptr::from_mut(Box::leak(Box::new(w))).cast::<MSStage>());
            } else {
                let walk = MSStageWalking::new(transportable.get_id(), &vec![origin, destination], self.myDestinationStop, self.myDuration, self.mySpeed, previous.get_arrival_pos(), self.myArrivalPos, self.myDepartPosLat, -1, "");
                stages.push(Ptr::from_mut(Box::leak(Box::new(walk))).cast::<MSStage>());
                if crate::microsim::ms_globals::check_routes() {
                    // if not pedestrians will teleport
                    return format!("Empty route between {} and {} for person '{}'.", self.get_origin_description(), self.get_destination_description(), transportable.get_id());
                }
            }
        }
        String::new()
    }

    /// C++: `const std::string MSStageTrip::setArrived(MSNet *, MSTransportable *, SUMOTime, const bool)` [virtual]
    /// logs end of the step
    // const std::string
    // MSStageTrip::setArrived(MSNet* net, MSTransportable* transportable, SUMOTime now, const bool vehicleArrived) {
    //     MSStage::setArrived(net, transportable, now, vehicleArrived);
    //     std::vector<MSStage*> stages;
    //     std::string result;
    //     if (!myComputedStages.empty()) {
    //         // routing was done in proceed() (serial or parallel)
    //         stages.swap(myComputedStages);
    //         result = myRoutingError;
    //         myRoutingError = "";
    //     } else {
    //         // TRIP is at index 0 with no preceding stage, so proceed() was never called
    //         myDepartPos = transportable->getParameter().departPos;
    //         if (transportable->getParameter().departPosProcedure == DepartPosDefinition::RANDOM) {
    //             // TODO we should probably use the rng of the lane here
    //             myDepartPos = RandHelper::rand(myOrigin->getLength());
    //         }
    //         MSStageWaiting start(myOrigin, myOriginStop, -1, transportable->getParameter().depart, myDepartPos, "start", true);
    //         result = reroute(transportable->getParameter().depart, net->getIntermodalRouter(0), transportable, &start, myOrigin, myDestination, stages);
    //     }
    //     if (myVehicleToAdd != nullptr) {
    //         net->getVehicleControl().addVehicle(myVehicleToAdd->getID(), myVehicleToAdd);
    //         myVehicleToAdd = nullptr;
    //     }
    //     int idx = 1;
    //     for (MSStage* stage : stages) {
    //         transportable->appendStage(stage, idx++);
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn set_arrived(&mut self, net: Ptr<MSNet>, transportable: Ptr<MSTransportable>, now: SUMOTime, vehicleArrived: bool) -> String {
        self.base.myArrived = now;
        let mut stages: Vec<Ptr<MSStage>> = Vec::new();
        let mut result: String;
        if !self.myComputedStages.is_empty() {
            // routing was done in proceed()
            stages = std::mem::take(&mut self.myComputedStages);
            result = std::mem::take(&mut self.myRoutingError);
        } else {
            // TRIP is at index 0 with no preceding stage, so proceed() was never called
            self.myDepartPos = transportable.get_parameter().departPos;
            if transportable.get_parameter().departPosProcedure == DepartPosDefinition::RANDOM {
                // TODO we should probably use the rng of the lane here
                self.myDepartPos = utils::common::rand_helper::with_default(|r| r.rand_max(self.myOrigin.get_length()));
            }
            let origin = self.myOrigin;
            let destination = self.myDestination;
            let depart = transportable.get_parameter().depart;
            let mut start = MSStageWaiting::new(origin, self.myOriginStop, -1, depart, self.myDepartPos, "start", true, -1);
            let router = net.m().get_intermodal_router(0, 0, &Default::default());
            result = self.reroute(depart, router, transportable, Ptr::from_mut(&mut start.base), origin, destination, &mut stages);
        }
        if !self.myVehicleToAdd.is_null() {
            let v = self.myVehicleToAdd;
            net.m().get_vehicle_control().add_vehicle(v.get_id(), v);
            self.myVehicleToAdd = Ptr::null();
        }
        let mut idx = 1;
        for stage in stages {
            transportable.m().append_stage(stage, idx);
            idx += 1;
        }
        result
    }

    /// C++: `void MSStageTrip::setOrigin(const MSEdge *, MSStoppingPlace *, double)` [virtual]
    /// change origin for parking area rerouting
    // void setOrigin(const MSEdge* origin, MSStoppingPlace* originStop, double departPos) {
    //         myOrigin = origin;
    //         myOriginStop = originStop;
    //         myDepartPos = departPos;
    //     }
    #[allow(unused_variables)]
    pub fn set_origin(&mut self, origin: Ptr<MSEdge>, originStop: Ptr<MSStoppingPlace>, departPos: f64) {
        self.myOrigin = origin;
        self.myOriginStop = originStop;
        self.myDepartPos = departPos;
    }

    /// C++: `bool MSStageTrip::proceed(MSNet *, MSTransportable *, SUMOTime, MSStage *)` [virtual]
    /// proceeds to the next step
    // bool
    // MSStageTrip::proceed(MSNet* net, MSTransportable* transportable, SUMOTime now, MSStage* previous) {
    //     myDepartPos = previous->getArrivalPos();
    // #ifndef THREAD_POOL
    // #ifdef HAVE_FOX
    //     MFXWorkerThread::Pool& threadPool = MSNet::getInstance()->getEdgeControl().getThreadPool();
    //     if (threadPool.size() > 0) {
    //         MSRoutingEngine::getIntermodalRouterTT(transportable->getRNGIndex());
    //         threadPool.add(new TripRoutingTask(*this, transportable, now, previous));
    //         net->getPersonControl().addPendingRouting(transportable);
    //         return true;
    //     }
    // #endif
    // #endif
    //     myRoutingError = reroute(now, net->getIntermodalRouter(0), transportable, previous, myOrigin, myDestination, myComputedStages);
    //     return transportable->proceed(net, now);
    // }
    #[allow(unused_variables)]
    pub fn proceed(&mut self, net: Ptr<MSNet>, transportable: Ptr<MSTransportable>, now: SUMOTime, previous: Ptr<MSStage>) -> bool {
        self.myDepartPos = previous.get_arrival_pos();
        // shortcut: the parallel TripRoutingTask path (thread pool) is not ported, so routing always runs serially here
        let router = net.m().get_intermodal_router(0, 0, &Default::default());
        let mut computed = std::mem::take(&mut self.myComputedStages);
        let err = self.reroute(now, router, transportable, previous, self.myOrigin, self.myDestination, &mut computed);
        self.myComputedStages = computed;
        self.myRoutingError = err;
        transportable.m().proceed(net, now, false)
    }

    /// C++: `void MSStageTrip::tripInfoOutput(OutputDevice &, const MSTransportable *const) const` [const, virtual]
    /// Called on writing tripinfo output
    // void tripInfoOutput(OutputDevice& os, const MSTransportable* const transportable) const {
    //         UNUSED_PARAMETER(os);
    //         UNUSED_PARAMETER(transportable);
    //     }
    #[allow(unused_variables)]
    pub fn trip_info_output(&self, os: &mut OutputDevice, transportable: Ptr<MSTransportable>) {
        // C++ body is empty
    }

    /// C++: `SUMOTime MSStageTrip::getDuration() const` [const, virtual]
    /// trip doesn't participate in plan summary
    // SUMOTime getDuration() const {
    //         return 0;
    //     }
    #[allow(unused_variables)]
    pub fn get_duration(&self) -> SUMOTime {
        // trip doesn't participate in plan summary
        0
    }

    /// C++: `SUMOTime MSStageTrip::getTravelTime() const` [const, virtual]
    // SUMOTime getTravelTime() const {
    //         return 0;
    //     }
    #[allow(unused_variables)]
    pub fn get_travel_time(&self) -> SUMOTime {
        0
    }

    /// C++: `void MSStageTrip::routeOutput(const bool, OutputDevice &, const bool, const MSStage *const, const bool, const bool) const` [const, virtual]
    /// Called on writing vehroute output
    // void
    // MSStageTrip::routeOutput(const bool /*isPerson*/, OutputDevice& os, const bool /*withRouteLength*/, const MSStage* const previous, const bool /*withTiming*/, const bool /*saveState*/) const {
    //     if (myArrived < 0) {
    //         const bool walkFactorSet = myWalkFactor != OptionsCont::getOptions().getFloat("persontrip.walkfactor");
    //         const bool groupSet = myGroup != OptionsCont::getOptions().getString("persontrip.default.group");
    //         // could still be a persontrip but most likely it was a walk in the input
    //         SumoXMLTag tag = myModeSet == 0 && !walkFactorSet && !groupSet ? SUMO_TAG_WALK : SUMO_TAG_PERSONTRIP;
    //         os.openTag(tag);
    //         if (previous == nullptr || previous->getStageType() == MSStageType::WAITING_FOR_DEPART) {
    //             os.writeAttr(SUMO_ATTR_FROM, myOrigin->getID());
    //         }
    //         if (myDestinationStop == nullptr) {
    //             os.writeAttr(SUMO_ATTR_TO, myDestination->getID());
    //             if (wasSet(VEHPARS_ARRIVALPOS_SET)) {
    //                 os.writeAttr(SUMO_ATTR_ARRIVALPOS, myArrivalPos);
    //             }
    //         } else {
    //             os.writeAttr(toString(myDestinationStop->getElement()), myDestinationStop->getID());
    //         }
    //         std::vector<std::string> modes;
    //         if ((myModeSet & SVC_PASSENGER) != 0) {
    //             modes.push_back("car");
    //         }
    //         if ((myModeSet & SVC_BICYCLE) != 0) {
    //             modes.push_back("bicycle");
    //         }
    //         if ((myModeSet & SVC_TAXI) != 0) {
    //             modes.push_back("taxi");
    //         }
    //         if ((myModeSet & SVC_BUS) != 0) {
    //             modes.push_back("public");
    //         }
    //         if (modes.size() > 0) {
    //             os.writeAttr(SUMO_ATTR_MODES, modes);
    //         }
    //         if (myVTypes.size() > 0) {
    //             os.writeAttr(SUMO_ATTR_VTYPES, myVTypes);
    //         }
    //         if (groupSet) {
    //             os.writeAttr(SUMO_ATTR_GROUP, myGroup);
    //         }
    //         if (walkFactorSet) {
    //             os.writeAttr(SUMO_ATTR_WALKFACTOR, myWalkFactor);
    //         }
    //         if (OptionsCont::getOptions().getBool("vehroute-output.cost")) {
    //             os.writeAttr(SUMO_ATTR_COST, getCosts());
    //         }
    //         os.closeTag();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn route_output(&self, isPerson: bool, os: &mut OutputDevice, withRouteLength: bool, previous: Ptr<MSStage>, withTiming: bool, saveState: bool) {
        if self.myArrived < 0 {
            let walkFactorSet = self.myWalkFactor != options_cont::get_float("persontrip.walkfactor");
            let groupSet = self.myGroup != options_cont::get_string("persontrip.default.group");
            // could still be a persontrip but most likely it was a walk in the input
            let tag = if self.myModeSet == 0 && !walkFactorSet && !groupSet { SUMO_TAG_WALK } else { SUMO_TAG_PERSONTRIP };
            os.open_tag_id(tag);
            if previous.is_null() || previous.get_stage_type() == MSStageType::WAITING_FOR_DEPART {
                os.write_attr(SUMO_ATTR_FROM, self.myOrigin.get_id());
            }
            if self.myDestinationStop.is_null() {
                os.write_attr(SUMO_ATTR_TO, self.myDestination.get_id());
                if self.was_set(VEHPARS_ARRIVALPOS_SET as i32) {
                    os.write_attr(SUMO_ATTR_ARRIVALPOS, &self.myArrivalPos);
                }
            } else {
                os.write_attr(utils::xml::sumo_xml_definitions::Tags.get_string(self.myDestinationStop.get_element()), self.myDestinationStop.get_id());
            }
            let mut modes: Vec<&str> = Vec::new();
            if (self.myModeSet & SVC_PASSENGER) != 0 {
                modes.push("car");
            }
            if (self.myModeSet & SVC_BICYCLE) != 0 {
                modes.push("bicycle");
            }
            if (self.myModeSet & SVC_TAXI) != 0 {
                modes.push("taxi");
            }
            if (self.myModeSet & SVC_BUS) != 0 {
                modes.push("public");
            }
            if modes.len() > 0 {
                os.write_attr(SUMO_ATTR_MODES, &modes.join(" "));
            }
            if self.myVTypes.len() > 0 {
                os.write_attr(SUMO_ATTR_VTYPES, &self.myVTypes);
            }
            if groupSet {
                os.write_attr(SUMO_ATTR_GROUP, &self.myGroup);
            }
            if walkFactorSet {
                os.write_attr(SUMO_ATTR_WALKFACTOR, &self.myWalkFactor);
            }
            if options_cont::get_bool("vehroute-output.cost") {
                os.write_attr(SUMO_ATTR_COST, &self.get_costs());
            }
            os.close_tag("");
        }
    }

    /// C++: `bool MSStageTrip::isWalk() const` [const, virtual]
    /// Whether the transportable is walking
    // bool isWalk() const {
    //         return myModeSet == 0;
    //     }
    #[allow(unused_variables)]
    pub fn is_walk(&self) -> bool {
        self.myModeSet == 0
    }

    /// C++: `void MSStageTrip::MSStageTrip(const MSStageTrip &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSStageTrip) -> Self {
        panic!("MSStageTrip copy constructor is private and invalidated in C++")
    }

    /// C++: `MSStageTrip & MSStageTrip::operator=(const MSStageTrip &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSStageTrip) -> &mut MSStageTrip {
        panic!("MSStageTrip assignment operator is private and invalidated in C++")
    }

}

