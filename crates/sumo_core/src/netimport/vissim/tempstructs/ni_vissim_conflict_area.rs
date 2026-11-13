//! Port of netimport/vissim/tempstructs/NIVissimConflictArea.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `std::map<int, NIVissimConflictArea *> NIVissimConflictArea::DictType`
pub type NIVissimConflictArea_DictType = BTreeMap<i32, Ptr<NIVissimConflictArea>>;

/// A temporary storage for conflict areas imported from Vissim
/// C++ class `NIVissimConflictArea` (netimport/vissim/tempstructs/NIVissimConflictArea.h)
pub struct NIVissimConflictArea {
    /// The id of the conflict area
    pub myConflictID: i32,
    /// The first link of the conflict area
    pub myFirstLink: String,
    /// The second link of the conflict area
    pub mySecondLink: String,
    /// The priority regulation of the conflict area
    pub myStatus: String,
}

// C++ static members of NIVissimConflictArea (port as module-level statics, see AGENTS.md):
//   static DictType NIVissimConflictArea::myDict;
thread_local! {
    static MY_DICT: std::cell::RefCell<NIVissimConflictArea_DictType> = std::cell::RefCell::new(BTreeMap::new());
}

impl NIVissimConflictArea {
    /// C++: `void NIVissimConflictArea::NIVissimConflictArea(int, const std::string &, const std::string &, const std::string &)`
    /// Constructor
    // NIVissimConflictArea::NIVissimConflictArea(int id,
    //         const std::string& link1,
    //         const std::string& link2,
    //         const std::string& status)
    //     : myConflictID(id), myFirstLink(link1), mySecondLink(link2), myStatus(status) {
    // }
    #[allow(unused_variables)]
    pub fn new(id: i32, link1: &str, link2: &str, status: &str) -> Self {
        Self { myConflictID: id, myFirstLink: link1.to_string(), mySecondLink: link2.to_string(), myStatus: status.to_string() }
    }

