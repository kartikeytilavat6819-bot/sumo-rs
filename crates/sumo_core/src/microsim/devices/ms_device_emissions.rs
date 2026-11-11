//! Port of microsim/devices/MSDevice_Emissions.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// A device which collects vehicular emissions
/// C++ class `MSDevice_Emissions` (microsim/devices/MSDevice_Emissions.h)
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
        todo!("port MSDevice_Emissions::insertOptions")
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
        todo!("port MSDevice_Emissions::buildVehicleDevices")
    }

    /// C++: `void MSDevice_Emissions::MSDevice_Emissions(SUMOVehicle &)`
    /// Constructor
    // MSDevice_Emissions::MSDevice_Emissions(SUMOVehicle& holder)
    //     : MSVehicleDevice(holder, "emissions_" + holder.getID()), myEmissions() {
    // }
    #[allow(unused_variables)]
    pub fn new(holder: &mut SUMOVehicle) -> Self {
        todo!("port MSDevice_Emissions::MSDevice_Emissions")
    }

    /// C++: `void MSDevice_Emissions::~MSDevice_Emissions() noexcept` [virtual]
    /// Destructor.
    // MSDevice_Emissions::~MSDevice_Emissions() {
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MSDevice_Emissions::~MSDevice_Emissions")
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
        todo!("port MSDevice_Emissions::notifyMove")
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
        todo!("port MSDevice_Emissions::notifyIdle")
    }

    /// C++: `const std::string MSDevice_Emissions::deviceName() const` [const, virtual]
    /// return the name for this type of device
    // const std::string deviceName() const {
    //         return "emissions";
    //     }
    #[allow(unused_variables)]
    pub fn device_name(&self) -> String {
        todo!("port MSDevice_Emissions::deviceName")
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
        todo!("port MSDevice_Emissions::generateOutput")
    }

    /// C++: `SumoXMLAttrMask MSDevice_Emissions::getWrittenAttributes()` [static]
    // static SumoXMLAttrMask getWrittenAttributes() {
    //         return myWrittenAttributes;
    //     }
    #[allow(unused_variables)]
    pub fn get_written_attributes() -> SumoXMLAttrMask {
        todo!("port MSDevice_Emissions::getWrittenAttributes")
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
        todo!("port MSDevice_Emissions::initOnce")
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
        todo!("port MSDevice_Emissions::cleanup")
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
        todo!("port MSDevice_Emissions::notifyMoveInternal")
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
        todo!("port MSDevice_Emissions::getDefaultMask")
    }

    /// C++: `void MSDevice_Emissions::MSDevice_Emissions(const MSDevice_Emissions &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSDevice_Emissions) -> Self {
        todo!("port MSDevice_Emissions::MSDevice_Emissions")
    }

    /// C++: `MSDevice_Emissions & MSDevice_Emissions::operator=(const MSDevice_Emissions &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSDevice_Emissions) -> &mut MSDevice_Emissions {
        todo!("port MSDevice_Emissions::operator=")
    }

}

