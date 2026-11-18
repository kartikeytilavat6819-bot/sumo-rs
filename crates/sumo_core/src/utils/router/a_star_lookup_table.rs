//! Port of utils/router/AStarLookupTable.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use crate::utils::router::router_edge::RouterVehicle;
use utils::common::msg_handler::{write_message, write_warning};
use utils::common::to_string::format_g;
use utils::common::util_exceptions::ProcessError;
use crate::utils::router::router_edge::RouterEdge;

/// Computes the shortest path through a network using the A* algorithm.
/// C++ class `AbstractLookupTable` (utils/router/AStarLookupTable.h)
pub struct AbstractLookupTable<E, V> {
    _p: std::marker::PhantomData<(E, V,)>,
}

impl<E, V> AbstractLookupTable<E, V> {
    /// C++: `void AbstractLookupTable::~AbstractLookupTable<E, V>()` [virtual]
    // virtual ~AbstractLookupTable() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
    }

    /// C++: `double AbstractLookupTable::lowerBound(const E *, const E *, double, double, double, double) const` [const, virtual, pure virtual]
    /// provide a lower bound on the distance between from and to (excluding traveltime of both edges)
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn lower_bound(&self, from: Ptr<E>, to: Ptr<E>, speed: f64, speedFactor: f64, fromEffort: f64, toEffort: f64) -> f64 {
        panic!("pure virtual AbstractLookupTable::lowerBound called: only FullLookupTable and LandmarkLookupTable are instantiated")
    }

    /// C++: `bool AbstractLookupTable::consistent() const` [const, virtual, pure virtual]
    /// whether the heuristic ist consistent (found nodes are always visited on the shortest path the first time)
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn consistent(&self) -> bool {
        panic!("pure virtual AbstractLookupTable::consistent called: only FullLookupTable and LandmarkLookupTable are instantiated")
    }

}

/// C++ class `FullLookupTable` (utils/router/AStarLookupTable.h)
#[repr(C)]
pub struct FullLookupTable<E, V> {
    /// C++ base class `AbstractLookupTable<E, V>`
    pub base: AbstractLookupTable<E, V>,
    pub myTable: Vec<Vec<f64>>,
    _p: std::marker::PhantomData<(E, V,)>,
}

impl<E, V> std::ops::Deref for FullLookupTable<E, V> {
    type Target = AbstractLookupTable<E, V>;
    fn deref(&self) -> &AbstractLookupTable<E, V> {
        &self.base
    }
}
impl<E, V> std::ops::DerefMut for FullLookupTable<E, V> {
    fn deref_mut(&mut self) -> &mut AbstractLookupTable<E, V> {
        &mut self.base
    }
}

impl<E, V> FullLookupTable<E, V> {
    /// C++: `void FullLookupTable::FullLookupTable<E, V>(const std::string &, const int)`
    // FullLookupTable(const std::string& filename, const int size) :
    //         myTable(size) {
    //         std::ifstream strm(filename.c_str());
    //         for (int i = 0; i < size; i++) {
    //             for (int j = 0; j < size; j++) {
    //                 double val;
    //                 strm >> val;
    //                 myTable[i].push_back(val);
    //             }
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn new(filename: &str, size: i32) -> Self {
        let mut myTable: Vec<Vec<f64>> = vec![Vec::new(); size as usize];
        let text = std::fs::read_to_string(filename).unwrap_or_default();
        let mut tokens = text.split_whitespace();
        // C++ `strm >> val`: the first failed extraction sets failbit, and every later extraction is a no-op
        let mut failed = false;
        for i in 0..(size as usize) {
            for _j in 0..(size as usize) {
                let mut val: f64 = 0.0;
                if !failed {
                    match tokens.next().and_then(|t| t.parse::<f64>().ok()) {
                        Some(v) => val = v,
                        None => failed = true,
                    }
                }
                myTable[i].push(val);
            }
        }
        Self {
            base: AbstractLookupTable { _p: std::marker::PhantomData },
            myTable,
            _p: std::marker::PhantomData,
        }
    }

    /// C++: `void FullLookupTable::~FullLookupTable<E, V>()` [virtual]
    // virtual ~FullLookupTable() {
    //     }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
    }

    /// C++: `double FullLookupTable::lowerBound(const E *, const E *, double, double, double, double) const` [const]
    // double lowerBound(const E* from, const E* to, double /*speed*/, double speedFactor, double /*fromEffort*/, double /*toEffort*/) const {
    //         return myTable[from->getNumericalID()][to->getNumericalID()] / speedFactor;
    //     }
    #[allow(unused_variables)]
    pub fn lower_bound(&self, from: Ptr<E>, to: Ptr<E>, arg2: f64, speedFactor: f64, arg4: f64, arg5: f64) -> f64 where E: RouterEdge<V> {
        self.myTable[e_numerical_id::<E, V>(from) as usize][e_numerical_id::<E, V>(to) as usize] / speedFactor
    }

    /// C++: `bool FullLookupTable::consistent() const` [const]
    // bool consistent() const {
    //         return true;
    //     }
    #[allow(unused_variables)]
    pub fn consistent(&self) -> bool {
        true
    }

}

