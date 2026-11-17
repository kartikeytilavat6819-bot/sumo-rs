//! Port of microsim/output/MSMeanData_Harmonoise.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use super::ms_detector_file_output::{DEFAULT_MSDETECTOR_FILE_OUTPUT_VT, MSDetectorFileOutputVt};

/// Data structure for mean (aggregated) edge/lane values
/// C++ class `MSMeanData_Harmonoise::MSLaneMeanDataValues` (microsim/output/MSMeanData_Harmonoise.h)
#[repr(C)] // base must be field 0 for the C++ downcasts
pub struct MSMeanData_Harmonoise_MSLaneMeanDataValues {
    /// C++ base class `MSMeanData::MeanDataValues`
    pub base: MSMeanData_MeanDataValues,
    /// Sum of produced noise at this time step(pow(10, (<NOISE>/10.)))
    pub currentTimeN: f64,
    /// Sum of produced noise over time (pow(10, (<NOISE>/10.)))
    pub meanNTemp: f64,
    /// The meandata parent
    pub myParent: Ptr<MSMeanData_Harmonoise>,
}

impl std::ops::Deref for MSMeanData_Harmonoise_MSLaneMeanDataValues {
    type Target = MSMeanData_MeanDataValues;
    fn deref(&self) -> &MSMeanData_MeanDataValues {
        &self.base
    }
}
impl std::ops::DerefMut for MSMeanData_Harmonoise_MSLaneMeanDataValues {
    fn deref_mut(&mut self) -> &mut MSMeanData_MeanDataValues {
        &mut self.base
    }
}

impl MSMeanData_Harmonoise_MSLaneMeanDataValues {
    /// C++: `void MSLaneMeanDataValues::MSLaneMeanDataValues(MSLane *const, const double, const bool, const MSMeanData_Harmonoise *)`
    /// Constructor
    // MSMeanData_Harmonoise::MSLaneMeanDataValues::MSLaneMeanDataValues(MSLane* const lane, const double length, const bool doAdd,
    //         const MSMeanData_Harmonoise* parent)
    //     : MSMeanData::MeanDataValues(lane, length, doAdd, parent),
    //       currentTimeN(0), meanNTemp(0), myParent(parent) {}
    #[allow(unused_variables)]
    pub fn new(lane: Ptr<MSLane>, length: f64, doAdd: bool, parent: Ptr<MSMeanData_Harmonoise>) -> Self {
        let mut base = MSMeanData_MeanDataValues::new(lane, length, doAdd, Ptr::from_mut(&mut parent.m().base));
        base.data_vt = &MS_MEAN_DATA_VALUES_HARMONOISE_VT;
        MSMeanData_Harmonoise_MSLaneMeanDataValues {
            base,
            currentTimeN: 0.0,
            meanNTemp: 0.0,
            myParent: parent,
        }
    }

