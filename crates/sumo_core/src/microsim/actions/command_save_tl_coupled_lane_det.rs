//! Port of microsim/actions/Command_SaveTLCoupledLaneDet.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// Writes e2 state of a link for the time the link has yellow/red
/// C++ class `Command_SaveTLCoupledLaneDet` (microsim/actions/Command_SaveTLCoupledLaneDet.h)
pub struct Command_SaveTLCoupledLaneDet {
    /// C++ base class `Command_SaveTLCoupledDet`
    pub base: Command_SaveTLCoupledDet,
    /// The link to observe
    pub myLink: Ptr<MSLink>,
    /// The state the link had the last time
    pub myLastState: LinkState,
    /// Whether the last link state was already saved
    pub myHadOne: bool,
}

impl std::ops::Deref for Command_SaveTLCoupledLaneDet {
    type Target = Command_SaveTLCoupledDet;
    fn deref(&self) -> &Command_SaveTLCoupledDet {
        &self.base
    }
}
impl std::ops::DerefMut for Command_SaveTLCoupledLaneDet {
    fn deref_mut(&mut self) -> &mut Command_SaveTLCoupledDet {
        &mut self.base
    }
}

impl Command_SaveTLCoupledLaneDet {
    /// C++: `void Command_SaveTLCoupledLaneDet::Command_SaveTLCoupledLaneDet(MSTLLogicControl::TLSLogicVariants &, MSDetectorFileOutput *, SUMOTime, OutputDevice &, const MSLink *)`
    /// Constructor
    // Command_SaveTLCoupledLaneDet::Command_SaveTLCoupledLaneDet(MSTLLogicControl::TLSLogicVariants& tlls,
    //         MSDetectorFileOutput* dtf, SUMOTime begin, OutputDevice& device, const MSLink* link)
    //     : Command_SaveTLCoupledDet(tlls, dtf, begin, device),
    //       myLink(link), myLastState(LINKSTATE_TL_RED),
    //       myHadOne(false) {
    //     execute();
    // }
    #[allow(unused_variables)]
    pub fn new(tlls: &mut MSTLLogicControl_TLSLogicVariants, dtf: Ptr<MSDetectorFileOutput>, begin: SUMOTime, device: &mut OutputDevice, link: Ptr<MSLink>) -> Self {
        todo!("port Command_SaveTLCoupledLaneDet::Command_SaveTLCoupledLaneDet")
    }

    /// C++: `void Command_SaveTLCoupledLaneDet::~Command_SaveTLCoupledLaneDet() noexcept` [virtual]
    /// Destructor
    // Command_SaveTLCoupledLaneDet::~Command_SaveTLCoupledLaneDet() {
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port Command_SaveTLCoupledLaneDet::~Command_SaveTLCoupledLaneDet")
    }

    /// C++: `void Command_SaveTLCoupledLaneDet::execute()` [virtual]
    /// Executes the command
    // void
    // Command_SaveTLCoupledLaneDet::execute() {
    //     if (myLink->getState() == myLastState && myHadOne) {
    //         return;
    //     }
    //     myHadOne = true;
    //     if (myLastState == LINKSTATE_TL_RED && myLink->getState() != LINKSTATE_TL_RED) {
    //         SUMOTime end = MSNet::getInstance()->getCurrentTimeStep();
    //         if (myStartTime != end) {
    //             myDetector->writeXMLOutput(myDevice, myStartTime, end);
    //             myStartTime = end;
    //         }
    //     } else if (myLink->getState() == LINKSTATE_TL_RED) {
    //         myDetector->reset();
    //         myStartTime = MSNet::getInstance()->getCurrentTimeStep();
    //     }
    //     myLastState = myLink->getState();
    // }
    #[allow(unused_variables)]
    pub fn execute(&mut self) {
        todo!("port Command_SaveTLCoupledLaneDet::execute")
    }

    /// C++: `void Command_SaveTLCoupledLaneDet::Command_SaveTLCoupledLaneDet(const Command_SaveTLCoupledLaneDet &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &Command_SaveTLCoupledLaneDet) -> Self {
        todo!("port Command_SaveTLCoupledLaneDet::Command_SaveTLCoupledLaneDet")
    }

    /// C++: `Command_SaveTLCoupledLaneDet & Command_SaveTLCoupledLaneDet::operator=(const Command_SaveTLCoupledLaneDet &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &Command_SaveTLCoupledLaneDet) -> &mut Command_SaveTLCoupledLaneDet {
        todo!("port Command_SaveTLCoupledLaneDet::operator=")
    }

}

