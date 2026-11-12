//! Port of utils/emissions/PHEMCEPHandler.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// std::getline over a whole file: the lines without their '\n', no empty element after a final newline
fn phem_getlines(s: &str) -> Vec<String> {
    if s.is_empty() {
        return Vec::new();
    }
    let mut v: Vec<String> = s.split('\n').map(str::to_string).collect();
    if s.ends_with('\n') {
        v.pop();
    }
    v
}
/// std::getline(stream, cell, ',') repeated: a trailing ',' does not produce an empty cell
fn phem_getline_split(line: &str) -> Vec<String> {
    if line.is_empty() {
        return Vec::new();
    }
    let mut v: Vec<String> = line.split(',').map(str::to_string).collect();
    if line.ends_with(',') {
        v.pop();
    }
    v
}
/// std::istringstream(cell) >> double: leading whitespace skipped, prefix parsed, 0 on failure (C++11)
fn phem_istream_f64(cell: &str) -> f64 {
    let (v, end, _range) = utils::common::string_utils::strtod(cell);
    if end == 0 { 0.0 } else { v }
}
thread_local! {
    // C++ static PHEMCEPHandler instance of getHandlerInstance()
    static PHEMCEPHANDLER_INSTANCE: std::cell::Cell<Ptr<PHEMCEPHandler>> = const { std::cell::Cell::new(Ptr::null()) };
}

/// Data Handler for all CEP emission and vehicle Data
/// C++ class `PHEMCEPHandler` (utils/emissions/PHEMCEPHandler.h)
pub struct PHEMCEPHandler {
    /// bijection between PHEMEmissionClass and CEPs
    pub _ceps: BTreeMap<SUMOEmissionClass, Ptr<PHEMCEP>>,
}

