//! Port of microsim/output/MSDetectorFileOutput.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ enum `DetectorUsage`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum DetectorUsage {
    DU_USER_DEFINED = 0,
    DU_SUMO_INTERNAL = 1,
    DU_TL_CONTROL = 2,
}

/// Base of value-generating classes (detectors)
/// C++ class `MSDetectorFileOutput` (microsim/output/MSDetectorFileOutput.h)
pub struct MSDetectorFileOutput {
    /// C++ base class `Named`
    pub base: Named,
    /// C++ base class `Parameterised`
    pub base2: Parameterised,
    /// The vehicle types to look for (empty means all)
    pub myVehicleTypes: BTreeSet<String>,
    /// The upcoming edges to filter by (empty means no filtering)
    pub myNextEdges: Vec<Ptr<MSEdge>>,
    /// Whether pedestrians shall be detected instead of vehicles
    pub myDetectPersons: i32,
}

impl std::ops::Deref for MSDetectorFileOutput {
    type Target = Named;
    fn deref(&self) -> &Named {
        &self.base
    }
}
impl std::ops::DerefMut for MSDetectorFileOutput {
    fn deref_mut(&mut self) -> &mut Named {
        &mut self.base
    }
}

impl MSDetectorFileOutput {
    /// C++: `void MSDetectorFileOutput::MSDetectorFileOutput(const std::string &, const std::string &, const std::string &, const int)`
    /// Constructor
    // MSDetectorFileOutput::MSDetectorFileOutput(const std::string& id,
    //         const std::string& vTypes,
    //         const std::string& nextEdges,
    //         const int detectPersons) :
    //     Named(id),
    //     myDetectPersons(detectPersons) {
    //     const std::vector<std::string> vt = StringTokenizer(vTypes).getVector();
    //     myVehicleTypes.insert(vt.begin(), vt.end());
    //     for (const std::string& edgeID : StringTokenizer(nextEdges).getVector()) {
    //         const MSEdge* e = MSEdge::dictionary(edgeID);
    //         if (e) {
    //             myNextEdges.push_back(e);
    //         } else {
    //             throw ProcessError("Unknown edge '" + edgeID + "' given as nextEdges in detector '" + id + "'");
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn new(id: &str, vTypes: &str, nextEdges: &str, detectPersons: i32) -> Self {
        todo!("port MSDetectorFileOutput::MSDetectorFileOutput")
    }

