//! Port of microsim/output/MSQueueExport.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use std::cell::{Cell, RefCell};
use utils::geom::POSITION_EPS;
use utils::options::options_cont::{get_bool, get_float, get_string};

fn min2(a: f64, b: f64) -> f64 {
    if a < b { a } else { b }
}
fn max2(a: f64, b: f64) -> f64 {
    if a > b { a } else { b }
}
/// C++ macro `SIMTIME`
fn simtime() -> f64 {
    steps2time(MSNet::get_instance().get_current_time_step())
}
fn sort_samples(v: &mut [f64]) {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
}

thread_local! {
    /// C++ static `MSQueueExport::myIntervalStart`
    static MY_INTERVAL_START: Cell<SUMOTime> = const { Cell::new(SUMOTIME_MIN) };
    /// C++ static `MSQueueExport::myEdgeSamples` (count, length) per edge
    static MY_EDGE_SAMPLES: RefCell<BTreeMap<Ptr<MSEdge>, Vec<(f64, f64)>>> = const { RefCell::new(BTreeMap::new()) };
}

/// Export the queueing length in front of a junction (very experimental!)
/// C++ class `MSQueueExport` (microsim/output/MSQueueExport.h)
pub struct MSQueueExport {
}

// C++ static members of MSQueueExport (port as module-level statics, see AGENTS.md):
//   static SUMOTime MSQueueExport::myIntervalStart;
//   static int MSQueueExport::myEdgeSamples;

impl MSQueueExport {
    /// C++: `void MSQueueExport::write(OutputDevice &, SUMOTime)` [static]
    /// Export the queueing length in front of a junction (very experimental!)
    // void
    // MSQueueExport::write(OutputDevice& of, SUMOTime timestep) {
    //     const OptionsCont& oc = OptionsCont::getOptions();
    //     const SUMOTime begin = string2time(oc.getString("begin"));
    //     const SUMOTime period = string2time(oc.getString("queue-output.period"));
    //     if (period > 0 && (timestep - begin) % period != 0) {
    //         return;
    //     }
    //     const double threshold = oc.getFloat("queue-output.speed-threshold");
    //     const SUMOTime aggregation = string2time(oc.getString("queue-output.aggregation"));
    //     if (aggregation > 0) {
    //         const SUMOTime intervalStart = begin + ((timestep - begin) / aggregation) * aggregation;
    //         if (myIntervalStart != SUMOTime_MIN && intervalStart != myIntervalStart) {
    //             writeInterval(of, myIntervalStart, myIntervalStart + aggregation);
    //         }
    //         myIntervalStart = intervalStart;
    //         writeEdge(nullptr, threshold, [] {});
    //         return;
    //     }
    //     // The timestep block is opened lazily so that with --queue-output.skip-empty
    //     // (and, in particular, for columnar formats like Parquet or CSV) no row is
    //     // emitted for time steps without any queue.
    //     bool wrapperOpen = false;
    //     auto ensureOpen = [&]() {
    //         if (!wrapperOpen) {
    //             of.openTag("data").writeAttr("timestep", time2string(timestep));
    //             of.openTag("lanes");
    //             wrapperOpen = true;
    //         }
    //     };
    //     if (!oc.getBool("queue-output.skip-empty")) {
    //         ensureOpen();
    //     }
    //     writeEdge(&of, threshold, ensureOpen);
    //     if (wrapperOpen) {
    //         of.closeTag(); // lanes
    //         of.closeTag(); // data
    //     }
    // }
    #[allow(unused_variables)]
    pub fn write(of: &mut OutputDevice, timestep: SUMOTime) {
        let begin = string2time(&get_string("begin")).unwrap_or_else(|e| panic!("{e}"));
        let period = string2time(&get_string("queue-output.period")).unwrap_or_else(|e| panic!("{e}"));
        if period > 0 && (timestep - begin) % period != 0 {
            return;
        }
        let threshold = get_float("queue-output.speed-threshold");
        let aggregation = string2time(&get_string("queue-output.aggregation")).unwrap_or_else(|e| panic!("{e}"));
        if aggregation > 0 {
            let interval_start = begin + ((timestep - begin) / aggregation) * aggregation;
            if MY_INTERVAL_START.get() != SUMOTIME_MIN && interval_start != MY_INTERVAL_START.get() {
                MSQueueExport::write_interval(of, MY_INTERVAL_START.get(), MY_INTERVAL_START.get() + aggregation);
            }
            MY_INTERVAL_START.set(interval_start);
            let noop: Box<dyn Fn()> = Box::new(|| {});
            MSQueueExport::write_edge(Ptr::null(), threshold, &noop);
            return;
        }
        // The timestep block is opened lazily so that with --queue-output.skip-empty
        // (and, in particular, for columnar formats like Parquet or CSV) no row is
        // emitted for time steps without any queue.
        let wrapper_open = Rc::new(Cell::new(false));
        let of_p = Ptr::from_mut(of);
        let ensure_open: Box<dyn Fn()> = {
            let wrapper_open = wrapper_open.clone();
            Box::new(move || {
                if !wrapper_open.get() {
                    of_p.m().open_tag("data").write_attr("timestep", &time2string(timestep));
                    of_p.m().open_tag("lanes");
                    wrapper_open.set(true);
                }
            })
        };
        if !get_bool("queue-output.skip-empty") {
            ensure_open();
        }
        MSQueueExport::write_edge(of_p, threshold, &ensure_open);
        if wrapper_open.get() {
            of_p.m().close_tag(""); // lanes
            of_p.m().close_tag(""); // data
        }
    }

