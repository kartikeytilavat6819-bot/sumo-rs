//! Port of microsim/devices/MSDevice_Vehroutes.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering::Relaxed};
use utils::common::sumo_vehicle_class::DEFAULT_VTYPE_ID;
use utils::common::std_defs::g_precision;
use utils::common::string_utils::to_long;
use utils::common::time::time2string;
use utils::common::to_string::to_string;
use utils::iodevices::output_device::OutputDeviceRef;
use utils::options::options_cont::{get_bool, is_default, is_set, with_options};
/// C++ `#define`d flags from utils/vehicle/SUMOVehicleParameter.h (not exported by the port yet)
const VEHPARS_DEPARTLANE_SET: i64 = 2 << 1;
const VEHPARS_DEPARTPOS_SET: i64 = 2 << 2;
const VEHPARS_DEPARTSPEED_SET: i64 = 2 << 3;
const VEHPARS_DEPARTPOSLAT_SET: i64 = 2 << 21;
const VEHPARS_SPEEDFACTOR_SET: i64 = 2 << 24;
const VEHPARS_FORCE_REROUTE: i64 = 2 << 17;
const STOP_ENDED_SET: i32 = 2 << 16;
const STOP_STARTED_SET: i32 = 2 << 17;

/// A device which collects info on the vehicle trip (mainly on departure and arrival)
/// C++ class `MSDevice_Vehroutes` (microsim/devices/MSDevice_Vehroutes.h)
// repr(C): the base sits at offset 0 so device pointers can be upcast to MSVehicleDevice (C++ implicit upcast)
#[repr(C)]
pub struct MSDevice_Vehroutes {
    /// C++ base class `MSVehicleDevice`
    pub base: MSVehicleDevice,
    /// The currently used route
    pub myCurrentRoute: ConstMSRoutePtr,
    /// Whether the currently used route is valid/complete
    pub myCurrentValid: bool,
    /// Prior routes
    pub myReplacedRoutes: Vec<MSDevice_Vehroutes_RouteReplaceInfo>,
    /// The times at which the vehicle exits an edge
    pub myExits: Vec<SUMOTime>,
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
    pub myPriorEdges: Vec<Ptr<MSEdge>>,
    /// the cost of the initial route if not set (cannot be stored in the route because it may be shared)
    pub myInitialCost: f64,
    pub myStopOut: OutputDevice,
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

// Plain bools: atomics keep them Sync; the single-threaded sim uses Relaxed.
static MY_SAVE_EXITS: AtomicBool = AtomicBool::new(false);
static MY_LAST_ROUTE_ONLY: AtomicBool = AtomicBool::new(false);
static MY_DUA_STYLE: AtomicBool = AtomicBool::new(false);
static MY_WRITE_COSTS: AtomicBool = AtomicBool::new(false);
static MY_SORTED: AtomicBool = AtomicBool::new(false);
static MY_INTENDED_DEPART: AtomicBool = AtomicBool::new(false);
static MY_ROUTE_LENGTH: AtomicBool = AtomicBool::new(false);
static MY_SKIP_PT_LINES: AtomicBool = AtomicBool::new(false);
static MY_INCLUDE_INCOMPLETE: AtomicBool = AtomicBool::new(false);
static MY_WRITE_STOP_PRIOR_EDGES: AtomicBool = AtomicBool::new(false);
static MY_WRITE_INTERNAL: AtomicBool = AtomicBool::new(false);

// Pointer-holding statics live per thread and are leaked once, so their address stays stable
// (C++ passes `&myStateListener` to MSNet and hands out `&myRouteInfos`).
thread_local! {
    static MY_STATE_LISTENER: std::cell::Cell<Ptr<MSDevice_Vehroutes_StateListener>> = std::cell::Cell::new(Ptr::default());
    static MY_ROUTE_INFOS: std::cell::Cell<Ptr<MSDevice_Vehroutes_SortedRouteInfo>> = std::cell::Cell::new(Ptr::default());
}

/// MSNet dispatches vehicle-state notifications by address (the base has no vtable): true for this listener
pub fn is_state_listener(listener: *mut MSNet_VehicleStateListener) -> bool {
    MY_STATE_LISTENER.with(|c| !c.get().is_null() && c.get().as_raw() as *mut MSNet_VehicleStateListener == listener)
}

/// the StateListener's vehicleStateChanged, reached from MSNet::informVehicleStateListener
pub fn state_listener_vehicle_state_changed(vehicle: Ptr<SUMOVehicle>, to: MSNet_VehicleState, info: &str) {
    my_state_listener().m().vehicle_state_changed(vehicle, to, info);
}

/// C++ static `myStateListener`
fn my_state_listener() -> Ptr<MSDevice_Vehroutes_StateListener> {
    MY_STATE_LISTENER.with(|c| {
        if c.get().is_null() {
            let l = MSDevice_Vehroutes_StateListener { base: MSNet_VehicleStateListener::new(), myDevices: BTreeMap::new() };
            c.set(Ptr::from_mut(Box::leak(Box::new(l))));
        }
        c.get()
    })
}

/// C++ static `myRouteInfos`
fn my_route_infos() -> Ptr<MSDevice_Vehroutes_SortedRouteInfo> {
    MY_ROUTE_INFOS.with(|c| {
        if c.get().is_null() {
            c.set(Ptr::from_mut(Box::leak(Box::new(MSDevice_Vehroutes_SortedRouteInfo::new()))));
        }
        c.get()
    })
}

/// The device registry keeps the Rc alive for the whole run, so the raw pointer stays valid (C++ `OutputDevice&`)
fn device_ptr(od: &OutputDeviceRef) -> Ptr<OutputDevice> {
    Ptr::from_raw(od.as_ptr())
}

/// C++ `MSNet::getInstance()->getCurrentTimeStep()` (SIMSTEP)
fn sim_step() -> SUMOTime {
    MSNet::get_instance().get_current_time_step()
}

/// C++ `istringstream >> x`: whitespace token parsed as T, default on failure
fn next_as<T: std::str::FromStr + Default>(bis: &mut std::str::SplitWhitespace) -> T {
    bis.next().and_then(|s| s.parse::<T>().ok()).unwrap_or_default()
}

/// C++ `istringstream >> string`
fn next_str(bis: &mut std::str::SplitWhitespace) -> String {
    bis.next().unwrap_or("").to_string()
}

/// Allocates a device for the lifetime of the run (C++ `new MSDevice_Vehroutes(...)`)
fn new_device(holder: &mut SUMOVehicle, id: &str, maxRoutes: i32) -> Ptr<MSDevice_Vehroutes> {
    Ptr::from_mut(Box::leak(Box::new(MSDevice_Vehroutes::new(holder, id, maxRoutes))))
}

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
        if is_set("vehroute-output") {
            OutputDevice::create_device_by_option("vehroute-output", "routes", "routes_file.xsd", -1).unwrap();
            let od = OutputDevice::get_device_by_option("vehroute-output").unwrap();
            if !od.borrow().is_xml() {
                write_warningf!("Vehroute output does not fully support tabular formats. Sorting and stop output will not work.");
            }
            MY_SAVE_EXITS.store(get_bool("vehroute-output.exit-times"), Relaxed);
            MY_LAST_ROUTE_ONLY.store(get_bool("vehroute-output.last-route"), Relaxed);
            MY_DUA_STYLE.store(get_bool("vehroute-output.dua"), Relaxed);
            MY_WRITE_COSTS.store(get_bool("vehroute-output.cost"), Relaxed);
            MY_SORTED.store(MY_DUA_STYLE.load(Relaxed) || get_bool("vehroute-output.sorted"), Relaxed);
            MY_INTENDED_DEPART.store(get_bool("vehroute-output.intended-depart"), Relaxed);
            MY_ROUTE_LENGTH.store(get_bool("vehroute-output.route-length"), Relaxed);
            MY_SKIP_PT_LINES.store(get_bool("vehroute-output.skip-ptlines"), Relaxed);
            MY_INCLUDE_INCOMPLETE.store(get_bool("vehroute-output.incomplete"), Relaxed);
            MY_WRITE_STOP_PRIOR_EDGES.store(get_bool("vehroute-output.stop-edges"), Relaxed);
            MY_WRITE_INTERNAL.store(get_bool("vehroute-output.internal"), Relaxed);
            MSNet::get_instance().m().add_vehicle_state_listener(Ptr::from_raw(my_state_listener().as_raw() as *mut MSNet_VehicleStateListener));
            my_route_infos().m().routeOut = device_ptr(&od);
        }
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
        oc.add_option_sub_topic("Vehroutes Device");
        MSDevice::insert_default_assignment_options("vehroute", "Vehroutes Device", oc, false);
    }

    /// C++: `MSDevice_Vehroutes * MSDevice_Vehroutes::buildVehicleDevices(SUMOVehicle &, int &, int)` [static]
    /// Build devices for the given vehicle, if needed
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn build_vehicle_devices(v: &mut SUMOVehicle, into: &mut Vec<Ptr<MSVehicleDevice>>, maxRoutes: i32) -> Ptr<MSDevice_Vehroutes> {
        let vp = Ptr::from_mut(v);
        let id = format!("vehroute_{}", vp.get_id());
        if maxRoutes < i32::MAX {
            return new_device(vp.m(), &id, maxRoutes);
        }
        if MY_SKIP_PT_LINES.load(Relaxed) && !vp.get_parameter().line.is_empty() {
            return Ptr::null();
        }
        let hasOption = is_set("vehroute-output");
        let vp2 = vp;
        let equipped = with_options(|oc| {
            MSDevice::equipped_by_default_assignment_options(
                oc,
                "vehroute",
                &vp2.get_id(),
                &vp2.get_parameter().base,
                &vp2.get_v_type_parameter().base,
                hasOption,
                false,
            )
        });
        if equipped {
            let mut maxRoutes = maxRoutes;
            if MY_LAST_ROUTE_ONLY.load(Relaxed) {
                maxRoutes = 0;
            }
            let dev = new_device(vp.m(), &id, maxRoutes);
            my_state_listener().m().myDevices.insert(vp.get_numerical_id(), (vp, dev));
            into.push(Ptr::from_raw(dev.as_raw() as *mut MSVehicleDevice));
            return dev;
        }
        Ptr::null()
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
        let net = MSNet::get_instance();

        if !includeUnfinished {
            if MY_SORTED.load(Relaxed) {
                for routeInfo in my_route_infos().m().routeXML.values() {
                    for rouXML in routeInfo.values() {
                        my_route_infos().routeOut.m().write(rouXML);
                    }
                }
                if net.has_persons() {
                    let personRouteInfos = net.m().get_person_control().get_route_info();
                    if personRouteInfos.routeOut != my_route_infos().routeOut {
                        for routeInfo in personRouteInfos.routeXML.values() {
                            for rouXML in routeInfo.values() {
                                personRouteInfos.routeOut.m().write(rouXML);
                            }
                        }
                    }
                }
            }
            return;
        }
        for (_, (vehicle, device)) in my_state_listener().myDevices.iter() {
            if vehicle.has_departed() {
                if vehicle.is_stopped() {
                    device.m().notify_stop_ended();
                }
                device.write_output(false);
            }
        }
        // unfinished persons
        if net.has_persons() {
            // shortcut: MSTransportableControl::loadedBegin/loadedEnd are Unmapped iterators, so the
            // loop that erases still-loaded persons is skipped; port it once the iterator is mapped
        }
        // Flush any remaining sorted outputs that may still be buffered
        if MY_SORTED.load(Relaxed) {
            for routeInfo in my_route_infos().m().routeXML.values() {
                for rouXML in routeInfo.values() {
                    my_route_infos().routeOut.m().write(rouXML);
                }
            }
            if net.has_persons() {
                let personRouteInfos = net.m().get_person_control().get_route_info();
                if personRouteInfos.routeOut != my_route_infos().routeOut {
                    for routeInfo in personRouteInfos.routeXML.values() {
                        for rouXML in routeInfo.values() {
                            personRouteInfos.routeOut.m().write(rouXML);
                        }
                    }
                }
            }
        }
    }

    /// C++: `void MSDevice_Vehroutes::~MSDevice_Vehroutes()` [virtual]
    /// Destructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        my_state_listener().m().myDevices.remove(&self.myHolder.get_numerical_id());
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
        let this = Ptr::from_mut(self);
        if matches!(reason, MSMoveReminder_Notification::NOTIFICATION_DEPARTED) {
            let key = veh.get_numerical_id();
            if MY_SORTED.load(Relaxed) && my_state_listener().myDevices.get(&key).is_some_and(|e| e.1 == this) {
                let departure = if MY_INTENDED_DEPART.load(Relaxed) { this.myHolder.get_parameter().depart } else { sim_step() };
                *my_route_infos().m().departureCounts.entry(departure).or_insert(0) += 1;
            }
            if !crate::microsim::ms_globals::use_meso_sim() {
                // C++ static_cast<MSVehicle&>(veh): the notified object is the holder, so its lane/lateral data are read from it
                this.m().myDepartLane = veh.get_lane().get_index();
                this.m().myDepartPosLat = this.myHolder.get_lateral_position_on_lane();
            }
            this.m().myDepartSpeed = veh.get_speed();
            this.m().myDepartPos = veh.get_position_on_lane();
        }
        if MY_WRITE_STOP_PRIOR_EDGES.load(Relaxed) {
            if crate::microsim::ms_globals::use_meso_sim() {
                let e = veh.get_edge();
                if this.myPriorEdges.is_empty() || *this.myPriorEdges.last().unwrap() != e {
                    this.m().myPriorEdges.push(e);
                }
            } else {
                this.m().myPriorEdges.push(Ptr::from_mut(enteredLane.get_edge()));
            }
        }
        this.m().myLastRouteIndex = this.myHolder.get_route_position();
        true
    }

    /// C++: `bool MSDevice_Vehroutes::notifyLeave(SUMOTrafficObject &, double, Notification, const MSLane *)` [virtual]
    /// Saves exit times if needed
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn notify_leave(&mut self, veh: &mut SUMOTrafficObject, lastPos: f64, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
        if MY_SAVE_EXITS.load(Relaxed)
            && !matches!(
                reason,
                MSMoveReminder_Notification::NOTIFICATION_LANE_CHANGE | MSMoveReminder_Notification::NOTIFICATION_PARKING | MSMoveReminder_Notification::NOTIFICATION_SEGMENT
            )
        {
            // C++ dynamic_cast<MSBaseVehicle&>(veh).getCurrentEdge() when writing internal edges
            let edge = if MY_WRITE_INTERNAL.load(Relaxed) { veh.get_current_edge() } else { veh.get_edge() };
            if self.myLastSavedAt != edge {
                self.myExits.push(sim_step());
                self.myLastSavedAt = edge;
            }
        }
        true
    }

    /// C++: `const std::string MSDevice_Vehroutes::deviceName() const` [const, virtual]
    /// return the name for this type of device
    // const std::string deviceName() const {
    //         return "vehroute";
    //     }
    #[allow(unused_variables)]
    pub fn device_name(&self) -> String {
        "vehroute".to_string()
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
        let mut stop = self.myHolder.get_stops().front().unwrap().pars.clone();
        let closeLater = MY_WRITE_STOP_PRIOR_EDGES.load(Relaxed) || MY_SAVE_EXITS.load(Relaxed);
        if MY_SAVE_EXITS.load(Relaxed) {
            // prevent duplicate output
            stop.parametersSet &= !(STOP_STARTED_SET | STOP_ENDED_SET);
        }
        stop.write(&mut self.myStopOut, !closeLater, true);
        if MY_WRITE_STOP_PRIOR_EDGES.load(Relaxed) {
            // calculate length
            let mut priorEdgesLength = 0.0;
            for i in 0..self.myPriorEdges.len() {
                if i == 0 {
                    priorEdgesLength += self.myPriorEdges[i].get_length();
                } else if self.myPriorEdges[i].get_id() != self.myPriorEdges[i - 1].get_id() {
                    priorEdgesLength += self.myPriorEdges[i].get_length();
                }
            }
            let priorEdgeIDs: Vec<String> = self.myPriorEdges.iter().map(|e| e.get_id().to_string()).collect();
            self.myStopOut.write_attr("priorEdges", &priorEdgeIDs);
            self.myPriorEdges.clear();
            self.myStopOut.write_attr("priorEdgesLength", &priorEdgesLength);
        }
        if MY_SAVE_EXITS.load(Relaxed) {
            self.myStopOut.write_attr(SUMO_ATTR_STARTED, &time2string(stop.started));
            let ended = if stop.ended < 0 { "-1".to_string() } else { time2string(stop.ended) };
            self.myStopOut.write_attr(SUMO_ATTR_ENDED, &ended);
        }
        if closeLater {
            self.myStopOut.close_tag("");
        }
    }

    /// C++: `void MSDevice_Vehroutes::generateOutput(OutputDevice *) const` [const, virtual]
    /// Called on writing vehroutes output
    // void
    // MSDevice_Vehroutes::generateOutput(OutputDevice* /*tripinfoOut*/) const {
    //     writeOutput(true);
    // }
    #[allow(unused_variables)]
    pub fn generate_output(&self, tripinfoOut: Ptr<OutputDevice>) {
        self.write_output(true);
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
    // Returns Option because C++ returns nullptr for an out-of-range index (ConstMSRoutePtr is a non-null Rc)
    pub fn get_route(&self, index: i32) -> Option<ConstMSRoutePtr> {
        if index >= 0 && (index as usize) < self.myReplacedRoutes.len() {
            Some(self.myReplacedRoutes[index as usize].route.clone())
        } else {
            None
        }
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
        out.open_tag_id(SUMO_TAG_DEVICE);
        out.write_attr(SUMO_ATTR_ID, self.base.base2.base.get_id());
        let mut internals: Vec<String> = Vec::new();
        if !crate::microsim::ms_globals::use_meso_sim() {
            internals.push(to_string(&self.myDepartLane));
            internals.push(to_string(&self.myDepartPosLat));
        }
        internals.push(to_string(&self.myDepartSpeed));
        internals.push(to_string(&self.myDepartPos));
        internals.push(to_string(&self.myInitialCost));
        internals.push(to_string(&self.myReplacedRoutes.len()));
        for r in &self.myReplacedRoutes {
            let replacedOnEdge = if r.edge.is_null() { "!NULL".to_string() } else { r.edge.get_id().to_string() };
            internals.push(replacedOnEdge);
            internals.push(to_string(&r.time));
            internals.push(r.route.get_id().to_string());
            internals.push(r.info.clone());
            internals.push(to_string(&r.lastRouteIndex));
            internals.push(to_string(&r.newRouteIndex));
            internals.push(to_string(&r.isValid));
        }
        // C++ toString(std::vector<std::string>) joins with " "
        out.write_attr(SUMO_ATTR_STATE, &internals.join(" "));
        if MY_SAVE_EXITS.load(Relaxed) && !self.myExits.is_empty() {
            let exits: Vec<String> = self.myExits.iter().map(|t| time2string(*t)).collect();
            out.write_attr(SUMO_ATTR_EXITTIMES, &exits);
            out.write_attr(SUMO_ATTR_EDGE, self.myLastSavedAt.get_id());
        }
        out.close_tag("");
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
        let state = attrs.get_string(SUMO_ATTR_STATE);
        let mut bis = state.split_whitespace();
        if !crate::microsim::ms_globals::use_meso_sim() {
            self.myDepartLane = next_as(&mut bis);
            self.myDepartPosLat = next_as(&mut bis);
        }
        self.myDepartSpeed = next_as(&mut bis);
        self.myDepartPos = next_as(&mut bis);
        self.myInitialCost = next_as(&mut bis);
        let size: i32 = next_as(&mut bis);
        for _ in 0..size {
            let edgeID = next_str(&mut bis);
            let time: SUMOTime = next_as(&mut bis);
            let routeID = next_str(&mut bis);
            let info = next_str(&mut bis);
            let lastIndex: i32 = next_as(&mut bis);
            let newIndex: i32 = next_as(&mut bis);
            // C++ `bis >> myCurrentValid` reads a 0/1 integer
            self.myCurrentValid = next_as::<i32>(&mut bis) != 0;

            let route: ConstMSRoutePtr = MSRoute::dictionary_3(&routeID, Ptr::null());
            if !crate::microsim::ms_route::is_null_route(&route) {
                let edge = MSEdge::dictionary_2(&edgeID);
                self.myReplacedRoutes.push(MSDevice_Vehroutes_RouteReplaceInfo::new(edge, time, route, &info, lastIndex, newIndex, self.myCurrentValid));
            }
        }
        if MY_SAVE_EXITS.load(Relaxed) && attrs.has_attribute(SUMO_ATTR_EXITTIMES) {
            let mut ok = true;
            let times = attrs.get::<Vec<String>>(SUMO_ATTR_EXITTIMES, "", &mut ok, true);
            for t in &times {
                self.myExits.push(to_long(t).unwrap());
            }
            if attrs.has_attribute(SUMO_ATTR_EDGE) {
                self.myLastSavedAt = MSEdge::dictionary_2(&attrs.get_string(SUMO_ATTR_EDGE));
            }
        }
        if self.myHolder.has_departed() {
            self.myLastRouteIndex = self.myHolder.get_route_position();
        }
    }

    /// C++: `void MSDevice_Vehroutes::registerTransportableDepart(SUMOTime)` [static]
    // void
    // MSDevice_Vehroutes::registerTransportableDepart(SUMOTime depart) {
    //     myRouteInfos.departureCounts[depart]++;
    // }
    #[allow(unused_variables)]
    pub fn register_transportable_depart(depart: SUMOTime) {
        *my_route_infos().m().departureCounts.entry(depart).or_insert(0) += 1;
    }

    /// C++: `void MSDevice_Vehroutes::writeSortedOutput(SortedRouteInfo *, SUMOTime, const SUMOTrafficObject::NumericalID, const std::string &)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn write_sorted_output(routeInfo: Ptr<MSDevice_Vehroutes_SortedRouteInfo>, depart: SUMOTime, id: SUMOTrafficObject_NumericalID, xmlOutput: &str) {
        let mut routeInfo = routeInfo;
        if routeInfo.routeOut == my_route_infos().routeOut {
            routeInfo = my_route_infos();
        }
        routeInfo.m().routeXML.entry(depart).or_default().insert(id, xmlOutput.to_string());
        *routeInfo.m().departureCounts.entry(depart).or_insert(0) -= 1;
        // flush leading entries whose departure count reached zero, in time order
        while let Some((&time, &count)) = routeInfo.departureCounts.iter().next() {
            if count != 0 {
                break;
            }
            let xmls = routeInfo.m().routeXML.remove(&time).unwrap_or_default();
            for rouXML in xmls.values() {
                routeInfo.routeOut.m().write(rouXML);
            }
            routeInfo.m().departureCounts.remove(&time);
        }
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
        fn mr_notify_enter(mr: &mut MSMoveReminder, veh: &mut SUMOTrafficObject, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
            Ptr::from_mut(mr).cast::<MSDevice_Vehroutes>().m().notify_enter(veh, reason, enteredLane)
        }
        fn mr_notify_leave(mr: &mut MSMoveReminder, veh: &mut SUMOTrafficObject, lastPos: f64, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
            Ptr::from_mut(mr).cast::<MSDevice_Vehroutes>().m().notify_leave(veh, lastPos, reason, enteredLane)
        }
        static MS_MOVE_REMINDER_VT: MSMoveReminderVt = MSMoveReminderVt {
            notify_enter: mr_notify_enter,
            notify_move: crate::microsim::ms_move_reminder::default_notify_move,
            notify_leave: mr_notify_leave,
        };
        fn mr_notify_stop_ended(mr: &mut MSMoveReminder) {
            Ptr::from_mut(mr).cast::<MSDevice_Vehroutes>().m().notify_stop_ended()
        }
        static MS_MOVE_REMINDER_IDLE_VT: MSMoveReminderIdleVt = MSMoveReminderIdleVt {
            notify_idle: crate::microsim::ms_move_reminder::default_notify_idle,
            notify_parking: crate::microsim::ms_move_reminder::default_notify_parking,
            notify_stop_ended: mr_notify_stop_ended,
        };
        let mut this = Self::new_device(holder, id, maxRoutes);
        this.base.base.vt = &MS_MOVE_REMINDER_VT;
        this.base.base.idle_vt = &MS_MOVE_REMINDER_IDLE_VT;
        this
    }

    fn new_device(holder: &mut SUMOVehicle, id: &str, maxRoutes: i32) -> Self {
        let currentRoute = holder.get_route_ptr();
        let mut this = MSDevice_Vehroutes {
            base: MSVehicleDevice::new(holder, id),
            myCurrentRoute: currentRoute,
            myCurrentValid: false,
            myReplacedRoutes: Vec::new(),
            myExits: Vec::new(),
            myMaxRoutes: maxRoutes,
            myLastSavedAt: Ptr::default(),
            myLastRouteIndex: -1,
            myDepartLane: -1,
            myDepartPos: -1.0,
            myDepartSpeed: -1.0,
            myDepartPosLat: 0.0,
            myPriorEdges: Vec::new(),
            myInitialCost: -1.0,
            myStopOut: OutputDevice::new_string(2),
        };
        let mut dummyMsg = String::new();
        this.myCurrentValid = this.myHolder.has_valid_route(&mut dummyMsg, this.myCurrentRoute.clone());
        if this.myCurrentRoute.get_costs() < 0.0 && MY_WRITE_COSTS.load(Relaxed) {
            this.myInitialCost = this.myHolder.get_router_tt().recompute_costs(this.myCurrentRoute.get_edges(), this.myHolder, sim_step(), Ptr::default());
        }
        this
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
        if index == 0
            && !MY_INCLUDE_INCOMPLETE.load(Relaxed)
            && self.myReplacedRoutes[0].route.size() == 2
            && self.myReplacedRoutes[0].route.get_edges().first().unwrap().is_taz_connector()
            && self.myReplacedRoutes[0].route.get_edges().last().unwrap().is_taz_connector()
        {
            return;
        }
        // check if a previous route shall be written
        let routesToSkip = if self.myHolder.get_parameter().was_set(VEHPARS_FORCE_REROUTE) { 1 } else { 0 };
        os.open_tag_id(SUMO_TAG_ROUTE);
        if index >= 0 {
            debug_assert!((self.myReplacedRoutes.len() as i32) > index);
            let iu = index as usize;
            if MY_DUA_STYLE.load(Relaxed) || MY_WRITE_COSTS.load(Relaxed) {
                let mut cost = self.myReplacedRoutes[iu].route.get_costs();
                if cost < 0.0 && index == 0 {
                    cost = self.myInitialCost;
                }
                os.write_attr(SUMO_ATTR_COST, &cost);
            }
            if MY_WRITE_COSTS.load(Relaxed) {
                os.write_attr(SUMO_ATTR_SAVINGS, &self.myReplacedRoutes[iu].route.get_savings());
            }
            // write edge on which the vehicle was when the route was valid
            let replacedOnEdge = if self.myReplacedRoutes[iu].edge.is_null() { "" } else { self.myReplacedRoutes[iu].edge.get_id() };
            os.write_attr("replacedOnEdge", replacedOnEdge);
            if self.myReplacedRoutes[iu].lastRouteIndex > 0 {
                // do not write the default
                os.write_attr(SUMO_ATTR_REPLACED_ON_INDEX, &self.myReplacedRoutes[iu].lastRouteIndex);
            }
            // write the reason for replacement
            os.write_attr("reason", self.myReplacedRoutes[iu].info.as_str());

            // write the time at which the route was replaced
            os.write_attr(SUMO_ATTR_REPLACED_AT_TIME, &time2string(self.myReplacedRoutes[iu].time));
            os.write_attr(SUMO_ATTR_PROB, "0");
            let mut edgesD = OutputDevice::new_string(0);
            // always write the part that was actually driven and the rest of the current route that wasn't yet driven
            let mut start = 0;
            for i in routesToSkip..index {
                let ri = i as usize;
                if !self.myReplacedRoutes[ri].edge.is_null() {
                    let end = self.myReplacedRoutes[ri].lastRouteIndex;
                    self.myReplacedRoutes[ri].route.write_edge_i_ds(&mut edgesD, start, end, MY_WRITE_INTERNAL.load(Relaxed), self.myHolder.get_v_class());
                }
                start = self.myReplacedRoutes[ri].newRouteIndex;
            }
            self.myReplacedRoutes[iu].route.write_edge_i_ds(&mut edgesD, start, -1, MY_WRITE_INTERNAL.load(Relaxed), self.myHolder.get_v_class());
            let mut edgesS = edgesD.get_string();
            edgesS.pop(); // remove last ' '
            os.write_attr(SUMO_ATTR_EDGES, &edgesS);
            if MY_ROUTE_LENGTH.load(Relaxed) {
                let includeInternalLengths = crate::microsim::ms_globals::using_internal_lanes() && MSNet::get_instance().has_internal_links();
                let route = self.myReplacedRoutes[iu].route.clone();
                let routeLength = route.get_distance_between_2(
                    self.myHolder.get_depart_pos(),
                    route.get_edges().last().unwrap().get_length(),
                    &route.begin(),
                    &route.end(),
                    includeInternalLengths,
                );
                os.write_attr("routeLength", &routeLength);
            }
        } else {
            if MY_DUA_STYLE.load(Relaxed) || MY_WRITE_COSTS.load(Relaxed) {
                os.write_attr(SUMO_ATTR_COST, &self.myHolder.get_route().get_costs());
            }
            if MY_WRITE_COSTS.load(Relaxed) {
                os.write_attr(SUMO_ATTR_SAVINGS, &self.myHolder.get_route().get_savings());
            }
            let mut edgesD = OutputDevice::new_string(0);
            let mut numWritten: i32 = 0;
            let mut start = 0;
            if self.myHolder.get_number_reroutes() > 0 {
                debug_assert!((self.myReplacedRoutes.len() as i32) <= self.myHolder.get_number_reroutes());
                for i in routesToSkip..(self.myReplacedRoutes.len() as i32) {
                    let ri = i as usize;
                    if !self.myReplacedRoutes[ri].edge.is_null() {
                        let end = self.myReplacedRoutes[ri].lastRouteIndex;
                        numWritten += self.myReplacedRoutes[ri].route.write_edge_i_ds(&mut edgesD, start, end, MY_WRITE_INTERNAL.load(Relaxed), self.myHolder.get_v_class());
                    }
                    start = self.myReplacedRoutes[ri].newRouteIndex;
                }
            }
            numWritten += self.myCurrentRoute.write_edge_i_ds(&mut edgesD, start, -1, MY_WRITE_INTERNAL.load(Relaxed), self.myHolder.get_v_class());
            let mut edgesS = edgesD.get_string();
            edgesS.pop(); // remove last ' '
            os.write_attr(SUMO_ATTR_EDGES, &edgesS);

            if MY_SAVE_EXITS.load(Relaxed) {
                let mut exits: Vec<String> = self.myExits.iter().map(|t| time2string(*t)).collect();
                debug_assert!(numWritten >= self.myExits.len() as i32);
                for _ in 0..(numWritten - self.myExits.len() as i32) {
                    exits.push("-1".to_string());
                }
                os.write_attr(SUMO_ATTR_EXITTIMES, &exits);
            }
        }
        os.close_tag("");
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
        let routeOutRef = OutputDevice::get_device_by_option("vehroute-output").unwrap();
        let routeOut = device_ptr(&routeOutRef);
        let mut stringOut = OutputDevice::new_string(1);
        let od: Ptr<OutputDevice> = if MY_SORTED.load(Relaxed) && routeOut.is_xml() { Ptr::from_mut(&mut stringOut) } else { routeOut };
        let mut tmp: SUMOVehicleParameter = self.myHolder.get_parameter().clone();
        tmp.depart = if MY_INTENDED_DEPART.load(Relaxed) { self.myHolder.get_parameter().depart } else { self.myHolder.get_departure() };
        if !crate::microsim::ms_globals::use_meso_sim() {
            if tmp.was_set(VEHPARS_DEPARTLANE_SET) {
                tmp.departLaneProcedure = DepartLaneDefinition::GIVEN;
                tmp.departLane = self.myDepartLane;
            }
            if tmp.was_set(VEHPARS_DEPARTPOSLAT_SET) {
                tmp.departPosLatProcedure = if matches!(tmp.departPosLatProcedure, DepartPosLatDefinition::RANDOM) {
                    DepartPosLatDefinition::GIVEN_VEHROUTE
                } else {
                    DepartPosLatDefinition::GIVEN
                };
                tmp.departPosLat = self.myDepartPosLat;
            }
        }
        if tmp.was_set(VEHPARS_DEPARTPOS_SET) {
            tmp.departPosProcedure = if !matches!(tmp.departPosProcedure, DepartPosDefinition::GIVEN | DepartPosDefinition::STOP) {
                DepartPosDefinition::GIVEN_VEHROUTE
            } else {
                DepartPosDefinition::GIVEN
            };
            tmp.departPos = self.myDepartPos;
        }
        if tmp.was_set(VEHPARS_DEPARTSPEED_SET) {
            tmp.departSpeedProcedure = if !matches!(tmp.departSpeedProcedure, DepartSpeedDefinition::GIVEN | DepartSpeedDefinition::LIMIT) {
                DepartSpeedDefinition::GIVEN_VEHROUTE
            } else {
                DepartSpeedDefinition::GIVEN
            };
            tmp.departSpeed = self.myDepartSpeed;
        }
        if get_bool("vehroute-output.speedfactor") || (is_default("vehroute-output.speedfactor") && tmp.was_set(VEHPARS_DEPARTSPEED_SET)) {
            tmp.parametersSet |= VEHPARS_SPEEDFACTOR_SET;
            tmp.speedFactor = self.myHolder.get_chosen_speed_factor();
        }

        let typeID = if self.myHolder.get_vehicle_type().get_id() != DEFAULT_VTYPE_ID { self.myHolder.get_vehicle_type().get_id().to_string() } else { String::new() };
        with_options(|oc| tmp.write(od.m(), oc, SUMO_TAG_VEHICLE, &typeID));
        if hasArrived {
            od.m().write_attr("arrival", &time2string(MSNet::get_instance().get_current_time_step()));
        }
        if MY_ROUTE_LENGTH.load(Relaxed) {
            let includeInternalLengths = crate::microsim::ms_globals::using_internal_lanes() && MSNet::get_instance().has_internal_links();
            let finalPos = if hasArrived { self.myHolder.get_arrival_pos() } else { self.myHolder.get_position_on_lane() };
            let route = self.myHolder.get_route();
            let routeLength = route.get_distance_between_2(
                self.myHolder.get_depart_pos(),
                finalPos,
                &route.begin(),
                self.myHolder.get_current_route_edge(),
                includeInternalLengths,
            );
            od.m().write_attr("routeLength", &routeLength);
        }
        if MY_DUA_STYLE.load(Relaxed) {
            let routeDist = MSRoute::dist_dictionary(&format!("!{}", self.myHolder.get_id()));
            if !routeDist.is_null() {
                let routes = routeDist.get_vals();
                let mut index = 0usize;
                while index < routes.len() && !Rc::ptr_eq(&routes[index], &self.myCurrentRoute) {
                    index += 1;
                }
                od.m().open_tag_id(SUMO_TAG_ROUTE_DISTRIBUTION).write_attr(SUMO_ATTR_LAST, &index);
                let probs = routeDist.get_probs();
                for i in 0..routes.len() {
                    od.m().set_precision(g_precision());
                    od.m().open_tag_id(SUMO_TAG_ROUTE);
                    od.m().write_attr(SUMO_ATTR_COST, &routes[i].get_costs());
                    if MY_WRITE_COSTS.load(Relaxed) {
                        od.m().write_attr(SUMO_ATTR_SAVINGS, &routes[i].get_savings());
                    }
                    od.m().set_precision(8);
                    od.m().write_attr(SUMO_ATTR_PROB, &probs[i]);
                    od.m().set_precision(g_precision());
                    let mut edgesD = OutputDevice::new_string(0);
                    routes[i].write_edge_i_ds(&mut edgesD, 0, -1, MY_WRITE_INTERNAL.load(Relaxed), self.myHolder.get_v_class());
                    let mut edgesS = edgesD.get_string();
                    edgesS.pop(); // remove last ' '
                    od.m().write_attr(SUMO_ATTR_EDGES, &edgesS);
                    od.m().close_tag("");
                }
                od.m().close_tag("");
            } else {
                self.write_xml_route(od.m(), -1);
            }
        } else {
            let routesToSkip = if self.myHolder.get_parameter().was_set(VEHPARS_FORCE_REROUTE)
                && !MY_INCLUDE_INCOMPLETE.load(Relaxed)
                && !self.myReplacedRoutes.is_empty()
                && !self.myReplacedRoutes[0].isValid
            {
                1
            } else {
                0
            };
            if (self.myReplacedRoutes.len() as i32) > routesToSkip {
                od.m().open_tag_id(SUMO_TAG_ROUTE_DISTRIBUTION);
                for i in routesToSkip..(self.myReplacedRoutes.len() as i32) {
                    self.write_xml_route(od.m(), i);
                }
                self.write_xml_route(od.m(), -1);
                od.m().close_tag("");
            } else {
                self.write_xml_route(od.m(), -1);
            }
        }
        if routeOut.is_xml() {
            od.m().write(&self.myStopOut.get_string());
        }
        self.myHolder.get_parameter().write_params(od.m());
        od.m().close_tag("");
        od.m().lf();
        if MY_SORTED.load(Relaxed) && routeOut.is_xml() {
            // numerical id reflects loading order
            MSDevice_Vehroutes::write_sorted_output(my_route_infos(), tmp.depart, self.myHolder.get_numerical_id(), &stringOut.get_string());
        }
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
        if self.myMaxRoutes > 0 {
            let edge = if self.myHolder.has_departed() { self.myHolder.get_edge() } else { Ptr::default() };
            let routeIndex = if self.myHolder.has_departed() { self.myHolder.get_route_position() } else { 0 };
            self.myReplacedRoutes.push(MSDevice_Vehroutes_RouteReplaceInfo::new(
                edge,
                sim_step(),
                self.myCurrentRoute.clone(),
                info,
                self.myLastRouteIndex,
                routeIndex,
                self.myCurrentValid,
            ));
            if (self.myReplacedRoutes.len() as i32) > self.myMaxRoutes {
                self.myReplacedRoutes.remove(0);
            }
        }
        let mut dummyMsg = String::new();
        self.myCurrentRoute = self.myHolder.get_route_ptr();
        self.myCurrentValid = self.myHolder.has_valid_route(&mut dummyMsg, self.myCurrentRoute.clone());
    }

    /// C++: `void MSDevice_Vehroutes::MSDevice_Vehroutes(const MSDevice_Vehroutes &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSDevice_Vehroutes) -> Self {
        // C++ declares this copy constructor private and never defines it: any use is a link error there
        panic!("MSDevice_Vehroutes is not copyable (C++ declares the copy constructor private and undefined)")
    }

    /// C++: `MSDevice_Vehroutes & MSDevice_Vehroutes::operator=(const MSDevice_Vehroutes &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSDevice_Vehroutes) -> &mut MSDevice_Vehroutes {
        // C++ declares the assignment operator private and never defines it
        panic!("MSDevice_Vehroutes is not assignable (C++ declares operator= private and undefined)")
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
        Self {
            edge: edge_,
            time: time_,
            route: route_,
            info: info_.to_string(),
            lastRouteIndex: lastRouteIndex_,
            newRouteIndex: newRouteIndex_,
            isValid: isValid_,
        }
    }

    /// C++: `void RouteReplaceInfo::~RouteReplaceInfo() noexcept`
    /// Destructor
    // ~RouteReplaceInfo() { }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // the C++ destructor body is empty; the Rc route and String are released by Rust's drop glue
    }

}

