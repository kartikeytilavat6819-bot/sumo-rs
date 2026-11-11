//! Port of netimport/vissim/tempstructs/NIVissimTL.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `std::map<int, NIVissimTL *> NIVissimTL::DictType`
pub type NIVissimTL_DictType = BTreeMap<i32, Ptr<NIVissimTL>>;

/// C++ typedef `std::map<int, SGroupDictType> NIVissimTL::GroupDictType`
pub type NIVissimTL_GroupDictType = BTreeMap<i32, NIVissimTL_SGroupDictType>;

/// C++ typedef `std::map<int, NIVissimTLSignalGroup *> NIVissimTL::SGroupDictType`
pub type NIVissimTL_SGroupDictType = BTreeMap<i32, Ptr<NIVissimTL_NIVissimTLSignalGroup>>;

/// C++ typedef `std::map<int, NIVissimTLSignal *> NIVissimTL::SSignalDictType`
pub type NIVissimTL_SSignalDictType = BTreeMap<i32, Ptr<NIVissimTL_NIVissimTLSignal>>;

/// C++ typedef `std::map<int, SSignalDictType> NIVissimTL::SignalDictType`
pub type NIVissimTL_SignalDictType = BTreeMap<i32, NIVissimTL_SSignalDictType>;

/// C++ class `NIVissimTL` (netimport/vissim/tempstructs/NIVissimTL.h)
pub struct NIVissimTL {
    pub myID: i32,
    pub myName: String,
    pub myAbsDuration: SUMOTime,
    pub myOffset: SUMOTime,
    pub myCurrentGroup: Ptr<NIVissimTL_NIVissimTLSignalGroup>,
    pub myType: String,
}

// C++ static members of NIVissimTL (port as module-level statics, see AGENTS.md):
//   static DictType NIVissimTL::myDict;
thread_local! {
    static MY_DICT: std::cell::RefCell<NIVissimTL_DictType> = std::cell::RefCell::new(BTreeMap::new());
}

impl NIVissimTL {
    /// C++: `void NIVissimTL::NIVissimTL(int, const std::string &, const std::string &, SUMOTime, SUMOTime)`
    // NIVissimTL::NIVissimTL(int id, const std::string& type,
    //                        const std::string& name, SUMOTime absdur,
    //                        SUMOTime offset)
    //     : myID(id), myName(name), myAbsDuration(absdur), myOffset(offset),
    //       myCurrentGroup(nullptr), myType(type)
    //
    // {}
    #[allow(unused_variables)]
    pub fn new(id: i32, type_: &str, name: &str, absdur: SUMOTime, offset: SUMOTime) -> Self {
        Self {
            myID: id,
            myName: name.to_string(),
            myAbsDuration: absdur,
            myOffset: offset,
            myCurrentGroup: Ptr::null(),
            myType: type_.to_string(),
        }
    }

