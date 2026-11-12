//! Port of netimport/vissim/tempstructs/NIVissimEdge.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `std::vector<NIVissimConnectionCluster *> NIVissimEdge::ConnectionClusters`
pub type NIVissimEdge_ConnectionClusters = Vec<Ptr<NIVissimConnectionCluster>>;

/// C++ typedef `std::map<int, NIVissimEdge *> NIVissimEdge::DictType`
pub type NIVissimEdge_DictType = BTreeMap<i32, Ptr<NIVissimEdge>>;

/// A temporary storage for edges imported from Vissim
/// C++ class `NIVissimEdge` (netimport/vissim/tempstructs/NIVissimEdge.h)
#[repr(C)]
pub struct NIVissimEdge {
    /// C++ base class `NIVissimAbstractEdge`
    pub base: NIVissimAbstractEdge,
    /// The name of the edge
    pub myName: String,
    /// The type of the edge
    pub myType: String,
    /// The number of lanes the edge has
    pub myNoLanes: i32,
    pub myLaneWidths: Vec<f64>,
    /// Additional load values for this edge
    pub myZuschlag1: f64,
    /// Additional load values for this edge
    pub myZuschlag2: f64,
    /// List of lanes closed on this edge
    pub myClosedLanes: NIVissimClosedLanesVector,
    /// List of connection clusters along this edge
    pub myConnectionClusters: NIVissimEdge_ConnectionClusters,
    /// List of connections incoming to this edge
    pub myIncomingConnections: Vec<i32>,
    /// List of connections outgoing from this edge
    pub myOutgoingConnections: Vec<i32>,
    pub myDistrictConnections: Vec<f64>,
    pub myPatchedSpeeds: Vec<i32>,
    pub myLaneSpeeds: Vec<f64>,
    pub myToTreatAsSame: Vec<Ptr<NIVissimEdge>>,
    /// Information whether this edge was not build due to being within a junction
    pub myAmWithinJunction: bool,
}

impl std::ops::Deref for NIVissimEdge {
    type Target = NIVissimAbstractEdge;
    fn deref(&self) -> &NIVissimAbstractEdge {
        &self.base
    }
}
impl std::ops::DerefMut for NIVissimEdge {
    fn deref_mut(&mut self) -> &mut NIVissimAbstractEdge {
        &mut self.base
    }
}

// C++ static members of NIVissimEdge (port as module-level statics, see AGENTS.md):
//   static DictType NIVissimEdge::myDict;
//   static int NIVissimEdge::myMaxID;
//   static std::vector<std::string> NIVissimEdge::myLanesWithMissingSpeeds;
thread_local! {
    static MY_DICT: std::cell::RefCell<NIVissimEdge_DictType> = std::cell::RefCell::new(BTreeMap::new());
    static MY_MAX_ID: std::cell::Cell<i32> = const { std::cell::Cell::new(0) };
    static MY_LANES_WITH_MISSING_SPEEDS: std::cell::RefCell<Vec<String>> = const { std::cell::RefCell::new(Vec::new()) };
}

