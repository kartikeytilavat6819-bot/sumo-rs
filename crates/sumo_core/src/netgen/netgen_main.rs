//! Port of netgen/netgen_main.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++: `NGNet *(NBNetBuilder &) buildNetwork`
// NGNet*
// buildNetwork(NBNetBuilder& nb) {
//     OptionsCont& oc = OptionsCont::getOptions();
//
//     const double laneWidth = oc.isDefault("default.lanewidth") ? SUMO_const_laneWidth : oc.getFloat("default.lanewidth");
//     double minLength = (oc.getInt("default.lanenumber") + oc.getInt("turn-lanes")) * 2 * laneWidth + oc.getFloat("default.junctions.radius") * 2 + POSITION_EPS;
//     // spider-net
//     if (oc.getBool("spider")) {
//         // check values
//         bool hadError = false;
//         if (oc.getInt("spider.arm-number") < 3) {
//             WRITE_ERROR(TL("Spider networks need at least 3 arms."));
//             hadError = true;
//         }
//         if (oc.getInt("spider.arm-number") > 4 && !oc.getBool("spider.omit-center")) {
//             WRITE_WARNING(TL("Spider networks with many arms should use option spider.omit-center"));
//         }
//         if (oc.getInt("spider.circle-number") < 1) {
//             WRITE_ERROR(TL("Spider networks need at least one circle."));
//             hadError = true;
//         }
//         minLength = MAX2(minLength, laneWidth * 2 * MAX2(oc.getInt("spider.arm-number"), 3) / (2 * M_PI));
//         if (oc.getFloat("spider.space-radius") < POSITION_EPS) {
//             WRITE_ERROR("The radius of spider networks must be at least "  + toString(POSITION_EPS));
//             hadError = true;
//         } else if (oc.getFloat("spider.space-radius") < minLength) {
//             WRITE_WARNINGF(TL("The radius of spider networks should be at least % for the given lanenumber, lanewidth and junction radius"), toString(minLength));
//         }
//         if (hadError) {
//             throw ProcessError();
//         }
//         // build if everything's ok
//         NGNet* net = new NGNet(nb);
//         net->createSpiderWeb(oc.getInt("spider.arm-number"), oc.getInt("spider.circle-number"),
//                              oc.getFloat("spider.space-radius"), !oc.getBool("spider.omit-center"),
//                              oc.getFloat("spider.attach-length"));
//         return net;
//     }
//     // grid-net
//     if (oc.getBool("grid")) {
//         // get options
//         int xNo = oc.getInt("grid.x-number");
//         int yNo = oc.getInt("grid.y-number");
//         double xLength = oc.getFloat("grid.x-length");
//         double yLength = oc.getFloat("grid.y-length");
//         double xAttachLength = oc.getFloat("grid.x-attach-length");
//         double yAttachLength = oc.getFloat("grid.y-attach-length");
//         if (oc.isDefault("grid.x-number") && !oc.isDefault("grid.number")) {
//             xNo = oc.getInt("grid.number");
//         }
//         if (oc.isDefault("grid.y-number") && !oc.isDefault("grid.number")) {
//             yNo = oc.getInt("grid.number");
//         }
//         if (oc.isDefault("grid.x-length") && !oc.isDefault("grid.length")) {
//             xLength = oc.getFloat("grid.length");
//         }
//         if (oc.isDefault("grid.y-length") && !oc.isDefault("grid.length")) {
//             yLength = oc.getFloat("grid.length");
//         }
//         if (oc.isDefault("grid.x-attach-length") && !oc.isDefault("grid.attach-length")) {
//             xAttachLength = oc.getFloat("grid.attach-length");
//         }
//         if (oc.isDefault("grid.y-attach-length") && !oc.isDefault("grid.attach-length")) {
//             yAttachLength = oc.getFloat("grid.attach-length");
//         }
//         // check values
//         bool hadError = false;
//         if (xNo < 1 || yNo < 1 || (xAttachLength == 0 && yAttachLength == 0 && (xNo < 2 && yNo < 2))) {
//             WRITE_ERROR(TL("The number of nodes must be positive and at least 2 in one direction if there are no attachments."));
//             hadError = true;
//         }
//         const double minAttachLength = minLength / 2 + POSITION_EPS / 2;
//         if (xLength < POSITION_EPS || yLength < POSITION_EPS) {
//             WRITE_ERROR("The distance between nodes must be at least " + toString(POSITION_EPS));
//             hadError = true;
//         } else if (xLength < minLength || yLength < minLength) {
//             WRITE_WARNINGF(TL("The distance between nodes should be at least % for the given lanenumber, lanewidth and junction radius"), toString(minLength));
//         }
//         if (xAttachLength != 0.0 && xAttachLength < POSITION_EPS) {
//             WRITE_ERROR("The length of attached streets must be at least " + toString(POSITION_EPS));
//             hadError = true;
//         } else if (xAttachLength != 0.0 && xAttachLength < minAttachLength) {
//             WRITE_WARNINGF(TL("The length of attached streets should be at least % for the given lanenumber, lanewidth and junction radius"), toString(minAttachLength));
//         } else if (yAttachLength != 0.0 && yAttachLength < POSITION_EPS) {
//             WRITE_ERROR("The length of attached streets must be at least " + toString(POSITION_EPS));
//             hadError = true;
//         } else if (yAttachLength != 0.0 && yAttachLength < minAttachLength) {
//             WRITE_WARNINGF(TL("The length of attached streets should be at least % for the given lanenumber, lanewidth and junction radius"), toString(minAttachLength));
//         }
//         if (hadError) {
//             throw ProcessError();
//         }
//         // build if everything's ok
//         NGNet* net = new NGNet(nb);
//         net->createChequerBoard(xNo, yNo, xLength, yLength, xAttachLength, yAttachLength);
//         return net;
//     }
//     // random net
//     RandomDistributor<int> neighborDist;
//     neighborDist.add(1, oc.getFloat("rand.neighbor-dist1"));
//     neighborDist.add(2, oc.getFloat("rand.neighbor-dist2"));
//     neighborDist.add(3, oc.getFloat("rand.neighbor-dist3"));
//     neighborDist.add(4, oc.getFloat("rand.neighbor-dist4"));
//     neighborDist.add(5, oc.getFloat("rand.neighbor-dist5"));
//     neighborDist.add(6, oc.getFloat("rand.neighbor-dist6"));
//     NGNet* net = new NGNet(nb);
//     NGRandomNetBuilder randomNet(*net,
//                                  DEG2RAD(oc.getFloat("rand.min-angle")),
//                                  oc.getFloat("rand.min-distance"),
//                                  oc.getFloat("rand.max-distance"),
//                                  oc.getFloat("rand.connectivity"),
//                                  oc.getInt("rand.num-tries"),
//                                  neighborDist);
//     randomNet.createNet(oc.getInt("rand.iterations"), oc.getBool("rand.grid"));
//     return net;
// }
#[allow(unused_variables)]
pub fn build_network(nb: &mut NBNetBuilder) -> Ptr<NGNet> {
    todo!("port buildNetwork")
}

