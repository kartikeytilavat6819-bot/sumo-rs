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
use crate::common::to_string::{f64_to_string, to_string};
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
