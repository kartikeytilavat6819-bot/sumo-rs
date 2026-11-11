//! Port of microsim/output/MSFullExport.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

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
        todo!("port MSFullExport::write")
    }

    /// C++: `void MSFullExport::MSFullExport(const MSFullExport &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new(arg0: &MSFullExport) -> Self {
        todo!("port MSFullExport::MSFullExport")
    }

    /// C++: `MSFullExport & MSFullExport::operator=(const MSFullExport &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSFullExport) -> &mut MSFullExport {
        todo!("port MSFullExport::operator=")
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
        todo!("port MSFullExport::writeVehicles")
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
        todo!("port MSFullExport::writePersons")
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
        todo!("port MSFullExport::writeEdge")
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
        todo!("port MSFullExport::writeLane")
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
        todo!("port MSFullExport::writeTLS")
    }

}

