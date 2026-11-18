//! Port of microsim/actions/Command_SaveTLSProgram.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// Writes the switch times of a tls into a file when the tls switches
/// C++ class `Command_SaveTLSProgram` (microsim/actions/Command_SaveTLSProgram.h)
#[repr(C)]
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
    pub fn new(logics: &MSTLLogicControl_TLSLogicVariants, od: &mut OutputDevice) -> Box<Self> {
        // shortcut: C++ registers this with MSNet end-of-timestep events (addEvent(this)); Command is not ported yet and a by-value constructor has no stable address, so registration is skipped; upgrade when Command is ported and constructors return Box<Self>
        od.write_xml_header("additional", "additional_file.xsd", BTreeMap::new(), true);
        let mut b = Box::new(Self {
            base: (),
            myOutputDevice: Ptr::from_mut(od),
            myLogics: Ptr::new(logics),
            myPreviousStates: Vec::new(),
            myPreviousProgramID: String::new(),
            myTLSID: String::new(),
        });
        // C++ constructor: MSNet::getInstance()->getEndOfTimestepEvents()->addEvent(this)
        MSNet::get_instance().m().get_end_of_timestep_events().m().add_event(Ptr::from_mut(&mut *b as &mut dyn Command), -1);
        b
    }

    /// C++: `void Command_SaveTLSProgram::~Command_SaveTLSProgram() noexcept` [virtual]
    /// Destructor
    // Command_SaveTLSProgram::~Command_SaveTLSProgram() {
    //     writeCurrent();
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        self.write_current();
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
        let logic = self.myLogics.get_active();
        let state = logic.get_current_phase_def().get_state().to_string();
        let name = logic.get_current_phase_def().get_name().to_string();
        if logic.get_program_id() != self.myPreviousProgramID {
            self.write_current();
            self.myPreviousProgramID = logic.get_program_id().to_string();
            self.myTLSID = logic.get_id().to_string();
        }
        if self.myPreviousStates.is_empty() || self.myPreviousStates.last().unwrap().get_state() != state {
            self.myPreviousStates.push(MSPhaseDefinition::new(0, &state, &name));
        }
        self.myPreviousStates.last_mut().unwrap().duration += delta_t();
        delta_t()
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
        if self.myPreviousStates.len() > 0 {
            let dev = self.myOutputDevice.m();
            dev.open_tag_id(SUMO_TAG_TLLOGIC);
            dev.write_attr(SUMO_ATTR_ID, self.myTLSID.as_str());
            dev.write_attr(SUMO_ATTR_TYPE, "static");
            dev.write_attr(SUMO_ATTR_PROGRAMID, self.myPreviousProgramID.as_str());
            // write the phases
            for p in &self.myPreviousStates {
                dev.open_tag_id(SUMO_TAG_PHASE);
                dev.write_attr(SUMO_ATTR_DURATION, &steps2time(p.duration));
                if p.duration < time2steps(10.0) {
                    dev.write_padding(" ");
                }
                dev.write_attr(SUMO_ATTR_STATE, p.get_state());
                if p.get_name() != "" {
                    dev.write_attr(SUMO_ATTR_NAME, p.get_name());
                }
                dev.close_tag("");
            }
            // write params
            dev.close_tag("");
            self.myPreviousStates.clear();
        }
    }

    /// C++: `void Command_SaveTLSProgram::Command_SaveTLSProgram(const Command_SaveTLSProgram &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &Command_SaveTLSProgram) -> Self {
        panic!("Command_SaveTLSProgram copy constructor is private and invalidated in C++")
    }

    /// C++: `Command_SaveTLSProgram & Command_SaveTLSProgram::operator=(const Command_SaveTLSProgram &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &Command_SaveTLSProgram) -> &mut Command_SaveTLSProgram {
        panic!("Command_SaveTLSProgram assignment operator is private and invalidated in C++")
    }

}

impl Command for Command_SaveTLSProgram {
    fn execute(&mut self, currentTime: SUMOTime) -> SUMOTime {
        Command_SaveTLSProgram::execute(self, currentTime)
    }
}
