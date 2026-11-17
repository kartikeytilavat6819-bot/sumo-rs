//! Port of utils/common/SUMOVehicleClass.{h,cpp}
use super::msg_handler::{write_error, write_warning};
use super::std_defs::g_ignore_unknown_vclass;
use super::string_bijection::StringBijection;
use super::string_tokenizer::StringTokenizer;
use super::util_exceptions::ProcessError;
use crate::iodevices::output_device::OutputDevice;
use crate::xml::sumo_sax_attributes::SUMOSAXAttributes;
use crate::xml::sumo_xml_definitions::{SUMO_ATTR_ALLOW, SUMO_ATTR_DISALLOW, SUMO_ATTR_EXCEPTIONS, SUMO_ATTR_PREFER, SUMO_ATTR_VALUE, SUMO_ATTR_VCLASSES};
use crate::{write_errorf, write_warningf};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::{LazyLock, Mutex, MutexGuard};

pub type SVCPermissions = i64;
/// the C++ enum SUMOVehicleClass (an int64 bit set)
pub type SUMOVehicleClass = i64;
pub type SUMOEmissionClass = i32;

pub const SVC_IGNORING: SUMOVehicleClass = 0;
pub const SVC_PRIVATE: SUMOVehicleClass = 1;
pub const SVC_EMERGENCY: SUMOVehicleClass = 1 << 1;
pub const SVC_AUTHORITY: SUMOVehicleClass = 1 << 2;
pub const SVC_ARMY: SUMOVehicleClass = 1 << 3;
pub const SVC_VIP: SUMOVehicleClass = 1 << 4;
pub const SVC_PEDESTRIAN: SUMOVehicleClass = 1 << 5;
pub const SVC_PASSENGER: SUMOVehicleClass = 1 << 6;
pub const SVC_HOV: SUMOVehicleClass = 1 << 7;
pub const SVC_TAXI: SUMOVehicleClass = 1 << 8;
pub const SVC_BUS: SUMOVehicleClass = 1 << 9;
pub const SVC_COACH: SUMOVehicleClass = 1 << 10;
pub const SVC_DELIVERY: SUMOVehicleClass = 1 << 11;
pub const SVC_TRUCK: SUMOVehicleClass = 1 << 12;
pub const SVC_TRAILER: SUMOVehicleClass = 1 << 13;
pub const SVC_MOTORCYCLE: SUMOVehicleClass = 1 << 14;
pub const SVC_MOPED: SUMOVehicleClass = 1 << 15;
pub const SVC_BICYCLE: SUMOVehicleClass = 1 << 16;
pub const SVC_E_VEHICLE: SUMOVehicleClass = 1 << 17;
pub const SVC_TRAM: SUMOVehicleClass = 1 << 18;
pub const SVC_RAIL_URBAN: SUMOVehicleClass = 1 << 19;
pub const SVC_RAIL: SUMOVehicleClass = 1 << 20;
pub const SVC_RAIL_ELECTRIC: SUMOVehicleClass = 1 << 21;
pub const SVC_RAIL_FAST: SUMOVehicleClass = 1 << 22;
pub const SVC_SHIP: SUMOVehicleClass = 1 << 23;
pub const SVC_CONTAINER: SUMOVehicleClass = 1 << 24;
pub const SVC_CABLE_CAR: SUMOVehicleClass = 1 << 25;
pub const SVC_SUBWAY: SUMOVehicleClass = 1 << 26;
pub const SVC_AIRCRAFT: SUMOVehicleClass = 1 << 27;
pub const SVC_WHEELCHAIR: SUMOVehicleClass = 1 << 28;
pub const SVC_SCOOTER: SUMOVehicleClass = 1 << 29;
pub const SVC_DRONE: SUMOVehicleClass = 1 << 30;
pub const SVC_CUSTOM1: SUMOVehicleClass = 1 << 31;
pub const SVC_CUSTOM2: SUMOVehicleClass = 1 << 32;
pub const SVC_RAIL_CLASSES: SUMOVehicleClass =
    SVC_RAIL_ELECTRIC | SVC_RAIL_FAST | SVC_RAIL | SVC_RAIL_URBAN | SVC_TRAM | SVC_SUBWAY | SVC_CABLE_CAR;
