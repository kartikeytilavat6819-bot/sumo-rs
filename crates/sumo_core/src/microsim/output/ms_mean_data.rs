//! Port of microsim/output/MSMeanData.{h,cpp}
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use super::ms_detector_file_output::{DEFAULT_MSDETECTOR_FILE_OUTPUT_VT, MSDetectorFileOutputVt};
use crate::microsim::cfmodels::mscf_model::{MSCFModel, g_semi_implicit_euler_update};
use utils::common::sumo_vehicle_class::SVC_IGNORING;

fn g_use_meso_sim() -> bool {
    crate::microsim::ms_globals::use_meso_sim()
}
fn g_using_internal_lanes() -> bool {
    crate::microsim::ms_globals::using_internal_lanes()
}
#[cfg(test)]
mod msglobals_stub_tests {
    // Stale-stub guard: these files must read MSGlobals (ms_globals.rs), not hardcode its defaults.
    #[test]
    fn no_hardcoded_msglobals_stand_ins() {
        let sources = [
            ("ms_edge.rs", include_str!("../ms_edge.rs")),
            ("ms_lane.rs", include_str!("../ms_lane.rs")),
            ("ms_link.rs", include_str!("../ms_link.rs")),
            ("ms_leader_info.rs", include_str!("../ms_leader_info.rs")),
            ("cfmodels/mscf_model.rs", include_str!("../cfmodels/mscf_model.rs")),
        ];
        let stand_ins = [
            "fn g_lateral_resolution() -> f64 {\n    -1.",
            "fn g_use_meso_sim() -> bool {\n    false",
            "fn g_using_internal_lanes() -> bool {\n    true",
            "fn g_compute_lc() -> bool {\n    false",
            "fn g_semi_implicit_euler_update() -> bool {\n    true",
            "const G_",
        ];
        for (name, src) in sources {
            let src = src.replace("\r\n", "\n");
            for pat in stand_ins {
                assert!(!src.contains(pat), "{name} still hardcodes an MSGlobals value: {pat:?}");
            }
        }
    }
}

// MIN2/MAX2/MIN4 from StdDefs.h
fn min2(a: f64, b: f64) -> f64 {
    if a < b { a } else { b }
}
fn max2(a: f64, b: f64) -> f64 {
    if a > b { a } else { b }
}
fn min4(a: f64, b: f64, c: f64, d: f64) -> f64 {
    min2(min2(a, b), min2(c, d))
}
// shortcut: C++ `delete` of value collectors and sub-data is not done (leaked). A base-typed Ptr cannot free
// the subclass allocation it points into. Upgrade with owned storage per collector.
/// C++ `new T(...)`: heap allocation owned by the simulation, referenced through Ptr.
fn heap<T>(value: T) -> Ptr<T> {
    Ptr::from_raw(Box::into_raw(Box::new(value)))
}
/// C++ implicit upcast `MeanDataValues*` from a tracker pointer (base is the first field, repr(C)).
fn upcast_tracker(t: Ptr<MSMeanData_MeanDataValueTracker>) -> Ptr<MSMeanData_MeanDataValues> {
    Ptr::from_mut(&mut t.m().base)
}
/// C++ implicit upcast `MSMoveReminder*` from `MeanDataValues*`.
fn reminder_of(v: Ptr<MSMeanData_MeanDataValues>) -> Ptr<MSMoveReminder> {
    Ptr::from_mut(&mut v.m().base)
}
/// C++ `(MeanDataValueTracker*)values` downcast; valid because the tracker's base is its first field (repr(C)).
fn as_tracker(v: Ptr<MSMeanData_MeanDataValues>) -> Ptr<MSMeanData_MeanDataValueTracker> {
    Ptr::from_raw(v.as_raw() as *mut MSMeanData_MeanDataValueTracker)
}

fn ms_mean_data_detector_write_xml_output(d: &mut MSDetectorFileOutput, dev: &mut OutputDevice, startTime: SUMOTime, stopTime: SUMOTime) {
    Ptr::from_mut(d).cast::<MSMeanData>().m().write_xml_output(dev, startTime, stopTime)
}
fn ms_mean_data_detector_write_xml_detector_prolog(d: &MSDetectorFileOutput, dev: &mut OutputDevice) {
    Ptr::new(d).cast::<MSMeanData>().m().write_xml_detector_prolog(dev)
}
fn ms_mean_data_detector_detector_update(d: &mut MSDetectorFileOutput, step: SUMOTime) {
    Ptr::from_mut(d).cast::<MSMeanData>().m().detector_update(step)
}

/// Virtual table of MSMeanData as MSDetectorFileOutput (inherited by all MSMeanData subclasses)
static MS_MEAN_DATA_DETECTOR_FILE_OUTPUT_VT: MSDetectorFileOutputVt = MSDetectorFileOutputVt {
    write_xml_output: ms_mean_data_detector_write_xml_output,
    write_xml_detector_prolog: ms_mean_data_detector_write_xml_detector_prolog,
    detector_update: ms_mean_data_detector_detector_update,
    ..DEFAULT_MSDETECTOR_FILE_OUTPUT_VT
};

/// Data collector for edges/lanes
/// C++ class `MSMeanData` (microsim/output/MSMeanData.h)
#[repr(C)]
pub struct MSMeanData {
    /// C++ base class `MSDetectorFileOutput`
    pub base: MSDetectorFileOutput,
    /// the minimum sample seconds
    pub myMinSamples: f64,
    /// the maximum travel time to write
    pub myMaxTravelTime: f64,
    /// Value collectors; sorted by edge, then by lane
    pub myMeasures: Vec<Vec<Ptr<MSMeanData_MeanDataValues>>>,
    /// Whether empty lanes/edges shall be written
    pub myDumpEmpty: bool,
    /// Information whether the output shall be edge-based (not lane-based)
    pub myAmEdgeBased: bool,
    /// The first and the last time step to write information (-1 indicates always)
    pub myDumpBegin: SUMOTime,
    /// The first and the last time step to write information (-1 indicates always)
    pub myDumpEnd: SUMOTime,
    /// time at which init was called();
    pub myInitTime: SUMOTime,
    /// time at which reset() was called
    pub myLastResetTime: SUMOTime,
    /// The corresponding first edges
    pub myEdges: MSEdgeVector,
    pub myTAZ: ConstMSEdgeVector,
    /// The index in myEdges / myMeasures
    pub myEdgeIndex: BTreeMap<Ptr<MSEdge>, i32>,
    /// Whether empty lanes/edges shall be written with default values
    pub myPrintDefaults: bool,
    /// Whether only empty lanes/edges which have been modified shall be written
    pub myPrintModified: bool,
    /// Whether internal lanes/edges shall be written
    pub myDumpInternal: bool,
    /// Whether vehicles are tracked
    pub myTrackVehicles: bool,
    /// bit mask for checking attributes to be written
    pub myWrittenAttributes: SumoXMLAttrMask,
    /// whether the data for all edges shall be aggregated
    pub myAggregate: AggregateType,
    /// more data collectors which write to the same output
    pub mySubData: Ptr<MSMeanData>,
    /// The intervals for which output still has to be generated (only in the tracking case)
    pub myPendingIntervals: VecDeque<(SUMOTime, SUMOTime)>,
    /// Virtual dispatch table for the overridable methods; each subclass constructor installs its own
    pub meandata_vt: &'static MSMeanDataVt,
}

impl std::ops::Deref for MSMeanData {
    type Target = MSDetectorFileOutput;
    fn deref(&self) -> &MSDetectorFileOutput {
        &self.base
    }
}
impl std::ops::DerefMut for MSMeanData {
    fn deref_mut(&mut self) -> &mut MSDetectorFileOutput {
        &mut self.base
    }
}