    /// C++: `void MSLaneMeanDataValues::~MSLaneMeanDataValues() noexcept` [virtual]
    /// Destructor
    // MSMeanData_Harmonoise::MSLaneMeanDataValues::~MSLaneMeanDataValues() {
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
    }

    /// C++: `void MSLaneMeanDataValues::reset(bool)` [virtual]
    /// Resets values so they may be used for the next interval
    // void
    // MSMeanData_Harmonoise::MSLaneMeanDataValues::reset(bool) {
    //     sampleSeconds = 0;
    //     currentTimeN = 0;
    //     meanNTemp = 0;
    //     travelledDistance = 0;
    //     resetTime = SIMSTEP;
    // }
    #[allow(unused_variables)]
    pub fn reset(&mut self, afterWrite: bool) {
        self.sampleSeconds = 0.0;
        self.currentTimeN = 0.0;
        self.meanNTemp = 0.0;
        self.travelledDistance = 0.0;
        self.resetTime = MSNet::get_instance().get_current_time_step();
    }

    /// C++: `void MSLaneMeanDataValues::addTo(MSMeanData::MeanDataValues &) const` [const, virtual]
    /// Add the values to this meanData
    // void
    // MSMeanData_Harmonoise::MSLaneMeanDataValues::addTo(MSMeanData::MeanDataValues& val) const {
    //     MSLaneMeanDataValues& v = (MSLaneMeanDataValues&) val;
    //     v.sampleSeconds += sampleSeconds;
    //     v.meanNTemp += meanNTemp;
    //     v.travelledDistance += travelledDistance;
    // }
    #[allow(unused_variables)]
    pub fn add_to(&self, val: &mut MSMeanData_MeanDataValues) {
        // C++ downcast; `base` is the first field (repr(C))
        let v = unsafe { &mut *(val as *mut MSMeanData_MeanDataValues as *mut MSMeanData_Harmonoise_MSLaneMeanDataValues) };
        v.sampleSeconds += self.sampleSeconds;
        v.meanNTemp += self.meanNTemp;
        v.travelledDistance += self.travelledDistance;
    }

    /// C++: `void MSLaneMeanDataValues::update()` [virtual]
    /// Computes the noise in the last time step
    // void
    // MSMeanData_Harmonoise::MSLaneMeanDataValues::update() {
    //     meanNTemp += currentTimeN;
    //     currentTimeN = 0;
    // }
    #[allow(unused_variables)]
    pub fn update(&mut self) {
        self.meanNTemp += self.currentTimeN;
        self.currentTimeN = 0.0;
    }

    /// C++: `void MSLaneMeanDataValues::write(OutputDevice &, const SumoXMLAttrMask &, const SUMOTime, const int, const double, const double, const int) const` [const, virtual]
    /// Writes output values into the given stream
    // void
    // MSMeanData_Harmonoise::MSLaneMeanDataValues::write(OutputDevice& dev, const SumoXMLAttrMask& attributeMask, const SUMOTime period,
    //         const int /*numLanes*/, const double /*speedLimit*/, const double defaultTravelTime, const int /*numVehicles*/) const {
    //     const double noise = meanNTemp != 0 ? (double)(10. * log10(meanNTemp * TS / STEPS2TIME(period))) : (double) 0.;
    //     dev.writeOptionalAttr(SUMO_ATTR_NOISE, noise, attributeMask);
    //     if (sampleSeconds > myParent->myMinSamples) {
    //         double traveltime = myParent->myMaxTravelTime;
    //         if (travelledDistance > 0.f) {
    //             traveltime = MIN2(traveltime, myLaneLength * sampleSeconds / travelledDistance);
    //         }
    //         dev.writeOptionalAttr(SUMO_ATTR_TRAVELTIME, traveltime, attributeMask);
    //     } else if (defaultTravelTime >= 0.) {
    //         // @todo default value for noise
    //         dev.writeOptionalAttr(SUMO_ATTR_TRAVELTIME, defaultTravelTime, attributeMask);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn write(&self, dev: &mut OutputDevice, attributeMask: &SumoXMLAttrMask, period: SUMOTime, numLanes: i32, speedLimit: f64, defaultTravelTime: f64, numVehicles: i32) {
        let noise = if self.meanNTemp != 0.0 { 10.0 * (self.meanNTemp * ts() / steps2time(period)).log10() } else { 0.0 };
        dev.write_optional_attr_mask(SUMO_ATTR_NOISE, &noise, *attributeMask, false);
        if self.sampleSeconds > self.myParent.myMinSamples {
            let mut traveltime = self.myParent.myMaxTravelTime;
            if self.travelledDistance > 0.0 {
                // MIN2(a, b) as std::min: (b < a) ? b : a
                let q = self.myLaneLength * self.sampleSeconds / self.travelledDistance;
                traveltime = if q < traveltime { q } else { traveltime };
            }
            dev.write_optional_attr_mask(SUMO_ATTR_TRAVELTIME, &traveltime, *attributeMask, false);
        } else if defaultTravelTime >= 0.0 {
            // @todo default value for noise (as in C++)
            dev.write_optional_attr_mask(SUMO_ATTR_TRAVELTIME, &defaultTravelTime, *attributeMask, false);
        }
    }

    /// C++: `void MSLaneMeanDataValues::notifyMoveInternal(const SUMOTrafficObject &, const double, const double, const double, const double, const double, const double, const double)` [virtual, protected]
    /// Internal notification about the vehicle moves
    // void
    // MSMeanData_Harmonoise::MSLaneMeanDataValues::notifyMoveInternal(const SUMOTrafficObject& veh, const double /* frontOnLane */, const double timeOnLane, const double /*meanSpeedFrontOnLane*/, const double meanSpeedVehicleOnLane, const double /*travelledDistanceFrontOnLane*/, const double travelledDistanceVehicleOnLane, const double /* meanLengthOnLane */) {
    //     const double sn = HelpersHarmonoise::computeNoise(veh.getVehicleType().getEmissionClass(),
    //                       // XXX: recheck, which value to use here for the speed. (Leo) Refs. #2579
    //                       (double) meanSpeedVehicleOnLane, veh.getAcceleration());
    //     currentTimeN += (double) pow(10., (sn / 10.));
    //     sampleSeconds += timeOnLane;
    //     travelledDistance += travelledDistanceVehicleOnLane;
    // }
    #[allow(unused_variables)]
    pub fn notify_move_internal(&mut self, veh: &SUMOTrafficObject, arg1: f64, timeOnLane: f64, meanSpeedFrontOnLane: f64, meanSpeedVehicleOnLane: f64, travelledDistanceFrontOnLane: f64, travelledDistanceVehicleOnLane: f64, arg7: f64) {
        let sn = HelpersHarmonoise::compute_noise(veh.get_vehicle_type().get_emission_class(), meanSpeedVehicleOnLane, veh.get_acceleration());
        self.currentTimeN += 10.0_f64.powf(sn / 10.0);
        self.sampleSeconds += timeOnLane;
        self.travelledDistance += travelledDistanceVehicleOnLane;
    }

}