pub const SVC_PUBLIC_CLASSES: SUMOVehicleClass = SVC_BUS | SVC_RAIL_CLASSES | SVC_CABLE_CAR | SVC_AIRCRAFT;
pub const SVC_NON_ROAD_RAIL: SUMOVehicleClass = SVC_SHIP | SVC_AIRCRAFT | SVC_DRONE | SVC_CONTAINER;
pub const SVC_NON_ROAD: SUMOVehicleClass = SVC_RAIL_CLASSES | SVC_NON_ROAD_RAIL;
pub const SVC_VULNERABLE: SUMOVehicleClass = SVC_PEDESTRIAN | SVC_WHEELCHAIR | SVC_BICYCLE | SVC_SCOOTER;
pub const SVC_ROAD_MOTOR_CLASSES: SUMOVehicleClass = SVC_PASSENGER
    | SVC_HOV
    | SVC_TAXI
    | SVC_BUS
    | SVC_COACH
    | SVC_DELIVERY
    | SVC_TRUCK
    | SVC_TRAILER
    | SVC_MOTORCYCLE
    | SVC_MOPED
    | SVC_E_VEHICLE;
pub const SVC_ROAD_CLASSES: SUMOVehicleClass = SVC_ROAD_MOTOR_CLASSES | SVC_VULNERABLE;

pub const SUMO_VEHICLE_CLASS_MAX: SUMOVehicleClass = SVC_CUSTOM2;
/// all relevant bits set to 1
pub const SVC_ALL: SVCPermissions = 2 * SUMO_VEHICLE_CLASS_MAX - 1;
pub const SVC_UNSPECIFIED: SVCPermissions = -1;
pub const EMISSION_CLASS_UNSPECIFIED: SUMOEmissionClass = -1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SUMOVehicleShape {
    Unknown,
    Pedestrian,
    Bicycle,
    Moped,
    Motorcycle,
    Passenger,
    PassengerSedan,
    PassengerHatchback,
    PassengerWagon,
    PassengerVan,
    Taxi,
    Delivery,
    Truck,
    TruckSemitrailer,
    Truck1Trailer,
    Bus,
    BusCoach,
    BusFlexible,
    BusTrolley,
    Rail,
    RailCar,
    RailCargo,
    EVehicle,
    Ant,
    Ship,
    Emergency,
    Firebrigade,
    Police,
    Rickshaw,
    Scooter,
    Aircraft,
}

pub static SUMO_VEHICLE_CLASS_STRINGS: LazyLock<StringBijection<SUMOVehicleClass>> = LazyLock::new(|| {
    StringBijection::new(
        &[
            ("ignoring", SVC_IGNORING),
            ("private", SVC_PRIVATE),
            ("public_emergency", SVC_EMERGENCY), // !!! deprecated
            ("emergency", SVC_EMERGENCY),
            ("public_authority", SVC_AUTHORITY), // !!! deprecated
            ("authority", SVC_AUTHORITY),
            ("public_army", SVC_ARMY), // !!! deprecated
            ("army", SVC_ARMY),
            ("vip", SVC_VIP),
            ("pedestrian", SVC_PEDESTRIAN),
            ("passenger", SVC_PASSENGER),
            ("hov", SVC_HOV),
            ("taxi", SVC_TAXI),
            ("public_transport", SVC_BUS), // !!! deprecated
            ("bus", SVC_BUS),
            ("coach", SVC_COACH),
            ("delivery", SVC_DELIVERY),
            ("transport", SVC_TRUCK), // !!! deprecated
            ("truck", SVC_TRUCK),
            ("trailer", SVC_TRAILER),
            ("motorcycle", SVC_MOTORCYCLE),
            ("moped", SVC_MOPED),
            ("bicycle", SVC_BICYCLE),
            ("evehicle", SVC_E_VEHICLE),
            ("lightrail", SVC_TRAM), // !!! deprecated
            ("tram", SVC_TRAM),
            ("cityrail", SVC_RAIL_URBAN), // !!! deprecated
            ("rail_urban", SVC_RAIL_URBAN),
            ("rail_slow", SVC_RAIL), // !!! deprecated
            ("rail", SVC_RAIL),
            ("rail_electric", SVC_RAIL_ELECTRIC),
            ("rail_fast", SVC_RAIL_FAST),
            ("ship", SVC_SHIP),
            ("container", SVC_CONTAINER),
            ("cable_car", SVC_CABLE_CAR),
            ("subway", SVC_SUBWAY),
            ("aircraft", SVC_AIRCRAFT),
            ("wheelchair", SVC_WHEELCHAIR),
            ("scooter", SVC_SCOOTER),
            ("drone", SVC_DRONE),
            ("custom1", SVC_CUSTOM1),
            ("custom2", SVC_CUSTOM2),
        ],
        false,
    )
    .unwrap()
});

