//! Port of netbuild/NBFrame.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::msg_handler::{write_error, write_warning};
use utils::common::std_defs::g_precision;
use utils::common::sumo_vehicle_class::parse_vehicle_classes_vec;
use utils::common::to_string::{f64_to_string_prec, join_to_string};
use utils::common::util_exceptions::ProcessError;
use utils::geom::POSITION_EPS;
use utils::xml::sumo_xml_definitions::{LaneSpreadFunctions, NodeTypes, RightOfWayValues, TrafficLightTypes};
use utils::{write_errorf};
use crate::netbuild::nb_edge::{UNSPECIFIED_WIDTH as NBEDGE_UNSPECIFIED_WIDTH, UNSPECIFIED_FRICTION as NBEDGE_UNSPECIFIED_FRICTION, UNSPECIFIED_CONTPOS as NBEDGE_UNSPECIFIED_CONTPOS, UNSPECIFIED_VISIBILITY_DISTANCE as NBEDGE_UNSPECIFIED_VISIBILITY_DISTANCE, UNSPECIFIED_LOADED_LENGTH as NBEDGE_UNSPECIFIED_LOADED_LENGTH};

/// Sets and checks options for netbuild
/// C++ class `NBFrame` (netbuild/NBFrame.h)
pub struct NBFrame {
}