impl NIVissimEdge {
    /// C++: `void NIVissimEdge::NIVissimEdge(int, const std::string &, const std::string &, std::vector<double>, double, double, double, const PositionVector &, const NIVissimClosedLanesVector &)`
    /// Constructor
    // NIVissimEdge::NIVissimEdge(int id, const std::string& name,
    //                            const std::string& type,
    //                            std::vector<double> laneWidths,
    //                            double zuschlag1, double zuschlag2,
    //                            double /*length*/, const PositionVector& geom,
    //                            const NIVissimClosedLanesVector& clv) :
    //     NIVissimAbstractEdge(id, geom),
    //     myName(name), myType(type), myNoLanes((int)laneWidths.size()),
    //     myLaneWidths(laneWidths),
    //     myZuschlag1(zuschlag1), myZuschlag2(zuschlag2),
    //     myClosedLanes(clv),
    //     myLaneSpeeds(myNoLanes, -1),
    //     myAmWithinJunction(false)
    //     //, mySpeed(-1)
    // {
    //     assert(myNoLanes >= 0);
    //     if (myMaxID < myID) {
    //         myMaxID = myID;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn new(id: i32, name: &str, type_: &str, laneWidths: Vec<f64>, zuschlag1: f64, zuschlag2: f64, length: f64, geom: &PositionVector, clv: &NIVissimClosedLanesVector) -> Box<Self> {
        // the base constructor registers `this` in its dictionary, so the object must stay at a fixed address: return the Box
        let myNoLanes = laneWidths.len() as i32;
        let b = Box::new(Self {
            base: NIVissimAbstractEdge::new(id, geom),
            myName: name.to_string(),
            myType: type_.to_string(),
            myNoLanes,
            myLaneWidths: laneWidths,
            myZuschlag1: zuschlag1,
            myZuschlag2: zuschlag2,
            myClosedLanes: clv.clone(),
            myConnectionClusters: Vec::new(),
            myIncomingConnections: Vec::new(),
            myOutgoingConnections: Vec::new(),
            myDistrictConnections: Vec::new(),
            myPatchedSpeeds: Vec::new(),
            myLaneSpeeds: vec![-1.0; myNoLanes as usize],
            myToTreatAsSame: Vec::new(),
            myAmWithinJunction: false,
        });
        MY_MAX_ID.with(|m| {
            if m.get() < id {
                m.set(id);
            }
        });
        b
    }

    /// C++: `void NIVissimEdge::~NIVissimEdge() noexcept` [virtual]
    /// Destructor
    // NIVissimEdge::~NIVissimEdge() {
    //     for (NIVissimClosedLanesVector::iterator i = myClosedLanes.begin(); i != myClosedLanes.end(); i++) {
    //         delete (*i);
    //     }
    //     myClosedLanes.clear();
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        for c in self.myClosedLanes.drain(..) {
            // SAFETY: each NIVissimClosedLaneDef was allocated by its creator and is owned by this edge only
            drop(unsafe { Box::from_raw(c.as_raw()) });
        }
    }

    /// C++: `void NIVissimEdge::setNodeCluster(int)` [virtual]
    // void
    // NIVissimEdge::setNodeCluster(int nodeid) {
    //     myNode = nodeid;
    // }
    #[allow(unused_variables)]
    pub fn set_node_cluster(&mut self, nodeid: i32) {
        self.myNode = nodeid;
    }

    /// C++: `void NIVissimEdge::buildGeom()` [virtual]
    // void
    // NIVissimEdge::buildGeom() {}
    #[allow(unused_variables)]
    pub fn build_geom(&mut self) {}

    /// C++: `void NIVissimEdge::addIncomingConnection(int)`
    /// Adds a connection where this edge is the destination
    // void
    // NIVissimEdge::addIncomingConnection(int id) {
    //     myIncomingConnections.push_back(id);
    // }
    #[allow(unused_variables)]
    pub fn add_incoming_connection(&mut self, id: i32) {
        self.myIncomingConnections.push(id);
    }

    /// C++: `void NIVissimEdge::addOutgoingConnection(int)`
    /// Adds a connection where this edge is the source
    // void
    // NIVissimEdge::addOutgoingConnection(int id) {
    //     myOutgoingConnections.push_back(id);
    // }
    #[allow(unused_variables)]
    pub fn add_outgoing_connection(&mut self, id: i32) {
        self.myOutgoingConnections.push(id);
    }

    /// C++: `Position NIVissimEdge::getBegin2D() const` [const]
    /// Returns the begin position of the edge
    // Position // !!! reference?
    // NIVissimEdge::getBegin2D() const {
    //     return myGeom[0];
    // }
    #[allow(unused_variables)]
    pub fn get_begin2_d(&self) -> Position {
        *self.myGeom.at(0)
    }

    /// C++: `Position NIVissimEdge::getEnd2D() const` [const]
    /// Returns the end position of the edge
    // Position // !!! reference?
    // NIVissimEdge::getEnd2D() const {
    //     return myGeom[-1];
    // }
    #[allow(unused_variables)]
    pub fn get_end2_d(&self) -> Position {
        *self.myGeom.at(-1)
    }

    /// C++: `double NIVissimEdge::getLength() const` [const]
    /// Returns the length of the node
    // double
    // NIVissimEdge::getLength() const {
    //     return myGeom.length();
    // }
    #[allow(unused_variables)]
    pub fn get_length(&self) -> f64 {
        self.myGeom.length()
    }

    /// C++: `void NIVissimEdge::checkDistrictConnectionExistanceAt(double)`
    // void
    // NIVissimEdge::checkDistrictConnectionExistanceAt(double pos) {
    //     if (find(myDistrictConnections.begin(), myDistrictConnections.end(), pos) == myDistrictConnections.end()) {
    //         myDistrictConnections.push_back(pos);
    //         /*        int id = NIVissimConnection::getMaxID() + 1;
    //                 std::vector<int> currentCluster;
    //                 currentCluster.push_back(id);
    //                 myConnectionClusters.push_back(
    //                     new NIVissimConnectionCluster(currentCluster, -1, myID));*/
    //     }
    // }
    #[allow(unused_variables)]
    pub fn check_district_connection_existance_at(&mut self, pos: f64) {
        if !self.myDistrictConnections.contains(&pos) {
            self.myDistrictConnections.push(pos);
        }
    }

    /// C++: `void NIVissimEdge::mergedInto(NIVissimConnectionCluster *, NIVissimConnectionCluster *)`
    // void
    // NIVissimEdge::mergedInto(NIVissimConnectionCluster* old,
    //                          NIVissimConnectionCluster* act) {
    //     ConnectionClusters::iterator i =
    //         std::find(myConnectionClusters.begin(), myConnectionClusters.end(), old);
    //     if (i != myConnectionClusters.end()) {
    //         myConnectionClusters.erase(i);
    //     }
    //     i = std::find(myConnectionClusters.begin(), myConnectionClusters.end(), act);
    //     if (i == myConnectionClusters.end()) {
    //         myConnectionClusters.push_back(act);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn merged_into(&mut self, old: Ptr<NIVissimConnectionCluster>, act: Ptr<NIVissimConnectionCluster>) {
        if let Some(i) = self.myConnectionClusters.iter().position(|c| *c == old) {
            self.myConnectionClusters.remove(i);
        }
        if !self.myConnectionClusters.iter().any(|c| *c == act) {
            self.myConnectionClusters.push(act);
        }
    }

    /// C++: `void NIVissimEdge::removeFromConnectionCluster(NIVissimConnectionCluster *)`
    // void
    // NIVissimEdge::removeFromConnectionCluster(NIVissimConnectionCluster* c) {
    //     ConnectionClusters::iterator i =
    //         std::find(myConnectionClusters.begin(), myConnectionClusters.end(), c);
    //     assert(i != myConnectionClusters.end());
    //     myConnectionClusters.erase(i);
    // }
    #[allow(unused_variables)]
    pub fn remove_from_connection_cluster(&mut self, c: Ptr<NIVissimConnectionCluster>) {
        let i = self.myConnectionClusters.iter().position(|x| *x == c);
        debug_assert!(i.is_some());
        if let Some(i) = i {
            self.myConnectionClusters.remove(i);
        }
    }

    /// C++: `void NIVissimEdge::addToConnectionCluster(NIVissimConnectionCluster *)`
    // void
    // NIVissimEdge::addToConnectionCluster(NIVissimConnectionCluster* c) {
    //     ConnectionClusters::iterator i =
    //         std::find(myConnectionClusters.begin(), myConnectionClusters.end(), c);
    //     if (i == myConnectionClusters.end()) {
    //         myConnectionClusters.push_back(c);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_to_connection_cluster(&mut self, c: Ptr<NIVissimConnectionCluster>) {
        if !self.myConnectionClusters.iter().any(|x| *x == c) {
            self.myConnectionClusters.push(c);
        }
    }

    /// C++: `void NIVissimEdge::setSpeed(int, int)`
    // void
    // NIVissimEdge::setSpeed(int lane, int speedDist) {
    //     while ((int)myPatchedSpeeds.size() <= lane) {
    //         myPatchedSpeeds.push_back(-1);
    //     }
    //     myPatchedSpeeds[lane] = speedDist;
    // }
    #[allow(unused_variables)]
    pub fn set_speed(&mut self, lane: i32, speedDist: i32) {
        while (self.myPatchedSpeeds.len() as i32) <= lane {
            self.myPatchedSpeeds.push(-1);
        }
        self.myPatchedSpeeds[lane as usize] = speedDist;
    }

    /// C++: `bool NIVissimEdge::addToTreatAsSame(NIVissimEdge *)`
    // bool
    // NIVissimEdge::addToTreatAsSame(NIVissimEdge* e) {
    //     if (e == this) {
    //         return false;
    //     }
    //     // check whether this edge already knows about the other
    //     if (find(myToTreatAsSame.begin(), myToTreatAsSame.end(), e) == myToTreatAsSame.end()) {
    //         myToTreatAsSame.push_back(e);
    //         return true;
    //     } else {
    //         return false; // !!! check this
    //     }
    //     /*
    //     //
    //     std::vector<NIVissimEdge*>::iterator i;
    //     // add to all other that shall be treated as same
    //     bool changed = true;
    //     while (changed) {
    //         changed = false;
    //         for (i = myToTreatAsSame.begin(); !changed && i != myToTreatAsSame.end(); i++) {
    //             changed |= (*i)->addToTreatAsSame(e);
    //         }
    //         for (i = myToTreatAsSame.begin(); !changed && i != myToTreatAsSame.end(); i++) {
    //             changed |= e->addToTreatAsSame(*i);
    //         }
    //     }
    //     */
    // }
    #[allow(unused_variables)]
    pub fn add_to_treat_as_same(&mut self, e: Ptr<NIVissimEdge>) -> bool {
        if e.as_raw() == (self as *mut NIVissimEdge) {
            return false;
        }
        // check whether this edge already knows about the other
        if !self.myToTreatAsSame.iter().any(|x| *x == e) {
            self.myToTreatAsSame.push(e);
            true
        } else {
            false // !!! check this
        }
    }

    /// C++: `NIVissimConnection * NIVissimEdge::getConnectionTo(NIVissimEdge *)`
    // NIVissimConnection*
    // NIVissimEdge::getConnectionTo(NIVissimEdge* e) {
    //     std::vector<int>::iterator i;
    //     for (i = myIncomingConnections.begin(); i != myIncomingConnections.end(); i++) {
    //         NIVissimConnection* c = NIVissimConnection::dictionary(*i);
    //         if (c->getFromEdgeID() == e->getID()) {
    //             return c;
    //         }
    //     }
    //     for (i = myOutgoingConnections.begin(); i != myOutgoingConnections.end(); i++) {
    //         NIVissimConnection* c = NIVissimConnection::dictionary(*i);
    //         if (c->getToEdgeID() == e->getID()) {
    //             return c;
    //         }
    //     }
    //     return nullptr;
    // }
    #[allow(unused_variables)]
    pub fn get_connection_to(&mut self, e: Ptr<NIVissimEdge>) -> Ptr<NIVissimConnection> {
        for &id in self.myIncomingConnections.iter() {
            let c = NIVissimConnection::dictionary_2(id);
            if c.get_from_edge_id() == e.get_id() {
                return c;
            }
        }
        for &id in self.myOutgoingConnections.iter() {
            let c = NIVissimConnection::dictionary_2(id);
            if c.get_to_edge_id() == e.get_id() {
                return c;
            }
        }
        Ptr::null()
    }

    /// C++: `const std::vector<NIVissimEdge *> & NIVissimEdge::getToTreatAsSame() const` [const]
    // const std::vector<NIVissimEdge*>&
    // NIVissimEdge::getToTreatAsSame() const {
    //     return myToTreatAsSame;
    // }
    #[allow(unused_variables)]
    pub fn get_to_treat_as_same(&self) -> &Vec<Ptr<NIVissimEdge>> {
        &self.myToTreatAsSame
    }

    /// C++: `bool NIVissimEdge::wasWithinAJunction() const` [const]
    /// Returns whether this edge was found to be within a junction
    // bool wasWithinAJunction() const {
    //         return myAmWithinJunction;
    //     }
    #[allow(unused_variables)]
    pub fn was_within_a_junction(&self) -> bool {
        self.myAmWithinJunction
    }

    /// C++: `NIVissimEdge * NIVissimEdge::getBestIncoming() const` [const]
    // NIVissimEdge*
    // NIVissimEdge::getBestIncoming() const {
    //     // @todo seems as this would have been a hard hack!
    //     /*
    //     for (std::vector<int>::const_iterator i = myIncomingConnections.begin(); i != myIncomingConnections.end(); ++i) {
    //         NIVissimConnection* c = NIVissimConnection::dictionary(*i);
    //         return NIVissimEdge::dictionary(c->getFromEdgeID());
    //     }
    //     return 0;
    //     */
    //     if (myIncomingConnections.size() != 0) {
    //         return NIVissimEdge::dictionary(NIVissimConnection::dictionary(myIncomingConnections.front())->getFromEdgeID());
    //     }
    //     return nullptr;
    // }
    #[allow(unused_variables)]
    pub fn get_best_incoming(&self) -> Ptr<NIVissimEdge> {
        if !self.myIncomingConnections.is_empty() {
            let c = NIVissimConnection::dictionary_2(self.myIncomingConnections[0]);
            return Self::dictionary_3(c.get_from_edge_id());
        }
        Ptr::null()
    }

    /// C++: `NIVissimEdge * NIVissimEdge::getBestOutgoing() const` [const]
    // NIVissimEdge*
    // NIVissimEdge::getBestOutgoing() const {
    //     // @todo seems as this would have been a hard hack!
    //     /*
    //     for (std::vector<int>::const_iterator i = myOutgoingConnections.begin(); i != myOutgoingConnections.end(); ++i) {
    //         NIVissimConnection* c = NIVissimConnection::dictionary(*i);
    //         return NIVissimEdge::dictionary(c->getToEdgeID());
    //     }
    //     return 0;
    //     */
    //     if (myOutgoingConnections.size() != 0) {
    //         return NIVissimEdge::dictionary(NIVissimConnection::dictionary(myOutgoingConnections.front())->getToEdgeID());
    //     }
    //     return nullptr;
    // }
    #[allow(unused_variables)]
    pub fn get_best_outgoing(&self) -> Ptr<NIVissimEdge> {
        if !self.myOutgoingConnections.is_empty() {
            let c = NIVissimConnection::dictionary_2(self.myOutgoingConnections[0]);
            return Self::dictionary_3(c.get_to_edge_id());
        }
        Ptr::null()
    }

    /// C++: `bool NIVissimEdge::dictionary(int, const std::string &, const std::string &, int, double, double, double, const PositionVector &, const NIVissimClosedLanesVector &)` [static]
    /// Adds the described item to the dictionary Builds the edge first
    // bool
    // NIVissimEdge::dictionary(int id, const std::string& name,
    //                          const std::string& type, int noLanes,
    //                          double zuschlag1, double zuschlag2, double length,
    //                          const PositionVector& geom,
    //                          const NIVissimClosedLanesVector& clv) {
    //     NIVissimEdge* o = new NIVissimEdge(id, name, type, std::vector<double>(noLanes, NBEdge::UNSPECIFIED_WIDTH),
    //                                        zuschlag1, zuschlag2, length, geom, clv);
    //     if (!dictionary(id, o)) {
    //         delete o;
    //         return false;
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn dictionary(id: i32, name: &str, type_: &str, noLanes: i32, zuschlag1: f64, zuschlag2: f64, length: f64, geom: &PositionVector, clv: &NIVissimClosedLanesVector) -> bool {
        let mut o = NIVissimEdge::new(id, name, type_, vec![crate::netbuild::nb_edge::UNSPECIFIED_WIDTH; noLanes as usize], zuschlag1, zuschlag2, length, geom, clv);
        if !Self::dictionary_2(id, Ptr::from_mut(&mut *o)) {
            return false; // o is dropped here, like C++ `delete o`
        }
        // shortcut: the dictionary owns the edge for the rest of the run (C++ deletes it in clearDict, not yet ported), so leak the Box
        Box::leak(o);
        true
    }

    /// C++: `bool NIVissimEdge::dictionary(int, NIVissimEdge *)` [static]
    /// Adds the edge to the dictionary
    // bool
    // NIVissimEdge::dictionary(int id, NIVissimEdge* o) {
    //     DictType::iterator i = myDict.find(id);
    //     if (i == myDict.end()) {
    //         myDict[id] = o;
    //         return true;
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn dictionary_2(id: i32, o: Ptr<NIVissimEdge>) -> bool {
        MY_DICT.with(|d| {
            let mut d = d.borrow_mut();
            if let std::collections::btree_map::Entry::Vacant(e) = d.entry(id) {
                e.insert(o);
                true
            } else {
                false
            }
        })
    }

    /// C++: `NIVissimEdge * NIVissimEdge::dictionary(int)` [static]
    /// Returns the named edge from the dictionary
    // NIVissimEdge*
    // NIVissimEdge::dictionary(int id) {
    //     DictType::iterator i = myDict.find(id);
    //     if (i == myDict.end()) {
    //         return nullptr;
    //     }
    //     return (*i).second;
    // }
    #[allow(unused_variables)]
    pub fn dictionary_3(id: i32) -> Ptr<NIVissimEdge> {
        MY_DICT.with(|d| d.borrow().get(&id).copied().unwrap_or_else(Ptr::null))
    }

    /// C++: `void NIVissimEdge::buildConnectionClusters()` [static]
    /// Clusters connections of each edge
    // void
    // NIVissimEdge::buildConnectionClusters() {
    //     const double MAX_CLUSTER_DISTANCE = 10;
    //     // build clusters for all edges made up from not previously assigne
    //     //  connections
    //     for (DictType::iterator i = myDict.begin(); i != myDict.end(); i++) {
    //         int edgeid = (*i).first;
    //         NIVissimEdge* edge = (*i).second;
    //         // get all connectors using this edge
    //         std::vector<int> connectors = edge->myIncomingConnections;
    //         copy(edge->myOutgoingConnections.begin(), edge->myOutgoingConnections.end(), back_inserter(connectors));
    //         if (connectors.size() == 0) {
    //             continue;
    //         }
    //         // sort the connectors by the place on the edge
    //         sort(connectors.begin(), connectors.end(), connection_position_sorter(edgeid));
    //         // try to cluster the connections participating within the current edge
    //         std::vector<int> currentCluster;
    //         std::vector<int>::iterator j = connectors.begin();
    //         bool outgoing = NIVissimConnection::dictionary(*j)->getFromEdgeID() == (*i).first;
    //         double position = outgoing
    //                           ? NIVissimConnection::dictionary(*j)->getFromPosition()
    //                           : NIVissimConnection::dictionary(*j)->getToPosition();
    //
    //         // skip connections already in a cluster
    //         // !!! (?)
    //         while (j != connectors.end() && NIVissimConnection::dictionary(*j)->hasNodeCluster()) {
    //             ++j;
    //         }
    //         if (j == connectors.end()) {
    //             continue;
    //         }
    //         currentCluster.push_back(*j);
    //         do {
    //             if (j + 1 != connectors.end() && !NIVissimConnection::dictionary(*j)->hasNodeCluster()) {
    //                 bool n_outgoing = NIVissimConnection::dictionary(*(j + 1))->getFromEdgeID() == edgeid;
    //                 double n_position = n_outgoing
    //                                     ? NIVissimConnection::dictionary(*(j + 1))->getFromPosition()
    //                                     : NIVissimConnection::dictionary(*(j + 1))->getToPosition();
    //                 if (n_outgoing == outgoing && fabs(n_position - position) < MAX_CLUSTER_DISTANCE) {
    //                     // ok, in same cluster as prior
    //                     currentCluster.push_back(*(j + 1));
    //                 } else {
    //                     // start new cluster
    //                     VectorHelper<int>::removeDouble(currentCluster);
    //                     edge->myConnectionClusters.push_back(new NIVissimConnectionCluster(currentCluster, -1, edgeid));
    //                     currentCluster.clear();
    //                     currentCluster.push_back(*(j + 1));
    //                 }
    //                 outgoing = n_outgoing;
    //                 position = n_position;
    //             }
    //             j++;
    //         } while (j != connectors.end());
    //         // add last connection
    //         if (currentCluster.size() > 0) {
    //             VectorHelper<int>::removeDouble(currentCluster);
    //             edge->myConnectionClusters.push_back(new NIVissimConnectionCluster(currentCluster, -1, edgeid));
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn build_connection_clusters() {
        todo!("port NIVissimEdge::buildConnectionClusters")
    }

    /// C++: `void NIVissimEdge::dict_buildNBEdges(NBDistrictCont &, NBNodeCont &, NBEdgeCont &, double)` [static]
    /// Builds NBEdges from the VissimEdges within the dictionary
    // void
    // NIVissimEdge::dict_buildNBEdges(NBDistrictCont& dc, NBNodeCont& nc,
    //                                 NBEdgeCont& ec, double offset) {
    //     for (DictType::iterator i = myDict.begin(); i != myDict.end(); i++) {
    //         NIVissimEdge* edge = (*i).second;
    //         edge->buildNBEdge(dc, nc, ec, offset);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn dict_build_nb_edges(dc: &mut NBDistrictCont, nc: &mut NBNodeCont, ec: &mut NBEdgeCont, offset: f64) {
        todo!("port NIVissimEdge::dict_buildNBEdges")
    }

    /// C++: `void NIVissimEdge::dict_propagateSpeeds()` [static]
    // void
    // NIVissimEdge::dict_propagateSpeeds() {
    //     DictType::iterator i;
    //     for (i = myDict.begin(); i != myDict.end(); i++) {
    //         NIVissimEdge* edge = (*i).second;
    //         edge->setDistrictSpeed();
    //     }
    //     for (i = myDict.begin(); i != myDict.end(); i++) {
    //         NIVissimEdge* edge = (*i).second;
    //         edge->propagateSpeed(-1, std::vector<int>());
    //     }
    //     for (int j = 0; j < 3; j++) {
    //         for (i = myDict.begin(); i != myDict.end(); i++) {
    //             NIVissimEdge* edge = (*i).second;
    //             edge->propagateOwn();
    //         }
    //         for (i = myDict.begin(); i != myDict.end(); i++) {
    //             NIVissimEdge* edge = (*i).second;
    //             edge->checkUnconnectedLaneSpeeds();
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn dict_propagate_speeds() {
        todo!("port NIVissimEdge::dict_propagateSpeeds")
    }

    /// C++: `void NIVissimEdge::dict_checkEdges2Join()` [static]
    // void
    // NIVissimEdge::dict_checkEdges2Join() {
    //     // go through the edges
    //     for (DictType::iterator i1 = myDict.begin(); i1 != myDict.end(); i1++) {
    //         // retrieve needed values from the first edge
    //         NIVissimEdge* e1 = (*i1).second;
    //         const PositionVector& g1 = e1->getGeometry();
    //         // check all other edges
    //         DictType::iterator i2 = i1;
    //         i2++;
    //         for (; i2 != myDict.end(); i2++) {
    //             // retrieve needed values from the second edge
    //             NIVissimEdge* e2 = (*i2).second;
    //             const PositionVector& g2 = e2->getGeometry();
    //             // get the connection description
    //             NIVissimConnection* c = e1->getConnectionTo(e2);
    //             if (c == nullptr) {
    //                 c = e2->getConnectionTo(e1);
    //             }
    //             // the edge must not be a direct contiuation of the other
    //             if (c != nullptr) {
    //                 if ((c->getFromEdgeID() == e1->getID() && fabs(c->getFromPosition() - e1->getGeometry().length()) < 5)
    //                         ||
    //                         (c->getFromEdgeID() == e2->getID() && fabs(c->getFromPosition() - e2->getGeometry().length()) < 5)) {
    //
    //                     continue;
    //                 }
    //             }
    //             // only parallel edges which do end at the same node
    //             //  should be joined
    //             // check for parallelity
    //             //  !!! the usage of an explicit value is not very fine
    //             if (fabs(GeomHelper::angleDiff(g1.beginEndAngle(), g2.beginEndAngle())) > DEG2RAD(2.0)) {
    //                 // continue if the lines are not parallel
    //                 continue;
    //             }
    //
    //             // check whether the same node is approached
    //             //  (the distance between the ends should not be too large)
    //             //  !!! the usage of an explicit value is not very fine
    //             if (g1.back().distanceTo(g2.back()) > 10) {
    //                 // continue if the lines do not end at the same length
    //                 continue;
    //             }
    //             // ok, seem to be different lanes for the same edge
    //             //  mark as possibly joined later
    //             e1->addToTreatAsSame(e2);
    //             e2->addToTreatAsSame(e1);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn dict_check_edges2_join() {
        todo!("port NIVissimEdge::dict_checkEdges2Join")
    }

    /// C++: `void NIVissimEdge::reportUnsetSpeeds()` [static]
    /// Writes edges with unset speeds to the warnings message log instance
    // void
    // NIVissimEdge::reportUnsetSpeeds() {
    //     if (myLanesWithMissingSpeeds.size() == 0) {
    //         return;
    //     }
    //     std::ostringstream str;
    //     str << "The following lanes have no explicit speed information:\n  ";
    //     for (std::vector<std::string>::iterator i = myLanesWithMissingSpeeds.begin(); i != myLanesWithMissingSpeeds.end(); ++i) {
    //         if (i != myLanesWithMissingSpeeds.begin()) {
    //             str << ", ";
    //         }
    //         str << *i;
    //     }
    //     WRITE_WARNING(str.str());
    // }
    #[allow(unused_variables)]
    pub fn report_unset_speeds() {
        todo!("port NIVissimEdge::reportUnsetSpeeds")
    }

    /// C++: `void NIVissimEdge::buildNBEdge(NBDistrictCont &, NBNodeCont &, NBEdgeCont &, double)` [private]
    /// Builds the NBEdge from this VissimEdge
    // void
    // NIVissimEdge::buildNBEdge(NBDistrictCont& dc, NBNodeCont& nc, NBEdgeCont& ec,
    //                           double sameNodesOffset) {
    //     // build the edge
    //     std::pair<NIVissimConnectionCluster*, NBNode*> fromInf, toInf;
    //     NBNode* fromNode, *toNode;
    //     fromNode = toNode = nullptr;
    //     sort(myConnectionClusters.begin(), myConnectionClusters.end(), connection_cluster_position_sorter(myID));
    //     sort(myDistrictConnections.begin(), myDistrictConnections.end());
    //     ConnectionClusters tmpClusters = myConnectionClusters;
    //     if (tmpClusters.size() != 0) {
    //         sort(tmpClusters.begin(), tmpClusters.end(), connection_cluster_position_sorter(myID));
    //         // get or build the from-node
    //         //  A node may have to be build when the edge starts or ends at
    //         //  a parking place or something like this
    //         fromInf = getFromNode(nc, tmpClusters);
    //         fromNode = fromInf.second;
    //         // get or build the to-node
    //         //if(tmpClusters.size()>0) {
    //         toInf = getToNode(nc, tmpClusters);
    //         toNode = toInf.second;
    //         if (fromInf.first != 0 && toNode != nullptr && fromInf.first->around(toNode->getPosition())) {
    //             WRITE_WARNINGF(TL("Will not build edge '%'."), toString(myID));
    //             myAmWithinJunction = true;
    //             return;
    //         }
    //         //}
    //         // if both nodes are the same, resolve the problem otherwise
    //         if (fromNode == toNode) {
    //             std::pair<NBNode*, NBNode*> tmp = resolveSameNode(nc, sameNodesOffset, fromNode, toNode);
    //             if (fromNode != tmp.first) {
    //                 fromInf.first = 0;
    //             }
    //             if (toNode != tmp.second) {
    //                 toInf.first = 0;
    //             }
    //             fromNode = tmp.first;
    //             toNode = tmp.second;
    //         }
    //     }
    //
    //     //
    //     if (fromNode == nullptr) {
    //         fromInf.first = 0;
    //         Position pos = myGeom[0];
    //         fromNode = new NBNode(toString<int>(myID) + "-SourceNode", pos, SumoXMLNodeType::NOJUNCTION);
    //         if (!nc.insert(fromNode)) {
    //             throw ProcessError(TLF("Could not insert node '%' to nodes container.", fromNode->getID()));
    //         }
    //     }
    //     if (toNode == nullptr) {
    //         toInf.first = 0;
    //         Position pos = myGeom[-1];
    //         toNode = new NBNode(toString<int>(myID) + "-DestinationNode", pos, SumoXMLNodeType::NOJUNCTION);
    //         if (!nc.insert(toNode)) {
    //             throw ProcessError(TLF("Could not insert node '%' to nodes container.", toNode->getID()));
    //         }
    //     }
    //
    //     // build the edge
    //     double avgSpeed = 0;
    //     for (int i = 0; i < myNoLanes; i++) {
    //         if ((int)myLaneSpeeds.size() <= i || myLaneSpeeds[i] == -1) {
    //             myLanesWithMissingSpeeds.push_back(toString(myID) + "_" + toString(i));
    //             avgSpeed += OptionsCont::getOptions().getFloat("vissim.default-speed");
    //         } else {
    //             avgSpeed += myLaneSpeeds[i];
    //         }
    //     }
    //     avgSpeed /= (double) myLaneSpeeds.size();
    //     avgSpeed *= OptionsCont::getOptions().getFloat("vissim.speed-norm");
    //
    //     if (fromNode == toNode) {
    //         WRITE_WARNINGF(TL("Could not build edge '%'; would connect same node."), toString(myID));
    //         return;
    //     }
    //
    //     NBEdge* buildEdge = new NBEdge(toString<int>(myID), fromNode, toNode, myType,
    //                                    avgSpeed / 3.6, NBEdge::UNSPECIFIED_FRICTION, myNoLanes, -1,
    //                                    NBEdge::UNSPECIFIED_WIDTH, NBEdge::UNSPECIFIED_OFFSET,
    //                                    myGeom, LaneSpreadFunction::CENTER, myName, "", true);
    //     for (int i = 0; i < myNoLanes; i++) {
    //         buildEdge->setLaneWidth(i, myLaneWidths[i]);
    //         if ((int) myLaneSpeeds.size() <= i || myLaneSpeeds[i] == -1) {
    //             buildEdge->setSpeed(i, OptionsCont::getOptions().getFloat("vissim.default-speed") / (double) 3.6);
    //         } else {
    //             buildEdge->setSpeed(i, myLaneSpeeds[i] / (double) 3.6);
    //         }
    //     }
    //     ec.insert(buildEdge);
    //     // check whether the edge contains any other clusters
    //     if (tmpClusters.size() > 0) {
    //         bool cont = true;
    //         for (ConnectionClusters::iterator j = tmpClusters.begin(); cont && j != tmpClusters.end(); ++j) {
    //             // split the edge at the previously build node
    //             std::string nextID = buildEdge->getID() + "[1]";
    //             cont = ec.splitAt(dc, buildEdge, (*j)->getNBNode());
    //             // !!! what to do if the edge could not be split?
    //             buildEdge = ec.retrieve(nextID);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn build_nb_edge(&mut self, dc: &mut NBDistrictCont, nc: &mut NBNodeCont, ec: &mut NBEdgeCont, sameNodesOffset: f64) {
        todo!("port NIVissimEdge::buildNBEdge")
    }

    /// C++: `std::pair<NIVissimConnectionCluster *, NBNode *> NIVissimEdge::getFromNode(NBNodeCont &, ConnectionClusters &)` [private]
    /// Returns the origin node
    // std::pair<NIVissimConnectionCluster*, NBNode*>
    // NIVissimEdge::getFromNode(NBNodeCont& nc, ConnectionClusters& clusters) {
    //     // changed MAX_DISTANCE from 10 to 3.5, because 3.5 is the default lane width in VISSIM
    //     const double MAX_DISTANCE = 3.5;
    //     assert(clusters.size() >= 1);
    //     const Position& beg = myGeom.front();
    //     NIVissimConnectionCluster* c = *(clusters.begin());
    //     // check whether the edge starts within a already build node
    //     if (c->around(beg, MAX_DISTANCE)) {
    //         clusters.erase(clusters.begin());
    //         return std::pair<NIVissimConnectionCluster*, NBNode*>
    //                (c, c->getNBNode());
    //     }
    //     // check for a parking place at the begin
    //     if (myDistrictConnections.size() > 0) {
    //         double pos = *(myDistrictConnections.begin());
    //         if (pos < 10) {
    //             NBNode* node = new NBNode(toString<int>(myID) + "-begin", beg, SumoXMLNodeType::NOJUNCTION);
    //             if (!nc.insert(node)) {
    //                 throw 1;
    //             }
    //             while (myDistrictConnections.size() > 0 && *(myDistrictConnections.begin()) < 10) {
    //                 myDistrictConnections.erase(myDistrictConnections.begin());
    //             }
    //             return std::pair<NIVissimConnectionCluster*, NBNode*>(static_cast<NIVissimConnectionCluster*>(nullptr), node);
    //         }
    //     }
    //     // build a new node for the edge's begin otherwise
    //     NBNode* node = new NBNode(toString<int>(myID) + "-begin", beg, SumoXMLNodeType::NOJUNCTION);
    //     if (!nc.insert(node)) {
    //         throw 1;
    //     }
    //     return std::pair<NIVissimConnectionCluster*, NBNode*>(static_cast<NIVissimConnectionCluster*>(nullptr), node);
    // }
    #[allow(unused_variables)]
    pub fn get_from_node(&mut self, nc: &mut NBNodeCont, clusters: &mut NIVissimEdge_ConnectionClusters) -> (Ptr<NIVissimConnectionCluster>, Ptr<NBNode>) {
        todo!("port NIVissimEdge::getFromNode")
    }

    /// C++: `std::pair<NIVissimConnectionCluster *, NBNode *> NIVissimEdge::getToNode(NBNodeCont &, ConnectionClusters &)` [private]
    /// Returns the destination node
    // std::pair<NIVissimConnectionCluster*, NBNode*>
    // NIVissimEdge::getToNode(NBNodeCont& nc, ConnectionClusters& clusters) {
    //     const Position& end = myGeom.back();
    //     if (clusters.size() > 0) {
    //         const double MAX_DISTANCE = 10.;
    //         assert(clusters.size() >= 1);
    //         NIVissimConnectionCluster* c = *(clusters.end() - 1);
    //         // check whether the edge ends within a already build node
    //         if (c->around(end, MAX_DISTANCE)) {
    //             clusters.erase(clusters.end() - 1);
    //             return std::pair<NIVissimConnectionCluster*, NBNode*>(c, c->getNBNode());
    //         }
    //     }
    //     // check for a parking place at the end
    //     if (myDistrictConnections.size() > 0) {
    //         double pos = *(myDistrictConnections.end() - 1);
    //         if (pos > myGeom.length() - 10) {
    //             NBNode* node = new NBNode(toString<int>(myID) + "-end", end, SumoXMLNodeType::NOJUNCTION);
    //             if (!nc.insert(node)) {
    //                 throw 1;
    //             }
    //             while (myDistrictConnections.size() > 0 && *(myDistrictConnections.end() - 1) < myGeom.length() - 10) {
    //                 myDistrictConnections.erase(myDistrictConnections.end() - 1);
    //             }
    //             return std::pair<NIVissimConnectionCluster*, NBNode*>(static_cast<NIVissimConnectionCluster*>(nullptr), node);
    //         }
    //     }
    //
    //     // build a new node for the edge's end otherwise
    //     NBNode* node = new NBNode(toString<int>(myID) + "-end", end, SumoXMLNodeType::NOJUNCTION);
    //     if (!nc.insert(node)) {
    //         throw 1;
    //     }
    //     return std::pair<NIVissimConnectionCluster*, NBNode*>(static_cast<NIVissimConnectionCluster*>(nullptr), node);
    //     /*
    //     if (clusters.size()>0) {
    //     NIVissimConnectionCluster *c = *(clusters.end()-1);
    //     clusters.erase(clusters.end()-1);
    //     return std::pair<NIVissimConnectionCluster*, NBNode*>(c, c->getNBNode());
    //     } else {
    //     // !!! self-loop edge?!
    //     return std::pair<NIVissimConnectionCluster*, NBNode*>(static_cast<NIVissimConnectionCluster*>(0), (*(myConnectionClusters.begin()))->getNBNode());
    //     }
    //     */
    // }
    #[allow(unused_variables)]
    pub fn get_to_node(&mut self, nc: &mut NBNodeCont, clusters: &mut NIVissimEdge_ConnectionClusters) -> (Ptr<NIVissimConnectionCluster>, Ptr<NBNode>) {
        todo!("port NIVissimEdge::getToNode")
    }

    /// C++: `std::pair<NBNode *, NBNode *> NIVissimEdge::resolveSameNode(NBNodeCont &, double, NBNode *, NBNode *)` [private]
    /// Tries to resolve the problem that the same node has been returned as origin and destination node
    // std::pair<NBNode*, NBNode*>
    // NIVissimEdge::resolveSameNode(NBNodeCont& nc, double offset,
    //                               NBNode* prevFrom, NBNode* prevTo) {
    //     // check whether the edge is connected to a district
    //     //  use it if so
    //     NIVissimDistrictConnection* d =
    //         NIVissimDistrictConnection::dict_findForEdge(myID);
    //     if (d != nullptr) {
    //         Position pos = d->geomPosition();
    //         double position = d->getPosition();
    //         // the district is at the begin of the edge
    //         if (myGeom.length() - position > position) {
    //             std::string nid = "ParkingPlace" + toString<int>(d->getID());
    //             NBNode* node = nc.retrieve(nid);
    //             if (node == nullptr) {
    //                 node = new NBNode(nid,
    //                                   pos, SumoXMLNodeType::NOJUNCTION);
    //                 if (!nc.insert(node)) {
    //                     throw 1;
    //                 }
    //             }
    //             return std::pair<NBNode*, NBNode*>(node, prevTo);
    //         }
    //         // the district is at the end of the edge
    //         else {
    //             std::string nid = "ParkingPlace" + toString<int>(d->getID());
    //             NBNode* node = nc.retrieve(nid);
    //             if (node == nullptr) {
    //                 node = new NBNode(nid, pos, SumoXMLNodeType::NOJUNCTION);
    //                 if (!nc.insert(node)) {
    //                     throw 1;
    //                 }
    //             }
    //             assert(node != 0);
    //             return std::pair<NBNode*, NBNode*>(prevFrom, node);
    //         }
    //     }
    //     // otherwise, check whether the edge is some kind of
    //     //  a dead end...
    //     // check which end is nearer to the node centre
    //     if (myConnectionClusters.size() == 1) {
    //         NBNode* node = prevFrom; // it is the same as getToNode()
    //
    //         NIVissimConnectionCluster* c = *(myConnectionClusters.begin());
    //         // no end node given
    //         if (c->around(myGeom.front(), offset) && !c->around(myGeom.back(), offset)) {
    //             NBNode* end = new NBNode(
    //                 toString<int>(myID) + "-End",
    //                 myGeom.back(),
    //                 SumoXMLNodeType::NOJUNCTION);
    //             if (!nc.insert(end)) {
    //                 throw 1;
    //             }
    //             return std::pair<NBNode*, NBNode*>(node, end);
    //         }
    //
    //         // no begin node given
    //         if (!c->around(myGeom.front(), offset) && c->around(myGeom.back(), offset)) {
    //             NBNode* beg = new NBNode(
    //                 toString<int>(myID) + "-Begin",
    //                 myGeom.front(),
    //                 SumoXMLNodeType::NOJUNCTION);
    //             if (!nc.insert(beg)) {
    //                 std::cout << "nope, NIVissimDisturbance" << std::endl;
    //                 throw 1;
    //             }
    //             return std::pair<NBNode*, NBNode*>(beg, node);
    //         }
    //
    //         // self-loop edge - both points lie within the same cluster
    //         if (c->around(myGeom.front()) && c->around(myGeom.back())) {
    //             return std::pair<NBNode*, NBNode*>(node, node);
    //         }
    //     }
    //     // what to do in other cases?
    //     //  It simply is a self-looping edge....
    //     return std::pair<NBNode*, NBNode*>(prevFrom, prevTo);
    // }
    #[allow(unused_variables)]
    pub fn resolve_same_node(&mut self, nc: &mut NBNodeCont, offset: f64, prevFrom: Ptr<NBNode>, prevTo: Ptr<NBNode>) -> (Ptr<NBNode>, Ptr<NBNode>) {
        todo!("port NIVissimEdge::resolveSameNode")
    }

    /// C++: `std::vector<NIVissimConnection *> NIVissimEdge::getOutgoingConnected(int) const` [const, private]
    // std::vector<NIVissimConnection*>
    // NIVissimEdge::getOutgoingConnected(int lane) const {
    //     std::vector<NIVissimConnection*> ret;
    //     for (std::vector<int>::const_iterator i = myOutgoingConnections.begin(); i != myOutgoingConnections.end(); i++) {
    //         NIVissimConnection* c = NIVissimConnection::dictionary(*i);
    //         const std::vector<int>& lanes = c->getFromLanes();
    //         if (find(lanes.begin(), lanes.end(), lane) != lanes.end()) {
    //             NIVissimEdge* e = NIVissimEdge::dictionary(c->getToEdgeID());
    //             if (e != nullptr) {
    //                 ret.push_back(c);
    //             }
    //         }
    //     }
    //     return ret;
    // }
    #[allow(unused_variables)]
    pub fn get_outgoing_connected(&self, lane: i32) -> Vec<Ptr<NIVissimConnection>> {
        todo!("port NIVissimEdge::getOutgoingConnected")
    }

    /// C++: `void NIVissimEdge::propagateSpeed(double, std::vector<int>)` [private]
    // void
    // NIVissimEdge::propagateSpeed(double speed, std::vector<int> forLanes) {
    //     // if no lane is given, all set be set
    //     if (forLanes.size() == 0) {
    //         for (int i = 0; i < myNoLanes; i++) {
    //             forLanes.push_back((int) i);
    //         }
    //     }
    //     // for the case of a first call
    //     // go through the lanes
    //     for (std::vector<int>::const_iterator i = forLanes.begin(); i < forLanes.end(); i++) {
    //         // check whether a speed was set before
    //         if (myLaneSpeeds[*i] != -1) {
    //             // do not reset it from incoming
    //             continue;
    //         }
    //         // check whether the lane has a new speed to set
    //         if ((int) myPatchedSpeeds.size() > *i && myPatchedSpeeds[*i] != -1) {
    //             // use it
    //             speed = getRealSpeed(/*dc, */myPatchedSpeeds[*i]);
    //         }
    //         // check whether a speed is given
    //         if (speed == -1) {
    //             // do nothing if not
    //             continue;
    //         }
    //         // set the lane's speed to the given
    //         myLaneSpeeds[*i] = speed;
    //         // propagate the speed further
    //         // get the list of connected edges
    //         std::vector<NIVissimConnection*> connected = getOutgoingConnected(*i);
    //         // go through the list
    //         for (std::vector<NIVissimConnection*>::iterator j = connected.begin(); j != connected.end(); j++) {
    //             NIVissimConnection* c = *j;
    //             NIVissimEdge* e = NIVissimEdge::dictionary(c->getToEdgeID());
    //             // propagate
    //             e->propagateSpeed(/*dc, */speed, c->getToLanes());
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn propagate_speed(&mut self, speed: f64, forLanes: Vec<i32>) {
        todo!("port NIVissimEdge::propagateSpeed")
    }

    /// C++: `void NIVissimEdge::setDistrictSpeed()` [private]
    // void
    // NIVissimEdge::setDistrictSpeed() {
    //     if (myDistrictConnections.size() > 0) {
    //         double pos = *(myDistrictConnections.begin());
    //         if (pos < getLength() - pos) {
    //             NIVissimDistrictConnection* d =
    //                 NIVissimDistrictConnection::dict_findForEdge(myID);
    //             if (d != nullptr) {
    //                 double speed = d->getMeanSpeed(/*dc*/);
    //                 if (speed == -1) {
    //                     return;
    //                 }
    //                 for (int i = 0; i < myNoLanes; i++) {
    //                     myLaneSpeeds[i] = speed;
    //                     // propagate the speed further
    //                     // get the list of connected edges
    //                     std::vector<NIVissimConnection*> connected = getOutgoingConnected(i);
    //                     // go through the list
    //                     for (std::vector<NIVissimConnection*>::iterator j = connected.begin(); j != connected.end(); j++) {
    //                         NIVissimConnection* c = *j;
    //                         NIVissimEdge* e = NIVissimEdge::dictionary(c->getToEdgeID());
    //                         // propagate
    //                         e->propagateSpeed(/*dc, */speed, c->getToLanes());
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_district_speed(&mut self) {
        todo!("port NIVissimEdge::setDistrictSpeed")
    }

    /// C++: `double NIVissimEdge::getRealSpeed(int)` [private]
    // double
    // NIVissimEdge::getRealSpeed(int distNo) {
    //     std::string id = toString<int>(distNo);
    //     Distribution* dist = DistributionCont::dictionary("speed", id);
    //     if (dist == nullptr) {
    //         WRITE_WARNINGF(TL("The referenced speed distribution '%' is not known."), id);
    //         return -1;
    //     }
    //     assert(dist != 0);
    //     double speed = dist->getMax();
    //     if (speed < 0 || speed > 1000) {
    //         WRITE_WARNINGF(TL("What about distribution '%'"), toString<int>(distNo));
    //     }
    //     return speed;
    // }
    #[allow(unused_variables)]
    pub fn get_real_speed(&mut self, distNo: i32) -> f64 {
        todo!("port NIVissimEdge::getRealSpeed")
    }

    /// C++: `void NIVissimEdge::checkUnconnectedLaneSpeeds()` [private]
    // void
    // NIVissimEdge::checkUnconnectedLaneSpeeds() {
    //     for (int i = 0; i < (int) myLaneSpeeds.size(); i++) {
    //         if (myLaneSpeeds[i] == -1) {
    //             double speed = -1;
    //             int j1 = i - 1; // !!! recheck - j1 may become negative?
    //             int j2 = i;
    //             while (j2 != (int) myLaneSpeeds.size() && myLaneSpeeds[j2] == -1) {
    //                 j2++;
    //             }
    //             if (j1 < 0) {
    //                 if (j2 < (int) myLaneSpeeds.size()) {
    //                     speed = myLaneSpeeds[j2];
    //                 }
    //             } else {
    //                 if (j2 >= (int) myLaneSpeeds.size()) {
    //                     speed = myLaneSpeeds[j1];
    //                 } else {
    //                     speed = (myLaneSpeeds[j1] + myLaneSpeeds[j2]) / (double) 2.0;
    //                 }
    //             }
    //             if (speed == -1) {
    //                 continue;
    //             }
    //             myLaneSpeeds[i] = speed;
    //             std::vector<NIVissimConnection*> connected = getOutgoingConnected(i);
    //             for (std::vector<NIVissimConnection*>::iterator j = connected.begin(); j != connected.end(); j++) {
    //                 NIVissimConnection* c = *j;
    //                 NIVissimEdge* e = NIVissimEdge::dictionary(c->getToEdgeID());
    //                 // propagate
    //                 e->propagateSpeed(/*dc, */speed, c->getToLanes());
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn check_unconnected_lane_speeds(&mut self) {
        todo!("port NIVissimEdge::checkUnconnectedLaneSpeeds")
    }

    /// C++: `void NIVissimEdge::propagateOwn()` [private]
    // void
    // NIVissimEdge::propagateOwn() {
    //     for (int i = 0; i < (int) myLaneSpeeds.size(); i++) {
    //         if (myLaneSpeeds[i] == -1) {
    //             continue;
    //         }
    //         std::vector<NIVissimConnection*> connected = getOutgoingConnected(i);
    //         for (std::vector<NIVissimConnection*>::iterator j = connected.begin(); j != connected.end(); j++) {
    //             NIVissimConnection* c = *j;
    //             NIVissimEdge* e = NIVissimEdge::dictionary(c->getToEdgeID());
    //             // propagate
    //             e->propagateSpeed(/*dc, */myLaneSpeeds[i], c->getToLanes());
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn propagate_own(&mut self) {
        todo!("port NIVissimEdge::propagateOwn")
    }

    /// C++: `NBNode * NIVissimEdge::getNodeSecure(int, const Position &, const std::string &)` [static, private]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_node_secure(nodeid: i32, pos: &Position, possibleName: &str) -> Ptr<NBNode> {
        todo!("port NIVissimEdge::getNodeSecure")
    }

    /// C++: `std::pair<NBNode *, NBNode *> NIVissimEdge::remapOneOfNodes(NBNodeCont &, NIVissimDistrictConnection *, NBNode *, NBNode *)` [private]
    // std::pair<NBNode*, NBNode*>
    // NIVissimEdge::remapOneOfNodes(NBNodeCont& nc,
    //                               NIVissimDistrictConnection* d,
    //                               NBNode* fromNode, NBNode* toNode) {
    //     std::string nid = "ParkingPlace" + toString<int>(d->getID());
    //     if (d->geomPosition().distanceTo(fromNode->getPosition())
    //             <
    //             d->geomPosition().distanceTo(toNode->getPosition())) {
    //
    //         NBNode* newNode = new NBNode(nid,
    //                                      fromNode->getPosition(),
    //                                      SumoXMLNodeType::NOJUNCTION);
    //         nc.erase(fromNode);
    //         nc.insert(newNode);
    //         return std::pair<NBNode*, NBNode*>(newNode, toNode);
    //     } else {
    //         NBNode* newNode = new NBNode(nid,
    //                                      toNode->getPosition(),
    //                                      SumoXMLNodeType::NOJUNCTION);
    //         nc.erase(toNode);
    //         nc.insert(newNode);
    //         return std::pair<NBNode*, NBNode*>(fromNode, newNode);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn remap_one_of_nodes(&mut self, nc: &mut NBNodeCont, d: Ptr<NIVissimDistrictConnection>, fromNode: Ptr<NBNode>, toNode: Ptr<NBNode>) -> (Ptr<NBNode>, Ptr<NBNode>) {
        todo!("port NIVissimEdge::remapOneOfNodes")
    }

}

