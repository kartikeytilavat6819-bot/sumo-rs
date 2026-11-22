//! Port of netimport/NILoader.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::msg_handler::{write_error, write_message, write_warning};

/// C++ `throw`: raised as a panic carrying the ProcessError until a top-level catch exists
fn throw<T>(r: Result<T, ProcessError>) -> T {
    r.unwrap_or_else(|e| std::panic::panic_any(e))
}

// shortcut: C++ global deprecatedVehicleClassesSeen (utils/common/SUMOVehicleClass.cpp) is not ported; local set until it is
thread_local! {
    static DEPRECATED_VEHICLE_CLASSES_SEEN: std::cell::RefCell<BTreeSet<String>> = std::cell::RefCell::new(BTreeSet::new());
}

/// Perfoms network import
/// C++ class `NILoader` (netimport/NILoader.h)
pub struct NILoader {
    /// The network builder to fill with loaded data
    pub myNetBuilder: Ptr<NBNetBuilder>,
}

impl NILoader {
    /// C++: `void NILoader::NILoader(NBNetBuilder &)`
    /// Constructor
    // NILoader::NILoader(NBNetBuilder& nb)
    //     : myNetBuilder(nb) {}
    #[allow(unused_variables)]
    pub fn new(nb: &mut NBNetBuilder) -> Self {
        NILoader { myNetBuilder: Ptr::from_mut(nb) }
    }

