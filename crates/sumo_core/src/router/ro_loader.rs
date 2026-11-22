//! Port of router/ROLoader.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::msg_handler::{progress_begin_message, progress_done_message, write_message, write_warning};

/// C++ `MsgHandler::getErrorInstance()->wasInformed()`
fn error_informed() -> bool {
    utils::common::msg_handler::with_instance(utils::common::msg_handler::MsgType::Error, |h| h.was_informed())
}

/// C++ `MIN2(a, b)` for SUMOTime
fn min2t(a: SUMOTime, b: SUMOTime) -> SUMOTime {
    if a < b { a } else { b }
}

/// C++ `XMLSubSys::runParser(handler, file, isNet)`: true if no error was reported
fn run_net_parser(handler: &mut RONetHandler, file: &str, isNet: bool) -> bool {
    utils::xml::xml_sub_sys::run_parser(handler, file, isNet, false, false, true).unwrap_or_else(|e| panic!("{e}"))
}

/// C++ `XMLSubSys::runParser(*handler, fileIt)`; false when the loading reported an error
fn run_route_parser(handler: &mut RORouteHandler, file: &str) -> bool {
    utils::xml::xml_sub_sys::run_parser(handler, file, false, false, false, true).unwrap_or_else(|e| std::panic::panic_any(e))
}

/// Obtains edge travel times from a weights handler and stores them within the edges
/// C++ class `ROLoader::EdgeFloatTimeLineRetriever_EdgeTravelTime` (router/ROLoader.h)
#[repr(C)]
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
        Self {
            base: (),
            myNet: Ptr::from_mut(net),
        }
    }

    /// C++: `void EdgeFloatTimeLineRetriever_EdgeTravelTime::~EdgeFloatTimeLineRetriever_EdgeTravelTime() noexcept` [virtual]
    /// Destructor
    // ~EdgeFloatTimeLineRetriever_EdgeTravelTime() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor has no body
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
        let e = self.myNet.get_edge(id);
        if !e.is_null() {
            e.m().add_travel_time(val, beg, end);
        } else if !id.starts_with(':') {
            if utils::options::options_cont::get_bool("ignore-errors") {
                write_warningf!("Trying to set a weight for the unknown edge '%'.", id);
            } else {
                write_errorf!("Trying to set a weight for the unknown edge '%'.", id);
            }
        }
    }

}

impl utils::xml::saxweights_handler::EdgeFloatTimeLineRetriever for ROLoader_EdgeFloatTimeLineRetriever_EdgeTravelTime {
    fn add_edge_weight(&self, id: &str, val: f64, beg: f64, end: f64) {
        ROLoader_EdgeFloatTimeLineRetriever_EdgeTravelTime::add_edge_weight(self, id, val, beg, end)
    }
}

/// Obtains edge weights from a weights handler and stores them within the edges
/// C++ class `ROLoader::EdgeFloatTimeLineRetriever_EdgeWeight` (router/ROLoader.h)
#[repr(C)]
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
        Self {
            base: (),
            myNet: Ptr::from_mut(net),
        }
    }

    /// C++: `void EdgeFloatTimeLineRetriever_EdgeWeight::~EdgeFloatTimeLineRetriever_EdgeWeight() noexcept` [virtual]
    /// Destructor
    // ~EdgeFloatTimeLineRetriever_EdgeWeight() { }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor has no body
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
        let e = self.myNet.get_edge(id);
        if !e.is_null() {
            e.m().add_effort(val, beg, end);
        } else if !id.starts_with(':') {
            if utils::options::options_cont::get_bool("ignore-errors") {
                write_warningf!("Trying to set a weight for the unknown edge '%'.", id);
            } else {
                write_errorf!("Trying to set a weight for the unknown edge '%'.", id);
            }
        }
    }

}

