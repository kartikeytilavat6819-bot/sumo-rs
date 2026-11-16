//! Port of microsim/output/MSMeanData_Amitran.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// Data structure for mean (aggregated) edge/lane values
/// C++ class `MSMeanData_Amitran::MSLaneMeanDataValues` (microsim/output/MSMeanData_Amitran.h)
#[repr(C)] // base must be field 0 for the C++ downcast in addTo
pub struct MSMeanData_Amitran_MSLaneMeanDataValues {
    /// C++ base class `MSMeanData::MeanDataValues`
    pub base: MSMeanData_MeanDataValues,
    /// The number of vehicles that entered this lane within the sample interval
    pub amount: i32,
    /// The number of vehicles that entered this lane within the sample interval by type
    pub typedAmount: BTreeMap<Ptr<MSVehicleType>, i32>,
    /// The number of sampled vehicle movements by type (in s)
    pub typedSamples: BTreeMap<Ptr<MSVehicleType>, f64>,
    /// The sum of the distances the vehicles travelled by type
    pub typedTravelDistance: BTreeMap<Ptr<MSVehicleType>, f64>,
}

impl std::ops::Deref for MSMeanData_Amitran_MSLaneMeanDataValues {
    type Target = MSMeanData_MeanDataValues;
    fn deref(&self) -> &MSMeanData_MeanDataValues {
        &self.base
    }
}
impl std::ops::DerefMut for MSMeanData_Amitran_MSLaneMeanDataValues {
    fn deref_mut(&mut self) -> &mut MSMeanData_MeanDataValues {
        &mut self.base
    }
}

impl MSMeanData_Amitran_MSLaneMeanDataValues {
    /// C++: `void MSLaneMeanDataValues::MSLaneMeanDataValues(MSLane *const, const double, const bool, const MSMeanData_Amitran *)`
    /// Constructor
    // MSMeanData_Amitran::MSLaneMeanDataValues::MSLaneMeanDataValues(MSLane* const lane,
    //         const double length,
    //         const bool doAdd,
    //         const MSMeanData_Amitran* parent)
    //     : MSMeanData::MeanDataValues(lane, length, doAdd, parent), amount(0) {}
    #[allow(unused_variables)]
    pub fn new(lane: Ptr<MSLane>, length: f64, doAdd: bool, parent: Ptr<MSMeanData_Amitran>) -> Self {
        let mut base = MSMeanData_MeanDataValues::new(lane, length, doAdd, Ptr::from_mut(&mut parent.m().base));
        base.data_vt = &MS_MEAN_DATA_VALUES_AMITRAN_VT;
        MSMeanData_Amitran_MSLaneMeanDataValues {
            base,
            amount: 0,
            typedAmount: BTreeMap::new(),
            typedSamples: BTreeMap::new(),
            typedTravelDistance: BTreeMap::new(),
        }
    }

