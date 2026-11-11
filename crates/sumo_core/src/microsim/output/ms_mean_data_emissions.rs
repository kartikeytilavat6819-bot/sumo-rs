//! Port of microsim/output/MSMeanData_Emissions.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// Data structure for mean (aggregated) edge/lane values
/// C++ class `MSMeanData_Emissions::MSLaneMeanDataValues` (microsim/output/MSMeanData_Emissions.h)
pub struct MSMeanData_Emissions_MSLaneMeanDataValues {
    /// C++ base class `MSMeanData::MeanDataValues`
    pub base: MSMeanData_MeanDataValues,
    /// Collected values
    pub myEmissions: PollutantsInterface_Emissions,
}

impl std::ops::Deref for MSMeanData_Emissions_MSLaneMeanDataValues {
    type Target = MSMeanData_MeanDataValues;
    fn deref(&self) -> &MSMeanData_MeanDataValues {
        &self.base
    }
}
impl std::ops::DerefMut for MSMeanData_Emissions_MSLaneMeanDataValues {
    fn deref_mut(&mut self) -> &mut MSMeanData_MeanDataValues {
        &mut self.base
    }
}

impl MSMeanData_Emissions_MSLaneMeanDataValues {
    /// C++: `void MSLaneMeanDataValues::MSLaneMeanDataValues(MSLane *const, const double, const bool, const MSMeanData_Emissions *)`
    /// Constructor
    // MSMeanData_Emissions::MSLaneMeanDataValues::MSLaneMeanDataValues(MSLane* const lane,
    //         const double length, const bool doAdd,
    //         const MSMeanData_Emissions* parent)
    //     : MSMeanData::MeanDataValues(lane, length, doAdd, parent),
    //       myEmissions() {}
    #[allow(unused_variables)]
    pub fn new(lane: Ptr<MSLane>, length: f64, doAdd: bool, parent: Ptr<MSMeanData_Emissions>) -> Self {
        todo!("port MSLaneMeanDataValues::MSLaneMeanDataValues")
    }

