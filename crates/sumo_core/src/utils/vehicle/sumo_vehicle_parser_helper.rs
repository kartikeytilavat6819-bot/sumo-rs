//! Port of utils/vehicle/SUMOVehicleParserHelper.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::msg_handler::{write_error, write_warning};
use utils::common::sumo_vehicle_class::{DEFAULT_CONTAINERTYPE_ID, DEFAULT_PEDTYPE_ID};
use utils::common::string_utils::to_double;
use utils::common::time::{time2steps, SUMOTIME_MAX};
use utils::xml::sumo_xml_definitions::{
    is_valid_vehicle_id, Tags, SUMO_ATTR_BEGIN, SUMO_ATTR_CONTAINERSPERHOUR, SUMO_ATTR_END, SUMO_ATTR_ID,
    SUMO_ATTR_NUMBER, SUMO_ATTR_PERHOUR, SUMO_ATTR_PERIOD, SUMO_ATTR_PERSONSPERHOUR, SUMO_ATTR_PROB,
    SUMO_ATTR_VEHSPERHOUR, SUMO_TAG_CONTAINERFLOW, SUMO_TAG_PERSONFLOW,
};

// C++ #define values from utils/vehicle/SUMOVehicleParameter.h (private there, mirrored for the parametersSet bits)
const VEHPARS_END_SET: i64 = 2 << 4;
const VEHPARS_NUMBER_SET: i64 = 2 << 5;
const VEHPARS_PERIOD_SET: i64 = 2 << 6;
const VEHPARS_VPH_SET: i64 = 2 << 7;
const VEHPARS_PROB_SET: i64 = 2 << 8;
const VEHPARS_POISSON_SET: i64 = 2 << 9;

/// C++ `throw ProcessError(msg)`: panics with the error so that `catch_process_error` can recover it
fn throw_process_error(msg: String) -> ! {
    std::panic::panic_any(ProcessError::new(msg))
}

/// C++ `try { f(); } catch (ProcessError& e) {...}`: returns the caught error, re-raises anything else
fn catch_process_error(f: impl FnOnce()) -> Result<(), ProcessError> {
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)) {
        Ok(()) => Ok(()),
        Err(payload) => match payload.downcast::<ProcessError>() {
            Ok(e) => Err(*e),
            Err(other) => std::panic::resume_unwind(other),
        },
    }
}

/// C++ `flow[0] = (char)::toupper((char)flow[0])`
fn upper_first(s: &str) -> String {
    let mut out = s.to_string();
    if let Some(c) = out.get_mut(0..1) {
        c.make_ascii_uppercase();
    }
    out
}

/// C++ `std::string::substr(pos, n)`: n is clamped to the end; pos past the end throws std::out_of_range
fn cpp_substr(s: &str, pos: usize, n: usize) -> &str {
    if pos > s.len() {
        panic!("basic_string::substr: __pos ({}) > this->size() ({})", pos, s.len());
    }
    &s[pos..pos.saturating_add(n).min(s.len())]
}

/// C++ typedef `int SUMOVehicleParserHelper::CFAttrMap`
pub type SUMOVehicleParserHelper_CFAttrMap = i32;

/// C++ typedef `int SUMOVehicleParserHelper::LCAttrMap`
pub type SUMOVehicleParserHelper_LCAttrMap = i32;

/// Helper methods for parsing vehicle attributes
/// C++ class `SUMOVehicleParserHelper` (utils/vehicle/SUMOVehicleParserHelper.h)
pub struct SUMOVehicleParserHelper {
}

// C++ static members of SUMOVehicleParserHelper (port as module-level statics, see AGENTS.md):
//   static CFAttrMap SUMOVehicleParserHelper::allowedCFModelAttrs;
//   static LCAttrMap SUMOVehicleParserHelper::allowedLCModelAttrs;

