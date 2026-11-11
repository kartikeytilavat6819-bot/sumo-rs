//! Port of microsim/devices/MSDevice_FCD.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// A device which collects info on the vehicle trip (mainly on departure and arrival)
/// C++ class `MSDevice_FCD` (microsim/devices/MSDevice_FCD.h)
pub struct MSDevice_FCD {
    /// C++ base class `MSVehicleDevice`
    pub base: MSVehicleDevice,
}

impl std::ops::Deref for MSDevice_FCD {
    type Target = MSVehicleDevice;
    fn deref(&self) -> &MSVehicleDevice {
        &self.base
    }
}
impl std::ops::DerefMut for MSDevice_FCD {
    fn deref_mut(&mut self) -> &mut MSVehicleDevice {
        &mut self.base
    }
}

// C++ static members of MSDevice_FCD (port as module-level statics, see AGENTS.md):
//   static SUMOTime MSDevice_FCD::myBegin;
//   static SUMOTime MSDevice_FCD::myPeriod;
//   static bool MSDevice_FCD::myUseGeo;
//   static bool MSDevice_FCD::myUseUTM;
//   static double MSDevice_FCD::myMaxLeaderDistance;
//   static std::vector<std::string> MSDevice_FCD::myParamsToWrite;
//   static double MSDevice_FCD::myRadius;
//   static std::set<const MSEdge *> MSDevice_FCD::myEdgeFilter;
//   static bool MSDevice_FCD::myEdgeFilterInitialized;
//   static std::vector<PositionVector> MSDevice_FCD::myShape4Filters;
//   static bool MSDevice_FCD::myShapeFilterInitialized;
//   static bool MSDevice_FCD::myShapeFilterDesired;
//   static SumoXMLAttrMask MSDevice_FCD::myWrittenAttributes;
//   static bool MSDevice_FCD::mySkipEmpty;

impl MSDevice_FCD {
    /// C++: `void MSDevice_FCD::insertOptions(OptionsCont &)` [static]
    /// Inserts MSDevice_FCD-options
    // void
    // MSDevice_FCD::insertOptions(OptionsCont& oc) {
    //     oc.addOptionSubTopic("FCD Device");
    //     insertDefaultAssignmentOptions("fcd", "FCD Device", oc);
    //
    //     oc.doRegister("device.fcd.begin", new Option_String("-1"));
    //     oc.addDescription("device.fcd.begin", "FCD Device", TL("Recording begin time for FCD-data"));
    //
    //     oc.doRegister("device.fcd.period", new Option_String("0"));
    //     oc.addDescription("device.fcd.period", "FCD Device", TL("Recording period for FCD-data"));
    //
    //     oc.doRegister("device.fcd.radius", new Option_Float(0));
    //     oc.addDescription("device.fcd.radius", "FCD Device", TL("Record objects in a radius around equipped vehicles"));
    // }
    #[allow(unused_variables)]
    pub fn insert_options(oc: &mut OptionsCont) {
        todo!("port MSDevice_FCD::insertOptions")
    }