    /// C++: `void MSLaneMeanDataValues::~MSLaneMeanDataValues() noexcept` [virtual]
    /// Destructor
    // MSMeanData_Amitran::MSLaneMeanDataValues::~MSLaneMeanDataValues() {
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
    }

    /// C++: `void MSLaneMeanDataValues::reset(bool)` [virtual]
    /// Resets values so they may be used for the next interval
    // void
    // MSMeanData_Amitran::MSLaneMeanDataValues::reset(bool) {
    //     amount = 0;
    //     typedAmount.clear();
    //     typedSamples.clear();
    //     typedTravelDistance.clear();
    //     resetTime = SIMSTEP;
    // }
    #[allow(unused_variables)]
    pub fn reset(&mut self, afterWrite: bool) {
        self.amount = 0;
        self.typedAmount.clear();
        self.typedSamples.clear();
        self.typedTravelDistance.clear();
        self.resetTime = MSNet::get_instance().get_current_time_step();
    }

    /// C++: `void MSLaneMeanDataValues::addTo(MSMeanData::MeanDataValues &) const` [const, virtual]
    /// Add the values of this to the given one and store them there
    // void
    // MSMeanData_Amitran::MSLaneMeanDataValues::addTo(MSMeanData::MeanDataValues& val) const {
    //     MSLaneMeanDataValues& v = (MSLaneMeanDataValues&) val;
    //     v.amount += amount;
    //     v.sampleSeconds += sampleSeconds;
    //     v.travelledDistance += travelledDistance;
    //     for (std::map<const MSVehicleType*, int>::const_iterator it = typedAmount.begin(); it != typedAmount.end(); ++it) {
    //         v.typedAmount[it->first] += it->second;
    //     }
    //     for (std::map<const MSVehicleType*, double>::const_iterator it = typedSamples.begin(); it != typedSamples.end(); ++it) {
    //         v.typedSamples[it->first] += it->second;
    //     }
    //     for (std::map<const MSVehicleType*, double>::const_iterator it = typedTravelDistance.begin(); it != typedTravelDistance.end(); ++it) {
    //         v.typedTravelDistance[it->first] += it->second;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_to(&self, val: &mut MSMeanData_MeanDataValues) {
        // C++ downcast; `base` is the first field (repr(C))
        let v = unsafe { &mut *(val as *mut MSMeanData_MeanDataValues as *mut MSMeanData_Amitran_MSLaneMeanDataValues) };
        v.amount += self.amount;
        v.sampleSeconds += self.sampleSeconds;
        v.travelledDistance += self.travelledDistance;
        for (vtype, amount) in &self.typedAmount {
            *v.typedAmount.entry(*vtype).or_insert(0) += *amount;
        }
        for (vtype, samples) in &self.typedSamples {
            *v.typedSamples.entry(*vtype).or_insert(0.0) += *samples;
        }
        for (vtype, distance) in &self.typedTravelDistance {
            *v.typedTravelDistance.entry(*vtype).or_insert(0.0) += *distance;
        }
    }

    /// C++: `bool MSLaneMeanDataValues::notifyEnter(SUMOTrafficObject &, MSMoveReminder::Notification, const MSLane *)` [virtual]
    /// Computes current values and adds them to their sums
    // bool
    // MSMeanData_Amitran::MSLaneMeanDataValues::notifyEnter(SUMOTrafficObject& veh, MSMoveReminder::Notification reason, const MSLane* /* enteredLane */) {
    //     if (myParent->vehicleApplies(veh)) {
    //         if (getLane() == nullptr || getLane() == static_cast<MSVehicle&>(veh).getLane()) {
    //             if (reason == MSMoveReminder::NOTIFICATION_DEPARTED || reason == MSMoveReminder::NOTIFICATION_JUNCTION) {
    //                 ++amount;
    //                 typedAmount[&veh.getVehicleType()]++;
    //             }
    //         }
    //         return true;
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn notify_enter(&mut self, veh: &mut SUMOTrafficObject, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
        if self.myParent.vehicle_applies(veh) {
            if self.get_lane().is_null() || self.get_lane() == veh.get_lane() {
                if reason == MSMoveReminder_Notification::NOTIFICATION_DEPARTED || reason == MSMoveReminder_Notification::NOTIFICATION_JUNCTION {
                    self.amount += 1;
                    *self.typedAmount.entry(Ptr::new(veh.get_vehicle_type())).or_insert(0) += 1;
                }
            }
            return true;
        }
        false
    }

    /// C++: `bool MSLaneMeanDataValues::isEmpty() const` [const, virtual]
    // bool
    // MSMeanData_Amitran::MSLaneMeanDataValues::isEmpty() const {
    //     return sampleSeconds == 0 && amount == 0;
    // }
    #[allow(unused_variables)]
    pub fn is_empty(&self) -> bool {
        self.sampleSeconds == 0.0 && self.amount == 0
    }

    /// C++: `void MSLaneMeanDataValues::write(OutputDevice &, const SumoXMLAttrMask &, const SUMOTime, const int, const double, const double, const int) const` [const, virtual]
    /// Writes output values into the given stream
    // void
    // MSMeanData_Amitran::MSLaneMeanDataValues::write(OutputDevice& dev, const SumoXMLAttrMask& attributeMask, const SUMOTime /* period */,
    //         const int /* numLanes */, const double /*speedLimit*/, const double defaultTravelTime, const int /* numVehicles */) const {
    //     int averageSpeed;
    //     if (sampleSeconds > 0) {
    //         averageSpeed = int(100 * travelledDistance / sampleSeconds);
    //     } else if (defaultTravelTime >= 0.) {
    //         averageSpeed = int(100 * myLaneLength / defaultTravelTime);
    //     } else {
    //         averageSpeed = -1;
    //     }
    //     dev.writeOptionalAttr(SUMO_ATTR_AMOUNT, amount, attributeMask);
    //     dev.writeOptionalAttr(SUMO_ATTR_AVERAGESPEED, averageSpeed, attributeMask);
    //
    //     if (myParent->isTyped()) {
    //         for (std::map<const MSVehicleType*, int>::const_iterator it = typedAmount.begin(); it != typedAmount.end(); ++it) {
    //             dev.openTag("actorConfig");
    //             const int typedAvgSpeed = int(100 * typedTravelDistance.find(it->first)->second / typedSamples.find(it->first)->second);
    //             dev.writeAttr(SUMO_ATTR_ID, it->first->getNumericalID());
    //             dev.writeOptionalAttr(SUMO_ATTR_AMOUNT, it->second, attributeMask);
    //             dev.writeOptionalAttr(SUMO_ATTR_AVERAGESPEED, typedAvgSpeed, attributeMask);
    //             dev.closeTag();
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn write(&self, dev: &mut OutputDevice, attributeMask: &SumoXMLAttrMask, period: SUMOTime, numLanes: i32, speedLimit: f64, defaultTravelTime: f64, numVehicles: i32) {
        let average_speed: i32 = if self.sampleSeconds > 0.0 {
            (100.0 * self.travelledDistance / self.sampleSeconds) as i32
        } else if defaultTravelTime >= 0.0 {
            (100.0 * self.myLaneLength / defaultTravelTime) as i32
        } else {
            -1
        };
        dev.write_optional_attr_mask(SUMO_ATTR_AMOUNT, &self.amount, *attributeMask, false);
        dev.write_optional_attr_mask(SUMO_ATTR_AVERAGESPEED, &average_speed, *attributeMask, false);

        if self.myParent.is_typed() {
            for (vtype, amount) in &self.typedAmount {
                dev.open_tag("actorConfig");
                let typedAvgSpeed = (100.0 * self.typedTravelDistance[vtype] / self.typedSamples[vtype]) as i32;
                dev.write_attr(SUMO_ATTR_ID, &vtype.get_numerical_id());
                dev.write_optional_attr_mask(SUMO_ATTR_AMOUNT, amount, *attributeMask, false);
                dev.write_optional_attr_mask(SUMO_ATTR_AVERAGESPEED, &typedAvgSpeed, *attributeMask, false);
                dev.close_tag("");
            }
        }
    }

    /// C++: `void MSLaneMeanDataValues::notifyMoveInternal(const SUMOTrafficObject &, const double, const double, const double, const double, const double, const double, const double)` [virtual, protected]
    /// Internal notification about the vehicle moves
    // void
    // MSMeanData_Amitran::MSLaneMeanDataValues::notifyMoveInternal(const SUMOTrafficObject& veh, const double /* frontOnLane */, const double timeOnLane, const double /*meanSpeedFrontOnLane*/, const double /*meanSpeedVehicleOnLane*/, const double /*travelledDistanceFrontOnLane*/, const double travelledDistanceVehicleOnLane, const double /* meanLengthOnLane */) {
    //     sampleSeconds += timeOnLane;
    //     travelledDistance += travelledDistanceVehicleOnLane;
    //     typedSamples[&veh.getVehicleType()] += timeOnLane;
    //     typedTravelDistance[&veh.getVehicleType()] += travelledDistanceVehicleOnLane;
    // }
    #[allow(unused_variables)]
    pub fn notify_move_internal(&mut self, veh: &SUMOTrafficObject, arg1: f64, timeOnLane: f64, arg3: f64, meanSpeedVehicleOnLane: f64, travelledDistanceFrontOnLane: f64, travelledDistanceVehicleOnLane: f64, arg7: f64) {
        self.sampleSeconds += timeOnLane;
        self.travelledDistance += travelledDistanceVehicleOnLane;
        *self.typedSamples.entry(Ptr::new(veh.get_vehicle_type())).or_insert(0.0) += timeOnLane;
        *self.typedTravelDistance.entry(Ptr::new(veh.get_vehicle_type())).or_insert(0.0) += travelledDistanceVehicleOnLane;
    }

}

