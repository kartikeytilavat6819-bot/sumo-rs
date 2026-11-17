//! Port of microsim/devices/MSDevice_Emissions.{h,cpp}
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use std::cell::Cell;
use utils::common::std_defs::{g_precision, g_precision_emissions};
use utils::common::time::ts;
use utils::options::options_cont::{get_string_vector, is_set, with_options};
use utils::xml::sumo_xml_definitions::SumoXMLAttrMask;

thread_local! {
    // C++ `myWrittenAttributes(getDefaultMask())`: all attributes written until emission-output.attributes is set
    static MY_WRITTEN_ATTRIBUTES: Cell<SumoXMLAttrMask> = const { Cell::new(SumoXMLAttrMask((1u128 << SumoXMLAttrMask::SIZE) - 1)) };
    static MY_AM_INITIALIZED: Cell<bool> = const { Cell::new(false) };
}

/// A device which collects vehicular emissions
/// C++ class `MSDevice_Emissions` (microsim/devices/MSDevice_Emissions.h)
#[repr(C)]
pub struct MSDevice_Emissions {
    /// C++ base class `MSVehicleDevice`
    pub base: MSVehicleDevice,
    /// Internal storages for pollutant/fuel sum in mg or ml
    pub myEmissions: PollutantsInterface_Emissions,
}

impl std::ops::Deref for MSDevice_Emissions {
    type Target = MSVehicleDevice;
    fn deref(&self) -> &MSVehicleDevice {
        &self.base
    }
}
impl std::ops::DerefMut for MSDevice_Emissions {
    fn deref_mut(&mut self) -> &mut MSVehicleDevice {
        &mut self.base
    }
}

// C++ static members of MSDevice_Emissions (port as module-level statics, see AGENTS.md):
//   static SumoXMLAttrMask MSDevice_Emissions::myWrittenAttributes;
//   static bool MSDevice_Emissions::myAmInitialized;

impl MSDevice_Emissions {
    /// C++: `void MSDevice_Emissions::insertOptions(OptionsCont &)` [static]
    /// Inserts MSDevice_Emissions-options
    // void
    // MSDevice_Emissions::insertOptions(OptionsCont& oc) {
    //     insertDefaultAssignmentOptions("emissions", "Emissions", oc);
    //
    //     oc.doRegister("device.emissions.begin", new Option_String("-1"));
    //     oc.addDescription("device.emissions.begin", "Emissions", TL("Recording begin time for emission-data"));
    //
    //     oc.doRegister("device.emissions.period", new Option_String("0"));
    //     oc.addDescription("device.emissions.period", "Emissions", TL("Recording period for emission-output"));
    // }
    #[allow(unused_variables)]
    pub fn insert_options(oc: &mut OptionsCont) {
        MSDevice::insert_default_assignment_options("emissions", "Emissions", oc, false);

        oc.do_register("device.emissions.begin", SumoOption::string_with("-1"));
        oc.add_description("device.emissions.begin", "Emissions", "Recording begin time for emission-data");

        oc.do_register("device.emissions.period", SumoOption::string_with("0"));
        oc.add_description("device.emissions.period", "Emissions", "Recording period for emission-output");
    }

