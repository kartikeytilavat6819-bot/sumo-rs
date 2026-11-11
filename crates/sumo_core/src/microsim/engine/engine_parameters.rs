//! Port of microsim/engine/EngineParameters.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::to_string::format_g;
use utils::geom::geom_helper::M_PI;

// C++ #defines from EngineParameters.h
const GRAVITY_MPS2: f64 = 9.81;
const HP_TO_W: f64 = 745.699872;

// gearRatios is a C `new double[nGears]` array owned by the struct: allocate and free it with these helpers
fn alloc_gear_ratios(n: i32) -> Ptr<f64> {
    Ptr::from_raw(Box::into_raw(vec![0.0f64; n as usize].into_boxed_slice()) as *mut f64)
}
// shortcut: the array length is taken from nGears, so nGears must not be changed by hand after construction; upgrade to Vec<f64> when callers are ported
fn free_gear_ratios(p: Ptr<f64>, n: i32) {
    if !p.is_null() {
        unsafe { drop(Box::from_raw(std::ptr::slice_from_raw_parts_mut(p.as_raw(), n as usize))) }
    }
}
fn gear_ratio_at(p: Ptr<f64>, i: usize) -> f64 {
    unsafe { *p.as_raw().add(i) }
}
fn set_gear_ratio(p: Ptr<f64>, i: usize, v: f64) {
    unsafe { *p.as_raw().add(i) = v }
}

/// C++ class `EngineParameters` (microsim/engine/EngineParameters.h)
pub struct EngineParameters {
    pub id: String,
    pub nGears: i32,
    pub gearRatios: Ptr<f64>,
    pub differentialRatio: f64,
    pub wheelDiameter_m: f64,
    pub mass_kg: f64,
    pub cAir: f64,
    pub a_m2: f64,
    pub rho_kgpm3: f64,
    pub __airFrictionCoefficient: f64,
    pub cr1: f64,
    pub cr2: f64,
    pub __cr1: f64,
    pub __cr2: f64,
    pub slope: f64,
    pub __gravity: f64,
    pub tiresFrictionCoefficient: f64,
    pub __maxNoSlipAcceleration: f64,
    pub engineMapping: EngineParameters_PolynomialEngineModelRpmToHp,
    pub shiftingRule: EngineParameters_GearShiftingRules,
    pub engineEfficiency: f64,
    pub massFactor: f64,
    pub cylinders: i32,
    pub dt: f64,
    pub minRpm: f64,
    pub maxRpm: f64,
    pub brakesTau_s: f64,
    pub tauEx_s: f64,
    pub tauBurn_s: f64,
    pub fixedTauBurn: bool,
    pub __rpmToSpeedCoefficient: f64,
    pub __speedToRpmCoefficient: f64,
    pub __speedToThrustCoefficient: f64,
    pub __maxAccelerationCoefficient: f64,
    pub __engineTau1: f64,
    pub __engineTau2: f64,
    pub __engineTauDe_s: f64,
    pub __brakesAlpha: f64,
    pub __brakesOneMinusAlpha: f64,
    pub minSpeed: f64,
}

