//! Port of od/ODCell.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `std::map<SUMOTime, std::vector<SUMOVehicleParameter>> ODCell::Departures`
pub type ODCell_Departures = BTreeMap<SUMOTime, Vec<SUMOVehicleParameter>>;

/// A single O/D-matrix cell
/// C++ class `ODCell` (od/ODCell.h)
pub struct ODCell {
    /// The number of vehicles
    pub vehicleNumber: f64,
    /// The begin time this cell describes
    pub begin: SUMOTime,
    /// The end time this cell describes
    pub end: SUMOTime,
    /// Name of the origin district
    pub origin: String,
    /// Name of the destination district
    pub destination: String,
    /// Name of the vehicle type
    pub vehicleType: String,
    /// the list of paths / routes
    pub pathsVector: Vec<Rc<RORoute>>,
    pub departures: ODCell_Departures,
    /// the origin "district" is an edge id
    pub originIsEdge: bool,
    /// the destination "district" is an edge id
    pub destinationIsEdge: bool,
}

impl ODCell {
}

