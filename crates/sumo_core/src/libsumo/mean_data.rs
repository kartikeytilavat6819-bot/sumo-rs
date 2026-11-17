//! Port of libsumo/MeanData.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use crate::libsumo::helper::Helper;
use crate::libsumo::storage_helper::StorageHelper;
use crate::libsumo::tra_ci_defs::*;
use utils::common::msg_handler::write_warning;
use utils::common::string_bijection::StringBijection;
use utils::traci::traci_constants::*;
use utils::xml::sumo_xml_definitions::{Attrs, SumoXMLAttr};
use std::cell::RefCell;

/// C++ class `libsumo::MeanData` (libsumo/MeanData.h)
pub struct MeanData {
}

// C++ static members of MeanData (port as module-level statics, see AGENTS.md):
//   static const int MeanData::DOMAIN_ID;
//   static SubscriptionResults MeanData::mySubscriptionResults;
//   static ContextSubscriptionResults MeanData::myContextSubscriptionResults;

// LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(MeanData, MEANDATA): const int MeanData::DOMAIN_ID(CMD_GET_MEANDATA_VARIABLE)
pub const MEAN_DATA_DOMAIN_ID: i32 = CMD_GET_MEANDATA_VARIABLE;
thread_local! {
    static MY_SUBSCRIPTION_RESULTS: RefCell<SubscriptionResults> = RefCell::new(SubscriptionResults::new());
    static MY_CONTEXT_SUBSCRIPTION_RESULTS: RefCell<ContextSubscriptionResults> = RefCell::new(ContextSubscriptionResults::new());
}

// shortcut: C++ throws libsumo::TraCIException; panics with the same message until TraCI error propagation is ported, upgrade when Result-based libsumo lands
fn throw_traci(msg: String) -> ! {
    panic!("{}", msg)
}

impl MeanData {
    /// C++: `double MeanData::getAttributeValue(const std::string &, const std::string &, const std::string &)` [static]
    // double
    // MeanData::getAttributeValue(const std::string& meanDataID, const std::string& laneID, const std::string& attr) {
    //     if (!SUMOXMLDefinitions::Attrs.hasString(attr)) {
    //         throw TraCIException("Unknown Meandata attribute '" + attr + "'.");
    //     }
    //     MSLane* const lane = MSLane::dictionary(laneID);
    //     if (lane == nullptr) {
    //         throw TraCIException("Lane '" + laneID + "' is not known");
    //     }
    //     MSMeanData* md = getMeanData(meanDataID);
    //     SumoXMLAttr a = (SumoXMLAttr)SUMOXMLDefinitions::Attrs.get(attr);
    //     return md->getAttributeValue(lane, a, INVALID_DOUBLE_VALUE);
    // }
    #[allow(unused_variables)]
    pub fn get_attribute_value(meanDataID: &str, laneID: &str, attr: &str) -> f64 {
        if !Attrs.has_string(attr) {
            throw_traci(format!("Unknown Meandata attribute '{}'.", attr));
        }
        let lane = MSLane::dictionary_2(laneID);
        if lane.is_null() {
            throw_traci(format!("Lane '{}' is not known", laneID));
        }
        let md = Self::get_mean_data(meanDataID);
        let a = Attrs.get(attr).unwrap() as SumoXMLAttr;
        md.get_attribute_value(lane, a, INVALID_DOUBLE_VALUE)
    }

    /// C++: `std::vector<std::string> MeanData::getIDs(const std::string &)` [static]
    // std::vector<std::string>
    // MeanData::getIDs(const std::string&meanDataID) {
    //     MSMeanData* md = getMeanData(meanDataID);
    //     std::vector<std::string> result;
    //     if (md->isEdgeData()) {
    //         for (const MSEdge* e : md->getEdges()) {
    //             result.push_back(e->getID());
    //         }
    //     } else {
    //         for (const MSEdge* e : md->getEdges()) {
    //             for (const MSLane* lane : e->getLanes()) {
    //                 result.push_back(lane->getID());
    //             }
    //         }
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_i_ds(meanDataID: &str) -> Vec<String> {
        let md = Self::get_mean_data(meanDataID);
        let mut result: Vec<String> = Vec::new();
        if md.is_edge_data() {
            for e in md.get_edges().iter() {
                result.push(e.get_id().to_string());
            }
        } else {
            for e in md.get_edges().iter() {
                for lane in e.get_lanes().iter() {
                    result.push(lane.get_id().to_string());
                }
            }
        }
        result
    }

