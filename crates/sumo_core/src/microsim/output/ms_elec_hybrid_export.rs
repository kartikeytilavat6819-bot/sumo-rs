//! Port of microsim/output/MSElecHybridExport.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// Realises dumping Battery Data
/// C++ class `MSElecHybridExport` (microsim/output/MSElecHybridExport.h)
pub struct MSElecHybridExport {
}

impl MSElecHybridExport {
    /// C++: `void MSElecHybridExport::write(OutputDevice &, const SUMOVehicle *, SUMOTime, int)` [static]
    /// Writes the complete network state of the given edges into the given device
    // void
    // MSElecHybridExport::write(OutputDevice& of, const SUMOVehicle* veh, SUMOTime timestep, int precision) {
    //     of.openTag(SUMO_TAG_TIMESTEP).writeAttr(SUMO_ATTR_TIME, time2string(timestep));
    //     of.setPrecision(precision);
    //
    //     if (!veh->isOnRoad()) {
    //         return;
    //     }
    //
    //     const MSVehicle* microVeh = dynamic_cast<const MSVehicle*>(veh);
    //
    //     if (static_cast<MSDevice_ElecHybrid*>(veh->getDevice(typeid(MSDevice_ElecHybrid))) != nullptr) {
    //         MSDevice_ElecHybrid* elecHybridToExport = dynamic_cast<MSDevice_ElecHybrid*>(veh->getDevice(typeid(MSDevice_ElecHybrid)));
    //         // Write Actual battery capacity
    //         of.writeAttr(SUMO_ATTR_ACTUALBATTERYCAPACITY, elecHybridToExport->getActualBatteryCapacity());
    //
    //         // Write consumed energy [Wh] (computed by HelpersEnergy::compute)
    //         of.writeAttr(SUMO_ATTR_ENERGYCONSUMED, elecHybridToExport->getConsum());
    //         // Write Energy charged in the Battery [Wh] (drawn energy from overhead wire minus consumed energy)
    //         of.writeAttr(SUMO_ATTR_ENERGYCHARGED, elecHybridToExport->getEnergyCharged());
    //         // Write Power demand (requsted from overhed wire) [W]
    //         of.writeAttr(SUMO_ATTR_CHARGINGPOWER, elecHybridToExport->getPowerWanted());
    //
    //         // Write OverheadWire Segment ID
    //         of.writeAttr(SUMO_ATTR_OVERHEADWIREID, elecHybridToExport->getOverheadWireSegmentID());
    //         // Write Traction Substation ID
    //         of.writeAttr(SUMO_ATTR_TRACTIONSUBSTATIONID, elecHybridToExport->getTractionSubstationID());
    //
    //         // Write current from overheadwire
    //         of.writeAttr(SUMO_ATTR_CURRENTFROMOVERHEADWIRE, elecHybridToExport->getCurrentFromOverheadWire());
    //         // Write voltage of overheadwire
    //         of.writeAttr(SUMO_ATTR_VOLTAGEOFOVERHEADWIRE, elecHybridToExport->getVoltageOfOverheadWire());
    //         // Write circuit alpha best (1 if the traction substation is not overloaded, number from interval [0,1) if the traction substation is overloaded, NAN if it is not applicable)
    //         of.writeAttr(SUMO_ATTR_ALPHACIRCUITSOLVER, elecHybridToExport->getCircuitAlpha());
    //
    //         // Write Speed
    //         of.writeAttr(SUMO_ATTR_SPEED, veh->getSpeed());
    //         // Write Acceleration
    //         of.writeAttr(SUMO_ATTR_ACCELERATION, veh->getAcceleration());
    //         // Write Distance
    //         of.writeAttr(SUMO_ATTR_DISTANCE, veh->getOdometer());
    //         // Write pos x
    //         of.writeAttr(SUMO_ATTR_X, veh->getPosition().x());
    //         // Write pos y
    //         of.writeAttr(SUMO_ATTR_Y, veh->getPosition().y());
    //         // Write pos z
    //         of.writeAttr(SUMO_ATTR_Z, veh->getPosition().z());
    //         // Write slope
    //         of.writeAttr(SUMO_ATTR_SLOPE, veh->getSlope());
    //         // Write Lane ID
    //         if (microVeh != 0) {
    //             of.writeAttr(SUMO_ATTR_LANE, microVeh->getLane()->getID());
    //         }
    //         // Write vehicle position in the lane
    //         of.writeAttr(SUMO_ATTR_POSONLANE, veh->getPositionOnLane());
    //     }
    //     of.closeTag();
    // }
    #[allow(unused_variables)]
    pub fn write(of: &mut OutputDevice, veh: Ptr<SUMOVehicle>, timestep: SUMOTime, precision: i32) {
        of.open_tag_id(SUMO_TAG_TIMESTEP).write_attr(SUMO_ATTR_TIME, &time2string(timestep));
        of.set_precision(precision);
        if !veh.is_on_road() {
            return;
        }
        // shortcut: getDevice(typeid(MSDevice_ElecHybrid)) takes an unmapped type_info and the result must be downcast to MSDevice_ElecHybrid (no repr(C) layout yet); the battery and vehicle attributes are not written until then
        of.close_tag("");
    }

