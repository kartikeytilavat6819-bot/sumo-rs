//! Port of libsumo/TrafficLight.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::string_utils::starts_with;
use utils::common::time::{steps2time, time2steps};
// shortcut: C++ `throw TraCIException(msg)` is mapped to `panic!("{}", msg)` (unwinds where C++ throws), upgrade when libsumo has a typed TraCI error that callers can catch

/// C++ class `libsumo::TrafficLight` (libsumo/TrafficLight.h)
pub struct TrafficLight {
}

// C++ static members of TrafficLight (port as module-level statics, see AGENTS.md):
//   static const int TrafficLight::DOMAIN_ID;
//   static SubscriptionResults TrafficLight::mySubscriptionResults;
//   static ContextSubscriptionResults TrafficLight::myContextSubscriptionResults;

thread_local! {
    /// C++ static `TrafficLight::mySubscriptionResults`
    static MY_SUBSCRIPTION_RESULTS: std::cell::RefCell<SubscriptionResults> = std::cell::RefCell::new(SubscriptionResults::new());
    /// C++ static `TrafficLight::myContextSubscriptionResults`
    static MY_CONTEXT_SUBSCRIPTION_RESULTS: std::cell::RefCell<ContextSubscriptionResults> = std::cell::RefCell::new(ContextSubscriptionResults::new());
}

impl TrafficLight {
    /// C++: `std::string TrafficLight::getRedYellowGreenState(const std::string &)` [static]
    // std::string
    // TrafficLight::getRedYellowGreenState(const std::string& tlsID) {
    //     return Helper::getTLS(tlsID).getActive()->getCurrentPhaseDef().getState();
    // }
    #[allow(unused_variables)]
    pub fn get_red_yellow_green_state(tlsID: &str) -> String {
        Helper::get_tls(tlsID).get_active().get_current_phase_def().get_state().to_string()
    }

