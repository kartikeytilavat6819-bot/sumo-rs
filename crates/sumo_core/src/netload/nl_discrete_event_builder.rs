//! Port of netload/NLDiscreteEventBuilder.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ enum `NLDiscreteEventBuilder::ActionType`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum NLDiscreteEventBuilder_ActionType {
    EV_SAVETLSTATE = 0,
    EV_SAVETLSWITCHES = 1,
    EV_SAVETLSWITCHSTATES = 2,
    EV_SAVETLSPROGRAM = 3,
}

/// C++ typedef `std::map<std::string, ActionType> NLDiscreteEventBuilder::KnownActions`
pub type NLDiscreteEventBuilder_KnownActions = BTreeMap<String, NLDiscreteEventBuilder_ActionType>;

/// This class is responsible for building event-handling actions which the simulation shall execute.
/// C++ class `NLDiscreteEventBuilder` (netload/NLDiscreteEventBuilder.h)
pub struct NLDiscreteEventBuilder {
    /// Build actions that shall be executed during the simulation
    pub myActions: NLDiscreteEventBuilder_KnownActions,
    pub myNet: Ptr<MSNet>,
}

impl NLDiscreteEventBuilder {
    /// C++: `void NLDiscreteEventBuilder::NLDiscreteEventBuilder(MSNet &)`
    /// Constructor
    // NLDiscreteEventBuilder::NLDiscreteEventBuilder(MSNet& net)
    //     : myNet(net) {
    //     myActions["SaveTLSStates"] = EV_SAVETLSTATE;
    //     myActions["SaveTLSSwitchTimes"] = EV_SAVETLSWITCHES;
    //     myActions["SaveTLSSwitchStates"] = EV_SAVETLSWITCHSTATES;
    //     myActions["SaveTLSProgram"] = EV_SAVETLSPROGRAM;
    // }
    #[allow(unused_variables)]
    pub fn new(net: &mut MSNet) -> Self {
        let mut myActions = BTreeMap::new();
        myActions.insert("SaveTLSStates".to_string(), NLDiscreteEventBuilder_ActionType::EV_SAVETLSTATE);
        myActions.insert("SaveTLSSwitchTimes".to_string(), NLDiscreteEventBuilder_ActionType::EV_SAVETLSWITCHES);
        myActions.insert("SaveTLSSwitchStates".to_string(), NLDiscreteEventBuilder_ActionType::EV_SAVETLSWITCHSTATES);
        myActions.insert("SaveTLSProgram".to_string(), NLDiscreteEventBuilder_ActionType::EV_SAVETLSPROGRAM);
        Self { myActions, myNet: Ptr::from_mut(net) }
    }

