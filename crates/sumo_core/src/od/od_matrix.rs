//! Port of od/ODMatrix.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// An O/D (origin/destination) matrix
/// C++ class `ODMatrix` (od/ODMatrix.h)
#[repr(C)]
pub struct ODMatrix {
    /// C++ base class `SAXWeightsHandler::EdgeFloatTimeLineRetriever`
    pub base: Unmapped /* SAXWeightsHandler::EdgeFloatTimeLineRetriever */,
    /// The loaded cells
    pub myContainer: Vec<Ptr<ODCell>>,
    /// The loaded cells indexed by origin and destination
    pub myShortCut: BTreeMap<(String, String), Vec<Ptr<ODCell>>>, // C++: std::map<const std::pair<const std::string, const std::string>, std::vector<ODCell *>>
    /// The districts to retrieve sources/sinks from
    pub myDistricts: Ptr<ODDistrictCont>,
    /// The missing districts already warned about
    pub myMissingDistricts: BTreeSet<String>,
    /// Number of loaded vehicles
    pub myNumLoaded: f64,
    /// Number of written vehicles
    pub myNumWritten: f64,
    /// Number of discarded vehicles
    pub myNumDiscarded: f64,
    /// parsed time bounds
    pub myBegin: SUMOTime,
    /// parsed time bounds
    pub myEnd: SUMOTime,
    /// user-defined vType
    pub myVType: String,
    /// the scaling factor for traffic
    pub myScale: f64,
}

impl std::ops::Deref for ODMatrix {
    type Target = Unmapped /* SAXWeightsHandler::EdgeFloatTimeLineRetriever */;
    fn deref(&self) -> &Unmapped /* SAXWeightsHandler::EdgeFloatTimeLineRetriever */ {
        &self.base
    }
}
impl std::ops::DerefMut for ODMatrix {
    fn deref_mut(&mut self) -> &mut Unmapped /* SAXWeightsHandler::EdgeFloatTimeLineRetriever */ {
        &mut self.base
    }
}

impl ODMatrix {
    /// C++: `void ODMatrix::ODMatrix(const ODDistrictCont &, double)`
    /// Constructor
    // ODMatrix::ODMatrix(const ODDistrictCont& dc, double scale) :
    //     myDistricts(dc),
    //     myNumLoaded(0),
    //     myNumWritten(0),
    //     myNumDiscarded(0),
    //     myBegin(-1),
    //     myEnd(-1),
    //     myScale(scale)
    // {}
    #[allow(unused_variables)]
    pub fn new(dc: &ODDistrictCont, scale: f64) -> Self {
        todo!("port ODMatrix::ODMatrix")
    }

