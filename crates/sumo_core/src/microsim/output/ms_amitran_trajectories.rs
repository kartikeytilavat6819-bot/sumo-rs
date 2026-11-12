//! Port of microsim/output/MSAmitranTrajectories.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// Realises dumping the complete network state
/// C++ class `MSAmitranTrajectories` (microsim/output/MSAmitranTrajectories.h)
pub struct MSAmitranTrajectories {
}

// C++ static members of MSAmitranTrajectories (port as module-level statics, see AGENTS.md):
//   static int MSAmitranTrajectories::myWrittenTypes;
//   static int MSAmitranTrajectories::myWrittenVehicles;
// The header declares them as std::set<std::string> and std::map<std::string, int>, which is what the code uses.
thread_local! {
    static MY_WRITTEN_TYPES: std::cell::RefCell<BTreeSet<String>> = const { std::cell::RefCell::new(BTreeSet::new()) };
    static MY_WRITTEN_VEHICLES: std::cell::RefCell<BTreeMap<String, i32>> = const { std::cell::RefCell::new(BTreeMap::new()) };
}

impl MSAmitranTrajectories {
    /// C++: `void MSAmitranTrajectories::write(OutputDevice &, const SUMOTime)` [static]
    /// Writes the complete network state into the given device
    // void
    // MSAmitranTrajectories::write(OutputDevice& of, const SUMOTime timestep) {
    //     MSVehicleControl& vc = MSNet::getInstance()->getVehicleControl();
    //     for (MSVehicleControl::constVehIt v = vc.loadedVehBegin(); v != vc.loadedVehEnd(); ++v) {
    //         writeVehicle(of, *v->second, timestep);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn write(of: &mut OutputDevice, timestep: SUMOTime) {
        let net = MSNet::get_instance();
        let vc = net.m().get_vehicle_control();
        // loadedVehBegin()/loadedVehEnd() are myVehicleDict.begin()/end()
        for (_, v) in vc.myVehicleDict.iter() {
            Self::write_vehicle(of, v.m(), timestep);
        }
    }

    /// C++: `void MSAmitranTrajectories::writeVehicle(OutputDevice &, const SUMOVehicle &, const SUMOTime)` [static, private]
    /// Writes the dump of the given vehicle into the given device
    // void
    // MSAmitranTrajectories::writeVehicle(OutputDevice& of, const SUMOVehicle& veh, const SUMOTime timestep) {
    //     if (veh.isOnRoad()) {
    //         const std::string& type = veh.getVehicleType().getID();
    //         if (myWrittenTypes.count(type) == 0) {
    //             of.openTag(SUMO_TAG_ACTORCONFIG).writeAttr(SUMO_ATTR_ID, veh.getVehicleType().getNumericalID());
    //             const SUMOEmissionClass c = veh.getVehicleType().getEmissionClass();
    //             if (c != 0) {
    //                 of.writeAttr(SUMO_ATTR_VEHICLECLASS, PollutantsInterface::getAmitranVehicleClass(c));
    //                 of.writeAttr("fuel", PollutantsInterface::getFuel(c));
    //                 of.writeAttr(SUMO_ATTR_EMISSIONCLASS, "Euro" + toString(PollutantsInterface::getEuroClass(c)));
    //                 const double weight = PollutantsInterface::getWeight(c);
    //                 if (weight > 0.) {
    //                     of.writeAttr(SUMO_ATTR_WEIGHT, int(weight / 10. + 0.5));
    //                 }
    //             }
    //             of.writeAttr(SUMO_ATTR_REF, type).closeTag();
    //             myWrittenTypes.insert(type);
    //         }
    //         if (myWrittenVehicles.count(veh.getID()) == 0) {
    //             const int index = (int)myWrittenVehicles.size();
    //             of.openTag(SUMO_TAG_VEHICLE).writeAttr(SUMO_ATTR_ID, index)
    //             .writeAttr(SUMO_ATTR_ACTORCONFIG, veh.getVehicleType().getNumericalID())
    //             .writeAttr(SUMO_ATTR_STARTTIME, STEPS2MS(veh.getDeparture()));
    //             of.writeAttr(SUMO_ATTR_REF, veh.getID()).closeTag();
    //             myWrittenVehicles[veh.getID()] = index;
    //         }
    //         of.openTag(SUMO_TAG_MOTIONSTATE).writeAttr(SUMO_ATTR_VEHICLE, myWrittenVehicles[veh.getID()])
    //         .writeAttr(SUMO_ATTR_SPEED, int(100.*veh.getSpeed() + 0.5))
    //         .writeAttr(SUMO_ATTR_TIME, STEPS2MS(timestep))
    //         .writeAttr(SUMO_ATTR_ACCELERATION, int(1000.*veh.getAcceleration() + 0.5));
    //         of.closeTag();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn write_vehicle(of: &mut OutputDevice, veh: &SUMOVehicle, timestep: SUMOTime) {
        if veh.is_on_road() {
            let type_ = veh.get_vehicle_type().get_id().to_string();
            if !MY_WRITTEN_TYPES.with_borrow(|s| s.contains(&type_)) {
                of.open_tag_id(SUMO_TAG_ACTORCONFIG).write_attr(SUMO_ATTR_ID, &veh.get_vehicle_type().get_numerical_id());
                let c = veh.get_vehicle_type().get_emission_class();
                if c != 0 {
                    // shortcut: PollutantsInterface (getAmitranVehicleClass, getFuel, getEuroClass, getWeight) is not ported, so the vehicleclass/fuel/emissionclass/weight attributes are skipped; upgrade when the pollutants interface is ported
                }
                of.write_attr(SUMO_ATTR_REF, type_.as_str()).close_tag("");
                MY_WRITTEN_TYPES.with_borrow_mut(|s| s.insert(type_.clone()));
            }
            let vid = veh.get_id().to_string();
            if !MY_WRITTEN_VEHICLES.with_borrow(|m| m.contains_key(&vid)) {
                let index = MY_WRITTEN_VEHICLES.with_borrow(|m| m.len() as i32);
                of.open_tag_id(SUMO_TAG_VEHICLE).write_attr(SUMO_ATTR_ID, &index)
                    .write_attr(SUMO_ATTR_ACTORCONFIG, &veh.get_vehicle_type().get_numerical_id())
                    .write_attr(SUMO_ATTR_STARTTIME, &veh.get_departure());
                of.write_attr(SUMO_ATTR_REF, vid.as_str()).close_tag("");
                MY_WRITTEN_VEHICLES.with_borrow_mut(|m| m.insert(vid.clone(), index));
            }
            let vehIndex = MY_WRITTEN_VEHICLES.with_borrow_mut(|m| *m.entry(vid.clone()).or_insert(0));
            // int(...) truncates toward zero, as Rust's `as` does
            let speed = (100. * veh.get_speed() + 0.5) as i32;
            let acceleration = (1000. * veh.get_acceleration() + 0.5) as i32;
            of.open_tag_id(SUMO_TAG_MOTIONSTATE).write_attr(SUMO_ATTR_VEHICLE, &vehIndex)
                .write_attr(SUMO_ATTR_SPEED, &speed)
                .write_attr(SUMO_ATTR_TIME, &timestep)
                .write_attr(SUMO_ATTR_ACCELERATION, &acceleration);
            of.close_tag("");
        }
    }

    /// C++: `void MSAmitranTrajectories::MSAmitranTrajectories(const MSAmitranTrajectories &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new(arg0: &MSAmitranTrajectories) -> Self {
        panic!("MSAmitranTrajectories copy constructor is private and invalidated in C++")
    }

    /// C++: `MSAmitranTrajectories & MSAmitranTrajectories::operator=(const MSAmitranTrajectories &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSAmitranTrajectories) -> &mut MSAmitranTrajectories {
        panic!("MSAmitranTrajectories assignment operator is private and invalidated in C++")
    }

}