/// Network state mean data collector for edges/lanes
/// Virtual dispatch table of MSMeanData_Amitran::MSLaneMeanDataValues (overrides reset, addTo, notifyEnter, notifyMoveInternal, isEmpty, write)
static MS_MEAN_DATA_VALUES_AMITRAN_VT: MeanDataValuesVt = MeanDataValuesVt {
    reset: amitran_values_reset,
    add_to: amitran_values_add_to,
    notify_enter: amitran_values_notify_enter,
    notify_leave: MSMeanData_MeanDataValues::notify_leave_base,
    notify_move_internal: amitran_values_notify_move_internal,
    is_empty: amitran_values_is_empty,
    update: MSMeanData_MeanDataValues::update_base,
    write: amitran_values_write,
    get_samples: MSMeanData_MeanDataValues::get_samples_base,
    get_attribute_value: MSMeanData_MeanDataValues::get_attribute_value_base,
};

fn amitran_values_reset(v: &mut MSMeanData_MeanDataValues, afterWrite: bool) {
    Ptr::from_mut(v).cast::<MSMeanData_Amitran_MSLaneMeanDataValues>().m().reset(afterWrite)
}

fn amitran_values_add_to(v: &MSMeanData_MeanDataValues, val: &mut MSMeanData_MeanDataValues) {
    Ptr::new(v).cast::<MSMeanData_Amitran_MSLaneMeanDataValues>().m().add_to(val)
}