    /// C++: `void NIVissimTL::~NIVissimTL()`
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor is implicit; nothing to release.
    }

    /// C++: `std::string NIVissimTL::getType() const` [const]
    // std::string
    // NIVissimTL::getType() const {
    //     return myType;
    // }
    #[allow(unused_variables)]
    pub fn get_type(&self) -> String {
        self.myType.clone()
    }

    /// C++: `int NIVissimTL::getID() const` [const]
    // int
    // NIVissimTL::getID() const {
    //     return myID;
    // }
    #[allow(unused_variables)]
    pub fn get_id(&self) -> i32 {
        self.myID
    }

    /// C++: `bool NIVissimTL::dictionary(int, const std::string &, const std::string &, SUMOTime, SUMOTime)` [static]
    // bool
    // NIVissimTL::dictionary(int id, const std::string& type,
    //                        const std::string& name, SUMOTime absdur,
    //                        SUMOTime offset) {
    //     NIVissimTL* o = new NIVissimTL(id, type, name, absdur, offset);
    //     if (!dictionary(id, o)) {
    //         delete o;
    //         return false;
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn dictionary(id: i32, type_: &str, name: &str, absdur: SUMOTime, offset: SUMOTime) -> bool {
        let o = Ptr::from_raw(Box::into_raw(Box::new(NIVissimTL::new(id, type_, name, absdur, offset))));
        if !NIVissimTL::dictionary_2(id, o) {
            // delete o;
            drop(unsafe { Box::from_raw(o.as_raw()) });
            return false;
        }
        true
    }

    /// C++: `bool NIVissimTL::dictionary(int, NIVissimTL *)` [static]
    // bool
    // NIVissimTL::dictionary(int id, NIVissimTL* o) {
    //     DictType::iterator i = myDict.find(id);
    //     if (i == myDict.end()) {
    //         myDict[id] = o;
    //         return true;
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn dictionary_2(id: i32, o: Ptr<NIVissimTL>) -> bool {
        MY_DICT.with(|d| {
            let mut d = d.borrow_mut();
            if !d.contains_key(&id) {
                d.insert(id, o);
                return true;
            }
            false
        })
    }

    /// C++: `NIVissimTL * NIVissimTL::dictionary(int)` [static]
    // NIVissimTL*
    // NIVissimTL::dictionary(int id) {
    //     DictType::iterator i = myDict.find(id);
    //     if (i == myDict.end()) {
    //         return nullptr;
    //     }
    //     return (*i).second;
    // }
    #[allow(unused_variables)]
    pub fn dictionary_3(id: i32) -> Ptr<NIVissimTL> {
        MY_DICT.with(|d| d.borrow().get(&id).copied().unwrap_or_else(Ptr::null))
    }

    /// C++: `void NIVissimTL::clearDict()` [static]
    // void
    // NIVissimTL::clearDict() {
    //     for (DictType::iterator i = myDict.begin(); i != myDict.end(); i++) {
    //         delete (*i).second;
    //     }
    //     myDict.clear();
    // }
    #[allow(unused_variables)]
    pub fn clear_dict() {
        MY_DICT.with(|d| {
            let mut d = d.borrow_mut();
            for (_, o) in d.iter() {
                // delete (*i).second;
                drop(unsafe { Box::from_raw(o.as_raw()) });
            }
            d.clear();
        })
    }

    /// C++: `bool NIVissimTL::dict_SetSignals(NBTrafficLightLogicCont &, NBEdgeCont &)` [static]
    // bool
    // NIVissimTL::dict_SetSignals(NBTrafficLightLogicCont& tlc,
    //                             NBEdgeCont& ec) {
    //     int ref = 0;
    //     int ref_groups = 0;
    //     int ref_signals = 0;
    //     int no_signals = 0;
    //     int no_groups = 0;
    //     for (DictType::iterator i = myDict.begin(); i != myDict.end(); i++) {
    //         NIVissimTL* tl = (*i).second;
    //         /*  if(tl->myType!="festzeit") {
    //            cout << " Warning: The traffic light '" << tl->myID
    //             << "' could not be assigned to a node." << endl;
    //            ref++;
    //            continue;
    //           }*/
    //         std::string id = toString<int>(tl->myID);
    //         TrafficLightType type = ((tl->getType() == "festzeit" || tl->getType() == "festzeit_fake") ?
    //                                  TrafficLightType::STATIC : TrafficLightType::ACTUATED);
    //         NBLoadedTLDef* def = new NBLoadedTLDef(ec, id, 0, type);
    //         if (!tlc.insert(def)) {
    //             WRITE_ERRORF(TL("Error on adding a traffic light\n Must be a multiple id ('%')"), id);
    //             continue;
    //         }
    //         def->setCycleDuration(tl->myAbsDuration);
    //         // add each group to the node's container
    //         SGroupDictType sgs = NIVissimTLSignalGroup::getGroupsFor(tl->getID());
    //         for (SGroupDictType::const_iterator j = sgs.begin(); j != sgs.end(); j++) {
    //             if (!(*j).second->addTo(def)) {
    //                 WRITE_WARNINGF(TL("The signal group '%' could not be assigned to tl '%'."), toString<int>((*j).first), toString<int>(tl->myID));
    //                 ref_groups++;
    //             }
    //             no_groups++;
    //         }
    //         // add the signal group signals to the node
    //         SSignalDictType signals = NIVissimTLSignal::getSignalsFor(tl->getID());
    //         for (SSignalDictType::const_iterator k = signals.begin(); k != signals.end(); k++) {
    //             if (!(*k).second->addTo(ec, def)) {
    //                 WRITE_WARNINGF(TL("The signal '%' could not be assigned to tl '%'."), toString<int>((*k).first), toString<int>(tl->myID));
    //                 ref_signals++;
    //             }
    //             no_signals++;
    //         }
    //     }
    //     if (ref != 0) {
    //         WRITE_WARNINGF(TL("Could not set % of % traffic lights."), toString<int>(ref), toString<int>((int)myDict.size()));
    //     }
    //     if (ref_groups != 0) {
    //         WRITE_WARNINGF(TL("Could not set % of % groups."), toString<int>(ref_groups), toString<int>(no_groups));
    //     }
    //     if (ref_signals != 0) {
    //         WRITE_WARNINGF(TL("Could not set % of % signals."), toString<int>(ref_signals), toString<int>(no_signals));
    //     }
    //     return true;
    //
    // }
    #[allow(unused_variables)]
    pub fn dict_set_signals(tlc: &mut NBTrafficLightLogicCont, ec: &mut NBEdgeCont) -> bool {
        todo!("port NIVissimTL::dict_SetSignals")
    }

}