    /// C++: `void NIVissimConflictArea::~NIVissimConflictArea()`
    /// Destructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty; the owned strings are freed by Rust drop glue
    }

    /// C++: `bool NIVissimConflictArea::dictionary(int, const std::string &, const std::string &, const std::string &)` [static]
    /// Adds the described item to the dictionary Builds the conflict area first
    // bool
    // NIVissimConflictArea::dictionary(int id, const std::string& link1,
    //                                  const std::string& link2,
    //                                  const std::string& status) {
    //     NIVissimConflictArea* ca = new NIVissimConflictArea(id, link1, link2, status);
    //     if (!dictionary(id, ca)) {
    //         delete ca;
    //         return false;
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn dictionary(id: i32, link1: &str, link2: &str, status: &str) -> bool {
        let ca = Box::into_raw(Box::new(Self::new(id, link1, link2, status)));
        if !Self::dictionary_2(id, Ptr::from_raw(ca)) {
            // SAFETY: ca was just allocated above and was not stored in the dictionary, so this is its only owner
            drop(unsafe { Box::from_raw(ca) });
            return false;
        }
        true
    }

    /// C++: `bool NIVissimConflictArea::dictionary(int, NIVissimConflictArea *)` [static]
    /// Adds the conflict area to the dictionary
    // bool
    // NIVissimConflictArea::dictionary(int id, NIVissimConflictArea* ca) {
    //     DictType::iterator i = myDict.find(id);
    //     if (i == myDict.end()) {
    //         myDict[id] = ca;
    //         return true;
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn dictionary_2(id: i32, ca: Ptr<NIVissimConflictArea>) -> bool {
        MY_DICT.with(|d| {
            let mut d = d.borrow_mut();
            if !d.contains_key(&id) {
                d.insert(id, ca);
                return true;
            }
            false
        })
    }

    /// C++: `NIVissimConflictArea * NIVissimConflictArea::dictionary(int)` [static]
    /// Returns the named dictionary
    // NIVissimConflictArea*
    // NIVissimConflictArea::dictionary(int id) {
    //     DictType::iterator i = myDict.find(id);
    //     if (i == myDict.end()) {
    //         return nullptr;
    //     }
    //     return (*i).second;
    // }
    #[allow(unused_variables)]
    pub fn dictionary_3(id: i32) -> Ptr<NIVissimConflictArea> {
        MY_DICT.with(|d| d.borrow().get(&id).copied().unwrap_or(Ptr::null()))
    }

    /// C++: `NIVissimConflictArea * NIVissimConflictArea::dict_findByLinks(const std::string &, const std::string &)` [static]
    /// Returns the conflict area from the dictionary, which defines the priority rule of the two given links
    // NIVissimConflictArea*
    // NIVissimConflictArea::dict_findByLinks(const std::string& link1,
    //                                        const std::string& link2) {
    //     for (DictType::iterator i = myDict.begin(); i != myDict.end(); i++) {
    //         if (((*i).second->myFirstLink == link1) &&
    //                 ((*i).second->mySecondLink == link2)) {
    //             return (*i).second;
    //         }
    //     }
    //     return nullptr;
    // }
    #[allow(unused_variables)]
    pub fn dict_find_by_links(link1: &str, link2: &str) -> Ptr<NIVissimConflictArea> {
        MY_DICT.with(|d| {
            for (_, ca) in d.borrow().iter() {
                if ca.myFirstLink == link1 && ca.mySecondLink == link2 {
                    return *ca;
                }
            }
            Ptr::null()
        })
    }

    /// C++: `void NIVissimConflictArea::clearDict()` [static]
    /// Clears the dictionary
    // void
    // NIVissimConflictArea::clearDict() {
    //     for (DictType::iterator i = myDict.begin(); i != myDict.end(); i++) {
    //         delete (*i).second;
    //     }
    //     myDict.clear();
    // }
    #[allow(unused_variables)]
    pub fn clear_dict() {
        let dict = MY_DICT.with(|d| std::mem::take(&mut *d.borrow_mut()));
        for (_, ca) in dict {
            // SAFETY: every entry was allocated by dictionary() via Box::into_raw and is owned by the dictionary only
            drop(unsafe { Box::from_raw(ca.as_raw()) });
        }
    }

    /// C++: `std::map<int, NIVissimConflictArea *> NIVissimConflictArea::getConflictAreas()` [static]
    /// Returns the dictionary including all conflict areas
    // static std::map<int, NIVissimConflictArea*> getConflictAreas() {
    //         return myDict;
    //     }
    #[allow(unused_variables)]
    pub fn get_conflict_areas() -> BTreeMap<i32, Ptr<NIVissimConflictArea>> {
        MY_DICT.with(|d| d.borrow().clone())
    }

    /// C++: `int NIVissimConflictArea::getID()`
    /// Returns the ID of the conflic area
    // int getID() {
    //         return myConflictID;
    //     }
    #[allow(unused_variables)]
    pub fn get_id(&mut self) -> i32 {
        self.myConflictID
    }

    /// C++: `std::string NIVissimConflictArea::getFirstLink()`
    /// Returns the first link of the conflic area
    // std::string getFirstLink() {
    //         return myFirstLink;
    //     }
    #[allow(unused_variables)]
    pub fn get_first_link(&mut self) -> String {
        self.myFirstLink.clone()
    }

    /// C++: `std::string NIVissimConflictArea::getSecondLink()`
    /// Returns the second link of the conflic area
    // std::string getSecondLink() {
    //         return mySecondLink;
    //     }
    #[allow(unused_variables)]
    pub fn get_second_link(&mut self) -> String {
        self.mySecondLink.clone()
    }

    /// C++: `std::string NIVissimConflictArea::getStatus()`
    /// Returns the priority regulation of the conflic area
    // std::string getStatus() {
    //         return myStatus;
    //     }
    #[allow(unused_variables)]
    pub fn get_status(&mut self) -> String {
        self.myStatus.clone()
    }

    /// C++: `void NIVissimConflictArea::setPriorityRegulation(NBEdgeCont &)` [static]
    /// Sets the priority regulation according to the VISSIM conflict area data
    // void
    // NIVissimConflictArea::setPriorityRegulation(NBEdgeCont& ec) {
    //     std::map<int, NIVissimConflictArea*>::iterator it;
    //     for (it = myDict.begin(); it != myDict.end(); it++) {
    //         NIVissimConflictArea* const conflictArea = it->second;
    //         NIVissimConnection* const firstLink = NIVissimConnection::dictionary(StringUtils::toInt(conflictArea->getFirstLink()));
    //         NIVissimConnection* const secondLink = NIVissimConnection::dictionary(StringUtils::toInt(conflictArea->getSecondLink()));
    //         if (firstLink == nullptr || secondLink == nullptr) {
    //             continue;
    //         }
    //         // status == "TWOYIELDSONE"
    //         NIVissimConnection* priority_conn = firstLink;
    //         NIVissimConnection* subordinate_conn = secondLink;
    //         if (conflictArea->getStatus() == "ONEYIELDSTWO") {
    //             priority_conn = secondLink;
    //             subordinate_conn = firstLink;
    //         }
    //         const std::string mayDriveFrom_id = toString<int>(priority_conn->getFromEdgeID());
    //         const std::string mayDriveTo_id = toString<int>(priority_conn->getToEdgeID());
    //         const std::string mustStopFrom_id = toString<int>(subordinate_conn->getFromEdgeID());
    //         const std::string mustStopTo_id = toString<int>(subordinate_conn->getToEdgeID());
    //
    //         NBEdge* const mayDriveFrom =  ec.retrievePossiblySplit(mayDriveFrom_id, true);
    //         NBEdge* const mayDriveTo =  ec.retrievePossiblySplit(mayDriveTo_id, false);
    //         NBEdge* const mustStopFrom =  ec.retrievePossiblySplit(mustStopFrom_id, true);
    //         NBEdge* const mustStopTo =  ec.retrievePossiblySplit(mustStopTo_id, false);
    //
    //         if (mayDriveFrom != nullptr && mayDriveTo != nullptr && mustStopFrom != nullptr && mustStopTo != nullptr) {
    //             NBNode* node = mayDriveFrom->getToNode();
    //             node->addSortedLinkFoes(
    //                 NBConnection(mayDriveFrom, mayDriveTo),
    //                 NBConnection(mustStopFrom, mustStopTo));
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_priority_regulation(ec: &mut NBEdgeCont) {
        let dict = MY_DICT.with(|d| d.borrow().clone());
        for (_, conflict_area) in dict.iter() {
            let conflict_area = *conflict_area;
            let first_link = NIVissimConnection::dictionary_2(utils::common::string_utils::to_int(&conflict_area.m().get_first_link()).unwrap_or_else(|e| panic!("{e}")));
            let second_link = NIVissimConnection::dictionary_2(utils::common::string_utils::to_int(&conflict_area.m().get_second_link()).unwrap_or_else(|e| panic!("{e}")));
            if first_link.is_null() || second_link.is_null() {
                continue;
            }
            // status == "TWOYIELDSONE"
            let mut priority_conn = first_link;
            let mut subordinate_conn = second_link;
            if conflict_area.m().get_status() == "ONEYIELDSTWO" {
                priority_conn = second_link;
                subordinate_conn = first_link;
            }
            let mayDriveFrom_id = priority_conn.get_from_edge_id().to_string();
            let mayDriveTo_id = priority_conn.get_to_edge_id().to_string();
            let mustStopFrom_id = subordinate_conn.get_from_edge_id().to_string();
            let mustStopTo_id = subordinate_conn.get_to_edge_id().to_string();

            let mayDriveFrom = ec.retrieve_possibly_split(&mayDriveFrom_id, true);
            let mayDriveTo = ec.retrieve_possibly_split(&mayDriveTo_id, false);
            let mustStopFrom = ec.retrieve_possibly_split(&mustStopFrom_id, true);
            let mustStopTo = ec.retrieve_possibly_split(&mustStopTo_id, false);

            if !mayDriveFrom.is_null() && !mayDriveTo.is_null() && !mustStopFrom.is_null() && !mustStopTo.is_null() {
                let node = mayDriveFrom.get_to_node();
                node.m().add_sorted_link_foes(
                    &NBConnection::new(mayDriveFrom, mayDriveTo),
                    &NBConnection::new(mustStopFrom, mustStopTo));
            }
        }
    }

}

