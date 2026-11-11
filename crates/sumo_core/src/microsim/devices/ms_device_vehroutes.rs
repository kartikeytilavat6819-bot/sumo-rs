//! Port of microsim/devices/MSDevice_Vehroutes.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// A device which collects info on the vehicle trip (mainly on departure and arrival)
/// C++ class `MSDevice_Vehroutes` (microsim/devices/MSDevice_Vehroutes.h)
pub struct MSDevice_Vehroutes {
    /// C++ base class `MSVehicleDevice`
    pub base: MSVehicleDevice,
    /// The currently used route
    pub myCurrentRoute: ConstMSRoutePtr,
    /// Whether the currently used route is valid/complete
    pub myCurrentValid: bool,
    /// Prior routes
    pub myReplacedRoutes: i32,
    /// The times at which the vehicle exits an edge
    pub myExits: i32,
    /// The maximum number of routes to report
    pub myMaxRoutes: i32,
    /// The last edge the exit time was saved for
    pub myLastSavedAt: Ptr<MSEdge>,
    /// The route index of the last edge that the vehicle left
    pub myLastRouteIndex: i32,
    /// The lane the vehicle departed at
    pub myDepartLane: i32,
    /// The lane the vehicle departed at
    pub myDepartPos: f64,
    /// The speed on departure
    pub myDepartSpeed: f64,
    /// The lateral depart position
    pub myDepartPosLat: f64,
    /// the edges that were passed before the current stop
    pub myPriorEdges: i32,
    /// the cost of the initial route if not set (cannot be stored in the route because it may be shared)
    pub myInitialCost: f64,
    pub myStopOut: Unmapped /* OutputDevice_String */,
}

impl std::ops::Deref for MSDevice_Vehroutes {
    type Target = MSVehicleDevice;
    fn deref(&self) -> &MSVehicleDevice {
        &self.base
    }
}
impl std::ops::DerefMut for MSDevice_Vehroutes {
    fn deref_mut(&mut self) -> &mut MSVehicleDevice {
        &mut self.base
    }
}

// C++ static members of MSDevice_Vehroutes (port as module-level statics, see AGENTS.md):
//   static bool MSDevice_Vehroutes::mySaveExits;
//   static bool MSDevice_Vehroutes::myLastRouteOnly;
//   static bool MSDevice_Vehroutes::myDUAStyle;
//   static bool MSDevice_Vehroutes::myWriteCosts;
//   static bool MSDevice_Vehroutes::mySorted;
//   static bool MSDevice_Vehroutes::myIntendedDepart;
//   static bool MSDevice_Vehroutes::myRouteLength;
//   static bool MSDevice_Vehroutes::mySkipPTLines;
//   static bool MSDevice_Vehroutes::myIncludeIncomplete;
//   static bool MSDevice_Vehroutes::myWriteStopPriorEdges;
//   static bool MSDevice_Vehroutes::myWriteInternal;
//   static StateListener MSDevice_Vehroutes::myStateListener;
//   static SortedRouteInfo MSDevice_Vehroutes::myRouteInfos;

