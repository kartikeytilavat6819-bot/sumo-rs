//! Port of traci-server/TraCIServerAPI_ChargingStation.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// APIs for getting/setting busstop values via TraCI
/// C++ class `TraCIServerAPI_ChargingStation` (traci-server/TraCIServerAPI_ChargingStation.h)
pub struct TraCIServerAPI_ChargingStation {
}

impl TraCIServerAPI_ChargingStation {
    /// C++: `bool TraCIServerAPI_ChargingStation::processSet(TraCIServer &, tcpip::Storage &, tcpip::Storage &)` [static]
    /// Processes a set value command (Command 0xc6: Change ChargingStation State)
    // bool
    // TraCIServerAPI_ChargingStation::processSet(TraCIServer& server, tcpip::Storage& inputStorage,
    //         tcpip::Storage& outputStorage) {
    //     std::string warning = ""; // additional description for response
    //     // variable
    //     int variable = inputStorage.readUnsignedByte();
    //     if (variable != libsumo::VAR_PARAMETER &&
    //             variable != libsumo::VAR_CS_POWER &&
    //             variable != libsumo::VAR_CS_EFFICIENCY &&
    //             variable != libsumo::VAR_CS_CHARGE_DELAY &&
    //             variable != libsumo::VAR_CS_CHARGE_IN_TRANSIT &&
    //             variable != libsumo::VAR_CS_TOTAL_POWER) {
    //         return server.writeErrorStatusCmd(libsumo::CMD_SET_CHARGINGSTATION_VARIABLE, "Change ChargingStation State: unsupported variable " + toHex(variable, 2) + " specified", outputStorage);
    //     }
    //     // id
    //     std::string id = inputStorage.readString();
    //
    //     try {
    //         // process
    //         switch (variable) {
    //             case libsumo::VAR_PARAMETER: {
    //                 StoHelp::readCompound(inputStorage, 2, "A compound object of size 2 is needed for setting a parameter.");
    //                 const std::string name = StoHelp::readTypedString(inputStorage, "The name of the parameter must be given as a string.");
    //                 const std::string value = StoHelp::readTypedString(inputStorage, "The value of the parameter must be given as a string.");
    //                 libsumo::ChargingStation::setParameter(id, name, value);
    //             }
    //             break;
    //             case libsumo::VAR_CS_POWER: {
    //                 libsumo::ChargingStation::setChargingPower(id, StoHelp::readTypedDouble(inputStorage, "Setting chargingPower requires a double."));
    //             }
    //             break;
    //             case libsumo::VAR_CS_EFFICIENCY: {
    //                 libsumo::ChargingStation::setEfficiency(id, StoHelp::readTypedDouble(inputStorage, "Setting efficiency requires a double."));
    //             }
    //             break;
    //             case libsumo::VAR_CS_CHARGE_DELAY: {
    //                 libsumo::ChargingStation::setChargeDelay(id, StoHelp::readTypedDouble(inputStorage, "Setting charge delay requires a double."));
    //             }
    //             break;
    //             case libsumo::VAR_CS_CHARGE_IN_TRANSIT: {
    //                 libsumo::ChargingStation::setChargeInTransit(id, StoHelp::readTypedInt(inputStorage, "Setting charge in transit requires an integer.") != 0);
    //             }
    //             break;
    //             case libsumo::VAR_CS_TOTAL_POWER: {
    //                 libsumo::ChargingStation::setTotalPower(id, StoHelp::readTypedDouble(inputStorage, "Setting totalPower requires a double."));
    //             }
    //             break;
    //             default:
    //                 break;
    //         }
    //     } catch (libsumo::TraCIException& e) {
    //         return server.writeErrorStatusCmd(libsumo::CMD_SET_CHARGINGSTATION_VARIABLE, e.what(), outputStorage);
    //     }
    //     server.writeStatusCmd(libsumo::CMD_SET_CHARGINGSTATION_VARIABLE, libsumo::RTYPE_OK, warning, outputStorage);
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn process_set(server: &mut TraCIServer, inputStorage: &mut Storage, outputStorage: &mut Storage) -> bool {
        let warning = String::new(); // additional description for response
        // variable
        let variable = inputStorage.read_unsigned_byte().expect("readUnsignedByte");
        if variable != VAR_PARAMETER
            && variable != VAR_CS_POWER
            && variable != VAR_CS_EFFICIENCY
            && variable != VAR_CS_CHARGE_DELAY
            && variable != VAR_CS_CHARGE_IN_TRANSIT
            && variable != VAR_CS_TOTAL_POWER
        {
            return server.write_error_status_cmd(
                CMD_SET_CHARGINGSTATION_VARIABLE,
                &format!("Change ChargingStation State: unsupported variable {} specified", to_hex(variable as u64, 2, 0)),
                outputStorage,
            );
        }
        // id
        let id = inputStorage.read_string().expect("readString");

        // shortcut: libsumo has no TraCIException mapping yet, so the C++ catch (which writes an error status) is not reproduced; upgrade when libsumo gets an error convention
        // process
        match variable {
            VAR_PARAMETER => {
                StorageHelper::read_compound(inputStorage, 2, "A compound object of size 2 is needed for setting a parameter.");
                let name = StorageHelper::read_typed_string(inputStorage, "The name of the parameter must be given as a string.");
                let value = StorageHelper::read_typed_string(inputStorage, "The value of the parameter must be given as a string.");
                ChargingStation::set_parameter(&id, &name, &value);
            }
            VAR_CS_POWER => {
                ChargingStation::set_charging_power(&id, StorageHelper::read_typed_double(inputStorage, "Setting chargingPower requires a double."));
            }
            VAR_CS_EFFICIENCY => {
                ChargingStation::set_efficiency(&id, StorageHelper::read_typed_double(inputStorage, "Setting efficiency requires a double."));
            }
            VAR_CS_CHARGE_DELAY => {
                ChargingStation::set_charge_delay(&id, StorageHelper::read_typed_double(inputStorage, "Setting charge delay requires a double."));
            }
            VAR_CS_CHARGE_IN_TRANSIT => {
                ChargingStation::set_charge_in_transit(&id, StorageHelper::read_typed_int(inputStorage, "Setting charge in transit requires an integer.") != 0);
            }
            VAR_CS_TOTAL_POWER => {
                ChargingStation::set_total_power(&id, StorageHelper::read_typed_double(inputStorage, "Setting totalPower requires a double."));
            }
            _ => {}
        }
        server.write_status_cmd(CMD_SET_CHARGINGSTATION_VARIABLE, RTYPE_OK, &warning, outputStorage);
        true
    }

    /// C++: `void TraCIServerAPI_ChargingStation::TraCIServerAPI_ChargingStation(const TraCIServerAPI_ChargingStation &)` [private]
    /// invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new(s: &TraCIServerAPI_ChargingStation) -> Self {
        panic!("TraCIServerAPI_ChargingStation copy constructor is private and undefined in C++; it cannot be called")
    }

    /// C++: `TraCIServerAPI_ChargingStation & TraCIServerAPI_ChargingStation::operator=(const TraCIServerAPI_ChargingStation &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &TraCIServerAPI_ChargingStation) -> &mut TraCIServerAPI_ChargingStation {
        panic!("TraCIServerAPI_ChargingStation assignment operator is private and undefined in C++; it cannot be called")
    }

}

