//! Port of libsumo/VariableWrapper.{h,cpp} (the class is declared in libsumo/Subscription.h)
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use crate::traci_server::tra_ci_server::TraCIServer;

// shortcut: C++ VariableWrapper is abstract and its derived objects are the TraCI server and Helper::SubscriptionWrapper;
// the sink is a raw pointer to the derived object, which its owner must keep alive and set with set_sink().

/// Macro list of the C++ `VariableWrapper::wrap*` virtuals: (name, extra params, C++ name). Used to define the
/// trait, the dispatching methods on VariableWrapper and the TraCIServer implementation from one list.
macro_rules! for_each_wrap {
    ($cb:ident) => {
        $cb! {
            wrap_connection_vector(value: &[TraCIConnection]) => "wrapConnectionVector";
            wrap_double(value: f64) => "wrapDouble";
            wrap_int(value: i32) => "wrapInt";
            wrap_string(value: &str) => "wrapString";
            wrap_string_list(value: &[String]) => "wrapStringList";
            wrap_double_list(value: &[f64]) => "wrapDoubleList";
            wrap_position(value: &TraCIPosition) => "wrapPosition";
            wrap_position_vector(value: &TraCIPositionVector) => "wrapPositionVector";
            wrap_color(value: &TraCIColor) => "wrapColor";
            wrap_string_double_pair(value: &(String, f64)) => "wrapStringDoublePair";
            wrap_string_double_pair_list(value: &[(String, f64)]) => "wrapStringDoublePairList";
            wrap_string_pair(value: &(String, String)) => "wrapStringPair";
            wrap_int_pair(value: &(i32, i32)) => "wrapIntPair";
            wrap_stage(value: &TraCIStage) => "wrapStage";
            wrap_reservation_vector(value: &[TraCIReservation]) => "wrapReservationVector";
            wrap_logic_vector(value: &[TraCILogic]) => "wrapLogicVector";
            wrap_link_vector_vector(value: &[Vec<TraCILink>]) => "wrapLinkVectorVector";
            wrap_signal_constraint_vector(value: &[TraCISignalConstraint]) => "wrapSignalConstraintVector";
            wrap_junction_foe_vector(value: &[TraCIJunctionFoe]) => "wrapJunctionFoeVector";
            wrap_next_stop_data_vector(value: &[TraCINextStopData]) => "wrapNextStopDataVector";
            wrap_vehicle_data_vector(value: &[TraCIVehicleData]) => "wrapVehicleDataVector";
            wrap_best_lanes_data_vector(value: &[TraCIBestLanesData]) => "wrapBestLanesDataVector";
            wrap_next_tls_data_vector(value: &[TraCINextTLSData]) => "wrapNextTLSDataVector";
        }
    };
}

macro_rules! define_wrapper_api {
    ($( $name:ident ( $( $arg:ident : $ty:ty ),* ) => $cpp:literal ; )*) => {
        /// The virtual `wrap*` interface of C++ `VariableWrapper`. The default bodies are the C++ pure virtuals.
        pub trait VariableWrapperSink {
            // C++ virtuals with empty default bodies
            fn set_context(&mut self, refID: Ptr<String>) {}
            fn clear(&mut self) {}
            fn empty(&mut self, objID: &str) {}
            $(
                fn $name(&mut self, objID: &str, variable: i32, $($arg: $ty),*) -> bool {
                    panic!("pure virtual VariableWrapper::{}", $cpp)
                }
            )*
        }

        impl VariableWrapper {
            $(
                /// Forwards to the attached derived object (C++ virtual call); panics like a pure virtual call if none is attached.
                pub fn $name(&mut self, objID: &str, variable: i32, $($arg: $ty),*) -> bool {
                    self.pure_or_sink($cpp).$name(objID, variable, $($arg),*)
                }
            )*
        }
    };
}

macro_rules! impl_server_wrapper {
    ($( $name:ident ( $( $arg:ident : $ty:ty ),* ) => $cpp:literal ; )*) => {
        /// C++ `TraCIServer` overrides of `VariableWrapper::wrap*`; the bodies live in the server.
        impl VariableWrapperSink for TraCIServer {
            $(
                fn $name(&mut self, objID: &str, variable: i32, $($arg: $ty),*) -> bool {
                    TraCIServer::$name(self, objID, variable, $($arg),*)
                }
            )*
        }
    };
}

for_each_wrap!(define_wrapper_api);
for_each_wrap!(impl_server_wrapper);

/// C++ class `libsumo::VariableWrapper` (libsumo/Subscription.h)
pub struct VariableWrapper {
    pub handle: VariableWrapper_SubscriptionHandler,
    /// The derived object (C++ `this` of the subclass) that implements the virtual wrap* calls
    pub sink: Option<*mut dyn VariableWrapperSink>,
}

impl VariableWrapper {
    /// C++: `void VariableWrapper::VariableWrapper(SubscriptionHandler)`
    // VariableWrapper(SubscriptionHandler handler = nullptr) : handle(handler) {}
    #[allow(unused_variables)]
    pub fn new(handler: VariableWrapper_SubscriptionHandler) -> Self {
        VariableWrapper { handle: handler, sink: None }
    }

    /// Attach the derived object whose wrap* methods this wrapper dispatches to.
    pub fn set_sink(&mut self, sink: *mut dyn VariableWrapperSink) {
        self.sink = Some(sink);
    }

    fn pure_or_sink(&mut self, cpp: &str) -> &mut dyn VariableWrapperSink {
        match self.sink {
            // SAFETY: the owner attaches the sink with set_sink and keeps it alive while the wrapper is used
            Some(p) => unsafe { &mut *p },
            None => panic!("pure virtual VariableWrapper::{}", cpp),
        }
    }

    /// C++: `void VariableWrapper::~VariableWrapper() noexcept` [virtual]
    // virtual ~VariableWrapper() {}
    pub fn drop_(&mut self) {}

    /// C++: `void VariableWrapper::setContext(const std::string *const)` [virtual]
    // virtual void setContext(const std::string* const /* refID */) {}
    pub fn set_context(&mut self, refID: Ptr<String>) {
        if let Some(p) = self.sink {
            unsafe { (*p).set_context(refID) }
        }
    }

    /// C++: `void VariableWrapper::clear()` [virtual]
    // virtual void clear() {}
    pub fn clear(&mut self) {
        if let Some(p) = self.sink {
            unsafe { (*p).clear() }
        }
    }

    /// C++: `void VariableWrapper::empty(const std::string &)` [virtual]
    // virtual void empty(const std::string& /* objID */) {}
    pub fn empty(&mut self, objID: &str) {
        if let Some(p) = self.sink {
            unsafe { (*p).empty(objID) }
        }
    }
}
