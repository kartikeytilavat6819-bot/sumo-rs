//! Port of microsim/trigger/MSChargingStation.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use crate::microsim::ms_event_control::Command;
use utils::common::time::{delta_t, time2string, ts};

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
        MSChargingStation_Charge {
            timeStep: _timeStep,
            vehicleID: _vehicleID,
            vehicleType: _vehicleType,
            status: _status,
            WCharged: _WCharged,
            actualBatteryCapacity: _actualBatteryCapacity,
            maxBatteryCapacity: _maxBatteryCapacity,
            chargingPower: _chargingPower,
            chargingEfficiency: _chargingEfficiency,
            totalEnergyCharged: _totalEnergyCharged,
        }
    }

}

/// C++ `WrappingCommand<MSChargingStation>` wrapping `MSChargingStation::checkTotalPower`
pub struct MSChargingStation_CheckTotalPowerCommand {
    pub station: Ptr<MSChargingStation>,
}

impl Command for MSChargingStation_CheckTotalPowerCommand {
    fn execute(&mut self, currentTime: SUMOTime) -> SUMOTime {
        self.station.m().check_total_power(currentTime)
    }
}

/// C++ `myTotalPowerCheckEvent = new WrappingCommand<MSChargingStation>(this, &MSChargingStation::checkTotalPower);`
/// followed by `MSNet::getInstance()->getEndOfTimestepEvents()->addEvent(myTotalPowerCheckEvent);`
fn schedule_total_power_check(station: &mut MSChargingStation) {
    let cmd: *mut MSChargingStation_CheckTotalPowerCommand = Box::into_raw(Box::new(MSChargingStation_CheckTotalPowerCommand { station: Ptr::from_mut(station) }));
    station.myTotalPowerCheckEvent = Ptr::from_raw(cmd);
    MSNet::get_instance().m().get_end_of_timestep_events().m().add_event(Ptr::from_raw(cmd as *mut dyn Command), -1);
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
    pub myChargeType: MSChargingStation_ChargeType,
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
    pub myTotalPowerCheckEvent: Ptr<MSChargingStation_CheckTotalPowerCommand>,
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
    pub fn charge_type_to_string(type_: MSChargingStation_ChargeType) -> String {
        if type_ == MSChargingStation_ChargeType::CHARGETYPE_NORMAL {
            "normal".to_string()
        } else if type_ == MSChargingStation_ChargeType::CHARGETYPE_BATTERYEXCHANGE {
            "battery-exchange".to_string()
        } else if type_ == MSChargingStation_ChargeType::CHARGETYPE_FUEL {
            "fuel".to_string()
        } else {
            utils::common::msg_handler::write_warning("Encountered an unknown charge type. Assuming charge type 'normal'.");
            "normal".to_string()
        }
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
    pub fn string_to_charge_type(repr: &str) -> MSChargingStation_ChargeType {
        if repr == "normal" {
            MSChargingStation_ChargeType::CHARGETYPE_NORMAL
        } else if repr == "battery-exchange" {
            MSChargingStation_ChargeType::CHARGETYPE_BATTERYEXCHANGE
        } else if repr == "fuel" {
            MSChargingStation_ChargeType::CHARGETYPE_FUEL
        } else {
            write_warningf!("Encountered an unknown charge type string '%'. Assuming charge type 'normal'.", repr);
            MSChargingStation_ChargeType::CHARGETYPE_NORMAL
        }
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
        let mut me = MSChargingStation {
            base: MSStoppingPlace::new(chargingStationID, SUMO_TAG_CHARGING_STATION, &Vec::new(), lane, startPos, endPos, name.to_string(), 0, 0.0, &RGBColor::INVISIBLE, 90.0),
            myNominalChargingPower: 0.0,
            myTotalChargingPower: 0.0,
            myEfficiency: 0.0,
            myChargeInTransit: chargeInTransit,
            myChargeDelay: 0,
            myChargeType: MSChargingStation::string_to_charge_type(chargeType),
            myWaitingTime: 0,
            myChargingVehicle: false,
            myTotalCharge: 0.0,
            myParkingArea: Ptr::null(),
            myChargeValues: BTreeMap::new(),
            myChargedVehicles: Vec::new(),
            myChargedBatteries: BTreeMap::new(),
            myTotalPowerCheckEvent: Ptr::null(),
        };
        if chargingPower < 0.0 {
            utils::common::msg_handler::write_warning(&utils::tlf!("Attribute % for chargingStation with ID='%' is invalid (%).", utils::xml::sumo_xml_definitions::Attrs.get_string(SUMO_ATTR_CHARGINGPOWER), me.get_id(), chargingPower));
        } else {
            me.myNominalChargingPower = chargingPower;
            me.myTotalChargingPower = totalPower;
        }
        if efficency < 0.0 || efficency > 1.0 {
            utils::common::msg_handler::write_warning(&utils::tlf!("Attribute % for chargingStation with ID='%' is invalid (%).", utils::xml::sumo_xml_definitions::Attrs.get_string(SUMO_ATTR_EFFICIENCY), me.get_id(), efficency));
        } else {
            me.myEfficiency = efficency;
        }
        if chargeDelay < 0 {
            utils::common::msg_handler::write_warning(&utils::tlf!("Attribute % for chargingStation with ID='%' is invalid (%).", utils::xml::sumo_xml_definitions::Attrs.get_string(SUMO_ATTR_CHARGEDELAY), me.get_id(), chargeDelay));
        } else {
            me.myChargeDelay = chargeDelay;
        }
        if waitingTime < 0 {
            utils::common::msg_handler::write_warning(&utils::tlf!("Attribute % for chargingStation with ID='%' is invalid (%).", utils::xml::sumo_xml_definitions::Attrs.get_string(SUMO_ATTR_WAITINGTIME), me.get_id(), waitingTime));
        } else {
            me.myWaitingTime = waitingTime;
        }
        if me.get_begin_lane_position() > me.get_end_lane_position() {
            utils::common::msg_handler::write_warning(&utils::tlf!("ChargingStation with ID='%' doesn't have a valid position (% < %).", me.get_id(), me.get_begin_lane_position(), me.get_end_lane_position()));
        }
        me
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
        let lane: &mut MSLane = parkingArea.m().myLane.m();
        let begin = parkingArea.m().get_begin_lane_position();
        let end = parkingArea.m().get_end_lane_position();
        let mut me = Self::new(chargingStationID, lane, begin, end, name, chargingPower, totalPower, efficency, chargeInTransit, chargeDelay, chargeType, waitingTime);
        me.myParkingArea = parkingArea;
        me
    }

    /// C++: `void MSChargingStation::~MSChargingStation() noexcept` [virtual]
    /// destructor
    // MSChargingStation::~MSChargingStation() {
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
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
        if usingFuel {
            self.myNominalChargingPower
        } else {
            // Convert from [Ws] to [Wh] (3600s / 1h):
            self.myNominalChargingPower / 3600.0
        }
    }

    /// C++: `double MSChargingStation::getEfficency() const` [const]
    /// Get efficiency of the charging station
    // double
    // MSChargingStation::getEfficency() const {
    //     return myEfficiency;
    // }
    #[allow(unused_variables)]
    pub fn get_efficency(&self) -> f64 {
        self.myEfficiency
    }

    /// C++: `bool MSChargingStation::getChargeInTransit() const` [const]
    /// Get chargeInTransit
    // bool
    // MSChargingStation::getChargeInTransit() const {
    //     return myChargeInTransit;
    // }
    #[allow(unused_variables)]
    pub fn get_charge_in_transit(&self) -> bool {
        self.myChargeInTransit
    }

    /// C++: `SUMOTime MSChargingStation::getChargeDelay() const` [const]
    /// Get Charge Delay
    // SUMOTime
    // MSChargingStation::getChargeDelay() const {
    //     return myChargeDelay;
    // }
    #[allow(unused_variables)]
    pub fn get_charge_delay(&self) -> SUMOTime {
        self.myChargeDelay
    }

    /// C++: `ChargeType MSChargingStation::getChargeType() const` [const]
    /// Get charge type
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_charge_type(&self) -> MSChargingStation_ChargeType {
        self.myChargeType
    }

    /// C++: `SUMOTime MSChargingStation::getWaitingTime() const` [const]
    /// Get waiting time
    // SUMOTime
    // MSChargingStation::getWaitingTime() const {
    //     return myWaitingTime;
    // }
    #[allow(unused_variables)]
    pub fn get_waiting_time(&self) -> SUMOTime {
        self.myWaitingTime
    }

    /// C++: `const MSParkingArea * MSChargingStation::getParkingArea() const` [const]
    /// Get the parking area the charging station is placed on
    // const MSParkingArea*
    // MSChargingStation::getParkingArea() const {
    //     return myParkingArea;
    // }
    #[allow(unused_variables)]
    pub fn get_parking_area(&self) -> Ptr<MSParkingArea> {
        self.myParkingArea
    }

    /// C++: `void MSChargingStation::setChargingPower(double)`
    /// set charging station's charging power
    // void
    // MSChargingStation::setChargingPower(double chargingPower) {
    //     myNominalChargingPower = chargingPower;
    // }
    #[allow(unused_variables)]
    pub fn set_charging_power(&mut self, chargingPower: f64) {
        self.myNominalChargingPower = chargingPower;
    }

    /// C++: `void MSChargingStation::setEfficiency(double)`
    /// set efficiency of the charging station
    // void
    // MSChargingStation::setEfficiency(double efficiency) {
    //     myEfficiency = efficiency;
    // }
    #[allow(unused_variables)]
    pub fn set_efficiency(&mut self, efficiency: f64) {
        self.myEfficiency = efficiency;
    }

    /// C++: `void MSChargingStation::setChargeDelay(SUMOTime)`
    /// set charging delay of the charging station
    // void
    // MSChargingStation::setChargeDelay(SUMOTime delay) {
    //     myChargeDelay = delay;
    // }
    #[allow(unused_variables)]
    pub fn set_charge_delay(&mut self, delay: SUMOTime) {
        self.myChargeDelay = delay;
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
        self.myChargeInTransit = value;
        if self.myTotalChargingPower > 0.0 && self.myChargeInTransit && self.myTotalPowerCheckEvent.is_null() {
            schedule_total_power_check(self);
        }
    }

    /// C++: `double MSChargingStation::getTotalChargingPower() const` [const]
    /// Get charging station's total power
    // double
    // MSChargingStation::getTotalChargingPower() const {
    //     return myTotalChargingPower;
    // }
    #[allow(unused_variables)]
    pub fn get_total_charging_power(&self) -> f64 {
        self.myTotalChargingPower
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
        if totalPower <= 0.0 {
            return;
        }
        let hadLimit = self.myTotalChargingPower > 0.0;
        self.myTotalChargingPower = totalPower;
        if !hadLimit && (self.myChargeInTransit || self.myChargingVehicle) && self.myTotalPowerCheckEvent.is_null() {
            schedule_total_power_check(self);
        }
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
        self.myChargingVehicle = value;
        if self.myTotalChargingPower > 0.0 && self.myChargingVehicle && self.myTotalPowerCheckEvent.is_null() {
            schedule_total_power_check(self);
        }
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
        if !self.myChargeInTransit && !self.myChargingVehicle {
            self.myTotalPowerCheckEvent = Ptr::null();
            self.myChargedBatteries.clear();
            return 0;
        }
        let mut sumReqWh: f64 = 0.0;
        let mut thisStepCharges: Vec<String> = Vec::new();
        for (key, values) in self.myChargeValues.iter() {
            if MSNet::get_instance().m().get_vehicle_control().get_vehicle(key).is_null() {
                continue;
            }
            let lastcharge = values.last().unwrap();
            if lastcharge.timeStep == currentTime {
                sumReqWh += lastcharge.WCharged;
                thisStepCharges.push(key.clone());
            }
        }
        if thisStepCharges.len() < 2 {
            return delta_t();
        }
        let capWh = self.myTotalChargingPower * self.myEfficiency /*W*/ * ts() /*s*/ / 3600.0; // convert to Wh
        if sumReqWh > capWh && sumReqWh > 0.0 {
            let ratio = capWh / sumReqWh;
            for key in thisStepCharges.iter() {
                let battery: Ptr<MSDevice_Battery> = *self.myChargedBatteries.entry(key.clone()).or_default();
                let abc = battery.m().get_actual_battery_capacity();
                let charge = self.myChargeValues.get_mut(key).unwrap().last_mut().unwrap();

                let deliveredWh = charge.WCharged * ratio;
                let excessWh = charge.WCharged - deliveredWh;
                charge.WCharged = deliveredWh;
                if charge.chargingEfficiency > 0.0 && ts() > 0.0 {
                    // derive power [W] from energy [Wh]: Power = (Energy [Wh] * 3600) [Ws] / (efficiency * TS [s])
                    // ergo we are doing [Ws/s -> W]
                    charge.chargingPower = (deliveredWh * 3600.0) / (charge.chargingEfficiency * ts());
                }
                charge.actualBatteryCapacity = abc - excessWh;
                charge.totalEnergyCharged -= excessWh;

                //  inform also battery device
                battery.m().set_actual_battery_capacity(abc - excessWh);
                battery.m().set_energy_charged(deliveredWh);
                self.myTotalCharge -= excessWh;
            }
        }
        delta_t()
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
        position >= self.get_begin_lane_position() && position <= self.get_end_lane_position()
    }

    /// C++: `bool MSChargingStation::isCharging() const` [const]
    /// Return true if in the current time step charging station is charging a vehicle
    // bool
    // MSChargingStation::isCharging() const {
    //     return myChargingVehicle;
    // }
    #[allow(unused_variables)]
    pub fn is_charging(&self) -> bool {
        self.myChargingVehicle
    }

    /// C++: `double MSChargingStation::getTotalCharged() const` [const]
    // double getTotalCharged() const {
    //         return myTotalCharge;
    //     }
    #[allow(unused_variables)]
    pub fn get_total_charged(&self) -> f64 {
        self.myTotalCharge
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
        if !utils::options::options_cont::is_set("chargingstations-output") {
            return;
        }
        let status: &str = if battery.m().get_charging_start_time() > self.myChargeDelay {
            if battery.m().get_holder().get_speed() < battery.m().get_stopping_threshold() {
                "chargingStopped"
            } else if self.myChargeInTransit {
                "chargingInTransit"
            } else {
                "noCharging"
            }
        } else {
            if self.myChargeInTransit {
                "waitingChargeInTransit"
            } else if battery.m().get_holder().get_speed() < battery.m().get_stopping_threshold() {
                "waitingChargeStopped"
            } else {
                "noWaitingCharge"
            }
        };
        // update total charge
        self.myTotalCharge += WCharged;
        // create charge row and insert it in myChargeValues
        let vehID: String = battery.m().get_holder().get_id().to_string();
        if !self.myChargeValues.contains_key(&vehID) {
            self.myChargedVehicles.push(vehID.clone());
        }
        self.myChargedBatteries.insert(vehID.clone(), battery);
        let C = MSChargingStation_Charge::new(
            MSNet::get_instance().get_current_time_step(),
            vehID.clone(),
            battery.m().get_holder().get_vehicle_type().get_id().to_string(),
            status.to_string(),
            WCharged,
            battery.m().get_actual_battery_capacity(),
            battery.m().get_maximum_battery_capacity(),
            self.myNominalChargingPower,
            self.myEfficiency,
            self.myTotalCharge,
        );
        self.myChargeValues.entry(vehID).or_default().push(C);
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
        let mut chargingSteps: i32 = 0;
        for item in self.myChargeValues.values() {
            chargingSteps += item.len() as i32;
        }
        output.open_tag_id(SUMO_TAG_CHARGING_STATION);
        output.write_attr(SUMO_ATTR_ID, self.get_id());
        output.write_attr(SUMO_ATTR_TOTALENERGYCHARGED, &self.myTotalCharge);
        output.write_attr(SUMO_ATTR_CHARGINGSTEPS, &chargingSteps);
        // start writing
        if self.myChargeValues.len() > 0 {
            for vehID in self.myChargedVehicles.iter() {
                let mut iStart: i32 = 0;
                let chargeSteps = self.myChargeValues.entry(vehID.clone()).or_default();
                while iStart < chargeSteps.len() as i32 {
                    let mut iEnd: i32 = iStart + 1;
                    let mut charged: f64 = chargeSteps[iStart as usize].WCharged;
                    while iEnd < chargeSteps.len() as i32 && chargeSteps[iEnd as usize].timeStep == chargeSteps[(iEnd - 1) as usize].timeStep + delta_t() {
                        charged += chargeSteps[iEnd as usize].WCharged;
                        iEnd += 1;
                    }
                    MSChargingStation::write_vehicle(output, chargeSteps, iStart, iEnd, charged);
                    iStart = iEnd;
                }
            }
        }
        // close charging station tag
        output.close_tag("");
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
        let mut terminatedChargers: Vec<String> = Vec::new();
        let simstep: SUMOTime = MSNet::get_instance().get_current_time_step();
        let my_id: String = self.get_id().to_string();
        for (vehID, chargeValues) in self.myChargeValues.iter() {
            let lastCharge = chargeValues.last().unwrap();
            // no charge during the last time step == has stopped charging
            let finished = lastCharge.timeStep < simstep - delta_t();
            if finished || includeUnfinished {
                if finished {
                    terminatedChargers.push(vehID.clone());
                }
                // aggregate values
                let mut charged: f64 = 0.;
                let mut minPower: f64 = lastCharge.chargingPower;
                let mut maxPower: f64 = lastCharge.chargingPower;
                let mut minCharge: f64 = lastCharge.WCharged;
                let mut maxCharge: f64 = lastCharge.WCharged;
                let mut minEfficiency: f64 = lastCharge.chargingEfficiency;
                let mut maxEfficiency: f64 = lastCharge.chargingEfficiency;
                for charge in chargeValues.iter() {
                    charged += charge.WCharged;
                    if charge.chargingPower < minPower {
                        minPower = charge.chargingPower;
                    }
                    if charge.chargingPower > maxPower {
                        maxPower = charge.chargingPower;
                    }
                    if charge.WCharged < minCharge {
                        minCharge = charge.WCharged;
                    }
                    if charge.WCharged > maxCharge {
                        maxCharge = charge.WCharged;
                    }
                    if charge.chargingEfficiency < minEfficiency {
                        minEfficiency = charge.chargingEfficiency;
                    }
                    if charge.chargingEfficiency > maxEfficiency {
                        maxEfficiency = charge.chargingEfficiency;
                    }
                }
                // actually write the data
                output.open_tag_id(SUMO_TAG_CHARGING_EVENT);
                output.write_attr(SUMO_ATTR_CHARGINGSTATIONID, my_id.as_str());
                output.write_attr(SUMO_ATTR_VEHICLE, lastCharge.vehicleID.as_str());
                output.write_attr(SUMO_ATTR_TYPE, lastCharge.vehicleType.as_str());
                output.write_attr(SUMO_ATTR_TOTALENERGYCHARGED_VEHICLE, &charged);
                output.write_attr(SUMO_ATTR_CHARGINGBEGIN, &time2string(chargeValues[0].timeStep));
                if finished {
                    output.write_attr(SUMO_ATTR_CHARGINGEND, &time2string(lastCharge.timeStep));
                }
                output.write_attr(SUMO_ATTR_ACTUALBATTERYCAPACITY, &lastCharge.actualBatteryCapacity);
                output.write_attr(SUMO_ATTR_MAXIMUMBATTERYCAPACITY, &lastCharge.maxBatteryCapacity);
                output.write_attr(SUMO_ATTR_MINPOWER, &minPower);
                output.write_attr(SUMO_ATTR_MAXPOWER, &maxPower);
                output.write_attr(SUMO_ATTR_MINCHARGE, &minCharge);
                output.write_attr(SUMO_ATTR_MAXCHARGE, &maxCharge);
                output.write_attr(SUMO_ATTR_MINEFFICIENCY, &minEfficiency);
                output.write_attr(SUMO_ATTR_MAXEFFICIENCY, &maxEfficiency);
                output.close_tag("");
            }
        }

        // clear charging data of vehicles which terminated charging
        for vehID in terminatedChargers {
            self.myChargeValues.remove(&vehID);
        }
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
        let first = &chargeSteps[iStart as usize];
        out.open_tag_id(SUMO_TAG_VEHICLE);
        out.write_attr(SUMO_ATTR_ID, first.vehicleID.as_str());
        out.write_attr(SUMO_ATTR_TYPE, first.vehicleType.as_str());
        out.write_attr(SUMO_ATTR_TOTALENERGYCHARGED_VEHICLE, &charged);
        out.write_attr(SUMO_ATTR_CHARGINGBEGIN, &time2string(first.timeStep));
        out.write_attr(SUMO_ATTR_CHARGINGEND, &time2string(chargeSteps[(iEnd - 1) as usize].timeStep));
        for i in iStart..iEnd {
            let c = &chargeSteps[i as usize];
            out.open_tag_id(SUMO_TAG_STEP);
            out.write_attr(SUMO_ATTR_TIME, &time2string(c.timeStep));
            // charge values
            out.write_attr(SUMO_ATTR_CHARGING_STATUS, c.status.as_str());
            out.write_attr(SUMO_ATTR_ENERGYCHARGED, &c.WCharged);
            out.write_attr(SUMO_ATTR_PARTIALCHARGE, &c.totalEnergyCharged);
            // charging values of charging station in this timestep
            out.write_attr(SUMO_ATTR_CHARGINGPOWER, &c.chargingPower);
            out.write_attr(SUMO_ATTR_EFFICIENCY, &c.chargingEfficiency);
            // battery status of vehicle
            out.write_attr(SUMO_ATTR_ACTUALBATTERYCAPACITY, &c.actualBatteryCapacity);
            out.write_attr(SUMO_ATTR_MAXIMUMBATTERYCAPACITY, &c.maxBatteryCapacity);
            // close tag timestep
            out.close_tag("");
        }
        out.close_tag("");
    }

    /// C++: `void MSChargingStation::MSChargingStation(const MSChargingStation &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_3(arg0: &MSChargingStation) -> Self {
        // C++ copy constructor is deleted ("Invalidated"), copying is a programming error
        panic!("MSChargingStation copy constructor is invalidated in C++ and must not be used");
    }

    /// C++: `MSChargingStation & MSChargingStation::operator=(const MSChargingStation &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSChargingStation) -> &mut MSChargingStation {
        // C++ assignment operator is deleted ("Invalidated"), assignment is a programming error
        panic!("MSChargingStation assignment operator is invalidated in C++ and must not be used");
    }

}

