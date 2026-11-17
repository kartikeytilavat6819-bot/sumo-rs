//! Port of libsumo/VariableSpeedSign.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::traci::traci_constants::*;

/// C++ class `libsumo::VariableSpeedSign` (libsumo/VariableSpeedSign.h)
pub struct VariableSpeedSign {
}

// C++ static members of VariableSpeedSign (port as module-level statics, see AGENTS.md):
//   static const int VariableSpeedSign::DOMAIN_ID;
//   static SubscriptionResults VariableSpeedSign::mySubscriptionResults;
//   static ContextSubscriptionResults VariableSpeedSign::myContextSubscriptionResults;

thread_local! {
    /// C++ static `VariableSpeedSign::mySubscriptionResults`
    static MY_SUBSCRIPTION_RESULTS: std::cell::RefCell<SubscriptionResults> = std::cell::RefCell::new(SubscriptionResults::new());
    /// C++ static `VariableSpeedSign::myContextSubscriptionResults`
    static MY_CONTEXT_SUBSCRIPTION_RESULTS: std::cell::RefCell<ContextSubscriptionResults> = std::cell::RefCell::new(ContextSubscriptionResults::new());
}

impl VariableSpeedSign {
    /// C++: `std::vector<std::string> VariableSpeedSign::getLanes(const std::string &)` [static]
    // std::vector<std::string>
    // VariableSpeedSign::getLanes(const std::string& vssID) {
    //     std::vector<std::string> result;
    //     MSLaneSpeedTrigger* vss = getVariableSpeedSign(vssID);
    //     for (MSLane* lane : vss->getLanes()) {
    //         result.push_back(lane->getID());
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_lanes(vssID: &str) -> Vec<String> {
        let vss = Self::get_variable_speed_sign(vssID);
        vss.m().get_lanes().iter().map(|lane| lane.get_id().to_string()).collect()
    }

    /// C++: `std::vector<std::string> VariableSpeedSign::getIDList()` [static]
    // std::vector<std::string>
    // VariableSpeedSign::getIDList() {
    //     MSNet::getInstance(); // just to check that we actually have a network
    //     std::vector<std::string> ids;
    //     for (auto& item : MSLaneSpeedTrigger::getInstances()) {
    //         ids.push_back(item.first);
    //     }
    //     return ids;
    // }
    #[allow(unused_variables)]
    pub fn get_id_list() -> Vec<String> {
        MSNet::get_instance(); // just to check that we actually have a network
        let mut ids: Vec<String> = Vec::new();
        for (key, _) in MSLaneSpeedTrigger::get_instances() {
            ids.push(key.clone());
        }
        ids
    }

    /// C++: `int VariableSpeedSign::getIDCount()` [static]
    // int
    // VariableSpeedSign::getIDCount() {
    //     return (int)getIDList().size();
    // }
    #[allow(unused_variables)]
    pub fn get_id_count() -> i32 {
        Self::get_id_list().len() as i32
    }

    /// C++: `std::string VariableSpeedSign::getParameter(const std::string &, const std::string &)` [static]
    // std::string
    // VariableSpeedSign::getParameter(const std::string& /* vssID */, const std::string& /* param */) {
    //     return "";
    // }
    #[allow(unused_variables)]
    pub fn get_parameter(objectID: &str, key: &str) -> String {
        String::new()
    }

    /// C++: `const std::pair<std::string, std::string> VariableSpeedSign::getParameterWithKey(const std::string &, const std::string &)` [static]
    // LIBSUMO_GET_PARAMETER_WITH_KEY_IMPLEMENTATION(VariableSpeedSign)
    #[allow(unused_variables)]
    pub fn get_parameter_with_key(objectID: &str, key: &str) -> (String, String) {
        (key.to_string(), Self::get_parameter(objectID, key))
    }