fn amitran_values_notify_enter(v: &mut MSMeanData_MeanDataValues, veh: &mut SUMOTrafficObject, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
    Ptr::from_mut(v).cast::<MSMeanData_Amitran_MSLaneMeanDataValues>().m().notify_enter(veh, reason, enteredLane)
}

fn amitran_values_notify_move_internal(v: &mut MSMeanData_MeanDataValues, veh: &SUMOTrafficObject, frontOnLane: f64, timeOnLane: f64, meanSpeedFrontOnLane: f64, meanSpeedVehicleOnLane: f64, travelledDistanceFrontOnLane: f64, travelledDistanceVehicleOnLane: f64, meanLengthOnLane: f64) {
    Ptr::from_mut(v).cast::<MSMeanData_Amitran_MSLaneMeanDataValues>().m().notify_move_internal(veh, frontOnLane, timeOnLane, meanSpeedFrontOnLane, meanSpeedVehicleOnLane, travelledDistanceFrontOnLane, travelledDistanceVehicleOnLane, meanLengthOnLane)
}

fn amitran_values_is_empty(v: &MSMeanData_MeanDataValues) -> bool {
    Ptr::new(v).cast::<MSMeanData_Amitran_MSLaneMeanDataValues>().m().is_empty()
}

fn amitran_values_write(v: &MSMeanData_MeanDataValues, dev: &mut OutputDevice, attributeMask: &SumoXMLAttrMask, period: SUMOTime, numLanes: i32, speedLimit: f64, defaultTravelTime: f64, numVehicles: i32) {
    Ptr::new(v).cast::<MSMeanData_Amitran_MSLaneMeanDataValues>().m().write(dev, attributeMask, period, numLanes, speedLimit, defaultTravelTime, numVehicles)
}

