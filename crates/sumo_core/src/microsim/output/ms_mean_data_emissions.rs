//! Port of microsim/output/MSMeanData_Emissions.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::std_defs::{g_precision, g_precision_emissions};
use super::ms_detector_file_output::{DEFAULT_MSDETECTOR_FILE_OUTPUT_VT, MSDetectorFileOutputVt};

/// Data structure for mean (aggregated) edge/lane values
/// C++ class `MSMeanData_Emissions::MSLaneMeanDataValues` (microsim/output/MSMeanData_Emissions.h)
#[repr(C)] // base must be field 0 for the C++ downcast in addTo
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
        let mut base = MSMeanData_MeanDataValues::new(lane, length, doAdd, Ptr::from_mut(&mut parent.m().base));
        base.data_vt = &MS_MEAN_DATA_VALUES_EMISSIONS_VT;
        MSMeanData_Emissions_MSLaneMeanDataValues {
            base,
            myEmissions: PollutantsInterface_Emissions::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        }
    }

    /// C++: `void MSLaneMeanDataValues::~MSLaneMeanDataValues() noexcept` [virtual]
    /// Destructor
    // MSMeanData_Emissions::MSLaneMeanDataValues::~MSLaneMeanDataValues() {
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
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
        self.sampleSeconds = 0.0;
        self.travelledDistance = 0.0;
        self.myEmissions = PollutantsInterface_Emissions::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        self.resetTime = MSNet::get_instance().get_current_time_step();
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
        // C++ downcast; `base` is the first field (repr(C))
        let v = unsafe { &mut *(val as *mut MSMeanData_MeanDataValues as *mut MSMeanData_Emissions_MSLaneMeanDataValues) };
        v.sampleSeconds += self.sampleSeconds;
        v.travelledDistance += self.travelledDistance;
        v.myEmissions.add_scaled(&self.myEmissions, 1.0);
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
        let normFactor = 3600.0 / steps2time(period) / self.myLaneLength;
        let mask = *attributeMask;
        dev.set_precision(g_precision_emissions());
        dev.write_optional_attr_mask(SUMO_ATTR_CO_ABS, &self.myEmissions.CO, mask, false);
        dev.write_optional_attr_mask(SUMO_ATTR_CO2_ABS, &self.myEmissions.CO2, mask, false);
        dev.write_optional_attr_mask(SUMO_ATTR_HC_ABS, &self.myEmissions.HC, mask, false);
        dev.write_optional_attr_mask(SUMO_ATTR_PMX_ABS, &self.myEmissions.PMx, mask, false);
        dev.write_optional_attr_mask(SUMO_ATTR_NOX_ABS, &self.myEmissions.NOx, mask, false);
        dev.write_optional_attr_mask(SUMO_ATTR_FUEL_ABS, &self.myEmissions.fuel, mask, false);
        dev.write_optional_attr_mask(SUMO_ATTR_ELECTRICITY_ABS, &self.myEmissions.electricity, mask, false);
        if attributeMask.none() {
            dev.write_padding("\n           ");
        }
        dev.write_optional_attr_mask(SUMO_ATTR_CO_NORMED, &(normFactor * self.myEmissions.CO), mask, false);
        dev.write_optional_attr_mask(SUMO_ATTR_CO2_NORMED, &(normFactor * self.myEmissions.CO2), mask, false);
        dev.write_optional_attr_mask(SUMO_ATTR_HC_NORMED, &(normFactor * self.myEmissions.HC), mask, false);
        dev.write_optional_attr_mask(SUMO_ATTR_PMX_NORMED, &(normFactor * self.myEmissions.PMx), mask, false);
        dev.write_optional_attr_mask(SUMO_ATTR_NOX_NORMED, &(normFactor * self.myEmissions.NOx), mask, false);
        dev.write_optional_attr_mask(SUMO_ATTR_FUEL_NORMED, &(normFactor * self.myEmissions.fuel), mask, false);
        dev.write_optional_attr_mask(SUMO_ATTR_ELECTRICITY_NORMED, &(normFactor * self.myEmissions.electricity), mask, false);

        if self.sampleSeconds > self.myParent.get_min_samples() {
            let mut vehFactor = self.myParent.get_max_travel_time() / self.sampleSeconds;
            let mut traveltime = self.myParent.get_max_travel_time();
            if self.travelledDistance > 0.0 {
                // MIN2(a, b) as std::min: (b < a) ? b : a
                let q1 = self.myLaneLength / self.travelledDistance;
                vehFactor = if q1 < vehFactor { q1 } else { vehFactor };
                let q2 = self.myLaneLength * self.sampleSeconds / self.travelledDistance;
                traveltime = if q2 < traveltime { q2 } else { traveltime };
            }
            if attributeMask.none() {
                dev.write_padding("\n           ");
            }
            dev.set_precision(g_precision());
            dev.write_optional_attr_mask(SUMO_ATTR_TRAVELTIME, &traveltime, mask, false);
            dev.set_precision(g_precision_emissions());
            dev.write_optional_attr_mask(SUMO_ATTR_CO_PERVEH, &(vehFactor * self.myEmissions.CO), mask, false);
            dev.write_optional_attr_mask(SUMO_ATTR_CO2_PERVEH, &(vehFactor * self.myEmissions.CO2), mask, false);
            dev.write_optional_attr_mask(SUMO_ATTR_HC_PERVEH, &(vehFactor * self.myEmissions.HC), mask, false);
            dev.write_optional_attr_mask(SUMO_ATTR_PMX_PERVEH, &(vehFactor * self.myEmissions.PMx), mask, false);
            dev.write_optional_attr_mask(SUMO_ATTR_NOX_PERVEH, &(vehFactor * self.myEmissions.NOx), mask, false);
            dev.write_optional_attr_mask(SUMO_ATTR_FUEL_PERVEH, &(vehFactor * self.myEmissions.fuel), mask, false);
            dev.write_optional_attr_mask(SUMO_ATTR_ELECTRICITY_PERVEH, &(vehFactor * self.myEmissions.electricity), mask, false);
        } else if defaultTravelTime >= 0.0 {
            let t = MSNet::get_instance().get_vehicle_control().get_v_type(DEFAULT_VTYPE_ID, Ptr::null(), false);
            // std::min(myLaneLength / defaultTravelTime, t->getMaxSpeed())
            let q = self.myLaneLength / defaultTravelTime;
            let m = t.get_max_speed();
            let speed = if m < q { m } else { q };

            if attributeMask.none() {
                dev.write_padding("\n           ");
            }
            dev.set_precision(g_precision());
            dev.write_optional_attr_mask(SUMO_ATTR_TRAVELTIME, &defaultTravelTime, mask, false);
            dev.set_precision(g_precision_emissions());
            let accel = t.get_car_follow_model().get_max_accel();
            let params = t.get_emission_parameters();
            let class = t.get_emission_class();
            dev.write_optional_attr_mask(SUMO_ATTR_CO_PERVEH, &PollutantsInterface::compute_default(class, PollutantsInterface_EmissionType::CO, speed, accel, 0.0, defaultTravelTime, params), mask, false);
            dev.write_optional_attr_mask(SUMO_ATTR_CO2_PERVEH, &PollutantsInterface::compute_default(class, PollutantsInterface_EmissionType::CO2, speed, accel, 0.0, defaultTravelTime, params), mask, false);
            dev.write_optional_attr_mask(SUMO_ATTR_HC_PERVEH, &PollutantsInterface::compute_default(class, PollutantsInterface_EmissionType::HC, speed, accel, 0.0, defaultTravelTime, params), mask, false);
            dev.write_optional_attr_mask(SUMO_ATTR_PMX_PERVEH, &PollutantsInterface::compute_default(class, PollutantsInterface_EmissionType::PM_X, speed, accel, 0.0, defaultTravelTime, params), mask, false);
            dev.write_optional_attr_mask(SUMO_ATTR_NOX_PERVEH, &PollutantsInterface::compute_default(class, PollutantsInterface_EmissionType::NO_X, speed, accel, 0.0, defaultTravelTime, params), mask, false);
            dev.write_optional_attr_mask(SUMO_ATTR_FUEL_PERVEH, &PollutantsInterface::compute_default(class, PollutantsInterface_EmissionType::FUEL, speed, accel, 0.0, defaultTravelTime, params), mask, false);
            dev.write_optional_attr_mask(SUMO_ATTR_ELECTRICITY_PERVEH, &PollutantsInterface::compute_default(class, PollutantsInterface_EmissionType::ELEC, speed, accel, 0.0, defaultTravelTime, params), mask, false);
        }
        dev.set_precision(g_precision());
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
        if !self.myParent.is_null() && !self.myParent.vehicle_applies(veh) {
            return;
        }
        if veh.is_vehicle() {
            self.sampleSeconds += arg1;
            self.travelledDistance += travelledDistanceFrontOnLane;
            let a = veh.get_acceleration();
            let e = PollutantsInterface::compute_all(veh.get_vehicle_type().get_emission_class(), arg3, a, veh.get_slope(), Ptr::new(veh).cast::<SUMOVehicle>().m().get_emission_parameters());
            self.myEmissions.add_scaled(&e, arg1);
        }
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
        if veh.is_vehicle() {
            let e = PollutantsInterface::compute_all(veh.get_vehicle_type().get_emission_class(), 0.0, 0.0, 0.0, Ptr::from_mut(veh).cast::<SUMOVehicle>().m().get_emission_parameters());
            self.myEmissions.add_scaled(&e, ts());
        }
        true
    }

}

