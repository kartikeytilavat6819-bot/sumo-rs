//! Port of utils/emissions/PollutantsInterface.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

use super::helpers_hbefa::HelpersHBEFA;
use super::helpers_hbefa3::HelpersHBEFA3;
use super::helpers_phe_mlight::HelpersPHEMlight;
use super::helpers_energy::HelpersEnergy;
use super::helpers_mmpevem::HelpersMMPEVEM;
use super::helpers_phe_mlight5::HelpersPHEMlight5;
use super::helpers_hbefa4::HelpersHBEFA4;

/// C++ `PollutantsInterface::ZERO_EMISSIONS`
pub const ZERO_EMISSIONS: i32 = 0;
/// C++ `PollutantsInterface::HEAVY_BIT` (1 << 15)
pub const HEAVY_BIT: i32 = 1 << 15;
/// C++ `PollutantsInterface::Helper::ZERO_SPEED_ACCURACY`
pub const ZERO_SPEED_ACCURACY: f64 = 0.5;

fn leak_ptr<T>(v: T) -> Ptr<T> {
    Ptr::from_mut(Box::leak(Box::new(v)))
}

/// The C++ process-wide static helper objects (`PollutantsInterface::my*Helper`), in `myHelpers[]` order.
/// shortcut: helper instances are per thread (thread_local), not process-wide statics, upgrade if helpers must be shared across threads.
#[derive(Clone, Copy)]
pub struct PollutantsInterface_HelperTable {
    zero: Ptr<PollutantsInterface_Helper>,
    hbefa2: Ptr<HelpersHBEFA>,
    hbefa3: Ptr<HelpersHBEFA3>,
    phemlight: Ptr<HelpersPHEMlight>,
    energy: Ptr<HelpersEnergy>,
    mmpevem: Ptr<HelpersMMPEVEM>,
    phemlight5: Ptr<HelpersPHEMlight5>,
    hbefa4: Ptr<HelpersHBEFA4>,
}

thread_local! {
    static HELPER_TABLE: PollutantsInterface_HelperTable = PollutantsInterface_HelperTable {
        zero: leak_ptr(PollutantsInterface_Helper::new("Zero".to_string(), ZERO_EMISSIONS, ZERO_EMISSIONS)),
        hbefa2: leak_ptr(HelpersHBEFA::new()),
        hbefa3: leak_ptr(HelpersHBEFA3::new()),
        phemlight: leak_ptr(HelpersPHEMlight::new()),
        energy: leak_ptr(HelpersEnergy::new()),
        mmpevem: leak_ptr(HelpersMMPEVEM::new()),
        phemlight5: leak_ptr(HelpersPHEMlight5::new()),
        hbefa4: leak_ptr(HelpersHBEFA4::new()),
    };
    static ALL_CLASSES_STR: std::cell::Cell<Option<Ptr<Vec<String>>>> = const { std::cell::Cell::new(None) };
}

fn helper_table() -> PollutantsInterface_HelperTable {
    HELPER_TABLE.with(|t| *t)
}

/// Runs `$body` with `$h` bound to the helper at index `$idx` (C++ `myHelpers[idx]`), keeping the concrete type for virtual overrides.
macro_rules! with_helper {
    ($idx:expr, |$h:ident| $body:expr) => {{
        let __table = helper_table();
        match ($idx) as usize {
            0 => { let $h = __table.zero.m(); $body }
            1 => { let $h = __table.hbefa2.m(); $body }
            2 => { let $h = __table.hbefa3.m(); $body }
            3 => { let $h = __table.phemlight.m(); $body }
            4 => { let $h = __table.energy.m(); $body }
            5 => { let $h = __table.mmpevem.m(); $body }
            6 => { let $h = __table.phemlight5.m(); $body }
            7 => { let $h = __table.hbefa4.m(); $body }
            _ => panic!("invalid emission class model index"),
        }
    }};
}

/// C++ enum `PollutantsInterface::EmissionType`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum PollutantsInterface_EmissionType {
    CO2 = 0,
    CO = 1,
    HC = 2,
    FUEL = 3,
    NO_X = 4,
    PM_X = 5,
    ELEC = 6,
}

/// Storage for collected values of all emission types
/// C++ class `PollutantsInterface::Emissions` (utils/emissions/PollutantsInterface.h)
pub struct PollutantsInterface_Emissions {
    /// emission types @{
    pub CO2: f64,
    pub CO: f64,
    pub HC: f64,
    pub fuel: f64,
    pub NOx: f64,
    pub PMx: f64,
    pub electricity: f64,
}

