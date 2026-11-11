//! Port of microsim/actions/Command_SaveTLSProgram.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// Writes the switch times of a tls into a file when the tls switches
/// C++ class `Command_SaveTLSProgram` (microsim/actions/Command_SaveTLSProgram.h)
pub struct Command_SaveTLSProgram {
    /// C++ base class `Command`
    pub base: Unmapped /* Command */,
    /// The device to write to
    pub myOutputDevice: Ptr<OutputDevice>,
    /// The traffic light logic to use
    pub myLogics: Ptr<MSTLLogicControl_TLSLogicVariants>,
    /// Storage for prior state
    pub myPreviousStates: Vec<MSPhaseDefinition>,
    /// Storage for prior sub-id
    pub myPreviousProgramID: String,
    /// Storage for the tls ID (needed in destructor)
    pub myTLSID: String,
}

impl std::ops::Deref for Command_SaveTLSProgram {
    type Target = Unmapped /* Command */;
    fn deref(&self) -> &Unmapped /* Command */ {
        &self.base
    }
}
impl std::ops::DerefMut for Command_SaveTLSProgram {
    fn deref_mut(&mut self) -> &mut Unmapped /* Command */ {
        &mut self.base
    }
}

impl Command_SaveTLSProgram {
    /// C++: `void Command_SaveTLSProgram::Command_SaveTLSProgram(const MSTLLogicControl::TLSLogicVariants &, OutputDevice &)`
    /// Constructor
    // Command_SaveTLSProgram::Command_SaveTLSProgram(const MSTLLogicControl::TLSLogicVariants& logics, OutputDevice& od):
    //     myOutputDevice(od),
    //     myLogics(logics) {
    //     MSNet::getInstance()->getEndOfTimestepEvents()->addEvent(this);
    //     myOutputDevice.writeXMLHeader("additional", "additional_file.xsd");
    // }
    #[allow(unused_variables)]
    pub fn new(logics: &MSTLLogicControl_TLSLogicVariants, od: &mut OutputDevice) -> Self {
        todo!("port Command_SaveTLSProgram::Command_SaveTLSProgram")
    }

    /// C++: `void Command_SaveTLSProgram::~Command_SaveTLSProgram() noexcept` [virtual]
    /// Destructor
    // Command_SaveTLSProgram::~Command_SaveTLSProgram() {
    //     writeCurrent();
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port Command_SaveTLSProgram::~Command_SaveTLSProgram")
    }

    /// C++: `SUMOTime Command_SaveTLSProgram::execute(SUMOTime)` [virtual]
    /// Writes the state of the tls if a change occurred
    // SUMOTime
    // Command_SaveTLSProgram::execute(SUMOTime /*currentTime*/) {
    //     const std::string& state = myLogics.getActive()->getCurrentPhaseDef().getState();
    //     const std::string& name = myLogics.getActive()->getCurrentPhaseDef().getName();
    //     if (myLogics.getActive()->getProgramID() != myPreviousProgramID) {
    //         writeCurrent();
    //         myPreviousProgramID = myLogics.getActive()->getProgramID();
    //         myTLSID = myLogics.getActive()->getID();
    //     }
    //     if (myPreviousStates.size() == 0 || myPreviousStates.back().getState() != state) {
    //         myPreviousStates.push_back(MSPhaseDefinition(0, state, name));
    //     }
    //     myPreviousStates.back().duration += DELTA_T;
    //     return DELTA_T;
    // }
    #[allow(unused_variables)]
    pub fn execute(&mut self, currentTime: SUMOTime) -> SUMOTime {
        todo!("port Command_SaveTLSProgram::execute")
    }

    /// C++: `void Command_SaveTLSProgram::writeCurrent()` [private]
    /// Write the current program
    // void
    // Command_SaveTLSProgram::writeCurrent() {
    //     if (myPreviousStates.size() > 0) {
    //         myOutputDevice.openTag(SUMO_TAG_TLLOGIC);
    //         myOutputDevice.writeAttr(SUMO_ATTR_ID, myTLSID);
    //         myOutputDevice.writeAttr(SUMO_ATTR_TYPE, "static");
    //         myOutputDevice.writeAttr(SUMO_ATTR_PROGRAMID, myPreviousProgramID);
    //         // write the phases
    //         for (const MSPhaseDefinition& p : myPreviousStates) {
    //             myOutputDevice.openTag(SUMO_TAG_PHASE);
    //             myOutputDevice.writeAttr(SUMO_ATTR_DURATION, STEPS2TIME(p.duration));
    //             if (p.duration < TIME2STEPS(10)) {
    //                 myOutputDevice.writePadding(" ");
    //             }
    //             myOutputDevice.writeAttr(SUMO_ATTR_STATE, p.getState());
    //             if (p.getName() != "") {
    //                 myOutputDevice.writeAttr(SUMO_ATTR_NAME, p.getName());
    //             }
    //             myOutputDevice.closeTag();
    //         }
    //         // write params
    //         myOutputDevice.closeTag();
    //         myPreviousStates.clear();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn write_current(&mut self) {
        todo!("port Command_SaveTLSProgram::writeCurrent")
    }

    /// C++: `void Command_SaveTLSProgram::Command_SaveTLSProgram(const Command_SaveTLSProgram &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &Command_SaveTLSProgram) -> Self {
        todo!("port Command_SaveTLSProgram::Command_SaveTLSProgram")
    }

    /// C++: `Command_SaveTLSProgram & Command_SaveTLSProgram::operator=(const Command_SaveTLSProgram &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &Command_SaveTLSProgram) -> &mut Command_SaveTLSProgram {
        todo!("port Command_SaveTLSProgram::operator=")
    }

}

