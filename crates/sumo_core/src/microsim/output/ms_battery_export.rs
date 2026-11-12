//! Port of microsim/output/MSBatteryExport.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// Realises dumping Battery Data
/// C++ class `MSBatteryExport` (microsim/output/MSBatteryExport.h)
pub struct MSBatteryExport {
}

impl MSBatteryExport {
    /// C++: `void MSBatteryExport::write(OutputDevice &, SUMOTime, int)` [static]
    /// Writes the complete network state of the given edges into the given device
    // void
    // MSBatteryExport::write(OutputDevice& of, SUMOTime timestep, int precision) {
    //     of.openTag(SUMO_TAG_TIMESTEP).writeAttr(SUMO_ATTR_TIME, time2string(timestep));
    //     of.setPrecision(precision);
    //
    //     MSVehicleControl& vc = MSNet::getInstance()->getVehicleControl();
    //     MSVehicleControl::constVehIt it = vc.loadedVehBegin();
    //     MSVehicleControl::constVehIt end = vc.loadedVehEnd();
    //     for (; it != end; ++it) {
    //         const MSVehicle* veh = static_cast<const MSVehicle*>((*it).second);
    //
    //         if (!(veh->isOnRoad() || veh->isParking() || veh->isRemoteControlled())) {
    //             continue;
    //         }
    //
    //         if (static_cast<MSDevice_Battery*>(veh->getDevice(typeid(MSDevice_Battery))) != nullptr) {
    //             MSDevice_Battery* batteryToExport = dynamic_cast<MSDevice_Battery*>(veh->getDevice(typeid(MSDevice_Battery)));
    //             if (batteryToExport->getMaximumBatteryCapacity() > 0) {
    //                 // Open Row
    //                 of.openTag(SUMO_TAG_VEHICLE);
    //                 // Write ID
    //                 of.writeAttr(SUMO_ATTR_ID, veh->getID());
    //                 // Write consum
    //                 of.writeAttr(SUMO_ATTR_ENERGYCONSUMED, batteryToExport->getConsum());
    //                 // Write total consumption
    //                 of.writeAttr(SUMO_ATTR_TOTALENERGYCONSUMED, batteryToExport->getTotalConsumption());
    //                 // Write total regeneration
    //                 of.writeAttr(SUMO_ATTR_TOTALENERGYREGENERATED, batteryToExport->getTotalRegenerated());
    //                 // Write Actual battery capacity
    //                 of.writeAttr(SUMO_ATTR_ACTUALBATTERYCAPACITY, batteryToExport->getActualBatteryCapacity());
    //                 // Write Maximum battery capacity
    //                 of.writeAttr(SUMO_ATTR_MAXIMUMBATTERYCAPACITY, batteryToExport->getMaximumBatteryCapacity());
    //                 // Write Charging Station ID
    //                 of.writeAttr(SUMO_ATTR_CHARGINGSTATIONID, batteryToExport->getChargingStationID(), batteryToExport->getChargingStation() == nullptr);
    //                 // Write Charge charged in the Battery
    //                 of.writeAttr(SUMO_ATTR_ENERGYCHARGED, batteryToExport->getEnergyCharged());
    //                 // Write ChargeInTransit
    //                 if (batteryToExport->isChargingInTransit()) {
    //                     of.writeAttr(SUMO_ATTR_ENERGYCHARGEDINTRANSIT, batteryToExport->getEnergyCharged());
    //                 } else {
    //                     of.writeAttr(SUMO_ATTR_ENERGYCHARGEDINTRANSIT, 0.00);
    //                 }
    //                 // Write ChargingStopped
    //                 if (batteryToExport->isChargingStopped()) {
    //                     of.writeAttr(SUMO_ATTR_ENERGYCHARGEDSTOPPED, batteryToExport->getEnergyCharged());
    //                 } else {
    //                     of.writeAttr(SUMO_ATTR_ENERGYCHARGEDSTOPPED, 0.00);
    //                 }
    //                 // Write Speed
    //                 of.writeAttr(SUMO_ATTR_SPEED, veh->getSpeed());
    //                 // Write Acceleration
    //                 of.writeAttr(SUMO_ATTR_ACCELERATION, veh->getAcceleration());
    //
    //                 Position pos = veh->getPosition();
    //                 of.writeAttr(SUMO_ATTR_X, pos.x());
    //                 of.writeAttr(SUMO_ATTR_Y, pos.y());
    //
    //                 // Write Lane ID / edge ID
    //                 if (MSGlobals::gUseMesoSim) {
    //                     of.writeAttr(SUMO_ATTR_EDGE, veh->getEdge()->getID());
    //                 } else {
    //                     of.writeAttr(SUMO_ATTR_LANE, veh->getLane()->getID());
    //                 }
    //                 // Write vehicle position in the lane
    //                 of.writeAttr(SUMO_ATTR_POSONLANE, veh->getPositionOnLane());
    //                 // Write Time stopped (In all cases)
    //                 of.writeAttr(SUMO_ATTR_TIMESTOPPED, batteryToExport->getVehicleStopped());
    //                 // Close Row
    //                 of.closeTag();
    //             }
    //         }
    //     }
    //     of.closeTag();
    // }
    #[allow(unused_variables)]
    pub fn write(of: &mut OutputDevice, timestep: SUMOTime, precision: i32) {
        of.open_tag_id(SUMO_TAG_TIMESTEP).write_attr(SUMO_ATTR_TIME, utils::common::time::time2string(timestep).as_str());
        of.set_precision(precision);

        // shortcut: the vehicle rows need static_cast<const MSVehicle*> on each loaded vehicle and getDevice(typeid(MSDevice_Battery)), and neither has a Rust equivalent yet (no downcast, type-keyed getDevice unported), so only the timestep frame is written; upgrade when both exist

        of.close_tag("");
    }

    /// C++: `void MSBatteryExport::MSBatteryExport(const MSBatteryExport &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new(arg0: &MSBatteryExport) -> Self {
        panic!("MSBatteryExport copy constructor is private and invalidated in C++")
    }

    /// C++: `MSBatteryExport & MSBatteryExport::operator=(const MSBatteryExport &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSBatteryExport) -> &mut MSBatteryExport {
        panic!("MSBatteryExport assignment operator is private and invalidated in C++")
    }

}

