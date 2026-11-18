//! Port of polyconvert/polyconvert_main.cpp
use std::any::Any;
use std::panic::{self, AssertUnwindSafe};
use std::process::ExitCode;

use sumo_core::polyconvert::pc_loader_arc_view::PCLoaderArcView;
use sumo_core::polyconvert::pc_loader_dlr_navteq::PCLoaderDlrNavteq;
use sumo_core::polyconvert::pc_loader_osm::PCLoaderOSM;
use sumo_core::polyconvert::pc_loader_visum::PCLoaderVisum;
use sumo_core::polyconvert::pc_loader_xml::PCLoaderXML;
use sumo_core::polyconvert::pc_net_projection_loader::PCNetProjectionLoader;
use sumo_core::polyconvert::pc_poly_container::PCPolyContainer;
use sumo_core::polyconvert::pc_type_def_handler::PCTypeDefHandler;
use sumo_core::polyconvert::pc_type_map::PCTypeMap;
use utils::common::msg_handler::{with_instance, write_error, write_warning, MsgHandler, MsgType};
use utils::common::to_string::f64_to_string_prec;
use utils::common::util_exceptions::ProcessError;
use utils::geom::boundary::Boundary;
use utils::geom::geo_conv_helper::GeoConvHelper;
use utils::geom::geom_conv_helper::parse_boundary_reporting;
use utils::options::options_cont::{with_options, OptionsCont};
use utils::options::option::SumoOption;
use utils::xml::xml_sub_sys;

// same version string as the netconvert port (the reference build reports it)
const VERSION_STRING: &str = "v1.28.0-0000000000";

// C++ pc_typemap.h: the data/typemap/*Polyconvert.typ.xml files compiled in
const NAVTEQ_TYPEMAP: &str = include_str!("../../sumo_core/data/typemap/navteqPolyconvert.typ.xml");
const OSM_TYPEMAP: &str = include_str!("../../sumo_core/data/typemap/osmPolyconvert.typ.xml");
const VISUM_TYPEMAP: &str = include_str!("../../sumo_core/data/typemap/visumPolyconvert.typ.xml");


