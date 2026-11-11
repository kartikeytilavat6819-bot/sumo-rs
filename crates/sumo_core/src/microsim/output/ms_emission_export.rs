//! Port of microsim/output/MSEmissionExport.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// Realises dumping Emission Data
/// C++ class `MSEmissionExport` (microsim/output/MSEmissionExport.h)
pub struct MSEmissionExport {
}

impl MSEmissionExport {
    /// C++: `void MSEmissionExport::write(OutputDevice &, SUMOTime)` [static]
    /// Writes emission values into the given device
    // void
    // MSEmissionExport::write(OutputDevice& of, SUMOTime timestep) {
    //     MSDevice_Emissions::initOnce();
    //     const OptionsCont& oc = OptionsCont::getOptions();
    //     const SUMOTime period = string2time(oc.getString("device.emissions.period"));
    //     const SUMOTime begin = string2time(oc.getString("device.emissions.begin"));
    //     const bool useGeo = oc.getBool("emission-output.geo");
    //     if ((period > 0 && (timestep - begin) % period != 0) || timestep < begin) {
    //         return;
    //     }
    //     const SumoXMLAttrMask mask = MSDevice_Emissions::getWrittenAttributes();
    //
    //     of.openTag("timestep").writeAttr("time", time2string(timestep));
    //     MSVehicleControl& vc = MSNet::getInstance()->getVehicleControl();
    //     for (MSVehicleControl::constVehIt it = vc.loadedVehBegin(); it != vc.loadedVehEnd(); ++it) {
    //         const SUMOVehicle* veh = it->second;
    //         MSDevice_Emissions* emissionsDevice = (MSDevice_Emissions*)veh->getDevice(typeid(MSDevice_Emissions));
    //         if (emissionsDevice != nullptr && (veh->isOnRoad() || veh->isIdling())) {
    //             of.openTag("vehicle");
    //             of.writeAttr(SUMO_ATTR_ID, veh->getID());
    //             writeEmissions(of, static_cast<const MSBaseVehicle*>(veh), true, mask);
    //             if (MSGlobals::gUseMesoSim) {
    //                 of.writeOptionalAttr(SUMO_ATTR_EDGE, veh->getEdge()->getID(), mask);
    //             } else {
    //                 of.writeOptionalAttr(SUMO_ATTR_LANE, veh->getLane()->getID(), mask);
    //             }
    //             of.writeOptionalAttr(SUMO_ATTR_POSITION, veh->getPositionOnLane(), mask);
    //             of.writeOptionalAttr(SUMO_ATTR_SPEED, veh->getSpeed(), mask);
    //             of.writeOptionalAttr(SUMO_ATTR_ANGLE, GeomHelper::naviDegree(veh->getAngle()), mask);
    //
    //             Position pos = veh->getPosition();
    //             if (useGeo) {
    //                 of.setPrecision(gPrecisionGeo);
    //                 GeoConvHelper::getFinal().cartesian2geo(pos);
    //             }
    //             of.writeOptionalAttr(SUMO_ATTR_X, pos.x(), mask);
    //             of.writeOptionalAttr(SUMO_ATTR_Y, pos.y(), mask);
    //             of.setPrecision(gPrecision);
    //             if (MSNet::getInstance()->hasElevation()) {
    //                 of.writeOptionalAttr(SUMO_ATTR_Z, pos.z(), mask);
    //             }
    //             of.closeTag();
    //         }
    //     }
    //     of.closeTag();
    // }
    #[allow(unused_variables)]
    pub fn write(of: &mut OutputDevice, timestep: SUMOTime) {
        todo!("port MSEmissionExport::write")
    }

    /// C++: `void MSEmissionExport::writeEmissions(OutputDevice &, const MSBaseVehicle *const, const bool, const SumoXMLAttrMask &)` [static]
    /// Writes emission values for a single vehicle into the given device
    // void
    // MSEmissionExport::writeEmissions(OutputDevice& of, const MSBaseVehicle* const veh, const bool includeType, const SumoXMLAttrMask& mask) {
    //     of.writeFuncAttr(SUMO_ATTR_ECLASS, [ = ]() {
    //         return  PollutantsInterface::getName(veh->getVehicleType().getEmissionClass());
    //     }, mask);
    //     if (mask.test(SUMO_ATTR_CO2) || mask.test(SUMO_ATTR_CO) || mask.test(SUMO_ATTR_HC) || mask.test(SUMO_ATTR_NOX) || mask.test(SUMO_ATTR_PMX)
    //             || mask.test(SUMO_ATTR_FUEL) || mask.test(SUMO_ATTR_ELECTRICITY)) {
    //         PollutantsInterface::Emissions emiss = PollutantsInterface::computeAll(
    //                 veh->getVehicleType().getEmissionClass(),
    //                 veh->getSpeed(), veh->getAcceleration(), veh->getSlope(),
    //                 veh->getEmissionParameters());
    //         if (OptionsCont::getOptions().getBool("emission-output.step-scaled")) {
    //             emiss.addScaled(emiss, TS - 1.);
    //         }
    //         of.setPrecision(gPrecisionEmissions);
    //         of.writeOptionalAttr(SUMO_ATTR_CO2, emiss.CO2, mask);
    //         of.writeOptionalAttr(SUMO_ATTR_CO, emiss.CO, mask);
    //         of.writeOptionalAttr(SUMO_ATTR_HC, emiss.HC, mask);
    //         of.writeOptionalAttr(SUMO_ATTR_NOX, emiss.NOx, mask);
    //         of.writeOptionalAttr(SUMO_ATTR_PMX, emiss.PMx, mask);
    //         of.writeOptionalAttr(SUMO_ATTR_FUEL, emiss.fuel, mask);
    //         of.writeOptionalAttr(SUMO_ATTR_ELECTRICITY, emiss.electricity, mask);
    //         of.setPrecision(gPrecision);
    //     }
    //     of.writeFuncAttr(SUMO_ATTR_NOISE, [ = ]() {
    //         return HelpersHarmonoise::computeNoise(veh->getVehicleType().getEmissionClass(), veh->getSpeed(), veh->getAcceleration());
    //     }, mask);
    //     of.writeOptionalAttr(SUMO_ATTR_ROUTE, veh->getRoute().getID(), mask);
    //     if (includeType) {
    //         std::string fclass = veh->getVehicleType().getID();
    //         fclass = fclass.substr(0, fclass.find_first_of("@"));
    //         of.writeOptionalAttr(SUMO_ATTR_TYPE, fclass, mask);
    //     }
    //     of.writeFuncAttr(SUMO_ATTR_WAITING, [ = ]() {
    //         return veh->getWaitingSeconds();
    //     }, mask);
    // }
    #[allow(unused_variables)]
    pub fn write_emissions(of: &mut OutputDevice, veh: Ptr<MSBaseVehicle>, includeType: bool, mask: &SumoXMLAttrMask) {
        todo!("port MSEmissionExport::writeEmissions")
    }

    /// C++: `void MSEmissionExport::MSEmissionExport(const MSEmissionExport &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new(arg0: &MSEmissionExport) -> Self {
        todo!("port MSEmissionExport::MSEmissionExport")
    }

    /// C++: `MSEmissionExport & MSEmissionExport::operator=(const MSEmissionExport &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSEmissionExport) -> &mut MSEmissionExport {
        todo!("port MSEmissionExport::operator=")
    }

}

