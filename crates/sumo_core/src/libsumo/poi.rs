//! Port of libsumo/POI.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
// shortcut: C++ throw TraCIException(msg) is ported as panic!("{}", msg), upgrade when TraCIException is ported
use crate::prelude::*;
use crate::utils::named_rtree::{delete_named_rtree, new_named_rtree, NamedRTree};

/// C++ class `libsumo::POI` (libsumo/POI.h)
pub struct POI {
}

// C++ static members of POI (port as module-level statics, see AGENTS.md):
//   static const int POI::DOMAIN_ID;
//   static SubscriptionResults POI::mySubscriptionResults;
//   static ContextSubscriptionResults POI::myContextSubscriptionResults;
//   static NamedRTree * POI::myTree;
thread_local! {
    /// C++ static `POI::mySubscriptionResults`
    static MY_SUBSCRIPTION_RESULTS: std::cell::RefCell<SubscriptionResults> = std::cell::RefCell::new(SubscriptionResults::new());
    /// C++ static `POI::myContextSubscriptionResults`
    static MY_CONTEXT_SUBSCRIPTION_RESULTS: std::cell::RefCell<ContextSubscriptionResults> = std::cell::RefCell::new(ContextSubscriptionResults::new());
    /// C++ static `POI::myTree`
    static MY_TREE: std::cell::Cell<Ptr<NamedRTree>> = std::cell::Cell::new(Ptr::null());
}


impl POI {
    /// C++: `std::string POI::getType(const std::string &)` [static]
    // std::string
    // POI::getType(const std::string& poiID) {
    //     return getPoI(poiID)->getShapeType();
    // }
    #[allow(unused_variables)]
    pub fn get_type(poiID: &str) -> String {
        Self::get_po_i(poiID).get_shape_type().to_string()
    }

