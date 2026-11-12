//! Port of microsim/cfmodels/MSCFModel_Rail.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `MSCFModel_Rail` (microsim/cfmodels/MSCFModel_Rail.h)
#[repr(C)]
pub struct MSCFModel_Rail {
    /// C++ base class `MSCFModel`
    pub base: MSCFModel,
    pub myTrainParams: MSCFModel_Rail_TrainParams,
}

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
        todo!("port MSCFModel_Rail::MSCFModel_Rail")
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
        todo!("port MSCFModel_Rail::followSpeed")
    }

    /// C++: `int MSCFModel_Rail::getModelID() const` [const, virtual]
    // int
    // MSCFModel_Rail::getModelID() const {
    //     return SUMO_TAG_CF_RAIL;
    // }
    #[allow(unused_variables)]
    pub fn get_model_id(&self) -> i32 {
        todo!("port MSCFModel_Rail::getModelID")
    }

    /// C++: `MSCFModel * MSCFModel_Rail::duplicate(const MSVehicleType *) const` [const, virtual]
    // MSCFModel*
    // MSCFModel_Rail::duplicate(const MSVehicleType* vtype) const {
    //     return new MSCFModel_Rail(vtype);
    // }
    #[allow(unused_variables)]
    pub fn duplicate(&self, vtype: Ptr<MSVehicleType>) -> Ptr<MSCFModel> {
        todo!("port MSCFModel_Rail::duplicate")
    }

    /// C++: `void MSCFModel_Rail::~MSCFModel_Rail() noexcept` [virtual]
    // MSCFModel_Rail::~MSCFModel_Rail() { }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MSCFModel_Rail::~MSCFModel_Rail")
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
        todo!("port MSCFModel_Rail::createVehicleVariables")
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
        todo!("port MSCFModel_Rail::maxNextSpeed")
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
        todo!("port MSCFModel_Rail::minNextSpeed")
    }

    /// C++: `double MSCFModel_Rail::minNextSpeedEmergency(double, const MSVehicle *const) const` [const, virtual]
    // double
    // MSCFModel_Rail::minNextSpeedEmergency(double speed, const MSVehicle* const veh) const {
    //     return minNextSpeed(speed, veh);
    // }
    #[allow(unused_variables)]
    pub fn min_next_speed_emergency(&self, speed: f64, veh: Ptr<MSVehicle>) -> f64 {
        todo!("port MSCFModel_Rail::minNextSpeedEmergency")
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
        todo!("port MSCFModel_Rail::getSpeedAfterMaxDecel")
    }

    /// C++: `double MSCFModel_Rail::finalizeSpeed(MSVehicle *const, double) const` [const, virtual]
    // double MSCFModel_Rail::finalizeSpeed(MSVehicle* const veh, double vPos) const {
    //     return MSCFModel::finalizeSpeed(veh, vPos);
    // }
    #[allow(unused_variables)]
    pub fn finalize_speed(&self, veh: Ptr<MSVehicle>, vPos: f64) -> f64 {
        todo!("port MSCFModel_Rail::finalizeSpeed")
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
        todo!("port MSCFModel_Rail::freeSpeed")
    }

    /// C++: `bool MSCFModel_Rail::startupDelayStopped() const` [const, virtual]
    // bool startupDelayStopped() const {
    //         // starup delay in trains is dominated by inertia + brake delay and thus applies to any kind of stopping
    //         return true;
    //     }
    #[allow(unused_variables)]
    pub fn startup_delay_stopped(&self) -> bool {
        todo!("port MSCFModel_Rail::startupDelayStopped")
    }

    /// C++: `double MSCFModel_Rail::stopSpeed(const MSVehicle *const, const double, double, double, const CalcReason) const` [const, virtual]
    // double MSCFModel_Rail::stopSpeed(const MSVehicle* const veh, const double speed, double gap, double decel, const CalcReason /*usage*/) const {
    //     return MIN2(maximumSafeStopSpeed(gap, decel, speed, false, TS, false), maxNextSpeed(speed, veh));
    // }
    #[allow(unused_variables)]
    pub fn stop_speed(&self, veh: Ptr<MSVehicle>, speed: f64, gap: f64, decel: f64, usage: MSCFModel_CalcReason) -> f64 {
        todo!("port MSCFModel_Rail::stopSpeed")
    }

    /// C++: `double MSCFModel_Rail::getRotWeight(const MSVehicle *const) const` [const, protected]
    // double
    // MSCFModel_Rail::getRotWeight(const MSVehicle* const veh) const {
    //     return getWeight(veh) * myTrainParams.mf;
    // }
    #[allow(unused_variables)]
    pub fn get_rot_weight(&self, veh: Ptr<MSVehicle>) -> f64 {
        todo!("port MSCFModel_Rail::getRotWeight")
    }

    /// C++: `double MSCFModel_Rail::getWeight(const MSVehicle *const) const` [const, protected]
    // double
    // MSCFModel_Rail::getWeight(const MSVehicle* const veh) const {
    //     // kg to tons
    //     return veh->getVehicleType().getMass() / 1000;
    // }
    #[allow(unused_variables)]
    pub fn get_weight(&self, veh: Ptr<MSVehicle>) -> f64 {
        todo!("port MSCFModel_Rail::getWeight")
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
        todo!("port MSCFModel_Rail::getCurveResistance")
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
    pub fn init_ngt400_traction(&self) -> Unmapped /* LinearApproxHelpers::LinearApproxMap */ {
        todo!("port MSCFModel_Rail::initNGT400Traction")
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
    pub fn init_ngt400_resistance(&self) -> Unmapped /* LinearApproxHelpers::LinearApproxMap */ {
        todo!("port MSCFModel_Rail::initNGT400Resistance")
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
        todo!("port MSCFModel_Rail::initNGT400Params")
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
    pub fn init_ngt400_16_traction(&self) -> Unmapped /* LinearApproxHelpers::LinearApproxMap */ {
        todo!("port MSCFModel_Rail::initNGT400_16Traction")
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
    pub fn init_ngt400_16_resistance(&self) -> Unmapped /* LinearApproxHelpers::LinearApproxMap */ {
        todo!("port MSCFModel_Rail::initNGT400_16Resistance")
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
        todo!("port MSCFModel_Rail::initNGT400_16Params")
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
    pub fn init_ice1_traction(&self) -> Unmapped /* LinearApproxHelpers::LinearApproxMap */ {
        todo!("port MSCFModel_Rail::initICE1Traction")
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
    pub fn init_ice1_resistance(&self) -> Unmapped /* LinearApproxHelpers::LinearApproxMap */ {
        todo!("port MSCFModel_Rail::initICE1Resistance")
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
        todo!("port MSCFModel_Rail::initICE1Params")
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
        todo!("port MSCFModel_Rail::initICE3Params")
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
    pub fn init_ice3_traction(&self) -> Unmapped /* LinearApproxHelpers::LinearApproxMap */ {
        todo!("port MSCFModel_Rail::initICE3Traction")
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
    pub fn init_ice3_resistance(&self) -> Unmapped /* LinearApproxHelpers::LinearApproxMap */ {
        todo!("port MSCFModel_Rail::initICE3Resistance")
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
        todo!("port MSCFModel_Rail::initREDosto7Params")
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
    pub fn init_re_dosto7_traction(&self) -> Unmapped /* LinearApproxHelpers::LinearApproxMap */ {
        todo!("port MSCFModel_Rail::initREDosto7Traction")
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
    pub fn init_re_dosto7_resistance(&self) -> Unmapped /* LinearApproxHelpers::LinearApproxMap */ {
        todo!("port MSCFModel_Rail::initREDosto7Resistance")
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
        todo!("port MSCFModel_Rail::initRB628Params")
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
    pub fn init_rb628_traction(&self) -> Unmapped /* LinearApproxHelpers::LinearApproxMap */ {
        todo!("port MSCFModel_Rail::initRB628Traction")
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
    pub fn init_rb628_resistance(&self) -> Unmapped /* LinearApproxHelpers::LinearApproxMap */ {
        todo!("port MSCFModel_Rail::initRB628Resistance")
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
        todo!("port MSCFModel_Rail::initFreightParams")
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
    pub fn init_freight_traction(&self) -> Unmapped /* LinearApproxHelpers::LinearApproxMap */ {
        todo!("port MSCFModel_Rail::initFreightTraction")
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
    pub fn init_freight_resistance(&self) -> Unmapped /* LinearApproxHelpers::LinearApproxMap */ {
        todo!("port MSCFModel_Rail::initFreightResistance")
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
    pub fn init_rb425_traction(&self) -> Unmapped /* LinearApproxHelpers::LinearApproxMap */ {
        todo!("port MSCFModel_Rail::initRB425Traction")
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
    pub fn init_rb425_resistance(&self) -> Unmapped /* LinearApproxHelpers::LinearApproxMap */ {
        todo!("port MSCFModel_Rail::initRB425Resistance")
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
        todo!("port MSCFModel_Rail::initRB425Params")
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
    pub fn init_mireo_plus_b2_t_traction(&self) -> Unmapped /* LinearApproxHelpers::LinearApproxMap */ {
        todo!("port MSCFModel_Rail::initMireoPlusB2TTraction")
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
    pub fn init_mireo_plus_b2_t_resistance(&self) -> Unmapped /* LinearApproxHelpers::LinearApproxMap */ {
        todo!("port MSCFModel_Rail::initMireoPlusB2TResistance")
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
        todo!("port MSCFModel_Rail::initMireoPlusB2TParams")
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
    pub fn init_mireo_plus_h2_t_traction(&self) -> Unmapped /* LinearApproxHelpers::LinearApproxMap */ {
        todo!("port MSCFModel_Rail::initMireoPlusH2TTraction")
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
    pub fn init_mireo_plus_h2_t_resistance(&self) -> Unmapped /* LinearApproxHelpers::LinearApproxMap */ {
        todo!("port MSCFModel_Rail::initMireoPlusH2TResistance")
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
        todo!("port MSCFModel_Rail::initMireoPlusH2TParams")
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
        todo!("port MSCFModel_Rail::initCustomParams")
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
        todo!("port RailVehicleVariables::getIntegratedRadius")
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
        todo!("port RailVehicleVariables::saveState")
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
        todo!("port RailVehicleVariables::loadState")
    }

}

/// C++ class `MSCFModel_Rail::TrainParams` (microsim/cfmodels/MSCFModel_Rail.h)
pub struct MSCFModel_Rail_TrainParams {
    pub weight: f64,
    pub mf: f64,
    pub length: f64,
    pub decl: f64,
    pub vmax: f64,
    pub recovery: f64,
    pub traction: Unmapped /* LinearApproxHelpers::LinearApproxMap */,
    pub resistance: Unmapped /* LinearApproxHelpers::LinearApproxMap */,
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
        todo!("port TrainParams::getResistance")
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
        todo!("port TrainParams::getTraction")
    }

}