/// Virtual dispatch table of MSMeanData_Amitran (overrides writeXMLDetectorProlog, getEdgeID, openInterval, writePrefix, getAttributeNames, createValues)
static MS_MEAN_DATA_AMITRAN_VT: MSMeanDataVt = MSMeanDataVt {
    create_values: amitran_mean_data_create_values,
    get_attribute_names: amitran_mean_data_get_attribute_names,
    write_xml_detector_prolog: amitran_mean_data_write_xml_detector_prolog,
    get_edge_id: amitran_mean_data_get_edge_id,
    open_interval: amitran_mean_data_open_interval,
    write_prefix: amitran_mean_data_write_prefix,
    reset_only: MSMeanData::reset_only_base,
};

fn amitran_mean_data_create_values(m: &MSMeanData, lane: Ptr<MSLane>, length: f64, doAdd: bool) -> Ptr<MSMeanData_MeanDataValues> {
    Ptr::new(m).cast::<MSMeanData_Amitran>().m().create_values(lane, length, doAdd)
}

fn amitran_mean_data_get_attribute_names(m: &MSMeanData) -> Vec<String> {
    Ptr::new(m).cast::<MSMeanData_Amitran>().m().get_attribute_names()
}

fn amitran_mean_data_write_xml_detector_prolog(m: &MSMeanData, dev: &mut OutputDevice) {
    Ptr::new(m).cast::<MSMeanData_Amitran>().m().write_xml_detector_prolog(dev)
}

fn amitran_mean_data_get_edge_id(m: &mut MSMeanData, edge: Ptr<MSEdge>) -> String {
    Ptr::from_mut(m).cast::<MSMeanData_Amitran>().m().get_edge_id(edge)
}

fn amitran_mean_data_open_interval(m: &mut MSMeanData, dev: &mut OutputDevice, startTime: SUMOTime, stopTime: SUMOTime) {
    Ptr::from_mut(m).cast::<MSMeanData_Amitran>().m().open_interval(dev, startTime, stopTime)
}

fn amitran_mean_data_write_prefix(m: &MSMeanData, dev: &mut OutputDevice, values: &MSMeanData_MeanDataValues, tag: SumoXMLTag, id: String) {
    Ptr::new(m).cast::<MSMeanData_Amitran>().m().write_prefix(dev, values, tag, id)
}

/// C++ class `MSMeanData_Amitran` (microsim/output/MSMeanData_Amitran.h)
#[repr(C)]
pub struct MSMeanData_Amitran {
    /// C++ base class `MSMeanData`
    pub base: MSMeanData,
    /// the minimum sample seconds
    pub myHaltSpeed: f64,
}

impl std::ops::Deref for MSMeanData_Amitran {
    type Target = MSMeanData;
    fn deref(&self) -> &MSMeanData {
        &self.base
    }
}
impl std::ops::DerefMut for MSMeanData_Amitran {
    fn deref_mut(&mut self) -> &mut MSMeanData {
        &mut self.base
    }
}

