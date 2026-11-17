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
pub type VariableWrapper_SubscriptionHandler = Option<fn(&str, i32, Ptr<VariableWrapper>, Ptr<Storage>) -> bool>;

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

