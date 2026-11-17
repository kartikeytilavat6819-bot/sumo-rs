//! Port of microsim/devices/MSDevice_Battery.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::to_string::to_string;
use utils::common::string_utils::to_double;
use utils::options::options_cont::{get_bool, with_options};

/// Battery device for electric vehicles
/// C++ class `MSDevice_Battery` (microsim/devices/MSDevice_Battery.h)
#[repr(C)]
pub struct MSDevice_Battery {
    /// C++ base class `MSVehicleDevice`
    pub base: MSVehicleDevice,
    /// Parameter, The actual vehicles's Battery Capacity in Wh, [myActualBatteryCapacity <= myMaximumBatteryCapacity]
    pub myActualBatteryCapacity: f64,
    /// Parameter, The total vehicles's Battery Capacity in Wh, [myMaximumBatteryCapacity >= 0]
    pub myMaximumBatteryCapacity: f64,
    /// Parameter, stopping vehicle threshold [myStoppingThreshold >= 0]
    pub myStoppingThreshold: f64,
    /// Parameter, maximum charge rate in W
    pub myMaximumChargeRate: f64,
    /// (Temporary) limitation in W of the maximum charge rate = charging strategy result
    pub myChargeLimit: f64,
    /// Parameter, Vehicle's last angle
    pub myLastAngle: f64,
    /// Parameter, Flag: Vehicles it's charging stopped (by default is false)
    pub myChargingStopped: bool,
    /// Parameter, Flag: Vehicles it's charging in transit (by default is false)
    pub myChargingInTransit: bool,
    /// Parameter, Moment, which the vehicle has beging to charging
    pub myChargingStartTime: SUMOTime,
    /// Parameter, Vehicle consum during a time step (by default is 0.)
    pub myConsum: f64,
    /// Parameter, total vehicle energy consumption
    pub myTotalConsumption: f64,
    /// Parameter, total vehicle energy regeneration
    pub myTotalRegenerated: f64,
    /// Charge curve data points storage
    pub myChargeCurve: LinearApproxMap,
    /// Parameter, Pointer to current charging station in which vehicle is placed (by default is NULL)
    pub myActChargingStation: Ptr<MSChargingStation>,
    /// Parameter, Pointer to charging station neighbouring with myActChargingStation in which vehicle was placed previously (by default is NULL), i.e. auxiliar pointer for disabling charging vehicle from previous (not current) ChargingStation (if there is no gap between two different chargingStations)
    pub myPreviousNeighbouringChargingStation: Ptr<MSChargingStation>,
    /// Parameter, Energy charged in each timestep
    pub myEnergyCharged: f64,
    /// Parameter, How many timestep the vehicle is stopped
    pub myVehicleStopped: i32,
    /// Count how many times the vehicle experienced a depleted battery
    pub myDepletedCount: i32,
    /// whether to track fuel consumption instead of electricity
    pub myTrackFuel: bool,
    /// the accepted charge type
    pub myChargeType: MSChargingStation_ChargeType,
}

impl std::ops::Deref for MSDevice_Battery {
    type Target = MSVehicleDevice;
    fn deref(&self) -> &MSVehicleDevice {
        &self.base
    }
}
impl std::ops::DerefMut for MSDevice_Battery {
    fn deref_mut(&mut self) -> &mut MSVehicleDevice {
        &mut self.base
    }
}

/// C++ `MIN2` macro
fn min2(a: f64, b: f64) -> f64 {
    if a < b { a } else { b }
}

/// C++ `toString(SumoXMLAttr)`: the XML name of the attribute
fn attr_str(attr: SumoXMLAttr) -> String {
    utils::xml::sumo_xml_definitions::Attrs.get_string(attr).to_string()
}

/// C++ `DEFAULT_MAX_CAPACITY` (MSDevice_Battery.cpp)
const DEFAULT_MAX_CAPACITY: f64 = 35000.;
/// C++ `DEFAULT_CHARGE_RATIO` (MSDevice_Battery.cpp)
const DEFAULT_CHARGE_RATIO: f64 = 0.5;
/// C++ `VTYPEPARS_EMISSIONCLASS_SET` bit (private in ms_vehicle_type.rs, same value)
const VTYPEPARS_EMISSIONCLASS_SET: i64 = 1 << 5;

/// C++ failed `istringstream`: every later extraction is a no-op, so the rest of the tokens are dropped
fn stream_fail(bis: &mut std::str::SplitWhitespace) {
    while bis.next().is_some() {}
}

/// C++ `istringstream >> double`: a bad or missing token sets failbit (value 0 for a bad token)
fn read_stream_f64(bis: &mut std::str::SplitWhitespace, target: &mut f64) {
    match bis.next().map(|t| t.parse::<f64>()) {
        Some(Ok(v)) => *target = v,
        Some(Err(_)) => {
            *target = 0.;
            stream_fail(bis);
        }
        None => stream_fail(bis),
    }
}

/// C++ `istringstream >> SUMOTime`: a bad or missing token sets failbit (value 0 for a bad token)
fn read_stream_i64(bis: &mut std::str::SplitWhitespace, target: &mut i64) {
    match bis.next().map(|t| t.parse::<i64>()) {
        Some(Ok(v)) => *target = v,
        Some(Err(_)) => {
            *target = 0;
            stream_fail(bis);
        }
        None => stream_fail(bis),
    }
}

/// C++ `istringstream >> int`: a bad or missing token sets failbit (value 0 for a bad token)
fn read_stream_i32(bis: &mut std::str::SplitWhitespace, target: &mut i32) {
    match bis.next().map(|t| t.parse::<i32>()) {
        Some(Ok(v)) => *target = v,
        Some(Err(_)) => {
            *target = 0;
            stream_fail(bis);
        }
        None => stream_fail(bis),
    }
}

/// C++ `istringstream >> bool` (written by toString as 1/0): 0 and 1 are read, other values set failbit and read as true
fn read_stream_bool(bis: &mut std::str::SplitWhitespace, target: &mut bool) {
    match bis.next().map(|t| t.parse::<i64>()) {
        Some(Ok(0)) => *target = false,
        Some(Ok(1)) => *target = true,
        Some(Ok(_)) => {
            *target = true;
            stream_fail(bis);
        }
        Some(Err(_)) => {
            *target = false;
            stream_fail(bis);
        }
        None => stream_fail(bis),
    }
}

/// C++ `LinearApproxHelpers::getValueTable`
fn get_value_table(dataString: &str) -> Vec<f64> {
    let mut result = Vec::new();
    if !dataString.is_empty() {
        let mut st = StringTokenizer::new(dataString);
        while st.has_next() {
            let value = st.next().unwrap_or_else(|e| panic!("{}", e));
            result.push(to_double(&value).unwrap_or_else(|e| panic!("{}", e)));
        }
    }
    result
}

/// C++ `LinearApproxHelpers::setPoints` (std::map::insert: an existing key is kept)
fn set_points(map: &mut LinearApproxMap, axisString: &str, heightString: &str) {
    let axisData = get_value_table(axisString);
    let heightData = get_value_table(heightString);
    if heightData.len() > 0 && heightData.len() != axisData.len() {
        panic!("{}", ProcessError::new(format!("Mismatching data rows of {} axis and {} height values.", axisData.len(), heightData.len())));
    } else {
        for (a, h) in axisData.iter().zip(heightData.iter()) {
            if let Err(pos) = map.binary_search_by(|(k, _)| k.partial_cmp(a).unwrap()) {
                map.insert(pos, (*a, *h));
            }
        }
    }
}

/// C++ `LinearApproxHelpers::getMaximumValue`
fn get_maximum_value(map: &LinearApproxMap) -> f64 {
    if map.is_empty() {
        panic!("{}", ProcessError::new("Cannot determine the maximum value from an empty map."));
    }
    let mut maxValue = f64::MIN_POSITIVE; // std::numeric_limits<double>::min()
    for item in map.iter() {
        if item.1 > maxValue {
            maxValue = item.1;
        }
    }
    maxValue
}

impl MSDevice_Battery {
    /// C++: `void MSDevice_Battery::insertOptions(OptionsCont &)` [static]
    /// Inserts MSDevice_Example-options
    // void
    // MSDevice_Battery::insertOptions(OptionsCont& oc) {
    //     insertDefaultAssignmentOptions("battery", "Battery", oc);
    //     // custom options
    //     oc.doRegister("device.battery.track-fuel", new Option_Bool(false));
    //     oc.addDescription("device.battery.track-fuel", "Battery", TL("Track fuel consumption for non-electric vehicles"));
    // }
    #[allow(unused_variables)]
    pub fn insert_options(oc: &mut OptionsCont) {
        MSDevice::insert_default_assignment_options("battery", "Battery", oc, false);
        // custom options
        oc.do_register("device.battery.track-fuel", SumoOption::bool(false));
        oc.add_description("device.battery.track-fuel", "Battery", "Track fuel consumption for non-electric vehicles");
    }

