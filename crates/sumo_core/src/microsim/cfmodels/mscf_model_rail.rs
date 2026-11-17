//! Port of microsim/cfmodels/MSCFModel_Rail.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use super::mscf_model::*;
use utils::common::time::{accel2dist, accel2speed, dist2speed, speed2dist, ts};
use utils::geom::geom_helper::{deg2rad, GRAVITY};
use utils::common::std_defs::INVALID_DOUBLE;
use utils::common::to_string::f64_to_string_prec;

// C++ VTYPEPARS_*_SET flags (utils/vehicle/SUMOVTypeParameter.h)
const VTYPEPARS_LENGTH_SET: i64 = 1;
const VTYPEPARS_MAXSPEED_SET: i64 = 1 << 2;
const VTYPEPARS_MASS_SET: i64 = 1i64 << 32;

/// C++ class `MSCFModel_Rail` (microsim/cfmodels/MSCFModel_Rail.h)
#[repr(C)]
pub struct MSCFModel_Rail {
    /// C++ base class `MSCFModel`
    pub base: MSCFModel,
    pub myTrainParams: MSCFModel_Rail_TrainParams,
}

fn rx01_on(veh: Ptr<MSVehicle>) -> bool {
    let t = MSNet::get_instance().get_current_time_step();
    std::env::var_os("RX01").is_some() && format!("{}", veh.get_id()) == "train3" && t >= 180000 && t <= 185000
}
fn rail_follow_speed(m: &MSCFModel, veh: Ptr<MSVehicle>, speed: f64, gap2pred: f64, predSpeed: f64, predMaxDecel: f64, pred: Ptr<MSVehicle>, usage: MSCFModel_CalcReason) -> f64 {
    cf_cast::<MSCFModel_Rail>(m).follow_speed(veh, speed, gap2pred, predSpeed, predMaxDecel, pred, usage)
}
fn rail_get_model_id(m: &MSCFModel) -> i32 {
    cf_cast::<MSCFModel_Rail>(m).get_model_id()
}
fn rail_duplicate(m: &MSCFModel, vtype: Ptr<MSVehicleType>) -> Ptr<MSCFModel> {
    cf_cast::<MSCFModel_Rail>(m).duplicate(vtype)
}
fn rail_save_vehicle_variables(m: &MSCFModel, vars: &MSCFModel_VehicleVariables, out: &mut OutputDevice) {
    unsafe { &*(vars as *const MSCFModel_VehicleVariables as *const MSCFModel_Rail_RailVehicleVariables) }.save_state(out, m)
}
fn rail_load_vehicle_variables(_m: &MSCFModel, vars: &mut MSCFModel_VehicleVariables, attrs: &SUMOSAXAttributes) {
    unsafe { &mut *(vars as *mut MSCFModel_VehicleVariables as *mut MSCFModel_Rail_RailVehicleVariables) }.load_state(attrs)
}
fn rail_create_vehicle_variables(m: &MSCFModel) -> Ptr<MSCFModel_VehicleVariables> {
    cf_cast::<MSCFModel_Rail>(m).create_vehicle_variables()
}
fn rail_max_next_speed(m: &MSCFModel, speed: f64, veh: Ptr<MSVehicle>) -> f64 {
    cf_cast::<MSCFModel_Rail>(m).max_next_speed(speed, veh)
}
fn rail_min_next_speed(m: &MSCFModel, speed: f64, veh: Ptr<MSVehicle>) -> f64 {
    cf_cast::<MSCFModel_Rail>(m).min_next_speed(speed, veh)
}
fn rail_min_next_speed_emergency(m: &MSCFModel, speed: f64, veh: Ptr<MSVehicle>) -> f64 {
    cf_cast::<MSCFModel_Rail>(m).min_next_speed_emergency(speed, veh)
}
fn rail_finalize_speed(m: &MSCFModel, veh: Ptr<MSVehicle>, vPos: f64) -> f64 {
    cf_cast::<MSCFModel_Rail>(m).finalize_speed(veh, vPos)
}
fn rail_free_speed(m: &MSCFModel, veh: Ptr<MSVehicle>, speed: f64, seen: f64, maxSpeed: f64, onInsertion: bool, usage: MSCFModel_CalcReason) -> f64 {
    cf_cast::<MSCFModel_Rail>(m).free_speed(veh, speed, seen, maxSpeed, onInsertion, usage)
}
fn rail_startup_delay_stopped(m: &MSCFModel) -> bool {
    cf_cast::<MSCFModel_Rail>(m).startup_delay_stopped()
}

fn rail_stop_speed_2(m: &MSCFModel, veh: Ptr<MSVehicle>, speed: f64, gap: f64, decel: f64, usage: MSCFModel_CalcReason) -> f64 {
    cf_cast::<MSCFModel_Rail>(m).stop_speed(veh, speed, gap, decel, usage)
}

/// Virtual table of MSCFModel_Rail: its overrides on top of the MSCFModel table
pub const RAIL_VT: MSCFModelVt = MSCFModelVt {
    stop_speed_2: rail_stop_speed_2,
    follow_speed: rail_follow_speed,
    get_model_id: rail_get_model_id,
    duplicate: rail_duplicate,
    create_vehicle_variables: rail_create_vehicle_variables,
    save_vehicle_variables: rail_save_vehicle_variables,
    load_vehicle_variables: rail_load_vehicle_variables,
    max_next_speed: rail_max_next_speed,
    min_next_speed: rail_min_next_speed,
    min_next_speed_emergency: rail_min_next_speed_emergency,
    finalize_speed: rail_finalize_speed,
    free_speed: rail_free_speed,
    startup_delay_stopped: rail_startup_delay_stopped,
    ..DEFAULT_MSCF_MODEL_VT
};

impl std::ops::Deref for MSCFModel_Rail {
    type Target = MSCFModel;
    fn deref(&self) -> &MSCFModel {
        &self.base
    }
}
impl std::ops::DerefMut for MSCFModel_Rail {
    fn deref_mut(&mut self) -> &mut MSCFModel {
        &mut self.base
    }
}

impl MSCFModel_Rail {
    /// C++: `void MSCFModel_Rail::MSCFModel_Rail(const MSVehicleType *)`
    /// Constructor
    // MSCFModel_Rail::MSCFModel_Rail(const MSVehicleType* vtype) :
    //     MSCFModel(vtype) {
    //     const std::string trainType = vtype->getParameter().getCFParamString(SUMO_ATTR_TRAIN_TYPE, "NGT400");
    //     if (trainType.compare("RB425") == 0) {
    //         myTrainParams = initRB425Params();
    //     } else if (trainType.compare("RB628") == 0) {
    //         myTrainParams = initRB628Params();
    //     } else if (trainType.compare("NGT400") == 0) {
    //         myTrainParams = initNGT400Params();
    //     } else if (trainType.compare("NGT400_16") == 0) {
    //         myTrainParams = initNGT400_16Params();
    //     } else if (trainType.compare("ICE1") == 0) {
    //         myTrainParams = initICE1Params();
    //     } else if (trainType.compare("REDosto7") == 0) {
    //         myTrainParams = initREDosto7Params();
    //     } else if (trainType.compare("Freight") == 0) {
    //         myTrainParams = initFreightParams();
    //     } else if (trainType.compare("ICE3") == 0) {
    //         myTrainParams = initICE3Params();
    //     } else if (trainType.compare("MireoPlusB") == 0) {
    //         myTrainParams = initMireoPlusB2TParams();
    //     } else if (trainType.compare("MireoPlusH") == 0) {
    //         myTrainParams = initMireoPlusH2TParams();
    //     } else if (trainType.compare("custom") == 0) {
    //         myTrainParams = initCustomParams();
    //     } else {
    //         WRITE_ERRORF(TL("Unknown train type: %. Exiting!"), trainType);
    //         throw ProcessError();
    //     }
    //     // override with user values
    //     if (vtype->wasSet(VTYPEPARS_MAXSPEED_SET)) {
    //         myTrainParams.vmax = vtype->getMaxSpeed();
    //     }
    //     if (vtype->wasSet(VTYPEPARS_LENGTH_SET)) {
    //         myTrainParams.length = vtype->getLength();
    //     }
    //     myTrainParams.mf = vtype->getParameter().getCFParam(SUMO_ATTR_MASSFACTOR, myTrainParams.mf);
    //     myTrainParams.decl = vtype->getParameter().getCFParam(SUMO_ATTR_DECEL, myTrainParams.decl);
    //     setMaxDecel(myTrainParams.decl);
    //     setEmergencyDecel(vtype->getParameter().getCFParam(SUMO_ATTR_EMERGENCYDECEL, myTrainParams.decl + 0.3));
    //     // update type parameters so they are shown correctly in the gui (if defaults from trainType are used)
    //     const_cast<MSVehicleType*>(vtype)->setMaxSpeed(myTrainParams.vmax);
    //     const_cast<MSVehicleType*>(vtype)->setLength(myTrainParams.length);
    //     if (!vtype->wasSet(VTYPEPARS_MASS_SET)) {
    //         // tons to kg
    //         const_cast<MSVehicleType*>(vtype)->setMass(myTrainParams.weight * 1000);
    //     }
    //
    //     // init tabular curves
    //     myTrainParams.traction = vtype->getParameter().getCFProfile(SUMO_ATTR_TRACTION_TABLE, myTrainParams.traction);
    //     myTrainParams.resistance = vtype->getParameter().getCFProfile(SUMO_ATTR_RESISTANCE_TABLE, myTrainParams.resistance);
    //
    //     // init parametric curves
    //     myTrainParams.maxPower = vtype->getParameter().getCFParam(SUMO_ATTR_MAXPOWER, INVALID_DOUBLE);
    //     myTrainParams.maxTraction = vtype->getParameter().getCFParam(SUMO_ATTR_MAXTRACTION, INVALID_DOUBLE);
    //     myTrainParams.resCoef_constant = vtype->getParameter().getCFParam(SUMO_ATTR_RESISTANCE_COEFFICIENT_CONSTANT, INVALID_DOUBLE);
    //     myTrainParams.resCoef_linear = vtype->getParameter().getCFParam(SUMO_ATTR_RESISTANCE_COEFFICIENT_LINEAR, INVALID_DOUBLE);
    //     myTrainParams.resCoef_quadratic = vtype->getParameter().getCFParam(SUMO_ATTR_RESISTANCE_COEFFICIENT_QUADRATIC, INVALID_DOUBLE);
    //     // curve resistance parameters
    //     myTrainParams.curveResistance = vtype->getParameter().getCFParam(SUMO_ATTR_CURVE_RESISTANCE, myTrainParams.curveResistance);
    //     myTrainParams.curveIntegration = vtype->getParameter().getCFParam(SUMO_ATTR_CURVE_INTEGRATION, myTrainParams.curveIntegration);
    //     myTrainParams.roeckl_sharp_radius = vtype->getParameter().getCFParam(SUMO_ATTR_ROECKL_SHARP_RADIUS, myTrainParams.roeckl_sharp_radius);
    //     myTrainParams.roeckl_numerator = vtype->getParameter().getCFParam(SUMO_ATTR_ROECKL_NUMERATOR, myTrainParams.roeckl_numerator);
    //     myTrainParams.roeckl_numerator_sharp = vtype->getParameter().getCFParam(SUMO_ATTR_ROECKL_NUMERATOR_SHARP, myTrainParams.roeckl_numerator_sharp);
    //     myTrainParams.roeckl_offset = vtype->getParameter().getCFParam(SUMO_ATTR_ROECKL_OFFSET, myTrainParams.roeckl_offset);
    //     myTrainParams.roeckl_offset_sharp = vtype->getParameter().getCFParam(SUMO_ATTR_ROECKL_OFFSET_SHARP, myTrainParams.roeckl_offset_sharp);
    //
    //     if (myTrainParams.maxPower != INVALID_DOUBLE && myTrainParams.maxTraction == INVALID_DOUBLE) {
    //         throw ProcessError(TLF("Undefined maxPower for vType '%'.", vtype->getID()));
    //     } else if (myTrainParams.maxPower == INVALID_DOUBLE && myTrainParams.maxTraction != INVALID_DOUBLE) {
    //         throw ProcessError(TLF("Undefined maxTraction for vType '%'.", vtype->getID()));
    //     }
    //     if (myTrainParams.maxPower != INVALID_DOUBLE && vtype->getParameter().getCFParamString(SUMO_ATTR_TRACTION_TABLE, "") != "") {
    //         WRITE_WARNING(TLF("Ignoring tractionTable because maxPower and maxTraction are set for vType '%'.", vtype->getID()));
    //     }
    //     const bool hasSomeResCoef = (myTrainParams.resCoef_constant != INVALID_DOUBLE
    //                                  || myTrainParams.resCoef_linear != INVALID_DOUBLE
    //                                  || myTrainParams.resCoef_quadratic != INVALID_DOUBLE);
    //     const bool hasAllResCoef = (myTrainParams.resCoef_constant != INVALID_DOUBLE
    //                                 && myTrainParams.resCoef_linear != INVALID_DOUBLE
    //                                 && myTrainParams.resCoef_quadratic != INVALID_DOUBLE);
    //     if (hasSomeResCoef && !hasAllResCoef) {
    //         throw ProcessError(TLF("Some undefined resistance coefficients for vType '%' (requires resCoef_constant, resCoef_linear and resCoef_quadratic)", vtype->getID()));
    //     }
    //     if (myTrainParams.resCoef_constant != INVALID_DOUBLE && vtype->getParameter().getCFParamString(SUMO_ATTR_RESISTANCE_TABLE, "") != "") {
    //         WRITE_WARNING(TLF("Ignoring resistanceTable because resistance coefficients are set for vType '%'.", vtype->getID()));
    //     }
    //
    //     if (myTrainParams.traction.empty() && myTrainParams.maxPower == INVALID_DOUBLE) {
    //         throw ProcessError(TLF("Either tractionTable or maxPower must be defined for vType '%' with Rail model type '%'.", vtype->getID(), trainType));
    //     }
    //     if (myTrainParams.resistance.empty() && myTrainParams.resCoef_constant == INVALID_DOUBLE) {
    //         throw ProcessError(TLF("Either resistanceTable or resCoef_constant must be defined for vType '%' with Rail model type '%'.", vtype->getID(), trainType));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn new(vtype: Ptr<MSVehicleType>) -> Self {
        let base = MSCFModel::new(vtype);
        let mut this = MSCFModel_Rail { base, myTrainParams: MSCFModel_Rail_TrainParams::default() };
        this.base.vt = &RAIL_VT;
        let trainType = vtype.get_parameter().get_cf_param_string(SUMO_ATTR_TRAIN_TYPE, "NGT400".to_string());
        if trainType == "RB425" {
            this.myTrainParams = this.init_rb425_params();
        } else if trainType == "RB628" {
            this.myTrainParams = this.init_rb628_params();
        } else if trainType == "NGT400" {
            this.myTrainParams = this.init_ngt400_params();
        } else if trainType == "NGT400_16" {
            this.myTrainParams = this.init_ngt400_16_params();
        } else if trainType == "ICE1" {
            this.myTrainParams = this.init_ice1_params();
        } else if trainType == "REDosto7" {
            this.myTrainParams = this.init_re_dosto7_params();
        } else if trainType == "Freight" {
            this.myTrainParams = this.init_freight_params();
        } else if trainType == "ICE3" {
            this.myTrainParams = this.init_ice3_params();
        } else if trainType == "MireoPlusB" {
            this.myTrainParams = this.init_mireo_plus_b2_t_params();
        } else if trainType == "MireoPlusH" {
            this.myTrainParams = this.init_mireo_plus_h2_t_params();
        } else if trainType == "custom" {
            this.myTrainParams = this.init_custom_params();
        } else {
            utils::common::msg_handler::write_error(&format!("Unknown train type: {}. Exiting!", trainType));
            panic!("{}", utils::common::util_exceptions::ProcessError::default_msg());
        }
        // override with user values
        if vtype.was_set(VTYPEPARS_MAXSPEED_SET) {
            this.myTrainParams.vmax = vtype.get_max_speed();
        }
        if vtype.was_set(VTYPEPARS_LENGTH_SET) {
            this.myTrainParams.length = vtype.get_length();
        }
        this.myTrainParams.mf = vtype.get_parameter().get_cf_param(SUMO_ATTR_MASSFACTOR, this.myTrainParams.mf);
        this.myTrainParams.decl = vtype.get_parameter().get_cf_param(SUMO_ATTR_DECEL, this.myTrainParams.decl);
        this.base.set_max_decel(this.myTrainParams.decl);
        let emergencyDefault = this.myTrainParams.decl + 0.3;
        this.base.set_emergency_decel(vtype.get_parameter().get_cf_param(SUMO_ATTR_EMERGENCYDECEL, emergencyDefault));
        // update type parameters so they are shown correctly in the gui (if defaults from trainType are used)
        vtype.m().set_max_speed(&this.myTrainParams.vmax);
        vtype.m().set_length(&this.myTrainParams.length);
        if !vtype.was_set(VTYPEPARS_MASS_SET) {
            // tons to kg
            vtype.m().set_mass(this.myTrainParams.weight * 1000.);
        }