impl NBFrame {
    /// C++: `void NBFrame::fillOptions(OptionsCont &, bool)` [static]
    /// Inserts options used by the network converter
    // void
    // NBFrame::fillOptions(OptionsCont& oc, bool forNetgen) {
    //     // register building defaults
    //     oc.doRegister("default.lanenumber", 'L', new Option_Integer(1));
    //     oc.addSynonyme("default.lanenumber", "lanenumber", true);
    //     oc.addDescription("default.lanenumber", "Building Defaults", TL("The default number of lanes in an edge"));
    //
    //     oc.doRegister("default.lanewidth", new Option_Float(NBEDGE_UNSPECIFIED_WIDTH));
    //     oc.addSynonyme("default.lanewidth", "lanewidth", true);
    //     oc.addDescription("default.lanewidth", "Building Defaults", TL("The default width of lanes"));
    //
    //     oc.doRegister("default.spreadtype", new Option_String("right"));
    //     oc.addDescription("default.spreadtype", "Building Defaults", TL("The default method for computing lane shapes from edge shapes"));
    //
    //     oc.doRegister("default.speed", 'S', new Option_Float((double) 13.89));
    //     oc.addSynonyme("default.speed", "speed", true);
    //     oc.addDescription("default.speed", "Building Defaults", TL("The default speed on an edge (in m/s)"));
    //
    //     oc.doRegister("default.friction", new Option_Float(NBEDGE_UNSPECIFIED_FRICTION));
    //     oc.addSynonyme("default.friction", "friction", true);
    //     oc.addDescription("default.friction", "Building Defaults", TL("The default friction on an edge"));
    //
    //     oc.doRegister("default.priority", 'P', new Option_Integer(-1));
    //     oc.addSynonyme("default.priority", "priority", true);
    //     oc.addDescription("default.priority", "Building Defaults", TL("The default priority of an edge"));
    //
    //     oc.doRegister("default.type", new Option_String());
    //     oc.addDescription("default.type", "Building Defaults", TL("The default edge type"));
    //
    //     oc.doRegister("default.sidewalk-width", new Option_Float((double) 2.0));
    //     oc.addDescription("default.sidewalk-width", "Building Defaults", TL("The default width of added sidewalks"));
    //
    //     oc.doRegister("default.bikelane-width", new Option_Float((double) 1.0));
    //     oc.addDescription("default.bikelane-width", "Building Defaults", TL("The default width of added bike lanes"));
    //
    //     oc.doRegister("default.crossing-width", new Option_Float((double) 4.0));
    //     oc.addDescription("default.crossing-width", "Building Defaults", TL("The default width of a pedestrian crossing"));
    //
    //     oc.doRegister("default.crossing-speed", new Option_Float(2.78));
    //     oc.addDescription("default.crossing-speed", "Building Defaults", TL("The default speed 'limit' on a pedestrian crossing (in m/s)"));
    //
    //     oc.doRegister("default.walkingarea-speed", new Option_Float(2.78));
    //     oc.addDescription("default.walkingarea-speed", "Building Defaults", TL("The default speed 'limit' on a pedestrian walkingarea (in m/s)"));
    //
    //     oc.doRegister("default.allow", new Option_String());
    //     oc.addDescription("default.allow", "Building Defaults", TL("The default for allowed vehicle classes"));
    //
    //     oc.doRegister("default.disallow", new Option_String());
    //     oc.addDescription("default.disallow", "Building Defaults", TL("The default for disallowed vehicle classes"));
    //
    //     oc.doRegister("default.junctions.keep-clear", new Option_Bool(true));
    //     oc.addDescription("default.junctions.keep-clear", "Building Defaults", TL("Whether junctions should be kept clear by default"));
    //
    //     oc.doRegister("default.junctions.radius", new Option_Float(4));
    //     oc.addDescription("default.junctions.radius", "Building Defaults", TL("The default turning radius of intersections"));
    //
    //     oc.doRegister("default.junctions.type", 'j', new Option_String());
    //     oc.addSynonyme("default.junctions.type", "default-junction-type"); // default netgenerate option name
    //     oc.addDescription("default.junctions.type", "Building Defaults", TL("[traffic_light|priority|right_before_left|left_before_right|traffic_light_right_on_red|priority_stop|allway_stop|...] Determines default junction type (see docs/Networks/PlainXML#node_types)"));
    //
    //     oc.doRegister("default.connection-length", new Option_Float((double) NBEDGE_UNSPECIFIED_LOADED_LENGTH));
    //     oc.addDescription("default.connection-length", "Building Defaults", TL("The default length when overriding connection lengths"));
    //
    //     oc.doRegister("default.connection.cont-pos", new Option_Float((double)NBEDGE_UNSPECIFIED_CONTPOS));
    //     oc.addDescription("default.connection.cont-pos", "Building Defaults", TL("Whether/where connections should have an internal junction"));
    //
    //     oc.doRegister("default.right-of-way", new Option_String("default"));
    //     oc.addDescription("default.right-of-way", "Building Defaults", TL("The default algorithm for computing right of way rules ('default', 'edgePriority')"));
    //
    //     oc.doRegister("junctions.right-before-left.speed-threshold", new Option_Float(49 / 3.6));
    //     oc.addDescription("junctions.right-before-left.speed-threshold", "Junctions", TL("Allow building right-before-left junctions when the incoming edge speeds are below FLOAT (m/s)"));
    //
    //     oc.doRegister("junctions.left-before-right", new Option_Bool(false));
    //     oc.addDescription("junctions.left-before-right", "Junctions", TL("Build left-before-right junctions instead of right-before-left junctions"));
    //
    //     // register the data processing options
    //     oc.doRegister("no-internal-links", new Option_Bool(false)); // !!! not described
    //     oc.addDescription("no-internal-links", "Junctions", TL("Omits internal links"));
    //
    //     oc.doRegister("numerical-ids", new Option_Bool(false));
    //     oc.addDescription("numerical-ids", "Processing", TL("Remaps alphanumerical IDs of nodes and edges to ensure that all IDs are integers"));
    //
    //     oc.doRegister("numerical-ids.node-start", new Option_Integer(std::numeric_limits<int>::max()));
    //     oc.addDescription("numerical-ids.node-start", "Processing", TL("Remaps IDs of nodes to integers starting at INT"));
    //
    //     oc.doRegister("numerical-ids.edge-start", new Option_Integer(std::numeric_limits<int>::max()));
    //     oc.addDescription("numerical-ids.edge-start", "Processing", TL("Remaps IDs of edges to integers starting at INT"));
    //
    //     /// @todo not working for netgen
    //     oc.doRegister("reserved-ids", new Option_FileName());
    //     oc.addDescription("reserved-ids", "Processing", TL("Ensures that generated ids do not included any of the typed IDs from FILE (sumo-gui selection file format)"));
    //
    //     oc.doRegister("kept-ids", new Option_FileName());
    //     oc.addDescription("kept-ids", "Processing", TL("Ensures that objects with typed IDs from FILE (sumo-gui selection file format) are not renamed"));
    //
    //     if (!forNetgen) {
    //         oc.doRegister("dismiss-vclasses", new Option_Bool(false));
    //         oc.addDescription("dismiss-vclasses", "Processing", TL("Removes vehicle class restrictions from imported edges"));
    //     }
    //
    //     oc.doRegister("no-turnarounds", new Option_Bool(false));
    //     oc.addDescription("no-turnarounds", "Junctions", TL("Disables building turnarounds"));
    //
    //     oc.doRegister("no-turnarounds.tls", new Option_Bool(false));
    //     oc.addSynonyme("no-turnarounds.tls", "no-tls-turnarounds", true);
    //     oc.addDescription("no-turnarounds.tls", "Junctions", TL("Disables building turnarounds at tls-controlled junctions"));
    //
    //     oc.doRegister("no-turnarounds.geometry", new Option_Bool(true));
    //     oc.addDescription("no-turnarounds.geometry", "Junctions", TL("Disables building turnarounds at geometry-like junctions"));
    //
    //     oc.doRegister("no-turnarounds.except-deadend", new Option_Bool(false));
    //     oc.addDescription("no-turnarounds.except-deadend", "Junctions", TL("Disables building turnarounds except at dead end junctions"));
    //
    //     oc.doRegister("no-turnarounds.except-turnlane", new Option_Bool(false));
    //     oc.addDescription("no-turnarounds.except-turnlane", "Junctions", TL("Disables building turnarounds except at junctions with a dedicated turning lane"));
    //
    //     oc.doRegister("no-turnarounds.fringe", new Option_Bool(false));
    //     oc.addDescription("no-turnarounds.fringe", "Junctions", TL("Disables building turnarounds at fringe junctions"));
    //
    //     oc.doRegister("no-left-connections", new Option_Bool(false));
    //     oc.addDescription("no-left-connections", "Junctions", TL("Disables building connections to left"));
    //
    //     oc.doRegister("geometry.split", new Option_Bool(false)); // !!!not described
    //     oc.addSynonyme("geometry.split", "split-geometry", true);
    //     oc.addDescription("geometry.split", "Processing", TL("Splits edges across geometry nodes"));
    //
    //     oc.doRegister("geometry.remove", 'R', new Option_Bool(false));
    //     oc.addSynonyme("geometry.remove", "remove-geometry", true);
    //     oc.addDescription("geometry.remove", "Processing", TL("Replace nodes which only define edge geometry by geometry points (joins edges)"));
    //
    //     oc.doRegister("geometry.remove.keep-edges.explicit", new Option_StringVector());
    //     oc.addDescription("geometry.remove.keep-edges.explicit", "Processing", TL("Ensure that the given list of edges is not modified"));
    //
    //     oc.doRegister("geometry.remove.keep-edges.input-file", new Option_FileName());
    //     oc.addDescription("geometry.remove.keep-edges.input-file", "Processing",
    //                       "Ensure that the edges in FILE are not modified (Each id on a single line. Selection files from sumo-gui are also supported)");
    //
    //     if (!forNetgen) {
    //         oc.doRegister("geometry.remove.keep-ptstops", new Option_Bool(false));
    //         oc.addDescription("geometry.remove.keep-ptstops", "Processing", TL("Ensure that edges with public transport stops are not modified"));
    //     }
    //
    //     oc.doRegister("geometry.remove.min-length", new Option_Float(0));
    //     oc.addDescription("geometry.remove.min-length", "Processing",
    //                       "Allow merging edges with differing attributes when their length is below min-length");
    //
    //     oc.doRegister("geometry.remove.width-tolerance", new Option_Float(0));
    //     oc.addDescription("geometry.remove.width-tolerance", "Processing",
    //                       "Allow merging edges with differing lane widths if the difference is below FLOAT");
    //
    //     oc.doRegister("geometry.remove.max-junction-size", new Option_Float(-1));
    //     oc.addDescription("geometry.remove.max-junction-size", "Processing",
    //                       "Prevent removal of junctions with a size above FLOAT as defined by custom edge endpoints");
    //
    //     oc.doRegister("geometry.max-segment-length", new Option_Float(0));
    //     oc.addDescription("geometry.max-segment-length", "Processing", TL("splits geometry to restrict segment length"));
    //
    //     if (!forNetgen) {
    //         oc.doRegister("geometry.min-dist", new Option_Float(-1));
    //         oc.addDescription("geometry.min-dist", "Processing", TL("reduces too similar geometry points"));
    //
    //         oc.doRegister("geometry.max-angle", new Option_Float(99));
    //         oc.addDescription("geometry.max-angle", "Processing", TL("Warn about edge geometries with an angle above DEGREES in successive segments"));
    //
    //         oc.doRegister("geometry.max-angle.fix", new Option_Bool(false));
    //         oc.addDescription("geometry.max-angle.fix", "Processing", TL("Straighten edge geometries with an angle above max-angle successive segments"));
    //
    //         oc.doRegister("geometry.min-radius", new Option_Float(9));
    //         oc.addDescription("geometry.min-radius", "Processing", TL("Warn about edge geometries with a turning radius less than METERS at the start or end"));
    //
    //         oc.doRegister("geometry.min-radius.fix", new Option_Bool(false));
    //         oc.addDescription("geometry.min-radius.fix", "Processing", TL("Straighten edge geometries to avoid turning radii less than geometry.min-radius"));
    //
    //         oc.doRegister("geometry.min-radius.fix.railways", new Option_Bool(true));
    //         oc.addDescription("geometry.min-radius.fix.railways", "Processing", TL("Straighten edge geometries to avoid turning radii less than geometry.min-radius (only railways)"));
    //
    //         oc.doRegister("geometry.junction-mismatch-threshold", new Option_Float(20));
    //         oc.addDescription("geometry.junction-mismatch-threshold", "Processing", TL("Warn if the junction shape is too far away from the original node position"));
    //
    //         oc.doRegister("geometry.check-overlap", new Option_Float(0));
    //         oc.addDescription("geometry.check-overlap", "Processing", TL("Warn if edges overlap by more than the given threshold value"));
    //
    //         oc.doRegister("geometry.check-overlap.vertical-threshold", new Option_Float(4));
    //         oc.addDescription("geometry.check-overlap.vertical-threshold", "Processing", TL("Ignore overlapping edges if they are separated vertically by the given threshold."));
    //
    //         oc.doRegister("geometry.avoid-overlap", new Option_Bool(true));
    //         oc.addDescription("geometry.avoid-overlap", "Processing", TL("Modify edge geometries to avoid overlap at junctions"));
    //
    //         oc.doRegister("join-lanes", new Option_Bool(false));
    //         oc.addDescription("join-lanes", "Processing", TL("join adjacent lanes that have the same permissions and which do not admit lane-changing (sidewalks and disallowed lanes)"));
    //
    //         oc.doRegister("ptline.match-dist", new Option_Float(100));
    //         oc.addDescription("ptline.match-dist", "Processing", TL("Matches stops outside the road network to the referencing pt line when below the given distance"));
    //         // railway processing options
    //
    //         oc.doRegister("railway.topology.repair", new Option_Bool(false));
    //         oc.addDescription("railway.topology.repair", "Railway", TL("Repair topology of the railway network"));
    //
    //         oc.doRegister("railway.topology.repair.minimal", new Option_Bool(false));
    //         oc.addDescription("railway.topology.repair.minimal", "Railway", TL("Repair topology of the railway network just enough to let loaded public transport lines to work"));
    //
    //         oc.doRegister("railway.topology.repair.connect-straight", new Option_Bool(false));
    //         oc.addDescription("railway.topology.repair.connect-straight", "Railway", TL("Allow bidirectional rail use wherever rails with opposite directions meet at a straight angle"));
    //
    //         oc.doRegister("railway.topology.repair.stop-turn", new Option_Bool(false));
    //         oc.addDescription("railway.topology.repair.stop-turn", "Railway", TL("Add turn-around connections at all loaded stops."));
    //
    //         oc.doRegister("railway.topology.repair.bidi-penalty", new Option_Float(1.2));
    //         oc.addDescription("railway.topology.repair.bidi-penalty", "Railway", TL("Penalty factor for adding new bidi edges to connect public transport stops"));
    //
    //         oc.doRegister("railway.topology.all-bidi", new Option_Bool(false));
    //         oc.addDescription("railway.topology.all-bidi", "Railway", TL("Make all rails usable in both direction"));
    //
    //         oc.doRegister("railway.topology.all-bidi.input-file", new Option_FileName());
    //         oc.addDescription("railway.topology.all-bidi.input-file", "Railway", TL("Make all rails edge ids from FILE usable in both direction"));
    //
    //         oc.doRegister("railway.topology.direction-priority", new Option_Bool(false));
    //         oc.addDescription("railway.topology.direction-priority", "Railway", TL("Set edge priority values based on estimated main direction"));
    //
    //         oc.doRegister("railway.topology.ptline-priority", new Option_StringVector());
    //         oc.addDescription("railway.topology.ptline-priority", "Railway", TL("Set edge priority values based on public transport lines for the given list of vClasses"));
    //
    //         oc.doRegister("railway.topology.extend-priority", new Option_Bool(false));
    //         oc.addDescription("railway.topology.extend-priority", "Railway", TL("Extend loaded edge priority values based on estimated main direction"));
    //
    //         oc.doRegister("railway.geometry.straighten", new Option_Bool(false));
    //         oc.addDescription("railway.geometry.straighten", "Railway", TL("Move junctions to straighten a sequence of rail edges"));
    //
    //         oc.doRegister("railway.signal.guess.by-stops", new Option_Bool(false));
    //         oc.addDescription("railway.signal.guess.by-stops", "Railway", TL("Guess signals that guard public transport stops"));
    //         oc.doRegister("railway.signal.guess.by-stops.split", new Option_Bool(false));
    //         oc.addDescription("railway.signal.guess.by-stops.split", "Railway", TL("Split edges to improve positioning of guessed signals and prevent switches as rail_signals"));
    //
    //         oc.doRegister("railway.signal.permit-unsignalized", new Option_StringVector({"tram", "cable_car"}));
    //         oc.addDescription("railway.signal.permit-unsignalized", "Railway", TL("List rail classes that may run without rail signals"));
    //
    //         oc.doRegister("railway.access-distance", new Option_Float(150.0));
    //         oc.addDescription("railway.access-distance", "Railway", TL("The search radius for finding suitable road accesses for rail stops"));
    //         oc.addSynonyme("railway.access-distance", "osm.stop-output.footway-access-distance", true);
    //
    //         oc.doRegister("railway.max-accesses", new Option_Integer(5));
    //         oc.addDescription("railway.max-accesses", "Railway", TL("The maximum road accesses registered per rail stops"));
    //         oc.addSynonyme("railway.max-accesses", "osm.stop-output.footway-max-accesses", true);
    //
    //         oc.doRegister("railway.access-factor", new Option_Float(1.5));
    //         oc.addDescription("railway.access-factor", "Railway", TL("The walking length of the access is computed as air-line distance multiplied by FLOAT"));
    //         oc.addSynonyme("railway.access-factor", "osm.stop-output.footway-access-factor", true);
    //
    //         oc.doRegister("ptstop-output.no-bidi", new Option_Bool(false));
    //         oc.addDescription("ptstop-output.no-bidi", "Processing", TL("Skips automatic generation of stops on the bidi-edge of a loaded stop"));
    //     }
    //
    //     oc.doRegister("geometry.max-grade", new Option_Float(10));
    //     oc.addDescription("geometry.max-grade", "Processing", TL("Warn about edge geometries with a grade in % above FLOAT."));
    //
    //     oc.doRegister("geometry.max-grade.fix", new Option_Bool(true));
    //     oc.addDescription("geometry.max-grade.fix", "Processing", TL("Smooth edge geometries with a grade above the warning threshold."));
    //
    //     oc.doRegister("offset.disable-normalization", new Option_Bool(false));
    //     oc.addSynonyme("offset.disable-normalization", "disable-normalize-node-positions", true);
    //     oc.addDescription("offset.disable-normalization", "Processing", TL("Turn off normalizing node positions"));
    //
    //     oc.doRegister("offset.x", new Option_Float(0));
    //     oc.addSynonyme("offset.x", "x-offset-to-apply", true);
    //     oc.addDescription("offset.x", "Processing", TL("Adds FLOAT to net x-positions"));
    //
    //     oc.doRegister("offset.y", new Option_Float(0));
    //     oc.addSynonyme("offset.y", "y-offset-to-apply", true);
    //     oc.addDescription("offset.y", "Processing", TL("Adds FLOAT to net y-positions"));
    //
    //     oc.doRegister("offset.z", new Option_Float(0));
    //     oc.addDescription("offset.z", "Processing", TL("Adds FLOAT to net z-positions"));
    //
    //     oc.doRegister("flip-y-axis", new Option_Bool(false));
    //     oc.addSynonyme("flip-y-axis", "flip-y");
    //     oc.addDescription("flip-y-axis", "Processing", TL("Flips the y-coordinate along zero"));
    //
    //     oc.doRegister("roundabouts.guess", new Option_Bool(true));
    //     oc.addSynonyme("roundabouts.guess", "guess-roundabouts", true);
    //     oc.addDescription("roundabouts.guess", "Processing", TL("Enable roundabout-guessing"));
    //
    //     // The Putrajaya Roundabout (Malaysia) holds the Guinness record for the world’s largest roundabout with 3.4km.
    //     oc.doRegister("roundabouts.guess.max-length", new Option_Float(3500));
    //     oc.addDescription("roundabouts.guess.max-length", "Processing", TL("Structures with a circumference above FLOAT threshold are not classified as roundabout"));
    //
    //     oc.doRegister("roundabouts.visibility-distance", new Option_Float(9));
    //     oc.addDescription("roundabouts.visibility-distance", "Processing", TL("Default visibility when approaching a roundabout"));
    //
    //     oc.doRegister("opposites.guess", new Option_Bool(false));
    //     oc.addDescription("opposites.guess", "Processing", TL("Enable guessing of opposite direction lanes usable for overtaking"));
    //
    //     oc.doRegister("opposites.guess.fix-lengths", new Option_Bool(true));
    //     oc.addDescription("opposites.guess.fix-lengths", "Processing", TL("Ensure that opposite edges have the same length"));
    //
    //     oc.doRegister("fringe.guess", new Option_Bool(false));
    //     oc.addDescription("fringe.guess", "Processing", TL("Enable guessing of network fringe nodes"));
    //
    //     oc.doRegister("fringe.guess.speed-threshold", new Option_Float(50 / 3.6));
    //     oc.addDescription("fringe.guess.speed-threshold", "Processing", TL("Guess disconnected edges above the given speed as outer fringe"));
    //
    //     oc.doRegister("lefthand", new Option_Bool(false));
    //     oc.addDescription("lefthand", "Processing", TL("Assumes left-hand traffic on the network"));
    //
    //     oc.doRegister("edges.join", new Option_Bool(false));
    //     oc.addDescription("edges.join", "Processing",
    //                       TL("Merges edges which connect the same nodes and are close to each other (recommended for VISSIM import)"));
    //
    //     oc.doRegister("junctions.join", new Option_Bool(false));
    //     oc.addDescription("junctions.join", "Junctions",
    //                       TL("Joins junctions that are close to each other (recommended for OSM import)"));
    //
    //     oc.doRegister("junctions.join-dist", new Option_Float(10));
    //     oc.addDescription("junctions.join-dist", "Junctions",
    //                       TL("Determines the maximal distance for joining junctions (defaults to 10)"));
    //
    //     oc.doRegister("junctions.join.parallel-threshold", new Option_Float(30));
    //     oc.addDescription("junctions.join.parallel-threshold", "Junctions",
    //                       TL("The angular threshold in degrees for rejection of parallel edges when joining junctions"));
    //
    //     if (!forNetgen) {
    //         oc.doRegister("junctions.join-exclude", new Option_StringVector());
    //         oc.addDescription("junctions.join-exclude", "Junctions", TL("Interprets STR[] as list of junctions to exclude from joining"));
    //     }
    //
    //     oc.doRegister("junctions.join-same", new Option_Float(-1));
    //     oc.addDescription("junctions.join-same", "Junctions",
    //                       TL("Joins junctions that have similar coordinates even if not connected"));
    //
    //     oc.doRegister("junctions.join-reset", new Option_Bool(false));
    //     oc.addDescription("junctions.join-reset", "Junctions", TL("Reset connections for joined junctions"));
    //
    //     if (!forNetgen) {
    //         oc.doRegister("junctions.attach-removed", new Option_Float(-1));
    //         oc.addDescription("junctions.attach-removed", "Junctions",
    //                           TL("Attach junction to the closest edge within FLOAT distance that has it's id in param removedNodeIDs (for joining networks)"));
    //     }
    //
    //     oc.doRegister("max-join-ids", new Option_Integer(4));
    //     oc.addDescription("max-join-ids", "Junctions", TL("Abbreviate junction or TLS id if it joins more than INT junctions"));
    //
    //     if (!forNetgen) {
    //         oc.doRegister("speed.offset", new Option_Float(0));
    //         oc.addDescription("speed.offset", "Processing", TL("Modifies all edge speeds by adding FLOAT"));
    //
    //         oc.doRegister("speed.factor", new Option_Float(1));
    //         oc.addDescription("speed.factor", "Processing", TL("Modifies all edge speeds by multiplying by FLOAT"));
    //
    //         oc.doRegister("speed.minimum", new Option_Float(0));
    //         oc.addDescription("speed.minimum", "Processing", TL("Modifies all edge speeds to at least FLOAT"));
    //
    //         oc.doRegister("edges.join-tram-dist", new Option_Float(-1));
    //         oc.addDescription("edges.join-tram-dist", "Processing",
    //                           TL("Joins tram edges into road lanes with similar geometry (within FLOAT distance)"));
    //     }
    //
    //     oc.doRegister("junctions.corner-detail", new Option_Integer(5));
    //     oc.addDescription("junctions.corner-detail", "Junctions", TL("Generate INT intermediate points to smooth out intersection corners"));
    //
    //     oc.doRegister("junctions.internal-link-detail", new Option_Integer(5));
    //     oc.addDescription("junctions.internal-link-detail", "Junctions", TL("Generate INT intermediate points to smooth out lanes within the intersection"));
    //
    //     oc.doRegister("junctions.scurve-stretch", new Option_Float(0));
    //     oc.addDescription("junctions.scurve-stretch", "Junctions", TL("Generate longer intersections to allow for smooth s-curves when the number of lanes changes"));
    //
    //     oc.doRegister("junctions.join-turns", new Option_Bool(false));
    //     oc.addDescription("junctions.join-turns", "Junctions",
    //                       TL("Builds common edges for turning connections with common from- and to-edge. This causes discrepancies between geometrical length and assigned length due to averaging but enables lane-changing while turning"));
    //
    //     oc.doRegister("junctions.limit-turn-speed", new Option_Float(5.5));
    //     oc.addDescription("junctions.limit-turn-speed", "Junctions",
    //                       TL("Limits speed on junctions to an average lateral acceleration of at most FLOAT (m/s^2)"));
    //
    //     oc.doRegister("junctions.limit-turn-speed.min-angle", new Option_Float(15));
    //     oc.addDescription("junctions.limit-turn-speed.min-angle", "Junctions",
    //                       TL("Do not limit turn speed for angular changes below FLOAT (degrees). The value is subtracted from the geometric angle before computing the turning radius."));
    //
    //     oc.doRegister("junctions.limit-turn-speed.min-angle.railway", new Option_Float(35));
    //     oc.addDescription("junctions.limit-turn-speed.min-angle.railway", "Junctions",
    //                       TL("Do not limit turn speed for angular changes below FLOAT (degrees) on railway edges. The value is subtracted from the geometric angle before computing the turning radius."));
    //
    //     oc.doRegister("junctions.limit-turn-speed.warn.straight", new Option_Float(5));
    //     oc.addDescription("junctions.limit-turn-speed.warn.straight", "Junctions",
    //                       TL("Warn about turn speed limits that reduce the speed of straight connections by more than FLOAT"));
    //
    //     oc.doRegister("junctions.limit-turn-speed.warn.turn", new Option_Float(22));
    //     oc.addDescription("junctions.limit-turn-speed.warn.turn", "Junctions",
    //                       TL("Warn about turn speed limits that reduce the speed of turning connections (no u-turns) by more than FLOAT"));
    //
    //     oc.doRegister("junctions.small-radius", new Option_Float(1.5));
    //     oc.addDescription("junctions.small-radius", "Junctions",
    //                       TL("Default radius for junctions that do not require wide vehicle turns"));
    //
    //     oc.doRegister("junctions.higher-speed", new Option_Bool(false));
    //     oc.addDescription("junctions.higher-speed", "Junctions",
    //                       TL("Use maximum value of incoming and outgoing edge speed on junction instead of average"));
    //
    //     oc.doRegister("junctions.minimal-shape", new Option_Bool(false));
    //     oc.addDescription("junctions.minimal-shape", "Junctions",
    //                       TL("Build junctions with minimal shapes (ignoring edge overlap)"));
    //
    //     oc.doRegister("junctions.endpoint-shape", new Option_Bool(false));
    //     oc.addDescription("junctions.endpoint-shape", "Junctions",
    //                       TL("Build junction shapes based on edge endpoints (ignoring edge overlap)"));
    //
    //     oc.doRegister("internal-junctions.vehicle-width", new Option_Float(1.8));
    //     oc.addDescription("internal-junctions.vehicle-width", "Junctions",
    //                       TL("Assumed vehicle width for computing internal junction positions"));
    //
    //     oc.doRegister("rectangular-lane-cut", new Option_Bool(false));
    //     oc.addDescription("rectangular-lane-cut", "Junctions", TL("Forces rectangular cuts between lanes and intersections"));
    //
    //     oc.doRegister("check-lane-foes.roundabout", new Option_Bool(true));
    //     oc.addDescription("check-lane-foes.roundabout", "Junctions",
    //                       TL("Allow driving onto a multi-lane road if there are foes on other lanes (at roundabouts)"));
    //
    //     oc.doRegister("check-lane-foes.all", new Option_Bool(false));
    //     oc.addDescription("check-lane-foes.all", "Junctions",
    //                       TL("Allow driving onto a multi-lane road if there are foes on other lanes (everywhere)"));
    //
    //     oc.doRegister("sidewalks.guess", new Option_Bool(false));
    //     oc.addDescription("sidewalks.guess", "Pedestrian",
    //                       TL("Guess pedestrian sidewalks based on edge speed"));
    //
    //     oc.doRegister("sidewalks.guess.max-speed", new Option_Float((double) 13.89));
    //     oc.addDescription("sidewalks.guess.max-speed", "Pedestrian",
    //                       TL("Add sidewalks for edges with a speed equal or below the given limit"));
    //
    //     oc.doRegister("sidewalks.guess.min-speed", new Option_Float((double) 5.8));
    //     oc.addDescription("sidewalks.guess.min-speed", "Pedestrian",
    //                       TL("Add sidewalks for edges with a speed above the given limit"));
    //
    //     oc.doRegister("sidewalks.guess.from-permissions", new Option_Bool(false));
    //     oc.addDescription("sidewalks.guess.from-permissions", "Pedestrian",
    //                       TL("Add sidewalks for edges that allow pedestrians on any of their lanes regardless of speed"));
    //
    //     oc.doRegister("sidewalks.guess.exclude", new Option_StringVector());
    //     oc.addDescription("sidewalks.guess.exclude", "Pedestrian",
    //                       TL("Do not guess sidewalks for the given list of edges"));
    //
    //     oc.doRegister("bikelanes.guess", new Option_Bool(false));
    //     oc.addDescription("bikelanes.guess", "Bicycle",
    //                       TL("Guess bike lanes based on edge speed"));
    //
    //     oc.doRegister("bikelanes.guess.max-speed", new Option_Float((double) 22.22));
    //     oc.addDescription("bikelanes.guess.max-speed", "Bicycle",
    //                       TL("Add bike lanes for edges with a speed equal or below the given limit"));
    //
    //     oc.doRegister("bikelanes.guess.min-speed", new Option_Float((double) 5.8));
    //     oc.addDescription("bikelanes.guess.min-speed", "Bicycle",
    //                       TL("Add bike lanes for edges with a speed above the given limit"));
    //
    //     oc.doRegister("bikelanes.guess.from-permissions", new Option_Bool(false));
    //     oc.addDescription("bikelanes.guess.from-permissions", "Bicycle",
    //                       TL("Add bike lanes for edges that allow bicycles on any of their lanes regardless of speed"));
    //
    //     oc.doRegister("bikelanes.guess.exclude", new Option_StringVector());
    //     oc.addDescription("bikelanes.guess.exclude", "Bicycle",
    //                       TL("Do not guess bikelanes for the given list of edges"));
    //
    //     oc.doRegister("crossings.guess", new Option_Bool(false));
    //     oc.addDescription("crossings.guess", "Pedestrian",
    //                       TL("Guess pedestrian crossings based on the presence of sidewalks"));
    //
    //     oc.doRegister("crossings.guess.speed-threshold", new Option_Float(13.89));
    //     oc.addDescription("crossings.guess.speed-threshold", "Pedestrian",
    //                       TL("At uncontrolled nodes, do not build crossings across edges with a speed above the threshold"));
    //
    //     oc.doRegister("crossings.guess.roundabout-priority", new Option_Bool(true));
    //     oc.addDescription("crossings.guess.roundabout-priority", "Pedestrian",
    //                       TL("Give priority to guessed crossings at roundabouts"));
    //
    //     oc.doRegister("walkingareas", new Option_Bool(false));
    //     oc.addDescription("walkingareas", "Pedestrian", TL("Always build walking areas even if there are no crossings"));
    //
    //     oc.doRegister("walkingareas.join-dist", new Option_Float(15));
    //     oc.addDescription("walkingareas.join-dist", "Pedestrian", TL("Do not create a walkingarea between sidewalks that are connected by a pedestrian junction within FLOAT"));
    //
    //     // tls setting options
    //     // explicit tls
    //     oc.doRegister("tls.set", new Option_StringVector());
    //     oc.addSynonyme("tls.set", "explicite-tls", true);
    //     oc.addDescription("tls.set", "TLS Building", TL("Interprets STR[] as list of junctions to be controlled by TLS"));
    //
    //     oc.doRegister("tls.unset", new Option_StringVector());
    //     oc.addSynonyme("tls.unset", "explicite-no-tls", true);
    //     oc.addDescription("tls.unset", "TLS Building", TL("Interprets STR[] as list of junctions to be not controlled by TLS"));
    //
    //     // tls-guessing
    //     oc.doRegister("tls.guess", new Option_Bool(false));
    //     oc.addSynonyme("tls.guess", "guess-tls", true);
    //     oc.addDescription("tls.guess", "TLS Building", TL("Turns on TLS guessing"));
    //
    //     oc.doRegister("tls.guess.threshold", new Option_Float(250 / 3.6));
    //     oc.addDescription("tls.guess.threshold", "TLS Building", TL("Sets minimum value for the sum of all incoming lane speeds when guessing TLS"));
    //
    //     if (!forNetgen) {
    //         oc.doRegister("tls.taz-nodes", new Option_Bool(false));
    //         oc.addSynonyme("tls.taz-nodes", "tls-guess.district-nodes", true);
    //         oc.addDescription("tls.taz-nodes", "TLS Building", TL("Sets district nodes as tls-controlled")); // !!! describe
    //     }
    //
    //     oc.doRegister("tls.guess.joining", new Option_Bool(false));
    //     oc.addSynonyme("tls.guess.joining", "tls-guess.joining", true);
    //     oc.addDescription("tls.guess.joining", "TLS Building", TL("Includes node clusters into guess")); // !!! describe
    //
    //     oc.doRegister("tls.join", new Option_Bool(false));
    //     oc.addSynonyme("tls.join", "try-join-tls", true);
    //     oc.addDescription("tls.join", "TLS Building", TL("Tries to cluster tls-controlled nodes")); // !!! describe
    //
    //     oc.doRegister("tls.join-dist", new Option_Float(20));
    //     oc.addDescription("tls.join-dist", "TLS Building",
    //                       TL("Determines the maximal distance for joining traffic lights (defaults to 20)"));
    //
    //     oc.doRegister("tls.join-exclude", new Option_StringVector());
    //     oc.addDescription("tls.join-exclude", "TLS Building", TL("Interprets STR[] as list of tls ids to exclude from joining"));
    //
    //     oc.doRegister("tls.uncontrolled-within", new Option_Bool(false));
    //     oc.addDescription("tls.uncontrolled-within", "TLS Building",
    //                       TL("Do not control edges that lie fully within a joined traffic light. This may cause collisions but allows old traffic light plans to be used"));
    //
    //     oc.doRegister("tls.ignore-internal-junction-jam", new Option_Bool(false));
    //     oc.addDescription("tls.ignore-internal-junction-jam", "TLS Building",
    //                       TL("Do not build mutually conflicting response matrix, potentially ignoring vehicles that are stuck at an internal junction when their phase has ended"));
    //
    //     if (!forNetgen) {
    //         oc.doRegister("tls.guess-signals", new Option_Bool(false));
    //         oc.addDescription("tls.guess-signals", "TLS Building", TL("Interprets tls nodes surrounding an intersection as signal positions for a larger TLS. This is typical pattern for OSM-derived networks"));
    //
    //         oc.doRegister("tls.guess-signals.dist", new Option_Float(25));
    //         oc.addDescription("tls.guess-signals.dist", "TLS Building", TL("Distance for interpreting nodes as signal locations"));
    //
    //         oc.doRegister("tls.guess-signals.slack", new Option_Integer(0));
    //         oc.addDescription("tls.guess-signals.slack", "TLS Building", TL("Number of uncontrolled entry edges to accept and still consider the central node as a traffic light"));
    //     }
    //
    //
    //     // computational
    //     oc.doRegister("tls.cycle.time", new Option_Integer(90));
    //     oc.addDescription("tls.cycle.time", "TLS Building", TL("Use INT as cycle duration"));
    //
    //     oc.doRegister("tls.green.time", new Option_Integer(31));
    //     oc.addSynonyme("tls.green.time", "traffic-light-green", true);
    //     oc.addDescription("tls.green.time", "TLS Building", TL("Use INT as green phase duration"));
    //
    //     oc.doRegister("tls.yellow.min-decel", 'D', new Option_Float(3.0));
    //     oc.addSynonyme("tls.yellow.min-decel", "min-decel", true);
    //     oc.addDescription("tls.yellow.min-decel", "TLS Building", TL("Defines smallest vehicle deceleration"));
    //
    //     oc.doRegister("tls.yellow.patch-small", new Option_Bool(false));
    //     oc.addSynonyme("tls.yellow.patch-small", "patch-small-tyellow", true);
    //     oc.addDescription("tls.yellow.patch-small", "TLS Building", TL("Given yellow times are patched even if being too short"));
    //
    //     oc.doRegister("tls.yellow.time", new Option_Integer(-1));
    //     oc.addSynonyme("tls.yellow.time", "traffic-light-yellow", true);
    //     oc.addDescription("tls.yellow.time", "TLS Building", TL("Set INT as fixed time for yellow phase durations"));
    //
    //     oc.doRegister("tls.red.time", new Option_Integer(5));
    //     oc.addDescription("tls.red.time", "TLS Building", TL("Set INT as fixed time for red phase duration at traffic lights that do not have a conflicting flow"));
    //
    //     oc.doRegister("tls.allred.time", new Option_Integer(0));
    //     oc.addDescription("tls.allred.time", "TLS Building", TL("Set INT as fixed time for intermediate red phase after every switch"));
    //
    //     oc.doRegister("tls.minor-left.max-speed", new Option_Float(19.44)); // 70km/h
    //     oc.addDescription("tls.minor-left.max-speed", "TLS Building", TL("Use FLOAT as threshold for allowing left-turning vehicles to move in the same phase as oncoming straight-going vehicles"));
    //
    //     oc.doRegister("tls.left-green.time", new Option_Integer(6));
    //     oc.addDescription("tls.left-green.time", "TLS Building", TL("Use INT as green phase duration for left turns (s). Setting this value to 0 disables additional left-turning phases"));
    //
    //     oc.doRegister("tls.nema.vehExt", new Option_Integer(2));
    //     oc.addDescription("tls.nema.vehExt", "TLS Building", TL("Set INT as fixed time for intermediate vehext phase after every switch"));
    //
    //     oc.doRegister("tls.nema.yellow", new Option_Integer(3));
    //     oc.addDescription("tls.nema.yellow", "TLS Building", TL("Set INT as fixed time for intermediate NEMA yellow phase after every switch"));
    //
    //     oc.doRegister("tls.nema.red", new Option_Integer(2));
    //     oc.addDescription("tls.nema.red", "TLS Building", TL("Set INT as fixed time for intermediate NEMA red phase after every switch"));
    //
    //     oc.doRegister("tls.crossing-min.time", new Option_Integer(4));
    //     oc.addDescription("tls.crossing-min.time", "TLS Building", TL("Use INT as minimum green duration for pedestrian crossings (s)."));
    //
    //     oc.doRegister("tls.crossing-clearance.time", new Option_Integer(5));
    //     oc.addDescription("tls.crossing-clearance.time", "TLS Building", TL("Use INT as clearance time for pedestrian crossings (s)."));
    //
    //     oc.doRegister("tls.scramble.time", new Option_Integer(5));
    //     oc.addDescription("tls.scramble.time", "TLS Building", TL("Use INT as green phase duration for pedestrian scramble phase (s)."));
    //
    //     // tls-shifts
    //     oc.doRegister("tls.half-offset", new Option_StringVector());
    //     oc.addSynonyme("tls.half-offset", "tl-logics.half-offset", true);
    //     oc.addDescription("tls.half-offset", "TLS Building", TL("TLSs in STR[] will be shifted by half-phase"));
    //
    //     oc.doRegister("tls.quarter-offset", new Option_StringVector());
    //     oc.addSynonyme("tls.quarter-offset", "tl-logics.quarter-offset", true);
    //     oc.addDescription("tls.quarter-offset", "TLS Building", TL("TLSs in STR[] will be shifted by quarter-phase"));
    //
    //     // tls type
    //     oc.doRegister("tls.default-type", new Option_String("static"));
    //     oc.addDescription("tls.default-type", "TLS Building", TL("TLSs with unspecified type will use STR as their algorithm"));
    //
    //     oc.doRegister("tls.layout", new Option_String("opposites"));
    //     oc.addDescription("tls.layout", "TLS Building", TL("Set phase layout four grouping opposite directions or grouping all movements for one incoming edge ['opposites', 'incoming']"));
    //
    //     oc.doRegister("tls.no-mixed", new Option_Bool(false));
    //     oc.addDescription("tls.no-mixed", "TLS Building", TL("Avoid phases with green and red signals for different connections from the same lane"));
    //
    //     oc.doRegister("tls.min-dur", new Option_Integer(5));
    //     oc.addDescription("tls.min-dur", "TLS Building", TL("Default minimum phase duration for traffic lights with variable phase length"));
    //
    //     oc.doRegister("tls.max-dur", new Option_Integer(50));
    //     oc.addDescription("tls.max-dur", "TLS Building", TL("Default maximum phase duration for traffic lights with variable phase length"));
    //
    //     oc.doRegister("tls.group-signals", new Option_Bool(false));
    //     oc.addDescription("tls.group-signals", "TLS Building", TL("Assign the same tls link index to connections that share the same states"));
    //
    //     oc.doRegister("tls.ungroup-signals", new Option_Bool(false));
    //     oc.addDescription("tls.ungroup-signals", "TLS Building", TL("Assign a distinct tls link index to every connection"));
    //
    //     oc.doRegister("tls.rebuild", new Option_Bool(false));
    //     oc.addDescription("tls.rebuild", "TLS Building", TL("rebuild all traffic light plans in the network"));
    //
    //     oc.doRegister("tls.actuated.jam-threshold", new Option_Float(-1.));
    //     oc.addDescription("tls.actuated.jam-threshold", "Processing", TL("Sets default jam-threshold parameter for all actuation detectors"));
    //     oc.doRegister("tls.actuated.show-detectors", new Option_Bool(false));
    //     oc.addDescription("tls.actuated.show-detectors", "Processing", TL("Sets default show-detectors parameter for all actuation detectors"));
    //     oc.doRegister("tls.actuated.passing-time", new Option_Float(-1.));
    //     oc.addDescription("tls.actuated.passing-time", "Processing", TL("Sets default passing-time parameter for all actuation detectors"));
    //     oc.doRegister("tls.actuated.detector-gap", new Option_Float(0.));
    //     oc.addDescription("tls.actuated.detector-gap", "Processing", TL("Sets default detector-gap parameter for all actuation detectors"));
    //     oc.doRegister("tls.actuated.detector-length", new Option_Float(-1.));
    //     oc.addDescription("tls.actuated.detector-length", "Processing", TL("Sets default detector-length parameter for all actuation detectors"));
    //     oc.doRegister("tls.actuated.inactive-threshold", new Option_Float(-1.));
    //     oc.addDescription("tls.actuated.inactive-threshold", "Processing", TL("Sets default inactive-threshold parameter for all actuation detectors"));
    //
    //     // edge pruning
    //     oc.doRegister("keep-edges.min-speed", new Option_Float(-1));
    //     oc.addSynonyme("keep-edges.min-speed", "edges-min-speed", true);
    //     oc.addDescription("keep-edges.min-speed", "Edge Removal", TL("Only keep edges with speed in meters/second > FLOAT"));
    //
    //     oc.doRegister("remove-edges.explicit", new Option_StringVector());
    //     oc.addSynonyme("remove-edges.explicit", "remove-edges");
    //     oc.addDescription("remove-edges.explicit", "Edge Removal", TL("Remove edges in STR[]"));
    //
    //     oc.doRegister("keep-edges.explicit", new Option_StringVector());
    //     oc.addSynonyme("keep-edges.explicit", "keep-edges");
    //     oc.addDescription("keep-edges.explicit", "Edge Removal", TL("Only keep edges in STR[] or those which are kept due to other keep-edges or remove-edges options"));
    //
    //     oc.doRegister("keep-edges.input-file", new Option_FileName());
    //     oc.addDescription("keep-edges.input-file", "Edge Removal", TL("Only keep edges in FILE (Each id on a single line. Selection files from sumo-gui are also supported) or those which are kept due to other keep-edges or remove-edges options"));
    //
    //     oc.doRegister("remove-edges.input-file", new Option_FileName());
    //     oc.addDescription("remove-edges.input-file", "Edge Removal", TL("Remove edges in FILE. (Each id on a single line. Selection files from sumo-gui are also supported)"));
    //
    //     if (!forNetgen) {
    //         oc.doRegister("keep-edges.postload", new Option_Bool(false));
    //         oc.addDescription("keep-edges.postload", "Edge Removal", TL("Remove edges after loading, patching and joining"));
    //     }
    //
    //     oc.doRegister("keep-edges.in-boundary", new Option_StringVector());
    //     oc.addDescription("keep-edges.in-boundary", "Edge Removal", TL("Only keep edges which are located within the given boundary (given either as CARTESIAN corner coordinates <xmin,ymin,xmax,ymax> or as polygon <x0,y0,x1,y1,...>)"));
    //
    //     oc.doRegister("keep-edges.in-geo-boundary", new Option_StringVector());
    //     oc.addDescription("keep-edges.in-geo-boundary", "Edge Removal", TL("Only keep edges which are located within the given boundary (given either as GEODETIC corner coordinates <lon-min,lat-min,lon-max,lat-max> or as polygon <lon0,lat0,lon1,lat1,...>)"));
    //
    //     if (!forNetgen) {
    //         oc.doRegister("keep-edges.by-vclass", new Option_StringVector());
    //         oc.addDescription("keep-edges.by-vclass", "Edge Removal", TL("Only keep edges which allow one of the vclasses in STR[]"));
    //
    //         oc.doRegister("remove-edges.by-vclass", new Option_StringVector());
    //         oc.addDescription("remove-edges.by-vclass", "Edge Removal", TL("Remove edges which allow only vclasses from STR[]"));
    //
    //         oc.doRegister("keep-edges.by-type", new Option_StringVector());
    //         oc.addDescription("keep-edges.by-type", "Edge Removal", TL("Only keep edges where type is in STR[]"));
    //
    //         oc.doRegister("keep-edges.components", new Option_Integer(0));
    //         oc.addDescription("keep-edges.components", "Edge Removal", TL("Only keep the INT largest weakly connected components"));
    //
    //         oc.doRegister("remove-edges.by-type", new Option_StringVector());
    //         oc.addDescription("remove-edges.by-type", "Edge Removal", TL("Remove edges where type is in STR[]"));
    //
    //         oc.doRegister("remove-edges.isolated", new Option_Bool(false));
    //         oc.addSynonyme("remove-edges.isolated", "remove-isolated", true);
    //         oc.addDescription("remove-edges.isolated", "Edge Removal", TL("Removes isolated edges"));
    //     }
    //
    //     oc.doRegister("keep-lanes.min-width", new Option_Float(0.01));
    //     oc.addDescription("keep-lanes.min-width", "Edge Removal", TL("Only keep lanes with width in meters > FLOAT"));
    //
    //
    //     // unregulated nodes options
    //     oc.doRegister("keep-nodes-unregulated", new Option_Bool(false));
    //     oc.addSynonyme("keep-nodes-unregulated", "keep-unregulated");
    //     oc.addDescription("keep-nodes-unregulated", "Unregulated Nodes", TL("All nodes will be unregulated"));
    //
    //     oc.doRegister("keep-nodes-unregulated.explicit", new Option_StringVector());
    //     oc.addSynonyme("keep-nodes-unregulated.explicit", "keep-unregulated.explicit");
    //     oc.addSynonyme("keep-nodes-unregulated.explicit", "keep-unregulated.nodes", true);
    //     oc.addDescription("keep-nodes-unregulated.explicit", "Unregulated Nodes", TL("Do not regulate nodes in STR[]"));
    //
    //     oc.doRegister("keep-nodes-unregulated.district-nodes", new Option_Bool(false));
    //     oc.addSynonyme("keep-nodes-unregulated.district-nodes", "keep-unregulated.district-nodes");
    //     oc.addDescription("keep-nodes-unregulated.district-nodes", "Unregulated Nodes", TL("Do not regulate district nodes"));
    //
    //     // ramp guessing options
    //     if (!forNetgen) {
    //         oc.doRegister("ramps.guess", new Option_Bool(false));
    //         oc.addSynonyme("ramps.guess", "guess-ramps", true);
    //         oc.addDescription("ramps.guess", "Ramp Guessing", TL("Enable ramp-guessing"));
    //
    //         oc.doRegister("ramps.guess-acceleration-lanes", new Option_Bool(true));
    //         oc.addDescription("ramps.guess-acceleration-lanes", "Ramp Guessing", TL("Guess on-ramps and mark acceleration lanes if they exist but do not add new lanes"));
    //
    //         oc.doRegister("ramps.max-ramp-speed", new Option_Float(-1));
    //         oc.addSynonyme("ramps.max-ramp-speed", "ramp-guess.max-ramp-speed", true);
    //         oc.addDescription("ramps.max-ramp-speed", "Ramp Guessing", TL("Treat edges with speed > FLOAT as no ramps"));
    //
    //         oc.doRegister("ramps.min-highway-speed", new Option_Float((double)(79 / 3.6)));
    //         oc.addSynonyme("ramps.min-highway-speed", "ramp-guess.min-highway-speed", true);
    //         oc.addDescription("ramps.min-highway-speed", "Ramp Guessing", TL("Treat edges with speed < FLOAT as no highways"));
    //
    //         oc.doRegister("ramps.ramp-length", new Option_Float(100));
    //         oc.addSynonyme("ramps.ramp-length", "ramp-guess.ramp-length", true);
    //         oc.addDescription("ramps.ramp-length", "Ramp Guessing", TL("Use FLOAT as ramp-length"));
    //
    //         //The Weaving Length Limit for Short Free Onramps, Chiu Liu, Zhongren WangPhD even suggest 70m
    //         oc.doRegister("ramps.min-weave-length", new Option_Float(50));
    //         oc.addDescription("ramps.min-weave-length", "Ramp Guessing", TL("Use FLOAT as minimum ramp-length"));
    //
    //         oc.doRegister("ramps.set", new Option_StringVector());
    //         oc.addSynonyme("ramps.set", "ramp-guess.explicite", true);
    //         oc.addDescription("ramps.set", "Ramp Guessing", TL("Tries to handle the given edges as ramps"));
    //
    //         oc.doRegister("ramps.unset", new Option_StringVector());
    //         oc.addDescription("ramps.unset", "Ramp Guessing", TL("Do not consider the given edges as ramps"));
    //
    //         oc.doRegister("ramps.no-split", new Option_Bool(false));
    //         oc.addSynonyme("ramps.no-split", "ramp-guess.no-split", true);
    //         oc.addDescription("ramps.no-split", "Ramp Guessing", TL("Avoids edge splitting"));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn fill_options(oc: &mut OptionsCont, forNetgen: bool) {
        // register building defaults
        oc.do_register_abbr("default.lanenumber", 'L', SumoOption::integer(1));
        oc.add_synonyme("default.lanenumber", "lanenumber", true);
        oc.add_description("default.lanenumber", "Building Defaults", "The default number of lanes in an edge");

