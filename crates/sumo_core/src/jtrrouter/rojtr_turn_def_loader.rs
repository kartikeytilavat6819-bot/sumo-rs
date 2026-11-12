//! Port of jtrrouter/ROJTRTurnDefLoader.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// Loader for the of turning percentages and source/sink definitions
/// C++ class `ROJTRTurnDefLoader` (jtrrouter/ROJTRTurnDefLoader.h)
#[repr(C)]
pub struct ROJTRTurnDefLoader {
    /// C++ base class `SUMOSAXHandler`
    pub base: Unmapped /* SUMOSAXHandler */,
    /// The network to set the information into
    pub myNet: Ptr<RONet>,
    /// The begin and the end of the current interval
    pub myIntervalBegin: f64,
    /// The begin and the end of the current interval
    pub myIntervalEnd: f64,
    /// The current incoming edge the turning probabilities are set into
    pub myEdge: Ptr<ROJTREdge>,
    /// whether all sources are sinks
    pub mySourcesAreSinks: bool,
    /// whether upstream flows should be discounted from source flows
    pub myDiscountSources: bool,
    /// whether the warning for the deprecated format has been issued
    pub myHaveWarnedAboutDeprecatedFormat: bool,
}

impl std::ops::Deref for ROJTRTurnDefLoader {
    type Target = Unmapped /* SUMOSAXHandler */;
    fn deref(&self) -> &Unmapped /* SUMOSAXHandler */ {
        &self.base
    }
}
impl std::ops::DerefMut for ROJTRTurnDefLoader {
    fn deref_mut(&mut self) -> &mut Unmapped /* SUMOSAXHandler */ {
        &mut self.base
    }
}

impl ROJTRTurnDefLoader {
    /// C++: `void ROJTRTurnDefLoader::ROJTRTurnDefLoader(RONet &)`
    /// Constructor
    // ROJTRTurnDefLoader::ROJTRTurnDefLoader(RONet& net) :
    //     SUMOSAXHandler("turn-ratio-file"), myNet(net),
    //     myIntervalBegin(0), myIntervalEnd(STEPS2TIME(SUMOTime_MAX)),
    //     myEdge(nullptr),
    //     mySourcesAreSinks(OptionsCont::getOptions().getBool("sources-are-sinks")),
    //     myDiscountSources(OptionsCont::getOptions().getBool("discount-sources")),
    //     myHaveWarnedAboutDeprecatedFormat(false)
    // {}
    #[allow(unused_variables)]
    pub fn new(net: &mut RONet) -> Self {
        todo!("port ROJTRTurnDefLoader::ROJTRTurnDefLoader")
    }