impl PollutantsInterface_Emissions {
    /// C++: `void Emissions::Emissions(double, double, double, double, double, double, double)`
    /// Constructor, intializes all members
    // PollutantsInterface::Emissions::Emissions(double co2, double co, double hc, double f, double nox, double pmx, double elec) :
    //     CO2(co2),
    //     CO(co),
    //     HC(hc),
    //     fuel(f),
    //     NOx(nox),
    //     PMx(pmx),
    //     electricity(elec) {
    // }
    #[allow(unused_variables)]
    pub fn new(co2: f64, co: f64, hc: f64, f: f64, nox: f64, pmx: f64, elec: f64) -> Self {
        PollutantsInterface_Emissions { CO2: co2, CO: co, HC: hc, fuel: f, NOx: nox, PMx: pmx, electricity: elec }
    }

    /// C++: `void Emissions::addScaled(const Emissions &, const double)`
    /// Add the values of the other struct to this one, scaling the values if needed
    // void PollutantsInterface::Emissions::addScaled(const Emissions& a, const double scale) {
    //     CO2 += scale * a.CO2;
    //     CO += scale * a.CO;
    //     HC += scale * a.HC;
    //     fuel += scale * a.fuel;
    //     NOx += scale * a.NOx;
    //     PMx += scale * a.PMx;
    //     electricity += scale * a.electricity;
    // }
    #[allow(unused_variables)]
    pub fn add_scaled(&mut self, a: &PollutantsInterface_Emissions, scale: f64) {
        self.CO2 += scale * a.CO2;
        self.CO += scale * a.CO;
        self.HC += scale * a.HC;
        self.fuel += scale * a.fuel;
        self.NOx += scale * a.NOx;
        self.PMx += scale * a.PMx;
        self.electricity += scale * a.electricity;
    }

}

/// zero emission model, used as superclass for the other model helpers
/// C++ class `PollutantsInterface::Helper` (utils/emissions/PollutantsInterface.h)
pub struct PollutantsInterface_Helper {
    /// the name of the model
    pub myName: String,
    /// the starting index for classes of this model
    pub myBaseIndex: i32,
    /// return fuel consumption in l instead of mg
    pub myVolumetricFuel: bool,
    /// Mapping between emission class names and integer representations
    pub myEmissionClassStrings: StringBijection<SUMOEmissionClass>,
}

// C++ static members of Helper (port as module-level statics, see AGENTS.md):
//   static const double Helper::ZERO_SPEED_ACCURACY;