        oc.do_register("default.lanewidth", SumoOption::float(NBEDGE_UNSPECIFIED_WIDTH));
        oc.add_synonyme("default.lanewidth", "lanewidth", true);
        oc.add_description("default.lanewidth", "Building Defaults", "The default width of lanes");

        oc.do_register("default.spreadtype", SumoOption::string_with("right"));
        oc.add_description("default.spreadtype", "Building Defaults", "The default method for computing lane shapes from edge shapes");

        oc.do_register_abbr("default.speed", 'S', SumoOption::float(13.89));
        oc.add_synonyme("default.speed", "speed", true);
        oc.add_description("default.speed", "Building Defaults", "The default speed on an edge (in m/s)");

        oc.do_register("default.friction", SumoOption::float(NBEDGE_UNSPECIFIED_FRICTION));
        oc.add_synonyme("default.friction", "friction", true);
        oc.add_description("default.friction", "Building Defaults", "The default friction on an edge");

        oc.do_register_abbr("default.priority", 'P', SumoOption::integer(-1));
        oc.add_synonyme("default.priority", "priority", true);
        oc.add_description("default.priority", "Building Defaults", "The default priority of an edge");

        oc.do_register("default.type", SumoOption::string());
        oc.add_description("default.type", "Building Defaults", "The default edge type");

