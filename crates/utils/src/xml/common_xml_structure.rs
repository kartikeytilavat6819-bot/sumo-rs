//! Port of utils/xml/CommonXMLStructure.{h,cpp}
// shortcut: SUMOVehicleParameter, SUMOVehicleParameter::Stop and SUMOVTypeParameter are not ported (no utils::vehicle module yet); they are stored as opaque `Box<dyn Any>`, and parameters that C++ would store into them are kept in `my_typed_parameters_pending`, upgrade when utils::vehicle is ported
#![allow(non_camel_case_types)]

use std::any::Any;
use std::collections::BTreeMap;

use crate::common::ptr::Ptr;
use crate::common::rgb_color::RGBColor;
use crate::common::string_tokenizer::StringTokenizer;
use crate::common::sumo_vehicle_class::{SUMOVehicleClass, SVC_IGNORING};
use crate::common::std_defs::g_precision;
use crate::common::time::{time2string, SUMOTime};
use crate::common::to_string::to_string;
use crate::common::util_exceptions::ProcessError;
use crate::geom::position::Position;
use crate::geom::position_vector::PositionVector;
use crate::xml::sumo_sax_attributes::SUMOSAXAttributes;
use crate::xml::sumo_xml_definitions::*;

// ===========================================================================
// helpers
// ===========================================================================

/// toString(SumoXMLTag)
fn tag_name(tag: SumoXMLTag) -> String {
    Tags.get_string(tag).to_string()
}

/// toString(SumoXMLAttr)
fn attr_name(attr: SumoXMLAttr) -> String {
    Attrs.get_string(attr).to_string()
}

// ===========================================================================
// class definitions
// ===========================================================================

/// C++ CommonXMLStructure::PlanParameters (plan parameters, used for group all from-to parameters related with plans)
#[derive(Debug, Default, Clone)]
pub struct CommonXMLStructure_PlanParameters {
    /// @brief from edge
    pub from_edge: String,
    /// @brief to edge
    pub to_edge: String,
    /// @brief consecutive edges
    pub consecutive_edges: Vec<String>,
    /// @brief from junction
    pub from_junction: String,
    /// @brief to junction
    pub to_junction: String,
    /// @brief from TAZ
    pub from_taz: String,
    /// @brief to TAZ
    pub to_taz: String,
    /// @brief from busStop
    pub from_bus_stop: String,
    /// @brief to busStop
    pub to_bus_stop: String,
    /// @brief from trainStop
    pub from_train_stop: String,
    /// @brief to trainStop
    pub to_train_stop: String,
    /// @brief from containerStop
    pub from_container_stop: String,
    /// @brief to containerStop
    pub to_container_stop: String,
    /// @brief from chargingStation
    pub from_charging_station: String,
    /// @brief to chargingStation
    pub to_charging_station: String,
    /// @brief from parkingArea
    pub from_parking_area: String,
    /// @brief to parkingArea
    pub to_parking_area: String,
    /// @brief from route
    pub from_route: String,
    /// @brief to route
    pub to_route: String,
}

/// C++ CommonXMLStructure::SumoBaseObject (renamed). Children are owned (`Box`, like C++ `new`/`delete`),
/// the parent is a non-owning back-pointer (`Ptr`, null for the root).
pub struct CommonXMLStructure_SumoBaseObject {
    /// @brief pointer to SumoBaseObject parent (If is null, then is the root)
    my_sumo_base_object_parent: Ptr<CommonXMLStructure_SumoBaseObject>,
    /// @brief XML myTag
    my_tag: SumoXMLTag,
    /// @brief flag to check if object was created in build<..> function (by default false)
    my_was_created: bool,
    /// @brief string attributes
    my_string_attributes: BTreeMap<SumoXMLAttr, String>,
    /// @brief int attributes
    my_int_attributes: BTreeMap<SumoXMLAttr, i32>,
    /// @brief double attributes
    my_double_attributes: BTreeMap<SumoXMLAttr, f64>,
    /// @brief bool attributes
    my_bool_attributes: BTreeMap<SumoXMLAttr, bool>,
    /// @brief Position attributes
    my_position_attributes: BTreeMap<SumoXMLAttr, Position>,
    /// @brief SUMOTime attributes
    my_time_attributes: BTreeMap<SumoXMLAttr, SUMOTime>,
    /// @brief RGBColor attributes
    my_color_attributes: BTreeMap<SumoXMLAttr, RGBColor>,
    /// @brief stringList attributes
    my_string_list_attributes: BTreeMap<SumoXMLAttr, Vec<String>>,
    /// @brief doubleList attributes
    my_double_list_attributes: BTreeMap<SumoXMLAttr, Vec<f64>>,
    /// @brief PositionVector attributes
    my_position_vector_attributes: BTreeMap<SumoXMLAttr, PositionVector>,
    /// @brief myParameters
    my_parameters: BTreeMap<String, String>,
    /// @brief parent IDs
    my_parent_ids: BTreeMap<SumoXMLTag, String>,
    /// @brief SumoBaseObject children (owned)
    my_sumo_base_object_children: Vec<Box<CommonXMLStructure_SumoBaseObject>>,
    /// @brief vehicle class
    my_vclass: SUMOVehicleClass,
    /// @brief vehicle type parameter (SUMOVTypeParameter, opaque; None = not defined)
    my_vehicle_type_parameter: Option<Box<dyn Any>>,
    /// @brief vehicle parameter (SUMOVehicleParameter, opaque; None = not defined)
    my_vehicle_parameter: Option<Box<dyn Any>>,
    /// @brief stop parameter (SUMOVehicleParameter::Stop, opaque; None = not defined)
    my_stop_parameter: Option<Box<dyn Any>>,
    /// @brief parameters that C++ stores into the defined vType/vehicle/stop parameter (see shortcut above)
    my_typed_parameters_pending: BTreeMap<String, String>,
    /// @brief plan parameters
    my_plan_parameters: CommonXMLStructure_PlanParameters,
}

/// C++ CommonXMLStructure
#[derive(Default)]
pub struct CommonXMLStructure {
    /// @brief SumoBaseObject root
    my_sumo_base_object_root: Ptr<CommonXMLStructure_SumoBaseObject>,
    /// @brief last inserted SumoBaseObject
    my_current_sumo_base_object: Ptr<CommonXMLStructure_SumoBaseObject>,
    /// @brief owner of the root object (C++ `new`/`delete` of mySumoBaseObjectRoot)
    my_root_owner: Option<Box<CommonXMLStructure_SumoBaseObject>>,
}

// ===========================================================================
// method definitions
// ===========================================================================

// ---------------------------------------------------------------------------
// CommonXMLStructure::PlanParameters - methods
// ---------------------------------------------------------------------------

impl CommonXMLStructure_PlanParameters {
    /// C++ PlanParameters(const SumoBaseObject*, const SUMOSAXAttributes&, bool&): parses the from-to attributes
    pub fn from_attributes(sumo_base_object: Ptr<CommonXMLStructure_SumoBaseObject>, attrs: &SUMOSAXAttributes, parsed_ok: &mut bool) -> Result<Self, ProcessError> {
        let mut pp = Self::default();
        if !sumo_base_object.get_parent_sumo_base_object().is_null() {
            // get plan parent ID (first check if exist!)
            let parent = sumo_base_object.get_parent_sumo_base_object();
            let plan_parent_id: String = if parent.has_string_attribute(SUMO_ATTR_ID) {
                parent.get_string_attribute(SUMO_ATTR_ID)?.clone()
            } else {
                String::new()
            };
            let id = plan_parent_id.as_str();
            // edges
            pp.from_edge = attrs.get_opt::<String>(SUMO_ATTR_FROM, id, parsed_ok, String::new(), true);
            pp.to_edge = attrs.get_opt::<String>(SUMO_ATTR_TO, id, parsed_ok, String::new(), true);
            if pp.to_edge.is_empty() {
                pp.to_edge = attrs.get_opt::<String>(SUMO_ATTR_EDGE, id, parsed_ok, String::new(), true);
            }
            pp.consecutive_edges = attrs.get_opt::<Vec<String>>(SUMO_ATTR_EDGES, id, parsed_ok, Vec::new(), true);
            // junctions
            pp.from_junction = attrs.get_opt::<String>(SUMO_ATTR_FROM_JUNCTION, id, parsed_ok, String::new(), true);
            pp.to_junction = attrs.get_opt::<String>(SUMO_ATTR_TO_JUNCTION, id, parsed_ok, String::new(), true);
            // TAZs
            pp.from_taz = attrs.get_opt::<String>(SUMO_ATTR_FROM_TAZ, id, parsed_ok, String::new(), true);
            pp.to_taz = attrs.get_opt::<String>(SUMO_ATTR_TO_TAZ, id, parsed_ok, String::new(), true);
            // bus stops
            pp.from_bus_stop = attrs.get_opt::<String>(GNE_ATTR_FROM_BUSSTOP, id, parsed_ok, String::new(), true);
            pp.to_bus_stop = attrs.get_opt::<String>(SUMO_ATTR_BUS_STOP, id, parsed_ok, String::new(), true);
            // train stops
            pp.from_train_stop = attrs.get_opt::<String>(GNE_ATTR_FROM_TRAINSTOP, id, parsed_ok, String::new(), true);
            pp.to_train_stop = attrs.get_opt::<String>(SUMO_ATTR_TRAIN_STOP, id, parsed_ok, String::new(), true);
            // container stops
            pp.from_container_stop = attrs.get_opt::<String>(GNE_ATTR_FROM_CONTAINERSTOP, id, parsed_ok, String::new(), true);
            pp.to_container_stop = attrs.get_opt::<String>(SUMO_ATTR_CONTAINER_STOP, id, parsed_ok, String::new(), true);
            // charging stations
            pp.from_charging_station = attrs.get_opt::<String>(GNE_ATTR_FROM_CHARGINGSTATION, id, parsed_ok, String::new(), true);
            pp.to_charging_station = attrs.get_opt::<String>(SUMO_ATTR_CHARGING_STATION, id, parsed_ok, String::new(), true);
            // parking areas
            pp.from_parking_area = attrs.get_opt::<String>(GNE_ATTR_FROM_PARKINGAREA, id, parsed_ok, String::new(), true);
            pp.to_parking_area = attrs.get_opt::<String>(SUMO_ATTR_PARKING_AREA, id, parsed_ok, String::new(), true);
            // routes
            pp.from_route = attrs.get_opt::<String>(GNE_ATTR_FROM_ROUTE, id, parsed_ok, String::new(), true);
            pp.to_route = attrs.get_opt::<String>(SUMO_ATTR_ROUTE, id, parsed_ok, String::new(), true);
            // update from attributes
            pp.update_from_attributes(sumo_base_object)?;
        }
        Ok(pp)
    }