impl MSMeanData_Amitran {
    /// C++: `void MSMeanData_Amitran::MSMeanData_Amitran(const std::string &, const SUMOTime, const SUMOTime, const bool, const std::string &, const bool, const bool, const int, const double, const double, const double, const std::string &, const std::string &, const std::vector<MSEdge *> &, AggregateType)`
    /// Constructor
    // MSMeanData_Amitran::MSMeanData_Amitran(const std::string& id,
    //                                        const SUMOTime dumpBegin,
    //                                        const SUMOTime dumpEnd, const bool useLanes,
    //                                        const std::string& excludeEmpty,
    //                                        const bool withInternal,
    //                                        const bool trackVehicles,
    //                                        const int detectPersons,
    //                                        const double maxTravelTime,
    //                                        const double minSamples,
    //                                        const double haltSpeed,
    //                                        const std::string& vTypes,
    //                                        const std::string& writeAttributes,
    //                                        const std::vector<MSEdge*>& edges,
    //                                        AggregateType aggregate) :
    //     MSMeanData(id, dumpBegin, dumpEnd, useLanes, excludeEmpty,
    //                withInternal, trackVehicles, detectPersons, maxTravelTime, minSamples, vTypes, writeAttributes, edges, aggregate),
    //     myHaltSpeed(haltSpeed) {
    // }
    #[allow(unused_variables)]
    pub fn new(id: &str, dumpBegin: SUMOTime, dumpEnd: SUMOTime, useLanes: bool, excludeEmpty: &str, withInternal: bool, trackVehicles: bool, detectPersons: i32, maxTravelTime: f64, minSamples: f64, haltSpeed: f64, vTypes: &str, writeAttributes: &str, edges: &Vec<Ptr<MSEdge>>, aggregate: AggregateType) -> Self {
        let mut base = MSMeanData::new(id, dumpBegin, dumpEnd, useLanes, excludeEmpty, withInternal, trackVehicles, detectPersons, maxTravelTime, minSamples, vTypes, writeAttributes, edges, aggregate);
        base.meandata_vt = &MS_MEAN_DATA_AMITRAN_VT;
        Self {
            base,
            myHaltSpeed: haltSpeed,
        }
    }

