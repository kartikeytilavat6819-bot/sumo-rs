//! Port of microsim/output/MSXMLRawOut.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// Realises dumping the complete network state
/// C++ class `MSXMLRawOut` (microsim/output/MSXMLRawOut.h)
pub struct MSXMLRawOut {
}

impl MSXMLRawOut {
    /// C++: `void MSXMLRawOut::write(OutputDevice &, const MSEdgeControl &, SUMOTime, int)` [static]
    /// Writes the complete network state of the given edges into the given device
    // void
    // MSXMLRawOut::write(OutputDevice& of, const MSEdgeControl& ec,
    //                    SUMOTime timestep, int precision) {
    //     of.openTag("timestep") << " time=\"" << time2string(timestep) << "\"";
    //     of.setPrecision(precision);
    //     const MSEdgeVector& edges = ec.getEdges();
    //     for (MSEdgeVector::const_iterator e = edges.begin(); e != edges.end(); ++e) {
    //         writeEdge(of, **e, timestep);
    //     }
    //     of.setPrecision(gPrecision);
    //     of.closeTag();
    // }
    #[allow(unused_variables)]
    pub fn write(of: &mut OutputDevice, ec: &MSEdgeControl, timestep: SUMOTime, precision: i32) {
        todo!("port MSXMLRawOut::write")
    }