/// Sorts connection clusters the edge participates in by their position along the given edge
/// C++ class `NIVissimEdge::connection_cluster_position_sorter` (netimport/vissim/tempstructs/NIVissimEdge.h)
pub struct NIVissimEdge_connection_cluster_position_sorter {
    /// The id of the edge
    pub myEdgeID: i32,
}

impl NIVissimEdge_connection_cluster_position_sorter {
    /// C++: `void connection_cluster_position_sorter::connection_cluster_position_sorter(int)`
    /// constructor
    // NIVissimEdge::connection_cluster_position_sorter::connection_cluster_position_sorter(int edgeid)
    //     : myEdgeID(edgeid) {}
    #[allow(unused_variables)]
    pub fn new(edgeid: i32) -> Self {
        todo!("port connection_cluster_position_sorter::connection_cluster_position_sorter")
    }

    /// C++: `int connection_cluster_position_sorter::operator()(NIVissimConnectionCluster *, NIVissimConnectionCluster *) const` [const]
    /// comparing operation
    // int
    // NIVissimEdge::connection_cluster_position_sorter::operator()(
    //     NIVissimConnectionCluster* cc1,
    //     NIVissimConnectionCluster* cc2) const {
    //     double pos1 = cc1->getPositionForEdge(myEdgeID);
    //     double pos2 = cc2->getPositionForEdge(myEdgeID);
    //     if (pos2 < 0 || pos1 < 0) {
    //         cc1->getPositionForEdge(myEdgeID);
    //         cc2->getPositionForEdge(myEdgeID);
    //     }
    //     assert(pos1 >= 0 && pos2 >= 0);
    //     return pos1 < pos2;
    // }
    #[allow(unused_variables)]
    pub fn call_op(&self, cc1: Ptr<NIVissimConnectionCluster>, cc2: Ptr<NIVissimConnectionCluster>) -> i32 {
        todo!("port connection_cluster_position_sorter::operator()")
    }

}

