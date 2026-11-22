//! Port of libsumo/TraCIDefs.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Every method is a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

// C++ `os << double` prints like %g with precision 6; `os << bool` prints 1/0 (no boolalpha)
fn cpp_dbl(v: f64) -> String {
    utils::common::to_string::f64_to_string_default(v)
}
fn cpp_bool(b: bool) -> i32 {
    b as i32
}

/// C++ typedef `std::map<std::string, libsumo::SubscriptionResults> ContextSubscriptionResults`
pub type ContextSubscriptionResults = BTreeMap<String, SubscriptionResults>;

/// C++ typedef `std::map<std::string, libsumo::TraCIResults> SubscriptionResults`
pub type SubscriptionResults = BTreeMap<String, TraCIResults>;

/// C++ typedef `std::map<int, std::shared_ptr<libsumo::TraCIResult>> TraCIResults`
pub type TraCIResults = BTreeMap<i32, Rc<TraCIResult>>;

/// An error which is not recoverable
/// C++ class `libsumo::FatalTraCIError` (libsumo/TraCIDefs.h)
#[repr(C)]
pub struct FatalTraCIError {
    /// C++ base class `std::runtime_error`
    pub base: String /* std::runtime_error */,
}

impl std::ops::Deref for FatalTraCIError {
    type Target = String /* std::runtime_error */;
    fn deref(&self) -> &String /* std::runtime_error */ {
        &self.base
    }
}
impl std::ops::DerefMut for FatalTraCIError {
    fn deref_mut(&mut self) -> &mut String /* std::runtime_error */ {
        &mut self.base
    }
}

impl FatalTraCIError {
    /// C++: `void FatalTraCIError::FatalTraCIError(std::string)`
    /// constructor
    // FatalTraCIError(std::string what)
    //         : std::runtime_error(what) {}
    #[allow(unused_variables)]
    pub fn new(what: String) -> Self {
        FatalTraCIError { base: what }
    }

}

/// C++ class `libsumo::TraCIBestLanesData` (libsumo/TraCIDefs.h)
#[derive(Clone)]
pub struct TraCIBestLanesData {
    /// The id of the lane
    pub laneID: String,
    /// The length than can be driven from that lane without lane change
    pub length: f64,
    /// The traffic density along length
    pub occupation: f64,
    /// The offset of this lane from the best lane
    pub bestLaneOffset: i32,
    /// Whether this lane allows continuing the route
    pub allowsContinuation: bool,
    /// The sequence of lanes that best allows continuing the route without lane change
    pub continuationLanes: Vec<String>,
}

impl TraCIBestLanesData {
    /// C++: `std::string TraCIBestLanesData::getString() const` [const]
    // std::string getString() const {
    //         std::ostringstream os;
    //         os << "TraCIBestLanesData(" << laneID << "," << length << "," << occupation
    //            << "," << bestLaneOffset << "," << allowsContinuation << ",[";
    //         for (const std::string& s : continuationLanes) {
    //             os << s << ",";
    //         }
    //         os << "])";
    //         return os.str();
    //     }
    #[allow(unused_variables)]
    pub fn get_string(&self) -> String {
        let mut os = format!("TraCIBestLanesData({},{},{},{},{},[", self.laneID, cpp_dbl(self.length), cpp_dbl(self.occupation), self.bestLaneOffset, cpp_bool(self.allowsContinuation));
        for s in &self.continuationLanes {
            os += &format!("{},", s);
        }
        os += "])";
        os
    }

}

/// C++ class `libsumo::TraCIBestLanesDataVectorWrapped` (libsumo/TraCIDefs.h)
#[derive(Clone)]
#[repr(C)]
pub struct TraCIBestLanesDataVectorWrapped {
    /// C++ base class `TraCIResult`
    pub base: TraCIResult,
    pub value: Vec<TraCIBestLanesData>,
}

impl std::ops::Deref for TraCIBestLanesDataVectorWrapped {
    type Target = TraCIResult;
    fn deref(&self) -> &TraCIResult {
        &self.base
    }
}
impl std::ops::DerefMut for TraCIBestLanesDataVectorWrapped {
    fn deref_mut(&mut self) -> &mut TraCIResult {
        &mut self.base
    }
}

impl TraCIBestLanesDataVectorWrapped {
    /// C++: `std::string TraCIBestLanesDataVectorWrapped::getString() const` [const, virtual]
    // std::string getString() const override {
    //         std::ostringstream os;
    //         os << "TraCIBestLanesDataVectorWrapped[";
    //         for (const TraCIBestLanesData& v : value) {
    //             os << v.getString() << ",";
    //         }
    //         os << "]";
    //         return os.str();
    //     }
    #[allow(unused_variables)]
    pub fn get_string(&self) -> String {
        let mut os = String::from("TraCIBestLanesDataVectorWrapped[");
        for v in &self.value {
            os += &v.get_string();
            os += ",";
        }
        os += "]";
        os
    }

}

/// C++ class `libsumo::TraCICollision` (libsumo/TraCIDefs.h)
#[derive(Clone)]
pub struct TraCICollision {
    /// The ids of the participating vehicles and persons
    pub collider: String,
    pub victim: String,
    pub colliderType: String,
    pub victimType: String,
    pub colliderSpeed: f64,
    pub victimSpeed: f64,
    /// The type of collision
    pub type_: String,
    /// The lane where the collision happended
    pub lane: String,
    /// The position of the collision along the lane
    pub pos: f64,
}

impl TraCICollision {
    /// C++: `std::string TraCICollision::getString() const` [const]
    // std::string getString() const {
    //         std::ostringstream os;
    //         os << "TraCICollision(collider=" << collider << ", victim=" << victim << ")";
    //         return os.str();
    //     }
    #[allow(unused_variables)]
    pub fn get_string(&self) -> String {
        format!("TraCICollision(collider={}, victim={})", self.collider, self.victim)
    }

}

/// C++ class `libsumo::TraCICollisionVectorWrapped` (libsumo/TraCIDefs.h)
#[derive(Clone)]
#[repr(C)]
pub struct TraCICollisionVectorWrapped {
    /// C++ base class `TraCIResult`
    pub base: TraCIResult,
    pub value: Vec<TraCICollision>,
}

impl std::ops::Deref for TraCICollisionVectorWrapped {
    type Target = TraCIResult;
    fn deref(&self) -> &TraCIResult {
        &self.base
    }
}
impl std::ops::DerefMut for TraCICollisionVectorWrapped {
    fn deref_mut(&mut self) -> &mut TraCIResult {
        &mut self.base
    }
}

impl TraCICollisionVectorWrapped {
    /// C++: `std::string TraCICollisionVectorWrapped::getString() const` [const, virtual]
    // std::string getString() const override {
    //         std::ostringstream os;
    //         os << "TraCICollisionVectorWrapped[";
    //         for (const TraCICollision& v : value) {
    //             os << v.getString() << ",";
    //         }
    //         os << "]";
    //         return os.str();
    //     }
    #[allow(unused_variables)]
    pub fn get_string(&self) -> String {
        let mut os = String::from("TraCICollisionVectorWrapped[");
        for v in &self.value {
            os += &v.get_string();
            os += ",";
        }
        os += "]";
        os
    }

}

/// A color
/// C++ class `libsumo::TraCIColor` (libsumo/TraCIDefs.h)
#[derive(Clone)]
#[repr(C)]
pub struct TraCIColor {
    /// C++ base class `TraCIResult`
    pub base: TraCIResult,
    pub r: i32,
    pub g: i32,
    pub b: i32,
    pub a: i32,
}

impl std::ops::Deref for TraCIColor {
    type Target = TraCIResult;
    fn deref(&self) -> &TraCIResult {
        &self.base
    }
}
impl std::ops::DerefMut for TraCIColor {
    fn deref_mut(&mut self) -> &mut TraCIResult {
        &mut self.base
    }
}

