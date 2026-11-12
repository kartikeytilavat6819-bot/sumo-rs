//! Port of netimport/NIFrame.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::msg_handler::{write_error, write_warning};
use utils::tlf;

/// OptionsCont lookups return Result (C++ throws ProcessError); options used here are all registered
/// in fillOptions, so a failure is a programming error and is reported as a panic.
fn must<T>(r: Result<T, ProcessError>) -> T {
    r.unwrap_or_else(|e| panic!("{e}"))
}

/// Sets and checks options for netimport
/// C++ class `NIFrame` (netimport/NIFrame.h)
pub struct NIFrame {
}

impl NIFrame {
    /// C++: `void NIFrame::fillOptions(OptionsCont &, bool)` [static]
    /// Inserts options used by the network importer and network building modules
    // void
    // NIFrame::fillOptions(OptionsCont& oc, bool forNetedit) {
    //     // register input formats
    //     oc.doRegister("sumo-net-file", 's', new Option_FileName());
    //     oc.addSynonyme("sumo-net-file", "sumo-net", true);
    //     oc.addSynonyme("sumo-net-file", "net-file");
    //     oc.addDescription("sumo-net-file", "Input", TL("Read SUMO-net from FILE"));
    //     oc.addXMLDefault("sumo-net-file", "net");
    //
    //     oc.doRegister("node-files", 'n', new Option_FileName());
    //     oc.addSynonyme("node-files", "xml-node-files", true);
    //     oc.addSynonyme("node-files", "xml-nodes", true);
    //     oc.addSynonyme("node-files", "nodes");
    //     oc.addDescription("node-files", "Input", TL("Read XML-node defs from FILE"));
    //
    //     oc.doRegister("edge-files", 'e', new Option_FileName());
    //     oc.addSynonyme("edge-files", "xml-edge-files", true);
    //     oc.addSynonyme("edge-files", "xml-edges", true);
    //     oc.addSynonyme("edge-files", "edges");
    //     oc.addDescription("edge-files", "Input", TL("Read XML-edge defs from FILE"));
    //
    //     oc.doRegister("connection-files", 'x', new Option_FileName());
    //     oc.addSynonyme("connection-files", "xml-connection-files", true);
    //     oc.addSynonyme("connection-files", "xml-connections", true);
    //     oc.addSynonyme("connection-files", "connections");
    //     oc.addDescription("connection-files", "Input", TL("Read XML-connection defs from FILE"));
    //
    //     oc.doRegister("tllogic-files", 'i', new Option_FileName());
    //     oc.addDescription("tllogic-files", "Input", TL("Read XML-traffic light defs from FILE"));
    //
    //     oc.doRegister("type-files", 't', new Option_FileName());
    //     oc.addSynonyme("type-files", "xml-type-files", true);
    //     oc.addSynonyme("type-files", "xml-types", true);
    //     oc.addSynonyme("type-files", "types");
    //     oc.addDescription("type-files", "Input", TL("Read XML-type defs from FILE"));
    //
    //     if (!forNetedit) {
    //         // would cause confusion because netedit loads stops and shapes using option --additional-files
    //         oc.doRegister("ptstop-files", new Option_FileName());
    //         oc.addDescription("ptstop-files", "Input", TL("Reads public transport stops from FILE"));
    //         oc.doRegister("ptline-files", new Option_FileName());
    //         oc.addDescription("ptline-files", "Input", TL("Reads public transport lines from FILE"));
    //         oc.doRegister("polygon-files", new Option_FileName());
    //         oc.addDescription("polygon-files", "Input", TL("Reads polygons from FILE for embedding in network where applicable"));
    //     }
    //
    //     oc.doRegister("shapefile-prefix", new Option_FileName());
    //     oc.addSynonyme("shapefile-prefix", "shapefile");
    //     oc.addSynonyme("shapefile-prefix", "arcview", true);
    //     oc.addSynonyme("shapefile-prefix", "tiger", true);
    //     oc.addDescription("shapefile-prefix", "Input", TL("Read shapefiles (ArcView, Tiger, ...) from files starting with 'FILE'"));
    //
    //     oc.doRegister("dlr-navteq-prefix", new Option_FileName());
    //     oc.addSynonyme("dlr-navteq-prefix", "dlr-navteq");
    //     oc.addSynonyme("dlr-navteq-prefix", "elmar2", true);
    //     oc.addDescription("dlr-navteq-prefix", "Input", TL("Read converted Navteq GDF data (unsplitted Elmar-network) from path 'FILE'"));
    //
    //     oc.doRegister("osm-files", new Option_FileName());
    //     oc.addSynonyme("osm-files", "osm");
    //     oc.addDescription("osm-files", "Input", TL("Read OSM-network from path 'FILE(s)'"));
    //
    //     oc.doRegister("opendrive-files", new Option_FileName());
    //     oc.addSynonyme("opendrive-files", "opendrive");
    //     oc.addDescription("opendrive-files", "Input", TL("Read OpenDRIVE-network from FILE"));
    //
    //     oc.doRegister("visum-file", new Option_FileName());
    //     oc.addSynonyme("visum-file", "visum");
    //     oc.addDescription("visum-file", "Input", TL("Read VISUM-net from FILE"));
    //
    //     oc.doRegister("vissim-file", new Option_FileName());
    //     oc.addSynonyme("vissim-file", "vissim");
    //     oc.addDescription("vissim-file", "Input", TL("Read VISSIM-net from FILE"));
    //
    //     oc.doRegister("robocup-dir", new Option_FileName());
    //     oc.addSynonyme("robocup-dir", "robocup-net", true);
    //     oc.addSynonyme("robocup-dir", "robocup");
    //     oc.addDescription("robocup-dir", "Input", TL("Read RoboCup-net from DIR"));
    //
    //     oc.doRegister("matsim-files", new Option_FileName());
    //     oc.addSynonyme("matsim-files", "matsim");
    //     oc.addDescription("matsim-files", "Input", TL("Read MATSim-net from FILE"));
    //
    //     oc.doRegister("itsumo-files", new Option_FileName());
    //     oc.addSynonyme("itsumo-files", "itsumo");
    //     oc.addDescription("itsumo-files", "Input", TL("Read ITSUMO-net from FILE"));
    //
    //     oc.doRegister("heightmap.shapefiles", new Option_FileName());
    //     oc.addDescription("heightmap.shapefiles", "Input", TL("Read heightmap from ArcGIS shapefile"));
    //
    //     oc.doRegister("heightmap.geotiff", new Option_FileName());
    //     oc.addDescription("heightmap.geotiff", "Input", TL("Read heightmap from GeoTIFF"));
    //
    //     // need to do this here to be able to check for network and route input options
    //     SystemFrame::addReportOptions(oc);
    //
    //     // register basic processing options
    //     oc.doRegister("ignore-errors", new Option_Bool(false));
    //     oc.addSynonyme("ignore-errors", "dismiss-loading-errors", true);
    //     oc.addDescription("ignore-errors", "Report", TL("Continue on broken input"));
    //
    //     oc.doRegister("ignore-errors.connections", new Option_Bool(false));
    //     oc.addDescription("ignore-errors.connections", "Report", TL("Continue on invalid connections"));
    //
    //     oc.doRegister("show-errors.connections-first-try", new Option_Bool(false));
    //     oc.addDescription("show-errors.connections-first-try", "Report", TL("Show errors in connections at parsing"));
    //
    //     oc.doRegister("ignore-errors.edge-type", new Option_Bool(false));
    //     oc.addDescription("ignore-errors.edge-type", "Report", TL("Continue on unknown edge types"));
    //
    //     oc.doRegister("speed-in-kmh", new Option_Bool(false));
    //     oc.addDescription("speed-in-kmh", "Processing", TL("vmax is parsed as given in km/h"));
    //
    //     oc.doRegister("construction-date", new Option_String());
    //     oc.addDescription("construction-date", "Processing", TL("Use YYYY-MM-DD date to determine the readiness of features under construction"));
    //
    //     oc.doRegister("flatten", new Option_Bool(false));
    //     oc.addDescription("flatten", "Processing", TL("Remove all z-data"));
    //
    //     oc.doRegister("discard-params", new Option_StringVector());
    //     oc.addDescription("discard-params", "Formats", TL("Remove the list of keys from all params"));
    //
    //     oc.doRegister("ignore-change-restrictions", new Option_StringVector(StringVector({"authority"})));
    //     oc.addDescription("ignore-change-restrictions", "Formats", TL("List vehicle classes that may ignore lane changing restrictions ('all' discards all restrictions)"));
    //
    //     oc.doRegister("ignore-widths", new Option_Bool(false));
    //     oc.addSynonyme("ignore-widths", "opendrive.ignore-widths", false);
    //     oc.addDescription("ignore-widths", "Formats", TL("Whether lane widths shall be ignored."));
    //
    //     // register xml options
    //     oc.doRegister("plain.extend-edge-shape", new Option_Bool(false));
    //     oc.addSynonyme("plain.extend-edge-shape", "xml.keep-shape", true);
    //     oc.addDescription("plain.extend-edge-shape", "Processing", TL("If edge shapes do not end at the node positions, extend them"));
    //
    //     // register osm options
    //     oc.doRegister("osm.skip-duplicates-check", new Option_Bool(false));
    //     oc.addDescription("osm.skip-duplicates-check", "Formats", TL("Skips the check for duplicate nodes and edges"));
    //
    //     oc.doRegister("osm.elevation", new Option_Bool(false));
    //     oc.addDescription("osm.elevation", "Formats", TL("Imports elevation data"));
    //
    //     oc.doRegister("osm.layer-elevation", new Option_Float(0));
    //     oc.addDescription("osm.layer-elevation", "Formats", TL("Reconstruct (relative) elevation based on layer data. Each layer is raised by FLOAT m"));
    //
    //     oc.doRegister("osm.layer-elevation.max-grade", new Option_Float(10));
    //     oc.addDescription("osm.layer-elevation.max-grade", "Formats", TL("Maximum grade threshold in % at 50km/h when reconstructing elevation based on layer data. The value is scaled according to road speed."));
    //
    //     oc.doRegister("osm.oneway-spread-right", new Option_Bool(false));
    //     oc.addDescription("osm.oneway-spread-right", "Formats", TL("Whether one-way roads should be spread to the side instead of centered"));
    //
    //     oc.doRegister("osm.lane-access", new Option_Bool(false));
    //     oc.addDescription("osm.lane-access", "Formats", TL("Import lane-specific access restrictions"));
    //
    //     oc.doRegister("osm.bike-access", new Option_Bool(false));
    //     oc.addSynonyme("osm.bike-access", "osm.bike-lanes");
    //     oc.addDescription("osm.bike-access", "Formats", TL("Import bike lanes and fix directions and permissions on bike paths"));
    //
    //     oc.doRegister("osm.sidewalks", new Option_Bool(false));
    //     oc.addDescription("osm.sidewalks", "Formats", TL("Import sidewalks"));
    //
    //     oc.doRegister("osm.oneway-reverse-sidewalk", new Option_Bool(false));
    //     oc.addDescription("osm.oneway-reverse-sidewalk", "Formats", TL("Default to building two sidewalks on oneway streets (may affect divided roads)"));
    //
    //     oc.doRegister("osm.crossings", new Option_Bool(false));
    //     oc.addDescription("osm.crossings", "Formats", TL("Import crossings"));
    //
    //     oc.doRegister("osm.turn-lanes", new Option_Bool(false));
    //     oc.addDescription("osm.turn-lanes", "Formats", TL("Import turning arrows from OSM to help with connection building"));
    //
    //     oc.doRegister("osm.stop-output.length", new Option_Float(25));
    //     oc.addDescription("osm.stop-output.length", "Formats", TL("The default length of a public transport stop in FLOAT m"));
    //     oc.doRegister("osm.stop-output.length.bus", new Option_Float(15));
    //     oc.addDescription("osm.stop-output.length.bus", "Formats", TL("The default length of a bus stop in FLOAT m"));
    //     oc.doRegister("osm.stop-output.length.tram", new Option_Float(25));
    //     oc.addDescription("osm.stop-output.length.tram", "Formats", TL("The default length of a tram stop in FLOAT m"));
    //     oc.doRegister("osm.stop-output.length.train", new Option_Float(200));
    //     oc.addDescription("osm.stop-output.length.train", "Formats", TL("The default length of a train stop in FLOAT m"));
    //
    //     oc.doRegister("osm.railsignals", new Option_StringVector(StringVector({ "DEFAULT"})));
    //     oc.addDescription("osm.railsignals", "Formats", TL("Specify custom rules for importing railway signals"));
    //
    //     oc.doRegister("osm.all-attributes", new Option_Bool(false));
    //     oc.addSynonyme("osm.all-attributes", "osm.all-tags");
    //     oc.addDescription("osm.all-attributes", "Formats", TL("Whether additional attributes shall be imported"));
    //
    //     oc.doRegister("osm.extra-attributes", new Option_StringVector(StringVector({ "all" })));
    //     oc.addSynonyme("osm.extra-attributes", "osm.extra-tags");
    //     oc.addDescription("osm.extra-attributes", "Formats", TL("List of additional attributes that shall be imported from OSM via osm.all-attributes (set 'all' to import all)"));
    //
    //     oc.doRegister("osm.speedlimit-none", new Option_Float(39.4444));
    //     oc.addDescription("osm.speedlimit-none", "Formats", TL("The speed limit to be set when there is no actual speed limit in reality"));
    //
    //     oc.doRegister("osm.annotate-defaults", new Option_Bool(false));
    //     oc.addDescription("osm.annotate-defaults", "Formats", TL("Whether edges shoulds carry information on the usage of typemap defaults"));
    //
    //     oc.doRegister("osm.etcs-moving-block", new Option_Bool(false));
    //     oc.addDescription("osm.etcs-moving-block", "Formats", TL("Whether ETCS2 track sections should default to moving-block mode"));
    //
    //     // register matsim options
    //     oc.doRegister("matsim.keep-length", new Option_Bool(false));
    //     oc.addDescription("matsim.keep-length", "Formats", TL("The edge lengths given in the MATSim-file will be kept"));
    //
    //     oc.doRegister("matsim.lanes-from-capacity", new Option_Bool(false));
    //     oc.addDescription("matsim.lanes-from-capacity", "Formats", TL("The lane number will be computed from the capacity"));
    //
    //
    //     // register shapefile options
    //     oc.doRegister("shapefile.street-id", new Option_String());
    //     oc.addSynonyme("shapefile.street-id", "arcview.street-id", true);
    //     oc.addDescription("shapefile.street-id", "Formats", TL("Read edge ids from column STR"));
    //
    //     oc.doRegister("shapefile.from-id", new Option_String());
    //     oc.addSynonyme("shapefile.from-id", "arcview.from-id", true);
    //     oc.addDescription("shapefile.from-id", "Formats", TL("Read from-node ids from column STR"));
    //
    //     oc.doRegister("shapefile.to-id", new Option_String());
    //     oc.addSynonyme("shapefile.to-id", "arcview.to-id", true);
    //     oc.addDescription("shapefile.to-id", "Formats", TL("Read to-node ids from column STR"));
    //
    //     oc.doRegister("shapefile.type-id", new Option_String());
    //     oc.addSynonyme("shapefile.type-id", "arcview.type-id", true);
    //     oc.addDescription("shapefile.type-id", "Formats", TL("Read type ids from column STR"));
    //
    //     oc.doRegister("shapefile.laneNumber", new Option_String());
    //     oc.addDescription("shapefile.laneNumber", "Formats", TL("Read lane number from column STR"));
    //
    //     oc.doRegister("shapefile.speed", new Option_String());
    //     oc.addDescription("shapefile.speed", "Formats", TL("Read speed from column STR"));
    //
    //     oc.doRegister("shapefile.length", new Option_String());
    //     oc.addDescription("shapefile.length", "Formats", TL("Read custom edge length from column STR"));
    //
    //     oc.doRegister("shapefile.width", new Option_String());
    //     oc.addDescription("shapefile.width", "Formats", TL("Read total edge width from column STR"));
    //
    //     oc.doRegister("shapefile.name", new Option_String());
    //     oc.addDescription("shapefile.name", "Formats", TL("Read (non-unique) name from column STR"));
    //
    //     oc.doRegister("shapefile.node-join-dist", new Option_Float(0));
    //     oc.addDescription("shapefile.node-join-dist", "Formats", TL("Distance threshold for determining whether distinct shapes are connected (used when from-id and to-id are not available)"));
    //
    //     oc.doRegister("shapefile.add-params", new Option_StringVector());
    //     oc.addDescription("shapefile.add-params", "Formats", TL("Add the list of field names as edge params"));
    //
    //     oc.doRegister("shapefile.use-defaults-on-failure", new Option_Bool(false));
    //     oc.addSynonyme("shapefile.use-defaults-on-failure", "arcview.use-defaults-on-failure", true);
    //     oc.addDescription("shapefile.use-defaults-on-failure", "Formats", TL("Uses edge type defaults on problems"));
    //
    //     oc.doRegister("shapefile.all-bidirectional", new Option_Bool(false));
    //     oc.addSynonyme("shapefile.all-bidirectional", "shapefile.all-bidi");
    //     oc.addSynonyme("shapefile.all-bidirectional", "arcview.all-bidi", true);
    //     oc.addDescription("shapefile.all-bidirectional", "Formats", TL("Insert edges in both directions"));
    //
    //     oc.doRegister("shapefile.guess-projection", new Option_Bool(false));
    //     oc.addSynonyme("shapefile.guess-projection", "arcview.guess-projection", true);
    //     oc.addDescription("shapefile.guess-projection", "Formats", TL("Guess the proper projection"));
    //
    //     oc.doRegister("shapefile.traditional-axis-mapping", new Option_Bool(false));
    //     oc.addDescription("shapefile.traditional-axis-mapping", "Formats", TL("Use traditional axis order (lon, lat)"));
    //
    //
    //     // register dlr-navteq options
    //     oc.doRegister("dlr-navteq.tolerant-permissions", new Option_Bool(false));
    //     oc.addDescription("dlr-navteq.tolerant-permissions", "Formats", TL("Allow more vehicle classes by default"));
    //
    //     oc.doRegister("dlr-navteq.keep-length", new Option_Bool(false));
    //     oc.addDescription("dlr-navteq.keep-length", "Formats", TL("The edge lengths given in the DLR Navteq-file will be kept"));
    //
    //
    //     // register vissim options
    //     oc.doRegister("vissim.join-distance", new Option_Float(5.0f));
    //     oc.addSynonyme("vissim.join-distance", "vissim.offset", true);
    //     oc.addDescription("vissim.join-distance", "Formats", TL("Structure join offset"));
    //
    //     oc.doRegister("vissim.default-speed", new Option_Float(50.0f));
    //     oc.addDescription("vissim.default-speed", "Formats", TL("Use FLOAT as default speed"));
    //
    //     oc.doRegister("vissim.speed-norm", new Option_Float(1.0f));
    //     oc.addDescription("vissim.speed-norm", "Formats", TL("Factor for edge velocity"));
    //
    //     oc.doRegister("vissim.report-unset-speeds", new Option_Bool(false));
    //     oc.addDescription("vissim.report-unset-speeds", "Formats", TL("Writes lanes without an explicit speed set"));
    //
    //
    //     // register visum options
    //     oc.doRegister("visum.language-file", new Option_FileName());
    //     oc.addDescription("visum.language-file", "Formats", TL("Load language mappings from FILE"));
    //
    //     oc.doRegister("visum.use-type-priority", new Option_Bool(false));
    //     oc.addDescription("visum.use-type-priority", "Formats", TL("Uses priorities from types"));
    //
    //     oc.doRegister("visum.use-type-laneno", new Option_Bool(false));
    //     oc.addDescription("visum.use-type-laneno", "Formats", TL("Uses lane numbers from types"));
    //
    //     oc.doRegister("visum.use-type-speed", new Option_Bool(false));
    //     oc.addDescription("visum.use-type-speed", "Formats", TL("Uses speeds from types"));
    //
    //     oc.doRegister("visum.connector-speeds", new Option_Float(100.));
    //     oc.addDescription("visum.connector-speeds", "Formats", TL("Sets connector speed"));
    //
    //     oc.doRegister("visum.connectors-lane-number", new Option_Integer(3));
    //     oc.addSynonyme("visum.connectors-lane-number", "visum.connector-laneno", true);
    //     oc.addDescription("visum.connectors-lane-number", "Formats", TL("Sets connector lane number"));
    //
    //     oc.doRegister("visum.no-connectors", new Option_Bool(true));
    //     oc.addDescription("visum.no-connectors", "Formats", TL("Excludes connectors"));
    //
    //     oc.doRegister("visum.recompute-lane-number", new Option_Bool(false));
    //     oc.addSynonyme("visum.recompute-lane-number", "visum.recompute-laneno", true);
    //     oc.addDescription("visum.recompute-lane-number", "Formats", TL("Computes the number of lanes from the edges' capacities"));
    //
    //     oc.doRegister("visum.verbose-warnings", new Option_Bool(false));
    //     oc.addDescription("visum.verbose-warnings", "Formats", TL("Prints all warnings, some of which are due to VISUM misbehaviour"));
    //
    //     oc.doRegister("visum.lanes-from-capacity.norm", new Option_Float(1800.));
    //     oc.addSynonyme("visum.lanes-from-capacity.norm", "capacity-norm", true);
    //     oc.addSynonyme("visum.lanes-from-capacity.norm", "lanes-from-capacity.norm");
    //     oc.addDescription("visum.lanes-from-capacity.norm", "Formats", TL("The factor for flow to no. lanes conversion"));
    //
    //
    //     // register opendrive options
    //     oc.doRegister("opendrive.import-all-lanes", new Option_Bool(false));
    //     oc.addDescription("opendrive.import-all-lanes", "Formats", TL("Imports all lane types"));
    //     oc.doRegister("opendrive.curve-resolution", new Option_Float(2.0));
    //     oc.addDescription("opendrive.curve-resolution", "Formats", TL("The geometry resolution in m when importing curved geometries as line segments."));
    //     oc.doRegister("opendrive.advance-stopline", new Option_Float(0.0));
    //     oc.addDescription("opendrive.advance-stopline", "Formats", TL("Allow stop lines to be built beyond the start of the junction if the geometries allow so"));
    //     oc.doRegister("opendrive.min-width", new Option_Float(1.8));
    //     oc.addDescription("opendrive.min-width", "Formats", TL("The minimum lane width for determining start or end of variable-width lanes"));
    //     oc.doRegister("opendrive.internal-shapes", new Option_Bool(false));
    //     oc.addDescription("opendrive.internal-shapes", "Formats", TL("Import internal lane shapes"));
    //     oc.doRegister("opendrive.position-ids", new Option_Bool(false));
    //     oc.addDescription("opendrive.position-ids", "Formats", TL("Sets edge-id based on road-id and offset in m (legacy)"));
    //     oc.doRegister("opendrive.lane-shapes", new Option_Bool(false));
    //     oc.addDescription("opendrive.lane-shapes", "Formats", TL("Use custom lane shapes to compensate discarded lane types"));
    //     oc.doRegister("opendrive.signal-groups", new Option_Bool(false));
    //     oc.addDescription("opendrive.signal-groups", "Formats", TL("Use the OpenDRIVE controller information for the generated signal program"));
    //     oc.doRegister("opendrive.ignore-misplaced-signals", new Option_Bool(false));
    //     oc.addDescription("opendrive.ignore-misplaced-signals", "Formats", TL("Ignore traffic signals which do not control any driving lane"));
    //
    //     // register some additional options
    //     oc.doRegister("tls.discard-loaded", new Option_Bool(false));
    //     oc.addDescription("tls.discard-loaded", "TLS Building", "Does not instantiate traffic lights loaded from other formats than plain-XML");
    //
    //     oc.doRegister("tls.discard-simple", new Option_Bool(false));
    //     oc.addDescription("tls.discard-simple", "TLS Building", "Does not instantiate traffic lights at geometry-like nodes loaded from other formats than plain-XML");
    //
    //     // register railway options
    //     oc.doRegister("railway.signals.discard", new Option_Bool(false));
    //     oc.addDescription("railway.signals.discard", "Railway", TL("Discard all railway signal information loaded from other formats than plain-xml"));
    // }
    #[allow(unused_variables)]
    pub fn fill_options(oc: &mut OptionsCont, forNetedit: bool) {
        // register input formats
        oc.do_register_abbr("sumo-net-file", 's', SumoOption::file_name());
        oc.add_synonyme("sumo-net-file", "sumo-net", true);
        oc.add_synonyme("sumo-net-file", "net-file", false);
        oc.add_description("sumo-net-file", "Input", "Read SUMO-net from FILE");
        oc.add_xml_default("sumo-net-file", "net");

        oc.do_register_abbr("node-files", 'n', SumoOption::file_name());
        oc.add_synonyme("node-files", "xml-node-files", true);
        oc.add_synonyme("node-files", "xml-nodes", true);
        oc.add_synonyme("node-files", "nodes", false);
        oc.add_description("node-files", "Input", "Read XML-node defs from FILE");

        oc.do_register_abbr("edge-files", 'e', SumoOption::file_name());
        oc.add_synonyme("edge-files", "xml-edge-files", true);
        oc.add_synonyme("edge-files", "xml-edges", true);
        oc.add_synonyme("edge-files", "edges", false);
        oc.add_description("edge-files", "Input", "Read XML-edge defs from FILE");

        oc.do_register_abbr("connection-files", 'x', SumoOption::file_name());
        oc.add_synonyme("connection-files", "xml-connection-files", true);
        oc.add_synonyme("connection-files", "xml-connections", true);
        oc.add_synonyme("connection-files", "connections", false);
        oc.add_description("connection-files", "Input", "Read XML-connection defs from FILE");

        oc.do_register_abbr("tllogic-files", 'i', SumoOption::file_name());
        oc.add_description("tllogic-files", "Input", "Read XML-traffic light defs from FILE");

        oc.do_register_abbr("type-files", 't', SumoOption::file_name());
        oc.add_synonyme("type-files", "xml-type-files", true);
        oc.add_synonyme("type-files", "xml-types", true);
        oc.add_synonyme("type-files", "types", false);
        oc.add_description("type-files", "Input", "Read XML-type defs from FILE");

        if !forNetedit {
            // would cause confusion because netedit loads stops and shapes using option --additional-files
            oc.do_register("ptstop-files", SumoOption::file_name());
            oc.add_description("ptstop-files", "Input", "Reads public transport stops from FILE");
            oc.do_register("ptline-files", SumoOption::file_name());
            oc.add_description("ptline-files", "Input", "Reads public transport lines from FILE");
            oc.do_register("polygon-files", SumoOption::file_name());
            oc.add_description("polygon-files", "Input", "Reads polygons from FILE for embedding in network where applicable");
        }

        oc.do_register("shapefile-prefix", SumoOption::file_name());
        oc.add_synonyme("shapefile-prefix", "shapefile", false);
        oc.add_synonyme("shapefile-prefix", "arcview", true);
        oc.add_synonyme("shapefile-prefix", "tiger", true);
        oc.add_description("shapefile-prefix", "Input", "Read shapefiles (ArcView, Tiger, ...) from files starting with 'FILE'");

        oc.do_register("dlr-navteq-prefix", SumoOption::file_name());
        oc.add_synonyme("dlr-navteq-prefix", "dlr-navteq", false);
        oc.add_synonyme("dlr-navteq-prefix", "elmar2", true);
        oc.add_description("dlr-navteq-prefix", "Input", "Read converted Navteq GDF data (unsplitted Elmar-network) from path 'FILE'");

        oc.do_register("osm-files", SumoOption::file_name());
        oc.add_synonyme("osm-files", "osm", false);
        oc.add_description("osm-files", "Input", "Read OSM-network from path 'FILE(s)'");

        oc.do_register("opendrive-files", SumoOption::file_name());
        oc.add_synonyme("opendrive-files", "opendrive", false);
        oc.add_description("opendrive-files", "Input", "Read OpenDRIVE-network from FILE");

        oc.do_register("visum-file", SumoOption::file_name());
        oc.add_synonyme("visum-file", "visum", false);
        oc.add_description("visum-file", "Input", "Read VISUM-net from FILE");

        oc.do_register("vissim-file", SumoOption::file_name());
        oc.add_synonyme("vissim-file", "vissim", false);
        oc.add_description("vissim-file", "Input", "Read VISSIM-net from FILE");

        oc.do_register("robocup-dir", SumoOption::file_name());
        oc.add_synonyme("robocup-dir", "robocup-net", true);
        oc.add_synonyme("robocup-dir", "robocup", false);
        oc.add_description("robocup-dir", "Input", "Read RoboCup-net from DIR");

        oc.do_register("matsim-files", SumoOption::file_name());
        oc.add_synonyme("matsim-files", "matsim", false);
        oc.add_description("matsim-files", "Input", "Read MATSim-net from FILE");

        oc.do_register("itsumo-files", SumoOption::file_name());
        oc.add_synonyme("itsumo-files", "itsumo", false);
        oc.add_description("itsumo-files", "Input", "Read ITSUMO-net from FILE");

        oc.do_register("heightmap.shapefiles", SumoOption::file_name());
        oc.add_description("heightmap.shapefiles", "Input", "Read heightmap from ArcGIS shapefile");

        oc.do_register("heightmap.geotiff", SumoOption::file_name());
        oc.add_description("heightmap.geotiff", "Input", "Read heightmap from GeoTIFF");

        // need to do this here to be able to check for network and route input options
        // shortcut: SystemFrame is not ported yet, so SystemFrame::addReportOptions(oc) (verbose, log, ...) is not called here, add it once system_frame.rs exists
        // register basic processing options
        oc.do_register("ignore-errors", SumoOption::bool(false));
        oc.add_synonyme("ignore-errors", "dismiss-loading-errors", true);
        oc.add_description("ignore-errors", "Report", "Continue on broken input");

        oc.do_register("ignore-errors.connections", SumoOption::bool(false));
        oc.add_description("ignore-errors.connections", "Report", "Continue on invalid connections");

        oc.do_register("show-errors.connections-first-try", SumoOption::bool(false));
        oc.add_description("show-errors.connections-first-try", "Report", "Show errors in connections at parsing");

        oc.do_register("ignore-errors.edge-type", SumoOption::bool(false));
        oc.add_description("ignore-errors.edge-type", "Report", "Continue on unknown edge types");

        oc.do_register("speed-in-kmh", SumoOption::bool(false));
        oc.add_description("speed-in-kmh", "Processing", "vmax is parsed as given in km/h");

        oc.do_register("construction-date", SumoOption::string());
        oc.add_description("construction-date", "Processing", "Use YYYY-MM-DD date to determine the readiness of features under construction");

        oc.do_register("flatten", SumoOption::bool(false));
        oc.add_description("flatten", "Processing", "Remove all z-data");

        oc.do_register("discard-params", SumoOption::string_vector());
        oc.add_description("discard-params", "Formats", "Remove the list of keys from all params");

        oc.do_register("ignore-change-restrictions", SumoOption::string_vector_with(vec!["authority".to_string()]));
        oc.add_description("ignore-change-restrictions", "Formats", "List vehicle classes that may ignore lane changing restrictions ('all' discards all restrictions)");

        oc.do_register("ignore-widths", SumoOption::bool(false));
        oc.add_synonyme("ignore-widths", "opendrive.ignore-widths", false);
        oc.add_description("ignore-widths", "Formats", "Whether lane widths shall be ignored.");

        // register xml options
        oc.do_register("plain.extend-edge-shape", SumoOption::bool(false));
        oc.add_synonyme("plain.extend-edge-shape", "xml.keep-shape", true);
        oc.add_description("plain.extend-edge-shape", "Processing", "If edge shapes do not end at the node positions, extend them");

        // register osm options
        oc.do_register("osm.skip-duplicates-check", SumoOption::bool(false));
        oc.add_description("osm.skip-duplicates-check", "Formats", "Skips the check for duplicate nodes and edges");

        oc.do_register("osm.elevation", SumoOption::bool(false));
        oc.add_description("osm.elevation", "Formats", "Imports elevation data");

        oc.do_register("osm.layer-elevation", SumoOption::float(0.0));
        oc.add_description("osm.layer-elevation", "Formats", "Reconstruct (relative) elevation based on layer data. Each layer is raised by FLOAT m");

        oc.do_register("osm.layer-elevation.max-grade", SumoOption::float(10.0));
        oc.add_description("osm.layer-elevation.max-grade", "Formats", "Maximum grade threshold in % at 50km/h when reconstructing elevation based on layer data. The value is scaled according to road speed.");

        oc.do_register("osm.oneway-spread-right", SumoOption::bool(false));
        oc.add_description("osm.oneway-spread-right", "Formats", "Whether one-way roads should be spread to the side instead of centered");

        oc.do_register("osm.lane-access", SumoOption::bool(false));
        oc.add_description("osm.lane-access", "Formats", "Import lane-specific access restrictions");

        oc.do_register("osm.bike-access", SumoOption::bool(false));
        oc.add_synonyme("osm.bike-access", "osm.bike-lanes", false);
        oc.add_description("osm.bike-access", "Formats", "Import bike lanes and fix directions and permissions on bike paths");

        oc.do_register("osm.sidewalks", SumoOption::bool(false));
        oc.add_description("osm.sidewalks", "Formats", "Import sidewalks");

        oc.do_register("osm.oneway-reverse-sidewalk", SumoOption::bool(false));
        oc.add_description("osm.oneway-reverse-sidewalk", "Formats", "Default to building two sidewalks on oneway streets (may affect divided roads)");

        oc.do_register("osm.crossings", SumoOption::bool(false));
        oc.add_description("osm.crossings", "Formats", "Import crossings");

        oc.do_register("osm.turn-lanes", SumoOption::bool(false));
        oc.add_description("osm.turn-lanes", "Formats", "Import turning arrows from OSM to help with connection building");

        oc.do_register("osm.stop-output.length", SumoOption::float(25.0));
        oc.add_description("osm.stop-output.length", "Formats", "The default length of a public transport stop in FLOAT m");
        oc.do_register("osm.stop-output.length.bus", SumoOption::float(15.0));
        oc.add_description("osm.stop-output.length.bus", "Formats", "The default length of a bus stop in FLOAT m");
        oc.do_register("osm.stop-output.length.tram", SumoOption::float(25.0));
        oc.add_description("osm.stop-output.length.tram", "Formats", "The default length of a tram stop in FLOAT m");
        oc.do_register("osm.stop-output.length.train", SumoOption::float(200.0));
        oc.add_description("osm.stop-output.length.train", "Formats", "The default length of a train stop in FLOAT m");

        oc.do_register("osm.railsignals", SumoOption::string_vector_with(vec!["DEFAULT".to_string()]));
        oc.add_description("osm.railsignals", "Formats", "Specify custom rules for importing railway signals");

        oc.do_register("osm.all-attributes", SumoOption::bool(false));
        oc.add_synonyme("osm.all-attributes", "osm.all-tags", false);
        oc.add_description("osm.all-attributes", "Formats", "Whether additional attributes shall be imported");

        oc.do_register("osm.extra-attributes", SumoOption::string_vector_with(vec!["all".to_string()]));
        oc.add_synonyme("osm.extra-attributes", "osm.extra-tags", false);
        oc.add_description("osm.extra-attributes", "Formats", "List of additional attributes that shall be imported from OSM via osm.all-attributes (set 'all' to import all)");

        oc.do_register("osm.speedlimit-none", SumoOption::float(39.4444));
        oc.add_description("osm.speedlimit-none", "Formats", "The speed limit to be set when there is no actual speed limit in reality");

        oc.do_register("osm.annotate-defaults", SumoOption::bool(false));
        oc.add_description("osm.annotate-defaults", "Formats", "Whether edges shoulds carry information on the usage of typemap defaults");

        oc.do_register("osm.etcs-moving-block", SumoOption::bool(false));
        oc.add_description("osm.etcs-moving-block", "Formats", "Whether ETCS2 track sections should default to moving-block mode");

        // register matsim options
        oc.do_register("matsim.keep-length", SumoOption::bool(false));
        oc.add_description("matsim.keep-length", "Formats", "The edge lengths given in the MATSim-file will be kept");

        oc.do_register("matsim.lanes-from-capacity", SumoOption::bool(false));
        oc.add_description("matsim.lanes-from-capacity", "Formats", "The lane number will be computed from the capacity");

        // register shapefile options
        oc.do_register("shapefile.street-id", SumoOption::string());
        oc.add_synonyme("shapefile.street-id", "arcview.street-id", true);
        oc.add_description("shapefile.street-id", "Formats", "Read edge ids from column STR");

        oc.do_register("shapefile.from-id", SumoOption::string());
        oc.add_synonyme("shapefile.from-id", "arcview.from-id", true);
        oc.add_description("shapefile.from-id", "Formats", "Read from-node ids from column STR");

        oc.do_register("shapefile.to-id", SumoOption::string());
        oc.add_synonyme("shapefile.to-id", "arcview.to-id", true);
        oc.add_description("shapefile.to-id", "Formats", "Read to-node ids from column STR");

        oc.do_register("shapefile.type-id", SumoOption::string());
        oc.add_synonyme("shapefile.type-id", "arcview.type-id", true);
        oc.add_description("shapefile.type-id", "Formats", "Read type ids from column STR");

        oc.do_register("shapefile.laneNumber", SumoOption::string());
        oc.add_description("shapefile.laneNumber", "Formats", "Read lane number from column STR");

        oc.do_register("shapefile.speed", SumoOption::string());
        oc.add_description("shapefile.speed", "Formats", "Read speed from column STR");

        oc.do_register("shapefile.length", SumoOption::string());
        oc.add_description("shapefile.length", "Formats", "Read custom edge length from column STR");

        oc.do_register("shapefile.width", SumoOption::string());
        oc.add_description("shapefile.width", "Formats", "Read total edge width from column STR");

        oc.do_register("shapefile.name", SumoOption::string());
        oc.add_description("shapefile.name", "Formats", "Read (non-unique) name from column STR");

        oc.do_register("shapefile.node-join-dist", SumoOption::float(0.0));
        oc.add_description("shapefile.node-join-dist", "Formats", "Distance threshold for determining whether distinct shapes are connected (used when from-id and to-id are not available)");

        oc.do_register("shapefile.add-params", SumoOption::string_vector());
        oc.add_description("shapefile.add-params", "Formats", "Add the list of field names as edge params");

        oc.do_register("shapefile.use-defaults-on-failure", SumoOption::bool(false));
        oc.add_synonyme("shapefile.use-defaults-on-failure", "arcview.use-defaults-on-failure", true);
        oc.add_description("shapefile.use-defaults-on-failure", "Formats", "Uses edge type defaults on problems");

        oc.do_register("shapefile.all-bidirectional", SumoOption::bool(false));
        oc.add_synonyme("shapefile.all-bidirectional", "shapefile.all-bidi", false);
        oc.add_synonyme("shapefile.all-bidirectional", "arcview.all-bidi", true);
        oc.add_description("shapefile.all-bidirectional", "Formats", "Insert edges in both directions");

        oc.do_register("shapefile.guess-projection", SumoOption::bool(false));
        oc.add_synonyme("shapefile.guess-projection", "arcview.guess-projection", true);
        oc.add_description("shapefile.guess-projection", "Formats", "Guess the proper projection");

        oc.do_register("shapefile.traditional-axis-mapping", SumoOption::bool(false));
        oc.add_description("shapefile.traditional-axis-mapping", "Formats", "Use traditional axis order (lon, lat)");

        // register dlr-navteq options
        oc.do_register("dlr-navteq.tolerant-permissions", SumoOption::bool(false));
        oc.add_description("dlr-navteq.tolerant-permissions", "Formats", "Allow more vehicle classes by default");

        oc.do_register("dlr-navteq.keep-length", SumoOption::bool(false));
        oc.add_description("dlr-navteq.keep-length", "Formats", "The edge lengths given in the DLR Navteq-file will be kept");

        // register vissim options
        oc.do_register("vissim.join-distance", SumoOption::float(5.0));
        oc.add_synonyme("vissim.join-distance", "vissim.offset", true);
        oc.add_description("vissim.join-distance", "Formats", "Structure join offset");

        oc.do_register("vissim.default-speed", SumoOption::float(50.0));
        oc.add_description("vissim.default-speed", "Formats", "Use FLOAT as default speed");

        oc.do_register("vissim.speed-norm", SumoOption::float(1.0));
        oc.add_description("vissim.speed-norm", "Formats", "Factor for edge velocity");

        oc.do_register("vissim.report-unset-speeds", SumoOption::bool(false));
        oc.add_description("vissim.report-unset-speeds", "Formats", "Writes lanes without an explicit speed set");

        // register visum options
        oc.do_register("visum.language-file", SumoOption::file_name());
        oc.add_description("visum.language-file", "Formats", "Load language mappings from FILE");

        oc.do_register("visum.use-type-priority", SumoOption::bool(false));
        oc.add_description("visum.use-type-priority", "Formats", "Uses priorities from types");

        oc.do_register("visum.use-type-laneno", SumoOption::bool(false));
        oc.add_description("visum.use-type-laneno", "Formats", "Uses lane numbers from types");

        oc.do_register("visum.use-type-speed", SumoOption::bool(false));
        oc.add_description("visum.use-type-speed", "Formats", "Uses speeds from types");

        oc.do_register("visum.connector-speeds", SumoOption::float(100.0));
        oc.add_description("visum.connector-speeds", "Formats", "Sets connector speed");

        oc.do_register("visum.connectors-lane-number", SumoOption::integer(3));
        oc.add_synonyme("visum.connectors-lane-number", "visum.connector-laneno", true);
        oc.add_description("visum.connectors-lane-number", "Formats", "Sets connector lane number");

        oc.do_register("visum.no-connectors", SumoOption::bool(true));
        oc.add_description("visum.no-connectors", "Formats", "Excludes connectors");

        oc.do_register("visum.recompute-lane-number", SumoOption::bool(false));
        oc.add_synonyme("visum.recompute-lane-number", "visum.recompute-laneno", true);
        oc.add_description("visum.recompute-lane-number", "Formats", "Computes the number of lanes from the edges' capacities");

        oc.do_register("visum.verbose-warnings", SumoOption::bool(false));
        oc.add_description("visum.verbose-warnings", "Formats", "Prints all warnings, some of which are due to VISUM misbehaviour");

        oc.do_register("visum.lanes-from-capacity.norm", SumoOption::float(1800.0));
        oc.add_synonyme("visum.lanes-from-capacity.norm", "capacity-norm", true);
        oc.add_synonyme("visum.lanes-from-capacity.norm", "lanes-from-capacity.norm", false);
        oc.add_description("visum.lanes-from-capacity.norm", "Formats", "The factor for flow to no. lanes conversion");

        // register opendrive options
        oc.do_register("opendrive.import-all-lanes", SumoOption::bool(false));
        oc.add_description("opendrive.import-all-lanes", "Formats", "Imports all lane types");
        oc.do_register("opendrive.curve-resolution", SumoOption::float(2.0));
        oc.add_description("opendrive.curve-resolution", "Formats", "The geometry resolution in m when importing curved geometries as line segments.");
        oc.do_register("opendrive.advance-stopline", SumoOption::float(0.0));
        oc.add_description("opendrive.advance-stopline", "Formats", "Allow stop lines to be built beyond the start of the junction if the geometries allow so");
        oc.do_register("opendrive.min-width", SumoOption::float(1.8));
        oc.add_description("opendrive.min-width", "Formats", "The minimum lane width for determining start or end of variable-width lanes");
        oc.do_register("opendrive.internal-shapes", SumoOption::bool(false));
        oc.add_description("opendrive.internal-shapes", "Formats", "Import internal lane shapes");
        oc.do_register("opendrive.position-ids", SumoOption::bool(false));
        oc.add_description("opendrive.position-ids", "Formats", "Sets edge-id based on road-id and offset in m (legacy)");
        oc.do_register("opendrive.lane-shapes", SumoOption::bool(false));
        oc.add_description("opendrive.lane-shapes", "Formats", "Use custom lane shapes to compensate discarded lane types");
        oc.do_register("opendrive.signal-groups", SumoOption::bool(false));
        oc.add_description("opendrive.signal-groups", "Formats", "Use the OpenDRIVE controller information for the generated signal program");
        oc.do_register("opendrive.ignore-misplaced-signals", SumoOption::bool(false));
        oc.add_description("opendrive.ignore-misplaced-signals", "Formats", "Ignore traffic signals which do not control any driving lane");

        // register some additional options
        oc.do_register("tls.discard-loaded", SumoOption::bool(false));
        oc.add_description("tls.discard-loaded", "TLS Building", "Does not instantiate traffic lights loaded from other formats than plain-XML");

        oc.do_register("tls.discard-simple", SumoOption::bool(false));
        oc.add_description("tls.discard-simple", "TLS Building", "Does not instantiate traffic lights at geometry-like nodes loaded from other formats than plain-XML");

        // register railway options
        oc.do_register("railway.signals.discard", SumoOption::bool(false));
        oc.add_description("railway.signals.discard", "Railway", "Discard all railway signal information loaded from other formats than plain-xml");
    }

