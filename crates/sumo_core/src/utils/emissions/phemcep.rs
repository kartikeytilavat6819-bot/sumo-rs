//! Port of utils/emissions/PHEMCEP.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// PHEMConstants.h: ZERO_SPEED_ACCURACY (0.1 here; the foreign PHEMlight Constants class uses 0.5)
const ZERO_SPEED_ACCURACY: f64 = 0.1;
/// PHEMConstants.h: M_PI2 is defined as pi in that header
const M_PI2: f64 = std::f64::consts::PI;
/// StringBijection::insert with checkDuplicates = true
fn sb_insert<T: PartialEq>(map: &mut BTreeMap<String, T>, str_: &str, key: T) {
    if map.values().any(|v| *v == key) {
        panic!("{}", ProcessError::InvalidArgument("Duplicate key.".to_string()));
    }
    if map.contains_key(str_) {
        panic!("{}", ProcessError::InvalidArgument(format!("Duplicate string '{}'.", str_)));
    }
    map.insert(str_.to_string(), key);
}
/// StringBijection::get(string): throws when the string is unknown
fn sb_get<T: Clone>(map: &BTreeMap<String, T>, str_: &str) -> T {
    match map.get(str_) {
        Some(v) => v.clone(),
        None => panic!("{}", ProcessError::InvalidArgument(format!("String '{}' not found.", str_))),
    }
}

/// Data Handler for a single CEP emission data set
/// C++ class `PHEMCEP` (utils/emissions/PHEMCEP.h)
pub struct PHEMCEP {
    /// PHEM emission class of vehicle
    pub _emissionClass: SUMOEmissionClass,
    pub _normalizingType: NormalizingType,
    /// Rolling resistance f0
    pub _resistanceF0: f64,
    /// Rolling resistance f1
    pub _resistanceF1: f64,
    /// Rolling resistance f2
    pub _resistanceF2: f64,
    /// Rolling resistance f3
    pub _resistanceF3: f64,
    /// Rolling resistance f4
    pub _resistanceF4: f64,
    /// Cw value
    pub _cdValue: f64,
    /// crosssectional area of vehicle
    pub _crossSectionalArea: f64,
    /// vehicle mass
    pub _massVehicle: f64,
    /// vehicle loading
    pub _vehicleLoading: f64,
    /// rotational mass of vehicle
    pub _massRot: f64,
    /// rated power of vehicle
    pub _ratedPower: f64,
    /// Step functions parameter for maximum rated power
    pub _pNormV0: f64,
    /// Step functions parameter for maximum rated power
    pub _pNormP0: f64,
    /// Step functions parameter for maximum rated power
    pub _pNormV1: f64,
    /// Step functions parameter for maximum rated power
    pub _pNormP1: f64,
    pub _axleRatio: f64,
    pub _engineIdlingSpeed: f64,
    pub _engineRatedSpeed: f64,
    pub _effictiveWheelDiameter: f64,
    pub _idlingFC: f64,
    pub _vehicleFuelType: String,
    pub _sizeOfPatternFC: i32,
    pub _sizeOfPatternPollutants: i32,
    pub _normalizingPower: f64,
    pub _drivingPower: f64,
    pub _heavyVehicle: bool,
    pub _speedPatternRotational: Vec<f64>,
    pub _powerPatternFC: Vec<f64>,
    pub _powerPatternPollutants: Vec<f64>,
    pub _normalizedPowerPatternFC: Vec<f64>,
    pub _normailzedPowerPatternPollutants: Vec<f64>,
    pub _cepCurveFC: Vec<f64>,
    pub _normedCepCurveFC: Vec<f64>,
    pub _speedCurveRotational: Vec<f64>,
    pub _gearTransmissionCurve: Vec<f64>,
    pub _nNormTable: Vec<f64>,
    pub _dragNormTable: Vec<f64>,
    pub _cepCurvePollutants: BTreeMap<String, Vec<f64>>,
    pub _normalizedCepCurvePollutants: BTreeMap<String, Vec<f64>>,
    pub _idlingValuesPollutants: BTreeMap<String, f64>,
}

