//! Port of activitygen/activitygen_main.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::file_helpers::is_readable;
use utils::common::msg_handler::{progress_begin_message, progress_done_message, progress_failed_message, write_message};
use utils::common::rand_helper::init_rand;
use utils::options::options_cont::{get_bool, get_int, get_string, with_options};
use utils::options::options_io::{get_options, set_args};
use utils::xml::xml_sub_sys;
use crate::activitygen::city::ag_street::AGStreet_Builder;

/// Adapter exposing `RONetHandler`'s SAX entry points to the parser trait object.
struct NetHandlerSax<'a>(&'a mut RONetHandler);

impl SaxHandler for NetHandlerSax<'_> {
    fn handler(&mut self) -> &mut GenericSAXHandler {
        &mut self.0.base
    }
    fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) -> Result<(), ProcessError> {
        self.0.my_start_element(element, attrs);
        Ok(())
    }
    fn my_end_element(&mut self, element: i32) -> Result<(), ProcessError> {
        self.0.my_end_element(element);
        Ok(())
    }
}

/// C++: `void (RONet &, ROAbstractEdgeBuilder &) loadNet`
// void
// loadNet(RONet& toFill, ROAbstractEdgeBuilder& eb) {
//     OptionsCont& oc = OptionsCont::getOptions();
//     std::string file = oc.getString("net-file");
//     if (file == "") {
//         throw ProcessError(TL("Missing definition of network to load!"));
//     }
//     if (!FileHelpers::isReadable(file)) {
//         throw ProcessError(TLF("The network file '%' could not be accessed.", file));
//     }
//     PROGRESS_BEGIN_MESSAGE(TL("Loading net"));
//     RONetHandler handler(toFill, eb, true, 0, 0, 0);
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
// }
#[allow(unused_variables)]
pub fn load_net(toFill: &mut RONet, eb: &mut ROAbstractEdgeBuilder) {
    let file = get_string("net-file");
    if file == "" {
        panic!("{}", ProcessError::new("Missing definition of network to load!"));
    }
    if !is_readable(&file) {
        panic!("{}", ProcessError::new(format!("The network file '{}' could not be accessed.", file)));
    }
    progress_begin_message("Loading net");
    let mut handler = RONetHandler::new(toFill, eb, true, 0.0, 0.0, 0.0);
    handler.set_file_name(&file);
    let parsed = xml_sub_sys::run_parser(&mut NetHandlerSax(&mut handler), &file, true, false, false, true).unwrap_or_else(|e| panic!("{}", e));
    if !parsed {
        progress_failed_message();
        panic!("{}", ProcessError::new("Process Error"));
    } else {
        progress_done_message();
    }
    // shortcut: C++ global deprecatedVehicleClassesSeen (utils/common/SUMOVehicleClass.cpp) is not ported, the deprecated-class warning is skipped until it is
}