    /// C++: `void ODMatrix::~ODMatrix() noexcept` [virtual]
    /// Destructor
    // ODMatrix::~ODMatrix() {
    //     for (ODCell* const cell : myContainer) {
    //         delete cell;
    //     }
    //     myContainer.clear();
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port ODMatrix::~ODMatrix")
    }

    /// C++: `bool ODMatrix::add(double, const std::pair<SUMOTime, SUMOTime> &, const std::string &, const std::string &, const std::string &, const bool, const bool, bool)`
    /// Builds a single cell from the given values, verifying them
    // bool
    // ODMatrix::add(double vehicleNumber, const std::pair<SUMOTime, SUMOTime>& beginEnd,
    //               const std::string& origin, const std::string& destination,
    //               const std::string& vehicleType, const bool originIsEdge, const bool destinationIsEdge,
    //               bool noScaling) {
    //     if (vehicleNumber == 0) {
    //         return false;
    //     }
    //     myNumLoaded += vehicleNumber;
    //     if (!originIsEdge && !destinationIsEdge && myDistricts.get(origin) == nullptr && myDistricts.get(destination) == nullptr) {
    //         WRITE_WARNINGF(TL("Missing origin '%' and destination '%' (% vehicles)."), origin, destination, toString(vehicleNumber));
    //         myNumDiscarded += vehicleNumber;
    //         myMissingDistricts.insert(origin);
    //         myMissingDistricts.insert(destination);
    //         return false;
    //     } else if (!originIsEdge && myDistricts.get(origin) == 0) {
    //         WRITE_ERRORF(TL("Missing origin '%' (% vehicles)."), origin, toString(vehicleNumber));
    //         myNumDiscarded += vehicleNumber;
    //         myMissingDistricts.insert(origin);
    //         return false;
    //     } else if (!destinationIsEdge && myDistricts.get(destination) == 0) {
    //         WRITE_ERRORF(TL("Missing destination '%' (% vehicles)."), destination, toString(vehicleNumber));
    //         myNumDiscarded += vehicleNumber;
    //         myMissingDistricts.insert(destination);
    //         return false;
    //     }
    //     if (!originIsEdge && myDistricts.get(origin)->sourceNumber() == 0) {
    //         WRITE_ERRORF(TL("District '%' has no source."), origin);
    //         myNumDiscarded += vehicleNumber;
    //         return false;
    //     } else if (!destinationIsEdge && myDistricts.get(destination)->sinkNumber() == 0) {
    //         WRITE_ERRORF(TL("District '%' has no sink."), destination);
    //         myNumDiscarded += vehicleNumber;
    //         return false;
    //     }
    //     ODCell* cell = new ODCell();
    //     cell->begin = beginEnd.first;
    //     cell->end = beginEnd.second;
    //     cell->origin = origin;
    //     cell->destination = destination;
    //     cell->vehicleType = vehicleType;
    //     cell->vehicleNumber = vehicleNumber * (noScaling ? 1 : myScale);
    //     cell->originIsEdge = originIsEdge;
    //     cell->destinationIsEdge = destinationIsEdge;
    //     myContainer.push_back(cell);
    //     if (myBegin == -1 || cell->begin < myBegin) {
    //         myBegin = cell->begin;
    //     }
    //     if (cell->end > myEnd) {
    //         myEnd = cell->end;
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn add(&mut self, vehicleNumber: f64, beginEnd: &(SUMOTime, SUMOTime), origin: &str, destination: &str, vehicleType: &str, originIsEdge: bool, destinationIsEdge: bool, noScaling: bool) -> bool {
        todo!("port ODMatrix::add")
    }

    /// C++: `bool ODMatrix::add(const SUMOVehicleParameter &, bool, bool)`
    /// Adds a single vehicle with departure time
    // bool
    // ODMatrix::add(const SUMOVehicleParameter& veh, bool originIsEdge, bool destinationIsEdge) {
    //     const std::string fromTaz = veh.fromTaz;
    //     const std::string toTaz = veh.toTaz;
    //     if (myMissingDistricts.count(fromTaz) > 0 || myMissingDistricts.count(toTaz) > 0) {
    //         myNumLoaded += 1.;
    //         myNumDiscarded += 1.;
    //         return false;
    //     }
    //     // we start looking from the end because there is a high probability that the input is sorted by time
    //     std::vector<ODCell*>& odList = myShortCut[std::make_pair(fromTaz, toTaz)];
    //     ODCell* cell = nullptr;
    //     for (std::vector<ODCell*>::const_reverse_iterator c = odList.rbegin(); c != odList.rend(); ++c) {
    //         if ((*c)->begin <= veh.depart && (*c)->end > veh.depart && (*c)->vehicleType == veh.vtypeid) {
    //             cell = *c;
    //             break;
    //         }
    //     }
    //     if (cell == nullptr) {
    //         const SUMOTime interval = string2time(OptionsCont::getOptions().getString("aggregation-interval"));
    //         const int intervalIdx = (int)(veh.depart / interval);
    //         // single vehicles are already scaled
    //         if (add(1., std::make_pair(intervalIdx * interval, (intervalIdx + 1) * interval),
    //                 fromTaz, toTaz, veh.vtypeid, originIsEdge, destinationIsEdge, true)) {
    //             cell = myContainer.back();
    //             odList.push_back(cell);
    //         } else {
    //             return false;
    //         }
    //     } else {
    //         myNumLoaded += 1.;
    //         cell->vehicleNumber += 1.;
    //     }
    //     cell->departures[veh.depart].push_back(veh);
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn add_2(&mut self, veh: &SUMOVehicleParameter, originIsEdge: bool, destinationIsEdgeconst: bool) -> bool {
        todo!("port ODMatrix::add")
    }

    /// C++: `void ODMatrix::writeDefaultAttrs(OutputDevice &, const bool, const ODCell *const)`
    /// Helper function for flow and trip output writing the depart and arrival attributes
    // void
    // ODMatrix::writeDefaultAttrs(OutputDevice& dev, const bool noVtype,
    //                             const ODCell* const cell) {
    //     const OptionsCont& oc = OptionsCont::getOptions();
    //     if (!noVtype && cell->vehicleType != "") {
    //         dev.writeAttr(SUMO_ATTR_TYPE, cell->vehicleType);
    //     }
    //     dev.writeAttr(SUMO_ATTR_FROM_TAZ, cell->origin).writeAttr(SUMO_ATTR_TO_TAZ, cell->destination);
    //     if (oc.isSet("departlane") && oc.getString("departlane") != "default") {
    //         dev.writeAttr(SUMO_ATTR_DEPARTLANE, oc.getString("departlane"));
    //     }
    //     if (oc.isSet("departpos")) {
    //         dev.writeAttr(SUMO_ATTR_DEPARTPOS, oc.getString("departpos"));
    //     }
    //     if (oc.isSet("departspeed") && oc.getString("departspeed") != "default") {
    //         dev.writeAttr(SUMO_ATTR_DEPARTSPEED, oc.getString("departspeed"));
    //     }
    //     if (oc.isSet("arrivallane")) {
    //         dev.writeAttr(SUMO_ATTR_ARRIVALLANE, oc.getString("arrivallane"));
    //     }
    //     if (oc.isSet("arrivalpos")) {
    //         dev.writeAttr(SUMO_ATTR_ARRIVALPOS, oc.getString("arrivalpos"));
    //     }
    //     if (oc.isSet("arrivalspeed")) {
    //         dev.writeAttr(SUMO_ATTR_ARRIVALSPEED, oc.getString("arrivalspeed"));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn write_default_attrs(&mut self, dev: &mut OutputDevice, noVtype: bool, cell: Ptr<ODCell>) {
        todo!("port ODMatrix::writeDefaultAttrs")
    }

    /// C++: `void ODMatrix::write(SUMOTime, const SUMOTime, OutputDevice &, const bool, const bool, const bool, const std::string &, const bool, bool, bool, const std::string &)`
    /// Writes the vehicles stored in the matrix assigning the sources and sinks
    // void
    // ODMatrix::write(SUMOTime begin, const SUMOTime end,
    //                 OutputDevice& dev, const bool uniform,
    //                 const bool differSourceSink, const bool noVtype,
    //                 const std::string& prefix, const bool stepLog,
    //                 bool pedestrians, bool persontrips,
    //                 const std::string& modes) {
    //     if (myContainer.size() == 0) {
    //         return;
    //     }
    //     std::map<std::pair<std::string, std::string>, double> fractionLeft;
    //     int vehName = 0;
    //     sortByBeginTime();
    //     // recheck begin time
    //     begin = MAX2(begin, myContainer.front()->begin);
    //     std::vector<ODCell*>::iterator next = myContainer.begin();
    //     std::vector<ODVehicle> vehicles;
    //     SUMOTime lastOut = -DELTA_T;
    //
    //     const OptionsCont& oc = OptionsCont::getOptions();
    //     std::string personDepartPos = oc.isSet("departpos") ? oc.getString("departpos") : "random";
    //     std::string personArrivalPos = oc.isSet("arrivalpos") ? oc.getString("arrivalpos") : "random";
    //     SumoXMLAttr fromAttr = oc.getBool("junctions") ? SUMO_ATTR_FROM_JUNCTION : SUMO_ATTR_FROM;
    //     SumoXMLAttr toAttr = oc.getBool("junctions") ? SUMO_ATTR_TO_JUNCTION : SUMO_ATTR_TO;
    //     const std::string vType = oc.isSet("vtype") ? oc.getString("vtype") : "";
    //
    //     // go through the time steps
    //     for (SUMOTime t = begin; t < end;) {
    //         if (stepLog && t - lastOut >= DELTA_T) {
    //             std::cout << "Parsing time " + time2string(t) << '\r';
    //             lastOut = t;
    //         }
    //         // recheck whether a new cell got valid
    //         bool changed = false;
    //         while (next != myContainer.end() && (*next)->begin <= t && (*next)->end > t) {
    //             std::pair<std::string, std::string> odID = std::make_pair((*next)->origin, (*next)->destination);
    //             // check whether the current cell must be extended by the last fraction
    //             if (fractionLeft.find(odID) != fractionLeft.end()) {
    //                 (*next)->vehicleNumber += fractionLeft[odID];
    //                 fractionLeft[odID] = 0;
    //             }
    //             // get the new departures (into tmp)
    //             const int oldSize = (int)vehicles.size();
    //             const double fraction = computeDeparts(*next, vehName, vehicles, uniform, differSourceSink, prefix);
    //             if (oldSize != (int)vehicles.size()) {
    //                 changed = true;
    //             }
    //             if (fraction != 0) {
    //                 fractionLeft[odID] = fraction;
    //             }
    //             ++next;
    //         }
    //         if (changed) {
    //             sort(vehicles.begin(), vehicles.end(), descending_departure_comperator());
    //         }
    //
    //         for (std::vector<ODVehicle>::reverse_iterator i = vehicles.rbegin(); i != vehicles.rend() && (*i).depart == t; ++i) {
    //             if (t >= begin) {
    //                 myNumWritten++;
    //                 if (pedestrians) {
    //                     dev.openTag(SUMO_TAG_PERSON).writeAttr(SUMO_ATTR_ID, (*i).id).writeAttr(SUMO_ATTR_DEPART, time2string(t));
    //                     dev.writeAttr(SUMO_ATTR_DEPARTPOS, personDepartPos);
    //                     if (!noVtype && vType.size() > 0) {
    //                         dev.writeAttr(SUMO_ATTR_TYPE, vType);
    //                     }
    //                     dev.openTag(SUMO_TAG_WALK);
    //                     dev.writeAttr(fromAttr, (*i).from);
    //                     dev.writeAttr(toAttr, (*i).to);
    //                     dev.writeAttr(SUMO_ATTR_FROM_TAZ, (*i).cell->origin).writeAttr(SUMO_ATTR_TO_TAZ, (*i).cell->destination);
    //                     dev.writeAttr(SUMO_ATTR_ARRIVALPOS, personArrivalPos);
    //                     dev.closeTag();
    //                     dev.closeTag();
    //                 } else if (persontrips) {
    //                     dev.openTag(SUMO_TAG_PERSON).writeAttr(SUMO_ATTR_ID, (*i).id).writeAttr(SUMO_ATTR_DEPART, time2string(t));
    //                     dev.writeAttr(SUMO_ATTR_DEPARTPOS, personDepartPos);
    //                     dev.openTag(SUMO_TAG_PERSONTRIP);
    //                     dev.writeAttr(fromAttr, (*i).from);
    //                     dev.writeAttr(toAttr, (*i).to);
    //                     dev.writeAttr(SUMO_ATTR_FROM_TAZ, (*i).cell->origin).writeAttr(SUMO_ATTR_TO_TAZ, (*i).cell->destination);
    //                     dev.writeAttr(SUMO_ATTR_ARRIVALPOS, personArrivalPos);
    //                     if (modes != "") {
    //                         dev.writeAttr(SUMO_ATTR_MODES, modes);
    //                     }
    //                     dev.closeTag();
    //                     dev.closeTag();
    //                 } else {
    //                     dev.openTag(SUMO_TAG_TRIP).writeAttr(SUMO_ATTR_ID, (*i).id).writeAttr(SUMO_ATTR_DEPART, time2string(t));
    //                     dev.writeAttr(fromAttr, (*i).from);
    //                     dev.writeAttr(toAttr, (*i).to);
    //                     writeDefaultAttrs(dev, noVtype, i->cell);
    //                     dev.closeTag();
    //                 }
    //             }
    //         }
    //         while (vehicles.size() != 0 && vehicles.back().depart == t) {
    //             vehicles.pop_back();
    //         }
    //         if (!vehicles.empty()) {
    //             t = vehicles.back().depart;
    //         }
    //         if (next != myContainer.end() && (t > (*next)->begin || vehicles.empty())) {
    //             t = (*next)->begin;
    //         }
    //         if (next == myContainer.end() && vehicles.empty()) {
    //             break;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn write(&mut self, begin: SUMOTime, end: SUMOTime, dev: &mut OutputDevice, uniform: bool, differSourceSink: bool, noVtype: bool, prefix: &str, stepLog: bool, pedestrians: bool, persontrips: bool, modes: &str) {
        todo!("port ODMatrix::write")
    }

    /// C++: `void ODMatrix::writeFlows(const SUMOTime, const SUMOTime, OutputDevice &, const bool, const std::string &, bool, bool, bool, bool, const std::string &)`
    /// Writes the flows stored in the matrix
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn write_flows(&mut self, begin: SUMOTime, end: SUMOTime, dev: &mut OutputDevice, noVtype: bool, prefix: &str, asProbability: bool, asPoisson: bool, pedestrians: bool, persontrips: bool, modes: &str) {
        todo!("port ODMatrix::writeFlows")
    }

    /// C++: `double ODMatrix::getNumLoaded() const` [const]
    /// Returns the number of loaded vehicles
    // double
    // ODMatrix::getNumLoaded() const {
    //     return myNumLoaded;
    // }
    #[allow(unused_variables)]
    pub fn get_num_loaded(&self) -> f64 {
        todo!("port ODMatrix::getNumLoaded")
    }

    /// C++: `double ODMatrix::getNumWritten() const` [const]
    /// Returns the number of written vehicles
    // double
    // ODMatrix::getNumWritten() const {
    //     return myNumWritten;
    // }
    #[allow(unused_variables)]
    pub fn get_num_written(&self) -> f64 {
        todo!("port ODMatrix::getNumWritten")
    }

    /// C++: `double ODMatrix::getNumDiscarded() const` [const]
    /// Returns the number of discarded vehicles
    // double
    // ODMatrix::getNumDiscarded() const {
    //     return myNumDiscarded;
    // }
    #[allow(unused_variables)]
    pub fn get_num_discarded(&self) -> f64 {
        todo!("port ODMatrix::getNumDiscarded")
    }

    /// C++: `void ODMatrix::applyCurve(const Distribution_Points &)`
    /// Splits the stored cells dividing them on the given time line
    // void
    // ODMatrix::applyCurve(const Distribution_Points& ps) {
    //     std::vector<ODCell*> oldCells = myContainer;
    //     myContainer.clear();
    //     for (std::vector<ODCell*>::iterator i = oldCells.begin(); i != oldCells.end(); ++i) {
    //         std::vector<ODCell*> newCells;
    //         applyCurve(ps, *i, newCells);
    //         copy(newCells.begin(), newCells.end(), back_inserter(myContainer));
    //         delete *i;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn apply_curve(&mut self, ps: &Unmapped /* Distribution_Points */) {
        todo!("port ODMatrix::applyCurve")
    }

    /// C++: `void ODMatrix::readO(LineReader &, double, std::string, bool)`
    /// read a VISUM-matrix with the O Format
    // void
    // ODMatrix::readO(LineReader& lr, double scale,
    //                 std::string vehType, bool matrixHasVehType) {
    //     PROGRESS_BEGIN_MESSAGE("Reading matrix '" + lr.getFileName() + "' stored as OR");
    //     // parse first defs
    //     std::string line;
    //     if (matrixHasVehType) {
    //         line = getNextNonCommentLine(lr);
    //         int type = StringUtils::toInt(StringUtils::prune(line));
    //         if (vehType == "") {
    //             vehType = toString(type);
    //         }
    //     }
    //
    //     const std::pair<SUMOTime, SUMOTime> beginEnd = readTime(lr);
    //     const double factor = readFactor(lr, scale);
    //
    //     // parse the cells
    //     while (lr.hasMore()) {
    //         line = getNextNonCommentLine(lr);
    //         if (line.length() == 0) {
    //             continue;
    //         }
    //         StringTokenizer st2(line, StringTokenizer::WHITECHARS);
    //         if (st2.size() == 0) {
    //             continue;
    //         }
    //         try {
    //             std::string sourceD = st2.next();
    //             std::string destD = st2.next();
    //             double vehNumber = StringUtils::toDouble(st2.next()) * factor;
    //             if (vehNumber != 0) {
    //                 add(vehNumber, beginEnd, sourceD, destD, vehType);
    //             }
    //         } catch (OutOfBoundsException&) {
    //             throw ProcessError(TLF("Missing at least one information in line '%'.", line));
    //         } catch (NumberFormatException&) {
    //             throw ProcessError(TLF("Not numeric vehicle number in line '%'.", line));
    //         }
    //     }
    //     PROGRESS_DONE_MESSAGE();
    // }
    #[allow(unused_variables)]
    pub fn read_o(&mut self, lr: &mut LineReader, scale: f64, vehType: String, matrixHasVehType: bool) {
        todo!("port ODMatrix::readO")
    }

    /// C++: `void ODMatrix::readV(LineReader &, double, std::string, bool)`
    /// read a VISUM-matrix with the V Format
    // void
    // ODMatrix::readV(LineReader& lr, double scale,
    //                 std::string vehType, bool matrixHasVehType) {
    //     PROGRESS_BEGIN_MESSAGE("Reading matrix '" + lr.getFileName() + "' stored as VMR");
    //     // parse first defs
    //     std::string line;
    //     if (matrixHasVehType) {
    //         line = getNextNonCommentLine(lr);
    //         if (vehType == "") {
    //             vehType = StringUtils::prune(line);
    //         }
    //     }
    //
    //     const std::pair<SUMOTime, SUMOTime> beginEnd = readTime(lr);
    //     const double factor = readFactor(lr, scale);
    //
    //     // districts
    //     line = getNextNonCommentLine(lr);
    //     const int numDistricts = StringUtils::toInt(StringUtils::prune(line));
    //     // parse district names (normally ints)
    //     std::vector<std::string> names;
    //     while ((int)names.size() != numDistricts && lr.hasMore()) {
    //         line = getNextNonCommentLine(lr);
    //         StringTokenizer st2(line, StringTokenizer::WHITECHARS);
    //         while (st2.hasNext()) {
    //             names.push_back(st2.next());
    //         }
    //     }
    //     if (!lr.hasMore()) {
    //         throw ProcessError(TLF("Missing line with % district names.", toString(numDistricts)));
    //     }
    //
    //     // parse the cells
    //     for (std::vector<std::string>::iterator si = names.begin(); si != names.end(); ++si) {
    //         std::vector<std::string>::iterator di = names.begin();
    //         do {
    //             try {
    //                 line = getNextNonCommentLine(lr);
    //             } catch (ProcessError&) {
    //                 throw ProcessError(TLF("Missing line for district %.", (*si)));
    //             }
    //             if (line.length() == 0) {
    //                 continue;
    //             }
    //             try {
    //                 StringTokenizer st2(line, StringTokenizer::WHITECHARS);
    //                 while (st2.hasNext()) {
    //                     assert(di != names.end());
    //                     double vehNumber = StringUtils::toDouble(st2.next()) * factor;
    //                     if (vehNumber != 0) {
    //                         add(vehNumber, beginEnd, *si, *di, vehType);
    //                     }
    //                     if (di == names.end()) {
    //                         throw ProcessError(TL("More entries than districts found."));
    //                     }
    //                     ++di;
    //                 }
    //             } catch (NumberFormatException&) {
    //                 throw ProcessError(TLF("Not numeric vehicle number in line '%'.", line));
    //             }
    //             if (!lr.hasMore()) {
    //                 break;
    //             }
    //         } while (di != names.end());
    //     }
    //     PROGRESS_DONE_MESSAGE();
    // }
    #[allow(unused_variables)]
    pub fn read_v(&mut self, lr: &mut LineReader, scale: f64, vehType: String, matrixHasVehType: bool) {
        todo!("port ODMatrix::readV")
    }

    /// C++: `void ODMatrix::loadMatrix(OptionsCont &)`
    /// read a matrix in one of several formats
    // void
    // ODMatrix::loadMatrix(OptionsCont& oc) {
    //     std::vector<std::string> files = oc.getStringVector("od-matrix-files");
    //     for (std::vector<std::string>::iterator i = files.begin(); i != files.end(); ++i) {
    //         LineReader lr(*i);
    //         if (!lr.good()) {
    //             throw ProcessError(TLF("Could not open '%'.", (*i)));
    //         }
    //         std::string type = lr.readLine();
    //         // get the type only
    //         if (type.find(';') != std::string::npos) {
    //             type = type.substr(0, type.find(';'));
    //         }
    //         // parse type-dependant
    //         if (type.length() > 1 && type[1] == 'V') {
    //             // process ptv's 'V'-matrices
    //             if (type.find('N') != std::string::npos) {
    //                 throw ProcessError(TLF("'%' does not contain the needed information about the time described.", *i));
    //             }
    //             readV(lr, 1, oc.getString("vtype"), type.find('M') != std::string::npos);
    //         } else if (type.length() > 1 && type[1] == 'O') {
    //             // process ptv's 'O'-matrices
    //             if (type.find('N') != std::string::npos) {
    //                 throw ProcessError(TLF("'%' does not contain the needed information about the time described.", *i));
    //             }
    //             readO(lr, 1, oc.getString("vtype"), type.find('M') != std::string::npos);
    //         } else {
    //             throw ProcessError("'" + *i + "' uses an unknown matrix type '" + type + "'.");
    //         }
    //     }
    //     std::vector<std::string> amitranFiles = oc.getStringVector("od-amitran-files");
    //     for (std::vector<std::string>::iterator i = amitranFiles.begin(); i != amitranFiles.end(); ++i) {
    //         if (!FileHelpers::isReadable(*i)) {
    //             throw ProcessError(TLF("Could not access matrix file '%' to load.", *i));
    //         }
    //         PROGRESS_BEGIN_MESSAGE("Loading matrix in Amitran format from '" + *i + "'");
    //         ODAmitranHandler handler(*this, *i);
    //         if (!XMLSubSys::runParser(handler, *i)) {
    //             PROGRESS_FAILED_MESSAGE();
    //         } else {
    //             PROGRESS_DONE_MESSAGE();
    //         }
    //     }
    //     myVType = oc.getString("vtype");
    //     for (std::string file : oc.getStringVector("tazrelation-files")) {
    //         if (!FileHelpers::isReadable(file)) {
    //             throw ProcessError(TLF("Could not access matrix file '%' to load.", file));
    //         }
    //         PROGRESS_BEGIN_MESSAGE("Loading matrix in tazRelation format from '" + file + "'");
    //
    //         std::vector<SAXWeightsHandler::ToRetrieveDefinition*> retrieverDefs;
    //         retrieverDefs.push_back(new SAXWeightsHandler::ToRetrieveDefinition(oc.getString("tazrelation-attribute"), true, *this));
    //         SAXWeightsHandler handler(retrieverDefs, "");
    //         if (!XMLSubSys::runParser(handler, file)) {
    //             PROGRESS_FAILED_MESSAGE();
    //         } else {
    //             PROGRESS_DONE_MESSAGE();
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn load_matrix(&mut self, oc: &mut OptionsCont) {
        todo!("port ODMatrix::loadMatrix")
    }

    /// C++: `void ODMatrix::loadRoutes(OptionsCont &, SUMOSAXHandler &)`
    /// read SUMO routes
    // void
    // ODMatrix::loadRoutes(OptionsCont& oc, SUMOSAXHandler& handler) {
    //     std::vector<std::string> routeFiles = oc.getStringVector("route-files");
    //     for (std::vector<std::string>::iterator i = routeFiles.begin(); i != routeFiles.end(); ++i) {
    //         if (!FileHelpers::isReadable(*i)) {
    //             throw ProcessError(TLF("Could not access route file '%' to load.", *i));
    //         }
    //         PROGRESS_BEGIN_MESSAGE("Loading routes and trips from '" + *i + "'");
    //         if (!XMLSubSys::runParser(handler, *i)) {
    //             PROGRESS_FAILED_MESSAGE();
    //         } else {
    //             PROGRESS_DONE_MESSAGE();
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn load_routes(&mut self, oc: &mut OptionsCont, handler: &mut Unmapped /* SUMOSAXHandler */) {
        todo!("port ODMatrix::loadRoutes")
    }

    /// C++: `Distribution_Points ODMatrix::parseTimeLine(const std::vector<std::string> &, bool)`
    /// split the given timeline
    // Distribution_Points
    // ODMatrix::parseTimeLine(const std::vector<std::string>& def, bool timelineDayInHours) {
    //     Distribution_Points result("N/A");
    //     if (timelineDayInHours) {
    //         if (def.size() != 24) {
    //             throw ProcessError(TLF("Assuming 24 entries for a day timeline, but got %.", toString(def.size())));
    //         }
    //         for (int chour = 0; chour < 24; ++chour) {
    //             result.add(chour * 3600., StringUtils::toDouble(def[chour]));
    //         }
    //         result.add(24 * 3600., 0.); // dummy value to finish the last interval
    //     } else {
    //         for (int i = 0; i < (int)def.size(); i++) {
    //             StringTokenizer st2(def[i], ":");
    //             if (st2.size() != 2) {
    //                 throw ProcessError(TLF("Broken time line definition: missing a value in '%'.", def[i]));
    //             }
    //             const double time = StringUtils::toDouble(st2.next());
    //             result.add(time, StringUtils::toDouble(st2.next()));
    //         }
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn parse_time_line(&mut self, def: &Vec<String>, timelineDayInHours: bool) -> Unmapped /* Distribution_Points */ {
        todo!("port ODMatrix::parseTimeLine")
    }

    /// C++: `const std::vector<ODCell *> & ODMatrix::getCells()`
    // const std::vector<ODCell*>& getCells() {
    //         return myContainer;
    //     }
    #[allow(unused_variables)]
    pub fn get_cells(&mut self) -> &Vec<Ptr<ODCell>> {
        todo!("port ODMatrix::getCells")
    }

    /// C++: `void ODMatrix::sortByBeginTime()`
    // void
    // ODMatrix::sortByBeginTime() {
    //     std::sort(myContainer.begin(), myContainer.end(), cell_by_begin_comparator());
    // }
    #[allow(unused_variables)]
    pub fn sort_by_begin_time(&mut self) {
        todo!("port ODMatrix::sortByBeginTime")
    }

    /// C++: `SUMOTime ODMatrix::getBegin() const` [const]
    // SUMOTime getBegin() const {
    //         return myBegin;
    //     }
    #[allow(unused_variables)]
    pub fn get_begin(&self) -> SUMOTime {
        todo!("port ODMatrix::getBegin")
    }

    /// C++: `SUMOTime ODMatrix::getEnd() const` [const]
    // SUMOTime getEnd() const {
    //         return myEnd;
    //     }
    #[allow(unused_variables)]
    pub fn get_end(&self) -> SUMOTime {
        todo!("port ODMatrix::getEnd")
    }

    /// C++: `void ODMatrix::addTazRelWeight(const std::string, const std::string &, const std::string &, double, double, double)` [virtual]
    // void
    // ODMatrix::addTazRelWeight(const std::string intervalID, const std::string& from, const std::string& to,
    //                           double val, double beg, double end) {
    //     add(val, std::make_pair(TIME2STEPS(beg), TIME2STEPS(end)), from, to, myVType == "" ? intervalID : myVType);
    // }
    #[allow(unused_variables)]
    pub fn add_taz_rel_weight(&mut self, intervalID: String, from: &str, to: &str, val: f64, beg: f64, end: f64) {
        todo!("port ODMatrix::addTazRelWeight")
    }

    /// C++: `double ODMatrix::computeDeparts(ODCell *, int &, std::vector<ODVehicle> &, const bool, const bool, const std::string &)` [protected]
    /// Computes the vehicle departs stored in the given cell and saves them in "into"
    // double
    // ODMatrix::computeDeparts(ODCell* cell,
    //                          int& vehName, std::vector<ODVehicle>& into,
    //                          const bool uniform, const bool differSourceSink,
    //                          const std::string& prefix) {
    //     int vehicles2insert = (int) cell->vehicleNumber;
    //     // compute whether the fraction forces an additional vehicle insertion
    //     if (RandHelper::rand() < cell->vehicleNumber - (double)vehicles2insert) {
    //         vehicles2insert++;
    //     }
    //     if (vehicles2insert == 0) {
    //         return cell->vehicleNumber;
    //     }
    //
    //     const double offset = (double)(cell->end - cell->begin) / (double) vehicles2insert / (double) 2.;
    //     for (int i = 0; i < vehicles2insert; ++i) {
    //         ODVehicle veh;
    //         veh.id = prefix + toString(vehName++);
    //
    //         if (uniform) {
    //             veh.depart = cell->begin + (SUMOTime)(offset + ((double)(cell->end - cell->begin) * (double) i / (double) vehicles2insert));
    //         } else {
    //             veh.depart = (SUMOTime)RandHelper::rand(cell->begin, cell->end);
    //         }
    //         const bool canDiffer = myDistricts.get(cell->origin)->sourceNumber() > 1 || myDistricts.get(cell->destination)->sinkNumber() > 1;
    //         do {
    //             veh.from = myDistricts.getRandomSourceFromDistrict(cell->origin);
    //             veh.to = myDistricts.getRandomSinkFromDistrict(cell->destination);
    //         } while (canDiffer && differSourceSink && (veh.to == veh.from));
    //         if (!canDiffer && differSourceSink && (veh.to == veh.from)) {
    //             WRITE_WARNINGF(TL("Cannot find different source and sink edge for origin '%' and destination '%'."), cell->origin, cell->destination);
    //         }
    //         veh.cell = cell;
    //         into.push_back(veh);
    //     }
    //     return cell->vehicleNumber - vehicles2insert;
    // }
    #[allow(unused_variables)]
    pub fn compute_departs(&mut self, cell: Ptr<ODCell>, vehName: &mut i32, into: &mut Vec<ODMatrix_ODVehicle>, uniform: bool, differSourceSink: bool, prefix: &str) -> f64 {
        todo!("port ODMatrix::computeDeparts")
    }

    /// C++: `void ODMatrix::applyCurve(const Distribution_Points &, ODCell *, std::vector<ODCell *> &)` [protected]
    /// Splits the given cell dividing it on the given time line and storing the results in the given container
    // void
    // ODMatrix::applyCurve(const Distribution_Points& ps, ODCell* cell, std::vector<ODCell*>& newCells) {
    //     const std::vector<double>& times = ps.getVals();
    //     for (int i = 0; i < (int)times.size() - 1; ++i) {
    //         ODCell* ncell = new ODCell();
    //         ncell->begin = TIME2STEPS(times[i]);
    //         ncell->end = TIME2STEPS(times[i + 1]);
    //         ncell->origin = cell->origin;
    //         ncell->destination = cell->destination;
    //         ncell->vehicleType = cell->vehicleType;
    //         ncell->vehicleNumber = cell->vehicleNumber * ps.getProbs()[i] / ps.getOverallProb();
    //         newCells.push_back(ncell);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn apply_curve_2(&mut self, ps: &Unmapped /* Distribution_Points */, cell: Ptr<ODCell>, newCells: &mut Vec<Ptr<ODCell>>) {
        todo!("port ODMatrix::applyCurve")
    }

    /// C++: `std::string ODMatrix::getNextNonCommentLine(LineReader &)` [private]
    /// in the functions readV and readO
    // std::string
    // ODMatrix::getNextNonCommentLine(LineReader& lr) {
    //     while (lr.good() && lr.hasMore()) {
    //         const std::string line = lr.readLine();
    //         if (line[0] != '*') {
    //             return StringUtils::prune(line);
    //         }
    //     }
    //     throw ProcessError(TLF("End of file while reading %.", lr.getFileName()));
    // }
    #[allow(unused_variables)]
    pub fn get_next_non_comment_line(&mut self, lr: &mut LineReader) -> String {
        todo!("port ODMatrix::getNextNonCommentLine")
    }

    /// C++: `SUMOTime ODMatrix::parseSingleTime(const std::string &)` [private]
    /// in the functions readV and readO
    // SUMOTime
    // ODMatrix::parseSingleTime(const std::string& time) {
    //     if (time.find('.') == std::string::npos) {
    //         throw NumberFormatException("no separator");
    //     }
    //     const std::string hours = time.substr(0, time.find('.'));
    //     const std::string minutes = time.substr(time.find('.') + 1);
    //     return TIME2STEPS(StringUtils::toInt(hours) * 3600 + StringUtils::toInt(minutes) * 60);
    // }
    #[allow(unused_variables)]
    pub fn parse_single_time(&mut self, time: &str) -> SUMOTime {
        todo!("port ODMatrix::parseSingleTime")
    }

    /// C++: `std::pair<SUMOTime, SUMOTime> ODMatrix::readTime(LineReader &)` [private]
    /// in the functions readV and readO
    // std::pair<SUMOTime, SUMOTime>
    // ODMatrix::readTime(LineReader& lr) {
    //     std::string line = getNextNonCommentLine(lr);
    //     try {
    //         StringTokenizer st(line, StringTokenizer::WHITECHARS);
    //         const SUMOTime begin = parseSingleTime(st.next());
    //         const SUMOTime end = parseSingleTime(st.next());
    //         if (begin >= end) {
    //             throw ProcessError("Matrix begin time " + time2string(begin) + " is larger than end time " + time2string(end) + ".");
    //         }
    //         return std::make_pair(begin, end);
    //     } catch (OutOfBoundsException&) {
    //         throw ProcessError(TLF("Broken period definition '%'.", line));
    //     } catch (NumberFormatException& e) {
    //         throw ProcessError("Broken period definition '" + line + "' (" + e.what() + ").");
    //     }
    // }
    #[allow(unused_variables)]
    pub fn read_time(&mut self, lr: &mut LineReader) -> (SUMOTime, SUMOTime) {
        todo!("port ODMatrix::readTime")
    }

    /// C++: `double ODMatrix::readFactor(LineReader &, double)` [private]
    /// in the functions readV and readO
    // double
    // ODMatrix::readFactor(LineReader& lr, double scale) {
    //     std::string line = getNextNonCommentLine(lr);
    //     double factor = -1;
    //     try {
    //         factor = StringUtils::toDouble(line) * scale;
    //     } catch (NumberFormatException&) {
    //         throw ProcessError(TLF("Broken factor: '%'.", line));
    //     }
    //     return factor;
    // }
    #[allow(unused_variables)]
    pub fn read_factor(&mut self, lr: &mut LineReader, scale: f64) -> f64 {
        todo!("port ODMatrix::readFactor")
    }

    /// C++: `void ODMatrix::ODMatrix(const ODMatrix &)` [private]
    /// invalid copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(s: &ODMatrix) -> Self {
        todo!("port ODMatrix::ODMatrix")
    }

    /// C++: `ODMatrix & ODMatrix::operator=(const ODMatrix &)` [private]
    /// invalid assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &ODMatrix) -> &mut ODMatrix {
        todo!("port ODMatrix::operator=")
    }

}

