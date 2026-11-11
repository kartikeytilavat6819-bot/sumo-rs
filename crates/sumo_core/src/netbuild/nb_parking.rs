//! Port of netbuild/NBParking.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// The representation of an imported parking area
/// C++ class `NBParking` (netbuild/NBParking.h)
pub struct NBParking {
    /// C++ base class `Named`
    pub base: Named,
    pub myEdgeID: String,
    pub myName: String,
}

impl std::ops::Deref for NBParking {
    type Target = Named;
    fn deref(&self) -> &Named {
        &self.base
    }
}
impl std::ops::DerefMut for NBParking {
    fn deref_mut(&mut self) -> &mut Named {
        &mut self.base
    }
}

impl NBParking {
    /// C++: `void NBParking::NBParking(const std::string &, const std::string &, const std::string &)`
    /// Constructor
    // NBParking::NBParking(const std::string& id, const std::string& edgeID, const std::string& name) :
    //     Named(id),
    //     myEdgeID(edgeID),
    //     myName(name) {
    // }
    #[allow(unused_variables)]
    pub fn new(id: &str, edgeID: &str, name: &str) -> Self {
        NBParking { base: Named::new(id), myEdgeID: edgeID.to_string(), myName: name.to_string() }
    }

    /// C++: `void NBParking::write(OutputDevice &, NBEdgeCont &) const` [const]
    // void
    // NBParking::write(OutputDevice& device, NBEdgeCont& ec) const {
    //     const NBEdge* e = ec.retrieve(myEdgeID);
    //     if (e != nullptr) {
    //         if ((e->getPermissions() & SVC_PASSENGER) == 0) {
    //             WRITE_WARNINGF(TL("Ignoring parking area on edge '%' due to invalid permissions."), e->getID());
    //             return;
    //         }
    //         // keep minimum distance of 5m to junction corners
    //         const int cornerDistance = 5;
    //         int capacity = (int)((e->getFinalLength() - 2 * cornerDistance) / 7.5);
    //         if (capacity <= 0) {
    //             WRITE_WARNINGF(TL("Ignoring parking area on edge '%' due to insufficient space."), e->getID());
    //             return;
    //         }
    //         int lane = 0;
    //         for (; lane < e->getNumLanes(); ++lane) {
    //             if ((e->getPermissions(lane) & SVC_PASSENGER) != 0) {
    //                 break;
    //             }
    //         }
    //         device.openTag(SUMO_TAG_PARKING_AREA);
    //         device.writeAttr(SUMO_ATTR_ID, getID());
    //         device.writeAttr(SUMO_ATTR_LANE, e->getLaneID(lane));
    //         device.writeAttr(SUMO_ATTR_STARTPOS, cornerDistance);
    //         device.writeAttr(SUMO_ATTR_ENDPOS, -cornerDistance);
    //         device.writeAttr(SUMO_ATTR_ROADSIDE_CAPACITY, capacity);
    //         if (!myName.empty()) {
    //             device.writeAttr(SUMO_ATTR_NAME, myName);
    //         }
    //         device.closeTag();
    //     } else {
    //         WRITE_WARNINGF(TL("Could not find edge for parkingArea '%'."), getID());
    //     }
    //     // XXX else: prevent edge merging via --geometry.remove
    // }
    #[allow(unused_variables)]
    pub fn write(&self, device: &mut OutputDevice, ec: &mut NBEdgeCont) {
        let e: Ptr<NBEdge> = ec.retrieve(&self.myEdgeID, false);
        if !e.is_null() {
            if (e.get_permissions(-1) & SVC_PASSENGER) == 0 {
                write_warningf!("Ignoring parking area on edge '%' due to invalid permissions.", e.get_id());
                return;
            }
            // keep minimum distance of 5m to junction corners
            let corner_distance: i32 = 5;
            let capacity: i32 = ((e.get_final_length() - 2.0 * corner_distance as f64) / 7.5) as i32;
            if capacity <= 0 {
                write_warningf!("Ignoring parking area on edge '%' due to insufficient space.", e.get_id());
                return;
            }
            let mut lane: i32 = 0;
            while lane < e.get_num_lanes() {
                if (e.get_permissions(lane) & SVC_PASSENGER) != 0 {
                    break;
                }
                lane += 1;
            }
            device.open_tag_id(SUMO_TAG_PARKING_AREA);
            device.write_attr(SUMO_ATTR_ID, self.get_id());
            device.write_attr(SUMO_ATTR_LANE, &e.get_lane_id(lane));
            device.write_attr(SUMO_ATTR_STARTPOS, &corner_distance);
            device.write_attr(SUMO_ATTR_ENDPOS, &-corner_distance);
            device.write_attr(SUMO_ATTR_ROADSIDE_CAPACITY, &capacity);
            if !self.myName.is_empty() {
                device.write_attr(SUMO_ATTR_NAME, &self.myName);
            }
            device.close_tag("");
        } else {
            write_warningf!("Could not find edge for parkingArea '%'.", self.get_id());
        }
        // XXX else: prevent edge merging via --geometry.remove
    }

    /// C++: `const std::string NBParking::getEdgeID() const` [const]
    // const std::string getEdgeID() const {
    //         return myEdgeID;
    //     }
    #[allow(unused_variables)]
    pub fn get_edge_id(&self) -> String {
        self.myEdgeID.clone()
    }

}

/// C++ class `NBParkingCont` (netbuild/NBParking.h)
pub struct NBParkingCont {
    /// C++ base class `std::vector<NBParking>`
    pub base: Vec<NBParking>,
}

impl std::ops::Deref for NBParkingCont {
    type Target = Vec<NBParking>;
    fn deref(&self) -> &Vec<NBParking> {
        &self.base
    }
}
impl std::ops::DerefMut for NBParkingCont {
    fn deref_mut(&mut self) -> &mut Vec<NBParking> {
        &mut self.base
    }
}

impl NBParkingCont {
    /// C++: `void NBParkingCont::addEdges2Keep(const OptionsCont &, std::set<std::string> &)`
    /// add edges that must be kept
    // void
    // NBParkingCont::addEdges2Keep(const OptionsCont& oc, std::set<std::string>& into) {
    //     if (oc.exists("parking-output") && oc.isSet("parking-output")) {
    //         for (NBParking& p : *this) {
    //             into.insert(p.getEdgeID());
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_edges2_keep(&mut self, oc: &OptionsCont, into: &mut BTreeSet<String>) {
        if oc.exists("parking-output") && oc.is_set("parking-output", true).unwrap_or(false) {
            for p in self.base.iter() {
                into.insert(p.get_edge_id());
            }
        }
    }

}