    /// C++: `libsumo::TraCIPosition POI::getPosition(const std::string &, const bool)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_position(poiID: &str, includeZ: bool) -> TraCIPosition {
        Helper::make_tra_ci_position(&Self::get_po_i(poiID).base2, includeZ)
    }

    /// C++: `libsumo::TraCIColor POI::getColor(const std::string &)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_color(poiID: &str) -> TraCIColor {
        Helper::make_tra_ci_color(Self::get_po_i(poiID).get_shape_color())
    }

    /// C++: `double POI::getWidth(const std::string &)` [static]
    // double
    // POI::getWidth(const std::string& poiID) {
    //     return getPoI(poiID)->getWidth();
    // }
    #[allow(unused_variables)]
    pub fn get_width(poiID: &str) -> f64 {
        Self::get_po_i(poiID).get_width()
    }

    /// C++: `double POI::getHeight(const std::string &)` [static]
    // double
    // POI::getHeight(const std::string& poiID) {
    //     return getPoI(poiID)->getHeight();
    // }
    #[allow(unused_variables)]
    pub fn get_height(poiID: &str) -> f64 {
        Self::get_po_i(poiID).get_height()
    }

    /// C++: `double POI::getAngle(const std::string &)` [static]
    // double
    // POI::getAngle(const std::string& poiID) {
    //     return getPoI(poiID)->getShapeNaviDegree();
    // }
    #[allow(unused_variables)]
    pub fn get_angle(poiID: &str) -> f64 {
        Self::get_po_i(poiID).get_shape_navi_degree()
    }

    /// C++: `std::string POI::getImageFile(const std::string &)` [static]
    // std::string
    // POI::getImageFile(const std::string& poiID) {
    //     return getPoI(poiID)->getShapeImgFile();
    // }
    #[allow(unused_variables)]
    pub fn get_image_file(poiID: &str) -> String {
        Self::get_po_i(poiID).get_shape_img_file().to_string()
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
        let mut ids: Vec<String> = Vec::new();
        for id in MSNet::get_instance().m().get_shape_container().get_po_is().keys() {
            ids.push(id.clone());
        }
        ids
    }

    /// C++: `int POI::getIDCount()` [static]
    // int
    // POI::getIDCount() {
    //     return (int)getIDList().size();
    // }
    #[allow(unused_variables)]
    pub fn get_id_count() -> i32 {
        Self::get_id_list().len() as i32
    }

    /// C++: `std::string POI::getParameter(const std::string &, const std::string &)` [static]
    // std::string
    // POI::getParameter(const std::string& poiID, const std::string& key) {
    //     return getPoI(poiID)->getParameter(key, "");
    // }
    #[allow(unused_variables)]
    pub fn get_parameter(objectID: &str, key: &str) -> String {
        Self::get_po_i(objectID).base3.get_parameter(key, "")
    }

    /// C++: `const std::pair<std::string, std::string> POI::getParameterWithKey(const std::string &, const std::string &)` [static]
    // LIBSUMO_GET_PARAMETER_WITH_KEY_IMPLEMENTATION(POI)
    #[allow(unused_variables)]
    pub fn get_parameter_with_key(objectID: &str, key: &str) -> (String, String) {
        (key.to_string(), Self::get_parameter(objectID, key))
    }

    /// C++: `void POI::setParameter(const std::string &, const std::string &, const std::string &)` [static]
    // void
    // POI::setParameter(const std::string& poiID, const std::string& key, const std::string& value) {
    //     PointOfInterest* p = getPoI(poiID);
    //     p->setParameter(key, value);
    // }
    #[allow(unused_variables)]
    pub fn set_parameter(objectID: &str, key: &str, value: &str) {
        Self::get_po_i(objectID).base3.set_parameter(key, value);
    }

    /// C++: `void POI::subscribe(const std::string &, const std::vector<int> &, double, double, const libsumo::TraCIResults &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(POI, POI)
    #[allow(unused_variables)]
    pub fn subscribe(objectID: &str, varIDs: &Vec<i32>, begin: f64, end: f64, parameters: &TraCIResults) {
        Helper::subscribe(CMD_SUBSCRIBE_POI_VARIABLE, objectID, varIDs, begin, end, parameters, 0, 0.0);
    }

    /// C++: `void POI::unsubscribe(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(POI, POI)
    #[allow(unused_variables)]
    pub fn unsubscribe(objectID: &str) {
        Helper::subscribe(CMD_SUBSCRIBE_POI_VARIABLE, objectID, &Vec::new(), INVALID_DOUBLE_VALUE, INVALID_DOUBLE_VALUE, &TraCIResults::new(), 0, 0.0);
    }

    /// C++: `void POI::subscribeContext(const std::string &, int, double, const std::vector<int> &, double, double, const libsumo::TraCIResults &)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn subscribe_context(objectID: &str, domain: i32, dist: f64, varIDs: &Vec<i32>, begin: f64, end: f64, parameters: &TraCIResults) {
        Helper::subscribe(CMD_SUBSCRIBE_POI_CONTEXT, objectID, varIDs, begin, end, parameters, domain, dist);
    }

    /// C++: `void POI::unsubscribeContext(const std::string &, int, double)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(POI, POI)
    #[allow(unused_variables)]
    pub fn unsubscribe_context(objectID: &str, domain: i32, dist: f64) {
        Helper::subscribe(CMD_SUBSCRIBE_POI_CONTEXT, objectID, &Vec::new(), INVALID_DOUBLE_VALUE, INVALID_DOUBLE_VALUE, &TraCIResults::new(), domain, dist);
    }

    /// C++: `const libsumo::SubscriptionResults POI::getAllSubscriptionResults()` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(POI, POI)
    #[allow(unused_variables)]
    pub fn get_all_subscription_results() -> SubscriptionResults {
        MY_SUBSCRIPTION_RESULTS.with(|r| r.borrow().clone())
    }

    /// C++: `const libsumo::TraCIResults POI::getSubscriptionResults(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(POI, POI)
    #[allow(unused_variables)]
    pub fn get_subscription_results(objectID: &str) -> TraCIResults {
        MY_SUBSCRIPTION_RESULTS.with(|r| r.borrow_mut().entry(objectID.to_string()).or_default().clone())
    }

    /// C++: `const libsumo::ContextSubscriptionResults POI::getAllContextSubscriptionResults()` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(POI, POI)
    #[allow(unused_variables)]
    pub fn get_all_context_subscription_results() -> ContextSubscriptionResults {
        MY_CONTEXT_SUBSCRIPTION_RESULTS.with(|r| r.borrow().clone())
    }

    /// C++: `const libsumo::SubscriptionResults POI::getContextSubscriptionResults(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(POI, POI)
    #[allow(unused_variables)]
    pub fn get_context_subscription_results(objectID: &str) -> SubscriptionResults {
        MY_CONTEXT_SUBSCRIPTION_RESULTS.with(|r| r.borrow_mut().entry(objectID.to_string()).or_default().clone())
    }

    /// C++: `void POI::subscribeParameterWithKey(const std::string &, const std::string &, double, double)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(POI, POI)
    #[allow(unused_variables)]
    pub fn subscribe_parameter_with_key(objectID: &str, key: &str, beginTime: f64, endTime: f64) {
        let mut params: TraCIResults = TraCIResults::new();
        params.insert(VAR_PARAMETER_WITH_KEY, crate::libsumo::helper::as_traci_result(TraCIString { base: TraCIResult { result_type: TYPE_STRING }, value: key.to_string() }));
        Helper::subscribe(CMD_SUBSCRIBE_POI_VARIABLE, objectID, &vec![VAR_PARAMETER_WITH_KEY], beginTime, endTime, &params, 0, 0.0);
    }

    /// C++: `int POI::domainID()` [static]
    // LIBSUMO_SUBSCRIPTION_API
    #[allow(unused_variables)]
    pub fn domain_id() -> i32 {
        CMD_GET_POI_VARIABLE
    }

    /// C++: `void POI::setType(const std::string &, const std::string &)` [static]
    // void
    // POI::setType(const std::string& poiID, const std::string& poiType) {
    //     getPoI(poiID)->setShapeType(poiType);
    // }
    #[allow(unused_variables)]
    pub fn set_type(poiID: &str, poiType: &str) {
        Self::get_po_i(poiID).set_shape_type(poiType);
    }

    /// C++: `void POI::setColor(const std::string &, const libsumo::TraCIColor &)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn set_color(poiID: &str, color: &TraCIColor) {
        Self::get_po_i(poiID).set_shape_color(&Helper::make_rgb_color(color));
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
        Self::get_po_i(poiID);
        MSNet::get_instance().m().get_shape_container().move_poi(poiID, &Position::new(x, y));
    }

    /// C++: `void POI::setWidth(const std::string &, double)` [static]
    // void
    // POI::setWidth(const std::string& poiID, double width) {
    //     getPoI(poiID)->setWidth(width);
    // }
    #[allow(unused_variables)]
    pub fn set_width(poiID: &str, width: f64) {
        Self::get_po_i(poiID).set_width(width);
    }

    /// C++: `void POI::setHeight(const std::string &, double)` [static]
    // void
    // POI::setHeight(const std::string& poiID, double height) {
    //     getPoI(poiID)->setHeight(height);
    // }
    #[allow(unused_variables)]
    pub fn set_height(poiID: &str, height: f64) {
        Self::get_po_i(poiID).set_height(height);
    }

    /// C++: `void POI::setAngle(const std::string &, double)` [static]
    // void
    // POI::setAngle(const std::string& poiID, double angle) {
    //     getPoI(poiID)->setShapeNaviDegree(angle);
    // }
    #[allow(unused_variables)]
    pub fn set_angle(poiID: &str, angle: f64) {
        Self::get_po_i(poiID).set_shape_navi_degree(angle);
    }

    /// C++: `void POI::setImageFile(const std::string &, const std::string &)` [static]
    // void
    // POI::setImageFile(const std::string& poiID, const std::string& imageFile) {
    //     getPoI(poiID)->setShapeImgFile(imageFile);
    // }
    #[allow(unused_variables)]
    pub fn set_image_file(poiID: &str, imageFile: &str) {
        Self::get_po_i(poiID).set_shape_img_file(imageFile);
    }

    /// C++: `bool POI::add(const std::string &, double, double, const libsumo::TraCIColor &, const std::string &, int, const std::string &, double, double, double, const std::string &)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn add(poiID: &str, x: f64, y: f64, color: &TraCIColor, poiType: &str, layer: i32, imgFile: &str, width: f64, height: f64, angle: f64, icon: &str) -> bool {
        let ok = MSNet::get_instance().m().get_shape_container().add_poi(poiID, poiType, &Helper::make_rgb_color(color), &Position::new(x, y), false, "", 0.0, false, 0.0, icon, layer as f64, angle, imgFile, width, height, false);
        if ok && !MY_TREE.with(|t| t.get()).is_null() {
            let p = MSNet::get_instance().m().get_shape_container().get_po_is().get(poiID).copied().unwrap_or(Ptr::null());
            let c = [p.base2.x() as f32, p.base2.y() as f32];
            MY_TREE.with(|t| t.get()).m().insert(&c, &c, &p.cast::<Named>());
        }
        ok
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
        let tree = MY_TREE.with(|t| t.get());
        let p = MSNet::get_instance().m().get_shape_container().get_po_is().get(poiID).copied().unwrap_or(Ptr::null());
        if !p.is_null() && !tree.is_null() {
            let c = [p.base2.x() as f32, p.base2.y() as f32];
            tree.m().remove(&c, &c, &p.cast::<Named>());
        }
        MSNet::get_instance().m().get_shape_container().remove_poi(poiID)
    }

    /// C++: `void POI::highlight(const std::string &, const libsumo::TraCIColor &, double, const int, const double, const int)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn highlight(poiID: &str, col: &TraCIColor, size: f64, alphaMax: i32, duration: f64, type_: i32) {
        let poi = Self::get_po_i(poiID);
        // Center of the highlight circle
        let center: Position = poi.base2;
        // Size of the highlight circle
        let mut size = size;
        if size <= 0.0 {
            size = (poi.get_height() * poi.get_height() + poi.get_width() * poi.get_width()).sqrt() * 0.7;
        }
        // Make polygon shape
        let n_points: u32 = 34;
        let circle_pv = utils::geom::geom_helper::make_ring(size, size + 1.0, &center, n_points);
        let circle = Helper::make_tra_ci_position_vector(&circle_pv);
        // Find a free polygon id
        let mut i: i32 = 0;
        let mut poly_id = format!("{}_hl{}", poi.get_id(), i);
        while Polygon::exists(poly_id.clone()) {
            i += 1;
            poly_id = format!("{}_hl{}", poi.get_id(), i);
        }
        // Line width
        let line_width: f64 = 0.0;
        // Layer
        let mut lyr: f64 = 0.0;
        if MSNet::get_instance().m().is_gui_net() {
            lyr = poi.get_shape_layer();
            lyr += (type_ + 1) as f64 / 257.0;
        }
        // Make Polygon
        Polygon::add_highlight_polygon(poiID, type_, &poly_id, &circle, col, true, "highlight", lyr as i32, line_width);
        // Animation time line
        let max_attack: f64 = 1.0;
        let mut time_span: Vec<f64> = Vec::new();
        if duration > 0.0 {
            let fade_in = duration / 3.0;
            time_span = vec![0.0, if max_attack < fade_in { max_attack } else { fade_in }, 2.0 * duration / 3.0, duration];
        }
        // Alpha time line
        let mut alpha_span: Vec<f64> = Vec::new();
        if alphaMax > 0 {
            alpha_span = vec![0.0, alphaMax as f64, (alphaMax as f64) / 3.0, 0.0];
        }
        // Attach dynamics
        Polygon::add_dynamics(&poly_id, "", &time_span, &alpha_span, false, false);
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
    pub fn get_tree() -> Ptr<NamedRTree> {
        if MY_TREE.with(|t| t.get()).is_null() {
            let tree = new_named_rtree();
            MY_TREE.with(|t| t.set(tree));
            let net = MSNet::get_instance();
            for (_, i) in net.m().get_shape_container().get_po_is().iter() {
                let c = [i.base2.x() as f32, i.base2.y() as f32];
                tree.m().insert(&c, &c, &i.cast::<Named>());
            }
        }
        MY_TREE.with(|t| t.get())
    }

    /// C++: `void POI::cleanup()` [static]
    // void
    // POI::cleanup() {
    //     delete myTree;
    //     myTree = nullptr;
    // }
    #[allow(unused_variables)]
    pub fn cleanup() {
        delete_named_rtree(MY_TREE.with(|t| t.replace(Ptr::null())));
    }

    /// C++: `void POI::storeShape(const std::string &, PositionVector &)` [static]
    /// Saves the shape of the requested object in the given container
    // void
    // POI::storeShape(const std::string& id, PositionVector& shape) {
    //     shape.push_back(*getPoI(id));
    // }
    #[allow(unused_variables)]
    pub fn store_shape(id: &str, shape: &mut PositionVector) {
        shape.0.push(Self::get_po_i(id).base2);
    }

    /// C++: `std::shared_ptr<VariableWrapper> POI::makeWrapper()` [static]
    // std::shared_ptr<VariableWrapper>
    // POI::makeWrapper() {
    //     return std::make_shared<Helper::SubscriptionWrapper>(handleVariable, mySubscriptionResults, myContextSubscriptionResults);
    // }
    #[allow(unused_variables)]
    pub fn make_wrapper() -> Rc<VariableWrapper> {
        Helper::make_subscription_wrapper(Some(Self::handle_variable), unsafe { &mut *MY_SUBSCRIPTION_RESULTS.with(|r| r.as_ptr()) }, unsafe { &mut *MY_CONTEXT_SUBSCRIPTION_RESULTS.with(|r| r.as_ptr()) })
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
        match variable {
            TRACI_ID_LIST => wrapper.m().wrap_string_list(objID, variable, &Self::get_id_list()),
            ID_COUNT => wrapper.m().wrap_int(objID, variable, Self::get_id_count()),
            VAR_TYPE => wrapper.m().wrap_string(objID, variable, &Self::get_type(objID)),
            VAR_COLOR => wrapper.m().wrap_color(objID, variable, &Self::get_color(objID)),
            VAR_POSITION => wrapper.m().wrap_position(objID, variable, &Self::get_position(objID, false)),
            VAR_POSITION3D => wrapper.m().wrap_position(objID, variable, &Self::get_position(objID, true)),
            VAR_WIDTH => wrapper.m().wrap_double(objID, variable, Self::get_width(objID)),
            VAR_HEIGHT => wrapper.m().wrap_double(objID, variable, Self::get_height(objID)),
            VAR_ANGLE => wrapper.m().wrap_double(objID, variable, Self::get_angle(objID)),
            VAR_IMAGEFILE => wrapper.m().wrap_string(objID, variable, &Self::get_image_file(objID)),
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
        let sumo_poi = MSNet::get_instance().m().get_shape_container().get_po_is().get(id).copied().unwrap_or(Ptr::null());
        if sumo_poi.is_null() {
            panic!("{}", format!("POI '{}' is not known", id));
        }
        sumo_poi
    }

    /// C++: `void POI::POI()` [private]
    /// invalidated standard constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new() -> Self {
        panic!("libsumo::POI cannot be constructed: the standard constructor is invalidated in C++")
    }

}

