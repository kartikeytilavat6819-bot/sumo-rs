//! Port of router/ROPerson.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

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
    pub myTripItems: Vec<Ptr<IntermodalRouter_TripItem>>,
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
        todo!("port PersonTrip::PersonTrip")
    }

    /// C++: `void PersonTrip::PersonTrip(const ROEdge *const, const ROEdge *const, const SVCPermissions, const double, const std::string &, const double, const std::string &, double, const std::string &)`
    // PersonTrip(const ROEdge* const _from, const ROEdge* const _to, const SVCPermissions modeSet,
    //                    const double departPos, const std::string& _stopOrigin, const double arrivalPos, const std::string& _stopDest, double _walkFactor, const std::string& _group) :
    //             from(_from), to(_to), modes(modeSet), dep(departPos), arr(arrivalPos), stopOrigin(_stopOrigin), stopDest(_stopDest), walkFactor(_walkFactor), group(_group) { }
    #[allow(unused_variables)]
    pub fn new_2(_from: Ptr<ROEdge>, _to: Ptr<ROEdge>, modeSet: SVCPermissions, departPos: f64, _stopOrigin: &str, arrivalPos: f64, _stopDest: &str, _walkFactor: f64, _group: &str) -> Self {
        todo!("port PersonTrip::PersonTrip")
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
        todo!("port PersonTrip::~PersonTrip")
    }

    /// C++: `PlanItem * PersonTrip::clone() const` [const, virtual]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn clone(&self) -> Ptr<ROPerson_PlanItem> {
        todo!("port PersonTrip::clone")
    }

    /// C++: `void PersonTrip::addTripItem(TripItem *)` [virtual]
    // virtual void addTripItem(TripItem* tripIt) {
    //             myTripItems.push_back(tripIt);
    //         }
    #[allow(unused_variables)]
    pub fn add_trip_item(&mut self, tripIt: Ptr<IntermodalRouter_TripItem>) {
        todo!("port PersonTrip::addTripItem")
    }

    /// C++: `void PersonTrip::addVehicle(ROVehicle *)`
    // void addVehicle(ROVehicle* veh) {
    //             myVehicles.push_back(veh);
    //         }
    #[allow(unused_variables)]
    pub fn add_vehicle(&mut self, veh: Ptr<ROVehicle>) {
        todo!("port PersonTrip::addVehicle")
    }

    /// C++: `std::vector<ROVehicle *> & PersonTrip::getVehicles()`
    // std::vector<ROVehicle*>& getVehicles() {
    //             return myVehicles;
    //         }
    #[allow(unused_variables)]
    pub fn get_vehicles(&mut self) -> &mut Vec<Ptr<ROVehicle>> {
        todo!("port PersonTrip::getVehicles")
    }

    /// C++: `const ROEdge * PersonTrip::getOrigin() const` [const, virtual]
    // const ROEdge* getOrigin() const {
    //             return from != 0 ? from : myTripItems.front()->getOrigin();
    //         }
    #[allow(unused_variables)]
    pub fn get_origin(&self) -> Ptr<ROEdge> {
        todo!("port PersonTrip::getOrigin")
    }

    /// C++: `const ROEdge * PersonTrip::getDestination() const` [const, virtual]
    // const ROEdge* getDestination() const {
    //             return to;
    //         }
    #[allow(unused_variables)]
    pub fn get_destination(&self) -> Ptr<ROEdge> {
        todo!("port PersonTrip::getDestination")
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
        todo!("port PersonTrip::getDestinationPos")
    }

    /// C++: `double PersonTrip::getDepartPos(bool) const` [const]
    // double getDepartPos(bool replaceDefault = true) const {
    //             return dep == std::numeric_limits<double>::infinity() && replaceDefault ? 0 : dep;
    //         }
    #[allow(unused_variables)]
    pub fn get_depart_pos(&self, replaceDefault: bool) -> f64 {
        todo!("port PersonTrip::getDepartPos")
    }

    /// C++: `double PersonTrip::getArrivalPos(bool) const` [const]
    // double getArrivalPos(bool replaceDefault = true) const {
    //             return arr == std::numeric_limits<double>::infinity() && replaceDefault ? to->getLength() / 2 : arr;
    //         }
    #[allow(unused_variables)]
    pub fn get_arrival_pos(&self, replaceDefault: bool) -> f64 {
        todo!("port PersonTrip::getArrivalPos")
    }

    /// C++: `SVCPermissions PersonTrip::getModes() const` [const]
    // SVCPermissions getModes() const {
    //             return modes;
    //         }
    #[allow(unused_variables)]
    pub fn get_modes(&self) -> SVCPermissions {
        todo!("port PersonTrip::getModes")
    }

    /// C++: `void PersonTrip::updateModes(SVCPermissions)`
    // void updateModes(SVCPermissions additionalModes) {
    //             modes |= additionalModes;
    //         }
    #[allow(unused_variables)]
    pub fn update_modes(&mut self, additionalModes: SVCPermissions) {
        todo!("port PersonTrip::updateModes")
    }

    /// C++: `const std::string & PersonTrip::getGroup() const` [const]
    // const std::string& getGroup() const {
    //             return group;
    //         }
    #[allow(unused_variables)]
    pub fn get_group(&self) -> &str {
        todo!("port PersonTrip::getGroup")
    }

    /// C++: `const std::string & PersonTrip::getStopOrigin() const` [const]
    // const std::string& getStopOrigin() const {
    //             return stopOrigin;
    //         }
    #[allow(unused_variables)]
    pub fn get_stop_origin(&self) -> &str {
        todo!("port PersonTrip::getStopOrigin")
    }

    /// C++: `const std::string & PersonTrip::getStopDest() const` [const, virtual]
    // const std::string& getStopDest() const {
    //             return stopDest;
    //         }
    #[allow(unused_variables)]
    pub fn get_stop_dest(&self) -> &str {
        todo!("port PersonTrip::getStopDest")
    }

    /// C++: `bool PersonTrip::needsRouting() const` [const, virtual]
    // virtual bool needsRouting() const {
    //             return myTripItems.empty();
    //         }
    #[allow(unused_variables)]
    pub fn needs_routing(&self) -> bool {
        todo!("port PersonTrip::needsRouting")
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
    pub fn set_items(&mut self, newItems: &mut Vec<Ptr<IntermodalRouter_TripItem>>, veh: Ptr<ROVehicle>) {
        todo!("port PersonTrip::setItems")
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
        todo!("port PersonTrip::saveVehicles")
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
        todo!("port PersonTrip::saveAsXML")
    }

    /// C++: `double PersonTrip::getWalkFactor() const` [const]
    // double getWalkFactor() const {
    //             return walkFactor;
    //         }
    #[allow(unused_variables)]
    pub fn get_walk_factor(&self) -> f64 {
        todo!("port PersonTrip::getWalkFactor")
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
        todo!("port PersonTrip::getDuration")
    }

    /// C++: `PersonTrip & PersonTrip::operator=(const PersonTrip &)` [private]
    /// Invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, src: &ROPerson_PersonTrip) -> &mut ROPerson_PersonTrip {
        todo!("port PersonTrip::operator=")
    }

}

