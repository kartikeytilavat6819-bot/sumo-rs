//! Port of microsim/output/MSMeanData.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// Data collector for edges/lanes
/// C++ class `MSMeanData` (microsim/output/MSMeanData.h)
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
    pub fn new(id: &str, dumpBegin: SUMOTime, dumpEnd: SUMOTime, useLanes: bool, excludeEmpty: &str, withInternal: bool, trackVehicles: bool, detectPersons: i32, minSamples: f64, maxTravelTime: f64, vTypes: &str, writeAttributes: &str, edges: &Vec<Ptr<MSEdge>>, aggregate: AggregateType) -> Self {
        todo!("port MSMeanData::MSMeanData")
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
        todo!("port MSMeanData::~MSMeanData")
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
        todo!("port MSMeanData::init")
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
    pub fn write_xml_output(&mut self, dev: &mut OutputDevice, startTime: SUMOTime, stopTime: SUMOTime) {
        todo!("port MSMeanData::writeXMLOutput")
    }

    /// C++: `void MSMeanData::writeXMLDetectorProlog(OutputDevice &) const` [const, virtual]
    /// Opens the XML-output using "netstats" as root element
    // void
    // MSMeanData::writeXMLDetectorProlog(OutputDevice& dev) const {
    //     dev.writeXMLHeader("meandata", "meandata_file.xsd");
    // }
    #[allow(unused_variables)]
    pub fn write_xml_detector_prolog(&self, dev: &mut OutputDevice) {
        todo!("port MSMeanData::writeXMLDetectorProlog")
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
        todo!("port MSMeanData::detectorUpdate")
    }

    /// C++: `double MSMeanData::getMinSamples() const` [const]
    // double getMinSamples() const {
    //         return myMinSamples;
    //     }
    #[allow(unused_variables)]
    pub fn get_min_samples(&self) -> f64 {
        todo!("port MSMeanData::getMinSamples")
    }

    /// C++: `double MSMeanData::getMaxTravelTime() const` [const]
    // double getMaxTravelTime() const {
    //         return myMaxTravelTime;
    //     }
    #[allow(unused_variables)]
    pub fn get_max_travel_time(&self) -> f64 {
        todo!("port MSMeanData::getMaxTravelTime")
    }

    /// C++: `bool MSMeanData::isEdgeData() const` [const]
    // bool isEdgeData() const {
    //         return myAmEdgeBased;
    //     }
    #[allow(unused_variables)]
    pub fn is_edge_data(&self) -> bool {
        todo!("port MSMeanData::isEdgeData")
    }

    /// C++: `std::vector<std::string> MSMeanData::getAttributeNames() const` [const, virtual, pure virtual]
    /// return all attributes that are (potentially) written by this output
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_attribute_names(&self) -> Vec<String> {
        todo!("port MSMeanData::getAttributeNames")
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
        todo!("port MSMeanData::getAttributeValue")
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
        todo!("port MSMeanData::getReminders")
    }

    /// C++: `void MSMeanData::setSubData(MSMeanData *)`
    /// set a sub collector
    // void setSubData(MSMeanData* subData) {
    //         mySubData = subData;
    //     }
    #[allow(unused_variables)]
    pub fn set_sub_data(&mut self, subData: Ptr<MSMeanData>) {
        todo!("port MSMeanData::setSubData")
    }

    /// C++: `const MSEdgeVector & MSMeanData::getEdges() const` [const]
    // const MSEdgeVector& getEdges() const {
    //         return myEdges;
    //     }
    #[allow(unused_variables)]
    pub fn get_edges(&self) -> &MSEdgeVector {
        todo!("port MSMeanData::getEdges")
    }

    /// C++: `MSMeanData::MeanDataValues * MSMeanData::createValues(MSLane *const, const double, const bool) const` [const, virtual, pure virtual, protected]
    /// Create an instance of MeanDataValues
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn create_values(&self, lane: Ptr<MSLane>, length: f64, doAdd: bool) -> Ptr<MSMeanData_MeanDataValues> {
        todo!("port MSMeanData::createValues")
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
    pub fn reset_only(&mut self, stopTime: SUMOTime) {
        todo!("port MSMeanData::resetOnly")
    }

    /// C++: `std::string MSMeanData::getEdgeID(const MSEdge *const)` [virtual, protected]
    /// Return the relevant edge id
    // std::string
    // MSMeanData::getEdgeID(const MSEdge* const edge) {
    //     return edge->getID();
    // }
    #[allow(unused_variables)]
    pub fn get_edge_id(&mut self, edge: Ptr<MSEdge>) -> String {
        todo!("port MSMeanData::getEdgeID")
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
        todo!("port MSMeanData::writeEdge")
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
        todo!("port MSMeanData::writeAggregated")
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
        todo!("port MSMeanData::writeAggregatedTAZ")
    }

    /// C++: `void MSMeanData::openInterval(OutputDevice &, const SUMOTime, const SUMOTime)` [virtual, protected]
    /// Writes the interval opener
    // void
    // MSMeanData::openInterval(OutputDevice& dev, const SUMOTime startTime, const SUMOTime stopTime) {
    //     dev.openTag(SUMO_TAG_INTERVAL).writeAttr(SUMO_ATTR_BEGIN, time2string(startTime)).writeAttr(SUMO_ATTR_END, time2string(stopTime));
    //     dev.writeAttr(SUMO_ATTR_ID, myID);
    // }
    #[allow(unused_variables)]
    pub fn open_interval(&mut self, dev: &mut OutputDevice, startTime: SUMOTime, stopTime: SUMOTime) {
        todo!("port MSMeanData::openInterval")
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
    pub fn write_prefix(&self, dev: &mut OutputDevice, values: &MSMeanData_MeanDataValues, tag: SumoXMLTag, id: String) {
        todo!("port MSMeanData::writePrefix")
    }

    /// C++: `const std::vector<MeanDataValues *> * MSMeanData::getEdgeValues(const MSEdge *) const` [const, protected]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_edge_values(&self, edge: Ptr<MSEdge>) -> Ptr<Vec<Ptr<MSMeanData_MeanDataValues>>> {
        todo!("port MSMeanData::getEdgeValues")
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
        todo!("port MSMeanData::createValuesSubData")
    }

    /// C++: `void MSMeanData::MSMeanData(const MSMeanData &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSMeanData) -> Self {
        todo!("port MSMeanData::MSMeanData")
    }

    /// C++: `MSMeanData & MSMeanData::operator=(const MSMeanData &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSMeanData) -> &mut MSMeanData {
        todo!("port MSMeanData::operator=")
    }

}

