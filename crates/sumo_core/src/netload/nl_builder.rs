//! Port of netload/NLBuilder.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::msg_handler::{with_instance, write_error, write_message, write_warning};

/// PROGRESS_BEGIN_TIME_MESSAGE(msg): prints the message and returns the start time
fn progress_begin_time_message(msg: &str) -> i64 {
    utils::common::msg_handler::progress_begin_message(msg);
    current_millis()
}

/// PROGRESS_TIME_MESSAGE(before)
fn progress_time_since(before: i64) {
    utils::common::msg_handler::progress_time_message(current_millis() - before);
}

fn current_millis() -> i64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as i64).unwrap_or(0)
}

/// Obtains edge efforts from a weights handler and stores them within the edges
/// C++ class `NLBuilder::EdgeFloatTimeLineRetriever_EdgeEffort` (netload/NLBuilder.h)
#[repr(C)]
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
    pub fn new(net: Ptr<MSNet>) -> Self {
        NLBuilder_EdgeFloatTimeLineRetriever_EdgeEffort { base: (), myNet: net }
    }

    /// C++: `void EdgeFloatTimeLineRetriever_EdgeEffort::~EdgeFloatTimeLineRetriever_EdgeEffort() noexcept` [virtual]
    /// Destructor
    // ~EdgeFloatTimeLineRetriever_EdgeEffort() { }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty; members are released by Rust drop glue
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
        let edge = MSEdge::dictionary_2(id);
        if !edge.is_null() {
            self.myNet.m().get_weights_storage().add_effort(edge, beg, end, val);
        } else {
            write_errorf!("Trying to set the effort for the unknown edge '%'.", id);
        }
    }

}

/// Obtains edge travel times from a weights handler and stores them within the edges
/// C++ class `NLBuilder::EdgeFloatTimeLineRetriever_EdgeTravelTime` (netload/NLBuilder.h)
#[repr(C)]
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
    pub fn new(net: Ptr<MSNet>) -> Self {
        NLBuilder_EdgeFloatTimeLineRetriever_EdgeTravelTime { base: (), myNet: net }
    }

    /// C++: `void EdgeFloatTimeLineRetriever_EdgeTravelTime::~EdgeFloatTimeLineRetriever_EdgeTravelTime() noexcept` [virtual]
    /// Destructor
    // ~EdgeFloatTimeLineRetriever_EdgeTravelTime() { }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty; members are released by Rust drop glue
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
        let edge = MSEdge::dictionary_2(id);
        if !edge.is_null() {
            self.myNet.m().get_weights_storage().add_travel_time(edge, beg, end, val);
        } else {
            write_errorf!("Trying to set the travel time for the unknown edge '%'.", id);
        }
    }

}

impl utils::xml::saxweights_handler::EdgeFloatTimeLineRetriever for NLBuilder_EdgeFloatTimeLineRetriever_EdgeEffort {
    fn add_edge_weight(&self, id: &str, val: f64, beg: f64, end: f64) {
        NLBuilder_EdgeFloatTimeLineRetriever_EdgeEffort::add_edge_weight(self, id, val, beg, end)
    }
}

impl utils::xml::saxweights_handler::EdgeFloatTimeLineRetriever for NLBuilder_EdgeFloatTimeLineRetriever_EdgeTravelTime {
    fn add_edge_weight(&self, id: &str, val: f64, beg: f64, end: f64) {
        NLBuilder_EdgeFloatTimeLineRetriever_EdgeTravelTime::add_edge_weight(self, id, val, beg, end)
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
        NLBuilder {
            myOptions: Ptr::from_mut(oc),
            myEdgeBuilder: Ptr::from_mut(eb),
            myJunctionBuilder: Ptr::from_mut(jb),
            myDetectorBuilder: Ptr::from_mut(db),
            myNet: Ptr::from_mut(net),
            myXMLHandler: Ptr::from_mut(xmlHandler),
        }
    }