    /// C++: `void VariableSpeedSign::setParameter(const std::string &, const std::string &, const std::string &)` [static]
    // void
    // VariableSpeedSign::setParameter(const std::string& /* vssID */, const std::string& /* key */, const std::string& /* value */) {
    //     //MSVariableSpeedSign* r = const_cast<MSVariableSpeedSign*>(getVariableSpeedSign(vssID));
    //     //r->setParameter(key, value);
    // }
    #[allow(unused_variables)]
    pub fn set_parameter(objectID: &str, key: &str, value: &str) {
    }

    /// C++: `void VariableSpeedSign::subscribe(const std::string &, const std::vector<int> &, double, double, const libsumo::TraCIResults &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(VariableSpeedSign, VARIABLESPEEDSIGN)
    #[allow(unused_variables)]
    pub fn subscribe(objectID: &str, varIDs: &Vec<i32>, begin: f64, end: f64, parameters: &TraCIResults) {
        Helper::subscribe(CMD_SUBSCRIBE_VARIABLESPEEDSIGN_VARIABLE, objectID, varIDs, begin, end, parameters, 0, 0.0);
    }

    /// C++: `void VariableSpeedSign::unsubscribe(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(VariableSpeedSign, VARIABLESPEEDSIGN)
    #[allow(unused_variables)]
    pub fn unsubscribe(objectID: &str) {
        Helper::subscribe(CMD_SUBSCRIBE_VARIABLESPEEDSIGN_VARIABLE, objectID, &Vec::new(), INVALID_DOUBLE_VALUE, INVALID_DOUBLE_VALUE, &TraCIResults::new(), 0, 0.0);
    }