/// count only non-deprecated classes
pub fn num_vclasses() -> usize {
    SUMO_VEHICLE_CLASS_STRINGS.size() - 8
}

pub static DEPRECATED_VEHICLE_CLASSES_SEEN: Mutex<BTreeSet<String>> = Mutex::new(BTreeSet::new());

pub static SUMO_VEHICLE_SHAPE_STRINGS: LazyLock<StringBijection<SUMOVehicleShape>> = LazyLock::new(|| {
    use SUMOVehicleShape::*;
    StringBijection::new(
        &[
            ("pedestrian", Pedestrian),
            ("bicycle", Bicycle),
            ("moped", Moped),
            ("motorcycle", Motorcycle),
            ("passenger", Passenger),
            ("passenger/sedan", PassengerSedan),
            ("passenger/hatchback", PassengerHatchback),
            ("passenger/wagon", PassengerWagon),
            ("passenger/van", PassengerVan),
            ("taxi", Taxi),
            ("delivery", Delivery),
            ("transport", Truck), // !!! deprecated
            ("truck", Truck),
            ("transport/semitrailer", TruckSemitrailer), // !!! deprecated
            ("truck/semitrailer", TruckSemitrailer),
            ("transport/trailer", Truck1Trailer), // !!! deprecated
            ("truck/trailer", Truck1Trailer),
            ("bus/city", Bus), // !!! deprecated
            ("bus", Bus),
            ("bus/overland", BusCoach), // !!! deprecated
            ("bus/coach", BusCoach),
            ("bus/flexible", BusFlexible),
            ("bus/trolley", BusTrolley),
            ("rail/slow", Rail), // !!! deprecated
            ("rail/fast", Rail), // !!! deprecated
            ("rail", Rail),
            ("rail/light", RailCar), // !!! deprecated
            ("rail/city", RailCar),  // !!! deprecated
            ("rail/railcar", RailCar),
            ("rail/cargo", RailCargo),
            ("evehicle", EVehicle),
            ("ant", Ant),
            ("ship", Ship),
            ("emergency", Emergency),
            ("firebrigade", Firebrigade),
            ("police", Police),
            ("rickshaw", Rickshaw),
            ("scooter", Scooter),
            ("aircraft", Aircraft),
            ("", Unknown),
        ],
        false,
    )
    .unwrap()
});

static VEHICLE_CLASS_NAMES_LIST_CACHED: Mutex<BTreeMap<SVCPermissions, Vec<String>>> = Mutex::new(BTreeMap::new());
static PARSE_VEHICLE_CLASSES_CACHED: Mutex<BTreeMap<String, SVCPermissions>> = Mutex::new(BTreeMap::new());
static GET_VEHICLE_CLASS_NAMES_CACHED: Mutex<BTreeMap<SVCPermissions, String>> = Mutex::new(BTreeMap::new());
const VEHICLE_CLASS_NAME_ALL: &str = "all";