impl MSDevice_Vehroutes {
    /// C++: `void MSDevice_Vehroutes::init()` [static]
    /// Static intialization
    // void
    // MSDevice_Vehroutes::init() {
    //     const OptionsCont& oc = OptionsCont::getOptions();
    //     if (oc.isSet("vehroute-output")) {
    //         OutputDevice::createDeviceByOption("vehroute-output", "routes", "routes_file.xsd");
    //         OutputDevice& od = OutputDevice::getDeviceByOption("vehroute-output");
    //         if (!od.isXML()) {
    //             WRITE_WARNING("Vehroute output does not fully support tabular formats. Sorting and stop output will not work.");
    //         }
    //         mySaveExits = oc.getBool("vehroute-output.exit-times");
    //         myLastRouteOnly = oc.getBool("vehroute-output.last-route");
    //         myDUAStyle = oc.getBool("vehroute-output.dua");
    //         myWriteCosts = oc.getBool("vehroute-output.cost");
    //         mySorted = myDUAStyle || oc.getBool("vehroute-output.sorted");
    //         myIntendedDepart = oc.getBool("vehroute-output.intended-depart");
    //         myRouteLength = oc.getBool("vehroute-output.route-length");
    //         mySkipPTLines = oc.getBool("vehroute-output.skip-ptlines");
    //         myIncludeIncomplete = oc.getBool("vehroute-output.incomplete");
    //         myWriteStopPriorEdges = oc.getBool("vehroute-output.stop-edges");
    //         myWriteInternal = oc.getBool("vehroute-output.internal");
    //         MSNet::getInstance()->addVehicleStateListener(&myStateListener);
    //         myRouteInfos.routeOut = &od;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn init() {
        todo!("port MSDevice_Vehroutes::init")
    }

    /// C++: `void MSDevice_Vehroutes::insertOptions(OptionsCont &)` [static]
    /// Inserts MSDevice_FCD-options
    // void
    // MSDevice_Vehroutes::insertOptions(OptionsCont& oc) {
    //     oc.addOptionSubTopic("Vehroutes Device");
    //     insertDefaultAssignmentOptions("vehroute", "Vehroutes Device", oc);
    // }
    #[allow(unused_variables)]
    pub fn insert_options(oc: &mut OptionsCont) {
        todo!("port MSDevice_Vehroutes::insertOptions")
    }

    /// C++: `MSDevice_Vehroutes * MSDevice_Vehroutes::buildVehicleDevices(SUMOVehicle &, int &, int)` [static]
    /// Build devices for the given vehicle, if needed
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn build_vehicle_devices(v: &mut SUMOVehicle, into: &mut i32, maxRoutes: i32) -> Ptr<MSDevice_Vehroutes> {
        todo!("port MSDevice_Vehroutes::buildVehicleDevices")
    }

    /// C++: `void MSDevice_Vehroutes::writePendingOutput(const bool)` [static]
    /// generate vehroute output for pending vehicles at sim end, either due to sorting or because they are still in the net
    // void
    // MSDevice_Vehroutes::writePendingOutput(const bool includeUnfinished) {
    //     MSNet* const net = MSNet::getInstance();
    //
    //     if (!includeUnfinished) {
    //         if (mySorted) {
    //             for (const auto& routeInfo : myRouteInfos.routeXML) {
    //                 for (const auto& rouXML : routeInfo.second) {
    //                     (*myRouteInfos.routeOut) << rouXML.second;
    //                 }
    //             }
    //             if (net->hasPersons()) {
    //                 const SortedRouteInfo& personRouteInfos = net->getPersonControl().getRouteInfo();
    //                 if (personRouteInfos.routeOut != myRouteInfos.routeOut) {
    //                     for (const auto& routeInfo : personRouteInfos.routeXML) {
    //                         for (const auto& rouXML : routeInfo.second) {
    //                             (*personRouteInfos.routeOut) << rouXML.second;
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //         return;
    //     }
    //     for (const auto& it : myStateListener.myDevices) {
    //         if (it.first->hasDeparted()) {
    //             if (it.first->isStopped()) {
    //                 it.second->notifyStopEnded();
    //             }
    //             it.second->writeOutput(false);
    //         }
    //     }
    //     // unfinished persons
    //     if (net->hasPersons()) {
    //         MSTransportableControl& pc = net->getPersonControl();
    //         while (pc.loadedBegin() != pc.loadedEnd()) {
    //             pc.erase(pc.loadedBegin()->second);
    //         }
    //     }
    //     // Flush any remaining sorted outputs that may still be buffered
    //     if (mySorted) {
    //         for (const auto& routeInfo : myRouteInfos.routeXML) {
    //             for (const auto& rouXML : routeInfo.second) {
    //                 (*myRouteInfos.routeOut) << rouXML.second;
    //             }
    //         }
    //         if (net->hasPersons()) {
    //             const SortedRouteInfo& personRouteInfos = net->getPersonControl().getRouteInfo();
    //             if (personRouteInfos.routeOut != myRouteInfos.routeOut) {
    //                 for (const auto& routeInfo : personRouteInfos.routeXML) {
    //                     for (const auto& rouXML : routeInfo.second) {
    //                         (*personRouteInfos.routeOut) << rouXML.second;
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn write_pending_output(includeUnfinished: bool) {
        todo!("port MSDevice_Vehroutes::writePendingOutput")
    }

    /// C++: `void MSDevice_Vehroutes::~MSDevice_Vehroutes()` [virtual]
    /// Destructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MSDevice_Vehroutes::~MSDevice_Vehroutes")
    }

    /// C++: `bool MSDevice_Vehroutes::notifyEnter(SUMOTrafficObject &, MSMoveReminder::Notification, const MSLane *)` [virtual]
    /// Does nothing, returns true only if exit times should be collected
    // bool
    // MSDevice_Vehroutes::notifyEnter(SUMOTrafficObject& veh, MSMoveReminder::Notification reason, const MSLane* enteredLane) {
    //     if (reason == MSMoveReminder::NOTIFICATION_DEPARTED) {
    //         if (mySorted && myStateListener.myDevices.count(static_cast<SUMOVehicle*>(&veh)) != 0
    //                 && myStateListener.myDevices[static_cast<SUMOVehicle*>(&veh)] == this) {
    //             const SUMOTime departure = myIntendedDepart ? myHolder.getParameter().depart : MSNet::getInstance()->getCurrentTimeStep();
    //             myRouteInfos.departureCounts[departure]++;
    //         }
    //         if (!MSGlobals::gUseMesoSim) {
    //             const MSVehicle& vehicle = static_cast<MSVehicle&>(veh);
    //             myDepartLane = vehicle.getLane()->getIndex();
    //             myDepartPosLat = vehicle.getLateralPositionOnLane();
    //         }
    //         myDepartSpeed = veh.getSpeed();
    //         myDepartPos = veh.getPositionOnLane();
    //     }
    //     if (myWriteStopPriorEdges) {
    //         if (MSGlobals::gUseMesoSim) {
    //             const MSEdge* e = veh.getEdge();
    //             if (myPriorEdges.empty() || myPriorEdges.back() != e) {
    //                 myPriorEdges.push_back(e);
    //             }
    //         } else {
    //             myPriorEdges.push_back(&enteredLane->getEdge());
    //         }
    //     }
    //     myLastRouteIndex = myHolder.getRoutePosition();
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn notify_enter(&mut self, veh: &mut SUMOTrafficObject, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
        todo!("port MSDevice_Vehroutes::notifyEnter")
    }

    /// C++: `bool MSDevice_Vehroutes::notifyLeave(SUMOTrafficObject &, double, Notification, const MSLane *)` [virtual]
    /// Saves exit times if needed
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn notify_leave(&mut self, veh: &mut SUMOTrafficObject, lastPos: f64, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
        todo!("port MSDevice_Vehroutes::notifyLeave")
    }

    /// C++: `const std::string MSDevice_Vehroutes::deviceName() const` [const, virtual]
    /// return the name for this type of device
    // const std::string deviceName() const {
    //         return "vehroute";
    //     }
    #[allow(unused_variables)]
    pub fn device_name(&self) -> String {
        todo!("port MSDevice_Vehroutes::deviceName")
    }

    /// C++: `void MSDevice_Vehroutes::notifyStopEnded()` [virtual]
    // void
    // MSDevice_Vehroutes::notifyStopEnded() {
    //     SUMOVehicleParameter::Stop stop = myHolder.getStops().front().pars;
    //     const bool closeLater = myWriteStopPriorEdges || mySaveExits;
    //     if (mySaveExits) {
    //         // prevent duplicate output
    //         stop.parametersSet &=  ~(STOP_STARTED_SET | STOP_ENDED_SET);
    //     }
    //     stop.write(myStopOut, !closeLater);
    //     if (myWriteStopPriorEdges) {
    //         // calculate length
    //         double priorEdgesLength = 0;
    //         for (int i = 0; i < (int)myPriorEdges.size(); i++) {
    //             if (i == 0) {
    //                 priorEdgesLength += myPriorEdges.at(i)->getLength();
    //             } else if (myPriorEdges.at(i)->getID() != myPriorEdges.at(i - 1)->getID()) {
    //                 priorEdgesLength += myPriorEdges.at(i)->getLength();
    //             }
    //         }
    //         myStopOut.writeAttr("priorEdges", myPriorEdges);
    //         myPriorEdges.clear();
    //         myStopOut.writeAttr("priorEdgesLength", priorEdgesLength);
    //     }
    //     if (mySaveExits) {
    //         myStopOut.writeAttr(SUMO_ATTR_STARTED, time2string(stop.started));
    //         myStopOut.writeAttr(SUMO_ATTR_ENDED, stop.ended < 0 ? "-1" : time2string(stop.ended));
    //     }
    //     if (closeLater) {
    //         myStopOut.closeTag();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn notify_stop_ended(&mut self) {
        todo!("port MSDevice_Vehroutes::notifyStopEnded")
    }

    /// C++: `void MSDevice_Vehroutes::generateOutput(OutputDevice *) const` [const, virtual]
    /// Called on writing vehroutes output
    // void
    // MSDevice_Vehroutes::generateOutput(OutputDevice* /*tripinfoOut*/) const {
    //     writeOutput(true);
    // }
    #[allow(unused_variables)]
    pub fn generate_output(&self, tripinfoOut: Ptr<OutputDevice>) {
        todo!("port MSDevice_Vehroutes::generateOutput")
    }

    /// C++: `ConstMSRoutePtr MSDevice_Vehroutes::getRoute(int) const` [const]
    /// Called on route retrieval
    // ConstMSRoutePtr
    // MSDevice_Vehroutes::getRoute(int index) const {
    //     if (index < (int)myReplacedRoutes.size()) {
    //         return myReplacedRoutes[index].route;
    //     } else {
    //         return nullptr;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_route(&self, index: i32) -> ConstMSRoutePtr {
        todo!("port MSDevice_Vehroutes::getRoute")
    }

    /// C++: `void MSDevice_Vehroutes::saveState(OutputDevice &) const` [const, virtual]
    /// Saves the state of the device
    // void
    // MSDevice_Vehroutes::saveState(OutputDevice& out) const {
    //     out.openTag(SUMO_TAG_DEVICE);
    //     out.writeAttr(SUMO_ATTR_ID, getID());
    //     std::vector<std::string> internals;
    //     if (!MSGlobals::gUseMesoSim) {
    //         internals.push_back(toString(myDepartLane));
    //         internals.push_back(toString(myDepartPosLat));
    //     }
    //     internals.push_back(toString(myDepartSpeed));
    //     internals.push_back(toString(myDepartPos));
    //     internals.push_back(toString(myInitialCost));
    //     internals.push_back(toString(myReplacedRoutes.size()));
    //     for (int i = 0; i < (int)myReplacedRoutes.size(); ++i) {
    //         const std::string replacedOnEdge = myReplacedRoutes[i].edge == nullptr ? "!NULL" : myReplacedRoutes[i].edge->getID();
    //         internals.push_back(replacedOnEdge);
    //         internals.push_back(toString(myReplacedRoutes[i].time));
    //         internals.push_back(myReplacedRoutes[i].route->getID());
    //         internals.push_back(myReplacedRoutes[i].info);
    //         internals.push_back(toString(myReplacedRoutes[i].lastRouteIndex));
    //         internals.push_back(toString(myReplacedRoutes[i].newRouteIndex));
    //         internals.push_back(toString(myReplacedRoutes[i].isValid));
    //     }
    //     out.writeAttr(SUMO_ATTR_STATE, toString(internals));
    //     if (mySaveExits && myExits.size() > 0) {
    //         out.writeAttr(SUMO_ATTR_EXITTIMES, myExits);
    //         out.writeAttr(SUMO_ATTR_EDGE, myLastSavedAt->getID());
    //     }
    //     out.closeTag();
    // }
    #[allow(unused_variables)]
    pub fn save_state(&self, out: &mut OutputDevice) {
        todo!("port MSDevice_Vehroutes::saveState")
    }

    /// C++: `void MSDevice_Vehroutes::loadState(const SUMOSAXAttributes &)` [virtual]
    /// Loads the state of the device from the given description
    // void
    // MSDevice_Vehroutes::loadState(const SUMOSAXAttributes& attrs) {
    //     std::istringstream bis(attrs.getString(SUMO_ATTR_STATE));
    //     if (!MSGlobals::gUseMesoSim) {
    //         bis >> myDepartLane;
    //         bis >> myDepartPosLat;
    //     }
    //     bis >> myDepartSpeed;
    //     bis >> myDepartPos;
    //     bis >> myInitialCost;
    //     int size;
    //     bis >> size;
    //     for (int i = 0; i < size; ++i) {
    //         std::string edgeID;
    //         SUMOTime time;
    //         std::string routeID;
    //         std::string info;
    //         int lastIndex;
    //         int newIndex;
    //         bis >> edgeID;
    //         bis >> time;
    //         bis >> routeID;
    //         bis >> info;
    //         bis >> lastIndex;
    //         bis >> newIndex;
    //         bis >> myCurrentValid;
    //
    //         ConstMSRoutePtr route = MSRoute::dictionary(routeID);
    //         if (route != nullptr) {
    //             myReplacedRoutes.push_back(RouteReplaceInfo(MSEdge::dictionary(edgeID), time, route, info, lastIndex, newIndex, myCurrentValid));
    //         }
    //     }
    //     if (mySaveExits && attrs.hasAttribute(SUMO_ATTR_EXITTIMES)) {
    //         bool ok = true;
    //         for (const std::string& t : attrs.get<std::vector<std::string> >(SUMO_ATTR_EXITTIMES, nullptr, ok)) {
    //             myExits.push_back(StringUtils::toLong(t));
    //         }
    //         if (attrs.hasAttribute(SUMO_ATTR_EDGE)) {
    //             myLastSavedAt = MSEdge::dictionary(attrs.getString(SUMO_ATTR_EDGE));
    //         }
    //     }
    //     if (myHolder.hasDeparted()) {
    //         myLastRouteIndex = myHolder.getRoutePosition();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn load_state(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port MSDevice_Vehroutes::loadState")
    }

    /// C++: `void MSDevice_Vehroutes::registerTransportableDepart(SUMOTime)` [static]
    // void
    // MSDevice_Vehroutes::registerTransportableDepart(SUMOTime depart) {
    //     myRouteInfos.departureCounts[depart]++;
    // }
    #[allow(unused_variables)]
    pub fn register_transportable_depart(depart: SUMOTime) {
        todo!("port MSDevice_Vehroutes::registerTransportableDepart")
    }

    /// C++: `void MSDevice_Vehroutes::writeSortedOutput(SortedRouteInfo *, SUMOTime, const SUMOTrafficObject::NumericalID, const std::string &)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn write_sorted_output(routeInfo: Ptr<MSDevice_Vehroutes_SortedRouteInfo>, depart: SUMOTime, id: SUMOTrafficObject_NumericalID, xmlOutput: &str) {
        todo!("port MSDevice_Vehroutes::writeSortedOutput")
    }

    /// C++: `void MSDevice_Vehroutes::MSDevice_Vehroutes(SUMOVehicle &, const std::string &, int)` [private]
    /// Constructor
    // MSDevice_Vehroutes::MSDevice_Vehroutes(SUMOVehicle& holder, const std::string& id, int maxRoutes) :
    //     MSVehicleDevice(holder, id),
    //     myCurrentRoute(holder.getRoutePtr()),
    //     myMaxRoutes(maxRoutes),
    //     myLastSavedAt(nullptr),
    //     myLastRouteIndex(-1),
    //     myDepartLane(-1),
    //     myDepartPos(-1),
    //     myDepartSpeed(-1),
    //     myDepartPosLat(0),
    //     myInitialCost(-1),
    //     myStopOut(2)
    // {
    //     std::string dummyMsg;
    //     myCurrentValid = myHolder.hasValidRoute(dummyMsg, myCurrentRoute);
    //     if (myCurrentRoute->getCosts() < 0 && myWriteCosts) {
    //         myInitialCost = myHolder.getRouterTT().recomputeCosts(myCurrentRoute->getEdges(), &myHolder, SIMSTEP);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn new(holder: &mut SUMOVehicle, id: &str, maxRoutes: i32) -> Self {
        todo!("port MSDevice_Vehroutes::MSDevice_Vehroutes")
    }

    /// C++: `void MSDevice_Vehroutes::writeXMLRoute(OutputDevice &, int) const` [const, private]
    /// Called on route output
    // void
    // MSDevice_Vehroutes::writeXMLRoute(OutputDevice& os, int index) const {
    //     if (index == 0 && !myIncludeIncomplete && myReplacedRoutes[index].route->size() == 2 &&
    //             myReplacedRoutes[index].route->getEdges().front()->isTazConnector() &&
    //             myReplacedRoutes[index].route->getEdges().back()->isTazConnector()) {
    //         return;
    //     }
    //     // check if a previous route shall be written
    //     //std::cout << " writeXMLRoute index=" << index << " numReroutes=" << myHolder.getNumberReroutes() << "\n";
    //     const int routesToSkip = myHolder.getParameter().wasSet(VEHPARS_FORCE_REROUTE) ? 1 : 0;
    //     os.openTag(SUMO_TAG_ROUTE);
    //     if (index >= 0) {
    //         assert((int)myReplacedRoutes.size() > index);
    //         if (myDUAStyle || myWriteCosts) {
    //             double cost = myReplacedRoutes[index].route->getCosts();
    //             if (cost < 0 && index == 0) {
    //                 cost = myInitialCost;
    //             }
    //             os.writeAttr(SUMO_ATTR_COST, cost);
    //         }
    //         if (myWriteCosts) {
    //             os.writeAttr(SUMO_ATTR_SAVINGS, myReplacedRoutes[index].route->getSavings());
    //         }
    //         // write edge on which the vehicle was when the route was valid
    //         os.writeAttr("replacedOnEdge", (myReplacedRoutes[index].edge ?
    //                                         myReplacedRoutes[index].edge->getID() : ""));
    //         if (myReplacedRoutes[index].lastRouteIndex > 0) {
    //             // do not write the default
    //             os.writeAttr(SUMO_ATTR_REPLACED_ON_INDEX, myReplacedRoutes[index].lastRouteIndex);
    //         }
    //         // write the reason for replacement
    //         os.writeAttr("reason", myReplacedRoutes[index].info);
    //
    //         // write the time at which the route was replaced
    //         os.writeAttr(SUMO_ATTR_REPLACED_AT_TIME, time2string(myReplacedRoutes[index].time));
    //         os.writeAttr(SUMO_ATTR_PROB, "0");
    //         OutputDevice_String edgesD;
    //         // always write the part that was actually driven and the rest of the current route that wasn't yet driven
    //         int start = 0;
    //         for (int i = routesToSkip; i < index; i++) {
    //             if (myReplacedRoutes[i].edge != nullptr) {
    //                 int end = myReplacedRoutes[i].lastRouteIndex;
    //                 myReplacedRoutes[i].route->writeEdgeIDs(edgesD, start, end, myWriteInternal, myHolder.getVClass());
    //             }
    //             start = myReplacedRoutes[i].newRouteIndex;
    //         }
    //         myReplacedRoutes[index].route->writeEdgeIDs(edgesD, start, -1, myWriteInternal, myHolder.getVClass());
    //         std::string edgesS = edgesD.getString();
    //         edgesS.pop_back(); // remove last ' '
    //         os.writeAttr(SUMO_ATTR_EDGES, edgesS);
    //         if (myRouteLength) {
    //             const bool includeInternalLengths = MSGlobals::gUsingInternalLanes && MSNet::getInstance()->hasInternalLinks();
    //             ConstMSRoutePtr route = myReplacedRoutes[index].route;
    //             const double routeLength = route->getDistanceBetween(myHolder.getDepartPos(), route->getEdges().back()->getLength(),
    //                                        route->begin(), route->end(), includeInternalLengths);
    //             os.writeAttr("routeLength", routeLength);
    //         }
    //     } else {
    //         if (myDUAStyle || myWriteCosts) {
    //             os.writeAttr(SUMO_ATTR_COST, myHolder.getRoute().getCosts());
    //         }
    //         if (myWriteCosts) {
    //             os.writeAttr(SUMO_ATTR_SAVINGS, myHolder.getRoute().getSavings());
    //         }
    //         OutputDevice_String edgesD;
    //         int numWritten = 0;
    //         int start = 0;
    //         if (myHolder.getNumberReroutes() > 0) {
    //             assert((int)myReplacedRoutes.size() <= myHolder.getNumberReroutes());
    //             for (int i = routesToSkip; i < (int)myReplacedRoutes.size(); i++) {
    //                 if (myReplacedRoutes[i].edge != nullptr) {
    //                     int end = myReplacedRoutes[i].lastRouteIndex;
    //                     numWritten += myReplacedRoutes[i].route->writeEdgeIDs(edgesD, start, end, myWriteInternal, myHolder.getVClass());
    //                 }
    //                 start = myReplacedRoutes[i].newRouteIndex;
    //             }
    //         }
    //         numWritten += myCurrentRoute->writeEdgeIDs(edgesD, start, -1, myWriteInternal, myHolder.getVClass());
    //         std::string edgesS = edgesD.getString();
    //         edgesS.pop_back(); // remove last ' '
    //         os.writeAttr(SUMO_ATTR_EDGES, edgesS);
    //
    //         if (mySaveExits) {
    //             std::vector<std::string> exits;
    //             for (SUMOTime t : myExits) {
    //                 exits.push_back(time2string(t));
    //             }
    //             assert(numWritten >= (int)myExits.size());
    //             std::vector<std::string> missing(numWritten - (int)myExits.size(), "-1");
    //             exits.insert(exits.end(), missing.begin(), missing.end());
    //             os.writeAttr(SUMO_ATTR_EXITTIMES, exits);
    //         }
    //     }
    //     os.closeTag();
    // }
    #[allow(unused_variables)]
    pub fn write_xml_route(&self, os: &mut OutputDevice, index: i32) {
        todo!("port MSDevice_Vehroutes::writeXMLRoute")
    }

    /// C++: `void MSDevice_Vehroutes::writeOutput(const bool) const` [const, private]
    /// Called on writing vehroutes output
    // void
    // MSDevice_Vehroutes::writeOutput(const bool hasArrived) const {
    //     const OptionsCont& oc = OptionsCont::getOptions();
    //     OutputDevice& routeOut = OutputDevice::getDeviceByOption("vehroute-output");
    //     OutputDevice_String stringOut(1);
    //     OutputDevice& od = mySorted && routeOut.isXML() ? stringOut : routeOut;
    //     SUMOVehicleParameter tmp = myHolder.getParameter();
    //     tmp.depart = myIntendedDepart ? myHolder.getParameter().depart : myHolder.getDeparture();
    //     if (!MSGlobals::gUseMesoSim) {
    //         if (tmp.wasSet(VEHPARS_DEPARTLANE_SET)) {
    //             tmp.departLaneProcedure = DepartLaneDefinition::GIVEN;
    //             tmp.departLane = myDepartLane;
    //         }
    //         if (tmp.wasSet(VEHPARS_DEPARTPOSLAT_SET)) {
    //             tmp.departPosLatProcedure = (tmp.departPosLatProcedure == DepartPosLatDefinition::RANDOM
    //                                          ? DepartPosLatDefinition::GIVEN_VEHROUTE
    //                                          : DepartPosLatDefinition::GIVEN);
    //             tmp.departPosLat = myDepartPosLat;
    //         }
    //     }
    //     if (tmp.wasSet(VEHPARS_DEPARTPOS_SET)) {
    //         tmp.departPosProcedure = ((tmp.departPosProcedure != DepartPosDefinition::GIVEN
    //                                    && tmp.departPosProcedure != DepartPosDefinition::STOP)
    //                                   ? DepartPosDefinition::GIVEN_VEHROUTE
    //                                   : DepartPosDefinition::GIVEN);
    //         tmp.departPos = myDepartPos;
    //     }
    //     if (tmp.wasSet(VEHPARS_DEPARTSPEED_SET)) {
    //         tmp.departSpeedProcedure = ((tmp.departSpeedProcedure != DepartSpeedDefinition::GIVEN
    //                                      && tmp.departSpeedProcedure != DepartSpeedDefinition::LIMIT)
    //                                     ? DepartSpeedDefinition::GIVEN_VEHROUTE
    //                                     : DepartSpeedDefinition::GIVEN);
    //         tmp.departSpeed = myDepartSpeed;
    //     }
    //     if (oc.getBool("vehroute-output.speedfactor") ||
    //             (oc.isDefault("vehroute-output.speedfactor") && tmp.wasSet(VEHPARS_DEPARTSPEED_SET))) {
    //         tmp.parametersSet |= VEHPARS_SPEEDFACTOR_SET;
    //         tmp.speedFactor = myHolder.getChosenSpeedFactor();
    //     }
    //
    //     const std::string typeID = myHolder.getVehicleType().getID() != DEFAULT_VTYPE_ID ? myHolder.getVehicleType().getID() : "";
    //     tmp.write(od, oc, SUMO_TAG_VEHICLE, typeID);
    //     if (hasArrived) {
    //         od.writeAttr("arrival", time2string(MSNet::getInstance()->getCurrentTimeStep()));
    //     }
    //     if (myRouteLength) {
    //         const bool includeInternalLengths = MSGlobals::gUsingInternalLanes && MSNet::getInstance()->hasInternalLinks();
    //         const double finalPos = hasArrived ? myHolder.getArrivalPos() : myHolder.getPositionOnLane();
    //         const double routeLength = myHolder.getRoute().getDistanceBetween(myHolder.getDepartPos(), finalPos,
    //                                    myHolder.getRoute().begin(), myHolder.getCurrentRouteEdge(), includeInternalLengths);
    //         od.writeAttr("routeLength", routeLength);
    //     }
    //     if (myDUAStyle) {
    //         const RandomDistributor<ConstMSRoutePtr>* const routeDist = MSRoute::distDictionary("!" + myHolder.getID());
    //         if (routeDist != nullptr) {
    //             const std::vector<ConstMSRoutePtr>& routes = routeDist->getVals();
    //             unsigned index = 0;
    //             while (index < routes.size() && routes[index] != myCurrentRoute) {
    //                 ++index;
    //             }
    //             od.openTag(SUMO_TAG_ROUTE_DISTRIBUTION).writeAttr(SUMO_ATTR_LAST, index);
    //             const std::vector<double>& probs = routeDist->getProbs();
    //             for (int i = 0; i < (int)routes.size(); ++i) {
    //                 od.setPrecision();
    //                 od.openTag(SUMO_TAG_ROUTE);
    //                 od.writeAttr(SUMO_ATTR_COST, routes[i]->getCosts());
    //                 if (myWriteCosts) {
    //                     od.writeAttr(SUMO_ATTR_SAVINGS, routes[i]->getSavings());
    //                 }
    //                 od.setPrecision(8);
    //                 od.writeAttr(SUMO_ATTR_PROB, probs[i]);
    //                 od.setPrecision();
    //                 OutputDevice_String edgesD;
    //                 routes[i]->writeEdgeIDs(edgesD, 0, -1, myWriteInternal, myHolder.getVClass());
    //                 std::string edgesS = edgesD.getString();
    //                 edgesS.pop_back(); // remove last ' '
    //                 od.writeAttr(SUMO_ATTR_EDGES, edgesS);
    //                 od.closeTag();
    //             }
    //             od.closeTag();
    //         } else {
    //             writeXMLRoute(od);
    //         }
    //     } else {
    //         std::string dummyMsg;
    //         const int routesToSkip = (myHolder.getParameter().wasSet(VEHPARS_FORCE_REROUTE)
    //                                   && !myIncludeIncomplete
    //                                   && myReplacedRoutes.size() > 0
    //                                   && !myReplacedRoutes[0].isValid ? 1 : 0);
    //         if ((int)myReplacedRoutes.size() > routesToSkip) {
    //             od.openTag(SUMO_TAG_ROUTE_DISTRIBUTION);
    //             for (int i = routesToSkip; i < (int)myReplacedRoutes.size(); ++i) {
    //                 writeXMLRoute(od, i);
    //             }
    //             writeXMLRoute(od);
    //             od.closeTag();
    //         } else {
    //             writeXMLRoute(od);
    //         }
    //     }
    //     if (routeOut.isXML()) {
    //         od << myStopOut.getString();
    //     }
    //     myHolder.getParameter().writeParams(od);
    //     od.closeTag();
    //     od.lf();
    //     if (mySorted && routeOut.isXML()) {
    //         // numerical id reflects loading order
    //         writeSortedOutput(&myRouteInfos, tmp.depart, myHolder.getNumericalID(), stringOut.getString());
    //     }
    // }
    #[allow(unused_variables)]
    pub fn write_output(&self, hasArrived: bool) {
        todo!("port MSDevice_Vehroutes::writeOutput")
    }

    /// C++: `void MSDevice_Vehroutes::addRoute(const std::string &)` [private]
    /// Called on route change
    // void
    // MSDevice_Vehroutes::addRoute(const std::string& info) {
    //     if (myMaxRoutes > 0) {
    //         //std::cout << SIMTIME << " " << getID() << " departed=" << myHolder.hasDeparted() << " lastIndex=" << myLastRouteIndex << " start=" << myHolder.getRoutePosition() << "\n";
    //         myReplacedRoutes.push_back(RouteReplaceInfo(
    //                                        myHolder.hasDeparted() ?  myHolder.getEdge() : nullptr,
    //                                        MSNet::getInstance()->getCurrentTimeStep(), myCurrentRoute, info,
    //                                        myLastRouteIndex,
    //                                        myHolder.hasDeparted() ? myHolder.getRoutePosition() : 0,
    //                                        myCurrentValid));
    //         if ((int)myReplacedRoutes.size() > myMaxRoutes) {
    //             myReplacedRoutes.erase(myReplacedRoutes.begin());
    //         }
    //     }
    //     std::string dummyMsg;
    //     myCurrentRoute = myHolder.getRoutePtr();
    //     myCurrentValid = myHolder.hasValidRoute(dummyMsg, myCurrentRoute);
    // }
    #[allow(unused_variables)]
    pub fn add_route(&mut self, info: &str) {
        todo!("port MSDevice_Vehroutes::addRoute")
    }

    /// C++: `void MSDevice_Vehroutes::MSDevice_Vehroutes(const MSDevice_Vehroutes &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSDevice_Vehroutes) -> Self {
        todo!("port MSDevice_Vehroutes::MSDevice_Vehroutes")
    }

    /// C++: `MSDevice_Vehroutes & MSDevice_Vehroutes::operator=(const MSDevice_Vehroutes &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSDevice_Vehroutes) -> &mut MSDevice_Vehroutes {
        todo!("port MSDevice_Vehroutes::operator=")
    }

}

