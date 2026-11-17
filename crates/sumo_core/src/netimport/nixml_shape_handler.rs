//! Port of netimport/NIXMLShapeHandler.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::msg_handler::{write_error, write_warning};

/// The XML-Handler for shapes loading network loading
/// C++ class `NIXMLShapeHandler` (netimport/NIXMLShapeHandler.h)
#[repr(C)]
pub struct NIXMLShapeHandler {
    /// C++ base class `ShapeHandler`
    pub base: ShapeHandler,
    pub myEdgeCont: Ptr<NBEdgeCont>,
}

impl std::ops::Deref for NIXMLShapeHandler {
    type Target = ShapeHandler;
    fn deref(&self) -> &ShapeHandler {
        &self.base
    }
}
impl std::ops::DerefMut for NIXMLShapeHandler {
    fn deref_mut(&mut self) -> &mut ShapeHandler {
        &mut self.base
    }
}

impl NIXMLShapeHandler {
    /// C++: `void NIXMLShapeHandler::NIXMLShapeHandler(ShapeContainer &, const NBEdgeCont &)`
    // NIXMLShapeHandler::NIXMLShapeHandler(ShapeContainer& sc, const NBEdgeCont& ec) :
    //     ShapeHandler("polgyon - file", sc, GeoConvHelper::getNumLoaded() == 0 ? nullptr : & GeoConvHelper::getLoaded()),
    //     myEdgeCont(ec)
    // {}
    #[allow(unused_variables)]
    pub fn new(sc: &mut ShapeContainer, ec: &NBEdgeCont) -> Self {
        Self {
            base: ShapeHandler::new("polgyon - file", sc, if GeoConvHelper::get_num_loaded() == 0 { Ptr::null() } else { GeoConvHelper::with_loaded(|g| Ptr::from_mut(g)) }),
            myEdgeCont: Ptr::new(ec),
        }
    }

    /// C++: `void NIXMLShapeHandler::~NIXMLShapeHandler() noexcept` [virtual]
    /// Destructor
    // virtual ~NIXMLShapeHandler() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // the C++ destructor has no body
    }

    /// C++: `Position NIXMLShapeHandler::getLanePos(const std::string &, const std::string &, double, bool, double)` [virtual]
    // Position
    // NIXMLShapeHandler::getLanePos(const std::string& poiID, const std::string& laneID, double lanePos, bool friendlyPos, double lanePosLat) {
    //     std::string edgeID;
    //     int laneIndex;
    //     NBHelpers::interpretLaneID(laneID, edgeID, laneIndex);
    //     NBEdge* edge = myEdgeCont.retrieve(edgeID);
    //     if (edge == 0 || laneIndex < 0 || edge->getNumLanes() <= laneIndex) {
    //         WRITE_ERRORF(TL("Lane '%' to place poi '%' on is not known."), laneID, poiID);
    //         return Position::INVALID;
    //     }
    //     if (lanePos < 0) {
    //         lanePos = edge->getLength() + lanePos;
    //     }
    //     if ((lanePos < 0) && friendlyPos) {
    //         lanePos = 0;
    //     }
    //     if ((lanePos > edge->getLength()) && friendlyPos) {
    //         lanePos = edge->getLength();
    //     }
    //     if (lanePos < 0 || lanePos > edge->getLength()) {
    //         WRITE_WARNINGF(TL("lane position % for poi '%' is not valid."), toString(lanePos), poiID);
    //     }
    //     return edge->getLanes()[laneIndex].shape.positionAtOffset(lanePos, -lanePosLat);
    // }
    #[allow(unused_variables)]
    pub fn get_lane_pos(&mut self, poiID: &str, laneID: &str, lanePos: f64, friendlyPos: bool, lanePosLat: f64) -> Position {
        let mut edgeID = String::new();
        let mut laneIndex: i32 = 0;
        NBHelpers::interpret_lane_id(laneID, &mut edgeID, &mut laneIndex);
        let edge = self.myEdgeCont.m().retrieve(&edgeID, false);
        if edge.is_null() || laneIndex < 0 || edge.m().get_num_lanes() <= laneIndex {
            write_error(&format!("Lane '{laneID}' to place poi '{poiID}' on is not known."));
            return Position::INVALID;
        }
        let mut lanePos = lanePos;
        if lanePos < 0.0 {
            lanePos = edge.m().get_length() + lanePos;
        }
        if (lanePos < 0.0) && friendlyPos {
            lanePos = 0.0;
        }
        if (lanePos > edge.m().get_length()) && friendlyPos {
            lanePos = edge.m().get_length();
        }
        if lanePos < 0.0 || lanePos > edge.m().get_length() {
            write_warning(&format!("lane position {} for poi '{poiID}' is not valid.", utils::common::to_string::to_string(&lanePos)));
        }
        edge.m().get_lanes()[laneIndex as usize].shape.position_at_offset(lanePos, -lanePosLat)
    }

    /// C++: `bool NIXMLShapeHandler::addLanePosParams()` [virtual]
    // virtual bool addLanePosParams() {
    //         return true;
    //     }
    #[allow(unused_variables)]
    pub fn add_lane_pos_params(&mut self) -> bool {
        true
    }

}

impl SaxHandler for NIXMLShapeHandler {
    fn handler(&mut self) -> &mut GenericSAXHandler {
        &mut self.base.base
    }
    fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) -> Result<(), ProcessError> {
        // shortcut: no virtual dispatch, so ShapeHandler::add_poi calls the base get_lane_pos (panics); POIs with lane= in polygon files fail until the vtable work lands
        ShapeHandler::my_start_element(&mut self.base, element, attrs);
        Ok(())
    }
    fn my_end_element(&mut self, element: i32) -> Result<(), ProcessError> {
        ShapeHandler::my_end_element(&mut self.base, element);
        Ok(())
    }
}