    /// C++: `std::vector<libsumo::TraCILogic> TrafficLight::getAllProgramLogics(const std::string &)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_all_program_logics(tlsID: &str) -> Vec<TraCILogic> {
        let mut result: Vec<TraCILogic> = Vec::new();
        let logics = Helper::get_tls(tlsID).get_all_logics();
        for logic in logics {
            let mut l = TraCILogic::new_2(logic.get_program_id(), logic.get_logic_type() as i32, logic.get_current_phase_index(), &Vec::new());
            l.subParameter = logic.base2.get_parameters_map().clone();
            for phase in logic.get_phases().iter() {
                l.phases.push(Rc::new(TraCIPhase::new_2(steps2time(phase.duration), phase.get_state(), steps2time(phase.minDuration), steps2time(phase.maxDuration), phase.get_next_phases(), phase.get_name(), "")));
            }
            result.push(l);
        }
        result
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
        let mut junctionIDs: BTreeSet<String> = BTreeSet::new();
        let active = Helper::get_tls(tlsID).get_active();
        let links = active.get_links();
        for llinks in links.iter() {
            for l in llinks.iter() {
                junctionIDs.insert(l.get_junction().get_id().to_string());
            }
        }
        junctionIDs.into_iter().collect()
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
        let mut laneIDs: Vec<String> = Vec::new();
        let active = Helper::get_tls(tlsID).get_active();
        for llanes in active.get_lane_vectors().iter() {
            for l in llanes.iter() {
                laneIDs.push(l.get_id().to_string());
            }
        }
        laneIDs
    }

    /// C++: `std::vector<std::vector<libsumo::TraCILink>> TrafficLight::getControlledLinks(const std::string &)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_controlled_links(tlsID: &str) -> Vec<Vec<TraCILink>> {
        let mut result: Vec<Vec<TraCILink>> = Vec::new();
        let active = Helper::get_tls(tlsID).get_active();
        let lanes = active.get_lane_vectors();
        let links = active.get_links();
        for i in 0..lanes.len() {
            let mut subList: Vec<TraCILink> = Vec::new();
            let llanes = &lanes[i];
            let llinks = &links[i];
            // number of links controlled by this signal (signal i)
            for j in 0..llanes.len() {
                let link = llinks[j];
                // approached non-internal lane (if any)
                let to = if !link.get_lane().is_null() { link.get_lane().get_id().to_string() } else { String::new() };
                // approached "via", internal lane (if any)
                let via = if !link.get_via_lane().is_null() { link.get_via_lane().get_id().to_string() } else { String::new() };
                subList.push(TraCILink::new_2(llanes[j].get_id(), &via, &to));
            }
            result.push(subList);
        }
        result
    }

    /// C++: `std::string TrafficLight::getProgram(const std::string &)` [static]
    // std::string
    // TrafficLight::getProgram(const std::string& tlsID) {
    //     return Helper::getTLS(tlsID).getActive()->getProgramID();
    // }
    #[allow(unused_variables)]
    pub fn get_program(tlsID: &str) -> String {
        Helper::get_tls(tlsID).get_active().get_program_id().to_string()
    }

    /// C++: `int TrafficLight::getPhase(const std::string &)` [static]
    // int
    // TrafficLight::getPhase(const std::string& tlsID) {
    //     return Helper::getTLS(tlsID).getActive()->getCurrentPhaseIndex();
    // }
    #[allow(unused_variables)]
    pub fn get_phase(tlsID: &str) -> i32 {
        Helper::get_tls(tlsID).get_active().get_current_phase_index()
    }

    /// C++: `std::string TrafficLight::getPhaseName(const std::string &)` [static]
    // std::string
    // TrafficLight::getPhaseName(const std::string& tlsID) {
    //     return Helper::getTLS(tlsID).getActive()->getCurrentPhaseDef().getName();
    // }
    #[allow(unused_variables)]
    pub fn get_phase_name(tlsID: &str) -> String {
        Helper::get_tls(tlsID).get_active().get_current_phase_def().get_name().to_string()
    }

    /// C++: `double TrafficLight::getPhaseDuration(const std::string &)` [static]
    // double
    // TrafficLight::getPhaseDuration(const std::string& tlsID) {
    //     return STEPS2TIME(Helper::getTLS(tlsID).getActive()->getCurrentPhaseDef().duration);
    // }
    #[allow(unused_variables)]
    pub fn get_phase_duration(tlsID: &str) -> f64 {
        steps2time(Helper::get_tls(tlsID).get_active().get_current_phase_def().duration)
    }

    /// C++: `double TrafficLight::getNextSwitch(const std::string &)` [static]
    // double
    // TrafficLight::getNextSwitch(const std::string& tlsID) {
    //     return STEPS2TIME(Helper::getTLS(tlsID).getActive()->getNextSwitchTime());
    // }
    #[allow(unused_variables)]
    pub fn get_next_switch(tlsID: &str) -> f64 {
        steps2time(Helper::get_tls(tlsID).get_active().get_next_switch_time())
    }

    /// C++: `double TrafficLight::getSpentDuration(const std::string &)` [static]
    // double
    // TrafficLight::getSpentDuration(const std::string& tlsID) {
    //     return STEPS2TIME(Helper::getTLS(tlsID).getActive()->getSpentDuration());
    // }
    #[allow(unused_variables)]
    pub fn get_spent_duration(tlsID: &str) -> f64 {
        steps2time(Helper::get_tls(tlsID).get_active().get_spent_duration(-1))
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
        let active = Helper::get_tls(tlsID).get_active();
        if index < 0 || active.get_phase_number() <= index {
            panic!("{}", format!("The phase index {} is not in the allowed range [0,{}].", index, active.get_phase_number() - 1));
        }
        // find all crossings which have a green light in that phase
        let mut result: i32 = 0;

        let state: String = active.get_phases()[index as usize].get_state().to_string();
        for i in 0..state.len() {
            if state.as_bytes()[i] as i32 == LINKSTATE_TL_GREEN_MAJOR {
                for link in active.get_links_at(i as i32).iter() {
                    let link = *link;
                    if link.get_lane().is_crossing() {
                        // walking forwards across
                        let laneBefore = link.get_lane_before();
                        for person in laneBefore.get_edge().get_persons().iter() {
                            if person.cast::<MSPerson>().get_next_edge() == link.get_lane().get_edge().get_id() {
                                result += 1;
                            }
                        }
                        // walking backwards across
                        let lane = link.get_lane();
                        let walkingAreaAcross = lane.get_link_cont().first().unwrap().get_lane();
                        for person in walkingAreaAcross.get_edge().get_persons().iter() {
                            if person.cast::<MSPerson>().get_next_edge() == link.get_lane().get_edge().get_id() {
                                result += 1;
                            }
                        }
                    } else if link.get_lane_before().is_crossing() {
                        // walking backwards across (in case both sides are separately controlled)
                        let lane = link.get_lane();
                        for person in lane.get_edge().get_persons().iter() {
                            if person.cast::<MSPerson>().get_next_edge() == link.get_lane_before().get_edge().get_id() {
                                result += 1;
                            }
                        }
                    }
                }
            }
        }
        result
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
        let mut result: Vec<String> = Vec::new();
        // for railsignals we cannot use the "online" program
        let active = Helper::get_tls(tlsID).get_default();
        if linkIndex < 0 || linkIndex >= active.get_num_links() {
            panic!("{}", format!("The link index {} is not in the allowed range [0,{}].", linkIndex, active.get_num_links() - 1));
        }
        for veh in active.m().get_blocking_vehicles(linkIndex).iter() {
            result.push(veh.get_id().to_string());
        }
        result
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
        let mut result: Vec<String> = Vec::new();
        let active = Helper::get_tls(tlsID).get_default();
        if linkIndex < 0 || linkIndex >= active.get_num_links() {
            panic!("{}", format!("The link index {} is not in the allowed range [0,{}].", linkIndex, active.get_num_links() - 1));
        }
        for veh in active.m().get_rival_vehicles(linkIndex).iter() {
            result.push(veh.get_id().to_string());
        }
        result
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
        let mut result: Vec<String> = Vec::new();
        let active = Helper::get_tls(tlsID).get_default();
        if linkIndex < 0 || linkIndex >= active.get_num_links() {
            panic!("{}", format!("The link index {} is not in the allowed range [0,{}].", linkIndex, active.get_num_links() - 1));
        }
        for veh in active.m().get_priority_vehicles(linkIndex).iter() {
            result.push(veh.get_id().to_string());
        }
        result
    }

    /// C++: `std::vector<libsumo::TraCISignalConstraint> TrafficLight::getConstraints(const std::string &, const std::string &)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_constraints(tlsID: &str, tripId: &str) -> Vec<TraCISignalConstraint> {
        // shortcut: needs dynamic_cast to MSRailSignal and MSRailSignalConstraint_Predecessor, which needs #[repr(C)] on those structs in microsim/traffic_lights, upgrade when they are added
        panic!("shortcut: TrafficLight::getConstraints needs dynamic_cast to MSRailSignal / MSRailSignalConstraint_Predecessor")
    }

    /// C++: `std::vector<libsumo::TraCISignalConstraint> TrafficLight::getConstraintsByFoe(const std::string &, const std::string &)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_constraints_by_foe(foeSignal: &str, foeId: &str) -> Vec<TraCISignalConstraint> {
        // shortcut: needs dynamic_cast to MSRailSignal / MSRailSignalConstraint_Predecessor, which needs #[repr(C)] on those structs in microsim/traffic_lights, upgrade when they are added
        panic!("shortcut: TrafficLight::getConstraintsByFoe needs dynamic_cast to MSRailSignal / MSRailSignalConstraint_Predecessor")
    }

    /// C++: `std::vector<std::string> TrafficLight::getIDList()` [static]
    // std::vector<std::string>
    // TrafficLight::getIDList() {
    //     return MSNet::getInstance()->getTLSControl().getAllTLIds();
    // }
    #[allow(unused_variables)]
    pub fn get_id_list() -> Vec<String> {
        MSNet::get_instance().m().get_tls_control().get_all_tl_ids()
    }

    /// C++: `int TrafficLight::getIDCount()` [static]
    // int
    // TrafficLight::getIDCount() {
    //     return (int)getIDList().size();
    // }
    #[allow(unused_variables)]
    pub fn get_id_count() -> i32 {
        Self::get_id_list().len() as i32
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
        let tll = Helper::get_tls(objectID).get_active();
        if starts_with(key, "NEMA.") && tll.get_logic_type() != TrafficLightType::NEMA {
            panic!("{}", format!("'{}' is not a NEMA controller", objectID));
        }
        tll.base2.get_parameter(key, "")
    }

    /// C++: `const std::pair<std::string, std::string> TrafficLight::getParameterWithKey(const std::string &, const std::string &)` [static]
    // LIBSUMO_GET_PARAMETER_WITH_KEY_IMPLEMENTATION(TrafficLight)
    #[allow(unused_variables)]
    pub fn get_parameter_with_key(objectID: &str, key: &str) -> (String, String) {
        (key.to_string(), Self::get_parameter(objectID, key))
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
        let tll = Helper::get_tls(objectID).get_active();
        if starts_with(key, "NEMA.") && tll.get_logic_type() != TrafficLightType::NEMA {
            panic!("{}", format!("'{}' is not a NEMA controller", objectID));
        }
        tll.m().base2.set_parameter(key, value);
    }

    /// C++: `void TrafficLight::subscribe(const std::string &, const std::vector<int> &, double, double, const libsumo::TraCIResults &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(TrafficLight, TL)
    #[allow(unused_variables)]
    pub fn subscribe(objectID: &str, varIDs: &Vec<i32>, begin: f64, end: f64, parameters: &TraCIResults) {
        Helper::subscribe(CMD_SUBSCRIBE_TL_VARIABLE, objectID, varIDs, begin, end, parameters, 0, 0.0);
    }

    /// C++: `void TrafficLight::unsubscribe(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(TrafficLight, TL)
    #[allow(unused_variables)]
    pub fn unsubscribe(objectID: &str) {
        Helper::subscribe(CMD_SUBSCRIBE_TL_VARIABLE, objectID, &Vec::new(), INVALID_DOUBLE_VALUE, INVALID_DOUBLE_VALUE, &TraCIResults::new(), 0, 0.0);
    }

    /// C++: `void TrafficLight::subscribeContext(const std::string &, int, double, const std::vector<int> &, double, double, const libsumo::TraCIResults &)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn subscribe_context(objectID: &str, domain: i32, dist: f64, varIDs: &Vec<i32>, begin: f64, end: f64, parameters: &TraCIResults) {
        Helper::subscribe(CMD_SUBSCRIBE_TL_CONTEXT, objectID, varIDs, begin, end, parameters, domain, dist);
    }

    /// C++: `void TrafficLight::unsubscribeContext(const std::string &, int, double)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(TrafficLight, TL)
    #[allow(unused_variables)]
    pub fn unsubscribe_context(objectID: &str, domain: i32, dist: f64) {
        Helper::subscribe(CMD_SUBSCRIBE_TL_CONTEXT, objectID, &Vec::new(), INVALID_DOUBLE_VALUE, INVALID_DOUBLE_VALUE, &TraCIResults::new(), domain, dist);
    }

    /// C++: `const libsumo::SubscriptionResults TrafficLight::getAllSubscriptionResults()` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(TrafficLight, TL)
    #[allow(unused_variables)]
    pub fn get_all_subscription_results() -> SubscriptionResults {
        MY_SUBSCRIPTION_RESULTS.with(|r| r.borrow().clone())
    }

    /// C++: `const libsumo::TraCIResults TrafficLight::getSubscriptionResults(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(TrafficLight, TL)
    #[allow(unused_variables)]
    pub fn get_subscription_results(objectID: &str) -> TraCIResults {
        MY_SUBSCRIPTION_RESULTS.with(|r| r.borrow_mut().entry(objectID.to_string()).or_default().clone())
    }

    /// C++: `const libsumo::ContextSubscriptionResults TrafficLight::getAllContextSubscriptionResults()` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(TrafficLight, TL)
    #[allow(unused_variables)]
    pub fn get_all_context_subscription_results() -> ContextSubscriptionResults {
        MY_CONTEXT_SUBSCRIPTION_RESULTS.with(|r| r.borrow().clone())
    }

    /// C++: `const libsumo::SubscriptionResults TrafficLight::getContextSubscriptionResults(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(TrafficLight, TL)
    #[allow(unused_variables)]
    pub fn get_context_subscription_results(objectID: &str) -> SubscriptionResults {
        MY_CONTEXT_SUBSCRIPTION_RESULTS.with(|r| r.borrow_mut().entry(objectID.to_string()).or_default().clone())
    }

    /// C++: `void TrafficLight::subscribeParameterWithKey(const std::string &, const std::string &, double, double)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(TrafficLight, TL)
    #[allow(unused_variables)]
    pub fn subscribe_parameter_with_key(objectID: &str, key: &str, beginTime: f64, endTime: f64) {
        let mut params: TraCIResults = TraCIResults::new();
        params.insert(VAR_PARAMETER_WITH_KEY, crate::libsumo::helper::as_traci_result(TraCIString { base: TraCIResult { result_type: TYPE_STRING }, value: key.to_string() }));
        Helper::subscribe(CMD_SUBSCRIBE_TL_VARIABLE, objectID, &vec![VAR_PARAMETER_WITH_KEY], beginTime, endTime, &params, 0, 0.0);
    }

    /// C++: `int TrafficLight::domainID()` [static]
    // LIBSUMO_SUBSCRIPTION_API
    #[allow(unused_variables)]
    pub fn domain_id() -> i32 {
        CMD_GET_TL_VARIABLE
    }

    /// C++: `void TrafficLight::setRedYellowGreenState(const std::string &, const std::string &)` [static]
    // void
    // TrafficLight::setRedYellowGreenState(const std::string& tlsID, const std::string& state) {
    //     Helper::getTLS(tlsID).setStateInstantiatingOnline(MSNet::getInstance()->getTLSControl(), state);
    // }
    #[allow(unused_variables)]
    pub fn set_red_yellow_green_state(tlsID: &str, state: &str) {
        Helper::get_tls(tlsID).set_state_instantiating_online(MSNet::get_instance().m().get_tls_control(), state);
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
        let active = Helper::get_tls(tlsID).get_active();
        if index < 0 || active.get_phase_number() <= index {
            panic!("{}", format!("The phase index {} is not in the allowed range [0,{}].", index, active.get_phase_number() - 1));
        }
        let cTime = MSNet::get_instance().get_current_time_step();
        let duration = active.get_phase(index).duration;
        active.m().change_step_and_duration(MSNet::get_instance().m().get_tls_control(), cTime, index, duration);
    }

    /// C++: `void TrafficLight::setPhaseName(const std::string &, const std::string &)` [static]
    // void
    // TrafficLight::setPhaseName(const std::string& tlsID, const std::string& name) {
    //     MSTrafficLightLogic* const active = Helper::getTLS(tlsID).getActive();
    //     const_cast<MSPhaseDefinition&>(active->getCurrentPhaseDef()).setName(name);
    // }
    #[allow(unused_variables)]
    pub fn set_phase_name(tlsID: &str, name: &str) {
        let active = Helper::get_tls(tlsID).get_active();
        // const_cast of the current phase: the phases are Ptr-owned, so mutate through the Ptr
        let step = active.get_current_phase_index() as usize;
        active.get_phases()[step].m().set_name(name);
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
        // a ProcessError from switchTo already carries the message the C++ catch rethrows as TraCIException
        Helper::get_tls(tlsID).switch_to(MSNet::get_instance().m().get_tls_control(), programID);
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
        let active = Helper::get_tls(tlsID).get_active();
        let cTime = MSNet::get_instance().get_current_time_step();
        active.m().change_step_and_duration(MSNet::get_instance().m().get_tls_control(), cTime, -1, time2steps(phaseDuration));
    }

    /// C++: `void TrafficLight::setProgramLogic(const std::string &, const libsumo::TraCILogic &)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn set_program_logic(tlsID: &str, logic: &TraCILogic) {
        // make sure index and phaseNo are consistent
        if logic.currentPhaseIndex >= logic.phases.len() as i32 {
            panic!("{}", "set program: parameter index must be less than parameter phase number.");
        }
        // shortcut: creating the MSActuatedTrafficLightLogic / NEMALogic / MSDelayBasedTrafficLightLogic / MSSimpleTrafficLightLogic instances needs the Parameterised::Map argument (Unmapped in ms_traffic_light_logic.rs) and the else branch needs a static_cast to MSSimpleTrafficLightLogic, upgrade when both are typed
        panic!("shortcut: TrafficLight::setProgramLogic needs typed Parameterised::Map logic constructors and MSSimpleTrafficLightLogic cast")
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
        // shortcut: needs dynamic_cast of the default logic to MSRailSignal and of the foe logic, which needs #[repr(C)] on MSRailSignal, upgrade when added
        panic!("shortcut: TrafficLight::addConstraint needs dynamic_cast to MSRailSignal")
    }

    /// C++: `std::vector<libsumo::TraCISignalConstraint> TrafficLight::swapConstraints(const std::string &, const std::string &, const std::string &, const std::string &)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn swap_constraints(tlsID: &str, tripId: &str, foeSignal: &str, foeId: &str) -> Vec<TraCISignalConstraint> {
        // shortcut: needs dynamic_cast to MSRailSignal / MSRailSignalConstraint_Predecessor and findConstraintsDeadLocks, upgrade when the rail signal structs are repr(C)
        panic!("shortcut: TrafficLight::swapConstraints needs dynamic_cast to MSRailSignal / MSRailSignalConstraint_Predecessor")
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
        // shortcut: needs dynamic_cast to MSRailSignal and MSRailSignalConstraint_Predecessor, upgrade when the rail signal structs are repr(C)
        panic!("shortcut: TrafficLight::removeConstraints needs dynamic_cast to MSRailSignal / MSRailSignalConstraint_Predecessor")
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
        // shortcut: needs dynamic_cast to MSRailSignal / MSRailSignalConstraint_Predecessor and the vehicle route/stop iteration, upgrade when those are repr(C)
        panic!("shortcut: TrafficLight::updateConstraints needs dynamic_cast to MSRailSignal / MSRailSignalConstraint_Predecessor")
    }

    /// C++: `std::vector<libsumo::TraCILogic> TrafficLight::getCompleteRedYellowGreenDefinition(const std::string &)` [static]
    // static inline std::vector<libsumo::TraCILogic> getCompleteRedYellowGreenDefinition(const std::string& tlsID) {
    //         return getAllProgramLogics(tlsID);
    //     }
    #[allow(unused_variables)]
    pub fn get_complete_red_yellow_green_definition(tlsID: &str) -> Vec<TraCILogic> {
        Self::get_all_program_logics(tlsID)
    }

    /// C++: `void TrafficLight::setCompleteRedYellowGreenDefinition(const std::string &, const libsumo::TraCILogic &)` [static]
    // static inline void setCompleteRedYellowGreenDefinition(const std::string& tlsID, const libsumo::TraCILogic& logic) {
    //         setProgramLogic(tlsID, logic);
    //     }
    #[allow(unused_variables)]
    pub fn set_complete_red_yellow_green_definition(tlsID: &str, logic: &TraCILogic) {
        Self::set_program_logic(tlsID, logic);
    }

    /// C++: `void TrafficLight::setNemaSplits(const std::string &, const std::vector<double> &)` [static]
    // void
    // TrafficLight::setNemaSplits(const std::string& tlsID, const std::vector<double>& splits) {
    //     setParameter(tlsID, "NEMA.splits", toString(splits));
    // }
    #[allow(unused_variables)]
    pub fn set_nema_splits(tlsID: &str, splits: &Vec<f64>) {
        Self::set_parameter(tlsID, "NEMA.splits", &utils::common::to_string::to_string(splits));
    }

    /// C++: `void TrafficLight::setNemaMaxGreens(const std::string &, const std::vector<double> &)` [static]
    // void
    // TrafficLight::setNemaMaxGreens(const std::string& tlsID, const std::vector<double>& maxGreens) {
    //     setParameter(tlsID, "NEMA.maxGreens", toString(maxGreens));
    // }
    #[allow(unused_variables)]
    pub fn set_nema_max_greens(tlsID: &str, maxGreens: &Vec<f64>) {
        Self::set_parameter(tlsID, "NEMA.maxGreens", &utils::common::to_string::to_string(maxGreens));
    }

    /// C++: `void TrafficLight::setNemaCycleLength(const std::string &, double)` [static]
    // void
    // TrafficLight::setNemaCycleLength(const std::string& tlsID, double cycleLength) {
    //     setParameter(tlsID, "NEMA.cycleLength", toString(cycleLength));
    // }
    #[allow(unused_variables)]
    pub fn set_nema_cycle_length(tlsID: &str, cycleLength: f64) {
        Self::set_parameter(tlsID, "NEMA.cycleLength", &utils::common::to_string::to_string(&cycleLength));
    }

    /// C++: `void TrafficLight::setNemaOffset(const std::string &, double)` [static]
    // void
    // TrafficLight::setNemaOffset(const std::string& tlsID, double offset) {
    //     setParameter(tlsID, "NEMA.offset", toString(offset));
    // }
    #[allow(unused_variables)]
    pub fn set_nema_offset(tlsID: &str, offset: f64) {
        Self::set_parameter(tlsID, "NEMA.offset", &utils::common::to_string::to_string(&offset));
    }

    /// C++: `std::shared_ptr<VariableWrapper> TrafficLight::makeWrapper()` [static]
    // std::shared_ptr<VariableWrapper>
    // TrafficLight::makeWrapper() {
    //     return std::make_shared<Helper::SubscriptionWrapper>(handleVariable, mySubscriptionResults, myContextSubscriptionResults);
    // }
    #[allow(unused_variables)]
    pub fn make_wrapper() -> Rc<VariableWrapper> {
        Helper::make_subscription_wrapper(Some(Self::handle_variable), unsafe { &mut *MY_SUBSCRIPTION_RESULTS.with(|r| r.as_ptr()) }, unsafe { &mut *MY_CONTEXT_SUBSCRIPTION_RESULTS.with(|r| r.as_ptr()) })
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
        match variable {
            TRACI_ID_LIST => wrapper.m().wrap_string_list(objID, variable, &Self::get_id_list()),
            ID_COUNT => wrapper.m().wrap_int(objID, variable, Self::get_id_count()),
            TL_RED_YELLOW_GREEN_STATE => wrapper.m().wrap_string(objID, variable, &Self::get_red_yellow_green_state(objID)),
            TL_COMPLETE_DEFINITION_RYG => wrapper.m().wrap_logic_vector(objID, variable, &Self::get_all_program_logics(objID)),
            TL_CONTROLLED_LANES => wrapper.m().wrap_string_list(objID, variable, &Self::get_controlled_lanes(objID)),
            TL_CONTROLLED_LINKS => wrapper.m().wrap_link_vector_vector(objID, variable, &Self::get_controlled_links(objID)),
            TL_CURRENT_PHASE => wrapper.m().wrap_int(objID, variable, Self::get_phase(objID)),
            VAR_NAME => wrapper.m().wrap_string(objID, variable, &Self::get_phase_name(objID)),
            TL_CURRENT_PROGRAM => wrapper.m().wrap_string(objID, variable, &Self::get_program(objID)),
            TL_PHASE_DURATION => wrapper.m().wrap_double(objID, variable, Self::get_phase_duration(objID)),
            TL_NEXT_SWITCH => wrapper.m().wrap_double(objID, variable, Self::get_next_switch(objID)),
            TL_SPENT_DURATION => wrapper.m().wrap_double(objID, variable, Self::get_spent_duration(objID)),
            VAR_PERSON_NUMBER => {
                let n = StoHelp::read_typed_int(paramData.m(), "");
                wrapper.m().wrap_int(objID, variable, Self::get_served_person_count(objID, n))
            }
            TL_BLOCKING_VEHICLES => {
                let n = StoHelp::read_typed_int(paramData.m(), "");
                wrapper.m().wrap_string_list(objID, variable, &Self::get_blocking_vehicles(objID, n))
            }
            TL_RIVAL_VEHICLES => {
                let n = StoHelp::read_typed_int(paramData.m(), "");
                wrapper.m().wrap_string_list(objID, variable, &Self::get_rival_vehicles(objID, n))
            }
            TL_PRIORITY_VEHICLES => {
                let n = StoHelp::read_typed_int(paramData.m(), "");
                wrapper.m().wrap_string_list(objID, variable, &Self::get_priority_vehicles(objID, n))
            }
            TL_CONSTRAINT => {
                let s = StoHelp::read_typed_string(paramData.m(), "");
                wrapper.m().wrap_signal_constraint_vector(objID, variable, &Self::get_constraints(objID, &s))
            }
            TL_CONSTRAINT_BYFOE => {
                let s = StoHelp::read_typed_string(paramData.m(), "");
                wrapper.m().wrap_signal_constraint_vector(objID, variable, &Self::get_constraints_by_foe(objID, &s))
            }
            TL_CONTROLLED_JUNCTIONS => wrapper.m().wrap_string_list(objID, variable, &Self::get_controlled_junctions(objID)),
            VAR_PARAMETER => {
                let key = StoHelp::read_typed_string(paramData.m(), "");
                wrapper.m().wrap_string(objID, variable, &Self::get_parameter(objID, &key))
            }
            VAR_PARAMETER_WITH_KEY => {
                let key = StoHelp::read_typed_string(paramData.m(), "");
                wrapper.m().wrap_string_pair(objID, variable, &Self::get_parameter_with_key(objID, &key))
            }
            _ => false,
        }
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
        // shortcut: needs dynamic_cast to MSRailSignalConstraint_Predecessor, which needs #[repr(C)] on it in microsim/traffic_lights, upgrade when added
        panic!("shortcut: TrafficLight::buildConstraint needs dynamic_cast to MSRailSignalConstraint_Predecessor")
    }

    /// C++: `std::vector<libsumo::TraCISignalConstraint> TrafficLight::findConstraintsDeadLocks(const std::string &, const std::string &, const std::string &, const std::string &)` [static, private]
    /// perform swapConstraints to resolve deadlocks and return the new constraints
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn find_constraints_dead_locks(foeId: &str, tripId: &str, foeSignal: &str, tlsID: &str) -> Vec<TraCISignalConstraint> {
        // shortcut: needs dynamic_cast to MSRailSignal / MSRailSignalConstraint_Predecessor (see buildConstraint), upgrade when the rail signal structs are repr(C)
        panic!("shortcut: TrafficLight::findConstraintsDeadLocks needs dynamic_cast to MSRailSignal / MSRailSignalConstraint_Predecessor")
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
        let net = MSNet::get_instance();
        let c = net.m().get_vehicle_control();
        for (_, veh) in c.myVehicleDict.iter() {
            if veh.get_parameter().base.get_parameter("tripId", &veh.get_id().to_string()) == tripOrVehID {
                return *veh;
            }
        }
        Ptr::null()
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
        let mut result: Vec<String> = Vec::new();
        // shortcut: dynamic_cast<MSBaseVehicle*> is not checked at runtime, the vehicle is assumed to be a base vehicle, upgrade with a typed vehicle check
        let sumoVeh = MSNet::get_instance().m().get_vehicle_control().get_vehicle(&vehID);
        if !sumoVeh.is_null() {
            let veh = sumoVeh.cast::<MSBaseVehicle>();
            let tripId = veh.get_parameter().base.get_parameter("tripId", "");
            if tripId != "" {
                result.push(tripId);
            }
            for stop in veh.get_stops().iter() {
                if stop.pars.tripId != "" {
                    result.push(stop.pars.tripId.clone());
                }
            }
        }
        result
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
        // swap parameters that were assigned by generateRailSignalConstraints.py
        for keys in Self::get_swap_params(c.get_type() as i32) {
            Self::swap_parameters_2(c, &keys.0, &keys.1);
        }
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
        let value1 = c.get_parameter(key1, "");
        let value2 = c.get_parameter(key2, "");
        if value1 != "" {
            c.m().base.set_parameter(key2, &value1);
        } else {
            c.m().base.unset_parameter(key2);
        }
        if value2 != "" {
            c.m().base.set_parameter(key1, &value2);
        } else {
            c.m().base.unset_parameter(key1);
        }
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
        // swap parameters that were assigned by generateRailSignalConstraints.py
        for keys in Self::get_swap_params(c.type_) {
            Self::swap_parameters_4(c, &keys.0, &keys.1);
        }
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
        let value1 = c.param.get(key1).cloned().unwrap_or_default();
        let value2 = c.param.get(key2).cloned().unwrap_or_default();
        if value1 != "" {
            c.param.insert(key2.to_string(), value1);
        } else {
            c.param.remove(key2);
        }
        if value2 != "" {
            c.param.insert(key1.to_string(), value2);
        } else {
            c.param.remove(key1);
        }
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
        let mut result: Vec<(String, String)> = vec![
            ("vehID".to_string(), "foeID".to_string()),
            ("line".to_string(), "foeLine".to_string()),
            ("arrival".to_string(), "foeArrival".to_string()),
        ];
        if constraintType == MSRailSignalConstraint_ConstraintType::BIDI_PREDECESSOR as i32 {
            let special = [("busStop", "busStop2"), ("priorStop", "priorStop2"), ("stopArrival", "foeStopArrival")];
            result.extend(special.iter().map(|(a, b)| (a.to_string(), b.to_string())));
        }
        result
    }

    /// C++: `void TrafficLight::TrafficLight()` [private]
    /// invalidated standard constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new() -> Self {
        panic!("libsumo::TrafficLight cannot be constructed: the standard constructor is invalidated in C++")
    }

}