        oc.do_register("default.sidewalk-width", SumoOption::float(2.0));
        oc.add_description("default.sidewalk-width", "Building Defaults", "The default width of added sidewalks");

        oc.do_register("default.bikelane-width", SumoOption::float(1.0));
        oc.add_description("default.bikelane-width", "Building Defaults", "The default width of added bike lanes");

        oc.do_register("default.crossing-width", SumoOption::float(4.0));
        oc.add_description("default.crossing-width", "Building Defaults", "The default width of a pedestrian crossing");

        oc.do_register("default.crossing-speed", SumoOption::float(2.78));
        oc.add_description("default.crossing-speed", "Building Defaults", "The default speed 'limit' on a pedestrian crossing (in m/s)");

        oc.do_register("default.walkingarea-speed", SumoOption::float(2.78));
        oc.add_description("default.walkingarea-speed", "Building Defaults", "The default speed 'limit' on a pedestrian walkingarea (in m/s)");

        oc.do_register("default.allow", SumoOption::string());
        oc.add_description("default.allow", "Building Defaults", "The default for allowed vehicle classes");

        oc.do_register("default.disallow", SumoOption::string());
        oc.add_description("default.disallow", "Building Defaults", "The default for disallowed vehicle classes");

        oc.do_register("default.junctions.keep-clear", SumoOption::bool(true));
        oc.add_description("default.junctions.keep-clear", "Building Defaults", "Whether junctions should be kept clear by default");

        oc.do_register("default.junctions.radius", SumoOption::float(4.0));
        oc.add_description("default.junctions.radius", "Building Defaults", "The default turning radius of intersections");

        oc.do_register_abbr("default.junctions.type", 'j', SumoOption::string());
        oc.add_synonyme("default.junctions.type", "default-junction-type", false);
        // default netgenerate option name
        oc.add_description("default.junctions.type", "Building Defaults", "[traffic_light|priority|right_before_left|left_before_right|traffic_light_right_on_red|priority_stop|allway_stop|...] Determines default junction type (see docs/Networks/PlainXML#node_types)");

        oc.do_register("default.connection-length", SumoOption::float(NBEDGE_UNSPECIFIED_LOADED_LENGTH));
        oc.add_description("default.connection-length", "Building Defaults", "The default length when overriding connection lengths");

        oc.do_register("default.connection.cont-pos", SumoOption::float(NBEDGE_UNSPECIFIED_CONTPOS));
        oc.add_description("default.connection.cont-pos", "Building Defaults", "Whether/where connections should have an internal junction");

        oc.do_register("default.right-of-way", SumoOption::string_with("default"));
        oc.add_description("default.right-of-way", "Building Defaults", "The default algorithm for computing right of way rules ('default', 'edgePriority')");

        oc.do_register("junctions.right-before-left.speed-threshold", SumoOption::float(49. / 3.6));
        oc.add_description("junctions.right-before-left.speed-threshold", "Junctions", "Allow building right-before-left junctions when the incoming edge speeds are below FLOAT (m/s)");

