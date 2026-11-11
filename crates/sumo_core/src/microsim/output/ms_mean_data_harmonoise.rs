//! Port of microsim/output/MSMeanData_Harmonoise.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// Data structure for mean (aggregated) edge/lane values
/// C++ class `MSMeanData_Harmonoise::MSLaneMeanDataValues` (microsim/output/MSMeanData_Harmonoise.h)
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
        todo!("port MSLaneMeanDataValues::MSLaneMeanDataValues")
    }

    /// C++: `void MSLaneMeanDataValues::~MSLaneMeanDataValues() noexcept` [virtual]
    /// Destructor
    // MSMeanData_Harmonoise::MSLaneMeanDataValues::~MSLaneMeanDataValues() {
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MSLaneMeanDataValues::~MSLaneMeanDataValues")
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
        todo!("port MSLaneMeanDataValues::reset")
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
        todo!("port MSLaneMeanDataValues::addTo")
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
        todo!("port MSLaneMeanDataValues::update")
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
        todo!("port MSLaneMeanDataValues::write")
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
        todo!("port MSLaneMeanDataValues::notifyMoveInternal")
    }

}

/// Noise data collector for edges/lanes
/// C++ class `MSMeanData_Harmonoise` (microsim/output/MSMeanData_Harmonoise.h)
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
    pub fn new(id: &str, dumpBegin: SUMOTime, dumpEnd: SUMOTime, useLanes: bool, excludeEmpty: &str, withInternal: bool, trackVehicles: bool, minSamples: f64, maxTravelTime: f64, vTypes: &str, writeAttributes: &str, edges: &Vec<Ptr<MSEdge>>, aggregate: AggregateType) -> Self {
        todo!("port MSMeanData_Harmonoise::MSMeanData_Harmonoise")
    }

    /// C++: `void MSMeanData_Harmonoise::~MSMeanData_Harmonoise() noexcept` [virtual]
    /// Destructor
    // MSMeanData_Harmonoise::~MSMeanData_Harmonoise() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MSMeanData_Harmonoise::~MSMeanData_Harmonoise")
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
        todo!("port MSMeanData_Harmonoise::detectorUpdate")
    }

    /// C++: `std::vector<std::string> MSMeanData_Harmonoise::getAttributeNames() const` [const, virtual]
    /// return all attributes that are (potentially) written by this output
    // std::vector<std::string>
    // MSMeanData_Harmonoise::getAttributeNames() const {
    //     return std::vector<std::string> {toString(SUMO_ATTR_NOISE)};
    // }
    #[allow(unused_variables)]
    pub fn get_attribute_names(&self) -> Vec<String> {
        todo!("port MSMeanData_Harmonoise::getAttributeNames")
    }

    /// C++: `bool MSMeanData_Harmonoise::supports(const SumoXMLAttrMask &)` [static]
    /// check whether any of the given attributes can be calculated by this meandata
    // bool
    // MSMeanData_Harmonoise::supports(const SumoXMLAttrMask& attributeMask) {
    //     return attributeMask.test(SUMO_ATTR_NOISE);
    // }
    #[allow(unused_variables)]
    pub fn supports(attributeMask: &SumoXMLAttrMask) -> bool {
        todo!("port MSMeanData_Harmonoise::supports")
    }

    /// C++: `MSMeanData::MeanDataValues * MSMeanData_Harmonoise::createValues(MSLane *const, const double, const bool) const` [const, virtual, protected]
    /// Create an instance of MeanDataValues
    // MSMeanData::MeanDataValues*
    // MSMeanData_Harmonoise::createValues(MSLane* const lane, const double length, const bool doAdd) const {
    //     return new MSLaneMeanDataValues(lane, length, doAdd, this);
    // }
    #[allow(unused_variables)]
    pub fn create_values(&self, lane: Ptr<MSLane>, length: f64, doAdd: bool) -> Ptr<MSMeanData_MeanDataValues> {
        todo!("port MSMeanData_Harmonoise::createValues")
    }

    /// C++: `void MSMeanData_Harmonoise::MSMeanData_Harmonoise(const MSMeanData_Harmonoise &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSMeanData_Harmonoise) -> Self {
        todo!("port MSMeanData_Harmonoise::MSMeanData_Harmonoise")
    }

    /// C++: `MSMeanData_Harmonoise & MSMeanData_Harmonoise::operator=(const MSMeanData_Harmonoise &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSMeanData_Harmonoise) -> &mut MSMeanData_Harmonoise {
        todo!("port MSMeanData_Harmonoise::operator=")
    }

}