    /// C++: `std::vector<double> MeanData::getAttributeValues(const std::string &, const std::string &)` [static]
    // std::vector<double>
    // MeanData::getAttributeValues(const std::string& meanDataID, const std::string& attr) {
    //     if (!SUMOXMLDefinitions::Attrs.hasString(attr)) {
    //         throw TraCIException("Unknown Meandata attribute '" + attr + "'.");
    //     }
    //     MSMeanData* md = getMeanData(meanDataID);
    //     SumoXMLAttr a = (SumoXMLAttr)SUMOXMLDefinitions::Attrs.get(attr);
    //     std::vector<double> result;
    //     if (md->isEdgeData()) {
    //         for (const MSEdge* e : md->getEdges()) {
    //             result.push_back(md->getAttributeValue(e->getLanes().front(), a, INVALID_DOUBLE_VALUE));
    //         }
    //     } else {
    //         for (const MSEdge* e : md->getEdges()) {
    //             for (const MSLane* lane : e->getLanes()) {
    //                 result.push_back(md->getAttributeValue(lane, a, INVALID_DOUBLE_VALUE));
    //             }
    //         }
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_attribute_values(meanDataID: &str, attr: &str) -> Vec<f64> {
        if !Attrs.has_string(attr) {
            throw_traci(format!("Unknown Meandata attribute '{}'.", attr));
        }
        let md = Self::get_mean_data(meanDataID);
        let a = Attrs.get(attr).unwrap() as SumoXMLAttr;
        let mut result: Vec<f64> = Vec::new();
        if md.is_edge_data() {
            for e in md.get_edges().iter() {
                result.push(md.get_attribute_value(e.get_lanes()[0], a, INVALID_DOUBLE_VALUE));
            }
        } else {
            for e in md.get_edges().iter() {
                for lane in e.get_lanes().iter() {
                    result.push(md.get_attribute_value(*lane, a, INVALID_DOUBLE_VALUE));
                }
            }
        }
        result
    }

    /// C++: `std::vector<std::string> MeanData::getIDList()` [static]
    // std::vector<std::string>
    // MeanData::getIDList() {
    //     std::vector<std::string> ids;
    //     for (const auto& item : MSNet::getInstance()->getDetectorControl().getMeanData()) {
    //         ids.push_back(item.first);
    //     }
    //     std::sort(ids.begin(), ids.end());
    //     return ids;
    // }
    #[allow(unused_variables)]
    pub fn get_id_list() -> Vec<String> {
        let net = MSNet::get_instance();
        let mut ids: Vec<String> = Vec::new();
        for (key, _) in net.m().get_detector_control().get_mean_data().iter() {
            ids.push(key.clone());
        }
        ids.sort();
        ids
    }

    /// C++: `int MeanData::getIDCount()` [static]
    // int
    // MeanData::getIDCount() {
    //     return (int)getIDList().size();
    // }
    #[allow(unused_variables)]
    pub fn get_id_count() -> i32 {
        Self::get_id_list().len() as i32
    }

    /// C++: `std::string MeanData::getParameter(const std::string &, const std::string &)` [static]
    // std::string
    // MeanData::getParameter(const std::string& /* dataID */, const std::string& /* param */) {
    //     return "";
    // }
    #[allow(unused_variables)]
    pub fn get_parameter(objectID: &str, key: &str) -> String {
        String::new()
    }

    /// C++: `const std::pair<std::string, std::string> MeanData::getParameterWithKey(const std::string &, const std::string &)` [static]
    // LIBSUMO_GET_PARAMETER_WITH_KEY_IMPLEMENTATION(MeanData)
    #[allow(unused_variables)]
    pub fn get_parameter_with_key(objectID: &str, key: &str) -> (String, String) {
        (key.to_string(), Self::get_parameter(objectID, key))
    }

    /// C++: `void MeanData::setParameter(const std::string &, const std::string &, const std::string &)` [static]
    // void
    // MeanData::setParameter(const std::string& /* dataID */, const std::string& /* key */, const std::string& /* value */) {
    //     //MSMeanData* r = const_cast<MSMeanData*>(getMeanData(dataID));
    //     //r->setParameter(key, value);
    // }
    #[allow(unused_variables)]
    pub fn set_parameter(objectID: &str, key: &str, value: &str) {
    }

    /// C++: `void MeanData::subscribe(const std::string &, const std::vector<int> &, double, double, const libsumo::TraCIResults &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(MeanData, MEANDATA)
    #[allow(unused_variables)]
    pub fn subscribe(objectID: &str, varIDs: &Vec<i32>, begin: f64, end: f64, parameters: &TraCIResults) {
        Helper::subscribe(CMD_SUBSCRIBE_MEANDATA_VARIABLE, objectID, varIDs, begin, end, parameters, 0, 0.);
    }

