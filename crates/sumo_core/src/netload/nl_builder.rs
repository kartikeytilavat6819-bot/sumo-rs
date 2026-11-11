//! Port of netload/NLBuilder.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// Obtains edge efforts from a weights handler and stores them within the edges
/// C++ class `NLBuilder::EdgeFloatTimeLineRetriever_EdgeEffort` (netload/NLBuilder.h)
pub struct NLBuilder_EdgeFloatTimeLineRetriever_EdgeEffort {
    /// C++ base class `SAXWeightsHandler::EdgeFloatTimeLineRetriever`
    pub base: Unmapped /* SAXWeightsHandler::EdgeFloatTimeLineRetriever */,
    /// The network edges shall be obtained from
    pub myNet: Ptr<MSNet>,
}

impl std::ops::Deref for NLBuilder_EdgeFloatTimeLineRetriever_EdgeEffort {
    type Target = Unmapped /* SAXWeightsHandler::EdgeFloatTimeLineRetriever */;
    fn deref(&self) -> &Unmapped /* SAXWeightsHandler::EdgeFloatTimeLineRetriever */ {
        &self.base
    }
}
impl std::ops::DerefMut for NLBuilder_EdgeFloatTimeLineRetriever_EdgeEffort {
    fn deref_mut(&mut self) -> &mut Unmapped /* SAXWeightsHandler::EdgeFloatTimeLineRetriever */ {
        &mut self.base
    }
}

impl NLBuilder_EdgeFloatTimeLineRetriever_EdgeEffort {
    /// C++: `void EdgeFloatTimeLineRetriever_EdgeEffort::EdgeFloatTimeLineRetriever_EdgeEffort(MSNet &)`
    /// Constructor
    // EdgeFloatTimeLineRetriever_EdgeEffort(MSNet& net) : myNet(net) {}
    #[allow(unused_variables)]
    pub fn new(net: &mut MSNet) -> Self {
        todo!("port EdgeFloatTimeLineRetriever_EdgeEffort::EdgeFloatTimeLineRetriever_EdgeEffort")
    }

    /// C++: `void EdgeFloatTimeLineRetriever_EdgeEffort::~EdgeFloatTimeLineRetriever_EdgeEffort() noexcept` [virtual]
    /// Destructor
    // ~EdgeFloatTimeLineRetriever_EdgeEffort() { }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port EdgeFloatTimeLineRetriever_EdgeEffort::~EdgeFloatTimeLineRetriever_EdgeEffort")
    }

    /// C++: `void EdgeFloatTimeLineRetriever_EdgeEffort::addEdgeWeight(const std::string &, double, double, double) const` [const, virtual]
    /// Adds an effort for a given edge and time period
    // void
    // NLBuilder::EdgeFloatTimeLineRetriever_EdgeEffort::addEdgeWeight(const std::string& id,
    //         double value, double begTime, double endTime) const {
    //     MSEdge* edge = MSEdge::dictionary(id);
    //     if (edge != nullptr) {
    //         myNet.getWeightsStorage().addEffort(edge, begTime, endTime, value);
    //     } else {
    //         WRITE_ERRORF(TL("Trying to set the effort for the unknown edge '%'."), id);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_edge_weight(&self, id: &str, val: f64, beg: f64, end: f64) {
        todo!("port EdgeFloatTimeLineRetriever_EdgeEffort::addEdgeWeight")
    }

}

/// Obtains edge travel times from a weights handler and stores them within the edges
/// C++ class `NLBuilder::EdgeFloatTimeLineRetriever_EdgeTravelTime` (netload/NLBuilder.h)
pub struct NLBuilder_EdgeFloatTimeLineRetriever_EdgeTravelTime {
    /// C++ base class `SAXWeightsHandler::EdgeFloatTimeLineRetriever`
    pub base: Unmapped /* SAXWeightsHandler::EdgeFloatTimeLineRetriever */,
    /// The network edges shall be obtained from
    pub myNet: Ptr<MSNet>,
}

