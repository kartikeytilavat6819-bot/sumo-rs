//! Port of netbuild/NBConnection.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `NBConnection` (netbuild/NBConnection.h)
#[derive(Clone)]
pub struct NBConnection {
    /// The from- and the to-edges
    pub myFrom: Ptr<NBEdge>,
    /// The from- and the to-edges
    pub myTo: Ptr<NBEdge>,
    /// The names of both edges, needed for verification of validity
    pub myFromID: String,
    /// The names of both edges, needed for verification of validity
    pub myToID: String,
    /// The lanes; may be -1 if no certain lane was specified
    pub myFromLane: i32,
    /// The lanes; may be -1 if no certain lane was specified
    pub myToLane: i32,
    pub myTlIndex: i32,
    /// The index of the internal junction within the controlling traffic light (optional)
    pub myTlIndex2: i32,
}

// C++ static members of NBConnection (port as module-level statics, see AGENTS.md):
//   static const int NBConnection::InvalidTlIndex;
//   static const NBConnection NBConnection::InvalidConnection;

// C++ `NBConnection::operator==` (NBConnection.cpp): same fields, same order.
impl PartialEq for NBConnection {
    fn eq(&self, c: &Self) -> bool {
        self.myFrom == c.myFrom
            && self.myTo == c.myTo
            && self.myFromID == c.myFromID
            && self.myToID == c.myToID
            && self.myFromLane == c.myFromLane
            && self.myToLane == c.myToLane
            && self.myTlIndex == c.myTlIndex
            && self.myTlIndex2 == c.myTlIndex2
    }
}
impl Eq for NBConnection {}

