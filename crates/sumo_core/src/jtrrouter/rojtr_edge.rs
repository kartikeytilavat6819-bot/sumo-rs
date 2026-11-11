//! Port of jtrrouter/ROJTREdge.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `std::map<ROJTREdge *, ValueTimeLine<double> *, ComparatorIdLess> ROJTREdge::FollowerUsageCont`
pub type ROJTREdge_FollowerUsageCont = Unmapped /* std::map<ROJTREdge *, ValueTimeLine<double> *, ComparatorIdLess> */;

/// An edge the jtr-router may route through
/// C++ class `ROJTREdge` (jtrrouter/ROJTREdge.h)
pub struct ROJTREdge {
    /// C++ base class `ROEdge`
    pub base: ROEdge,
    /// Storage for the probabilities of using a certain follower over time
    pub myFollowingDefs: ROJTREdge_FollowerUsageCont,
    /// The defaults for turnings
    pub myParsedTurnings: Vec<f64>,
    pub mySourceFlows: i64,
}

impl std::ops::Deref for ROJTREdge {
    type Target = ROEdge;
    fn deref(&self) -> &ROEdge {
        &self.base
    }
}
impl std::ops::DerefMut for ROJTREdge {
    fn deref_mut(&mut self) -> &mut ROEdge {
        &mut self.base
    }
}

impl ROJTREdge {
    /// C++: `void ROJTREdge::ROJTREdge(const std::string &, RONode *, RONode *, int, const int, const std::string &, const std::string &)`
    /// Constructor
    // ROJTREdge::ROJTREdge(const std::string& id, RONode* from, RONode* to, int index, const int priority, const std::string& type, const std::string& routingType) :
    //     ROEdge(id, from, to, index, priority, type, routingType),
    //     mySourceFlows(0)
    // {}
    #[allow(unused_variables)]
    pub fn new(id: &str, from: Ptr<RONode>, to: Ptr<RONode>, index: i32, priority: i32, type_: &str, routingType: &str) -> Self {
        todo!("port ROJTREdge::ROJTREdge")
    }

