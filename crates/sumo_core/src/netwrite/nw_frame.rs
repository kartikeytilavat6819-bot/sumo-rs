//! Port of netwrite/NWFrame.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::msg_handler::{progress_begin_message, progress_time_message, write_error, write_warning};

/// C++ `OptionsCont` getters/setters throw ProcessError; the options are required to exist here
fn or_panic<T, E: std::fmt::Display>(r: Result<T, E>) -> T {
    r.unwrap_or_else(|e| panic!("{e}"))
}

/// Sets and checks options for netwrite; saves the network
/// C++ class `NWFrame` (netwrite/NWFrame.h)
pub struct NWFrame {
}

impl NWFrame {
    /// C++: `void NWFrame::fillOptions(OptionsCont &, bool)` [static]
    /// Inserts options used by the network writer
    // void
    // NWFrame::fillOptions(OptionsCont& oc, bool forNetgen) {
    //     // register options
    //     oc.doRegister("output-file", 'o', new Option_FileName());
    //     oc.addSynonyme("output-file", "sumo-output");
    //     oc.addSynonyme("output-file", "output");
    //     oc.addDescription("output-file", "Output", TL("The generated net will be written to FILE"));
    //
    //     oc.doRegister("plain-output-prefix", 'p', new Option_FileName());
    //     oc.addSynonyme("plain-output-prefix", "plain-output");
    //     oc.addSynonyme("plain-output-prefix", "plain");
    //     oc.addDescription("plain-output-prefix", "Output", TL("Prefix of files to write plain xml nodes, edges and connections to"));
    //
    //     oc.doRegister("plain-output.lanes", new Option_Bool(false));
    //     oc.addDescription("plain-output.lanes", "Output", TL("Write all lanes and their attributes even when they are not customized"));
    //
    //     oc.doRegister("junctions.join-output", new Option_FileName());
    //     oc.addDescription("junctions.join-output", "Output",
    //                       "Writes information about joined junctions to FILE (can be loaded as additional node-file to reproduce joins");
    //
    //     oc.doRegister("prefix", new Option_String(""));
    //     oc.addDescription("prefix", "Output", TL("Defines a prefix for edge and junction IDs"));
    //
    //     oc.doRegister("prefix.junction", new Option_String(""));
    //     oc.addDescription("prefix.junction", "Output", TL("Defines a prefix for junction IDs"));
    //
    //     oc.doRegister("prefix.edge", new Option_String(""));
    //     oc.addDescription("prefix.edge", "Output", TL("Defines a prefix for edge IDs"));
    //
    // #ifdef PROJ_API_FILE
    //     if (!forNetgen) {
    //         oc.doRegister("proj.plain-geo", new Option_Bool(false));
    //         oc.addDescription("proj.plain-geo", "Projection", TL("Write geo coordinates in plain-xml"));
    //     }
    // #endif // PROJ_API_FILE
    //
    //     oc.doRegister("amitran-output", new Option_FileName());
    //     oc.addDescription("amitran-output", "Output", TL("The generated net will be written to FILE using Amitran format"));
    //
    //     oc.doRegister("matsim-output", new Option_FileName());
    //     oc.addDescription("matsim-output", "Output", TL("The generated net will be written to FILE using MATSim format"));
    //
    //     oc.doRegister("opendrive-output", new Option_FileName());
    //     oc.addDescription("opendrive-output", "Output", TL("The generated net will be written to FILE using OpenDRIVE format"));
    //
    //     oc.doRegister("dlr-navteq-output", new Option_FileName());
    //     oc.addDescription("dlr-navteq-output", "Output", TL("The generated net will be written to dlr-navteq files with the given PREFIX"));
    //
    //     oc.doRegister("dlr-navteq.version", new Option_String("6.5"));
    //     oc.addDescription("dlr-navteq.version", "Output", TL("The dlr-navteq output format version to write"));
    //
    //     oc.doRegister("dlr-navteq.precision", new Option_Integer(2));
    //     oc.addDescription("dlr-navteq.precision", "Output", TL("The network coordinates are written with the specified level of output precision"));
    //
    //     oc.doRegister("dlr-navteq.plain", new Option_Bool(false));
    //     oc.addDescription("dlr-navteq.plain", "Output", TL("Write extra attributes into plain output"));
    //
    //     oc.doRegister("output.street-names", new Option_Bool(false));
    //     oc.addDescription("output.street-names", "Output", TL("Street names will be included in the output (if available)"));
    //
    //     oc.doRegister("output.original-names", new Option_Bool(false));
    //     oc.addDescription("output.original-names", "Output", TL("Writes original names, if given, as parameter"));
    //
    //     oc.doRegister("output.removed-nodes", new Option_Bool(false));
    //     oc.addDescription("output.removed-nodes", "Output", TL("Writes IDs of nodes remove with --geometry.remove into edge param"));
    //
    //     oc.doRegister("street-sign-output", new Option_FileName());
    //     oc.addDescription("street-sign-output", "Output", TL("Writes street signs as POIs to FILE"));
    //
    //     if (!forNetgen) {
    //         oc.doRegister("ptstop-output", new Option_FileName());
    //         oc.addDescription("ptstop-output", "Output", TL("Writes public transport stops to FILE"));
    //         oc.doRegister("ptline-output", new Option_FileName());
    //         oc.addDescription("ptline-output", "Output", TL("Writes public transport lines to FILE"));
    //         oc.doRegister("ptline-clean-up", new Option_Bool(false));
    //         oc.addDescription("ptline-clean-up", "Output", TL("Clean-up pt stops that are not served by any line"));
    //
    //         oc.doRegister("parking-output", new Option_FileName());
    //         oc.addDescription("parking-output", "Output", TL("Writes parking areas to FILE"));
    //
    //         oc.doRegister("railway.topology.output", new Option_FileName());
    //         oc.addDescription("railway.topology.output", "Output", TL("Analyze topology of the railway network"));
    //
    //         oc.doRegister("polygon-output", new Option_FileName());
    //         oc.addSynonyme("polygon-output", "taz-output");
    //         oc.addDescription("polygon-output", "Output", TL("Write shapes that are embedded in the network input and that are not supported by polyconvert (OpenDRIVE)"));
    //     }
    //
    //     // register opendrive options
    //     oc.doRegister("opendrive-output.straight-threshold", new Option_Float(0.00000001)); // matching the angular output precision in NWWriter_OpenDrive
    //     oc.addDescription("opendrive-output.straight-threshold", "Output", TL("Builds parameterized curves whenever the angular change between straight segments exceeds FLOAT degrees"));
    //
    //     if (!forNetgen) {
    //         oc.doRegister("opendrive-output.lefthand-left", new Option_Bool(false));
    //         oc.addDescription("opendrive-output.lefthand-left", "Output", TL("Write lanes in lefthand networks on the left side (positive indices)"));
    //
    //         oc.doRegister("opendrive-output.shape-match-dist", new Option_Float(-1));
    //         oc.addDescription("opendrive-output.shape-match-dist", "Output", TL("Match loaded shapes to the closest edge within FLOAT and export as road objects"));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn fill_options(oc: &mut OptionsCont, forNetgen: bool) {
        // register options
        oc.do_register_abbr("output-file", 'o', SumoOption::file_name());
        oc.add_synonyme("output-file", "sumo-output", false);
        oc.add_synonyme("output-file", "output", false);
        oc.add_description("output-file", "Output", "The generated net will be written to FILE");

