//! Port of netimport/vissim/tempstructs/NIVissimDistrictConnection.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `std::map<int, NIVissimDistrictConnection *> NIVissimDistrictConnection::DictType`
pub type NIVissimDistrictConnection_DictType = BTreeMap<i32, Ptr<NIVissimDistrictConnection>>;

/// C++ typedef `std::map<int, double> NIVissimDistrictConnection::DistrictPercentages`
pub type NIVissimDistrictConnection_DistrictPercentages = BTreeMap<i32, f64>;

/// C++ class `NIVissimDistrictConnection` (netimport/vissim/tempstructs/NIVissimDistrictConnection.h)
pub struct NIVissimDistrictConnection {
    /// The id of the connections
    pub myID: i32,
    /// The name of the connections
    pub myName: String,
    /// The connected districts
    pub myDistricts: Vec<i32>,
    /// A map how many vehicles (key, amount) should leave to a district (key)
    pub myPercentages: NIVissimDistrictConnection_DistrictPercentages,
    /// The id of the connected edge
    pub myEdgeID: i32,
    /// The position on the edge
    pub myPosition: f64,
    /// The vehicles using this connection
    pub myAssignedVehicles: Vec<(i32, i32)>,
}

// C++ static members of NIVissimDistrictConnection (port as module-level statics, see AGENTS.md):
//   static DictType NIVissimDistrictConnection::myDict;
//   static std::map<int, std::vector<int>> NIVissimDistrictConnection::myDistrictsConnections;

thread_local! {
    static MY_DICT: std::cell::RefCell<NIVissimDistrictConnection_DictType> = std::cell::RefCell::new(BTreeMap::new());
    static MY_DISTRICTS_CONNECTIONS: std::cell::RefCell<BTreeMap<i32, Vec<i32>>> = std::cell::RefCell::new(BTreeMap::new());
}

