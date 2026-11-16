//! Port of libsumo/RouteProbe.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
// shortcut: C++ throw TraCIException(msg) is ported as panic!("{}", msg), upgrade when TraCIException is ported
use crate::prelude::*;

/// C++ class `libsumo::RouteProbe` (libsumo/RouteProbe.h)
pub struct RouteProbe {
}

// C++ static members of RouteProbe (port as module-level statics, see AGENTS.md):
//   static const int RouteProbe::DOMAIN_ID;
//   static SubscriptionResults RouteProbe::mySubscriptionResults;
//   static ContextSubscriptionResults RouteProbe::myContextSubscriptionResults;
thread_local! {
    /// C++ static `RouteProbe::mySubscriptionResults`
    static MY_SUBSCRIPTION_RESULTS: std::cell::RefCell<SubscriptionResults> = std::cell::RefCell::new(SubscriptionResults::new());
    /// C++ static `RouteProbe::myContextSubscriptionResults`
    static MY_CONTEXT_SUBSCRIPTION_RESULTS: std::cell::RefCell<ContextSubscriptionResults> = std::cell::RefCell::new(ContextSubscriptionResults::new());
}


impl RouteProbe {
    /// C++: `std::string RouteProbe::getEdgeID(const std::string &)` [static]
    // std::string
    // RouteProbe::getEdgeID(const std::string& probeID) {
    //     MSRouteProbe* rp = getRouteProbe(probeID);
    //     return rp->getEdge()->getID();
    // }
    #[allow(unused_variables)]
    pub fn get_edge_id(probeID: &str) -> String {
        Self::get_route_probe(probeID).m().get_edge().get_id().to_string()
    }

    /// C++: `std::string RouteProbe::sampleLastRouteID(const std::string &)` [static]
    // std::string
    // RouteProbe::sampleLastRouteID(const std::string& probeID) {
    //     MSRouteProbe* rp = getRouteProbe(probeID);
    //     ConstMSRoutePtr route = rp->sampleRoute(true);
    //     if (route == nullptr) {
    //         throw TraCIException("RouteProbe '" + probeID + "' did not collect any routes yet");
    //     }
    //     return route->getID();
    // }
    #[allow(unused_variables)]
    pub fn sample_last_route_id(probeID: &str) -> String {
        let rp = Self::get_route_probe(probeID);
        let Some(route) = rp.sample_route(true) else {
            panic!("RouteProbe '{}' did not collect any routes yet", probeID);
        };
        route.base.get_id().to_string()
    }

    /// C++: `std::string RouteProbe::sampleCurrentRouteID(const std::string &)` [static]
    // std::string
    // RouteProbe::sampleCurrentRouteID(const std::string& probeID) {
    //     MSRouteProbe* rp = getRouteProbe(probeID);
    //     ConstMSRoutePtr route = rp->sampleRoute(false);
    //     if (route == nullptr) {
    //         throw TraCIException("RouteProbe '" + probeID + "' did not collect any routes yet");
    //     }
    //     return route->getID();
    // }
    #[allow(unused_variables)]
    pub fn sample_current_route_id(probeID: &str) -> String {
        let rp = Self::get_route_probe(probeID);
        let Some(route) = rp.sample_route(false) else {
            panic!("RouteProbe '{}' did not collect any routes yet", probeID);
        };
        route.base.get_id().to_string()
    }

    /// C++: `std::vector<std::string> RouteProbe::getIDList()` [static]
    // std::vector<std::string>
    // RouteProbe::getIDList() {
    //     std::vector<std::string> ids;
    //     MSNet::getInstance()->getDetectorControl().getTypedDetectors(SUMO_TAG_ROUTEPROBE).insertIDs(ids);
    //     return ids;
    // }
    #[allow(unused_variables)]
    pub fn get_id_list() -> Vec<String> {
        let mut ids: Vec<String> = Vec::new();
        for id in MSNet::get_instance().m().get_detector_control().get_typed_detectors(SUMO_TAG_ROUTEPROBE).keys() {
            ids.push(id.clone());
        }
        ids
    }

