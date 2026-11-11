//! Port of libsumo/Junction.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `libsumo::Junction` (libsumo/Junction.h)
pub struct Junction {
}

// C++ static members of Junction (port as module-level statics, see AGENTS.md):
//   static const int Junction::DOMAIN_ID;
//   static SubscriptionResults Junction::mySubscriptionResults;
//   static ContextSubscriptionResults Junction::myContextSubscriptionResults;
//   static NamedRTree * Junction::myTree;

impl Junction {
    /// C++: `libsumo::TraCIPosition Junction::getPosition(const std::string &, bool)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_position(junctionID: &str, includeZ: bool) -> TraCIPosition {
        todo!("port Junction::getPosition")
    }

    /// C++: `libsumo::TraCIPositionVector Junction::getShape(const std::string &)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_shape(junctionID: &str) -> TraCIPositionVector {
        todo!("port Junction::getShape")
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
        todo!("port Junction::getIncomingEdges")
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
        todo!("port Junction::getOutgoingEdges")
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
        todo!("port Junction::getIDList")
    }

    /// C++: `int Junction::getIDCount()` [static]
    // int
    // Junction::getIDCount() {
    //     return (int)getIDList().size();
    // }
    #[allow(unused_variables)]
    pub fn get_id_count() -> i32 {
        todo!("port Junction::getIDCount")
    }

    /// C++: `std::string Junction::getParameter(const std::string &, const std::string &)` [static]
    // std::string
    // Junction::getParameter(const std::string& junctionID, const std::string& param) {
    //     return getJunction(junctionID)->getParameter(param, "");
    // }
    #[allow(unused_variables)]
    pub fn get_parameter(objectID: &str, key: &str) -> String {
        todo!("port Junction::getParameter")
    }

    /// C++: `const std::pair<std::string, std::string> Junction::getParameterWithKey(const std::string &, const std::string &)` [static]
    // LIBSUMO_GET_PARAMETER_WITH_KEY_IMPLEMENTATION(Junction)
    #[allow(unused_variables)]
    pub fn get_parameter_with_key(objectID: &str, key: &str) -> (String, String) {
        todo!("port Junction::getParameterWithKey")
    }

    /// C++: `void Junction::setParameter(const std::string &, const std::string &, const std::string &)` [static]
    // void
    // Junction::setParameter(const std::string& junctionID, const std::string& name, const std::string& value) {
    //     getJunction(junctionID)->setParameter(name, value);
    // }
    #[allow(unused_variables)]
    pub fn set_parameter(objectID: &str, key: &str, value: &str) {
        todo!("port Junction::setParameter")
    }

    /// C++: `void Junction::subscribe(const std::string &, const std::vector<int> &, double, double, const libsumo::TraCIResults &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Junction, JUNCTION)
    #[allow(unused_variables)]
    pub fn subscribe(objectID: &str, varIDs: &Vec<i32>, begin: f64, end: f64, parameters: &TraCIResults) {
        todo!("port Junction::subscribe")
    }

    /// C++: `void Junction::unsubscribe(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Junction, JUNCTION)
    #[allow(unused_variables)]
    pub fn unsubscribe(objectID: &str) {
        todo!("port Junction::unsubscribe")
    }

    /// C++: `void Junction::subscribeContext(const std::string &, int, double, const std::vector<int> &, double, double, const libsumo::TraCIResults &)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn subscribe_context(objectID: &str, domain: i32, dist: f64, varIDs: &Vec<i32>, begin: f64, end: f64, parameters: &TraCIResults) {
        todo!("port Junction::subscribeContext")
    }

    /// C++: `void Junction::unsubscribeContext(const std::string &, int, double)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Junction, JUNCTION)
    #[allow(unused_variables)]
    pub fn unsubscribe_context(objectID: &str, domain: i32, dist: f64) {
        todo!("port Junction::unsubscribeContext")
    }

    /// C++: `const libsumo::SubscriptionResults Junction::getAllSubscriptionResults()` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Junction, JUNCTION)
    #[allow(unused_variables)]
    pub fn get_all_subscription_results() -> SubscriptionResults {
        todo!("port Junction::getAllSubscriptionResults")
    }

    /// C++: `const libsumo::TraCIResults Junction::getSubscriptionResults(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Junction, JUNCTION)
    #[allow(unused_variables)]
    pub fn get_subscription_results(objectID: &str) -> TraCIResults {
        todo!("port Junction::getSubscriptionResults")
    }

    /// C++: `const libsumo::ContextSubscriptionResults Junction::getAllContextSubscriptionResults()` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Junction, JUNCTION)
    #[allow(unused_variables)]
    pub fn get_all_context_subscription_results() -> ContextSubscriptionResults {
        todo!("port Junction::getAllContextSubscriptionResults")
    }

    /// C++: `const libsumo::SubscriptionResults Junction::getContextSubscriptionResults(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Junction, JUNCTION)
    #[allow(unused_variables)]
    pub fn get_context_subscription_results(objectID: &str) -> SubscriptionResults {
        todo!("port Junction::getContextSubscriptionResults")
    }

    /// C++: `void Junction::subscribeParameterWithKey(const std::string &, const std::string &, double, double)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Junction, JUNCTION)
    #[allow(unused_variables)]
    pub fn subscribe_parameter_with_key(objectID: &str, key: &str, beginTime: f64, endTime: f64) {
        todo!("port Junction::subscribeParameterWithKey")
    }

    /// C++: `int Junction::domainID()` [static]
    // LIBSUMO_SUBSCRIPTION_API
    #[allow(unused_variables)]
    pub fn domain_id() -> i32 {
        todo!("port Junction::domainID")
    }

    /// C++: `void Junction::storeShape(const std::string &, PositionVector &)` [static]
    /// Saves the shape of the requested object in the given container
    // void
    // Junction::storeShape(const std::string& id, PositionVector& shape) {
    //     shape.push_back(getJunction(id)->getPosition());
    // }
    #[allow(unused_variables)]
    pub fn store_shape(id: &str, shape: &mut PositionVector) {
        todo!("port Junction::storeShape")
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
        todo!("port Junction::getTree")
    }

    /// C++: `void Junction::cleanup()` [static]
    // void
    // Junction::cleanup() {
    //     delete myTree;
    //     myTree = nullptr;
    // }
    #[allow(unused_variables)]
    pub fn cleanup() {
        todo!("port Junction::cleanup")
    }

    /// C++: `std::shared_ptr<VariableWrapper> Junction::makeWrapper()` [static]
    // std::shared_ptr<VariableWrapper>
    // Junction::makeWrapper() {
    //     return std::make_shared<Helper::SubscriptionWrapper>(handleVariable, mySubscriptionResults, myContextSubscriptionResults);
    // }
    #[allow(unused_variables)]
    pub fn make_wrapper() -> Rc<VariableWrapper> {
        todo!("port Junction::makeWrapper")
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
        todo!("port Junction::handleVariable")
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
        todo!("port Junction::getJunction")
    }

    /// C++: `void Junction::Junction()` [private]
    /// invalidated standard constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new() -> Self {
        todo!("port Junction::Junction")
    }

}