/// C++ class `NIVissimTL::NIVissimTLSignal` (netimport/vissim/tempstructs/NIVissimTL.h)
pub struct NIVissimTL_NIVissimTLSignal {
    pub myID: i32,
    pub myName: String,
    pub myGroupIDs: Vec<i32>,
    pub myEdgeID: i32,
    pub myLane: i32,
    pub myPosition: f64,
    pub myVehicleTypes: Vec<i32>,
}

// C++ static members of NIVissimTLSignal (port as module-level statics, see AGENTS.md):
//   static SignalDictType NIVissimTLSignal::myDict;

impl NIVissimTL_NIVissimTLSignal {
    /// C++: `void NIVissimTLSignal::NIVissimTLSignal(int, const std::string &, const std::vector<int> &, int, int, double, const std::vector<int> &)`
    // NIVissimTL::NIVissimTLSignal::NIVissimTLSignal(int id,
    //         const std::string& name,
    //         const std::vector<int>& groupids,
    //         int edgeid,
    //         int laneno,
    //         double position,
    //         const std::vector<int>& vehicleTypes)
    //     : myID(id), myName(name), myGroupIDs(groupids),
    //       myEdgeID(edgeid), myLane(laneno), myPosition(position),
    //       myVehicleTypes(vehicleTypes) {}
    #[allow(unused_variables)]
    pub fn new(id: i32, name: &str, groupids: &Vec<i32>, edgeid: i32, laneno: i32, position: f64, assignedVehicleTypes: &Vec<i32>) -> Self {
        todo!("port NIVissimTLSignal::NIVissimTLSignal")
    }

