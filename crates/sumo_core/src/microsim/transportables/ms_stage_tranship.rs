//! Port of microsim/transportables/MSStageTranship.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::xml::sumo_xml_definitions::Tags;
use utils::options::options_cont;

/// A "real" stage performing the tranship of a container A container is in this stage if it gets transhiped between two stops that are assumed to be connected.
/// C++ class `MSStageTranship` (microsim/transportables/MSStageTranship.h)
#[repr(C)]
pub struct MSStageTranship {
    /// C++ base class `MSStageMoving`
    pub base: MSStageMoving,
}

impl std::ops::Deref for MSStageTranship {
    type Target = MSStageMoving;
    fn deref(&self) -> &MSStageMoving {
        &self.base
    }
}
impl std::ops::DerefMut for MSStageTranship {
    fn deref_mut(&mut self) -> &mut MSStageMoving {
        &mut self.base
    }
}

impl MSStageTranship {
    /// C++: `void MSStageTranship::MSStageTranship(const std::vector<const MSEdge *> &, MSStoppingPlace *, double, double, double)`
    /// constructor
    // MSStageTranship::MSStageTranship(const std::vector<const MSEdge*>& route,
    //                                  MSStoppingPlace* toStop,
    //                                  double speed,
    //                                  double departPos, double arrivalPos) :
    //     MSStageMoving(MSStageType::TRANSHIP, route, "", toStop, speed, departPos, arrivalPos, 0., -1) {
    //     myDepartPos = SUMOVehicleParameter::interpretEdgePos(
    //                       departPos, myRoute.front()->getLength(), SUMO_ATTR_DEPARTPOS,
    //                       "container getting transhipped from " + myRoute.front()->getID());
    //     myArrivalPos = SUMOVehicleParameter::interpretEdgePos(
    //                        arrivalPos, route.back()->getLength(), SUMO_ATTR_ARRIVALPOS,
    //                        "container getting transhipped to " + route.back()->getID());
    // }
    #[allow(unused_variables)]
    pub fn new(route: &Vec<Ptr<MSEdge>>, toStop: Ptr<MSStoppingPlace>, speed: f64, departPos: f64, arrivalPos: f64) -> Self {
        let mut t = MSStageTranship {
            base: MSStageMoving::new(MSStageType::TRANSHIP, route, "", toStop, speed, departPos, arrivalPos, 0., -1),
        };
        t.myDepartPos = SUMOVehicleParameter::interpret_edge_pos(
            departPos, route[0].get_length(), SUMO_ATTR_DEPARTPOS,
            &format!("container getting transhipped from {}", route[0].get_id()), false);
        t.myArrivalPos = SUMOVehicleParameter::interpret_edge_pos(
            arrivalPos, route[route.len() - 1].get_length(), SUMO_ATTR_ARRIVALPOS,
            &format!("container getting transhipped to {}", route[route.len() - 1].get_id()), false);
        t
    }