/// C++ class `LandmarkLookupTable` (utils/router/AStarLookupTable.h)
#[repr(C)]
pub struct LandmarkLookupTable<E, V, M> {
    /// C++ base class `AbstractLookupTable<E, V>`
    pub base: AbstractLookupTable<E, V>,
    pub myLandmarks: BTreeMap<String, i32>,
    pub myFromLandmarkDists: Vec<Vec<f64>>,
    pub myToLandmarkDists: Vec<Vec<f64>>,
    pub myFirstNonInternal: i32,
    _p: std::marker::PhantomData<(E, V, M,)>,
}

impl<E, V, M> std::ops::Deref for LandmarkLookupTable<E, V, M> {
    type Target = AbstractLookupTable<E, V>;
    fn deref(&self) -> &AbstractLookupTable<E, V> {
        &self.base
    }
}
impl<E, V, M> std::ops::DerefMut for LandmarkLookupTable<E, V, M> {
    fn deref_mut(&mut self) -> &mut AbstractLookupTable<E, V> {
        &mut self.base
    }
}

impl<E: RouterEdge<V>, V: RouterVehicle, M> LandmarkLookupTable<E, V, M> {
    /// C++: `void LandmarkLookupTable::LandmarkLookupTable<E, V, M>(const std::string &, const std::vector<E *> &, SUMOAbstractRouter<E, V> *, SUMOAbstractRouter<ReversedEdge<E, V>, V> *, const V *, const std::string &, const int, M *)`
    // LandmarkLookupTable(const std::string& filename, const std::vector<E*>& edges, SUMOAbstractRouter<E, V>* router,
    //                         SUMOAbstractRouter<ReversedEdge<E, V>, V>* reverseRouter,
    //                         const V* defaultVehicle, const std::string& outfile, const int maxNumThreads, M* mapMatcher) {
    //         myFirstNonInternal = -1;
    //         std::map<std::string, int> numericID;
    //         bool haveTaz = false;
    //         for (E* e : edges) {
    //             if (!e->isInternal()) {
    //                 if (myFirstNonInternal == -1) {
    //                     myFirstNonInternal = e->getNumericalID();
    //                 }
    //                 if (e->isTazConnector()) {
    //                     haveTaz = true;
    //                 }
    //                 numericID[e->getID()] = e->getNumericalID() - myFirstNonInternal;
    //             }
    //         }
    // #ifdef HAVE_ZLIB
    //         zstr::ifstream strm(filename.c_str(), std::fstream::in | std::fstream::binary);
    // #else
    //         std::ifstream strm(filename.c_str());
    // #endif
    //         if (!strm.good()) {
    //             throw ProcessError(TLF("Could not load landmark-lookup-table from '%'.", filename));
    //         }
    //         std::string line;
    //         std::vector<const E*> landmarks;
    //         int numLandMarks = 0;
    //         bool haveData = false;
    //         while (std::getline(strm, line)) {
    //             if (line == "") {
    //                 break;
    //             }
    //             StringTokenizer st(line);
    //             if (st.size() == 1) {
    //                 const std::string lm = st.get(0);
    //                 if (myLandmarks.count(lm) != 0) {
    //                     throw ProcessError(TLF("Duplicate edge '%' in landmark file.", lm));
    //                 }
    //                 // retrieve landmark edge
    //                 const auto& it = numericID.find(lm);
    //                 if (it == numericID.end()) {
    //                     throw ProcessError(TLF("Landmark edge '%' does not exist in the network.", lm));
    //                 }
    //                 myLandmarks[lm] = numLandMarks++;
    //                 myFromLandmarkDists.push_back(std::vector<double>(0));
    //                 myToLandmarkDists.push_back(std::vector<double>(0));
    //                 landmarks.push_back(edges[it->second + myFirstNonInternal]);
    //             } else if (st.size() == 2) {
    //                 // geo landmark
    //                 try {
    //                     std::string lonStr = st.get(0);
    //                     if (!lonStr.empty() && lonStr.back() == ',') {
    //                         // remove trailing comma
    //                         lonStr = lonStr.substr(0, lonStr.size() - 1);
    //                     }
    //                     double lon = StringUtils::toDouble(lonStr);
    //                     double lat = StringUtils::toDouble(st.get(1));
    //                     std::vector<const E*> mapped;
    //                     bool ok;
    //                     mapMatcher->parseGeoEdges(PositionVector({Position(lon, lat)}), true, SVC_IGNORING, mapped, "LMLT", false, ok, true);
    //                     if (mapped.size() != 1) {
    //                         throw ProcessError(TLF("Invalid coordinate in landmark file, could not find edge at '%'", line));
    //                     }
    //                     std::string lm = mapped.front()->getID();
    //                     const auto& it = numericID.find(lm);
    //                     if (it == numericID.end()) {
    //                         throw ProcessError(TLF("Landmark edge '%' does not exist in the network.", lm));
    //                     }
    //                     myLandmarks[lm] = numLandMarks++;
    //                     myFromLandmarkDists.push_back(std::vector<double>(0));
    //                     myToLandmarkDists.push_back(std::vector<double>(0));
    //                     landmarks.push_back(edges[it->second + myFirstNonInternal]);
    //                 } catch (const NumberFormatException&) {
    //                     throw ProcessError(TLF("Broken landmark file, could not parse '%' as coordinates.", line));
    //                 }
    //             } else if (st.size() == 4) {
    //                 // legacy style landmark table
    //                 const std::string lm = st.get(0);
    //                 const std::string edge = st.get(1);
    //                 if (numericID[edge] != (int)myFromLandmarkDists[myLandmarks[lm]].size()) {
    //                     throw ProcessError(TLF("Unknown or unordered edge '%' in landmark file.", edge));
    //                 }
    //                 const double distFrom = StringUtils::toDouble(st.get(2));
    //                 const double distTo = StringUtils::toDouble(st.get(3));
    //                 myFromLandmarkDists[myLandmarks[lm]].push_back(distFrom);
    //                 myToLandmarkDists[myLandmarks[lm]].push_back(distTo);
    //                 haveData = true;
    //             } else {
    //                 const std::string edge = st.get(0);
    //                 if ((int)st.size() != 2 * numLandMarks + 1) {
    //                     throw ProcessError(TLF("Broken landmark file, unexpected number of entries (%) for edge '%'.", st.size() - 1, edge));
    //                 }
    //                 if (numericID[edge] != (int)myFromLandmarkDists[0].size()) {
    //                     throw ProcessError(TLF("Unknown or unordered edge '%' in landmark file.", edge));
    //                 }
    //                 for (int i = 0; i < numLandMarks; i++) {
    //                     const double distFrom = StringUtils::toDouble(st.get(2 * i + 1));
    //                     const double distTo = StringUtils::toDouble(st.get(2 * i + 2));
    //                     myFromLandmarkDists[i].push_back(distFrom);
    //                     myToLandmarkDists[i].push_back(distTo);
    //                 }
    //                 haveData = true;
    //             }
    //         }
    //         if (myLandmarks.empty()) {
    //             WRITE_WARNINGF("No landmarks in '%', falling back to standard A*.", filename);
    //             return;
    //         }
    // #ifdef HAVE_FOX
    //         MFXWorkerThread::Pool threadPool;
    //         std::vector<RoutingTask*> currentTasks;
    // #endif
    //         if (!haveData) {
    //             WRITE_MESSAGE(TL("Calculating new lookup table."));
    //         }
    //         for (int i = 0; i < numLandMarks; ++i) {
    //             if ((int)myFromLandmarkDists[i].size() != (int)edges.size() - myFirstNonInternal) {
    //                 const E* const landmark = landmarks[i];
    //                 if (haveData) {
    //                     if (myFromLandmarkDists[i].empty()) {
    //                         WRITE_WARNINGF(TL("No lookup table for landmark edge '%', recalculating."), landmark->getID());
    //                     } else {
    //                         const std::string tazWarning = haveTaz ? " Make sure that any used taz or junction-taz definitions are loaded when generating the table" : "";
    //                         throw ProcessError(TLF("Not all network edges were found in the lookup table '%' for landmark edge '%'.%", filename, landmark->getID(), tazWarning));
    //                     }
    //                 }
    // #ifdef HAVE_FOX
    //                 if (maxNumThreads > 0) {
    //                     const double lmCost = router->recomputeCosts({landmark}, defaultVehicle, 0);
    //                     router->setAutoBulkMode(true);
    //                     if (threadPool.size() == 0) {
    //                         if (reverseRouter == nullptr) {
    //                             // The CHRouter needs initialization
    //                             // before it gets cloned, so we do a dummy routing which is not in parallel
    //                             std::vector<const E*> route;
    //                             router->compute(landmark, landmark, defaultVehicle, 0, route, true);
    //                         } else {
    //                             reverseRouter->setAutoBulkMode(true);
    //                         }
    //                         while ((int)threadPool.size() < maxNumThreads) {
    //                             auto revClone = reverseRouter == nullptr ? nullptr : reverseRouter->clone();
    //                             new WorkerThread(threadPool, router->clone(), revClone, defaultVehicle);
    //                         }
    //                     }
    //                     for (int j = (int)myFromLandmarkDists[i].size() + myFirstNonInternal; j < (int)edges.size(); ++j) {
    //                         const E* const edge = edges[j];
    //                         if (landmark != edge) {
    //                             const double sourceDestCost = lmCost + router->recomputeCosts({edge}, defaultVehicle, 0);
    //                             currentTasks.push_back(new RoutingTask(landmark, edge, sourceDestCost));
    //                             threadPool.add(currentTasks.back(), i % maxNumThreads);
    //                         }
    //                     }
    //                 }
    // #else
    //                 UNUSED_PARAMETER(reverseRouter);
    // #endif
    //             }
    //         }
    // #ifdef HAVE_FOX
    //         threadPool.waitAll(false);
    //         int taskIndex = 0;
    // #endif
    //         for (int i = 0; i < numLandMarks; ++i) {
    //             if ((int)myFromLandmarkDists[i].size() != (int)edges.size() - myFirstNonInternal) {
    //                 const E* landmark = landmarks[i];
    //                 const double lmCost = router->recomputeCosts({landmark}, defaultVehicle, 0);
    //                 int unreachableFrom = 0;
    //                 int unreachableTo = 0;
    //                 for (int j = (int)myFromLandmarkDists[i].size() + myFirstNonInternal; j < (int)edges.size(); ++j) {
    //                     const E* const edge = edges[j];
    //                     double distFrom = -1;
    //                     double distTo = -1;
    //                     if (landmark == edge) {
    //                         distFrom = 0;
    //                         distTo = 0;
    //                     } else {
    //                         if (maxNumThreads > 0) {
    // #ifdef HAVE_FOX
    //                             distFrom = currentTasks[taskIndex]->getFromCost();
    //                             distTo = currentTasks[taskIndex]->getToCost();
    //                             delete currentTasks[taskIndex++];
    // #endif
    //                         } else {
    //                             const double sourceDestCost = lmCost + router->recomputeCosts({edge}, defaultVehicle, 0);
    //                             std::vector<const E*> route;
    //                             std::vector<const ReversedEdge<E, V>*> reversedRoute;
    //                             // compute from-distance (skip taz-sources and other unreachable edges)
    //                             if (edge->getPredecessors().size() > 0 && landmark->getSuccessors().size() > 0) {
    //                                 if (router->compute(landmark, edge, defaultVehicle, 0, route, true)) {
    //                                     distFrom = MAX2(0.0, router->recomputeCosts(route, defaultVehicle, 0) - sourceDestCost);
    //                                     route.clear();
    //                                 }
    //                             }
    //                             // compute to-distance (skip unreachable landmarks)
    //                             if (landmark->getPredecessors().size() > 0 && edge->getSuccessors().size() > 0) {
    //                                 if (router->compute(edge, landmark, defaultVehicle, 0, route, true)) {
    //                                     distTo = MAX2(0.0, router->recomputeCosts(route, defaultVehicle, 0) - sourceDestCost);
    //                                     route.clear();
    //                                 }
    //                             }
    //                         }
    //                     }
    //                     myFromLandmarkDists[i].push_back(distFrom);
    //                     myToLandmarkDists[i].push_back(distTo);
    //                     if (!edge->isTazConnector()) {
    //                         if (distFrom == -1) {
    //                             unreachableFrom++;
    //                         }
    //                         if (distTo == -1) {
    //                             unreachableTo++;
    //                         }
    //                     }
    //                 }
    //                 if (unreachableFrom > 0 || unreachableTo > 0) {
    //                     WRITE_WARNINGF(TL("Landmark % is not reachable from % edges and is unable to reach % out of % total edges."),
    //                                    landmark->getID(), unreachableFrom, unreachableTo, numericID.size());
    //                 }
    //             }
    //         }
    //         if (!outfile.empty()) {
    //             std::ostream* ostrm = nullptr;
    // #ifdef HAVE_ZLIB
    //             if (StringUtils::endsWith(outfile, ".gz")) {
    //                 ostrm = new zstr::ofstream(outfile.c_str(), std::ios_base::out);
    //             } else {
    // #endif
    //                 ostrm = new std::ofstream(outfile.c_str());
    // #ifdef HAVE_ZLIB
    //             }
    // #endif
    //             if (!ostrm->good()) {
    //                 delete ostrm;
    //                 throw ProcessError(TLF("Could not open file '%' for writing.", outfile));
    //             }
    //             WRITE_MESSAGEF(TL("Saving new matrix to '%'."), outfile);
    //             for (int i = 0; i < numLandMarks; ++i) {
    //                 (*ostrm) << getLandmark(i) << "\n";
    //             }
    //             for (int j = 0; j < (int)myFromLandmarkDists[0].size(); ++j) {
    //                 (*ostrm) << edges[j + myFirstNonInternal]->getID();
    //                 for (int i = 0; i < numLandMarks; ++i) {
    //                     (*ostrm) << " " << myFromLandmarkDists[i][j] << " " << myToLandmarkDists[i][j];
    //                 }
    //                 (*ostrm) << "\n";
    //             }
    //             delete ostrm;
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn new(filename: &str, edges: &Vec<Ptr<E>>, router: Ptr<SUMOAbstractRouter<E, V>>, reverseRouter: Ptr<SUMOAbstractRouter<ReversedEdge<E, V>, V>>, defaultVehicle: Ptr<V>, outfile: &str, maxNumThreads: i32, mapMatcher: Ptr<M>) -> Self where E: RouterEdge<V> {
        let mut s = Self {
            base: AbstractLookupTable { _p: std::marker::PhantomData },
            myLandmarks: BTreeMap::new(),
            myFromLandmarkDists: Vec::new(),
            myToLandmarkDists: Vec::new(),
            myFirstNonInternal: -1,
            _p: std::marker::PhantomData,
        };
        let mut numericID: BTreeMap<String, i32> = BTreeMap::new();
        let mut haveTaz = false;
        for e in edges.iter() {
            if !e_is_internal::<E, V>(*e) {
                if s.myFirstNonInternal == -1 {
                    s.myFirstNonInternal = e_numerical_id::<E, V>(*e);
                }
                if e_is_taz_connector::<E, V>(*e) {
                    haveTaz = true;
                }
                numericID.insert(e_id::<E, V>(*e), e_numerical_id::<E, V>(*e) - s.myFirstNonInternal);
            }
        }
        let text = match utils::iodevices::zstr::open_read(filename).and_then(|mut r| {
            let mut t = String::new();
            std::io::Read::read_to_string(&mut r, &mut t).map(|_| t)
        }) {
            Ok(t) => t,
            Err(_) => panic!("{}", ProcessError::new(format!("Could not load landmark-lookup-table from '{}'.", filename))),
        };
        let mut landmarks: Vec<Ptr<E>> = Vec::new();
        let mut numLandMarks: i32 = 0;
        let mut haveData = false;
        for line in text.split('\n') {
            if line == "" {
                break;
            }
            let st = utils::common::string_tokenizer::StringTokenizer::new(line);
            if st.size() == 1 {
                let lm = tok(&st, 0);
                if s.myLandmarks.contains_key(&lm) {
                    panic!("{}", ProcessError::new(format!("Duplicate edge '{}' in landmark file.", lm)));
                }
                // retrieve landmark edge
                let it = match numericID.get(&lm) {
                    Some(v) => *v,
                    None => panic!("{}", ProcessError::new(format!("Landmark edge '{}' does not exist in the network.", lm))),
                };
                s.myLandmarks.insert(lm.clone(), numLandMarks);
                numLandMarks += 1;
                s.myFromLandmarkDists.push(Vec::new());
                s.myToLandmarkDists.push(Vec::new());
                landmarks.push(edges[(it + s.myFirstNonInternal) as usize]);
            } else if st.size() == 2 {
                // geo landmark
                let mut lonStr = tok(&st, 0);
                if lonStr.ends_with(',') {
                    // remove trailing comma
                    lonStr.pop();
                }
                let lon = match utils::common::string_utils::to_double(&lonStr) {
                    Ok(v) => v,
                    Err(_) => panic!("{}", ProcessError::new(format!("Broken landmark file, could not parse '{}' as coordinates.", line))),
                };
                let lat = match utils::common::string_utils::to_double(&tok(&st, 1)) {
                    Ok(v) => v,
                    Err(_) => panic!("{}", ProcessError::new(format!("Broken landmark file, could not parse '{}' as coordinates.", line))),
                };
                let mapped: Vec<Ptr<E>> = e_geo_landmark_edges(mapMatcher, lon, lat);
                if mapped.len() != 1 {
                    panic!("{}", ProcessError::new(format!("Invalid coordinate in landmark file, could not find edge at '{}'", line)));
                }
                let lm = e_id::<E, V>(mapped[0]);
                let it = match numericID.get(&lm) {
                    Some(v) => *v,
                    None => panic!("{}", ProcessError::new(format!("Landmark edge '{}' does not exist in the network.", lm))),
                };
                s.myLandmarks.insert(lm.clone(), numLandMarks);
                numLandMarks += 1;
                s.myFromLandmarkDists.push(Vec::new());
                s.myToLandmarkDists.push(Vec::new());
                landmarks.push(edges[(it + s.myFirstNonInternal) as usize]);
            } else if st.size() == 4 {
                // legacy style landmark table
                let lm = tok(&st, 0);
                let edge = tok(&st, 1);
                let numEdge = *numericID.entry(edge.clone()).or_insert(0);
                let lmIdx = *s.myLandmarks.entry(lm.clone()).or_insert(0) as usize;
                if numEdge != s.myFromLandmarkDists[lmIdx].len() as i32 {
                    panic!("{}", ProcessError::new(format!("Unknown or unordered edge '{}' in landmark file.", edge)));
                }
                let distFrom = to_double_or_throw(&tok(&st, 2));
                let distTo = to_double_or_throw(&tok(&st, 3));
                s.myFromLandmarkDists[lmIdx].push(distFrom);
                s.myToLandmarkDists[lmIdx].push(distTo);
                haveData = true;
            } else {
                let edge = tok(&st, 0);
                if st.size() as i32 != 2 * numLandMarks + 1 {
                    panic!("{}", ProcessError::new(format!("Broken landmark file, unexpected number of entries ({}) for edge '{}'.", st.size() - 1, edge)));
                }
                let numEdge = *numericID.entry(edge.clone()).or_insert(0);
                if numEdge != s.myFromLandmarkDists[0].len() as i32 {
                    panic!("{}", ProcessError::new(format!("Unknown or unordered edge '{}' in landmark file.", edge)));
                }
                for i in 0..(numLandMarks as usize) {
                    let distFrom = to_double_or_throw(&tok(&st, 2 * i + 1));
                    let distTo = to_double_or_throw(&tok(&st, 2 * i + 2));
                    s.myFromLandmarkDists[i].push(distFrom);
                    s.myToLandmarkDists[i].push(distTo);
                }
                haveData = true;
            }
        }
        if s.myLandmarks.is_empty() {
            write_warning(&format!("No landmarks in '{}', falling back to standard A*.", filename));
            return s;
        }
        if !haveData {
            write_message("Calculating new lookup table.");
        }
        // shortcut: HAVE_FOX worker-thread pool is not ported; all tables are computed by the serial path below (same values)
        for i in 0..(numLandMarks as usize) {
            if s.myFromLandmarkDists[i].len() as i32 != edges.len() as i32 - s.myFirstNonInternal {
                let landmark = landmarks[i];
                if haveData {
                    if s.myFromLandmarkDists[i].is_empty() {
                        write_warning(&format!("No lookup table for landmark edge '{}', recalculating.", e_id::<E, V>(landmark)));
                    } else {
                        let tazWarning = if haveTaz { " Make sure that any used taz or junction-taz definitions are loaded when generating the table" } else { "" };
                        panic!("{}", ProcessError::new(format!("Not all network edges were found in the lookup table '{}' for landmark edge '{}'.{}", filename, e_id::<E, V>(landmark), tazWarning)));
                    }
                }
            }
        }
        for i in 0..(numLandMarks as usize) {
            if s.myFromLandmarkDists[i].len() as i32 != edges.len() as i32 - s.myFirstNonInternal {
                let landmark = landmarks[i];
                let lmCost = router.m().recompute_costs(&vec![landmark], defaultVehicle, 0, Ptr::null());
                let mut unreachableFrom = 0;
                let mut unreachableTo = 0;
                let start = s.myFromLandmarkDists[i].len() as i32 + s.myFirstNonInternal;
                for j in start..(edges.len() as i32) {
                    let edge = edges[j as usize];
                    let mut distFrom: f64 = -1.0;
                    let mut distTo: f64 = -1.0;
                    if landmark.as_raw() == edge.as_raw() {
                        distFrom = 0.0;
                        distTo = 0.0;
                    } else {
                        let sourceDestCost = lmCost + router.m().recompute_costs(&vec![edge], defaultVehicle, 0, Ptr::null());
                        let mut route: Vec<Ptr<E>> = Vec::new();
                        // compute from-distance (skip taz-sources and other unreachable edges)
                        if e_predecessor_count::<E, V>(edge) > 0 && e_successor_count::<E, V>(landmark) > 0 {
                            if router.m().compute(landmark, edge, defaultVehicle, 0, &mut route, true) {
                                let d = router.m().recompute_costs(&route, defaultVehicle, 0, Ptr::null()) - sourceDestCost;
                                distFrom = max2(0.0, d);
                                route.clear();
                            }
                        }
                        // compute to-distance (skip unreachable landmarks)
                        if e_predecessor_count::<E, V>(landmark) > 0 && e_successor_count::<E, V>(edge) > 0 {
                            if router.m().compute(edge, landmark, defaultVehicle, 0, &mut route, true) {
                                let d = router.m().recompute_costs(&route, defaultVehicle, 0, Ptr::null()) - sourceDestCost;
                                distTo = max2(0.0, d);
                                route.clear();
                            }
                        }
                    }
                    s.myFromLandmarkDists[i].push(distFrom);
                    s.myToLandmarkDists[i].push(distTo);
                    if !e_is_taz_connector::<E, V>(edge) {
                        if distFrom == -1.0 {
                            unreachableFrom += 1;
                        }
                        if distTo == -1.0 {
                            unreachableTo += 1;
                        }
                    }
                }
                if unreachableFrom > 0 || unreachableTo > 0 {
                    write_warning(&format!("Landmark {} is not reachable from {} edges and is unable to reach {} out of {} total edges.", e_id::<E, V>(landmark), unreachableFrom, unreachableTo, numericID.len()));
                }
            }
        }
        if !outfile.is_empty() {
            let mut ostrm = match utils::iodevices::zstr::create_write(outfile) {
                Ok(f) => f,
                Err(_) => panic!("{}", ProcessError::new(format!("Could not open file '{}' for writing.", outfile))),
            };
            write_message(&format!("Saving new matrix to '{}'.", outfile));
            let mut out = String::new();
            for i in 0..(numLandMarks as usize) {
                out += &s.get_landmark(i as i32);
                out += "\n";
            }
            for j in 0..s.myFromLandmarkDists[0].len() {
                out += &e_id::<E, V>(edges[(j as i32 + s.myFirstNonInternal) as usize]);
                for i in 0..(numLandMarks as usize) {
                    // ostream default formatting of doubles (precision 6)
                    out += &format!(" {} {}", format_g(s.myFromLandmarkDists[i][j], 6), format_g(s.myToLandmarkDists[i][j], 6));
                }
                out += "\n";
            }
            let _ = std::io::Write::write_all(&mut ostrm, out.as_bytes());
        }
        s
    }