impl MSMeanData {
    /// C++: `void MSMeanData::MSMeanData(const std::string &, const SUMOTime, const SUMOTime, const bool, const std::string &, const bool, const bool, const int, const double, const double, const std::string &, const std::string &, const std::vector<MSEdge *> &, AggregateType)`
    /// Constructor
    // MSMeanData::MSMeanData(const std::string& id,
    //                        const SUMOTime dumpBegin, const SUMOTime dumpEnd,
    //                        const bool useLanes, const std::string& excludeEmpty, const bool withInternal,
    //                        const bool trackVehicles,
    //                        const int detectPersons,
    //                        const double maxTravelTime,
    //                        const double minSamples,
    //                        const std::string& vTypes,
    //                        const std::string& writeAttributes,
    //                        const std::vector<MSEdge*>& edges,
    //                        AggregateType aggregate) :
    //     MSDetectorFileOutput(id, vTypes, "", detectPersons),
    //     myMinSamples(minSamples),
    //     myMaxTravelTime(maxTravelTime),
    //     myAmEdgeBased(!useLanes),
    //     myDumpBegin(dumpBegin),
    //     myDumpEnd(dumpEnd),
    //     myInitTime(SUMOTime_MAX),
    //     myLastResetTime(dumpBegin),
    //     myEdges(edges),
    //     myDumpInternal(withInternal && MSGlobals::gUsingInternalLanes),
    //     myTrackVehicles(trackVehicles),
    //     myWrittenAttributes(OutputDevice::parseWrittenAttributes(StringTokenizer(writeAttributes).getVector(), "meandata '" + id + "'")),
    //     myAggregate(aggregate) {
    //     try {
    //         myDumpEmpty = !StringUtils::toBool(excludeEmpty);
    //     } catch (const BoolFormatException&) {
    //         if (excludeEmpty == "default" || excludeEmpty == "defaults") {
    //             myPrintDefaults = true;
    //         } else if (excludeEmpty == "modified") {
    //             myDumpEmpty = false;
    //             myPrintDefaults = true;
    //             myPrintModified = true;
    //         } else {
    //             throw;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn new(id: &str, dumpBegin: SUMOTime, dumpEnd: SUMOTime, useLanes: bool, excludeEmpty: &str, withInternal: bool, trackVehicles: bool, detectPersons: i32, maxTravelTime: f64, minSamples: f64, vTypes: &str, writeAttributes: &str, edges: &Vec<Ptr<MSEdge>>, aggregate: AggregateType) -> Self {
        let attrs = OutputDevice::parse_written_attributes(&StringTokenizer::new(writeAttributes).get_vector(), &format!("meandata '{}'", id), &HashMap::new());
        let mut this = MSMeanData {
            base: MSDetectorFileOutput::new(id, vTypes, "", detectPersons).unwrap(),
            myMinSamples: minSamples,
            myMaxTravelTime: maxTravelTime,
            myMeasures: Vec::new(),
            myDumpEmpty: false,
            myAmEdgeBased: !useLanes,
            myDumpBegin: dumpBegin,
            myDumpEnd: dumpEnd,
            myInitTime: SUMOTIME_MAX,
            myLastResetTime: dumpBegin,
            myEdges: edges.clone(),
            myTAZ: Vec::new(),
            myEdgeIndex: BTreeMap::new(),
            myPrintDefaults: false,
            myPrintModified: false,
            myDumpInternal: withInternal && g_using_internal_lanes(),
            myTrackVehicles: trackVehicles,
            myWrittenAttributes: attrs,
            myAggregate: aggregate,
            mySubData: Ptr::null(),
            myPendingIntervals: VecDeque::new(),
            meandata_vt: &MS_MEAN_DATA_BASE_VT,
        };
        this.base.vt = &MS_MEAN_DATA_DETECTOR_FILE_OUTPUT_VT;
        match utils::common::string_utils::to_bool(excludeEmpty) {
            Ok(b) => this.myDumpEmpty = !b,
            Err(e @ ProcessError::BoolFormat(_)) => {
                if excludeEmpty == "default" || excludeEmpty == "defaults" {
                    this.myPrintDefaults = true;
                } else if excludeEmpty == "modified" {
                    this.myDumpEmpty = false;
                    this.myPrintDefaults = true;
                    this.myPrintModified = true;
                } else {
                    // shortcut: C++ rethrows here; a constructor cannot return Err, so this panics with the same message
                    panic!("{}", e);
                }
            }
            // shortcut: C++ propagates non-bool exceptions out of the constructor; panics here instead
            Err(e) => panic!("{}", e),
        }
        this
    }

    /// C++: `void MSMeanData::~MSMeanData() noexcept` [virtual]
    /// Destructor
    // MSMeanData::~MSMeanData() {
    //     for (const std::vector<MeanDataValues*>& valueList : myMeasures) {
    //         for (MeanDataValues* const values : valueList) {
    //             delete values;
    //         }
    //     }
    //     delete mySubData;
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // shortcut: C++ deletes every value collector and mySubData here; they are leaked (see file header)
    }

    /// C++: `void MSMeanData::init()`
    /// Adds the value collectors to all relevant edges.
    // void
    // MSMeanData::init() {
    //     myInitTime = MSNet::getInstance()->getCurrentTimeStep();
    //     if (myEdges.empty()) {
    //         // use all edges by default
    //         for (MSEdge* const edge : MSNet::getInstance()->getEdgeControl().getEdges()) {
    //             if ((myDumpInternal || !edge->isInternal()) &&
    //                     ((detectsPersons() && myDumpInternal) || (!edge->isCrossing() && !edge->isWalkingArea()))) {
    //                 myEdges.push_back(edge);
    //             }
    //         }
    //     }
    //     int index = 0;
    //     for (MSEdge* edge : myEdges) {
    //         myMeasures.push_back(std::vector<MeanDataValues*>());
    //         myEdgeIndex[edge] = index++;
    //         const std::vector<MSLane*>& lanes = edge->getLanes();
    //         if (MSGlobals::gUseMesoSim) {
    //             MeanDataValues* data;
    //             if (!myAmEdgeBased) {
    //                 for (MSLane* const lane : lanes) {
    //                     data = createValues(lane, lanes[0]->getLength(), false);
    //                     myMeasures.back().push_back(data);
    //                     MESegment* s = MSGlobals::gMesoNet->getSegmentForEdge(*edge);
    //                     while (s != nullptr) {
    //                         s->addDetector(data, lane->getIndex());
    //                         s->prepareDetectorForWriting(*data, lane->getIndex());
    //                         s = s->getNextSegment();
    //                     }
    //                     data->reset();
    //                     data->reset(true);
    //                 }
    //             } else {
    //                 if (myTrackVehicles) {
    //                     data = new MeanDataValueTracker(nullptr, lanes[0]->getLength(), this);
    //                 } else {
    //                     data = createValues(nullptr, lanes[0]->getLength(), false);
    //                 }
    //                 data->setDescription("meandata_" + getID() + "|" + edge->getID());
    //                 myMeasures.back().push_back(data);
    //                 MESegment* s = MSGlobals::gMesoNet->getSegmentForEdge(*edge);
    //                 while (s != nullptr) {
    //                     s->addDetector(data);
    //                     s->prepareDetectorForWriting(*data);
    //                     s = s->getNextSegment();
    //                 }
    //                 data->reset();
    //                 data->reset(true);
    //             }
    //             continue;
    //         }
    //         if (myAmEdgeBased && myTrackVehicles) {
    //             myMeasures.back().push_back(new MeanDataValueTracker(nullptr, lanes[0]->getLength(), this));
    //         }
    //         for (MSLane* const lane : lanes) {
    //             if (myTrackVehicles) {
    //                 if (myAmEdgeBased) {
    //                     lane->addMoveReminder(myMeasures.back().back());
    //                 } else {
    //                     myMeasures.back().push_back(new MeanDataValueTracker(lane, lane->getLength(), this));
    //                 }
    //             } else {
    //                 myMeasures.back().push_back(createValues(lane, lane->getLength(), true));
    //             }
    //         }
    //     }
    //     if (myAggregate == AggregateType::TAZ) {
    //         for (const MSEdge* e : MSEdge::getAllEdges()) {
    //             if (e->isTazConnector()) {
    //                 myTAZ.push_back(e);
    //             }
    //         }
    //     }
    //     if (mySubData != nullptr) {
    //         mySubData->init();
    //         std::vector<std::vector<MeanDataValues*> >::const_iterator subMeasureList = mySubData->myMeasures.begin();
    //         for (const std::vector<MeanDataValues*>& valueList : myMeasures) {
    //             std::vector<MeanDataValues*>::const_iterator subMeasures = subMeasureList->begin();
    //             for (MeanDataValues* values : valueList) {
    //                 values->setSubData(*subMeasures);
    //                 subMeasures++;
    //             }
    //             subMeasureList++;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn init(&mut self) {
        self.myInitTime = MSNet::get_instance().get_current_time_step();
        if self.myEdges.is_empty() {
            // use all edges by default
            let net = MSNet::get_instance();
            let all_edges = net.m().get_edge_control().get_edges().clone();
            for edge in all_edges {
                if (self.myDumpInternal || !edge.is_internal()) && ((self.detects_persons() && self.myDumpInternal) || (!edge.is_crossing() && !edge.is_walking_area())) {
                    self.myEdges.push(edge);
                }
            }
        }
        let mut index = 0;
        for edge in self.myEdges.clone() {
            self.myMeasures.push(Vec::new());
            self.myEdgeIndex.insert(edge, index);
            index += 1;
            let lanes: Vec<Ptr<MSLane>> = edge.get_lanes().clone();
            if crate::microsim::ms_globals::use_meso_sim() {
                if !self.myAmEdgeBased {
                    for lane in lanes.iter().copied() {
                        let data = self.create_values(lane, lanes[0].get_length(), false);
                        self.myMeasures.last_mut().unwrap().push(data);
                        let mut s = crate::microsim::ms_globals::meso_net().m().get_segment_for_edge(edge.m(), 0.);
                        while !s.is_null() {
                            s.m().add_detector(reminder_of(data), lane.get_index());
                            s.m().prepare_detector_for_writing(reminder_of(data).m(), lane.get_index());
                            s = s.m().get_next_segment();
                        }
                        data.m().reset(false);
                        data.m().reset(true);
                    }
                } else {
                    let data = if self.myTrackVehicles {
                        upcast_tracker(heap(MSMeanData_MeanDataValueTracker::new(Ptr::null(), lanes[0].get_length(), Ptr::from_mut(&mut *self))))
                    } else {
                        self.create_values(Ptr::null(), lanes[0].get_length(), false)
                    };
                    reminder_of(data).m().set_description(&format!("meandata_{}|{}", self.get_id(), edge.get_id()));
                    self.myMeasures.last_mut().unwrap().push(data);
                    let mut s = crate::microsim::ms_globals::meso_net().m().get_segment_for_edge(edge.m(), 0.);
                    while !s.is_null() {
                        s.m().add_detector(reminder_of(data), -1);
                        s.m().prepare_detector_for_writing(reminder_of(data).m(), -1);
                        s = s.m().get_next_segment();
                    }
                    data.m().reset(false);
                    data.m().reset(true);
                }
                continue;
            }
            if self.myAmEdgeBased && self.myTrackVehicles {
                let t = heap(MSMeanData_MeanDataValueTracker::new(Ptr::null(), lanes[0].get_length(), Ptr::from_mut(&mut *self)));
                self.myMeasures.last_mut().unwrap().push(upcast_tracker(t));
            }
            for lane in lanes.iter().copied() {
                if self.myTrackVehicles {
                    if self.myAmEdgeBased {
                        let rem = reminder_of(*self.myMeasures.last().unwrap().last().unwrap());
                        lane.m().add_move_reminder(rem, true);
                    } else {
                        let t = heap(MSMeanData_MeanDataValueTracker::new(lane, lane.get_length(), Ptr::from_mut(&mut *self)));
                        register_values(upcast_tracker(t), lane, true);
                        self.myMeasures.last_mut().unwrap().push(upcast_tracker(t));
                    }
                } else {
                    let v = self.create_values(lane, lane.get_length(), true);
                    self.myMeasures.last_mut().unwrap().push(v);
                }
            }
        }
        if self.myAggregate == AggregateType::TAZ {
            for &e in MSEdge::get_all_edges().iter() {
                if e.is_taz_connector() {
                    self.myTAZ.push(e);
                }
            }
        }
        if !self.mySubData.is_null() {
            let sub = self.mySubData;
            sub.m().init();
            for (i, valueList) in self.myMeasures.iter().enumerate() {
                for (j, values) in valueList.iter().enumerate() {
                    values.m().set_sub_data(sub.myMeasures[i][j]);
                }
            }
        }
    }

    /// C++: `void MSMeanData::writeXMLOutput(OutputDevice &, SUMOTime, SUMOTime)` [virtual]
    /// Writes collected values into the given stream
    // void
    // MSMeanData::writeXMLOutput(OutputDevice& dev,
    //                            SUMOTime startTime, SUMOTime stopTime) {
    //     // check whether this dump shall be written for the current time
    //     myLastResetTime = SIMSTEP;
    //     int numReady = myDumpBegin < stopTime && myDumpEnd - DELTA_T >= startTime ? 1 : 0;
    //     if (myTrackVehicles && myDumpBegin < stopTime) {
    //         myPendingIntervals.push_back(std::make_pair(startTime, stopTime));
    //         numReady = (int)myPendingIntervals.size();
    //         for (std::vector<std::vector<MeanDataValues*> >::const_iterator i = myMeasures.begin(); i != myMeasures.end(); ++i) {
    //             for (std::vector<MeanDataValues*>::const_iterator j = (*i).begin(); j != (*i).end(); ++j) {
    //                 numReady = MIN2(numReady, ((MeanDataValueTracker*)*j)->getNumReady());
    //                 if (numReady == 0) {
    //                     break;
    //                 }
    //             }
    //             if (numReady == 0) {
    //                 break;
    //             }
    //         }
    //     }
    //     const bool partialInterval = startTime < myInitTime;
    //     if (numReady == 0 || myTrackVehicles || partialInterval) {
    //         resetOnly(stopTime);
    //     }
    //     if (partialInterval) {
    //         return;
    //     }
    //     while (numReady-- > 0) {
    //         if (!myPendingIntervals.empty()) {
    //             startTime = myPendingIntervals.front().first;
    //             stopTime = myPendingIntervals.front().second;
    //             myPendingIntervals.pop_front();
    //         }
    //         openInterval(dev, startTime, stopTime);
    //         if (myAggregate == AggregateType::YES) {
    //             writeAggregated(dev, startTime, stopTime);
    //         } else if (myAggregate == AggregateType::TAZ) {
    //             writeAggregatedTAZ(dev, startTime, stopTime);
    //         } else {
    //             MSEdgeVector::const_iterator edge = myEdges.begin();
    //             for (const std::vector<MeanDataValues*>& measures : myMeasures) {
    //                 writeEdge(dev, measures, *edge, startTime, stopTime);
    //                 ++edge;
    //             }
    //         }
    //         dev.closeTag();
    //     }
    //     dev.flush();
    // }
    #[allow(unused_variables)]
    pub fn write_xml_output(&mut self, dev: &mut OutputDevice, mut startTime: SUMOTime, mut stopTime: SUMOTime) {
        // check whether this dump shall be written for the current time
        self.myLastResetTime = MSNet::get_instance().get_current_time_step();
        let mut numReady: i32 = if self.myDumpBegin < stopTime && self.myDumpEnd - delta_t() >= startTime { 1 } else { 0 };
        if self.myTrackVehicles && self.myDumpBegin < stopTime {
            self.myPendingIntervals.push_back((startTime, stopTime));
            numReady = self.myPendingIntervals.len() as i32;
            for list in self.myMeasures.iter() {
                for values in list.iter() {
                    numReady = std::cmp::min(numReady, as_tracker(*values).get_num_ready());
                    if numReady == 0 {
                        break;
                    }
                }
                if numReady == 0 {
                    break;
                }
            }
        }
        let partialInterval = startTime < self.myInitTime;
        if numReady == 0 || self.myTrackVehicles || partialInterval {
            self.reset_only(stopTime);
        }
        if partialInterval {
            return;
        }
        while numReady > 0 {
            numReady -= 1;
            if !self.myPendingIntervals.is_empty() {
                let (s, e) = *self.myPendingIntervals.front().unwrap();
                startTime = s;
                stopTime = e;
                self.myPendingIntervals.pop_front();
            }
            self.open_interval(dev, startTime, stopTime);
            if self.myAggregate == AggregateType::YES {
                self.write_aggregated(dev, startTime, stopTime);
            } else if self.myAggregate == AggregateType::TAZ {
                self.write_aggregated_taz(dev, startTime, stopTime);
            } else {
                for (i, measures) in self.myMeasures.clone().iter().enumerate() {
                    let edge = self.myEdges[i];
                    self.write_edge(dev, measures, edge, startTime, stopTime);
                }
            }
            dev.close_tag("");
        }
        dev.flush();
    }

    /// C++: `void MSMeanData::writeXMLDetectorProlog(OutputDevice &) const` [const, virtual]
    /// Opens the XML-output using "netstats" as root element
    // void
    // MSMeanData::writeXMLDetectorProlog(OutputDevice& dev) const {
    //     dev.writeXMLHeader("meandata", "meandata_file.xsd");
    // }
    #[allow(unused_variables)]
    pub fn write_xml_detector_prolog_base(&self, dev: &mut OutputDevice) {
        dev.write_xml_header("meandata", "meandata_file.xsd", BTreeMap::new(), true);
    }

    /// C++: `void MSMeanData::detectorUpdate(const SUMOTime)` [virtual]
    /// Updates the detector
    // void
    // MSMeanData::detectorUpdate(const SUMOTime step) {
    //     if (step + DELTA_T == myDumpBegin) {
    //         init();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn detector_update(&mut self, step: SUMOTime) {
        if step + delta_t() == self.myDumpBegin {
            self.init();
        }
    }

    /// C++: `double MSMeanData::getMinSamples() const` [const]
    // double getMinSamples() const {
    //         return myMinSamples;
    //     }
    #[allow(unused_variables)]
    pub fn get_min_samples(&self) -> f64 {
        self.myMinSamples
    }

    /// C++: `double MSMeanData::getMaxTravelTime() const` [const]
    // double getMaxTravelTime() const {
    //         return myMaxTravelTime;
    //     }
    #[allow(unused_variables)]
    pub fn get_max_travel_time(&self) -> f64 {
        self.myMaxTravelTime
    }

    /// C++: `bool MSMeanData::isEdgeData() const` [const]
    // bool isEdgeData() const {
    //         return myAmEdgeBased;
    //     }
    #[allow(unused_variables)]
    pub fn is_edge_data(&self) -> bool {
        self.myAmEdgeBased
    }

    /// C++: `std::vector<std::string> MSMeanData::getAttributeNames() const` [const, virtual, pure virtual]
    /// return all attributes that are (potentially) written by this output
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_attribute_names_base(&self) -> Vec<String> {
        // shortcut: pure virtual in C++; the subclass table supplies it, so this panics like a pure-virtual call.
        panic!("MSMeanData::getAttributeNames is pure virtual; the subclass must implement it")
    }

    /// C++: `double MSMeanData::getAttributeValue(const MSLane *, SumoXMLAttr, double) const` [const]
    /// return attribute value for the given lane
    // double
    // MSMeanData::getAttributeValue(const MSLane* lane, SumoXMLAttr a, double defaultValue) const {
    //     double result = defaultValue;
    //     const std::vector<MeanDataValues*>* edgeValues = getEdgeValues(&lane->getEdge());
    //     if (edgeValues == nullptr) {
    //         return result;
    //     }
    //     MeanDataValues* values = nullptr;
    //     if (!myAmEdgeBased) {
    //         values = (*edgeValues)[lane->getIndex()];
    //     } else {
    //         MeanDataValues* sumData = createValuesSubData(nullptr, lane->getLength(), false);
    //         for (MeanDataValues* meanData : (*edgeValues)) {
    //             meanData->addTo(*sumData);
    //         }
    //         values = sumData;
    //     }
    //     const SUMOTime period = SIMSTEP - myLastResetTime;
    //     result = values->getAttributeValue(a, period, lane->getEdge().getNumLanes(), lane->getSpeedLimit());
    //     if (myAmEdgeBased) {
    //         delete values;
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_attribute_value(&self, lane: Ptr<MSLane>, a: SumoXMLAttr, defaultValue: f64) -> f64 {
        let mut result = defaultValue;
        let edgeValues = self.get_edge_values(Ptr::from_mut(lane.get_edge()));
        if edgeValues.is_null() {
            return result;
        }
        let edgeValues = edgeValues.get().unwrap();
        let values: Ptr<MSMeanData_MeanDataValues>;
        if !self.myAmEdgeBased {
            values = edgeValues[lane.get_index() as usize];
        } else {
            let sumData = self.create_values_sub_data(Ptr::null(), lane.get_length(), false);
            for meanData in edgeValues.iter() {
                meanData.add_to(sumData.m());
            }
            values = sumData;
        }
        let period = MSNet::get_instance().get_current_time_step() - self.myLastResetTime;
        result = values.get_attribute_value(a, period, lane.get_edge().get_num_lanes() as f64, lane.get_speed_limit());
        result
    }

    /// C++: `const std::vector<MSMoveReminder *> MSMeanData::getReminders() const` [const]
    /// retrieve all MeanDataValues
    // const std::vector<MSMoveReminder*>
    // MSMeanData::getReminders() const {
    //     std::vector<MSMoveReminder*> result;
    //     for (auto vec : myMeasures) {
    //         result.insert(result.end(), vec.begin(), vec.end());
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_reminders(&self) -> Vec<Ptr<MSMoveReminder>> {
        let mut result = Vec::new();
        for vec in self.myMeasures.iter() {
            for v in vec.iter() {
                result.push(reminder_of(*v));
            }
        }
        result
    }

    /// C++: `void MSMeanData::setSubData(MSMeanData *)`
    /// set a sub collector
    // void setSubData(MSMeanData* subData) {
    //         mySubData = subData;
    //     }
    #[allow(unused_variables)]
    pub fn set_sub_data(&mut self, subData: Ptr<MSMeanData>) {
        self.mySubData = subData;
    }

    /// C++: `const MSEdgeVector & MSMeanData::getEdges() const` [const]
    // const MSEdgeVector& getEdges() const {
    //         return myEdges;
    //     }
    #[allow(unused_variables)]
    pub fn get_edges(&self) -> &MSEdgeVector {
        &self.myEdges
    }

    /// C++: `MSMeanData::MeanDataValues * MSMeanData::createValues(MSLane *const, const double, const bool) const` [const, virtual, pure virtual, protected]
    /// Create an instance of MeanDataValues
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn create_values_base(&self, lane: Ptr<MSLane>, length: f64, doAdd: bool) -> Ptr<MSMeanData_MeanDataValues> {
        // shortcut: pure virtual in C++ (each output type creates its own MeanDataValues subclass); the subclass table supplies it, so this panics like a pure-virtual call.
        panic!("MSMeanData::createValues is pure virtual; the subclass must implement it")
    }

    /// C++: `void MSMeanData::resetOnly(SUMOTime)` [protected]
    /// Resets network value in order to allow processing of the next interval
    // void
    // MSMeanData::resetOnly(SUMOTime stopTime) {
    //     UNUSED_PARAMETER(stopTime);
    //     if (MSGlobals::gUseMesoSim) {
    //         MSEdgeVector::iterator edge = myEdges.begin();
    //         for (std::vector<std::vector<MeanDataValues*> >::const_iterator i = myMeasures.begin(); i != myMeasures.end(); ++i, ++edge) {
    //             MESegment* s = MSGlobals::gMesoNet->getSegmentForEdge(**edge);
    //             for (MeanDataValues* data : *i) {
    //                 while (s != nullptr) {
    //                     s->prepareDetectorForWriting(*data);
    //                     s = s->getNextSegment();
    //                 }
    //                 data->reset();
    //             }
    //         }
    //         return;
    //     }
    //     for (std::vector<std::vector<MeanDataValues*> >::const_iterator i = myMeasures.begin(); i != myMeasures.end(); ++i) {
    //         for (std::vector<MeanDataValues*>::const_iterator j = (*i).begin(); j != (*i).end(); ++j) {
    //             (*j)->reset();
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn reset_only_base(&mut self, stopTime: SUMOTime) {
        if crate::microsim::ms_globals::use_meso_sim() {
            for (edge, i) in self.myEdges.iter().zip(self.myMeasures.iter()) {
                // C++ walks the segments once per edge, so only the first value sees them
                let mut s = crate::microsim::ms_globals::meso_net().m().get_segment_for_edge(edge.m(), 0.);
                for data in i.iter() {
                    while !s.is_null() {
                        s.m().prepare_detector_for_writing(reminder_of(*data).m(), -1);
                        s = s.m().get_next_segment();
                    }
                    data.m().reset(false);
                }
            }
            return;
        }
        for i in self.myMeasures.iter() {
            for j in i.iter() {
                j.m().reset(false);
            }
        }
    }

    /// C++: `std::string MSMeanData::getEdgeID(const MSEdge *const)` [virtual, protected]
    /// Return the relevant edge id
    // std::string
    // MSMeanData::getEdgeID(const MSEdge* const edge) {
    //     return edge->getID();
    // }
    #[allow(unused_variables)]
    pub fn get_edge_id_base(&mut self, edge: Ptr<MSEdge>) -> String {
        edge.get_id().to_string()
    }

    /// C++: `void MSMeanData::writeEdge(OutputDevice &, const std::vector<MeanDataValues *> &, const MSEdge *const, SUMOTime, SUMOTime)` [protected]
    /// Writes edge values into the given stream
    // void
    // MSMeanData::writeEdge(OutputDevice& dev,
    //                       const std::vector<MeanDataValues*>& edgeValues,
    //                       const MSEdge* const edge, SUMOTime startTime, SUMOTime stopTime) {
    //     if (MSGlobals::gUseMesoSim) {
    //         int idx = 0;
    //         for (MeanDataValues* const data : edgeValues) {
    //             MESegment* s = MSGlobals::gMesoNet->getSegmentForEdge(*edge);
    //             while (s != nullptr) {
    //                 s->prepareDetectorForWriting(*data, myAmEdgeBased ? -1 : idx);
    //                 s = s->getNextSegment();
    //             }
    //             idx++;
    //         }
    //         if (myAmEdgeBased) {
    //             MeanDataValues* const data = edgeValues.front();
    //             if (myDumpEmpty || (myPrintModified && edge->getLanes()[0]->isSpeedModified()) || !data->isEmpty()) {
    //                 writePrefix(dev, *data, SUMO_TAG_EDGE, getEdgeID(edge));
    //                 for (MeanDataValues* d = data; d != nullptr; d = d->getSubData()) {
    //                     d->write(dev, myWrittenAttributes, stopTime - startTime,
    //                              edge->getNumDrivingLanes(),
    //                              edge->getSpeedLimit(),
    //                              myPrintDefaults ? edge->getLength() / edge->getSpeedLimit() : -1.);
    //                 }
    //                 dev.closeTag();
    //             }
    //             if (!MSNet::getInstance()->skipFinalReset()) {
    //                 data->reset(true);
    //             }
    //             return;
    //         }
    //     }
    //     if (!myAmEdgeBased) {
    //         bool writeCheck = myDumpEmpty;
    //         if (!writeCheck) {
    //             for (const MeanDataValues* const laneData : edgeValues) {
    //                 if (!laneData->isEmpty() || (myPrintModified && laneData->getLane()->isSpeedModified())) {
    //                     writeCheck = true;
    //                     break;
    //                 }
    //             }
    //         }
    //         if (writeCheck) {
    //             dev.openTag(SUMO_TAG_EDGE).writeAttr(SUMO_ATTR_ID, edge->getID());
    //         }
    //         for (MeanDataValues* const laneData : edgeValues) {
    //             const MSLane* const lane = laneData->getLane();
    //             if (myDumpEmpty || (myPrintModified && lane->isSpeedModified()) || !laneData->isEmpty()) {
    //                 writePrefix(dev, *laneData, SUMO_TAG_LANE, lane->getID());
    //                 for (MeanDataValues* data = laneData; data != nullptr; data = data->getSubData()) {
    //                     data->write(dev, myWrittenAttributes, stopTime - startTime, 1, lane->getSpeedLimit(),
    //                                 myPrintDefaults ? lane->getLength() / lane->getSpeedLimit() : -1.);
    //                 }
    //                 dev.closeTag();
    //             }
    //             if (!MSNet::getInstance()->skipFinalReset()) {
    //                 laneData->reset(true);
    //             }
    //         }
    //         if (writeCheck) {
    //             dev.closeTag();
    //         }
    //     } else {
    //         if (myTrackVehicles) {
    //             MeanDataValues* meanData = *edgeValues.begin();
    //             if (myDumpEmpty || !meanData->isEmpty()) {
    //                 writePrefix(dev, *meanData, SUMO_TAG_EDGE, edge->getID());
    //                 for (MeanDataValues* data = meanData; data != nullptr; data = data->getSubData()) {
    //                     data->write(dev, myWrittenAttributes, stopTime - startTime, edge->getNumDrivingLanes(), edge->getSpeedLimit(),
    //                                 myPrintDefaults ? edge->getLength() / edge->getSpeedLimit() : -1.);
    //                 }
    //                 dev.closeTag();
    //             }
    //             if (!MSNet::getInstance()->skipFinalReset()) {
    //                 meanData->reset(true);
    //             }
    //         } else {
    //             MeanDataValues* sumData = createValuesSubData(nullptr, edge->getLength(), false);
    //             bool writeCheck = myDumpEmpty;
    //             for (MeanDataValues* const laneData : edgeValues) {
    //                 laneData->addTo(*sumData);
    //                 if (myPrintModified && laneData->getLane()->isSpeedModified()) {
    //                     writeCheck = true;
    //                 }
    //                 if (!MSNet::getInstance()->skipFinalReset()) {
    //                     laneData->reset();
    //                 }
    //             }
    //             if (writeCheck || !sumData->isEmpty()) {
    //                 writePrefix(dev, *sumData, SUMO_TAG_EDGE, getEdgeID(edge));
    //                 for (MeanDataValues* data = sumData; data != nullptr; data = data->getSubData()) {
    //                     data->write(dev, myWrittenAttributes, stopTime - startTime, edge->getNumDrivingLanes(), edge->getSpeedLimit(),
    //                                 myPrintDefaults ? edge->getLength() / edge->getSpeedLimit() : -1.);
    //                 }
    //                 dev.closeTag();
    //             }
    //             delete sumData;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn write_edge(&mut self, dev: &mut OutputDevice, edgeValues: &Vec<Ptr<MSMeanData_MeanDataValues>>, edge: Ptr<MSEdge>, startTime: SUMOTime, stopTime: SUMOTime) {
        let skip_final_reset = MSNet::get_instance().skip_final_reset();
        if crate::microsim::ms_globals::use_meso_sim() {
            let mut idx: i32 = 0;
            for data in edgeValues.iter() {
                let mut s = crate::microsim::ms_globals::meso_net().m().get_segment_for_edge(edge.m(), 0.);
                while !s.is_null() {
                    s.m().prepare_detector_for_writing(reminder_of(*data).m(), if self.myAmEdgeBased { -1 } else { idx });
                    s = s.m().get_next_segment();
                }
                idx += 1;
            }
            if self.myAmEdgeBased {
                let data = edgeValues[0];
                if self.myDumpEmpty || (self.myPrintModified && edge.get_lanes()[0].is_speed_modified()) || !data.is_empty() {
                    let id = edge.get_id().to_string();
                    self.write_prefix(dev, &data, SUMO_TAG_EDGE, id);
                    let mut d = data;
                    while !d.is_null() {
                        let defaultTravelTime = if self.myPrintDefaults { edge.get_length() / edge.get_speed_limit() } else { -1. };
                        d.write(dev, &self.myWrittenAttributes, stopTime - startTime, edge.get_num_driving_lanes(), edge.get_speed_limit(), defaultTravelTime, -1);
                        d = d.m().get_sub_data();
                    }
                    dev.close_tag("");
                }
                if !skip_final_reset {
                    data.m().reset(true);
                }
                return;
            }
        }
        if !self.myAmEdgeBased {
            let mut writeCheck = self.myDumpEmpty;
            if !writeCheck {
                for laneData in edgeValues.iter() {
                    if !laneData.is_empty() || (self.myPrintModified && laneData.get_lane().is_speed_modified()) {
                        writeCheck = true;
                        break;
                    }
                }
            }
            if writeCheck {
                dev.open_tag_id(SUMO_TAG_EDGE).write_attr(SUMO_ATTR_ID, edge.get_id());
            }
            for laneData in edgeValues.iter() {
                let lane = laneData.get_lane();
                if self.myDumpEmpty || (self.myPrintModified && lane.is_speed_modified()) || !laneData.is_empty() {
                    let id = lane.get_id().to_string();
                    self.write_prefix(dev, laneData, SUMO_TAG_LANE, id);
                    let mut data = *laneData;
                    while !data.is_null() {
                        let defaultTravelTime = if self.myPrintDefaults { lane.get_length() / lane.get_speed_limit() } else { -1. };
                        data.write(dev, &self.myWrittenAttributes, stopTime - startTime, 1, lane.get_speed_limit(), defaultTravelTime, -1);
                        data = data.m().get_sub_data();
                    }
                    dev.close_tag("");
                }
                if !skip_final_reset {
                    laneData.m().reset(true);
                }
            }
            if writeCheck {
                dev.close_tag("");
            }
        } else {
            if self.myTrackVehicles {
                let meanData = edgeValues[0];
                if self.myDumpEmpty || !meanData.is_empty() {
                    let id = edge.get_id().to_string();
                    self.write_prefix(dev, &meanData, SUMO_TAG_EDGE, id);
                    let mut data = meanData;
                    while !data.is_null() {
                        let defaultTravelTime = if self.myPrintDefaults { edge.get_length() / edge.get_speed_limit() } else { -1. };
                        data.write(dev, &self.myWrittenAttributes, stopTime - startTime, edge.get_num_driving_lanes(), edge.get_speed_limit(), defaultTravelTime, -1);
                        data = data.m().get_sub_data();
                    }
                    dev.close_tag("");
                }
                if !skip_final_reset {
                    meanData.m().reset(true);
                }
            } else {
                let sumData = self.create_values_sub_data(Ptr::null(), edge.get_length(), false);
                let mut writeCheck = self.myDumpEmpty;
                for laneData in edgeValues.iter() {
                    laneData.add_to(sumData.m());
                    if self.myPrintModified && laneData.get_lane().is_speed_modified() {
                        writeCheck = true;
                    }
                    if !skip_final_reset {
                        laneData.m().reset(false);
                    }
                }
                if writeCheck || !sumData.is_empty() {
                    let id = self.get_edge_id(edge);
                    self.write_prefix(dev, &sumData, SUMO_TAG_EDGE, id);
                    let mut data = sumData;
                    while !data.is_null() {
                        let defaultTravelTime = if self.myPrintDefaults { edge.get_length() / edge.get_speed_limit() } else { -1. };
                        data.write(dev, &self.myWrittenAttributes, stopTime - startTime, edge.get_num_driving_lanes(), edge.get_speed_limit(), defaultTravelTime, -1);
                        data = data.m().get_sub_data();
                    }
                    dev.close_tag("");
                }
                // C++ `delete sumData`: not freed, see file header shortcut
            }
        }
    }

    /// C++: `void MSMeanData::writeAggregated(OutputDevice &, SUMOTime, SUMOTime)` [protected]
    /// Writes aggregated data of all edge values into the given stream
    // void
    // MSMeanData::writeAggregated(OutputDevice& dev, SUMOTime startTime, SUMOTime stopTime) {
    //     if (myTrackVehicles) {
    //         throw ProcessError(TL("aggregated meanData output not yet implemented for trackVehicles"));
    //     }
    //
    //     double edgeLengthSum = 0;
    //     int laneNumber = 0;
    //     double speedSum = 0;
    //     double totalTT = 0;
    //     for (MSEdge* edge : myEdges) {
    //         edgeLengthSum += edge->getLength();
    //         laneNumber += edge->getNumDrivingLanes();
    //         speedSum += edge->getSpeedLimit();
    //         totalTT += edge->getLength() / edge->getSpeedLimit();
    //     }
    //     MeanDataValues* sumData = createValuesSubData(nullptr, edgeLengthSum, false);
    //     for (const std::vector<MeanDataValues*>& edgeValues : myMeasures) {
    //         for (MeanDataValues* meanData : edgeValues) {
    //             meanData->addTo(*sumData);
    //             if (!MSNet::getInstance()->skipFinalReset()) {
    //                 meanData->reset();
    //             }
    //         }
    //     }
    //     if (MSGlobals::gUseMesoSim) {
    //         for (int i = 0; i < (int)myEdges.size(); i++) {
    //             MSEdge* edge = myEdges[i];
    //             std::vector<MeanDataValues*>& edgeValues = myMeasures[i];
    //             MESegment* s = MSGlobals::gMesoNet->getSegmentForEdge(*edge);
    //             while (s != nullptr) {
    //                 for (MeanDataValues* meanData : edgeValues) {
    //                     s->prepareDetectorForWriting(*meanData);
    //                     meanData->addTo(*sumData);
    //                     if (!MSNet::getInstance()->skipFinalReset()) {
    //                         meanData->reset();
    //                     }
    //                 }
    //                 s = s->getNextSegment();
    //             }
    //         }
    //     }
    //
    //     if (myDumpEmpty || !sumData->isEmpty()) {
    //         writePrefix(dev, *sumData, SUMO_TAG_EDGE, "AGGREGATED");
    //         dev.writeAttr(SUMO_ATTR_NUMEDGES, myEdges.size());
    //         for (MeanDataValues* data = sumData; data != nullptr; data = data->getSubData()) {
    //             data->write(dev, myWrittenAttributes, stopTime - startTime, laneNumber, speedSum / (double)myEdges.size(),
    //                         myPrintDefaults ? totalTT : -1.);
    //         }
    //         dev.closeTag();
    //     }
    //     delete sumData;
    // }
    #[allow(unused_variables)]
    pub fn write_aggregated(&mut self, dev: &mut OutputDevice, startTime: SUMOTime, stopTime: SUMOTime) {
        if self.myTrackVehicles {
            // shortcut: C++ throws ProcessError here; this unit-returning function panics with the same text
            panic!("{}", ProcessError::Process("aggregated meanData output not yet implemented for trackVehicles".to_string()));
        }
        let skip_final_reset = MSNet::get_instance().skip_final_reset();

        let mut edgeLengthSum = 0.0;
        let mut laneNumber: i32 = 0;
        let mut speedSum = 0.0;
        let mut totalTT = 0.0;
        for edge in self.myEdges.clone() {
            edgeLengthSum += edge.get_length();
            laneNumber += edge.get_num_driving_lanes();
            speedSum += edge.get_speed_limit();
            totalTT += edge.get_length() / edge.get_speed_limit();
        }
        let sumData = self.create_values_sub_data(Ptr::null(), edgeLengthSum, false);
        for edgeValues in self.myMeasures.iter() {
            for meanData in edgeValues.iter() {
                meanData.add_to(sumData.m());
                if !skip_final_reset {
                    meanData.m().reset(false);
                }
            }
        }
        if crate::microsim::ms_globals::use_meso_sim() {
            for (edge, edgeValues) in self.myEdges.iter().zip(self.myMeasures.iter()) {
                let mut s = crate::microsim::ms_globals::meso_net().m().get_segment_for_edge(edge.m(), 0.);
                while !s.is_null() {
                    for meanData in edgeValues.iter() {
                        s.m().prepare_detector_for_writing(reminder_of(*meanData).m(), -1);
                        meanData.add_to(sumData.m());
                        if !skip_final_reset {
                            meanData.m().reset(false);
                        }
                    }
                    s = s.m().get_next_segment();
                }
            }
        }

        if self.myDumpEmpty || !sumData.is_empty() {
            self.write_prefix(dev, &sumData, SUMO_TAG_EDGE, "AGGREGATED".to_string());
            dev.write_attr(SUMO_ATTR_NUMEDGES, &self.myEdges.len());
            let mut data = sumData;
            while !data.is_null() {
                let defaultTravelTime = if self.myPrintDefaults { totalTT } else { -1. };
                data.write(dev, &self.myWrittenAttributes, stopTime - startTime, laneNumber, speedSum / (self.myEdges.len() as f64), defaultTravelTime, -1);
                data = data.m().get_sub_data();
            }
            dev.close_tag("");
        }
        // C++ `delete sumData`: not freed, see file header shortcut
    }

    /// C++: `void MSMeanData::writeAggregatedTAZ(OutputDevice &, SUMOTime, SUMOTime)` [protected]
    /// Writes aggregated data for each TAZ into the given stream
    // void
    // MSMeanData::writeAggregatedTAZ(OutputDevice& dev, SUMOTime startTime, SUMOTime stopTime) {
    //     if (myTrackVehicles) {
    //         throw ProcessError(TL("aggregated meanData output not yet implemented for trackVehicles"));
    //     }
    //
    //     for (const MSEdge* taz : myTAZ) {
    //         double edgeLengthSum = 0;
    //         int laneNumber = 0;
    //         double speedSum = 0;
    //         double totalTT = 0;
    //         std::set<const MSEdge*> connected;
    //         for (const MSEdge* edge : taz->getSuccessors()) {
    //             connected.insert(edge);
    //         }
    //         for (const MSEdge* edge : taz->getPredecessors()) {
    //             connected.insert(edge);
    //         }
    //         for (const MSEdge* edge : connected) {
    //             edgeLengthSum += edge->getLength();
    //             laneNumber += edge->getNumDrivingLanes();
    //             speedSum += edge->getSpeedLimit();
    //             totalTT += edge->getLength() / edge->getSpeedLimit();
    //         }
    //         MeanDataValues* sumData = createValuesSubData(nullptr, edgeLengthSum, false);
    //         for (int i = 0; i < (int)myEdges.size(); i++) {
    //             MSEdge* edge = myEdges[i];
    //             if (connected.count(edge) != 0) {
    //                 std::vector<MeanDataValues*>& edgeValues = myMeasures[i];
    //                 if (MSGlobals::gUseMesoSim) {
    //                     MESegment* s = MSGlobals::gMesoNet->getSegmentForEdge(*edge);
    //                     while (s != nullptr) {
    //                         for (MeanDataValues* meanData : edgeValues) {
    //                             s->prepareDetectorForWriting(*meanData);
    //                             meanData->addTo(*sumData);
    //                         }
    //                         s = s->getNextSegment();
    //                     }
    //                 } else {
    //                     for (MeanDataValues* meanData : edgeValues) {
    //                         meanData->addTo(*sumData);
    //                     }
    //                 }
    //             }
    //         }
    //         if (myDumpEmpty || !sumData->isEmpty()) {
    //             writePrefix(dev, *sumData, SUMO_TAG_EDGE, taz->getID());
    //             dev.writeAttr(SUMO_ATTR_NUMEDGES, connected.size());
    //             for (MeanDataValues* data = sumData; data != nullptr; data = data->getSubData()) {
    //                 data->write(dev, myWrittenAttributes, stopTime - startTime, laneNumber, speedSum / (double)connected.size(),
    //                             myPrintDefaults ? totalTT : -1.);
    //             }
    //             dev.closeTag();
    //         }
    //         delete sumData;
    //     }
    //
    //     if (!MSNet::getInstance()->skipFinalReset()) {
    //         for (const std::vector<MeanDataValues*>& edgeValues : myMeasures) {
    //             for (MeanDataValues* meanData : edgeValues) {
    //                 meanData->reset();
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn write_aggregated_taz(&mut self, dev: &mut OutputDevice, startTime: SUMOTime, stopTime: SUMOTime) {
        if self.myTrackVehicles {
            // shortcut: C++ throws ProcessError here; this unit-returning function panics with the same text
            panic!("{}", ProcessError::Process("aggregated meanData output not yet implemented for trackVehicles".to_string()));
        }
        let skip_final_reset = MSNet::get_instance().skip_final_reset();

        for taz in self.myTAZ.clone() {
            let mut edgeLengthSum = 0.0;
            let mut laneNumber: i32 = 0;
            let mut speedSum = 0.0;
            let mut totalTT = 0.0;
            // determinism: std::set<const MSEdge*> iterates by address; BTreeSet over Ptr orders the same way
            let mut connected: BTreeSet<Ptr<MSEdge>> = BTreeSet::new();
            for edge in taz.get_successors(SVC_IGNORING).iter() {
                connected.insert(*edge);
            }
            for edge in taz.get_predecessors().iter() {
                connected.insert(*edge);
            }
            for edge in connected.iter() {
                edgeLengthSum += edge.get_length();
                laneNumber += edge.get_num_driving_lanes();
                speedSum += edge.get_speed_limit();
                totalTT += edge.get_length() / edge.get_speed_limit();
            }
            let sumData = self.create_values_sub_data(Ptr::null(), edgeLengthSum, false);
            for i in 0..self.myEdges.len() {
                let edge = self.myEdges[i];
                if connected.contains(&edge) {
                    let edgeValues = &self.myMeasures[i];
                    if crate::microsim::ms_globals::use_meso_sim() {
                        let mut s = crate::microsim::ms_globals::meso_net().m().get_segment_for_edge(edge.m(), 0.);
                        while !s.is_null() {
                            for meanData in edgeValues.iter() {
                                s.m().prepare_detector_for_writing(reminder_of(*meanData).m(), -1);
                                meanData.add_to(sumData.m());
                            }
                            s = s.m().get_next_segment();
                        }
                    } else {
                        for meanData in edgeValues.iter() {
                            meanData.add_to(sumData.m());
                        }
                    }
                }
            }
            if self.myDumpEmpty || !sumData.is_empty() {
                let id = taz.get_id().to_string();
                self.write_prefix(dev, &sumData, SUMO_TAG_EDGE, id);
                dev.write_attr(SUMO_ATTR_NUMEDGES, &connected.len());
                let mut data = sumData;
                while !data.is_null() {
                    let defaultTravelTime = if self.myPrintDefaults { totalTT } else { -1. };
                    data.write(dev, &self.myWrittenAttributes, stopTime - startTime, laneNumber, speedSum / (connected.len() as f64), defaultTravelTime, -1);
                    data = data.m().get_sub_data();
                }
                dev.close_tag("");
            }
            // C++ `delete sumData`: not freed, see file header shortcut
        }

        if !skip_final_reset {
            for edgeValues in self.myMeasures.iter() {
                for meanData in edgeValues.iter() {
                    meanData.m().reset(false);
                }
            }
        }
    }

    /// C++: `void MSMeanData::openInterval(OutputDevice &, const SUMOTime, const SUMOTime)` [virtual, protected]
    /// Writes the interval opener
    // void
    // MSMeanData::openInterval(OutputDevice& dev, const SUMOTime startTime, const SUMOTime stopTime) {
    //     dev.openTag(SUMO_TAG_INTERVAL).writeAttr(SUMO_ATTR_BEGIN, time2string(startTime)).writeAttr(SUMO_ATTR_END, time2string(stopTime));
    //     dev.writeAttr(SUMO_ATTR_ID, myID);
    // }
    #[allow(unused_variables)]
    pub fn open_interval_base(&mut self, dev: &mut OutputDevice, startTime: SUMOTime, stopTime: SUMOTime) {
        dev.open_tag_id(SUMO_TAG_INTERVAL).write_attr(SUMO_ATTR_BEGIN, &time2string(startTime)).write_attr(SUMO_ATTR_END, &time2string(stopTime));
        dev.write_attr(SUMO_ATTR_ID, self.get_id());
    }

    /// C++: `void MSMeanData::writePrefix(OutputDevice &, const MeanDataValues &, const SumoXMLTag, const std::string) const` [const, virtual, protected]
    /// Writes the surrounding element into the given stream
    // void
    // MSMeanData::writePrefix(OutputDevice& dev, const MeanDataValues& values, const SumoXMLTag tag, const std::string id) const {
    //     dev.openTag(tag);
    //     dev.writeAttr(SUMO_ATTR_ID, id);
    //     dev.writeOptionalAttr(SUMO_ATTR_SAMPLEDSECONDS, values.getSamples(), myWrittenAttributes);
    // }
    #[allow(unused_variables)]
    pub fn write_prefix_base(&self, dev: &mut OutputDevice, values: &MSMeanData_MeanDataValues, tag: SumoXMLTag, id: String) {
        dev.open_tag_id(tag);
        dev.write_attr(SUMO_ATTR_ID, id.as_str());
        dev.write_optional_attr_mask(SUMO_ATTR_SAMPLEDSECONDS, &values.get_samples(), self.myWrittenAttributes, false);
    }

    /// C++: `const std::vector<MeanDataValues *> * MSMeanData::getEdgeValues(const MSEdge *) const` [const, protected]
    // const std::vector<MeanDataValues*>* MSMeanData::getEdgeValues(const MSEdge* edge) const {
    //     auto it = myEdgeIndex.find(edge);
    //     if (it != myEdgeIndex.end()) {
    //         return &myMeasures[it->second];
    //     } else {
    //         return nullptr;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_edge_values(&self, edge: Ptr<MSEdge>) -> Ptr<Vec<Ptr<MSMeanData_MeanDataValues>>> {
        match self.myEdgeIndex.get(&edge) {
            Some(&idx) => Ptr::new(&self.myMeasures[idx as usize]),
            None => Ptr::null(),
        }
    }

    /// C++: `MSMeanData::MeanDataValues * MSMeanData::createValuesSubData(MSLane *const, const double, const bool) const` [const, private]
    /// Create an instance of MeanDataValues with subdata
    // MSMeanData::MeanDataValues*
    // MSMeanData::createValuesSubData(MSLane* const lane, const double length, const bool doAdd) const {
    //     MeanDataValues* values = createValues(lane, length, doAdd);
    //     if (mySubData != nullptr) {
    //         values->setSubData(mySubData->createValues(lane, length, doAdd));
    //     }
    //     return values;
    // }
    #[allow(unused_variables)]
    pub fn create_values_sub_data(&self, lane: Ptr<MSLane>, length: f64, doAdd: bool) -> Ptr<MSMeanData_MeanDataValues> {
        let values = self.create_values(lane, length, doAdd);
        if !self.mySubData.is_null() {
            values.m().set_sub_data(self.mySubData.create_values(lane, length, doAdd));
        }
        values
    }

    /// C++: `void MSMeanData::MSMeanData(const MSMeanData &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSMeanData) -> Self {
        // C++ declares the copy constructor private and never defines it: copying is an error
        panic!("MSMeanData copy constructor is invalidated (private in C++)")
    }

    /// C++: `MSMeanData & MSMeanData::operator=(const MSMeanData &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSMeanData) -> &mut MSMeanData {
        // C++ declares the assignment operator private and never defines it: assignment is an error
        panic!("MSMeanData assignment operator is invalidated (private in C++)")
    }

}

