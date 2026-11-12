//! Port of microsim/output/MSFCDExport.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::std_defs::{g_precision, g_precision_geo, INVALID_DOUBLE};

/// C++ `const Named*` key, used for the std::set<const Named*> membership tests
fn as_named<T>(p: Ptr<T>) -> Ptr<Named> {
    Ptr::from_raw(p.as_raw().cast())
}

/// Realises dumping FCD Data
/// C++ class `MSFCDExport` (microsim/output/MSFCDExport.h)
pub struct MSFCDExport {
}

impl MSFCDExport {
    /// C++: `void MSFCDExport::write(OutputDevice &, const SUMOTime, const SumoXMLTag)` [static]
    /// Writes the position and the angle of each vehicle into the given device
    // void
    // MSFCDExport::write(OutputDevice& of, const SUMOTime timestep, const SumoXMLTag tag) {
    //     MSDevice_FCD::initOnce();
    //     const SUMOTime period = MSDevice_FCD::getPeriod();
    //     const SUMOTime begin = MSDevice_FCD::getBegin();
    //     if ((period > 0 && (timestep - begin) % period != 0) || timestep < begin) {
    //         return;
    //     }
    //     const SumoXMLAttrMask& mask = MSDevice_FCD::getWrittenAttributes();
    //     const bool useGeo = MSDevice_FCD::useGeo();
    //     const bool useUTM = MSDevice_FCD::useUTM();
    //     const double maxLeaderDistance = MSDevice_FCD::getMaxLeaderDistance();
    //     const std::vector<std::string>& params = MSDevice_FCD::getParamsToWrite();
    //     MSNet* net = MSNet::getInstance();
    //     MSVehicleControl& vc = net->getVehicleControl();
    //     const double radius = MSDevice_FCD::getRadius();
    //     const bool filter = MSDevice_FCD::getEdgeFilter().size() > 0;
    //     const bool shapeFilter = MSDevice_FCD::hasShapeFilter();
    //     std::set<const Named*> inRadius;
    //     if (radius > 0) {
    //         // collect all vehicles in radius around equipped vehicles
    //         for (MSVehicleControl::constVehIt it = vc.loadedVehBegin(); it != vc.loadedVehEnd(); ++it) {
    //             const SUMOVehicle* veh = it->second;
    //             if (isVisible(veh) && hasOwnOutput(veh, filter, shapeFilter)) {
    //                 PositionVector shape;
    //                 shape.push_back(veh->getPosition());
    //                 libsumo::Helper::collectObjectsInRange(libsumo::CMD_GET_VEHICLE_VARIABLE, shape, radius, inRadius);
    //                 libsumo::Helper::collectObjectsInRange(libsumo::CMD_GET_PERSON_VARIABLE, shape, radius, inRadius);
    //             }
    //         }
    //     }
    //
    //     bool wroteTimestep = false;
    //     auto openTimestep = [&]() {
    //         if (!wroteTimestep) {
    //             of.openTag("timestep").writeTime(SUMO_ATTR_TIME, timestep);
    //             wroteTimestep = true;
    //         }
    //     };
    //     if (!MSDevice_FCD::skipEmpty()) {
    //         openTimestep();
    //     }
    //     for (MSVehicleControl::constVehIt it = vc.loadedVehBegin(); it != vc.loadedVehEnd(); ++it) {
    //         const SUMOVehicle* const veh = it->second;
    //         if (isVisible(veh)) {
    //             const bool hasOutput = (tag == SUMO_TAG_NOTHING || tag == SUMO_TAG_VEHICLE) && hasOwnOutput(veh, filter, shapeFilter, (radius > 0 && inRadius.count(veh) > 0));
    //             if (hasOutput) {
    //                 openTimestep();
    //                 const MSVehicle* const microVeh = MSGlobals::gUseMesoSim ? nullptr : static_cast<const MSVehicle*>(veh);
    //                 Position pos = veh->getPosition();
    //                 if (useGeo) {
    //                     of.setPrecision(gPrecisionGeo);
    //                     GeoConvHelper::getFinal().cartesian2geo(pos);
    //                 } else if (useUTM) {
    //                     pos.sub(GeoConvHelper::getFinal().getOffset());
    //                 }
    //                 of.openTag(SUMO_TAG_VEHICLE);
    //                 of.writeAttr(SUMO_ATTR_ID, veh->getID());
    //                 of.writeOptionalAttr(SUMO_ATTR_X, pos.x(), mask);
    //                 of.writeOptionalAttr(SUMO_ATTR_Y, pos.y(), mask);
    //                 of.setPrecision(gPrecision);
    //                 of.writeOptionalAttr(SUMO_ATTR_Z, pos.z(), mask);
    //                 of.writeFuncAttr(SUMO_ATTR_ANGLE, [ = ]() {
    //                     return GeomHelper::naviDegree(veh->getAngle());
    //                 }, mask);
    //                 of.writeFuncAttr(SUMO_ATTR_TYPE, [ = ]() {
    //                     return veh->getVehicleType().getID();
    //                 }, mask);
    //                 of.writeFuncAttr(SUMO_ATTR_SPEED, [ = ]() {
    //                     return veh->getSpeed();
    //                 }, mask);
    //                 of.writeFuncAttr(SUMO_ATTR_SPEEDREL, [ = ]() {
    //                     const double speedLimit = veh->getEdge()->getSpeedLimit();
    //                     return speedLimit > 0 ? veh->getSpeed() / speedLimit : 0.;
    //                 }, mask);
    //                 of.writeFuncAttr(SUMO_ATTR_POSITION, [ = ]() {
    //                     return veh->getPositionOnLane();
    //                 }, mask);
    //                 of.writeFuncAttr(SUMO_ATTR_LANE, [ = ]() {
    //                     return MSGlobals::gUseMesoSim ? "" : microVeh->getLane()->getID();
    //                 }, mask, MSGlobals::gUseMesoSim);
    //                 of.writeFuncAttr(SUMO_ATTR_EDGE, [ = ]() {
    //                     return veh->getCurrentEdge()->getID();
    //                 }, mask, !MSGlobals::gUseMesoSim);
    //                 of.writeFuncAttr(SUMO_ATTR_SLOPE, [ = ]() {
    //                     return veh->getSlope();
    //                 }, mask);
    //                 if (!MSGlobals::gUseMesoSim) {
    //                     of.writeFuncAttr(SUMO_ATTR_SIGNALS, [ = ]() {
    //                         return microVeh->getSignals();
    //                     }, mask);
    //                     of.writeFuncAttr(SUMO_ATTR_ACCELERATION, [ = ]() {
    //                         return microVeh->getAcceleration();
    //                     }, mask);
    //                     of.writeFuncAttr(SUMO_ATTR_ACCELERATION_LAT, [ = ]() {
    //                         return microVeh->getLaneChangeModel().getAccelerationLat();
    //                     }, mask);
    //                     of.writeFuncAttr(SUMO_ATTR_SPEED_VEC, [ = ]() {
    //                         return GeomHelper::vectorize(microVeh->getSpeed(), microVeh->getAngle());
    //                     }, mask);
    //                     of.writeFuncAttr(SUMO_ATTR_ACCEL_VEC, [ = ]() {
    //                         return GeomHelper::vectorize(microVeh->getAcceleration(), microVeh->getAngle());
    //                     }, mask);
    //                 }
    //                 of.writeFuncAttr(SUMO_ATTR_DISTANCE, [ = ]() {
    //                     double lanePos = veh->getPositionOnLane();
    //                     if (!MSGlobals::gUseMesoSim && microVeh->getLane()->isInternal()) {
    //                         lanePos = microVeh->getRoute().getDistanceBetween(0., lanePos, microVeh->getEdge()->getLanes()[0], microVeh->getLane(),
    //                                   microVeh->getRoutePosition());
    //                     }
    //                     return veh->getEdge()->getDistanceAt(lanePos);
    //                 }, mask);
    //                 of.writeFuncAttr(SUMO_ATTR_ODOMETER, [ = ]() {
    //                     return veh->getOdometer();
    //                 }, mask);
    //                 of.writeFuncAttr(SUMO_ATTR_POSITION_LAT, [ = ]() {
    //                     return veh->getLateralPositionOnLane();
    //                 }, mask);
    //                 if (!MSGlobals::gUseMesoSim) {
    //                     of.writeFuncAttr(SUMO_ATTR_SPEED_LAT, [ = ]() {
    //                         return microVeh->getLaneChangeModel().getSpeedLat();
    //                     }, mask);
    //                 }
    //                 if (maxLeaderDistance >= 0 && !MSGlobals::gUseMesoSim) {
    //                     const std::pair<const MSVehicle* const, double> leader = microVeh->getLeader(maxLeaderDistance);
    //                     if (leader.first != nullptr) {
    //                         of.writeFuncAttr(SUMO_ATTR_LEADER_ID, [ = ]() {
    //                             return leader.first->getID();
    //                         }, mask);
    //                         of.writeFuncAttr(SUMO_ATTR_LEADER_SPEED, [ = ]() {
    //                             return leader.first->getSpeed();
    //                         }, mask);
    //                         of.writeFuncAttr(SUMO_ATTR_LEADER_GAP, [ = ]() {
    //                             return leader.second + microVeh->getVehicleType().getMinGap();
    //                         }, mask);
    //                     } else {
    //                         of.writeFuncAttr(SUMO_ATTR_LEADER_ID, [ = ]() {
    //                             return "";
    //                         }, mask);
    //                         of.writeFuncAttr(SUMO_ATTR_LEADER_SPEED, [ = ]() {
    //                             return -1;
    //                         }, mask);
    //                         of.writeFuncAttr(SUMO_ATTR_LEADER_GAP, [ = ]() {
    //                             return -1;
    //                         }, mask);
    //                     }
    //                 }
    //                 for (const std::string& key : params) {
    //                     std::string error;
    //                     const std::string value = static_cast<const MSBaseVehicle*>(veh)->getPrefixedParameter(key, error);
    //                     if (value != "") {
    //                         of.writeAttr(StringUtils::escapeXML(key), StringUtils::escapeXML(value));
    //                     }
    //                 }
    //                 of.writeFuncAttr(SUMO_ATTR_ARRIVALDELAY, [ = ]() {
    //                     const double arrivalDelay = static_cast<const MSBaseVehicle*>(veh)->getStopArrivalDelay();
    //                     if (arrivalDelay == INVALID_DOUBLE) {
    //                         // no upcoming stop also means that there is no delay
    //                         return 0.;
    //                     }
    //                     return arrivalDelay;
    //                 }, mask);
    //                 of.writeFuncAttr(SUMO_ATTR_DELAY, [ = ]() {
    //                     const double delay = static_cast<const MSBaseVehicle*>(veh)->getStopDelay();
    //                     if (delay < 0) {
    //                         // no upcoming stop also means that there is no delay
    //                         return 0.;
    //                     }
    //                     return delay;
    //                 }, mask);
    //                 if (MSGlobals::gUseMesoSim) {
    //                     const MEVehicle* mesoVeh = static_cast<const MEVehicle*>(veh);
    //                     of.writeFuncAttr(SUMO_ATTR_SEGMENT, [ = ]() {
    //                         return mesoVeh->getSegmentIndex();
    //                     }, mask);
    //                     of.writeFuncAttr(SUMO_ATTR_QUEUE, [ = ]() {
    //                         return mesoVeh->getQueIndex();
    //                     }, mask);
    //                     of.writeFuncAttr(SUMO_ATTR_ENTRYTIME, [ = ]() {
    //                         return mesoVeh->getLastEntryTimeSeconds();
    //                     }, mask);
    //                     of.writeFuncAttr(SUMO_ATTR_EVENTTIME, [ = ]() {
    //                         return mesoVeh->getEventTimeSeconds();
    //                     }, mask);
    //                     of.writeFuncAttr(SUMO_ATTR_BLOCKTIME, [ = ]() {
    //                         return mesoVeh->getBlockTime() == SUMOTime_MAX ? -1.0 : mesoVeh->getBlockTimeSeconds();
    //                     }, mask);
    //                 }
    //                 of.writeFuncAttr(SUMO_ATTR_TAG, [ = ]() {
    //                     return toString(SUMO_TAG_VEHICLE);
    //                 }, mask);
    //                 of.writeOptionalAttr(SUMO_ATTR_PERSON_NUMBER, veh->getPersonNumber(), mask);
    //                 of.writeOptionalAttr(SUMO_ATTR_CONTAINER_NUMBER, veh->getContainerNumber(), mask);
    //                 MSEmissionExport::writeEmissions(of, static_cast<const MSBaseVehicle*>(veh), false, mask);
    //                 of.closeTag();
    //             }
    //             // write persons and containers in the vehicle
    //             if (tag == SUMO_TAG_NOTHING || tag == SUMO_TAG_PERSON) {
    //                 const MSEdge* edge = MSGlobals::gUseMesoSim ? veh->getEdge() : &veh->getLane()->getEdge();
    //                 for (const MSTransportable* const person : veh->getPersons()) {
    //                     if (hasOwnOutput(person, filter, shapeFilter, inRadius.count(person) > 0)) {
    //                         openTimestep();
    //                         writeTransportable(of, edge, person, veh, SUMO_TAG_PERSON, useGeo, mask);
    //                     }
    //                 }
    //                 for (const MSTransportable* const container : veh->getContainers()) {
    //                     if (hasOwnOutput(container, filter, shapeFilter, inRadius.count(container) > 0)) {
    //                         openTimestep();
    //                         writeTransportable(of, edge, container, veh, SUMO_TAG_CONTAINER, useGeo, mask);
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     if (tag == SUMO_TAG_NOTHING || tag == SUMO_TAG_PERSON) {
    //         if (net->hasPersons() && net->getPersonControl().hasTransportables()) {
    //             // write persons who are not in a vehicle
    //             for (const MSEdge* const e : net->getEdgeControl().getEdges()) {
    //                 if (filter && MSDevice_FCD::getEdgeFilter().count(e) == 0) {
    //                     continue;
    //                 }
    //                 for (const MSTransportable* const person : e->getSortedPersons(timestep)) {
    //                     if (hasOwnOutput(person, filter, shapeFilter, inRadius.count(person) > 0)) {
    //                         openTimestep();
    //                         writeTransportable(of, e, person, nullptr, SUMO_TAG_PERSON, useGeo, mask);
    //                     }
    //                 }
    //             }
    //         }
    //         if (net->hasContainers() && net->getContainerControl().hasTransportables()) {
    //             // write containers which are not in a vehicle
    //             for (const MSEdge* const e : net->getEdgeControl().getEdges()) {
    //                 if (filter && MSDevice_FCD::getEdgeFilter().count(e) == 0) {
    //                     continue;
    //                 }
    //                 for (MSTransportable* container : e->getSortedContainers(timestep)) {
    //                     if (hasOwnOutput(container, filter, shapeFilter, inRadius.count(container) > 0)) {
    //                         openTimestep();
    //                         writeTransportable(of, e, container, nullptr, SUMO_TAG_CONTAINER, useGeo, mask);
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     if (wroteTimestep) {
    //         of.closeTag();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn write(of: &mut OutputDevice, timestep: SUMOTime, tag: SumoXMLTag) {
        // C++ lambda openTimestep, as a nested fn so `of` is not captured twice
        fn open_timestep(of: &mut OutputDevice, wroteTimestep: &mut bool, timestep: SUMOTime) {
            if !*wroteTimestep {
                of.open_tag("timestep").write_time(SUMO_ATTR_TIME, timestep);
                *wroteTimestep = true;
            }
        }
        MSDevice_FCD::init_once();
        let period = MSDevice_FCD::get_period();
        let begin = MSDevice_FCD::get_begin();
        if (period > 0 && (timestep - begin) % period != 0) || timestep < begin {
            return;
        }
        let mask = MSDevice_FCD::get_written_attributes();
        let useGeo = MSDevice_FCD::use_geo();
        let useUTM = MSDevice_FCD::use_utm();
        let maxLeaderDistance = MSDevice_FCD::get_max_leader_distance();
        let params = MSDevice_FCD::get_params_to_write();
        let net = MSNet::get_instance();
        let vc = net.m().get_vehicle_control();
        let radius = MSDevice_FCD::get_radius();
        let edgeFilter = MSDevice_FCD::get_edge_filter();
        let filter = edgeFilter.len() > 0;
        let shapeFilter = MSDevice_FCD::has_shape_filter();
        // determinism: BTreeMap/BTreeSet iterate in key order, like std::map/std::set
        let mut inRadius: BTreeSet<Ptr<Named>> = BTreeSet::new();
        if radius > 0.0 {
            // collect all vehicles in radius around equipped vehicles
            for veh in vc.myVehicleDict.values() {
                let veh: Ptr<SUMOVehicle> = *veh;
                if Self::is_visible(veh) && Self::has_own_output(veh, filter, shapeFilter, false) {
                    let shape = PositionVector(vec![veh.get_position(0.0)]);
                    crate::libsumo::helper::Helper::collect_objects_in_range(CMD_GET_VEHICLE_VARIABLE, &shape, radius, &mut inRadius);
                    crate::libsumo::helper::Helper::collect_objects_in_range(CMD_GET_PERSON_VARIABLE, &shape, radius, &mut inRadius);
                }
            }
        }

        let mut wroteTimestep = false;
        if !MSDevice_FCD::skip_empty() {
            open_timestep(of, &mut wroteTimestep, timestep);
        }
        for veh in vc.myVehicleDict.values() {
            let veh: Ptr<SUMOVehicle> = *veh;
            if Self::is_visible(veh) {
                let hasOutput = (tag == SUMO_TAG_NOTHING || tag == SUMO_TAG_VEHICLE) && Self::has_own_output(veh, filter, shapeFilter, radius > 0.0 && inRadius.contains(&as_named(veh)));
                if hasOutput {
                    open_timestep(of, &mut wroteTimestep, timestep);
                    // static_cast<const MSVehicle*>(veh), null in meso
                    let microVeh: Ptr<MSVehicle> = if use_meso_sim() { Ptr::default() } else { veh.cast::<MSVehicle>() };
                    let mut pos = veh.get_position(0.0);
                    if useGeo {
                        of.set_precision(g_precision_geo());
                        GeoConvHelper::with_final(|g| g.cartesian2geo(&mut pos));
                    } else if useUTM {
                        let offset = GeoConvHelper::with_final(|g| g.get_offset());
                        pos.sub(&offset);
                    }
                    of.open_tag_id(SUMO_TAG_VEHICLE);
                    of.write_attr(SUMO_ATTR_ID, veh.get_id());
                    of.write_optional_attr_mask(SUMO_ATTR_X, &pos.x(), mask, false);
                    of.write_optional_attr_mask(SUMO_ATTR_Y, &pos.y(), mask, false);
                    of.set_precision(g_precision());
                    of.write_optional_attr_mask(SUMO_ATTR_Z, &pos.z(), mask, false);
                    of.write_func_attr(SUMO_ATTR_ANGLE, || utils::geom::geom_helper::navi_degree(veh.get_angle()), mask, false);
                    of.write_func_attr(SUMO_ATTR_TYPE, || veh.get_vehicle_type().get_id().to_string(), mask, false);
                    of.write_func_attr(SUMO_ATTR_SPEED, || veh.get_speed(), mask, false);
                    of.write_func_attr(
                        SUMO_ATTR_SPEEDREL,
                        || {
                            let speedLimit = veh.get_edge().get_speed_limit();
                            if speedLimit > 0.0 { veh.get_speed() / speedLimit } else { 0.0 }
                        },
                        mask,
                        false,
                    );
                    of.write_func_attr(SUMO_ATTR_POSITION, || veh.get_position_on_lane(), mask, false);
                    of.write_func_attr(SUMO_ATTR_LANE, || if use_meso_sim() { String::new() } else { microVeh.get_lane().get_id().to_string() }, mask, use_meso_sim());
                    of.write_func_attr(SUMO_ATTR_EDGE, || veh.get_current_edge().get_id().to_string(), mask, !use_meso_sim());
                    of.write_func_attr(SUMO_ATTR_SLOPE, || veh.get_slope(), mask, false);
                    if !use_meso_sim() {
                        of.write_func_attr(SUMO_ATTR_SIGNALS, || microVeh.get_signals(), mask, false);
                        of.write_func_attr(SUMO_ATTR_ACCELERATION, || microVeh.get_acceleration(), mask, false);
                        of.write_func_attr(SUMO_ATTR_ACCELERATION_LAT, || microVeh.get_lane_change_model_2().get_acceleration_lat(), mask, false);
                        of.write_func_attr(SUMO_ATTR_SPEED_VEC, || utils::geom::geom_helper::vectorize(microVeh.get_speed(), microVeh.get_angle()).to_string(), mask, false);
                        of.write_func_attr(SUMO_ATTR_ACCEL_VEC, || utils::geom::geom_helper::vectorize(microVeh.get_acceleration(), microVeh.get_angle()).to_string(), mask, false);
                    }
                    of.write_func_attr(
                        SUMO_ATTR_DISTANCE,
                        || {
                            let mut lanePos = veh.get_position_on_lane();
                            if !use_meso_sim() && microVeh.get_lane().is_internal() {
                                lanePos = microVeh.get_route().get_distance_between(0.0, lanePos, microVeh.get_edge().get_lanes()[0], microVeh.get_lane(), microVeh.get_route_position());
                            }
                            veh.get_edge().get_distance_at(lanePos)
                        },
                        mask,
                        false,
                    );
                    of.write_func_attr(SUMO_ATTR_ODOMETER, || veh.get_odometer(), mask, false);
                    of.write_func_attr(SUMO_ATTR_POSITION_LAT, || veh.get_lateral_position_on_lane(), mask, false);
                    if !use_meso_sim() {
                        of.write_func_attr(SUMO_ATTR_SPEED_LAT, || microVeh.get_lane_change_model_2().get_speed_lat(), mask, false);
                    }
                    if maxLeaderDistance >= 0.0 && !use_meso_sim() {
                        let leader = microVeh.get_leader(maxLeaderDistance, true);
                        if !leader.0.is_null() {
                            of.write_func_attr(SUMO_ATTR_LEADER_ID, || leader.0.get_id().to_string(), mask, false);
                            of.write_func_attr(SUMO_ATTR_LEADER_SPEED, || leader.0.get_speed(), mask, false);
                            of.write_func_attr(SUMO_ATTR_LEADER_GAP, || leader.1 + microVeh.get_vehicle_type().get_min_gap(), mask, false);
                        } else {
                            of.write_func_attr(SUMO_ATTR_LEADER_ID, || String::new(), mask, false);
                            of.write_func_attr(SUMO_ATTR_LEADER_SPEED, || -1i32, mask, false);
                            of.write_func_attr(SUMO_ATTR_LEADER_GAP, || -1i32, mask, false);
                        }
                    }
                    for key in params.iter() {
                        let mut error = String::new();
                        let value = veh.cast::<MSBaseVehicle>().get_prefixed_parameter(key, &mut error);
                        if value != "" {
                            let escKey = utils::common::string_utils::escape_xml(key, false);
                            of.write_attr(escKey.as_str(), &utils::common::string_utils::escape_xml(&value, false));
                        }
                    }
                    of.write_func_attr(
                        SUMO_ATTR_ARRIVALDELAY,
                        || {
                            let arrivalDelay = veh.cast::<MSBaseVehicle>().get_stop_arrival_delay();
                            if arrivalDelay == INVALID_DOUBLE {
                                // no upcoming stop also means that there is no delay
                                return 0.0;
                            }
                            arrivalDelay
                        },
                        mask,
                        false,
                    );
                    of.write_func_attr(
                        SUMO_ATTR_DELAY,
                        || {
                            let delay = veh.cast::<MSBaseVehicle>().get_stop_delay();
                            if delay < 0.0 {
                                // no upcoming stop also means that there is no delay
                                return 0.0;
                            }
                            delay
                        },
                        mask,
                        false,
                    );
                    if use_meso_sim() {
                        let mesoVeh: Ptr<MEVehicle> = veh.cast::<MEVehicle>();
                        of.write_func_attr(SUMO_ATTR_SEGMENT, || mesoVeh.get_segment_index(), mask, false);
                        of.write_func_attr(SUMO_ATTR_QUEUE, || mesoVeh.get_que_index(), mask, false);
                        of.write_func_attr(SUMO_ATTR_ENTRYTIME, || mesoVeh.get_last_entry_time_seconds(), mask, false);
                        of.write_func_attr(SUMO_ATTR_EVENTTIME, || mesoVeh.get_event_time_seconds(), mask, false);
                        of.write_func_attr(SUMO_ATTR_BLOCKTIME, || if mesoVeh.get_block_time() == SUMOTIME_MAX { -1.0 } else { mesoVeh.get_block_time_seconds() }, mask, false);
                    }
                    of.write_func_attr(SUMO_ATTR_TAG, || crate::utils::handlers::common_handler::tag_str(SUMO_TAG_VEHICLE), mask, false);
                    of.write_optional_attr_mask(SUMO_ATTR_PERSON_NUMBER, &veh.get_person_number(), mask, false);
                    of.write_optional_attr_mask(SUMO_ATTR_CONTAINER_NUMBER, &veh.get_container_number(), mask, false);
                    MSEmissionExport::write_emissions(of, veh.cast::<MSBaseVehicle>(), false, &mask);
                    of.close_tag("");
                }
                // write persons and containers in the vehicle
                if tag == SUMO_TAG_NOTHING || tag == SUMO_TAG_PERSON {
                    let edge: Ptr<MSEdge> = if use_meso_sim() { veh.get_edge() } else { Ptr::from_mut(veh.get_lane().get_edge()) };
                    for person in veh.get_persons().iter() {
                        let person: Ptr<MSTransportable> = *person;
                        if Self::has_own_output_2(person, filter, shapeFilter, inRadius.contains(&as_named(person))) {
                            open_timestep(of, &mut wroteTimestep, timestep);
                            Self::write_transportable(of, edge, person, veh, SUMO_TAG_PERSON, useGeo, mask);
                        }
                    }
                    for container in veh.get_containers().iter() {
                        let container: Ptr<MSTransportable> = *container;
                        if Self::has_own_output_2(container, filter, shapeFilter, inRadius.contains(&as_named(container))) {
                            open_timestep(of, &mut wroteTimestep, timestep);
                            Self::write_transportable(of, edge, container, veh, SUMO_TAG_CONTAINER, useGeo, mask);
                        }
                    }
                }
            }
        }
        if tag == SUMO_TAG_NOTHING || tag == SUMO_TAG_PERSON {
            if net.m().has_persons() && net.m().get_person_control().has_transportables() {
                // write persons who are not in a vehicle
                for e in net.m().get_edge_control().get_edges().iter() {
                    let e: Ptr<MSEdge> = *e;
                    if filter && !edgeFilter.contains(&e) {
                        continue;
                    }
                    for person in e.get_sorted_persons(timestep, false).iter() {
                        let person: Ptr<MSTransportable> = *person;
                        if Self::has_own_output_2(person, filter, shapeFilter, inRadius.contains(&as_named(person))) {
                            open_timestep(of, &mut wroteTimestep, timestep);
                            Self::write_transportable(of, e, person, Ptr::default(), SUMO_TAG_PERSON, useGeo, mask);
                        }
                    }
                }
            }
            if net.m().has_containers() && net.m().get_container_control().has_transportables() {
                // write containers which are not in a vehicle
                for e in net.m().get_edge_control().get_edges().iter() {
                    let e: Ptr<MSEdge> = *e;
                    if filter && !edgeFilter.contains(&e) {
                        continue;
                    }
                    for container in e.get_sorted_containers(timestep, false).iter() {
                        let container: Ptr<MSTransportable> = *container;
                        if Self::has_own_output_2(container, filter, shapeFilter, inRadius.contains(&as_named(container))) {
                            open_timestep(of, &mut wroteTimestep, timestep);
                            Self::write_transportable(of, e, container, Ptr::default(), SUMO_TAG_CONTAINER, useGeo, mask);
                        }
                    }
                }
            }
        }
        if wroteTimestep {
            of.close_tag("");
        }
    }

    /// C++: `void MSFCDExport::writeTransportable(OutputDevice &, const MSEdge *const, const MSTransportable *const, const SUMOVehicle *const, const SumoXMLTag, const bool, const SumoXMLAttrMask)` [static, private]
    /// write transportable
    // void
    // MSFCDExport::writeTransportable(OutputDevice& of, const MSEdge* const e, const MSTransportable* const p, const SUMOVehicle* const v,
    //                                 const SumoXMLTag tag, const bool useGeo, const SumoXMLAttrMask mask) {
    //     Position pos = p->getPosition();
    //     if (useGeo) {
    //         of.setPrecision(gPrecisionGeo);
    //         GeoConvHelper::getFinal().cartesian2geo(pos);
    //     }
    //     of.openTag(tag);
    //     of.writeAttr(SUMO_ATTR_ID, p->getID());
    //     of.writeOptionalAttr(SUMO_ATTR_X, pos.x(), mask);
    //     of.writeOptionalAttr(SUMO_ATTR_Y, pos.y(), mask);
    //     of.setPrecision(gPrecision);
    //     of.writeOptionalAttr(SUMO_ATTR_Z, pos.z(), mask);
    //     of.writeOptionalAttr(SUMO_ATTR_ANGLE, GeomHelper::naviDegree(p->getAngle()), mask);
    //     of.writeOptionalAttr(SUMO_ATTR_TYPE, p->getVehicleType().getID(), mask);
    //     of.writeOptionalAttr(SUMO_ATTR_SPEED, p->getSpeed(), mask);
    //     of.writeOptionalAttr(SUMO_ATTR_SPEEDREL, e->getSpeedLimit() > 0 ? p->getSpeed() / e->getSpeedLimit() : 0., mask);
    //     of.writeOptionalAttr(SUMO_ATTR_POSITION, p->getEdgePos(), mask);
    //     of.writeOptionalAttr(SUMO_ATTR_LANE, "", mask, true);
    //     of.writeOptionalAttr(SUMO_ATTR_EDGE, e->getID(), mask);
    //     of.writeOptionalAttr(SUMO_ATTR_SLOPE, e->getLanes()[0]->getShape().slopeDegreeAtOffset(p->getEdgePos()), mask);
    //     of.writeOptionalAttr(SUMO_ATTR_VEHICLE, v == nullptr ? "" : v->getID(), mask);
    //     of.writeOptionalAttr(SUMO_ATTR_STAGE, p->getCurrentStageDescription(), mask);
    //     of.writeOptionalAttr(SUMO_ATTR_TAG, toString(tag), mask);
    //     of.closeTag();
    // }
    #[allow(unused_variables)]
    pub fn write_transportable(of: &mut OutputDevice, e: Ptr<MSEdge>, p: Ptr<MSTransportable>, v: Ptr<SUMOVehicle>, tag: SumoXMLTag, useGeo: bool, mask: SumoXMLAttrMask) {
        let mut pos = p.get_position_2();
        if useGeo {
            of.set_precision(g_precision_geo());
            GeoConvHelper::with_final(|g| g.cartesian2geo(&mut pos));
        }
        of.open_tag_id(tag);
        of.write_attr(SUMO_ATTR_ID, p.get_id());
        of.write_optional_attr_mask(SUMO_ATTR_X, &pos.x(), mask, false);
        of.write_optional_attr_mask(SUMO_ATTR_Y, &pos.y(), mask, false);
        of.set_precision(g_precision());
        of.write_optional_attr_mask(SUMO_ATTR_Z, &pos.z(), mask, false);
        of.write_optional_attr_mask(SUMO_ATTR_ANGLE, &utils::geom::geom_helper::navi_degree(p.get_angle()), mask, false);
        of.write_optional_attr_mask(SUMO_ATTR_TYPE, p.get_vehicle_type().get_id(), mask, false);
        of.write_optional_attr_mask(SUMO_ATTR_SPEED, &p.get_speed(), mask, false);
        let speedRel = if e.get_speed_limit() > 0.0 { p.get_speed() / e.get_speed_limit() } else { 0.0 };
        of.write_optional_attr_mask(SUMO_ATTR_SPEEDREL, &speedRel, mask, false);
        of.write_optional_attr_mask(SUMO_ATTR_POSITION, &p.get_edge_pos(), mask, false);
        of.write_optional_attr_mask(SUMO_ATTR_LANE, "", mask, true);
        of.write_optional_attr_mask(SUMO_ATTR_EDGE, e.get_id(), mask, false);
        let slope = e.get_lanes()[0].get_shape(false).slope_degree_at_offset(p.get_edge_pos());
        of.write_optional_attr_mask(SUMO_ATTR_SLOPE, &slope, mask, false);
        let vehId: String = if v.is_null() { String::new() } else { v.get_id().to_string() };
        of.write_optional_attr_mask(SUMO_ATTR_VEHICLE, &vehId, mask, false);
        of.write_optional_attr_mask(SUMO_ATTR_STAGE, &p.get_current_stage_description(), mask, false);
        of.write_optional_attr_mask(SUMO_ATTR_TAG, &crate::utils::handlers::common_handler::tag_str(tag), mask, false);
        of.close_tag("");
    }

    /// C++: `bool MSFCDExport::isVisible(const SUMOVehicle *)` [static, private]
    // bool
    // MSFCDExport::isVisible(const SUMOVehicle* veh) {
    //     return veh->isOnRoad() || veh->isParking() || veh->isRemoteControlled();
    // }
    #[allow(unused_variables)]
    pub fn is_visible(veh: Ptr<SUMOVehicle>) -> bool {
        veh.is_on_road() || veh.is_parking() || veh.is_remote_controlled()
    }

    /// C++: `bool MSFCDExport::hasOwnOutput(const SUMOVehicle *, bool, bool, bool)` [static, private]
    // bool
    // MSFCDExport::hasOwnOutput(const SUMOVehicle* veh, bool filter, bool shapeFilter, bool isInRadius) {
    //     return ((!filter || MSDevice_FCD::getEdgeFilter().count(veh->getEdge()) > 0)
    //             && (!shapeFilter || MSDevice_FCD::shapeFilter(veh))
    //             && ((veh->getDevice(typeid(MSDevice_FCD)) != nullptr) || isInRadius));
    // }
    #[allow(unused_variables)]
    pub fn has_own_output(veh: Ptr<SUMOVehicle>, filter: bool, shapeFilter: bool, isInRadius: bool) -> bool {
        // shortcut: getDevice(typeid(MSDevice_FCD)) has no type-keyed lookup, any device on the vehicle counts (same as ms_vehicle.rs), upgrade when getDevice takes a type key
        (!filter || MSDevice_FCD::get_edge_filter().contains(&veh.get_edge()))
            && (!shapeFilter || MSDevice_FCD::shape_filter(veh.cast::<SUMOTrafficObject>()))
            && (!veh.get_device(&()).is_null() || isInRadius)
    }

    /// C++: `bool MSFCDExport::hasOwnOutput(const MSTransportable *, bool, bool, bool)` [static, private]
    // bool
    // MSFCDExport::hasOwnOutput(const MSTransportable* p, bool filter, bool shapeFilter, bool isInRadius) {
    //     return ((!filter || MSDevice_FCD::getEdgeFilter().count(p->getEdge()) > 0)
    //             && (!shapeFilter || MSDevice_FCD::shapeFilter(p))
    //             && ((p->getDevice(typeid(MSTransportableDevice_FCD)) != nullptr) || isInRadius));
    // }
    #[allow(unused_variables)]
    pub fn has_own_output_2(p: Ptr<MSTransportable>, filter: bool, shapeFilter: bool, isInRadius: bool) -> bool {
        // shortcut: MSDevice_FCD::shapeFilter has only the SUMOTrafficObject overload, the MSTransportable one is not ported; the device check uses the first device like the vehicle variant
        (!filter || MSDevice_FCD::get_edge_filter().contains(&p.get_edge()))
            && (!shapeFilter || panic!("MSDevice_FCD::shapeFilter(MSTransportable) is not ported yet"))
            && (!p.get_device(&()).is_null() || isInRadius)
    }

    /// C++: `void MSFCDExport::MSFCDExport(const MSFCDExport &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new(arg0: &MSFCDExport) -> Self {
        panic!("MSFCDExport copy constructor is private and undefined in C++")
    }

    /// C++: `MSFCDExport & MSFCDExport::operator=(const MSFCDExport &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSFCDExport) -> &mut MSFCDExport {
        panic!("MSFCDExport assignment operator is private and undefined in C++")
    }

}