impl EngineParameters {
    /// C++: `void EngineParameters::computeCoefficients()`
    /// Computes the precomputed coefficients used to speedup the model
    // void EngineParameters::computeCoefficients() {
    //     __airFrictionCoefficient = 0.5 * cAir * a_m2 * rho_kgpm3;
    //     __cr1 = mass_kg * massFactor * GRAVITY_MPS2 * cr1;
    //     __cr2 = mass_kg * massFactor * GRAVITY_MPS2 * cr2;
    //     __gravity = mass_kg * massFactor * GRAVITY_MPS2 * sin(slope / 180 * M_PI);
    //     __maxNoSlipAcceleration = tiresFrictionCoefficient * GRAVITY_MPS2 * cos(slope / 180 * M_PI);
    //     __rpmToSpeedCoefficient = wheelDiameter_m * M_PI / (differentialRatio * 60);
    //     __speedToRpmCoefficient = differentialRatio * 60 / (wheelDiameter_m * M_PI);
    //     __speedToThrustCoefficient = engineEfficiency * HP_TO_W;
    //     __maxAccelerationCoefficient = mass_kg * massFactor;
    //     __brakesAlpha = dt / (brakesTau_s + dt);
    //     __brakesOneMinusAlpha = 1 - __brakesAlpha;
    //     __engineTau1 = (420.0 * cylinders - 240.0) / (2.0 * cylinders);
    //     __engineTau2 = (120.0 * cylinders - 120.0) / cylinders;
    //     __engineTauDe_s = tauBurn_s + tauEx_s;
    // }
    #[allow(unused_variables)]
    pub fn compute_coefficients(&mut self) {
        self.__airFrictionCoefficient = 0.5 * self.cAir * self.a_m2 * self.rho_kgpm3;
        self.__cr1 = self.mass_kg * self.massFactor * GRAVITY_MPS2 * self.cr1;
        self.__cr2 = self.mass_kg * self.massFactor * GRAVITY_MPS2 * self.cr2;
        self.__gravity = self.mass_kg * self.massFactor * GRAVITY_MPS2 * (self.slope / 180.0 * M_PI).sin();
        self.__maxNoSlipAcceleration = self.tiresFrictionCoefficient * GRAVITY_MPS2 * (self.slope / 180.0 * M_PI).cos();
        self.__rpmToSpeedCoefficient = self.wheelDiameter_m * M_PI / (self.differentialRatio * 60.0);
        self.__speedToRpmCoefficient = self.differentialRatio * 60.0 / (self.wheelDiameter_m * M_PI);
        self.__speedToThrustCoefficient = self.engineEfficiency * HP_TO_W;
        self.__maxAccelerationCoefficient = self.mass_kg * self.massFactor;
        self.__brakesAlpha = self.dt / (self.brakesTau_s + self.dt);
        self.__brakesOneMinusAlpha = 1.0 - self.__brakesAlpha;
        self.__engineTau1 = (420.0 * self.cylinders as f64 - 240.0) / (2.0 * self.cylinders as f64);
        self.__engineTau2 = (120.0 * self.cylinders as f64 - 120.0) / self.cylinders as f64;
        self.__engineTauDe_s = self.tauBurn_s + self.tauEx_s;
    }

