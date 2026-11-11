//! Port of traci-server/TraCIServerAPI_MeanData.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// APIs for getting/setting busstop values via TraCI
/// C++ class `TraCIServerAPI_MeanData` (traci-server/TraCIServerAPI_MeanData.h)
pub struct TraCIServerAPI_MeanData {
}

impl TraCIServerAPI_MeanData {
    /// C++: `bool TraCIServerAPI_MeanData::processSet(TraCIServer &, tcpip::Storage &, tcpip::Storage &)` [static]
    /// Processes a set value command (Command 0xc6: Change MeanData State)
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn process_set(server: &mut TraCIServer, inputStorage: &mut Storage, outputStorage: &mut Storage) -> bool {
        // C++ declares processSet but has no definition in TraCIServerAPI_MeanData.cpp; a call is a link error in C++
        panic!("TraCIServerAPI_MeanData::processSet has no C++ definition")
    }

    /// C++: `void TraCIServerAPI_MeanData::TraCIServerAPI_MeanData(const TraCIServerAPI_MeanData &)` [private]
    /// invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new(s: &TraCIServerAPI_MeanData) -> Self {
        panic!("TraCIServerAPI_MeanData copy constructor is private and undefined in C++; it cannot be called")
    }

    /// C++: `TraCIServerAPI_MeanData & TraCIServerAPI_MeanData::operator=(const TraCIServerAPI_MeanData &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &TraCIServerAPI_MeanData) -> &mut TraCIServerAPI_MeanData {
        panic!("TraCIServerAPI_MeanData assignment operator is private and undefined in C++; it cannot be called")
    }

}