fn fill_options() {
    with_options(|oc| {
        oc.add_call_example("-c <CONFIGURATION>", "run with configuration options set in file");
        utils::common::system_frame::add_configuration_options(oc);
        oc.add_option_sub_topic("Input");
        oc.add_option_sub_topic("Output");
        GeoConvHelper::add_projection_options(oc);
        oc.add_option_sub_topic("Pruning");
        oc.add_option_sub_topic("Processing");
        oc.add_option_sub_topic("Building Defaults");

        oc.do_register_abbr("net-file", 'n', SumoOption::file_name());
        oc.add_synonyme("net-file", "net", false);
        oc.add_description("net-file", "Input", "Loads SUMO-network FILE as reference to offset and projection");

        oc.do_register("dlr-navteq-poly-files", SumoOption::file_name());
        oc.add_description("dlr-navteq-poly-files", "Input", "Reads polygons from FILE assuming they're coded in DLR-Navteq (Elmar)-format");
        oc.do_register("dlr-navteq-poi-files", SumoOption::file_name());
        oc.add_description("dlr-navteq-poi-files", "Input", "Reads pois from FILE assuming they're coded in DLR-Navteq (Elmar)-format");

        oc.do_register("visum-files", SumoOption::file_name());
        oc.add_synonyme("visum-files", "visum", false);
        oc.add_description("visum-files", "Input", "Reads polygons from FILE assuming it's a Visum-net");
        oc.do_register("visum.language-file", SumoOption::file_name());
        oc.add_description("visum.language-file", "Input", "Load language mappings from FILE");

        oc.do_register("xml-files", SumoOption::file_name());
        oc.add_synonyme("xml-files", "xml", false);
        oc.add_description("xml-files", "Input", "Reads pois and shapes from FILE assuming they're coded in XML");

        oc.do_register("osm-files", SumoOption::file_name());
        oc.add_synonyme("osm-files", "osm", false);
        oc.add_description("osm-files", "Input", "Reads pois from FILE assuming they're coded in OSM");
        oc.do_register("osm.keep-full-type", SumoOption::bool(false));
        oc.add_description("osm.keep-full-type", "Input", "The type will be made of the key-value - pair");
        oc.do_register("osm.use-name", SumoOption::bool(false));
        oc.add_description("osm.use-name", "Input", "The id will be set from the given 'name' attribute");
        oc.do_register("osm.merge-relations", SumoOption::float(-1.0));
        oc.add_description("osm.merge-relations", "Input", "If FLOAT >= 0, assemble one polygon from all ways of a relation if they all connect with gaps below FLOAT");

        oc.do_register("geojson-files", SumoOption::file_name());
        oc.add_description("geojson-files", "Input", "Reads shapes from geojson FILE");

        oc.do_register("shapefile-prefixes", SumoOption::file_name());
        oc.add_synonyme("shapefile-prefixes", "shapefile-prefix", false);
        oc.add_synonyme("shapefile-prefixes", "shapefile", false);
        oc.add_synonyme("shapefile-prefixes", "shape-files", true);
        oc.add_description("shapefile-prefixes", "Input", "Reads shapes from shapefiles FILE");

        oc.do_register("shapefile.guess-projection", SumoOption::bool(false));
        oc.add_synonyme("shapefile.guess-projection", "arcview.guess-projection", true);
        oc.add_description("shapefile.guess-projection", "Input", "Guesses the shapefile's projection");

        oc.do_register("shapefile.traditional-axis-mapping", SumoOption::bool(false));
        oc.add_description("shapefile.traditional-axis-mapping", "Input", "Use traditional axis order (lon, lat)");

        oc.do_register("shapefile.id-column", SumoOption::string());
        oc.add_synonyme("shapefile.id-column", "shapefile.id-name", true);
        oc.add_synonyme("shapefile.id-column", "shape-files.id-name", true);
        oc.add_description("shapefile.id-column", "Input", "Defines in which column the id can be found");

        oc.do_register("shapefile.type-columns", SumoOption::string_vector());
        oc.add_synonyme("shapefile.type-columns", "shapefile.type-column", false);
        oc.add_description("shapefile.type-columns", "Input", "Defines which columns form the type id (comma separated list)");

        oc.do_register("shapefile.use-running-id", SumoOption::bool(false));
        oc.add_description("shapefile.use-running-id", "Input", "A running number will be used as id");

        oc.do_register("shapefile.add-param", SumoOption::bool(false));
        oc.add_description("shapefile.add-param", "Input", "Extract all additional columns as params");

        oc.do_register("shapefile.fill", SumoOption::string());
        oc.add_description("shapefile.fill", "Input", "[auto|true|false]. Forces the 'fill' status to the given value. Default 'auto' tries to determine it from the data type");

        oc.do_register("type-file", SumoOption::file_name());
        oc.add_synonyme("type-file", "typemap", true);
        oc.add_description("type-file", "Input", "Reads types from FILE");

        // need to do this here to be able to check for network and route input options
        utils::common::system_frame::add_report_options(oc);

        oc.do_register_abbr("output-file", 'o', SumoOption::file_name());
        oc.add_synonyme("output-file", "output", false);
        oc.add_description("output-file", "Output", "Write generated polygons/pois to FILE");

        oc.do_register("dlr-tdp-output", SumoOption::file_name());
        oc.add_description("dlr-tdp-output", "Output", "Write generated polygons/pois to a dlr-tdp file with the given prefix");

        oc.do_register("prune.in-net", SumoOption::bool(false));
        oc.add_synonyme("prune.in-net", "prune.on-net", true);
        oc.add_description("prune.in-net", "Pruning", "Enables pruning on net boundaries");

        oc.do_register("prune.in-net.offsets", SumoOption::string_with("0,0,0,0"));
        oc.add_synonyme("prune.in-net.offsets", "prune.on-net.offsets", true);
        oc.add_description("prune.in-net.offsets", "Pruning", "Uses FLOAT,FLOAT,FLOAT,FLOAT as offset definition added to the net boundary. Positive values grow the boundary on all sides while negative values shrink it.");

        oc.do_register("prune.boundary", SumoOption::string());
        oc.add_description("prune.boundary", "Pruning", "Uses STR as pruning boundary");

        oc.do_register("prune.keep-list", SumoOption::string());
        oc.add_synonyme("prune.keep-list", "prune.keep", false);
        oc.add_synonyme("prune.keep-list", "prune.ignore", true);
        oc.add_description("prune.keep-list", "Pruning", "Items in STR will be kept though out of boundary");

        oc.do_register("prune.explicit", SumoOption::string_vector_with(vec![String::new()]));
        oc.add_synonyme("prune.explicit", "remove", false);
        oc.add_description("prune.explicit", "Pruning", "Items with names in STR[] will be removed");

        oc.do_register("offset.x", SumoOption::float(0.0));
        oc.add_synonyme("offset.x", "x-offset-to-apply", true);
        oc.add_description("offset.x", "Processing", "Adds FLOAT to net x-positions");
        oc.do_register("offset.y", SumoOption::float(0.0));
        oc.add_synonyme("offset.y", "y-offset-to-apply", true);
        oc.add_description("offset.y", "Processing", "Adds FLOAT to net y-positions");
        oc.do_register("offset.z", SumoOption::float(0.0));
        oc.add_description("offset.z", "Processing", "Adds FLOAT to net z-positions");

        oc.do_register("all-attributes", SumoOption::bool(false));
        oc.add_description("all-attributes", "Processing", "Imports all attributes as key/value pairs");
        oc.do_register("ignore-errors", SumoOption::bool(false));
        oc.add_description("ignore-errors", "Processing", "Continue on broken input");
        oc.do_register("poi-layer-offset", SumoOption::float(0.0));
        oc.add_description("poi-layer-offset", "Processing", "Adds FLOAT to the layer value for each poi (i.e. to raise it above polygons)");
        oc.do_register("flatten", SumoOption::bool(false));
        oc.add_description("flatten", "Processing", "Remove all z-data");

        oc.do_register("color", SumoOption::string_with("0.2,0.5,1."));
        oc.add_description("color", "Building Defaults", "Sets STR as default color");
        oc.do_register("prefix", SumoOption::string_with(""));
        oc.add_description("prefix", "Building Defaults", "Sets STR as default prefix");
        oc.do_register("type", SumoOption::string_with("unknown"));
        oc.add_description("type", "Building Defaults", "Sets STR as default type");
        oc.do_register("fill", SumoOption::bool(true));
        oc.add_description("fill", "Building Defaults", "Fills polygons by default");
        oc.do_register("icon", SumoOption::string_with(""));
        oc.add_description("icon", "Building Defaults", "Sets STR as default icon");
        oc.do_register("layer", SumoOption::float(-1.0));
        oc.add_description("layer", "Building Defaults", "Sets FLOAT as default layer");
        oc.do_register("discard", SumoOption::bool(false));
        oc.add_description("discard", "Building Defaults", "Sets default action to discard");

        oc.do_register("proj.plain-geo", SumoOption::bool(false));
        oc.add_description("proj.plain-geo", "Projection", "Write geo coordinates in output");
    });
}