/// Every person has a plan comprising of multiple planItems
/// C++ class `ROPerson::PlanItem` (router/ROPerson.h)
#[repr(C)]
pub struct ROPerson_PlanItem {
    /// C++ base class `Parameterised`
    pub base: Parameterised,
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
        todo!("port PlanItem::~PlanItem")
    }

    /// C++: `PlanItem * PlanItem::clone() const` [const, virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn clone(&self) -> Ptr<ROPerson_PlanItem> {
        todo!("port PlanItem::clone")
    }

    /// C++: `void PlanItem::addTripItem(TripItem *)` [virtual]
    // virtual void addTripItem(TripItem* /* tripIt */) {
    //             throw ProcessError();
    //         }
    #[allow(unused_variables)]
    pub fn add_trip_item(&mut self, arg0: Ptr<IntermodalRouter_TripItem>) {
        todo!("port PlanItem::addTripItem")
    }

    /// C++: `const ROEdge * PlanItem::getOrigin() const` [const, virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_origin(&self) -> Ptr<ROEdge> {
        todo!("port PlanItem::getOrigin")
    }

    /// C++: `const ROEdge * PlanItem::getDestination() const` [const, virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_destination(&self) -> Ptr<ROEdge> {
        todo!("port PlanItem::getDestination")
    }

    /// C++: `double PlanItem::getDestinationPos() const` [const, virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_destination_pos(&self) -> f64 {
        todo!("port PlanItem::getDestinationPos")
    }

    /// C++: `void PlanItem::saveVehicles(OutputDevice &, OutputDevice *const, bool, OptionsCont &) const` [const, virtual]
    // virtual void saveVehicles(OutputDevice& /* os */, OutputDevice* const /* typeos */, bool /* asAlternatives */, OptionsCont& /* options */) const {}
    #[allow(unused_variables)]
    pub fn save_vehicles(&self, arg0: &mut OutputDevice, arg1: Ptr<OutputDevice>, arg2: bool, arg3: &mut OptionsCont) {
        todo!("port PlanItem::saveVehicles")
    }

    /// C++: `void PlanItem::saveAsXML(OutputDevice &, const bool, const bool, OptionsCont &) const` [const, virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn save_as_xml(&self, os: &mut OutputDevice, extended: bool, asTrip: bool, options: &mut OptionsCont) {
        todo!("port PlanItem::saveAsXML")
    }

    /// C++: `bool PlanItem::isStop() const` [const, virtual]
    // virtual bool isStop() const {
    //             return false;
    //         }
    #[allow(unused_variables)]
    pub fn is_stop(&self) -> bool {
        todo!("port PlanItem::isStop")
    }

    /// C++: `bool PlanItem::needsRouting() const` [const, virtual]
    // virtual bool needsRouting() const {
    //             return false;
    //         }
    #[allow(unused_variables)]
    pub fn needs_routing(&self) -> bool {
        todo!("port PlanItem::needsRouting")
    }

    /// C++: `SUMOVehicleParameter::Stop * PlanItem::getStopParameters()` [virtual]
    // virtual SUMOVehicleParameter::Stop* getStopParameters() {
    //             return nullptr;
    //         }
    #[allow(unused_variables)]
    pub fn get_stop_parameters(&mut self) -> Ptr<SUMOVehicleParameter_Stop> {
        todo!("port PlanItem::getStopParameters")
    }

    /// C++: `SUMOTime PlanItem::getDuration() const` [const, virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_duration(&self) -> SUMOTime {
        todo!("port PlanItem::getDuration")
    }

    /// C++: `const std::string & PlanItem::getStopDest() const` [const, virtual]
    // virtual const std::string& getStopDest() const {
    //             return UNDEFINED_STOPPING_PLACE;
    //         }
    #[allow(unused_variables)]
    pub fn get_stop_dest(&self) -> &str {
        todo!("port PlanItem::getStopDest")
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

impl ROPerson {
    /// C++: `void ROPerson::ROPerson(const SUMOVehicleParameter &, const SUMOVTypeParameter *)`
    /// Constructor
    // ROPerson::ROPerson(const SUMOVehicleParameter& pars, const SUMOVTypeParameter* type)
    //     : RORoutable(pars, type) {
    // }
    #[allow(unused_variables)]
    pub fn new(pars: &SUMOVehicleParameter, type_: Ptr<SUMOVTypeParameter>) -> Self {
        todo!("port ROPerson::ROPerson")
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
        todo!("port ROPerson::~ROPerson")
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
        todo!("port ROPerson::addTrip")
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
        todo!("port ROPerson::addRide")
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
        todo!("port ROPerson::addWalk")
    }

    /// C++: `void ROPerson::addStop(std::vector<PlanItem *> &, const SUMOVehicleParameter::Stop &, const ROEdge *const)` [static]
    // void
    // ROPerson::addStop(std::vector<PlanItem*>& plan, const SUMOVehicleParameter::Stop& stopPar, const ROEdge* const stopEdge) {
    //     plan.push_back(new Stop(stopPar, stopEdge));
    // }
    #[allow(unused_variables)]
    pub fn add_stop(plan: &mut Vec<Ptr<ROPerson_PlanItem>>, stopPar: &SUMOVehicleParameter_Stop, stopEdge: Ptr<ROEdge>) {
        todo!("port ROPerson::addStop")
    }

    /// C++: `const ROEdge * ROPerson::getDepartEdge() const` [const, virtual]
    /// Returns the first edge the person takes
    // const ROEdge* getDepartEdge() const {
    //         return myPlan.front()->getOrigin();
    //     }
    #[allow(unused_variables)]
    pub fn get_depart_edge(&self) -> Ptr<ROEdge> {
        todo!("port ROPerson::getDepartEdge")
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
        todo!("port ROPerson::computeRoute")
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
        todo!("port ROPerson::saveAsXML")
    }

    /// C++: `std::vector<PlanItem *> & ROPerson::getPlan()`
    // std::vector<PlanItem*>& getPlan() {
    //         return myPlan;
    //     }
    #[allow(unused_variables)]
    pub fn get_plan(&mut self) -> &mut Vec<Ptr<ROPerson_PlanItem>> {
        todo!("port ROPerson::getPlan")
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
    pub fn compute_intermodal(&mut self, time: SUMOTime, provider: &RORouterProvider, trip: Ptr<ROPerson_PersonTrip>, veh: Ptr<ROVehicle>, resultItems: &mut Vec<Ptr<IntermodalRouter_TripItem>>, errorHandler: Ptr<MsgHandler>) -> bool {
        todo!("port ROPerson::computeIntermodal")
    }

    /// C++: `void ROPerson::ROPerson(const ROPerson &)` [private]
    /// Invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(src: &ROPerson) -> Self {
        todo!("port ROPerson::ROPerson")
    }

    /// C++: `ROPerson & ROPerson::operator=(const ROPerson &)` [private]
    /// Invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, src: &ROPerson) -> &mut ROPerson {
        todo!("port ROPerson::operator=")
    }

}