    /// C++: `void NLDiscreteEventBuilder::~NLDiscreteEventBuilder()`
    /// Destructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // implicit C++ destructor: nothing to release beyond the members
    }

    /// C++: `void NLDiscreteEventBuilder::addAction(const SUMOSAXAttributes &, const std::string &)`
    /// Builds an action and saves it for further use
    // void
    // NLDiscreteEventBuilder::addAction(const SUMOSAXAttributes& attrs,
    //                                   const std::string& basePath) {
    //     bool ok = true;
    //     const std::string type = attrs.getOpt<std::string>(SUMO_ATTR_TYPE, nullptr, ok, "");
    //     // check whether the type was given
    //     if (type == "" || !ok) {
    //         throw InvalidArgument("An action's type is not given.");
    //     }
    //     // get the numerical representation
    //     KnownActions::iterator i = myActions.find(type);
    //     if (i == myActions.end()) {
    //         throw InvalidArgument("The action type '" + type + "' is not known.");
    //     }
    //     // build the action
    //     switch ((*i).second) {
    //         case EV_SAVETLSTATE:
    //             buildSaveTLStateCommand(attrs, basePath);
    //             break;
    //         case EV_SAVETLSWITCHES:
    //             buildSaveTLSwitchesCommand(attrs, basePath);
    //             break;
    //         case EV_SAVETLSWITCHSTATES:
    //             buildSaveTLSwitchStatesCommand(attrs, basePath);
    //             break;
    //         case EV_SAVETLSPROGRAM:
    //             buildSaveTLSProgramCommand(attrs, basePath);
    //             break;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_action(&mut self, attrs: &SUMOSAXAttributes, basePath: &str) -> Result<(), ProcessError> {
        let mut ok = true;
        let type_ = attrs.get_opt::<String>(SUMO_ATTR_TYPE, "", &mut ok, String::new(), true);
        // check whether the type was given
        if type_ == "" || !ok {
            return Err(ProcessError::InvalidArgument("An action's type is not given.".into()));
        }
        // get the numerical representation
        let Some(&action) = self.myActions.get(&type_) else {
            return Err(ProcessError::InvalidArgument(format!("The action type '{}' is not known.", type_)));
        };
        // build the action
        match action {
            NLDiscreteEventBuilder_ActionType::EV_SAVETLSTATE => self.build_save_tl_state_command(attrs, basePath)?,
            NLDiscreteEventBuilder_ActionType::EV_SAVETLSWITCHES => self.build_save_tl_switches_command(attrs, basePath)?,
            NLDiscreteEventBuilder_ActionType::EV_SAVETLSWITCHSTATES => self.build_save_tl_switch_states_command(attrs, basePath)?,
            NLDiscreteEventBuilder_ActionType::EV_SAVETLSPROGRAM => self.build_save_tls_program_command(attrs, basePath)?,
        }
        Ok(())
    }

    /// C++: `void NLDiscreteEventBuilder::buildSaveTLStateCommand(const SUMOSAXAttributes &, const std::string &)` [private]
    /// Builds an action which saves the state of a certain tls into a file
    // void
    // NLDiscreteEventBuilder::buildSaveTLStateCommand(const SUMOSAXAttributes& attrs,
    //         const std::string& basePath) {
    //     bool ok = true;
    //     const std::string dest = attrs.getOpt<std::string>(SUMO_ATTR_DEST, nullptr, ok, "");
    //     const std::string source = attrs.getOpt<std::string>(SUMO_ATTR_SOURCE, nullptr, ok, "");
    //     const bool saveDetectors = attrs.getOpt<bool>(SUMO_ATTR_SAVE_DETECTORS, nullptr, ok, false);
    //     const bool saveConditions = attrs.getOpt<bool>(SUMO_ATTR_SAVE_CONDITIONS, nullptr, ok, false);
    //     // check the parameter
    //     if (dest == "" || !ok) {
    //         throw InvalidArgument("Incomplete description of an 'SaveTLSState'-action occurred.");
    //     }
    //     if (source == "") {
    //         const std::vector<std::string> ids = myNet.getTLSControl().getAllTLIds();
    //         for (std::vector<std::string>::const_iterator tls = ids.begin(); tls != ids.end(); ++tls) {
    //             const MSTLLogicControl::TLSLogicVariants& logics = myNet.getTLSControl().get(*tls);
    //             new Command_SaveTLSState(logics, OutputDevice::getDevice(FileHelpers::checkForRelativity(dest, basePath)),
    //                                      saveDetectors, saveConditions);
    //         }
    //     } else {
    //         // get the logic
    //         if (!myNet.getTLSControl().knows(source)) {
    //             throw InvalidArgument("The traffic light logic to save (" + source +  ") is not known.");
    //         }
    //         const MSTLLogicControl::TLSLogicVariants& logics = myNet.getTLSControl().get(source);
    //         // build the action
    //         new Command_SaveTLSState(logics, OutputDevice::getDevice(FileHelpers::checkForRelativity(dest, basePath)),
    //                                  saveDetectors, saveConditions);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn build_save_tl_state_command(&mut self, attrs: &SUMOSAXAttributes, basePath: &str) -> Result<(), ProcessError> {
        let mut ok = true;
        let dest = attrs.get_opt::<String>(SUMO_ATTR_DEST, "", &mut ok, String::new(), true);
        let source = attrs.get_opt::<String>(SUMO_ATTR_SOURCE, "", &mut ok, String::new(), true);
        let saveDetectors = attrs.get_opt::<bool>(SUMO_ATTR_SAVE_DETECTORS, "", &mut ok, false, true);
        let saveConditions = attrs.get_opt::<bool>(SUMO_ATTR_SAVE_CONDITIONS, "", &mut ok, false, true);
        // check the parameter
        if dest == "" || !ok {
            return Err(ProcessError::InvalidArgument("Incomplete description of an 'SaveTLSState'-action occurred.".into()));
        }
        if source == "" {
            let ids = self.myNet.m().get_tls_control().get_all_tl_ids();
            for tls in ids.iter() {
                let logics = self.myNet.m().get_tls_control().get(tls);
                let od = utils::iodevices::output_device::OutputDevice::get_device(&utils::common::file_helpers::check_for_relativity(&dest, basePath), true)?;
                let mut od = od.borrow_mut();
                // C++ `new` without delete: the command is owned by the event control
                Box::leak(Command_SaveTLSState::new(logics, &mut od, saveDetectors, saveConditions));
            }
        } else {
            // get the logic
            if !self.myNet.m().get_tls_control().knows(&source) {
                return Err(ProcessError::InvalidArgument(format!("The traffic light logic to save ({}) is not known.", source)));
            }
            let logics = self.myNet.m().get_tls_control().get(&source);
            // build the action
            let od = utils::iodevices::output_device::OutputDevice::get_device(&utils::common::file_helpers::check_for_relativity(&dest, basePath), true)?;
            let mut od = od.borrow_mut();
            Box::leak(Command_SaveTLSState::new(logics, &mut od, saveDetectors, saveConditions));
        }
        Ok(())
    }

    /// C++: `void NLDiscreteEventBuilder::buildSaveTLSwitchesCommand(const SUMOSAXAttributes &, const std::string &)` [private]
    /// Builds an action which saves the switch times of links into a file
    // void
    // NLDiscreteEventBuilder::buildSaveTLSwitchesCommand(const SUMOSAXAttributes& attrs,
    //         const std::string& basePath) {
    //     bool ok = true;
    //     const std::string dest = attrs.getOpt<std::string>(SUMO_ATTR_DEST, nullptr, ok, "");
    //     const std::string source = attrs.getOpt<std::string>(SUMO_ATTR_SOURCE, nullptr, ok, "");
    //     // check the parameter
    //     if (dest == "" || !ok) {
    //         throw InvalidArgument("Incomplete description of an 'SaveTLSSwitchTimes'-action occurred.");
    //     }
    //     if (source == "") {
    //         const std::vector<std::string> ids = myNet.getTLSControl().getAllTLIds();
    //         for (std::vector<std::string>::const_iterator tls = ids.begin(); tls != ids.end(); ++tls) {
    //             const MSTLLogicControl::TLSLogicVariants& logics = myNet.getTLSControl().get(*tls);
    //             new Command_SaveTLSSwitches(logics, OutputDevice::getDevice(FileHelpers::checkForRelativity(dest, basePath)));
    //         }
    //     } else {
    //         // get the logic
    //         if (!myNet.getTLSControl().knows(source)) {
    //             throw InvalidArgument("The traffic light logic to save (" + source +  ") is not known.");
    //         }
    //         const MSTLLogicControl::TLSLogicVariants& logics = myNet.getTLSControl().get(source);
    //         // build the action
    //         new Command_SaveTLSSwitches(logics, OutputDevice::getDevice(FileHelpers::checkForRelativity(dest, basePath)));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn build_save_tl_switches_command(&mut self, attrs: &SUMOSAXAttributes, basePath: &str) -> Result<(), ProcessError> {
        let mut ok = true;
        let dest = attrs.get_opt::<String>(SUMO_ATTR_DEST, "", &mut ok, String::new(), true);
        let source = attrs.get_opt::<String>(SUMO_ATTR_SOURCE, "", &mut ok, String::new(), true);
        // check the parameter
        if dest == "" || !ok {
            return Err(ProcessError::InvalidArgument("Incomplete description of an 'SaveTLSSwitchTimes'-action occurred.".into()));
        }
        if source == "" {
            let ids = self.myNet.m().get_tls_control().get_all_tl_ids();
            for tls in ids.iter() {
                let logics = self.myNet.m().get_tls_control().get(tls);
                let od = utils::iodevices::output_device::OutputDevice::get_device(&utils::common::file_helpers::check_for_relativity(&dest, basePath), true)?;
                let mut od = od.borrow_mut();
                Box::leak(Command_SaveTLSSwitches::new(logics, &mut od));
            }
        } else {
            // get the logic
            if !self.myNet.m().get_tls_control().knows(&source) {
                return Err(ProcessError::InvalidArgument(format!("The traffic light logic to save ({}) is not known.", source)));
            }
            let logics = self.myNet.m().get_tls_control().get(&source);
            // build the action
            let od = utils::iodevices::output_device::OutputDevice::get_device(&utils::common::file_helpers::check_for_relativity(&dest, basePath), true)?;
            let mut od = od.borrow_mut();
            Box::leak(Command_SaveTLSSwitches::new(logics, &mut od));
        }
        Ok(())
    }

    /// C++: `void NLDiscreteEventBuilder::buildSaveTLSwitchStatesCommand(const SUMOSAXAttributes &, const std::string &)` [private]
    /// Builds an action which saves the switch times and states of tls into a file
    // void
    // NLDiscreteEventBuilder::buildSaveTLSwitchStatesCommand(const SUMOSAXAttributes& attrs,
    //         const std::string& basePath) {
    //     bool ok = true;
    //     const std::string dest = attrs.getOpt<std::string>(SUMO_ATTR_DEST, nullptr, ok, "");
    //     const std::string source = attrs.getOpt<std::string>(SUMO_ATTR_SOURCE, nullptr, ok, "");
    //     // check the parameter
    //     if (dest == "" || !ok) {
    //         throw InvalidArgument("Incomplete description of an 'SaveTLSSwitchStates'-action occurred.");
    //     }
    //     if (source == "") {
    //         const std::vector<std::string> ids = myNet.getTLSControl().getAllTLIds();
    //         for (std::vector<std::string>::const_iterator tls = ids.begin(); tls != ids.end(); ++tls) {
    //             const MSTLLogicControl::TLSLogicVariants& logics = myNet.getTLSControl().get(*tls);
    //             new Command_SaveTLSSwitchStates(logics, OutputDevice::getDevice(FileHelpers::checkForRelativity(dest, basePath)));
    //         }
    //     } else {
    //         // get the logic
    //         if (!myNet.getTLSControl().knows(source)) {
    //             throw InvalidArgument("The traffic light logic to save (" + source +  ") is not known.");
    //         }
    //         const MSTLLogicControl::TLSLogicVariants& logics = myNet.getTLSControl().get(source);
    //         // build the action
    //         new Command_SaveTLSSwitchStates(logics, OutputDevice::getDevice(FileHelpers::checkForRelativity(dest, basePath)));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn build_save_tl_switch_states_command(&mut self, attrs: &SUMOSAXAttributes, basePath: &str) -> Result<(), ProcessError> {
        let mut ok = true;
        let dest = attrs.get_opt::<String>(SUMO_ATTR_DEST, "", &mut ok, String::new(), true);
        let source = attrs.get_opt::<String>(SUMO_ATTR_SOURCE, "", &mut ok, String::new(), true);
        // check the parameter
        if dest == "" || !ok {
            return Err(ProcessError::InvalidArgument("Incomplete description of an 'SaveTLSSwitchStates'-action occurred.".into()));
        }
        if source == "" {
            let ids = self.myNet.m().get_tls_control().get_all_tl_ids();
            for tls in ids.iter() {
                let logics = self.myNet.m().get_tls_control().get(tls);
                let od = utils::iodevices::output_device::OutputDevice::get_device(&utils::common::file_helpers::check_for_relativity(&dest, basePath), true)?;
                let mut od = od.borrow_mut();
                Box::leak(Command_SaveTLSSwitchStates::new(logics, &mut od));
            }
        } else {
            // get the logic
            if !self.myNet.m().get_tls_control().knows(&source) {
                return Err(ProcessError::InvalidArgument(format!("The traffic light logic to save ({}) is not known.", source)));
            }
            let logics = self.myNet.m().get_tls_control().get(&source);
            // build the action
            let od = utils::iodevices::output_device::OutputDevice::get_device(&utils::common::file_helpers::check_for_relativity(&dest, basePath), true)?;
            let mut od = od.borrow_mut();
            Box::leak(Command_SaveTLSSwitchStates::new(logics, &mut od));
        }
        Ok(())
    }

    /// C++: `void NLDiscreteEventBuilder::buildSaveTLSProgramCommand(const SUMOSAXAttributes &, const std::string &)` [private]
    /// Builds an action which saves the tls states as a loadable program into a file
    // void
    // NLDiscreteEventBuilder::buildSaveTLSProgramCommand(const SUMOSAXAttributes& attrs,
    //         const std::string& basePath) {
    //     bool ok = true;
    //     const std::string dest = attrs.getOpt<std::string>(SUMO_ATTR_DEST, nullptr, ok, "");
    //     const std::string source = attrs.getOpt<std::string>(SUMO_ATTR_SOURCE, nullptr, ok, "");
    //     // check the parameter
    //     if (dest == "" || !ok) {
    //         throw InvalidArgument("Incomplete description of an 'SaveTLSProgram'-action occurred.");
    //     }
    //     if (source == "") {
    //         const std::vector<std::string> ids = myNet.getTLSControl().getAllTLIds();
    //         for (std::vector<std::string>::const_iterator tls = ids.begin(); tls != ids.end(); ++tls) {
    //             const MSTLLogicControl::TLSLogicVariants& logics = myNet.getTLSControl().get(*tls);
    //             new Command_SaveTLSProgram(logics, OutputDevice::getDevice(FileHelpers::checkForRelativity(dest, basePath)));
    //         }
    //     } else {
    //         // get the logic
    //         if (!myNet.getTLSControl().knows(source)) {
    //             throw InvalidArgument("The traffic light logic to save (" + source +  ") is not known.");
    //         }
    //         const MSTLLogicControl::TLSLogicVariants& logics = myNet.getTLSControl().get(source);
    //         // build the action
    //         new Command_SaveTLSProgram(logics, OutputDevice::getDevice(FileHelpers::checkForRelativity(dest, basePath)));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn build_save_tls_program_command(&mut self, attrs: &SUMOSAXAttributes, basePath: &str) -> Result<(), ProcessError> {
        let mut ok = true;
        let dest = attrs.get_opt::<String>(SUMO_ATTR_DEST, "", &mut ok, String::new(), true);
        let source = attrs.get_opt::<String>(SUMO_ATTR_SOURCE, "", &mut ok, String::new(), true);
        // check the parameter
        if dest == "" || !ok {
            return Err(ProcessError::InvalidArgument("Incomplete description of an 'SaveTLSProgram'-action occurred.".into()));
        }
        if source == "" {
            let ids = self.myNet.m().get_tls_control().get_all_tl_ids();
            for tls in ids.iter() {
                let logics = self.myNet.m().get_tls_control().get(tls);
                let od = utils::iodevices::output_device::OutputDevice::get_device(&utils::common::file_helpers::check_for_relativity(&dest, basePath), true)?;
                let mut od = od.borrow_mut();
                Box::leak(Command_SaveTLSProgram::new(logics, &mut od));
            }
        } else {
            // get the logic
            if !self.myNet.m().get_tls_control().knows(&source) {
                return Err(ProcessError::InvalidArgument(format!("The traffic light logic to save ({}) is not known.", source)));
            }
            let logics = self.myNet.m().get_tls_control().get(&source);
            // build the action
            let od = utils::iodevices::output_device::OutputDevice::get_device(&utils::common::file_helpers::check_for_relativity(&dest, basePath), true)?;
            let mut od = od.borrow_mut();
            Box::leak(Command_SaveTLSProgram::new(logics, &mut od));
        }
        Ok(())
    }

    /// C++: `NLDiscreteEventBuilder & NLDiscreteEventBuilder::operator=(const NLDiscreteEventBuilder &)` [private]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &NLDiscreteEventBuilder) -> &mut NLDiscreteEventBuilder {
        // implicit C++ copy assignment: member-wise copy
        self.myActions = arg0.myActions.clone();
        self.myNet = arg0.myNet;
        self
    }

}

