//! Port of microsim/actions/Command_SaveTLSSwitchStates.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// Writes the switch times of a tls into a file when the tls switches
/// C++ class `Command_SaveTLSSwitchStates` (microsim/actions/Command_SaveTLSSwitchStates.h)
pub struct Command_SaveTLSSwitchStates {
    /// C++ base class `Command`
    pub base: Unmapped /* Command */,
    /// The device to write to
    pub myOutputDevice: Ptr<OutputDevice>,
    /// The traffic light logic to use
    pub myLogics: Ptr<MSTLLogicControl_TLSLogicVariants>,
    /// Storage for prior state
    pub myPreviousState: String,
    /// Storage for prior sub-id
    pub myPreviousProgramID: String,
}

impl std::ops::Deref for Command_SaveTLSSwitchStates {
    type Target = Unmapped /* Command */;
    fn deref(&self) -> &Unmapped /* Command */ {
        &self.base
    }
}
impl std::ops::DerefMut for Command_SaveTLSSwitchStates {
    fn deref_mut(&mut self) -> &mut Unmapped /* Command */ {
        &mut self.base
    }
}

impl Command_SaveTLSSwitchStates {
    /// C++: `void Command_SaveTLSSwitchStates::Command_SaveTLSSwitchStates(const MSTLLogicControl::TLSLogicVariants &, OutputDevice &)`
    /// Constructor
    // Command_SaveTLSSwitchStates::Command_SaveTLSSwitchStates(const MSTLLogicControl::TLSLogicVariants& logics,
    //         OutputDevice& od)
    //     : myOutputDevice(od), myLogics(logics) {
    //     MSNet::getInstance()->getEndOfTimestepEvents()->addEvent(this);
    //     myOutputDevice.writeXMLHeader("tlsStates", "tlsstates_file.xsd");
    // }
    #[allow(unused_variables)]
    pub fn new(logics: &MSTLLogicControl_TLSLogicVariants, od: &mut OutputDevice) -> Self {
        todo!("port Command_SaveTLSSwitchStates::Command_SaveTLSSwitchStates")
    }

    /// C++: `void Command_SaveTLSSwitchStates::~Command_SaveTLSSwitchStates() noexcept` [virtual]
    /// Destructor
    // Command_SaveTLSSwitchStates::~Command_SaveTLSSwitchStates() {
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port Command_SaveTLSSwitchStates::~Command_SaveTLSSwitchStates")
    }

    /// C++: `SUMOTime Command_SaveTLSSwitchStates::execute(SUMOTime)` [virtual]
    /// Writes the state of the tls if a change occurred
    // SUMOTime
    // Command_SaveTLSSwitchStates::execute(SUMOTime currentTime) {
    //     const std::string& state = myLogics.getActive()->getCurrentPhaseDef().getState();
    //     if (state != myPreviousState || myLogics.getActive()->getProgramID() != myPreviousProgramID) {
    //         myOutputDevice.openTag("tlsState");
    //         myOutputDevice.writeAttr(SUMO_ATTR_TIME, time2string(currentTime));
    //         myOutputDevice.writeAttr(SUMO_ATTR_ID, myLogics.getActive()->getID());
    //         myOutputDevice.writeAttr(SUMO_ATTR_PROGRAMID, myLogics.getActive()->getProgramID());
    //         myOutputDevice.writeAttr(SUMO_ATTR_PHASE, myLogics.getActive()->getCurrentPhaseIndex());
    //         myOutputDevice.writeAttr(SUMO_ATTR_STATE, myLogics.getActive()->getCurrentPhaseDef().getState());
    //         if (!myLogics.getActive()->getCurrentPhaseDef().getName().empty()) {
    //             myOutputDevice.writeAttr(SUMO_ATTR_NAME, myLogics.getActive()->getCurrentPhaseDef().getName());
    //         }
    //         myOutputDevice.closeTag();
    //         myPreviousState = state;
    //         myPreviousProgramID = myLogics.getActive()->getProgramID();
    //     }
    //     return DELTA_T;
    // }
    #[allow(unused_variables)]
    pub fn execute(&mut self, currentTime: SUMOTime) -> SUMOTime {
        todo!("port Command_SaveTLSSwitchStates::execute")
    }

    /// C++: `void Command_SaveTLSSwitchStates::Command_SaveTLSSwitchStates(const Command_SaveTLSSwitchStates &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &Command_SaveTLSSwitchStates) -> Self {
        todo!("port Command_SaveTLSSwitchStates::Command_SaveTLSSwitchStates")
    }

    /// C++: `Command_SaveTLSSwitchStates & Command_SaveTLSSwitchStates::operator=(const Command_SaveTLSSwitchStates &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &Command_SaveTLSSwitchStates) -> &mut Command_SaveTLSSwitchStates {
        todo!("port Command_SaveTLSSwitchStates::operator=")
    }

}

