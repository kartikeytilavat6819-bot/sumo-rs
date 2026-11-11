//! Port of utils/emissions/PollutantsInterface.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

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
        todo!("port Emissions::Emissions")
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
        todo!("port Emissions::addScaled")
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
        todo!("port Helper::Helper")
    }

    /// C++: `void Helper::~Helper() noexcept` [virtual]
    /// empty destructor
    // virtual ~Helper() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port Helper::~Helper")
    }

    /// C++: `const std::string & Helper::getName() const` [const]
    /// Returns the name of the model
    // const
    // std::string& PollutantsInterface::Helper::getName() const {
    //     return myName;
    // }
    #[allow(unused_variables)]
    pub fn get_name(&self) -> &str {
        todo!("port Helper::getName")
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
        todo!("port Helper::getClassByName")
    }

    /// C++: `const std::string Helper::getClassName(const SUMOEmissionClass) const` [const]
    /// Returns the complete name of the emission class including the model
    // const std::string
    // PollutantsInterface::Helper::getClassName(const SUMOEmissionClass c) const {
    //     return myName + "/" + myEmissionClassStrings.getString(c);
    // }
    #[allow(unused_variables)]
    pub fn get_class_name(&self, c: SUMOEmissionClass) -> String {
        todo!("port Helper::getClassName")
    }

    /// C++: `bool Helper::isSilent(const SUMOEmissionClass)` [virtual]
    /// Returns whether the class denotes a silent vehicle for interfacing with the noise model. By default the first class in each model is the silent class.
    // bool
    // PollutantsInterface::Helper::isSilent(const SUMOEmissionClass c) {
    //     return (c & (int)0xffffffff & ~HEAVY_BIT) == 0;
    // }
    #[allow(unused_variables)]
    pub fn is_silent(&mut self, c: SUMOEmissionClass) -> bool {
        todo!("port Helper::isSilent")
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
        todo!("port Helper::getClass")
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
        todo!("port Helper::getAmitranVehicleClass")
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
        todo!("port Helper::getFuel")
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
        todo!("port Helper::getEuroClass")
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
        todo!("port Helper::getWeight")
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
        todo!("port Helper::compute")
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
        todo!("port Helper::getModifiedAccel")
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
        todo!("port Helper::getCoastingDecel")
    }

    /// C++: `void Helper::addAllClassesInto(std::vector<SUMOEmissionClass> &) const` [const]
    /// Add all known emission classes of this model to the given container
    // void
    // PollutantsInterface::Helper::addAllClassesInto(std::vector<SUMOEmissionClass>& list) const {
    //     myEmissionClassStrings.addKeysInto(list);
    // }
    #[allow(unused_variables)]
    pub fn add_all_classes_into(&self, list: &mut Vec<SUMOEmissionClass>) {
        todo!("port Helper::addAllClassesInto")
    }

    /// C++: `bool Helper::includesClass(const SUMOEmissionClass) const` [const]
    // bool
    // PollutantsInterface::Helper::includesClass(const SUMOEmissionClass c) const {
    //     return (c >> 16) == (myBaseIndex >> 16);
    // }
    #[allow(unused_variables)]
    pub fn includes_class(&self, c: SUMOEmissionClass) -> bool {
        todo!("port Helper::includesClass")
    }

    /// C++: `Helper & Helper::operator=(const Helper &)` [private]
    /// invalidate copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &Helper) -> &mut Helper {
        todo!("port Helper::operator=")
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
        todo!("port PollutantsInterface::getClassByName")
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
        todo!("port PollutantsInterface::getAllClasses")
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
        todo!("port PollutantsInterface::getAllClassesStr")
    }

    /// C++: `std::string PollutantsInterface::getName(const SUMOEmissionClass)` [static]
    /// Checks whether the string describes a known vehicle class
    // std::string
    // PollutantsInterface::getName(const SUMOEmissionClass c) {
    //     return myHelpers[c >> 16]->getClassName(c);
    // }
    #[allow(unused_variables)]
    pub fn get_name(c: SUMOEmissionClass) -> String {
        todo!("port PollutantsInterface::getName")
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
        todo!("port PollutantsInterface::getPollutantName")
    }

    /// C++: `bool PollutantsInterface::isHeavy(const SUMOEmissionClass)` [static]
    /// Checks whether the emission class describes a bus, truck or similar vehicle
    // bool
    // PollutantsInterface::isHeavy(const SUMOEmissionClass c) {
    //     return (c & HEAVY_BIT) != 0;
    // }
    #[allow(unused_variables)]
    pub fn is_heavy(c: SUMOEmissionClass) -> bool {
        todo!("port PollutantsInterface::isHeavy")
    }

    /// C++: `bool PollutantsInterface::isSilent(const SUMOEmissionClass)` [static]
    /// Checks whether the emission class describes an electric or similar silent vehicle
    // bool
    // PollutantsInterface::isSilent(const SUMOEmissionClass c) {
    //     return myHelpers[c >> 16]->isSilent(c);
    // }
    #[allow(unused_variables)]
    pub fn is_silent(c: SUMOEmissionClass) -> bool {
        todo!("port PollutantsInterface::isSilent")
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
        todo!("port PollutantsInterface::getClass")
    }

    /// C++: `std::string PollutantsInterface::getAmitranVehicleClass(const SUMOEmissionClass)` [static]
    /// Returns the vehicle class described by the given emission class
    // std::string
    // PollutantsInterface::getAmitranVehicleClass(const SUMOEmissionClass c) {
    //     return myHelpers[c >> 16]->getAmitranVehicleClass(c);
    // }
    #[allow(unused_variables)]
    pub fn get_amitran_vehicle_class(c: SUMOEmissionClass) -> String {
        todo!("port PollutantsInterface::getAmitranVehicleClass")
    }

    /// C++: `std::string PollutantsInterface::getFuel(const SUMOEmissionClass)` [static]
    /// Returns the fuel type of the given emission class
    // std::string
    // PollutantsInterface::getFuel(const SUMOEmissionClass c) {
    //     return myHelpers[c >> 16]->getFuel(c);
    // }
    #[allow(unused_variables)]
    pub fn get_fuel(c: SUMOEmissionClass) -> String {
        todo!("port PollutantsInterface::getFuel")
    }

    /// C++: `int PollutantsInterface::getEuroClass(const SUMOEmissionClass)` [static]
    /// Returns the Euro norm described by the given emission class
    // int
    // PollutantsInterface::getEuroClass(const SUMOEmissionClass c) {
    //     return myHelpers[c >> 16]->getEuroClass(c);
    // }
    #[allow(unused_variables)]
    pub fn get_euro_class(c: SUMOEmissionClass) -> i32 {
        todo!("port PollutantsInterface::getEuroClass")
    }

    /// C++: `double PollutantsInterface::getWeight(const SUMOEmissionClass)` [static]
    /// Returns a representative weight for the given emission class see http://colombo-fp7.eu/deliverables/COLOMBO_D4.2_ExtendedPHEMSUMO_v1.7.pdf
    // double
    // PollutantsInterface::getWeight(const SUMOEmissionClass c) {
    //     return myHelpers[c >> 16]->getWeight(c);
    // }
    #[allow(unused_variables)]
    pub fn get_weight(c: SUMOEmissionClass) -> f64 {
        todo!("port PollutantsInterface::getWeight")
    }

    /// C++: `double PollutantsInterface::compute(const SUMOEmissionClass, const EmissionType, const double, const double, const double, const EnergyParams *)` [static]
    /// Returns the amount of the emitted pollutant given the vehicle type and state (in mg/s or ml/s for fuel)
    // double
    // PollutantsInterface::compute(const SUMOEmissionClass c, const EmissionType e, const double v, const double a, const double slope, const EnergyParams* param) {
    //     return myHelpers[c >> 16]->compute(c, e, v, a, slope, param);
    // }
    #[allow(unused_variables)]
    pub fn compute(c: SUMOEmissionClass, e: PollutantsInterface_EmissionType, v: f64, a: f64, slope: f64, param: Ptr<EnergyParams>) -> f64 {
        todo!("port PollutantsInterface::compute")
    }

    /// C++: `Emissions PollutantsInterface::computeAll(const SUMOEmissionClass, const double, const double, const double, const EnergyParams *)` [static]
    /// Returns the amount of all emitted pollutants given the vehicle type and state (in mg/s or ml/s for fuel)
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn compute_all(c: SUMOEmissionClass, v: f64, a: f64, slope: f64, param: Ptr<EnergyParams>) -> PollutantsInterface_Emissions {
        todo!("port PollutantsInterface::computeAll")
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
        todo!("port PollutantsInterface::computeDefault")
    }

    /// C++: `double PollutantsInterface::getModifiedAccel(const SUMOEmissionClass, const double, const double, const double, const EnergyParams *)` [static]
    /// Returns the adapted acceleration value, useful for comparing with external PHEMlight references.
    // double
    // PollutantsInterface::getModifiedAccel(const SUMOEmissionClass c, const double v, const double a, const double slope, const EnergyParams* param) {
    //     return myHelpers[c >> 16]->getModifiedAccel(c, v, a, slope, param);
    // }
    #[allow(unused_variables)]
    pub fn get_modified_accel(c: SUMOEmissionClass, v: f64, a: f64, slope: f64, param: Ptr<EnergyParams>) -> f64 {
        todo!("port PollutantsInterface::getModifiedAccel")
    }

    /// C++: `double PollutantsInterface::getCoastingDecel(const SUMOEmissionClass, const double, const double, const double, const EnergyParams *)` [static]
    /// Returns the coasting deceleration value, useful for comparing with external PHEMlight references.
    // double
    // PollutantsInterface::getCoastingDecel(const SUMOEmissionClass c, const double v, const double a, const double slope, const EnergyParams* param) {
    //     return myHelpers[c >> 16]->getCoastingDecel(c, v, a, slope, param);
    // }
    #[allow(unused_variables)]
    pub fn get_coasting_decel(c: SUMOEmissionClass, v: f64, a: f64, slope: f64, param: Ptr<EnergyParams>) -> f64 {
        todo!("port PollutantsInterface::getCoastingDecel")
    }

    /// C++: `const HelpersEnergy & PollutantsInterface::getEnergyHelper()` [static]
    /// get energy helper
    // const HelpersEnergy&
    // PollutantsInterface::getEnergyHelper() {
    //     return myEnergyHelper;
    // }
    #[allow(unused_variables)]
    pub fn get_energy_helper() -> &'static HelpersEnergy {
        todo!("port PollutantsInterface::getEnergyHelper")
    }

}

