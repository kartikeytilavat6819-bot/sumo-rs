//! Port of utils/router/SPTree.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `std::pair<const C *, const C *> SPTree::CHConnectionPair`
pub type SPTree_CHConnectionPair = Unmapped /* std::pair<const C *, const C *> */;

/// C++ typedef `int SPTree::CHConnectionPairs`
pub type SPTree_CHConnectionPairs = i32;

/// C++ typedef `int SPTree::CHConnections`
pub type SPTree_CHConnections = i32;

/// Class to compare (and so sort) nodes by their effort
/// C++ class `SPTree::EdgeByTTComparator` (utils/router/SPTree.h)
pub struct SPTree_EdgeByTTComparator {
}

impl SPTree_EdgeByTTComparator {
    /// C++: `bool EdgeByTTComparator::operator()(const E *, const E *) const` [const]
    /// Comparing method
    // bool operator()(const E* a, const E* b) const {
    //             if (a->traveltime == b->traveltime) {
    //                 return a->edge->getNumericalID() > b->edge->getNumericalID();
    //             }
    //             return a->traveltime > b->traveltime;
    //         }
    #[allow(unused_variables)]
    pub fn call_op(&self, a: Ptr<Unmapped /* E */>, b: Ptr<Unmapped /* E */>) -> bool {
        todo!("port EdgeByTTComparator::operator()")
    }

}

/// C++ class `SPTree` (utils/router/SPTree.h)
pub struct SPTree<E, C> {
    /// the min edge heap
    pub myFrontier: i32,
    /// the list of visited edges (used when resetting)
    pub myFound: i32,
    /// comparator for search queue
    pub myCmp: SPTree_EdgeByTTComparator,
    /// maximum search depth
    pub myMaxDepth: i32,
    /// whether permissions should be validated
    pub myValidatePermissions: bool,
    /// vector of needed shortcuts after validation
    pub myShortcutsToValidate: SPTree_CHConnectionPairs,
    /// vector of needed shortcuts after validation
    pub myNeededShortcuts: SPTree_CHConnectionPairs,
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
        todo!("port SPTree::SPTree<E, C>")
    }

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
        todo!("port SPTree::init")
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
        todo!("port SPTree::rebuildFrom")
    }

    /// C++: `bool SPTree::validatePermissions()`
    /// whether permissions should be validated;
    // inline bool validatePermissions() {
    //         return myValidatePermissions;
    //     }
    #[allow(unused_variables)]
    pub fn validate_permissions(&mut self) -> bool {
        todo!("port SPTree::validatePermissions")
    }

    /// C++: `void SPTree::registerForValidation(const C *, const C *)`
    /// save source/target pair for later validation
    // void registerForValidation(const C* aInfo, const C* fInfo) {
    //         assert(myValidatePermissions);
    //         myShortcutsToValidate.push_back(CHConnectionPair(aInfo, fInfo));
    //     }
    #[allow(unused_variables)]
    pub fn register_for_validation(&mut self, aInfo: Ptr<C>, fInfo: Ptr<C>) {
        todo!("port SPTree::registerForValidation")
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
    pub fn get_needed_shortcuts(&mut self, excluded: Ptr<E>) -> &SPTree_CHConnectionPairs {
        todo!("port SPTree::getNeededShortcuts")
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
        todo!("port SPTree::dijkstraTT")
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
    pub fn debug_print_vector(&mut self, vec: &mut i32, start: Ptr<E>, excluded: Ptr<E>) {
        todo!("port SPTree::debugPrintVector")
    }

}