    /// clear parameters
    pub fn clear(&mut self) {
        self.from_junction.clear();
        self.to_junction.clear();
        self.from_edge.clear();
        self.to_edge.clear();
        self.from_taz.clear();
        self.to_taz.clear();
        self.from_bus_stop.clear();
        self.to_bus_stop.clear();
        self.from_train_stop.clear();
        self.to_train_stop.clear();
        self.from_container_stop.clear();
        self.to_container_stop.clear();
        self.from_charging_station.clear();
        self.to_charging_station.clear();
        self.from_parking_area.clear();
        self.to_parking_area.clear();
        self.consecutive_edges.clear();
        self.from_route.clear();
        self.to_route.clear();
    }

    /// check if this is a single-edge plan
    pub fn is_single_edge_plan(&self) -> bool {
        if self.from_edge.is_empty() {
            false
        } else {
            self.get_number_of_defined_parameters() == 1
        }
    }

    /// get number of defined plans
    pub fn get_number_of_defined_parameters(&self) -> i32 {
        let defined = |s: &String| i32::from(!s.is_empty());
        self.consecutive_edges.len() as i32
            + defined(&self.from_junction)
            + defined(&self.to_junction)
            + defined(&self.from_edge)
            + defined(&self.to_edge)
            + defined(&self.from_taz)
            + defined(&self.to_taz)
            + defined(&self.from_bus_stop)
            + defined(&self.to_bus_stop)
            + defined(&self.from_train_stop)
            + defined(&self.to_train_stop)
            + defined(&self.from_container_stop)
            + defined(&self.to_container_stop)
            + defined(&self.from_charging_station)
            + defined(&self.to_charging_station)
            + defined(&self.from_parking_area)
            + defined(&self.to_parking_area)
            + defined(&self.from_route)
            + defined(&self.to_route)
    }

    pub fn get_walk_tag(&self) -> SumoXMLTag {
        if self.is_single_edge_plan() {
            return GNE_TAG_WALK_EDGE_EDGE;
        } else if !self.consecutive_edges.is_empty() {
            return GNE_TAG_WALK_EDGES;
        } else if !self.to_route.is_empty() {
            return GNE_TAG_WALK_ROUTE;
        } else if !self.from_edge.is_empty() {
            if !self.to_edge.is_empty() {
                return GNE_TAG_WALK_EDGE_EDGE;
            } else if !self.to_taz.is_empty() {
                return GNE_TAG_WALK_EDGE_TAZ;
            } else if !self.to_junction.is_empty() {
                return GNE_TAG_WALK_EDGE_JUNCTION;
            } else if !self.to_bus_stop.is_empty() {
                return GNE_TAG_WALK_EDGE_BUSSTOP;
            } else if !self.to_train_stop.is_empty() {
                return GNE_TAG_WALK_EDGE_TRAINSTOP;
            } else if !self.to_container_stop.is_empty() {
                return GNE_TAG_WALK_EDGE_CONTAINERSTOP;
            } else if !self.to_charging_station.is_empty() {
                return GNE_TAG_WALK_EDGE_CHARGINGSTATION;
            } else if !self.to_parking_area.is_empty() {
                return GNE_TAG_WALK_EDGE_PARKINGAREA;
            } else {
                return SUMO_TAG_NOTHING;
            }
        } else if !self.from_taz.is_empty() {
            if !self.to_edge.is_empty() {
                return GNE_TAG_WALK_TAZ_EDGE;
            } else if !self.to_taz.is_empty() {
                return GNE_TAG_WALK_TAZ_TAZ;
            } else if !self.to_junction.is_empty() {
                return GNE_TAG_WALK_TAZ_JUNCTION;
            } else if !self.to_bus_stop.is_empty() {
                return GNE_TAG_WALK_TAZ_BUSSTOP;
            } else if !self.to_train_stop.is_empty() {
                return GNE_TAG_WALK_TAZ_TRAINSTOP;
            } else if !self.to_container_stop.is_empty() {
                return GNE_TAG_WALK_TAZ_CONTAINERSTOP;
            } else if !self.to_charging_station.is_empty() {
                return GNE_TAG_WALK_TAZ_CHARGINGSTATION;
            } else if !self.to_parking_area.is_empty() {
                return GNE_TAG_WALK_TAZ_PARKINGAREA;
            } else {
                return SUMO_TAG_NOTHING;
            }
        } else if !self.from_junction.is_empty() {
            if !self.to_edge.is_empty() {
                return GNE_TAG_WALK_JUNCTION_EDGE;
            } else if !self.to_taz.is_empty() {
                return GNE_TAG_WALK_JUNCTION_TAZ;
            } else if !self.to_junction.is_empty() {
                return GNE_TAG_WALK_JUNCTION_JUNCTION;
            } else if !self.to_bus_stop.is_empty() {
                return GNE_TAG_WALK_JUNCTION_BUSSTOP;
            } else if !self.to_train_stop.is_empty() {
                return GNE_TAG_WALK_JUNCTION_TRAINSTOP;
            } else if !self.to_container_stop.is_empty() {
                return GNE_TAG_WALK_JUNCTION_CONTAINERSTOP;
            } else if !self.to_charging_station.is_empty() {
                return GNE_TAG_WALK_JUNCTION_CHARGINGSTATION;
            } else if !self.to_parking_area.is_empty() {
                return GNE_TAG_WALK_JUNCTION_PARKINGAREA;
            } else {
                return SUMO_TAG_NOTHING;
            }
        } else if !self.from_bus_stop.is_empty() {
            if !self.to_edge.is_empty() {
                return GNE_TAG_WALK_BUSSTOP_EDGE;
            } else if !self.to_taz.is_empty() {
                return GNE_TAG_WALK_BUSSTOP_TAZ;
            } else if !self.to_junction.is_empty() {
                return GNE_TAG_WALK_BUSSTOP_JUNCTION;
            } else if !self.to_bus_stop.is_empty() {
                return GNE_TAG_WALK_BUSSTOP_BUSSTOP;
            } else if !self.to_train_stop.is_empty() {
                return GNE_TAG_WALK_BUSSTOP_TRAINSTOP;
            } else if !self.to_container_stop.is_empty() {
                return GNE_TAG_WALK_BUSSTOP_CONTAINERSTOP;
            } else if !self.to_charging_station.is_empty() {
                return GNE_TAG_WALK_BUSSTOP_CHARGINGSTATION;
            } else if !self.to_parking_area.is_empty() {
                return GNE_TAG_WALK_BUSSTOP_PARKINGAREA;
            } else {
                return SUMO_TAG_NOTHING;
            }
        } else if !self.from_train_stop.is_empty() {
            if !self.to_edge.is_empty() {
                return GNE_TAG_WALK_TRAINSTOP_EDGE;
            } else if !self.to_taz.is_empty() {
                return GNE_TAG_WALK_TRAINSTOP_TAZ;
            } else if !self.to_junction.is_empty() {
                return GNE_TAG_WALK_TRAINSTOP_JUNCTION;
            } else if !self.to_bus_stop.is_empty() {
                return GNE_TAG_WALK_TRAINSTOP_BUSSTOP;
            } else if !self.to_train_stop.is_empty() {
                return GNE_TAG_WALK_TRAINSTOP_TRAINSTOP;
            } else if !self.to_container_stop.is_empty() {
                return GNE_TAG_WALK_TRAINSTOP_CONTAINERSTOP;
            } else if !self.to_charging_station.is_empty() {
                return GNE_TAG_WALK_TRAINSTOP_CHARGINGSTATION;
            } else if !self.to_parking_area.is_empty() {
                return GNE_TAG_WALK_TRAINSTOP_PARKINGAREA;
            } else {
                return SUMO_TAG_NOTHING;
            }
        } else if !self.from_container_stop.is_empty() {
            if !self.to_edge.is_empty() {
                return GNE_TAG_WALK_CONTAINERSTOP_EDGE;
            } else if !self.to_taz.is_empty() {
                return GNE_TAG_WALK_CONTAINERSTOP_TAZ;
            } else if !self.to_junction.is_empty() {
                return GNE_TAG_WALK_CONTAINERSTOP_JUNCTION;
            } else if !self.to_bus_stop.is_empty() {
                return GNE_TAG_WALK_CONTAINERSTOP_BUSSTOP;
            } else if !self.to_train_stop.is_empty() {
                return GNE_TAG_WALK_CONTAINERSTOP_TRAINSTOP;
            } else if !self.to_container_stop.is_empty() {
                return GNE_TAG_WALK_CONTAINERSTOP_CONTAINERSTOP;
            } else if !self.to_charging_station.is_empty() {
                return GNE_TAG_WALK_CONTAINERSTOP_CHARGINGSTATION;
            } else if !self.to_parking_area.is_empty() {
                return GNE_TAG_WALK_CONTAINERSTOP_PARKINGAREA;
            } else {
                return SUMO_TAG_NOTHING;
            }
        } else if !self.from_charging_station.is_empty() {
            if !self.to_edge.is_empty() {
                return GNE_TAG_WALK_CHARGINGSTATION_EDGE;
            } else if !self.to_taz.is_empty() {
                return GNE_TAG_WALK_CHARGINGSTATION_TAZ;
            } else if !self.to_junction.is_empty() {
                return GNE_TAG_WALK_CHARGINGSTATION_JUNCTION;
            } else if !self.to_bus_stop.is_empty() {
                return GNE_TAG_WALK_CHARGINGSTATION_BUSSTOP;
            } else if !self.to_train_stop.is_empty() {
                return GNE_TAG_WALK_CHARGINGSTATION_TRAINSTOP;
            } else if !self.to_container_stop.is_empty() {
                return GNE_TAG_WALK_CHARGINGSTATION_CONTAINERSTOP;
            } else if !self.to_charging_station.is_empty() {
                return GNE_TAG_WALK_CHARGINGSTATION_CHARGINGSTATION;
            } else if !self.to_parking_area.is_empty() {
                return GNE_TAG_WALK_CHARGINGSTATION_PARKINGAREA;
            } else {
                return SUMO_TAG_NOTHING;
            }
        } else if !self.from_parking_area.is_empty() {
            if !self.to_edge.is_empty() {
                return GNE_TAG_WALK_PARKINGAREA_EDGE;
            } else if !self.to_taz.is_empty() {
                return GNE_TAG_WALK_PARKINGAREA_TAZ;
            } else if !self.to_junction.is_empty() {
                return GNE_TAG_WALK_PARKINGAREA_JUNCTION;
            } else if !self.to_bus_stop.is_empty() {
                return GNE_TAG_WALK_PARKINGAREA_BUSSTOP;
            } else if !self.to_train_stop.is_empty() {
                return GNE_TAG_WALK_PARKINGAREA_TRAINSTOP;
            } else if !self.to_container_stop.is_empty() {
                return GNE_TAG_WALK_PARKINGAREA_CONTAINERSTOP;
            } else if !self.to_charging_station.is_empty() {
                return GNE_TAG_WALK_PARKINGAREA_CHARGINGSTATION;
            } else if !self.to_parking_area.is_empty() {
                return GNE_TAG_WALK_PARKINGAREA_PARKINGAREA;
            } else {
                return SUMO_TAG_NOTHING;
            }
        } else {
            return SUMO_TAG_NOTHING;
        }
    }