    /// C++: `void MSMeanData_Amitran::~MSMeanData_Amitran() noexcept` [virtual]
    /// Destructor
    // MSMeanData_Amitran::~MSMeanData_Amitran() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
    }

    /// C++: `void MSMeanData_Amitran::writeXMLDetectorProlog(OutputDevice &) const` [const, virtual]
    /// Opens the XML-output using "netstats" as root element
    // void
    // MSMeanData_Amitran::writeXMLDetectorProlog(OutputDevice& dev) const {
    //     dev.writeXMLHeader("linkData", "amitran/linkdata.xsd");
    // }
    #[allow(unused_variables)]
    pub fn write_xml_detector_prolog(&self, dev: &mut OutputDevice) {
        dev.write_xml_header("linkData", "amitran/linkdata.xsd", BTreeMap::new(), true);
    }

    /// C++: `std::string MSMeanData_Amitran::getEdgeID(const MSEdge *const)` [virtual]
    /// Return the relevant edge id
    // std::string
    // MSMeanData_Amitran::getEdgeID(const MSEdge* const edge) {
    //     return toString(edge->getNumericalID());
    // }
    #[allow(unused_variables)]
    pub fn get_edge_id(&mut self, edge: Ptr<MSEdge>) -> String {
        edge.get_numerical_id().to_string()
    }

    /// C++: `void MSMeanData_Amitran::openInterval(OutputDevice &, const SUMOTime, const SUMOTime)` [virtual]
    /// Writes the interval opener
    // void
    // MSMeanData_Amitran::openInterval(OutputDevice& dev, const SUMOTime startTime, const SUMOTime stopTime) {
    //     const int duration = int(1000 * STEPS2TIME(stopTime - startTime) + 0.5);
    //     dev.openTag(SUMO_TAG_TIMESLICE).writeAttr(SUMO_ATTR_STARTTIME, int(1000 * STEPS2TIME(startTime) + 0.5)).writeAttr(SUMO_ATTR_DURATION, duration);
    // }
    #[allow(unused_variables)]
    pub fn open_interval(&mut self, dev: &mut OutputDevice, startTime: SUMOTime, stopTime: SUMOTime) {
        let duration = (1000.0 * steps2time(stopTime - startTime) + 0.5) as i32;
        dev.open_tag(Tags.get_string(SUMO_TAG_TIMESLICE))
            .write_attr(SUMO_ATTR_STARTTIME, &((1000.0 * steps2time(startTime) + 0.5) as i32))
            .write_attr(SUMO_ATTR_DURATION, &duration);
    }

    /// C++: `void MSMeanData_Amitran::writePrefix(OutputDevice &, const MeanDataValues &, const SumoXMLTag, const std::string) const` [const, virtual]
    /// Writes the surrounding element into the given stream
    // void
    // MSMeanData_Amitran::writePrefix(OutputDevice& dev, const MeanDataValues& /*values*/, const SumoXMLTag /* tag */, const std::string id) const {
    //     dev.openTag("link").writeAttr(SUMO_ATTR_ID, id);
    // }
    #[allow(unused_variables)]
    pub fn write_prefix(&self, dev: &mut OutputDevice, values: &MSMeanData_MeanDataValues, tag: SumoXMLTag, id: String) {
        dev.open_tag("link").write_attr(SUMO_ATTR_ID, id.as_str());
    }

    /// C++: `std::vector<std::string> MSMeanData_Amitran::getAttributeNames() const` [const, virtual]
    /// return all attributes that are (potentially) written by this output
    // std::vector<std::string>
    // MSMeanData_Amitran::getAttributeNames() const {
    //     return std::vector<std::string> {toString(SUMO_ATTR_AMOUNT), toString(SUMO_ATTR_AVERAGESPEED)};
    // }
    #[allow(unused_variables)]
    pub fn get_attribute_names(&self) -> Vec<String> {
        vec![
            utils::xml::sumo_xml_definitions::Attrs.get_string(SUMO_ATTR_AMOUNT).to_string(),
            utils::xml::sumo_xml_definitions::Attrs.get_string(SUMO_ATTR_AVERAGESPEED).to_string(),
        ]
    }

    /// C++: `MSMeanData::MeanDataValues * MSMeanData_Amitran::createValues(MSLane *const, const double, const bool) const` [const, virtual, protected]
    /// Create an instance of MeanDataValues
    // MSMeanData::MeanDataValues*
    // MSMeanData_Amitran::createValues(MSLane* const lane, const double length, const bool doAdd) const {
    //     return new MSLaneMeanDataValues(lane, length, doAdd, this);
    // }
    #[allow(unused_variables)]
    pub fn create_values(&self, lane: Ptr<MSLane>, length: f64, doAdd: bool) -> Ptr<MSMeanData_MeanDataValues> {
        // shortcut: the values are leaked (C++ deletes them in ~MSMeanData), upgrade when myMeasures owns its values
        let values = Box::leak(Box::new(MSMeanData_Amitran_MSLaneMeanDataValues::new(lane, length, doAdd, Ptr::new(self))));
        let values = Ptr::from_mut(&mut values.base);
        register_values(values, lane, doAdd);
        values
    }

    /// C++: `void MSMeanData_Amitran::resetOnly(SUMOTime)` [protected]
    /// Resets network value in order to allow processing of the next interval
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn reset_only(&mut self, stopTime: SUMOTime) {
        // not overridden in C++ MSMeanData_Amitran: inherited behaviour
        self.base.reset_only(stopTime);
    }

    /// C++: `void MSMeanData_Amitran::MSMeanData_Amitran(const MSMeanData_Amitran &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSMeanData_Amitran) -> Self {
        // C++ copy constructor is private and invalidated (never called)
        panic!("MSMeanData_Amitran copy constructor is invalidated");
    }

    /// C++: `MSMeanData_Amitran & MSMeanData_Amitran::operator=(const MSMeanData_Amitran &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSMeanData_Amitran) -> &mut MSMeanData_Amitran {
        // C++ assignment operator is private and invalidated (never called)
        panic!("MSMeanData_Amitran assignment operator is invalidated");
    }

}