/// A ride is part of a trip, e.g., go from here to here by car or bus
/// C++ class `ROPerson::Ride` (router/ROPerson.h)
#[repr(C)]
pub struct ROPerson_Ride {
    /// C++ base class `TripItem`
    pub base: IntermodalRouter_TripItem,
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
    type Target = IntermodalRouter_TripItem;
    fn deref(&self) -> &IntermodalRouter_TripItem {
        &self.base
    }
}
impl std::ops::DerefMut for ROPerson_Ride {
    fn deref_mut(&mut self) -> &mut IntermodalRouter_TripItem {
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
        todo!("port Ride::Ride")
    }

    /// C++: `TripItem * Ride::clone() const` [const, virtual]
    // TripItem* clone() const {
    //             return new Ride(myStart, from, to, lines, group, myCost, arrPos, length, destStop, intended, depart);
    //         }
    #[allow(unused_variables)]
    pub fn clone(&self) -> Ptr<IntermodalRouter_TripItem> {
        todo!("port Ride::clone")
    }

    /// C++: `const ROEdge * Ride::getOrigin() const` [const, virtual]
    // inline const ROEdge* getOrigin() const {
    //             return from;
    //         }
    #[allow(unused_variables)]
    pub fn get_origin(&self) -> Ptr<ROEdge> {
        todo!("port Ride::getOrigin")
    }

