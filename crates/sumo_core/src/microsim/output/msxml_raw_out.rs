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
        of.open_tag("timestep").write(" time=\"").write(&time2string(timestep)).write("\"");
        of.set_precision(precision);
        let edges = ec.get_edges();
        for e in edges.iter() {
            Self::write_edge(of, e, timestep);
        }
        of.set_precision(g_precision());
        of.close_tag("");
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
        if veh.is_on_road() {
            of.open_tag("vehicle");
            of.write_attr(SUMO_ATTR_ID, veh.get_id());
            of.write_attr(SUMO_ATTR_POSITION, &veh.get_position_on_lane());
            of.write_attr(SUMO_ATTR_SPEED, &veh.get_speed());
            // TODO: activate action step length output, if required
            //of.writeAttr(SUMO_ATTR_ACTIONSTEPLENGTH, veh.getActionStepLength());
            if !use_meso_sim() {
                // static_cast<const MSVehicle&>(veh): caller guarantees a microsim vehicle when !gUseMesoSim
                let microVeh: Ptr<MSVehicle> = Ptr::new(veh).cast::<MSVehicle>();
                // microsim-specific stuff
                if sublane() {
                    let posLat = microVeh.get_lateral_position_on_lane();
                    of.write_attr(SUMO_ATTR_POSITION_LAT, &posLat);
                    of.write_attr(SUMO_ATTR_SPEED_LAT, &microVeh.get_lane_change_model_2().get_speed_lat());
                }
                let personNumber = microVeh.get_person_number();
                if personNumber > 0 {
                    of.write_attr(SUMO_ATTR_PERSON_NUMBER, &personNumber);
                }
                let containerNumber = microVeh.get_container_number();
                if containerNumber > 0 {
                    of.write_attr(SUMO_ATTR_CONTAINER_NUMBER, &containerNumber);
                }
                let persons = microVeh.get_persons();
                for it_p in persons.iter() {
                    Self::write_transportable(of, *it_p, SUMO_TAG_PERSON);
                }
                let containers = microVeh.get_containers();
                for it_c in containers.iter() {
                    Self::write_transportable(of, *it_c, SUMO_TAG_CONTAINER);
                }
            }
            of.close_tag("");
        }
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
        if !using_internal_lanes() && !edge.is_normal() {
            return;
        }
        //en
        let mut dump = !omit_empty_edges_on_dump();
        if !dump {
            if use_meso_sim() {
                let mut seg = meso_net().m().get_segment_for_edge(edge, 0.0);
                while !seg.is_null() {
                    if seg.get_car_number() != 0 {
                        dump = true;
                        break;
                    }
                    seg = seg.get_next_segment();
                }
            } else {
                let lanes = edge.get_lanes();
                for lane in lanes.iter() {
                    if lane.get_vehicle_number() != 0 {
                        dump = true;
                        break;
                    }
                }
            }
        }
        //en
        let persons = edge.get_sorted_persons(timestep, false);
        let containers = edge.get_sorted_containers(timestep, false);
        if dump || persons.len() > 0 || containers.len() > 0 {
            of.open_tag("edge").write(" id=\"").write(edge.get_id()).write("\"");
            if dump {
                if use_meso_sim() {
                    let mut seg = meso_net().m().get_segment_for_edge(edge, 0.0);
                    while !seg.is_null() {
                        seg.write_vehicles(of);
                        seg = seg.get_next_segment();
                    }
                } else {
                    let lanes = edge.get_lanes();
                    for lane in lanes.iter() {
                        Self::write_lane(of, lane);
                    }
                }
            }
            // write persons
            for it_p in persons.iter() {
                Self::write_transportable(of, *it_p, SUMO_TAG_PERSON);
            }
            // write containers
            for it_c in containers.iter() {
                Self::write_transportable(of, *it_c, SUMO_TAG_CONTAINER);
            }
            of.close_tag("");
        }
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
        of.open_tag("lane").write_attr(SUMO_ATTR_ID, lane.get_id());
        for veh in lane.get_vehicles_secure().iter() {
            Self::write_vehicle(of, veh);
        }
        lane.release_vehicles();
        of.close_tag("");
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
        of.open_tag_id(tag);
        of.write_attr(SUMO_ATTR_ID, p.get_id());
        of.write_attr(SUMO_ATTR_POSITION, &p.get_edge_pos());
        of.write_attr(SUMO_ATTR_ANGLE, &utils::geom::geom_helper::navi_degree(p.get_angle()));
        of.write_attr(SUMO_ATTR_STAGE, &p.get_current_stage_description());
        of.close_tag("");
    }

    /// C++: `void MSXMLRawOut::MSXMLRawOut(const MSXMLRawOut &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new(arg0: &MSXMLRawOut) -> Self {
        panic!("MSXMLRawOut copy constructor is private and undefined in C++")
    }

    /// C++: `MSXMLRawOut & MSXMLRawOut::operator=(const MSXMLRawOut &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSXMLRawOut) -> &mut MSXMLRawOut {
        panic!("MSXMLRawOut assignment operator is private and undefined in C++")
    }

}

