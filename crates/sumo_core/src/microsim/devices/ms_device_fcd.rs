//! Port of microsim/devices/MSDevice_FCD.{h,cpp}
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use std::cell::{Cell, RefCell};
use utils::common::string_utils::starts_with;
use utils::common::time::string2time;
use utils::iodevices::output_device::OutputDevice;
use utils::options::options_cont::{get_bool, get_float, get_string, get_string_vector, is_set, with_options};
use utils::xml::sumo_xml_definitions::SumoXMLAttrMask;

/// shortcut: SumoXMLAttrMask has no reset()/all() in utils (bitset<96> semantics), local helpers mirror them, upgrade when the utils mask gains these methods
fn mask_all(m: SumoXMLAttrMask) -> bool {
    m.0 == (1u128 << SumoXMLAttrMask::SIZE) - 1
}
fn mask_reset(m: &mut SumoXMLAttrMask, bit: i32) {
    m.0 &= !(1u128 << bit);
}

/// shortcut: MSGlobals is unported (no accessors); gUseMesoSim is set from the "mesosim" option in MSFrame::init, so read it from there
/// Names read by `while (strm.good()) { strm >> name; ... }`: whitespace-separated tokens, plus one
/// empty name when the stream hits EOF on a `>>` (trailing whitespace or an empty file)
fn edge_filter_names(content: &str) -> Vec<&str> {
    let mut names: Vec<&str> = content.split_whitespace().collect();
    if content.is_empty() || content.ends_with(char::is_whitespace) {
        names.push("");
    }
    names
}


thread_local! {
    static MY_BEGIN: Cell<SUMOTime> = const { Cell::new(0) };
    static MY_PERIOD: Cell<SUMOTime> = const { Cell::new(0) };
    static MY_USE_GEO: Cell<bool> = const { Cell::new(false) };
    static MY_USE_UTM: Cell<bool> = const { Cell::new(false) };
    static MY_MAX_LEADER_DISTANCE: Cell<f64> = const { Cell::new(0.0) };
    static MY_PARAMS_TO_WRITE: RefCell<Vec<String>> = const { RefCell::new(Vec::new()) };
    static MY_RADIUS: Cell<f64> = const { Cell::new(0.0) };
    static MY_EDGE_FILTER: RefCell<BTreeSet<Ptr<MSEdge>>> = const { RefCell::new(BTreeSet::new()) };
    static MY_EDGE_FILTER_INITIALIZED: Cell<bool> = const { Cell::new(false) };
    static MY_SHAPE4_FILTERS: RefCell<Vec<PositionVector>> = const { RefCell::new(Vec::new()) };
    static MY_SHAPE_FILTER_INITIALIZED: Cell<bool> = const { Cell::new(false) };
    static MY_SHAPE_FILTER_DESIRED: Cell<bool> = const { Cell::new(false) };
    static MY_WRITTEN_ATTRIBUTES: Cell<SumoXMLAttrMask> = const { Cell::new(SumoXMLAttrMask(0)) };
    static MY_SKIP_EMPTY: Cell<bool> = const { Cell::new(false) };
}