    /// C++: `int RouteProbe::getIDCount()` [static]
    // int
    // RouteProbe::getIDCount() {
    //     return (int)getIDList().size();
    // }
    #[allow(unused_variables)]
    pub fn get_id_count() -> i32 {
        Self::get_id_list().len() as i32
    }

    /// C++: `std::string RouteProbe::getParameter(const std::string &, const std::string &)` [static]
    // std::string
    // RouteProbe::getParameter(const std::string& /* probeID */, const std::string& /* param */) {
    //     return "";
    // }
    #[allow(unused_variables)]
    pub fn get_parameter(objectID: &str, key: &str) -> String {
        String::new()
    }

    /// C++: `const std::pair<std::string, std::string> RouteProbe::getParameterWithKey(const std::string &, const std::string &)` [static]
    // LIBSUMO_GET_PARAMETER_WITH_KEY_IMPLEMENTATION(RouteProbe)
    #[allow(unused_variables)]
    pub fn get_parameter_with_key(objectID: &str, key: &str) -> (String, String) {
        (key.to_string(), Self::get_parameter(objectID, key))
    }

    /// C++: `void RouteProbe::setParameter(const std::string &, const std::string &, const std::string &)` [static]
    // void
    // RouteProbe::setParameter(const std::string& /* probeID */, const std::string& /* key */, const std::string& /* value */) {
    //     //MSRouteProbe* rp = getRouteProbe(probeID);
    //     //r->setParameter(key, value);
    // }
    #[allow(unused_variables)]
    pub fn set_parameter(objectID: &str, key: &str, value: &str) {
        // the C++ body is commented out upstream: route probes have no settable parameters
    }

    /// C++: `void RouteProbe::subscribe(const std::string &, const std::vector<int> &, double, double, const libsumo::TraCIResults &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(RouteProbe, ROUTEPROBE)
    #[allow(unused_variables)]
    pub fn subscribe(objectID: &str, varIDs: &Vec<i32>, begin: f64, end: f64, parameters: &TraCIResults) {
        Helper::subscribe(CMD_SUBSCRIBE_ROUTEPROBE_VARIABLE, objectID, varIDs, begin, end, parameters, 0, 0.0);
    }

    /// C++: `void RouteProbe::unsubscribe(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(RouteProbe, ROUTEPROBE)
    #[allow(unused_variables)]
    pub fn unsubscribe(objectID: &str) {
        Helper::subscribe(CMD_SUBSCRIBE_ROUTEPROBE_VARIABLE, objectID, &Vec::new(), INVALID_DOUBLE_VALUE, INVALID_DOUBLE_VALUE, &TraCIResults::new(), 0, 0.0);
    }