/// Emission data collector for edges/lanes
/// Virtual dispatch table of MSMeanData_Emissions::MSLaneMeanDataValues (overrides reset, addTo, write, notifyMoveInternal; notifyIdle is not reachable yet)
static MS_MEAN_DATA_VALUES_EMISSIONS_VT: MeanDataValuesVt = MeanDataValuesVt {
    reset: emissions_values_reset,
    add_to: emissions_values_add_to,
    notify_enter: MSMeanData_MeanDataValues::notify_enter_base,
    notify_leave: MSMeanData_MeanDataValues::notify_leave_base,
    notify_move_internal: emissions_values_notify_move_internal,
    is_empty: MSMeanData_MeanDataValues::is_empty_base,
    update: MSMeanData_MeanDataValues::update_base,
    write: emissions_values_write,
    get_samples: MSMeanData_MeanDataValues::get_samples_base,
    get_attribute_value: MSMeanData_MeanDataValues::get_attribute_value_base,
};

fn emissions_values_reset(v: &mut MSMeanData_MeanDataValues, afterWrite: bool) {
    Ptr::from_mut(v).cast::<MSMeanData_Emissions_MSLaneMeanDataValues>().m().reset(afterWrite)
}

fn emissions_values_add_to(v: &MSMeanData_MeanDataValues, val: &mut MSMeanData_MeanDataValues) {
    Ptr::new(v).cast::<MSMeanData_Emissions_MSLaneMeanDataValues>().m().add_to(val)
}

