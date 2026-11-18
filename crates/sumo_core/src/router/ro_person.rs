//! Port of router/ROPerson.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

const VEHPARS_VTYPE_SET: i64 = 2;
const VEHPARS_DEPARTPOS_SET: i64 = 2 << 2;
const LINE_ANY: &str = "ANY";
// C++ static const std::string ROPerson::PlanItem::UNDEFINED_STOPPING_PLACE (empty string)
const UNDEFINED_STOPPING_PLACE: &str = "";
// C++ static bool ROPerson::myHaveWarnedPTMissing
static MY_HAVE_WARNED_PT_MISSING: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// A planItem can be a Trip which contains multiple tripItems
/// C++ class `ROPerson::PersonTrip` (router/ROPerson.h)
#[repr(C)]
pub struct ROPerson_PersonTrip {
    /// C++ base class `PlanItem`
    pub base: ROPerson_PlanItem,
    pub from: Ptr<ROEdge>,
    pub to: Ptr<ROEdge>,
    pub modes: SVCPermissions,
    pub dep: f64,
    pub arr: f64,
    pub stopOrigin: String,
    pub stopDest: String,
    /// the fully specified trips
    pub myTripItems: Vec<Ptr<ROPerson_TripItem>>,
    /// the vehicles which may be used for routing
    pub myVehicles: Vec<Ptr<ROVehicle>>,
    /// walking speed factor
    pub walkFactor: f64,
    /// group id for travelling in groups
    pub group: String,
}

impl std::ops::Deref for ROPerson_PersonTrip {
    type Target = ROPerson_PlanItem;
    fn deref(&self) -> &ROPerson_PlanItem {
        &self.base
    }
}
impl std::ops::DerefMut for ROPerson_PersonTrip {
    fn deref_mut(&mut self) -> &mut ROPerson_PlanItem {
        &mut self.base
    }
}

impl ROPerson_PersonTrip {
    /// C++: `void PersonTrip::PersonTrip(const ROEdge *, const std::string)`
    // PersonTrip(const ROEdge* _to, const std::string _stopDest) :
    //             from(0), to(_to), modes(SVC_PEDESTRIAN), dep(0), arr(0), stopDest(_stopDest), walkFactor(1.0) {}
    #[allow(unused_variables)]
    pub fn new(_to: Ptr<ROEdge>, _stopDest: String) -> Self {
        ROPerson_PersonTrip {
            base: ROPerson_PlanItem { base: Parameterised::default(), kind: 1 },
            from: Ptr::null(),
            to: _to,
            modes: SVC_PEDESTRIAN,
            dep: 0.,
            arr: 0.,
            stopOrigin: String::new(),
            stopDest: _stopDest,
            myTripItems: Vec::new(),
            myVehicles: Vec::new(),
            walkFactor: 1.0,
            group: String::new(),
        }
    }

    /// C++: `void PersonTrip::PersonTrip(const ROEdge *const, const ROEdge *const, const SVCPermissions, const double, const std::string &, const double, const std::string &, double, const std::string &)`
    // PersonTrip(const ROEdge* const _from, const ROEdge* const _to, const SVCPermissions modeSet,
    //                    const double departPos, const std::string& _stopOrigin, const double arrivalPos, const std::string& _stopDest, double _walkFactor, const std::string& _group) :
    //             from(_from), to(_to), modes(modeSet), dep(departPos), arr(arrivalPos), stopOrigin(_stopOrigin), stopDest(_stopDest), walkFactor(_walkFactor), group(_group) { }
    #[allow(unused_variables)]
    pub fn new_2(_from: Ptr<ROEdge>, _to: Ptr<ROEdge>, modeSet: SVCPermissions, departPos: f64, _stopOrigin: &str, arrivalPos: f64, _stopDest: &str, _walkFactor: f64, _group: &str) -> Self {
        ROPerson_PersonTrip {
            base: ROPerson_PlanItem { base: Parameterised::default(), kind: 1 },
            from: _from,
            to: _to,
            modes: modeSet,
            dep: departPos,
            arr: arrivalPos,
            stopOrigin: _stopOrigin.to_string(),
            stopDest: _stopDest.to_string(),
            myTripItems: Vec::new(),
            myVehicles: Vec::new(),
            walkFactor: _walkFactor,
            group: _group.to_string(),
        }
    }

