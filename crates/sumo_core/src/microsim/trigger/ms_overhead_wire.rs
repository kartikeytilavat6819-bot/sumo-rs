//! Port of microsim/trigger/MSOverheadWire.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::time::{delta_t, time2string, ts};
use utils::traction_wire::circuit::AlphaFlag;
use crate::microsim::ms_globals::overhead_wire_solver;

/// C++ `static std::mutex ow_mutex` (MSOverheadWire.cpp); a spin lock because lock()/unlock() are split calls
static OW_MUTEX: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

fn ow_mutex_lock() {
    while OW_MUTEX.compare_exchange(false, true, std::sync::atomic::Ordering::Acquire, std::sync::atomic::Ordering::Relaxed).is_err() {
        std::hint::spin_loop();
    }
}

fn ow_mutex_unlock() {
    OW_MUTEX.store(false, std::sync::atomic::Ordering::Release);
}

/// C++ `WATT2WATTHR` macro (MSOverheadWire.h)
fn watt2watthr(x: f64) -> f64 {
    x * ts() / 3600.0
}

/// `MSNet::getStoppingPlace("ovrhd_inner_" + lane.getID(), SUMO_TAG_OVERHEAD_WIRE_SEGMENT)` cast to the segment type
fn ovrhd_inner_segment(lane: Ptr<MSLane>) -> Ptr<MSOverheadWire> {
    MSNet::get_instance().get_stopping_place(&format!("ovrhd_inner_{}", lane.m().get_id()), SUMO_TAG_OVERHEAD_WIRE_SEGMENT).cast::<MSOverheadWire>()
}

/// struct to save information for the overhead wire segment output
/// C++ class `MSOverheadWire::Charge` (microsim/trigger/MSOverheadWire.h)
pub struct MSOverheadWire_Charge {
    pub timeStep: SUMOTime,
    pub vehicleID: String,
    pub vehicleType: String,
    /// status
    pub status: String,
    pub WCharged: f64,
    pub actualBatteryCapacity: f64,
    pub maxBatteryCapacity: f64,
    pub voltage: f64,
    pub chargingEfficiency: f64,
    pub totalEnergyCharged: f64,
}

impl MSOverheadWire_Charge {
    /// C++: `void Charge::Charge(SUMOTime, std::string, std::string, std::string, double, double, double, double, double)`
    /// constructor
    // Charge(SUMOTime _timeStep, std::string _vehicleID, std::string _vehicleType, std::string _status,
    //                double _WCharged, double _actualBatteryCapacity, double _maxBatteryCapacity, double _voltage,
    //                double _totalEnergyCharged) :
    //             timeStep(_timeStep),
    //             vehicleID(_vehicleID),
    //             vehicleType(_vehicleType),
    //             status(_status),
    //             WCharged(_WCharged),
    //             actualBatteryCapacity(_actualBatteryCapacity),
    //             maxBatteryCapacity(_maxBatteryCapacity),
    //             voltage(_voltage),
    //             totalEnergyCharged(_totalEnergyCharged) {}
    #[allow(unused_variables)]
    pub fn new(_timeStep: SUMOTime, _vehicleID: String, _vehicleType: String, _status: String, _WCharged: f64, _actualBatteryCapacity: f64, _maxBatteryCapacity: f64, _voltage: f64, _totalEnergyCharged: f64) -> Self {
        MSOverheadWire_Charge {
            timeStep: _timeStep,
            vehicleID: _vehicleID,
            vehicleType: _vehicleType,
            status: _status,
            WCharged: _WCharged,
            actualBatteryCapacity: _actualBatteryCapacity,
            maxBatteryCapacity: _maxBatteryCapacity,
            voltage: _voltage,
            chargingEfficiency: 0.0, // not a C++ member of Charge; unused
            totalEnergyCharged: _totalEnergyCharged,
        }
    }

}

/// Definition of overhead wire segment
/// C++ class `MSOverheadWire` (microsim/trigger/MSOverheadWire.h)
#[repr(C)]
pub struct MSOverheadWire {
    /// C++ base class `MSStoppingPlace`
    pub base: MSStoppingPlace,
    /// Overhead wire's voltage
    pub myVoltage: f64,
    /// Check if in the current TimeStep overheadWireSegment is charging a vehicle
    pub myChargingVehicle: bool,
    /// total energy charged by this charging station
    pub myTotalCharge: f64,
    /// map with the charges of this charging station (key = vehicleID)
    pub myChargeValues: BTreeMap<String, Vec<MSOverheadWire_Charge>>,
    /// order vehicles by time of first charge
    pub myChargedVehicles: Vec<String>,
    pub myChargingVehicles: Vec<Ptr<SUMOVehicle>>,
    /// Parameter, Pointer to the electrical substation (by default is nullptr)
    pub myTractionSubstation: Ptr<MSTractionSubstation>,
    pub myVoltageSource: bool,
    pub myCircuitElementPos: Ptr<Element>,
    pub myCircuitStartNodePos: Ptr<Node>,
    pub myCircuitEndNodePos: Ptr<Node>,
}

impl std::ops::Deref for MSOverheadWire {
    type Target = MSStoppingPlace;
    fn deref(&self) -> &MSStoppingPlace {
        &self.base
    }
}
impl std::ops::DerefMut for MSOverheadWire {
    fn deref_mut(&mut self) -> &mut MSStoppingPlace {
        &mut self.base
    }
}

impl MSOverheadWire {
    /// C++: `void MSOverheadWire::MSOverheadWire(const std::string &, MSLane &, double, double, bool)`
    /// constructor
    // MSOverheadWire::MSOverheadWire(const std::string& overheadWireSegmentID, MSLane& lane, double startPos, double endPos, bool voltageSource) :
    //     MSStoppingPlace(overheadWireSegmentID, SUMO_TAG_OVERHEAD_WIRE_SEGMENT, std::vector<std::string>(), lane, startPos, endPos),
    //     myVoltage(0),
    //     myChargingVehicle(false),
    //     myTotalCharge(0),
    //     myChargingVehicles({}),
    //                    // RICE_TODO: think about some better structure storing circuit pointers below
    //                    myTractionSubstation(nullptr),
    //                    myVoltageSource(voltageSource),
    //                    myCircuitElementPos(nullptr),
    //                    myCircuitStartNodePos(nullptr),
    // myCircuitEndNodePos(nullptr) {
    //     if (getBeginLanePosition() > getEndLanePosition()) {
    //         WRITE_WARNING(toString(SUMO_TAG_OVERHEAD_WIRE_SEGMENT) + " with ID = " + getID() + " doesn't have a valid range (" + toString(getBeginLanePosition()) + " < " + toString(getEndLanePosition()) + ").");
    //     }
    // }
    #[allow(unused_variables)]
    pub fn new(overheadWireSegmentID: &str, lane: &mut MSLane, startPos: f64, endPos: f64, voltageSource: bool) -> Self {
        let base = MSStoppingPlace::new(overheadWireSegmentID, SUMO_TAG_OVERHEAD_WIRE_SEGMENT, &Vec::new(), lane, startPos, endPos, String::new(), 0, 0.0, &RGBColor::INVISIBLE, 90.0);
        let ow = MSOverheadWire {
            base,
            myVoltage: 0.0,
            myChargingVehicle: false,
            myTotalCharge: 0.0,
            myChargeValues: BTreeMap::new(),
            myChargedVehicles: Vec::new(),
            myChargingVehicles: Vec::new(),
            myTractionSubstation: Ptr::null(),
            myVoltageSource: voltageSource,
            myCircuitElementPos: Ptr::null(),
            myCircuitStartNodePos: Ptr::null(),
            myCircuitEndNodePos: Ptr::null(),
        };
        if ow.get_begin_lane_position() > ow.get_end_lane_position() {
            utils::common::msg_handler::write_warning(&utils::tlf!("overheadWireSegment with ID = % doesn't have a valid range (% < %).", ow.get_id(), ow.get_begin_lane_position(), ow.get_end_lane_position()));
        }
        ow
    }