    /// C++: `void NLBuilder::~NLBuilder()` [virtual]
    /// Destructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // implicit C++ destructor: nothing to release, the Ptr members are non-owning
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
        // try to build the net
        if !self.load("net-file", true) {
            return false;
        }
        if self.myXMLHandler.network_version() == (0, 0.) {
            std::panic::panic_any(ProcessError::new("Invalid network, no network version declared."));
        }
        // check whether the loaded net agrees with the simulation options
        if self.myOptions.get_bool("no-internal-links").unwrap() && self.myXMLHandler.have_seen_internal_edge() && self.myXMLHandler.have_seen_default_length() {
            write_warning("Network contains internal links which are ignored. Vehicles will 'jump' across junctions and thus underestimate route lengths and travel times.");
        }
        if !self.myXMLHandler.have_seen_internal_edge() && self.myOptions.get_bool("mesosim").unwrap() {
            // @todo: setting this option has some side effect in microsim that manifest even in networks without internal lanes this should be checked
            crate::microsim::ms_globals::set_using_internal_lanes(false);
        }

        self.build_net();
        if self.myOptions.is_set("alternative-net-file", true).unwrap() {
            for fname in self.myOptions.get_string_vector("alternative-net-file").unwrap().clone() {
                let before = progress_begin_time_message(&format!("Loading alternative net from '{fname}'"));
                let mut nsh = NLNetShapeHandler::new(&fname, self.myNet.m());
                if !utils::xml::xml_sub_sys::run_parser(&mut nsh, &fname, true, false, false, true).unwrap_or(false) {
                    write_message("Loading of alternative net failed.");
                    return false;
                }
                nsh.sort_internal_shapes();
                progress_time_since(before);
            }
        }

        // @note on loading order constraints:
        // - additional-files before route-files and state-files due to referencing
        // - additional-files before weight-files since the latter might contain intermodal edge data and the intermodal net depends on the stops and public transport from the additionals

        let mut stateBeginMismatch = false;
        let stateOffset = string2time(&self.myOptions.get_string("load-state.offset").unwrap()).unwrap();
        if self.myOptions.is_set("load-state", true).unwrap() {
            // first, load only the time
            let stateTime = MSStateHandler_MSStateTimeHandler::get_time(&self.myOptions.get_string("load-state").unwrap());
            let mut defaultBegin = self.myOptions.is_default("begin");
            if stateTime != string2time(&self.myOptions.get_string("begin").unwrap()).unwrap() {
                if self.myOptions.is_default("load-state.offset") {
                    if !defaultBegin {
                        write_warningf!("State was written at a different time=% than the begin time % (set option load-state.offset to force a different begin)!",
                            time2string(stateTime), self.myOptions.get_string("begin").unwrap());
                    }
                    self.myOptions.m().reset_writable_name("begin").unwrap();
                    defaultBegin = true;
                } else {
                    stateBeginMismatch = true;
                }
            }
            if defaultBegin {
                self.myOptions.m().set("begin", &time2string(stateTime), false).unwrap();
                // if vehicles are deliberately removed from the state, we must allow them to be reloaded from a route file at state time
                let allowReloadOffset: SUMOTime = if self.myOptions.is_default("load-state.remove-vehicles") { 0 } else { 1 };
                self.myNet.m().set_loader_time(stateTime - allowReloadOffset);
                if !TraCIServer::get_instance().is_null() {
                    TraCIServer::get_instance().m().state_loaded(stateTime);
                }
            }
            self.myNet.m().set_current_time_step(string2time(&self.myOptions.get_string("begin").unwrap()).unwrap());
        }

