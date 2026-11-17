//! Port of libsumo/VehicleType.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::msg_handler::write_warning;
use utils::common::string_utils::starts_with;
use utils::common::sumo_vehicle_class::{get_vehicle_class_id, get_vehicle_shape_id, get_vehicle_shape_name, SUMO_VEHICLE_CLASS_STRINGS};
use utils::common::time::{steps2time, time2steps};
use crate::libsumo::helper::check_time_bounds;
use utils::common::to_string::to_string;

/// C++ class `libsumo::VehicleType` (libsumo/VehicleType.h)
pub struct VehicleType {
}

// C++ static members of VehicleType (port as module-level statics, see AGENTS.md):
//   static const int VehicleType::DOMAIN_ID;
//   static SubscriptionResults VehicleType::mySubscriptionResults;
//   static ContextSubscriptionResults VehicleType::myContextSubscriptionResults;

thread_local! {
    /// C++ static `VehicleType::mySubscriptionResults`
    static MY_SUBSCRIPTION_RESULTS: std::cell::RefCell<SubscriptionResults> = std::cell::RefCell::new(SubscriptionResults::new());
    /// C++ static `VehicleType::myContextSubscriptionResults`
    static MY_CONTEXT_SUBSCRIPTION_RESULTS: std::cell::RefCell<ContextSubscriptionResults> = std::cell::RefCell::new(ContextSubscriptionResults::new());
}

/// Unwraps a ProcessError-returning utility the way C++ propagates the exception.
fn unwrap_or_panic<T>(r: Result<T, ProcessError>) -> T {
    r.unwrap_or_else(|e| panic!("{}", e))
}

/// C++ `toString(SUMOVehicleClass)` (utils/common/ToString.h)
fn vehicle_class_to_string(vc: SUMOVehicleClass) -> String {
    SUMO_VEHICLE_CLASS_STRINGS.get_string(vc).unwrap().to_string()
}

/// C++ `toString(LatAlignmentDefinition)` (utils/common/ToString.h)
pub(crate) fn lat_alignment_to_string(lad: LatAlignmentDefinition) -> &'static str {
    match lad {
        LatAlignmentDefinition::RIGHT => "right",
        LatAlignmentDefinition::CENTER => "center",
        LatAlignmentDefinition::ARBITRARY => "arbitrary",
        LatAlignmentDefinition::NICE => "nice",
        LatAlignmentDefinition::COMPACT => "compact",
        LatAlignmentDefinition::LEFT => "left",
        LatAlignmentDefinition::GIVEN | LatAlignmentDefinition::DEFAULT => "",
    }
}

impl VehicleType {
    /// C++: `double VehicleType::getLength(const std::string &)` [static]
    // double
    // VehicleType::getLength(const std::string& typeID) {
    //     return getVType(typeID)->getLength();
    // }
    #[allow(unused_variables)]
    pub fn get_length(typeID: &str) -> f64 {
        Self::get_v_type(typeID.to_string()).get_length()
    }

    /// C++: `double VehicleType::getMaxSpeed(const std::string &)` [static]
    // double
    // VehicleType::getMaxSpeed(const std::string& typeID) {
    //     return getVType(typeID)->getMaxSpeed();
    // }
    #[allow(unused_variables)]
    pub fn get_max_speed(typeID: &str) -> f64 {
        Self::get_v_type(typeID.to_string()).get_max_speed()
    }

    /// C++: `std::string VehicleType::getVehicleClass(const std::string &)` [static]
    // std::string
    // VehicleType::getVehicleClass(const std::string& typeID) {
    //     return toString(getVType(typeID)->getVehicleClass());
    // }
    #[allow(unused_variables)]
    pub fn get_vehicle_class(typeID: &str) -> String {
        vehicle_class_to_string(Self::get_v_type(typeID.to_string()).get_vehicle_class())
    }

    /// C++: `double VehicleType::getSpeedFactor(const std::string &)` [static]
    // double
    // VehicleType::getSpeedFactor(const std::string& typeID) {
    //     return getVType(typeID)->getSpeedFactor().getParameter(0);
    // }
    #[allow(unused_variables)]
    pub fn get_speed_factor(typeID: &str) -> f64 {
        Self::get_v_type(typeID.to_string()).get_speed_factor().get_parameter(0)
    }

    /// C++: `double VehicleType::getAccel(const std::string &)` [static]
    // double
    // VehicleType::getAccel(const std::string& typeID) {
    //     return getVType(typeID)->getCarFollowModel().getMaxAccel();
    // }
    #[allow(unused_variables)]
    pub fn get_accel(typeID: &str) -> f64 {
        Self::get_v_type(typeID.to_string()).get_car_follow_model().get_max_accel()
    }

    /// C++: `double VehicleType::getDecel(const std::string &)` [static]
    // double
    // VehicleType::getDecel(const std::string& typeID) {
    //     return getVType(typeID)->getCarFollowModel().getMaxDecel();
    // }
    #[allow(unused_variables)]
    pub fn get_decel(typeID: &str) -> f64 {
        Self::get_v_type(typeID.to_string()).get_car_follow_model().get_max_decel()
    }

    /// C++: `double VehicleType::getEmergencyDecel(const std::string &)` [static]
    // double
    // VehicleType::getEmergencyDecel(const std::string& typeID) {
    //     return getVType(typeID)->getCarFollowModel().getEmergencyDecel();
    // }
    #[allow(unused_variables)]
    pub fn get_emergency_decel(typeID: &str) -> f64 {
        Self::get_v_type(typeID.to_string()).get_car_follow_model().get_emergency_decel()
    }

    /// C++: `double VehicleType::getApparentDecel(const std::string &)` [static]
    // double
    // VehicleType::getApparentDecel(const std::string& typeID) {
    //     return getVType(typeID)->getCarFollowModel().getApparentDecel();
    // }
    #[allow(unused_variables)]
    pub fn get_apparent_decel(typeID: &str) -> f64 {
        Self::get_v_type(typeID.to_string()).get_car_follow_model().get_apparent_decel()
    }

