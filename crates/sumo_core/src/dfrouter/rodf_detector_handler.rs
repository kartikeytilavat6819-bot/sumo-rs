//! Port of dfrouter/RODFDetectorHandler.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use utils::xml::generic_sax_handler::GenericSAXHandler;
use crate::prelude::*;
use utils::common::msg_handler::write_warning;
use utils::tlf;

/// SAX2-Handler for loading DFROUTER-detector definitions
/// C++ class `RODFDetectorHandler` (dfrouter/RODFDetectorHandler.h)
#[repr(C)]
pub struct RODFDetectorHandler {
    /// C++ base class `SUMOSAXHandler`
    pub base: GenericSAXHandler,
    /// the net
    pub myNet: Ptr<RODFNet>,
    /// whether to ignore errors on parsing
    pub myIgnoreErrors: bool,
    /// the container to put the detectors into
    pub myContainer: Ptr<RODFDetectorCon>,
}

impl std::ops::Deref for RODFDetectorHandler {
    type Target = GenericSAXHandler;
    fn deref(&self) -> &GenericSAXHandler {
        &self.base
    }
}
impl std::ops::DerefMut for RODFDetectorHandler {
    fn deref_mut(&mut self) -> &mut GenericSAXHandler {
        &mut self.base
    }
}

impl RODFDetectorHandler {
    /// C++: `void RODFDetectorHandler::RODFDetectorHandler(RODFNet *, bool, RODFDetectorCon &, const std::string &)`
    /// Constructor
    // RODFDetectorHandler::RODFDetectorHandler(RODFNet* optNet, bool ignoreErrors, RODFDetectorCon& con,
    //         const std::string& file)
    //     : SUMOSAXHandler(file),
    //       myNet(optNet), myIgnoreErrors(ignoreErrors), myContainer(con) {}
    #[allow(unused_variables)]
    pub fn new(optNet: Ptr<RODFNet>, ignoreErrors: bool, con: &mut RODFDetectorCon, file: &str) -> Self {
        RODFDetectorHandler {
            base: GenericSAXHandler::new(&TAGS, SUMO_TAG_NOTHING, &ATTRS, SUMO_ATTR_NOTHING, file, ""),
            myNet: optNet,
            myIgnoreErrors: ignoreErrors,
            myContainer: Ptr::from_mut(con),
        }
    }