    pub fn get_person_trip_tag(&self) -> SumoXMLTag {
        if self.is_single_edge_plan() {
            return GNE_TAG_PERSONTRIP_EDGE_EDGE;
        } else if !self.from_edge.is_empty() {
            if !self.to_edge.is_empty() {
                return GNE_TAG_PERSONTRIP_EDGE_EDGE;
            } else if !self.to_taz.is_empty() {
                return GNE_TAG_PERSONTRIP_EDGE_TAZ;
            } else if !self.to_junction.is_empty() {
                return GNE_TAG_PERSONTRIP_EDGE_JUNCTION;
            } else if !self.to_bus_stop.is_empty() {
                return GNE_TAG_PERSONTRIP_EDGE_BUSSTOP;
            } else if !self.to_train_stop.is_empty() {
                return GNE_TAG_PERSONTRIP_EDGE_TRAINSTOP;
            } else if !self.to_container_stop.is_empty() {
                return GNE_TAG_PERSONTRIP_EDGE_CONTAINERSTOP;
            } else if !self.to_charging_station.is_empty() {
                return GNE_TAG_PERSONTRIP_EDGE_CHARGINGSTATION;
            } else if !self.to_parking_area.is_empty() {
                return GNE_TAG_PERSONTRIP_EDGE_PARKINGAREA;
            } else {
                return SUMO_TAG_NOTHING;
            }
        } else if !self.from_taz.is_empty() {
            if !self.to_edge.is_empty() {
                return GNE_TAG_PERSONTRIP_TAZ_EDGE;
            } else if !self.to_taz.is_empty() {
                return GNE_TAG_PERSONTRIP_TAZ_TAZ;
            } else if !self.to_junction.is_empty() {
                return GNE_TAG_PERSONTRIP_TAZ_JUNCTION;
            } else if !self.to_bus_stop.is_empty() {
                return GNE_TAG_PERSONTRIP_TAZ_BUSSTOP;
            } else if !self.to_train_stop.is_empty() {
                return GNE_TAG_PERSONTRIP_TAZ_TRAINSTOP;
            } else if !self.to_container_stop.is_empty() {
                return GNE_TAG_PERSONTRIP_TAZ_CONTAINERSTOP;
            } else if !self.to_charging_station.is_empty() {
                return GNE_TAG_PERSONTRIP_TAZ_CHARGINGSTATION;
            } else if !self.to_parking_area.is_empty() {
                return GNE_TAG_PERSONTRIP_TAZ_PARKINGAREA;
            } else {
                return SUMO_TAG_NOTHING;
            }
        } else if !self.from_junction.is_empty() {
            if !self.to_edge.is_empty() {
                return GNE_TAG_PERSONTRIP_JUNCTION_EDGE;
            } else if !self.to_taz.is_empty() {
                return GNE_TAG_PERSONTRIP_JUNCTION_TAZ;
            } else if !self.to_junction.is_empty() {
                return GNE_TAG_PERSONTRIP_JUNCTION_JUNCTION;
            } else if !self.to_bus_stop.is_empty() {
                return GNE_TAG_PERSONTRIP_JUNCTION_BUSSTOP;
            } else if !self.to_train_stop.is_empty() {
                return GNE_TAG_PERSONTRIP_JUNCTION_TRAINSTOP;
            } else if !self.to_container_stop.is_empty() {
                return GNE_TAG_PERSONTRIP_JUNCTION_CONTAINERSTOP;
            } else if !self.to_charging_station.is_empty() {
                return GNE_TAG_PERSONTRIP_JUNCTION_CHARGINGSTATION;
            } else if !self.to_parking_area.is_empty() {
                return GNE_TAG_PERSONTRIP_JUNCTION_PARKINGAREA;
            } else {
                return SUMO_TAG_NOTHING;
            }
        } else if !self.from_bus_stop.is_empty() {
            if !self.to_edge.is_empty() {
                return GNE_TAG_PERSONTRIP_BUSSTOP_EDGE;
            } else if !self.to_taz.is_empty() {
                return GNE_TAG_PERSONTRIP_BUSSTOP_TAZ;
            } else if !self.to_junction.is_empty() {
                return GNE_TAG_PERSONTRIP_BUSSTOP_JUNCTION;
            } else if !self.to_bus_stop.is_empty() {
                return GNE_TAG_PERSONTRIP_BUSSTOP_BUSSTOP;
            } else if !self.to_train_stop.is_empty() {
                return GNE_TAG_PERSONTRIP_BUSSTOP_TRAINSTOP;
            } else if !self.to_container_stop.is_empty() {
                return GNE_TAG_PERSONTRIP_BUSSTOP_CONTAINERSTOP;
            } else if !self.to_charging_station.is_empty() {
                return GNE_TAG_PERSONTRIP_BUSSTOP_CHARGINGSTATION;
            } else if !self.to_parking_area.is_empty() {
                return GNE_TAG_PERSONTRIP_BUSSTOP_PARKINGAREA;
            } else {
                return SUMO_TAG_NOTHING;
            }
        } else if !self.from_train_stop.is_empty() {
            if !self.to_edge.is_empty() {
                return GNE_TAG_PERSONTRIP_TRAINSTOP_EDGE;
            } else if !self.to_taz.is_empty() {
                return GNE_TAG_PERSONTRIP_TRAINSTOP_TAZ;
            } else if !self.to_junction.is_empty() {
                return GNE_TAG_PERSONTRIP_TRAINSTOP_JUNCTION;
            } else if !self.to_bus_stop.is_empty() {
                return GNE_TAG_PERSONTRIP_TRAINSTOP_BUSSTOP;
            } else if !self.to_train_stop.is_empty() {
                return GNE_TAG_PERSONTRIP_TRAINSTOP_TRAINSTOP;
            } else if !self.to_container_stop.is_empty() {
                return GNE_TAG_PERSONTRIP_TRAINSTOP_CONTAINERSTOP;
            } else if !self.to_charging_station.is_empty() {
                return GNE_TAG_PERSONTRIP_TRAINSTOP_CHARGINGSTATION;
            } else if !self.to_parking_area.is_empty() {
                return GNE_TAG_PERSONTRIP_TRAINSTOP_PARKINGAREA;
            } else {
                return SUMO_TAG_NOTHING;
            }
        } else if !self.from_container_stop.is_empty() {
            if !self.to_edge.is_empty() {
                return GNE_TAG_PERSONTRIP_CONTAINERSTOP_EDGE;
            } else if !self.to_taz.is_empty() {
                return GNE_TAG_PERSONTRIP_CONTAINERSTOP_TAZ;
            } else if !self.to_junction.is_empty() {
                return GNE_TAG_PERSONTRIP_CONTAINERSTOP_JUNCTION;
            } else if !self.to_bus_stop.is_empty() {
                return GNE_TAG_PERSONTRIP_CONTAINERSTOP_BUSSTOP;
            } else if !self.to_train_stop.is_empty() {
                return GNE_TAG_PERSONTRIP_CONTAINERSTOP_TRAINSTOP;
            } else if !self.to_container_stop.is_empty() {
                return GNE_TAG_PERSONTRIP_CONTAINERSTOP_CONTAINERSTOP;
            } else if !self.to_charging_station.is_empty() {
                return GNE_TAG_PERSONTRIP_CONTAINERSTOP_CHARGINGSTATION;
            } else if !self.to_parking_area.is_empty() {
                return GNE_TAG_PERSONTRIP_CONTAINERSTOP_PARKINGAREA;
            } else {
                return SUMO_TAG_NOTHING;
            }
        } else if !self.from_charging_station.is_empty() {
            if !self.to_edge.is_empty() {
                return GNE_TAG_PERSONTRIP_CHARGINGSTATION_EDGE;
            } else if !self.to_taz.is_empty() {
                return GNE_TAG_PERSONTRIP_CHARGINGSTATION_TAZ;
            } else if !self.to_junction.is_empty() {
                return GNE_TAG_PERSONTRIP_CHARGINGSTATION_JUNCTION;
            } else if !self.to_bus_stop.is_empty() {
                return GNE_TAG_PERSONTRIP_CHARGINGSTATION_BUSSTOP;
            } else if !self.to_train_stop.is_empty() {
                return GNE_TAG_PERSONTRIP_CHARGINGSTATION_TRAINSTOP;
            } else if !self.to_container_stop.is_empty() {
                return GNE_TAG_PERSONTRIP_CHARGINGSTATION_CONTAINERSTOP;
            } else if !self.to_charging_station.is_empty() {
                return GNE_TAG_PERSONTRIP_CHARGINGSTATION_CHARGINGSTATION;
            } else if !self.to_parking_area.is_empty() {
                return GNE_TAG_PERSONTRIP_CHARGINGSTATION_PARKINGAREA;
            } else {
                return SUMO_TAG_NOTHING;
            }
        } else if !self.from_parking_area.is_empty() {
            if !self.to_edge.is_empty() {
                return GNE_TAG_PERSONTRIP_PARKINGAREA_EDGE;
            } else if !self.to_taz.is_empty() {
                return GNE_TAG_PERSONTRIP_PARKINGAREA_TAZ;
            } else if !self.to_junction.is_empty() {
                return GNE_TAG_PERSONTRIP_PARKINGAREA_JUNCTION;
            } else if !self.to_bus_stop.is_empty() {
                return GNE_TAG_PERSONTRIP_PARKINGAREA_BUSSTOP;
            } else if !self.to_train_stop.is_empty() {
                return GNE_TAG_PERSONTRIP_PARKINGAREA_TRAINSTOP;
            } else if !self.to_container_stop.is_empty() {
                return GNE_TAG_PERSONTRIP_PARKINGAREA_CONTAINERSTOP;
            } else if !self.to_charging_station.is_empty() {
                return GNE_TAG_PERSONTRIP_PARKINGAREA_CHARGINGSTATION;
            } else if !self.to_parking_area.is_empty() {
                return GNE_TAG_PERSONTRIP_PARKINGAREA_PARKINGAREA;
            } else {
                return SUMO_TAG_NOTHING;
            }
        } else {
            return SUMO_TAG_NOTHING;
        }
    }

