//! Port of libsumo/TrafficLight.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `libsumo::TrafficLight` (libsumo/TrafficLight.h)
pub struct TrafficLight {
}

// C++ static members of TrafficLight (port as module-level statics, see AGENTS.md):
//   static const int TrafficLight::DOMAIN_ID;
//   static SubscriptionResults TrafficLight::mySubscriptionResults;
//   static ContextSubscriptionResults TrafficLight::myContextSubscriptionResults;

impl TrafficLight {
    /// C++: `std::string TrafficLight::getRedYellowGreenState(const std::string &)` [static]
    // std::string
    // TrafficLight::getRedYellowGreenState(const std::string& tlsID) {
    //     return Helper::getTLS(tlsID).getActive()->getCurrentPhaseDef().getState();
    // }
    #[allow(unused_variables)]
    pub fn get_red_yellow_green_state(tlsID: &str) -> String {
        todo!("port TrafficLight::getRedYellowGreenState")
    }

    /// C++: `std::vector<libsumo::TraCILogic> TrafficLight::getAllProgramLogics(const std::string &)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_all_program_logics(tlsID: &str) -> Vec<TraCILogic> {
        todo!("port TrafficLight::getAllProgramLogics")
    }

    /// C++: `std::vector<std::string> TrafficLight::getControlledJunctions(const std::string &)` [static]
    // std::vector<std::string>
    // TrafficLight::getControlledJunctions(const std::string& tlsID) {
    //     std::set<std::string> junctionIDs;
    //     const MSTrafficLightLogic::LinkVectorVector& links = Helper::getTLS(tlsID).getActive()->getLinks();
    //     for (const MSTrafficLightLogic::LinkVector& llinks : links) {
    //         for (const MSLink* l : llinks) {
    //             junctionIDs.insert(l->getJunction()->getID());
    //         }
    //     }
    //     return std::vector<std::string>(junctionIDs.begin(), junctionIDs.end());
    // }
    #[allow(unused_variables)]
    pub fn get_controlled_junctions(tlsID: &str) -> Vec<String> {
        todo!("port TrafficLight::getControlledJunctions")
    }

    /// C++: `std::vector<std::string> TrafficLight::getControlledLanes(const std::string &)` [static]
    // std::vector<std::string>
    // TrafficLight::getControlledLanes(const std::string& tlsID) {
    //     std::vector<std::string> laneIDs;
    //     const MSTrafficLightLogic::LaneVectorVector& lanes = Helper::getTLS(tlsID).getActive()->getLaneVectors();
    //     for (const MSTrafficLightLogic::LaneVector& llanes : lanes) {
    //         for (const MSLane* l : llanes) {
    //             laneIDs.push_back(l->getID());
    //         }
    //     }
    //     return laneIDs;
    // }
    #[allow(unused_variables)]
    pub fn get_controlled_lanes(tlsID: &str) -> Vec<String> {
        todo!("port TrafficLight::getControlledLanes")
    }

    /// C++: `std::vector<std::vector<libsumo::TraCILink>> TrafficLight::getControlledLinks(const std::string &)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_controlled_links(tlsID: &str) -> Vec<Vec<TraCILink>> {
        todo!("port TrafficLight::getControlledLinks")
    }

    /// C++: `std::string TrafficLight::getProgram(const std::string &)` [static]
    // std::string
    // TrafficLight::getProgram(const std::string& tlsID) {
    //     return Helper::getTLS(tlsID).getActive()->getProgramID();
    // }
    #[allow(unused_variables)]
    pub fn get_program(tlsID: &str) -> String {
        todo!("port TrafficLight::getProgram")
    }

    /// C++: `int TrafficLight::getPhase(const std::string &)` [static]
    // int
    // TrafficLight::getPhase(const std::string& tlsID) {
    //     return Helper::getTLS(tlsID).getActive()->getCurrentPhaseIndex();
    // }
    #[allow(unused_variables)]
    pub fn get_phase(tlsID: &str) -> i32 {
        todo!("port TrafficLight::getPhase")
    }

    /// C++: `std::string TrafficLight::getPhaseName(const std::string &)` [static]
    // std::string
    // TrafficLight::getPhaseName(const std::string& tlsID) {
    //     return Helper::getTLS(tlsID).getActive()->getCurrentPhaseDef().getName();
    // }
    #[allow(unused_variables)]
    pub fn get_phase_name(tlsID: &str) -> String {
        todo!("port TrafficLight::getPhaseName")
    }

    /// C++: `double TrafficLight::getPhaseDuration(const std::string &)` [static]
    // double
    // TrafficLight::getPhaseDuration(const std::string& tlsID) {
    //     return STEPS2TIME(Helper::getTLS(tlsID).getActive()->getCurrentPhaseDef().duration);
    // }
    #[allow(unused_variables)]
    pub fn get_phase_duration(tlsID: &str) -> f64 {
        todo!("port TrafficLight::getPhaseDuration")
    }

    /// C++: `double TrafficLight::getNextSwitch(const std::string &)` [static]
    // double
    // TrafficLight::getNextSwitch(const std::string& tlsID) {
    //     return STEPS2TIME(Helper::getTLS(tlsID).getActive()->getNextSwitchTime());
    // }
    #[allow(unused_variables)]
    pub fn get_next_switch(tlsID: &str) -> f64 {
        todo!("port TrafficLight::getNextSwitch")
    }

    /// C++: `double TrafficLight::getSpentDuration(const std::string &)` [static]
    // double
    // TrafficLight::getSpentDuration(const std::string& tlsID) {
    //     return STEPS2TIME(Helper::getTLS(tlsID).getActive()->getSpentDuration());
    // }
    #[allow(unused_variables)]
    pub fn get_spent_duration(tlsID: &str) -> f64 {
        todo!("port TrafficLight::getSpentDuration")
    }

    /// C++: `int TrafficLight::getServedPersonCount(const std::string &, int)` [static]
    // int
    // TrafficLight::getServedPersonCount(const std::string& tlsID, int index) {
    //     MSTrafficLightLogic* const active = Helper::getTLS(tlsID).getActive();
    //     if (index < 0 || active->getPhaseNumber() <= index) {
    //         throw TraCIException("The phase index " + toString(index) + " is not in the allowed range [0,"
    //                              + toString(active->getPhaseNumber() - 1) + "].");
    //     }
    //     // find all crossings which have a green light in that phas
    //     int result = 0;
    //
    //     const std::string& state = active->getPhases()[index]->getState();
    //     for (int i = 0; i < (int)state.size(); i++) {
    //         if (state[i] == LINKSTATE_TL_GREEN_MAJOR) {
    //             for (MSLink* link : active->getLinksAt(i)) {
    //                 if (link->getLane()->isCrossing()) {
    //                     // walking forwards across
    //                     for (MSTransportable* person : link->getLaneBefore()->getEdge().getPersons()) {
    //                         if (static_cast<MSPerson*>(person)->getNextEdge() == link->getLane()->getEdge().getID()) {
    //                             result += 1;
    //                         }
    //                     }
    //                     // walking backwards across
    //                     MSLane* walkingAreaAcross = link->getLane()->getLinkCont().front()->getLane();
    //                     for (MSTransportable* person : walkingAreaAcross->getEdge().getPersons()) {
    //                         if (static_cast<MSPerson*>(person)->getNextEdge() == link->getLane()->getEdge().getID()) {
    //                             result += 1;
    //                         }
    //                     }
    //                 } else if (link->getLaneBefore()->isCrossing()) {
    //                     // walking backwards across (in case both sides are separately controlled)
    //                     for (MSTransportable* person : link->getLane()->getEdge().getPersons()) {
    //                         if (static_cast<MSPerson*>(person)->getNextEdge() == link->getLaneBefore()->getEdge().getID()) {
    //                             result += 1;
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_served_person_count(tlsID: &str, index: i32) -> i32 {
        todo!("port TrafficLight::getServedPersonCount")
    }

    /// C++: `std::vector<std::string> TrafficLight::getBlockingVehicles(const std::string &, int)` [static]
    // std::vector<std::string>
    // TrafficLight::getBlockingVehicles(const std::string& tlsID, int linkIndex) {
    //     std::vector<std::string> result;
    //     // for railsignals we cannot use the "online" program
    //     MSTrafficLightLogic* const active = Helper::getTLS(tlsID).getDefault();
    //     if (linkIndex < 0 || linkIndex >= active->getNumLinks()) {
    //         throw TraCIException("The link index " + toString(linkIndex) + " is not in the allowed range [0,"
    //                              + toString(active->getNumLinks() - 1) + "].");
    //     }
    //     for (const SUMOVehicle* veh : active->getBlockingVehicles(linkIndex)) {
    //         result.push_back(veh->getID());
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_blocking_vehicles(tlsID: &str, linkIndex: i32) -> Vec<String> {
        todo!("port TrafficLight::getBlockingVehicles")
    }

    /// C++: `std::vector<std::string> TrafficLight::getRivalVehicles(const std::string &, int)` [static]
    // std::vector<std::string>
    // TrafficLight::getRivalVehicles(const std::string& tlsID, int linkIndex) {
    //     std::vector<std::string> result;
    //     MSTrafficLightLogic* const active = Helper::getTLS(tlsID).getDefault();
    //     if (linkIndex < 0 || linkIndex >= active->getNumLinks()) {
    //         throw TraCIException("The link index " + toString(linkIndex) + " is not in the allowed range [0,"
    //                              + toString(active->getNumLinks() - 1) + "].");
    //     }
    //     for (const SUMOVehicle* veh : active->getRivalVehicles(linkIndex)) {
    //         result.push_back(veh->getID());
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_rival_vehicles(tlsID: &str, linkIndex: i32) -> Vec<String> {
        todo!("port TrafficLight::getRivalVehicles")
    }

    /// C++: `std::vector<std::string> TrafficLight::getPriorityVehicles(const std::string &, int)` [static]
    // std::vector<std::string>
    // TrafficLight::getPriorityVehicles(const std::string& tlsID, int linkIndex) {
    //     std::vector<std::string> result;
    //     MSTrafficLightLogic* const active = Helper::getTLS(tlsID).getDefault();
    //     if (linkIndex < 0 || linkIndex >= active->getNumLinks()) {
    //         throw TraCIException("The link index " + toString(linkIndex) + " is not in the allowed range [0,"
    //                              + toString(active->getNumLinks() - 1) + "].");
    //     }
    //     for (const SUMOVehicle* veh : active->getPriorityVehicles(linkIndex)) {
    //         result.push_back(veh->getID());
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_priority_vehicles(tlsID: &str, linkIndex: i32) -> Vec<String> {
        todo!("port TrafficLight::getPriorityVehicles")
    }

    /// C++: `std::vector<libsumo::TraCISignalConstraint> TrafficLight::getConstraints(const std::string &, const std::string &)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_constraints(tlsID: &str, tripId: &str) -> Vec<TraCISignalConstraint> {
        todo!("port TrafficLight::getConstraints")
    }

    /// C++: `std::vector<libsumo::TraCISignalConstraint> TrafficLight::getConstraintsByFoe(const std::string &, const std::string &)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_constraints_by_foe(foeSignal: &str, foeId: &str) -> Vec<TraCISignalConstraint> {
        todo!("port TrafficLight::getConstraintsByFoe")
    }

    /// C++: `std::vector<std::string> TrafficLight::getIDList()` [static]
    // std::vector<std::string>
    // TrafficLight::getIDList() {
    //     return MSNet::getInstance()->getTLSControl().getAllTLIds();
    // }
    #[allow(unused_variables)]
    pub fn get_id_list() -> Vec<String> {
        todo!("port TrafficLight::getIDList")
    }

    /// C++: `int TrafficLight::getIDCount()` [static]
    // int
    // TrafficLight::getIDCount() {
    //     return (int)getIDList().size();
    // }
    #[allow(unused_variables)]
    pub fn get_id_count() -> i32 {
        todo!("port TrafficLight::getIDCount")
    }

    /// C++: `std::string TrafficLight::getParameter(const std::string &, const std::string &)` [static]
    // std::string
    // TrafficLight::getParameter(const std::string& tlsID, const std::string& paramName) {
    //     MSTrafficLightLogic* tll = Helper::getTLS(tlsID).getActive();
    //     if (StringUtils::startsWith(paramName, "NEMA.") && tll->getLogicType() != TrafficLightType::NEMA) {
    //         throw TraCIException("'" + tlsID + "' is not a NEMA controller");
    //     }
    //     return tll->getParameter(paramName, "");
    // }
    #[allow(unused_variables)]
    pub fn get_parameter(objectID: &str, key: &str) -> String {
        todo!("port TrafficLight::getParameter")
    }

    /// C++: `const std::pair<std::string, std::string> TrafficLight::getParameterWithKey(const std::string &, const std::string &)` [static]
    // LIBSUMO_GET_PARAMETER_WITH_KEY_IMPLEMENTATION(TrafficLight)
    #[allow(unused_variables)]
    pub fn get_parameter_with_key(objectID: &str, key: &str) -> (String, String) {
        todo!("port TrafficLight::getParameterWithKey")
    }

    /// C++: `void TrafficLight::setParameter(const std::string &, const std::string &, const std::string &)` [static]
    // void
    // TrafficLight::setParameter(const std::string& tlsID, const std::string& paramName, const std::string& value) {
    //     MSTrafficLightLogic* tll = Helper::getTLS(tlsID).getActive();
    //     if (StringUtils::startsWith(paramName, "NEMA.") && tll->getLogicType() != TrafficLightType::NEMA) {
    //         throw TraCIException("'" + tlsID + "' is not a NEMA controller");
    //     }
    //     tll->setParameter(paramName, value);
    // }
    #[allow(unused_variables)]
    pub fn set_parameter(objectID: &str, key: &str, value: &str) {
        todo!("port TrafficLight::setParameter")
    }

    /// C++: `void TrafficLight::subscribe(const std::string &, const std::vector<int> &, double, double, const libsumo::TraCIResults &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(TrafficLight, TL)
    #[allow(unused_variables)]
    pub fn subscribe(objectID: &str, varIDs: &Vec<i32>, begin: f64, end: f64, parameters: &TraCIResults) {
        todo!("port TrafficLight::subscribe")
    }

    /// C++: `void TrafficLight::unsubscribe(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(TrafficLight, TL)
    #[allow(unused_variables)]
    pub fn unsubscribe(objectID: &str) {
        todo!("port TrafficLight::unsubscribe")
    }

    /// C++: `void TrafficLight::subscribeContext(const std::string &, int, double, const std::vector<int> &, double, double, const libsumo::TraCIResults &)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn subscribe_context(objectID: &str, domain: i32, dist: f64, varIDs: &Vec<i32>, begin: f64, end: f64, parameters: &TraCIResults) {
        todo!("port TrafficLight::subscribeContext")
    }

    /// C++: `void TrafficLight::unsubscribeContext(const std::string &, int, double)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(TrafficLight, TL)
    #[allow(unused_variables)]
    pub fn unsubscribe_context(objectID: &str, domain: i32, dist: f64) {
        todo!("port TrafficLight::unsubscribeContext")
    }

    /// C++: `const libsumo::SubscriptionResults TrafficLight::getAllSubscriptionResults()` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(TrafficLight, TL)
    #[allow(unused_variables)]
    pub fn get_all_subscription_results() -> SubscriptionResults {
        todo!("port TrafficLight::getAllSubscriptionResults")
    }

    /// C++: `const libsumo::TraCIResults TrafficLight::getSubscriptionResults(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(TrafficLight, TL)
    #[allow(unused_variables)]
    pub fn get_subscription_results(objectID: &str) -> TraCIResults {
        todo!("port TrafficLight::getSubscriptionResults")
    }

    /// C++: `const libsumo::ContextSubscriptionResults TrafficLight::getAllContextSubscriptionResults()` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(TrafficLight, TL)
    #[allow(unused_variables)]
    pub fn get_all_context_subscription_results() -> ContextSubscriptionResults {
        todo!("port TrafficLight::getAllContextSubscriptionResults")
    }

    /// C++: `const libsumo::SubscriptionResults TrafficLight::getContextSubscriptionResults(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(TrafficLight, TL)
    #[allow(unused_variables)]
    pub fn get_context_subscription_results(objectID: &str) -> SubscriptionResults {
        todo!("port TrafficLight::getContextSubscriptionResults")
    }

    /// C++: `void TrafficLight::subscribeParameterWithKey(const std::string &, const std::string &, double, double)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(TrafficLight, TL)
    #[allow(unused_variables)]
    pub fn subscribe_parameter_with_key(objectID: &str, key: &str, beginTime: f64, endTime: f64) {
        todo!("port TrafficLight::subscribeParameterWithKey")
    }

    /// C++: `int TrafficLight::domainID()` [static]
    // LIBSUMO_SUBSCRIPTION_API
    #[allow(unused_variables)]
    pub fn domain_id() -> i32 {
        todo!("port TrafficLight::domainID")
    }

    /// C++: `void TrafficLight::setRedYellowGreenState(const std::string &, const std::string &)` [static]
    // void
    // TrafficLight::setRedYellowGreenState(const std::string& tlsID, const std::string& state) {
    //     Helper::getTLS(tlsID).setStateInstantiatingOnline(MSNet::getInstance()->getTLSControl(), state);
    // }
    #[allow(unused_variables)]
    pub fn set_red_yellow_green_state(tlsID: &str, state: &str) {
        todo!("port TrafficLight::setRedYellowGreenState")
    }

    /// C++: `void TrafficLight::setPhase(const std::string &, const int)` [static]
    // void
    // TrafficLight::setPhase(const std::string& tlsID, const int index) {
    //     MSTrafficLightLogic* const active = Helper::getTLS(tlsID).getActive();
    //     if (index < 0 || active->getPhaseNumber() <= index) {
    //         throw TraCIException("The phase index " + toString(index) + " is not in the allowed range [0,"
    //                              + toString(active->getPhaseNumber() - 1) + "].");
    //     }
    //     const SUMOTime cTime = MSNet::getInstance()->getCurrentTimeStep();
    //     const SUMOTime duration = active->getPhase(index).duration;
    //     active->changeStepAndDuration(MSNet::getInstance()->getTLSControl(), cTime, index, duration);
    // }
    #[allow(unused_variables)]
    pub fn set_phase(tlsID: &str, index: i32) {
        todo!("port TrafficLight::setPhase")
    }

    /// C++: `void TrafficLight::setPhaseName(const std::string &, const std::string &)` [static]
    // void
    // TrafficLight::setPhaseName(const std::string& tlsID, const std::string& name) {
    //     MSTrafficLightLogic* const active = Helper::getTLS(tlsID).getActive();
    //     const_cast<MSPhaseDefinition&>(active->getCurrentPhaseDef()).setName(name);
    // }
    #[allow(unused_variables)]
    pub fn set_phase_name(tlsID: &str, name: &str) {
        todo!("port TrafficLight::setPhaseName")
    }

    /// C++: `void TrafficLight::setProgram(const std::string &, const std::string &)` [static]
    // void
    // TrafficLight::setProgram(const std::string& tlsID, const std::string& programID) {
    //     try {
    //         Helper::getTLS(tlsID).switchTo(MSNet::getInstance()->getTLSControl(), programID);
    //     } catch (ProcessError& e) {
    //         throw TraCIException(e.what());
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_program(tlsID: &str, programID: &str) {
        todo!("port TrafficLight::setProgram")
    }

    /// C++: `void TrafficLight::setPhaseDuration(const std::string &, const double)` [static]
    // void
    // TrafficLight::setPhaseDuration(const std::string& tlsID, const double phaseDuration) {
    //     MSTrafficLightLogic* const active = Helper::getTLS(tlsID).getActive();
    //     const SUMOTime cTime = MSNet::getInstance()->getCurrentTimeStep();
    //     active->changeStepAndDuration(MSNet::getInstance()->getTLSControl(), cTime, -1, TIME2STEPS(phaseDuration));
    // }
    #[allow(unused_variables)]
    pub fn set_phase_duration(tlsID: &str, phaseDuration: f64) {
        todo!("port TrafficLight::setPhaseDuration")
    }

    /// C++: `void TrafficLight::setProgramLogic(const std::string &, const libsumo::TraCILogic &)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn set_program_logic(tlsID: &str, logic: &TraCILogic) {
        todo!("port TrafficLight::setProgramLogic")
    }

    /// C++: `void TrafficLight::addConstraint(const std::string &, const std::string &, const std::string &, const std::string &, const int, const int)` [static]
    // void
    // TrafficLight::addConstraint(const std::string& tlsID, const std::string& tripId, const std::string& foeSignal, const std::string& foeId, const int type, const int limit) {
    //     MSTrafficLightLogic* const active = Helper::getTLS(tlsID).getDefault();
    //     MSTrafficLightLogic* const active2 = Helper::getTLS(foeSignal).getDefault();
    //     MSRailSignal* s = dynamic_cast<MSRailSignal*>(active);
    //     MSRailSignal* s2 = dynamic_cast<MSRailSignal*>(active2);
    //     if (s == nullptr) {
    //         throw TraCIException("'" + tlsID + "' is not a rail signal");
    //     }
    //     if (s2 == nullptr) {
    //         throw TraCIException("'" + foeSignal + "' is not a rail signal");
    //     }
    //     MSRailSignalConstraint* c = new MSRailSignalConstraint_Predecessor((MSRailSignalConstraint::ConstraintType)type, s2, foeId, limit, true);
    //     s->addConstraint(tripId, c);
    // }
    #[allow(unused_variables)]
    pub fn add_constraint(tlsID: &str, tripId: &str, foeSignal: &str, foeId: &str, type_: i32, limit: i32) {
        todo!("port TrafficLight::addConstraint")
    }

    /// C++: `std::vector<libsumo::TraCISignalConstraint> TrafficLight::swapConstraints(const std::string &, const std::string &, const std::string &, const std::string &)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn swap_constraints(tlsID: &str, tripId: &str, foeSignal: &str, foeId: &str) -> Vec<TraCISignalConstraint> {
        todo!("port TrafficLight::swapConstraints")
    }

    /// C++: `void TrafficLight::removeConstraints(const std::string &, const std::string &, const std::string &, const std::string &)` [static]
    // void
    // TrafficLight::removeConstraints(const std::string& tlsID, const std::string& tripId, const std::string& foeSignal, const std::string& foeId) {
    //     // remove all constraints that have the given foeId
    //     // @note could improve efficiency by storing a map of rail signals in MSRailSignalControl
    //     for (const std::string& tlsCand : getIDList()) {
    //         if (tlsID == "" || tlsCand == tlsID) {
    //             MSTrafficLightLogic* const active = Helper::getTLS(tlsCand).getDefault();
    //             MSRailSignal* s = dynamic_cast<MSRailSignal*>(active);
    //             if (s != nullptr) {
    //                 for (auto item : s->getConstraints()) {
    //                     if (tripId == "" || item.first == tripId) {
    //                         for (MSRailSignalConstraint* cand : item.second) {
    //                             MSRailSignalConstraint_Predecessor* pc = dynamic_cast<MSRailSignalConstraint_Predecessor*>(cand);
    //                             if (pc != nullptr
    //                                     && (foeId == "" || pc->myTripId == foeId)
    //                                     && (foeSignal == "" || pc->myFoeSignal->getID() == foeSignal)) {
    //                                 cand->setActive(false);
    //                             }
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn remove_constraints(tlsID: &str, tripId: &str, foeSignal: &str, foeId: &str) {
        todo!("port TrafficLight::removeConstraints")
    }

    /// C++: `void TrafficLight::updateConstraints(const std::string &, std::string)` [static]
    // void
    // TrafficLight::updateConstraints(const std::string& vehID, std::string tripId) {
    //     // Removes all constraints that can no longer be met because the route of
    //     // vehID does not pass the signal involved in the constraint with the given tripId.
    //     // This includes constraints on tripId as well as constraints where tripId is the foeId.
    //
    //     MSBaseVehicle* veh = Helper::getVehicle(vehID);
    //     std::string curTripId = veh->getParameter().getParameter("tripId", veh->getID());
    //     tripId = tripId == "" ? curTripId : tripId;
    //
    //     // find signals and tripId along the route of veh
    //     std::map<const MSRailSignal*, std::set<std::string> > onRoute;
    //     const ConstMSEdgeVector& route = veh->getRoute().getEdges();
    //     auto routeIt = veh->getCurrentRouteEdge();
    //     for (const MSStop& stop : veh->getStops()) {
    //         for (auto it = routeIt; it < stop.edge; it++) {
    //             const MSEdge* edge = *it;
    //             if (edge->getToJunction()->getType() == SumoXMLNodeType::RAIL_SIGNAL) {
    //                 if (it + 1 != route.end()) {
    //                     const MSEdge* next = *(it + 1);
    //                     const MSLink* link = edge->getLanes()[0]->getLinkTo(next->getLanes()[0]);
    //                     if (link != nullptr && link->getTLLogic() != nullptr) {
    //                         const MSRailSignal* s = dynamic_cast<const MSRailSignal*>(link->getTLLogic());
    //                         onRoute[s].insert(curTripId);
    //                     }
    //                 }
    //             }
    //         }
    //         if (stop.pars.tripId != "") {
    //             curTripId = stop.pars.tripId;
    //         }
    //         routeIt = stop.edge;
    //     }
    //     for (auto it = routeIt; it < route.end(); it++) {
    //         const MSEdge* edge = *it;
    //         if (edge->getToJunction()->getType() == SumoXMLNodeType::RAIL_SIGNAL) {
    //             if (it + 1 != route.end()) {
    //                 const MSEdge* next = *(it + 1);
    //                 const MSLink* link = edge->getLanes()[0]->getLinkTo(next->getLanes()[0]);
    //                 if (link != nullptr && link->getTLLogic() != nullptr) {
    //                     const MSRailSignal* s = dynamic_cast<const MSRailSignal*>(link->getTLLogic());
    //                     onRoute[s].insert(curTripId);
    //                 }
    //             }
    //         }
    //     }
    //     //for (auto item : onRoute) {
    //     //    std::cout << " s=" << item.first->getID() << " @" << item.first <<  " ids=" << toString(item.second) << "\n";
    //     //}
    //
    //     // check relevance for all active contraints
    //     for (MSRailSignal* s : MSRailSignalControl::getInstance().getSignals()) {
    //
    //         // record outdated constraints on and by the vehicle
    //         std::vector<MSRailSignalConstraint*> onVeh;
    //         std::vector<std::pair<std::string, MSRailSignalConstraint*> > byVeh;
    //
    //         for (auto item : s->getConstraints()) {
    //             for (MSRailSignalConstraint* cand : item.second) {
    //                 MSRailSignalConstraint_Predecessor* pc = dynamic_cast<MSRailSignalConstraint_Predecessor*>(cand);
    //                 if (pc != nullptr && !pc->cleared() && pc->isActive()) {
    //                     if (item.first == tripId) {
    //                         if (onRoute[s].count(tripId) == 0) {
    //                             // constraint on our veh no longer relevant
    //                             onVeh.push_back(cand);
    //                         }
    //                     } else if (pc->myTripId == tripId) {
    //                         if (onRoute[pc->myFoeSignal].count(tripId) == 0) {
    //                             // constraint by our veh no longer relevant
    //                             byVeh.push_back(std::make_pair(item.first, cand));
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //         for (MSRailSignalConstraint* c : onVeh) {
    //             s->removeConstraint(tripId, c);
    //         }
    //         for (auto item : byVeh) {
    //             s->removeConstraint(item.first, item.second);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn update_constraints(vehID: &str, tripId: String) {
        todo!("port TrafficLight::updateConstraints")
    }

    /// C++: `std::vector<libsumo::TraCILogic> TrafficLight::getCompleteRedYellowGreenDefinition(const std::string &)` [static]
    // static inline std::vector<libsumo::TraCILogic> getCompleteRedYellowGreenDefinition(const std::string& tlsID) {
    //         return getAllProgramLogics(tlsID);
    //     }
    #[allow(unused_variables)]
    pub fn get_complete_red_yellow_green_definition(tlsID: &str) -> Vec<TraCILogic> {
        todo!("port TrafficLight::getCompleteRedYellowGreenDefinition")
    }

    /// C++: `void TrafficLight::setCompleteRedYellowGreenDefinition(const std::string &, const libsumo::TraCILogic &)` [static]
    // static inline void setCompleteRedYellowGreenDefinition(const std::string& tlsID, const libsumo::TraCILogic& logic) {
    //         setProgramLogic(tlsID, logic);
    //     }
    #[allow(unused_variables)]
    pub fn set_complete_red_yellow_green_definition(tlsID: &str, logic: &TraCILogic) {
        todo!("port TrafficLight::setCompleteRedYellowGreenDefinition")
    }

    /// C++: `void TrafficLight::setNemaSplits(const std::string &, const std::vector<double> &)` [static]
    // void
    // TrafficLight::setNemaSplits(const std::string& tlsID, const std::vector<double>& splits) {
    //     setParameter(tlsID, "NEMA.splits", toString(splits));
    // }
    #[allow(unused_variables)]
    pub fn set_nema_splits(tlsID: &str, splits: &Vec<f64>) {
        todo!("port TrafficLight::setNemaSplits")
    }

    /// C++: `void TrafficLight::setNemaMaxGreens(const std::string &, const std::vector<double> &)` [static]
    // void
    // TrafficLight::setNemaMaxGreens(const std::string& tlsID, const std::vector<double>& maxGreens) {
    //     setParameter(tlsID, "NEMA.maxGreens", toString(maxGreens));
    // }
    #[allow(unused_variables)]
    pub fn set_nema_max_greens(tlsID: &str, maxGreens: &Vec<f64>) {
        todo!("port TrafficLight::setNemaMaxGreens")
    }

    /// C++: `void TrafficLight::setNemaCycleLength(const std::string &, double)` [static]
    // void
    // TrafficLight::setNemaCycleLength(const std::string& tlsID, double cycleLength) {
    //     setParameter(tlsID, "NEMA.cycleLength", toString(cycleLength));
    // }
    #[allow(unused_variables)]
    pub fn set_nema_cycle_length(tlsID: &str, cycleLength: f64) {
        todo!("port TrafficLight::setNemaCycleLength")
    }

    /// C++: `void TrafficLight::setNemaOffset(const std::string &, double)` [static]
    // void
    // TrafficLight::setNemaOffset(const std::string& tlsID, double offset) {
    //     setParameter(tlsID, "NEMA.offset", toString(offset));
    // }
    #[allow(unused_variables)]
    pub fn set_nema_offset(tlsID: &str, offset: f64) {
        todo!("port TrafficLight::setNemaOffset")
    }

    /// C++: `std::shared_ptr<VariableWrapper> TrafficLight::makeWrapper()` [static]
    // std::shared_ptr<VariableWrapper>
    // TrafficLight::makeWrapper() {
    //     return std::make_shared<Helper::SubscriptionWrapper>(handleVariable, mySubscriptionResults, myContextSubscriptionResults);
    // }
    #[allow(unused_variables)]
    pub fn make_wrapper() -> Rc<VariableWrapper> {
        todo!("port TrafficLight::makeWrapper")
    }

    /// C++: `bool TrafficLight::handleVariable(const std::string &, const int, VariableWrapper *, tcpip::Storage *)` [static]
    // bool
    // TrafficLight::handleVariable(const std::string& objID, const int variable, VariableWrapper* wrapper, tcpip::Storage* paramData) {
    //     switch (variable) {
    //         case TRACI_ID_LIST:
    //             return wrapper->wrapStringList(objID, variable, getIDList());
    //         case ID_COUNT:
    //             return wrapper->wrapInt(objID, variable, getIDCount());
    //         case TL_RED_YELLOW_GREEN_STATE:
    //             return wrapper->wrapString(objID, variable, getRedYellowGreenState(objID));
    //         case TL_COMPLETE_DEFINITION_RYG:
    //             return wrapper->wrapLogicVector(objID, variable, getAllProgramLogics(objID));
    //         case TL_CONTROLLED_LANES:
    //             return wrapper->wrapStringList(objID, variable, getControlledLanes(objID));
    //         case TL_CONTROLLED_LINKS:
    //             return wrapper->wrapLinkVectorVector(objID, variable, getControlledLinks(objID));
    //         case TL_CURRENT_PHASE:
    //             return wrapper->wrapInt(objID, variable, getPhase(objID));
    //         case VAR_NAME:
    //             return wrapper->wrapString(objID, variable, getPhaseName(objID));
    //         case TL_CURRENT_PROGRAM:
    //             return wrapper->wrapString(objID, variable, getProgram(objID));
    //         case TL_PHASE_DURATION:
    //             return wrapper->wrapDouble(objID, variable, getPhaseDuration(objID));
    //         case TL_NEXT_SWITCH:
    //             return wrapper->wrapDouble(objID, variable, getNextSwitch(objID));
    //         case TL_SPENT_DURATION:
    //             return wrapper->wrapDouble(objID, variable, getSpentDuration(objID));
    //         case VAR_PERSON_NUMBER:
    //             return wrapper->wrapInt(objID, variable, getServedPersonCount(objID, StoHelp::readTypedInt(*paramData)));
    //         case TL_BLOCKING_VEHICLES:
    //             return wrapper->wrapStringList(objID, variable, getBlockingVehicles(objID, StoHelp::readTypedInt(*paramData)));
    //         case TL_RIVAL_VEHICLES:
    //             return wrapper->wrapStringList(objID, variable, getRivalVehicles(objID, StoHelp::readTypedInt(*paramData)));
    //         case TL_PRIORITY_VEHICLES:
    //             return wrapper->wrapStringList(objID, variable, getPriorityVehicles(objID, StoHelp::readTypedInt(*paramData)));
    //         case TL_CONSTRAINT:
    //             return wrapper->wrapSignalConstraintVector(objID, variable, getConstraints(objID, StoHelp::readTypedString(*paramData)));
    //         case TL_CONSTRAINT_BYFOE:
    //             return wrapper->wrapSignalConstraintVector(objID, variable, getConstraintsByFoe(objID, StoHelp::readTypedString(*paramData)));
    //         case TL_CONTROLLED_JUNCTIONS:
    //             return wrapper->wrapStringList(objID, variable, getControlledJunctions(objID));
    //         case VAR_PARAMETER:
    //             return wrapper->wrapString(objID, variable, getParameter(objID, StoHelp::readTypedString(*paramData)));
    //         case VAR_PARAMETER_WITH_KEY:
    //             return wrapper->wrapStringPair(objID, variable, getParameterWithKey(objID, StoHelp::readTypedString(*paramData)));
    //         default:
    //             return false;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn handle_variable(objID: &str, variable: i32, wrapper: Ptr<VariableWrapper>, paramData: Ptr<Storage>) -> bool {
        todo!("port TrafficLight::handleVariable")
    }

    /// C++: `libsumo::TraCISignalConstraint TrafficLight::buildConstraint(const std::string &, const std::string &, MSRailSignalConstraint *)` [static, private]
    // libsumo::TraCISignalConstraint
    // TrafficLight::buildConstraint(const std::string& tlsID, const std::string& tripId, MSRailSignalConstraint* constraint) {
    //     TraCISignalConstraint c;
    //     c.tripId = tripId;
    //     MSRailSignalConstraint_Predecessor* pc = dynamic_cast<MSRailSignalConstraint_Predecessor*>(constraint);
    //     if (pc == nullptr) {
    //         // unsupported constraint
    //         c.type = -1;
    //     } else {
    //         c.signalId = tlsID;
    //         c.foeId = pc->myTripId;
    //         c.foeSignal = pc->myFoeSignal->getID();
    //         c.limit = pc->myLimit;
    //         c.type = pc->getType();
    //         c.mustWait = !pc->cleared() && pc->isActive();
    //         c.active = pc->isActive();
    //         c.param = constraint->getParametersMap();
    //     }
    //     return c;
    // }
    #[allow(unused_variables)]
    pub fn build_constraint(tlsID: &str, tripId: &str, constraint: Ptr<MSRailSignalConstraint>) -> TraCISignalConstraint {
        todo!("port TrafficLight::buildConstraint")
    }

    /// C++: `std::vector<libsumo::TraCISignalConstraint> TrafficLight::findConstraintsDeadLocks(const std::string &, const std::string &, const std::string &, const std::string &)` [static, private]
    /// perform swapConstraints to resolve deadlocks and return the new constraints
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn find_constraints_dead_locks(foeId: &str, tripId: &str, foeSignal: &str, tlsID: &str) -> Vec<TraCISignalConstraint> {
        todo!("port TrafficLight::findConstraintsDeadLocks")
    }

    /// C++: `SUMOVehicle * TrafficLight::getVehicleByTripId(const std::string)` [static, private]
    // SUMOVehicle*
    // TrafficLight::getVehicleByTripId(const std::string tripOrVehID) {
    //     MSVehicleControl& c = MSNet::getInstance()->getVehicleControl();
    //     for (MSVehicleControl::constVehIt i = c.loadedVehBegin(); i != c.loadedVehEnd(); ++i) {
    //         SUMOVehicle* veh = i->second;
    //         if (veh->getParameter().getParameter("tripId", veh->getID()) == tripOrVehID) {
    //             return veh;
    //         }
    //     }
    //     return nullptr;
    // }
    #[allow(unused_variables)]
    pub fn get_vehicle_by_trip_id(tripOrVehID: String) -> Ptr<SUMOVehicle> {
        todo!("port TrafficLight::getVehicleByTripId")
    }

    /// C++: `std::vector<std::string> TrafficLight::getFutureTripIds(const std::string)` [static, private]
    // std::vector<std::string>
    // TrafficLight::getFutureTripIds(const std::string vehID) {
    //     std::vector<std::string> result;
    //     MSBaseVehicle* veh = dynamic_cast<MSBaseVehicle*>(MSNet::getInstance()->getVehicleControl().getVehicle(vehID));
    //     if (veh) {
    //         std::string tripId = veh->getParameter().getParameter("tripId");
    //         if (tripId != "") {
    //             result.push_back(tripId);
    //         }
    //         for (const MSStop& stop : veh->getStops()) {
    //             if (stop.pars.tripId != "") {
    //                 result.push_back(stop.pars.tripId);
    //             }
    //         }
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_future_trip_ids(vehID: String) -> Vec<String> {
        todo!("port TrafficLight::getFutureTripIds")
    }

    /// C++: `void TrafficLight::swapParameters(MSRailSignalConstraint *)` [static, private]
    // void
    // TrafficLight::swapParameters(MSRailSignalConstraint* c) {
    //     // swap parameters that were assigned by generateRailSignalConstraints.py
    //     for (auto keys : getSwapParams(c->getType())) {
    //         swapParameters(c, keys.first, keys.second);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn swap_parameters(c: Ptr<MSRailSignalConstraint>) {
        todo!("port TrafficLight::swapParameters")
    }

    /// C++: `void TrafficLight::swapParameters(MSRailSignalConstraint *, const std::string &, const std::string &)` [static, private]
    // void
    // TrafficLight::swapParameters(MSRailSignalConstraint* c, const std::string& key1, const std::string& key2) {
    //     const std::string value1 = c->getParameter(key1);
    //     const std::string value2 = c->getParameter(key2);
    //     if (value1 != "") {
    //         c->setParameter(key2, value1);
    //     } else {
    //         c->unsetParameter(key2);
    //     }
    //     if (value2 != "") {
    //         c->setParameter(key1, value2);
    //     } else {
    //         c->unsetParameter(key1);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn swap_parameters_2(c: Ptr<MSRailSignalConstraint>, key1: &str, key2: &str) {
        todo!("port TrafficLight::swapParameters")
    }

    /// C++: `void TrafficLight::swapParameters(TraCISignalConstraint &)` [static, private]
    // void
    // TrafficLight::swapParameters(TraCISignalConstraint& c) {
    //     // swap parameters that were assigned by generateRailSignalConstraints.py
    //     for (auto keys : getSwapParams(c.type)) {
    //         swapParameters(c, keys.first, keys.second);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn swap_parameters_3(c: &mut TraCISignalConstraint) {
        todo!("port TrafficLight::swapParameters")
    }

    /// C++: `void TrafficLight::swapParameters(TraCISignalConstraint &, const std::string &, const std::string &)` [static, private]
    // void
    // TrafficLight::swapParameters(TraCISignalConstraint& c, const std::string& key1, const std::string& key2) {
    //     auto it1 = c.param.find(key1);
    //     auto it2 = c.param.find(key2);
    //     const std::string value1 = it1 != c.param.end() ? it1->second : "";
    //     const std::string value2 = it2 != c.param.end() ? it2->second : "";
    //     if (value1 != "") {
    //         c.param[key2] = value1;
    //     } else {
    //         c.param.erase(key2);
    //     }
    //     if (value2 != "") {
    //         c.param[key1] = value2;
    //     } else {
    //         c.param.erase(key1);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn swap_parameters_4(c: &mut TraCISignalConstraint, key1: &str, key2: &str) {
        todo!("port TrafficLight::swapParameters")
    }

    /// C++: `std::vector<std::pair<std::string, std::string>> TrafficLight::getSwapParams(int)` [static, private]
    // std::vector<std::pair<std::string, std::string> >
    // TrafficLight::getSwapParams(int constraintType) {
    //     std::vector<std::pair<std::string, std::string> > result({
    //         {"vehID", "foeID"},
    //         {"line", "foeLine"},
    //         {"arrival", "foeArrival"}});
    //
    //     if (constraintType == MSRailSignalConstraint::ConstraintType::BIDI_PREDECESSOR) {
    //         std::vector<std::pair<std::string, std::string> > special({
    //             {"busStop", "busStop2"},
    //             {"priorStop", "priorStop2"},
    //             {"stopArrival", "foeStopArrival"}});
    //         result.insert(result.end(), special.begin(), special.end());
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_swap_params(constraintType: i32) -> Vec<(String, String)> {
        todo!("port TrafficLight::getSwapParams")
    }

    /// C++: `void TrafficLight::TrafficLight()` [private]
    /// invalidated standard constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new() -> Self {
        todo!("port TrafficLight::TrafficLight")
    }

}