impl TraCIColor {
    /// C++: `void TraCIColor::TraCIColor()`
    // TraCIColor() : r(0), g(0), b(0), a(255) {}
    #[allow(unused_variables)]
    pub fn new() -> Self {
        TraCIColor { base: TraCIResult::new(), r: 0, g: 0, b: 0, a: 255 }
    }

    /// C++: `void TraCIColor::TraCIColor(int, int, int, int)`
    // TraCIColor(int r, int g, int b, int a = 255) : r(r), g(g), b(b), a(a) {}
    #[allow(unused_variables)]
    pub fn new_2(r: i32, g: i32, b: i32, a: i32) -> Self {
        TraCIColor { base: TraCIResult::new(), r, g, b, a }
    }

    /// C++: `std::string TraCIColor::getString() const` [const, virtual]
    // std::string getString() const override {
    //         std::ostringstream os;
    //         os << "TraCIColor(" << r << "," << g << "," << b << "," << a << ")";
    //         return os.str();
    //     }
    #[allow(unused_variables)]
    pub fn get_string(&self) -> String {
        format!("TraCIColor({},{},{},{})", self.r, self.g, self.b, self.a)
    }

}

/// C++ class `libsumo::TraCIConnection` (libsumo/TraCIDefs.h)
#[derive(Clone)]
pub struct TraCIConnection {
    pub approachedLane: String,
    pub hasPrio: bool,
    pub isOpen: bool,
    pub hasFoe: bool,
    pub approachedInternal: String,
    pub state: String,
    pub direction: String,
    pub length: f64,
}

impl TraCIConnection {
    /// C++: `void TraCIConnection::TraCIConnection()`
    // TraCIConnection() {}
    #[allow(unused_variables)]
    pub fn new() -> Self {
        TraCIConnection { approachedLane: String::new(), hasPrio: false, isOpen: false, hasFoe: false, approachedInternal: String::new(), state: String::new(), direction: String::new(), length: 0.0 }
    }

    /// C++: `void TraCIConnection::TraCIConnection(const std::string &, const bool, const bool, const bool, const std::string, const std::string, const std::string, const double)`
    // TraCIConnection(const std::string& _approachedLane, const bool _hasPrio, const bool _isOpen, const bool _hasFoe,
    //                     const std::string _approachedInternal, const std::string _state, const std::string _direction, const double _length)
    //         : approachedLane(_approachedLane), hasPrio(_hasPrio), isOpen(_isOpen), hasFoe(_hasFoe),
    //           approachedInternal(_approachedInternal), state(_state), direction(_direction), length(_length) {}
    #[allow(unused_variables)]
    pub fn new_2(_approachedLane: &str, _hasPrio: bool, _isOpen: bool, _hasFoe: bool, _approachedInternal: String, _state: String, _direction: String, _length: f64) -> Self {
        TraCIConnection { approachedLane: _approachedLane.to_string(), hasPrio: _hasPrio, isOpen: _isOpen, hasFoe: _hasFoe, approachedInternal: _approachedInternal, state: _state, direction: _direction, length: _length }
    }

    /// C++: `std::string TraCIConnection::getString() const` [const]
    // std::string getString() const {
    //         std::ostringstream os;
    //         os << "TraCIConnection(" << approachedLane << "," << hasPrio << "," << isOpen
    //            << "," << hasFoe << "," << approachedInternal << "," << state << "," << direction << "," << length << ")";
    //         return os.str();
    //     }
    #[allow(unused_variables)]
    pub fn get_string(&self) -> String {
        format!("TraCIConnection({},{},{},{},{},{},{},{})", self.approachedLane, cpp_bool(self.hasPrio), cpp_bool(self.isOpen), cpp_bool(self.hasFoe), self.approachedInternal, self.state, self.direction, cpp_dbl(self.length))
    }

}

/// C++ class `libsumo::TraCIConnectionVectorWrapped` (libsumo/TraCIDefs.h)
#[derive(Clone)]
#[repr(C)]
pub struct TraCIConnectionVectorWrapped {
    /// C++ base class `TraCIResult`
    pub base: TraCIResult,
    pub value: Vec<TraCIConnection>,
}

impl std::ops::Deref for TraCIConnectionVectorWrapped {
    type Target = TraCIResult;
    fn deref(&self) -> &TraCIResult {
        &self.base
    }
}
impl std::ops::DerefMut for TraCIConnectionVectorWrapped {
    fn deref_mut(&mut self) -> &mut TraCIResult {
        &mut self.base
    }
}

impl TraCIConnectionVectorWrapped {
    /// C++: `std::string TraCIConnectionVectorWrapped::getString() const` [const, virtual]
    // std::string getString() const override {
    //         std::ostringstream os;
    //         os << "TraCIConnectionVectorWrapped[";
    //         for (const TraCIConnection& v : value) {
    //             os << v.getString() << ",";
    //         }
    //         os << "]";
    //         return os.str();
    //     }
    #[allow(unused_variables)]
    pub fn get_string(&self) -> String {
        let mut os = String::from("TraCIConnectionVectorWrapped[");
        for v in &self.value {
            os += &v.get_string();
            os += ",";
        }
        os += "]";
        os
    }

}

/// C++ class `libsumo::TraCIDouble` (libsumo/TraCIDefs.h)
#[derive(Clone)]
#[repr(C)]
pub struct TraCIDouble {
    /// C++ base class `TraCIResult`
    pub base: TraCIResult,
    pub value: f64,
}

impl std::ops::Deref for TraCIDouble {
    type Target = TraCIResult;
    fn deref(&self) -> &TraCIResult {
        &self.base
    }
}
impl std::ops::DerefMut for TraCIDouble {
    fn deref_mut(&mut self) -> &mut TraCIResult {
        &mut self.base
    }
}

impl TraCIDouble {
    /// C++: `void TraCIDouble::TraCIDouble(double)`
    // TraCIDouble(double v = 0.) : value(v) {}
    #[allow(unused_variables)]
    pub fn new(v: f64) -> Self {
        TraCIDouble { base: TraCIResult { result_type: TYPE_DOUBLE }, value: v }
    }

    /// C++: `std::string TraCIDouble::getString() const` [const, virtual]
    // std::string getString() const override {
    //         std::ostringstream os;
    //         os << value;
    //         return os.str();
    //     }
    #[allow(unused_variables)]
    pub fn get_string(&self) -> String {
        cpp_dbl(self.value)
    }

    /// C++: `int TraCIDouble::getType() const` [const, virtual]
    // int getType() const override {
    //         return libsumo::TYPE_DOUBLE;
    //     }
    #[allow(unused_variables)]
    pub fn get_type(&self) -> i32 {
        TYPE_DOUBLE
    }

}

/// C++ class `libsumo::TraCIDoubleList` (libsumo/TraCIDefs.h)
#[derive(Clone)]
#[repr(C)]
pub struct TraCIDoubleList {
    /// C++ base class `TraCIResult`
    pub base: TraCIResult,
    pub value: Vec<f64>,
}

impl std::ops::Deref for TraCIDoubleList {
    type Target = TraCIResult;
    fn deref(&self) -> &TraCIResult {
        &self.base
    }
}
impl std::ops::DerefMut for TraCIDoubleList {
    fn deref_mut(&mut self) -> &mut TraCIResult {
        &mut self.base
    }
}

impl TraCIDoubleList {
    /// C++: `std::string TraCIDoubleList::getString() const` [const, virtual]
    // std::string getString() const override {
    //         std::ostringstream os;
    //         os << "[";
    //         for (double v : value) {
    //             os << v << ",";
    //         }
    //         os << "]";
    //         return os.str();
    //     }
    #[allow(unused_variables)]
    pub fn get_string(&self) -> String {
        let mut os = String::from("[");
        for v in &self.value {
            os += &cpp_dbl(*v);
            os += ",";
        }
        os += "]";
        os
    }

}