    pub fn get_ride_tag(&self) -> SumoXMLTag {
        if self.is_single_edge_plan() {
            return GNE_TAG_RIDE_EDGE_EDGE;
        } else if !self.from_edge.is_empty() {
            if !self.to_edge.is_empty() {
                return GNE_TAG_RIDE_EDGE_EDGE;
            } else if !self.to_bus_stop.is_empty() {
                return GNE_TAG_RIDE_EDGE_BUSSTOP;
            } else if !self.to_train_stop.is_empty() {
                return GNE_TAG_RIDE_EDGE_TRAINSTOP;
            } else if !self.to_container_stop.is_empty() {
                return GNE_TAG_RIDE_EDGE_CONTAINERSTOP;
            } else if !self.to_charging_station.is_empty() {
                return GNE_TAG_RIDE_EDGE_CHARGINGSTATION;
            } else if !self.to_parking_area.is_empty() {
                return GNE_TAG_RIDE_EDGE_PARKINGAREA;
            } else {
                return SUMO_TAG_NOTHING;
            }
        } else if !self.from_bus_stop.is_empty() {
            if !self.to_edge.is_empty() {
                return GNE_TAG_RIDE_BUSSTOP_EDGE;
            } else if !self.to_bus_stop.is_empty() {
                return GNE_TAG_RIDE_BUSSTOP_BUSSTOP;
            } else if !self.to_train_stop.is_empty() {
                return GNE_TAG_RIDE_BUSSTOP_TRAINSTOP;
            } else if !self.to_container_stop.is_empty() {
                return GNE_TAG_RIDE_BUSSTOP_CONTAINERSTOP;
            } else if !self.to_charging_station.is_empty() {
                return GNE_TAG_RIDE_BUSSTOP_CHARGINGSTATION;
            } else if !self.to_parking_area.is_empty() {
                return GNE_TAG_RIDE_BUSSTOP_PARKINGAREA;
            } else {
                return SUMO_TAG_NOTHING;
            }
        } else if !self.from_train_stop.is_empty() {
            if !self.to_edge.is_empty() {
                return GNE_TAG_RIDE_TRAINSTOP_EDGE;
            } else if !self.to_bus_stop.is_empty() {
                return GNE_TAG_RIDE_TRAINSTOP_BUSSTOP;
            } else if !self.to_train_stop.is_empty() {
                return GNE_TAG_RIDE_TRAINSTOP_TRAINSTOP;
            } else if !self.to_container_stop.is_empty() {
                return GNE_TAG_RIDE_TRAINSTOP_CONTAINERSTOP;
            } else if !self.to_charging_station.is_empty() {
                return GNE_TAG_RIDE_TRAINSTOP_CHARGINGSTATION;
            } else if !self.to_parking_area.is_empty() {
                return GNE_TAG_RIDE_TRAINSTOP_PARKINGAREA;
            } else {
                return SUMO_TAG_NOTHING;
            }
        } else if !self.from_container_stop.is_empty() {
            if !self.to_edge.is_empty() {
                return GNE_TAG_RIDE_CONTAINERSTOP_EDGE;
            } else if !self.to_bus_stop.is_empty() {
                return GNE_TAG_RIDE_CONTAINERSTOP_BUSSTOP;
            } else if !self.to_train_stop.is_empty() {
                return GNE_TAG_RIDE_CONTAINERSTOP_TRAINSTOP;
            } else if !self.to_container_stop.is_empty() {
                return GNE_TAG_RIDE_CONTAINERSTOP_CONTAINERSTOP;
            } else if !self.to_charging_station.is_empty() {
                return GNE_TAG_RIDE_CONTAINERSTOP_CHARGINGSTATION;
            } else if !self.to_parking_area.is_empty() {
                return GNE_TAG_RIDE_CONTAINERSTOP_PARKINGAREA;
            } else {
                return SUMO_TAG_NOTHING;
            }
        } else if !self.from_charging_station.is_empty() {
            if !self.to_edge.is_empty() {
                return GNE_TAG_RIDE_CHARGINGSTATION_EDGE;
            } else if !self.to_bus_stop.is_empty() {
                return GNE_TAG_RIDE_CHARGINGSTATION_BUSSTOP;
            } else if !self.to_train_stop.is_empty() {
                return GNE_TAG_RIDE_CHARGINGSTATION_TRAINSTOP;
            } else if !self.to_container_stop.is_empty() {
                return GNE_TAG_RIDE_CHARGINGSTATION_CONTAINERSTOP;
            } else if !self.to_charging_station.is_empty() {
                return GNE_TAG_RIDE_CHARGINGSTATION_CHARGINGSTATION;
            } else if !self.to_parking_area.is_empty() {
                return GNE_TAG_RIDE_CHARGINGSTATION_PARKINGAREA;
            } else {
                return SUMO_TAG_NOTHING;
            }
        } else if !self.from_parking_area.is_empty() {
            if !self.to_edge.is_empty() {
                return GNE_TAG_RIDE_PARKINGAREA_EDGE;
            } else if !self.to_bus_stop.is_empty() {
                return GNE_TAG_RIDE_PARKINGAREA_BUSSTOP;
            } else if !self.to_train_stop.is_empty() {
                return GNE_TAG_RIDE_PARKINGAREA_TRAINSTOP;
            } else if !self.to_container_stop.is_empty() {
                return GNE_TAG_RIDE_PARKINGAREA_CONTAINERSTOP;
            } else if !self.to_charging_station.is_empty() {
                return GNE_TAG_RIDE_PARKINGAREA_CHARGINGSTATION;
            } else if !self.to_parking_area.is_empty() {
                return GNE_TAG_RIDE_PARKINGAREA_PARKINGAREA;
            } else {
                return SUMO_TAG_NOTHING;
            }
        } else {
            return SUMO_TAG_NOTHING;
        }
    }

    pub fn get_transport_tag(&self) -> SumoXMLTag {
        if self.is_single_edge_plan() {
            return GNE_TAG_TRANSPORT_EDGE_EDGE;
        } else if !self.from_edge.is_empty() {
            if !self.to_edge.is_empty() {
                return GNE_TAG_TRANSPORT_EDGE_EDGE;
            } else if !self.to_bus_stop.is_empty() {
                return GNE_TAG_TRANSPORT_EDGE_BUSSTOP;
            } else if !self.to_train_stop.is_empty() {
                return GNE_TAG_TRANSPORT_EDGE_TRAINSTOP;
            } else if !self.to_container_stop.is_empty() {
                return GNE_TAG_TRANSPORT_EDGE_CONTAINERSTOP;
            } else if !self.to_charging_station.is_empty() {
                return GNE_TAG_TRANSPORT_EDGE_CHARGINGSTATION;
            } else if !self.to_parking_area.is_empty() {
                return GNE_TAG_TRANSPORT_EDGE_PARKINGAREA;
            } else {
                return SUMO_TAG_NOTHING;
            }
        } else if !self.from_bus_stop.is_empty() {
            if !self.to_edge.is_empty() {
                return GNE_TAG_TRANSPORT_BUSSTOP_EDGE;
            } else if !self.to_bus_stop.is_empty() {
                return GNE_TAG_TRANSPORT_BUSSTOP_BUSSTOP;
            } else if !self.to_train_stop.is_empty() {
                return GNE_TAG_TRANSPORT_BUSSTOP_TRAINSTOP;
            } else if !self.to_container_stop.is_empty() {
                return GNE_TAG_TRANSPORT_BUSSTOP_CONTAINERSTOP;
            } else if !self.to_charging_station.is_empty() {
                return GNE_TAG_TRANSPORT_BUSSTOP_CHARGINGSTATION;
            } else if !self.to_parking_area.is_empty() {
                return GNE_TAG_TRANSPORT_BUSSTOP_PARKINGAREA;
            } else {
                return SUMO_TAG_NOTHING;
            }
        } else if !self.from_train_stop.is_empty() {
            if !self.to_edge.is_empty() {
                return GNE_TAG_TRANSPORT_TRAINSTOP_EDGE;
            } else if !self.to_bus_stop.is_empty() {
                return GNE_TAG_TRANSPORT_TRAINSTOP_BUSSTOP;
            } else if !self.to_train_stop.is_empty() {
                return GNE_TAG_TRANSPORT_TRAINSTOP_TRAINSTOP;
            } else if !self.to_container_stop.is_empty() {
                return GNE_TAG_TRANSPORT_TRAINSTOP_CONTAINERSTOP;
            } else if !self.to_charging_station.is_empty() {
                return GNE_TAG_TRANSPORT_TRAINSTOP_CHARGINGSTATION;
            } else if !self.to_parking_area.is_empty() {
                return GNE_TAG_TRANSPORT_TRAINSTOP_PARKINGAREA;
            } else {
                return SUMO_TAG_NOTHING;
            }
        } else if !self.from_container_stop.is_empty() {
            if !self.to_edge.is_empty() {
                return GNE_TAG_TRANSPORT_CONTAINERSTOP_EDGE;
            } else if !self.to_bus_stop.is_empty() {
                return GNE_TAG_TRANSPORT_CONTAINERSTOP_BUSSTOP;
            } else if !self.to_train_stop.is_empty() {
                return GNE_TAG_TRANSPORT_CONTAINERSTOP_TRAINSTOP;
            } else if !self.to_container_stop.is_empty() {
                return GNE_TAG_TRANSPORT_CONTAINERSTOP_CONTAINERSTOP;
            } else if !self.to_charging_station.is_empty() {
                return GNE_TAG_TRANSPORT_CONTAINERSTOP_CHARGINGSTATION;
            } else if !self.to_parking_area.is_empty() {
                return GNE_TAG_TRANSPORT_CONTAINERSTOP_PARKINGAREA;
            } else {
                return SUMO_TAG_NOTHING;
            }
        } else if !self.from_charging_station.is_empty() {
            if !self.to_edge.is_empty() {
                return GNE_TAG_TRANSPORT_CHARGINGSTATION_EDGE;
            } else if !self.to_bus_stop.is_empty() {
                return GNE_TAG_TRANSPORT_CHARGINGSTATION_BUSSTOP;
            } else if !self.to_train_stop.is_empty() {
                return GNE_TAG_TRANSPORT_CHARGINGSTATION_TRAINSTOP;
            } else if !self.to_container_stop.is_empty() {
                return GNE_TAG_TRANSPORT_CHARGINGSTATION_CONTAINERSTOP;
            } else if !self.to_charging_station.is_empty() {
                return GNE_TAG_TRANSPORT_CHARGINGSTATION_CHARGINGSTATION;
            } else if !self.to_parking_area.is_empty() {
                return GNE_TAG_TRANSPORT_CHARGINGSTATION_PARKINGAREA;
            } else {
                return SUMO_TAG_NOTHING;
            }
        } else if !self.from_parking_area.is_empty() {
            if !self.to_edge.is_empty() {
                return GNE_TAG_TRANSPORT_PARKINGAREA_EDGE;
            } else if !self.to_bus_stop.is_empty() {
                return GNE_TAG_TRANSPORT_PARKINGAREA_BUSSTOP;
            } else if !self.to_train_stop.is_empty() {
                return GNE_TAG_TRANSPORT_PARKINGAREA_TRAINSTOP;
            } else if !self.to_container_stop.is_empty() {
                return GNE_TAG_TRANSPORT_PARKINGAREA_CONTAINERSTOP;
            } else if !self.to_charging_station.is_empty() {
                return GNE_TAG_TRANSPORT_PARKINGAREA_CHARGINGSTATION;
            } else if !self.to_parking_area.is_empty() {
                return GNE_TAG_TRANSPORT_PARKINGAREA_PARKINGAREA;
            } else {
                return SUMO_TAG_NOTHING;
            }
        } else {
            return SUMO_TAG_NOTHING;
        }
    }

