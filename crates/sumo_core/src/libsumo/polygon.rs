//! Port of libsumo/Polygon.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
// shortcut: C++ throw TraCIException(msg) is ported as panic!("{}", msg), upgrade when TraCIException is ported
use crate::prelude::*;
use crate::utils::named_rtree::{delete_named_rtree, new_named_rtree, NamedRTree};
use crate::microsim::ms_event_control::Command;
use crate::utils::parametrised_wrapping_command::ParametrisedWrappingCommand;

/// C++ class `libsumo::Polygon` (libsumo/Polygon.h)
pub struct Polygon {
}

// C++ static members of Polygon (port as module-level statics, see AGENTS.md):
//   static const int Polygon::DOMAIN_ID;
//   static SubscriptionResults Polygon::mySubscriptionResults;
//   static ContextSubscriptionResults Polygon::myContextSubscriptionResults;
//   static NamedRTree * Polygon::myTree;
thread_local! {
    /// C++ static `Polygon::mySubscriptionResults`
    static MY_SUBSCRIPTION_RESULTS: std::cell::RefCell<SubscriptionResults> = std::cell::RefCell::new(SubscriptionResults::new());
    /// C++ static `Polygon::myContextSubscriptionResults`
    static MY_CONTEXT_SUBSCRIPTION_RESULTS: std::cell::RefCell<ContextSubscriptionResults> = std::cell::RefCell::new(ContextSubscriptionResults::new());
    /// C++ static `Polygon::myTree`
    static MY_TREE: std::cell::Cell<Ptr<NamedRTree>> = std::cell::Cell::new(Ptr::null());
}


impl Polygon {
    /// C++: `std::string Polygon::getType(const std::string &)` [static]
    // std::string
    // Polygon::getType(const std::string& polygonID) {
    //     return getPolygon(polygonID)->getShapeType();
    // }
    #[allow(unused_variables)]
    pub fn get_type(polygonID: &str) -> String {
        Self::get_polygon(polygonID).get_shape_type().to_string()
    }

