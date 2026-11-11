//! Port of router/ROLoader.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// Obtains edge travel times from a weights handler and stores them within the edges
/// C++ class `ROLoader::EdgeFloatTimeLineRetriever_EdgeTravelTime` (router/ROLoader.h)
pub struct ROLoader_EdgeFloatTimeLineRetriever_EdgeTravelTime {
    /// C++ base class `SAXWeightsHandler::EdgeFloatTimeLineRetriever`
    pub base: Unmapped /* SAXWeightsHandler::EdgeFloatTimeLineRetriever */,
    /// The network edges shall be obtained from
    pub myNet: Ptr<RONet>,
}

impl std::ops::Deref for ROLoader_EdgeFloatTimeLineRetriever_EdgeTravelTime {
    type Target = Unmapped /* SAXWeightsHandler::EdgeFloatTimeLineRetriever */;
    fn deref(&self) -> &Unmapped /* SAXWeightsHandler::EdgeFloatTimeLineRetriever */ {
        &self.base
    }
}
impl std::ops::DerefMut for ROLoader_EdgeFloatTimeLineRetriever_EdgeTravelTime {
    fn deref_mut(&mut self) -> &mut Unmapped /* SAXWeightsHandler::EdgeFloatTimeLineRetriever */ {
        &mut self.base
    }
}

impl ROLoader_EdgeFloatTimeLineRetriever_EdgeTravelTime {
    /// C++: `void EdgeFloatTimeLineRetriever_EdgeTravelTime::EdgeFloatTimeLineRetriever_EdgeTravelTime(RONet &)`
    /// Constructor
    // EdgeFloatTimeLineRetriever_EdgeTravelTime(RONet& net) : myNet(net) {}
    #[allow(unused_variables)]
    pub fn new(net: &mut RONet) -> Self {
        todo!("port EdgeFloatTimeLineRetriever_EdgeTravelTime::EdgeFloatTimeLineRetriever_EdgeTravelTime")
    }

    /// C++: `void EdgeFloatTimeLineRetriever_EdgeTravelTime::~EdgeFloatTimeLineRetriever_EdgeTravelTime() noexcept` [virtual]
    /// Destructor
    // ~EdgeFloatTimeLineRetriever_EdgeTravelTime() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port EdgeFloatTimeLineRetriever_EdgeTravelTime::~EdgeFloatTimeLineRetriever_EdgeTravelTime")
    }

    /// C++: `void EdgeFloatTimeLineRetriever_EdgeTravelTime::addEdgeWeight(const std::string &, double, double, double) const` [const, virtual]
    /// Adds a travel time for a given edge and time period
    // void
    // ROLoader::EdgeFloatTimeLineRetriever_EdgeTravelTime::addEdgeWeight(const std::string& id,
    //         double val, double beg, double end) const {
    //     ROEdge* e = myNet.getEdge(id);
    //     if (e != nullptr) {
    //         e->addTravelTime(val, beg, end);
    //     } else {
    //         if (id[0] != ':') {
    //             if (OptionsCont::getOptions().getBool("ignore-errors")) {
    //                 WRITE_WARNINGF(TL("Trying to set a weight for the unknown edge '%'."), id);
    //             } else {
    //                 WRITE_ERRORF(TL("Trying to set a weight for the unknown edge '%'."), id);
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_edge_weight(&self, id: &str, val: f64, beg: f64, end: f64) {
        todo!("port EdgeFloatTimeLineRetriever_EdgeTravelTime::addEdgeWeight")
    }

}

/// Obtains edge weights from a weights handler and stores them within the edges
/// C++ class `ROLoader::EdgeFloatTimeLineRetriever_EdgeWeight` (router/ROLoader.h)
pub struct ROLoader_EdgeFloatTimeLineRetriever_EdgeWeight {
    /// C++ base class `SAXWeightsHandler::EdgeFloatTimeLineRetriever`
    pub base: Unmapped /* SAXWeightsHandler::EdgeFloatTimeLineRetriever */,
    /// The network edges shall be obtained from
    pub myNet: Ptr<RONet>,
}

