//! Port of libsumo/Subscription.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::traci::traci_constants::*;

/// C++ enum `libsumo::SubscriptionFilterType`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum SubscriptionFilterType {
    SUBS_FILTER_NONE = 0,
    SUBS_FILTER_LANES = 1,
    SUBS_FILTER_NOOPPOSITE = 2,
    SUBS_FILTER_DOWNSTREAM_DIST = 4,
    SUBS_FILTER_UPSTREAM_DIST = 8,
    SUBS_FILTER_LEAD_FOLLOW = 16,
    SUBS_FILTER_TURN = 64,
    SUBS_FILTER_VCLASS = 128,
    SUBS_FILTER_VTYPE = 256,
    SUBS_FILTER_FIELD_OF_VISION = 512,
    SUBS_FILTER_LATERAL_DIST = 1024,
    SUBS_FILTER_NO_RTREE = 1117,
    SUBS_FILTER_MANEUVER = 80,
}

/// C++ typedef `bool (*)(const std::string &, const int, VariableWrapper *, tcpip::Storage *) VariableWrapper::SubscriptionHandler`
pub type VariableWrapper_SubscriptionHandler = Unmapped /* bool (*)(const std::string &, const int, VariableWrapper *, tcpip::Storage *) */;

/// Representation of a subscription
/// C++ class `libsumo::Subscription` (libsumo/Subscription.h)
pub struct Subscription {
    /// commandIdArg The command id of the subscription
    pub commandId: i32,
    /// The id of the object that is subscribed
    pub id: String,
    /// The subscribed variables
    pub variables: Vec<i32>,
    /// The parameters for the subscribed variables
    pub parameters: Vec<Rc<Storage>>,
    /// The begin time of the subscription
    pub beginTime: SUMOTime,
    /// The end time of the subscription
    pub endTime: SUMOTime,
    /// The domain ID of the context
    pub contextDomain: i32,
    /// The range of the context
    pub range: f64,
    /// Active filters for the subscription (bitset,
    pub activeFilters: i32,
    /// lanes specified by the lanes filter
    pub filterLanes: Vec<i32>,
    /// Downstream distance specified by the downstream distance filter
    pub filterDownstreamDist: f64,
    /// Upstream distance specified by the upstream distance filter
    pub filterUpstreamDist: f64,
    /// Foe distance to junction specified by the turn filter
    pub filterFoeDistToJunction: f64,
    /// vTypes specified by the vTypes filter
    pub filterVTypes: BTreeSet<String>,
    /// vClasses specified by the vClasses filter,
    pub filterVClasses: SVCPermissions,
    /// Opening angle (in deg) specified by the field of vision filter
    pub filterFieldOfVisionOpeningAngle: f64,
    /// Lateral distance specified by the lateral distance filter
    pub filterLateralDist: f64,
}

