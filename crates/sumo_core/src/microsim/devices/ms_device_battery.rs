//! Port of microsim/devices/MSDevice_Battery.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// Battery device for electric vehicles
/// C++ class `MSDevice_Battery` (microsim/devices/MSDevice_Battery.h)
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
    pub myChargeCurve: Unmapped /* LinearApproxHelpers::LinearApproxMap */,
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
        todo!("port MSDevice_Battery::insertOptions")
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
        todo!("port MSDevice_Battery::buildVehicleDevices")
    }

    /// C++: `void MSDevice_Battery::~MSDevice_Battery() noexcept` [virtual]
    /// Destructor.
    // MSDevice_Battery::~MSDevice_Battery() {
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MSDevice_Battery::~MSDevice_Battery")
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
        todo!("port MSDevice_Battery::notifyMove")
    }

    /// C++: `const std::string MSDevice_Battery::deviceName() const` [const, virtual]
    /// return the name for this type of device
    // const std::string deviceName() const override {
    //         return "battery";
    //     }
    #[allow(unused_variables)]
    pub fn device_name(&self) -> String {
        todo!("port MSDevice_Battery::deviceName")
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
        todo!("port MSDevice_Battery::saveState")
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
        todo!("port MSDevice_Battery::loadState")
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
        todo!("port MSDevice_Battery::getParameter")
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
        todo!("port MSDevice_Battery::setParameter")
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
        todo!("port MSDevice_Battery::notifyParking")
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
        todo!("port MSDevice_Battery::generateOutput")
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
        todo!("port MSDevice_Battery::notifyMoveInternal")
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
        todo!("port MSDevice_Battery::MSDevice_Battery")
    }

    /// C++: `double MSDevice_Battery::getActualBatteryCapacity() const` [const]
    /// Get the actual vehicle's Battery Capacity in Wh
    // double
    // MSDevice_Battery::getActualBatteryCapacity() const {
    //     return myActualBatteryCapacity;
    // }
    #[allow(unused_variables)]
    pub fn get_actual_battery_capacity(&self) -> f64 {
        todo!("port MSDevice_Battery::getActualBatteryCapacity")
    }

    /// C++: `double MSDevice_Battery::getMaximumBatteryCapacity() const` [const]
    /// Get the total vehicle's Battery Capacity in Wh
    // double
    // MSDevice_Battery::getMaximumBatteryCapacity() const {
    //     return myMaximumBatteryCapacity;
    // }
    #[allow(unused_variables)]
    pub fn get_maximum_battery_capacity(&self) -> f64 {
        todo!("port MSDevice_Battery::getMaximumBatteryCapacity")
    }

    /// C++: `double MSDevice_Battery::getMaximumPower() const` [const]
    /// Get the maximum power when accelerating
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_maximum_power(&self) -> f64 {
        todo!("port MSDevice_Battery::getMaximumPower")
    }

    /// C++: `bool MSDevice_Battery::isChargingStopped() const` [const]
    /// Get true if Vehicle is charging, false if not.
    // bool
    // MSDevice_Battery::isChargingStopped() const {
    //     return myChargingStopped;
    // }
    #[allow(unused_variables)]
    pub fn is_charging_stopped(&self) -> bool {
        todo!("port MSDevice_Battery::isChargingStopped")
    }

    /// C++: `bool MSDevice_Battery::isChargingInTransit() const` [const]
    /// Get true if Vehicle it's charging, false if not.
    // bool
    // MSDevice_Battery::isChargingInTransit() const {
    //     return myChargingInTransit;
    // }
    #[allow(unused_variables)]
    pub fn is_charging_in_transit(&self) -> bool {
        todo!("port MSDevice_Battery::isChargingInTransit")
    }

    /// C++: `SUMOTime MSDevice_Battery::getChargingStartTime() const` [const]
    /// Get charging start time.
    // SUMOTime
    // MSDevice_Battery::getChargingStartTime() const {
    //     return myChargingStartTime;
    // }
    #[allow(unused_variables)]
    pub fn get_charging_start_time(&self) -> SUMOTime {
        todo!("port MSDevice_Battery::getChargingStartTime")
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
        todo!("port MSDevice_Battery::estimateChargingDuration")
    }

    /// C++: `double MSDevice_Battery::getConsum() const` [const]
    /// Get consum
    // double
    // MSDevice_Battery::getConsum() const {
    //     return myConsum;
    // }
    #[allow(unused_variables)]
    pub fn get_consum(&self) -> f64 {
        todo!("port MSDevice_Battery::getConsum")
    }

    /// C++: `double MSDevice_Battery::getTotalConsumption() const` [const]
    /// Get total consumption
    // double
    // MSDevice_Battery::getTotalConsumption() const {
    //     return myTotalConsumption;
    // }
    #[allow(unused_variables)]
    pub fn get_total_consumption(&self) -> f64 {
        todo!("port MSDevice_Battery::getTotalConsumption")
    }

    /// C++: `double MSDevice_Battery::getTotalRegenerated() const` [const]
    /// Get total regenerated
    // double
    // MSDevice_Battery::getTotalRegenerated() const {
    //     return myTotalRegenerated;
    // }
    #[allow(unused_variables)]
    pub fn get_total_regenerated(&self) -> f64 {
        todo!("port MSDevice_Battery::getTotalRegenerated")
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
        todo!("port MSDevice_Battery::getChargingStationID")
    }

    /// C++: `MSChargingStation * MSDevice_Battery::getChargingStation() const` [const]
    // MSChargingStation* getChargingStation() const {
    //         return myActChargingStation;
    //     }
    #[allow(unused_variables)]
    pub fn get_charging_station(&self) -> Ptr<MSChargingStation> {
        todo!("port MSDevice_Battery::getChargingStation")
    }

    /// C++: `double MSDevice_Battery::getEnergyCharged() const` [const]
    /// Get charged energy
    // double
    // MSDevice_Battery::getEnergyCharged() const {
    //     return myEnergyCharged;
    // }
    #[allow(unused_variables)]
    pub fn get_energy_charged(&self) -> f64 {
        todo!("port MSDevice_Battery::getEnergyCharged")
    }

    /// C++: `int MSDevice_Battery::getVehicleStopped() const` [const]
    /// Get number of timestep that vehicle is stopped
    // int
    // MSDevice_Battery::getVehicleStopped() const {
    //     return myVehicleStopped;
    // }
    #[allow(unused_variables)]
    pub fn get_vehicle_stopped(&self) -> i32 {
        todo!("port MSDevice_Battery::getVehicleStopped")
    }

    /// C++: `double MSDevice_Battery::getStoppingThreshold() const` [const]
    /// Get stopping threshold
    // double
    // MSDevice_Battery::getStoppingThreshold() const {
    //     return myStoppingThreshold;
    // }
    #[allow(unused_variables)]
    pub fn get_stopping_threshold(&self) -> f64 {
        todo!("port MSDevice_Battery::getStoppingThreshold")
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
        todo!("port MSDevice_Battery::getMaximumChargeRate")
    }

    /// C++: `bool MSDevice_Battery::tracksFuel() const` [const]
    /// Whether the battery device is actually used as a tank of a combustion vehicle
    // bool
    // MSDevice_Battery::tracksFuel() const {
    //     return myTrackFuel;
    // }
    #[allow(unused_variables)]
    pub fn tracks_fuel(&self) -> bool {
        todo!("port MSDevice_Battery::tracksFuel")
    }

    /// C++: `MSChargingStation::ChargeType MSDevice_Battery::getChargeType() const` [const]
    /// Get the charge type
    // MSChargingStation::ChargeType
    // MSDevice_Battery::getChargeType() const {
    //     return myChargeType;
    // }
    #[allow(unused_variables)]
    pub fn get_charge_type(&self) -> MSChargingStation_ChargeType {
        todo!("port MSDevice_Battery::getChargeType")
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
        todo!("port MSDevice_Battery::setActualBatteryCapacity")
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
        todo!("port MSDevice_Battery::setMaximumBatteryCapacity")
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
        todo!("port MSDevice_Battery::setStoppingThreshold")
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
        todo!("port MSDevice_Battery::setMaximumChargeRate")
    }

    /// C++: `void MSDevice_Battery::setChargeLimit(const double)`
    /// Set (temporary) charge limit
    // void
    // MSDevice_Battery::setChargeLimit(const double limit) {
    //     myChargeLimit = limit;
    // }
    #[allow(unused_variables)]
    pub fn set_charge_limit(&mut self, limit: f64) {
        todo!("port MSDevice_Battery::setChargeLimit")
    }

    /// C++: `void MSDevice_Battery::resetChargingStartTime()`
    /// Reset charging start time
    // void
    // MSDevice_Battery::resetChargingStartTime() {
    //     myChargingStartTime = 0;
    // }
    #[allow(unused_variables)]
    pub fn reset_charging_start_time(&mut self) {
        todo!("port MSDevice_Battery::resetChargingStartTime")
    }

    /// C++: `void MSDevice_Battery::increaseChargingStartTime()`
    /// Increase Charging Start time
    // void
    // MSDevice_Battery::increaseChargingStartTime() {
    //     myChargingStartTime += DELTA_T;
    // }
    #[allow(unused_variables)]
    pub fn increase_charging_start_time(&mut self) {
        todo!("port MSDevice_Battery::increaseChargingStartTime")
    }

    /// C++: `void MSDevice_Battery::resetVehicleStoppedTimer()`
    /// Reset myVehicleStopped
    // void
    // MSDevice_Battery::resetVehicleStoppedTimer() {
    //     myVehicleStopped = 0;
    // }
    #[allow(unused_variables)]
    pub fn reset_vehicle_stopped_timer(&mut self) {
        todo!("port MSDevice_Battery::resetVehicleStoppedTimer")
    }

    /// C++: `void MSDevice_Battery::increaseVehicleStoppedTimer()`
    /// Increase myVehicleStopped
    // void
    // MSDevice_Battery::increaseVehicleStoppedTimer() {
    //     myVehicleStopped++;
    // }
    #[allow(unused_variables)]
    pub fn increase_vehicle_stopped_timer(&mut self) {
        todo!("port MSDevice_Battery::increaseVehicleStoppedTimer")
    }

    /// C++: `void MSDevice_Battery::setEnergyCharged(const double)`
    /// Set Energy Charged in Wh
    // void
    // MSDevice_Battery::setEnergyCharged(const double energyCharged) {
    //     myEnergyCharged = energyCharged;
    // }
    #[allow(unused_variables)]
    pub fn set_energy_charged(&mut self, energyCharged: f64) {
        todo!("port MSDevice_Battery::setEnergyCharged")
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
        todo!("port MSDevice_Battery::readParameterValue")
    }

    /// C++: `void MSDevice_Battery::MSDevice_Battery(const MSDevice_Battery &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSDevice_Battery) -> Self {
        todo!("port MSDevice_Battery::MSDevice_Battery")
    }

    /// C++: `MSDevice_Battery & MSDevice_Battery::operator=(const MSDevice_Battery &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSDevice_Battery) -> &mut MSDevice_Battery {
        todo!("port MSDevice_Battery::operator=")
    }

}