    /// C++: `const ROEdge * Ride::getDestination() const` [const, virtual]
    // inline const ROEdge* getDestination() const {
    //             return to;
    //         }
    #[allow(unused_variables)]
    pub fn get_destination(&self) -> Ptr<ROEdge> {
        todo!("port Ride::getDestination")
    }

    /// C++: `double Ride::getDestinationPos() const` [const, virtual]
    // inline double getDestinationPos() const {
    //             return arrPos == std::numeric_limits<double>::infinity() ? -NUMERICAL_EPS : arrPos;
    //         }
    #[allow(unused_variables)]
    pub fn get_destination_pos(&self) -> f64 {
        todo!("port Ride::getDestinationPos")
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
        todo!("port Ride::saveAsXML")
    }

    /// C++: `Ride & Ride::operator=(const Ride &)` [private]
    /// Invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, src: &ROPerson_Ride) -> &mut ROPerson_Ride {
        todo!("port Ride::operator=")
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
        todo!("port Stop::Stop")
    }

    /// C++: `PlanItem * Stop::clone() const` [const, virtual]
    // PlanItem* clone() const {
    //             return new Stop(stopDesc, edge);
    //         }
    #[allow(unused_variables)]
    pub fn clone(&self) -> Ptr<ROPerson_PlanItem> {
        todo!("port Stop::clone")
    }

    /// C++: `const ROEdge * Stop::getOrigin() const` [const, virtual]
    // const ROEdge* getOrigin() const {
    //             return edge;
    //         }
    #[allow(unused_variables)]
    pub fn get_origin(&self) -> Ptr<ROEdge> {
        todo!("port Stop::getOrigin")
    }

    /// C++: `const ROEdge * Stop::getDestination() const` [const, virtual]
    // const ROEdge* getDestination() const {
    //             return edge;
    //         }
    #[allow(unused_variables)]
    pub fn get_destination(&self) -> Ptr<ROEdge> {
        todo!("port Stop::getDestination")
    }

    /// C++: `double Stop::getDestinationPos() const` [const, virtual]
    // double getDestinationPos() const {
    //             return (stopDesc.startPos + stopDesc.endPos) / 2;
    //         }
    #[allow(unused_variables)]
    pub fn get_destination_pos(&self) -> f64 {
        todo!("port Stop::getDestinationPos")
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
        todo!("port Stop::saveAsXML")
    }

    /// C++: `bool Stop::isStop() const` [const, virtual]
    // bool isStop() const {
    //             return true;
    //         }
    #[allow(unused_variables)]
    pub fn is_stop(&self) -> bool {
        todo!("port Stop::isStop")
    }

    /// C++: `SUMOVehicleParameter::Stop * Stop::getStopParameters()` [virtual]
    // virtual SUMOVehicleParameter::Stop* getStopParameters() {
    //             return &stopDesc;
    //         }
    #[allow(unused_variables)]
    pub fn get_stop_parameters(&mut self) -> Ptr<SUMOVehicleParameter_Stop> {
        todo!("port Stop::getStopParameters")
    }

    /// C++: `SUMOTime Stop::getDuration() const` [const, virtual]
    // SUMOTime getDuration() const {
    //             return stopDesc.duration;
    //         }
    #[allow(unused_variables)]
    pub fn get_duration(&self) -> SUMOTime {
        todo!("port Stop::getDuration")
    }

    /// C++: `const std::string & Stop::getStopDest() const` [const, virtual]
    // inline const std::string& getStopDest() const {
    //             return stopDesc.busstop;
    //         }
    #[allow(unused_variables)]
    pub fn get_stop_dest(&self) -> &str {
        todo!("port Stop::getStopDest")
    }

    /// C++: `Stop & Stop::operator=(const Stop &)` [private]
    /// Invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, src: &SUMOVehicleParameter_Stop) -> &mut SUMOVehicleParameter_Stop {
        todo!("port Stop::operator=")
    }

}

