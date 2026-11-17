//! Port of mesosim/METypeHandler.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// The XML-Handler for meso edge type loading
/// C++ class `METypeHandler` (mesosim/METypeHandler.h)
#[repr(C)]
pub struct METypeHandler {
    /// C++ base class `SUMOSAXHandler`
    pub base: GenericSAXHandler,
    /// The net to fill (preinitialised)
    pub myNet: Ptr<MSNet>,
    /// The id of the currently processed edge type
    pub myCurrentTypeID: String,
    /// whether edge type specific meso parameters were loaded
    pub myHaveSeenMesoEdgeType: bool,
}

impl std::ops::Deref for METypeHandler {
    type Target = GenericSAXHandler;
    fn deref(&self) -> &GenericSAXHandler {
        &self.base
    }
}
impl std::ops::DerefMut for METypeHandler {
    fn deref_mut(&mut self) -> &mut GenericSAXHandler {
        &mut self.base
    }
}

impl METypeHandler {
    /// C++: `void METypeHandler::METypeHandler(const std::string &, MSNet &)`
    /// Constructor
    // METypeHandler::METypeHandler(const std::string& file, MSNet& net) :
    //     SUMOSAXHandler(file),
    //     myNet(net),
    //     myHaveSeenMesoEdgeType(false)
    // {
    // }
    #[allow(unused_variables)]
    pub fn new(file: &str, net: &mut MSNet) -> Self {
        METypeHandler {
            base: utils::xml::sumo_sax_handler::new(file, ""),
            myNet: Ptr::from_mut(net),
            myCurrentTypeID: String::new(),
            myHaveSeenMesoEdgeType: false,
        }
    }

