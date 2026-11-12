//! Port of jtrrouter/ROJTREdge.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `std::map<ROJTREdge *, ValueTimeLine<double> *, ComparatorIdLess> ROJTREdge::FollowerUsageCont`
/// keyed by the follower's id (C++ ComparatorIdLess); the value keeps the follower pointer with its time line
pub type ROJTREdge_FollowerUsageCont = BTreeMap<String, (Ptr<ROJTREdge>, crate::router::ro_edge::ValueTimeLine)>;

/// An edge the jtr-router may route through
/// C++ class `ROJTREdge` (jtrrouter/ROJTREdge.h)
#[repr(C)]
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
        ROJTREdge {
            base: ROEdge::new(id, from, to, index, priority, type_, routingType),
            myFollowingDefs: BTreeMap::new(),
            myParsedTurnings: Vec::new(),
            mySourceFlows: 0,
        }
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
        // the ValueTimeLines are owned by myFollowingDefs and are freed with it
        self.myFollowingDefs.clear();
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
        self.base.add_successor(s, via, dir);
        let js = s.cast::<ROJTREdge>();
        let key = js.get_id().to_string();
        if !self.myFollowingDefs.contains_key(&key) {
            self.myFollowingDefs.insert(key, (js, crate::router::ro_edge::ValueTimeLine::default()));
        }
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
        let myId = self.base.get_id().to_string();
        match self.myFollowingDefs.get_mut(follower.get_id()) {
            Some(entry) => entry.1.add(begTime, endTime, probability),
            None => utils::common::msg_handler::write_error(&format!("The edges '{}' and '{}' are not connected.", myId, follower.get_id())),
        }
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
        // if no usable follower exist, return 0
        //  their probabilities are not yet regarded
        if self.myFollowingEdges.len() == 0 || (!veh.is_null() && self.all_followers_prohibit(veh)) {
            return Ptr::null();
        }
        // gather information about the probabilities at this time
        let mut dist: RandomDistributor<Ptr<ROJTREdge>> = RandomDistributor::default();
        // use the loaded definitions, first
        for (_, (follower, tl)) in self.myFollowingDefs.iter() {
            if !avoid.contains(&follower.cast::<ROEdge>()) {
                if (veh.is_null() || !follower.prohibits(veh, true)) && tl.describes_time(time) {
                    dist.add(*follower, tl.get_value(time), false);
                }
            }
        }
        // if no loaded definitions are valid for this time, try to use the defaults
        if dist.get_overall_prob() == 0.0 {
            for i in 0..self.myParsedTurnings.len() {
                if !avoid.contains(&self.myFollowingEdges[i]) {
                    if veh.is_null() || !self.myFollowingEdges[i].prohibits(veh, true) {
                        dist.add(self.myFollowingEdges[i].cast::<ROJTREdge>(), self.myParsedTurnings[i], false);
                    }
                }
            }
        }
        // if still no valid follower exists, return null
        if dist.get_overall_prob() == 0.0 {
            return Ptr::null();
        }
        // return one of the possible followers
        dist.get(None).unwrap()
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
        // I hope, we'll find a less ridiculous solution for this
        let nEdges = self.myFollowingEdges.len();
        let mut tmp: Vec<f64> = vec![0.0; defs.len() * nEdges];
        // store in less common multiple
        for i in 0..defs.len() {
            for j in 0..nEdges {
                tmp[i * nEdges + j] = defs[i] / 100.0 / (nEdges as f64);
            }
        }
        // parse from less common multiple
        for i in 0..nEdges {
            let mut value = 0.0;
            for j in 0..defs.len() {
                value += tmp[i * defs.len() + j];
            }
            self.myParsedTurnings.push(value as f64);
        }
    }

    /// C++: `long long ROJTREdge::getSourceFlow() const` [const]
    /// register source flow on this edge
    // long long int getSourceFlow() const {
    //         return mySourceFlows;
    //     }
    #[allow(unused_variables)]
    pub fn get_source_flow(&self) -> i64 {
        self.mySourceFlows
    }

    /// C++: `void ROJTREdge::changeSourceFlow(long long)`
    /// register flow on this edge
    // void changeSourceFlow(long long int value) {
    //         mySourceFlows += value;
    //     }
    #[allow(unused_variables)]
    pub fn change_source_flow(&mut self, value: i64) {
        self.mySourceFlows += value;
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