    /// C++: `void NIVissimTLSignal::~NIVissimTLSignal()`
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port NIVissimTLSignal::~NIVissimTLSignal")
    }

    /// C++: `bool NIVissimTLSignal::isWithin(const PositionVector &) const` [const]
    // bool
    // NIVissimTL::NIVissimTLSignal::isWithin(const PositionVector& poly) const {
    //     return poly.around(getPosition());
    // }
    #[allow(unused_variables)]
    pub fn is_within(&self, poly: &PositionVector) -> bool {
        todo!("port NIVissimTLSignal::isWithin")
    }

    /// C++: `Position NIVissimTLSignal::getPosition() const` [const]
    // Position
    // NIVissimTL::NIVissimTLSignal::getPosition() const {
    //     return NIVissimAbstractEdge::dictionary(myEdgeID)->getGeomPosition(myPosition);
    // }
    #[allow(unused_variables)]
    pub fn get_position(&self) -> Position {
        todo!("port NIVissimTLSignal::getPosition")
    }

    /// C++: `bool NIVissimTLSignal::addTo(NBEdgeCont &, NBLoadedTLDef *) const` [const]
    // bool
    // NIVissimTL::NIVissimTLSignal::addTo(NBEdgeCont& ec, NBLoadedTLDef* tl) const {
    //     NIVissimConnection* c = NIVissimConnection::dictionary(myEdgeID);
    //     NBConnectionVector assignedConnections;
    //     if (c == nullptr) {
    //         // What to do if on an edge? -> close all outgoing connections
    //         NBEdge* edge = ec.retrievePossiblySplit(toString<int>(myEdgeID), myPosition);
    //         if (edge == nullptr) {
    //             WRITE_WARNINGF(TL("Could not set tls signal at edge '%' - the edge was not built."), myEdgeID);
    //             return false;
    //         }
    //         // Check whether it is already known, which edges are approached
    //         //  by which lanes
    //         // check whether to use the original lanes only
    //         if (edge->lanesWereAssigned()) {
    //             std::vector<NBEdge::Connection> connections = edge->getConnectionsFromLane(myLane - 1);
    //             for (std::vector<NBEdge::Connection>::iterator i = connections.begin(); i != connections.end(); i++) {
    //                 const NBEdge::Connection& conn = *i;
    //                 assert(myLane - 1 < (int)edge->getNumLanes());
    //                 assignedConnections.push_back(NBConnection(edge, myLane - 1, conn.toEdge, conn.toLane));
    //             }
    //         } else {
    //             WRITE_WARNINGF(TL("Edge '%': Lanes were not assigned."), myEdgeID);
    //             for (int j = 0; j < edge->getNumLanes(); j++) {
    //                 std::vector<NBEdge::Connection> connections = edge->getConnectionsFromLane(j);
    //                 for (std::vector<NBEdge::Connection>::iterator i = connections.begin(); i != connections.end(); i++) {
    //                     const NBEdge::Connection& conn = *i;
    //                     assignedConnections.push_back(NBConnection(edge, j, conn.toEdge, conn.toLane));
    //                 }
    //             }
    //         }
    //     } else {
    //         // get the edges
    //         NBEdge* tmpFrom = ec.retrievePossiblySplit(toString<int>(c->getFromEdgeID()), toString<int>(c->getToEdgeID()), true);
    //         NBEdge* tmpTo = ec.retrievePossiblySplit(toString<int>(c->getToEdgeID()), toString<int>(c->getFromEdgeID()), false);
    //         // check whether the edges are known
    //         if (tmpFrom != nullptr && tmpTo != nullptr) {
    //             // add connections this signal is responsible for
    //             assignedConnections.push_back(NBConnection(tmpFrom, -1, tmpTo, -1));
    //         } else {
    //             return false;
    //             // !!! one of the edges could not be build
    //         }
    //     }
    //     // add to the group
    //     assert(myGroupIDs.size() != 0);
    //     // @todo just another hack?!
    //     /*
    //     if (myGroupIDs.size() == 1) {
    //         return tl->addToSignalGroup(toString<int>(*(myGroupIDs.begin())),
    //                                     assignedConnections);
    //     } else {
    //         // !!!
    //         return tl->addToSignalGroup(toString<int>(*(myGroupIDs.begin())),
    //                                     assignedConnections);
    //     }
    //     */
    //     return tl->addToSignalGroup(toString<int>(myGroupIDs.front()), assignedConnections);
    // }
    #[allow(unused_variables)]
    pub fn add_to(&self, ec: &mut NBEdgeCont, node: Ptr<NBLoadedTLDef>) -> bool {
        todo!("port NIVissimTLSignal::addTo")
    }

    /// C++: `bool NIVissimTLSignal::dictionary(int, int, NIVissimTLSignal *)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn dictionary(lsaid: i32, id: i32, o: Ptr<NIVissimTL_NIVissimTLSignal>) -> bool {
        todo!("port NIVissimTLSignal::dictionary")
    }

    /// C++: `NIVissimTLSignal * NIVissimTLSignal::dictionary(int, int)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn dictionary_2(lsaid: i32, id: i32) -> Ptr<NIVissimTL_NIVissimTLSignal> {
        todo!("port NIVissimTLSignal::dictionary")
    }

    /// C++: `void NIVissimTLSignal::clearDict()` [static]
    // void
    // NIVissimTL::NIVissimTLSignal::clearDict() {
    //     for (SignalDictType::iterator i = myDict.begin(); i != myDict.end(); i++) {
    //         for (SSignalDictType::iterator j = (*i).second.begin(); j != (*i).second.end(); j++) {
    //             delete (*j).second;
    //         }
    //     }
    //     myDict.clear();
    // }
    #[allow(unused_variables)]
    pub fn clear_dict() {
        todo!("port NIVissimTLSignal::clearDict")
    }

    /// C++: `SSignalDictType NIVissimTLSignal::getSignalsFor(int)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_signals_for(tlid: i32) -> NIVissimTL_SSignalDictType {
        todo!("port NIVissimTLSignal::getSignalsFor")
    }

}

/// C++ class `NIVissimTL::NIVissimTLSignalGroup` (netimport/vissim/tempstructs/NIVissimTL.h)
pub struct NIVissimTL_NIVissimTLSignalGroup {
    pub myID: i32,
    pub myName: String,
    pub myTimes: Vec<SUMOTime>,
    pub myFirstIsRed: bool,
    pub myTRedYellow: SUMOTime,
    pub myTYellow: SUMOTime,
}

