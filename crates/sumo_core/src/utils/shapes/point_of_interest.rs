//! Port of utils/shapes/PointOfInterest.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// A point-of-interest
/// C++ class `PointOfInterest` (utils/shapes/PointOfInterest.h)
pub struct PointOfInterest {
    /// C++ base class `Shape`
    pub base: Shape,
    /// C++ base class `Position`
    pub base2: Position,
    /// C++ base class `Parameterised`
    pub base3: Parameterised,
    /// flag to check if POI was loaded as GEO Position (main used by netedit)
    pub myGeo: bool,
    /// ID of lane in which this POI is placed (main used by netedit)
    pub myLane: String,
    /// position over lane in which this POI is placed (main used by netedit)
    pub myPosOverLane: f64,
    /// friendlyPos enable or disable friendly position for position over lane
    pub myFriendlyPos: bool,
    /// lateral position over lane in which this POI is placed (main used by netedit)
    pub myPosLat: f64,
    /// POI icon
    pub myIcon: POIIcon,
    /// The half width of the image when rendering this POI
    pub myHalfImgWidth: f64,
    /// The half height of the image when rendering this POI
    pub myHalfImgHeight: f64,
}

impl std::ops::Deref for PointOfInterest {
    type Target = Shape;
    fn deref(&self) -> &Shape {
        &self.base
    }
}
impl std::ops::DerefMut for PointOfInterest {
    fn deref_mut(&mut self) -> &mut Shape {
        &mut self.base
    }
}

impl PointOfInterest {
    /// C++: `void PointOfInterest::PointOfInterest(const std::string &, const std::string &, const RGBColor &, const Position &, bool, const std::string &, double, bool, double, const std::string &, double, double, const std::string &, double, double, const std::string &, const Parameterised::Map &)`
    /// Constructor
    // PointOfInterest::PointOfInterest(const std::string& id, const std::string& type, const RGBColor& color, const Position& pos,
    //                                  bool geo, const std::string& lane, double posOverLane, bool friendlyPos, double posLat,
    //                                  const std::string& icon, double layer, double angle, const std::string& imgFile, double width,
    //                                  double height, const std::string& name, const Parameterised::Map& parameters) :
    //     Shape(id, type, color, layer, angle, imgFile, name),
    //     Position(pos),
    //     Parameterised(parameters),
    //     myGeo(geo),
    //     myLane(lane),
    //     myPosOverLane(posOverLane),
    //     myFriendlyPos(friendlyPos),
    //     myPosLat(posLat),
    //     myIcon(SUMOXMLDefinitions::POIIcons.get(icon)),
    //     myHalfImgWidth(width / 2.0),
    //     myHalfImgHeight(height / 2.0) {
    // }
    #[allow(unused_variables)]
    pub fn new(id: &str, type_: &str, color: &RGBColor, pos: &Position, geo: bool, lane: &str, posOverLane: f64, friendlyPos: bool, posLat: f64, icon: &str, layer: f64, angle: f64, imgFile: &str, width: f64, height: f64, name: &str, parameters: &Unmapped /* Parameterised::Map */) -> Self {
        todo!("port PointOfInterest::PointOfInterest")
    }

