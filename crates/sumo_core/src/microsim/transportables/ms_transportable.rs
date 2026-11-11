//! Port of microsim/transportables/MSTransportable.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `std::vector<MSStage *> MSTransportable::MSTransportablePlan`
pub type MSTransportable_MSTransportablePlan = Vec<Ptr<MSStage>>;

/// The class holds a simulated moveable object
/// C++ class `MSTransportable` (microsim/transportables/MSTransportable.h)
pub struct MSTransportable {
    /// C++ base class `SUMOTrafficObject`
    pub base: SUMOTrafficObject,
    /// the plan of the transportable
    pub myParameter: Ptr<SUMOVehicleParameter>,
    /// This transportable's type. (mainly used for drawing related information Note sure if it is really necessary
    pub myVType: Ptr<MSVehicleType>,
    /// Whether events shall be written
    pub myWriteEvents: bool,
    /// the plan of the transportable
    pub myPlan: Ptr<MSTransportable_MSTransportablePlan>,
    /// the iterator over the route
    pub myStep: Unmapped /* MSTransportablePlan::iterator */,
    /// The devices this transportable has
    pub myDevices: Vec<Ptr<MSTransportableDevice>>,
    pub myAmPerson: bool,
    pub myNumericalID: SUMOTrafficObject_NumericalID,
    pub myRandomSeed: i64,
    pub myAbortCommand: Ptr<Unmapped /* WrappingCommand<MSTransportable> */>,
}

impl std::ops::Deref for MSTransportable {
    type Target = SUMOTrafficObject;
    fn deref(&self) -> &SUMOTrafficObject {
        &self.base
    }
}
impl std::ops::DerefMut for MSTransportable {
    fn deref_mut(&mut self) -> &mut SUMOTrafficObject {
        &mut self.base
    }
}

// C++ static members of MSTransportable (port as module-level statics, see AGENTS.md):
//   static NumericalID MSTransportable::myCurrentNumericalIndex;

impl MSTransportable {
    /// C++: `bool MSTransportable::isPerson() const` [const, virtual]
    /// @{
    // inline bool isPerson() const override {
    //         return myAmPerson;
    //     }
    #[allow(unused_variables)]
    pub fn is_person(&self) -> bool {
        todo!("port MSTransportable::isPerson")
    }

    /// C++: `bool MSTransportable::isContainer() const` [const, virtual]
    // inline bool isContainer() const override {
    //         return !myAmPerson;
    //     }
    #[allow(unused_variables)]
    pub fn is_container(&self) -> bool {
        todo!("port MSTransportable::isContainer")
    }

    /// C++: `std::string MSTransportable::getObjectType()`
    // inline std::string getObjectType() {
    //         return myAmPerson ? "Person" : "Container";
    //     }
    #[allow(unused_variables)]
    pub fn get_object_type(&mut self) -> String {
        todo!("port MSTransportable::getObjectType")
    }

    /// C++: `NumericalID MSTransportable::getNumericalID() const` [const, virtual]
    // inline NumericalID getNumericalID() const override {
    //         return myNumericalID;
    //     }
    #[allow(unused_variables)]
    pub fn get_numerical_id(&self) -> SUMOTrafficObject_NumericalID {
        todo!("port MSTransportable::getNumericalID")
    }

    /// C++: `long long MSTransportable::getRandomSeed() const` [const, virtual]
    /// return transportable-specific random number
    // long long int getRandomSeed() const override {
    //         return myRandomSeed;
    //     }
    #[allow(unused_variables)]
    pub fn get_random_seed(&self) -> i64 {
        todo!("port MSTransportable::getRandomSeed")
    }

    /// C++: `bool MSTransportable::isStopped() const` [const, virtual]
    // inline bool isStopped() const override {
    //         return getCurrentStageType() == MSStageType::WAITING;
    //     }
    #[allow(unused_variables)]
    pub fn is_stopped(&self) -> bool {
        todo!("port MSTransportable::isStopped")
    }

    /// C++: `double MSTransportable::getSlope() const` [const, virtual]
    // double
    // MSTransportable::getSlope() const {
    //     const MSEdge* edge = getEdge();
    //     const double ep = getEdgePos();
    //     const double gp = edge->getLanes()[0]->interpolateLanePosToGeometryPos(ep);
    //     return edge->getLanes()[0]->getShape().slopeDegreeAtOffset(gp);
    // }
    #[allow(unused_variables)]
    pub fn get_slope(&self) -> f64 {
        todo!("port MSTransportable::getSlope")
    }

    /// C++: `SUMOVehicleClass MSTransportable::getVClass() const` [const, virtual]
    // SUMOVehicleClass
    // MSTransportable::getVClass() const {
    //     return getVehicleType().getVehicleClass();
    // }
    #[allow(unused_variables)]
    pub fn get_v_class(&self) -> SUMOVehicleClass {
        todo!("port MSTransportable::getVClass")
    }

    /// C++: `bool MSTransportable::isJammed() const` [const, virtual]
    /// whether the transportable (persons) is jammed as defined by the current pedestrian model
    // virtual bool isJammed() const {
    //         return false;
    //     }
    #[allow(unused_variables)]
    pub fn is_jammed(&self) -> bool {
        todo!("port MSTransportable::isJammed")
    }

    /// C++: `double MSTransportable::getMaxSpeed() const` [const, virtual]
    /// Returns the maximum speed (the minimum of desired and physical maximum speed)
    // double
    // MSTransportable::getMaxSpeed() const {
    //     return MIN2(getVehicleType().getMaxSpeed(), getVehicleType().getDesiredMaxSpeed() * getChosenSpeedFactor());
    // }
    #[allow(unused_variables)]
    pub fn get_max_speed(&self) -> f64 {
        todo!("port MSTransportable::getMaxSpeed")
    }

    /// C++: `SUMOTime MSTransportable::getWaitingTime(const bool) const` [const, virtual]
    // SUMOTime
    // MSTransportable::getWaitingTime(const bool /* accumulated */) const {
    //     return (*myStep)->getWaitingTime();
    // }
    #[allow(unused_variables)]
    pub fn get_waiting_time(&self, accumulated: bool) -> SUMOTime {
        todo!("port MSTransportable::getWaitingTime")
    }

    /// C++: `double MSTransportable::getPreviousSpeed() const` [const, virtual]
    // double getPreviousSpeed() const override {
    //         return getSpeed();
    //     }
    #[allow(unused_variables)]
    pub fn get_previous_speed(&self) -> f64 {
        todo!("port MSTransportable::getPreviousSpeed")
    }

    /// C++: `double MSTransportable::getAcceleration() const` [const, virtual]
    // double getAcceleration() const override {
    //         return 0.0;
    //     }
    #[allow(unused_variables)]
    pub fn get_acceleration(&self) -> f64 {
        todo!("port MSTransportable::getAcceleration")
    }

    /// C++: `double MSTransportable::getPositionOnLane() const` [const, virtual]
    // double getPositionOnLane() const override {
    //         return getEdgePos();
    //     }
    #[allow(unused_variables)]
    pub fn get_position_on_lane(&self) -> f64 {
        todo!("port MSTransportable::getPositionOnLane")
    }

    /// C++: `double MSTransportable::getBackPositionOnLane(const MSLane *) const` [const, virtual]
    // double
    // MSTransportable::getBackPositionOnLane(const MSLane* /*lane*/) const {
    //     return getEdgePos() - getVehicleType().getLength();
    // }
    #[allow(unused_variables)]
    pub fn get_back_position_on_lane(&self, lane: Ptr<MSLane>) -> f64 {
        todo!("port MSTransportable::getBackPositionOnLane")
    }