// C++ static members of NIVissimTLSignalGroup (port as module-level statics, see AGENTS.md):
//   static GroupDictType NIVissimTLSignalGroup::myDict;

impl NIVissimTL_NIVissimTLSignalGroup {
    /// C++: `void NIVissimTLSignalGroup::NIVissimTLSignalGroup(int, const std::string &, bool, const std::vector<SUMOTime> &, SUMOTime, SUMOTime)`
    // NIVissimTL::NIVissimTLSignalGroup::NIVissimTLSignalGroup(
    //     int id,
    //     const std::string& name,
    //     bool isGreenBegin, const std::vector<SUMOTime>& times,
    //     SUMOTime tredyellow, SUMOTime tyellow)
    //     : myID(id), myName(name), myTimes(times),
    //       myFirstIsRed(!isGreenBegin), myTRedYellow(tredyellow),
    //       myTYellow(tyellow) {}
    #[allow(unused_variables)]
    pub fn new(id: i32, name: &str, isGreenBegin: bool, times: &Vec<SUMOTime>, tredyellow: SUMOTime, tyellow: SUMOTime) -> Self {
        todo!("port NIVissimTLSignalGroup::NIVissimTLSignalGroup")
    }

    /// C++: `void NIVissimTLSignalGroup::~NIVissimTLSignalGroup()`
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port NIVissimTLSignalGroup::~NIVissimTLSignalGroup")
    }

    /// C++: `bool NIVissimTLSignalGroup::addTo(NBLoadedTLDef *) const` [const]
    // bool
    // NIVissimTL::NIVissimTLSignalGroup::addTo(NBLoadedTLDef* tl) const {
    //     // get the color at the begin
    //     NBTrafficLightDefinition::TLColor color = myFirstIsRed
    //             ? NBTrafficLightDefinition::TLCOLOR_RED : NBTrafficLightDefinition::TLCOLOR_GREEN;
    //     std::string id = toString<int>(myID);
    //     tl->addSignalGroup(id);
    //     for (SUMOTime t : myTimes) {
    //         tl->addSignalGroupPhaseBegin(id, t, color);
    //         color = color == NBTrafficLightDefinition::TLCOLOR_RED
    //                 ? NBTrafficLightDefinition::TLCOLOR_GREEN : NBTrafficLightDefinition::TLCOLOR_RED;
    //     }
    //     if (myTimes.size() == 0) {
    //         if (myFirstIsRed) {
    //             tl->addSignalGroupPhaseBegin(id, 0, NBTrafficLightDefinition::TLCOLOR_RED);
    //         } else {
    //             tl->addSignalGroupPhaseBegin(id, 0, NBTrafficLightDefinition::TLCOLOR_GREEN);
    //         }
    //     }
    //     tl->setSignalYellowTimes(id, myTRedYellow, myTYellow);
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn add_to(&self, node: Ptr<NBLoadedTLDef>) -> bool {
        todo!("port NIVissimTLSignalGroup::addTo")
    }

    /// C++: `bool NIVissimTLSignalGroup::dictionary(int, int, NIVissimTLSignalGroup *)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn dictionary(lsaid: i32, id: i32, o: Ptr<NIVissimTL_NIVissimTLSignalGroup>) -> bool {
        todo!("port NIVissimTLSignalGroup::dictionary")
    }

    /// C++: `NIVissimTLSignalGroup * NIVissimTLSignalGroup::dictionary(int, int)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn dictionary_2(lsaid: i32, id: i32) -> Ptr<NIVissimTL_NIVissimTLSignalGroup> {
        todo!("port NIVissimTLSignalGroup::dictionary")
    }

    /// C++: `void NIVissimTLSignalGroup::clearDict()` [static]
    // void
    // NIVissimTL::NIVissimTLSignalGroup::clearDict() {
    //     for (GroupDictType::iterator i = myDict.begin(); i != myDict.end(); i++) {
    //         for (SGroupDictType::iterator j = (*i).second.begin(); j != (*i).second.end(); j++) {
    //             delete (*j).second;
    //         }
    //     }
    //     myDict.clear();
    // }
    #[allow(unused_variables)]
    pub fn clear_dict() {
        todo!("port NIVissimTLSignalGroup::clearDict")
    }

    /// C++: `SGroupDictType NIVissimTLSignalGroup::getGroupsFor(int)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_groups_for(tlid: i32) -> NIVissimTL_SGroupDictType {
        todo!("port NIVissimTLSignalGroup::getGroupsFor")
    }

}

