//! Port of netgen/NGFrame.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::msg_handler::{write_error, write_warning};
use utils::options::options_cont::with_options;

/// Sets and checks options for netgen
/// C++ class `NGFrame` (netgen/NGFrame.h)
pub struct NGFrame {
}

impl NGFrame {
    /// C++: `void NGFrame::fillOptions()` [static]
    /// Inserts options used by the network generator
    // void
    // NGFrame::fillOptions() {
    //     OptionsCont& oc = OptionsCont::getOptions();
    //     oc.doRegister("type-files", 't', new Option_FileName());
    //     oc.addDescription("type-files", "Input", TL("Read edge-type defs from FILE"));
    //
    //     oc.doRegister("alphanumerical-ids", new Option_Bool(true));
    //     oc.addDescription("alphanumerical-ids", "Output", TL("The Ids of generated nodes use an alphanumerical code for easier readability when possible"));
    //
    //     // register processing options
    //     oc.doRegister("turn-lanes", new Option_Integer(0));
    //     oc.addDescription("turn-lanes", "Processing", TL("Generate INT left-turn lanes"));
    //     oc.doRegister("turn-lanes.length", new Option_Float(20));
    //     oc.addDescription("turn-lanes.length", "Processing", TL("Set the length of generated turning lanes to FLOAT"));
    //
    //     oc.doRegister("perturb-x", new Option_String("0"));
    //     oc.addDescription("perturb-x", "Processing", TL("Apply random spatial perturbation in x direction according to the given distribution"));
    //     oc.doRegister("perturb-y", new Option_String("0"));
    //     oc.addDescription("perturb-y", "Processing", TL("Apply random spatial perturbation in y direction according to the given distribution"));
    //     oc.doRegister("perturb-z", new Option_String("0"));
    //     oc.addDescription("perturb-z", "Processing", TL("Apply random spatial perturbation in z direction according to the given distribution"));
    //
    //     oc.doRegister("bidi-probability", new Option_Float(1));
    //     oc.addSynonyme("bidi-probability", "rand-bidi-probability", true);
    //     oc.addSynonyme("bidi-probability", "rand.bidi-probability");
    //     oc.addSynonyme("bidi-probability", "bidi");
    //     oc.addDescription("bidi-probability", "Processing", TL("Defines the probability to build a reverse edge"));
    //
    //     oc.doRegister("random-lanenumber", new Option_Bool(false));
    //     oc.addSynonyme("random-lanenumber", "rand.random-lanenumber", false);
    //     oc.addDescription("random-lanenumber", "Processing", TL("Draw lane numbers randomly from [1,default.lanenumber]"));
    //
    //     oc.doRegister("random-priority", new Option_Bool(false));
    //     oc.addSynonyme("random-priority", "rand.random-priority", false);
    //     oc.addDescription("random-priority", "Processing", TL("Draw edge priority randomly from [1,default.priority]"));
    //
    //     oc.doRegister("random-type", new Option_Bool(false));
    //     oc.addDescription("random-type", "Processing", TL("Draw edge type randomly from all loaded types"));
    //
    //
    //     //  register grid-net options
    //     oc.doRegister("grid", 'g', new Option_Bool(false));
    //     oc.addSynonyme("grid", "grid-net", true);
    //     oc.addDescription("grid", "Grid Network", TL("Forces NETGEN to build a grid-like network"));
    //
    //     oc.doRegister("grid.number", new Option_Integer(5));
    //     oc.addSynonyme("grid.number", "grid-number", true);
    //     oc.addSynonyme("grid.number", "number");
    //     oc.addDescription("grid.number", "Grid Network", TL("The number of junctions in both dirs"));
    //
    //     oc.doRegister("grid.length", new Option_Float(100));
    //     oc.addSynonyme("grid.length", "grid-length", true);
    //     oc.addSynonyme("grid.length", "length");
    //     oc.addDescription("grid.length", "Grid Network", TL("The length of streets in both dirs"));
    //
    //     oc.doRegister("grid.x-number", new Option_Integer(5));
    //     oc.addSynonyme("grid.x-number", "grid-x-number", true);
    //     oc.addSynonyme("grid.x-number", "x-no");
    //     oc.addDescription("grid.x-number", "Grid Network", TL("The number of junctions in x-dir; Overrides --grid-number"));
    //
    //     oc.doRegister("grid.y-number", new Option_Integer(5));
    //     oc.addSynonyme("grid.y-number", "grid-y-number", true);
    //     oc.addSynonyme("grid.y-number", "y-no");
    //     oc.addDescription("grid.y-number", "Grid Network", TL("The number of junctions in y-dir; Overrides --grid-number"));
    //
    //     oc.doRegister("grid.x-length", new Option_Float(100));
    //     oc.addSynonyme("grid.x-length", "grid-x-length", true);
    //     oc.addSynonyme("grid.x-length", "x-length");
    //     oc.addDescription("grid.x-length", "Grid Network", TL("The length of horizontal streets; Overrides --grid-length"));
    //
    //     oc.doRegister("grid.y-length", new Option_Float(100));
    //     oc.addSynonyme("grid.y-length", "grid-y-length", true);
    //     oc.addSynonyme("grid.y-length", "y-length");
    //     oc.addDescription("grid.y-length", "Grid Network", TL("The length of vertical streets; Overrides --grid-length"));
    //
    //     oc.doRegister("grid.attach-length", new Option_Float(0));
    //     oc.addSynonyme("grid.attach-length", "attach-length", true);
    //     oc.addDescription("grid.attach-length", "Grid Network", TL("The length of streets attached at the boundary; 0 means no streets are attached"));
    //
    //     oc.doRegister("grid.x-attach-length", new Option_Float(0));
    //     oc.addDescription("grid.x-attach-length", "Grid Network", TL("The length of streets attached at the boundary in x direction; 0 means no streets are attached"));
    //     oc.doRegister("grid.y-attach-length", new Option_Float(0));
    //     oc.addDescription("grid.y-attach-length", "Grid Network", TL("The length of streets attached at the boundary in y direction; 0 means no streets are attached"));
    //
    //     //  register spider-net options
    //     oc.doRegister("spider", 's', new Option_Bool(false));
    //     oc.addSynonyme("spider", "spider-net", true);
    //     oc.addDescription("spider", "Spider Network", TL("Forces NETGEN to build a spider-net-like network"));
    //
    //     oc.doRegister("spider.arm-number", new Option_Integer(7));
    //     oc.addSynonyme("spider.arm-number", "spider-arm-number", true);
    //     oc.addSynonyme("spider.arm-number", "arms");
    //     oc.addDescription("spider.arm-number", "Spider Network", TL("The number of axes within the net"));
    //
    //     oc.doRegister("spider.circle-number", new Option_Integer(5));
    //     oc.addSynonyme("spider.circle-number", "spider-circle-number", true);
    //     oc.addSynonyme("spider.circle-number", "circles");
    //     oc.addDescription("spider.circle-number", "Spider Network", TL("The number of circles of the net"));
    //
    //     oc.doRegister("spider.space-radius", new Option_Float(100));
    //     oc.addSynonyme("spider.space-radius", "spider-space-rad", true);
    //     oc.addSynonyme("spider.space-radius", "radius");
    //     oc.addDescription("spider.space-radius", "Spider Network", TL("The distances between the circles"));
    //
    //     oc.doRegister("spider.omit-center", new Option_Bool(false));
    //     oc.addSynonyme("spider.omit-center", "spider-omit-center", true);
    //     oc.addSynonyme("spider.omit-center", "nocenter");
    //     oc.addDescription("spider.omit-center", "Spider Network", TL("Omit the central node of the network"));
    //
    //     oc.doRegister("spider.attach-length", new Option_Float(0));
    //     oc.addDescription("spider.attach-length", "Spider Network", TL("The length of streets attached at the boundary; 0 means no streets are attached"));
    //
    //     //  register random-net options
    //     oc.doRegister("rand", 'r', new Option_Bool(false));
    //     oc.addSynonyme("rand", "random-net", true);
    //     oc.addDescription("rand", "Random Network", TL("Forces NETGEN to build a random network"));
    //
    //     oc.doRegister("rand.iterations", new Option_Integer(100));
    //     oc.addSynonyme("rand.iterations", "rand-iterations", true);
    //     oc.addSynonyme("rand.iterations", "iterations");
    //     oc.addDescription("rand.iterations", "Random Network", TL("Describes how many times an edge shall be added to the net"));
    //
    //     oc.doRegister("rand.max-distance", new Option_Float(250));
    //     oc.addSynonyme("rand.max-distance", "rand-max-distance", true);
    //     oc.addSynonyme("rand.max-distance", "max-dist");
    //     oc.addDescription("rand.max-distance", "Random Network", TL("The maximum distance for each edge"));
    //
    //     oc.doRegister("rand.min-distance", new Option_Float(100));
    //     oc.addSynonyme("rand.min-distance", "rand-min-distance", true);
    //     oc.addSynonyme("rand.min-distance", "min-dist");
    //     oc.addDescription("rand.min-distance", "Random Network", TL("The minimum distance for each edge"));
    //
    //     oc.doRegister("rand.min-angle", new Option_Float(45.0));
    //     oc.addSynonyme("rand.min-angle", "rand-min-anglee", true);
    //     oc.addSynonyme("rand.min-angle", "min-angle");
    //     oc.addDescription("rand.min-angle", "Random Network", TL("The minimum angle for each pair of (bidirectional) roads in DEGREES"));
    //
    //     oc.doRegister("rand.num-tries", new Option_Integer(50));
    //     oc.addSynonyme("rand.num-tries", "rand-num-tries", true);
    //     oc.addSynonyme("rand.num-tries", "num-tries");
    //     oc.addDescription("rand.num-tries", "Random Network", TL("The number of tries for creating each node"));
    //
    //     oc.doRegister("rand.connectivity", new Option_Float((double) 0.95));
    //     oc.addSynonyme("rand.connectivity", "rand-connectivity", true);
    //     oc.addSynonyme("rand.connectivity", "connectivity");
    //     oc.addDescription("rand.connectivity", "Random Network", TL("Probability for roads to continue at each node"));
    //
    //     oc.doRegister("rand.neighbor-dist1", new Option_Float(0));
    //     oc.addSynonyme("rand.neighbor-dist1", "rand-neighbor-dist1", true);
    //     oc.addSynonyme("rand.neighbor-dist1", "dist1");
    //     oc.addDescription("rand.neighbor-dist1", "Random Network", TL("Probability for a node having at most 1 neighbor"));
    //
    //     oc.doRegister("rand.neighbor-dist2", new Option_Float(0));
    //     oc.addSynonyme("rand.neighbor-dist2", "rand-neighbor-dist2", true);
    //     oc.addSynonyme("rand.neighbor-dist2", "dist2");
    //     oc.addDescription("rand.neighbor-dist2", "Random Network", TL("Probability for a node having at most 2 neighbors"));
    //
    //     oc.doRegister("rand.neighbor-dist3", new Option_Float(10));
    //     oc.addSynonyme("rand.neighbor-dist3", "rand-neighbor-dist3", true);
    //     oc.addSynonyme("rand.neighbor-dist3", "dist3");
    //     oc.addDescription("rand.neighbor-dist3", "Random Network", TL("Probability for a node having at most 3 neighbors"));
    //
    //     oc.doRegister("rand.neighbor-dist4", new Option_Float(10));
    //     oc.addSynonyme("rand.neighbor-dist4", "rand-neighbor-dist4", true);
    //     oc.addSynonyme("rand.neighbor-dist4", "dist4");
    //     oc.addDescription("rand.neighbor-dist4", "Random Network", TL("Probability for a node having at most 4 neighbors"));
    //
    //     oc.doRegister("rand.neighbor-dist5", new Option_Float(2));
    //     oc.addSynonyme("rand.neighbor-dist5", "rand-neighbor-dist5", true);
    //     oc.addSynonyme("rand.neighbor-dist5", "dist5");
    //     oc.addDescription("rand.neighbor-dist5", "Random Network", TL("Probability for a node having at most 5 neighbors"));
    //
    //     oc.doRegister("rand.neighbor-dist6", new Option_Float(1));
    //     oc.addSynonyme("rand.neighbor-dist6", "rand-neighbor-dist6", true);
    //     oc.addSynonyme("rand.neighbor-dist6", "dist6");
    //     oc.addDescription("rand.neighbor-dist6", "Random Network", TL("Probability for a node having at most 6 neighbors"));
    //
    //     oc.doRegister("rand.grid", new Option_Bool(false));
    //     oc.addDescription("rand.grid", "Random Network", TL("Place nodes on a regular grid with spacing rand.min-distance"));
    // }
    #[allow(unused_variables)]
    pub fn fill_options() {
        with_options(|oc| {
            oc.do_register("type-files", SumoOption::file_name());
            oc.add_description("type-files", "Input", "Read edge-type defs from FILE");

            oc.do_register("alphanumerical-ids", SumoOption::bool(true));
            oc.add_description("alphanumerical-ids", "Output", "The Ids of generated nodes use an alphanumerical code for easier readability when possible");

            // register processing options
            oc.do_register("turn-lanes", SumoOption::integer(0));
            oc.add_description("turn-lanes", "Processing", "Generate INT left-turn lanes");
            oc.do_register("turn-lanes.length", SumoOption::float(20.0));
            oc.add_description("turn-lanes.length", "Processing", "Set the length of generated turning lanes to FLOAT");

            oc.do_register("perturb-x", SumoOption::string_with("0"));
            oc.add_description("perturb-x", "Processing", "Apply random spatial perturbation in x direction according to the given distribution");
            oc.do_register("perturb-y", SumoOption::string_with("0"));
            oc.add_description("perturb-y", "Processing", "Apply random spatial perturbation in y direction according to the given distribution");
            oc.do_register("perturb-z", SumoOption::string_with("0"));
            oc.add_description("perturb-z", "Processing", "Apply random spatial perturbation in z direction according to the given distribution");

            oc.do_register("bidi-probability", SumoOption::float(1.0));
            oc.add_synonyme("bidi-probability", "rand-bidi-probability", true);
            oc.add_synonyme("bidi-probability", "rand.bidi-probability", false);
            oc.add_synonyme("bidi-probability", "bidi", false);
            oc.add_description("bidi-probability", "Processing", "Defines the probability to build a reverse edge");

            oc.do_register("random-lanenumber", SumoOption::bool(false));
            oc.add_synonyme("random-lanenumber", "rand.random-lanenumber", false);
            oc.add_description("random-lanenumber", "Processing", "Draw lane numbers randomly from [1,default.lanenumber]");

            oc.do_register("random-priority", SumoOption::bool(false));
            oc.add_synonyme("random-priority", "rand.random-priority", false);
            oc.add_description("random-priority", "Processing", "Draw edge priority randomly from [1,default.priority]");

            oc.do_register("random-type", SumoOption::bool(false));
            oc.add_description("random-type", "Processing", "Draw edge type randomly from all loaded types");

            //  register grid-net options
            oc.do_register_abbr("grid", 'g', SumoOption::bool(false));
            oc.add_synonyme("grid", "grid-net", true);
            oc.add_description("grid", "Grid Network", "Forces NETGEN to build a grid-like network");

            oc.do_register("grid.number", SumoOption::integer(5));
            oc.add_synonyme("grid.number", "grid-number", true);
            oc.add_synonyme("grid.number", "number", false);
            oc.add_description("grid.number", "Grid Network", "The number of junctions in both dirs");

            oc.do_register("grid.length", SumoOption::float(100.0));
            oc.add_synonyme("grid.length", "grid-length", true);
            oc.add_synonyme("grid.length", "length", false);
            oc.add_description("grid.length", "Grid Network", "The length of streets in both dirs");

            oc.do_register("grid.x-number", SumoOption::integer(5));
            oc.add_synonyme("grid.x-number", "grid-x-number", true);
            oc.add_synonyme("grid.x-number", "x-no", false);
            oc.add_description("grid.x-number", "Grid Network", "The number of junctions in x-dir; Overrides --grid-number");

            oc.do_register("grid.y-number", SumoOption::integer(5));
            oc.add_synonyme("grid.y-number", "grid-y-number", true);
            oc.add_synonyme("grid.y-number", "y-no", false);
            oc.add_description("grid.y-number", "Grid Network", "The number of junctions in y-dir; Overrides --grid-number");

            oc.do_register("grid.x-length", SumoOption::float(100.0));
            oc.add_synonyme("grid.x-length", "grid-x-length", true);
            oc.add_synonyme("grid.x-length", "x-length", false);
            oc.add_description("grid.x-length", "Grid Network", "The length of horizontal streets; Overrides --grid-length");

            oc.do_register("grid.y-length", SumoOption::float(100.0));
            oc.add_synonyme("grid.y-length", "grid-y-length", true);
            oc.add_synonyme("grid.y-length", "y-length", false);
            oc.add_description("grid.y-length", "Grid Network", "The length of vertical streets; Overrides --grid-length");

            oc.do_register("grid.attach-length", SumoOption::float(0.0));
            oc.add_synonyme("grid.attach-length", "attach-length", true);
            oc.add_description("grid.attach-length", "Grid Network", "The length of streets attached at the boundary; 0 means no streets are attached");

            oc.do_register("grid.x-attach-length", SumoOption::float(0.0));
            oc.add_description("grid.x-attach-length", "Grid Network", "The length of streets attached at the boundary in x direction; 0 means no streets are attached");
            oc.do_register("grid.y-attach-length", SumoOption::float(0.0));
            oc.add_description("grid.y-attach-length", "Grid Network", "The length of streets attached at the boundary in y direction; 0 means no streets are attached");

            //  register spider-net options
            oc.do_register_abbr("spider", 's', SumoOption::bool(false));
            oc.add_synonyme("spider", "spider-net", true);
            oc.add_description("spider", "Spider Network", "Forces NETGEN to build a spider-net-like network");

            oc.do_register("spider.arm-number", SumoOption::integer(7));
            oc.add_synonyme("spider.arm-number", "spider-arm-number", true);
            oc.add_synonyme("spider.arm-number", "arms", false);
            oc.add_description("spider.arm-number", "Spider Network", "The number of axes within the net");

            oc.do_register("spider.circle-number", SumoOption::integer(5));
            oc.add_synonyme("spider.circle-number", "spider-circle-number", true);
            oc.add_synonyme("spider.circle-number", "circles", false);
            oc.add_description("spider.circle-number", "Spider Network", "The number of circles of the net");

            oc.do_register("spider.space-radius", SumoOption::float(100.0));
            oc.add_synonyme("spider.space-radius", "spider-space-rad", true);
            oc.add_synonyme("spider.space-radius", "radius", false);
            oc.add_description("spider.space-radius", "Spider Network", "The distances between the circles");

            oc.do_register("spider.omit-center", SumoOption::bool(false));
            oc.add_synonyme("spider.omit-center", "spider-omit-center", true);
            oc.add_synonyme("spider.omit-center", "nocenter", false);
            oc.add_description("spider.omit-center", "Spider Network", "Omit the central node of the network");

            oc.do_register("spider.attach-length", SumoOption::float(0.0));
            oc.add_description("spider.attach-length", "Spider Network", "The length of streets attached at the boundary; 0 means no streets are attached");

            //  register random-net options
            oc.do_register_abbr("rand", 'r', SumoOption::bool(false));
            oc.add_synonyme("rand", "random-net", true);
            oc.add_description("rand", "Random Network", "Forces NETGEN to build a random network");

            oc.do_register("rand.iterations", SumoOption::integer(100));
            oc.add_synonyme("rand.iterations", "rand-iterations", true);
            oc.add_synonyme("rand.iterations", "iterations", false);
            oc.add_description("rand.iterations", "Random Network", "Describes how many times an edge shall be added to the net");

            oc.do_register("rand.max-distance", SumoOption::float(250.0));
            oc.add_synonyme("rand.max-distance", "rand-max-distance", true);
            oc.add_synonyme("rand.max-distance", "max-dist", false);
            oc.add_description("rand.max-distance", "Random Network", "The maximum distance for each edge");

            oc.do_register("rand.min-distance", SumoOption::float(100.0));
            oc.add_synonyme("rand.min-distance", "rand-min-distance", true);
            oc.add_synonyme("rand.min-distance", "min-dist", false);
            oc.add_description("rand.min-distance", "Random Network", "The minimum distance for each edge");

            oc.do_register("rand.min-angle", SumoOption::float(45.0));
            oc.add_synonyme("rand.min-angle", "rand-min-anglee", true);
            oc.add_synonyme("rand.min-angle", "min-angle", false);
            oc.add_description("rand.min-angle", "Random Network", "The minimum angle for each pair of (bidirectional) roads in DEGREES");

            oc.do_register("rand.num-tries", SumoOption::integer(50));
            oc.add_synonyme("rand.num-tries", "rand-num-tries", true);
            oc.add_synonyme("rand.num-tries", "num-tries", false);
            oc.add_description("rand.num-tries", "Random Network", "The number of tries for creating each node");

            oc.do_register("rand.connectivity", SumoOption::float(0.95));
            oc.add_synonyme("rand.connectivity", "rand-connectivity", true);
            oc.add_synonyme("rand.connectivity", "connectivity", false);
            oc.add_description("rand.connectivity", "Random Network", "Probability for roads to continue at each node");

            oc.do_register("rand.neighbor-dist1", SumoOption::float(0.0));
            oc.add_synonyme("rand.neighbor-dist1", "rand-neighbor-dist1", true);
            oc.add_synonyme("rand.neighbor-dist1", "dist1", false);
            oc.add_description("rand.neighbor-dist1", "Random Network", "Probability for a node having at most 1 neighbor");

            oc.do_register("rand.neighbor-dist2", SumoOption::float(0.0));
            oc.add_synonyme("rand.neighbor-dist2", "rand-neighbor-dist2", true);
            oc.add_synonyme("rand.neighbor-dist2", "dist2", false);
            oc.add_description("rand.neighbor-dist2", "Random Network", "Probability for a node having at most 2 neighbors");

            oc.do_register("rand.neighbor-dist3", SumoOption::float(10.0));
            oc.add_synonyme("rand.neighbor-dist3", "rand-neighbor-dist3", true);
            oc.add_synonyme("rand.neighbor-dist3", "dist3", false);
            oc.add_description("rand.neighbor-dist3", "Random Network", "Probability for a node having at most 3 neighbors");

            oc.do_register("rand.neighbor-dist4", SumoOption::float(10.0));
            oc.add_synonyme("rand.neighbor-dist4", "rand-neighbor-dist4", true);
            oc.add_synonyme("rand.neighbor-dist4", "dist4", false);
            oc.add_description("rand.neighbor-dist4", "Random Network", "Probability for a node having at most 4 neighbors");

            oc.do_register("rand.neighbor-dist5", SumoOption::float(2.0));
            oc.add_synonyme("rand.neighbor-dist5", "rand-neighbor-dist5", true);
            oc.add_synonyme("rand.neighbor-dist5", "dist5", false);
            oc.add_description("rand.neighbor-dist5", "Random Network", "Probability for a node having at most 5 neighbors");

            oc.do_register("rand.neighbor-dist6", SumoOption::float(1.0));
            oc.add_synonyme("rand.neighbor-dist6", "rand-neighbor-dist6", true);
            oc.add_synonyme("rand.neighbor-dist6", "dist6", false);
            oc.add_description("rand.neighbor-dist6", "Random Network", "Probability for a node having at most 6 neighbors");

            oc.do_register("rand.grid", SumoOption::bool(false));
            oc.add_description("rand.grid", "Random Network", "Place nodes on a regular grid with spacing rand.min-distance");
        });
    }

