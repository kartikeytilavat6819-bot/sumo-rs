//! Port of microsim/cfmodels/CC_VehicleVariables.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

// C++ CC_VehicleVariables::defaultL / defaultK / defaultB / defaultH (CC_VehicleVariables.cpp)
const DEFAULT_L: [[i32; 8]; 8] = [
    [0, 0, 0, 0, 0, 0, 0, 0],
    [1, 0, 0, 0, 0, 0, 0, 0],
    [1, 1, 0, 0, 0, 0, 0, 0],
    [1, 0, 1, 0, 0, 0, 0, 0],
    [1, 0, 0, 1, 0, 0, 0, 0],
    [1, 0, 0, 0, 1, 0, 0, 0],
    [1, 0, 0, 0, 0, 1, 0, 0],
    [1, 0, 0, 0, 0, 0, 1, 0],
];
const DEFAULT_K: [[f64; 8]; 8] = [
    [0., 0., 0., 0., 0., 0., 0., 0.],
    [460., 0., 0., 0., 0., 0., 0., 0.],
    [80., 860., 0., 0., 0., 0., 0., 0.],
    [80., 0., 860., 0., 0., 0., 0., 0.],
    [80., 0., 0., 860., 0., 0., 0., 0.],
    [80., 0., 0., 0., 860., 0., 0., 0.],
    [80., 0., 0., 0., 0., 860., 0., 0.],
    [80., 0., 0., 0., 0., 0., 860., 0.],
];
const DEFAULT_B: [f64; 8] = [1800., 1800., 1800., 1800., 1800., 1800., 1800., 1800.];
const DEFAULT_H: [f64; 8] = [0.8, 0.8, 0.8, 0.8, 0.8, 0.8, 0.8, 0.8];

/// C++ class `CC_VehicleVariables` (microsim/cfmodels/CC_VehicleVariables.h)
#[repr(C)]
pub struct CC_VehicleVariables {
    /// C++ base class `MSCFModel::VehicleVariables`
    pub base: MSCFModel_VehicleVariables,
    /// acceleration as computed by the controller, to be sent to other vehicles
    pub controllerAcceleration: f64,
    /// current front vehicle speed
    pub frontSpeed: f64,
    /// current front vehicle acceleration (used by CACC)
    pub frontAcceleration: f64,
    /// front vehicle controller acceleration (used by CACC)
    pub frontControllerAcceleration: f64,
    /// current front vehicle position
    pub frontPosition: Position,
    /// when front vehicle data has been readed from GPS
    pub frontDataReadTime: f64,
    /// front vehicle velocity vector
    pub frontVelocity: Position,
    /// front vehicle angle in radians
    pub frontAngle: f64,
    pub frontInitialized: bool,
    /// determines whether CACC should automatically fetch data about other vehicles
    pub autoFeed: bool,
    /// leader vehicle, used for auto feeding
    pub leaderVehicle: Ptr<MSVehicle>,
    /// sumo id of the leader vehicle
    pub leaderVehicleId: String,
    /// front sumo id, used for auto feeding
    pub frontVehicle: Ptr<MSVehicle>,
    /// sumo id of the front vehicle
    pub frontVehicleId: String,
    /// whether this vehicle is leader of a platoon or not. This is mainly used by the lane change logic. By default this is true as a single vehicle is treated as a 1-vehicle platoon
    pub isLeader: bool,
    /// headway time for ACC
    pub accHeadwayTime: f64,
    pub accLambda: f64,
    /// determines whether PATH's CACC should use the real vehicle acceleration or the controller computed one
    pub useControllerAcceleration: bool,
    /// platoon's leader speed (used by CACC)
    pub leaderSpeed: f64,
    /// platoon's leader acceleration (used by CACC)
    pub leaderAcceleration: f64,
    /// platoon's leader controller acceleration (used by CACC)
    pub leaderControllerAcceleration: f64,
    /// platoon's leader position
    pub leaderPosition: Position,
    /// when leader data has been readed from GPS
    pub leaderDataReadTime: f64,
    /// platoon's leader velocity vector
    pub leaderVelocity: Position,
    /// platoon's leader angle in radians
    pub leaderAngle: f64,
    pub leaderInitialized: bool,
    pub caccInitialized: bool,
    pub useFixedAcceleration: i32,
    pub fixedAcceleration: f64,
    pub crashed: bool,
    /// CC desired speed
    pub ccDesiredSpeed: f64,
    pub ccKp: f64,
    /// currently active controller (raw int: C++ casts any int to enum Plexe::ACTIVE_CONTROLLER)
    pub activeController: i32,
    /// fake controller data.
    pub fakeData: CC_VehicleVariables_FAKE_CONTROLLER_DATA,
    /// L matrix
    pub L: [[i32; 8]; 8],
    /// K matrix
    pub K: [[f64; 8]; 8],
    /// vector of damping ratios b
    pub b: [f64; 8],
    /// vector of time headways h
    pub h: [f64; 8],
    /// data about vehicles in the platoon
    pub vehicles: [VEHICLE_DATA; 8],
    /// tells whether data about a certain vehicle has been initialized
    pub initialized: [bool; 8],
    /// count of initialized vehicles
    pub nInitialized: i32,
    /// my position within the platoon (0 = first car)
    pub position: i32,
    /// number of cars in the platoon
    pub nCars: i32,
    /// controller related parameters
    pub caccXi: f64,
    pub caccOmegaN: f64,
    pub caccC1: f64,
    pub caccAlpha1: f64,
    pub caccAlpha2: f64,
    pub caccAlpha3: f64,
    pub caccAlpha4: f64,
    pub caccAlpha5: f64,
    /// fixed spacing for CACC
    pub caccSpacing: f64,
    pub engineTau: f64,
    /// limits for u
    pub uMin: f64,
    /// limits for u
    pub uMax: f64,
    pub ploegH: f64,
    pub ploegKp: f64,
    pub ploegKd: f64,
    pub flatbedKa: f64,
    pub flatbedKv: f64,
    pub flatbedKp: f64,
    pub flatbedD: f64,
    pub flatbedH: f64,
    /// engine model employed by this car
    pub engine: Ptr<GenericEngineModel>,
    /// numeric value indicating the employed model
    pub engineModel: i32,
    /// enable/disable data prediction (interpolation) for missing data
    pub usePrediction: bool,
    /// list of members belonging to my platoon
    pub members: BTreeMap<i32, String>,
    /// automatic whole platoon lane change
    pub autoLaneChange: bool,
    /// whole platoon lane change (not automatic). -1 indicates no need to change lane (mechanism disabled)
    pub platoonFixedLane: i32,
    /// when followers asks whether to actually change lane or not, what should the leader tell them?¬
    pub commitToLaneChange: bool,
    /// if a follower asks and we don't commit, what should be the blocked state to return?
    pub noCommitReason: i32,
    /// timestep for which the above commit is valid¬
    pub laneChangeCommitTime: SUMOTime,
}