impl NIVissimDistrictConnection {
    /// C++: `void NIVissimDistrictConnection::NIVissimDistrictConnection(int, const std::string &, const std::vector<int> &, const std::vector<double> &, int, double, const std::vector<std::pair<int, int>> &)`
    /// Contructor
    // NIVissimDistrictConnection::NIVissimDistrictConnection(int id,
    //         const std::string& name,
    //         const std::vector<int>& districts, const std::vector<double>& percentages,
    //         int edgeid, double position,
    //         const std::vector<std::pair<int, int> >& assignedVehicles)
    //     : myID(id), myName(name), myDistricts(districts),
    //       myEdgeID(edgeid), myPosition(position),
    //       myAssignedVehicles(assignedVehicles) {
    //     std::vector<int>::iterator i = myDistricts.begin();
    //     std::vector<double>::const_iterator j = percentages.begin();
    //     while (i != myDistricts.end()) {
    //         myPercentages[*i] = *j;
    //         i++;
    //         j++;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn new(id: i32, name: &str, districts: &Vec<i32>, percentages: &Vec<f64>, edgeid: i32, position: f64, assignedVehicles: &Vec<(i32, i32)>) -> Self {
        let mut myPercentages = NIVissimDistrictConnection_DistrictPercentages::new();
        for (i, j) in districts.iter().zip(percentages.iter()) {
            myPercentages.insert(*i, *j);
        }
        Self {
            myID: id,
            myName: name.to_string(),
            myDistricts: districts.clone(),
            myPercentages,
            myEdgeID: edgeid,
            myPosition: position,
            myAssignedVehicles: assignedVehicles.clone(),
        }
    }

    /// C++: `void NIVissimDistrictConnection::~NIVissimDistrictConnection()`
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ implicit destructor: the vector and map members are released automatically
    }

    /// C++: `Position NIVissimDistrictConnection::geomPosition() const` [const]
    /// Returns the position The position yields from the edge geometry and the place the connection is plaed at
    // Position
    // NIVissimDistrictConnection::geomPosition() const {
    //     NIVissimAbstractEdge* e = NIVissimEdge::dictionary(myEdgeID);
    //     return e->getGeomPosition(myPosition);
    // }
    #[allow(unused_variables)]
    pub fn geom_position(&self) -> Position {
        let e = NIVissimEdge::dictionary_3(self.myEdgeID);
        e.get_geom_position(self.myPosition)
    }

    /// C++: `int NIVissimDistrictConnection::getID() const` [const]
    /// Returns the id of the connection
    // int getID() const {
    //         return myID;
    //     }
    #[allow(unused_variables)]
    pub fn get_id(&self) -> i32 {
        self.myID
    }

    /// C++: `double NIVissimDistrictConnection::getPosition() const` [const]
    /// Returns the position of the connection at the edge
    // double getPosition() const {
    //         return myPosition;
    //     }
    #[allow(unused_variables)]
    pub fn get_position(&self) -> f64 {
        self.myPosition
    }

    /// C++: `double NIVissimDistrictConnection::getMeanSpeed() const` [const]
    // double
    // NIVissimDistrictConnection::getMeanSpeed() const {
    //     //assert(myAssignedVehicles.size()!=0);
    //     if (myAssignedVehicles.size() == 0) {
    //         WRITE_WARNINGF(TL("No streams assigned at district'%'.\n Using default speed 200km/h"), toString(myID));
    //         return (double) 200 / (double) 3.6;
    //     }
    //     double speed = 0;
    //     std::vector<std::pair<int, int> >::const_iterator i;
    //     for (i = myAssignedVehicles.begin(); i != myAssignedVehicles.end(); i++) {
    //         speed += getRealSpeed((*i).second);
    //     }
    //     return speed / (double) myAssignedVehicles.size();
    // }
    #[allow(unused_variables)]
    pub fn get_mean_speed(&self) -> f64 {
        //assert(myAssignedVehicles.size()!=0);
        if self.myAssignedVehicles.len() == 0 {
            write_warningf!("No streams assigned at district'%'.\n Using default speed 200km/h", self.myID);
            return 200.0 / 3.6;
        }
        let mut speed: f64 = 0.0;
        for i in self.myAssignedVehicles.iter() {
            speed += self.get_real_speed(i.1);
        }
        speed / self.myAssignedVehicles.len() as f64
    }

    /// C++: `bool NIVissimDistrictConnection::dictionary(int, const std::string &, const std::vector<int> &, const std::vector<double> &, int, double, const std::vector<std::pair<int, int>> &)` [static]
    /// Inserts the connection into the dictionary after building it
    // bool
    // NIVissimDistrictConnection::dictionary(int id, const std::string& name,
    //                                        const std::vector<int>& districts, const std::vector<double>& percentages,
    //                                        int edgeid, double position,
    //                                        const std::vector<std::pair<int, int> >& assignedVehicles) {
    //     NIVissimDistrictConnection* o =
    //         new NIVissimDistrictConnection(id, name, districts, percentages,
    //                                        edgeid, position, assignedVehicles);
    //     if (!dictionary(id, o)) {
    //         delete o;
    //         return false;
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn dictionary(id: i32, name: &str, districts: &Vec<i32>, percentages: &Vec<f64>, edgeid: i32, position: f64, assignedVehicles: &Vec<(i32, i32)>) -> bool {
        let mut o = Box::new(NIVissimDistrictConnection::new(id, name, districts, percentages, edgeid, position, assignedVehicles));
        if !Self::dictionary_2(id, Ptr::from_mut(&mut *o)) {
            return false; // o is dropped here, like C++ `delete o`
        }
        // shortcut: the dictionary owns the connection for the rest of the run (C++ deletes it in clearDict), so leak the Box
        Box::leak(o);
        true
    }

    /// C++: `bool NIVissimDistrictConnection::dictionary(int, NIVissimDistrictConnection *)` [static]
    /// Inserts the build connection to the dictionary
    // bool
    // NIVissimDistrictConnection::dictionary(int id, NIVissimDistrictConnection* o) {
    //     DictType::iterator i = myDict.find(id);
    //     if (i == myDict.end()) {
    //         myDict[id] = o;
    //         return true;
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn dictionary_2(id: i32, o: Ptr<NIVissimDistrictConnection>) -> bool {
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

    /// C++: `NIVissimDistrictConnection * NIVissimDistrictConnection::dictionary(int)` [static]
    /// Returns the named dictionary
    // NIVissimDistrictConnection*
    // NIVissimDistrictConnection::dictionary(int id) {
    //     DictType::iterator i = myDict.find(id);
    //     if (i == myDict.end()) {
    //         return nullptr;
    //     }
    //     return (*i).second;
    // }
    #[allow(unused_variables)]
    pub fn dictionary_3(id: i32) -> Ptr<NIVissimDistrictConnection> {
        MY_DICT.with(|d| d.borrow().get(&id).copied().unwrap_or_else(Ptr::null))
    }

    /// C++: `void NIVissimDistrictConnection::dict_BuildDistrictNodes(NBDistrictCont &, NBNodeCont &)` [static]
    /// Builds the nodes that belong to a district
    // void
    // NIVissimDistrictConnection::dict_BuildDistrictNodes(NBDistrictCont& dc,
    //         NBNodeCont& nc) {
    //     for (std::map<int, std::vector<int> >::iterator k = myDistrictsConnections.begin(); k != myDistrictsConnections.end(); k++) {
    //         // get the connections
    //         const std::vector<int>& connections = (*k).second;
    //         // retrieve the current district
    //         std::string dsid = toString<int>((*k).first);
    //         NBDistrict* district = new NBDistrict(dsid);
    //         dc.insert(district);
    //         // compute the middle of the district
    //         PositionVector pos;
    //         for (std::vector<int>::const_iterator j = connections.begin(); j != connections.end(); j++) {
    //             NIVissimDistrictConnection* c = dictionary(*j);
    //             pos.push_back(c->geomPosition());
    //         }
    //         Position distCenter = pos.getPolygonCenter();
    //         if (connections.size() == 1) { // !!! ok, ok, maybe not the best way just to add an offset
    //             distCenter.add(10, 10);
    //         }
    //         district->setCenter(distCenter);
    //         // build the node
    //         std::string id = "District" + district->getID();
    //         NBNode* districtNode =
    //             new NBNode(id, district->getPosition(), district);
    //         if (!nc.insert(districtNode)) {
    //             throw 1;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn dict_build_district_nodes(dc: &mut NBDistrictCont, nc: &mut NBNodeCont) {
        let districtsConnections = MY_DISTRICTS_CONNECTIONS.with(|m| m.borrow().clone());
        for (k, connections) in districtsConnections.iter() {
            // retrieve the current district
            let dsid = k.to_string();
            let district = Ptr::from_mut(Box::leak(Box::new(NBDistrict::new_2(&dsid))));
            dc.insert(district);
            // compute the middle of the district
            let mut pos = PositionVector::default();
            for j in connections.iter() {
                let c = Self::dictionary_3(*j);
                pos.0.push(c.geom_position());
            }
            let mut distCenter = pos.get_polygon_center();
            if connections.len() == 1 { // !!! ok, ok, maybe not the best way just to add an offset
                distCenter.add_xy(10.0, 10.0);
            }
            district.m().set_center(&distCenter);
            // build the node
            let id = format!("District{}", district.get_id());
            let districtNode = Ptr::from_mut(Box::leak(Box::new(NBNode::new_2(&id, district.get_position(), district))));
            if !nc.insert_2(districtNode) {
                panic!("1");
            }
        }
    }

    /// C++: `void NIVissimDistrictConnection::dict_BuildDistricts(NBDistrictCont &, NBEdgeCont &, NBNodeCont &)` [static]
    /// Builds the districts
    // void
    // NIVissimDistrictConnection::dict_BuildDistricts(NBDistrictCont& dc,
    //         NBEdgeCont& ec,
    //         NBNodeCont& nc) {
    //     // add the sources and sinks
    //     //  their normalised probability is computed within NBDistrict
    //     //   to avoid double code writing and more securty within the converter
    //     //  go through the district table
    //     for (std::map<int, std::vector<int> >::iterator k = myDistrictsConnections.begin(); k != myDistrictsConnections.end(); k++) {
    //         // get the connections
    //         const std::vector<int>& connections = (*k).second;
    //         // retrieve the current district
    //         NBDistrict* district =
    //             dc.retrieve(toString<int>((*k).first));
    //         NBNode* districtNode = nc.retrieve("District" + district->getID());
    //         assert(district != 0 && districtNode != 0);
    //
    //         for (std::vector<int>::const_iterator l = connections.begin(); l != connections.end(); l++) {
    //             NIVissimDistrictConnection* c = dictionary(*l);
    //             // get the edge to connect the parking place to
    //             NBEdge* e = ec.retrieve(toString<int>(c->myEdgeID));
    //             if (e == nullptr) {
    //                 e = ec.retrievePossiblySplit(toString<int>(c->myEdgeID), c->myPosition);
    //             }
    //             if (e == nullptr) {
    //                 WRITE_WARNINGF(TL("Could not build district '%' - edge '%' is missing."), toString<int>((*k).first), toString<int>(c->myEdgeID));
    //                 continue;
    //             }
    //             std::string id = "ParkingPlace" + toString<int>(*l);
    //             NBNode* parkingPlace = nc.retrieve(id);
    //             if (parkingPlace == nullptr) {
    //                 double pos = c->getPosition();
    //                 if (pos < e->getLength() - pos) {
    //                     parkingPlace = e->getFromNode();
    //                     parkingPlace->invalidateIncomingConnections();
    //                 } else {
    //                     parkingPlace = e->getToNode();
    //                     parkingPlace->invalidateOutgoingConnections();
    //                 }
    //             }
    //             assert(
    //                 e->getToNode() == parkingPlace
    //                 ||
    //                 e->getFromNode() == parkingPlace);
    //
    //             // build the connection to the source
    //             if (e->getFromNode() == parkingPlace) {
    //                 id = "VissimFromParkingplace" + toString<int>((*k).first) + "-" + toString<int>(c->myID);
    //                 NBEdge* source =
    //                     new NBEdge(id, districtNode, parkingPlace,
    //                                "Connection", c->getMeanSpeed(/*distc*/) / 3.6, NBEdge::UNSPECIFIED_FRICTION, 3, -1,
    //                                NBEdge::UNSPECIFIED_WIDTH, NBEdge::UNSPECIFIED_OFFSET, LaneSpreadFunction::RIGHT);
    //                 if (!ec.insert(source)) { // !!! in den Konstruktor
    //                     throw 1; // !!!
    //                 }
    //                 double percNormed =
    //                     c->myPercentages[(*k).first];
    //                 if (!district->addSource(source, percNormed)) {
    //                     throw 1;
    //                 }
    //             }
    //
    //             // build the connection to the destination
    //             if (e->getToNode() == parkingPlace) {
    //                 id = "VissimToParkingplace"  + toString<int>((*k).first) + "-" + toString<int>(c->myID);
    //                 NBEdge* destination =
    //                     new NBEdge(id, parkingPlace, districtNode,
    //                                "Connection", 100. / 3.6, NBEdge::UNSPECIFIED_FRICTION, 2, -1,
    //                                NBEdge::UNSPECIFIED_WIDTH, NBEdge::UNSPECIFIED_OFFSET, LaneSpreadFunction::RIGHT);
    //                 if (!ec.insert(destination)) { // !!! (in den Konstruktor)
    //                     throw 1; // !!!
    //                 }
    //                 double percNormed2 =
    //                     c->myPercentages[(*k).first];
    //                 if (!district->addSink(destination, percNormed2)) {
    //                     throw 1; // !!!
    //                 }
    //             }
    //
    //             /*
    //             if(e->getToNode()==districtNode) {
    //             double percNormed =
    //                 c->myPercentages[(*k).first];
    //             district->addSink(e, percNormed);
    //             }
    //             if(e->getFromNode()==districtNode) {
    //             double percNormed =
    //                 c->myPercentages[(*k).first];
    //             district->addSource(e, percNormed);
    //             }
    //             */
    //         }
    //
    //         /*
    //         // add them as sources and sinks to the current district
    //         for(std::vector<int>::const_iterator l=connections.begin(); l!=connections.end(); l++) {
    //             // get the current connections
    //             NIVissimDistrictConnection *c = dictionary(*l);
    //             // get the edge to connect the parking place to
    //             NBEdge *e = NBEdgeCont::retrieve(toString<int>(c->myEdgeID));
    //             Position edgepos = c->geomPosition();
    //             NBNode *edgeend = e->tryGetNodeAtPosition(c->myPosition,
    //                 e->getLength()/4.0);
    //             if(edgeend==0) {
    //                 // Edge splitting omitted on build district connections by now
    //                 assert(false);
    //             }
    //
    //             // build the district-node if not yet existing
    //             std::string id = "VissimParkingplace" + district->getID();
    //             NBNode *districtNode = nc.retrieve(id);
    //             assert(districtNode!=0);
    //
    //             if(e->getToNode()==edgeend) {
    //                 // build the connection to the source
    //                 id = std::string("VissimFromParkingplace")
    //                     + toString<int>((*k).first) + "-"
    //                     + toString<int>(c->myID);
    //                 NBEdge *source =
    //                     new NBEdge(id, id, districtNode, edgeend,
    //                     "Connection", 100/3.6, 2, 100, 0,
    //                     NBEdge::EDGEFUNCTION_SOURCE);
    //                 NBEdgeCont::insert(source); // !!! (in den Konstruktor)
    //                 double percNormed =
    //                     c->myPercentages[(*k).first];
    //                 district->addSource(source, percNormed);
    //             } else {
    //                 // build the connection to the destination
    //                 id = std::string("VissimToParkingplace")
    //                     + toString<int>((*k).first) + "-"
    //                     + toString<int>(c->myID);
    //                 NBEdge *destination =
    //                     new NBEdge(id, id, edgeend, districtNode,
    //                     "Connection", 100/3.6, 2, 100, 0,
    //                     NBEdge::EDGEFUNCTION_SINK);
    //                 NBEdgeCont::insert(destination); // !!! (in den Konstruktor)
    //
    //                 // add both the source and the sink to the district
    //                 double percNormed =
    //                     c->myPercentages[(*k).first];
    //                 district->addSink(destination, percNormed);
    //             }
    //         }
    //         */
    //     }
    // }
    #[allow(unused_variables)]
    pub fn dict_build_districts(dc: &mut NBDistrictCont, ec: &mut NBEdgeCont, nc: &mut NBNodeCont) {
        let districtsConnections = MY_DISTRICTS_CONNECTIONS.with(|m| m.borrow().clone());
        for (k, connections) in districtsConnections.iter() {
            // retrieve the current district
            let district = dc.retrieve(&k.to_string());
            let districtNode = nc.retrieve(&format!("District{}", district.get_id()));
            assert!(!district.is_null() && !districtNode.is_null());

            for l in connections.iter() {
                let c = Self::dictionary_3(*l);
                // get the edge to connect the parking place to
                let mut e = ec.retrieve(&c.myEdgeID.to_string(), false);
                if e.is_null() {
                    e = ec.retrieve_possibly_split_3(&c.myEdgeID.to_string(), c.myPosition);
                }
                if e.is_null() {
                    write_warningf!("Could not build district '%' - edge '%' is missing.", *k, c.myEdgeID);
                    continue;
                }
                let id = format!("ParkingPlace{}", l);
                let mut parkingPlace = nc.retrieve(&id);
                if parkingPlace.is_null() {
                    let pos = c.myPosition;
                    if pos < e.get_length() - pos {
                        parkingPlace = e.get_from_node();
                        parkingPlace.m().invalidate_incoming_connections(false);
                    } else {
                        parkingPlace = e.get_to_node();
                        parkingPlace.m().invalidate_outgoing_connections(false);
                    }
                }
                assert!(
                    e.get_to_node() == parkingPlace
                    ||
                    e.get_from_node() == parkingPlace);

                // build the connection to the source
                if e.get_from_node() == parkingPlace {
                    let id = format!("VissimFromParkingplace{}-{}", k, c.myID);
                    let source = Ptr::from_mut(Box::leak(NBEdge::new(&id, districtNode, parkingPlace, "Connection".to_string(), c.get_mean_speed() / 3.6, crate::netbuild::nb_edge::UNSPECIFIED_FRICTION, 3, -1,
                        crate::netbuild::nb_edge::UNSPECIFIED_WIDTH, crate::netbuild::nb_edge::UNSPECIFIED_OFFSET, LaneSpreadFunction::RIGHT, "")));
                    if !ec.insert(source, false) { // !!! in den Konstruktor
                        panic!("1");
                    }
                    let percNormed = *c.m().myPercentages.entry(*k).or_insert(0.0);
                    if !district.m().add_source(source, percNormed) {
                        panic!("1");
                    }
                }

                // build the connection to the destination
                if e.get_to_node() == parkingPlace {
                    let id = format!("VissimToParkingplace{}-{}", k, c.myID);
                    let destination = Ptr::from_mut(Box::leak(NBEdge::new(&id, parkingPlace, districtNode, "Connection".to_string(), 100.0 / 3.6, crate::netbuild::nb_edge::UNSPECIFIED_FRICTION, 2, -1,
                        crate::netbuild::nb_edge::UNSPECIFIED_WIDTH, crate::netbuild::nb_edge::UNSPECIFIED_OFFSET, LaneSpreadFunction::RIGHT, "")));
                    if !ec.insert(destination, false) { // !!! (in den Konstruktor)
                        panic!("1");
                    }
                    let percNormed2 = *c.m().myPercentages.entry(*k).or_insert(0.0);
                    if !district.m().add_sink(destination, percNormed2) {
                        panic!("1");
                    }
                }
            }
        }
    }

    /// C++: `NIVissimDistrictConnection * NIVissimDistrictConnection::dict_findForEdge(int)` [static]
    /// Returns the connection to a district placed at the given node Yep, there onyl should be one, there is no need to build a single edge as connection between two parking places
    // NIVissimDistrictConnection*
    // NIVissimDistrictConnection::dict_findForEdge(int edgeid) {
    //     for (DictType::iterator i = myDict.begin(); i != myDict.end(); i++) {
    //         if ((*i).second->myEdgeID == edgeid) {
    //             return (*i).second;
    //         }
    //     }
    //     return nullptr;
    // }
    #[allow(unused_variables)]
    pub fn dict_find_for_edge(edgeid: i32) -> Ptr<NIVissimDistrictConnection> {
        MY_DICT.with(|d| d.borrow().values().find(|c| c.myEdgeID == edgeid).copied().unwrap_or_else(Ptr::null))
    }

    /// C++: `void NIVissimDistrictConnection::clearDict()` [static]
    /// Clears the dictionary
    // void
    // NIVissimDistrictConnection::clearDict() {
    //     for (DictType::iterator i = myDict.begin(); i != myDict.end(); i++) {
    //         delete (*i).second;
    //     }
    //     myDict.clear();
    // }
    #[allow(unused_variables)]
    pub fn clear_dict() {
        // shortcut: the connections are leaked by dictionary(), so clearing only drops the map entries, upgrade to Box::from_raw when ownership is settled
        MY_DICT.with(|d| d.borrow_mut().clear());
    }

    /// C++: `void NIVissimDistrictConnection::dict_BuildDistrictConnections()` [static]
    // void
    // NIVissimDistrictConnection::dict_BuildDistrictConnections() {
    //     //  pre-assign connections to districts
    //     for (DictType::iterator i = myDict.begin(); i != myDict.end(); i++) {
    //         NIVissimDistrictConnection* c = (*i).second;
    //         const std::vector<int>& districts = c->myDistricts;
    //         for (std::vector<int>::const_iterator j = districts.begin(); j != districts.end(); j++) {
    //             // assign connection to district
    //             myDistrictsConnections[*j].push_back((*i).first);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn dict_build_district_connections() {
        //  pre-assign connections to districts
        MY_DICT.with(|d| {
            for (i, c) in d.borrow().iter() {
                for j in c.myDistricts.iter() {
                    // assign connection to district
                    MY_DISTRICTS_CONNECTIONS.with(|m| m.borrow_mut().entry(*j).or_default().push(*i));
                }
            }
        });
    }

    /// C++: `void NIVissimDistrictConnection::dict_CheckEdgeEnds()` [static]
    // void
    // NIVissimDistrictConnection::dict_CheckEdgeEnds() {
    //     for (std::map<int, std::vector<int> >::iterator k = myDistrictsConnections.begin(); k != myDistrictsConnections.end(); k++) {
    //         const std::vector<int>& connections = (*k).second;
    //         for (std::vector<int>::const_iterator j = connections.begin(); j != connections.end(); j++) {
    //             NIVissimDistrictConnection* c = dictionary(*j);
    //             c->checkEdgeEnd();
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn dict_check_edge_ends() {
        let districtsConnections = MY_DISTRICTS_CONNECTIONS.with(|m| m.borrow().clone());
        for (_, connections) in districtsConnections.iter() {
            for j in connections.iter() {
                let c = Self::dictionary_3(*j);
                c.m().check_edge_end();
            }
        }
    }

    /// C++: `void NIVissimDistrictConnection::checkEdgeEnd()` [private]
    // void
    // NIVissimDistrictConnection::checkEdgeEnd() {
    //     NIVissimEdge* edge = NIVissimEdge::dictionary(myEdgeID);
    //     assert(edge != 0);
    //     edge->checkDistrictConnectionExistanceAt(myPosition);
    // }
    #[allow(unused_variables)]
    pub fn check_edge_end(&mut self) {
        let edge = NIVissimEdge::dictionary_3(self.myEdgeID);
        assert!(!edge.is_null());
        edge.m().check_district_connection_existance_at(self.myPosition);
    }

    /// C++: `double NIVissimDistrictConnection::getRealSpeed(int) const` [const, private]
    // double
    // NIVissimDistrictConnection::getRealSpeed(int distNo) const {
    //     std::string id = toString<int>(distNo);
    //     Distribution* dist = DistributionCont::dictionary("speed", id);
    //     if (dist == nullptr) {
    //         WRITE_WARNINGF(TL("The referenced speed distribution '%' is not known."), id);
    //         WRITE_WARNING(TL(". Using default."));
    //         return OptionsCont::getOptions().getFloat("vissim.default-speed");
    //     }
    //     assert(dist != 0);
    //     double speed = dist->getMax();
    //     if (speed < 0 || speed > 1000) {
    //         WRITE_WARNING(" False speed at district '" + id);
    //         WRITE_WARNING(TL(". Using default."));
    //         speed = OptionsCont::getOptions().getFloat("vissim.default-speed");
    //     }
    //     return speed;
    // }
    #[allow(unused_variables)]
    pub fn get_real_speed(&self, distNo: i32) -> f64 {
        let id = distNo.to_string();
        let dist = match utils::distribution::distribution_cont::dictionary_get("speed", &id) {
            Some(d) => d,
            None => {
                write_warningf!("The referenced speed distribution '%' is not known.", id);
                utils::common::msg_handler::write_warning(". Using default.");
                return utils::options::options_cont::get_float("vissim.default-speed");
            }
        };
        let mut speed = dist.get_max();
        if speed < 0.0 || speed > 1000.0 {
            utils::common::msg_handler::write_warning(&format!(" False speed at district '{}", id));
            utils::common::msg_handler::write_warning(". Using default.");
            speed = utils::options::options_cont::get_float("vissim.default-speed");
        }
        speed
    }

}

