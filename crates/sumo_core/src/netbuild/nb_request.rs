//! Port of netbuild/NBRequest.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `std::vector<LinkInfoCont> NBRequest::CombinationsCont`
pub type NBRequest_CombinationsCont = Vec<NBRequest_LinkInfoCont>;

/// C++ typedef `std::vector<bool> NBRequest::LinkInfoCont`
pub type NBRequest_LinkInfoCont = Vec<bool>;

/// Given a special node, this class builds the logic of this (junction) regarding the relationships between the incoming and outgoing edges and their priorities. The junction's logic is saved when it does not yet exist.
/// C++ class `NBRequest` (netbuild/NBRequest.h)
pub struct NBRequest {
    /// the node the request is assigned to
    pub myJunction: Ptr<NBNode>,
    /// all (icoming and outgoing) of the junctions edges
    pub myAll: Ptr<EdgeVector>,
    /// edges incoming to the junction
    pub myIncoming: Ptr<EdgeVector>,
    /// edges outgoing from the junction
    pub myOutgoing: Ptr<EdgeVector>,
    /// the link X link blockings
    pub myForbids: NBRequest_CombinationsCont,
    /// the link X link is done-checks
    pub myDone: NBRequest_CombinationsCont,
    /// precomputed right-of-way matrices for each lane-to-lane link
    pub myFoes: Vec<String>,
    pub myResponse: Vec<String>,
    pub myHaveVia: Vec<bool>,
}

// C++ static members of NBRequest (port as module-level statics, see AGENTS.md):
//   static int NBRequest::myGoodBuilds;
//   static int NBRequest::myNotBuild;

impl NBRequest {
    /// C++: `void NBRequest::NBRequest(const NBEdgeCont &, NBNode *, const EdgeVector &, const EdgeVector &, const EdgeVector &, const NBConnectionProhibits &)`
    /// constructor The parameter are the logic's lists of edges (all, incoming only and outgoing only edges). By now no further informations are needed to describe the junctions. These parameter must not be changed during the logic's building
    // NBRequest::NBRequest(const NBEdgeCont& ec,
    //                      NBNode* junction,
    //                      const EdgeVector& all,
    //                      const EdgeVector& incoming,
    //                      const EdgeVector& outgoing,
    //                      const NBConnectionProhibits& loadedProhibits) :
    //     myJunction(junction),
    //     myAll(all),
    //     myIncoming(incoming),
    //     myOutgoing(outgoing) {
    //     const int variations = numLinks();
    //     // build maps with information which forbidding connection were
    //     //  computed and what's in there
    //     myForbids.reserve(variations);
    //     myDone.reserve(variations);
    //     for (int i = 0; i < variations; i++) {
    //         myForbids.push_back(LinkInfoCont(variations, false));
    //         myDone.push_back(LinkInfoCont(variations, false));
    //     }
    //     // insert loaded prohibits
    //     for (NBConnectionProhibits::const_iterator j = loadedProhibits.begin(); j != loadedProhibits.end(); j++) {
    //         NBConnection prohibited = (*j).first;
    //         bool ok1 = prohibited.check(ec);
    //         if (find(myIncoming.begin(), myIncoming.end(), prohibited.getFrom()) == myIncoming.end()) {
    //             ok1 = false;
    //         }
    //         if (find(myOutgoing.begin(), myOutgoing.end(), prohibited.getTo()) == myOutgoing.end()) {
    //             ok1 = false;
    //         }
    //         int idx1 = 0;
    //         if (ok1) {
    //             idx1 = getIndex(prohibited.getFrom(), prohibited.getTo());
    //             if (idx1 < 0) {
    //                 ok1 = false;
    //             }
    //         }
    //         const NBConnectionVector& prohibiting = (*j).second;
    //         for (NBConnectionVector::const_iterator k = prohibiting.begin(); k != prohibiting.end(); k++) {
    //             NBConnection sprohibiting = *k;
    //             bool ok2 = sprohibiting.check(ec);
    //             if (find(myIncoming.begin(), myIncoming.end(), sprohibiting.getFrom()) == myIncoming.end()) {
    //                 ok2 = false;
    //             }
    //             if (find(myOutgoing.begin(), myOutgoing.end(), sprohibiting.getTo()) == myOutgoing.end()) {
    //                 ok2 = false;
    //             }
    //             if (ok1 && ok2) {
    //                 int idx2 = getIndex(sprohibiting.getFrom(), sprohibiting.getTo());
    //                 if (idx2 < 0) {
    //                     ok2 = false;
    //                 } else {
    //                     myForbids[idx2][idx1] = true;
    //                     myDone[idx2][idx1] = true;
    //                     myDone[idx1][idx2] = true;
    //                     myGoodBuilds++;
    //                 }
    //             } else {
    //                 std::string pfID = prohibited.getFrom() != nullptr ? prohibited.getFrom()->getID() : "UNKNOWN";
    //                 std::string ptID = prohibited.getTo() != nullptr ? prohibited.getTo()->getID() : "UNKNOWN";
    //                 std::string bfID = sprohibiting.getFrom() != nullptr ? sprohibiting.getFrom()->getID() : "UNKNOWN";
    //                 std::string btID = sprohibiting.getTo() != nullptr ? sprohibiting.getTo()->getID() : "UNKNOWN";
    //                 WRITE_WARNING("could not prohibit " + pfID + "->" + ptID + " by " + bfID + "->" + btID);
    //                 myNotBuild++;
    //             }
    //         }
    //     }
    //     // ok, check whether someone has prohibited two links vice versa
    //     //  (this happens also in some Vissim-networks, when edges are joined)
    //     for (int s1 = 0; s1 < variations; s1++) {
    //         for (int s2 = s1 + 1; s2 < variations; s2++) {
    //             // not set, yet
    //             if (!myDone[s1][s2]) {
    //                 continue;
    //             }
    //             // check whether both prohibit vice versa
    //             if (myForbids[s1][s2] && myForbids[s2][s1]) {
    //                 // mark unset - let our algorithm fix it later
    //                 myDone[s1][s2] = false;
    //                 myDone[s2][s1] = false;
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn new(ec: &NBEdgeCont, junction: Ptr<NBNode>, all: &EdgeVector, incoming: &EdgeVector, outgoing: &EdgeVector, loadedProhibits: &NBConnectionProhibits) -> Self {
        todo!("port NBRequest::NBRequest")
    }