    /// C++: `void VariableSpeedSign::subscribeContext(const std::string &, int, double, const std::vector<int> &, double, double, const libsumo::TraCIResults &)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn subscribe_context(objectID: &str, domain: i32, dist: f64, varIDs: &Vec<i32>, begin: f64, end: f64, parameters: &TraCIResults) {
        Helper::subscribe(CMD_SUBSCRIBE_VARIABLESPEEDSIGN_CONTEXT, objectID, varIDs, begin, end, parameters, domain, dist);
    }

    /// C++: `void VariableSpeedSign::unsubscribeContext(const std::string &, int, double)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(VariableSpeedSign, VARIABLESPEEDSIGN)
    #[allow(unused_variables)]
    pub fn unsubscribe_context(objectID: &str, domain: i32, dist: f64) {
        Helper::subscribe(CMD_SUBSCRIBE_VARIABLESPEEDSIGN_CONTEXT, objectID, &Vec::new(), INVALID_DOUBLE_VALUE, INVALID_DOUBLE_VALUE, &TraCIResults::new(), domain, dist);
    }

    /// C++: `const libsumo::SubscriptionResults VariableSpeedSign::getAllSubscriptionResults()` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(VariableSpeedSign, VARIABLESPEEDSIGN)
    #[allow(unused_variables)]
    pub fn get_all_subscription_results() -> SubscriptionResults {
        MY_SUBSCRIPTION_RESULTS.with(|r| r.borrow().clone())
    }

    /// C++: `const libsumo::TraCIResults VariableSpeedSign::getSubscriptionResults(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(VariableSpeedSign, VARIABLESPEEDSIGN)
    #[allow(unused_variables)]
    pub fn get_subscription_results(objectID: &str) -> TraCIResults {
        MY_SUBSCRIPTION_RESULTS.with(|r| r.borrow_mut().entry(objectID.to_string()).or_default().clone())
    }

    /// C++: `const libsumo::ContextSubscriptionResults VariableSpeedSign::getAllContextSubscriptionResults()` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(VariableSpeedSign, VARIABLESPEEDSIGN)
    #[allow(unused_variables)]
    pub fn get_all_context_subscription_results() -> ContextSubscriptionResults {
        MY_CONTEXT_SUBSCRIPTION_RESULTS.with(|r| r.borrow().clone())
    }

    /// C++: `const libsumo::SubscriptionResults VariableSpeedSign::getContextSubscriptionResults(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(VariableSpeedSign, VARIABLESPEEDSIGN)
    #[allow(unused_variables)]
    pub fn get_context_subscription_results(objectID: &str) -> SubscriptionResults {
        MY_CONTEXT_SUBSCRIPTION_RESULTS.with(|r| r.borrow_mut().entry(objectID.to_string()).or_default().clone())
    }

    /// C++: `void VariableSpeedSign::subscribeParameterWithKey(const std::string &, const std::string &, double, double)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(VariableSpeedSign, VARIABLESPEEDSIGN)
    #[allow(unused_variables)]
    pub fn subscribe_parameter_with_key(objectID: &str, key: &str, beginTime: f64, endTime: f64) {
        let mut params: TraCIResults = TraCIResults::new();
        params.insert(VAR_PARAMETER_WITH_KEY, crate::libsumo::helper::as_traci_result(TraCIString { base: TraCIResult { result_type: TYPE_STRING }, value: key.to_string() }));
        Helper::subscribe(CMD_SUBSCRIBE_VARIABLESPEEDSIGN_VARIABLE, objectID, &vec![VAR_PARAMETER_WITH_KEY], beginTime, endTime, &params, 0, 0.0);
    }

    /// C++: `int VariableSpeedSign::domainID()` [static]
    // LIBSUMO_SUBSCRIPTION_API
    #[allow(unused_variables)]
    pub fn domain_id() -> i32 {
        CMD_GET_VARIABLESPEEDSIGN_VARIABLE
    }

    /// C++: `std::shared_ptr<VariableWrapper> VariableSpeedSign::makeWrapper()` [static]
    // std::shared_ptr<VariableWrapper>
    // VariableSpeedSign::makeWrapper() {
    //     return std::make_shared<Helper::SubscriptionWrapper>(handleVariable, mySubscriptionResults, myContextSubscriptionResults);
    // }
    #[allow(unused_variables)]
    pub fn make_wrapper() -> Rc<VariableWrapper> {
        Helper::make_subscription_wrapper(Some(Self::handle_variable), unsafe { &mut *MY_SUBSCRIPTION_RESULTS.with(|r| r.as_ptr()) }, unsafe { &mut *MY_CONTEXT_SUBSCRIPTION_RESULTS.with(|r| r.as_ptr()) })
    }

    /// C++: `bool VariableSpeedSign::handleVariable(const std::string &, const int, VariableWrapper *, tcpip::Storage *)` [static]
    // bool
    // VariableSpeedSign::handleVariable(const std::string& objID, const int variable, VariableWrapper* wrapper, tcpip::Storage* paramData) {
    //     switch (variable) {
    //         case TRACI_ID_LIST:
    //             return wrapper->wrapStringList(objID, variable, getIDList());
    //         case ID_COUNT:
    //             return wrapper->wrapInt(objID, variable, getIDCount());
    //         case VAR_LANES:
    //             return wrapper->wrapStringList(objID, variable, getLanes(objID));
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
            VAR_LANES => wrapper.m().wrap_string_list(objID, variable, &Self::get_lanes(objID)),
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

    /// C++: `MSLaneSpeedTrigger * VariableSpeedSign::getVariableSpeedSign(const std::string &)` [static, private]
    // MSLaneSpeedTrigger*
    // VariableSpeedSign::getVariableSpeedSign(const std::string& id) {
    //     const auto& dict = MSLaneSpeedTrigger::getInstances();
    //     auto it = dict.find(id);
    //     if (it == dict.end()) {
    //         throw TraCIException("VariableSpeedSign '" + id + "' is not known");
    //     }
    //     return it->second;
    // }
    #[allow(unused_variables)]
    pub fn get_variable_speed_sign(vssID: &str) -> Ptr<MSLaneSpeedTrigger> {
        match MSLaneSpeedTrigger::get_instances().get(vssID) {
            Some(c) => *c,
            None => panic!("VariableSpeedSign '{vssID}' is not known"),
        }
    }

    /// C++: `void VariableSpeedSign::VariableSpeedSign()` [private]
    /// invalidated standard constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new() -> Self {
        panic!("libsumo::VariableSpeedSign cannot be constructed: the standard constructor is invalidated in C++")
    }

}
