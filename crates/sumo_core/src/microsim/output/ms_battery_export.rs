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

        let net = MSNet::get_instance();
        let vc = net.m().get_vehicle_control();
        for (_, veh) in vc.myVehicleDict.iter() {
            let mveh: Ptr<MSVehicle> = veh.cast::<MSVehicle>();
            if !(mveh.is_on_road() || mveh.is_parking() || mveh.is_remote_controlled()) {
                continue;
            }
            // Port note: C++ getDevice(typeid(MSDevice_Battery)) is the exact-class lookup, emulated by deviceName (see MSDevice::get_device_by_name)
            let dev = MSDevice::get_device_by_name(&mveh.myDevices, "battery");
            if dev.is_null() {
                continue;
            }
            let battery = dev.cast::<MSDevice_Battery>();
            if battery.get_maximum_battery_capacity() > 0. {
                of.open_tag_id(SUMO_TAG_VEHICLE);
                of.write_attr(SUMO_ATTR_ID, &mveh.get_id().to_string());
                of.write_attr(SUMO_ATTR_ENERGYCONSUMED, &battery.get_consum());
                of.write_attr(SUMO_ATTR_TOTALENERGYCONSUMED, &battery.get_total_consumption());
                of.write_attr(SUMO_ATTR_TOTALENERGYREGENERATED, &battery.get_total_regenerated());
                of.write_attr(SUMO_ATTR_ACTUALBATTERYCAPACITY, &battery.get_actual_battery_capacity());
                of.write_attr(SUMO_ATTR_MAXIMUMBATTERYCAPACITY, &battery.get_maximum_battery_capacity());
                of.write_attr(SUMO_ATTR_CHARGINGSTATIONID, &battery.get_charging_station_id());
                of.write_attr(SUMO_ATTR_ENERGYCHARGED, &battery.get_energy_charged());
                if battery.is_charging_in_transit() {
                    of.write_attr(SUMO_ATTR_ENERGYCHARGEDINTRANSIT, &battery.get_energy_charged());
                } else {
                    of.write_attr(SUMO_ATTR_ENERGYCHARGEDINTRANSIT, &0.00);
                }
                if battery.is_charging_stopped() {
                    of.write_attr(SUMO_ATTR_ENERGYCHARGEDSTOPPED, &battery.get_energy_charged());
                } else {
                    of.write_attr(SUMO_ATTR_ENERGYCHARGEDSTOPPED, &0.00);
                }
                of.write_attr(SUMO_ATTR_SPEED, &mveh.get_speed());
                of.write_attr(SUMO_ATTR_ACCELERATION, &mveh.get_acceleration());
                let pos = mveh.get_position(0.);
                of.write_attr(SUMO_ATTR_X, &pos.x());
                of.write_attr(SUMO_ATTR_Y, &pos.y());
                if crate::microsim::ms_globals::use_meso_sim() {
                    of.write_attr(SUMO_ATTR_EDGE, &mveh.get_edge().get_id().to_string());
                } else {
                    of.write_attr(SUMO_ATTR_LANE, &mveh.get_lane().get_id().to_string());
                }
                of.write_attr(SUMO_ATTR_POSONLANE, &mveh.get_position_on_lane());
                of.write_attr(SUMO_ATTR_TIMESTOPPED, &battery.get_vehicle_stopped());
                of.close_tag("");
            }
        }

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