/// C++: `int (int, char **) main`
// int
// main(int argc, char* argv[]) {
//     OptionsCont& oc = OptionsCont::getOptions();
//     oc.setApplicationDescription(
//         TL("Generates trips of persons throughout a day for the microscopic, multi-modal traffic simulation SUMO."));
//     oc.setApplicationName("activitygen", "Eclipse SUMO activitygen " VERSION_STRING);
//     oc.addCopyrightNotice("Copyright (C) 2010-2012 Technische Universitaet Muenchen");
//     int ret = 0;
//     RONet* net = nullptr;
//     try {
//         // Initialise subsystems and process options
//         XMLSubSys::init();
//         AGFrame::fillOptions();
//         OptionsIO::setArgs(argc, argv);
//         OptionsIO::getOptions();
//         if (oc.processMetaOptions(argc < 2)) {
//             SystemFrame::close();
//             return 0;
//         }
//         XMLSubSys::setValidation(oc.getString("xml-validation"), oc.getString("xml-validation.net"), "never");
//         MsgHandler::initOutputOptions();
//         RandHelper::initRandGlobal();
//         SystemFrame::checkOptions(oc);
//
//         // Load network
//         net = new RONet();
//         AGStreet::Builder builder;
//         loadNet(*net, builder);
//         WRITE_MESSAGEF(TL("Loaded % edges."), toString(net->getEdgeNumber()));
//         if (oc.getBool("debug")) {
//             WRITE_MESSAGE("\n\t ---- begin ActivityGen ----\n");
//         }
//
//         std::string statFile = oc.getString("stat-file");
//         OutputDevice::createDeviceByOption("output-file", "routes", "routes_file.xsd");
//         AGTime duration(oc.getInt("duration-d"), 0, 0);
//         AGTime begin(oc.getInt("begin") % 86400);
//         AGTime end(oc.getInt("end") % 86400);
//         AGActivityGen actiGen(statFile, OutputDevice::getDevice(oc.getString("output-file")), net);
//         actiGen.importInfoCity();
//         actiGen.makeActivityTrips(duration.getDay(), begin.getTime(), end.getTime());
//
//         if (oc.getBool("debug")) {
//             WRITE_MESSAGE("\n\t ---- end of ActivityGen ----\n");
//         }
//         ret = 0;
//     } catch (const ProcessError& e) {
//         if (std::string(e.what()) != std::string("Process Error") && std::string(e.what()) != std::string("")) {
//             WRITE_ERROR(e.what());
//         }
//         MsgHandler::getErrorInstance()->inform("Quitting (on error).", false);
//         ret = 1;
// #ifndef _DEBUG
//     } catch (const std::exception& e) {
//         if (std::string(e.what()) != std::string("")) {
//             WRITE_ERROR(e.what());
//         }
//         MsgHandler::getErrorInstance()->inform("Quitting (on error).", false);
//         ret = 1;
//     } catch (...) {
//         MsgHandler::getErrorInstance()->inform("Quitting (on unknown error).", false);
//         ret = 1;
// #endif
//     }
//     SystemFrame::close();
//     if (ret == 0) {
//         std::cout << "Success." << std::endl;
//     }
//     return ret;
// }
#[allow(unused_variables)]
pub fn main(argc: i32, argv: &[String]) -> i32 {
    with_options(|oc| {
        oc.set_application_description("Generates trips of persons throughout a day for the microscopic, multi-modal traffic simulation SUMO.");
        oc.set_application_name("activitygen", "Eclipse SUMO activitygen 1.28.0"); // config.h VERSION_STRING is 1.28.0 for this release; the port has no config.h
        oc.add_copyright_notice("Copyright (C) 2010-2012 Technische Universitaet Muenchen");
    });
    // shortcut: C++ try/catch that reports "Quitting (on error)" and returns 1 is not reproduced, errors panic; upgrade when the error path of the frame lands
    xml_sub_sys::init();
    AGFrame::fill_options();
    set_args(argv);
    get_options(false).unwrap_or_else(|e| panic!("{}", e));
    if with_options(|oc| oc.process_meta_options(argc < 2)).unwrap_or_else(|e| panic!("{}", e)) {
        utils::common::system_frame::close();
        return 0;
    }
    xml_sub_sys::set_validation(&get_string("xml-validation"), &get_string("xml-validation.net"), "never").unwrap_or_else(|e| panic!("{}", e));
    MsgHandler::init_output_options().unwrap_or_else(|e| panic!("{}", e));
    init_rand(None, get_bool("random"), get_int("seed")); // RandHelper::initRandGlobal()
    with_options(|oc| {
        utils::common::system_frame::check_options(oc);
    });

    // Load network
    let mut net = RONet::new();
    let mut builder = AGStreet_Builder::new();
    load_net(&mut net, &mut builder.base);
    write_message(&format!("Loaded {} edges.", net.get_edge_number()));
    if get_bool("debug") {
        write_message("\n\t ---- begin ActivityGen ----\n");
    }

    let statFile = get_string("stat-file");
    OutputDevice::create_device_by_option("output-file", "routes", "routes_file.xsd", -1).unwrap_or_else(|e| panic!("{}", e));
    let mut duration = AGTime::new_3(get_int("duration-d"), 0, 0);
    let mut begin = AGTime::new(get_int("begin") % 86400);
    let mut end = AGTime::new(get_int("end") % 86400);
    let outDevice = OutputDevice::get_device(&get_string("output-file"), true).unwrap_or_else(|e| panic!("{}", e));
    let mut actiGen = AGActivityGen::new(statFile, &mut *outDevice.borrow_mut(), Ptr::from_mut(&mut net));
    actiGen.import_info_city();
    actiGen.make_activity_trips(duration.get_day(), begin.get_time(), end.get_time());

    if get_bool("debug") {
        write_message("\n\t ---- end of ActivityGen ----\n");
    }
    let ret = 0;

    utils::common::system_frame::close();
    if ret == 0 {
        println!("Success.");
    }
    ret
}