    /// C++: `bool NGFrame::checkOptions()` [static]
    /// Checks set options from the OptionsCont-singleton for being valid
    // bool
    // NGFrame::checkOptions() {
    //     OptionsCont& oc = OptionsCont::getOptions();
    //     bool ok = true;
    //     // check whether exactly one type of a network to build was wished
    //     int no = 0;
    //     if (oc.getBool("spider")) {
    //         no++;
    //     }
    //     if (oc.getBool("grid")) {
    //         no++;
    //     }
    //     if (oc.getBool("rand")) {
    //         no++;
    //     }
    //     if (no == 0) {
    //         WRITE_ERROR(TL("You have to specify the type of network to generate."));
    //         ok = false;
    //     }
    //     if (no > 1) {
    //         WRITE_ERROR(TL("You may specify only one type of network to generate at once."));
    //         ok = false;
    //     }
    //     if (oc.getBool("random-type") && !oc.isSet("type-files")) {
    //         WRITE_WARNING(TL("Option 'random-type' takes no effect unless 'type-files' are loaded"));
    //     }
    //     if (oc.getFloat("rand.connectivity") > 0.999) {
    //         WRITE_ERROR(TL("Option 'rand.connectivity' requires a value below 0.999"));
    //         ok = false;
    //     }
    //     return ok;
    // }
    #[allow(unused_variables)]
    pub fn check_options() -> bool {
        with_options(|oc| {
            let mut ok = true;
            // check whether exactly one type of a network to build was wished
            let mut no = 0;
            if oc.get_bool("spider").unwrap() {
                no += 1;
            }
            if oc.get_bool("grid").unwrap() {
                no += 1;
            }
            if oc.get_bool("rand").unwrap() {
                no += 1;
            }
            if no == 0 {
                write_error("You have to specify the type of network to generate.");
                ok = false;
            }
            if no > 1 {
                write_error("You may specify only one type of network to generate at once.");
                ok = false;
            }
            if oc.get_bool("random-type").unwrap() && !oc.is_set("type-files", true).unwrap() {
                write_warning("Option 'random-type' takes no effect unless 'type-files' are loaded");
            }
            if oc.get_float("rand.connectivity").unwrap() > 0.999 {
                write_error("Option 'rand.connectivity' requires a value below 0.999");
                ok = false;
            }
            ok
        })
    }

}

