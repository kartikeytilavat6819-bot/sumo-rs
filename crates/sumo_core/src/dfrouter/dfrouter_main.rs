//! Port of dfrouter/dfrouter_main.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::msg_handler::{progress_begin_message, progress_done_message, progress_failed_message};

/// C++: `void (RODFDetectorFlows &, OptionsCont &, RODFDetectorCon &) readDetectorFlows`
// void
// readDetectorFlows(RODFDetectorFlows& flows, OptionsCont& oc, RODFDetectorCon& dc) {
//     if (!oc.isSet("measure-files")) {
//         // ok, not given, return an empty container
//         return;
//     }
//     // check whether the file exists
//     std::vector<std::string> files = oc.getStringVector("measure-files");
//     for (std::vector<std::string>::const_iterator fileIt = files.begin(); fileIt != files.end(); ++fileIt) {
//         if (!FileHelpers::isReadable(*fileIt)) {
//             throw ProcessError(TLF("The measure-file '%' can not be opened.", *fileIt));
//         }
//         // parse
//         PROGRESS_BEGIN_MESSAGE("Loading flows from '" + *fileIt + "'");
//         RODFDetFlowLoader dfl(dc, flows, string2time(oc.getString("begin")), string2time(oc.getString("end")),
//                               string2time(oc.getString("time-offset")), string2time(oc.getString("time-factor")));
//         dfl.read(*fileIt);
//         PROGRESS_DONE_MESSAGE();
//     }
// }
#[allow(unused_variables)]
pub fn read_detector_flows(flows: &mut RODFDetectorFlows, oc: &mut OptionsCont, dc: &mut RODFDetectorCon) {
    if !oc.is_set("measure-files", true).unwrap() {
        // ok, not given, return an empty container
        return;
    }
    // check whether the file exists
    let files = oc.get_string_vector("measure-files").unwrap().clone();
    for fileIt in files.iter() {
        if !utils::common::file_helpers::is_readable(fileIt) {
            panic!("{}", ProcessError::new(format!("The measure-file '{}' can not be opened.", fileIt)));
        }
        // parse
        progress_begin_message(&format!("Loading flows from '{}'", fileIt));
        let mut dfl = RODFDetFlowLoader::new(&*dc, &mut *flows,
                                             string2time(&oc.get_string("begin").unwrap()).unwrap(), string2time(&oc.get_string("end").unwrap()).unwrap(),
                                             string2time(&oc.get_string("time-offset").unwrap()).unwrap(), string2time(&oc.get_string("time-factor").unwrap()).unwrap());
        dfl.read(fileIt);
        progress_done_message();
    }
}

/// Adapter exposing `RODFDetectorHandler`'s SAX entry points to the parser trait object.
struct DetectorHandlerSax<'a>(&'a mut RODFDetectorHandler);

impl SaxHandler for DetectorHandlerSax<'_> {
    fn handler(&mut self) -> &mut GenericSAXHandler {
        &mut self.0.base
    }
    fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) -> Result<(), ProcessError> {
        self.0.my_start_element(element, attrs);
        Ok(())
    }
}