    /// C++: `void PersonTrip::~PersonTrip() noexcept` [virtual]
    /// Destructor
    // virtual ~PersonTrip() {
    //             for (TripItem* const it : myTripItems) {
    //                 delete it;
    //             }
    //             for (ROVehicle* const v : myVehicles) {
    //                 delete v->getRouteDefinition();
    //                 delete v;
    //             }
    //         }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // shortcut: items and vehicles are freed through their base types until virtual destructors are dispatched
        for it in self.myTripItems.drain(..) {
            it.m().drop_();
            drop(unsafe { Box::from_raw(it.as_raw()) });
        }
        for v in self.myVehicles.drain(..) {
            let rd = v.get_route_definition();
            drop(unsafe { Box::from_raw(rd.as_raw()) });
            drop(unsafe { Box::from_raw(v.as_raw()) });
        }
    }

    /// C++: `PlanItem * PersonTrip::clone() const` [const, virtual]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn clone(&self) -> Ptr<ROPerson_PlanItem> {
        let result: Ptr<ROPerson_PersonTrip> = Ptr::from_raw(Box::into_raw(Box::new(ROPerson_PersonTrip::new_2(self.from, self.to, self.modes, self.dep, &self.stopOrigin, self.arr, &self.stopDest, self.walkFactor, &self.group))));
        for item in self.myTripItems.iter() {
            result.m().myTripItems.push(ROPerson_TripItem::clone(item));
        }
        result.cast::<ROPerson_PlanItem>()
    }

    /// C++: `void PersonTrip::addTripItem(TripItem *)` [virtual]
    // virtual void addTripItem(TripItem* tripIt) {
    //             myTripItems.push_back(tripIt);
    //         }
    #[allow(unused_variables)]
    pub fn add_trip_item(&mut self, tripIt: Ptr<ROPerson_TripItem>) {
        self.myTripItems.push(tripIt);
    }

    /// C++: `void PersonTrip::addVehicle(ROVehicle *)`
    // void addVehicle(ROVehicle* veh) {
    //             myVehicles.push_back(veh);
    //         }
    #[allow(unused_variables)]
    pub fn add_vehicle(&mut self, veh: Ptr<ROVehicle>) {
        self.myVehicles.push(veh);
    }

    /// C++: `std::vector<ROVehicle *> & PersonTrip::getVehicles()`
    // std::vector<ROVehicle*>& getVehicles() {
    //             return myVehicles;
    //         }
    #[allow(unused_variables)]
    pub fn get_vehicles(&mut self) -> &mut Vec<Ptr<ROVehicle>> {
        &mut self.myVehicles
    }

    /// C++: `const ROEdge * PersonTrip::getOrigin() const` [const, virtual]
    // const ROEdge* getOrigin() const {
    //             return from != 0 ? from : myTripItems.front()->getOrigin();
    //         }
    #[allow(unused_variables)]
    pub fn get_origin(&self) -> Ptr<ROEdge> {
        if !self.from.is_null() { self.from } else { self.myTripItems[0].get_origin() }
    }

    /// C++: `const ROEdge * PersonTrip::getDestination() const` [const, virtual]
    // const ROEdge* getDestination() const {
    //             return to;
    //         }
    #[allow(unused_variables)]
    pub fn get_destination(&self) -> Ptr<ROEdge> {
        self.to
    }

    /// C++: `double PersonTrip::getDestinationPos() const` [const, virtual]
    // double getDestinationPos() const {
    //             if (myTripItems.empty()) {
    //                 return getArrivalPos(true);
    //             } else {
    //                 return myTripItems.back()->getDestinationPos();
    //             }
    //         }
    #[allow(unused_variables)]
    pub fn get_destination_pos(&self) -> f64 {
        if self.myTripItems.is_empty() {
            self.get_arrival_pos(true)
        } else {
            self.myTripItems[self.myTripItems.len() - 1].get_destination_pos()
        }
    }

    /// C++: `double PersonTrip::getDepartPos(bool) const` [const]
    // double getDepartPos(bool replaceDefault = true) const {
    //             return dep == std::numeric_limits<double>::infinity() && replaceDefault ? 0 : dep;
    //         }
    #[allow(unused_variables)]
    pub fn get_depart_pos(&self, replaceDefault: bool) -> f64 {
        if self.dep == f64::INFINITY && replaceDefault { 0. } else { self.dep }
    }

    /// C++: `double PersonTrip::getArrivalPos(bool) const` [const]
    // double getArrivalPos(bool replaceDefault = true) const {
    //             return arr == std::numeric_limits<double>::infinity() && replaceDefault ? to->getLength() / 2 : arr;
    //         }
    #[allow(unused_variables)]
    pub fn get_arrival_pos(&self, replaceDefault: bool) -> f64 {
        if self.arr == f64::INFINITY && replaceDefault { self.to.get_length() / 2. } else { self.arr }
    }

    /// C++: `SVCPermissions PersonTrip::getModes() const` [const]
    // SVCPermissions getModes() const {
    //             return modes;
    //         }
    #[allow(unused_variables)]
    pub fn get_modes(&self) -> SVCPermissions {
        self.modes
    }

    /// C++: `void PersonTrip::updateModes(SVCPermissions)`
    // void updateModes(SVCPermissions additionalModes) {
    //             modes |= additionalModes;
    //         }
    #[allow(unused_variables)]
    pub fn update_modes(&mut self, additionalModes: SVCPermissions) {
        self.modes |= additionalModes;
    }

    /// C++: `const std::string & PersonTrip::getGroup() const` [const]
    // const std::string& getGroup() const {
    //             return group;
    //         }
    #[allow(unused_variables)]
    pub fn get_group(&self) -> &str {
        self.group.as_str()
    }

    /// C++: `const std::string & PersonTrip::getStopOrigin() const` [const]
    // const std::string& getStopOrigin() const {
    //             return stopOrigin;
    //         }
    #[allow(unused_variables)]
    pub fn get_stop_origin(&self) -> &str {
        self.stopOrigin.as_str()
    }

    /// C++: `const std::string & PersonTrip::getStopDest() const` [const, virtual]
    // const std::string& getStopDest() const {
    //             return stopDest;
    //         }
    #[allow(unused_variables)]
    pub fn get_stop_dest(&self) -> &str {
        self.stopDest.as_str()
    }

    /// C++: `bool PersonTrip::needsRouting() const` [const, virtual]
    // virtual bool needsRouting() const {
    //             return myTripItems.empty();
    //         }
    #[allow(unused_variables)]
    pub fn needs_routing(&self) -> bool {
        self.myTripItems.is_empty()
    }

    /// C++: `void PersonTrip::setItems(std::vector<TripItem *> &, const ROVehicle *const)`
    // void setItems(std::vector<TripItem*>& newItems, const ROVehicle* const veh) {
    //             assert(myTripItems.empty());
    //             myTripItems.swap(newItems);
    //             for (auto it = myVehicles.begin(); it != myVehicles.end();) {
    //                 if (*it != veh) {
    //                     delete (*it)->getRouteDefinition();
    //                     delete (*it);
    //                     it = myVehicles.erase(it);
    //                 } else {
    //                     it++;
    //                 }
    //             }
    //         }
    #[allow(unused_variables)]
    pub fn set_items(&mut self, newItems: &mut Vec<Ptr<ROPerson_TripItem>>, veh: Ptr<ROVehicle>) {
        debug_assert!(self.myTripItems.is_empty());
        std::mem::swap(&mut self.myTripItems, newItems);
        let mut kept: Vec<Ptr<ROVehicle>> = Vec::new();
        for v in std::mem::take(&mut self.myVehicles) {
            if v != veh {
                let rd = v.get_route_definition();
                drop(unsafe { Box::from_raw(rd.as_raw()) });
                drop(unsafe { Box::from_raw(v.as_raw()) });
            } else {
                kept.push(v);
            }
        }
        self.myVehicles = kept;
    }

    /// C++: `void PersonTrip::saveVehicles(OutputDevice &, OutputDevice *const, bool, OptionsCont &) const` [const, virtual]
    // void
    // ROPerson::PersonTrip::saveVehicles(OutputDevice& os, OutputDevice* const typeos, bool asAlternatives, OptionsCont& options) const {
    //     for (ROVehicle* veh : myVehicles) {
    //         if (!RONet::getInstance()->knowsVehicle(veh->getID())) {
    //             veh->saveAsXML(os, typeos, asAlternatives, options);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn save_vehicles(&self, os: &mut OutputDevice, typeos: Ptr<OutputDevice>, asAlternatives: bool, options: &mut OptionsCont) {
        for veh in self.myVehicles.iter() {
            if !RONet::get_instance().knows_vehicle(veh.get_id()) {
                veh.save_as_xml(os, typeos, asAlternatives, options, 0);
            }
        }
    }

    /// C++: `void PersonTrip::saveAsXML(OutputDevice &, const bool, const bool, OptionsCont &) const` [const, virtual]
    // void
    // ROPerson::PersonTrip::saveAsXML(OutputDevice& os, const bool extended, const bool asTrip, OptionsCont& options) const {
    //     if ((asTrip || extended) && from != nullptr) {
    //         const bool writeGeoTrip = asTrip && options.getBool("write-trips.geo");
    //         os.openTag(SUMO_TAG_PERSONTRIP);
    //         if (writeGeoTrip) {
    //             Position fromPos = from->getLanes()[0]->getShape().positionAtOffset2D(getDepartPos());
    //             if (GeoConvHelper::getFinal().usingGeoProjection()) {
    //                 os.setPrecision(gPrecisionGeo);
    //                 GeoConvHelper::getFinal().cartesian2geo(fromPos);
    //                 os.writeAttr(SUMO_ATTR_FROMLONLAT, fromPos);
    //                 os.setPrecision(gPrecision);
    //             } else {
    //                 os.writeAttr(SUMO_ATTR_FROMXY, fromPos);
    //             }
    //         } else {
    //             os.writeAttr(SUMO_ATTR_FROM, from->getID());
    //         }
    //         if (writeGeoTrip) {
    //             Position toPos = to->getLanes()[0]->getShape().positionAtOffset2D(MIN2(getArrivalPos(), to->getLanes()[0]->getShape().length2D()));
    //             if (GeoConvHelper::getFinal().usingGeoProjection()) {
    //                 os.setPrecision(gPrecisionGeo);
    //                 GeoConvHelper::getFinal().cartesian2geo(toPos);
    //                 os.writeAttr(SUMO_ATTR_TOLONLAT, toPos);
    //                 os.setPrecision(gPrecision);
    //             } else {
    //                 os.writeAttr(SUMO_ATTR_TOXY, toPos);
    //             }
    //         } else {
    //             os.writeAttr(SUMO_ATTR_TO, to->getID());
    //         }
    //         std::vector<std::string> allowedModes;
    //         if ((modes & SVC_BUS) != 0) {
    //             allowedModes.push_back("public");
    //         }
    //         if ((modes & SVC_PASSENGER) != 0) {
    //             allowedModes.push_back("car");
    //         }
    //         if ((modes & SVC_TAXI) != 0) {
    //             allowedModes.push_back("taxi");
    //         }
    //         if ((modes & SVC_BICYCLE) != 0) {
    //             allowedModes.push_back("bicycle");
    //         }
    //         if (allowedModes.size() > 0) {
    //             os.writeAttr(SUMO_ATTR_MODES, toString(allowedModes));
    //         }
    //         if (!writeGeoTrip) {
    //             if (dep != 0 && dep != std::numeric_limits<double>::infinity()) {
    //                 os.writeAttr(SUMO_ATTR_DEPARTPOS, dep);
    //             }
    //             if (arr != 0 && arr != std::numeric_limits<double>::infinity()) {
    //                 os.writeAttr(SUMO_ATTR_ARRIVALPOS, arr);
    //             }
    //         }
    //         if (getStopDest() != "") {
    //             os.writeAttr(SUMO_ATTR_BUS_STOP, getStopDest());
    //         }
    //         if (walkFactor != 1) {
    //             os.writeAttr(SUMO_ATTR_WALKFACTOR, walkFactor);
    //         }
    //         if (extended && myTripItems.size() != 0) {
    //             std::vector<double> costs;
    //             for (const TripItem* const tripItem : myTripItems) {
    //                 costs.push_back(tripItem->getCost());
    //             }
    //             os.writeAttr(SUMO_ATTR_COSTS, costs);
    //         }
    //         os.closeTag();
    //     } else {
    //         for (const TripItem* const it : myTripItems) {
    //             it->saveAsXML(os, extended, options);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn save_as_xml(&self, os: &mut OutputDevice, extended: bool, asTrip: bool, options: &mut OptionsCont) {
        if (asTrip || extended) && !self.from.is_null() {
            let writeGeoTrip = asTrip && options.get_bool("write-trips.geo").unwrap_or_else(|e| panic!("{e}"));
            os.open_tag_id(SUMO_TAG_PERSONTRIP);
            if writeGeoTrip {
                let mut fromPos = self.from.get_lanes()[0].get_shape().position_at_offset2d(self.get_depart_pos(true), 0., false);
                if GeoConvHelper::with_final(|g| g.using_geo_projection()) {
                    os.set_precision(utils::common::std_defs::g_precision_geo());
                    GeoConvHelper::with_final(|g| g.cartesian2geo(&mut fromPos));
                    os.write_attr(SUMO_ATTR_FROMLONLAT, &utils::common::to_string::ToStringPrec::to_string_prec(&fromPos, utils::common::std_defs::g_precision_geo()));
                    os.set_precision(utils::common::std_defs::g_precision());
                } else {
                    os.write_attr(SUMO_ATTR_FROMXY, &utils::common::to_string::ToStringPrec::to_string_prec(&fromPos, utils::common::std_defs::g_precision()));
                }
            } else {
                os.write_attr(SUMO_ATTR_FROM, self.from.get_id());
            }
            if writeGeoTrip {
                let shape = self.to.get_lanes()[0].get_shape();
                let len = shape.length2d();
                let arrPos = self.get_arrival_pos(true);
                let offset = if arrPos < len { arrPos } else { len };
                let mut toPos = shape.position_at_offset2d(offset, 0., false);
                if GeoConvHelper::with_final(|g| g.using_geo_projection()) {
                    os.set_precision(utils::common::std_defs::g_precision_geo());
                    GeoConvHelper::with_final(|g| g.cartesian2geo(&mut toPos));
                    os.write_attr(SUMO_ATTR_TOLONLAT, &utils::common::to_string::ToStringPrec::to_string_prec(&toPos, utils::common::std_defs::g_precision_geo()));
                    os.set_precision(utils::common::std_defs::g_precision());
                } else {
                    os.write_attr(SUMO_ATTR_TOXY, &utils::common::to_string::ToStringPrec::to_string_prec(&toPos, utils::common::std_defs::g_precision()));
                }
            } else {
                os.write_attr(SUMO_ATTR_TO, self.to.get_id());
            }
            let mut allowedModes: Vec<String> = Vec::new();
            if (self.modes & SVC_BUS) != 0 { allowedModes.push("public".to_string()); }
            if (self.modes & SVC_PASSENGER) != 0 { allowedModes.push("car".to_string()); }
            if (self.modes & SVC_TAXI) != 0 { allowedModes.push("taxi".to_string()); }
            if (self.modes & SVC_BICYCLE) != 0 { allowedModes.push("bicycle".to_string()); }
            if allowedModes.len() > 0 {
                os.write_attr(SUMO_ATTR_MODES, &allowedModes);
            }
            if !writeGeoTrip {
                if self.dep != 0. && self.dep != f64::INFINITY {
                    os.write_attr(SUMO_ATTR_DEPARTPOS, &self.dep);
                }
                if self.arr != 0. && self.arr != f64::INFINITY {
                    os.write_attr(SUMO_ATTR_ARRIVALPOS, &self.arr);
                }
            }
            if self.get_stop_dest() != "" {
                os.write_attr(SUMO_ATTR_BUS_STOP, self.get_stop_dest());
            }
            if self.walkFactor != 1. {
                os.write_attr(SUMO_ATTR_WALKFACTOR, &self.walkFactor);
            }
            if extended && self.myTripItems.len() != 0 {
                let costs: Vec<f64> = self.myTripItems.iter().map(|t| t.get_cost()).collect();
                os.write_attr(SUMO_ATTR_COSTS, &costs);
            }
            os.close_tag("");
        } else {
            for it in self.myTripItems.iter() {
                it.save_as_xml(os, extended, options);
            }
        }
    }

    /// C++: `double PersonTrip::getWalkFactor() const` [const]
    // double getWalkFactor() const {
    //             return walkFactor;
    //         }
    #[allow(unused_variables)]
    pub fn get_walk_factor(&self) -> f64 {
        self.walkFactor
    }

    /// C++: `SUMOTime PersonTrip::getDuration() const` [const, virtual]
    /// return duration sum of all trip items
    // SUMOTime
    // ROPerson::PersonTrip::getDuration() const {
    //     SUMOTime result = 0;
    //     for (TripItem* tItem : myTripItems) {
    //         result += tItem->getDuration();
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_duration(&self) -> SUMOTime {
        let mut result: SUMOTime = 0;
        for tItem in self.myTripItems.iter() {
            result += tItem.get_duration();
        }
        result
    }

    /// C++: `PersonTrip & PersonTrip::operator=(const PersonTrip &)` [private]
    /// Invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, src: &ROPerson_PersonTrip) -> &mut ROPerson_PersonTrip {
        panic!("ROPerson::PersonTrip::operator= is private and undefined in C++ (invalidated copy)")
    }

}

/// Every person has a plan comprising of multiple planItems
/// C++ class `ROPerson::PlanItem` (router/ROPerson.h)
#[repr(C)]
pub struct ROPerson_PlanItem {
    /// C++ base class `Parameterised`
    pub base: Parameterised,
    /// determinism-free virtual dispatch tag: 1 = PersonTrip, 2 = Stop (C++ vtable)
    pub kind: u8,
}

impl std::ops::Deref for ROPerson_PlanItem {
    type Target = Parameterised;
    fn deref(&self) -> &Parameterised {
        &self.base
    }
}
impl std::ops::DerefMut for ROPerson_PlanItem {
    fn deref_mut(&mut self) -> &mut Parameterised {
        &mut self.base
    }
}

// C++ static members of PlanItem (port as module-level statics, see AGENTS.md):
//   static const std::string PlanItem::UNDEFINED_STOPPING_PLACE;

