//! Port of netgen/netgen_main.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use crate::netbuild::nb_frame::NBFrame;
use crate::netwrite::nw_frame::NWFrame;
use crate::netgen::ng_frame::NGFrame;
use crate::netgen::ng_random_net_builder::NGRandomNetBuilder;
use utils::common::msg_handler::{write_error, write_warning};
use utils::common::std_defs::SUMO_CONST_LANE_WIDTH;
use utils::common::to_string::f64_to_string;
use utils::distribution::random_distributor::RandomDistributor;
use utils::geom::geom_helper::deg2rad;
use utils::geom::POSITION_EPS;
use utils::options::options_cont::{get_bool, get_float, get_int, is_default, with_options};

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
    let laneWidth: f64 = if is_default("default.lanewidth") { SUMO_CONST_LANE_WIDTH } else { get_float("default.lanewidth") };
    let mut minLength: f64 = ((get_int("default.lanenumber") + get_int("turn-lanes")) * 2) as f64 * laneWidth + get_float("default.junctions.radius") * 2.0 + POSITION_EPS;
    // spider-net
    if get_bool("spider") {
        // check values
        let mut hadError = false;
        if get_int("spider.arm-number") < 3 {
            write_error("Spider networks need at least 3 arms.");
            hadError = true;
        }
        if get_int("spider.arm-number") > 4 && !get_bool("spider.omit-center") {
            write_warning("Spider networks with many arms should use option spider.omit-center");
        }
        if get_int("spider.circle-number") < 1 {
            write_error("Spider networks need at least one circle.");
            hadError = true;
        }
        minLength = minLength.max(laneWidth * 2.0 * (std::cmp::max(get_int("spider.arm-number"), 3) as f64) / (2.0 * std::f64::consts::PI));
        if get_float("spider.space-radius") < POSITION_EPS {
            write_error(&format!("The radius of spider networks must be at least {}", f64_to_string(POSITION_EPS)));
            hadError = true;
        } else if get_float("spider.space-radius") < minLength {
            write_warningf!("The radius of spider networks should be at least % for the given lanenumber, lanewidth and junction radius", f64_to_string(minLength));
        }
        if hadError {
            panic!("{}", ProcessError::new(""));
        }
        // build if everything's ok
        let net: Ptr<NGNet> = Ptr::from_raw(Box::into_raw(Box::new(NGNet::new(nb))));
        net.m().create_spider_web(get_int("spider.arm-number"), get_int("spider.circle-number"), get_float("spider.space-radius"), !get_bool("spider.omit-center"), get_float("spider.attach-length"));
        return net;
    }
    // grid-net
    if get_bool("grid") {
        // get options
        let mut xNo: i32 = get_int("grid.x-number");
        let mut yNo: i32 = get_int("grid.y-number");
        let mut xLength: f64 = get_float("grid.x-length");
        let mut yLength: f64 = get_float("grid.y-length");
        let mut xAttachLength: f64 = get_float("grid.x-attach-length");
        let mut yAttachLength: f64 = get_float("grid.y-attach-length");
        if is_default("grid.x-number") && !is_default("grid.number") {
            xNo = get_int("grid.number");
        }
        if is_default("grid.y-number") && !is_default("grid.number") {
            yNo = get_int("grid.number");
        }
        if is_default("grid.x-length") && !is_default("grid.length") {
            xLength = get_float("grid.length");
        }
        if is_default("grid.y-length") && !is_default("grid.length") {
            yLength = get_float("grid.length");
        }
        if is_default("grid.x-attach-length") && !is_default("grid.attach-length") {
            xAttachLength = get_float("grid.attach-length");
        }
        if is_default("grid.y-attach-length") && !is_default("grid.attach-length") {
            yAttachLength = get_float("grid.attach-length");
        }
        // check values
        let mut hadError = false;
        if xNo < 1 || yNo < 1 || (xAttachLength == 0.0 && yAttachLength == 0.0 && (xNo < 2 && yNo < 2)) {
            write_error("The number of nodes must be positive and at least 2 in one direction if there are no attachments.");
            hadError = true;
        }
        let minAttachLength: f64 = minLength / 2.0 + POSITION_EPS / 2.0;
        if xLength < POSITION_EPS || yLength < POSITION_EPS {
            write_error(&format!("The distance between nodes must be at least {}", f64_to_string(POSITION_EPS)));
            hadError = true;
        } else if xLength < minLength || yLength < minLength {
            write_warningf!("The distance between nodes should be at least % for the given lanenumber, lanewidth and junction radius", f64_to_string(minLength));
        }
        if xAttachLength != 0.0 && xAttachLength < POSITION_EPS {
            write_error(&format!("The length of attached streets must be at least {}", f64_to_string(POSITION_EPS)));
            hadError = true;
        } else if xAttachLength != 0.0 && xAttachLength < minAttachLength {
            write_warningf!("The length of attached streets should be at least % for the given lanenumber, lanewidth and junction radius", f64_to_string(minAttachLength));
        } else if yAttachLength != 0.0 && yAttachLength < POSITION_EPS {
            write_error(&format!("The length of attached streets must be at least {}", f64_to_string(POSITION_EPS)));
            hadError = true;
        } else if yAttachLength != 0.0 && yAttachLength < minAttachLength {
            write_warningf!("The length of attached streets should be at least % for the given lanenumber, lanewidth and junction radius", f64_to_string(minAttachLength));
        }
        if hadError {
            panic!("{}", ProcessError::new(""));
        }
        // build if everything's ok
        let net: Ptr<NGNet> = Ptr::from_raw(Box::into_raw(Box::new(NGNet::new(nb))));
        net.m().create_chequer_board(xNo, yNo, xLength, yLength, xAttachLength, yAttachLength);
        return net;
    }
    // random net
    let mut neighborDist: RandomDistributor<i32> = RandomDistributor::<i32>::default();
    neighborDist.add(1, get_float("rand.neighbor-dist1"), true);
    neighborDist.add(2, get_float("rand.neighbor-dist2"), true);
    neighborDist.add(3, get_float("rand.neighbor-dist3"), true);
    neighborDist.add(4, get_float("rand.neighbor-dist4"), true);
    neighborDist.add(5, get_float("rand.neighbor-dist5"), true);
    neighborDist.add(6, get_float("rand.neighbor-dist6"), true);
    let net: Ptr<NGNet> = Ptr::from_raw(Box::into_raw(Box::new(NGNet::new(nb))));
    let mut randomNet = NGRandomNetBuilder::new(net.m(), deg2rad(get_float("rand.min-angle")), get_float("rand.min-distance"), get_float("rand.max-distance"), get_float("rand.connectivity"), get_int("rand.num-tries"), &neighborDist);
    randomNet.create_net(get_int("rand.iterations"), get_bool("rand.grid"));
    net
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
    let mut ok = NGFrame::check_options();
    with_options(|oc| {
        ok &= NBFrame::check_options(oc);
        ok &= NWFrame::check_options(oc);
        ok &= utils::common::system_frame::check_options(oc);
    });
    ok
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
    with_options(|oc| {
        oc.add_call_example("-c <CONFIGURATION>", "create net from given configuration");
        oc.add_call_example("--grid [grid-network options] -o <OUTPUTFILE>", "create grid net");
        oc.add_call_example("--spider [spider-network options] -o <OUTPUTFILE>", "create spider net");
        oc.add_call_example("--rand [random-network options] -o <OUTPUTFILE>", "create random net");

        oc.set_additional_help_message(" Either \"--grid\", \"--spider\" or \"--rand\" must be supplied.\n  In dependence to these switches other options are used.");

        // insert options sub-topics
        utils::common::system_frame::add_configuration_options(oc);
        oc.add_option_sub_topic("Grid Network");
        oc.add_option_sub_topic("Spider Network");
        oc.add_option_sub_topic("Random Network");
        oc.add_option_sub_topic("Input");
        oc.add_option_sub_topic("Output");
        oc.add_option_sub_topic("Processing");
        oc.add_option_sub_topic("Building Defaults");
        oc.add_option_sub_topic("TLS Building");
        oc.add_option_sub_topic("Edge Removal");
        oc.add_option_sub_topic("Unregulated Nodes");
        oc.add_option_sub_topic("Junctions");
        oc.add_option_sub_topic("Pedestrian");
        oc.add_option_sub_topic("Bicycle");
        utils::common::system_frame::add_report_options(oc);
    });

    NGFrame::fill_options();
    with_options(|oc| {
        NBFrame::fill_options(oc, true);
        NWFrame::fill_options(oc, true);
        utils::common::rand_helper::insert_rand_options(oc);

        oc.do_register("tls.discard-simple", SumoOption::bool(false));
        oc.add_description("tls.discard-simple", "TLS Building", "Does not instantiate traffic lights at geometry-like nodes");

        oc.do_register("railway.signal.permit-unsignalized", SumoOption::string_vector_with(vec!["tram".to_string(), "cable_car".to_string()]));
        oc.add_description("railway.signal.permit-unsignalized", "TLS Building", "List rail classes that may run without rail signals");
    });
}