/// A TripItem is part of a trip, e.g., go from here to here by car
/// C++ class `ROPerson::TripItem` (router/ROPerson.h)
pub struct ROPerson_TripItem {
    pub myStart: SUMOTime,
    pub myCost: f64,
}

impl ROPerson_TripItem {
    /// C++: `void TripItem::TripItem(const SUMOTime, const double)`
    // TripItem(const SUMOTime start, const double cost)
    //             : myStart(start), myCost(cost) {}
    #[allow(unused_variables)]
    pub fn new(start: SUMOTime, cost: f64) -> Self {
        todo!("port TripItem::TripItem")
    }

    /// C++: `void TripItem::~TripItem() noexcept` [virtual]
    /// Destructor
    // virtual ~TripItem() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port TripItem::~TripItem")
    }

    /// C++: `TripItem * TripItem::clone() const` [const, virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn clone(&self) -> Ptr<IntermodalRouter_TripItem> {
        todo!("port TripItem::clone")
    }

    /// C++: `const ROEdge * TripItem::getOrigin() const` [const, virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_origin(&self) -> Ptr<ROEdge> {
        todo!("port TripItem::getOrigin")
    }

    /// C++: `const ROEdge * TripItem::getDestination() const` [const, virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_destination(&self) -> Ptr<ROEdge> {
        todo!("port TripItem::getDestination")
    }

    /// C++: `double TripItem::getDestinationPos() const` [const, virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_destination_pos(&self) -> f64 {
        todo!("port TripItem::getDestinationPos")
    }

    /// C++: `void TripItem::saveAsXML(OutputDevice &, const bool, OptionsCont &) const` [const, virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn save_as_xml(&self, os: &mut OutputDevice, extended: bool, options: &mut OptionsCont) {
        todo!("port TripItem::saveAsXML")
    }

    /// C++: `SUMOTime TripItem::getStart() const` [const]
    // inline SUMOTime getStart() const {
    //             return myStart;
    //         }
    #[allow(unused_variables)]
    pub fn get_start(&self) -> SUMOTime {
        todo!("port TripItem::getStart")
    }

    /// C++: `SUMOTime TripItem::getDuration() const` [const]
    // inline SUMOTime getDuration() const {
    //             return TIME2STEPS(myCost);
    //         }
    #[allow(unused_variables)]
    pub fn get_duration(&self) -> SUMOTime {
        todo!("port TripItem::getDuration")
    }

    /// C++: `double TripItem::getCost() const` [const]
    // inline double getCost() const {
    //             return myCost;
    //         }
    #[allow(unused_variables)]
    pub fn get_cost(&self) -> f64 {
        todo!("port TripItem::getCost")
    }

}