/// Virtual dispatch table for the overridable methods of MSMeanData (C++ virtuals).
/// Each subclass constructor installs its own table in `meandata_vt`; the base table forwards to the `*_base` methods.
pub struct MSMeanDataVt {
    pub create_values: fn(&MSMeanData, Ptr<MSLane>, f64, bool) -> Ptr<MSMeanData_MeanDataValues>,
    pub get_attribute_names: fn(&MSMeanData) -> Vec<String>,
    pub write_xml_detector_prolog: fn(&MSMeanData, &mut OutputDevice),
    pub get_edge_id: fn(&mut MSMeanData, Ptr<MSEdge>) -> String,
    pub open_interval: fn(&mut MSMeanData, &mut OutputDevice, SUMOTime, SUMOTime),
    pub write_prefix: fn(&MSMeanData, &mut OutputDevice, &MSMeanData_MeanDataValues, SumoXMLTag, String),
    pub reset_only: fn(&mut MSMeanData, SUMOTime),
}

fn base_detector_write_xml_output(d: &mut MSDetectorFileOutput, dev: &mut OutputDevice, startTime: SUMOTime, stopTime: SUMOTime) {
    Ptr::from_mut(d).cast::<MSMeanData>().m().write_xml_output(dev, startTime, stopTime)
}