        // init tabular curves
        this.myTrainParams.traction = vtype.get_parameter().get_cf_profile(SUMO_ATTR_TRACTION_TABLE, &this.myTrainParams.traction);
        this.myTrainParams.resistance = vtype.get_parameter().get_cf_profile(SUMO_ATTR_RESISTANCE_TABLE, &this.myTrainParams.resistance);

        // init parametric curves
        this.myTrainParams.maxPower = vtype.get_parameter().get_cf_param(SUMO_ATTR_MAXPOWER, INVALID_DOUBLE);
        this.myTrainParams.maxTraction = vtype.get_parameter().get_cf_param(SUMO_ATTR_MAXTRACTION, INVALID_DOUBLE);
        this.myTrainParams.resCoef_constant = vtype.get_parameter().get_cf_param(SUMO_ATTR_RESISTANCE_COEFFICIENT_CONSTANT, INVALID_DOUBLE);
        this.myTrainParams.resCoef_linear = vtype.get_parameter().get_cf_param(SUMO_ATTR_RESISTANCE_COEFFICIENT_LINEAR, INVALID_DOUBLE);
        this.myTrainParams.resCoef_quadratic = vtype.get_parameter().get_cf_param(SUMO_ATTR_RESISTANCE_COEFFICIENT_QUADRATIC, INVALID_DOUBLE);
        // curve resistance parameters
        this.myTrainParams.curveResistance = vtype.get_parameter().get_cf_param(SUMO_ATTR_CURVE_RESISTANCE, this.myTrainParams.curveResistance);
        this.myTrainParams.curveIntegration = vtype.get_parameter().get_cf_param(SUMO_ATTR_CURVE_INTEGRATION, this.myTrainParams.curveIntegration);
        this.myTrainParams.roeckl_sharp_radius = vtype.get_parameter().get_cf_param(SUMO_ATTR_ROECKL_SHARP_RADIUS, this.myTrainParams.roeckl_sharp_radius);
        this.myTrainParams.roeckl_numerator = vtype.get_parameter().get_cf_param(SUMO_ATTR_ROECKL_NUMERATOR, this.myTrainParams.roeckl_numerator);
        this.myTrainParams.roeckl_numerator_sharp = vtype.get_parameter().get_cf_param(SUMO_ATTR_ROECKL_NUMERATOR_SHARP, this.myTrainParams.roeckl_numerator_sharp);
        this.myTrainParams.roeckl_offset = vtype.get_parameter().get_cf_param(SUMO_ATTR_ROECKL_OFFSET, this.myTrainParams.roeckl_offset);
        this.myTrainParams.roeckl_offset_sharp = vtype.get_parameter().get_cf_param(SUMO_ATTR_ROECKL_OFFSET_SHARP, this.myTrainParams.roeckl_offset_sharp);

        if this.myTrainParams.maxPower != INVALID_DOUBLE && this.myTrainParams.maxTraction == INVALID_DOUBLE {
            panic!("{}", utils::common::util_exceptions::ProcessError::new(format!("Undefined maxPower for vType '{}'.", vtype.get_id())));
        } else if this.myTrainParams.maxPower == INVALID_DOUBLE && this.myTrainParams.maxTraction != INVALID_DOUBLE {
            panic!("{}", utils::common::util_exceptions::ProcessError::new(format!("Undefined maxTraction for vType '{}'.", vtype.get_id())));
        }
        if this.myTrainParams.maxPower != INVALID_DOUBLE && vtype.get_parameter().get_cf_param_string(SUMO_ATTR_TRACTION_TABLE, "".to_string()) != "" {
            utils::common::msg_handler::write_warning(&format!("Ignoring tractionTable because maxPower and maxTraction are set for vType '{}'.", vtype.get_id()));
        }
        let hasSomeResCoef = this.myTrainParams.resCoef_constant != INVALID_DOUBLE
            || this.myTrainParams.resCoef_linear != INVALID_DOUBLE
            || this.myTrainParams.resCoef_quadratic != INVALID_DOUBLE;
        let hasAllResCoef = this.myTrainParams.resCoef_constant != INVALID_DOUBLE
            && this.myTrainParams.resCoef_linear != INVALID_DOUBLE
            && this.myTrainParams.resCoef_quadratic != INVALID_DOUBLE;
        if hasSomeResCoef && !hasAllResCoef {
            panic!("{}", utils::common::util_exceptions::ProcessError::new(format!("Some undefined resistance coefficients for vType '{}' (requires resCoef_constant, resCoef_linear and resCoef_quadratic)", vtype.get_id())));
        }
        if this.myTrainParams.resCoef_constant != INVALID_DOUBLE && vtype.get_parameter().get_cf_param_string(SUMO_ATTR_RESISTANCE_TABLE, "".to_string()) != "" {
            utils::common::msg_handler::write_warning(&format!("Ignoring resistanceTable because resistance coefficients are set for vType '{}'.", vtype.get_id()));
        }