    /// C++: `void MSDetectorFileOutput::~MSDetectorFileOutput() noexcept` [virtual]
    /// (virtual) destructor
    // virtual ~MSDetectorFileOutput() { }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MSDetectorFileOutput::~MSDetectorFileOutput")
    }

    /// C++: `void MSDetectorFileOutput::writeXMLOutput(OutputDevice &, SUMOTime, SUMOTime)` [virtual, pure virtual]
    /// Write the generated output to the given device
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn write_xml_output(&mut self, dev: &mut OutputDevice, startTime: SUMOTime, stopTime: SUMOTime) {
        todo!("port MSDetectorFileOutput::writeXMLOutput")
    }

    /// C++: `void MSDetectorFileOutput::writeXMLDetectorProlog(OutputDevice &) const` [const, virtual, pure virtual]
    /// Open the XML-output
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn write_xml_detector_prolog(&self, dev: &mut OutputDevice) {
        todo!("port MSDetectorFileOutput::writeXMLDetectorProlog")
    }

    /// C++: `void MSDetectorFileOutput::reset()` [virtual]
    /// Resets collected values
    // virtual void reset() { }
    #[allow(unused_variables)]
    pub fn reset(&mut self) {
        todo!("port MSDetectorFileOutput::reset")
    }

    /// C++: `void MSDetectorFileOutput::detectorUpdate(const SUMOTime)` [virtual]
    /// Updates the detector (computes values)
    // virtual void detectorUpdate(const SUMOTime step) {
    //         UNUSED_PARAMETER(step);
    //     }
    #[allow(unused_variables)]
    pub fn detector_update(&mut self, step: SUMOTime) {
        todo!("port MSDetectorFileOutput::detectorUpdate")
    }

    /// C++: `GUIDetectorWrapper * MSDetectorFileOutput::buildDetectorGUIRepresentation()` [virtual]
    /// Builds the graphical representation
    // virtual GUIDetectorWrapper* buildDetectorGUIRepresentation() {
    //         return 0;
    //     }
    #[allow(unused_variables)]
    pub fn build_detector_gui_representation(&mut self) -> Ptr<Unmapped /* GUIDetectorWrapper */> {
        todo!("port MSDetectorFileOutput::buildDetectorGUIRepresentation")
    }

    /// C++: `bool MSDetectorFileOutput::vehicleApplies(const SUMOTrafficObject &) const` [const]
    /// Checks whether the detector measures vehicles of the given type.
    // bool
    // MSDetectorFileOutput::vehicleApplies(const SUMOTrafficObject& veh) const {
    //     if (veh.isVehicle() == detectsPersons()) {
    //         return false;
    //     }
    //     if (!myVehicleTypes.empty() && myVehicleTypes.count(veh.getVehicleType().getOriginalID()) == 0) {
    //         std::set<std::string> vTypeDists = MSNet::getInstance()->getVehicleControl().getVTypeDistributionMembership(veh.getVehicleType().getOriginalID());
    //         bool typeMatches = false;
    //         for (auto vTypeDist : vTypeDists) {
    //             if (myVehicleTypes.count(vTypeDist) > 0) {
    //                 typeMatches = true;
    //                 break;
    //             }
    //         }
    //         if (!typeMatches) {
    //             return false;
    //         }
    //     }
    //     if (!myNextEdges.empty()) {
    //         MSRouteIterator it;
    //         MSRouteIterator end;
    //         ConstMSEdgeVector route;
    //         if (veh.isVehicle()) {
    //             const SUMOVehicle& v = dynamic_cast<const SUMOVehicle&>(veh);
    //             it = v.getCurrentRouteEdge();
    //             end = v.getRoute().end();
    //         } else if (veh.isPerson()) {
    //             const MSTransportable& p = dynamic_cast<const MSTransportable&>(veh);
    //             route = p.getCurrentStage()->getEdges();
    //             it = route.begin() + p.getRoutePosition();
    //             end = route.end();
    //         }
    //         for (const MSEdge* e : myNextEdges) {
    //             it = std::find(it, end, e);
    //             if (it == end) {
    //                 if (e != veh.getNextEdgePtr()) {
    //                     //std::cout << SIMTIME << " det=" << getID() << " veh=" << veh.getID() << " e=" << e->getID() << " vehNext=" << Named::getIDSecure(veh.getNextEdgePtr()) << "\n";
    //                     return false;
    //                 }
    //             }
    //         }
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn vehicle_applies(&self, veh: &SUMOTrafficObject) -> bool {
        todo!("port MSDetectorFileOutput::vehicleApplies")
    }

    /// C++: `bool MSDetectorFileOutput::personApplies(const MSTransportable &, int) const` [const]
    // bool
    // MSDetectorFileOutput::personApplies(const MSTransportable& p, int dir) const {
    //     //std::cout << getID() << " p=" << p.getID() << " veh=" << Named::getIDSecure(p.getVehicle()) << "\n";
    //     if (p.getVehicle() == nullptr) {
    //         const int dirCode = dir < 0 ? 2 : dir;
    //         //std::cout << "   dir=" << dir << " dirCode=" << dirCode << " myDetectPersons=" << myDetectPersons << "\n";
    //         if ((dirCode & myDetectPersons) == 0) {
    //             // person walking in wrong direction or not walking at all
    //             return false;
    //         }
    //     } else {
    //         const SUMOVehicleClass svc = p.getVehicle()->getVClass();
    //         int vClassCode;
    //         if ((svc & SVC_PUBLIC_CLASSES) != 0) {
    //             vClassCode = (int)PersonMode::PUBLIC;
    //         } else if ((svc & SVC_BICYCLE) != 0) {
    //             vClassCode = (int)PersonMode::BICYCLE;
    //         } else if ((svc & SVC_TAXI) != 0) {
    //             vClassCode = (int)PersonMode::TAXI;
    //         } else {
    //             vClassCode = (int)PersonMode::CAR;
    //         }
    //         if ((vClassCode & myDetectPersons) == 0) {
    //             // person riding in the wrong vehicle
    //             return false;
    //         }
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn person_applies(&self, p: &MSTransportable, dir: i32) -> bool {
        todo!("port MSDetectorFileOutput::personApplies")
    }

    /// C++: `bool MSDetectorFileOutput::isTyped() const` [const]
    /// Checks whether the detector is type specific.
    // bool isTyped() const {
    //         return !myVehicleTypes.empty();
    //     }
    #[allow(unused_variables)]
    pub fn is_typed(&self) -> bool {
        todo!("port MSDetectorFileOutput::isTyped")
    }

    /// C++: `const std::set<std::string> & MSDetectorFileOutput::getVehicleTypes() const` [const]
    // const std::set<std::string>& getVehicleTypes() const {
    //         return myVehicleTypes;
    //     }
    #[allow(unused_variables)]
    pub fn get_vehicle_types(&self) -> &BTreeSet<String> {
        todo!("port MSDetectorFileOutput::getVehicleTypes")
    }

    /// C++: `bool MSDetectorFileOutput::detectsPersons() const` [const]
    // inline bool detectsPersons() const {
    //         return myDetectPersons != 0;
    //     }
    #[allow(unused_variables)]
    pub fn detects_persons(&self) -> bool {
        todo!("port MSDetectorFileOutput::detectsPersons")
    }

    /// C++: `void MSDetectorFileOutput::clearState(SUMOTime)` [virtual]
    /// Remove all vehicles before quick-loading state
    // virtual void clearState(SUMOTime /*step*/) {}
    #[allow(unused_variables)]
    pub fn clear_state(&mut self, arg0: SUMOTime) {
        todo!("port MSDetectorFileOutput::clearState")
    }

    /// C++: `void MSDetectorFileOutput::saveState(OutputDevice &) const` [const, virtual]
    /// Saves the state which is needed to continue after loading state
    // virtual void saveState(OutputDevice& /*out*/) const {}
    #[allow(unused_variables)]
    pub fn save_state(&self, arg0: &mut OutputDevice) {
        todo!("port MSDetectorFileOutput::saveState")
    }

    /// C++: `void MSDetectorFileOutput::loadState(const SUMOSAXAttributes &, const SUMOTime)` [virtual]
    /// Loads the state, shifting all times by the given offset
    // virtual void loadState(const SUMOSAXAttributes& /*attrs*/, const SUMOTime /*offset*/) {}
    #[allow(unused_variables)]
    pub fn load_state(&mut self, arg0: &SUMOSAXAttributes, arg1: SUMOTime) {
        todo!("port MSDetectorFileOutput::loadState")
    }

    /// C++: `void MSDetectorFileOutput::MSDetectorFileOutput(const MSDetectorFileOutput &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSDetectorFileOutput) -> Self {
        todo!("port MSDetectorFileOutput::MSDetectorFileOutput")
    }

    /// C++: `MSDetectorFileOutput & MSDetectorFileOutput::operator=(const MSDetectorFileOutput &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSDetectorFileOutput) -> &mut MSDetectorFileOutput {
        todo!("port MSDetectorFileOutput::operator=")
    }

}

