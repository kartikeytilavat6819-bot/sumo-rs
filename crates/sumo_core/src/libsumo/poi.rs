//! Port of libsumo/POI.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `libsumo::POI` (libsumo/POI.h)
pub struct POI {
}

// C++ static members of POI (port as module-level statics, see AGENTS.md):
//   static const int POI::DOMAIN_ID;
//   static SubscriptionResults POI::mySubscriptionResults;
//   static ContextSubscriptionResults POI::myContextSubscriptionResults;
//   static NamedRTree * POI::myTree;

impl POI {
    /// C++: `std::string POI::getType(const std::string &)` [static]
    // std::string
    // POI::getType(const std::string& poiID) {
    //     return getPoI(poiID)->getShapeType();
    // }
    #[allow(unused_variables)]
    pub fn get_type(poiID: &str) -> String {
        todo!("port POI::getType")
    }

    /// C++: `libsumo::TraCIPosition POI::getPosition(const std::string &, const bool)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_position(poiID: &str, includeZ: bool) -> TraCIPosition {
        todo!("port POI::getPosition")
    }

    /// C++: `libsumo::TraCIColor POI::getColor(const std::string &)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_color(poiID: &str) -> TraCIColor {
        todo!("port POI::getColor")
    }

    /// C++: `double POI::getWidth(const std::string &)` [static]
    // double
    // POI::getWidth(const std::string& poiID) {
    //     return getPoI(poiID)->getWidth();
    // }
    #[allow(unused_variables)]
    pub fn get_width(poiID: &str) -> f64 {
        todo!("port POI::getWidth")
    }

    /// C++: `double POI::getHeight(const std::string &)` [static]
    // double
    // POI::getHeight(const std::string& poiID) {
    //     return getPoI(poiID)->getHeight();
    // }
    #[allow(unused_variables)]
    pub fn get_height(poiID: &str) -> f64 {
        todo!("port POI::getHeight")
    }

    /// C++: `double POI::getAngle(const std::string &)` [static]
    // double
    // POI::getAngle(const std::string& poiID) {
    //     return getPoI(poiID)->getShapeNaviDegree();
    // }
    #[allow(unused_variables)]
    pub fn get_angle(poiID: &str) -> f64 {
        todo!("port POI::getAngle")
    }

    /// C++: `std::string POI::getImageFile(const std::string &)` [static]
    // std::string
    // POI::getImageFile(const std::string& poiID) {
    //     return getPoI(poiID)->getShapeImgFile();
    // }
    #[allow(unused_variables)]
    pub fn get_image_file(poiID: &str) -> String {
        todo!("port POI::getImageFile")
    }

    /// C++: `std::vector<std::string> POI::getIDList()` [static]
    // std::vector<std::string>
    // POI::getIDList() {
    //     std::vector<std::string> ids;
    //     MSNet::getInstance()->getShapeContainer().getPOIs().insertIDs(ids);
    //     return ids;
    // }
    #[allow(unused_variables)]
    pub fn get_id_list() -> Vec<String> {
        todo!("port POI::getIDList")
    }

    /// C++: `int POI::getIDCount()` [static]
    // int
    // POI::getIDCount() {
    //     return (int)getIDList().size();
    // }
    #[allow(unused_variables)]
    pub fn get_id_count() -> i32 {
        todo!("port POI::getIDCount")
    }

    /// C++: `std::string POI::getParameter(const std::string &, const std::string &)` [static]
    // std::string
    // POI::getParameter(const std::string& poiID, const std::string& key) {
    //     return getPoI(poiID)->getParameter(key, "");
    // }
    #[allow(unused_variables)]
    pub fn get_parameter(objectID: &str, key: &str) -> String {
        todo!("port POI::getParameter")
    }

    /// C++: `const std::pair<std::string, std::string> POI::getParameterWithKey(const std::string &, const std::string &)` [static]
    // LIBSUMO_GET_PARAMETER_WITH_KEY_IMPLEMENTATION(POI)
    #[allow(unused_variables)]
    pub fn get_parameter_with_key(objectID: &str, key: &str) -> (String, String) {
        todo!("port POI::getParameterWithKey")
    }

    /// C++: `void POI::setParameter(const std::string &, const std::string &, const std::string &)` [static]
    // void
    // POI::setParameter(const std::string& poiID, const std::string& key, const std::string& value) {
    //     PointOfInterest* p = getPoI(poiID);
    //     p->setParameter(key, value);
    // }
    #[allow(unused_variables)]
    pub fn set_parameter(objectID: &str, key: &str, value: &str) {
        todo!("port POI::setParameter")
    }

    /// C++: `void POI::subscribe(const std::string &, const std::vector<int> &, double, double, const libsumo::TraCIResults &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(POI, POI)
    #[allow(unused_variables)]
    pub fn subscribe(objectID: &str, varIDs: &Vec<i32>, begin: f64, end: f64, parameters: &TraCIResults) {
        todo!("port POI::subscribe")
    }

    /// C++: `void POI::unsubscribe(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(POI, POI)
    #[allow(unused_variables)]
    pub fn unsubscribe(objectID: &str) {
        todo!("port POI::unsubscribe")
    }

    /// C++: `void POI::subscribeContext(const std::string &, int, double, const std::vector<int> &, double, double, const libsumo::TraCIResults &)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn subscribe_context(objectID: &str, domain: i32, dist: f64, varIDs: &Vec<i32>, begin: f64, end: f64, parameters: &TraCIResults) {
        todo!("port POI::subscribeContext")
    }

    /// C++: `void POI::unsubscribeContext(const std::string &, int, double)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(POI, POI)
    #[allow(unused_variables)]
    pub fn unsubscribe_context(objectID: &str, domain: i32, dist: f64) {
        todo!("port POI::unsubscribeContext")
    }

    /// C++: `const libsumo::SubscriptionResults POI::getAllSubscriptionResults()` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(POI, POI)
    #[allow(unused_variables)]
    pub fn get_all_subscription_results() -> SubscriptionResults {
        todo!("port POI::getAllSubscriptionResults")
    }

    /// C++: `const libsumo::TraCIResults POI::getSubscriptionResults(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(POI, POI)
    #[allow(unused_variables)]
    pub fn get_subscription_results(objectID: &str) -> TraCIResults {
        todo!("port POI::getSubscriptionResults")
    }

    /// C++: `const libsumo::ContextSubscriptionResults POI::getAllContextSubscriptionResults()` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(POI, POI)
    #[allow(unused_variables)]
    pub fn get_all_context_subscription_results() -> ContextSubscriptionResults {
        todo!("port POI::getAllContextSubscriptionResults")
    }

    /// C++: `const libsumo::SubscriptionResults POI::getContextSubscriptionResults(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(POI, POI)
    #[allow(unused_variables)]
    pub fn get_context_subscription_results(objectID: &str) -> SubscriptionResults {
        todo!("port POI::getContextSubscriptionResults")
    }

    /// C++: `void POI::subscribeParameterWithKey(const std::string &, const std::string &, double, double)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(POI, POI)
    #[allow(unused_variables)]
    pub fn subscribe_parameter_with_key(objectID: &str, key: &str, beginTime: f64, endTime: f64) {
        todo!("port POI::subscribeParameterWithKey")
    }

    /// C++: `int POI::domainID()` [static]
    // LIBSUMO_SUBSCRIPTION_API
    #[allow(unused_variables)]
    pub fn domain_id() -> i32 {
        todo!("port POI::domainID")
    }

    /// C++: `void POI::setType(const std::string &, const std::string &)` [static]
    // void
    // POI::setType(const std::string& poiID, const std::string& poiType) {
    //     getPoI(poiID)->setShapeType(poiType);
    // }
    #[allow(unused_variables)]
    pub fn set_type(poiID: &str, poiType: &str) {
        todo!("port POI::setType")
    }

    /// C++: `void POI::setColor(const std::string &, const libsumo::TraCIColor &)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn set_color(poiID: &str, color: &TraCIColor) {
        todo!("port POI::setColor")
    }

    /// C++: `void POI::setPosition(const std::string &, double, double)` [static]
    // void
    // POI::setPosition(const std::string& poiID, double x, double y) {
    //     // try to retrieve so that the correct error is generated for unknown poiIDs
    //     getPoI(poiID);
    //     MSNet::getInstance()->getShapeContainer().movePOI(poiID, Position(x, y));
    // }
    #[allow(unused_variables)]
    pub fn set_position(poiID: &str, x: f64, y: f64) {
        todo!("port POI::setPosition")
    }

    /// C++: `void POI::setWidth(const std::string &, double)` [static]
    // void
    // POI::setWidth(const std::string& poiID, double width) {
    //     getPoI(poiID)->setWidth(width);
    // }
    #[allow(unused_variables)]
    pub fn set_width(poiID: &str, width: f64) {
        todo!("port POI::setWidth")
    }

    /// C++: `void POI::setHeight(const std::string &, double)` [static]
    // void
    // POI::setHeight(const std::string& poiID, double height) {
    //     getPoI(poiID)->setHeight(height);
    // }
    #[allow(unused_variables)]
    pub fn set_height(poiID: &str, height: f64) {
        todo!("port POI::setHeight")
    }

    /// C++: `void POI::setAngle(const std::string &, double)` [static]
    // void
    // POI::setAngle(const std::string& poiID, double angle) {
    //     getPoI(poiID)->setShapeNaviDegree(angle);
    // }
    #[allow(unused_variables)]
    pub fn set_angle(poiID: &str, angle: f64) {
        todo!("port POI::setAngle")
    }

    /// C++: `void POI::setImageFile(const std::string &, const std::string &)` [static]
    // void
    // POI::setImageFile(const std::string& poiID, const std::string& imageFile) {
    //     getPoI(poiID)->setShapeImgFile(imageFile);
    // }
    #[allow(unused_variables)]
    pub fn set_image_file(poiID: &str, imageFile: &str) {
        todo!("port POI::setImageFile")
    }

    /// C++: `bool POI::add(const std::string &, double, double, const libsumo::TraCIColor &, const std::string &, int, const std::string &, double, double, double, const std::string &)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn add(poiID: &str, x: f64, y: f64, color: &TraCIColor, poiType: &str, layer: i32, imgFile: &str, width: f64, height: f64, angle: f64, icon: &str) -> bool {
        todo!("port POI::add")
    }

    /// C++: `bool POI::remove(const std::string &, int)` [static]
    // bool
    // POI::remove(const std::string& poiID, int /* layer */) {
    //     ShapeContainer& shapeCont = MSNet::getInstance()->getShapeContainer();
    //     PointOfInterest* p = shapeCont.getPOIs().get(poiID);
    //     if (p != nullptr && myTree != nullptr) {
    //         const float cmin[2] = {(float)p->x(), (float)p->y()};
    //         const float cmax[2] = {(float)p->x(), (float)p->y()};
    //         myTree->Remove(cmin, cmax, p);
    //     }
    //     return shapeCont.removePOI(poiID);
    // }
    #[allow(unused_variables)]
    pub fn remove(poiID: &str, layer: i32) -> bool {
        todo!("port POI::remove")
    }

    /// C++: `void POI::highlight(const std::string &, const libsumo::TraCIColor &, double, const int, const double, const int)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn highlight(poiID: &str, col: &TraCIColor, size: f64, alphaMax: i32, duration: f64, type_: i32) {
        todo!("port POI::highlight")
    }

    /// C++: `NamedRTree * POI::getTree()` [static]
    /// Returns a tree filled with PoI instances
    // NamedRTree*
    // POI::getTree() {
    //     if (myTree == nullptr) {
    //         myTree = new NamedRTree();
    //         ShapeContainer& shapeCont = MSNet::getInstance()->getShapeContainer();
    //         for (const auto& i : shapeCont.getPOIs()) {
    //             const float cmin[2] = {(float)i.second->x(), (float)i.second->y()};
    //             const float cmax[2] = {(float)i.second->x(), (float)i.second->y()};
    //             myTree->Insert(cmin, cmax, i.second);
    //         }
    //     }
    //     return myTree;
    // }
    #[allow(unused_variables)]
    pub fn get_tree() -> Ptr<Unmapped /* NamedRTree */> {
        todo!("port POI::getTree")
    }

    /// C++: `void POI::cleanup()` [static]
    // void
    // POI::cleanup() {
    //     delete myTree;
    //     myTree = nullptr;
    // }
    #[allow(unused_variables)]
    pub fn cleanup() {
        todo!("port POI::cleanup")
    }

    /// C++: `void POI::storeShape(const std::string &, PositionVector &)` [static]
    /// Saves the shape of the requested object in the given container
    // void
    // POI::storeShape(const std::string& id, PositionVector& shape) {
    //     shape.push_back(*getPoI(id));
    // }
    #[allow(unused_variables)]
    pub fn store_shape(id: &str, shape: &mut PositionVector) {
        todo!("port POI::storeShape")
    }

    /// C++: `std::shared_ptr<VariableWrapper> POI::makeWrapper()` [static]
    // std::shared_ptr<VariableWrapper>
    // POI::makeWrapper() {
    //     return std::make_shared<Helper::SubscriptionWrapper>(handleVariable, mySubscriptionResults, myContextSubscriptionResults);
    // }
    #[allow(unused_variables)]
    pub fn make_wrapper() -> Rc<VariableWrapper> {
        todo!("port POI::makeWrapper")
    }

    /// C++: `bool POI::handleVariable(const std::string &, const int, VariableWrapper *, tcpip::Storage *)` [static]
    // bool
    // POI::handleVariable(const std::string& objID, const int variable, VariableWrapper* wrapper, tcpip::Storage* paramData) {
    //     switch (variable) {
    //         case TRACI_ID_LIST:
    //             return wrapper->wrapStringList(objID, variable, getIDList());
    //         case ID_COUNT:
    //             return wrapper->wrapInt(objID, variable, getIDCount());
    //         case VAR_TYPE:
    //             return wrapper->wrapString(objID, variable, getType(objID));
    //         case VAR_COLOR:
    //             return wrapper->wrapColor(objID, variable, getColor(objID));
    //         case VAR_POSITION:
    //             return wrapper->wrapPosition(objID, variable, getPosition(objID));
    //         case VAR_POSITION3D:
    //             return wrapper->wrapPosition(objID, variable, getPosition(objID, true));
    //         case VAR_WIDTH:
    //             return wrapper->wrapDouble(objID, variable, getWidth(objID));
    //         case VAR_HEIGHT:
    //             return wrapper->wrapDouble(objID, variable, getHeight(objID));
    //         case VAR_ANGLE:
    //             return wrapper->wrapDouble(objID, variable, getAngle(objID));
    //         case VAR_IMAGEFILE:
    //             return wrapper->wrapString(objID, variable, getImageFile(objID));
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
        todo!("port POI::handleVariable")
    }

    /// C++: `PointOfInterest * POI::getPoI(const std::string &)` [static, private]
    // PointOfInterest*
    // POI::getPoI(const std::string& id) {
    //     PointOfInterest* sumoPoi = MSNet::getInstance()->getShapeContainer().getPOIs().get(id);
    //     if (sumoPoi == nullptr) {
    //         throw TraCIException("POI '" + id + "' is not known");
    //     }
    //     return sumoPoi;
    // }
    #[allow(unused_variables)]
    pub fn get_po_i(id: &str) -> Ptr<PointOfInterest> {
        todo!("port POI::getPoI")
    }

    /// C++: `void POI::POI()` [private]
    /// invalidated standard constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new() -> Self {
        todo!("port POI::POI")
    }

}