        oc.do_register("junctions.left-before-right", SumoOption::bool(false));
        oc.add_description("junctions.left-before-right", "Junctions", "Build left-before-right junctions instead of right-before-left junctions");

        // register the data processing options
        oc.do_register("no-internal-links", SumoOption::bool(false));
        // !!! not described
        oc.add_description("no-internal-links", "Junctions", "Omits internal links");

        oc.do_register("numerical-ids", SumoOption::bool(false));
        oc.add_description("numerical-ids", "Processing", "Remaps alphanumerical IDs of nodes and edges to ensure that all IDs are integers");

        oc.do_register("numerical-ids.node-start", SumoOption::integer(i32::MAX));
        oc.add_description("numerical-ids.node-start", "Processing", "Remaps IDs of nodes to integers starting at INT");

        oc.do_register("numerical-ids.edge-start", SumoOption::integer(i32::MAX));
        oc.add_description("numerical-ids.edge-start", "Processing", "Remaps IDs of edges to integers starting at INT");

        /// @todo not working for netgen
        oc.do_register("reserved-ids", SumoOption::file_name());
        oc.add_description("reserved-ids", "Processing", "Ensures that generated ids do not included any of the typed IDs from FILE (sumo-gui selection file format)");

        oc.do_register("kept-ids", SumoOption::file_name());
        oc.add_description("kept-ids", "Processing", "Ensures that objects with typed IDs from FILE (sumo-gui selection file format) are not renamed");

        if !forNetgen {
        oc.do_register("dismiss-vclasses", SumoOption::bool(false));
        oc.add_description("dismiss-vclasses", "Processing", "Removes vehicle class restrictions from imported edges");
        }

        oc.do_register("no-turnarounds", SumoOption::bool(false));
        oc.add_description("no-turnarounds", "Junctions", "Disables building turnarounds");

        oc.do_register("no-turnarounds.tls", SumoOption::bool(false));
        oc.add_synonyme("no-turnarounds.tls", "no-tls-turnarounds", true);
        oc.add_description("no-turnarounds.tls", "Junctions", "Disables building turnarounds at tls-controlled junctions");

        oc.do_register("no-turnarounds.geometry", SumoOption::bool(true));
        oc.add_description("no-turnarounds.geometry", "Junctions", "Disables building turnarounds at geometry-like junctions");

        oc.do_register("no-turnarounds.except-deadend", SumoOption::bool(false));
        oc.add_description("no-turnarounds.except-deadend", "Junctions", "Disables building turnarounds except at dead end junctions");

        oc.do_register("no-turnarounds.except-turnlane", SumoOption::bool(false));
        oc.add_description("no-turnarounds.except-turnlane", "Junctions", "Disables building turnarounds except at junctions with a dedicated turning lane");

        oc.do_register("no-turnarounds.fringe", SumoOption::bool(false));
        oc.add_description("no-turnarounds.fringe", "Junctions", "Disables building turnarounds at fringe junctions");

        oc.do_register("no-left-connections", SumoOption::bool(false));
        oc.add_description("no-left-connections", "Junctions", "Disables building connections to left");

        oc.do_register("geometry.split", SumoOption::bool(false));
        // !!!not described
        oc.add_synonyme("geometry.split", "split-geometry", true);
        oc.add_description("geometry.split", "Processing", "Splits edges across geometry nodes");

        oc.do_register_abbr("geometry.remove", 'R', SumoOption::bool(false));
        oc.add_synonyme("geometry.remove", "remove-geometry", true);
        oc.add_description("geometry.remove", "Processing", "Replace nodes which only define edge geometry by geometry points (joins edges)");

        oc.do_register("geometry.remove.keep-edges.explicit", SumoOption::string_vector());
        oc.add_description("geometry.remove.keep-edges.explicit", "Processing", "Ensure that the given list of edges is not modified");

        oc.do_register("geometry.remove.keep-edges.input-file", SumoOption::file_name());
        oc.add_description("geometry.remove.keep-edges.input-file", "Processing", "Ensure that the edges in FILE are not modified (Each id on a single line. Selection files from sumo-gui are also supported)");

        if !forNetgen {
        oc.do_register("geometry.remove.keep-ptstops", SumoOption::bool(false));
        oc.add_description("geometry.remove.keep-ptstops", "Processing", "Ensure that edges with public transport stops are not modified");
        }

        oc.do_register("geometry.remove.min-length", SumoOption::float(0.0));
        oc.add_description("geometry.remove.min-length", "Processing", "Allow merging edges with differing attributes when their length is below min-length");

        oc.do_register("geometry.remove.width-tolerance", SumoOption::float(0.0));
        oc.add_description("geometry.remove.width-tolerance", "Processing", "Allow merging edges with differing lane widths if the difference is below FLOAT");

        oc.do_register("geometry.remove.max-junction-size", SumoOption::float(-1.0));
        oc.add_description("geometry.remove.max-junction-size", "Processing", "Prevent removal of junctions with a size above FLOAT as defined by custom edge endpoints");

        oc.do_register("geometry.max-segment-length", SumoOption::float(0.0));
        oc.add_description("geometry.max-segment-length", "Processing", "splits geometry to restrict segment length");

        if !forNetgen {
        oc.do_register("geometry.min-dist", SumoOption::float(-1.0));
        oc.add_description("geometry.min-dist", "Processing", "reduces too similar geometry points");

        oc.do_register("geometry.max-angle", SumoOption::float(99.0));
        oc.add_description("geometry.max-angle", "Processing", "Warn about edge geometries with an angle above DEGREES in successive segments");

        oc.do_register("geometry.max-angle.fix", SumoOption::bool(false));
        oc.add_description("geometry.max-angle.fix", "Processing", "Straighten edge geometries with an angle above max-angle successive segments");

        oc.do_register("geometry.min-radius", SumoOption::float(9.0));
        oc.add_description("geometry.min-radius", "Processing", "Warn about edge geometries with a turning radius less than METERS at the start or end");

        oc.do_register("geometry.min-radius.fix", SumoOption::bool(false));
        oc.add_description("geometry.min-radius.fix", "Processing", "Straighten edge geometries to avoid turning radii less than geometry.min-radius");

        oc.do_register("geometry.min-radius.fix.railways", SumoOption::bool(true));
        oc.add_description("geometry.min-radius.fix.railways", "Processing", "Straighten edge geometries to avoid turning radii less than geometry.min-radius (only railways)");

        oc.do_register("geometry.junction-mismatch-threshold", SumoOption::float(20.0));
        oc.add_description("geometry.junction-mismatch-threshold", "Processing", "Warn if the junction shape is too far away from the original node position");

        oc.do_register("geometry.check-overlap", SumoOption::float(0.0));
        oc.add_description("geometry.check-overlap", "Processing", "Warn if edges overlap by more than the given threshold value");

        oc.do_register("geometry.check-overlap.vertical-threshold", SumoOption::float(4.0));
        oc.add_description("geometry.check-overlap.vertical-threshold", "Processing", "Ignore overlapping edges if they are separated vertically by the given threshold.");

        oc.do_register("geometry.avoid-overlap", SumoOption::bool(true));
        oc.add_description("geometry.avoid-overlap", "Processing", "Modify edge geometries to avoid overlap at junctions");

        oc.do_register("join-lanes", SumoOption::bool(false));
        oc.add_description("join-lanes", "Processing", "join adjacent lanes that have the same permissions and which do not admit lane-changing (sidewalks and disallowed lanes)");

        oc.do_register("ptline.match-dist", SumoOption::float(100.0));
        oc.add_description("ptline.match-dist", "Processing", "Matches stops outside the road network to the referencing pt line when below the given distance");
        // railway processing options

        oc.do_register("railway.topology.repair", SumoOption::bool(false));
        oc.add_description("railway.topology.repair", "Railway", "Repair topology of the railway network");

        oc.do_register("railway.topology.repair.minimal", SumoOption::bool(false));
        oc.add_description("railway.topology.repair.minimal", "Railway", "Repair topology of the railway network just enough to let loaded public transport lines to work");

        oc.do_register("railway.topology.repair.connect-straight", SumoOption::bool(false));
        oc.add_description("railway.topology.repair.connect-straight", "Railway", "Allow bidirectional rail use wherever rails with opposite directions meet at a straight angle");

        oc.do_register("railway.topology.repair.stop-turn", SumoOption::bool(false));
        oc.add_description("railway.topology.repair.stop-turn", "Railway", "Add turn-around connections at all loaded stops.");

        oc.do_register("railway.topology.repair.bidi-penalty", SumoOption::float(1.2));
        oc.add_description("railway.topology.repair.bidi-penalty", "Railway", "Penalty factor for adding new bidi edges to connect public transport stops");

        oc.do_register("railway.topology.all-bidi", SumoOption::bool(false));
        oc.add_description("railway.topology.all-bidi", "Railway", "Make all rails usable in both direction");

        oc.do_register("railway.topology.all-bidi.input-file", SumoOption::file_name());
        oc.add_description("railway.topology.all-bidi.input-file", "Railway", "Make all rails edge ids from FILE usable in both direction");

        oc.do_register("railway.topology.direction-priority", SumoOption::bool(false));
        oc.add_description("railway.topology.direction-priority", "Railway", "Set edge priority values based on estimated main direction");

        oc.do_register("railway.topology.ptline-priority", SumoOption::string_vector());
        oc.add_description("railway.topology.ptline-priority", "Railway", "Set edge priority values based on public transport lines for the given list of vClasses");

        oc.do_register("railway.topology.extend-priority", SumoOption::bool(false));
        oc.add_description("railway.topology.extend-priority", "Railway", "Extend loaded edge priority values based on estimated main direction");

        oc.do_register("railway.geometry.straighten", SumoOption::bool(false));
        oc.add_description("railway.geometry.straighten", "Railway", "Move junctions to straighten a sequence of rail edges");

        oc.do_register("railway.signal.guess.by-stops", SumoOption::bool(false));
        oc.add_description("railway.signal.guess.by-stops", "Railway", "Guess signals that guard public transport stops");
        oc.do_register("railway.signal.guess.by-stops.split", SumoOption::bool(false));
        oc.add_description("railway.signal.guess.by-stops.split", "Railway", "Split edges to improve positioning of guessed signals and prevent switches as rail_signals");

        oc.do_register("railway.signal.permit-unsignalized", SumoOption::string_vector_with(vec!["tram".to_string(), "cable_car".to_string()]));
        oc.add_description("railway.signal.permit-unsignalized", "Railway", "List rail classes that may run without rail signals");

        oc.do_register("railway.access-distance", SumoOption::float(150.0));
        oc.add_description("railway.access-distance", "Railway", "The search radius for finding suitable road accesses for rail stops");
        oc.add_synonyme("railway.access-distance", "osm.stop-output.footway-access-distance", true);

        oc.do_register("railway.max-accesses", SumoOption::integer(5));
        oc.add_description("railway.max-accesses", "Railway", "The maximum road accesses registered per rail stops");
        oc.add_synonyme("railway.max-accesses", "osm.stop-output.footway-max-accesses", true);

        oc.do_register("railway.access-factor", SumoOption::float(1.5));
        oc.add_description("railway.access-factor", "Railway", "The walking length of the access is computed as air-line distance multiplied by FLOAT");
        oc.add_synonyme("railway.access-factor", "osm.stop-output.footway-access-factor", true);

        oc.do_register("ptstop-output.no-bidi", SumoOption::bool(false));
        oc.add_description("ptstop-output.no-bidi", "Processing", "Skips automatic generation of stops on the bidi-edge of a loaded stop");
        }

        oc.do_register("geometry.max-grade", SumoOption::float(10.0));
        oc.add_description("geometry.max-grade", "Processing", "Warn about edge geometries with a grade in % above FLOAT.");

        oc.do_register("geometry.max-grade.fix", SumoOption::bool(true));
        oc.add_description("geometry.max-grade.fix", "Processing", "Smooth edge geometries with a grade above the warning threshold.");

        oc.do_register("offset.disable-normalization", SumoOption::bool(false));
        oc.add_synonyme("offset.disable-normalization", "disable-normalize-node-positions", true);
        oc.add_description("offset.disable-normalization", "Processing", "Turn off normalizing node positions");

        oc.do_register("offset.x", SumoOption::float(0.0));
        oc.add_synonyme("offset.x", "x-offset-to-apply", true);
        oc.add_description("offset.x", "Processing", "Adds FLOAT to net x-positions");

        oc.do_register("offset.y", SumoOption::float(0.0));
        oc.add_synonyme("offset.y", "y-offset-to-apply", true);
        oc.add_description("offset.y", "Processing", "Adds FLOAT to net y-positions");

        oc.do_register("offset.z", SumoOption::float(0.0));
        oc.add_description("offset.z", "Processing", "Adds FLOAT to net z-positions");

        oc.do_register("flip-y-axis", SumoOption::bool(false));
        oc.add_synonyme("flip-y-axis", "flip-y", false);
        oc.add_description("flip-y-axis", "Processing", "Flips the y-coordinate along zero");

        oc.do_register("roundabouts.guess", SumoOption::bool(true));
        oc.add_synonyme("roundabouts.guess", "guess-roundabouts", true);
        oc.add_description("roundabouts.guess", "Processing", "Enable roundabout-guessing");

        // The Putrajaya Roundabout (Malaysia) holds the Guinness record for the world’s largest roundabout with 3.4km.
        oc.do_register("roundabouts.guess.max-length", SumoOption::float(3500.0));
        oc.add_description("roundabouts.guess.max-length", "Processing", "Structures with a circumference above FLOAT threshold are not classified as roundabout");

        oc.do_register("roundabouts.visibility-distance", SumoOption::float(9.0));
        oc.add_description("roundabouts.visibility-distance", "Processing", "Default visibility when approaching a roundabout");

        oc.do_register("opposites.guess", SumoOption::bool(false));
        oc.add_description("opposites.guess", "Processing", "Enable guessing of opposite direction lanes usable for overtaking");

        oc.do_register("opposites.guess.fix-lengths", SumoOption::bool(true));
        oc.add_description("opposites.guess.fix-lengths", "Processing", "Ensure that opposite edges have the same length");

        oc.do_register("fringe.guess", SumoOption::bool(false));
        oc.add_description("fringe.guess", "Processing", "Enable guessing of network fringe nodes");

        oc.do_register("fringe.guess.speed-threshold", SumoOption::float(50. / 3.6));
        oc.add_description("fringe.guess.speed-threshold", "Processing", "Guess disconnected edges above the given speed as outer fringe");