impl ROPerson_PlanItem {
    /// C++: `void PlanItem::~PlanItem() noexcept` [virtual]
    /// Destructor
    // virtual ~PlanItem() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
    }

    /// C++: `PlanItem * PlanItem::clone() const` [const, virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn clone(&self) -> Ptr<ROPerson_PlanItem> {
        match self.kind {
            1 => Ptr::from_raw(self as *const _ as *mut Self).cast::<ROPerson_PersonTrip>().m().clone(),
            2 => Ptr::from_raw(self as *const _ as *mut Self).cast::<ROPerson_Stop>().m().clone(),
            _ => panic!("ROPerson::PlanItem::clone is pure virtual"),
        }
    }

    /// C++: `void PlanItem::addTripItem(TripItem *)` [virtual]
    // virtual void addTripItem(TripItem* /* tripIt */) {
    //             throw ProcessError();
    //         }
    #[allow(unused_variables)]
    pub fn add_trip_item(&mut self, arg0: Ptr<ROPerson_TripItem>) {
        panic!("{}", utils::common::util_exceptions::ProcessError::Process(String::new()))
    }

    /// C++: `const ROEdge * PlanItem::getOrigin() const` [const, virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_origin(&self) -> Ptr<ROEdge> {
        match self.kind {
            1 => Ptr::from_raw(self as *const _ as *mut Self).cast::<ROPerson_PersonTrip>().m().get_origin(),
            2 => Ptr::from_raw(self as *const _ as *mut Self).cast::<ROPerson_Stop>().m().get_origin(),
            _ => panic!("ROPerson::PlanItem::getOrigin is pure virtual"),
        }
    }

    /// C++: `const ROEdge * PlanItem::getDestination() const` [const, virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_destination(&self) -> Ptr<ROEdge> {
        match self.kind {
            1 => Ptr::from_raw(self as *const _ as *mut Self).cast::<ROPerson_PersonTrip>().m().get_destination(),
            2 => Ptr::from_raw(self as *const _ as *mut Self).cast::<ROPerson_Stop>().m().get_destination(),
            _ => panic!("ROPerson::PlanItem::getDestination is pure virtual"),
        }
    }

    /// C++: `double PlanItem::getDestinationPos() const` [const, virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_destination_pos(&self) -> f64 {
        match self.kind {
            1 => Ptr::from_raw(self as *const _ as *mut Self).cast::<ROPerson_PersonTrip>().m().get_destination_pos(),
            2 => Ptr::from_raw(self as *const _ as *mut Self).cast::<ROPerson_Stop>().m().get_destination_pos(),
            _ => panic!("ROPerson::PlanItem::getDestinationPos is pure virtual"),
        }
    }

    /// C++: `void PlanItem::saveVehicles(OutputDevice &, OutputDevice *const, bool, OptionsCont &) const` [const, virtual]
    // virtual void saveVehicles(OutputDevice& /* os */, OutputDevice* const /* typeos */, bool /* asAlternatives */, OptionsCont& /* options */) const {}
    #[allow(unused_variables)]
    pub fn save_vehicles(&self, arg0: &mut OutputDevice, arg1: Ptr<OutputDevice>, arg2: bool, arg3: &mut OptionsCont) {
        if self.kind == 1 {
            Ptr::from_raw(self as *const _ as *mut Self).cast::<ROPerson_PersonTrip>().m().save_vehicles(arg0, arg1, arg2, arg3);
        }
    }

    /// C++: `void PlanItem::saveAsXML(OutputDevice &, const bool, const bool, OptionsCont &) const` [const, virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn save_as_xml(&self, os: &mut OutputDevice, extended: bool, asTrip: bool, options: &mut OptionsCont) {
        match self.kind {
            1 => Ptr::from_raw(self as *const _ as *mut Self).cast::<ROPerson_PersonTrip>().m().save_as_xml(os, extended, asTrip, options),
            2 => Ptr::from_raw(self as *const _ as *mut Self).cast::<ROPerson_Stop>().m().save_as_xml(os, extended, asTrip, options),
            _ => panic!("ROPerson::PlanItem::saveAsXML is pure virtual"),
        }
    }

    /// C++: `bool PlanItem::isStop() const` [const, virtual]
    // virtual bool isStop() const {
    //             return false;
    //         }
    #[allow(unused_variables)]
    pub fn is_stop(&self) -> bool {
        self.kind == 2
    }

    /// C++: `bool PlanItem::needsRouting() const` [const, virtual]
    // virtual bool needsRouting() const {
    //             return false;
    //         }
    #[allow(unused_variables)]
    pub fn needs_routing(&self) -> bool {
        self.kind == 1 && Ptr::from_raw(self as *const _ as *mut Self).cast::<ROPerson_PersonTrip>().m().needs_routing()
    }

    /// C++: `SUMOVehicleParameter::Stop * PlanItem::getStopParameters()` [virtual]
    // virtual SUMOVehicleParameter::Stop* getStopParameters() {
    //             return nullptr;
    //         }
    #[allow(unused_variables)]
    pub fn get_stop_parameters(&mut self) -> Ptr<SUMOVehicleParameter_Stop> {
        if self.kind == 2 {
            return Ptr::from_raw(self as *const _ as *mut Self).cast::<ROPerson_Stop>().m().get_stop_parameters();
        }
        Ptr::null()
    }

    /// C++: `SUMOTime PlanItem::getDuration() const` [const, virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_duration(&self) -> SUMOTime {
        match self.kind {
            1 => Ptr::from_raw(self as *const _ as *mut Self).cast::<ROPerson_PersonTrip>().m().get_duration(),
            2 => Ptr::from_raw(self as *const _ as *mut Self).cast::<ROPerson_Stop>().m().get_duration(),
            _ => panic!("ROPerson::PlanItem::getDuration is pure virtual"),
        }
    }

    /// C++: `const std::string & PlanItem::getStopDest() const` [const, virtual]
    // virtual const std::string& getStopDest() const {
    //             return UNDEFINED_STOPPING_PLACE;
    //         }
    #[allow(unused_variables)]
    pub fn get_stop_dest(&self) -> &str {
        match self.kind {
            1 => Ptr::from_raw(self as *const _ as *mut Self).cast::<ROPerson_PersonTrip>().m().get_stop_dest(),
            2 => Ptr::from_raw(self as *const _ as *mut Self).cast::<ROPerson_Stop>().m().get_stop_dest(),
            _ => UNDEFINED_STOPPING_PLACE,
        }
    }

}

/// A person as used by router
/// C++ class `ROPerson` (router/ROPerson.h)
#[repr(C)]
pub struct ROPerson {
    /// C++ base class `RORoutable`
    pub base: RORoutable,
    /// The plan of the person
    pub myPlan: Vec<Ptr<ROPerson_PlanItem>>,
}

impl std::ops::Deref for ROPerson {
    type Target = RORoutable;
    fn deref(&self) -> &RORoutable {
        &self.base
    }
}
impl std::ops::DerefMut for ROPerson {
    fn deref_mut(&mut self) -> &mut RORoutable {
        &mut self.base
    }
}

// C++ static members of ROPerson (port as module-level statics, see AGENTS.md):
//   static bool ROPerson::myHaveWarnedPTMissing;

fn ro_person_compute_route(r: &mut RORoutable, provider: &RORouterProvider, removeLoops: bool, errorHandler: Ptr<MsgHandler>) {
    Ptr::from_mut(r).cast::<ROPerson>().m().compute_route(provider, removeLoops, errorHandler)
}
fn ro_person_get_depart_edge(r: &RORoutable) -> Ptr<ROEdge> {
    Ptr::from_raw(r as *const _ as *mut RORoutable).cast::<ROPerson>().m().get_depart_edge()
}
fn ro_person_save_as_xml(r: &RORoutable, os: &mut OutputDevice, typeos: Ptr<OutputDevice>, asAlternatives: bool, options: &mut OptionsCont, cloneIndex: i32) {
    Ptr::from_raw(r as *const _ as *mut RORoutable).cast::<ROPerson>().m().save_as_xml(os, typeos, asAlternatives, options, cloneIndex)
}
/// C++ ROPerson overrides of the RORoutable virtual methods
static RO_PERSON_ROUTABLE_VT: crate::router::ro_routable::RORoutableVt = crate::router::ro_routable::RORoutableVt {
    compute_route: ro_person_compute_route,
    get_depart_edge: ro_person_get_depart_edge,
    save_as_xml: ro_person_save_as_xml,
};

impl ROPerson {
    /// C++: `void ROPerson::ROPerson(const SUMOVehicleParameter &, const SUMOVTypeParameter *)`
    /// Constructor
    // ROPerson::ROPerson(const SUMOVehicleParameter& pars, const SUMOVTypeParameter* type)
    //     : RORoutable(pars, type) {
    // }
    #[allow(unused_variables)]
    pub fn new(pars: &SUMOVehicleParameter, type_: Ptr<SUMOVTypeParameter>) -> Self {
        let mut base = RORoutable::new(pars, type_);
        base.vt = &RO_PERSON_ROUTABLE_VT;
        ROPerson {
            base,
            myPlan: Vec::new(),
        }
    }