    /// C++: `void MSLaneMeanDataValues::~MSLaneMeanDataValues() noexcept` [virtual]
    /// Destructor
    // MSMeanData_Emissions::MSLaneMeanDataValues::~MSLaneMeanDataValues() {
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MSLaneMeanDataValues::~MSLaneMeanDataValues")
    }

    /// C++: `void MSLaneMeanDataValues::reset(bool)` [virtual]
    /// Resets values so they may be used for the next interval
    // void
    // MSMeanData_Emissions::MSLaneMeanDataValues::reset(bool) {
    //     sampleSeconds = 0.;
    //     travelledDistance = 0.;
    //     myEmissions = PollutantsInterface::Emissions();
    //     resetTime = SIMSTEP;
    // }
    #[allow(unused_variables)]
    pub fn reset(&mut self, afterWrite: bool) {
        todo!("port MSLaneMeanDataValues::reset")
    }

    /// C++: `void MSLaneMeanDataValues::addTo(MSMeanData::MeanDataValues &) const` [const, virtual]
    /// Add the values of this to the given one and store them there
    // void
    // MSMeanData_Emissions::MSLaneMeanDataValues::addTo(MSMeanData::MeanDataValues& val) const {
    //     MSLaneMeanDataValues& v = (MSLaneMeanDataValues&) val;
    //     v.sampleSeconds += sampleSeconds;
    //     v.travelledDistance += travelledDistance;
    //     v.myEmissions.addScaled(myEmissions);
    // }
    #[allow(unused_variables)]
    pub fn add_to(&self, val: &mut MSMeanData_MeanDataValues) {
        todo!("port MSLaneMeanDataValues::addTo")
    }

    /// C++: `void MSLaneMeanDataValues::write(OutputDevice &, const SumoXMLAttrMask &, const SUMOTime, const int, const double, const double, const int) const` [const, virtual]
    /// Writes output values into the given stream
    // void
    // MSMeanData_Emissions::MSLaneMeanDataValues::write(OutputDevice& dev, const SumoXMLAttrMask& attributeMask, const SUMOTime period,
    //         const int /*numLanes*/, const double /*speedLimit*/, const double defaultTravelTime, const int /*numVehicles*/) const {
    //     const double normFactor = double(3600. / STEPS2TIME(period) / myLaneLength);
    //     dev.setPrecision(gPrecisionEmissions);
    //     dev.writeOptionalAttr(SUMO_ATTR_CO_ABS,          myEmissions.CO, attributeMask);
    //     dev.writeOptionalAttr(SUMO_ATTR_CO2_ABS,         myEmissions.CO2, attributeMask);
    //     dev.writeOptionalAttr(SUMO_ATTR_HC_ABS,          myEmissions.HC, attributeMask);
    //     dev.writeOptionalAttr(SUMO_ATTR_PMX_ABS,         myEmissions.PMx, attributeMask);
    //     dev.writeOptionalAttr(SUMO_ATTR_NOX_ABS,         myEmissions.NOx, attributeMask);
    //     dev.writeOptionalAttr(SUMO_ATTR_FUEL_ABS,        myEmissions.fuel, attributeMask);
    //     dev.writeOptionalAttr(SUMO_ATTR_ELECTRICITY_ABS, myEmissions.electricity, attributeMask);
    //     if (attributeMask == 0) {
    //         dev.writePadding("\n           ");
    //     }
    //     dev.writeOptionalAttr(SUMO_ATTR_CO_NORMED,          normFactor * myEmissions.CO, attributeMask);
    //     dev.writeOptionalAttr(SUMO_ATTR_CO2_NORMED,         normFactor * myEmissions.CO2, attributeMask);
    //     dev.writeOptionalAttr(SUMO_ATTR_HC_NORMED,          normFactor * myEmissions.HC, attributeMask);
    //     dev.writeOptionalAttr(SUMO_ATTR_PMX_NORMED,         normFactor * myEmissions.PMx, attributeMask);
    //     dev.writeOptionalAttr(SUMO_ATTR_NOX_NORMED,         normFactor * myEmissions.NOx, attributeMask);
    //     dev.writeOptionalAttr(SUMO_ATTR_FUEL_NORMED,        normFactor * myEmissions.fuel, attributeMask);
    //     dev.writeOptionalAttr(SUMO_ATTR_ELECTRICITY_NORMED, normFactor * myEmissions.electricity, attributeMask);
    //
    //     if (sampleSeconds > myParent->getMinSamples()) {
    //         double vehFactor = myParent->getMaxTravelTime() / sampleSeconds;
    //         double traveltime = myParent->getMaxTravelTime();
    //         if (travelledDistance > 0.f) {
    //             vehFactor = MIN2(vehFactor, myLaneLength / travelledDistance);
    //             traveltime = MIN2(traveltime, myLaneLength * sampleSeconds / travelledDistance);
    //         }
    //         if (attributeMask == 0) {
    //             dev.writePadding("\n           ");
    //         }
    //         dev.setPrecision(gPrecision);
    //         dev.writeOptionalAttr(SUMO_ATTR_TRAVELTIME,         traveltime, attributeMask);
    //         dev.setPrecision(gPrecisionEmissions);
    //         dev.writeOptionalAttr(SUMO_ATTR_CO_PERVEH,          vehFactor * myEmissions.CO, attributeMask);
    //         dev.writeOptionalAttr(SUMO_ATTR_CO2_PERVEH,         vehFactor * myEmissions.CO2, attributeMask);
    //         dev.writeOptionalAttr(SUMO_ATTR_HC_PERVEH,          vehFactor * myEmissions.HC, attributeMask);
    //         dev.writeOptionalAttr(SUMO_ATTR_PMX_PERVEH,         vehFactor * myEmissions.PMx, attributeMask);
    //         dev.writeOptionalAttr(SUMO_ATTR_NOX_PERVEH,         vehFactor * myEmissions.NOx, attributeMask);
    //         dev.writeOptionalAttr(SUMO_ATTR_FUEL_PERVEH,        vehFactor * myEmissions.fuel, attributeMask);
    //         dev.writeOptionalAttr(SUMO_ATTR_ELECTRICITY_PERVEH, vehFactor * myEmissions.electricity, attributeMask);
    //     } else if (defaultTravelTime >= 0.) {
    //         const MSVehicleType* t = MSNet::getInstance()->getVehicleControl().getVType();
    //         const double speed = MIN2(myLaneLength / defaultTravelTime, t->getMaxSpeed());
    //
    //         if (attributeMask == 0) {
    //             dev.writePadding("\n           ");
    //         }
    //         dev.setPrecision(gPrecision);
    //         dev.writeOptionalAttr(SUMO_ATTR_TRAVELTIME,         defaultTravelTime, attributeMask);
    //         dev.setPrecision(gPrecisionEmissions);
    //         dev.writeOptionalAttr(SUMO_ATTR_CO_PERVEH,          PollutantsInterface::computeDefault(t->getEmissionClass(), PollutantsInterface::CO,   speed, t->getCarFollowModel().getMaxAccel(), 0, defaultTravelTime, t->getEmissionParameters()), attributeMask);
    //         dev.writeOptionalAttr(SUMO_ATTR_CO2_PERVEH,         PollutantsInterface::computeDefault(t->getEmissionClass(), PollutantsInterface::CO2,  speed, t->getCarFollowModel().getMaxAccel(), 0, defaultTravelTime, t->getEmissionParameters()), attributeMask);
    //         dev.writeOptionalAttr(SUMO_ATTR_HC_PERVEH,          PollutantsInterface::computeDefault(t->getEmissionClass(), PollutantsInterface::HC,   speed, t->getCarFollowModel().getMaxAccel(), 0, defaultTravelTime, t->getEmissionParameters()), attributeMask);
    //         dev.writeOptionalAttr(SUMO_ATTR_PMX_PERVEH,         PollutantsInterface::computeDefault(t->getEmissionClass(), PollutantsInterface::PM_X, speed, t->getCarFollowModel().getMaxAccel(), 0, defaultTravelTime, t->getEmissionParameters()), attributeMask);
    //         dev.writeOptionalAttr(SUMO_ATTR_NOX_PERVEH,         PollutantsInterface::computeDefault(t->getEmissionClass(), PollutantsInterface::NO_X, speed, t->getCarFollowModel().getMaxAccel(), 0, defaultTravelTime, t->getEmissionParameters()), attributeMask);
    //         dev.writeOptionalAttr(SUMO_ATTR_FUEL_PERVEH,        PollutantsInterface::computeDefault(t->getEmissionClass(), PollutantsInterface::FUEL, speed, t->getCarFollowModel().getMaxAccel(), 0, defaultTravelTime, t->getEmissionParameters()), attributeMask);
    //         dev.writeOptionalAttr(SUMO_ATTR_ELECTRICITY_PERVEH, PollutantsInterface::computeDefault(t->getEmissionClass(), PollutantsInterface::ELEC, speed, t->getCarFollowModel().getMaxAccel(), 0, defaultTravelTime, t->getEmissionParameters()), attributeMask);
    //     }
    //     dev.setPrecision(gPrecision);
    // }
    #[allow(unused_variables)]
    pub fn write(&self, dev: &mut OutputDevice, attributeMask: &SumoXMLAttrMask, period: SUMOTime, numLanes: i32, speedLimit: f64, defaultTravelTime: f64, numVehicles: i32) {
        todo!("port MSLaneMeanDataValues::write")
    }

    /// C++: `void MSLaneMeanDataValues::notifyMoveInternal(const SUMOTrafficObject &, const double, const double, const double, const double, const double, const double, const double)` [virtual, protected]
    /// Internal notification about the vehicle moves
    // void
    // MSMeanData_Emissions::MSLaneMeanDataValues::notifyMoveInternal(const SUMOTrafficObject& veh, const double frontOnLane, const double /*timeOnLane*/, const double meanSpeedFrontOnLane, const double /*meanSpeedVehicleOnLane*/, const double travelledDistanceFrontOnLane, const double /*travelledDistanceVehicleOnLane*/, const double /*meanLengthOnLane*/) {
    //     if (myParent != nullptr && !myParent->vehicleApplies(veh)) {
    //         return;
    //     }
    //     if (veh.isVehicle()) {
    //         sampleSeconds += frontOnLane;
    //         travelledDistance += travelledDistanceFrontOnLane;
    //         const double a = veh.getAcceleration();
    //         myEmissions.addScaled(PollutantsInterface::computeAll(veh.getVehicleType().getEmissionClass(),
    //                               // XXX: recheck, which value to use here for the speed. (Leo) Refs. #2579
    //                               meanSpeedFrontOnLane, a, veh.getSlope(),
    //                               static_cast<const SUMOVehicle&>(veh).getEmissionParameters()), frontOnLane);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn notify_move_internal(&mut self, veh: &SUMOTrafficObject, arg1: f64, timeOnLane: f64, arg3: f64, meanSpeedVehicleOnLane: f64, travelledDistanceFrontOnLane: f64, travelledDistanceVehicleOnLane: f64, arg7: f64) {
        todo!("port MSLaneMeanDataValues::notifyMoveInternal")
    }

    /// C++: `bool MSLaneMeanDataValues::notifyIdle(SUMOTrafficObject &)` [virtual, protected]
    /// Computes idling emission values and adds them to the aggregate emission sums
    // bool
    // MSMeanData_Emissions::MSLaneMeanDataValues::notifyIdle(SUMOTrafficObject& veh) {
    //     if (veh.isVehicle()) {
    //         myEmissions.addScaled(PollutantsInterface::computeAll(veh.getVehicleType().getEmissionClass(),
    //                               0., 0., 0.,
    //                               static_cast<const SUMOVehicle&>(veh).getEmissionParameters()), TS);
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn notify_idle(&mut self, veh: &mut SUMOTrafficObject) -> bool {
        todo!("port MSLaneMeanDataValues::notifyIdle")
    }

}