fn base_detector_write_xml_detector_prolog(d: &MSDetectorFileOutput, dev: &mut OutputDevice) {
    Ptr::new(d).cast::<MSMeanData>().m().write_xml_detector_prolog(dev)
}

fn base_detector_detector_update(d: &mut MSDetectorFileOutput, step: SUMOTime) {
    Ptr::from_mut(d).cast::<MSMeanData>().m().detector_update(step)
}

/// Detector-level dispatch of MSMeanData itself; subclasses with their own detectorUpdate/writeXMLOutput install a table over it
static MS_MEAN_DATA_BASE_DETECTOR_VT: MSDetectorFileOutputVt = MSDetectorFileOutputVt {
    write_xml_output: base_detector_write_xml_output,
    write_xml_detector_prolog: base_detector_write_xml_detector_prolog,
    detector_update: base_detector_detector_update,
    ..DEFAULT_MSDETECTOR_FILE_OUTPUT_VT
};

static MS_MEAN_DATA_BASE_VT: MSMeanDataVt = MSMeanDataVt {
    create_values: MSMeanData::create_values_base,
    get_attribute_names: MSMeanData::get_attribute_names_base,
    write_xml_detector_prolog: MSMeanData::write_xml_detector_prolog_base,
    get_edge_id: MSMeanData::get_edge_id_base,
    open_interval: MSMeanData::open_interval_base,
    write_prefix: MSMeanData::write_prefix_base,
    reset_only: MSMeanData::reset_only_base,
};