fn emissions_values_notify_move_internal(v: &mut MSMeanData_MeanDataValues, veh: &SUMOTrafficObject, frontOnLane: f64, timeOnLane: f64, meanSpeedFrontOnLane: f64, meanSpeedVehicleOnLane: f64, travelledDistanceFrontOnLane: f64, travelledDistanceVehicleOnLane: f64, meanLengthOnLane: f64) {
    Ptr::from_mut(v).cast::<MSMeanData_Emissions_MSLaneMeanDataValues>().m().notify_move_internal(veh, frontOnLane, timeOnLane, meanSpeedFrontOnLane, meanSpeedVehicleOnLane, travelledDistanceFrontOnLane, travelledDistanceVehicleOnLane, meanLengthOnLane)
}

fn emissions_values_write(v: &MSMeanData_MeanDataValues, dev: &mut OutputDevice, attributeMask: &SumoXMLAttrMask, period: SUMOTime, numLanes: i32, speedLimit: f64, defaultTravelTime: f64, numVehicles: i32) {
    Ptr::new(v).cast::<MSMeanData_Emissions_MSLaneMeanDataValues>().m().write(dev, attributeMask, period, numLanes, speedLimit, defaultTravelTime, numVehicles)
}

/// Virtual dispatch table of MSMeanData_Emissions (overrides getAttributeNames and createValues)
pub(crate) static MS_MEAN_DATA_EMISSIONS_VT: MSMeanDataVt = MSMeanDataVt {
    create_values: emissions_mean_data_create_values,
    get_attribute_names: emissions_mean_data_get_attribute_names,
    write_xml_detector_prolog: MSMeanData::write_xml_detector_prolog_base,
    get_edge_id: MSMeanData::get_edge_id_base,
    open_interval: MSMeanData::open_interval_base,
    write_prefix: MSMeanData::write_prefix_base,
    reset_only: MSMeanData::reset_only_base,
    detector_update: MSMeanData::detector_update_base,
};

