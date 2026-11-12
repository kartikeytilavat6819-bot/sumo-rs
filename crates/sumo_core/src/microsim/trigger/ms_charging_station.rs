//! Port of microsim/trigger/MSChargingStation.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ enum `MSChargingStation::ChargeType`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum MSChargingStation_ChargeType {
    CHARGETYPE_NORMAL = 0,
    CHARGETYPE_BATTERYEXCHANGE = 1,
    CHARGETYPE_FUEL = 2,
}

/// struct to save information for the chargingStation output
/// C++ class `MSChargingStation::Charge` (microsim/trigger/MSChargingStation.h)
pub struct MSChargingStation_Charge {
    pub timeStep: SUMOTime,
    pub vehicleID: String,
    pub vehicleType: String,
    /// status
    pub status: String,
    pub WCharged: f64,
    pub actualBatteryCapacity: f64,
    pub maxBatteryCapacity: f64,
    pub chargingPower: f64,
    pub chargingEfficiency: f64,
    pub totalEnergyCharged: f64,
}

impl MSChargingStation_Charge {
    /// C++: `void Charge::Charge(SUMOTime, std::string, std::string, std::string, double, double, double, double, double, double)`
    /// constructor
    // Charge(SUMOTime _timeStep, std::string _vehicleID, std::string _vehicleType, std::string _status,
    //                double _WCharged, double _actualBatteryCapacity, double _maxBatteryCapacity, double _chargingPower,
    //                double _chargingEfficiency, double _totalEnergyCharged) :
    //             timeStep(_timeStep),
    //             vehicleID(_vehicleID),
    //             vehicleType(_vehicleType),
    //             status(_status),
    //             WCharged(_WCharged),
    //             actualBatteryCapacity(_actualBatteryCapacity),
    //             maxBatteryCapacity(_maxBatteryCapacity),
    //             chargingPower(_chargingPower),
    //             chargingEfficiency(_chargingEfficiency),
    //             totalEnergyCharged(_totalEnergyCharged) {}
    #[allow(unused_variables)]
    pub fn new(_timeStep: SUMOTime, _vehicleID: String, _vehicleType: String, _status: String, _WCharged: f64, _actualBatteryCapacity: f64, _maxBatteryCapacity: f64, _chargingPower: f64, _chargingEfficiency: f64, _totalEnergyCharged: f64) -> Self {
        todo!("port Charge::Charge")
    }

}

/// Definition of charging stations
/// C++ class `MSChargingStation` (microsim/trigger/MSChargingStation.h)
#[repr(C)]
pub struct MSChargingStation {
    /// C++ base class `MSStoppingPlace`
    pub base: MSStoppingPlace,
    /// Charging station's nominal charging power per vehicle
    pub myNominalChargingPower: f64,
    /// The maximal charging power available to serve all charging vehicles (value <= 0 take no effect)
    pub myTotalChargingPower: f64,
    /// Efficiency of the charging station
    pub myEfficiency: f64,
    /// Allow charge in transit
    pub myChargeInTransit: bool,
    /// Charge Delay
    pub myChargeDelay: SUMOTime,
    /// charge type
    pub myChargeType: ChargeType,
    /// waiting time
    pub myWaitingTime: SUMOTime,
    /// Check if in the current TimeStep chargingStation is charging a vehicle
    pub myChargingVehicle: bool,
    /// total energy charged by this charging station
    pub myTotalCharge: f64,
    /// parkingArea the charging station is placed on
    pub myParkingArea: Ptr<MSParkingArea>,
    /// map with the charges of this charging station (key = vehicleID)
    pub myChargeValues: BTreeMap<String, Vec<MSChargingStation_Charge>>,
    /// order vehicles by time of first charge
    pub myChargedVehicles: Vec<String>,
    /// map with the Batteries charged by this charging station (key = vehicleID)
    pub myChargedBatteries: BTreeMap<String, Ptr<MSDevice_Battery>>,
    /// Event for checking at every time-step if myTotalPower has been exceeded
    pub myTotalPowerCheckEvent: Ptr<Unmapped /* Command */>,
}