/// C++: `bool () checkOptions`
// bool
// checkOptions() {
//     OptionsCont& oc = OptionsCont::getOptions();
//     bool ok = NGFrame::checkOptions();
//     ok &= NBFrame::checkOptions(oc);
//     ok &= NWFrame::checkOptions(oc);
//     ok &= SystemFrame::checkOptions(oc);
//     return ok;
// }
#[allow(unused_variables)]
pub fn check_options() -> bool {
    todo!("port checkOptions")
}

/// C++: `void () fillOptions`
// void
// fillOptions() {
//     OptionsCont& oc = OptionsCont::getOptions();
//     oc.addCallExample("-c <CONFIGURATION>", TL("create net from given configuration"));
//     oc.addCallExample("--grid [grid-network options] -o <OUTPUTFILE>", TL("create grid net"));
//     oc.addCallExample("--spider [spider-network options] -o <OUTPUTFILE>", TL("create spider net"));
//     oc.addCallExample("--rand [random-network options] -o <OUTPUTFILE>", TL("create random net"));
//
//     oc.setAdditionalHelpMessage(" Either \"--grid\", \"--spider\" or \"--rand\" must be supplied.\n  In dependence to these switches other options are used.");
//
//     // insert options sub-topics
//     SystemFrame::addConfigurationOptions(oc); // this subtopic is filled here, too
//     oc.addOptionSubTopic("Grid Network");
//     oc.addOptionSubTopic("Spider Network");
//     oc.addOptionSubTopic("Random Network");
//     oc.addOptionSubTopic("Input");
//     oc.addOptionSubTopic("Output");
//     oc.addOptionSubTopic("Processing");
//     oc.addOptionSubTopic("Building Defaults");
//     oc.addOptionSubTopic("TLS Building");
//     oc.addOptionSubTopic("Edge Removal");
//     oc.addOptionSubTopic("Unregulated Nodes");
//     oc.addOptionSubTopic("Junctions");
//     oc.addOptionSubTopic("Pedestrian");
//     oc.addOptionSubTopic("Bicycle");
//     SystemFrame::addReportOptions(oc); // this subtopic is filled here, too
//
//     NGFrame::fillOptions();
//     NBFrame::fillOptions(oc, true);
//     NWFrame::fillOptions(oc, true);
//     RandHelper::insertRandOptions(oc);
//
//     oc.doRegister("tls.discard-simple", new Option_Bool(false));
//     oc.addDescription("tls.discard-simple", "TLS Building", "Does not instantiate traffic lights at geometry-like nodes");
//
//     oc.doRegister("railway.signal.permit-unsignalized", new Option_StringVector({"tram", "cable_car"}));
//     oc.addDescription("railway.signal.permit-unsignalized", "TLS Building", TL("List rail classes that may run without rail signals"));
// }
#[allow(unused_variables)]
pub fn fill_options() {
    todo!("port fillOptions")
}