    /// C++: `void MSOverheadWire::~MSOverheadWire() noexcept` [virtual]
    /// destructor
    // MSOverheadWire::~MSOverheadWire() {
    //     if (myTractionSubstation != nullptr) {
    //         Circuit* circuit = myTractionSubstation->getCircuit();
    //         if (circuit != nullptr && myCircuitElementPos != nullptr && myCircuitElementPos->getPosNode() == myCircuitStartNodePos && myCircuitElementPos->getNegNode() == myCircuitEndNodePos) {
    //             circuit->eraseElement(myCircuitElementPos);
    //             delete myCircuitElementPos;
    //             if (myCircuitEndNodePos->getElements()->size() == 0) {
    //                 circuit->eraseNode(myCircuitEndNodePos);
    //                 delete myCircuitEndNodePos;
    //             }
    //             if (myCircuitStartNodePos->getElements()->size() == 0) {
    //                 circuit->eraseNode(myCircuitStartNodePos);
    //                 delete myCircuitStartNodePos;
    //             }
    //         }
    //
    //         if (myTractionSubstation->numberOfOverheadSegments() <= 1) {
    //             myTractionSubstation->eraseOverheadWireSegmentFromCircuit(this);
    //             //RICE_TODO We should "delete myTractionSubstation;" here ...
    //         } else {
    //             myTractionSubstation->eraseOverheadWireSegmentFromCircuit(this);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        if !self.myTractionSubstation.is_null() {
            // shortcut: circuit element/node teardown needs Ptr<Element>/Ptr<Node> handles, but utils::traction_wire::Circuit is arena-indexed (ElemIdx/NodeIdx); upgrade when myCircuitElementPos/StartNodePos/EndNodePos become indices
            // C++ calls eraseOverheadWireSegmentFromCircuit(this) in both branches of numberOfOverheadSegments() <= 1
            let mut sub = self.myTractionSubstation;
            sub.m().erase_overhead_wire_segment_from_circuit(Ptr::from_mut(self));
        }
    }

    /// C++: `double MSOverheadWire::getVoltage() const` [const]
    /// Get overhead wire's voltage
    // double
    // MSOverheadWire::getVoltage() const {
    //     return myVoltage;
    // }
    #[allow(unused_variables)]
    pub fn get_voltage(&self) -> f64 {
        self.myVoltage
    }

    /// C++: `void MSOverheadWire::setVoltage(double)`
    /// Set overhead wire's voltage
    // void
    // MSOverheadWire::setVoltage(double voltage) {
    //     if (voltage < 0) {
    //         WRITE_WARNING("New " + toString(SUMO_ATTR_VOLTAGE) + " for " + toString(SUMO_TAG_OVERHEAD_WIRE_SEGMENT) + " with ID = " + getID() + " isn't valid (" + toString(voltage) + ").")
    //     } else {
    //         myVoltage = voltage;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_voltage(&mut self, voltage: f64) {
        if voltage < 0.0 {
            utils::common::msg_handler::write_warning(&utils::tlf!("New voltage for overheadWireSegment with ID = % isn't valid (%).", self.get_id(), voltage));
        } else {
            self.myVoltage = voltage;
        }
    }

    /// C++: `void MSOverheadWire::setChargingVehicle(bool)`
    /// enable or disable charging vehicle
    // void
    // MSOverheadWire::setChargingVehicle(bool value) {
    //     myChargingVehicle = value;
    // }
    #[allow(unused_variables)]
    pub fn set_charging_vehicle(&mut self, value: bool) {
        self.myChargingVehicle = value;
    }

    /// C++: `bool MSOverheadWire::vehicleIsInside(const double) const` [const]
    /// Check if a vehicle is inside in the Charge Station
    // bool
    // MSOverheadWire::vehicleIsInside(const double position) const {
    //     if ((position >= getBeginLanePosition()) && (position <= getEndLanePosition())) {
    //         return true;
    //     } else {
    //         return false;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn vehicle_is_inside(&self, position: f64) -> bool {
        (position >= self.get_begin_lane_position()) && (position <= self.get_end_lane_position())
    }

    /// C++: `bool MSOverheadWire::isCharging() const` [const]
    /// Return true if in the current time step charging station is charging a vehicle
    // bool
    // MSOverheadWire::isCharging() const {
    //     return myChargingVehicle;
    // }
    #[allow(unused_variables)]
    pub fn is_charging(&self) -> bool {
        self.myChargingVehicle
    }

    /// C++: `void MSOverheadWire::addVehicle(SUMOVehicle &)`
    // void
    // MSOverheadWire::addVehicle(SUMOVehicle& veh) {
    //     std::lock_guard<std::mutex> guard(ow_mutex);
    //     setChargingVehicle(true);
    //     myChargingVehicles.push_back(&veh);
    //     sort(myChargingVehicles.begin(), myChargingVehicles.end(), vehicle_position_sorter());
    // }
    #[allow(unused_variables)]
    pub fn add_vehicle(&mut self, veh: &mut SUMOVehicle) {
        ow_mutex_lock();
        self.set_charging_vehicle(true);
        self.myChargingVehicles.push(Ptr::from_mut(veh));
        // determinism: C++ std::sort is unstable; std_sort reproduces libstdc++ tie order (equal positions tie)
        utils::common::std_sort::std_sort_vec(&mut self.myChargingVehicles, |a, b| MSOverheadWire_vehicle_position_sorter::new().call_op(*a, *b) != 0);
        ow_mutex_unlock();
    }

    /// C++: `void MSOverheadWire::eraseVehicle(SUMOVehicle &)`
    // void
    // MSOverheadWire::eraseVehicle(SUMOVehicle& veh) {
    //     std::lock_guard<std::mutex> guard(ow_mutex);
    //     myChargingVehicles.erase(std::remove(myChargingVehicles.begin(), myChargingVehicles.end(), &veh), myChargingVehicles.end());
    //     if (myChargingVehicles.size() == 0) {
    //         setChargingVehicle(false);
    //     }
    //     //sort(myChargingVehicles.begin(), myChargingVehicles.end(), vehicle_position_sorter());
    // }
    #[allow(unused_variables)]
    pub fn erase_vehicle(&mut self, veh: &mut SUMOVehicle) {
        ow_mutex_lock();
        let target = Ptr::from_mut(veh);
        self.myChargingVehicles.retain(|v| (*v).as_raw() != target.as_raw());
        if self.myChargingVehicles.len() == 0 {
            self.set_charging_vehicle(false);
        }
        ow_mutex_unlock();
    }

    /// C++: `int MSOverheadWire::getElecHybridCount() const` [const]
    // int getElecHybridCount() const {
    //         return (int)myChargingVehicles.size();
    //     }
    #[allow(unused_variables)]
    pub fn get_elec_hybrid_count(&self) -> i32 {
        self.myChargingVehicles.len() as i32
    }

    /// C++: `const std::vector<SUMOVehicle *> & MSOverheadWire::getChargingVehicles() const` [const]
    // const std::vector<SUMOVehicle*>& getChargingVehicles() const {
    //         return myChargingVehicles;
    //     }
    #[allow(unused_variables)]
    pub fn get_charging_vehicles(&self) -> &Vec<Ptr<SUMOVehicle>> {
        &self.myChargingVehicles
    }

    /// C++: `double MSOverheadWire::getTotalCharged() const` [const]
    // double getTotalCharged() const {
    //         return myTotalCharge;
    //     }
    #[allow(unused_variables)]
    pub fn get_total_charged(&self) -> f64 {
        self.myTotalCharge
    }

    /// C++: `void MSOverheadWire::addChargeValueForOutput(double, MSDevice_ElecHybrid *, bool)`
    /// add charge value for output
    // void
    // MSOverheadWire::addChargeValueForOutput(double WCharged, MSDevice_ElecHybrid* elecHybrid, bool ischarging) {
    //     std::string status = "charging";
    //     if (!ischarging) {
    //         status = "not-charging";
    //     }
    //
    //     // update total charge
    //     myTotalCharge += WCharged;
    //     // create charge row and insert it in myChargeValues
    //     const std::string vehID = elecHybrid->getHolder().getID();
    //     if (myChargeValues.count(vehID) == 0) {
    //         myChargedVehicles.push_back(vehID);
    //     }
    //     Charge C(MSNet::getInstance()->getCurrentTimeStep(), elecHybrid->getHolder().getID(), elecHybrid->getHolder().getVehicleType().getID(),
    //              status, WCharged, elecHybrid->getActualBatteryCapacity(), elecHybrid->getMaximumBatteryCapacity(),
    //              elecHybrid->getVoltageOfOverheadWire(), myTotalCharge);
    //     myChargeValues[vehID].push_back(C);
    // }
    #[allow(unused_variables)]
    pub fn add_charge_value_for_output(&mut self, WCharged: f64, elecHybrid: Ptr<MSDevice_ElecHybrid>, ischarging: bool) {
        let status: &str = if ischarging { "charging" } else { "not-charging" };
        // update total charge
        self.myTotalCharge += WCharged;
        // create charge row and insert it in myChargeValues
        let vehID: String = elecHybrid.m().get_holder().get_id().to_string();
        if !self.myChargeValues.contains_key(&vehID) {
            self.myChargedVehicles.push(vehID.clone());
        }
        let C = MSOverheadWire_Charge::new(
            MSNet::get_instance().get_current_time_step(),
            vehID.clone(),
            elecHybrid.m().get_holder().get_vehicle_type().get_id().to_string(),
            status.to_string(),
            WCharged,
            elecHybrid.m().get_actual_battery_capacity(),
            elecHybrid.m().get_maximum_battery_capacity(),
            elecHybrid.m().get_voltage_of_overhead_wire(),
            self.myTotalCharge,
        );
        self.myChargeValues.entry(vehID).or_default().push(C);
    }

    /// C++: `void MSOverheadWire::writeOverheadWireSegmentOutput(OutputDevice &)`
    /// write overhead wire segment values
    // void
    // MSOverheadWire::writeOverheadWireSegmentOutput(OutputDevice& output) {
    //     int chargingSteps = 0;
    //     std::vector<SUMOTime> chargingSteps_list;
    //     for (const auto& item : myChargeValues) {
    //         for (auto it : item.second) {
    //             if (std::find(chargingSteps_list.begin(), chargingSteps_list.end(), it.timeStep) == chargingSteps_list.end()) {
    //                 chargingSteps_list.push_back(it.timeStep);
    //             }
    //         }
    //     }
    //     chargingSteps = (int) chargingSteps_list.size();
    //     output.openTag(SUMO_TAG_OVERHEAD_WIRE_SEGMENT);
    //     output.writeAttr(SUMO_ATTR_ID, myID);
    //     if (getTractionSubstation() != nullptr) {
    //         output.writeAttr(SUMO_ATTR_TRACTIONSUBSTATIONID, getTractionSubstation()->getID());
    //     } else {
    //         output.writeAttr(SUMO_ATTR_TRACTIONSUBSTATIONID, "");
    //     }
    //     output.writeAttr(SUMO_ATTR_TOTALENERGYCHARGED, myTotalCharge);
    //
    //     // RICE_TODO QUESTION myChargeValues.size() vs. chargingSteps
    //     // myChargeValues.size() is the number of vehicles charging sometimes from this overheadwire segment during simulation
    //     // chargingSteps is now the sum of chargingSteps of each vehicle, but takes also into account that at the given
    //     // step more than one vehicle may be charged from this segment
    //     output.writeAttr(SUMO_ATTR_CHARGINGSTEPS, chargingSteps);
    //     // output.writeAttr(SUMO_ATTR_EDGE, getLane().getEdge());
    //     output.writeAttr(SUMO_ATTR_LANE, getLane().getID());
    //
    //     // Start writing
    //     if (myChargeValues.size() > 0) {
    //         for (const std::string& vehID : myChargedVehicles) {
    //             int iStart = 0;
    //             const auto& chargeSteps = myChargeValues[vehID];
    //             while (iStart < (int)chargeSteps.size()) {
    //                 int iEnd = iStart + 1;
    //                 double charged = chargeSteps[iStart].WCharged;
    //                 while (iEnd < (int)chargeSteps.size() && chargeSteps[iEnd].timeStep == chargeSteps[iEnd - 1].timeStep + DELTA_T) {
    //                     charged += chargeSteps[iEnd].WCharged;
    //                     iEnd++;
    //                 }
    //                 writeVehicle(output, chargeSteps, iStart, iEnd, charged);
    //                 iStart = iEnd;
    //             }
    //         }
    //     }
    //     // close charging station tag
    //     output.closeTag();
    // }
    #[allow(unused_variables)]
    pub fn write_overhead_wire_segment_output(&mut self, output: &mut OutputDevice) {
        let mut chargingSteps_list: Vec<SUMOTime> = Vec::new();
        for item in self.myChargeValues.values() {
            for it in item.iter() {
                if !chargingSteps_list.contains(&it.timeStep) {
                    chargingSteps_list.push(it.timeStep);
                }
            }
        }
        let chargingSteps: i32 = chargingSteps_list.len() as i32;
        output.open_tag_id(SUMO_TAG_OVERHEAD_WIRE_SEGMENT);
        output.write_attr(SUMO_ATTR_ID, self.get_id());
        let substationID: String = if !self.myTractionSubstation.is_null() {
            self.myTractionSubstation.m().get_id().to_string()
        } else {
            String::new()
        };
        output.write_attr(SUMO_ATTR_TRACTIONSUBSTATIONID, substationID.as_str());
        output.write_attr(SUMO_ATTR_TOTALENERGYCHARGED, &self.myTotalCharge);
        // RICE_TODO QUESTION myChargeValues.size() vs. chargingSteps
        output.write_attr(SUMO_ATTR_CHARGINGSTEPS, &chargingSteps);
        output.write_attr(SUMO_ATTR_LANE, self.get_lane().get_id());

        // Start writing
        if self.myChargeValues.len() > 0 {
            for vehID in self.myChargedVehicles.iter() {
                let mut iStart: i32 = 0;
                let chargeSteps = self.myChargeValues.entry(vehID.clone()).or_default();
                while iStart < chargeSteps.len() as i32 {
                    let mut iEnd: i32 = iStart + 1;
                    let mut charged: f64 = chargeSteps[iStart as usize].WCharged;
                    while iEnd < chargeSteps.len() as i32 && chargeSteps[iEnd as usize].timeStep == chargeSteps[(iEnd - 1) as usize].timeStep + delta_t() {
                        charged += chargeSteps[iEnd as usize].WCharged;
                        iEnd += 1;
                    }
                    MSOverheadWire::write_vehicle(output, chargeSteps, iStart, iEnd, charged);
                    iStart = iEnd;
                }
            }
        }
        // close charging station tag
        output.close_tag("");
    }

    /// C++: `std::string MSOverheadWire::getOverheadWireSegmentName()`
    // std::string MSOverheadWire::getOverheadWireSegmentName() {
    //     return toString(getID());
    // }
    #[allow(unused_variables)]
    pub fn get_overhead_wire_segment_name(&mut self) -> String {
        self.get_id().to_string()
    }

    /// C++: `MSTractionSubstation * MSOverheadWire::getTractionSubstation() const` [const]
    // MSTractionSubstation* getTractionSubstation() const {
    //         return myTractionSubstation;
    //     }
    #[allow(unused_variables)]
    pub fn get_traction_substation(&self) -> Ptr<MSTractionSubstation> {
        self.myTractionSubstation
    }

    /// C++: `void MSOverheadWire::setTractionSubstation(MSTractionSubstation *)`
    // void setTractionSubstation(MSTractionSubstation* substation) {
    //         myTractionSubstation = substation;
    //     }
    #[allow(unused_variables)]
    pub fn set_traction_substation(&mut self, substation: Ptr<MSTractionSubstation>) {
        self.myTractionSubstation = substation;
    }

    /// C++: `Circuit * MSOverheadWire::getCircuit() const` [const]
    // Circuit*
    // MSOverheadWire::getCircuit() const {
    //     if (getTractionSubstation() != nullptr) {
    //         return getTractionSubstation()->getCircuit();
    //     }
    //     return nullptr;
    // }
    #[allow(unused_variables)]
    pub fn get_circuit(&self) -> Ptr<Circuit> {
        if !self.myTractionSubstation.is_null() {
            return self.myTractionSubstation.m().get_circuit();
        }
        Ptr::null()
    }

    /// C++: `void MSOverheadWire::setCircuitStartNodePos(Node *)`
    // void setCircuitStartNodePos(Node* node) {
    //         myCircuitStartNodePos = node;
    //     }
    #[allow(unused_variables)]
    pub fn set_circuit_start_node_pos(&mut self, node: Ptr<Node>) {
        self.myCircuitStartNodePos = node;
    }

    /// C++: `void MSOverheadWire::setCircuitEndNodePos(Node *)`
    // void setCircuitEndNodePos(Node* node) {
    //         myCircuitEndNodePos = node;
    //     }
    #[allow(unused_variables)]
    pub fn set_circuit_end_node_pos(&mut self, node: Ptr<Node>) {
        self.myCircuitEndNodePos = node;
    }

    /// C++: `void MSOverheadWire::setCircuitElementPos(Element *)`
    // void setCircuitElementPos(Element* element) {
    //         myCircuitElementPos = element;
    //     }
    #[allow(unused_variables)]
    pub fn set_circuit_element_pos(&mut self, element: Ptr<Element>) {
        self.myCircuitElementPos = element;
    }

    /// C++: `Node * MSOverheadWire::getCircuitStartNodePos() const` [const]
    // Node* getCircuitStartNodePos() const {
    //         return myCircuitStartNodePos;
    //     }
    #[allow(unused_variables)]
    pub fn get_circuit_start_node_pos(&self) -> Ptr<Node> {
        self.myCircuitStartNodePos
    }

    /// C++: `Node * MSOverheadWire::getCircuitEndNodePos() const` [const]
    // Node* getCircuitEndNodePos() const {
    //         return myCircuitEndNodePos;
    //     }
    #[allow(unused_variables)]
    pub fn get_circuit_end_node_pos(&self) -> Ptr<Node> {
        self.myCircuitEndNodePos
    }

    /// C++: `Element * MSOverheadWire::getCircuitElementPos() const` [const]
    // Element* getCircuitElementPos() const {
    //         return myCircuitElementPos;
    //     }
    #[allow(unused_variables)]
    pub fn get_circuit_element_pos(&self) -> Ptr<Element> {
        self.myCircuitElementPos
    }

    /// C++: `bool MSOverheadWire::isThereVoltageSource() const` [const]
    // bool isThereVoltageSource() const {
    //         return myVoltageSource;
    //     }
    #[allow(unused_variables)]
    pub fn is_there_voltage_source(&self) -> bool {
        self.myVoltageSource
    }

    /// C++: `void MSOverheadWire::lock() const` [const]
    // void
    // MSOverheadWire::lock() const {
    //     ow_mutex.lock();
    // }
    #[allow(unused_variables)]
    pub fn lock(&self) {
        ow_mutex_lock();
    }

    /// C++: `void MSOverheadWire::unlock() const` [const]
    // void
    // MSOverheadWire::unlock() const {
    //     ow_mutex.unlock();
    // }
    #[allow(unused_variables)]
    pub fn unlock(&self) {
        ow_mutex_unlock();
    }

    /// C++: `void MSOverheadWire::writeVehicle(OutputDevice &, const std::vector<Charge> &, int, int, double)` [static, protected]
    // void
    // MSOverheadWire::writeVehicle(OutputDevice& out, const std::vector<Charge>& chargeSteps, int iStart, int iEnd, double charged) {
    //     const Charge& first = chargeSteps[iStart];
    //     out.openTag(SUMO_TAG_VEHICLE);
    //     out.writeAttr(SUMO_ATTR_ID, first.vehicleID);
    //     out.writeAttr(SUMO_ATTR_TYPE, first.vehicleType);
    //     out.writeAttr(SUMO_ATTR_TOTALENERGYCHARGED_VEHICLE, charged);
    //     out.writeAttr(SUMO_ATTR_CHARGINGBEGIN, time2string(first.timeStep));
    //     out.writeAttr(SUMO_ATTR_CHARGINGEND, time2string(chargeSteps[iEnd - 1].timeStep));
    //     out.writeAttr(SUMO_ATTR_MAXIMUMBATTERYCAPACITY, first.maxBatteryCapacity);
    //     for (int i = iStart; i < iEnd; i++) {
    //         const Charge& c = chargeSteps[i];
    //         out.openTag(SUMO_TAG_STEP);
    //         out.writeAttr(SUMO_ATTR_TIME, time2string(c.timeStep));
    //         // charge values
    //         out.writeAttr(SUMO_ATTR_CHARGING_STATUS, c.status);
    //         out.writeAttr(SUMO_ATTR_ENERGYCHARGED, c.WCharged);
    //         out.writeAttr(SUMO_ATTR_PARTIALCHARGE, c.totalEnergyCharged);
    //         // charging values of charging station in this timestep
    //         out.writeAttr(SUMO_ATTR_VOLTAGE, c.voltage);
    //         // battery status of vehicle
    //         out.writeAttr(SUMO_ATTR_ACTUALBATTERYCAPACITY, c.actualBatteryCapacity);
    //         // close tag timestep
    //         out.closeTag();
    //     }
    //     out.closeTag();
    // }
    #[allow(unused_variables)]
    pub fn write_vehicle(out: &mut OutputDevice, chargeSteps: &Vec<MSOverheadWire_Charge>, iStart: i32, iEnd: i32, charged: f64) {
        let first = &chargeSteps[iStart as usize];
        out.open_tag_id(SUMO_TAG_VEHICLE);
        out.write_attr(SUMO_ATTR_ID, first.vehicleID.as_str());
        out.write_attr(SUMO_ATTR_TYPE, first.vehicleType.as_str());
        out.write_attr(SUMO_ATTR_TOTALENERGYCHARGED_VEHICLE, &charged);
        out.write_attr(SUMO_ATTR_CHARGINGBEGIN, &time2string(first.timeStep));
        out.write_attr(SUMO_ATTR_CHARGINGEND, &time2string(chargeSteps[(iEnd - 1) as usize].timeStep));
        out.write_attr(SUMO_ATTR_MAXIMUMBATTERYCAPACITY, &first.maxBatteryCapacity);
        for i in iStart..iEnd {
            let c = &chargeSteps[i as usize];
            out.open_tag_id(SUMO_TAG_STEP);
            out.write_attr(SUMO_ATTR_TIME, &time2string(c.timeStep));
            // charge values
            out.write_attr(SUMO_ATTR_CHARGING_STATUS, c.status.as_str());
            out.write_attr(SUMO_ATTR_ENERGYCHARGED, &c.WCharged);
            out.write_attr(SUMO_ATTR_PARTIALCHARGE, &c.totalEnergyCharged);
            // charging values of charging station in this timestep
            out.write_attr(SUMO_ATTR_VOLTAGE, &c.voltage);
            // battery status of vehicle
            out.write_attr(SUMO_ATTR_ACTUALBATTERYCAPACITY, &c.actualBatteryCapacity);
            // close tag timestep
            out.close_tag("");
        }
        out.close_tag("");
    }

    /// C++: `void MSOverheadWire::MSOverheadWire(const MSOverheadWire &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSOverheadWire) -> Self {
        // C++ copy constructor is deleted ("Invalidated"), copying is a programming error
        panic!("MSOverheadWire copy constructor is invalidated in C++ and must not be used");
    }

    /// C++: `MSOverheadWire & MSOverheadWire::operator=(const MSOverheadWire &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSOverheadWire) -> &mut MSOverheadWire {
        // C++ assignment operator is deleted ("Invalidated"), assignment is a programming error
        panic!("MSOverheadWire assignment operator is invalidated in C++ and must not be used");
    }

}

