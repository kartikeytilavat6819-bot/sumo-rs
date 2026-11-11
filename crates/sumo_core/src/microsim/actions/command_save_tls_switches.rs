//! Port of microsim/actions/Command_SaveTLSSwitches.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// Writes information about the green durations of a tls
/// C++ class `Command_SaveTLSSwitches` (microsim/actions/Command_SaveTLSSwitches.h)
pub struct Command_SaveTLSSwitches {
    /// C++ base class `Command`
    pub base: Unmapped /* Command */,
    /// The device to write to
    pub myOutputDevice: Ptr<OutputDevice>,
    /// The traffic light logic to use
    pub myLogics: Ptr<MSTLLogicControl_TLSLogicVariants>,
    /// Storage for prior states; map from signal group to last green time begin
    pub myPreviousLinkStates: BTreeMap<i32, SUMOTime>,
}

impl std::ops::Deref for Command_SaveTLSSwitches {
    type Target = Unmapped /* Command */;
    fn deref(&self) -> &Unmapped /* Command */ {
        &self.base
    }
}
impl std::ops::DerefMut for Command_SaveTLSSwitches {
    fn deref_mut(&mut self) -> &mut Unmapped /* Command */ {
        &mut self.base
    }
}

impl Command_SaveTLSSwitches {
    /// C++: `void Command_SaveTLSSwitches::Command_SaveTLSSwitches(const MSTLLogicControl::TLSLogicVariants &, OutputDevice &)`
    /// Constructor
    // Command_SaveTLSSwitches::Command_SaveTLSSwitches(const MSTLLogicControl::TLSLogicVariants& logics,
    //         OutputDevice& od)
    //     : myOutputDevice(od), myLogics(logics) {
    //     MSNet::getInstance()->getEndOfTimestepEvents()->addEvent(this);
    //     myOutputDevice.writeXMLHeader("tlsSwitches", "tlsswitches_file.xsd");
    // }
    #[allow(unused_variables)]
    pub fn new(logics: &MSTLLogicControl_TLSLogicVariants, od: &mut OutputDevice) -> Self {
        todo!("port Command_SaveTLSSwitches::Command_SaveTLSSwitches")
    }

    /// C++: `void Command_SaveTLSSwitches::~Command_SaveTLSSwitches() noexcept` [virtual]
    /// Destructor
    // Command_SaveTLSSwitches::~Command_SaveTLSSwitches() {
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port Command_SaveTLSSwitches::~Command_SaveTLSSwitches")
    }

    /// C++: `SUMOTime Command_SaveTLSSwitches::execute(SUMOTime)` [virtual]
    /// Writes the output if a change occurred
    // SUMOTime
    // Command_SaveTLSSwitches::execute(SUMOTime currentTime) {
    //     MSTrafficLightLogic* light = myLogics.getActive();
    //     const MSTrafficLightLogic::LinkVectorVector& links = light->getLinks();
    //     const std::string& state = light->getCurrentPhaseDef().getState();
    //     for (int i = 0; i < (int) links.size(); i++) {
    //         if (state[i] == LINKSTATE_TL_GREEN_MAJOR || state[i] == LINKSTATE_TL_GREEN_MINOR) {
    //             if (myPreviousLinkStates.find(i) == myPreviousLinkStates.end()) {
    //                 // was not saved before
    //                 myPreviousLinkStates[i] = currentTime;
    //                 continue;
    //             }
    //         } else {
    //             if (myPreviousLinkStates.find(i) == myPreviousLinkStates.end()) {
    //                 // was not yet green
    //                 continue;
    //             }
    //             const MSTrafficLightLogic::LinkVector& currLinks = links[i];
    //             const MSTrafficLightLogic::LaneVector& currLanes = light->getLanesAt(i);
    //             SUMOTime lastOn = myPreviousLinkStates[i];
    //             for (int j = 0; j < (int) currLinks.size(); j++) {
    //                 MSLink* link = currLinks[j];
    //                 myOutputDevice << "   <tlsSwitch id=\"" << light->getID()
    //                                << "\" programID=\"" << light->getProgramID()
    //                                << "\" fromLane=\"" << currLanes[j]->getID()
    //                                << "\" toLane=\"" << link->getLane()->getID()
    //                                << "\" begin=\"" << time2string(lastOn)
    //                                << "\" end=\"" << time2string(currentTime)
    //                                << "\" duration=\"" << time2string(currentTime - lastOn)
    //                                << "\"/>\n";
    //             }
    //             myPreviousLinkStates.erase(myPreviousLinkStates.find(i));
    //         }
    //     }
    //     return DELTA_T;
    // }
    #[allow(unused_variables)]
    pub fn execute(&mut self, currentTime: SUMOTime) -> SUMOTime {
        todo!("port Command_SaveTLSSwitches::execute")
    }

    /// C++: `void Command_SaveTLSSwitches::Command_SaveTLSSwitches(const Command_SaveTLSSwitches &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &Command_SaveTLSSwitches) -> Self {
        todo!("port Command_SaveTLSSwitches::Command_SaveTLSSwitches")
    }

    /// C++: `Command_SaveTLSSwitches & Command_SaveTLSSwitches::operator=(const Command_SaveTLSSwitches &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &Command_SaveTLSSwitches) -> &mut Command_SaveTLSSwitches {
        todo!("port Command_SaveTLSSwitches::operator=")
    }

}