    /// C++: `void RouteProbe::subscribeContext(const std::string &, int, double, const std::vector<int> &, double, double, const libsumo::TraCIResults &)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn subscribe_context(objectID: &str, domain: i32, dist: f64, varIDs: &Vec<i32>, begin: f64, end: f64, parameters: &TraCIResults) {
        Helper::subscribe(CMD_SUBSCRIBE_ROUTEPROBE_CONTEXT, objectID, varIDs, begin, end, parameters, domain, dist);
    }

    /// C++: `void RouteProbe::unsubscribeContext(const std::string &, int, double)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(RouteProbe, ROUTEPROBE)
    #[allow(unused_variables)]
    pub fn unsubscribe_context(objectID: &str, domain: i32, dist: f64) {
        Helper::subscribe(CMD_SUBSCRIBE_ROUTEPROBE_CONTEXT, objectID, &Vec::new(), INVALID_DOUBLE_VALUE, INVALID_DOUBLE_VALUE, &TraCIResults::new(), domain, dist);
    }

    /// C++: `const libsumo::SubscriptionResults RouteProbe::getAllSubscriptionResults()` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(RouteProbe, ROUTEPROBE)
    #[allow(unused_variables)]
    pub fn get_all_subscription_results() -> SubscriptionResults {
        MY_SUBSCRIPTION_RESULTS.with(|r| r.borrow().clone())
    }

    /// C++: `const libsumo::TraCIResults RouteProbe::getSubscriptionResults(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(RouteProbe, ROUTEPROBE)
    #[allow(unused_variables)]
    pub fn get_subscription_results(objectID: &str) -> TraCIResults {
        MY_SUBSCRIPTION_RESULTS.with(|r| r.borrow_mut().entry(objectID.to_string()).or_default().clone())
    }

    /// C++: `const libsumo::ContextSubscriptionResults RouteProbe::getAllContextSubscriptionResults()` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(RouteProbe, ROUTEPROBE)
    #[allow(unused_variables)]
    pub fn get_all_context_subscription_results() -> ContextSubscriptionResults {
        MY_CONTEXT_SUBSCRIPTION_RESULTS.with(|r| r.borrow().clone())
    }

    /// C++: `const libsumo::SubscriptionResults RouteProbe::getContextSubscriptionResults(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(RouteProbe, ROUTEPROBE)
    #[allow(unused_variables)]
    pub fn get_context_subscription_results(objectID: &str) -> SubscriptionResults {
        MY_CONTEXT_SUBSCRIPTION_RESULTS.with(|r| r.borrow_mut().entry(objectID.to_string()).or_default().clone())
    }

    /// C++: `void RouteProbe::subscribeParameterWithKey(const std::string &, const std::string &, double, double)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(RouteProbe, ROUTEPROBE)
    #[allow(unused_variables)]
    pub fn subscribe_parameter_with_key(objectID: &str, key: &str, beginTime: f64, endTime: f64) {
        let mut params: TraCIResults = TraCIResults::new();
        params.insert(VAR_PARAMETER_WITH_KEY, crate::libsumo::helper::as_traci_result(TraCIString { base: TraCIResult { result_type: TYPE_STRING }, value: key.to_string() }));
        Helper::subscribe(CMD_SUBSCRIBE_ROUTEPROBE_VARIABLE, objectID, &vec![VAR_PARAMETER_WITH_KEY], beginTime, endTime, &params, 0, 0.0);
    }

    /// C++: `int RouteProbe::domainID()` [static]
    // LIBSUMO_SUBSCRIPTION_API
    #[allow(unused_variables)]
    pub fn domain_id() -> i32 {
        CMD_GET_ROUTEPROBE_VARIABLE
    }

    /// C++: `std::shared_ptr<VariableWrapper> RouteProbe::makeWrapper()` [static]
    // std::shared_ptr<VariableWrapper>
    // RouteProbe::makeWrapper() {
    //     return std::make_shared<Helper::SubscriptionWrapper>(handleVariable, mySubscriptionResults, myContextSubscriptionResults);
    // }
    #[allow(unused_variables)]
    pub fn make_wrapper() -> Rc<VariableWrapper> {
        Helper::make_subscription_wrapper(Some(Self::handle_variable), unsafe { &mut *MY_SUBSCRIPTION_RESULTS.with(|r| r.as_ptr()) }, unsafe { &mut *MY_CONTEXT_SUBSCRIPTION_RESULTS.with(|r| r.as_ptr()) })
    }

    /// C++: `bool RouteProbe::handleVariable(const std::string &, const int, VariableWrapper *, tcpip::Storage *)` [static]
    // bool
    // RouteProbe::handleVariable(const std::string& objID, const int variable, VariableWrapper* wrapper, tcpip::Storage* paramData) {
    //     switch (variable) {
    //         case TRACI_ID_LIST:
    //             return wrapper->wrapStringList(objID, variable, getIDList());
    //         case ID_COUNT:
    //             return wrapper->wrapInt(objID, variable, getIDCount());
    //         case VAR_ROAD_ID:
    //             return wrapper->wrapString(objID, variable, getEdgeID(objID));
    //         case VAR_SAMPLE_LAST:
    //             return wrapper->wrapString(objID, variable, sampleLastRouteID(objID));
    //         case VAR_SAMPLE_CURRENT:
    //             return wrapper->wrapString(objID, variable, sampleCurrentRouteID(objID));
    //         case libsumo::VAR_PARAMETER:
    //             paramData->readUnsignedByte();
    //             return wrapper->wrapString(objID, variable, getParameter(objID, paramData->readString()));
    //         case libsumo::VAR_PARAMETER_WITH_KEY:
    //             paramData->readUnsignedByte();
    //             return wrapper->wrapStringPair(objID, variable, getParameterWithKey(objID, paramData->readString()));
    //         default:
    //             return false;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn handle_variable(objID: &str, variable: i32, wrapper: Ptr<VariableWrapper>, paramData: Ptr<Storage>) -> bool {
        match variable {
            TRACI_ID_LIST => wrapper.m().wrap_string_list(objID, variable, &Self::get_id_list()),
            ID_COUNT => wrapper.m().wrap_int(objID, variable, Self::get_id_count()),
            VAR_ROAD_ID => wrapper.m().wrap_string(objID, variable, &Self::get_edge_id(objID)),
            VAR_SAMPLE_LAST => wrapper.m().wrap_string(objID, variable, &Self::sample_last_route_id(objID)),
            VAR_SAMPLE_CURRENT => wrapper.m().wrap_string(objID, variable, &Self::sample_current_route_id(objID)),
            VAR_PARAMETER => {
                paramData.m().read_unsigned_byte();
                // shortcut: C++ readString() throws ProcessError on malformed storage, here it panics, upgrade when handle_variable returns Result
                let key = paramData.m().read_string().expect("shortcut: malformed TraCI parameter string");
                wrapper.m().wrap_string(objID, variable, &Self::get_parameter(objID, &key))
            }
            VAR_PARAMETER_WITH_KEY => {
                paramData.m().read_unsigned_byte();
                // shortcut: C++ readString() throws ProcessError on malformed storage, here it panics, upgrade when handle_variable returns Result
                let key = paramData.m().read_string().expect("shortcut: malformed TraCI parameter string");
                wrapper.m().wrap_string_pair(objID, variable, &Self::get_parameter_with_key(objID, &key))
            }
            _ => false,
        }
    }

    /// C++: `MSRouteProbe * RouteProbe::getRouteProbe(const std::string &)` [static, private]
    // MSRouteProbe*
    // RouteProbe::getRouteProbe(const std::string& id) {
    //     MSRouteProbe* rp = dynamic_cast<MSRouteProbe*>(MSNet::getInstance()->getDetectorControl().getTypedDetectors(SUMO_TAG_ROUTEPROBE).get(id));
    //     if (rp == nullptr) {
    //         throw TraCIException("Lane area detector '" + id + "' is not known");
    //     }
    //     return rp;
    // }
    #[allow(unused_variables)]
    pub fn get_route_probe(id: &str) -> Ptr<MSRouteProbe> {
        // shortcut: C++ dynamic_cast is replaced by a cast, valid because only ROUTEPROBE-typed detectors are looked up; MSRouteProbe needs #[repr(C)] (base first), upgrade when ms_route_probe.rs adds it
        let rp = MSNet::get_instance().m().get_detector_control().get_typed_detectors(SUMO_TAG_ROUTEPROBE).get(id).copied().unwrap_or(Ptr::null()).cast::<MSRouteProbe>();
        if rp.is_null() {
            panic!("{}", format!("Lane area detector '{}' is not known", id));
        }
        rp
    }

    /// C++: `void RouteProbe::RouteProbe()` [private]
    /// invalidated standard constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new() -> Self {
        panic!("libsumo::RouteProbe cannot be constructed: the standard constructor is invalidated in C++")
    }

}