/// The try block of C++ `main()`; Ok(false) when a meta option ended the run
fn run(argv: &[String]) -> Result<bool, ProcessError> {
    xml_sub_sys::init();
    fill_options();
    utils::options::options_io::set_args_argv(argv);
    utils::options::options_io::get_options(false)?;
    if with_options(|oc| oc.process_meta_options(argv.len() < 2))? {
        utils::common::system_frame::close();
        return Ok(false);
    }
    with_options(|oc| utils::common::system_frame::check_options(oc));
    with_options(|oc| -> Result<(), ProcessError> {
        xml_sub_sys::set_validation(&oc.get_string("xml-validation")?, &oc.get_string("xml-validation.net")?, "never")?;
        if oc.is_default("aggregate-warnings") {
            oc.set_default("aggregate-warnings", "5")?;
        }
        Ok(())
    })?;
    MsgHandler::init_output_options()?;
    with_options(run_oc)
}

fn run_oc(oc: &mut OptionsCont) -> Result<bool, ProcessError> {
    // build the projection
    let mut scale = 1.0;
    if (oc.is_set("dlr-navteq-poly-files", true)? || oc.is_set("dlr-navteq-poi-files", true)?) && oc.is_default("proj.scale") {
        scale = 1e-5;
    }
    if !oc.is_set("net", true)? {
        // from the given options (PROJ_API_FILE is defined in the reference build)
        let num_projections = oc.get_bool("simple-projection")? as i32 + oc.get_bool("proj.utm")? as i32 + oc.get_bool("proj.dhdn")? as i32 + (oc.get_string("proj")?.len() > 1) as i32;
        if (oc.is_set("osm-files", true)?
            || oc.is_set("dlr-navteq-poly-files", true)?
            || oc.is_set("dlr-navteq-poi-files", true)?
            || oc.is_set("geojson-files", true)?
            || oc.is_set("shapefile-prefixes", true)?)
            && num_projections == 0
        {
            // input is lon,lat and projecting it to UTM ensures accurate handling of geometry
            oc.set("proj.utm", "true", false)?;
            if oc.is_default("proj.plain-geo") {
                // without reference to a network, raw UTM isn't helpful so we better write the data out as lon,lat
                oc.set("proj.plain-geo", "true", false)?;
            }
        }
        if oc.is_default("proj.scale") {
            oc.set("proj.scale", &f64_to_string_prec(scale, 5), false)?;
        }
        if !GeoConvHelper::init_from_options(oc)? {
            return Err(ProcessError::new("Could not build projection!"));
        }
    } else {
        // from the supplied network
        PCNetProjectionLoader::load(&oc.get_string("net")?, scale);
    }
    let mut pruning_boundary = GeoConvHelper::with_final(|h| *h.get_conv_boundary());
    // check whether the input shall be pruned
    let mut prune = false;
    if oc.get_bool("prune.in-net")? {
        if !oc.is_set("net", true)? {
            return Err(ProcessError::new("In order to prune the input on the net, you have to supply a network."));
        }
        // !!! no proper error handling
        let offsets = parse_boundary_reporting(&oc.get_string("prune.in-net.offsets")?, "--prune.on-net.offsets", None, true, true).unwrap_or_else(Boundary::new);
        pruning_boundary.set_offsets(
            pruning_boundary.xmin() - offsets.xmin(),
            pruning_boundary.ymin() - offsets.ymin(),
            pruning_boundary.xmax() + offsets.xmax(),
            pruning_boundary.ymax() + offsets.ymax(),
        );
        prune = true;
    }
    if oc.is_set("prune.boundary", true)? {
        // !!! no proper error handling
        pruning_boundary = parse_boundary_reporting(&oc.get_string("prune.boundary")?, "--prune.boundary", None, true, false).unwrap_or_else(Boundary::new);
        prune = true;
    }
    if oc.is_set("osm-files", true)? && oc.is_default("poi-layer-offset") {
        oc.set_default("poi-layer-offset", "5")?; // sufficient when using the default typemap
    }

    let mut to_fill = PCPolyContainer::new(prune, &pruning_boundary, &oc.get_string_vector("remove")?.clone());

    // read in the type defaults
    if !oc.is_set("type-file", true)? {
        match std::env::var("SUMO_HOME") {
            Err(_) => write_warning("Environment variable SUMO_HOME is not set, using built in type maps."),
            Ok(sumo_path) => {
                let path = format!("{sumo_path}/data/typemap/");
                if oc.is_set("dlr-navteq-poly-files", true)? {
                    oc.set_default("type-file", &format!("{path}navteqPolyconvert.typ.xml"))?;
                }
                if oc.is_set("osm-files", true)? {
                    oc.set_default("type-file", &format!("{path}osmPolyconvert.typ.xml"))?;
                }
                if oc.is_set("visum-files", true)? {
                    oc.set_default("type-file", &format!("{path}visumPolyconvert.typ.xml"))?;
                }
            }
        }
    }
    let mut tm = PCTypeMap::new(oc);
    let mut handler = PCTypeDefHandler::new(oc, &mut tm);
    if oc.is_set("type-file", true)? {
        if !xml_sub_sys::run_parser(&mut handler, &oc.get_string("type-file")?, false, false, false, true)? {
            // something failed
            return Err(ProcessError::default_msg());
        }
    } else {
        handler.set_file_name("built in type map");
        let mut reader = xml_sub_sys::get_sax_reader(false, false);
        if oc.is_set("dlr-navteq-poly-files", true)? {
            reader.parse_string(&mut handler, NAVTEQ_TYPEMAP)?;
        }
        if oc.is_set("osm-files", true)? {
            reader.parse_string(&mut handler, OSM_TYPEMAP)?;
        }
        if oc.is_set("visum-files", true)? {
            reader.parse_string(&mut handler, VISUM_TYPEMAP)?;
        }
    }
    // read in the data
    PCLoaderXML::load_if_set(oc, &mut to_fill, &mut tm); // SUMO-XML
    PCLoaderOSM::load_if_set(oc, &mut to_fill, &mut tm); // OSM-XML
    PCLoaderDlrNavteq::load_if_set(oc, &mut to_fill, &mut tm); // Elmar-files
    PCLoaderVisum::load_if_set(oc, &mut to_fill, &mut tm); // VISUM
    PCLoaderArcView::load_if_set(oc, &mut to_fill, &mut tm); // shape-files
    GeoConvHelper::compute_final(false);
    // error processing
    if with_instance(MsgType::Error, |h| h.was_informed()) && !oc.get_bool("ignore-errors")? {
        return Err(ProcessError::default_msg());
    }
    // output
    if !oc.is_set("output-file", true)? && !oc.is_set("dlr-tdp-output", true)? {
        let mut out = "polygons.xml".to_string();
        if oc.is_set("configuration-file", true)? {
            out = utils::common::file_helpers::get_configuration_relative(&oc.get_string("configuration-file")?, &out);
        }
        oc.set_default("output-file", &out)?;
    }
    if oc.is_set("output-file", true)? {
        to_fill.save(&oc.get_string("output-file")?, oc.get_bool("proj.plain-geo")?);
    }
    if oc.is_set("dlr-tdp-output", true)? {
        to_fill.save_dlr_tdp(&oc.get_string("dlr-tdp-output")?);
    }
    Ok(true)
}