    pub fn get_tranship_tag(&self) -> SumoXMLTag {
        if self.is_single_edge_plan() {
            return GNE_TAG_TRANSHIP_EDGE_EDGE;
        } else if !self.consecutive_edges.is_empty() {
            return GNE_TAG_TRANSHIP_EDGES;
        } else if !self.from_edge.is_empty() {
            if !self.to_edge.is_empty() {
                return GNE_TAG_TRANSHIP_EDGE_EDGE;
            } else if !self.to_bus_stop.is_empty() {
                return GNE_TAG_TRANSHIP_EDGE_BUSSTOP;
            } else if !self.to_train_stop.is_empty() {
                return GNE_TAG_TRANSHIP_EDGE_TRAINSTOP;
            } else if !self.to_container_stop.is_empty() {
                return GNE_TAG_TRANSHIP_EDGE_CONTAINERSTOP;
            } else if !self.to_charging_station.is_empty() {
                return GNE_TAG_TRANSHIP_EDGE_CHARGINGSTATION;
            } else if !self.to_parking_area.is_empty() {
                return GNE_TAG_TRANSHIP_EDGE_PARKINGAREA;
            } else {
                return SUMO_TAG_NOTHING;
            }
        } else if !self.from_bus_stop.is_empty() {
            if !self.to_edge.is_empty() {
                return GNE_TAG_TRANSHIP_BUSSTOP_EDGE;
            } else if !self.to_bus_stop.is_empty() {
                return GNE_TAG_TRANSHIP_BUSSTOP_BUSSTOP;
            } else if !self.to_train_stop.is_empty() {
                return GNE_TAG_TRANSHIP_BUSSTOP_TRAINSTOP;
            } else if !self.to_container_stop.is_empty() {
                return GNE_TAG_TRANSHIP_BUSSTOP_CONTAINERSTOP;
            } else if !self.to_charging_station.is_empty() {
                return GNE_TAG_TRANSHIP_BUSSTOP_CHARGINGSTATION;
            } else if !self.to_parking_area.is_empty() {
                return GNE_TAG_TRANSHIP_BUSSTOP_PARKINGAREA;
            } else {
                return SUMO_TAG_NOTHING;
            }
        } else if !self.from_train_stop.is_empty() {
            if !self.to_edge.is_empty() {
                return GNE_TAG_TRANSHIP_TRAINSTOP_EDGE;
            } else if !self.to_bus_stop.is_empty() {
                return GNE_TAG_TRANSHIP_TRAINSTOP_BUSSTOP;
            } else if !self.to_train_stop.is_empty() {
                return GNE_TAG_TRANSHIP_TRAINSTOP_TRAINSTOP;
            } else if !self.to_container_stop.is_empty() {
                return GNE_TAG_TRANSHIP_TRAINSTOP_CONTAINERSTOP;
            } else if !self.to_charging_station.is_empty() {
                return GNE_TAG_TRANSHIP_TRAINSTOP_CHARGINGSTATION;
            } else if !self.to_parking_area.is_empty() {
                return GNE_TAG_TRANSHIP_TRAINSTOP_PARKINGAREA;
            } else {
                return SUMO_TAG_NOTHING;
            }
        } else if !self.from_container_stop.is_empty() {
            if !self.to_edge.is_empty() {
                return GNE_TAG_TRANSHIP_CONTAINERSTOP_EDGE;
            } else if !self.to_bus_stop.is_empty() {
                return GNE_TAG_TRANSHIP_CONTAINERSTOP_BUSSTOP;
            } else if !self.to_train_stop.is_empty() {
                return GNE_TAG_TRANSHIP_CONTAINERSTOP_TRAINSTOP;
            } else if !self.to_container_stop.is_empty() {
                return GNE_TAG_TRANSHIP_CONTAINERSTOP_CONTAINERSTOP;
            } else if !self.to_charging_station.is_empty() {
                return GNE_TAG_TRANSHIP_CONTAINERSTOP_CHARGINGSTATION;
            } else if !self.to_parking_area.is_empty() {
                return GNE_TAG_TRANSHIP_CONTAINERSTOP_PARKINGAREA;
            } else {
                return SUMO_TAG_NOTHING;
            }
        } else if !self.from_charging_station.is_empty() {
            if !self.to_edge.is_empty() {
                return GNE_TAG_TRANSHIP_CHARGINGSTATION_EDGE;
            } else if !self.to_bus_stop.is_empty() {
                return GNE_TAG_TRANSHIP_CHARGINGSTATION_BUSSTOP;
            } else if !self.to_train_stop.is_empty() {
                return GNE_TAG_TRANSHIP_CHARGINGSTATION_TRAINSTOP;
            } else if !self.to_container_stop.is_empty() {
                return GNE_TAG_TRANSHIP_CHARGINGSTATION_CONTAINERSTOP;
            } else if !self.to_charging_station.is_empty() {
                return GNE_TAG_TRANSHIP_CHARGINGSTATION_CHARGINGSTATION;
            } else if !self.to_parking_area.is_empty() {
                return GNE_TAG_TRANSHIP_CHARGINGSTATION_PARKINGAREA;
            } else {
                return SUMO_TAG_NOTHING;
            }
        } else if !self.from_parking_area.is_empty() {
            if !self.to_edge.is_empty() {
                return GNE_TAG_TRANSHIP_PARKINGAREA_EDGE;
            } else if !self.to_bus_stop.is_empty() {
                return GNE_TAG_TRANSHIP_PARKINGAREA_BUSSTOP;
            } else if !self.to_train_stop.is_empty() {
                return GNE_TAG_TRANSHIP_PARKINGAREA_TRAINSTOP;
            } else if !self.to_container_stop.is_empty() {
                return GNE_TAG_TRANSHIP_PARKINGAREA_CONTAINERSTOP;
            } else if !self.to_charging_station.is_empty() {
                return GNE_TAG_TRANSHIP_PARKINGAREA_CHARGINGSTATION;
            } else if !self.to_parking_area.is_empty() {
                return GNE_TAG_TRANSHIP_PARKINGAREA_PARKINGAREA;
            } else {
                return SUMO_TAG_NOTHING;
            }
        } else {
            return SUMO_TAG_NOTHING;
        }
    }

    pub fn get_person_stop_tag(&self) -> SumoXMLTag {
        if !self.to_edge.is_empty() {
            return GNE_TAG_STOPPERSON_EDGE;
        } else if !self.to_bus_stop.is_empty() {
            return GNE_TAG_STOPPERSON_BUSSTOP;
        } else if !self.to_train_stop.is_empty() {
            return GNE_TAG_STOPPERSON_TRAINSTOP;
        } else if !self.to_container_stop.is_empty() {
            return GNE_TAG_STOPPERSON_CONTAINERSTOP;
        } else if !self.to_charging_station.is_empty() {
            return GNE_TAG_STOPPERSON_CHARGINGSTATION;
        } else if !self.to_parking_area.is_empty() {
            return GNE_TAG_STOPPERSON_PARKINGAREA;
        } else {
            return SUMO_TAG_NOTHING;
        }
    }

    pub fn get_container_stop_tag(&self) -> SumoXMLTag {
        if !self.to_edge.is_empty() {
            return GNE_TAG_STOPCONTAINER_EDGE;
        } else if !self.to_bus_stop.is_empty() {
            return GNE_TAG_STOPCONTAINER_BUSSTOP;
        } else if !self.to_train_stop.is_empty() {
            return GNE_TAG_STOPCONTAINER_TRAINSTOP;
        } else if !self.to_container_stop.is_empty() {
            return GNE_TAG_STOPCONTAINER_CONTAINERSTOP;
        } else if !self.to_charging_station.is_empty() {
            return GNE_TAG_STOPCONTAINER_CHARGINGSTATION;
        } else if !self.to_parking_area.is_empty() {
            return GNE_TAG_STOPCONTAINER_PARKINGAREA;
        } else {
            return SUMO_TAG_NOTHING;
        }
    }

    /// get previous plan obj
    fn get_previous_plan_obj(&self, sumo_base_object: Ptr<CommonXMLStructure_SumoBaseObject>) -> Option<Ptr<CommonXMLStructure_SumoBaseObject>> {
        // first check if object exist
        if sumo_base_object.is_null() {
            return None;
        }
        // check if object has parent
        let parent_object = sumo_base_object.get_parent_sumo_base_object();
        if parent_object.is_null() {
            return None;
        }
        // check number of children
        let children = parent_object.get_sumo_base_object_children();
        if children.len() < 2 {
            return None;
        }
        // search position of the given plan obj in the parent children (std::find: end() if missing)
        let idx = children.iter().position(|c| Ptr::from_box(c) == sumo_base_object).unwrap_or(children.len());
        // if obj is the first plan of person/container parent, then return null. If not, return previous object
        if idx == 0 {
            None
        } else {
            Some(Ptr::from_box(&children[idx - 1]))
        }
    }