impl MSMeanData {
    /// C++ virtual `MSMeanData::createValues`: dispatches to the subclass table
    pub fn create_values(&self, lane: Ptr<MSLane>, length: f64, doAdd: bool) -> Ptr<MSMeanData_MeanDataValues> {
        (self.meandata_vt.create_values)(self, lane, length, doAdd)
    }

    /// C++ virtual `MSMeanData::getAttributeNames`: dispatches to the subclass table
    pub fn get_attribute_names(&self) -> Vec<String> {
        (self.meandata_vt.get_attribute_names)(self)
    }

    /// C++ virtual `MSMeanData::writeXMLDetectorProlog`: dispatches to the subclass table
    pub fn write_xml_detector_prolog(&self, dev: &mut OutputDevice) {
        (self.meandata_vt.write_xml_detector_prolog)(self, dev)
    }

    /// C++ virtual `MSMeanData::getEdgeID`: dispatches to the subclass table
    pub fn get_edge_id(&mut self, edge: Ptr<MSEdge>) -> String {
        (self.meandata_vt.get_edge_id)(self, edge)
    }

    /// C++ virtual `MSMeanData::openInterval`: dispatches to the subclass table
    pub fn open_interval(&mut self, dev: &mut OutputDevice, startTime: SUMOTime, stopTime: SUMOTime) {
        (self.meandata_vt.open_interval)(self, dev, startTime, stopTime)
    }

    /// C++ virtual `MSMeanData::writePrefix`: dispatches to the subclass table
    pub fn write_prefix(&self, dev: &mut OutputDevice, values: &MSMeanData_MeanDataValues, tag: SumoXMLTag, id: String) {
        (self.meandata_vt.write_prefix)(self, dev, values, tag, id)
    }

    /// C++ virtual `MSMeanData::resetOnly`: dispatches to the subclass table
    pub fn reset_only(&mut self, stopTime: SUMOTime) {
        (self.meandata_vt.reset_only)(self, stopTime)
    }
}

/// Data structure for mean (aggregated) edge/lane values for tracked vehicles
/// C++ class `MSMeanData::MeanDataValueTracker` (microsim/output/MSMeanData.h)
#[repr(C)] // base must stay at offset 0 for the C++ downcast in as_tracker
pub struct MSMeanData_MeanDataValueTracker {
    /// C++ base class `MeanDataValues`
    pub base: MSMeanData_MeanDataValues,
    /// The map of vehicles to data entries
    pub myTrackedData: BTreeMap<Ptr<SUMOTrafficObject>, Rc<MeanDataValueTracker_TrackerEntry>>,
    /// The currently active meandata "intervals"
    pub myCurrentData: VecDeque<Rc<MeanDataValueTracker_TrackerEntry>>,
}

impl std::ops::Deref for MSMeanData_MeanDataValueTracker {
    type Target = MSMeanData_MeanDataValues;
    fn deref(&self) -> &MSMeanData_MeanDataValues {
        &self.base
    }
}
impl std::ops::DerefMut for MSMeanData_MeanDataValueTracker {
    fn deref_mut(&mut self) -> &mut MSMeanData_MeanDataValues {
        &mut self.base
    }
}

impl MSMeanData_MeanDataValueTracker {
    /// C++: `void MeanDataValueTracker::MeanDataValueTracker(MSLane *const, const double, const MSMeanData *const)`
    /// Constructor
    // MSMeanData::MeanDataValueTracker::MeanDataValueTracker(MSLane* const lane,
    //         const double length,
    //         const MSMeanData* const parent)
    //     : MSMeanData::MeanDataValues(lane, length, true, parent) {
    //     myCurrentData.push_back(std::make_shared<TrackerEntry>(parent->createValuesSubData(lane, length, false)));
    // }
    #[allow(unused_variables)]
    pub fn new(lane: Ptr<MSLane>, length: f64, parent: Ptr<MSMeanData>) -> Self {
        let base = MSMeanData_MeanDataValues::new(lane, length, true, parent);
        let mut tracker = Self {
            base,
            myTrackedData: BTreeMap::new(),
            myCurrentData: VecDeque::new(),
        };
        tracker.base.data_vt = &MEAN_DATA_VALUES_TRACKER_VT;
        tracker.myCurrentData.push_back(Rc::new(MeanDataValueTracker_TrackerEntry::new(parent.create_values_sub_data(lane, length, false))));
        tracker
    }

    /// C++: `void MeanDataValueTracker::~MeanDataValueTracker() noexcept` [virtual]
    /// Destructor
    // MSMeanData::MeanDataValueTracker::~MeanDataValueTracker() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty.
    }

    /// C++: `void MeanDataValueTracker::reset(bool)` [virtual]
    /// Resets values so they may be used for the next interval
    // void
    // MSMeanData::MeanDataValueTracker::reset(bool afterWrite) {
    //     if (afterWrite) {
    //         if (!myCurrentData.empty()) {
    //             myCurrentData.pop_front();
    //         }
    //     } else {
    //         myCurrentData.push_back(std::make_shared<TrackerEntry>(myParent->createValuesSubData(myLane, myLaneLength, false)));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn reset(&mut self, afterWrite: bool) {
        if afterWrite {
            if !self.myCurrentData.is_empty() {
                self.myCurrentData.pop_front();
            }
        } else {
            let entry = MeanDataValueTracker_TrackerEntry::new(self.myParent.create_values_sub_data(self.myLane, self.myLaneLength, false));
            self.myCurrentData.push_back(Rc::new(entry));
        }
    }

    /// C++: `void MeanDataValueTracker::addTo(MSMeanData::MeanDataValues &) const` [const, virtual]
    /// Add the values of this to the given one and store them there
    // void
    // MSMeanData::MeanDataValueTracker::addTo(MSMeanData::MeanDataValues& val) const {
    //     myCurrentData.front()->myValues->addTo(val);
    // }
    #[allow(unused_variables)]
    pub fn add_to(&self, val: &mut MSMeanData_MeanDataValues) {
        self.myCurrentData.front().unwrap().myValues.add_to(val);
    }

    /// C++: `void MeanDataValueTracker::notifyMoveInternal(const SUMOTrafficObject &, const double, const double, const double, const double, const double, const double, const double)` [virtual]
    /// Internal notification about the vehicle moves
    // void
    // MSMeanData::MeanDataValueTracker::notifyMoveInternal(const SUMOTrafficObject& veh, const double frontOnLane, const double timeOnLane, const double meanSpeedFrontOnLane, const double meanSpeedVehicleOnLane, const double travelledDistanceFrontOnLane, const double travelledDistanceVehicleOnLane, const double meanLengthOnLane) {
    //     myTrackedData[&veh]->myValues->notifyMoveInternal(veh, frontOnLane, timeOnLane, meanSpeedFrontOnLane, meanSpeedVehicleOnLane, travelledDistanceFrontOnLane, travelledDistanceVehicleOnLane, meanLengthOnLane);
    // }
    #[allow(unused_variables)]
    pub fn notify_move_internal(&mut self, veh: &SUMOTrafficObject, frontOnLane: f64, timeOnLane: f64, meanSpeedFrontOnLane: f64, meanSpeedVehicleOnLane: f64, travelledDistanceFrontOnLane: f64, travelledDistanceVehicleOnLane: f64, meanLengthOnLane: f64) {
        let key = Ptr::new(veh);
        self.myTrackedData[&key].myValues.m().notify_move_internal(veh, frontOnLane, timeOnLane, meanSpeedFrontOnLane, meanSpeedVehicleOnLane, travelledDistanceFrontOnLane, travelledDistanceVehicleOnLane, meanLengthOnLane);
    }

    /// C++: `bool MeanDataValueTracker::notifyLeave(SUMOTrafficObject &, double, MSMoveReminder::Notification, const MSLane *)` [virtual]
    /// Called if the vehicle leaves the reminder's lane
    // bool
    // MSMeanData::MeanDataValueTracker::notifyLeave(SUMOTrafficObject& veh, double lastPos, MSMoveReminder::Notification reason, const MSLane* /* enteredLane */) {
    //     if (myParent == nullptr || reason != MSMoveReminder::NOTIFICATION_SEGMENT) {
    //         myTrackedData[&veh]->myNumVehicleLeft++;
    //     }
    //     return myTrackedData[&veh]->myValues->notifyLeave(veh, lastPos, reason);
    // }
    #[allow(unused_variables)]
    pub fn notify_leave(&mut self, veh: &mut SUMOTrafficObject, lastPos: f64, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
        let key = Ptr::from_mut(&mut *veh);
        if self.myParent.is_null() || reason != MSMoveReminder_Notification::NOTIFICATION_SEGMENT {
            let entry = self.myTrackedData[&key].clone();
            entry.myNumVehicleLeft.set(entry.myNumVehicleLeft.get() + 1);
        }
        let entry = self.myTrackedData[&key].clone();
        entry.myValues.m().notify_leave(veh, lastPos, reason, enteredLane)
    }

    /// C++: `bool MeanDataValueTracker::notifyEnter(SUMOTrafficObject &, MSMoveReminder::Notification, const MSLane *)` [virtual]
    /// Computes current values and adds them to their sums
    // bool
    // MSMeanData::MeanDataValueTracker::notifyEnter(SUMOTrafficObject& veh, MSMoveReminder::Notification reason, const MSLane* enteredLane) {
    // #ifdef DEBUG_NOTIFY_ENTER
    //     std::cout << "\n" << SIMTIME << " MSMeanData::MeanDataValueTracker: veh '" << veh.getID() << "' enters lane '" << enteredLane->getID() << "'" << std::endl;
    // #else
    //     UNUSED_PARAMETER(enteredLane);
    // #endif
    //     if (reason == MSMoveReminder::NOTIFICATION_SEGMENT) {
    //         return true;
    //     }
    //     if (myParent->vehicleApplies(veh) && myTrackedData.find(&veh) == myTrackedData.end()) {
    //         myTrackedData[&veh] = myCurrentData.back();
    //         myTrackedData[&veh]->myNumVehicleEntered++;
    //         if (!myTrackedData[&veh]->myValues->notifyEnter(veh, reason)) {
    //             myTrackedData[&veh]->myNumVehicleLeft++;
    //             myTrackedData.erase(&veh);
    //             return false;
    //         }
    //         return true;
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn notify_enter(&mut self, veh: &mut SUMOTrafficObject, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
        if reason == MSMoveReminder_Notification::NOTIFICATION_SEGMENT {
            return true;
        }
        let key = Ptr::from_mut(&mut *veh);
        if self.myParent.base.vehicle_applies(veh) && !self.myTrackedData.contains_key(&key) {
            let entry = self.myCurrentData.back().unwrap().clone();
            self.myTrackedData.insert(key, entry.clone());
            entry.myNumVehicleEntered.set(entry.myNumVehicleEntered.get() + 1);
            if !entry.myValues.m().notify_enter(veh, reason, enteredLane) {
                entry.myNumVehicleLeft.set(entry.myNumVehicleLeft.get() + 1);
                self.myTrackedData.remove(&key);
                return false;
            }
            return true;
        }
        false
    }

    /// C++: `bool MeanDataValueTracker::isEmpty() const` [const, virtual]
    // bool
    // MSMeanData::MeanDataValueTracker::isEmpty() const {
    //     return myCurrentData.front()->myValues->isEmpty();
    // }
    #[allow(unused_variables)]
    pub fn is_empty(&self) -> bool {
        self.myCurrentData.front().unwrap().myValues.is_empty()
    }

    /// C++: `void MeanDataValueTracker::write(OutputDevice &, const SumoXMLAttrMask &, const SUMOTime, const int, const double, const double, const int) const` [const, virtual]
    /// Writes output values into the given stream
    // void
    // MSMeanData::MeanDataValueTracker::write(OutputDevice& dev,
    //                                         const SumoXMLAttrMask& attributeMask,
    //                                         const SUMOTime period,
    //                                         const int numLanes,
    //                                         const double speedLimit,
    //                                         const double defaultTravelTime,
    //                                         const int /*numVehicles*/) const {
    //     myCurrentData.front()->myValues->write(dev, attributeMask, period, numLanes, speedLimit,
    //                                            defaultTravelTime,
    //                                            myCurrentData.front()->myNumVehicleEntered);
    // }
    #[allow(unused_variables)]
    pub fn write(&self, dev: &mut OutputDevice, attributeMask: &SumoXMLAttrMask, period: SUMOTime, numLanes: i32, speedLimit: f64, defaultTravelTime: f64, numVehicles: i32) {
        let front = self.myCurrentData.front().unwrap();
        front.myValues.write(dev, attributeMask, period, numLanes, speedLimit, defaultTravelTime, front.myNumVehicleEntered.get());
    }

    /// C++: `int MeanDataValueTracker::getNumReady() const` [const]
    // int
    // MSMeanData::MeanDataValueTracker::getNumReady() const {
    //     int result = 0;
    //     for (const auto& it : myCurrentData) {
    //         if (it->myNumVehicleEntered == it->myNumVehicleLeft) {
    //             result++;
    //         } else {
    //             break;
    //         }
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_num_ready(&self) -> i32 {
        let mut result = 0;
        for it in self.myCurrentData.iter() {
            if it.myNumVehicleEntered.get() == it.myNumVehicleLeft.get() {
                result += 1;
            } else {
                break;
            }
        }
        result
    }

    /// C++: `void MeanDataValueTracker::clearFirst()`
    // (declared in MSMeanData.h, never defined in C++: calling it fails to link)
    #[allow(unused_variables)]
    pub fn clear_first(&mut self) {
        // shortcut: declared but never defined in C++, so no behaviour to port; panics if reached
        panic!("MeanDataValueTracker::clearFirst is declared but not defined in SUMO")
    }

    /// C++: `double MeanDataValueTracker::getSamples() const` [const, virtual]
    // double
    // MSMeanData::MeanDataValueTracker::getSamples() const {
    //     return myCurrentData.front()->myValues->getSamples();
    // }
    #[allow(unused_variables)]
    pub fn get_samples(&self) -> f64 {
        self.myCurrentData.front().unwrap().myValues.get_samples()
    }

}