impl std::ops::Deref for CC_VehicleVariables {
    type Target = MSCFModel_VehicleVariables;
    fn deref(&self) -> &MSCFModel_VehicleVariables {
        &self.base
    }
}
impl std::ops::DerefMut for CC_VehicleVariables {
    fn deref_mut(&mut self) -> &mut MSCFModel_VehicleVariables {
        &mut self.base
    }
}

// C++ static members of CC_VehicleVariables (port as module-level statics, see AGENTS.md):
//   static const int[8][8] CC_VehicleVariables::defaultL;
//   static const double[8][8] CC_VehicleVariables::defaultK;
//   static const double[] CC_VehicleVariables::defaultB;
//   static const double[] CC_VehicleVariables::defaultH;

impl CC_VehicleVariables {
    /// C++: `void CC_VehicleVariables::CC_VehicleVariables()`
    // CC_VehicleVariables::CC_VehicleVariables() :
    //     controllerAcceleration(0), frontSpeed(0), frontAcceleration(0),
    //     frontControllerAcceleration(0), frontDataReadTime(0), frontAngle(0), frontInitialized(false),
    //     autoFeed(false), leaderVehicle(nullptr), leaderVehicleId(""), frontVehicle(nullptr), frontVehicleId(""),
    //     isLeader(true),
    //     accHeadwayTime(1.5), accLambda(0.1),
    //     useControllerAcceleration(true), leaderSpeed(0),
    //     leaderAcceleration(0), leaderControllerAcceleration(0), leaderDataReadTime(0), leaderAngle(0),
    //     leaderInitialized(false), caccInitialized(false),
    //     useFixedAcceleration(0), fixedAcceleration(0),
    //     crashed(false),
    //     ccDesiredSpeed(14), ccKp(1), activeController(Plexe::DRIVER),
    //     nInitialized(0), position(-1), nCars(8),
    //     caccXi(-1), caccOmegaN(-1), caccC1(-1), caccAlpha1(-1), caccAlpha2(-1),
    //     caccAlpha3(-1), caccAlpha4(-1), caccAlpha5(-1), caccSpacing(5),
    //     engineTau(0.5),
    //     uMin(-1e6), uMax(1e6),
    //     ploegH(0.5), ploegKp(0.2), ploegKd(0.7),
    //     flatbedKa(2.4), flatbedKv(0.6), flatbedKp(12), flatbedD(5), flatbedH(4),
    //     engine(0), engineModel(CC_ENGINE_MODEL_FOLM),
    //     usePrediction(false),
    //     autoLaneChange(false),
    //     platoonFixedLane(-1),
    //     commitToLaneChange(true), noCommitReason(0), laneChangeCommitTime(-1) {
    //     fakeData.frontAcceleration = 0;
    //     fakeData.frontControllerAcceleration = 0;
    //     fakeData.frontDistance = 0;
    //     fakeData.frontSpeed = 0;
    //     fakeData.leaderAcceleration = 0;
    //     fakeData.leaderControllerAcceleration = 0;
    //     fakeData.leaderSpeed = 0;
    //     leaderPosition.set(0, 0);
    //     frontPosition.set(0, 0);
    //     //init L, K, b, and h with default values
    //     memcpy(L, defaultL, sizeof(int)*MAX_N_CARS * MAX_N_CARS);
    //     memcpy(K, defaultK, sizeof(double)*MAX_N_CARS * MAX_N_CARS);
    //     memcpy(b, defaultB, sizeof(double)*MAX_N_CARS);
    //     memcpy(h, defaultH, sizeof(double)*MAX_N_CARS);
    //     //no data about any vehicle has been set
    //     for (int i = 0; i < MAX_N_CARS; i++) {
    //         initialized[i] = false;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn new() -> Self {
        // C++ member-init order and defaults; L, K, b, h are memcpy'd from the default tables above
        Self {
            base: MSCFModel_VehicleVariables {},
            controllerAcceleration: 0., frontSpeed: 0., frontAcceleration: 0.,
            frontControllerAcceleration: 0., frontPosition: Position::new(0., 0.), frontDataReadTime: 0., frontVelocity: Position::new(0., 0.),
            frontAngle: 0., frontInitialized: false,
            autoFeed: false, leaderVehicle: Ptr::null(), leaderVehicleId: String::new(), frontVehicle: Ptr::null(), frontVehicleId: String::new(),
            isLeader: true,
            accHeadwayTime: 1.5, accLambda: 0.1,
            useControllerAcceleration: true, leaderSpeed: 0.,
            leaderAcceleration: 0., leaderControllerAcceleration: 0., leaderPosition: Position::new(0., 0.), leaderDataReadTime: 0., leaderVelocity: Position::new(0., 0.), leaderAngle: 0.,
            leaderInitialized: false, caccInitialized: false,
            useFixedAcceleration: 0, fixedAcceleration: 0.,
            crashed: false,
            ccDesiredSpeed: 14., ccKp: 1., activeController: ACTIVE_CONTROLLER::DRIVER as i32,
            fakeData: CC_VehicleVariables_FAKE_CONTROLLER_DATA {
                frontDistance: 0., frontSpeed: 0., frontAcceleration: 0., frontControllerAcceleration: 0.,
                leaderSpeed: 0., leaderAcceleration: 0., leaderControllerAcceleration: 0.,
            },
            L: DEFAULT_L,
            K: DEFAULT_K,
            b: DEFAULT_B,
            h: DEFAULT_H,
            // zero-filled: the C++ VEHICLE_DATA array has no initializer
            vehicles: std::array::from_fn(|_| VEHICLE_DATA {
                index: 0, speed: 0., acceleration: 0., positionX: 0., positionY: 0., time: 0., length: 0., u: 0., speedX: 0., speedY: 0., angle: 0.,
            }),
            initialized: [false; 8],
            nInitialized: 0, position: -1, nCars: 8,
            caccXi: -1., caccOmegaN: -1., caccC1: -1., caccAlpha1: -1., caccAlpha2: -1.,
            caccAlpha3: -1., caccAlpha4: -1., caccAlpha5: -1., caccSpacing: 5.,
            engineTau: 0.5,
            uMin: -1e6, uMax: 1e6,
            ploegH: 0.5, ploegKp: 0.2, ploegKd: 0.7,
            flatbedKa: 2.4, flatbedKv: 0.6, flatbedKp: 12., flatbedD: 5., flatbedH: 4.,
            engine: Ptr::null(), engineModel: 0, // 0 = CC_ENGINE_MODEL_FOLM
            usePrediction: false,
            members: BTreeMap::new(),
            autoLaneChange: false,
            platoonFixedLane: -1,
            commitToLaneChange: true, noCommitReason: 0, laneChangeCommitTime: -1,
        }
    }

    /// C++: `void CC_VehicleVariables::~CC_VehicleVariables() noexcept` [virtual]
    // CC_VehicleVariables::~CC_VehicleVariables() {
    //     if (engine) {
    //         delete engine;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        if !self.engine.is_null() {
            // C++ `delete engine`
            drop(unsafe { Box::from_raw(self.engine.as_raw()) });
        }
    }

}

/// represent the set of fake data which is sent to the controller in order to automatically make the car move to a precise position before joining the platoon. we expect to get from the upper application the data that the CACC needs, i.e.: - front distance, front speed and front vehicle acceleration: this information regards the car that the vehicle joining the platoon will have directly in front. this data might be real or might be fake: for example, if the platoon management algorithm decides to set the vehicle as the new leader, there won't be a car in front, and the fake data will be used only for positioning. in the case of fake data, acceleration must be set to 0 - leader front speed and acceleration: this information is the same as previously described for vehicle in front, but regards the leader. again, if the vehicle is being set as the new leader, this data might be fake data
/// C++ class `CC_VehicleVariables::FAKE_CONTROLLER_DATA` (microsim/cfmodels/CC_VehicleVariables.h)
pub struct CC_VehicleVariables_FAKE_CONTROLLER_DATA {
    pub frontDistance: f64,
    pub frontSpeed: f64,
    pub frontAcceleration: f64,
    pub frontControllerAcceleration: f64,
    pub leaderSpeed: f64,
    pub leaderAcceleration: f64,
    pub leaderControllerAcceleration: f64,
}

impl CC_VehicleVariables_FAKE_CONTROLLER_DATA {
}