    /// C++: `void EngineParameters::dumpParameters(std::ostream &)`
    /// Writes the parameter to an output stream, for debugging purposes
    // void EngineParameters::dumpParameters(std::ostream& out) {
    //     out << "ID: " << id.c_str() << std::endl;
    //
    //     out << "Gearbox:\n";
    //     out << "\tGears number: " << (int)nGears << std::endl;
    //     for (int i = 0; i < nGears; i++) {
    //         out << std::setprecision(4) << "\tRatio of gear " << (i + 1) << ": " << gearRatios[i] << std::endl;
    //     }
    //     out << std::setprecision(4) << "\tFinal drive ratio: " << differentialRatio << std::endl;
    //
    //     out << "Wheels:\n";
    //     out << std::setprecision(3) << "\tDiameter: " << wheelDiameter_m << " m\n";
    //     out << std::setprecision(3) << "\tFriction coefficient: " << tiresFrictionCoefficient << std::endl;
    //     out << std::setprecision(10) << "\tcr1: " << cr1 << std::endl;
    //     out << std::setprecision(10) << "\tcr2: " << cr2 << std::endl;
    //
    //     out << "Mass:\n";
    //     out << std::setprecision(2) << "\tMass: " << mass_kg << " kg\n";
    //     out << std::setprecision(4) << "\tMass factor: " << massFactor << std::endl;
    //
    //     out << "Air drag:\n";
    //     out << std::setprecision(4) << "\tDrag coefficient: " << cAir << std::endl;
    //     out << std::setprecision(3) << "\tMax section: " << a_m2 << " m^2\n";
    //
    //     out << "Engine:\n";
    //     out << "\tEfficiency: " << engineEfficiency << std::endl;
    //     out << "\tCylinders: " << cylinders << std::endl;
    //     out << "\tMinimum rpm: " << minRpm << std::endl;
    //     out << "\tMaximum rpm: " << maxRpm << std::endl;
    //     out << "\tMapping (rpm to hp) degree: " << engineMapping.degree << std::endl;
    //     for (int i = 0; i < engineMapping.degree; i++) {
    //         out << "\t\tMapping coefficient x" << i << ": " << engineMapping.x[i] << std::endl;
    //     }
    //     out << "\tShifting rpm: " << shiftingRule.rpm << std::endl;
    //     out << "\tShifting delta: " << shiftingRule.deltaRpm << std::endl;
    //
    //     out << "Brakes:\n";
    //     out << "\tTime constant (s): " << brakesTau_s << std::endl;
    //
    //     out << "Vehicle unrelated parameters:\n";
    //     out << std::setprecision(4) << "\tAir density: " << rho_kgpm3 << " kg/m^3\n";
    //     out << "\tRoad slope: " << slope << " degrees\n";
    //     out << std::setprecision(3) << "\tSimulation sampling time: " << dt << " s\n";
    // }
    #[allow(unused_variables)]
    pub fn dump_parameters(&mut self, out: &mut dyn std::io::Write) {
        // prec mirrors the C++ stream state set by each std::setprecision (default 6)
        let mut prec: usize;
        let mut s = String::new();
        s += &format!("ID: {}\n", self.id);
        s += "Gearbox:\n";
        s += &format!("\tGears number: {}\n", self.nGears);
        prec = 4;
        for i in 0..self.nGears {
            s += &format!("\tRatio of gear {}: {}\n", i + 1, format_g(gear_ratio_at(self.gearRatios, i as usize), prec));
        }
        s += &format!("\tFinal drive ratio: {}\n", format_g(self.differentialRatio, prec));
        s += "Wheels:\n";
        prec = 3;
        s += &format!("\tDiameter: {} m\n", format_g(self.wheelDiameter_m, prec));
        s += &format!("\tFriction coefficient: {}\n", format_g(self.tiresFrictionCoefficient, prec));
        prec = 10;
        s += &format!("\tcr1: {}\n", format_g(self.cr1, prec));
        s += &format!("\tcr2: {}\n", format_g(self.cr2, prec));
        s += "Mass:\n";
        prec = 2;
        s += &format!("\tMass: {} kg\n", format_g(self.mass_kg, prec));
        prec = 4;
        s += &format!("\tMass factor: {}\n", format_g(self.massFactor, prec));
        s += "Air drag:\n";
        s += &format!("\tDrag coefficient: {}\n", format_g(self.cAir, prec));
        prec = 3;
        s += &format!("\tMax section: {} m^2\n", format_g(self.a_m2, prec));
        s += "Engine:\n";
        s += &format!("\tEfficiency: {}\n", format_g(self.engineEfficiency, prec));
        s += &format!("\tCylinders: {}\n", self.cylinders);
        s += &format!("\tMinimum rpm: {}\n", format_g(self.minRpm, prec));
        s += &format!("\tMaximum rpm: {}\n", format_g(self.maxRpm, prec));
        s += &format!("\tMapping (rpm to hp) degree: {}\n", self.engineMapping.degree);
        for i in 0..self.engineMapping.degree {
            s += &format!("\t\tMapping coefficient x{}: {}\n", i, format_g(self.engineMapping.x[i as usize], prec));
        }
        s += &format!("\tShifting rpm: {}\n", format_g(self.shiftingRule.rpm, prec));
        s += &format!("\tShifting delta: {}\n", format_g(self.shiftingRule.deltaRpm, prec));
        s += "Brakes:\n";
        s += &format!("\tTime constant (s): {}\n", format_g(self.brakesTau_s, prec));
        s += "Vehicle unrelated parameters:\n";
        prec = 4;
        s += &format!("\tAir density: {} kg/m^3\n", format_g(self.rho_kgpm3, prec));
        s += &format!("\tRoad slope: {} degrees\n", format_g(self.slope, prec));
        prec = 3;
        s += &format!("\tSimulation sampling time: {} s\n", format_g(self.dt, prec));
        // C++ writes to a std::ostream; write errors are ignored there too
        let _ = out.write_all(s.as_bytes());
    }