    /// C++: `void MeanData::unsubscribe(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(MeanData, MEANDATA)
    #[allow(unused_variables)]
    pub fn unsubscribe(objectID: &str) {
        Helper::subscribe(CMD_SUBSCRIBE_MEANDATA_VARIABLE, objectID, &Vec::new(), INVALID_DOUBLE_VALUE, INVALID_DOUBLE_VALUE, &TraCIResults::new(), 0, 0.);
    }

    /// C++: `void MeanData::subscribeContext(const std::string &, int, double, const std::vector<int> &, double, double, const libsumo::TraCIResults &)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn subscribe_context(objectID: &str, domain: i32, dist: f64, varIDs: &Vec<i32>, begin: f64, end: f64, parameters: &TraCIResults) {
        Helper::subscribe(CMD_SUBSCRIBE_MEANDATA_CONTEXT, objectID, varIDs, begin, end, parameters, domain, dist);
    }

    /// C++: `void MeanData::unsubscribeContext(const std::string &, int, double)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(MeanData, MEANDATA)
    #[allow(unused_variables)]
    pub fn unsubscribe_context(objectID: &str, domain: i32, dist: f64) {
        Helper::subscribe(CMD_SUBSCRIBE_MEANDATA_CONTEXT, objectID, &Vec::new(), INVALID_DOUBLE_VALUE, INVALID_DOUBLE_VALUE, &TraCIResults::new(), domain, dist);
    }

    /// C++: `const libsumo::SubscriptionResults MeanData::getAllSubscriptionResults()` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(MeanData, MEANDATA)
    #[allow(unused_variables)]
    pub fn get_all_subscription_results() -> SubscriptionResults {
        MY_SUBSCRIPTION_RESULTS.with(|r| r.borrow().clone())
    }

    /// C++: `const libsumo::TraCIResults MeanData::getSubscriptionResults(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(MeanData, MEANDATA)
    #[allow(unused_variables)]
    pub fn get_subscription_results(objectID: &str) -> TraCIResults {
        MY_SUBSCRIPTION_RESULTS.with(|r| r.borrow_mut().entry(objectID.to_string()).or_default().clone())
    }

    /// C++: `const libsumo::ContextSubscriptionResults MeanData::getAllContextSubscriptionResults()` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(MeanData, MEANDATA)
    #[allow(unused_variables)]
    pub fn get_all_context_subscription_results() -> ContextSubscriptionResults {
        MY_CONTEXT_SUBSCRIPTION_RESULTS.with(|r| r.borrow().clone())
    }

    /// C++: `const libsumo::SubscriptionResults MeanData::getContextSubscriptionResults(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(MeanData, MEANDATA)
    #[allow(unused_variables)]
    pub fn get_context_subscription_results(objectID: &str) -> SubscriptionResults {
        MY_CONTEXT_SUBSCRIPTION_RESULTS.with(|r| r.borrow_mut().entry(objectID.to_string()).or_default().clone())
    }

    /// C++: `void MeanData::subscribeParameterWithKey(const std::string &, const std::string &, double, double)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(MeanData, MEANDATA)
    #[allow(unused_variables)]
    pub fn subscribe_parameter_with_key(objectID: &str, key: &str, beginTime: f64, endTime: f64) {
        let mut params: TraCIResults = TraCIResults::new();
        params.insert(VAR_PARAMETER_WITH_KEY, crate::libsumo::helper::as_traci_result(TraCIString { base: TraCIResult { result_type: TYPE_STRING }, value: key.to_string() }));
        Helper::subscribe(CMD_SUBSCRIBE_MEANDATA_VARIABLE, objectID, &vec![VAR_PARAMETER_WITH_KEY], beginTime, endTime, &params, 0, 0.0);
    }

    /// C++: `int MeanData::domainID()` [static]
    // LIBSUMO_SUBSCRIPTION_API
    #[allow(unused_variables)]
    pub fn domain_id() -> i32 {
        MEAN_DATA_DOMAIN_ID
    }

    /// C++: `std::shared_ptr<VariableWrapper> MeanData::makeWrapper()` [static]
    // std::shared_ptr<VariableWrapper>
    // MeanData::makeWrapper() {
    //     return std::make_shared<Helper::SubscriptionWrapper>(handleVariable, mySubscriptionResults, myContextSubscriptionResults);
    // }
    #[allow(unused_variables)]
    pub fn make_wrapper() -> Rc<VariableWrapper> {
        Helper::make_subscription_wrapper(Some(Self::handle_variable), unsafe { &mut *MY_SUBSCRIPTION_RESULTS.with(|r| r.as_ptr()) }, unsafe { &mut *MY_CONTEXT_SUBSCRIPTION_RESULTS.with(|r| r.as_ptr()) })
    }

    /// C++: `bool MeanData::handleVariable(const std::string &, const int, VariableWrapper *, tcpip::Storage *)` [static]
    // bool
    // MeanData::handleVariable(const std::string& objID, const int variable, VariableWrapper* wrapper, tcpip::Storage* paramData) {
    //     switch (variable) {
    //         case TRACI_ID_LIST:
    //             return wrapper->wrapStringList(objID, variable, getIDList());
    //         case ID_COUNT:
    //             return wrapper->wrapInt(objID, variable, getIDCount());
    //         case libsumo::VAR_PARAMETER:
    //             paramData->readUnsignedByte();
    //             return wrapper->wrapString(objID, variable, getParameter(objID, paramData->readString()));
    //         case libsumo::VAR_PARAMETER_WITH_KEY:
    //             paramData->readUnsignedByte();
    //             return wrapper->wrapStringPair(objID, variable, getParameterWithKey(objID, paramData->readString()));
    //         case libsumo::VAR_MEANDATA_LANE: {
    //             StoHelp::readCompound(*paramData);
    //             const std::string laneID = StoHelp::readTypedString(*paramData);
    //             const std::string attr = StoHelp::readTypedString(*paramData);
    //             return wrapper->wrapDouble(objID, variable, getAttributeValue(objID, laneID, attr));
    //         }
    //         case libsumo::VAR_MEANDATA_IDS:
    //             return wrapper->wrapStringList(objID, variable, getIDs(objID));
    //         case libsumo::VAR_MEANDATA_VALUES: {
    //             return wrapper->wrapDoubleList(objID, variable, getAttributeValues(objID, StoHelp::readTypedString(*paramData)));
    //         }
    //         default:
    //             return false;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn handle_variable(objID: &str, variable: i32, wrapper: Ptr<VariableWrapper>, paramData: Ptr<Storage>) -> bool {
        let mut w = wrapper;
        let ps = paramData.m();
        match variable {
            TRACI_ID_LIST => w.m().wrap_string_list(objID, variable, &Self::get_id_list()),
            ID_COUNT => w.m().wrap_int(objID, variable, Self::get_id_count()),
            VAR_PARAMETER => {
                ps.read_unsigned_byte().unwrap();
                let key = ps.read_string().unwrap();
                w.m().wrap_string(objID, variable, &Self::get_parameter(objID, &key))
            }
            VAR_PARAMETER_WITH_KEY => {
                ps.read_unsigned_byte().unwrap();
                let key = ps.read_string().unwrap();
                w.m().wrap_string_pair(objID, variable, &Self::get_parameter_with_key(objID, &key))
            }
            VAR_MEANDATA_LANE => {
                StorageHelper::read_compound(ps, -1, "");
                let laneID = StorageHelper::read_typed_string(ps, "");
                let attr = StorageHelper::read_typed_string(ps, "");
                w.m().wrap_double(objID, variable, Self::get_attribute_value(objID, &laneID, &attr))
            }
            VAR_MEANDATA_IDS => w.m().wrap_string_list(objID, variable, &Self::get_i_ds(objID)),
            VAR_MEANDATA_VALUES => {
                let attr = StorageHelper::read_typed_string(ps, "");
                w.m().wrap_double_list(objID, variable, &Self::get_attribute_values(objID, &attr))
            }
            _ => false,
        }
    }

    /// C++: `MSMeanData * MeanData::getMeanData(const std::string &)` [static, private]
    // MSMeanData*
    // MeanData::getMeanData(const std::string& id) {
    //     auto mdMap = MSNet::getInstance()->getDetectorControl().getMeanData();
    //     auto it = mdMap.find(id);
    //     if (it == mdMap.end() || it->second.size() == 0) {
    //         throw TraCIException("MeanData '" + id + "' is not known");
    //     }
    //     if (it->second.size() > 1) {
    //         WRITE_WARNINGF(TL("Found % meanData definitions with id '%'."), toString(it->second.size()), id);
    //     }
    //     return it->second.front();
    // }
    #[allow(unused_variables)]
    pub fn get_mean_data(id: &str) -> Ptr<MSMeanData> {
        let net = MSNet::get_instance();
        let mut n = net.m();
        let mdMap = n.get_detector_control().get_mean_data();
        let it = match mdMap.get(id) {
            Some(v) if !v.is_empty() => v,
            _ => throw_traci(format!("MeanData '{}' is not known", id)),
        };
        if it.len() > 1 {
            write_warning(&format!("Found {} meanData definitions with id '{}'.", it.len(), id));
        }
        it[0].clone()
    }

    /// C++: `void MeanData::MeanData()` [private]
    /// invalidated standard constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new() -> Self {
        unreachable!("libsumo::MeanData has an invalidated (private, undefined) default constructor")
    }

}