/// An error which allows to continue
/// C++ class `libsumo::TraCIException` (libsumo/TraCIDefs.h)
#[derive(Clone)]
#[repr(C)]
pub struct TraCIException {
    /// C++ base class `std::runtime_error`
    pub base: String /* std::runtime_error */,
}

impl std::ops::Deref for TraCIException {
    type Target = String /* std::runtime_error */;
    fn deref(&self) -> &String /* std::runtime_error */ {
        &self.base
    }
}
impl std::ops::DerefMut for TraCIException {
    fn deref_mut(&mut self) -> &mut String /* std::runtime_error */ {
        &mut self.base
    }
}

impl TraCIException {
    /// C++: `void TraCIException::TraCIException(std::string)`
    /// constructor
    // TraCIException(std::string what)
    //         : std::runtime_error(what) {}
    #[allow(unused_variables)]
    pub fn new(what: String) -> Self {
        TraCIException { base: what }
    }

}

/// C++ class `libsumo::TraCIInt` (libsumo/TraCIDefs.h)
#[derive(Clone)]
#[repr(C)]
pub struct TraCIInt {
    /// C++ base class `TraCIResult`
    pub base: TraCIResult,
    pub value: i32,
    pub traciType: i32,
}

impl std::ops::Deref for TraCIInt {
    type Target = TraCIResult;
    fn deref(&self) -> &TraCIResult {
        &self.base
    }
}
impl std::ops::DerefMut for TraCIInt {
    fn deref_mut(&mut self) -> &mut TraCIResult {
        &mut self.base
    }
}

impl TraCIInt {
    /// C++: `void TraCIInt::TraCIInt(int, int)`
    // TraCIInt(int v = 0, int t = libsumo::TYPE_INTEGER) : value(v), traciType(t) {}
    #[allow(unused_variables)]
    pub fn new(v: i32, t: i32) -> Self {
        TraCIInt { base: TraCIResult { result_type: t }, value: v, traciType: t }
    }

    /// C++: `std::string TraCIInt::getString() const` [const, virtual]
    // std::string getString() const override {
    //         std::ostringstream os;
    //         os << value;
    //         return os.str();
    //     }
    #[allow(unused_variables)]
    pub fn get_string(&self) -> String {
        self.value.to_string()
    }

    /// C++: `int TraCIInt::getType() const` [const, virtual]
    // int getType() const override {
    //         return traciType;
    //     }
    #[allow(unused_variables)]
    pub fn get_type(&self) -> i32 {
        self.traciType
    }

}

/// C++ class `libsumo::TraCIIntList` (libsumo/TraCIDefs.h)
#[derive(Clone)]
#[repr(C)]
pub struct TraCIIntList {
    /// C++ base class `TraCIResult`
    pub base: TraCIResult,
    pub value: Vec<i32>,
}

impl std::ops::Deref for TraCIIntList {
    type Target = TraCIResult;
    fn deref(&self) -> &TraCIResult {
        &self.base
    }
}
impl std::ops::DerefMut for TraCIIntList {
    fn deref_mut(&mut self) -> &mut TraCIResult {
        &mut self.base
    }
}

impl TraCIIntList {
    /// C++: `std::string TraCIIntList::getString() const` [const, virtual]
    // std::string getString() const override {
    //         std::ostringstream os;
    //         os << "[";
    //         for (int v : value) {
    //             os << v << ",";
    //         }
    //         os << "]";
    //         return os.str();
    //     }
    #[allow(unused_variables)]
    pub fn get_string(&self) -> String {
        let mut os = String::from("[");
        for v in &self.value {
            os += &v.to_string();
            os += ",";
        }
        os += "]";
        os
    }

}

/// C++ class `libsumo::TraCIJunctionFoe` (libsumo/TraCIDefs.h)
#[derive(Clone)]
pub struct TraCIJunctionFoe {
    /// the id of the vehicle with intersecting trajectory
    pub foeId: String,
    pub egoDist: f64,
    pub foeDist: f64,
    pub egoExitDist: f64,
    pub foeExitDist: f64,
    pub egoLane: String,
    pub foeLane: String,
    pub egoResponse: bool,
    pub foeResponse: bool,
}

impl TraCIJunctionFoe {
    /// C++: `std::string TraCIJunctionFoe::getString() const` [const]
    // std::string getString() const {
    //         std::ostringstream os;
    //         os << "TraCIJunctionFoe(foeId=" << foeId << ", egoDist=" << egoDist << ", foeDist=" << foeDist << ", foeDist=" << foeDist << ")";
    //         return os.str();
    //     }
    #[allow(unused_variables)]
    pub fn get_string(&self) -> String {
        format!("TraCIJunctionFoe(foeId={}, egoDist={}, foeDist={}, foeDist={})", self.foeId, cpp_dbl(self.egoDist), cpp_dbl(self.foeDist), cpp_dbl(self.foeDist))
    }

}

/// C++ class `libsumo::TraCIJunctionFoeVectorWrapped` (libsumo/TraCIDefs.h)
#[derive(Clone)]
#[repr(C)]
pub struct TraCIJunctionFoeVectorWrapped {
    /// C++ base class `TraCIResult`
    pub base: TraCIResult,
    pub value: Vec<TraCIJunctionFoe>,
}

impl std::ops::Deref for TraCIJunctionFoeVectorWrapped {
    type Target = TraCIResult;
    fn deref(&self) -> &TraCIResult {
        &self.base
    }
}
impl std::ops::DerefMut for TraCIJunctionFoeVectorWrapped {
    fn deref_mut(&mut self) -> &mut TraCIResult {
        &mut self.base
    }
}

impl TraCIJunctionFoeVectorWrapped {
    /// C++: `std::string TraCIJunctionFoeVectorWrapped::getString() const` [const, virtual]
    // std::string getString() const override {
    //         std::ostringstream os;
    //         os << "TraCIJunctionFoeVectorWrapped[";
    //         for (const TraCIJunctionFoe& v : value) {
    //             os << v.getString() << ",";
    //         }
    //         os << "]";
    //         return os.str();
    //     }
    #[allow(unused_variables)]
    pub fn get_string(&self) -> String {
        let mut os = String::from("TraCIJunctionFoeVectorWrapped[");
        for v in &self.value {
            os += &v.get_string();
            os += ",";
        }
        os += "]";
        os
    }

}

/// C++ class `libsumo::TraCILink` (libsumo/TraCIDefs.h)
#[derive(Clone)]
pub struct TraCILink {
    pub fromLane: String,
    pub viaLane: String,
    pub toLane: String,
}

impl TraCILink {
    /// C++: `void TraCILink::TraCILink()`
    // TraCILink() {}
    #[allow(unused_variables)]
    pub fn new() -> Self {
        TraCILink { fromLane: String::new(), viaLane: String::new(), toLane: String::new() }
    }

    /// C++: `void TraCILink::TraCILink(const std::string &, const std::string &, const std::string &)`
    // TraCILink(const std::string& _from, const std::string& _via, const std::string& _to)
    //         : fromLane(_from), viaLane(_via), toLane(_to) {}
    #[allow(unused_variables)]
    pub fn new_2(_from: &str, _via: &str, _to: &str) -> Self {
        TraCILink { fromLane: _from.to_string(), viaLane: _via.to_string(), toLane: _to.to_string() }
    }

    /// C++: `std::string TraCILink::getString() const` [const]
    // std::string getString() const {
    //         std::ostringstream os;
    //         os << "TraCILink(" << fromLane << "," << viaLane << "," << toLane << ")";
    //         return os.str();
    //     }
    #[allow(unused_variables)]
    pub fn get_string(&self) -> String {
        format!("TraCILink({},{},{})", self.fromLane, self.viaLane, self.toLane)
    }

}