    /// C++: `void ROPerson::~ROPerson() noexcept` [virtual]
    /// Destructor
    // ROPerson::~ROPerson() {
    //     for (PlanItem* const it : myPlan) {
    //         delete it;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // shortcut: plan items are freed through the base type until virtual destructors are dispatched
        for it in self.myPlan.drain(..) {
            it.m().drop_();
            drop(unsafe { Box::from_raw(it.as_raw()) });
        }
    }

    /// C++: `void ROPerson::addTrip(std::vector<PlanItem *> &, const std::string &, const ROEdge *const, const ROEdge *const, const SVCPermissions, const std::string &, const double, const std::string &, const double, const std::string &, double, const std::string &)` [static]
    // void
    // ROPerson::addTrip(std::vector<PlanItem*>& plan, const std::string& id,
    //                   const ROEdge* const from, const ROEdge* const to, const SVCPermissions modeSet,
    //                   const std::string& vTypes,
    //                   const double departPos, const std::string& stopOrigin,
    //                   const double arrivalPos, const std::string& busStop,
    //                   double walkFactor, const std::string& group) {
    //     PersonTrip* trip = new PersonTrip(from, to, modeSet, departPos, stopOrigin, arrivalPos, busStop, walkFactor, group);
    //     RONet* net = RONet::getInstance();
    //     SUMOVehicleParameter pars;
    //     pars.departProcedure = DepartDefinition::TRIGGERED;
    //     if (departPos != 0) {
    //         pars.departPosProcedure = DepartPosDefinition::GIVEN;
    //         pars.departPos = departPos;
    //         pars.parametersSet |= VEHPARS_DEPARTPOS_SET;
    //     }
    //     for (StringTokenizer st(vTypes); st.hasNext();) {
    //         pars.vtypeid = st.next();
    //         pars.parametersSet |= VEHPARS_VTYPE_SET;
    //         const SUMOVTypeParameter* type = net->getVehicleTypeSecure(pars.vtypeid);
    //         if (type == nullptr) {
    //             delete trip;
    //             throw InvalidArgument("The vehicle type '" + pars.vtypeid + "' in a trip for person '" + id + "' is not known.");
    //         }
    //         pars.id = id + "_" + toString(trip->getVehicles().size());
    //         trip->addVehicle(new ROVehicle(pars, new RORouteDef("!" + pars.id, 0, false, false), type, net));
    //         // update modeset with routing-category vClass
    //         if (type->vehicleClass == SVC_BICYCLE) {
    //             trip->updateModes(SVC_BICYCLE);
    //         } else {
    //             trip->updateModes(SVC_PASSENGER);
    //         }
    //     }
    //     if (trip->getVehicles().empty()) {
    //         if ((modeSet & SVC_PASSENGER) != 0) {
    //             pars.id = id + "_0";
    //             trip->addVehicle(new ROVehicle(pars, new RORouteDef("!" + pars.id, 0, false, false), net->getVehicleTypeSecure(DEFAULT_VTYPE_ID), net));
    //         }
    //         if ((modeSet & SVC_BICYCLE) != 0) {
    //             pars.id = id + "_b0";
    //             pars.vtypeid = DEFAULT_BIKETYPE_ID;
    //             pars.parametersSet |= VEHPARS_VTYPE_SET;
    //             trip->addVehicle(new ROVehicle(pars, new RORouteDef("!" + pars.id, 0, false, false), net->getVehicleTypeSecure(DEFAULT_BIKETYPE_ID), net));
    //         }
    //         if ((modeSet & SVC_TAXI) != 0) {
    //             // add dummy taxi for routing (never added to output)
    //             pars.id = "taxi"; // id is written as 'line'
    //             pars.vtypeid = DEFAULT_TAXITYPE_ID;
    //             trip->addVehicle(new ROVehicle(pars, new RORouteDef("!" + pars.id, 0, false, false), net->getVehicleTypeSecure(DEFAULT_TAXITYPE_ID), net));
    //         }
    //     }
    //     plan.push_back(trip);
    // }
    #[allow(unused_variables)]
    pub fn add_trip(plan: &mut Vec<Ptr<ROPerson_PlanItem>>, id: &str, from: Ptr<ROEdge>, to: Ptr<ROEdge>, modeSet: SVCPermissions, vTypes: &str, departPos: f64, stopOrigin: &str, arrivalPos: f64, busStop: &str, walkFactor: f64, group: &str) {
        let trip: Ptr<ROPerson_PersonTrip> = Ptr::from_raw(Box::into_raw(Box::new(ROPerson_PersonTrip::new_2(from, to, modeSet, departPos, stopOrigin, arrivalPos, busStop, walkFactor, group))));
        let net = RONet::get_instance();
        let mut pars = SUMOVehicleParameter::new();
        pars.departProcedure = DepartDefinition::TRIGGERED;
        if departPos != 0. {
            pars.departPosProcedure = DepartPosDefinition::GIVEN;
            pars.departPos = departPos;
            pars.parametersSet |= VEHPARS_DEPARTPOS_SET;
        }
        let mut st = utils::common::string_tokenizer::StringTokenizer::new(vTypes);
        while st.has_next() {
            pars.vtypeid = st.next().unwrap_or_else(|e| panic!("{e}"));
            pars.parametersSet |= VEHPARS_VTYPE_SET;
            let type_ = net.m().get_vehicle_type_secure(&pars.vtypeid);
            if type_.is_null() {
                drop(unsafe { Box::from_raw(trip.as_raw()) });
                panic!("{}", utils::common::util_exceptions::ProcessError::InvalidArgument(format!("The vehicle type '{}' in a trip for person '{}' is not known.", pars.vtypeid, id)));
            }
            pars.id = format!("{}_{}", id, trip.m().get_vehicles().len());
            let rd = Ptr::from_raw(Box::into_raw(Box::new(RORouteDef::new(&format!("!{}", pars.id), 0, false, false))));
            trip.m().add_vehicle(Ptr::from_raw(Box::into_raw(Box::new(ROVehicle::new(&pars, rd, type_, net, Ptr::null())))));
            // update modeset with routing-category vClass
            if type_.vehicleClass == SVC_BICYCLE {
                trip.m().update_modes(SVC_BICYCLE);
            } else {
                trip.m().update_modes(SVC_PASSENGER);
            }
        }
        if trip.m().get_vehicles().is_empty() {
            if (modeSet & SVC_PASSENGER) != 0 {
                pars.id = format!("{}_0", id);
                let rd = Ptr::from_raw(Box::into_raw(Box::new(RORouteDef::new(&format!("!{}", pars.id), 0, false, false))));
                trip.m().add_vehicle(Ptr::from_raw(Box::into_raw(Box::new(ROVehicle::new(&pars, rd, net.m().get_vehicle_type_secure(DEFAULT_VTYPE_ID), net, Ptr::null())))));
            }
            if (modeSet & SVC_BICYCLE) != 0 {
                pars.id = format!("{}_b0", id);
                pars.vtypeid = DEFAULT_BIKETYPE_ID.to_string();
                pars.parametersSet |= VEHPARS_VTYPE_SET;
                let rd = Ptr::from_raw(Box::into_raw(Box::new(RORouteDef::new(&format!("!{}", pars.id), 0, false, false))));
                trip.m().add_vehicle(Ptr::from_raw(Box::into_raw(Box::new(ROVehicle::new(&pars, rd, net.m().get_vehicle_type_secure(DEFAULT_BIKETYPE_ID), net, Ptr::null())))));
            }
            if (modeSet & SVC_TAXI) != 0 {
                // add dummy taxi for routing (never added to output)
                pars.id = "taxi".to_string(); // id is written as 'line'
                pars.vtypeid = DEFAULT_TAXITYPE_ID.to_string();
                let rd = Ptr::from_raw(Box::into_raw(Box::new(RORouteDef::new(&format!("!{}", pars.id), 0, false, false))));
                trip.m().add_vehicle(Ptr::from_raw(Box::into_raw(Box::new(ROVehicle::new(&pars, rd, net.m().get_vehicle_type_secure(DEFAULT_TAXITYPE_ID), net, Ptr::null())))));
            }
        }
        plan.push(trip.cast::<ROPerson_PlanItem>());
    }

    /// C++: `void ROPerson::addRide(std::vector<PlanItem *> &, const ROEdge *const, const ROEdge *const, const std::string &, double, const std::string &, const std::string &)` [static]
    // void
    // ROPerson::addRide(std::vector<PlanItem*>& plan, const ROEdge* const from, const ROEdge* const to, const std::string& lines,
    //                   double arrivalPos, const std::string& destStop, const std::string& group) {
    //     plan.push_back(new PersonTrip(to, destStop));
    //     plan.back()->addTripItem(new Ride(-1, from, to, lines, group, -1., arrivalPos, -1., destStop));
    // }
    #[allow(unused_variables)]
    pub fn add_ride(plan: &mut Vec<Ptr<ROPerson_PlanItem>>, from: Ptr<ROEdge>, to: Ptr<ROEdge>, lines: &str, arrivalPos: f64, destStop: &str, group: &str) {
        plan.push(Ptr::from_raw(Box::into_raw(Box::new(ROPerson_PersonTrip::new(to, destStop.to_string())))).cast::<ROPerson_PlanItem>());
        // the item just pushed is the PersonTrip
        let trip: Ptr<ROPerson_PersonTrip> = plan[plan.len() - 1].cast::<ROPerson_PersonTrip>();
        trip.m().add_trip_item(Ptr::from_raw(Box::into_raw(Box::new(ROPerson_Ride::new(-1, from, to, lines, group, -1., arrivalPos, -1., destStop, "", -1)))).cast::<ROPerson_TripItem>());
    }

    /// C++: `void ROPerson::addWalk(std::vector<PlanItem *> &, const ConstROEdgeVector &, const double, const double, const double, const double, const std::string &)` [static]
    // void
    // ROPerson::addWalk(std::vector<PlanItem*>& plan, const ConstROEdgeVector& edges, const double duration, const double speed, const double departPos, const double arrivalPos, const std::string& busStop) {
    //     if (plan.empty() || plan.back()->isStop()) {
    //         plan.push_back(new PersonTrip(edges.back(), busStop));
    //     }
    //     plan.back()->addTripItem(new Walk(-1, edges, -1., duration, speed, departPos, arrivalPos, busStop));
    // }
    #[allow(unused_variables)]
    pub fn add_walk(plan: &mut Vec<Ptr<ROPerson_PlanItem>>, edges: &ConstROEdgeVector, duration: f64, speed: f64, departPos: f64, arrivalPos: f64, busStop: &str) {
        if plan.is_empty() || plan[plan.len() - 1].is_stop() {
            plan.push(Ptr::from_raw(Box::into_raw(Box::new(ROPerson_PersonTrip::new(*edges.last().unwrap(), busStop.to_string())))).cast::<ROPerson_PlanItem>());
        }
        // only PersonTrip is not a stop, so the back item is a PersonTrip here
        let trip: Ptr<ROPerson_PersonTrip> = plan[plan.len() - 1].cast::<ROPerson_PersonTrip>();
        trip.m().add_trip_item(Ptr::from_raw(Box::into_raw(Box::new(ROPerson_Walk::new_2(-1, edges, -1., duration, speed, departPos, arrivalPos, busStop)))).cast::<ROPerson_TripItem>());
    }

    /// C++: `void ROPerson::addStop(std::vector<PlanItem *> &, const SUMOVehicleParameter::Stop &, const ROEdge *const)` [static]
    // void
    // ROPerson::addStop(std::vector<PlanItem*>& plan, const SUMOVehicleParameter::Stop& stopPar, const ROEdge* const stopEdge) {
    //     plan.push_back(new Stop(stopPar, stopEdge));
    // }
    #[allow(unused_variables)]
    pub fn add_stop(plan: &mut Vec<Ptr<ROPerson_PlanItem>>, stopPar: &SUMOVehicleParameter_Stop, stopEdge: Ptr<ROEdge>) {
        plan.push(Ptr::from_raw(Box::into_raw(Box::new(ROPerson_Stop::new(stopPar, stopEdge)))).cast::<ROPerson_PlanItem>());
    }

    /// C++: `const ROEdge * ROPerson::getDepartEdge() const` [const, virtual]
    /// Returns the first edge the person takes
    // const ROEdge* getDepartEdge() const {
    //         return myPlan.front()->getOrigin();
    //     }
    #[allow(unused_variables)]
    pub fn get_depart_edge(&self) -> Ptr<ROEdge> {
        self.myPlan[0].get_origin()
    }

    /// C++: `void ROPerson::computeRoute(const RORouterProvider &, const bool, MsgHandler *)` [virtual]
    // void
    // ROPerson::computeRoute(const RORouterProvider& provider,
    //                        const bool /* removeLoops */, MsgHandler* errorHandler) {
    //     myRoutingSuccess = true;
    //     SUMOTime time = getParameter().depart;
    //     for (PlanItem* const it : myPlan) {
    //         if (it->needsRouting()) {
    //             PersonTrip* trip = static_cast<PersonTrip*>(it);
    //             const std::vector<ROVehicle*>& vehicles = trip->getVehicles();
    //             std::vector<TripItem*> resultItems;
    //             std::vector<TripItem*> best;
    //             const ROVehicle* bestVeh = nullptr;
    //             if (vehicles.empty()) {
    //                 computeIntermodal(time, provider, trip, nullptr, best, errorHandler);
    //             } else {
    //                 double bestCost = std::numeric_limits<double>::infinity();
    //                 for (const ROVehicle* const v : vehicles) {
    //                     const bool carUsed = computeIntermodal(time, provider, trip, v, resultItems, errorHandler);
    //                     double cost = 0.;
    //                     for (const TripItem* const tripIt : resultItems) {
    //                         cost += tripIt->getCost();
    //                     }
    //                     if (cost < bestCost) {
    //                         bestCost = cost;
    //                         bestVeh = carUsed ? v : nullptr;
    //                         best.swap(resultItems);
    //                     }
    //                     for (const TripItem* const tripIt : resultItems) {
    //                         delete tripIt;
    //                     }
    //                     resultItems.clear();
    //                 }
    //             }
    //             trip->setItems(best, bestVeh);
    //
    //             // test after routing to ensuer network was initialized
    //             if (!myHaveWarnedPTMissing && (trip->getModes() & SVC_BUS) != 0) {
    //                 myHaveWarnedPTMissing = true;
    //                 if (!provider.getIntermodalRouter().getNetwork()->hasPTSchedules()) {
    //                     WRITE_WARNINGF("Person '%' is configured to use public transport but no schedules are loaded.", getID());
    //                 }
    //             }
    //         }
    //         time += it->getDuration();
    //     }
    //     if (RONet::getInstance()->getMaxTraveltime() > 0) {
    //         double costs = STEPS2TIME(time - getParameter().depart);
    //         if (costs > RONet::getInstance()->getMaxTraveltime()) {
    //             errorHandler->informf(TL("Person '%' has no valid route (traveltime % exceeds max-traveltime)."), getID(), time2string(TIME2STEPS(costs)));
    //             myRoutingSuccess = false;
    //             return;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn compute_route(&mut self, provider: &RORouterProvider, removeLoops: bool, errorHandler: Ptr<MsgHandler>) {
        self.myRoutingSuccess = true;
        let mut time: SUMOTime = self.get_parameter().depart;
        for i in 0..self.myPlan.len() {
            let it = self.myPlan[i];
            if it.needs_routing() {
                let trip: Ptr<ROPerson_PersonTrip> = it.cast::<ROPerson_PersonTrip>();
                let vehicles: Vec<Ptr<ROVehicle>> = trip.m().get_vehicles().clone();
                let mut resultItems: Vec<Ptr<ROPerson_TripItem>> = Vec::new();
                let mut best: Vec<Ptr<ROPerson_TripItem>> = Vec::new();
                let mut bestVeh: Ptr<ROVehicle> = Ptr::null();
                if vehicles.is_empty() {
                    self.compute_intermodal(time, provider, trip, Ptr::null(), &mut best, errorHandler);
                } else {
                    let mut bestCost = f64::INFINITY;
                    for v in vehicles.iter() {
                        let carUsed = self.compute_intermodal(time, provider, trip, *v, &mut resultItems, errorHandler);
                        let mut cost = 0.;
                        for tripIt in resultItems.iter() {
                            cost += tripIt.get_cost();
                        }
                        if cost < bestCost {
                            bestCost = cost;
                            bestVeh = if carUsed { *v } else { Ptr::null() };
                            std::mem::swap(&mut best, &mut resultItems);
                        }
                        // shortcut: items are freed through the base type until virtual destructors are dispatched
                        for tripIt in resultItems.drain(..) {
                            drop(unsafe { Box::from_raw(tripIt.as_raw()) });
                        }
                    }
                }
                trip.m().set_items(&mut best, bestVeh);

                // test after routing to ensure network was initialized
                if !MY_HAVE_WARNED_PT_MISSING.load(std::sync::atomic::Ordering::Relaxed) && (trip.get_modes() & SVC_BUS) != 0 {
                    MY_HAVE_WARNED_PT_MISSING.store(true, std::sync::atomic::Ordering::Relaxed);
                    if !provider.get_intermodal_router().get_network().has_pt_schedules() {
                        write_warningf!("Person '%' is configured to use public transport but no schedules are loaded.", self.get_id());
                    }
                }
            }
            time += it.get_duration();
        }
        if RONet::get_instance().get_max_traveltime() > 0. {
            let costs = utils::common::time::steps2time(time - self.get_parameter().depart);
            if costs > RONet::get_instance().get_max_traveltime() {
                let args: [&dyn utils::common::to_string::ToStringPrec; 2] = [&self.get_id(), &utils::common::time::time2string(utils::common::time::time2steps(costs))];
                errorHandler.m().informf("Person '%' has no valid route (traveltime % exceeds max-traveltime).", &args);
                self.myRoutingSuccess = false;
                return;
            }
        }
    }

    /// C++: `void ROPerson::saveAsXML(OutputDevice &, OutputDevice *const, bool, OptionsCont &, int) const` [const, virtual]
    /// Saves the complete person description.
    // void
    // ROPerson::saveAsXML(OutputDevice& os, OutputDevice* const typeos, bool asAlternatives, OptionsCont& options, int cloneIndex) const {
    //     // write the person's vehicles
    //     const bool writeTrip = options.exists("write-trips") && options.getBool("write-trips");
    //     const bool writeFlow = options.exists("keep-flows") && options.getBool("keep-flows") && isPartOfFlow();
    //     if (!writeTrip) {
    //         for (const PlanItem* const it : myPlan) {
    //             it->saveVehicles(os, typeos, asAlternatives, options);
    //         }
    //     }
    //
    //     if (typeos != nullptr && getType() != nullptr && !getType()->saved) {
    //         getType()->write(*typeos);
    //         getType()->saved = true;
    //     }
    //     if (getType() != nullptr && !getType()->saved) {
    //         getType()->write(os);
    //         getType()->saved = asAlternatives;
    //     }
    //     const SumoXMLTag tag = writeFlow ? SUMO_TAG_PERSONFLOW : SUMO_TAG_PERSON;
    //     // write the person
    //     if (cloneIndex == 0) {
    //         getParameter().write(os, options, tag);
    //     } else {
    //         SUMOVehicleParameter p = getParameter();
    //         // @note id collisions may occur if scale-suffic occurs in other vehicle ids
    //         p.id += options.getString("scale-suffix") + toString(cloneIndex);
    //         p.write(os, options, tag);
    //     }
    //
    //     for (const PlanItem* const it : myPlan) {
    //         it->saveAsXML(os, asAlternatives, writeTrip, options);
    //     }
    //
    //     // write params
    //     getParameter().writeParams(os);
    //     os.closeTag();
    // }
    #[allow(unused_variables)]
    pub fn save_as_xml(&self, os: &mut OutputDevice, typeos: Ptr<OutputDevice>, asAlternatives: bool, options: &mut OptionsCont, cloneIndex: i32) {
        // write the person's vehicles
        let writeTrip = options.exists("write-trips") && options.get_bool("write-trips").unwrap_or_else(|e| panic!("{e}"));
        let writeFlow = options.exists("keep-flows") && options.get_bool("keep-flows").unwrap_or_else(|e| panic!("{e}")) && self.is_part_of_flow();
        if !writeTrip {
            for it in self.myPlan.iter() {
                it.save_vehicles(os, typeos, asAlternatives, options);
            }
        }

        if !typeos.is_null() && !self.get_type().is_null() && !self.get_type().saved {
            self.get_type().write(typeos.m());
            self.get_type().m().saved = true;
        }
        if !self.get_type().is_null() && !self.get_type().saved {
            self.get_type().write(os);
            self.get_type().m().saved = asAlternatives;
        }
        let tag = if writeFlow { SUMO_TAG_PERSONFLOW } else { SUMO_TAG_PERSON };
        // write the person
        if cloneIndex == 0 {
            self.get_parameter().write(os, options, tag, "");
        } else {
            let mut p = self.get_parameter().clone();
            let suffix = options.get_string("scale-suffix").unwrap_or_else(|e| panic!("{e}"));
            p.id += &format!("{}{}", suffix, cloneIndex);
            p.write(os, options, tag, "");
        }

        for it in self.myPlan.iter() {
            it.save_as_xml(os, asAlternatives, writeTrip, options);
        }

        // write params
        self.get_parameter().write_params(os);
        os.close_tag("");
    }

    /// C++: `std::vector<PlanItem *> & ROPerson::getPlan()`
    // std::vector<PlanItem*>& getPlan() {
    //         return myPlan;
    //     }
    #[allow(unused_variables)]
    pub fn get_plan(&mut self) -> &mut Vec<Ptr<ROPerson_PlanItem>> {
        &mut self.myPlan
    }

    /// C++: `bool ROPerson::computeIntermodal(SUMOTime, const RORouterProvider &, const PersonTrip *const, const ROVehicle *const, std::vector<TripItem *> &, MsgHandler *const)` [private]
    // bool
    // ROPerson::computeIntermodal(SUMOTime time, const RORouterProvider& provider,
    //                             const PersonTrip* const trip, const ROVehicle* const veh,
    //                             std::vector<TripItem*>& resultItems, MsgHandler* const errorHandler) {
    //     const double speed = getMaxSpeed() * trip->getWalkFactor();
    //     std::vector<ROIntermodalRouter::TripItem> result;
    //     provider.getIntermodalRouter().compute(trip->getOrigin(), trip->getDestination(),
    //                                            trip->getDepartPos(), trip->getStopOrigin(),
    //                                            trip->getArrivalPos(), trip->getStopDest(),
    //                                            speed, veh, *getType(), trip->getModes(), time, result);
    //     bool carUsed = false;
    //     SUMOTime start = time;
    //     int index = 0;
    //     const int lastIndex = (int)result.size() - 1;
    //     for (const ROIntermodalRouter::TripItem& item : result) {
    //         if (!item.edges.empty()) {
    //             if (item.line == "") {
    //                 double depPos = trip->getDepartPos(false);
    //                 double arrPos = trip->getArrivalPos(false);
    //                 if (trip->getOrigin()->isTazConnector()) {
    //                     // walk the whole length of the first edge
    //                     const ROEdge* first = item.edges.front();
    //                     if (std::find(first->getPredecessors().begin(), first->getPredecessors().end(), trip->getOrigin()) != first->getPredecessors().end()) {
    //                         depPos = 0;
    //                     } else {
    //                         depPos = first->getLength();
    //                     }
    //                 }
    //                 if (trip->getDestination()->isTazConnector()) {
    //                     // walk the whole length of the last edge
    //                     const ROEdge* last = item.edges.back();
    //                     if (std::find(last->getSuccessors().begin(), last->getSuccessors().end(), trip->getDestination()) != last->getSuccessors().end()) {
    //                         arrPos = last->getLength();
    //                     } else {
    //                         arrPos = 0;
    //                     }
    //                 }
    //                 if (&item == &result.back() && trip->getStopDest() == "") {
    //                     resultItems.push_back(new Walk(start, item.edges, item.cost, item.exitTimes, depPos, arrPos));
    //                 } else {
    //                     resultItems.push_back(new Walk(start, item.edges, item.cost, item.exitTimes, depPos, arrPos, item.destStop));
    //                 }
    //             } else if (veh != nullptr && item.line == veh->getID()) {
    //                 double cost = item.cost;
    //                 if (veh->getVClass() != SVC_TAXI) {
    //                     std::shared_ptr<RORoute> route = std::make_shared<RORoute>(veh->getID() + "_RouteDef", item.edges);
    //                     route->setProbability(1);
    //                     veh->getRouteDefinition()->addLoadedAlternative(route);
    //                     carUsed = true;
    //                 } else if (resultItems.empty()) {
    //                     // if this is the first plan item the initial taxi waiting time wasn't added yet
    //                     const double taxiWait = STEPS2TIME(string2time(OptionsCont::getOptions().getString("persontrip.taxi.waiting-time")));
    //                     cost += taxiWait;
    //                 }
    //                 double arrPos = index == lastIndex ? trip->getArrivalPos(false) : item.arrivalPos;
    //                 resultItems.push_back(new Ride(start, item.edges.front(), item.edges.back(), veh->getID(), trip->getGroup(), cost, arrPos, item.length, item.destStop));
    //             } else {
    //                 // write origin for first element of the plan
    //                 const ROEdge* origin = trip == myPlan.front() && resultItems.empty() ? trip->getOrigin() : nullptr;
    //                 const std::string line = OptionsCont::getOptions().getBool("persontrip.ride-public-line") ? item.line : LINE_ANY;
    //                 resultItems.push_back(new Ride(start, origin, nullptr, line, trip->getGroup(), item.cost, item.arrivalPos, item.length, item.destStop, item.intended, TIME2STEPS(item.depart)));
    //             }
    //         }
    //         start += TIME2STEPS(item.cost);
    //         index++;
    //     }
    //     if (result.empty()) {
    //         errorHandler->informf(TL("No route for trip in person '%'."), getID());
    //         myRoutingSuccess = false;
    //     }
    //     return carUsed;
    // }
    #[allow(unused_variables)]
    pub fn compute_intermodal(&mut self, time: SUMOTime, provider: &RORouterProvider, trip: Ptr<ROPerson_PersonTrip>, veh: Ptr<ROVehicle>, resultItems: &mut Vec<Ptr<ROPerson_TripItem>>, errorHandler: Ptr<MsgHandler>) -> bool {
        let speed = self.get_max_speed() * trip.get_walk_factor();
        let mut result: Vec<IntermodalRouter_TripItem> = Vec::new();
        provider.get_intermodal_router().compute(trip.get_origin(), trip.get_destination(), trip.get_depart_pos(true), trip.get_stop_origin(), trip.get_arrival_pos(true), trip.get_stop_dest(), speed, veh, &*self.get_type(), trip.get_modes(), time, &mut result, 0.);
        let mut carUsed = false;
        let mut start: SUMOTime = time;
        let lastIndex: i32 = result.len() as i32 - 1;
        for (index, item) in result.iter().enumerate() {
            if !item.edges.is_empty() {
                // shortcut: IntermodalRouter_TripItem.edges is Vec<Ptr<Unmapped>>, cast back to ROEdge
                let edges: ConstROEdgeVector = item.edges.iter().map(|e| e.cast::<ROEdge>()).collect();
                if item.line == "" {
                    let mut depPos = trip.get_depart_pos(false);
                    let mut arrPos = trip.get_arrival_pos(false);
                    if trip.get_origin().is_taz_connector() {
                        // walk the whole length of the first edge
                        let first = edges[0];
                        if first.get_predecessors().contains(&trip.get_origin()) {
                            depPos = 0.;
                        } else {
                            depPos = first.get_length();
                        }
                    }
                    if trip.get_destination().is_taz_connector() {
                        // walk the whole length of the last edge
                        let last = edges[edges.len() - 1];
                        if last.get_successors(SVC_IGNORING).contains(&trip.get_destination()) {
                            arrPos = last.get_length();
                        } else {
                            arrPos = 0.;
                        }
                    }
                    if index == result.len() - 1 && trip.get_stop_dest() == "" {
                        resultItems.push(Ptr::from_raw(Box::into_raw(Box::new(ROPerson_Walk::new(start, &edges, item.cost, &item.exitTimes, depPos, arrPos, "")))).cast::<ROPerson_TripItem>());
                    } else {
                        resultItems.push(Ptr::from_raw(Box::into_raw(Box::new(ROPerson_Walk::new(start, &edges, item.cost, &item.exitTimes, depPos, arrPos, &item.destStop)))).cast::<ROPerson_TripItem>());
                    }
                } else if !veh.is_null() && item.line == veh.get_id() {
                    let mut cost = item.cost;
                    if veh.get_v_class() != SVC_TAXI {
                        let mut route = RORoute::new_2(&format!("{}_RouteDef", veh.get_id()), &edges);
                        route.set_probability(1.);
                        veh.get_route_definition().m().add_loaded_alternative(Rc::new(route));
                        carUsed = true;
                    } else if resultItems.is_empty() {
                        // if this is the first plan item the initial taxi waiting time wasn't added yet
                        let taxiWait = utils::common::time::steps2time(utils::common::time::string2time(&utils::options::options_cont::get_string("persontrip.taxi.waiting-time")).unwrap_or_else(|e| panic!("{e}")));
                        cost += taxiWait;
                    }
                    let arrPos = if index as i32 == lastIndex { trip.get_arrival_pos(false) } else { item.arrivalPos };
                    resultItems.push(Ptr::from_raw(Box::into_raw(Box::new(ROPerson_Ride::new(start, edges[0], edges[edges.len() - 1], veh.get_id(), trip.get_group(), cost, arrPos, item.length, &item.destStop, "", -1)))).cast::<ROPerson_TripItem>());
                } else {
                    // write origin for first element of the plan
                    let origin = if trip.cast::<ROPerson_PlanItem>() == self.myPlan[0] && resultItems.is_empty() { trip.get_origin() } else { Ptr::null() };
                    let line: String = if utils::options::options_cont::get_bool("persontrip.ride-public-line") { item.line.clone() } else { LINE_ANY.to_string() };
                    resultItems.push(Ptr::from_raw(Box::into_raw(Box::new(ROPerson_Ride::new(start, origin, Ptr::null(), &line, trip.get_group(), item.cost, item.arrivalPos, item.length, &item.destStop, &item.intended, utils::common::time::time2steps(item.depart))))).cast::<ROPerson_TripItem>());
                }
            }
            start += utils::common::time::time2steps(item.cost);
        }
        if result.is_empty() {
            let args: [&dyn utils::common::to_string::ToStringPrec; 1] = [&self.get_id()];
            errorHandler.m().informf("No route for trip in person '%'.", &args);
            self.myRoutingSuccess = false;
        }
        carUsed
    }

    /// C++: `void ROPerson::ROPerson(const ROPerson &)` [private]
    /// Invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(src: &ROPerson) -> Self {
        panic!("ROPerson::ROPerson(const ROPerson &) is private and undefined in C++ (invalidated copy)")
    }

    /// C++: `ROPerson & ROPerson::operator=(const ROPerson &)` [private]
    /// Invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, src: &ROPerson) -> &mut ROPerson {
        panic!("ROPerson::operator= is private and undefined in C++ (invalidated copy)")
    }

}