    /// C++: `double VehicleType::getImperfection(const std::string &)` [static]
    // double
    // VehicleType::getImperfection(const std::string& typeID) {
    //     return getVType(typeID)->getCarFollowModel().getImperfection();
    // }
    #[allow(unused_variables)]
    pub fn get_imperfection(typeID: &str) -> f64 {
        Self::get_v_type(typeID.to_string()).get_car_follow_model().get_imperfection()
    }

    /// C++: `double VehicleType::getTau(const std::string &)` [static]
    // double
    // VehicleType::getTau(const std::string& typeID) {
    //     return getVType(typeID)->getCarFollowModel().getHeadwayTime();
    // }
    #[allow(unused_variables)]
    pub fn get_tau(typeID: &str) -> f64 {
        Self::get_v_type(typeID.to_string()).get_car_follow_model().get_headway_time()
    }

    /// C++: `std::string VehicleType::getEmissionClass(const std::string &)` [static]
    // std::string
    // VehicleType::getEmissionClass(const std::string& typeID) {
    //     return PollutantsInterface::getName(getVType(typeID)->getEmissionClass());
    // }
    #[allow(unused_variables)]
    pub fn get_emission_class(typeID: &str) -> String {
        PollutantsInterface::get_name(Self::get_v_type(typeID.to_string()).get_emission_class())
    }

    /// C++: `std::string VehicleType::getShapeClass(const std::string &)` [static]
    // std::string
    // VehicleType::getShapeClass(const std::string& typeID) {
    //     return getVehicleShapeName(getVType(typeID)->getGuiShape());
    // }
    #[allow(unused_variables)]
    pub fn get_shape_class(typeID: &str) -> String {
        get_vehicle_shape_name(Self::get_v_type(typeID.to_string()).get_gui_shape())
    }

    /// C++: `double VehicleType::getMinGap(const std::string &)` [static]
    // double
    // VehicleType::getMinGap(const std::string& typeID) {
    //     return getVType(typeID)->getMinGap();
    // }
    #[allow(unused_variables)]
    pub fn get_min_gap(typeID: &str) -> f64 {
        Self::get_v_type(typeID.to_string()).get_min_gap()
    }

    /// C++: `double VehicleType::getWidth(const std::string &)` [static]
    // double
    // VehicleType::getWidth(const std::string& typeID) {
    //     return getVType(typeID)->getWidth();
    // }
    #[allow(unused_variables)]
    pub fn get_width(typeID: &str) -> f64 {
        Self::get_v_type(typeID.to_string()).get_width()
    }

    /// C++: `double VehicleType::getHeight(const std::string &)` [static]
    // double
    // VehicleType::getHeight(const std::string& typeID) {
    //     return getVType(typeID)->getHeight();
    // }
    #[allow(unused_variables)]
    pub fn get_height(typeID: &str) -> f64 {
        Self::get_v_type(typeID.to_string()).get_height()
    }

    /// C++: `double VehicleType::getMass(const std::string &)` [static]
    // double
    // VehicleType::getMass(const std::string& typeID) {
    //     return getVType(typeID)->getMass();
    // }
    #[allow(unused_variables)]
    pub fn get_mass(typeID: &str) -> f64 {
        Self::get_v_type(typeID.to_string()).get_mass()
    }