impl std::ops::Deref for ROLoader_EdgeFloatTimeLineRetriever_EdgeWeight {
    type Target = Unmapped /* SAXWeightsHandler::EdgeFloatTimeLineRetriever */;
    fn deref(&self) -> &Unmapped /* SAXWeightsHandler::EdgeFloatTimeLineRetriever */ {
        &self.base
    }
}
impl std::ops::DerefMut for ROLoader_EdgeFloatTimeLineRetriever_EdgeWeight {
    fn deref_mut(&mut self) -> &mut Unmapped /* SAXWeightsHandler::EdgeFloatTimeLineRetriever */ {
        &mut self.base
    }
}

impl ROLoader_EdgeFloatTimeLineRetriever_EdgeWeight {
    /// C++: `void EdgeFloatTimeLineRetriever_EdgeWeight::EdgeFloatTimeLineRetriever_EdgeWeight(RONet &)`
    /// Constructor
    // EdgeFloatTimeLineRetriever_EdgeWeight(RONet& net) : myNet(net) {}
    #[allow(unused_variables)]
    pub fn new(net: &mut RONet) -> Self {
        todo!("port EdgeFloatTimeLineRetriever_EdgeWeight::EdgeFloatTimeLineRetriever_EdgeWeight")
    }

    /// C++: `void EdgeFloatTimeLineRetriever_EdgeWeight::~EdgeFloatTimeLineRetriever_EdgeWeight() noexcept` [virtual]
    /// Destructor
    // ~EdgeFloatTimeLineRetriever_EdgeWeight() { }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port EdgeFloatTimeLineRetriever_EdgeWeight::~EdgeFloatTimeLineRetriever_EdgeWeight")
    }

    /// C++: `void EdgeFloatTimeLineRetriever_EdgeWeight::addEdgeWeight(const std::string &, double, double, double) const` [const, virtual]
    /// Adds an effort for a given edge and time period
    // void
    // ROLoader::EdgeFloatTimeLineRetriever_EdgeWeight::addEdgeWeight(const std::string& id,
    //         double val, double beg, double end) const {
    //     ROEdge* e = myNet.getEdge(id);
    //     if (e != nullptr) {
    //         e->addEffort(val, beg, end);
    //     } else {
    //         if (id[0] != ':') {
    //             if (OptionsCont::getOptions().getBool("ignore-errors")) {
    //                 WRITE_WARNINGF(TL("Trying to set a weight for the unknown edge '%'."), id);
    //             } else {
    //                 WRITE_ERRORF(TL("Trying to set a weight for the unknown edge '%'."), id);
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_edge_weight(&self, id: &str, val: f64, beg: f64, end: f64) {
        todo!("port EdgeFloatTimeLineRetriever_EdgeWeight::addEdgeWeight")
    }

}

/// The data loader.
/// C++ class `ROLoader` (router/ROLoader.h)
pub struct ROLoader {
    /// Options to use
    pub myOptions: Ptr<OptionsCont>,
    /// Information whether empty destinations are allowed
    pub myEmptyDestinationsAllowed: bool,
    /// Information whether the routing steps should be logged
    pub myLogSteps: bool,
    /// List of route loaders
    pub myLoaders: SUMORouteLoaderControl,
}

impl ROLoader {
    /// C++: `void ROLoader::ROLoader(OptionsCont &, const bool, const bool)`
    /// Constructor
    // ROLoader::ROLoader(OptionsCont& oc, const bool emptyDestinationsAllowed, const bool logSteps) :
    //     myOptions(oc),
    //     myEmptyDestinationsAllowed(emptyDestinationsAllowed),
    //     myLogSteps(logSteps),
    //     myLoaders(oc.exists("unsorted-input") && oc.getBool("unsorted-input") ? 0 : DELTA_T) {
    // }
    #[allow(unused_variables)]
    pub fn new(oc: &mut OptionsCont, emptyDestinationsAllowed: bool, logSteps: bool) -> Self {
        todo!("port ROLoader::ROLoader")
    }

