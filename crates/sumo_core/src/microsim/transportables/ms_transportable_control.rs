//! Port of microsim/transportables/MSTransportableControl.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `int MSTransportableControl::TransportableVector`
pub type MSTransportableControl_TransportableVector = i32;

/// C++ typedef `int MSTransportableControl::constVehIt`
pub type MSTransportableControl_constVehIt = i32;

/// The class is used to handle transportables (persons and containers) who are not using a transportation system but are walking or waiting. This includes waiting for the arrival or departure time / the time the waiting is over.
/// C++ class `MSTransportableControl` (microsim/transportables/MSTransportableControl.h)
pub struct MSTransportableControl {
    /// all currently created transportables by id
    pub myTransportables: i32,
    /// Transportables waiting for departure
    pub myWaiting4Departure: i32,
    /// the lists of walking / stopping transportables
    pub myWaitingUntil: i32,
    /// the lists of waiting transportables
    pub myWaiting4Vehicle: i32,
    /// The number of build transportables
    pub myLoadedNumber: i32,
    /// The number of discarded transportables
    pub myDiscardedNumber: i32,
    /// The number of transportables within the network (build and inserted but not removed)
    pub myRunningNumber: i32,
    /// The number of jammed transportables
    pub myJammedNumber: i32,
    /// The number of transportables waiting for departure
    pub myWaitingForDepartureNumber: i32,
    /// The number of transportables waiting for vehicles
    pub myWaitingForVehicleNumber: i32,
    /// The number of transportables waiting for a specified time
    pub myWaitingUntilNumber: i32,
    /// The number of transportables currently in an access stage
    pub myAccessNumber: i32,
    /// The number of transportables that exited the simulation
    pub myEndedNumber: i32,
    /// The number of transportables that arrived at their destination
    pub myArrivedNumber: i32,
    /// The number of teleports due to long waits for a ride
    pub myTeleportsAbortWait: i32,
    /// The number of teleports due to wrong destination
    pub myTeleportsWrongDest: i32,
    /// whether a new transportable waiting for a vehicle has been added in the last step
    pub myHaveNewWaiting: bool,
    /// maximum transportable count
    pub myMaxTransportableNumber: i32,
    pub myMovementModel: Ptr<MSPModel>,
    pub myNonInteractingModel: Ptr<MSPModel>,
    /// Information needed to sort transportable output by departure time
    pub myRouteInfos: MSDevice_Vehroutes_SortedRouteInfo,
    /// The time until waiting for a ride is aborted
    pub myAbortWaitingTimeout: SUMOTime,
}

impl MSTransportableControl {
    /// C++: `void MSTransportableControl::MSTransportableControl(const bool)`
    /// Constructor
    // MSTransportableControl::MSTransportableControl(const bool isPerson):
    //     myLoadedNumber(0),
    //     myDiscardedNumber(0),
    //     myRunningNumber(0),
    //     myJammedNumber(0),
    //     myWaitingForDepartureNumber(0),
    //     myWaitingForVehicleNumber(0),
    //     myWaitingUntilNumber(0),
    //     myAccessNumber(0),
    //     myEndedNumber(0),
    //     myArrivedNumber(0),
    //     myTeleportsAbortWait(0),
    //     myTeleportsWrongDest(0),
    //     myHaveNewWaiting(false) {
    //     const OptionsCont& oc = OptionsCont::getOptions();
    //     MSNet* const net = MSNet::getInstance();
    //     myMovementModel = myNonInteractingModel = new MSPModel_NonInteracting(oc, net);
    //     if (isPerson) {
    //         const std::string& model = oc.getString("pedestrian.model");
    //         if (model == "striping") {
    //             myMovementModel = new MSPModel_Striping(oc, net);
    // #ifdef HAVE_JUPEDSIM_GRPC
    //         } else if (model == "jupedsim") {
    //             myMovementModel = new MSPModel_JuPedSim(oc, net);
    // #endif
    // #ifdef HAVE_JUPEDSIM_LEGACY
    //         } else if (model == "jupedsim_legacy") {
    //             myMovementModel = new MSPModel_JuPedSim_legacy(oc, net);
    // #endif
    //         } else if (model != "nonInteracting") {
    //             delete myNonInteractingModel;
    //             throw ProcessError(TLF("Unknown pedestrian model '%'", model));
    //         }
    //     }
    //     if (oc.isSet("vehroute-output")) {
    //         myRouteInfos.routeOut = &OutputDevice::getDeviceByOption("vehroute-output");
    //     }
    //     if (oc.isSet("personroute-output")) {
    //         OutputDevice::createDeviceByOption("personroute-output", "routes", "routes_file.xsd");
    //         myRouteInfos.routeOut = &OutputDevice::getDeviceByOption("personroute-output");
    //     }
    //     if (oc.isSet("personinfo-output")) {
    //         OutputDevice::createDeviceByOption("personinfo-output", "tripinfos", "tripinfo_file.xsd");
    //     }
    //     myAbortWaitingTimeout = string2time(oc.getString("time-to-teleport.ride"));
    //     myMaxTransportableNumber = isPerson ? oc.getInt("max-num-persons") : -1;
    // }
    #[allow(unused_variables)]
    pub fn new(isPerson: bool) -> Self {
        todo!("port MSTransportableControl::MSTransportableControl")
    }

