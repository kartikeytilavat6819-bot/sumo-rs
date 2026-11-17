//! Port of microsim/output/MSRouteProbe.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use super::ms_detector_file_output::{DEFAULT_MSDETECTOR_FILE_OUTPUT_VT, MSDetectorFileOutputVt};

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

fn ms_route_probe_notify_enter(mr: &mut MSMoveReminder, veh: &mut SUMOTrafficObject, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
    Ptr::from_raw((mr as *mut MSMoveReminder as *mut u8).wrapping_sub(std::mem::offset_of!(MSRouteProbe, base2)) as *mut MSRouteProbe).m().notify_enter(veh, reason, enteredLane)
}
/// Virtual table of MSRouteProbe (MSMoveReminder override: notifyEnter; notifyMove and notifyLeave are inherited)
static MS_ROUTE_PROBE_MSMOVE_REMINDER_VT: MSMoveReminderVt = MSMoveReminderVt {
    notify_enter: ms_route_probe_notify_enter,
    notify_move: default_notify_move,
    notify_leave: default_notify_leave,
};

fn ms_route_probe_write_xml_output(d: &mut MSDetectorFileOutput, dev: &mut OutputDevice, startTime: SUMOTime, stopTime: SUMOTime) {
    Ptr::from_mut(d).cast::<MSRouteProbe>().m().write_xml_output(dev, startTime, stopTime)
}
fn ms_route_probe_write_xml_detector_prolog(d: &MSDetectorFileOutput, dev: &mut OutputDevice) {
    Ptr::new(d).cast::<MSRouteProbe>().m().write_xml_detector_prolog(dev)
}

