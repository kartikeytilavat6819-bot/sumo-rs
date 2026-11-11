//! Port of netbuild/NBSign.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ enum `NBSign::SignType`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum NBSign_SignType {
    SIGN_TYPE_SPEED = 0,
    SIGN_TYPE_YIELD = 1,
    SIGN_TYPE_STOP = 2,
    SIGN_TYPE_ALLWAY_STOP = 3,
    SIGN_TYPE_ON_RAMP = 4,
    SIGN_TYPE_PRIORITY = 5,
    SIGN_TYPE_RIGHT_BEFORE_LEFT = 6,
    SIGN_TYPE_LEFT_BEFORE_RIGHT = 7,
    SIGN_TYPE_ROUNDABOUT = 8,
    SIGN_TYPE_RAIL_CROSSING = 9,
    SIGN_TYPE_SLOPE = 10,
    SIGN_TYPE_CITY = 11,
    SIGN_TYPE_INFO = 12,
}

/// A class representing a single street sign
/// C++ class `NBSign` (netbuild/NBSign.h)
pub struct NBSign {
    /// the type of the sign
    pub myType: NBSign_SignType,
    /// The offset of the sign from the start of its edge
    pub myOffset: f64,
    /// The (optional) label (for SPEED, SLOPE etc)
    pub myLabel: String,
}

// C++ static members of NBSign (port as module-level statics, see AGENTS.md):
//   static StringBijection<SignType> NBSign::SignTypeStrings;
//   static StringBijection<SignType> NBSign::SignTypeColors;

impl NBSign {
    /// C++: `void NBSign::NBSign(SignType, double, const std::string)`
    /// Constructor with id, and position
    // NBSign::NBSign(SignType type, double offset, const std::string label) :
    //     myType(type),
    //     myOffset(offset),
    //     myLabel(label) {
    // }
    #[allow(unused_variables)]
    pub fn new(type_: NBSign_SignType, offset: f64, label: String) -> Self {
        todo!("port NBSign::NBSign")
    }

    /// C++: `void NBSign::~NBSign()`
    /// Destructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port NBSign::~NBSign")
    }

    /// C++: `void NBSign::writeAsPOI(OutputDevice &, const NBEdge *) const` [const]
    /// write into device as POI positioned relative to the given edge
    // void
    // NBSign::writeAsPOI(OutputDevice& into, const NBEdge* edge) const {
    //     PositionVector shp = edge->getLanes()[0].shape;
    //     try {
    //         shp.move2side(3);
    //     } catch (InvalidArgument&) {
    //         // we do not write anything, maybe we should
    //     }
    //     Position pos = shp.positionAtOffset(myOffset);
    //     into.openTag(SUMO_TAG_POI);
    //     into.writeAttr(SUMO_ATTR_ID, edge->getID() + "." + toString(myOffset));
    //     into.writeAttr(SUMO_ATTR_TYPE, SignTypeStrings.getString(myType));
    //     switch (myType) { /// XXX @todo add default colors
    //         case SIGN_TYPE_SPEED:
    //         case SIGN_TYPE_SLOPE:
    //         case SIGN_TYPE_CITY:
    //         case SIGN_TYPE_INFO:
    //             into.writeAttr(SUMO_ATTR_COLOR, RGBColor::GREY);
    //             break;
    //         case SIGN_TYPE_YIELD:
    //         case SIGN_TYPE_STOP:
    //         case SIGN_TYPE_ALLWAY_STOP:
    //         case SIGN_TYPE_ON_RAMP:
    //         case SIGN_TYPE_RAIL_CROSSING:
    //             into.writeAttr(SUMO_ATTR_COLOR, RGBColor::RED);
    //             break;
    //         case SIGN_TYPE_PRIORITY:
    //             into.writeAttr(SUMO_ATTR_COLOR, RGBColor::YELLOW);
    //             break;
    //         case SIGN_TYPE_RIGHT_BEFORE_LEFT:
    //         case SIGN_TYPE_LEFT_BEFORE_RIGHT:
    //             into.writeAttr(SUMO_ATTR_COLOR, RGBColor(255, 153, 0, 255));
    //             break;
    //         case SIGN_TYPE_ROUNDABOUT:
    //             into.writeAttr(SUMO_ATTR_COLOR, RGBColor::BLUE);
    //             break;
    //     }
    //     into.writeAttr(SUMO_ATTR_X, pos.x());
    //     into.writeAttr(SUMO_ATTR_Y, pos.y());
    //     into.writeAttr(SUMO_ATTR_ANGLE, GeomHelper::naviDegree(shp.rotationAtOffset(myOffset)));
    //     // @todo add image resources and default images for all signs
    //     //into.writeAttr(SUMO_ATTR_IMGFILE, p->getImgFile());
    //     //into.writeAttr(SUMO_ATTR_WIDTH, p->getWidth());
    //     //into.writeAttr(SUMO_ATTR_HEIGHT, p->getHeight());
    //     into.closeTag();
    // }
    #[allow(unused_variables)]
    pub fn write_as_poi(&self, into: &mut OutputDevice, edge: Ptr<NBEdge>) {
        todo!("port NBSign::writeAsPOI")
    }

}