    /// C++: `void EngineParameters::EngineParameters()`
    // EngineParameters::EngineParameters() : nGears(5), differentialRatio(3.714), wheelDiameter_m(0.94),
    //     mass_kg(1300), cAir(0.3), a_m2(2.7), rho_kgpm3(1.2), cr1(0.0136), cr2(5.18e-7),
    //     slope(0.), tiresFrictionCoefficient(0.7), engineEfficiency(0.8),
    //     massFactor(1.089), cylinders(4), dt(0.01), minRpm(1000), maxRpm(7000),
    //     brakesTau_s(0.2), tauEx_s(0.1), tauBurn_s(-1), fixedTauBurn(false) {
    //     id = "";
    //     //set default gear ratios
    //     gearRatios = new double[nGears];
    //     gearRatios[0] = 3.909;//4.5;
    //     gearRatios[1] = 2.238;//3.5;//2.6;
    //     gearRatios[2] = 1.520;//2.5;//1.5;
    //     gearRatios[3] = 1.156;//1.5;//1.0;
    //     gearRatios[4] = 0.971;//1.0;//0.8;
    //     //default engine mapping
    //     engineMapping.degree = 1;
    //     engineMapping.x[0] = -7.50084;
    //     engineMapping.x[1] = 0.02045;
    //     //default shifting rule
    //     shiftingRule.rpm = 6000;
    //     shiftingRule.deltaRpm = 100;
    //     //initialize precomputed coefficients
    //     computeCoefficients();
    //
    // }
    #[allow(unused_variables)]
    pub fn new() -> Self {
        // C++ leaves minSpeed uninitialized; it is zero here
        let mut this = EngineParameters {
            id: String::new(),
            nGears: 5,
            gearRatios: alloc_gear_ratios(5),
            differentialRatio: 3.714,
            wheelDiameter_m: 0.94,
            mass_kg: 1300.0,
            cAir: 0.3,
            a_m2: 2.7,
            rho_kgpm3: 1.2,
            __airFrictionCoefficient: 0.0,
            cr1: 0.0136,
            cr2: 5.18e-7,
            __cr1: 0.0,
            __cr2: 0.0,
            slope: 0.0,
            __gravity: 0.0,
            tiresFrictionCoefficient: 0.7,
            __maxNoSlipAcceleration: 0.0,
            engineMapping: EngineParameters_PolynomialEngineModelRpmToHp { degree: 1, x: [0.0; 9] },
            shiftingRule: EngineParameters_GearShiftingRules { rpm: 6000.0, deltaRpm: 100.0 },
            engineEfficiency: 0.8,
            massFactor: 1.089,
            cylinders: 4,
            dt: 0.01,
            minRpm: 1000.0,
            maxRpm: 7000.0,
            brakesTau_s: 0.2,
            tauEx_s: 0.1,
            tauBurn_s: -1.0,
            fixedTauBurn: false,
            __rpmToSpeedCoefficient: 0.0,
            __speedToRpmCoefficient: 0.0,
            __speedToThrustCoefficient: 0.0,
            __maxAccelerationCoefficient: 0.0,
            __engineTau1: 0.0,
            __engineTau2: 0.0,
            __engineTauDe_s: 0.0,
            __brakesAlpha: 0.0,
            __brakesOneMinusAlpha: 0.0,
            minSpeed: 0.0,
        };
        // set default gear ratios
        set_gear_ratio(this.gearRatios, 0, 3.909);
        set_gear_ratio(this.gearRatios, 1, 2.238);
        set_gear_ratio(this.gearRatios, 2, 1.520);
        set_gear_ratio(this.gearRatios, 3, 1.156);
        set_gear_ratio(this.gearRatios, 4, 0.971);
        // default engine mapping
        this.engineMapping.degree = 1;
        this.engineMapping.x[0] = -7.50084;
        this.engineMapping.x[1] = 0.02045;
        // default shifting rule
        this.shiftingRule.rpm = 6000.0;
        this.shiftingRule.deltaRpm = 100.0;
        // initialize precomputed coefficients
        this.compute_coefficients();
        this
    }

    /// C++: `void EngineParameters::EngineParameters(const EngineParameters &)`
    // EngineParameters::EngineParameters(const EngineParameters& other) :
    //     nGears(other.nGears), differentialRatio(other.differentialRatio), wheelDiameter_m(other.wheelDiameter_m),
    //     mass_kg(other.mass_kg), cAir(other.cAir), a_m2(other.a_m2), rho_kgpm3(other.rho_kgpm3), cr1(other.cr1), cr2(other.cr2),
    //     slope(other.slope), tiresFrictionCoefficient(other.tiresFrictionCoefficient), engineEfficiency(other.engineEfficiency),
    //     massFactor(other.massFactor), cylinders(other.cylinders), dt(other.dt), minRpm(other.minRpm), maxRpm(other.maxRpm),
    //     brakesTau_s(other.brakesTau_s), tauEx_s(other.tauEx_s), tauBurn_s(other.tauBurn_s), fixedTauBurn(other.fixedTauBurn) {
    //     id = other.id;
    //     gearRatios = new double[nGears];
    //     for (int i = 0; i < nGears; i++) {
    //         gearRatios[i] = other.gearRatios[i];
    //     }
    //     engineMapping.degree = other.engineMapping.degree;
    //     for (int i = 0; i < engineMapping.degree; i++) {
    //         engineMapping.x[i] = other.engineMapping.x[i];
    //     }
    //     shiftingRule.rpm = other.shiftingRule.rpm;
    //     shiftingRule.deltaRpm = other.shiftingRule.deltaRpm;
    //     computeCoefficients();
    // }
    #[allow(unused_variables)]
    pub fn new_2(other: &EngineParameters) -> Self {
        // copies the same fields as the C++ copy constructor, then computes the coefficients
        let mut this = Self::new();
        this.assign_op(other);
        this
    }