/// C++: `void (RODFDetectorCon &, OptionsCont &, RODFNet *) readDetectors`
// void
// readDetectors(RODFDetectorCon& detectors, OptionsCont& oc, RODFNet* optNet) {
//     if (!oc.isSet("detector-files")) {
//         throw ProcessError(TL("No detector file given (use --detector-files <FILE>)."));
//     }
//     // read definitions stored in XML-format
//     std::vector<std::string> files = oc.getStringVector("detector-files");
//     for (std::vector<std::string>::const_iterator fileIt = files.begin(); fileIt != files.end(); ++fileIt) {
//         if (!FileHelpers::isReadable(*fileIt)) {
//             throw ProcessError(TLF("Could not open detector file '%'", *fileIt));
//         }
//         PROGRESS_BEGIN_MESSAGE("Loading detector definitions from '" + *fileIt + "'");
//         RODFDetectorHandler handler(optNet, oc.getBool("ignore-invalid-detectors"), detectors, *fileIt);
//         if (XMLSubSys::runParser(handler, *fileIt)) {
//             PROGRESS_DONE_MESSAGE();
//         } else {
//             PROGRESS_FAILED_MESSAGE();
//             throw ProcessError();
//         }
//     }
//     if (detectors.getDetectors().empty()) {
//         throw ProcessError(TL("No detectors found."));
//     }
// }
#[allow(unused_variables)]
pub fn read_detectors(detectors: &mut RODFDetectorCon, oc: &mut OptionsCont, optNet: Ptr<RODFNet>) {
    if !oc.is_set("detector-files", true).unwrap() {
        panic!("{}", ProcessError::new("No detector file given (use --detector-files <FILE>)."));
    }
    // read definitions stored in XML-format
    let files = oc.get_string_vector("detector-files").unwrap().clone();
    for fileIt in files.iter() {
        if !utils::common::file_helpers::is_readable(fileIt) {
            panic!("{}", ProcessError::new(format!("Could not open detector file '{}'", fileIt)));
        }
        progress_begin_message(&format!("Loading detector definitions from '{}'", fileIt));
        let mut handler = RODFDetectorHandler::new(optNet, oc.get_bool("ignore-invalid-detectors").unwrap(), detectors, fileIt);
        let mut sax = DetectorHandlerSax(&mut handler);
        if utils::xml::xml_sub_sys::run_parser(&mut sax, fileIt, false, false, false, true).unwrap() {
            progress_done_message();
        } else {
            progress_failed_message();
            panic!("{}", ProcessError::default_msg());
        }
    }
    if detectors.get_detectors().is_empty() {
        panic!("{}", ProcessError::new("No detectors found."));
    }
}

