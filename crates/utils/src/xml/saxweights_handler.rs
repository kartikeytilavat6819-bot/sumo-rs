//! Port of utils/xml/SAXWeightsHandler.{h,cpp}
use super::generic_sax_handler::{GenericSAXHandler, SaxHandler};
use super::sumo_sax_attributes::SUMOSAXAttributes;
use super::sumo_sax_handler;
use super::sumo_xml_definitions::{SUMO_ATTR_BEGIN, SUMO_ATTR_END, SUMO_ATTR_FROM, SUMO_ATTR_ID, SUMO_ATTR_TO, SUMO_TAG_EDGE, SUMO_TAG_EDGEREL, SUMO_TAG_INTERVAL, SUMO_TAG_LANE, SUMO_TAG_TAZREL};
use crate::common::msg_handler::write_error;
use crate::common::time::{steps2time, time2steps, time2string};
use crate::common::to_string::to_string;
use crate::common::util_exceptions::ProcessError;

/// C++ `SAXWeightsHandler::EdgeFloatTimeLineRetriever`: obtains the weights read for named edges.
/// The C++ default bodies are empty, so they are default methods here.
pub trait EdgeFloatTimeLineRetriever {
    /// C++ `addEdgeWeight(const std::string&, double, double, double) const`
    fn add_edge_weight(&self, _id: &str, _val: f64, _beg: f64, _end: f64) {}

    /// C++ `addEdgeRelWeight(const std::string&, const std::string&, double, double, double) const`
    fn add_edge_rel_weight(&self, _from: &str, _to: &str, _val: f64, _beg: f64, _end: f64) {}

    /// C++ `addTazRelWeight(const std::string, const std::string&, const std::string&, double, double, double)` (non-const)
    fn add_taz_rel_weight(&mut self, _interval_id: &str, _from: &str, _to: &str, _val: f64, _beg: f64, _end: f64) {}
}

/// C++ `SAXWeightsHandler::ToRetrieveDefinition`: what shall be retrieved and where it is stored.
/// The destination is borrowed for the lifetime of the handler (C++ keeps a reference, not a copy).
pub struct ToRetrieveDefinition<'a> {
    /// The attribute name that shall be parsed
    pub myAttributeName: String,
    /// Information whether edge values shall be used (lane value if false)
    pub myAmEdgeBased: bool,
    /// The class that shall be called when new data is avaiable
    pub myDestination: &'a mut dyn EdgeFloatTimeLineRetriever,
    /// Aggregated value over the lanes read within the current edge
    pub myAggValue: f64,
    /// The number of lanes read for the current edge
    pub myNoLanes: i32,
    /// Information whether the attribute has been found for the current edge
    pub myHadAttribute: bool,
    /// Information whether the attribute was found to contain non-numerical data (for any edge)
    pub myHadNonNumeric: bool,
}

impl<'a> ToRetrieveDefinition<'a> {
    /// C++: `SAXWeightsHandler::ToRetrieveDefinition::ToRetrieveDefinition(const std::string&, bool, EdgeFloatTimeLineRetriever&)`
    pub fn new(attributeName: &str, edgeBased: bool, destination: &'a mut dyn EdgeFloatTimeLineRetriever) -> Self {
        ToRetrieveDefinition {
            myAttributeName: attributeName.to_string(),
            myAmEdgeBased: edgeBased,
            myDestination: destination,
            myAggValue: 0.,
            myNoLanes: 0,
            myHadAttribute: false,
            myHadNonNumeric: false,
        }
    }
}

/// An XML-handler for network weights
/// C++ class `SAXWeightsHandler` (utils/xml/SAXWeightsHandler.h)
pub struct SAXWeightsHandler<'a> {
    /// C++ base class `SUMOSAXHandler`
    pub base: GenericSAXHandler,
    /// List of definitions what shall be read and whereto stored while parsing the file
    pub myDefinitions: Vec<ToRetrieveDefinition<'a>>,
    /// the id of the interval being parsed
    pub myCurrentID: String,
    /// the begin of the time period that is currently processed
    pub myCurrentTimeBeg: f64,
    /// the end of the time period that is currently processed
    pub myCurrentTimeEnd: f64,
    /// the edge which is currently being processed
    pub myCurrentEdgeID: String,
}

