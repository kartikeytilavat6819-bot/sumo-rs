//! Port of microsim/actions/Command_SaveTLSSwitchStates.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// Writes the switch times of a tls into a file when the tls switches
/// C++ class `Command_SaveTLSSwitchStates` (microsim/actions/Command_SaveTLSSwitchStates.h)
#[repr(C)]
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
        // shortcut: C++ registers this with MSNet end-of-timestep events (addEvent(this)); Command is not ported yet and a by-value constructor has no stable address, so registration is skipped; upgrade when Command is ported and constructors return Box<Self>
        od.write_xml_header("tlsStates", "tlsstates_file.xsd", BTreeMap::new(), true);
        Self {
            base: (),
            myOutputDevice: Ptr::from_mut(od),
            myLogics: Ptr::new(logics),
            myPreviousState: String::new(),
            myPreviousProgramID: String::new(),
        }
    }

    /// C++: `void Command_SaveTLSSwitchStates::~Command_SaveTLSSwitchStates() noexcept` [virtual]
    /// Destructor
    // Command_SaveTLSSwitchStates::~Command_SaveTLSSwitchStates() {
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
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
        let logic = self.myLogics.get_active();
        let state = logic.get_current_phase_def().get_state().to_string();
        if state != self.myPreviousState || logic.get_program_id() != self.myPreviousProgramID {
            let dev = self.myOutputDevice.m();
            dev.open_tag("tlsState");
            dev.write_attr(SUMO_ATTR_TIME, &time2string(currentTime));
            dev.write_attr(SUMO_ATTR_ID, logic.get_id());
            dev.write_attr(SUMO_ATTR_PROGRAMID, logic.get_program_id());
            dev.write_attr(SUMO_ATTR_PHASE, &logic.get_current_phase_index());
            dev.write_attr(SUMO_ATTR_STATE, logic.get_current_phase_def().get_state());
            if !logic.get_current_phase_def().get_name().is_empty() {
                dev.write_attr(SUMO_ATTR_NAME, logic.get_current_phase_def().get_name());
            }
            dev.close_tag("");
            self.myPreviousState = state;
            self.myPreviousProgramID = logic.get_program_id().to_string();
        }
        delta_t()
    }

    /// C++: `void Command_SaveTLSSwitchStates::Command_SaveTLSSwitchStates(const Command_SaveTLSSwitchStates &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &Command_SaveTLSSwitchStates) -> Self {
        panic!("Command_SaveTLSSwitchStates copy constructor is private and invalidated in C++")
    }

    /// C++: `Command_SaveTLSSwitchStates & Command_SaveTLSSwitchStates::operator=(const Command_SaveTLSSwitchStates &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &Command_SaveTLSSwitchStates) -> &mut Command_SaveTLSSwitchStates {
        panic!("Command_SaveTLSSwitchStates assignment operator is private and invalidated in C++")
    }

}