    /// update the from attributes
    fn update_from_attributes(&mut self, sumo_base_object: Ptr<CommonXMLStructure_SumoBaseObject>) -> Result<(), ProcessError> {
        // check if previous plan object was defined but not the from
        if let Some(previous_plan_obj) = self.get_previous_plan_obj(sumo_base_object) {
            // ge previous plan parameters
            let previous_plan_parameters = previous_plan_obj.get_plan_parameters().clone();
            if !previous_plan_parameters.to_edge.is_empty() {
                // edge (to)
                self.reset_previous_from_attributes(previous_plan_obj, "edge", &previous_plan_parameters.to_edge)?;
                self.from_edge = previous_plan_parameters.to_edge.clone();
            } else if !previous_plan_parameters.consecutive_edges.is_empty() {
                // consecutive edge
                let last = previous_plan_parameters.consecutive_edges.last().cloned().unwrap_or_default();
                self.reset_previous_from_attributes(previous_plan_obj, "consecutive edge", &last)?;
                self.from_edge = last;
            } else if !previous_plan_parameters.to_route.is_empty() {
                // route
                self.reset_previous_from_attributes(previous_plan_obj, "route edge", &previous_plan_parameters.to_route)?;
                self.from_route = previous_plan_parameters.to_route.clone();
            } else if !previous_plan_parameters.to_junction.is_empty() {
                // junction
                self.reset_previous_from_attributes(previous_plan_obj, "junction", &previous_plan_parameters.to_junction)?;
                self.from_junction = previous_plan_parameters.to_junction.clone();
            } else if !previous_plan_parameters.to_taz.is_empty() {
                // TAZ
                self.reset_previous_from_attributes(previous_plan_obj, "TAZ", &previous_plan_parameters.to_taz)?;
                self.from_taz = previous_plan_parameters.to_taz.clone();
            } else if !previous_plan_parameters.to_bus_stop.is_empty() {
                // busStop
                self.reset_previous_from_attributes(previous_plan_obj, "bus stop", &previous_plan_parameters.to_bus_stop)?;
                self.from_bus_stop = previous_plan_parameters.to_bus_stop.clone();
            } else if !previous_plan_parameters.to_train_stop.is_empty() {
                // trainStop
                self.reset_previous_from_attributes(previous_plan_obj, "train stop", &previous_plan_parameters.to_train_stop)?;
                self.from_train_stop = previous_plan_parameters.to_train_stop.clone();
            } else if !previous_plan_parameters.to_container_stop.is_empty() {
                // containerStop
                self.reset_previous_from_attributes(previous_plan_obj, "container stop", &previous_plan_parameters.to_container_stop)?;
                self.from_container_stop = previous_plan_parameters.to_container_stop.clone();
            } else if !previous_plan_parameters.to_charging_station.is_empty() {
                // chargingStation
                self.reset_previous_from_attributes(previous_plan_obj, "charging station", &previous_plan_parameters.to_charging_station)?;
                self.from_charging_station = previous_plan_parameters.to_charging_station.clone();
            } else if !previous_plan_parameters.to_parking_area.is_empty() {
                // parkingArea
                self.reset_previous_from_attributes(previous_plan_obj, "parking area", &previous_plan_parameters.to_parking_area)?;
                self.from_parking_area = previous_plan_parameters.to_parking_area.clone();
            }
        }
        Ok(())
    }

    /// reste all previous from attributes
    fn reset_previous_from_attributes(&self, previous_plan_obj: Ptr<CommonXMLStructure_SumoBaseObject>, new_type: &str, new_id: &str) -> Result<(), ProcessError> {
        if !self.from_edge.is_empty() {
            self.write_ignoring_message(previous_plan_obj, "edge", &self.from_edge, new_type, new_id)?;
        }
        if !self.from_junction.is_empty() {
            self.write_ignoring_message(previous_plan_obj, "junction", &self.from_junction, new_type, new_id)?;
        }
        if !self.from_taz.is_empty() {
            self.write_ignoring_message(previous_plan_obj, "TAZ", &self.from_taz, new_type, new_id)?;
        }
        if !self.from_bus_stop.is_empty() {
            self.write_ignoring_message(previous_plan_obj, "bus stop", &self.from_bus_stop, new_type, new_id)?;
        }
        if !self.from_train_stop.is_empty() {
            self.write_ignoring_message(previous_plan_obj, "train stop", &self.from_train_stop, new_type, new_id)?;
        }
        if !self.from_container_stop.is_empty() {
            self.write_ignoring_message(previous_plan_obj, "container stop", &self.from_container_stop, new_type, new_id)?;
        }
        if !self.from_charging_station.is_empty() {
            self.write_ignoring_message(previous_plan_obj, "charging station", &self.from_charging_station, new_type, new_id)?;
        }
        if !self.from_parking_area.is_empty() {
            self.write_ignoring_message(previous_plan_obj, "parking area", &self.from_parking_area, new_type, new_id)?;
        }
        Ok(())
    }