/// C++: `void (RODFNet *, RODFDetectorFlows &, RODFDetectorCon &, OptionsCont &) startComputation`
// void
// startComputation(RODFNet* optNet, RODFDetectorFlows& flows, RODFDetectorCon& detectors, OptionsCont& oc) {
//     if (oc.getBool("print-absolute-flows")) {
//         flows.printAbsolute();
//     }
//
//     if (oc.getBool("remove-empty-detectors")) {
//         PROGRESS_BEGIN_MESSAGE(TL("Removing empty detectors"));
//         optNet->removeEmptyDetectors(detectors, flows);
//         PROGRESS_DONE_MESSAGE();
//     } else  if (oc.getBool("report-empty-detectors")) {
//         PROGRESS_BEGIN_MESSAGE(TL("Scanning for empty detectors"));
//         optNet->reportEmptyDetectors(detectors, flows);
//         PROGRESS_DONE_MESSAGE();
//     }
//     // compute the detector types (optionally)
//     if (!detectors.detectorsHaveCompleteTypes() || oc.getBool("revalidate-detectors")) {
//         optNet->computeTypes(detectors, oc.getBool("strict-sources"));
//     }
//     std::vector<RODFDetector*>::const_iterator i = detectors.getDetectors().begin();
//     for (; i != detectors.getDetectors().end(); ++i) {
//         if ((*i)->getType() == SOURCE_DETECTOR) {
//             break;
//         }
//     }
//     if (i == detectors.getDetectors().end() && !oc.getBool("routes-for-all")) {
//         throw ProcessError(TL("No source detectors found."));
//     }
//     // compute routes between the detectors (optionally)
//     if (!detectors.detectorsHaveRoutes() || oc.getBool("revalidate-routes") || oc.getBool("guess-empty-flows")) {
//         PROGRESS_BEGIN_MESSAGE(TL("Computing routes"));
//         optNet->buildRoutes(detectors,
//                             oc.getBool("keep-unfinished-routes"), oc.getBool("routes-for-all"),
//                             !oc.getBool("keep-longer-routes"), oc.getInt("max-search-depth"));
//         PROGRESS_DONE_MESSAGE();
//     }
//
//     // check
//     // whether the detectors are valid
//     if (!detectors.detectorsHaveCompleteTypes()) {
//         throw ProcessError(TL("The detector types are not defined; use in combination with a network"));
//     }
//     // whether the detectors have routes
//     if (!detectors.detectorsHaveRoutes()) {
//         throw ProcessError(TL("The emitters have no routes; use in combination with a network"));
//     }
//
//     // save the detectors if wished
//     if (oc.isSet("detector-output")) {
//         detectors.save(oc.getString("detector-output"));
//     }
//     // save their positions as POIs if wished
//     if (oc.isSet("detectors-poi-output")) {
//         detectors.saveAsPOIs(oc.getString("detectors-poi-output"));
//     }
//
//     // save the routes file if it was changed or it's wished
//     if (detectors.detectorsHaveRoutes() && oc.isSet("routes-output")) {
//         detectors.saveRoutes(oc.getString("routes-output"));
//     }
//
//     // guess flows if wished
//     if (oc.getBool("guess-empty-flows")) {
//         optNet->buildDetectorDependencies(detectors);
//         detectors.guessEmptyFlows(flows);
//     }
//
//     const SUMOTime begin = string2time(oc.getString("begin"));
//     const SUMOTime end = string2time(oc.getString("end"));
//     const SUMOTime step = string2time(oc.getString("time-step"));
//
//     // save emitters if wished
//     if (oc.isSet("emitters-output") || oc.isSet("emitters-poi-output")) {
//         optNet->buildEdgeFlowMap(flows, detectors, begin, end, step); // !!!
//         if (oc.getBool("revalidate-flows")) {
//             PROGRESS_BEGIN_MESSAGE(TL("Rechecking loaded flows"));
//             optNet->revalidateFlows(detectors, flows, begin, end, step);
//             PROGRESS_DONE_MESSAGE();
//         }
//         if (oc.isSet("emitters-output")) {
//             PROGRESS_BEGIN_MESSAGE(TL("Writing emitters"));
//             detectors.writeEmitters(oc.getString("emitters-output"), flows,
//                                     begin, end, step,
//                                     *optNet,
//                                     oc.getBool("calibrator-output"),
//                                     oc.getBool("include-unused-routes"),
//                                     oc.getFloat("scale"),
// //                                    oc.getInt("max-search-depth"),
//                                     oc.getBool("emissions-only"));
//             PROGRESS_DONE_MESSAGE();
//         }
//         if (oc.isSet("emitters-poi-output")) {
//             PROGRESS_BEGIN_MESSAGE(TL("Writing emitter pois"));
//             detectors.writeEmitterPOIs(oc.getString("emitters-poi-output"), flows);
//             PROGRESS_DONE_MESSAGE();
//         }
//     }
//     // save end speed trigger if wished
//     if (oc.isSet("variable-speed-sign-output")) {
//         PROGRESS_BEGIN_MESSAGE(TL("Writing speed triggers"));
//         detectors.writeSpeedTrigger(optNet, oc.getString("variable-speed-sign-output"), flows,
//                                     begin, end, step);
//         PROGRESS_DONE_MESSAGE();
//     }
//     // save checking detectors if wished
//     if (oc.isSet("validation-output")) {
//         PROGRESS_BEGIN_MESSAGE(TL("Writing validation detectors"));
//         detectors.writeValidationDetectors(oc.getString("validation-output"),
//                                            oc.getBool("validation-output.add-sources"), true, true); // !!!
//         PROGRESS_DONE_MESSAGE();
//     }
//     // build global rerouter on end if wished
//     if (oc.isSet("end-reroute-output")) {
//         PROGRESS_BEGIN_MESSAGE(TL("Writing highway end rerouter"));
//         detectors.writeEndRerouterDetectors(oc.getString("end-reroute-output")); // !!!
//         PROGRESS_DONE_MESSAGE();
//     }
//     /*
//        // save the insertion definitions
//        if(oc.isSet("flow-definitions")) {
//            buildVehicleEmissions(oc.getString("flow-definitions"));
//        }
//     */
//     //
// }
#[allow(unused_variables)]
pub fn start_computation(optNet: Ptr<RODFNet>, flows: &mut RODFDetectorFlows, detectors: &mut RODFDetectorCon, oc: &mut OptionsCont) {
    if oc.get_bool("print-absolute-flows").unwrap() {
        flows.print_absolute();
    }

    if oc.get_bool("remove-empty-detectors").unwrap() {
        progress_begin_message("Removing empty detectors");
        optNet.m().remove_empty_detectors(detectors, flows);
        progress_done_message();
    } else if oc.get_bool("report-empty-detectors").unwrap() {
        progress_begin_message("Scanning for empty detectors");
        optNet.m().report_empty_detectors(detectors, flows);
        progress_done_message();
    }
    // compute the detector types (optionally)
    if !detectors.detectors_have_complete_types() || oc.get_bool("revalidate-detectors").unwrap() {
        optNet.m().compute_types(detectors, oc.get_bool("strict-sources").unwrap());
    }
    let hasSource = detectors.get_detectors().iter().any(|d| matches!(d.get_type(), RODFDetectorType::SOURCE_DETECTOR));
    if !hasSource && !oc.get_bool("routes-for-all").unwrap() {
        panic!("{}", ProcessError::new("No source detectors found."));
    }
    // compute routes between the detectors (optionally)
    if !detectors.detectors_have_routes() || oc.get_bool("revalidate-routes").unwrap() || oc.get_bool("guess-empty-flows").unwrap() {
        progress_begin_message("Computing routes");
        optNet.m().build_routes(detectors, oc.get_bool("keep-unfinished-routes").unwrap(),
                            oc.get_bool("routes-for-all").unwrap(), !oc.get_bool("keep-longer-routes").unwrap(), oc.get_int("max-search-depth").unwrap());
        progress_done_message();
    }

    // check
    // whether the detectors are valid
    if !detectors.detectors_have_complete_types() {
        panic!("{}", ProcessError::new("The detector types are not defined; use in combination with a network"));
    }
    // whether the detectors have routes
    if !detectors.detectors_have_routes() {
        panic!("{}", ProcessError::new("The emitters have no routes; use in combination with a network"));
    }

    // save the detectors if wished
    if oc.is_set("detector-output", true).unwrap() {
        detectors.save(&oc.get_string("detector-output").unwrap());
    }
    // save their positions as POIs if wished
    if oc.is_set("detectors-poi-output", true).unwrap() {
        detectors.save_as_po_is(&oc.get_string("detectors-poi-output").unwrap());
    }

    // save the routes file if it was changed or it's wished
    if detectors.detectors_have_routes() && oc.is_set("routes-output", true).unwrap() {
        detectors.save_routes(&oc.get_string("routes-output").unwrap());
    }

    // guess flows if wished
    if oc.get_bool("guess-empty-flows").unwrap() {
        optNet.m().build_detector_dependencies(detectors);
        detectors.guess_empty_flows(flows);
    }

    let begin = string2time(&oc.get_string("begin").unwrap()).unwrap();
    let end = string2time(&oc.get_string("end").unwrap()).unwrap();
    let step = string2time(&oc.get_string("time-step").unwrap()).unwrap();

    // save emitters if wished
    if oc.is_set("emitters-output", true).unwrap() || oc.is_set("emitters-poi-output", true).unwrap() {
        optNet.m().build_edge_flow_map(flows, detectors, begin, end, step); // !!!
        if oc.get_bool("revalidate-flows").unwrap() {
            progress_begin_message("Rechecking loaded flows");
            optNet.m().revalidate_flows(detectors, flows, begin, end, step);
            progress_done_message();
        }
        if oc.is_set("emitters-output", true).unwrap() {
            progress_begin_message("Writing emitters");
            detectors.write_emitters(&oc.get_string("emitters-output").unwrap(), flows,
                                     begin, end, step,
                                     &*optNet,
                                     oc.get_bool("calibrator-output").unwrap(),
                                     oc.get_bool("include-unused-routes").unwrap(),
                                     oc.get_float("scale").unwrap(),
//                                    oc.getInt("max-search-depth"),
                                     oc.get_bool("emissions-only").unwrap());
            progress_done_message();
        }
        if oc.is_set("emitters-poi-output", true).unwrap() {
            progress_begin_message("Writing emitter pois");
            detectors.write_emitter_po_is(&oc.get_string("emitters-poi-output").unwrap(), flows);
            progress_done_message();
        }
    }
    // save end speed trigger if wished
    if oc.is_set("variable-speed-sign-output", true).unwrap() {
        progress_begin_message("Writing speed triggers");
        detectors.write_speed_trigger(optNet, &oc.get_string("variable-speed-sign-output").unwrap(), flows,
                                      begin, end, step);
        progress_done_message();
    }
    // save checking detectors if wished
    if oc.is_set("validation-output", true).unwrap() {
        progress_begin_message("Writing validation detectors");
        detectors.write_validation_detectors(&oc.get_string("validation-output").unwrap(),
                                             oc.get_bool("validation-output.add-sources").unwrap(), true, true); // !!!
        progress_done_message();
    }
    // build global rerouter on end if wished
    if oc.is_set("end-reroute-output", true).unwrap() {
        progress_begin_message("Writing highway end rerouter");
        detectors.write_end_rerouter_detectors(&oc.get_string("end-reroute-output").unwrap()); // !!!
        progress_done_message();
    }
}

