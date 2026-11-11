//! Port of dfrouter/dfrouter_main.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

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
    todo!("port readDetectorFlows")
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
    todo!("port readDetectors")
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
    todo!("port startComputation")
}