/// Information needed to sort vehicle / transportable output by departure time
/// C++ class `MSDevice_Vehroutes::SortedRouteInfo` (microsim/devices/MSDevice_Vehroutes.h)
pub struct MSDevice_Vehroutes_SortedRouteInfo {
    /// route output device
    pub routeOut: Ptr<OutputDevice>,
    /// Map needed to sort vehicles by departure time
    pub departureCounts: BTreeMap<SUMOTime, i32>,
    /// pregenerated route output sorted by time and creation
    pub routeXML: BTreeMap<SUMOTime, BTreeMap<SUMOTrafficObject_NumericalID, String>>,
}

impl MSDevice_Vehroutes_SortedRouteInfo {
    /// C++ default member initialisers: `routeOut = nullptr`, empty maps
    pub fn new() -> Self {
        Self { routeOut: Ptr::default(), departureCounts: BTreeMap::new(), routeXML: BTreeMap::new() }
    }
}

/// A class that is notified about reroutings
/// C++ class `MSDevice_Vehroutes::StateListener` (microsim/devices/MSDevice_Vehroutes.h)
// repr(C): the base sits at offset 0 so the listener can be passed as MSNet::VehicleStateListener
#[repr(C)]
pub struct MSDevice_Vehroutes_StateListener {
    /// C++ base class `MSNet::VehicleStateListener`
    pub base: MSNet_VehicleStateListener,
    /// A map for internal notification, keyed by numerical id (C++ ComparatorNumericalIdLess order)
    pub myDevices: BTreeMap<SUMOTrafficObject_NumericalID, (Ptr<SUMOVehicle>, Ptr<MSDevice_Vehroutes>)>,
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
        // the C++ destructor body is empty; the device map is released by Rust's drop glue
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
        if matches!(to, MSNet_VehicleState::NEWROUTE) {
            if let Some((_, device)) = self.myDevices.get(&vehicle.get_numerical_id()) {
                device.m().add_route(info);
            }
        }
    }

}

