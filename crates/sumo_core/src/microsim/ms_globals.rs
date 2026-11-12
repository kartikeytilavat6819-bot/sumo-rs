//! Port of microsim/MSGlobals.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// This class holds some static variables, filled mostly with values coming from the command line or the simulation configuration file. They are stored herein to allow a faster access than from the options container.
/// C++ class `MSGlobals` (microsim/MSGlobals.h)
pub struct MSGlobals {
}

// C++ static members of MSGlobals (port as module-level statics, see AGENTS.md):
//   static bool MSGlobals::gOmitEmptyEdgesOnDump;
//   static bool MSGlobals::gSemiImplicitEulerUpdate;
//   static SUMOTime MSGlobals::gTimeToGridlock;
//   static SUMOTime MSGlobals::gTimeToGridlockHighways;
//   static double MSGlobals::gGridlockHighwaysSpeed;
//   static SUMOTime MSGlobals::gTimeToTeleportDisconnected;
//   static SUMOTime MSGlobals::gTimeToTeleportBidi;
//   static SUMOTime MSGlobals::gTimeToTeleportRSDeadlock;
//   static bool MSGlobals::gRemoveGridlocked;
//   static SUMOTime MSGlobals::gTimeToImpatience;
//   static bool MSGlobals::gUsingInternalLanes;
//   static SUMOTime MSGlobals::gIgnoreJunctionBlocker;
//   static bool MSGlobals::gCheck4Accidents;
//   static bool MSGlobals::gCheckRoutes;
//   static bool MSGlobals::gEmergencyInsert;
//   static SUMOTime MSGlobals::gLaneChangeDuration;
//   static double MSGlobals::gLateralResolution;
//   static bool MSGlobals::gStateLoaded;
//   static bool MSGlobals::gUseMesoSim;
//   static bool MSGlobals::gMesoLimitedJunctionControl;
//   static bool MSGlobals::gMesoInterpolatePos;
//   static MELoop * MSGlobals::gMesoNet;
//   static SUMOTime MSGlobals::gWaitingTimeMemory;
//   static SUMOTime MSGlobals::gActionStepLength;
//   static bool MSGlobals::gOverheadWireSolver;
//   static bool MSGlobals::gOverheadWireRecuperation;
//   static double MSGlobals::gDefaultEmergencyDecel;
//   static bool MSGlobals::gUseStopEnded;
//   static bool MSGlobals::gUseStopStarted;
//   static bool MSGlobals::gSlopeCentered;
//   static bool MSGlobals::gUnitTests;
//   static bool MSGlobals::gComputeLC;
//   static bool MSGlobals::gClearState;
//   static int MSGlobals::gNumSimThreads;
//   static int MSGlobals::gNumThreads;
//   static double MSGlobals::gEmergencyDecelWarningThreshold;
//   static double MSGlobals::gMinorPenalty;
//   static double MSGlobals::gTLSPenalty;
//   static double MSGlobals::gTurnaroundPenalty;
//   static bool MSGlobals::gModelParkingManoeuver;
//   static bool MSGlobals::gSublane;
//   static double MSGlobals::gStopTolerance;
//   static double MSGlobals::gTLSYellowMinDecel;
//   static bool MSGlobals::gLefthand;
//   static double MSGlobals::gWeightsSeparateTurns;
//   static SUMOTime MSGlobals::gStartupWaitThreshold;
//   static bool MSGlobals::gHaveEmissions;
//   static int MSGlobals::gInsertionChecks;
//   static double MSGlobals::gMaxRailSignalBlockLength;

impl MSGlobals {
}

// Module-level storage for the statics above. Atomics (not thread_local) because C++ statics are
// process-wide and the simulation may read them from several threads (gNumSimThreads).
// Each C++ static gets a getter and a setter; the C++ name is given in the doc line.
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicI64, AtomicPtr, AtomicU64, Ordering};
use crate::mesosim::me_loop::MELoop;

macro_rules! ms_global {
    ($static_name:ident, $get:ident, $set:ident, $atomic:ident, $ty:ty, $init:expr) => {
        static $static_name: $atomic = $atomic::new($init);
        pub fn $get() -> $ty {
            $static_name.load(Ordering::Relaxed)
        }
        pub fn $set(v: $ty) {
            $static_name.store(v, Ordering::Relaxed)
        }
    };
}

// f64 is stored as its bit pattern in an AtomicU64.
macro_rules! ms_global_f64 {
    ($static_name:ident, $get:ident, $set:ident, $init:expr) => {
        static $static_name: AtomicU64 = AtomicU64::new(($init as f64).to_bits());
        pub fn $get() -> f64 {
            f64::from_bits($static_name.load(Ordering::Relaxed))
        }
        pub fn $set(v: f64) {
            $static_name.store(v.to_bits(), Ordering::Relaxed)
        }
    };
}

