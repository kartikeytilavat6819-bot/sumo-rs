//! Port of libsumo/Junction.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::traci::traci_constants::*;

/// C++ class `libsumo::Junction` (libsumo/Junction.h)
pub struct Junction {
}

// C++ static members of Junction (port as module-level statics, see AGENTS.md):
//   static const int Junction::DOMAIN_ID;
//   static SubscriptionResults Junction::mySubscriptionResults;
//   static ContextSubscriptionResults Junction::myContextSubscriptionResults;

thread_local! {
    /// C++ static `Junction::mySubscriptionResults`
    static MY_SUBSCRIPTION_RESULTS: std::cell::RefCell<SubscriptionResults> = std::cell::RefCell::new(SubscriptionResults::new());
    /// C++ static `Junction::myContextSubscriptionResults`
    static MY_CONTEXT_SUBSCRIPTION_RESULTS: std::cell::RefCell<ContextSubscriptionResults> = std::cell::RefCell::new(ContextSubscriptionResults::new());
}
//   static NamedRTree * Junction::myTree;

impl Junction {
    /// C++: `libsumo::TraCIPosition Junction::getPosition(const std::string &, bool)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_position(junctionID: &str, includeZ: bool) -> TraCIPosition {
        Helper::make_tra_ci_position(Self::get_junction(junctionID).get_position(false), includeZ)
    }

    /// C++: `libsumo::TraCIPositionVector Junction::getShape(const std::string &)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_shape(junctionID: &str) -> TraCIPositionVector {
        Helper::make_tra_ci_position_vector(Self::get_junction(junctionID).get_shape())
    }

    /// C++: `const std::vector<std::string> Junction::getIncomingEdges(const std::string &)` [static]
    // const std::vector<std::string>
    // Junction::getIncomingEdges(const std::string& junctionID) {
    //     std::vector<std::string> result;
    //     for (const MSEdge* edge : getJunction(junctionID)->getIncoming()) {
    //         result.push_back(edge->getID());
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_incoming_edges(junctionID: &str) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        for edge in Self::get_junction(junctionID).get_incoming() {
            result.push(edge.get_id().to_string());
        }
        result
    }

    /// C++: `const std::vector<std::string> Junction::getOutgoingEdges(const std::string &)` [static]
    // const std::vector<std::string>
    // Junction::getOutgoingEdges(const std::string& junctionID) {
    //     std::vector<std::string> result;
    //     for (const MSEdge* edge : getJunction(junctionID)->getOutgoing()) {
    //         result.push_back(edge->getID());
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_outgoing_edges(junctionID: &str) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        for edge in Self::get_junction(junctionID).get_outgoing() {
            result.push(edge.get_id().to_string());
        }
        result
    }

    /// C++: `std::vector<std::string> Junction::getIDList()` [static]
    // std::vector<std::string>
    // Junction::getIDList() {
    //     std::vector<std::string> ids;
    //     MSNet::getInstance()->getJunctionControl().insertIDs(ids);
    //     return ids;
    // }
    #[allow(unused_variables)]
    pub fn get_id_list() -> Vec<String> {
        // shortcut: MSNet junction control has no insertIDs yet (MSJunctionControl is a skeleton), upgrade when MSJunctionControl is ported
        panic!("shortcut: Junction::getIDList needs MSJunctionControl::insertIDs")
    }

    /// C++: `int Junction::getIDCount()` [static]
    // int
    // Junction::getIDCount() {
    //     return (int)getIDList().size();
    // }
    #[allow(unused_variables)]
    pub fn get_id_count() -> i32 {
        Self::get_id_list().len() as i32
    }

    /// C++: `std::string Junction::getParameter(const std::string &, const std::string &)` [static]
    // std::string
    // Junction::getParameter(const std::string& junctionID, const std::string& param) {
    //     return getJunction(junctionID)->getParameter(param, "");
    // }
    #[allow(unused_variables)]
    pub fn get_parameter(objectID: &str, key: &str) -> String {
        Self::get_junction(objectID).base2.get_parameter(key, "")
    }

    /// C++: `const std::pair<std::string, std::string> Junction::getParameterWithKey(const std::string &, const std::string &)` [static]
    // LIBSUMO_GET_PARAMETER_WITH_KEY_IMPLEMENTATION(Junction)
    #[allow(unused_variables)]
    pub fn get_parameter_with_key(objectID: &str, key: &str) -> (String, String) {
        (key.to_string(), Self::get_parameter(objectID, key))
    }

    /// C++: `void Junction::setParameter(const std::string &, const std::string &, const std::string &)` [static]
    // void
    // Junction::setParameter(const std::string& junctionID, const std::string& name, const std::string& value) {
    //     getJunction(junctionID)->setParameter(name, value);
    // }
    #[allow(unused_variables)]
    pub fn set_parameter(objectID: &str, key: &str, value: &str) {
        Self::get_junction(objectID).base2.set_parameter(key, value);
    }

    /// C++: `void Junction::subscribe(const std::string &, const std::vector<int> &, double, double, const libsumo::TraCIResults &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Junction, JUNCTION)
    #[allow(unused_variables)]
    pub fn subscribe(objectID: &str, varIDs: &Vec<i32>, begin: f64, end: f64, parameters: &TraCIResults) {
        Helper::subscribe(CMD_SUBSCRIBE_JUNCTION_VARIABLE, objectID, varIDs, begin, end, parameters, 0, 0.0);
    }

    /// C++: `void Junction::unsubscribe(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Junction, JUNCTION)
    #[allow(unused_variables)]
    pub fn unsubscribe(objectID: &str) {
        Helper::subscribe(CMD_SUBSCRIBE_JUNCTION_VARIABLE, objectID, &Vec::new(), INVALID_DOUBLE_VALUE, INVALID_DOUBLE_VALUE, &TraCIResults::new(), 0, 0.0);
    }

    /// C++: `void Junction::subscribeContext(const std::string &, int, double, const std::vector<int> &, double, double, const libsumo::TraCIResults &)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn subscribe_context(objectID: &str, domain: i32, dist: f64, varIDs: &Vec<i32>, begin: f64, end: f64, parameters: &TraCIResults) {
        Helper::subscribe(CMD_SUBSCRIBE_JUNCTION_CONTEXT, objectID, varIDs, begin, end, parameters, domain, dist);
    }

    /// C++: `void Junction::unsubscribeContext(const std::string &, int, double)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Junction, JUNCTION)
    #[allow(unused_variables)]
    pub fn unsubscribe_context(objectID: &str, domain: i32, dist: f64) {
        Helper::subscribe(CMD_SUBSCRIBE_JUNCTION_CONTEXT, objectID, &Vec::new(), INVALID_DOUBLE_VALUE, INVALID_DOUBLE_VALUE, &TraCIResults::new(), domain, dist);
    }

    /// C++: `const libsumo::SubscriptionResults Junction::getAllSubscriptionResults()` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Junction, JUNCTION)
    #[allow(unused_variables)]
    pub fn get_all_subscription_results() -> SubscriptionResults {
        MY_SUBSCRIPTION_RESULTS.with(|r| r.borrow().clone())
    }

    /// C++: `const libsumo::TraCIResults Junction::getSubscriptionResults(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Junction, JUNCTION)
    #[allow(unused_variables)]
    pub fn get_subscription_results(objectID: &str) -> TraCIResults {
        MY_SUBSCRIPTION_RESULTS.with(|r| r.borrow_mut().entry(objectID.to_string()).or_default().clone())
    }

    /// C++: `const libsumo::ContextSubscriptionResults Junction::getAllContextSubscriptionResults()` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Junction, JUNCTION)
    #[allow(unused_variables)]
    pub fn get_all_context_subscription_results() -> ContextSubscriptionResults {
        MY_CONTEXT_SUBSCRIPTION_RESULTS.with(|r| r.borrow().clone())
    }

    /// C++: `const libsumo::SubscriptionResults Junction::getContextSubscriptionResults(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Junction, JUNCTION)
    #[allow(unused_variables)]
    pub fn get_context_subscription_results(objectID: &str) -> SubscriptionResults {
        MY_CONTEXT_SUBSCRIPTION_RESULTS.with(|r| r.borrow_mut().entry(objectID.to_string()).or_default().clone())
    }

    /// C++: `void Junction::subscribeParameterWithKey(const std::string &, const std::string &, double, double)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Junction, JUNCTION)
    #[allow(unused_variables)]
    pub fn subscribe_parameter_with_key(objectID: &str, key: &str, beginTime: f64, endTime: f64) {
        // shortcut: TraCIString is a subclass wrapper (base: TraCIResult), so std::make_shared<TraCIString> cannot be stored as Rc<TraCIResult> yet, upgrade with the TraCIResult class hierarchy
        panic!("shortcut: Junction::subscribeParameterWithKey needs TraCIString as Rc<TraCIResult>")
    }

    /// C++: `int Junction::domainID()` [static]
    // LIBSUMO_SUBSCRIPTION_API
    #[allow(unused_variables)]
    pub fn domain_id() -> i32 {
        CMD_GET_JUNCTION_VARIABLE
    }

    /// C++: `void Junction::storeShape(const std::string &, PositionVector &)` [static]
    /// Saves the shape of the requested object in the given container
    // void
    // Junction::storeShape(const std::string& id, PositionVector& shape) {
    //     shape.push_back(getJunction(id)->getPosition());
    // }
    #[allow(unused_variables)]
    pub fn store_shape(id: &str, shape: &mut PositionVector) {
        shape.push(Self::get_junction(id).get_position(false).clone());
    }

    /// C++: `NamedRTree * Junction::getTree()` [static]
    /// Returns a tree filled with junction instances
    // NamedRTree*
    // Junction::getTree() {
    //     if (myTree == nullptr) {
    //         myTree = new NamedRTree();
    //         for (const auto& i : MSNet::getInstance()->getJunctionControl()) {
    //             Boundary b = i.second->getShape().getBoxBoundary();
    //             const float cmin[2] = {(float) b.xmin(), (float) b.ymin()};
    //             const float cmax[2] = {(float) b.xmax(), (float) b.ymax()};
    //             myTree->Insert(cmin, cmax, i.second);
    //         }
    //     }
    //     return myTree;
    // }
    #[allow(unused_variables)]
    pub fn get_tree() -> Ptr<Unmapped /* NamedRTree */> {
        // shortcut: NamedRTree is not ported yet, upgrade when the R-tree type is mapped
        panic!("shortcut: Junction::getTree needs NamedRTree (unmapped)")
    }

    /// C++: `void Junction::cleanup()` [static]
    // void
    // Junction::cleanup() {
    //     delete myTree;
    //     myTree = nullptr;
    // }
    #[allow(unused_variables)]
    pub fn cleanup() {
        // shortcut: myTree (NamedRTree) is never built while getTree is a shortcut, so there is nothing to delete yet
    }

    /// C++: `std::shared_ptr<VariableWrapper> Junction::makeWrapper()` [static]
    // std::shared_ptr<VariableWrapper>
    // Junction::makeWrapper() {
    //     return std::make_shared<Helper::SubscriptionWrapper>(handleVariable, mySubscriptionResults, myContextSubscriptionResults);
    // }
    #[allow(unused_variables)]
    pub fn make_wrapper() -> Rc<VariableWrapper> {
        // shortcut: Helper::SubscriptionWrapper borrows the class-level subscription statics by reference, which thread_local RefCells cannot hand out yet
        panic!("shortcut: Junction::makeWrapper needs the subscription wrapper over class statics")
    }

    /// C++: `bool Junction::handleVariable(const std::string &, const int, VariableWrapper *, tcpip::Storage *)` [static]
    // bool
    // Junction::handleVariable(const std::string& objID, const int variable, VariableWrapper* wrapper, tcpip::Storage* paramData) {
    //     switch (variable) {
    //         case TRACI_ID_LIST:
    //             return wrapper->wrapStringList(objID, variable, getIDList());
    //         case ID_COUNT:
    //             return wrapper->wrapInt(objID, variable, getIDCount());
    //         case VAR_POSITION:
    //         case VAR_POSITION3D:
    //             return wrapper->wrapPosition(objID, variable, getPosition(objID, variable == VAR_POSITION3D));
    //         case VAR_SHAPE:
    //             return wrapper->wrapPositionVector(objID, variable, getShape(objID));
    //         case libsumo::VAR_PARAMETER:
    //             paramData->readUnsignedByte();
    //             return wrapper->wrapString(objID, variable, getParameter(objID, paramData->readString()));
    //         case INCOMING_EDGES:
    //             return wrapper->wrapStringList(objID, variable, getIncomingEdges(objID));
    //         case OUTGOING_EDGES:
    //             return wrapper->wrapStringList(objID, variable, getOutgoingEdges(objID));
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
            VAR_POSITION | VAR_POSITION3D => wrapper.m().wrap_position(objID, variable, &Self::get_position(objID, variable == VAR_POSITION3D)),
            VAR_SHAPE => wrapper.m().wrap_position_vector(objID, variable, &Self::get_shape(objID)),
            VAR_PARAMETER => {
                paramData.m().read_unsigned_byte();
                // shortcut: C++ readString() throws ProcessError on malformed storage, here it panics, upgrade when handle_variable returns Result
                let key = paramData.m().read_string().expect("shortcut: malformed TraCI parameter string");
                wrapper.m().wrap_string(objID, variable, &Self::get_parameter(objID, &key))
            }
            INCOMING_EDGES => wrapper.m().wrap_string_list(objID, variable, &Self::get_incoming_edges(objID)),
            OUTGOING_EDGES => wrapper.m().wrap_string_list(objID, variable, &Self::get_outgoing_edges(objID)),
            VAR_PARAMETER_WITH_KEY => {
                paramData.m().read_unsigned_byte();
                // shortcut: C++ readString() throws ProcessError on malformed storage, here it panics, upgrade when handle_variable returns Result
                let key = paramData.m().read_string().expect("shortcut: malformed TraCI parameter string");
                wrapper.m().wrap_string_pair(objID, variable, &Self::get_parameter_with_key(objID, &key))
            }
            _ => false,
        }
    }

    /// C++: `MSJunction * Junction::getJunction(const std::string &)` [static, private]
    // MSJunction*
    // Junction::getJunction(const std::string& id) {
    //     MSJunction* j = MSNet::getInstance()->getJunctionControl().get(id);
    //     if (j == nullptr) {
    //         throw TraCIException("Junction '" + id + "' is not known");
    //     }
    //     return j;
    // }
    #[allow(unused_variables)]
    pub fn get_junction(id: &str) -> Ptr<MSJunction> {
        // shortcut: MSJunctionControl::get is not mapped yet, and C++ throws TraCIException for unknown ids (here it panics), upgrade when MSJunctionControl is ported
        panic!("shortcut: Junction '{id}' lookup needs MSJunctionControl::get")
    }

    /// C++: `void Junction::Junction()` [private]
    /// invalidated standard constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new() -> Self {
        panic!("libsumo::Junction cannot be constructed: the standard constructor is invalidated in C++")
    }

}