        if self.myOptions.get_bool("junction-taz").unwrap() {
            // create a TAZ for every junction
            let junctions: Vec<(String, Ptr<MSJunction>)> = self.myNet.m().get_junction_control().iter().map(|(k, v)| (k.clone(), *v)).collect();
            for (id, junction) in junctions {
                let sinkID = format!("{id}-sink");
                let sourceID = format!("{id}-source");
                if MSEdge::dictionary_2(&sinkID).is_null() && MSEdge::dictionary_2(&sourceID).is_null() {
                    // sink must be built and added before source
                    let sink = self.myEdgeBuilder.m().build_edge(&sinkID, SumoXMLEdgeFunc::CONNECTOR, "", "", "", -1, 0.);
                    let source = self.myEdgeBuilder.m().build_edge(&sourceID, SumoXMLEdgeFunc::CONNECTOR, "", "", "", -1, 0.);
                    sink.m().set_other_taz_connector(source);
                    source.m().set_other_taz_connector(sink);
                    MSEdge::dictionary(&sinkID, sink);
                    MSEdge::dictionary(&sourceID, source);
                    sink.m().initialize(Ptr::from_raw(Box::into_raw(Box::new(Vec::new()))));
                    source.m().initialize(Ptr::from_raw(Box::into_raw(Box::new(Vec::new()))));
                    for edge in junction.get_incoming().iter().copied() {
                        if !edge.is_internal() {
                            edge.m().add_successor(sink, Ptr::default());
                        }
                    }
                    for edge in junction.get_outgoing().iter().copied() {
                        if !edge.is_internal() {
                            source.m().add_successor(edge, Ptr::default());
                        }
                    }
                } else {
                    write_warningf!("A TAZ with id '%' already exists. Not building junction TAZ.", id);
                }
            }
        }
        // load meso edge types
        let mut haveSeenMesoEdgeType = false;
        if crate::microsim::ms_globals::use_meso_sim() {
            if self.myOptions.is_set("additional-files", true).unwrap() {
                haveSeenMesoEdgeType = self.load_meso_edge_types("additional-files");
            }
            // meso segment building must be delayed until meso edge types have been read from additional files
            self.myNet.m().get_edge_control().build_meso_segments();
        }
        // load additional net elements (sources, detectors, ...)
        if self.myOptions.is_set("additional-files", true).unwrap() {
            if !self.load("additional-files", false) {
                return false;
            }
            // load shapes with separate handler
            let mut sh = NLShapeHandler::new("", self.myNet.m().get_shape_container());
            if !ShapeHandler::load_files(&self.myOptions.get_string_vector("additional-files").unwrap().clone(), &mut sh.base) {
                return false;
            }
            if self.myXMLHandler.have_seen_additional_speed_restrictions() {
                self.myNet.m().get_edge_control().set_additional_restrictions();
            }
            MSTriggeredRerouter::check_parking_reroute_consistency();
        }
        if crate::microsim::ms_globals::use_meso_sim() {
            if haveSeenMesoEdgeType || self.myXMLHandler.have_seen_tls_params() {
                for tll in self.myNet.m().get_tls_control().get_all_logics() {
                    tll.m().init_meso_tls_penalties();
                }
            }
        }
        // init after preferences have been loaded from additional-files
        MSRoutingEngine::init_weight_constants(&self.myOptions);
        // init tls after all detectors have been loaded
        self.myJunctionBuilder.m().post_load_initialization();
        // declare meandata set by options
        self.build_default_mean_data("edgedata-output", "DEFAULT_EDGEDATA", false);
        self.build_default_mean_data("lanedata-output", "DEFAULT_LANEDATA", true);

        if stateBeginMismatch && self.myNet.m().get_vehicle_control().get_loaded_vehicle_no() > 0 {
            std::panic::panic_any(ProcessError::new("Loading vehicles ahead of a state file is not supported. Correct --begin option or load vehicles with option --route-files"));
        }

        // load weights if wished
        if self.myOptions.is_set("weight-files", true).unwrap() {
            if !self.myOptions.is_usable_file_list("weight-files").unwrap() {
                return false;
            }
            // build and prepare the weights handler
            // travel time, first (always used)
            let mut ttRetriever = NLBuilder_EdgeFloatTimeLineRetriever_EdgeTravelTime::new(self.myNet);
            let mut eRetriever = NLBuilder_EdgeFloatTimeLineRetriever_EdgeEffort::new(self.myNet);
            let mut retrieverDefs = vec![utils::xml::saxweights_handler::ToRetrieveDefinition::new(
                "traveltime", true, &mut ttRetriever)];
            // the measure to use, then
            let mut measure = self.myOptions.get_string("weight-attribute").unwrap();
            if !self.myOptions.is_default("weight-attribute") {
                if measure == "CO" || measure == "CO2" || measure == "HC" || measure == "PMx"
                        || measure == "NOx" || measure == "fuel" || measure == "electricity" {
                    measure = format!("{measure}_perVeh");
                }
                retrieverDefs.push(utils::xml::saxweights_handler::ToRetrieveDefinition::new(
                    &measure, true, &mut eRetriever));
            }
            let mut handler = utils::xml::saxweights_handler::SAXWeightsHandler::new(retrieverDefs, "");
            let files = self.myOptions.get_string_vector("weight-files").unwrap().clone();
            for f in files.iter() {
                write_message(&format!("Loading weights from '{f}'..."));
                if !utils::xml::xml_sub_sys::run_parser(&mut handler, f, false, false, false, true).unwrap() {
                    return false;
                }
            }
        }