    /// C++: `libsumo::TraCIPositionVector Polygon::getShape(const std::string &)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_shape(polygonID: &str) -> TraCIPositionVector {
        Helper::make_tra_ci_position_vector(Self::get_polygon(polygonID).get_shape())
    }

    /// C++: `libsumo::TraCIColor Polygon::getColor(const std::string &)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_color(polygonID: &str) -> TraCIColor {
        Helper::make_tra_ci_color(Self::get_polygon(polygonID).get_shape_color())
    }

    /// C++: `bool Polygon::getFilled(const std::string &)` [static]
    // bool
    // Polygon::getFilled(const std::string& polygonID) {
    //     return getPolygon(polygonID)->getFill();
    // }
    #[allow(unused_variables)]
    pub fn get_filled(polygonID: &str) -> bool {
        Self::get_polygon(polygonID).get_fill()
    }

    /// C++: `double Polygon::getLineWidth(const std::string &)` [static]
    // double
    // Polygon::getLineWidth(const std::string& polygonID) {
    //     return getPolygon(polygonID)->getLineWidth();
    // }
    #[allow(unused_variables)]
    pub fn get_line_width(polygonID: &str) -> f64 {
        Self::get_polygon(polygonID).get_line_width()
    }

    /// C++: `double Polygon::getHeight(const std::string &)` [static]
    // double
    // Polygon::getHeight(const std::string& polygonID) {
    //     return getPolygon(polygonID)->getHeight();
    // }
    #[allow(unused_variables)]
    pub fn get_height(polygonID: &str) -> f64 {
        Self::get_polygon(polygonID).get_height()
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
        let mut ids: Vec<String> = Vec::new();
        for id in MSNet::get_instance().m().get_shape_container().get_polygons().keys() {
            ids.push(id.clone());
        }
        ids
    }

    /// C++: `int Polygon::getIDCount()` [static]
    // int
    // Polygon::getIDCount() {
    //     return (int)getIDList().size();
    // }
    #[allow(unused_variables)]
    pub fn get_id_count() -> i32 {
        Self::get_id_list().len() as i32
    }

    /// C++: `std::string Polygon::getParameter(const std::string &, const std::string &)` [static]
    // std::string
    // Polygon::getParameter(const std::string& polygonID, const std::string& key) {
    //     return getPolygon(polygonID)->getParameter(key, "");
    // }
    #[allow(unused_variables)]
    pub fn get_parameter(objectID: &str, key: &str) -> String {
        Self::get_polygon(objectID).base2.get_parameter(key, "")
    }

    /// C++: `const std::pair<std::string, std::string> Polygon::getParameterWithKey(const std::string &, const std::string &)` [static]
    // LIBSUMO_GET_PARAMETER_WITH_KEY_IMPLEMENTATION(Polygon)
    #[allow(unused_variables)]
    pub fn get_parameter_with_key(objectID: &str, key: &str) -> (String, String) {
        (key.to_string(), Self::get_parameter(objectID, key))
    }

    /// C++: `void Polygon::setParameter(const std::string &, const std::string &, const std::string &)` [static]
    // void
    // Polygon::setParameter(const std::string& polygonID, const std::string& key, const std::string& value) {
    //     SUMOPolygon* p = getPolygon(polygonID);
    //     p->setParameter(key, value);
    // }
    #[allow(unused_variables)]
    pub fn set_parameter(objectID: &str, key: &str, value: &str) {
        Self::get_polygon(objectID).base2.set_parameter(key, value);
    }

    /// C++: `void Polygon::subscribe(const std::string &, const std::vector<int> &, double, double, const libsumo::TraCIResults &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Polygon, POLYGON)
    #[allow(unused_variables)]
    pub fn subscribe(objectID: &str, varIDs: &Vec<i32>, begin: f64, end: f64, parameters: &TraCIResults) {
        Helper::subscribe(CMD_SUBSCRIBE_POLYGON_VARIABLE, objectID, varIDs, begin, end, parameters, 0, 0.0);
    }

    /// C++: `void Polygon::unsubscribe(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Polygon, POLYGON)
    #[allow(unused_variables)]
    pub fn unsubscribe(objectID: &str) {
        Helper::subscribe(CMD_SUBSCRIBE_POLYGON_VARIABLE, objectID, &Vec::new(), INVALID_DOUBLE_VALUE, INVALID_DOUBLE_VALUE, &TraCIResults::new(), 0, 0.0);
    }

    /// C++: `void Polygon::subscribeContext(const std::string &, int, double, const std::vector<int> &, double, double, const libsumo::TraCIResults &)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn subscribe_context(objectID: &str, domain: i32, dist: f64, varIDs: &Vec<i32>, begin: f64, end: f64, parameters: &TraCIResults) {
        Helper::subscribe(CMD_SUBSCRIBE_POLYGON_CONTEXT, objectID, varIDs, begin, end, parameters, domain, dist);
    }

    /// C++: `void Polygon::unsubscribeContext(const std::string &, int, double)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Polygon, POLYGON)
    #[allow(unused_variables)]
    pub fn unsubscribe_context(objectID: &str, domain: i32, dist: f64) {
        Helper::subscribe(CMD_SUBSCRIBE_POLYGON_CONTEXT, objectID, &Vec::new(), INVALID_DOUBLE_VALUE, INVALID_DOUBLE_VALUE, &TraCIResults::new(), domain, dist);
    }

    /// C++: `const libsumo::SubscriptionResults Polygon::getAllSubscriptionResults()` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Polygon, POLYGON)
    #[allow(unused_variables)]
    pub fn get_all_subscription_results() -> SubscriptionResults {
        MY_SUBSCRIPTION_RESULTS.with(|r| r.borrow().clone())
    }

    /// C++: `const libsumo::TraCIResults Polygon::getSubscriptionResults(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Polygon, POLYGON)
    #[allow(unused_variables)]
    pub fn get_subscription_results(objectID: &str) -> TraCIResults {
        MY_SUBSCRIPTION_RESULTS.with(|r| r.borrow_mut().entry(objectID.to_string()).or_default().clone())
    }

    /// C++: `const libsumo::ContextSubscriptionResults Polygon::getAllContextSubscriptionResults()` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Polygon, POLYGON)
    #[allow(unused_variables)]
    pub fn get_all_context_subscription_results() -> ContextSubscriptionResults {
        MY_CONTEXT_SUBSCRIPTION_RESULTS.with(|r| r.borrow().clone())
    }

    /// C++: `const libsumo::SubscriptionResults Polygon::getContextSubscriptionResults(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Polygon, POLYGON)
    #[allow(unused_variables)]
    pub fn get_context_subscription_results(objectID: &str) -> SubscriptionResults {
        MY_CONTEXT_SUBSCRIPTION_RESULTS.with(|r| r.borrow_mut().entry(objectID.to_string()).or_default().clone())
    }

    /// C++: `void Polygon::subscribeParameterWithKey(const std::string &, const std::string &, double, double)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Polygon, POLYGON)
    #[allow(unused_variables)]
    pub fn subscribe_parameter_with_key(objectID: &str, key: &str, beginTime: f64, endTime: f64) {
        let mut params: TraCIResults = TraCIResults::new();
        params.insert(VAR_PARAMETER_WITH_KEY, crate::libsumo::helper::as_traci_result(TraCIString { base: TraCIResult { result_type: TYPE_STRING }, value: key.to_string() }));
        Helper::subscribe(CMD_SUBSCRIBE_POLYGON_VARIABLE, objectID, &vec![VAR_PARAMETER_WITH_KEY], beginTime, endTime, &params, 0, 0.0);
    }

    /// C++: `int Polygon::domainID()` [static]
    // LIBSUMO_SUBSCRIPTION_API
    #[allow(unused_variables)]
    pub fn domain_id() -> i32 {
        CMD_GET_POLYGON_VARIABLE
    }

    /// C++: `void Polygon::setType(const std::string &, const std::string &)` [static]
    // void
    // Polygon::setType(const std::string& polygonID, const std::string& polygonType) {
    //     getPolygon(polygonID)->setShapeType(polygonType);
    // }
    #[allow(unused_variables)]
    pub fn set_type(polygonID: &str, polygonType: &str) {
        Self::get_polygon(polygonID).set_shape_type(polygonType);
    }

    /// C++: `void Polygon::setShape(const std::string &, const libsumo::TraCIPositionVector &)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn set_shape(polygonID: &str, shape: &TraCIPositionVector) {
        let position_vector = Helper::make_position_vector(shape);
        Self::get_polygon(polygonID); // just to check whether it exists
        MSNet::get_instance().m().get_shape_container().reshape_polygon(polygonID, &position_vector);
    }

    /// C++: `void Polygon::setColor(const std::string &, const libsumo::TraCIColor &)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn set_color(polygonID: &str, color: &TraCIColor) {
        Self::get_polygon(polygonID).set_shape_color(&Helper::make_rgb_color(color));
    }

    /// C++: `void Polygon::add(const std::string &, const libsumo::TraCIPositionVector &, const libsumo::TraCIColor &, bool, const std::string &, int, double)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn add(polygonID: &str, shape: &TraCIPositionVector, color: &TraCIColor, fill: bool, polygonType: &str, layer: i32, lineWidth: f64) {
        let p_shape = Helper::make_position_vector(shape);
        let col = Helper::make_rgb_color(color);
        if !MSNet::get_instance().m().get_shape_container().add_polygon(polygonID, polygonType, &col, layer as f64, crate::utils::shapes::shape::DEFAULT_ANGLE, crate::utils::shapes::shape::DEFAULT_IMG_FILE, &p_shape, false, fill, lineWidth, false, "") {
            panic!("{}", format!("Could not add polygon '{}'", polygonID));
        }
        if !MY_TREE.with(|t| t.get()).is_null() {
            let p = MSNet::get_instance().m().get_shape_container().get_polygons().get(polygonID).copied().unwrap_or(Ptr::null());
            let b = p.get_shape().get_box_boundary();
            let cmin = [b.xmin() as f32, b.ymin() as f32];
            let cmax = [b.xmax() as f32, b.ymax() as f32];
            MY_TREE.with(|t| t.get()).m().insert(&cmin, &cmax, &p.cast::<Named>());
        }
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
        if timeSpan.is_empty() {
            if trackedObjectID == "" {
                panic!("{}", format!("Could not add polygon dynamics for polygon '{}': dynamics underspecified (either a tracked object ID or a time span have to be provided).", polygonID));
            }
            if looped {
                panic!("{}", format!("Could not add polygon dynamics for polygon '{}': looped==true requires time line of positive length.", polygonID));
            }
        }
        if timeSpan.len() == 1 {
            panic!("{}", format!("Could not add polygon dynamics for polygon '{}': time span cannot have length one.", polygonID));
        } else if !timeSpan.is_empty() && timeSpan[0] != 0.0 {
            panic!("{}", format!("Could not add polygon dynamics for polygon '{}': first element of time span must be zero.", polygonID));
        }
        if timeSpan.len() != alphaSpan.len() && !alphaSpan.is_empty() {
            panic!("{}", format!("Could not add polygon dynamics for polygon '{}': alpha span must have length zero or equal to time span length.", polygonID));
        }
        if timeSpan.len() >= 2 {
            for i in 1..timeSpan.len() {
                if timeSpan[i - 1] > timeSpan[i] {
                    panic!("{}", format!("Could not add polygon dynamics for polygon '{}': entries of time span must be ordered ascendingly.", polygonID));
                }
            }
        }
        let obj = Self::get_traffic_object(trackedObjectID);
        let sim_time = steps2time(MSNet::get_instance().m().get_current_time_step());
        let pd = MSNet::get_instance().m().get_shape_container().add_polygon_dynamics(sim_time, polygonID.to_string(), obj, timeSpan, alphaSpan, looped, rotate);
        if pd.is_null() {
            panic!("{}", format!("Could not add polygon dynamics for polygon '{}': polygon doesn't exist.", polygonID));
        }
        // Ensure existence of a DynamicShapeUpdater
        if MSNet::get_instance().m().get_dynamic_shape_updater().is_null() {
            // shortcut: relies on MSDynamicShapeUpdater having its VehicleStateListener base first (repr(C)), upgrade when the updater layout is checked
            let listener = MSNet::get_instance().m().make_dynamic_shape_updater().cast::<MSNet_VehicleStateListener>();
            MSNet::get_instance().m().add_vehicle_state_listener(listener);
        }
        // Schedule the regular polygon update
        let shapeCont = Ptr::from_mut(MSNet::get_instance().m().get_shape_container());
        let cmd: *mut ParametrisedWrappingCommand<ShapeContainer, Ptr<PolygonDynamics>> = Box::into_raw(Box::new(ParametrisedWrappingCommand::new(shapeCont, &pd, ShapeContainer::polygon_dynamics_update)));
        MSNet::get_instance().m().get_shape_container().add_polygon_update_command(pd.get_polygon_id().to_string(), Ptr::from_raw(cmd));
        MSNet::get_instance().m().get_end_of_timestep_events().m().add_event(Ptr::from_raw(cmd as *mut dyn Command), MSNet::get_instance().m().get_current_time_step());
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
        let tree = MY_TREE.with(|t| t.get());
        if !tree.is_null() {
            let p = MSNet::get_instance().m().get_shape_container().get_polygons().get(polygonID).copied().unwrap_or(Ptr::null());
            if !p.is_null() {
                let b = p.get_shape().get_box_boundary();
                let cmin = [b.xmin() as f32, b.ymin() as f32];
                let cmax = [b.xmax() as f32, b.ymax() as f32];
                tree.m().remove(&cmin, &cmax, &p.cast::<Named>());
            }
        }
        if !MSNet::get_instance().m().get_shape_container().remove_polygon(polygonID, true) {
            panic!("{}", format!("Could not remove polygon '{}'", polygonID));
        }
    }

    /// C++: `void Polygon::setFilled(std::string, bool)` [static]
    // void
    // Polygon::setFilled(std::string polygonID, bool filled) {
    //     getPolygon(polygonID)->setFill(filled);
    // }
    #[allow(unused_variables)]
    pub fn set_filled(polygonID: String, filled: bool) {
        Self::get_polygon(&polygonID).set_fill(filled);
    }

    /// C++: `void Polygon::setLineWidth(std::string, double)` [static]
    // void
    // Polygon::setLineWidth(std::string polygonID, double lineWidth) {
    //     getPolygon(polygonID)->setLineWidth(lineWidth);
    // }
    #[allow(unused_variables)]
    pub fn set_line_width(polygonID: String, lineWidth: f64) {
        Self::get_polygon(&polygonID).set_line_width(lineWidth);
    }

    /// C++: `void Polygon::setHeight(std::string, double)` [static]
    // void
    // Polygon::setHeight(std::string polygonID, double height) {
    //     getPolygon(polygonID)->setHeight(height);
    // }
    #[allow(unused_variables)]
    pub fn set_height(polygonID: String, height: f64) {
        Self::get_polygon(&polygonID).set_height(height);
    }

    /// C++: `void Polygon::addHighlightPolygon(const std::string &, const int, const std::string &, const libsumo::TraCIPositionVector &, const libsumo::TraCIColor &, bool, const std::string &, int, double)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn add_highlight_polygon(objectID: &str, type_: i32, polygonID: &str, shape: &TraCIPositionVector, color: &TraCIColor, fill: bool, polygonType: &str, layer: i32, lineWidth: f64) {
        Self::add(polygonID, shape, color, fill, polygonType, layer, lineWidth);
        MSNet::get_instance().m().get_shape_container().register_highlight(objectID, type_, polygonID);
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
    pub fn get_tree() -> Ptr<NamedRTree> {
        if MY_TREE.with(|t| t.get()).is_null() {
            let tree = new_named_rtree();
            MY_TREE.with(|t| t.set(tree));
            let net = MSNet::get_instance();
            for (_, i) in net.m().get_shape_container().get_polygons().iter() {
                let b = i.get_shape().get_box_boundary();
                let cmin = [b.xmin() as f32, b.ymin() as f32];
                let cmax = [b.xmax() as f32, b.ymax() as f32];
                tree.m().insert(&cmin, &cmax, &i.cast::<Named>());
            }
        }
        MY_TREE.with(|t| t.get())
    }

    /// C++: `void Polygon::cleanup()` [static]
    // void
    // Polygon::cleanup() {
    //     delete myTree;
    //     myTree = nullptr;
    // }
    #[allow(unused_variables)]
    pub fn cleanup() {
        delete_named_rtree(MY_TREE.with(|t| t.replace(Ptr::null())));
    }

    /// C++: `void Polygon::storeShape(const std::string &, PositionVector &)` [static]
    /// Saves the shape of the requested object in the given container
    // void
    // Polygon::storeShape(const std::string& id, PositionVector& shape) {
    //     shape = getPolygon(id)->getShape();
    // }
    #[allow(unused_variables)]
    pub fn store_shape(id: &str, shape: &mut PositionVector) {
        *shape = Self::get_polygon(id).get_shape().clone();
    }

    /// C++: `std::shared_ptr<VariableWrapper> Polygon::makeWrapper()` [static]
    // std::shared_ptr<VariableWrapper>
    // Polygon::makeWrapper() {
    //     return std::make_shared<Helper::SubscriptionWrapper>(handleVariable, mySubscriptionResults, myContextSubscriptionResults);
    // }
    #[allow(unused_variables)]
    pub fn make_wrapper() -> Rc<VariableWrapper> {
        Helper::make_subscription_wrapper(Some(Self::handle_variable), unsafe { &mut *MY_SUBSCRIPTION_RESULTS.with(|r| r.as_ptr()) }, unsafe { &mut *MY_CONTEXT_SUBSCRIPTION_RESULTS.with(|r| r.as_ptr()) })
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
        match variable {
            TRACI_ID_LIST => wrapper.m().wrap_string_list(objID, variable, &Self::get_id_list()),
            ID_COUNT => wrapper.m().wrap_int(objID, variable, Self::get_id_count()),
            VAR_TYPE => wrapper.m().wrap_string(objID, variable, &Self::get_type(objID)),
            VAR_COLOR => wrapper.m().wrap_color(objID, variable, &Self::get_color(objID)),
            VAR_FILL => wrapper.m().wrap_int(objID, variable, Self::get_filled(objID) as i32),
            VAR_WIDTH => wrapper.m().wrap_double(objID, variable, Self::get_line_width(objID)),
            VAR_HEIGHT => wrapper.m().wrap_double(objID, variable, Self::get_height(objID)),
            VAR_SHAPE => wrapper.m().wrap_position_vector(objID, variable, &Self::get_shape(objID)),
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

    /// C++: `bool Polygon::exists(std::string)` [static]
    /// Checks if a polygon of the given name exists already in the simulation
    // bool
    // Polygon::exists(std::string polyID) {
    //     SUMOPolygon* p = MSNet::getInstance()->getShapeContainer().getPolygons().get(polyID);
    //     return p != nullptr;
    // }
    #[allow(unused_variables)]
    pub fn exists(polyID: String) -> bool {
        MSNet::get_instance().m().get_shape_container().get_polygons().get(&polyID).is_some()
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
        let p = MSNet::get_instance().m().get_shape_container().get_polygons().get(id).copied().unwrap_or(Ptr::null());
        if p.is_null() {
            panic!("{}", format!("Polygon '{}' is not known", id));
        }
        p
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
        if id == "" {
            return Ptr::null();
        }
        let net = MSNet::get_instance();
        // First try to find a vehicle with the given id
        let sumo_vehicle = net.m().get_vehicle_control().get_vehicle(id);
        if !sumo_vehicle.is_null() {
            return sumo_vehicle.cast::<SUMOTrafficObject>();
        }
        let transportable = net.m().get_person_control().get(id);
        if !transportable.is_null() {
            return transportable.cast::<SUMOTrafficObject>();
        }
        panic!("{}", format!("Traffic object '{}' is not known", id))
    }

    /// C++: `void Polygon::Polygon()` [private]
    /// invalidated standard constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new() -> Self {
        panic!("libsumo::Polygon cannot be constructed: the standard constructor is invalidated in C++")
    }

}