impl std::ops::Deref for NLBuilder_EdgeFloatTimeLineRetriever_EdgeTravelTime {
    type Target = Unmapped /* SAXWeightsHandler::EdgeFloatTimeLineRetriever */;
    fn deref(&self) -> &Unmapped /* SAXWeightsHandler::EdgeFloatTimeLineRetriever */ {
        &self.base
    }
}
impl std::ops::DerefMut for NLBuilder_EdgeFloatTimeLineRetriever_EdgeTravelTime {
    fn deref_mut(&mut self) -> &mut Unmapped /* SAXWeightsHandler::EdgeFloatTimeLineRetriever */ {
        &mut self.base
    }
}

impl NLBuilder_EdgeFloatTimeLineRetriever_EdgeTravelTime {
    /// C++: `void EdgeFloatTimeLineRetriever_EdgeTravelTime::EdgeFloatTimeLineRetriever_EdgeTravelTime(MSNet &)`
    /// Constructor
    // EdgeFloatTimeLineRetriever_EdgeTravelTime(MSNet& net) : myNet(net) {}
    #[allow(unused_variables)]
    pub fn new(net: &mut MSNet) -> Self {
        todo!("port EdgeFloatTimeLineRetriever_EdgeTravelTime::EdgeFloatTimeLineRetriever_EdgeTravelTime")
    }

    /// C++: `void EdgeFloatTimeLineRetriever_EdgeTravelTime::~EdgeFloatTimeLineRetriever_EdgeTravelTime() noexcept` [virtual]
    /// Destructor
    // ~EdgeFloatTimeLineRetriever_EdgeTravelTime() { }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port EdgeFloatTimeLineRetriever_EdgeTravelTime::~EdgeFloatTimeLineRetriever_EdgeTravelTime")
    }

    /// C++: `void EdgeFloatTimeLineRetriever_EdgeTravelTime::addEdgeWeight(const std::string &, double, double, double) const` [const, virtual]
    /// Adds a travel time for a given edge and time period
    // void
    // NLBuilder::EdgeFloatTimeLineRetriever_EdgeTravelTime::addEdgeWeight(const std::string& id,
    //         double value, double begTime, double endTime) const {
    //     MSEdge* edge = MSEdge::dictionary(id);
    //     if (edge != nullptr) {
    //         myNet.getWeightsStorage().addTravelTime(edge, begTime, endTime, value);
    //     } else {
    //         WRITE_ERRORF(TL("Trying to set the travel time for the unknown edge '%'."), id);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_edge_weight(&self, id: &str, val: f64, beg: f64, end: f64) {
        todo!("port EdgeFloatTimeLineRetriever_EdgeTravelTime::addEdgeWeight")
    }

}

/// The main interface for loading a microsim
/// C++ class `NLBuilder` (netload/NLBuilder.h)
pub struct NLBuilder {
    /// The options to get the names of the files to load and further information from
    pub myOptions: Ptr<OptionsCont>,
    /// The edge control builder to use
    pub myEdgeBuilder: Ptr<NLEdgeControlBuilder>,
    /// The junction control builder to use
    pub myJunctionBuilder: Ptr<NLJunctionControlBuilder>,
    /// The detector control builder to use
    pub myDetectorBuilder: Ptr<NLDetectorBuilder>,
    /// The net to fill
    pub myNet: Ptr<MSNet>,
    /// The handler used to parse the net
    pub myXMLHandler: Ptr<NLHandler>,
}

impl NLBuilder {
    /// C++: `void NLBuilder::NLBuilder(OptionsCont &, MSNet &, NLEdgeControlBuilder &, NLJunctionControlBuilder &, NLDetectorBuilder &, NLHandler &)`
    /// Constructor
    // NLBuilder::NLBuilder(OptionsCont& oc,
    //                      MSNet& net,
    //                      NLEdgeControlBuilder& eb,
    //                      NLJunctionControlBuilder& jb,
    //                      NLDetectorBuilder& db,
    //                      NLHandler& xmlHandler)
    //     : myOptions(oc), myEdgeBuilder(eb), myJunctionBuilder(jb),
    //       myDetectorBuilder(db),
    //       myNet(net), myXMLHandler(xmlHandler) {}
    #[allow(unused_variables)]
    pub fn new(oc: &mut OptionsCont, net: &mut MSNet, eb: &mut NLEdgeControlBuilder, jb: &mut NLJunctionControlBuilder, db: &mut NLDetectorBuilder, xmlHandler: &mut NLHandler) -> Self {
        todo!("port NLBuilder::NLBuilder")
    }

