//! Port of microsim/output/MSFullExport.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::geom::geom_helper::navi_degree;
use utils::geom::NUMERICAL_EPS;


/// C++ `MSLane::getEmissions<ET>()` template body (MSLane.h): sums the per-vehicle emissions of the lane.
/// The ms_lane.rs stub has no template parameter, so the sum is computed here from the same pieces.
pub(crate) fn lane_emissions(lane: &MSLane, et: PollutantsInterface_EmissionType) -> f64 {
    let mut ret = 0.;
    for v in lane.get_vehicles_secure() {
        // C++ MSBaseVehicle::getEmissions<ET>()
        if v.is_on_road() || v.is_idling() {
            ret += PollutantsInterface::compute(v.get_vehicle_type().get_emission_class(), et, v.get_speed(), v.get_acceleration(), v.get_slope(), v.m().get_emission_parameters());
        }
    }
    lane.release_vehicles();
    ret
}

/// Dumping a hugh List of Parameters available in the Simulation
/// C++ class `MSFullExport` (microsim/output/MSFullExport.h)
pub struct MSFullExport {
}

impl MSFullExport {
    /// C++: `void MSFullExport::write(OutputDevice &, SUMOTime)` [static]
    /// Dumping a hugh List of Parameters available in the Simulation
    // void
    // MSFullExport::write(OutputDevice& of, SUMOTime timestep) {
    //     of.openTag("data") << " timestep=\"" << time2string(timestep) << "\"";
    //     writeVehicles(of);
    //     if (MSNet::getInstance()->hasPersons()) {
    //         writePersons(of);
    //     }
    //     writeEdge(of);
    //     writeTLS(of, timestep);
    //     of.closeTag();
    // }
    #[allow(unused_variables)]
    pub fn write(of: &mut OutputDevice, timestep: SUMOTime) {
        of.open_tag("data").write(" timestep=\"").write(&time2string(timestep)).write("\"");
        MSFullExport::write_vehicles(of);
        if MSNet::get_instance().has_persons() {
            MSFullExport::write_persons(of);
        }
        MSFullExport::write_edge(of);
        MSFullExport::write_tls(of, timestep);
        of.close_tag("");
    }