/// A ride is part of a trip, e.g., go from here to here by car or bus
/// C++ class `ROPerson::Ride` (router/ROPerson.h)
#[repr(C)]
pub struct ROPerson_Ride {
    /// C++ base class `TripItem`
    pub base: ROPerson_TripItem,
    pub from: Ptr<ROEdge>,
    pub to: Ptr<ROEdge>,
    pub lines: String,
    pub group: String,
    pub destStop: String,
    pub intended: String,
    pub depart: SUMOTime,
    pub arrPos: f64,
    pub length: f64,
}

impl std::ops::Deref for ROPerson_Ride {
    type Target = ROPerson_TripItem;
    fn deref(&self) -> &ROPerson_TripItem {
        &self.base
    }
}
impl std::ops::DerefMut for ROPerson_Ride {
    fn deref_mut(&mut self) -> &mut ROPerson_TripItem {
        &mut self.base
    }
}

impl ROPerson_Ride {
    /// C++: `void Ride::Ride(const SUMOTime, const ROEdge *const, const ROEdge *const, const std::string &, const std::string &, const double, const double, const double, const std::string &, const std::string &, const SUMOTime)`
    // Ride(const SUMOTime start, const ROEdge* const _from, const ROEdge* const _to,
    //              const std::string& _lines, const std::string& _group, const double cost,
    //              const double arrivalPos, const double _length,
    //              const std::string& _destStop = "", const std::string& _intended = "", const SUMOTime _depart = -1) :
    //             TripItem(start, cost),
    //             from(_from), to(_to),
    //             lines(_lines),
    //             group(_group),
    //             destStop(_destStop),
    //             intended(_intended),
    //             depart(_depart),
    //             arrPos(arrivalPos),
    //             length(_length) {
    //         }
    #[allow(unused_variables)]
    pub fn new(start: SUMOTime, _from: Ptr<ROEdge>, _to: Ptr<ROEdge>, _lines: &str, _group: &str, cost: f64, arrivalPos: f64, _length: f64, _destStop: &str, _intended: &str, _depart: SUMOTime) -> Self {
        ROPerson_Ride {
            base: ROPerson_TripItem { kind: 1, ..ROPerson_TripItem { kind: 2, ..ROPerson_TripItem { kind: 2, ..ROPerson_TripItem::new(start, cost) } } },
            from: _from,
            to: _to,
            lines: _lines.to_string(),
            group: _group.to_string(),
            destStop: _destStop.to_string(),
            intended: _intended.to_string(),
            depart: _depart,
            arrPos: arrivalPos,
            length: _length,
        }
    }