    /// C++: `void NLBuilder::~NLBuilder()` [virtual]
    /// Destructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port NLBuilder::~NLBuilder")
    }

    /// C++: `bool NLBuilder::build()` [virtual]
    /// Builds and initialises the simulation
    // bool
    // NLBuilder::build() {
    //     // try to build the net
    //     if (!load("net-file", true)) {
    //         return false;
    //     }
    //     if (myXMLHandler.networkVersion() == MMVersion(0, 0)) {
    //         throw ProcessError(TL("Invalid network, no network version declared."));
    //     }
    //     // check whether the loaded net agrees with the simulation options
    //     if ((myOptions.getBool("no-internal-links")) && myXMLHandler.haveSeenInternalEdge() && myXMLHandler.haveSeenDefaultLength()) {
    //         WRITE_WARNING(TL("Network contains internal links which are ignored. Vehicles will 'jump' across junctions and thus underestimate route lengths and travel times."));
    //     }
    //     if (!myXMLHandler.haveSeenInternalEdge() && myOptions.getBool("mesosim")) {
    //         // @todo: setting this option has some side effect in microsim that manifest even in networks without internal lanes this should be checked
    //         MSGlobals::gUsingInternalLanes = false;
    //     }
    //     buildNet();
    //     if (myOptions.isSet("alternative-net-file")) {
    //         for (std::string fname : myOptions.getStringVector("alternative-net-file")) {
    //             const long before = PROGRESS_BEGIN_TIME_MESSAGE("Loading alternative net from '" + fname + "'");
    //             NLNetShapeHandler nsh(fname, myNet);
    //             if (!XMLSubSys::runParser(nsh, fname, true)) {
    //                 WRITE_MESSAGE("Loading of alternative net failed.");
    //                 return false;
    //             }
    //             nsh.sortInternalShapes();
    //             PROGRESS_TIME_MESSAGE(before);
    //         }
    //     }
    //     // @note on loading order constraints:
    //     // - additional-files before route-files and state-files due to referencing
    //     // - additional-files before weight-files since the latter might contain intermodal edge data and the intermodal net depends on the stops and public transport from the additionals
    //
    //     bool stateBeginMismatch = false;
    //     const SUMOTime stateOffset = string2time(myOptions.getString("load-state.offset"));
    //     if (myOptions.isSet("load-state")) {
    //         // first, load only the time
    //         const SUMOTime stateTime = MSStateHandler::MSStateTimeHandler::getTime(myOptions.getString("load-state"));
    //         bool defaultBegin = myOptions.isDefault("begin");
    //         if (stateTime != string2time(myOptions.getString("begin"))) {
    //             if (myOptions.isDefault("load-state.offset")) {
    //                 if (!defaultBegin) {
    //                     WRITE_WARNINGF(TL("State was written at a different time=% than the begin time % (set option load-state.offset to force a different begin)!"),
    //                                    time2string(stateTime), myOptions.getString("begin"));
    //                 }
    //                 myOptions.resetWritable("begin");
    //                 defaultBegin = true;
    //             } else {
    //                 stateBeginMismatch = true;
    //             }
    //         }
    //         if (defaultBegin) {
    //             myOptions.set("begin", time2string(stateTime));
    //             // if vehicles are deliberately removed from the state, we must allow them to be reloaded from a route file at state time
    //             SUMOTime allowReloadOffset = myOptions.isDefault("load-state.remove-vehicles") ? 0 : 1;
    //             myNet.setLoaderTime(stateTime - allowReloadOffset);
    //             if (TraCIServer::getInstance() != nullptr) {
    //                 TraCIServer::getInstance()->stateLoaded(stateTime);
    //             }
    //         }
    //         myNet.setCurrentTimeStep(string2time(myOptions.getString("begin")));
    //     }
    //
    //     if (myOptions.getBool("junction-taz")) {
    //         // create a TAZ for every junction
    //         const MSJunctionControl& junctions = myNet.getJunctionControl();
    //         for (auto it = junctions.begin(); it != junctions.end(); it++) {
    //             const std::string sinkID = it->first + "-sink";
    //             const std::string sourceID = it->first + "-source";
    //             if (MSEdge::dictionary(sinkID) == nullptr && MSEdge::dictionary(sourceID) == nullptr) {
    //                 // sink must be built and added before source
    //                 MSEdge* sink = myEdgeBuilder.buildEdge(sinkID, SumoXMLEdgeFunc::CONNECTOR, "", "", "", -1, 0);
    //                 MSEdge* source = myEdgeBuilder.buildEdge(sourceID, SumoXMLEdgeFunc::CONNECTOR, "", "", "", -1, 0);
    //                 sink->setOtherTazConnector(source);
    //                 source->setOtherTazConnector(sink);
    //                 MSEdge::dictionary(sinkID, sink);
    //                 MSEdge::dictionary(sourceID, source);
    //                 sink->initialize(new std::vector<MSLane*>());
    //                 source->initialize(new std::vector<MSLane*>());
    //                 const MSJunction* junction = it->second;
    //                 for (const MSEdge* edge : junction->getIncoming()) {
    //                     if (!edge->isInternal()) {
    //                         const_cast<MSEdge*>(edge)->addSuccessor(sink);
    //                     }
    //                 }
    //                 for (const MSEdge* edge : junction->getOutgoing()) {
    //                     if (!edge->isInternal()) {
    //                         source->addSuccessor(const_cast<MSEdge*>(edge));
    //                     }
    //                 }
    //             } else {
    //                 WRITE_WARNINGF(TL("A TAZ with id '%' already exists. Not building junction TAZ."), it->first)
    //             }
    //         }
    //     }
    //     // load meso edge types
    //     bool haveSeenMesoEdgeType = false;
    //     if (MSGlobals::gUseMesoSim) {
    //         if (myOptions.isSet("additional-files") && !myOptions.getBool("ignore.additionalelements")) {
    //             haveSeenMesoEdgeType = loadMesoEdgeTypes("additional-files");
    //         }
    //         // meso segment building must be delayed until meso edge types have been read from additional files
    //         myNet.getEdgeControl().buildMesoSegments();
    //     }
    //     // load additional net elements (sources, detectors, ...)
    //     if (myOptions.isSet("additional-files") && !myOptions.getBool("ignore.additionalelements")) {
    //         if (!load("additional-files")) {
    //             return false;
    //         }
    //         // load shapes with separate handler
    //         NLShapeHandler sh("", myNet.getShapeContainer());
    //         if (!ShapeHandler::loadFiles(myOptions.getStringVector("additional-files"), sh)) {
    //             return false;
    //         }
    //         if (myXMLHandler.haveSeenAdditionalSpeedRestrictions()) {
    //             myNet.getEdgeControl().setAdditionalRestrictions();
    //         }
    //         MSTriggeredRerouter::checkParkingRerouteConsistency();
    //     }
    //     if (MSGlobals::gUseMesoSim) {
    //         if (haveSeenMesoEdgeType || myXMLHandler.haveSeenTLSParams()) {
    //             for (MSTrafficLightLogic* tll : myNet.getTLSControl().getAllLogics()) {
    //                 tll->initMesoTLSPenalties();
    //             }
    //         }
    //     }
    //     // init after preferences have been loaded from additional-files
    //     MSRoutingEngine::initWeightConstants(myOptions);
    //     // init tls after all detectors have been loaded
    //     myJunctionBuilder.postLoadInitialization();
    //     // declare meandata set by options
    //     buildDefaultMeanData("edgedata-output", "DEFAULT_EDGEDATA", false);
    //     buildDefaultMeanData("lanedata-output", "DEFAULT_LANEDATA", true);
    //
    //     if (stateBeginMismatch && myNet.getVehicleControl().getLoadedVehicleNo() > 0) {
    //         throw ProcessError(TL("Loading vehicles ahead of a state file is not supported. Correct --begin option or load vehicles with option --route-files"));
    //     }
    //
    //     // load weights if wished
    //     if (myOptions.isSet("weight-files")) {
    //         if (!myOptions.isUsableFileList("weight-files")) {
    //             return false;
    //         }
    //         // build and prepare the weights handler
    //         std::vector<SAXWeightsHandler::ToRetrieveDefinition*> retrieverDefs;
    //         //  travel time, first (always used)
    //         EdgeFloatTimeLineRetriever_EdgeTravelTime ttRetriever(myNet);
    //         retrieverDefs.push_back(new SAXWeightsHandler::ToRetrieveDefinition("traveltime", true, ttRetriever));
    //         //  the measure to use, then
    //         EdgeFloatTimeLineRetriever_EdgeEffort eRetriever(myNet);
    //         std::string measure = myOptions.getString("weight-attribute");
    //         if (!myOptions.isDefault("weight-attribute")) {
    //             if (measure == "CO" || measure == "CO2" || measure == "HC" || measure == "PMx" || measure == "NOx" || measure == "fuel" || measure == "electricity") {
    //                 measure += "_perVeh";
    //             }
    //             retrieverDefs.push_back(new SAXWeightsHandler::ToRetrieveDefinition(measure, true, eRetriever));
    //         }
    //         //  set up handler
    //         SAXWeightsHandler handler(retrieverDefs, "");
    //         // start parsing; for each file in the list
    //         std::vector<std::string> files = myOptions.getStringVector("weight-files");
    //         for (std::vector<std::string>::iterator i = files.begin(); i != files.end(); ++i) {
    //             // report about loading when wished
    //             WRITE_MESSAGEF(TL("Loading weights from '%'..."), *i);
    //             // parse the file
    //             if (!XMLSubSys::runParser(handler, *i)) {
    //                 return false;
    //             }
    //         }
    //     }
    //     // load the previous state if wished
    //     if (myOptions.isSet("load-state")) {
    //         const std::string& f = myOptions.getString("load-state");
    //         long before = PROGRESS_BEGIN_TIME_MESSAGE(TLF("Loading state from '%'", f));
    //         MSStateHandler h(f, stateOffset);
    //         XMLSubSys::runParser(h, f);
    //         if (MsgHandler::getErrorInstance()->wasInformed()) {
    //             return false;
    //         }
    //         PROGRESS_TIME_MESSAGE(before);
    //     }
    //     // routes from FCD files
    //     MSDevice_FCDReplay::init();
    //     // load routes
    //     if (myOptions.isSet("route-files") && !myOptions.getBool("ignore.routeelements")) {
    //         if (string2time(myOptions.getString("route-steps")) <= 0) {
    //             // incremental loading is disabled. Load route files fully
    //             if (!load("route-files")) {
    //                 return false;
    //             }
    //         } else {
    //             // message must come after additional-files have been loaded (but buildRouteLoaderControl was called earlier)
    //             for (std::string file : myOptions.getStringVector("route-files")) {
    //                 WRITE_MESSAGE(TLF("Loading route-files incrementally from '%'", file));
    //             }
    //         }
    //     }
    //     // optionally switch off traffic lights
    //     if (myOptions.getBool("tls.all-off")) {
    //         myNet.getTLSControl().switchOffAll();
    //     }
    //     WRITE_MESSAGE(TL("Loading done."));
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn build(&mut self) -> bool {
        todo!("port NLBuilder::build")
    }

    /// C++: `MSNet * NLBuilder::init(const bool)` [static]
    /// loads the net, additional routes and the detectors
    // MSNet*
    // NLBuilder::init(const bool isLibsumo) {
    //     OptionsCont& oc = OptionsCont::getOptions();
    //     oc.clear();
    //     MSFrame::fillOptions();
    //     OptionsIO::getOptions();
    //     if (oc.processMetaOptions(OptionsIO::getArgC() < 2)) {
    //         SystemFrame::close();
    //         return nullptr;
    //     }
    //     SystemFrame::checkOptions(oc);
    //     std::string validation = oc.getString("xml-validation");
    //     std::string routeValidation = oc.getString("xml-validation.routes");
    //     if (isLibsumo) {
    //         if (oc.isDefault("xml-validation")) {
    //             validation = "never";
    //         }
    //         if (oc.isDefault("xml-validation.routes")) {
    //             routeValidation = "never";
    //         }
    //     }
    //     XMLSubSys::setValidation(validation, oc.getString("xml-validation.net"), routeValidation);
    //     if (!MSFrame::checkOptions()) {
    //         throw ProcessError();
    //     }
    // #ifdef HAVE_FOX
    //     if (oc.getInt("threads") > 1 || oc.getInt("device.rerouting.threads") > 1) {
    //         // make the output aware of threading
    //         MsgHandler::setFactory(&MsgHandlerSynchronized::create);
    //     }
    // #endif
    //     MsgHandler::initOutputOptions();
    //     initRandomness();
    //     MSFrame::setMSGlobals(oc);
    //     MSVehicleControl* vc = nullptr;
    //     if (MSGlobals::gUseMesoSim) {
    //         vc = new MEVehicleControl();
    //     } else {
    //         vc = new MSVehicleControl();
    //     }
    //     MSNet* net = new MSNet(vc, new MSEventControl(), new MSEventControl(), new MSEventControl());
    //     // need to init TraCI-Server before loading routes to catch VehicleState::BUILT
    //     TraCIServer::openSocket(std::map<int, TraCIServer::CmdExecutor>());
    //     if (isLibsumo) {
    //         libsumo::Helper::registerStateListener();
    //     }
    //
    //     NLEdgeControlBuilder eb;
    //     NLDetectorBuilder db(*net);
    //     NLJunctionControlBuilder jb(*net, db);
    //     NLTriggerBuilder tb;
    //     NLHandler handler("", *net, db, tb, eb, jb);
    //     tb.setHandler(&handler);
    //     NLBuilder builder(oc, *net, eb, jb, db, handler);
    //     MsgHandler::getErrorInstance()->clear();
    //     MsgHandler::getWarningInstance()->clear();
    //     MsgHandler::getMessageInstance()->clear();
    //     if (builder.build()) {
    //         // preload the routes especially for TraCI
    //         net->loadRoutes();
    //         return net;
    //     }
    //     delete net;
    //     throw ProcessError();
    // }
    #[allow(unused_variables)]
    pub fn init(isLibsumo: bool) -> Ptr<MSNet> {
        todo!("port NLBuilder::init")
    }

    /// C++: `void NLBuilder::initRandomness()` [static]
    /// initializes all RNGs
    // void
    // NLBuilder::initRandomness() {
    //     RandHelper::initRandGlobal();
    //     RandHelper::initRandGlobal(MSRouteHandler::getParsingRNG());
    //     RandHelper::initRandGlobal(MSDevice::getEquipmentRNG());
    //     RandHelper::initRandGlobal(OUProcess::getRNG());
    //     RandHelper::initRandGlobal(MSDevice_ToC::getResponseTimeRNG());
    //     RandHelper::initRandGlobal(MSDevice_BTreceiver::getRecognitionRNG());
    //     MSLane::initRNGs(OptionsCont::getOptions());
    // }
    #[allow(unused_variables)]
    pub fn init_randomness() {
        todo!("port NLBuilder::initRandomness")
    }

    /// C++: `SUMORouteLoaderControl * NLBuilder::buildRouteLoaderControl(const OptionsCont &)` [static]
    /// Builds the route loader control
    // SUMORouteLoaderControl*
    // NLBuilder::buildRouteLoaderControl(const OptionsCont& oc) {
    //     // build the loaders
    //     SUMORouteLoaderControl* loaders =  new SUMORouteLoaderControl(string2time(oc.getString("route-steps")));
    //     // check whether a list is existing
    //     if (oc.isSet("route-files") && !oc.getBool("ignore.routeelements") && string2time(oc.getString("route-steps")) > 0) {
    //         std::vector<std::string> files = oc.getStringVector("route-files");
    //         for (std::vector<std::string>::const_iterator fileIt = files.begin(); fileIt != files.end(); ++fileIt) {
    //             if (!FileHelpers::isReadable(*fileIt)) {
    //                 throw ProcessError(TLF("The route file '%' is not accessible.", *fileIt));
    //             }
    //         }
    //         // open files for reading
    //         for (std::vector<std::string>::const_iterator fileIt = files.begin(); fileIt != files.end(); ++fileIt) {
    //             loaders->add(new SUMORouteLoader(new MSRouteHandler(*fileIt, false)));
    //         }
    //     }
    //     return loaders;
    // }
    #[allow(unused_variables)]
    pub fn build_route_loader_control(oc: &OptionsCont) -> Ptr<SUMORouteLoaderControl> {
        todo!("port NLBuilder::buildRouteLoaderControl")
    }

    /// C++: `bool NLBuilder::load(const std::string &, const bool)` [protected]
    /// Loads a described subpart form the given list of files
    // bool
    // NLBuilder::load(const std::string& mmlWhat, const bool isNet) {
    //     if (!myOptions.isUsableFileList(mmlWhat)) {
    //         return false;
    //     }
    //     std::vector<std::string> files = myOptions.getStringVector(mmlWhat);
    //     for (std::vector<std::string>::const_iterator fileIt = files.begin(); fileIt != files.end(); ++fileIt) {
    //         const long before = PROGRESS_BEGIN_TIME_MESSAGE(TLF("Loading % from '%'", mmlWhat, *fileIt));
    //         if (!XMLSubSys::runParser(myXMLHandler, *fileIt, isNet)) {
    //             WRITE_MESSAGEF(TL("Loading of % failed."), mmlWhat);
    //             return false;
    //         }
    //         PROGRESS_TIME_MESSAGE(before);
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn load(&mut self, mmlWhat: &str, isNet: bool) -> bool {
        todo!("port NLBuilder::load")
    }

    /// C++: `bool NLBuilder::loadMesoEdgeTypes(const std::string &)` [protected]
    /// loads meso edge types with a dedicated handler and return whether any were loaded
    // bool
    // NLBuilder::loadMesoEdgeTypes(const std::string& mmlWhat) {
    //     if (!myOptions.isUsableFileList(mmlWhat)) {
    //         return false;
    //     }
    //     METypeHandler meTypeHandler("", myNet);
    //     for (const std::string& file : myOptions.getStringVector(mmlWhat)) {
    //         if (!XMLSubSys::runParser(meTypeHandler, file)) {
    //             continue;
    //         }
    //     }
    //     return meTypeHandler.haveSeenMesoEdgeType();
    // }
    #[allow(unused_variables)]
    pub fn load_meso_edge_types(&mut self, mmlWhat: &str) -> bool {
        todo!("port NLBuilder::loadMesoEdgeTypes")
    }

    /// C++: `void NLBuilder::buildNet()` [protected]
    /// Closes the net building process
    // void
    // NLBuilder::buildNet() {
    //     MSEdgeControl* edges = nullptr;
    //     MSJunctionControl* junctions = nullptr;
    //     SUMORouteLoaderControl* routeLoaders = nullptr;
    //     MSTLLogicControl* tlc = nullptr;
    //     std::vector<SUMOTime> stateDumpTimes;
    //     std::vector<std::string> stateDumpFiles;
    //     try {
    //         MSFrame::buildStreams(); // ensure streams are ready for output during building
    //         edges = myEdgeBuilder.build(myXMLHandler.networkVersion());
    //         junctions = myJunctionBuilder.build();
    //         junctions->postloadInitContainer();
    //         for (MSEdge* e : edges->getEdges()) {
    //             e->postLoadInitLaneChanger();
    //         }
    //         routeLoaders = buildRouteLoaderControl(myOptions);
    //         tlc = myJunctionBuilder.buildTLLogics();
    //         for (std::string timeStr : myOptions.getStringVector("save-state.times")) {
    //             stateDumpTimes.push_back(string2time(timeStr));
    //         }
    //         if (myOptions.isSet("save-state.files")) {
    //             stateDumpFiles = myOptions.getStringVector("save-state.files");
    //             if (stateDumpFiles.size() != stateDumpTimes.size()) {
    //                 throw ProcessError(TL("Wrong number of state file names!"));
    //             }
    //         } else {
    //             const std::string prefix = myOptions.getString("save-state.prefix");
    //             const std::string suffix = myOptions.getString("save-state.suffix");
    //             for (std::vector<SUMOTime>::iterator i = stateDumpTimes.begin(); i != stateDumpTimes.end(); ++i) {
    //                 std::string timeStamp = time2string(*i);
    //                 std::replace(timeStamp.begin(), timeStamp.end(), ':', '-');
    //                 stateDumpFiles.push_back(prefix + "_" + timeStamp + suffix);
    //             }
    //         }
    //     } catch (ProcessError&) {
    //         MSEdge::clear();
    //         MSLane::clear();
    //         delete edges;
    //         delete junctions;
    //         delete routeLoaders;
    //         delete tlc;
    //         throw;
    //     }
    //     // if anthing goes wrong after this point, the net is responsible for cleaning up
    //     myNet.closeBuilding(myOptions, edges, junctions, routeLoaders, tlc, stateDumpTimes, stateDumpFiles,
    //                         myXMLHandler.haveSeenInternalEdge(),
    //                         myXMLHandler.hasJunctionHigherSpeeds(),
    //                         myXMLHandler.networkVersion());
    // }
    #[allow(unused_variables)]
    pub fn build_net(&mut self) {
        todo!("port NLBuilder::buildNet")
    }

    /// C++: `void NLBuilder::buildDefaultMeanData(const std::string &, const std::string &, bool)` [protected]
    /// build meanData definition based on option
    // void
    // NLBuilder::buildDefaultMeanData(const std::string& optionName, const std::string& id, bool useLanes) {
    //     if (OptionsCont::getOptions().isSet(optionName)) {
    //         if (useLanes && MSGlobals::gUseMesoSim && !OptionsCont::getOptions().getBool("meso-lane-queue")) {
    //             WRITE_WARNING(TL("LaneData requested for mesoscopic simulation but --meso-lane-queue is not active. Falling back to edgeData."));
    //             useLanes = false;
    //         }
    //         try {
    //             SUMOTime begin = string2time(OptionsCont::getOptions().getString("begin"));
    //             myDetectorBuilder.createEdgeLaneMeanData(id, -1, begin, -1, "traffic", useLanes, "true",
    //                     false, false, 0, 100000, 0, SUMO_const_haltingSpeed, 0, "", "", std::vector<MSEdge*>(), AggregateType::NO,
    //                     OptionsCont::getOptions().getString(optionName));
    //         } catch (InvalidArgument& e) {
    //             WRITE_ERROR(e.what());
    //         } catch (IOError& e) {
    //             WRITE_ERROR(e.what());
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn build_default_mean_data(&mut self, optionName: &str, id: &str, useLanes: bool) {
        todo!("port NLBuilder::buildDefaultMeanData")
    }

    /// C++: `void NLBuilder::NLBuilder(const NLBuilder &)` [private]
    /// invalidated copy operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(s: &NLBuilder) -> Self {
        todo!("port NLBuilder::NLBuilder")
    }

    /// C++: `NLBuilder & NLBuilder::operator=(const NLBuilder &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &NLBuilder) -> &mut NLBuilder {
        todo!("port NLBuilder::operator=")
    }

}

