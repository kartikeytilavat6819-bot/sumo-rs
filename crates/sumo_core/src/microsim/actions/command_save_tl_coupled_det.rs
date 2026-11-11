//! Port of microsim/actions/Command_SaveTLCoupledDet.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// Writes e2 state on each tls switch
/// C++ class `Command_SaveTLCoupledDet` (microsim/actions/Command_SaveTLCoupledDet.h)
pub struct Command_SaveTLCoupledDet {
    /// C++ base class `MSTLLogicControl::OnSwitchAction`
    pub base: MSTLLogicControl_OnSwitchAction,
    /// The file to write the output to
    pub myDevice: Ptr<OutputDevice>,
    /// The logic to use
    pub myLogics: Ptr<MSTLLogicControl_TLSLogicVariants>,
    /// The detector to use
    pub myDetector: Ptr<MSDetectorFileOutput>,
    /// The last time the values were written
    pub myStartTime: SUMOTime,
}

impl std::ops::Deref for Command_SaveTLCoupledDet {
    type Target = MSTLLogicControl_OnSwitchAction;
    fn deref(&self) -> &MSTLLogicControl_OnSwitchAction {
        &self.base
    }
}
impl std::ops::DerefMut for Command_SaveTLCoupledDet {
    fn deref_mut(&mut self) -> &mut MSTLLogicControl_OnSwitchAction {
        &mut self.base
    }
}

impl Command_SaveTLCoupledDet {
    /// C++: `void Command_SaveTLCoupledDet::Command_SaveTLCoupledDet(MSTLLogicControl::TLSLogicVariants &, MSDetectorFileOutput *, SUMOTime, OutputDevice &)`
    /// Constructor
    // Command_SaveTLCoupledDet::Command_SaveTLCoupledDet(MSTLLogicControl::TLSLogicVariants& tlls,
    //         MSDetectorFileOutput* dtf, SUMOTime begin, OutputDevice& device)
    //     : myDevice(device), myLogics(tlls), myDetector(dtf),
    //       myStartTime(begin) {
    //     tlls.addSwitchCommand(this);
    //     dtf->writeXMLDetectorProlog(device);
    // }
    #[allow(unused_variables)]
    pub fn new(tlls: &mut MSTLLogicControl_TLSLogicVariants, dtf: Ptr<MSDetectorFileOutput>, begin: SUMOTime, device: &mut OutputDevice) -> Self {
        todo!("port Command_SaveTLCoupledDet::Command_SaveTLCoupledDet")
    }

    /// C++: `void Command_SaveTLCoupledDet::~Command_SaveTLCoupledDet() noexcept` [virtual]
    /// Destructor
    // Command_SaveTLCoupledDet::~Command_SaveTLCoupledDet() {
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port Command_SaveTLCoupledDet::~Command_SaveTLCoupledDet")
    }

    /// C++: `void Command_SaveTLCoupledDet::execute()` [virtual]
    /// Executes the command
    // void
    // Command_SaveTLCoupledDet::execute() {
    //     SUMOTime end = MSNet::getInstance()->getCurrentTimeStep();
    //     if (myStartTime != end) {
    //         myDetector->writeXMLOutput(myDevice, myStartTime, end);
    //         myStartTime = end;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn execute(&mut self) {
        todo!("port Command_SaveTLCoupledDet::execute")
    }

    /// C++: `void Command_SaveTLCoupledDet::Command_SaveTLCoupledDet(const Command_SaveTLCoupledDet &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &Command_SaveTLCoupledDet) -> Self {
        todo!("port Command_SaveTLCoupledDet::Command_SaveTLCoupledDet")
    }

    /// C++: `Command_SaveTLCoupledDet & Command_SaveTLCoupledDet::operator=(const Command_SaveTLCoupledDet &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &Command_SaveTLCoupledDet) -> &mut Command_SaveTLCoupledDet {
        todo!("port Command_SaveTLCoupledDet::operator=")
    }

}