    /// C++: `bool NIFrame::checkOptions(OptionsCont &)` [static]
    /// Checks set options for being valid
    // bool
    // NIFrame::checkOptions(OptionsCont& oc) {
    //     bool ok = oc.checkDependingSuboptions("shapefile", "shapefile.");
    //     ok &= oc.checkDependingSuboptions("visum-file", "visum.");
    //     ok &= oc.checkDependingSuboptions("vissim-file", "vissim.");
    // #ifdef PROJ_API_FILE
    //     int numProjections = oc.getBool("simple-projection") + oc.getBool("proj.utm") + oc.getBool("proj.dhdn") + (oc.getString("proj").length() > 1);
    //     if ((oc.isSet("osm-files") || oc.isSet("dlr-navteq-prefix") || oc.isSet("shapefile-prefix")) && numProjections == 0) {
    //         if (oc.isDefault("proj")) {
    //             oc.set("proj.utm", "true");
    //         }
    //     }
    //     if (oc.isSet("dlr-navteq-prefix") && oc.isDefault("proj.scale")) {
    //         oc.setDefault("proj.scale", NIImporter_DlrNavteq::GEO_SCALE);
    //     }
    // #else
    //     if ((oc.isSet("osm-files") || oc.isSet("dlr-navteq-prefix") || oc.isSet("shapefile-prefix")) && !oc.getBool("simple-projection")) {
    //         WRITE_ERROR(TL("Cannot import network data without PROJ-Library. Please install package proj before building sumo"));
    //         ok = false;
    //     }
    // #endif
    //     if (oc.isSet("sumo-net-file")) {
    //         if (oc.isWriteable("no-turnarounds")) {
    //             // changed default since turnarounds are loaded from the net file.
    //             oc.set("no-turnarounds", "true");
    //         }
    //         if (oc.isWriteable("offset.disable-normalization")) {
    //             // changed default since we wish to preserve the network as far as possible
    //             oc.set("offset.disable-normalization", "true");
    //         }
    //         if (oc.isWriteable("geometry.max-grade.fix")) {
    //             // changed default since we wish to preserve the network as far as possible
    //             oc.set("geometry.max-grade.fix", "false");
    //         }
    //         if (oc.isWriteable("geometry.min-radius.fix.railways")) {
    //             // changed default since we wish to preserve the network as far as possible
    //             oc.set("geometry.min-radius.fix.railways", "false");
    //         }
    //         if (oc.isWriteable("geometry.avoid-overlap")) {
    //             // changed default since we wish to preserve the network as far as possible
    //             oc.set("geometry.avoid-overlap", "false");
    //         }
    //     }
    //     if (!oc.isSet("type-files")) {
    //         const char* sumoPath = std::getenv("SUMO_HOME");
    //         if (sumoPath == nullptr) {
    //             WRITE_WARNING(TL("Environment variable SUMO_HOME is not set, using built in type maps."));
    //         } else {
    //             const std::string path = sumoPath + std::string("/data/typemap/");
    //             if (oc.isSet("osm-files")) {
    //                 oc.setDefault("type-files", path + "osmNetconvert.typ.xml");
    //             }
    //             if (oc.isSet("opendrive-files")) {
    //                 oc.setDefault("type-files", path + "opendriveNetconvert.typ.xml");
    //             }
    //         }
    //     }
    //     if (oc.isSet("opendrive-files")) {
    //         if (oc.isDefault("tls.left-green.time")) {
    //             // legacy behavior. see #2114
    //             oc.setDefault("tls.left-green.time", "0");
    //         }
    //         if (oc.isDefault("rectangular-lane-cut")) {
    //             // a better interpretation of imported geometries
    //             oc.setDefault("rectangular-lane-cut", "true");
    //         }
    //         if (oc.isDefault("geometry.max-grade.fix")) {
    //             // a better interpretation of imported geometries
    //             oc.setDefault("geometry.max-grade.fix", "false");
    //         }
    //         if (oc.isDefault("no-turnarounds") && oc.isDefault("no-turnarounds.except-deadend")) {
    //             // changed default since all connections (connecting roads) are loaded from the input.
    //             oc.set("no-turnarounds.except-deadend", "true");
    //         }
    //     }
    //     if (!oc.isDefault("osm.extra-attributes") && oc.isDefault("osm.all-attributes")) {
    //         oc.setDefault("osm.all-attributes", "true");
    //     }
    //     if (oc.getBool("osm.crossings") && !oc.getBool("osm.sidewalks")) {
    //         WRITE_WARNING(TL("It is recommended to use option osm.crossings with osm.sidewalks"));
    //     }
    //     if (oc.isSet("shapefile-prefix") && !oc.isDefault("shapefile.name")) {
    //         oc.setDefault("output.street-names", "true");
    //     }
    //
    //     return ok;
    // }
    #[allow(unused_variables)]
    pub fn check_options(oc: &mut OptionsCont) -> bool {
        // shortcut: no PROJ binding in this port (C++ PROJ_API_FILE branch is not built), so the #else branch of checkOptions is used
        let mut ok = must(oc.check_depending_suboptions("shapefile", "shapefile."));
        ok &= must(oc.check_depending_suboptions("visum-file", "visum."));
        ok &= must(oc.check_depending_suboptions("vissim-file", "vissim."));
        if (must(oc.is_set("osm-files", true)) || must(oc.is_set("dlr-navteq-prefix", true)) || must(oc.is_set("shapefile-prefix", true))) && !must(oc.get_bool("simple-projection")) {
            write_error(&tlf!("Cannot import network data without PROJ-Library. Please install package proj before building sumo"));
            ok = false;
        }
        if must(oc.is_set("sumo-net-file", true)) {
            if must(oc.is_writeable("no-turnarounds")) {
                // changed default since turnarounds are loaded from the net file.
                must(oc.set("no-turnarounds", "true", false));
            }
            if must(oc.is_writeable("offset.disable-normalization")) {
                // changed default since we wish to preserve the network as far as possible
                must(oc.set("offset.disable-normalization", "true", false));
            }
            if must(oc.is_writeable("geometry.max-grade.fix")) {
                // changed default since we wish to preserve the network as far as possible
                must(oc.set("geometry.max-grade.fix", "false", false));
            }
            if must(oc.is_writeable("geometry.min-radius.fix.railways")) {
                // changed default since we wish to preserve the network as far as possible
                must(oc.set("geometry.min-radius.fix.railways", "false", false));
            }
            if must(oc.is_writeable("geometry.avoid-overlap")) {
                // changed default since we wish to preserve the network as far as possible
                must(oc.set("geometry.avoid-overlap", "false", false));
            }
        }
        if !must(oc.is_set("type-files", true)) {
            match std::env::var("SUMO_HOME") {
                Err(_) => write_warning(&tlf!("Environment variable SUMO_HOME is not set, using built in type maps.")),
                Ok(sumo_path) => {
                    let path = sumo_path + "/data/typemap/";
                    if must(oc.is_set("osm-files", true)) {
                        must(oc.set_default("type-files", &(path.clone() + "osmNetconvert.typ.xml")));
                    }
                    if must(oc.is_set("opendrive-files", true)) {
                        must(oc.set_default("type-files", &(path + "opendriveNetconvert.typ.xml")));
                    }
                }
            }
        }
        if must(oc.is_set("opendrive-files", true)) {
            if oc.is_default("tls.left-green.time") {
                // legacy behavior. see #2114
                must(oc.set_default("tls.left-green.time", "0"));
            }
            if oc.is_default("rectangular-lane-cut") {
                // a better interpretation of imported geometries
                must(oc.set_default("rectangular-lane-cut", "true"));
            }
            if oc.is_default("geometry.max-grade.fix") {
                // a better interpretation of imported geometries
                must(oc.set_default("geometry.max-grade.fix", "false"));
            }
            if oc.is_default("no-turnarounds") && oc.is_default("no-turnarounds.except-deadend") {
                // changed default since all connections (connecting roads) are loaded from the input.
                must(oc.set("no-turnarounds.except-deadend", "true", false));
            }
        }
        if !oc.is_default("osm.extra-attributes") && oc.is_default("osm.all-attributes") {
            must(oc.set_default("osm.all-attributes", "true"));
        }
        if must(oc.get_bool("osm.crossings")) && !must(oc.get_bool("osm.sidewalks")) {
            write_warning(&tlf!("It is recommended to use option osm.crossings with osm.sidewalks"));
        }
        if must(oc.is_set("shapefile-prefix", true)) && !oc.is_default("shapefile.name") {
            must(oc.set_default("output.street-names", "true"));
        }

        ok
    }

}

