//! Port of microsim/Command_RouteReplacement.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::msg_handler::write_warning;
use utils::options::options_cont;

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
        Command_RouteReplacement {
            base: (),
            myVehID: vehID.to_string(),
            myRoute: route,
        }
    }

    /// C++: `void Command_RouteReplacement::~Command_RouteReplacement() noexcept` [virtual]
    /// Destructor
    // Command_RouteReplacement::~Command_RouteReplacement() { }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
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
        let veh = MSNet::get_instance().m().get_vehicle_control().get_vehicle(&self.myVehID);
        // if the vehicle is not available anymore, silently ignore replacement
        if !veh.is_null() {
            let veh = veh.m();
            let errorPrefix = format!(
                "Replayed route replacement failed for vehicle '{}' route={} time={}",
                veh.get_id(),
                self.myRoute.get_id(),
                time2string(MSNet::get_instance().get_current_time_step())
            );
            let mut msg = String::new();
            if !veh.has_valid_route(&mut msg, self.myRoute.clone()) {
                write_warning(&format!("Invalid route replacement for vehicle '{}'. {}", veh.get_id(), msg));
                // shortcut: MSGlobals::gCheckRoutes is not ported (ms_globals.rs has no statics); read its source option (set as !ignore-route-errors in MSFrame) directly, upgrade when MSGlobals statics land
                if !options_cont::get_bool("ignore-route-errors") {
                    panic!("{}", ProcessError::Process(format!("{}.", errorPrefix)));
                }
            }
            let mut errorMsg = String::new();
            let onInit = veh.get_lane().is_null();
            let replacedIndex = veh.get_route().get_replaced_index();
            if !veh.replace_route(self.myRoute.clone(), "replayRerouting", onInit, replacedIndex, true, true, Ptr::from_mut(&mut errorMsg)) {
                panic!("{}", ProcessError::Process(format!("{} ({}).", errorPrefix, errorMsg)));
            }
        }
        0
    }

    /// C++: `void Command_RouteReplacement::Command_RouteReplacement(const Command_RouteReplacement &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &Command_RouteReplacement) -> Self {
        panic!("Command_RouteReplacement copy constructor is invalidated")
    }

    /// C++: `Command_RouteReplacement & Command_RouteReplacement::operator=(const Command_RouteReplacement &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &Command_RouteReplacement) -> &mut Command_RouteReplacement {
        panic!("Command_RouteReplacement assignment operator is invalidated")
    }

}


impl Command for Command_RouteReplacement {
    fn execute(&mut self, currentTime: SUMOTime) -> SUMOTime {
        Command_RouteReplacement::execute(self, currentTime)
    }
}