/// Data structure for mean (aggregated) edge/lane values
/// C++ class `MSMeanData::MeanDataValues` (microsim/output/MSMeanData.h)
#[repr(C)] // base must stay at offset 0 for the C++ downcast in as_tracker
pub struct MSMeanData_MeanDataValues {
    /// C++ base class `MSMoveReminder`
    pub base: MSMoveReminder,
    /// The meandata parent
    pub myParent: Ptr<MSMeanData>,
    /// The length of the lane / edge the data collector is on
    pub myLaneLength: f64,
    /// The number of sampled vehicle movements (in s)
    pub sampleSeconds: f64,
    /// The sum of the distances the vehicles travelled
    pub travelledDistance: f64,
    /// time at which collection was reset;
    pub resetTime: SUMOTime,
    /// additional data to collect and write
    pub mySubData: Ptr<MSMeanData_MeanDataValues>,
    /// Virtual dispatch table for the overridable methods; each subclass constructor installs its own
    pub data_vt: &'static MeanDataValuesVt,
}

impl std::ops::Deref for MSMeanData_MeanDataValues {
    type Target = MSMoveReminder;
    fn deref(&self) -> &MSMoveReminder {
        &self.base
    }
}
impl std::ops::DerefMut for MSMeanData_MeanDataValues {
    fn deref_mut(&mut self) -> &mut MSMoveReminder {
        &mut self.base
    }
}

impl MSMeanData_MeanDataValues {
    /// C++: `void MeanDataValues::MeanDataValues(MSLane *const, const double, const bool, const MSMeanData *const)`
    /// Constructor
    // MSMeanData::MeanDataValues::MeanDataValues(
    //     MSLane* const lane, const double length, const bool doAdd,
    //     const MSMeanData* const parent) :
    //     MSMoveReminder("meandata_" + (parent == nullptr ? "" : parent->getID() + "|") + (lane == nullptr ? "NULL" :  lane->getID()), lane, doAdd),
    //     myParent(parent),
    //     myLaneLength(length),
    //     sampleSeconds(0),
    //     travelledDistance(0) { }
    #[allow(unused_variables)]
    pub fn new(lane: Ptr<MSLane>, length: f64, doAdd: bool, parent: Ptr<MSMeanData>) -> Self {
        let parentId = if parent.is_null() { String::new() } else { format!("{}|", parent.get_id()) };
        let laneId = if lane.is_null() { "NULL".to_string() } else { lane.get_id().to_string() };
        let mut base = MSMoveReminder::new(&format!("meandata_{}{}", parentId, laneId), lane, doAdd);
        base.vt = &MEAN_DATA_VALUES_MSMOVE_VT;
        base.internal_vt = &MEAN_DATA_VALUES_MSMOVE_INTERNAL_VT;
        Self {
            base,
            myParent: parent,
            myLaneLength: length,
            sampleSeconds: 0.0,
            travelledDistance: 0.0,
            resetTime: 0,
            mySubData: Ptr::null(),
            data_vt: &MEAN_DATA_VALUES_VT,
        }
    }