        if this.myTrainParams.traction.is_empty() && this.myTrainParams.maxPower == INVALID_DOUBLE {
            panic!("{}", utils::common::util_exceptions::ProcessError::new(format!("Either tractionTable or maxPower must be defined for vType '{}' with Rail model type '{}'.", vtype.get_id(), trainType)));
        }
        if this.myTrainParams.resistance.is_empty() && this.myTrainParams.resCoef_constant == INVALID_DOUBLE {
            panic!("{}", utils::common::util_exceptions::ProcessError::new(format!("Either resistanceTable or resCoef_constant must be defined for vType '{}' with Rail model type '{}'.", vtype.get_id(), trainType)));
        }
        this
    }

    /// C++: `double MSCFModel_Rail::followSpeed(const MSVehicle *const, double, double, double, double, const MSVehicle *const, const CalcReason) const` [const, virtual]
    // double MSCFModel_Rail::followSpeed(const MSVehicle* const veh, double speed, double gap,
    //                                    double /* predSpeed */, double /* predMaxDecel*/, const MSVehicle* const /*pred*/, const CalcReason /*usage*/) const {
    //
    //     // followSpeed module is used for the simulation of moving block operations. The safety gap is chosen similar to the existing german
    //     // system CIR-ELKE (based on LZB). Other implementations of moving block systems may differ, but for now no appropriate parameter
    //     // can be set (would be per lane, not per train) -> hard-coded
    //
    //     // @note: default train minGap of 5 is already subtracted from gap
    //     if (speed >= 30 / 3.6) {
    //         // safety distance for higher speeds (>= 30 km/h)
    //         gap = MAX2(0.0, gap + veh->getVehicleType().getMinGap() - 50);
    //     }
    //
    //     const double vsafe = maximumSafeStopSpeed(gap, myDecel, speed, false, TS, false); // absolute breaking distance
    //     const double vmin = minNextSpeed(speed, veh);
    //     const double vmax = maxNextSpeed(speed, veh);
    //
    //     if (MSGlobals::gSemiImplicitEulerUpdate) {
    //         return MIN2(vsafe, vmax);
    //     } else {
    //         // ballistic
    //         // XXX: the euler variant can break as strong as it wishes immediately! The ballistic cannot, refs. #2575.
    //         return MAX2(MIN2(vsafe, vmax), vmin);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn follow_speed(&self, veh: Ptr<MSVehicle>, speed: f64, gap2pred: f64, predSpeed: f64, predMaxDecel: f64, pred: Ptr<MSVehicle>, usage: MSCFModel_CalcReason) -> f64 {
        let mut gap = gap2pred;
        // @note: default train minGap of 5 is already subtracted from gap
        if speed >= 30. / 3.6 {
            // safety distance for higher speeds (>= 30 km/h)
            gap = max2(0.0, gap + veh.get_vehicle_type().get_min_gap() - 50.);
        }

        let vsafe = self.base.maximum_safe_stop_speed(gap, self.myDecel, speed, false, ts(), false); // absolute breaking distance
        let vmin = self.min_next_speed(speed, veh);
        let vmax = self.max_next_speed(speed, veh);
        if rx01_on(veh) { eprintln!("RX01 follow t={} speed={:x} gap={:x} vsafe={:x} vmin={:x} vmax={:x}", MSNet::get_instance().get_current_time_step(), speed.to_bits(), gap.to_bits(), vsafe.to_bits(), vmin.to_bits(), vmax.to_bits()); }

        if g_semi_implicit_euler_update() {
            min2(vsafe, vmax)
        } else {
            // ballistic
            max2(min2(vsafe, vmax), vmin)
        }
    }

    /// C++: `int MSCFModel_Rail::getModelID() const` [const, virtual]
    // int
    // MSCFModel_Rail::getModelID() const {
    //     return SUMO_TAG_CF_RAIL;
    // }
    #[allow(unused_variables)]
    pub fn get_model_id(&self) -> i32 {
        SUMO_TAG_CF_RAIL as i32
    }

    /// C++: `MSCFModel * MSCFModel_Rail::duplicate(const MSVehicleType *) const` [const, virtual]
    // MSCFModel*
    // MSCFModel_Rail::duplicate(const MSVehicleType* vtype) const {
    //     return new MSCFModel_Rail(vtype);
    // }
    #[allow(unused_variables)]
    pub fn duplicate(&self, vtype: Ptr<MSVehicleType>) -> Ptr<MSCFModel> {
        // shortcut: duplicate() boxes the copy like C++ `new`; the owner must free it once model ownership is ported
        Ptr::from_raw(Box::into_raw(Box::new(MSCFModel_Rail::new(vtype))) as *mut MSCFModel)
    }

    /// C++: `void MSCFModel_Rail::~MSCFModel_Rail() noexcept` [virtual]
    // MSCFModel_Rail::~MSCFModel_Rail() { }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {

    }

    /// C++: `VehicleVariables * MSCFModel_Rail::createVehicleVariables() const` [const, virtual]
    // VehicleVariables* createVehicleVariables() const {
    //         if (myTrainParams.curveResistance > 0) {
    //             return new RailVehicleVariables();
    //         }
    //         return 0;
    //     }
    #[allow(unused_variables)]
    pub fn create_vehicle_variables(&self) -> Ptr<MSCFModel_VehicleVariables> {
        if self.myTrainParams.curveResistance > 0. {
            // shortcut: the vehicle variables are leaked like C++ `new`; the owner must free them once ownership is ported
            return Ptr::from_mut(&mut Box::leak(Box::new(MSCFModel_Rail_RailVehicleVariables {
                base: MSCFModel_VehicleVariables {},
                odometerAngles: Vec::new(),
            })).base);
        }
        Ptr::default()
    }

    /// C++: `double MSCFModel_Rail::maxNextSpeed(double, const MSVehicle *const) const` [const, virtual]
    // double MSCFModel_Rail::maxNextSpeed(double speed, const MSVehicle* const veh) const {
    //
    //     if (speed >= myTrainParams.vmax) {
    //         return myTrainParams.vmax;
    //     }
    //
    //     double targetSpeed = myTrainParams.vmax;
    //
    //     double res = myTrainParams.getResistance(speed); // kN
    //
    //     double slope = veh->getSlope();
    //     double gr = getWeight(veh) * GRAVITY * sin(DEG2RAD(slope)); //kN
    //     double cr = getWeight(veh) * getCurveResistance(veh); //kN
    //
    //     double totalRes = res + gr + cr; //kN
    //
    //     double trac = myTrainParams.getTraction(speed); // kN
    //     double a;
    //     if (speed < targetSpeed) {
    //         a = (trac - totalRes) / getRotWeight(veh); //kN/t == N/kg
    //     } else {
    //         a = 0.;
    //         if (totalRes > trac) {
    //             a = (trac - totalRes) / getRotWeight(veh); //kN/t == N/kg
    //         }
    //     }
    //     double maxNextSpeed = speed + ACCEL2SPEED(a);
    //
    // //    std::cout << veh->getID() << " speed: " << (speed*3.6) << std::endl;
    //
    //     return MIN2(myTrainParams.vmax, maxNextSpeed);
    // }
    #[allow(unused_variables)]
    pub fn max_next_speed(&self, speed: f64, veh: Ptr<MSVehicle>) -> f64 {
        if speed >= self.myTrainParams.vmax {
            return self.myTrainParams.vmax;
        }

        let targetSpeed = self.myTrainParams.vmax;

        let res = self.myTrainParams.get_resistance(speed); // kN

        let slope = veh.get_slope();
        let gr = self.get_weight(veh) * GRAVITY * deg2rad(slope).sin(); //kN
        let cr = self.get_weight(veh) * self.get_curve_resistance(veh); //kN

        let totalRes = res + gr + cr; //kN

        let trac = self.myTrainParams.get_traction(speed); // kN
        let mut a;
        if speed < targetSpeed {
            a = (trac - totalRes) / self.get_rot_weight(veh); //kN/t == N/kg
        } else {
            a = 0.;
            if totalRes > trac {
                a = (trac - totalRes) / self.get_rot_weight(veh); //kN/t == N/kg
            }
        }
        let maxNextSpeed = speed + accel2speed(a);

        if rx01_on(veh) { eprintln!("RX01 max t={} speed={:x} res={:x} gr={:x} cr={:x} trac={:x} a={:x} mns={:x}", MSNet::get_instance().get_current_time_step(), speed.to_bits(), res.to_bits(), gr.to_bits(), cr.to_bits(), trac.to_bits(), a.to_bits(), maxNextSpeed.to_bits()); }
        min2(self.myTrainParams.vmax, maxNextSpeed)
    }

    /// C++: `double MSCFModel_Rail::minNextSpeed(double, const MSVehicle *const) const` [const, virtual]
    // double MSCFModel_Rail::minNextSpeed(double speed, const MSVehicle* const veh) const {
    //
    //     const double slope = veh->getSlope();
    //     const double gr = getWeight(veh) * GRAVITY * sin(DEG2RAD(slope)); //kN
    //     const double cr = getWeight(veh) * getCurveResistance(veh);
    //     const double res = myTrainParams.getResistance(speed); // kN
    //     const double totalRes = res + gr + cr; //kN
    //     const double a = myTrainParams.decl + totalRes / getRotWeight(veh);
    //     const double vMin = speed - ACCEL2SPEED(a);
    //     if (MSGlobals::gSemiImplicitEulerUpdate) {
    //         return MAX2(vMin, 0.);
    //     } else {
    //         // NOTE: ballistic update allows for negative speeds to indicate a stop within the next timestep
    //         return vMin;
    //     }
    //
    // }
    #[allow(unused_variables)]
    pub fn min_next_speed(&self, speed: f64, veh: Ptr<MSVehicle>) -> f64 {
        let slope = veh.get_slope();
        let gr = self.get_weight(veh) * GRAVITY * deg2rad(slope).sin(); //kN
        let cr = self.get_weight(veh) * self.get_curve_resistance(veh);
        let res = self.myTrainParams.get_resistance(speed); // kN
        let totalRes = res + gr + cr; //kN
        let a = self.myTrainParams.decl + totalRes / self.get_rot_weight(veh);
        let vMin = speed - accel2speed(a);
        if rx01_on(veh) { eprintln!("RX01 min t={} speed={:x} res={:x} gr={:x} cr={:x} a={:x} vMin={:x}", MSNet::get_instance().get_current_time_step(), speed.to_bits(), res.to_bits(), gr.to_bits(), cr.to_bits(), a.to_bits(), vMin.to_bits()); }
        if g_semi_implicit_euler_update() {
            max2(vMin, 0.)
        } else {
            // NOTE: ballistic update allows for negative speeds to indicate a stop within the next timestep
            vMin
        }
    }

    /// C++: `double MSCFModel_Rail::minNextSpeedEmergency(double, const MSVehicle *const) const` [const, virtual]
    // double
    // MSCFModel_Rail::minNextSpeedEmergency(double speed, const MSVehicle* const veh) const {
    //     return minNextSpeed(speed, veh);
    // }
    #[allow(unused_variables)]
    pub fn min_next_speed_emergency(&self, speed: f64, veh: Ptr<MSVehicle>) -> f64 {
        self.min_next_speed(speed, veh)
    }

    /// C++: `double MSCFModel_Rail::getSpeedAfterMaxDecel(double) const` [const, virtual]
    // double MSCFModel_Rail::getSpeedAfterMaxDecel(double /* speed */) const {
    //
    // //    //TODO: slope not known here
    // //    double gr = 0; //trainParams.weight * GRAVITY * edge.grade
    // //
    // //    double a = 0;//trainParams.decl - gr/trainParams.rotWeight;
    // //
    // //    return speed + a * DELTA_T / 1000.;
    //     WRITE_ERROR("function call not allowed for rail model. Exiting!");
    //     throw ProcessError();
    // }
    #[allow(unused_variables)]
    pub fn get_speed_after_max_decel(&self, v: f64) -> f64 {
        utils::common::msg_handler::write_error("function call not allowed for rail model. Exiting!");
        panic!("{}", utils::common::util_exceptions::ProcessError::default_msg())
    }

    /// C++: `double MSCFModel_Rail::finalizeSpeed(MSVehicle *const, double) const` [const, virtual]
    // double MSCFModel_Rail::finalizeSpeed(MSVehicle* const veh, double vPos) const {
    //     return MSCFModel::finalizeSpeed(veh, vPos);
    // }
    #[allow(unused_variables)]
    pub fn finalize_speed(&self, veh: Ptr<MSVehicle>, vPos: f64) -> f64 {
        self.base.finalize_speed_base(veh, vPos)
    }

    /// C++: `double MSCFModel_Rail::freeSpeed(const MSVehicle *const, double, double, double, const bool, const CalcReason) const` [const, virtual]
    // double MSCFModel_Rail::freeSpeed(const MSVehicle* const /* veh */, double /* speed */, double dist, double targetSpeed,
    //                                  const bool onInsertion, const CalcReason /*usage*/) const {
    //
    // //    MSCFModel_Rail::VehicleVariables *vars = (MSCFModel_Rail::VehicleVariables *) veh->getCarFollowVariables();
    // //    if (vars->isNotYetInitialized()) {
    // //        initVehicleVariables(veh, vars);
    // //    }
    //
    //     //TODO: signals, coasting, ...
    //
    //     if (MSGlobals::gSemiImplicitEulerUpdate) {
    //         // adapt speed to succeeding lane, no reaction time is involved
    //         // when breaking for y steps the following distance g is covered
    //         // (drive with v in the final step)
    //         // g = (y^2 + y) * 0.5 * b + y * v
    //         // y = ((((sqrt((b + 2.0*v)*(b + 2.0*v) + 8.0*b*g)) - b)*0.5 - v)/b)
    //         const double v = SPEED2DIST(targetSpeed);
    //         if (dist < v) {
    //             return targetSpeed;
    //         }
    //         const double b = ACCEL2DIST(myDecel);
    //         const double y = MAX2(0.0, ((sqrt((b + 2.0 * v) * (b + 2.0 * v) + 8.0 * b * dist) - b) * 0.5 - v) / b);
    //         const double yFull = floor(y);
    //         const double exactGap = (yFull * yFull + yFull) * 0.5 * b + yFull * v + (y > yFull ? v : 0.0);
    //         const double fullSpeedGain = (yFull + (onInsertion ? 1. : 0.)) * ACCEL2SPEED(myTrainParams.decl);
    //         return DIST2SPEED(MAX2(0.0, dist - exactGap) / (yFull + 1)) + fullSpeedGain + targetSpeed;
    //     } else {
    //         WRITE_ERROR(TL("Anything else than semi implicit euler update is not yet implemented. Exiting!"));
    //         throw ProcessError();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn free_speed(&self, veh: Ptr<MSVehicle>, speed: f64, seen: f64, maxSpeed: f64, onInsertion: bool, usage: MSCFModel_CalcReason) -> f64 {
        if g_semi_implicit_euler_update() {
            // adapt speed to succeeding lane, no reaction time is involved
            // when breaking for y steps the following distance g is covered
            // (drive with v in the final step)
            // g = (y^2 + y) * 0.5 * b + y * v
            // y = ((((sqrt((b + 2.0*v)*(b + 2.0*v) + 8.0*b*g)) - b)*0.5 - v)/b)
            let v = speed2dist(maxSpeed);
            if seen < v {
                return maxSpeed;
            }
            let b = accel2dist(self.myDecel);
            let root = ((b + 2.0 * v) * (b + 2.0 * v) + 8.0 * b * seen).sqrt();
            let y = max2(0.0, ((root - b) * 0.5 - v) / b);
            let yFull = y.floor();
            let exactGap = (yFull * yFull + yFull) * 0.5 * b + yFull * v + (if y > yFull { v } else { 0.0 });
            let fullSpeedGain = (yFull + (if onInsertion { 1. } else { 0. })) * accel2speed(self.myTrainParams.decl);
            dist2speed(max2(0.0, seen - exactGap) / (yFull + 1.)) + fullSpeedGain + maxSpeed
        } else {
            utils::common::msg_handler::write_error("Anything else than semi implicit euler update is not yet implemented. Exiting!");
            panic!("{}", utils::common::util_exceptions::ProcessError::default_msg())
        }
    }

    /// C++: `bool MSCFModel_Rail::startupDelayStopped() const` [const, virtual]
    // bool startupDelayStopped() const {
    //         // starup delay in trains is dominated by inertia + brake delay and thus applies to any kind of stopping
    //         return true;
    //     }
    #[allow(unused_variables)]
    pub fn startup_delay_stopped(&self) -> bool {
        // starup delay in trains is dominated by inertia + brake delay and thus applies to any kind of stopping
        true
    }

    /// C++: `double MSCFModel_Rail::stopSpeed(const MSVehicle *const, const double, double, double, const CalcReason) const` [const, virtual]
    // double MSCFModel_Rail::stopSpeed(const MSVehicle* const veh, const double speed, double gap, double decel, const CalcReason /*usage*/) const {
    //     return MIN2(maximumSafeStopSpeed(gap, decel, speed, false, TS, false), maxNextSpeed(speed, veh));
    // }
    #[allow(unused_variables)]
    pub fn stop_speed(&self, veh: Ptr<MSVehicle>, speed: f64, gap: f64, decel: f64, usage: MSCFModel_CalcReason) -> f64 {
        if rx01_on(veh) { eprintln!("RX01 stop t={} speed={:x} gap={:x} decel={:x} vsafe={:x} mns={:x}", MSNet::get_instance().get_current_time_step(), speed.to_bits(), gap.to_bits(), decel.to_bits(), self.base.maximum_safe_stop_speed(gap, decel, speed, false, ts(), false).to_bits(), self.max_next_speed(speed, veh).to_bits()); }
        min2(self.base.maximum_safe_stop_speed(gap, decel, speed, false, ts(), false), self.max_next_speed(speed, veh))
    }

    /// C++: `double MSCFModel_Rail::getRotWeight(const MSVehicle *const) const` [const, protected]
    // double
    // MSCFModel_Rail::getRotWeight(const MSVehicle* const veh) const {
    //     return getWeight(veh) * myTrainParams.mf;
    // }
    #[allow(unused_variables)]
    pub fn get_rot_weight(&self, veh: Ptr<MSVehicle>) -> f64 {
        self.get_weight(veh) * self.myTrainParams.mf
    }

    /// C++: `double MSCFModel_Rail::getWeight(const MSVehicle *const) const` [const, protected]
    // double
    // MSCFModel_Rail::getWeight(const MSVehicle* const veh) const {
    //     // kg to tons
    //     return veh->getVehicleType().getMass() / 1000;
    // }
    #[allow(unused_variables)]
    pub fn get_weight(&self, veh: Ptr<MSVehicle>) -> f64 {
        // kg to tons
        veh.get_vehicle_type().get_mass() / 1000.
    }

    /// C++: `double MSCFModel_Rail::getCurveResistance(const MSVehicle *) const` [const, protected]
    // double
    // MSCFModel_Rail::getCurveResistance(const MSVehicle* veh) const {
    //     if (myTrainParams.curveResistance > 0) {
    //         RailVehicleVariables* vars = (RailVehicleVariables*)veh->getCarFollowVariables();
    //         assert(vars != nullptr);
    //         const double r = vars->getIntegratedRadius(veh, myTrainParams.curveIntegration);
    //         if (r == std::numeric_limits<double>::max()) {
    //             return 0;
    //         } else if (r >= myTrainParams.roeckl_sharp_radius) {
    //             return 0.001 * myTrainParams.curveResistance * myTrainParams.roeckl_numerator / (r - myTrainParams.roeckl_offset);
    //         } else if (r > myTrainParams.roeckl_offset_sharp) {
    //             return 0.001 * myTrainParams.curveResistance * myTrainParams.roeckl_numerator_sharp / (r - myTrainParams.roeckl_offset_sharp);
    //         } else {
    //             WRITE_WARNINGF("Cannot compute curve resistance for vehicle '%' with radius % at time %",
    //                     veh->getID(), r, time2string(SIMSTEP));
    //             return 0;
    //         }
    //     }
    //     return 0;
    // }
    #[allow(unused_variables)]
    pub fn get_curve_resistance(&self, veh: Ptr<MSVehicle>) -> f64 {
        if self.myTrainParams.curveResistance > 0. {
            let vars = Ptr::from_raw(veh.get_car_follow_variables().as_raw() as *mut MSCFModel_Rail_RailVehicleVariables);
            debug_assert!(!vars.is_null());
            let r = vars.m().get_integrated_radius(veh, self.myTrainParams.curveIntegration);
            if r == f64::MAX {
                return 0.;
            } else if r >= self.myTrainParams.roeckl_sharp_radius {
                return 0.001 * self.myTrainParams.curveResistance * self.myTrainParams.roeckl_numerator / (r - self.myTrainParams.roeckl_offset);
            } else if r > self.myTrainParams.roeckl_offset_sharp {
                return 0.001 * self.myTrainParams.curveResistance * self.myTrainParams.roeckl_numerator_sharp / (r - self.myTrainParams.roeckl_offset_sharp);
            } else {
                utils::common::msg_handler::write_warning(&format!("Cannot compute curve resistance for vehicle '{}' with radius {} at time {}", veh.get_id(), utils::common::to_string::f64_to_string_default(r), utils::common::time::time2string(MSNet::get_instance().get_current_time_step())));
                return 0.;
            }
        }
        0.
    }

    /// C++: `LinearApproxHelpers::LinearApproxMap MSCFModel_Rail::initNGT400Traction() const` [const, private]
    // LinearApproxHelpers::LinearApproxMap initNGT400Traction() const { // early version of NGT 400
    //         LinearApproxHelpers::LinearApproxMap map;
    //         map[0] = 716.0;
    //         map[10] = 700.0;
    //         map[20] = 684.0;
    //         map[30] = 669.0;
    //         map[40] = 653.0;
    //         map[50] = 637.0;
    //         map[60] = 622.0;
    //         map[70] = 606.0;
    //         map[80] = 590.0;
    //         map[90] = 574.0;
    //         map[100] = 559.0;
    //         map[110] = 543.0;
    //         map[120] = 527.0;
    //         map[130] = 507.0;
    //         map[140] = 471.0;
    //         map[150] = 439.0;
    //         map[160] = 412.0;
    //         map[170] = 388.0;
    //         map[180] = 366.0;
    //         map[190] = 347.0;
    //         map[200] = 329.0;
    //         map[210] = 314.0;
    //         map[220] = 299.0;
    //         map[230] = 286.0;
    //         map[240] = 275.0;
    //         map[250] = 264.0;
    //         map[260] = 253.0;
    //         map[270] = 244.0;
    //         map[280] = 235.0;
    //         map[290] = 227.0;
    //         map[300] = 220.0;
    //         map[310] = 213.0;
    //         map[320] = 206.0;
    //         map[330] = 200.0;
    //         map[340] = 194.0;
    //         map[350] = 188.0;
    //         map[360] = 183.0;
    //         map[370] = 178.0;
    //         map[380] = 173.0;
    //         map[390] = 169.0;
    //         map[400] = 165.0;
    //         map[410] = 160.6;
    //         map[420] = 156.8;
    //         map[430] = 153.2;
    //         map[440] = 149.7;
    //         map[450] = 146.4;
    //         map[460] = 143.2;
    //         map[470] = 140.1;
    //         map[480] = 137.2;
    //         LinearApproxHelpers::scalePoints(map, 1 / 3.6, 1);
    //         return map;
    //     }
    #[allow(unused_variables)]
    pub fn init_ngt400_traction(&self) -> LinearApproxMap {
        let mut map: LinearApproxMap = Vec::new();
        map.push((0.0, 716.0));
        map.push((10.0, 700.0));
        map.push((20.0, 684.0));
        map.push((30.0, 669.0));
        map.push((40.0, 653.0));
        map.push((50.0, 637.0));
        map.push((60.0, 622.0));
        map.push((70.0, 606.0));
        map.push((80.0, 590.0));
        map.push((90.0, 574.0));
        map.push((100.0, 559.0));
        map.push((110.0, 543.0));
        map.push((120.0, 527.0));
        map.push((130.0, 507.0));
        map.push((140.0, 471.0));
        map.push((150.0, 439.0));
        map.push((160.0, 412.0));
        map.push((170.0, 388.0));
        map.push((180.0, 366.0));
        map.push((190.0, 347.0));
        map.push((200.0, 329.0));
        map.push((210.0, 314.0));
        map.push((220.0, 299.0));
        map.push((230.0, 286.0));
        map.push((240.0, 275.0));
        map.push((250.0, 264.0));
        map.push((260.0, 253.0));
        map.push((270.0, 244.0));
        map.push((280.0, 235.0));
        map.push((290.0, 227.0));
        map.push((300.0, 220.0));
        map.push((310.0, 213.0));
        map.push((320.0, 206.0));
        map.push((330.0, 200.0));
        map.push((340.0, 194.0));
        map.push((350.0, 188.0));
        map.push((360.0, 183.0));
        map.push((370.0, 178.0));
        map.push((380.0, 173.0));
        map.push((390.0, 169.0));
        map.push((400.0, 165.0));
        map.push((410.0, 160.6));
        map.push((420.0, 156.8));
        map.push((430.0, 153.2));
        map.push((440.0, 149.7));
        map.push((450.0, 146.4));
        map.push((460.0, 143.2));
        map.push((470.0, 140.1));
        map.push((480.0, 137.2));
        map = map.into_iter().map(|(k, v)| (k * (1.0 / 3.6), v * (1.0))).collect();
        map
    }

    /// C++: `LinearApproxHelpers::LinearApproxMap MSCFModel_Rail::initNGT400Resistance() const` [const, private]
    // LinearApproxHelpers::LinearApproxMap initNGT400Resistance() const { // early version of NGT 400
    //         LinearApproxHelpers::LinearApproxMap map;
    //         map[0] = 1.9;
    //         map[10] = 2.1;
    //         map[20] = 2.4;
    //         map[30] = 2.9;
    //         map[40] = 3.5;
    //         map[50] = 4.2;
    //         map[60] = 5.0;
    //         map[70] = 6.0;
    //         map[80] = 7.0;
    //         map[90] = 8.2;
    //         map[100] = 9.6;
    //         map[110] = 11.0;
    //         map[120] = 12.6;
    //         map[130] = 14.3;
    //         map[140] = 16.1;
    //         map[150] = 18.0;
    //         map[160] = 20.1;
    //         map[170] = 22.3;
    //         map[180] = 24.6;
    //         map[190] = 27.0;
    //         map[200] = 29.5;
    //         map[210] = 32.2;
    //         map[220] = 35.0;
    //         map[230] = 37.9;
    //         map[240] = 40.9;
    //         map[250] = 44.1;
    //         map[260] = 47.4;
    //         map[270] = 50.8;
    //         map[280] = 54.3;
    //         map[290] = 58.0;
    //         map[300] = 61.7;
    //         map[310] = 65.6;
    //         map[320] = 69.7;
    //         map[330] = 73.8;
    //         map[340] = 78.1;
    //         map[350] = 82.4;
    //         map[360] = 86.9;
    //         map[370] = 91.6;
    //         map[380] = 96.3;
    //         map[390] = 101.2;
    //         map[400] = 106.2;
    //         map[410] = 111.3;
    //         map[420] = 116.6;
    //         map[430] = 121.9;
    //         map[440] = 127.4;
    //         map[450] = 133.0;
    //         map[460] = 138.7;
    //         map[470] = 144.6;
    //         map[480] = 150.6;
    //         LinearApproxHelpers::scalePoints(map, 1 / 3.6, 1);
    //         return map;
    //     }
    #[allow(unused_variables)]
    pub fn init_ngt400_resistance(&self) -> LinearApproxMap {
        let mut map: LinearApproxMap = Vec::new();
        map.push((0.0, 1.9));
        map.push((10.0, 2.1));
        map.push((20.0, 2.4));
        map.push((30.0, 2.9));
        map.push((40.0, 3.5));
        map.push((50.0, 4.2));
        map.push((60.0, 5.0));
        map.push((70.0, 6.0));
        map.push((80.0, 7.0));
        map.push((90.0, 8.2));
        map.push((100.0, 9.6));
        map.push((110.0, 11.0));
        map.push((120.0, 12.6));
        map.push((130.0, 14.3));
        map.push((140.0, 16.1));
        map.push((150.0, 18.0));
        map.push((160.0, 20.1));
        map.push((170.0, 22.3));
        map.push((180.0, 24.6));
        map.push((190.0, 27.0));
        map.push((200.0, 29.5));
        map.push((210.0, 32.2));
        map.push((220.0, 35.0));
        map.push((230.0, 37.9));
        map.push((240.0, 40.9));
        map.push((250.0, 44.1));
        map.push((260.0, 47.4));
        map.push((270.0, 50.8));
        map.push((280.0, 54.3));
        map.push((290.0, 58.0));
        map.push((300.0, 61.7));
        map.push((310.0, 65.6));
        map.push((320.0, 69.7));
        map.push((330.0, 73.8));
        map.push((340.0, 78.1));
        map.push((350.0, 82.4));
        map.push((360.0, 86.9));
        map.push((370.0, 91.6));
        map.push((380.0, 96.3));
        map.push((390.0, 101.2));
        map.push((400.0, 106.2));
        map.push((410.0, 111.3));
        map.push((420.0, 116.6));
        map.push((430.0, 121.9));
        map.push((440.0, 127.4));
        map.push((450.0, 133.0));
        map.push((460.0, 138.7));
        map.push((470.0, 144.6));
        map.push((480.0, 150.6));
        map = map.into_iter().map(|(k, v)| (k * (1.0 / 3.6), v * (1.0))).collect();
        map
    }

    /// C++: `TrainParams MSCFModel_Rail::initNGT400Params() const` [const, private]
    // TrainParams initNGT400Params() const {
    //         TrainParams params;
    //         params.weight = 384;
    //         params.mf = 1.04;
    //         params.length = 200;
    //         params.decl = 0.9;
    //         params.vmax = 500 / 3.6;
    //         params.recovery = 0.6;
    //         params.traction = initNGT400Traction();
    //         params.resistance = initNGT400Resistance();
    //         return params;
    //     }
    #[allow(unused_variables)]
    pub fn init_ngt400_params(&self) -> MSCFModel_Rail_TrainParams {
        let mut params = MSCFModel_Rail_TrainParams::default();
        params.weight = 384.0;
        params.mf = 1.04;
        params.length = 200.0;
        params.decl = 0.9;
        params.vmax = 500.0 / 3.6;
        params.recovery = 0.6;
        params.traction = self.init_ngt400_traction();
        params.resistance = self.init_ngt400_resistance();
        params
    }

    /// C++: `LinearApproxHelpers::LinearApproxMap MSCFModel_Rail::initNGT400_16Traction() const` [const, private]
    // LinearApproxHelpers::LinearApproxMap initNGT400_16Traction() const {
    //         LinearApproxHelpers::LinearApproxMap map;
    //         map[0] = 274.5;
    //         map[10] = 274.5;
    //         map[20] = 274.5;
    //         map[30] = 274.5;
    //         map[40] = 274.5;
    //         map[50] = 274.5;
    //         map[60] = 274.5;
    //         map[70] = 274.5;
    //         map[80] = 274.5;
    //         map[90] = 274.5;
    //         map[100] = 274.5;
    //         map[110] = 274.5;
    //         map[120] = 274.5;
    //         map[130] = 274.5;
    //         map[140] = 274.5;
    //         map[150] = 274.5;
    //         map[160] = 274.5;
    //         map[170] = 274.5;
    //         map[180] = 274.5;
    //         map[190] = 274.5;
    //         map[200] = 274.5;
    //         map[210] = 273;
    //         map[220] = 262.3;
    //         map[230] = 250;
    //         map[240] = 239;
    //         map[250] = 229;
    //         map[260] = 222;
    //         map[270] = 213;
    //         map[280] = 205;
    //         map[290] = 198;
    //         map[300] = 191;
    //         map[310] = 186;
    //         map[320] = 180;
    //         map[330] = 174;
    //         map[340] = 169;
    //         map[350] = 165;
    //         map[360] = 160;
    //         map[370] = 155.5;
    //         map[380] = 151;
    //         map[390] = 148;
    //         map[400] = 144;
    //         map[410] = 140;
    //         map[420] = 136;
    //         map[430] = 134;
    //         map[440] = 131;
    //         map[450] = 128;
    //         map[460] = 125;
    //         map[470] = 123;
    //         LinearApproxHelpers::scalePoints(map, 1 / 3.6, 1);
    //         return map;
    //     }
    #[allow(unused_variables)]
    pub fn init_ngt400_16_traction(&self) -> LinearApproxMap {
        let mut map: LinearApproxMap = Vec::new();
        map.push((0.0, 274.5));
        map.push((10.0, 274.5));
        map.push((20.0, 274.5));
        map.push((30.0, 274.5));
        map.push((40.0, 274.5));
        map.push((50.0, 274.5));
        map.push((60.0, 274.5));
        map.push((70.0, 274.5));
        map.push((80.0, 274.5));
        map.push((90.0, 274.5));
        map.push((100.0, 274.5));
        map.push((110.0, 274.5));
        map.push((120.0, 274.5));
        map.push((130.0, 274.5));
        map.push((140.0, 274.5));
        map.push((150.0, 274.5));
        map.push((160.0, 274.5));
        map.push((170.0, 274.5));
        map.push((180.0, 274.5));
        map.push((190.0, 274.5));
        map.push((200.0, 274.5));
        map.push((210.0, 273.0));
        map.push((220.0, 262.3));
        map.push((230.0, 250.0));
        map.push((240.0, 239.0));
        map.push((250.0, 229.0));
        map.push((260.0, 222.0));
        map.push((270.0, 213.0));
        map.push((280.0, 205.0));
        map.push((290.0, 198.0));
        map.push((300.0, 191.0));
        map.push((310.0, 186.0));
        map.push((320.0, 180.0));
        map.push((330.0, 174.0));
        map.push((340.0, 169.0));
        map.push((350.0, 165.0));
        map.push((360.0, 160.0));
        map.push((370.0, 155.5));
        map.push((380.0, 151.0));
        map.push((390.0, 148.0));
        map.push((400.0, 144.0));
        map.push((410.0, 140.0));
        map.push((420.0, 136.0));
        map.push((430.0, 134.0));
        map.push((440.0, 131.0));
        map.push((450.0, 128.0));
        map.push((460.0, 125.0));
        map.push((470.0, 123.0));
        map = map.into_iter().map(|(k, v)| (k * (1.0 / 3.6), v * (1.0))).collect();
        map
    }

    /// C++: `LinearApproxHelpers::LinearApproxMap MSCFModel_Rail::initNGT400_16Resistance() const` [const, private]
    // LinearApproxHelpers::LinearApproxMap initNGT400_16Resistance() const {
    //         LinearApproxHelpers::LinearApproxMap map;
    //         map[0] = 5.71;
    //         map[10] = 6.01;
    //         map[20] = 6.4;
    //         map[30] = 7.0;
    //         map[40] = 7.51;
    //         map[50] = 8.34;
    //         map[60] = 9.2;
    //         map[70] = 10.4;
    //         map[80] = 11.3;
    //         map[90] = 12.58;
    //         map[100] = 14.05;
    //         map[110] = 15.5;
    //         map[120] = 16.9;
    //         map[130] = 18.75;
    //         map[140] = 20.77;
    //         map[150] = 22.9;
    //         map[160] = 25.0;
    //         map[170] = 26.9;
    //         map[180] = 29.38;
    //         map[190] = 32.0;
    //         map[200] = 34.5;
    //         map[210] = 37.0;
    //         map[220] = 39.91;
    //         map[230] = 43.11;
    //         map[240] = 46.4;
    //         map[250] = 49.3;
    //         map[260] = 52.4;
    //         map[270] = 56.09;
    //         map[280] = 59.8;
    //         map[290] = 63.7;
    //         map[300] = 66.8;
    //         map[310] = 71.0;
    //         map[320] = 75.39;
    //         map[330] = 79.9;
    //         map[340] = 84.1;
    //         map[350] = 87.9;
    //         map[360] = 92.7;
    //         map[370] = 97.6;
    //         map[380] = 102.0;
    //         map[390] = 107.0;
    //         map[400] = 112.0;
    //         map[410] = 117.5;
    //         map[420] = 123.0;
    //         map[430] = 128.3;
    //         map[440] = 133.3;
    //         map[450] = 139.2;
    //         map[460] = 145.5;
    //         map[470] = 150.0;
    //         LinearApproxHelpers::scalePoints(map, 1 / 3.6, 1);
    //         return map;
    //     }
    #[allow(unused_variables)]
    pub fn init_ngt400_16_resistance(&self) -> LinearApproxMap {
        let mut map: LinearApproxMap = Vec::new();
        map.push((0.0, 5.71));
        map.push((10.0, 6.01));
        map.push((20.0, 6.4));
        map.push((30.0, 7.0));
        map.push((40.0, 7.51));
        map.push((50.0, 8.34));
        map.push((60.0, 9.2));
        map.push((70.0, 10.4));
        map.push((80.0, 11.3));
        map.push((90.0, 12.58));
        map.push((100.0, 14.05));
        map.push((110.0, 15.5));
        map.push((120.0, 16.9));
        map.push((130.0, 18.75));
        map.push((140.0, 20.77));
        map.push((150.0, 22.9));
        map.push((160.0, 25.0));
        map.push((170.0, 26.9));
        map.push((180.0, 29.38));
        map.push((190.0, 32.0));
        map.push((200.0, 34.5));
        map.push((210.0, 37.0));
        map.push((220.0, 39.91));
        map.push((230.0, 43.11));
        map.push((240.0, 46.4));
        map.push((250.0, 49.3));
        map.push((260.0, 52.4));
        map.push((270.0, 56.09));
        map.push((280.0, 59.8));
        map.push((290.0, 63.7));
        map.push((300.0, 66.8));
        map.push((310.0, 71.0));
        map.push((320.0, 75.39));
        map.push((330.0, 79.9));
        map.push((340.0, 84.1));
        map.push((350.0, 87.9));
        map.push((360.0, 92.7));
        map.push((370.0, 97.6));
        map.push((380.0, 102.0));
        map.push((390.0, 107.0));
        map.push((400.0, 112.0));
        map.push((410.0, 117.5));
        map.push((420.0, 123.0));
        map.push((430.0, 128.3));
        map.push((440.0, 133.3));
        map.push((450.0, 139.2));
        map.push((460.0, 145.5));
        map.push((470.0, 150.0));
        map = map.into_iter().map(|(k, v)| (k * (1.0 / 3.6), v * (1.0))).collect();
        map
    }

    /// C++: `TrainParams MSCFModel_Rail::initNGT400_16Params() const` [const, private]
    // TrainParams initNGT400_16Params() const {
    //         TrainParams params;
    //         params.weight = 384;
    //         params.mf = 1.04;
    //         params.length = 200;
    //         params.decl = 0.9;
    //         params.vmax = 430 / 3.6;
    //         params.recovery = 0.6;
    //         params.traction = initNGT400_16Traction();
    //         params.resistance = initNGT400_16Resistance();
    //         return params;
    //     }
    #[allow(unused_variables)]
    pub fn init_ngt400_16_params(&self) -> MSCFModel_Rail_TrainParams {
        let mut params = MSCFModel_Rail_TrainParams::default();
        params.weight = 384.0;
        params.mf = 1.04;
        params.length = 200.0;
        params.decl = 0.9;
        params.vmax = 430.0 / 3.6;
        params.recovery = 0.6;
        params.traction = self.init_ngt400_16_traction();
        params.resistance = self.init_ngt400_16_resistance();
        params
    }

    /// C++: `LinearApproxHelpers::LinearApproxMap MSCFModel_Rail::initICE1Traction() const` [const, private]
    // LinearApproxHelpers::LinearApproxMap initICE1Traction() const {
    //         LinearApproxHelpers::LinearApproxMap map;
    //         map[0] = 400;
    //         map[10] = 394;
    //         map[20] = 388;
    //         map[30] = 382;
    //         map[40] = 377;
    //         map[50] = 372;
    //         map[60] = 369;
    //         map[70] = 366;
    //         map[80] = 363;
    //         map[90] = 361;
    //         map[100] = 349;
    //         map[110] = 317;
    //         map[120] = 290;
    //         map[130] = 268;
    //         map[140] = 249;
    //         map[150] = 232;
    //         map[160] = 217;
    //         map[170] = 205;
    //         map[180] = 193;
    //         map[190] = 183;
    //         map[200] = 174;
    //         map[210] = 165;
    //         map[220] = 158;
    //         map[230] = 151;
    //         map[240] = 145;
    //         map[250] = 139;
    //         LinearApproxHelpers::scalePoints(map, 1 / 3.6, 1);
    //         return map;
    //     }
    #[allow(unused_variables)]
    pub fn init_ice1_traction(&self) -> LinearApproxMap {
        let mut map: LinearApproxMap = Vec::new();
        map.push((0.0, 400.0));
        map.push((10.0, 394.0));
        map.push((20.0, 388.0));
        map.push((30.0, 382.0));
        map.push((40.0, 377.0));
        map.push((50.0, 372.0));
        map.push((60.0, 369.0));
        map.push((70.0, 366.0));
        map.push((80.0, 363.0));
        map.push((90.0, 361.0));
        map.push((100.0, 349.0));
        map.push((110.0, 317.0));
        map.push((120.0, 290.0));
        map.push((130.0, 268.0));
        map.push((140.0, 249.0));
        map.push((150.0, 232.0));
        map.push((160.0, 217.0));
        map.push((170.0, 205.0));
        map.push((180.0, 193.0));
        map.push((190.0, 183.0));
        map.push((200.0, 174.0));
        map.push((210.0, 165.0));
        map.push((220.0, 158.0));
        map.push((230.0, 151.0));
        map.push((240.0, 145.0));
        map.push((250.0, 139.0));
        map = map.into_iter().map(|(k, v)| (k * (1.0 / 3.6), v * (1.0))).collect();
        map
    }

    /// C++: `LinearApproxHelpers::LinearApproxMap MSCFModel_Rail::initICE1Resistance() const` [const, private]
    // LinearApproxHelpers::LinearApproxMap initICE1Resistance() const {
    //         LinearApproxHelpers::LinearApproxMap map;
    //         map[0] = 10.7;
    //         map[10] = 12.3;
    //         map[20] = 14.2;
    //         map[30] = 16.4;
    //         map[40] = 18.7;
    //         map[50] = 21.3;
    //         map[60] = 24.2;
    //         map[70] = 27.3;
    //         map[80] = 30.6;
    //         map[90] = 34.1;
    //         map[100] = 37.9;
    //         map[110] = 41.9;
    //         map[120] = 46.2;
    //         map[130] = 50.6;
    //         map[140] = 55.4;
    //         map[150] = 60.4;
    //         map[160] = 65.6;
    //         map[170] = 71.1;
    //         map[180] = 76.7;
    //         map[190] = 82.6;
    //         map[200] = 88.8;
    //         map[210] = 95.2;
    //         map[220] = 101.8;
    //         map[230] = 108.7;
    //         map[240] = 115.8;
    //         map[250] = 123.1;
    //         LinearApproxHelpers::scalePoints(map, 1 / 3.6, 1);
    //         return map;
    //     }
    #[allow(unused_variables)]
    pub fn init_ice1_resistance(&self) -> LinearApproxMap {
        let mut map: LinearApproxMap = Vec::new();
        map.push((0.0, 10.7));
        map.push((10.0, 12.3));
        map.push((20.0, 14.2));
        map.push((30.0, 16.4));
        map.push((40.0, 18.7));
        map.push((50.0, 21.3));
        map.push((60.0, 24.2));
        map.push((70.0, 27.3));
        map.push((80.0, 30.6));
        map.push((90.0, 34.1));
        map.push((100.0, 37.9));
        map.push((110.0, 41.9));
        map.push((120.0, 46.2));
        map.push((130.0, 50.6));
        map.push((140.0, 55.4));
        map.push((150.0, 60.4));
        map.push((160.0, 65.6));
        map.push((170.0, 71.1));
        map.push((180.0, 76.7));
        map.push((190.0, 82.6));
        map.push((200.0, 88.8));
        map.push((210.0, 95.2));
        map.push((220.0, 101.8));
        map.push((230.0, 108.7));
        map.push((240.0, 115.8));
        map.push((250.0, 123.1));
        map = map.into_iter().map(|(k, v)| (k * (1.0 / 3.6), v * (1.0))).collect();
        map
    }

    /// C++: `TrainParams MSCFModel_Rail::initICE1Params() const` [const, private]
    // TrainParams initICE1Params() const {
    //         TrainParams params;
    //         params.weight = 876;
    //         params.mf = 1.1;
    //         params.length = 358;
    //         params.decl = 0.5;
    //         params.vmax = 250 / 3.6;
    //         params.recovery = 0.1;
    //         params.traction = initICE1Traction();
    //         params.resistance = initICE1Resistance();
    //         return params;
    //     }
    #[allow(unused_variables)]
    pub fn init_ice1_params(&self) -> MSCFModel_Rail_TrainParams {
        let mut params = MSCFModel_Rail_TrainParams::default();
        params.weight = 876.0;
        params.mf = 1.1;
        params.length = 358.0;
        params.decl = 0.5;
        params.vmax = 250.0 / 3.6;
        params.recovery = 0.1;
        params.traction = self.init_ice1_traction();
        params.resistance = self.init_ice1_resistance();
        params
    }

    /// C++: `TrainParams MSCFModel_Rail::initICE3Params() const` [const, private]
    // TrainParams initICE3Params() const {
    //         TrainParams params;
    //         params.weight = 420;
    //         params.mf = 1.04;
    //         params.length = 200;
    //         params.decl = 0.5;
    //         params.vmax = 300 / 3.6;
    //         params.recovery = 0.6;
    //         params.traction = initICE3Traction();
    //         params.resistance = initICE3Resistance();
    //         return params;
    //     }
    #[allow(unused_variables)]
    pub fn init_ice3_params(&self) -> MSCFModel_Rail_TrainParams {
        let mut params = MSCFModel_Rail_TrainParams::default();
        params.weight = 420.0;
        params.mf = 1.04;
        params.length = 200.0;
        params.decl = 0.5;
        params.vmax = 300.0 / 3.6;
        params.recovery = 0.6;
        params.traction = self.init_ice3_traction();
        params.resistance = self.init_ice3_resistance();
        params
    }

    /// C++: `LinearApproxHelpers::LinearApproxMap MSCFModel_Rail::initICE3Traction() const` [const, private]
    // LinearApproxHelpers::LinearApproxMap initICE3Traction() const {
    //         LinearApproxHelpers::LinearApproxMap map;
    //         map[0] = 300;
    //         map[10] = 298;
    //         map[20] = 297;
    //         map[30] = 295;
    //         map[40] = 293;
    //         map[50] = 292;
    //         map[60] = 290;
    //         map[70] = 288;
    //         map[80] = 286.5;
    //         map[90] = 285;
    //         map[100] = 283;
    //         map[110] = 262;
    //         map[120] = 240;
    //         map[130] = 221;
    //         map[140] = 206;
    //         map[150] = 192;
    //         map[160] = 180;
    //         map[170] = 169;
    //         map[180] = 160;
    //         map[190] = 152;
    //         map[200] = 144;
    //         map[210] = 137;
    //         map[220] = 131;
    //         map[230] = 125;
    //         map[240] = 120;
    //         map[250] = 115;
    //         map[260] = 111;
    //         map[270] = 107;
    //         map[280] = 103;
    //         map[290] = 99;
    //         map[300] = 96;
    //         LinearApproxHelpers::scalePoints(map, 1 / 3.6, 1);
    //         return map;
    //     }
    #[allow(unused_variables)]
    pub fn init_ice3_traction(&self) -> LinearApproxMap {
        let mut map: LinearApproxMap = Vec::new();
        map.push((0.0, 300.0));
        map.push((10.0, 298.0));
        map.push((20.0, 297.0));
        map.push((30.0, 295.0));
        map.push((40.0, 293.0));
        map.push((50.0, 292.0));
        map.push((60.0, 290.0));
        map.push((70.0, 288.0));
        map.push((80.0, 286.5));
        map.push((90.0, 285.0));
        map.push((100.0, 283.0));
        map.push((110.0, 262.0));
        map.push((120.0, 240.0));
        map.push((130.0, 221.0));
        map.push((140.0, 206.0));
        map.push((150.0, 192.0));
        map.push((160.0, 180.0));
        map.push((170.0, 169.0));
        map.push((180.0, 160.0));
        map.push((190.0, 152.0));
        map.push((200.0, 144.0));
        map.push((210.0, 137.0));
        map.push((220.0, 131.0));
        map.push((230.0, 125.0));
        map.push((240.0, 120.0));
        map.push((250.0, 115.0));
        map.push((260.0, 111.0));
        map.push((270.0, 107.0));
        map.push((280.0, 103.0));
        map.push((290.0, 99.0));
        map.push((300.0, 96.0));
        map = map.into_iter().map(|(k, v)| (k * (1.0 / 3.6), v * (1.0))).collect();
        map
    }

    /// C++: `LinearApproxHelpers::LinearApproxMap MSCFModel_Rail::initICE3Resistance() const` [const, private]
    // LinearApproxHelpers::LinearApproxMap initICE3Resistance() const {
    //         LinearApproxHelpers::LinearApproxMap map;
    //         map[0] = 7.4;
    //         map[10] = 7.6;
    //         map[20] = 8.0;
    //         map[30] = 8.4;
    //         map[40] = 9.1;
    //         map[50] = 9.8;
    //         map[60] = 10.7;
    //         map[70] = 11.7;
    //         map[80] = 12.8;
    //         map[90] = 14.1;
    //         map[100] = 15.5;
    //         map[110] = 17.1;
    //         map[120] = 18.8;
    //         map[130] = 20.6;
    //         map[140] = 22.6;
    //         map[150] = 24.6;
    //         map[160] = 26.9;
    //         map[170] = 29.2;
    //         map[180] = 31.7;
    //         map[190] = 34.3;
    //         map[200] = 37.1;
    //         map[210] = 40.0;
    //         map[220] = 43.1;
    //         map[230] = 46.2;
    //         map[240] = 49.6;
    //         map[250] = 53.0;
    //         map[260] = 56.6;
    //         map[270] = 60.3;
    //         map[280] = 64.1;
    //         map[290] = 68.1;
    //         map[300] = 71.8;
    //         LinearApproxHelpers::scalePoints(map, 1 / 3.6, 1);
    //         return map;
    //     }
    #[allow(unused_variables)]
    pub fn init_ice3_resistance(&self) -> LinearApproxMap {
        let mut map: LinearApproxMap = Vec::new();
        map.push((0.0, 7.4));
        map.push((10.0, 7.6));
        map.push((20.0, 8.0));
        map.push((30.0, 8.4));
        map.push((40.0, 9.1));
        map.push((50.0, 9.8));
        map.push((60.0, 10.7));
        map.push((70.0, 11.7));
        map.push((80.0, 12.8));
        map.push((90.0, 14.1));
        map.push((100.0, 15.5));
        map.push((110.0, 17.1));
        map.push((120.0, 18.8));
        map.push((130.0, 20.6));
        map.push((140.0, 22.6));
        map.push((150.0, 24.6));
        map.push((160.0, 26.9));
        map.push((170.0, 29.2));
        map.push((180.0, 31.7));
        map.push((190.0, 34.3));
        map.push((200.0, 37.1));
        map.push((210.0, 40.0));
        map.push((220.0, 43.1));
        map.push((230.0, 46.2));
        map.push((240.0, 49.6));
        map.push((250.0, 53.0));
        map.push((260.0, 56.6));
        map.push((270.0, 60.3));
        map.push((280.0, 64.1));
        map.push((290.0, 68.1));
        map.push((300.0, 71.8));
        map = map.into_iter().map(|(k, v)| (k * (1.0 / 3.6), v * (1.0))).collect();
        map
    }

    /// C++: `TrainParams MSCFModel_Rail::initREDosto7Params() const` [const, private]
    // TrainParams initREDosto7Params() const {
    //         TrainParams params;
    //         params.weight = 425.5;
    //         params.mf = 1.1;
    //         params.length = 207;
    //         params.decl = 0.5;
    //         params.vmax = 160 / 3.6;
    //         params.recovery = 0.1;
    //         params.traction = initREDosto7Traction();
    //         params.resistance = initREDosto7Resistance();
    //         return params;
    //     }
    #[allow(unused_variables)]
    pub fn init_re_dosto7_params(&self) -> MSCFModel_Rail_TrainParams {
        let mut params = MSCFModel_Rail_TrainParams::default();
        params.weight = 425.5;
        params.mf = 1.1;
        params.length = 207.0;
        params.decl = 0.5;
        params.vmax = 160.0 / 3.6;
        params.recovery = 0.1;
        params.traction = self.init_re_dosto7_traction();
        params.resistance = self.init_re_dosto7_resistance();
        params
    }

    /// C++: `LinearApproxHelpers::LinearApproxMap MSCFModel_Rail::initREDosto7Traction() const` [const, private]
    // LinearApproxHelpers::LinearApproxMap initREDosto7Traction() const {
    //         LinearApproxHelpers::LinearApproxMap map;
    //         map[0] = 300;
    //         map[10] = 300;
    //         map[20] = 300;
    //         map[30] = 300;
    //         map[40] = 300;
    //         map[50] = 300;
    //         map[60] = 300;
    //         map[70] = 289;
    //         map[80] = 253;
    //         map[90] = 224;
    //         map[100] = 202;
    //         map[110] = 183;
    //         map[120] = 168;
    //         map[130] = 155;
    //         map[140] = 144;
    //         map[150] = 134;
    //         map[160] = 125;
    //         LinearApproxHelpers::scalePoints(map, 1 / 3.6, 1);
    //         return map;
    //     }
    #[allow(unused_variables)]
    pub fn init_re_dosto7_traction(&self) -> LinearApproxMap {
        let mut map: LinearApproxMap = Vec::new();
        map.push((0.0, 300.0));
        map.push((10.0, 300.0));
        map.push((20.0, 300.0));
        map.push((30.0, 300.0));
        map.push((40.0, 300.0));
        map.push((50.0, 300.0));
        map.push((60.0, 300.0));
        map.push((70.0, 289.0));
        map.push((80.0, 253.0));
        map.push((90.0, 224.0));
        map.push((100.0, 202.0));
        map.push((110.0, 183.0));
        map.push((120.0, 168.0));
        map.push((130.0, 155.0));
        map.push((140.0, 144.0));
        map.push((150.0, 134.0));
        map.push((160.0, 125.0));
        map = map.into_iter().map(|(k, v)| (k * (1.0 / 3.6), v * (1.0))).collect();
        map
    }

    /// C++: `LinearApproxHelpers::LinearApproxMap MSCFModel_Rail::initREDosto7Resistance() const` [const, private]
    // LinearApproxHelpers::LinearApproxMap initREDosto7Resistance() const {
    //         LinearApproxHelpers::LinearApproxMap map;
    //         map[0] = 8.5;
    //         map[10] = 8.9;
    //         map[20] = 9.5;
    //         map[30] = 10.4;
    //         map[40] = 11.4;
    //         map[50] = 12.7;
    //         map[60] = 14.1;
    //         map[70] = 15.8;
    //         map[80] = 17.7;
    //         map[90] = 19.8;
    //         map[100] = 22.6;
    //         map[110] = 24.6;
    //         map[120] = 27.3;
    //         map[130] = 30.2;
    //         map[140] = 33.3;
    //         map[150] = 36.6;
    //         map[160] = 40.2;
    //         LinearApproxHelpers::scalePoints(map, 1 / 3.6, 1);
    //         return map;
    //     }
    #[allow(unused_variables)]
    pub fn init_re_dosto7_resistance(&self) -> LinearApproxMap {
        let mut map: LinearApproxMap = Vec::new();
        map.push((0.0, 8.5));
        map.push((10.0, 8.9));
        map.push((20.0, 9.5));
        map.push((30.0, 10.4));
        map.push((40.0, 11.4));
        map.push((50.0, 12.7));
        map.push((60.0, 14.1));
        map.push((70.0, 15.8));
        map.push((80.0, 17.7));
        map.push((90.0, 19.8));
        map.push((100.0, 22.6));
        map.push((110.0, 24.6));
        map.push((120.0, 27.3));
        map.push((130.0, 30.2));
        map.push((140.0, 33.3));
        map.push((150.0, 36.6));
        map.push((160.0, 40.2));
        map = map.into_iter().map(|(k, v)| (k * (1.0 / 3.6), v * (1.0))).collect();
        map
    }

    /// C++: `TrainParams MSCFModel_Rail::initRB628Params() const` [const, private]
    // TrainParams initRB628Params() const {
    //         TrainParams params;
    //         params.weight = 72.2;
    //         params.mf = 1.04;
    //         params.length = 46;
    //         params.decl = 0.5;
    //         params.vmax = 120 / 3.6;
    //         params.recovery = 0;
    //         params.traction = initRB628Traction();
    //         params.resistance = initRB628Resistance();
    //         return params;
    //     }
    #[allow(unused_variables)]
    pub fn init_rb628_params(&self) -> MSCFModel_Rail_TrainParams {
        let mut params = MSCFModel_Rail_TrainParams::default();
        params.weight = 72.2;
        params.mf = 1.04;
        params.length = 46.0;
        params.decl = 0.5;
        params.vmax = 120.0 / 3.6;
        params.recovery = 0.0;
        params.traction = self.init_rb628_traction();
        params.resistance = self.init_rb628_resistance();
        params
    }

    /// C++: `LinearApproxHelpers::LinearApproxMap MSCFModel_Rail::initRB628Traction() const` [const, private]
    // LinearApproxHelpers::LinearApproxMap initRB628Traction() const {
    //         LinearApproxHelpers::LinearApproxMap map;
    //         map[0] = 60;
    //         map[10] = 53.8;
    //         map[20] = 47.6;
    //         map[30] = 36.9;
    //         map[40] = 28.7;
    //         map[50] = 23.5;
    //         map[60] = 20.0;
    //         map[70] = 17.5;
    //         map[80] = 15.2;
    //         map[90] = 13.9;
    //         map[100] = 12.8;
    //         map[110] = 11.7;
    //         map[120] = 10.8;
    //         LinearApproxHelpers::scalePoints(map, 1 / 3.6, 1);
    //         return map;
    //     }
    #[allow(unused_variables)]
    pub fn init_rb628_traction(&self) -> LinearApproxMap {
        let mut map: LinearApproxMap = Vec::new();
        map.push((0.0, 60.0));
        map.push((10.0, 53.8));
        map.push((20.0, 47.6));
        map.push((30.0, 36.9));
        map.push((40.0, 28.7));
        map.push((50.0, 23.5));
        map.push((60.0, 20.0));
        map.push((70.0, 17.5));
        map.push((80.0, 15.2));
        map.push((90.0, 13.9));
        map.push((100.0, 12.8));
        map.push((110.0, 11.7));
        map.push((120.0, 10.8));
        map = map.into_iter().map(|(k, v)| (k * (1.0 / 3.6), v * (1.0))).collect();
        map
    }

    /// C++: `LinearApproxHelpers::LinearApproxMap MSCFModel_Rail::initRB628Resistance() const` [const, private]
    // LinearApproxHelpers::LinearApproxMap initRB628Resistance() const {
    //         LinearApproxHelpers::LinearApproxMap map;
    //         map[0] = 1.29;
    //         map[10] = 1.46;
    //         map[20] = 1.73;
    //         map[30] = 2.08;
    //         map[40] = 2.52;
    //         map[50] = 3.05;
    //         map[60] = 3.66;
    //         map[70] = 4.36;
    //         map[80] = 5.16;
    //         map[90] = 6.03;
    //         map[100] = 7.00;
    //         map[110] = 8.06;
    //         map[120] = 9.2;
    //         LinearApproxHelpers::scalePoints(map, 1 / 3.6, 1);
    //         return map;
    //     }
    #[allow(unused_variables)]
    pub fn init_rb628_resistance(&self) -> LinearApproxMap {
        let mut map: LinearApproxMap = Vec::new();
        map.push((0.0, 1.29));
        map.push((10.0, 1.46));
        map.push((20.0, 1.73));
        map.push((30.0, 2.08));
        map.push((40.0, 2.52));
        map.push((50.0, 3.05));
        map.push((60.0, 3.66));
        map.push((70.0, 4.36));
        map.push((80.0, 5.16));
        map.push((90.0, 6.03));
        map.push((100.0, 7.00));
        map.push((110.0, 8.06));
        map.push((120.0, 9.2));
        map = map.into_iter().map(|(k, v)| (k * (1.0 / 3.6), v * (1.0))).collect();
        map
    }

    /// C++: `TrainParams MSCFModel_Rail::initFreightParams() const` [const, private]
    // TrainParams initFreightParams() const {
    //         TrainParams params;
    //         params.weight = 1440;
    //         params.mf = 1.06;
    //         params.length = 512;
    //         params.decl = 0.3;
    //         params.vmax = 120 / 3.6;
    //         params.recovery = 0.05;
    //         params.traction = initFreightTraction();
    //         params.resistance = initFreightResistance();
    //         return params;
    //     }
    #[allow(unused_variables)]
    pub fn init_freight_params(&self) -> MSCFModel_Rail_TrainParams {
        let mut params = MSCFModel_Rail_TrainParams::default();
        params.weight = 1440.0;
        params.mf = 1.06;
        params.length = 512.0;
        params.decl = 0.3;
        params.vmax = 120.0 / 3.6;
        params.recovery = 0.05;
        params.traction = self.init_freight_traction();
        params.resistance = self.init_freight_resistance();
        params
    }

    /// C++: `LinearApproxHelpers::LinearApproxMap MSCFModel_Rail::initFreightTraction() const` [const, private]
    // LinearApproxHelpers::LinearApproxMap initFreightTraction() const {
    //         LinearApproxHelpers::LinearApproxMap map;
    //         map[0] = 300;
    //         map[10] = 296;
    //         map[20] = 293;
    //         map[30] = 289;
    //         map[40] = 286;
    //         map[50] = 282;
    //         map[60] = 279;
    //         map[70] = 275;
    //         map[80] = 272;
    //         map[90] = 255;
    //         map[100] = 230;
    //         map[110] = 209;
    //         map[120] = 190;//guessed value
    //         LinearApproxHelpers::scalePoints(map, 1 / 3.6, 1);
    //         return map;
    //     }
    #[allow(unused_variables)]
    pub fn init_freight_traction(&self) -> LinearApproxMap {
        let mut map: LinearApproxMap = Vec::new();
        map.push((0.0, 300.0));
        map.push((10.0, 296.0));
        map.push((20.0, 293.0));
        map.push((30.0, 289.0));
        map.push((40.0, 286.0));
        map.push((50.0, 282.0));
        map.push((60.0, 279.0));
        map.push((70.0, 275.0));
        map.push((80.0, 272.0));
        map.push((90.0, 255.0));
        map.push((100.0, 230.0));
        map.push((110.0, 209.0));
        map.push((120.0, 190.0));
        map = map.into_iter().map(|(k, v)| (k * (1.0 / 3.6), v * (1.0))).collect();
        map
    }

    /// C++: `LinearApproxHelpers::LinearApproxMap MSCFModel_Rail::initFreightResistance() const` [const, private]
    // LinearApproxHelpers::LinearApproxMap initFreightResistance() const {
    //         LinearApproxHelpers::LinearApproxMap map;
    //         map[0] = 1.9;
    //         map[10] = 4.3;
    //         map[20] = 8.5;
    //         map[30] = 14.3;
    //         map[40] = 21.7;
    //         map[50] = 30.8;
    //         map[60] = 41.5;
    //         map[70] = 53.8;
    //         map[80] = 67.8;
    //         map[90] = 83.5;
    //         map[100] = 110.7;
    //         map[110] = 119.6;
    //         map[120] = 140.2;
    //         LinearApproxHelpers::scalePoints(map, 1 / 3.6, 1);
    //         return map;
    //     }
    #[allow(unused_variables)]
    pub fn init_freight_resistance(&self) -> LinearApproxMap {
        let mut map: LinearApproxMap = Vec::new();
        map.push((0.0, 1.9));
        map.push((10.0, 4.3));
        map.push((20.0, 8.5));
        map.push((30.0, 14.3));
        map.push((40.0, 21.7));
        map.push((50.0, 30.8));
        map.push((60.0, 41.5));
        map.push((70.0, 53.8));
        map.push((80.0, 67.8));
        map.push((90.0, 83.5));
        map.push((100.0, 110.7));
        map.push((110.0, 119.6));
        map.push((120.0, 140.2));
        map = map.into_iter().map(|(k, v)| (k * (1.0 / 3.6), v * (1.0))).collect();
        map
    }

    /// C++: `LinearApproxHelpers::LinearApproxMap MSCFModel_Rail::initRB425Traction() const` [const, private]
    // LinearApproxHelpers::LinearApproxMap initRB425Traction() const {
    //         LinearApproxHelpers::LinearApproxMap map;
    //         map[0] = 150;
    //         map[10] = 150;
    //         map[20] = 150;
    //         map[30] = 150;
    //         map[40] = 150;
    //         map[50] = 150;
    //         map[60] = 140;
    //         map[70] = 120;
    //         map[80] = 105;
    //         map[90] = 93;
    //         map[100] = 84;
    //         map[110] = 75;
    //         map[120] = 70;
    //         map[130] = 56;
    //         map[140] = 52;
    //         map[150] = 46;
    //         map[160] = 40;
    //         LinearApproxHelpers::scalePoints(map, 1 / 3.6, 1);
    //         return map;
    //     }
    #[allow(unused_variables)]
    pub fn init_rb425_traction(&self) -> LinearApproxMap {
        let mut map: LinearApproxMap = Vec::new();
        map.push((0.0, 150.0));
        map.push((10.0, 150.0));
        map.push((20.0, 150.0));
        map.push((30.0, 150.0));
        map.push((40.0, 150.0));
        map.push((50.0, 150.0));
        map.push((60.0, 140.0));
        map.push((70.0, 120.0));
        map.push((80.0, 105.0));
        map.push((90.0, 93.0));
        map.push((100.0, 84.0));
        map.push((110.0, 75.0));
        map.push((120.0, 70.0));
        map.push((130.0, 56.0));
        map.push((140.0, 52.0));
        map.push((150.0, 46.0));
        map.push((160.0, 40.0));
        map = map.into_iter().map(|(k, v)| (k * (1.0 / 3.6), v * (1.0))).collect();
        map
    }

    /// C++: `LinearApproxHelpers::LinearApproxMap MSCFModel_Rail::initRB425Resistance() const` [const, private]
    // LinearApproxHelpers::LinearApproxMap initRB425Resistance() const {
    //         LinearApproxHelpers::LinearApproxMap map;
    //         map[0] = 2.6;
    //         map[10] = 2.9;
    //         map[20] = 3.3;
    //         map[30] = 3.7;
    //         map[40] = 4.3;
    //         map[50] = 4.9;
    //         map[60] = 5.7;
    //         map[70] = 6.6;
    //         map[80] = 7.5;
    //         map[90] = 8.6;
    //         map[100] = 9.7;
    //         map[110] = 11.0;
    //         map[120] = 12.3;
    //         map[130] = 13.8;
    //         map[140] = 15.3;
    //         map[150] = 16.9;
    //         map[160] = 18.7;
    //         LinearApproxHelpers::scalePoints(map, 1 / 3.6, 1);
    //         return map;
    //     }
    #[allow(unused_variables)]
    pub fn init_rb425_resistance(&self) -> LinearApproxMap {
        let mut map: LinearApproxMap = Vec::new();
        map.push((0.0, 2.6));
        map.push((10.0, 2.9));
        map.push((20.0, 3.3));
        map.push((30.0, 3.7));
        map.push((40.0, 4.3));
        map.push((50.0, 4.9));
        map.push((60.0, 5.7));
        map.push((70.0, 6.6));
        map.push((80.0, 7.5));
        map.push((90.0, 8.6));
        map.push((100.0, 9.7));
        map.push((110.0, 11.0));
        map.push((120.0, 12.3));
        map.push((130.0, 13.8));
        map.push((140.0, 15.3));
        map.push((150.0, 16.9));
        map.push((160.0, 18.7));
        map = map.into_iter().map(|(k, v)| (k * (1.0 / 3.6), v * (1.0))).collect();
        map
    }

    /// C++: `TrainParams MSCFModel_Rail::initRB425Params() const` [const, private]
    // TrainParams initRB425Params() const {
    //         TrainParams params;
    //         params.weight = 138;
    //         params.mf = 1.04;
    //         params.length = 67.5;
    //         params.decl = 1.0;
    //         params.vmax = 160 / 3.6;
    //         params.recovery = 0.6;
    //         params.traction = initRB425Traction();
    //         params.resistance = initRB425Resistance();
    //         return params;
    //     }
    #[allow(unused_variables)]
    pub fn init_rb425_params(&self) -> MSCFModel_Rail_TrainParams {
        let mut params = MSCFModel_Rail_TrainParams::default();
        params.weight = 138.0;
        params.mf = 1.04;
        params.length = 67.5;
        params.decl = 1.0;
        params.vmax = 160.0 / 3.6;
        params.recovery = 0.6;
        params.traction = self.init_rb425_traction();
        params.resistance = self.init_rb425_resistance();
        params
    }

    /// C++: `LinearApproxHelpers::LinearApproxMap MSCFModel_Rail::initMireoPlusB2TTraction() const` [const, private]
    // LinearApproxHelpers::LinearApproxMap initMireoPlusB2TTraction() const {
    //         LinearApproxHelpers::LinearApproxMap map;
    //         map[0] = 106.15;
    //         map[10] = 106.15;
    //         map[20] = 106.15;
    //         map[30] = 106.15;
    //         map[40] = 106.15;
    //         map[50] = 106.15;
    //         map[60] = 103.73;
    //         map[70] = 88.70;
    //         map[80] = 77.47;
    //         map[90] = 68.76;
    //         map[100] = 61.82;
    //         map[110] = 56.15;
    //         map[120] = 51.43;
    //         map[130] = 47.44;
    //         map[140] = 44.03;
    //         map[150] = 41.07;
    //         map[160] = 38.49;
    //         LinearApproxHelpers::scalePoints(map, 1 / 3.6, 1);
    //         return map;
    //     }
    #[allow(unused_variables)]
    pub fn init_mireo_plus_b2_t_traction(&self) -> LinearApproxMap {
        let mut map: LinearApproxMap = Vec::new();
        map.push((0.0, 106.15));
        map.push((10.0, 106.15));
        map.push((20.0, 106.15));
        map.push((30.0, 106.15));
        map.push((40.0, 106.15));
        map.push((50.0, 106.15));
        map.push((60.0, 103.73));
        map.push((70.0, 88.70));
        map.push((80.0, 77.47));
        map.push((90.0, 68.76));
        map.push((100.0, 61.82));
        map.push((110.0, 56.15));
        map.push((120.0, 51.43));
        map.push((130.0, 47.44));
        map.push((140.0, 44.03));
        map.push((150.0, 41.07));
        map.push((160.0, 38.49));
        map = map.into_iter().map(|(k, v)| (k * (1.0 / 3.6), v * (1.0))).collect();
        map
    }

    /// C++: `LinearApproxHelpers::LinearApproxMap MSCFModel_Rail::initMireoPlusB2TResistance() const` [const, private]
    // LinearApproxHelpers::LinearApproxMap initMireoPlusB2TResistance() const {
    //         LinearApproxHelpers::LinearApproxMap map;
    //         map[0] = 1.01;
    //         map[10] = 1.09;
    //         map[20] = 1.27;
    //         map[30] = 1.55;
    //         map[40] = 1.93;
    //         map[50] = 2.41;
    //         map[60] = 2.99;
    //         map[70] = 3.67;
    //         map[80] = 4.46;
    //         map[90] = 5.34;
    //         map[100] = 6.34;
    //         map[110] = 7.44;
    //         map[120] = 8.64;
    //         map[130] = 9.96;
    //         map[140] = 11.38;
    //         map[150] = 12.91;
    //         map[160] = 14.56;
    //         LinearApproxHelpers::scalePoints(map, 1 / 3.6, 1);
    //         return map;
    //     }
    #[allow(unused_variables)]
    pub fn init_mireo_plus_b2_t_resistance(&self) -> LinearApproxMap {
        let mut map: LinearApproxMap = Vec::new();
        map.push((0.0, 1.01));
        map.push((10.0, 1.09));
        map.push((20.0, 1.27));
        map.push((30.0, 1.55));
        map.push((40.0, 1.93));
        map.push((50.0, 2.41));
        map.push((60.0, 2.99));
        map.push((70.0, 3.67));
        map.push((80.0, 4.46));
        map.push((90.0, 5.34));
        map.push((100.0, 6.34));
        map.push((110.0, 7.44));
        map.push((120.0, 8.64));
        map.push((130.0, 9.96));
        map.push((140.0, 11.38));
        map.push((150.0, 12.91));
        map.push((160.0, 14.56));
        map = map.into_iter().map(|(k, v)| (k * (1.0 / 3.6), v * (1.0))).collect();
        map
    }

    /// C++: `TrainParams MSCFModel_Rail::initMireoPlusB2TParams() const` [const, private]
    // TrainParams initMireoPlusB2TParams() const {
    //         TrainParams params;
    //         params.weight = 105.5;
    //         params.mf = 1.05;
    //         params.length = 46.56;
    //         params.decl = 1.1;
    //         params.vmax = 160 / 3.6;
    //         params.recovery = 0.3;
    //         params.traction = initMireoPlusB2TTraction();
    //         params.resistance = initMireoPlusB2TResistance();
    //         return params;
    //     }
    #[allow(unused_variables)]
    pub fn init_mireo_plus_b2_t_params(&self) -> MSCFModel_Rail_TrainParams {
        let mut params = MSCFModel_Rail_TrainParams::default();
        params.weight = 105.5;
        params.mf = 1.05;
        params.length = 46.56;
        params.decl = 1.1;
        params.vmax = 160.0 / 3.6;
        params.recovery = 0.3;
        params.traction = self.init_mireo_plus_b2_t_traction();
        params.resistance = self.init_mireo_plus_b2_t_resistance();
        params
    }

    /// C++: `LinearApproxHelpers::LinearApproxMap MSCFModel_Rail::initMireoPlusH2TTraction() const` [const, private]
    // LinearApproxHelpers::LinearApproxMap initMireoPlusH2TTraction() const {
    //         LinearApproxHelpers::LinearApproxMap map;
    //         map[0] = 104.50;
    //         map[10] = 104.50;
    //         map[20] = 104.50;
    //         map[30] = 104.50;
    //         map[40] = 104.50;
    //         map[50] = 104.50;
    //         map[60] = 102.00;
    //         map[70] = 87.43;
    //         map[80] = 76.50;
    //         map[90] = 68.00;
    //         map[100] = 61.20;
    //         map[110] = 55.64;
    //         map[120] = 51.00;
    //         map[130] = 47.08;
    //         map[140] = 43.71;
    //         map[150] = 40.80;
    //         map[160] = 38.25;
    //         LinearApproxHelpers::scalePoints(map, 1 / 3.6, 1);
    //         return map;
    //     }
    #[allow(unused_variables)]
    pub fn init_mireo_plus_h2_t_traction(&self) -> LinearApproxMap {
        let mut map: LinearApproxMap = Vec::new();
        map.push((0.0, 104.50));
        map.push((10.0, 104.50));
        map.push((20.0, 104.50));
        map.push((30.0, 104.50));
        map.push((40.0, 104.50));
        map.push((50.0, 104.50));
        map.push((60.0, 102.00));
        map.push((70.0, 87.43));
        map.push((80.0, 76.50));
        map.push((90.0, 68.00));
        map.push((100.0, 61.20));
        map.push((110.0, 55.64));
        map.push((120.0, 51.00));
        map.push((130.0, 47.08));
        map.push((140.0, 43.71));
        map.push((150.0, 40.80));
        map.push((160.0, 38.25));
        map = map.into_iter().map(|(k, v)| (k * (1.0 / 3.6), v * (1.0))).collect();
        map
    }

    /// C++: `LinearApproxHelpers::LinearApproxMap MSCFModel_Rail::initMireoPlusH2TResistance() const` [const, private]
    // LinearApproxHelpers::LinearApproxMap initMireoPlusH2TResistance() const {
    //         LinearApproxHelpers::LinearApproxMap map;
    //         map[0] = 1.01;
    //         map[10] = 1.09;
    //         map[20] = 1.27;
    //         map[30] = 1.55;
    //         map[40] = 1.93;
    //         map[50] = 2.41;
    //         map[60] = 2.99;
    //         map[70] = 3.67;
    //         map[80] = 4.45;
    //         map[90] = 5.34;
    //         map[100] = 6.34;
    //         map[110] = 7.43;
    //         map[120] = 8.64;
    //         map[130] = 9.95;
    //         map[140] = 11.38;
    //         map[150] = 12.91;
    //         map[160] = 14.56;
    //         LinearApproxHelpers::scalePoints(map, 1 / 3.6, 1);
    //         return map;
    //     }
    #[allow(unused_variables)]
    pub fn init_mireo_plus_h2_t_resistance(&self) -> LinearApproxMap {
        let mut map: LinearApproxMap = Vec::new();
        map.push((0.0, 1.01));
        map.push((10.0, 1.09));
        map.push((20.0, 1.27));
        map.push((30.0, 1.55));
        map.push((40.0, 1.93));
        map.push((50.0, 2.41));
        map.push((60.0, 2.99));
        map.push((70.0, 3.67));
        map.push((80.0, 4.45));
        map.push((90.0, 5.34));
        map.push((100.0, 6.34));
        map.push((110.0, 7.43));
        map.push((120.0, 8.64));
        map.push((130.0, 9.95));
        map.push((140.0, 11.38));
        map.push((150.0, 12.91));
        map.push((160.0, 14.56));
        map = map.into_iter().map(|(k, v)| (k * (1.0 / 3.6), v * (1.0))).collect();
        map
    }

    /// C++: `TrainParams MSCFModel_Rail::initMireoPlusH2TParams() const` [const, private]
    // TrainParams initMireoPlusH2TParams() const {
    //         TrainParams params;
    //         params.weight = 105.4;
    //         params.mf = 1.05;
    //         params.length = 46.56;
    //         params.decl = 1.1;
    //         params.vmax = 160 / 3.6;
    //         params.recovery = 0.3;
    //         params.traction = initMireoPlusH2TTraction();
    //         params.resistance = initMireoPlusH2TResistance();
    //         return params;
    //     }
    #[allow(unused_variables)]
    pub fn init_mireo_plus_h2_t_params(&self) -> MSCFModel_Rail_TrainParams {
        let mut params = MSCFModel_Rail_TrainParams::default();
        params.weight = 105.4;
        params.mf = 1.05;
        params.length = 46.56;
        params.decl = 1.1;
        params.vmax = 160.0 / 3.6;
        params.recovery = 0.3;
        params.traction = self.init_mireo_plus_h2_t_traction();
        params.resistance = self.init_mireo_plus_h2_t_resistance();
        params
    }

    /// C++: `TrainParams MSCFModel_Rail::initCustomParams() const` [const, private]
    // TrainParams initCustomParams() const {
    //         TrainParams params;
    //         params.weight = 100;
    //         params.mf = 1.05;
    //         params.length = 100;
    //         params.decl = 1;
    //         params.vmax = 200 / 3.6;
    //         params.recovery = 0.3;
    //         return params;
    //     }
    #[allow(unused_variables)]
    pub fn init_custom_params(&self) -> MSCFModel_Rail_TrainParams {
        let mut params = MSCFModel_Rail_TrainParams::default();
        params.weight = 100.0;
        params.mf = 1.05;
        params.length = 100.0;
        params.decl = 1.0;
        params.vmax = 200.0 / 3.6;
        params.recovery = 0.3;
        params
    }

}