impl std::ops::Deref for MSChargingStation {
    type Target = MSStoppingPlace;
    fn deref(&self) -> &MSStoppingPlace {
        &self.base
    }
}
impl std::ops::DerefMut for MSChargingStation {
    fn deref_mut(&mut self) -> &mut MSStoppingPlace {
        &mut self.base
    }
}

impl MSChargingStation {
    /// C++: `std::string MSChargingStation::chargeTypeToString(ChargeType)` [static]
    /// Get the string representation of a charge type
    // static inline std::string chargeTypeToString(ChargeType type) {
    //         if (type == CHARGETYPE_NORMAL) {
    //             return "normal";
    //         } else if (type == CHARGETYPE_BATTERYEXCHANGE) {
    //             return "battery-exchange";
    //         } else if (type == CHARGETYPE_FUEL) {
    //             return "fuel";
    //         } else {
    //             WRITE_WARNING(TL("Encountered an unknown charge type. Assuming charge type 'normal'."));
    //             return "normal";
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn charge_type_to_string(type_: ChargeType) -> String {
        todo!("port MSChargingStation::chargeTypeToString")
    }

    /// C++: `ChargeType MSChargingStation::stringToChargeType(const std::string &)` [static]
    /// Get the charge type from its string representation
    // static inline ChargeType stringToChargeType(const std::string& repr) {
    //         if (repr == "normal") {
    //             return ChargeType::CHARGETYPE_NORMAL;
    //         } else if (repr == "battery-exchange") {
    //             return ChargeType::CHARGETYPE_BATTERYEXCHANGE;
    //         } else if (repr == "fuel") {
    //             return ChargeType::CHARGETYPE_FUEL;
    //         } else {
    //             WRITE_WARNINGF("Encountered an unknown charge type string '%'. Assuming charge type 'normal'.", repr);
    //             return ChargeType::CHARGETYPE_NORMAL;
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn string_to_charge_type(repr: &str) -> ChargeType {
        todo!("port MSChargingStation::stringToChargeType")
    }

