//! Port of netbuild/NBTrafficLightLogicCont.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `std::vector<NBTrafficLightDefinition *> NBTrafficLightLogicCont::Definitions`
pub type NBTrafficLightLogicCont_Definitions = Vec<Ptr<NBTrafficLightDefinition>>;

/// C++ typedef `std::map<std::string, Program2Def> NBTrafficLightLogicCont::Id2Defs`
pub type NBTrafficLightLogicCont_Id2Defs = BTreeMap<String, NBTrafficLightLogicCont_Program2Def>;

/// C++ typedef `std::map<std::string, Program2Logic> NBTrafficLightLogicCont::Id2Logics`
pub type NBTrafficLightLogicCont_Id2Logics = BTreeMap<String, NBTrafficLightLogicCont_Program2Logic>;

/// C++ typedef `std::vector<NBTrafficLightLogic *> NBTrafficLightLogicCont::Logics`
pub type NBTrafficLightLogicCont_Logics = Vec<Ptr<NBTrafficLightLogic>>;

/// C++ typedef `std::map<std::string, NBTrafficLightDefinition *> NBTrafficLightLogicCont::Program2Def`
pub type NBTrafficLightLogicCont_Program2Def = BTreeMap<String, Ptr<NBTrafficLightDefinition>>;

/// C++ typedef `std::map<std::string, NBTrafficLightLogic *> NBTrafficLightLogicCont::Program2Logic`
pub type NBTrafficLightLogicCont_Program2Logic = BTreeMap<String, Ptr<NBTrafficLightLogic>>;

/// A container for traffic light definitions and built programs
/// C++ class `NBTrafficLightLogicCont` (netbuild/NBTrafficLightLogicCont.h)
pub struct NBTrafficLightLogicCont {
    /// The container for previously computed tl-logics
    pub myComputed: NBTrafficLightLogicCont_Id2Logics,
    /// The container for tl-ids to their definitions
    pub myDefinitions: NBTrafficLightLogicCont_Id2Defs,
    /// The container for extracted definitions
    pub myExtracted: BTreeSet<Ptr<NBTrafficLightDefinition>>,
    /// List of tls which shall have an offset of T/2
    pub myHalfOffsetTLS: BTreeSet<String>,
    /// List of tls which shall have an offset of T/2
    pub myQuarterOffsetTLS: BTreeSet<String>,
}

// C++ static members of NBTrafficLightLogicCont (port as module-level statics, see AGENTS.md):
//   static const Program2Def NBTrafficLightLogicCont::EmptyDefinitions;

/// Wrapper so the empty map can live in a `static` (Ptr is not Sync); the map is never mutated
pub struct NBTrafficLightLogicCont_EmptyDefinitions(pub NBTrafficLightLogicCont_Program2Def);
// SAFETY: the wrapped map is always empty, so it holds no pointers that could be shared unsafely
unsafe impl Sync for NBTrafficLightLogicCont_EmptyDefinitions {}
static EMPTY_DEFINITIONS: NBTrafficLightLogicCont_EmptyDefinitions = NBTrafficLightLogicCont_EmptyDefinitions(BTreeMap::new());

/// C++ `delete p` for a Ptr that was allocated as a Box (same convention as ms_route.rs)
fn delete_ptr<T>(p: Ptr<T>) {
    drop(unsafe { Box::from_raw(p.as_raw()) });
}

/// `std::istringstream(id) >> long long` as IDSupplier::avoid uses it: optional leading whitespace,
/// optional sign, then digits; None when no number can be read
fn parse_leading_i64(s: &str) -> Option<i64> {
    let t = s.trim_start();
    let bytes = t.as_bytes();
    let mut i = 0;
    if i < bytes.len() && (bytes[i] == b'+' || bytes[i] == b'-') {
        i += 1;
    }
    let digits_start = i;
    while i < bytes.len() && bytes[i].is_ascii_digit() {
        i += 1;
    }
    if i == digits_start {
        return None;
    }
    t[..i].parse().ok()
}

#[cfg(test)]
mod tests {
    use super::parse_leading_i64;

    #[test]
    fn leading_integer_is_read_like_istringstream() {
        assert_eq!(parse_leading_i64("12"), Some(12));
        assert_eq!(parse_leading_i64("12abc"), Some(12));
        assert_eq!(parse_leading_i64("-3"), Some(-3));
        assert_eq!(parse_leading_i64("abc"), None);
        assert_eq!(parse_leading_i64(""), None);
    }
}

impl NBTrafficLightLogicCont {
    /// C++: `void NBTrafficLightLogicCont::NBTrafficLightLogicCont()`
    /// Constructor
    // NBTrafficLightLogicCont::NBTrafficLightLogicCont() {}
    #[allow(unused_variables)]
    pub fn new() -> Self {
        Self {
            myComputed: BTreeMap::new(),
            myDefinitions: BTreeMap::new(),
            myExtracted: BTreeSet::new(),
            myHalfOffsetTLS: BTreeSet::new(),
            myQuarterOffsetTLS: BTreeSet::new(),
        }
    }