/// Information about a replaced route
/// C++ class `MSDevice_Vehroutes::RouteReplaceInfo` (microsim/devices/MSDevice_Vehroutes.h)
pub struct MSDevice_Vehroutes_RouteReplaceInfo {
    /// The edge the vehicle was on when the route was replaced
    pub edge: Ptr<MSEdge>,
    /// The time the route was replaced
    pub time: SUMOTime,
    /// The prior route
    pub route: ConstMSRoutePtr,
    /// Information regarding rerouting
    pub info: String,
    /// The last index in the replaced route
    pub lastRouteIndex: i32,
    /// The current index in the replacement route
    pub newRouteIndex: i32,
    /// whether the current route is valid/complete
    pub isValid: bool,
}

impl MSDevice_Vehroutes_RouteReplaceInfo {
    /// C++: `void RouteReplaceInfo::RouteReplaceInfo(const MSEdge *const, const SUMOTime, const ConstMSRoutePtr, const std::string &, int, int, bool)`
    /// Constructor
    // RouteReplaceInfo(const MSEdge* const edge_, const SUMOTime time_, ConstMSRoutePtr const route_,
    //                          const std::string& info_, int lastRouteIndex_, int newRouteIndex_, bool isValid_) :
    //             edge(edge_), time(time_), route(route_), info(info_),
    //             lastRouteIndex(lastRouteIndex_),
    //             newRouteIndex(newRouteIndex_),
    //             isValid(isValid_)
    //         {}
    #[allow(unused_variables)]
    pub fn new(edge_: Ptr<MSEdge>, time_: SUMOTime, route_: ConstMSRoutePtr, info_: &str, lastRouteIndex_: i32, newRouteIndex_: i32, isValid_: bool) -> Self {
        todo!("port RouteReplaceInfo::RouteReplaceInfo")
    }