impl PHEMCEPHandler {
    /// C++: `void PHEMCEPHandler::~PHEMCEPHandler()`
    /// Destructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // the C++ destructor is implicit: the map of CEPs is not owned here, as in C++
    }

    /// C++: `PHEMCEPHandler & PHEMCEPHandler::getHandlerInstance()` [static]
    /// Implementatio of Singelton pattern
    // PHEMCEPHandler&
    // PHEMCEPHandler::getHandlerInstance() {
    //     static PHEMCEPHandler instance;
    //     return instance;
    // }
    #[allow(unused_variables)]
    pub fn get_handler_instance() -> &'static mut PHEMCEPHandler {
        // shortcut: the singleton is per thread (thread_local) and leaked like the C++ static, upgrade if it is used from several threads
        let p = PHEMCEPHANDLER_INSTANCE.with(|cell| {
            if cell.get().is_null() {
                cell.set(Ptr::from_mut(Box::leak(Box::new(PHEMCEPHandler::new()))));
            }
            cell.get()
        });
        unsafe { &mut *p.as_raw() }
    }

    /// C++: `PHEMCEP * PHEMCEPHandler::GetCep(SUMOEmissionClass)`
    /// Returns the CEP data for a PHEM emission class
    // PHEMCEP*
    // PHEMCEPHandler::GetCep(SUMOEmissionClass emissionClass) {
    //     // check if Cep has been loaded
    //     if (_ceps.find(emissionClass) == _ceps.end()) {
    //         return nullptr;
    //     } // end if
    //
    //     return _ceps[emissionClass];
    // }
    #[allow(unused_variables)]
    pub fn get_cep(&mut self, emissionClass: SUMOEmissionClass) -> Ptr<PHEMCEP> {
        // check if Cep has been loaded
        if !self._ceps.contains_key(&emissionClass) {
            return Ptr::null();
        }

        self._ceps[&emissionClass]
    }

    /// C++: `bool PHEMCEPHandler::Load(SUMOEmissionClass, const std::string &)`
    /// Helper method to load CEP and vehicle files from file system
    // bool
    // PHEMCEPHandler::Load(SUMOEmissionClass emissionClass, const std::string& emissionClassIdentifier) {
    //     // to hold everything.
    //     std::vector< std::vector<double> > matrixSpeedInertiaTable;
    //     std::vector< std::vector<double> > normedDragTable;
    //     std::vector< std::vector<double> > matrixFC;
    //     std::vector< std::vector<double> > matrixPollutants;
    //     std::vector<std::string> headerFC;
    //     std::vector<std::string> headerPollutants;
    //     std::vector<double> idlingValues;
    //     std::vector<double> idlingValuesFC;
    //
    //     double vehicleMass;
    //     double vehicleLoading;
    //     double vehicleMassRot;
    //     double crosssectionalArea;
    //     double cwValue;
    //     double f0;
    //     double f1;
    //     double f2;
    //     double f3;
    //     double f4;
    //     double axleRatio;
    //     double ratedPower;
    //     double engineIdlingSpeed;
    //     double engineRatedSpeed;
    //     double effectiveWheelDiameter;
    //     std::string vehicleMassType;
    //     std::string vehicleFuelType;
    //     double pNormV0;
    //     double pNormP0;
    //     double pNormV1;
    //     double pNormP1;
    //
    //     OptionsCont& oc = OptionsCont::getOptions();
    //     //std::string phemPath = oc.getString("phemlight-path") + "/";
    //     std::vector<std::string> phemPath;
    //     phemPath.push_back(oc.getString("phemlight-path") + "/");
    //     if (getenv("PHEMLIGHT_PATH") != nullptr) {
    //         phemPath.push_back(std::string(getenv("PHEMLIGHT_PATH")) + "/");
    //     }
    //     if (getenv("SUMO_HOME") != nullptr) {
    //         phemPath.push_back(std::string(getenv("SUMO_HOME")) + "/data/emissions/PHEMlight/");
    //     }
    //     if (!ReadVehicleFile(phemPath, emissionClassIdentifier,
    //                          vehicleMass,
    //                          vehicleLoading,
    //                          vehicleMassRot,
    //                          crosssectionalArea,
    //                          cwValue,
    //                          f0,
    //                          f1,
    //                          f2,
    //                          f3,
    //                          f4,
    //                          axleRatio,
    //                          ratedPower,
    //                          engineIdlingSpeed,
    //                          engineRatedSpeed,
    //                          effectiveWheelDiameter,
    //                          vehicleMassType,
    //                          vehicleFuelType,
    //                          pNormV0,
    //                          pNormP0,
    //                          pNormV1,
    //                          pNormP1,
    //                          matrixSpeedInertiaTable,
    //                          normedDragTable)) {
    //         return false;
    //     }
    //
    //     if (!ReadEmissionData(true, phemPath, emissionClassIdentifier, headerFC, matrixFC, idlingValuesFC)) {
    //         return false;
    //     }
    //
    //     if (!ReadEmissionData(false, phemPath, emissionClassIdentifier, headerPollutants, matrixPollutants, idlingValues)) {
    //         return false;
    //     }
    //
    //     _ceps[emissionClass] = new PHEMCEP(vehicleMassType == "HV",
    //                                        emissionClass, emissionClassIdentifier,
    //                                        vehicleMass,
    //                                        vehicleLoading,
    //                                        vehicleMassRot,
    //                                        crosssectionalArea,
    //                                        cwValue,
    //                                        f0,
    //                                        f1,
    //                                        f2,
    //                                        f3,
    //                                        f4,
    //                                        ratedPower,
    //                                        pNormV0,
    //                                        pNormP0,
    //                                        pNormV1,
    //                                        pNormP1,
    //                                        axleRatio,
    //                                        engineIdlingSpeed,
    //                                        engineRatedSpeed,
    //                                        effectiveWheelDiameter,
    //                                        idlingValuesFC.front(),
    //                                        vehicleFuelType,
    //                                        matrixFC,
    //                                        headerPollutants,
    //                                        matrixPollutants,
    //                                        matrixSpeedInertiaTable,
    //                                        normedDragTable,
    //                                        idlingValues);
    //
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn load(&mut self, emissionClass: SUMOEmissionClass, emissionClassIdentifier: &str) -> bool {
        // to hold everything.
        let mut matrixSpeedInertiaTable: Vec<Vec<f64>> = Vec::new();
        let mut normedDragTable: Vec<Vec<f64>> = Vec::new();
        let mut matrixFC: Vec<Vec<f64>> = Vec::new();
        let mut matrixPollutants: Vec<Vec<f64>> = Vec::new();
        let mut headerFC: Vec<String> = Vec::new();
        let mut headerPollutants: Vec<String> = Vec::new();
        let mut idlingValues: Vec<f64> = Vec::new();
        let mut idlingValuesFC: Vec<f64> = Vec::new();

        let mut vehicleMass: f64 = 0.0;
        let mut vehicleLoading: f64 = 0.0;
        let mut vehicleMassRot: f64 = 0.0;
        let mut crosssectionalArea: f64 = 0.0;
        let mut cwValue: f64 = 0.0;
        let mut f0: f64 = 0.0;
        let mut f1: f64 = 0.0;
        let mut f2: f64 = 0.0;
        let mut f3: f64 = 0.0;
        let mut f4: f64 = 0.0;
        let mut axleRatio: f64 = 0.0;
        let mut ratedPower: f64 = 0.0;
        let mut engineIdlingSpeed: f64 = 0.0;
        let mut engineRatedSpeed: f64 = 0.0;
        let mut effectiveWheelDiameter: f64 = 0.0;
        let mut vehicleMassType = String::new();
        let mut vehicleFuelType = String::new();
        let mut pNormV0: f64 = 0.0;
        let mut pNormP0: f64 = 0.0;
        let mut pNormV1: f64 = 0.0;
        let mut pNormP1: f64 = 0.0;

        // C++ OptionsCont& oc = OptionsCont::getOptions();
        let mut phemPath: Vec<String> = Vec::new();
        phemPath.push(format!("{}/", utils::options::options_cont::get_string("phemlight-path")));
        if let Ok(v) = std::env::var("PHEMLIGHT_PATH") {
            phemPath.push(format!("{}/", v));
        }
        if let Ok(v) = std::env::var("SUMO_HOME") {
            phemPath.push(format!("{}/data/emissions/PHEMlight/", v));
        }
        if !self.read_vehicle_file(
            &phemPath,
            emissionClassIdentifier,
            &mut vehicleMass,
            &mut vehicleLoading,
            &mut vehicleMassRot,
            &mut crosssectionalArea,
            &mut cwValue,
            &mut f0,
            &mut f1,
            &mut f2,
            &mut f3,
            &mut f4,
            &mut axleRatio,
            &mut ratedPower,
            &mut engineIdlingSpeed,
            &mut engineRatedSpeed,
            &mut effectiveWheelDiameter,
            &mut vehicleMassType,
            &mut vehicleFuelType,
            &mut pNormV0,
            &mut pNormP0,
            &mut pNormV1,
            &mut pNormP1,
            &mut matrixSpeedInertiaTable,
            &mut normedDragTable,
        ) {
            return false;
        }

        if !self.read_emission_data(true, &phemPath, emissionClassIdentifier, &mut headerFC, &mut matrixFC, &mut idlingValuesFC) {
            return false;
        }

        if !self.read_emission_data(false, &phemPath, emissionClassIdentifier, &mut headerPollutants, &mut matrixPollutants, &mut idlingValues) {
            return false;
        }

        let cep = PHEMCEP::new(
            vehicleMassType == "HV",
            emissionClass,
            emissionClassIdentifier,
            vehicleMass,
            vehicleLoading,
            vehicleMassRot,
            crosssectionalArea,
            cwValue,
            f0,
            f1,
            f2,
            f3,
            f4,
            ratedPower,
            pNormV0,
            pNormP0,
            pNormV1,
            pNormP1,
            axleRatio,
            engineIdlingSpeed,
            engineRatedSpeed,
            effectiveWheelDiameter,
            idlingValuesFC[0],
            &vehicleFuelType,
            &matrixFC,
            &headerPollutants,
            &matrixPollutants,
            &matrixSpeedInertiaTable,
            &normedDragTable,
            &idlingValues,
        );
        self._ceps.insert(emissionClass, Ptr::from_raw(Box::into_raw(Box::new(cep))));

        true
    }

    /// C++: `bool PHEMCEPHandler::ReadVehicleFile(const std::vector<std::string> &, const std::string &, double &, double &, double &, double &, double &, double &, double &, double &, double &, double &, double &, double &, double &, double &, double &, std::string &, std::string &, double &, double &, double &, double &, std::vector<std::vector<double>> &, std::vector<std::vector<double>> &)` [private]
    /// Helper method to read a vehicle file from file system
    // bool
    // PHEMCEPHandler::ReadVehicleFile(const std::vector<std::string>& path, const std::string& emissionClass,
    //                                 double& vehicleMass,
    //                                 double& vehicleLoading,
    //                                 double& vehicleMassRot,
    //                                 double& crossArea,
    //                                 double& cWValue,
    //                                 double& f0,
    //                                 double& f1,
    //                                 double& f2,
    //                                 double& f3,
    //                                 double& f4,
    //                                 double& axleRatio,
    //                                 double& ratedPower,
    //                                 double& engineIdlingSpeed,
    //                                 double& engineRatedSpeed,
    //                                 double& effectiveWheelDiameter,
    //                                 std::string& vehicleMassType,
    //                                 std::string& vehicleFuelType,
    //                                 double& pNormV0,
    //                                 double& pNormP0,
    //                                 double& pNormV1,
    //                                 double& pNormP1,
    //                                 std::vector< std::vector<double> >& matrixSpeedInertiaTable,
    //                                 std::vector< std::vector<double> >& normedDragTable)
    //
    // {
    //     std::ifstream fileVehicle;
    //     for (std::vector<std::string>::const_iterator i = path.begin(); i != path.end(); i++) {
    //         fileVehicle.open(((*i) + emissionClass + ".PHEMLight.veh").c_str());
    //         if (fileVehicle.good()) {
    //             break;
    //         }
    //     }
    //     if (!fileVehicle.good()) {
    //         return false;
    //     }
    //
    //     std::string line;
    //     std::string cell;
    //     std::string commentPrefix = "c";
    //     int dataCount = 0;
    //
    //     // skip header
    //     std::getline(fileVehicle, line);
    //
    //     while (std::getline(fileVehicle, line) && dataCount <= 49) {
    //         // EOL handling for Linux
    //         if (line.size() > 0 && line.substr(line.size() - 1) == "\r") {
    //             line = line.substr(0, line.size() - 1);
    //         }
    //
    //         std::stringstream  lineStream(line);
    //
    //         if (line.substr(0, 1) == commentPrefix) {
    //             continue;
    //         } else {
    //             dataCount++;
    //         }
    //
    //         std::getline(lineStream, cell, ',');
    //
    //         // reading Mass
    //         if (dataCount == 1) {
    //             std::istringstream(cell) >> vehicleMass;
    //         }
    //
    //         // reading vehicle loading
    //         if (dataCount == 2) {
    //             std::istringstream(cell) >> vehicleLoading;
    //         }
    //
    //         // reading cWValue
    //         if (dataCount == 3) {
    //             std::istringstream(cell) >> cWValue;
    //         }
    //
    //         // reading crossectional area
    //         if (dataCount == 4) {
    //             std::istringstream(cell) >> crossArea;
    //         }
    //
    //         // reading vehicle mass rotational
    //         if (dataCount == 7) {
    //             std::istringstream(cell) >> vehicleMassRot;
    //         }
    //
    //         // reading rated power
    //         if (dataCount == 10) {
    //             std::istringstream(cell) >> ratedPower;
    //         }
    //
    //         // reading engine rated speed
    //         if (dataCount == 11) {
    //             std::istringstream(cell) >> engineRatedSpeed;
    //         }
    //
    //         // reading engine idling speed
    //         if (dataCount == 12) {
    //             std::istringstream(cell) >> engineIdlingSpeed;
    //         }
    //
    //         // reading f0
    //         if (dataCount == 14) {
    //             std::istringstream(cell) >> f0;
    //         }
    //
    //         // reading f1
    //         if (dataCount == 15) {
    //             std::istringstream(cell) >> f1;
    //         }
    //
    //         // reading f2
    //         if (dataCount == 16) {
    //             std::istringstream(cell) >> f2;
    //         }
    //
    //         // reading f3
    //         if (dataCount == 17) {
    //             std::istringstream(cell) >> f3;
    //         }
    //
    //         // reading f4
    //         if (dataCount == 18) {
    //             std::istringstream(cell) >> f4;
    //         }
    //         // reading axleRatio
    //         if (dataCount == 21) {
    //             std::istringstream(cell) >> axleRatio;
    //         }
    //
    //         // reading effective wheel diameter
    //         if (dataCount == 22) {
    //             std::istringstream(cell) >> effectiveWheelDiameter;
    //         }
    //
    //         // reading vehicleMassType
    //         if (dataCount == 45) {
    //             vehicleMassType = cell;
    //         }
    //
    //         // reading vehicleFuelType
    //         if (dataCount == 46) {
    //             vehicleFuelType = cell;
    //         }
    //
    //         // reading pNormV0
    //         if (dataCount == 47) {
    //             std::istringstream(cell) >> pNormV0;
    //         }
    //
    //         // reading pNormP0
    //         if (dataCount == 48) {
    //             std::istringstream(cell) >> pNormP0;
    //         }
    //
    //         // reading pNormV1
    //         if (dataCount == 49) {
    //             std::istringstream(cell) >> pNormV1;
    //         }
    //
    //         // reading pNormP1
    //         if (dataCount == 50) {
    //             std::istringstream(cell) >> pNormP1;
    //         }
    //     } // end while
    //
    //     while (std::getline(fileVehicle, line) && line.substr(0, 1) != commentPrefix) {
    //         std::stringstream lineStream(line);
    //         std::vector<double> vi;
    //         while (std::getline(lineStream, cell, ',')) {
    //             double entry;
    //             std::istringstream(cell) >> entry;
    //             vi.push_back(entry);
    //
    //         } // end while
    //         matrixSpeedInertiaTable.push_back(vi);
    //     } // end while
    //
    //     while (std::getline(fileVehicle, line)) {
    //         if (line.substr(0, 1) == commentPrefix) {
    //             continue;
    //         }
    //
    //         std::stringstream  lineStream(line);
    //         std::vector<double> vi;
    //         while (std::getline(lineStream, cell, ',')) {
    //             double entry;
    //             std::istringstream(cell) >> entry;
    //             vi.push_back(entry);
    //
    //         } // end while
    //         normedDragTable.push_back(vi);
    //     } // end while
    //
    //
    //     fileVehicle.close();
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn read_vehicle_file(&mut self, path: &Vec<String>, emissionClass: &str, vehicleMass: &mut f64, vehicleLoading: &mut f64, vehicleMassRot: &mut f64, crossArea: &mut f64, cWValue: &mut f64, f0: &mut f64, f1: &mut f64, f2: &mut f64, f3: &mut f64, f4: &mut f64, axleRatio: &mut f64, ratedPower: &mut f64, engineIdlingSpeed: &mut f64, engineRatedSpeed: &mut f64, effectiveWheelDiameter: &mut f64, vehicleMassType: &mut String, vehicleFuelType: &mut String, pNormV0: &mut f64, pNormP0: &mut f64, pNormV1: &mut f64, pNormP1: &mut f64, matrixSpeedInertiaTable: &mut Vec<Vec<f64>>, normedDragTable: &mut Vec<Vec<f64>>) -> bool {
        let mut content: Option<String> = None;
        for p in path.iter() {
            if let Ok(bytes) = std::fs::read(format!("{}{}.PHEMLight.veh", p, emissionClass)) {
                content = Some(String::from_utf8_lossy(&bytes).into_owned());
                break;
            }
        }
        let Some(content) = content else {
            return false;
        };

        let mut lines = phem_getlines(&content).into_iter();
        let commentPrefix = "c";
        let mut dataCount: i32 = 0;
        let mut cell = String::new();

        // skip header
        lines.next();

        loop {
            // std::getline is evaluated before the dataCount check, so the line that ends the loop is consumed
            let Some(mut line) = lines.next() else {
                break;
            };
            if dataCount > 49 {
                break;
            }
            // EOL handling for Linux
            if line.ends_with('\r') {
                line.pop();
            }

            if line.starts_with(commentPrefix) {
                continue;
            } else {
                dataCount += 1;
            }

            // a getline on an empty line leaves the previous cell in place
            if let Some(first) = phem_getline_split(&line).into_iter().next() {
                cell = first;
            }

            // reading Mass
            if dataCount == 1 {
                *vehicleMass = phem_istream_f64(&cell);
            }
            // reading vehicle loading
            if dataCount == 2 {
                *vehicleLoading = phem_istream_f64(&cell);
            }
            // reading cWValue
            if dataCount == 3 {
                *cWValue = phem_istream_f64(&cell);
            }
            // reading crossectional area
            if dataCount == 4 {
                *crossArea = phem_istream_f64(&cell);
            }
            // reading vehicle mass rotational
            if dataCount == 7 {
                *vehicleMassRot = phem_istream_f64(&cell);
            }
            // reading rated power
            if dataCount == 10 {
                *ratedPower = phem_istream_f64(&cell);
            }
            // reading engine rated speed
            if dataCount == 11 {
                *engineRatedSpeed = phem_istream_f64(&cell);
            }
            // reading engine idling speed
            if dataCount == 12 {
                *engineIdlingSpeed = phem_istream_f64(&cell);
            }
            // reading f0
            if dataCount == 14 {
                *f0 = phem_istream_f64(&cell);
            }
            // reading f1
            if dataCount == 15 {
                *f1 = phem_istream_f64(&cell);
            }
            // reading f2
            if dataCount == 16 {
                *f2 = phem_istream_f64(&cell);
            }
            // reading f3
            if dataCount == 17 {
                *f3 = phem_istream_f64(&cell);
            }
            // reading f4
            if dataCount == 18 {
                *f4 = phem_istream_f64(&cell);
            }
            // reading axleRatio
            if dataCount == 21 {
                *axleRatio = phem_istream_f64(&cell);
            }
            // reading effective wheel diameter
            if dataCount == 22 {
                *effectiveWheelDiameter = phem_istream_f64(&cell);
            }
            // reading vehicleMassType
            if dataCount == 45 {
                *vehicleMassType = cell.clone();
            }
            // reading vehicleFuelType
            if dataCount == 46 {
                *vehicleFuelType = cell.clone();
            }
            // reading pNormV0
            if dataCount == 47 {
                *pNormV0 = phem_istream_f64(&cell);
            }
            // reading pNormP0
            if dataCount == 48 {
                *pNormP0 = phem_istream_f64(&cell);
            }
            // reading pNormV1
            if dataCount == 49 {
                *pNormV1 = phem_istream_f64(&cell);
            }
            // reading pNormP1
            if dataCount == 50 {
                *pNormP1 = phem_istream_f64(&cell);
            }
        }

        loop {
            let Some(line) = lines.next() else {
                break;
            };
            if line.starts_with(commentPrefix) {
                break;
            }
            let vi: Vec<f64> = phem_getline_split(&line).iter().map(|c| phem_istream_f64(c)).collect();
            matrixSpeedInertiaTable.push(vi);
        }

        for line in lines {
            if line.starts_with(commentPrefix) {
                continue;
            }
            let vi: Vec<f64> = phem_getline_split(&line).iter().map(|c| phem_istream_f64(c)).collect();
            normedDragTable.push(vi);
        }

        true
    }

    /// C++: `bool PHEMCEPHandler::ReadEmissionData(bool, const std::vector<std::string> &, const std::string &, std::vector<std::string> &, std::vector<std::vector<double>> &, std::vector<double> &)` [private]
    /// Helper method to read a CEP file from file system
    // bool PHEMCEPHandler::ReadEmissionData(bool readFC, const std::vector<std::string>& path, const std::string& emissionClass,
    //                                       std::vector<std::string>& header, std::vector<std::vector<double> >& matrix, std::vector<double>& idlingValues) {
    //
    //     std::string pollutantExtension = "";
    //     if (readFC) {
    //         pollutantExtension += "_FC";
    //     }
    //     // declare file stream
    //     std::ifstream fileEmission;
    //     for (std::vector<std::string>::const_iterator i = path.begin(); i != path.end(); i++) {
    //         fileEmission.open(((*i) + emissionClass + pollutantExtension + ".csv").c_str());
    //         if (fileEmission.good()) {
    //             break;
    //         }
    //     }
    //
    //     if (!fileEmission.good()) {
    //         return false;
    //     }
    //
    //     std::string line;
    //     std::string cell;
    //     // read header line for pollutant identifiers
    //     if (std::getline(fileEmission, line)) {
    //         std::stringstream  lineStream(line);
    //
    //         // skip first entry "Pe"
    //         std::getline(lineStream, cell, ',');
    //
    //         while (std::getline(lineStream, cell, ',')) {
    //             header.push_back(cell);
    //         } // end while
    //
    //     } // end if
    //
    //     // skip units
    //     std::getline(fileEmission, line);
    //
    //     // skip comments
    //     std::getline(fileEmission, line);
    //
    //     // reading idlingValues
    //     std::getline(fileEmission, line);
    //
    //     std::stringstream idlingStream(line);
    //     std::string idlingCell;
    //
    //     //skipping idle comment
    //     std::getline(idlingStream, idlingCell, ',');
    //
    //     while (std::getline(idlingStream, idlingCell, ',')) {
    //         double entry;
    //         std::istringstream(idlingCell) >> entry;
    //         idlingValues.push_back(entry);
    //     } // end while
    //
    //     while (std::getline(fileEmission, line)) {
    //         std::stringstream  lineStream(line);
    //         std::vector <double> vi;
    //         while (std::getline(lineStream, cell, ',')) {
    //             double entry;
    //             std::istringstream(cell) >> entry;
    //             vi.push_back(entry);
    //
    //         } // end while
    //         matrix.push_back(vi);
    //     } // end while
    //
    //     fileEmission.close();
    //
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn read_emission_data(&mut self, readFC: bool, path: &Vec<String>, emissionClass: &str, header: &mut Vec<String>, matrix: &mut Vec<Vec<f64>>, idlingValues: &mut Vec<f64>) -> bool {
        let mut pollutantExtension = String::new();
        if readFC {
            pollutantExtension += "_FC";
        }
        // declare file stream
        let mut content: Option<String> = None;
        for p in path.iter() {
            if let Ok(bytes) = std::fs::read(format!("{}{}{}.csv", p, emissionClass, pollutantExtension)) {
                content = Some(String::from_utf8_lossy(&bytes).into_owned());
                break;
            }
        }
        let Some(content) = content else {
            return false;
        };

        let mut lines = phem_getlines(&content).into_iter();
        // read header line for pollutant identifiers
        if let Some(line) = lines.next() {
            // skip first entry "Pe"
            for c in phem_getline_split(&line).into_iter().skip(1) {
                header.push(c);
            }
        }

        // skip units
        lines.next();

        // skip comments
        lines.next();

        // reading idlingValues
        let line = lines.next().unwrap_or_default();
        // skipping idle comment
        for c in phem_getline_split(&line).into_iter().skip(1) {
            idlingValues.push(phem_istream_f64(&c));
        }

        for line in lines {
            let vi: Vec<f64> = phem_getline_split(&line).iter().map(|c| phem_istream_f64(c)).collect();
            matrix.push(vi);
        }

        true
    }

    /// C++: `void PHEMCEPHandler::PHEMCEPHandler()` [private]
    /// Implementation of Singelton pattern private (copy) constructor and =operator to avoid more than one instances
    // PHEMCEPHandler::PHEMCEPHandler() {
    // }
    #[allow(unused_variables)]
    pub fn new() -> Self {
        PHEMCEPHandler { _ceps: BTreeMap::new() }
    }

    /// C++: `void PHEMCEPHandler::PHEMCEPHandler(const PHEMCEPHandler &)` [private]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &PHEMCEPHandler) -> Self {
        // private and never defined in C++ (invalidated), so it cannot be called there either
        panic!("PHEMCEPHandler copy constructor is invalidated")
    }

    /// C++: `void PHEMCEPHandler::operator=(const PHEMCEPHandler &)` [private]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &PHEMCEPHandler) {
        // private and never defined in C++ (invalidated), so it cannot be called there either
        panic!("PHEMCEPHandler operator= is invalidated")
    }

}