/// Traction substation powering one or more overhead wire sections
/// C++ class `MSTractionSubstation` (microsim/trigger/MSOverheadWire.h)
#[repr(C)]
pub struct MSTractionSubstation {
    /// C++ base class `Named`
    pub base: Named,
    pub myChargeValues: Vec<MSTractionSubstation_chargeTS>,
    /// Check if in the current TimeStep substation (overhead wire section) is charging a vehicle
    pub myChargingVehicle: bool,
    pub myElecHybridCount: i32,
    pub mySubstationVoltage: f64,
    pub myCircuit: Ptr<Circuit>,
    pub myOverheadWireSegments: Vec<Ptr<MSOverheadWire>>,
    pub myElecHybrid: Vec<Ptr<MSDevice_ElecHybrid>>,
    pub myForbiddenLanes: Vec<Ptr<MSLane>>,
    pub myTotalEnergy: f64,
    pub myOverheadWireClamps: Vec<MSTractionSubstation_OverheadWireClamp>,
}

impl std::ops::Deref for MSTractionSubstation {
    type Target = Named;
    fn deref(&self) -> &Named {
        &self.base
    }
}
impl std::ops::DerefMut for MSTractionSubstation {
    fn deref_mut(&mut self) -> &mut Named {
        &mut self.base
    }
}

// C++ static members of MSTractionSubstation (port as module-level statics, see AGENTS.md):
//   static Command * MSTractionSubstation::myCommandForSolvingCircuit;

impl MSTractionSubstation {
    /// C++: `void MSTractionSubstation::MSTractionSubstation(const std::string &, double, double)`
    /// Constructor instantiates a substation providing certain voltage and a maximum current
    // MSTractionSubstation::MSTractionSubstation(const std::string& substationId, double voltage, double currentLimit) :
    //     Named(substationId),
    //     myChargingVehicle(false),
    //     myElecHybridCount(0),
    //     mySubstationVoltage(voltage),
    //     myCircuit(new Circuit(currentLimit)),
    //     myTotalEnergy(0)
    // {}
    #[allow(unused_variables)]
    pub fn new(substationId: &str, voltage: f64, currentLimit: f64) -> Self {
        MSTractionSubstation {
            base: Named::new(substationId),
            myChargeValues: Vec::new(),
            myChargingVehicle: false,
            myElecHybridCount: 0,
            mySubstationVoltage: voltage,
            // C++ never deletes myCircuit (the destructor body is empty)
            myCircuit: Ptr::from_mut(Box::leak(Box::new(Circuit::with_current_limit(currentLimit)))),
            myOverheadWireSegments: Vec::new(),
            myElecHybrid: Vec::new(),
            myForbiddenLanes: Vec::new(),
            myTotalEnergy: 0.0,
            myOverheadWireClamps: Vec::new(),
        }
    }

