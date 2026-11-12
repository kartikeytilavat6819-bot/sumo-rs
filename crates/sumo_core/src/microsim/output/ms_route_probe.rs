//! Port of microsim/output/MSRouteProbe.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// Writes routes of vehicles passing a certain edge
/// C++ class `MSRouteProbe` (microsim/output/MSRouteProbe.h)
#[repr(C)]
pub struct MSRouteProbe {
    /// C++ base class `MSDetectorFileOutput`
    pub base: MSDetectorFileOutput,
    /// C++ base class `MSMoveReminder`
    pub base2: MSMoveReminder,
    pub myDistID: String,
    pub myLastID: String,
    /// The previous distribution of routes (probability->route)
    pub myLastRouteDistribution: Ptr<RandomDistributor<ConstMSRoutePtr>>,
    /// The current distribution of routes (probability->route)
    pub myCurrentRouteDistribution: Ptr<RandomDistributor<ConstMSRoutePtr>>,
    /// the edge of this route probe
    pub myEdge: Ptr<MSEdge>,
}

impl std::ops::Deref for MSRouteProbe {
    type Target = MSDetectorFileOutput;
    fn deref(&self) -> &MSDetectorFileOutput {
        &self.base
    }
}
impl std::ops::DerefMut for MSRouteProbe {
    fn deref_mut(&mut self) -> &mut MSDetectorFileOutput {
        &mut self.base
    }
}