    /// C++: `void LandmarkLookupTable::~LandmarkLookupTable<E, V, M>()` [virtual]
    // virtual ~LandmarkLookupTable() {
    //     }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
    }

    /// C++: `double LandmarkLookupTable::lowerBound(const E *, const E *, double, double, double, double) const` [const]
    // double lowerBound(const E* from, const E* to, double speed, double speedFactor, double fromEffort, double toEffort) const {
    //         double result = from->getDistanceTo(to) / speed;
    // #ifdef ASTAR_DEBUG_LOOKUPTABLE
    //         if (from->getID() == ASTAR_DEBUG_LOOKUPTABLE_FROM) {
    //             std::cout << " lowerBound to=" << to->getID() << " result1=" << result << "\n";
    //         }
    // #endif
    //         for (int i = 0; i < (int)myLandmarks.size(); ++i) {
    //             // a cost of -1 is used to encode unreachability.
    //             const double fl = myToLandmarkDists[i][from->getNumericalID() - myFirstNonInternal];
    //             const double tl = myToLandmarkDists[i][to->getNumericalID() - myFirstNonInternal];
    //             if (fl >= 0 && tl >= 0) {
    //                 const double bound = (fl - tl - toEffort) / speedFactor;
    // #ifdef ASTAR_DEBUG_LOOKUPTABLE
    //                 if (from->getID() == ASTAR_DEBUG_LOOKUPTABLE_FROM && result < bound) {
    //                     std::cout << "   landmarkTo=" << getLandmark(i) << " result2=" << bound
    //                               << " fl=" << fl << " tl=" << tl << "\n";
    //                 }
    // #endif
    //                 result = MAX2(result, bound);
    //             }
    //             const double lt = myFromLandmarkDists[i][to->getNumericalID() - myFirstNonInternal];
    //             const double lf = myFromLandmarkDists[i][from->getNumericalID() - myFirstNonInternal];
    //             if (lt >= 0 && lf >= 0) {
    //                 const double bound = (lt - lf - fromEffort) / speedFactor;
    // #ifdef ASTAR_DEBUG_LOOKUPTABLE
    //                 if (from->getID() == ASTAR_DEBUG_LOOKUPTABLE_FROM && result < bound) {
    //                     std::cout << "   landmarkFrom=" << getLandmark(i) << " result3=" << bound
    //                               << " lt=" << lt << " lf=" << lf << "\n";
    //                 }
    // #endif
    //                 result = MAX2(result, bound);
    //             }
    //             if ((tl >= 0 && fl < 0)
    //                     || (lf >= 0 && lt < 0)) {
    //                 // target unreachable.
    // #ifdef ASTAR_DEBUG_UNREACHABLE
    //                 std::cout << "   unreachable: from=" << from->getID() << " to=" << to->getID() << " landmark=" << getLandmark(i) << " "
    //                           << ((tl >= 0 && fl < 0) ? " (toLandmark)" : " (fromLandmark)")
    //                           << " fl=" << fl << " tl=" << tl << " lt=" << lt << " lf=" << lf
    //                           << "\n";
    // #endif
    //                 return UNREACHABLE;
    //             }
    //         }
    //         return result;
    //     }
    #[allow(unused_variables)]
    pub fn lower_bound(&self, from: Ptr<E>, to: Ptr<E>, speed: f64, speedFactor: f64, fromEffort: f64, toEffort: f64) -> f64 where E: RouterEdge<V> {
        let mut result = e_distance_to::<E, V>(from, to) / speed;
        for i in 0..(self.myLandmarks.len() as usize) {
            // a cost of -1 is used to encode unreachability.
            let fl = self.myToLandmarkDists[i][(e_numerical_id::<E, V>(from) - self.myFirstNonInternal) as usize];
            let tl = self.myToLandmarkDists[i][(e_numerical_id::<E, V>(to) - self.myFirstNonInternal) as usize];
            if fl >= 0.0 && tl >= 0.0 {
                let bound = (fl - tl - toEffort) / speedFactor;
                result = max2(result, bound);
            }
            let lt = self.myFromLandmarkDists[i][(e_numerical_id::<E, V>(to) - self.myFirstNonInternal) as usize];
            let lf = self.myFromLandmarkDists[i][(e_numerical_id::<E, V>(from) - self.myFirstNonInternal) as usize];
            if lt >= 0.0 && lf >= 0.0 {
                let bound = (lt - lf - fromEffort) / speedFactor;
                result = max2(result, bound);
            }
            if (tl >= 0.0 && fl < 0.0) || (lf >= 0.0 && lt < 0.0) {
                // target unreachable.
                return UNREACHABLE;
            }
        }
        result
    }

    /// C++: `bool LandmarkLookupTable::consistent() const` [const]
    // bool consistent() const {
    //         return false;
    //     }
    #[allow(unused_variables)]
    pub fn consistent(&self) -> bool {
        false
    }

    /// C++: `std::string LandmarkLookupTable::getLandmark(int) const` [const, private]
    // std::string getLandmark(int i) const {
    //         for (std::map<std::string, int>::const_iterator it = myLandmarks.begin(); it != myLandmarks.end(); ++it) {
    //             if (it->second == i) {
    //                 return it->first;
    //             }
    //         }
    //         return "";
    //     }
    #[allow(unused_variables)]
    pub fn get_landmark(&self, i: i32) -> String {
        for (k, v) in self.myLandmarks.iter() {
            if *v == i {
                return k.clone();
            }
        }
        String::new()
    }

}

