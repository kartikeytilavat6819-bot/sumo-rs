//! Port of libsumo/Route.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
// shortcut: C++ throw TraCIException(msg) is ported as panic!("{}", msg), upgrade when TraCIException is ported
use crate::prelude::*;

/// C++ class `libsumo::Route` (libsumo/Route.h)
pub struct Route {
}

// C++ static members of Route (port as module-level statics, see AGENTS.md):
//   static const int Route::DOMAIN_ID;
//   static SubscriptionResults Route::mySubscriptionResults;
//   static ContextSubscriptionResults Route::myContextSubscriptionResults;
thread_local! {
    /// C++ static `Route::mySubscriptionResults`
    static MY_SUBSCRIPTION_RESULTS: std::cell::RefCell<SubscriptionResults> = std::cell::RefCell::new(SubscriptionResults::new());
    /// C++ static `Route::myContextSubscriptionResults`
    static MY_CONTEXT_SUBSCRIPTION_RESULTS: std::cell::RefCell<ContextSubscriptionResults> = std::cell::RefCell::new(ContextSubscriptionResults::new());
}


impl Route {
    /// C++: `std::vector<std::string> Route::getEdges(const std::string &)` [static]
    // std::vector<std::string>
    // Route::getEdges(const std::string& routeID) {
    //     ConstMSRoutePtr r = getRoute(routeID);
    //     std::vector<std::string> ids;
    //     for (ConstMSEdgeVector::const_iterator i = r->getEdges().begin(); i != r->getEdges().end(); ++i) {
    //         ids.push_back((*i)->getID());
    //     }
    //     return ids;
    // }
    #[allow(unused_variables)]
    pub fn get_edges(routeID: &str) -> Vec<String> {
        let r = Self::get_route(routeID);
        let mut ids: Vec<String> = Vec::new();
        for e in r.get_edges() {
            ids.push(e.get_id().to_string());
        }
        ids
    }

    /// C++: `std::vector<std::string> Route::getIDList()` [static]
    // std::vector<std::string>
    // Route::getIDList() {
    //     MSNet::getInstance(); // just to check that we actually have a network
    //     std::vector<std::string> ids;
    //     MSRoute::insertIDs(ids);
    //     return ids;
    // }
    #[allow(unused_variables)]
    pub fn get_id_list() -> Vec<String> {
        MSNet::get_instance(); // just to check that we actually have a network
        let mut ids: Vec<String> = Vec::new();
        MSRoute::insert_i_ds(&mut ids);
        ids
    }

    /// C++: `int Route::getIDCount()` [static]
    // int
    // Route::getIDCount() {
    //     return (int)getIDList().size();
    // }
    #[allow(unused_variables)]
    pub fn get_id_count() -> i32 {
        Self::get_id_list().len() as i32
    }

    /// C++: `std::string Route::getParameter(const std::string &, const std::string &)` [static]
    // std::string
    // Route::getParameter(const std::string& routeID, const std::string& param) {
    //     ConstMSRoutePtr r = getRoute(routeID);
    //     return r->getParameter(param, "");
    // }
    #[allow(unused_variables)]
    pub fn get_parameter(objectID: &str, key: &str) -> String {
        Self::get_route(objectID).base2.get_parameter(key, "")
    }

    /// C++: `const std::pair<std::string, std::string> Route::getParameterWithKey(const std::string &, const std::string &)` [static]
    // LIBSUMO_GET_PARAMETER_WITH_KEY_IMPLEMENTATION(Route)
    #[allow(unused_variables)]
    pub fn get_parameter_with_key(objectID: &str, key: &str) -> (String, String) {
        (key.to_string(), Self::get_parameter(objectID, key))
    }

    /// C++: `void Route::setParameter(const std::string &, const std::string &, const std::string &)` [static]
    // void
    // Route::setParameter(const std::string& routeID, const std::string& key, const std::string& value) {
    //     MSRoute* r = const_cast<MSRoute*>(getRoute(routeID).get());
    //     r->setParameter(key, value);
    // }
    #[allow(unused_variables)]
    pub fn set_parameter(objectID: &str, key: &str, value: &str) {
        let r = Self::get_route(objectID);
        // C++ const_cast: the route dictionary holds const pointers, parameters are still set in place
        Ptr::from_raw(Rc::as_ptr(&r) as *mut MSRoute).m().base2.set_parameter(key, value);
    }