/// Sorts connections the edge participates in by their position along the given edge
/// C++ class `NIVissimEdge::connection_position_sorter` (netimport/vissim/tempstructs/NIVissimEdge.h)
pub struct NIVissimEdge_connection_position_sorter {
    /// The id of the edge
    pub myEdgeID: i32,
}

impl NIVissimEdge_connection_position_sorter {
    /// C++: `void connection_position_sorter::connection_position_sorter(int)`
    /// constructor
    // NIVissimEdge::connection_position_sorter::connection_position_sorter(int edgeid)
    //     : myEdgeID(edgeid) {}
    #[allow(unused_variables)]
    pub fn new(edgeid: i32) -> Self {
        todo!("port connection_position_sorter::connection_position_sorter")
    }

    /// C++: `int connection_position_sorter::operator()(int, int) const` [const]
    /// comparing operation
    // int
    // NIVissimEdge::connection_position_sorter::operator()(int c1id,
    //         int c2id) const {
    //     NIVissimConnection* c1 = NIVissimConnection::dictionary(c1id);
    //     NIVissimConnection* c2 = NIVissimConnection::dictionary(c2id);
    //     double pos1 =
    //         c1->getFromEdgeID() == myEdgeID
    //         ? c1->getFromPosition() : c1->getToPosition();
    //     double pos2 =
    //         c2->getFromEdgeID() == myEdgeID
    //         ? c2->getFromPosition() : c2->getToPosition();
    //     return pos1 < pos2;
    // }
    #[allow(unused_variables)]
    pub fn call_op(&self, c1id: i32, c2id: i32) -> i32 {
        todo!("port connection_position_sorter::operator()")
    }

}