    /// C++: `void METypeHandler::~METypeHandler() noexcept` [virtual]
    /// Destructor
    // METypeHandler::~METypeHandler() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty; the Rust fields are dropped automatically
    }

    /// C++: `bool METypeHandler::haveSeenMesoEdgeType() const` [const]
    // bool haveSeenMesoEdgeType() const {
    //         return myHaveSeenMesoEdgeType;
    //     }
    #[allow(unused_variables)]
    pub fn have_seen_meso_edge_type(&self) -> bool {
        self.myHaveSeenMesoEdgeType
    }

    /// C++: `void METypeHandler::myStartElement(int, const SUMOSAXAttributes &)` [virtual, protected]
    /// Called on the opening of a tag;
    // void
    // METypeHandler::myStartElement(int element,
    //                           const SUMOSAXAttributes& attrs) {
    //     try {
    //         switch (element) {
    //             case SUMO_TAG_TYPE: {
    //                 bool ok = true;
    //                 myCurrentTypeID = attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
    //                 break;
    //             }
    //             case SUMO_TAG_MESO: {
    //                 addMesoEdgeType(attrs);
    //                 break;
    //             }
    //             default:
    //                 break;
    //         }
    //     } catch (InvalidArgument& e) {
    //         WRITE_ERROR(e.what());
    //     }
    // }
    #[allow(unused_variables)]
    pub fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) {
        // C++ `throw InvalidArgument` is modelled as panic_any(InvalidArgument); catch + WRITE_ERROR
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            match element {
                SUMO_TAG_TYPE => {
                    let mut ok = true;
                    self.myCurrentTypeID = attrs.get::<String>(SUMO_ATTR_ID, "", &mut ok, true);
                }
                SUMO_TAG_MESO => {
                    self.add_meso_edge_type(attrs);
                }
                _ => {}
            }
        }));
        if let Err(payload) = result {
            match payload.downcast::<utils::geom::InvalidArgument>() {
                Ok(e) => utils::common::msg_handler::write_error(&e.0),
                Err(other) => std::panic::resume_unwind(other),
            }
        }
    }

    /// C++: `void METypeHandler::myEndElement(int)` [virtual, protected]
    /// Called when a closing tag occurs
    // void
    // METypeHandler::myEndElement(int element) {
    //     switch (element) {
    //         case SUMO_TAG_TYPE:
    //             myCurrentTypeID = "";
    //             break;
    //         default:
    //             break;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn my_end_element(&mut self, element: i32) {
        match element {
            SUMO_TAG_TYPE => {
                self.myCurrentTypeID = String::new();
            }
            _ => {}
        }
    }

    /// C++: `void METypeHandler::addMesoEdgeType(const SUMOSAXAttributes &)` [protected]
    /// Loads edge type specific meso parameters
    // void
    // METypeHandler::addMesoEdgeType(const SUMOSAXAttributes& attrs) {
    //     bool ok = true;
    //     MESegment::MesoEdgeType edgeType = myNet.getMesoType(""); // init defaults
    //     edgeType.tauff = attrs.getOptSUMOTimeReporting(SUMO_ATTR_MESO_TAUFF, myCurrentTypeID.c_str(), ok, edgeType.tauff);
    //     edgeType.taufj = attrs.getOptSUMOTimeReporting(SUMO_ATTR_MESO_TAUFJ, myCurrentTypeID.c_str(), ok, edgeType.taufj);
    //     edgeType.taujf = attrs.getOptSUMOTimeReporting(SUMO_ATTR_MESO_TAUJF, myCurrentTypeID.c_str(), ok, edgeType.taujf);
    //     edgeType.taujj = attrs.getOptSUMOTimeReporting(SUMO_ATTR_MESO_TAUJJ, myCurrentTypeID.c_str(), ok, edgeType.taujj);
    //     edgeType.jamThreshold = attrs.getOpt<double>(SUMO_ATTR_JAM_DIST_THRESHOLD, myCurrentTypeID.c_str(), ok, edgeType.jamThreshold);
    //     edgeType.junctionControl = attrs.getOpt<bool>(SUMO_ATTR_MESO_JUNCTION_CONTROL, myCurrentTypeID.c_str(), ok, edgeType.junctionControl);
    //     edgeType.tlsPenalty = attrs.getOpt<double>(SUMO_ATTR_MESO_TLS_PENALTY, myCurrentTypeID.c_str(), ok, edgeType.tlsPenalty);
    //     edgeType.tlsFlowPenalty = attrs.getOpt<double>(SUMO_ATTR_MESO_TLS_FLOW_PENALTY, myCurrentTypeID.c_str(), ok, edgeType.tlsFlowPenalty);
    //     edgeType.minorPenalty = attrs.getOptSUMOTimeReporting(SUMO_ATTR_MESO_MINOR_PENALTY, myCurrentTypeID.c_str(), ok, edgeType.minorPenalty);
    //     edgeType.overtaking = attrs.getOpt<bool>(SUMO_ATTR_MESO_OVERTAKING, myCurrentTypeID.c_str(), ok, edgeType.overtaking);
    //     edgeType.edgeLength = attrs.getOpt<double>(SUMO_ATTR_MESO_EDGELENGHT, myCurrentTypeID.c_str(), ok, edgeType.edgeLength);
    //     edgeType.lookahead = attrs.getOpt<double>(SUMO_ATTR_MESO_LOOKAHEAD, myCurrentTypeID.c_str(), ok, edgeType.lookahead);
    //
    //     if (ok) {
    //         myNet.addMesoType(myCurrentTypeID, edgeType);
    //     }
    //     myHaveSeenMesoEdgeType = true;
    // }
    #[allow(unused_variables)]
    pub fn add_meso_edge_type(&mut self, attrs: &SUMOSAXAttributes) {
        let mut ok = true;
        let mut edgeType = *self.myNet.m().get_meso_type(""); // init defaults
        let id = self.myCurrentTypeID.clone();
        edgeType.tauff = attrs.get_opt_sumo_time_reporting(SUMO_ATTR_MESO_TAUFF, &id, &mut ok, edgeType.tauff, true);
        edgeType.taufj = attrs.get_opt_sumo_time_reporting(SUMO_ATTR_MESO_TAUFJ, &id, &mut ok, edgeType.taufj, true);
        edgeType.taujf = attrs.get_opt_sumo_time_reporting(SUMO_ATTR_MESO_TAUJF, &id, &mut ok, edgeType.taujf, true);
        edgeType.taujj = attrs.get_opt_sumo_time_reporting(SUMO_ATTR_MESO_TAUJJ, &id, &mut ok, edgeType.taujj, true);
        edgeType.jamThreshold = attrs.get_opt::<f64>(SUMO_ATTR_JAM_DIST_THRESHOLD, &id, &mut ok, edgeType.jamThreshold, true);
        edgeType.junctionControl = attrs.get_opt::<bool>(SUMO_ATTR_MESO_JUNCTION_CONTROL, &id, &mut ok, edgeType.junctionControl, true);
        edgeType.tlsPenalty = attrs.get_opt::<f64>(SUMO_ATTR_MESO_TLS_PENALTY, &id, &mut ok, edgeType.tlsPenalty, true);
        edgeType.tlsFlowPenalty = attrs.get_opt::<f64>(SUMO_ATTR_MESO_TLS_FLOW_PENALTY, &id, &mut ok, edgeType.tlsFlowPenalty, true);
        edgeType.minorPenalty = attrs.get_opt_sumo_time_reporting(SUMO_ATTR_MESO_MINOR_PENALTY, &id, &mut ok, edgeType.minorPenalty, true);
        edgeType.overtaking = attrs.get_opt::<bool>(SUMO_ATTR_MESO_OVERTAKING, &id, &mut ok, edgeType.overtaking, true);
        edgeType.edgeLength = attrs.get_opt::<f64>(SUMO_ATTR_MESO_EDGELENGHT, &id, &mut ok, edgeType.edgeLength, true);
        edgeType.lookahead = attrs.get_opt::<f64>(SUMO_ATTR_MESO_LOOKAHEAD, &id, &mut ok, edgeType.lookahead, true);

        if ok {
            self.myNet.m().add_meso_type(&self.myCurrentTypeID, &edgeType);
        }
        self.myHaveSeenMesoEdgeType = true;
    }

    /// C++: `void METypeHandler::METypeHandler(const METypeHandler &)` [private]
    /// invalid copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(s: &METypeHandler) -> Self {
        panic!("METypeHandler(const METypeHandler&) is private and undefined in C++; it cannot be called")
    }

    /// C++: `METypeHandler & METypeHandler::operator=(const METypeHandler &)` [private]
    /// invalid assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &METypeHandler) -> &mut METypeHandler {
        panic!("METypeHandler assignment operator is private and undefined in C++; it cannot be called")
    }

}


impl utils::xml::generic_sax_handler::SaxHandler for METypeHandler {
    fn handler(&mut self) -> &mut utils::xml::generic_sax_handler::GenericSAXHandler { &mut self.base }
}