    /// C++: `EngineParameters & EngineParameters::operator=(const EngineParameters &)`
    // EngineParameters& EngineParameters::operator =(const EngineParameters& other) {
    //     id = other.id;
    //     nGears = other.nGears;
    //     differentialRatio = other.differentialRatio;
    //     wheelDiameter_m = other.wheelDiameter_m;
    //     mass_kg = other.mass_kg;
    //     cAir = other.cAir;
    //     a_m2 = other.a_m2;
    //     rho_kgpm3 = other.rho_kgpm3;
    //     cr1 = other.cr1;
    //     cr2 = other.cr2;
    //     slope = other.slope;
    //     tiresFrictionCoefficient = other.tiresFrictionCoefficient;
    //     engineEfficiency = other.engineEfficiency;
    //     massFactor  = other.massFactor;
    //     cylinders = other.cylinders;
    //     dt = other.dt;
    //     minRpm = other.minRpm;
    //     maxRpm = other.maxRpm;
    //     delete [] gearRatios;
    //     gearRatios = new double[nGears];
    //     for (int i = 0; i < nGears; i++) {
    //         gearRatios[i] = other.gearRatios[i];
    //     }
    //     engineMapping.degree = other.engineMapping.degree;
    //     for (int i = 0; i < engineMapping.degree; i++) {
    //         engineMapping.x[i] = other.engineMapping.x[i];
    //     }
    //     shiftingRule.rpm = other.shiftingRule.rpm;
    //     shiftingRule.deltaRpm = other.shiftingRule.deltaRpm;
    //     brakesTau_s = other.brakesTau_s;
    //     tauBurn_s = other.tauBurn_s;
    //     tauEx_s = other.tauEx_s;
    //     fixedTauBurn = other.fixedTauBurn;
    //     computeCoefficients();
    //     return *this;
    // }
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, other: &EngineParameters) -> &mut EngineParameters {
        let old_nGears = self.nGears;
        self.id = other.id.clone();
        self.nGears = other.nGears;
        self.differentialRatio = other.differentialRatio;
        self.wheelDiameter_m = other.wheelDiameter_m;
        self.mass_kg = other.mass_kg;
        self.cAir = other.cAir;
        self.a_m2 = other.a_m2;
        self.rho_kgpm3 = other.rho_kgpm3;
        self.cr1 = other.cr1;
        self.cr2 = other.cr2;
        self.slope = other.slope;
        self.tiresFrictionCoefficient = other.tiresFrictionCoefficient;
        self.engineEfficiency = other.engineEfficiency;
        self.massFactor = other.massFactor;
        self.cylinders = other.cylinders;
        self.dt = other.dt;
        self.minRpm = other.minRpm;
        self.maxRpm = other.maxRpm;
        free_gear_ratios(self.gearRatios, old_nGears);
        self.gearRatios = alloc_gear_ratios(self.nGears);
        for i in 0..self.nGears as usize {
            set_gear_ratio(self.gearRatios, i, gear_ratio_at(other.gearRatios, i));
        }
        self.engineMapping.degree = other.engineMapping.degree;
        for i in 0..self.engineMapping.degree as usize {
            self.engineMapping.x[i] = other.engineMapping.x[i];
        }
        self.shiftingRule.rpm = other.shiftingRule.rpm;
        self.shiftingRule.deltaRpm = other.shiftingRule.deltaRpm;
        self.brakesTau_s = other.brakesTau_s;
        self.tauBurn_s = other.tauBurn_s;
        self.tauEx_s = other.tauEx_s;
        self.fixedTauBurn = other.fixedTauBurn;
        self.compute_coefficients();
        self
    }

    /// C++: `void EngineParameters::~EngineParameters() noexcept` [virtual]
    // EngineParameters::~EngineParameters() {
    //     delete [] gearRatios;
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        free_gear_ratios(self.gearRatios, self.nGears);
    }

}

/// Defines the rules for gear shifting. The structure contains the rpm at which shifting should occur and the delta used for up and down shifting. So to shift up, the rpm should be at least rpm + delta, while to shift down the should be lower than rpm - delta. this is needed to avoid oscillations when running aroud rpm
/// C++ class `EngineParameters::GearShiftingRules` (microsim/engine/EngineParameters.h)
pub struct EngineParameters_GearShiftingRules {
    pub rpm: f64,
    pub deltaRpm: f64,
}

impl EngineParameters_GearShiftingRules {
}

/// Defines the polynomial coefficients of the engine model mapping engine speed to horsepower. The model is of type power(rpm) [hp] = x0 + x1*hp + x2*hp^2 + ...
/// C++ class `EngineParameters::PolynomialEngineModelRpmToHp` (microsim/engine/EngineParameters.h)
pub struct EngineParameters_PolynomialEngineModelRpmToHp {
    pub degree: i32,
    pub x: [f64; 9],
}

impl EngineParameters_PolynomialEngineModelRpmToHp {
}

