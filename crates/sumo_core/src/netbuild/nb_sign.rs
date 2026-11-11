//! Port of netbuild/NBSign.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::to_string::to_string;
use utils::geom::geom_helper::navi_degree;

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

/// C++ static `NBSign::SignTypeStrings` (initialised from signTypeStringsInitializer in NBSign.cpp)
pub static NBSign_SignTypeStrings: std::sync::LazyLock<StringBijection<NBSign_SignType>> =
    std::sync::LazyLock::new(|| {
        StringBijection::new(
            &[
                ("speed limit", NBSign_SignType::SIGN_TYPE_SPEED),
                ("yield", NBSign_SignType::SIGN_TYPE_YIELD),
                ("stop", NBSign_SignType::SIGN_TYPE_STOP),
                ("allway_stop", NBSign_SignType::SIGN_TYPE_ALLWAY_STOP),
                ("on ramp", NBSign_SignType::SIGN_TYPE_ON_RAMP),
                ("priority", NBSign_SignType::SIGN_TYPE_PRIORITY),
                ("right before left", NBSign_SignType::SIGN_TYPE_RIGHT_BEFORE_LEFT),
                ("left before right", NBSign_SignType::SIGN_TYPE_LEFT_BEFORE_RIGHT),
                ("roundabout", NBSign_SignType::SIGN_TYPE_ROUNDABOUT),
                ("rail crossing", NBSign_SignType::SIGN_TYPE_RAIL_CROSSING),
                ("slope", NBSign_SignType::SIGN_TYPE_SLOPE),
                ("city limits", NBSign_SignType::SIGN_TYPE_CITY),
                ("info", NBSign_SignType::SIGN_TYPE_INFO),
            ],
            true,
        )
        .expect("NBSign sign type strings are unique")
    });

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
        NBSign { myType: type_, myOffset: offset, myLabel: label }
    }

    /// C++: `void NBSign::~NBSign()`
    /// Destructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
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
        let mut shp: PositionVector = edge.get_lanes()[0].shape.clone();
        // C++ move2side throws InvalidArgument (caught and ignored); the ported move2side cannot fail
        shp.move2side(3.0, 100.0);
        let pos: Position = shp.position_at_offset(self.myOffset, 0.0);
        into.open_tag_id(SUMO_TAG_POI);
        into.write_attr(SUMO_ATTR_ID, &format!("{}.{}", edge.get_id(), to_string(&self.myOffset)));
        into.write_attr(SUMO_ATTR_TYPE, NBSign_SignTypeStrings.get_string(self.myType).unwrap());
        match self.myType {
            // XXX @todo add default colors
            NBSign_SignType::SIGN_TYPE_SPEED
            | NBSign_SignType::SIGN_TYPE_SLOPE
            | NBSign_SignType::SIGN_TYPE_CITY
            | NBSign_SignType::SIGN_TYPE_INFO => {
                into.write_attr(SUMO_ATTR_COLOR, &to_string(&RGBColor::GREY));
            }
            NBSign_SignType::SIGN_TYPE_YIELD
            | NBSign_SignType::SIGN_TYPE_STOP
            | NBSign_SignType::SIGN_TYPE_ALLWAY_STOP
            | NBSign_SignType::SIGN_TYPE_ON_RAMP
            | NBSign_SignType::SIGN_TYPE_RAIL_CROSSING => {
                into.write_attr(SUMO_ATTR_COLOR, &to_string(&RGBColor::RED));
            }
            NBSign_SignType::SIGN_TYPE_PRIORITY => {
                into.write_attr(SUMO_ATTR_COLOR, &to_string(&RGBColor::YELLOW));
            }
            NBSign_SignType::SIGN_TYPE_RIGHT_BEFORE_LEFT | NBSign_SignType::SIGN_TYPE_LEFT_BEFORE_RIGHT => {
                into.write_attr(SUMO_ATTR_COLOR, &to_string(&RGBColor::new(255, 153, 0, 255)));
            }
            NBSign_SignType::SIGN_TYPE_ROUNDABOUT => {
                into.write_attr(SUMO_ATTR_COLOR, &to_string(&RGBColor::BLUE));
            }
        }
        into.write_attr(SUMO_ATTR_X, &pos.x());
        into.write_attr(SUMO_ATTR_Y, &pos.y());
        into.write_attr(SUMO_ATTR_ANGLE, &navi_degree(shp.rotation_at_offset(self.myOffset)));
        // @todo add image resources and default images for all signs
        into.close_tag("");
    }

}