    /// C++: `void MeanDataValues::~MeanDataValues() noexcept` [virtual]
    /// Destructor
    // MSMeanData::MeanDataValues::~MeanDataValues() {
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty.
    }

    /// C++: `void MeanDataValues::reset(bool)` [virtual, pure virtual]
    /// Resets values so they may be used for the next interval
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn reset_base(&mut self, afterWrite: bool) {
        // shortcut: pure virtual in C++; the subclass table supplies it, so this panics like a pure-virtual call
        panic!("MeanDataValues::reset is pure virtual; the subclass must implement it")
    }

    /// C++: `void MeanDataValues::addTo(MeanDataValues &) const` [const, virtual, pure virtual]
    /// Add the values of this to the given one and store them there
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn add_to_base(&self, val: &mut MSMeanData_MeanDataValues) {
        // shortcut: pure virtual in C++; see reset_base
        panic!("MeanDataValues::addTo is pure virtual; the subclass must implement it")
    }

    /// C++: `bool MeanDataValues::notifyEnter(SUMOTrafficObject &, MSMoveReminder::Notification, const MSLane *)` [virtual]
    /// Called if the vehicle enters the reminder's lane
    // bool
    // MSMeanData::MeanDataValues::notifyEnter(SUMOTrafficObject& veh, MSMoveReminder::Notification reason, const MSLane* enteredLane) {
    // #ifdef DEBUG_NOTIFY_ENTER
    //     std::cout << "\n" << SIMTIME << " MSMeanData_Net::MSLaneMeanDataValues: veh '" << veh.getID() << "' enters lane '" << enteredLane->getID() << "'" << std::endl;
    // #else
    //     UNUSED_PARAMETER(enteredLane);
    // #endif
    //     UNUSED_PARAMETER(reason);
    //     return myParent == nullptr || myParent->vehicleApplies(veh);
    // }
    #[allow(unused_variables)]
    pub fn notify_enter_base(&mut self, veh: &mut SUMOTrafficObject, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
        // UNUSED_PARAMETER(reason), enteredLane is only used in debug output in C++
        self.myParent.is_null() || self.myParent.base.vehicle_applies(veh)
    }

    /// C++: `bool MeanDataValues::notifyMove(SUMOTrafficObject &, double, double, double)` [virtual]
    /// Checks whether the reminder still has to be notified about the vehicle moves
    // bool
    // MSMeanData::MeanDataValues::notifyMove(SUMOTrafficObject& veh, double oldPos, double newPos, double newSpeed) {
    //     // if the vehicle has arrived, the reminder must be kept so it can be
    //     // notified of the arrival subsequently
    //     const double oldSpeed = veh.getPreviousSpeed();
    //     double enterSpeed = MSGlobals::gSemiImplicitEulerUpdate ? newSpeed : oldSpeed; // NOTE: For the euler update, the vehicle is assumed to travel at constant speed for the whole time step
    //     double leaveSpeed = newSpeed, leaveSpeedFront = newSpeed;
    //
    //     // These values will be further decreased below
    //     double timeOnLane = TS;
    //     double frontOnLane = oldPos > myLaneLength ? 0. : TS;
    //     bool ret = true;
    //
    //     // entry and exit times (will be modified below)
    //     double timeBeforeEnter = 0.;
    //     double timeBeforeEnterBack = 0.;
    //     double timeBeforeLeaveFront = newPos <= myLaneLength ? TS : 0.;
    //     double timeBeforeLeave = TS;
    //
    //     // Treat the case that the vehicle entered the lane in the last step
    //     if (oldPos < 0 && newPos >= 0) {
    //         // Vehicle was not on this lane in the last time step
    //         timeBeforeEnter = MSCFModel::passingTime(oldPos, 0, newPos, oldSpeed, newSpeed);
    //         timeOnLane = TS - timeBeforeEnter;
    //         frontOnLane = timeOnLane;
    //         enterSpeed = MSCFModel::speedAfterTime(timeBeforeEnter, oldSpeed, newPos - oldPos);
    //     }
    //
    //     const double oldBackPos = oldPos - veh.getVehicleType().getLength();
    //     const double newBackPos = newPos - veh.getVehicleType().getLength();
    //
    //     // Determine the time before the vehicle back enters
    //     if (oldBackPos < 0. && newBackPos > 0.) {
    //         timeBeforeEnterBack = MSCFModel::passingTime(oldBackPos, 0., newBackPos, oldSpeed, newSpeed);
    //     } else if (newBackPos <= 0) {
    //         timeBeforeEnterBack = TS;
    //     } else {
    //         timeBeforeEnterBack = 0.;
    //     }
    //
    //     // Treat the case that the vehicle's back left the lane in the last step
    //     if (newBackPos > myLaneLength // vehicle's back has left the lane
    //             && oldBackPos <= myLaneLength) { // and hasn't left the lane before
    //         assert(!MSGlobals::gSemiImplicitEulerUpdate || newSpeed != 0); // how could it move across the lane boundary otherwise
    //         // (Leo) vehicle left this lane (it can also have skipped over it in one time step -> therefore we use "timeOnLane -= ..." and ( ... - timeOnLane) below)
    //         timeBeforeLeave = MSCFModel::passingTime(oldBackPos, myLaneLength, newBackPos, oldSpeed, newSpeed);
    //         const double timeAfterLeave = TS - timeBeforeLeave;
    //         timeOnLane -= timeAfterLeave;
    //         leaveSpeed = MSCFModel::speedAfterTime(timeBeforeLeave, oldSpeed, newPos - oldPos);
    //         // XXX: Do we really need this? Why would this "reduce rounding errors"? (Leo) Refs. #2579
    //         if (fabs(timeOnLane) < NUMERICAL_EPS) { // reduce rounding errors
    //             timeOnLane = 0.;
    //         }
    //         ret = veh.hasArrived();
    //     }
    //
    //     // Treat the case that the vehicle's front left the lane in the last step
    //     if (newPos > myLaneLength && oldPos <= myLaneLength) {
    //         // vehicle's front has left the lane and has not left before
    //         assert(!MSGlobals::gSemiImplicitEulerUpdate || newSpeed != 0);
    //         timeBeforeLeaveFront = MSCFModel::passingTime(oldPos, myLaneLength, newPos, oldSpeed, newSpeed);
    //         const double timeAfterLeave = TS - timeBeforeLeaveFront;
    //         frontOnLane -= timeAfterLeave;
    //         // XXX: Do we really need this? Why would this "reduce rounding errors"? (Leo) Refs. #2579
    //         if (fabs(frontOnLane) < NUMERICAL_EPS) { // reduce rounding errors
    //             frontOnLane = 0.;
    //         }
    //         leaveSpeedFront = MSCFModel::speedAfterTime(timeBeforeLeaveFront, oldSpeed, newPos - oldPos);
    //     }
    //
    //     assert(frontOnLane <= TS);
    //     assert(timeOnLane <= TS);
    //
    //     if (timeOnLane < 0) {
    //         WRITE_ERRORF(TL("Negative vehicle step fraction for '%' on lane '%'."), veh.getID(), getLane()->getID());
    //         return veh.hasArrived();
    //     }
    //     if (timeOnLane == 0) {
    //         return veh.hasArrived();
    //     }
    //
    // #ifdef DEBUG_NOTIFY_MOVE
    //     std::stringstream ss;
    //     ss << "\n"
    //        << "lane length: " << myLaneLength
    //        << "\noldPos: " << oldPos
    //        << "\nnewPos: " << newPos
    //        << "\noldPosBack: " << oldBackPos
    //        << "\nnewPosBack: " << newBackPos
    //        << "\ntimeBeforeEnter: " << timeBeforeEnter
    //        << "\ntimeBeforeEnterBack: " << timeBeforeEnterBack
    //        << "\ntimeBeforeLeaveFront: " << timeBeforeLeaveFront
    //        << "\ntimeBeforeLeave: " << timeBeforeLeave;
    //     if (!(timeBeforeLeave >= MAX2(timeBeforeEnterBack, timeBeforeLeaveFront))
    //             || !(timeBeforeEnter <= MIN2(timeBeforeEnterBack, timeBeforeLeaveFront))) {
    //         WRITE_ERROR(ss.str());
    //     } else {
    //         std::cout << ss.str() << std::endl;
    //     }
    //
    // #endif
    //
    //     assert(timeBeforeEnter <= MIN2(timeBeforeEnterBack, timeBeforeLeaveFront));
    //     assert(timeBeforeLeave >= MAX2(timeBeforeEnterBack, timeBeforeLeaveFront));
    //     // compute average vehicle length on lane in last step
    //     double vehLength = veh.getVehicleType().getLength();
    //     // occupied lane length at timeBeforeEnter (resp. stepStart if already on lane)
    //     double lengthOnLaneAtStepStart = MAX2(0., MIN4(myLaneLength, vehLength, vehLength - (oldPos - myLaneLength), oldPos));
    //     // occupied lane length at timeBeforeLeave (resp. stepEnd if still on lane)
    //     double lengthOnLaneAtStepEnd = MAX2(0., MIN4(myLaneLength, vehLength, vehLength - (newPos - myLaneLength), newPos));
    //     double integratedLengthOnLane = 0.;
    //     if (timeBeforeEnterBack < timeBeforeLeaveFront) {
    //         // => timeBeforeLeaveFront>0, myLaneLength>vehLength
    //         // vehicle length on detector at timeBeforeEnterBack
    //         double lengthOnLaneAtBackEnter = MIN2(veh.getVehicleType().getLength(), newPos);
    //         // linear quadrature of occupancy between timeBeforeEnter and timeBeforeEnterBack
    //         integratedLengthOnLane += (timeBeforeEnterBack - timeBeforeEnter) * (lengthOnLaneAtBackEnter + lengthOnLaneAtStepStart) * 0.5;
    //         // linear quadrature of occupancy between timeBeforeEnterBack and timeBeforeLeaveFront
    //         // (vehicle is completely on the edge in between)
    //         integratedLengthOnLane += (timeBeforeLeaveFront - timeBeforeEnterBack) * vehLength;
    //         // and until vehicle leaves/stepEnd
    //         integratedLengthOnLane += (timeBeforeLeave - timeBeforeLeaveFront) * (vehLength + lengthOnLaneAtStepEnd) * 0.5;
    //     } else if (timeBeforeEnterBack >= timeBeforeLeaveFront) {
    //         // => myLaneLength <= vehLength or (timeBeforeLeaveFront == timeBeforeEnterBack == 0)
    //         // vehicle length on detector at timeBeforeLeaveFront
    //         double lengthOnLaneAtLeaveFront;
    //         if (timeBeforeLeaveFront == timeBeforeEnter) {
    //             // for the case that front already left
    //             lengthOnLaneAtLeaveFront = lengthOnLaneAtStepStart;
    //         } else if (timeBeforeLeaveFront == timeBeforeLeave) {
    //             // for the case that front doesn't leave in this step
    //             lengthOnLaneAtLeaveFront = lengthOnLaneAtStepEnd;
    //         } else {
    //             lengthOnLaneAtLeaveFront = myLaneLength;
    //         }
    // #ifdef DEBUG_NOTIFY_MOVE
    //         std::cout << "lengthOnLaneAtLeaveFront=" << lengthOnLaneAtLeaveFront << std::endl;
    // #endif
    //         // linear quadrature of occupancy between timeBeforeEnter and timeBeforeLeaveFront
    //         integratedLengthOnLane += (timeBeforeLeaveFront - timeBeforeEnter) * (lengthOnLaneAtLeaveFront + lengthOnLaneAtStepStart) * 0.5;
    //         // linear quadrature of occupancy between timeBeforeLeaveFront and timeBeforeEnterBack
    //         integratedLengthOnLane += (timeBeforeEnterBack - timeBeforeLeaveFront) * lengthOnLaneAtLeaveFront;
    //         // and until vehicle leaves/stepEnd
    //         integratedLengthOnLane += (timeBeforeLeave - timeBeforeEnterBack) * (lengthOnLaneAtLeaveFront + lengthOnLaneAtStepEnd) * 0.5;
    //     }
    //
    //     double meanLengthOnLane = integratedLengthOnLane / TS;
    // #ifdef DEBUG_NOTIFY_MOVE
    //     std::cout << "Calculated mean length on lane '" << myLane->getID() << "' in last step as " << meanLengthOnLane
    //               << "\nlengthOnLaneAtStepStart=" << lengthOnLaneAtStepStart << ", lengthOnLaneAtStepEnd=" << lengthOnLaneAtStepEnd << ", integratedLengthOnLane=" << integratedLengthOnLane
    //               << std::endl;
    // #endif
    //
    // //    // XXX: use this, when #2556 is fixed! Refs. #2575
    // //    const double travelledDistanceFrontOnLane = MAX2(0., MIN2(newPos, myLaneLength) - MAX2(oldPos, 0.));
    // //    const double travelledDistanceVehicleOnLane = MIN2(newPos, myLaneLength) - MAX2(oldPos, 0.) + MIN2(MAX2(0., newPos - myLaneLength), veh.getVehicleType().getLength());
    // //    // XXX: #2556 fixed for ballistic update
    //     const double travelledDistanceFrontOnLane = MSGlobals::gSemiImplicitEulerUpdate ? frontOnLane * newSpeed
    //             : MAX2(0., MIN2(newPos, myLaneLength) - MAX2(oldPos, 0.));
    //     const double travelledDistanceVehicleOnLane = MSGlobals::gSemiImplicitEulerUpdate ? timeOnLane * newSpeed
    //             : MIN2(newPos, myLaneLength) - MAX2(oldPos, 0.) + MIN2(MAX2(0., newPos - myLaneLength), veh.getVehicleType().getLength());
    // //    // XXX: no fix
    // //    const double travelledDistanceFrontOnLane = frontOnLane*newSpeed;
    // //    const double travelledDistanceVehicleOnLane = timeOnLane*newSpeed;
    //
    // #ifdef HAVE_FOX
    //     ScopedLocker<> lock(myNotificationMutex, MSGlobals::gNumSimThreads > 1);
    // #endif
    //     notifyMoveInternal(veh, frontOnLane, timeOnLane, (enterSpeed + leaveSpeedFront) / 2., (enterSpeed + leaveSpeed) / 2., travelledDistanceFrontOnLane, travelledDistanceVehicleOnLane, meanLengthOnLane);
    //     return ret;
    // }
    #[allow(unused_variables)]
    pub fn notify_move(&mut self, veh: &mut SUMOTrafficObject, oldPos: f64, newPos: f64, newSpeed: f64) -> bool {
        // if the vehicle has arrived, the reminder must be kept so it can be
        // notified of the arrival subsequently
        let oldSpeed = veh.get_previous_speed();
        let mut enterSpeed = if g_semi_implicit_euler_update() { newSpeed } else { oldSpeed }; // NOTE: For the euler update, the vehicle is assumed to travel at constant speed for the whole time step
        let mut leaveSpeed = newSpeed;
        let mut leaveSpeedFront = newSpeed;

        // These values will be further decreased below
        let TS = ts();
        let mut timeOnLane = TS;
        let mut frontOnLane = if oldPos > self.myLaneLength { 0. } else { TS };
        let mut ret = true;

        // entry and exit times (will be modified below)
        let mut timeBeforeEnter = 0.;
        let mut timeBeforeEnterBack = 0.;
        let mut timeBeforeLeaveFront = if newPos <= self.myLaneLength { TS } else { 0. };
        let mut timeBeforeLeave = TS;

        // Treat the case that the vehicle entered the lane in the last step
        if oldPos < 0. && newPos >= 0. {
            // Vehicle was not on this lane in the last time step
            timeBeforeEnter = MSCFModel::passing_time(oldPos, 0., newPos, oldSpeed, newSpeed);
            timeOnLane = TS - timeBeforeEnter;
            frontOnLane = timeOnLane;
            enterSpeed = MSCFModel::speed_after_time(timeBeforeEnter, oldSpeed, newPos - oldPos);
        }

        let oldBackPos = oldPos - veh.get_vehicle_type().get_length();
        let newBackPos = newPos - veh.get_vehicle_type().get_length();

        // Determine the time before the vehicle back enters
        if oldBackPos < 0. && newBackPos > 0. {
            timeBeforeEnterBack = MSCFModel::passing_time(oldBackPos, 0., newBackPos, oldSpeed, newSpeed);
        } else if newBackPos <= 0. {
            timeBeforeEnterBack = TS;
        } else {
            timeBeforeEnterBack = 0.;
        }

        // Treat the case that the vehicle's back left the lane in the last step
        if newBackPos > self.myLaneLength // vehicle's back has left the lane
            && oldBackPos <= self.myLaneLength
        {
            // and hasn't left the lane before
            debug_assert!(!g_semi_implicit_euler_update() || newSpeed != 0.); // how could it move across the lane boundary otherwise
            // (Leo) vehicle left this lane (it can also have skipped over it in one time step -> therefore we use "timeOnLane -= ..." and ( ... - timeOnLane) below)
            timeBeforeLeave = MSCFModel::passing_time(oldBackPos, self.myLaneLength, newBackPos, oldSpeed, newSpeed);
            let timeAfterLeave = TS - timeBeforeLeave;
            timeOnLane -= timeAfterLeave;
            leaveSpeed = MSCFModel::speed_after_time(timeBeforeLeave, oldSpeed, newPos - oldPos);
            // XXX: Do we really need this? Why would this "reduce rounding errors"? (Leo) Refs. #2579
            if timeOnLane.abs() < utils::geom::NUMERICAL_EPS {
                // reduce rounding errors
                timeOnLane = 0.;
            }
            ret = veh.has_arrived();
        }

        // Treat the case that the vehicle's front left the lane in the last step
        if newPos > self.myLaneLength && oldPos <= self.myLaneLength {
            // vehicle's front has left the lane and has not left before
            debug_assert!(!g_semi_implicit_euler_update() || newSpeed != 0.);
            timeBeforeLeaveFront = MSCFModel::passing_time(oldPos, self.myLaneLength, newPos, oldSpeed, newSpeed);
            let timeAfterLeave = TS - timeBeforeLeaveFront;
            frontOnLane -= timeAfterLeave;
            // XXX: Do we really need this? Why would this "reduce rounding errors"? (Leo) Refs. #2579
            if frontOnLane.abs() < utils::geom::NUMERICAL_EPS {
                // reduce rounding errors
                frontOnLane = 0.;
            }
            leaveSpeedFront = MSCFModel::speed_after_time(timeBeforeLeaveFront, oldSpeed, newPos - oldPos);
        }

        debug_assert!(frontOnLane <= TS);
        debug_assert!(timeOnLane <= TS);

        if timeOnLane < 0. {
            utils::write_errorf!("Negative vehicle step fraction for '%' on lane '%'.", veh.get_id(), self.get_lane().get_id());
            return veh.has_arrived();
        }
        if timeOnLane == 0. {
            return veh.has_arrived();
        }

        debug_assert!(timeBeforeEnter <= min2(timeBeforeEnterBack, timeBeforeLeaveFront));
        debug_assert!(timeBeforeLeave >= max2(timeBeforeEnterBack, timeBeforeLeaveFront));
        // compute average vehicle length on lane in last step
        let vehLength = veh.get_vehicle_type().get_length();
        // occupied lane length at timeBeforeEnter (resp. stepStart if already on lane)
        let lengthOnLaneAtStepStart = max2(0., min4(self.myLaneLength, vehLength, vehLength - (oldPos - self.myLaneLength), oldPos));
        // occupied lane length at timeBeforeLeave (resp. stepEnd if still on lane)
        let lengthOnLaneAtStepEnd = max2(0., min4(self.myLaneLength, vehLength, vehLength - (newPos - self.myLaneLength), newPos));
        let mut integratedLengthOnLane = 0.;
        if timeBeforeEnterBack < timeBeforeLeaveFront {
            // => timeBeforeLeaveFront>0, myLaneLength>vehLength
            // vehicle length on detector at timeBeforeEnterBack
            let lengthOnLaneAtBackEnter = min2(veh.get_vehicle_type().get_length(), newPos);
            // linear quadrature of occupancy between timeBeforeEnter and timeBeforeEnterBack
            integratedLengthOnLane += (timeBeforeEnterBack - timeBeforeEnter) * (lengthOnLaneAtBackEnter + lengthOnLaneAtStepStart) * 0.5;
            // linear quadrature of occupancy between timeBeforeEnterBack and timeBeforeLeaveFront
            // (vehicle is completely on the edge in between)
            integratedLengthOnLane += (timeBeforeLeaveFront - timeBeforeEnterBack) * vehLength;
            // and until vehicle leaves/stepEnd
            integratedLengthOnLane += (timeBeforeLeave - timeBeforeLeaveFront) * (vehLength + lengthOnLaneAtStepEnd) * 0.5;
        } else if timeBeforeEnterBack >= timeBeforeLeaveFront {
            // => myLaneLength <= vehLength or (timeBeforeLeaveFront == timeBeforeEnterBack == 0)
            // vehicle length on detector at timeBeforeLeaveFront
            let lengthOnLaneAtLeaveFront;
            if timeBeforeLeaveFront == timeBeforeEnter {
                // for the case that front already left
                lengthOnLaneAtLeaveFront = lengthOnLaneAtStepStart;
            } else if timeBeforeLeaveFront == timeBeforeLeave {
                // for the case that front doesn't leave in this step
                lengthOnLaneAtLeaveFront = lengthOnLaneAtStepEnd;
            } else {
                lengthOnLaneAtLeaveFront = self.myLaneLength;
            }
            // linear quadrature of occupancy between timeBeforeEnter and timeBeforeLeaveFront
            integratedLengthOnLane += (timeBeforeLeaveFront - timeBeforeEnter) * (lengthOnLaneAtLeaveFront + lengthOnLaneAtStepStart) * 0.5;
            // linear quadrature of occupancy between timeBeforeLeaveFront and timeBeforeEnterBack
            integratedLengthOnLane += (timeBeforeEnterBack - timeBeforeLeaveFront) * lengthOnLaneAtLeaveFront;
            // and until vehicle leaves/stepEnd
            integratedLengthOnLane += (timeBeforeLeave - timeBeforeEnterBack) * (lengthOnLaneAtLeaveFront + lengthOnLaneAtStepEnd) * 0.5;
        }

        let meanLengthOnLane = integratedLengthOnLane / TS;

        // XXX: use this, when #2556 is fixed! Refs. #2575
        // XXX: #2556 fixed for ballistic update
        let travelledDistanceFrontOnLane = if g_semi_implicit_euler_update() { frontOnLane * newSpeed } else { max2(0., min2(newPos, self.myLaneLength) - max2(oldPos, 0.)) };
        let travelledDistanceVehicleOnLane = if g_semi_implicit_euler_update() {
            timeOnLane * newSpeed
        } else {
            min2(newPos, self.myLaneLength) - max2(oldPos, 0.) + min2(max2(0., newPos - self.myLaneLength), veh.get_vehicle_type().get_length())
        };

        // shortcut: the HAVE_FOX ScopedLocker around notifyMoveInternal is not needed (single-threaded port)
        let veh_ref: &SUMOTrafficObject = veh;
        self.notify_move_internal(veh_ref, frontOnLane, timeOnLane, (enterSpeed + leaveSpeedFront) / 2., (enterSpeed + leaveSpeed) / 2., travelledDistanceFrontOnLane, travelledDistanceVehicleOnLane, meanLengthOnLane);
        ret
    }

    /// C++: `bool MeanDataValues::notifyLeave(SUMOTrafficObject &, double, MSMoveReminder::Notification, const MSLane *)` [virtual]
    /// Called if the vehicle leaves the reminder's lane
    // bool
    // MSMeanData::MeanDataValues::notifyLeave(SUMOTrafficObject& /*veh*/, double /*lastPos*/, MSMoveReminder::Notification reason, const MSLane* /* enteredLane */) {
    //     if (MSGlobals::gUseMesoSim) {
    //         return false; // reminder is re-added on every segment (@recheck for performance)
    //     }
    //     return reason == MSMoveReminder::NOTIFICATION_JUNCTION;
    // }
    #[allow(unused_variables)]
    pub fn notify_leave_base(&mut self, veh: &mut SUMOTrafficObject, lastPos: f64, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
        if g_use_meso_sim() {
            return false; // reminder is re-added on every segment (@recheck for performance)
        }
        reason == MSMoveReminder_Notification::NOTIFICATION_JUNCTION
    }

    /// C++: `bool MeanDataValues::isEmpty() const` [const, virtual]
    /// Returns whether any data was collected.
    // bool
    // MSMeanData::MeanDataValues::isEmpty() const {
    //     return sampleSeconds == 0;
    // }
    #[allow(unused_variables)]
    pub fn is_empty_base(&self) -> bool {
        self.sampleSeconds == 0.
    }

    /// C++: `void MeanDataValues::update()` [virtual]
    /// Called if a per timestep update is needed. Default does nothing.
    // void
    // MSMeanData::MeanDataValues::update() {
    // }
    #[allow(unused_variables)]
    pub fn update_base(&mut self) {}

    /// C++: `void MeanDataValues::write(OutputDevice &, const SumoXMLAttrMask &, const SUMOTime, const int, const double, const double, const int) const` [const, virtual, pure virtual]
    /// Writes output values into the given stream
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn write_base(&self, dev: &mut OutputDevice, attributeMask: &SumoXMLAttrMask, period: SUMOTime, numLanes: i32, speedLimit: f64, defaultTravelTime: f64, numVehicles: i32) {
        // shortcut: pure virtual in C++; see reset_base
        panic!("MeanDataValues::write is pure virtual; the subclass must implement it")
    }

    /// C++: `double MeanDataValues::getSamples() const` [const, virtual]
    /// Returns the number of collected sample seconds.
    // double
    // MSMeanData::MeanDataValues::getSamples() const {
    //     return sampleSeconds;
    // }
    #[allow(unused_variables)]
    pub fn get_samples_base(&self) -> f64 {
        self.sampleSeconds
    }

    /// C++: `double MeanDataValues::getTravelledDistance() const` [const]
    /// Returns the total travelled distance.
    // double getTravelledDistance() const {
    //             return travelledDistance;
    //         }
    #[allow(unused_variables)]
    pub fn get_travelled_distance(&self) -> f64 {
        self.travelledDistance
    }

    /// C++: `SUMOTime MeanDataValues::getResetTime() const` [const]
    // SUMOTime getResetTime() const {
    //             return resetTime;
    //         }
    #[allow(unused_variables)]
    pub fn get_reset_time(&self) -> SUMOTime {
        self.resetTime
    }

    /// C++: `double MeanDataValues::getLaneLength() const` [const]
    // double getLaneLength() const {
    //             return myLaneLength;
    //         }
    #[allow(unused_variables)]
    pub fn get_lane_length(&self) -> f64 {
        self.myLaneLength
    }

    /// C++: `double MeanDataValues::getAttributeValue(SumoXMLAttr, const SUMOTime, const double, const double) const` [const, virtual]
    /// return attribute value
    // virtual double getAttributeValue(SumoXMLAttr a, const SUMOTime period, const double numLanes, const double speedLimit) const {
    //             UNUSED_PARAMETER(a);
    //             UNUSED_PARAMETER(period);
    //             UNUSED_PARAMETER(numLanes);
    //             UNUSED_PARAMETER(speedLimit);
    //             return 0;
    //         }
    #[allow(unused_variables)]
    pub fn get_attribute_value_base(&self, a: SumoXMLAttr, period: SUMOTime, numLanes: f64, speedLimit: f64) -> f64 {
        0.
    }

    /// C++: `MeanDataValues * MeanDataValues::getSubData()`
    // MeanDataValues* getSubData() {
    //             return mySubData;
    //         }
    #[allow(unused_variables)]
    pub fn get_sub_data(&mut self) -> Ptr<MSMeanData_MeanDataValues> {
        self.mySubData
    }

    /// C++: `void MeanDataValues::setSubData(MeanDataValues *)`
    // void setSubData(MeanDataValues* subData) {
    //             mySubData = subData;
    //         }
    #[allow(unused_variables)]
    pub fn set_sub_data(&mut self, subData: Ptr<MSMeanData_MeanDataValues>) {
        self.mySubData = subData;
    }

}

