//! Port of microsim/output/MSQueueExport.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

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
        todo!("port MSQueueExport::write")
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
        todo!("port MSQueueExport::finish")
    }

    /// C++: `void MSQueueExport::MSQueueExport(const MSQueueExport &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new(arg0: &MSQueueExport) -> Self {
        todo!("port MSQueueExport::MSQueueExport")
    }

    /// C++: `MSQueueExport & MSQueueExport::operator=(const MSQueueExport &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSQueueExport) -> &mut MSQueueExport {
        todo!("port MSQueueExport::operator=")
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
        todo!("port MSQueueExport::writeEdge")
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
        todo!("port MSQueueExport::writeLane")
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
        todo!("port MSQueueExport::writeMesoQueue")
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
        todo!("port MSQueueExport::writeInterval")
    }

    /// C++: `double MSQueueExport::percentile(const int &, double)` [static, private]
    /// Returns the given percentile (with linear interpolation) of a sorted sample vector
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn percentile(sorted: &i32, p: f64) -> f64 {
        todo!("port MSQueueExport::percentile")
    }

}