impl SUMOVehicleParserHelper {
    /// C++: `SUMOVehicleParameter * SUMOVehicleParserHelper::parseFlowAttributes(SumoXMLTag, const SUMOSAXAttributes &, const bool, const bool, const SUMOTime, const SUMOTime, const bool)` [static]
    /// Parses a flow's attributes
    // SUMOVehicleParameter*
    // SUMOVehicleParserHelper::parseFlowAttributes(SumoXMLTag tag, const SUMOSAXAttributes& attrs, const bool hardFail, const bool needID,
    //         const SUMOTime beginDefault, const SUMOTime endDefault, const bool allowInternalRoutes) {
    //     // first parse ID
    //     const std::string id = attrs.hasAttribute(SUMO_ATTR_ID) ? parseID(attrs, tag) : "";
    //     // check if ID is valid
    //     if (!needID || !id.empty()) {
    //         if (needID && !SUMOXMLDefinitions::isValidVehicleID(id)) {
    //             return handleVehicleError(hardFail, nullptr, "Invalid flow id '" + id + "'.");
    //         }
    //         // declare flags
    //         const bool hasPeriod = attrs.hasAttribute(SUMO_ATTR_PERIOD);
    //         const bool hasVPH = attrs.hasAttribute(SUMO_ATTR_VEHSPERHOUR);
    //         const bool hasPPH = attrs.hasAttribute(SUMO_ATTR_PERSONSPERHOUR);
    //         const bool hasCPH = attrs.hasAttribute(SUMO_ATTR_CONTAINERSPERHOUR);
    //         const bool hasPH = attrs.hasAttribute(SUMO_ATTR_PERHOUR);
    //         const bool hasXPH = hasVPH || hasPPH || hasCPH || hasPH;
    //         const bool hasProb = attrs.hasAttribute(SUMO_ATTR_PROB);
    //         const bool hasNumber = attrs.hasAttribute(SUMO_ATTR_NUMBER);
    //         const bool hasBegin = attrs.hasAttribute(SUMO_ATTR_BEGIN);
    //         const bool hasEnd = attrs.hasAttribute(SUMO_ATTR_END);
    //         SumoXMLAttr PERHOUR = SUMO_ATTR_PERHOUR;
    //         if (hasVPH) {
    //             PERHOUR = SUMO_ATTR_VEHSPERHOUR;
    //         }
    //         if (hasPPH) {
    //             PERHOUR = SUMO_ATTR_PERSONSPERHOUR;
    //         }
    //         if (hasCPH) {
    //             PERHOUR = SUMO_ATTR_CONTAINERSPERHOUR;
    //         }
    //         if (hasXPH && !(hasVPH ^ hasPPH ^ hasCPH ^ hasPH)) {
    //             return handleVehicleError(hardFail, nullptr,
    //                                       "At most one of '" + attrs.getName(SUMO_ATTR_PERHOUR) +
    //                                       "', '" + attrs.getName(SUMO_ATTR_VEHSPERHOUR) +
    //                                       "', '" + attrs.getName(SUMO_ATTR_PERSONSPERHOUR) +
    //                                       "' and '" + attrs.getName(SUMO_ATTR_CONTAINERSPERHOUR) +
    //                                       "' has to be given in the definition of " + toString(tag) + " '" + id + "'.");
    //         }
    //         if (hasPeriod && hasXPH) {
    //             return handleVehicleError(hardFail, nullptr,
    //                                       "At most one of '" + attrs.getName(SUMO_ATTR_PERIOD) +
    //                                       "' and '" + attrs.getName(PERHOUR) +
    //                                       "' has to be given in the definition of "
    //                                       + toString(tag) + " '" + id + "'.");
    //         }
    //         if (hasPeriod && hasProb) {
    //             return handleVehicleError(hardFail, nullptr,
    //                                       "At most one of '" + attrs.getName(SUMO_ATTR_PERIOD) +
    //                                       "' and '" + attrs.getName(SUMO_ATTR_PROB) +
    //                                       "' has to be given in the definition of "
    //                                       + toString(tag) + " '" + id + "'.");
    //         }
    //         if (hasProb && hasXPH) {
    //             return handleVehicleError(hardFail, nullptr,
    //                                       "At most one of '" + attrs.getName(SUMO_ATTR_PROB) +
    //                                       "' and '" + attrs.getName(PERHOUR) +
    //                                       "' has to be given in the definition of "
    //                                       + toString(tag) + " '" + id + "'.");
    //         }
    //         if (hasPeriod || hasXPH || hasProb) {
    //             if (hasEnd && hasNumber) {
    //                 return handleVehicleError(hardFail, nullptr,
    //                                           "If '" + attrs.getName(SUMO_ATTR_PERIOD) +
    //                                           "', '" + attrs.getName(SUMO_ATTR_VEHSPERHOUR) +
    //                                           "', '" + attrs.getName(SUMO_ATTR_PERSONSPERHOUR) +
    //                                           "', '" + attrs.getName(SUMO_ATTR_CONTAINERSPERHOUR) +
    //                                           "', '" + attrs.getName(SUMO_ATTR_PERHOUR) +
    //                                           "' or '" + attrs.getName(SUMO_ATTR_PROB) +
    //                                           "' are given at most one of '" + attrs.getName(SUMO_ATTR_END) +
    //                                           "' and '" + attrs.getName(SUMO_ATTR_NUMBER) +
    //                                           "' are allowed in "
    //                                           + toString(tag) + " '" + id + "'.");
    //             }
    //         } else {
    //             if (!hasNumber) {
    //                 return handleVehicleError(hardFail, nullptr,
    //                                           "At least one of '" + attrs.getName(SUMO_ATTR_PERIOD) +
    //                                           "', '" + attrs.getName(SUMO_ATTR_VEHSPERHOUR) +
    //                                           "', '" + attrs.getName(SUMO_ATTR_PERSONSPERHOUR) +
    //                                           "', '" + attrs.getName(SUMO_ATTR_CONTAINERSPERHOUR) +
    //                                           "', '" + attrs.getName(SUMO_ATTR_PERHOUR) +
    //                                           "', '" + attrs.getName(SUMO_ATTR_PROB) +
    //                                           "', and '" + attrs.getName(SUMO_ATTR_NUMBER) +
    //                                           "' is needed in "
    //                                           + toString(tag) + " '" + id + "'.");
    //             }
    //         }
    //         // declare flow
    //         SUMOVehicleParameter* flowParameter = new SUMOVehicleParameter();
    //         // set tag
    //         flowParameter->tag = tag;
    //         // set id
    //         flowParameter->id = id;
    //         if (tag == SUMO_TAG_PERSONFLOW) {
    //             flowParameter->vtypeid = DEFAULT_PEDTYPE_ID;
    //         }
    //         if (tag == SUMO_TAG_CONTAINERFLOW) {
    //             flowParameter->vtypeid = DEFAULT_CONTAINERTYPE_ID;
    //         }
    //         // parse common vehicle attributes
    //         try {
    //             parseCommonAttributes(attrs, flowParameter, tag, allowInternalRoutes);
    //         } catch (ProcessError& attributeError) {
    //             // check if continue handling another vehicles or stop handling
    //             if (hardFail) {
    //                 throw ProcessError(attributeError.what());
    //             } else {
    //                 return nullptr;
    //             }
    //         }
    //         // parse period
    //         bool poissonFlow = false;
    //         if (hasPeriod) {
    //             bool ok = true;
    //             const std::string description = attrs.get<std::string>(SUMO_ATTR_PERIOD, id.c_str(), ok);
    //             const std::string distName = description.substr(0, description.find('('));
    //             if (distName == "exp") {
    //                 // declare rate
    //                 double rate = -1;
    //                 // parse rate
    //                 try {
    //                     rate = StringUtils::toDouble(description.substr(distName.size() + 1, description.size() - distName.size() - 2));
    //                 } catch (ProcessError& attributeError) {
    //                     // check if continue handling another vehicles or stop handling
    //                     if (hardFail) {
    //                         throw ProcessError(attributeError.what());
    //                     } else {
    //                         return nullptr;
    //                     }
    //                 }
    //                 if (rate <= 0) {
    //                     return handleVehicleError(hardFail, flowParameter, "Invalid rate parameter for exponentially distributed period in the definition of " + toString(tag) + " '" + id + "'.");
    //                 }
    //                 flowParameter->poissonRate = rate;
    //                 poissonFlow = true;
    //             } else {
    //                 flowParameter->repetitionOffset = attrs.getSUMOTimeReporting(SUMO_ATTR_PERIOD, id.c_str(), ok);
    //             }
    //             if (!ok) {
    //                 return handleVehicleError(hardFail, flowParameter);
    //             } else {
    //                 flowParameter->parametersSet |= VEHPARS_PERIOD_SET;
    //             }
    //         }
    //         // parse vehicle/person/container/etc per hour
    //         if (hasXPH) {
    //             bool ok = true;
    //             const double vph = attrs.get<double>(PERHOUR, id.c_str(), ok);
    //             if (!ok) {
    //                 return handleVehicleError(hardFail, flowParameter);
    //             } else if (vph <= 0) {
    //                 return handleVehicleError(hardFail, flowParameter, "Invalid repetition rate in the definition of " + toString(tag) + " '" + id + "'.");
    //             } else {
    //                 if (vph != 0) {
    //                     flowParameter->repetitionOffset = TIME2STEPS(3600. / vph);
    //                 }
    //                 flowParameter->parametersSet |= VEHPARS_VPH_SET;
    //             }
    //         }
    //         // parse probability
    //         if (hasProb) {
    //             bool ok = true;
    //             flowParameter->repetitionProbability = attrs.get<double>(SUMO_ATTR_PROB, id.c_str(), ok);
    //             if (!ok) {
    //                 return handleVehicleError(hardFail, flowParameter);
    //             } else if (flowParameter->repetitionProbability <= 0 || flowParameter->repetitionProbability > 1) {
    //                 return handleVehicleError(hardFail, flowParameter, "Invalid repetition probability in the definition of " + toString(tag) + " '" + id + "'.");
    //             } else {
    //                 flowParameter->parametersSet |= VEHPARS_PROB_SET;
    //             }
    //         }
    //         // set default begin
    //         flowParameter->depart = beginDefault;
    //         // parse begin
    //         if (hasBegin) {
    //             // first get begin
    //             bool ok = true;
    //             const std::string begin = attrs.get<std::string>(SUMO_ATTR_BEGIN, id.c_str(), ok);
    //             if (!ok) {
    //                 return handleVehicleError(hardFail, flowParameter);
    //             } else {
    //                 // parse begin
    //                 std::string errorMsg;
    //                 if (!SUMOVehicleParameter::parseDepart(begin, toString(tag), id, flowParameter->depart, flowParameter->departProcedure, errorMsg, "begin")) {
    //                     return handleVehicleError(hardFail, flowParameter, errorMsg);
    //                 }
    //             }
    //         }
    //         if (flowParameter->depart < 0) {
    //             return handleVehicleError(hardFail, flowParameter, "Negative begin time in the definition of " + toString(tag) + " '" + id + "'.");
    //         }
    //         // set default end
    //         flowParameter->repetitionEnd = endDefault;
    //         if (flowParameter->repetitionEnd < 0) {
    //             flowParameter->repetitionEnd = SUMOTime_MAX;
    //         }
    //         // parse end
    //         if (hasEnd) {
    //             bool ok = true;
    //             flowParameter->repetitionEnd = attrs.getSUMOTimeReporting(SUMO_ATTR_END, id.c_str(), ok);
    //             if (!ok) {
    //                 return handleVehicleError(hardFail, flowParameter);
    //             } else {
    //                 flowParameter->parametersSet |= VEHPARS_END_SET;
    //             }
    //         } else if (flowParameter->departProcedure == DepartDefinition::TRIGGERED) {
    //             if (!hasNumber) {
    //                 return handleVehicleError(hardFail, flowParameter, toString(tag) + " '" + id + "' with triggered begin must define 'number'.");
    //             } else {
    //                 flowParameter->repetitionEnd = flowParameter->depart;
    //             }
    //         } else if ((endDefault == SUMOTime_MAX || endDefault < 0) && (!hasNumber || (!hasProb && !hasPeriod && !hasXPH))) {
    //             WRITE_WARNINGF(TL("Undefined end for % '%', defaulting to 24hour duration."), toString(tag), id);
    //             flowParameter->repetitionEnd = flowParameter->depart + TIME2STEPS(24 * 3600);
    //         }
    //         if (flowParameter->repetitionEnd < flowParameter->depart) {
    //             std::string flow = toString(tag);
    //             flow[0] = (char)::toupper((char)flow[0]);
    //             return handleVehicleError(hardFail, flowParameter, flow + " '" + id + "' ends before its begin time.");
    //         }
    //         // parse number
    //         if (hasNumber) {
    //             bool ok = true;
    //             flowParameter->repetitionNumber = attrs.get<long long int>(SUMO_ATTR_NUMBER, id.c_str(), ok);
    //             if (!ok) {
    //                 return handleVehicleError(hardFail, flowParameter);
    //             } else {
    //                 flowParameter->parametersSet |= VEHPARS_NUMBER_SET;
    //                 if (flowParameter->repetitionNumber == 0) {
    //                     std::string flow = toString(tag);
    //                     flow[0] = (char)::toupper((char)flow[0]);
    //                     WRITE_WARNING(flow + " '" + id + "' has no instances; will skip it.");
    //                     flowParameter->repetitionEnd = flowParameter->depart;
    //                 } else {
    //                     if (flowParameter->repetitionNumber < 0) {
    //                         return handleVehicleError(hardFail, flowParameter, "Negative repetition number in the definition of " + toString(tag) + " '" + id + "'.");
    //                     }
    //                     if (flowParameter->repetitionOffset < 0 && !hasProb) {
    //                         if (poissonFlow) {
    //                             flowParameter->repetitionEnd = SUMOTime_MAX;
    //                         } else {
    //                             flowParameter->repetitionOffset = (flowParameter->repetitionEnd - flowParameter->depart) / flowParameter->repetitionNumber;
    //                         }
    //                     }
    //                 }
    //             }
    //         } else {
    //             // interpret repetitionNumber
    //             if (flowParameter->repetitionProbability > 0) {
    //                 flowParameter->repetitionNumber = std::numeric_limits<long long int>::max();
    //             } else {
    //                 if (flowParameter->repetitionOffset <= 0) {
    //                     if (poissonFlow) {
    //                         // number is random but flow has a fixed end time
    //                         flowParameter->repetitionNumber = std::numeric_limits<long long int>::max();
    //                     } else {
    //                         return handleVehicleError(hardFail, flowParameter, "Invalid repetition rate in the definition of " + toString(tag) + " '" + id + "'.");
    //                     }
    //                 } else {
    //                     if (flowParameter->repetitionEnd == SUMOTime_MAX) {
    //                         flowParameter->repetitionNumber = std::numeric_limits<long long int>::max();
    //                     } else {
    //                         const SUMOTime repLength = flowParameter->repetitionEnd - flowParameter->depart;
    //                         flowParameter->repetitionNumber = (long long int)ceil((double)repLength / (double)flowParameter->repetitionOffset);
    //                     }
    //                 }
    //             }
    //         }
    //         // all ok, then return flow parameter
    //         return flowParameter;
    //     } else {
    //         return handleVehicleError(hardFail, nullptr, toString(tag) + " cannot be created");
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_flow_attributes(tag: SumoXMLTag, attrs: &SUMOSAXAttributes, hardFail: bool, needID: bool, beginDefault: SUMOTime, endDefault: SUMOTime, allowInternalRoutes: bool) -> Ptr<SUMOVehicleParameter> {
        // first parse ID
        let id: String = if attrs.has_attribute(SUMO_ATTR_ID) { Self::parse_id(attrs, tag) } else { String::new() };
        // check if ID is valid
        if !needID || !id.is_empty() {
            if needID && !is_valid_vehicle_id(&id) {
                return Self::handle_vehicle_error(hardFail, Ptr::null(), format!("Invalid flow id '{}'.", id));
            }
            // declare flags
            let hasPeriod = attrs.has_attribute(SUMO_ATTR_PERIOD);
            let hasVPH = attrs.has_attribute(SUMO_ATTR_VEHSPERHOUR);
            let hasPPH = attrs.has_attribute(SUMO_ATTR_PERSONSPERHOUR);
            let hasCPH = attrs.has_attribute(SUMO_ATTR_CONTAINERSPERHOUR);
            let hasPH = attrs.has_attribute(SUMO_ATTR_PERHOUR);
            let hasXPH = hasVPH || hasPPH || hasCPH || hasPH;
            let hasProb = attrs.has_attribute(SUMO_ATTR_PROB);
            let hasNumber = attrs.has_attribute(SUMO_ATTR_NUMBER);
            let hasBegin = attrs.has_attribute(SUMO_ATTR_BEGIN);
            let hasEnd = attrs.has_attribute(SUMO_ATTR_END);
            let mut PERHOUR = SUMO_ATTR_PERHOUR;
            if hasVPH {
                PERHOUR = SUMO_ATTR_VEHSPERHOUR;
            }
            if hasPPH {
                PERHOUR = SUMO_ATTR_PERSONSPERHOUR;
            }
            if hasCPH {
                PERHOUR = SUMO_ATTR_CONTAINERSPERHOUR;
            }
            if hasXPH && !(hasVPH ^ hasPPH ^ hasCPH ^ hasPH) {
                return Self::handle_vehicle_error(
                    hardFail,
                    Ptr::null(),
                    format!(
                        "At most one of '{}', '{}', '{}' and '{}' has to be given in the definition of {} '{}'.",
                        attrs.get_name(SUMO_ATTR_PERHOUR),
                        attrs.get_name(SUMO_ATTR_VEHSPERHOUR),
                        attrs.get_name(SUMO_ATTR_PERSONSPERHOUR),
                        attrs.get_name(SUMO_ATTR_CONTAINERSPERHOUR),
                        Tags.get_string(tag),
                        id
                    ),
                );
            }
            if hasPeriod && hasXPH {
                return Self::handle_vehicle_error(
                    hardFail,
                    Ptr::null(),
                    format!(
                        "At most one of '{}' and '{}' has to be given in the definition of {} '{}'.",
                        attrs.get_name(SUMO_ATTR_PERIOD),
                        attrs.get_name(PERHOUR),
                        Tags.get_string(tag),
                        id
                    ),
                );
            }
            if hasPeriod && hasProb {
                return Self::handle_vehicle_error(
                    hardFail,
                    Ptr::null(),
                    format!(
                        "At most one of '{}' and '{}' has to be given in the definition of {} '{}'.",
                        attrs.get_name(SUMO_ATTR_PERIOD),
                        attrs.get_name(SUMO_ATTR_PROB),
                        Tags.get_string(tag),
                        id
                    ),
                );
            }
            if hasProb && hasXPH {
                return Self::handle_vehicle_error(
                    hardFail,
                    Ptr::null(),
                    format!(
                        "At most one of '{}' and '{}' has to be given in the definition of {} '{}'.",
                        attrs.get_name(SUMO_ATTR_PROB),
                        attrs.get_name(PERHOUR),
                        Tags.get_string(tag),
                        id
                    ),
                );
            }
            if hasPeriod || hasXPH || hasProb {
                if hasEnd && hasNumber {
                    return Self::handle_vehicle_error(
                        hardFail,
                        Ptr::null(),
                        format!(
                            "If '{}', '{}', '{}', '{}', '{}' or '{}' are given at most one of '{}' and '{}' are allowed in {} '{}'.",
                            attrs.get_name(SUMO_ATTR_PERIOD),
                            attrs.get_name(SUMO_ATTR_VEHSPERHOUR),
                            attrs.get_name(SUMO_ATTR_PERSONSPERHOUR),
                            attrs.get_name(SUMO_ATTR_CONTAINERSPERHOUR),
                            attrs.get_name(SUMO_ATTR_PERHOUR),
                            attrs.get_name(SUMO_ATTR_PROB),
                            attrs.get_name(SUMO_ATTR_END),
                            attrs.get_name(SUMO_ATTR_NUMBER),
                            Tags.get_string(tag),
                            id
                        ),
                    );
                }
            } else if !hasNumber {
                return Self::handle_vehicle_error(
                    hardFail,
                    Ptr::null(),
                    format!(
                        "At least one of '{}', '{}', '{}', '{}', '{}', '{}', and '{}' is needed in {} '{}'.",
                        attrs.get_name(SUMO_ATTR_PERIOD),
                        attrs.get_name(SUMO_ATTR_VEHSPERHOUR),
                        attrs.get_name(SUMO_ATTR_PERSONSPERHOUR),
                        attrs.get_name(SUMO_ATTR_CONTAINERSPERHOUR),
                        attrs.get_name(SUMO_ATTR_PERHOUR),
                        attrs.get_name(SUMO_ATTR_PROB),
                        attrs.get_name(SUMO_ATTR_NUMBER),
                        Tags.get_string(tag),
                        id
                    ),
                );
            }
            // declare flow
            let flowParameter: Ptr<SUMOVehicleParameter> = Ptr::from_raw(Box::into_raw(Box::new(SUMOVehicleParameter::new())));
            let fp = flowParameter.m();
            // set tag
            fp.tag = tag;
            // set id
            fp.id = id.clone();
            if tag == SUMO_TAG_PERSONFLOW {
                fp.vtypeid = DEFAULT_PEDTYPE_ID.to_string();
            }
            if tag == SUMO_TAG_CONTAINERFLOW {
                fp.vtypeid = DEFAULT_CONTAINERTYPE_ID.to_string();
            }
            // parse common vehicle attributes
            if let Err(attributeError) = catch_process_error(|| Self::parse_common_attributes(attrs, flowParameter, tag, allowInternalRoutes)) {
                // check if continue handling another vehicles or stop handling
                if hardFail {
                    throw_process_error(attributeError.to_string());
                } else {
                    return Ptr::null();
                }
            }
            // parse period
            let mut poissonFlow = false;
            if hasPeriod {
                let mut ok = true;
                let description: String = attrs.get::<String>(SUMO_ATTR_PERIOD, &id, &mut ok, true);
                let distName: &str = match description.find('(') {
                    Some(i) => &description[..i],
                    None => &description[..],
                };
                if distName == "exp" {
                    // declare rate
                    let mut rate: f64 = -1.0;
                    // parse rate
                    match to_double(cpp_substr(&description, distName.len() + 1, description.len().wrapping_sub(distName.len()).wrapping_sub(2))) {
                        Ok(v) => rate = v,
                        Err(attributeError) => {
                            // check if continue handling another vehicles or stop handling
                            if hardFail {
                                throw_process_error(attributeError.to_string());
                            } else {
                                return Ptr::null();
                            }
                        }
                    }
                    if rate <= 0.0 {
                        return Self::handle_vehicle_error(
                            hardFail,
                            flowParameter,
                            format!("Invalid rate parameter for exponentially distributed period in the definition of {} '{}'.", Tags.get_string(tag), id),
                        );
                    }
                    fp.poissonRate = rate;
                    poissonFlow = true;
                } else {
                    fp.repetitionOffset = attrs.get_sumo_time_reporting(SUMO_ATTR_PERIOD, &id, &mut ok, true);
                }
                if !ok {
                    return Self::handle_vehicle_error(hardFail, flowParameter, String::new());
                } else {
                    fp.parametersSet |= VEHPARS_PERIOD_SET;
                }
            }
            // parse vehicle/person/container/etc per hour
            if hasXPH {
                let mut ok = true;
                let vph: f64 = attrs.get::<f64>(PERHOUR, &id, &mut ok, true);
                if !ok {
                    return Self::handle_vehicle_error(hardFail, flowParameter, String::new());
                } else if vph <= 0.0 {
                    return Self::handle_vehicle_error(
                        hardFail,
                        flowParameter,
                        format!("Invalid repetition rate in the definition of {} '{}'.", Tags.get_string(tag), id),
                    );
                } else {
                    if vph != 0.0 {
                        fp.repetitionOffset = time2steps(3600.0 / vph);
                    }
                    fp.parametersSet |= VEHPARS_VPH_SET;
                }
            }
            // parse probability
            if hasProb {
                let mut ok = true;
                fp.repetitionProbability = attrs.get::<f64>(SUMO_ATTR_PROB, &id, &mut ok, true);
                if !ok {
                    return Self::handle_vehicle_error(hardFail, flowParameter, String::new());
                } else if fp.repetitionProbability <= 0.0 || fp.repetitionProbability > 1.0 {
                    return Self::handle_vehicle_error(
                        hardFail,
                        flowParameter,
                        format!("Invalid repetition probability in the definition of {} '{}'.", Tags.get_string(tag), id),
                    );
                } else {
                    fp.parametersSet |= VEHPARS_PROB_SET;
                }
            }
            // set default begin
            fp.depart = beginDefault;
            // parse begin
            if hasBegin {
                // first get begin
                let mut ok = true;
                let begin: String = attrs.get::<String>(SUMO_ATTR_BEGIN, &id, &mut ok, true);
                if !ok {
                    return Self::handle_vehicle_error(hardFail, flowParameter, String::new());
                } else {
                    // parse begin
                    let mut errorMsg = String::new();
                    if !SUMOVehicleParameter::parse_depart(&begin, Tags.get_string(tag), &id, &mut fp.depart, &mut fp.departProcedure, &mut errorMsg, "begin") {
                        return Self::handle_vehicle_error(hardFail, flowParameter, errorMsg);
                    }
                }
            }
            if fp.depart < 0 {
                return Self::handle_vehicle_error(hardFail, flowParameter, format!("Negative begin time in the definition of {} '{}'.", Tags.get_string(tag), id));
            }
            // set default end
            fp.repetitionEnd = endDefault;
            if fp.repetitionEnd < 0 {
                fp.repetitionEnd = SUMOTIME_MAX;
            }
            // parse end
            if hasEnd {
                let mut ok = true;
                fp.repetitionEnd = attrs.get_sumo_time_reporting(SUMO_ATTR_END, &id, &mut ok, true);
                if !ok {
                    return Self::handle_vehicle_error(hardFail, flowParameter, String::new());
                } else {
                    fp.parametersSet |= VEHPARS_END_SET;
                }
            } else if matches!(fp.departProcedure, DepartDefinition::TRIGGERED) {
                if !hasNumber {
                    return Self::handle_vehicle_error(
                        hardFail,
                        flowParameter,
                        format!("{} '{}' with triggered begin must define 'number'.", Tags.get_string(tag), id),
                    );
                } else {
                    fp.repetitionEnd = fp.depart;
                }
            } else if (endDefault == SUMOTIME_MAX || endDefault < 0) && (!hasNumber || (!hasProb && !hasPeriod && !hasXPH)) {
                write_warning(&format!("Undefined end for {} '{}', defaulting to 24hour duration.", Tags.get_string(tag), id));
                fp.repetitionEnd = fp.depart + time2steps(24.0 * 3600.0);
            }
            if fp.repetitionEnd < fp.depart {
                let flow = upper_first(Tags.get_string(tag));
                return Self::handle_vehicle_error(hardFail, flowParameter, format!("{} '{}' ends before its begin time.", flow, id));
            }
            // parse number
            if hasNumber {
                let mut ok = true;
                fp.repetitionNumber = attrs.get::<i64>(SUMO_ATTR_NUMBER, &id, &mut ok, true);
                if !ok {
                    return Self::handle_vehicle_error(hardFail, flowParameter, String::new());
                } else {
                    fp.parametersSet |= VEHPARS_NUMBER_SET;
                    if fp.repetitionNumber == 0 {
                        let flow = upper_first(Tags.get_string(tag));
                        write_warning(&format!("{} '{}' has no instances; will skip it.", flow, id));
                        fp.repetitionEnd = fp.depart;
                    } else {
                        if fp.repetitionNumber < 0 {
                            return Self::handle_vehicle_error(
                                hardFail,
                                flowParameter,
                                format!("Negative repetition number in the definition of {} '{}'.", Tags.get_string(tag), id),
                            );
                        }
                        if fp.repetitionOffset < 0 && !hasProb {
                            if poissonFlow {
                                fp.repetitionEnd = SUMOTIME_MAX;
                            } else {
                                fp.repetitionOffset = (fp.repetitionEnd - fp.depart) / fp.repetitionNumber;
                            }
                        }
                    }
                }
            } else {
                // interpret repetitionNumber
                if fp.repetitionProbability > 0.0 {
                    fp.repetitionNumber = i64::MAX;
                } else {
                    if fp.repetitionOffset <= 0 {
                        if poissonFlow {
                            // number is random but flow has a fixed end time
                            fp.repetitionNumber = i64::MAX;
                        } else {
                            return Self::handle_vehicle_error(
                                hardFail,
                                flowParameter,
                                format!("Invalid repetition rate in the definition of {} '{}'.", Tags.get_string(tag), id),
                            );
                        }
                    } else {
                        if fp.repetitionEnd == SUMOTIME_MAX {
                            fp.repetitionNumber = i64::MAX;
                        } else {
                            let repLength = fp.repetitionEnd - fp.depart;
                            fp.repetitionNumber = ((repLength as f64) / (fp.repetitionOffset as f64)).ceil() as i64;
                        }
                    }
                }
            }
            // all ok, then return flow parameter
            return flowParameter;
        } else {
            return Self::handle_vehicle_error(hardFail, Ptr::null(), format!("{} cannot be created", Tags.get_string(tag)));
        }
    }