    /// C++: `void RouteReplaceInfo::~RouteReplaceInfo() noexcept`
    /// Destructor
    // ~RouteReplaceInfo() { }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port RouteReplaceInfo::~RouteReplaceInfo")
    }

}

/// Information needed to sort vehicle / transportable output by departure time
/// C++ class `MSDevice_Vehroutes::SortedRouteInfo` (microsim/devices/MSDevice_Vehroutes.h)
pub struct MSDevice_Vehroutes_SortedRouteInfo {
    /// route output device
    pub routeOut: Ptr<OutputDevice>,
    /// Map needed to sort vehicles by departure time
    pub departureCounts: i32,
    /// pregenerated route output sorted by time and creation
    pub routeXML: i32,
}

impl MSDevice_Vehroutes_SortedRouteInfo {
}

/// A class that is notified about reroutings
/// C++ class `MSDevice_Vehroutes::StateListener` (microsim/devices/MSDevice_Vehroutes.h)
pub struct MSDevice_Vehroutes_StateListener {
    /// C++ base class `MSNet::VehicleStateListener`
    pub base: MSNet_VehicleStateListener,
    /// A map for internal notification
    pub myDevices: i32,
}

impl std::ops::Deref for MSDevice_Vehroutes_StateListener {
    type Target = MSNet_VehicleStateListener;
    fn deref(&self) -> &MSNet_VehicleStateListener {
        &self.base
    }
}
impl std::ops::DerefMut for MSDevice_Vehroutes_StateListener {
    fn deref_mut(&mut self) -> &mut MSNet_VehicleStateListener {
        &mut self.base
    }
}

impl MSDevice_Vehroutes_StateListener {
    /// C++: `void StateListener::~StateListener() noexcept` [virtual]
    /// Destructor
    // ~StateListener() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port StateListener::~StateListener")
    }

    /// C++: `void StateListener::vehicleStateChanged(const SUMOVehicle *const, MSNet::VehicleState, const std::string &)` [virtual]
    /// Called if a vehicle changes its state
    // void
    // MSDevice_Vehroutes::StateListener::vehicleStateChanged(const SUMOVehicle* const vehicle, MSNet::VehicleState to, const std::string& info) {
    //     if (to == MSNet::VehicleState::NEWROUTE) {
    //         const auto& deviceEntry = myDevices.find(vehicle);
    //         if (deviceEntry != myDevices.end()) {
    //             deviceEntry->second->addRoute(info);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn vehicle_state_changed(&mut self, vehicle: Ptr<SUMOVehicle>, to: MSNet_VehicleState, info: &str) {
        todo!("port StateListener::vehicleStateChanged")
    }

}