/// C++ class `libsumo::TraCILinkVectorVectorWrapped` (libsumo/TraCIDefs.h)
#[derive(Clone)]
#[repr(C)]
pub struct TraCILinkVectorVectorWrapped {
    /// C++ base class `TraCIResult`
    pub base: TraCIResult,
    pub value: Vec<Vec<TraCILink>>,
}

impl std::ops::Deref for TraCILinkVectorVectorWrapped {
    type Target = TraCIResult;
    fn deref(&self) -> &TraCIResult {
        &self.base
    }
}
impl std::ops::DerefMut for TraCILinkVectorVectorWrapped {
    fn deref_mut(&mut self) -> &mut TraCIResult {
        &mut self.base
    }
}

impl TraCILinkVectorVectorWrapped {
    /// C++: `std::string TraCILinkVectorVectorWrapped::getString() const` [const, virtual]
    // std::string getString() const override {
    //         std::ostringstream os;
    //         os << "TraCILinkVectorVectorWrapped[";
    //         for (const std::vector<TraCILink>& v : value) {
    //             os << "[";
    //             for (const TraCILink& tl : v) {
    //                 os << tl.getString() << ",";
    //             }
    //         }
    //         os << "]";
    //         return os.str();
    //     }
    #[allow(unused_variables)]
    pub fn get_string(&self) -> String {
        let mut os = String::from("TraCILinkVectorVectorWrapped[");
        for v in &self.value {
            os += "[";
            for tl in v {
                os += &tl.get_string();
                os += ",";
            }
        }
        os += "]";
        os
    }

}

/// C++ class `libsumo::TraCILogic` (libsumo/TraCIDefs.h)
#[derive(Clone)]
pub struct TraCILogic {
    pub programID: String,
    pub type_: i32,
    pub currentPhaseIndex: i32,
    pub phases: Vec<Rc<TraCIPhase>>,
    pub subParameter: BTreeMap<String, String>,
}

impl TraCILogic {
    /// C++: `void TraCILogic::TraCILogic()`
    // TraCILogic() {}
    #[allow(unused_variables)]
    pub fn new() -> Self {
        TraCILogic { programID: String::new(), type_: 0, currentPhaseIndex: 0, phases: Vec::new(), subParameter: BTreeMap::new() }
    }

    /// C++: `void TraCILogic::TraCILogic(const std::string &, const int, const int, const std::vector<std::shared_ptr<libsumo::TraCIPhase>> &)`
    // TraCILogic(const std::string& _programID, const int _type, const int _currentPhaseIndex,
    //                const std::vector<std::shared_ptr<libsumo::TraCIPhase> >& _phases = std::vector<std::shared_ptr<libsumo::TraCIPhase> >())
    //         : programID(_programID), type(_type), currentPhaseIndex(_currentPhaseIndex), phases(_phases) {}
    #[allow(unused_variables)]
    pub fn new_2(_programID: &str, _type: i32, _currentPhaseIndex: i32, _phases: &Vec<Rc<TraCIPhase>>) -> Self {
        TraCILogic { programID: _programID.to_string(), type_: _type, currentPhaseIndex: _currentPhaseIndex, phases: _phases.clone(), subParameter: BTreeMap::new() }
    }

    /// C++: `std::string TraCILogic::getString() const` [const]
    // std::string getString() const {
    //         std::ostringstream os;
    //         os << "TraCILink(" << programID << "," << type << "," << currentPhaseIndex << ")";
    //         return os.str();
    //     }
    #[allow(unused_variables)]
    pub fn get_string(&self) -> String {
        // C++ prints "TraCILink(" for a TraCILogic (upstream typo), kept for identical output
        format!("TraCILink({},{},{})", self.programID, self.type_, self.currentPhaseIndex)
    }

}

/// C++ class `libsumo::TraCILogicVectorWrapped` (libsumo/TraCIDefs.h)
#[derive(Clone)]
#[repr(C)]
pub struct TraCILogicVectorWrapped {
    /// C++ base class `TraCIResult`
    pub base: TraCIResult,
    pub value: Vec<TraCILogic>,
}

impl std::ops::Deref for TraCILogicVectorWrapped {
    type Target = TraCIResult;
    fn deref(&self) -> &TraCIResult {
        &self.base
    }
}
impl std::ops::DerefMut for TraCILogicVectorWrapped {
    fn deref_mut(&mut self) -> &mut TraCIResult {
        &mut self.base
    }
}

impl TraCILogicVectorWrapped {
    /// C++: `std::string TraCILogicVectorWrapped::getString() const` [const, virtual]
    // std::string getString() const override {
    //         std::ostringstream os;
    //         os << "TraCILogicVectorWrapped[";
    //         for (const TraCILogic& v : value) {
    //             os << v.getString() << ",";
    //         }
    //         os << "]";
    //         return os.str();
    //     }
    #[allow(unused_variables)]
    pub fn get_string(&self) -> String {
        let mut os = String::from("TraCILogicVectorWrapped[");
        for v in &self.value {
            os += &v.get_string();
            os += ",";
        }
        os += "]";
        os
    }

}

/// C++ class `libsumo::TraCINextStopData` (libsumo/TraCIDefs.h)
#[derive(Clone)]
pub struct TraCINextStopData {
    /// The lane to stop at
    pub lane: String,
    /// The stopping position start
    pub startPos: f64,
    /// The stopping position end
    pub endPos: f64,
    /// Id assigned to the stop
    pub stoppingPlaceID: String,
    /// Stop flags
    pub stopFlags: i32,
    /// The intended (minimum) stopping duration
    pub duration: f64,
    /// The time at which the vehicle may continue its journey
    pub until: f64,
    /// The intended arrival time
    pub intendedArrival: f64,
    /// The actual arrival time (only for past stops)
    pub arrival: f64,
    /// The time at which this stop was ended
    pub depart: f64,
    /// the id of the vehicle (train portion) that splits of upon reaching this stop
    pub split: String,
    /// the id of the vehicle (train portion) to which this vehicle shall be joined
    pub join: String,
    /// additional information for this stop
    pub actType: String,
    /// id of the trip within a cyclical public transport route
    pub tripId: String,
    /// the new line id of the trip within a cyclical public transport route
    pub line: String,
    /// the speed at which this stop counts as reached (waypoint mode)
    pub speed: f64,
}

impl TraCINextStopData {
    /// C++: `void TraCINextStopData::TraCINextStopData(const std::string &, double, double, const std::string &, int, double, double, double, double, double, const std::string &, const std::string &, const std::string &, const std::string &, const std::string &, double)`
    // TraCINextStopData(const std::string& lane = "",
    //                       double startPos = INVALID_DOUBLE_VALUE,
    //                       double endPos = INVALID_DOUBLE_VALUE,
    //                       const std::string& stoppingPlaceID = "",
    //                       int stopFlags = 0,
    //                       double duration = INVALID_DOUBLE_VALUE,
    //                       double until = INVALID_DOUBLE_VALUE,
    //                       double intendedArrival = INVALID_DOUBLE_VALUE,
    //                       double arrival = INVALID_DOUBLE_VALUE,
    //                       double depart = INVALID_DOUBLE_VALUE,
    //                       const std::string& split = "",
    //                       const std::string& join = "",
    //                       const std::string& actType = "",
    //                       const std::string& tripId = "",
    //                       const std::string& line = "",
    //                       double speed = 0):
    //         lane(lane),
    //         startPos(startPos),
    //         endPos(endPos),
    //         stoppingPlaceID(stoppingPlaceID),
    //         stopFlags(stopFlags),
    //         duration(duration),
    //         until(until),
    //         intendedArrival(intendedArrival),
    //         arrival(arrival),
    //         depart(depart),
    //         split(split),
    //         join(join),
    //         actType(actType),
    //         tripId(tripId),
    //         line(line),
    //         speed(speed)
    //     {}
    #[allow(unused_variables)]
    pub fn new(lane: &str, startPos: f64, endPos: f64, stoppingPlaceID: &str, stopFlags: i32, duration: f64, until: f64, intendedArrival: f64, arrival: f64, depart: f64, split: &str, join: &str, actType: &str, tripId: &str, line: &str, speed: f64) -> Self {
        TraCINextStopData { lane: lane.to_string(), startPos, endPos, stoppingPlaceID: stoppingPlaceID.to_string(), stopFlags, duration, until, intendedArrival, arrival, depart, split: split.to_string(), join: join.to_string(), actType: actType.to_string(), tripId: tripId.to_string(), line: line.to_string(), speed }
    }

