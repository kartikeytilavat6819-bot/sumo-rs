//! Port of utils/router/SPTree.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use crate::utils::router::ch_builder::{std_pop_heap, std_push_heap};
use utils::common::to_string::f64_to_string_prec;

/// C++ typedef `std::pair<const C *, const C *> SPTree::CHConnectionPair`
pub type SPTree_CHConnectionPair<C> = (Ptr<C>, Ptr<C>);

/// C++ typedef `int SPTree::CHConnectionPairs`
pub type SPTree_CHConnectionPairs<C> = Vec<SPTree_CHConnectionPair<C>>;

/// C++ typedef `int SPTree::CHConnections`
pub type SPTree_CHConnections = i32;

/// Node fields and edge accessors the SPTree reads on its element type `E` (C++ `E::traveltime`, `E::depth`,
/// `E::visited`, `E::permissions`, `E::followers`, `E::edge`, `E::reset()`).
pub trait SPTreeNode {
    type Connection: SPTreeConnection<Self>;
    fn traveltime(&self) -> f64;
    fn set_traveltime(&mut self, traveltime: f64);
    fn depth(&self) -> i32;
    fn set_depth(&mut self, depth: i32);
    fn visited(&self) -> bool;
    fn set_visited(&mut self, visited: bool);
    fn permissions(&self) -> SVCPermissions;
    fn set_permissions(&mut self, permissions: SVCPermissions);
    fn followers(&self) -> &[Self::Connection];
    /// C++ `E::edge->getNumericalID()`
    fn edge_numerical_id(&self) -> i32;
    /// C++ `E::edge->getID()`
    fn edge_id(&self) -> String;
    /// C++ `E::edge->getPermissions()`
    fn edge_permissions(&self) -> SVCPermissions;
    fn reset(&mut self);
}

/// Connection fields the SPTree reads on its connection type `C` (C++ `C::target`, `C::cost`, `C::permissions`)
pub trait SPTreeConnection<E: ?Sized> {
    fn target(&self) -> Ptr<E>;
    fn cost(&self) -> f64;
    fn permissions(&self) -> SVCPermissions;
}

/// Class to compare (and so sort) nodes by their effort
/// C++ class `SPTree::EdgeByTTComparator` (utils/router/SPTree.h)
pub struct SPTree_EdgeByTTComparator<E> {
    _p: std::marker::PhantomData<E>,
}

impl<E: SPTreeNode> SPTree_EdgeByTTComparator<E> {
    /// C++: `bool EdgeByTTComparator::operator()(const E *, const E *) const` [const]
    /// Comparing method
    // bool operator()(const E* a, const E* b) const {
    //             if (a->traveltime == b->traveltime) {
    //                 return a->edge->getNumericalID() > b->edge->getNumericalID();
    //             }
    //             return a->traveltime > b->traveltime;
    //         }
    #[allow(unused_variables)]
    pub fn call_op(&self, a: Ptr<E>, b: Ptr<E>) -> bool {
        if a.m().traveltime() == b.m().traveltime() {
            return a.m().edge_numerical_id() > b.m().edge_numerical_id();
        }
        a.m().traveltime() > b.m().traveltime()
    }

}

/// C++ class `SPTree` (utils/router/SPTree.h)
pub struct SPTree<E, C> {
    /// the min edge heap
    pub myFrontier: Vec<Ptr<E>>,
    /// the list of visited edges (used when resetting)
    pub myFound: Vec<Ptr<E>>,
    /// comparator for search queue
    pub myCmp: SPTree_EdgeByTTComparator<E>,
    /// maximum search depth
    pub myMaxDepth: i32,
    /// whether permissions should be validated
    pub myValidatePermissions: bool,
    /// vector of needed shortcuts after validation
    pub myShortcutsToValidate: SPTree_CHConnectionPairs<C>,
    /// vector of needed shortcuts after validation
    pub myNeededShortcuts: SPTree_CHConnectionPairs<C>,
    _p: std::marker::PhantomData<(E, C,)>,
}