    /// C++: `void MSTractionSubstation::~MSTractionSubstation() noexcept` [virtual]
    /// destructor
    // MSTractionSubstation::~MSTractionSubstation() {
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty

    }

    /// C++: `Circuit * MSTractionSubstation::getCircuit() const` [const]
    // Circuit* getCircuit() const {
    //         return myCircuit;
    //     }
    #[allow(unused_variables)]
    pub fn get_circuit(&self) -> Ptr<Circuit> {
        self.myCircuit
    }

    /// C++: `void MSTractionSubstation::addOverheadWireSegmentToCircuit(MSOverheadWire *)`
    // void
    // MSTractionSubstation::addOverheadWireSegmentToCircuit(MSOverheadWire* newOverheadWireSegment) {
    //     MSLane& lane = const_cast<MSLane&>(newOverheadWireSegment->getLane());
    //     if (lane.isInternal()) {
    //         return;
    //     }
    //
    //     // RICE_TODO: consider the possibility of having more segments that belong to one lane.
    //
    //     myOverheadWireSegments.push_back(newOverheadWireSegment);
    //     newOverheadWireSegment->setTractionSubstation(this);
    //
    //     if (MSGlobals::gOverheadWireSolver) {
    // #ifdef HAVE_EIGEN
    //         Circuit* circuit = newOverheadWireSegment->getCircuit();
    //         const std::string segmentID = newOverheadWireSegment->getID();
    //
    //         if (circuit->getNode("negNode_ground") == nullptr) {
    //             circuit->addNode("negNode_ground");
    //         }
    //
    //         // convention: pNode is at the beginning of the wire segment, nNode is at the end of the wire segment
    //         newOverheadWireSegment->setCircuitStartNodePos(circuit->addNode("pNode_pos_" + segmentID));
    //         newOverheadWireSegment->setCircuitEndNodePos(circuit->addNode("nNode_pos_" + segmentID));
    //         // RICE_TODO: to use startPos and endPos of ovhdsegment: set the length of wire here properly
    //         newOverheadWireSegment->setCircuitElementPos(
    //             circuit->addElement("pos_" + segmentID,
    //                                 (newOverheadWireSegment->getLane().getLength()) * WIRE_RESISTIVITY,
    //                                 newOverheadWireSegment->getCircuitStartNodePos(),
    //                                 newOverheadWireSegment->getCircuitEndNodePos(),
    //                                 Element::ElementType::RESISTOR_traction_wire));
    // #else
    //         WRITE_WARNING(TL("Overhead circuit solver requested, but solver support (Eigen) not compiled in."));
    // #endif
    //     }
    //
    //     const MSLane* connection = nullptr;
    //     std::string ovrhdSegmentID = ""; //ID of outgoing or incoming overhead wire segment
    //     MSOverheadWire* ovrhdSegment = nullptr; //pointer to outgoing or incoming overhead wire segment
    //
    //     // RICE_TODO: simplify the code, two similar code-blocks below
    //     // RICE_TODO: to use startPos and endPos of ovhdsegment: if endPos+EPS > newOverheadWireSegment->getLane().getLength(),
    //     //            and the outgoing lanes will be skipped as there is no wire at the end of the lane
    //
    //     /* in version before SUMO 1.0.1 the function getOutgoingLanes() returning MSLane* exists,
    //        in new version of SUMO the funciton getOutgoingViaLanes() returning MSLane* and MSEdge* pair exists */
    //     // std::vector<const MSLane*> outgoing = lane.getOutgoingLanes();
    //     const std::vector<std::pair<const MSLane*, const MSEdge*> > outgoingLanesAndEdges = lane.getOutgoingViaLanes();
    //     std::vector<const MSLane*> neigboringInnerLanes;
    //     neigboringInnerLanes.reserve(outgoingLanesAndEdges.size());
    //     for (size_t it = 0; it < outgoingLanesAndEdges.size(); ++it) {
    //         neigboringInnerLanes.push_back(outgoingLanesAndEdges[it].first);
    //     }
    //
    //     // Check if there is an overhead wire segment on the outgoing lane. If not, do nothing, otherwise find connnecting internal lanes and
    //     // add all lanes (this and inner) to circuit
    //     for (std::vector<const MSLane*>::iterator it = neigboringInnerLanes.begin(); it != neigboringInnerLanes.end(); ++it) {
    //         ovrhdSegmentID = MSNet::getInstance()->getStoppingPlaceID(*it, NUMERICAL_EPS, SUMO_TAG_OVERHEAD_WIRE_SEGMENT);
    //         // If the overhead wire segment is over the outgoing (not internal) lane
    //         if (ovrhdSegmentID != "" && !(*it)->isInternal()) {
    //             ovrhdSegment = dynamic_cast<MSOverheadWire*>(MSNet::getInstance()->getStoppingPlace(ovrhdSegmentID, SUMO_TAG_OVERHEAD_WIRE_SEGMENT));
    //             // If the outgoing overhead wire segment belongs to the same substation as newOverheadWireSegment
    //             // RICE_TODO: define what happens if the traction stations are different (overhead wire should continue over inner segments but it is unclear to which traction substation or even circuit it should be connected)
    //             if (ovrhdSegment->getTractionSubstation() == newOverheadWireSegment->getTractionSubstation()) {
    //                 connection = lane.getInternalFollowingLane(*it);
    //                 if (connection != nullptr) {
    //                     //is connection a forbidden lane?
    //                     if (!(ovrhdSegment->getTractionSubstation()->isForbidden(connection) ||
    //                             ovrhdSegment->getTractionSubstation()->isForbidden(lane.getInternalFollowingLane(connection)) ||
    //                             ovrhdSegment->getTractionSubstation()->isForbidden(connection->getInternalFollowingLane(*it)))) {
    //                         addOverheadWireInnerSegmentToCircuit(newOverheadWireSegment, ovrhdSegment, connection, lane.getInternalFollowingLane(connection), connection->getInternalFollowingLane(*it));
    //                     }
    //
    //                 } else {
    //                     if (MSGlobals::gOverheadWireSolver) {
    // #ifdef HAVE_EIGEN
    //                         Node* const unusedNode = newOverheadWireSegment->getCircuitEndNodePos();
    //                         for (MSOverheadWire* const ows : myOverheadWireSegments) {
    //                             if (ows->getCircuitStartNodePos() == unusedNode) {
    //                                 ows->setCircuitStartNodePos(ovrhdSegment->getCircuitStartNodePos());
    //                             }
    //                             if (ows->getCircuitEndNodePos() == unusedNode) {
    //                                 ows->setCircuitEndNodePos(ovrhdSegment->getCircuitStartNodePos());
    //                             }
    //                         }
    //                         newOverheadWireSegment->getCircuit()->replaceAndDeleteNode(unusedNode, ovrhdSegment->getCircuitStartNodePos());
    // #else
    //                         WRITE_WARNING(TL("Overhead circuit solver requested, but solver support (Eigen) not compiled in."));
    // #endif
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //
    //     // RICE_TODO: to use startPos and endPos of ovhdsegment: if startPos-EPS < 0,
    //     //            and the incoming lanes will be skipped as there is no wire at the beginning of the lane
    //
    //     // This is the same as above, only this time checking the wires on some incoming lanes. If some of them
    //     // has an overhead wire segment, find the connnecting internal lanes and add all lanes (the internal
    //     // and this) to the circuit, otherwise do nothing.
    //     neigboringInnerLanes = lane.getNormalIncomingLanes();
    //     for (std::vector<const MSLane*>::iterator it = neigboringInnerLanes.begin(); it != neigboringInnerLanes.end(); ++it) {
    //         ovrhdSegmentID = MSNet::getInstance()->getStoppingPlaceID(*it, (*it)->getLength() - NUMERICAL_EPS, SUMO_TAG_OVERHEAD_WIRE_SEGMENT);
    //         // If the overhead wire segment is over the incoming (not internal) lane
    //         if (ovrhdSegmentID != "" && !(*it)->isInternal()) {
    //             ovrhdSegment = dynamic_cast<MSOverheadWire*>(MSNet::getInstance()->getStoppingPlace(ovrhdSegmentID, SUMO_TAG_OVERHEAD_WIRE_SEGMENT));
    //             // If the incoming overhead wire segment belongs to the same substation as newOverheadWireSegment
    //             // RICE_TODO: define what happens if the traction stations are different (overhead wire should continue over inner segments but it is unclear to which traction substation or even circuit it should be connected)
    //             if (ovrhdSegment->getTractionSubstation() == newOverheadWireSegment->getTractionSubstation()) {
    //                 connection = (*it)->getInternalFollowingLane(&lane);
    //                 if (connection != nullptr) {
    //                     //is connection a forbidden lane?
    //                     if (!(ovrhdSegment->getTractionSubstation()->isForbidden(connection) ||
    //                             ovrhdSegment->getTractionSubstation()->isForbidden((*it)->getInternalFollowingLane(connection)) ||
    //                             ovrhdSegment->getTractionSubstation()->isForbidden(connection->getInternalFollowingLane(&lane)))) {
    //                         addOverheadWireInnerSegmentToCircuit(ovrhdSegment, newOverheadWireSegment, connection, (*it)->getInternalFollowingLane(connection), connection->getInternalFollowingLane(&lane));
    //                     }
    //                 } else {
    //                     if (MSGlobals::gOverheadWireSolver) {
    // #ifdef HAVE_EIGEN
    //                         Node* const unusedNode = newOverheadWireSegment->getCircuitStartNodePos();
    //                         for (MSOverheadWire* const ows : myOverheadWireSegments) {
    //                             if (ows->getCircuitStartNodePos() == unusedNode) {
    //                                 ows->setCircuitStartNodePos(ovrhdSegment->getCircuitEndNodePos());
    //                             }
    //                             if (ows->getCircuitEndNodePos() == unusedNode) {
    //                                 ows->setCircuitEndNodePos(ovrhdSegment->getCircuitEndNodePos());
    //                             }
    //                         }
    //                         newOverheadWireSegment->getCircuit()->replaceAndDeleteNode(unusedNode, ovrhdSegment->getCircuitEndNodePos());
    // #else
    //                         WRITE_WARNING(TL("Overhead circuit solver requested, but solver support (Eigen) not compiled in."));
    // #endif
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //
    //     if (MSGlobals::gOverheadWireSolver && newOverheadWireSegment->isThereVoltageSource()) {
    // #ifdef HAVE_EIGEN
    //         newOverheadWireSegment->getCircuit()->addElement(
    //             "voltage_source_" + newOverheadWireSegment->getID(),
    //             mySubstationVoltage,
    //             newOverheadWireSegment->getCircuitStartNodePos(),
    //             newOverheadWireSegment->getCircuit()->getNode("negNode_ground"),
    //             Element::ElementType::VOLTAGE_SOURCE_traction_wire);
    // #else
    //         WRITE_WARNING(TL("Overhead circuit solver requested, but solver support (Eigen) not compiled in."));
    // #endif
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_overhead_wire_segment_to_circuit(&mut self, newOverheadWireSegment: Ptr<MSOverheadWire>) {
        let lane: Ptr<MSLane> = Ptr::new(newOverheadWireSegment.m().get_lane());
        if lane.m().is_internal() {
            return;
        }

        // RICE_TODO: consider the possibility of having more segments that belong to one lane.

        self.myOverheadWireSegments.push(newOverheadWireSegment);
        newOverheadWireSegment.m().set_traction_substation(Ptr::from_mut(self));

        if overhead_wire_solver() {
            // shortcut: circuit nodes/elements of the segment need Ptr<Node>/Ptr<Element> handles, but utils::traction_wire::Circuit is arena-indexed (NodeIdx/ElemIdx); upgrade when the MSOverheadWire circuit fields become indices
        }

        /* in version before SUMO 1.0.1 the function getOutgoingLanes() returning MSLane* exists,
           in new version of SUMO the funciton getOutgoingViaLanes() returning MSLane* and MSEdge* pair exists */
        let outgoingLanesAndEdges = lane.m().get_outgoing_via_lanes();
        let mut neigboringInnerLanes: Vec<Ptr<MSLane>> = outgoingLanesAndEdges.iter().map(|p| p.0).collect();

        // Check if there is an overhead wire segment on the outgoing lane. If not, do nothing, otherwise find connnecting internal lanes and
        // add all lanes (this and inner) to circuit
        for it in neigboringInnerLanes.iter() {
            let it: Ptr<MSLane> = *it;
            let ovrhdSegmentID: String = MSNet::get_instance().get_stopping_place_id(it, utils::geom::NUMERICAL_EPS, SUMO_TAG_OVERHEAD_WIRE_SEGMENT);
            // If the overhead wire segment is over the outgoing (not internal) lane
            if !ovrhdSegmentID.is_empty() && !it.m().is_internal() {
                let ovrhdSegment: Ptr<MSOverheadWire> = MSNet::get_instance().get_stopping_place(&ovrhdSegmentID, SUMO_TAG_OVERHEAD_WIRE_SEGMENT).cast::<MSOverheadWire>();
                // If the outgoing overhead wire segment belongs to the same substation as newOverheadWireSegment
                // RICE_TODO: define what happens if the traction stations are different
                if ovrhdSegment.m().get_traction_substation().as_raw() == newOverheadWireSegment.m().get_traction_substation().as_raw() {
                    let connection: Ptr<MSLane> = lane.m().get_internal_following_lane(it);
                    if !connection.is_null() {
                        // is connection a forbidden lane?
                        let sub = ovrhdSegment.m().get_traction_substation();
                        if !(sub.m().is_forbidden(connection) || sub.m().is_forbidden(lane.m().get_internal_following_lane(connection)) || sub.m().is_forbidden(connection.m().get_internal_following_lane(it))) {
                            self.add_overhead_wire_inner_segment_to_circuit(newOverheadWireSegment, ovrhdSegment, connection, lane.m().get_internal_following_lane(connection), connection.m().get_internal_following_lane(it));
                        }
                    } else if overhead_wire_solver() {
                        // shortcut: relinking the unused circuit node to the outgoing segment needs circuit node handles (see above); upgrade with the circuit node-handle change
                    }
                }
            }
        }

        // RICE_TODO: to use startPos and endPos of ovhdsegment: if startPos-EPS < 0,
        //            and the incoming lanes will be skipped as there is no wire at the beginning of the lane

        // This is the same as above, only this time checking the wires on some incoming lanes.
        neigboringInnerLanes = lane.m().get_normal_incoming_lanes();
        for it in neigboringInnerLanes.iter() {
            let it: Ptr<MSLane> = *it;
            let ovrhdSegmentID: String = MSNet::get_instance().get_stopping_place_id(it, it.m().get_length() - utils::geom::NUMERICAL_EPS, SUMO_TAG_OVERHEAD_WIRE_SEGMENT);
            // If the overhead wire segment is over the incoming (not internal) lane
            if !ovrhdSegmentID.is_empty() && !it.m().is_internal() {
                let ovrhdSegment: Ptr<MSOverheadWire> = MSNet::get_instance().get_stopping_place(&ovrhdSegmentID, SUMO_TAG_OVERHEAD_WIRE_SEGMENT).cast::<MSOverheadWire>();
                // If the incoming overhead wire segment belongs to the same substation as newOverheadWireSegment
                if ovrhdSegment.m().get_traction_substation().as_raw() == newOverheadWireSegment.m().get_traction_substation().as_raw() {
                    let connection: Ptr<MSLane> = it.m().get_internal_following_lane(lane);
                    if !connection.is_null() {
                        // is connection a forbidden lane?
                        let sub = ovrhdSegment.m().get_traction_substation();
                        if !(sub.m().is_forbidden(connection) || sub.m().is_forbidden(it.m().get_internal_following_lane(connection)) || sub.m().is_forbidden(connection.m().get_internal_following_lane(lane))) {
                            self.add_overhead_wire_inner_segment_to_circuit(ovrhdSegment, newOverheadWireSegment, connection, it.m().get_internal_following_lane(connection), connection.m().get_internal_following_lane(lane));
                        }
                    } else if overhead_wire_solver() {
                        // shortcut: relinking the unused circuit node to the incoming segment needs circuit node handles (see above); upgrade with the circuit node-handle change
                    }
                }
            }
        }

        if overhead_wire_solver() && newOverheadWireSegment.m().is_there_voltage_source() {
            // shortcut: the voltage source element of the segment needs circuit node/element handles (see above); upgrade with the circuit node-handle change
        }
    }

    /// C++: `void MSTractionSubstation::addOverheadWireClampToCircuit(const std::string, MSOverheadWire *, MSOverheadWire *)`
    // void MSTractionSubstation::addOverheadWireClampToCircuit(const std::string id, MSOverheadWire* startSegment, MSOverheadWire* endSegment) {
    //     PositionVector pos_start = startSegment->getLane().getShape();
    //     PositionVector pos_end = endSegment->getLane().getShape();
    //     double distance = pos_start[0].distanceTo2D(pos_end.back());
    //
    //     if (distance > 10) {
    //         WRITE_WARNING("The distance between two overhead wires during adding overhead wire clamp '" + id + "' defined for traction substation '" + startSegment->getTractionSubstation()->getID() + "' is " + toString(distance) + " m.")
    //     }
    //     getCircuit()->addElement(id, distance * WIRE_RESISTIVITY, startSegment->getCircuitStartNodePos(), endSegment->getCircuitEndNodePos(), Element::ElementType::RESISTOR_traction_wire);
    // }
    #[allow(unused_variables)]
    pub fn add_overhead_wire_clamp_to_circuit(&mut self, id: String, startSegment: Ptr<MSOverheadWire>, endSegment: Ptr<MSOverheadWire>) {
        let pos_start = startSegment.m().get_lane().get_shape(false);
        let pos_end = endSegment.m().get_lane().get_shape(false);
        let distance: f64 = pos_start.front().distance_to2d(pos_end.back());

        if distance > 10.0 {
            utils::common::msg_handler::write_warning(&utils::tlf!("The distance between two overhead wires during adding overhead wire clamp '%' defined for traction substation '%' is % m.", id.as_str(), startSegment.m().get_traction_substation().m().get_id(), distance));
        }
        // shortcut: the clamp's circuit element needs circuit node handles (startSegment/endSegment circuit nodes), which utils::traction_wire::Circuit does not take as Ptr<Node>; upgrade with the circuit node-handle change (only reached with overhead-wire.solver)
    }

    /// C++: `void MSTractionSubstation::eraseOverheadWireSegmentFromCircuit(MSOverheadWire *)`
    // void
    // MSTractionSubstation::eraseOverheadWireSegmentFromCircuit(MSOverheadWire* oldSegment) {
    //     //myOverheadWireSegments.push_back(static_cast<MSOverheadWire*>(MSNet::getInstance()->getStoppingPlace(overheadWireSegmentID, SUMO_TAG_OVERHEAD_WIRE_SEGMENT)));
    //     myOverheadWireSegments.erase(std::remove(myOverheadWireSegments.begin(), myOverheadWireSegments.end(), oldSegment), myOverheadWireSegments.end());
    // }
    #[allow(unused_variables)]
    pub fn erase_overhead_wire_segment_from_circuit(&mut self, oldWireSegment: Ptr<MSOverheadWire>) {
        self.myOverheadWireSegments.retain(|s| (*s).as_raw() != oldWireSegment.as_raw());
    }

    /// C++: `void MSTractionSubstation::writeOut()`
    // void
    // MSTractionSubstation::writeOut() {
    //     std::cout << "substation " << getID() << " constrols segments: \n";
    //     for (std::vector<MSOverheadWire*>::iterator it = myOverheadWireSegments.begin(); it != myOverheadWireSegments.end(); ++it) {
    //         std::cout << "        " << (*it)->getOverheadWireSegmentName() << "\n";
    //     }
    // }
    #[allow(unused_variables)]
    pub fn write_out(&mut self) {
        print!("substation {} constrols segments: \n", self.get_id());
        for seg in self.myOverheadWireSegments.iter() {
            print!("        {}\n", seg.m().get_overhead_wire_segment_name());
        }
    }

    /// C++: `std::size_t MSTractionSubstation::numberOfOverheadSegments() const` [const]
    // std::size_t numberOfOverheadSegments() const {
    //         return myOverheadWireSegments.size();
    //     }
    #[allow(unused_variables)]
    pub fn number_of_overhead_segments(&self) -> usize {
        self.myOverheadWireSegments.len()
    }

    /// C++: `void MSTractionSubstation::setChargingVehicle(bool)`
    /// enable or disable charging vehicle
    // void
    // MSTractionSubstation::setChargingVehicle(bool value) {
    //     myChargingVehicle = value;
    // }
    #[allow(unused_variables)]
    pub fn set_charging_vehicle(&mut self, value: bool) {
        self.myChargingVehicle = value;
    }

    /// C++: `bool MSTractionSubstation::isCharging() const` [const]
    /// Return true if in the current time step the substation (overhead wire section) is charging a vehicle
    // bool
    // MSTractionSubstation::isCharging() const {
    //     return myChargingVehicle;
    // }
    #[allow(unused_variables)]
    pub fn is_charging(&self) -> bool {
        self.myChargingVehicle
    }

    /// C++: `void MSTractionSubstation::increaseElecHybridCount()`
    // void
    // MSTractionSubstation::increaseElecHybridCount() {
    //     myElecHybridCount++;
    // }
    #[allow(unused_variables)]
    pub fn increase_elec_hybrid_count(&mut self) {
        self.myElecHybridCount += 1;
    }

    /// C++: `void MSTractionSubstation::decreaseElecHybridCount()`
    // void
    // MSTractionSubstation::decreaseElecHybridCount() {
    //     myElecHybridCount--;
    // }
    #[allow(unused_variables)]
    pub fn decrease_elec_hybrid_count(&mut self) {
        self.myElecHybridCount -= 1;
    }

    /// C++: `void MSTractionSubstation::addForbiddenLane(MSLane *)`
    // void MSTractionSubstation::addForbiddenLane(MSLane* lane) {
    //     myForbiddenLanes.push_back(lane);
    // }
    #[allow(unused_variables)]
    pub fn add_forbidden_lane(&mut self, lane: Ptr<MSLane>) {
        self.myForbiddenLanes.push(lane);
    }

    /// C++: `bool MSTractionSubstation::isForbidden(const MSLane *)`
    // bool MSTractionSubstation::isForbidden(const MSLane* lane) {
    //     for (std::vector<MSLane*>::iterator it = myForbiddenLanes.begin(); it != myForbiddenLanes.end(); ++it) {
    //         if (lane == (*it)) {
    //             return true;
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn is_forbidden(&mut self, lane: Ptr<MSLane>) -> bool {
        self.myForbiddenLanes.iter().any(|l| (*l).as_raw() == lane.as_raw())
    }

    /// C++: `void MSTractionSubstation::addClamp(const std::string &, MSOverheadWire *, MSOverheadWire *)`
    // void
    // MSTractionSubstation::addClamp(const std::string& id, MSOverheadWire* startPos, MSOverheadWire* endPos) {
    //     OverheadWireClamp clamp(id, startPos, endPos, false);
    //     myOverheadWireClamps.push_back(clamp);
    // }
    #[allow(unused_variables)]
    pub fn add_clamp(&mut self, id: &str, startPos: Ptr<MSOverheadWire>, endPos: Ptr<MSOverheadWire>) {
        self.myOverheadWireClamps.push(MSTractionSubstation_OverheadWireClamp::new_2(id.to_string(), startPos, endPos, false));
    }

    /// C++: `int MSTractionSubstation::getElecHybridCount() const` [const]
    // int getElecHybridCount() const {
    //         return myElecHybridCount;
    //     }
    #[allow(unused_variables)]
    pub fn get_elec_hybrid_count(&self) -> i32 {
        self.myElecHybridCount
    }

    /// C++: `void MSTractionSubstation::addVehicle(MSDevice_ElecHybrid *)`
    // void
    // MSTractionSubstation::addVehicle(MSDevice_ElecHybrid* elecHybrid) {
    //     myElecHybrid.push_back(elecHybrid);
    // }
    #[allow(unused_variables)]
    pub fn add_vehicle(&mut self, elecHybrid: Ptr<MSDevice_ElecHybrid>) {
        self.myElecHybrid.push(elecHybrid);
    }

    /// C++: `void MSTractionSubstation::eraseVehicle(MSDevice_ElecHybrid *)`
    // void
    // MSTractionSubstation::eraseVehicle(MSDevice_ElecHybrid* veh) {
    //     myElecHybrid.erase(std::remove(myElecHybrid.begin(), myElecHybrid.end(), veh), myElecHybrid.end());
    // }
    #[allow(unused_variables)]
    pub fn erase_vehicle(&mut self, elecHybrid: Ptr<MSDevice_ElecHybrid>) {
        self.myElecHybrid.retain(|v| (*v).as_raw() != elecHybrid.as_raw());
    }

    /// C++: `double MSTractionSubstation::getSubstationVoltage() const` [const]
    // double getSubstationVoltage() const {
    //         return mySubstationVoltage;
    //     }
    #[allow(unused_variables)]
    pub fn get_substation_voltage(&self) -> f64 {
        self.mySubstationVoltage
    }

    /// C++: `bool MSTractionSubstation::isAnySectionPreviouslyDefined()`
    // bool
    // MSTractionSubstation::isAnySectionPreviouslyDefined() {
    //     if (myOverheadWireSegments.size() > 0 || myForbiddenLanes.size() > 0 || getCircuit()->getLastId() > 0) {
    //         return true;
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn is_any_section_previously_defined(&mut self) -> bool {
        !self.myOverheadWireSegments.is_empty() || !self.myForbiddenLanes.is_empty() || self.myCircuit.m().get_last_id() > 0
    }

    /// C++: `void MSTractionSubstation::addSolvingCircuitToEndOfTimestepEvents()`
    // void
    // MSTractionSubstation::addSolvingCircuitToEndOfTimestepEvents() {
    //     if (!myChargingVehicle) {
    //         // myCommandForSolvingCircuit = new StaticCommand<MSTractionSubstation>(&MSTractionSubstation::solveCircuit);
    //         myCommandForSolvingCircuit = new WrappingCommand<MSTractionSubstation>(this, &MSTractionSubstation::solveCircuit);
    //         MSNet::getInstance()->getEndOfTimestepEvents()->addEvent(myCommandForSolvingCircuit);
    //         setChargingVehicle(true);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_solving_circuit_to_end_of_timestep_events(&mut self) {
        if !self.myChargingVehicle {
            // shortcut: WrappingCommand is not ported, so solveCircuit is not scheduled on the end-of-timestep events; upgrade when Command exists
            self.set_charging_vehicle(true);
        }
    }

    /// C++: `SUMOTime MSTractionSubstation::solveCircuit(SUMOTime)`
    // SUMOTime
    // MSTractionSubstation::solveCircuit(SUMOTime /*currentTime*/) {
    //     /*Circuit evaluation*/
    //     setChargingVehicle(false);
    //
    // #ifdef HAVE_EIGEN
    //
    //     // RICE_TODO: Allow for updating current limits in each time step if changed e.g. via traci or similar
    //     // getCircuit()->setCurrentLimit(myCurrentLimit);
    //
    //     // Solve the electrical circuit
    //     myCircuit->solve();
    //
    //     if (myCircuit->getAlphaBest() != 1.0) {
    //         WRITE_WARNINGF(TL("The requested total power could not be delivered by the overhead wire. Only % of originally requested power was provided."), toString(myCircuit->getAlphaBest()));
    //     }
    // #endif
    //
    //     // RICE_TODO: verify what happens if eigen is not defined?
    //     // Note: addSolvingCircuitToEndOfTimestepEvents() and thus solveCircuit() should be called from notifyMove only if eigen is defined.
    //     addChargeValueForOutput(WATT2WATTHR(myCircuit->getTotalPowerOfCircuitSources()), myCircuit->getTotalCurrentOfCircuitSources(), myCircuit->getAlphaBest(), myCircuit->getAlphaReason());
    //
    //     for (auto* it : myElecHybrid) {
    //
    //         Element* vehElem = it->getVehElem();
    //         double voltage = vehElem->getVoltage();
    //         double current = -vehElem->getCurrent();  // Vehicle is a power source, hence its current (returned by getCurrent()) flows in opposite direction
    //
    //         it->setCurrentFromOverheadWire(current);
    //         it->setVoltageOfOverheadWire(voltage);
    //
    //         // Calulate energy charged
    //         double energyIn = WATT2WATTHR(voltage * current);  // [Wh]
    //
    //         // Compute energy charged into/from battery considering recuperation and propulsion efficiency (not considering battery capacity)
    //         double energyCharged = it->computeChargedEnergy(energyIn);
    //
    //         // Update energy saved in the battery pack and return trully charged energy considering limits of battery
    //         double realEnergyCharged = it->storeEnergyToBattery(energyCharged);
    //
    //         it->setEnergyCharged(realEnergyCharged);
    //
    //         // Add energy wasted to the total sum
    //         it->updateTotalEnergyWasted(energyCharged - realEnergyCharged);
    //         // Add the energy provided by the overhead wire segment to the output of the segment
    //         it->getActOverheadWireSegment()->addChargeValueForOutput(energyIn, it);
    //     }
    //
    //     return 0;
    // }
    #[allow(unused_variables)]
    pub fn solve_circuit(&mut self, currentTime: SUMOTime) -> SUMOTime {
        // Circuit evaluation
        self.set_charging_vehicle(false);

        let circuit = self.myCircuit;
        // RICE_TODO: Allow for updating current limits in each time step if changed e.g. via traci or similar
        circuit.m().solve();

        if circuit.m().get_alpha_best() != 1.0 {
            utils::common::msg_handler::write_warning(&utils::tlf!("The requested total power could not be delivered by the overhead wire. Only % of originally requested power was provided.", circuit.m().get_alpha_best()));
        }

        // RICE_TODO: verify what happens if eigen is not defined?
        // Note: addSolvingCircuitToEndOfTimestepEvents() and thus solveCircuit() should be called from notifyMove only if eigen is defined.
        self.add_charge_value_for_output(watt2watthr(circuit.m().get_total_power_of_circuit_sources()), circuit.m().get_total_current_of_circuit_sources(), circuit.m().get_alpha_best(), circuit.m().get_alpha_reason());

        // shortcut: the per-vehicle update (vehicle element voltage/current, setCurrentFromOverheadWire, battery energy storage) needs MSDevice_ElecHybrid::get_veh_elem (Ptr<Element>) and Element::get_voltage over the arena-indexed Circuit; upgrade when the vehicle element handle becomes ElemIdx
        0
    }

    /// C++: `void MSTractionSubstation::addChargeValueForOutput(double, double, double, Circuit::alphaFlag)`
    /// add charge value for output
    // void
    // MSTractionSubstation::addChargeValueForOutput(double energy, double current, double alpha, Circuit::alphaFlag alphaReason) {
    //     std::string status = "";
    //
    //     myTotalEnergy += energy; //[Wh]
    //
    //     std::string vehicleIDs = "";
    //     for (std::vector<MSDevice_ElecHybrid*>::iterator it = myElecHybrid.begin(); it != myElecHybrid.end(); it++) {
    //         vehicleIDs += (*it)->getID() + " ";
    //     }
    //     //vehicleIDs.erase(vehicleIDs.end());
    //     // TODO vehicleIDs should not be empty, but in some case, it is (due to teleporting of vehicle?)
    //     if (!vehicleIDs.empty()) {
    //         vehicleIDs.pop_back();
    //     }
    //
    //     std::string currents = "";
    //     currents = myCircuit->getCurrentsOfCircuitSource(currents);
    //
    //     // create charge row and insert it in myChargeValues
    //     chargeTS C(MSNet::getInstance()->getCurrentTimeStep(), getID(), vehicleIDs, energy, current, currents, mySubstationVoltage, status,
    //                (int)myElecHybrid.size(), (int)getCircuit()->getNumVoltageSources(), alpha, alphaReason);
    //     myChargeValues.push_back(C);
    // }
    #[allow(unused_variables)]
    pub fn add_charge_value_for_output(&mut self, energy: f64, current: f64, alpha: f64, alphaReason: AlphaFlag) {
        let mut vehicleIDs: String = String::new();
        self.myTotalEnergy += energy; //[Wh]
        for it in self.myElecHybrid.iter() {
            vehicleIDs += &format!("{} ", it.m().base2.get_id());
        }
        // TODO vehicleIDs should not be empty, but in some case, it is (due to teleporting of vehicle?)
        if !vehicleIDs.is_empty() {
            vehicleIDs.pop();
        }

        let currents: String = self.myCircuit.m().get_currents_of_circuit_source();

        // create charge row and insert it in myChargeValues
        let C = MSTractionSubstation_chargeTS::new(
            MSNet::get_instance().get_current_time_step(),
            self.get_id().to_string(),
            vehicleIDs,
            energy,
            current,
            currents,
            self.mySubstationVoltage,
            String::new(),
            self.myElecHybrid.len() as i32,
            self.myCircuit.m().get_num_voltage_sources() as i32,
            alpha,
            alphaReason,
        );
        self.myChargeValues.push(C);
    }

    /// C++: `void MSTractionSubstation::writeTractionSubstationOutput(OutputDevice &)`
    /// write traction substation values
    // void
    // MSTractionSubstation::writeTractionSubstationOutput(OutputDevice& output) {
    //     output.openTag(SUMO_TAG_TRACTION_SUBSTATION);
    //     output.writeAttr(SUMO_ATTR_ID, myID);
    //     output.writeAttr(SUMO_ATTR_TOTALENERGYCHARGED, myTotalEnergy); //[Wh]
    //     double length = 0;
    //     for (auto it = myOverheadWireSegments.begin(); it != myOverheadWireSegments.end(); it++) {
    //         length += (*it)->getEndLanePosition() - (*it)->getBeginLanePosition();
    //     }
    //     output.writeAttr(SUMO_ATTR_LENGTH, length);
    //     output.writeAttr("numVoltageSources", myCircuit->getNumVoltageSources());
    //     output.writeAttr("numClamps", myOverheadWireClamps.size());
    //     output.writeAttr(SUMO_ATTR_CHARGINGSTEPS, myChargeValues.size());
    //
    //     // start writting
    //     if (myChargeValues.size() > 0) {
    //         // iterate over charging values
    //         for (std::vector<MSTractionSubstation::chargeTS>::const_iterator i = myChargeValues.begin(); i != myChargeValues.end(); i++) {
    //             // open tag for timestep and write all parameters
    //             output.openTag(SUMO_TAG_STEP);
    //             output.writeAttr(SUMO_ATTR_TIME, time2string(i->timeStep));
    //             // charge values
    //             output.writeAttr("vehicleIDs", i->vehicleIDs);
    //             output.writeAttr("numVehicles", i->numVehicles);
    //             // same number of numVoltageSources for all time, parameter is written in the superordinate tag
    //             //output.writeAttr("numVoltageSources", i->numVoltageSources);
    //             // charging status is always ""
    //             //output.writeAttr(SUMO_ATTR_CHARGING_STATUS, i->status);
    //             output.writeAttr(SUMO_ATTR_ENERGYCHARGED, i->energy);
    //             output.writeAttr(SUMO_ATTR_CURRENTFROMOVERHEADWIRE, i->current);
    //             output.writeAttr("currents", i->currentsString);
    //             // charging values of charging station in this timestep
    //             output.writeAttr(SUMO_ATTR_VOLTAGE, i->voltage);
    //             output.writeAttr(SUMO_ATTR_ALPHACIRCUITSOLVER, i->alpha);
    //             output.writeAttr("alphaFlag", i->alphaReason);
    //             // close tag timestep
    //             output.closeTag();
    //             // update timestep of charge
    //         }
    //     }
    //     // close charging station tag
    //     output.closeTag();
    // }
    #[allow(unused_variables)]
    pub fn write_traction_substation_output(&mut self, output: &mut OutputDevice) {
        output.open_tag_id(SUMO_TAG_TRACTION_SUBSTATION);
        output.write_attr(SUMO_ATTR_ID, self.get_id());
        output.write_attr(SUMO_ATTR_TOTALENERGYCHARGED, &self.myTotalEnergy); //[Wh]
        let mut length: f64 = 0.0;
        for it in self.myOverheadWireSegments.iter() {
            length += it.m().get_end_lane_position() - it.m().get_begin_lane_position();
        }
        output.write_attr(SUMO_ATTR_LENGTH, &length);
        output.write_attr("numVoltageSources", &(self.myCircuit.m().get_num_voltage_sources() as i32));
        output.write_attr("numClamps", &(self.myOverheadWireClamps.len() as i32));
        output.write_attr(SUMO_ATTR_CHARGINGSTEPS, &(self.myChargeValues.len() as i32));

        // start writting
        if self.myChargeValues.len() > 0 {
            // iterate over charging values
            for i in self.myChargeValues.iter() {
                // open tag for timestep and write all parameters
                output.open_tag_id(SUMO_TAG_STEP);
                output.write_attr(SUMO_ATTR_TIME, &time2string(i.timeStep));
                // charge values
                output.write_attr("vehicleIDs", i.vehicleIDs.as_str());
                output.write_attr("numVehicles", &i.numVehicles);
                output.write_attr(SUMO_ATTR_ENERGYCHARGED, &i.energy);
                output.write_attr(SUMO_ATTR_CURRENTFROMOVERHEADWIRE, &i.current);
                output.write_attr("currents", i.currentsString.as_str());
                // charging values of charging station in this timestep
                output.write_attr(SUMO_ATTR_VOLTAGE, &i.voltage);
                output.write_attr(SUMO_ATTR_ALPHACIRCUITSOLVER, &i.alpha);
                output.write_attr("alphaFlag", &(i.alphaReason as i32));
                // close tag timestep
                output.close_tag("");
            }
        }
        // close charging station tag
        output.close_tag("");
    }

    /// C++: `OverheadWireClamp * MSTractionSubstation::findClamp(std::string)`
    /// Find an overhead wire clamp by its ID
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn find_clamp(&mut self, id: String) -> Ptr<MSTractionSubstation_OverheadWireClamp> {
        for it in self.myOverheadWireClamps.iter_mut() {
            if it.id == id {
                return Ptr::from_mut(it);
            }
        }
        Ptr::null()
    }

    /// C++: `void MSTractionSubstation::addOverheadWireInnerSegmentToCircuit(MSOverheadWire *, MSOverheadWire *, const MSLane *, const MSLane *, const MSLane *)` [private]
    // void
    // MSTractionSubstation::addOverheadWireInnerSegmentToCircuit(MSOverheadWire* incomingSegment, MSOverheadWire* outgoingSegment, const MSLane* connection, const MSLane* frontConnection, const MSLane* behindConnection) {
    //     if (frontConnection == nullptr && behindConnection == nullptr) {
    //         // addOverheadWire from nNode of newOverheadWireSegment to pNode
    //         MSOverheadWire* innerSegment = dynamic_cast<MSOverheadWire*>(MSNet::getInstance()->getStoppingPlace("ovrhd_inner_" + connection->getID(), SUMO_TAG_OVERHEAD_WIRE_SEGMENT));
    //         myOverheadWireSegments.push_back(innerSegment);
    //         innerSegment->setTractionSubstation(incomingSegment->getTractionSubstation());
    //         if (MSGlobals::gOverheadWireSolver) {
    // #ifdef HAVE_EIGEN
    //             Element* elem = incomingSegment->getCircuit()->addElement("pos_ovrhd_inner_" + connection->getID(), (connection->getLength()) * WIRE_RESISTIVITY, incomingSegment->getCircuitEndNodePos(), outgoingSegment->getCircuitStartNodePos(), Element::ElementType::RESISTOR_traction_wire);
    //             innerSegment->setCircuitElementPos(elem);
    //             innerSegment->setCircuitStartNodePos(incomingSegment->getCircuitEndNodePos());
    //             innerSegment->setCircuitEndNodePos(outgoingSegment->getCircuitStartNodePos());
    // #else
    //             UNUSED_PARAMETER(outgoingSegment);
    //             WRITE_WARNING(TL("Overhead circuit solver requested, but solver support (Eigen) not compiled in."));
    // #endif
    //         }
    //     } else if (frontConnection != nullptr && behindConnection == nullptr) {
    //         MSOverheadWire* innerSegment = dynamic_cast<MSOverheadWire*>(MSNet::getInstance()->getStoppingPlace("ovrhd_inner_" + frontConnection->getID(), SUMO_TAG_OVERHEAD_WIRE_SEGMENT));
    //         MSOverheadWire* innerSegment2 = dynamic_cast<MSOverheadWire*>(MSNet::getInstance()->getStoppingPlace("ovrhd_inner_" + connection->getID(), SUMO_TAG_OVERHEAD_WIRE_SEGMENT));
    //
    //         innerSegment->setTractionSubstation(incomingSegment->getTractionSubstation());
    //         myOverheadWireSegments.push_back(innerSegment);
    //         innerSegment2->setTractionSubstation(incomingSegment->getTractionSubstation());
    //         myOverheadWireSegments.push_back(innerSegment2);
    //
    //         if (MSGlobals::gOverheadWireSolver) {
    // #ifdef HAVE_EIGEN
    //             Node* betweenFrontNode_pos = incomingSegment->getCircuit()->addNode("betweenFrontNode_pos_" + connection->getID());
    //             Element* elem = incomingSegment->getCircuit()->addElement("pos_ovrhd_inner_" + frontConnection->getID(), (frontConnection->getLength()) * WIRE_RESISTIVITY, incomingSegment->getCircuitEndNodePos(), betweenFrontNode_pos, Element::ElementType::RESISTOR_traction_wire);
    //             Element* elem2 = incomingSegment->getCircuit()->addElement("pos_ovrhd_inner_" + connection->getID(), (connection->getLength()) * WIRE_RESISTIVITY, betweenFrontNode_pos, outgoingSegment->getCircuitStartNodePos(), Element::ElementType::RESISTOR_traction_wire);
    //
    //             innerSegment->setCircuitElementPos(elem);
    //             innerSegment->setCircuitStartNodePos(incomingSegment->getCircuitEndNodePos());
    //             innerSegment->setCircuitEndNodePos(betweenFrontNode_pos);
    //
    //             innerSegment2->setCircuitElementPos(elem2);
    //             innerSegment2->setCircuitStartNodePos(betweenFrontNode_pos);
    //             innerSegment2->setCircuitEndNodePos(outgoingSegment->getCircuitStartNodePos());
    // #else
    //             WRITE_WARNING(TL("Overhead circuit solver requested, but solver support (Eigen) not compiled in."));
    // #endif
    //         }
    //     } else if (frontConnection == nullptr && behindConnection != nullptr) {
    //         MSOverheadWire* innerSegment = dynamic_cast<MSOverheadWire*>(MSNet::getInstance()->getStoppingPlace("ovrhd_inner_" + connection->getID(), SUMO_TAG_OVERHEAD_WIRE_SEGMENT));
    //         MSOverheadWire* innerSegment2 = dynamic_cast<MSOverheadWire*>(MSNet::getInstance()->getStoppingPlace("ovrhd_inner_" + behindConnection->getID(), SUMO_TAG_OVERHEAD_WIRE_SEGMENT));
    //
    //         innerSegment->setTractionSubstation(incomingSegment->getTractionSubstation());
    //         myOverheadWireSegments.push_back(innerSegment);
    //         innerSegment2->setTractionSubstation(incomingSegment->getTractionSubstation());
    //         myOverheadWireSegments.push_back(innerSegment2);
    //
    //         if (MSGlobals::gOverheadWireSolver) {
    // #ifdef HAVE_EIGEN
    //             Node* betweenBehindNode_pos = incomingSegment->getCircuit()->addNode("betweenBehindNode_pos_" + connection->getID());
    //             Element* elem = incomingSegment->getCircuit()->addElement("pos_ovrhd_inner_" + connection->getID(), (connection->getLength()) * WIRE_RESISTIVITY, incomingSegment->getCircuitEndNodePos(), betweenBehindNode_pos, Element::ElementType::RESISTOR_traction_wire);
    //             Element* elem2 = incomingSegment->getCircuit()->addElement("pos_ovrhd_inner_" + behindConnection->getID(), (behindConnection->getLength()) * WIRE_RESISTIVITY, betweenBehindNode_pos, outgoingSegment->getCircuitStartNodePos(), Element::ElementType::RESISTOR_traction_wire);
    //
    //             innerSegment->setCircuitElementPos(elem);
    //             innerSegment->setCircuitStartNodePos(incomingSegment->getCircuitEndNodePos());
    //             innerSegment->setCircuitEndNodePos(betweenBehindNode_pos);
    //
    //             innerSegment2->setCircuitElementPos(elem2);
    //             innerSegment2->setCircuitStartNodePos(betweenBehindNode_pos);
    //             innerSegment2->setCircuitEndNodePos(outgoingSegment->getCircuitStartNodePos());
    // #else
    //             WRITE_WARNING(TL("Overhead circuit solver requested, but solver support (Eigen) not compiled in."));
    // #endif
    //         }
    //     } else if (frontConnection != nullptr && behindConnection != nullptr) {
    //         MSOverheadWire* innerSegment = dynamic_cast<MSOverheadWire*>(MSNet::getInstance()->getStoppingPlace("ovrhd_inner_" + frontConnection->getID(), SUMO_TAG_OVERHEAD_WIRE_SEGMENT));
    //         MSOverheadWire* innerSegment2 = dynamic_cast<MSOverheadWire*>(MSNet::getInstance()->getStoppingPlace("ovrhd_inner_" + connection->getID(), SUMO_TAG_OVERHEAD_WIRE_SEGMENT));
    //         MSOverheadWire* innerSegment3 = dynamic_cast<MSOverheadWire*>(MSNet::getInstance()->getStoppingPlace("ovrhd_inner_" + behindConnection->getID(), SUMO_TAG_OVERHEAD_WIRE_SEGMENT));
    //
    //         innerSegment->setTractionSubstation(incomingSegment->getTractionSubstation());
    //         myOverheadWireSegments.push_back(innerSegment);
    //         innerSegment2->setTractionSubstation(incomingSegment->getTractionSubstation());
    //         myOverheadWireSegments.push_back(innerSegment2);
    //         innerSegment3->setTractionSubstation(incomingSegment->getTractionSubstation());
    //         myOverheadWireSegments.push_back(innerSegment3);
    //
    //         if (MSGlobals::gOverheadWireSolver) {
    // #ifdef HAVE_EIGEN
    //             Node* betweenFrontNode_pos = incomingSegment->getCircuit()->addNode("betweenFrontNode_pos_" + connection->getID());
    //             Node* betweenBehindNode_pos = incomingSegment->getCircuit()->addNode("betweenBehindNode_pos_" + connection->getID());
    //             Element* elem = incomingSegment->getCircuit()->addElement("pos_ovrhd_inner_" + frontConnection->getID(), (frontConnection->getLength()) * WIRE_RESISTIVITY, incomingSegment->getCircuitEndNodePos(), betweenFrontNode_pos, Element::ElementType::RESISTOR_traction_wire);
    //             Element* elem2 = incomingSegment->getCircuit()->addElement("pos_ovrhd_inner_" + connection->getID(), (connection->getLength()) * WIRE_RESISTIVITY, betweenFrontNode_pos, betweenBehindNode_pos, Element::ElementType::RESISTOR_traction_wire);
    //             Element* elem3 = incomingSegment->getCircuit()->addElement("pos_ovrhd_inner_" + behindConnection->getID(), (behindConnection->getLength()) * WIRE_RESISTIVITY, betweenBehindNode_pos, outgoingSegment->getCircuitStartNodePos(), Element::ElementType::RESISTOR_traction_wire);
    //
    //             innerSegment->setCircuitElementPos(elem);
    //             innerSegment->setCircuitStartNodePos(incomingSegment->getCircuitEndNodePos());
    //             innerSegment->setCircuitEndNodePos(betweenFrontNode_pos);
    //
    //             innerSegment2->setCircuitElementPos(elem2);
    //             innerSegment2->setCircuitStartNodePos(betweenFrontNode_pos);
    //             innerSegment2->setCircuitEndNodePos(betweenBehindNode_pos);
    //
    //             innerSegment3->setCircuitElementPos(elem3);
    //             innerSegment3->setCircuitStartNodePos(betweenBehindNode_pos);
    //             innerSegment3->setCircuitEndNodePos(outgoingSegment->getCircuitStartNodePos());
    // #else
    //             WRITE_WARNING(TL("Overhead circuit solver requested, but solver support not compiled in."));
    // #endif
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_overhead_wire_inner_segment_to_circuit(&mut self, incomingSegment: Ptr<MSOverheadWire>, outgoingSegment: Ptr<MSOverheadWire>, connection: Ptr<MSLane>, frontConnection: Ptr<MSLane>, behindConnection: Ptr<MSLane>) {
        if frontConnection.is_null() && behindConnection.is_null() {
            // addOverheadWire from nNode of newOverheadWireSegment to pNode
            let innerSegment = ovrhd_inner_segment(connection);
            self.myOverheadWireSegments.push(innerSegment);
            innerSegment.m().set_traction_substation(incomingSegment.m().get_traction_substation());
            if overhead_wire_solver() {
                // shortcut: inner segment circuit element needs circuit node handles (incomingSegment/outgoingSegment), which utils::traction_wire::Circuit does not take as Ptr<Node>; upgrade with the circuit node-handle change
            }
        } else if !frontConnection.is_null() && behindConnection.is_null() {
            let innerSegment = ovrhd_inner_segment(frontConnection);
            let innerSegment2 = ovrhd_inner_segment(connection);

            innerSegment.m().set_traction_substation(incomingSegment.m().get_traction_substation());
            self.myOverheadWireSegments.push(innerSegment);
            innerSegment2.m().set_traction_substation(incomingSegment.m().get_traction_substation());
            self.myOverheadWireSegments.push(innerSegment2);

            if overhead_wire_solver() {
                // shortcut: inner segment circuit elements need circuit node handles (incomingSegment/outgoingSegment), which utils::traction_wire::Circuit does not take as Ptr<Node>; upgrade with the circuit node-handle change
            }
        } else if frontConnection.is_null() && !behindConnection.is_null() {
            let innerSegment = ovrhd_inner_segment(connection);
            let innerSegment2 = ovrhd_inner_segment(behindConnection);

            innerSegment.m().set_traction_substation(incomingSegment.m().get_traction_substation());
            self.myOverheadWireSegments.push(innerSegment);
            innerSegment2.m().set_traction_substation(incomingSegment.m().get_traction_substation());
            self.myOverheadWireSegments.push(innerSegment2);

            if overhead_wire_solver() {
                // shortcut: inner segment circuit elements need circuit node handles (incomingSegment/outgoingSegment), which utils::traction_wire::Circuit does not take as Ptr<Node>; upgrade with the circuit node-handle change
            }
        } else if !frontConnection.is_null() && !behindConnection.is_null() {
            let innerSegment = ovrhd_inner_segment(frontConnection);
            let innerSegment2 = ovrhd_inner_segment(connection);
            let innerSegment3 = ovrhd_inner_segment(behindConnection);

            innerSegment.m().set_traction_substation(incomingSegment.m().get_traction_substation());
            self.myOverheadWireSegments.push(innerSegment);
            innerSegment2.m().set_traction_substation(incomingSegment.m().get_traction_substation());
            self.myOverheadWireSegments.push(innerSegment2);
            innerSegment3.m().set_traction_substation(incomingSegment.m().get_traction_substation());
            self.myOverheadWireSegments.push(innerSegment3);

            if overhead_wire_solver() {
                // shortcut: inner segment circuit elements need circuit node handles (incomingSegment/outgoingSegment), which utils::traction_wire::Circuit does not take as Ptr<Node>; upgrade with the circuit node-handle change
            }
        }
    }

}

