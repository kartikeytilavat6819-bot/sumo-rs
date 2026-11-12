//! Port of netload/NLEdgeControlBuilder.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// Interface for building edges
/// C++ class `NLEdgeControlBuilder` (netload/NLEdgeControlBuilder.h)
pub struct NLEdgeControlBuilder {
    /// A running number for lane numbering
    pub myCurrentNumericalLaneID: i32,
    /// A running number for edge numbering
    pub myCurrentNumericalEdgeID: i32,
    /// Temporary, internal storage for built edges
    pub myEdges: MSEdgeVector,
    /// pointer to the currently chosen edge
    pub myActiveEdge: Ptr<MSEdge>,
    /// The default stop offset for all lanes belonging to the active edge (this is set if the edge was given a stopOffset child)
    pub myCurrentDefaultStopOffset: StopOffset,
    /// The index of the currently active lane (-1 if none is active)
    pub myCurrentLaneIndex: i32,
    /// pointer to a temporary lane storage
    pub myLaneStorage: Ptr<Vec<Ptr<MSLane>>>,
    /// temporary storage for bidi attributes (to be resolved after loading all edges)
    pub myBidiEdges: BTreeMap<Ptr<MSEdge>, String>, // C++: std::map<MSEdge *, std::string, ComparatorNumericalIdLess>
    pub myOppositeLanes: Vec<(Ptr<MSLane>, String)>,
}

impl NLEdgeControlBuilder {
    /// C++: `void NLEdgeControlBuilder::NLEdgeControlBuilder()`
    /// Constructor
    // NLEdgeControlBuilder::NLEdgeControlBuilder()
    //     : myCurrentNumericalLaneID(0), myCurrentNumericalEdgeID(0), myEdges(0), myCurrentLaneIndex(-1) {
    //     myActiveEdge = (MSEdge*) nullptr;
    //     myLaneStorage = new std::vector<MSLane*>();
    // }
    #[allow(unused_variables)]
    pub fn new() -> Self {
        NLEdgeControlBuilder {
            myCurrentNumericalLaneID: 0,
            myCurrentNumericalEdgeID: 0,
            myEdges: Vec::new(),
            myActiveEdge: Ptr::null(),
            myCurrentDefaultStopOffset: StopOffset::default(),
            myCurrentLaneIndex: -1,
            myLaneStorage: Ptr::from_mut(Box::leak(Box::new(Vec::new()))),
            myBidiEdges: BTreeMap::new(),
            myOppositeLanes: Vec::new(),
        }
    }