    /// C++: `void MSFullExport::MSFullExport(const MSFullExport &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new(arg0: &MSFullExport) -> Self {
        panic!("MSFullExport copy constructor is invalidated (private, undefined in C++)")
    }

    /// C++: `MSFullExport & MSFullExport::operator=(const MSFullExport &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSFullExport) -> &mut MSFullExport {
        panic!("MSFullExport::operator= is invalidated (private, undefined in C++)")
    }

    /// C++: `void MSFullExport::writeVehicles(OutputDevice &)` [static, private]
    /// Writes the XML Nodes for the vehicles (e.g. speed, position, emissions)
    // void
    // MSFullExport::writeVehicles(OutputDevice& of) {
    //     of.openTag("vehicles");
    //     MSVehicleControl& vc = MSNet::getInstance()->getVehicleControl();
    //     const bool hasEle = MSNet::getInstance()->hasElevation();
    //     for (MSVehicleControl::constVehIt it = vc.loadedVehBegin(); it != vc.loadedVehEnd(); ++it) {
    //         const SUMOVehicle* veh = it->second;
    //         const MSVehicle* microVeh = dynamic_cast<const MSVehicle*>(veh);
    //         if (veh->isOnRoad()) {
    //             std::string fclass = veh->getVehicleType().getID();
    //             fclass = fclass.substr(0, fclass.find_first_of("@"));
    //             PollutantsInterface::Emissions emiss = PollutantsInterface::computeAll(
    //                     veh->getVehicleType().getEmissionClass(), veh->getSpeed(),
    //                     veh->getAcceleration(), veh->getSlope(),
    //                     veh->getEmissionParameters());
    //             of.openTag(SUMO_TAG_VEHICLE);
    //             of.writeAttr(SUMO_ATTR_ID, veh->getID());
    //             of.writeAttr(SUMO_ATTR_ECLASS, PollutantsInterface::getName(veh->getVehicleType().getEmissionClass()));
    //             of.writeAttr(SUMO_ATTR_CO2, emiss.CO2);
    //             of.writeAttr(SUMO_ATTR_CO, emiss.CO);
    //             of.writeAttr(SUMO_ATTR_HC, emiss.HC);
    //             of.writeAttr(SUMO_ATTR_NOX, emiss.NOx);
    //             of.writeAttr(SUMO_ATTR_PMX, emiss.PMx);
    //             of.writeAttr(SUMO_ATTR_FUEL, emiss.fuel);
    //             of.writeAttr(SUMO_ATTR_ELECTRICITY, emiss.electricity);
    //             of.writeAttr(SUMO_ATTR_NOISE, HelpersHarmonoise::computeNoise(veh->getVehicleType().getEmissionClass(), veh->getSpeed(), veh->getAcceleration()));
    //             of.writeAttr(SUMO_ATTR_ROUTE, veh->getRoute().getID());
    //             of.writeAttr(SUMO_ATTR_TYPE, fclass);
    //             if (microVeh != nullptr) {
    //                 of.writeAttr(SUMO_ATTR_WAITING, microVeh->getWaitingSeconds());
    //                 of.writeAttr(SUMO_ATTR_LANE, microVeh->getLane()->getID());
    //             }
    //             of.writeAttr(SUMO_ATTR_POSITION, veh->getPositionOnLane());
    //             of.writeAttr(SUMO_ATTR_SPEED, veh->getSpeed());
    //             of.writeAttr(SUMO_ATTR_ANGLE, GeomHelper::naviDegree(veh->getAngle()));
    //             const Position pos = veh->getPosition();
    //             of.writeAttr(SUMO_ATTR_X, pos.x());
    //             of.writeAttr(SUMO_ATTR_Y, pos.y());
    //             if (hasEle) {
    //                 of.writeAttr(SUMO_ATTR_Z, pos.z());
    //                 of.writeAttr(SUMO_ATTR_SLOPE, veh->getSlope());
    //             }
    //             of.closeTag();
    //         }
    //     }
    //     of.closeTag();
    // }
    #[allow(unused_variables)]
    pub fn write_vehicles(of: &mut OutputDevice) {
        of.open_tag("vehicles");
        let net = MSNet::get_instance();
        let vc = net.m().get_vehicle_control();
        let has_ele = net.has_elevation();
        for (_, v) in vc.myVehicleDict.iter() {
            if v.is_on_road() {
                // dynamic_cast<const MSVehicle*>: only microsim vehicles have a waiting time and a lane
                let micro_veh: Ptr<MSVehicle> = if MSGlobals::use_meso_sim() { Ptr::null() } else { v.cast::<MSVehicle>() };
                let vtype_id = v.get_vehicle_type().get_id();
                let fclass = &vtype_id[..vtype_id.find('@').unwrap_or(vtype_id.len())];
                let eclass = v.get_vehicle_type().get_emission_class();
                let emiss = PollutantsInterface::compute_all(eclass, v.get_speed(), v.get_acceleration(), v.get_slope(), v.get_emission_parameters());
                of.open_tag("vehicle");
                of.write_attr("id", v.get_id());
                of.write_attr("eclass", &PollutantsInterface::get_name(eclass));
                of.write_attr("CO2", &emiss.CO2);
                of.write_attr("CO", &emiss.CO);
                of.write_attr("HC", &emiss.HC);
                of.write_attr("NOx", &emiss.NOx);
                of.write_attr("PMx", &emiss.PMx);
                of.write_attr("fuel", &emiss.fuel);
                of.write_attr("electricity", &emiss.electricity);
                of.write_attr("noise", &HelpersHarmonoise::compute_noise(eclass, v.get_speed(), v.get_acceleration()));
                of.write_attr("route", v.get_route().get_id());
                of.write_attr("type", fclass);
                if !micro_veh.is_null() {
                    of.write_attr("waiting", &micro_veh.get_waiting_seconds());
                    of.write_attr("lane", micro_veh.get_lane().get_id());
                }
                of.write_attr("pos", &v.get_position_on_lane());
                of.write_attr("speed", &v.get_speed());
                of.write_attr("angle", &navi_degree(v.get_angle()));
                let pos = v.get_position(0.);
                of.write_attr("x", &pos.x());
                of.write_attr("y", &pos.y());
                if has_ele {
                    of.write_attr("z", &pos.z());
                    of.write_attr("slope", &v.get_slope());
                }
                of.close_tag("");
            }
        }
        of.close_tag("");
    }

    /// C++: `void MSFullExport::writePersons(OutputDevice &)` [static, private]
    /// Writes the XML Nodes for the persons (e.g. speed, position)
    // void
    // MSFullExport::writePersons(OutputDevice& of) {
    //     MSTransportableControl& tc = MSNet::getInstance()->getPersonControl();
    //     const bool hasEle = MSNet::getInstance()->hasElevation();
    //     of.openTag("persons");
    //     for (auto it = tc.loadedBegin(); it != tc.loadedEnd(); ++it) {
    //         const MSTransportable* p = it->second;
    //         if (p->getCurrentStageType() != MSStageType::WAITING_FOR_DEPART) {
    //             const MSEdge* e = p->getEdge();
    //             const SUMOVehicle* v = p->getVehicle();
    //             Position pos = p->getPosition();
    //             of.openTag(SUMO_TAG_PERSON);
    //             of.writeAttr(SUMO_ATTR_ID, p->getID());
    //             of.writeAttr(SUMO_ATTR_X, pos.x());
    //             of.writeAttr(SUMO_ATTR_Y, pos.y());
    //             if (hasEle) {
    //                 of.writeAttr("z", pos.z());
    //             }
    //             of.writeAttr(SUMO_ATTR_ANGLE, GeomHelper::naviDegree(p->getAngle()));
    //             of.writeAttr(SUMO_ATTR_SPEED, p->getSpeed());
    //             of.writeAttr(SUMO_ATTR_POSITION, p->getEdgePos());
    //             of.writeAttr(SUMO_ATTR_EDGE, e->getID());
    //             of.writeAttr(SUMO_ATTR_SLOPE, e->getLanes()[0]->getShape().slopeDegreeAtOffset(p->getEdgePos()));
    //             of.writeAttr(SUMO_ATTR_VEHICLE, v == nullptr ? "" : v->getID());
    //             of.writeAttr(SUMO_ATTR_TYPE, p->getVehicleType().getID());
    //             of.writeAttr("stage", (int)p->getCurrentStageType());
    //             of.closeTag();
    //         }
    //     }
    //     of.closeTag();
    // }
    #[allow(unused_variables)]
    pub fn write_persons(of: &mut OutputDevice) {
        let net = MSNet::get_instance();
        let has_ele = net.has_elevation();
        let tc = net.m().get_person_control();
        of.open_tag("persons");
        for (id, p) in tc.myTransportables.iter() {
            if p.get_current_stage_type() != MSStageType::WAITING_FOR_DEPART {
                let e = p.get_edge();
                let v = p.get_vehicle();
                let pos = p.get_position_2();
                of.open_tag("person");
                of.write_attr("id", id.as_str());
                of.write_attr("x", &pos.x());
                of.write_attr("y", &pos.y());
                if has_ele {
                    of.write_attr("z", &pos.z());
                }
                of.write_attr("angle", &navi_degree(p.get_angle()));
                of.write_attr("speed", &p.get_speed());
                of.write_attr("position", &p.get_edge_pos());
                of.write_attr("edge", e.get_id());
                of.write_attr("slope", &e.get_lanes()[0].get_shape_2().slope_degree_at_offset(p.get_edge_pos()));
                if v.is_null() {
                    of.write_attr("vehicle", "");
                } else {
                    of.write_attr("vehicle", &v.get_id());
                }
                of.write_attr("type", p.get_vehicle_type().get_id());
                of.write_attr("stage", &(p.get_current_stage_type() as i64));
                of.close_tag("");
            }
        }
        of.close_tag("");
    }

    /// C++: `void MSFullExport::writeEdge(OutputDevice &)` [static, private]
    /// Writes the XML Nodes for the edges (e.g. traveltime)
    // void
    // MSFullExport::writeEdge(OutputDevice& of) {
    //     of.openTag("edges");
    //     MSEdgeControl& ec = MSNet::getInstance()->getEdgeControl();
    //     const MSEdgeVector& edges = ec.getEdges();
    //     for (MSEdgeVector::const_iterator e = edges.begin(); e != edges.end(); ++e) {
    //         MSEdge& edge = **e;
    //         if (!MSGlobals::gUsingInternalLanes && !edge.isNormal()) {
    //             continue;
    //         }
    //         of.openTag("edge").writeAttr("id", edge.getID()).writeAttr("traveltime", edge.getCurrentTravelTime());
    //         const std::vector<MSLane*>& lanes = edge.getLanes();
    //         for (std::vector<MSLane*>::const_iterator lane = lanes.begin(); lane != lanes.end(); ++lane) {
    //             writeLane(of, **lane);
    //         }
    //         of.closeTag();
    //     }
    //     of.closeTag();
    // }
    #[allow(unused_variables)]
    pub fn write_edge(of: &mut OutputDevice) {
        of.open_tag("edges");
        let net = MSNet::get_instance();
        let ec = net.m().get_edge_control();
        let edges = ec.get_edges();
        for edge in edges.iter() {
            if !MSGlobals::using_internal_lanes() && !edge.is_normal() {
                continue;
            }
            of.open_tag("edge").write_attr("id", edge.get_id()).write_attr("traveltime", &edge.get_current_travel_time(NUMERICAL_EPS));
            for lane in edge.get_lanes().iter() {
                MSFullExport::write_lane(of, lane);
            }
            of.close_tag("");
        }
        of.close_tag("");
    }

    /// C++: `void MSFullExport::writeLane(OutputDevice &, const MSLane &)` [static, private]
    /// Writes the XML Nodes for the lanes (e.g. emissions, occupancy)
    // void
    // MSFullExport::writeLane(OutputDevice& of, const MSLane& lane) {
    //     of.openTag("lane").writeAttr("id", lane.getID()).writeAttr("CO", lane.getEmissions<PollutantsInterface::CO>()).writeAttr("CO2", lane.getEmissions<PollutantsInterface::CO2>());
    //     of.writeAttr("NOx", lane.getEmissions<PollutantsInterface::NO_X>()).writeAttr("PMx", lane.getEmissions<PollutantsInterface::PM_X>()).writeAttr("HC", lane.getEmissions<PollutantsInterface::HC>());
    //     of.writeAttr("noise", lane.getHarmonoise_NoiseEmissions()).writeAttr("fuel", lane.getEmissions<PollutantsInterface::FUEL>());
    //     of.writeAttr("electricity", lane.getEmissions<PollutantsInterface::ELEC>()).writeAttr("maxspeed", lane.getSpeedLimit());
    //     of.writeAttr("meanspeed", lane.getMeanSpeed()).writeAttr("occupancy", lane.getNettoOccupancy()).writeAttr("vehicle_count", lane.getVehicleNumber());
    //     of.closeTag();
    // }
    #[allow(unused_variables)]
    pub fn write_lane(of: &mut OutputDevice, lane: &MSLane) {
        of.open_tag("lane").write_attr("id", lane.get_id()).write_attr("CO", &lane_emissions(lane, PollutantsInterface_EmissionType::CO)).write_attr("CO2", &lane_emissions(lane, PollutantsInterface_EmissionType::CO2));
        of.write_attr("NOx", &lane_emissions(lane, PollutantsInterface_EmissionType::NO_X)).write_attr("PMx", &lane_emissions(lane, PollutantsInterface_EmissionType::PM_X)).write_attr("HC", &lane_emissions(lane, PollutantsInterface_EmissionType::HC));
        of.write_attr("noise", &lane.get_harmonoise_noise_emissions()).write_attr("fuel", &lane_emissions(lane, PollutantsInterface_EmissionType::FUEL));
        of.write_attr("electricity", &lane_emissions(lane, PollutantsInterface_EmissionType::ELEC)).write_attr("maxspeed", &lane.get_speed_limit());
        of.write_attr("meanspeed", &lane.get_mean_speed()).write_attr("occupancy", &lane.get_netto_occupancy()).write_attr("vehicle_count", &lane.get_vehicle_number());
        of.close_tag("");
    }

    /// C++: `void MSFullExport::writeTLS(OutputDevice &, SUMOTime)` [static, private]
    /// Writes the XML Nodes for the traffic lights (e.g. actual state)
    // void
    // MSFullExport::writeTLS(OutputDevice& of, SUMOTime /* timestep */) {
    //     of.openTag("tls");
    //     MSTLLogicControl& vc = MSNet::getInstance()->getTLSControl();
    //     std::vector<std::string> ids = vc.getAllTLIds();
    //     for (std::vector<std::string>::const_iterator id_it = ids.begin(); id_it != ids.end(); ++id_it) {
    //         MSTLLogicControl::TLSLogicVariants& vars = MSNet::getInstance()->getTLSControl().get(*id_it);
    //         const MSTrafficLightLogic::LaneVectorVector& lanes = vars.getActive()->getLaneVectors();
    //
    //         std::vector<std::string> laneIDs;
    //         for (MSTrafficLightLogic::LaneVectorVector::const_iterator i = lanes.begin(); i != lanes.end(); ++i) {
    //             const MSTrafficLightLogic::LaneVector& llanes = (*i);
    //             for (MSTrafficLightLogic::LaneVector::const_iterator j = llanes.begin(); j != llanes.end(); ++j) {
    //                 laneIDs.push_back((*j)->getID());
    //             }
    //         }
    //
    //         std::string lane_output = "";
    //         for (int i1 = 0; i1 < (int)laneIDs.size(); ++i1) {
    //             lane_output += laneIDs[i1] + " ";
    //         }
    //
    //         std::string state = vars.getActive()->getCurrentPhaseDef().getState();
    //         of.openTag("trafficlight").writeAttr("id", *id_it).writeAttr("state", state).closeTag();
    //     }
    //     of.closeTag();
    // }
    #[allow(unused_variables)]
    pub fn write_tls(of: &mut OutputDevice, timestep: SUMOTime) {
        of.open_tag("tls");
        let net = MSNet::get_instance();
        let vc = net.m().get_tls_control();
        let ids = vc.get_all_tl_ids();
        for id in ids.iter() {
            let vars = vc.get(id);
            // C++ also builds laneIDs / lane_output from getLaneVectors(), but never writes them, so they are not computed here
            let state = vars.get_active().get_current_phase_def().get_state().to_string();
            of.open_tag("trafficlight").write_attr("id", id.as_str()).write_attr("state", &state).close_tag("");
        }
        of.close_tag("");
    }

}