    /// C++: `std::string TraCINextStopData::getString() const` [const]
    // std::string getString() const {
    //         std::ostringstream os;
    //         os << "TraCINextStopData(" << lane << "," << endPos << "," << stoppingPlaceID
    //            << "," << stopFlags << "," << duration << "," << until
    //            << "," << arrival << ")";
    //         return os.str();
    //     }
    #[allow(unused_variables)]
    pub fn get_string(&self) -> String {
        format!("TraCINextStopData({},{},{},{},{},{},{})", self.lane, cpp_dbl(self.endPos), self.stoppingPlaceID, self.stopFlags, cpp_dbl(self.duration), cpp_dbl(self.until), cpp_dbl(self.arrival))
    }

}

/// A list of vehicle stops
/// C++ class `libsumo::TraCINextStopDataVectorWrapped` (libsumo/TraCIDefs.h)
#[derive(Clone)]
#[repr(C)]
pub struct TraCINextStopDataVectorWrapped {
    /// C++ base class `TraCIResult`
    pub base: TraCIResult,
    pub value: Vec<TraCINextStopData>,
}

impl std::ops::Deref for TraCINextStopDataVectorWrapped {
    type Target = TraCIResult;
    fn deref(&self) -> &TraCIResult {
        &self.base
    }
}
impl std::ops::DerefMut for TraCINextStopDataVectorWrapped {
    fn deref_mut(&mut self) -> &mut TraCIResult {
        &mut self.base
    }
}

impl TraCINextStopDataVectorWrapped {
    /// C++: `std::string TraCINextStopDataVectorWrapped::getString() const` [const, virtual]
    // std::string getString() const override {
    //         std::ostringstream os;
    //         os << "TraCINextStopDataVectorWrapped[";
    //         for (const TraCINextStopData& v : value) {
    //             os << v.getString() << ",";
    //         }
    //         os << "]";
    //         return os.str();
    //     }
    #[allow(unused_variables)]
    pub fn get_string(&self) -> String {
        let mut os = String::from("TraCINextStopDataVectorWrapped[");
        for v in &self.value {
            os += &v.get_string();
            os += ",";
        }
        os += "]";
        os
    }

}

/// C++ class `libsumo::TraCINextTLSData` (libsumo/TraCIDefs.h)
#[derive(Clone)]
pub struct TraCINextTLSData {
    /// The id of the next tls
    pub id: String,
    /// The tls index of the controlled link
    pub tlIndex: i32,
    /// The distance to the tls
    pub dist: f64,
    /// The current state of the tls
    pub state: i8,
}

impl TraCINextTLSData {
    /// C++: `std::string TraCINextTLSData::getString() const` [const]
    // std::string getString() const {
    //         std::ostringstream os;
    //         os << "TraCINextTLSData(" << id << "," << tlIndex << "," << dist
    //            << "," << state << ")";
    //         return os.str();
    //     }
    #[allow(unused_variables)]
    pub fn get_string(&self) -> String {
        format!("TraCINextTLSData({},{},{},{})", self.id, self.tlIndex, cpp_dbl(self.dist), self.state as u8 as char)
    }

}

/// C++ class `libsumo::TraCINextTLSDataVectorWrapped` (libsumo/TraCIDefs.h)
#[derive(Clone)]
#[repr(C)]
pub struct TraCINextTLSDataVectorWrapped {
    /// C++ base class `TraCIResult`
    pub base: TraCIResult,
    pub value: Vec<TraCINextTLSData>,
}

impl std::ops::Deref for TraCINextTLSDataVectorWrapped {
    type Target = TraCIResult;
    fn deref(&self) -> &TraCIResult {
        &self.base
    }
}
impl std::ops::DerefMut for TraCINextTLSDataVectorWrapped {
    fn deref_mut(&mut self) -> &mut TraCIResult {
        &mut self.base
    }
}

impl TraCINextTLSDataVectorWrapped {
    /// C++: `std::string TraCINextTLSDataVectorWrapped::getString() const` [const, virtual]
    // std::string getString() const override {
    //         std::ostringstream os;
    //         os << "TraCINextTLSDataVectorWrapped[";
    //         for (const TraCINextTLSData& v : value) {
    //             os << v.getString() << ",";
    //         }
    //         os << "]";
    //         return os.str();
    //     }
    #[allow(unused_variables)]
    pub fn get_string(&self) -> String {
        let mut os = String::from("TraCINextTLSDataVectorWrapped[");
        for v in &self.value {
            os += &v.get_string();
            os += ",";
        }
        os += "]";
        os
    }

}

/// C++ class `libsumo::TraCIPhase` (libsumo/TraCIDefs.h)
#[derive(Clone)]
pub struct TraCIPhase {
    pub duration: f64,
    pub state: String,
    pub minDur: f64,
    pub maxDur: f64,
    pub next: Vec<i32>,
    pub name: String,
    pub earlyTarget: String,
}

impl TraCIPhase {
    /// C++: `void TraCIPhase::TraCIPhase()`
    // TraCIPhase() {}
    #[allow(unused_variables)]
    pub fn new() -> Self {
        TraCIPhase { duration: 0.0, state: String::new(), minDur: 0.0, maxDur: 0.0, next: Vec::new(), name: String::new(), earlyTarget: String::new() }
    }

    /// C++: `void TraCIPhase::TraCIPhase(const double, const std::string &, const double, const double, const std::vector<int> &, const std::string &, const std::string &)`
    // TraCIPhase(const double _duration, const std::string& _state, const double _minDur = libsumo::INVALID_DOUBLE_VALUE,
    //                const double _maxDur = libsumo::INVALID_DOUBLE_VALUE,
    //                const std::vector<int>& _next = std::vector<int>(),
    //                const std::string& _name = "",
    //                const std::string& _earlyTarget = "") :
    //         duration(_duration), state(_state), minDur(_minDur), maxDur(_maxDur), next(_next), name(_name), earlyTarget(_earlyTarget) {}
    #[allow(unused_variables)]
    pub fn new_2(_duration: f64, _state: &str, _minDur: f64, _maxDur: f64, _next: &Vec<i32>, _name: &str, _earlyTarget: &str) -> Self {
        TraCIPhase { duration: _duration, state: _state.to_string(), minDur: _minDur, maxDur: _maxDur, next: _next.clone(), name: _name.to_string(), earlyTarget: _earlyTarget.to_string() }
    }

}

/// A 2D or 3D-position, for 2D positions z == INVALID_DOUBLE_VALUE
/// C++ class `libsumo::TraCIPosition` (libsumo/TraCIDefs.h)
#[derive(Clone)]
#[repr(C)]
pub struct TraCIPosition {
    /// C++ base class `TraCIResult`
    pub base: TraCIResult,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl std::ops::Deref for TraCIPosition {
    type Target = TraCIResult;
    fn deref(&self) -> &TraCIResult {
        &self.base
    }
}
impl std::ops::DerefMut for TraCIPosition {
    fn deref_mut(&mut self) -> &mut TraCIResult {
        &mut self.base
    }
}

impl TraCIPosition {
    /// C++: `std::string TraCIPosition::getString() const` [const, virtual]
    // std::string getString() const override {
    //         std::ostringstream os;
    //         os << "TraCIPosition(" << x << "," << y;
    //         if (z != INVALID_DOUBLE_VALUE) {
    //             os << "," << z;
    //         }
    //         os << ")";
    //         return os.str();
    //     }
    #[allow(unused_variables)]
    pub fn get_string(&self) -> String {
        let mut os = format!("TraCIPosition({},{}", cpp_dbl(self.x), cpp_dbl(self.y));
        if self.z != INVALID_DOUBLE_VALUE {
            os += &format!(",{}", cpp_dbl(self.z));
        }
        os += ")";
        os
    }