/// Noise data collector for edges/lanes
/// Virtual dispatch table of MSMeanData_Harmonoise::MSLaneMeanDataValues (overrides reset, addTo, update, write, notifyMoveInternal)
static MS_MEAN_DATA_VALUES_HARMONOISE_VT: MeanDataValuesVt = MeanDataValuesVt {
    reset: harmonoise_values_reset,
    add_to: harmonoise_values_add_to,
    notify_enter: MSMeanData_MeanDataValues::notify_enter_base,
    notify_leave: MSMeanData_MeanDataValues::notify_leave_base,
    notify_move_internal: harmonoise_values_notify_move_internal,
    is_empty: MSMeanData_MeanDataValues::is_empty_base,
    update: harmonoise_values_update,
    write: harmonoise_values_write,
    get_samples: MSMeanData_MeanDataValues::get_samples_base,
    get_attribute_value: MSMeanData_MeanDataValues::get_attribute_value_base,
};

fn harmonoise_values_reset(v: &mut MSMeanData_MeanDataValues, afterWrite: bool) {
    Ptr::from_mut(v).cast::<MSMeanData_Harmonoise_MSLaneMeanDataValues>().m().reset(afterWrite)
}

fn harmonoise_values_add_to(v: &MSMeanData_MeanDataValues, val: &mut MSMeanData_MeanDataValues) {
    Ptr::new(v).cast::<MSMeanData_Harmonoise_MSLaneMeanDataValues>().m().add_to(val)
}

fn harmonoise_values_update(v: &mut MSMeanData_MeanDataValues) {
    Ptr::from_mut(v).cast::<MSMeanData_Harmonoise_MSLaneMeanDataValues>().m().update()
}

fn harmonoise_values_notify_move_internal(v: &mut MSMeanData_MeanDataValues, veh: &SUMOTrafficObject, frontOnLane: f64, timeOnLane: f64, meanSpeedFrontOnLane: f64, meanSpeedVehicleOnLane: f64, travelledDistanceFrontOnLane: f64, travelledDistanceVehicleOnLane: f64, meanLengthOnLane: f64) {
    Ptr::from_mut(v).cast::<MSMeanData_Harmonoise_MSLaneMeanDataValues>().m().notify_move_internal(veh, frontOnLane, timeOnLane, meanSpeedFrontOnLane, meanSpeedVehicleOnLane, travelledDistanceFrontOnLane, travelledDistanceVehicleOnLane, meanLengthOnLane)
}

fn harmonoise_values_write(v: &MSMeanData_MeanDataValues, dev: &mut OutputDevice, attributeMask: &SumoXMLAttrMask, period: SUMOTime, numLanes: i32, speedLimit: f64, defaultTravelTime: f64, numVehicles: i32) {
    Ptr::new(v).cast::<MSMeanData_Harmonoise_MSLaneMeanDataValues>().m().write(dev, attributeMask, period, numLanes, speedLimit, defaultTravelTime, numVehicles)
}

/// Virtual dispatch table of MSMeanData_Harmonoise (overrides getAttributeNames and createValues)
static MS_MEAN_DATA_HARMONOISE_VT: MSMeanDataVt = MSMeanDataVt {
    create_values: harmonoise_mean_data_create_values,
    get_attribute_names: harmonoise_mean_data_get_attribute_names,
    write_xml_detector_prolog: MSMeanData::write_xml_detector_prolog_base,
    get_edge_id: MSMeanData::get_edge_id_base,
    open_interval: MSMeanData::open_interval_base,
    write_prefix: MSMeanData::write_prefix_base,
    reset_only: MSMeanData::reset_only_base,
    detector_update: harmonoise_mean_data_detector_update,
};