    /// C++: `void ROJTREdge::~ROJTREdge() noexcept` [virtual]
    /// Destructor
    // ROJTREdge::~ROJTREdge() {
    //     for (FollowerUsageCont::iterator i = myFollowingDefs.begin(); i != myFollowingDefs.end(); ++i) {
    //         delete (*i).second;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port ROJTREdge::~ROJTREdge")
    }

    /// C++: `void ROJTREdge::addSuccessor(ROEdge *, ROEdge *, std::string)` [virtual]
    /// Adds information about a connected edge
    // void
    // ROJTREdge::addSuccessor(ROEdge* s, ROEdge* via, std::string dir) {
    //     ROEdge::addSuccessor(s, via, dir);
    //     ROJTREdge* js = static_cast<ROJTREdge*>(s);
    //     if (myFollowingDefs.find(js) == myFollowingDefs.end()) {
    //         myFollowingDefs[js] = new ValueTimeLine<double>();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_successor(&mut self, s: Ptr<ROEdge>, via: Ptr<ROEdge>, dir: String) {
        todo!("port ROJTREdge::addSuccessor")
    }

    /// C++: `void ROJTREdge::addFollowerProbability(ROJTREdge *, double, double, double)`
    /// adds the information about the percentage of using a certain follower
    // void
    // ROJTREdge::addFollowerProbability(ROJTREdge* follower, double begTime,
    //                                   double endTime, double probability) {
    //     FollowerUsageCont::iterator i = myFollowingDefs.find(follower);
    //     if (i == myFollowingDefs.end()) {
    //         WRITE_ERRORF(TL("The edges '%' and '%' are not connected."), getID(), follower->getID());
    //         return;
    //     }
    //     (*i).second->add(begTime, endTime, probability);
    // }
    #[allow(unused_variables)]
    pub fn add_follower_probability(&mut self, follower: Ptr<ROJTREdge>, begTime: f64, endTime: f64, probability: f64) {
        todo!("port ROJTREdge::addFollowerProbability")
    }

    /// C++: `ROJTREdge * ROJTREdge::chooseNext(const ROVehicle *const, double, const std::set<const ROEdge *> &) const` [const]
    /// Returns the next edge to use
    // ROJTREdge*
    // ROJTREdge::chooseNext(const ROVehicle* const veh, double time, const std::set<const ROEdge*>& avoid) const {
    //     // if no usable follower exist, return 0
    //     //  their probabilities are not yet regarded
    //     if (myFollowingEdges.size() == 0 || (veh != nullptr && allFollowersProhibit(veh))) {
    //         return nullptr;
    //     }
    //     // gather information about the probabilities at this time
    //     RandomDistributor<ROJTREdge*> dist;
    //     // use the loaded definitions, first
    //     for (FollowerUsageCont::const_iterator i = myFollowingDefs.begin(); i != myFollowingDefs.end(); ++i) {
    //         if (avoid.count(i->first) == 0) {
    //             if ((veh == nullptr || !(*i).first->prohibits(veh)) && (*i).second->describesTime(time)) {
    //                 dist.add((*i).first, (*i).second->getValue(time));
    //             }
    //         }
    //     }
    //     // if no loaded definitions are valid for this time, try to use the defaults
    //     if (dist.getOverallProb() == 0) {
    //         for (int i = 0; i < (int)myParsedTurnings.size(); ++i) {
    //             if (avoid.count(myFollowingEdges[i]) == 0) {
    //                 if (veh == nullptr || !myFollowingEdges[i]->prohibits(veh)) {
    //                     dist.add(static_cast<ROJTREdge*>(myFollowingEdges[i]), myParsedTurnings[i]);
    //                 }
    //             }
    //         }
    //     }
    //     // if still no valid follower exists, return null
    //     if (dist.getOverallProb() == 0) {
    //         return nullptr;
    //     }
    //     // return one of the possible followers
    //     return dist.get();
    // }
    #[allow(unused_variables)]
    pub fn choose_next(&self, veh: Ptr<ROVehicle>, time: f64, avoid: &BTreeSet<Ptr<ROEdge>>) -> Ptr<ROJTREdge> {
        todo!("port ROJTREdge::chooseNext")
    }

    /// C++: `void ROJTREdge::setTurnDefaults(const std::vector<double> &)`
    /// Sets the turning definition defaults
    // void
    // ROJTREdge::setTurnDefaults(const std::vector<double>& defs) {
    //     // I hope, we'll find a less ridiculous solution for this
    //     std::vector<double> tmp(defs.size()*myFollowingEdges.size(), 0);
    //     // store in less common multiple
    //     for (int i = 0; i < (int)defs.size(); ++i) {
    //         for (int j = 0; j < (int)myFollowingEdges.size(); ++j) {
    //             tmp[i * myFollowingEdges.size() + j] = defs[i] / 100. / (double)myFollowingEdges.size();
    //         }
    //     }
    //     // parse from less common multiple
    //     for (int i = 0; i < (int)myFollowingEdges.size(); ++i) {
    //         double value = 0;
    //         for (int j = 0; j < (int)defs.size(); ++j) {
    //             value += tmp[i * defs.size() + j];
    //         }
    //         myParsedTurnings.push_back((double) value);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_turn_defaults(&mut self, defs: &Vec<f64>) {
        todo!("port ROJTREdge::setTurnDefaults")
    }

    /// C++: `long long ROJTREdge::getSourceFlow() const` [const]
    /// register source flow on this edge
    // long long int getSourceFlow() const {
    //         return mySourceFlows;
    //     }
    #[allow(unused_variables)]
    pub fn get_source_flow(&self) -> i64 {
        todo!("port ROJTREdge::getSourceFlow")
    }

    /// C++: `void ROJTREdge::changeSourceFlow(long long)`
    /// register flow on this edge
    // void changeSourceFlow(long long int value) {
    //         mySourceFlows += value;
    //     }
    #[allow(unused_variables)]
    pub fn change_source_flow(&mut self, value: i64) {
        todo!("port ROJTREdge::changeSourceFlow")
    }

    /// C++: `void ROJTREdge::ROJTREdge(const ROJTREdge &)` [private]
    /// invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(src: &ROJTREdge) -> Self {
        todo!("port ROJTREdge::ROJTREdge")
    }

    /// C++: `ROJTREdge & ROJTREdge::operator=(const ROJTREdge &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, src: &ROJTREdge) -> &mut ROJTREdge {
        todo!("port ROJTREdge::operator=")
    }

}