/// Virtual table of MSRouteProbe as MSDetectorFileOutput (overrides writeXMLOutput and writeXMLDetectorProlog)
static MS_ROUTE_PROBE_DETECTOR_FILE_OUTPUT_VT: MSDetectorFileOutputVt = MSDetectorFileOutputVt {
    write_xml_output: ms_route_probe_write_xml_output,
    write_xml_detector_prolog: ms_route_probe_write_xml_detector_prolog,
    ..DEFAULT_MSDETECTOR_FILE_OUTPUT_VT
};

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
    pub fn new(id: &str, edge: Ptr<MSEdge>, distID: &str, lastID: &str, vTypes: &str) -> Box<Self> {
        let mut base = MSDetectorFileOutput::new(id, vTypes, "", 0).unwrap_or_else(|e| panic!("{e}"));
        base.vt = &MS_ROUTE_PROBE_DETECTOR_FILE_OUTPUT_VT;
        let base2 = MSMoveReminder {
            vt: &MS_ROUTE_PROBE_MSMOVE_REMINDER_VT,
            ..MSMoveReminder::new(id, Ptr::null(), true)
        };
        let mut b = Box::new(MSRouteProbe {
            base,
            base2,
            myDistID: distID.to_string(),
            myLastID: lastID.to_string(),
            myLastRouteDistribution: Ptr::null(),
            myCurrentRouteDistribution: Ptr::null(),
            myEdge: edge,
        });
        // C++ registers `this` on each meso segment of the edge, or on each lane otherwise
        if crate::microsim::ms_globals::use_meso_sim() {
            let mut seg = crate::microsim::ms_globals::meso_net().m().get_segment_for_edge(edge.m(), 0.);
            while !seg.is_null() {
                seg.m().add_detector(Ptr::from_mut(&mut b.base2), -1);
                seg = seg.m().get_next_segment();
            }
        } else {
            for lane in edge.get_lanes().iter() {
                lane.m().add_move_reminder(Ptr::from_mut(&mut b.base2), true);
            }
        }
        b
    }

    /// C++: `void MSRouteProbe::~MSRouteProbe() noexcept` [virtual]
    /// Destructor
    // MSRouteProbe::~MSRouteProbe() {
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor has an empty body; the members are released by Rust drop glue
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
        if !self.vehicle_applies(veh) {
            return false;
        }
        if reason != MSMoveReminder_Notification::NOTIFICATION_SEGMENT && reason != MSMoveReminder_Notification::NOTIFICATION_LANE_CHANGE {
            // dynamic_cast<SUMOVehicle*>: isVehicle() is the type check
            if veh.is_vehicle() {
                let vehicle: Ptr<SUMOVehicle> = Ptr::from_mut(veh).cast::<SUMOVehicle>();
                self.init_distributions();
                self.myCurrentRouteDistribution.m().add(vehicle.get_route_ptr(), 1., true);
            }
        }
        false
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
        let cur = self.myCurrentRouteDistribution;
        if !cur.is_null() && cur.get_overall_prob() > 0. {
            dev.open_tag_id(SUMO_TAG_ROUTE_DISTRIBUTION).write_attr(SUMO_ATTR_ID, &format!("{}_{}", self.get_id(), time2string_hr(startTime, false)));
            let routes = cur.get_vals();
            let probs = cur.get_probs();
            for j in 0..routes.len() {
                let edges: Vec<&str> = routes[j].get_edges().iter().map(|e| e.get_id()).collect();
                dev.open_tag_id(SUMO_TAG_ROUTE).write_attr(SUMO_ATTR_ID, &format!("{}_{}", routes[j].get_id(), time2string_hr(startTime, false))).write_attr(SUMO_ATTR_EDGES, &edges.join(" ")).write_attr(SUMO_ATTR_PROB, &probs[j]).close_tag("");
            }
            dev.close_tag("");
            if !self.myLastRouteDistribution.is_null() {
                MSRoute::check_dist(&self.myLastID);
            }
            self.myLastRouteDistribution = cur;
            self.myLastID = self.myDistID.clone();
            self.myDistID = format!("{}_{}", self.get_id(), stopTime);
            self.myCurrentRouteDistribution = Ptr::from_raw(Box::into_raw(Box::new(RandomDistributor::<ConstMSRoutePtr>::default())));
            MSRoute::dictionary_2(&self.myDistID, self.myCurrentRouteDistribution, false);
        }
    }

    /// C++: `void MSRouteProbe::writeXMLDetectorProlog(OutputDevice &) const` [const, virtual]
    /// Opens the XML-output using "detector" as root element
    // void
    // MSRouteProbe::writeXMLDetectorProlog(OutputDevice& dev) const {
    //     dev.writeXMLHeader("routes", "routes_file.xsd");
    // }
    #[allow(unused_variables)]
    pub fn write_xml_detector_prolog(&self, dev: &mut OutputDevice) {
        dev.write_xml_header("routes", "routes_file.xsd", BTreeMap::new(), true);
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
    pub fn sample_route(&self, last: bool) -> Option<ConstMSRoutePtr> {
        // C++ returns nullptr when there is no distribution; ConstMSRoutePtr is an Rc, hence the Option
        if self.myLastRouteDistribution.is_null() || !last {
            if !self.myCurrentRouteDistribution.is_null() && self.myCurrentRouteDistribution.get_overall_prob() > 0. {
                return Some((*self.myCurrentRouteDistribution).get(None).unwrap_or_else(|e| panic!("{e}")));
            }
            return None;
        }
        Some((*self.myLastRouteDistribution).get(None).unwrap_or_else(|e| panic!("{e}")))
    }

    /// C++: `const MSEdge * MSRouteProbe::getEdge()`
    // const MSEdge* getEdge() {
    //         return myEdge;
    //     }
    #[allow(unused_variables)]
    pub fn get_edge(&mut self) -> Ptr<MSEdge> {
        self.myEdge
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
        self.myCurrentRouteDistribution = Ptr::null();
        self.myLastRouteDistribution = Ptr::null();
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
        if self.myCurrentRouteDistribution.is_null() {
            self.myCurrentRouteDistribution = MSRoute::dist_dictionary(&self.myDistID);
            if self.myCurrentRouteDistribution.is_null() {
                self.myCurrentRouteDistribution = Ptr::from_raw(Box::into_raw(Box::new(RandomDistributor::<ConstMSRoutePtr>::default())));
                MSRoute::dictionary_2(&self.myDistID, self.myCurrentRouteDistribution, false);
            }
            self.myLastRouteDistribution = MSRoute::dist_dictionary(&self.myLastID);
        }
    }

    /// C++: `void MSRouteProbe::MSRouteProbe(const MSRouteProbe &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSRouteProbe) -> Self {
        panic!("MSRouteProbe copy constructor is private and invalidated in SUMO")
    }

    /// C++: `MSRouteProbe & MSRouteProbe::operator=(const MSRouteProbe &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSRouteProbe) -> &mut MSRouteProbe {
        panic!("MSRouteProbe assignment operator is private and invalidated in SUMO")
    }

}