/// C++ class `MSCFModel_Rail::RailVehicleVariables` (microsim/cfmodels/MSCFModel_Rail.h)
#[repr(C)]
pub struct MSCFModel_Rail_RailVehicleVariables {
    /// C++ base class `MSCFModel::VehicleVariables`
    pub base: MSCFModel_VehicleVariables,
    pub odometerAngles: Vec<(f64, f64)>,
}

impl std::ops::Deref for MSCFModel_Rail_RailVehicleVariables {
    type Target = MSCFModel_VehicleVariables;
    fn deref(&self) -> &MSCFModel_VehicleVariables {
        &self.base
    }
}
impl std::ops::DerefMut for MSCFModel_Rail_RailVehicleVariables {
    fn deref_mut(&mut self) -> &mut MSCFModel_VehicleVariables {
        &mut self.base
    }
}

impl MSCFModel_Rail_RailVehicleVariables {
    /// C++: `double RailVehicleVariables::getIntegratedRadius(const MSVehicle *, double)`
    // double
    // MSCFModel_Rail::RailVehicleVariables::getIntegratedRadius(const MSVehicle* veh, double curveIntegration) {
    //     const double odo = veh->getOdometer();
    //     // add new data point
    //     if ((odometerAngles.empty() || odometerAngles.back().first != odo) && veh->hasDeparted()) {
    //         odometerAngles.push_back(std::make_pair(odo, veh->getAngle()));
    //         // clean up old data points beyond integration distance
    //         while (odometerAngles.size() > 2) {
    //             double distCleaned = odometerAngles.back().first - odometerAngles[1].first;
    //             if (distCleaned >= curveIntegration) {
    //                 odometerAngles.erase(odometerAngles.begin());
    //             } else {
    //                 break;
    //             }
    //         }
    //     }
    //     if (odometerAngles.size() > 1) {
    //         const double dist = odometerAngles.back().first - odometerAngles.front().first;
    //         const double angleDiff = GeomHelper::angleDiff(odometerAngles.back().second, odometerAngles.front().second);
    //         return angleDiff == 0
    //             ? std::numeric_limits<double>::max()
    //             : dist / fabs(angleDiff);
    //     } else {
    //         return veh->getCurveRadius();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_integrated_radius(&mut self, veh: Ptr<MSVehicle>, curveIntegration: f64) -> f64 {
        let odo = veh.get_odometer();
        // add new data point
        if (self.odometerAngles.is_empty() || self.odometerAngles.last().unwrap().0 != odo) && veh.has_departed() {
            self.odometerAngles.push((odo, veh.get_angle()));
            // clean up old data points beyond integration distance
            while self.odometerAngles.len() > 2 {
                let distCleaned = self.odometerAngles.last().unwrap().0 - self.odometerAngles[1].0;
                if distCleaned >= curveIntegration {
                    self.odometerAngles.remove(0);
                } else {
                    break;
                }
            }
        }
        if self.odometerAngles.len() > 1 {
            let dist = self.odometerAngles.last().unwrap().0 - self.odometerAngles.first().unwrap().0;
            let angleDiff = utils::geom::geom_helper::angle_diff(self.odometerAngles.last().unwrap().1, self.odometerAngles.first().unwrap().1);
            if angleDiff == 0. {
                f64::MAX
            } else {
                dist / angleDiff.abs()
            }
        } else {
            veh.get_curve_radius()
        }
    }

    /// C++: `void RailVehicleVariables::saveState(OutputDevice &, const MSCFModel &) const` [const, virtual]
    /// Saves the vehicle variables
    // void
    // MSCFModel_Rail::RailVehicleVariables::saveState(OutputDevice& out, const MSCFModel& /*cfm*/) const {
    //     out.openTag(SUMO_TAG_CFM_VARIABLES);
    //     out.writeAttr(SUMO_ATTR_ID, "Rail");
    //     std::ostringstream internals;
    //     internals << odometerAngles.size() << " ";
    //     for (auto item : odometerAngles) {
    //         internals << item.first << " " << item.second << " ";
    //     }
    //     out.writeAttr(SUMO_ATTR_STATE, internals.str());
    //     out.closeTag();
    // }
    #[allow(unused_variables)]
    pub fn save_state(&self, out: &mut OutputDevice, cfm: &MSCFModel) {
        out.open_tag_id(SUMO_TAG_CFM_VARIABLES);
        out.write_attr(SUMO_ATTR_ID, "Rail");
        let mut internals = String::new();
        internals += &format!("{} ", self.odometerAngles.len());
        for item in self.odometerAngles.iter() {
            internals += &format!("{} {} ", f64_to_string_prec(item.0, 6), f64_to_string_prec(item.1, 6));
        }
        out.write_attr(SUMO_ATTR_STATE, internals.as_str());
        out.close_tag("");
        let _ = cfm;
    }

    /// C++: `void RailVehicleVariables::loadState(const SUMOSAXAttributes &)` [virtual]
    /// Loads the state of the vehicle variables from the given description
    // void
    // MSCFModel_Rail::RailVehicleVariables::loadState(const SUMOSAXAttributes& attrs) {
    //     bool ok = true;
    //     const std::string cfmID = attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
    //     if (cfmID != "Rail") {
    //         throw ProcessError(TLF("incompatible carFollowModel '%' when loading state for Rail", cfmID));
    //     }
    //     std::istringstream bis(attrs.getString(SUMO_ATTR_STATE));
    //     int odometerAnglesSize;
    //     bis >> odometerAnglesSize;
    //     for (int i = 0; i < odometerAnglesSize; i++) {
    //         double o;
    //         double a;
    //         bis >> o;
    //         bis >> a;
    //         odometerAngles.push_back(std::make_pair(o, a));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn load_state(&mut self, attrs: &SUMOSAXAttributes) {
        let mut ok = true;
        let cfmID: String = attrs.get(SUMO_ATTR_ID, "", &mut ok, true);
        if cfmID != "Rail" {
            panic!("{}", utils::common::util_exceptions::ProcessError::new(format!("incompatible carFollowModel '{}' when loading state for Rail", cfmID)));
        }
        let state = attrs.get_string(SUMO_ATTR_STATE);
        let mut bis = state.split_whitespace();
        let odometerAnglesSize: i32 = bis.next().and_then(|s| s.parse().ok()).unwrap_or(0);
        for _ in 0..odometerAnglesSize {
            let o: f64 = bis.next().and_then(|s| s.parse().ok()).unwrap_or(0.);
            let a: f64 = bis.next().and_then(|s| s.parse().ok()).unwrap_or(0.);
            self.odometerAngles.push((o, a));
        }
    }

}