fn emissions_detector_write_xml_output(d: &mut MSDetectorFileOutput, dev: &mut OutputDevice, startTime: SUMOTime, stopTime: SUMOTime) {
    Ptr::from_mut(d).cast::<MSMeanData>().m().write_xml_output(dev, startTime, stopTime)
}

fn emissions_detector_write_xml_detector_prolog(d: &MSDetectorFileOutput, dev: &mut OutputDevice) {
    Ptr::new(d).cast::<MSMeanData>().m().write_xml_detector_prolog(dev)
}

fn emissions_detector_detector_update(d: &mut MSDetectorFileOutput, step: SUMOTime) {
    Ptr::from_mut(d).cast::<MSMeanData>().m().detector_update(step)
}

static MS_MEAN_DATA_EMISSIONS_DETECTOR_VT: MSDetectorFileOutputVt = MSDetectorFileOutputVt {
    write_xml_output: emissions_detector_write_xml_output,
    write_xml_detector_prolog: emissions_detector_write_xml_detector_prolog,
    detector_update: emissions_detector_detector_update,
    ..DEFAULT_MSDETECTOR_FILE_OUTPUT_VT
};

fn emissions_mean_data_create_values(m: &MSMeanData, lane: Ptr<MSLane>, length: f64, doAdd: bool) -> Ptr<MSMeanData_MeanDataValues> {
    Ptr::new(m).cast::<MSMeanData_Emissions>().m().create_values(lane, length, doAdd)
}

fn emissions_mean_data_get_attribute_names(m: &MSMeanData) -> Vec<String> {
    Ptr::new(m).cast::<MSMeanData_Emissions>().m().get_attribute_names()
}