    /// C++: `SUMOVehicleParameter * SUMOVehicleParserHelper::parseVehicleAttributes(int, const SUMOSAXAttributes &, const bool, const bool, const bool, const bool)` [static]
    /// Parses a vehicle's attributes
    // SUMOVehicleParameter*
    // SUMOVehicleParserHelper::parseVehicleAttributes(int element, const SUMOSAXAttributes& attrs, const bool hardFail, const bool optionalID, const bool skipDepart, const bool allowInternalRoutes) {
    //     // declare vehicle ID
    //     std::string id;
    //     // for certain vehicles, ID can be optional
    //     if (optionalID) {
    //         bool ok = true;
    //         id = attrs.getOpt<std::string>(SUMO_ATTR_ID, nullptr, ok, "");
    //         if (!ok) {
    //             return handleVehicleError(hardFail, nullptr);
    //         }
    //     } else {
    //         // parse ID
    //         id = parseID(attrs, (SumoXMLTag)element);
    //     }
    //     // only continue if id is valid, or if is optional
    //     if (optionalID || !id.empty()) {
    //         // declare vehicle parameter
    //         SUMOVehicleParameter* vehicleParameter = new SUMOVehicleParameter();
    //         vehicleParameter->id = id;
    //         if (element == SUMO_TAG_PERSON) {
    //             vehicleParameter->vtypeid = DEFAULT_PEDTYPE_ID;
    //         } else if (element == SUMO_TAG_CONTAINER) {
    //             vehicleParameter->vtypeid = DEFAULT_CONTAINERTYPE_ID;
    //         }
    //         // parse common attributes
    //         try {
    //             parseCommonAttributes(attrs, vehicleParameter, (SumoXMLTag)element, allowInternalRoutes);
    //         } catch (ProcessError& attributeError) {
    //             // check if continue handling another vehicles or stop handling
    //             if (hardFail) {
    //                 throw ProcessError(attributeError.what());
    //             } else {
    //                 return nullptr;
    //             }
    //         }
    //         // check depart
    //         if (!skipDepart) {
    //             bool ok = true;
    //             const std::string helper = attrs.get<std::string>(SUMO_ATTR_DEPART, vehicleParameter->id.c_str(), ok);
    //             if (!ok) {
    //                 return handleVehicleError(hardFail, vehicleParameter);
    //             }
    //             // now parse depart
    //             std::string departErrorMsg;
    //             if (!SUMOVehicleParameter::parseDepart(helper, "vehicle", vehicleParameter->id, vehicleParameter->depart, vehicleParameter->departProcedure, departErrorMsg)) {
    //                 return handleVehicleError(hardFail, vehicleParameter, departErrorMsg);
    //             }
    //         }
    //         // set tag
    //         vehicleParameter->tag = (SumoXMLTag)element;
    //         // all ok, then return vehicleParameter
    //         return vehicleParameter;
    //     } else {
    //         return handleVehicleError(hardFail, nullptr, toString((SumoXMLTag)element) + " cannot be created");
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_vehicle_attributes(element: i32, attrs: &SUMOSAXAttributes, hardFail: bool, optionalID: bool, skipDepart: bool, allowInternalRoutes: bool) -> Ptr<SUMOVehicleParameter> {
        // declare vehicle ID
        let id: String;
        // for certain vehicles, ID can be optional
        if optionalID {
            let mut ok = true;
            id = attrs.get_opt::<String>(SUMO_ATTR_ID, "", &mut ok, String::new(), true);
            if !ok {
                return Self::handle_vehicle_error(hardFail, Ptr::null(), String::new());
            }
        } else {
            // parse ID
            id = Self::parse_id(attrs, element);
        }
        // only continue if id is valid, or if is optional
        if optionalID || !id.is_empty() {
            // declare vehicle parameter
            let vehicleParameter: Ptr<SUMOVehicleParameter> = Ptr::from_raw(Box::into_raw(Box::new(SUMOVehicleParameter::new())));
            let vp = vehicleParameter.m();
            vp.id = id.clone();
            if element == SUMO_TAG_PERSON {
                vp.vtypeid = DEFAULT_PEDTYPE_ID.to_string();
            } else if element == SUMO_TAG_CONTAINER {
                vp.vtypeid = DEFAULT_CONTAINERTYPE_ID.to_string();
            }
            // parse common attributes
            if let Err(attributeError) = catch_process_error(|| Self::parse_common_attributes(attrs, vehicleParameter, element, allowInternalRoutes)) {
                // check if continue handling another vehicles or stop handling
                if hardFail {
                    throw_process_error(attributeError.to_string());
                } else {
                    return Ptr::null();
                }
            }
            // check depart
            if !skipDepart {
                let mut ok = true;
                let helper: String = attrs.get::<String>(SUMO_ATTR_DEPART, &vp.id, &mut ok, true);
                if !ok {
                    return Self::handle_vehicle_error(hardFail, vehicleParameter, String::new());
                }
                // now parse depart
                let mut departErrorMsg = String::new();
                if !SUMOVehicleParameter::parse_depart(&helper, "vehicle", &vp.id, &mut vp.depart, &mut vp.departProcedure, &mut departErrorMsg, "departure") {
                    return Self::handle_vehicle_error(hardFail, vehicleParameter, departErrorMsg);
                }
            }
            // set tag
            vp.tag = element;
            // all ok, then return vehicleParameter
            return vehicleParameter;
        } else {
            return Self::handle_vehicle_error(hardFail, Ptr::null(), format!("{} cannot be created", Tags.get_string(element)));
        }
    }