    /// C++: `void NBRequest::~NBRequest()`
    /// destructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port NBRequest::~NBRequest")
    }

    /// C++: `void NBRequest::buildBitfieldLogic()`
    /// builds the bitset-representation of the logic
    // void
    // NBRequest::buildBitfieldLogic() {
    //     EdgeVector::const_iterator i, j;
    //     for (i = myIncoming.begin(); i != myIncoming.end(); i++) {
    //         for (j = myOutgoing.begin(); j != myOutgoing.end(); j++) {
    //             computeRightOutgoingLinkCrossings(*i, *j);
    //             computeLeftOutgoingLinkCrossings(*i, *j);
    //         }
    //     }
    //     // reset signalised/non-signalised dependencies
    //     resetSignalised();
    //     // reset foes it the number of lanes matches (or exceeds) the number of incoming connections
    //     resetCooperating();
    // }
    #[allow(unused_variables)]
    pub fn build_bitfield_logic(&mut self) {
        todo!("port NBRequest::buildBitfieldLogic")
    }

    /// C++: `std::pair<int, int> NBRequest::getSizes() const` [const]
    /// returns the number of the junction's lanes and the number of the junction's links in respect.
    // std::pair<int, int>
    // NBRequest::getSizes() const {
    //     int numLanes = 0;
    //     int numLinks = 0;
    //     for (const NBEdge* const e : myIncoming) {
    //         const int numLanesEdge = e->getNumLanes();
    //         for (int j = 0; j < numLanesEdge; j++) {
    //             int numConnections = (int)e->getConnectionsFromLane(j).size();
    //             numLinks += numConnections;
    //             if (numConnections > 0) {
    //                 numLanes++;
    //             }
    //         }
    //     }
    //     return std::make_pair(numLanes, numLinks);
    // }
    #[allow(unused_variables)]
    pub fn get_sizes(&self) -> (i32, i32) {
        todo!("port NBRequest::getSizes")
    }

    /// C++: `bool NBRequest::mustBrake(const NBEdge *const, const NBEdge *const, const NBEdge *const, const NBEdge *const) const` [const]
    /// Returns the information whether "prohibited" flow must let "prohibitor" flow pass
    // bool
    // NBRequest::mustBrake(const NBEdge* const possProhibitorFrom, const NBEdge* const possProhibitorTo,
    //                      const NBEdge* const possProhibitedFrom, const NBEdge* const possProhibitedTo) const {
    //     // get the indices
    //     int idx1 = getIndex(possProhibitorFrom, possProhibitorTo);
    //     int idx2 = getIndex(possProhibitedFrom, possProhibitedTo);
    //     return (myForbids[idx2][idx1]);
    // }
    #[allow(unused_variables)]
    pub fn must_brake(&self, possProhibitorFrom: Ptr<NBEdge>, possProhibitorTo: Ptr<NBEdge>, possProhibitedFrom: Ptr<NBEdge>, possProhibitedTo: Ptr<NBEdge>) -> bool {
        todo!("port NBRequest::mustBrake")
    }

    /// C++: `bool NBRequest::mustBrake(const NBEdge *const, const NBEdge *const, int, int, bool) const` [const]
    /// Returns the information whether the described flow must let any other flow pass
    // bool
    // NBRequest::mustBrake(const NBEdge* const from, const NBEdge* const to, int fromLane, int toLane, bool includePedCrossings) const {
    //     NBEdge::Connection con(fromLane, const_cast<NBEdge*>(to), toLane);
    //     const int linkIndex = myJunction->getConnectionIndex(from, con);
    //     if (linkIndex >= 0 && (int)myResponse.size() > linkIndex) {
    //         std::string response = getResponse(linkIndex);
    //         if (!includePedCrossings) {
    //             response = response.substr(0, response.size() - myJunction->getCrossings().size());
    //         }
    //         if (response.find_first_of("1") == std::string::npos) {
    //             return false;
    //         } else if (!myJunction->isTLControlled()) {
    //             return true;
    //         }
    //         // if the link must respond it could also be due to a tlsConflict. This
    //         // must not carry over the off-state response so we continue with
    //         // the regular check
    //     }
    //     // get the indices
    //     int idx2 = getIndex(from, to);
    //     if (idx2 == -1) {
    //         return false;
    //     }
    //     // go through all (existing) connections;
    //     //  check whether any of these forbids the one to determine
    //     assert(idx2 < (int)(myIncoming.size()*myOutgoing.size()));
    //     for (int idx1 = 0; idx1 < numLinks(); idx1++) {
    //         //assert(myDone[idx1][idx2]);
    //         if (myDone[idx1][idx2] && myForbids[idx1][idx2]) {
    //             return true;
    //         }
    //     }
    //     // maybe we need to brake for a pedestrian crossing
    //     if (includePedCrossings) {
    //         auto crossings = myJunction->getCrossings();
    //         for (std::vector<NBNode::Crossing*>::const_reverse_iterator i = crossings.rbegin(); i != crossings.rend(); i++) {
    //             if (mustBrakeForCrossing(myJunction, from, to, **i)) {
    //                 return true;
    //             }
    //         }
    //     }
    //     // maybe we need to brake due to a right-turn conflict with straight-going
    //     // bicycles
    //     const NBEdge::Connection& queryCon = from->getConnection(fromLane, to, toLane);
    //     LinkDirection dir = myJunction->getDirection(from, to);
    //     if (dir == LinkDirection::RIGHT || dir == LinkDirection::PARTRIGHT) {
    //         for (const NBEdge::Connection& fromCon : from->getConnections()) {
    //             if (rightTurnConflict(from, queryCon, from, fromCon)) {
    //                 return true;
    //             }
    //         }
    //     }
    //     // maybe we need to brake due to a merge conflict
    //     for (EdgeVector::const_reverse_iterator i = myIncoming.rbegin(); i != myIncoming.rend(); i++) {
    //         int noLanes = (*i)->getNumLanes();
    //         for (int j = noLanes; j-- > 0;) {
    //             std::vector<NBEdge::Connection> connected = (*i)->getConnectionsFromLane(j);
    //             const int size = (int) connected.size();
    //             for (int k = size; k-- > 0;) {
    //                 if ((*i) == from && fromLane != j
    //                         && mergeConflict(from, queryCon, *i, connected[k], myJunction->getType() == SumoXMLNodeType::ZIPPER)) {
    //                     return true;
    //                 }
    //             }
    //         }
    //     }
    //     // maybe we need to brake due to a zipper conflict
    //     if (myJunction->getType() == SumoXMLNodeType::ZIPPER) {
    //         for (int idx1 = 0; idx1 < numLinks(); idx1++) {
    //             //assert(myDone[idx1][idx2]);
    //             if (myDone[idx1][idx2] && myForbids[idx2][idx1]) {
    //                 return true;
    //             }
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn must_brake_2(&self, from: Ptr<NBEdge>, to: Ptr<NBEdge>, fromLane: i32, toLane: i32, includePedCrossings: bool) -> bool {
        todo!("port NBRequest::mustBrake")
    }

    /// C++: `bool NBRequest::mustBrakeForCrossing(const NBNode *, const NBEdge *const, const NBEdge *const, const NBNode::Crossing &)` [static]
    /// Returns the information whether the described flow must brake for the given crossing
    // bool
    // NBRequest::mustBrakeForCrossing(const NBNode* node, const NBEdge* const from, const NBEdge* const to, const NBNode::Crossing& crossing) {
    //     const LinkDirection dir = node->getDirection(from, to);
    //     const bool mustYield = dir == LinkDirection::LEFT || dir == LinkDirection::RIGHT;
    //     if (crossing.priority || mustYield) {
    //         for (const NBEdge* const e : crossing.edges) {
    //             // left and right turns must yield to unprioritized crossings only on their destination edge
    //             if ((e == from && crossing.priority) || e == to) {
    //                 return true;
    //             }
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn must_brake_for_crossing(node: Ptr<NBNode>, from: Ptr<NBEdge>, to: Ptr<NBEdge>, crossing: &NBNode_Crossing) -> bool {
        todo!("port NBRequest::mustBrakeForCrossing")
    }

    /// C++: `bool NBRequest::foes(const NBEdge *const, const NBEdge *const, const NBEdge *const, const NBEdge *const) const` [const]
    /// Returns the information whether the given flows cross
    // bool
    // NBRequest::foes(const NBEdge* const from1, const NBEdge* const to1,
    //                 const NBEdge* const from2, const NBEdge* const to2) const {
    //     // unconnected edges do not forbid other edges
    //     if (to1 == nullptr || to2 == nullptr) {
    //         return false;
    //     }
    //     // get the indices
    //     int idx1 = getIndex(from1, to1);
    //     int idx2 = getIndex(from2, to2);
    //     if (idx1 < 0 || idx2 < 0) {
    //         return false; // sure? (The connection does not exist within this junction)
    //     }
    //     assert(idx1 < (int)(myIncoming.size() * myOutgoing.size()));
    //     assert(idx2 < (int)(myIncoming.size()*myOutgoing.size()));
    //     return myForbids[idx1][idx2] || myForbids[idx2][idx1];
    // }
    #[allow(unused_variables)]
    pub fn foes(&self, from1: Ptr<NBEdge>, to1: Ptr<NBEdge>, from2: Ptr<NBEdge>, to2: Ptr<NBEdge>) -> bool {
        todo!("port NBRequest::foes")
    }

    /// C++: `bool NBRequest::forbids(const NBEdge *const, const NBEdge *const, const NBEdge *const, const NBEdge *const, bool) const` [const]
    /// Returns the information whether "prohibited" flow must let "prohibitor" flow pass
    // bool
    // NBRequest::forbids(const NBEdge* const possProhibitorFrom, const NBEdge* const possProhibitorTo,
    //                    const NBEdge* const possProhibitedFrom, const NBEdge* const possProhibitedTo,
    //                    bool regardNonSignalisedLowerPriority) const {
    //     // unconnected edges do not forbid other edges
    //     if (possProhibitorTo == nullptr || possProhibitedTo == nullptr) {
    //         return false;
    //     }
    //     // get the indices
    //     int possProhibitorIdx = getIndex(possProhibitorFrom, possProhibitorTo);
    //     int possProhibitedIdx = getIndex(possProhibitedFrom, possProhibitedTo);
    //     if (possProhibitorIdx < 0 || possProhibitedIdx < 0) {
    //         return false; // sure? (The connection does not exist within this junction)
    //     }
    //     assert(possProhibitorIdx < (int)(myIncoming.size() * myOutgoing.size()));
    //     assert(possProhibitedIdx < (int)(myIncoming.size() * myOutgoing.size()));
    //     // check simple right-of-way-rules
    //     if (!regardNonSignalisedLowerPriority) {
    //         return myForbids[possProhibitorIdx][possProhibitedIdx];
    //     }
    //     // if its not forbidden, report
    //     if (!myForbids[possProhibitorIdx][possProhibitedIdx]) {
    //         return false;
    //     }
    //     // do not forbid a signalised stream by a non-signalised
    //     if (!possProhibitorFrom->hasSignalisedConnectionTo(possProhibitorTo)) {
    //         return false;
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn forbids(&self, possProhibitorFrom: Ptr<NBEdge>, possProhibitorTo: Ptr<NBEdge>, possProhibitedFrom: Ptr<NBEdge>, possProhibitedTo: Ptr<NBEdge>, regardNonSignalisedLowerPriority: bool) -> bool {
        todo!("port NBRequest::forbids")
    }

    /// C++: `void NBRequest::computeLogic(const bool)`
    /// writes the XML-representation of the logic as a bitset-logic XML representation
    // void
    // NBRequest::computeLogic(const bool checkLaneFoes) {
    //     myResponse.clear();
    //     myFoes.clear();
    //     myHaveVia.clear();
    //     int pos = 0;
    //     EdgeVector::const_iterator i;
    //     // normal connections
    //     for (i = myIncoming.begin(); i != myIncoming.end(); i++) {
    //         int noLanes = (*i)->getNumLanes();
    //         for (int k = 0; k < noLanes; k++) {
    //             pos = computeLaneResponse(*i, k, pos, checkLaneFoes || myJunction->getType() == SumoXMLNodeType::ZIPPER);
    //         }
    //     }
    //     // crossings
    //     auto crossings = myJunction->getCrossings();
    //     for (auto c : crossings) {
    //         pos = computeCrossingResponse(*c, pos);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn compute_logic(&mut self, checkLaneFoes: bool) {
        todo!("port NBRequest::computeLogic")
    }

    /// C++: `void NBRequest::writeLogic(OutputDevice &) const` [const]
    // void
    // NBRequest::writeLogic(OutputDevice& into) const {
    //     int numLinks = (int)myResponse.size();
    //     assert((int)myFoes.size() == numLinks);
    //     assert((int)myHaveVia.size() == numLinks);
    //     const bool padding = numLinks > 10;
    //     for (int i = 0; i < numLinks; i++) {
    //         into.openTag(SUMO_TAG_REQUEST);
    //         into.writeAttr(SUMO_ATTR_INDEX, i);
    //         if (padding && i < 10) {
    //             into.writePadding(" ");
    //         }
    //         into.writeAttr(SUMO_ATTR_RESPONSE, myResponse[i]);
    //         into.writeAttr(SUMO_ATTR_FOES, myFoes[i]);
    //         if (!OptionsCont::getOptions().getBool("no-internal-links")) {
    //             into.writeAttr(SUMO_ATTR_CONT, myHaveVia[i]);
    //         }
    //         into.closeTag();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn write_logic(&self, into: &mut OutputDevice) {
        todo!("port NBRequest::writeLogic")
    }

    /// C++: `const std::string & NBRequest::getFoes(int) const` [const]
    // const std::string&
    // NBRequest::getFoes(int linkIndex) const {
    //     assert(linkIndex >= 0);
    //     assert(linkIndex < (int)myFoes.size());
    //     return myFoes[linkIndex];
    // }
    #[allow(unused_variables)]
    pub fn get_foes(&self, linkIndex: i32) -> &str {
        todo!("port NBRequest::getFoes")
    }

    /// C++: `const std::string & NBRequest::getResponse(int) const` [const]
    // const std::string&
    // NBRequest::getResponse(int linkIndex) const {
    //     assert(linkIndex >= 0);
    //     assert(linkIndex < (int)myResponse.size());
    //     return myResponse[linkIndex];
    // }
    #[allow(unused_variables)]
    pub fn get_response(&self, linkIndex: i32) -> &str {
        todo!("port NBRequest::getResponse")
    }

    /// C++: `void NBRequest::reportWarnings()` [static]
    /// reports warnings if any occurred
    // void
    // NBRequest::reportWarnings() {
    //     // check if any errors occurred on build the link prohibitions
    //     if (myNotBuild != 0) {
    //         WRITE_WARNING(toString(myNotBuild) + " of " + toString(myNotBuild + myGoodBuilds) + " prohibitions were not build.");
    //     }
    // }
    #[allow(unused_variables)]
    pub fn report_warnings() {
        todo!("port NBRequest::reportWarnings")
    }

    /// C++: `bool NBRequest::rightTurnConflict(const NBEdge *, const NBEdge::Connection &, const NBEdge *, const NBEdge::Connection &) const` [const]
    /// whether the given connections crosses the foe connection from the same lane and must yield
    // bool
    // NBRequest::rightTurnConflict(const NBEdge* from, const NBEdge::Connection& con,
    //                              const NBEdge* prohibitorFrom, const NBEdge::Connection& prohibitorCon) const {
    //     return (!con.mayDefinitelyPass &&
    //             (NBNode::rightTurnConflict(from, con.toEdge, con.fromLane, prohibitorFrom, prohibitorCon.toEdge, prohibitorCon.fromLane)
    //              // reverse conflicht (override)
    //              || (prohibitorCon.mayDefinitelyPass &&
    //                  NBNode::rightTurnConflict(prohibitorFrom, prohibitorCon.toEdge, prohibitorCon.fromLane, from, con.toEdge, con.fromLane))));
    //
    //
    // }
    #[allow(unused_variables)]
    pub fn right_turn_conflict(&self, from: Ptr<NBEdge>, con: &NBEdge_Connection, prohibitorFrom: Ptr<NBEdge>, prohibitorCon: &NBEdge_Connection) -> bool {
        todo!("port NBRequest::rightTurnConflict")
    }

    /// C++: `bool NBRequest::mergeConflict(const NBEdge *, const NBEdge::Connection &, const NBEdge *, const NBEdge::Connection &, bool) const` [const]
    /// whether multple connections from the same edge target the same lane
    // bool
    // NBRequest::mergeConflict(const NBEdge* from, const NBEdge::Connection& con,
    //                          const NBEdge* prohibitorFrom,  const NBEdge::Connection& prohibitorCon, bool foes) const {
    //     if (from == prohibitorFrom
    //             && con.toEdge == prohibitorCon.toEdge
    //             && ((con.toLane == prohibitorCon.toLane
    //                  && con.fromLane != prohibitorCon.fromLane
    //                  && !myJunction->isConstantWidthTransition())
    //                 // this is actually a crossing rather than a merger
    //                 || (con.fromLane > prohibitorCon.fromLane && con.toLane < prohibitorCon.toLane)
    //                 || (con.fromLane < prohibitorCon.fromLane && con.toLane > prohibitorCon.toLane))) {
    //         if (foes) {
    //             return true;
    //         }
    //         if (prohibitorCon.mayDefinitelyPass) {
    //             return true;
    //         }
    //         if (con.mayDefinitelyPass) {
    //             return false;
    //         }
    //         const bool bike = from->getPermissions(con.fromLane) == SVC_BICYCLE;
    //         const bool prohibitorBike = prohibitorFrom->getPermissions(prohibitorCon.fromLane) == SVC_BICYCLE;
    //         if (myOutgoing.size() == 1) {
    //             // at on-ramp like situations, right lane should yield
    //             return bike || (con.fromLane < prohibitorCon.fromLane && !prohibitorBike);
    //         } else {
    //             // priority depends on direction:
    //             // for right turns the rightmost lane gets priority
    //             // otherwise the left lane
    //             LinkDirection dir = myJunction->getDirection(from, con.toEdge);
    //             if (dir == LinkDirection::RIGHT || dir == LinkDirection::PARTRIGHT) {
    //                 return con.fromLane > prohibitorCon.fromLane;
    //             } else {
    //                 if (myIncoming.size() == 1) {
    //                     // at off-ramp like situations, right lane should pass unless it's a bicycle lane
    //                     return bike || (con.fromLane > prohibitorCon.fromLane && !prohibitorBike);
    //                 } else {
    //                     return con.fromLane < prohibitorCon.fromLane;
    //                 }
    //             }
    //         }
    //
    //     } else {
    //         return false;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn merge_conflict(&self, from: Ptr<NBEdge>, con: &NBEdge_Connection, prohibitorFrom: Ptr<NBEdge>, prohibitorCon: &NBEdge_Connection, foes: bool) -> bool {
        todo!("port NBRequest::mergeConflict")
    }

    /// C++: `bool NBRequest::oppositeLeftTurnConflict(const NBEdge *, const NBEdge::Connection &, const NBEdge *, const NBEdge::Connection &, bool) const` [const]
    /// whether opposite left turns intersect
    // bool
    // NBRequest::oppositeLeftTurnConflict(const NBEdge* from, const NBEdge::Connection& con,
    //                                     const NBEdge* prohibitorFrom,  const NBEdge::Connection& prohibitorCon, bool foes) const {
    //     LinkDirection dir = myJunction->getDirection(from, con.toEdge);
    //     // XXX lefthand issue (solve via #4256)
    //     if (dir != LinkDirection::LEFT && dir != LinkDirection::PARTLEFT) {
    //         return false;
    //     }
    //     dir = myJunction->getDirection(prohibitorFrom, prohibitorCon.toEdge);
    //     if (dir != LinkDirection::LEFT && dir != LinkDirection::PARTLEFT) {
    //         return false;
    //     }
    //     if (from == prohibitorFrom || NBRequest::foes(from, con.toEdge, prohibitorFrom, prohibitorCon.toEdge)) {
    //         // not an opposite pair
    //         return false;
    //     }
    //
    //     const double width1 = MIN2(from->getLaneWidth(con.fromLane) / 2, OptionsCont::getOptions().getFloat("internal-junctions.vehicle-width") / 2);
    //     const double width2 = prohibitorCon.toEdge->getLaneWidth(prohibitorCon.toLane) / 2;
    //     PositionVector shape = con.shape;
    //     shape.append(con.viaShape);
    //     PositionVector otherShape = prohibitorCon.shape;
    //     otherShape.append(prohibitorCon.viaShape);
    //     if (shape.size() == 0 || otherShape.size() == 0) {
    //         // no internal lanes built
    //         return false;
    //     }
    //     const double minDV = NBEdge::firstIntersection(shape, otherShape, width1, width2);
    //     if (minDV < shape.length() - POSITION_EPS && minDV > POSITION_EPS) {
    //         // break symmetry using edge id
    //         return foes || from->getID() < prohibitorFrom->getID();
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn opposite_left_turn_conflict(&self, from: Ptr<NBEdge>, con: &NBEdge_Connection, prohibitorFrom: Ptr<NBEdge>, prohibitorCon: &NBEdge_Connection, foes: bool) -> bool {
        todo!("port NBRequest::oppositeLeftTurnConflict")
    }

    /// C++: `bool NBRequest::indirectLeftTurnConflict(const NBEdge *, const NBEdge::Connection &, const NBEdge *, const NBEdge::Connection &, bool) const` [const]
    /// whether straight and indirect left turn are in conflict
    // bool
    // NBRequest::indirectLeftTurnConflict(const NBEdge* from, const NBEdge::Connection& con,
    //                                     const NBEdge* prohibitorFrom,  const NBEdge::Connection& prohibitorCon, bool foes) const {
    //     if (from == prohibitorFrom) {
    //         if (con.toEdge == prohibitorCon.toEdge) {
    //             return false;
    //         } else if (con.indirectLeft) {
    //             LinkDirection dir = myJunction->getDirection(prohibitorFrom, prohibitorCon.toEdge);
    //             return (dir == LinkDirection::STRAIGHT);
    //         } else if (foes && prohibitorCon.indirectLeft) {
    //             LinkDirection dir = myJunction->getDirection(from, con.toEdge);
    //             return (dir == LinkDirection::STRAIGHT);
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn indirect_left_turn_conflict(&self, from: Ptr<NBEdge>, con: &NBEdge_Connection, prohibitorFrom: Ptr<NBEdge>, prohibitorCon: &NBEdge_Connection, foes: bool) -> bool {
        todo!("port NBRequest::indirectLeftTurnConflict")
    }

    /// C++: `bool NBRequest::bidiConflict(const NBEdge *, const NBEdge::Connection &, const NBEdge *, const NBEdge::Connection &, bool) const` [const]
    /// whether straight connections are in conflict via bidirectional lane use
    // bool
    // NBRequest::bidiConflict(const NBEdge* from, const NBEdge::Connection& con,
    //                         const NBEdge* prohibitorFrom,  const NBEdge::Connection& prohibitorCon, bool foes) const {
    //     if (from == prohibitorFrom) {
    //         return false;
    //     }
    //     if (isRailway(from->getPermissions())) {
    //         // railways manage right-of-way via signals
    //         return false;
    //     }
    //     if ((foes && (from->getBidiEdge() == prohibitorCon.toEdge))
    //             || prohibitorFrom->getBidiEdge() == con.toEdge) {
    //         const bool fromBidi = from->getLaneShape(con.fromLane).reverse().almostSame(
    //                                   prohibitorCon.toEdge->getLaneShape(prohibitorCon.toLane), POSITION_EPS);
    //         const bool prohibitorFromBidi = prohibitorFrom->getLaneShape(prohibitorCon.fromLane).reverse().almostSame(
    //                                             con.toEdge->getLaneShape(con.toLane), POSITION_EPS);
    //         if (!foes && fromBidi && prohibitorFromBidi) {
    //             // do not create a symmetrical conflict
    //             return false;
    //         }
    //         if (prohibitorFromBidi &&
    //                 prohibitorFrom->getLaneShape(prohibitorCon.fromLane).reverse().almostSame(
    //                     prohibitorCon.toEdge->getLaneShape(prohibitorCon.toLane), POSITION_EPS)) {
    //             // prohibitor has a bidi-turnaround
    //             return false;
    //         }
    //
    //         return fromBidi || prohibitorFromBidi;
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn bidi_conflict(&self, from: Ptr<NBEdge>, con: &NBEdge_Connection, prohibitorFrom: Ptr<NBEdge>, prohibitorCon: &NBEdge_Connection, foes: bool) -> bool {
        todo!("port NBRequest::bidiConflict")
    }

    /// C++: `bool NBRequest::hasConflict() const` [const]
    /// whether there are conflicting streams of traffic at this node
    // bool
    // NBRequest::hasConflict() const {
    //     for (int i = 0; i < (int)myFoes.size(); i++) {
    //         if (hasConflictAtLink(i)) {
    //             return true;
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn has_conflict(&self) -> bool {
        todo!("port NBRequest::hasConflict")
    }

    /// C++: `bool NBRequest::hasConflictAtLink(int) const` [const]
    /// whether there are conflicting streams of traffic for the given link index
    // bool
    // NBRequest::hasConflictAtLink(int linkIndex) const {
    //     return myFoes[linkIndex].find_first_of("1") != std::string::npos;
    // }
    #[allow(unused_variables)]
    pub fn has_conflict_at_link(&self, linkIndex: i32) -> bool {
        todo!("port NBRequest::hasConflictAtLink")
    }

    /// C++: `void NBRequest::setBlocking(NBEdge *, NBEdge *, NBEdge *, NBEdge *)` [private]
    /// sets the information that the edge from1->to1 blocks the edge from2->to2 (is higher priorised than this)
    // void
    // NBRequest::setBlocking(NBEdge* from1, NBEdge* to1,
    //                        NBEdge* from2, NBEdge* to2) {
    //     // check whether one of the links has a dead end
    //     if (to1 == nullptr || to2 == nullptr) {
    //         return;
    //     }
    //     // get the indices of both links
    //     int idx1 = getIndex(from1, to1);
    //     int idx2 = getIndex(from2, to2);
    //     if (idx1 < 0 || idx2 < 0) {
    //         return; // !!! error output? did not happend, yet
    //     }
    //     // check whether the link crossing has already been checked
    //     assert(idx1 < (int)(myIncoming.size() * myOutgoing.size()));
    //     if (myDone[idx1][idx2]) {
    //         return;
    //     }
    //     // mark the crossings as done
    //     myDone[idx1][idx2] = true;
    //     myDone[idx2][idx1] = true;
    //     // special case all-way stop
    //     if (myJunction->getType() == SumoXMLNodeType::ALLWAY_STOP) {
    //         // all ways forbid each other. Conflict resolution happens via arrival time
    //         myForbids[idx1][idx2] = true;
    //         myForbids[idx2][idx1] = true;
    //         return;
    //     }
    //     // check if one of the links is a turn; this link is always not priorised
    //     //  true for right-before-left and priority
    //     if (from1->isTurningDirectionAt(to1)) {
    //         myForbids[idx2][idx1] = true;
    //         return;
    //     }
    //     if (from2->isTurningDirectionAt(to2)) {
    //         myForbids[idx1][idx2] = true;
    //         return;
    //     }
    //     // if  there are no connections, there are no prohibitions
    //     if (from1->isConnectedTo(to1)) {
    //         if (!from2->isConnectedTo(to2)) {
    //             myForbids[idx1][idx2] = true;
    //             myForbids[idx2][idx1] = false;
    //             return;
    //         }
    //     } else {
    //         if (!from2->isConnectedTo(to2)) {
    //             myForbids[idx1][idx2] = false;
    //             myForbids[idx2][idx1] = false;
    //             return;
    //         } else {
    //             myForbids[idx1][idx2] = false;
    //             myForbids[idx2][idx1] = true;
    //             return;
    //         }
    //     }
    // #ifdef DEBUG_SETBLOCKING
    //     if (DEBUGCOND) std::cout << "setBlocking type=" << toString(myJunction->getType())
    //                                  << " bentPrio=" << myJunction->isBentPriority()
    //                                  << " 1:" << from1->getID() << "->" << to1->getID()
    //                                  << " 2:" << from2->getID() << "->" << to2->getID() << "\n";
    // #endif
    //     // check the priorities if required by node type
    //     const bool typeEqual = myJunction->getType() == SumoXMLNodeType::RIGHT_BEFORE_LEFT || myJunction->getType() == SumoXMLNodeType::LEFT_BEFORE_RIGHT;
    //     if (!typeEqual) {
    //         int from1p = from1->getJunctionPriority(myJunction);
    //         int from2p = from2->getJunctionPriority(myJunction);
    // #ifdef DEBUG_SETBLOCKING
    //         if (DEBUGCOND) {
    //             std::cout << "     p1=" << from1p << " p2=" << from2p << "\n";
    //         }
    // #endif
    //         // check if one of the connections is higher priorised when incoming into
    //         //  the junction, the connection road will yield
    //         if (from1p > from2p) {
    //             myForbids[idx1][idx2] = true;
    //             return;
    //         }
    //         if (from2p > from1p) {
    //             myForbids[idx2][idx1] = true;
    //             return;
    //         }
    //     }
    //     // straight connections prohibit turning connections if the priorities are equal
    //     // (unless the junction is a bent priority junction)
    //     LinkDirection ld1 = myJunction->getDirection(from1, to1);
    //     LinkDirection ld2 = myJunction->getDirection(from2, to2);
    //     if (!typeEqual && !myJunction->isBentPriority()) {
    // #ifdef DEBUG_SETBLOCKING
    //         if (DEBUGCOND) std::cout << "setBlocking"
    //                                      << " 1:" << from1->getID() << "->" << to1->getID()
    //                                      << " 2:" << from2->getID() << "->" << to2->getID()
    //                                      << " dir1=" << toString(ld1) << " dir2=" << toString(ld2) << "\n";
    // #endif
    //         if (ld1 == LinkDirection::STRAIGHT) {
    //             if (ld2 != LinkDirection::STRAIGHT) {
    //                 myForbids[idx1][idx2] = true;
    //                 myForbids[idx2][idx1] = false;
    //                 return;
    //             }
    //         } else {
    //             if (ld2 == LinkDirection::STRAIGHT) {
    //                 myForbids[idx1][idx2] = false;
    //                 myForbids[idx2][idx1] = true;
    //                 return;
    //             }
    //         }
    //     }
    //
    //     // check whether one of the connections is higher priorised on
    //     //  the outgoing edge when both roads are high priorised
    //     //  the connection with the lower priorised outgoing edge will lead
    //     // should be valid for priority junctions only
    //     /*
    //     if (from1p > 0 && from2p > 0) {
    //         assert(myJunction->getType() != SumoXMLNodeType::RIGHT_BEFORE_LEFT);
    //         int to1p = to1->getJunctionPriority(myJunction);
    //         int to2p = to2->getJunctionPriority(myJunction);
    //         if (to1p > to2p) {
    //             myForbids[idx1][idx2] = true;
    //             return;
    //         }
    //         if (to2p > to1p) {
    //             myForbids[idx2][idx1] = true;
    //             return;
    //         }
    //     }
    //     */
    // #ifdef DEBUG_SETBLOCKING
    //     if (DEBUGCOND) std::cout << "setBlocking1"
    //                                  << " 1:" << from1->getID() << "->" << to1->getID()
    //                                  << " 2:" << from2->getID() << "->" << to2->getID();
    //             << " relAngle=" << NBHelpers::relAngle(from1->getAngleAtNode(myJunction), from2->getAngleAtNode(myJunction))
    // #endif
    //
    //             // compute the yielding due to the right-before-left rule
    //             // (or left-before-right rule)
    //             // get the position of the incoming lanes in the junction-wheel
    //             EdgeVector::const_iterator c1 = std::find(myAll.begin(), myAll.end(), from1);
    //     NBContHelper::nextCW(myAll, c1);
    //     // go through next edges clockwise...
    //     while (*c1 != from1 && *c1 != from2) {
    //         if (*c1 == to2) {
    //             // if we encounter to2 the second one prohibits the first
    //             if (myJunction->getType() == SumoXMLNodeType::LEFT_BEFORE_RIGHT
    //                     && (ld1 != LinkDirection::LEFT || ld2 == LinkDirection::LEFT || (from1->getTurnDestination(true) != to2 &&
    //                             (ld2 != LinkDirection::RIGHT ||
    //                              fabs(NBHelpers::relAngle(from1->getAngleAtNode(myJunction),
    //                                      from2->getAngleAtNode(myJunction))) < 150))
    //                        )) {
    //                 myForbids[idx1][idx2] = true;
    // #ifdef DEBUG_SETBLOCKING
    //                 if (DEBUGCOND) {
    //                     std::cout << " case1: 2 yields\n";
    //                 }
    // #endif
    //             } else {
    //                 myForbids[idx2][idx1] = true;
    // #ifdef DEBUG_SETBLOCKING
    //                 if (DEBUGCOND) {
    //                     std::cout << " case1: 1 yields\n";
    //                 }
    // #endif
    //             }
    //             return;
    //         }
    //         NBContHelper::nextCW(myAll, c1);
    //     }
    //     // get the position of the incoming lanes in the junction-wheel
    //     EdgeVector::const_iterator c2 = std::find(myAll.begin(), myAll.end(), from2);
    //     NBContHelper::nextCW(myAll, c2);
    //     // go through next edges clockwise...
    //     while (*c2 != from2 && *c2 != from1) {
    //         if (*c2 == to1) {
    //             // if we encounter to1 the second one prohibits the first
    //             if (myJunction->getType() == SumoXMLNodeType::LEFT_BEFORE_RIGHT
    //                     && (ld2 != LinkDirection::LEFT || ld1 == LinkDirection::LEFT || (from2->getTurnDestination(true) != to1 &&
    //                             (ld1 != LinkDirection::RIGHT ||
    //                              fabs(NBHelpers::relAngle(from1->getAngleAtNode(myJunction),
    //                                      from2->getAngleAtNode(myJunction))) < 150))
    //                        )) {
    //                 myForbids[idx2][idx1] = true;
    // #ifdef DEBUG_SETBLOCKING
    //                 if (DEBUGCOND) {
    //                     std::cout << " case2: 1 yields\n";
    //                 }
    // #endif
    //             } else {
    //                 myForbids[idx1][idx2] = true;
    // #ifdef DEBUG_SETBLOCKING
    //                 if (DEBUGCOND) {
    //                     std::cout << " case2: 2 yields\n";
    //                 }
    // #endif
    //             }
    //             return;
    //         }
    //         NBContHelper::nextCW(myAll, c2);
    //     }
    // #ifdef DEBUG_SETBLOCKING
    //     if (DEBUGCOND) {
    //         std::cout << " noDecision\n";
    //     }
    // #endif
    // }
    #[allow(unused_variables)]
    pub fn set_blocking(&mut self, from1: Ptr<NBEdge>, to1: Ptr<NBEdge>, from2: Ptr<NBEdge>, to2: Ptr<NBEdge>) {
        todo!("port NBRequest::setBlocking")
    }

    /// C++: `int NBRequest::computeLaneResponse(NBEdge *, int, int, const bool)` [private]
    /// computes the response of a certain lane Returns the next link index within the junction
    // int
    // NBRequest::computeLaneResponse(NBEdge* from, int fromLane, int pos, const bool checkLaneFoes) {
    //     for (const NBEdge::Connection& c : from->getConnectionsFromLane(fromLane)) {
    //         assert(c.toEdge != 0);
    //         pos++;
    //         const std::string foes = getFoesString(from, c, checkLaneFoes);
    //         const std::string response = getResponseString(from, c, checkLaneFoes);
    //         myFoes.push_back(foes);
    //         myResponse.push_back(response);
    //         myHaveVia.push_back(c.haveVia);
    //     }
    //     return pos;
    // }
    #[allow(unused_variables)]
    pub fn compute_lane_response(&mut self, from: Ptr<NBEdge>, lane: i32, pos: i32, checkLaneFoes: bool) -> i32 {
        todo!("port NBRequest::computeLaneResponse")
    }

    /// C++: `int NBRequest::computeCrossingResponse(const NBNode::Crossing &, int)` [private]
    /// computes the response of a certain crossing Returns the next link index within the junction
    // int
    // NBRequest::computeCrossingResponse(const NBNode::Crossing& crossing, int pos) {
    //     std::string foes(myJunction->getCrossings().size(), '0');
    //     std::string response(myJunction->getCrossings().size(), '0');
    //     // conflicts with normal connections
    //     for (EdgeVector::const_reverse_iterator i = myIncoming.rbegin(); i != myIncoming.rend(); i++) {
    //         const NBEdge* from = *i;
    //         for (int j = from->getNumLanes(); j-- > 0;) {
    //             const std::vector<NBEdge::Connection>& connected = from->getConnectionsFromLane(j);
    //             const int size = (int) connected.size();
    //             for (int k = size; k-- > 0;) {
    //                 const NBEdge* to = connected[k].toEdge;
    //                 bool foe = false;
    //                 for (const NBEdge* const e : crossing.edges) {
    //                     if (e == from || e == to) {
    //                         foe = true;
    //                         break;
    //                     }
    //                 }
    //                 foes += foe ? '1' : '0';
    //                 response += mustBrakeForCrossing(myJunction, from, to, crossing) || !foe ? '0' : '1';
    //             }
    //         }
    //     }
    //     pos++;
    //     myResponse.push_back(response);
    //     myFoes.push_back(foes);
    //     myHaveVia.push_back(false);
    //     return pos;
    // }
    #[allow(unused_variables)]
    pub fn compute_crossing_response(&mut self, crossing: &NBNode_Crossing, pos: i32) -> i32 {
        todo!("port NBRequest::computeCrossingResponse")
    }

    /// C++: `std::string NBRequest::getResponseString(const NBEdge *const, const NBEdge::Connection &, const bool) const` [const, private]
    /// Writes the response of a certain link
    // std::string
    // NBRequest::getResponseString(const NBEdge* const from, const NBEdge::Connection& c, const bool checkLaneFoes) const {
    //     const NBEdge* const to = c.toEdge;
    //     const int fromLane = c.fromLane;
    //     const int toLane = c.toLane;
    //     int idx = 0;
    //     if (to != nullptr) {
    //         idx = getIndex(from, to);
    //     }
    //     std::string result;
    //     const bool zipper = myJunction->getType() == SumoXMLNodeType::ZIPPER;
    //     // crossings
    //     auto crossings = myJunction->getCrossings();
    //     for (std::vector<NBNode::Crossing*>::const_reverse_iterator i = crossings.rbegin(); i != crossings.rend(); i++) {
    //         result += mustBrakeForCrossing(myJunction, from, to, **i) ? '1' : '0';
    //     }
    //     // normal connections
    //     for (EdgeVector::const_reverse_iterator i = myIncoming.rbegin(); i != myIncoming.rend(); i++) {
    //         //const std::vector<NBEdge::Connection> &allConnections = (*i)->getConnections();
    //         int noLanes = (*i)->getNumLanes();
    //         for (int j = noLanes; j-- > 0;) {
    //             std::vector<NBEdge::Connection> connected = (*i)->getConnectionsFromLane(j);
    //             int size = (int) connected.size();
    //             for (int k = size; k-- > 0;) {
    //                 if (c.mayDefinitelyPass) {
    //                     result += '0';
    // #ifdef DEBUG_RESPONSE
    //                     if (DEBUGCOND) {
    //                         std::cout << " c=" << c.getDescription(from) << " pass\n";
    //                     }
    // #endif
    //                 } else if ((*i) == from && fromLane == j) {
    //                     // do not prohibit a connection by others from same lane
    //                     // except for indirect turns
    // #ifdef DEBUG_RESPONSE
    //                     if (DEBUGCOND) {
    //                         std::cout << " c=" << c.getDescription(from) << " prohibitC=" << connected[k].getDescription(*i)
    //                                   << " itc=" <<  indirectLeftTurnConflict(from, c, *i, connected[k], false)
    //                                   << "\n";
    //                     }
    // #endif
    //                     if (indirectLeftTurnConflict(from, c, *i, connected[k], false)) {
    //                         result += '1';
    //                     } else {
    //                         result += '0';
    //                     }
    //                 } else {
    //                     assert(connected[k].toEdge != 0);
    //                     const int idx2 = getIndex(*i, connected[k].toEdge);
    //                     assert(k < (int) connected.size());
    //                     assert(idx < (int)(myIncoming.size() * myOutgoing.size()));
    //                     assert(idx2 < (int)(myIncoming.size() * myOutgoing.size()));
    //                     // check whether the connection is prohibited by another one
    // #ifdef DEBUG_RESPONSE
    //                     if (DEBUGCOND) {
    //                         std::cout << " c=" << c.getDescription(from) << " prohibitC=" << connected[k].getDescription(*i)
    //                                   << " idx=" << idx << " idx2=" << idx2
    //                                   << " f=" << myForbids[idx2][idx]
    //                                   << " clf=" << checkLaneFoes
    //                                   << " clfbc=" << checkLaneFoesByClass(c, *i, connected[k])
    //                                   << " clfbcoop=" << checkLaneFoesByCooperation(from, c, *i, connected[k])
    //                                   << " lc=" << laneConflict(from, to, toLane, *i, connected[k].toEdge, connected[k].toLane)
    //                                   << " rtc=" << NBNode::rightTurnConflict(from, to, fromLane, *i, connected[k].toEdge, connected[k].fromLane)
    //                                   << " rtc2=" << rightTurnConflict(from, c, *i, connected[k])
    //                                   << " mc=" << mergeConflict(from, c, *i, connected[k], false)
    //                                   << " oltc=" << oppositeLeftTurnConflict(from, c, *i, connected[k], false)
    //                                   << " itc=" <<  indirectLeftTurnConflict(from, c, *i, connected[k], zipper)
    //                                   << " bc=" <<  bidiConflict(from, c, *i, connected[k], false)
    //                                   << " ec=" << (myJunction->extraConflict(c.tlLinkIndex, connected[k].tlLinkIndex) && (myForbids[idx2][idx] || myForbids[idx][idx2]))
    //                                   << " tlscc=" << myJunction->tlsStrandedConflict(from, c, *i, connected[k])
    //                                   << "\n";
    //                     }
    // #endif
    //                     const bool hasLaneConflict = (!(checkLaneFoes || checkLaneFoesByClass(c, *i, connected[k])
    //                                                     || checkLaneFoesByCooperation(from, c, *i, connected[k]))
    //                                                   || laneConflict(from, to, toLane, *i, connected[k].toEdge, connected[k].toLane));
    //                     if (((myForbids[idx2][idx] || (zipper && myForbids[idx][idx2])) && hasLaneConflict && !bidiConflict(*i, connected[k], from, c, false))
    //                             || rightTurnConflict(from, c, *i, connected[k])
    //                             || mergeConflict(from, c, *i, connected[k], zipper)
    //                             || oppositeLeftTurnConflict(from, c, *i, connected[k], zipper)
    //                             || indirectLeftTurnConflict(from, c, *i, connected[k], zipper)
    //                             || bidiConflict(from, c, *i, connected[k], false)
    //                             // the extra conflict could be specific to another connection within the same signal group
    //                             || (myJunction->extraConflict(c.tlLinkIndex, connected[k].tlLinkIndex) && (myForbids[idx2][idx] || myForbids[idx][idx2]))
    //                             || (myJunction->tlsStrandedConflict(from, c, *i, connected[k])
    //                                 && laneConflict(from, to, toLane, *i, connected[k].toEdge, connected[k].toLane)
    //                                 && !OptionsCont::getOptions().getBool("tls.ignore-internal-junction-jam"))
    //                        ) {
    //                         result += '1';
    //                     } else {
    //                         result += '0';
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_response_string(&self, from: Ptr<NBEdge>, c: &NBEdge_Connection, checkLaneFoes: bool) -> String {
        todo!("port NBRequest::getResponseString")
    }

    /// C++: `std::string NBRequest::getFoesString(NBEdge *, const NBEdge::Connection &, const bool) const` [const, private]
    /// writes which participating links are foes to the given
    // std::string
    // NBRequest::getFoesString(NBEdge* from, const NBEdge::Connection& c, const bool checkLaneFoes) const {
    //     const bool lefthand = OptionsCont::getOptions().getBool("lefthand");
    //     // remember the case when the lane is a "dead end" in the meaning that
    //     // vehicles must choose another lane to move over the following
    //     // junction
    //     // !!! move to forbidden
    //     std::string result;
    //     // crossings
    //     auto crossings = myJunction->getCrossings();
    //     for (std::vector<NBNode::Crossing*>::const_reverse_iterator i = crossings.rbegin(); i != crossings.rend(); i++) {
    //         bool foes = false;
    //         for (EdgeVector::const_iterator it_e = (**i).edges.begin(); it_e != (**i).edges.end(); ++it_e) {
    //             if ((*it_e) == from || (*it_e) == c.toEdge) {
    //                 foes = true;
    //                 break;
    //             }
    //         }
    //         result += foes ? '1' : '0';
    //     }
    //     // normal connections
    //     for (EdgeVector::const_reverse_iterator i = myIncoming.rbegin(); i != myIncoming.rend(); i++) {
    //         for (int j = (*i)->getNumLanes() - 1; j >= 0; --j) {
    //             const std::vector<NBEdge::Connection>& connected = (*i)->getConnectionsFromLane(j);
    //             int size = (int) connected.size();
    //             for (int k = size; k-- > 0;) {
    //                 const bool hasLaneConflict = (!(checkLaneFoes || checkLaneFoesByClass(c, *i, connected[k])
    //                                                 || checkLaneFoesByCooperation(from, c, *i, connected[k]))
    //                                               || laneConflict(from, c.toEdge, c.toLane, *i, connected[k].toEdge, connected[k].toLane));
    //                 if ((foes(from, c.toEdge, (*i), connected[k].toEdge) && hasLaneConflict)
    //                         || rightTurnConflict(from, c, *i, connected[k])
    //                         || myJunction->turnFoes(from, c.toEdge, c.fromLane, *i, connected[k].toEdge, connected[k].fromLane, lefthand)
    //                         || mergeConflict(from, c, *i, connected[k], true)
    //                         || oppositeLeftTurnConflict(from, c, *i, connected[k], true)
    //                         || indirectLeftTurnConflict(from, c, *i, connected[k], true)
    //                         || bidiConflict(from, c, *i, connected[k], true)
    //                    ) {
    //                     result += '1';
    //                 } else {
    //                     result += '0';
    //                 }
    //             }
    //         }
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_foes_string(&self, from: Ptr<NBEdge>, c: &NBEdge_Connection, checkLaneFoes: bool) -> String {
        todo!("port NBRequest::getFoesString")
    }

    /// C++: `int NBRequest::getIndex(const NBEdge *const, const NBEdge *const) const` [const, private]
    /// Returns the index to the internal combination container for the given edge combination
    // int
    // NBRequest::getIndex(const NBEdge* const from, const NBEdge* const to) const {
    //     EdgeVector::const_iterator fp = std::find(myIncoming.begin(), myIncoming.end(), from);
    //     EdgeVector::const_iterator tp = std::find(myOutgoing.begin(), myOutgoing.end(), to);
    //     if (fp == myIncoming.end() || tp == myOutgoing.end()) {
    //         return -1;
    //     }
    //     // compute the index
    //     return (int)(distance(myIncoming.begin(), fp) * myOutgoing.size() + distance(myOutgoing.begin(), tp));
    // }
    #[allow(unused_variables)]
    pub fn get_index(&self, from: Ptr<NBEdge>, to: Ptr<NBEdge>) -> i32 {
        todo!("port NBRequest::getIndex")
    }

    /// C++: `int NBRequest::distanceCounterClockwise(NBEdge *, NBEdge *)` [private]
    /// returns the distance between the incoming (from) and the outgoing (to) edge clockwise in edges
    // int
    // NBRequest::distanceCounterClockwise(NBEdge* from, NBEdge* to) {
    //     EdgeVector::const_iterator p = std::find(myAll.begin(), myAll.end(), from);
    //     int ret = 0;
    //     do {
    //         ret++;
    //         if (p == myAll.begin()) {
    //             p = myAll.end();
    //         }
    //         p--;
    //     } while (*p != to);
    //     return ret;
    // }
    #[allow(unused_variables)]
    pub fn distance_counter_clockwise(&mut self, from: Ptr<NBEdge>, to: Ptr<NBEdge>) -> i32 {
        todo!("port NBRequest::distanceCounterClockwise")
    }

    /// C++: `void NBRequest::computeRightOutgoingLinkCrossings(NBEdge *, NBEdge *)` [private]
    /// computes the relationships between links outgoing right of the given link */
    // void
    // NBRequest::computeRightOutgoingLinkCrossings(NBEdge* from, NBEdge* to) {
    //     EdgeVector::const_iterator pfrom = std::find(myAll.begin(), myAll.end(), from);
    //     while (*pfrom != to) {
    //         NBContHelper::nextCCW(myAll, pfrom);
    //         if ((*pfrom)->getToNode() == myJunction) {
    //             EdgeVector::const_iterator pto = std::find(myAll.begin(), myAll.end(), to);
    //             while (*pto != from) {
    //                 if (!((*pto)->getToNode() == myJunction)) {
    //                     setBlocking(from, to, *pfrom, *pto);
    //                 }
    //                 NBContHelper::nextCCW(myAll, pto);
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn compute_right_outgoing_link_crossings(&mut self, from: Ptr<NBEdge>, to: Ptr<NBEdge>) {
        todo!("port NBRequest::computeRightOutgoingLinkCrossings")
    }

    /// C++: `void NBRequest::computeLeftOutgoingLinkCrossings(NBEdge *, NBEdge *)` [private]
    /// computes the relationships between links outgoing left of the given link
    // void
    // NBRequest::computeLeftOutgoingLinkCrossings(NBEdge* from, NBEdge* to) {
    //     EdgeVector::const_iterator pfrom = std::find(myAll.begin(), myAll.end(), from);
    //     while (*pfrom != to) {
    //         NBContHelper::nextCW(myAll, pfrom);
    //         if ((*pfrom)->getToNode() == myJunction) {
    //             EdgeVector::const_iterator pto = std::find(myAll.begin(), myAll.end(), to);
    //             while (*pto != from) {
    //                 if (!((*pto)->getToNode() == myJunction)) {
    //                     setBlocking(from, to, *pfrom, *pto);
    //                 }
    //                 NBContHelper::nextCW(myAll, pto);
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn compute_left_outgoing_link_crossings(&mut self, from: Ptr<NBEdge>, to: Ptr<NBEdge>) {
        todo!("port NBRequest::computeLeftOutgoingLinkCrossings")
    }

    /// C++: `void NBRequest::resetSignalised()` [private]
    // void
    // NBRequest::resetSignalised() {
    //     // go through possible prohibitions
    //     for (EdgeVector::const_iterator i11 = myIncoming.begin(); i11 != myIncoming.end(); i11++) {
    //         int noLanesEdge1 = (*i11)->getNumLanes();
    //         for (int j1 = 0; j1 < noLanesEdge1; j1++) {
    //             std::vector<NBEdge::Connection> el1 = (*i11)->getConnectionsFromLane(j1);
    //             for (std::vector<NBEdge::Connection>::iterator i12 = el1.begin(); i12 != el1.end(); ++i12) {
    //                 int idx1 = getIndex((*i11), (*i12).toEdge);
    //                 if (idx1 < 0) {
    //                     continue;
    //                 }
    //                 // go through possibly prohibited
    //                 for (EdgeVector::const_iterator i21 = myIncoming.begin(); i21 != myIncoming.end(); i21++) {
    //                     int noLanesEdge2 = (*i21)->getNumLanes();
    //                     for (int j2 = 0; j2 < noLanesEdge2; j2++) {
    //                         std::vector<NBEdge::Connection> el2 = (*i21)->getConnectionsFromLane(j2);
    //                         for (std::vector<NBEdge::Connection>::iterator i22 = el2.begin(); i22 != el2.end(); i22++) {
    //                             int idx2 = getIndex((*i21), (*i22).toEdge);
    //                             if (idx2 < 0) {
    //                                 continue;
    //                             }
    //                             // check
    //                             // same incoming connections do not prohibit each other
    //                             if ((*i11) == (*i21)) {
    //                                 myForbids[idx1][idx2] = false;
    //                                 myForbids[idx2][idx1] = false;
    // #ifdef DEBUG_SETBLOCKING
    //                                 if (DEBUGCOND) std::cout << "resetSignalised both"
    //                                                              << " 1:" << (*i11)->getID() << "->" << (*i12).toEdge->getID()
    //                                                              << " 2:" << (*i21)->getID() << "->" << (*i22).toEdge->getID()
    //                                                              << "\n";
    // #endif
    //                                 continue;
    //                             }
    //                             // check other
    //                             // if both are non-signalised or both are signalised
    //                             if (((*i12).tlID == "" && (*i22).tlID == "")
    //                                     ||
    //                                     ((*i12).tlID != "" && (*i22).tlID != "")) {
    //                                 // do nothing
    //                                 continue;
    //                             }
    //                             // supposing, we don not have to
    //                             //  brake if we are no foes
    //                             if (!foes(*i11, (*i12).toEdge, *i21, (*i22).toEdge)) {
    //                                 continue;
    //                             }
    //                             // otherwise:
    //                             //  the non-signalised must break
    //                             if ((*i12).tlID != "") {
    //                                 myForbids[idx1][idx2] = true;
    //                                 myForbids[idx2][idx1] = false;
    // #ifdef DEBUG_SETBLOCKING
    //                                 if (DEBUGCOND) std::cout << "resetSignalised:2 yields"
    //                                                              << " 1:" << (*i11)->getID() << "->" << (*i12).toEdge->getID()
    //                                                              << " 2:" << (*i21)->getID() << "->" << (*i22).toEdge->getID() << "\n";
    // #endif
    //                             } else {
    //                                 myForbids[idx1][idx2] = false;
    //                                 myForbids[idx2][idx1] = true;
    // #ifdef DEBUG_SETBLOCKING
    //                                 if (DEBUGCOND) std::cout << "resetSignalised:1 yields"
    //                                                              << " 1:" << (*i11)->getID() << "->" << (*i12).toEdge->getID()
    //                                                              << " 2:" << (*i21)->getID() << "->" << (*i22).toEdge->getID() << "\n";
    // #endif
    //                             }
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn reset_signalised(&mut self) {
        todo!("port NBRequest::resetSignalised")
    }

    /// C++: `void NBRequest::resetCooperating()` [private]
    /// reset foes it the number of lanes matches (or exceeds) the number of incoming connections for an edge
    // void
    // NBRequest::resetCooperating() {
    //     // map from edge to number of incoming connections
    //     std::map<const NBEdge*, int> incomingCount; // initialized to 0
    //     // map from edge to indices of approached lanes
    //     std::map<const NBEdge*, std::set<int> > approachedLanes;
    //     // map from edge to list of incoming edges
    //     std::map<const NBEdge*, std::vector<const NBEdge*> > incomingEdges;
    //     for (const NBEdge* const e : myIncoming) {
    //         for (const NBEdge::Connection& con : e->getConnections()) {
    //             incomingCount[con.toEdge]++;
    //             approachedLanes[con.toEdge].insert(con.toLane);
    //             incomingEdges[con.toEdge].push_back(e);
    //         }
    //     }
    //     for (std::map<const NBEdge*, int>::iterator it = incomingCount.begin(); it != incomingCount.end(); ++it) {
    //         const NBEdge* const to = it->first;
    //         // we cannot test against to->getNumLanes() since not all lanes may be used
    //         if ((int)approachedLanes[to].size() >= it->second) {
    //             const std::vector<const NBEdge*>& incoming = incomingEdges[to];
    //             // make these connections mutually unconflicting
    //             for (const NBEdge* const e1 : incoming) {
    //                 for (const NBEdge* const e2 : incoming) {
    //                     myForbids[getIndex(e1, to)][getIndex(e2, to)] = false;
    // #ifdef DEBUG_SETBLOCKING
    //                     if (DEBUGCOND) std::cout << "resetCooperating"
    //                                                  << " 1:" << e1->getID() << "->" << to->getID()
    //                                                  << " 2:" << e2->getID() << "->" << to->getID()
    //                                                  << "\n";
    // #endif
    //                 }
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn reset_cooperating(&mut self) {
        todo!("port NBRequest::resetCooperating")
    }

    /// C++: `bool NBRequest::checkLaneFoesByClass(const NBEdge::Connection &, const NBEdge *, const NBEdge::Connection &) const` [const, private]
    /// whether the given connections must be checked for lane conflicts due to the vClasses involved
    // bool
    // NBRequest::checkLaneFoesByClass(const NBEdge::Connection& con,
    //                                 const NBEdge* prohibitorFrom,  const NBEdge::Connection& prohibitorCon) const {
    //     if (con.toEdge != prohibitorCon.toEdge) {
    //         return false;
    //     }
    //     SVCPermissions svc = con.toEdge->getPermissions(con.toLane);
    //     SVCPermissions svc2 = prohibitorFrom->getPermissions(prohibitorCon.fromLane) & prohibitorCon.toEdge->getPermissions(prohibitorCon.toLane);
    //     // check for lane level conflict if the only common classes are bicycles or pedestrians
    //     return ((svc & svc2 & ~(SVC_BICYCLE | SVC_PEDESTRIAN)) == 0
    //             // or if the connection is to a dedicated lane whereas the prohibitor is a "general" lane
    //             || (((svc & SVC_PASSENGER) == 0) && ((svc2 & SVC_PASSENGER) != 0)));
    // }
    #[allow(unused_variables)]
    pub fn check_lane_foes_by_class(&self, con: &NBEdge_Connection, prohibitorFrom: Ptr<NBEdge>, prohibitorCon: &NBEdge_Connection) -> bool {
        todo!("port NBRequest::checkLaneFoesByClass")
    }

    /// C++: `bool NBRequest::checkLaneFoesByCooperation(const NBEdge *, const NBEdge::Connection &, const NBEdge *, const NBEdge::Connection &) const` [const, private]
    /// whether the given connections must be checked for lane conflicts due to disjunct target lanes
    // bool
    // NBRequest::checkLaneFoesByCooperation(const NBEdge* from, const NBEdge::Connection& con,
    //                                       const NBEdge* prohibitorFrom,  const NBEdge::Connection& prohibitorCon) const {
    //     if (con.toEdge != prohibitorCon.toEdge) {
    //         return false;
    //     }
    //     // if from and prohibitorFrom target distinct lanes for all their
    //     // connections to the common target edge, cooperation is possible
    //     // (and should always happen unless the connections cross for some byzantine reason)
    //
    //     std::set<int> fromTargetLanes;
    //     for (const NBEdge::Connection& c : from->getConnections()) {
    //         if (c.toEdge == con.toEdge) {
    //             fromTargetLanes.insert(c.toLane);
    //         }
    //     }
    //     for (const NBEdge::Connection& c : prohibitorFrom->getConnections()) {
    //         if (c.toEdge == con.toEdge && fromTargetLanes.count(c.toLane) != 0) {
    //             //std::cout << " con=" << con->getDescription(from) << " foe=" << prohibitorCon.getDescription(prohibitorFrom)
    //             //    << " no cooperation (targets=" << joinToString(fromTargetLanes, ' ') << " index=" << c.toLane << "\n";
    //             return false;
    //         }
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn check_lane_foes_by_cooperation(&self, from: Ptr<NBEdge>, con: &NBEdge_Connection, prohibitorFrom: Ptr<NBEdge>, prohibitorCon: &NBEdge_Connection) -> bool {
        todo!("port NBRequest::checkLaneFoesByCooperation")
    }

    /// C++: `bool NBRequest::laneConflict(const NBEdge *, const NBEdge *, int, const NBEdge *, const NBEdge *, int) const` [const, private]
    /// return whether the given laneToLane connections prohibit each other under the assumption that the edge2edge connections are in conflict
    // bool
    // NBRequest::laneConflict(const NBEdge* from, const NBEdge* to, int toLane,
    //                         const NBEdge* prohibitorFrom, const NBEdge* prohibitorTo, int prohibitorToLane) const {
    //     if (to != prohibitorTo) {
    //         return true;
    //     }
    //     // since we know that the edge2edge connections are in conflict, the only
    //     // situation in which the lane2lane connections can be conflict-free is, if
    //     // they target the same edge but do not cross each other
    //     double angle = NBHelpers::relAngle(
    //                        from->getAngleAtNode(from->getToNode()), to->getAngleAtNode(to->getFromNode()));
    //     if (angle == 180) {
    //         angle = -180; // turnarounds are left turns
    //     }
    //     const double prohibitorAngle = NBHelpers::relAngle(
    //                                        prohibitorFrom->getAngleAtNode(prohibitorFrom->getToNode()), to->getAngleAtNode(to->getFromNode()));
    //     const bool rightOfProhibitor = prohibitorFrom->isTurningDirectionAt(to)
    //                                    || (angle > prohibitorAngle && !from->isTurningDirectionAt(to));
    //     return rightOfProhibitor ? toLane >= prohibitorToLane : toLane <= prohibitorToLane;
    // }
    #[allow(unused_variables)]
    pub fn lane_conflict(&self, from: Ptr<NBEdge>, to: Ptr<NBEdge>, toLane: i32, prohibitorFrom: Ptr<NBEdge>, prohibitorTo: Ptr<NBEdge>, prohibitorToLane: i32) -> bool {
        todo!("port NBRequest::laneConflict")
    }

    /// C++: `int NBRequest::numLinks() const` [const, private]
    /// return to total number of edge-to-edge connections of this request-logic
    // int
    // NBRequest::numLinks() const {
    //     return (int)(myIncoming.size() * myOutgoing.size() + myJunction->getCrossings().size());
    // }
    #[allow(unused_variables)]
    pub fn num_links(&self) -> i32 {
        todo!("port NBRequest::numLinks")
    }

    /// C++: `NBRequest & NBRequest::operator=(const NBRequest &)` [private]
    /// Invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &NBRequest) -> &mut NBRequest {
        todo!("port NBRequest::operator=")
    }

}

/// C++: `std::ostream &(std::ostream &, const NBRequest &) operator<<`
// std::ostream&
// operator<<(std::ostream& os, const NBRequest& r) {
//     int variations = r.numLinks();
//     for (int i = 0; i < variations; i++) {
//         os << i << ' ';
//         for (int j = 0; j < variations; j++) {
//             if (r.myForbids[i][j]) {
//                 os << '1';
//             } else {
//                 os << '0';
//             }
//         }
//         os << std::endl;
//     }
//     os << std::endl;
//     return os;
// }
#[allow(unused_variables)]
pub fn shl_op(os: &mut Unmapped /* std::ostream */, r: &NBRequest) -> &'static mut Unmapped /* std::ostream */ {
    todo!("port operator<<")
}