impl Subscription {
    /// C++: `void Subscription::Subscription(int, const std::string &, const std::vector<int> &, const std::vector<std::shared_ptr<tcpip::Storage>> &, SUMOTime, SUMOTime, int, double)`
    /// Constructor
    // Subscription(int commandIdArg, const std::string& idArg,
    //                  const std::vector<int>& variablesArg,
    //                  const std::vector<std::shared_ptr<tcpip::Storage> >& paramsArg,
    //                  SUMOTime beginTimeArg, SUMOTime endTimeArg,
    //                  int contextDomainArg, double rangeArg)
    //         : commandId(commandIdArg),
    //           id(idArg),
    //           variables(variablesArg),
    //           parameters(paramsArg),
    //           beginTime(beginTimeArg),
    //           endTime(endTimeArg),
    //           contextDomain(contextDomainArg),
    //           range(rangeArg),
    //           activeFilters(SUBS_FILTER_NONE),
    //           filterLanes(),
    //           filterDownstreamDist(-1),
    //           filterUpstreamDist(-1),
    //           filterFoeDistToJunction(-1),
    //           filterVTypes(),
    //           filterVClasses(0),
    //           filterFieldOfVisionOpeningAngle(-1),
    //           filterLateralDist(-1) {}
    #[allow(unused_variables)]
    pub fn new(commandIdArg: i32, idArg: &str, variablesArg: &Vec<i32>, paramsArg: &Vec<Rc<Storage>>, beginTimeArg: SUMOTime, endTimeArg: SUMOTime, contextDomainArg: i32, rangeArg: f64) -> Self {
        Subscription {
            commandId: commandIdArg,
            id: idArg.to_string(),
            variables: variablesArg.clone(),
            parameters: paramsArg.clone(),
            beginTime: beginTimeArg,
            endTime: endTimeArg,
            contextDomain: contextDomainArg,
            range: rangeArg,
            activeFilters: SubscriptionFilterType::SUBS_FILTER_NONE as i32,
            filterLanes: Vec::new(),
            filterDownstreamDist: -1.,
            filterUpstreamDist: -1.,
            filterFoeDistToJunction: -1.,
            filterVTypes: BTreeSet::new(),
            filterVClasses: 0,
            filterFieldOfVisionOpeningAngle: -1.,
            filterLateralDist: -1.,
        }
    }

    /// C++: `bool Subscription::isVehicleToVehicleContextSubscription() const` [const]
    // bool isVehicleToVehicleContextSubscription() const {
    //         return commandId == CMD_SUBSCRIBE_VEHICLE_CONTEXT && contextDomain == CMD_GET_VEHICLE_VARIABLE;
    //     }
    #[allow(unused_variables)]
    pub fn is_vehicle_to_vehicle_context_subscription(&self) -> bool {
        self.commandId == CMD_SUBSCRIBE_VEHICLE_CONTEXT && self.contextDomain == CMD_GET_VEHICLE_VARIABLE
    }

    /// C++: `bool Subscription::isVehicleToPersonContextSubscription() const` [const]
    // bool isVehicleToPersonContextSubscription() const {
    //         return commandId == CMD_SUBSCRIBE_VEHICLE_CONTEXT && contextDomain == CMD_GET_PERSON_VARIABLE;
    //     }
    #[allow(unused_variables)]
    pub fn is_vehicle_to_person_context_subscription(&self) -> bool {
        self.commandId == CMD_SUBSCRIBE_VEHICLE_CONTEXT && self.contextDomain == CMD_GET_PERSON_VARIABLE
    }

}

/// C++ class `libsumo::VariableWrapper` (libsumo/Subscription.h)
pub struct VariableWrapper {
    pub handle: VariableWrapper_SubscriptionHandler,
}

impl VariableWrapper {
    /// C++: `void VariableWrapper::VariableWrapper(SubscriptionHandler)`
    // VariableWrapper(SubscriptionHandler handler = nullptr) : handle(handler) {}
    #[allow(unused_variables)]
    pub fn new(handler: VariableWrapper_SubscriptionHandler) -> Self {
        VariableWrapper { handle: handler }
    }

