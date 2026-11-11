//! Port of dfrouter/RODFEdge.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `RODFEdge` (dfrouter/RODFEdge.h)
pub struct RODFEdge {
    /// C++ base class `ROEdge`
    pub base: ROEdge,
    pub myFlows: Vec<FlowDef>,
}

impl std::ops::Deref for RODFEdge {
    type Target = ROEdge;
    fn deref(&self) -> &ROEdge {
        &self.base
    }
}
impl std::ops::DerefMut for RODFEdge {
    fn deref_mut(&mut self) -> &mut ROEdge {
        &mut self.base
    }
}

impl RODFEdge {
    /// C++: `void RODFEdge::RODFEdge(const std::string &, RONode *, RONode *, int, const int, const std::string &, const std::string &)`
    /// Constructor
    // RODFEdge::RODFEdge(const std::string& id, RONode* from, RONode* to, int index, const int priority, const std::string& type, const std::string& routingType)
    //     : ROEdge(id, from, to, index, priority, type, routingType) {}
    #[allow(unused_variables)]
    pub fn new(id: &str, from: Ptr<RONode>, to: Ptr<RONode>, index: i32, priority: i32, type_: &str, routingType: &str) -> Self {
        todo!("port RODFEdge::RODFEdge")
    }

    /// C++: `void RODFEdge::~RODFEdge() noexcept` [virtual]
    /// Destructor
    // RODFEdge::~RODFEdge() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port RODFEdge::~RODFEdge")
    }

    /// C++: `void RODFEdge::setFlows(const std::vector<FlowDef> &)`
    // void
    // RODFEdge::setFlows(const std::vector<FlowDef>& flows) {
    //     myFlows = flows;
    // }
    #[allow(unused_variables)]
    pub fn set_flows(&mut self, flows: &Vec<FlowDef>) {
        todo!("port RODFEdge::setFlows")
    }

    /// C++: `const std::vector<FlowDef> & RODFEdge::getFlows() const` [const]
    // const std::vector<FlowDef>&
    // RODFEdge::getFlows() const {
    //     return myFlows;
    // }
    #[allow(unused_variables)]
    pub fn get_flows(&self) -> &Vec<FlowDef> {
        todo!("port RODFEdge::getFlows")
    }

    /// C++: `void RODFEdge::RODFEdge(const RODFEdge &)` [private]
    /// Invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(src: &RODFEdge) -> Self {
        todo!("port RODFEdge::RODFEdge")
    }

    /// C++: `RODFEdge & RODFEdge::operator=(const RODFEdge &)` [private]
    /// Invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, src: &RODFEdge) -> &mut RODFEdge {
        todo!("port RODFEdge::operator=")
    }

}