    /// C++: `void MSElecHybridExport::writeAggregated(OutputDevice &, SUMOTime, int)` [static]
    // void
    // MSElecHybridExport::writeAggregated(OutputDevice& of, SUMOTime timestep, int precision) {
    //     of.openTag(SUMO_TAG_TIMESTEP).writeAttr(SUMO_ATTR_TIME, time2string(timestep));
    //     of.setPrecision(precision);
    //
    //     MSVehicleControl& vc = MSNet::getInstance()->getVehicleControl();
    //     for (MSVehicleControl::constVehIt it = vc.loadedVehBegin(); it != vc.loadedVehEnd(); ++it) {
    //
    //         const SUMOVehicle* veh = it->second;
    //         const MSVehicle* microVeh = dynamic_cast<const MSVehicle*>(veh);
    //
    //         if (!veh->isOnRoad()) {
    //             continue;
    //         }
    //
    //         if (static_cast<MSDevice_ElecHybrid*>(veh->getDevice(typeid(MSDevice_ElecHybrid))) != 0) {
    //             MSDevice_ElecHybrid* elecHybridToExport = dynamic_cast<MSDevice_ElecHybrid*>(veh->getDevice(typeid(MSDevice_ElecHybrid)));
    //             if (elecHybridToExport->getMaximumBatteryCapacity() > 0) {
    //                 // Open Row
    //                 of.openTag(SUMO_TAG_VEHICLE);
    //                 // Write ID
    //                 of.writeAttr(SUMO_ATTR_ID, veh->getID());
    //
    //                 // Write Maximum battery capacity
    //                 of.writeAttr(SUMO_ATTR_MAXIMUMBATTERYCAPACITY, elecHybridToExport->getMaximumBatteryCapacity());
    //                 // Write Actual battery capacity
    //                 of.writeAttr(SUMO_ATTR_ACTUALBATTERYCAPACITY, elecHybridToExport->getActualBatteryCapacity());
    //
    //                 // Write consumed energy [Wh] (computed by HelpersEnergy::compute)
    //                 of.writeAttr(SUMO_ATTR_ENERGYCONSUMED, elecHybridToExport->getConsum());
    //                 // Write Energy charged in the Battery [Wh] (drawn energy from overhead wire minus consumed energy)
    //                 of.writeAttr(SUMO_ATTR_ENERGYCHARGED, elecHybridToExport->getEnergyCharged());
    //                 // Write Power demand (requsted from overhed wire) [W]
    //                 of.writeAttr(SUMO_ATTR_CHARGINGPOWER, elecHybridToExport->getPowerWanted());
    //
    //                 // Write OverheadWire Segment ID
    //                 of.writeAttr(SUMO_ATTR_OVERHEADWIREID, elecHybridToExport->getOverheadWireSegmentID());
    //                 // Write Traction Substation ID
    //                 of.writeAttr(SUMO_ATTR_TRACTIONSUBSTATIONID, elecHybridToExport->getTractionSubstationID());
    //
    //                 // Write current from overheadwire
    //                 of.writeAttr(SUMO_ATTR_CURRENTFROMOVERHEADWIRE, elecHybridToExport->getCurrentFromOverheadWire());
    //                 // Write voltage of overheadwire
    //                 of.writeAttr(SUMO_ATTR_VOLTAGEOFOVERHEADWIRE, elecHybridToExport->getVoltageOfOverheadWire());
    //                 // Write circuit alpha best (1 if the traction substation is not overloaded, number from interval [0,1) if the traction substation is overloaded, NAN if it is not applicable)
    //                 of.writeAttr(SUMO_ATTR_ALPHACIRCUITSOLVER, elecHybridToExport->getCircuitAlpha());
    //
    //                 // Write Speed
    //                 of.writeAttr(SUMO_ATTR_SPEED, veh->getSpeed());
    //                 // Write Acceleration
    //                 of.writeAttr(SUMO_ATTR_ACCELERATION, veh->getAcceleration());
    //                 // Write Distance
    //                 of.writeAttr(SUMO_ATTR_DISTANCE, veh->getOdometer());
    //                 // Write pos x
    //                 of.writeAttr(SUMO_ATTR_X, veh->getPosition().x());
    //                 // Write pos y
    //                 of.writeAttr(SUMO_ATTR_Y, veh->getPosition().y());
    //                 // Write pos z
    //                 of.writeAttr(SUMO_ATTR_Z, veh->getPosition().z());
    //                 // Write slope
    //                 of.writeAttr(SUMO_ATTR_SLOPE, veh->getSlope());
    //                 if (microVeh != nullptr) {
    //                     // Write Lane ID
    //                     of.writeAttr(SUMO_ATTR_LANE, microVeh->getLane()->getID());
    //                 }
    //                 // Write vehicle position in the lane
    //                 of.writeAttr(SUMO_ATTR_POSONLANE, veh->getPositionOnLane());
    //                 // Close Row
    //                 of.closeTag();
    //             }
    //         }
    //     }
    //     of.closeTag();
    // }
    #[allow(unused_variables)]
    pub fn write_aggregated(of: &mut OutputDevice, timestep: SUMOTime, precision: i32) {
        of.open_tag_id(SUMO_TAG_TIMESTEP).write_attr(SUMO_ATTR_TIME, &time2string(timestep));
        of.set_precision(precision);
        // shortcut: the vehicle rows need the getDevice(typeid(MSDevice_ElecHybrid)) lookup (unmapped type_info) and MSVehicle attributes; no rows are written until those exist
        of.close_tag("");
    }

    /// C++: `void MSElecHybridExport::MSElecHybridExport(const MSElecHybridExport &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new(arg0: &MSElecHybridExport) -> Self {
        panic!("MSElecHybridExport copy constructor is invalidated (private, undefined in C++)")
    }

    /// C++: `MSElecHybridExport & MSElecHybridExport::operator=(const MSElecHybridExport &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSElecHybridExport) -> &mut MSElecHybridExport {
        panic!("MSElecHybridExport::operator= is invalidated (private, undefined in C++)")
    }

}

