//! Port of microsim/output/MSEmissionExport.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::std_defs::{g_precision, g_precision_emissions, g_precision_geo};
use utils::common::time::ts;
use utils::options::options_cont;

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
        MSDevice_Emissions::init_once();
        let period = string2time(&options_cont::get_string("device.emissions.period")).unwrap();
        let begin = string2time(&options_cont::get_string("device.emissions.begin")).unwrap();
        let useGeo = options_cont::get_bool("emission-output.geo");
        if (period > 0 && (timestep - begin) % period != 0) || timestep < begin {
            return;
        }
        let mask = MSDevice_Emissions::get_written_attributes();
        of.open_tag("timestep").write_attr("time", &time2string(timestep));
        let net = MSNet::get_instance();
        let hasElevation = net.has_elevation();
        let vc = net.m().get_vehicle_control();
        for veh in vc.myVehicleDict.values() {
            let veh: Ptr<SUMOVehicle> = *veh;
            // no typeid in Rust: "emissions" is the device name only MSDevice_Emissions reports, so this equals getDevice(typeid(MSDevice_Emissions))
            let hasEmissionsDevice = veh.get_devices().iter().any(|d| d.base2.device_name() == "emissions");
            if hasEmissionsDevice && (veh.is_on_road() || veh.is_idling()) {
                of.open_tag("vehicle");
                of.write_attr(SUMO_ATTR_ID, veh.get_id());
                Self::write_emissions(of, veh.cast::<MSBaseVehicle>(), true, &mask);
                if use_meso_sim() {
                    of.write_optional_attr_mask(SUMO_ATTR_EDGE, &veh.get_current_edge().get_id().to_string(), mask, false);
                } else {
                    of.write_optional_attr_mask(SUMO_ATTR_LANE, &veh.cast::<MSVehicle>().get_lane().get_id().to_string(), mask, false);
                }
                of.write_optional_attr_mask(SUMO_ATTR_POSITION, &veh.get_position_on_lane(), mask, false);
                of.write_optional_attr_mask(SUMO_ATTR_SPEED, &veh.get_speed(), mask, false);
                of.write_optional_attr_mask(SUMO_ATTR_ANGLE, &utils::geom::geom_helper::navi_degree(veh.get_angle()), mask, false);

                let mut pos = veh.get_position(0.0);
                if useGeo {
                    of.set_precision(g_precision_geo());
                    GeoConvHelper::with_final(|g| g.cartesian2geo(&mut pos));
                }
                of.write_optional_attr_mask(SUMO_ATTR_X, &pos.x(), mask, false);
                of.write_optional_attr_mask(SUMO_ATTR_Y, &pos.y(), mask, false);
                of.set_precision(g_precision());
                if hasElevation {
                    of.write_optional_attr_mask(SUMO_ATTR_Z, &pos.z(), mask, false);
                }
                of.close_tag("");
            }
        }
        of.close_tag("");
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
        let mask = *mask;
        of.write_func_attr(SUMO_ATTR_ECLASS, || PollutantsInterface::get_name(veh.get_vehicle_type().get_emission_class()), mask, false);
        if mask.test(SUMO_ATTR_CO2) || mask.test(SUMO_ATTR_CO) || mask.test(SUMO_ATTR_HC) || mask.test(SUMO_ATTR_NOX) || mask.test(SUMO_ATTR_PMX)
            || mask.test(SUMO_ATTR_FUEL) || mask.test(SUMO_ATTR_ELECTRICITY) {
            // virtual calls in C++: read through SUMOTrafficObject so MSVehicle's acceleration is used, not the base 0
            let obj: &SUMOTrafficObject = &*veh;
            let mut emiss = PollutantsInterface::compute_all(obj.get_vehicle_type().get_emission_class(), obj.get_speed(), obj.get_acceleration(), obj.get_slope(), veh.m().get_emission_parameters());
            if options_cont::get_bool("emission-output.step-scaled") {
                // C++ addScaled(emiss, TS - 1.) aliases its own operand; the copy reads the pre-scaling values
                let copy = PollutantsInterface_Emissions { CO2: emiss.CO2, CO: emiss.CO, HC: emiss.HC, fuel: emiss.fuel, NOx: emiss.NOx, PMx: emiss.PMx, electricity: emiss.electricity };
                emiss.add_scaled(&copy, ts() - 1.);
            }
            of.set_precision(g_precision_emissions());
            of.write_optional_attr_mask(SUMO_ATTR_CO2, &emiss.CO2, mask, false);
            of.write_optional_attr_mask(SUMO_ATTR_CO, &emiss.CO, mask, false);
            of.write_optional_attr_mask(SUMO_ATTR_HC, &emiss.HC, mask, false);
            of.write_optional_attr_mask(SUMO_ATTR_NOX, &emiss.NOx, mask, false);
            of.write_optional_attr_mask(SUMO_ATTR_PMX, &emiss.PMx, mask, false);
            of.write_optional_attr_mask(SUMO_ATTR_FUEL, &emiss.fuel, mask, false);
            of.write_optional_attr_mask(SUMO_ATTR_ELECTRICITY, &emiss.electricity, mask, false);
            of.set_precision(g_precision());
        }
        let obj: &SUMOTrafficObject = &*veh;
        of.write_func_attr(SUMO_ATTR_NOISE, || HelpersHarmonoise::compute_noise(obj.get_vehicle_type().get_emission_class(), obj.get_speed(), obj.get_acceleration()), mask, false);
        of.write_optional_attr_mask(SUMO_ATTR_ROUTE, veh.get_route().base.get_id(), mask, false);
        if includeType {
            let fclass = veh.get_vehicle_type().get_id().split('@').next().unwrap().to_string();
            of.write_optional_attr_mask(SUMO_ATTR_TYPE, &fclass, mask, false);
        }
        of.write_func_attr(SUMO_ATTR_WAITING, || veh.get_waiting_seconds(), mask, false);
    }

    /// C++: `void MSEmissionExport::MSEmissionExport(const MSEmissionExport &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new(arg0: &MSEmissionExport) -> Self {
        panic!("MSEmissionExport copy constructor is invalidated (private, undefined in C++)")
    }

    /// C++: `MSEmissionExport & MSEmissionExport::operator=(const MSEmissionExport &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSEmissionExport) -> &mut MSEmissionExport {
        panic!("MSEmissionExport::operator= is invalidated (private, undefined in C++)")
    }

}