pub const DEFAULT_VTYPE_ID: &str = "DEFAULT_VEHTYPE";
pub const DEFAULT_PEDTYPE_ID: &str = "DEFAULT_PEDTYPE";
pub const DEFAULT_BIKETYPE_ID: &str = "DEFAULT_BIKETYPE";
pub const DEFAULT_CONTAINERTYPE_ID: &str = "DEFAULT_CONTAINERTYPE";
pub const DEFAULT_TAXITYPE_ID: &str = "DEFAULT_TAXITYPE";
pub const DEFAULT_RAILTYPE_ID: &str = "DEFAULT_RAILTYPE";
pub const DEFAULT_VTYPES: [&str; 6] =
    [DEFAULT_VTYPE_ID, DEFAULT_PEDTYPE_ID, DEFAULT_BIKETYPE_ID, DEFAULT_CONTAINERTYPE_ID, DEFAULT_TAXITYPE_ID, DEFAULT_RAILTYPE_ID];
pub const DEFAULT_VEH_PROB: f64 = 1.;
pub const DEFAULT_VEH_MASS: f64 = 1500.;
pub const DEFAULT_VEH_WIDTH: f64 = 1.8;
pub const DEFAULT_VEH_HEIGHT: f64 = 1.5;
pub const DEFAULT_VEH_SHUT_OFF_STOP: f64 = 300.;
pub const DEFAULT_PEDESTRIAN_SPEED: f64 = 5. / 3.6;
pub const DEFAULT_BICYCLE_SPEED: f64 = 20. / 3.6;
pub const DEFAULT_CONTAINER_TRANSHIP_SPEED: f64 = 5. / 3.6;

fn lock<T>(m: &Mutex<T>) -> MutexGuard<'_, T> {
    m.lock().unwrap_or_else(|e| e.into_inner())
}