        oc.do_register("lefthand", SumoOption::bool(false));
        oc.add_description("lefthand", "Processing", "Assumes left-hand traffic on the network");

        oc.do_register("edges.join", SumoOption::bool(false));
        oc.add_description("edges.join", "Processing", "Merges edges which connect the same nodes and are close to each other (recommended for VISSIM import)");

        oc.do_register("junctions.join", SumoOption::bool(false));
        oc.add_description("junctions.join", "Junctions", "Joins junctions that are close to each other (recommended for OSM import)");

        oc.do_register("junctions.join-dist", SumoOption::float(10.0));
        oc.add_description("junctions.join-dist", "Junctions", "Determines the maximal distance for joining junctions (defaults to 10)");

        oc.do_register("junctions.join.parallel-threshold", SumoOption::float(30.0));
        oc.add_description("junctions.join.parallel-threshold", "Junctions", "The angular threshold in degrees for rejection of parallel edges when joining junctions");

        if !forNetgen {
        oc.do_register("junctions.join-exclude", SumoOption::string_vector());
        oc.add_description("junctions.join-exclude", "Junctions", "Interprets STR[] as list of junctions to exclude from joining");
        }

        oc.do_register("junctions.join-same", SumoOption::float(-1.0));
        oc.add_description("junctions.join-same", "Junctions", "Joins junctions that have similar coordinates even if not connected");

        oc.do_register("junctions.join-reset", SumoOption::bool(false));
        oc.add_description("junctions.join-reset", "Junctions", "Reset connections for joined junctions");

        if !forNetgen {
        oc.do_register("junctions.attach-removed", SumoOption::float(-1.0));
        oc.add_description("junctions.attach-removed", "Junctions", "Attach junction to the closest edge within FLOAT distance that has it's id in param removedNodeIDs (for joining networks)");
        }

        oc.do_register("max-join-ids", SumoOption::integer(4));
        oc.add_description("max-join-ids", "Junctions", "Abbreviate junction or TLS id if it joins more than INT junctions");

        if !forNetgen {
        oc.do_register("speed.offset", SumoOption::float(0.0));
        oc.add_description("speed.offset", "Processing", "Modifies all edge speeds by adding FLOAT");

        oc.do_register("speed.factor", SumoOption::float(1.0));
        oc.add_description("speed.factor", "Processing", "Modifies all edge speeds by multiplying by FLOAT");

        oc.do_register("speed.minimum", SumoOption::float(0.0));
        oc.add_description("speed.minimum", "Processing", "Modifies all edge speeds to at least FLOAT");

        oc.do_register("edges.join-tram-dist", SumoOption::float(-1.0));
        oc.add_description("edges.join-tram-dist", "Processing", "Joins tram edges into road lanes with similar geometry (within FLOAT distance)");
        }

        oc.do_register("junctions.corner-detail", SumoOption::integer(5));
        oc.add_description("junctions.corner-detail", "Junctions", "Generate INT intermediate points to smooth out intersection corners");

        oc.do_register("junctions.internal-link-detail", SumoOption::integer(5));
        oc.add_description("junctions.internal-link-detail", "Junctions", "Generate INT intermediate points to smooth out lanes within the intersection");

        oc.do_register("junctions.scurve-stretch", SumoOption::float(0.0));
        oc.add_description("junctions.scurve-stretch", "Junctions", "Generate longer intersections to allow for smooth s-curves when the number of lanes changes");

        oc.do_register("junctions.join-turns", SumoOption::bool(false));
        oc.add_description("junctions.join-turns", "Junctions", "Builds common edges for turning connections with common from- and to-edge. This causes discrepancies between geometrical length and assigned length due to averaging but enables lane-changing while turning");

        oc.do_register("junctions.limit-turn-speed", SumoOption::float(5.5));
        oc.add_description("junctions.limit-turn-speed", "Junctions", "Limits speed on junctions to an average lateral acceleration of at most FLOAT (m/s^2)");

        oc.do_register("junctions.limit-turn-speed.min-angle", SumoOption::float(15.0));
        oc.add_description("junctions.limit-turn-speed.min-angle", "Junctions", "Do not limit turn speed for angular changes below FLOAT (degrees). The value is subtracted from the geometric angle before computing the turning radius.");

        oc.do_register("junctions.limit-turn-speed.min-angle.railway", SumoOption::float(35.0));
        oc.add_description("junctions.limit-turn-speed.min-angle.railway", "Junctions", "Do not limit turn speed for angular changes below FLOAT (degrees) on railway edges. The value is subtracted from the geometric angle before computing the turning radius.");

        oc.do_register("junctions.limit-turn-speed.warn.straight", SumoOption::float(5.0));
        oc.add_description("junctions.limit-turn-speed.warn.straight", "Junctions", "Warn about turn speed limits that reduce the speed of straight connections by more than FLOAT");

        oc.do_register("junctions.limit-turn-speed.warn.turn", SumoOption::float(22.0));
        oc.add_description("junctions.limit-turn-speed.warn.turn", "Junctions", "Warn about turn speed limits that reduce the speed of turning connections (no u-turns) by more than FLOAT");

        oc.do_register("junctions.small-radius", SumoOption::float(1.5));
        oc.add_description("junctions.small-radius", "Junctions", "Default radius for junctions that do not require wide vehicle turns");

        oc.do_register("junctions.higher-speed", SumoOption::bool(false));
        oc.add_description("junctions.higher-speed", "Junctions", "Use maximum value of incoming and outgoing edge speed on junction instead of average");

        oc.do_register("junctions.minimal-shape", SumoOption::bool(false));
        oc.add_description("junctions.minimal-shape", "Junctions", "Build junctions with minimal shapes (ignoring edge overlap)");

        oc.do_register("junctions.endpoint-shape", SumoOption::bool(false));
        oc.add_description("junctions.endpoint-shape", "Junctions", "Build junction shapes based on edge endpoints (ignoring edge overlap)");

        oc.do_register("internal-junctions.vehicle-width", SumoOption::float(1.8));
        oc.add_description("internal-junctions.vehicle-width", "Junctions", "Assumed vehicle width for computing internal junction positions");

        oc.do_register("rectangular-lane-cut", SumoOption::bool(false));
        oc.add_description("rectangular-lane-cut", "Junctions", "Forces rectangular cuts between lanes and intersections");

        oc.do_register("check-lane-foes.roundabout", SumoOption::bool(true));
        oc.add_description("check-lane-foes.roundabout", "Junctions", "Allow driving onto a multi-lane road if there are foes on other lanes (at roundabouts)");

        oc.do_register("check-lane-foes.all", SumoOption::bool(false));
        oc.add_description("check-lane-foes.all", "Junctions", "Allow driving onto a multi-lane road if there are foes on other lanes (everywhere)");

        oc.do_register("sidewalks.guess", SumoOption::bool(false));
        oc.add_description("sidewalks.guess", "Pedestrian", "Guess pedestrian sidewalks based on edge speed");

        oc.do_register("sidewalks.guess.max-speed", SumoOption::float(13.89));
        oc.add_description("sidewalks.guess.max-speed", "Pedestrian", "Add sidewalks for edges with a speed equal or below the given limit");

        oc.do_register("sidewalks.guess.min-speed", SumoOption::float(5.8));
        oc.add_description("sidewalks.guess.min-speed", "Pedestrian", "Add sidewalks for edges with a speed above the given limit");

        oc.do_register("sidewalks.guess.from-permissions", SumoOption::bool(false));
        oc.add_description("sidewalks.guess.from-permissions", "Pedestrian", "Add sidewalks for edges that allow pedestrians on any of their lanes regardless of speed");

        oc.do_register("sidewalks.guess.exclude", SumoOption::string_vector());
        oc.add_description("sidewalks.guess.exclude", "Pedestrian", "Do not guess sidewalks for the given list of edges");

        oc.do_register("bikelanes.guess", SumoOption::bool(false));
        oc.add_description("bikelanes.guess", "Bicycle", "Guess bike lanes based on edge speed");

        oc.do_register("bikelanes.guess.max-speed", SumoOption::float(22.22));
        oc.add_description("bikelanes.guess.max-speed", "Bicycle", "Add bike lanes for edges with a speed equal or below the given limit");

        oc.do_register("bikelanes.guess.min-speed", SumoOption::float(5.8));
        oc.add_description("bikelanes.guess.min-speed", "Bicycle", "Add bike lanes for edges with a speed above the given limit");

        oc.do_register("bikelanes.guess.from-permissions", SumoOption::bool(false));
        oc.add_description("bikelanes.guess.from-permissions", "Bicycle", "Add bike lanes for edges that allow bicycles on any of their lanes regardless of speed");

        oc.do_register("bikelanes.guess.exclude", SumoOption::string_vector());
        oc.add_description("bikelanes.guess.exclude", "Bicycle", "Do not guess bikelanes for the given list of edges");

        oc.do_register("crossings.guess", SumoOption::bool(false));
        oc.add_description("crossings.guess", "Pedestrian", "Guess pedestrian crossings based on the presence of sidewalks");

        oc.do_register("crossings.guess.speed-threshold", SumoOption::float(13.89));
        oc.add_description("crossings.guess.speed-threshold", "Pedestrian", "At uncontrolled nodes, do not build crossings across edges with a speed above the threshold");

        oc.do_register("crossings.guess.roundabout-priority", SumoOption::bool(true));
        oc.add_description("crossings.guess.roundabout-priority", "Pedestrian", "Give priority to guessed crossings at roundabouts");

        oc.do_register("walkingareas", SumoOption::bool(false));
        oc.add_description("walkingareas", "Pedestrian", "Always build walking areas even if there are no crossings");

        oc.do_register("walkingareas.join-dist", SumoOption::float(15.0));
        oc.add_description("walkingareas.join-dist", "Pedestrian", "Do not create a walkingarea between sidewalks that are connected by a pedestrian junction within FLOAT");

        // tls setting options
        // explicit tls
        oc.do_register("tls.set", SumoOption::string_vector());
        oc.add_synonyme("tls.set", "explicite-tls", true);
        oc.add_description("tls.set", "TLS Building", "Interprets STR[] as list of junctions to be controlled by TLS");

        oc.do_register("tls.unset", SumoOption::string_vector());
        oc.add_synonyme("tls.unset", "explicite-no-tls", true);
        oc.add_description("tls.unset", "TLS Building", "Interprets STR[] as list of junctions to be not controlled by TLS");

        // tls-guessing
        oc.do_register("tls.guess", SumoOption::bool(false));
        oc.add_synonyme("tls.guess", "guess-tls", true);
        oc.add_description("tls.guess", "TLS Building", "Turns on TLS guessing");

        oc.do_register("tls.guess.threshold", SumoOption::float(250. / 3.6));
        oc.add_description("tls.guess.threshold", "TLS Building", "Sets minimum value for the sum of all incoming lane speeds when guessing TLS");

        if !forNetgen {
        oc.do_register("tls.taz-nodes", SumoOption::bool(false));
        oc.add_synonyme("tls.taz-nodes", "tls-guess.district-nodes", true);
        oc.add_description("tls.taz-nodes", "TLS Building", "Sets district nodes as tls-controlled");
        // !!! describe
        }

        oc.do_register("tls.guess.joining", SumoOption::bool(false));
        oc.add_synonyme("tls.guess.joining", "tls-guess.joining", true);
        oc.add_description("tls.guess.joining", "TLS Building", "Includes node clusters into guess");
        // !!! describe

        oc.do_register("tls.join", SumoOption::bool(false));
        oc.add_synonyme("tls.join", "try-join-tls", true);
        oc.add_description("tls.join", "TLS Building", "Tries to cluster tls-controlled nodes");
        // !!! describe

        oc.do_register("tls.join-dist", SumoOption::float(20.0));
        oc.add_description("tls.join-dist", "TLS Building", "Determines the maximal distance for joining traffic lights (defaults to 20)");

        oc.do_register("tls.join-exclude", SumoOption::string_vector());
        oc.add_description("tls.join-exclude", "TLS Building", "Interprets STR[] as list of tls ids to exclude from joining");

        oc.do_register("tls.uncontrolled-within", SumoOption::bool(false));
        oc.add_description("tls.uncontrolled-within", "TLS Building", "Do not control edges that lie fully within a joined traffic light. This may cause collisions but allows old traffic light plans to be used");

        oc.do_register("tls.ignore-internal-junction-jam", SumoOption::bool(false));
        oc.add_description("tls.ignore-internal-junction-jam", "TLS Building", "Do not build mutually conflicting response matrix, potentially ignoring vehicles that are stuck at an internal junction when their phase has ended");

        if !forNetgen {
        oc.do_register("tls.guess-signals", SumoOption::bool(false));
        oc.add_description("tls.guess-signals", "TLS Building", "Interprets tls nodes surrounding an intersection as signal positions for a larger TLS. This is typical pattern for OSM-derived networks");

        oc.do_register("tls.guess-signals.dist", SumoOption::float(25.0));
        oc.add_description("tls.guess-signals.dist", "TLS Building", "Distance for interpreting nodes as signal locations");

        oc.do_register("tls.guess-signals.slack", SumoOption::integer(0));
        oc.add_description("tls.guess-signals.slack", "TLS Building", "Number of uncontrolled entry edges to accept and still consider the central node as a traffic light");
        }


        // computational
        oc.do_register("tls.cycle.time", SumoOption::integer(90));
        oc.add_description("tls.cycle.time", "TLS Building", "Use INT as cycle duration");

        oc.do_register("tls.green.time", SumoOption::integer(31));
        oc.add_synonyme("tls.green.time", "traffic-light-green", true);
        oc.add_description("tls.green.time", "TLS Building", "Use INT as green phase duration");

        oc.do_register_abbr("tls.yellow.min-decel", 'D', SumoOption::float(3.0));
        oc.add_synonyme("tls.yellow.min-decel", "min-decel", true);
        oc.add_description("tls.yellow.min-decel", "TLS Building", "Defines smallest vehicle deceleration");

        oc.do_register("tls.yellow.patch-small", SumoOption::bool(false));
        oc.add_synonyme("tls.yellow.patch-small", "patch-small-tyellow", true);
        oc.add_description("tls.yellow.patch-small", "TLS Building", "Given yellow times are patched even if being too short");

        oc.do_register("tls.yellow.time", SumoOption::integer(-1));
        oc.add_synonyme("tls.yellow.time", "traffic-light-yellow", true);
        oc.add_description("tls.yellow.time", "TLS Building", "Set INT as fixed time for yellow phase durations");