impl PollutantsInterface_Helper {
    /// C++: `void Helper::Helper(std::string, const int, const int)`
    /// Constructor, intializes the name
    // PollutantsInterface::Helper::Helper(std::string name, const int baseIndex, const int defaultClass) :
    //     myName(name),
    //     myBaseIndex(baseIndex) {
    //     if (defaultClass != -1) {
    //         myEmissionClassStrings.insert("default", defaultClass);
    //         myEmissionClassStrings.addAlias("unknown", defaultClass);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn new(name: String, baseIndex: i32, defaultClass: i32) -> Self {
        let mut h = Self {
            myName: name,
            myBaseIndex: baseIndex,
            myVolumetricFuel: false,
            myEmissionClassStrings: StringBijection::default(),
        };
        if defaultClass != -1 {
            h.myEmissionClassStrings.insert("default", defaultClass, true).unwrap_or_else(|e| panic!("{e}"));
            h.myEmissionClassStrings.add_alias("unknown", defaultClass);
        }
        h
    }

    /// C++: `void Helper::~Helper() noexcept` [virtual]
    /// empty destructor
    // virtual ~Helper() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // empty destructor
    }

    /// C++: `const std::string & Helper::getName() const` [const]
    /// Returns the name of the model
    // const
    // std::string& PollutantsInterface::Helper::getName() const {
    //     return myName;
    // }
    #[allow(unused_variables)]
    pub fn get_name(&self) -> &str {
        &self.myName
    }

    /// C++: `SUMOEmissionClass Helper::getClassByName(const std::string &, const SUMOVehicleClass)` [virtual]
    /// Returns the emission class associated with the given name, aliases are possible If this method is asked for the "unknown" class it should return the default (possibly depending on the given vehicle class). The class name is case insensitive.
    // SUMOEmissionClass
    // PollutantsInterface::Helper::getClassByName(const std::string& eClass, const SUMOVehicleClass vc) {
    //     UNUSED_PARAMETER(vc);
    //     myVolumetricFuel = OptionsCont::getOptions().exists("emissions.volumetric-fuel") && OptionsCont::getOptions().getBool("emissions.volumetric-fuel");
    //     if (myEmissionClassStrings.hasString(eClass)) {
    //         return myEmissionClassStrings.get(eClass);
    //     }
    //     return myEmissionClassStrings.get(StringUtils::to_lower_case(eClass));
    // }
    #[allow(unused_variables)]
    pub fn get_class_by_name(&mut self, eClass: &str, vc: SUMOVehicleClass) -> SUMOEmissionClass {
        self.myVolumetricFuel = utils::options::options_cont::exists("emissions.volumetric-fuel") && utils::options::options_cont::get_bool("emissions.volumetric-fuel");
        if self.myEmissionClassStrings.has_string(eClass) {
            return self.myEmissionClassStrings.get(eClass).unwrap_or_else(|e| panic!("{e}"));
        }
        self.myEmissionClassStrings.get(&utils::common::string_utils::to_lower_case(eClass)).unwrap_or_else(|e| panic!("{e}"))
    }

    /// C++: `const std::string Helper::getClassName(const SUMOEmissionClass) const` [const]
    /// Returns the complete name of the emission class including the model
    // const std::string
    // PollutantsInterface::Helper::getClassName(const SUMOEmissionClass c) const {
    //     return myName + "/" + myEmissionClassStrings.getString(c);
    // }
    #[allow(unused_variables)]
    pub fn get_class_name(&self, c: SUMOEmissionClass) -> String {
        format!("{}/{}", self.myName, self.myEmissionClassStrings.get_string(c).unwrap_or_else(|e| panic!("{e}")))
    }

    /// C++: `bool Helper::isSilent(const SUMOEmissionClass)` [virtual]
    /// Returns whether the class denotes a silent vehicle for interfacing with the noise model. By default the first class in each model is the silent class.
    // bool
    // PollutantsInterface::Helper::isSilent(const SUMOEmissionClass c) {
    //     return (c & (int)0xffffffff & ~HEAVY_BIT) == 0;
    // }
    #[allow(unused_variables)]
    pub fn is_silent(&mut self, c: SUMOEmissionClass) -> bool {
        (c & (-1i32) & !HEAVY_BIT) == 0
    }

    /// C++: `SUMOEmissionClass Helper::getClass(const SUMOEmissionClass, const std::string &, const std::string &, const std::string &, const double) const` [const, virtual]
    /// Returns the emission class described by the given parameters. The base is used to determine the model to use and as default return values. Default implementation returns always base.
    // SUMOEmissionClass
    // PollutantsInterface::Helper::getClass(const SUMOEmissionClass base, const std::string& vClass, const std::string& fuel, const std::string& eClass, const double weight) const {
    //     UNUSED_PARAMETER(vClass);
    //     UNUSED_PARAMETER(fuel);
    //     UNUSED_PARAMETER(eClass);
    //     UNUSED_PARAMETER(weight);
    //     return base;
    // }
    #[allow(unused_variables)]
    pub fn get_class(&self, base: SUMOEmissionClass, vClass: &str, fuel: &str, eClass: &str, weight: f64) -> SUMOEmissionClass {
        base
    }

    /// C++: `std::string Helper::getAmitranVehicleClass(const SUMOEmissionClass) const` [const, virtual]
    /// Returns the vehicle class described by this emission class as described in the Amitran interface (Passenger, ...) Default implementation returns always "Passenger".
    // std::string
    // PollutantsInterface::Helper::getAmitranVehicleClass(const SUMOEmissionClass c) const {
    //     UNUSED_PARAMETER(c);
    //     return "Passenger";
    // }
    #[allow(unused_variables)]
    pub fn get_amitran_vehicle_class(&self, c: SUMOEmissionClass) -> String {
        "Passenger".to_string()
    }

    /// C++: `std::string Helper::getFuel(const SUMOEmissionClass) const` [const, virtual]
    /// Returns the fuel type described by this emission class as described in the Amitran interface (Gasoline, Diesel, ...) Default implementation returns always "Gasoline".
    // std::string
    // PollutantsInterface::Helper::getFuel(const SUMOEmissionClass c) const {
    //     UNUSED_PARAMETER(c);
    //     return "Gasoline";
    // }
    #[allow(unused_variables)]
    pub fn get_fuel(&self, c: SUMOEmissionClass) -> String {
        "Gasoline".to_string()
    }

    /// C++: `int Helper::getEuroClass(const SUMOEmissionClass) const` [const, virtual]
    /// Returns the Euro emission class described by this emission class as described in the Amitran interface (0, ..., 6) Default implementation returns always 0.
    // int
    // PollutantsInterface::Helper::getEuroClass(const SUMOEmissionClass c) const {
    //     UNUSED_PARAMETER(c);
    //     return 0;
    // }
    #[allow(unused_variables)]
    pub fn get_euro_class(&self, c: SUMOEmissionClass) -> i32 {
        0
    }

    /// C++: `double Helper::getWeight(const SUMOEmissionClass) const` [const, virtual]
    /// Returns a reference weight in kg described by this emission class as described in the Amitran interface It might return -1, if the weight is not important to distinguish different emission classes. Default implementation returns always -1.
    // double
    // PollutantsInterface::Helper::getWeight(const SUMOEmissionClass c) const {
    //     UNUSED_PARAMETER(c);
    //     return -1.;
    // }
    #[allow(unused_variables)]
    pub fn get_weight(&self, c: SUMOEmissionClass) -> f64 {
        -1.
    }

    /// C++: `double Helper::compute(const SUMOEmissionClass, const EmissionType, const double, const double, const double, const EnergyParams *) const` [const, virtual]
    /// Returns the amount of the emitted pollutant given the vehicle type and state (in mg/s or ml/s for fuel)
    // double
    // PollutantsInterface::Helper::compute(const SUMOEmissionClass c, const EmissionType e, const double v, const double a, const double slope, const EnergyParams* param) const {
    //     UNUSED_PARAMETER(c);
    //     UNUSED_PARAMETER(e);
    //     UNUSED_PARAMETER(v);
    //     UNUSED_PARAMETER(a);
    //     UNUSED_PARAMETER(slope);
    //     UNUSED_PARAMETER(param);
    //     return 0.;
    // }
    #[allow(unused_variables)]
    pub fn compute(&self, c: SUMOEmissionClass, e: PollutantsInterface_EmissionType, v: f64, a: f64, slope: f64, param: Ptr<EnergyParams>) -> f64 {
        0.
    }

    /// C++: `double Helper::getModifiedAccel(const SUMOEmissionClass, const double, const double, const double, const EnergyParams *) const` [const, virtual]
    /// Returns the adapted acceleration value, useful for comparing with external PHEMlight references. Default implementation returns always the input accel.
    // double
    // PollutantsInterface::Helper::getModifiedAccel(const SUMOEmissionClass c, const double v, const double a, const double slope, const EnergyParams* param) const {
    //     UNUSED_PARAMETER(c);
    //     UNUSED_PARAMETER(v);
    //     UNUSED_PARAMETER(slope);
    //     UNUSED_PARAMETER(param);
    //     return a;
    // }
    #[allow(unused_variables)]
    pub fn get_modified_accel(&self, c: SUMOEmissionClass, v: f64, a: f64, slope: f64, param: Ptr<EnergyParams>) -> f64 {
        a
    }

    /// C++: `double Helper::getCoastingDecel(const SUMOEmissionClass, const double, const double, const double, const EnergyParams *) const` [const, virtual]
    /// Returns the maximum deceleration value (as a negative number), which can still be considered as non-braking. Default implementation returns always zero.
    // double
    // PollutantsInterface::Helper::getCoastingDecel(const SUMOEmissionClass c, const double v, const double a, const double slope, const EnergyParams* param) const {
    //     // the interpolation for small v is basically the same as in PHEMlightdllV5::CEP::GetDecelCoast
    //     if (v < PHEMlightdllV5::Constants::SPEED_DCEL_MIN) {
    //         return v / PHEMlightdllV5::Constants::SPEED_DCEL_MIN * getCoastingDecel(c, PHEMlightdllV5::Constants::SPEED_DCEL_MIN, a, slope, param);
    //     }
    //     if (param == nullptr) {
    //         param = EnergyParams::getDefault();
    //     }
    //     // the magic numbers below come from a linear interpolation with http://ts-sim-service-ba/svn/simo/trunk/projects/sumo/data/emissions/linear.py
    //     const double mass = param->getDouble(SUMO_ATTR_MASS);
    //     const double incl = param->getDouble(SUMO_ATTR_FRONTSURFACEAREA) / mass * -9.05337017 + -0.00017774;
    //     const double grad = slope == 0. ? 0. : PHEMlightdllV5::Constants::GRAVITY_CONST * sin(DEG2RAD(slope));
    //     return MIN2(0., incl * v + 0.00001066 * mass + -0.38347107 - 20.0 * incl - grad);
    // }
    #[allow(unused_variables)]
    pub fn get_coasting_decel(&self, c: SUMOEmissionClass, v: f64, a: f64, slope: f64, param: Ptr<EnergyParams>) -> f64 {
        // the interpolation for small v is basically the same as in PHEMlightdllV5::CEP::GetDecelCoast
        if v < foreign::phemlight::v5::constants::SPEED_DCEL_MIN {
            return v / foreign::phemlight::v5::constants::SPEED_DCEL_MIN * self.get_coasting_decel(c, foreign::phemlight::v5::constants::SPEED_DCEL_MIN, a, slope, param);
        }
        let param = if param.is_null() { EnergyParams::get_default() } else { param };
        // the magic numbers below come from a linear interpolation with http://ts-sim-service-ba/svn/simo/trunk/projects/sumo/data/emissions/linear.py
        let mass = param.m().get_double(SUMO_ATTR_MASS);
        let incl = param.m().get_double(SUMO_ATTR_FRONTSURFACEAREA) / mass * -9.05337017 + -0.00017774;
        let grad = if slope == 0. { 0. } else { foreign::phemlight::v5::constants::GRAVITY_CONST * utils::geom::geom_helper::deg2rad(slope).sin() };
        let r = incl * v + 0.00001066 * mass + -0.38347107 - 20.0 * incl - grad;
        if 0. < r { 0. } else { r } // MIN2(0., ...)
    }

    /// C++: `void Helper::addAllClassesInto(std::vector<SUMOEmissionClass> &) const` [const]
    /// Add all known emission classes of this model to the given container
    // void
    // PollutantsInterface::Helper::addAllClassesInto(std::vector<SUMOEmissionClass>& list) const {
    //     myEmissionClassStrings.addKeysInto(list);
    // }
    #[allow(unused_variables)]
    pub fn add_all_classes_into(&self, list: &mut Vec<SUMOEmissionClass>) {
        list.extend(self.myEmissionClassStrings.get_values());
    }

    /// C++: `bool Helper::includesClass(const SUMOEmissionClass) const` [const]
    // bool
    // PollutantsInterface::Helper::includesClass(const SUMOEmissionClass c) const {
    //     return (c >> 16) == (myBaseIndex >> 16);
    // }
    #[allow(unused_variables)]
    pub fn includes_class(&self, c: SUMOEmissionClass) -> bool {
        (c >> 16) == (self.myBaseIndex >> 16)
    }

    /// C++: `Helper & Helper::operator=(const Helper &)` [private]
    /// invalidate copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &PollutantsInterface_Helper) -> &mut PollutantsInterface_Helper {
        panic!("PollutantsInterface::Helper copy assignment is private in C++ and must not be used")
    }

}