        oc.do_register_abbr("plain-output-prefix", 'p', SumoOption::file_name());
        oc.add_synonyme("plain-output-prefix", "plain-output", false);
        oc.add_synonyme("plain-output-prefix", "plain", false);
        oc.add_description("plain-output-prefix", "Output", "Prefix of files to write plain xml nodes, edges and connections to");

        oc.do_register("plain-output.lanes", SumoOption::bool(false));
        oc.add_description("plain-output.lanes", "Output", "Write all lanes and their attributes even when they are not customized");

        oc.do_register("junctions.join-output", SumoOption::file_name());
        oc.add_description("junctions.join-output", "Output",
                           "Writes information about joined junctions to FILE (can be loaded as additional node-file to reproduce joins");

        oc.do_register("prefix", SumoOption::string_with(""));
        oc.add_description("prefix", "Output", "Defines a prefix for edge and junction IDs");

        oc.do_register("prefix.junction", SumoOption::string_with(""));
        oc.add_description("prefix.junction", "Output", "Defines a prefix for junction IDs");

        oc.do_register("prefix.edge", SumoOption::string_with(""));
        oc.add_description("prefix.edge", "Output", "Defines a prefix for edge IDs");

        // shortcut: the PROJ_API_FILE branch (proj.plain-geo) is not built in this port, same as ni_frame.rs