        oc.do_register("tls.red.time", SumoOption::integer(5));
        oc.add_description("tls.red.time", "TLS Building", "Set INT as fixed time for red phase duration at traffic lights that do not have a conflicting flow");

        oc.do_register("tls.allred.time", SumoOption::integer(0));
        oc.add_description("tls.allred.time", "TLS Building", "Set INT as fixed time for intermediate red phase after every switch");

        oc.do_register("tls.minor-left.max-speed", SumoOption::float(19.44));
        // 70km/h
        oc.add_description("tls.minor-left.max-speed", "TLS Building", "Use FLOAT as threshold for allowing left-turning vehicles to move in the same phase as oncoming straight-going vehicles");

        oc.do_register("tls.left-green.time", SumoOption::integer(6));
        oc.add_description("tls.left-green.time", "TLS Building", "Use INT as green phase duration for left turns (s). Setting this value to 0 disables additional left-turning phases");

        oc.do_register("tls.nema.vehExt", SumoOption::integer(2));
        oc.add_description("tls.nema.vehExt", "TLS Building", "Set INT as fixed time for intermediate vehext phase after every switch");

        oc.do_register("tls.nema.yellow", SumoOption::integer(3));
        oc.add_description("tls.nema.yellow", "TLS Building", "Set INT as fixed time for intermediate NEMA yellow phase after every switch");

        oc.do_register("tls.nema.red", SumoOption::integer(2));
        oc.add_description("tls.nema.red", "TLS Building", "Set INT as fixed time for intermediate NEMA red phase after every switch");

        oc.do_register("tls.crossing-min.time", SumoOption::integer(4));
        oc.add_description("tls.crossing-min.time", "TLS Building", "Use INT as minimum green duration for pedestrian crossings (s).");

        oc.do_register("tls.crossing-clearance.time", SumoOption::integer(5));
        oc.add_description("tls.crossing-clearance.time", "TLS Building", "Use INT as clearance time for pedestrian crossings (s).");

        oc.do_register("tls.scramble.time", SumoOption::integer(5));
        oc.add_description("tls.scramble.time", "TLS Building", "Use INT as green phase duration for pedestrian scramble phase (s).");

        // tls-shifts
        oc.do_register("tls.half-offset", SumoOption::string_vector());
        oc.add_synonyme("tls.half-offset", "tl-logics.half-offset", true);
        oc.add_description("tls.half-offset", "TLS Building", "TLSs in STR[] will be shifted by half-phase");

        oc.do_register("tls.quarter-offset", SumoOption::string_vector());
        oc.add_synonyme("tls.quarter-offset", "tl-logics.quarter-offset", true);
        oc.add_description("tls.quarter-offset", "TLS Building", "TLSs in STR[] will be shifted by quarter-phase");

        // tls type
        oc.do_register("tls.default-type", SumoOption::string_with("static"));
        oc.add_description("tls.default-type", "TLS Building", "TLSs with unspecified type will use STR as their algorithm");

        oc.do_register("tls.layout", SumoOption::string_with("opposites"));
        oc.add_description("tls.layout", "TLS Building", "Set phase layout four grouping opposite directions or grouping all movements for one incoming edge ['opposites', 'incoming']");

        oc.do_register("tls.no-mixed", SumoOption::bool(false));
        oc.add_description("tls.no-mixed", "TLS Building", "Avoid phases with green and red signals for different connections from the same lane");

        oc.do_register("tls.min-dur", SumoOption::integer(5));
        oc.add_description("tls.min-dur", "TLS Building", "Default minimum phase duration for traffic lights with variable phase length");

        oc.do_register("tls.max-dur", SumoOption::integer(50));
        oc.add_description("tls.max-dur", "TLS Building", "Default maximum phase duration for traffic lights with variable phase length");

        oc.do_register("tls.group-signals", SumoOption::bool(false));
        oc.add_description("tls.group-signals", "TLS Building", "Assign the same tls link index to connections that share the same states");

        oc.do_register("tls.ungroup-signals", SumoOption::bool(false));
        oc.add_description("tls.ungroup-signals", "TLS Building", "Assign a distinct tls link index to every connection");

        oc.do_register("tls.rebuild", SumoOption::bool(false));
        oc.add_description("tls.rebuild", "TLS Building", "rebuild all traffic light plans in the network");

        oc.do_register("tls.actuated.jam-threshold", SumoOption::float(-1.));
        oc.add_description("tls.actuated.jam-threshold", "Processing", "Sets default jam-threshold parameter for all actuation detectors");
        oc.do_register("tls.actuated.show-detectors", SumoOption::bool(false));
        oc.add_description("tls.actuated.show-detectors", "Processing", "Sets default show-detectors parameter for all actuation detectors");
        oc.do_register("tls.actuated.passing-time", SumoOption::float(-1.));
        oc.add_description("tls.actuated.passing-time", "Processing", "Sets default passing-time parameter for all actuation detectors");
        oc.do_register("tls.actuated.detector-gap", SumoOption::float(0.));
        oc.add_description("tls.actuated.detector-gap", "Processing", "Sets default detector-gap parameter for all actuation detectors");
        oc.do_register("tls.actuated.detector-length", SumoOption::float(-1.));
        oc.add_description("tls.actuated.detector-length", "Processing", "Sets default detector-length parameter for all actuation detectors");
        oc.do_register("tls.actuated.inactive-threshold", SumoOption::float(-1.));
        oc.add_description("tls.actuated.inactive-threshold", "Processing", "Sets default inactive-threshold parameter for all actuation detectors");

        // edge pruning
        oc.do_register("keep-edges.min-speed", SumoOption::float(-1.0));
        oc.add_synonyme("keep-edges.min-speed", "edges-min-speed", true);
        oc.add_description("keep-edges.min-speed", "Edge Removal", "Only keep edges with speed in meters/second > FLOAT");

        oc.do_register("remove-edges.explicit", SumoOption::string_vector());
        oc.add_synonyme("remove-edges.explicit", "remove-edges", false);
        oc.add_description("remove-edges.explicit", "Edge Removal", "Remove edges in STR[]");

        oc.do_register("keep-edges.explicit", SumoOption::string_vector());
        oc.add_synonyme("keep-edges.explicit", "keep-edges", false);
        oc.add_description("keep-edges.explicit", "Edge Removal", "Only keep edges in STR[] or those which are kept due to other keep-edges or remove-edges options");

        oc.do_register("keep-edges.input-file", SumoOption::file_name());
        oc.add_description("keep-edges.input-file", "Edge Removal", "Only keep edges in FILE (Each id on a single line. Selection files from sumo-gui are also supported) or those which are kept due to other keep-edges or remove-edges options");

        oc.do_register("remove-edges.input-file", SumoOption::file_name());
        oc.add_description("remove-edges.input-file", "Edge Removal", "Remove edges in FILE. (Each id on a single line. Selection files from sumo-gui are also supported)");

        if !forNetgen {
        oc.do_register("keep-edges.postload", SumoOption::bool(false));
        oc.add_description("keep-edges.postload", "Edge Removal", "Remove edges after loading, patching and joining");
        }

        oc.do_register("keep-edges.in-boundary", SumoOption::string_vector());
        oc.add_description("keep-edges.in-boundary", "Edge Removal", "Only keep edges which are located within the given boundary (given either as CARTESIAN corner coordinates <xmin,ymin,xmax,ymax> or as polygon <x0,y0,x1,y1,...>)");

        oc.do_register("keep-edges.in-geo-boundary", SumoOption::string_vector());
        oc.add_description("keep-edges.in-geo-boundary", "Edge Removal", "Only keep edges which are located within the given boundary (given either as GEODETIC corner coordinates <lon-min,lat-min,lon-max,lat-max> or as polygon <lon0,lat0,lon1,lat1,...>)");

        if !forNetgen {
        oc.do_register("keep-edges.by-vclass", SumoOption::string_vector());
        oc.add_description("keep-edges.by-vclass", "Edge Removal", "Only keep edges which allow one of the vclasses in STR[]");

        oc.do_register("remove-edges.by-vclass", SumoOption::string_vector());
        oc.add_description("remove-edges.by-vclass", "Edge Removal", "Remove edges which allow only vclasses from STR[]");

        oc.do_register("keep-edges.by-type", SumoOption::string_vector());
        oc.add_description("keep-edges.by-type", "Edge Removal", "Only keep edges where type is in STR[]");

        oc.do_register("keep-edges.components", SumoOption::integer(0));
        oc.add_description("keep-edges.components", "Edge Removal", "Only keep the INT largest weakly connected components");

        oc.do_register("remove-edges.by-type", SumoOption::string_vector());
        oc.add_description("remove-edges.by-type", "Edge Removal", "Remove edges where type is in STR[]");

        oc.do_register("remove-edges.isolated", SumoOption::bool(false));
        oc.add_synonyme("remove-edges.isolated", "remove-isolated", true);
        oc.add_description("remove-edges.isolated", "Edge Removal", "Removes isolated edges");
        }

        oc.do_register("keep-lanes.min-width", SumoOption::float(0.01));
        oc.add_description("keep-lanes.min-width", "Edge Removal", "Only keep lanes with width in meters > FLOAT");


        // unregulated nodes options
        oc.do_register("keep-nodes-unregulated", SumoOption::bool(false));
        oc.add_synonyme("keep-nodes-unregulated", "keep-unregulated", false);
        oc.add_description("keep-nodes-unregulated", "Unregulated Nodes", "All nodes will be unregulated");

        oc.do_register("keep-nodes-unregulated.explicit", SumoOption::string_vector());
        oc.add_synonyme("keep-nodes-unregulated.explicit", "keep-unregulated.explicit", false);
        oc.add_synonyme("keep-nodes-unregulated.explicit", "keep-unregulated.nodes", true);
        oc.add_description("keep-nodes-unregulated.explicit", "Unregulated Nodes", "Do not regulate nodes in STR[]");

        oc.do_register("keep-nodes-unregulated.district-nodes", SumoOption::bool(false));
        oc.add_synonyme("keep-nodes-unregulated.district-nodes", "keep-unregulated.district-nodes", false);
        oc.add_description("keep-nodes-unregulated.district-nodes", "Unregulated Nodes", "Do not regulate district nodes");