    /// C++: `void MSStageTranship::~MSStageTranship() noexcept` [virtual]
    /// destructor
    // MSStageTranship::~MSStageTranship() {
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // empty C++ destructor body
    }

    /// C++: `MSStage * MSStageTranship::clone() const` [const, virtual]
    // MSStage*
    // MSStageTranship::clone() const {
    //     MSStage* const clon = new MSStageTranship(myRoute, myDestinationStop, mySpeed, myDepartPos, myArrivalPos);
    //     clon->setParameters(*this);
    //     return clon;
    // }
    #[allow(unused_variables)]
    pub fn clone(&self) -> Ptr<MSStage> {
        let clon = MSStageTranship::new(&self.myRoute, self.myDestinationStop, self.mySpeed, self.myDepartPos, self.myArrivalPos);
        let clonPtr = Ptr::from_mut(Box::leak(Box::new(clon))).cast::<MSStage>();
        clonPtr.m().set_parameters(self);
        clonPtr
    }

    /// C++: `bool MSStageTranship::proceed(MSNet *, MSTransportable *, SUMOTime, MSStage *)` [virtual]
    /// proceeds to the next step
    // bool
    // MSStageTranship::proceed(MSNet* net, MSTransportable* transportable, SUMOTime now, MSStage* previous) {
    //     myDeparted = now;
    //     //MSPModel_NonInteracting moves the transportable straight from start to end in
    //     //a single step and assumes that moveToNextEdge is only called once)
    //     //therefore we define that the transportable is already on its destination edge
    //     myRouteStep = myRoute.end() - 1;
    //     myDepartPos = previous->getEdgePos(now);
    //     if (transportable->isPerson()) {
    //         myPState = net->getPersonControl().getNonInteractingModel()->add(transportable, this, now);
    //         (*myRouteStep)->addTransportable(transportable);
    //     } else {
    //         myPState = net->getContainerControl().getNonInteractingModel()->add(transportable, this, now);
    //         (*myRouteStep)->addTransportable(transportable);
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn proceed(&mut self, net: Ptr<MSNet>, transportable: Ptr<MSTransportable>, now: SUMOTime, previous: Ptr<MSStage>) -> bool {
        self.myDeparted = now;
        //MSPModel_NonInteracting moves the transportable straight from start to end in
        //a single step and assumes that moveToNextEdge is only called once)
        //therefore we define that the transportable is already on its destination edge
        self.myRouteStep = self.myRoute.len() - 1;
        self.myDepartPos = previous.get_edge_pos(now);
        if transportable.is_person() {
            self.myPState = net.m().get_person_control().get_non_interacting_model().m().add(transportable, Ptr::from_mut(&mut self.base), now);
            self.myRoute[self.myRouteStep].add_transportable(transportable);
        } else {
            self.myPState = net.m().get_container_control().get_non_interacting_model().m().add(transportable, Ptr::from_mut(&mut self.base), now);
            self.myRoute[self.myRouteStep].add_transportable(transportable);
        }
        true
    }

    /// C++: `double MSStageTranship::getDistance() const` [const, virtual]
    /// get travel distance in this stage
    // double
    // MSStageTranship::getDistance() const {
    //     if (myArrived >= 0) {
    //         const SUMOTime duration = myArrived - myDeparted;
    //         return mySpeed * STEPS2TIME(duration);
    //     } else {
    //         return -1;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_distance(&self) -> f64 {
        if self.myArrived >= 0 {
            let duration = self.myArrived - self.myDeparted;
            self.mySpeed * steps2time(duration)
        } else {
            -1.
        }
    }

    /// C++: `std::string MSStageTranship::getStageDescription(const bool) const` [const, virtual]
    /// Returns the current stage description as a string
    // std::string getStageDescription(const bool isPerson) const {
    //         UNUSED_PARAMETER(isPerson);
    //         return "tranship";
    //     }
    #[allow(unused_variables)]
    pub fn get_stage_description(&self, isPerson: bool) -> String {
        "tranship".to_string()
    }

    /// C++: `std::string MSStageTranship::getStageSummary(const bool) const` [const, virtual]
    // std::string
    // MSStageTranship::getStageSummary(const bool /*isPerson*/) const {
    //     const std::string dest = (getDestinationStop() == nullptr ?
    //                               " edge '" + getDestination()->getID() + "'" :
    //                               " stop '" + getDestinationStop()->getID() + "'");
    //     return "transhipped to " + dest;
    // }
    #[allow(unused_variables)]
    pub fn get_stage_summary(&self, isPerson: bool) -> String {
        let dest = if self.get_destination_stop().is_null() {
            format!(" edge '{}'", self.get_destination().get_id())
        } else {
            format!(" stop '{}'", self.get_destination_stop().get_id())
        };
        format!("transhipped to {}", dest)
    }

    /// C++: `void MSStageTranship::tripInfoOutput(OutputDevice &, const MSTransportable *const) const` [const, virtual]
    /// Called on writing tripinfo output
    // void
    // MSStageTranship::tripInfoOutput(OutputDevice& os, const MSTransportable* const) const {
    //     os.openTag("tranship");
    //     os.writeAttr("depart", time2string(myDeparted));
    //     os.writeAttr("departPos", myDepartPos);
    //     os.writeAttr("arrival", time2string(myArrived));
    //     os.writeAttr("arrivalPos", myArrivalPos);
    //     os.writeAttr("duration", myArrived >= 0 ? time2string(getDuration()) : "-1");
    //     os.writeAttr("routeLength", getDistance());
    //     os.writeAttr("maxSpeed", mySpeed);
    //     os.closeTag();
    // }
    #[allow(unused_variables)]
    pub fn trip_info_output(&self, os: &mut OutputDevice, transportable: Ptr<MSTransportable>) {
        os.open_tag("tranship");
        os.write_attr("depart", &time2string(self.myDeparted));
        os.write_attr("departPos", &self.myDepartPos);
        os.write_attr("arrival", &time2string(self.myArrived));
        os.write_attr("arrivalPos", &self.myArrivalPos);
        let duration = if self.myArrived >= 0 { time2string(self.get_duration()) } else { "-1".to_string() };
        os.write_attr("duration", &duration);
        os.write_attr("routeLength", &self.get_distance());
        os.write_attr("maxSpeed", &self.mySpeed);
        os.close_tag("");
    }

    /// C++: `void MSStageTranship::routeOutput(const bool, OutputDevice &, const bool, const MSStage *const, const bool, const bool) const` [const, virtual]
    /// Called on writing vehroute output
    // void
    // MSStageTranship::routeOutput(const bool /*isPerson*/, OutputDevice& os, const bool withRouteLength, const MSStage* const /* previous */, const bool withTiming, const bool /*saveState*/) const {
    //     os.openTag("tranship").writeAttr(SUMO_ATTR_EDGES, myRoute);
    //     std::string comment = "";
    //     if (myDestinationStop != nullptr) {
    //         os.writeAttr(toString(myDestinationStop->getElement()), myDestinationStop->getID());
    //         if (myDestinationStop->getMyName() != "") {
    //             comment =  " <!-- " + StringUtils::escapeXML(myDestinationStop->getMyName(), true) + " -->";
    //         }
    //     }
    //     os.writeAttr(SUMO_ATTR_SPEED, mySpeed);
    //     if (withRouteLength) {
    //         os.writeAttr("routeLength", mySpeed * STEPS2TIME(myArrived - myDeparted));
    //     }
    //     if (withTiming) {
    //         os.writeAttr(SUMO_ATTR_STARTED, myDeparted >= 0 ? time2string(myDeparted) : "-1");
    //         os.writeAttr(SUMO_ATTR_ENDED, myArrived >= 0 ? time2string(myArrived) : "-1");
    //     }
    //     if (OptionsCont::getOptions().getBool("vehroute-output.cost")) {
    //         os.writeAttr(SUMO_ATTR_COST, getCosts());
    //     }
    //     os.closeTag(comment);
    // }
    #[allow(unused_variables)]
    pub fn route_output(&self, isPerson: bool, os: &mut OutputDevice, withRouteLength: bool, previous: Ptr<MSStage>, withTiming: bool, saveState: bool) {
        os.open_tag("tranship");
        let edges: Vec<String> = self.myRoute.iter().map(|e| e.get_id().to_string()).collect();
        os.write_attr(SUMO_ATTR_EDGES, &edges.join(" "));
        let mut comment = String::new();
        if !self.myDestinationStop.is_null() {
            let stop = self.myDestinationStop;
            os.write_attr(Tags.get_string(stop.get_element()), stop.get_id());
            if stop.get_my_name() != "" {
                comment = format!(" <!-- {} -->", utils::common::string_utils::escape_xml(stop.get_my_name(), true));
            }
        }
        os.write_attr(SUMO_ATTR_SPEED, &self.mySpeed);
        if withRouteLength {
            os.write_attr("routeLength", &(self.mySpeed * steps2time(self.myArrived - self.myDeparted)));
        }
        if withTiming {
            let started = if self.myDeparted >= 0 { time2string(self.myDeparted) } else { "-1".to_string() };
            os.write_attr(SUMO_ATTR_STARTED, &started);
            let ended = if self.myArrived >= 0 { time2string(self.myArrived) } else { "-1".to_string() };
            os.write_attr(SUMO_ATTR_ENDED, &ended);
        }
        if options_cont::get_bool("vehroute-output.cost") {
            os.write_attr(SUMO_ATTR_COST, &self.get_costs());
        }
        os.close_tag(&comment);
    }

    /// C++: `bool MSStageTranship::moveToNextEdge(MSTransportable *, SUMOTime, int, MSEdge *, const bool)` [virtual]
    /// move forward and return whether the container arrived
    // bool
    // MSStageTranship::moveToNextEdge(MSTransportable* transportable, SUMOTime currentTime, int /*prevDir*/, MSEdge* /* nextInternal */, const bool /* isReplay */) {
    //     getEdge()->removeTransportable(transportable);
    //     // transship does a direct move so we are already at our destination
    //     if (myDestinationStop != nullptr) {
    //         myDestinationStop->addTransportable(transportable);    //jakob
    //     }
    //     if (!transportable->proceed(MSNet::getInstance(), currentTime)) {
    //         if (transportable->isPerson()) {
    //             MSNet::getInstance()->getPersonControl().erase(transportable);
    //         } else {
    //             MSNet::getInstance()->getContainerControl().erase(transportable);
    //         }
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn move_to_next_edge(&mut self, container: Ptr<MSTransportable>, currentTime: SUMOTime, prevDir: i32, nextInternal: Ptr<MSEdge>, isReplay: bool) -> bool {
        self.get_edge().remove_transportable(container);
        // transship does a direct move so we are already at our destination
        if !self.myDestinationStop.is_null() {
            self.myDestinationStop.m().add_transportable(container, -1); //jakob
        }
        if !container.m().proceed(MSNet::get_instance(), currentTime, false) {
            if container.is_person() {
                MSNet::get_instance().m().get_person_control().erase(container);
            } else {
                MSNet::get_instance().m().get_container_control().erase(container);
            }
        }
        true
    }

    /// C++: `double MSStageTranship::getMaxSpeed(const MSTransportable *const) const` [const, virtual]
    /// the maximum speed of the transportable
    // inline double getMaxSpeed(const MSTransportable* const transportable = nullptr) const {
    //         UNUSED_PARAMETER(transportable);
    //         return mySpeed;
    //     }
    #[allow(unused_variables)]
    pub fn get_max_speed(&self, transportable: Ptr<MSTransportable>) -> f64 {
        self.mySpeed
    }

    /// C++: `double MSStageTranship::getArrivalPos() const` [const, virtual]
    // inline double getArrivalPos() const {
    //         return myArrivalPos;
    //     }
    #[allow(unused_variables)]
    pub fn get_arrival_pos(&self) -> f64 {
        self.myArrivalPos
    }

    /// C++: `const MSEdge * MSStageTranship::getNextRouteEdge() const` [const, virtual]
    // inline const MSEdge* getNextRouteEdge() const {
    //         return myRouteStep == myRoute.end() - 1 ? 0 : *(myRouteStep + 1);
    //     }
    #[allow(unused_variables)]
    pub fn get_next_route_edge(&self) -> Ptr<MSEdge> {
        if self.myRouteStep == self.myRoute.len() - 1 {
            Ptr::null()
        } else {
            self.myRoute[self.myRouteStep + 1]
        }
    }

    /// C++: `void MSStageTranship::MSStageTranship(const MSStageTranship &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSStageTranship) -> Self {
        panic!("MSStageTranship copy constructor is private and invalidated in C++")
    }

    /// C++: `MSStageTranship & MSStageTranship::operator=(const MSStageTranship &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSStageTranship) -> &mut MSStageTranship {
        panic!("MSStageTranship assignment operator is private and invalidated in C++")
    }

}