/// A device which collects info on the vehicle trip (mainly on departure and arrival)
/// C++ class `MSDevice_FCD` (microsim/devices/MSDevice_FCD.h)
#[repr(C)]
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
        oc.add_option_sub_topic("FCD Device");
        MSDevice::insert_default_assignment_options("fcd", "FCD Device", oc, false);

        oc.do_register("device.fcd.begin", SumoOption::string_with("-1"));
        oc.add_description("device.fcd.begin", "FCD Device", "Recording begin time for FCD-data");

        oc.do_register("device.fcd.period", SumoOption::string_with("0"));
        oc.add_description("device.fcd.period", "FCD Device", "Recording period for FCD-data");

        oc.do_register("device.fcd.radius", SumoOption::float(0.0));
        oc.add_description("device.fcd.radius", "FCD Device", "Record objects in a radius around equipped vehicles");
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
        let vp = Ptr::from_mut(v);
        let equipped = with_options(|oc| {
            MSDevice::equipped_by_default_assignment_options(
                oc,
                "fcd",
                &vp.get_id(),
                &vp.get_parameter().base,
                &vp.get_v_type_parameter().base,
                is_set("fcd-output"),
                false,
            )
        });
        if equipped {
            let id = format!("fcd_{}", vp.get_id());
            // the device is owned by the vehicle's device list for the rest of the run, as in C++
            let device: Ptr<MSDevice_FCD> = Ptr::from_mut(Box::leak(Box::new(MSDevice_FCD::new(vp.m(), &id))));
            into.push(device.cast::<MSVehicleDevice>());
        }
    }

    /// C++: `void MSDevice_FCD::~MSDevice_FCD() noexcept` [virtual]
    /// Destructor.
    // MSDevice_FCD::~MSDevice_FCD() {
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
    }

    /// C++: `bool MSDevice_FCD::notifyEnter(SUMOTrafficObject &, MSMoveReminder::Notification, const MSLane *)` [virtual]
    // bool notifyEnter(SUMOTrafficObject& /*veh*/, MSMoveReminder::Notification /*reason*/, const MSLane* /*enteredLane*/) {
    //         return false;
    //     }
    #[allow(unused_variables)]
    pub fn notify_enter(&mut self, arg0: &mut SUMOTrafficObject, arg1: MSMoveReminder_Notification, arg2: Ptr<MSLane>) -> bool {
        false
    }

    /// C++: `void MSDevice_FCD::saveState(OutputDevice &) const` [const, virtual]
    // void saveState(OutputDevice& /* out */) const {
    //     }
    #[allow(unused_variables)]
    pub fn save_state(&self, arg0: &mut OutputDevice) {
        // C++ saveState body is empty

    }

    /// C++: `const std::string MSDevice_FCD::deviceName() const` [const, virtual]
    /// return the name for this type of device
    // const std::string deviceName() const {
    //         return "fcd";
    //     }
    #[allow(unused_variables)]
    pub fn device_name(&self) -> String {
        "fcd".to_string()
    }

    /// C++: `SUMOTime MSDevice_FCD::getBegin()` [static]
    // static inline SUMOTime getBegin() {
    //         return myBegin;
    //     }
    #[allow(unused_variables)]
    pub fn get_begin() -> SUMOTime {
        MY_BEGIN.with(|c| c.get())
    }

    /// C++: `SUMOTime MSDevice_FCD::getPeriod()` [static]
    // static inline SUMOTime getPeriod() {
    //         return myPeriod;
    //     }
    #[allow(unused_variables)]
    pub fn get_period() -> SUMOTime {
        MY_PERIOD.with(|c| c.get())
    }

    /// C++: `bool MSDevice_FCD::useGeo()` [static]
    // static inline bool useGeo() {
    //         return myUseGeo;
    //     }
    #[allow(unused_variables)]
    pub fn use_geo() -> bool {
        MY_USE_GEO.with(|c| c.get())
    }

    /// C++: `bool MSDevice_FCD::useUTM()` [static]
    // static inline bool useUTM() {
    //         return myUseUTM;
    //     }
    #[allow(unused_variables)]
    pub fn use_utm() -> bool {
        MY_USE_UTM.with(|c| c.get())
    }

    /// C++: `double MSDevice_FCD::getMaxLeaderDistance()` [static]
    // static inline double getMaxLeaderDistance() {
    //         return myMaxLeaderDistance;
    //     }
    #[allow(unused_variables)]
    pub fn get_max_leader_distance() -> f64 {
        MY_MAX_LEADER_DISTANCE.with(|c| c.get())
    }

    /// C++: `const std::vector<std::string> & MSDevice_FCD::getParamsToWrite()` [static]
    // static inline const std::vector<std::string>& getParamsToWrite() {
    //         return myParamsToWrite;
    //     }
    #[allow(unused_variables)]
    // shortcut: returns a copy instead of a reference, since the static lives in a thread_local (no &'static)
    pub fn get_params_to_write() -> Vec<String> {
        MY_PARAMS_TO_WRITE.with(|c| c.borrow().clone())
    }

    /// C++: `double MSDevice_FCD::getRadius()` [static]
    // static inline double getRadius() {
    //         return myRadius;
    //     }
    #[allow(unused_variables)]
    pub fn get_radius() -> f64 {
        MY_RADIUS.with(|c| c.get())
    }

    /// C++: `const std::set<const MSEdge *> & MSDevice_FCD::getEdgeFilter()` [static]
    // static inline const std::set<const MSEdge*>& getEdgeFilter() {
    //         return myEdgeFilter;
    //     }
    #[allow(unused_variables)]
    // shortcut: returns a copy instead of a reference, since the static lives in a thread_local (no &'static)
    pub fn get_edge_filter() -> BTreeSet<Ptr<MSEdge>> {
        MY_EDGE_FILTER.with(|c| c.borrow().clone())
    }

    /// C++: `const SumoXMLAttrMask & MSDevice_FCD::getWrittenAttributes()` [static]
    // static inline const SumoXMLAttrMask& getWrittenAttributes() {
    //         return myWrittenAttributes;
    //     }
    #[allow(unused_variables)]
    // shortcut: returns a copy instead of a reference, since the static lives in a thread_local (no &'static)
    pub fn get_written_attributes() -> SumoXMLAttrMask {
        MY_WRITTEN_ATTRIBUTES.with(|c| c.get())
    }

    /// C++: `bool MSDevice_FCD::skipEmpty()` [static]
    // static inline bool skipEmpty() {
    //         return mySkipEmpty;
    //     }
    #[allow(unused_variables)]
    pub fn skip_empty() -> bool {
        MY_SKIP_EMPTY.with(|c| c.get())
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
        if MY_EDGE_FILTER_INITIALIZED.get() {
            return;
        }
        MY_EDGE_FILTER_INITIALIZED.set(true);
        MY_PERIOD.set(string2time(&get_string("device.fcd.period")).unwrap_or_else(|e| panic!("{e}")));
        MY_BEGIN.set(string2time(&get_string("device.fcd.begin")).unwrap_or_else(|e| panic!("{e}")));
        MY_USE_GEO.set(get_bool("fcd-output.geo"));
        MY_USE_UTM.set(get_bool("fcd-output.utm"));
        MY_MAX_LEADER_DISTANCE.set(get_float("fcd-output.max-leader-distance"));
        MY_PARAMS_TO_WRITE.with(|c| *c.borrow_mut() = get_string_vector("fcd-output.params"));
        MY_RADIUS.set(get_float("device.fcd.radius"));
        MY_SKIP_EMPTY.set(get_bool("fcd-output.skip-empty"));
        if is_set("fcd-output.filter-edges.input-file") {
            let file = get_string("fcd-output.filter-edges.input-file");
            let content = match std::fs::read_to_string(&file) {
                Ok(content) => content,
                Err(_) => panic!("{}", utils::tlf!("Could not load names of edges for filtering fcd-output from '%'.", file)),
            };
            for name in edge_filter_names(&content) {
                let name = if starts_with(name, "edge:") { &name[5..] } else { name };
                MY_EDGE_FILTER.with(|c| c.borrow_mut().insert(MSEdge::dictionary_2(name)));
            }
        }
        let mut emissions = SumoXMLAttrMask::default();
        emissions.set(SUMO_ATTR_ECLASS);
        emissions.set(SUMO_ATTR_CO2);
        emissions.set(SUMO_ATTR_CO);
        emissions.set(SUMO_ATTR_HC);
        emissions.set(SUMO_ATTR_NOX);
        emissions.set(SUMO_ATTR_PMX);
        emissions.set(SUMO_ATTR_FUEL);
        emissions.set(SUMO_ATTR_ELECTRICITY);
        emissions.set(SUMO_ATTR_NOISE);
        let mut misc = SumoXMLAttrMask::default();
        misc.set(SUMO_ATTR_SIGNALS);
        misc.set(SUMO_ATTR_ACCELERATION);
        misc.set(SUMO_ATTR_ACCELERATION_LAT);
        misc.set(SUMO_ATTR_SPEED_VEC);
        misc.set(SUMO_ATTR_ACCEL_VEC);
        misc.set(SUMO_ATTR_DISTANCE);
        misc.set(SUMO_ATTR_ODOMETER);
        misc.set(SUMO_ATTR_POSITION_LAT);
        misc.set(SUMO_ATTR_SPEED_LAT);
        misc.set(SUMO_ATTR_SPEEDREL);
        misc.set(SUMO_ATTR_LEADER_ID);
        misc.set(SUMO_ATTR_LEADER_SPEED);
        misc.set(SUMO_ATTR_ARRIVALDELAY);
        misc.set(SUMO_ATTR_DELAY);
        misc.set(SUMO_ATTR_SEGMENT);
        misc.set(SUMO_ATTR_QUEUE);
        misc.set(SUMO_ATTR_ENTRYTIME);
        misc.set(SUMO_ATTR_EVENTTIME);
        misc.set(SUMO_ATTR_BLOCKTIME);
        let special: HashMap<String, SumoXMLAttrMask> = HashMap::from([
            ("location".to_string(), MSDevice_FCD::get_default_mask()),
            ("emissions".to_string(), emissions),
            ("misc".to_string(), misc),
        ]);
        let mut written = if is_set("fcd-output.attributes") {
            OutputDevice::parse_written_attributes(&get_string_vector("fcd-output.attributes"), "fcd output", &special)
        } else {
            MSDevice_FCD::get_default_mask()
        };
        // need to store this because some attributes are reset later
        let all = mask_all(written);
        written.set(SUMO_ATTR_ID);
        if !MSNet::get_instance().has_elevation() {
            mask_reset(&mut written, SUMO_ATTR_Z);
        }
        if get_bool("fcd-output.signals") {
            written.set(SUMO_ATTR_SIGNALS);
        }
        if get_bool("fcd-output.acceleration") {
            written.set(SUMO_ATTR_ACCELERATION);
        }
        let accel_lat = written.test(SUMO_ATTR_ACCELERATION) && MSGlobals::sublane();
        if accel_lat {
            written.set(SUMO_ATTR_ACCELERATION_LAT);
        } else {
            mask_reset(&mut written, SUMO_ATTR_ACCELERATION_LAT);
        }
        if get_bool("fcd-output.distance") {
            written.set(SUMO_ATTR_DISTANCE);
        }
        if get_bool("fcd-output.speed-relative") {
            written.set(SUMO_ATTR_SPEEDREL);
        }
        MY_WRITTEN_ATTRIBUTES.set(written);

        if is_set("fcd-output.filter-shapes") {
            // build the shape filter if it is desired
            MY_SHAPE_FILTER_DESIRED.set(true);
            MSDevice_FCD::build_shape_filter();
        }
        let expected = if all { SumoXMLAttrMask::default() } else { written };
        OutputDevice::get_device_by_option("fcd-output")
            .unwrap_or_else(|e| panic!("{e}"))
            .borrow_mut()
            .set_expected_attributes(expected, 2);
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
        MY_EDGE_FILTER.with(|c| c.borrow_mut().clear());
        MY_SHAPE4_FILTERS.with(|c| c.borrow_mut().clear());
        MY_EDGE_FILTER_INITIALIZED.set(false);
        MY_SHAPE_FILTER_INITIALIZED.set(false);
        MY_SHAPE_FILTER_DESIRED.set(false);
        MY_WRITTEN_ATTRIBUTES.set(SumoXMLAttrMask::default());
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
        // lazily build the shape filter in the case where route file is loaded as an additional file
        if !MY_SHAPE_FILTER_INITIALIZED.get() {
            MSDevice_FCD::build_shape_filter();
        }
        // shortcut: dynamic_cast<const MSVehicle*> has no Rust equivalent for SUMOTrafficObject, so the back-position test (msVeh->getBackPosition()) is skipped, upgrade when a SUMOTrafficObject-to-MSVehicle downcast exists
        let pos = veh.get_position(0.0);
        MY_SHAPE4_FILTERS.with(|c| c.borrow().iter().any(|shape| shape.around(&pos, 0.0)))
    }

    /// C++: `bool MSDevice_FCD::hasShapeFilter()` [static]
    /// is there a filter based on shapes?
    // inline static bool hasShapeFilter() {
    //         return myShapeFilterDesired;
    //     }
    #[allow(unused_variables)]
    pub fn has_shape_filter() -> bool {
        MY_SHAPE_FILTER_DESIRED.get()
    }

    /// C++: `void MSDevice_FCD::MSDevice_FCD(SUMOVehicle &, const std::string &)` [private]
    /// Constructor
    // MSDevice_FCD::MSDevice_FCD(SUMOVehicle& holder, const std::string& id) :
    //     MSVehicleDevice(holder, id) {
    // }
    #[allow(unused_variables)]
    pub fn new(holder: &mut SUMOVehicle, id: &str) -> Self {
        fn mr_notify_enter(mr: &mut MSMoveReminder, veh: &mut SUMOTrafficObject, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
            Ptr::from_mut(mr).cast::<MSDevice_FCD>().m().notify_enter(veh, reason, enteredLane)
        }
        static MS_MOVE_REMINDER_VT: MSMoveReminderVt = MSMoveReminderVt {
            notify_enter: mr_notify_enter,
            notify_move: crate::microsim::ms_move_reminder::default_notify_move,
            notify_leave: crate::microsim::ms_move_reminder::default_notify_leave,
        };
        let mut this = Self::new_device(holder, id);
        this.base.base.vt = &MS_MOVE_REMINDER_VT;
        this
    }

    fn new_device(holder: &mut SUMOVehicle, id: &str) -> Self {
        MSDevice_FCD {
            base: MSVehicleDevice::new(holder, id),
        }
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
        let mut mask = SumoXMLAttrMask::default();
        mask.set(SUMO_ATTR_X);
        mask.set(SUMO_ATTR_Y);
        if MSNet::get_instance().has_elevation() {
            mask.set(SUMO_ATTR_Z);
        }
        mask.set(SUMO_ATTR_ANGLE);
        mask.set(SUMO_ATTR_TYPE);
        mask.set(SUMO_ATTR_SPEED);
        mask.set(SUMO_ATTR_POSITION);
        mask.set(SUMO_ATTR_LANE); // for micro vehicles only
        mask.set(SUMO_ATTR_EDGE); // for persons and meso vehicles
        mask.set(SUMO_ATTR_SLOPE);
        if !MSGlobals::use_meso_sim() && get_float("fcd-output.max-leader-distance") > 0.0 {
            mask.set(SUMO_ATTR_LEADER_ID);
            mask.set(SUMO_ATTR_LEADER_SPEED);
            mask.set(SUMO_ATTR_LEADER_GAP);
        }
        mask
    }

    /// C++: `void MSDevice_FCD::MSDevice_FCD(const MSDevice_FCD &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSDevice_FCD) -> Self {
        // the C++ copy constructor is private and has no definition, so it is never called
        unreachable!("MSDevice_FCD copy constructor is private and undefined in C++")
    }

    /// C++: `MSDevice_FCD & MSDevice_FCD::operator=(const MSDevice_FCD &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSDevice_FCD) -> &mut MSDevice_FCD {
        // the C++ assignment operator is private and has no definition, so it is never called
        unreachable!("MSDevice_FCD assignment operator is private and undefined in C++")
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
        if is_set("fcd-output.filter-shapes") {
            let net = MSNet::get_instance();
            let loadedShapes = net.m().get_shape_container();
            if loadedShapes.get_polygons().len() > 0 {
                for attrName in get_string_vector("fcd-output.filter-shapes") {
                    match loadedShapes.get_polygons().get(&attrName).copied() {
                        None => utils::common::msg_handler::write_error(&utils::tlf!("Specified shape '%' for filtering fcd-output could not be found.", attrName)),
                        // store the PositionVector, not reference, as traci can manipulate / detete the polygons
                        Some(poly) => MY_SHAPE4_FILTERS.with(|f| f.borrow_mut().push(poly.get_shape().clone())),
                    }
                }
                MY_SHAPE_FILTER_INITIALIZED.set(true);
            }
        } else {
            MY_SHAPE_FILTER_INITIALIZED.set(true);
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn edge_filter_names_match_stream_extraction() {
        assert_eq!(edge_filter_names("a b\nc"), vec!["a", "b", "c"]);
        // `strm >> name` hits EOF after trailing whitespace and yields an empty name
        assert_eq!(edge_filter_names("a b\n"), vec!["a", "b", ""]);
        assert_eq!(edge_filter_names(""), vec![""]);
        assert_eq!(edge_filter_names("   "), vec![""]);
    }

    #[test]
    fn mask_helpers_mirror_bitset() {
        let mut m = SumoXMLAttrMask::default();
        assert!(!mask_all(m));
        m.set(3);
        assert!(m.test(3));
        mask_reset(&mut m, 3);
        assert!(!m.test(3));
        let mut full = SumoXMLAttrMask((1u128 << SumoXMLAttrMask::SIZE) - 1);
        assert!(mask_all(full));
        mask_reset(&mut full, 95);
        assert!(!mask_all(full));
    }
}

