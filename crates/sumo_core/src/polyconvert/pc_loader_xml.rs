//! Port of polyconvert/PCLoaderXML.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// A reader for polygons and pois stored in XML-format
/// C++ class `PCLoaderXML` (polyconvert/PCLoaderXML.h)
#[repr(C)]
pub struct PCLoaderXML {
    /// C++ base class `ShapeHandler`
    pub base: ShapeHandler,
    /// The type map to use
    pub myTypeMap: Ptr<PCTypeMap>,
    /// Settings to use
    pub myOptions: Ptr<OptionsCont>,
}

impl std::ops::Deref for PCLoaderXML {
    type Target = ShapeHandler;
    fn deref(&self) -> &ShapeHandler {
        &self.base
    }
}
impl std::ops::DerefMut for PCLoaderXML {
    fn deref_mut(&mut self) -> &mut ShapeHandler {
        &mut self.base
    }
}

impl PCLoaderXML {
    /// C++: `void PCLoaderXML::loadIfSet(OptionsCont &, PCPolyContainer &, PCTypeMap &)` [static]
    /// Loads pois/polygons assumed to be stored as XML
    // void
    // PCLoaderXML::loadIfSet(OptionsCont& oc, PCPolyContainer& toFill,
    //                        PCTypeMap& tm) {
    //     if (!oc.isSet("xml-files")) {
    //         return;
    //     }
    //     PCLoaderXML handler(toFill, tm, oc);
    //     // parse file(s)
    //     std::vector<std::string> files = oc.getStringVector("xml");
    //     for (std::vector<std::string>::const_iterator file = files.begin(); file != files.end(); ++file) {
    //         if (!FileHelpers::isReadable(*file)) {
    //             throw ProcessError(TLF("Could not open xml-file '%'.", *file));
    //         }
    //         const long before = PROGRESS_BEGIN_TIME_MESSAGE("Parsing XML from '" + *file + "'");
    //         if (!XMLSubSys::runParser(handler, *file)) {
    //             throw ProcessError();
    //         }
    //         PROGRESS_TIME_MESSAGE(before);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn load_if_set(oc: &mut OptionsCont, toFill: &mut PCPolyContainer, tm: &mut PCTypeMap) {
        todo!("port PCLoaderXML::loadIfSet")
    }

    /// C++: `Position PCLoaderXML::getLanePos(const std::string &, const std::string &, double, bool, double)` [virtual]
    // Position
    // PCLoaderXML::getLanePos(const std::string& poiID, const std::string& laneID, double lanePos, bool friendlyPos, double lanePosLat) {
    //     static_cast<PCPolyContainer&>(myShapeContainer).addLanePos(poiID, laneID, lanePos, friendlyPos, lanePosLat);
    //     return Position::INVALID;
    // }
    #[allow(unused_variables)]
    pub fn get_lane_pos(&mut self, poiID: &str, laneID: &str, lanePos: f64, friendlyPos: bool, lanePosLat: f64) -> Position {
        todo!("port PCLoaderXML::getLanePos")
    }

    /// C++: `void PCLoaderXML::PCLoaderXML(PCPolyContainer &, PCTypeMap &, OptionsCont &)` [protected]
    /// Constructor
    // PCLoaderXML::PCLoaderXML(PCPolyContainer& toFill,
    //                          PCTypeMap& tm, OptionsCont& oc)
    //     : ShapeHandler("xml-poi-definition", toFill),
    //       myTypeMap(tm), myOptions(oc) {}
    #[allow(unused_variables)]
    pub fn new(toFill: &mut PCPolyContainer, tm: &mut PCTypeMap, oc: &mut OptionsCont) -> Self {
        todo!("port PCLoaderXML::PCLoaderXML")
    }

    /// C++: `void PCLoaderXML::~PCLoaderXML() noexcept` [virtual, protected]
    /// Destructor
    // PCLoaderXML::~PCLoaderXML() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port PCLoaderXML::~PCLoaderXML")
    }

    /// C++: `void PCLoaderXML::myStartElement(int, const SUMOSAXAttributes &)` [virtual, protected]
    /// Called on the opening of a tag;
    // void
    // PCLoaderXML::myStartElement(int element,
    //                             const SUMOSAXAttributes& attrs) {
    //     if (element != SUMO_TAG_POI && element != SUMO_TAG_POLY) {
    //         return;
    //     }
    //     bool ok = true;
    //     // get the id, report an error if not given or empty...
    //     std::string id = attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
    //     std::string type = attrs.getOpt<std::string>(SUMO_ATTR_TYPE, id.c_str(), ok, myOptions.getString("type"));
    //     if (!ok) {
    //         return;
    //     }
    //     // patch the values
    //     bool discard = myOptions.getBool("discard");
    //     if (myTypeMap.has(type)) {
    //         const PCTypeMap::TypeDef& def = myTypeMap.get(type);
    //         discard = def.discard;
    //         setDefaults(def.prefix, def.color, def.icon, def.layer, def.allowFill != PCTypeMap::Filltype::NOFILL);
    //     } else {
    //         setDefaults(myOptions.getString("prefix"), RGBColor::parseColor(myOptions.getString("color")),
    //                     myOptions.getString("icon"), myOptions.getFloat("layer"), myOptions.getBool("fill"));
    //     }
    //     if (!discard) {
    //         if (element == SUMO_TAG_POI) {
    //             addPOI(attrs, myOptions.isInStringVector("prune.keep-list", id), true);
    //         }
    //         if (element == SUMO_TAG_POLY) {
    //             addPoly(attrs, myOptions.isInStringVector("prune.keep-list", id), true);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) {
        todo!("port PCLoaderXML::myStartElement")
    }

}