    /// C++: `SUMOVTypeParameter * SUMOVehicleParserHelper::beginVTypeParsing(const SUMOSAXAttributes &, const bool, const std::string &, const SUMOVTypeParameter *const)` [static]
    /// Starts to parse a vehicle type
    // SUMOVTypeParameter*
    // SUMOVehicleParserHelper::beginVTypeParsing(const SUMOSAXAttributes& attrs, const bool hardFail, const std::string& file, const SUMOVTypeParameter* const base) {
    //     const std::string id = (attrs.hasAttribute(SUMO_ATTR_ID) || base == nullptr) ? parseID(attrs, SUMO_TAG_VTYPE) : "";
    //     if (!id.empty()) {
    //         // create vType
    //         SUMOVehicleClass vClass = base == nullptr ? SVC_PASSENGER : base->vehicleClass;
    //         if (attrs.hasAttribute(SUMO_ATTR_VCLASS)) {
    //             vClass = parseVehicleClass(attrs, id);
    //         }
    //         SUMOVTypeParameter* vType;
    //         if (base == nullptr) {
    //             vType = new SUMOVTypeParameter(id, vClass);
    //         } else {
    //             vType = new SUMOVTypeParameter(*base);
    //             vType->id = id;
    //             vType->vehicleClass = vClass;
    //         }
    //         // parse attributes
    //         if (attrs.hasAttribute(SUMO_ATTR_VCLASS)) {
    //             vType->parametersSet |= VTYPEPARS_VEHICLECLASS_SET;
    //         }
    //         if (attrs.hasAttribute(SUMO_ATTR_LENGTH)) {
    //             bool ok = true;
    //             const double length = attrs.get<double>(SUMO_ATTR_LENGTH, vType->id.c_str(), ok);
    //             if (!ok) {
    //                 return handleVehicleTypeError(hardFail, vType);
    //             } else if (length <= 0) {
    //                 return handleVehicleTypeError(hardFail, vType, toString(SUMO_ATTR_LENGTH) + " must be greater than 0");
    //             } else {
    //                 vType->length = length;
    //                 vType->parametersSet |= VTYPEPARS_LENGTH_SET;
    //             }
    //         }
    //         if (attrs.hasAttribute(SUMO_ATTR_MINGAP)) {
    //             bool ok = true;
    //             const double minGap = attrs.get<double>(SUMO_ATTR_MINGAP, vType->id.c_str(), ok);
    //             if (!ok) {
    //                 return handleVehicleTypeError(hardFail, vType);
    //             } else if (minGap < 0) {
    //                 return handleVehicleTypeError(hardFail, vType, toString(SUMO_ATTR_MINGAP) + " must be equal or greater than 0");
    //             } else {
    //                 vType->minGap = minGap;
    //                 vType->parametersSet |= VTYPEPARS_MINGAP_SET;
    //             }
    //         }
    //         if (attrs.hasAttribute(SUMO_ATTR_MAXSPEED)) {
    //             bool ok = true;
    //             const double maxSpeed = attrs.get<double>(SUMO_ATTR_MAXSPEED, vType->id.c_str(), ok);
    //             if (!ok) {
    //                 return handleVehicleTypeError(hardFail, vType);
    //             } else if (maxSpeed <= 0) {
    //                 return handleVehicleTypeError(hardFail, vType, toString(SUMO_ATTR_MAXSPEED) + " must be greater than 0");
    //             } else {
    //                 vType->maxSpeed = maxSpeed;
    //                 vType->parametersSet |= VTYPEPARS_MAXSPEED_SET;
    //             }
    //         }
    //         if (attrs.hasAttribute(SUMO_ATTR_DESIRED_MAXSPEED)) {
    //             bool ok = true;
    //             const double desiredMaxSpeed = attrs.get<double>(SUMO_ATTR_DESIRED_MAXSPEED, vType->id.c_str(), ok);
    //             if (!ok) {
    //                 return handleVehicleTypeError(hardFail, vType);
    //             } else if (desiredMaxSpeed <= 0) {
    //                 return handleVehicleTypeError(hardFail, vType, toString(SUMO_ATTR_DESIRED_MAXSPEED) + " must be greater than 0");
    //             } else {
    //                 vType->desiredMaxSpeed = desiredMaxSpeed;
    //                 vType->parametersSet |= VTYPEPARS_DESIRED_MAXSPEED_SET;
    //             }
    //         } else if (attrs.hasAttribute(SUMO_ATTR_MAXSPEED)) {
    //             if (vClass == SVC_PEDESTRIAN) {
    //                 // backward compatibility because pedestrian maxSpeed was subject to speedFactor up to 1.14.1
    //                 vType->desiredMaxSpeed = vType->maxSpeed;;
    //                 vType->maxSpeed = MAX2(vType->maxSpeed, SUMOVTypeParameter::VClassDefaultValues(vClass).maxSpeed);
    //             } else if (vClass == SVC_BICYCLE) {
    //                 // backward compatibility because default desired speed did not exist up to 1.14.1
    //                 vType->desiredMaxSpeed = MAX2(vType->maxSpeed, vType->desiredMaxSpeed);
    //             }
    //         }
    //
    //         if (attrs.hasAttribute(SUMO_ATTR_SPEEDFACTOR)) {
    //             bool ok = true;
    //             vType->speedFactor.parse(attrs.get<std::string>(SUMO_ATTR_SPEEDFACTOR, vType->id.c_str(), ok), hardFail);
    //             if (!ok) {
    //                 return handleVehicleTypeError(hardFail, vType);
    //             } else {
    //                 vType->parametersSet |= VTYPEPARS_SPEEDFACTOR_SET;
    //             }
    //         }
    //         if (attrs.hasAttribute(SUMO_ATTR_SPEEDDEV)) {
    //             bool ok = true;
    //             const double speedDev = attrs.get<double>(SUMO_ATTR_SPEEDDEV, vType->id.c_str(), ok);
    //             if (!ok) {
    //                 return handleVehicleTypeError(hardFail, vType);
    //             } else if (speedDev < 0) {
    //                 return handleVehicleTypeError(hardFail, vType, toString(SUMO_ATTR_SPEEDDEV) + " must be equal or greater than 0");
    //             } else {
    //                 vType->speedFactor.setParameter(1, speedDev);
    //                 vType->parametersSet |= VTYPEPARS_SPEEDFACTOR_SET;
    //             }
    //         }
    //         // validate speed distribution
    //         const std::string& error = vType->speedFactor.isValid();
    //         if (error != "") {
    //             return handleVehicleTypeError(hardFail, vType, "Invalid speed distribution when parsing vType '" + vType->id + "' (" + error + ")");
    //         }
    //         if (attrs.hasAttribute(SUMO_ATTR_ACTIONSTEPLENGTH)) {
    //             bool ok = true;
    //             const double actionStepLengthSecs = attrs.get<double>(SUMO_ATTR_ACTIONSTEPLENGTH, vType->id.c_str(), ok);
    //             if (!ok) {
    //                 return handleVehicleTypeError(hardFail, vType);
    //             } else {
    //                 // processActionStepLength(...) function includes warnings
    //                 vType->actionStepLength = processActionStepLength(actionStepLengthSecs);
    //                 vType->parametersSet |= VTYPEPARS_ACTIONSTEPLENGTH_SET;
    //             }
    //         }
    //         if (attrs.hasAttribute(SUMO_ATTR_EMISSIONCLASS)) {
    //             bool ok = true;
    //             const std::string parsedEmissionClass = attrs.getOpt<std::string>(SUMO_ATTR_EMISSIONCLASS, id.c_str(), ok, "");
    //             // check if emission class is correct
    //             try {
    //                 vType->emissionClass = PollutantsInterface::getClassByName(parsedEmissionClass);
    //                 vType->parametersSet |= VTYPEPARS_EMISSIONCLASS_SET;
    //             } catch (...) {
    //                 return handleVehicleTypeError(hardFail, vType, toString(SUMO_ATTR_EMISSIONCLASS) + " with name '" + parsedEmissionClass + "' doesn't exist.");
    //             }
    //         }
    //         if (attrs.hasAttribute(SUMO_ATTR_MASS)) {
    //             bool ok = true;
    //             const double mass = attrs.get<double>(SUMO_ATTR_MASS, vType->id.c_str(), ok);
    //             if (!ok) {
    //                 return handleVehicleTypeError(hardFail, vType);
    //             } else if (mass < 0) {
    //                 return handleVehicleTypeError(hardFail, vType, toString(SUMO_ATTR_MASS) + " must be equal or greater than 0");
    //             } else {
    //                 vType->mass = mass;
    //                 vType->parametersSet |= VTYPEPARS_MASS_SET;
    //             }
    //         }
    //         if (attrs.hasAttribute(SUMO_ATTR_IMPATIENCE)) {
    //             bool ok = true;
    //             const std::string impatienceStr = attrs.get<std::string>(SUMO_ATTR_IMPATIENCE, vType->id.c_str(), ok);
    //             if (!ok) {
    //                 return handleVehicleTypeError(hardFail, vType);
    //             } else if (impatienceStr == "off") {
    //                 vType->impatience = -std::numeric_limits<double>::max();
    //             } else {
    //                 const double impatienceDouble = attrs.get<double>(SUMO_ATTR_IMPATIENCE, vType->id.c_str(), ok);
    //                 if (!ok) {
    //                     return handleVehicleTypeError(hardFail, vType);
    //                 } else {
    //                     vType->impatience = impatienceDouble;
    //                     vType->parametersSet |= VTYPEPARS_IMPATIENCE_SET;
    //                 }
    //             }
    //         }
    //         if (attrs.hasAttribute(SUMO_ATTR_WIDTH)) {
    //             bool ok = true;
    //             const double width = attrs.get<double>(SUMO_ATTR_WIDTH, vType->id.c_str(), ok);
    //             if (!ok) {
    //                 return handleVehicleTypeError(hardFail, vType);
    //             } else if (width <= 0) {
    //                 return handleVehicleTypeError(hardFail, vType, toString(SUMO_ATTR_WIDTH) + " must be greater than 0");
    //             } else {
    //                 vType->width = width;
    //                 vType->parametersSet |= VTYPEPARS_WIDTH_SET;
    //                 if (vClass == SVC_PEDESTRIAN
    //                         && OptionsCont::getOptions().exists("pedestrian.striping.stripe-width")
    //                         && OptionsCont::getOptions().getString("pedestrian.model") == "striping"
    //                         && OptionsCont::getOptions().getFloat("pedestrian.striping.stripe-width") < vType->width) {
    //                     WRITE_WARNINGF(TL("Pedestrian vType '%' width % is larger than pedestrian.striping.stripe-width and this may cause collisions with vehicles."), id, vType->width);
    //                 }
    //             }
    //         }
    //         if (attrs.hasAttribute(SUMO_ATTR_HEIGHT)) {
    //             bool ok = true;
    //             const double height = attrs.get<double>(SUMO_ATTR_HEIGHT, vType->id.c_str(), ok);
    //             if (!ok) {
    //                 return handleVehicleTypeError(hardFail, vType);
    //             } else if (height < 0) {
    //                 return handleVehicleTypeError(hardFail, vType, toString(SUMO_ATTR_HEIGHT) + " must be equal or greater than 0");
    //             } else {
    //                 vType->height = height;
    //                 vType->parametersSet |= VTYPEPARS_HEIGHT_SET;
    //             }
    //         }
    //         if (attrs.hasAttribute(SUMO_ATTR_GUISHAPE)) {
    //             vType->shape = parseGuiShape(attrs, vType->id);
    //             if (vType->shape != SUMOVehicleShape::UNKNOWN) {
    //                 vType->parametersSet |= VTYPEPARS_SHAPE_SET;
    //             }
    //         }
    //         if (attrs.hasAttribute(SUMO_ATTR_OSGFILE)) {
    //             bool ok = true;
    //             const std::string osgFile = attrs.get<std::string>(SUMO_ATTR_OSGFILE, vType->id.c_str(), ok);
    //             if (!ok) {
    //                 return handleVehicleTypeError(hardFail, vType);
    //             } else {
    //                 vType->osgFile = osgFile;
    //                 vType->parametersSet |= VTYPEPARS_OSGFILE_SET;
    //             }
    //         }
    //         if (attrs.hasAttribute(SUMO_ATTR_IMGFILE)) {
    //             bool ok = true;
    //             std::string imgFile = attrs.get<std::string>(SUMO_ATTR_IMGFILE, vType->id.c_str(), ok);
    //             if (!ok) {
    //                 return handleVehicleTypeError(hardFail, vType);
    //             } else {
    //                 // check relative path
    //                 if ((imgFile != "") && !FileHelpers::isAbsolute(imgFile)) {
    //                     imgFile = FileHelpers::getConfigurationRelative(file, imgFile);
    //                 }
    //                 vType->imgFile = imgFile;
    //                 vType->parametersSet |= VTYPEPARS_IMGFILE_SET;
    //             }
    //         }
    //         if (attrs.hasAttribute(SUMO_ATTR_COLOR)) {
    //             bool ok = true;
    //             const RGBColor color = attrs.get<RGBColor>(SUMO_ATTR_COLOR, vType->id.c_str(), ok);
    //             if (!ok) {
    //                 return handleVehicleTypeError(hardFail, vType);
    //             } else {
    //                 vType->color = color;
    //                 vType->parametersSet |= VTYPEPARS_COLOR_SET;
    //             }
    //         } else {
    //             vType->color = RGBColor::YELLOW;
    //         }
    //         if (attrs.hasAttribute(SUMO_ATTR_PROB)) {
    //             bool ok = true;
    //             const double defaultProbability = attrs.get<double>(SUMO_ATTR_PROB, vType->id.c_str(), ok);
    //             if (!ok) {
    //                 return handleVehicleTypeError(hardFail, vType);
    //             } else if (defaultProbability < 0) {
    //                 return handleVehicleTypeError(hardFail, vType, toString(SUMO_ATTR_PROB) + " must be equal or greater than 0");
    //             } else {
    //                 vType->defaultProbability = defaultProbability;
    //                 vType->parametersSet |= VTYPEPARS_PROBABILITY_SET;
    //             }
    //         }
    //         if (attrs.hasAttribute(SUMO_ATTR_LANE_CHANGE_MODEL)) {
    //             bool ok = true;
    //             std::string lcmS = attrs.get<std::string>(SUMO_ATTR_LANE_CHANGE_MODEL, vType->id.c_str(), ok);
    //             if (!ok) {
    //                 return handleVehicleTypeError(hardFail, vType);
    //             } else if (lcmS == "JE2013") {
    //                 WRITE_WARNING(TL("Lane change model 'JE2013' is deprecated. Using default model instead."));
    //                 lcmS = "default";
    //             }
    //             if (SUMOXMLDefinitions::LaneChangeModels.hasString(lcmS)) {
    //                 vType->lcModel = SUMOXMLDefinitions::LaneChangeModels.get(lcmS);
    //                 vType->parametersSet |= VTYPEPARS_LANE_CHANGE_MODEL_SET;
    //             } else {
    //                 return handleVehicleTypeError(hardFail, vType, "Unknown lane change model '" + lcmS + "' when parsing vType '" + vType->id + "'");
    //             }
    //         }
    //         if (attrs.hasAttribute(SUMO_ATTR_CAR_FOLLOW_MODEL)) {
    //             bool ok = true;
    //             const std::string cfmValue = attrs.get<std::string>(SUMO_ATTR_CAR_FOLLOW_MODEL, vType->id.c_str(), ok);
    //             if (!ok) {
    //                 return handleVehicleTypeError(hardFail, vType);
    //             } else if (SUMOXMLDefinitions::CarFollowModels.hasString(cfmValue)) {
    //                 vType->cfModel = SUMOXMLDefinitions::CarFollowModels.get(cfmValue);
    //                 vType->parametersSet |= VTYPEPARS_CAR_FOLLOW_MODEL;
    //             } else {
    //                 return handleVehicleTypeError(hardFail, vType, "Unknown car following model '" + cfmValue + "' when parsing vType '" + vType->id + "'");
    //             }
    //         }
    //         if (attrs.hasAttribute(SUMO_ATTR_PERSON_CAPACITY)) {
    //             bool ok = true;
    //             const int personCapacity = attrs.get<int>(SUMO_ATTR_PERSON_CAPACITY, vType->id.c_str(), ok);
    //             if (!ok) {
    //                 return handleVehicleTypeError(hardFail, vType);
    //             } else if (personCapacity < 0) {
    //                 return handleVehicleTypeError(hardFail, vType, toString(SUMO_ATTR_PERSON_CAPACITY) + " must be equal or greater than 0");
    //             } else {
    //                 vType->personCapacity = personCapacity;
    //                 vType->parametersSet |= VTYPEPARS_PERSON_CAPACITY;
    //             }
    //         }
    //         if (attrs.hasAttribute(SUMO_ATTR_CONTAINER_CAPACITY)) {
    //             bool ok = true;
    //             const int containerCapacity = attrs.get<int>(SUMO_ATTR_CONTAINER_CAPACITY, vType->id.c_str(), ok);
    //             if (!ok) {
    //                 return handleVehicleTypeError(hardFail, vType);
    //             } else if (containerCapacity < 0) {
    //                 return handleVehicleTypeError(hardFail, vType, toString(SUMO_ATTR_CONTAINER_CAPACITY) + " must be equal or greater than 0");
    //             } else {
    //                 vType->containerCapacity = containerCapacity;
    //                 vType->parametersSet |= VTYPEPARS_CONTAINER_CAPACITY;
    //             }
    //         }
    //         if (attrs.hasAttribute(SUMO_ATTR_BOARDING_DURATION)) {
    //             bool ok = true;
    //             const SUMOTime boardingDuration = attrs.getSUMOTimeReporting(SUMO_ATTR_BOARDING_DURATION, vType->id.c_str(), ok);
    //             if (!ok) {
    //                 return handleVehicleTypeError(hardFail, vType);
    //             } else if (boardingDuration < 0) {
    //                 return handleVehicleTypeError(hardFail, vType, toString(SUMO_ATTR_BOARDING_DURATION) + " must be equal or greater than 0");
    //             } else {
    //                 vType->boardingDuration = boardingDuration;
    //                 vType->parametersSet |= VTYPEPARS_BOARDING_DURATION;
    //             }
    //         }
    //         if (attrs.hasAttribute(SUMO_ATTR_LOADING_DURATION)) {
    //             bool ok = true;
    //             const SUMOTime loadingDuration = attrs.getSUMOTimeReporting(SUMO_ATTR_LOADING_DURATION, vType->id.c_str(), ok);
    //             if (!ok) {
    //                 return handleVehicleTypeError(hardFail, vType);
    //             } else if (loadingDuration < 0) {
    //                 return handleVehicleTypeError(hardFail, vType, toString(SUMO_ATTR_LOADING_DURATION) + " must be equal or greater than 0");
    //             } else {
    //                 vType->loadingDuration = loadingDuration;
    //                 vType->parametersSet |= VTYPEPARS_LOADING_DURATION;
    //             }
    //         }
    //         if (attrs.hasAttribute(SUMO_ATTR_SCALE)) {
    //             bool ok = true;
    //             const double scale = attrs.get<double>(SUMO_ATTR_SCALE, id.c_str(), ok);
    //             if (!ok) {
    //                 return handleVehicleTypeError(hardFail, vType);
    //             } else if (scale < 0) {
    //                 return handleVehicleTypeError(hardFail, vType, toString(SUMO_ATTR_SCALE) + " may be not be negative");
    //             } else {
    //                 vType->scale = scale;
    //                 vType->parametersSet |= VTYPEPARS_SCALE_SET;
    //             }
    //         }
    //         if (attrs.hasAttribute(SUMO_ATTR_TIME_TO_TELEPORT)) {
    //             bool ok = true;
    //             const SUMOTime ttt = attrs.getSUMOTimeReporting(SUMO_ATTR_TIME_TO_TELEPORT, vType->id.c_str(), ok);
    //             if (!ok) {
    //                 return handleVehicleTypeError(hardFail, vType);
    //             } else {
    //                 vType->timeToTeleport = ttt;
    //                 vType->parametersSet |= VTYPEPARS_TTT_SET;
    //             }
    //         }
    //         if (attrs.hasAttribute(SUMO_ATTR_TIME_TO_TELEPORT_BIDI)) {
    //             bool ok = true;
    //             const SUMOTime tttb = attrs.getSUMOTimeReporting(SUMO_ATTR_TIME_TO_TELEPORT_BIDI, vType->id.c_str(), ok);
    //             if (!ok) {
    //                 return handleVehicleTypeError(hardFail, vType);
    //             } else {
    //                 vType->timeToTeleportBidi = tttb;
    //                 vType->parametersSet |= VTYPEPARS_TTT_BIDI_SET;
    //             }
    //         }
    //         if (attrs.hasAttribute(SUMO_ATTR_SPEEDFACTOR_PREMATURE)) {
    //             bool ok = true;
    //             const double sfp = attrs.get<double>(SUMO_ATTR_SPEEDFACTOR_PREMATURE, id.c_str(), ok);
    //             if (!ok) {
    //                 return handleVehicleTypeError(hardFail, vType);
    //             } else {
    //                 vType->speedFactorPremature = sfp;
    //                 vType->parametersSet |= VTYPEPARS_SPEEDFACTOR_PREMATURE_SET;
    //             }
    //         }
    //         if (attrs.hasAttribute(SUMO_ATTR_BOARDING_FACTOR)) {
    //             bool ok = true;
    //             const double bf = attrs.get<double>(SUMO_ATTR_BOARDING_FACTOR, id.c_str(), ok);
    //             if (!ok) {
    //                 return handleVehicleTypeError(hardFail, vType);
    //             } else if (bf < 0) {
    //                 return handleVehicleTypeError(hardFail, vType, toString(SUMO_ATTR_BOARDING_FACTOR) + " must be equal or greater than 0");
    //             } else {
    //                 vType->boardingFactor = bf;
    //                 vType->parametersSet |= VTYPEPARS_BOARDING_FACTOR_SET;
    //             }
    //         }
    //         if (attrs.hasAttribute(SUMO_ATTR_MAXSPEED_LAT)) {
    //             bool ok = true;
    //             const double maxSpeedLat = attrs.get<double>(SUMO_ATTR_MAXSPEED_LAT, vType->id.c_str(), ok);
    //             if (!ok) {
    //                 return handleVehicleTypeError(hardFail, vType);
    //             } else if (maxSpeedLat <= 0) {
    //                 return handleVehicleTypeError(hardFail, vType, toString(SUMO_ATTR_MAXSPEED_LAT) + " must be greater than 0");
    //             } else {
    //                 vType->maxSpeedLat = maxSpeedLat;
    //                 vType->parametersSet |= VTYPEPARS_MAXSPEED_LAT_SET;
    //             }
    //         }
    //         if (attrs.hasAttribute(SUMO_ATTR_MINGAP_LAT)) {
    //             bool ok = true;
    //             const double minGapLat = attrs.get<double>(SUMO_ATTR_MINGAP_LAT, vType->id.c_str(), ok);
    //             if (!ok) {
    //                 return handleVehicleTypeError(hardFail, vType);
    //             } else if (minGapLat < 0) {
    //                 return handleVehicleTypeError(hardFail, vType, toString(SUMO_ATTR_MINGAP_LAT) + " must be equal or greater than 0");
    //             } else {
    //                 vType->minGapLat = minGapLat;
    //                 vType->parametersSet |= VTYPEPARS_MINGAP_LAT_SET;
    //             }
    //         }
    //         if (attrs.hasAttribute(SUMO_ATTR_LATALIGNMENT)) {
    //             bool ok = true;
    //             const std::string alignS = attrs.get<std::string>(SUMO_ATTR_LATALIGNMENT, vType->id.c_str(), ok);
    //             if (!ok) {
    //                 return handleVehicleTypeError(hardFail, vType);
    //             } else {
    //                 double lao;
    //                 LatAlignmentDefinition lad;
    //                 if (SUMOVTypeParameter::parseLatAlignment(alignS, lao, lad)) {
    //                     vType->latAlignmentOffset = lao;
    //                     vType->latAlignmentProcedure = lad;
    //                     vType->parametersSet |= VTYPEPARS_LATALIGNMENT_SET;
    //                 } else {
    //                     return handleVehicleTypeError(hardFail, vType, "Unknown lateral alignment '" + alignS + "' when parsing vType '" + vType->id + "';\n must be one of (\"right\", \"center\", \"arbitrary\", \"nice\", \"compact\", \"left\" or a float)");
    //                 }
    //             }
    //         }
    //         if (attrs.hasAttribute(SUMO_ATTR_MANEUVER_ANGLE_TIMES)) {
    //             bool ok = true;
    //             const std::string angleTimesS = attrs.get<std::string>(SUMO_ATTR_MANEUVER_ANGLE_TIMES, vType->id.c_str(), ok);
    //             if (!ok) {
    //                 return handleVehicleTypeError(hardFail, vType);
    //             } else if (parseAngleTimesMap(vType, angleTimesS)) {
    //                 vType->parametersSet |= VTYPEPARS_MANEUVER_ANGLE_TIMES_SET;
    //             } else {
    //                 return handleVehicleTypeError(hardFail, vType, "Invalid manoeuver angle times map for vType '" + vType->id + "'");
    //             }
    //         }
    //         if (attrs.hasAttribute(SUMO_ATTR_PARKING_BADGES)) {
    //             bool ok = true;
    //             std::vector<std::string> badges = attrs.get<std::vector<std::string>>(SUMO_ATTR_PARKING_BADGES, vType->id.c_str(), ok);
    //             if (!ok) {
    //                 return handleVehicleTypeError(hardFail, vType);
    //             } else {
    //                 vType->parametersSet |= VTYPEPARS_PARKING_BADGES_SET;
    //                 vType->parkingBadges = badges;
    //             }
    //         }
    //         // try to parse Car Following Model params
    //         if (!parseCFMParams(vType, vType->cfModel, attrs, false)) {
    //             return handleVehicleTypeError(hardFail, vType, "Invalid parsing embedded VType");
    //         }
    //         // try to parse Lane Change Model params
    //         if (!parseLCParams(vType, vType->lcModel, attrs)) {
    //             return handleVehicleTypeError(hardFail, vType, "Invalid Lane Change Model Parameters");
    //         }
    //         // try to Junction Model params
    //         if (!parseJMParams(vType, attrs)) {
    //             return handleVehicleTypeError(hardFail, vType, "Invalid Junction Model Parameters");
    //         }
    //         // all ok, then return vType
    //         return vType;
    //     } else if (base != nullptr) {
    //         return const_cast<SUMOVTypeParameter*>(base);
    //     } else {
    //         return handleVehicleTypeError(hardFail, nullptr, "VType cannot be created");
    //     }
    // }
    #[allow(unused_variables)]
    pub fn begin_v_type_parsing(attrs: &SUMOSAXAttributes, hardFail: bool, file: &str, base: Ptr<SUMOVTypeParameter>) -> Ptr<SUMOVTypeParameter> {
        todo!("port SUMOVehicleParserHelper::beginVTypeParsing")
    }

