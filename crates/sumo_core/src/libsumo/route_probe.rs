//! Port of libsumo/RouteProbe.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `libsumo::RouteProbe` (libsumo/RouteProbe.h)
pub struct RouteProbe {
}

// C++ static members of RouteProbe (port as module-level statics, see AGENTS.md):
//   static const int RouteProbe::DOMAIN_ID;
//   static SubscriptionResults RouteProbe::mySubscriptionResults;
//   static ContextSubscriptionResults RouteProbe::myContextSubscriptionResults;

impl RouteProbe {
    /// C++: `std::string RouteProbe::getEdgeID(const std::string &)` [static]
    // std::string
    // RouteProbe::getEdgeID(const std::string& probeID) {
    //     MSRouteProbe* rp = getRouteProbe(probeID);
    //     return rp->getEdge()->getID();
    // }
    #[allow(unused_variables)]
    pub fn get_edge_id(probeID: &str) -> String {
        todo!("port RouteProbe::getEdgeID")
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
        todo!("port RouteProbe::sampleLastRouteID")
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
        todo!("port RouteProbe::sampleCurrentRouteID")
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
        todo!("port RouteProbe::getIDList")
    }

    /// C++: `int RouteProbe::getIDCount()` [static]
    // int
    // RouteProbe::getIDCount() {
    //     return (int)getIDList().size();
    // }
    #[allow(unused_variables)]
    pub fn get_id_count() -> i32 {
        todo!("port RouteProbe::getIDCount")
    }

    /// C++: `std::string RouteProbe::getParameter(const std::string &, const std::string &)` [static]
    // std::string
    // RouteProbe::getParameter(const std::string& /* probeID */, const std::string& /* param */) {
    //     return "";
    // }
    #[allow(unused_variables)]
    pub fn get_parameter(objectID: &str, key: &str) -> String {
        todo!("port RouteProbe::getParameter")
    }

    /// C++: `const std::pair<std::string, std::string> RouteProbe::getParameterWithKey(const std::string &, const std::string &)` [static]
    // LIBSUMO_GET_PARAMETER_WITH_KEY_IMPLEMENTATION(RouteProbe)
    #[allow(unused_variables)]
    pub fn get_parameter_with_key(objectID: &str, key: &str) -> (String, String) {
        todo!("port RouteProbe::getParameterWithKey")
    }

    /// C++: `void RouteProbe::setParameter(const std::string &, const std::string &, const std::string &)` [static]
    // void
    // RouteProbe::setParameter(const std::string& /* probeID */, const std::string& /* key */, const std::string& /* value */) {
    //     //MSRouteProbe* rp = getRouteProbe(probeID);
    //     //r->setParameter(key, value);
    // }
    #[allow(unused_variables)]
    pub fn set_parameter(objectID: &str, key: &str, value: &str) {
        todo!("port RouteProbe::setParameter")
    }

    /// C++: `void RouteProbe::subscribe(const std::string &, const std::vector<int> &, double, double, const libsumo::TraCIResults &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(RouteProbe, ROUTEPROBE)
    #[allow(unused_variables)]
    pub fn subscribe(objectID: &str, varIDs: &Vec<i32>, begin: f64, end: f64, parameters: &TraCIResults) {
        todo!("port RouteProbe::subscribe")
    }

    /// C++: `void RouteProbe::unsubscribe(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(RouteProbe, ROUTEPROBE)
    #[allow(unused_variables)]
    pub fn unsubscribe(objectID: &str) {
        todo!("port RouteProbe::unsubscribe")
    }

    /// C++: `void RouteProbe::subscribeContext(const std::string &, int, double, const std::vector<int> &, double, double, const libsumo::TraCIResults &)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn subscribe_context(objectID: &str, domain: i32, dist: f64, varIDs: &Vec<i32>, begin: f64, end: f64, parameters: &TraCIResults) {
        todo!("port RouteProbe::subscribeContext")
    }

    /// C++: `void RouteProbe::unsubscribeContext(const std::string &, int, double)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(RouteProbe, ROUTEPROBE)
    #[allow(unused_variables)]
    pub fn unsubscribe_context(objectID: &str, domain: i32, dist: f64) {
        todo!("port RouteProbe::unsubscribeContext")
    }

    /// C++: `const libsumo::SubscriptionResults RouteProbe::getAllSubscriptionResults()` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(RouteProbe, ROUTEPROBE)
    #[allow(unused_variables)]
    pub fn get_all_subscription_results() -> SubscriptionResults {
        todo!("port RouteProbe::getAllSubscriptionResults")
    }

    /// C++: `const libsumo::TraCIResults RouteProbe::getSubscriptionResults(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(RouteProbe, ROUTEPROBE)
    #[allow(unused_variables)]
    pub fn get_subscription_results(objectID: &str) -> TraCIResults {
        todo!("port RouteProbe::getSubscriptionResults")
    }

    /// C++: `const libsumo::ContextSubscriptionResults RouteProbe::getAllContextSubscriptionResults()` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(RouteProbe, ROUTEPROBE)
    #[allow(unused_variables)]
    pub fn get_all_context_subscription_results() -> ContextSubscriptionResults {
        todo!("port RouteProbe::getAllContextSubscriptionResults")
    }

    /// C++: `const libsumo::SubscriptionResults RouteProbe::getContextSubscriptionResults(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(RouteProbe, ROUTEPROBE)
    #[allow(unused_variables)]
    pub fn get_context_subscription_results(objectID: &str) -> SubscriptionResults {
        todo!("port RouteProbe::getContextSubscriptionResults")
    }

    /// C++: `void RouteProbe::subscribeParameterWithKey(const std::string &, const std::string &, double, double)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(RouteProbe, ROUTEPROBE)
    #[allow(unused_variables)]
    pub fn subscribe_parameter_with_key(objectID: &str, key: &str, beginTime: f64, endTime: f64) {
        todo!("port RouteProbe::subscribeParameterWithKey")
    }

    /// C++: `int RouteProbe::domainID()` [static]
    // LIBSUMO_SUBSCRIPTION_API
    #[allow(unused_variables)]
    pub fn domain_id() -> i32 {
        todo!("port RouteProbe::domainID")
    }

    /// C++: `std::shared_ptr<VariableWrapper> RouteProbe::makeWrapper()` [static]
    // std::shared_ptr<VariableWrapper>
    // RouteProbe::makeWrapper() {
    //     return std::make_shared<Helper::SubscriptionWrapper>(handleVariable, mySubscriptionResults, myContextSubscriptionResults);
    // }
    #[allow(unused_variables)]
    pub fn make_wrapper() -> Rc<VariableWrapper> {
        todo!("port RouteProbe::makeWrapper")
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
        todo!("port RouteProbe::handleVariable")
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
        todo!("port RouteProbe::getRouteProbe")
    }

    /// C++: `void RouteProbe::RouteProbe()` [private]
    /// invalidated standard constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new() -> Self {
        todo!("port RouteProbe::RouteProbe")
    }

}