    /// C++: `Position MSTransportable::getPosition(const double) const` [const, virtual]
    // Position getPosition(const double /*offset*/) const override {
    //         return getPosition();
    //     }
    #[allow(unused_variables)]
    pub fn get_position(&self, arg0: f64) -> Position {
        todo!("port MSTransportable::getPosition")
    }

    /// C++: `void MSTransportable::MSTransportable(const SUMOVehicleParameter *, MSVehicleType *, MSTransportablePlan *, const bool)`
    /// constructor
    // MSTransportable::MSTransportable(const SUMOVehicleParameter* pars, MSVehicleType* vtype, MSTransportablePlan* plan, const bool isPerson) :
    //     SUMOTrafficObject(pars->id),
    //     myParameter(pars), myVType(vtype), myPlan(plan),
    //     myAmPerson(isPerson),
    //     myNumericalID(myCurrentNumericalIndex++),
    //     myRandomSeed(RandHelper::murmur3_32(pars->id, RandHelper::getSeed())) {
    //     myStep = myPlan->begin();
    //     // init devices
    //     MSDevice::buildTransportableDevices(*this, myDevices);
    //     for (MSStage* const stage : * myPlan) {
    //         stage->init(this);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn new(pars: Ptr<SUMOVehicleParameter>, vtype: Ptr<MSVehicleType>, plan: Ptr<MSTransportable_MSTransportablePlan>, isPerson: bool) -> Self {
        todo!("port MSTransportable::MSTransportable")
    }

    /// C++: `void MSTransportable::~MSTransportable() noexcept` [virtual]
    /// destructor
    // MSTransportable::~MSTransportable() {
    //     if (myStep != myPlan->end() && getCurrentStageType() == MSStageType::DRIVING) {
    //         MSStageDriving* const stage = dynamic_cast<MSStageDriving*>(*myStep);
    //         if (stage->getVehicle() != nullptr) {
    //             stage->getVehicle()->removeTransportable(this);
    //         } else if (stage->getOriginStop() != nullptr)  {
    //             stage->getOriginStop()->removeTransportable(this);
    //         }
    //     }
    //     if (myPlan != nullptr) {
    //         for (MSTransportablePlan::const_iterator i = myPlan->begin(); i != myPlan->end(); ++i) {
    //             delete *i;
    //         }
    //         delete myPlan;
    //         myPlan = nullptr;
    //     }
    //     for (MSTransportableDevice* dev : myDevices) {
    //         delete dev;
    //     }
    //     delete myParameter;
    //     if (myVType->isVehicleSpecific()) {
    //         MSNet::getInstance()->getVehicleControl().removeVType(myVType);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MSTransportable::~MSTransportable")
    }

    /// C++: `bool MSTransportable::proceed(MSNet *, SUMOTime, const bool)` [virtual]
    // bool
    // MSTransportable::proceed(MSNet* net, SUMOTime time, const bool vehicleArrived) {
    //     MSStage* const prior = *myStep;
    //     const std::string& error = prior->setArrived(net, this, time, vehicleArrived);
    //     // must be done before increasing myStep to avoid invalid state for rendering
    //     prior->getEdge()->removeTransportable(this);
    //     myStep++;
    //     if (error != "") {
    //         throw ProcessError(error);
    //     }
    //     /* We need to check whether an access stage is needed (or maybe even two).
    //        The general scheme is: If the prior stage ended at a stop and the next stage
    //        starts at an edge which is not the one the stop is at, but the stop has an access to it
    //        we need an access stage. The same is true if prior ends at an edge, the next stage
    //        is allowed to start at any stop the edge has access to.
    //        If we start at a stop or end at a stop no access is needed.
    //     */
    //     bool accessToStop = false;
    //     if (prior->getStageType() == MSStageType::WALKING || prior->getStageType() == MSStageType::DRIVING) {
    //         accessToStop = checkAccess(prior);
    //     } else if (prior->getStageType() == MSStageType::WAITING_FOR_DEPART) {
    //         for (MSTransportableDevice* const dev : myDevices) {
    //             dev->notifyEnter(*this, MSMoveReminder::NOTIFICATION_DEPARTED, nullptr);
    //         }
    //     }
    //     if (!accessToStop && (myStep == myPlan->end()
    //                           || ((*myStep)->getStageType() != MSStageType::DRIVING
    //                               && (*myStep)->getStageType() != MSStageType::TRIP))) {
    //         MSStoppingPlace* priorStop = prior->getStageType() == MSStageType::TRIP ? prior->getOriginStop() : prior->getDestinationStop();
    //         // a trip might resolve to DRIVING so we would have to stay at the stop
    //         // if a trip resolves to something else, this step will do stop removal
    //         if (priorStop != nullptr) {
    //             priorStop->removeTransportable(this);
    //         }
    //     }
    //     MSTransportableControl& tc = isPerson() ? MSNet::getInstance()->getPersonControl() : MSNet::getInstance()->getContainerControl();
    //     if (myStep != myPlan->end()) {
    //         if ((*myStep)->getStageType() == MSStageType::WALKING && (prior->getStageType() != MSStageType::ACCESS || prior->getDestination() != (*myStep)->getFromEdge())) {
    //             checkAccess(prior, false);
    //         } else if ((*myStep)->getStageType() == MSStageType::WAITING && prior->getStageType() == MSStageType::WAITING && prior->getDestination() != (*myStep)->getFromEdge()) {
    //             checkAccess(prior, false);
    //         }
    //         if ((*myStep)->proceed(net, this, time, prior)) {
    //             return true;
    //         } else {
    //             // some problem was encountered
    //             tc.addArrived();
    //             return false;
    //         }
    //     } else {
    //         tc.addArrived();
    //         return false;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn proceed(&mut self, net: Ptr<MSNet>, time: SUMOTime, vehicleArrived: bool) -> bool {
        todo!("port MSTransportable::proceed")
    }

    /// C++: `bool MSTransportable::checkAccess(const MSStage *const, const bool)` [virtual]
    // virtual bool checkAccess(const MSStage* const prior, const bool waitAtStop = true) {
    //         UNUSED_PARAMETER(prior);
    //         UNUSED_PARAMETER(waitAtStop);
    //         return false;
    //     }
    #[allow(unused_variables)]
    pub fn check_access(&mut self, prior: Ptr<MSStage>, waitAtStop: bool) -> bool {
        todo!("port MSTransportable::checkAccess")
    }

    /// C++: `void MSTransportable::setID(const std::string &)` [virtual]
    /// set the id (inherited from Named but forbidden for transportables)
    // void
    // MSTransportable::setID(const std::string& /*newID*/) {
    //     throw ProcessError(TL("Changing a transportable ID is not permitted"));
    // }
    #[allow(unused_variables)]
    pub fn set_id(&mut self, newID: &str) {
        todo!("port MSTransportable::setID")
    }

    /// C++: `const SUMOVehicleParameter & MSTransportable::getParameter() const` [const, virtual]
    // inline const SUMOVehicleParameter& getParameter() const override {
    //         return *myParameter;
    //     }
    #[allow(unused_variables)]
    pub fn get_parameter(&self) -> &SUMOVehicleParameter {
        todo!("port MSTransportable::getParameter")
    }

    /// C++: `const MSVehicleType & MSTransportable::getVehicleType() const` [const, virtual]
    // inline const MSVehicleType& getVehicleType() const override {
    //         return *myVType;
    //     }
    #[allow(unused_variables)]
    pub fn get_vehicle_type(&self) -> &MSVehicleType {
        todo!("port MSTransportable::getVehicleType")
    }

    /// C++: `const SUMOVTypeParameter & MSTransportable::getVTypeParameter() const` [const, virtual]
    /// Returns the object's "vehicle" type parameter
    // inline const SUMOVTypeParameter& getVTypeParameter() const override {
    //         return myVType->getParameter();
    //     }
    #[allow(unused_variables)]
    pub fn get_v_type_parameter(&self) -> &SUMOVTypeParameter {
        todo!("port MSTransportable::getVTypeParameter")
    }

    /// C++: `SumoRNG * MSTransportable::getRNG() const` [const, virtual]
    /// returns the associated RNG
    // SumoRNG*
    // MSTransportable::getRNG() const {
    //     return getEdge()->getLanes()[0]->getRNG();
    // }
    #[allow(unused_variables)]
    pub fn get_rng(&self) -> Ptr<SumoRNG> {
        todo!("port MSTransportable::getRNG")
    }

    /// C++: `int MSTransportable::getRNGIndex() const` [const, virtual]
    /// returns the index of the associated RNG
    // int
    // MSTransportable::getRNGIndex() const {
    //     const std::vector<MSLane*>& lanes = getEdge()->getLanes();
    //     return lanes.empty() ? 0 : lanes[0]->getRNGIndex();
    // }
    #[allow(unused_variables)]
    pub fn get_rng_index(&self) -> i32 {
        todo!("port MSTransportable::getRNGIndex")
    }

    /// C++: `SUMOTime MSTransportable::getDesiredDepart() const` [const]
    /// Returns the desired departure time.
    // SUMOTime
    // MSTransportable::getDesiredDepart() const {
    //     return myParameter->depart;
    // }
    #[allow(unused_variables)]
    pub fn get_desired_depart(&self) -> SUMOTime {
        todo!("port MSTransportable::getDesiredDepart")
    }

    /// C++: `void MSTransportable::setDeparted(SUMOTime)`
    /// logs depart time of the current stage
    // void
    // MSTransportable::setDeparted(SUMOTime now) {
    //     (*myStep)->setDeparted(now);
    // }
    #[allow(unused_variables)]
    pub fn set_departed(&mut self, now: SUMOTime) {
        todo!("port MSTransportable::setDeparted")
    }

    /// C++: `SUMOTime MSTransportable::getDeparture() const` [const]
    /// logs depart time of the current stage
    // SUMOTime
    // MSTransportable::getDeparture() const {
    //     for (const MSStage* const stage : *myPlan) {
    //         if (stage->getDeparted() >= 0) {
    //             return stage->getDeparted();
    //         }
    //     }
    //     return -1;
    // }
    #[allow(unused_variables)]
    pub fn get_departure(&self) -> SUMOTime {
        todo!("port MSTransportable::getDeparture")
    }

    /// C++: `const MSEdge * MSTransportable::getDestination() const` [const]
    /// Returns the current destination.
    // const MSEdge* getDestination() const {
    //         return (*myStep)->getDestination();
    //     }
    #[allow(unused_variables)]
    pub fn get_destination(&self) -> Ptr<MSEdge> {
        todo!("port MSTransportable::getDestination")
    }

    /// C++: `const MSEdge * MSTransportable::getNextDestination() const` [const]
    /// Returns the destination after the current destination.
    // const MSEdge* getNextDestination() const {
    //         return (*(myStep + 1))->getDestination();
    //     }
    #[allow(unused_variables)]
    pub fn get_next_destination(&self) -> Ptr<MSEdge> {
        todo!("port MSTransportable::getNextDestination")
    }

    /// C++: `const MSEdge * MSTransportable::getEdge() const` [const, virtual]
    /// Returns the current edge
    // const MSEdge* getEdge() const override {
    //         return (*myStep)->getEdge();
    //     }
    #[allow(unused_variables)]
    pub fn get_edge(&self) -> Ptr<MSEdge> {
        todo!("port MSTransportable::getEdge")
    }

    /// C++: `const MSEdge * MSTransportable::getCurrentEdge() const` [const, virtual]
    // const MSEdge*
    // MSTransportable::getCurrentEdge() const {
    //     const MSLane* lane = getLane();
    //     return lane == nullptr ? getEdge() : &lane->getEdge();
    // }
    #[allow(unused_variables)]
    pub fn get_current_edge(&self) -> Ptr<MSEdge> {
        todo!("port MSTransportable::getCurrentEdge")
    }

    /// C++: `const MSLane * MSTransportable::getLane() const` [const, virtual]
    /// Returns the current lane (may be nullptr)
    // const MSLane* getLane() const override {
    //         return (*myStep)->getLane();
    //     }
    #[allow(unused_variables)]
    pub fn get_lane(&self) -> Ptr<MSLane> {
        todo!("port MSTransportable::getLane")
    }

    /// C++: `const MSLane * MSTransportable::getBackLane() const` [const, virtual]
    // const MSLane* getBackLane() const override {
    //         return getLane();
    //     }
    #[allow(unused_variables)]
    pub fn get_back_lane(&self) -> Ptr<MSLane> {
        todo!("port MSTransportable::getBackLane")
    }

    /// C++: `const MSEdge * MSTransportable::getFromEdge() const` [const]
    /// Returns the departure edge
    // const MSEdge* getFromEdge() const {
    //         return (*myStep)->getFromEdge();
    //     }
    #[allow(unused_variables)]
    pub fn get_from_edge(&self) -> Ptr<MSEdge> {
        todo!("port MSTransportable::getFromEdge")
    }

    /// C++: `double MSTransportable::getEdgePos() const` [const, virtual]
    /// Return the position on the edge
    // double
    // MSTransportable::getEdgePos() const {
    //     return (*myStep)->getEdgePos(MSNet::getInstance()->getCurrentTimeStep());
    // }
    #[allow(unused_variables)]
    pub fn get_edge_pos(&self) -> f64 {
        todo!("port MSTransportable::getEdgePos")
    }

    /// C++: `int MSTransportable::getDirection() const` [const, virtual]
    /// Return the movement directon on the edge
    // int
    // MSTransportable::getDirection() const {
    //     return (*myStep)->getDirection();
    // }
    #[allow(unused_variables)]
    pub fn get_direction(&self) -> i32 {
        todo!("port MSTransportable::getDirection")
    }

    /// C++: `Position MSTransportable::getPosition() const` [const, virtual]
    /// Return the Network coordinate of the transportable
    // Position
    // MSTransportable::getPosition() const {
    //     return (*myStep)->getPosition(MSNet::getInstance()->getCurrentTimeStep());
    // }
    #[allow(unused_variables)]
    pub fn get_position_2(&self) -> Position {
        todo!("port MSTransportable::getPosition")
    }

    /// C++: `double MSTransportable::getAngle() const` [const, virtual]
    /// return the current angle of the transportable
    // double
    // MSTransportable::getAngle() const {
    //     return (*myStep)->getAngle(MSNet::getInstance()->getCurrentTimeStep());
    // }
    #[allow(unused_variables)]
    pub fn get_angle(&self) -> f64 {
        todo!("port MSTransportable::getAngle")
    }

    /// C++: `double MSTransportable::getWaitingSeconds() const` [const, virtual]
    /// the time this transportable spent waiting in seconds
    // double
    // MSTransportable::getWaitingSeconds() const {
    //     return STEPS2TIME((*myStep)->getWaitingTime());
    // }
    #[allow(unused_variables)]
    pub fn get_waiting_seconds(&self) -> f64 {
        todo!("port MSTransportable::getWaitingSeconds")
    }

    /// C++: `double MSTransportable::getSpeed() const` [const, virtual]
    /// the current speed of the transportable
    // double
    // MSTransportable::getSpeed() const {
    //     return (*myStep)->getSpeed();
    // }
    #[allow(unused_variables)]
    pub fn get_speed(&self) -> f64 {
        todo!("port MSTransportable::getSpeed")
    }

    /// C++: `double MSTransportable::getChosenSpeedFactor() const` [const, virtual]
    /// the current speed factor of the transportable (where applicable)
    // virtual double getChosenSpeedFactor() const override {
    //         return 1;
    //     }
    #[allow(unused_variables)]
    pub fn get_chosen_speed_factor(&self) -> f64 {
        todo!("port MSTransportable::getChosenSpeedFactor")
    }

    /// C++: `MSStageType MSTransportable::getCurrentStageType() const` [const]
    /// the current stage type of the transportable
    // MSStageType getCurrentStageType() const {
    //         return (*myStep)->getStageType();
    //     }
    #[allow(unused_variables)]
    pub fn get_current_stage_type(&self) -> MSStageType {
        todo!("port MSTransportable::getCurrentStageType")
    }

    /// C++: `MSStageType MSTransportable::getStageType(int) const` [const]
    /// the stage type for the nth next stage
    // MSStageType getStageType(int next) const {
    //         assert(myStep + next < myPlan->end());
    //         assert(myStep + next >= myPlan->begin());
    //         return (*(myStep + next))->getStageType();
    //     }
    #[allow(unused_variables)]
    pub fn get_stage_type(&self, next: i32) -> MSStageType {
        todo!("port MSTransportable::getStageType")
    }

    /// C++: `std::string MSTransportable::getStageSummary(int) const` [const]
    /// return textual summary for the given stage
    // std::string
    // MSTransportable::getStageSummary(int stageIndex) const {
    //     assert(stageIndex < (int)myPlan->size());
    //     assert(stageIndex >= 0);
    //     return (*myPlan)[stageIndex]->getStageSummary(myAmPerson);
    // }
    #[allow(unused_variables)]
    pub fn get_stage_summary(&self, stageIndex: i32) -> String {
        todo!("port MSTransportable::getStageSummary")
    }

    /// C++: `std::string MSTransportable::getCurrentStageDescription() const` [const]
    /// Returns the current stage description as a string
    // std::string getCurrentStageDescription() const {
    //         return (*myStep)->getStageDescription(myAmPerson);
    //     }
    #[allow(unused_variables)]
    pub fn get_current_stage_description(&self) -> String {
        todo!("port MSTransportable::getCurrentStageDescription")
    }

    /// C++: `MSStage * MSTransportable::getCurrentStage() const` [const]
    /// Return the current stage
    // MSStage* getCurrentStage() const {
    //         return *myStep;
    //     }
    #[allow(unused_variables)]
    pub fn get_current_stage(&self) -> Ptr<MSStage> {
        todo!("port MSTransportable::getCurrentStage")
    }

    /// C++: `MSStage * MSTransportable::getNextStage(int) const` [const]
    /// Return the next (or previous) stage denoted by the offset
    // inline MSStage* getNextStage(int offset) const {
    //         assert(myStep + offset >= myPlan->begin());
    //         assert(myStep + offset < myPlan->end());
    //         return *(myStep + offset);
    //     }
    #[allow(unused_variables)]
    pub fn get_next_stage(&self, offset: i32) -> Ptr<MSStage> {
        todo!("port MSTransportable::getNextStage")
    }

    /// C++: `const std::set<NumericalID> MSTransportable::getUpcomingEdgeIDs() const` [const, virtual]
    /// returns the numerical IDs of edges to be used (possibly of future stages)
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_upcoming_edge_i_ds(&self) -> BTreeSet<SUMOTrafficObject_NumericalID> {
        todo!("port MSTransportable::getUpcomingEdgeIDs")
    }

    /// C++: `int MSTransportable::getNumStages() const` [const]
    /// Return the total number stages in this person's plan
    // inline int getNumStages() const {
    //         return (int)myPlan->size();
    //     }
    #[allow(unused_variables)]
    pub fn get_num_stages(&self) -> i32 {
        todo!("port MSTransportable::getNumStages")
    }

    /// C++: `int MSTransportable::getNumRemainingStages() const` [const]
    /// Return the number of remaining stages (including the current)
    // inline int getNumRemainingStages() const {
    //         return (int)(myPlan->end() - myStep);
    //     }
    #[allow(unused_variables)]
    pub fn get_num_remaining_stages(&self) -> i32 {
        todo!("port MSTransportable::getNumRemainingStages")
    }

    /// C++: `int MSTransportable::getCurrentStageIndex() const` [const]
    /// Return the index of the current stage
    // inline int getCurrentStageIndex() const {
    //         return (int)(myStep - myPlan->begin());
    //     }
    #[allow(unused_variables)]
    pub fn get_current_stage_index(&self) -> i32 {
        todo!("port MSTransportable::getCurrentStageIndex")
    }

    /// C++: `int MSTransportable::getRoutePosition() const` [const, virtual]
    /// return the index of the edge within the route
    // inline int getRoutePosition() const override {
    //         return (*myStep)->getRoutePosition();
    //     }
    #[allow(unused_variables)]
    pub fn get_route_position(&self) -> i32 {
        todo!("port MSTransportable::getRoutePosition")
    }

    /// C++: `const MSEdge * MSTransportable::getNextEdgePtr() const` [const, virtual]
    /// returns the next edge ptr (used by walking persons)
    // virtual const MSEdge* getNextEdgePtr() const override {
    //         return nullptr;
    //     }
    #[allow(unused_variables)]
    pub fn get_next_edge_ptr(&self) -> Ptr<MSEdge> {
        todo!("port MSTransportable::getNextEdgePtr")
    }

    /// C++: `void MSTransportable::tripInfoOutput(OutputDevice &) const` [const]
    /// Called on writing tripinfo output
    // void
    // MSTransportable::tripInfoOutput(OutputDevice& os) const {
    //     SUMOTime departure = myPlan->front()->getDeparted();
    //     os.openTag(isPerson() ? "personinfo" : "containerinfo");
    //     os.writeAttr(SUMO_ATTR_ID, getID());
    //     os.writeAttr(SUMO_ATTR_DEPART, departure >= 0 ? time2string(departure) : "-1");
    //     os.writeAttr(SUMO_ATTR_TYPE, getVehicleType().getID());
    //     if (isPerson()) {
    //         os.writeAttr(SUMO_ATTR_SPEEDFACTOR, getChosenSpeedFactor());
    //     }
    //     SUMOTime duration = 0;
    //     SUMOTime waitingTime = 0;
    //     SUMOTime timeLoss = 0;
    //     SUMOTime travelTime = 0;
    //     bool durationOK = true;
    //     bool waitingTimeOK = true;
    //     bool timeLossOK = true;
    //     bool travelTimeOK = true;
    //     for (MSStage* const i : *myPlan) {
    //         SUMOTime t = i->getDuration();
    //         if (t != SUMOTime_MAX) {
    //             duration += t;
    //         } else {
    //             durationOK = false;
    //         }
    //         t = i->getTotalWaitingTime();
    //         if (t != SUMOTime_MAX) {
    //             waitingTime += t;
    //         } else {
    //             waitingTimeOK = false;
    //         }
    //         t = i->getTimeLoss(this);
    //         if (t != SUMOTime_MAX) {
    //             timeLoss += t;
    //         } else {
    //             timeLossOK = false;
    //         }
    //         t = i->getTravelTime();
    //         if (t != SUMOTime_MAX) {
    //             travelTime += t;
    //         } else {
    //             travelTimeOK = false;
    //         }
    //     }
    //     os.writeAttr(SUMO_ATTR_DURATION, durationOK ? time2string(duration) : "-1");
    //     os.writeAttr(SUMO_ATTR_WAITINGTIME, waitingTimeOK ? time2string(waitingTime) : "-1");
    //     os.writeAttr(SUMO_ATTR_TIMELOSS, timeLossOK ? time2string(timeLoss) : "-1");
    //     os.writeAttr(SUMO_ATTR_TRAVELTIME, travelTimeOK ? time2string(travelTime) : "-1");
    //     for (MSStage* const i : *myPlan) {
    //         i->tripInfoOutput(os, this);
    //     }
    //     os.closeTag();
    // }
    #[allow(unused_variables)]
    pub fn trip_info_output(&self, os: &mut OutputDevice) {
        todo!("port MSTransportable::tripInfoOutput")
    }

    /// C++: `void MSTransportable::routeOutput(OutputDevice &, const bool) const` [const]
    /// Called on writing vehroute output
    // void
    // MSTransportable::routeOutput(OutputDevice& os, const bool withRouteLength) const {
    //     const std::string typeID = (
    //                                    (isPerson() && getVehicleType().getID() == DEFAULT_PEDTYPE_ID)
    //                                    || (isContainer() && getVehicleType().getID() == DEFAULT_CONTAINERTYPE_ID)) ? "" : getVehicleType().getID();
    //     myParameter->write(os, OptionsCont::getOptions(), isPerson() ? SUMO_TAG_PERSON : SUMO_TAG_CONTAINER, typeID);
    //     if (hasArrived()) {
    //         os.writeAttr("arrival", time2string(MSNet::getInstance()->getCurrentTimeStep()));
    //     }
    //     const MSStage* previous = nullptr;
    //     const bool withTiming = OptionsCont::getOptions().getBool("vehroute-output.exit-times");
    //     for (const MSStage* const stage : *myPlan) {
    //         stage->routeOutput(myAmPerson, os, withRouteLength, previous, withTiming);
    //         previous = stage;
    //     }
    //     myParameter->writeParams(os);
    //     os.closeTag();
    //     os.lf();
    // }
    #[allow(unused_variables)]
    pub fn route_output(&self, os: &mut OutputDevice, withRouteLength: bool) {
        todo!("port MSTransportable::routeOutput")
    }

    /// C++: `bool MSTransportable::isWaitingFor(const SUMOVehicle *) const` [const]
    /// Whether the transportable waits for the given vehicle in the current step
    // bool isWaitingFor(const SUMOVehicle* vehicle) const {
    //         return (*myStep)->isWaitingFor(vehicle);
    //     }
    #[allow(unused_variables)]
    pub fn is_waiting_for(&self, vehicle: Ptr<SUMOVehicle>) -> bool {
        todo!("port MSTransportable::isWaitingFor")
    }

    /// C++: `bool MSTransportable::isWaiting4Vehicle() const` [const]
    /// Whether the transportable waits for a vehicle
    // bool isWaiting4Vehicle() const {
    //         return (*myStep)->isWaiting4Vehicle();
    //     }
    #[allow(unused_variables)]
    pub fn is_waiting4_vehicle(&self) -> bool {
        todo!("port MSTransportable::isWaiting4Vehicle")
    }

    /// C++: `void MSTransportable::setAbortWaiting(const SUMOTime)`
    // void
    // MSTransportable::setAbortWaiting(const SUMOTime timeout) {
    //     if (timeout < 0 && myAbortCommand != nullptr) {
    //         myAbortCommand->deschedule();
    //         myAbortCommand = nullptr;
    //         return;
    //     }
    //     myAbortCommand = new WrappingCommand<MSTransportable>(this, &MSTransportable::abortStage);
    //     MSNet::getInstance()->getBeginOfTimestepEvents()->addEvent(myAbortCommand, SIMSTEP + timeout);
    // }
    #[allow(unused_variables)]
    pub fn set_abort_waiting(&mut self, timeout: SUMOTime) {
        todo!("port MSTransportable::setAbortWaiting")
    }

    /// C++: `SUMOTime MSTransportable::abortStage(SUMOTime)`
    /// Abort current stage (used for aborting waiting for a vehicle)
    // SUMOTime
    // MSTransportable::abortStage(SUMOTime step) {
    //     WRITE_WARNINGF(TL("Teleporting % '%'; waited too long, from edge '%', time=%."),
    //                    isPerson() ? "person" : "container", getID(), (*myStep)->getEdge()->getID(), time2string(step));
    //     MSTransportableControl& tc = isPerson() ? MSNet::getInstance()->getPersonControl() : MSNet::getInstance()->getContainerControl();
    //     tc.registerTeleportAbortWait();
    //     (*myStep)->abort(this);
    //     if (!proceed(MSNet::getInstance(), step)) {
    //         tc.erase(this);
    //     }
    //     return 0;
    // }
    #[allow(unused_variables)]
    pub fn abort_stage(&mut self, step: SUMOTime) -> SUMOTime {
        todo!("port MSTransportable::abortStage")
    }

    /// C++: `SUMOVehicle * MSTransportable::getVehicle() const` [const]
    /// The vehicle associated with this transportable
    // SUMOVehicle* getVehicle() const {
    //         return (*myStep)->getVehicle();
    //     }
    #[allow(unused_variables)]
    pub fn get_vehicle(&self) -> Ptr<SUMOVehicle> {
        todo!("port MSTransportable::getVehicle")
    }

    /// C++: `void MSTransportable::appendStage(MSStage *, int)`
    /// Appends the given stage to the current plan
    // void
    // MSTransportable::appendStage(MSStage* stage, int next) {
    //     // myStep is invalidated upon modifying myPlan
    //     const int stepIndex = (int)(myStep - myPlan->begin());
    //     if (next < 0) {
    //         myPlan->push_back(stage);
    //     } else {
    //         if (stepIndex + next > (int)myPlan->size()) {
    //             throw ProcessError("invalid index '" + toString(next) + "' for inserting new stage into plan of '" + getID() + "'");
    //         }
    //         myPlan->insert(myPlan->begin() + stepIndex + next, stage);
    //     }
    //     myStep = myPlan->begin() + stepIndex;
    // }
    #[allow(unused_variables)]
    pub fn append_stage(&mut self, stage: Ptr<MSStage>, next: i32) {
        todo!("port MSTransportable::appendStage")
    }

    /// C++: `void MSTransportable::removeStage(int, bool)`
    /// removes the nth next stage
    // void
    // MSTransportable::removeStage(int next, bool stayInSim) {
    //     assert(myStep + next < myPlan->end());
    //     assert(next >= 0);
    //     if (next > 0) {
    //         // myStep is invalidated upon modifying myPlan
    //         int stepIndex = (int)(myStep - myPlan->begin());
    //         delete *(myStep + next);
    //         myPlan->erase(myStep + next);
    //         myStep = myPlan->begin() + stepIndex;
    //     } else {
    //         if (myStep + 1 == myPlan->end() && stayInSim) {
    //             // stay in the simulation until the start of simStep to allow appending new stages (at the correct position)
    //             appendStage(new MSStageWaiting(getEdge(), nullptr, 0, 0, getEdgePos(), "last stage removed", false));
    //         } else if (myStep + 1 != myPlan->end()) {
    //             (*(myStep + 1))->setOrigin(getEdge(), getEdge() == getFromEdge() ? (*myStep)->getOriginStop() : nullptr, getEdgePos());
    //         }
    //         (*myStep)->abort(this);
    //         if (!proceed(MSNet::getInstance(), SIMSTEP)) {
    //             MSNet::getInstance()->getPersonControl().erase(this);
    //         } else if (myPlan->front()->getDeparted() < 0) {
    //             myPlan->front()->setDeparted(SIMSTEP);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn remove_stage(&mut self, next: i32, stayInSim: bool) {
        todo!("port MSTransportable::removeStage")
    }

    /// C++: `void MSTransportable::setSpeed(double)`
    /// set the speed for all present and future (walking) stages and modify the vType so that stages added later are also affected
    // void
    // MSTransportable::setSpeed(double speed) {
    //     for (MSTransportablePlan::const_iterator i = myStep; i != myPlan->end(); ++i) {
    //         (*i)->setSpeed(speed);
    //     }
    //     getSingularType().setMaxSpeed(speed);
    // }
    #[allow(unused_variables)]
    pub fn set_speed(&mut self, speed: f64) {
        todo!("port MSTransportable::setSpeed")
    }

    /// C++: `double MSTransportable::getArrivalPos() const` [const]
    /// returns the final arrival pos
    // double getArrivalPos() const {
    //         return myPlan->back()->getArrivalPos();
    //     }
    #[allow(unused_variables)]
    pub fn get_arrival_pos(&self) -> f64 {
        todo!("port MSTransportable::getArrivalPos")
    }

    /// C++: `const MSEdge * MSTransportable::getArrivalEdge() const` [const]
    /// returns the final arrival edge
    // const MSEdge* getArrivalEdge() const {
    //         return myPlan->back()->getEdges().back();
    //     }
    #[allow(unused_variables)]
    pub fn get_arrival_edge(&self) -> Ptr<MSEdge> {
        todo!("port MSTransportable::getArrivalEdge")
    }

    /// C++: `const MSEdge * MSTransportable::getRerouteDestination() const` [const, virtual]
    /// Returns the end point for reroutes (usually the last edge of the route)
    // const MSEdge* getRerouteDestination() const override {
    //         return getArrivalEdge();
    //     }
    #[allow(unused_variables)]
    pub fn get_reroute_destination(&self) -> Ptr<MSEdge> {
        todo!("port MSTransportable::getRerouteDestination")
    }

    /// C++: `bool MSTransportable::reroute(SUMOTime, const std::string &, MSTransportableRouter &, const bool, const bool, const bool, const MSEdge *)`
    // bool
    // MSTransportable::reroute(SUMOTime t, const std::string& /* info */, MSTransportableRouter& router, const bool /* onInit */, const bool /* withTaz */, const bool /* silent */, const MSEdge* /* sink */) {
    //     MSStageTrip* trip = getCurrentStage()->getTrip();
    //     if (trip == nullptr) {
    //         // TODO this should be possible after factoring out MSStageTrip::reroute
    //         return false;
    //     }
    //     if (getCurrentStage()->getVehicle() != nullptr) {
    //         // TODO rerouting during a ride still needs to be implemented
    //         return false;
    //     }
    //     // find the final stage of the trip
    //     int tripEndOffset = -1;
    //     for (int i = getNumRemainingStages() - 1; i >= 0; i--) {
    //         if (getNextStage(i)->getTrip() == trip) {
    //             tripEndOffset = i;
    //             break;
    //         }
    //     }
    //     std::vector<MSStage*> stages;
    //     MSStageWaiting start(getEdge(), getCurrentStage()->getOriginStop(), -1, t, getEdgePos(), "start", true);
    //     if (trip->reroute(t, router, this, &start, getEdge(), getRerouteDestination(), stages) == "") {
    //         // check whether the new plan actually differs
    //         while (tripEndOffset >= 0 && !stages.empty() && stages.back()->equals(*getNextStage(tripEndOffset))) {
    //             delete stages.back();
    //             stages.pop_back();
    //             tripEndOffset--;
    //         }
    //         bool abortCurrent = true;
    //         // check whether the future route of the current stage is identical to the route
    //         if (!stages.empty() && stages.front()->isWalk() && getCurrentStage()->isWalk()) {
    //             // TODO this check should be done for rides as well
    //             MSStageMoving* s = static_cast<MSStageMoving*>(getCurrentStage());
    //             int routeIndex = (int)(s->getRouteStep() - s->getRoute().begin());
    //             ConstMSEdgeVector oldEdges = s->getEdges();
    //             oldEdges.erase(oldEdges.begin(), oldEdges.begin() + routeIndex);
    //             ConstMSEdgeVector newEdges = stages.front()->getEdges();
    //             if (newEdges == oldEdges) {
    //                 delete stages.front();
    //                 stages.erase(stages.begin());
    //                 abortCurrent = false;
    //             }
    //         }
    //         if (stages.empty()) {
    //             return false;
    //         }
    //         // remove future stages of the trip
    //         for (int i = tripEndOffset; i >= 1; i--) {
    //             removeStage(i);
    //         }
    //         // insert new stages of the rerouting
    //         int idx = 1;
    //         for (MSStage* stage : stages) {
    //             appendStage(stage, idx++);
    //         }
    //         if (abortCurrent) {
    //             removeStage(0);
    //         }
    //         return true;
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn reroute(&mut self, t: SUMOTime, info: &str, router: &mut MSTransportableRouter, onInit: bool, withTaz: bool, silent: bool, sink: Ptr<MSEdge>) -> bool {
        todo!("port MSTransportable::reroute")
    }

    /// C++: `bool MSTransportable::replaceRoute(ConstMSRoutePtr, const std::string &, bool, int, bool, bool, std::string *)` [virtual]
    /// Replaces the current route by the given one
    // bool
    // MSTransportable::replaceRoute(ConstMSRoutePtr newRoute, const std::string& /* info */, bool /* onInit */, int /* offset */, bool /* addRouteStops */, bool /* removeStops */, std::string* /* msgReturn */) {
    //     if (isPerson()) {
    //         static_cast<MSPerson*>(this)->replaceWalk(newRoute->getEdges(), getPositionOnLane(), 0, 1);
    //         return true;
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn replace_route(&mut self, route: ConstMSRoutePtr, info: &str, onInit: bool, offset: i32, addStops: bool, removeStops: bool, msgReturn: Ptr<String>) -> bool {
        todo!("port MSTransportable::replaceRoute")
    }

    /// C++: `void MSTransportable::replaceVehicleType(const MSVehicleType *)` [virtual]
    /// Replaces the current vehicle type by the one given
    // void
    // MSTransportable::replaceVehicleType(const MSVehicleType* type) {
    //     const SUMOVehicleClass oldVClass = myVType->getVehicleClass();
    //     if (myVType->isVehicleSpecific()) {
    //         MSNet::getInstance()->getVehicleControl().removeVType(myVType);
    //     }
    //     if (isPerson()
    //             && type->getVehicleClass() != oldVClass
    //             && type->getVehicleClass() != SVC_PEDESTRIAN
    //             && !type->getParameter().wasSet(VTYPEPARS_VEHICLECLASS_SET)) {
    //         WRITE_WARNINGF(TL("Person '%' receives type '%' which implicitly uses unsuitable vClass '%'."), getID(), type->getID(), toString(type->getVehicleClass()));
    //     }
    //     myVType = type;
    // }
    #[allow(unused_variables)]
    pub fn replace_vehicle_type(&mut self, type_: Ptr<MSVehicleType>) {
        todo!("port MSTransportable::replaceVehicleType")
    }

    /// C++: `MSVehicleType & MSTransportable::getSingularType()`
    /// Replaces the current vehicle type with a new one used by this vehicle only
    // MSVehicleType&
    // MSTransportable::getSingularType() {
    //     if (myVType->isVehicleSpecific()) {
    //         return *const_cast<MSVehicleType*>(myVType);
    //     }
    //     MSVehicleType* type = myVType->buildSingularType(myVType->getID() + "@" + getID());
    //     replaceVehicleType(type);
    //     return *type;
    // }
    #[allow(unused_variables)]
    pub fn get_singular_type(&mut self) -> &mut MSVehicleType {
        todo!("port MSTransportable::getSingularType")
    }

    /// C++: `PositionVector MSTransportable::getBoundingBox() const` [const]
    /// return the bounding box of the person
    // PositionVector
    // MSTransportable::getBoundingBox() const {
    //     PositionVector centerLine;
    //     const Position p = getPosition();
    //     const double angle = getAngle();
    //     const double length = getVehicleType().getLength();
    //     const Position back = p + Position(-cos(angle) * length, -sin(angle) * length);
    //     centerLine.push_back(p);
    //     centerLine.push_back(back);
    //     centerLine.move2side(0.5 * getVehicleType().getWidth());
    //     PositionVector result = centerLine;
    //     centerLine.move2side(-getVehicleType().getWidth());
    //     result.append(centerLine.reverse(), POSITION_EPS);
    //     //std::cout << " transp=" << getID() << " p=" << p << " angle=" << GeomHelper::naviDegree(angle) << " back=" << back << " result=" << result << "\n";
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_bounding_box(&self) -> PositionVector {
        todo!("port MSTransportable::getBoundingBox")
    }

    /// C++: `bool MSTransportable::hasArrived() const` [const, virtual]
    /// return whether the person has reached the end of its plan
    // bool
    // MSTransportable::hasArrived() const {
    //     return myStep == myPlan->end();
    // }
    #[allow(unused_variables)]
    pub fn has_arrived(&self) -> bool {
        todo!("port MSTransportable::hasArrived")
    }

    /// C++: `bool MSTransportable::hasDeparted() const` [const]
    /// return whether the transportable has started its plan
    // bool
    // MSTransportable::hasDeparted() const {
    //     return myPlan->size() > 0 && (myPlan->front()->getDeparted() >= 0 || myStep > myPlan->begin());
    // }
    #[allow(unused_variables)]
    pub fn has_departed(&self) -> bool {
        todo!("port MSTransportable::hasDeparted")
    }

    /// C++: `void MSTransportable::rerouteParkingArea(MSStoppingPlace *, MSStoppingPlace *)`
    /// adapt plan when the vehicle reroutes and now stops at replacement instead of orig
    // void
    // MSTransportable::rerouteParkingArea(MSStoppingPlace* orig, MSStoppingPlace* replacement) {
    //     // check whether the transportable was riding to the orignal stop
    //     // @note: parkingArea can currently not be set as myDestinationStop so we
    //     // check for stops on the edge instead
    // #ifdef DEBUG_PARKING
    //     std::cout << SIMTIME << " person=" << getID() << " rerouteParkingArea orig=" << orig->getID() << " replacement=" << replacement->getID() << "\n";
    // #endif
    //     assert(getCurrentStageType() == MSStageType::DRIVING);
    //     if (!myAmPerson) {
    //         WRITE_WARNING(TL("parkingAreaReroute not supported for containers"));
    //         return;
    //     }
    //     if (getDestination() == &orig->getLane().getEdge()) {
    //         MSStageDriving* const stage = dynamic_cast<MSStageDriving*>(*myStep);
    //         assert(stage != 0);
    //         assert(stage->getVehicle() != 0);
    //         // adapt plan
    //         stage->setDestination(&replacement->getLane().getEdge(), replacement);
    //         stage->setArrivalPos((replacement->getBeginLanePosition() + replacement->getEndLanePosition()) / 2);
    // #ifdef DEBUG_PARKING
    //         std::cout << " set ride destination\n";
    // #endif
    //         if (myStep + 1 == myPlan->end()) {
    //             return;
    //         }
    //         // if the next step is a walk, adapt the route
    //         MSStage* nextStage = *(myStep + 1);
    //         if (nextStage->getStageType() == MSStageType::TRIP) {
    //             dynamic_cast<MSStageTrip*>(nextStage)->setOrigin(stage->getDestination(), stage->getDestinationStop(), stage->getArrivalPos());
    // #ifdef DEBUG_PARKING
    //             std::cout << " set subsequent trip origin\n";
    // #endif
    //         } else if (nextStage->getStageType() == MSStageType::WALKING) {
    // #ifdef DEBUG_PARKING
    //             std::cout << " replace subsequent walk with a trip\n";
    // #endif
    //             MSStageTrip* newStage = new MSStageTrip(stage->getDestination(), nullptr, nextStage->getDestination(),
    //                                                     nextStage->getDestinationStop(), -1, 0, "", -1, 1, getID(), 0, true, nextStage->getArrivalPos());
    //             removeStage(1);
    //             appendStage(newStage, 1);
    //         } else if (nextStage->getStageType() == MSStageType::WAITING) {
    // #ifdef DEBUG_PARKING
    //             std::cout << " add subsequent walk to reach stop\n";
    //             std::cout << "   arrivalPos=" << nextStage->getArrivalPos() << "\n";
    // #endif
    //             MSStageTrip* newStage = new MSStageTrip(stage->getDestination(), nullptr, nextStage->getDestination(),
    //                                                     nextStage->getDestinationStop(), -1, 0, "", -1, 1, getID(), 0, true, nextStage->getArrivalPos());
    //             appendStage(newStage, 1);
    //         }
    //         // if the plan contains another ride with the same vehicle from the same
    //         // parking area, adapt the preceeding walk to end at the replacement
    //         for (auto it = myStep + 2; it != myPlan->end(); it++) {
    //             MSStage* const futureStage = *it;
    //             MSStage* const prevStage = *(it - 1);
    //             if (futureStage->getStageType() == MSStageType::DRIVING) {
    //                 MSStageDriving* const ds = static_cast<MSStageDriving*>(futureStage);
    //                 // ride origin is set implicitly from the walk destination
    //                 ds->setOrigin(nullptr, nullptr, -1);
    //                 if (ds->getLines() == stage->getLines()
    //                         && prevStage->getDestination() == &orig->getLane().getEdge()) {
    //                     if (prevStage->getStageType() == MSStageType::TRIP) {
    //                         dynamic_cast<MSStageTrip*>(prevStage)->setDestination(stage->getDestination(), replacement);
    // #ifdef DEBUG_PARKING
    //                         std::cout << " replace later trip before ride (" << (it - myPlan->begin()) << ")\n";
    // #endif
    //                     } else if (prevStage->getStageType() == MSStageType::WALKING) {
    // #ifdef DEBUG_PARKING
    //                         std::cout << " replace later walk before ride (" << (it - myPlan->begin()) << ")\n";
    // #endif
    //                         MSStageTrip* newStage = new MSStageTrip(prevStage->getFromEdge(), nullptr, stage->getDestination(),
    //                                                                 replacement, -1, 0, "", -1, 1, getID(), 0, true, stage->getArrivalPos());
    //                         int prevStageRelIndex = (int)(it - 1 - myStep);
    //                         removeStage(prevStageRelIndex);
    //                         appendStage(newStage, prevStageRelIndex);
    //                     }
    //                     break;
    //                 }
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn reroute_parking_area(&mut self, orig: Ptr<MSStoppingPlace>, replacement: Ptr<MSStoppingPlace>) {
        todo!("port MSTransportable::rerouteParkingArea")
    }

    /// C++: `MSDevice * MSTransportable::getDevice(const std::type_info &) const` [const, virtual]
    /// Returns a device of the given type if it exists or nullptr if not
    // MSDevice*
    // MSTransportable::getDevice(const std::type_info& type) const {
    //     for (MSTransportableDevice* const dev : myDevices) {
    //         if (typeid(*dev) == type) {
    //             return dev;
    //         }
    //     }
    //     return nullptr;
    // }
    #[allow(unused_variables)]
    pub fn get_device(&self, type_: &Unmapped /* std::type_info */) -> Ptr<MSDevice> {
        todo!("port MSTransportable::getDevice")
    }

    /// C++: `void MSTransportable::setJunctionModelParameter(const std::string &, const std::string &)`
    /// set individual junction model paramete (not type related)
    // void
    // MSTransportable::setJunctionModelParameter(const std::string& key, const std::string& value) {
    //     if (key == toString(SUMO_ATTR_JM_IGNORE_IDS) || key == toString(SUMO_ATTR_JM_IGNORE_TYPES)) {
    //         getParameter().parametersSet |= VEHPARS_JUNCTIONMODEL_PARAMS_SET;
    //         const_cast<SUMOVehicleParameter&>(getParameter()).setParameter(key, value);
    //         // checked in MSLink::ignoreFoe
    //     } else {
    //         throw InvalidArgument(getObjectType() + " '" + getID() + "' does not support junctionModel parameter '" + key + "'");
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_junction_model_parameter(&mut self, key: &str, value: &str) {
        todo!("port MSTransportable::setJunctionModelParameter")
    }

    /// C++: `const std::vector<MSTransportableDevice *> & MSTransportable::getDevices() const` [const]
    /// Returns this vehicle's devices
    // inline const std::vector<MSTransportableDevice*>& getDevices() const {
    //         return myDevices;
    //     }
    #[allow(unused_variables)]
    pub fn get_devices(&self) -> &Vec<Ptr<MSTransportableDevice>> {
        todo!("port MSTransportable::getDevices")
    }

    /// C++: `bool MSTransportable::hasInfluencer() const` [const, virtual]
    // virtual bool hasInfluencer() const override {
    //         return false;
    //     }
    #[allow(unused_variables)]
    pub fn has_influencer(&self) -> bool {
        todo!("port MSTransportable::hasInfluencer")
    }

    /// C++: `bool MSTransportable::isSelected() const` [const, virtual]
    /// whether this transportable is selected in the GUI
    // virtual bool isSelected() const override {
    //         return false;
    //     }
    #[allow(unused_variables)]
    pub fn is_selected(&self) -> bool {
        todo!("port MSTransportable::isSelected")
    }

    /// C++: `int MSTransportable::getRoutingMode() const` [const, virtual]
    /// return routing mode (configures router choice but also handling of transient permission changes)
    // int
    // MSTransportable::getRoutingMode() const {
    //     /// @todo: allow configuring routing mode
    //     return libsumo::ROUTING_MODE_DEFAULT;
    // }
    #[allow(unused_variables)]
    pub fn get_routing_mode(&self) -> i32 {
        todo!("port MSTransportable::getRoutingMode")
    }

    /// C++: `void MSTransportable::setRoutingMode(int)` [virtual]
    /// Sets routing behavior
    // void
    // MSTransportable::setRoutingMode(int value) {
    //     /// @todo: allow configuring routing mode
    //     UNUSED_PARAMETER(value);
    // }
    #[allow(unused_variables)]
    pub fn set_routing_mode(&mut self, value: i32) {
        todo!("port MSTransportable::setRoutingMode")
    }

    /// C++: `void MSTransportable::saveState(OutputDevice &)`
    /// Saves the current state into the given stream
    // void
    // MSTransportable::saveState(OutputDevice& out) {
    //     // this saves lots of departParameters which are only needed for transportables that did not yet depart
    //     // the parameters may hold the name of a vTypeDistribution but we are interested in the actual type
    //     const SUMOTime desiredDepart = myParameter->depart;
    //     if (myPlan->front()->getDeparted() >= 0) {
    //         // this is only relevant in the context of delayed departure (max-num-persons)
    //         const_cast<SUMOVehicleParameter*>(myParameter)->depart = myPlan->front()->getDeparted();
    //     }
    //     myParameter->write(out, OptionsCont::getOptions(), myAmPerson ? SUMO_TAG_PERSON : SUMO_TAG_CONTAINER, getVehicleType().getID());
    //     const_cast<SUMOVehicleParameter*>(myParameter)->depart = desiredDepart;
    //     if (!myParameter->wasSet(VEHPARS_SPEEDFACTOR_SET)) {
    //         out.setPrecision(MAX2(gPrecisionRandom, gPrecision));
    //         out.writeAttr(SUMO_ATTR_SPEEDFACTOR, getChosenSpeedFactor());
    //         out.setPrecision(gPrecision);
    //     }
    //     int stepIdx = (int)(myStep - myPlan->begin());
    //     for (auto it = myPlan->begin(); it != myStep; ++it) {
    //         const MSStageType st = (*it)->getStageType();
    //         if (st == MSStageType::TRIP || st == MSStageType::ACCESS) {
    //             stepIdx--;
    //         }
    //     }
    //     const bool isAccess = (*myStep)->getStageType() == MSStageType::ACCESS;
    //     std::ostringstream state;
    //     state << myParameter->parametersSet << " " << (isAccess ? toString(stepIdx - 0.5, 1) : toString(stepIdx));
    //     (*myStep)->saveState(state, this);
    //     out.writeAttr(SUMO_ATTR_STATE, state.str());
    //     const MSStage* previous = nullptr;
    //     for (const MSStage* const stage : *myPlan) {
    //         const bool routeLength = stage->getDeparted() >= 0 && stage->getStageType() == MSStageType::DRIVING;
    //         const bool withTiming = stage->getDeparted() >= 0;
    //         stage->routeOutput(myAmPerson, out, routeLength, previous, withTiming, true);
    //         previous = stage;
    //     }
    //     out.closeTag();
    // }
    #[allow(unused_variables)]
    pub fn save_state(&mut self, out: &mut OutputDevice) {
        todo!("port MSTransportable::saveState")
    }

    /// C++: `void MSTransportable::loadState(const std::string &)`
    /// Reconstructs the current state
    // void
    // MSTransportable::loadState(const std::string& state) {
    //     std::istringstream iss(state);
    //     double step;
    //     iss >> myParameter->parametersSet >> step;
    //     myPlan->front()->setDeparted(myParameter->depart);
    //     if (step != floor(step)) {
    //         // we are in an access stage
    //         int priorIndex = (int)(step - 0.5);
    //         MSStage* prior = *(myPlan->begin() + priorIndex);
    //         myStep = myPlan->begin() + priorIndex + 1;
    //         bool waitAtStop = prior->getDestinationStop() != nullptr
    //                           && &prior->getDestinationStop()->getLane().getEdge() != prior->getDestination();
    //         checkAccess(prior, waitAtStop);
    //         //std::cout << " step=" << step << " i=" << getCurrentStageIndex() << " stage=" << getStageSummary(true) << "\n";
    //     } else {
    //         myStep = myPlan->begin() + (int)step;
    //     }
    //     (*myStep)->loadState(this, iss);
    //     for (int i = 1; i < step; i++) {
    //         if ((*myPlan)[i]->getStageType() == MSStageType::DRIVING) {
    //             dynamic_cast<MSStageDriving*>((*myPlan)[i])->setWaitingSince((*myPlan)[i - 1]->getArrived());
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn load_state(&mut self, state: &str) {
        todo!("port MSTransportable::loadState")
    }

    /// C++: `void MSTransportable::MSTransportable(const MSTransportable &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSTransportable) -> Self {
        todo!("port MSTransportable::MSTransportable")
    }

    /// C++: `MSTransportable & MSTransportable::operator=(const MSTransportable &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSTransportable) -> &mut MSTransportable {
        todo!("port MSTransportable::operator=")
    }

}

