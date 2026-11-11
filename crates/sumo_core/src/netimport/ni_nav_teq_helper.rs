//! Port of netimport/NINavTeqHelper.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// Some parser methods shared around several formats containing NavTeq-Nets
/// C++ class `NINavTeqHelper` (netimport/NINavTeqHelper.h)
pub struct NINavTeqHelper {
}

impl NINavTeqHelper {
    /// C++: `double NINavTeqHelper::getSpeed(const std::string &, const std::string &)` [static]
    /// Returns the speed evaluating the given Navteq-description
    // double
    // NINavTeqHelper::getSpeed(const std::string& id, const std::string& speedClassS) {
    //     try {
    //         int speedClass = StringUtils::toInt(speedClassS);
    //         switch (speedClass) {
    //             case -1:
    //                 return (double) 1.0 / (double) 3.6;
    //             case 1:
    //                 return (double) 200 / (double) 3.6; //> 130 KPH / > 80 MPH
    //             case 2:
    //                 return (double) 120 / (double) 3.6; //101-130 KPH / 65-80 MPH
    //             case 3:
    //                 return (double) 100 / (double) 3.6; // 91-100 KPH / 55-64 MPH
    //             case 4:
    //                 return (double) 80 / (double) 3.6; // 71-90 KPH / 41-54 MPH
    //             case 5:
    //                 return (double) 70 / (double) 3.6; // 51-70 KPH / 31-40 MPH
    //             case 6:
    //                 return (double) 50 / (double) 3.6; // 31-50 KPH / 21-30 MPH
    //             case 7:
    //                 return (double) 30 / (double) 3.6; // 11-30 KPH / 6-20 MPH
    //             case 8:
    //                 return (double) 5 / (double) 3.6; //< 11 KPH / < 6 MPH
    //             default:
    //                 throw ProcessError(TLF("Invalid speed code (edge '%').", id));
    //         }
    //     } catch (NumberFormatException&) {
    //         throw ProcessError(TLF("Non-numerical value for an edge's speed type occurred (edge '%').", id));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_speed(id: &str, speedClassS: &str) -> f64 {
        let speedClass = match utils::common::string_utils::to_int(speedClassS) {
            Ok(v) => v,
            Err(_) => std::panic::panic_any(ProcessError::new(format!("Non-numerical value for an edge's speed type occurred (edge '{id}')."))),
        };
        match speedClass {
            -1 => 1.0_f64 / 3.6_f64,
            1 => 200.0_f64 / 3.6_f64,
            2 => 120.0_f64 / 3.6_f64,
            3 => 100.0_f64 / 3.6_f64,
            4 => 80.0_f64 / 3.6_f64,
            5 => 70.0_f64 / 3.6_f64,
            6 => 50.0_f64 / 3.6_f64,
            7 => 30.0_f64 / 3.6_f64,
            8 => 5.0_f64 / 3.6_f64,
            _ => std::panic::panic_any(ProcessError::new(format!("Invalid speed code (edge '{id}')."))),
        }
    }

    /// C++: `int NINavTeqHelper::getLaneNumber(const std::string &, const std::string &, double)` [static]
    /// Returns the lane number evaluating the given Navteq-description
    // int
    // NINavTeqHelper::getLaneNumber(const std::string& id, const std::string& laneNoS, double speed) {
    //     try {
    //         int nolanes = StringUtils::toInt(laneNoS);
    //         if (nolanes < 0) {
    //             return 1;
    //         } else if (nolanes / 10 > 0) {
    //             return nolanes / 10;
    //         } else {
    //             switch (nolanes % 10) {
    //                 case 1:
    //                     return 1;
    //                 case 2:
    //                     nolanes = 2;
    //                     if (speed > 78.0 / 3.6) {
    //                         nolanes = 3;
    //                     }
    //                     return nolanes;
    //                 case 3:
    //                     return 4;
    //                 default:
    //                     throw ProcessError(TLF("Invalid lane number (edge '%').", id));
    //             }
    //         }
    //     } catch (NumberFormatException&) {
    //         throw ProcessError(TLF("Non-numerical value for an edge's lane number occurred (edge '%').", id));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_lane_number(id: &str, laneNoS: &str, speed: f64) -> i32 {
        let nolanes = match utils::common::string_utils::to_int(laneNoS) {
            Ok(v) => v,
            Err(_) => std::panic::panic_any(ProcessError::new(format!("Non-numerical value for an edge's lane number occurred (edge '{id}')."))),
        };
        if nolanes < 0 {
            return 1;
        } else if nolanes / 10 > 0 {
            return nolanes / 10;
        }
        match nolanes % 10 {
            1 => 1,
            2 => if speed > 78.0 / 3.6 { 3 } else { 2 },
            3 => 4,
            _ => std::panic::panic_any(ProcessError::new(format!("Invalid lane number (edge '{id}')."))),
        }
    }

    /// C++: `void NINavTeqHelper::addVehicleClasses(NBEdge &, const std::string &, const SVCPermissions, const SVCPermissions)` [static]
    /// Adds vehicle classes parsing the given list of allowed vehicles
    // void
    // NINavTeqHelper::addVehicleClasses(NBEdge& e, const std::string& oclassS, const SVCPermissions allPermissions, const SVCPermissions defaultPermissions) {
    //     std::string classS = "0000000000" + oclassS;
    //     classS = classS.substr(classS.length() - 10);
    //     // 0: allow all vehicle types
    //     if (classS[0] == '1') {
    //         e.setPermissions(allPermissions);
    //         return;
    //     }
    //     bool haveCar = false;
    //     e.setPermissions(defaultPermissions);
    //     // Passenger cars -- becomes SVC_PASSENGER
    //     if (classS[1] == '1') {
    //         e.allowVehicleClass(-1, SVC_PASSENGER);
    //         haveCar = true;
    //     } else {
    //         e.disallowVehicleClass(-1, SVC_PASSENGER);
    //     }
    //     haveCar |= addCommonVehicleClasses(e, classS, 2);
    //     if (!haveCar) {
    //         e.setPermissions(0);
    //     }
    //     // Bicycle -- becomes SVC_BICYCLE
    //     if (classS[8] == '1') {
    //         e.allowVehicleClass(-1, SVC_BICYCLE);
    //     } else {
    //         e.disallowVehicleClass(-1, SVC_BICYCLE);
    //     }
    //     // Pedestrian -- becomes SVC_PEDESTRIAN
    //     if (classS[9] == '1') {
    //         e.allowVehicleClass(-1, SVC_PEDESTRIAN);
    //     } else {
    //         e.disallowVehicleClass(-1, SVC_PEDESTRIAN);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_vehicle_classes(e: &mut NBEdge, classS: &str, allPermissions: SVCPermissions, defaultPermissions: SVCPermissions) {
        use utils::common::sumo_vehicle_class::*;
        let full = format!("0000000000{classS}");
        // shortcut: non-ASCII bytes in the class string would shift the indices (C++ indexes bytes), fine for valid input
        let cs = String::from_utf8_lossy(&full.as_bytes()[full.len() - 10..]).into_owned();
        let c = cs.as_bytes();
        // 0: allow all vehicle types
        if c[0] == b'1' {
            e.set_permissions(allPermissions, -1);
            return;
        }
        let mut haveCar = false;
        e.set_permissions(defaultPermissions, -1);
        // Passenger cars -- becomes SVC_PASSENGER
        if c[1] == b'1' {
            e.allow_vehicle_class(-1, SVC_PASSENGER);
            haveCar = true;
        } else {
            e.disallow_vehicle_class(-1, SVC_PASSENGER);
        }
        haveCar |= add_common_vehicle_classes(e, &cs, 2);
        if !haveCar {
            e.set_permissions(0, -1);
        }
        // Bicycle -- becomes SVC_BICYCLE
        if c[8] == b'1' {
            e.allow_vehicle_class(-1, SVC_BICYCLE);
        } else {
            e.disallow_vehicle_class(-1, SVC_BICYCLE);
        }
        // Pedestrian -- becomes SVC_PEDESTRIAN
        if c[9] == b'1' {
            e.allow_vehicle_class(-1, SVC_PEDESTRIAN);
        } else {
            e.disallow_vehicle_class(-1, SVC_PEDESTRIAN);
        }
    }

    /// C++: `void NINavTeqHelper::addVehicleClassesV6(NBEdge &, const std::string &, const SVCPermissions, const SVCPermissions)` [static]
    /// same as addVehicleClasses but for version 6+
    // void
    // NINavTeqHelper::addVehicleClassesV6(NBEdge& e, const std::string& oclassS, const SVCPermissions allPermissions, const SVCPermissions defaultPermissions) {
    //     std::string classS = "0000000000" + oclassS;
    //     classS = classS.substr(classS.length() - 12);
    //     // 0: allow all vehicle types
    //     if (classS[0] == '1') {
    //         e.setPermissions(allPermissions);
    //         return;
    //     }
    //     bool haveCar = false;
    //     e.setPermissions(defaultPermissions);
    //     // Passenger cars -- becomes SVC_PASSENGER
    //     if (classS[1] == '1') {
    //         e.allowVehicleClass(-1, SVC_PASSENGER);
    //         haveCar = true;
    //     } else {
    //         e.disallowVehicleClass(-1, SVC_PASSENGER);
    //     }
    //     // Residential Vehicle -- becomes SVC_PASSENGER
    //     if (classS[2] == '1') {
    //         e.allowVehicleClass(-1, SVC_PASSENGER);
    //         haveCar = true;
    //     }
    //     haveCar |= addCommonVehicleClasses(e, classS, 3);
    //     if (!haveCar) {
    //         e.setPermissions(0);
    //     }
    //     // Motorcycle -- becomes SVC_MOTORCYCLE
    //     if (classS[9] == '1') {
    //         e.allowVehicleClass(-1, SVC_MOTORCYCLE);
    //     } else {
    //         e.disallowVehicleClass(-1, SVC_MOTORCYCLE);
    //     }
    //     // Bicycle -- becomes SVC_BICYCLE
    //     if (classS[10] == '1') {
    //         e.allowVehicleClass(-1, SVC_BICYCLE);
    //     } else {
    //         e.disallowVehicleClass(-1, SVC_BICYCLE);
    //     }
    //     // Pedestrian -- becomes SVC_PEDESTRIAN
    //     if (classS[11] == '1') {
    //         e.allowVehicleClass(-1, SVC_PEDESTRIAN);
    //     } else {
    //         e.disallowVehicleClass(-1, SVC_PEDESTRIAN);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_vehicle_classes_v6(e: &mut NBEdge, classS: &str, allPermissions: SVCPermissions, defaultPermissions: SVCPermissions) {
        use utils::common::sumo_vehicle_class::*;
        let full = format!("0000000000{classS}");
        // shortcut: non-ASCII bytes in the class string would shift the indices (C++ indexes bytes), fine for valid input
        let cs = String::from_utf8_lossy(&full.as_bytes()[full.len() - 12..]).into_owned();
        let c = cs.as_bytes();
        // 0: allow all vehicle types
        if c[0] == b'1' {
            e.set_permissions(allPermissions, -1);
            return;
        }
        let mut haveCar = false;
        e.set_permissions(defaultPermissions, -1);
        // Passenger cars -- becomes SVC_PASSENGER
        if c[1] == b'1' {
            e.allow_vehicle_class(-1, SVC_PASSENGER);
            haveCar = true;
        } else {
            e.disallow_vehicle_class(-1, SVC_PASSENGER);
        }
        // Residential Vehicle -- becomes SVC_PASSENGER
        if c[2] == b'1' {
            e.allow_vehicle_class(-1, SVC_PASSENGER);
            haveCar = true;
        }
        haveCar |= add_common_vehicle_classes(e, &cs, 3);
        if !haveCar {
            e.set_permissions(0, -1);
        }
        // Motorcycle -- becomes SVC_MOTORCYCLE
        if c[9] == b'1' {
            e.allow_vehicle_class(-1, SVC_MOTORCYCLE);
        } else {
            e.disallow_vehicle_class(-1, SVC_MOTORCYCLE);
        }
        // Bicycle -- becomes SVC_BICYCLE
        if c[10] == b'1' {
            e.allow_vehicle_class(-1, SVC_BICYCLE);
        } else {
            e.disallow_vehicle_class(-1, SVC_BICYCLE);
        }
        // Pedestrian -- becomes SVC_PEDESTRIAN
        if c[11] == b'1' {
            e.allow_vehicle_class(-1, SVC_PEDESTRIAN);
        } else {
            e.disallow_vehicle_class(-1, SVC_PEDESTRIAN);
        }
    }

    /// C++: `bool NINavTeqHelper::addCommonVehicleClasses(NBEdge &, const std::string &, const int)` [static, private]
    // bool
    // NINavTeqHelper::addCommonVehicleClasses(NBEdge& e, const std::string& classS, const int offset) {
    //     bool haveCar = false;
    //     // High Occupancy Vehicle -- becomes SVC_PASSENGER|SVC_HOV
    //     if (classS[offset] == '1') {
    //         e.allowVehicleClass(-1, SVC_HOV);
    //         e.allowVehicleClass(-1, SVC_PASSENGER);
    //         haveCar = true;
    //     } else {
    //         e.disallowVehicleClass(-1, SVC_HOV);
    //     }
    //     // Emergency Vehicle -- becomes SVC_EMERGENCY
    //     if (classS[offset + 1] == '1') {
    //         e.allowVehicleClass(-1, SVC_EMERGENCY);
    //     } else {
    //         e.disallowVehicleClass(-1, SVC_EMERGENCY);
    //     }
    //     // Taxi -- becomes SVC_TAXI
    //     if (classS[offset + 2] == '1') {
    //         e.allowVehicleClass(-1, SVC_TAXI);
    //         haveCar = true;
    //     } else {
    //         e.disallowVehicleClass(-1, SVC_TAXI);
    //     }
    //     // Public Bus -- becomes SVC_BUS|SVC_COACH
    //     if (classS[offset + 3] == '1') {
    //         e.allowVehicleClass(-1, SVC_BUS);
    //         e.allowVehicleClass(-1, SVC_COACH);
    //         haveCar = true;
    //     } else {
    //         e.disallowVehicleClass(-1, SVC_BUS);
    //         e.disallowVehicleClass(-1, SVC_COACH);
    //     }
    //     // Delivery Truck -- becomes SVC_DELIVERY
    //     if (classS[offset + 4] == '1') {
    //         e.allowVehicleClass(-1, SVC_DELIVERY);
    //         haveCar = true;
    //     } else {
    //         e.disallowVehicleClass(-1, SVC_DELIVERY);
    //     }
    //     // Transport Truck -- becomes SVC_TRUCK|SVC_TRAILER
    //     if (classS[offset + 5] == '1') {
    //         e.allowVehicleClass(-1, SVC_TRUCK);
    //         e.allowVehicleClass(-1, SVC_TRAILER);
    //         haveCar = true;
    //     } else {
    //         e.disallowVehicleClass(-1, SVC_TRUCK);
    //         e.disallowVehicleClass(-1, SVC_TRAILER);
    //     }
    //     return haveCar;
    // }
    #[allow(unused_variables)]
    pub fn add_common_vehicle_classes(e: &mut NBEdge, classS: &str, offset: i32) -> bool {
        use utils::common::sumo_vehicle_class::*;
        let c = classS.as_bytes();
        let o = offset as usize;
        let mut haveCar = false;
        // High Occupancy Vehicle -- becomes SVC_PASSENGER|SVC_HOV
        if c[o] == b'1' {
            e.allow_vehicle_class(-1, SVC_HOV);
            e.allow_vehicle_class(-1, SVC_PASSENGER);
            haveCar = true;
        } else {
            e.disallow_vehicle_class(-1, SVC_HOV);
        }
        // Emergency Vehicle -- becomes SVC_EMERGENCY
        if c[o + 1] == b'1' {
            e.allow_vehicle_class(-1, SVC_EMERGENCY);
        } else {
            e.disallow_vehicle_class(-1, SVC_EMERGENCY);
        }
        // Taxi -- becomes SVC_TAXI
        if c[o + 2] == b'1' {
            e.allow_vehicle_class(-1, SVC_TAXI);
            haveCar = true;
        } else {
            e.disallow_vehicle_class(-1, SVC_TAXI);
        }
        // Public Bus -- becomes SVC_BUS|SVC_COACH
        if c[o + 3] == b'1' {
            e.allow_vehicle_class(-1, SVC_BUS);
            e.allow_vehicle_class(-1, SVC_COACH);
            haveCar = true;
        } else {
            e.disallow_vehicle_class(-1, SVC_BUS);
            e.disallow_vehicle_class(-1, SVC_COACH);
        }
        // Delivery Truck -- becomes SVC_DELIVERY
        if c[o + 4] == b'1' {
            e.allow_vehicle_class(-1, SVC_DELIVERY);
            haveCar = true;
        } else {
            e.disallow_vehicle_class(-1, SVC_DELIVERY);
        }
        // Transport Truck -- becomes SVC_TRUCK|SVC_TRAILER
        if c[o + 5] == b'1' {
            e.allow_vehicle_class(-1, SVC_TRUCK);
            e.allow_vehicle_class(-1, SVC_TRAILER);
            haveCar = true;
        } else {
            e.disallow_vehicle_class(-1, SVC_TRUCK);
            e.disallow_vehicle_class(-1, SVC_TRAILER);
        }
        haveCar
    }

}