    /// C++: `void NILoader::~NILoader()`
    /// Destructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // the C++ destructor has no body
    }

    /// C++: `void NILoader::load(OptionsCont &)`
    /// loads data from the files specified in the given option container
    // void
    // NILoader::load(OptionsCont& oc) {
    //     bool ok = true;
    //     // load types first
    //     NIXMLTypesHandler handler(myNetBuilder.getTypeCont());
    //     if (!oc.isSet("type-files")) {
    //         std::vector<std::string> files;
    //         if (oc.isSet("osm-files")) {
    //             files.push_back(osmTypemap);
    //         }
    //         if (oc.isSet("opendrive-files")) {
    //             files.push_back(opendriveTypemap);
    //         }
    //         if (oc.isSet("visum-file")) {
    //             files.push_back(visumTypemap);
    //         }
    //         ok &= NITypeLoader::load(handler, files, toString(SUMO_TAG_TYPES), true);
    //     } else {
    //         ok &= NITypeLoader::load(handler, oc.getStringVector("type-files"), toString(SUMO_TAG_TYPES));
    //     }
    //     // try to load height data so it is ready for use by other importers
    //     NBHeightMapper::loadIfSet(oc);
    //     // try to load using different methods
    //     NIImporter_SUMO::loadNetwork(oc, myNetBuilder);
    //     NIImporter_OpenStreetMap::loadNetwork(oc, myNetBuilder);
    //     NIImporter_VISUM::loadNetwork(oc, myNetBuilder);
    //     NIImporter_ArcView::loadNetwork(oc, myNetBuilder);
    //     NIImporter_Vissim::loadNetwork(oc, myNetBuilder);
    //     NIImporter_DlrNavteq::loadNetwork(oc, myNetBuilder);
    //     NIImporter_OpenDrive::loadNetwork(oc, myNetBuilder);
    //     NIImporter_MATSim::loadNetwork(oc, myNetBuilder);
    //     NIImporter_ITSUMO::loadNetwork(oc, myNetBuilder);
    //     if (oc.getBool("tls.discard-loaded") || oc.getBool("tls.discard-simple")) {
    //         myNetBuilder.getNodeCont().discardTrafficLights(myNetBuilder.getTLLogicCont(), oc.getBool("tls.discard-simple"));
    //         int removed = myNetBuilder.getTLLogicCont().getNumExtracted();
    //         if (removed > 0) {
    //             WRITE_MESSAGEF(TL(" Removed % traffic lights before loading plain-XML"), toString(removed));
    //         }
    //     }
    //     if (oc.getBool("railway.signals.discard")) {
    //         myNetBuilder.getNodeCont().discardRailSignals();
    //     }
    //     ok &= loadXML(oc);
    //     // check the loaded structures
    //     if (myNetBuilder.getNodeCont().size() == 0) {
    //         throw ProcessError(TL("No nodes loaded."));
    //     }
    //     if (myNetBuilder.getEdgeCont().size() == 0) {
    //         throw ProcessError(TL("No edges loaded."));
    //     }
    //     if (!myNetBuilder.getEdgeCont().checkConsistency(myNetBuilder.getNodeCont())) {
    //         throw ProcessError();
    //     }
    //     if (!ok && !oc.getBool("ignore-errors")) {
    //         throw ProcessError();
    //     }
    //     // configure default values that depend on other values
    //     myNetBuilder.getNodeCont().applyConditionalDefaults();
    //     // report loaded structures
    //     WRITE_MESSAGE(TL(" Import done:"));
    //     if (myNetBuilder.getDistrictCont().size() > 0) {
    //         WRITE_MESSAGEF(TL("   % districts loaded."), toString(myNetBuilder.getDistrictCont().size()));
    //     }
    //     WRITE_MESSAGEF(TL("   % nodes loaded."), toString(myNetBuilder.getNodeCont().size()));
    //     if (myNetBuilder.getTypeCont().size() > 0) {
    //         WRITE_MESSAGEF(TL("   % types loaded."), toString(myNetBuilder.getTypeCont().size()));
    //     }
    //     WRITE_MESSAGEF(TL("   % edges loaded."), toString(myNetBuilder.getEdgeCont().size()));
    //     if (myNetBuilder.getEdgeCont().getNumEdgeSplits() > 0) {
    //         WRITE_MESSAGEF(TL("The split of edges was performed % times."), toString(myNetBuilder.getEdgeCont().getNumEdgeSplits()));
    //     }
    //
    //     //TODO: uncomment the following lines + adapt tests! [Gregor March '17]
    // //  if (myNetBuilder.getPTStopCont().size() > 0) {
    // //    WRITE_MESSAGEF(TL("   % pt stops loaded."), toString(myNetBuilder.getPTStopCont().size()));
    // //  }
    //     if (GeoConvHelper::getProcessing().usingGeoProjection()) {
    //         WRITE_MESSAGEF(TL("Proj projection parameters used: '%'."), GeoConvHelper::getProcessing().getProjString());
    //     }
    // }
    #[allow(unused_variables)]
    pub fn load(&mut self, oc: &mut OptionsCont) {
        let mut ok = true;
        // load types first
        let mut handler = NIXMLTypesHandler::new(self.myNetBuilder.m().get_type_cont());
        if !throw(oc.is_set("type-files", true)) {
            // built-in typemaps: C++ compiles data/typemap/*.typ.xml into typemap.h; the same files are embedded here
            let mut files: Vec<String> = Vec::new();
            if throw(oc.is_set("osm-files", true)) {
                files.push(include_str!("../../data/typemap/osmNetconvert.typ.xml").to_string());
            }
            if throw(oc.is_set("opendrive-files", true)) {
                files.push(include_str!("../../data/typemap/opendriveNetconvert.typ.xml").to_string());
            }
            if throw(oc.is_set("visum-file", true)) {
                files.push(include_str!("../../data/typemap/visumNetconvert.typ.xml").to_string());
            }
            ok &= NITypeLoader::load(&mut handler, &files, "types", true);
        } else {
            ok &= NITypeLoader::load(&mut handler, throw(oc.get_string_vector("type-files")).as_slice(), "types", false);
        }
        // try to load height data so it is ready for use by other importers
        NBHeightMapper::load_if_set(oc);
        // try to load using different methods
        NIImporter_SUMO::load_network(oc, self.myNetBuilder.m());
        NIImporter_OpenStreetMap::load_network(oc, self.myNetBuilder.m());
        NIImporter_VISUM::load_network(oc, self.myNetBuilder.m());
        NIImporter_ArcView::load_network(oc, self.myNetBuilder.m());
        NIImporter_Vissim::load_network(oc, self.myNetBuilder.m());
        NIImporter_DlrNavteq::load_network(oc, self.myNetBuilder.m());
        NIImporter_OpenDrive::load_network(oc, self.myNetBuilder.m());
        NIImporter_MATSim::load_network(oc, self.myNetBuilder.m());
        NIImporter_ITSUMO::load_network(oc, self.myNetBuilder.m());
        if throw(oc.get_bool("tls.discard-loaded")) || throw(oc.get_bool("tls.discard-simple")) {
            let simple = throw(oc.get_bool("tls.discard-simple"));
            self.myNetBuilder.m().get_node_cont().discard_traffic_lights(self.myNetBuilder.m().get_tl_logic_cont(), simple);
            let removed = self.myNetBuilder.m().get_tl_logic_cont().get_num_extracted();
            if removed > 0 {
                write_message(&format!(" Removed {removed} traffic lights before loading plain-XML"));
            }
        }
        if throw(oc.get_bool("railway.signals.discard")) {
            self.myNetBuilder.m().get_node_cont().discard_rail_signals();
        }
        ok &= self.load_xml(oc);
        // check the loaded structures
        // C++ throws ProcessError here; the error must go through the throw path, not write_error,
        // so it is not recorded as an initial message that clear(false) would replay.
        if self.myNetBuilder.m().get_node_cont().size() == 0 {
            std::panic::panic_any(ProcessError::new("No nodes loaded."));
        }
        if self.myNetBuilder.m().get_edge_cont().size() == 0 {
            std::panic::panic_any(ProcessError::new("No edges loaded."));
        }
        if !self.myNetBuilder.m().get_edge_cont().check_consistency(self.myNetBuilder.m().get_node_cont()) {
            // C++ throws ProcessError() here; netconvert's catch deliberately suppresses the
            // "Process Error" message and only prints "Quitting (on error)." with exit code 1.
            // Route it through the port's throw mechanism so main's filter handles it.
            std::panic::panic_any(ProcessError::default_msg());
        }
        if !ok && !throw(oc.get_bool("ignore-errors")) {
            std::panic::panic_any(ProcessError::default_msg());
        }
        // configure default values that depend on other values
        self.myNetBuilder.m().get_node_cont().apply_conditional_defaults();
        // report loaded structures
        write_message(" Import done:");
        if self.myNetBuilder.m().get_district_cont().size() > 0 {
            write_message(&format!("   {} districts loaded.", self.myNetBuilder.m().get_district_cont().size()));
        }
        write_message(&format!("   {} nodes loaded.", self.myNetBuilder.m().get_node_cont().size()));
        if self.myNetBuilder.m().get_type_cont().size() > 0 {
            write_message(&format!("   {} types loaded.", self.myNetBuilder.m().get_type_cont().size()));
        }
        write_message(&format!("   {} edges loaded.", self.myNetBuilder.m().get_edge_cont().size()));
        if self.myNetBuilder.m().get_edge_cont().get_num_edge_splits() > 0 {
            write_message(&format!("The split of edges was performed {} times.", self.myNetBuilder.m().get_edge_cont().get_num_edge_splits()));
        }

        //TODO: uncomment the following lines + adapt tests! [Gregor March '17]
        //  if (myNetBuilder.getPTStopCont().size() > 0) {
        //    WRITE_MESSAGEF(TL("   % pt stops loaded."), toString(myNetBuilder.getPTStopCont().size()));
        //  }
        if GeoConvHelper::with_processing(|g| g.using_geo_projection()) {
            let proj = GeoConvHelper::with_processing(|g| g.get_proj_string().to_string());
            write_message(&format!("Proj projection parameters used: '{proj}'."));
        }
    }

    /// C++: `bool NILoader::loadXML(OptionsCont &)` [private]
    /// loads data from XML-files
    // bool
    // NILoader::loadXML(OptionsCont& oc) {
    //     // load nodes
    //     NIXMLNodesHandler nodesHandler(myNetBuilder.getNodeCont(), myNetBuilder.getEdgeCont(),
    //                                    myNetBuilder.getTLLogicCont(), oc);
    //     bool ok = NITypeLoader::load(nodesHandler, oc.getStringVector("node-files"), "nodes");
    //     // load the edges
    //     if (ok) {
    //         NIXMLEdgesHandler edgesHandler(myNetBuilder.getNodeCont(), myNetBuilder.getEdgeCont(),
    //                                        myNetBuilder.getTypeCont(), myNetBuilder.getDistrictCont(),
    //                                        myNetBuilder.getTLLogicCont(), oc);
    //         ok = NITypeLoader::load(edgesHandler, oc.getStringVector("edge-files"), "edges");
    //     }
    //     if (!deprecatedVehicleClassesSeen.empty()) {
    //         WRITE_WARNINGF(TL("Deprecated vehicle class(es) '%' in input edge files."), toString(deprecatedVehicleClassesSeen));
    //     }
    //     // load the connections
    //     if (ok) {
    //         NIXMLConnectionsHandler connectionsHandler(myNetBuilder.getEdgeCont(),
    //                 myNetBuilder.getNodeCont(), myNetBuilder.getTLLogicCont());
    //         ok = NITypeLoader::load(connectionsHandler, oc.getStringVector("connection-files"), "connections");
    //     }
    //     // load traffic lights (needs to come last, references loaded edges and connections)
    //     if (ok) {
    //         NIXMLTrafficLightsHandler tlHandler(myNetBuilder.getTLLogicCont(), myNetBuilder.getEdgeCont());
    //         ok = NITypeLoader::load(tlHandler, oc.getStringVector("tllogic-files"), "traffic lights");
    //     }
    //
    //     // load public transport stops (used for restricting edge removal and as input when repairing railroad topology)
    //     if (ok && oc.exists("ptstop-files")) {
    //         NIXMLPTHandler ptHandler(myNetBuilder.getEdgeCont(),
    //                                  myNetBuilder.getPTStopCont(), myNetBuilder.getPTLineCont());
    //         ok = NITypeLoader::load(ptHandler, oc.getStringVector("ptstop-files"), "public transport stops");
    //     }
    //
    //     // load public transport lines (used as input when repairing railroad topology)
    //     if (ok && oc.exists("ptline-files")) {
    //         NIXMLPTHandler ptHandler(myNetBuilder.getEdgeCont(),
    //                                  myNetBuilder.getPTStopCont(), myNetBuilder.getPTLineCont());
    //         ok = NITypeLoader::load(ptHandler, oc.getStringVector("ptline-files"), "public transport lines");
    //     }
    //
    //     // load shapes for output formats that embed shape data
    //     if (ok && oc.exists("polygon-files")) {
    //         NIXMLShapeHandler shapeHandler(myNetBuilder.getShapeCont(), myNetBuilder.getEdgeCont());
    //         ok = NITypeLoader::load(shapeHandler, oc.getStringVector("polygon-files"), "polygon data");
    //     }
    //     return ok;
    // }
    #[allow(unused_variables)]
    pub fn load_xml(&mut self, oc: &mut OptionsCont) -> bool {
        // load nodes
        let mut nodesHandler = NIXMLNodesHandler::new(self.myNetBuilder.m().get_node_cont(), self.myNetBuilder.m().get_edge_cont(), self.myNetBuilder.m().get_tl_logic_cont(), oc);
        let mut ok = NITypeLoader::load(&mut nodesHandler, throw(oc.get_string_vector("node-files")).as_slice(), "nodes", false);
        // load the edges
        if ok {
            let mut edgesHandler = NIXMLEdgesHandler::new(self.myNetBuilder.m().get_node_cont(), self.myNetBuilder.m().get_edge_cont(), self.myNetBuilder.m().get_type_cont(), self.myNetBuilder.m().get_district_cont(), self.myNetBuilder.m().get_tl_logic_cont(), oc);
            ok = NITypeLoader::load(&mut edgesHandler, throw(oc.get_string_vector("edge-files")).as_slice(), "edges", false);
        }
        let seen = DEPRECATED_VEHICLE_CLASSES_SEEN.with(|s| s.borrow().clone());
        if !seen.is_empty() {
            write_warning(&format!("Deprecated vehicle class(es) '{}' in input edge files.", seen.iter().cloned().collect::<Vec<_>>().join(" ")));
        }
        // load the connections
        if ok {
            let mut connectionsHandler = NIXMLConnectionsHandler::new(self.myNetBuilder.m().get_edge_cont(), self.myNetBuilder.m().get_node_cont(), self.myNetBuilder.m().get_tl_logic_cont());
            ok = NITypeLoader::load(&mut connectionsHandler, throw(oc.get_string_vector("connection-files")).as_slice(), "connections", false);
        }
        // load traffic lights (needs to come last, references loaded edges and connections)
        if ok {
            let mut tlHandler = NIXMLTrafficLightsHandler::new(self.myNetBuilder.m().get_tl_logic_cont(), self.myNetBuilder.m().get_edge_cont(), false);
            ok = NITypeLoader::load(&mut tlHandler, throw(oc.get_string_vector("tllogic-files")).as_slice(), "traffic lights", false);
        }

        // load public transport stops (used for restricting edge removal and as input when repairing railroad topology)
        if ok && oc.exists("ptstop-files") {
            let mut ptHandler = NIXMLPTHandler::new(self.myNetBuilder.m().get_edge_cont(), self.myNetBuilder.m().get_pt_stop_cont(), self.myNetBuilder.m().get_pt_line_cont());
            ok = NITypeLoader::load(&mut ptHandler, throw(oc.get_string_vector("ptstop-files")).as_slice(), "public transport stops", false);
        }

        // load public transport lines (used as input when repairing railroad topology)
        if ok && oc.exists("ptline-files") {
            let mut ptHandler = NIXMLPTHandler::new(self.myNetBuilder.m().get_edge_cont(), self.myNetBuilder.m().get_pt_stop_cont(), self.myNetBuilder.m().get_pt_line_cont());
            ok = NITypeLoader::load(&mut ptHandler, throw(oc.get_string_vector("ptline-files")).as_slice(), "public transport lines", false);
        }

        // load shapes for output formats that embed shape data
        if ok && oc.exists("polygon-files") {
            let mut shapeHandler = NIXMLShapeHandler::new(self.myNetBuilder.m().get_shape_cont(), self.myNetBuilder.m().get_edge_cont());
            ok = NITypeLoader::load(&mut shapeHandler, throw(oc.get_string_vector("polygon-files")).as_slice(), "polygon data", false);
        }
        ok
    }

    /// C++: `void NILoader::NILoader(const NILoader &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &NILoader) -> Self {
        // the copy constructor is declared private and never defined in C++
        panic!("NILoader is not copyable")
    }

    /// C++: `NILoader & NILoader::operator=(const NILoader &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &NILoader) -> &mut NILoader {
        // the assignment operator is declared private and never defined in C++
        panic!("NILoader is not assignable")
    }

}