/// SUMOXMLDefinitions::isValidNetID
fn is_valid_net_id(value: &str) -> bool {
    !value.is_empty() && !value.contains([' ', '\t', '\n', '\r', '|', '\\', '\'', '"', ';', ',', '<', '>', '&']) && !value.starts_with(':')
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StopOffset {
    permissions: SVCPermissions,
    offset: f64,
}

impl Default for StopOffset {
    fn default() -> Self {
        StopOffset { permissions: SVC_ALL, offset: 0. }
    }
}

impl StopOffset {
    /// C++ `StopOffset(const SUMOSAXAttributes& attrs, bool& ok)`
    pub fn from_attrs(attrs: &SUMOSAXAttributes, ok: &mut bool) -> Self {
        let mut s = StopOffset { permissions: SVC_IGNORING, offset: 0. };
        if attrs.has_attribute(SUMO_ATTR_VCLASSES) && attrs.has_attribute(SUMO_ATTR_EXCEPTIONS) {
            write_error("Simultaneous specification of vClasses and exceptions is not allowed");
            *ok = false;
        }
        if !attrs.has_attribute(SUMO_ATTR_VALUE) {
            write_error("StopOffset requires an offset value");
            *ok = false;
        }
        let v_classes = attrs.get_opt::<String>(SUMO_ATTR_VCLASSES, "", ok, String::new(), true);
        let exceptions = attrs.get_opt::<String>(SUMO_ATTR_EXCEPTIONS, "", ok, String::new(), true);
        if attrs.has_attribute(SUMO_ATTR_VCLASSES) {
            s.permissions = parse_vehicle_classes(&v_classes);
        } else if attrs.has_attribute(SUMO_ATTR_EXCEPTIONS) {
            s.permissions = !parse_vehicle_classes(&exceptions);
        } else {
            // no vClasses specified, thus apply to all
            s.permissions = parse_vehicle_classes("all");
        }
        s.offset = attrs.get_opt::<f64>(SUMO_ATTR_VALUE, "", ok, 0., true);
        s
    }
    pub fn is_defined(&self) -> bool {
        self.offset != 0.
    }
    pub fn reset(&mut self) {
        self.permissions = SVC_IGNORING;
        self.offset = 0.;
    }
    pub fn get_permissions(&self) -> SVCPermissions {
        self.permissions
    }
    pub fn get_exceptions(&self) -> String {
        get_vehicle_class_names(!self.permissions, false)
    }
    pub fn get_offset(&self) -> f64 {
        self.offset
    }
    pub fn set_permissions(&mut self, permissions: SVCPermissions) {
        self.permissions = permissions;
    }
    pub fn set_exceptions(&mut self, permissions: &str) {
        self.permissions = !parse_vehicle_classes(permissions);
    }
    pub fn set_offset(&mut self, offset: f64) {
        self.offset = offset;
    }
}

pub fn get_vehicle_class_names(permissions: SVCPermissions, expand: bool) -> String {
    if (permissions & SVC_ALL) == SVC_ALL && !expand {
        return VEHICLE_CLASS_NAME_ALL.to_string();
    }
    if let Some(s) = lock(&GET_VEHICLE_CLASS_NAMES_CACHED).get(&permissions) {
        return s.clone();
    }
    let s = get_vehicle_class_names_list(permissions).join(" ");
    lock(&GET_VEHICLE_CLASS_NAMES_CACHED).insert(permissions, s.clone());
    s
}

pub fn get_vehicle_class_names_list(permissions: SVCPermissions) -> Vec<String> {
    lock(&VEHICLE_CLASS_NAMES_LIST_CACHED)
        .entry(permissions)
        .or_insert_with(|| {
            SUMO_VEHICLE_CLASS_STRINGS
                .get_strings()
                .into_iter()
                .filter(|name| {
                    let svc = SUMO_VEHICLE_CLASS_STRINGS.get(name).unwrap();
                    (svc & permissions) == svc && svc != SVC_IGNORING
                })
                .collect()
        })
        .clone()
}

/// C++: `void writePreferences(OutputDevice& into, SVCPermissions preferred)`
pub fn write_preferences(into: &mut OutputDevice, preferred: SVCPermissions) {
    into.write_optional_attr(SUMO_ATTR_PREFER, get_vehicle_class_names(preferred, false).as_str(), preferred == SVC_ALL || preferred == 0);
}

/// C++: `void writePermissions(OutputDevice& into, SVCPermissions permissions)`
pub fn write_permissions(into: &mut OutputDevice, permissions: SVCPermissions) {
    if permissions == SVC_ALL || permissions == SVC_UNSPECIFIED {
        into.write_optional_attr(SUMO_ATTR_ALLOW, "", true);
        into.write_optional_attr(SUMO_ATTR_DISALLOW, "", true);
        return;
    }
    if permissions == 0 {
        into.write_optional_attr(SUMO_ATTR_ALLOW, "", true);
        into.write_attr(SUMO_ATTR_DISALLOW, VEHICLE_CLASS_NAME_ALL);
        return;
    }
    let mut num_allowed: i64 = 0;
    let mut mask: SVCPermissions = 1;
    while mask <= SUMO_VEHICLE_CLASS_MAX {
        if (mask & permissions) == mask {
            num_allowed += 1;
        }
        mask <<= 1;
    }
    let num_vc = num_vclasses() as i64;
    if num_allowed <= (num_vc - num_allowed) && num_allowed > 0 {
        into.write_attr(SUMO_ATTR_ALLOW, get_vehicle_class_names(permissions, false).as_str());
        into.write_optional_attr(SUMO_ATTR_DISALLOW, "", true);
    } else {
        into.write_optional_attr(SUMO_ATTR_ALLOW, "", true);
        into.write_attr(SUMO_ATTR_DISALLOW, get_vehicle_class_names(!permissions, false).as_str());
    }
}

pub fn get_vehicle_class_id(name: &str) -> Result<SUMOVehicleClass, ProcessError> {
    if SUMO_VEHICLE_CLASS_STRINGS.has_string(name) {
        return SUMO_VEHICLE_CLASS_STRINGS.get(name);
    }
    Err(ProcessError::InvalidArgument(format!("Unknown vehicle class '{name}'.")))
}

pub fn get_vehicle_class_compound_id(name: &str) -> SVCPermissions {
    let mut ret = SVC_IGNORING;
    for it in SUMO_VEHICLE_CLASS_STRINGS.get_strings() {
        if name.contains(it.as_str()) {
            ret |= SUMO_VEHICLE_CLASS_STRINGS.get(&it).unwrap();
        }
    }
    ret
}

pub fn parse_vehicle_classes(allowed_s: &str) -> SVCPermissions {
    if allowed_s == VEHICLE_CLASS_NAME_ALL {
        return SVC_ALL;
    }
    if let Some(&r) = lock(&PARSE_VEHICLE_CLASSES_CACHED).get(allowed_s) {
        return r;
    }
    let mut result = 0;
    let mut sta = StringTokenizer::with_token(allowed_s, " ", false);
    while sta.has_next() {
        let s = sta.next().unwrap();
        if !SUMO_VEHICLE_CLASS_STRINGS.has_string(&s) {
            if !is_valid_net_id(&s) {
                write_errorf!("Invalid character in vehicle class '%' encountered.", s);
            } else if g_ignore_unknown_vclass() {
                write_warningf!("Unknown vehicle class '%' ignored.", s);
            } else {
                write_errorf!("Unknown vehicle class '%' encountered.", s);
            }
        } else {
            let vc = get_vehicle_class_id(&s).unwrap();
            if SUMO_VEHICLE_CLASS_STRINGS.get_string(vc).unwrap() != s {
                lock(&DEPRECATED_VEHICLE_CLASSES_SEEN).insert(s);
            }
            result |= vc;
        }
    }
    lock(&PARSE_VEHICLE_CLASSES_CACHED).insert(allowed_s.to_string(), result);
    result
}

pub fn clear_parse_vehicle_classes_cache() {
    lock(&PARSE_VEHICLE_CLASSES_CACHED).clear();
}

pub fn can_parse_vehicle_classes(classes: &str) -> bool {
    if classes == VEHICLE_CLASS_NAME_ALL {
        return true;
    }
    if lock(&PARSE_VEHICLE_CLASSES_CACHED).contains_key(classes) {
        return true;
    }
    StringTokenizer::with_token(classes, " ", false).get_vector().iter().all(|s| SUMO_VEHICLE_CLASS_STRINGS.has_string(s))
}

/// parseVehicleClasses(allowedS, disallowedS, networkVersion = NETWORK_VERSION)
pub fn parse_vehicle_classes_allow_disallow(allowed_s: &str, disallowed_s: &str, network_version: (i32, f64)) -> SVCPermissions {
    if allowed_s.is_empty() && disallowed_s.is_empty() {
        SVC_ALL
    } else if !allowed_s.is_empty() && !disallowed_s.is_empty() {
        write_warning("SVCPermissions must be specified either via 'allow' or 'disallow'. Ignoring 'disallow'");
        parse_vehicle_classes(allowed_s)
    } else if !allowed_s.is_empty() {
        parse_vehicle_classes(allowed_s)
    } else {
        invert_permissions(extra_disallowed(parse_vehicle_classes(disallowed_s), network_version))
    }
}

pub fn extra_disallowed(mut disallowed: SVCPermissions, network_version: (i32, f64)) -> SVCPermissions {
    if network_version < (1, 3.) {
        disallowed |= SVC_RAIL_FAST;
    }
    if network_version < (1, 20.) && (disallowed & SVC_RAIL_URBAN) != 0 {
        disallowed |= SVC_SUBWAY;
        disallowed |= SVC_CABLE_CAR;
    }
    disallowed
}

pub fn invert_permissions(permissions: SVCPermissions) -> SVCPermissions {
    SVC_ALL & !permissions
}

/// parseVehicleClasses(const std::vector<std::string>&)
pub fn parse_vehicle_classes_vec(allowed_s: &[String]) -> Result<SVCPermissions, ProcessError> {
    if allowed_s.iter().any(|s| s == VEHICLE_CLASS_NAME_ALL) {
        return Ok(SVC_ALL);
    }
    let mut result = 0;
    for i in allowed_s {
        let vc = get_vehicle_class_id(i)?;
        let real_name = SUMO_VEHICLE_CLASS_STRINGS.get_string(vc)?;
        if real_name != i {
            write_warningf!("The vehicle class '%' is deprecated, use '%' instead.", i, real_name);
        }
        result |= vc;
    }
    Ok(result)
}

pub fn get_vehicle_shape_id(name: &str) -> Result<SUMOVehicleShape, ProcessError> {
    if SUMO_VEHICLE_SHAPE_STRINGS.has_string(name) {
        SUMO_VEHICLE_SHAPE_STRINGS.get(name)
    } else {
        Err(ProcessError::InvalidArgument(format!("Unknown vehicle shape '{name}'.")))
    }
}

pub fn can_parse_vehicle_shape(shape: &str) -> bool {
    SUMO_VEHICLE_SHAPE_STRINGS.has_string(shape)
}

pub fn get_vehicle_shape_name(id: SUMOVehicleShape) -> String {
    SUMO_VEHICLE_SHAPE_STRINGS.get_string(id).unwrap().to_string()
}

pub fn is_railway(permissions: SVCPermissions) -> bool {
    (permissions & SVC_RAIL_CLASSES) != 0 && (permissions & !(SVC_RAIL_CLASSES | SVC_TAXI)) == 0
}

pub fn is_railway_or_shared(permissions: SVCPermissions) -> bool {
    // basically check that it isn't SVC_ALL
    (permissions & SVC_RAIL_CLASSES) != 0 && (permissions & SVC_NON_ROAD_RAIL) == 0
}

pub fn is_tram(permissions: SVCPermissions) -> bool {
    (permissions & SVC_RAIL_CLASSES) == SVC_TRAM && (permissions & SVC_PASSENGER) == 0
}

pub fn is_bikepath(permissions: SVCPermissions) -> bool {
    (permissions & SVC_BICYCLE) == SVC_BICYCLE && (permissions & SVC_ROAD_MOTOR_CLASSES) == 0
}

pub fn is_waterway(permissions: SVCPermissions) -> bool {
    permissions == SVC_SHIP
}

pub fn is_airway(permissions: SVCPermissions) -> bool {
    permissions == SVC_AIRCRAFT || permissions == SVC_DRONE
}

pub fn is_forbidden(permissions: SVCPermissions) -> bool {
    (permissions & SVC_ALL) == 0
}

pub fn is_sidewalk(permissions: SVCPermissions) -> bool {
    (permissions & SVC_ALL) == SVC_PEDESTRIAN
}

pub fn is_for_vulnerable_modes(permissions: SVCPermissions) -> bool {
    (permissions & SVC_VULNERABLE) != 0 && (permissions & !SVC_VULNERABLE) == 0
}

pub fn no_vehicles(permissions: SVCPermissions) -> bool {
    is_forbidden(permissions) || is_sidewalk(permissions)
}

/// getDefaultVehicleLength(vc = SVC_IGNORING)
pub fn get_default_vehicle_length(vc: SUMOVehicleClass) -> f64 {
    match vc {
        SVC_PEDESTRIAN => 0.215,
        SVC_WHEELCHAIR => 1.2,
        SVC_BICYCLE => 1.6,
        SVC_SCOOTER => 1.2,
        SVC_MOPED => 2.1,
        SVC_MOTORCYCLE => 2.2,
        SVC_TRUCK => 7.1,
        SVC_TRAILER => 16.5,
        SVC_BUS => 12.,
        SVC_COACH => 14.,
        SVC_TRAM => 22.,
        SVC_RAIL_URBAN | SVC_SUBWAY => 36.5 * 3.,
        SVC_RAIL => 67.5 * 2.,
        SVC_RAIL_ELECTRIC | SVC_RAIL_FAST => 25. * 8.,
        SVC_DELIVERY | SVC_EMERGENCY => 6.5,
        SVC_SHIP => 17.,
        SVC_CONTAINER => 6.096,
        SVC_DRONE => 0.5,
        SVC_AIRCRAFT => 72.7,
        _ => 5., /*4.3*/
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names() {
        assert_eq!(num_vclasses(), 34);
        assert_eq!(SVC_ALL, (1i64 << 33) - 1);
        assert_eq!(get_vehicle_class_names(SVC_ALL, false), "all");
        assert_eq!(get_vehicle_class_names(SVC_PASSENGER | SVC_BUS, false), "passenger bus");
        assert_eq!(get_vehicle_class_names(SVC_PRIVATE, false), "private");
        assert_eq!(get_vehicle_class_id("transport").unwrap(), SVC_TRUCK);
        assert_eq!(get_vehicle_class_id("x").unwrap_err().to_string(), "Unknown vehicle class 'x'.");
        assert_eq!(get_vehicle_shape_name(SUMOVehicleShape::Truck), "truck");
        assert_eq!(get_vehicle_shape_id("rail/city").unwrap(), SUMOVehicleShape::RailCar);
    }

    #[test]
    fn parsing() {
        assert_eq!(parse_vehicle_classes("passenger  bicycle"), SVC_PASSENGER | SVC_BICYCLE);
        assert_eq!(parse_vehicle_classes("all"), SVC_ALL);
        parse_vehicle_classes("lightrail");
        assert!(lock(&DEPRECATED_VEHICLE_CLASSES_SEEN).contains("lightrail"));
        assert!(can_parse_vehicle_classes("tram bus") && !can_parse_vehicle_classes("tram foo"));
        assert_eq!(parse_vehicle_classes_allow_disallow("", "pedestrian", (1, 20.)), SVC_ALL & !SVC_PEDESTRIAN);
        assert_eq!(
            parse_vehicle_classes_allow_disallow("", "rail_urban", (1, 0.)),
            SVC_ALL & !(SVC_RAIL_URBAN | SVC_RAIL_FAST | SVC_SUBWAY | SVC_CABLE_CAR)
        );
        assert_eq!(parse_vehicle_classes_vec(&["bus".into(), "tram".into()]).unwrap(), SVC_BUS | SVC_TRAM);
        assert!(is_railway(SVC_TRAM | SVC_RAIL) && !is_railway(SVC_ALL));
        assert!(is_sidewalk(SVC_PEDESTRIAN) && no_vehicles(0));
        assert_eq!(get_default_vehicle_length(SVC_RAIL), 135.);
    }

    #[test]
    fn stop_offset_from_attributes() {
        use crate::xml::sumo_xml_definitions::Attrs;
        use std::sync::Arc;
        let attrs = |a: &[(&str, &str)]| {
            SUMOSAXAttributes::new(a.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect(), Arc::new(Attrs.get_strings()), "stop")
        };
        let mut ok = true;
        let s = StopOffset::from_attrs(&attrs(&[("value", "2.5"), ("vClasses", "passenger")]), &mut ok);
        assert!(ok && s.is_defined());
        assert_eq!(s.get_offset(), 2.5);
        assert_eq!(s.get_permissions(), SVC_PASSENGER);
        let mut ok = true;
        let e = StopOffset::from_attrs(&attrs(&[("value", "1"), ("exceptions", "bus")]), &mut ok);
        assert!(ok);
        assert_eq!(e.get_permissions(), !SVC_BUS);
        let mut ok = true;
        let d = StopOffset::from_attrs(&attrs(&[("value", "1")]), &mut ok);
        assert_eq!(d.get_permissions(), SVC_ALL);
        let mut ok = true;
        StopOffset::from_attrs(&attrs(&[("vClasses", "bus")]), &mut ok);
        assert!(!ok, "a missing value makes the stop offset invalid");
    }
}