    /// C++: `TripItem * Ride::clone() const` [const, virtual]
    // TripItem* clone() const {
    //             return new Ride(myStart, from, to, lines, group, myCost, arrPos, length, destStop, intended, depart);
    //         }
    #[allow(unused_variables)]
    pub fn clone(&self) -> Ptr<ROPerson_TripItem> {
        Ptr::from_raw(Box::into_raw(Box::new(ROPerson_Ride::new(self.base.myStart, self.from, self.to, &self.lines, &self.group, self.base.myCost, self.arrPos, self.length, &self.destStop, &self.intended, self.depart)))).cast::<ROPerson_TripItem>()
    }

    /// C++: `const ROEdge * Ride::getOrigin() const` [const, virtual]
    // inline const ROEdge* getOrigin() const {
    //             return from;
    //         }
    #[allow(unused_variables)]
    pub fn get_origin(&self) -> Ptr<ROEdge> {
        self.from
    }

    /// C++: `const ROEdge * Ride::getDestination() const` [const, virtual]
    // inline const ROEdge* getDestination() const {
    //             return to;
    //         }
    #[allow(unused_variables)]
    pub fn get_destination(&self) -> Ptr<ROEdge> {
        self.to
    }

    /// C++: `double Ride::getDestinationPos() const` [const, virtual]
    // inline double getDestinationPos() const {
    //             return arrPos == std::numeric_limits<double>::infinity() ? -NUMERICAL_EPS : arrPos;
    //         }
    #[allow(unused_variables)]
    pub fn get_destination_pos(&self) -> f64 {
        if self.arrPos == f64::INFINITY { -utils::geom::NUMERICAL_EPS } else { self.arrPos }
    }