/// A walk is part of a trip, e.g., go from here to here by foot
/// C++ class `ROPerson::Walk` (router/ROPerson.h)
#[repr(C)]
pub struct ROPerson_Walk {
    /// C++ base class `TripItem`
    pub base: IntermodalRouter_TripItem,
    pub edges: ConstROEdgeVector,
    pub exitTimes: Vec<f64>,
    pub dur: f64,
    pub v: f64,
    pub dep: f64,
    pub arr: f64,
    pub destStop: String,
}

impl std::ops::Deref for ROPerson_Walk {
    type Target = IntermodalRouter_TripItem;
    fn deref(&self) -> &IntermodalRouter_TripItem {
        &self.base
    }
}
impl std::ops::DerefMut for ROPerson_Walk {
    fn deref_mut(&mut self) -> &mut IntermodalRouter_TripItem {
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
        todo!("port Walk::Walk")
    }

    /// C++: `void Walk::Walk(const SUMOTime, const ConstROEdgeVector &, const double, const double, const double, const double, const double, const std::string &)`
    // Walk(const SUMOTime start, const ConstROEdgeVector& edges, const double cost, const double duration, const double speed,
    //              const double departPos, const double arrivalPos, const std::string& _destStop)
    //             : TripItem(start, cost), edges(edges), dur(duration), v(speed), dep(departPos), arr(arrivalPos), destStop(_destStop) {}
    #[allow(unused_variables)]
    pub fn new_2(start: SUMOTime, edges: &ConstROEdgeVector, cost: f64, duration: f64, speed: f64, departPos: f64, arrivalPos: f64, _destStop: &str) -> Self {
        todo!("port Walk::Walk")
    }

    /// C++: `TripItem * Walk::clone() const` [const, virtual]
    // TripItem* clone() const {
    //             return new Walk(myStart, edges, myCost, exitTimes, dep, arr, destStop);
    //         }
    #[allow(unused_variables)]
    pub fn clone(&self) -> Ptr<IntermodalRouter_TripItem> {
        todo!("port Walk::clone")
    }

    /// C++: `const ROEdge * Walk::getOrigin() const` [const, virtual]
    // inline const ROEdge* getOrigin() const {
    //             return edges.front();
    //         }
    #[allow(unused_variables)]
    pub fn get_origin(&self) -> Ptr<ROEdge> {
        todo!("port Walk::getOrigin")
    }

    /// C++: `const ROEdge * Walk::getDestination() const` [const, virtual]
    // inline const ROEdge* getDestination() const {
    //             return edges.back();
    //         }
    #[allow(unused_variables)]
    pub fn get_destination(&self) -> Ptr<ROEdge> {
        todo!("port Walk::getDestination")
    }

    /// C++: `double Walk::getDestinationPos() const` [const, virtual]
    // inline double getDestinationPos() const {
    //             return arr == std::numeric_limits<double>::infinity() ? 0 : arr;
    //         }
    #[allow(unused_variables)]
    pub fn get_destination_pos(&self) -> f64 {
        todo!("port Walk::getDestinationPos")
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
        todo!("port Walk::saveAsXML")
    }

    /// C++: `Walk & Walk::operator=(const Walk &)` [private]
    /// Invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, src: &ROPerson_Walk) -> &mut ROPerson_Walk {
        todo!("port Walk::operator=")
    }

}