fn harmonoise_detector_write_xml_output(d: &mut MSDetectorFileOutput, dev: &mut OutputDevice, startTime: SUMOTime, stopTime: SUMOTime) {
    Ptr::from_mut(d).cast::<MSMeanData>().m().write_xml_output(dev, startTime, stopTime)
}

fn harmonoise_detector_write_xml_detector_prolog(d: &MSDetectorFileOutput, dev: &mut OutputDevice) {
    Ptr::new(d).cast::<MSMeanData>().m().write_xml_detector_prolog(dev)
}

fn harmonoise_detector_detector_update(d: &mut MSDetectorFileOutput, step: SUMOTime) {
    // C++ MSMeanData_Harmonoise::detectorUpdate override (calls update() on every lane value)
    Ptr::from_mut(d).cast::<MSMeanData_Harmonoise>().m().detector_update(step)
}

/// Detector-level dispatch (MSDetectorFileOutput virtuals reached through the base pointer), as in MSMeanData_Emissions
static MS_MEAN_DATA_HARMONOISE_DETECTOR_VT: MSDetectorFileOutputVt = MSDetectorFileOutputVt {
    write_xml_output: harmonoise_detector_write_xml_output,
    write_xml_detector_prolog: harmonoise_detector_write_xml_detector_prolog,
    detector_update: harmonoise_detector_detector_update,
    ..DEFAULT_MSDETECTOR_FILE_OUTPUT_VT
};

fn harmonoise_mean_data_create_values(m: &MSMeanData, lane: Ptr<MSLane>, length: f64, doAdd: bool) -> Ptr<MSMeanData_MeanDataValues> {
    Ptr::new(m).cast::<MSMeanData_Harmonoise>().m().create_values(lane, length, doAdd)
}

fn harmonoise_mean_data_get_attribute_names(m: &MSMeanData) -> Vec<String> {
    Ptr::new(m).cast::<MSMeanData_Harmonoise>().m().get_attribute_names()
}

fn harmonoise_mean_data_detector_update(m: &mut MSMeanData, step: SUMOTime) {
    Ptr::from_mut(m).cast::<MSMeanData_Harmonoise>().m().detector_update(step)
}

/// C++ class `MSMeanData_Harmonoise` (microsim/output/MSMeanData_Harmonoise.h)
#[repr(C)]
pub struct MSMeanData_Harmonoise {
    /// C++ base class `MSMeanData`
    pub base: MSMeanData,
}

impl std::ops::Deref for MSMeanData_Harmonoise {
    type Target = MSMeanData;
    fn deref(&self) -> &MSMeanData {
        &self.base
    }
}
impl std::ops::DerefMut for MSMeanData_Harmonoise {
    fn deref_mut(&mut self) -> &mut MSMeanData {
        &mut self.base
    }
}

impl MSMeanData_Harmonoise {
    /// C++: `void MSMeanData_Harmonoise::MSMeanData_Harmonoise(const std::string &, const SUMOTime, const SUMOTime, const bool, const std::string &, const bool, const bool, const double, const double, const std::string &, const std::string &, const std::vector<MSEdge *> &, AggregateType)`
    /// Constructor
    // MSMeanData_Harmonoise::MSMeanData_Harmonoise(const std::string& id,
    //         const SUMOTime dumpBegin, const SUMOTime dumpEnd,
    //         const bool useLanes, const std::string& excludeEmpty, const bool withInternal,
    //         const bool trackVehicles,
    //         const double maxTravelTime, const double minSamples,
    //         const std::string& vTypes,
    //         const std::string& writeAttributes,
    //         const std::vector<MSEdge*>& edges,
    //         AggregateType aggregate) :
    //     MSMeanData(id, dumpBegin, dumpEnd, useLanes, excludeEmpty,
    //                withInternal, trackVehicles, 0, maxTravelTime, minSamples, vTypes, writeAttributes, edges, aggregate) {
    // }
    #[allow(unused_variables)]
    pub fn new(id: &str, dumpBegin: SUMOTime, dumpEnd: SUMOTime, useLanes: bool, excludeEmpty: &str, withInternal: bool, trackVehicles: bool, maxTravelTime: f64, minSamples: f64, vTypes: &str, writeAttributes: &str, edges: &Vec<Ptr<MSEdge>>, aggregate: AggregateType) -> Self {
        let mut base = MSMeanData::new(id, dumpBegin, dumpEnd, useLanes, excludeEmpty, withInternal, trackVehicles, 0, maxTravelTime, minSamples, vTypes, writeAttributes, edges, aggregate);
        base.meandata_vt = &MS_MEAN_DATA_HARMONOISE_VT;
        base.base.vt = &MS_MEAN_DATA_HARMONOISE_DETECTOR_VT;
        Self { base }
    }