    /// C++: `int TraCIPosition::getType() const` [const, virtual]
    // int getType() const override {
    //         return z != INVALID_DOUBLE_VALUE ? POSITION_3D : POSITION_2D;
    //     }
    #[allow(unused_variables)]
    pub fn get_type(&self) -> i32 {
        if self.z != INVALID_DOUBLE_VALUE { POSITION_3D } else { POSITION_2D }
    }

}

/// A list of positions
/// C++ class `libsumo::TraCIPositionVector` (libsumo/TraCIDefs.h)
#[derive(Clone)]
#[repr(C)]
pub struct TraCIPositionVector {
    /// C++ base class `TraCIResult`
    pub base: TraCIResult,
    pub value: Vec<TraCIPosition>,
}

impl std::ops::Deref for TraCIPositionVector {
    type Target = TraCIResult;
    fn deref(&self) -> &TraCIResult {
        &self.base
    }
}
impl std::ops::DerefMut for TraCIPositionVector {
    fn deref_mut(&mut self) -> &mut TraCIResult {
        &mut self.base
    }
}

impl TraCIPositionVector {
    /// C++: `std::string TraCIPositionVector::getString() const` [const, virtual]
    // std::string getString() const override {
    //         std::ostringstream os;
    //         os << "[";
    //         for (const TraCIPosition& v : value) {
    //             os << "(" << v.x << "," << v.y << "," << v.z << ")";
    //         }
    //         os << "]";
    //         return os.str();
    //     }
    #[allow(unused_variables)]
    pub fn get_string(&self) -> String {
        let mut os = String::from("[");
        for v in &self.value {
            os += &format!("({},{},{})", cpp_dbl(v.x), cpp_dbl(v.y), cpp_dbl(v.z));
        }
        os += "]";
        os
    }

}

/// C++ class `libsumo::TraCIReservation` (libsumo/TraCIDefs.h)
#[derive(Clone)]
pub struct TraCIReservation {
    /// The id of the taxi reservation (usable for traci.vehicle.dispatchTaxi)
    pub id: String,
    /// The persons ids that are part of this reservation
    pub persons: Vec<String>,
    /// The group id of this reservation
    pub group: String,
    /// The origin edge id
    pub fromEdge: String,
    /// The destination edge id
    pub toEdge: String,
    /// pickup position on the origin edge
    pub departPos: f64,
    /// drop-off position on the destination edge
    pub arrivalPos: f64,
    /// pickup-time
    pub depart: f64,
    /// time when the reservation was made
    pub reservationTime: f64,
    /// the state of this reservation
    pub state: i32,
}

impl TraCIReservation {
    /// C++: `void TraCIReservation::TraCIReservation()`
    // TraCIReservation() {}
    #[allow(unused_variables)]
    pub fn new() -> Self {
        TraCIReservation { id: String::new(), persons: Vec::new(), group: String::new(), fromEdge: String::new(), toEdge: String::new(), departPos: 0.0, arrivalPos: 0.0, depart: 0.0, reservationTime: 0.0, state: 0 }
    }

    /// C++: `void TraCIReservation::TraCIReservation(const std::string &, const std::vector<std::string> &, const std::string &, const std::string &, const std::string &, double, double, double, double, int)`
    // TraCIReservation(const std::string& id,
    //                      const std::vector<std::string>& persons,
    //                      const std::string& group,
    //                      const std::string& fromEdge,
    //                      const std::string& toEdge,
    //                      double departPos,
    //                      double arrivalPos,
    //                      double depart,
    //                      double reservationTime,
    //                      int state) :
    //         id(id), persons(persons), group(group), fromEdge(fromEdge), toEdge(toEdge), departPos(departPos), arrivalPos(arrivalPos),
    //         depart(depart), reservationTime(reservationTime), state(state) {}
    #[allow(unused_variables)]
    pub fn new_2(id: &str, persons: &Vec<String>, group: &str, fromEdge: &str, toEdge: &str, departPos: f64, arrivalPos: f64, depart: f64, reservationTime: f64, state: i32) -> Self {
        TraCIReservation { id: id.to_string(), persons: persons.clone(), group: group.to_string(), fromEdge: fromEdge.to_string(), toEdge: toEdge.to_string(), departPos, arrivalPos, depart, reservationTime, state }
    }

    /// C++: `std::string TraCIReservation::getString() const` [const]
    // std::string getString() const {
    //         std::ostringstream os;
    //         os << "TraCIReservation(id=" << id << ")";
    //         return os.str();
    //     }
    #[allow(unused_variables)]
    pub fn get_string(&self) -> String {
        format!("TraCIReservation(id={})", self.id)
    }

}

/// C++ class `libsumo::TraCIReservationVectorWrapped` (libsumo/TraCIDefs.h)
#[derive(Clone)]
#[repr(C)]
pub struct TraCIReservationVectorWrapped {
    /// C++ base class `TraCIResult`
    pub base: TraCIResult,
    pub value: Vec<TraCIReservation>,
}

impl std::ops::Deref for TraCIReservationVectorWrapped {
    type Target = TraCIResult;
    fn deref(&self) -> &TraCIResult {
        &self.base
    }
}
impl std::ops::DerefMut for TraCIReservationVectorWrapped {
    fn deref_mut(&mut self) -> &mut TraCIResult {
        &mut self.base
    }
}

impl TraCIReservationVectorWrapped {
    /// C++: `std::string TraCIReservationVectorWrapped::getString() const` [const, virtual]
    // std::string getString() const override {
    //         std::ostringstream os;
    //         os << "TraCIReservationVectorWrapped[";
    //         for (const TraCIReservation& v : value) {
    //             os << v.getString() << ",";
    //         }
    //         os << "]";
    //         return os.str();
    //     }
    #[allow(unused_variables)]
    pub fn get_string(&self) -> String {
        let mut os = String::from("TraCIReservationVectorWrapped[");
        for v in &self.value {
            os += &v.get_string();
            os += ",";
        }
        os += "]";
        os
    }

}

/// @{
/// C++ class `libsumo::TraCIResult` (libsumo/TraCIDefs.h)
#[derive(Clone)]
#[repr(C)]
pub struct TraCIResult {
    /// Stands in for C++ virtual getType(): -1 for a plain TraCIResult, otherwise the type set by the subclass constructor
    pub result_type: i32,
}

impl TraCIResult {
    /// C++: `void TraCIResult::TraCIResult()`
    // TraCIResult() {}
    #[allow(unused_variables)]
    pub fn new() -> Self {
        TraCIResult { result_type: -1 }
    }

    /// C++: `void TraCIResult::~TraCIResult() noexcept` [virtual]
    // virtual ~TraCIResult() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
    }

    /// C++: `void TraCIResult::TraCIResult(const TraCIResult &)`
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &TraCIResult) -> Self {
        TraCIResult { result_type: arg0.result_type }
    }

    /// C++: `TraCIResult & TraCIResult::operator=(const TraCIResult &)`
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &TraCIResult) -> &mut TraCIResult {
        self.result_type = arg0.result_type;
        self
    }

    /// C++: `std::string TraCIResult::getString() const` [const, virtual]
    // virtual std::string getString() const {
    //         return "";
    //     }
    #[allow(unused_variables)]
    pub fn get_string(&self) -> String {
        String::new()
    }

    /// C++: `int TraCIResult::getType() const` [const, virtual]
    // virtual int getType() const {
    //         return -1;
    //     }
    #[allow(unused_variables)]
    pub fn get_type(&self) -> i32 {
        self.result_type
    }

}