    /// C++: `void RODFDetectorHandler::~RODFDetectorHandler() noexcept` [virtual]
    /// Destructor
    // RODFDetectorHandler::~RODFDetectorHandler() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        {
            // Destructor logic here if needed
        }
    }

    /// C++: `void RODFDetectorHandler::myStartElement(int, const SUMOSAXAttributes &)` [virtual, protected]
    /// Called on the opening of a tag;
    // void
    // RODFDetectorHandler::myStartElement(int element,
    //                                     const SUMOSAXAttributes& attrs) {
    //     if (element == SUMO_TAG_DETECTOR_DEFINITION || element == SUMO_TAG_E1DETECTOR || element == SUMO_TAG_INDUCTION_LOOP) {
    //         try {
    //             bool ok = true;
    //             // get the id, report an error if not given or empty...
    //             std::string id = attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
    //             if (!ok) {
    //                 throw ProcessError();
    //             }
    //             std::string lane = attrs.get<std::string>(SUMO_ATTR_LANE, id.c_str(), ok);
    //             if (!ok) {
    //                 throw ProcessError();
    //             }
    //             ROEdge* edge = myNet->getEdge(SUMOXMLDefinitions::getEdgeIDFromLane(lane));
    //             int laneIndex = SUMOXMLDefinitions::getIndexFromLane(lane);
    //             if (edge == nullptr || laneIndex >= edge->getNumLanes()) {
    //                 throw ProcessError("Unknown lane '" + lane + "' for detector '" + id + "' in '" + getFileName() + "'.");
    //             }
    //             double pos = attrs.get<double>(SUMO_ATTR_POSITION, id.c_str(), ok);
    //             std::string mml_type = attrs.getOpt<std::string>(SUMO_ATTR_TYPE, id.c_str(), ok, "");
    //             if (!ok) {
    //                 throw ProcessError();
    //             }
    //             RODFDetectorType type = TYPE_NOT_DEFINED;
    //             if (mml_type == "between") {
    //                 type = BETWEEN_DETECTOR;
    //             } else if (mml_type == "source" || mml_type == "highway_source") { // !!! highway-source is legacy (removed accoring output on 06.08.2007)
    //                 type = SOURCE_DETECTOR;
    //             } else if (mml_type == "sink") {
    //                 type = SINK_DETECTOR;
    //             }
    //             RODFDetector* detector = new RODFDetector(id, lane, pos, type);
    //             if (!myContainer.addDetector(detector)) {
    //                 delete detector;
    //                 throw ProcessError(TLF("Could not add detector '%' (probably the id is already used).", id));
    //             }
    //         } catch (ProcessError& e) {
    //             if (myIgnoreErrors) {
    //                 WRITE_WARNING(e.what());
    //             } else {
    //                 throw;
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) {
        if element == SUMO_TAG_DETECTOR_DEFINITION || element == SUMO_TAG_E1DETECTOR || element == SUMO_TAG_INDUCTION_LOOP {
            if let Err(e) = self.add_detector_from_attrs(attrs) {
                if self.myIgnoreErrors {
                    write_warning(&e.to_string());
                } else {
                    panic!("{}", e);
                }
            }
        }
    }

    /// body of the try block in myStartElement; errors are returned instead of thrown
    fn add_detector_from_attrs(&mut self, attrs: &SUMOSAXAttributes) -> Result<(), ProcessError> {
        let mut ok = true;
        // get the id, report an error if not given or empty...
        let id: String = attrs.get::<String>(SUMO_ATTR_ID, "", &mut ok, true);
        if !ok {
            return Err(ProcessError::default_msg());
        }
        let lane: String = attrs.get::<String>(SUMO_ATTR_LANE, &id, &mut ok, true);
        if !ok {
            return Err(ProcessError::default_msg());
        }
        let edge = self.myNet.get_edge(&get_edge_id_from_lane(&lane));
        let laneIndex = get_index_from_lane(&lane)?;
        if edge.is_null() || laneIndex >= edge.get_num_lanes() {
            return Err(ProcessError::new(format!("Unknown lane '{}' for detector '{}' in '{}'.", lane, id, self.get_file_name())));
        }
        let pos: f64 = attrs.get::<f64>(SUMO_ATTR_POSITION, &id, &mut ok, true);
        let mml_type: String = attrs.get_opt::<String>(SUMO_ATTR_TYPE, &id, &mut ok, String::new(), true);
        if !ok {
            return Err(ProcessError::default_msg());
        }
        let mut type_ = RODFDetectorType::TYPE_NOT_DEFINED;
        if mml_type == "between" {
            type_ = RODFDetectorType::BETWEEN_DETECTOR;
        } else if mml_type == "source" || mml_type == "highway_source" {
            // !!! highway-source is legacy (removed accoring output on 06.08.2007)
            type_ = RODFDetectorType::SOURCE_DETECTOR;
        } else if mml_type == "sink" {
            type_ = RODFDetectorType::SINK_DETECTOR;
        }
        let detector = Box::new(RODFDetector::new(&id, &lane, pos, type_));
        if !self.myContainer.m().add_detector(Ptr::from_box(&detector)) {
            // detector is dropped here (C++: delete detector)
            return Err(ProcessError::new(tlf!("Could not add detector '%' (probably the id is already used).", id)));
        }
        // shortcut: C++ container owns and deletes detectors, but RODFDetectorCon stores Ptr; leaked until the container owns Box<RODFDetector>, upgrade with RODFDetectorCon
        Box::leak(detector);
        Ok(())
    }

    /// C++: `void RODFDetectorHandler::RODFDetectorHandler(const RODFDetectorHandler &)` [private]
    /// invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(src: &RODFDetectorHandler) -> Self {
        panic!("RODFDetectorHandler copy constructor is private and never defined in C++")
    }

    /// C++: `RODFDetectorHandler & RODFDetectorHandler::operator=(const RODFDetectorHandler &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, src: &RODFDetectorHandler) -> &mut RODFDetectorHandler {
        panic!("RODFDetectorHandler::operator= is private and never defined in C++")
    }

}