impl MSRouteProbe {
    /// C++: `void MSRouteProbe::MSRouteProbe(const std::string &, const MSEdge *, const std::string &, const std::string &, const std::string &)`
    /// Constructor
    // MSRouteProbe::MSRouteProbe(const std::string& id, const MSEdge* edge, const std::string& distID, const std::string& lastID,
    //                            const std::string& vTypes) :
    //     MSDetectorFileOutput(id, vTypes), MSMoveReminder(id),
    //     myDistID(distID),
    //     myLastID(lastID),
    //     myLastRouteDistribution(nullptr),
    //     myCurrentRouteDistribution(nullptr),
    //     myEdge(edge) {
    //     if (MSGlobals::gUseMesoSim) {
    //         MESegment* seg = MSGlobals::gMesoNet->getSegmentForEdge(*edge);
    //         while (seg != nullptr) {
    //             seg->addDetector(this);
    //             seg = seg->getNextSegment();
    //         }
    //         return;
    //     }
    //     for (std::vector<MSLane*>::const_iterator it = edge->getLanes().begin(); it != edge->getLanes().end(); ++it) {
    //         (*it)->addMoveReminder(this);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn new(id: &str, edge: Ptr<MSEdge>, distID: &str, lastID: &str, vTypes: &str) -> Self {
        todo!("port MSRouteProbe::MSRouteProbe")
    }

    /// C++: `void MSRouteProbe::~MSRouteProbe() noexcept` [virtual]
    /// Destructor
    // MSRouteProbe::~MSRouteProbe() {
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MSRouteProbe::~MSRouteProbe")
    }

    /// C++: `bool MSRouteProbe::notifyEnter(SUMOTrafficObject &, MSMoveReminder::Notification, const MSLane *)` [virtual]
    /// Returns whether the vehicle shall be aware of this entry
    // bool
    // MSRouteProbe::notifyEnter(SUMOTrafficObject& veh, MSMoveReminder::Notification reason, const MSLane* /* enteredLane */) {
    //     if (!vehicleApplies(veh)) {
    //         return false;
    //     }
    //     if (reason != MSMoveReminder::NOTIFICATION_SEGMENT && reason != MSMoveReminder::NOTIFICATION_LANE_CHANGE) {
    //         SUMOVehicle* vehicle = dynamic_cast<SUMOVehicle*>(&veh);
    //         if (vehicle != nullptr) {
    //             initDistributions();
    //             myCurrentRouteDistribution->add(vehicle->getRoutePtr(), 1.);
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn notify_enter(&mut self, veh: &mut SUMOTrafficObject, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
        todo!("port MSRouteProbe::notifyEnter")
    }

    /// C++: `void MSRouteProbe::writeXMLOutput(OutputDevice &, SUMOTime, SUMOTime)` [virtual]
    /// Writes values into the given stream
    // void
    // MSRouteProbe::writeXMLOutput(OutputDevice& dev,
    //                              SUMOTime startTime, SUMOTime stopTime) {
    //     if (myCurrentRouteDistribution && myCurrentRouteDistribution->getOverallProb() > 0) {
    //         dev.openTag(SUMO_TAG_ROUTE_DISTRIBUTION).writeAttr(SUMO_ATTR_ID, getID() + "_" + time2string(startTime, false));
    //         const std::vector<ConstMSRoutePtr>& routes = myCurrentRouteDistribution->getVals();
    //         const std::vector<double>& probs = myCurrentRouteDistribution->getProbs();
    //         for (int j = 0; j < (int)routes.size(); ++j) {
    //             dev.openTag(SUMO_TAG_ROUTE).writeAttr(SUMO_ATTR_ID, routes[j]->getID() + "_" + time2string(startTime, false))
    //             .writeAttr(SUMO_ATTR_EDGES, toString(routes[j]->getEdges())).writeAttr(SUMO_ATTR_PROB, probs[j]).closeTag();
    //         }
    //         dev.closeTag();
    //         if (myLastRouteDistribution != 0) {
    //             MSRoute::checkDist(myLastID);
    //         }
    //         myLastRouteDistribution = myCurrentRouteDistribution;
    //         myLastID = myDistID;
    //         myDistID = getID() + "_" + toString(stopTime);
    //         myCurrentRouteDistribution = new RandomDistributor<ConstMSRoutePtr>();
    //         MSRoute::dictionary(myDistID, myCurrentRouteDistribution, false);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn write_xml_output(&mut self, dev: &mut OutputDevice, startTime: SUMOTime, stopTime: SUMOTime) {
        todo!("port MSRouteProbe::writeXMLOutput")
    }

    /// C++: `void MSRouteProbe::writeXMLDetectorProlog(OutputDevice &) const` [const, virtual]
    /// Opens the XML-output using "detector" as root element
    // void
    // MSRouteProbe::writeXMLDetectorProlog(OutputDevice& dev) const {
    //     dev.writeXMLHeader("routes", "routes_file.xsd");
    // }
    #[allow(unused_variables)]
    pub fn write_xml_detector_prolog(&self, dev: &mut OutputDevice) {
        todo!("port MSRouteProbe::writeXMLDetectorProlog")
    }

    /// C++: `ConstMSRoutePtr MSRouteProbe::sampleRoute(bool) const` [const]
    // ConstMSRoutePtr
    // MSRouteProbe::sampleRoute(bool last) const {
    //     if (myLastRouteDistribution == 0 || !last) {
    //         if (myCurrentRouteDistribution && myCurrentRouteDistribution->getOverallProb() > 0) {
    //             return myCurrentRouteDistribution->get();
    //         }
    //         return nullptr;
    //     }
    //     return myLastRouteDistribution->get();
    // }
    #[allow(unused_variables)]
    pub fn sample_route(&self, last: bool) -> ConstMSRoutePtr {
        todo!("port MSRouteProbe::sampleRoute")
    }

    /// C++: `const MSEdge * MSRouteProbe::getEdge()`
    // const MSEdge* getEdge() {
    //         return myEdge;
    //     }
    #[allow(unused_variables)]
    pub fn get_edge(&mut self) -> Ptr<MSEdge> {
        todo!("port MSRouteProbe::getEdge")
    }

    /// C++: `void MSRouteProbe::clearState(SUMOTime)` [virtual]
    // void
    // MSRouteProbe::clearState(SUMOTime step) {
    //     UNUSED_PARAMETER(step);
    //     myCurrentRouteDistribution = nullptr;
    //     myLastRouteDistribution = nullptr;
    // }
    #[allow(unused_variables)]
    pub fn clear_state(&mut self, step: SUMOTime) {
        todo!("port MSRouteProbe::clearState")
    }

    /// C++: `void MSRouteProbe::initDistributions()` [private]
    // void
    // MSRouteProbe::initDistributions() {
    //     if (myCurrentRouteDistribution == nullptr) {
    //         myCurrentRouteDistribution = MSRoute::distDictionary(myDistID);
    //         if (myCurrentRouteDistribution == 0) {
    //             myCurrentRouteDistribution = new RandomDistributor<ConstMSRoutePtr>();
    //             MSRoute::dictionary(myDistID, myCurrentRouteDistribution, false);
    //         }
    //         myLastRouteDistribution = MSRoute::distDictionary(myLastID);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn init_distributions(&mut self) {
        todo!("port MSRouteProbe::initDistributions")
    }

    /// C++: `void MSRouteProbe::MSRouteProbe(const MSRouteProbe &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSRouteProbe) -> Self {
        todo!("port MSRouteProbe::MSRouteProbe")
    }

    /// C++: `MSRouteProbe & MSRouteProbe::operator=(const MSRouteProbe &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSRouteProbe) -> &mut MSRouteProbe {
        todo!("port MSRouteProbe::operator=")
    }

}