/// C++ `AStarRouter::lookupTable` as a value: the router dispatches virtual lowerBound()/consistent() to the table kind
pub enum AStarLookupTableKind<E, V, M> {
    Full(FullLookupTable<E, V>),
    Landmark(LandmarkLookupTable<E, V, M>),
}

impl<E: RouterEdge<V>, V: RouterVehicle, M> AStarLookupTableKind<E, V, M> {
    pub fn lower_bound(&self, from: Ptr<E>, to: Ptr<E>, speed: f64, speedFactor: f64, fromEffort: f64, toEffort: f64) -> f64 {
        match self {
            AStarLookupTableKind::Full(t) => t.lower_bound(from, to, speed, speedFactor, fromEffort, toEffort),
            AStarLookupTableKind::Landmark(t) => t.lower_bound(from, to, speed, speedFactor, fromEffort, toEffort),
        }
    }

    pub fn consistent(&self) -> bool {
        match self {
            AStarLookupTableKind::Full(t) => t.consistent(),
            AStarLookupTableKind::Landmark(t) => t.consistent(),
        }
    }
}

pub const UNREACHABLE: f64 = f64::MAX / 1000.0;

fn max2(a: f64, b: f64) -> f64 {
    if a > b { a } else { b }
}

fn tok(st: &utils::common::string_tokenizer::StringTokenizer, i: usize) -> String {
    st.get(i).unwrap_or_else(|e| panic!("{}", e))
}