/// C++ class `MSTractionSubstation::OverheadWireClamp` (microsim/trigger/MSOverheadWire.h)
pub struct MSTractionSubstation_OverheadWireClamp {
    pub id: String,
    pub start: Ptr<MSOverheadWire>,
    pub end: Ptr<MSOverheadWire>,
    pub usage: bool,
}

impl MSTractionSubstation_OverheadWireClamp {
    /// C++: `void OverheadWireClamp::OverheadWireClamp()`
    // OverheadWireClamp() :
    //             id("undefined"),
    //             start(nullptr),
    //             end(nullptr),
    //             usage(false) {}
    #[allow(unused_variables)]
    pub fn new() -> Self {
        MSTractionSubstation_OverheadWireClamp {
            id: "undefined".to_string(),
            start: Ptr::null(),
            end: Ptr::null(),
            usage: false,
        }
    }

    /// C++: `void OverheadWireClamp::OverheadWireClamp(const std::string, MSOverheadWire *, MSOverheadWire *, bool)`
    // OverheadWireClamp(const std::string _id, MSOverheadWire* _start, MSOverheadWire* _end, bool _usage):
    //             id(_id),
    //             start(_start),
    //             end(_end),
    //             usage(_usage) {}
    #[allow(unused_variables)]
    pub fn new_2(_id: String, _start: Ptr<MSOverheadWire>, _end: Ptr<MSOverheadWire>, _usage: bool) -> Self {
        MSTractionSubstation_OverheadWireClamp {
            id: _id,
            start: _start,
            end: _end,
            usage: _usage,
        }
    }

}