        oc.do_register("amitran-output", SumoOption::file_name());
        oc.add_description("amitran-output", "Output", "The generated net will be written to FILE using Amitran format");

        oc.do_register("matsim-output", SumoOption::file_name());
        oc.add_description("matsim-output", "Output", "The generated net will be written to FILE using MATSim format");

        oc.do_register("opendrive-output", SumoOption::file_name());
        oc.add_description("opendrive-output", "Output", "The generated net will be written to FILE using OpenDRIVE format");

        oc.do_register("dlr-navteq-output", SumoOption::file_name());
        oc.add_description("dlr-navteq-output", "Output", "The generated net will be written to dlr-navteq files with the given PREFIX");

        oc.do_register("dlr-navteq.version", SumoOption::string_with("6.5"));
        oc.add_description("dlr-navteq.version", "Output", "The dlr-navteq output format version to write");

        oc.do_register("dlr-navteq.precision", SumoOption::integer(2));
        oc.add_description("dlr-navteq.precision", "Output", "The network coordinates are written with the specified level of output precision");

        oc.do_register("dlr-navteq.plain", SumoOption::bool(false));
        oc.add_description("dlr-navteq.plain", "Output", "Write extra attributes into plain output");

        oc.do_register("output.street-names", SumoOption::bool(false));
        oc.add_description("output.street-names", "Output", "Street names will be included in the output (if available)");

        oc.do_register("output.original-names", SumoOption::bool(false));
        oc.add_description("output.original-names", "Output", "Writes original names, if given, as parameter");

        oc.do_register("output.removed-nodes", SumoOption::bool(false));
        oc.add_description("output.removed-nodes", "Output", "Writes IDs of nodes remove with --geometry.remove into edge param");

        oc.do_register("street-sign-output", SumoOption::file_name());
        oc.add_description("street-sign-output", "Output", "Writes street signs as POIs to FILE");

        if !forNetgen {
            oc.do_register("ptstop-output", SumoOption::file_name());
            oc.add_description("ptstop-output", "Output", "Writes public transport stops to FILE");
            oc.do_register("ptline-output", SumoOption::file_name());
            oc.add_description("ptline-output", "Output", "Writes public transport lines to FILE");
            oc.do_register("ptline-clean-up", SumoOption::bool(false));
            oc.add_description("ptline-clean-up", "Output", "Clean-up pt stops that are not served by any line");

            oc.do_register("parking-output", SumoOption::file_name());
            oc.add_description("parking-output", "Output", "Writes parking areas to FILE");

            oc.do_register("railway.topology.output", SumoOption::file_name());
            oc.add_description("railway.topology.output", "Output", "Analyze topology of the railway network");

            oc.do_register("polygon-output", SumoOption::file_name());
            oc.add_synonyme("polygon-output", "taz-output", false);
            oc.add_description("polygon-output", "Output", "Write shapes that are embedded in the network input and that are not supported by polyconvert (OpenDRIVE)");
        }

        // register opendrive options
        oc.do_register("opendrive-output.straight-threshold", SumoOption::float(0.00000001)); // matching the angular output precision in NWWriter_OpenDrive
        oc.add_description("opendrive-output.straight-threshold", "Output", "Builds parameterized curves whenever the angular change between straight segments exceeds FLOAT degrees");