impl utils::xml::saxweights_handler::EdgeFloatTimeLineRetriever for ROLoader_EdgeFloatTimeLineRetriever_EdgeWeight {
    fn add_edge_weight(&self, id: &str, val: f64, beg: f64, end: f64) {
        ROLoader_EdgeFloatTimeLineRetriever_EdgeWeight::add_edge_weight(self, id, val, beg, end)
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
        ROLoader {
            myOptions: Ptr::from_mut(oc),
            myEmptyDestinationsAllowed: emptyDestinationsAllowed,
            myLogSteps: logSteps,
            myLoaders: SUMORouteLoaderControl::new(if oc.exists("unsorted-input") && oc.get_bool("unsorted-input").unwrap_or_else(|e| panic!("{e}")) { 0 } else { utils::common::time::delta_t() }),
        }
    }

    /// C++: `void ROLoader::~ROLoader()` [virtual]
    /// Destructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor has no body (the route loaders are owned by myLoaders and freed by its owner)
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
        let file = self.myOptions.get_string("net-file").unwrap_or_else(|e| panic!("{e}"));
        if file == "" {
            panic!("Missing definition of network to load!");
        }
        if !utils::common::file_helpers::is_readable(&file) {
            panic!("The network file '{}' is not accessible.", file);
        }
        let opts = self.myOptions;
        let ignoreInternal = !opts.exists("no-internal-links") || opts.get_bool("no-internal-links").unwrap_or_else(|e| panic!("{e}"));
        let minorPenalty = if opts.exists("weights.minor-penalty") { opts.get_float("weights.minor-penalty").unwrap_or_else(|e| panic!("{e}")) } else { 0. };
        let tlsPenalty = if opts.exists("weights.tls-penalty") { opts.get_float("weights.tls-penalty").unwrap_or_else(|e| panic!("{e}")) } else { 0. };
        let turnaroundPenalty = if opts.exists("weights.turnaround-penalty") { opts.get_float("weights.turnaround-penalty").unwrap_or_else(|e| panic!("{e}")) } else { 0. };
        let mut handler = RONetHandler::new(toFill, eb, ignoreInternal, minorPenalty, tlsPenalty, turnaroundPenalty);
        utils::common::msg_handler::progress_begin_message("Loading net");
        if !run_net_parser(&mut handler, &file, true) {
            utils::common::msg_handler::progress_failed_message();
            panic!("{}", ProcessError::default_msg());
        } else {
            utils::common::msg_handler::progress_done_message();
        }
        if !utils::common::sumo_vehicle_class::DEPRECATED_VEHICLE_CLASSES_SEEN.lock().unwrap_or_else(|e| e.into_inner()).is_empty() {
            let mut seen = utils::common::sumo_vehicle_class::DEPRECATED_VEHICLE_CLASSES_SEEN.lock().unwrap_or_else(|e| e.into_inner());
            let names = utils::common::to_string::to_string_prec(&*seen, utils::common::std_defs::g_precision());
            write_warningf!("Deprecated vehicle classes '%' in input network.", names);
            seen.clear();
        }
        if opts.is_set("additional-files", false).unwrap_or(false) {
            let files = utils::options::options_cont::get_string_vector("additional-files");
            for fileIt in files.iter() {
                if !utils::common::file_helpers::is_readable(fileIt) {
                    panic!("The additional file '{}' is not accessible.", fileIt);
                }
                utils::common::msg_handler::progress_begin_message(&format!("Loading additional file '{}' ", fileIt));
                if !run_net_parser(&mut handler, fileIt, false) {
                    utils::common::msg_handler::progress_failed_message();
                    panic!("{}", ProcessError::default_msg());
                } else {
                    utils::common::msg_handler::progress_done_message();
                }
            }
        }
        if opts.exists("junction-taz") && utils::options::options_cont::get_bool("junction-taz") {
            // create a TAZ for every junction
            toFill.add_junction_taz(eb);
        }
        toFill.set_bidi_edges(handler.get_bidi_map());
        if opts.exists("restriction-params") && opts.is_set("restriction-params", false).unwrap_or(false) {
            toFill.set_param_restrictions();
            let paramKeys = utils::options::options_cont::get_string_vector("restriction-params");
            for (_, edge) in toFill.get_edge_map().iter() {
                edge.m().cache_param_restrictions(&paramKeys);
            }
        }
        if toFill.has_speed_restrictions() {
            for (_, edge) in toFill.get_edge_map().iter() {
                let restrictions = toFill.get_restrictions(edge.get_type());
                edge.m().set_speed_restrictions(restrictions);
            }
        }
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
        // check whether the file exists
        if !self.myOptions.is_usable_file_list(optionName).unwrap_or_else(|e| panic!("{e}")) {
            return false;
        }
        // build and prepare the weights handler
        let mut ttRetriever = ROLoader_EdgeFloatTimeLineRetriever_EdgeTravelTime::new(net);
        let mut eRetriever = ROLoader_EdgeFloatTimeLineRetriever_EdgeWeight::new(net);
        //  travel time, first (always used)
        let mut retrieverDefs = vec![utils::xml::saxweights_handler::ToRetrieveDefinition::new("traveltime", !useLanes, &mut ttRetriever)];
        //  the measure to use, then
        if measure != "traveltime" {
            let mut umeasure = measure.to_string();
            if measure == "CO" || measure == "CO2" || measure == "HC" || measure == "PMx" || measure == "NOx" || measure == "fuel" || measure == "electricity" {
                umeasure = format!("{}_perVeh", measure);
            }
            retrieverDefs.push(utils::xml::saxweights_handler::ToRetrieveDefinition::new(&umeasure, !useLanes, &mut eRetriever));
        }
        //  set up handler
        let mut handler = utils::xml::saxweights_handler::SAXWeightsHandler::new(retrieverDefs, "");
        // go through files
        let files: Vec<String> = self.myOptions.get_string_vector(optionName).unwrap_or_else(|e| panic!("{e}")).clone();
        for fileIt in files.iter() {
            progress_begin_message(&format!("Loading precomputed net weights from '{}'", fileIt));
            if utils::xml::xml_sub_sys::run_parser(&mut handler, fileIt, false, false, false, true).unwrap_or_else(|e| panic!("{e}")) {
                progress_done_message();
            } else {
                write_message("failed.");
                return false;
            }
        }
        // build edge-internal time lines
        for (_, edge) in net.get_edge_map().iter() {
            edge.m().build_time_lines(measure, boundariesOverride);
        }
        true
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
        // build loader
        // load relevant elements from additional file
        let mut ok = self.open_typed_routes("additional-files", net, true);
        // load sumo routes, trips, and flows
        ok &= self.open_typed_routes("route-files", net, false);
        // check
        if ok {
            let begin = utils::common::time::string2time(&self.myOptions.get_string("begin").unwrap_or_else(|e| panic!("{e}"))).unwrap_or_else(|e| panic!("{e}"));
            self.myLoaders.load_next(begin);
            if !net.further_stored() {
                if error_informed() {
                    panic!("{}", ProcessError::default_msg());
                } else {
                    let error = "No route input specified or all routes were invalid.";
                    if utils::options::options_cont::get_bool("ignore-errors") {
                        write_warning(error);
                    } else {
                        std::panic::panic_any(ProcessError::new(error.to_string()));
                    }
                }
            }
            // skip routes prior to the begin time
            if !utils::options::options_cont::get_bool("unsorted-input") {
                write_message(&format!("Skipped until: {}", utils::common::time::time2string(self.myLoaders.get_first_load_time())));
            }
        }
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
        let absNo = end - start;
        let endGiven = !self.myOptions.is_default("end");
        // skip routes that begin before the simulation's begin
        // loop till the end
        let firstStep = self.myLoaders.get_first_load_time();
        let mut lastStep = firstStep;
        let mut time = min2t(firstStep, end);
        while time <= end {
            self.write_stats(time, start, absNo, endGiven);
            self.myLoaders.load_next(time);
            if !net.further_stored() || error_informed() {
                break;
            }
            lastStep = net.save_and_remove_routes_until(self.myOptions.m(), provider, time);
            if time == end || (!net.further_stored() && self.myLoaders.have_all_loaded()) || error_informed() {
                break;
            }
            if time < end && time > end - increment {
                time = end;
            } else {
                time += increment;
            }
        }
        if self.myLogSteps {
            write_messagef!("Routes found between time steps % and %.", utils::common::time::time2string(firstStep), utils::common::time::time2string(lastStep));
        }
        // v1_28_0 ROLoader.cpp:224 ends after the log message; the "No routes found
        // between begin time ..." warning was added later.
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
        let loader = self.myLoaders.get_first_loader();
        if !loader.is_null() {
            loader.m().get_route_handler()
        } else {
            Ptr::null()
        }
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
        // check whether the current loader is wished
        //  and the file(s) can be used
        if !self.myOptions.is_usable_file_list(optionName).unwrap_or_else(|e| panic!("{e}")) {
            return !self.myOptions.is_set(optionName, true).unwrap_or_else(|e| panic!("{e}"));
        }
        let files = utils::options::options_cont::get_string_vector(optionName);
        for fileIt in files.iter() {
            let emptyDestinationsAllowed = self.myEmptyDestinationsAllowed;
            let repair = utils::options::options_cont::get_bool("repair");
            let ignoreErrors = utils::options::options_cont::get_bool("ignore-errors");
            // the C++ try/catch around the handler construction: a ProcessError is reported and the loader fails
            let built = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                RORouteHandler::new(net, fileIt, repair, emptyDestinationsAllowed, ignoreErrors, !readAll)
            }));
            let handler = match built {
                Ok(h) => Ptr::from_raw(Box::into_raw(Box::new(h))),
                Err(payload) => {
                    let what = payload
                        .downcast_ref::<String>()
                        .cloned()
                        .or_else(|| payload.downcast_ref::<&str>().map(|s| s.to_string()))
                        .unwrap_or_default();
                    write_errorf!("The loader for % from file '%' could not be initialised (%).", optionName, fileIt, what);
                    return false;
                }
            };
            if readAll {
                if !run_route_parser(handler.m(), fileIt) {
                    write_errorf!("Loading of % failed.", fileIt);
                    return false;
                }
            } else {
                self.myLoaders.add(Ptr::from_raw(Box::into_raw(Box::new(SUMORouteLoader::new(Ptr::new(&handler.base))))));
            }
        }
        true
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
        if self.myLogSteps {
            if endGiven {
                let perc = (time - start) as f64 / absNo as f64;
                print!(
                    "Reading up to time step: {}  ({}/{} = {}% done)       \r",
                    utils::common::time::time2string(time),
                    utils::common::time::time2string(time - start),
                    utils::common::time::time2string(absNo),
                    utils::common::to_string::f64_to_string_prec(perc * 100., utils::common::std_defs::g_precision())
                );
            } else {
                print!("Reading up to time step: {}\r", utils::common::time::time2string(time));
            }
        }
    }

    /// C++: `void ROLoader::ROLoader(const ROLoader &)` [private]
    /// Invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(src: &ROLoader) -> Self {
        // C++ declares this copy constructor private and never defines it, so it can not be called
        panic!("ROLoader copy constructor is private and not defined in C++");
    }

    /// C++: `ROLoader & ROLoader::operator=(const ROLoader &)` [private]
    /// Invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, src: &ROLoader) -> &mut ROLoader {
        // C++ declares this assignment operator private and never defines it, so it can not be called
        panic!("ROLoader assignment operator is private and not defined in C++");
    }

}