    /// C++: `libsumo::TraCIColor VehicleType::getColor(const std::string &)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_color(typeID: &str) -> TraCIColor {
        Helper::make_tra_ci_color(Self::get_v_type(typeID.to_string()).get_color())
    }

    /// C++: `double VehicleType::getMinGapLat(const std::string &)` [static]
    // double
    // VehicleType::getMinGapLat(const std::string& typeID) {
    //     return getVType(typeID)->getMinGapLat();
    // }
    #[allow(unused_variables)]
    pub fn get_min_gap_lat(typeID: &str) -> f64 {
        Self::get_v_type(typeID.to_string()).get_min_gap_lat()
    }

    /// C++: `double VehicleType::getMaxSpeedLat(const std::string &)` [static]
    // double
    // VehicleType::getMaxSpeedLat(const std::string& typeID) {
    //     return getVType(typeID)->getMaxSpeedLat();
    // }
    #[allow(unused_variables)]
    pub fn get_max_speed_lat(typeID: &str) -> f64 {
        Self::get_v_type(typeID.to_string()).get_max_speed_lat()
    }

    /// C++: `std::string VehicleType::getLateralAlignment(const std::string &)` [static]
    // std::string
    // VehicleType::getLateralAlignment(const std::string& typeID) {
    //     if (getVType(typeID)->getPreferredLateralAlignment() != LatAlignmentDefinition::GIVEN) {
    //         return toString(getVType(typeID)->getPreferredLateralAlignment());
    //     } else {
    //         return toString(getVType(typeID)->getPreferredLateralAlignmentOffset());
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_lateral_alignment(typeID: &str) -> String {
        let v = Self::get_v_type(typeID.to_string());
        if *v.get_preferred_lateral_alignment() != LatAlignmentDefinition::GIVEN {
            lat_alignment_to_string(*v.get_preferred_lateral_alignment()).to_string()
        } else {
            to_string(&v.get_preferred_lateral_alignment_offset())
        }
    }

    /// C++: `int VehicleType::getPersonCapacity(const std::string &)` [static]
    // int
    // VehicleType::getPersonCapacity(const std::string& typeID) {
    //     return getVType(typeID)->getPersonCapacity();
    // }
    #[allow(unused_variables)]
    pub fn get_person_capacity(typeID: &str) -> i32 {
        Self::get_v_type(typeID.to_string()).get_person_capacity()
    }

    /// C++: `double VehicleType::getActionStepLength(const std::string &)` [static]
    // double
    // VehicleType::getActionStepLength(const std::string& typeID) {
    //     return getVType(typeID)->getActionStepLengthSecs();
    // }
    #[allow(unused_variables)]
    pub fn get_action_step_length(typeID: &str) -> f64 {
        Self::get_v_type(typeID.to_string()).get_action_step_length_secs()
    }

    /// C++: `double VehicleType::getSpeedDeviation(const std::string &)` [static]
    // double
    // VehicleType::getSpeedDeviation(const std::string& typeID) {
    //     return getVType(typeID)->getSpeedFactor().getParameter(1);
    // }
    #[allow(unused_variables)]
    pub fn get_speed_deviation(typeID: &str) -> f64 {
        Self::get_v_type(typeID.to_string()).get_speed_factor().get_parameter(1)
    }

    /// C++: `double VehicleType::getBoardingDuration(const std::string &)` [static]
    // double
    // VehicleType::getBoardingDuration(const std::string& typeID) {
    //     return STEPS2TIME(getVType(typeID)->getBoardingDuration(true));
    // }
    #[allow(unused_variables)]
    pub fn get_boarding_duration(typeID: &str) -> f64 {
        steps2time(Self::get_v_type(typeID.to_string()).get_boarding_duration(true))
    }

    /// C++: `double VehicleType::getImpatience(const std::string &)` [static]
    // double
    // VehicleType::getImpatience(const std::string& typeID) {
    //     return getVType(typeID)->getImpatience();
    // }
    #[allow(unused_variables)]
    pub fn get_impatience(typeID: &str) -> f64 {
        Self::get_v_type(typeID.to_string()).get_impatience()
    }

    /// C++: `std::vector<std::string> VehicleType::getIDList()` [static]
    // std::vector<std::string>
    // VehicleType::getIDList() {
    //     std::vector<std::string> ids;
    //     MSNet::getInstance()->getVehicleControl().insertVTypeIDs(ids);
    //     return ids;
    // }
    #[allow(unused_variables)]
    pub fn get_id_list() -> Vec<String> {
        let mut ids: Vec<String> = Vec::new();
        MSNet::get_instance().m().get_vehicle_control().insert_v_type_i_ds(&mut ids);
        ids
    }

    /// C++: `int VehicleType::getIDCount()` [static]
    // int
    // VehicleType::getIDCount() {
    //     return (int)getIDList().size();
    // }
    #[allow(unused_variables)]
    pub fn get_id_count() -> i32 {
        Self::get_id_list().len() as i32
    }

    /// C++: `std::string VehicleType::getParameter(const std::string &, const std::string &)` [static]
    // std::string
    // VehicleType::getParameter(const std::string& typeID, const std::string& key) {
    //     if (StringUtils::startsWith(key, "junctionModel.")) {
    //         const std::string attrName = key.substr(14);
    //         if (!SUMOXMLDefinitions::Attrs.hasString(attrName)) {
    //             throw TraCIException("Invalid junctionModel parameter '" + key + "' for type '" + typeID + "'");
    //         }
    //         SumoXMLAttr attr = (SumoXMLAttr)SUMOXMLDefinitions::Attrs.get(attrName);
    //         if (SUMOVTypeParameter::AllowedJMAttrs.count(attr) == 0) {
    //             throw TraCIException("Invalid junctionModel parameter '" + key + "' for type '" + typeID + "'");
    //         }
    //         if (getVType(typeID)->getParameter().jmParameter.count(attr) != 0) {
    //             return getVType(typeID)->getParameter().jmParameter.find(attr)->second;
    //         } else {
    //             return "";
    //         }
    //     } else {
    //         return getVType(typeID)->getParameter().getParameter(key, "");
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_parameter(typeID: &str, key: &str) -> String {
        if starts_with(key, "junctionModel.") {
            let attrName = &key[14..];
            if !Attrs.has_string(attrName) {
                panic!("Invalid junctionModel parameter '{}' for type '{}'", key, typeID);
            }
            let attr = Attrs.get(attrName).unwrap() as SumoXMLAttr;
            if !allowed_jm_attrs().contains(&attr) {
                panic!("Invalid junctionModel parameter '{}' for type '{}'", key, typeID);
            }
            match Self::get_v_type(typeID.to_string()).get_parameter().jmParameter.get(&attr) {
                Some(v) => v.clone(),
                None => String::new(),
            }
        } else {
            Self::get_v_type(typeID.to_string()).get_parameter().get_parameter(key, "")
        }
    }

    /// C++: `const std::pair<std::string, std::string> VehicleType::getParameterWithKey(const std::string &, const std::string &)` [static]
    // LIBSUMO_GET_PARAMETER_WITH_KEY_IMPLEMENTATION(VehicleType)
    #[allow(unused_variables)]
    pub fn get_parameter_with_key(objectID: &str, key: &str) -> (String, String) {
        (key.to_string(), Self::get_parameter(objectID, key))
    }

    /// C++: `void VehicleType::setParameter(const std::string &, const std::string &, const std::string &)` [static]
    // void
    // VehicleType::setParameter(const std::string& typeID, const std::string& name, const std::string& value) {
    //     if (StringUtils::startsWith(name, "junctionModel.")) {
    //         const std::string attrName = name.substr(14);
    //         if (!SUMOXMLDefinitions::Attrs.hasString(attrName)) {
    //             throw TraCIException("Invalid junctionModel parameter '" + name + "' for type '" + typeID + "'");
    //         }
    //         SumoXMLAttr attr = (SumoXMLAttr)SUMOXMLDefinitions::Attrs.get(attrName);
    //         if (SUMOVTypeParameter::AllowedJMAttrs.count(attr) == 0) {
    //             throw TraCIException("Invalid junctionModel parameter '" + name + "' for type '" + typeID + "'");
    //         }
    //         try {
    //             StringUtils::toDouble(value); // check number format
    //             ((SUMOVTypeParameter&)getVType(typeID)->getParameter()).jmParameter[attr] = value;
    //         } catch (NumberFormatException&) {
    //             throw TraCIException("Invalid junctionModel parameter value '" + value + "' for type '" + typeID + " (should be numeric)'");
    //         }
    //     } else {
    //         ((SUMOVTypeParameter&)getVType(typeID)->getParameter()).setParameter(name, value);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_parameter(objectID: &str, key: &str, value: &str) {
        if starts_with(key, "junctionModel.") {
            let attrName = &key[14..];
            if !Attrs.has_string(attrName) {
                panic!("Invalid junctionModel parameter '{}' for type '{}'", key, objectID);
            }
            let attr = Attrs.get(attrName).unwrap() as SumoXMLAttr;
            if !allowed_jm_attrs().contains(&attr) {
                panic!("Invalid junctionModel parameter '{}' for type '{}'", key, objectID);
            }
            if utils::common::string_utils::to_double(value).is_err() {
                panic!("Invalid junctionModel parameter value '{}' for type '{} (should be numeric)'", value, objectID);
            }
            Self::get_v_type(objectID.to_string()).m().myParameter.jmParameter.insert(attr, value.to_string());
        } else {
            Self::get_v_type(objectID.to_string()).m().myParameter.set_parameter(key, value);
        }
    }

    /// C++: `void VehicleType::setLength(const std::string &, double)` [static]
    // void
    // VehicleType::setLength(const std::string& typeID, double length)  {
    //     getVType(typeID)->setLength(length);
    // }
    #[allow(unused_variables)]
    pub fn set_length(typeID: &str, length: f64) {
        Self::get_v_type(typeID.to_string()).m().set_length(&length);
    }

    /// C++: `void VehicleType::setMaxSpeed(const std::string &, double)` [static]
    // void
    // VehicleType::setMaxSpeed(const std::string& typeID, double speed)  {
    //     getVType(typeID)->setMaxSpeed(speed);
    // }
    #[allow(unused_variables)]
    pub fn set_max_speed(typeID: &str, speed: f64) {
        Self::get_v_type(typeID.to_string()).m().set_max_speed(&speed);
    }

    /// C++: `void VehicleType::setVehicleClass(const std::string &, const std::string &)` [static]
    // void
    // VehicleType::setVehicleClass(const std::string& typeID, const std::string& clazz)  {
    //     getVType(typeID)->setVClass(getVehicleClassID(clazz));
    // }
    #[allow(unused_variables)]
    pub fn set_vehicle_class(typeID: &str, clazz: &str) {
        Self::get_v_type(typeID.to_string()).m().set_v_class(unwrap_or_panic(get_vehicle_class_id(clazz)));
    }

    /// C++: `void VehicleType::setSpeedFactor(const std::string &, double)` [static]
    // void
    // VehicleType::setSpeedFactor(const std::string& typeID, double factor)  {
    //     getVType(typeID)->setSpeedFactor(factor);
    // }
    #[allow(unused_variables)]
    pub fn set_speed_factor(typeID: &str, factor: f64) {
        Self::get_v_type(typeID.to_string()).m().set_speed_factor(&factor);
    }

    /// C++: `void VehicleType::setAccel(const std::string &, double)` [static]
    // void
    // VehicleType::setAccel(const std::string& typeID, double accel)  {
    //     getVType(typeID)->setAccel(accel);
    // }
    #[allow(unused_variables)]
    pub fn set_accel(typeID: &str, accel: f64) {
        Self::get_v_type(typeID.to_string()).m().set_accel(accel);
    }

    /// C++: `void VehicleType::setDecel(const std::string &, double)` [static]
    // void
    // VehicleType::setDecel(const std::string& typeID, double decel)  {
    //     MSVehicleType* v = getVType(typeID);
    //     v->setDecel(decel);
    //     // automatically raise emergencyDecel to ensure it is at least as high as decel
    //     if (decel > v->getCarFollowModel().getEmergencyDecel()) {
    //         if (v->getParameter().cfParameter.count(SUMO_ATTR_EMERGENCYDECEL) > 0) {
    //             // notify user only if emergencyDecel was previously specified
    //             WRITE_WARNINGF(TL("Automatically setting emergencyDecel to % for vType '%' to match decel."), toString(decel), typeID);
    //         }
    //         v->setEmergencyDecel(decel);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_decel(typeID: &str, decel: f64) {
        let mut v = Self::get_v_type(typeID.to_string());
        v.m().set_decel(decel);
        // automatically raise emergencyDecel to ensure it is at least as high as decel
        if decel > v.get_car_follow_model().get_emergency_decel() {
            if v.get_parameter().cfParameter.contains_key(&SUMO_ATTR_EMERGENCYDECEL) {
                // notify user only if emergencyDecel was previously specified
                write_warning(&format!("Automatically setting emergencyDecel to {} for vType '{}' to match decel.", to_string(&decel), typeID));
            }
            v.m().set_emergency_decel(decel);
        }
    }

    /// C++: `void VehicleType::setEmergencyDecel(const std::string &, double)` [static]
    // void
    // VehicleType::setEmergencyDecel(const std::string& typeID, double decel)  {
    //     MSVehicleType* v = getVType(typeID);
    //     v->setEmergencyDecel(decel);
    //     if (decel < v->getCarFollowModel().getMaxDecel()) {
    //         WRITE_WARNINGF(TL("New value of emergencyDecel (%) is lower than decel (%)"), toString(decel), toString(v->getCarFollowModel().getMaxDecel()));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_emergency_decel(typeID: &str, decel: f64) {
        let mut v = Self::get_v_type(typeID.to_string());
        v.m().set_emergency_decel(decel);
        if decel < v.get_car_follow_model().get_max_decel() {
            write_warning(&format!("New value of emergencyDecel ({}) is lower than decel ({})", to_string(&decel), to_string(&v.get_car_follow_model().get_max_decel())));
        }
    }

    /// C++: `void VehicleType::setApparentDecel(const std::string &, double)` [static]
    // void
    // VehicleType::setApparentDecel(const std::string& typeID, double decel)  {
    //     getVType(typeID)->setApparentDecel(decel);
    // }
    #[allow(unused_variables)]
    pub fn set_apparent_decel(typeID: &str, decel: f64) {
        Self::get_v_type(typeID.to_string()).m().set_apparent_decel(decel);
    }

    /// C++: `void VehicleType::setImperfection(const std::string &, double)` [static]
    // void
    // VehicleType::setImperfection(const std::string& typeID, double imperfection)  {
    //     getVType(typeID)->setImperfection(imperfection);
    // }
    #[allow(unused_variables)]
    pub fn set_imperfection(typeID: &str, imperfection: f64) {
        Self::get_v_type(typeID.to_string()).m().set_imperfection(imperfection);
    }

    /// C++: `void VehicleType::setTau(const std::string &, double)` [static]
    // void
    // VehicleType::setTau(const std::string& typeID, double tau)  {
    //     getVType(typeID)->setTau(tau);
    // }
    #[allow(unused_variables)]
    pub fn set_tau(typeID: &str, tau: f64) {
        Self::get_v_type(typeID.to_string()).m().set_tau(tau);
    }

    /// C++: `void VehicleType::setEmissionClass(const std::string &, const std::string &)` [static]
    // void
    // VehicleType::setEmissionClass(const std::string& typeID, const std::string& clazz)  {
    //     getVType(typeID)->setEmissionClass(PollutantsInterface::getClassByName(clazz));
    // }
    #[allow(unused_variables)]
    pub fn set_emission_class(typeID: &str, clazz: &str) {
        Self::get_v_type(typeID.to_string()).m().set_emission_class(PollutantsInterface::get_class_by_name(clazz, SVC_IGNORING));
    }

    /// C++: `void VehicleType::setShapeClass(const std::string &, const std::string &)` [static]
    // void
    // VehicleType::setShapeClass(const std::string& typeID, const std::string& shapeClass)  {
    //     getVType(typeID)->setShape(getVehicleShapeID(shapeClass));
    // }
    #[allow(unused_variables)]
    pub fn set_shape_class(typeID: &str, shapeClass: &str) {
        Self::get_v_type(typeID.to_string()).m().set_shape(unwrap_or_panic(get_vehicle_shape_id(shapeClass)));
    }

    /// C++: `void VehicleType::setWidth(const std::string &, double)` [static]
    // void
    // VehicleType::setWidth(const std::string& typeID, double width)  {
    //     getVType(typeID)->setWidth(width);
    // }
    #[allow(unused_variables)]
    pub fn set_width(typeID: &str, width: f64) {
        Self::get_v_type(typeID.to_string()).m().set_width(&width);
    }

    /// C++: `void VehicleType::setHeight(const std::string &, double)` [static]
    // void
    // VehicleType::setHeight(const std::string& typeID, double height)  {
    //     getVType(typeID)->setHeight(height);
    // }
    #[allow(unused_variables)]
    pub fn set_height(typeID: &str, height: f64) {
        Self::get_v_type(typeID.to_string()).m().set_height(&height);
    }

    /// C++: `void VehicleType::setMass(const std::string &, double)` [static]
    // void
    // VehicleType::setMass(const std::string& typeID, double mass)  {
    //     getVType(typeID)->setMass(mass);
    // }
    #[allow(unused_variables)]
    pub fn set_mass(typeID: &str, mass: f64) {
        Self::get_v_type(typeID.to_string()).m().set_mass(mass);
    }

    /// C++: `void VehicleType::setColor(const std::string &, const libsumo::TraCIColor &)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn set_color(typeID: &str, color: &TraCIColor) {
        Self::get_v_type(typeID.to_string()).m().set_color(&Helper::make_rgb_color(color));
    }

    /// C++: `void VehicleType::setMinGap(const std::string &, double)` [static]
    // void
    // VehicleType::setMinGap(const std::string& typeID, double minGap)  {
    //     getVType(typeID)->setMinGap(minGap);
    // }
    #[allow(unused_variables)]
    pub fn set_min_gap(typeID: &str, minGap: f64) {
        Self::get_v_type(typeID.to_string()).m().set_min_gap(&minGap);
    }

    /// C++: `void VehicleType::setMinGapLat(const std::string &, double)` [static]
    // void
    // VehicleType::setMinGapLat(const std::string& typeID, double minGapLat)  {
    //     getVType(typeID)->setMinGapLat(minGapLat);
    // }
    #[allow(unused_variables)]
    pub fn set_min_gap_lat(typeID: &str, minGapLat: f64) {
        Self::get_v_type(typeID.to_string()).m().set_min_gap_lat(&minGapLat);
    }

    /// C++: `void VehicleType::setMaxSpeedLat(const std::string &, double)` [static]
    // void
    // VehicleType::setMaxSpeedLat(const std::string& typeID, double speed)  {
    //     getVType(typeID)->setMaxSpeedLat(speed);
    // }
    #[allow(unused_variables)]
    pub fn set_max_speed_lat(typeID: &str, speed: f64) {
        Self::get_v_type(typeID.to_string()).m().set_max_speed_lat(&speed);
    }

    /// C++: `void VehicleType::setLateralAlignment(const std::string &, const std::string &)` [static]
    // void
    // VehicleType::setLateralAlignment(const std::string& typeID, const std::string& latAlignment)  {
    //     double lao;
    //     LatAlignmentDefinition lad;
    //     if (SUMOVTypeParameter::parseLatAlignment(latAlignment, lao, lad)) {
    //         getVType(typeID)->setPreferredLateralAlignment(lad, lao);
    //     } else {
    //         throw TraCIException("Unknown value '" + latAlignment + "' when setting latAlignment for vType '" + typeID + "';\n must be one of (\"right\", \"center\", \"arbitrary\", \"nice\", \"compact\", \"left\" or a float)");
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_lateral_alignment(typeID: &str, latAlignment: &str) {
        let mut lao: f64 = 0.0;
        let mut lad = LatAlignmentDefinition::DEFAULT;
        if SUMOVTypeParameter::parse_lat_alignment(latAlignment, &mut lao, &mut lad) {
            Self::get_v_type(typeID.to_string()).m().set_preferred_lateral_alignment(&lad, lao);
        } else {
            let msg = format!("Unknown value '{}' when setting latAlignment for vType '{}';\n must be one of (\"right\", \"center\", \"arbitrary\", \"nice\", \"compact\", \"left\" or a float)", latAlignment, typeID);
            panic!("{}", msg);
        }
    }

    /// C++: `void VehicleType::setActionStepLength(const std::string &, double, bool)` [static]
    // void
    // VehicleType::setActionStepLength(const std::string& typeID, double actionStepLength, bool resetActionOffset)  {
    //     getVType(typeID)->setActionStepLength(SUMOVehicleParserHelper::processActionStepLength(actionStepLength), resetActionOffset);
    // }
    #[allow(unused_variables)]
    pub fn set_action_step_length(typeID: &str, actionStepLength: f64, resetActionOffset: bool) {
        Self::get_v_type(typeID.to_string()).m().set_action_step_length(SUMOVehicleParserHelper::process_action_step_length(actionStepLength), resetActionOffset);
    }

    /// C++: `void VehicleType::setBoardingDuration(const std::string &, double)` [static]
    // void
    // VehicleType::setBoardingDuration(const std::string& typeID, double boardingDuration)  {
    //     try {
    //         checkTimeBounds(boardingDuration);
    //     } catch (ProcessError&) {
    //         throw TraCIException("BoardingDuration parameter exceeds the time value range.");
    //     }
    //     getVType(typeID)->setBoardingDuration(TIME2STEPS(boardingDuration), true);
    // }
    #[allow(unused_variables)]
    pub fn set_boarding_duration(typeID: &str, boardingDuration: f64) {
        if !check_time_bounds(boardingDuration) {
            panic!("BoardingDuration parameter exceeds the time value range.");
        }
        Self::get_v_type(typeID.to_string()).m().set_boarding_duration(time2steps(boardingDuration), true);
    }

    /// C++: `void VehicleType::setImpatience(const std::string &, double)` [static]
    // void
    // VehicleType::setImpatience(const std::string& typeID, double impatience)  {
    //     getVType(typeID)->setImpatience(impatience);
    // }
    #[allow(unused_variables)]
    pub fn set_impatience(typeID: &str, impatience: f64) {
        Self::get_v_type(typeID.to_string()).m().set_impatience(impatience);
    }

    /// C++: `void VehicleType::copy(const std::string &, const std::string &)` [static]
    // void
    // VehicleType::copy(const std::string& origTypeID, const std::string& newTypeID)  {
    //     getVType(origTypeID)->duplicateType(newTypeID, true);
    // }
    #[allow(unused_variables)]
    pub fn copy(origTypeID: &str, newTypeID: &str) {
        Self::get_v_type(origTypeID.to_string()).duplicate_type(newTypeID, true);
    }

    /// C++: `void VehicleType::setSpeedDeviation(const std::string &, double)` [static]
    // void
    // VehicleType::setSpeedDeviation(const std::string& typeID, double deviation)  {
    //     getVType(typeID)->setSpeedDeviation(deviation);
    // }
    #[allow(unused_variables)]
    pub fn set_speed_deviation(typeID: &str, deviation: f64) {
        Self::get_v_type(typeID.to_string()).m().set_speed_deviation(&deviation);
    }

    /// C++: `double VehicleType::getScale(const std::string &)` [static]
    // double
    // VehicleType::getScale(const std::string& typeID) {
    //     return getVType(typeID)->getParameter().scale;
    // }
    #[allow(unused_variables)]
    pub fn get_scale(typeID: &str) -> f64 {
        Self::get_v_type(typeID.to_string()).get_parameter().scale
    }

    /// C++: `void VehicleType::setScale(const std::string &, double)` [static]
    // void
    // VehicleType::setScale(const std::string& typeID, double value)  {
    //     getVType(typeID)->setScale(value);
    // }
    #[allow(unused_variables)]
    pub fn set_scale(typeID: &str, value: f64) {
        Self::get_v_type(typeID.to_string()).m().set_scale(value);
    }

    /// C++: `void VehicleType::subscribe(const std::string &, const std::vector<int> &, double, double, const libsumo::TraCIResults &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(VehicleType, VEHICLETYPE)
    #[allow(unused_variables)]
    pub fn subscribe(objectID: &str, varIDs: &Vec<i32>, begin: f64, end: f64, parameters: &TraCIResults) {
        Helper::subscribe(CMD_SUBSCRIBE_VEHICLETYPE_VARIABLE, objectID, varIDs, begin, end, parameters, 0, 0.0);
    }

    /// C++: `void VehicleType::unsubscribe(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(VehicleType, VEHICLETYPE)
    #[allow(unused_variables)]
    pub fn unsubscribe(objectID: &str) {
        Helper::subscribe(CMD_SUBSCRIBE_VEHICLETYPE_VARIABLE, objectID, &Vec::new(), INVALID_DOUBLE_VALUE, INVALID_DOUBLE_VALUE, &TraCIResults::new(), 0, 0.0);
    }

    /// C++: `void VehicleType::subscribeContext(const std::string &, int, double, const std::vector<int> &, double, double, const libsumo::TraCIResults &)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn subscribe_context(objectID: &str, domain: i32, dist: f64, varIDs: &Vec<i32>, begin: f64, end: f64, parameters: &TraCIResults) {
        Helper::subscribe(CMD_SUBSCRIBE_VEHICLETYPE_CONTEXT, objectID, varIDs, begin, end, parameters, domain, dist);
    }

    /// C++: `void VehicleType::unsubscribeContext(const std::string &, int, double)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(VehicleType, VEHICLETYPE)
    #[allow(unused_variables)]
    pub fn unsubscribe_context(objectID: &str, domain: i32, dist: f64) {
        Helper::subscribe(CMD_SUBSCRIBE_VEHICLETYPE_CONTEXT, objectID, &Vec::new(), INVALID_DOUBLE_VALUE, INVALID_DOUBLE_VALUE, &TraCIResults::new(), domain, dist);
    }

    /// C++: `const libsumo::SubscriptionResults VehicleType::getAllSubscriptionResults()` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(VehicleType, VEHICLETYPE)
    #[allow(unused_variables)]
    pub fn get_all_subscription_results() -> SubscriptionResults {
        MY_SUBSCRIPTION_RESULTS.with(|r| r.borrow().clone())
    }

    /// C++: `const libsumo::TraCIResults VehicleType::getSubscriptionResults(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(VehicleType, VEHICLETYPE)
    #[allow(unused_variables)]
    pub fn get_subscription_results(objectID: &str) -> TraCIResults {
        MY_SUBSCRIPTION_RESULTS.with(|r| r.borrow_mut().entry(objectID.to_string()).or_default().clone())
    }

    /// C++: `const libsumo::ContextSubscriptionResults VehicleType::getAllContextSubscriptionResults()` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(VehicleType, VEHICLETYPE)
    #[allow(unused_variables)]
    pub fn get_all_context_subscription_results() -> ContextSubscriptionResults {
        MY_CONTEXT_SUBSCRIPTION_RESULTS.with(|r| r.borrow().clone())
    }

    /// C++: `const libsumo::SubscriptionResults VehicleType::getContextSubscriptionResults(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(VehicleType, VEHICLETYPE)
    #[allow(unused_variables)]
    pub fn get_context_subscription_results(objectID: &str) -> SubscriptionResults {
        MY_CONTEXT_SUBSCRIPTION_RESULTS.with(|r| r.borrow_mut().entry(objectID.to_string()).or_default().clone())
    }

    /// C++: `void VehicleType::subscribeParameterWithKey(const std::string &, const std::string &, double, double)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(VehicleType, VEHICLETYPE)
    #[allow(unused_variables)]
    pub fn subscribe_parameter_with_key(objectID: &str, key: &str, beginTime: f64, endTime: f64) {
        let mut params: TraCIResults = TraCIResults::new();
        params.insert(VAR_PARAMETER_WITH_KEY, crate::libsumo::helper::as_traci_result(TraCIString { base: TraCIResult { result_type: TYPE_STRING }, value: key.to_string() }));
        Helper::subscribe(CMD_SUBSCRIBE_VEHICLETYPE_VARIABLE, objectID, &vec![VAR_PARAMETER_WITH_KEY], beginTime, endTime, &params, 0, 0.0);
    }

    /// C++: `int VehicleType::domainID()` [static]
    // LIBSUMO_SUBSCRIPTION_API
    #[allow(unused_variables)]
    pub fn domain_id() -> i32 {
        CMD_GET_VEHICLETYPE_VARIABLE
    }

    /// C++: `MSVehicleType * VehicleType::getVType(std::string)` [static]
    // MSVehicleType*
    // VehicleType::getVType(std::string id) {
    //     MSVehicleType* t = MSNet::getInstance()->getVehicleControl().getVType(id);
    //     if (t == nullptr) {
    //         throw TraCIException("Vehicle type '" + id + "' is not known");
    //     }
    //     return t;
    // }
    #[allow(unused_variables)]
    pub fn get_v_type(id: String) -> Ptr<MSVehicleType> {
        let t = MSNet::get_instance().m().get_vehicle_control().get_v_type(&id, Ptr::null(), false);
        if t.is_null() {
            let msg = format!("Vehicle type '{}' is not known", id);
            panic!("{}", msg);
        }
        t
    }

    /// C++: `std::shared_ptr<VariableWrapper> VehicleType::makeWrapper()` [static]
    // std::shared_ptr<VariableWrapper>
    // VehicleType::makeWrapper() {
    //     return std::make_shared<Helper::SubscriptionWrapper>(handleVariable, mySubscriptionResults, myContextSubscriptionResults);
    // }
    #[allow(unused_variables)]
    pub fn make_wrapper() -> Rc<VariableWrapper> {
        Helper::make_subscription_wrapper(Some(Self::handle_variable), unsafe { &mut *MY_SUBSCRIPTION_RESULTS.with(|r| r.as_ptr()) }, unsafe { &mut *MY_CONTEXT_SUBSCRIPTION_RESULTS.with(|r| r.as_ptr()) })
    }

    /// C++: `bool VehicleType::handleVariable(const std::string &, const int, VariableWrapper *, tcpip::Storage *)` [static]
    // bool
    // VehicleType::handleVariable(const std::string& objID, const int variable, VariableWrapper* wrapper, tcpip::Storage* paramData) {
    //     return handleVariableWithID(objID, objID, variable, wrapper, paramData);
    // }
    #[allow(unused_variables)]
    pub fn handle_variable(objID: &str, variable: i32, wrapper: Ptr<VariableWrapper>, paramData: Ptr<Storage>) -> bool {
        Self::handle_variable_with_id(objID, objID, variable, wrapper, paramData)
    }

    /// C++: `bool VehicleType::handleVariableWithID(const std::string &, const std::string &, const int, VariableWrapper *, tcpip::Storage *)` [static]
    // bool
    // VehicleType::handleVariableWithID(const std::string& objID, const std::string& typeID, const int variable, VariableWrapper* wrapper, tcpip::Storage* paramData) {
    //     switch (variable) {
    //         case TRACI_ID_LIST:
    //             return wrapper->wrapStringList(objID, variable, getIDList());
    //         case ID_COUNT:
    //             return wrapper->wrapInt(objID, variable, getIDCount());
    //         case VAR_LENGTH:
    //             return wrapper->wrapDouble(objID, variable, getLength(typeID));
    //         case VAR_HEIGHT:
    //             return wrapper->wrapDouble(objID, variable, getHeight(typeID));
    //         case VAR_MASS:
    //             return wrapper->wrapDouble(objID, variable, getMass(typeID));
    //         case VAR_MINGAP:
    //             return wrapper->wrapDouble(objID, variable, getMinGap(typeID));
    //         case VAR_MAXSPEED:
    //             return wrapper->wrapDouble(objID, variable, getMaxSpeed(typeID));
    //         case VAR_ACCEL:
    //             return wrapper->wrapDouble(objID, variable, getAccel(typeID));
    //         case VAR_DECEL:
    //             return wrapper->wrapDouble(objID, variable, getDecel(typeID));
    //         case VAR_EMERGENCY_DECEL:
    //             return wrapper->wrapDouble(objID, variable, getEmergencyDecel(typeID));
    //         case VAR_APPARENT_DECEL:
    //             return wrapper->wrapDouble(objID, variable, getApparentDecel(typeID));
    //         case VAR_ACTIONSTEPLENGTH:
    //             return wrapper->wrapDouble(objID, variable, getActionStepLength(typeID));
    //         case VAR_IMPERFECTION:
    //             return wrapper->wrapDouble(objID, variable, getImperfection(typeID));
    //         case VAR_TAU:
    //             return wrapper->wrapDouble(objID, variable, getTau(typeID));
    //         case VAR_SPEED_FACTOR:
    //             return wrapper->wrapDouble(objID, variable, getSpeedFactor(typeID));
    //         case VAR_SPEED_DEVIATION:
    //             return wrapper->wrapDouble(objID, variable, getSpeedDeviation(typeID));
    //         case VAR_VEHICLECLASS:
    //             return wrapper->wrapString(objID, variable, getVehicleClass(typeID));
    //         case VAR_EMISSIONCLASS:
    //             return wrapper->wrapString(objID, variable, getEmissionClass(typeID));
    //         case VAR_SHAPECLASS:
    //             return wrapper->wrapString(objID, variable, getShapeClass(typeID));
    //         case VAR_WIDTH:
    //             return wrapper->wrapDouble(objID, variable, getWidth(typeID));
    //         case VAR_COLOR:
    //             return wrapper->wrapColor(objID, variable, getColor(typeID));
    //         case VAR_MINGAP_LAT:
    //             return wrapper->wrapDouble(objID, variable, getMinGapLat(typeID));
    //         case VAR_MAXSPEED_LAT:
    //             return wrapper->wrapDouble(objID, variable, getMaxSpeedLat(typeID));
    //         case VAR_LATALIGNMENT:
    //             return wrapper->wrapString(objID, variable, getLateralAlignment(typeID));
    //         case VAR_PERSON_CAPACITY:
    //             return wrapper->wrapInt(objID, variable, getPersonCapacity(typeID));
    //         case VAR_BOARDING_DURATION:
    //             return wrapper->wrapDouble(objID, variable, getBoardingDuration(typeID));
    //         case VAR_IMPATIENCE:
    //             return wrapper->wrapDouble(objID, variable, getImpatience(typeID));
    //         case VAR_SCALE:
    //             return wrapper->wrapDouble(objID, variable, getScale(typeID));
    //         case libsumo::VAR_PARAMETER:
    //             paramData->readUnsignedByte();
    //             return wrapper->wrapString(objID, variable, getParameter(objID, paramData->readString()));
    //         case libsumo::VAR_PARAMETER_WITH_KEY:
    //             paramData->readUnsignedByte();
    //             return wrapper->wrapStringPair(objID, variable, getParameterWithKey(objID, paramData->readString()));
    //         default:
    //             return false;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn handle_variable_with_id(objID: &str, typeID: &str, variable: i32, wrapper: Ptr<VariableWrapper>, paramData: Ptr<Storage>) -> bool {
        match variable {
            TRACI_ID_LIST => wrapper.m().wrap_string_list(objID, variable, &Self::get_id_list()),
            ID_COUNT => wrapper.m().wrap_int(objID, variable, Self::get_id_count()),
            VAR_LENGTH => wrapper.m().wrap_double(objID, variable, Self::get_length(typeID)),
            VAR_HEIGHT => wrapper.m().wrap_double(objID, variable, Self::get_height(typeID)),
            VAR_MASS => wrapper.m().wrap_double(objID, variable, Self::get_mass(typeID)),
            VAR_MINGAP => wrapper.m().wrap_double(objID, variable, Self::get_min_gap(typeID)),
            VAR_MAXSPEED => wrapper.m().wrap_double(objID, variable, Self::get_max_speed(typeID)),
            VAR_ACCEL => wrapper.m().wrap_double(objID, variable, Self::get_accel(typeID)),
            VAR_DECEL => wrapper.m().wrap_double(objID, variable, Self::get_decel(typeID)),
            VAR_EMERGENCY_DECEL => wrapper.m().wrap_double(objID, variable, Self::get_emergency_decel(typeID)),
            VAR_APPARENT_DECEL => wrapper.m().wrap_double(objID, variable, Self::get_apparent_decel(typeID)),
            VAR_ACTIONSTEPLENGTH => wrapper.m().wrap_double(objID, variable, Self::get_action_step_length(typeID)),
            VAR_IMPERFECTION => wrapper.m().wrap_double(objID, variable, Self::get_imperfection(typeID)),
            VAR_TAU => wrapper.m().wrap_double(objID, variable, Self::get_tau(typeID)),
            VAR_SPEED_FACTOR => wrapper.m().wrap_double(objID, variable, Self::get_speed_factor(typeID)),
            VAR_SPEED_DEVIATION => wrapper.m().wrap_double(objID, variable, Self::get_speed_deviation(typeID)),
            VAR_VEHICLECLASS => wrapper.m().wrap_string(objID, variable, &Self::get_vehicle_class(typeID)),
            VAR_EMISSIONCLASS => wrapper.m().wrap_string(objID, variable, &Self::get_emission_class(typeID)),
            VAR_SHAPECLASS => wrapper.m().wrap_string(objID, variable, &Self::get_shape_class(typeID)),
            VAR_WIDTH => wrapper.m().wrap_double(objID, variable, Self::get_width(typeID)),
            VAR_COLOR => wrapper.m().wrap_color(objID, variable, &Self::get_color(typeID)),
            VAR_MINGAP_LAT => wrapper.m().wrap_double(objID, variable, Self::get_min_gap_lat(typeID)),
            VAR_MAXSPEED_LAT => wrapper.m().wrap_double(objID, variable, Self::get_max_speed_lat(typeID)),
            VAR_LATALIGNMENT => wrapper.m().wrap_string(objID, variable, &Self::get_lateral_alignment(typeID)),
            VAR_PERSON_CAPACITY => wrapper.m().wrap_int(objID, variable, Self::get_person_capacity(typeID)),
            VAR_BOARDING_DURATION => wrapper.m().wrap_double(objID, variable, Self::get_boarding_duration(typeID)),
            VAR_IMPATIENCE => wrapper.m().wrap_double(objID, variable, Self::get_impatience(typeID)),
            VAR_SCALE => wrapper.m().wrap_double(objID, variable, Self::get_scale(typeID)),
            VAR_PARAMETER => {
                paramData.m().read_unsigned_byte().expect("shortcut: malformed TraCI parameter header");
                let key = paramData.m().read_string().expect("shortcut: malformed TraCI parameter string");
                wrapper.m().wrap_string(objID, variable, &Self::get_parameter(objID, &key))
            }
            VAR_PARAMETER_WITH_KEY => {
                paramData.m().read_unsigned_byte().expect("shortcut: malformed TraCI parameter header");
                let key = paramData.m().read_string().expect("shortcut: malformed TraCI parameter string");
                wrapper.m().wrap_string_pair(objID, variable, &Self::get_parameter_with_key(objID, &key))
            }
            _ => false,
        }
    }

    /// C++: `void VehicleType::VehicleType()` [private]
    /// invalidated standard constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new() -> Self {
        panic!("libsumo::VehicleType cannot be constructed: the standard constructor is invalidated in C++")
    }

}