/// An edgeId, position and laneIndex
/// C++ class `libsumo::TraCIRoadPosition` (libsumo/TraCIDefs.h)
#[derive(Clone)]
#[repr(C)]
pub struct TraCIRoadPosition {
    /// C++ base class `TraCIResult`
    pub base: TraCIResult,
    pub edgeID: String,
    pub pos: f64,
    pub laneIndex: i32,
}

impl std::ops::Deref for TraCIRoadPosition {
    type Target = TraCIResult;
    fn deref(&self) -> &TraCIResult {
        &self.base
    }
}
impl std::ops::DerefMut for TraCIRoadPosition {
    fn deref_mut(&mut self) -> &mut TraCIResult {
        &mut self.base
    }
}

impl TraCIRoadPosition {
    /// C++: `void TraCIRoadPosition::TraCIRoadPosition(const std::string, const double, const int)`
    // TraCIRoadPosition(const std::string e = "", const double p = INVALID_DOUBLE_VALUE, const int li = INVALID_INT_VALUE) : edgeID(e), pos(p), laneIndex(li) {}
    #[allow(unused_variables)]
    pub fn new(e: String, p: f64, li: i32) -> Self {
        TraCIRoadPosition { base: TraCIResult { result_type: POSITION_ROADMAP }, edgeID: e, pos: p, laneIndex: li }
    }

    /// C++: `std::string TraCIRoadPosition::getString() const` [const, virtual]
    // std::string getString() const override {
    //         std::ostringstream os;
    //         os << "TraCIRoadPosition(" << edgeID << "_" << laneIndex << "," << pos << ")";
    //         return os.str();
    //     }
    #[allow(unused_variables)]
    pub fn get_string(&self) -> String {
        format!("TraCIRoadPosition({}_{},{})", self.edgeID, self.laneIndex, cpp_dbl(self.pos))
    }

    /// C++: `int TraCIRoadPosition::getType() const` [const, virtual]
    // int getType() const override {
    //         return POSITION_ROADMAP;
    //     }
    #[allow(unused_variables)]
    pub fn get_type(&self) -> i32 {
        POSITION_ROADMAP
    }

}

/// C++ class `libsumo::TraCISignalConstraint` (libsumo/TraCIDefs.h)
#[derive(Clone)]
pub struct TraCISignalConstraint {
    /// the idea of the rail signal where this constraint is active
    pub signalId: String,
    /// the tripId or vehicle id of the train that is constrained
    pub tripId: String,
    /// the tripId or vehicle id of the train that must pass first
    pub foeId: String,
    /// the tlsID of the rail signla that the foe must pass first
    pub foeSignal: String,
    /// the number of trains that must be recorded at the foeSignal
    pub limit: i32,
    /// the type of constraint (predecessor:0, insertionPredecessor:1)
    pub type_: i32,
    /// whether tripId must still wait for foeId to pass foeSignal
    pub mustWait: bool,
    /// whether this constraint is active
    pub active: bool,
    /// additional parameters
    pub param: BTreeMap<String, String>,
}

impl TraCISignalConstraint {
    /// C++: `std::string TraCISignalConstraint::getString() const` [const]
    // std::string getString() const {
    //         std::ostringstream os;
    //         os << "TraCISignalConstraint(signalId=" << signalId << ", tripid=" << tripId << ", foeSignal=" << foeSignal << ", foeId=" << foeId << ")";
    //         return os.str();
    //     }
    #[allow(unused_variables)]
    pub fn get_string(&self) -> String {
        format!("TraCISignalConstraint(signalId={}, tripid={}, foeSignal={}, foeId={})", self.signalId, self.tripId, self.foeSignal, self.foeId)
    }

}

/// C++ class `libsumo::TraCISignalConstraintVectorWrapped` (libsumo/TraCIDefs.h)
#[derive(Clone)]
#[repr(C)]
pub struct TraCISignalConstraintVectorWrapped {
    /// C++ base class `TraCIResult`
    pub base: TraCIResult,
    pub value: Vec<TraCISignalConstraint>,
}

impl std::ops::Deref for TraCISignalConstraintVectorWrapped {
    type Target = TraCIResult;
    fn deref(&self) -> &TraCIResult {
        &self.base
    }
}
impl std::ops::DerefMut for TraCISignalConstraintVectorWrapped {
    fn deref_mut(&mut self) -> &mut TraCIResult {
        &mut self.base
    }
}

impl TraCISignalConstraintVectorWrapped {
    /// C++: `std::string TraCISignalConstraintVectorWrapped::getString() const` [const, virtual]
    // std::string getString() const override {
    //         std::ostringstream os;
    //         os << "TraCISignalConstraintVectorWrapped[";
    //         for (const TraCISignalConstraint& v : value) {
    //             os << v.getString() << ",";
    //         }
    //         os << "]";
    //         return os.str();
    //     }
    #[allow(unused_variables)]
    pub fn get_string(&self) -> String {
        let mut os = String::from("TraCISignalConstraintVectorWrapped[");
        for v in &self.value {
            os += &v.get_string();
            os += ",";
        }
        os += "]";
        os
    }

}

/// C++ class `libsumo::TraCIStage` (libsumo/TraCIDefs.h)
#[derive(Clone)]
#[repr(C)]
pub struct TraCIStage {
    /// C++ base class `TraCIResult`
    pub base: TraCIResult,
    /// The type of stage (walking, driving, ...)
    pub type_: i32,
    /// The vehicle type when using a private car or bike
    pub vType: String,
    /// The line or the id of the vehicle type
    pub line: String,
    /// The id of the destination stop
    pub destStop: String,
    /// The sequence of edges to travel
    pub edges: Vec<String>,
    /// duration of the stage in seconds
    pub travelTime: f64,
    /// effort needed
    pub cost: f64,
    /// length in m
    pub length: f64,
    /// id of the intended vehicle for public transport ride
    pub intended: String,
    /// intended depart time for public transport ride or INVALID_DOUBLE_VALUE
    pub depart: f64,
    /// position on the lane when starting the stage
    pub departPos: f64,
    /// position on the lane when ending the stage
    pub arrivalPos: f64,
    /// arbitrary description string
    pub description: String,
}

impl std::ops::Deref for TraCIStage {
    type Target = TraCIResult;
    fn deref(&self) -> &TraCIResult {
        &self.base
    }
}
impl std::ops::DerefMut for TraCIStage {
    fn deref_mut(&mut self) -> &mut TraCIResult {
        &mut self.base
    }
}

impl TraCIStage {
    /// C++: `void TraCIStage::TraCIStage(int, const std::string &, const std::string &, const std::string &, const std::vector<std::string> &, double, double, double, const std::string &, double, double, double, const std::string &)`
    // TraCIStage(int type = INVALID_INT_VALUE, const std::string& vType = "", const std::string& line = "", const std::string& destStop = "",
    //                const std::vector<std::string>& edges = std::vector<std::string>(),
    //                double travelTime = INVALID_DOUBLE_VALUE, double cost = INVALID_DOUBLE_VALUE, double length = INVALID_DOUBLE_VALUE,
    //                const std::string& intended = "", double depart = INVALID_DOUBLE_VALUE, double departPos = INVALID_DOUBLE_VALUE,
    //                double arrivalPos = INVALID_DOUBLE_VALUE, const std::string& description = "") :
    //         type(type), vType(vType), line(line), destStop(destStop), edges(edges), travelTime(travelTime), cost(cost),
    //         length(length), intended(intended), depart(depart), departPos(departPos), arrivalPos(arrivalPos), description(description) {}
    #[allow(unused_variables)]
    pub fn new(type_: i32, vType: &str, line: &str, destStop: &str, edges: &Vec<String>, travelTime: f64, cost: f64, length: f64, intended: &str, depart: f64, departPos: f64, arrivalPos: f64, description: &str) -> Self {
        TraCIStage { base: TraCIResult::new(), type_, vType: vType.to_string(), line: line.to_string(), destStop: destStop.to_string(), edges: edges.clone(), travelTime, cost, length, intended: intended.to_string(), depart, departPos, arrivalPos, description: description.to_string() }
    }

}