    /// C++: `void MSQueueExport::finish(OutputDevice &, SUMOTime)` [static]
    /// Writes the last (possibly incomplete) aggregation interval and clears the collected samples
    // void
    // MSQueueExport::finish(OutputDevice& of, SUMOTime timestep) {
    //     const SUMOTime aggregation = string2time(OptionsCont::getOptions().getString("queue-output.aggregation"));
    //     if (aggregation > 0 && myIntervalStart != SUMOTime_MIN) {
    //         writeInterval(of, myIntervalStart, MIN2(myIntervalStart + aggregation, timestep));
    //     }
    //     myIntervalStart = SUMOTime_MIN;
    //     myEdgeSamples.clear();
    // }
    #[allow(unused_variables)]
    pub fn finish(of: &mut OutputDevice, timestep: SUMOTime) {
        let aggregation = string2time(&get_string("queue-output.aggregation")).unwrap_or_else(|e| panic!("{e}"));
        if aggregation > 0 && MY_INTERVAL_START.get() != SUMOTIME_MIN {
            MSQueueExport::write_interval(of, MY_INTERVAL_START.get(), std::cmp::min(MY_INTERVAL_START.get() + aggregation, timestep));
        }
        MY_INTERVAL_START.set(SUMOTIME_MIN);
        MY_EDGE_SAMPLES.with(|s| s.borrow_mut().clear());
    }