/// C++ class `MSCFModel_Rail::TrainParams` (microsim/cfmodels/MSCFModel_Rail.h)
#[derive(Default)]
pub struct MSCFModel_Rail_TrainParams {
    pub weight: f64,
    pub mf: f64,
    pub length: f64,
    pub decl: f64,
    pub vmax: f64,
    pub recovery: f64,
    pub traction: LinearApproxMap,
    pub resistance: LinearApproxMap,
    pub maxPower: f64,
    pub maxTraction: f64,
    pub resCoef_constant: f64,
    pub resCoef_linear: f64,
    pub resCoef_quadratic: f64,
    pub curveResistance: f64,
    pub curveIntegration: f64,
    pub roeckl_sharp_radius: f64,
    pub roeckl_numerator: f64,
    pub roeckl_numerator_sharp: f64,
    pub roeckl_offset: f64,
    pub roeckl_offset_sharp: f64,
}

impl MSCFModel_Rail_TrainParams {
    /// C++: `double TrainParams::getResistance(double) const` [const]
    // double
    // MSCFModel_Rail::TrainParams::getResistance(double speed) const {
    //     if (resCoef_constant != INVALID_DOUBLE) {
    //         return (resCoef_quadratic * speed * speed + resCoef_linear * speed + resCoef_constant); // kN
    //     } else {
    //         return LinearApproxHelpers::getInterpolatedValue(resistance, speed); // kN
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_resistance(&self, speed: f64) -> f64 {
        if self.resCoef_constant != INVALID_DOUBLE {
            (self.resCoef_quadratic * speed * speed + self.resCoef_linear * speed + self.resCoef_constant) // kN
        } else {
            get_interpolated_value(&self.resistance, speed) // kN
        }
    }

    /// C++: `double TrainParams::getTraction(double) const` [const]
    // double
    // MSCFModel_Rail::TrainParams::getTraction(double speed) const {
    //     if (maxPower != INVALID_DOUBLE) {
    //         return MIN2(maxPower / speed, maxTraction); // kN
    //     } else {
    //         return LinearApproxHelpers::getInterpolatedValue(traction, speed); // kN
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_traction(&self, speed: f64) -> f64 {
        if self.maxPower != INVALID_DOUBLE {
            min2(self.maxPower / speed, self.maxTraction) // kN
        } else {
            get_interpolated_value(&self.traction, speed) // kN
        }
    }

}