    /// C++: `void MSMeanData_Harmonoise::~MSMeanData_Harmonoise() noexcept` [virtual]
    /// Destructor
    // MSMeanData_Harmonoise::~MSMeanData_Harmonoise() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
    }

    /// C++: `void MSMeanData_Harmonoise::detectorUpdate(const SUMOTime)` [virtual]
    /// Updates the detector
    // void
    // MSMeanData_Harmonoise::detectorUpdate(const SUMOTime step) {
    //     MSMeanData::detectorUpdate(step);
    //     for (std::vector<std::vector<MeanDataValues*> >::const_iterator i = myMeasures.begin(); i != myMeasures.end(); ++i) {
    //         const std::vector<MeanDataValues*>& lm = *i;
    //         for (std::vector<MeanDataValues*>::const_iterator j = lm.begin(); j != lm.end(); ++j) {
    //             (*j)->update();
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn detector_update(&mut self, step: SUMOTime) {
        self.base.detector_update_base(step);
        for lm in &self.myMeasures {
            for j in lm {
                // virtual (*j)->update(): lane values run the harmonoise override, trackers the base no-op
                j.m().update();
            }
        }
    }

    /// C++: `std::vector<std::string> MSMeanData_Harmonoise::getAttributeNames() const` [const, virtual]
    /// return all attributes that are (potentially) written by this output
    // std::vector<std::string>
    // MSMeanData_Harmonoise::getAttributeNames() const {
    //     return std::vector<std::string> {toString(SUMO_ATTR_NOISE)};
    // }
    #[allow(unused_variables)]
    pub fn get_attribute_names(&self) -> Vec<String> {
        vec![utils::xml::sumo_xml_definitions::Attrs.get_string(SUMO_ATTR_NOISE).to_string()]
    }

    /// C++: `bool MSMeanData_Harmonoise::supports(const SumoXMLAttrMask &)` [static]
    /// check whether any of the given attributes can be calculated by this meandata
    // bool
    // MSMeanData_Harmonoise::supports(const SumoXMLAttrMask& attributeMask) {
    //     return attributeMask.test(SUMO_ATTR_NOISE);
    // }
    #[allow(unused_variables)]
    pub fn supports(attributeMask: &SumoXMLAttrMask) -> bool {
        attributeMask.test(SUMO_ATTR_NOISE)
    }

    /// C++: `MSMeanData::MeanDataValues * MSMeanData_Harmonoise::createValues(MSLane *const, const double, const bool) const` [const, virtual, protected]
    /// Create an instance of MeanDataValues
    // MSMeanData::MeanDataValues*
    // MSMeanData_Harmonoise::createValues(MSLane* const lane, const double length, const bool doAdd) const {
    //     return new MSLaneMeanDataValues(lane, length, doAdd, this);
    // }
    #[allow(unused_variables)]
    pub fn create_values(&self, lane: Ptr<MSLane>, length: f64, doAdd: bool) -> Ptr<MSMeanData_MeanDataValues> {
        // shortcut: the values are leaked (C++ deletes them in ~MSMeanData), upgrade when myMeasures owns its values
        let values = Box::leak(Box::new(MSMeanData_Harmonoise_MSLaneMeanDataValues::new(lane, length, doAdd, Ptr::new(self))));
        let values = Ptr::from_mut(&mut values.base);
        register_values(values, lane, doAdd);
        values
    }

    /// C++: `void MSMeanData_Harmonoise::MSMeanData_Harmonoise(const MSMeanData_Harmonoise &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSMeanData_Harmonoise) -> Self {
        // C++ copy constructor is private and invalidated (never called)
        panic!("MSMeanData_Harmonoise copy constructor is invalidated");
    }

    /// C++: `MSMeanData_Harmonoise & MSMeanData_Harmonoise::operator=(const MSMeanData_Harmonoise &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSMeanData_Harmonoise) -> &mut MSMeanData_Harmonoise {
        // C++ assignment operator is private and invalidated (never called)
        panic!("MSMeanData_Harmonoise assignment operator is invalidated");
    }

}