// C++ MSGlobals::gOmitEmptyEdgesOnDump
ms_global!(G_OMIT_EMPTY_EDGES_ON_DUMP, omit_empty_edges_on_dump, set_omit_empty_edges_on_dump, AtomicBool, bool, false);
// C++ MSGlobals::gSemiImplicitEulerUpdate (default true, for unitTest)
ms_global!(G_SEMI_IMPLICIT_EULER_UPDATE, semi_implicit_euler_update, set_semi_implicit_euler_update, AtomicBool, bool, true);
// C++ MSGlobals::gTimeToGridlock
ms_global!(G_TIME_TO_GRIDLOCK, time_to_gridlock, set_time_to_gridlock, AtomicI64, SUMOTime, 0);
// C++ MSGlobals::gTimeToGridlockHighways
ms_global!(G_TIME_TO_GRIDLOCK_HIGHWAYS, time_to_gridlock_highways, set_time_to_gridlock_highways, AtomicI64, SUMOTime, 0);
// C++ MSGlobals::gGridlockHighwaysSpeed
ms_global_f64!(G_GRIDLOCK_HIGHWAYS_SPEED, gridlock_highways_speed, set_gridlock_highways_speed, 0.0);
// C++ MSGlobals::gTimeToTeleportDisconnected
ms_global!(G_TIME_TO_TELEPORT_DISCONNECTED, time_to_teleport_disconnected, set_time_to_teleport_disconnected, AtomicI64, SUMOTime, 0);
// C++ MSGlobals::gTimeToTeleportBidi
ms_global!(G_TIME_TO_TELEPORT_BIDI, time_to_teleport_bidi, set_time_to_teleport_bidi, AtomicI64, SUMOTime, 0);
// C++ MSGlobals::gTimeToTeleportRSDeadlock
ms_global!(G_TIME_TO_TELEPORT_RS_DEADLOCK, time_to_teleport_rs_deadlock, set_time_to_teleport_rs_deadlock, AtomicI64, SUMOTime, 0);
// C++ MSGlobals::gRemoveGridlocked
ms_global!(G_REMOVE_GRIDLOCKED, remove_gridlocked, set_remove_gridlocked, AtomicBool, bool, false);
// C++ MSGlobals::gTimeToImpatience
ms_global!(G_TIME_TO_IMPATIENCE, time_to_impatience, set_time_to_impatience, AtomicI64, SUMOTime, 0);
// C++ MSGlobals::gUsingInternalLanes
ms_global!(G_USING_INTERNAL_LANES, using_internal_lanes, set_using_internal_lanes, AtomicBool, bool, false);
// C++ MSGlobals::gIgnoreJunctionBlocker
ms_global!(G_IGNORE_JUNCTION_BLOCKER, ignore_junction_blocker, set_ignore_junction_blocker, AtomicI64, SUMOTime, 0);
// C++ MSGlobals::gCheck4Accidents
ms_global!(G_CHECK4_ACCIDENTS, check4_accidents, set_check4_accidents, AtomicBool, bool, false);
// C++ MSGlobals::gCheckRoutes
ms_global!(G_CHECK_ROUTES, check_routes, set_check_routes, AtomicBool, bool, false);
// C++ MSGlobals::gEmergencyInsert
ms_global!(G_EMERGENCY_INSERT, emergency_insert, set_emergency_insert, AtomicBool, bool, false);
// C++ MSGlobals::gLaneChangeDuration
ms_global!(G_LANE_CHANGE_DURATION, lane_change_duration, set_lane_change_duration, AtomicI64, SUMOTime, 0);
// C++ MSGlobals::gLateralResolution (default -1 for unitTest)
ms_global_f64!(G_LATERAL_RESOLUTION, lateral_resolution, set_lateral_resolution, -1.0);
// C++ MSGlobals::gStateLoaded
ms_global!(G_STATE_LOADED, state_loaded, set_state_loaded, AtomicBool, bool, false);
// C++ MSGlobals::gUseMesoSim
ms_global!(G_USE_MESO_SIM, use_meso_sim, set_use_meso_sim, AtomicBool, bool, false);
// C++ MSGlobals::gMesoLimitedJunctionControl
ms_global!(G_MESO_LIMITED_JUNCTION_CONTROL, meso_limited_junction_control, set_meso_limited_junction_control, AtomicBool, bool, false);
// C++ MSGlobals::gMesoInterpolatePos
ms_global!(G_MESO_INTERPOLATE_POS, meso_interpolate_pos, set_meso_interpolate_pos, AtomicBool, bool, false);

// C++ MSGlobals::gMesoNet (MELoop*, null by default); pointer stored atomically like the other statics.
static G_MESO_NET: AtomicPtr<MELoop> = AtomicPtr::new(std::ptr::null_mut());
pub fn meso_net() -> Ptr<MELoop> {
    Ptr::from_raw(G_MESO_NET.load(Ordering::Relaxed))
}
pub fn set_meso_net(v: Ptr<MELoop>) {
    G_MESO_NET.store(v.as_raw(), Ordering::Relaxed)
}