    /// C++: `void Route::subscribe(const std::string &, const std::vector<int> &, double, double, const libsumo::TraCIResults &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Route, ROUTE)
    #[allow(unused_variables)]
    pub fn subscribe(objectID: &str, varIDs: &Vec<i32>, begin: f64, end: f64, parameters: &TraCIResults) {
        Helper::subscribe(CMD_SUBSCRIBE_ROUTE_VARIABLE, objectID, varIDs, begin, end, parameters, 0, 0.0);
    }

    /// C++: `void Route::unsubscribe(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Route, ROUTE)
    #[allow(unused_variables)]
    pub fn unsubscribe(objectID: &str) {
        Helper::subscribe(CMD_SUBSCRIBE_ROUTE_VARIABLE, objectID, &Vec::new(), INVALID_DOUBLE_VALUE, INVALID_DOUBLE_VALUE, &TraCIResults::new(), 0, 0.0);
    }

    /// C++: `void Route::subscribeContext(const std::string &, int, double, const std::vector<int> &, double, double, const libsumo::TraCIResults &)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn subscribe_context(objectID: &str, domain: i32, dist: f64, varIDs: &Vec<i32>, begin: f64, end: f64, parameters: &TraCIResults) {
        Helper::subscribe(CMD_SUBSCRIBE_ROUTE_CONTEXT, objectID, varIDs, begin, end, parameters, domain, dist);
    }

    /// C++: `void Route::unsubscribeContext(const std::string &, int, double)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Route, ROUTE)
    #[allow(unused_variables)]
    pub fn unsubscribe_context(objectID: &str, domain: i32, dist: f64) {
        Helper::subscribe(CMD_SUBSCRIBE_ROUTE_CONTEXT, objectID, &Vec::new(), INVALID_DOUBLE_VALUE, INVALID_DOUBLE_VALUE, &TraCIResults::new(), domain, dist);
    }

    /// C++: `const libsumo::SubscriptionResults Route::getAllSubscriptionResults()` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Route, ROUTE)
    #[allow(unused_variables)]
    pub fn get_all_subscription_results() -> SubscriptionResults {
        MY_SUBSCRIPTION_RESULTS.with(|r| r.borrow().clone())
    }

    /// C++: `const libsumo::TraCIResults Route::getSubscriptionResults(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Route, ROUTE)
    #[allow(unused_variables)]
    pub fn get_subscription_results(objectID: &str) -> TraCIResults {
        MY_SUBSCRIPTION_RESULTS.with(|r| r.borrow_mut().entry(objectID.to_string()).or_default().clone())
    }

    /// C++: `const libsumo::ContextSubscriptionResults Route::getAllContextSubscriptionResults()` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Route, ROUTE)
    #[allow(unused_variables)]
    pub fn get_all_context_subscription_results() -> ContextSubscriptionResults {
        MY_CONTEXT_SUBSCRIPTION_RESULTS.with(|r| r.borrow().clone())
    }

    /// C++: `const libsumo::SubscriptionResults Route::getContextSubscriptionResults(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Route, ROUTE)
    #[allow(unused_variables)]
    pub fn get_context_subscription_results(objectID: &str) -> SubscriptionResults {
        MY_CONTEXT_SUBSCRIPTION_RESULTS.with(|r| r.borrow_mut().entry(objectID.to_string()).or_default().clone())
    }

    /// C++: `void Route::subscribeParameterWithKey(const std::string &, const std::string &, double, double)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Route, ROUTE)
    #[allow(unused_variables)]
    pub fn subscribe_parameter_with_key(objectID: &str, key: &str, beginTime: f64, endTime: f64) {
        // shortcut: TraCIString is a subclass wrapper (base: TraCIResult), so std::make_shared<TraCIString> cannot be stored as Rc<TraCIResult> yet, upgrade with the TraCIResult class hierarchy
        panic!("shortcut: Route::subscribeParameterWithKey needs TraCIString as Rc<TraCIResult>")
    }

    /// C++: `int Route::domainID()` [static]
    // LIBSUMO_SUBSCRIPTION_API
    #[allow(unused_variables)]
    pub fn domain_id() -> i32 {
        CMD_GET_ROUTE_VARIABLE
    }

    /// C++: `void Route::add(const std::string &, const std::vector<std::string> &)` [static]
    // void
    // Route::add(const std::string& routeID, const std::vector<std::string>& edgeIDs) {
    //     ConstMSEdgeVector edges;
    //     if (edgeIDs.size() == 0) {
    //         throw TraCIException("Cannot add route '" + routeID + "' without edges.");
    //     }
    //     for (std::vector<std::string>::const_iterator ei = edgeIDs.begin(); ei != edgeIDs.end(); ++ei) {
    //         MSEdge* edge = MSEdge::dictionary(*ei);
    //         if (edge == nullptr) {
    //             throw TraCIException("Unknown edge '" + *ei + "' in route.");
    //         }
    //         edges.push_back(edge);
    //     }
    //     const StopParVector stops;
    //     ConstMSRoutePtr route = std::make_shared<MSRoute>(routeID, edges, true, nullptr, stops);
    //     if (!MSRoute::dictionary(routeID, route)) {
    //         throw TraCIException("Could not add route '" + routeID + "'.");
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add(routeID: &str, edges: &Vec<String>) {
        let mut edges: ConstMSEdgeVector = Vec::new();
        if edgeIDs.len() == 0 {
            panic!("{}", format!("Cannot add route '{}' without edges.", routeID));
        }
        for ei in edgeIDs.iter() {
            let edge = MSEdge::dictionary_2(ei);
            if edge.is_null() {
                panic!("{}", format!("Unknown edge '{}' in route.", ei));
            }
            edges.push(edge);
        }
        let stops: StopParVector = Vec::new();
        let route: ConstMSRoutePtr = Rc::new(MSRoute::new(routeID, &edges, true, Ptr::null(), &stops, -1, -1));
        if !MSRoute::dictionary(routeID, route) {
            panic!("{}", format!("Could not add route '{}'.", routeID));
        }
    }

    /// C++: `void Route::remove(const std::string &)` [static]
    // void
    // Route::remove(const std::string& routeID) {
    //     MSRoute* r = const_cast<MSRoute*>(getRoute(routeID).get());
    //     r->checkRemoval(true);
    // }
    #[allow(unused_variables)]
    pub fn remove(routeID: &str) {
        Self::get_route(routeID).check_removal(true);
    }

    /// C++: `std::shared_ptr<VariableWrapper> Route::makeWrapper()` [static]
    // std::shared_ptr<VariableWrapper>
    // Route::makeWrapper() {
    //     return std::make_shared<Helper::SubscriptionWrapper>(handleVariable, mySubscriptionResults, myContextSubscriptionResults);
    // }
    #[allow(unused_variables)]
    pub fn make_wrapper() -> Rc<VariableWrapper> {
        // shortcut: Helper::SubscriptionWrapper borrows the class-level subscription statics by reference, which thread_local RefCells cannot hand out yet
        panic!("shortcut: Route::makeWrapper needs the subscription wrapper over class statics")
    }

    /// C++: `bool Route::handleVariable(const std::string &, const int, VariableWrapper *, tcpip::Storage *)` [static]
    // bool
    // Route::handleVariable(const std::string& objID, const int variable, VariableWrapper* wrapper, tcpip::Storage* paramData) {
    //     switch (variable) {
    //         case TRACI_ID_LIST:
    //             return wrapper->wrapStringList(objID, variable, getIDList());
    //         case ID_COUNT:
    //             return wrapper->wrapInt(objID, variable, getIDCount());
    //         case VAR_EDGES:
    //             return wrapper->wrapStringList(objID, variable, getEdges(objID));
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
            VAR_EDGES => wrapper.m().wrap_string_list(objID, variable, &Self::get_edges(objID)),
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

    /// C++: `ConstMSRoutePtr Route::getRoute(const std::string &)` [static, private]
    // ConstMSRoutePtr
    // Route::getRoute(const std::string& id) {
    //     ConstMSRoutePtr r = MSRoute::dictionary(id);
    //     if (r == nullptr) {
    //         throw TraCIException("Route '" + id + "' is not known");
    //     }
    //     return r;
    // }
    #[allow(unused_variables)]
    pub fn get_route(id: &str) -> ConstMSRoutePtr {
        // shortcut: MSRoute::dictionary cannot return nullptr through Rc<MSRoute>, so existence is checked with hasRoute first
        if !MSRoute::has_route(id) {
            panic!("{}", format!("Route '{}' is not known", id));
        }
        MSRoute::dictionary_3(id, Ptr::null())
    }

    /// C++: `void Route::Route()` [private]
    /// invalidated standard constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new() -> Self {
        panic!("libsumo::Route cannot be constructed: the standard constructor is invalidated in C++")
    }

}