        // ramp guessing options
        if !forNetgen {
        oc.do_register("ramps.guess", SumoOption::bool(false));
        oc.add_synonyme("ramps.guess", "guess-ramps", true);
        oc.add_description("ramps.guess", "Ramp Guessing", "Enable ramp-guessing");

        oc.do_register("ramps.guess-acceleration-lanes", SumoOption::bool(true));
        oc.add_description("ramps.guess-acceleration-lanes", "Ramp Guessing", "Guess on-ramps and mark acceleration lanes if they exist but do not add new lanes");

        oc.do_register("ramps.max-ramp-speed", SumoOption::float(-1.0));
        oc.add_synonyme("ramps.max-ramp-speed", "ramp-guess.max-ramp-speed", true);
        oc.add_description("ramps.max-ramp-speed", "Ramp Guessing", "Treat edges with speed > FLOAT as no ramps");

        oc.do_register("ramps.min-highway-speed", SumoOption::float((79. / 3.6)));
        oc.add_synonyme("ramps.min-highway-speed", "ramp-guess.min-highway-speed", true);
        oc.add_description("ramps.min-highway-speed", "Ramp Guessing", "Treat edges with speed < FLOAT as no highways");

        oc.do_register("ramps.ramp-length", SumoOption::float(100.0));
        oc.add_synonyme("ramps.ramp-length", "ramp-guess.ramp-length", true);
        oc.add_description("ramps.ramp-length", "Ramp Guessing", "Use FLOAT as ramp-length");

        //The Weaving Length Limit for Short Free Onramps, Chiu Liu, Zhongren WangPhD even suggest 70m
        oc.do_register("ramps.min-weave-length", SumoOption::float(50.0));
        oc.add_description("ramps.min-weave-length", "Ramp Guessing", "Use FLOAT as minimum ramp-length");

        oc.do_register("ramps.set", SumoOption::string_vector());
        oc.add_synonyme("ramps.set", "ramp-guess.explicite", true);
        oc.add_description("ramps.set", "Ramp Guessing", "Tries to handle the given edges as ramps");

        oc.do_register("ramps.unset", SumoOption::string_vector());
        oc.add_description("ramps.unset", "Ramp Guessing", "Do not consider the given edges as ramps");

        oc.do_register("ramps.no-split", SumoOption::bool(false));
        oc.add_synonyme("ramps.no-split", "ramp-guess.no-split", true);
        oc.add_description("ramps.no-split", "Ramp Guessing", "Avoids edge splitting");
        }
    }

    /// C++: `bool NBFrame::checkOptions(OptionsCont &)` [static]
    /// Checks set options from the OptionsCont-singleton for being valid
    // bool
    // NBFrame::checkOptions(OptionsCont& oc) {
    //     bool ok = true;
    //     //
    //     if (!SUMOXMLDefinitions::TrafficLightTypes.hasString(oc.getString("tls.default-type"))) {
    //         WRITE_ERRORF(TL("unsupported value '%' for option '--tls.default-type'"), oc.getString("tls.default-type"));
    //         ok = false;
    //     }
    //     if (oc.isSet("keep-edges.in-boundary") && oc.isSet("keep-edges.in-geo-boundary")) {
    //         WRITE_ERROR(TL("only one of the options 'keep-edges.in-boundary' or 'keep-edges.in-geo-boundary' may be given"));
    //         ok = false;
    //     }
    //     if (oc.getBool("no-internal-links") && oc.getBool("crossings.guess")) {
    //         WRITE_ERROR(TL("only one of the options 'no-internal-links' or 'crossings.guess' may be given"));
    //         ok = false;
    //     }
    //     if (oc.getBool("no-internal-links") && oc.getBool("walkingareas")) {
    //         WRITE_ERROR(TL("only one of the options 'no-internal-links' or 'walkareas' may be given"));
    //         ok = false;
    //     }
    //     if (!oc.isDefault("tls.green.time") && !oc.isDefault("tls.cycle.time")) {
    //         WRITE_ERROR(TL("only one of the options 'tls.green.time' or 'tls.cycle.time' may be given"));
    //         ok = false;
    //     }
    //     if (oc.getInt("tls.green.time") <= 0) {
    //         WRITE_ERROR(TL("'tls.green.time' must be positive"));
    //         ok = false;
    //     }
    //     if (oc.getInt("default.lanenumber") < 1) {
    //         WRITE_ERROR(TL("default.lanenumber must be at least 1"));
    //         ok = false;
    //     }
    //     if (!oc.isDefault("default.lanewidth") && oc.getFloat("default.lanewidth") < POSITION_EPS) {
    //         WRITE_ERROR("default.lanewidth must be at least " + toString(POSITION_EPS));
    //         ok = false;
    //     }
    //     if (!oc.isDefault("default.disallow") && !oc.isDefault("default.allow")) {
    //         WRITE_ERROR(TL("only one of the options 'default.disallow' or 'default.allow' may be given"));
    //         ok = false;
    //     }
    //     if (oc.getInt("junctions.internal-link-detail") < 2) {
    //         WRITE_ERROR(TL("junctions.internal-link-detail must >= 2"));
    //         ok = false;
    //     }
    //     if (oc.getFloat("junctions.scurve-stretch") > 0) {
    //         if (oc.getBool("no-internal-links")) {
    //             WRITE_WARNING(TL("Option 'junctions.scurve-stretch' requires internal lanes to work. Option '--no-internal-links' will be disabled."));
    //         }
    //         // make sure the option is set so heuristics cannot ignore it
    //         oc.set("no-internal-links", "false");
    //     }
    //     if (oc.getFloat("junctions.small-radius") > oc.getFloat("default.junctions.radius") && oc.getFloat("default.junctions.radius") >= 0) {
    //         if (!oc.isDefault("junctions.small-radius")) {
    //             WRITE_WARNING(TL("option 'default.junctions.radius' is smaller than option 'junctions.small-radius'"));
    //         } else {
    //             oc.setDefault("junctions.small-radius", oc.getValueString("default.junctions.radius"));
    //         }
    //     }
    //     if (oc.getString("tls.layout") != "opposites"
    //             && oc.getString("tls.layout") != "incoming"
    //             && oc.getString("tls.layout") != "alternateOneWay") {
    //         WRITE_ERROR(TL("tls.layout must be 'opposites', 'incoming' or 'alternateOneWay'"));
    //         ok = false;
    //     }
    //     if (!oc.isDefault("default.right-of-way") &&
    //             !SUMOXMLDefinitions::RightOfWayValues.hasString(oc.getString("default.right-of-way"))) {
    //         WRITE_ERRORF(TL("default.right-of-way must be one of '%'"), toString(SUMOXMLDefinitions::RightOfWayValues.getStrings()));
    //         ok = false;
    //     }
    //     if (oc.getFloat("roundabouts.visibility-distance") < 0 && oc.getFloat("roundabouts.visibility-distance") != NBEDGE_UNSPECIFIED_VISIBILITY_DISTANCE) {
    //         WRITE_ERROR(TL("roundabouts.visibility-distance must be positive or -1"));
    //         ok = false;
    //     }
    //     if (oc.isDefault("railway.topology.repair") && oc.getBool("railway.topology.repair.connect-straight")) {
    //         oc.setDefault("railway.topology.repair", "true");
    //     }
    //     if (oc.isDefault("railway.topology.repair") && oc.getBool("railway.topology.repair.minimal")) {
    //         oc.setDefault("railway.topology.repair", "true");
    //     }
    //     if (oc.isDefault("railway.topology.all-bidi") && !oc.isDefault("railway.topology.all-bidi.input-file")) {
    //         oc.setDefault("railway.topology.all-bidi", "true");
    //     }
    //     if (oc.isDefault("railway.topology.repair.stop-turn") && !oc.isDefault("railway.topology.repair")
    //             // restore legacy behavior after making the option effective during general import
    //             && oc.exists("sumo-net-file") && oc.isSet("sumo-net-file")) {
    //         oc.setDefault("railway.topology.repair.stop-turn", "true");
    //     }
    //     if (!oc.isDefault("railway.signal.permit-unsignalized")) {
    //         try {
    //             parseVehicleClasses(oc.getStringVector("railway.signal.permit-unsignalized"));
    //         } catch (InvalidArgument& e) {
    //             WRITE_ERROR(e.what())
    //             ok = false;
    //         }
    //     }
    //     if (!SUMOXMLDefinitions::LaneSpreadFunctions.hasString(oc.getString("default.spreadtype"))) {
    //         WRITE_ERRORF(TL("Unknown value for default.spreadtype '%'."), oc.getString("default.spreadtype"));
    //         ok = false;
    //     }
    //     // check whether the junction type to use is properly set
    //     if (oc.isSet("default.junctions.type")) {
    //         std::string type = oc.getString("default.junctions.type");
    //         if (type != toString(SumoXMLNodeType::TRAFFIC_LIGHT) &&
    //                 type != toString(SumoXMLNodeType::TRAFFIC_LIGHT_NOJUNCTION) &&
    //                 type != toString(SumoXMLNodeType::TRAFFIC_LIGHT_RIGHT_ON_RED) &&
    //                 type != toString(SumoXMLNodeType::PRIORITY) &&
    //                 type != toString(SumoXMLNodeType::PRIORITY_STOP) &&
    //                 type != toString(SumoXMLNodeType::ALLWAY_STOP) &&
    //                 type != toString(SumoXMLNodeType::ZIPPER) &&
    //                 type != toString(SumoXMLNodeType::NOJUNCTION) &&
    //                 type != toString(SumoXMLNodeType::RAIL_SIGNAL) &&
    //                 type != toString(SumoXMLNodeType::RAIL_CROSSING) &&
    //                 type != toString(SumoXMLNodeType::LEFT_BEFORE_RIGHT) &&
    //                 type != toString(SumoXMLNodeType::RIGHT_BEFORE_LEFT) &&
    //                 type != toString(SumoXMLNodeType::NOJUNCTION) &&
    //                 type != toString(SumoXMLNodeType::UNKNOWN)) {
    //             WRITE_ERROR("Only the following junction types are known: " +
    //                         toString(SumoXMLNodeType::TRAFFIC_LIGHT) + ", " +
    //                         toString(SumoXMLNodeType::TRAFFIC_LIGHT_NOJUNCTION) + ", " +
    //                         toString(SumoXMLNodeType::TRAFFIC_LIGHT_RIGHT_ON_RED) + ", " +
    //                         toString(SumoXMLNodeType::PRIORITY) + ", " +
    //                         toString(SumoXMLNodeType::PRIORITY_STOP) + ", " +
    //                         toString(SumoXMLNodeType::ALLWAY_STOP) + ", " +
    //                         toString(SumoXMLNodeType::ZIPPER) + ", " +
    //                         toString(SumoXMLNodeType::NOJUNCTION) + ", " +
    //                         toString(SumoXMLNodeType::RAIL_SIGNAL) + ", " +
    //                         toString(SumoXMLNodeType::RAIL_CROSSING) + ", " +
    //                         toString(SumoXMLNodeType::LEFT_BEFORE_RIGHT) + ", " +
    //                         toString(SumoXMLNodeType::RIGHT_BEFORE_LEFT) + ", " +
    //                         toString(SumoXMLNodeType::UNKNOWN));
    //             ok = false;
    //         }
    //     }
    //     return ok;
    // }
    #[allow(unused_variables)]
    pub fn check_options(oc: &mut OptionsCont) -> bool {
        let mut ok = true;
        //
        if !TrafficLightTypes.has_string(&oc.get_string("tls.default-type").unwrap()) {
            write_errorf!("unsupported value '%' for option '--tls.default-type'", oc.get_string("tls.default-type").unwrap());
            ok = false;
        }
        if oc.is_set("keep-edges.in-boundary", true).unwrap() && oc.is_set("keep-edges.in-geo-boundary", true).unwrap() {
            write_error("only one of the options 'keep-edges.in-boundary' or 'keep-edges.in-geo-boundary' may be given");
            ok = false;
        }
        if oc.get_bool("no-internal-links").unwrap() && oc.get_bool("crossings.guess").unwrap() {
            write_error("only one of the options 'no-internal-links' or 'crossings.guess' may be given");
            ok = false;
        }
        if oc.get_bool("no-internal-links").unwrap() && oc.get_bool("walkingareas").unwrap() {
            write_error("only one of the options 'no-internal-links' or 'walkareas' may be given");
            ok = false;
        }
        if !oc.is_default("tls.green.time") && !oc.is_default("tls.cycle.time") {
            write_error("only one of the options 'tls.green.time' or 'tls.cycle.time' may be given");
            ok = false;
        }
        if oc.get_int("tls.green.time").unwrap() <= 0 {
            write_error("'tls.green.time' must be positive");
            ok = false;
        }
        if oc.get_int("default.lanenumber").unwrap() < 1 {
            write_error("default.lanenumber must be at least 1");
            ok = false;
        }
        if !oc.is_default("default.lanewidth") && oc.get_float("default.lanewidth").unwrap() < POSITION_EPS {
            write_error(&format!("default.lanewidth must be at least {}", f64_to_string_prec(POSITION_EPS, g_precision())));
            ok = false;
        }
        if !oc.is_default("default.disallow") && !oc.is_default("default.allow") {
            write_error("only one of the options 'default.disallow' or 'default.allow' may be given");
            ok = false;
        }
        if oc.get_int("junctions.internal-link-detail").unwrap() < 2 {
            write_error("junctions.internal-link-detail must >= 2");
            ok = false;
        }
        if oc.get_float("junctions.scurve-stretch").unwrap() > 0. {
            if oc.get_bool("no-internal-links").unwrap() {
                write_warning("Option 'junctions.scurve-stretch' requires internal lanes to work. Option '--no-internal-links' will be disabled.");
            }
            // make sure the option is set so heuristics cannot ignore it
            oc.set("no-internal-links", "false", false).unwrap();
        }
        if oc.get_float("junctions.small-radius").unwrap() > oc.get_float("default.junctions.radius").unwrap() && oc.get_float("default.junctions.radius").unwrap() >= 0. {
            if !oc.is_default("junctions.small-radius") {
                write_warning("option 'default.junctions.radius' is smaller than option 'junctions.small-radius'");
            } else {
                let value = oc.get_value_string("default.junctions.radius").unwrap();
                oc.set_default("junctions.small-radius", &value).unwrap();
            }
        }
        let layout = oc.get_string("tls.layout").unwrap();
        if layout != "opposites" && layout != "incoming" && layout != "alternateOneWay" {
            write_error("tls.layout must be 'opposites', 'incoming' or 'alternateOneWay'");
            ok = false;
        }
        if !oc.is_default("default.right-of-way") && !RightOfWayValues.has_string(&oc.get_string("default.right-of-way").unwrap()) {
            write_errorf!("default.right-of-way must be one of '%'", join_to_string(&RightOfWayValues.get_strings(), " "));
            ok = false;
        }
        if oc.get_float("roundabouts.visibility-distance").unwrap() < 0. && oc.get_float("roundabouts.visibility-distance").unwrap() != NBEDGE_UNSPECIFIED_VISIBILITY_DISTANCE {
            write_error("roundabouts.visibility-distance must be positive or -1");
            ok = false;
        }
        if oc.is_default("railway.topology.repair") && oc.get_bool("railway.topology.repair.connect-straight").unwrap() {
            oc.set_default("railway.topology.repair", "true").unwrap();
        }
        if oc.is_default("railway.topology.repair") && oc.get_bool("railway.topology.repair.minimal").unwrap() {
            oc.set_default("railway.topology.repair", "true").unwrap();
        }
        if oc.is_default("railway.topology.all-bidi") && !oc.is_default("railway.topology.all-bidi.input-file") {
            oc.set_default("railway.topology.all-bidi", "true").unwrap();
        }
        // restore legacy behavior after making the option effective during general import
        if oc.is_default("railway.topology.repair.stop-turn") && !oc.is_default("railway.topology.repair") && oc.exists("sumo-net-file") && oc.is_set("sumo-net-file", true).unwrap() {
            oc.set_default("railway.topology.repair.stop-turn", "true").unwrap();
        }
        if !oc.is_default("railway.signal.permit-unsignalized") {
            match parse_vehicle_classes_vec(&oc.get_string_vector("railway.signal.permit-unsignalized").unwrap().clone()) {
                Ok(_) => {}
                Err(ProcessError::InvalidArgument(msg)) => {
                    write_error(&msg);
                    ok = false;
                }
                Err(other) => panic!("{other}"),
            }
        }
        if !LaneSpreadFunctions.has_string(&oc.get_string("default.spreadtype").unwrap()) {
            write_errorf!("Unknown value for default.spreadtype '%'.", oc.get_string("default.spreadtype").unwrap());
            ok = false;
        }
        // check whether the junction type to use is properly set
        if oc.is_set("default.junctions.type", true).unwrap() {
            let node_type = oc.get_string("default.junctions.type").unwrap();
            let known = [
                SumoXMLNodeType::TRAFFIC_LIGHT,
                SumoXMLNodeType::TRAFFIC_LIGHT_NOJUNCTION,
                SumoXMLNodeType::TRAFFIC_LIGHT_RIGHT_ON_RED,
                SumoXMLNodeType::PRIORITY,
                SumoXMLNodeType::PRIORITY_STOP,
                SumoXMLNodeType::ALLWAY_STOP,
                SumoXMLNodeType::ZIPPER,
                SumoXMLNodeType::NOJUNCTION,
                SumoXMLNodeType::RAIL_SIGNAL,
                SumoXMLNodeType::RAIL_CROSSING,
                SumoXMLNodeType::LEFT_BEFORE_RIGHT,
                SumoXMLNodeType::RIGHT_BEFORE_LEFT,
                SumoXMLNodeType::NOJUNCTION,
                SumoXMLNodeType::UNKNOWN,
            ];
            if known.iter().all(|t| node_type != NodeTypes.get_string(*t)) {
                let names = [
                    SumoXMLNodeType::TRAFFIC_LIGHT,
                    SumoXMLNodeType::TRAFFIC_LIGHT_NOJUNCTION,
                    SumoXMLNodeType::TRAFFIC_LIGHT_RIGHT_ON_RED,
                    SumoXMLNodeType::PRIORITY,
                    SumoXMLNodeType::PRIORITY_STOP,
                    SumoXMLNodeType::ALLWAY_STOP,
                    SumoXMLNodeType::ZIPPER,
                    SumoXMLNodeType::NOJUNCTION,
                    SumoXMLNodeType::RAIL_SIGNAL,
                    SumoXMLNodeType::RAIL_CROSSING,
                    SumoXMLNodeType::LEFT_BEFORE_RIGHT,
                    SumoXMLNodeType::RIGHT_BEFORE_LEFT,
                    SumoXMLNodeType::UNKNOWN,
                ];
                let list: Vec<&str> = names.iter().map(|t| NodeTypes.get_string(*t)).collect();
                write_error(&format!("Only the following junction types are known: {}", list.join(", ")));
                ok = false;
            }
        }
        ok
    }

}