impl PHEMCEP {
    /// C++: `void PHEMCEP::PHEMCEP(bool, SUMOEmissionClass, const std::string &, double, double, double, double, double, double, double, double, double, double, double, double, double, double, double, double, double, double, double, double, const std::string &, const std::vector<std::vector<double>> &, const std::vector<std::string> &, const std::vector<std::vector<double>> &, const std::vector<std::vector<double>> &, const std::vector<std::vector<double>> &, const std::vector<double> &)`
    /// Constructor
    // PHEMCEP::PHEMCEP(bool heavyVehicle, SUMOEmissionClass emissionClass, const std::string& emissionClassIdentifier,
    //                  double vehicleMass, double vehicleLoading, double vehicleMassRot,
    //                  double crossArea, double cdValue,
    //                  double f0, double f1, double f2, double f3, double f4,
    //                  double ratedPower, double pNormV0, double pNormP0, double pNormV1, double pNormP1,
    //                  double axleRatio, double engineIdlingSpeed, double engineRatedSpeed, double effectiveWheelDiameter,
    //                  double idlingFC,
    //                  const std::string& vehicleFuelType,
    //                  const std::vector< std::vector<double> >& matrixFC,
    //                  const std::vector<std::string>& headerLinePollutants,
    //                  const std::vector< std::vector<double> >& matrixPollutants,
    //                  const std::vector< std::vector<double> >& matrixSpeedRotational,
    //                  const std::vector< std::vector<double> >& normedDragTable,
    //                  const std::vector<double>& idlingValuesPollutants) {
    //     _emissionClass = emissionClass;
    //     _resistanceF0 = f0;
    //     _resistanceF1 = f1;
    //     _resistanceF2 = f2;
    //     _resistanceF3 = f3;
    //     _resistanceF4 = f4;
    //     _cdValue = cdValue;
    //     _crossSectionalArea = crossArea;
    //     _massVehicle = vehicleMass;
    //     _vehicleLoading = vehicleLoading;
    //     _massRot = vehicleMassRot;
    //     _ratedPower = ratedPower;
    //     _vehicleFuelType = vehicleFuelType;
    //
    //     _pNormV0 = pNormV0 / 3.6;
    //     _pNormP0 = pNormP0;
    //     _pNormV1 = pNormV1 / 3.6;
    //     _pNormP1 = pNormP1;
    //
    //     _axleRatio = axleRatio;
    //     _engineIdlingSpeed = engineIdlingSpeed;
    //     _engineRatedSpeed = engineRatedSpeed;
    //     _effictiveWheelDiameter = effectiveWheelDiameter;
    //
    //     _heavyVehicle = heavyVehicle;
    //     _idlingFC = idlingFC;
    //
    //     std::vector<std::string> pollutantIdentifier;
    //     std::vector< std::vector<double> > pollutantMeasures;
    //     std::vector<std::vector<double> > normalizedPollutantMeasures;
    //
    //     // init pollutant identifiers
    //     for (int i = 0; i < (int)headerLinePollutants.size(); i++) {
    //         pollutantIdentifier.push_back(headerLinePollutants[i]);
    //     } // end for
    //
    //     // get size of powerPatterns
    //     _sizeOfPatternFC = (int)matrixFC.size();
    //     _sizeOfPatternPollutants = (int)matrixPollutants.size();
    //
    //     // initialize measures
    //     for (int i = 0; i < (int)headerLinePollutants.size(); i++) {
    //         pollutantMeasures.push_back(std::vector<double>());
    //         normalizedPollutantMeasures.push_back(std::vector<double>());
    //     } // end for
    //
    //     // looping through matrix and assigning values for speed rotational table
    //     _speedCurveRotational.clear();
    //     _speedPatternRotational.clear();
    //     _gearTransmissionCurve.clear();
    //     for (int i = 0; i < (int)matrixSpeedRotational.size(); i++) {
    //         if (matrixSpeedRotational[i].size() != 3) {
    //             throw InvalidArgument("Error loading vehicle file for: " + emissionClassIdentifier);
    //         }
    //
    //         _speedPatternRotational.push_back(matrixSpeedRotational[i][0] / 3.6);
    //         _speedCurveRotational.push_back(matrixSpeedRotational[i][1]);
    //         _gearTransmissionCurve.push_back(matrixSpeedRotational[i][2]);
    //     } // end for
    //
    //     // looping through matrix and assigning values for drag table
    //     _nNormTable.clear();
    //     _dragNormTable.clear();
    //     for (int i = 0; i < (int) normedDragTable.size(); i++) {
    //         if (normedDragTable[i].size() != 2) {
    //             return;
    //         }
    //
    //         _nNormTable.push_back(normedDragTable[i][0]);
    //         _dragNormTable.push_back(normedDragTable[i][1]);
    //
    //     } // end for
    //
    //     // looping through matrix and assigning values for Fuel consumption
    //     _cepCurveFC.clear();
    //     _powerPatternFC.clear();
    //     _normalizedPowerPatternFC.clear();
    //     _normedCepCurveFC.clear();
    //     for (int i = 0; i < (int)matrixFC.size(); i++) {
    //         if (matrixFC[i].size() != 2) {
    //             throw InvalidArgument("Error loading vehicle file for: " + emissionClassIdentifier);
    //         }
    //
    //         _powerPatternFC.push_back(matrixFC[i][0] * _ratedPower);
    //         _normalizedPowerPatternFC.push_back(matrixFC[i][0]);
    //         _cepCurveFC.push_back(matrixFC[i][1] * _ratedPower);
    //         _normedCepCurveFC.push_back(matrixFC[i][1]);
    //
    //     } // end for
    //
    //     _powerPatternPollutants.clear();
    //     double pollutantMultiplyer = 1;
    //
    //     _drivingPower = _normalizingPower = CalcPower(NORMALIZING_SPEED, NORMALIZING_ACCELARATION, 0, vehicleLoading);
    //
    //     // looping through matrix and assigning values for pollutants
    //
    //     if (heavyVehicle) {
    //         _normalizingPower = _ratedPower;
    //         pollutantMultiplyer = _ratedPower;
    //         _normalizingType = RatedPower;
    //     } else {
    //         _normalizingPower = _drivingPower;
    //         _normalizingType = DrivingPower;
    //     } // end if
    //
    //     const int headerCount = (int)headerLinePollutants.size();
    //     for (int i = 0; i < (int)matrixPollutants.size(); i++) {
    //         for (int j = 0; j < (int)matrixPollutants[i].size(); j++) {
    //             if ((int)matrixPollutants[i].size() != headerCount + 1) {
    //                 return;
    //             }
    //
    //             if (j == 0) {
    //                 _normailzedPowerPatternPollutants.push_back(matrixPollutants[i][j]);
    //                 _powerPatternPollutants.push_back(matrixPollutants[i][j] * _normalizingPower);
    //             } else {
    //                 pollutantMeasures[j - 1].push_back(matrixPollutants[i][j] * pollutantMultiplyer);
    //                 normalizedPollutantMeasures[j - 1].push_back(matrixPollutants[i][j]);
    //             } // end if
    //         } // end for
    //     } // end for
    //
    //     for (int i = 0; i < (int) headerLinePollutants.size(); i++) {
    //         _cepCurvePollutants.insert(pollutantIdentifier[i], pollutantMeasures[i]);
    //         _normalizedCepCurvePollutants.insert(pollutantIdentifier[i], normalizedPollutantMeasures[i]);
    //         _idlingValuesPollutants.insert(pollutantIdentifier[i], idlingValuesPollutants[i] * pollutantMultiplyer);
    //     } // end for
    //
    //     _idlingFC = idlingFC * _ratedPower;
    //
    // }
    #[allow(unused_variables)]
    pub fn new(heavyVehicel: bool, emissionClass: SUMOEmissionClass, emissionClassIdentifier: &str, vehicleMass: f64, vehicleLoading: f64, vehicleMassRot: f64, crossArea: f64, cdValue: f64, f0: f64, f1: f64, f2: f64, f3: f64, f4: f64, ratedPower: f64, pNormV0: f64, pNormP0: f64, pNormV1: f64, pNormP1: f64, axleRatio: f64, engineIdlingSpeed: f64, engineRatedSpeed: f64, effectiveWheelDiameter: f64, idlingFC: f64, vehicleFuelType: &str, matrixFC: &Vec<Vec<f64>>, headerLinePollutants: &Vec<String>, matrixPollutants: &Vec<Vec<f64>>, matrixSpeedRotational: &Vec<Vec<f64>>, normedDragTable: &Vec<Vec<f64>>, idlingValuesPollutants: &Vec<f64>) -> Self {
        let mut s = PHEMCEP {
            _emissionClass: 0,
            _normalizingType: NormalizingType::RatedPower,
            _resistanceF0: 0.0,
            _resistanceF1: 0.0,
            _resistanceF2: 0.0,
            _resistanceF3: 0.0,
            _resistanceF4: 0.0,
            _cdValue: 0.0,
            _crossSectionalArea: 0.0,
            _massVehicle: 0.0,
            _vehicleLoading: 0.0,
            _massRot: 0.0,
            _ratedPower: 0.0,
            _pNormV0: 0.0,
            _pNormP0: 0.0,
            _pNormV1: 0.0,
            _pNormP1: 0.0,
            _axleRatio: 0.0,
            _engineIdlingSpeed: 0.0,
            _engineRatedSpeed: 0.0,
            _effictiveWheelDiameter: 0.0,
            _idlingFC: 0.0,
            _vehicleFuelType: String::new(),
            _sizeOfPatternFC: 0,
            _sizeOfPatternPollutants: 0,
            _normalizingPower: 0.0,
            _drivingPower: 0.0,
            _heavyVehicle: false,
            _speedPatternRotational: Vec::new(),
            _powerPatternFC: Vec::new(),
            _powerPatternPollutants: Vec::new(),
            _normalizedPowerPatternFC: Vec::new(),
            _normailzedPowerPatternPollutants: Vec::new(),
            _cepCurveFC: Vec::new(),
            _normedCepCurveFC: Vec::new(),
            _speedCurveRotational: Vec::new(),
            _gearTransmissionCurve: Vec::new(),
            _nNormTable: Vec::new(),
            _dragNormTable: Vec::new(),
            _cepCurvePollutants: BTreeMap::new(),
            _normalizedCepCurvePollutants: BTreeMap::new(),
            _idlingValuesPollutants: BTreeMap::new(),
        };
        // the C++ constructor leaves its body early with return; a labeled block gives the same exit
        'ctor: {
            s._emissionClass = emissionClass;
            s._resistanceF0 = f0;
            s._resistanceF1 = f1;
            s._resistanceF2 = f2;
            s._resistanceF3 = f3;
            s._resistanceF4 = f4;
            s._cdValue = cdValue;
            s._crossSectionalArea = crossArea;
            s._massVehicle = vehicleMass;
            s._vehicleLoading = vehicleLoading;
            s._massRot = vehicleMassRot;
            s._ratedPower = ratedPower;
            s._vehicleFuelType = vehicleFuelType.to_string();

            s._pNormV0 = pNormV0 / 3.6;
            s._pNormP0 = pNormP0;
            s._pNormV1 = pNormV1 / 3.6;
            s._pNormP1 = pNormP1;

            s._axleRatio = axleRatio;
            s._engineIdlingSpeed = engineIdlingSpeed;
            s._engineRatedSpeed = engineRatedSpeed;
            s._effictiveWheelDiameter = effectiveWheelDiameter;

            s._heavyVehicle = heavyVehicel;
            s._idlingFC = idlingFC;

            let mut pollutantIdentifier: Vec<String> = Vec::new();
            let mut pollutantMeasures: Vec<Vec<f64>> = Vec::new();
            let mut normalizedPollutantMeasures: Vec<Vec<f64>> = Vec::new();

            // init pollutant identifiers
            for h in headerLinePollutants.iter() {
                pollutantIdentifier.push(h.clone());
            }

            // get size of powerPatterns
            s._sizeOfPatternFC = matrixFC.len() as i32;
            s._sizeOfPatternPollutants = matrixPollutants.len() as i32;

            // initialize measures
            for _ in 0..headerLinePollutants.len() {
                pollutantMeasures.push(Vec::new());
                normalizedPollutantMeasures.push(Vec::new());
            }

            // looping through matrix and assigning values for speed rotational table
            s._speedCurveRotational.clear();
            s._speedPatternRotational.clear();
            s._gearTransmissionCurve.clear();
            for row in matrixSpeedRotational.iter() {
                if row.len() != 3 {
                    panic!("{}", ProcessError::InvalidArgument(format!("Error loading vehicle file for: {}", emissionClassIdentifier)));
                }
                s._speedPatternRotational.push(row[0] / 3.6);
                s._speedCurveRotational.push(row[1]);
                s._gearTransmissionCurve.push(row[2]);
            }

            // looping through matrix and assigning values for drag table
            s._nNormTable.clear();
            s._dragNormTable.clear();
            for row in normedDragTable.iter() {
                if row.len() != 2 {
                    break 'ctor;
                }
                s._nNormTable.push(row[0]);
                s._dragNormTable.push(row[1]);
            }

            // looping through matrix and assigning values for Fuel consumption
            s._cepCurveFC.clear();
            s._powerPatternFC.clear();
            s._normalizedPowerPatternFC.clear();
            s._normedCepCurveFC.clear();
            for row in matrixFC.iter() {
                if row.len() != 2 {
                    panic!("{}", ProcessError::InvalidArgument(format!("Error loading vehicle file for: {}", emissionClassIdentifier)));
                }
                s._powerPatternFC.push(row[0] * s._ratedPower);
                s._normalizedPowerPatternFC.push(row[0]);
                s._cepCurveFC.push(row[1] * s._ratedPower);
                s._normedCepCurveFC.push(row[1]);
            }

            s._powerPatternPollutants.clear();
            let mut pollutantMultiplyer: f64 = 1.0;

            s._drivingPower = s.calc_power(NORMALIZING_SPEED, NORMALIZING_ACCELARATION, 0.0, vehicleLoading);
            s._normalizingPower = s._drivingPower;

            // looping through matrix and assigning values for pollutants
            if heavyVehicel {
                s._normalizingPower = s._ratedPower;
                pollutantMultiplyer = s._ratedPower;
                s._normalizingType = NormalizingType::RatedPower;
            } else {
                s._normalizingPower = s._drivingPower;
                s._normalizingType = NormalizingType::DrivingPower;
            }

            for i in 0..matrixPollutants.len() {
                for j in 0..matrixPollutants[i].len() {
                    if matrixPollutants[i].len() != headerLinePollutants.len() + 1 {
                        break 'ctor;
                    }

                    if j == 0 {
                        s._normailzedPowerPatternPollutants.push(matrixPollutants[i][j]);
                        s._powerPatternPollutants.push(matrixPollutants[i][j] * s._normalizingPower);
                    } else {
                        pollutantMeasures[j - 1].push(matrixPollutants[i][j] * pollutantMultiplyer);
                        normalizedPollutantMeasures[j - 1].push(matrixPollutants[i][j]);
                    }
                }
            }

            for i in 0..headerLinePollutants.len() {
                sb_insert(&mut s._cepCurvePollutants, &pollutantIdentifier[i], pollutantMeasures[i].clone());
                sb_insert(&mut s._normalizedCepCurvePollutants, &pollutantIdentifier[i], normalizedPollutantMeasures[i].clone());
                sb_insert(&mut s._idlingValuesPollutants, &pollutantIdentifier[i], idlingValuesPollutants[i] * pollutantMultiplyer);
            }

            s._idlingFC = idlingFC * s._ratedPower;
        }
        s
    }

    /// C++: `void PHEMCEP::~PHEMCEP()`
    /// Destructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // the C++ destructor is implicit: the members release themselves, which Rust drop does too
    }

    /// C++: `double PHEMCEP::CalcPower(double, double, double, double) const` [const]
    /// Returns the power of used for a vehicle at state v,a, slope and loading
    // double
    // PHEMCEP::CalcPower(double v, double a, double slope, double /* vehicleLoading */) const {
    //     const double rotFactor = GetRotationalCoeffecient(v);
    //     double power = (_massVehicle + _vehicleLoading) * GRAVITY_CONST * (_resistanceF0 + _resistanceF1 * v + _resistanceF4 * pow(v, 4)) * v;
    //     power += (_crossSectionalArea * _cdValue * AIR_DENSITY_CONST / 2) * pow(v, 3);
    //     power += (_massVehicle * rotFactor + _massRot + _vehicleLoading) * a * v;
    //     power += (_massVehicle + _vehicleLoading) * slope * 0.01 * v;
    //     return power / 950.;
    // }
    #[allow(unused_variables)]
    pub fn calc_power(&self, v: f64, a: f64, slope: f64, vehicleLoading: f64) -> f64 {
        let rotFactor = self.get_rotational_coeffecient(v);
        let mut power = (self._massVehicle + self._vehicleLoading) * GRAVITY_CONST * (self._resistanceF0 + self._resistanceF1 * v + self._resistanceF4 * v.powf(4.0)) * v;
        power += (self._crossSectionalArea * self._cdValue * AIR_DENSITY_CONST / 2.0) * v.powf(3.0);
        power += (self._massVehicle * rotFactor + self._massRot + self._vehicleLoading) * a * v;
        power += (self._massVehicle + self._vehicleLoading) * slope * 0.01 * v;
        power / 950.0
    }

    /// C++: `double PHEMCEP::GetMaxAccel(double, double, double, double) const` [const]
    /// Returns the maximum accelaration for a vehicle at state v,a, slope and loading
    // double
    // PHEMCEP::GetMaxAccel(double v, double a, double gradient, double /* vehicleLoading */) const {
    //     UNUSED_PARAMETER(a);
    //     double rotFactor = GetRotationalCoeffecient(v);
    //     const double pMaxForAcc = GetPMaxNorm(v) * _ratedPower - PHEMCEP::CalcPower(v, 0, gradient, _vehicleLoading); // !!!vehicleLoading
    //     return (pMaxForAcc * 1000) / ((_massVehicle * rotFactor + _massRot + _vehicleLoading) * v); // !!!vehicleLoading
    // }
    #[allow(unused_variables)]
    pub fn get_max_accel(&self, v: f64, a: f64, gradient: f64, vehicleLoading: f64) -> f64 {
        let _ = a;
        let rotFactor = self.get_rotational_coeffecient(v);
        let pMaxForAcc = self.get_p_max_norm(v) * self._ratedPower - self.calc_power(v, 0.0, gradient, self._vehicleLoading); // !!!vehicleLoading
        (pMaxForAcc * 1000.0) / ((self._massVehicle * rotFactor + self._massRot + self._vehicleLoading) * v) // !!!vehicleLoading
    }

    /// C++: `double PHEMCEP::GetEmission(const std::string &, double, double, bool) const` [const]
    /// Returns a emission measure for power[kW] level
    // double
    // PHEMCEP::GetEmission(const std::string& pollutant, double power, double speed, bool normalized) const {
    //     std::vector<double> emissionCurve;
    //     std::vector<double> powerPattern;
    //
    //     if (!normalized && fabs(speed) <= ZERO_SPEED_ACCURACY) {
    //         if (pollutant == "FC") {
    //             return _idlingFC;
    //         } else {
    //             return _idlingValuesPollutants.get(pollutant);
    //         }
    //     } // end if
    //
    //     if (pollutant == "FC") {
    //         if (normalized) {
    //             emissionCurve = _normedCepCurveFC;
    //             powerPattern = _normalizedPowerPatternFC;
    //         } else {
    //             emissionCurve = _cepCurveFC;
    //             powerPattern = _powerPatternFC;
    //         }
    //     } else {
    //         if (!_cepCurvePollutants.hasString(pollutant)) {
    //             throw InvalidArgument("Emission pollutant " + pollutant + " not found!");
    //         }
    //
    //         if (normalized) {
    //             emissionCurve = _normalizedCepCurvePollutants.get(pollutant);
    //             powerPattern = _normailzedPowerPatternPollutants;
    //         } else {
    //             emissionCurve = _cepCurvePollutants.get(pollutant);
    //             powerPattern = _powerPatternPollutants;
    //         }
    //
    //     } // end if
    //
    //
    //
    //     if (emissionCurve.size() == 0) {
    //         throw InvalidArgument("Empty emission curve for " + pollutant + " found!");
    //     }
    //
    //     if (emissionCurve.size() == 1) {
    //         return emissionCurve[0];
    //     }
    //
    //     // in case that the demanded power is smaller than the first entry (smallest) in the power pattern the first two entries are extrapolated
    //     if (power <= powerPattern.front()) {
    //         double calcEmission =  PHEMCEP::Interpolate(power, powerPattern[0], powerPattern[1], emissionCurve[0], emissionCurve[1]);
    //
    //         if (calcEmission < 0) {
    //             return 0;
    //         } else {
    //             return calcEmission;
    //         }
    //
    //     } // end if
    //
    //     // if power bigger than all entries in power pattern the last two values are linearly extrapolated
    //     if (power >= powerPattern.back()) {
    //         return PHEMCEP::Interpolate(power, powerPattern[powerPattern.size() - 2], powerPattern.back(), emissionCurve[emissionCurve.size() - 2], emissionCurve.back());
    //     } // end if
    //
    //     // bisection search to find correct position in power pattern
    //     int upperIndex;
    //     int lowerIndex;
    //
    //     PHEMCEP::FindLowerUpperInPattern(lowerIndex, upperIndex, powerPattern, power);
    //
    //     return PHEMCEP::Interpolate(power, powerPattern[lowerIndex], powerPattern[upperIndex], emissionCurve[lowerIndex], emissionCurve[upperIndex]);
    //
    // }
    #[allow(unused_variables)]
    pub fn get_emission(&self, pollutantIdentifier: &str, power: f64, speed: f64, normalized: bool) -> f64 {
        let emissionCurve: Vec<f64>;
        let powerPattern: Vec<f64>;

        if !normalized && speed.abs() <= ZERO_SPEED_ACCURACY {
            if pollutantIdentifier == "FC" {
                return self._idlingFC;
            } else {
                return sb_get(&self._idlingValuesPollutants, pollutantIdentifier);
            }
        }

        if pollutantIdentifier == "FC" {
            if normalized {
                emissionCurve = self._normedCepCurveFC.clone();
                powerPattern = self._normalizedPowerPatternFC.clone();
            } else {
                emissionCurve = self._cepCurveFC.clone();
                powerPattern = self._powerPatternFC.clone();
            }
        } else {
            if !self._cepCurvePollutants.contains_key(pollutantIdentifier) {
                panic!("{}", ProcessError::InvalidArgument(format!("Emission pollutant {} not found!", pollutantIdentifier)));
            }

            if normalized {
                emissionCurve = sb_get(&self._normalizedCepCurvePollutants, pollutantIdentifier);
                powerPattern = self._normailzedPowerPatternPollutants.clone();
            } else {
                emissionCurve = sb_get(&self._cepCurvePollutants, pollutantIdentifier);
                powerPattern = self._powerPatternPollutants.clone();
            }
        }

        if emissionCurve.len() == 0 {
            panic!("{}", ProcessError::InvalidArgument(format!("Empty emission curve for {} found!", pollutantIdentifier)));
        }

        if emissionCurve.len() == 1 {
            return emissionCurve[0];
        }

        // in case that the demanded power is smaller than the first entry (smallest) in the power pattern the first two entries are extrapolated
        if power <= powerPattern[0] {
            let calcEmission = self.interpolate(power, powerPattern[0], powerPattern[1], emissionCurve[0], emissionCurve[1]);

            if calcEmission < 0.0 {
                return 0.0;
            } else {
                return calcEmission;
            }
        }

        // if power bigger than all entries in power pattern the last two values are linearly extrapolated
        let n = powerPattern.len();
        if power >= powerPattern[n - 1] {
            return self.interpolate(power, powerPattern[n - 2], powerPattern[n - 1], emissionCurve[emissionCurve.len() - 2], emissionCurve[emissionCurve.len() - 1]);
        }

        // bisection search to find correct position in power pattern
        let mut upperIndex: i32 = 0;
        let mut lowerIndex: i32 = 0;

        self.find_lower_upper_in_pattern(&mut lowerIndex, &mut upperIndex, &powerPattern, power);

        self.interpolate(
            power,
            powerPattern[lowerIndex as usize],
            powerPattern[upperIndex as usize],
            emissionCurve[lowerIndex as usize],
            emissionCurve[upperIndex as usize],
        )
    }

    /// C++: `double PHEMCEP::GetDecelCoast(double, double, double, double) const` [const]
    // double PHEMCEP::GetDecelCoast(double speed, double acc, double gradient, double /* vehicleLoading */) const {
    //     if (speed < SPEED_DCEL_MIN) {
    //         return speed / SPEED_DCEL_MIN * GetDecelCoast(SPEED_DCEL_MIN, acc, gradient, _vehicleLoading); // !!!vehicleLoading
    //     } // end if
    //
    //     double rotCoeff = GetRotationalCoeffecient(speed);
    //
    //     int upperIndex;
    //     int lowerIndex;
    //
    //     double iGear = GetGearCoeffecient(speed);
    //
    //     double iTot = iGear * _axleRatio;
    //
    //     double n = (30 * speed * iTot) / ((_effictiveWheelDiameter / 2) * M_PI2);
    //     double nNorm = (n - _engineIdlingSpeed) / (_engineRatedSpeed - _engineIdlingSpeed);
    //
    //     FindLowerUpperInPattern(lowerIndex, upperIndex, _nNormTable, nNorm);
    //
    //     double fMot = 0;
    //
    //     if (speed >= 10e-2) {
    //         fMot = (-GetDragCoeffecient(nNorm) * _ratedPower * 1000 / speed) / 0.9;
    //     } // end if
    //
    //     double fRoll = (_resistanceF0
    //                     + _resistanceF1 * speed
    //                     + pow(_resistanceF2 * speed, 2)
    //                     + pow(_resistanceF3 * speed, 3)
    //                     + pow(_resistanceF4 * speed, 4)) * (_massVehicle + _vehicleLoading) * GRAVITY_CONST; // !!!vehicleLoading
    //
    //     double fAir = _cdValue * _crossSectionalArea * 1.2 * 0.5 * pow(speed, 2);
    //
    //     double fGrad = (_massVehicle + _vehicleLoading) * GRAVITY_CONST * gradient / 100; // !!!vehicleLoading
    //
    //     return -(fMot + fRoll + fAir + fGrad) / ((_massVehicle + _vehicleLoading) * rotCoeff); // !!!vehicleLoading
    // }
    #[allow(unused_variables)]
    pub fn get_decel_coast(&self, speed: f64, acc: f64, gradient: f64, vehicleLoading: f64) -> f64 {
        let _ = vehicleLoading;
        if speed < SPEED_DCEL_MIN {
            return speed / SPEED_DCEL_MIN * self.get_decel_coast(SPEED_DCEL_MIN, acc, gradient, self._vehicleLoading); // !!!vehicleLoading
        }

        let rotCoeff = self.get_rotational_coeffecient(speed);

        let mut upperIndex: i32 = 0;
        let mut lowerIndex: i32 = 0;

        let iGear = self.get_gear_coeffecient(speed);

        let iTot = iGear * self._axleRatio;

        let n = (30.0 * speed * iTot) / ((self._effictiveWheelDiameter / 2.0) * M_PI2);
        let nNorm = (n - self._engineIdlingSpeed) / (self._engineRatedSpeed - self._engineIdlingSpeed);

        self.find_lower_upper_in_pattern(&mut lowerIndex, &mut upperIndex, &self._nNormTable, nNorm);

        let mut fMot = 0.0;

        if speed >= 10e-2 {
            fMot = (-self.get_drag_coeffecient(nNorm) * self._ratedPower * 1000.0 / speed) / 0.9;
        }

        let fRoll = (self._resistanceF0
            + self._resistanceF1 * speed
            + (self._resistanceF2 * speed).powf(2.0)
            + (self._resistanceF3 * speed).powf(3.0)
            + (self._resistanceF4 * speed).powf(4.0))
            * (self._massVehicle + self._vehicleLoading)
            * GRAVITY_CONST; // !!!vehicleLoading

        let fAir = self._cdValue * self._crossSectionalArea * 1.2 * 0.5 * speed.powf(2.0);

        let fGrad = (self._massVehicle + self._vehicleLoading) * GRAVITY_CONST * gradient / 100.0; // !!!vehicleLoading

        -(fMot + fRoll + fAir + fGrad) / ((self._massVehicle + self._vehicleLoading) * rotCoeff) // !!!vehicleLoading
    }

    /// C++: `SUMOEmissionClass PHEMCEP::GetEmissionClass() const` [const]
    /// Getter function to receive vehicle data from CEP
    // SUMOEmissionClass GetEmissionClass() const {
    //         return _emissionClass;
    //     }
    #[allow(unused_variables)]
    pub fn get_emission_class(&self) -> SUMOEmissionClass {
        self._emissionClass
    }

    /// C++: `double PHEMCEP::GetResistanceF0() const` [const]
    /// Getter function to receive vehicle data from CEP
    // double GetResistanceF0() const {
    //         return _resistanceF0;
    //     }
    #[allow(unused_variables)]
    pub fn get_resistance_f0(&self) -> f64 {
        self._resistanceF0
    }

    /// C++: `double PHEMCEP::GetResistanceF1() const` [const]
    /// Getter function to receive vehicle data from CEP
    // double GetResistanceF1() const {
    //         return _resistanceF1;
    //     }
    #[allow(unused_variables)]
    pub fn get_resistance_f1(&self) -> f64 {
        self._resistanceF1
    }

    /// C++: `double PHEMCEP::GetResistanceF2() const` [const]
    /// Getter function to receive vehicle data from CEP
    // double GetResistanceF2() const {
    //         return _resistanceF2;
    //     }
    #[allow(unused_variables)]
    pub fn get_resistance_f2(&self) -> f64 {
        self._resistanceF2
    }

    /// C++: `double PHEMCEP::GetResistanceF3() const` [const]
    /// Getter function to receive vehicle data from CEP
    // double GetResistanceF3() const {
    //         return _resistanceF3;
    //     }
    #[allow(unused_variables)]
    pub fn get_resistance_f3(&self) -> f64 {
        self._resistanceF3
    }

    /// C++: `double PHEMCEP::GetResistanceF4() const` [const]
    /// Getter function to receive vehicle data from CEP
    // double GetResistanceF4() const {
    //         return _resistanceF4;
    //     }
    #[allow(unused_variables)]
    pub fn get_resistance_f4(&self) -> f64 {
        self._resistanceF4
    }

    /// C++: `double PHEMCEP::GetCdValue() const` [const]
    /// Getter function to receive vehicle data from CEP
    // double GetCdValue() const {
    //         return _cdValue;
    //     }
    #[allow(unused_variables)]
    pub fn get_cd_value(&self) -> f64 {
        self._cdValue
    }

    /// C++: `double PHEMCEP::GetCrossSectionalArea() const` [const]
    /// Getter function to receive vehicle data from CEP
    // double GetCrossSectionalArea() const {
    //         return _crossSectionalArea;
    //     }
    #[allow(unused_variables)]
    pub fn get_cross_sectional_area(&self) -> f64 {
        self._crossSectionalArea
    }

    /// C++: `double PHEMCEP::GetMassVehicle() const` [const]
    /// Getter function to receive vehicle data from CEP
    // double GetMassVehicle() const {
    //         return _massVehicle;
    //     }
    #[allow(unused_variables)]
    pub fn get_mass_vehicle(&self) -> f64 {
        self._massVehicle
    }

    /// C++: `double PHEMCEP::GetVehicleLoading() const` [const]
    /// Getter function to receive vehicle data from CEP
    // double GetVehicleLoading() const {
    //         return _vehicleLoading;
    //     }
    #[allow(unused_variables)]
    pub fn get_vehicle_loading(&self) -> f64 {
        self._vehicleLoading
    }

    /// C++: `double PHEMCEP::GetMassRot() const` [const]
    /// Getter function to receive vehicle data from CEP
    // double GetMassRot() const {
    //         return _massRot;
    //     }
    #[allow(unused_variables)]
    pub fn get_mass_rot(&self) -> f64 {
        self._massRot
    }

    /// C++: `double PHEMCEP::GetRatedPower() const` [const]
    /// Getter function to receive vehicle data from CEP
    // double GetRatedPower() const {
    //         return _ratedPower;
    //     }
    #[allow(unused_variables)]
    pub fn get_rated_power(&self) -> f64 {
        self._ratedPower
    }

    /// C++: `const std::string & PHEMCEP::GetVehicleFuelType() const` [const]
    /// Getter function to receive vehicle data from CEP
    // const std::string& GetVehicleFuelType() const {
    //         return _vehicleFuelType;
    //     }
    #[allow(unused_variables)]
    pub fn get_vehicle_fuel_type(&self) -> &str {
        &self._vehicleFuelType
    }

    /// C++: `double PHEMCEP::Interpolate(double, double, double, double, double) const` [const, private]
    /// Interpolates emission linearly between two known power-emission pairs
    // double
    // PHEMCEP::Interpolate(double px, double p1, double p2, double e1, double e2) const {
    //     if (p2 == p1) {
    //         return e1;
    //     }
    //     return e1 + (px - p1) / (p2 - p1) * (e2 - e1);
    // }
    #[allow(unused_variables)]
    pub fn interpolate(&self, px: f64, p1: f64, p2: f64, e1: f64, e2: f64) -> f64 {
        if p2 == p1 {
            return e1;
        }
        e1 + (px - p1) / (p2 - p1) * (e2 - e1)
    }

    /// C++: `void PHEMCEP::FindLowerUpperInPattern(int &, int &, const std::vector<double> &, double) const` [const, private]
    /// Finds bounding upper and lower index in pattern for value
    // void PHEMCEP::FindLowerUpperInPattern(int& lowerIndex, int& upperIndex, const std::vector<double>& pattern, double value) const {
    //     if (value <= pattern.front()) {
    //         lowerIndex = 0;
    //         upperIndex = 0;
    //         return;
    //
    //     } // end if
    //
    //     if (value >= pattern.back()) {
    //         lowerIndex = (int)pattern.size() - 1;
    //         upperIndex = (int)pattern.size() - 1;
    //         return;
    //     } // end if
    //
    //     // bisection search to find correct position in power pattern
    //     int middleIndex = ((int)pattern.size() - 1) / 2;
    //     upperIndex = (int)pattern.size() - 1;
    //     lowerIndex = 0;
    //
    //     while (upperIndex - lowerIndex > 1) {
    //         if (pattern[middleIndex] == value) {
    //             lowerIndex = middleIndex;
    //             upperIndex = middleIndex;
    //             return;
    //         } else if (pattern[middleIndex] < value) {
    //             lowerIndex = middleIndex;
    //             middleIndex = (upperIndex - lowerIndex) / 2 + lowerIndex;
    //         } else {
    //             upperIndex = middleIndex;
    //             middleIndex = (upperIndex - lowerIndex) / 2 + lowerIndex;
    //         } // end if
    //     } // end while
    //
    //     if (pattern[lowerIndex] <= value && value < pattern[upperIndex]) {
    //         return;
    //     } else {
    //         throw ProcessError("Error during calculation of position in pattern!");
    //     }
    // }
    #[allow(unused_variables)]
    pub fn find_lower_upper_in_pattern(&self, lowerIndex: &mut i32, upperIndex: &mut i32, pattern: &Vec<f64>, value: f64) {
        if value <= pattern[0] {
            *lowerIndex = 0;
            *upperIndex = 0;
            return;
        }

        if value >= pattern[pattern.len() - 1] {
            *lowerIndex = pattern.len() as i32 - 1;
            *upperIndex = pattern.len() as i32 - 1;
            return;
        }

        // bisection search to find correct position in power pattern
        let mut middleIndex: i32 = (pattern.len() as i32 - 1) / 2;
        *upperIndex = pattern.len() as i32 - 1;
        *lowerIndex = 0;

        while *upperIndex - *lowerIndex > 1 {
            if pattern[middleIndex as usize] == value {
                *lowerIndex = middleIndex;
                *upperIndex = middleIndex;
                return;
            } else if pattern[middleIndex as usize] < value {
                *lowerIndex = middleIndex;
                middleIndex = (*upperIndex - *lowerIndex) / 2 + *lowerIndex;
            } else {
                *upperIndex = middleIndex;
                middleIndex = (*upperIndex - *lowerIndex) / 2 + *lowerIndex;
            }
        }

        if pattern[*lowerIndex as usize] <= value && value < pattern[*upperIndex as usize] {
            return;
        } else {
            panic!("{}", ProcessError::new("Error during calculation of position in pattern!"));
        }
    }

    /// C++: `double PHEMCEP::GetRotationalCoeffecient(double) const` [const, private]
    /// Calculates rotational index for speed
    // double
    // PHEMCEP::GetRotationalCoeffecient(double speed) const {
    //     int upperIndex;
    //     int lowerIndex;
    //
    //     PHEMCEP::FindLowerUpperInPattern(lowerIndex, upperIndex, _speedPatternRotational, speed);
    //
    //     return PHEMCEP::Interpolate(speed,
    //                                 _speedPatternRotational[lowerIndex],
    //                                 _speedPatternRotational[upperIndex],
    //                                 _speedCurveRotational[lowerIndex],
    //                                 _speedCurveRotational[upperIndex]);
    // }
    #[allow(unused_variables)]
    pub fn get_rotational_coeffecient(&self, speed: f64) -> f64 {
        let mut upperIndex: i32 = 0;
        let mut lowerIndex: i32 = 0;

        self.find_lower_upper_in_pattern(&mut lowerIndex, &mut upperIndex, &self._speedPatternRotational, speed);

        self.interpolate(
            speed,
            self._speedPatternRotational[lowerIndex as usize],
            self._speedPatternRotational[upperIndex as usize],
            self._speedCurveRotational[lowerIndex as usize],
            self._speedCurveRotational[upperIndex as usize],
        )
    }

    /// C++: `double PHEMCEP::GetGearCoeffecient(double) const` [const, private]
    // double PHEMCEP::GetGearCoeffecient(double speed) const {
    //     int upperIndex;
    //     int lowerIndex;
    //
    //     FindLowerUpperInPattern(lowerIndex, upperIndex, _gearTransmissionCurve, speed);
    //
    //     return Interpolate(speed,
    //                        _speedPatternRotational[lowerIndex],
    //                        _speedPatternRotational[upperIndex],
    //                        _gearTransmissionCurve[lowerIndex],
    //                        _gearTransmissionCurve[upperIndex]);
    // }
    #[allow(unused_variables)]
    pub fn get_gear_coeffecient(&self, speed: f64) -> f64 {
        let mut upperIndex: i32 = 0;
        let mut lowerIndex: i32 = 0;

        self.find_lower_upper_in_pattern(&mut lowerIndex, &mut upperIndex, &self._gearTransmissionCurve, speed);

        self.interpolate(
            speed,
            self._speedPatternRotational[lowerIndex as usize],
            self._speedPatternRotational[upperIndex as usize],
            self._gearTransmissionCurve[lowerIndex as usize],
            self._gearTransmissionCurve[upperIndex as usize],
        )
    }

    /// C++: `double PHEMCEP::GetDragCoeffecient(double) const` [const, private]
    // double PHEMCEP::GetDragCoeffecient(double nNorm)  const {
    //     int upperIndex;
    //     int lowerIndex;
    //
    //     FindLowerUpperInPattern(lowerIndex, upperIndex, _dragNormTable, nNorm);
    //
    //     return Interpolate(nNorm,
    //                        _nNormTable[lowerIndex],
    //                        _nNormTable[upperIndex],
    //                        _dragNormTable[lowerIndex],
    //                        _dragNormTable[upperIndex]);
    // }
    #[allow(unused_variables)]
    pub fn get_drag_coeffecient(&self, nNorm: f64) -> f64 {
        let mut upperIndex: i32 = 0;
        let mut lowerIndex: i32 = 0;

        self.find_lower_upper_in_pattern(&mut lowerIndex, &mut upperIndex, &self._dragNormTable, nNorm);

        self.interpolate(
            nNorm,
            self._nNormTable[lowerIndex as usize],
            self._nNormTable[upperIndex as usize],
            self._dragNormTable[lowerIndex as usize],
            self._dragNormTable[upperIndex as usize],
        )
    }

    /// C++: `double PHEMCEP::GetPMaxNorm(double) const` [const, private]
    /// Calculates maximum available rated power for speed
    // double PHEMCEP::GetPMaxNorm(double speed) const {
    //     // Linear function between v0 and v1, constant elsewhere
    //     if (speed <= _pNormV0) {
    //         return _pNormP0;
    //     } else if (speed >= _pNormV1) {
    //         return _pNormP1;
    //     } else {
    //         return PHEMCEP::Interpolate(speed, _pNormV0, _pNormV1, _pNormP0, _pNormP1);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_p_max_norm(&self, speed: f64) -> f64 {
        // Linear function between v0 and v1, constant elsewhere
        if speed <= self._pNormV0 {
            self._pNormP0
        } else if speed >= self._pNormV1 {
            self._pNormP1
        } else {
            self.interpolate(speed, self._pNormV0, self._pNormV1, self._pNormP0, self._pNormP1)
        }
    }

}