        if !forNetgen {
            oc.do_register("opendrive-output.lefthand-left", SumoOption::bool(false));
            oc.add_description("opendrive-output.lefthand-left", "Output", "Write lanes in lefthand networks on the left side (positive indices)");

            oc.do_register("opendrive-output.shape-match-dist", SumoOption::float(-1.0));
            oc.add_description("opendrive-output.shape-match-dist", "Output", "Match loaded shapes to the closest edge within FLOAT and export as road objects");
        }
    }

    /// C++: `bool NWFrame::checkOptions(OptionsCont &)` [static]
    /// Checks set options for being valid
    // bool
    // NWFrame::checkOptions(OptionsCont& oc) {
    //     bool ok = true;
    //     // check whether the output is valid and can be build
    //     if (!oc.isSet("output-file")
    //             && !oc.isSet("plain-output-prefix")
    //             && !oc.isSet("amitran-output")
    //             && !oc.isSet("matsim-output")
    //             && !oc.isSet("opendrive-output")
    //             && !oc.isSet("dlr-navteq-output")) {
    //         std::string net = "net.net.xml";
    //         if (oc.isSet("configuration-file")) {
    //             net = FileHelpers::getConfigurationRelative(oc.getString("configuration-file"), net);
    //         }
    //         oc.setDefault("output-file", net);
    //     }
    //     // some outputs need internal lanes
    //     if (oc.isSet("opendrive-output") && oc.getBool("no-internal-links")) {
    //         WRITE_ERROR(TL("OpenDRIVE export needs internal links computation."));
    //         ok = false;
    //     }
    //     if (oc.isSet("opendrive-output") && oc.isDefault("no-internal-links")) {
    //         oc.setDefault("no-internal-links", "false");
    //     }
    //     if (oc.isSet("opendrive-output") && oc.isDefault("rectangular-lane-cut")) {
    //         oc.setDefault("rectangular-lane-cut", "true");
    //     }
    //     if (oc.isSet("opendrive-output") && !oc.getBool("rectangular-lane-cut")) {
    //         WRITE_WARNING(TL("OpenDRIVE cannot represent oblique lane cuts and should use option 'rectangular-lane-cut'."));
    //     }
    //     if (oc.isSet("dlr-navteq-output") && oc.isDefault("osm.all-attributes")) {
    //         oc.setDefault("osm.all-attributes", "true");
    //         oc.setDefault("osm.extra-attributes", "bridge,tunnel,layer,postal_code,maxheight,maxwidth,maxweight,surface");
    //     }
    //     if (oc.exists("ptline-output") && oc.isSet("ptline-output") && !oc.isSet("ptstop-output")) {
    //         WRITE_ERROR(TL("public transport lines output requires 'ptstop-output' to be set"));
    //         ok = false;
    //     }
    //     if (oc.exists("ptline-clean-up") && oc.getBool("ptline-clean-up") && !oc.isSet("ptline-output")) {
    //         WRITE_WARNING(TL("'ptline-clean-up' only works in conjunction with 'ptline-output'. Ignoring invalid option."));
    //     }
    //     if (oc.isDefault("prefix.junction")) {
    //         oc.setDefault("prefix.junction", oc.getString("prefix"));
    //     }
    //     if (oc.isDefault("prefix.edge")) {
    //         oc.setDefault("prefix.edge", oc.getString("prefix"));
    //     }
    //
    //     return ok;
    // }
    #[allow(unused_variables)]
    pub fn check_options(oc: &mut OptionsCont) -> bool {
        let mut ok = true;
        // check whether the output is valid and can be build
        if !or_panic(oc.is_set("output-file", true))
            && !or_panic(oc.is_set("plain-output-prefix", true))
            && !or_panic(oc.is_set("amitran-output", true))
            && !or_panic(oc.is_set("matsim-output", true))
            && !or_panic(oc.is_set("opendrive-output", true))
            && !or_panic(oc.is_set("dlr-navteq-output", true))
        {
            let mut net = String::from("net.net.xml");
            if or_panic(oc.is_set("configuration-file", true)) {
                let config = or_panic(oc.get_string("configuration-file"));
                net = utils::common::file_helpers::get_configuration_relative(&config, &net);
            }
            or_panic(oc.set_default("output-file", &net));
        }
        // some outputs need internal lanes
        if or_panic(oc.is_set("opendrive-output", true)) && or_panic(oc.get_bool("no-internal-links")) {
            write_error("OpenDRIVE export needs internal links computation.");
            ok = false;
        }
        if or_panic(oc.is_set("opendrive-output", true)) && oc.is_default("no-internal-links") {
            or_panic(oc.set_default("no-internal-links", "false"));
        }
        if or_panic(oc.is_set("opendrive-output", true)) && oc.is_default("rectangular-lane-cut") {
            or_panic(oc.set_default("rectangular-lane-cut", "true"));
        }
        if or_panic(oc.is_set("opendrive-output", true)) && !or_panic(oc.get_bool("rectangular-lane-cut")) {
            write_warning("OpenDRIVE cannot represent oblique lane cuts and should use option 'rectangular-lane-cut'.");
        }
        if or_panic(oc.is_set("dlr-navteq-output", true)) && oc.is_default("osm.all-attributes") {
            or_panic(oc.set_default("osm.all-attributes", "true"));
            or_panic(oc.set_default("osm.extra-attributes", "bridge,tunnel,layer,postal_code,maxheight,maxwidth,maxweight,surface"));
        }
        if oc.exists("ptline-output") && or_panic(oc.is_set("ptline-output", true)) && !or_panic(oc.is_set("ptstop-output", true)) {
            write_error("public transport lines output requires 'ptstop-output' to be set");
            ok = false;
        }
        if oc.exists("ptline-clean-up") && or_panic(oc.get_bool("ptline-clean-up")) && !or_panic(oc.is_set("ptline-output", true)) {
            write_warning("'ptline-clean-up' only works in conjunction with 'ptline-output'. Ignoring invalid option.");
        }
        if oc.is_default("prefix.junction") {
            let prefix = or_panic(oc.get_string("prefix"));
            or_panic(oc.set_default("prefix.junction", &prefix));
        }
        if oc.is_default("prefix.edge") {
            let prefix = or_panic(oc.get_string("prefix"));
            or_panic(oc.set_default("prefix.edge", &prefix));
        }

        ok
    }

    /// C++: `void NWFrame::writeNetwork(const OptionsCont &, NBNetBuilder &)` [static]
    /// Writes the network stored in the given net builder
    // void
    // NWFrame::writeNetwork(const OptionsCont& oc, NBNetBuilder& nb) {
    //     const long before = PROGRESS_BEGIN_TIME_MESSAGE(TL("Writing network"));
    //     NWWriter_SUMO::writeNetwork(oc, nb);
    //     NWWriter_Amitran::writeNetwork(oc, nb);
    //     NWWriter_MATSim::writeNetwork(oc, nb);
    //     NWWriter_OpenDrive::writeNetwork(oc, nb);
    //     NWWriter_DlrNavteq::writeNetwork(oc, nb);
    //     NWWriter_XML::writeNetwork(oc, oc.isSet("plain-output-prefix") ? oc.getString("plain-output-prefix") : "", nb);
    //     PROGRESS_TIME_MESSAGE(before);
    // }
    #[allow(unused_variables)]
    pub fn write_network(oc: &OptionsCont, nb: &mut NBNetBuilder) {
        let before = std::time::Instant::now();
        progress_begin_message("Writing network");
        NWWriter_SUMO::write_network(oc, nb);
        NWWriter_Amitran::write_network(oc, nb);
        NWWriter_MATSim::write_network(oc, nb);
        NWWriter_OpenDrive::write_network(oc, nb);
        NWWriter_DlrNavteq::write_network(oc, nb);
        let prefix = if or_panic(oc.is_set("plain-output-prefix", true)) {
            or_panic(oc.get_string("plain-output-prefix"))
        } else {
            String::new()
        };
        NWWriter_XML::write_network(oc, &prefix, nb);
        progress_time_message(before.elapsed().as_millis() as i64);
    }

    /// C++: `void NWFrame::writePositionLong(const Position &, OutputDevice &)` [static]
    /// Writes the given position to device in long format (one attribute per dimension)
    // void
    // NWFrame::writePositionLong(const Position& pos, OutputDevice& dev) {
    //     dev.writeAttr(SUMO_ATTR_X, pos.x());
    //     dev.writeAttr(SUMO_ATTR_Y, pos.y());
    //     if (pos.z() != 0) {
    //         dev.writeAttr(SUMO_ATTR_Z, pos.z());
    //     }
    // }
    #[allow(unused_variables)]
    pub fn write_position_long(pos: &Position, dev: &mut OutputDevice) {
        dev.write_attr(SUMO_ATTR_X, &pos.x());
        dev.write_attr(SUMO_ATTR_Y, &pos.y());
        if pos.z() != 0.0 {
            dev.write_attr(SUMO_ATTR_Z, &pos.z());
        }
    }

}