    /// write ignoring message
    fn write_ignoring_message(&self, previous_plan_obj: Ptr<CommonXMLStructure_SumoBaseObject>, old_type: &str, old_id: &str, new_type: &str, new_id: &str) -> Result<(), ProcessError> {
        let parent = previous_plan_obj.get_parent_sumo_base_object();
        let msg = crate::tlf!(
            "Ignoring from % '%' used in % '%' and using instead the previous end element % '%'",
            old_type,
            old_id,
            tag_name(parent.get_tag()),
            parent.get_string_attribute(SUMO_ATTR_ID)?,
            new_type,
            new_id
        );
        crate::common::msg_handler::write_warning(&msg);
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// CommonXMLStructure::SumoBaseObject - methods
// ---------------------------------------------------------------------------

impl Drop for CommonXMLStructure_SumoBaseObject {
    /// C++ ~SumoBaseObject: deletes all children. Each child is detached from this (dying) parent first;
    /// the removal from the parent's child list is done by the owner (see remove_sumo_base_object_child).
    fn drop(&mut self) {
        for mut child in std::mem::take(&mut self.my_sumo_base_object_children) {
            child.my_sumo_base_object_parent = Ptr::null();
            drop(child);
        }
    }
}

impl CommonXMLStructure_SumoBaseObject {
    /// C++ SumoBaseObject(SumoBaseObject* parent). The C++ constructor also appends the new object to its
    /// parent's children; here the caller owns the returned box and registers it with
    /// `parent.add_sumo_base_object_child(box)`.
    pub fn new(sumo_base_object_parent: Ptr<CommonXMLStructure_SumoBaseObject>) -> Box<CommonXMLStructure_SumoBaseObject> {
        Box::new(CommonXMLStructure_SumoBaseObject {
            my_sumo_base_object_parent: sumo_base_object_parent,
            my_tag: SUMO_TAG_NOTHING,
            my_was_created: false,
            my_string_attributes: BTreeMap::new(),
            my_int_attributes: BTreeMap::new(),
            my_double_attributes: BTreeMap::new(),
            my_bool_attributes: BTreeMap::new(),
            my_position_attributes: BTreeMap::new(),
            my_time_attributes: BTreeMap::new(),
            my_color_attributes: BTreeMap::new(),
            my_string_list_attributes: BTreeMap::new(),
            my_double_list_attributes: BTreeMap::new(),
            my_position_vector_attributes: BTreeMap::new(),
            my_parameters: BTreeMap::new(),
            my_parent_ids: BTreeMap::new(),
            my_sumo_base_object_children: Vec::new(),
            my_vclass: SVC_IGNORING,
            my_vehicle_type_parameter: None,
            my_vehicle_parameter: None,
            my_stop_parameter: None,
            my_typed_parameters_pending: BTreeMap::new(),
            my_plan_parameters: CommonXMLStructure_PlanParameters::default(),
        })
    }

    /// clear SumoBaseObject
    pub fn clear(&mut self) {
        // reset tag
        self.my_tag = SUMO_TAG_NOTHING;
        // reset vClass
        self.my_vclass = SVC_IGNORING;
        // clear containers
        self.my_string_attributes.clear();
        self.my_int_attributes.clear();
        self.my_double_attributes.clear();
        self.my_bool_attributes.clear();
        self.my_position_attributes.clear();
        self.my_time_attributes.clear();
        self.my_color_attributes.clear();
        self.my_string_list_attributes.clear();
        self.my_double_list_attributes.clear();
        self.my_position_vector_attributes.clear();
        self.my_parent_ids.clear();
        self.my_parameters.clear();
        // C++ clears the child pointer list without deleting the children (they stay allocated)
        for child in std::mem::take(&mut self.my_sumo_base_object_children) {
            let _ = Box::leak(child);
        }
        // reset flags (defined typed parameters)
        self.my_vehicle_type_parameter = None;
        self.my_vehicle_parameter = None;
        self.my_stop_parameter = None;
        self.my_typed_parameters_pending.clear();
    }

    /// set SumoBaseObject tag
    pub fn set_tag(&mut self, tag: SumoXMLTag) {
        self.my_tag = tag;
    }

    /// mark as successfully created
    pub fn mark_as_created(&mut self) {
        self.my_was_created = true;
    }

    /// get XML myTag
    pub fn get_tag(&self) -> SumoXMLTag {
        self.my_tag
    }

    /// check if the object was successfully created in build<...> function
    pub fn was_created(&self) -> bool {
        self.my_was_created
    }

    /// get pointer to mySumoBaseObjectParent SumoBaseObject (if is null, then is the root)
    pub fn get_parent_sumo_base_object(&self) -> Ptr<CommonXMLStructure_SumoBaseObject> {
        self.my_sumo_base_object_parent
    }

    /// get all attributes in string format
    pub fn get_all_attributes(&self) -> BTreeMap<String, String> {
        let mut result: BTreeMap<String, String> = BTreeMap::new();
        for (attr, value) in &self.my_string_attributes {
            result.insert(attr_name(*attr), value.clone());
        }
        for (attr, value) in &self.my_int_attributes {
            result.insert(attr_name(*attr), to_string(value));
        }
        for (attr, value) in &self.my_double_attributes {
            result.insert(attr_name(*attr), to_string(value));
        }
        for (attr, value) in &self.my_bool_attributes {
            result.insert(attr_name(*attr), to_string(value));
        }
        for (attr, value) in &self.my_position_attributes {
            result.insert(attr_name(*attr), value.to_string_prec(g_precision()));
        }
        for (attr, value) in &self.my_time_attributes {
            result.insert(attr_name(*attr), time2string(*value));
        }
        for (attr, value) in &self.my_color_attributes {
            result.insert(attr_name(*attr), to_string(value));
        }
        for (attr, value) in &self.my_string_list_attributes {
            result.insert(attr_name(*attr), to_string(value));
        }
        for (attr, value) in &self.my_double_list_attributes {
            result.insert(attr_name(*attr), to_string(value));
        }
        for (attr, value) in &self.my_position_vector_attributes {
            result.insert(attr_name(*attr), value.to_string_prec(g_precision()));
        }
        result
    }

    /// get string attribute
    pub fn get_string_attribute(&self, attr: SumoXMLAttr) -> Result<&String, ProcessError> {
        match self.my_string_attributes.get(&attr) {
            Some(value) => Ok(value),
            None => {
                self.handle_attribute_error(attr, "string");
                Err(ProcessError::default_msg())
            }
        }
    }

    /// get int attribute
    pub fn get_int_attribute(&self, attr: SumoXMLAttr) -> Result<i32, ProcessError> {
        match self.my_int_attributes.get(&attr) {
            Some(value) => Ok(*value),
            None => {
                self.handle_attribute_error(attr, "int");
                Err(ProcessError::default_msg())
            }
        }
    }

    /// get double attribute
    pub fn get_double_attribute(&self, attr: SumoXMLAttr) -> Result<f64, ProcessError> {
        match self.my_double_attributes.get(&attr) {
            Some(value) => Ok(*value),
            None => {
                self.handle_attribute_error(attr, "double");
                Err(ProcessError::default_msg())
            }
        }
    }

    /// get bool attribute
    pub fn get_bool_attribute(&self, attr: SumoXMLAttr) -> Result<bool, ProcessError> {
        match self.my_bool_attributes.get(&attr) {
            Some(value) => Ok(*value),
            None => {
                self.handle_attribute_error(attr, "bool");
                Err(ProcessError::default_msg())
            }
        }
    }

    /// get Position attribute
    pub fn get_position_attribute(&self, attr: SumoXMLAttr) -> Result<&Position, ProcessError> {
        match self.my_position_attributes.get(&attr) {
            Some(value) => Ok(value),
            None => {
                self.handle_attribute_error(attr, "position");
                Err(ProcessError::default_msg())
            }
        }
    }

    /// get time attribute
    pub fn get_time_attribute(&self, attr: SumoXMLAttr) -> Result<SUMOTime, ProcessError> {
        match self.my_time_attributes.get(&attr) {
            Some(value) => Ok(*value),
            None => {
                self.handle_attribute_error(attr, "time");
                Err(ProcessError::default_msg())
            }
        }
    }

    /// get 'period' attribute
    pub fn get_period_attribute(&self) -> Result<SUMOTime, ProcessError> {
        let mut attr = SUMO_ATTR_PERIOD;
        if let Some(value) = self.my_time_attributes.get(&attr) {
            return Ok(*value);
        }
        // try 'freq' as alias for 'period'
        attr = SUMO_ATTR_FREQUENCY;
        if let Some(value) = self.my_time_attributes.get(&attr) {
            return Ok(*value);
        }
        self.handle_attribute_error(SUMO_ATTR_PERIOD, "time");
        Err(ProcessError::default_msg())
    }

    /// get color attribute
    pub fn get_color_attribute(&self, attr: SumoXMLAttr) -> Result<&RGBColor, ProcessError> {
        match self.my_color_attributes.get(&attr) {
            Some(value) => Ok(value),
            None => {
                self.handle_attribute_error(attr, "color");
                Err(ProcessError::default_msg())
            }
        }
    }

    /// get string list attribute
    pub fn get_string_list_attribute(&self, attr: SumoXMLAttr) -> Result<&Vec<String>, ProcessError> {
        match self.my_string_list_attributes.get(&attr) {
            Some(value) => Ok(value),
            None => {
                self.handle_attribute_error(attr, "string list");
                Err(ProcessError::default_msg())
            }
        }
    }

    /// get double list attribute
    pub fn get_double_list_attribute(&self, attr: SumoXMLAttr) -> Result<&Vec<f64>, ProcessError> {
        match self.my_double_list_attributes.get(&attr) {
            Some(value) => Ok(value),
            None => {
                self.handle_attribute_error(attr, "double list");
                Err(ProcessError::default_msg())
            }
        }
    }

    /// get PositionVector attribute
    pub fn get_position_vector_attribute(&self, attr: SumoXMLAttr) -> Result<&PositionVector, ProcessError> {
        match self.my_position_vector_attributes.get(&attr) {
            Some(value) => Ok(value),
            None => {
                self.handle_attribute_error(attr, "position vector");
                Err(ProcessError::default_msg())
            }
        }
    }

    /// get parent ID
    pub fn get_parent_id(&self, tag: SumoXMLTag) -> Result<&String, ProcessError> {
        match self.my_parent_ids.get(&tag) {
            Some(value) => Ok(value),
            None => {
                crate::write_errorf!("Trying to get undefined parent '%' in SUMOBaseObject '%'", tag_name(tag), tag_name(self.my_tag));
                Err(ProcessError::default_msg())
            }
        }
    }

    /// vehicle class
    pub fn get_vclass(&self) -> SUMOVehicleClass {
        self.my_vclass
    }

    /// get current vType (opaque SUMOVTypeParameter)
    pub fn get_vehicle_type_parameter(&self) -> Result<&dyn Any, ProcessError> {
        match &self.my_vehicle_type_parameter {
            Some(p) => Ok(&**p),
            None => Err(ProcessError::new("Undefined vehicleType parameter")),
        }
    }

    /// get vehicle parameters (opaque SUMOVehicleParameter)
    pub fn get_vehicle_parameter(&self) -> Result<&dyn Any, ProcessError> {
        match &self.my_vehicle_parameter {
            Some(p) => Ok(&**p),
            None => Err(ProcessError::new("Undefined vehicle parameter")),
        }
    }

    /// get stop parameters (opaque SUMOVehicleParameter::Stop)
    pub fn get_stop_parameter(&self) -> Result<&dyn Any, ProcessError> {
        match &self.my_stop_parameter {
            Some(p) => Ok(&**p),
            None => Err(ProcessError::new("Undefined stop parameter")),
        }
    }

    /// get parameters
    pub fn get_parameters(&self) -> &BTreeMap<String, String> {
        &self.my_parameters
    }

    /// get plan parameteres
    pub fn get_plan_parameters(&self) -> &CommonXMLStructure_PlanParameters {
        &self.my_plan_parameters
    }

    /// get SumoBaseObject children
    pub fn get_sumo_base_object_children(&self) -> &Vec<Box<CommonXMLStructure_SumoBaseObject>> {
        &self.my_sumo_base_object_children
    }

    /// check if current SumoBaseObject has the given string attribute
    pub fn has_string_attribute(&self, attr: SumoXMLAttr) -> bool {
        self.my_string_attributes.contains_key(&attr)
    }

    /// check if current SumoBaseObject has the given int attribute
    pub fn has_int_attribute(&self, attr: SumoXMLAttr) -> bool {
        self.my_int_attributes.contains_key(&attr)
    }

    /// check if current SumoBaseObject has the given double attribute
    pub fn has_double_attribute(&self, attr: SumoXMLAttr) -> bool {
        self.my_double_attributes.contains_key(&attr)
    }

    /// check if current SumoBaseObject has the given bool attribute
    pub fn has_bool_attribute(&self, attr: SumoXMLAttr) -> bool {
        self.my_bool_attributes.contains_key(&attr)
    }

    /// check if current SumoBaseObject has the given bool attribute
    pub fn has_position_attribute(&self, attr: SumoXMLAttr) -> bool {
        self.my_position_attributes.contains_key(&attr)
    }

    /// check if current SumoBaseObject has the given time attribute
    pub fn has_time_attribute(&self, attr: SumoXMLAttr) -> bool {
        self.my_time_attributes.contains_key(&attr)
    }

    /// check if current SumoBaseObject has the given color attribute
    pub fn has_color_attribute(&self, attr: SumoXMLAttr) -> bool {
        self.my_color_attributes.contains_key(&attr)
    }

    /// check if current SumoBaseObject has the given string list attribute
    pub fn has_string_list_attribute(&self, attr: SumoXMLAttr) -> bool {
        self.my_string_list_attributes.contains_key(&attr)
    }

    /// check if current SumoBaseObject has the given double list attribute
    pub fn has_double_list_attribute(&self, attr: SumoXMLAttr) -> bool {
        self.my_double_list_attributes.contains_key(&attr)
    }

    /// check if current SumoBaseObject has the given positionVector attribute
    pub fn has_position_vector_attribute(&self, attr: SumoXMLAttr) -> bool {
        self.my_position_vector_attributes.contains_key(&attr)
    }

    /// check if current SumoBaseObject has the given parent ID
    pub fn has_parent_id(&self, tag: SumoXMLTag) -> bool {
        self.my_parent_ids.contains_key(&tag)
    }

    /// add string attribute into current SumoBaseObject node
    pub fn add_string_attribute(&mut self, attr: SumoXMLAttr, value: &str) {
        self.my_string_attributes.insert(attr, value.to_string());
    }

    /// add int attribute into current SumoBaseObject node
    pub fn add_int_attribute(&mut self, attr: SumoXMLAttr, value: i32) {
        self.my_int_attributes.insert(attr, value);
    }

    /// add double attribute into current SumoBaseObject node
    pub fn add_double_attribute(&mut self, attr: SumoXMLAttr, value: f64) {
        self.my_double_attributes.insert(attr, value);
    }

    /// add bool attribute into current SumoBaseObject node
    pub fn add_bool_attribute(&mut self, attr: SumoXMLAttr, value: bool) {
        self.my_bool_attributes.insert(attr, value);
    }

    /// add Position attribute into current SumoBaseObject node
    pub fn add_position_attribute(&mut self, attr: SumoXMLAttr, value: &Position) {
        self.my_position_attributes.insert(attr, *value);
    }

    /// add time attribute into current SumoBaseObject node
    pub fn add_time_attribute(&mut self, attr: SumoXMLAttr, value: SUMOTime) {
        self.my_time_attributes.insert(attr, value);
    }

    /// add color attribute into current SumoBaseObject node
    pub fn add_color_attribute(&mut self, attr: SumoXMLAttr, value: &RGBColor) {
        self.my_color_attributes.insert(attr, *value);
    }

    /// add string list attribute into current SumoBaseObject node
    pub fn add_string_list_attribute(&mut self, attr: SumoXMLAttr, value: &[String]) {
        self.my_string_list_attributes.insert(attr, value.to_vec());
    }

    /// add double list attribute into current SumoBaseObject node
    pub fn add_double_list_attribute(&mut self, attr: SumoXMLAttr, value: &[f64]) {
        self.my_double_list_attributes.insert(attr, value.to_vec());
    }

    /// add PositionVector attribute into current SumoBaseObject node
    pub fn add_position_vector_attribute(&mut self, attr: SumoXMLAttr, value: &PositionVector) {
        self.my_position_vector_attributes.insert(attr, value.clone());
    }

    /// add parameters into current SumoBaseObject node (format: key=value1|key2=value2|....)
    pub fn add_parameters(&mut self, value: &str) {
        let parameters = StringTokenizer::with_token(value, "|", false).get_vector();
        for parameter in parameters {
            // C++ indexes keyValue[0] and keyValue[1] without checking the size
            let key_value = StringTokenizer::with_token(&parameter, "=", false).get_vector();
            self.add_parameter(&key_value[0], &key_value[1]);
        }
    }

    /// add parameter into current SumoBaseObject node
    pub fn add_parameter(&mut self, key: &str, value: &str) {
        // check if we have to insert in vType, vehicle or stop parameters
        if self.my_vehicle_type_parameter.is_some() || self.my_vehicle_parameter.is_some() || self.my_stop_parameter.is_some() {
            // shortcut: typed parameters are kept here until SUMOVTypeParameter/SUMOVehicleParameter are ported, then they go into those objects
            self.my_typed_parameters_pending.insert(key.to_string(), value.to_string());
        } else {
            self.my_parameters.insert(key.to_string(), value.to_string());
        }
    }

    /// add parent (string) attribute into current SumoBaseObject node
    pub fn add_parent_id(&mut self, tag: SumoXMLTag, id: &str) {
        self.my_parent_ids.insert(tag, id.to_string());
    }

    /// set vehicle class
    pub fn set_vclass(&mut self, vclass: SUMOVehicleClass) {
        self.my_vclass = vclass;
    }

    /// set vehicle type parameters (`vehicle_type_parameter` is the opaque SUMOVTypeParameter, `id` its id)
    pub fn set_vehicle_type_parameter(&mut self, vehicle_type_parameter: Box<dyn Any>, id: &str) {
        self.my_vehicle_type_parameter = Some(vehicle_type_parameter);
        // set attribute id
        self.add_string_attribute(SUMO_ATTR_ID, id);
    }

    /// set vehicle parameters (`vehicle_parameter` is the opaque SUMOVehicleParameter, `id`/`routeid` its fields)
    pub fn set_vehicle_parameter(&mut self, vehicle_parameter: Box<dyn Any>, id: &str, routeid: &str) {
        self.my_vehicle_parameter = Some(vehicle_parameter);
        // set attribute id
        if !id.is_empty() {
            self.add_string_attribute(SUMO_ATTR_ID, id);
        }
        // set attribute route
        if !routeid.is_empty() {
            self.add_string_attribute(SUMO_ATTR_ROUTE, routeid);
        }
    }

    /// add stop parameters (`stop_parameter` is the opaque SUMOVehicleParameter::Stop, the other arguments its fields)
    #[allow(clippy::too_many_arguments)]
    pub fn set_stop_parameter(&mut self, stop_parameter: Box<dyn Any>, edge: &str, lane: &str, busstop: &str, containerstop: &str, parkingarea: &str, charging_station: &str) {
        self.my_stop_parameter = Some(stop_parameter);
        // set attribute edge
        if !edge.is_empty() {
            self.add_string_attribute(SUMO_ATTR_EDGE, edge);
        }
        // set attribute lane
        if !lane.is_empty() {
            self.add_string_attribute(SUMO_ATTR_LANE, lane);
        }
        // set attribute busStop
        if !busstop.is_empty() {
            self.add_string_attribute(SUMO_ATTR_BUS_STOP, busstop);
        }
        // set attribute containerstop
        if !containerstop.is_empty() {
            self.add_string_attribute(SUMO_ATTR_CONTAINER_STOP, containerstop);
        }
        // set attribute parkingarea
        if !parkingarea.is_empty() {
            self.add_string_attribute(SUMO_ATTR_PARKING_AREA, parkingarea);
        }
        // set attribute chargingStation
        if !charging_station.is_empty() {
            self.add_string_attribute(SUMO_ATTR_CHARGING_STATION, charging_station);
        }
    }

    /// set plan parmeter
    pub fn set_plan_parameters(&mut self, plan_parameters: &CommonXMLStructure_PlanParameters) {
        self.my_plan_parameters = plan_parameters.clone();
    }

    /// add SumoBaseObject child (takes ownership, like C++ adding a `new` object)
    pub fn add_sumo_base_object_child(&mut self, sumo_base_object: Box<CommonXMLStructure_SumoBaseObject>) {
        // just add it into mySumoBaseObjectChildren
        self.my_sumo_base_object_children.push(sumo_base_object);
    }

    /// remove SumoBaseObject child: returns the owned child, which the caller drops (C++ `delete`)
    pub fn remove_sumo_base_object_child(&mut self, sumo_base_object: Ptr<CommonXMLStructure_SumoBaseObject>) -> Option<Box<CommonXMLStructure_SumoBaseObject>> {
        // find sumoBaseObject
        let idx = self.my_sumo_base_object_children.iter().position(|c| Ptr::from_box(c) == sumo_base_object)?;
        Some(self.my_sumo_base_object_children.remove(idx))
    }

    /// handle attribute error
    fn handle_attribute_error(&self, attr: SumoXMLAttr, attr_type: &str) {
        crate::write_errorf!("Trying to get undefined % attribute '%' in SUMOBaseObject '%'", attr_type, attr_name(attr), tag_name(self.my_tag));
    }
}

// ---------------------------------------------------------------------------
// CommonXMLStructure - methods
// ---------------------------------------------------------------------------

impl CommonXMLStructure {
    /// open SUMOBaseOBject
    pub fn open_sumo_base_object(&mut self) {
        // first check if root is empty
        if self.my_sumo_base_object_root.is_null() {
            // create root
            let mut root = CommonXMLStructure_SumoBaseObject::new(Ptr::null());
            // set tag
            root.set_tag(SUMO_TAG_ROOTFILE);
            self.my_sumo_base_object_root = Ptr::from_box(&root);
            // update last inserted Root
            self.my_current_sumo_base_object = self.my_sumo_base_object_root;
            self.my_root_owner = Some(root);
        } else {
            // create new node
            let new_sumo_base_object = CommonXMLStructure_SumoBaseObject::new(self.my_current_sumo_base_object);
            let new_ptr = Ptr::from_box(&new_sumo_base_object);
            self.my_current_sumo_base_object.m().add_sumo_base_object_child(new_sumo_base_object);
            // update last inserted node
            self.my_current_sumo_base_object = new_ptr;
        }
    }

    /// close SUMOBaseOBject
    pub fn close_sumo_base_object(&mut self) {
        // check that myCurrentSumoBaseObject is valid
        if !self.my_current_sumo_base_object.is_null() {
            // check if last inserted SumoBaseObject is the root
            if self.my_current_sumo_base_object.get_parent_sumo_base_object().is_null() {
                // reset both pointers (C++ does not delete the root here, so it stays allocated)
                self.my_current_sumo_base_object = Ptr::null();
                self.my_sumo_base_object_root = Ptr::null();
                if let Some(root) = self.my_root_owner.take() {
                    let _ = Box::leak(root);
                }
            } else {
                // update last inserted SumoBaseObject
                self.my_current_sumo_base_object = self.my_current_sumo_base_object.get_parent_sumo_base_object();
            }
        }
    }

    /// abort SUMOBaseOBject
    pub fn abort_sumo_base_object(&mut self) {
        // delete current sumo base object and use their parent as sumo base object
        if !self.my_current_sumo_base_object.is_null() {
            if self.my_current_sumo_base_object == self.my_sumo_base_object_root {
                self.my_root_owner = None;
                self.my_current_sumo_base_object = Ptr::null();
                self.my_sumo_base_object_root = Ptr::null();
            } else {
                let parent_sumo_base_object = self.my_current_sumo_base_object.get_parent_sumo_base_object();
                let current = self.my_current_sumo_base_object;
                // unregister and drop the current object (C++ delete)
                parent_sumo_base_object.m().remove_sumo_base_object_child(current);
                self.my_current_sumo_base_object = parent_sumo_base_object;
            }
        }
    }

    /// get SumoBaseObject root
    pub fn get_sumo_base_object_root(&self) -> Ptr<CommonXMLStructure_SumoBaseObject> {
        self.my_sumo_base_object_root
    }

    /// get current editedSumoBaseObject
    pub fn get_current_sumo_base_object(&self) -> Ptr<CommonXMLStructure_SumoBaseObject> {
        self.my_current_sumo_base_object
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn plan(from: &str, to: &str) -> CommonXMLStructure_PlanParameters {
        CommonXMLStructure_PlanParameters { from_edge: from.to_string(), to_edge: to.to_string(), ..Default::default() }
    }

    #[test]
    fn plan_single_edge_and_counts() {
        // from only: single-edge plan (one defined parameter)
        let p = plan("a", "");
        assert!(p.is_single_edge_plan());
        assert_eq!(p.get_number_of_defined_parameters(), 1);
        // from and to: two defined parameters, not single-edge
        let q = plan("a", "b");
        assert!(!q.is_single_edge_plan());
        assert_eq!(q.get_number_of_defined_parameters(), 2);
        assert_eq!(q.get_walk_tag(), GNE_TAG_WALK_EDGE_EDGE);
        assert_eq!(q.get_person_trip_tag(), GNE_TAG_PERSONTRIP_EDGE_EDGE);
        // no from: nothing defined
        assert_eq!(CommonXMLStructure_PlanParameters::default().get_walk_tag(), SUMO_TAG_NOTHING);
    }

    #[test]
    fn plan_consecutive_and_stops() {
        let mut p = CommonXMLStructure_PlanParameters::default();
        p.consecutive_edges = vec!["a".to_string(), "b".to_string()];
        assert_eq!(p.get_number_of_defined_parameters(), 2);
        assert_eq!(p.get_walk_tag(), GNE_TAG_WALK_EDGES);
        assert_eq!(p.get_tranship_tag(), GNE_TAG_TRANSHIP_EDGES);
        let mut s = CommonXMLStructure_PlanParameters::default();
        s.to_bus_stop = "stop".to_string();
        assert_eq!(s.get_person_stop_tag(), GNE_TAG_STOPPERSON_BUSSTOP);
        assert_eq!(s.get_container_stop_tag(), GNE_TAG_STOPCONTAINER_BUSSTOP);
    }

    #[test]
    fn base_object_tree_open_abort_close() {
        let mut cxs = CommonXMLStructure::default();
        cxs.open_sumo_base_object();
        let root = cxs.get_sumo_base_object_root();
        assert_eq!(root.get_tag(), SUMO_TAG_ROOTFILE);
        cxs.open_sumo_base_object();
        assert_eq!(root.get_sumo_base_object_children().len(), 1);
        assert_ne!(cxs.get_current_sumo_base_object(), root);
        cxs.abort_sumo_base_object();
        assert_eq!(root.get_sumo_base_object_children().len(), 0);
        assert_eq!(cxs.get_current_sumo_base_object(), root);
        cxs.close_sumo_base_object();
        assert!(cxs.get_sumo_base_object_root().is_null());
        assert!(cxs.get_current_sumo_base_object().is_null());
    }

    #[test]
    fn base_object_attributes() {
        let mut obj = CommonXMLStructure_SumoBaseObject::new(Ptr::null());
        obj.add_int_attribute(SUMO_ATTR_ID, 3);
        assert!(obj.has_int_attribute(SUMO_ATTR_ID));
        assert_eq!(obj.get_int_attribute(SUMO_ATTR_ID).unwrap(), 3);
        assert!(obj.get_string_attribute(SUMO_ATTR_ID).is_err());
        obj.add_parameters("k1=v1|k2=v2");
        assert_eq!(obj.get_parameters().get("k2").map(String::as_str), Some("v2"));
    }
}