        // load the previous state if wished
        if self.myOptions.is_set("load-state", true).unwrap() {
            let f = self.myOptions.get_string("load-state").unwrap();
            let before = progress_begin_time_message(&format!("Loading state from '{f}'"));
            let mut h = MSStateHandler::new(&f, stateOffset);
            let _ = utils::xml::xml_sub_sys::run_parser(&mut h, &f, false, false, false, true);
            if with_instance(MsgType::Error, |m| m.was_informed()) {
                return false;
            }
            progress_time_since(before);
        }

        // routes from FCD files
        MSDevice_FCDReplay::init();

        // load routes
        if self.myOptions.is_set("route-files", true).unwrap() {
            if string2time(&self.myOptions.get_string("route-steps").unwrap()).unwrap() <= 0 {
                // incremental loading is disabled. Load route files fully
                if !self.load("route-files", false) {
                    return false;
                }
            } else {
                // message must come after additional-files have been loaded (but buildRouteLoaderControl was called earlier)
                for file in self.myOptions.get_string_vector("route-files").unwrap().clone() {
                    write_message(&format!("Loading route-files incrementally from '{file}'"));
                }
            }
        }

        // optionally switch off traffic lights
        if self.myOptions.get_bool("tls.all-off").unwrap() {
            self.myNet.m().get_tls_control().switch_off_all();
        }
        write_message("Loading done.");
        true
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
        // the C++ singleton OptionsCont::getOptions() is only reachable through with_options (reentrant on this thread)
        utils::options::options_cont::with_options(|oc| {
            oc.clear();
            MSFrame::fill_options();
            utils::options::options_io::get_options(false).unwrap();
            if oc.process_meta_options(utils::options::options_io::get_arg_c() < 2).unwrap() {
                utils::common::system_frame::close();
                return Ptr::default();
            }
            utils::common::system_frame::check_options(oc);
            let mut validation = oc.get_string("xml-validation").unwrap();
            let mut routeValidation = oc.get_string("xml-validation.routes").unwrap();
            if isLibsumo {
                if oc.is_default("xml-validation") {
                    validation = "never".to_string();
                }
                if oc.is_default("xml-validation.routes") {
                    routeValidation = "never".to_string();
                }
            }
            utils::xml::xml_sub_sys::set_validation(&validation, &oc.get_string("xml-validation.net").unwrap(), &routeValidation).unwrap();
            if !MSFrame::check_options() {
                std::panic::panic_any(ProcessError::default_msg());
            }
            // C++ HAVE_FOX branch (MsgHandlerSynchronized for threads > 1) is not part of this port
            MsgHandler::init_output_options().unwrap();
            Self::init_randomness();
            MSFrame::set_ms_globals(oc);
            let vc: Ptr<MSVehicleControl> = if crate::microsim::ms_globals::use_meso_sim() {
                Ptr::from_raw(Box::into_raw(Box::new(MEVehicleControl::new()))).cast::<MSVehicleControl>()
            } else {
                Ptr::from_raw(Box::into_raw(Box::new(MSVehicleControl::new())))
            };
            let net: Ptr<MSNet> = Ptr::from_raw(Box::into_raw(MSNet::new(
                vc,
                Ptr::from_raw(Box::into_raw(Box::new(MSEventControl::new()))),
                Ptr::from_raw(Box::into_raw(Box::new(MSEventControl::new()))),
                Ptr::from_raw(Box::into_raw(Box::new(MSEventControl::new()))),
                Ptr::default(),
            )));
            // need to init TraCI-Server before loading routes to catch VehicleState::BUILT
            TraCIServer::open_socket(&std::collections::BTreeMap::new());
            if isLibsumo {
                crate::libsumo::helper::Helper::register_state_listener();
            }

            let mut eb = NLEdgeControlBuilder::new();
            let mut db = NLDetectorBuilder::new(net.m());
            let mut jb = NLJunctionControlBuilder::new(net.m(), &mut db);
            let mut tb = NLTriggerBuilder::new();
            let mut handler = NLHandler::new("", net.m(), &mut db, &mut tb, &mut eb, &mut jb);
            tb.set_handler(Ptr::from_mut(&mut handler));
            let mut builder = NLBuilder::new(oc, net.m(), &mut eb, &mut jb, &mut db, &mut handler);
            with_instance(MsgType::Error, |m| m.clear(true));
            with_instance(MsgType::Warning, |m| m.clear(true));
            with_instance(MsgType::Message, |m| m.clear(true));
            if builder.build() {
                // preload the routes especially for TraCI
                net.m().load_routes();
                return net;
            }
            drop(unsafe { Box::from_raw(net.as_raw()) });
            std::panic::panic_any(ProcessError::default_msg());
        })
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
        utils::common::rand_helper::init_rand(None, utils::options::options_cont::get_bool("random"), utils::options::options_cont::get_int("seed"));
        utils::common::rand_helper::init_rand(Some(unsafe { &mut *SUMORouteHandler::get_parsing_rng().as_raw() }), utils::options::options_cont::get_bool("random"), utils::options::options_cont::get_int("seed"));
        utils::common::rand_helper::init_rand(Some(unsafe { &mut *MSDevice::get_equipment_rng().as_raw() }), utils::options::options_cont::get_bool("random"), utils::options::options_cont::get_int("seed"));
        utils::common::rand_helper::init_rand(Some(unsafe { &mut *OUProcess::get_rng().as_raw() }), utils::options::options_cont::get_bool("random"), utils::options::options_cont::get_int("seed"));
        utils::common::rand_helper::init_rand(Some(unsafe { &mut *MSDevice_ToC::get_response_time_rng().as_raw() }), utils::options::options_cont::get_bool("random"), utils::options::options_cont::get_int("seed"));
        utils::common::rand_helper::init_rand(Some(unsafe { &mut *MSDevice_BTreceiver::get_recognition_rng().as_raw() }), utils::options::options_cont::get_bool("random"), utils::options::options_cont::get_int("seed"));
        utils::options::options_cont::with_options(|oc| MSLane::init_rn_gs(oc));
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
        // build the loaders
        let loaders: Ptr<SUMORouteLoaderControl> = Ptr::from_raw(Box::into_raw(Box::new(SUMORouteLoaderControl::new(string2time(&oc.get_string("route-steps").unwrap()).unwrap()))));
        // check whether a list is existing
        if oc.is_set("route-files", true).unwrap() && string2time(&oc.get_string("route-steps").unwrap()).unwrap() > 0 {
            let files = oc.get_string_vector("route-files").unwrap().clone();
            for file in files.iter() {
                if !utils::common::file_helpers::is_readable(file) {
                    std::panic::panic_any(ProcessError::new(format!("The route file '{file}' is not accessible.")));
                }
            }
            // open files for reading
            for file in files.iter() {
                let handler: Ptr<MSRouteHandler> = Ptr::from_raw(Box::into_raw(Box::new(MSRouteHandler::new(file, false))));
                loaders.m().add(Ptr::from_raw(Box::into_raw(Box::new(SUMORouteLoader::new(handler.cast::<SUMORouteHandler>())))));
            }
        }
        loaders
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
        if !self.myOptions.is_usable_file_list(mmlWhat).unwrap() {
            return false;
        }
        let files = self.myOptions.get_string_vector(mmlWhat).unwrap().clone();
        for fileIt in files.iter() {
            let before = progress_begin_time_message(&format!("Loading {mmlWhat} from '{fileIt}'"));
            if !utils::xml::xml_sub_sys::run_parser(self.myXMLHandler.m(), fileIt, isNet, false, false, true).unwrap_or(false) {
                write_message(&format!("Loading of {mmlWhat} failed."));
                return false;
            }
            progress_time_since(before);
        }
        true
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
        if !self.myOptions.is_usable_file_list(mmlWhat).unwrap() {
            return false;
        }
        let mut meTypeHandler = METypeHandler::new("", self.myNet.m());
        for file in self.myOptions.get_string_vector(mmlWhat).unwrap().clone() {
            if !utils::xml::xml_sub_sys::run_parser(&mut meTypeHandler, &file, false, false, false, true).unwrap_or(false) {
                continue;
            }
        }
        meTypeHandler.have_seen_meso_edge_type()
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
        // shortcut: the C++ try/catch (cleanup of edges/junctions/loaders/tlc, then rethrow) is not replicated; a ProcessError here unwinds the run instead
        // ensure streams are ready for output during building
        MSFrame::build_streams();
        let networkVersion = self.myXMLHandler.network_version();
        let edges = self.myEdgeBuilder.m().build(&networkVersion);
        let junctions = self.myJunctionBuilder.m().build();
        junctions.m().postload_init_container();
        for e in edges.m().get_edges().iter().copied() {
            e.m().post_load_init_lane_changer();
        }
        let routeLoaders = Self::build_route_loader_control(&self.myOptions);
        let tlc = self.myJunctionBuilder.m().build_tl_logics();
        let mut stateDumpTimes: Vec<SUMOTime> = Vec::new();
        let mut stateDumpFiles: Vec<String> = Vec::new();
        for timeStr in self.myOptions.get_string_vector("save-state.times").unwrap().clone() {
            stateDumpTimes.push(string2time(&timeStr).unwrap());
        }
        if self.myOptions.is_set("save-state.files", true).unwrap() {
            stateDumpFiles = self.myOptions.get_string_vector("save-state.files").unwrap().clone();
            if stateDumpFiles.len() != stateDumpTimes.len() {
                std::panic::panic_any(ProcessError::new("Wrong number of state file names!"));
            }
        } else {
            let prefix = self.myOptions.get_string("save-state.prefix").unwrap();
            let suffix = self.myOptions.get_string("save-state.suffix").unwrap();
            for i in stateDumpTimes.iter() {
                let timeStamp = time2string(*i).replace(':', "-");
                stateDumpFiles.push(format!("{prefix}_{timeStamp}{suffix}"));
            }
        }