/// Virtual dispatch table for the overridable methods of MeanDataValues (C++ virtuals).
/// Each subclass constructor installs its own table in `data_vt`; the base table forwards to the `*_base` methods.
/// notifyMove is not overridden by any subclass, so it is called directly.
pub struct MeanDataValuesVt {
    pub reset: fn(&mut MSMeanData_MeanDataValues, bool),
    pub add_to: fn(&MSMeanData_MeanDataValues, &mut MSMeanData_MeanDataValues),
    pub notify_enter: fn(&mut MSMeanData_MeanDataValues, &mut SUMOTrafficObject, MSMoveReminder_Notification, Ptr<MSLane>) -> bool,
    pub notify_leave: fn(&mut MSMeanData_MeanDataValues, &mut SUMOTrafficObject, f64, MSMoveReminder_Notification, Ptr<MSLane>) -> bool,
    pub notify_move_internal: fn(&mut MSMeanData_MeanDataValues, &SUMOTrafficObject, f64, f64, f64, f64, f64, f64, f64),
    pub is_empty: fn(&MSMeanData_MeanDataValues) -> bool,
    pub update: fn(&mut MSMeanData_MeanDataValues),
    pub write: fn(&MSMeanData_MeanDataValues, &mut OutputDevice, &SumoXMLAttrMask, SUMOTime, i32, f64, f64, i32),
    pub get_samples: fn(&MSMeanData_MeanDataValues) -> f64,
    pub get_attribute_value: fn(&MSMeanData_MeanDataValues, SumoXMLAttr, SUMOTime, f64, f64) -> f64,
}

impl MSMeanData_MeanDataValues {
    /// C++ virtual `MeanDataValues::reset`: dispatches to the subclass table
    pub fn reset(&mut self, afterWrite: bool) {
        (self.data_vt.reset)(self, afterWrite)
    }

    /// C++ virtual `MeanDataValues::addTo`: dispatches to the subclass table
    pub fn add_to(&self, val: &mut MSMeanData_MeanDataValues) {
        (self.data_vt.add_to)(self, val)
    }

    /// C++ virtual `MeanDataValues::notifyEnter`: dispatches to the subclass table
    pub fn notify_enter(&mut self, veh: &mut SUMOTrafficObject, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
        (self.data_vt.notify_enter)(self, veh, reason, enteredLane)
    }

    /// C++ virtual `MeanDataValues::notifyLeave`: dispatches to the subclass table
    pub fn notify_leave(&mut self, veh: &mut SUMOTrafficObject, lastPos: f64, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
        (self.data_vt.notify_leave)(self, veh, lastPos, reason, enteredLane)
    }

    /// C++ virtual `MSMoveReminder::notifyMoveInternal`: dispatches to the subclass table
    pub fn notify_move_internal(&mut self, veh: &SUMOTrafficObject, frontOnLane: f64, timeOnLane: f64, meanSpeedFrontOnLane: f64, meanSpeedVehicleOnLane: f64, travelledDistanceFrontOnLane: f64, travelledDistanceVehicleOnLane: f64, meanLengthOnLane: f64) {
        (self.data_vt.notify_move_internal)(self, veh, frontOnLane, timeOnLane, meanSpeedFrontOnLane, meanSpeedVehicleOnLane, travelledDistanceFrontOnLane, travelledDistanceVehicleOnLane, meanLengthOnLane)
    }

    /// C++ virtual `MeanDataValues::isEmpty`: dispatches to the subclass table
    pub fn is_empty(&self) -> bool {
        (self.data_vt.is_empty)(self)
    }

    /// C++ virtual `MeanDataValues::update`: dispatches to the subclass table
    pub fn update(&mut self) {
        (self.data_vt.update)(self)
    }

    /// C++ virtual `MeanDataValues::write`: dispatches to the subclass table
    pub fn write(&self, dev: &mut OutputDevice, attributeMask: &SumoXMLAttrMask, period: SUMOTime, numLanes: i32, speedLimit: f64, defaultTravelTime: f64, numVehicles: i32) {
        (self.data_vt.write)(self, dev, attributeMask, period, numLanes, speedLimit, defaultTravelTime, numVehicles)
    }

    /// C++ virtual `MeanDataValues::getSamples`: dispatches to the subclass table
    pub fn get_samples(&self) -> f64 {
        (self.data_vt.get_samples)(self)
    }

    /// C++ virtual `MeanDataValues::getAttributeValue`: dispatches to the subclass table
    pub fn get_attribute_value(&self, a: SumoXMLAttr, period: SUMOTime, numLanes: f64, speedLimit: f64) -> f64 {
        (self.data_vt.get_attribute_value)(self, a, period, numLanes, speedLimit)
    }

    /// C++ `MeanDataValues` has no notifyMoveInternal override: the MSMoveReminder body applies (empty)
    pub fn notify_move_internal_base(&mut self, veh: &SUMOTrafficObject, frontOnLane: f64, timeOnLane: f64, meanSpeedFrontOnLane: f64, meanSpeedVehicleOnLane: f64, travelledDistanceFrontOnLane: f64, travelledDistanceVehicleOnLane: f64, meanLengthOnLane: f64) {
        // C++ MSMoveReminder::notifyMoveInternal body only consumes its parameters
    }
}

/// C++ `MSMoveReminder` ctor registration (`if (myLane != nullptr && doAdd) myLane->addMoveReminder(this)`).
/// Rust constructors cannot register, so the creator calls this once the values object has its final address.
pub fn register_values(values: Ptr<MSMeanData_MeanDataValues>, lane: Ptr<MSLane>, doAdd: bool) {
    if doAdd && !lane.is_null() {
        lane.m().add_move_reminder(reminder_of(values), false);
    }
}

/// MSMoveReminder vtable shared by every MeanDataValues object: lanes call it through the base, it forwards to `data_vt`.
static MEAN_DATA_VALUES_MSMOVE_VT: MSMoveReminderVt = MSMoveReminderVt {
    notify_enter: mean_data_values_reminder_enter,
    notify_move: mean_data_values_reminder_move,
    notify_leave: mean_data_values_reminder_leave,
};

fn mean_data_values_reminder_enter(r: &mut MSMoveReminder, veh: &mut SUMOTrafficObject, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
    Ptr::from_mut(r).cast::<MSMeanData_MeanDataValues>().m().notify_enter(veh, reason, enteredLane)
}

fn mean_data_values_reminder_move(r: &mut MSMoveReminder, veh: &mut SUMOTrafficObject, oldPos: f64, newPos: f64, newSpeed: f64) -> bool {
    Ptr::from_mut(r).cast::<MSMeanData_MeanDataValues>().m().notify_move(veh, oldPos, newPos, newSpeed)
}

fn mean_data_values_reminder_leave(r: &mut MSMoveReminder, veh: &mut SUMOTrafficObject, lastPos: f64, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
    Ptr::from_mut(r).cast::<MSMeanData_MeanDataValues>().m().notify_leave(veh, lastPos, reason, enteredLane)
}

/// MSMoveReminder internal table shared by every MeanDataValues object: forwards to `data_vt`.
static MEAN_DATA_VALUES_MSMOVE_INTERNAL_VT: MSMoveReminderInternalVt = MSMoveReminderInternalVt {
    notify_move_internal: mean_data_values_reminder_notify_move_internal,
};

fn mean_data_values_reminder_notify_move_internal(r: &mut MSMoveReminder, veh: &SUMOTrafficObject, frontOnLane: f64, timeOnLane: f64, meanSpeedFrontOnLane: f64, meanSpeedVehicleOnLane: f64, travelledDistanceFrontOnLane: f64, travelledDistanceVehicleOnLane: f64, meanLengthOnLane: f64) {
    Ptr::from_mut(r).cast::<MSMeanData_MeanDataValues>().m().notify_move_internal(veh, frontOnLane, timeOnLane, meanSpeedFrontOnLane, meanSpeedVehicleOnLane, travelledDistanceFrontOnLane, travelledDistanceVehicleOnLane, meanLengthOnLane)
}

/// Base MeanDataValues table: the `*_base` implementations
static MEAN_DATA_VALUES_VT: MeanDataValuesVt = MeanDataValuesVt {
    reset: MSMeanData_MeanDataValues::reset_base,
    add_to: MSMeanData_MeanDataValues::add_to_base,
    notify_enter: MSMeanData_MeanDataValues::notify_enter_base,
    notify_leave: MSMeanData_MeanDataValues::notify_leave_base,
    notify_move_internal: MSMeanData_MeanDataValues::notify_move_internal_base,
    is_empty: MSMeanData_MeanDataValues::is_empty_base,
    update: MSMeanData_MeanDataValues::update_base,
    write: MSMeanData_MeanDataValues::write_base,
    get_samples: MSMeanData_MeanDataValues::get_samples_base,
    get_attribute_value: MSMeanData_MeanDataValues::get_attribute_value_base,
};

/// MeanDataValueTracker table: the tracker overrides everything but update and getAttributeValue
static MEAN_DATA_VALUES_TRACKER_VT: MeanDataValuesVt = MeanDataValuesVt {
    reset: mean_data_values_tracker_reset,
    add_to: mean_data_values_tracker_add_to,
    notify_enter: mean_data_values_tracker_notify_enter,
    notify_leave: mean_data_values_tracker_notify_leave,
    notify_move_internal: mean_data_values_tracker_notify_move_internal,
    is_empty: mean_data_values_tracker_is_empty,
    update: MSMeanData_MeanDataValues::update_base,
    write: mean_data_values_tracker_write,
    get_samples: mean_data_values_tracker_get_samples,
    get_attribute_value: MSMeanData_MeanDataValues::get_attribute_value_base,
};

fn mean_data_values_tracker_reset(v: &mut MSMeanData_MeanDataValues, afterWrite: bool) {
    as_tracker(Ptr::from_mut(v)).m().reset(afterWrite)
}

fn mean_data_values_tracker_add_to(v: &MSMeanData_MeanDataValues, val: &mut MSMeanData_MeanDataValues) {
    as_tracker(Ptr::new(v)).m().add_to(val)
}

fn mean_data_values_tracker_notify_enter(v: &mut MSMeanData_MeanDataValues, veh: &mut SUMOTrafficObject, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
    as_tracker(Ptr::from_mut(v)).m().notify_enter(veh, reason, enteredLane)
}

fn mean_data_values_tracker_notify_leave(v: &mut MSMeanData_MeanDataValues, veh: &mut SUMOTrafficObject, lastPos: f64, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
    as_tracker(Ptr::from_mut(v)).m().notify_leave(veh, lastPos, reason, enteredLane)
}

fn mean_data_values_tracker_notify_move_internal(v: &mut MSMeanData_MeanDataValues, veh: &SUMOTrafficObject, frontOnLane: f64, timeOnLane: f64, meanSpeedFrontOnLane: f64, meanSpeedVehicleOnLane: f64, travelledDistanceFrontOnLane: f64, travelledDistanceVehicleOnLane: f64, meanLengthOnLane: f64) {
    as_tracker(Ptr::from_mut(v)).m().notify_move_internal(veh, frontOnLane, timeOnLane, meanSpeedFrontOnLane, meanSpeedVehicleOnLane, travelledDistanceFrontOnLane, travelledDistanceVehicleOnLane, meanLengthOnLane)
}

fn mean_data_values_tracker_is_empty(v: &MSMeanData_MeanDataValues) -> bool {
    as_tracker(Ptr::new(v)).m().is_empty()
}

fn mean_data_values_tracker_write(v: &MSMeanData_MeanDataValues, dev: &mut OutputDevice, attributeMask: &SumoXMLAttrMask, period: SUMOTime, numLanes: i32, speedLimit: f64, defaultTravelTime: f64, numVehicles: i32) {
    as_tracker(Ptr::new(v)).m().write(dev, attributeMask, period, numLanes, speedLimit, defaultTravelTime, numVehicles)
}

fn mean_data_values_tracker_get_samples(v: &MSMeanData_MeanDataValues) -> f64 {
    as_tracker(Ptr::new(v)).m().get_samples()
}

/// C++ class `MeanDataValueTracker::TrackerEntry` (microsim/output/MSMeanData.h)
pub struct MeanDataValueTracker_TrackerEntry {
    /// The number of vehicles which entered in the current interval
    pub myNumVehicleEntered: std::cell::Cell<i32>, // Cell: entries are shared via Rc and mutated in place (C++ ++ on a shared pointer)
    /// The number of vehicles which left in the current interval
    pub myNumVehicleLeft: std::cell::Cell<i32>,
    /// The number of vehicles which left in the current interval
    pub myValues: Ptr<MSMeanData_MeanDataValues>,
}

impl MeanDataValueTracker_TrackerEntry {
    /// C++: `void TrackerEntry::TrackerEntry(MeanDataValues *const)`
    /// Constructor
    // TrackerEntry(MeanDataValues* const values)
    //                 : myNumVehicleEntered(0), myNumVehicleLeft(0), myValues(values) {}
    #[allow(unused_variables)]
    pub fn new(values: Ptr<MSMeanData_MeanDataValues>) -> Self {
        Self {
            myNumVehicleEntered: std::cell::Cell::new(0),
            myNumVehicleLeft: std::cell::Cell::new(0),
            myValues: values,
        }
    }

    /// C++: `void TrackerEntry::~TrackerEntry() noexcept` [virtual]
    /// Constructor
    // virtual ~TrackerEntry() {
    //                 delete myValues;
    //             }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // shortcut: C++ deletes myValues here; leaked, see file header
    }

}