    /// C++: `void NLEdgeControlBuilder::~NLEdgeControlBuilder() noexcept` [virtual]
    /// Destructor
    // NLEdgeControlBuilder::~NLEdgeControlBuilder() {
    //     delete myLaneStorage;
    //     if (myActiveEdge != nullptr && MSEdge::dictionary(myActiveEdge->getID()) != myActiveEdge) {
    //         delete myActiveEdge;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        unsafe { drop(Box::from_raw(self.myLaneStorage.as_raw())) };
        if !self.myActiveEdge.is_null() && MSEdge::dictionary_2(self.myActiveEdge.get_id()) != self.myActiveEdge {
            unsafe { drop(Box::from_raw(self.myActiveEdge.as_raw())) };
        }
    }

    /// C++: `void NLEdgeControlBuilder::beginEdgeParsing(const std::string &, const SumoXMLEdgeFunc, const std::string &, const std::string &, const std::string &, int, const std::string &, double)`
    /// Begins building of an MSEdge
    // void
    // NLEdgeControlBuilder::beginEdgeParsing(
    //     const std::string& id, const SumoXMLEdgeFunc function,
    //     const std::string& streetName,
    //     const std::string& edgeType,
    //     const std::string& routingType,
    //     int priority,
    //     const std::string& bidi,
    //     double distance) {
    //     // closeEdge might not have been called because the last edge had an error, so we clear the lane storage
    //     myLaneStorage->clear();
    //     // if the previous edge was broken (never registered in the static dict), remove and delete it
    //     if (myActiveEdge != nullptr && MSEdge::dictionary(myActiveEdge->getID()) != myActiveEdge) {
    //         myEdges.pop_back();
    //         delete myActiveEdge;
    //         myActiveEdge = nullptr;
    //     }
    //     if (MSEdge::dictionary(id) != nullptr) {
    //         throw InvalidArgument("Another edge with the id '" + id + "' exists.");
    //     }
    //     myActiveEdge = buildEdge(id, function, streetName, edgeType, routingType, priority, distance);
    //     myEdges.push_back(myActiveEdge);
    //     if (bidi != "") {
    //         myBidiEdges[myActiveEdge] = bidi;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn begin_edge_parsing(&mut self, id: &str, function: SumoXMLEdgeFunc, streetName: &str, edgeType: &str, routingType: &str, priority: i32, bidi: &str, distance: f64) {
        self.myLaneStorage.m().clear();
        if !self.myActiveEdge.is_null() && MSEdge::dictionary_2(self.myActiveEdge.get_id()) != self.myActiveEdge {
            self.myEdges.pop();
            unsafe { drop(Box::from_raw(self.myActiveEdge.as_raw())) };
            self.myActiveEdge = Ptr::null();
        }
        if !MSEdge::dictionary_2(id).is_null() {
            std::panic::panic_any(ProcessError::InvalidArgument(format!("Another edge with the id '{}' exists.", id)));
        }
        self.myActiveEdge = self.build_edge(id, function, streetName, edgeType, routingType, priority, distance);
        self.myEdges.push(self.myActiveEdge);
        if bidi != "" {
            self.myBidiEdges.insert(self.myActiveEdge, bidi.to_string());
        }
    }

    /// C++: `MSLane * NLEdgeControlBuilder::addLane(const std::string &, double, double, double, const PositionVector &, double, SVCPermissions, SVCPermissions, SVCPermissions, int, bool, const std::string &, const PositionVector &)` [virtual]
    /// Adds a lane to the current edge
    // MSLane*
    // NLEdgeControlBuilder::addLane(const std::string& id,
    //                               double maxSpeed, double friction, double length,
    //                               const PositionVector& shape, double width,
    //                               SVCPermissions permissions,
    //                               SVCPermissions changeLeft, SVCPermissions changeRight,
    //                               int index, bool isRampAccel,
    //                               const std::string& type,
    //                               const PositionVector& outlineShape) {
    //     MSLane* lane = new MSLane(id, maxSpeed, friction, length, myActiveEdge, myCurrentNumericalLaneID++, shape, width, permissions, changeLeft, changeRight, index, isRampAccel, type, outlineShape);
    //     myLaneStorage->push_back(lane);
    //     myCurrentLaneIndex = index;
    //     return lane;
    // }
    #[allow(unused_variables)]
    pub fn add_lane(&mut self, id: &str, maxSpeed: f64, friction: f64, length: f64, shape: &PositionVector, width: f64, permissions: SVCPermissions, changeLeft: SVCPermissions, changeRight: SVCPermissions, index: i32, isRampAccel: bool, type_: &str, outlineShape: &PositionVector) -> Ptr<MSLane> {
        let numericalID = self.myCurrentNumericalLaneID;
        self.myCurrentNumericalLaneID += 1;
        let lane = Ptr::from_mut(Box::leak(Box::new(MSLane::new(id, maxSpeed, friction, length, self.myActiveEdge, numericalID, shape, width, permissions, changeLeft, changeRight, index, isRampAccel, type_, outlineShape))));
        self.myLaneStorage.m().push(lane);
        self.myCurrentLaneIndex = index;
        lane
    }

    /// C++: `void NLEdgeControlBuilder::addStopOffsets(const StopOffset &)`
    /// process a stopOffset element (originates either from the active edge or lane).
    // void
    // NLEdgeControlBuilder::addStopOffsets(const StopOffset& stopOffset) {
    //     if (myCurrentLaneIndex == -1) {
    //         setDefaultStopOffset(stopOffset);
    //     } else {
    //         updateCurrentLaneStopOffset(stopOffset);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_stop_offsets(&mut self, stopOffsets: &StopOffset) {
        if self.myCurrentLaneIndex == -1 {
            self.set_default_stop_offset(stopOffsets);
        } else {
            self.update_current_lane_stop_offset(stopOffsets);
        }
    }

    /// C++: `std::string NLEdgeControlBuilder::reportCurrentEdgeOrLane() const` [const]
    /// Return info about currently processed edge or lane
    // std::string
    // NLEdgeControlBuilder::reportCurrentEdgeOrLane() const {
    //     std::stringstream ss;
    //     if (myCurrentLaneIndex != -1) {
    //         ss << "lane " << myCurrentLaneIndex << " of ";
    //     }
    //     ss << "edge '" << myActiveEdge->getID() << "'";
    //     return ss.str();
    // }
    #[allow(unused_variables)]
    pub fn report_current_edge_or_lane(&self) -> String {
        let mut ss = String::new();
        if self.myCurrentLaneIndex != -1 {
            ss += &format!("lane {} of ", self.myCurrentLaneIndex);
        }
        ss += &format!("edge '{}'", self.myActiveEdge.get_id());
        ss
    }

    /// C++: `void NLEdgeControlBuilder::addNeigh(const std::string)` [virtual]
    /// Adds a neighbor to the current lane
    // void
    // NLEdgeControlBuilder::addNeigh(const std::string id) {
    //     myOppositeLanes.push_back({myLaneStorage->back(), id});
    // }
    #[allow(unused_variables)]
    pub fn add_neigh(&mut self, id: String) {
        let lane = *self.myLaneStorage.m().last().unwrap();
        self.myOppositeLanes.push((lane, id));
    }

    /// C++: `MSEdge * NLEdgeControlBuilder::closeEdge()` [virtual]
    /// Closes the building of an edge; The edge is completely described by now and may not be opened again
    // MSEdge*
    // NLEdgeControlBuilder::closeEdge() {
    //     applyDefaultStopOffsetsToLanes();
    //     std::vector<MSLane*>* lanes = new std::vector<MSLane*>();
    //     lanes->reserve(myLaneStorage->size());
    //     copy(myLaneStorage->begin(), myLaneStorage->end(), back_inserter(*lanes));
    //     myLaneStorage->clear();
    //     myActiveEdge->initialize(lanes);
    //     myCurrentDefaultStopOffset.reset();
    //     MSEdge* result = myActiveEdge;
    //     myActiveEdge = nullptr;
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn close_edge(&mut self) -> Ptr<MSEdge> {
        self.apply_default_stop_offsets_to_lanes();
        let lanes: Vec<Ptr<MSLane>> = self.myLaneStorage.m().clone();
        self.myLaneStorage.m().clear();
        self.myActiveEdge.m().initialize(Ptr::from_mut(Box::leak(Box::new(lanes))));
        self.myCurrentDefaultStopOffset.reset();
        let result = self.myActiveEdge;
        self.myActiveEdge = Ptr::null();
        result
    }

    /// C++: `void NLEdgeControlBuilder::closeLane()`
    /// Closes the building of a lane; The edge is completely described by now and may not be opened again
    // void
    // NLEdgeControlBuilder::closeLane() {
    //     myCurrentLaneIndex = -1;
    // }
    #[allow(unused_variables)]
    pub fn close_lane(&mut self) {
        self.myCurrentLaneIndex = -1;
    }

    /// C++: `MSEdgeControl * NLEdgeControlBuilder::build(const MMVersion &)`
    /// builds the MSEdgeControl-class which holds all edges
    // MSEdgeControl*
    // NLEdgeControlBuilder::build(const MMVersion& networkVersion) {
    //     if (MSGlobals::gUseMesoSim && !OptionsCont::getOptions().getBool("meso-lane-queue")) {
    //         MSEdge::setMesoIgnoredVClasses(parseVehicleClasses(OptionsCont::getOptions().getStringVector("meso-ignore-lanes-by-vclass")));
    //     }
    //     // connecting opposite lanes must happen before MSEdge::closeBuilding
    //     for (auto item : myOppositeLanes) {
    //         MSLane* oppo = MSLane::dictionary(item.second);
    //         if (oppo == nullptr) {
    //             WRITE_ERRORF("Unknown neigh lane '%' for lane '%'", item.second, item.first->getID());
    //         } else {
    //             item.first->setOpposite(oppo);
    //         }
    //     }
    //     // consistency check
    //     std::set<const MSLane*> checked;
    //     for (auto item : myOppositeLanes) {
    //         if (item.first->getOpposite() != nullptr) {
    //             if (item.first->getOpposite()->getOpposite() != item.first) {
    //                 if (checked.count(item.first->getOpposite()) == 0) {
    //                     WRITE_WARNINGF(TL("Asymmetrical neigh lane '%' for lane '%'"), item.second, item.first->getID());
    //                     item.first->getOpposite()->setOpposite(item.first);
    //                 } else {
    //                     throw ProcessError(TLF("Mutually inconsistent neigh lane definitions for lanes '%', '%' and '%'",
    //                                            item.first->getID(), item.first->getOpposite()->getID(), Named::getIDSecure(item.first->getOpposite()->getOpposite())));
    //                 }
    //             }
    //             checked.insert(item.first);
    //             checked.insert(item.first->getOpposite());
    //         }
    //     }
    //     for (MSEdge* const edge : myEdges) {
    //         edge->closeBuilding();
    //     }
    //     for (MSEdge* const edge : myEdges) {
    //         edge->rebuildAllowedTargets(false);
    //     }
    //     // mark internal edges belonging to a roundabout (after all edges are build)
    //     if (MSGlobals::gUsingInternalLanes) {
    //         for (MSEdge* const edge : myEdges) {
    //             if (edge->isInternal()) {
    //                 if (edge->getNumSuccessors() != 1 || edge->getNumPredecessors() != 1) {
    //                     throw ProcessError(TLF("Internal edge '%' is not properly connected (probably a manually modified net.xml).", edge->getID()));
    //                 }
    //                 if (edge->getSuccessors()[0]->isRoundabout() || edge->getPredecessors()[0]->isRoundabout()) {
    //                     edge->markAsRoundabout();
    //                 }
    //             }
    //         }
    //     }
    //     if (!deprecatedVehicleClassesSeen.empty()) {
    //         WRITE_WARNINGF(TL("Deprecated vehicle classes '%' in input network."), toString(deprecatedVehicleClassesSeen));
    //         deprecatedVehicleClassesSeen.clear();
    //     }
    //     // check for bi-directional edges (this are edges in opposing direction and superposable/congruent shapes)
    //     if (myBidiEdges.size() > 0 || networkVersion > MMVersion(1, 0)) {
    //         for (auto& item : myBidiEdges) {
    //             item.first->checkAndRegisterBiDirEdge(item.second);
    //         }
    //         //WRITE_MESSAGEF(TL("Loaded % bidirectional edges"), toString(myBidiEdges.size()));
    //     } else {
    //         // legacy network
    //         for (MSEdge* e : myEdges) {
    //             e->checkAndRegisterBiDirEdge();
    //         }
    //     }
    //     // take into account bidi lanes when deciding on whether an edge allows changing
    //     for (MSEdge* const edge : myEdges) {
    //         edge->buildLaneChanger();
    //     }
    //     return new MSEdgeControl(myEdges);
    // }
    #[allow(unused_variables)]
    pub fn build(&mut self, networkVersion: &(i32, f64) /* MMVersion */) -> Ptr<MSEdgeControl> {
        if use_meso_sim() && !utils::options::options_cont::get_bool("meso-lane-queue") {
            MSEdge::set_meso_ignored_v_classes(parse_vehicle_classes_vec(&utils::options::options_cont::get_string_vector("meso-ignore-lanes-by-vclass")).unwrap_or_else(|e| std::panic::panic_any(e)));
        }
        // connecting opposite lanes must happen before MSEdge::closeBuilding
        for item in self.myOppositeLanes.iter() {
            let oppo = MSLane::dictionary_2(&item.1);
            if oppo.is_null() {
                write_errorf!("Unknown neigh lane '%' for lane '%'", item.1, item.0.get_id());
            } else {
                item.0.m().set_opposite(oppo);
            }
        }
        // consistency check
        let mut checked: BTreeSet<Ptr<MSLane>> = BTreeSet::new();
        for item in self.myOppositeLanes.iter() {
            if !item.0.get_opposite().is_null() {
                if item.0.get_opposite().get_opposite() != item.0 {
                    if !checked.contains(&item.0.get_opposite()) {
                        write_warningf!("Asymmetrical neigh lane '%' for lane '%'", item.1, item.0.get_id());
                        item.0.get_opposite().m().set_opposite(item.0);
                    } else {
                        std::panic::panic_any(ProcessError::new(format!(
                            "Mutually inconsistent neigh lane definitions for lanes '{}', '{}' and '{}'",
                            item.0.get_id(),
                            item.0.get_opposite().get_id(),
                            Named::get_id_secure(item.0.get_opposite().get_opposite().get(), "")
                        )));
                    }
                }
                checked.insert(item.0);
                checked.insert(item.0.get_opposite());
            }
        }
        for edge in self.myEdges.iter() {
            edge.m().close_building();
        }
        for edge in self.myEdges.iter() {
            edge.m().rebuild_allowed_targets(false);
        }
        // mark internal edges belonging to a roundabout (after all edges are build)
        if using_internal_lanes() {
            for edge in self.myEdges.iter() {
                if edge.is_internal() {
                    if edge.get_num_successors() != 1 || edge.get_num_predecessors() != 1 {
                        std::panic::panic_any(ProcessError::new(format!("Internal edge '{}' is not properly connected (probably a manually modified net.xml).", edge.get_id())));
                    }
                    if edge.get_successors(SVC_IGNORING)[0].is_roundabout() || edge.get_predecessors()[0].is_roundabout() {
                        edge.m().mark_as_roundabout();
                    }
                }
            }
        }
        // shortcut: C++ global deprecatedVehicleClassesSeen (utils/common/SUMOVehicleClass.cpp) is not ported, the deprecated-class warning is skipped until it is
        // check for bi-directional edges (this are edges in opposing direction and superposable/congruent shapes)
        if !self.myBidiEdges.is_empty() || *networkVersion > (1, 0.0) {
            for (edge, bidi) in self.myBidiEdges.iter() {
                edge.m().check_and_register_bi_dir_edge(bidi);
            }
        } else {
            // legacy network
            for e in self.myEdges.iter() {
                e.m().check_and_register_bi_dir_edge("");
            }
        }
        // take into account bidi lanes when deciding on whether an edge allows changing
        for edge in self.myEdges.iter() {
            edge.m().build_lane_changer();
        }
        Ptr::from_mut(Box::leak(Box::new(MSEdgeControl::new(&self.myEdges))))
    }

    /// C++: `MSEdge * NLEdgeControlBuilder::buildEdge(const std::string &, const SumoXMLEdgeFunc, const std::string &, const std::string &, const std::string &, const int, const double)` [virtual]
    /// Builds an edge instance (MSEdge in this case)
    // MSEdge*
    // NLEdgeControlBuilder::buildEdge(const std::string& id, const SumoXMLEdgeFunc function,
    //                                 const std::string& streetName, const std::string& edgeType, const std::string& routingType,
    //                                 const int priority, const double distance) {
    //     return new MSEdge(id, myCurrentNumericalEdgeID++, function, streetName, edgeType, routingType, priority, distance);
    // }
    #[allow(unused_variables)]
    pub fn build_edge(&mut self, id: &str, function: SumoXMLEdgeFunc, streetName: &str, edgeType: &str, routingType: &str, priority: i32, distance: f64) -> Ptr<MSEdge> {
        let numericalID = self.myCurrentNumericalEdgeID;
        self.myCurrentNumericalEdgeID += 1;
        Ptr::from_mut(Box::leak(Box::new(MSEdge::new(id, numericalID, function, streetName, edgeType, routingType, priority, distance))))
    }

    /// C++: `void NLEdgeControlBuilder::addCrossingEdges(const std::vector<std::string> &)` [virtual]
    /// add the crossingEdges in a crossing edge if present
    // void NLEdgeControlBuilder::addCrossingEdges(const std::vector<std::string>& crossingEdges) {
    //     myActiveEdge->setCrossingEdges(crossingEdges);
    // }
    #[allow(unused_variables)]
    pub fn add_crossing_edges(&mut self, arg0: &Vec<String>) {
        self.myActiveEdge.m().set_crossing_edges(arg0);
    }

    /// C++: `SumoXMLEdgeFunc NLEdgeControlBuilder::getCurrentEdgeFunction() const` [const]
    // SumoXMLEdgeFunc
    // NLEdgeControlBuilder::getCurrentEdgeFunction() const {
    //     if (myActiveEdge == nullptr) {
    //         return SumoXMLEdgeFunc::UNKNOWN;
    //     } else {
    //         return myActiveEdge->getFunction();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_current_edge_function(&self) -> SumoXMLEdgeFunc {
        if self.myActiveEdge.is_null() {
            SumoXMLEdgeFunc::UNKNOWN
        } else {
            self.myActiveEdge.get_function()
        }
    }

    /// C++: `void NLEdgeControlBuilder::updateCurrentLaneStopOffset(const StopOffset &)` [protected]
    /// set the stopOffset for the last added lane.
    // void
    // NLEdgeControlBuilder::updateCurrentLaneStopOffset(const StopOffset& stopOffset) {
    //     if (myLaneStorage->size() == 0) {
    //         throw ProcessError("myLaneStorage cannot be empty");
    //     }
    //     if (stopOffset.isDefined()) {
    //         if (myLaneStorage->back()->getLaneStopOffsets().isDefined()) {
    //             WRITE_WARNING("Duplicate stopOffset definition for lane " + toString(myLaneStorage->back()->getIndex()) +
    //                           " on edge " + myActiveEdge->getID() + "!")
    //         } else {
    //             myLaneStorage->back()->setLaneStopOffset(stopOffset);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn update_current_lane_stop_offset(&mut self, stopOffset: &StopOffset) {
        if self.myLaneStorage.m().len() == 0 {
            std::panic::panic_any(ProcessError::new("myLaneStorage cannot be empty"));
        }
        if stopOffset.is_defined() {
            let back = *self.myLaneStorage.m().last().unwrap();
            if back.get_lane_stop_offsets().is_defined() {
                utils::common::msg_handler::write_warning(&format!("Duplicate stopOffset definition for lane {} on edge {}!", back.get_index(), self.myActiveEdge.get_id()));
            } else {
                back.m().set_lane_stop_offset(stopOffset);
            }
        }
    }

    /// C++: `void NLEdgeControlBuilder::setDefaultStopOffset(const StopOffset &)` [protected]
    /// set the stopOffset for the last added lane.
    // void
    // NLEdgeControlBuilder::setDefaultStopOffset(const StopOffset& stopOffsets) {
    //     if (myCurrentDefaultStopOffset.isDefined()) {
    //         WRITE_WARNING("Duplicate stopOffset definition for edge " + myActiveEdge->getID() + ". Ignoring duplicate specification.")
    //     } else {
    //         myCurrentDefaultStopOffset = stopOffsets;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_default_stop_offset(&mut self, stopOffset: &StopOffset) {
        if self.myCurrentDefaultStopOffset.is_defined() {
            utils::common::msg_handler::write_warning(&format!("Duplicate stopOffset definition for edge {}. Ignoring duplicate specification.", self.myActiveEdge.get_id()));
        } else {
            self.myCurrentDefaultStopOffset = *stopOffset;
        }
    }

    /// C++: `void NLEdgeControlBuilder::applyDefaultStopOffsetsToLanes()` [protected]
    // void
    // NLEdgeControlBuilder::applyDefaultStopOffsetsToLanes() {
    //     if (myActiveEdge == nullptr) {
    //         throw ProcessError("myActiveEdge cannot be nullptr");
    //     }
    //     if (myCurrentDefaultStopOffset.isDefined()) {
    //         for (const auto& l : *myLaneStorage) {
    //             if (!l->getLaneStopOffsets().isDefined()) {
    //                 l->setLaneStopOffset(myCurrentDefaultStopOffset);
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn apply_default_stop_offsets_to_lanes(&mut self) {
        if self.myActiveEdge.is_null() {
            std::panic::panic_any(ProcessError::new("myActiveEdge cannot be nullptr"));
        }
        if self.myCurrentDefaultStopOffset.is_defined() {
            for l in self.myLaneStorage.m().iter() {
                if !l.get_lane_stop_offsets().is_defined() {
                    l.m().set_lane_stop_offset(&self.myCurrentDefaultStopOffset);
                }
            }
        }
    }

    /// C++: `void NLEdgeControlBuilder::NLEdgeControlBuilder(const NLEdgeControlBuilder &)` [private]
    /// invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(s: &NLEdgeControlBuilder) -> Self {
        panic!("NLEdgeControlBuilder copy constructor is private and undefined in C++, it must not be used")
    }

    /// C++: `NLEdgeControlBuilder & NLEdgeControlBuilder::operator=(const NLEdgeControlBuilder &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &NLEdgeControlBuilder) -> &mut NLEdgeControlBuilder {
        panic!("NLEdgeControlBuilder assignment is private and undefined in C++, it must not be used")
    }

}