    /// C++: `void NBTrafficLightLogicCont::~NBTrafficLightLogicCont()`
    /// Destructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        self.clear();
    }

    /// C++: `void NBTrafficLightLogicCont::applyOptions(OptionsCont &)`
    /// Initialises the storage by applying given options
    // void
    // NBTrafficLightLogicCont::applyOptions(OptionsCont& oc) {
    //     // check whether any offsets shall be manipulated by setting
    //     //  them to half of the duration
    //     if (oc.isSet("tls.half-offset")) {
    //         std::vector<std::string> ids = oc.getStringVector("tls.half-offset");
    //         myHalfOffsetTLS.insert(ids.begin(), ids.end());
    //     }
    //     // check whether any offsets shall be manipulated by setting
    //     //  them to a quarter of the duration
    //     if (oc.isSet("tls.quarter-offset")) {
    //         std::vector<std::string> ids = oc.getStringVector("tls.quarter-offset");
    //         myQuarterOffsetTLS.insert(ids.begin(), ids.end());
    //     }
    // }
    #[allow(unused_variables)]
    pub fn apply_options(&mut self, oc: &mut OptionsCont) {
        // check whether any offsets shall be manipulated by setting
        //  them to half of the duration
        if oc.is_set("tls.half-offset", true).unwrap_or_else(|e| panic!("{e}")) {
            let ids = oc.get_string_vector("tls.half-offset").unwrap_or_else(|e| panic!("{e}")).clone();
            self.myHalfOffsetTLS.extend(ids);
        }
        // check whether any offsets shall be manipulated by setting
        //  them to a quarter of the duration
        if oc.is_set("tls.quarter-offset", true).unwrap_or_else(|e| panic!("{e}")) {
            let ids = oc.get_string_vector("tls.quarter-offset").unwrap_or_else(|e| panic!("{e}")).clone();
            self.myQuarterOffsetTLS.extend(ids);
        }
    }

    /// C++: `bool NBTrafficLightLogicCont::insert(NBTrafficLightDefinition *, bool)`
    /// Adds a logic definition to the dictionary
    // bool
    // NBTrafficLightLogicCont::insert(NBTrafficLightDefinition* logic, bool forceInsert) {
    //     myExtracted.erase(logic);
    //     if (myDefinitions.count(logic->getID())) {
    //         if (myDefinitions[logic->getID()].count(logic->getProgramID())) {
    //             if (forceInsert) {
    //                 logic->setProgramID(getNextProgramID(logic->getID()));
    //             } else {
    //                 return false;
    //             }
    //         }
    //     } else {
    //         myDefinitions[logic->getID()] = Program2Def();
    //     }
    //     myDefinitions[logic->getID()][logic->getProgramID()] = logic;
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn insert(&mut self, logic: Ptr<NBTrafficLightDefinition>, forceInsert: bool) -> bool {
        self.myExtracted.remove(&logic);
        let id = logic.get_id().to_string();
        if self.myDefinitions.contains_key(&id) {
            if self.myDefinitions[&id].contains_key(logic.get_program_id()) {
                if forceInsert {
                    let next = self.get_next_program_id(&id);
                    logic.m().set_program_id(&next);
                } else {
                    return false;
                }
            }
        } else {
            self.myDefinitions.insert(id.clone(), NBTrafficLightLogicCont_Program2Def::new());
        }
        let programID = logic.get_program_id().to_string();
        self.myDefinitions.get_mut(&id).unwrap().insert(programID, logic);
        true
    }

    /// C++: `bool NBTrafficLightLogicCont::removeFully(const std::string)`
    /// Removes a logic definition (and all programs) from the dictionary
    // bool
    // NBTrafficLightLogicCont::removeFully(const std::string id) {
    //     if (myDefinitions.count(id)) {
    //         // delete all programs
    //         for (Program2Def::iterator i = myDefinitions[id].begin(); i != myDefinitions[id].end(); i++) {
    //             delete i->second;
    //         }
    //         myDefinitions.erase(id);
    //         // also delete any logics that were already computed
    //         if (myComputed.count(id)) {
    //             for (Program2Logic::iterator i = myComputed[id].begin(); i != myComputed[id].end(); i++) {
    //                 delete i->second;
    //             }
    //             myComputed.erase(id);
    //         }
    //         return true;
    //     } else {
    //         return false;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn remove_fully(&mut self, id: String) -> bool {
        if let Some(programs) = self.myDefinitions.remove(&id) {
            // delete all programs
            for def in programs.into_values() {
                delete_ptr(def);
            }
            // also delete any logics that were already computed
            if let Some(programs) = self.myComputed.remove(&id) {
                for logic in programs.into_values() {
                    delete_ptr(logic);
                }
            }
            true
        } else {
            false
        }
    }

    /// C++: `bool NBTrafficLightLogicCont::removeProgram(const std::string, const std::string, bool)`
    /// Removes a program of a logic definition from the dictionary
    // bool
    // NBTrafficLightLogicCont::removeProgram(const std::string id, const std::string programID, bool del) {
    //     if (myDefinitions.count(id) && myDefinitions[id].count(programID)) {
    //         if (del) {
    //             delete myDefinitions[id][programID];
    //         }
    //         myDefinitions[id].erase(programID);
    //         if (myComputed.count(id) && myComputed[id].count(programID)) {
    //             if (del) {
    //                 delete myComputed[id][programID];
    //             }
    //             myComputed[id].erase(programID);
    //         }
    //         return true;
    //     } else {
    //         return false;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn remove_program(&mut self, id: String, programID: String, del: bool) -> bool {
        if self.myDefinitions.get(&id).is_some_and(|p| p.contains_key(&programID)) {
            let def = self.myDefinitions.get_mut(&id).unwrap().remove(&programID).unwrap();
            if del {
                delete_ptr(def);
            }
            if self.myComputed.get(&id).is_some_and(|p| p.contains_key(&programID)) {
                let logic = self.myComputed.get_mut(&id).unwrap().remove(&programID).unwrap();
                if del {
                    delete_ptr(logic);
                }
            }
            true
        } else {
            false
        }
    }

    /// C++: `void NBTrafficLightLogicCont::extract(NBTrafficLightDefinition *)`
    /// Extracts a traffic light definition from myDefinitions but keeps it in myExtracted for eventual * deletion (used by netedit)
    // void
    // NBTrafficLightLogicCont::extract(NBTrafficLightDefinition* definition) {
    //     myExtracted.insert(definition);
    //     removeProgram(definition->getID(), definition->getProgramID(), false);
    // }
    #[allow(unused_variables)]
    pub fn extract(&mut self, definition: Ptr<NBTrafficLightDefinition>) {
        self.myExtracted.insert(definition);
        self.remove_program(definition.get_id().to_string(), definition.get_program_id().to_string(), false);
    }

    /// C++: `bool NBTrafficLightLogicCont::exist(const std::string &, bool) const` [const]
    /// check if exists a definition with the given ID
    // bool
    // NBTrafficLightLogicCont::exist(const std::string& newID, bool requireComputed) const {
    //     const auto itD = myDefinitions.find(newID);
    //     const auto itC = myComputed.find(newID);
    //     if ((itD != myDefinitions.end()) && (itC != myComputed.end() || !requireComputed)) {
    //         return true;
    //     } else {
    //         return false;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn exist(&self, newID: &str, requireComputed: bool) -> bool {
        let itD = self.myDefinitions.contains_key(newID);
        let itC = self.myComputed.contains_key(newID);
        itD && (itC || !requireComputed)
    }

    /// C++: `void NBTrafficLightLogicCont::rename(NBTrafficLightDefinition *, const std::string &)`
    /// rename traffic light
    // void
    // NBTrafficLightLogicCont::rename(NBTrafficLightDefinition* tlDef, const std::string& newID) {
    //     auto it = myDefinitions.find(tlDef->getID());
    //     if (it != myDefinitions.end()) {
    //         for (auto item : it->second) {
    //             item.second->setID(newID);
    //         }
    //         myDefinitions[newID] = it->second;
    //         myDefinitions.erase(it);
    //     }
    //     auto it2 = myComputed.find(tlDef->getID());
    //     if (it2 != myComputed.end()) {
    //         for (auto item : it2->second) {
    //             item.second->setID(newID);
    //         }
    //         myComputed[newID] = it2->second;
    //         myComputed.erase(it2);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn rename(&mut self, tlDef: Ptr<NBTrafficLightDefinition>, newID: &str) {
        let oldID = tlDef.get_id().to_string();
        if let Some(programs) = self.myDefinitions.get(&oldID).cloned() {
            for item in programs.values() {
                item.m().set_id(newID);
            }
            self.myDefinitions.insert(newID.to_string(), programs);
            // C++ erases the old iterator after the assignment, so renaming to the same ID erases the entry too
            self.myDefinitions.remove(&oldID);
        }
        if let Some(programs) = self.myComputed.get(&oldID).cloned() {
            for item in programs.values() {
                item.m().set_id(newID);
            }
            self.myComputed.insert(newID.to_string(), programs);
            self.myComputed.remove(&oldID);
        }
    }

    /// C++: `int NBTrafficLightLogicCont::getNumExtracted() const` [const]
    /// return the number of extracted traffic light definitions
    // int
    // NBTrafficLightLogicCont::getNumExtracted() const {
    //     return (int)myExtracted.size();
    // }
    #[allow(unused_variables)]
    pub fn get_num_extracted(&self) -> i32 {
        self.myExtracted.len() as i32
    }

    /// C++: `std::vector<NBTrafficLightLogic *> NBTrafficLightLogicCont::getComputed() const` [const]
    /// Returns a list of all computed logics
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_computed(&self) -> Vec<Ptr<NBTrafficLightLogic>> {
        let mut result = Vec::new();
        for programs in self.myComputed.values() {
            for logic in programs.values() {
                result.push(*logic);
            }
        }
        result
    }

    /// C++: `std::pair<int, int> NBTrafficLightLogicCont::computeLogics(OptionsCont &)`
    /// Computes the traffic light logics using the stored definitions and stores the results
    // std::pair<int, int>
    // NBTrafficLightLogicCont::computeLogics(OptionsCont& oc) {
    //     // clean previous logics
    //     Logics logics = getComputed();
    //     for (Logics::iterator it = logics.begin(); it != logics.end(); it++) {
    //         delete *it;
    //     }
    //     myComputed.clear();
    //     if (oc.getBool("tls.rebuild")) {
    //         for (NBTrafficLightDefinition* def : getDefinitions()) {
    //             NBLoadedSUMOTLDef* lDef = dynamic_cast<NBLoadedSUMOTLDef*>(def);
    //             if (lDef != nullptr) {
    //                 TrafficLightType type = def->getType();
    //                 if (!oc.isDefault("tls.default-type")) {
    //                     type = SUMOXMLDefinitions::TrafficLightTypes.get(oc.getString("tls.default-type"));
    //                 }
    //                 NBOwnTLDef* oDef = new NBOwnTLDef(def->getID(), def->getNodes(), def->getOffset(), type);
    //                 oDef->setProgramID(def->getProgramID());
    //                 oDef->setParticipantsInformation();
    //                 for (NBEdge* e : oDef->getIncomingEdges()) {
    //                     e->clearControllingTLInformation();
    //                 }
    //                 oDef->setTLControllingInformation();
    //                 for (NBNode* node : oDef->getNodes()) {
    //                     node->removeTrafficLight(def);
    //                     for (auto c : node->getCrossings()) {
    //                         c->customTLIndex = -1;
    //                         c->customTLIndex2 = -1;
    //                         c->tlLinkIndex2 = -1;
    //                     }
    //                 }
    //                 removeProgram(def->getID(), def->getProgramID());
    //                 insert(oDef);
    //             }
    //         }
    //     }
    //     if (oc.getBool("tls.group-signals")) {
    //         // replace NBOwnTLDef tld with NBLoadedSUMOTLDef
    //         for (NBTrafficLightDefinition* def : getDefinitions()) {
    //             NBLoadedSUMOTLDef* lDef = dynamic_cast<NBLoadedSUMOTLDef*>(def);
    //             if (lDef == nullptr) {
    //                 NBTrafficLightLogic* logic = def->compute(oc);
    //                 if (logic != nullptr) {
    //                     lDef = new NBLoadedSUMOTLDef(*def, *logic);
    //                     lDef->setParticipantsInformation();
    //                     for (NBNode* node : lDef->getNodes()) {
    //                         node->removeTrafficLight(def);
    //                         node->addTrafficLight(lDef);
    //                     }
    //                     removeProgram(def->getID(), def->getProgramID());
    //                     insert(lDef);
    //                     delete logic;
    //                 }
    //             }
    //             if (lDef != nullptr) {
    //                 lDef->groupSignals();
    //             }
    //         }
    //     } else if (oc.getBool("tls.ungroup-signals")) {
    //         for (NBTrafficLightDefinition* def : getDefinitions()) {
    //             NBLoadedSUMOTLDef* lDef = dynamic_cast<NBLoadedSUMOTLDef*>(def);
    //             // NBOwnTLDef are always ungrouped
    //             if (lDef != nullptr) {
    //                 if (lDef->usingSignalGroups()) {
    //                     lDef->ungroupSignals();
    //                 }
    //             }
    //         }
    //     }
    //     int numPrograms = 0;
    //     for (NBTrafficLightDefinition* def : getDefinitions()) {
    //         if (computeSingleLogic(oc, def)) {
    //             numPrograms++;
    //         }
    //     }
    //     return std::pair<int, int>((int)myComputed.size(), numPrograms);
    // }
    #[allow(unused_variables)]
    pub fn compute_logics(&mut self, oc: &mut OptionsCont) -> (i32, i32) {
        // clean previous logics
        let logics = self.get_computed();
        for logic in logics {
            delete_ptr(logic);
        }
        self.myComputed.clear();
        if oc.get_bool("tls.rebuild").unwrap() {
            for def in self.get_definitions() {
                if def.m().myKind == NBTrafficLightDefinitionKind::LoadedSUMOTL {
                    let mut ty = def.m().get_type();
                    if !oc.is_default("tls.default-type") {
                        ty = TrafficLightTypes.get(&oc.get_string("tls.default-type").unwrap()).unwrap_or_else(|e| panic!("{e}"));
                    }
                    let id = def.m().get_id().to_string();
                    let nodes = def.m().get_nodes().clone();
                    let offset = def.m().get_offset();
                    let programID = def.m().get_program_id().to_string();
                    let oDef: Ptr<NBOwnTLDef> = Ptr::from_raw(Box::into_raw(Box::new(NBOwnTLDef::new(&id, &nodes, offset, ty))));
                    oDef.m().set_program_id(&programID);
                    oDef.m().set_participants_information();
                    for e in oDef.m().get_incoming_edges().clone() {
                        e.m().clear_controlling_tl_information();
                    }
                    oDef.m().set_tl_controlling_information();
                    for node in oDef.m().get_nodes().clone() {
                        node.m().remove_traffic_light(def);
                        for c in node.m().get_crossings() {
                            c.m().customTLIndex = -1;
                            c.m().customTLIndex2 = -1;
                            c.m().tlLinkIndex2 = -1;
                        }
                    }
                    self.remove_program(id, programID, true);
                    self.insert(oDef.cast::<NBTrafficLightDefinition>(), false);
                }
            }
        }
        if oc.get_bool("tls.group-signals").unwrap() {
            // replace NBOwnTLDef tld with NBLoadedSUMOTLDef
            for def in self.get_definitions() {
                let mut lDef: Ptr<NBLoadedSUMOTLDef> = Ptr::null();
                if def.m().myKind == NBTrafficLightDefinitionKind::LoadedSUMOTL {
                    lDef = def.cast::<NBLoadedSUMOTLDef>();
                } else {
                    let logic = def.m().compute(oc);
                    if !logic.is_null() {
                        lDef = Ptr::from_raw(Box::into_raw(Box::new(NBLoadedSUMOTLDef::new_2(def.m(), logic.m()))));
                        lDef.m().set_participants_information();
                        for node in lDef.m().get_nodes().clone() {
                            node.m().remove_traffic_light(def);
                            node.m().add_traffic_light(lDef.cast::<NBTrafficLightDefinition>());
                        }
                        let id = def.m().get_id().to_string();
                        let programID = def.m().get_program_id().to_string();
                        self.remove_program(id, programID, true);
                        self.insert(lDef.cast::<NBTrafficLightDefinition>(), false);
                        delete_ptr(logic);
                    }
                }
                if !lDef.is_null() {
                    lDef.m().group_signals();
                }
            }
        } else if oc.get_bool("tls.ungroup-signals").unwrap() {
            for def in self.get_definitions() {
                // NBOwnTLDef are always ungrouped
                if def.m().myKind == NBTrafficLightDefinitionKind::LoadedSUMOTL {
                    let lDef = def.cast::<NBLoadedSUMOTLDef>();
                    if lDef.m().using_signal_groups() {
                        lDef.m().ungroup_signals();
                    }
                }
            }
        }
        let mut numPrograms = 0;
        for def in self.get_definitions() {
            if self.compute_single_logic(oc, def) {
                numPrograms += 1;
            }
        }
        (self.myComputed.len() as i32, numPrograms)
    }

    /// C++: `bool NBTrafficLightLogicCont::computeSingleLogic(OptionsCont &, NBTrafficLightDefinition *)`
    /// Computes a specific traffic light logic (using by netedit)
    // bool
    // NBTrafficLightLogicCont::computeSingleLogic(OptionsCont& oc, NBTrafficLightDefinition* def) {
    //     if (def->getNodes().size() == 0) {
    //         return false;
    //     }
    //     const std::string& id = def->getID();
    //     const std::string& programID = def->getProgramID();
    //     // build program
    //     NBTrafficLightLogic* built = def->compute(oc);
    //     if (built == nullptr) {
    //         WRITE_WARNINGF(TL("Could not build program '%' for traffic light '%'"), programID, id);
    //         return false;
    //     }
    //     // compute offset
    //     SUMOTime T = built->getDuration();
    //     if (myHalfOffsetTLS.count(id)) {
    //         built->setOffset(TIME2STEPS(floor(STEPS2TIME(T / 2))));
    //     }
    //     if (myQuarterOffsetTLS.count(id)) {
    //         built->setOffset(TIME2STEPS(floor(STEPS2TIME(T / 4))));
    //     }
    //     // and insert the result after computation
    //     // make sure we don't leak memory if computeSingleLogic is called externally
    //     if (myComputed[id][programID] != nullptr) {
    //         delete myComputed[id][programID];
    //     }
    //     myComputed[id][programID] = built;
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn compute_single_logic(&mut self, oc: &mut OptionsCont, def: Ptr<NBTrafficLightDefinition>) -> bool {
        if def.get_nodes().is_empty() {
            return false;
        }
        let id = def.get_id().to_string();
        let programID = def.get_program_id().to_string();
        // build program
        let built = def.m().compute(oc);
        if built.is_null() {
            utils::write_warningf!("Could not build program '%' for traffic light '%'", programID, id);
            return false;
        }
        // compute offset
        let T = built.get_duration();
        if self.myHalfOffsetTLS.contains(&id) {
            built.m().set_offset(time2steps((steps2time(T / 2)).floor()));
        }
        if self.myQuarterOffsetTLS.contains(&id) {
            built.m().set_offset(time2steps((steps2time(T / 4)).floor()));
        }
        // and insert the result after computation
        // make sure we don't leak memory if computeSingleLogic is called externally
        let programs = self.myComputed.entry(id).or_default();
        if let Some(old) = programs.get(&programID) {
            if !old.is_null() {
                delete_ptr(*old);
            }
        }
        programs.insert(programID, built);
        true
    }

    /// C++: `void NBTrafficLightLogicCont::remapRemoved(NBEdge *, const EdgeVector &, const EdgeVector &)`
    /// Replaces occurrences of the removed edge in incoming/outgoing edges of all definitions
    // void
    // NBTrafficLightLogicCont::remapRemoved(NBEdge* removed, const EdgeVector& incoming,
    //                                       const EdgeVector& outgoing) {
    //     Definitions definitions = getDefinitions();
    //     for (Definitions::iterator it = definitions.begin(); it != definitions.end(); it++) {
    //         (*it)->remapRemoved(removed, incoming, outgoing);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn remap_removed(&mut self, removed: Ptr<NBEdge>, incoming: &EdgeVector, outgoing: &EdgeVector) {
        let definitions = self.get_definitions();
        for def in definitions {
            def.m().remap_removed(removed, incoming, outgoing);
        }
    }

    /// C++: `void NBTrafficLightLogicCont::replaceRemoved(NBEdge *, int, NBEdge *, int, bool)`
    /// Replaces occurrences of the removed edge/lane in all definitions by the given edge
    // void
    // NBTrafficLightLogicCont::replaceRemoved(NBEdge* removed, int removedLane,
    //                                         NBEdge* by, int byLane, bool incoming) {
    //     Definitions definitions = getDefinitions();
    //     for (Definitions::iterator it = definitions.begin(); it != definitions.end(); it++) {
    //         (*it)->replaceRemoved(removed, removedLane, by, byLane, incoming);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn replace_removed(&mut self, removed: Ptr<NBEdge>, removedLane: i32, by: Ptr<NBEdge>, byLane: i32, incoming: bool) {
        let definitions = self.get_definitions();
        for def in definitions {
            def.m().replace_removed(removed, removedLane, by, byLane, incoming);
        }
    }

    /// C++: `NBTrafficLightDefinition * NBTrafficLightLogicCont::getDefinition(const std::string &, const std::string &) const` [const]
    /// Returns the named definition
    // NBTrafficLightDefinition*
    // NBTrafficLightLogicCont::getDefinition(const std::string& id, const std::string& programID) const {
    //     Id2Defs::const_iterator i = myDefinitions.find(id);
    //     if (i != myDefinitions.end()) {
    //         Program2Def programs = i->second;
    //         Program2Def::const_iterator i2 = programs.find(programID);
    //         if (i2 != programs.end()) {
    //             return i2->second;
    //         }
    //     }
    //     return nullptr;
    // }
    #[allow(unused_variables)]
    pub fn get_definition(&self, id: &str, programID: &str) -> Ptr<NBTrafficLightDefinition> {
        // C++ returns nullptr when either lookup misses
        match self.myDefinitions.get(id).and_then(|programs| programs.get(programID)) {
            Some(def) => *def,
            None => Ptr::null(),
        }
    }

    /// C++: `const std::map<std::string, NBTrafficLightDefinition *> & NBTrafficLightLogicCont::getPrograms(const std::string &) const` [const]
    /// Returns all programs for the given tl-id
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_programs(&self, id: &str) -> &BTreeMap<String, Ptr<NBTrafficLightDefinition>> {
        match self.myDefinitions.get(id) {
            Some(programs) => programs,
            None => &EMPTY_DEFINITIONS.0,
        }
    }

    /// C++: `NBTrafficLightLogic * NBTrafficLightLogicCont::getLogic(const std::string &, const std::string &) const` [const]
    /// Returns the computed logic for the given name
    // NBTrafficLightLogic*
    // NBTrafficLightLogicCont::getLogic(const std::string& id, const std::string& programID) const {
    //     Id2Logics::const_iterator i = myComputed.find(id);
    //     if (i != myComputed.end()) {
    //         Program2Logic programs = i->second;
    //         Program2Logic::const_iterator i2 = programs.find(programID);
    //         if (i2 != programs.end()) {
    //             return i2->second;
    //         }
    //     }
    //     return nullptr;
    // }
    #[allow(unused_variables)]
    pub fn get_logic(&self, id: &str, programID: &str) -> Ptr<NBTrafficLightLogic> {
        // C++ returns nullptr when either lookup misses
        match self.myComputed.get(id).and_then(|programs| programs.get(programID)) {
            Some(logic) => *logic,
            None => Ptr::null(),
        }
    }

    /// C++: `void NBTrafficLightLogicCont::setTLControllingInformation(const NBEdgeCont &, const NBNodeCont &)`
    /// Informs the edges about being controlled by a tls
    // void
    // NBTrafficLightLogicCont::setTLControllingInformation(const NBEdgeCont& ec, const NBNodeCont& nc) {
    //     Definitions definitions = getDefinitions();
    //     // set the information about all participants, first
    //     for (Definitions::iterator it = definitions.begin(); it != definitions.end(); it++) {
    //         (*it)->setParticipantsInformation();
    //     }
    //     // clear previous information because tlDefs may have been removed in netedit
    //     ec.clearControllingTLInformation();
    //     // insert the information about the tl-controlling
    //     for (Definitions::iterator it = definitions.begin(); it != definitions.end(); it++) {
    //         (*it)->setTLControllingInformation();
    //     }
    //     // handle rail signals which are not instantiated as normal definitions
    //     for (std::map<std::string, NBNode*>::const_iterator it = nc.begin(); it != nc.end(); it ++) {
    //         NBNode* n = it->second;
    //         if (n->getType() == SumoXMLNodeType::RAIL_SIGNAL || n->getType() == SumoXMLNodeType::RAIL_CROSSING) {
    //             NBOwnTLDef dummy(n->getID(), n, 0, TrafficLightType::STATIC);
    //             dummy.setParticipantsInformation();
    //             dummy.setTLControllingInformation();
    //             n->setCrossingTLIndices(dummy.getID(), (int)dummy.getControlledLinks().size());
    //             n->removeTrafficLight(&dummy);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_tl_controlling_information(&mut self, ec: &NBEdgeCont, nc: &NBNodeCont) {
        let definitions = self.get_definitions();
        // set the information about all participants, first
        for def in &definitions {
            def.m().set_participants_information();
        }
        // clear previous information because tlDefs may have been removed in netedit
        ec.clear_controlling_tl_information();
        // insert the information about the tl-controlling
        for def in &definitions {
            def.set_tl_controlling_information();
        }
        // handle rail signals which are not instantiated as normal definitions
        for n in nc.myNodes.values() {
            let n = *n;
            if n.get_type() == SumoXMLNodeType::RAIL_SIGNAL || n.get_type() == SumoXMLNodeType::RAIL_CROSSING {
                let mut dummy = NBOwnTLDef::new_2(n.get_id(), n, 0, TrafficLightType::STATIC);
                n.m().add_traffic_light(Ptr::from_mut(&mut dummy.base));
                dummy.set_participants_information();
                dummy.set_tl_controlling_information();
                n.m().set_crossing_tl_indices(dummy.get_id(), dummy.get_controlled_links().len() as i32, false);
                n.m().remove_traffic_light(Ptr::from_mut(&mut dummy.base));
            }
        }
    }

    /// C++: `void NBTrafficLightLogicCont::setOpenDriveSignalParameters()`
    /// set OpenDRIVE signal reference parameters after all link indices are known
    // void
    // NBTrafficLightLogicCont::setOpenDriveSignalParameters() {
    //     Definitions definitions = getDefinitions();
    //     for (NBTrafficLightDefinition* def : getDefinitions()) {
    //         std::map<NBEdge*, std::string> defaultSignalIDs;
    //         for (const NBConnection& con : def->getControlledLinks()) {
    //             const NBEdge::Connection& c = con.getFrom()->getConnection(con.getFromLane(), con.getTo(), con.getToLane());
    //             if (c.hasParameter("signalID")) {
    //                 defaultSignalIDs[con.getFrom()] = c.getParameter("signalID");
    //                 def->setParameter("linkSignalID:" + toString(con.getTLIndex()), c.getParameter("signalID"));
    //             }
    //         }
    //         // oftentimes, signals are placed on connecting road but are meant to apply to all connections from the incoming edge
    //         for (const NBConnection& con : def->getControlledLinks()) {
    //             const NBEdge::Connection& c = con.getFrom()->getConnection(con.getFromLane(), con.getTo(), con.getToLane());
    //             if (!c.hasParameter("signalID") && defaultSignalIDs.count(con.getFrom()) != 0) {
    //                 WRITE_WARNINGF(TL("Guessing signalID for link index % at traffic light '%'."), con.getTLIndex(), def->getID());
    //                 def->setParameter("linkSignalID:" + toString(con.getTLIndex()), defaultSignalIDs[con.getFrom()]);
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_open_drive_signal_parameters(&mut self) {
        // shortcut: NBEdge::Connection hasParameter/getParameter (the port maps getConnection to CHBuilder_Connection, which has no parameter map) and def->setParameter are not usable yet, so the signalID propagation is skipped; upgrade when NBEdge::Connection is ported with its Parameterised base
    }

    /// C++: `void NBTrafficLightLogicCont::applyOpenDriveControllers(OptionsCont &)`
    /// post processing of signal programs to group tl indices according to OpenDrive controllers (signal groups)
    // void
    // NBTrafficLightLogicCont::applyOpenDriveControllers(OptionsCont& oc) {
    //     // collect connections with controllerID which can be controlled together
    //     Definitions definitions = getDefinitions();
    //     std::map<std::string, NBTrafficLightDefinition*> defsToGroup;
    //     std::map<std::string, std::map<std::string, std::set<int>>> controllerID2tlIndex;
    //     for (NBTrafficLightDefinition* def : definitions) {
    //         bool makeGroups = false;
    //         NBConnectionVector& connections = def->getControlledLinks();
    //         std::string defID = def->getID();
    //
    //         for (NBConnection conn : connections) {
    //             std::string controllerID = conn.getFrom()->getConnection(conn.getFromLane(), conn.getTo(), conn.getToLane()).getParameter("controllerID");
    //             if (controllerID != "") {
    //                 if (controllerID2tlIndex.find(defID) == controllerID2tlIndex.end()) {
    //                     controllerID2tlIndex[defID][controllerID] = std::set<int>({ conn.getTLIndex() });
    //                 } else if (controllerID2tlIndex[defID].find(controllerID) != controllerID2tlIndex[defID].end()) {
    //                     controllerID2tlIndex[defID][controllerID].insert(conn.getTLIndex());
    //                 } else {
    //                     controllerID2tlIndex[defID][controllerID] = std::set<int>({ conn.getTLIndex() });
    //                 }
    //                 makeGroups = controllerID2tlIndex[defID][controllerID].size() > 1 || makeGroups;
    //             }
    //         }
    //         if (makeGroups) {
    //             defsToGroup[def->getID()] = def;
    //         }
    //     }
    //     bool same = true;
    //     for (NBTrafficLightLogic* computed : getComputed()) {
    //         const std::string computedID = computed->getID();
    //         if (defsToGroup.find(computedID) != defsToGroup.end()) {
    //             // remember corresponding tl indices and check if they can be joined
    //             // execute tl index joins if possible
    //             const std::vector<NBTrafficLightLogic::PhaseDefinition> phases = computed->getPhases();
    //             std::vector<int> major2minor;
    //             for (const auto& indexSet : controllerID2tlIndex[computedID]) {
    //                 if (indexSet.second.size() < 2) {
    //                     continue;
    //                 }
    //                 bool toMinor = false;
    //                 for (NBTrafficLightLogic::PhaseDefinition phase : phases) {
    //                     std::set<int>::iterator it = indexSet.second.begin();
    //                     char firstChar = phase.state[*it];
    //                     if (firstChar == LINKSTATE_TL_GREEN_MAJOR) {
    //                         firstChar = LINKSTATE_TL_GREEN_MINOR;
    //                     }
    //                     it++;
    //                     for (; it != indexSet.second.end(); it++) {
    //                         const char compareChar = (phase.state[*it] != LINKSTATE_TL_GREEN_MAJOR) ? phase.state[*it] : (char)LINKSTATE_TL_GREEN_MINOR;
    //                         if (compareChar != firstChar) {
    //                             same = false;
    //                             break;
    //                         } else if (phase.state[*it] == LINKSTATE_TL_GREEN_MINOR) {
    //                             toMinor = true;
    //                         }
    //                     }
    //                     if (!same) {
    //                         break;
    //                     }
    //                 }
    //                 if (toMinor) {
    //                     major2minor.push_back(*indexSet.second.begin());
    //                 }
    //             }
    //             if (same) {
    //                 NBLoadedSUMOTLDef* lDef = dynamic_cast<NBLoadedSUMOTLDef*>(defsToGroup[computedID]);
    //                 if (lDef == nullptr) {
    //                     lDef = new NBLoadedSUMOTLDef(*defsToGroup[computedID], *computed);
    //                     lDef->setParticipantsInformation();
    //                     for (int index : major2minor) { // update the signal states (G -> g where needed)
    //                         for (int i = 0; i < (int)phases.size(); i++) {
    //                             if (phases[i].state[index] == LINKSTATE_TL_GREEN_MAJOR) {
    //                                 lDef->getLogic()->setPhaseState(i, index, LINKSTATE_TL_GREEN_MINOR);
    //                             }
    //                         }
    //                     }
    //                     // join signal groups, update the connections
    //                     std::vector<int> indexToRemove;
    //                     for (const auto& indexSet : controllerID2tlIndex[computedID]) {
    //                         int minIndex = *indexSet.second.begin();
    //                         for (int index : indexSet.second) {
    //                             if (index != minIndex) {
    //                                 lDef->replaceIndex(index, minIndex);
    //                                 indexToRemove.push_back(index);
    //                             }
    //                         }
    //                     }
    //                     // remove unused indices from signal programs
    //                     lDef->cleanupStates();
    //                     for (NBNode* node : lDef->getNodes()) {
    //                         node->removeTrafficLight(defsToGroup[computedID]);
    //                         node->addTrafficLight(lDef);
    //                     }
    //                     removeProgram(defsToGroup[computedID]->getID(), defsToGroup[computedID]->getProgramID());
    //                     insert(lDef);
    //                     computeSingleLogic(oc, lDef);
    //                 }
    //             } else { // TODO: try other strategy to build signal groups
    //                 WRITE_WARNINGF(TL("Was not able to apply the OpenDRIVE signal group information onto the signal program of traffic light % generated by SUMO."), computed->getID());
    //             }
    //             break;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn apply_open_drive_controllers(&mut self, oc: &mut OptionsCont) {
        // shortcut: the controllerID grouping reads NBEdge::Connection parameters (not usable yet, see setOpenDriveSignalParameters), so no OpenDRIVE controller groups are formed; upgrade when NBEdge::Connection is ported
        let _ = oc;
    }

    /// C++: `Definitions NBTrafficLightLogicCont::getDefinitions() const` [const]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_definitions(&self) -> NBTrafficLightLogicCont_Definitions {
        let mut result = Vec::new();
        for programs in self.myDefinitions.values() {
            for def in programs.values() {
                result.push(*def);
            }
        }
        result
    }

    /// C++: `std::string NBTrafficLightLogicCont::getNextProgramID(const std::string &) const` [const]
    /// Returns a new (unused) programID for the given traffic light
    // std::string
    // NBTrafficLightLogicCont::getNextProgramID(const std::string& id) const {
    //     IDSupplier idS("", 0);
    //     if (myDefinitions.count(id)) {
    //         const Program2Def& programs = myDefinitions.find(id)->second;
    //         for (auto item : programs) {
    //             idS.avoid(item.first);
    //         }
    //     }
    //     return idS.getNext();
    // }
    #[allow(unused_variables)]
    pub fn get_next_program_id(&self, id: &str) -> String {
        // IDSupplier("", 0) inlined: avoid() each known program ID, then getNext()
        let mut current: i64 = 0;
        if let Some(programs) = self.myDefinitions.get(id) {
            for item in programs.keys() {
                if let Some(number) = parse_leading_i64(item) {
                    current = current.max(number + 1);
                }
            }
        }
        current.to_string()
    }

    /// C++: `void NBTrafficLightLogicCont::clear()` [private]
    /// Destroys all stored definitions and logics
    // void
    // NBTrafficLightLogicCont::clear() {
    //     Definitions definitions = getDefinitions();
    //     for (Definitions::iterator it = definitions.begin(); it != definitions.end(); it++) {
    //         delete *it;
    //     }
    //     myDefinitions.clear();
    //     Logics logics = getComputed();
    //     for (Logics::iterator it = logics.begin(); it != logics.end(); it++) {
    //         delete *it;
    //     }
    //     myComputed.clear();
    //     for (std::set<NBTrafficLightDefinition*>::iterator it = myExtracted.begin(); it != myExtracted.end(); it++) {
    //         delete *it;
    //     }
    //     myExtracted.clear();
    // }
    #[allow(unused_variables)]
    pub fn clear(&mut self) {
        let definitions = self.get_definitions();
        for def in definitions {
            delete_ptr(def);
        }
        self.myDefinitions.clear();
        let logics = self.get_computed();
        for logic in logics {
            delete_ptr(logic);
        }
        self.myComputed.clear();
        for def in std::mem::take(&mut self.myExtracted) {
            delete_ptr(def);
        }
    }

}