/// C++ class `libsumo::TraCIString` (libsumo/TraCIDefs.h)
#[derive(Clone)]
#[repr(C)]
pub struct TraCIString {
    /// C++ base class `TraCIResult`
    pub base: TraCIResult,
    pub value: String,
}

impl std::ops::Deref for TraCIString {
    type Target = TraCIResult;
    fn deref(&self) -> &TraCIResult {
        &self.base
    }
}
impl std::ops::DerefMut for TraCIString {
    fn deref_mut(&mut self) -> &mut TraCIResult {
        &mut self.base
    }
}

impl TraCIString {
    /// C++: `void TraCIString::TraCIString(std::string)`
    // TraCIString(std::string v = "") : value(v) {}
    #[allow(unused_variables)]
    pub fn new(v: String) -> Self {
        TraCIString { base: TraCIResult { result_type: TYPE_STRING }, value: v }
    }

    /// C++: `std::string TraCIString::getString() const` [const, virtual]
    // std::string getString() const override {
    //         return value;
    //     }
    #[allow(unused_variables)]
    pub fn get_string(&self) -> String {
        self.value.clone()
    }

    /// C++: `int TraCIString::getType() const` [const, virtual]
    // int getType() const override {
    //         return libsumo::TYPE_STRING;
    //     }
    #[allow(unused_variables)]
    pub fn get_type(&self) -> i32 {
        TYPE_STRING
    }

}

/// C++ class `libsumo::TraCIStringDoublePairList` (libsumo/TraCIDefs.h)
#[derive(Clone)]
#[repr(C)]
pub struct TraCIStringDoublePairList {
    /// C++ base class `TraCIResult`
    pub base: TraCIResult,
    pub value: Vec<(String, f64)>,
}

impl std::ops::Deref for TraCIStringDoublePairList {
    type Target = TraCIResult;
    fn deref(&self) -> &TraCIResult {
        &self.base
    }
}
impl std::ops::DerefMut for TraCIStringDoublePairList {
    fn deref_mut(&mut self) -> &mut TraCIResult {
        &mut self.base
    }
}

impl TraCIStringDoublePairList {
    /// C++: `std::string TraCIStringDoublePairList::getString() const` [const, virtual]
    // std::string getString() const override {
    //         std::ostringstream os;
    //         os << "[";
    //         for (const auto& v : value) {
    //             os << "(" << v.first << "," << v.second << "),";
    //         }
    //         os << "]";
    //         return os.str();
    //     }
    #[allow(unused_variables)]
    pub fn get_string(&self) -> String {
        let mut os = String::from("[");
        for v in &self.value {
            os += &format!("({},{}),", v.0, cpp_dbl(v.1));
        }
        os += "]";
        os
    }

}

/// C++ class `libsumo::TraCIStringList` (libsumo/TraCIDefs.h)
#[derive(Clone)]
#[repr(C)]
pub struct TraCIStringList {
    /// C++ base class `TraCIResult`
    pub base: TraCIResult,
    pub value: Vec<String>,
}

impl std::ops::Deref for TraCIStringList {
    type Target = TraCIResult;
    fn deref(&self) -> &TraCIResult {
        &self.base
    }
}
impl std::ops::DerefMut for TraCIStringList {
    fn deref_mut(&mut self) -> &mut TraCIResult {
        &mut self.base
    }
}

impl TraCIStringList {
    /// C++: `std::string TraCIStringList::getString() const` [const, virtual]
    // std::string getString() const override {
    //         std::ostringstream os;
    //         os << "[";
    //         for (std::string v : value) {
    //             os << v << ",";
    //         }
    //         os << "]";
    //         return os.str();
    //     }
    #[allow(unused_variables)]
    pub fn get_string(&self) -> String {
        let mut os = String::from("[");
        for v in &self.value {
            os += v.as_str();
            os += ",";
        }
        os += "]";
        os
    }

}

/// mirrors MSInductLoop::VehicleData
/// C++ class `libsumo::TraCIVehicleData` (libsumo/TraCIDefs.h)
#[derive(Clone)]
pub struct TraCIVehicleData {
    /// The id of the vehicle
    pub id: String,
    /// Length of the vehicle
    pub length: f64,
    /// Entry-time of the vehicle in [s]
    pub entryTime: f64,
    /// Leave-time of the vehicle in [s]
    pub leaveTime: f64,
    /// Type of the vehicle in
    pub typeID: String,
}

impl TraCIVehicleData {
    /// C++: `std::string TraCIVehicleData::getString() const` [const]
    // std::string getString() const {
    //         std::ostringstream os;
    //         os << "TraCIVehicleData(" << id << "," << length << "," << entryTime
    //            << "," << leaveTime << "," << typeID << ")";
    //         return os.str();
    //     }
    #[allow(unused_variables)]
    pub fn get_string(&self) -> String {
        format!("TraCIVehicleData({},{},{},{},{})", self.id, cpp_dbl(self.length), cpp_dbl(self.entryTime), cpp_dbl(self.leaveTime), self.typeID)
    }

}

/// C++ class `libsumo::TraCIVehicleDataVectorWrapped` (libsumo/TraCIDefs.h)
#[derive(Clone)]
#[repr(C)]
pub struct TraCIVehicleDataVectorWrapped {
    /// C++ base class `TraCIResult`
    pub base: TraCIResult,
    pub value: Vec<TraCIVehicleData>,
}

impl std::ops::Deref for TraCIVehicleDataVectorWrapped {
    type Target = TraCIResult;
    fn deref(&self) -> &TraCIResult {
        &self.base
    }
}
impl std::ops::DerefMut for TraCIVehicleDataVectorWrapped {
    fn deref_mut(&mut self) -> &mut TraCIResult {
        &mut self.base
    }
}

impl TraCIVehicleDataVectorWrapped {
    /// C++: `std::string TraCIVehicleDataVectorWrapped::getString() const` [const, virtual]
    // std::string getString() const override {
    //         std::ostringstream os;
    //         os << "TraCIVehicleDataVectorWrapped[";
    //         for (const TraCIVehicleData& v : value) {
    //             os << v.getString() << ",";
    //         }
    //         os << "]";
    //         return os.str();
    //     }
    #[allow(unused_variables)]
    pub fn get_string(&self) -> String {
        let mut os = String::from("TraCIVehicleDataVectorWrapped[");
        for v in &self.value {
            os += &v.get_string();
            os += ",";
        }
        os += "]";
        os
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cpp_scalar_formatting() {
        // C++ ostream default: %g with precision 6, bool without boolalpha.
        assert_eq!(cpp_dbl(13.89), "13.89");
        assert_eq!(cpp_dbl(0.5), "0.5");
        assert_eq!(cpp_bool(true), 1);
        assert_eq!(cpp_bool(false), 0);
    }

    #[test]
    fn collision_string_matches_cpp() {
        let c = TraCICollision {
            collider: "a".to_string(),
            victim: "b".to_string(),
            colliderType: "car".to_string(),
            victimType: "bike".to_string(),
            colliderSpeed: 1.,
            victimSpeed: 2.,
            type_: "collision".to_string(),
            lane: "edge_0".to_string(),
            pos: 3.,
        };
        assert_eq!(c.get_string(), "TraCICollision(collider=a, victim=b)");
    }
}

