//! Port of utils/router/AFInfo.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `AFInfo` (utils/router/AFInfo.h)
pub struct AFInfo<E> {
    _p: std::marker::PhantomData<(E,)>,
}

impl<E> AFInfo<E> {
}

/// Derived from ArcInfoBase. Therefore it tholds a pointer to the associated edge, flag information for arc flag routing, a key for sorting the heap, a flag indicating whether the edge has already been touched. Additionally, it has labels for holding (tentative/final) distances to the boundary nodes
/// C++ class `AFInfo::ArcInfo` (utils/router/AFInfo.h)
#[repr(C)]
pub struct AFInfo_ArcInfo {
    /// C++ base class `ArcInfoBase`
    pub base: AFInfo_ArcInfoBase,
    /// The efforts to boundary nodes
    pub effortsToBoundaryNodes: i32,
}

impl std::ops::Deref for AFInfo_ArcInfo {
    type Target = AFInfo_ArcInfoBase;
    fn deref(&self) -> &AFInfo_ArcInfoBase {
        &self.base
    }
}
impl std::ops::DerefMut for AFInfo_ArcInfo {
    fn deref_mut(&mut self) -> &mut AFInfo_ArcInfoBase {
        &mut self.base
    }
}

impl AFInfo_ArcInfo {
    /// C++: `void ArcInfo::ArcInfo(const E *const)`
    /// Constructor param[in] e The edge
    // ArcInfo(const E* const e) :
    //             ArcInfoBase(e) {
    //         }
    #[allow(unused_variables)]
    pub fn new(e: Ptr<Unmapped /* E */>) -> Self {
        todo!("port ArcInfo::ArcInfo")
    }

    /// C++: `ArcInfo & ArcInfo::operator=(const ArcInfo &)`
    /// Copy constructor
    // ArcInfo& operator=(const ArcInfo& other) {
    //             // guard self assignment
    //             if (this == &other) {
    //                 return *this;
    //             }
    //             static_cast<ArcInfoBase>(this) = static_cast<ArcInfoBase>(other);
    //             effortsToBoundaryNodes = other.effortsToBoundaryNodes;
    //             return *this;
    //         }
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, other: &AFInfo_ArcInfo) -> &mut AFInfo_ArcInfo {
        todo!("port ArcInfo::operator=")
    }

    /// C++: `void ArcInfo::~ArcInfo()`
    /// Destructor
    // ~ArcInfo() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port ArcInfo::~ArcInfo")
    }

    /// C++: `void ArcInfo::reset()`
    /// Reset the arc information
    // void reset() {
    //             // do nothing
    //         }
    #[allow(unused_variables)]
    pub fn reset(&mut self) {
        todo!("port ArcInfo::reset")
    }

}

/// Derived from FlagInfo. Therefore holds a pointer to the associated edge, and flag information for arc flag routing. Additionally, it holds a key for sorting the heap, and a flag indicating whether the edge has already been touched
/// C++ class `AFInfo::ArcInfoBase` (utils/router/AFInfo.h)
#[repr(C)]
pub struct AFInfo_ArcInfoBase {
    /// C++ base class `FlagInfo`
    pub base: AFInfo_FlagInfo,
    /// The arc flags
    pub arcFlags: i32,
    /// The key for sorting the heap
    pub key: f64,
    /// The flag indicating whether the edge has already been touched or not
    pub touched: bool,
}

impl std::ops::Deref for AFInfo_ArcInfoBase {
    type Target = AFInfo_FlagInfo;
    fn deref(&self) -> &AFInfo_FlagInfo {
        &self.base
    }
}
impl std::ops::DerefMut for AFInfo_ArcInfoBase {
    fn deref_mut(&mut self) -> &mut AFInfo_FlagInfo {
        &mut self.base
    }
}

impl AFInfo_ArcInfoBase {
    /// C++: `void ArcInfoBase::ArcInfoBase(const E *const)`
    /// Constructor param[in] e The edge
    // ArcInfoBase(const E* const e) :
    //             FlagInfo(e),
    //             key(std::numeric_limits<double>::max()),
    //             touched(false) {
    //         }
    #[allow(unused_variables)]
    pub fn new(e: Ptr<Unmapped /* E */>) -> Self {
        todo!("port ArcInfoBase::ArcInfoBase")
    }

    /// C++: `ArcInfoBase & ArcInfoBase::operator=(const ArcInfoBase &)`
    /// Copy constructor
    // ArcInfoBase& operator=(const ArcInfoBase& other) {
    //             // guard self assignment
    //             if (this == &other) {
    //                 return *this;
    //             }
    //             static_cast<FlagInfo>(this) = static_cast<FlagInfo>(other);
    //             key = other.key;
    //             touched = other.touched;
    //             return *this;
    //         }
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, other: &AFInfo_ArcInfoBase) -> &mut AFInfo_ArcInfoBase {
        todo!("port ArcInfoBase::operator=")
    }

    /// C++: `void ArcInfoBase::~ArcInfoBase()`
    // ~ArcInfoBase() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port ArcInfoBase::~ArcInfoBase")
    }

    /// C++: `void ArcInfoBase::reset()`
    // void reset() {
    //             // do nothing
    //         }
    #[allow(unused_variables)]
    pub fn reset(&mut self) {
        todo!("port ArcInfoBase::reset")
    }

}

/// It holds a pointer to the associated edge, and flag information for arc flag routing
/// C++ class `AFInfo::FlagInfo` (utils/router/AFInfo.h)
pub struct AFInfo_FlagInfo {
    /// The current edge
    pub edge: Ptr<Unmapped /* E */>,
    /// The arc flags
    pub arcFlags: i32,
}

impl AFInfo_FlagInfo {
    /// C++: `void FlagInfo::FlagInfo(const E *const)`
    /// Constructor param[in] e The edge
    // FlagInfo(const E* const e) :
    //             edge(e) {
    //         }
    #[allow(unused_variables)]
    pub fn new(e: Ptr<Unmapped /* E */>) -> Self {
        todo!("port FlagInfo::FlagInfo")
    }

    /// C++: `FlagInfo & FlagInfo::operator=(const FlagInfo &)`
    /// Copy constructor
    // FlagInfo& operator=(const FlagInfo& other) {
    //             // guard self assignment
    //             if (this == &other) {
    //                 return *this;
    //             }
    //             edge = other.edge;
    //             arcFlags = other.arcFlags;
    //             return *this;
    //         }
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, other: &AFInfo_FlagInfo) -> &mut AFInfo_FlagInfo {
        todo!("port FlagInfo::operator=")
    }

    /// C++: `void FlagInfo::~FlagInfo()` [virtual]
    /// Destructor
    // virtual ~FlagInfo() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port FlagInfo::~FlagInfo")
    }

    /// C++: `void FlagInfo::reset()` [virtual]
    /// Reset the flag information
    // virtual void reset() {
    //             arcFlags.clear();
    //         }
    #[allow(unused_variables)]
    pub fn reset(&mut self) {
        todo!("port FlagInfo::reset")
    }

}

