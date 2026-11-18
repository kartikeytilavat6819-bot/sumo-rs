//! Port of libsumo/Rerouter.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
// shortcut: C++ throw TraCIException(msg) is ported as panic!("{}", msg), upgrade when TraCIException is ported
use crate::prelude::*;

/// C++ class `libsumo::Rerouter` (libsumo/Rerouter.h)
pub struct Rerouter {
}

// C++ static members of Rerouter (port as module-level statics, see AGENTS.md):
//   static const int Rerouter::DOMAIN_ID;
//   static SubscriptionResults Rerouter::mySubscriptionResults;
//   static ContextSubscriptionResults Rerouter::myContextSubscriptionResults;
thread_local! {
    /// C++ static `Rerouter::mySubscriptionResults`
    static MY_SUBSCRIPTION_RESULTS: std::cell::RefCell<SubscriptionResults> = std::cell::RefCell::new(SubscriptionResults::new());
    /// C++ static `Rerouter::myContextSubscriptionResults`
    static MY_CONTEXT_SUBSCRIPTION_RESULTS: std::cell::RefCell<ContextSubscriptionResults> = std::cell::RefCell::new(ContextSubscriptionResults::new());
}


impl Rerouter {
    /// C++: `std::vector<std::string> Rerouter::getIDList()` [static]
    // std::vector<std::string>
    // Rerouter::getIDList() {
    //     MSNet::getInstance(); // just to check that we actually have a network
    //     std::vector<std::string> ids;
    //     for (const auto& item : MSTriggeredRerouter::getInstances()) {
    //         ids.push_back(item.first);
    //     }
    //     return ids;
    // }
    #[allow(unused_variables)]
    pub fn get_id_list() -> Vec<String> {
        MSNet::get_instance(); // just to check that we actually have a network
        let mut ids: Vec<String> = Vec::new();
        for (id, _) in MSTriggeredRerouter::get_instances() {
            ids.push(id);
        }
        ids
    }

    /// C++: `int Rerouter::getIDCount()` [static]
    // int
    // Rerouter::getIDCount() {
    //     return (int)MSTriggeredRerouter::getInstances().size();
    // }
    #[allow(unused_variables)]
    pub fn get_id_count() -> i32 {
        MSTriggeredRerouter::get_instances().len() as i32
    }

    /// C++: `std::string Rerouter::getParameter(const std::string &, const std::string &)` [static]
    // std::string
    // Rerouter::getParameter(const std::string& /* rerouterID */, const std::string& /* param */) {
    //     return "";
    // }
    #[allow(unused_variables)]
    pub fn get_parameter(objectID: &str, key: &str) -> String {
        String::new()
    }

    /// C++: `const std::pair<std::string, std::string> Rerouter::getParameterWithKey(const std::string &, const std::string &)` [static]
    // LIBSUMO_GET_PARAMETER_WITH_KEY_IMPLEMENTATION(Rerouter)
    #[allow(unused_variables)]
    pub fn get_parameter_with_key(objectID: &str, key: &str) -> (String, String) {
        (key.to_string(), Self::get_parameter(objectID, key))
    }

    /// C++: `void Rerouter::setParameter(const std::string &, const std::string &, const std::string &)` [static]
    // void
    // Rerouter::setParameter(const std::string& /* rerouterID */, const std::string& /* key */, const std::string& /* value */) {
    //     //MSRerouter* r = const_cast<MSRerouter*>(getRerouter(rerouterID));
    //     //r->setParameter(key, value);
    // }
    #[allow(unused_variables)]
    pub fn set_parameter(objectID: &str, key: &str, value: &str) {
        // the C++ body is commented out upstream: rerouters have no settable parameters through libsumo
    }

    /// C++: `void Rerouter::subscribe(const std::string &, const std::vector<int> &, double, double, const libsumo::TraCIResults &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Rerouter, REROUTER)
    #[allow(unused_variables)]
    pub fn subscribe(objectID: &str, varIDs: &Vec<i32>, begin: f64, end: f64, parameters: &TraCIResults) {
        Helper::subscribe(CMD_SUBSCRIBE_REROUTER_VARIABLE, objectID, varIDs, begin, end, parameters, 0, 0.0);
    }

    /// C++: `void Rerouter::unsubscribe(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Rerouter, REROUTER)
    #[allow(unused_variables)]
    pub fn unsubscribe(objectID: &str) {
        Helper::subscribe(CMD_SUBSCRIBE_REROUTER_VARIABLE, objectID, &Vec::new(), INVALID_DOUBLE_VALUE, INVALID_DOUBLE_VALUE, &TraCIResults::new(), 0, 0.0);
    }