/// Data structure for mean (aggregated) edge/lane values for tracked vehicles
/// C++ class `MSMeanData::MeanDataValueTracker` (microsim/output/MSMeanData.h)
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
        todo!("port MeanDataValueTracker::MeanDataValueTracker")
    }

    /// C++: `void MeanDataValueTracker::~MeanDataValueTracker() noexcept` [virtual]
    /// Destructor
    // MSMeanData::MeanDataValueTracker::~MeanDataValueTracker() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MeanDataValueTracker::~MeanDataValueTracker")
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
        todo!("port MeanDataValueTracker::reset")
    }

    /// C++: `void MeanDataValueTracker::addTo(MSMeanData::MeanDataValues &) const` [const, virtual]
    /// Add the values of this to the given one and store them there
    // void
    // MSMeanData::MeanDataValueTracker::addTo(MSMeanData::MeanDataValues& val) const {
    //     myCurrentData.front()->myValues->addTo(val);
    // }
    #[allow(unused_variables)]
    pub fn add_to(&self, val: &mut MSMeanData_MeanDataValues) {
        todo!("port MeanDataValueTracker::addTo")
    }

    /// C++: `void MeanDataValueTracker::notifyMoveInternal(const SUMOTrafficObject &, const double, const double, const double, const double, const double, const double, const double)` [virtual]
    /// Internal notification about the vehicle moves
    // void
    // MSMeanData::MeanDataValueTracker::notifyMoveInternal(const SUMOTrafficObject& veh, const double frontOnLane, const double timeOnLane, const double meanSpeedFrontOnLane, const double meanSpeedVehicleOnLane, const double travelledDistanceFrontOnLane, const double travelledDistanceVehicleOnLane, const double meanLengthOnLane) {
    //     myTrackedData[&veh]->myValues->notifyMoveInternal(veh, frontOnLane, timeOnLane, meanSpeedFrontOnLane, meanSpeedVehicleOnLane, travelledDistanceFrontOnLane, travelledDistanceVehicleOnLane, meanLengthOnLane);
    // }
    #[allow(unused_variables)]
    pub fn notify_move_internal(&mut self, veh: &SUMOTrafficObject, frontOnLane: f64, timeOnLane: f64, meanSpeedFrontOnLane: f64, meanSpeedVehicleOnLane: f64, travelledDistanceFrontOnLane: f64, travelledDistanceVehicleOnLane: f64, meanLengthOnLane: f64) {
        todo!("port MeanDataValueTracker::notifyMoveInternal")
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
        todo!("port MeanDataValueTracker::notifyLeave")
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
        todo!("port MeanDataValueTracker::notifyEnter")
    }

    /// C++: `bool MeanDataValueTracker::isEmpty() const` [const, virtual]
    // bool
    // MSMeanData::MeanDataValueTracker::isEmpty() const {
    //     return myCurrentData.front()->myValues->isEmpty();
    // }
    #[allow(unused_variables)]
    pub fn is_empty(&self) -> bool {
        todo!("port MeanDataValueTracker::isEmpty")
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
        todo!("port MeanDataValueTracker::write")
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
        todo!("port MeanDataValueTracker::getNumReady")
    }

    /// C++: `void MeanDataValueTracker::clearFirst()`
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn clear_first(&mut self) {
        todo!("port MeanDataValueTracker::clearFirst")
    }

    /// C++: `double MeanDataValueTracker::getSamples() const` [const, virtual]
    // double
    // MSMeanData::MeanDataValueTracker::getSamples() const {
    //     return myCurrentData.front()->myValues->getSamples();
    // }
    #[allow(unused_variables)]
    pub fn get_samples(&self) -> f64 {
        todo!("port MeanDataValueTracker::getSamples")
    }

}