    /// C++: `void MSDevice_Battery::buildVehicleDevices(SUMOVehicle &, std::vector<MSVehicleDevice *> &, MSDevice_StationFinder *)` [static]
    /// Build devices for the given vehicle, if needed
    // void
    // MSDevice_Battery::buildVehicleDevices(SUMOVehicle& v, std::vector<MSVehicleDevice*>& into, MSDevice_StationFinder* sf) {
    //     // Check if vehicle should get a battery
    //     if (sf != nullptr || equippedByDefaultAssignmentOptions(OptionsCont::getOptions(), "battery", v, false)) {
    //         // obtain parameter values
    //         const double maximumBatteryCapacity = readParameterValue(v, SUMO_ATTR_MAXIMUMBATTERYCAPACITY, "battery.capacity", DEFAULT_MAX_CAPACITY);
    //         const double actualBatteryCapacity = readParameterValue(v, SUMO_ATTR_ACTUALBATTERYCAPACITY, "battery.chargeLevel", maximumBatteryCapacity * DEFAULT_CHARGE_RATIO);
    //         const double stoppingThreshold = readParameterValue(v, SUMO_ATTR_STOPPINGTHRESHOLD, "battery.stoppingThreshold", 0.1);
    //         const double maximumChargeRate = readParameterValue(v, SUMO_ATTR_MAXIMUMCHARGERATE, "battery.maximumChargeRate", 150000.);
    //         const std::string chargeLevelTable = v.getStringParam("device.battery.chargeLevelTable");
    //         const std::string chargeCurveTable = v.getStringParam("device.battery.chargeCurveTable");
    //
    //         // battery constructor
    //         MSDevice_Battery* device = new MSDevice_Battery(v, "battery_" + v.getID(),
    //                 actualBatteryCapacity, maximumBatteryCapacity, stoppingThreshold, maximumChargeRate, chargeLevelTable, chargeCurveTable);
    //
    //         // Add device to vehicle
    //         into.push_back(device);
    //
    //         if (sf != nullptr) {
    //             sf->setBattery(device);
    //         }
    //         // ensure that parameters are initialized
    //         v.getEmissionParameters();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn build_vehicle_devices(v: &mut SUMOVehicle, into: &mut Vec<Ptr<MSVehicleDevice>>, sf: Ptr<MSDevice_StationFinder>) {
        // Check if vehicle should get a battery
        let equipped = !sf.is_null() || with_options(|oc| {
            MSDevice::equipped_by_default_assignment_options(oc, "battery", v.get_id(), v.get_parameter(), v.get_vehicle_type().get_parameter(), false, false)
        });
        if equipped {
            // obtain parameter values
            let maximumBatteryCapacity = MSDevice_Battery::read_parameter_value(v, &SUMO_ATTR_MAXIMUMBATTERYCAPACITY, "battery.capacity", DEFAULT_MAX_CAPACITY);
            let actualBatteryCapacity = MSDevice_Battery::read_parameter_value(v, &SUMO_ATTR_ACTUALBATTERYCAPACITY, "battery.chargeLevel", maximumBatteryCapacity * DEFAULT_CHARGE_RATIO);
            let stoppingThreshold = MSDevice_Battery::read_parameter_value(v, &SUMO_ATTR_STOPPINGTHRESHOLD, "battery.stoppingThreshold", 0.1);
            let maximumChargeRate = MSDevice_Battery::read_parameter_value(v, &SUMO_ATTR_MAXIMUMCHARGERATE, "battery.maximumChargeRate", 150000.);
            let chargeLevelTable = v.get_string_param("device.battery.chargeLevelTable", false, "");
            let chargeCurveTable = v.get_string_param("device.battery.chargeCurveTable", false, "");

            // battery constructor
            let id = format!("battery_{}", v.get_id());
            // shortcut: the device is leaked like the C++ new, so the vehicle's device list keeps a stable pointer; upgrade when MSVehicle owns its devices as Box
            let device: &mut MSDevice_Battery = Box::leak(Box::new(MSDevice_Battery::new(v, &id, actualBatteryCapacity, maximumBatteryCapacity, stoppingThreshold, maximumChargeRate, &chargeLevelTable, &chargeCurveTable)));

            // Add device to vehicle
            into.push(Ptr::from_mut(&mut device.base));

            if !sf.is_null() {
                sf.m().set_battery(Ptr::from_mut(device));
            }
            // ensure that parameters are initialized
            let _ = Ptr::from_mut(v).cast::<MSBaseVehicle>().m().get_emission_parameters();
        }
    }

    /// C++: `void MSDevice_Battery::~MSDevice_Battery() noexcept` [virtual]
    /// Destructor.
    // MSDevice_Battery::~MSDevice_Battery() {
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
    }

    /// C++: `bool MSDevice_Battery::notifyMove(SUMOTrafficObject &, double, double, double)` [virtual]
    /// Checks for waiting steps when the vehicle moves
    // bool MSDevice_Battery::notifyMove(SUMOTrafficObject& tObject, double /* oldPos */, double /* newPos */, double newSpeed) {
    //     if (!tObject.isVehicle()) {
    //         return false;
    //     }
    //     notifyMoveInternal(tObject, 0., TS, 0., newSpeed, 0., 0., 0.);
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn notify_move(&mut self, tObject: &mut SUMOTrafficObject, oldPos: f64, newPos: f64, newSpeed: f64) -> bool {
        if !tObject.is_vehicle() {
            return false;
        }
        self.notify_move_internal(&*tObject, 0., ts(), 0., newSpeed, 0., 0., 0.);
        true
    }

    /// C++: `const std::string MSDevice_Battery::deviceName() const` [const, virtual]
    /// return the name for this type of device
    // const std::string deviceName() const override {
    //         return "battery";
    //     }
    #[allow(unused_variables)]
    pub fn device_name(&self) -> String {
        "battery".to_string()
    }

    /// C++: `void MSDevice_Battery::saveState(OutputDevice &) const` [const, virtual]
    /// Saves the state of the device
    // void
    // MSDevice_Battery::saveState(OutputDevice& out) const {
    //     out.openTag(SUMO_TAG_DEVICE);
    //     out.writeAttr(SUMO_ATTR_ID, getID());
    //     std::vector<std::string> internals;
    //     internals.push_back(toString(myActualBatteryCapacity));
    //     internals.push_back(toString(myLastAngle));
    //     internals.push_back(toString(myChargingStopped));
    //     internals.push_back(toString(myChargingInTransit));
    //     internals.push_back(toString(myChargingStartTime));
    //     internals.push_back(toString(myTotalConsumption));
    //     internals.push_back(toString(myTotalRegenerated));
    //     internals.push_back(toString(myEnergyCharged));
    //     internals.push_back(toString(myVehicleStopped));
    //     internals.push_back(getChargingStationID());
    //     std::string prevChargingID = (myPreviousNeighbouringChargingStation == nullptr) ? "NULL" : myPreviousNeighbouringChargingStation->getID();
    //     internals.push_back(prevChargingID);
    //     internals.push_back(toString(myMaximumChargeRate));
    //     out.writeAttr(SUMO_ATTR_STATE, toString(internals));
    //     out.closeTag();
    // }
    #[allow(unused_variables)]
    pub fn save_state(&self, out: &mut OutputDevice) {
        out.open_tag_id(SUMO_TAG_DEVICE);
        out.write_attr(SUMO_ATTR_ID, self.base.base2.get_id());
        let mut internals: Vec<String> = Vec::new();
        internals.push(to_string(&self.myActualBatteryCapacity));
        internals.push(to_string(&self.myLastAngle));
        internals.push(to_string(&self.myChargingStopped));
        internals.push(to_string(&self.myChargingInTransit));
        internals.push(to_string(&self.myChargingStartTime));
        internals.push(to_string(&self.myTotalConsumption));
        internals.push(to_string(&self.myTotalRegenerated));
        internals.push(to_string(&self.myEnergyCharged));
        internals.push(to_string(&self.myVehicleStopped));
        internals.push(self.get_charging_station_id());
        let prevChargingID = if self.myPreviousNeighbouringChargingStation.is_null() { "NULL".to_string() } else { self.myPreviousNeighbouringChargingStation.get_id().to_string() };
        internals.push(prevChargingID);
        internals.push(to_string(&self.myMaximumChargeRate));
        out.write_attr(SUMO_ATTR_STATE, &to_string(&internals));
        out.close_tag("");
    }

    /// C++: `void MSDevice_Battery::loadState(const SUMOSAXAttributes &)` [virtual]
    /// Loads the state of the device from the given description
    // void
    // MSDevice_Battery::loadState(const SUMOSAXAttributes& attrs) {
    //     std::istringstream bis(attrs.getString(SUMO_ATTR_STATE));
    //     bis >> myActualBatteryCapacity;
    //     bis >> myLastAngle;
    //     bis >> myChargingStopped;
    //     bis >> myChargingInTransit;
    //     bis >> myChargingStartTime;
    //     bis >> myTotalConsumption;
    //     bis >> myTotalRegenerated;
    //     bis >> myEnergyCharged;
    //     bis >> myVehicleStopped;
    //     std::string chargingID;
    //     bis >> chargingID;
    //     if (chargingID != "NULL") {
    //         myActChargingStation = dynamic_cast<MSChargingStation*>(MSNet::getInstance()->getStoppingPlace(chargingID, SUMO_TAG_CHARGING_STATION));
    //     }
    //     std::string prevChargingID;
    //     bis >> prevChargingID;
    //     if (prevChargingID != "NULL") {
    //         myPreviousNeighbouringChargingStation = dynamic_cast<MSChargingStation*>(MSNet::getInstance()->getStoppingPlace(prevChargingID, SUMO_TAG_CHARGING_STATION));
    //     }
    //     bis >> myMaximumChargeRate;
    // }
    #[allow(unused_variables)]
    pub fn load_state(&mut self, attrs: &SUMOSAXAttributes) {
        let state = attrs.get_string(SUMO_ATTR_STATE);
        let mut bis = state.split_whitespace();
        read_stream_f64(&mut bis, &mut self.myActualBatteryCapacity);
        read_stream_f64(&mut bis, &mut self.myLastAngle);
        read_stream_bool(&mut bis, &mut self.myChargingStopped);
        read_stream_bool(&mut bis, &mut self.myChargingInTransit);
        read_stream_i64(&mut bis, &mut self.myChargingStartTime);
        read_stream_f64(&mut bis, &mut self.myTotalConsumption);
        read_stream_f64(&mut bis, &mut self.myTotalRegenerated);
        read_stream_f64(&mut bis, &mut self.myEnergyCharged);
        read_stream_i32(&mut bis, &mut self.myVehicleStopped);
        let chargingID = bis.next().unwrap_or("").to_string();
        if chargingID != "NULL" {
            self.myActChargingStation = MSNet::get_instance().get_stopping_place(&chargingID, SUMO_TAG_CHARGING_STATION).cast::<MSChargingStation>();
        }
        let prevChargingID = bis.next().unwrap_or("").to_string();
        if prevChargingID != "NULL" {
            self.myPreviousNeighbouringChargingStation = MSNet::get_instance().get_stopping_place(&prevChargingID, SUMO_TAG_CHARGING_STATION).cast::<MSChargingStation>();
        }
        read_stream_f64(&mut bis, &mut self.myMaximumChargeRate);
    }

    /// C++: `std::string MSDevice_Battery::getParameter(const std::string &) const` [const, virtual]
    /// try to retrieve the given parameter from this device. Throw exception for unsupported key
    // std::string
    // MSDevice_Battery::getParameter(const std::string& key) const {
    //     if (key == toString(SUMO_ATTR_ACTUALBATTERYCAPACITY)
    //             || key == toString(SUMO_ATTR_CHARGELEVEL)) {
    //         return toString(getActualBatteryCapacity());
    //     } else if (key == toString(SUMO_ATTR_ENERGYCONSUMED)) {
    //         return toString(getConsum());
    //     } else if (key == "chargePower") {
    //         return toString(getEnergyCharged() * 3600.);
    //     } else if (key == "usedAverage") {
    //         return toString(getTotalConsumption() * 3600 / STEPS2TIME(SIMSTEP - myHolder.getDeparture()));
    //     } else if (key == toString(SUMO_ATTR_TOTALENERGYCONSUMED)) {
    //         return toString(getTotalConsumption());
    //     } else if (key == toString(SUMO_ATTR_TOTALENERGYREGENERATED)) {
    //         return toString(getTotalRegenerated());
    //     } else if (key == toString(SUMO_ATTR_ENERGYCHARGED)) {
    //         return toString(getEnergyCharged());
    //     } else if (key == toString(SUMO_ATTR_MAXIMUMBATTERYCAPACITY) || key == "capacity") {
    //         return toString(getMaximumBatteryCapacity());
    //     } else if (key == toString(SUMO_ATTR_MAXIMUMCHARGERATE)) {
    //         return toString(getMaximumChargeRate());
    //     } else if (key == toString(SUMO_ATTR_CHARGINGSTATIONID)) {
    //         return getChargingStationID();
    //     }
    //     throw InvalidArgument("Parameter '" + key + "' is not supported for device of type '" + deviceName() + "'");
    // }
    #[allow(unused_variables)]
    pub fn get_parameter(&self, key: &str) -> String {
        // shortcut: C++ throws InvalidArgument; the port panics with the same text, as MSDevice::getParameter does
        if key == attr_str(SUMO_ATTR_ACTUALBATTERYCAPACITY) || key == attr_str(SUMO_ATTR_CHARGELEVEL) {
            to_string(&self.get_actual_battery_capacity())
        } else if key == attr_str(SUMO_ATTR_ENERGYCONSUMED) {
            to_string(&self.get_consum())
        } else if key == "chargePower" {
            to_string(&(self.get_energy_charged() * 3600.))
        } else if key == "usedAverage" {
            to_string(&(self.get_total_consumption() * 3600. / steps2time(MSNet::get_instance().get_current_time_step() - self.get_holder().get_departure())))
        } else if key == attr_str(SUMO_ATTR_TOTALENERGYCONSUMED) {
            to_string(&self.get_total_consumption())
        } else if key == attr_str(SUMO_ATTR_TOTALENERGYREGENERATED) {
            to_string(&self.get_total_regenerated())
        } else if key == attr_str(SUMO_ATTR_ENERGYCHARGED) {
            to_string(&self.get_energy_charged())
        } else if key == attr_str(SUMO_ATTR_MAXIMUMBATTERYCAPACITY) || key == "capacity" {
            to_string(&self.get_maximum_battery_capacity())
        } else if key == attr_str(SUMO_ATTR_MAXIMUMCHARGERATE) {
            to_string(&self.get_maximum_charge_rate())
        } else if key == attr_str(SUMO_ATTR_CHARGINGSTATIONID) {
            self.get_charging_station_id()
        } else {
            panic!("Parameter '{}' is not supported for device of type '{}'", key, self.device_name())
        }
    }

    /// C++: `void MSDevice_Battery::setParameter(const std::string &, const std::string &)` [virtual]
    /// try to set the given parameter for this device. Throw exception for unsupported key
    // void
    // MSDevice_Battery::setParameter(const std::string& key, const std::string& value) {
    //     double doubleValue;
    //     try {
    //         doubleValue = StringUtils::toDouble(value);
    //     } catch (NumberFormatException&) {
    //         throw InvalidArgument("Setting parameter '" + key + "' requires a number for device of type '" + deviceName() + "'");
    //     }
    //     if (key == toString(SUMO_ATTR_ACTUALBATTERYCAPACITY) || key == toString(SUMO_ATTR_CHARGELEVEL)) {
    //         setActualBatteryCapacity(doubleValue);
    //     } else if (key == toString(SUMO_ATTR_MAXIMUMBATTERYCAPACITY) || key == "capacity") {
    //         setMaximumBatteryCapacity(doubleValue);
    //     } else if (key == toString(SUMO_ATTR_MAXIMUMCHARGERATE)) {
    //         setMaximumChargeRate(doubleValue);
    //     } else {
    //         throw InvalidArgument("Setting parameter '" + key + "' is not supported for device of type '" + deviceName() + "'");
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_parameter(&mut self, key: &str, value: &str) {
        let doubleValue = match to_double(value) {
            Ok(d) => d,
            Err(_) => panic!("Setting parameter '{}' requires a number for device of type '{}'", key, self.device_name()),
        };
        if key == attr_str(SUMO_ATTR_ACTUALBATTERYCAPACITY) || key == attr_str(SUMO_ATTR_CHARGELEVEL) {
            self.set_actual_battery_capacity(doubleValue);
        } else if key == attr_str(SUMO_ATTR_MAXIMUMBATTERYCAPACITY) || key == "capacity" {
            self.set_maximum_battery_capacity(doubleValue);
        } else if key == attr_str(SUMO_ATTR_MAXIMUMCHARGERATE) {
            self.set_maximum_charge_rate(doubleValue);
        } else {
            panic!("Setting parameter '{}' is not supported for device of type '{}'", key, self.device_name());
        }
    }

    /// C++: `void MSDevice_Battery::notifyParking()` [virtual]
    /// called to update state for parking vehicles
    // void
    // MSDevice_Battery::notifyParking() {
    //     // @note: only charing is performed but no energy is consumed
    //     notifyMove(myHolder, myHolder.getPositionOnLane(), myHolder.getPositionOnLane(), myHolder.getSpeed());
    //     myConsum = 0;
    // }
    #[allow(unused_variables)]
    pub fn notify_parking(&mut self) {
        // @note: only charing is performed but no energy is consumed
        let holder = self.myHolder;
        let hv = holder.m();
        let pos = hv.get_position_on_lane();
        let spd = hv.get_speed();
        self.notify_move(&mut hv.base, pos, pos, spd);
        self.myConsum = 0.;
    }

    /// C++: `void MSDevice_Battery::generateOutput(OutputDevice *) const` [const, virtual]
    /// Called on vehicle deletion to extend tripinfo
    // void
    // MSDevice_Battery::generateOutput(OutputDevice* tripinfoOut) const {
    //     if (tripinfoOut != nullptr) {
    //         tripinfoOut->openTag("battery");
    //         tripinfoOut->writeAttr("depleted", toString(myDepletedCount));
    //         tripinfoOut->writeAttr("actualBatteryCapacity", toString(myActualBatteryCapacity));
    //         tripinfoOut->writeAttr("totalEnergyConsumed", toString(myTotalConsumption));
    //         tripinfoOut->writeAttr("totalEnergyRegenerated", toString(myTotalRegenerated));
    //         tripinfoOut->closeTag();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn generate_output(&self, tripinfoOut: Ptr<OutputDevice>) {
        if !tripinfoOut.is_null() {
            let out = tripinfoOut.m();
            out.open_tag("battery");
            out.write_attr("depleted", &to_string(&self.myDepletedCount));
            out.write_attr("actualBatteryCapacity", &to_string(&self.myActualBatteryCapacity));
            out.write_attr("totalEnergyConsumed", &to_string(&self.myTotalConsumption));
            out.write_attr("totalEnergyRegenerated", &to_string(&self.myTotalRegenerated));
            out.close_tag("");
        }
    }

    /// C++: `void MSDevice_Battery::notifyMoveInternal(const SUMOTrafficObject &, const double, const double, const double, const double, const double, const double, const double)` [virtual, protected]
    /// Internal notification about the vehicle moves, see MSMoveReminder::notifyMoveInternal()
    // void
    // MSDevice_Battery::notifyMoveInternal(const SUMOTrafficObject& tObject,
    //                                      const double /* frontOnLane */,
    //                                      const double timeOnLane,
    //                                      const double /* meanSpeedFrontOnLane */,
    //                                      const double meanSpeedVehicleOnLane,
    //                                      const double /* travelledDistanceFrontOnLane */,
    //                                      const double /* travelledDistanceVehicleOnLane */,
    //                                      const double /* meanLengthOnLane */) {
    //     const SUMOVehicle& veh = static_cast<const SUMOVehicle&>(tObject);
    //     // Start vehicleStoppedTimer if the vehicle is stopped. In other case reset timer
    //     if (meanSpeedVehicleOnLane < myStoppingThreshold) {
    //         // Increase vehicle stopped timer
    //         increaseVehicleStoppedTimer();
    //     } else {
    //         // Reset vehicle Stopped
    //         resetVehicleStoppedTimer();
    //     }
    //
    //     // Update Energy from the battery
    //     EnergyParams* const params = myHolder.getEmissionParameters();
    //     if (getMaximumBatteryCapacity() != 0) {
    //         if (!myTrackFuel && !veh.getVehicleType().getParameter().wasSet(VTYPEPARS_EMISSIONCLASS_SET)) {
    //             // no explicit emission class, we fall back to the energy model; a warning has been issued on creation
    //             myConsum = PollutantsInterface::getEnergyHelper().compute(0, PollutantsInterface::ELEC, meanSpeedVehicleOnLane, veh.getAcceleration(),
    //                        veh.getSlope(), params) * timeOnLane;
    //         } else {
    //             myConsum = PollutantsInterface::compute(veh.getVehicleType().getEmissionClass(),
    //                                                     myTrackFuel ? PollutantsInterface::FUEL : PollutantsInterface::ELEC,
    //                                                     meanSpeedVehicleOnLane, veh.getAcceleration(),
    //                                                     veh.getSlope(), params) * timeOnLane;
    //         }
    //         if (veh.isParking()) {
    //             // recuperation from last braking step is ok but further consumption should cease
    //             myConsum = MIN2(myConsum, 0.0);
    //         }
    //
    //         // saturate between 0 and myMaximumBatteryCapacity [Wh]
    //         if (myConsum > getActualBatteryCapacity() && getActualBatteryCapacity() > 0 && getMaximumBatteryCapacity() > 0) {
    //             myDepletedCount++;
    //             if (myDepletedCount == 1) {
    //                 WRITE_WARNINGF(TL("Battery of vehicle '%' is depleted, time=%."), veh.getID(), time2string(SIMSTEP));
    //             }
    //         }
    //
    //         // Energy lost/gained from vehicle movement (via vehicle energy model) [Wh]
    //         setActualBatteryCapacity(getActualBatteryCapacity() - myConsum);
    //
    //         // Track total energy consumption and regeneration
    //         if (myConsum > 0.0) {
    //             myTotalConsumption += myConsum;
    //         } else {
    //             myTotalRegenerated -= myConsum;
    //         }
    //
    //         myLastAngle = veh.getAngle();
    //     }
    //
    //     // Check if vehicle has under their position one charge Station
    //     const std::string chargingStationID = MSNet::getInstance()->getStoppingPlaceID(veh.getLane(), veh.getPositionOnLane(), SUMO_TAG_CHARGING_STATION);
    //
    //     // If vehicle is over a charging station
    //     if (chargingStationID != "") {
    //         // if the vehicle is almost stopped, or charge in transit is enabled, then charge vehicle
    //         MSChargingStation* const cs = static_cast<MSChargingStation*>(MSNet::getInstance()->getStoppingPlace(chargingStationID, SUMO_TAG_CHARGING_STATION));
    //         const MSParkingArea* pa = cs->getParkingArea();
    //         if (((meanSpeedVehicleOnLane < myStoppingThreshold) || cs->getChargeInTransit()) && (pa == nullptr || veh.isParking()) && cs->getChargeType() == myChargeType) {
    //             // Set Flags Stopped/intransit to
    //             if (meanSpeedVehicleOnLane < myStoppingThreshold) {
    //                 // vehicle ist almost stopped, then is charging stopped
    //                 myChargingStopped = true;
    //
    //                 // therefore isn't charging in transit
    //                 myChargingInTransit = false;
    //             } else {
    //                 // vehicle is moving, and the Charging station allow charge in transit
    //                 myChargingStopped = false;
    //
    //                 // Therefore charge in transit
    //                 myChargingInTransit = true;
    //             }
    //
    //             // get pointer to charging station
    //             myActChargingStation = cs;
    //
    //             // Only update charging start time if vehicle allow charge in transit, or in other case
    //             // if the vehicle not allow charge in transit but it's stopped.
    //             if ((myActChargingStation->getChargeInTransit()) || (meanSpeedVehicleOnLane < myStoppingThreshold)) {
    //                 // Update Charging start time
    //                 increaseChargingStartTime();
    //             }
    //
    //             // time it takes the vehicle at the station < charging station time delay?
    //             if (getChargingStartTime() > myActChargingStation->getChargeDelay()) {
    //                 // Enable charging vehicle
    //                 myActChargingStation->setChargingVehicle(true);
    //
    //                 // Calulate energy charged
    //                 myEnergyCharged = MIN2(MIN2(myActChargingStation->getChargingPower(myTrackFuel) * myActChargingStation->getEfficency(), getMaximumChargeRate() * (myTrackFuel ? 1 : 1. / 3600.)) * timeOnLane, getMaximumBatteryCapacity() - getActualBatteryCapacity());
    //
    //                 // Update Battery charge
    //                 setActualBatteryCapacity(getActualBatteryCapacity() + myEnergyCharged);
    //             }
    //             // add charge value for output to myActChargingStation
    //             myActChargingStation->addChargeValueForOutput(myEnergyCharged, this);
    //         }
    //         // else disable charging vehicle
    //         else {
    //             cs->setChargingVehicle(false);
    //         }
    //         // disable charging vehicle from previous (not current) ChargingStation (reason: if there is no gap between two different chargingStations = the vehicle switches from used charging station to other one in a single timestap)
    //         if (myPreviousNeighbouringChargingStation != nullptr && myPreviousNeighbouringChargingStation != cs) {
    //             myPreviousNeighbouringChargingStation->setChargingVehicle(false);
    //         }
    //         myPreviousNeighbouringChargingStation = cs;
    //     }
    //     // In other case, vehicle will be not charged
    //     else {
    //         // Disable flags
    //         myChargingInTransit = false;
    //         myChargingStopped = false;
    //
    //         // Disable charging vehicle
    //         if (myActChargingStation != nullptr) {
    //             myActChargingStation->setChargingVehicle(false);
    //         }
    //
    //         // Set charging station pointer to NULL
    //         myActChargingStation = nullptr;
    //
    //         // Set energy charged to 0
    //         myEnergyCharged = 0;
    //
    //         // Reset timer
    //         resetChargingStartTime();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn notify_move_internal(&mut self, tObject: &SUMOTrafficObject, frontOnLane: f64, timeOnLane: f64, meanSpeedFrontOnLane: f64, meanSpeedVehicleOnLane: f64, travelledDistanceFrontOnLane: f64, travelledDistanceVehicleOnLane: f64, meanLengthOnLane: f64) {
        // C++ static_cast<const SUMOVehicle&>(tObject); MSBaseVehicle embeds SUMOVehicle as its first field
        let veh: Ptr<MSBaseVehicle> = Ptr::new(tObject).cast::<SUMOVehicle>().cast::<MSBaseVehicle>();
        // Start vehicleStoppedTimer if the vehicle is stopped. In other case reset timer
        if meanSpeedVehicleOnLane < self.myStoppingThreshold {
            // Increase vehicle stopped timer
            self.increase_vehicle_stopped_timer();
        } else {
            // Reset vehicle Stopped
            self.reset_vehicle_stopped_timer();
        }

        // Update Energy from the battery
        let params = self.myHolder.cast::<MSBaseVehicle>().m().get_emission_parameters();
        if self.get_maximum_battery_capacity() != 0. {
            if !self.myTrackFuel && !veh.get_vehicle_type().get_parameter().was_set(VTYPEPARS_EMISSIONCLASS_SET) {
                // no explicit emission class, we fall back to the energy model; a warning has been issued on creation
                self.myConsum = PollutantsInterface::get_energy_helper().compute(0, PollutantsInterface_EmissionType::ELEC, meanSpeedVehicleOnLane, veh.get_acceleration(), veh.get_slope(), params) * timeOnLane;
            } else {
                self.myConsum = PollutantsInterface::compute(veh.get_vehicle_type().get_emission_class(),
                    if self.myTrackFuel { PollutantsInterface_EmissionType::FUEL } else { PollutantsInterface_EmissionType::ELEC },
                    meanSpeedVehicleOnLane, veh.get_acceleration(), veh.get_slope(), params) * timeOnLane;
            }
            if veh.is_parking() {
                // recuperation from last braking step is ok but further consumption should cease
                self.myConsum = min2(self.myConsum, 0.0);
            }

            // saturate between 0 and myMaximumBatteryCapacity [Wh]
            if self.myConsum > self.get_actual_battery_capacity() && self.get_actual_battery_capacity() > 0. && self.get_maximum_battery_capacity() > 0. {
                self.myDepletedCount += 1;
                if self.myDepletedCount == 1 {
                    write_warningf!("Battery of vehicle '%' is depleted, time=%.", veh.get_id(), time2string(MSNet::get_instance().get_current_time_step()));
                }
            }

            // Energy lost/gained from vehicle movement (via vehicle energy model) [Wh]
            self.set_actual_battery_capacity(self.get_actual_battery_capacity() - self.myConsum);

            // Track total energy consumption and regeneration
            if self.myConsum > 0.0 {
                self.myTotalConsumption += self.myConsum;
            } else {
                self.myTotalRegenerated -= self.myConsum;
            }

            self.myLastAngle = veh.get_angle();
        }

        // Check if vehicle has under their position one charge Station
        let chargingStationID = MSNet::get_instance().get_stopping_place_id(veh.get_lane(), veh.get_position_on_lane(), SUMO_TAG_CHARGING_STATION);

        // If vehicle is over a charging station
        if chargingStationID != "" {
            // if the vehicle is almost stopped, or charge in transit is enabled, then charge vehicle
            let cs: Ptr<MSChargingStation> = MSNet::get_instance().get_stopping_place(&chargingStationID, SUMO_TAG_CHARGING_STATION).cast::<MSChargingStation>();
            let pa: Ptr<MSParkingArea> = cs.get_parking_area();
            if ((meanSpeedVehicleOnLane < self.myStoppingThreshold) || cs.get_charge_in_transit()) && (pa.is_null() || veh.is_parking()) && cs.get_charge_type() == self.myChargeType {
                // Set Flags Stopped/intransit to
                if meanSpeedVehicleOnLane < self.myStoppingThreshold {
                    // vehicle ist almost stopped, then is charging stopped
                    self.myChargingStopped = true;
                    // therefore isn't charging in transit
                    self.myChargingInTransit = false;
                } else {
                    // vehicle is moving, and the Charging station allow charge in transit
                    self.myChargingStopped = false;
                    // Therefore charge in transit
                    self.myChargingInTransit = true;
                }

                // get pointer to charging station
                self.myActChargingStation = cs;

                // Only update charging start time if vehicle allow charge in transit, or in other case
                // if the vehicle not allow charge in transit but it's stopped.
                if self.myActChargingStation.get_charge_in_transit() || (meanSpeedVehicleOnLane < self.myStoppingThreshold) {
                    // Update Charging start time
                    self.increase_charging_start_time();
                }

                // time it takes the vehicle at the station < charging station time delay?
                if self.get_charging_start_time() > self.myActChargingStation.get_charge_delay() {
                    // Enable charging vehicle
                    self.myActChargingStation.m().set_charging_vehicle(true);

                    // Calulate energy charged
                    self.myEnergyCharged = min2(min2(self.myActChargingStation.get_charging_power(self.myTrackFuel) * self.myActChargingStation.get_efficency(), self.get_maximum_charge_rate() * (if self.myTrackFuel { 1. } else { 1. / 3600. })) * timeOnLane, self.get_maximum_battery_capacity() - self.get_actual_battery_capacity());

                    // Update Battery charge
                    self.set_actual_battery_capacity(self.get_actual_battery_capacity() + self.myEnergyCharged);
                }
                // add charge value for output to myActChargingStation
                let me: Ptr<MSDevice_Battery> = Ptr::from_mut(self);
                self.myActChargingStation.m().add_charge_value_for_output(self.myEnergyCharged, me);
            }
            // else disable charging vehicle
            else {
                cs.m().set_charging_vehicle(false);
            }
            // disable charging vehicle from previous (not current) ChargingStation (reason: if there is no gap between two different chargingStations = the vehicle switches from used charging station to other one in a single timestap)
            if !self.myPreviousNeighbouringChargingStation.is_null() && self.myPreviousNeighbouringChargingStation != cs {
                self.myPreviousNeighbouringChargingStation.m().set_charging_vehicle(false);
            }
            self.myPreviousNeighbouringChargingStation = cs;
        }
        // In other case, vehicle will be not charged
        else {
            // Disable flags
            self.myChargingInTransit = false;
            self.myChargingStopped = false;

            // Disable charging vehicle
            if !self.myActChargingStation.is_null() {
                self.myActChargingStation.m().set_charging_vehicle(false);
            }

            // Set charging station pointer to NULL
            self.myActChargingStation = Ptr::default();

            // Set energy charged to 0
            self.myEnergyCharged = 0.;

            // Reset timer
            self.reset_charging_start_time();
        }
    }

    /// C++: `void MSDevice_Battery::MSDevice_Battery(SUMOVehicle &, const std::string &, const double, const double, const double, const double, const std::string &, const std::string &)` [private]
    /// Constructor
    // MSDevice_Battery::MSDevice_Battery(SUMOVehicle& holder, const std::string& id, const double actualBatteryCapacity, const double maximumBatteryCapacity,
    //                                    const double stoppingThreshold, const double maximumChargeRate, const std::string& chargeLevelTable, const std::string& chargeCurveTable) :
    //     MSVehicleDevice(holder, id),
    //     myActualBatteryCapacity(0),         // [actualBatteryCapacity <= maximumBatteryCapacity]
    //     myMaximumBatteryCapacity(0),        // [maximumBatteryCapacity >= 0]
    //     myStoppingThreshold(0),             // [stoppingThreshold >= 0]
    //     myMaximumChargeRate(0),
    //     myChargeLimit(-1),
    //     myLastAngle(std::numeric_limits<double>::infinity()),
    //     myChargingStopped(false),           // Initially vehicle don't charge stopped
    //     myChargingInTransit(false),         // Initially vehicle don't charge in transit
    //     myChargingStartTime(0),             // Initially charging start time (must be if the vehicle was launched at the charging station)
    //     myConsum(0),                        // Initially the vehicle is stopped and therefore the consum is zero.
    //     myTotalConsumption(0.0),
    //     myTotalRegenerated(0.0),
    //     myActChargingStation(nullptr),         // Initially the vehicle isn't over a Charging Station
    //     myPreviousNeighbouringChargingStation(nullptr),    // Initially the vehicle wasn't over a Charging Station
    //     myEnergyCharged(0),                 // Initially the energy charged is zero
    //     myVehicleStopped(0),
    //     myDepletedCount(0) {
    //     // Initially the vehicle is stopped and the corresponding variable is 0
    //
    //     if (maximumBatteryCapacity < 0) {
    //         WRITE_WARNINGF(TL("Battery builder: Vehicle '%' doesn't have a valid value for parameter % (%)."), getID(), toString(SUMO_ATTR_MAXIMUMBATTERYCAPACITY), toString(maximumBatteryCapacity));
    //     } else {
    //         myMaximumBatteryCapacity = maximumBatteryCapacity;
    //     }
    //
    //     if (actualBatteryCapacity > maximumBatteryCapacity) {
    //         WRITE_WARNINGF(TL("Battery builder: Vehicle '%' has a % (%) greater than its % (%). A max battery capacity value will be assigned."),
    //                        getID(), toString(SUMO_ATTR_ACTUALBATTERYCAPACITY), toString(actualBatteryCapacity), toString(SUMO_ATTR_MAXIMUMBATTERYCAPACITY), toString(maximumBatteryCapacity));
    //         myActualBatteryCapacity = myMaximumBatteryCapacity;
    //     } else {
    //         myActualBatteryCapacity = actualBatteryCapacity;
    //     }
    //
    //     if (stoppingThreshold < 0) {
    //         WRITE_WARNINGF(TL("Battery builder: Vehicle '%' doesn't have a valid value for parameter % (%)."), getID(), toString(SUMO_ATTR_STOPPINGTHRESHOLD), toString(stoppingThreshold));
    //     } else {
    //         myStoppingThreshold = stoppingThreshold;
    //     }
    //
    //     myTrackFuel = PollutantsInterface::getFuel(holder.getVehicleType().getEmissionClass()) != "Electricity" && OptionsCont::getOptions().getBool("device.battery.track-fuel");
    //     if (!myTrackFuel && !holder.getVehicleType().getParameter().wasSet(VTYPEPARS_EMISSIONCLASS_SET)) {
    //         WRITE_WARNINGF(TL("The battery device is active for vehicle '%' but no emission class is set. "
    //                           "Please consider setting an explicit emission class or battery outputs might be inconsistent with emission outputs!"),
    //                        holder.getID());
    //     }
    //     myChargeType = (myTrackFuel) ? MSChargingStation::ChargeType::CHARGETYPE_FUEL : MSChargingStation::ChargeType::CHARGETYPE_NORMAL;
    //
    //     if (maximumChargeRate < 0) {
    //         WRITE_WARNINGF(TL("Battery builder: Vehicle '%' doesn't have a valid value for parameter % (%)."), getID(), toString(SUMO_ATTR_MAXIMUMCHARGERATE), toString(maximumChargeRate));
    //     } else {
    //         if (!chargeLevelTable.empty() && !chargeCurveTable.empty()) {
    //             LinearApproxHelpers::setPoints(myChargeCurve, chargeLevelTable, chargeCurveTable);
    //             myMaximumChargeRate = LinearApproxHelpers::getMaximumValue(myChargeCurve);
    //         } else {
    //             myMaximumChargeRate = maximumChargeRate;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn new(holder: &mut SUMOVehicle, id: &str, actualBatteryCapacity: f64, maximumBatteryCapacity: f64, stoppingThreshold: f64, maximumChargeRate: f64, chargeLevelTable: &str, chargeCurveTable: &str) -> Self {
        fn mr_notify_move(mr: &mut MSMoveReminder, veh: &mut SUMOTrafficObject, oldPos: f64, newPos: f64, newSpeed: f64) -> bool {
            Ptr::from_mut(mr).cast::<MSDevice_Battery>().m().notify_move(veh, oldPos, newPos, newSpeed)
        }
        static MS_MOVE_REMINDER_VT: MSMoveReminderVt = MSMoveReminderVt {
            notify_enter: crate::microsim::ms_move_reminder::default_notify_enter,
            notify_move: mr_notify_move,
            notify_leave: crate::microsim::ms_move_reminder::default_notify_leave,
        };
        fn mr_notify_parking(mr: &mut MSMoveReminder) {
            Ptr::from_mut(mr).cast::<MSDevice_Battery>().m().notify_parking()
        }
        static MS_MOVE_REMINDER_IDLE_VT: MSMoveReminderIdleVt = MSMoveReminderIdleVt {
            notify_idle: crate::microsim::ms_move_reminder::default_notify_idle,
            notify_parking: mr_notify_parking,
            notify_stop_ended: crate::microsim::ms_move_reminder::default_notify_stop_ended,
        };
        fn mr_notify_move_internal(mr: &mut MSMoveReminder, veh: &SUMOTrafficObject, frontOnLane: f64, timeOnLane: f64, meanSpeedFrontOnLane: f64, meanSpeedVehicleOnLane: f64, travelledDistanceFrontOnLane: f64, travelledDistanceVehicleOnLane: f64, meanLengthOnLane: f64) {
            Ptr::from_mut(mr).cast::<MSDevice_Battery>().m().notify_move_internal(veh, frontOnLane, timeOnLane, meanSpeedFrontOnLane, meanSpeedVehicleOnLane, travelledDistanceFrontOnLane, travelledDistanceVehicleOnLane, meanLengthOnLane)
        }
        static MS_MOVE_REMINDER_INTERNAL_VT: crate::microsim::ms_move_reminder::MSMoveReminderInternalVt = crate::microsim::ms_move_reminder::MSMoveReminderInternalVt {
            notify_move_internal: mr_notify_move_internal,
        };
        let mut this = Self::new_device(holder, id, actualBatteryCapacity, maximumBatteryCapacity, stoppingThreshold, maximumChargeRate, chargeLevelTable, chargeCurveTable);
        this.base.base.vt = &MS_MOVE_REMINDER_VT;
        this.base.base.idle_vt = &MS_MOVE_REMINDER_IDLE_VT;
        this.base.base.internal_vt = &MS_MOVE_REMINDER_INTERNAL_VT;
        this
    }

    fn new_device(holder: &mut SUMOVehicle, id: &str, actualBatteryCapacity: f64, maximumBatteryCapacity: f64, stoppingThreshold: f64, maximumChargeRate: f64, chargeLevelTable: &str, chargeCurveTable: &str) -> Self {
        let mut s = MSDevice_Battery {
            base: MSVehicleDevice::new(holder, id),
            myActualBatteryCapacity: 0.,          // [actualBatteryCapacity <= maximumBatteryCapacity]
            myMaximumBatteryCapacity: 0.,         // [maximumBatteryCapacity >= 0]
            myStoppingThreshold: 0.,              // [stoppingThreshold >= 0]
            myMaximumChargeRate: 0.,
            myChargeLimit: -1.,
            myLastAngle: f64::INFINITY,
            myChargingStopped: false,             // Initially vehicle don't charge stopped
            myChargingInTransit: false,           // Initially vehicle don't charge in transit
            myChargingStartTime: 0,               // Initially charging start time (must be if the vehicle was launched at the charging station)
            myConsum: 0.,                         // Initially the vehicle is stopped and therefore the consum is zero.
            myTotalConsumption: 0.0,
            myTotalRegenerated: 0.0,
            myChargeCurve: Vec::new(),
            myActChargingStation: Ptr::default(),                  // Initially the vehicle isn't over a Charging Station
            myPreviousNeighbouringChargingStation: Ptr::default(), // Initially the vehicle wasn't over a Charging Station
            myEnergyCharged: 0.,                  // Initially the energy charged is zero
            myVehicleStopped: 0,
            myDepletedCount: 0,
            myTrackFuel: false,
            myChargeType: MSChargingStation_ChargeType::CHARGETYPE_NORMAL,
        };
        // Initially the vehicle is stopped and the corresponding variable is 0

        if maximumBatteryCapacity < 0. {
            write_warningf!("Battery builder: Vehicle '%' doesn't have a valid value for parameter % (%).", s.base.base2.get_id(), attr_str(SUMO_ATTR_MAXIMUMBATTERYCAPACITY), to_string(&maximumBatteryCapacity));
        } else {
            s.myMaximumBatteryCapacity = maximumBatteryCapacity;
        }

        if actualBatteryCapacity > maximumBatteryCapacity {
            write_warningf!("Battery builder: Vehicle '%' has a % (%) greater than its % (%). A max battery capacity value will be assigned.",
                s.base.base2.get_id(), attr_str(SUMO_ATTR_ACTUALBATTERYCAPACITY), to_string(&actualBatteryCapacity), attr_str(SUMO_ATTR_MAXIMUMBATTERYCAPACITY), to_string(&maximumBatteryCapacity));
            s.myActualBatteryCapacity = s.myMaximumBatteryCapacity;
        } else {
            s.myActualBatteryCapacity = actualBatteryCapacity;
        }

        if stoppingThreshold < 0. {
            write_warningf!("Battery builder: Vehicle '%' doesn't have a valid value for parameter % (%).", s.base.base2.get_id(), attr_str(SUMO_ATTR_STOPPINGTHRESHOLD), to_string(&stoppingThreshold));
        } else {
            s.myStoppingThreshold = stoppingThreshold;
        }

        s.myTrackFuel = PollutantsInterface::get_fuel(holder.get_vehicle_type().get_emission_class()) != "Electricity" && get_bool("device.battery.track-fuel");
        if !s.myTrackFuel && !holder.get_vehicle_type().get_parameter().was_set(VTYPEPARS_EMISSIONCLASS_SET) {
            write_warningf!("The battery device is active for vehicle '%' but no emission class is set. Please consider setting an explicit emission class or battery outputs might be inconsistent with emission outputs!",
                holder.get_id());
        }
        s.myChargeType = if s.myTrackFuel { MSChargingStation_ChargeType::CHARGETYPE_FUEL } else { MSChargingStation_ChargeType::CHARGETYPE_NORMAL };

        if maximumChargeRate < 0. {
            write_warningf!("Battery builder: Vehicle '%' doesn't have a valid value for parameter % (%).", s.base.base2.get_id(), attr_str(SUMO_ATTR_MAXIMUMCHARGERATE), to_string(&maximumChargeRate));
        } else if !chargeLevelTable.is_empty() && !chargeCurveTable.is_empty() {
            set_points(&mut s.myChargeCurve, chargeLevelTable, chargeCurveTable);
            s.myMaximumChargeRate = get_maximum_value(&s.myChargeCurve);
        } else {
            s.myMaximumChargeRate = maximumChargeRate;
        }
        s
    }

    /// C++: `double MSDevice_Battery::getActualBatteryCapacity() const` [const]
    /// Get the actual vehicle's Battery Capacity in Wh
    // double
    // MSDevice_Battery::getActualBatteryCapacity() const {
    //     return myActualBatteryCapacity;
    // }
    #[allow(unused_variables)]
    pub fn get_actual_battery_capacity(&self) -> f64 {
        self.myActualBatteryCapacity
    }

    /// C++: `double MSDevice_Battery::getMaximumBatteryCapacity() const` [const]
    /// Get the total vehicle's Battery Capacity in Wh
    // double
    // MSDevice_Battery::getMaximumBatteryCapacity() const {
    //     return myMaximumBatteryCapacity;
    // }
    #[allow(unused_variables)]
    pub fn get_maximum_battery_capacity(&self) -> f64 {
        self.myMaximumBatteryCapacity
    }

    /// C++: `double MSDevice_Battery::getMaximumPower() const` [const]
    /// Get the maximum power when accelerating
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_maximum_power(&self) -> f64 {
        // C++ declares getMaximumPower() in MSDevice_Battery.h but never defines it, so no caller can link against it
        panic!("MSDevice_Battery::getMaximumPower is declared in C++ but has no definition")
    }

    /// C++: `bool MSDevice_Battery::isChargingStopped() const` [const]
    /// Get true if Vehicle is charging, false if not.
    // bool
    // MSDevice_Battery::isChargingStopped() const {
    //     return myChargingStopped;
    // }
    #[allow(unused_variables)]
    pub fn is_charging_stopped(&self) -> bool {
        self.myChargingStopped
    }

    /// C++: `bool MSDevice_Battery::isChargingInTransit() const` [const]
    /// Get true if Vehicle it's charging, false if not.
    // bool
    // MSDevice_Battery::isChargingInTransit() const {
    //     return myChargingInTransit;
    // }
    #[allow(unused_variables)]
    pub fn is_charging_in_transit(&self) -> bool {
        self.myChargingInTransit
    }

    /// C++: `SUMOTime MSDevice_Battery::getChargingStartTime() const` [const]
    /// Get charging start time.
    // SUMOTime
    // MSDevice_Battery::getChargingStartTime() const {
    //     return myChargingStartTime;
    // }
    #[allow(unused_variables)]
    pub fn get_charging_start_time(&self) -> SUMOTime {
        self.myChargingStartTime
    }

    /// C++: `SUMOTime MSDevice_Battery::estimateChargingDuration(const double, const double) const` [const]
    /// Estimate the charging duration given the current battery state
    // SUMOTime
    // MSDevice_Battery::estimateChargingDuration(const double toCharge, const double csPower) const {
    //     //if (!myChargeCurve.empty()) {
    //     //    // TODO: integrate charge curve
    //     //}
    //     return TIME2STEPS(toCharge / MIN2(csPower, myMaximumChargeRate));
    // }
    #[allow(unused_variables)]
    pub fn estimate_charging_duration(&self, toCharge: f64, csPower: f64) -> SUMOTime {
        //if (!myChargeCurve.empty()) {
        //    // TODO: integrate charge curve
        //}
        time2steps(toCharge / min2(csPower, self.myMaximumChargeRate))
    }

    /// C++: `double MSDevice_Battery::getConsum() const` [const]
    /// Get consum
    // double
    // MSDevice_Battery::getConsum() const {
    //     return myConsum;
    // }
    #[allow(unused_variables)]
    pub fn get_consum(&self) -> f64 {
        self.myConsum
    }

    /// C++: `double MSDevice_Battery::getTotalConsumption() const` [const]
    /// Get total consumption
    // double
    // MSDevice_Battery::getTotalConsumption() const {
    //     return myTotalConsumption;
    // }
    #[allow(unused_variables)]
    pub fn get_total_consumption(&self) -> f64 {
        self.myTotalConsumption
    }

    /// C++: `double MSDevice_Battery::getTotalRegenerated() const` [const]
    /// Get total regenerated
    // double
    // MSDevice_Battery::getTotalRegenerated() const {
    //     return myTotalRegenerated;
    // }
    #[allow(unused_variables)]
    pub fn get_total_regenerated(&self) -> f64 {
        self.myTotalRegenerated
    }

    /// C++: `std::string MSDevice_Battery::getChargingStationID() const` [const]
    /// Get current Charging Station ID
    // std::string
    // MSDevice_Battery::getChargingStationID() const {
    //     if (myActChargingStation != nullptr) {
    //         return myActChargingStation->getID();
    //     } else {
    //         return "NULL";
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_charging_station_id(&self) -> String {
        if !self.myActChargingStation.is_null() {
            self.myActChargingStation.get_id().to_string()
        } else {
            "NULL".to_string()
        }
    }

    /// C++: `MSChargingStation * MSDevice_Battery::getChargingStation() const` [const]
    // MSChargingStation* getChargingStation() const {
    //         return myActChargingStation;
    //     }
    #[allow(unused_variables)]
    pub fn get_charging_station(&self) -> Ptr<MSChargingStation> {
        self.myActChargingStation
    }

    /// C++: `double MSDevice_Battery::getEnergyCharged() const` [const]
    /// Get charged energy
    // double
    // MSDevice_Battery::getEnergyCharged() const {
    //     return myEnergyCharged;
    // }
    #[allow(unused_variables)]
    pub fn get_energy_charged(&self) -> f64 {
        self.myEnergyCharged
    }

    /// C++: `int MSDevice_Battery::getVehicleStopped() const` [const]
    /// Get number of timestep that vehicle is stopped
    // int
    // MSDevice_Battery::getVehicleStopped() const {
    //     return myVehicleStopped;
    // }
    #[allow(unused_variables)]
    pub fn get_vehicle_stopped(&self) -> i32 {
        self.myVehicleStopped
    }

    /// C++: `double MSDevice_Battery::getStoppingThreshold() const` [const]
    /// Get stopping threshold
    // double
    // MSDevice_Battery::getStoppingThreshold() const {
    //     return myStoppingThreshold;
    // }
    #[allow(unused_variables)]
    pub fn get_stopping_threshold(&self) -> f64 {
        self.myStoppingThreshold
    }

    /// C++: `double MSDevice_Battery::getMaximumChargeRate() const` [const]
    /// Get current charge rate in W depending on the state of charge
    // double
    // MSDevice_Battery::getMaximumChargeRate() const {
    //     double baseVal = (myChargeCurve.empty()) ? myMaximumChargeRate : LinearApproxHelpers::getInterpolatedValue(myChargeCurve, myActualBatteryCapacity / myMaximumBatteryCapacity);
    //     return (myChargeLimit < 0) ? baseVal : MIN2(myChargeLimit, baseVal);
    // }
    #[allow(unused_variables)]
    pub fn get_maximum_charge_rate(&self) -> f64 {
        let baseVal = if self.myChargeCurve.is_empty() {
            self.myMaximumChargeRate
        } else {
            crate::microsim::cfmodels::mscf_model::get_interpolated_value(&self.myChargeCurve, self.myActualBatteryCapacity / self.myMaximumBatteryCapacity)
        };
        if self.myChargeLimit < 0. { baseVal } else { min2(self.myChargeLimit, baseVal) }
    }

    /// C++: `bool MSDevice_Battery::tracksFuel() const` [const]
    /// Whether the battery device is actually used as a tank of a combustion vehicle
    // bool
    // MSDevice_Battery::tracksFuel() const {
    //     return myTrackFuel;
    // }
    #[allow(unused_variables)]
    pub fn tracks_fuel(&self) -> bool {
        self.myTrackFuel
    }

    /// C++: `MSChargingStation::ChargeType MSDevice_Battery::getChargeType() const` [const]
    /// Get the charge type
    // MSChargingStation::ChargeType
    // MSDevice_Battery::getChargeType() const {
    //     return myChargeType;
    // }
    #[allow(unused_variables)]
    pub fn get_charge_type(&self) -> MSChargingStation_ChargeType {
        self.myChargeType
    }

    /// C++: `void MSDevice_Battery::setActualBatteryCapacity(const double)`
    /// Set actual vehicle's Battery Capacity in kWh
    // void
    // MSDevice_Battery::setActualBatteryCapacity(const double actualBatteryCapacity) {
    //     if (actualBatteryCapacity < 0) {
    //         myActualBatteryCapacity = 0;
    //     } else if (actualBatteryCapacity > myMaximumBatteryCapacity) {
    //         myActualBatteryCapacity = myMaximumBatteryCapacity;
    //     } else {
    //         myActualBatteryCapacity = actualBatteryCapacity;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_actual_battery_capacity(&mut self, actualBatteryCapacity: f64) {
        if actualBatteryCapacity < 0. {
            self.myActualBatteryCapacity = 0.;
        } else if actualBatteryCapacity > self.myMaximumBatteryCapacity {
            self.myActualBatteryCapacity = self.myMaximumBatteryCapacity;
        } else {
            self.myActualBatteryCapacity = actualBatteryCapacity;
        }
    }

    /// C++: `void MSDevice_Battery::setMaximumBatteryCapacity(const double)`
    /// Set total vehicle's Battery Capacity in kWh
    // void
    // MSDevice_Battery::setMaximumBatteryCapacity(const double maximumBatteryCapacity) {
    //     if (myMaximumBatteryCapacity < 0) {
    //         WRITE_WARNINGF(TL("Trying to set into the battery device of vehicle '%' an invalid % (%)."), getID(), toString(SUMO_ATTR_MAXIMUMBATTERYCAPACITY), toString(maximumBatteryCapacity));
    //     } else {
    //         myMaximumBatteryCapacity = maximumBatteryCapacity;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_maximum_battery_capacity(&mut self, maximumBatteryCapacity: f64) {
        if self.myMaximumBatteryCapacity < 0. {
            write_warningf!("Trying to set into the battery device of vehicle '%' an invalid % (%).", self.base.base2.get_id(), attr_str(SUMO_ATTR_MAXIMUMBATTERYCAPACITY), to_string(&maximumBatteryCapacity));
        } else {
            self.myMaximumBatteryCapacity = maximumBatteryCapacity;
        }
    }

    /// C++: `void MSDevice_Battery::setStoppingThreshold(const double)`
    /// Set vehicle's stopping threshold
    // void
    // MSDevice_Battery::setStoppingThreshold(const double stoppingThreshold) {
    //     if (stoppingThreshold < 0) {
    //         WRITE_WARNINGF(TL("Trying to set into the battery device of vehicle '%' an invalid % (%)."), getID(), toString(SUMO_ATTR_STOPPINGTHRESHOLD), toString(stoppingThreshold));
    //     } else {
    //         myStoppingThreshold = stoppingThreshold;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_stopping_threshold(&mut self, stoppingThreshold: f64) {
        if stoppingThreshold < 0. {
            write_warningf!("Trying to set into the battery device of vehicle '%' an invalid % (%).", self.base.base2.get_id(), attr_str(SUMO_ATTR_STOPPINGTHRESHOLD), to_string(&stoppingThreshold));
        } else {
            self.myStoppingThreshold = stoppingThreshold;
        }
    }

    /// C++: `void MSDevice_Battery::setMaximumChargeRate(const double)`
    /// Set vehicle's stopping threshold
    // void
    // MSDevice_Battery::setMaximumChargeRate(const double chargeRate) {
    //     if (chargeRate < 0) {
    //         WRITE_WARNINGF(TL("Trying to set into the battery device of vehicle '%' an invalid % (%)."), getID(), toString(SUMO_ATTR_MAXIMUMCHARGERATE), toString(chargeRate));
    //     } else {
    //         myMaximumChargeRate = chargeRate;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_maximum_charge_rate(&mut self, chargeRate: f64) {
        if chargeRate < 0. {
            write_warningf!("Trying to set into the battery device of vehicle '%' an invalid % (%).", self.base.base2.get_id(), attr_str(SUMO_ATTR_MAXIMUMCHARGERATE), to_string(&chargeRate));
        } else {
            self.myMaximumChargeRate = chargeRate;
        }
    }

    /// C++: `void MSDevice_Battery::setChargeLimit(const double)`
    /// Set (temporary) charge limit
    // void
    // MSDevice_Battery::setChargeLimit(const double limit) {
    //     myChargeLimit = limit;
    // }
    #[allow(unused_variables)]
    pub fn set_charge_limit(&mut self, limit: f64) {
        self.myChargeLimit = limit;
    }

    /// C++: `void MSDevice_Battery::resetChargingStartTime()`
    /// Reset charging start time
    // void
    // MSDevice_Battery::resetChargingStartTime() {
    //     myChargingStartTime = 0;
    // }
    #[allow(unused_variables)]
    pub fn reset_charging_start_time(&mut self) {
        self.myChargingStartTime = 0;
    }

    /// C++: `void MSDevice_Battery::increaseChargingStartTime()`
    /// Increase Charging Start time
    // void
    // MSDevice_Battery::increaseChargingStartTime() {
    //     myChargingStartTime += DELTA_T;
    // }
    #[allow(unused_variables)]
    pub fn increase_charging_start_time(&mut self) {
        self.myChargingStartTime += delta_t();
    }

    /// C++: `void MSDevice_Battery::resetVehicleStoppedTimer()`
    /// Reset myVehicleStopped
    // void
    // MSDevice_Battery::resetVehicleStoppedTimer() {
    //     myVehicleStopped = 0;
    // }
    #[allow(unused_variables)]
    pub fn reset_vehicle_stopped_timer(&mut self) {
        self.myVehicleStopped = 0;
    }

    /// C++: `void MSDevice_Battery::increaseVehicleStoppedTimer()`
    /// Increase myVehicleStopped
    // void
    // MSDevice_Battery::increaseVehicleStoppedTimer() {
    //     myVehicleStopped++;
    // }
    #[allow(unused_variables)]
    pub fn increase_vehicle_stopped_timer(&mut self) {
        self.myVehicleStopped += 1;
    }

    /// C++: `void MSDevice_Battery::setEnergyCharged(const double)`
    /// Set Energy Charged in Wh
    // void
    // MSDevice_Battery::setEnergyCharged(const double energyCharged) {
    //     myEnergyCharged = energyCharged;
    // }
    #[allow(unused_variables)]
    pub fn set_energy_charged(&mut self, energyCharged: f64) {
        self.myEnergyCharged = energyCharged;
    }

    /// C++: `double MSDevice_Battery::readParameterValue(SUMOVehicle &, const SumoXMLAttr &, const std::string &, double)` [static, protected]
    /// Read device parameters from input
    // double
    // MSDevice_Battery::readParameterValue(SUMOVehicle& v, const SumoXMLAttr& attr, const std::string& paramName, double defaultVal) {
    //     const std::string& oldParam = toString(attr);
    //     const SUMOVTypeParameter& typeParams = v.getVehicleType().getParameter();
    //     if (v.getParameter().hasParameter(oldParam) || typeParams.hasParameter(oldParam)) {
    //         WRITE_WARNINGF(TL("Battery device in vehicle '%' still uses old parameter '%'. Please update to 'device.%'."), v.getID(), oldParam, paramName);
    //         if (v.getParameter().getParameter(oldParam, "-") == "-") {
    //             return typeParams.getDouble(oldParam, defaultVal);
    //         }
    //         return StringUtils::toDouble(v.getParameter().getParameter(oldParam, "0"));
    //     }
    //     return v.getFloatParam("device." + paramName, false, defaultVal);
    // }
    #[allow(unused_variables)]
    pub fn read_parameter_value(v: &mut SUMOVehicle, attr: &SumoXMLAttr, paramName: &str, defaultVal: f64) -> f64 {
        let oldParam = attr_str(*attr);
        let typeParams = v.get_vehicle_type().get_parameter();
        if v.get_parameter().has_parameter(&oldParam) || typeParams.has_parameter(&oldParam) {
            write_warningf!("Battery device in vehicle '%' still uses old parameter '%'. Please update to 'device.%'.", v.get_id(), oldParam, paramName);
            if v.get_parameter().get_parameter(&oldParam, "-") == "-" {
                return typeParams.get_double(&oldParam, defaultVal);
            }
            return to_double(&v.get_parameter().get_parameter(&oldParam, "0")).unwrap_or_else(|e| panic!("{}", e));
        }
        v.get_float_param(&format!("device.{}", paramName), false, defaultVal, true)
    }

    /// C++: `void MSDevice_Battery::MSDevice_Battery(const MSDevice_Battery &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSDevice_Battery) -> Self {
        panic!("MSDevice_Battery copy constructor is private and undefined in C++")
    }

    /// C++: `MSDevice_Battery & MSDevice_Battery::operator=(const MSDevice_Battery &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSDevice_Battery) -> &mut MSDevice_Battery {
        // C++ declares the assignment operator private and never defines it, so it cannot be called
        panic!("MSDevice_Battery assignment operator is private and undefined in C++")
    }

}