    /// C++: `void MSQueueExport::MSQueueExport(const MSQueueExport &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new(arg0: &MSQueueExport) -> Self {
        panic!("MSQueueExport copy constructor is private and invalidated in SUMO")
    }

    /// C++: `MSQueueExport & MSQueueExport::operator=(const MSQueueExport &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSQueueExport) -> &mut MSQueueExport {
        panic!("MSQueueExport assignment operator is private and invalidated in SUMO")
    }

    /// C++: `void MSQueueExport::writeEdge(OutputDevice *, double, const std::function<void ()> &)` [static, private]
    /// Iterates through the edges and their lanes (micro) or segment queues (meso). If of is nullptr, queue length samples are collected for aggregation instead of being written. ensureOpen lazily opens the enclosing timestep element before the first record is written.
    // void
    // MSQueueExport::writeEdge(OutputDevice* of, double threshold, const std::function<void()>& ensureOpen) {
    //     MSEdgeControl& ec = MSNet::getInstance()->getEdgeControl();
    //     for (const MSEdge* const edge : ec.getEdges()) {
    //         if (MSGlobals::gUseMesoSim) {
    //             double segmentOffset = 0.;
    //             for (const MESegment* segment = MSGlobals::gMesoNet->getSegmentForEdge(*edge); segment != nullptr; segment = segment->getNextSegment()) {
    //                 for (int qIdx = 0; qIdx < segment->numQueues(); ++qIdx) {
    //                     writeMesoQueue(of, *edge, *segment, qIdx, segmentOffset, threshold, ensureOpen);
    //                 }
    //                 segmentOffset += segment->getLength();
    //             }
    //         } else {
    //             for (const MSLane* const lane : edge->getLanes()) {
    //                 writeLane(of, *lane, threshold, ensureOpen);
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn write_edge(of: Ptr<OutputDevice>, threshold: f64, ensureOpen: &Box<dyn Fn()>) {
        let net = MSNet::get_instance();
        let ec = net.m().get_edge_control();
        for &edge in ec.get_edges() {
            if MSGlobals::use_meso_sim() {
                let mut segment_offset = 0.0_f64;
                let mut segment = meso_net().get_segment_for_edge(&*edge, 0.0);
                while !segment.is_null() {
                    for q_idx in 0..segment.num_queues() {
                        MSQueueExport::write_meso_queue(of, &*edge, &*segment, q_idx, segment_offset, threshold, ensureOpen);
                    }
                    segment_offset += segment.get_length();
                    segment = segment.get_next_segment();
                }
            } else {
                for &lane in edge.get_lanes().iter() {
                    MSQueueExport::write_lane(of, &*lane, threshold, ensureOpen);
                }
            }
        }
    }

    /// C++: `void MSQueueExport::writeLane(OutputDevice *, const MSLane &, double, const std::function<void ()> &)` [static, private]
    /// Checks a single lane for a vehicle queue (micro)
    // void
    // MSQueueExport::writeLane(OutputDevice* of, const MSLane& lane, double threshold, const std::function<void()>& ensureOpen) {
    //     // maximum of all vehicle waiting times
    //     double queueing_time = 0.0;
    //     // back of last stopped vehicle (XXX does not check for continuous queue)
    //     double queueing_length = 0.0;
    //     // back of last slow vehicle (XXX does not check for continuous queue)
    //     double queueing_length2 = 0.0;
    //     // number of slow vehicles
    //     int queueing_count = 0;
    //
    //     if (!lane.empty()) {
    //         for (MSLane::VehCont::const_iterator it_veh = lane.myVehicles.begin(); it_veh != lane.myVehicles.end(); ++it_veh) {
    //             const MSVehicle& veh = **it_veh;
    //             if (!veh.isOnRoad()) {
    //                 continue;
    //             }
    //
    //             if (veh.getWaitingSeconds() > 0) {
    //                 queueing_time = MAX2(veh.getWaitingSeconds(), queueing_time);
    //                 const double veh_back_to_lane_end = (lane.getLength() - veh.getPositionOnLane()) + veh.getVehicleType().getLength();
    //                 queueing_length = MAX2(veh_back_to_lane_end, queueing_length);
    //             }
    //
    //             //Experimental
    //             if (veh.getSpeed() < (threshold) && (veh.getPositionOnLane() > (veh.getLane()->getLength()) * 0.25)) {
    //                 const double veh_back_to_lane_end = (lane.getLength() - veh.getPositionOnLane()) + veh.getVehicleType().getLength();
    //                 queueing_length2 = MAX2(veh_back_to_lane_end, queueing_length2);
    //                 queueing_count++;
    //             }
    //         }
    //     }
    //
    //     //Output
    //     if (of != nullptr) {
    //         if (queueing_length > 1 || queueing_length2 > 1) {
    //             ensureOpen();
    //             of->openTag("lane").writeAttr("id", lane.getID()).writeAttr("queueing_time", queueing_time).writeAttr("queueing_length", queueing_length);
    //             of->writeAttr("queueing_length_experimental", queueing_length2).closeTag();
    //         }
    //     } else if (queueing_count > 0) {
    //         myEdgeSamples[&lane.getEdge()].push_back(std::make_pair((double)queueing_count, queueing_length2));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn write_lane(of: Ptr<OutputDevice>, lane: &MSLane, threshold: f64, ensureOpen: &Box<dyn Fn()>) {
        // maximum of all vehicle waiting times
        let mut queueing_time = 0.0_f64;
        // back of last stopped vehicle (XXX does not check for continuous queue)
        let mut queueing_length = 0.0_f64;
        // back of last slow vehicle (XXX does not check for continuous queue)
        let mut queueing_length2 = 0.0_f64;
        // number of slow vehicles
        let mut queueing_count: i32 = 0;

        if !lane.empty() {
            for &veh in lane.myVehicles.iter() {
                if !veh.is_on_road() {
                    continue;
                }

                if veh.get_waiting_seconds() > 0.0 {
                    queueing_time = max2(veh.get_waiting_seconds(), queueing_time);
                    let veh_back_to_lane_end = (lane.get_length() - veh.get_position_on_lane()) + veh.get_vehicle_type().get_length();
                    queueing_length = max2(veh_back_to_lane_end, queueing_length);
                }

                //Experimental
                if veh.get_speed() < threshold && veh.get_position_on_lane() > (veh.get_lane().get_length()) * 0.25 {
                    let veh_back_to_lane_end = (lane.get_length() - veh.get_position_on_lane()) + veh.get_vehicle_type().get_length();
                    queueing_length2 = max2(veh_back_to_lane_end, queueing_length2);
                    queueing_count += 1;
                }
            }
        }

        //Output
        if !of.is_null() {
            if queueing_length > 1.0 || queueing_length2 > 1.0 {
                ensureOpen();
                of.m().open_tag("lane").write_attr("id", lane.get_id()).write_attr("queueing_time", &queueing_time).write_attr("queueing_length", &queueing_length);
                of.m().write_attr("queueing_length_experimental", &queueing_length2).close_tag("");
            }
        } else if queueing_count > 0 {
            let key = Ptr::new(lane.get_edge());
            MY_EDGE_SAMPLES.with(|s| s.borrow_mut().entry(key).or_default().push((queueing_count as f64, queueing_length2)));
        }
    }

    /// C++: `void MSQueueExport::writeMesoQueue(OutputDevice *, const MSEdge &, const MESegment &, int, double, double, const std::function<void ()> &)` [static, private]
    /// Checks a single segment queue (meso)
    // void
    // MSQueueExport::writeMesoQueue(OutputDevice* of, const MSEdge& edge, const MESegment& segment, int qIdx, double segmentOffset, double threshold, const std::function<void()>& ensureOpen) {
    //     const std::vector<MEVehicle*>& queue = segment.getQueue(qIdx);
    //     const int queueSize = (int)queue.size();
    //     // maximum of all vehicle waiting times
    //     double queueing_time = 0.0;
    //     // position of the last slow vehicle counted from the head of the queue
    //     int queueing_count = 0;
    //     // distance from the segment end to the back of the last slow vehicle
    //     double queueing_length = 0.0;
    //     double occupancy = 0.0;
    //     const double segLength = segment.getLength();
    //     const double segmentEnd = segmentOffset + segLength;
    //     const double lanesCovered = segment.numQueues() == 1 ? std::round(segment.getCapacity() / segLength) : 1.;
    //     // positions and earliest exit times are interpolated as in MSEdge::getMesoPositions
    //     SUMOTime earliestExitTime = segment.getQueueBlockTime(qIdx);
    //     // conservative exit times use the jam-dependent headway as in MESegment::getMeanSpeed
    //     SUMOTime conservativeExitTime = earliestExitTime;
    //     double prevPos = std::numeric_limits<double>::max();
    //     bool prevQueued = false;
    //     const double now = SIMTIME;
    //     for (int i = 0; i < queueSize; ++i) {
    //         // vehicles are stored in reverse entry order (the queue head is at the back)
    //         const MEVehicle* const veh = queue[queueSize - 1 - i];
    //         const double lengthWithGap = veh->getVehicleType().getLengthWithGap();
    //         occupancy += lengthWithGap;
    //         earliestExitTime = MAX2(earliestExitTime, veh->getEventTime());
    //         conservativeExitTime = MAX2(conservativeExitTime, veh->getEventTime());
    //         double maxPos = segmentEnd;
    //         if (i > 0) {
    //             earliestExitTime += segment.getMinTauWithVehLength(lengthWithGap, veh->getVehicleType().getCarFollowModel().getHeadwayTime());
    //             conservativeExitTime += segment.getTauWithVehLength(qIdx, lengthWithGap, veh->getVehicleType().getCarFollowModel().getHeadwayTime());
    //             maxPos = MIN2(maxPos, prevPos - lengthWithGap / lanesCovered);
    //         }
    //         const double entry = veh->getLastEntryTimeSeconds();
    //         const double travelTime = MAX2(STEPS2TIME(earliestExitTime) - entry, TS);
    //         const double linearPos = MIN2(segmentEnd, segmentOffset + segLength * (now - entry) / travelTime);
    //         // conservative speed of the interpolated position trajectory. Unlike
    //         // getSpeed() this accounts for blocked vehicles further ahead in the queue
    //         const double interpolatedSpeed = segLength / MAX2(STEPS2TIME(conservativeExitTime) - entry, TS);
    //         // a vehicle is queued if it is slow by itself or if it has caught up
    //         // with the clamped position behind a queued vehicle ahead
    //         const bool queued = MIN2(veh->getSpeed(), interpolatedSpeed) < threshold || (prevQueued && linearPos >= maxPos - POSITION_EPS);
    //         if (queued) {
    //             queueing_count = i + 1;
    //             queueing_time = MAX2(veh->getWaitingSeconds(), queueing_time);
    //             if (MSGlobals::gMesoInterpolatePos) {
    //                 queueing_length = MAX2(0., segmentEnd - (veh->getPositionOnLane() - veh->getVehicleType().getLength()));
    //             } else {
    //                 queueing_length = occupancy / lanesCovered;
    //             }
    //         }
    //         prevQueued = queued;
    //         prevPos = MIN2(linearPos, maxPos);
    //     }
    //     if (queueing_count == 0) {
    //         return;
    //     }
    //     if (of != nullptr) {
    //         ensureOpen();
    //         of->openTag("lane").writeAttr("id", edge.getLanes()[qIdx]->getID()).writeAttr("segment", segment.getIndex());
    //         of->writeAttr("queueing_time", queueing_time).writeAttr("queueing_length", queueing_length).writeAttr("queueing_count", queueing_count).closeTag();
    //     } else {
    //         myEdgeSamples[&edge].push_back(std::make_pair((double)queueing_count, queueing_length));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn write_meso_queue(of: Ptr<OutputDevice>, edge: &MSEdge, segment: &MESegment, qIdx: i32, segmentOffset: f64, threshold: f64, ensureOpen: &Box<dyn Fn()>) {
        let queue = segment.get_queue(qIdx);
        let queue_size = queue.len() as i32;
        // maximum of all vehicle waiting times
        let mut queueing_time = 0.0_f64;
        // position of the last slow vehicle counted from the head of the queue
        let mut queueing_count: i32 = 0;
        // distance from the segment end to the back of the last slow vehicle
        let mut queueing_length = 0.0_f64;
        let mut occupancy = 0.0_f64;
        let seg_length = segment.get_length();
        let segment_end = segmentOffset + seg_length;
        let lanes_covered = if segment.num_queues() == 1 { (segment.get_capacity() / seg_length).round() } else { 1. };
        // positions and earliest exit times are interpolated as in MSEdge::getMesoPositions
        let mut earliest_exit_time: SUMOTime = segment.get_queue_block_time(qIdx);
        // conservative exit times use the jam-dependent headway as in MESegment::getMeanSpeed
        let mut conservative_exit_time: SUMOTime = earliest_exit_time;
        let mut prev_pos = f64::MAX;
        let mut prev_queued = false;
        let now = simtime();
        for i in 0..queue_size {
            // vehicles are stored in reverse entry order (the queue head is at the back)
            let veh: Ptr<MEVehicle> = queue[(queue_size - 1 - i) as usize];
            let length_with_gap = veh.get_vehicle_type().get_length_with_gap();
            occupancy += length_with_gap;
            earliest_exit_time = std::cmp::max(earliest_exit_time, veh.get_event_time());
            conservative_exit_time = std::cmp::max(conservative_exit_time, veh.get_event_time());
            let mut max_pos = segment_end;
            if i > 0 {
                let headway = veh.get_vehicle_type().get_car_follow_model().get_headway_time();
                earliest_exit_time += segment.get_min_tau_with_veh_length(length_with_gap, headway);
                conservative_exit_time += segment.get_tau_with_veh_length(qIdx, length_with_gap, headway);
                max_pos = min2(max_pos, prev_pos - length_with_gap / lanes_covered);
            }
            let entry = veh.get_last_entry_time_seconds();
            let travel_time = max2(steps2time(earliest_exit_time) - entry, ts());
            let linear_pos = min2(segment_end, segmentOffset + seg_length * (now - entry) / travel_time);
            // conservative speed of the interpolated position trajectory. Unlike
            // getSpeed() this accounts for blocked vehicles further ahead in the queue
            let interpolated_speed = seg_length / max2(steps2time(conservative_exit_time) - entry, ts());
            // a vehicle is queued if it is slow by itself or if it has caught up
            // with the clamped position behind a queued vehicle ahead
            let queued = min2(veh.get_speed(), interpolated_speed) < threshold || (prev_queued && linear_pos >= max_pos - POSITION_EPS);
            if queued {
                queueing_count = i + 1;
                queueing_time = max2(veh.get_waiting_seconds(), queueing_time);
                if MSGlobals::meso_interpolate_pos() {
                    queueing_length = max2(0., segment_end - (veh.get_position_on_lane() - veh.get_vehicle_type().get_length()));
                } else {
                    queueing_length = occupancy / lanes_covered;
                }
            }
            prev_queued = queued;
            prev_pos = min2(linear_pos, max_pos);
        }
        if queueing_count == 0 {
            return;
        }
        if !of.is_null() {
            ensureOpen();
            of.m().open_tag("lane").write_attr("id", edge.get_lanes()[qIdx as usize].get_id()).write_attr("segment", &segment.get_index());
            of.m().write_attr("queueing_time", &queueing_time).write_attr("queueing_length", &queueing_length).write_attr("queueing_count", &queueing_count).close_tag("");
        } else {
            MY_EDGE_SAMPLES.with(|s| s.borrow_mut().entry(Ptr::new(edge)).or_default().push((queueing_count as f64, queueing_length)));
        }
    }

    /// C++: `void MSQueueExport::writeInterval(OutputDevice &, SUMOTime, SUMOTime)` [static, private]
    /// Writes aggregated per-edge queue length statistics for the given interval
    // void
    // MSQueueExport::writeInterval(OutputDevice& of, SUMOTime begin, SUMOTime end) {
    //     // The interval element is opened lazily so that with --queue-output.skip-empty
    //     // (and, in particular, for columnar formats) no row is emitted for intervals
    //     // without any queue.
    //     bool intervalOpen = false;
    //     auto ensureOpen = [&]() {
    //         if (!intervalOpen) {
    //             of.openTag("interval").writeAttr("begin", time2string(begin)).writeAttr("end", time2string(end));
    //             intervalOpen = true;
    //         }
    //     };
    //     const OptionsCont& oc = OptionsCont::getOptions();
    //     if (!oc.getBool("queue-output.skip-empty")) {
    //         ensureOpen();
    //     }
    //     const double p = oc.getFloat("queue-output.percentile") / 100.;
    //     // iterate in network edge order for deterministic output
    //     for (const MSEdge* const edge : MSNet::getInstance()->getEdgeControl().getEdges()) {
    //         const auto it = myEdgeSamples.find(edge);
    //         if (it == myEdgeSamples.end()) {
    //             continue;
    //         }
    //         std::vector<double> counts;
    //         std::vector<double> lengths;
    //         for (const auto& sample : it->second) {
    //             counts.push_back(sample.first);
    //             lengths.push_back(sample.second);
    //         }
    //         std::sort(counts.begin(), counts.end());
    //         std::sort(lengths.begin(), lengths.end());
    //         ensureOpen();
    //         of.openTag("edge").writeAttr("id", edge->getID()).writeAttr("samples", (int)counts.size());
    //         of.writeAttr("maxQueueLengthInVehicles", counts.back());
    //         of.writeAttr("medianQueueLengthInVehicles", percentile(counts, 0.5));
    //         of.writeAttr("percentileQueueLengthInVehicles", percentile(counts, p));
    //         of.writeAttr("maxQueueLengthInMeters", lengths.back());
    //         of.writeAttr("medianQueueLengthInMeters", percentile(lengths, 0.5));
    //         of.writeAttr("percentileQueueLengthInMeters", percentile(lengths, p));
    //         of.closeTag();
    //     }
    //     if (intervalOpen) {
    //         of.closeTag();
    //     }
    //     myEdgeSamples.clear();
    // }
    #[allow(unused_variables)]
    pub fn write_interval(of: &mut OutputDevice, begin: SUMOTime, end: SUMOTime) {
        // The interval element is opened lazily so that with --queue-output.skip-empty
        // (and, in particular, for columnar formats) no row is emitted for intervals
        // without any queue.
        let of_p = Ptr::from_mut(of);
        let interval_open = Rc::new(Cell::new(false));
        let ensure_open: Box<dyn Fn()> = {
            let interval_open = interval_open.clone();
            Box::new(move || {
                if !interval_open.get() {
                    of_p.m().open_tag("interval").write_attr("begin", &time2string(begin)).write_attr("end", &time2string(end));
                    interval_open.set(true);
                }
            })
        };
        if !get_bool("queue-output.skip-empty") {
            ensure_open();
        }
        let p = get_float("queue-output.percentile") / 100.;
        // iterate in network edge order for deterministic output
        let net = MSNet::get_instance();
        let ec = net.m().get_edge_control();
        for &edge in ec.get_edges() {
            let samples = MY_EDGE_SAMPLES.with(|s| s.borrow().get(&edge).cloned());
            let Some(samples) = samples else {
                continue;
            };
            let mut counts: Vec<f64> = samples.iter().map(|sample| sample.0).collect();
            let mut lengths: Vec<f64> = samples.iter().map(|sample| sample.1).collect();
            sort_samples(&mut counts);
            sort_samples(&mut lengths);
            ensure_open();
            of_p.m().open_tag("edge").write_attr("id", edge.get_id()).write_attr("samples", &(counts.len() as i32));
            of_p.m().write_attr("maxQueueLengthInVehicles", counts.last().unwrap());
            of_p.m().write_attr("medianQueueLengthInVehicles", &MSQueueExport::percentile(&counts, 0.5));
            of_p.m().write_attr("percentileQueueLengthInVehicles", &MSQueueExport::percentile(&counts, p));
            of_p.m().write_attr("maxQueueLengthInMeters", lengths.last().unwrap());
            of_p.m().write_attr("medianQueueLengthInMeters", &MSQueueExport::percentile(&lengths, 0.5));
            of_p.m().write_attr("percentileQueueLengthInMeters", &MSQueueExport::percentile(&lengths, p));
            of_p.m().close_tag("");
        }
        if interval_open.get() {
            of_p.m().close_tag("");
        }
        MY_EDGE_SAMPLES.with(|s| s.borrow_mut().clear());
    }

    /// C++: `double MSQueueExport::percentile(const int &, double)` [static, private]
    /// Returns the given percentile (with linear interpolation) of a sorted sample vector
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn percentile(sorted: &[f64], p: f64) -> f64 {
        debug_assert!(!sorted.is_empty());
        let rank = p * (sorted.len() - 1) as f64;
        let lower = rank as i32;
        if lower + 1 >= sorted.len() as i32 {
            return *sorted.last().unwrap();
        }
        let frac = rank - lower as f64;
        sorted[lower as usize] * (1. - frac) + sorted[(lower + 1) as usize] * frac
    }

}