/// Helper methods for PHEMlight-based emission computation
/// C++ class `PollutantsInterface` (utils/emissions/PollutantsInterface.h)
pub struct PollutantsInterface {
}

// C++ static members of PollutantsInterface (port as module-level statics, see AGENTS.md):
//   static const int PollutantsInterface::ZERO_EMISSIONS;
//   static const int PollutantsInterface::HEAVY_BIT;
//   static Helper PollutantsInterface::myZeroHelper;
//   static HelpersHBEFA PollutantsInterface::myHBEFA2Helper;
//   static HelpersHBEFA3 PollutantsInterface::myHBEFA3Helper;
//   static HelpersPHEMlight PollutantsInterface::myPHEMlightHelper;
//   static HelpersEnergy PollutantsInterface::myEnergyHelper;
//   static HelpersMMPEVEM PollutantsInterface::myMMPEVEMHelper;
//   static HelpersPHEMlight5 PollutantsInterface::myPHEMlight5Helper;
//   static HelpersHBEFA4 PollutantsInterface::myHBEFA4Helper;
//   static Helper *[] PollutantsInterface::myHelpers;
//   static std::vector<std::string> PollutantsInterface::myAllClassesStr;

impl PollutantsInterface {
    /// C++: `SUMOEmissionClass PollutantsInterface::getClassByName(const std::string &, const SUMOVehicleClass)` [static]
    /// Checks whether the string describes a known vehicle class
    // SUMOEmissionClass
    // PollutantsInterface::getClassByName(const std::string& eClass, const SUMOVehicleClass vc) {
    //     const std::string::size_type sep = eClass.find("/");
    //     const std::string model = eClass.substr(0, sep); // this includes the case of no separator
    //     for (int i = 0; i < 8; i++) {
    //         if (myHelpers[i]->getName() == model) {
    //             if (sep != std::string::npos) {
    //                 const std::string subClass = eClass.substr(sep + 1);
    //                 if (subClass == "zero") {
    //                     return myZeroHelper.getClassByName("default", vc);
    //                 }
    //                 return myHelpers[i]->getClassByName(subClass, vc);
    //             }
    //             return myHelpers[i]->getClassByName("default", vc);
    //         }
    //     }
    //     if (sep == std::string::npos) {
    //         if (eClass == "zero") {
    //             return myZeroHelper.getClassByName("default", vc);
    //         }
    //         WRITE_WARNINGF("Emission classes should always use the model as a prefix, please recheck '%'. In later SUMO versions this might become an error.", eClass)
    //         // default HBEFA2
    //         return myHBEFA2Helper.getClassByName(eClass, vc);
    //     }
    //     throw InvalidArgument(TLF("Unknown emission class '%'.", eClass));
    // }
    #[allow(unused_variables)]
    pub fn get_class_by_name(eClass: &str, vc: SUMOVehicleClass) -> SUMOEmissionClass {
        let sep = eClass.find('/');
        let model = &eClass[..sep.unwrap_or(eClass.len())]; // this includes the case of no separator
        for i in 0..8usize {
            if with_helper!(i, |h| h.get_name() == model) {
                if let Some(sep) = sep {
                    let subClass = &eClass[sep + 1..];
                    if subClass == "zero" {
                        return with_helper!(0, |h| h.get_class_by_name("default", vc));
                    }
                    return with_helper!(i, |h| h.get_class_by_name(subClass, vc));
                }
                return with_helper!(i, |h| h.get_class_by_name("default", vc));
            }
        }
        if sep.is_none() {
            if eClass == "zero" {
                return with_helper!(0, |h| h.get_class_by_name("default", vc));
            }
            write_warningf!("Emission classes should always use the model as a prefix, please recheck '%'. In later SUMO versions this might become an error.", eClass);
            // default HBEFA2
            return with_helper!(1, |h| h.get_class_by_name(eClass, vc));
        }
        panic!("Unknown emission class '{}'.", eClass)
    }