    /// C++: `void ROLoader::~ROLoader()` [virtual]
    /// Destructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port ROLoader::~ROLoader")
    }

    /// C++: `void ROLoader::loadNet(RONet &, ROAbstractEdgeBuilder &)` [virtual]
    /// Loads the network
    // void
    // ROLoader::loadNet(RONet& toFill, ROAbstractEdgeBuilder& eb) {
    //     std::string file = myOptions.getString("net-file");
    //     if (file == "") {
    //         throw ProcessError(TL("Missing definition of network to load!"));
    //     }
    //     if (!FileHelpers::isReadable(file)) {
    //         throw ProcessError(TLF("The network file '%' is not accessible.", file));
    //     }
    //     PROGRESS_BEGIN_MESSAGE(TL("Loading net"));
    //     RONetHandler handler(toFill, eb, !myOptions.exists("no-internal-links") || myOptions.getBool("no-internal-links"),
    //                          myOptions.exists("weights.minor-penalty") ? myOptions.getFloat("weights.minor-penalty") : 0,
    //                          myOptions.exists("weights.tls-penalty") ? myOptions.getFloat("weights.tls-penalty") : 0,
    //                          myOptions.exists("weights.turnaround-penalty") ? myOptions.getFloat("weights.turnaround-penalty") : 0);
    //     handler.setFileName(file);
    //     if (!XMLSubSys::runParser(handler, file, true)) {
    //         PROGRESS_FAILED_MESSAGE();
    //         throw ProcessError();
    //     } else {
    //         PROGRESS_DONE_MESSAGE();
    //     }
    //     if (!deprecatedVehicleClassesSeen.empty()) {
    //         WRITE_WARNINGF(TL("Deprecated vehicle classes '%' in input network."), toString(deprecatedVehicleClassesSeen));
    //         deprecatedVehicleClassesSeen.clear();
    //     }
    //     if (myOptions.isSet("additional-files", false)) { // dfrouter does not register this option
    //         std::vector<std::string> files = myOptions.getStringVector("additional-files");
    //         for (std::vector<std::string>::const_iterator fileIt = files.begin(); fileIt != files.end(); ++fileIt) {
    //             if (!FileHelpers::isReadable(*fileIt)) {
    //                 throw ProcessError(TLF("The additional file '%' is not accessible.", *fileIt));
    //             }
    //             PROGRESS_BEGIN_MESSAGE("Loading additional file '" + *fileIt + "' ");
    //             handler.setFileName(*fileIt);
    //             if (!XMLSubSys::runParser(handler, *fileIt)) {
    //                 PROGRESS_FAILED_MESSAGE();
    //                 throw ProcessError();
    //             } else {
    //                 PROGRESS_DONE_MESSAGE();
    //             }
    //         }
    //     }
    //     if (myOptions.exists("junction-taz") && myOptions.getBool("junction-taz")) {
    //         // create a TAZ for every junction
    //         toFill.addJunctionTaz(eb);
    //     }
    //     toFill.setBidiEdges(handler.getBidiMap());
    //     if (myOptions.exists("restriction-params") && myOptions.isSet("restriction-params")) {
    //         toFill.setParamRestrictions();
    //         const std::vector<std::string> paramKeys = myOptions.getStringVector("restriction-params");
    //         for (auto& edgeIt : toFill.getEdgeMap()) {
    //             edgeIt.second->cacheParamRestrictions(paramKeys);
    //         }
    //     }
    //     if (toFill.hasSpeedRestrictions()) {
    //         for (auto& edgeIt : toFill.getEdgeMap()) {
    //             edgeIt.second->setSpeedRestrictions(toFill.getRestrictions(edgeIt.second->getType()));
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn load_net(&mut self, toFill: &mut RONet, eb: &mut ROAbstractEdgeBuilder) {
        todo!("port ROLoader::loadNet")
    }

    /// C++: `bool ROLoader::loadWeights(RONet &, const std::string &, const std::string &, const bool, const bool)`
    /// Loads the net weights
    // bool
    // ROLoader::loadWeights(RONet& net, const std::string& optionName,
    //                       const std::string& measure, const bool useLanes, const bool boundariesOverride) {
    //     // check whether the file exists
    //     if (!myOptions.isUsableFileList(optionName)) {
    //         return false;
    //     }
    //     // build and prepare the weights handler
    //     std::vector<SAXWeightsHandler::ToRetrieveDefinition*> retrieverDefs;
    //     //  travel time, first (always used)
    //     EdgeFloatTimeLineRetriever_EdgeTravelTime ttRetriever(net);
    //     retrieverDefs.push_back(new SAXWeightsHandler::ToRetrieveDefinition("traveltime", !useLanes, ttRetriever));
    //     //  the measure to use, then
    //     EdgeFloatTimeLineRetriever_EdgeWeight eRetriever(net);
    //     if (measure != "traveltime") {
    //         std::string umeasure = measure;
    //         if (measure == "CO" || measure == "CO2" || measure == "HC" || measure == "PMx" || measure == "NOx" || measure == "fuel" || measure == "electricity") {
    //             umeasure = measure + "_perVeh";
    //         }
    //         retrieverDefs.push_back(new SAXWeightsHandler::ToRetrieveDefinition(umeasure, !useLanes, eRetriever));
    //     }
    //     //  set up handler
    //     SAXWeightsHandler handler(retrieverDefs, "");
    //     // go through files
    //     std::vector<std::string> files = myOptions.getStringVector(optionName);
    //     for (std::vector<std::string>::const_iterator fileIt = files.begin(); fileIt != files.end(); ++fileIt) {
    //         PROGRESS_BEGIN_MESSAGE("Loading precomputed net weights from '" + *fileIt + "'");
    //         if (XMLSubSys::runParser(handler, *fileIt)) {
    //             PROGRESS_DONE_MESSAGE();
    //         } else {
    //             WRITE_MESSAGE(TL("failed."));
    //             return false;
    //         }
    //     }
    //     // build edge-internal time lines
    //     for (const auto& i : net.getEdgeMap()) {
    //         i.second->buildTimeLines(measure, boundariesOverride);
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn load_weights(&mut self, net: &mut RONet, optionName: &str, measure: &str, useLanes: bool, boundariesOverride: bool) -> bool {
        todo!("port ROLoader::loadWeights")
    }

    /// C++: `void ROLoader::openRoutes(RONet &)`
    /// Builds and opens all route loaders
    // void
    // ROLoader::openRoutes(RONet& net) {
    //     // build loader
    //     // load relevant elements from additional file
    //     bool ok = openTypedRoutes("additional-files", net, true);
    //     // load sumo routes, trips, and flows
    //     ok &= openTypedRoutes("route-files", net);
    //     // check
    //     if (ok) {
    //         myLoaders.loadNext(string2time(myOptions.getString("begin")));
    //         if (!net.furtherStored()) {
    //             if (MsgHandler::getErrorInstance()->wasInformed()) {
    //                 throw ProcessError();
    //             } else if (!net.haveSkippedRoutes()) {
    //                 const std::string error = "No route input specified or all routes were invalid.";
    //                 if (myOptions.getBool("ignore-errors")) {
    //                     WRITE_WARNING(error);
    //                 } else {
    //                     throw ProcessError(error);
    //                 }
    //             }
    //         }
    //         // skip routes prior to the begin time
    //         if (!myOptions.getBool("unsorted-input")) {
    //             WRITE_MESSAGE("Skipped until: " + time2string(myLoaders.getFirstLoadTime()));
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn open_routes(&mut self, net: &mut RONet) {
        todo!("port ROLoader::openRoutes")
    }

    /// C++: `void ROLoader::processRoutes(const SUMOTime, const SUMOTime, const SUMOTime, RONet &, const RORouterProvider &)`
    /// Loads routes from all previously build route loaders
    // void
    // ROLoader::processRoutes(const SUMOTime start, const SUMOTime end, const SUMOTime increment,
    //                         RONet& net, const RORouterProvider& provider) {
    //     const SUMOTime absNo = end - start;
    //     const bool endGiven = !OptionsCont::getOptions().isDefault("end");
    //     // skip routes that begin before the simulation's begin
    //     // loop till the end
    //     const SUMOTime firstStep = myLoaders.getFirstLoadTime();
    //     SUMOTime lastStep = firstStep;
    //     SUMOTime time = MIN2(firstStep, end);
    //     while (time <= end) {
    //         writeStats(time, start, absNo, endGiven);
    //         myLoaders.loadNext(time);
    //         if (!net.furtherStored() || MsgHandler::getErrorInstance()->wasInformed()) {
    //             break;
    //         }
    //         lastStep = net.saveAndRemoveRoutesUntil(myOptions, provider, time);
    //         if (time == end || (!net.furtherStored() && myLoaders.haveAllLoaded()) || MsgHandler::getErrorInstance()->wasInformed()) {
    //             break;
    //         }
    //         if (time < end && time > end - increment) {
    //             time = end;
    //         } else {
    //             time += increment;
    //         }
    //     }
    //     if (myLogSteps) {
    //         WRITE_MESSAGEF(TL("Routes found between time steps % and %."), time2string(firstStep), time2string(lastStep));
    //     }
    //     if (!net.haveProcessedRoutes() && (net.haveSkippedRoutes() || net.furtherStored())
    //             && !MsgHandler::getErrorInstance()->wasInformed()) {
    //         WRITE_WARNINGF(TL("No routes found between begin time % and end time %."), time2string(start), time2string(end));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn process_routes(&mut self, start: SUMOTime, end: SUMOTime, increment: SUMOTime, net: &mut RONet, provider: &RORouterProvider) {
        todo!("port ROLoader::processRoutes")
    }

    /// C++: `SUMORouteHandler * ROLoader::getRouteHandler()`
    // SUMORouteHandler*
    // ROLoader::getRouteHandler() {
    //     auto loader = myLoaders.getFirstLoader();
    //     if (loader != nullptr) {
    //         return loader->getRouteHandler();
    //     } else {
    //         return nullptr;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_route_handler(&mut self) -> Ptr<SUMORouteHandler> {
        todo!("port ROLoader::getRouteHandler")
    }

    /// C++: `bool ROLoader::openTypedRoutes(const std::string &, RONet &, const bool)` [protected]
    /// Opens route handler of the given type
    // bool
    // ROLoader::openTypedRoutes(const std::string& optionName,
    //                           RONet& net, const bool readAll) {
    //     // check whether the current loader is wished
    //     //  and the file(s) can be used
    //     if (!myOptions.isUsableFileList(optionName)) {
    //         return !myOptions.isSet(optionName);
    //     }
    //     for (const std::string& fileIt : myOptions.getStringVector(optionName)) {
    //         try {
    //             RORouteHandler* handler = new RORouteHandler(net, fileIt, myOptions.getBool("repair"), myEmptyDestinationsAllowed, myOptions.getBool("ignore-errors"), !readAll);
    //             if (readAll) {
    //                 if (!XMLSubSys::runParser(*handler, fileIt)) {
    //                     WRITE_ERRORF(TL("Loading of % failed."), fileIt);
    //                     return false;
    //                 }
    //                 delete handler;
    //             } else {
    //                 myLoaders.add(new SUMORouteLoader(handler));
    //             }
    //         } catch (ProcessError& e) {
    //             WRITE_ERRORF(TL("The loader for % from file '%' could not be initialised (%)."), optionName, fileIt, e.what());
    //             return false;
    //         }
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn open_typed_routes(&mut self, optionName: &str, net: &mut RONet, readAll: bool) -> bool {
        todo!("port ROLoader::openTypedRoutes")
    }

    /// C++: `void ROLoader::writeStats(const SUMOTime, const SUMOTime, const SUMOTime, bool)` [protected]
    // void
    // ROLoader::writeStats(const SUMOTime time, const SUMOTime start, const SUMOTime absNo, bool endGiven) {
    //     if (myLogSteps) {
    //         if (endGiven) {
    //             const double perc = (double)(time - start) / (double) absNo;
    //             std::cout << "Reading up to time step: " + time2string(time) + "  (" + time2string(time - start) + "/" + time2string(absNo) + " = " + toString(perc * 100) + "% done)       \r";
    //         } else {
    //             std::cout << "Reading up to time step: " + time2string(time) + "\r";
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn write_stats(&mut self, time: SUMOTime, start: SUMOTime, absNo: SUMOTime, endGiven: bool) {
        todo!("port ROLoader::writeStats")
    }

    /// C++: `void ROLoader::ROLoader(const ROLoader &)` [private]
    /// Invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(src: &ROLoader) -> Self {
        todo!("port ROLoader::ROLoader")
    }

    /// C++: `ROLoader & ROLoader::operator=(const ROLoader &)` [private]
    /// Invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, src: &ROLoader) -> &mut ROLoader {
        todo!("port ROLoader::operator=")
    }

}

