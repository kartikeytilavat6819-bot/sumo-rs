//! Port of microsim/Command_RouteReplacement.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// Writes the state of the tls to a file (in each second)
/// C++ class `Command_RouteReplacement` (microsim/Command_RouteReplacement.h)
pub struct Command_RouteReplacement {
    /// C++ base class `Command`
    pub base: Unmapped /* Command */,
    /// The id of the vehicle to be rerouted
    pub myVehID: String,
    /// The replacement route
    pub myRoute: ConstMSRoutePtr,
}

impl std::ops::Deref for Command_RouteReplacement {
    type Target = Unmapped /* Command */;
    fn deref(&self) -> &Unmapped /* Command */ {
        &self.base
    }
}
impl std::ops::DerefMut for Command_RouteReplacement {
    fn deref_mut(&mut self) -> &mut Unmapped /* Command */ {
        &mut self.base
    }
}

impl Command_RouteReplacement {
    /// C++: `void Command_RouteReplacement::Command_RouteReplacement(const std::string &, ConstMSRoutePtr)`
    /// Constructor
    // Command_RouteReplacement::Command_RouteReplacement(const std::string& vehID, ConstMSRoutePtr route) :
    //     myVehID(vehID),
    //     myRoute(route) {
    // }
    #[allow(unused_variables)]
    pub fn new(vehID: &str, route: ConstMSRoutePtr) -> Self {
        todo!("port Command_RouteReplacement::Command_RouteReplacement")
    }

    /// C++: `void Command_RouteReplacement::~Command_RouteReplacement() noexcept` [virtual]
    /// Destructor
    // Command_RouteReplacement::~Command_RouteReplacement() { }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port Command_RouteReplacement::~Command_RouteReplacement")
    }

    /// C++: `SUMOTime Command_RouteReplacement::execute(SUMOTime)` [virtual]
    /// toggles override
    // SUMOTime
    // Command_RouteReplacement::execute(SUMOTime /*currentTime*/) {
    //     SUMOVehicle* veh = MSNet::getInstance()->getVehicleControl().getVehicle(myVehID);
    //     // if the vehicle is not available anymore, silently ignore replacement
    //     if (veh != nullptr) {
    //         std::string errorPrefix = ("Replayed route replacement failed for vehicle '"
    //                                    + veh->getID() + "' route=" + myRoute->getID() + " time=" + time2string(SIMSTEP));
    //         std::string msg;
    //         if (!veh->hasValidRoute(msg, myRoute)) {
    //             WRITE_WARNING("Invalid route replacement for vehicle '" + veh->getID() + "'. " + msg);
    //             if (MSGlobals::gCheckRoutes) {
    //                 throw ProcessError(errorPrefix + ".");
    //             }
    //         }
    //         std::string errorMsg;
    //         if (!veh->replaceRoute(myRoute, "replayRerouting", veh->getLane() == nullptr,
    //                                veh->getRoute().getReplacedIndex(), true, true, &errorMsg)) {
    //             throw ProcessError(errorPrefix + " (" + errorMsg + ").");
    //         }
    //     }
    //     return 0;
    // }
    #[allow(unused_variables)]
    pub fn execute(&mut self, currentTime: SUMOTime) -> SUMOTime {
        todo!("port Command_RouteReplacement::execute")
    }

    /// C++: `void Command_RouteReplacement::Command_RouteReplacement(const Command_RouteReplacement &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &Command_RouteReplacement) -> Self {
        todo!("port Command_RouteReplacement::Command_RouteReplacement")
    }

    /// C++: `Command_RouteReplacement & Command_RouteReplacement::operator=(const Command_RouteReplacement &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &Command_RouteReplacement) -> &mut Command_RouteReplacement {
        todo!("port Command_RouteReplacement::operator=")
    }

}