    /// C++: `const std::vector<SUMOEmissionClass> PollutantsInterface::getAllClasses()` [static]
    /// Checks whether the string describes a known vehicle class
    // const std::vector<SUMOEmissionClass>
    // PollutantsInterface::getAllClasses() {
    //     std::vector<SUMOEmissionClass> result;
    //     for (int i = 0; i < 8; i++) {
    //         myHelpers[i]->addAllClassesInto(result);
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_all_classes() -> Vec<SUMOEmissionClass> {
        let mut result: Vec<SUMOEmissionClass> = Vec::new();
        for i in 0..8usize {
            with_helper!(i, |h| h.add_all_classes_into(&mut result));
        }
        result
    }

    /// C++: `const std::vector<std::string> & PollutantsInterface::getAllClassesStr()` [static]
    /// Get all SUMOEmissionClass in string format
    // const std::vector<std::string>&
    // PollutantsInterface::getAllClassesStr() {
    //     // first check if myAllClassesStr has to be filled
    //     if (myAllClassesStr.empty()) {
    //         // first obtain all emissionClasses
    //         std::vector<SUMOEmissionClass> emissionClasses;
    //         for (int i = 0; i < 8; i++) {
    //             myHelpers[i]->addAllClassesInto(emissionClasses);
    //         }
    //         // now write all emissionClasses in myAllClassesStr
    //         for (const auto& i : emissionClasses) {
    //             myAllClassesStr.push_back(getName(i));
    //         }
    //     }
    //     return myAllClassesStr;
    // }
    #[allow(unused_variables)]
    pub fn get_all_classes_str() -> &'static Vec<String> {
        if let Some(v) = ALL_CLASSES_STR.with(|c| c.get()).and_then(|p| p.get()) {
            return v;
        }
        // first obtain all emissionClasses
        let mut emissionClasses: Vec<SUMOEmissionClass> = Vec::new();
        for i in 0..8usize {
            with_helper!(i, |h| h.add_all_classes_into(&mut emissionClasses));
        }
        // now write all emissionClasses in myAllClassesStr
        let mut strs: Vec<String> = Vec::new();
        for i in emissionClasses {
            strs.push(PollutantsInterface::get_name(i));
        }
        let p = leak_ptr(strs);
        ALL_CLASSES_STR.with(|c| c.set(Some(p)));
        p.get().unwrap()
    }

    /// C++: `std::string PollutantsInterface::getName(const SUMOEmissionClass)` [static]
    /// Checks whether the string describes a known vehicle class
    // std::string
    // PollutantsInterface::getName(const SUMOEmissionClass c) {
    //     return myHelpers[c >> 16]->getClassName(c);
    // }
    #[allow(unused_variables)]
    pub fn get_name(c: SUMOEmissionClass) -> String {
        with_helper!(c >> 16, |h| h.get_class_name(c))
    }

    /// C++: `std::string PollutantsInterface::getPollutantName(const EmissionType)` [static]
    /// return the name for the given emission type
    // std::string
    // PollutantsInterface::getPollutantName(const EmissionType e) {
    //     switch (e) {
    //         case CO2:
    //             return "CO2";
    //         case CO:
    //             return "CO";
    //         case HC:
    //             return "HC";
    //         case FUEL:
    //             return "fuel";
    //         case NO_X:
    //             return "NOx";
    //         case PM_X:
    //             return "PMx";
    //         case ELEC:
    //             return "electricity";
    //         default:
    //             throw InvalidArgument("Unknown emission type '" + toString(e) + "'");
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_pollutant_name(e: PollutantsInterface_EmissionType) -> String {
        match e {
            PollutantsInterface_EmissionType::CO2 => "CO2".to_string(),
            PollutantsInterface_EmissionType::CO => "CO".to_string(),
            PollutantsInterface_EmissionType::HC => "HC".to_string(),
            PollutantsInterface_EmissionType::FUEL => "fuel".to_string(),
            PollutantsInterface_EmissionType::NO_X => "NOx".to_string(),
            PollutantsInterface_EmissionType::PM_X => "PMx".to_string(),
            PollutantsInterface_EmissionType::ELEC => "electricity".to_string(),
        }
    }

    /// C++: `bool PollutantsInterface::isHeavy(const SUMOEmissionClass)` [static]
    /// Checks whether the emission class describes a bus, truck or similar vehicle
    // bool
    // PollutantsInterface::isHeavy(const SUMOEmissionClass c) {
    //     return (c & HEAVY_BIT) != 0;
    // }
    #[allow(unused_variables)]
    pub fn is_heavy(c: SUMOEmissionClass) -> bool {
        (c & HEAVY_BIT) != 0
    }

    /// C++: `bool PollutantsInterface::isSilent(const SUMOEmissionClass)` [static]
    /// Checks whether the emission class describes an electric or similar silent vehicle
    // bool
    // PollutantsInterface::isSilent(const SUMOEmissionClass c) {
    //     return myHelpers[c >> 16]->isSilent(c);
    // }
    #[allow(unused_variables)]
    pub fn is_silent(c: SUMOEmissionClass) -> bool {
        with_helper!(c >> 16, |h| h.is_silent(c))
    }

    /// C++: `SUMOEmissionClass PollutantsInterface::getClass(const SUMOEmissionClass, const std::string &, const std::string &, const std::string &, const double)` [static]
    /// Returns the emission class fittig the given parameters
    // SUMOEmissionClass
    // PollutantsInterface::getClass(const SUMOEmissionClass base, const std::string& vClass,
    //                               const std::string& fuel, const std::string& eClass, const double weight) {
    //     return myHelpers[base >> 16]->getClass(base, vClass, fuel, eClass, weight);
    // }
    #[allow(unused_variables)]
    pub fn get_class(base: SUMOEmissionClass, vClass: &str, fuel: &str, eClass: &str, weight: f64) -> SUMOEmissionClass {
        with_helper!(base >> 16, |h| h.get_class(base, vClass, fuel, eClass, weight))
    }

    /// C++: `std::string PollutantsInterface::getAmitranVehicleClass(const SUMOEmissionClass)` [static]
    /// Returns the vehicle class described by the given emission class
    // std::string
    // PollutantsInterface::getAmitranVehicleClass(const SUMOEmissionClass c) {
    //     return myHelpers[c >> 16]->getAmitranVehicleClass(c);
    // }
    #[allow(unused_variables)]
    pub fn get_amitran_vehicle_class(c: SUMOEmissionClass) -> String {
        with_helper!(c >> 16, |h| h.get_amitran_vehicle_class(c))
    }

    /// C++: `std::string PollutantsInterface::getFuel(const SUMOEmissionClass)` [static]
    /// Returns the fuel type of the given emission class
    // std::string
    // PollutantsInterface::getFuel(const SUMOEmissionClass c) {
    //     return myHelpers[c >> 16]->getFuel(c);
    // }
    #[allow(unused_variables)]
    pub fn get_fuel(c: SUMOEmissionClass) -> String {
        with_helper!(c >> 16, |h| h.get_fuel(c))
    }

    /// C++: `int PollutantsInterface::getEuroClass(const SUMOEmissionClass)` [static]
    /// Returns the Euro norm described by the given emission class
    // int
    // PollutantsInterface::getEuroClass(const SUMOEmissionClass c) {
    //     return myHelpers[c >> 16]->getEuroClass(c);
    // }
    #[allow(unused_variables)]
    pub fn get_euro_class(c: SUMOEmissionClass) -> i32 {
        with_helper!(c >> 16, |h| h.get_euro_class(c))
    }

    /// C++: `double PollutantsInterface::getWeight(const SUMOEmissionClass)` [static]
    /// Returns a representative weight for the given emission class see http://colombo-fp7.eu/deliverables/COLOMBO_D4.2_ExtendedPHEMSUMO_v1.7.pdf
    // double
    // PollutantsInterface::getWeight(const SUMOEmissionClass c) {
    //     return myHelpers[c >> 16]->getWeight(c);
    // }
    #[allow(unused_variables)]
    pub fn get_weight(c: SUMOEmissionClass) -> f64 {
        with_helper!(c >> 16, |h| h.get_weight(c))
    }

    /// C++: `double PollutantsInterface::compute(const SUMOEmissionClass, const EmissionType, const double, const double, const double, const EnergyParams *)` [static]
    /// Returns the amount of the emitted pollutant given the vehicle type and state (in mg/s or ml/s for fuel)
    // double
    // PollutantsInterface::compute(const SUMOEmissionClass c, const EmissionType e, const double v, const double a, const double slope, const EnergyParams* param) {
    //     return myHelpers[c >> 16]->compute(c, e, v, a, slope, param);
    // }
    #[allow(unused_variables)]
    pub fn compute(c: SUMOEmissionClass, e: PollutantsInterface_EmissionType, v: f64, a: f64, slope: f64, param: Ptr<EnergyParams>) -> f64 {
        with_helper!(c >> 16, |h| h.compute(c, e, v, a, slope, param))
    }

    /// C++: `Emissions PollutantsInterface::computeAll(const SUMOEmissionClass, const double, const double, const double, const EnergyParams *)` [static]
    /// Returns the amount of all emitted pollutants given the vehicle type and state (in mg/s or ml/s for fuel)
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn compute_all(c: SUMOEmissionClass, v: f64, a: f64, slope: f64, param: Ptr<EnergyParams>) -> PollutantsInterface_Emissions {
        with_helper!(c >> 16, |h| PollutantsInterface_Emissions::new(
            h.compute(c, PollutantsInterface_EmissionType::CO2, v, a, slope, param),
            h.compute(c, PollutantsInterface_EmissionType::CO, v, a, slope, param),
            h.compute(c, PollutantsInterface_EmissionType::HC, v, a, slope, param),
            h.compute(c, PollutantsInterface_EmissionType::FUEL, v, a, slope, param),
            h.compute(c, PollutantsInterface_EmissionType::NO_X, v, a, slope, param),
            h.compute(c, PollutantsInterface_EmissionType::PM_X, v, a, slope, param),
            h.compute(c, PollutantsInterface_EmissionType::ELEC, v, a, slope, param)))
    }

    /// C++: `double PollutantsInterface::computeDefault(const SUMOEmissionClass, const EmissionType, const double, const double, const double, const double, const EnergyParams *)` [static]
    /// Returns the amount of emitted pollutant given the vehicle type and default values for the state (in mg)
    // double
    // PollutantsInterface::computeDefault(const SUMOEmissionClass c, const EmissionType e, const double v, const double a, const double slope, const double tt, const EnergyParams* param) {
    //     const Helper* const h = myHelpers[c >> 16];
    //     return (h->compute(c, e, v, 0, slope, param) + h->compute(c, e, v - a, a, slope, param)) * tt / 2.;
    // }
    #[allow(unused_variables)]
    pub fn compute_default(c: SUMOEmissionClass, e: PollutantsInterface_EmissionType, v: f64, a: f64, slope: f64, tt: f64, param: Ptr<EnergyParams>) -> f64 {
        with_helper!(c >> 16, |h| (h.compute(c, e, v, 0., slope, param) + h.compute(c, e, v - a, a, slope, param)) * tt / 2.)
    }

    /// C++: `double PollutantsInterface::getModifiedAccel(const SUMOEmissionClass, const double, const double, const double, const EnergyParams *)` [static]
    /// Returns the adapted acceleration value, useful for comparing with external PHEMlight references.
    // double
    // PollutantsInterface::getModifiedAccel(const SUMOEmissionClass c, const double v, const double a, const double slope, const EnergyParams* param) {
    //     return myHelpers[c >> 16]->getModifiedAccel(c, v, a, slope, param);
    // }
    #[allow(unused_variables)]
    pub fn get_modified_accel(c: SUMOEmissionClass, v: f64, a: f64, slope: f64, param: Ptr<EnergyParams>) -> f64 {
        with_helper!(c >> 16, |h| h.get_modified_accel(c, v, a, slope, param))
    }

    /// C++: `double PollutantsInterface::getCoastingDecel(const SUMOEmissionClass, const double, const double, const double, const EnergyParams *)` [static]
    /// Returns the coasting deceleration value, useful for comparing with external PHEMlight references.
    // double
    // PollutantsInterface::getCoastingDecel(const SUMOEmissionClass c, const double v, const double a, const double slope, const EnergyParams* param) {
    //     return myHelpers[c >> 16]->getCoastingDecel(c, v, a, slope, param);
    // }
    #[allow(unused_variables)]
    pub fn get_coasting_decel(c: SUMOEmissionClass, v: f64, a: f64, slope: f64, param: Ptr<EnergyParams>) -> f64 {
        with_helper!(c >> 16, |h| h.get_coasting_decel(c, v, a, slope, param))
    }

    /// C++: `const HelpersEnergy & PollutantsInterface::getEnergyHelper()` [static]
    /// get energy helper
    // const HelpersEnergy&
    // PollutantsInterface::getEnergyHelper() {
    //     return myEnergyHelper;
    // }
    #[allow(unused_variables)]
    pub fn get_energy_helper() -> &'static HelpersEnergy {
        helper_table().energy.get().unwrap()
    }

}

