//! Port of libsumo/Polygon.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `libsumo::Polygon` (libsumo/Polygon.h)
pub struct Polygon {
}

// C++ static members of Polygon (port as module-level statics, see AGENTS.md):
//   static const int Polygon::DOMAIN_ID;
//   static SubscriptionResults Polygon::mySubscriptionResults;
//   static ContextSubscriptionResults Polygon::myContextSubscriptionResults;
//   static NamedRTree * Polygon::myTree;

impl Polygon {
    /// C++: `std::string Polygon::getType(const std::string &)` [static]
    // std::string
    // Polygon::getType(const std::string& polygonID) {
    //     return getPolygon(polygonID)->getShapeType();
    // }
    #[allow(unused_variables)]
    pub fn get_type(polygonID: &str) -> String {
        todo!("port Polygon::getType")
    }

    /// C++: `libsumo::TraCIPositionVector Polygon::getShape(const std::string &)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_shape(polygonID: &str) -> TraCIPositionVector {
        todo!("port Polygon::getShape")
    }

    /// C++: `libsumo::TraCIColor Polygon::getColor(const std::string &)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_color(polygonID: &str) -> TraCIColor {
        todo!("port Polygon::getColor")
    }

    /// C++: `bool Polygon::getFilled(const std::string &)` [static]
    // bool
    // Polygon::getFilled(const std::string& polygonID) {
    //     return getPolygon(polygonID)->getFill();
    // }
    #[allow(unused_variables)]
    pub fn get_filled(polygonID: &str) -> bool {
        todo!("port Polygon::getFilled")
    }

    /// C++: `double Polygon::getLineWidth(const std::string &)` [static]
    // double
    // Polygon::getLineWidth(const std::string& polygonID) {
    //     return getPolygon(polygonID)->getLineWidth();
    // }
    #[allow(unused_variables)]
    pub fn get_line_width(polygonID: &str) -> f64 {
        todo!("port Polygon::getLineWidth")
    }

    /// C++: `double Polygon::getHeight(const std::string &)` [static]
    // double
    // Polygon::getHeight(const std::string& polygonID) {
    //     return getPolygon(polygonID)->getHeight();
    // }
    #[allow(unused_variables)]
    pub fn get_height(polygonID: &str) -> f64 {
        todo!("port Polygon::getHeight")
    }

    /// C++: `std::vector<std::string> Polygon::getIDList()` [static]
    // std::vector<std::string>
    // Polygon::getIDList() {
    //     std::vector<std::string> ids;
    //     MSNet::getInstance()->getShapeContainer().getPolygons().insertIDs(ids);
    //     return ids;
    // }
    #[allow(unused_variables)]
    pub fn get_id_list() -> Vec<String> {
        todo!("port Polygon::getIDList")
    }

    /// C++: `int Polygon::getIDCount()` [static]
    // int
    // Polygon::getIDCount() {
    //     return (int)getIDList().size();
    // }
    #[allow(unused_variables)]
    pub fn get_id_count() -> i32 {
        todo!("port Polygon::getIDCount")
    }

    /// C++: `std::string Polygon::getParameter(const std::string &, const std::string &)` [static]
    // std::string
    // Polygon::getParameter(const std::string& polygonID, const std::string& key) {
    //     return getPolygon(polygonID)->getParameter(key, "");
    // }
    #[allow(unused_variables)]
    pub fn get_parameter(objectID: &str, key: &str) -> String {
        todo!("port Polygon::getParameter")
    }

    /// C++: `const std::pair<std::string, std::string> Polygon::getParameterWithKey(const std::string &, const std::string &)` [static]
    // LIBSUMO_GET_PARAMETER_WITH_KEY_IMPLEMENTATION(Polygon)
    #[allow(unused_variables)]
    pub fn get_parameter_with_key(objectID: &str, key: &str) -> (String, String) {
        todo!("port Polygon::getParameterWithKey")
    }

    /// C++: `void Polygon::setParameter(const std::string &, const std::string &, const std::string &)` [static]
    // void
    // Polygon::setParameter(const std::string& polygonID, const std::string& key, const std::string& value) {
    //     SUMOPolygon* p = getPolygon(polygonID);
    //     p->setParameter(key, value);
    // }
    #[allow(unused_variables)]
    pub fn set_parameter(objectID: &str, key: &str, value: &str) {
        todo!("port Polygon::setParameter")
    }

    /// C++: `void Polygon::subscribe(const std::string &, const std::vector<int> &, double, double, const libsumo::TraCIResults &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Polygon, POLYGON)
    #[allow(unused_variables)]
    pub fn subscribe(objectID: &str, varIDs: &Vec<i32>, begin: f64, end: f64, parameters: &TraCIResults) {
        todo!("port Polygon::subscribe")
    }

    /// C++: `void Polygon::unsubscribe(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Polygon, POLYGON)
    #[allow(unused_variables)]
    pub fn unsubscribe(objectID: &str) {
        todo!("port Polygon::unsubscribe")
    }

    /// C++: `void Polygon::subscribeContext(const std::string &, int, double, const std::vector<int> &, double, double, const libsumo::TraCIResults &)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn subscribe_context(objectID: &str, domain: i32, dist: f64, varIDs: &Vec<i32>, begin: f64, end: f64, parameters: &TraCIResults) {
        todo!("port Polygon::subscribeContext")
    }

    /// C++: `void Polygon::unsubscribeContext(const std::string &, int, double)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Polygon, POLYGON)
    #[allow(unused_variables)]
    pub fn unsubscribe_context(objectID: &str, domain: i32, dist: f64) {
        todo!("port Polygon::unsubscribeContext")
    }

    /// C++: `const libsumo::SubscriptionResults Polygon::getAllSubscriptionResults()` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Polygon, POLYGON)
    #[allow(unused_variables)]
    pub fn get_all_subscription_results() -> SubscriptionResults {
        todo!("port Polygon::getAllSubscriptionResults")
    }

    /// C++: `const libsumo::TraCIResults Polygon::getSubscriptionResults(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Polygon, POLYGON)
    #[allow(unused_variables)]
    pub fn get_subscription_results(objectID: &str) -> TraCIResults {
        todo!("port Polygon::getSubscriptionResults")
    }

    /// C++: `const libsumo::ContextSubscriptionResults Polygon::getAllContextSubscriptionResults()` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Polygon, POLYGON)
    #[allow(unused_variables)]
    pub fn get_all_context_subscription_results() -> ContextSubscriptionResults {
        todo!("port Polygon::getAllContextSubscriptionResults")
    }

    /// C++: `const libsumo::SubscriptionResults Polygon::getContextSubscriptionResults(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Polygon, POLYGON)
    #[allow(unused_variables)]
    pub fn get_context_subscription_results(objectID: &str) -> SubscriptionResults {
        todo!("port Polygon::getContextSubscriptionResults")
    }

    /// C++: `void Polygon::subscribeParameterWithKey(const std::string &, const std::string &, double, double)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Polygon, POLYGON)
    #[allow(unused_variables)]
    pub fn subscribe_parameter_with_key(objectID: &str, key: &str, beginTime: f64, endTime: f64) {
        todo!("port Polygon::subscribeParameterWithKey")
    }

    /// C++: `int Polygon::domainID()` [static]
    // LIBSUMO_SUBSCRIPTION_API
    #[allow(unused_variables)]
    pub fn domain_id() -> i32 {
        todo!("port Polygon::domainID")
    }

    /// C++: `void Polygon::setType(const std::string &, const std::string &)` [static]
    // void
    // Polygon::setType(const std::string& polygonID, const std::string& polygonType) {
    //     getPolygon(polygonID)->setShapeType(polygonType);
    // }
    #[allow(unused_variables)]
    pub fn set_type(polygonID: &str, polygonType: &str) {
        todo!("port Polygon::setType")
    }

    /// C++: `void Polygon::setShape(const std::string &, const libsumo::TraCIPositionVector &)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn set_shape(polygonID: &str, shape: &TraCIPositionVector) {
        todo!("port Polygon::setShape")
    }

    /// C++: `void Polygon::setColor(const std::string &, const libsumo::TraCIColor &)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn set_color(polygonID: &str, color: &TraCIColor) {
        todo!("port Polygon::setColor")
    }

    /// C++: `void Polygon::add(const std::string &, const libsumo::TraCIPositionVector &, const libsumo::TraCIColor &, bool, const std::string &, int, double)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn add(polygonID: &str, shape: &TraCIPositionVector, color: &TraCIColor, fill: bool, polygonType: &str, layer: i32, lineWidth: f64) {
        todo!("port Polygon::add")
    }

    /// C++: `void Polygon::addDynamics(const std::string &, const std::string &, const std::vector<double> &, const std::vector<double> &, bool, bool)` [static]
    // void
    // Polygon::addDynamics(const std::string& polygonID, const std::string& trackedObjectID, const std::vector<double>& timeSpan, const std::vector<double>& alphaSpan, bool looped, bool rotate) {
    //     if (timeSpan.empty()) {
    //         if (trackedObjectID == "") {
    //             throw TraCIException("Could not add polygon dynamics for polygon '" + polygonID + "': dynamics underspecified (either a tracked object ID or a time span have to be provided).");
    //         }
    //         if (looped) {
    //             throw TraCIException("Could not add polygon dynamics for polygon '" + polygonID + "': looped==true requires time line of positive length.");
    //         }
    //     }
    //     if (timeSpan.size() == 1) {
    //         throw TraCIException("Could not add polygon dynamics for polygon '" + polygonID + "': time span cannot have length one.");
    //     } else if (timeSpan.size() > 0 && timeSpan[0] != 0.0) {
    //         throw TraCIException("Could not add polygon dynamics for polygon '" + polygonID + "': first element of time span must be zero.");
    //     }
    //     if (timeSpan.size() != alphaSpan.size() && alphaSpan.size() != 0) {
    //         throw TraCIException("Could not add polygon dynamics for polygon '" + polygonID + "': alpha span must have length zero or equal to time span length.");
    //     }
    //     if (timeSpan.size() >= 2) {
    //         for (unsigned int i = 1; i < timeSpan.size(); ++i) {
    //             if (timeSpan[i - 1] > timeSpan[i]) {
    //                 throw TraCIException("Could not add polygon dynamics for polygon '" + polygonID + "': entries of time span must be ordered ascendingly.");
    //             }
    //         }
    //     }
    //
    //     SUMOTrafficObject* obj = getTrafficObject(trackedObjectID);
    //     ShapeContainer& shapeCont = MSNet::getInstance()->getShapeContainer();
    //     PolygonDynamics* pd = shapeCont.addPolygonDynamics(SIMTIME, polygonID, obj, timeSpan, alphaSpan, looped, rotate);
    //     if (pd == nullptr) {
    //         throw TraCIException("Could not add polygon dynamics for polygon '" + polygonID + "': polygon doesn't exist.");
    //     }
    //     // Ensure existence of a DynamicShapeUpdater
    //     if (MSNet::getInstance()->getDynamicShapeUpdater() == nullptr) {
    //         MSNet::VehicleStateListener* listener = dynamic_cast<MSNet::VehicleStateListener*>(MSNet::getInstance()->makeDynamicShapeUpdater());
    //         MSNet::getInstance()->addVehicleStateListener(listener);
    //     }
    //
    //     // Schedule the regular polygon update
    //     auto cmd = new ParametrisedWrappingCommand<ShapeContainer, PolygonDynamics*>(&shapeCont, pd, &ShapeContainer::polygonDynamicsUpdate);
    //     shapeCont.addPolygonUpdateCommand(pd->getPolygonID(), cmd);
    //     MSNet::getInstance()->getEndOfTimestepEvents()->addEvent(cmd, SIMSTEP);
    // }
    #[allow(unused_variables)]
    pub fn add_dynamics(polygonID: &str, trackedObjectID: &str, timeSpan: &Vec<f64>, alphaSpan: &Vec<f64>, looped: bool, rotate: bool) {
        todo!("port Polygon::addDynamics")
    }

    /// C++: `void Polygon::remove(const std::string &, int)` [static]
    // void
    // Polygon::remove(const std::string& polygonID, int /* layer */) {
    //     // !!! layer not used yet (shouldn't the id be enough?)
    //     ShapeContainer& shapeCont = MSNet::getInstance()->getShapeContainer();
    //     if (myTree != nullptr) {
    //         SUMOPolygon* p = shapeCont.getPolygons().get(polygonID);
    //         if (p != nullptr) {
    //             Boundary b = p->getShape().getBoxBoundary();
    //             const float cmin[2] = {(float) b.xmin(), (float) b.ymin()};
    //             const float cmax[2] = {(float) b.xmax(), (float) b.ymax()};
    //             myTree->Remove(cmin, cmax, p);
    //         }
    //     }
    //     if (!shapeCont.removePolygon(polygonID)) {
    //         throw TraCIException("Could not remove polygon '" + polygonID + "'");
    //     }
    // }
    #[allow(unused_variables)]
    pub fn remove(polygonID: &str, layer: i32) {
        todo!("port Polygon::remove")
    }

    /// C++: `void Polygon::setFilled(std::string, bool)` [static]
    // void
    // Polygon::setFilled(std::string polygonID, bool filled) {
    //     getPolygon(polygonID)->setFill(filled);
    // }
    #[allow(unused_variables)]
    pub fn set_filled(polygonID: String, filled: bool) {
        todo!("port Polygon::setFilled")
    }

    /// C++: `void Polygon::setLineWidth(std::string, double)` [static]
    // void
    // Polygon::setLineWidth(std::string polygonID, double lineWidth) {
    //     getPolygon(polygonID)->setLineWidth(lineWidth);
    // }
    #[allow(unused_variables)]
    pub fn set_line_width(polygonID: String, lineWidth: f64) {
        todo!("port Polygon::setLineWidth")
    }

    /// C++: `void Polygon::setHeight(std::string, double)` [static]
    // void
    // Polygon::setHeight(std::string polygonID, double height) {
    //     getPolygon(polygonID)->setHeight(height);
    // }
    #[allow(unused_variables)]
    pub fn set_height(polygonID: String, height: f64) {
        todo!("port Polygon::setHeight")
    }

    /// C++: `void Polygon::addHighlightPolygon(const std::string &, const int, const std::string &, const libsumo::TraCIPositionVector &, const libsumo::TraCIColor &, bool, const std::string &, int, double)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn add_highlight_polygon(objectID: &str, type_: i32, polygonID: &str, shape: &TraCIPositionVector, color: &TraCIColor, fill: bool, polygonType: &str, layer: i32, lineWidth: f64) {
        todo!("port Polygon::addHighlightPolygon")
    }

    /// C++: `NamedRTree * Polygon::getTree()` [static]
    /// Returns a tree filled with polygon instances
    // NamedRTree*
    // Polygon::getTree() {
    //     if (myTree == nullptr) {
    //         myTree = new NamedRTree();
    //         ShapeContainer& shapeCont = MSNet::getInstance()->getShapeContainer();
    //         for (const auto& i : shapeCont.getPolygons()) {
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
        todo!("port Polygon::getTree")
    }

    /// C++: `void Polygon::cleanup()` [static]
    // void
    // Polygon::cleanup() {
    //     delete myTree;
    //     myTree = nullptr;
    // }
    #[allow(unused_variables)]
    pub fn cleanup() {
        todo!("port Polygon::cleanup")
    }

    /// C++: `void Polygon::storeShape(const std::string &, PositionVector &)` [static]
    /// Saves the shape of the requested object in the given container
    // void
    // Polygon::storeShape(const std::string& id, PositionVector& shape) {
    //     shape = getPolygon(id)->getShape();
    // }
    #[allow(unused_variables)]
    pub fn store_shape(id: &str, shape: &mut PositionVector) {
        todo!("port Polygon::storeShape")
    }

    /// C++: `std::shared_ptr<VariableWrapper> Polygon::makeWrapper()` [static]
    // std::shared_ptr<VariableWrapper>
    // Polygon::makeWrapper() {
    //     return std::make_shared<Helper::SubscriptionWrapper>(handleVariable, mySubscriptionResults, myContextSubscriptionResults);
    // }
    #[allow(unused_variables)]
    pub fn make_wrapper() -> Rc<VariableWrapper> {
        todo!("port Polygon::makeWrapper")
    }

    /// C++: `bool Polygon::handleVariable(const std::string &, const int, VariableWrapper *, tcpip::Storage *)` [static]
    // bool
    // Polygon::handleVariable(const std::string& objID, const int variable, VariableWrapper* wrapper, tcpip::Storage* paramData) {
    //     switch (variable) {
    //         case TRACI_ID_LIST:
    //             return wrapper->wrapStringList(objID, variable, getIDList());
    //         case ID_COUNT:
    //             return wrapper->wrapInt(objID, variable, getIDCount());
    //         case VAR_TYPE:
    //             return wrapper->wrapString(objID, variable, getType(objID));
    //         case VAR_COLOR:
    //             return wrapper->wrapColor(objID, variable, getColor(objID));
    //         case VAR_FILL:
    //             return wrapper->wrapInt(objID, variable, getFilled(objID));
    //         case VAR_WIDTH:
    //             return wrapper->wrapDouble(objID, variable, getLineWidth(objID));
    //         case VAR_HEIGHT:
    //             return wrapper->wrapDouble(objID, variable, getHeight(objID));
    //         case VAR_SHAPE:
    //             return wrapper->wrapPositionVector(objID, variable, getShape(objID));
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
        todo!("port Polygon::handleVariable")
    }

    /// C++: `bool Polygon::exists(std::string)` [static]
    /// Checks if a polygon of the given name exists already in the simulation
    // bool
    // Polygon::exists(std::string polyID) {
    //     SUMOPolygon* p = MSNet::getInstance()->getShapeContainer().getPolygons().get(polyID);
    //     return p != nullptr;
    // }
    #[allow(unused_variables)]
    pub fn exists(polyID: String) -> bool {
        todo!("port Polygon::exists")
    }

    /// C++: `SUMOPolygon * Polygon::getPolygon(const std::string &)` [static, private]
    // SUMOPolygon*
    // Polygon::getPolygon(const std::string& id) {
    //     SUMOPolygon* p = MSNet::getInstance()->getShapeContainer().getPolygons().get(id);
    //     if (p == nullptr) {
    //         throw TraCIException("Polygon '" + id + "' is not known");
    //     }
    //     return p;
    // }
    #[allow(unused_variables)]
    pub fn get_polygon(id: &str) -> Ptr<SUMOPolygon> {
        todo!("port Polygon::getPolygon")
    }

    /// C++: `SUMOTrafficObject * Polygon::getTrafficObject(const std::string &)` [static, private]
    /// Obtain a traffic object with the given id if one exists
    // SUMOTrafficObject*
    // Polygon::getTrafficObject(const std::string& id) {
    //     if (id == "") {
    //         return nullptr;
    //     }
    //     MSNet* net = MSNet::getInstance();
    //     // First try to find a vehicle with the given id
    //     SUMOVehicle* sumoVehicle = net->getVehicleControl().getVehicle(id);
    //     if (sumoVehicle != nullptr) {
    //         return static_cast<SUMOTrafficObject*>(sumoVehicle);
    //     }
    //     MSTransportable* transportable = net->getPersonControl().get(id);
    //     if (transportable != nullptr) {
    //         return static_cast<SUMOTrafficObject*>(transportable);
    //     } else {
    //         throw TraCIException("Traffic object '" + id + "' is not known");
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_traffic_object(id: &str) -> Ptr<SUMOTrafficObject> {
        todo!("port Polygon::getTrafficObject")
    }

    /// C++: `void Polygon::Polygon()` [private]
    /// invalidated standard constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new() -> Self {
        todo!("port Polygon::Polygon")
    }

}

