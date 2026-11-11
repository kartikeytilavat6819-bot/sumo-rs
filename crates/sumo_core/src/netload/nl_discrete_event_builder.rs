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

/// C++ typedef `int NLDiscreteEventBuilder::KnownActions`
pub type NLDiscreteEventBuilder_KnownActions = i32;

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
        todo!("port NLDiscreteEventBuilder::NLDiscreteEventBuilder")
    }

    /// C++: `void NLDiscreteEventBuilder::~NLDiscreteEventBuilder()`
    /// Destructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port NLDiscreteEventBuilder::~NLDiscreteEventBuilder")
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
    pub fn add_action(&mut self, attrs: &SUMOSAXAttributes, basePath: &str) {
        todo!("port NLDiscreteEventBuilder::addAction")
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
    pub fn build_save_tl_state_command(&mut self, attrs: &SUMOSAXAttributes, basePath: &str) {
        todo!("port NLDiscreteEventBuilder::buildSaveTLStateCommand")
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
    pub fn build_save_tl_switches_command(&mut self, attrs: &SUMOSAXAttributes, basePath: &str) {
        todo!("port NLDiscreteEventBuilder::buildSaveTLSwitchesCommand")
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
    pub fn build_save_tl_switch_states_command(&mut self, attrs: &SUMOSAXAttributes, basePath: &str) {
        todo!("port NLDiscreteEventBuilder::buildSaveTLSwitchStatesCommand")
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
    pub fn build_save_tls_program_command(&mut self, attrs: &SUMOSAXAttributes, basePath: &str) {
        todo!("port NLDiscreteEventBuilder::buildSaveTLSProgramCommand")
    }

    /// C++: `NLDiscreteEventBuilder & NLDiscreteEventBuilder::operator=(const NLDiscreteEventBuilder &)` [private]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &NLDiscreteEventBuilder) -> &mut NLDiscreteEventBuilder {
        todo!("port NLDiscreteEventBuilder::operator=")
    }

}