    /// C++: `bool SUMOVehicleParserHelper::parseAngleTimesMap(SUMOVTypeParameter *, const std::string)` [static]
    /// Parse string containing AngleTimes triplets (angle, entry time, exit time)
    // bool
    // SUMOVehicleParserHelper::parseAngleTimesMap(SUMOVTypeParameter* vtype, const std::string atm) {
    //     StringTokenizer st(atm, ",");
    //     std::map<int, std::pair<SUMOTime, SUMOTime>> angleTimesMap;
    //     int tripletCount = 0;
    //     while (st.hasNext()) {
    //         StringTokenizer pos(st.next());
    //         if (pos.size() != 3) {
    //             WRITE_ERRORF(TL("maneuverAngleTimes format for vType '%' % contains an invalid triplet."), vtype->id, atm);
    //             return false;
    //         } else {
    //             try {
    //                 const int angle = StringUtils::toInt(pos.next());
    //                 const SUMOTime t1 = string2time(pos.next());
    //                 const SUMOTime t2 = string2time(pos.next());
    //                 angleTimesMap[angle] = std::make_pair(t1, t2);
    //             } catch (...) {
    //                 WRITE_ERRORF(TL("Triplet '%' for vType '%' maneuverAngleTimes cannot be parsed as 'int double double'"), st.get(tripletCount), vtype->id);
    //                 return false;
    //             }
    //             tripletCount++;
    //         }
    //     }
    //     if (angleTimesMap.size() > 0) {
    //         vtype->myManoeuverAngleTimes.clear();
    //         for (const auto& angleTime : angleTimesMap) {
    //             vtype->myManoeuverAngleTimes.insert(angleTime);
    //         }
    //         angleTimesMap.clear();
    //         return true;
    //     } else {
    //         return false;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_angle_times_map(vtype: Ptr<SUMOVTypeParameter>, arg1: String) -> bool {
        todo!("port SUMOVehicleParserHelper::parseAngleTimesMap")
    }

    /// C++: `bool SUMOVehicleParserHelper::parseCFMParams(SUMOVTypeParameter *, const SumoXMLTag, const SUMOSAXAttributes &, const bool)` [static]
    /// Parses Car Following Mode params
    // bool
    // SUMOVehicleParserHelper::parseCFMParams(SUMOVTypeParameter* into, const SumoXMLTag element, const SUMOSAXAttributes& attrs, const bool nestedCFM) {
    //     const CFAttrMap& allowedCFM = getAllowedCFModelAttrs();
    //     CFAttrMap::const_iterator cf_it = allowedCFM.find(element);
    //     // check if given CFM is allowed
    //     if (cf_it == allowedCFM.end()) {
    //         if (SUMOXMLDefinitions::Tags.has((int)element)) {
    //             WRITE_ERRORF(TL("Unknown car-following model % when parsing vType '%'"), toString(element), into->id);
    //         } else {
    //             WRITE_ERRORF(TL("Unknown car-following model when parsing vType '%'"), into->id);
    //         }
    //         return false;
    //     }
    //     // check if we're parsing a nested CFM
    //     if (nestedCFM) {
    //         into->cfModel = cf_it->first;
    //         into->parametersSet |= VTYPEPARS_CAR_FOLLOW_MODEL;
    //     }
    //     // set CFM values
    //     for (const auto& it : cf_it->second) {
    //         if (attrs.hasAttribute(it)) {
    //             // first obtain  CFM attribute in string format
    //             bool ok = true;
    //             std::string parsedCFMAttribute = attrs.get<std::string>(it, into->id.c_str(), ok);
    //             // check CFM Attribute
    //             if (!ok) {
    //                 return false;
    //             } else if (it == SUMO_ATTR_TRAIN_TYPE) {
    //                 // check if train value is valid
    //                 if (!SUMOXMLDefinitions::TrainTypes.hasString(parsedCFMAttribute)) {
    //                     WRITE_ERROR("Invalid train type '" + parsedCFMAttribute + "' used in Car-Following-Attribute " + toString(it));
    //                     return false;
    //                 }
    //                 // add parsedCFMAttribute to cfParameter
    //                 into->cfParameter[it] = parsedCFMAttribute;
    //             } else if (it == SUMO_ATTR_SPEED_TABLE || it == SUMO_ATTR_TRACTION_TABLE || it == SUMO_ATTR_RESISTANCE_TABLE) {
    //                 into->cfParameter[it] = parsedCFMAttribute;
    //             } else if (it == SUMO_ATTR_CF_IDM_STEPPING) {
    //                 // declare a int in which save CFM int attribute
    //                 double CFMDoubleAttribute = -1;
    //                 try {
    //                     // obtain CFM attribute in int format
    //                     CFMDoubleAttribute = StringUtils::toDouble(parsedCFMAttribute);
    //                 } catch (...) {
    //                     WRITE_ERRORF(TL("Invalid Car-Following-Model Attribute %. Cannot be parsed to float"), toString(it));
    //                     return false;
    //                 }
    //                 if (CFMDoubleAttribute <= 0) {
    //                     WRITE_ERRORF(TL("Invalid Car-Following-Model Attribute %. Must be greater than 0"), toString(it));
    //                     return false;
    //                 }
    //                 // add parsedCFMAttribute to cfParameter
    //                 into->cfParameter[it] = parsedCFMAttribute;
    //             } else if (it == SUMO_ATTR_MAXACCEL_PROFILE || it == SUMO_ATTR_DESACCEL_PROFILE) {
    //                 if (validProfile(into, parsedCFMAttribute, it)) {
    //                     into->cfParameter[it] = parsedCFMAttribute;
    //                 } else {
    //                     WRITE_ERRORF(TL("Invalid Car-Following-Model Attribute %. Cannot be parsed as a vector of <speed accel> pairs"), toString(it));
    //                     return false;
    //                 }
    //             } else {
    //                 // declare a double in which save CFM float attribute
    //                 double CFMDoubleAttribute = -1;
    //                 try {
    //                     // obtain CFM attribute in double format
    //                     CFMDoubleAttribute = StringUtils::toDouble(parsedCFMAttribute);
    //                 } catch (...) {
    //                     WRITE_ERRORF(TL("Invalid Car-Following-Model Attribute %. Cannot be parsed to float"), toString(it));
    //                     return false;
    //                 }
    //                 // check attributes of type "positiveFloatType" (> 0)
    //                 switch (it) {
    //                     case SUMO_ATTR_ACCEL:
    //                     case SUMO_ATTR_DECEL:
    //                     case SUMO_ATTR_APPARENTDECEL:
    //                     case SUMO_ATTR_EMERGENCYDECEL:
    //                     case SUMO_ATTR_TAU:
    //                         if (CFMDoubleAttribute <= 0) {
    //                             WRITE_ERRORF(TL("Invalid Car-Following-Model Attribute %. Must be greater than 0"), toString(it));
    //                             return false;
    //                         }
    //                         break;
    //                     default:
    //                         break;
    //                 }
    //                 // check attributes restricted to [0-1]
    //                 switch (it) {
    //                     case SUMO_ATTR_SIGMA:
    //                         if ((CFMDoubleAttribute < 0) || (CFMDoubleAttribute > 1)) {
    //                             WRITE_ERRORF(TL("Invalid Car-Following-Model Attribute %. Only values between [0-1] are allowed"), toString(it));
    //                             return false;
    //                         }
    //                         break;
    //                     default:
    //                         break;
    //                 }
    //                 // add parsedCFMAttribute to cfParameter
    //                 into->cfParameter[it] = parsedCFMAttribute;
    //             }
    //         }
    //     }
    //     // all CFM successfully parsed, then return true
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn parse_cfm_params(into: Ptr<SUMOVTypeParameter>, element: SumoXMLTag, attrs: &SUMOSAXAttributes, nestedCFM: bool) -> bool {
        todo!("port SUMOVehicleParserHelper::parseCFMParams")
    }

    /// C++: `bool SUMOVehicleParserHelper::parseLCParams(SUMOVTypeParameter *, LaneChangeModel, const SUMOSAXAttributes &)` [static]
    /// Parses lane change model attributes
    // bool
    // SUMOVehicleParserHelper::parseLCParams(SUMOVTypeParameter* into, LaneChangeModel model, const SUMOSAXAttributes& attrs) {
    //     if (allowedLCModelAttrs.size() == 0) {
    //         // lc2013
    //         std::set<SumoXMLAttr> lc2013Params;
    //         lc2013Params.insert(SUMO_ATTR_LCA_STRATEGIC_PARAM);
    //         lc2013Params.insert(SUMO_ATTR_LCA_COOPERATIVE_PARAM);
    //         lc2013Params.insert(SUMO_ATTR_LCA_SPEEDGAIN_PARAM);
    //         lc2013Params.insert(SUMO_ATTR_LCA_KEEPRIGHT_PARAM);
    //         lc2013Params.insert(SUMO_ATTR_LCA_OPPOSITE_PARAM);
    //         lc2013Params.insert(SUMO_ATTR_LCA_LOOKAHEADLEFT);
    //         lc2013Params.insert(SUMO_ATTR_LCA_SPEEDGAINRIGHT);
    //         lc2013Params.insert(SUMO_ATTR_LCA_MAXSPEEDLATSTANDING);
    //         lc2013Params.insert(SUMO_ATTR_LCA_MAXSPEEDLATFACTOR);
    //         lc2013Params.insert(SUMO_ATTR_LCA_MAXDISTLATSTANDING);
    //         lc2013Params.insert(SUMO_ATTR_LCA_ASSERTIVE);
    //         lc2013Params.insert(SUMO_ATTR_LCA_STRATEGIC_LOOKAHEAD);
    //         lc2013Params.insert(SUMO_ATTR_LCA_SPEEDGAIN_LOOKAHEAD);
    //         lc2013Params.insert(SUMO_ATTR_LCA_SPEEDGAIN_REMAIN_TIME);
    //         lc2013Params.insert(SUMO_ATTR_LCA_SPEEDGAIN_URGENCY);
    //         lc2013Params.insert(SUMO_ATTR_LCA_COOPERATIVE_ROUNDABOUT);
    //         lc2013Params.insert(SUMO_ATTR_LCA_COOPERATIVE_SPEED);
    //         lc2013Params.insert(SUMO_ATTR_LCA_COOPERATIVE_HELPTIME);
    //         lc2013Params.insert(SUMO_ATTR_LCA_COOPERATIVE_HELPTHRESHOLD);
    //         lc2013Params.insert(SUMO_ATTR_LCA_COOPERATIVE_MINSPEED);
    //         lc2013Params.insert(SUMO_ATTR_LCA_OVERTAKE_RIGHT);
    //         lc2013Params.insert(SUMO_ATTR_LCA_SIGMA);
    //         lc2013Params.insert(SUMO_ATTR_LCA_KEEPRIGHT_ACCEPTANCE_TIME);
    //         lc2013Params.insert(SUMO_ATTR_LCA_OVERTAKE_DELTASPEED_FACTOR);
    //         lc2013Params.insert(SUMO_ATTR_LCA_CONTRIGHT);
    //         lc2013Params.insert(SUMO_ATTR_LCA_EXPERIMENTAL1);
    //         allowedLCModelAttrs[LaneChangeModel::LC2013] = lc2013Params;
    //         // sl2015 (extension of lc2013)
    //         std::set<SumoXMLAttr> sl2015Params = lc2013Params;
    //         sl2015Params.insert(SUMO_ATTR_LCA_PUSHY);
    //         sl2015Params.insert(SUMO_ATTR_LCA_PUSHYGAP);
    //         sl2015Params.insert(SUMO_ATTR_LCA_SUBLANE_PARAM);
    //         sl2015Params.insert(SUMO_ATTR_LCA_IMPATIENCE);
    //         sl2015Params.insert(SUMO_ATTR_LCA_TIME_TO_IMPATIENCE);
    //         sl2015Params.insert(SUMO_ATTR_LCA_ACCEL_LAT);
    //         sl2015Params.insert(SUMO_ATTR_LCA_TURN_ALIGNMENT_DISTANCE);
    //         sl2015Params.insert(SUMO_ATTR_LCA_LANE_DISCIPLINE);
    //         allowedLCModelAttrs[LaneChangeModel::SL2015] = sl2015Params;
    //         // DK2008
    //         std::set<SumoXMLAttr> noParams;
    //         allowedLCModelAttrs[LaneChangeModel::DK2008] = noParams;
    //         // default model may be either LC2013 or SL2015
    //         // we allow both sets (sl2015 is a superset of lc2013Params)
    //         allowedLCModelAttrs[LaneChangeModel::DEFAULT] = sl2015Params;
    //     }
    //     std::set<SumoXMLAttr> allowed = allowedLCModelAttrs[model];
    //     // iterate over LCM attributes
    //     for (const auto& it : allowed) {
    //         if (attrs.hasAttribute(it)) {
    //             // first obtain  CFM attribute in string format
    //             bool ok = true;
    //             std::string parsedLCMAttribute = attrs.get<std::string>(it, into->id.c_str(), ok);
    //             if (!ok) {
    //                 return false;
    //             }
    //             // declare a double in which save CFM attribute
    //             double LCMAttribute = -1;
    //             try {
    //                 // obtain CFM attribute in double format
    //                 LCMAttribute = StringUtils::toDouble(parsedLCMAttribute);
    //             } catch (...) {
    //                 WRITE_ERRORF(TL("Invalid Lane-Change-Model Attribute %. Cannot be parsed to float"), toString(it));
    //                 return false;
    //             }
    //             // check attributes of type "nonNegativeFloatType" (>= 0)
    //             switch (it) {
    //                 case SUMO_ATTR_LCA_PUSHYGAP:
    //                 case SUMO_ATTR_LCA_MAXSPEEDLATSTANDING:
    //                 case SUMO_ATTR_LCA_IMPATIENCE:
    //                 case SUMO_ATTR_LCA_OVERTAKE_RIGHT:
    //                 case SUMO_ATTR_LCA_ASSERTIVE:
    //                 case SUMO_ATTR_LCA_LOOKAHEADLEFT:
    //                 case SUMO_ATTR_LCA_SPEEDGAINRIGHT:
    //                 case SUMO_ATTR_LCA_TURN_ALIGNMENT_DISTANCE:
    //                 case SUMO_ATTR_LCA_TIME_TO_IMPATIENCE:
    //                 case SUMO_ATTR_LCA_LANE_DISCIPLINE:
    //                 case SUMO_ATTR_LCA_SIGMA:
    //                     if (LCMAttribute < 0) {
    //                         WRITE_ERRORF(TL("Invalid Lane-Change-Model Attribute %. Must be equal or greater than 0"), toString(it));
    //                         return false;
    //                     }
    //                     break;
    //                 default:
    //                     break;
    //             }
    //             // check attributes of type "positiveFloatType" (> 0)
    //             switch (it) {
    //                 case SUMO_ATTR_LCA_ACCEL_LAT:
    //                     if (LCMAttribute <= 0) {
    //                         WRITE_ERRORF(TL("Invalid Lane-Change-Model Attribute %. Must be greater than 0"), toString(it));
    //                         return false;
    //                     }
    //                     break;
    //                 default:
    //                     break;
    //             }
    //             // check limits of attributes
    //             switch (it) {
    //                 case SUMO_ATTR_LCA_OVERTAKE_DELTASPEED_FACTOR:
    //                     if (LCMAttribute < -1 || LCMAttribute > 1) {
    //                         WRITE_ERRORF(TL("Invalid Lane-Change-Model Attribute %. Must be between -1 and 1"), toString(it));
    //                         return false;
    //                     }
    //                     break;
    //                 default:
    //                     break;
    //             }
    //             // add parsedLCMAttribute to cfParameter
    //             into->lcParameter[it] = parsedLCMAttribute;
    //         }
    //     }
    //     // all LCM parsed ok, then return true
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn parse_lc_params(into: Ptr<SUMOVTypeParameter>, model: LaneChangeModel, attrs: &SUMOSAXAttributes) -> bool {
        todo!("port SUMOVehicleParserHelper::parseLCParams")
    }

    /// C++: `bool SUMOVehicleParserHelper::parseJMParams(SUMOVTypeParameter *, const SUMOSAXAttributes &)` [static]
    /// Parses junction model attributes
    // bool
    // SUMOVehicleParserHelper::parseJMParams(SUMOVTypeParameter* into, const SUMOSAXAttributes& attrs) {
    //     for (const auto& it : SUMOVTypeParameter::AllowedJMAttrs) {
    //         if (attrs.hasAttribute(it)) {
    //             // first obtain  CFM attribute in string format
    //             bool ok = true;
    //             std::string parsedJMAttribute = attrs.get<std::string>(it, into->id.c_str(), ok);
    //             if (!ok) {
    //                 return false;
    //             }
    //             // declare a double in which save CFM attribute
    //             double JMAttribute = INVALID_DOUBLE;
    //             try {
    //                 // obtain CFM attribute in double format
    //                 JMAttribute = StringUtils::toDouble(parsedJMAttribute);
    //             } catch (...) {
    //                 WRITE_ERRORF(TL("Invalid Junction-Model Attribute %. Cannot be parsed to float"), toString(it));
    //                 return false;
    //             }
    //             // now continue checking other properties (-1 is the default value)
    //             if (JMAttribute != INVALID_DOUBLE) {
    //                 // special case for sigma minor
    //                 if (it == SUMO_ATTR_JM_SIGMA_MINOR) {
    //                     // check attributes sigma minor
    //                     if ((JMAttribute < 0) || (JMAttribute > 1)) {
    //                         WRITE_ERRORF(TL("Invalid Junction-Model Attribute %. Only values between [0-1] are allowed"), toString(it));
    //                         return false;
    //                     }
    //                 } else if (JMAttribute < 0
    //                            && it != SUMO_ATTR_JM_TIMEGAP_MINOR
    //                            && it != SUMO_ATTR_JM_EXTRA_GAP) {
    //                     // attributes with error value
    //                     if (JMAttribute != -1 || (it != SUMO_ATTR_JM_DRIVE_AFTER_YELLOW_TIME
    //                                               && it != SUMO_ATTR_JM_DRIVE_AFTER_RED_TIME
    //                                               && it != SUMO_ATTR_JM_IGNORE_KEEPCLEAR_TIME)) {
    //                         // check attributes of type "nonNegativeFloatType" (>= 0)
    //                         WRITE_ERRORF(TL("Invalid Junction-Model Attribute %. Must be equal or greater than 0"), toString(it));
    //                         return false;
    //                     }
    //                 }
    //                 // add parsedJMAttribute to cfParameter
    //                 into->jmParameter[it] = parsedJMAttribute;
    //             }
    //         }
    //     }
    //     // all JM parameters successfully parsed, then return true
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn parse_jm_params(into: Ptr<SUMOVTypeParameter>, attrs: &SUMOSAXAttributes) -> bool {
        todo!("port SUMOVehicleParserHelper::parseJMParams")
    }

    /// C++: `SUMOVehicleClass SUMOVehicleParserHelper::parseVehicleClass(const SUMOSAXAttributes &, const std::string &)` [static]
    /// Parses the vehicle class
    // SUMOVehicleClass
    // SUMOVehicleParserHelper::parseVehicleClass(const SUMOSAXAttributes& attrs, const std::string& id) {
    //     SUMOVehicleClass vclass = SVC_IGNORING;
    //     bool ok = true;
    //     std::string vclassS = attrs.getOpt<std::string>(SUMO_ATTR_VCLASS, id.c_str(), ok, "");
    //     if (vclassS == "") {
    //         return vclass;
    //     }
    //     try {
    //         const SUMOVehicleClass result = getVehicleClassID(vclassS);
    //         const std::string& realName = SumoVehicleClassStrings.getString(result);
    //         if (realName != vclassS) {
    //             WRITE_WARNING("The vehicle class '" + vclassS + "' for " + attrs.getObjectType() + " '" + id + "' is deprecated, use '" + realName + "' instead.");
    //         }
    //         return result;
    //     } catch (...) {
    //         WRITE_ERRORF(TL("The vehicle class '%' for % '%' is not known."), vclassS, attrs.getObjectType(), id);
    //     }
    //     return vclass;
    // }
    #[allow(unused_variables)]
    pub fn parse_vehicle_class(attrs: &SUMOSAXAttributes, id: &str) -> SUMOVehicleClass {
        let mut vclass = SVC_IGNORING;
        let mut ok = true;
        let vclassS = attrs.get_opt::<String>(SUMO_ATTR_VCLASS, id, &mut ok, String::new(), true);
        if vclassS == "" {
            return vclass;
        }
        let attempt = get_vehicle_class_id(&vclassS).and_then(|result| {
            SUMO_VEHICLE_CLASS_STRINGS.get_string(result).map(|realName| (result, realName.to_string()))
        });
        match attempt {
            Ok((result, realName)) => {
                if realName != vclassS {
                    write_warning(&format!(
                        "The vehicle class '{}' for {} '{}' is deprecated, use '{}' instead.",
                        vclassS,
                        attrs.get_object_type(),
                        id,
                        realName
                    ));
                }
                return result;
            }
            Err(_) => {
                write_error(&format!("The vehicle class '{}' for {} '{}' is not known.", vclassS, attrs.get_object_type(), id));
            }
        }
        vclass
    }

    /// C++: `SUMOVehicleShape SUMOVehicleParserHelper::parseGuiShape(const SUMOSAXAttributes &, const std::string &)` [static]
    /// Parses the vehicle class
    // SUMOVehicleShape
    // SUMOVehicleParserHelper::parseGuiShape(const SUMOSAXAttributes& attrs, const std::string& id) {
    //     bool ok = true;
    //     std::string vclassS = attrs.getOpt<std::string>(SUMO_ATTR_GUISHAPE, id.c_str(), ok, "");
    //     if (SumoVehicleShapeStrings.hasString(vclassS)) {
    //         const SUMOVehicleShape result = SumoVehicleShapeStrings.get(vclassS);
    //         const std::string& realName = SumoVehicleShapeStrings.getString(result);
    //         if (realName != vclassS) {
    //             WRITE_WARNING("The shape '" + vclassS + "' for " + attrs.getObjectType() + " '" + id + "' is deprecated, use '" + realName + "' instead.");
    //         }
    //         return result;
    //     } else {
    //         WRITE_ERRORF(TL("The shape '%' for % '%' is not known."), vclassS, attrs.getObjectType(), id);
    //         return SUMOVehicleShape::UNKNOWN;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_gui_shape(attrs: &SUMOSAXAttributes, id: &str) -> SUMOVehicleShape {
        let mut ok = true;
        let vclassS = attrs.get_opt::<String>(SUMO_ATTR_GUISHAPE, id, &mut ok, String::new(), true);
        if SUMO_VEHICLE_SHAPE_STRINGS.has_string(&vclassS) {
            let result = SUMO_VEHICLE_SHAPE_STRINGS.get(&vclassS).unwrap_or_else(|e| throw_process_error(e.to_string()));
            let realName = SUMO_VEHICLE_SHAPE_STRINGS.get_string(result).unwrap_or_else(|e| throw_process_error(e.to_string())).to_string();
            if realName != vclassS {
                write_warning(&format!("The shape '{}' for {} '{}' is deprecated, use '{}' instead.", vclassS, attrs.get_object_type(), id, realName));
            }
            return result;
        } else {
            write_error(&format!("The shape '{}' for {} '{}' is not known.", vclassS, attrs.get_object_type(), id));
            return SUMOVehicleShape::UNKNOWN;
        }
    }

    /// C++: `double SUMOVehicleParserHelper::parseWalkPos(SumoXMLAttr, const bool, const std::string &, double, const std::string &, SumoRNG *)` [static]
    /// parse departPos or arrivalPos for a walk
    // double
    // SUMOVehicleParserHelper::parseWalkPos(SumoXMLAttr attr, const bool hardFail, const std::string& id, double maxPos, const std::string& val, SumoRNG* rng) {
    //     double result;
    //     std::string error;
    //     ArrivalPosDefinition proc = ArrivalPosDefinition::DEFAULT;
    //     // only supports 'random' and 'max'
    //     if (!SUMOVehicleParameter::parseArrivalPos(val, toString(SUMO_TAG_WALK), id, result, proc, error)) {
    //         handleVehicleError(hardFail, nullptr, error);
    //     }
    //     if (proc == ArrivalPosDefinition::RANDOM) {
    //         result = RandHelper::rand(maxPos, rng);
    //     } else if (proc == ArrivalPosDefinition::CENTER) {
    //         result = maxPos / 2.;
    //     } else if (proc == ArrivalPosDefinition::MAX) {
    //         result = maxPos;
    //     }
    //     return SUMOVehicleParameter::interpretEdgePos(result, maxPos, attr, id);
    // }
    #[allow(unused_variables)]
    pub fn parse_walk_pos(attr: SumoXMLAttr, hardFail: bool, id: &str, maxPos: f64, val: &str, rng: Ptr<SumoRNG>) -> f64 {
        let mut result: f64 = 0.0;
        let mut error = String::new();
        let mut proc_ = ArrivalPosDefinition::DEFAULT;
        // only supports 'random' and 'max'
        if !SUMOVehicleParameter::parse_arrival_pos(val, Tags.get_string(SUMO_TAG_WALK), id, &mut result, &mut proc_, &mut error) {
            Self::handle_vehicle_error(hardFail, Ptr::null(), error);
        }
        if matches!(proc_, ArrivalPosDefinition::RANDOM) {
            // RandHelper::rand(maxV, rng) = maxV * rand(rng); a null rng selects the default generator
            let r = if rng.is_null() { utils::common::rand_helper::rand() } else { rng.m().rand() };
            result = maxPos * r;
        } else if matches!(proc_, ArrivalPosDefinition::CENTER) {
            result = maxPos / 2.0;
        } else if matches!(proc_, ArrivalPosDefinition::MAX) {
            result = maxPos;
        }
        SUMOVehicleParameter::interpret_edge_pos(result, maxPos, attr, id, false)
    }

    /// C++: `SUMOTime SUMOVehicleParserHelper::processActionStepLength(double)` [static]
    /// Checks and converts given value for the action step length from seconds to miliseconds assuring it being a positive multiple of the simulation step width
    // SUMOTime
    // SUMOVehicleParserHelper::processActionStepLength(double given) {
    //     const std::string defaultError = "The parameter action-step-length must be a non-negative multiple of the simulation step-length. ";
    //     SUMOTime result = TIME2STEPS(given);
    //     if (result <= 0) {
    //         if (result < 0) {
    //             WRITE_WARNING(defaultError + "Ignoring given value (=" + toString(STEPS2TIME(result)) + " s.)");
    //         }
    //         result = DELTA_T;
    //     } else if (result % DELTA_T != 0 && OptionsCont::getOptions().exists("step-length")) {
    //         result = (SUMOTime)((double)DELTA_T * floor(double(result) / double(DELTA_T)));
    //         result = MAX2(DELTA_T, result);
    //         if (fabs(given * 1000. - double(result)) > NUMERICAL_EPS) {
    //             WRITE_WARNING(defaultError + "Parsing given value (" + toString(given) + " s.) to the adjusted value " + toString(STEPS2TIME(result)) + " s.");
    //         }
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn process_action_step_length(given: f64) -> SUMOTime {
        let defaultError = "The parameter action-step-length must be a non-negative multiple of the simulation step-length. ";
        let mut result: SUMOTime = time2steps(given);
        if result <= 0 {
            if result < 0 {
                write_warning(&format!("{}Ignoring given value (={} s.)", defaultError, utils::common::to_string::to_string(&steps2time(result))));
            }
            result = delta_t();
        } else if result % delta_t() != 0 && utils::options::options_cont::exists("step-length") {
            result = (delta_t() as f64 * (result as f64 / delta_t() as f64).floor()) as SUMOTime;
            result = std::cmp::max(delta_t(), result);
            if (given * 1000.0 - result as f64).abs() > NUMERICAL_EPS {
                write_warning(&format!(
                    "{}Parsing given value ({} s.) to the adjusted value {} s.",
                    defaultError,
                    utils::common::to_string::to_string(&given),
                    utils::common::to_string::to_string(&steps2time(result))
                ));
            }
        }
        result
    }

    /// C++: `bool SUMOVehicleParserHelper::isInternalRouteID(const std::string &)` [static]
    /// Checks whether the route ID uses the syntax of internal routes.
    // bool
    // SUMOVehicleParserHelper::isInternalRouteID(const std::string& id) {
    //     return id.substr(0, 1) == "!";
    // }
    #[allow(unused_variables)]
    pub fn is_internal_route_id(id: &str) -> bool {
        id.starts_with('!')
    }

    /// C++: `bool SUMOVehicleParserHelper::validProfile(SUMOVTypeParameter *, const std::string, const SumoXMLAttr)` [static]
    /// Parse string containing a vector of pairs
    // bool
    // SUMOVehicleParserHelper::validProfile(SUMOVTypeParameter* vtype, const std::string data, const SumoXMLAttr attr) {
    //     for (std::string value : StringTokenizer(data).getVector()) {
    //         try {
    //             double v = StringUtils::toDouble(value);
    //             if (v < 0.) {
    //                 WRITE_ERRORF(TL("Invalid Car-Following-Model Attribute %. An acceleration profile value cannot be negative"), toString(attr));
    //                 return false;
    //             }
    //         } catch (...) {
    //             WRITE_ERRORF(TL("Entry '%' of % table for vType '%' cannot be parsed as 'double'"), value, toString(attr), vtype->id);
    //             return false;
    //         }
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn valid_profile(vtype: Ptr<SUMOVTypeParameter>, data: String, attr: SumoXMLAttr) -> bool {
        for value in StringTokenizer::new(&data).get_vector() {
            match to_double(&value) {
                Ok(v) => {
                    if v < 0.0 {
                        write_error(&format!("Invalid Car-Following-Model Attribute {}. An acceleration profile value cannot be negative", Attrs.get_string(attr)));
                        return false;
                    }
                }
                Err(_) => {
                    write_error(&format!("Entry '{}' of {} table for vType '{}' cannot be parsed as 'double'", value, Attrs.get_string(attr), vtype.id));
                    return false;
                }
            }
        }
        true
    }

    /// C++: `int SUMOVehicleParserHelper::parseCarWalkTransfer(const OptionsCont &, const bool)` [static]
    // int
    // SUMOVehicleParserHelper::parseCarWalkTransfer(const OptionsCont& oc, const bool hasTaxi) {
    //     int carWalk = 0;
    //     for (const std::string& opt : oc.getStringVector("persontrip.transfer.car-walk")) {
    //         if (opt == "parkingAreas") {
    //             carWalk |= ModeChangeOptions::PARKING_AREAS;
    //         } else if (opt == "ptStops") {
    //             carWalk |= ModeChangeOptions::PT_STOPS;
    //         } else if (opt == "allJunctions") {
    //             carWalk |= ModeChangeOptions::ALL_JUNCTIONS;
    //         } else {
    //             WRITE_ERRORF(TL("Invalid transfer option '%'. Must be one of 'parkingAreas', 'ptStops' and 'allJunctions'"), opt);
    //         }
    //     }
    //     const StringVector taxiDropoff = oc.getStringVector("persontrip.transfer.taxi-walk");
    //     const StringVector taxiPickup = oc.getStringVector("persontrip.transfer.walk-taxi");
    //     if (taxiDropoff.empty() && hasTaxi) {
    //         carWalk |= ModeChangeOptions::TAXI_DROPOFF_ANYWHERE;
    //     } else {
    //         for (const std::string& opt : taxiDropoff) {
    //             if (opt == "parkingAreas") {
    //                 carWalk |= ModeChangeOptions::TAXI_DROPOFF_PARKING_AREAS;
    //             } else if (opt == "ptStops") {
    //                 carWalk |= ModeChangeOptions::TAXI_DROPOFF_PT;
    //             } else if (opt == "allJunctions") {
    //                 carWalk |= ModeChangeOptions::TAXI_DROPOFF_ANYWHERE;
    //             } else {
    //                 WRITE_ERRORF(TL("Invalid transfer option '%'. Must be one of 'parkingAreas', 'ptStops' and 'allJunctions'"), opt);
    //             }
    //         }
    //     }
    //     if (taxiPickup.empty() && hasTaxi) {
    //         carWalk |= ModeChangeOptions::TAXI_PICKUP_ANYWHERE;
    //     } else {
    //         for (const std::string& opt : taxiPickup) {
    //             if (opt == "parkingAreas") {
    //                 carWalk |= ModeChangeOptions::TAXI_PICKUP_PARKING_AREAS;
    //             } else if (opt == "ptStops") {
    //                 carWalk |= ModeChangeOptions::TAXI_PICKUP_PT;
    //             } else if (opt == "allJunctions") {
    //                 carWalk |= ModeChangeOptions::TAXI_PICKUP_ANYWHERE;
    //             } else {
    //                 WRITE_ERRORF(TL("Invalid transfer option '%'. Must be one of 'parkingAreas', 'ptStops' and 'allJunctions'"), opt);
    //             }
    //         }
    //     }
    //     return carWalk;
    // }
    #[allow(unused_variables)]
    pub fn parse_car_walk_transfer(oc: &OptionsCont, hasTaxi: bool) -> i32 {
        let mut carWalk: i32 = 0;
        for opt in oc.get_string_vector("persontrip.transfer.car-walk").unwrap_or_else(|e| throw_process_error(e.to_string())).iter() {
            if opt.as_str() == "parkingAreas" {
                carWalk |= ModeChangeOptions::PARKING_AREAS as i32;
            } else if opt.as_str() == "ptStops" {
                carWalk |= ModeChangeOptions::PT_STOPS as i32;
            } else if opt.as_str() == "allJunctions" {
                carWalk |= ModeChangeOptions::ALL_JUNCTIONS as i32;
            } else {
                write_error(&format!("Invalid transfer option '{}'. Must be one of 'parkingAreas', 'ptStops' and 'allJunctions'", opt));
            }
        }
        let taxiDropoff = oc.get_string_vector("persontrip.transfer.taxi-walk").unwrap_or_else(|e| throw_process_error(e.to_string()));
        let taxiPickup = oc.get_string_vector("persontrip.transfer.walk-taxi").unwrap_or_else(|e| throw_process_error(e.to_string()));
        if taxiDropoff.is_empty() && hasTaxi {
            carWalk |= ModeChangeOptions::TAXI_DROPOFF_ANYWHERE as i32;
        } else {
            for opt in taxiDropoff.iter() {
                if opt.as_str() == "parkingAreas" {
                    carWalk |= ModeChangeOptions::TAXI_DROPOFF_PARKING_AREAS as i32;
                } else if opt.as_str() == "ptStops" {
                    carWalk |= ModeChangeOptions::TAXI_DROPOFF_PT as i32;
                } else if opt.as_str() == "allJunctions" {
                    carWalk |= ModeChangeOptions::TAXI_DROPOFF_ANYWHERE as i32;
                } else {
                    write_error(&format!("Invalid transfer option '{}'. Must be one of 'parkingAreas', 'ptStops' and 'allJunctions'", opt));
                }
            }
        }
        if taxiPickup.is_empty() && hasTaxi {
            carWalk |= ModeChangeOptions::TAXI_PICKUP_ANYWHERE as i32;
        } else {
            for opt in taxiPickup.iter() {
                if opt.as_str() == "parkingAreas" {
                    carWalk |= ModeChangeOptions::TAXI_PICKUP_PARKING_AREAS as i32;
                } else if opt.as_str() == "ptStops" {
                    carWalk |= ModeChangeOptions::TAXI_PICKUP_PT as i32;
                } else if opt.as_str() == "allJunctions" {
                    carWalk |= ModeChangeOptions::TAXI_PICKUP_ANYWHERE as i32;
                } else {
                    write_error(&format!("Invalid transfer option '{}'. Must be one of 'parkingAreas', 'ptStops' and 'allJunctions'", opt));
                }
            }
        }
        carWalk
    }

    /// C++: `bool SUMOVehicleParserHelper::parseStopTag(const SUMOSAXAttributes &, const SUMOVehicleParameter::Stop *, SumoXMLTag &, std::string &)` [static]
    /// parse named stopping place attrs (or use stopParam attrs) and return it's tag and id
    // bool
    // SUMOVehicleParserHelper::parseStopTag(const SUMOSAXAttributes& attrs, const SUMOVehicleParameter::Stop* stopParam, SumoXMLTag& tag, std::string& id) {
    //     bool ok = true;
    //     // dummy stop parameter to hold the attributes
    //     SUMOVehicleParameter::Stop stop;
    //     if (stopParam != nullptr) {
    //         stop = *stopParam;
    //     } else {
    //         stop.busstop = attrs.getOpt<std::string>(SUMO_ATTR_BUS_STOP, nullptr, ok, "");
    //         stop.busstop = attrs.getOpt<std::string>(SUMO_ATTR_TRAIN_STOP, nullptr, ok, stop.busstop); // alias
    //         stop.chargingStation = attrs.getOpt<std::string>(SUMO_ATTR_CHARGING_STATION, nullptr, ok, "");
    //         stop.overheadWireSegment = attrs.getOpt<std::string>(SUMO_ATTR_OVERHEAD_WIRE_SEGMENT, nullptr, ok, "");
    //         stop.containerstop = attrs.getOpt<std::string>(SUMO_ATTR_CONTAINER_STOP, nullptr, ok, "");
    //         stop.parkingarea = attrs.getOpt<std::string>(SUMO_ATTR_PARKING_AREA, nullptr, ok, "");
    //     }
    //     if (stop.busstop != "") {
    //         tag = SUMO_TAG_BUS_STOP;
    //         id = stop.busstop;
    //     } else if (stop.containerstop != "") {
    //         tag = SUMO_TAG_CONTAINER_STOP;
    //         id = stop.containerstop;
    //     } else if (stop.parkingarea != "") {
    //         tag = SUMO_TAG_PARKING_AREA;
    //         id = stop.parkingarea;
    //     } else if (stop.chargingStation != "") {
    //         tag = SUMO_TAG_CHARGING_STATION;
    //         id = stop.chargingStation;
    //     } else if (stop.overheadWireSegment != "") {
    //         tag = SUMO_TAG_OVERHEAD_WIRE_SEGMENT;
    //         id = stop.overheadWireSegment;
    //     }
    //     return ok;
    // }
    #[allow(unused_variables)]
    pub fn parse_stop_tag(attrs: &SUMOSAXAttributes, stopParam: Ptr<SUMOVehicleParameter_Stop>, tag: &mut SumoXMLTag, id: &mut String) -> bool {
        let mut ok = true;
        // dummy stop parameter to hold the attributes
        let mut stop: SUMOVehicleParameter_Stop = SUMOVehicleParameter_Stop::default();
        if !stopParam.is_null() {
            stop = stopParam.m().clone();
        } else {
            stop.busstop = attrs.get_opt::<String>(SUMO_ATTR_BUS_STOP, "", &mut ok, String::new(), true);
            stop.busstop = attrs.get_opt::<String>(SUMO_ATTR_TRAIN_STOP, "", &mut ok, stop.busstop.clone(), true); // alias
            stop.chargingStation = attrs.get_opt::<String>(SUMO_ATTR_CHARGING_STATION, "", &mut ok, String::new(), true);
            stop.overheadWireSegment = attrs.get_opt::<String>(SUMO_ATTR_OVERHEAD_WIRE_SEGMENT, "", &mut ok, String::new(), true);
            stop.containerstop = attrs.get_opt::<String>(SUMO_ATTR_CONTAINER_STOP, "", &mut ok, String::new(), true);
            stop.parkingarea = attrs.get_opt::<String>(SUMO_ATTR_PARKING_AREA, "", &mut ok, String::new(), true);
        }
        if stop.busstop != "" {
            *tag = SUMO_TAG_BUS_STOP;
            *id = stop.busstop.clone();
        } else if stop.containerstop != "" {
            *tag = SUMO_TAG_CONTAINER_STOP;
            *id = stop.containerstop.clone();
        } else if stop.parkingarea != "" {
            *tag = SUMO_TAG_PARKING_AREA;
            *id = stop.parkingarea.clone();
        } else if stop.chargingStation != "" {
            *tag = SUMO_TAG_CHARGING_STATION;
            *id = stop.chargingStation.clone();
        } else if stop.overheadWireSegment != "" {
            *tag = SUMO_TAG_OVERHEAD_WIRE_SEGMENT;
            *id = stop.overheadWireSegment.clone();
        }
        ok
    }

    /// C++: `std::string SUMOVehicleParserHelper::parseID(const SUMOSAXAttributes &, const SumoXMLTag)` [static, private]
    /// parse ID
    // std::string
    // SUMOVehicleParserHelper::parseID(const SUMOSAXAttributes& attrs, const SumoXMLTag element) {
    //     bool ok = true;
    //     std::string id;
    //     // first check if attrs contain an ID
    //     if (attrs.hasAttribute(SUMO_ATTR_ID)) {
    //         id = attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
    //         if (SUMOXMLDefinitions::isValidVehicleID(id)) {
    //             return id;
    //         } else if (id.empty()) {
    //             // add extra information for empty IDs
    //             WRITE_ERRORF(TL("Invalid % id '%'."), toString(element), id);
    //         } else {
    //             WRITE_ERRORF(TL("Invalid % id '%'. Contains invalid characters."), toString(element), id);
    //         }
    //     } else {
    //         WRITE_ERROR("Attribute '" + toString(SUMO_ATTR_ID) + "' is missing in definition of " + toString(element));
    //     }
    //     // return empty (invalid) ID
    //     return "";
    // }
    #[allow(unused_variables)]
    pub fn parse_id(attrs: &SUMOSAXAttributes, element: SumoXMLTag) -> String {
        let mut ok = true;
        let mut id = String::new();
        // first check if attrs contain an ID
        if attrs.has_attribute(SUMO_ATTR_ID) {
            id = attrs.get::<String>(SUMO_ATTR_ID, "", &mut ok, true);
            if is_valid_vehicle_id(&id) {
                return id;
            } else if id.is_empty() {
                // add extra information for empty IDs
                write_error(&format!("Invalid {} id '{}'.", Tags.get_string(element), id));
            } else {
                write_error(&format!("Invalid {} id '{}'. Contains invalid characters.", Tags.get_string(element), id));
            }
        } else {
            write_error(&format!("Attribute '{}' is missing in definition of {}", Attrs.get_string(SUMO_ATTR_ID), Tags.get_string(element)));
        }
        // return empty (invalid) ID
        String::new()
    }

    /// C++: `void SUMOVehicleParserHelper::parseCommonAttributes(const SUMOSAXAttributes &, SUMOVehicleParameter *, SumoXMLTag, const bool)` [static, private]
    /// Parses attributes common to vehicles and flows
    // void
    // SUMOVehicleParserHelper::parseCommonAttributes(const SUMOSAXAttributes& attrs, SUMOVehicleParameter* ret, SumoXMLTag tag, const bool allowInternalRoutes) {
    //     const std::string element = toString(tag);
    //     //ret->refid = attrs.getStringSecure(SUMO_ATTR_REFID, "");
    //     // parse route information
    //     if (attrs.hasAttribute(SUMO_ATTR_ROUTE)) {
    //         bool ok = true;
    //         std::string routeID = attrs.get<std::string>(SUMO_ATTR_ROUTE, ret->id.c_str(), ok);
    //         if (!allowInternalRoutes && isInternalRouteID(routeID)) {
    //             WRITE_WARNINGF(TL("Internal routes receive an ID starting with '!' and must not be referenced in other vehicle or flow definitions. Please remove all references to route '%' in case it is internal."), routeID);
    //         }
    //         ret->routeid = routeID;
    //         if (ok) {
    //             ret->parametersSet |= VEHPARS_ROUTE_SET; // !!! needed?
    //         } else {
    //             handleVehicleError(true, ret);
    //         }
    //     }
    //     // parse type information
    //     if (attrs.hasAttribute(SUMO_ATTR_TYPE)) {
    //         bool ok = true;
    //         ret->vtypeid = attrs.get<std::string>(SUMO_ATTR_TYPE, ret->id.c_str(), ok);
    //         if (ok) {
    //             ret->parametersSet |= VEHPARS_VTYPE_SET; // !!! needed?
    //         } else {
    //             handleVehicleError(true, ret);
    //         }
    //     }
    //     // parse line information
    //     if (attrs.hasAttribute(SUMO_ATTR_LINE)) {
    //         bool ok = true;
    //         ret->line = attrs.get<std::string>(SUMO_ATTR_LINE, ret->id.c_str(), ok);
    //         if (ok) {
    //             ret->parametersSet |= VEHPARS_LINE_SET; // !!! needed?
    //         } else {
    //             handleVehicleError(true, ret);
    //         }
    //     }
    //     // parse zone information
    //     if (attrs.hasAttribute(SUMO_ATTR_FROM_TAZ)) {
    //         bool ok = true;
    //         ret->fromTaz = attrs.get<std::string>(SUMO_ATTR_FROM_TAZ, ret->id.c_str(), ok);
    //         if (ok) {
    //             ret->parametersSet |= VEHPARS_FROM_TAZ_SET;
    //         } else {
    //             handleVehicleError(true, ret);
    //         }
    //     }
    //     if (attrs.hasAttribute(SUMO_ATTR_TO_TAZ)) {
    //         bool ok = true;
    //         ret->toTaz = attrs.get<std::string>(SUMO_ATTR_TO_TAZ, ret->id.c_str(), ok);
    //         if (ok) {
    //             ret->parametersSet |= VEHPARS_TO_TAZ_SET;
    //         } else {
    //             handleVehicleError(true, ret);
    //         }
    //     }
    //     // parse reroute information
    //     if (attrs.hasAttribute(SUMO_ATTR_REROUTE)) {
    //         bool ok = true;
    //         if (attrs.get<bool>(SUMO_ATTR_REROUTE, ret->id.c_str(), ok)) {
    //             if (ok) {
    //                 ret->parametersSet |= VEHPARS_FORCE_REROUTE;
    //             } else {
    //                 handleVehicleError(true, ret);
    //             }
    //         }
    //     }
    //     // parse depart lane information
    //     if (attrs.hasAttribute(SUMO_ATTR_DEPARTLANE)) {
    //         bool ok = true;
    //         const std::string departLaneStr = attrs.get<std::string>(SUMO_ATTR_DEPARTLANE, ret->id.c_str(), ok);
    //         int lane;
    //         DepartLaneDefinition dld;
    //         std::string error;
    //         if (SUMOVehicleParameter::parseDepartLane(departLaneStr, element, ret->id, lane, dld, error)) {
    //             ret->parametersSet |= VEHPARS_DEPARTLANE_SET;
    //             ret->departLane = lane;
    //             ret->departLaneProcedure = dld;
    //         } else {
    //             handleVehicleError(true, ret, error);
    //         }
    //     }
    //     // parse depart position information
    //     if (attrs.hasAttribute(SUMO_ATTR_DEPARTPOS)) {
    //         bool ok = true;
    //         const std::string departPosStr = attrs.get<std::string>(SUMO_ATTR_DEPARTPOS, ret->id.c_str(), ok);
    //         double pos;
    //         DepartPosDefinition dpd;
    //         std::string error;
    //         if (SUMOVehicleParameter::parseDepartPos(departPosStr, element, ret->id, pos, dpd, error)) {
    //             ret->parametersSet |= VEHPARS_DEPARTPOS_SET;
    //             ret->departPos = pos;
    //             ret->departPosProcedure = dpd;
    //         } else {
    //             handleVehicleError(true, ret, error);
    //         }
    //     }
    //     // parse lateral depart position information
    //     if (attrs.hasAttribute(SUMO_ATTR_DEPARTPOS_LAT)) {
    //         bool ok = true;
    //         const std::string departPosLatStr = attrs.get<std::string>(SUMO_ATTR_DEPARTPOS_LAT, ret->id.c_str(), ok);
    //         double pos;
    //         DepartPosLatDefinition dpd;
    //         std::string error;
    //         if (SUMOVehicleParameter::parseDepartPosLat(departPosLatStr, element, ret->id, pos, dpd, error)) {
    //             ret->parametersSet |= VEHPARS_DEPARTPOSLAT_SET;
    //             ret->departPosLat = pos;
    //             ret->departPosLatProcedure = dpd;
    //         } else {
    //             handleVehicleError(true, ret, error);
    //         }
    //     }
    //     // parse depart speed information
    //     if (attrs.hasAttribute(SUMO_ATTR_DEPARTSPEED)) {
    //         bool ok = true;
    //         const std::string departSpeed = attrs.get<std::string>(SUMO_ATTR_DEPARTSPEED, ret->id.c_str(), ok);
    //         double speed;
    //         DepartSpeedDefinition dsd;
    //         std::string error;
    //         if (SUMOVehicleParameter::parseDepartSpeed(departSpeed, element, ret->id, speed, dsd, error)) {
    //             ret->parametersSet |= VEHPARS_DEPARTSPEED_SET;
    //             ret->departSpeed = speed;
    //             ret->departSpeedProcedure = dsd;
    //         } else {
    //             handleVehicleError(true, ret, error);
    //         }
    //     }
    //     // parse depart edge information
    //     if (attrs.hasAttribute(SUMO_ATTR_DEPARTEDGE)) {
    //         bool ok = true;
    //         const std::string departEdgeStr = attrs.get<std::string>(SUMO_ATTR_DEPARTEDGE, ret->id.c_str(), ok);
    //         int edgeIndex;
    //         RouteIndexDefinition rid;
    //         std::string error;
    //         if (SUMOVehicleParameter::parseRouteIndex(departEdgeStr, element, ret->id, SUMO_ATTR_DEPARTEDGE, edgeIndex, rid, error)) {
    //             ret->parametersSet |= VEHPARS_DEPARTEDGE_SET;
    //             ret->departEdge = edgeIndex;
    //             ret->departEdgeProcedure = rid;
    //         } else {
    //             handleVehicleError(true, ret, error);
    //         }
    //     }
    //     // parse arrival lane information
    //     if (attrs.hasAttribute(SUMO_ATTR_ARRIVALLANE)) {
    //         bool ok = true;
    //         const std::string arrivalLaneStr = attrs.get<std::string>(SUMO_ATTR_ARRIVALLANE, ret->id.c_str(), ok);
    //         int lane;
    //         ArrivalLaneDefinition ald;
    //         std::string error;
    //         if (SUMOVehicleParameter::parseArrivalLane(arrivalLaneStr, element, ret->id, lane, ald, error)) {
    //             ret->parametersSet |= VEHPARS_ARRIVALLANE_SET;
    //             ret->arrivalLane = lane;
    //             ret->arrivalLaneProcedure = ald;
    //         } else {
    //             handleVehicleError(true, ret, error);
    //         }
    //     }
    //     // parse arrival position information
    //     if (attrs.hasAttribute(SUMO_ATTR_ARRIVALPOS)) {
    //         bool ok = true;
    //         const std::string arrivalPosStr = attrs.get<std::string>(SUMO_ATTR_ARRIVALPOS, ret->id.c_str(), ok);
    //         double pos;
    //         ArrivalPosDefinition apd;
    //         std::string error;
    //         if (SUMOVehicleParameter::parseArrivalPos(arrivalPosStr, element, ret->id, pos, apd, error)) {
    //             ret->parametersSet |= VEHPARS_ARRIVALPOS_SET;
    //             ret->arrivalPos = pos;
    //             ret->arrivalPosProcedure = apd;
    //         } else {
    //             handleVehicleError(true, ret, error);
    //         }
    //     }
    //     // parse lateral arrival position information
    //     if (attrs.hasAttribute(SUMO_ATTR_ARRIVALPOS_LAT)) {
    //         bool ok = true;
    //         const std::string arrivalPosLatStr = attrs.get<std::string>(SUMO_ATTR_ARRIVALPOS_LAT, ret->id.c_str(), ok);
    //         double pos;
    //         ArrivalPosLatDefinition apd;
    //         std::string error;
    //         if (SUMOVehicleParameter::parseArrivalPosLat(arrivalPosLatStr, element, ret->id, pos, apd, error)) {
    //             ret->parametersSet |= VEHPARS_ARRIVALPOSLAT_SET;
    //             ret->arrivalPosLat = pos;
    //             ret->arrivalPosLatProcedure = apd;
    //         } else {
    //             handleVehicleError(true, ret, error);
    //         }
    //     }
    //     // parse arrival speed information
    //     if (attrs.hasAttribute(SUMO_ATTR_ARRIVALSPEED)) {
    //         bool ok = true;
    //         std::string arrivalSpeedStr = attrs.get<std::string>(SUMO_ATTR_ARRIVALSPEED, ret->id.c_str(), ok);
    //         double speed;
    //         ArrivalSpeedDefinition asd;
    //         std::string error;
    //         if (SUMOVehicleParameter::parseArrivalSpeed(arrivalSpeedStr, element, ret->id, speed, asd, error)) {
    //             ret->parametersSet |= VEHPARS_ARRIVALSPEED_SET;
    //             ret->arrivalSpeed = speed;
    //             ret->arrivalSpeedProcedure = asd;
    //         } else {
    //             handleVehicleError(true, ret, error);
    //         }
    //     }
    //     // parse arrival edge information
    //     if (attrs.hasAttribute(SUMO_ATTR_ARRIVALEDGE)) {
    //         bool ok = true;
    //         std::string arrivalEdgeStr = attrs.get<std::string>(SUMO_ATTR_ARRIVALEDGE, ret->id.c_str(), ok);
    //         int edgeIndex;
    //         RouteIndexDefinition rid;
    //         std::string error;
    //         if (SUMOVehicleParameter::parseRouteIndex(arrivalEdgeStr, element, ret->id, SUMO_ATTR_ARRIVALEDGE, edgeIndex, rid, error)) {
    //             ret->parametersSet |= VEHPARS_ARRIVALEDGE_SET;
    //             ret->arrivalEdge = edgeIndex;
    //             ret->arrivalEdgeProcedure = rid;
    //         } else {
    //             handleVehicleError(true, ret, error);
    //         }
    //     }
    //     // parse color
    //     if (attrs.hasAttribute(SUMO_ATTR_COLOR)) {
    //         bool ok = true;
    //         ret->color = attrs.get<RGBColor>(SUMO_ATTR_COLOR, ret->id.c_str(), ok);
    //         if (ok) {
    //             ret->parametersSet |= VEHPARS_COLOR_SET;
    //         } else {
    //             handleVehicleError(true, ret, "Invalid RGBColor format");
    //         }
    //     } else {
    //         ret->color = RGBColor::DEFAULT_COLOR;
    //     }
    //     // parse person number
    //     if (attrs.hasAttribute(SUMO_ATTR_PERSON_NUMBER)) {
    //         bool ok = true;
    //         int personNumber = attrs.get<int>(SUMO_ATTR_PERSON_NUMBER, ret->id.c_str(), ok);
    //         if (!ok) {
    //             handleVehicleError(true, ret);
    //         } else if (personNumber >= 0) {
    //             ret->parametersSet |= VEHPARS_PERSON_NUMBER_SET;
    //             ret->personNumber = personNumber;
    //         } else {
    //             handleVehicleError(true, ret, toString(SUMO_ATTR_PERSON_NUMBER) + " cannot be negative");
    //         }
    //     }
    //     // parse container number
    //     if (attrs.hasAttribute(SUMO_ATTR_CONTAINER_NUMBER)) {
    //         bool ok = true;
    //         int containerNumber = attrs.get<int>(SUMO_ATTR_CONTAINER_NUMBER, ret->id.c_str(), ok);
    //         if (!ok) {
    //             handleVehicleError(true, ret);
    //         } else if (containerNumber >= 0) {
    //             ret->parametersSet |= VEHPARS_CONTAINER_NUMBER_SET;
    //             ret->containerNumber = containerNumber;
    //         } else {
    //             handleVehicleError(true, ret, toString(SUMO_ATTR_CONTAINER_NUMBER) + " cannot be negative");
    //         }
    //     }
    //     // parse individual speedFactor
    //     if (attrs.hasAttribute(SUMO_ATTR_SPEEDFACTOR)) {
    //         bool ok = true;
    //         double speedFactor = attrs.get<double>(SUMO_ATTR_SPEEDFACTOR, ret->id.c_str(), ok);
    //         if (!ok) {
    //             handleVehicleError(true, ret);
    //         } else if (speedFactor > 0) {
    //             ret->parametersSet |= VEHPARS_SPEEDFACTOR_SET;
    //             ret->speedFactor = speedFactor;
    //         } else {
    //             handleVehicleError(true, ret, toString(SUMO_ATTR_SPEEDFACTOR) + " must be positive");
    //         }
    //     }
    //     // parse insertion checks
    //     if (attrs.hasAttribute(SUMO_ATTR_INSERTIONCHECKS)) {
    //         ret->parametersSet |= VEHPARS_INSERTION_CHECKS_SET;
    //         bool ok = true;
    //         std::string checks = attrs.get<std::string>(SUMO_ATTR_INSERTIONCHECKS, ret->id.c_str(), ok);
    //         if (!ok) {
    //             handleVehicleError(true, ret);
    //         } else {
    //             try {
    //                 ret->insertionChecks = SUMOVehicleParameter::parseInsertionChecks(checks);
    //             } catch (InvalidArgument& e) {
    //                 handleVehicleError(true, ret, e.what());
    //             }
    //         }
    //     }
    //     // parse parking access rights
    //     if (attrs.hasAttribute(SUMO_ATTR_PARKING_BADGES)) {
    //         bool ok = true;
    //         std::vector<std::string> badges = attrs.get<std::vector<std::string>>(SUMO_ATTR_PARKING_BADGES, ret->id.c_str(), ok);
    //         if (!ok) {
    //             handleVehicleError(true, ret);
    //         } else {
    //             ret->parametersSet |= VEHPARS_PARKING_BADGES_SET;
    //             ret->parkingBadges = badges;
    //         }
    //     }
    //     // parse modes (transportables only)
    //     ret->modes = 0;
    //     if (attrs.hasAttribute(SUMO_ATTR_MODES)) {
    //         bool ok = true;
    //         const std::string modeString = attrs.get<std::string>(SUMO_ATTR_MODES, ret->id.c_str(), ok);
    //         if (!ok) {
    //             handleVehicleError(true, ret);
    //         } else {
    //             std::string errorMsg;
    //             if (!SUMOVehicleParameter::parsePersonModes(modeString, toString(tag), ret->id, ret->modes, errorMsg)) {
    //                 handleVehicleError(true, ret, errorMsg);
    //             }
    //         }
    //     }
    //     // parse usable vehicle types (transportables only)
    //     if (attrs.hasAttribute(SUMO_ATTR_VTYPES)) {
    //         bool ok = true;
    //         ret->vTypes = attrs.get<std::string>(SUMO_ATTR_VTYPES, ret->id.c_str(), ok);
    //         if (!ok) {
    //             handleVehicleError(true, ret);
    //         }
    //     }
    //     // parse speed (only used by calibrators flow)
    //     // also used by vehicle in saved state but this is parsed elsewhere
    //     if (tag == SUMO_TAG_FLOW && attrs.hasAttribute(SUMO_ATTR_SPEED)) {
    //         bool ok = true;
    //         double calibratorSpeed = attrs.get<double>(SUMO_ATTR_SPEED, ret->id.c_str(), ok);
    //         if (!ok) {
    //             handleVehicleError(true, ret);
    //         } else if (calibratorSpeed >= 0 || calibratorSpeed == -1) {
    //             ret->parametersSet |= VEHPARS_CALIBRATORSPEED_SET;
    //             ret->calibratorSpeed = calibratorSpeed;
    //         } else {
    //             handleVehicleError(true, ret, toString(SUMO_ATTR_SPEED) + " may not be negative");
    //         }
    //     }
    //     /*/ parse via
    //     if (attrs.hasAttribute(SUMO_ATTR_VIA)) {
    //         ret->setParameter |= VEHPARS_VIA_SET;
    //         SUMOSAXAttributes::parseStringVector(attrs.get<std::string>(SUMO_ATTR_VIA, ret->id.c_str(), ok), ret->via);
    //     }
    //     */
    // }
    #[allow(unused_variables)]
    pub fn parse_common_attributes(attrs: &SUMOSAXAttributes, ret: Ptr<SUMOVehicleParameter>, tag: SumoXMLTag, allowInternalRoutes: bool) {
        todo!("port SUMOVehicleParserHelper::parseCommonAttributes")
    }

    /// C++: `SUMOVehicleParameter * SUMOVehicleParserHelper::handleVehicleError(const bool, SUMOVehicleParameter *, const std::string)` [static, private]
    /// handle error loading SUMOVehicleParameter
    // SUMOVehicleParameter*
    // SUMOVehicleParserHelper::handleVehicleError(const bool hardFail, SUMOVehicleParameter* vehicleParameter, const std::string message) {
    //     if (vehicleParameter) {
    //         delete vehicleParameter;
    //     }
    //     if (hardFail) {
    //         throw ProcessError(message);
    //     } else if (message.size() > 0) {
    //         WRITE_ERROR(message);
    //     }
    //     return nullptr;
    // }
    #[allow(unused_variables)]
    pub fn handle_vehicle_error(hardFail: bool, vehicleParameter: Ptr<SUMOVehicleParameter>, message: String) -> Ptr<SUMOVehicleParameter> {
        if !vehicleParameter.is_null() {
            drop(unsafe { Box::from_raw(vehicleParameter.as_raw()) });
        }
        if hardFail {
            throw_process_error(message);
        } else if message.len() > 0 {
            write_error(&message);
        }
        Ptr::null()
    }

    /// C++: `SUMOVTypeParameter * SUMOVehicleParserHelper::handleVehicleTypeError(const bool, SUMOVTypeParameter *, const std::string)` [static, private]
    /// handle error loading SUMOVTypeParameter
    // SUMOVTypeParameter*
    // SUMOVehicleParserHelper::handleVehicleTypeError(const bool hardFail, SUMOVTypeParameter* vehicleTypeParameter, const std::string message) {
    //     if (vehicleTypeParameter) {
    //         delete vehicleTypeParameter;
    //     }
    //     if (hardFail) {
    //         throw ProcessError(message);
    //     } else if (message.size() > 0) {
    //         WRITE_ERROR(message);
    //     }
    //     return nullptr;
    // }
    #[allow(unused_variables)]
    pub fn handle_vehicle_type_error(hardFail: bool, vehicleTypeParameter: Ptr<SUMOVTypeParameter>, message: String) -> Ptr<SUMOVTypeParameter> {
        if !vehicleTypeParameter.is_null() {
            drop(unsafe { Box::from_raw(vehicleTypeParameter.as_raw()) });
        }
        if hardFail {
            throw_process_error(message);
        } else if message.len() > 0 {
            write_error(&message);
        }
        Ptr::null()
    }

    /// C++: `const CFAttrMap & SUMOVehicleParserHelper::getAllowedCFModelAttrs()` [static, private]
    /// returns allowed attrs for each known CF-model (init on first use)
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_allowed_cf_model_attrs() -> &'static SUMOVehicleParserHelper_CFAttrMap {
        todo!("port SUMOVehicleParserHelper::getAllowedCFModelAttrs")
    }

}