    /// C++: `void MSChargingStation::MSChargingStation(const std::string &, MSLane &, double, double, const std::string &, double, double, double, bool, SUMOTime, const std::string &, SUMOTime)`
    /// constructor
    // MSChargingStation::MSChargingStation(const std::string& chargingStationID, MSLane& lane, double startPos, double endPos,
    //                                      const std::string& name, double chargingPower, double totalPower, double efficency, bool chargeInTransit,
    //                                      SUMOTime chargeDelay, const std::string& chargeType, SUMOTime waitingTime) :
    //     MSStoppingPlace(chargingStationID, SUMO_TAG_CHARGING_STATION, std::vector<std::string>(), lane, startPos, endPos, name),
    //     myChargeInTransit(chargeInTransit), myChargeType(stringToChargeType(chargeType)), myTotalPowerCheckEvent(nullptr) {
    //     if (chargingPower < 0) {
    //         WRITE_WARNING(TLF("Attribute % for chargingStation with ID='%' is invalid (%).", toString(SUMO_ATTR_CHARGINGPOWER), getID(), toString(chargingPower)))
    //     } else {
    //         myNominalChargingPower = chargingPower;
    //         myTotalChargingPower = totalPower;
    //     }
    //     if (efficency < 0 || efficency > 1) {
    //         WRITE_WARNING(TLF("Attribute % for chargingStation with ID='%' is invalid (%).", toString(SUMO_ATTR_EFFICIENCY), getID(), toString(efficency)))
    //     } else {
    //         myEfficiency = efficency;
    //     }
    //     if (chargeDelay < 0) {
    //         WRITE_WARNING(TLF("Attribute % for chargingStation with ID='%' is invalid (%).", toString(SUMO_ATTR_CHARGEDELAY), getID(), toString(chargeDelay)))
    //     } else {
    //         myChargeDelay = chargeDelay;
    //     }
    //     if (waitingTime < 0) {
    //         WRITE_WARNING(TLF("Attribute % for chargingStation with ID='%' is invalid (%).", toString(SUMO_ATTR_WAITINGTIME), getID(), toString(waitingTime)))
    //     } else {
    //         myWaitingTime = waitingTime;
    //     }
    //     if (getBeginLanePosition() > getEndLanePosition()) {
    //         WRITE_WARNING(TLF("ChargingStation with ID='%' doesn't have a valid position (% < %).", getID(), toString(getBeginLanePosition()), toString(getEndLanePosition())));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn new(chargingStationID: &str, lane: &mut MSLane, startPos: f64, endPos: f64, name: &str, chargingPower: f64, totalPower: f64, efficency: f64, chargeInTransit: bool, chargeDelay: SUMOTime, chargeType: &str, waitingTime: SUMOTime) -> Self {
        todo!("port MSChargingStation::MSChargingStation")
    }

    /// C++: `void MSChargingStation::MSChargingStation(const std::string &, const MSParkingArea *, const std::string &, double, double, double, bool, SUMOTime, const std::string &, SUMOTime)`
    // MSChargingStation::MSChargingStation(const std::string& chargingStationID, const MSParkingArea* parkingArea, const std::string& name, double chargingPower,
    //                                      double totalPower, double efficency, bool chargeInTransit, SUMOTime chargeDelay, const std::string& chargeType, SUMOTime waitingTime) :
    //     MSChargingStation(chargingStationID, const_cast<MSLane&>(parkingArea->getLane()), parkingArea->getBeginLanePosition(), parkingArea->getEndLanePosition(),
    //                       name, chargingPower, totalPower, efficency, chargeInTransit, chargeDelay, chargeType, waitingTime) {
    //     myParkingArea = parkingArea;
    // }
    #[allow(unused_variables)]
    pub fn new_2(chargingStationID: &str, parkingArea: Ptr<MSParkingArea>, name: &str, chargingPower: f64, totalPower: f64, efficency: f64, chargeInTransit: bool, chargeDelay: SUMOTime, chargeType: &str, waitingTime: SUMOTime) -> Self {
        todo!("port MSChargingStation::MSChargingStation")
    }

    /// C++: `void MSChargingStation::~MSChargingStation() noexcept` [virtual]
    /// destructor
    // MSChargingStation::~MSChargingStation() {
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MSChargingStation::~MSChargingStation")
    }

    /// C++: `double MSChargingStation::getChargingPower(bool) const` [const]
    /// Get charging station's charging power
    // double
    // MSChargingStation::getChargingPower(bool usingFuel) const {
    //     if (usingFuel) {
    //         return myNominalChargingPower;
    //     } else {
    //         // Convert from [Ws] to [Wh] (3600s / 1h):
    //         return myNominalChargingPower / 3600;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_charging_power(&self, usingFuel: bool) -> f64 {
        todo!("port MSChargingStation::getChargingPower")
    }

    /// C++: `double MSChargingStation::getEfficency() const` [const]
    /// Get efficiency of the charging station
    // double
    // MSChargingStation::getEfficency() const {
    //     return myEfficiency;
    // }
    #[allow(unused_variables)]
    pub fn get_efficency(&self) -> f64 {
        todo!("port MSChargingStation::getEfficency")
    }

    /// C++: `bool MSChargingStation::getChargeInTransit() const` [const]
    /// Get chargeInTransit
    // bool
    // MSChargingStation::getChargeInTransit() const {
    //     return myChargeInTransit;
    // }
    #[allow(unused_variables)]
    pub fn get_charge_in_transit(&self) -> bool {
        todo!("port MSChargingStation::getChargeInTransit")
    }

    /// C++: `SUMOTime MSChargingStation::getChargeDelay() const` [const]
    /// Get Charge Delay
    // SUMOTime
    // MSChargingStation::getChargeDelay() const {
    //     return myChargeDelay;
    // }
    #[allow(unused_variables)]
    pub fn get_charge_delay(&self) -> SUMOTime {
        todo!("port MSChargingStation::getChargeDelay")
    }

    /// C++: `ChargeType MSChargingStation::getChargeType() const` [const]
    /// Get charge type
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_charge_type(&self) -> ChargeType {
        todo!("port MSChargingStation::getChargeType")
    }

    /// C++: `SUMOTime MSChargingStation::getWaitingTime() const` [const]
    /// Get waiting time
    // SUMOTime
    // MSChargingStation::getWaitingTime() const {
    //     return myWaitingTime;
    // }
    #[allow(unused_variables)]
    pub fn get_waiting_time(&self) -> SUMOTime {
        todo!("port MSChargingStation::getWaitingTime")
    }

    /// C++: `const MSParkingArea * MSChargingStation::getParkingArea() const` [const]
    /// Get the parking area the charging station is placed on
    // const MSParkingArea*
    // MSChargingStation::getParkingArea() const {
    //     return myParkingArea;
    // }
    #[allow(unused_variables)]
    pub fn get_parking_area(&self) -> Ptr<MSParkingArea> {
        todo!("port MSChargingStation::getParkingArea")
    }

    /// C++: `void MSChargingStation::setChargingPower(double)`
    /// set charging station's charging power
    // void
    // MSChargingStation::setChargingPower(double chargingPower) {
    //     myNominalChargingPower = chargingPower;
    // }
    #[allow(unused_variables)]
    pub fn set_charging_power(&mut self, chargingPower: f64) {
        todo!("port MSChargingStation::setChargingPower")
    }

    /// C++: `void MSChargingStation::setEfficiency(double)`
    /// set efficiency of the charging station
    // void
    // MSChargingStation::setEfficiency(double efficiency) {
    //     myEfficiency = efficiency;
    // }
    #[allow(unused_variables)]
    pub fn set_efficiency(&mut self, efficiency: f64) {
        todo!("port MSChargingStation::setEfficiency")
    }

    /// C++: `void MSChargingStation::setChargeDelay(SUMOTime)`
    /// set charging delay of the charging station
    // void
    // MSChargingStation::setChargeDelay(SUMOTime delay) {
    //     myChargeDelay = delay;
    // }
    #[allow(unused_variables)]
    pub fn set_charge_delay(&mut self, delay: SUMOTime) {
        todo!("port MSChargingStation::setChargeDelay")
    }

    /// C++: `void MSChargingStation::setChargeInTransit(bool)`
    /// set charging in transit
    // void
    // MSChargingStation::setChargeInTransit(bool value) {
    //     myChargeInTransit = value;
    //     if (myTotalChargingPower > 0 && myChargeInTransit && myTotalPowerCheckEvent == nullptr) {
    //         myTotalPowerCheckEvent = new WrappingCommand<MSChargingStation>(this, &MSChargingStation::checkTotalPower);
    //         MSNet::getInstance()->getEndOfTimestepEvents()->addEvent(myTotalPowerCheckEvent);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_charge_in_transit(&mut self, value: bool) {
        todo!("port MSChargingStation::setChargeInTransit")
    }

    /// C++: `double MSChargingStation::getTotalChargingPower() const` [const]
    /// Get charging station's total power
    // double
    // MSChargingStation::getTotalChargingPower() const {
    //     return myTotalChargingPower;
    // }
    #[allow(unused_variables)]
    pub fn get_total_charging_power(&self) -> f64 {
        todo!("port MSChargingStation::getTotalChargingPower")
    }

    /// C++: `void MSChargingStation::setTotalChargingPower(double)`
    /// set charging station's total power
    // void
    // MSChargingStation::setTotalChargingPower(double totalPower) {
    //     if (totalPower <= 0) {
    //         return;
    //     }
    //     const bool hadLimit = myTotalChargingPower > 0;
    //     myTotalChargingPower = totalPower;
    //     if (!hadLimit && (myChargeInTransit || myChargingVehicle) && myTotalPowerCheckEvent == nullptr) {
    //         myTotalPowerCheckEvent = new WrappingCommand<MSChargingStation>(this, &MSChargingStation::checkTotalPower);
    //         MSNet::getInstance()->getEndOfTimestepEvents()->addEvent(myTotalPowerCheckEvent);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_total_charging_power(&mut self, totalPower: f64) {
        todo!("port MSChargingStation::setTotalChargingPower")
    }

    /// C++: `void MSChargingStation::setChargingVehicle(bool)`
    /// enable or disable charging vehicle
    // void
    // MSChargingStation::setChargingVehicle(bool value) {
    //     myChargingVehicle = value;
    //     if (myTotalChargingPower > 0 && myChargingVehicle && myTotalPowerCheckEvent == nullptr) {
    //         myTotalPowerCheckEvent = new WrappingCommand<MSChargingStation>(this, &MSChargingStation::checkTotalPower);
    //         MSNet::getInstance()->getEndOfTimestepEvents()->addEvent(myTotalPowerCheckEvent);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_charging_vehicle(&mut self, value: bool) {
        todo!("port MSChargingStation::setChargingVehicle")
    }

    /// C++: `SUMOTime MSChargingStation::checkTotalPower(SUMOTime)`
    /// update the delivered power to all charging vehicles after all requests are known
    // SUMOTime
    // MSChargingStation::checkTotalPower(SUMOTime currentTime) {
    //     if (!myChargeInTransit && !myChargingVehicle) {
    //         myTotalPowerCheckEvent = nullptr;
    //         myChargedBatteries.clear();
    //         return 0;
    //     }
    //     double sumReqWh = 0;
    //     std::vector<Charge*> thisStepCharges;
    //     for (auto& kv : myChargeValues) {
    //         if (MSNet::getInstance()->getVehicleControl().getVehicle(kv.first) == nullptr) {
    //             continue;
    //         }
    //         Charge& lastcharge = kv.second.back();
    //         if (lastcharge.timeStep == currentTime) {
    //             sumReqWh += lastcharge.WCharged;
    //             thisStepCharges.push_back(&lastcharge);
    //         }
    //     }
    //     if (thisStepCharges.size() < 2) {
    //         return DELTA_T;
    //     }
    //     const double capWh = myTotalChargingPower * myEfficiency /*W*/ * TS /*s*/ / 3600.0; // convert to Wh
    // #ifdef DEBUG_SIMSTEP
    //     std::cout << "checkTotalPower: CS="
    //               << this->myID << " currentTime=" << currentTime << " myTotalChargingPower=" << myTotalChargingPower;
    //     if (sumReqWh > capWh && sumReqWh > 0) {
    //         std::cout << " exceeded, needs rebalancing!";
    //     }
    //     std::cout << std::endl;
    // #endif
    //     if (sumReqWh > capWh && sumReqWh > 0) {
    //         const double ratio = capWh / sumReqWh;
    //         for (auto* charge : thisStepCharges) {
    //             MSDevice_Battery* battery = myChargedBatteries[charge->vehicleID];
    //             double abc = battery->getActualBatteryCapacity();
    //
    //             const double deliveredWh = charge->WCharged * ratio;
    //             const double excessWh = charge->WCharged - deliveredWh;
    //             charge->WCharged = deliveredWh;
    //             if (charge->chargingEfficiency > 0 && TS > 0) {
    //                 // derive power [W] from energy [Wh]: Power = (Energy [Wh] * 3600) [Ws] / (efficiency * TS [s])
    //                 // ergo we are doing [Ws/s -> W]
    //                 charge->chargingPower = (deliveredWh * 3600.0) / (charge->chargingEfficiency * TS);
    //             }
    //             charge->actualBatteryCapacity = abc - excessWh;
    //             charge->totalEnergyCharged -= excessWh;
    //
    //             //  inform also battery device
    //             battery->setActualBatteryCapacity(abc - excessWh);
    //             battery->setEnergyCharged(deliveredWh);
    //             myTotalCharge -= excessWh;
    //
    // #ifdef DEBUG_SIMSTEP
    //             std::cout << "time=" << time2string(currentTime)
    //                       << " vehID=" << charge->vehicleID
    //                       << " requestedWh=" << (deliveredWh + excessWh)
    //                       << " deliveredWh=" << deliveredWh
    //                       << " deliveredW="  << charge->chargingPower
    //                       << " ratio=" << ratio << std::endl;
    // #endif
    //         }
    // #ifdef DEBUG_SIMSTEP
    //         std::cout << "===============\n\n";
    // #endif
    //     }
    //     return DELTA_T;
    // }
    #[allow(unused_variables)]
    pub fn check_total_power(&mut self, currentTime: SUMOTime) -> SUMOTime {
        todo!("port MSChargingStation::checkTotalPower")
    }

    /// C++: `bool MSChargingStation::vehicleIsInside(const double) const` [const]
    /// Check if a vehicle is inside in the Charge Station
    // bool
    // MSChargingStation::vehicleIsInside(const double position) const {
    //     if ((position >= getBeginLanePosition()) && (position <= getEndLanePosition())) {
    //         return true;
    //     } else {
    //         return false;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn vehicle_is_inside(&self, position: f64) -> bool {
        todo!("port MSChargingStation::vehicleIsInside")
    }

    /// C++: `bool MSChargingStation::isCharging() const` [const]
    /// Return true if in the current time step charging station is charging a vehicle
    // bool
    // MSChargingStation::isCharging() const {
    //     return myChargingVehicle;
    // }
    #[allow(unused_variables)]
    pub fn is_charging(&self) -> bool {
        todo!("port MSChargingStation::isCharging")
    }

    /// C++: `double MSChargingStation::getTotalCharged() const` [const]
    // double getTotalCharged() const {
    //         return myTotalCharge;
    //     }
    #[allow(unused_variables)]
    pub fn get_total_charged(&self) -> f64 {
        todo!("port MSChargingStation::getTotalCharged")
    }

    /// C++: `void MSChargingStation::addChargeValueForOutput(double, MSDevice_Battery *)`
    /// add charge value for output
    // void
    // MSChargingStation::addChargeValueForOutput(double WCharged, MSDevice_Battery* battery) {
    //     if (!OptionsCont::getOptions().isSet("chargingstations-output")) {
    //         return;
    //     }
    //     std::string status = "";
    //     if (battery->getChargingStartTime() > myChargeDelay) {
    //         if (battery->getHolder().getSpeed() < battery->getStoppingThreshold()) {
    //             status = "chargingStopped";
    //         } else if (myChargeInTransit) {
    //             status = "chargingInTransit";
    //         } else {
    //             status = "noCharging";
    //         }
    //     } else {
    //         if (myChargeInTransit) {
    //             status = "waitingChargeInTransit";
    //         } else if (battery->getHolder().getSpeed() < battery->getStoppingThreshold()) {
    //             status = "waitingChargeStopped";
    //         } else {
    //             status = "noWaitingCharge";
    //         }
    //     }
    //     // update total charge
    //     myTotalCharge += WCharged;
    //     // create charge row and insert it in myChargeValues
    //     const std::string vehID = battery->getHolder().getID();
    //     if (myChargeValues.count(vehID) == 0) {
    //         myChargedVehicles.push_back(vehID);
    //     }
    //     myChargedBatteries[vehID] = battery;
    //     Charge C(MSNet::getInstance()->getCurrentTimeStep(), vehID, battery->getHolder().getVehicleType().getID(),
    //              status, WCharged, battery->getActualBatteryCapacity(), battery->getMaximumBatteryCapacity(),
    //              myNominalChargingPower, myEfficiency, myTotalCharge);
    //     myChargeValues[vehID].push_back(C);
    // }
    #[allow(unused_variables)]
    pub fn add_charge_value_for_output(&mut self, WCharged: f64, battery: Ptr<MSDevice_Battery>) {
        todo!("port MSChargingStation::addChargeValueForOutput")
    }

    /// C++: `void MSChargingStation::writeChargingStationOutput(OutputDevice &)`
    /// write charging station values
    // void
    // MSChargingStation::writeChargingStationOutput(OutputDevice& output) {
    //     int chargingSteps = 0;
    //     for (const auto& item : myChargeValues) {
    //         chargingSteps += (int)item.second.size();
    //     }
    //     output.openTag(SUMO_TAG_CHARGING_STATION);
    //     output.writeAttr(SUMO_ATTR_ID, myID);
    //     output.writeAttr(SUMO_ATTR_TOTALENERGYCHARGED, myTotalCharge);
    //     output.writeAttr(SUMO_ATTR_CHARGINGSTEPS, chargingSteps);
    //     // start writing
    //     if (myChargeValues.size() > 0) {
    //         for (const std::string& vehID : myChargedVehicles) {
    //             int iStart = 0;
    //             const auto& chargeSteps = myChargeValues[vehID];
    //             while (iStart < (int)chargeSteps.size()) {
    //                 int iEnd = iStart + 1;
    //                 double charged = chargeSteps[iStart].WCharged;
    //                 while (iEnd < (int)chargeSteps.size() && chargeSteps[iEnd].timeStep == chargeSteps[iEnd - 1].timeStep + DELTA_T) {
    //                     charged += chargeSteps[iEnd].WCharged;
    //                     iEnd++;
    //                 }
    //                 writeVehicle(output, chargeSteps, iStart, iEnd, charged);
    //                 iStart = iEnd;
    //             }
    //         }
    //     }
    //     // close charging station tag
    //     output.closeTag();
    // }
    #[allow(unused_variables)]
    pub fn write_charging_station_output(&mut self, output: &mut OutputDevice) {
        todo!("port MSChargingStation::writeChargingStationOutput")
    }

    /// C++: `void MSChargingStation::writeAggregatedChargingStationOutput(OutputDevice &, bool)`
    /// write ungrouped output (flush data after writing)
    // void
    // MSChargingStation::writeAggregatedChargingStationOutput(OutputDevice& output, bool includeUnfinished) {
    //     std::vector<std::string> terminatedChargers;
    //     for (const auto& item : myChargeValues) {
    //         const Charge& lastCharge = item.second.back();
    //         // no charge during the last time step == has stopped charging
    //         bool finished = lastCharge.timeStep < SIMSTEP - DELTA_T;
    //         if (finished || includeUnfinished) {
    //             if (finished) {
    //                 terminatedChargers.push_back(item.first);
    //             }
    //             // aggregate values
    //             double charged = 0.;
    //             double minPower = lastCharge.chargingPower;
    //             double maxPower = lastCharge.chargingPower;
    //             double minCharge = lastCharge.WCharged;
    //             double maxCharge = lastCharge.WCharged;
    //             double minEfficiency = lastCharge.chargingEfficiency;
    //             double maxEfficiency = lastCharge.chargingEfficiency;
    //             for (const auto& charge : item.second) {
    //                 charged += charge.WCharged;
    //                 if (charge.chargingPower < minPower) {
    //                     minPower = charge.chargingPower;
    //                 }
    //                 if (charge.chargingPower > maxPower) {
    //                     maxPower = charge.chargingPower;
    //                 }
    //                 if (charge.WCharged < minCharge) {
    //                     minCharge = charge.WCharged;
    //                 }
    //                 if (charge.WCharged > maxCharge) {
    //                     maxCharge = charge.WCharged;
    //                 }
    //                 if (charge.chargingEfficiency < minEfficiency) {
    //                     minEfficiency = charge.chargingEfficiency;
    //                 }
    //                 if (charge.chargingEfficiency > maxEfficiency) {
    //                     maxEfficiency = charge.chargingEfficiency;
    //                 }
    //             }
    //             // actually write the data
    //             output.openTag(SUMO_TAG_CHARGING_EVENT);
    //             output.writeAttr(SUMO_ATTR_CHARGINGSTATIONID, myID);
    //             output.writeAttr(SUMO_ATTR_VEHICLE, lastCharge.vehicleID);
    //             output.writeAttr(SUMO_ATTR_TYPE, lastCharge.vehicleType);
    //             output.writeAttr(SUMO_ATTR_TOTALENERGYCHARGED_VEHICLE, charged);
    //             output.writeAttr(SUMO_ATTR_CHARGINGBEGIN, time2string(item.second.at(0).timeStep));
    //             if (finished) {
    //                 output.writeAttr(SUMO_ATTR_CHARGINGEND, time2string(lastCharge.timeStep));
    //             }
    //             output.writeAttr(SUMO_ATTR_ACTUALBATTERYCAPACITY, lastCharge.actualBatteryCapacity);
    //             output.writeAttr(SUMO_ATTR_MAXIMUMBATTERYCAPACITY, lastCharge.maxBatteryCapacity);
    //             output.writeAttr(SUMO_ATTR_MINPOWER, minPower);
    //             output.writeAttr(SUMO_ATTR_MAXPOWER, maxPower);
    //             output.writeAttr(SUMO_ATTR_MINCHARGE, minCharge);
    //             output.writeAttr(SUMO_ATTR_MAXCHARGE, maxCharge);
    //             output.writeAttr(SUMO_ATTR_MINEFFICIENCY, minEfficiency);
    //             output.writeAttr(SUMO_ATTR_MAXEFFICIENCY, maxEfficiency);
    //             output.closeTag();
    //         }
    //     }
    //
    //     // clear charging data of vehicles which terminated charging
    //     for (auto vehID : terminatedChargers) {
    //         myChargeValues.erase(vehID);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn write_aggregated_charging_station_output(&mut self, output: &mut OutputDevice, includeUnfinished: bool) {
        todo!("port MSChargingStation::writeAggregatedChargingStationOutput")
    }

    /// C++: `void MSChargingStation::writeVehicle(OutputDevice &, const std::vector<Charge> &, int, int, double)` [static, protected]
    // void
    // MSChargingStation::writeVehicle(OutputDevice& out, const std::vector<Charge>& chargeSteps, int iStart, int iEnd, double charged) {
    //     const Charge& first = chargeSteps[iStart];
    //     out.openTag(SUMO_TAG_VEHICLE);
    //     out.writeAttr(SUMO_ATTR_ID, first.vehicleID);
    //     out.writeAttr(SUMO_ATTR_TYPE, first.vehicleType);
    //     out.writeAttr(SUMO_ATTR_TOTALENERGYCHARGED_VEHICLE, charged);
    //     out.writeAttr(SUMO_ATTR_CHARGINGBEGIN, time2string(first.timeStep));
    //     out.writeAttr(SUMO_ATTR_CHARGINGEND, time2string(chargeSteps[iEnd - 1].timeStep));
    //     for (int i = iStart; i < iEnd; i++) {
    //         const Charge& c = chargeSteps[i];
    //         out.openTag(SUMO_TAG_STEP);
    //         out.writeAttr(SUMO_ATTR_TIME, time2string(c.timeStep));
    //         // charge values
    //         out.writeAttr(SUMO_ATTR_CHARGING_STATUS, c.status);
    //         out.writeAttr(SUMO_ATTR_ENERGYCHARGED, c.WCharged);
    //         out.writeAttr(SUMO_ATTR_PARTIALCHARGE, c.totalEnergyCharged);
    //         // charging values of charging station in this timestep
    //         out.writeAttr(SUMO_ATTR_CHARGINGPOWER, c.chargingPower);
    //         out.writeAttr(SUMO_ATTR_EFFICIENCY, c.chargingEfficiency);
    //         // battery status of vehicle
    //         out.writeAttr(SUMO_ATTR_ACTUALBATTERYCAPACITY, c.actualBatteryCapacity);
    //         out.writeAttr(SUMO_ATTR_MAXIMUMBATTERYCAPACITY, c.maxBatteryCapacity);
    //         // close tag timestep
    //         out.closeTag();
    //     }
    //     out.closeTag();
    // }
    #[allow(unused_variables)]
    pub fn write_vehicle(out: &mut OutputDevice, chargeSteps: &Vec<MSChargingStation_Charge>, iStart: i32, iEnd: i32, charged: f64) {
        todo!("port MSChargingStation::writeVehicle")
    }

    /// C++: `void MSChargingStation::MSChargingStation(const MSChargingStation &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_3(arg0: &MSChargingStation) -> Self {
        todo!("port MSChargingStation::MSChargingStation")
    }

    /// C++: `MSChargingStation & MSChargingStation::operator=(const MSChargingStation &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSChargingStation) -> &mut MSChargingStation {
        todo!("port MSChargingStation::operator=")
    }

}