impl<'a> SAXWeightsHandler<'a> {
    /// C++: `SAXWeightsHandler::SAXWeightsHandler(const std::vector<ToRetrieveDefinition*>&, const std::string&)`
    pub fn new(defs: Vec<ToRetrieveDefinition<'a>>, file: &str) -> Self {
        SAXWeightsHandler {
            base: sumo_sax_handler::new(file, ""),
            myDefinitions: defs,
            myCurrentID: String::new(),
            myCurrentTimeBeg: -1.,
            myCurrentTimeEnd: -1.,
            myCurrentEdgeID: String::new(),
        }
    }

    /// C++: `SAXWeightsHandler::SAXWeightsHandler(ToRetrieveDefinition*, const std::string&)`
    pub fn new_2(def: ToRetrieveDefinition<'a>, file: &str) -> Self {
        Self::new(vec![def], file)
    }

    /// C++: `void SAXWeightsHandler::myStartElement(int, const SUMOSAXAttributes&)`
    pub fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) -> Result<(), ProcessError> {
        match element {
            SUMO_TAG_INTERVAL => {
                let mut ok = true;
                self.myCurrentID = attrs.get_opt::<String>(SUMO_ATTR_ID, "", &mut ok, String::new(), true);
                self.myCurrentTimeBeg = steps2time(attrs.get_sumo_time_reporting(SUMO_ATTR_BEGIN, "", &mut ok, true));
                self.myCurrentTimeEnd = steps2time(attrs.get_sumo_time_reporting(SUMO_ATTR_END, "", &mut ok, true));
                if self.myCurrentTimeEnd < self.myCurrentTimeBeg {
                    write_error(&format!(
                        "Interval end time {} is lower than interval begin time {}",
                        to_string(&self.myCurrentTimeEnd),
                        to_string(&self.myCurrentTimeBeg)
                    ));
                    self.myCurrentTimeEnd = self.myCurrentTimeBeg;
                }
            }
            SUMO_TAG_EDGE => {
                let mut ok = true;
                self.myCurrentEdgeID = attrs.get_opt::<String>(SUMO_ATTR_ID, "", &mut ok, String::new(), true);
                self.try_parse(attrs, true)?;
            }
            SUMO_TAG_EDGEREL => {
                self.try_parse_edge_rel(attrs)?;
            }
            SUMO_TAG_TAZREL => {
                self.try_parse_taz_rel(attrs)?;
            }
            SUMO_TAG_LANE => {
                self.try_parse(attrs, false)?;
            }
            _ => {}
        }
        Ok(())
    }

    /// C++: `void SAXWeightsHandler::tryParse(const SUMOSAXAttributes&, bool)`
    fn try_parse(&mut self, attrs: &SUMOSAXAttributes, isEdge: bool) -> Result<(), ProcessError> {
        if isEdge {
            // process all that want values directly from the edge
            for definition in self.myDefinitions.iter_mut() {
                if definition.myAmEdgeBased {
                    if attrs.has_attribute_name(&definition.myAttributeName) {
                        match attrs.get_float_name(&definition.myAttributeName) {
                            Ok(v) => {
                                definition.myAggValue = v;
                                definition.myNoLanes = 1;
                                definition.myHadAttribute = true;
                            }
                            Err(ProcessError::EmptyData) => {
                                crate::write_errorf!("Missing value '%' in edge '%'.", definition.myAttributeName, self.myCurrentEdgeID);
                            }
                            Err(ProcessError::NumberFormat(_)) => {
                                if !definition.myHadNonNumeric {
                                    // warn only once
                                    definition.myHadNonNumeric = true;
                                    crate::write_errorf!(
                                        "The value '%' of attribute '%' should be numeric in edge '%' at time step %.",
                                        attrs.get_string_secure_name(&definition.myAttributeName, ""),
                                        definition.myAttributeName,
                                        self.myCurrentEdgeID,
                                        time2string(time2steps(self.myCurrentTimeBeg))
                                    );
                                }
                            }
                            Err(e) => return Err(e),
                        }
                    } else {
                        definition.myHadAttribute = false;
                    }
                } else {
                    definition.myAggValue = 0.;
                    definition.myNoLanes = 0;
                }
            }
        } else {
            // process the current lane values
            for definition in self.myDefinitions.iter_mut() {
                if !definition.myAmEdgeBased {
                    match attrs.get_float_name(&definition.myAttributeName) {
                        Ok(v) => {
                            definition.myAggValue += v;
                            definition.myNoLanes += 1;
                            definition.myHadAttribute = true;
                        }
                        Err(ProcessError::EmptyData) => {
                            crate::write_errorf!("Missing value '%' in edge '%'.", definition.myAttributeName, self.myCurrentEdgeID);
                        }
                        Err(ProcessError::NumberFormat(_)) => {
                            if !definition.myHadNonNumeric {
                                // warn only once
                                definition.myHadNonNumeric = true;
                                crate::write_errorf!(
                                    "The value '%' of attribute '%' should be numeric in edge '%' at time step %.",
                                    attrs.get_string_secure_name(&definition.myAttributeName, ""),
                                    definition.myAttributeName,
                                    self.myCurrentEdgeID,
                                    time2string(time2steps(self.myCurrentTimeBeg))
                                );
                            }
                        }
                        Err(e) => return Err(e),
                    }
                }
            }
        }
        Ok(())
    }

    /// C++: `void SAXWeightsHandler::tryParseEdgeRel(const SUMOSAXAttributes&)`
    fn try_parse_edge_rel(&mut self, attrs: &SUMOSAXAttributes) -> Result<(), ProcessError> {
        if attrs.has_attribute(SUMO_ATTR_FROM) && attrs.has_attribute(SUMO_ATTR_TO) {
            let mut ok = true;
            let from = attrs.get::<String>(SUMO_ATTR_FROM, "", &mut ok, true);
            let to = attrs.get::<String>(SUMO_ATTR_TO, "", &mut ok, true);
            for ret in self.myDefinitions.iter_mut() {
                if attrs.has_attribute_name(&ret.myAttributeName) {
                    let v = attrs.get_float_name(&ret.myAttributeName)?;
                    ret.myDestination.add_edge_rel_weight(&from, &to, v, self.myCurrentTimeBeg, self.myCurrentTimeEnd);
                }
            }
        }
        Ok(())
    }

    /// C++: `void SAXWeightsHandler::tryParseTazRel(const SUMOSAXAttributes&)`
    fn try_parse_taz_rel(&mut self, attrs: &SUMOSAXAttributes) -> Result<(), ProcessError> {
        if attrs.has_attribute(SUMO_ATTR_FROM) && attrs.has_attribute(SUMO_ATTR_TO) {
            let mut ok = true;
            let from = attrs.get::<String>(SUMO_ATTR_FROM, "", &mut ok, true);
            let to = attrs.get::<String>(SUMO_ATTR_TO, "", &mut ok, true);
            for ret in self.myDefinitions.iter_mut() {
                if attrs.has_attribute_name(&ret.myAttributeName) {
                    let v = attrs.get_float_name(&ret.myAttributeName)?;
                    ret.myDestination.add_taz_rel_weight(&self.myCurrentID, &from, &to, v, self.myCurrentTimeBeg, self.myCurrentTimeEnd);
                }
            }
        }
        Ok(())
    }

    /// C++: `void SAXWeightsHandler::myEndElement(int)`
    pub fn my_end_element(&mut self, element: i32) {
        if element == SUMO_TAG_EDGE {
            for definition in self.myDefinitions.iter() {
                if definition.myHadAttribute {
                    definition.myDestination.add_edge_weight(
                        &self.myCurrentEdgeID,
                        definition.myAggValue / (definition.myNoLanes as f64),
                        self.myCurrentTimeBeg,
                        self.myCurrentTimeEnd,
                    );
                }
            }
        }
    }
}

impl<'a> SaxHandler for SAXWeightsHandler<'a> {
    fn handler(&mut self) -> &mut GenericSAXHandler {
        &mut self.base
    }
    fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) -> Result<(), ProcessError> {
        SAXWeightsHandler::my_start_element(self, element, attrs)
    }
    fn my_end_element(&mut self, element: i32) -> Result<(), ProcessError> {
        SAXWeightsHandler::my_end_element(self, element);
        Ok(())
    }
}