fn to_double_or_throw(s: &str) -> f64 {
    match utils::common::string_utils::to_double(s) {
        Ok(v) => v,
        Err(e) => panic!("{}", e),
    }
}

fn e_numerical_id<E, V>(e: Ptr<E>) -> i32 where E: RouterEdge<V> {
    e.numerical_id()
}

fn e_id<E, V>(e: Ptr<E>) -> String where E: RouterEdge<V> {
    e.id().to_string()
}

fn e_is_internal<E, V>(e: Ptr<E>) -> bool where E: RouterEdge<V> {
    e.is_internal()
}

fn e_is_taz_connector<E, V>(e: Ptr<E>) -> bool where E: RouterEdge<V> {
    e.is_taz_connector()
}

fn e_predecessor_count<E, V>(e: Ptr<E>) -> usize where E: RouterEdge<V> {
    e.predecessors().len()
}

fn e_successor_count<E, V>(e: Ptr<E>) -> usize where E: RouterEdge<V> {
    e.successors(SVC_IGNORING).len()
}

fn e_distance_to<E, V>(from: Ptr<E>, to: Ptr<E>) -> f64 where E: RouterEdge<V> {
    from.distance_to(to, false)
}

// shortcut: geo (lon,lat) landmarks need M::parseGeoEdges, but the Rust MapMatcher needs its host route handler (C++ passes the RORouteHandler itself) and duarouter passes no map matcher; upgrade by passing the handler as a MapMatcherHost
fn e_geo_landmark_edges<E, M>(_mapMatcher: Ptr<M>, _lon: f64, _lat: f64) -> Vec<Ptr<E>> {
    panic!("shortcut: needs map-matcher trait bounds (lead), M::parseGeoEdges")
}
