//! Port of microsim/actions/Command_SaveTLSState.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// Writes the state of the tls to a file (in each second)
/// C++ class `Command_SaveTLSState` (microsim/actions/Command_SaveTLSState.h)
pub struct Command_SaveTLSState {
    /// C++ base class `Command`
    pub base: Unmapped /* Command */,
    /// The device to write to
    pub myOutputDevice: Ptr<OutputDevice>,
    /// The traffic light logic to use
    pub myLogics: Ptr<MSTLLogicControl_TLSLogicVariants>,
    /// additiona logging
    pub mySaveDetectors: bool,
    pub mySaveConditions: bool,
}

impl std::ops::Deref for Command_SaveTLSState {
    type Target = Unmapped /* Command */;
    fn deref(&self) -> &Unmapped /* Command */ {
        &self.base
    }
}
impl std::ops::DerefMut for Command_SaveTLSState {
    fn deref_mut(&mut self) -> &mut Unmapped /* Command */ {
        &mut self.base
    }
}

impl Command_SaveTLSState {
    /// C++: `void Command_SaveTLSState::Command_SaveTLSState(const MSTLLogicControl::TLSLogicVariants &, OutputDevice &, bool, bool)`
    /// Constructor
    // Command_SaveTLSState::Command_SaveTLSState(const MSTLLogicControl::TLSLogicVariants& logics,
    //         OutputDevice& od, bool saveDetectors, bool saveConditions) :
    //     myOutputDevice(od),
    //     myLogics(logics),
    //     mySaveDetectors(saveDetectors),
    //     mySaveConditions(saveConditions) {
    //     MSNet::getInstance()->getEndOfTimestepEvents()->addEvent(this);
    // }
    #[allow(unused_variables)]
    pub fn new(logics: &MSTLLogicControl_TLSLogicVariants, od: &mut OutputDevice, saveDetectors: bool, saveConditions: bool) -> Self {
        todo!("port Command_SaveTLSState::Command_SaveTLSState")
    }

    /// C++: `void Command_SaveTLSState::~Command_SaveTLSState() noexcept` [virtual]
    /// Destructor
    // Command_SaveTLSState::~Command_SaveTLSState() {
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port Command_SaveTLSState::~Command_SaveTLSState")
    }

    /// C++: `SUMOTime Command_SaveTLSState::execute(SUMOTime)` [virtual]
    /// Writes the current state of the tls
    // SUMOTime
    // Command_SaveTLSState::execute(SUMOTime currentTime) {
    //     if (!myOutputDevice.wroteHeader()) {
    //         // delay writing header to ensure tls is initialized
    //         std::map<SumoXMLAttr, std::string> attrs;
    //         if (mySaveDetectors) {
    //             std::vector<std::string> IDs;
    //             for (auto item : myLogics.getActive()->getDetectorStates()) {
    //                 IDs.push_back(item.first);
    //             }
    //             attrs[SUMO_ATTR_DETECTORS] = joinToString(IDs, " ");
    //
    //         }
    //         if (mySaveConditions) {
    //             std::vector<std::string> IDs;
    //             for (auto item : myLogics.getActive()->getConditions()) {
    //                 IDs.push_back(item.first);
    //             }
    //             attrs[SUMO_ATTR_CONDITIONS] = joinToString(IDs, " ");
    //         }
    //         myOutputDevice.writeXMLHeader("tlsStates", "tlsstates_file.xsd", attrs);
    //     }
    //     myOutputDevice.openTag("tlsState");
    //     myOutputDevice.writeAttr(SUMO_ATTR_TIME, time2string(currentTime));
    //     myOutputDevice.writeAttr(SUMO_ATTR_ID, myLogics.getActive()->getID());
    //     myOutputDevice.writeAttr(SUMO_ATTR_PROGRAMID, myLogics.getActive()->getProgramID());
    //     myOutputDevice.writeAttr(SUMO_ATTR_PHASE, myLogics.getActive()->getCurrentPhaseIndex());
    //     myOutputDevice.writeAttr(SUMO_ATTR_STATE, myLogics.getActive()->getCurrentPhaseDef().getState());
    //     if (!myLogics.getActive()->getCurrentPhaseDef().getName().empty()) {
    //         myOutputDevice.writeAttr(SUMO_ATTR_NAME, myLogics.getActive()->getCurrentPhaseDef().getName());
    //     }
    //     if (mySaveDetectors) {
    //         std::vector<int> states;
    //         for (auto item : myLogics.getActive()->getDetectorStates()) {
    //             states.push_back((int)item.second);
    //         }
    //         myOutputDevice.writeAttr(SUMO_ATTR_DETECTORS, joinToString(states, " "));
    //
    //     }
    //     if (mySaveConditions) {
    //         std::vector<double> states;
    //         for (auto item : myLogics.getActive()->getConditions()) {
    //             states.push_back(item.second);
    //         }
    //         myOutputDevice.writeAttr(SUMO_ATTR_CONDITIONS, joinToString(states, " "));
    //     }
    //     myOutputDevice.closeTag();
    //     return DELTA_T;
    // }
    #[allow(unused_variables)]
    pub fn execute(&mut self, currentTime: SUMOTime) -> SUMOTime {
        todo!("port Command_SaveTLSState::execute")
    }

    /// C++: `void Command_SaveTLSState::Command_SaveTLSState(const Command_SaveTLSState &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &Command_SaveTLSState) -> Self {
        todo!("port Command_SaveTLSState::Command_SaveTLSState")
    }

    /// C++: `Command_SaveTLSState & Command_SaveTLSState::operator=(const Command_SaveTLSState &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &Command_SaveTLSState) -> &mut Command_SaveTLSState {
        todo!("port Command_SaveTLSState::operator=")
    }

}

