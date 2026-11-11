//! Port of marouter/ROMAEdge.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// A basic edge for routing applications
/// C++ class `ROMAEdge` (marouter/ROMAEdge.h)
pub struct ROMAEdge {
    /// C++ base class `ROEdge`
    pub base: ROEdge,
    pub myLeftTurns: BTreeSet<Ptr<ROMAEdge>>,
    pub myFlow: Unmapped /* ValueTimeLine<double> */,
    pub myHelpFlow: Unmapped /* ValueTimeLine<double> */,
}

impl std::ops::Deref for ROMAEdge {
    type Target = ROEdge;
    fn deref(&self) -> &ROEdge {
        &self.base
    }
}
impl std::ops::DerefMut for ROMAEdge {
    fn deref_mut(&mut self) -> &mut ROEdge {
        &mut self.base
    }
}

impl ROMAEdge {
    /// C++: `void ROMAEdge::ROMAEdge(const std::string &, RONode *, RONode *, int, const int, const std::string &, const std::string &)`
    /// Constructor
    // ROMAEdge::ROMAEdge(const std::string& id, RONode* from, RONode* to, int index, const int priority, const std::string& type, const std::string& routingType)
    //     : ROEdge(id, from, to, index, priority, type, routingType) {
    // }
    #[allow(unused_variables)]
    pub fn new(id: &str, from: Ptr<RONode>, to: Ptr<RONode>, index: i32, priority: i32, type_: &str, routingType: &str) -> Self {
        todo!("port ROMAEdge::ROMAEdge")
    }

    /// C++: `void ROMAEdge::~ROMAEdge() noexcept` [virtual]
    /// Destructor
    // ROMAEdge::~ROMAEdge() {
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port ROMAEdge::~ROMAEdge")
    }

    /// C++: `void ROMAEdge::addSuccessor(ROEdge *, ROEdge *, std::string)` [virtual]
    /// Adds information about a connected edge.
    // void
    // ROMAEdge::addSuccessor(ROEdge* s, ROEdge* via, std::string dir) {
    //     ROEdge::addSuccessor(s, via, dir);
    //     if (dir == "l" || dir == "L") {
    //         myLeftTurns.insert(static_cast<ROMAEdge*>(s));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_successor(&mut self, s: Ptr<ROEdge>, via: Ptr<ROEdge>, dir: String) {
        todo!("port ROMAEdge::addSuccessor")
    }

    /// C++: `void ROMAEdge::setFlow(const double, const double, const double)`
    // void setFlow(const double begin, const double end, const double flow) {
    //         myFlow.add(begin, end, flow);
    //     }
    #[allow(unused_variables)]
    pub fn set_flow(&mut self, begin: f64, end: f64, flow: f64) {
        todo!("port ROMAEdge::setFlow")
    }

    /// C++: `double ROMAEdge::getFlow(const double) const` [const]
    // double getFlow(const double time) const {
    //         return myFlow.getValue(time);
    //     }
    #[allow(unused_variables)]
    pub fn get_flow(&self, time: f64) -> f64 {
        todo!("port ROMAEdge::getFlow")
    }

    /// C++: `void ROMAEdge::setHelpFlow(const double, const double, const double)`
    // void setHelpFlow(const double begin, const double end, const double flow) {
    //         myHelpFlow.add(begin, end, flow);
    //     }
    #[allow(unused_variables)]
    pub fn set_help_flow(&mut self, begin: f64, end: f64, flow: f64) {
        todo!("port ROMAEdge::setHelpFlow")
    }

    /// C++: `double ROMAEdge::getHelpFlow(const double) const` [const]
    // double getHelpFlow(const double time) const {
    //         return myHelpFlow.getValue(time);
    //     }
    #[allow(unused_variables)]
    pub fn get_help_flow(&self, time: f64) -> f64 {
        todo!("port ROMAEdge::getHelpFlow")
    }

    /// C++: `void ROMAEdge::ROMAEdge(const ROMAEdge &)` [private]
    /// Invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(src: &ROMAEdge) -> Self {
        todo!("port ROMAEdge::ROMAEdge")
    }

    /// C++: `ROMAEdge & ROMAEdge::operator=(const ROMAEdge &)` [private]
    /// Invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, src: &ROMAEdge) -> &mut ROMAEdge {
        todo!("port ROMAEdge::operator=")
    }

}