/// Emission data collector for edges/lanes
/// C++ class `MSMeanData_Emissions` (microsim/output/MSMeanData_Emissions.h)
pub struct MSMeanData_Emissions {
    /// C++ base class `MSMeanData`
    pub base: MSMeanData,
}

impl std::ops::Deref for MSMeanData_Emissions {
    type Target = MSMeanData;
    fn deref(&self) -> &MSMeanData {
        &self.base
    }
}
impl std::ops::DerefMut for MSMeanData_Emissions {
    fn deref_mut(&mut self) -> &mut MSMeanData {
        &mut self.base
    }
}

impl MSMeanData_Emissions {
    /// C++: `void MSMeanData_Emissions::MSMeanData_Emissions(const std::string &, const SUMOTime, const SUMOTime, const bool, const std::string &, const bool, const bool, const double, const double, const std::string &, const std::string &, const std::vector<MSEdge *> &, AggregateType)`
    /// Constructor
    // MSMeanData_Emissions::MSMeanData_Emissions(const std::string& id,
    //         const SUMOTime dumpBegin,
    //         const SUMOTime dumpEnd,
    //         const bool useLanes, const std::string& excludeEmpty,
    //         const bool withInternal,
    //         const bool trackVehicles,
    //         const double maxTravelTime,
    //         const double minSamples,
    //         const std::string& vTypes,
    //         const std::string& writeAttributes,
    //         const std::vector<MSEdge*>& edges,
    //         AggregateType aggregate) :
    //     MSMeanData(id, dumpBegin, dumpEnd, useLanes, excludeEmpty,
    //                withInternal, trackVehicles, 0, maxTravelTime, minSamples, vTypes, writeAttributes, edges, aggregate)
    // { }
    #[allow(unused_variables)]
    pub fn new(id: &str, dumpBegin: SUMOTime, dumpEnd: SUMOTime, useLanes: bool, excludeEmpty: &str, withInternal: bool, trackVehicles: bool, minSamples: f64, maxTravelTime: f64, vTypes: &str, writeAttributes: &str, edges: &Vec<Ptr<MSEdge>>, aggregate: AggregateType) -> Self {
        todo!("port MSMeanData_Emissions::MSMeanData_Emissions")
    }