/// C++ class `MSMeanData_Emissions` (microsim/output/MSMeanData_Emissions.h)
#[repr(C)]
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
    pub fn new(id: &str, dumpBegin: SUMOTime, dumpEnd: SUMOTime, useLanes: bool, excludeEmpty: &str, withInternal: bool, trackVehicles: bool, maxTravelTime: f64, minSamples: f64, vTypes: &str, writeAttributes: &str, edges: &Vec<Ptr<MSEdge>>, aggregate: AggregateType) -> Self {
        let mut base = MSMeanData::new(id, dumpBegin, dumpEnd, useLanes, excludeEmpty, withInternal, trackVehicles, 0, maxTravelTime, minSamples, vTypes, writeAttributes, edges, aggregate);
        base.meandata_vt = &MS_MEAN_DATA_EMISSIONS_VT;
        base.base.vt = &MS_MEAN_DATA_EMISSIONS_DETECTOR_VT;
        Self { base }
    }

    /// C++: `void MSMeanData_Emissions::~MSMeanData_Emissions() noexcept` [virtual]
    /// Destructor
    // MSMeanData_Emissions::~MSMeanData_Emissions() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
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
        [
            SUMO_ATTR_CO_ABS, SUMO_ATTR_CO2_ABS, SUMO_ATTR_HC_ABS, SUMO_ATTR_PMX_ABS, SUMO_ATTR_NOX_ABS, SUMO_ATTR_FUEL_ABS, SUMO_ATTR_ELECTRICITY_ABS,
            SUMO_ATTR_CO_NORMED, SUMO_ATTR_CO2_NORMED, SUMO_ATTR_HC_NORMED, SUMO_ATTR_PMX_NORMED, SUMO_ATTR_NOX_NORMED, SUMO_ATTR_FUEL_NORMED, SUMO_ATTR_ELECTRICITY_NORMED,
            SUMO_ATTR_CO_PERVEH, SUMO_ATTR_CO2_PERVEH, SUMO_ATTR_HC_PERVEH, SUMO_ATTR_PMX_PERVEH, SUMO_ATTR_NOX_PERVEH, SUMO_ATTR_FUEL_PERVEH, SUMO_ATTR_ELECTRICITY_PERVEH,
        ]
        .iter()
        .map(|a| utils::xml::sumo_xml_definitions::Attrs.get_string(*a).to_string())
        .collect()
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
        [
            SUMO_ATTR_CO_ABS, SUMO_ATTR_CO2_ABS, SUMO_ATTR_HC_ABS, SUMO_ATTR_PMX_ABS, SUMO_ATTR_NOX_ABS, SUMO_ATTR_FUEL_ABS, SUMO_ATTR_ELECTRICITY_ABS,
            SUMO_ATTR_CO_NORMED, SUMO_ATTR_CO2_NORMED, SUMO_ATTR_HC_NORMED, SUMO_ATTR_PMX_NORMED, SUMO_ATTR_NOX_NORMED, SUMO_ATTR_FUEL_NORMED, SUMO_ATTR_ELECTRICITY_NORMED,
            SUMO_ATTR_CO_PERVEH, SUMO_ATTR_CO2_PERVEH, SUMO_ATTR_HC_PERVEH, SUMO_ATTR_PMX_PERVEH, SUMO_ATTR_NOX_PERVEH, SUMO_ATTR_FUEL_PERVEH, SUMO_ATTR_ELECTRICITY_PERVEH,
        ]
        .iter()
        .any(|a| attributeMask.test(*a))
    }

    /// C++: `MSMeanData::MeanDataValues * MSMeanData_Emissions::createValues(MSLane *const, const double, const bool) const` [const, virtual, protected]
    /// Create an instance of MeanDataValues
    // MSMeanData::MeanDataValues*
    // MSMeanData_Emissions::createValues(MSLane* const lane, const double length, const bool doAdd) const {
    //     return new MSLaneMeanDataValues(lane, length, doAdd, this);
    // }
    #[allow(unused_variables)]
    pub fn create_values(&self, lane: Ptr<MSLane>, length: f64, doAdd: bool) -> Ptr<MSMeanData_MeanDataValues> {
        // shortcut: the values are leaked (C++ deletes them in ~MSMeanData), upgrade when myMeasures owns its values
        let values = Box::leak(Box::new(MSMeanData_Emissions_MSLaneMeanDataValues::new(lane, length, doAdd, Ptr::new(self))));
        let values = Ptr::from_mut(&mut values.base);
        register_values(values, lane, doAdd);
        values
    }

    /// C++: `void MSMeanData_Emissions::MSMeanData_Emissions(const MSMeanData_Emissions &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSMeanData_Emissions) -> Self {
        // C++ copy constructor is private and invalidated (never called)
        panic!("MSMeanData_Emissions copy constructor is invalidated");
    }

    /// C++: `MSMeanData_Emissions & MSMeanData_Emissions::operator=(const MSMeanData_Emissions &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSMeanData_Emissions) -> &mut MSMeanData_Emissions {
        // C++ assignment operator is private and invalidated (never called)
        panic!("MSMeanData_Emissions assignment operator is invalidated");
    }

}