    /// C++: `void Ride::saveAsXML(OutputDevice &, const bool, OptionsCont &) const` [const, virtual]
    // void
    // ROPerson::Ride::saveAsXML(OutputDevice& os, const bool extended, OptionsCont& options) const {
    //     os.openTag(SUMO_TAG_RIDE);
    //     std::string comment = "";
    //     if ((extended || options.getBool("write-costs")) && myCost >= 0.) {
    //         os.writeAttr(SUMO_ATTR_COST, myCost);
    //     }
    //     if (from != nullptr) {
    //         os.writeAttr(SUMO_ATTR_FROM, from->getID());
    //     }
    //     if (to != nullptr) {
    //         os.writeAttr(SUMO_ATTR_TO, to->getID());
    //     }
    //     if (destStop != "") {
    //         const std::string element = RONet::getInstance()->getStoppingPlaceElement(destStop);
    //         os.writeAttr(element, destStop);
    //         const std::string name = RONet::getInstance()->getStoppingPlaceName(destStop);
    //         if (name != "") {
    //             comment =  " <!-- " + name + " -->";
    //         }
    //     } else if (arrPos != 0 && arrPos != std::numeric_limits<double>::infinity()) {
    //         os.writeAttr(SUMO_ATTR_ARRIVALPOS, arrPos);
    //     }
    //     if (lines != LINE_ANY) {
    //         // no need to write the default
    //         os.writeAttr(SUMO_ATTR_LINES, lines);
    //     }
    //     if (group != "") {
    //         os.writeAttr(SUMO_ATTR_GROUP, group);
    //     }
    //     if (intended != "" && intended != lines) {
    //         os.writeAttr(SUMO_ATTR_INTENDED, intended);
    //     }
    //     if (depart >= 0) {
    //         os.writeAttr(SUMO_ATTR_DEPART, time2string(depart));
    //     }
    //     if (options.getBool("exit-times")) {
    //         os.writeAttr("started", time2string(getStart()));
    //         os.writeAttr("ended", time2string(getStart() + getDuration()));
    //     }
    //     if (options.getBool("route-length") && length != -1) {
    //         os.writeAttr("routeLength", length);
    //     }
    //     os.closeTag(comment);
    // }
    #[allow(unused_variables)]
    pub fn save_as_xml(&self, os: &mut OutputDevice, extended: bool, options: &mut OptionsCont) {
        os.open_tag_id(SUMO_TAG_RIDE);
        let mut comment = String::new();
        if (extended || options.get_bool("write-costs").unwrap_or_else(|e| panic!("{e}"))) && self.base.myCost >= 0. {
            os.write_attr(SUMO_ATTR_COST, &self.base.myCost);
        }
        if !self.from.is_null() {
            os.write_attr(SUMO_ATTR_FROM, self.from.get_id());
        }
        if !self.to.is_null() {
            os.write_attr(SUMO_ATTR_TO, self.to.get_id());
        }
        if self.destStop != "" {
            let element = RONet::get_instance().get_stopping_place_element(&self.destStop);
            os.write_attr(element.as_str(), &self.destStop);
            let name = RONet::get_instance().get_stopping_place_name(&self.destStop);
            if name != "" {
                comment = format!(" <!-- {} -->", name);
            }
        } else if self.arrPos != 0. && self.arrPos != f64::INFINITY {
            os.write_attr(SUMO_ATTR_ARRIVALPOS, &self.arrPos);
        }
        if self.lines != LINE_ANY {
            // no need to write the default
            os.write_attr(SUMO_ATTR_LINES, &self.lines);
        }
        if self.group != "" {
            os.write_attr(SUMO_ATTR_GROUP, &self.group);
        }
        if self.intended != "" && self.intended != self.lines {
            os.write_attr(SUMO_ATTR_INTENDED, &self.intended);
        }
        if self.depart >= 0 {
            os.write_attr(SUMO_ATTR_DEPART, &utils::common::time::time2string(self.depart));
        }
        if options.get_bool("exit-times").unwrap_or_else(|e| panic!("{e}")) {
            os.write_attr("started", &utils::common::time::time2string(self.get_start()));
            os.write_attr("ended", &utils::common::time::time2string(self.get_start() + self.get_duration()));
        }
        if options.get_bool("route-length").unwrap_or_else(|e| panic!("{e}")) && self.length != -1. {
            os.write_attr("routeLength", &self.length);
        }
        os.close_tag(&comment);
    }

    /// C++: `Ride & Ride::operator=(const Ride &)` [private]
    /// Invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, src: &ROPerson_Ride) -> &mut ROPerson_Ride {
        panic!("ROPerson::Ride::operator= is private and undefined in C++ (invalidated copy)")
    }

}

/// A planItem can be a Stop
/// C++ class `ROPerson::Stop` (router/ROPerson.h)
#[repr(C)]
pub struct ROPerson_Stop {
    /// C++ base class `PlanItem`
    pub base: ROPerson_PlanItem,
    pub stopDesc: SUMOVehicleParameter_Stop,
    pub edge: Ptr<ROEdge>,
}

impl std::ops::Deref for ROPerson_Stop {
    type Target = ROPerson_PlanItem;
    fn deref(&self) -> &ROPerson_PlanItem {
        &self.base
    }
}
impl std::ops::DerefMut for ROPerson_Stop {
    fn deref_mut(&mut self) -> &mut ROPerson_PlanItem {
        &mut self.base
    }
}

impl ROPerson_Stop {
    /// C++: `void Stop::Stop(const SUMOVehicleParameter::Stop &, const ROEdge *const)`
    // Stop(const SUMOVehicleParameter::Stop& stop, const ROEdge* const stopEdge)
    //             : stopDesc(stop), edge(stopEdge) {}
    #[allow(unused_variables)]
    pub fn new(stop: &SUMOVehicleParameter_Stop, stopEdge: Ptr<ROEdge>) -> Self {
        ROPerson_Stop {
            base: ROPerson_PlanItem { base: Parameterised::default(), kind: 2 },
            stopDesc: stop.clone(),
            edge: stopEdge,
        }
    }

    /// C++: `PlanItem * Stop::clone() const` [const, virtual]
    // PlanItem* clone() const {
    //             return new Stop(stopDesc, edge);
    //         }
    #[allow(unused_variables)]
    pub fn clone(&self) -> Ptr<ROPerson_PlanItem> {
        Ptr::from_raw(Box::into_raw(Box::new(ROPerson_Stop::new(&self.stopDesc, self.edge)))).cast::<ROPerson_PlanItem>()
    }

    /// C++: `const ROEdge * Stop::getOrigin() const` [const, virtual]
    // const ROEdge* getOrigin() const {
    //             return edge;
    //         }
    #[allow(unused_variables)]
    pub fn get_origin(&self) -> Ptr<ROEdge> {
        self.edge
    }

    /// C++: `const ROEdge * Stop::getDestination() const` [const, virtual]
    // const ROEdge* getDestination() const {
    //             return edge;
    //         }
    #[allow(unused_variables)]
    pub fn get_destination(&self) -> Ptr<ROEdge> {
        self.edge
    }

    /// C++: `double Stop::getDestinationPos() const` [const, virtual]
    // double getDestinationPos() const {
    //             return (stopDesc.startPos + stopDesc.endPos) / 2;
    //         }
    #[allow(unused_variables)]
    pub fn get_destination_pos(&self) -> f64 {
        (self.stopDesc.startPos + self.stopDesc.endPos) / 2.
    }

    /// C++: `void Stop::saveAsXML(OutputDevice &, const bool, const bool, OptionsCont &) const` [const, virtual]
    // void
    // ROPerson::Stop::saveAsXML(OutputDevice& os, const bool /*extended*/, const bool /*asTrip*/, OptionsCont& /*options*/) const {
    //     stopDesc.write(os, false);
    //     std::string comment = "";
    //     for (std::string sID : stopDesc.getStoppingPlaceIDs()) {
    //         const std::string name = RONet::getInstance()->getStoppingPlaceName(sID);
    //         if (name != "") {
    //             comment += name + " ";
    //         }
    //     }
    //     if (comment != "") {
    //         comment =  " <!-- " + comment + " -->";
    //     }
    //     stopDesc.writeParams(os);
    //     os.closeTag(comment);
    // }
    #[allow(unused_variables)]
    pub fn save_as_xml(&self, os: &mut OutputDevice, extended: bool, asTrip: bool, options: &mut OptionsCont) {
        self.stopDesc.write(os, false, true);
        let mut comment = String::new();
        for sID in self.stopDesc.get_stopping_place_i_ds() {
            let name = RONet::get_instance().get_stopping_place_name(&sID);
            if name != "" {
                comment += &format!("{} ", name);
            }
        }
        if comment != "" {
            comment = format!(" <!-- {} -->", comment);
        }
        self.stopDesc.write_params(os);
        os.close_tag(&comment);
    }

    /// C++: `bool Stop::isStop() const` [const, virtual]
    // bool isStop() const {
    //             return true;
    //         }
    #[allow(unused_variables)]
    pub fn is_stop(&self) -> bool {
        true
    }

    /// C++: `SUMOVehicleParameter::Stop * Stop::getStopParameters()` [virtual]
    // virtual SUMOVehicleParameter::Stop* getStopParameters() {
    //             return &stopDesc;
    //         }
    #[allow(unused_variables)]
    pub fn get_stop_parameters(&mut self) -> Ptr<SUMOVehicleParameter_Stop> {
        Ptr::from_mut(&mut self.stopDesc)
    }

    /// C++: `SUMOTime Stop::getDuration() const` [const, virtual]
    // SUMOTime getDuration() const {
    //             return stopDesc.duration;
    //         }
    #[allow(unused_variables)]
    pub fn get_duration(&self) -> SUMOTime {
        self.stopDesc.duration
    }

    /// C++: `const std::string & Stop::getStopDest() const` [const, virtual]
    // inline const std::string& getStopDest() const {
    //             return stopDesc.busstop;
    //         }
    #[allow(unused_variables)]
    pub fn get_stop_dest(&self) -> &str {
        self.stopDesc.busstop.as_str()
    }

    /// C++: `Stop & Stop::operator=(const Stop &)` [private]
    /// Invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, src: &SUMOVehicleParameter_Stop) -> &mut SUMOVehicleParameter_Stop {
        panic!("ROPerson::Stop::operator= is private and undefined in C++ (invalidated copy)")
    }

}

/// A TripItem is part of a trip, e.g., go from here to here by car
/// C++ class `ROPerson::TripItem` (router/ROPerson.h)
#[repr(C)]
pub struct ROPerson_TripItem {
    pub myStart: SUMOTime,
    pub myCost: f64,
    /// virtual dispatch tag: 1 = Ride, 2 = Walk (C++ vtable)
    pub kind: u8,
}

impl ROPerson_TripItem {
    /// C++: `void TripItem::TripItem(const SUMOTime, const double)`
    // TripItem(const SUMOTime start, const double cost)
    //             : myStart(start), myCost(cost) {}
    #[allow(unused_variables)]
    pub fn new(start: SUMOTime, cost: f64) -> Self {
        ROPerson_TripItem { myStart: start, myCost: cost, kind: 0 }
    }