    /// C++: `void MSDevice_Emissions::buildVehicleDevices(SUMOVehicle &, std::vector<MSVehicleDevice *> &)` [static]
    /// Build devices for the given vehicle, if needed
    // void
    // MSDevice_Emissions::buildVehicleDevices(SUMOVehicle& v, std::vector<MSVehicleDevice*>& into) {
    //     OptionsCont& oc = OptionsCont::getOptions();
    //     if (equippedByDefaultAssignmentOptions(oc, "emissions", v, oc.isSet("emission-output"))) {
    //         into.push_back(new MSDevice_Emissions(v));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn build_vehicle_devices(v: &mut SUMOVehicle, into: &mut Vec<Ptr<MSVehicleDevice>>) {
        let outputOptionSet = is_set("emission-output");
        let equipped = with_options(|oc| {
            MSDevice::equipped_by_default_assignment_options(
                oc,
                "emissions",
                &v.get_id(),
                &v.get_parameter().base,
                &v.get_v_type_parameter().base,
                outputOptionSet,
                false,
            )
        });
        if equipped {
            // the device is owned by the vehicle's device list for the rest of the run, as in C++
            let device: &mut MSDevice_Emissions = Box::leak(Box::new(MSDevice_Emissions::new(v)));
            into.push(Ptr::from_mut(&mut device.base));
        }
    }

    /// C++: `void MSDevice_Emissions::MSDevice_Emissions(SUMOVehicle &)`
    /// Constructor
    // MSDevice_Emissions::MSDevice_Emissions(SUMOVehicle& holder)
    //     : MSVehicleDevice(holder, "emissions_" + holder.getID()), myEmissions() {
    // }
    #[allow(unused_variables)]
    pub fn new(holder: &mut SUMOVehicle) -> Self {
        fn mr_notify_move(mr: &mut MSMoveReminder, veh: &mut SUMOTrafficObject, oldPos: f64, newPos: f64, newSpeed: f64) -> bool {
            Ptr::from_mut(mr).cast::<MSDevice_Emissions>().m().notify_move(veh, oldPos, newPos, newSpeed)
        }
        static MS_MOVE_REMINDER_VT: MSMoveReminderVt = MSMoveReminderVt {
            notify_enter: crate::microsim::ms_move_reminder::default_notify_enter,
            notify_move: mr_notify_move,
            notify_leave: crate::microsim::ms_move_reminder::default_notify_leave,
        };
        fn mr_notify_idle(mr: &mut MSMoveReminder, veh: &mut SUMOTrafficObject) -> bool {
            Ptr::from_mut(mr).cast::<MSDevice_Emissions>().m().notify_idle(veh)
        }
        static MS_MOVE_REMINDER_IDLE_VT: MSMoveReminderIdleVt = MSMoveReminderIdleVt {
            notify_idle: mr_notify_idle,
            notify_parking: crate::microsim::ms_move_reminder::default_notify_parking,
            notify_stop_ended: crate::microsim::ms_move_reminder::default_notify_stop_ended,
        };
        fn mr_notify_move_internal(mr: &mut MSMoveReminder, veh: &SUMOTrafficObject, frontOnLane: f64, timeOnLane: f64, meanSpeedFrontOnLane: f64, meanSpeedVehicleOnLane: f64, travelledDistanceFrontOnLane: f64, travelledDistanceVehicleOnLane: f64, meanLengthOnLane: f64) {
            Ptr::from_mut(mr).cast::<MSDevice_Emissions>().m().notify_move_internal(veh, frontOnLane, timeOnLane, meanSpeedFrontOnLane, meanSpeedVehicleOnLane, travelledDistanceFrontOnLane, travelledDistanceVehicleOnLane, meanLengthOnLane)
        }
        static MS_MOVE_REMINDER_INTERNAL_VT: crate::microsim::ms_move_reminder::MSMoveReminderInternalVt = crate::microsim::ms_move_reminder::MSMoveReminderInternalVt {
            notify_move_internal: mr_notify_move_internal,
        };
        let mut this = Self::new_device(holder);
        this.base.base.vt = &MS_MOVE_REMINDER_VT;
        this.base.base.idle_vt = &MS_MOVE_REMINDER_IDLE_VT;
        this.base.base.internal_vt = &MS_MOVE_REMINDER_INTERNAL_VT;
        this
    }

    fn new_device(holder: &mut SUMOVehicle) -> Self {
        let id = format!("emissions_{}", holder.get_id());
        MSDevice_Emissions {
            base: MSVehicleDevice::new(holder, &id),
            // C++ `myEmissions()` value-initializes all sums to zero
            myEmissions: PollutantsInterface_Emissions { CO2: 0.0, CO: 0.0, HC: 0.0, fuel: 0.0, NOx: 0.0, PMx: 0.0, electricity: 0.0 },
        }
    }

    /// C++: `void MSDevice_Emissions::~MSDevice_Emissions() noexcept` [virtual]
    /// Destructor.
    // MSDevice_Emissions::~MSDevice_Emissions() {
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty

    }

    /// C++: `bool MSDevice_Emissions::notifyMove(SUMOTrafficObject &, double, double, double)` [virtual]
    /// Computes current emission values and adds them to their sums
    // bool
    // MSDevice_Emissions::notifyMove(SUMOTrafficObject& veh, double /*oldPos*/, double /*newPos*/, double newSpeed) {
    //     const SUMOEmissionClass c = veh.getVehicleType().getEmissionClass();
    //     myEmissions.addScaled(PollutantsInterface::computeAll(c, newSpeed, veh.getAcceleration(), veh.getSlope(), myHolder.getEmissionParameters()), TS);
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn notify_move(&mut self, veh: &mut SUMOTrafficObject, oldPos: f64, newPos: f64, newSpeed: f64) -> bool {
        let c = veh.get_vehicle_type().get_emission_class();
        let computed = PollutantsInterface::compute_all(c, newSpeed, veh.get_acceleration(), veh.get_slope(), self.myHolder.get_emission_parameters());
        self.myEmissions.add_scaled(&computed, ts());
        true
    }

    /// C++: `bool MSDevice_Emissions::notifyIdle(SUMOTrafficObject &)` [virtual]
    /// Computes idling emission values and adds them to the emission sums
    // bool
    // MSDevice_Emissions::notifyIdle(SUMOTrafficObject& veh) {
    //     const SUMOEmissionClass c = veh.getVehicleType().getEmissionClass();
    //     myEmissions.addScaled(PollutantsInterface::computeAll(c, 0., 0., 0., myHolder.getEmissionParameters()), TS);
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn notify_idle(&mut self, veh: &mut SUMOTrafficObject) -> bool {
        let c = veh.get_vehicle_type().get_emission_class();
        let computed = PollutantsInterface::compute_all(c, 0., 0., 0., self.myHolder.get_emission_parameters());
        self.myEmissions.add_scaled(&computed, ts());
        true
    }

    /// C++: `const std::string MSDevice_Emissions::deviceName() const` [const, virtual]
    /// return the name for this type of device
    // const std::string deviceName() const {
    //         return "emissions";
    //     }
    #[allow(unused_variables)]
    pub fn device_name(&self) -> String {
        "emissions".to_string()
    }

    /// C++: `void MSDevice_Emissions::generateOutput(OutputDevice *) const` [const, virtual]
    /// Called on writing tripinfo output
    // void
    // MSDevice_Emissions::generateOutput(OutputDevice* tripinfoOut) const {
    //     if (tripinfoOut != nullptr) {
    //         tripinfoOut->setPrecision(gPrecisionEmissions);
    //         tripinfoOut->openTag("emissions");
    //         tripinfoOut->writeAttr(SUMO_ATTR_CO_ABS, myEmissions.CO);
    //         tripinfoOut->writeAttr(SUMO_ATTR_CO2_ABS, myEmissions.CO2);
    //         tripinfoOut->writeAttr(SUMO_ATTR_HC_ABS, myEmissions.HC);
    //         tripinfoOut->writeAttr(SUMO_ATTR_PMX_ABS, myEmissions.PMx);
    //         tripinfoOut->writeAttr(SUMO_ATTR_NOX_ABS, myEmissions.NOx);
    //         tripinfoOut->writeAttr(SUMO_ATTR_FUEL_ABS, myEmissions.fuel);
    //         tripinfoOut->writeAttr(SUMO_ATTR_ELECTRICITY_ABS, myEmissions.electricity);
    //         tripinfoOut->closeTag();
    //         tripinfoOut->setPrecision(gPrecision);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn generate_output(&self, tripinfoOut: Ptr<OutputDevice>) {
        if !tripinfoOut.is_null() {
            let out = tripinfoOut.m();
            out.set_precision(g_precision_emissions());
            out.open_tag("emissions");
            out.write_attr(SUMO_ATTR_CO_ABS, &self.myEmissions.CO);
            out.write_attr(SUMO_ATTR_CO2_ABS, &self.myEmissions.CO2);
            out.write_attr(SUMO_ATTR_HC_ABS, &self.myEmissions.HC);
            out.write_attr(SUMO_ATTR_PMX_ABS, &self.myEmissions.PMx);
            out.write_attr(SUMO_ATTR_NOX_ABS, &self.myEmissions.NOx);
            out.write_attr(SUMO_ATTR_FUEL_ABS, &self.myEmissions.fuel);
            out.write_attr(SUMO_ATTR_ELECTRICITY_ABS, &self.myEmissions.electricity);
            out.close_tag("");
            out.set_precision(g_precision());
        }
    }

    /// C++: `SumoXMLAttrMask MSDevice_Emissions::getWrittenAttributes()` [static]
    // static SumoXMLAttrMask getWrittenAttributes() {
    //         return myWrittenAttributes;
    //     }
    #[allow(unused_variables)]
    pub fn get_written_attributes() -> SumoXMLAttrMask {
        MY_WRITTEN_ATTRIBUTES.with(|c| c.get())
    }

    /// C++: `void MSDevice_Emissions::initOnce()` [static]
    /// initialize attribute mask (once)
    // void
    // MSDevice_Emissions::initOnce() {
    //     if (myAmInitialized) {
    //         return;
    //     }
    //     myAmInitialized = true;
    //     const OptionsCont& oc = OptionsCont::getOptions();
    //     if (oc.isSet("emission-output.attributes")) {
    //         myWrittenAttributes.reset();
    //         for (std::string attrName : oc.getStringVector("emission-output.attributes")) {
    //             if (!SUMOXMLDefinitions::Attrs.hasString(attrName)) {
    //                 if (attrName == "all") {
    //                     myWrittenAttributes.set();
    //                 } else {
    //                     WRITE_ERRORF(TL("Unknown attribute '%' to write in emission output."), attrName);
    //                 }
    //                 continue;
    //             }
    //             int attr = SUMOXMLDefinitions::Attrs.get(attrName);
    //             myWrittenAttributes.set(attr);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn init_once() {
        if MY_AM_INITIALIZED.get() {
            return;
        }
        MY_AM_INITIALIZED.set(true);
        if is_set("emission-output.attributes") {
            let mut written = SumoXMLAttrMask::default();
            for attr_name in get_string_vector("emission-output.attributes") {
                if !Attrs.has_string(&attr_name) {
                    if attr_name == "all" {
                        written.set_all();
                    } else {
                        utils::write_errorf!("Unknown attribute '%' to write in emission output.", attr_name);
                    }
                    continue;
                }
                let attr = Attrs.get(&attr_name).unwrap_or_else(|e| panic!("{e}"));
                written.set(attr);
            }
            MY_WRITTEN_ATTRIBUTES.set(written);
        }
    }

    /// C++: `void MSDevice_Emissions::cleanup()` [static]
    /// resets the attribute mask
    // void
    // MSDevice_Emissions::cleanup() {
    //     myWrittenAttributes = getDefaultMask();
    //     myAmInitialized = false;
    // }
    #[allow(unused_variables)]
    pub fn cleanup() {
        MY_WRITTEN_ATTRIBUTES.set(MSDevice_Emissions::get_default_mask());
        MY_AM_INITIALIZED.set(false);
    }

    /// C++: `void MSDevice_Emissions::notifyMoveInternal(const SUMOTrafficObject &, const double, const double, const double, const double, const double, const double, const double)` [virtual, protected]
    /// Internal notification about the vehicle moves, see MSMoveReminder::notifyMoveInternal()
    // void
    // MSDevice_Emissions::notifyMoveInternal(const SUMOTrafficObject& veh,
    //                                        const double /* frontOnLane */,
    //                                        const double timeOnLane,
    //                                        const double /* meanSpeedFrontOnLane */,
    //                                        const double meanSpeedVehicleOnLane,
    //                                        const double /* travelledDistanceFrontOnLane */,
    //                                        const double /* travelledDistanceVehicleOnLane */,
    //                                        const double /* meanLengthOnLane */) {
    //
    //     // called by meso (see MSMeanData_Emissions::MSLaneMeanDataValues::notifyMoveInternal)
    //     const SUMOEmissionClass c = veh.getVehicleType().getEmissionClass();
    //     myEmissions.addScaled(PollutantsInterface::computeAll(c, meanSpeedVehicleOnLane, veh.getAcceleration(), veh.getSlope(), myHolder.getEmissionParameters()), timeOnLane);
    // }
    #[allow(unused_variables)]
    pub fn notify_move_internal(&mut self, veh: &SUMOTrafficObject, frontOnLane: f64, timeOnLane: f64, meanSpeedFrontOnLane: f64, meanSpeedVehicleOnLane: f64, travelledDistanceFrontOnLane: f64, travelledDistanceVehicleOnLane: f64, meanLengthOnLane: f64) {
        // called by meso (see MSMeanData_Emissions::MSLaneMeanDataValues::notifyMoveInternal)
        let c = veh.get_vehicle_type().get_emission_class();
        let computed = PollutantsInterface::compute_all(c, meanSpeedVehicleOnLane, veh.get_acceleration(), veh.get_slope(), self.myHolder.get_emission_parameters());
        self.myEmissions.add_scaled(&computed, timeOnLane);
    }

    /// C++: `SumoXMLAttrMask MSDevice_Emissions::getDefaultMask()` [static, private]
    // SumoXMLAttrMask
    // MSDevice_Emissions::getDefaultMask() {
    //     SumoXMLAttrMask mask;
    //     // set all bits to 1
    //     mask.set();
    //     return mask;
    // }
    #[allow(unused_variables)]
    pub fn get_default_mask() -> SumoXMLAttrMask {
        // set all bits to 1
        let mut mask = SumoXMLAttrMask::default();
        mask.set_all();
        mask
    }

    /// C++: `void MSDevice_Emissions::MSDevice_Emissions(const MSDevice_Emissions &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSDevice_Emissions) -> Self {
        // the C++ copy constructor is private and has no definition, so it is never called
        unreachable!("MSDevice_Emissions copy constructor is private and undefined in C++")
    }

    /// C++: `MSDevice_Emissions & MSDevice_Emissions::operator=(const MSDevice_Emissions &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSDevice_Emissions) -> &mut MSDevice_Emissions {
        // the C++ assignment operator is private and has no definition, so it is never called
        unreachable!("MSDevice_Emissions assignment operator is private and undefined in C++")
    }

}