    /// C++: `void MSDevice_FCD::buildVehicleDevices(SUMOVehicle &, std::vector<MSVehicleDevice *> &)` [static]
    /// Build devices for the given vehicle, if needed
    // void
    // MSDevice_FCD::buildVehicleDevices(SUMOVehicle& v, std::vector<MSVehicleDevice*>& into) {
    //     OptionsCont& oc = OptionsCont::getOptions();
    //     if (equippedByDefaultAssignmentOptions(oc, "fcd", v, oc.isSet("fcd-output"))) {
    //         MSDevice_FCD* device = new MSDevice_FCD(v, "fcd_" + v.getID());
    //         into.push_back(device);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn build_vehicle_devices(v: &mut SUMOVehicle, into: &mut Vec<Ptr<MSVehicleDevice>>) {
        todo!("port MSDevice_FCD::buildVehicleDevices")
    }

    /// C++: `void MSDevice_FCD::~MSDevice_FCD() noexcept` [virtual]
    /// Destructor.
    // MSDevice_FCD::~MSDevice_FCD() {
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MSDevice_FCD::~MSDevice_FCD")
    }

    /// C++: `bool MSDevice_FCD::notifyEnter(SUMOTrafficObject &, MSMoveReminder::Notification, const MSLane *)` [virtual]
    // bool notifyEnter(SUMOTrafficObject& /*veh*/, MSMoveReminder::Notification /*reason*/, const MSLane* /*enteredLane*/) {
    //         return false;
    //     }
    #[allow(unused_variables)]
    pub fn notify_enter(&mut self, arg0: &mut SUMOTrafficObject, arg1: MSMoveReminder_Notification, arg2: Ptr<MSLane>) -> bool {
        todo!("port MSDevice_FCD::notifyEnter")
    }

    /// C++: `void MSDevice_FCD::saveState(OutputDevice &) const` [const, virtual]
    // void saveState(OutputDevice& /* out */) const {
    //     }
    #[allow(unused_variables)]
    pub fn save_state(&self, arg0: &mut OutputDevice) {
        todo!("port MSDevice_FCD::saveState")
    }

    /// C++: `const std::string MSDevice_FCD::deviceName() const` [const, virtual]
    /// return the name for this type of device
    // const std::string deviceName() const {
    //         return "fcd";
    //     }
    #[allow(unused_variables)]
    pub fn device_name(&self) -> String {
        todo!("port MSDevice_FCD::deviceName")
    }

    /// C++: `SUMOTime MSDevice_FCD::getBegin()` [static]
    // static inline SUMOTime getBegin() {
    //         return myBegin;
    //     }
    #[allow(unused_variables)]
    pub fn get_begin() -> SUMOTime {
        todo!("port MSDevice_FCD::getBegin")
    }

    /// C++: `SUMOTime MSDevice_FCD::getPeriod()` [static]
    // static inline SUMOTime getPeriod() {
    //         return myPeriod;
    //     }
    #[allow(unused_variables)]
    pub fn get_period() -> SUMOTime {
        todo!("port MSDevice_FCD::getPeriod")
    }

    /// C++: `bool MSDevice_FCD::useGeo()` [static]
    // static inline bool useGeo() {
    //         return myUseGeo;
    //     }
    #[allow(unused_variables)]
    pub fn use_geo() -> bool {
        todo!("port MSDevice_FCD::useGeo")
    }

    /// C++: `bool MSDevice_FCD::useUTM()` [static]
    // static inline bool useUTM() {
    //         return myUseUTM;
    //     }
    #[allow(unused_variables)]
    pub fn use_utm() -> bool {
        todo!("port MSDevice_FCD::useUTM")
    }

    /// C++: `double MSDevice_FCD::getMaxLeaderDistance()` [static]
    // static inline double getMaxLeaderDistance() {
    //         return myMaxLeaderDistance;
    //     }
    #[allow(unused_variables)]
    pub fn get_max_leader_distance() -> f64 {
        todo!("port MSDevice_FCD::getMaxLeaderDistance")
    }

    /// C++: `const std::vector<std::string> & MSDevice_FCD::getParamsToWrite()` [static]
    // static inline const std::vector<std::string>& getParamsToWrite() {
    //         return myParamsToWrite;
    //     }
    #[allow(unused_variables)]
    pub fn get_params_to_write() -> &'static Vec<String> {
        todo!("port MSDevice_FCD::getParamsToWrite")
    }

    /// C++: `double MSDevice_FCD::getRadius()` [static]
    // static inline double getRadius() {
    //         return myRadius;
    //     }
    #[allow(unused_variables)]
    pub fn get_radius() -> f64 {
        todo!("port MSDevice_FCD::getRadius")
    }

    /// C++: `const std::set<const MSEdge *> & MSDevice_FCD::getEdgeFilter()` [static]
    // static inline const std::set<const MSEdge*>& getEdgeFilter() {
    //         return myEdgeFilter;
    //     }
    #[allow(unused_variables)]
    pub fn get_edge_filter() -> &'static BTreeSet<Ptr<MSEdge>> {
        todo!("port MSDevice_FCD::getEdgeFilter")
    }

    /// C++: `const SumoXMLAttrMask & MSDevice_FCD::getWrittenAttributes()` [static]
    // static inline const SumoXMLAttrMask& getWrittenAttributes() {
    //         return myWrittenAttributes;
    //     }
    #[allow(unused_variables)]
    pub fn get_written_attributes() -> &'static SumoXMLAttrMask {
        todo!("port MSDevice_FCD::getWrittenAttributes")
    }

    /// C++: `bool MSDevice_FCD::skipEmpty()` [static]
    // static inline bool skipEmpty() {
    //         return mySkipEmpty;
    //     }
    #[allow(unused_variables)]
    pub fn skip_empty() -> bool {
        todo!("port MSDevice_FCD::skipEmpty")
    }

    /// C++: `void MSDevice_FCD::initOnce()` [static]
    /// initialize edge filter and attribute mask (once)
    // void
    // MSDevice_FCD::initOnce() {
    //     if (myEdgeFilterInitialized) {
    //         return;
    //     }
    //     myEdgeFilterInitialized = true;
    //     const OptionsCont& oc = OptionsCont::getOptions();
    //     myPeriod = string2time(oc.getString("device.fcd.period"));
    //     myBegin = string2time(oc.getString("device.fcd.begin"));
    //     myUseGeo = oc.getBool("fcd-output.geo");
    //     myUseUTM = oc.getBool("fcd-output.utm");
    //     myMaxLeaderDistance = oc.getFloat("fcd-output.max-leader-distance");
    //     myParamsToWrite = oc.getStringVector("fcd-output.params");
    //     myRadius = oc.getFloat("device.fcd.radius");
    //     mySkipEmpty = oc.getBool("fcd-output.skip-empty");
    //     if (oc.isSet("fcd-output.filter-edges.input-file")) {
    //         const std::string file = oc.getString("fcd-output.filter-edges.input-file");
    //         std::ifstream strm(file.c_str());
    //         if (!strm.good()) {
    //             throw ProcessError(TLF("Could not load names of edges for filtering fcd-output from '%'.", file));
    //         }
    //         while (strm.good()) {
    //             std::string name;
    //             strm >> name;
    //             // maybe we're loading an edge-selection
    //             if (StringUtils::startsWith(name, "edge:")) {
    //                 name = name.substr(5);
    //             }
    //             myEdgeFilter.insert(MSEdge::dictionary(name));
    //         }
    //     }
    //     SumoXMLAttrMask emissions;
    //     emissions.set(SUMO_ATTR_ECLASS);
    //     emissions.set(SUMO_ATTR_CO2);
    //     emissions.set(SUMO_ATTR_CO);
    //     emissions.set(SUMO_ATTR_HC);
    //     emissions.set(SUMO_ATTR_NOX);
    //     emissions.set(SUMO_ATTR_PMX);
    //     emissions.set(SUMO_ATTR_FUEL);
    //     emissions.set(SUMO_ATTR_ELECTRICITY);
    //     emissions.set(SUMO_ATTR_NOISE);
    //     SumoXMLAttrMask misc;
    //     misc.set(SUMO_ATTR_SIGNALS);
    //     misc.set(SUMO_ATTR_ACCELERATION);
    //     misc.set(SUMO_ATTR_ACCELERATION_LAT);
    //     misc.set(SUMO_ATTR_SPEED_VEC);
    //     misc.set(SUMO_ATTR_ACCEL_VEC);
    //     misc.set(SUMO_ATTR_DISTANCE);
    //     misc.set(SUMO_ATTR_ODOMETER);
    //     misc.set(SUMO_ATTR_POSITION_LAT);
    //     misc.set(SUMO_ATTR_SPEED_LAT);
    //     misc.set(SUMO_ATTR_SPEEDREL);
    //     misc.set(SUMO_ATTR_LEADER_ID);
    //     misc.set(SUMO_ATTR_LEADER_SPEED);
    //     misc.set(SUMO_ATTR_ARRIVALDELAY);
    //     misc.set(SUMO_ATTR_DELAY);
    //     misc.set(SUMO_ATTR_SEGMENT);
    //     misc.set(SUMO_ATTR_QUEUE);
    //     misc.set(SUMO_ATTR_ENTRYTIME);
    //     misc.set(SUMO_ATTR_EVENTTIME);
    //     misc.set(SUMO_ATTR_BLOCKTIME);
    //     const std::map<std::string, SumoXMLAttrMask> special = {{"location", getDefaultMask()}, {"emissions", emissions}, {"misc", misc}};
    //     if (oc.isSet("fcd-output.attributes")) {
    //         myWrittenAttributes = OutputDevice::parseWrittenAttributes(oc.getStringVector("fcd-output.attributes"), "fcd output", special);
    //     } else {
    //         myWrittenAttributes = getDefaultMask();
    //     }
    //     // need to store this because some attributes are reset later
    //     const bool all = myWrittenAttributes.all();
    //     myWrittenAttributes.set(SUMO_ATTR_ID);
    //     if (!MSNet::getInstance()->hasElevation()) {
    //         myWrittenAttributes.reset(SUMO_ATTR_Z);
    //     }
    //     if (oc.getBool("fcd-output.signals")) {
    //         myWrittenAttributes.set(SUMO_ATTR_SIGNALS);
    //     }
    //     if (oc.getBool("fcd-output.acceleration")) {
    //         myWrittenAttributes.set(SUMO_ATTR_ACCELERATION);
    //     }
    //     myWrittenAttributes.set(SUMO_ATTR_ACCELERATION_LAT, myWrittenAttributes.test(SUMO_ATTR_ACCELERATION) && MSGlobals::gSublane);
    //     if (oc.getBool("fcd-output.distance")) {
    //         myWrittenAttributes.set(SUMO_ATTR_DISTANCE);
    //     }
    //     if (oc.getBool("fcd-output.speed-relative")) {
    //         myWrittenAttributes.set(SUMO_ATTR_SPEEDREL);
    //     }
    //
    //     if (oc.isSet("fcd-output.filter-shapes")) {
    //         // build the shape filter if it is desired
    //         myShapeFilterDesired = true;
    //         buildShapeFilter();
    //     }
    //     OutputDevice::getDeviceByOption("fcd-output").setExpectedAttributes(all ? 0 : myWrittenAttributes, 2);
    // }
    #[allow(unused_variables)]
    pub fn init_once() {
        todo!("port MSDevice_FCD::initOnce")
    }

    /// C++: `void MSDevice_FCD::cleanup()` [static]
    /// resets the edge filter
    // void
    // MSDevice_FCD::cleanup() {
    //     myEdgeFilter.clear();
    //     myShape4Filters.clear();
    //     myEdgeFilterInitialized = false;
    //     myShapeFilterInitialized = false;
    //     myShapeFilterDesired = false;
    //     myWrittenAttributes.reset();
    // }
    #[allow(unused_variables)]
    pub fn cleanup() {
        todo!("port MSDevice_FCD::cleanup")
    }

    /// C++: `bool MSDevice_FCD::shapeFilter(const SUMOTrafficObject *)` [static]
    /// checks if in polygon
    // bool
    // MSDevice_FCD::shapeFilter(const SUMOTrafficObject* veh) {
    //     // lazily build the shape filter in the case where route file is loaded as an additional file
    //     if (!myShapeFilterInitialized) {
    //         buildShapeFilter();
    //     }
    //     const MSVehicle* msVeh = dynamic_cast<const MSVehicle*>(veh);
    //     for (auto shape : myShape4Filters) {
    //         if (shape.around(veh->getPosition()) || ((msVeh != nullptr) && shape.around(msVeh->getBackPosition()))) {
    //             return true;
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn shape_filter(veh: Ptr<SUMOTrafficObject>) -> bool {
        todo!("port MSDevice_FCD::shapeFilter")
    }

    /// C++: `bool MSDevice_FCD::hasShapeFilter()` [static]
    /// is there a filter based on shapes?
    // inline static bool hasShapeFilter() {
    //         return myShapeFilterDesired;
    //     }
    #[allow(unused_variables)]
    pub fn has_shape_filter() -> bool {
        todo!("port MSDevice_FCD::hasShapeFilter")
    }

    /// C++: `void MSDevice_FCD::MSDevice_FCD(SUMOVehicle &, const std::string &)` [private]
    /// Constructor
    // MSDevice_FCD::MSDevice_FCD(SUMOVehicle& holder, const std::string& id) :
    //     MSVehicleDevice(holder, id) {
    // }
    #[allow(unused_variables)]
    pub fn new(holder: &mut SUMOVehicle, id: &str) -> Self {
        todo!("port MSDevice_FCD::MSDevice_FCD")
    }

    /// C++: `SumoXMLAttrMask MSDevice_FCD::getDefaultMask()` [static, private]
    // SumoXMLAttrMask
    // MSDevice_FCD::getDefaultMask() {
    //     SumoXMLAttrMask mask;
    //     mask.set(SUMO_ATTR_X);
    //     mask.set(SUMO_ATTR_Y);
    //     if (MSNet::getInstance()->hasElevation()) {
    //         mask.set(SUMO_ATTR_Z);
    //     }
    //     mask.set(SUMO_ATTR_ANGLE);
    //     mask.set(SUMO_ATTR_TYPE);
    //     mask.set(SUMO_ATTR_SPEED);
    //     mask.set(SUMO_ATTR_POSITION);
    //     mask.set(SUMO_ATTR_LANE); // for micro vehicles only
    //     mask.set(SUMO_ATTR_EDGE); // for persons and meso vehicles
    //     mask.set(SUMO_ATTR_SLOPE);
    //     if (!MSGlobals::gUseMesoSim && OptionsCont::getOptions().getFloat("fcd-output.max-leader-distance") > 0.) {
    //         mask.set(SUMO_ATTR_LEADER_ID);
    //         mask.set(SUMO_ATTR_LEADER_SPEED);
    //         mask.set(SUMO_ATTR_LEADER_GAP);
    //     }
    //     return mask;
    // }
    #[allow(unused_variables)]
    pub fn get_default_mask() -> SumoXMLAttrMask {
        todo!("port MSDevice_FCD::getDefaultMask")
    }

    /// C++: `void MSDevice_FCD::MSDevice_FCD(const MSDevice_FCD &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSDevice_FCD) -> Self {
        todo!("port MSDevice_FCD::MSDevice_FCD")
    }

    /// C++: `MSDevice_FCD & MSDevice_FCD::operator=(const MSDevice_FCD &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSDevice_FCD) -> &mut MSDevice_FCD {
        todo!("port MSDevice_FCD::operator=")
    }

    /// C++: `void MSDevice_FCD::buildShapeFilter()` [static, private]
    // void
    // MSDevice_FCD::buildShapeFilter(void) {
    //     const OptionsCont& oc = OptionsCont::getOptions();
    //     if (oc.isSet("fcd-output.filter-shapes")) {
    //         const ShapeContainer& loadedShapes = MSNet::getInstance()->getShapeContainer();
    //         if (loadedShapes.getPolygons().size() > 0) {
    //             for (std::string attrName : oc.getStringVector("fcd-output.filter-shapes")) {
    //                 if (loadedShapes.getPolygons().get(attrName) == 0) {
    //                     WRITE_ERRORF(TL("Specified shape '%' for filtering fcd-output could not be found."), attrName);
    //                 } else {
    //                     // store the PositionVector, not reference, as traci can manipulate / detete the polygons
    //                     myShape4Filters.push_back(loadedShapes.getPolygons().get(attrName)->getShape());
    //                 }
    //             }
    //             myShapeFilterInitialized = true;
    //         }
    //     } else {
    //         myShapeFilterInitialized = true;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn build_shape_filter() {
        todo!("port MSDevice_FCD::buildShapeFilter")
    }

}