/// An internal representation of a single vehicle
/// C++ class `ODMatrix::ODVehicle` (od/ODMatrix.h)
pub struct ODMatrix_ODVehicle {
    /// The id of the vehicle
    pub id: String,
    /// The departure time of the vehicle
    pub depart: SUMOTime,
    /// The cell of the ODMatrix which generated the vehicle
    pub cell: Ptr<ODCell>,
    /// The edge the vehicles shall start at
    pub from: String,
    /// The edge the vehicles shall end at
    pub to: String,
}

impl ODMatrix_ODVehicle {
}

/// Used for sorting the cells by the begin time they describe
/// C++ class `ODMatrix::cell_by_begin_comparator` (od/ODMatrix.h)
pub struct ODMatrix_cell_by_begin_comparator {
}

impl ODMatrix_cell_by_begin_comparator {
    /// C++: `void cell_by_begin_comparator::cell_by_begin_comparator()`
    /// constructor
    // explicit cell_by_begin_comparator() { }
    #[allow(unused_variables)]
    pub fn new() -> Self {
        todo!("port cell_by_begin_comparator::cell_by_begin_comparator")
    }

    /// C++: `int cell_by_begin_comparator::operator()(ODCell *, ODCell *) const` [const]
    /// Comparing operator
    // int operator()(ODCell* p1, ODCell* p2) const {
    //             if (p1->begin == p2->begin) {
    //                 if (p1->origin == p2->origin) {
    //                     return p1->destination < p2->destination;
    //                 }
    //                 return p1->origin < p2->origin;
    //             }
    //             return p1->begin < p2->begin;
    //         }
    #[allow(unused_variables)]
    pub fn call_op(&self, p1: Ptr<ODCell>, p2: Ptr<ODCell>) -> i32 {
        todo!("port cell_by_begin_comparator::operator()")
    }

}

/// Used for sorting vehicles by their departure (latest first)
/// C++ class `ODMatrix::descending_departure_comperator` (od/ODMatrix.h)
pub struct ODMatrix_descending_departure_comperator {
}

impl ODMatrix_descending_departure_comperator {
    /// C++: `void descending_departure_comperator::descending_departure_comperator()`
    /// constructor
    // descending_departure_comperator() { }
    #[allow(unused_variables)]
    pub fn new() -> Self {
        todo!("port descending_departure_comperator::descending_departure_comperator")
    }

    /// C++: `bool descending_departure_comperator::operator()(const ODVehicle &, const ODVehicle &) const` [const]
    /// Comparing operator
    // bool operator()(const ODVehicle& p1, const ODVehicle& p2) const {
    //             if (p1.depart == p2.depart) {
    //                 return p1.id > p2.id;
    //             }
    //             return p1.depart > p2.depart;
    //         }
    #[allow(unused_variables)]
    pub fn call_op(&self, p1: &ODMatrix_ODVehicle, p2: &ODMatrix_ODVehicle) -> bool {
        todo!("port descending_departure_comperator::operator()")
    }

}