/// struct to save information for the traction substation output
/// C++ class `MSTractionSubstation::chargeTS` (microsim/trigger/MSOverheadWire.h)
pub struct MSTractionSubstation_chargeTS {
    pub timeStep: SUMOTime,
    pub substationID: String,
    pub vehicleIDs: String,
    pub energy: f64,
    pub current: f64,
    pub currentsString: String,
    pub voltage: f64,
    /// status
    pub status: String,
    pub numVehicles: i32,
    pub numVoltageSources: i32,
    pub alpha: f64,
    pub alphaReason: AlphaFlag,
}

impl MSTractionSubstation_chargeTS {
    /// C++: `void chargeTS::chargeTS(SUMOTime, std::string, std::string, double, double, std::string, double, std::string, int, int, double, Circuit::alphaFlag)`
    /// constructor
    // chargeTS(SUMOTime _timeStep, std::string _substationID, std::string _vehicleIDs, double _energy,
    //                  double _current, std::string _currentsString, double _voltage, std::string _status,
    //                  int _numVehicle, int _numVoltageSources, double _alpha, Circuit::alphaFlag _alphaReason) :
    //             timeStep(_timeStep),
    //             substationID(_substationID),
    //             vehicleIDs(_vehicleIDs),
    //             energy(_energy),
    //             current(_current),
    //             currentsString(_currentsString),
    //             voltage(_voltage),
    //             status(_status),
    //             numVehicles(_numVehicle),
    //             numVoltageSources(_numVoltageSources),
    //             alpha(_alpha),
    //             alphaReason(_alphaReason) {}
    #[allow(unused_variables)]
    pub fn new(_timeStep: SUMOTime, _substationID: String, _vehicleIDs: String, _energy: f64, _current: f64, _currentsString: String, _voltage: f64, _status: String, _numVehicle: i32, _numVoltageSources: i32, _alpha: f64, _alphaReason: AlphaFlag) -> Self {
        MSTractionSubstation_chargeTS {
            timeStep: _timeStep,
            substationID: _substationID,
            vehicleIDs: _vehicleIDs,
            energy: _energy,
            current: _current,
            currentsString: _currentsString,
            voltage: _voltage,
            status: _status,
            numVehicles: _numVehicle,
            numVoltageSources: _numVoltageSources,
            alpha: _alpha,
            alphaReason: _alphaReason,
        }
    }

}

/// A class for sorting vehicle on lane under the overhead wire segment
/// C++ class `MSOverheadWire::vehicle_position_sorter` (microsim/trigger/MSOverheadWire.h)
pub struct MSOverheadWire_vehicle_position_sorter {
}

impl MSOverheadWire_vehicle_position_sorter {
    /// C++: `void vehicle_position_sorter::vehicle_position_sorter()`
    /// Constructor
    // explicit vehicle_position_sorter() { }
    #[allow(unused_variables)]
    pub fn new() -> Self {
        MSOverheadWire_vehicle_position_sorter {}
    }

    /// C++: `int vehicle_position_sorter::operator()(SUMOVehicle *, SUMOVehicle *)`
    /// Sorting function; compares RODFRouteDesc::distance2Last
    // int operator()(SUMOVehicle* v1, SUMOVehicle* v2) {
    //             return v1->getPositionOnLane() > v2->getPositionOnLane();
    //         }
    #[allow(unused_variables)]
    pub fn call_op(&mut self, v1: Ptr<SUMOVehicle>, v2: Ptr<SUMOVehicle>) -> i32 {
        (v1.m().get_position_on_lane() > v2.m().get_position_on_lane()) as i32
    }

}