    /// C++: `void PointOfInterest::~PointOfInterest() noexcept` [virtual]
    /// Destructor
    // PointOfInterest::~PointOfInterest() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port PointOfInterest::~PointOfInterest")
    }

    /// C++: `POIIcon PointOfInterest::getIcon() const` [const]
    /// get icon
    // POIIcon
    // PointOfInterest::getIcon() const {
    //     return myIcon;
    // }
    #[allow(unused_variables)]
    pub fn get_icon(&self) -> POIIcon {
        todo!("port PointOfInterest::getIcon")
    }

    /// C++: `const std::string & PointOfInterest::getIconStr() const` [const]
    /// get icon(in string format)
    // const std::string&
    // PointOfInterest::getIconStr() const {
    //     return SUMOXMLDefinitions::POIIcons.getString(myIcon);
    // }
    #[allow(unused_variables)]
    pub fn get_icon_str(&self) -> &str {
        todo!("port PointOfInterest::getIconStr")
    }

    /// C++: `double PointOfInterest::getWidth() const` [const]
    /// Returns the image width of the POI
    // double
    // PointOfInterest::getWidth() const {
    //     return myHalfImgWidth * 2.0;
    // }
    #[allow(unused_variables)]
    pub fn get_width(&self) -> f64 {
        todo!("port PointOfInterest::getWidth")
    }

    /// C++: `double PointOfInterest::getHeight() const` [const]
    /// Returns the image height of the POI
    // double
    // PointOfInterest::getHeight() const {
    //     return myHalfImgHeight * 2.0;
    // }
    #[allow(unused_variables)]
    pub fn get_height(&self) -> f64 {
        todo!("port PointOfInterest::getHeight")
    }

    /// C++: `Position PointOfInterest::getCenter() const` [const]
    /// Returns the image center of the POI
    // Position
    // PointOfInterest::getCenter() const {
    //     return { x() + myHalfImgWidth, y() + myHalfImgHeight };
    // }
    #[allow(unused_variables)]
    pub fn get_center(&self) -> Position {
        todo!("port PointOfInterest::getCenter")
    }

    /// C++: `bool PointOfInterest::getFriendlyPos() const` [const]
    /// returns friendly position
    // bool
    // PointOfInterest::getFriendlyPos() const {
    //     return myFriendlyPos;
    // }
    #[allow(unused_variables)]
    pub fn get_friendly_pos(&self) -> bool {
        todo!("port PointOfInterest::getFriendlyPos")
    }

    /// C++: `void PointOfInterest::setIcon(const std::string &)`
    /// set icon
    // void
    // PointOfInterest::setIcon(const std::string& icon) {
    //     myIcon = SUMOXMLDefinitions::POIIcons.get(icon);
    // }
    #[allow(unused_variables)]
    pub fn set_icon(&mut self, icon: &str) {
        todo!("port PointOfInterest::setIcon")
    }

    /// C++: `void PointOfInterest::setWidth(double)`
    /// set the image width of the POI
    // void
    // PointOfInterest::setWidth(double width) {
    //     myHalfImgWidth = width / 2.0;
    // }
    #[allow(unused_variables)]
    pub fn set_width(&mut self, width: f64) {
        todo!("port PointOfInterest::setWidth")
    }

    /// C++: `void PointOfInterest::setHeight(double)`
    /// set the image height of the POI
    // void
    // PointOfInterest::setHeight(double height) {
    //     myHalfImgHeight = height / 2.0;
    // }
    #[allow(unused_variables)]
    pub fn set_height(&mut self, height: f64) {
        todo!("port PointOfInterest::setHeight")
    }

    /// C++: `void PointOfInterest::setFriendlyPos(const bool)`
    /// set friendly position
    // void
    // PointOfInterest::setFriendlyPos(const bool friendlyPos) {
    //     myFriendlyPos = friendlyPos;
    // }
    #[allow(unused_variables)]
    pub fn set_friendly_pos(&mut self, friendlyPos: bool) {
        todo!("port PointOfInterest::setFriendlyPos")
    }

    /// C++: `void PointOfInterest::writeXML(OutputDevice &, const bool, const double, const std::string, const double, const bool, const double) const` [const]
    // void
    // PointOfInterest::writeXML(OutputDevice& out, const bool geo, const double zOffset, const std::string laneID,
    //                           const double pos, const bool friendlyPos, const double posLat) const {
    //     out.openTag(SUMO_TAG_POI);
    //     out.writeAttr(SUMO_ATTR_ID, StringUtils::escapeXML(getID()));
    //     if (getShapeType().size() > 0) {
    //         out.writeAttr(SUMO_ATTR_TYPE, StringUtils::escapeXML(getShapeType()));
    //     }
    //     if (myIcon != POIIcon::NONE) {
    //         out.writeAttr(SUMO_ATTR_ICON, SUMOXMLDefinitions::POIIcons.getString(myIcon));
    //     }
    //     out.writeAttr(SUMO_ATTR_COLOR, getShapeColor());
    //     out.writeAttr(SUMO_ATTR_LAYER, getShapeLayer() + zOffset);
    //     if (!getShapeName().empty()) {
    //         out.writeAttr(SUMO_ATTR_NAME, getShapeName());
    //     }
    //     if (laneID != "") {
    //         out.writeAttr(SUMO_ATTR_LANE, laneID);
    //         out.writeAttr(SUMO_ATTR_POSITION, pos);
    //         if (posLat != 0) {
    //             out.writeAttr(SUMO_ATTR_POSITION_LAT, posLat);
    //         }
    //         if (friendlyPos) {
    //             out.writeAttr(SUMO_ATTR_FRIENDLY_POS, friendlyPos);
    //         }
    //     } else {
    //         if (geo) {
    //             Position POICartesianPos(*this);
    //             GeoConvHelper::getFinal().cartesian2geo(POICartesianPos);
    //             out.setPrecision(gPrecisionGeo);
    //             out.writeAttr(SUMO_ATTR_LON, POICartesianPos.x());
    //             out.writeAttr(SUMO_ATTR_LAT, POICartesianPos.y());
    //             out.setPrecision();
    //         } else {
    //             out.writeAttr(SUMO_ATTR_X, x());
    //             out.writeAttr(SUMO_ATTR_Y, y());
    //         }
    //         if (z() != 0.) {
    //             out.writeAttr(SUMO_ATTR_Z, z());
    //         }
    //     }
    //     if (getShapeNaviDegree() != Shape::DEFAULT_ANGLE) {
    //         out.writeAttr(SUMO_ATTR_ANGLE, getShapeNaviDegree());
    //     }
    //     if (getShapeImgFile() != Shape::DEFAULT_IMG_FILE) {
    //         out.writeAttr(SUMO_ATTR_IMGFILE, getShapeImgFile());
    //     }
    //     if (getWidth() != Shape::DEFAULT_IMG_WIDTH) {
    //         out.writeAttr(SUMO_ATTR_WIDTH, getWidth());
    //     }
    //     if (getHeight() != Shape::DEFAULT_IMG_HEIGHT) {
    //         out.writeAttr(SUMO_ATTR_HEIGHT, getHeight());
    //     }
    //     writeParams(out);
    //     out.closeTag();
    // }
    #[allow(unused_variables)]
    pub fn write_xml(&self, out: &mut OutputDevice, geo: bool, zOffset: f64, laneID: String, pos: f64, friendlyPos: bool, posLat: f64) {
        todo!("port PointOfInterest::writeXML")
    }

}