    /// C++: `void Rerouter::subscribeContext(const std::string &, int, double, const std::vector<int> &, double, double, const libsumo::TraCIResults &)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn subscribe_context(objectID: &str, domain: i32, dist: f64, varIDs: &Vec<i32>, begin: f64, end: f64, parameters: &TraCIResults) {
        Helper::subscribe(CMD_SUBSCRIBE_REROUTER_CONTEXT, objectID, varIDs, begin, end, parameters, domain, dist);
    }

    /// C++: `void Rerouter::unsubscribeContext(const std::string &, int, double)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Rerouter, REROUTER)
    #[allow(unused_variables)]
    pub fn unsubscribe_context(objectID: &str, domain: i32, dist: f64) {
        Helper::subscribe(CMD_SUBSCRIBE_REROUTER_CONTEXT, objectID, &Vec::new(), INVALID_DOUBLE_VALUE, INVALID_DOUBLE_VALUE, &TraCIResults::new(), domain, dist);
    }

    /// C++: `const libsumo::SubscriptionResults Rerouter::getAllSubscriptionResults()` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Rerouter, REROUTER)
    #[allow(unused_variables)]
    pub fn get_all_subscription_results() -> SubscriptionResults {
        MY_SUBSCRIPTION_RESULTS.with(|r| r.borrow().clone())
    }

    /// C++: `const libsumo::TraCIResults Rerouter::getSubscriptionResults(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Rerouter, REROUTER)
    #[allow(unused_variables)]
    pub fn get_subscription_results(objectID: &str) -> TraCIResults {
        MY_SUBSCRIPTION_RESULTS.with(|r| r.borrow_mut().entry(objectID.to_string()).or_default().clone())
    }

    /// C++: `const libsumo::ContextSubscriptionResults Rerouter::getAllContextSubscriptionResults()` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Rerouter, REROUTER)
    #[allow(unused_variables)]
    pub fn get_all_context_subscription_results() -> ContextSubscriptionResults {
        MY_CONTEXT_SUBSCRIPTION_RESULTS.with(|r| r.borrow().clone())
    }

    /// C++: `const libsumo::SubscriptionResults Rerouter::getContextSubscriptionResults(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Rerouter, REROUTER)
    #[allow(unused_variables)]
    pub fn get_context_subscription_results(objectID: &str) -> SubscriptionResults {
        MY_CONTEXT_SUBSCRIPTION_RESULTS.with(|r| r.borrow_mut().entry(objectID.to_string()).or_default().clone())
    }

    /// C++: `void Rerouter::subscribeParameterWithKey(const std::string &, const std::string &, double, double)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Rerouter, REROUTER)
    #[allow(unused_variables)]
    pub fn subscribe_parameter_with_key(objectID: &str, key: &str, beginTime: f64, endTime: f64) {
        let mut params: TraCIResults = TraCIResults::new();
        params.insert(VAR_PARAMETER_WITH_KEY, crate::libsumo::helper::as_traci_result(TraCIString { base: TraCIResult { result_type: TYPE_STRING }, value: key.to_string() }));
        Helper::subscribe(CMD_SUBSCRIBE_REROUTER_VARIABLE, objectID, &vec![VAR_PARAMETER_WITH_KEY], beginTime, endTime, &params, 0, 0.0);
    }

    /// C++: `int Rerouter::domainID()` [static]
    // LIBSUMO_SUBSCRIPTION_API
    #[allow(unused_variables)]
    pub fn domain_id() -> i32 {
        CMD_GET_REROUTER_VARIABLE
    }

    /// C++: `std::shared_ptr<VariableWrapper> Rerouter::makeWrapper()` [static]
    // std::shared_ptr<VariableWrapper>
    // Rerouter::makeWrapper() {
    //     return std::make_shared<Helper::SubscriptionWrapper>(handleVariable, mySubscriptionResults, myContextSubscriptionResults);
    // }
    #[allow(unused_variables)]
    pub fn make_wrapper() -> Rc<VariableWrapper> {
        Helper::make_subscription_wrapper(Some(Self::handle_variable), unsafe { &mut *MY_SUBSCRIPTION_RESULTS.with(|r| r.as_ptr()) }, unsafe { &mut *MY_CONTEXT_SUBSCRIPTION_RESULTS.with(|r| r.as_ptr()) })
    }

    /// C++: `bool Rerouter::handleVariable(const std::string &, const int, VariableWrapper *, tcpip::Storage *)` [static]
    // bool
    // Rerouter::handleVariable(const std::string& objID, const int variable, VariableWrapper* wrapper, tcpip::Storage* paramData) {
    //     switch (variable) {
    //         case TRACI_ID_LIST:
    //             return wrapper->wrapStringList(objID, variable, getIDList());
    //         case ID_COUNT:
    //             return wrapper->wrapInt(objID, variable, getIDCount());
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
            VAR_PARAMETER => {
                paramData.m().read_unsigned_byte();
                // C++ readString() throws ProcessError on malformed storage; panic_any keeps it as ProcessError so dispatch_command re-raises it like C++
                let key = paramData.m().read_string().unwrap_or_else(|e| std::panic::panic_any(e));
                wrapper.m().wrap_string(objID, variable, &Self::get_parameter(objID, &key))
            }
            VAR_PARAMETER_WITH_KEY => {
                paramData.m().read_unsigned_byte();
                // C++ readString() throws ProcessError on malformed storage; panic_any keeps it as ProcessError so dispatch_command re-raises it like C++
                let key = paramData.m().read_string().unwrap_or_else(|e| std::panic::panic_any(e));
                wrapper.m().wrap_string_pair(objID, variable, &Self::get_parameter_with_key(objID, &key))
            }
            _ => false,
        }
    }

    /// C++: `MSTriggeredRerouter * Rerouter::getRerouter(const std::string &)` [static, private]
    // MSTriggeredRerouter*
    // Rerouter::getRerouter(const std::string& id) {
    //     MSTriggeredRerouter* s = nullptr;
    //     if (s == nullptr) {
    //         throw TraCIException("Rerouter '" + id + "' is not known");
    //     }
    //     return s;
    // }
    #[allow(unused_variables)]
    pub fn get_rerouter(id: &str) -> Ptr<MSTriggeredRerouter> {
        // C++ always throws here: the lookup is a stub that never finds a rerouter
        let s: Ptr<MSTriggeredRerouter> = Ptr::null();
        if s.is_null() {
            panic!("{}", format!("Rerouter '{}' is not known", id));
        }
        s
    }

    /// C++: `void Rerouter::Rerouter()` [private]
    /// invalidated standard constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new() -> Self {
        panic!("libsumo::Rerouter cannot be constructed: the standard constructor is invalidated in C++")
    }

}