impl<E, C> SPTree<E, C> {
    /// C++: `void SPTree::SPTree<E, C>(int, bool)`
    /// Constructor
    // SPTree(int maxDepth, bool validatePermissions) :
    //         myMaxDepth(maxDepth),
    //         myValidatePermissions(validatePermissions) {
    //     }
    #[allow(unused_variables)]
    pub fn new(maxDepth: i32, validatePermissions: bool) -> Self {
        SPTree {
            myFrontier: Vec::new(),
            myFound: Vec::new(),
            myCmp: SPTree_EdgeByTTComparator { _p: std::marker::PhantomData },
            myMaxDepth: maxDepth,
            myValidatePermissions: validatePermissions,
            myShortcutsToValidate: Vec::new(),
            myNeededShortcuts: Vec::new(),
            _p: std::marker::PhantomData,
        }
    }
}

impl<E, C> SPTree<E, C>
where
    E: SPTreeNode<Connection = C>,
    C: SPTreeConnection<E>,
{
    /// C++: `void SPTree::init()`
    // void init() {
    //         // all EdgeInfos touched in the previous query are either in myFrontier or myFound: clean those up
    //         for (typename std::vector<E*>::iterator i = myFrontier.begin(); i != myFrontier.end(); i++) {
    //             (*i)->reset();
    //         }
    //         myFrontier.clear();
    //         for (typename std::vector<E*>::iterator i = myFound.begin(); i != myFound.end(); i++) {
    //             (*i)->reset();
    //         }
    //         myFound.clear();
    //     }
    #[allow(unused_variables)]
    pub fn init(&mut self) {
        for i in self.myFrontier.iter() {
            i.m().reset();
        }
        self.myFrontier.clear();
        for i in self.myFound.iter() {
            i.m().reset();
        }
        self.myFound.clear();
    }

    /// C++: `void SPTree::rebuildFrom(E *, const E *)`
    /// build a shortest path tree from start to a depth of myMaxdepth. The given edge is excluded from this tree
    // void rebuildFrom(E* start, const E* excluded) {
    //         init();
    //         start->traveltime = 0;
    //         start->depth = 0;
    //         start->permissions = start->edge->getPermissions();
    //         myFrontier.push_back(start);
    //         // build SPT
    //         while (!myFrontier.empty()) {
    //             E* min = myFrontier.front();
    //             std::pop_heap(myFrontier.begin(), myFrontier.end(), myCmp);
    //             myFrontier.pop_back();
    //             myFound.push_back(min);
    //             min->visited = true;
    //             if (min->depth < myMaxDepth) {
    //                 for (typename CHConnections::iterator it = min->followers.begin(); it != min->followers.end(); it++) {
    //                     C& con = *it;
    //                     E* follower = con.target;
    //                     if (follower == excluded) {
    //                         continue;
    //                     }
    //                     const double traveltime = min->traveltime + con.cost;
    //                     const double oldTraveltime = follower->traveltime;
    //                     if (!follower->visited && traveltime < oldTraveltime) {
    //                         follower->traveltime = traveltime;
    //                         follower->depth = min->depth + 1;
    //                         follower->permissions = (min->permissions & con.permissions);
    //                         if (oldTraveltime == std::numeric_limits<double>::max()) {
    //                             myFrontier.push_back(follower);
    //                             std::push_heap(myFrontier.begin(), myFrontier.end(), myCmp);
    //                         } else {
    //                             std::push_heap(myFrontier.begin(),
    //                                            std::find(myFrontier.begin(), myFrontier.end(), follower) + 1,
    //                                            myCmp);
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn rebuild_from(&mut self, start: Ptr<E>, excluded: Ptr<E>) {
        self.init();
        start.m().set_traveltime(0.);
        start.m().set_depth(0);
        let startPermissions = start.m().edge_permissions();
        start.m().set_permissions(startPermissions);
        self.myFrontier.push(start);
        // build SPT
        while !self.myFrontier.is_empty() {
            let min = self.myFrontier[0];
            std_pop_heap(&mut self.myFrontier, &|a, b| self.myCmp.call_op(a, b));
            self.myFrontier.pop();
            self.myFound.push(min);
            min.m().set_visited(true);
            if min.m().depth() < self.myMaxDepth {
                for con in min.m().followers().iter() {
                    let follower = con.target();
                    if follower == excluded {
                        continue;
                    }
                    let traveltime = min.m().traveltime() + con.cost();
                    let oldTraveltime = follower.m().traveltime();
                    if !follower.m().visited() && traveltime < oldTraveltime {
                        follower.m().set_traveltime(traveltime);
                        follower.m().set_depth(min.m().depth() + 1);
                        follower.m().set_permissions(min.m().permissions() & con.permissions());
                        if oldTraveltime == f64::MAX {
                            self.myFrontier.push(follower);
                            std_push_heap(&mut self.myFrontier, &|a, b| self.myCmp.call_op(a, b));
                        } else {
                            let pos = self.myFrontier.iter().position(|x| *x == follower).expect("follower with a finite traveltime is on the frontier");
                            std_push_heap(&mut self.myFrontier[..=pos], &|a, b| self.myCmp.call_op(a, b));
                        }
                    }
                }
            }
        }
    }

    /// C++: `bool SPTree::validatePermissions()`
    /// whether permissions should be validated;
    // inline bool validatePermissions() {
    //         return myValidatePermissions;
    //     }
    #[allow(unused_variables)]
    pub fn validate_permissions(&mut self) -> bool {
        self.myValidatePermissions
    }

    /// C++: `void SPTree::registerForValidation(const C *, const C *)`
    /// save source/target pair for later validation
    // void registerForValidation(const C* aInfo, const C* fInfo) {
    //         assert(myValidatePermissions);
    //         myShortcutsToValidate.push_back(CHConnectionPair(aInfo, fInfo));
    //     }
    #[allow(unused_variables)]
    pub fn register_for_validation(&mut self, aInfo: Ptr<C>, fInfo: Ptr<C>) {
        assert!(self.myValidatePermissions);
        self.myShortcutsToValidate.push((aInfo, fInfo));
    }

    /// C++: `const CHConnectionPairs & SPTree::getNeededShortcuts(const E *)`
    // const CHConnectionPairs& getNeededShortcuts(const E* excluded) {
    //         assert(myValidatePermissions);
    //         myNeededShortcuts.clear();
    //         for (typename CHConnectionPairs::iterator it = myShortcutsToValidate.begin(); it != myShortcutsToValidate.end(); ++it) {
    //             const C* const aInfo = it->first;
    //             const C* const fInfo = it->second;
    //             const double bestWitness = dijkstraTT(
    //                                            aInfo->target, fInfo->target, excluded, (aInfo->permissions & fInfo->permissions));
    //             const double viaCost = aInfo->cost + fInfo->cost;
    //             if (viaCost < bestWitness) {
    //                 myNeededShortcuts.push_back(*it);
    //             }
    //         }
    //         myShortcutsToValidate.clear();
    //         return myNeededShortcuts;
    //     }
    #[allow(unused_variables)]
    pub fn get_needed_shortcuts(&mut self, excluded: Ptr<E>) -> &SPTree_CHConnectionPairs<C> {
        assert!(self.myValidatePermissions);
        self.myNeededShortcuts.clear();
        for it in 0..self.myShortcutsToValidate.len() {
            let (aInfo, fInfo) = self.myShortcutsToValidate[it];
            let bestWitness = self.dijkstra_tt(aInfo.m().target(), fInfo.m().target(), excluded, aInfo.m().permissions() & fInfo.m().permissions());
            let viaCost = aInfo.m().cost() + fInfo.m().cost();
            if viaCost < bestWitness {
                self.myNeededShortcuts.push((aInfo, fInfo));
            }
        }
        self.myShortcutsToValidate.clear();
        &self.myNeededShortcuts
    }

    /// C++: `double SPTree::dijkstraTT(E *, E *, const E *, SVCPermissions)` [private]
    // double dijkstraTT(E* start, E* dest, const E* excluded, SVCPermissions permissions) {
    //         init();
    //         start->traveltime = 0;
    //         start->depth = 0;
    //         myFrontier.push_back(start);
    //         // build SPT
    //         while (!myFrontier.empty()) {
    //             E* min = myFrontier.front();
    //             if (min == dest) {
    //                 return dest->traveltime;
    //             }
    //             std::pop_heap(myFrontier.begin(), myFrontier.end(), myCmp);
    //             myFrontier.pop_back();
    //             myFound.push_back(min);
    //             min->visited = true;
    //             if (min->depth < myMaxDepth) {
    //                 for (typename CHConnections::iterator it = min->followers.begin(); it != min->followers.end(); it++) {
    //                     C& con = *it;
    //                     E* follower = con.target;
    //                     if (follower == excluded) {
    //                         continue;
    //                     }
    //                     if ((con.permissions & permissions) != permissions) {
    //                         continue;
    //                     }
    //                     const double traveltime = min->traveltime + con.cost;
    //                     const double oldTraveltime = follower->traveltime;
    //                     if (!follower->visited && traveltime < oldTraveltime) {
    //                         follower->traveltime = traveltime;
    //                         follower->depth = min->depth + 1;
    //                         follower->permissions = (min->permissions & con.permissions);
    //                         if (oldTraveltime == std::numeric_limits<double>::max()) {
    //                             myFrontier.push_back(follower);
    //                             std::push_heap(myFrontier.begin(), myFrontier.end(), myCmp);
    //                         } else {
    //                             std::push_heap(myFrontier.begin(),
    //                                            std::find(myFrontier.begin(), myFrontier.end(), follower) + 1,
    //                                            myCmp);
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //         return dest->traveltime;
    //     }
    #[allow(unused_variables)]
    pub fn dijkstra_tt(&mut self, start: Ptr<E>, dest: Ptr<E>, excluded: Ptr<E>, permissions: SVCPermissions) -> f64 {
        self.init();
        start.m().set_traveltime(0.);
        start.m().set_depth(0);
        self.myFrontier.push(start);
        // build SPT
        while !self.myFrontier.is_empty() {
            let min = self.myFrontier[0];
            if min == dest {
                return dest.m().traveltime();
            }
            std_pop_heap(&mut self.myFrontier, &|a, b| self.myCmp.call_op(a, b));
            self.myFrontier.pop();
            self.myFound.push(min);
            min.m().set_visited(true);
            if min.m().depth() < self.myMaxDepth {
                for con in min.m().followers().iter() {
                    let follower = con.target();
                    if follower == excluded {
                        continue;
                    }
                    if (con.permissions() & permissions) != permissions {
                        continue;
                    }
                    let traveltime = min.m().traveltime() + con.cost();
                    let oldTraveltime = follower.m().traveltime();
                    if !follower.m().visited() && traveltime < oldTraveltime {
                        follower.m().set_traveltime(traveltime);
                        follower.m().set_depth(min.m().depth() + 1);
                        follower.m().set_permissions(min.m().permissions() & con.permissions());
                        if oldTraveltime == f64::MAX {
                            self.myFrontier.push(follower);
                            std_push_heap(&mut self.myFrontier, &|a, b| self.myCmp.call_op(a, b));
                        } else {
                            let pos = self.myFrontier.iter().position(|x| *x == follower).expect("follower with a finite traveltime is on the frontier");
                            std_push_heap(&mut self.myFrontier[..=pos], &|a, b| self.myCmp.call_op(a, b));
                        }
                    }
                }
            }
        }
        dest.m().traveltime()
    }

    /// C++: `void SPTree::debugPrintVector(int &, E *, const E *)` [private]
    // void debugPrintVector(std::vector<E*>& vec, E* start, const E* excluded) {
    //         std::cout << "computed SPT from '" << start->edge->getID() << "' (excluding " << excluded->edge->getID() <<  ") with " << myFound.size() << " edges\n";
    //         for (typename std::vector<E*>::iterator it = vec.begin(); it != vec.end(); it++) {
    //             E* e = *it;
    //             std::cout << "(" << e->edge->getID() << "," << e->traveltime << ") ";
    //         }
    //         std::cout << "\n";
    //     }
    #[allow(unused_variables)]
    pub fn debug_print_vector(&mut self, vec: &mut Vec<Ptr<E>>, start: Ptr<E>, excluded: Ptr<E>) {
        print!("computed SPT from '{}' (excluding {}) with {} edges\n", start.m().edge_id(), excluded.m().edge_id(), self.myFound.len());
        for e in vec.iter() {
            print!("({},{}) ", e.m().edge_id(), f64_to_string_prec(e.m().traveltime(), 6));
        }
        print!("\n");
    }

}
