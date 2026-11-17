//! Port of microsim/output/MSChargingStationExport.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// Realises dumping Battery Data
/// C++ class `MSChargingStationExport` (microsim/output/MSChargingStationExport.h)
pub struct MSChargingStationExport {
}

impl MSChargingStationExport {
    /// C++: `void MSChargingStationExport::write(OutputDevice &, bool)` [static]
    /// Writes the recently completed charging events
    // void
    // MSChargingStationExport::write(OutputDevice& of, bool end) {
    //     // loop through charging stations
    //     bool includeUnfinished = end && OptionsCont::getOptions().getBool("chargingstations-output.aggregated.write-unfinished");
    //     for (const auto& stop : MSNet::getInstance()->getStoppingPlaces(SUMO_TAG_CHARGING_STATION)) {
    //         static_cast<MSChargingStation*>(stop.second)->writeAggregatedChargingStationOutput(of, includeUnfinished);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn write(of: &mut OutputDevice, end: bool) {
        // loop through charging stations
        let includeUnfinished = end && utils::options::options_cont::get_bool("chargingstations-output.aggregated.write-unfinished");
        for (_, stop) in MSNet::get_instance().m().get_stopping_places(utils::xml::sumo_xml_definitions::SUMO_TAG_CHARGING_STATION) {
            stop.cast::<crate::microsim::trigger::ms_charging_station::MSChargingStation>().m().write_aggregated_charging_station_output(of, includeUnfinished);
        }
    }

    /// C++: `void MSChargingStationExport::MSChargingStationExport(const MSChargingStationExport &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new(arg0: &MSChargingStationExport) -> Self {
        panic!("MSChargingStationExport copy constructor is private and invalidated in C++")
    }

    /// C++: `MSChargingStationExport & MSChargingStationExport::operator=(const MSChargingStationExport &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSChargingStationExport) -> &mut MSChargingStationExport {
        panic!("MSChargingStationExport assignment operator is private and invalidated in C++")
    }

}