// C++ MSGlobals::gWaitingTimeMemory
ms_global!(G_WAITING_TIME_MEMORY, waiting_time_memory, set_waiting_time_memory, AtomicI64, SUMOTime, 0);
// C++ MSGlobals::gActionStepLength
ms_global!(G_ACTION_STEP_LENGTH, action_step_length, set_action_step_length, AtomicI64, SUMOTime, 0);
// C++ MSGlobals::gOverheadWireSolver
ms_global!(G_OVERHEAD_WIRE_SOLVER, overhead_wire_solver, set_overhead_wire_solver, AtomicBool, bool, false);
// C++ MSGlobals::gOverheadWireRecuperation
ms_global!(G_OVERHEAD_WIRE_RECUPERATION, overhead_wire_recuperation, set_overhead_wire_recuperation, AtomicBool, bool, false);
// C++ MSGlobals::gDefaultEmergencyDecel (default -1 for unitTest)
ms_global_f64!(G_DEFAULT_EMERGENCY_DECEL, default_emergency_decel, set_default_emergency_decel, -1.0);
// C++ MSGlobals::gUseStopEnded
ms_global!(G_USE_STOP_ENDED, use_stop_ended, set_use_stop_ended, AtomicBool, bool, false);
// C++ MSGlobals::gUseStopStarted
ms_global!(G_USE_STOP_STARTED, use_stop_started, set_use_stop_started, AtomicBool, bool, false);
// C++ MSGlobals::gSlopeCentered
ms_global!(G_SLOPE_CENTERED, slope_centered, set_slope_centered, AtomicBool, bool, false);
// C++ MSGlobals::gUnitTests
ms_global!(G_UNIT_TESTS, unit_tests, set_unit_tests, AtomicBool, bool, false);
// C++ MSGlobals::gComputeLC
ms_global!(G_COMPUTE_LC, compute_lc, set_compute_lc, AtomicBool, bool, false);
// C++ MSGlobals::gClearState
ms_global!(G_CLEAR_STATE, clear_state, set_clear_state, AtomicBool, bool, false);
// C++ MSGlobals::gNumSimThreads
ms_global!(G_NUM_SIM_THREADS, num_sim_threads, set_num_sim_threads, AtomicI32, i32, 0);
// C++ MSGlobals::gNumThreads
ms_global!(G_NUM_THREADS, num_threads, set_num_threads, AtomicI32, i32, 0);
// C++ MSGlobals::gEmergencyDecelWarningThreshold (default 1)
ms_global_f64!(G_EMERGENCY_DECEL_WARNING_THRESHOLD, emergency_decel_warning_threshold, set_emergency_decel_warning_threshold, 1.0);
// C++ MSGlobals::gMinorPenalty
ms_global_f64!(G_MINOR_PENALTY, minor_penalty, set_minor_penalty, 0.0);
// C++ MSGlobals::gTLSPenalty
ms_global_f64!(G_TLS_PENALTY, tls_penalty, set_tls_penalty, 0.0);
// C++ MSGlobals::gTurnaroundPenalty
ms_global_f64!(G_TURNAROUND_PENALTY, turnaround_penalty, set_turnaround_penalty, 0.0);
// C++ MSGlobals::gModelParkingManoeuver
ms_global!(G_MODEL_PARKING_MANOEUVER, model_parking_manoeuver, set_model_parking_manoeuver, AtomicBool, bool, false);
// C++ MSGlobals::gSublane
ms_global!(G_SUBLANE, sublane, set_sublane, AtomicBool, bool, false);
// C++ MSGlobals::gStopTolerance
ms_global_f64!(G_STOP_TOLERANCE, stop_tolerance, set_stop_tolerance, 0.0);
// C++ MSGlobals::gTLSYellowMinDecel
ms_global_f64!(G_TLS_YELLOW_MIN_DECEL, tls_yellow_min_decel, set_tls_yellow_min_decel, 0.0);
// C++ MSGlobals::gLefthand
ms_global!(G_LEFTHAND, lefthand, set_lefthand, AtomicBool, bool, false);
// C++ MSGlobals::gWeightsSeparateTurns
ms_global_f64!(G_WEIGHTS_SEPARATE_TURNS, weights_separate_turns, set_weights_separate_turns, 0.0);
// C++ MSGlobals::gStartupWaitThreshold
ms_global!(G_STARTUP_WAIT_THRESHOLD, startup_wait_threshold, set_startup_wait_threshold, AtomicI64, SUMOTime, 0);
// C++ MSGlobals::gHaveEmissions
ms_global!(G_HAVE_EMISSIONS, have_emissions, set_have_emissions, AtomicBool, bool, false);
// C++ MSGlobals::gInsertionChecks
ms_global!(G_INSERTION_CHECKS, insertion_checks, set_insertion_checks, AtomicI32, i32, 0);
// C++ MSGlobals::gMaxRailSignalBlockLength
ms_global_f64!(G_MAX_RAIL_SIGNAL_BLOCK_LENGTH, max_rail_signal_block_length, set_max_rail_signal_block_length, 2e4);