    /// C++: `void VariableWrapper::~VariableWrapper() noexcept` [virtual]
    // virtual ~VariableWrapper() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
    }

    /// C++: `void VariableWrapper::setContext(const std::string *const)` [virtual]
    // virtual void setContext(const std::string* const /* refID */) {}
    #[allow(unused_variables)]
    pub fn set_context(&mut self, arg0: Ptr<String>) {
        // C++ body is empty; the context reference is not used by this base class
    }

    /// C++: `void VariableWrapper::clear()` [virtual]
    // virtual void clear() {}
    #[allow(unused_variables)]
    pub fn clear(&mut self) {
        // C++ body is empty
    }

    /// C++: `bool VariableWrapper::wrapConnectionVector(const std::string &, const int, const std::vector<TraCIConnection> &)` [virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn wrap_connection_vector(&mut self, objID: &str, variable: i32, value: &Vec<TraCIConnection>) -> bool {
        panic!("pure virtual VariableWrapper::wrapConnectionVector")
    }

    /// C++: `bool VariableWrapper::wrapDouble(const std::string &, const int, const double)` [virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn wrap_double(&mut self, objID: &str, variable: i32, value: f64) -> bool {
        panic!("pure virtual VariableWrapper::wrapDouble")
    }

    /// C++: `bool VariableWrapper::wrapInt(const std::string &, const int, const int)` [virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn wrap_int(&mut self, objID: &str, variable: i32, value: i32) -> bool {
        panic!("pure virtual VariableWrapper::wrapInt")
    }

    /// C++: `bool VariableWrapper::wrapString(const std::string &, const int, const std::string &)` [virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn wrap_string(&mut self, objID: &str, variable: i32, value: &str) -> bool {
        panic!("pure virtual VariableWrapper::wrapString")
    }

    /// C++: `bool VariableWrapper::wrapStringList(const std::string &, const int, const std::vector<std::string> &)` [virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn wrap_string_list(&mut self, objID: &str, variable: i32, value: &Vec<String>) -> bool {
        panic!("pure virtual VariableWrapper::wrapStringList")
    }

    /// C++: `bool VariableWrapper::wrapDoubleList(const std::string &, const int, const std::vector<double> &)` [virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn wrap_double_list(&mut self, objID: &str, variable: i32, value: &Vec<f64>) -> bool {
        panic!("pure virtual VariableWrapper::wrapDoubleList")
    }

    /// C++: `bool VariableWrapper::wrapPosition(const std::string &, const int, const TraCIPosition &)` [virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn wrap_position(&mut self, objID: &str, variable: i32, value: &TraCIPosition) -> bool {
        panic!("pure virtual VariableWrapper::wrapPosition")
    }

    /// C++: `bool VariableWrapper::wrapPositionVector(const std::string &, const int, const TraCIPositionVector &)` [virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn wrap_position_vector(&mut self, objID: &str, variable: i32, value: &TraCIPositionVector) -> bool {
        panic!("pure virtual VariableWrapper::wrapPositionVector")
    }

    /// C++: `bool VariableWrapper::wrapColor(const std::string &, const int, const TraCIColor &)` [virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn wrap_color(&mut self, objID: &str, variable: i32, value: &TraCIColor) -> bool {
        panic!("pure virtual VariableWrapper::wrapColor")
    }

    /// C++: `bool VariableWrapper::wrapStringDoublePair(const std::string &, const int, const std::pair<std::string, double> &)` [virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn wrap_string_double_pair(&mut self, objID: &str, variable: i32, value: &(String, f64)) -> bool {
        panic!("pure virtual VariableWrapper::wrapStringDoublePair")
    }

    /// C++: `bool VariableWrapper::wrapStringDoublePairList(const std::string &, const int, const std::vector<std::pair<std::string, double>> &)` [virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn wrap_string_double_pair_list(&mut self, objID: &str, variable: i32, value: &Vec<(String, f64)>) -> bool {
        panic!("pure virtual VariableWrapper::wrapStringDoublePairList")
    }

    /// C++: `bool VariableWrapper::wrapStringPair(const std::string &, const int, const std::pair<std::string, std::string> &)` [virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn wrap_string_pair(&mut self, objID: &str, variable: i32, value: &(String, String)) -> bool {
        panic!("pure virtual VariableWrapper::wrapStringPair")
    }

    /// C++: `bool VariableWrapper::wrapIntPair(const std::string &, const int, const std::pair<int, int> &)` [virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn wrap_int_pair(&mut self, objID: &str, variable: i32, value: &(i32, i32)) -> bool {
        panic!("pure virtual VariableWrapper::wrapIntPair")
    }

    /// C++: `bool VariableWrapper::wrapStage(const std::string &, const int, const TraCIStage &)` [virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn wrap_stage(&mut self, objID: &str, variable: i32, value: &TraCIStage) -> bool {
        panic!("pure virtual VariableWrapper::wrapStage")
    }

    /// C++: `bool VariableWrapper::wrapReservationVector(const std::string &, const int, const std::vector<TraCIReservation> &)` [virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn wrap_reservation_vector(&mut self, objID: &str, variable: i32, value: &Vec<TraCIReservation>) -> bool {
        panic!("pure virtual VariableWrapper::wrapReservationVector")
    }

    /// C++: `bool VariableWrapper::wrapLogicVector(const std::string &, const int, const std::vector<TraCILogic> &)` [virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn wrap_logic_vector(&mut self, objID: &str, variable: i32, value: &Vec<TraCILogic>) -> bool {
        panic!("pure virtual VariableWrapper::wrapLogicVector")
    }

    /// C++: `bool VariableWrapper::wrapLinkVectorVector(const std::string &, const int, const std::vector<std::vector<TraCILink>> &)` [virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn wrap_link_vector_vector(&mut self, objID: &str, variable: i32, value: &Vec<Vec<TraCILink>>) -> bool {
        panic!("pure virtual VariableWrapper::wrapLinkVectorVector")
    }

    /// C++: `bool VariableWrapper::wrapSignalConstraintVector(const std::string &, const int, const std::vector<TraCISignalConstraint> &)` [virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn wrap_signal_constraint_vector(&mut self, objID: &str, variable: i32, value: &Vec<TraCISignalConstraint>) -> bool {
        panic!("pure virtual VariableWrapper::wrapSignalConstraintVector")
    }

    /// C++: `bool VariableWrapper::wrapJunctionFoeVector(const std::string &, const int, const std::vector<TraCIJunctionFoe> &)` [virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn wrap_junction_foe_vector(&mut self, objID: &str, variable: i32, value: &Vec<TraCIJunctionFoe>) -> bool {
        panic!("pure virtual VariableWrapper::wrapJunctionFoeVector")
    }

    /// C++: `bool VariableWrapper::wrapNextStopDataVector(const std::string &, const int, const std::vector<TraCINextStopData> &)` [virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn wrap_next_stop_data_vector(&mut self, objID: &str, variable: i32, value: &Vec<TraCINextStopData>) -> bool {
        panic!("pure virtual VariableWrapper::wrapNextStopDataVector")
    }

    /// C++: `bool VariableWrapper::wrapVehicleDataVector(const std::string &, const int, const std::vector<TraCIVehicleData> &)` [virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn wrap_vehicle_data_vector(&mut self, objID: &str, variable: i32, value: &Vec<TraCIVehicleData>) -> bool {
        panic!("pure virtual VariableWrapper::wrapVehicleDataVector")
    }

    /// C++: `bool VariableWrapper::wrapBestLanesDataVector(const std::string &, const int, const std::vector<TraCIBestLanesData> &)` [virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn wrap_best_lanes_data_vector(&mut self, objID: &str, variable: i32, value: &Vec<TraCIBestLanesData>) -> bool {
        panic!("pure virtual VariableWrapper::wrapBestLanesDataVector")
    }

    /// C++: `bool VariableWrapper::wrapNextTLSDataVector(const std::string &, const int, const std::vector<TraCINextTLSData> &)` [virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn wrap_next_tls_data_vector(&mut self, objID: &str, variable: i32, value: &Vec<TraCINextTLSData>) -> bool {
        panic!("pure virtual VariableWrapper::wrapNextTLSDataVector")
    }

    /// C++: `void VariableWrapper::empty(const std::string &)` [virtual]
    // virtual void empty(const std::string& /* objID */) {}
    #[allow(unused_variables)]
    pub fn empty(&mut self, arg0: &str) {
        // C++ body is empty
    }

}