    /// C++: `void MSMeanData_Emissions::~MSMeanData_Emissions() noexcept` [virtual]
    /// Destructor
    // MSMeanData_Emissions::~MSMeanData_Emissions() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MSMeanData_Emissions::~MSMeanData_Emissions")
    }

    /// C++: `std::vector<std::string> MSMeanData_Emissions::getAttributeNames() const` [const, virtual]
    /// return all attributes that are (potentially) written by this output
    // std::vector<std::string>
    // MSMeanData_Emissions::getAttributeNames() const {
    //     std::vector<std::string> result;
    //     result.push_back(toString(SUMO_ATTR_CO_ABS));
    //     result.push_back(toString(SUMO_ATTR_CO2_ABS));
    //     result.push_back(toString(SUMO_ATTR_HC_ABS));
    //     result.push_back(toString(SUMO_ATTR_PMX_ABS));
    //     result.push_back(toString(SUMO_ATTR_NOX_ABS));
    //     result.push_back(toString(SUMO_ATTR_FUEL_ABS));
    //     result.push_back(toString(SUMO_ATTR_ELECTRICITY_ABS));
    //     result.push_back(toString(SUMO_ATTR_CO_NORMED));
    //     result.push_back(toString(SUMO_ATTR_CO2_NORMED));
    //     result.push_back(toString(SUMO_ATTR_HC_NORMED));
    //     result.push_back(toString(SUMO_ATTR_PMX_NORMED));
    //     result.push_back(toString(SUMO_ATTR_NOX_NORMED));
    //     result.push_back(toString(SUMO_ATTR_FUEL_NORMED));
    //     result.push_back(toString(SUMO_ATTR_ELECTRICITY_NORMED));
    //     result.push_back(toString(SUMO_ATTR_CO_PERVEH));
    //     result.push_back(toString(SUMO_ATTR_CO2_PERVEH));
    //     result.push_back(toString(SUMO_ATTR_HC_PERVEH));
    //     result.push_back(toString(SUMO_ATTR_PMX_PERVEH));
    //     result.push_back(toString(SUMO_ATTR_NOX_PERVEH));
    //     result.push_back(toString(SUMO_ATTR_FUEL_PERVEH));
    //     result.push_back(toString(SUMO_ATTR_ELECTRICITY_PERVEH));
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_attribute_names(&self) -> Vec<String> {
        todo!("port MSMeanData_Emissions::getAttributeNames")
    }

    /// C++: `bool MSMeanData_Emissions::supports(const SumoXMLAttrMask &)` [static]
    /// check whether any of the given attributes can be calculated by this meandata
    // bool
    // MSMeanData_Emissions::supports(const SumoXMLAttrMask& attributeMask) {
    //     return
    //         attributeMask.test(SUMO_ATTR_CO_ABS) ||
    //         attributeMask.test(SUMO_ATTR_CO2_ABS) ||
    //         attributeMask.test(SUMO_ATTR_HC_ABS) ||
    //         attributeMask.test(SUMO_ATTR_PMX_ABS) ||
    //         attributeMask.test(SUMO_ATTR_NOX_ABS) ||
    //         attributeMask.test(SUMO_ATTR_FUEL_ABS) ||
    //         attributeMask.test(SUMO_ATTR_ELECTRICITY_ABS) ||
    //         attributeMask.test(SUMO_ATTR_CO_NORMED) ||
    //         attributeMask.test(SUMO_ATTR_CO2_NORMED) ||
    //         attributeMask.test(SUMO_ATTR_HC_NORMED) ||
    //         attributeMask.test(SUMO_ATTR_PMX_NORMED) ||
    //         attributeMask.test(SUMO_ATTR_NOX_NORMED) ||
    //         attributeMask.test(SUMO_ATTR_FUEL_NORMED) ||
    //         attributeMask.test(SUMO_ATTR_ELECTRICITY_NORMED) ||
    //         attributeMask.test(SUMO_ATTR_CO_PERVEH) ||
    //         attributeMask.test(SUMO_ATTR_CO2_PERVEH) ||
    //         attributeMask.test(SUMO_ATTR_HC_PERVEH) ||
    //         attributeMask.test(SUMO_ATTR_PMX_PERVEH) ||
    //         attributeMask.test(SUMO_ATTR_NOX_PERVEH) ||
    //         attributeMask.test(SUMO_ATTR_FUEL_PERVEH) ||
    //         attributeMask.test(SUMO_ATTR_ELECTRICITY_PERVEH);
    // }
    #[allow(unused_variables)]
    pub fn supports(attributeMask: &SumoXMLAttrMask) -> bool {
        todo!("port MSMeanData_Emissions::supports")
    }

    /// C++: `MSMeanData::MeanDataValues * MSMeanData_Emissions::createValues(MSLane *const, const double, const bool) const` [const, virtual, protected]
    /// Create an instance of MeanDataValues
    // MSMeanData::MeanDataValues*
    // MSMeanData_Emissions::createValues(MSLane* const lane, const double length, const bool doAdd) const {
    //     return new MSLaneMeanDataValues(lane, length, doAdd, this);
    // }
    #[allow(unused_variables)]
    pub fn create_values(&self, lane: Ptr<MSLane>, length: f64, doAdd: bool) -> Ptr<MSMeanData_MeanDataValues> {
        todo!("port MSMeanData_Emissions::createValues")
    }

    /// C++: `void MSMeanData_Emissions::MSMeanData_Emissions(const MSMeanData_Emissions &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSMeanData_Emissions) -> Self {
        todo!("port MSMeanData_Emissions::MSMeanData_Emissions")
    }

    /// C++: `MSMeanData_Emissions & MSMeanData_Emissions::operator=(const MSMeanData_Emissions &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSMeanData_Emissions) -> &mut MSMeanData_Emissions {
        todo!("port MSMeanData_Emissions::operator=")
    }

}