    /// C++: `void TripItem::~TripItem() noexcept` [virtual]
    /// Destructor
    // virtual ~TripItem() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
    }

    /// C++: `TripItem * TripItem::clone() const` [const, virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn clone(&self) -> Ptr<ROPerson_TripItem> {
        match self.kind {
            1 => Ptr::from_raw(self as *const _ as *mut Self).cast::<ROPerson_Ride>().m().clone(),
            2 => Ptr::from_raw(self as *const _ as *mut Self).cast::<ROPerson_Walk>().m().clone(),
            _ => panic!("ROPerson::TripItem::clone is pure virtual"),
        }
    }

    /// C++: `const ROEdge * TripItem::getOrigin() const` [const, virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_origin(&self) -> Ptr<ROEdge> {
        match self.kind {
            1 => Ptr::from_raw(self as *const _ as *mut Self).cast::<ROPerson_Ride>().m().get_origin(),
            2 => Ptr::from_raw(self as *const _ as *mut Self).cast::<ROPerson_Walk>().m().get_origin(),
            _ => panic!("ROPerson::TripItem::getOrigin is pure virtual"),
        }
    }

    /// C++: `const ROEdge * TripItem::getDestination() const` [const, virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_destination(&self) -> Ptr<ROEdge> {
        match self.kind {
            1 => Ptr::from_raw(self as *const _ as *mut Self).cast::<ROPerson_Ride>().m().get_destination(),
            2 => Ptr::from_raw(self as *const _ as *mut Self).cast::<ROPerson_Walk>().m().get_destination(),
            _ => panic!("ROPerson::TripItem::getDestination is pure virtual"),
        }
    }

    /// C++: `double TripItem::getDestinationPos() const` [const, virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_destination_pos(&self) -> f64 {
        match self.kind {
            1 => Ptr::from_raw(self as *const _ as *mut Self).cast::<ROPerson_Ride>().m().get_destination_pos(),
            2 => Ptr::from_raw(self as *const _ as *mut Self).cast::<ROPerson_Walk>().m().get_destination_pos(),
            _ => panic!("ROPerson::TripItem::getDestinationPos is pure virtual"),
        }
    }

    /// C++: `void TripItem::saveAsXML(OutputDevice &, const bool, OptionsCont &) const` [const, virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn save_as_xml(&self, os: &mut OutputDevice, extended: bool, options: &mut OptionsCont) {
        match self.kind {
            1 => Ptr::from_raw(self as *const _ as *mut Self).cast::<ROPerson_Ride>().m().save_as_xml(os, extended, options),
            2 => Ptr::from_raw(self as *const _ as *mut Self).cast::<ROPerson_Walk>().m().save_as_xml(os, extended, options),
            _ => panic!("ROPerson::TripItem::saveAsXML is pure virtual"),
        }
    }

    /// C++: `SUMOTime TripItem::getStart() const` [const]
    // inline SUMOTime getStart() const {
    //             return myStart;
    //         }
    #[allow(unused_variables)]
    pub fn get_start(&self) -> SUMOTime {
        self.myStart
    }

    /// C++: `SUMOTime TripItem::getDuration() const` [const]
    // inline SUMOTime getDuration() const {
    //             return TIME2STEPS(myCost);
    //         }
    #[allow(unused_variables)]
    pub fn get_duration(&self) -> SUMOTime {
        utils::common::time::time2steps(self.myCost)
    }

    /// C++: `double TripItem::getCost() const` [const]
    // inline double getCost() const {
    //             return myCost;
    //         }
    #[allow(unused_variables)]
    pub fn get_cost(&self) -> f64 {
        self.myCost
    }

}

/// A walk is part of a trip, e.g., go from here to here by foot
/// C++ class `ROPerson::Walk` (router/ROPerson.h)
#[repr(C)]
pub struct ROPerson_Walk {
    /// C++ base class `TripItem`
    pub base: ROPerson_TripItem,
    pub edges: ConstROEdgeVector,
    pub exitTimes: Vec<f64>,
    pub dur: f64,
    pub v: f64,
    pub dep: f64,
    pub arr: f64,
    pub destStop: String,
}

impl std::ops::Deref for ROPerson_Walk {
    type Target = ROPerson_TripItem;
    fn deref(&self) -> &ROPerson_TripItem {
        &self.base
    }
}
impl std::ops::DerefMut for ROPerson_Walk {
    fn deref_mut(&mut self) -> &mut ROPerson_TripItem {
        &mut self.base
    }
}

impl ROPerson_Walk {
    /// C++: `void Walk::Walk(const SUMOTime, const ConstROEdgeVector &, const double, const std::vector<double> &, double, double, const std::string &)`
    // Walk(const SUMOTime start, const ConstROEdgeVector& _edges, const double cost,
    //              const std::vector<double>& _exitTimes,
    //              double departPos = std::numeric_limits<double>::infinity(),
    //              double arrivalPos = std::numeric_limits<double>::infinity(),
    //              const std::string& _destStop = "")
    //             : TripItem(start, cost), edges(_edges), exitTimes(_exitTimes), dur(-1), v(-1), dep(departPos), arr(arrivalPos), destStop(_destStop) {}
    #[allow(unused_variables)]
    pub fn new(start: SUMOTime, _edges: &ConstROEdgeVector, cost: f64, _exitTimes: &Vec<f64>, departPos: f64, arrivalPos: f64, _destStop: &str) -> Self {
        ROPerson_Walk {
            base: ROPerson_TripItem::new(start, cost),
            edges: _edges.clone(),
            exitTimes: _exitTimes.clone(),
            dur: -1.,
            v: -1.,
            dep: departPos,
            arr: arrivalPos,
            destStop: _destStop.to_string(),
        }
    }

    /// C++: `void Walk::Walk(const SUMOTime, const ConstROEdgeVector &, const double, const double, const double, const double, const double, const std::string &)`
    // Walk(const SUMOTime start, const ConstROEdgeVector& edges, const double cost, const double duration, const double speed,
    //              const double departPos, const double arrivalPos, const std::string& _destStop)
    //             : TripItem(start, cost), edges(edges), dur(duration), v(speed), dep(departPos), arr(arrivalPos), destStop(_destStop) {}
    #[allow(unused_variables)]
    pub fn new_2(start: SUMOTime, edges: &ConstROEdgeVector, cost: f64, duration: f64, speed: f64, departPos: f64, arrivalPos: f64, _destStop: &str) -> Self {
        ROPerson_Walk {
            base: ROPerson_TripItem::new(start, cost),
            edges: edges.clone(),
            exitTimes: Vec::new(),
            dur: duration,
            v: speed,
            dep: departPos,
            arr: arrivalPos,
            destStop: _destStop.to_string(),
        }
    }

    /// C++: `TripItem * Walk::clone() const` [const, virtual]
    // TripItem* clone() const {
    //             return new Walk(myStart, edges, myCost, exitTimes, dep, arr, destStop);
    //         }
    #[allow(unused_variables)]
    pub fn clone(&self) -> Ptr<ROPerson_TripItem> {
        Ptr::from_raw(Box::into_raw(Box::new(ROPerson_Walk::new(self.base.myStart, &self.edges, self.base.myCost, &self.exitTimes, self.dep, self.arr, &self.destStop)))).cast::<ROPerson_TripItem>()
    }

    /// C++: `const ROEdge * Walk::getOrigin() const` [const, virtual]
    // inline const ROEdge* getOrigin() const {
    //             return edges.front();
    //         }
    #[allow(unused_variables)]
    pub fn get_origin(&self) -> Ptr<ROEdge> {
        self.edges[0]
    }

    /// C++: `const ROEdge * Walk::getDestination() const` [const, virtual]
    // inline const ROEdge* getDestination() const {
    //             return edges.back();
    //         }
    #[allow(unused_variables)]
    pub fn get_destination(&self) -> Ptr<ROEdge> {
        self.edges[self.edges.len() - 1]
    }

    /// C++: `double Walk::getDestinationPos() const` [const, virtual]
    // inline double getDestinationPos() const {
    //             return arr == std::numeric_limits<double>::infinity() ? 0 : arr;
    //         }
    #[allow(unused_variables)]
    pub fn get_destination_pos(&self) -> f64 {
        if self.arr == f64::INFINITY { 0. } else { self.arr }
    }

    /// C++: `void Walk::saveAsXML(OutputDevice &, const bool, OptionsCont &) const` [const, virtual]
    // void
    // ROPerson::Walk::saveAsXML(OutputDevice& os, const bool extended, OptionsCont& options) const {
    //     os.openTag(SUMO_TAG_WALK);
    //     std::string comment = "";
    //     if ((extended || options.getBool("write-costs")) && myCost >= 0.) {
    //         os.writeAttr(SUMO_ATTR_COST, myCost);
    //     }
    //     if (dur > 0) {
    //         os.writeAttr(SUMO_ATTR_DURATION, dur);
    //     }
    //     if (v > 0) {
    //         os.writeAttr(SUMO_ATTR_SPEED, v);
    //     }
    //     os.writeAttr(SUMO_ATTR_EDGES, edges);
    //     if (options.getBool("exit-times")) {
    //         os.writeAttr("started", time2string(getStart()));
    //         os.writeAttr("ended", time2string(getStart() + getDuration()));
    //         if (!exitTimes.empty()) {
    //             os.writeAttr("exitTimes", exitTimes);
    //         }
    //     }
    //     if (options.getBool("route-length")) {
    //         double length = 0;
    //         for (const ROEdge* roe : edges) {
    //             length += roe->getLength();
    //         }
    //         os.writeAttr("routeLength", length);
    //     }
    //     if (destStop != "") {
    //         const std::string element = RONet::getInstance()->getStoppingPlaceElement(destStop);
    //         os.writeAttr(element, destStop);
    //         const std::string name = RONet::getInstance()->getStoppingPlaceName(destStop);
    //         if (name != "") {
    //             comment =  " <!-- " + name + " -->";
    //         }
    //     } else if (arr != 0 && arr != std::numeric_limits<double>::infinity()) {
    //         os.writeAttr(SUMO_ATTR_ARRIVALPOS, arr);
    //     }
    //     os.closeTag(comment);
    // }
    #[allow(unused_variables)]
    pub fn save_as_xml(&self, os: &mut OutputDevice, extended: bool, options: &mut OptionsCont) {
        os.open_tag_id(SUMO_TAG_WALK);
        let mut comment = String::new();
        if (extended || options.get_bool("write-costs").unwrap_or_else(|e| panic!("{e}"))) && self.base.myCost >= 0. {
            os.write_attr(SUMO_ATTR_COST, &self.base.myCost);
        }
        if self.dur > 0. {
            os.write_attr(SUMO_ATTR_DURATION, &self.dur);
        }
        if self.v > 0. {
            os.write_attr(SUMO_ATTR_SPEED, &self.v);
        }
        let edgeIds: Vec<String> = self.edges.iter().map(|e| e.get_id().to_string()).collect();
        os.write_attr(SUMO_ATTR_EDGES, &edgeIds.join(" "));
        if options.get_bool("exit-times").unwrap_or_else(|e| panic!("{e}")) {
            os.write_attr("started", &utils::common::time::time2string(self.get_start()));
            os.write_attr("ended", &utils::common::time::time2string(self.get_start() + self.get_duration()));
            if !self.exitTimes.is_empty() {
                os.write_attr("exitTimes", &self.exitTimes);
            }
        }
        if options.get_bool("route-length").unwrap_or_else(|e| panic!("{e}")) {
            let mut length = 0.;
            for roe in self.edges.iter() {
                length += roe.get_length();
            }
            os.write_attr("routeLength", &length);
        }
        if self.destStop != "" {
            let element = RONet::get_instance().get_stopping_place_element(&self.destStop);
            os.write_attr(element.as_str(), &self.destStop);
            let name = RONet::get_instance().get_stopping_place_name(&self.destStop);
            if name != "" {
                comment = format!(" <!-- {} -->", name);
            }
        } else if self.arr != 0. && self.arr != f64::INFINITY {
            os.write_attr(SUMO_ATTR_ARRIVALPOS, &self.arr);
        }
        os.close_tag(&comment);
    }

    /// C++: `Walk & Walk::operator=(const Walk &)` [private]
    /// Invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, src: &ROPerson_Walk) -> &mut ROPerson_Walk {
        panic!("ROPerson::Walk::operator= is private and undefined in C++ (invalidated copy)")
    }

}