    /// C++: `void MSTransportableControl::~MSTransportableControl()` [virtual]
    /// Destructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MSTransportableControl::~MSTransportableControl")
    }

    /// C++: `bool MSTransportableControl::add(MSTransportable *)`
    /// Adds a single transportable, returns false if an id clash occurred
    // bool
    // MSTransportableControl::add(MSTransportable* transportable) {
    //     const SUMOVehicleParameter& param = transportable->getParameter();
    //     if (myTransportables.find(param.id) == myTransportables.end()) {
    //         myTransportables[param.id] = transportable;
    //         const SUMOTime step = param.depart % DELTA_T == 0 ? param.depart : (param.depart / DELTA_T + 1) * DELTA_T;
    //         myWaiting4Departure[step].push_back(transportable);
    //         myLoadedNumber++;
    //         myWaitingForDepartureNumber++;
    //         return true;
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn add(&mut self, transportable: Ptr<MSTransportable>) -> bool {
        todo!("port MSTransportableControl::add")
    }

    /// C++: `MSTransportable * MSTransportableControl::get(const std::string &) const` [const]
    /// Returns the named transportable, if existing
    // MSTransportable*
    // MSTransportableControl::get(const std::string& id) const {
    //     std::map<std::string, MSTransportable*>::const_iterator i = myTransportables.find(id);
    //     if (i == myTransportables.end()) {
    //         return nullptr;
    //     }
    //     return (*i).second;
    // }
    #[allow(unused_variables)]
    pub fn get(&self, id: &str) -> Ptr<MSTransportable> {
        todo!("port MSTransportableControl::get")
    }

    /// C++: `void MSTransportableControl::erase(MSTransportable *)` [virtual]
    /// removes a single transportable
    // void
    // MSTransportableControl::erase(MSTransportable* transportable) {
    //     const OptionsCont& oc = OptionsCont::getOptions();
    //     if (oc.isSet("personinfo-output")) {
    //         transportable->tripInfoOutput(OutputDevice::getDeviceByOption("personinfo-output"));
    //     } else if (oc.isSet("tripinfo-output")) {
    //         transportable->tripInfoOutput(OutputDevice::getDeviceByOption("tripinfo-output"));
    //     } else if (oc.getBool("duration-log.statistics")) {
    //         // collecting statistics is a sideffect
    //         OutputDevice_String dev;
    //         transportable->tripInfoOutput(dev);
    //     }
    //     if (oc.isSet("vehroute-output") || oc.isSet("personroute-output")) {
    //         if (transportable->hasArrived() || oc.getBool("vehroute-output.write-unfinished")) {
    //             // @todo: adapt this if we ever introduce person-device.vehroute
    //             if (transportable->getBoolParam("has.vehroute.person-device", false, true)) {
    //                 if (oc.getBool("vehroute-output.sorted")) {
    //                     const SUMOTime departure = oc.getBool("vehroute-output.intended-depart") ? transportable->getParameter().depart : transportable->getDeparture();
    //                     OutputDevice_String od(1);
    //                     transportable->routeOutput(od, oc.getBool("vehroute-output.route-length"));
    //                     MSDevice_Vehroutes::writeSortedOutput(&myRouteInfos,
    //                                                           departure, transportable->getNumericalID(), od.getString());
    //                 } else {
    //                     transportable->routeOutput(*myRouteInfos.routeOut, oc.getBool("vehroute-output.route-length"));
    //                 }
    //             }
    //         }
    //     }
    //     const std::map<std::string, MSTransportable*>::iterator i = myTransportables.find(transportable->getID());
    //     if (i != myTransportables.end()) {
    //         if (i->second->hasDeparted()) {
    //             myRunningNumber--;
    //         }
    //         myEndedNumber++;
    //         MSNet::getInstance()->informTransportableStateListener(transportable,
    //                 transportable->isPerson() ? MSNet::TransportableState::PERSON_ARRIVED : MSNet::TransportableState::CONTAINER_ARRIVED);
    //         delete i->second;
    //         myTransportables.erase(i);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn erase(&mut self, transportable: Ptr<MSTransportable>) {
        todo!("port MSTransportableControl::erase")
    }

    /// C++: `void MSTransportableControl::eraseAll()`
    /// all transportables
    // void
    // MSTransportableControl::eraseAll() {
    //     // avoid taxi device having dangling references to erased customers
    //     MSDevice_Taxi::allCustomersErased();
    //     while (loadedBegin() != loadedEnd()) {
    //         erase(loadedBegin()->second);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn erase_all(&mut self) {
        todo!("port MSTransportableControl::eraseAll")
    }

    /// C++: `void MSTransportableControl::setWaitEnd(SUMOTime, MSTransportable *)`
    /// sets the arrival time for a waiting transportable
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn set_wait_end(&mut self, time: SUMOTime, transportable: Ptr<MSTransportable>) {
        todo!("port MSTransportableControl::setWaitEnd")
    }

    /// C++: `void MSTransportableControl::checkWaiting(MSNet *, const SUMOTime)`
    /// checks whether any transportables waiting time is over
    // void
    // MSTransportableControl::checkWaiting(MSNet* net, const SUMOTime time) {
    //     myHaveNewWaiting = false;
    //     while (myWaiting4Departure.find(time) != myWaiting4Departure.end()) {
    //         TransportableVector& transportables = myWaiting4Departure[time];
    //         // we cannot use an iterator here because there might be additions to the vector while proceeding
    //         for (auto it = transportables.begin(); it != transportables.end();) {
    //             MSTransportable* t = *it;
    //             if (myMaxTransportableNumber > 0 && myRunningNumber >= myMaxTransportableNumber) {
    //                 TransportableVector& nextStep = myWaiting4Departure[time + DELTA_T];
    //                 nextStep.insert(nextStep.begin(), transportables.begin(), transportables.end());
    //                 transportables.clear();
    //                 break;
    //             } else if (myMaxTransportableNumber == 0) {
    //                 erase(t);
    //                 it = transportables.erase(it);
    //                 continue;
    //             }
    //             it = transportables.erase(it);
    //             myWaitingForDepartureNumber--;
    //             const bool isPerson = t->isPerson();
    //             t->setDeparted(time);
    //             if (t->proceed(net, time)) {
    //                 myRunningNumber++;
    //                 MSNet::getInstance()->informTransportableStateListener(t,
    //                         isPerson ? MSNet::TransportableState::PERSON_DEPARTED : MSNet::TransportableState::CONTAINER_DEPARTED);
    //                 const OptionsCont& oc = OptionsCont::getOptions();
    //                 if (oc.getBool("vehroute-output.sorted")) {
    //                     const SUMOTime departure = oc.getBool("vehroute-output.intended-depart") ? t->getParameter().depart : time;
    //                     if (oc.isSet("personroute-output")) {
    //                         myRouteInfos.departureCounts[departure]++;
    //                     } else {
    //                         MSDevice_Vehroutes::registerTransportableDepart(departure);
    //                     }
    //                 }
    //             } else {
    //                 erase(t);
    //             }
    //         }
    //         myWaiting4Departure.erase(time);
    //     }
    //     while (myWaitingUntil.find(time) != myWaitingUntil.end()) {
    //         // make a copy because 0-duration stops might modify the vector
    //         const TransportableVector transportables = myWaitingUntil[time];
    //         myWaitingUntil.erase(time);
    //         for (MSTransportable* t : transportables) {
    //             myWaitingUntilNumber--;
    //             if (!t->proceed(net, time)) {
    //                 erase(t);
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn check_waiting(&mut self, net: Ptr<MSNet>, time: SUMOTime) {
        todo!("port MSTransportableControl::checkWaiting")
    }

    /// C++: `void MSTransportableControl::addWaiting(const MSEdge *, MSTransportable *)`
    /// adds a transportable to the list of transportables waiting for a vehicle on the specified edge
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn add_waiting(&mut self, edge: Ptr<MSEdge>, person: Ptr<MSTransportable>) {
        todo!("port MSTransportableControl::addWaiting")
    }

    /// C++: `void MSTransportableControl::forceDeparture()`
    /// register forced (traci) departure
    // void
    // MSTransportableControl::forceDeparture() {
    //     myRunningNumber++;
    // }
    #[allow(unused_variables)]
    pub fn force_departure(&mut self) {
        todo!("port MSTransportableControl::forceDeparture")
    }

    /// C++: `bool MSTransportableControl::hasAnyWaiting(const MSEdge *, SUMOVehicle *) const` [const]
    /// check whether any transportables are waiting for the given vehicle
    // bool
    // MSTransportableControl::hasAnyWaiting(const MSEdge* edge, SUMOVehicle* vehicle) const {
    //     const auto wait = myWaiting4Vehicle.find(edge);
    //     if (wait != myWaiting4Vehicle.end()) {
    //         for (const MSTransportable* t : wait->second) {
    //             if (t->isWaitingFor(vehicle)
    //                     && vehicle->allowsBoarding(t)
    //                     && vehicle->isStoppedInRange(t->getEdgePos(), MSGlobals::gStopTolerance, true)) {
    //                 return true;
    //             }
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn has_any_waiting(&self, edge: Ptr<MSEdge>, vehicle: Ptr<SUMOVehicle>) -> bool {
        todo!("port MSTransportableControl::hasAnyWaiting")
    }

    /// C++: `bool MSTransportableControl::loadAnyWaiting(const MSEdge *, SUMOVehicle *, SUMOTime &, SUMOTime &, MSTransportable *const)`
    /// load any applicable transportables Loads any person / container that is waiting on that edge for the given vehicle and removes them from myWaiting
    // bool
    // MSTransportableControl::loadAnyWaiting(const MSEdge* edge, SUMOVehicle* vehicle, SUMOTime& timeToLoadNext, SUMOTime& stopDuration, MSTransportable* const force) {
    //     bool ret = false;
    //     const auto wait = myWaiting4Vehicle.find(edge);
    //     if (wait != myWaiting4Vehicle.end()) {
    //         const SUMOTime currentTime = SIMSTEP;
    //         TransportableVector& transportables = wait->second;
    //         for (TransportableVector::iterator i = transportables.begin(); i != transportables.end();) {
    //             MSTransportable* const t = *i;
    //             if (t->isWaitingFor(vehicle) && (t == force ||
    //                                              (vehicle->allowsBoarding(t)
    //                                               && timeToLoadNext - DELTA_T <= currentTime
    //                                               && vehicle->isStoppedInRange(t->getEdgePos(), MSGlobals::gStopTolerance)))) {
    //                 edge->removeTransportable(t);
    //                 vehicle->addTransportable(t);
    //                 if (myAbortWaitingTimeout >= 0) {
    //                     t->setAbortWaiting(-1);
    //                 }
    //                 if (timeToLoadNext >= 0) { // meso does not have loading times
    //                     const SUMOTime loadingDuration = (SUMOTime)((double)vehicle->getVehicleType().getLoadingDuration(t->isPerson()) * t->getVehicleType().getBoardingFactor());
    //                     //update the time point at which the next transportable can be loaded on the vehicle
    //                     if (timeToLoadNext > currentTime - DELTA_T) {
    //                         timeToLoadNext += loadingDuration;
    //                     } else {
    //                         timeToLoadNext = currentTime + loadingDuration;
    //                     }
    //                 }
    //
    //                 static_cast<MSStageDriving*>(t->getCurrentStage())->setVehicle(vehicle);
    //                 if (t->getCurrentStage()->getOriginStop() != nullptr) {
    //                     t->getCurrentStage()->getOriginStop()->removeTransportable(*i);
    //                 }
    //                 i = transportables.erase(i);
    //                 myWaitingForVehicleNumber--;
    //                 ret = true;
    //             } else {
    //                 ++i;
    //             }
    //         }
    //         if (transportables.empty()) {
    //             myWaiting4Vehicle.erase(wait);
    //         }
    //         if (ret && timeToLoadNext >= 0) {
    //             //if the time a transportable needs to get loaded on the vehicle extends the duration of the stop of the vehicle extend
    //             //the duration by setting it to the loading duration of the transportable
    //             stopDuration = MAX2(stopDuration, timeToLoadNext - currentTime);
    //         }
    //     }
    //     return ret;
    // }
    #[allow(unused_variables)]
    pub fn load_any_waiting(&mut self, edge: Ptr<MSEdge>, vehicle: Ptr<SUMOVehicle>, timeToLoadNext: &mut SUMOTime, stopDuration: &mut SUMOTime, force: Ptr<MSTransportable>) -> bool {
        todo!("port MSTransportableControl::loadAnyWaiting")
    }

    /// C++: `bool MSTransportableControl::hasTransportables() const` [const]
    /// checks whether any transportable waits to finish her plan
    // bool
    // MSTransportableControl::hasTransportables() const {
    //     return !myTransportables.empty();
    // }
    #[allow(unused_variables)]
    pub fn has_transportables(&self) -> bool {
        todo!("port MSTransportableControl::hasTransportables")
    }

    /// C++: `bool MSTransportableControl::hasNonWaiting() const` [const]
    /// checks whether any transportable is still engaged in walking / stopping
    // bool
    // MSTransportableControl::hasNonWaiting() const {
    //     return !myWaiting4Departure.empty() || getMovingNumber() > 0 || myWaitingUntilNumber > 0 || myHaveNewWaiting;
    // }
    #[allow(unused_variables)]
    pub fn has_non_waiting(&self) -> bool {
        todo!("port MSTransportableControl::hasNonWaiting")
    }

    /// C++: `int MSTransportableControl::getActiveCount()`
    /// return the number of active transportable objects
    // int
    // MSTransportableControl::getActiveCount() {
    //     return (int)myWaiting4Departure.size() + myRunningNumber - myWaitingForVehicleNumber;
    // }
    #[allow(unused_variables)]
    pub fn get_active_count(&mut self) -> i32 {
        todo!("port MSTransportableControl::getActiveCount")
    }

    /// C++: `void MSTransportableControl::abortAnyWaitingForVehicle()`
    /// aborts the plan for any transportable that is still waiting for a ride
    // void
    // MSTransportableControl::abortAnyWaitingForVehicle() {
    //     for (const auto& it : myWaiting4Vehicle) {
    //         const MSEdge* edge = it.first;
    //         for (MSTransportable* const p : it.second) {
    //             edge->removeTransportable(p);
    //             MSStageDriving* stage = dynamic_cast<MSStageDriving*>(p->getCurrentStage());
    //             const std::string waitDescription = stage == nullptr ? "waiting" : stage->getWaitingDescription();
    //             WRITE_WARNING(p->getObjectType() + " '" + p->getID() + "' aborted " + waitDescription + ".");
    //             if (myAbortWaitingTimeout >= 0) {
    //                 p->setAbortWaiting(-1);
    //             }
    //             erase(p);
    //         }
    //     }
    //     myWaiting4Vehicle.clear();
    //     myWaitingForVehicleNumber = 0;
    // }
    #[allow(unused_variables)]
    pub fn abort_any_waiting_for_vehicle(&mut self) {
        todo!("port MSTransportableControl::abortAnyWaitingForVehicle")
    }

    /// C++: `void MSTransportableControl::abortWaitingForVehicle(MSTransportable *)`
    /// let the given transportable abort waiting for a vehicle (when removing stage via TraCI)
    // void
    // MSTransportableControl::abortWaitingForVehicle(MSTransportable* t) {
    //     const MSEdge* edge = t->getEdge();
    //     auto it = myWaiting4Vehicle.find(edge);
    //     if (it != myWaiting4Vehicle.end()) {
    //         TransportableVector& waiting = it->second;
    //         auto it2 = std::find(waiting.begin(), waiting.end(), t);
    //         if (it2 != waiting.end()) {
    //             if (myAbortWaitingTimeout >= 0) {
    //                 (*it2)->setAbortWaiting(-1);
    //             }
    //             waiting.erase(it2);
    //             myWaitingForVehicleNumber--;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn abort_waiting_for_vehicle(&mut self, t: Ptr<MSTransportable>) {
        todo!("port MSTransportableControl::abortWaitingForVehicle")
    }

    /// C++: `void MSTransportableControl::abortWaiting(MSTransportable *)`
    /// aborts waiting stage of transportable
    // void
    // MSTransportableControl::abortWaiting(MSTransportable* t) {
    //     for (std::map<SUMOTime, TransportableVector>::iterator it = myWaiting4Departure.begin(); it != myWaiting4Departure.end(); ++it) {
    //         TransportableVector& ts = it->second;
    //         TransportableVector::iterator it2 = std::find(ts.begin(), ts.end(), t);
    //         if (it2 != ts.end()) {
    //             ts.erase(it2);
    //         }
    //     }
    //     for (std::map<SUMOTime, TransportableVector>::iterator it = myWaitingUntil.begin(); it != myWaitingUntil.end(); ++it) {
    //         TransportableVector& ts = it->second;
    //         TransportableVector::iterator it2 = std::find(ts.begin(), ts.end(), t);
    //         if (it2 != ts.end()) {
    //             ts.erase(it2);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn abort_waiting(&mut self, t: Ptr<MSTransportable>) {
        todo!("port MSTransportableControl::abortWaiting")
    }

    /// C++: `MSTransportable * MSTransportableControl::buildPerson(const SUMOVehicleParameter *, MSVehicleType *, MSTransportable::MSTransportablePlan *, SumoRNG *) const` [const, virtual]
    /// Builds a new person
    // MSTransportable*
    // MSTransportableControl::buildPerson(const SUMOVehicleParameter* pars, MSVehicleType* vtype, MSTransportable::MSTransportablePlan* plan,
    //                                     SumoRNG* rng) const {
    //     const double speedFactor = vtype->computeChosenSpeedDeviation(pars->speedFactor, rng);
    //     return new MSPerson(pars, vtype, plan, speedFactor);
    // }
    #[allow(unused_variables)]
    pub fn build_person(&self, pars: Ptr<SUMOVehicleParameter>, vtype: Ptr<MSVehicleType>, plan: Ptr<MSTransportable_MSTransportablePlan>, rng: Ptr<SumoRNG>) -> Ptr<MSTransportable> {
        todo!("port MSTransportableControl::buildPerson")
    }

    /// C++: `MSTransportable * MSTransportableControl::buildContainer(const SUMOVehicleParameter *, MSVehicleType *, MSTransportable::MSTransportablePlan *) const` [const, virtual]
    /// Builds a new container
    // MSTransportable*
    // MSTransportableControl::buildContainer(const SUMOVehicleParameter* pars, MSVehicleType* vtype, MSTransportable::MSTransportablePlan* plan) const {
    //     return new MSTransportable(pars, vtype, plan, false);
    // }
    #[allow(unused_variables)]
    pub fn build_container(&self, pars: Ptr<SUMOVehicleParameter>, vtype: Ptr<MSVehicleType>, plan: Ptr<MSTransportable_MSTransportablePlan>) -> Ptr<MSTransportable> {
        todo!("port MSTransportableControl::buildContainer")
    }

    /// C++: `constVehIt MSTransportableControl::loadedBegin() const` [const]
    /// Returns the begin of the internal transportables map
    // constVehIt loadedBegin() const {
    //         return myTransportables.begin();
    //     }
    #[allow(unused_variables)]
    pub fn loaded_begin(&self) -> MSVehicleControl_constVehIt {
        todo!("port MSTransportableControl::loadedBegin")
    }

    /// C++: `constVehIt MSTransportableControl::loadedEnd() const` [const]
    /// Returns the end of the internal transportables map
    // constVehIt loadedEnd() const {
    //         return myTransportables.end();
    //     }
    #[allow(unused_variables)]
    pub fn loaded_end(&self) -> MSVehicleControl_constVehIt {
        todo!("port MSTransportableControl::loadedEnd")
    }

    /// C++: `int MSTransportableControl::size() const` [const]
    /// Returns the number of known transportables
    // int size() const {
    //         return (int)myTransportables.size();
    //     }
    #[allow(unused_variables)]
    pub fn size(&self) -> i32 {
        todo!("port MSTransportableControl::size")
    }

    /// C++: `void MSTransportableControl::registerJammed()`
    /// register a jammed transportable
    // void registerJammed() {
    //         myJammedNumber++;
    //     }
    #[allow(unused_variables)]
    pub fn register_jammed(&mut self) {
        todo!("port MSTransportableControl::registerJammed")
    }

    /// C++: `void MSTransportableControl::registerTeleportAbortWait()`
    /// register a teleport after aborting a long wait
    // void registerTeleportAbortWait() {
    //         myTeleportsAbortWait++;
    //     }
    #[allow(unused_variables)]
    pub fn register_teleport_abort_wait(&mut self) {
        todo!("port MSTransportableControl::registerTeleportAbortWait")
    }

    /// C++: `void MSTransportableControl::registerTeleportWrongDest()`
    /// register a teleport to the final destination
    // void registerTeleportWrongDest() {
    //         myTeleportsWrongDest++;
    //     }
    #[allow(unused_variables)]
    pub fn register_teleport_wrong_dest(&mut self) {
        todo!("port MSTransportableControl::registerTeleportWrongDest")
    }

    /// C++: `void MSTransportableControl::fixLoadCount(const MSTransportable *)`
    /// decrement counter to avoid double counting transportables loaded from state
    // void
    // MSTransportableControl::fixLoadCount(const MSTransportable* transportable) {
    //     myLoadedNumber--;
    //     if (transportable->hasDeparted()) {
    //         const SUMOVehicleParameter& param = transportable->getParameter();
    //         const SUMOTime step = param.depart % DELTA_T == 0 ? param.depart : (param.depart / DELTA_T + 1) * DELTA_T;
    //         TransportableVector& waiting = myWaiting4Departure[step];
    //         auto it = std::find(waiting.begin(), waiting.end(), transportable);
    //         if (it != waiting.end()) {
    //             waiting.erase(it);
    //             if (waiting.size() == 0) {
    //                 myWaiting4Departure.erase(step);
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn fix_load_count(&mut self, transportable: Ptr<MSTransportable>) {
        todo!("port MSTransportableControl::fixLoadCount")
    }

    /// C++: `int MSTransportableControl::getLoadedNumber() const` [const]
    /// Returns the number of build transportables
    // int getLoadedNumber() const {
    //         return myLoadedNumber;
    //     }
    #[allow(unused_variables)]
    pub fn get_loaded_number(&self) -> i32 {
        todo!("port MSTransportableControl::getLoadedNumber")
    }

    /// C++: `int MSTransportableControl::getDepartedNumber() const` [const]
    // int
    // MSTransportableControl::getDepartedNumber() const {
    //     return myLoadedNumber - myWaitingForDepartureNumber - myDiscardedNumber;
    // }
    #[allow(unused_variables)]
    pub fn get_departed_number(&self) -> i32 {
        todo!("port MSTransportableControl::getDepartedNumber")
    }

    /// C++: `int MSTransportableControl::getRunningNumber() const` [const]
    /// Returns the number of build and inserted, but not yet deleted transportables
    // int getRunningNumber() const {
    //         return myRunningNumber;
    //     }
    #[allow(unused_variables)]
    pub fn get_running_number(&self) -> i32 {
        todo!("port MSTransportableControl::getRunningNumber")
    }

    /// C++: `int MSTransportableControl::getJammedNumber() const` [const]
    /// Returns the number of times a transportables was jammed
    // int getJammedNumber() const {
    //         return myJammedNumber;
    //     }
    #[allow(unused_variables)]
    pub fn get_jammed_number(&self) -> i32 {
        todo!("port MSTransportableControl::getJammedNumber")
    }

    /// C++: `int MSTransportableControl::getWaitingForVehicleNumber() const` [const]
    /// Returns the number of transportables waiting for a ride
    // int getWaitingForVehicleNumber() const {
    //         return myWaitingForVehicleNumber;
    //     }
    #[allow(unused_variables)]
    pub fn get_waiting_for_vehicle_number(&self) -> i32 {
        todo!("port MSTransportableControl::getWaitingForVehicleNumber")
    }

    /// C++: `int MSTransportableControl::getWaitingUntilNumber() const` [const]
    /// Returns the number of transportables waiting for a specified amount of time
    // int getWaitingUntilNumber() const {
    //         return myWaitingUntilNumber;
    //     }
    #[allow(unused_variables)]
    pub fn get_waiting_until_number(&self) -> i32 {
        todo!("port MSTransportableControl::getWaitingUntilNumber")
    }

    /// C++: `int MSTransportableControl::getMovingNumber() const` [const]
    /// Returns the number of transportables moving by themselvs (i.e. walking)
    // int
    // MSTransportableControl::getMovingNumber() const {
    //     return myMovementModel->getActiveNumber() + myAccessNumber;
    // }
    #[allow(unused_variables)]
    pub fn get_moving_number(&self) -> i32 {
        todo!("port MSTransportableControl::getMovingNumber")
    }

    /// C++: `int MSTransportableControl::getRidingNumber() const` [const]
    /// Returns the number of transportables riding a vehicle
    // int
    // MSTransportableControl::getRidingNumber() const {
    //     return myRunningNumber - myWaitingUntilNumber - myWaitingForVehicleNumber - getMovingNumber();
    // }
    #[allow(unused_variables)]
    pub fn get_riding_number(&self) -> i32 {
        todo!("port MSTransportableControl::getRidingNumber")
    }

    /// C++: `int MSTransportableControl::getEndedNumber() const` [const]
    /// Returns the number of transportables that exited the simulation
    // int getEndedNumber() const {
    //         return myEndedNumber;
    //     }
    #[allow(unused_variables)]
    pub fn get_ended_number(&self) -> i32 {
        todo!("port MSTransportableControl::getEndedNumber")
    }

    /// C++: `int MSTransportableControl::getArrivedNumber() const` [const]
    /// Returns the number of transportables that arrived at their destination
    // int getArrivedNumber() const {
    //         return myArrivedNumber;
    //     }
    #[allow(unused_variables)]
    pub fn get_arrived_number(&self) -> i32 {
        todo!("port MSTransportableControl::getArrivedNumber")
    }

    /// C++: `int MSTransportableControl::getDiscardedNumber() const` [const]
    /// Returns the number of discarded transportables
    // int getDiscardedNumber() const {
    //         return myDiscardedNumber;
    //     }
    #[allow(unused_variables)]
    pub fn get_discarded_number(&self) -> i32 {
        todo!("port MSTransportableControl::getDiscardedNumber")
    }

    /// C++: `int MSTransportableControl::getTeleportsAbortWait() const` [const]
    /// return the number of teleports due to excessive waiting for a ride
    // int getTeleportsAbortWait() const {
    //         return myTeleportsAbortWait;
    //     }
    #[allow(unused_variables)]
    pub fn get_teleports_abort_wait(&self) -> i32 {
        todo!("port MSTransportableControl::getTeleportsAbortWait")
    }

    /// C++: `int MSTransportableControl::getTeleportsWrongDest() const` [const]
    /// return the number of teleports of transportables riding to the wrong destination
    // int getTeleportsWrongDest() const {
    //         return myTeleportsWrongDest;
    //     }
    #[allow(unused_variables)]
    pub fn get_teleports_wrong_dest(&self) -> i32 {
        todo!("port MSTransportableControl::getTeleportsWrongDest")
    }

    /// C++: `int MSTransportableControl::getTeleportCount() const` [const]
    /// Returns the number of teleports transportables did
    // int getTeleportCount() const {
    //         return myTeleportsAbortWait + myTeleportsWrongDest;
    //     }
    #[allow(unused_variables)]
    pub fn get_teleport_count(&self) -> i32 {
        todo!("port MSTransportableControl::getTeleportCount")
    }

    /// C++: `MSPModel * MSTransportableControl::getMovementModel()`
    /// Returns the default movement model for this kind of transportables
    // inline MSPModel* getMovementModel() {
    //         return myMovementModel;
    //     }
    #[allow(unused_variables)]
    pub fn get_movement_model(&mut self) -> Ptr<MSPModel> {
        todo!("port MSTransportableControl::getMovementModel")
    }

    /// C++: `MSPModel * MSTransportableControl::getNonInteractingModel()`
    /// Returns the non interacting movement model (for tranship and "beaming")
    // inline MSPModel* getNonInteractingModel() {
    //         return myNonInteractingModel;
    //     }
    #[allow(unused_variables)]
    pub fn get_non_interacting_model(&mut self) -> Ptr<MSPModel> {
        todo!("port MSTransportableControl::getNonInteractingModel")
    }

    /// C++: `void MSTransportableControl::addArrived()`
    // void addArrived() {
    //         myArrivedNumber++;
    //     }
    #[allow(unused_variables)]
    pub fn add_arrived(&mut self) {
        todo!("port MSTransportableControl::addArrived")
    }

    /// C++: `void MSTransportableControl::addDiscarded()`
    // void addDiscarded() {
    //         myLoadedNumber++;
    //         myDiscardedNumber++;
    //     }
    #[allow(unused_variables)]
    pub fn add_discarded(&mut self) {
        todo!("port MSTransportableControl::addDiscarded")
    }

    /// C++: `void MSTransportableControl::startedAccess()`
    // void startedAccess() {
    //         myAccessNumber++;
    //     }
    #[allow(unused_variables)]
    pub fn started_access(&mut self) {
        todo!("port MSTransportableControl::startedAccess")
    }

    /// C++: `void MSTransportableControl::endedAccess()`
    // void endedAccess() {
    //         myAccessNumber--;
    //     }
    #[allow(unused_variables)]
    pub fn ended_access(&mut self) {
        todo!("port MSTransportableControl::endedAccess")
    }

    /// C++: `void MSTransportableControl::saveState(OutputDevice &)`
    /// Saves the current state into the given stream
    // void
    // MSTransportableControl::saveState(OutputDevice& out) {
    //     std::ostringstream oss;
    //     oss << myRunningNumber << " " << myLoadedNumber << " " << myEndedNumber << " " << myWaitingForDepartureNumber << " " << myArrivedNumber << " " << myDiscardedNumber;
    //     oss << " " << myJammedNumber << " " << myWaitingForVehicleNumber << " " << myHaveNewWaiting;
    //     out.writeAttr(SUMO_ATTR_STATE, oss.str());
    //     for (const auto& it : myTransportables) {
    //         it.second->saveState(out);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn save_state(&mut self, out: &mut OutputDevice) {
        todo!("port MSTransportableControl::saveState")
    }

    /// C++: `void MSTransportableControl::loadState(const std::string &)`
    /// Reconstruct the current state
    // void
    // MSTransportableControl::loadState(const std::string& state) {
    //     std::istringstream iss(state);
    //     iss >> myRunningNumber >> myLoadedNumber >> myEndedNumber >> myWaitingForDepartureNumber >> myArrivedNumber >> myDiscardedNumber;
    //     iss >> myJammedNumber >> myWaitingForVehicleNumber >> myHaveNewWaiting;
    // }
    #[allow(unused_variables)]
    pub fn load_state(&mut self, state: &str) {
        todo!("port MSTransportableControl::loadState")
    }

    /// C++: `void MSTransportableControl::clearState()`
    /// Resets transportables when quick-loading state
    // void
    // MSTransportableControl::clearState() {
    //     for (std::map<std::string, MSTransportable*>::iterator i = myTransportables.begin(); i != myTransportables.end(); ++i) {
    //         delete (*i).second;
    //     }
    //     myTransportables.clear();
    //     myWaiting4Vehicle.clear();
    //     myWaiting4Departure.clear();
    //     myWaitingUntil.clear();
    //     myLoadedNumber = 0;
    //     myDiscardedNumber = 0;
    //     myRunningNumber = 0;
    //     myJammedNumber = 0;
    //     myWaitingForDepartureNumber = 0;
    //     myWaitingForVehicleNumber = 0;
    //     myWaitingUntilNumber = 0;
    //     myEndedNumber = 0;
    //     myArrivedNumber = 0;
    //     myHaveNewWaiting = false;
    //     if (myMovementModel != myNonInteractingModel) {
    //         myMovementModel->clearState();
    //     }
    //     myNonInteractingModel->clearState();
    // }
    #[allow(unused_variables)]
    pub fn clear_state(&mut self) {
        todo!("port MSTransportableControl::clearState")
    }

    /// C++: `const MSDevice_Vehroutes::SortedRouteInfo & MSTransportableControl::getRouteInfo()`
    // const MSDevice_Vehroutes::SortedRouteInfo& getRouteInfo() {
    //         return myRouteInfos;
    //     }
    #[allow(unused_variables)]
    pub fn get_route_info(&mut self) -> &MSDevice_Vehroutes_SortedRouteInfo {
        todo!("port MSTransportableControl::getRouteInfo")
    }

    /// C++: `MSTransportableControl & MSTransportableControl::operator=(const MSTransportableControl &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, src: &MSTransportableControl) -> &mut MSTransportableControl {
        todo!("port MSTransportableControl::operator=")
    }

}