    /// C++: `void MSXMLRawOut::writeVehicle(OutputDevice &, const MSBaseVehicle &)` [static]
    /// Writes the dump of the given vehicle into the given device
    // void
    // MSXMLRawOut::writeVehicle(OutputDevice& of, const MSBaseVehicle& veh) {
    //     if (veh.isOnRoad()) {
    //         of.openTag("vehicle");
    //         of.writeAttr(SUMO_ATTR_ID, veh.getID());
    //         of.writeAttr(SUMO_ATTR_POSITION, veh.getPositionOnLane());
    //         of.writeAttr(SUMO_ATTR_SPEED, veh.getSpeed());
    //         // TODO: activate action step length output, if required
    //         //of.writeAttr(SUMO_ATTR_ACTIONSTEPLENGTH, veh.getActionStepLength());
    //         if (!MSGlobals::gUseMesoSim) {
    //             const MSVehicle& microVeh = static_cast<const MSVehicle&>(veh);
    //             // microsim-specific stuff
    //             if (MSGlobals::gSublane) {
    //                 const double posLat = microVeh.getLateralPositionOnLane();
    //                 of.writeAttr(SUMO_ATTR_POSITION_LAT, posLat);
    //                 of.writeAttr(SUMO_ATTR_SPEED_LAT, microVeh.getLaneChangeModel().getSpeedLat());
    //             }
    //             const int personNumber = microVeh.getPersonNumber();
    //             if (personNumber > 0) {
    //                 of.writeAttr(SUMO_ATTR_PERSON_NUMBER, personNumber);
    //             }
    //             const int containerNumber = microVeh.getContainerNumber();
    //             if (containerNumber > 0) {
    //                 of.writeAttr(SUMO_ATTR_CONTAINER_NUMBER, containerNumber);
    //             }
    //             const std::vector<MSTransportable*>& persons = microVeh.getPersons();
    //             for (std::vector<MSTransportable*>::const_iterator it_p = persons.begin(); it_p != persons.end(); ++it_p) {
    //                 writeTransportable(of, *it_p, SUMO_TAG_PERSON);
    //             }
    //             const std::vector<MSTransportable*>& containers = microVeh.getContainers();
    //             for (std::vector<MSTransportable*>::const_iterator it_c = containers.begin(); it_c != containers.end(); ++it_c) {
    //                 writeTransportable(of, *it_c, SUMO_TAG_CONTAINER);
    //             }
    //         }
    //         of.closeTag();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn write_vehicle(of: &mut OutputDevice, veh: &MSBaseVehicle) {
        todo!("port MSXMLRawOut::writeVehicle")
    }

    /// C++: `void MSXMLRawOut::writeEdge(OutputDevice &, const MSEdge &, SUMOTime)` [static, private]
    /// Writes the dump of the given edge into the given device
    // void
    // MSXMLRawOut::writeEdge(OutputDevice& of, const MSEdge& edge, SUMOTime timestep) {
    //     if (!MSGlobals::gUsingInternalLanes && !edge.isNormal()) {
    //         return;
    //     }
    //     //en
    //     bool dump = !MSGlobals::gOmitEmptyEdgesOnDump;
    //     if (!dump) {
    //         if (MSGlobals::gUseMesoSim) {
    //             MESegment* seg = MSGlobals::gMesoNet->getSegmentForEdge(edge);
    //             while (seg != nullptr) {
    //                 if (seg->getCarNumber() != 0) {
    //                     dump = true;
    //                     break;
    //                 }
    //                 seg = seg->getNextSegment();
    //             }
    //         } else {
    //             const std::vector<MSLane*>& lanes = edge.getLanes();
    //             for (std::vector<MSLane*>::const_iterator lane = lanes.begin(); lane != lanes.end(); ++lane) {
    //                 if (((**lane).getVehicleNumber() != 0)) {
    //                     dump = true;
    //                     break;
    //                 }
    //             }
    //         }
    //     }
    //     //en
    //     const std::vector<MSTransportable*>& persons = edge.getSortedPersons(timestep);
    //     const std::vector<MSTransportable*>& containers = edge.getSortedContainers(timestep);
    //     if (dump || persons.size() > 0 || containers.size() > 0) {
    //         of.openTag("edge") << " id=\"" << edge.getID() << "\"";
    //         if (dump) {
    //             if (MSGlobals::gUseMesoSim) {
    //                 MESegment* seg = MSGlobals::gMesoNet->getSegmentForEdge(edge);
    //                 while (seg != nullptr) {
    //                     seg->writeVehicles(of);
    //                     seg = seg->getNextSegment();
    //                 }
    //             } else {
    //                 const std::vector<MSLane*>& lanes = edge.getLanes();
    //                 for (std::vector<MSLane*>::const_iterator lane = lanes.begin(); lane != lanes.end(); ++lane) {
    //                     writeLane(of, **lane);
    //                 }
    //             }
    //         }
    //         // write persons
    //         for (std::vector<MSTransportable*>::const_iterator it_p = persons.begin(); it_p != persons.end(); ++it_p) {
    //             writeTransportable(of, *it_p, SUMO_TAG_PERSON);
    //         }
    //         // write containers
    //         for (std::vector<MSTransportable*>::const_iterator it_c = containers.begin(); it_c != containers.end(); ++it_c) {
    //             writeTransportable(of, *it_c, SUMO_TAG_CONTAINER);
    //         }
    //         of.closeTag();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn write_edge(of: &mut OutputDevice, edge: &MSEdge, timestep: SUMOTime) {
        todo!("port MSXMLRawOut::writeEdge")
    }

    /// C++: `void MSXMLRawOut::writeLane(OutputDevice &, const MSLane &)` [static, private]
    /// Writes the dump of the given lane into the given device
    // void
    // MSXMLRawOut::writeLane(OutputDevice& of, const MSLane& lane) {
    //     of.openTag("lane").writeAttr(SUMO_ATTR_ID, lane.getID());
    //     for (const MSBaseVehicle* const veh : lane.getVehiclesSecure()) {
    //         writeVehicle(of, *veh);
    //     }
    //     lane.releaseVehicles();
    //     of.closeTag();
    // }
    #[allow(unused_variables)]
    pub fn write_lane(of: &mut OutputDevice, lane: &MSLane) {
        todo!("port MSXMLRawOut::writeLane")
    }

    /// C++: `void MSXMLRawOut::writeTransportable(OutputDevice &, const MSTransportable *, SumoXMLTag)` [static, private]
    /// write transportable
    // void
    // MSXMLRawOut::writeTransportable(OutputDevice& of, const MSTransportable* p, SumoXMLTag tag) {
    //     of.openTag(tag);
    //     of.writeAttr(SUMO_ATTR_ID, p->getID());
    //     of.writeAttr(SUMO_ATTR_POSITION, p->getEdgePos());
    //     of.writeAttr(SUMO_ATTR_ANGLE, GeomHelper::naviDegree(p->getAngle()));
    //     of.writeAttr(SUMO_ATTR_STAGE, p->getCurrentStageDescription());
    //     of.closeTag();
    // }
    #[allow(unused_variables)]
    pub fn write_transportable(of: &mut OutputDevice, p: Ptr<MSTransportable>, tag: SumoXMLTag) {
        todo!("port MSXMLRawOut::writeTransportable")
    }

    /// C++: `void MSXMLRawOut::MSXMLRawOut(const MSXMLRawOut &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new(arg0: &MSXMLRawOut) -> Self {
        todo!("port MSXMLRawOut::MSXMLRawOut")
    }

    /// C++: `MSXMLRawOut & MSXMLRawOut::operator=(const MSXMLRawOut &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSXMLRawOut) -> &mut MSXMLRawOut {
        todo!("port MSXMLRawOut::operator=")
    }

}