/// the catch (const ProcessError&) block
fn quit_on_process_error(e: &ProcessError) {
    let what = e.to_string();
    if what != "Process Error" && !what.is_empty() {
        write_error(&what);
    }
    with_instance(MsgType::Error, |h| h.inform("Quitting (on error).", false));
}

/// C++ throws that unwound to main: ProcessError payloads, other panics as std::exception
fn report_panic(payload: Box<dyn Any + Send>) {
    if let Some(e) = payload.downcast_ref::<ProcessError>() {
        quit_on_process_error(e);
    } else if let Some(s) = payload.downcast_ref::<String>() {
        if !s.is_empty() {
            write_error(s);
        }
        with_instance(MsgType::Error, |h| h.inform("Quitting (on error).", false));
    } else if let Some(s) = payload.downcast_ref::<&str>() {
        if !s.is_empty() {
            write_error(s);
        }
        with_instance(MsgType::Error, |h| h.inform("Quitting (on error).", false));
    } else {
        with_instance(MsgType::Error, |h| h.inform("Quitting (on unknown error).", false));
    }
}

fn main() -> ExitCode {
    let argv: Vec<String> = std::env::args().collect();
    let default_hook = panic::take_hook();
    panic::set_hook(Box::new(move |info| {
        if !info.payload().is::<ProcessError>() {
            default_hook(info);
        }
    }));
    with_options(|oc| {
        oc.set_application_description("Importer of polygons and POIs for the microscopic, multi-modal traffic simulation SUMO.");
        oc.set_application_name("polyconvert", &format!("Eclipse SUMO polyconvert {VERSION_STRING}"));
    });
    let ret: u8 = match panic::catch_unwind(AssertUnwindSafe(|| run(&argv))) {
        Ok(Ok(true)) => 0,
        Ok(Ok(false)) => return ExitCode::SUCCESS,
        Ok(Err(e)) => {
            quit_on_process_error(&e);
            1
        }
        Err(payload) => {
            report_panic(payload);
            1
        }
    };
    utils::common::system_frame::close();
    if ret == 0 {
        println!("Success.");
    }
    ExitCode::from(ret)
}