// C++ friend `operator<(const NBConnection&, const NBConnection&)` (NBConnection.cpp).
// Ordering uses only the IDs and lanes, exactly as the C++ does.
impl PartialOrd for NBConnection {
    fn partial_cmp(&self, o: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(o))
    }
}
impl Ord for NBConnection {
    fn cmp(&self, c2: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering;
        if self.myFromID != c2.myFromID {
            return self.myFromID.cmp(&c2.myFromID);
        }
        if self.myToID != c2.myToID {
            return self.myToID.cmp(&c2.myToID);
        }
        if self.myFromLane != c2.myFromLane {
            return self.myFromLane.cmp(&c2.myFromLane);
        }
        if self.myToLane < c2.myToLane {
            Ordering::Less
        } else if self.myToLane > c2.myToLane {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl NBConnection {
    /// C++: `void NBConnection::NBConnection(NBEdge *, NBEdge *)`
    /// Constructor
    // NBConnection::NBConnection(NBEdge* from, NBEdge* to) :
    //     myFrom(from), myTo(to),
    //     myFromID(from->getID()), myToID(to->getID()),
    //     myFromLane(-1), myToLane(-1),
    //     myTlIndex(InvalidTlIndex),
    //     myTlIndex2(InvalidTlIndex) {
    // }
    #[allow(unused_variables)]
    pub fn new(from: Ptr<NBEdge>, to: Ptr<NBEdge>) -> Self {
        todo!("port NBConnection::NBConnection")
    }

    /// C++: `void NBConnection::NBConnection(NBEdge *, int, NBEdge *, int, int, int)`
    /// Constructor
    // NBConnection::NBConnection(NBEdge* from, int fromLane,
    //                            NBEdge* to, int toLane, int tlIndex, int tlIndex2) :
    //     myFrom(from), myTo(to),
    //     myFromLane(fromLane), myToLane(toLane),
    //     myTlIndex(tlIndex),
    //     myTlIndex2(tlIndex2) {
    //     /* @todo what should we assert here?
    //     assert(myFromLane<0||from->getNumLanes()>(int) myFromLane);
    //     assert(myToLane<0||to->getNumLanes()>(int) myToLane);
    //     */
    //     myFromID = from != nullptr ? from->getID() : "";
    //     myToID = to != nullptr ? to->getID() : "";
    // }
    #[allow(unused_variables)]
    pub fn new_2(from: Ptr<NBEdge>, fromLane: i32, to: Ptr<NBEdge>, toLane: i32, tlIndex: i32, tlIndex2: i32) -> Self {
        todo!("port NBConnection::NBConnection")
    }

    /// C++: `void NBConnection::NBConnection(const std::string &, NBEdge *, const std::string &, NBEdge *)`
    /// Constructor
    // NBConnection::NBConnection(const std::string& fromID, NBEdge* from,
    //                            const std::string& toID, NBEdge* to) :
    //     myFrom(from), myTo(to),
    //     myFromID(fromID), myToID(toID),
    //     myFromLane(-1), myToLane(-1),
    //     myTlIndex(InvalidTlIndex),
    //     myTlIndex2(InvalidTlIndex) {
    // }
    #[allow(unused_variables)]
    pub fn new_3(fromID: &str, from: Ptr<NBEdge>, toID: &str, to: Ptr<NBEdge>) -> Self {
        todo!("port NBConnection::NBConnection")
    }

    /// C++: `void NBConnection::NBConnection(const NBConnection &)`
    /// Constructor
    // NBConnection::NBConnection(const NBConnection& c) :
    //     myFrom(c.myFrom), myTo(c.myTo),
    //     myFromID(c.myFromID), myToID(c.myToID),
    //     myFromLane(c.myFromLane), myToLane(c.myToLane),
    //     myTlIndex(c.myTlIndex),
    //     myTlIndex2(c.myTlIndex2) {
    // }
    #[allow(unused_variables)]
    pub fn new_4(c: &NBConnection) -> Self {
        todo!("port NBConnection::NBConnection")
    }

    /// C++: `void NBConnection::~NBConnection()` [virtual]
    /// Destructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port NBConnection::~NBConnection")
    }

    /// C++: `NBEdge * NBConnection::getFrom() const` [const]
    /// returns the from-edge (start of the connection)
    // NBEdge*
    // NBConnection::getFrom() const {
    //     return myFrom;
    // }
    #[allow(unused_variables)]
    pub fn get_from(&self) -> Ptr<NBEdge> {
        todo!("port NBConnection::getFrom")
    }

    /// C++: `NBEdge * NBConnection::getTo() const` [const]
    /// returns the to-edge (end of the connection)
    // NBEdge*
    // NBConnection::getTo() const {
    //     return myTo;
    // }
    #[allow(unused_variables)]
    pub fn get_to(&self) -> Ptr<NBEdge> {
        todo!("port NBConnection::getTo")
    }

    /// C++: `bool NBConnection::replaceFrom(NBEdge *, NBEdge *)`
    /// replaces the from-edge by the one given
    // bool
    // NBConnection::replaceFrom(NBEdge* which, NBEdge* by) {
    //     if (myFrom == which) {
    //         myFrom = by;
    //         if (myFrom != nullptr) {
    //             myFromID = myFrom->getID();
    //         } else {
    //             myFromID = "invalidFrom";
    //         }
    //         return true;
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn replace_from(&mut self, which: Ptr<NBEdge>, by: Ptr<NBEdge>) -> bool {
        todo!("port NBConnection::replaceFrom")
    }

    /// C++: `bool NBConnection::replaceFrom(NBEdge *, int, NBEdge *, int)`
    /// replaces the from-edge by the one given
    // bool
    // NBConnection::replaceFrom(NBEdge* which, int whichLane,
    //                           NBEdge* by, int byLane) {
    //     if (myFrom == which && (myFromLane == whichLane || myFromLane < 0 || whichLane < 0)) {
    //         myFrom = by;
    //         if (myFrom != nullptr) {
    //             myFromID = myFrom->getID();
    //         } else {
    //             myFromID = "invalidFrom";
    //         }
    //         if (byLane >= 0) {
    //             myFromLane = byLane;
    //         }
    //         return true;
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn replace_from_2(&mut self, which: Ptr<NBEdge>, whichLane: i32, by: Ptr<NBEdge>, byLane: i32) -> bool {
        todo!("port NBConnection::replaceFrom")
    }

    /// C++: `bool NBConnection::replaceTo(NBEdge *, NBEdge *)`
    /// replaces the to-edge by the one given
    // bool
    // NBConnection::replaceTo(NBEdge* which, NBEdge* by) {
    //     if (myTo == which) {
    //         myTo = by;
    //         if (myTo != nullptr) {
    //             myToID = myTo->getID();
    //         } else {
    //             myToID = "invalidTo";
    //         }
    //         return true;
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn replace_to(&mut self, which: Ptr<NBEdge>, by: Ptr<NBEdge>) -> bool {
        todo!("port NBConnection::replaceTo")
    }

    /// C++: `bool NBConnection::replaceTo(NBEdge *, int, NBEdge *, int)`
    /// replaces the to-edge by the one given
    // bool
    // NBConnection::replaceTo(NBEdge* which, int whichLane,
    //                         NBEdge* by, int byLane) {
    //     if (myTo == which && (myToLane == whichLane || myFromLane < 0 || whichLane < 0)) {
    //         myTo = by;
    //         if (myTo != nullptr) {
    //             myToID = myTo->getID();
    //         } else {
    //             myToID = "invalidTo";
    //         }
    //         if (byLane >= 0) {
    //             myToLane = byLane;
    //         }
    //         return true;
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn replace_to_2(&mut self, which: Ptr<NBEdge>, whichLane: i32, by: Ptr<NBEdge>, byLane: i32) -> bool {
        todo!("port NBConnection::replaceTo")
    }

    /// C++: `void NBConnection::shiftLaneIndex(NBEdge *, int, int)`
    /// patches lane indices refering to the given edge and above the threshold by the given offset
    // void
    // NBConnection::shiftLaneIndex(NBEdge* edge, int offset, int threshold) {
    //     if (myFrom == edge && myFromLane > threshold) {
    //         myFromLane += offset;
    //     } else if (myTo == edge && myToLane > threshold) {
    //         myToLane += offset;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn shift_lane_index(&mut self, edge: Ptr<NBEdge>, offset: i32, threshold: i32) {
        todo!("port NBConnection::shiftLaneIndex")
    }

    /// C++: `bool NBConnection::check(const NBEdgeCont &)`
    /// checks whether the edges are still valid
    // bool
    // NBConnection::check(const NBEdgeCont& ec) {
    //     myFrom = checkFrom(ec);
    //     myTo = checkTo(ec);
    //     return myFrom != nullptr && myTo != nullptr;
    // }
    #[allow(unused_variables)]
    pub fn check(&mut self, ec: &NBEdgeCont) -> bool {
        todo!("port NBConnection::check")
    }

    /// C++: `int NBConnection::getFromLane() const` [const]
    /// returns the from-lane
    // int
    // NBConnection::getFromLane() const {
    //     return myFromLane;
    // }
    #[allow(unused_variables)]
    pub fn get_from_lane(&self) -> i32 {
        todo!("port NBConnection::getFromLane")
    }

    /// C++: `int NBConnection::getToLane() const` [const]
    /// returns the to-lane
    // int
    // NBConnection::getToLane() const {
    //     return myToLane;
    // }
    #[allow(unused_variables)]
    pub fn get_to_lane(&self) -> i32 {
        todo!("port NBConnection::getToLane")
    }

    /// C++: `int NBConnection::getTLIndex() const` [const]
    /// returns the index within the controlling tls or InvalidTLIndex if this link is unontrolled
    // int getTLIndex() const {
    //         return myTlIndex;
    //     }
    #[allow(unused_variables)]
    pub fn get_tl_index(&self) -> i32 {
        todo!("port NBConnection::getTLIndex")
    }

    /// C++: `int NBConnection::getTLIndex2() const` [const]
    // int getTLIndex2() const {
    //         return myTlIndex2;
    //     }
    #[allow(unused_variables)]
    pub fn get_tl_index2(&self) -> i32 {
        todo!("port NBConnection::getTLIndex2")
    }

    /// C++: `void NBConnection::setTLIndex(int)`
    // void setTLIndex(int tlIndex) {
    //         myTlIndex = tlIndex;
    //     }
    #[allow(unused_variables)]
    pub fn set_tl_index(&mut self, tlIndex: i32) {
        todo!("port NBConnection::setTLIndex")
    }

    /// C++: `void NBConnection::setTLIndex2(int)`
    // void setTLIndex2(int tlIndex) {
    //         myTlIndex2 = tlIndex;
    //     }
    #[allow(unused_variables)]
    pub fn set_tl_index2(&mut self, tlIndex: i32) {
        todo!("port NBConnection::setTLIndex2")
    }

    /// C++: `std::string NBConnection::getID() const` [const]
    /// returns the id of the connection (!!! not really pretty)
    // std::string
    // NBConnection::getID() const {
    //     std::stringstream str;
    //     str << myFromID << "_" << myFromLane << "->" << myToID << "_" << myToLane;
    //     return str.str();
    // }
    #[allow(unused_variables)]
    pub fn get_id(&self) -> String {
        todo!("port NBConnection::getID")
    }

    /// C++: `bool NBConnection::operator==(const NBConnection &) const` [const]
    /// Comparison operator
    // bool
    // NBConnection::operator==(const NBConnection& c) const {
    //     return (myFrom    == c.myFrom     && myTo    == c.myTo &&
    //             myFromID  == c.myFromID   && myToID  == c.myToID &&
    //             myFromLane == c.myFromLane && myToLane == c.myToLane &&
    //             myTlIndex == c.myTlIndex &&
    //             myTlIndex2 == c.myTlIndex2);
    // }
    #[allow(unused_variables)]
    pub fn eq_op(&self, c: &NBConnection) -> bool {
        todo!("port NBConnection::operator==")
    }

    /// C++: `bool NBConnection::operator!=(const NBConnection &) const` [const]
    /// Comparison operator
    // bool operator!=(const NBConnection& c) const {
    //         return !(*this == c);
    //     }
    #[allow(unused_variables)]
    pub fn ne_op(&self, c: &NBConnection) -> bool {
        todo!("port NBConnection::operator!=")
    }

    /// C++: `NBEdge * NBConnection::checkFrom(const NBEdgeCont &)` [private]
    /// Checks whether the from-edge is still valid
    // NBEdge*
    // NBConnection::checkFrom(const NBEdgeCont& ec) {
    //     NBEdge* e = ec.retrieve(myFromID);
    //     // ok, the edge was not changed
    //     if (e == myFrom) {
    //         return myFrom;
    //     }
    //     // try to get the edge
    //     return ec.retrievePossiblySplit(myFromID, myToID, true);
    // }
    #[allow(unused_variables)]
    pub fn check_from(&mut self, ec: &NBEdgeCont) -> Ptr<NBEdge> {
        todo!("port NBConnection::checkFrom")
    }

    /// C++: `NBEdge * NBConnection::checkTo(const NBEdgeCont &)` [private]
    /// Checks whether the to-edge is still valid
    // NBEdge*
    // NBConnection::checkTo(const NBEdgeCont& ec) {
    //     NBEdge* e = ec.retrieve(myToID);
    //     // ok, the edge was not changed
    //     if (e == myTo) {
    //         return myTo;
    //     }
    //     // try to get the edge
    //     return ec.retrievePossiblySplit(myToID, myFromID, false);
    // }
    #[allow(unused_variables)]
    pub fn check_to(&mut self, ec: &NBEdgeCont) -> Ptr<NBEdge> {
        todo!("port NBConnection::checkTo")
    }

}

/// C++: `bool (const NBConnection &, const NBConnection &) operator<`
// bool
// operator<(const NBConnection& c1, const NBConnection& c2) {
//     if (c1.myFromID   != c2.myFromID) {
//         return c1.myFromID   < c2.myFromID;
//     }
//     if (c1.myToID     != c2.myToID) {
//         return c1.myToID     < c2.myToID;
//     }
//     if (c1.myFromLane != c2.myFromLane) {
//         return c1.myFromLane < c2.myFromLane;
//     }
//     return c1.myToLane < c2.myToLane;
// }
#[allow(unused_variables)]
pub fn lt_op(c1: &NBConnection, c2: &NBConnection) -> bool {
    todo!("port operator<")
}

/// C++: `std::ostream &(std::ostream &, const NBConnection &) operator<<`
// std::ostream&
// operator<<(std::ostream& os, const NBConnection& c) {
//     os
//             << "Con(from=" << Named::getIDSecure(c.getFrom())
//             << " fromLane=" << c.getFromLane()
//             << " to=" << Named::getIDSecure(c.getTo())
//             << " toLane=" << c.getToLane()
//             << " tlIndex=" << c.getTLIndex()
//             << ")";
//     return os;
// }
#[allow(unused_variables)]
pub fn shl_op(os: &mut Unmapped /* std::ostream */, c: &NBConnection) -> &'static mut Unmapped /* std::ostream */ {
    todo!("port operator<<")
}