/// Data structure for mean (aggregated) edge/lane values
/// C++ class `MSMeanData::MeanDataValues` (microsim/output/MSMeanData.h)
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
        todo!("port MeanDataValues::MeanDataValues")
    }

    /// C++: `void MeanDataValues::~MeanDataValues() noexcept` [virtual]
    /// Destructor
    // MSMeanData::MeanDataValues::~MeanDataValues() {
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MeanDataValues::~MeanDataValues")
    }

    /// C++: `void MeanDataValues::reset(bool)` [virtual, pure virtual]
    /// Resets values so they may be used for the next interval
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn reset(&mut self, afterWrite: bool) {
        todo!("port MeanDataValues::reset")
    }

    /// C++: `void MeanDataValues::addTo(MeanDataValues &) const` [const, virtual, pure virtual]
    /// Add the values of this to the given one and store them there
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn add_to(&self, val: &mut MSMeanData_MeanDataValues) {
        todo!("port MeanDataValues::addTo")
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
    pub fn notify_enter(&mut self, veh: &mut SUMOTrafficObject, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
        todo!("port MeanDataValues::notifyEnter")
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
        todo!("port MeanDataValues::notifyMove")
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
    pub fn notify_leave(&mut self, veh: &mut SUMOTrafficObject, lastPos: f64, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
        todo!("port MeanDataValues::notifyLeave")
    }

    /// C++: `bool MeanDataValues::isEmpty() const` [const, virtual]
    /// Returns whether any data was collected.
    // bool
    // MSMeanData::MeanDataValues::isEmpty() const {
    //     return sampleSeconds == 0;
    // }
    #[allow(unused_variables)]
    pub fn is_empty(&self) -> bool {
        todo!("port MeanDataValues::isEmpty")
    }

    /// C++: `void MeanDataValues::update()` [virtual]
    /// Called if a per timestep update is needed. Default does nothing.
    // void
    // MSMeanData::MeanDataValues::update() {
    // }
    #[allow(unused_variables)]
    pub fn update(&mut self) {
        todo!("port MeanDataValues::update")
    }

    /// C++: `void MeanDataValues::write(OutputDevice &, const SumoXMLAttrMask &, const SUMOTime, const int, const double, const double, const int) const` [const, virtual, pure virtual]
    /// Writes output values into the given stream
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn write(&self, dev: &mut OutputDevice, attributeMask: &SumoXMLAttrMask, period: SUMOTime, numLanes: i32, speedLimit: f64, defaultTravelTime: f64, numVehicles: i32) {
        todo!("port MeanDataValues::write")
    }

    /// C++: `double MeanDataValues::getSamples() const` [const, virtual]
    /// Returns the number of collected sample seconds.
    // double
    // MSMeanData::MeanDataValues::getSamples() const {
    //     return sampleSeconds;
    // }
    #[allow(unused_variables)]
    pub fn get_samples(&self) -> f64 {
        todo!("port MeanDataValues::getSamples")
    }

    /// C++: `double MeanDataValues::getTravelledDistance() const` [const]
    /// Returns the total travelled distance.
    // double getTravelledDistance() const {
    //             return travelledDistance;
    //         }
    #[allow(unused_variables)]
    pub fn get_travelled_distance(&self) -> f64 {
        todo!("port MeanDataValues::getTravelledDistance")
    }

    /// C++: `SUMOTime MeanDataValues::getResetTime() const` [const]
    // SUMOTime getResetTime() const {
    //             return resetTime;
    //         }
    #[allow(unused_variables)]
    pub fn get_reset_time(&self) -> SUMOTime {
        todo!("port MeanDataValues::getResetTime")
    }

    /// C++: `double MeanDataValues::getLaneLength() const` [const]
    // double getLaneLength() const {
    //             return myLaneLength;
    //         }
    #[allow(unused_variables)]
    pub fn get_lane_length(&self) -> f64 {
        todo!("port MeanDataValues::getLaneLength")
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
    pub fn get_attribute_value(&self, a: SumoXMLAttr, period: SUMOTime, numLanes: f64, speedLimit: f64) -> f64 {
        todo!("port MeanDataValues::getAttributeValue")
    }

    /// C++: `MeanDataValues * MeanDataValues::getSubData()`
    // MeanDataValues* getSubData() {
    //             return mySubData;
    //         }
    #[allow(unused_variables)]
    pub fn get_sub_data(&mut self) -> Ptr<MSMeanData_MeanDataValues> {
        todo!("port MeanDataValues::getSubData")
    }

    /// C++: `void MeanDataValues::setSubData(MeanDataValues *)`
    // void setSubData(MeanDataValues* subData) {
    //             mySubData = subData;
    //         }
    #[allow(unused_variables)]
    pub fn set_sub_data(&mut self, subData: Ptr<MSMeanData_MeanDataValues>) {
        todo!("port MeanDataValues::setSubData")
    }

}

/// C++ class `MeanDataValueTracker::TrackerEntry` (microsim/output/MSMeanData.h)
pub struct MeanDataValueTracker_TrackerEntry {
    /// The number of vehicles which entered in the current interval
    pub myNumVehicleEntered: i32,
    /// The number of vehicles which left in the current interval
    pub myNumVehicleLeft: i32,
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
        todo!("port TrackerEntry::TrackerEntry")
    }

    /// C++: `void TrackerEntry::~TrackerEntry() noexcept` [virtual]
    /// Constructor
    // virtual ~TrackerEntry() {
    //                 delete myValues;
    //             }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port TrackerEntry::~TrackerEntry")
    }

}

