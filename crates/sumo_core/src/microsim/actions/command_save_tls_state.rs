//! Port of microsim/actions/Command_SaveTLSState.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// Writes the state of the tls to a file (in each second)
/// C++ class `Command_SaveTLSState` (microsim/actions/Command_SaveTLSState.h)
#[repr(C)]
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
    pub fn new(logics: &MSTLLogicControl_TLSLogicVariants, od: &mut OutputDevice, saveDetectors: bool, saveConditions: bool) -> Box<Self> {
        let mut b = Box::new(Self {
            base: (),
            myOutputDevice: Ptr::from_mut(od),
            myLogics: Ptr::new(logics),
            mySaveDetectors: saveDetectors,
            mySaveConditions: saveConditions,
        });
        // C++ constructor: MSNet::getInstance()->getEndOfTimestepEvents()->addEvent(this)
        MSNet::get_instance().m().get_end_of_timestep_events().m().add_event(Ptr::from_mut(&mut *b as &mut dyn Command), -1);
        b
    }

    /// C++: `void Command_SaveTLSState::~Command_SaveTLSState() noexcept` [virtual]
    /// Destructor
    // Command_SaveTLSState::~Command_SaveTLSState() {
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
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
        let dev = self.myOutputDevice.m();
        if !dev.wrote_header() {
            // delay writing header to ensure tls is initialized
            let mut attrs: BTreeMap<SumoXMLAttr, String> = BTreeMap::new();
            if self.mySaveDetectors {
                let IDs: Vec<String> = self.myLogics.get_active().get_detector_states().into_keys().collect();
                attrs.insert(SUMO_ATTR_DETECTORS, utils::common::to_string::join_to_string(&IDs, " "));
            }
            if self.mySaveConditions {
                let IDs: Vec<String> = self.myLogics.get_active().get_conditions().into_keys().collect();
                attrs.insert(SUMO_ATTR_CONDITIONS, utils::common::to_string::join_to_string(&IDs, " "));
            }
            dev.write_xml_header("tlsStates", "tlsstates_file.xsd", attrs, true);
        }
        let logic = self.myLogics.get_active();
        dev.open_tag("tlsState");
        dev.write_attr(SUMO_ATTR_TIME, &time2string(currentTime));
        dev.write_attr(SUMO_ATTR_ID, logic.get_id());
        dev.write_attr(SUMO_ATTR_PROGRAMID, logic.get_program_id());
        dev.write_attr(SUMO_ATTR_PHASE, &logic.get_current_phase_index());
        dev.write_attr(SUMO_ATTR_STATE, logic.get_current_phase_def().get_state());
        if !logic.get_current_phase_def().get_name().is_empty() {
            dev.write_attr(SUMO_ATTR_NAME, logic.get_current_phase_def().get_name());
        }
        if self.mySaveDetectors {
            let states: Vec<i32> = logic.get_detector_states().values().map(|item| *item as i32).collect();
            dev.write_attr(SUMO_ATTR_DETECTORS, &utils::common::to_string::join_to_string(&states, " "));
        }
        if self.mySaveConditions {
            let states: Vec<f64> = logic.get_conditions().values().cloned().collect();
            dev.write_attr(SUMO_ATTR_CONDITIONS, &utils::common::to_string::join_to_string(&states, " "));
        }
        dev.close_tag("");
        delta_t()
    }

    /// C++: `void Command_SaveTLSState::Command_SaveTLSState(const Command_SaveTLSState &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &Command_SaveTLSState) -> Self {
        panic!("Command_SaveTLSState copy constructor is private and invalidated in C++")
    }

    /// C++: `Command_SaveTLSState & Command_SaveTLSState::operator=(const Command_SaveTLSState &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &Command_SaveTLSState) -> &mut Command_SaveTLSState {
        panic!("Command_SaveTLSState assignment operator is private and invalidated in C++")
    }

}

impl Command for Command_SaveTLSState {
    fn execute(&mut self, currentTime: SUMOTime) -> SUMOTime {
        Command_SaveTLSState::execute(self, currentTime)
    }
}