    /// C++: `void ROJTRTurnDefLoader::~ROJTRTurnDefLoader() noexcept` [virtual]
    /// Destructor
    // ROJTRTurnDefLoader::~ROJTRTurnDefLoader() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port ROJTRTurnDefLoader::~ROJTRTurnDefLoader")
    }

    /// C++: `void ROJTRTurnDefLoader::myStartElement(int, const SUMOSAXAttributes &)` [virtual, protected]
    /// Called on the opening of a tag;
    // void
    // ROJTRTurnDefLoader::myStartElement(int element,
    //                                    const SUMOSAXAttributes& attrs) {
    //     bool ok = true;
    //     switch (element) {
    //         case SUMO_TAG_INTERVAL:
    //             myIntervalBegin = attrs.get<double>(SUMO_ATTR_BEGIN, nullptr, ok);
    //             myIntervalEnd = attrs.get<double>(SUMO_ATTR_END, nullptr, ok);
    //             break;
    //         case SUMO_TAG_FROMEDGE:
    //             if (!myHaveWarnedAboutDeprecatedFormat) {
    //                 myHaveWarnedAboutDeprecatedFormat = true;
    //                 WRITE_WARNINGF(TL("The turn-file format with elements %, % is deprecated, please use % instead."),
    //                                toString(SUMO_TAG_FROMEDGE), toString(SUMO_TAG_TOEDGE), toString(SUMO_TAG_EDGEREL));
    //             }
    //             beginFromEdge(attrs);
    //             break;
    //         case SUMO_TAG_TOEDGE:
    //             addToEdge(attrs);
    //             break;
    //         case SUMO_TAG_EDGEREL:
    //             addEdgeRel(attrs);
    //             break;
    //         case SUMO_TAG_SINK:
    //             if (attrs.hasAttribute(SUMO_ATTR_EDGES)) {
    //                 std::string edges = attrs.get<std::string>(SUMO_ATTR_EDGES, nullptr, ok);
    //                 StringTokenizer st(edges, StringTokenizer::WHITECHARS);
    //                 while (st.hasNext()) {
    //                     std::string id = st.next();
    //                     ROEdge* edge = myNet.getEdge(id);
    //                     if (edge == nullptr) {
    //                         throw ProcessError(TLF("The edge '%' declared as a sink is not known.", id));
    //                     }
    //                     edge->setSink();
    //                 }
    //             }
    //             break;
    //         case SUMO_TAG_FLOW: {
    //             const std::string flowID = attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
    //             if (attrs.hasAttribute(SUMO_ATTR_FROM)) {
    //                 const std::string edgeID = attrs.get<std::string>(SUMO_ATTR_FROM, nullptr, ok);
    //                 ROEdge* edge = myNet.getEdge(edgeID);
    //                 if (edge == nullptr) {
    //                     throw ProcessError(TLF("The from-edge '%' in flow '%' is not known.", edgeID, flowID));
    //                 }
    //                 if (mySourcesAreSinks) {
    //                     edge->setSink();
    //                 }
    //                 if (myDiscountSources) {
    //                     SUMOVehicleParameter* pars = SUMOVehicleParserHelper::parseFlowAttributes(SUMO_TAG_FLOW, attrs, true, true, 0, TIME2STEPS(3600 * 24));
    //                     long long int numVehs = 0;
    //                     if (pars->repetitionProbability > 0) {
    //                         numVehs = int(STEPS2TIME(pars->repetitionEnd - pars->depart) * pars->repetitionProbability);
    //                     } else {
    //                         numVehs = pars->repetitionNumber;
    //                     }
    //                     delete pars;
    //                     static_cast<ROJTREdge*>(edge)->changeSourceFlow(numVehs);
    //                 }
    //             } else {
    //                 WRITE_WARNINGF(TL("Ignoring flow '%' without 'from'"), flowID);
    //             }
    //             break;
    //         }
    //         case SUMO_TAG_SOURCE:
    //             if (attrs.hasAttribute(SUMO_ATTR_EDGES)) {
    //                 std::string edges = attrs.get<std::string>(SUMO_ATTR_EDGES, nullptr, ok);
    //                 StringTokenizer st(edges, StringTokenizer::WHITECHARS);
    //                 while (st.hasNext()) {
    //                     std::string id = st.next();
    //                     ROEdge* edge = myNet.getEdge(id);
    //                     if (edge == nullptr) {
    //                         throw ProcessError(TLF("The edge '%' declared as a source is not known.", id));
    //                     }
    //                     edge->setSource();
    //                 }
    //             }
    //             break;
    //         default:
    //             break;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) {
        todo!("port ROJTRTurnDefLoader::myStartElement")
    }

    /// C++: `void ROJTRTurnDefLoader::beginFromEdge(const SUMOSAXAttributes &)` [private]
    /// Begins the processing of a incoming edge definition
    // void
    // ROJTRTurnDefLoader::beginFromEdge(const SUMOSAXAttributes& attrs) {
    //     myEdge = nullptr;
    //     bool ok = true;
    //     // get the id, report an error if not given or empty...
    //     std::string id = attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
    //     if (!ok) {
    //         return;
    //     }
    //     //
    //     myEdge = static_cast<ROJTREdge*>(myNet.getEdge(id));
    //     if (myEdge == nullptr) {
    //         WRITE_ERRORF(TL("The edge '%' is not known within the network (within a 'from-edge' tag)."), id);
    //         return;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn begin_from_edge(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port ROJTRTurnDefLoader::beginFromEdge")
    }

    /// C++: `void ROJTRTurnDefLoader::addToEdge(const SUMOSAXAttributes &)` [private]
    /// Parses the probability to use a certain outgoing edge
    // void
    // ROJTRTurnDefLoader::addToEdge(const SUMOSAXAttributes& attrs) {
    //     if (myEdge == nullptr) {
    //         return;
    //     }
    //     bool ok = true;
    //     // get the id, report an error if not given or empty...
    //     std::string id = attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
    //     if (!ok) {
    //         return;
    //     }
    //     //
    //     ROJTREdge* edge = static_cast<ROJTREdge*>(myNet.getEdge(id));
    //     if (edge == nullptr) {
    //         WRITE_ERRORF(TL("The edge '%' is not known within the network (within a 'to-edge' tag)."), id);
    //         return;
    //     }
    //     const double probability = attrs.get<double>(SUMO_ATTR_PROB, id.c_str(), ok);
    //     if (ok) {
    //         if (probability < 0) {
    //             WRITE_ERRORF(TL("'probability' must be positive (in definition of to-edge '%')."), id);
    //         } else {
    //             myEdge->addFollowerProbability(edge, myIntervalBegin, myIntervalEnd, probability);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_to_edge(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port ROJTRTurnDefLoader::addToEdge")
    }

    /// C++: `void ROJTRTurnDefLoader::addEdgeRel(const SUMOSAXAttributes &)` [private]
    /// Parses the probability to use a certain incoming-outgoing edge relation
    // void
    // ROJTRTurnDefLoader::addEdgeRel(const SUMOSAXAttributes& attrs) {
    //     bool ok = true;
    //     // get the id, report an error if not given or empty...
    //     std::string fromID = attrs.get<std::string>(SUMO_ATTR_FROM, nullptr, ok);
    //     std::string toID = attrs.get<std::string>(SUMO_ATTR_TO, nullptr, ok);
    //     double probability = attrs.get<double>(
    //                              attrs.hasAttribute(SUMO_ATTR_COUNT) && !attrs.hasAttribute(SUMO_ATTR_PROB) ? SUMO_ATTR_COUNT : SUMO_ATTR_PROB,
    //                              (fromID + "->" + toID).c_str(), ok);
    //     if (!ok) {
    //         return;
    //     }
    //     //
    //     ROJTREdge* from = static_cast<ROJTREdge*>(myNet.getEdge(fromID));
    //     if (from == nullptr) {
    //         WRITE_ERRORF(TL("The edge '%' is not known."), fromID);
    //         return;
    //     }
    //     ROJTREdge* to = static_cast<ROJTREdge*>(myNet.getEdge(toID));
    //     if (to == nullptr) {
    //         WRITE_ERRORF(TL("The edge '%' is not known."), toID);
    //         return;
    //     }
    //     if (probability < 0) {
    //         WRITE_ERRORF(TL("'probability' must be positive (in edgeRelation from '%' to '%'."), fromID, toID);
    //     } else {
    //         from->addFollowerProbability(to, myIntervalBegin, myIntervalEnd, probability);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_edge_rel(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port ROJTRTurnDefLoader::addEdgeRel")
    }

}