        // if anthing goes wrong after this point, the net is responsible for cleaning up
        self.myNet.m().close_building(
            &self.myOptions,
            edges,
            junctions,
            routeLoaders,
            tlc,
            stateDumpTimes,
            stateDumpFiles,
            self.myXMLHandler.have_seen_internal_edge(),
            self.myXMLHandler.has_junction_higher_speeds(),
            &self.myXMLHandler.network_version(),
        );
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
        if self.myOptions.is_set(optionName, true).unwrap() {
            let mut useLanes = useLanes;
            if useLanes && crate::microsim::ms_globals::use_meso_sim() && !self.myOptions.get_bool("meso-lane-queue").unwrap() {
                write_warning("LaneData requested for mesoscopic simulation but --meso-lane-queue is not active. Falling back to edgeData.");
                useLanes = false;
            }
            let begin = string2time(&self.myOptions.get_string("begin").unwrap()).unwrap();
            let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                self.myDetectorBuilder.m().create_edge_lane_mean_data(id, -1, begin, -1, "traffic", useLanes, "true",
                    false, false, 0, 100000., 0., utils::common::std_defs::SUMO_CONST_HALTING_SPEED, 0., "", "", Vec::new(), AggregateType::NO,
                    &self.myOptions.get_string(optionName).unwrap());
            }));
            // C++ catches InvalidArgument and IOError and reports them as errors
            if let Err(payload) = res {
                match payload.downcast::<ProcessError>() {
                    Ok(e) => write_error(&e.to_string()),
                    Err(other) => std::panic::resume_unwind(other),
                }
            }
        }
    }

    /// C++: `void NLBuilder::NLBuilder(const NLBuilder &)` [private]
    /// invalidated copy operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(s: &NLBuilder) -> Self {
        panic!("NLBuilder copy constructor is private and undefined in C++, it must not be used")
    }

    /// C++: `NLBuilder & NLBuilder::operator=(const NLBuilder &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &NLBuilder) -> &mut NLBuilder {
        panic!("NLBuilder assignment is private and undefined in C++, it must not be used")
    }

}

