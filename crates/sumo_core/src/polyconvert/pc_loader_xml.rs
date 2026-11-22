//! Port of polyconvert/PCLoaderXML.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// Trampoline for the C++ virtual `PCLoaderXML::getLanePos` (PCLoaderXML.h:65).
fn get_lane_pos_virtual(base: &mut ShapeHandler, poiID: &str, laneID: &str, lanePos: f64, friendlyPos: bool, lanePosLat: f64) -> Position {
    // SAFETY: PCLoaderXML is #[repr(C)] and base is its first field.
    let this = unsafe { &mut *(base as *mut ShapeHandler as *mut PCLoaderXML) };
    PCLoaderXML::get_lane_pos(this, poiID, laneID, lanePos, friendlyPos, lanePosLat)
}

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
        if !or_panic(oc.is_set("xml-files", true)) {
            return;
        }
        let mut handler = PCLoaderXML::new(toFill, tm, oc);
        // parse file(s)
        let files: Vec<String> = or_panic(oc.get_string_vector("xml")).clone();
        for file in files.iter() {
            if !utils::common::file_helpers::is_readable(file) {
                panic!("{}", utils::tlf!("Could not open xml-file '%'.", file));
            }
            let before = current_millis();
            utils::common::msg_handler::progress_begin_message(&format!("Parsing XML from '{}'", file));
            if !or_panic(utils::xml::xml_sub_sys::run_parser(&mut handler, file, false, false, false, true)) {
                std::panic::panic_any(ProcessError::default_msg());
            }
            utils::common::msg_handler::progress_time_message(current_millis() - before);
        }
    }

    /// C++: `Position PCLoaderXML::getLanePos(const std::string &, const std::string &, double, bool, double)` [virtual]
    // Position
    // PCLoaderXML::getLanePos(const std::string& poiID, const std::string& laneID, double lanePos, bool friendlyPos, double lanePosLat) {
    //     static_cast<PCPolyContainer&>(myShapeContainer).addLanePos(poiID, laneID, lanePos, friendlyPos, lanePosLat);
    //     return Position::INVALID;
    // }
    #[allow(unused_variables)]
    pub fn get_lane_pos(&mut self, poiID: &str, laneID: &str, lanePos: f64, friendlyPos: bool, lanePosLat: f64) -> Position {
        self.myShapeContainer.cast::<PCPolyContainer>().m().add_lane_pos(poiID, laneID, lanePos, friendlyPos, lanePosLat);
        Position::INVALID
    }

    /// C++: `void PCLoaderXML::PCLoaderXML(PCPolyContainer &, PCTypeMap &, OptionsCont &)` [protected]
    /// Constructor
    // PCLoaderXML::PCLoaderXML(PCPolyContainer& toFill,
    //                          PCTypeMap& tm, OptionsCont& oc)
    //     : ShapeHandler("xml-poi-definition", toFill),
    //       myTypeMap(tm), myOptions(oc) {}
    #[allow(unused_variables)]
    pub fn new(toFill: &mut PCPolyContainer, tm: &mut PCTypeMap, oc: &mut OptionsCont) -> Self {
        let mut base = ShapeHandler::new("xml-poi-definition", &mut toFill.base, Ptr::null());
        // v1_28_0 PCLoaderXML.cpp: getLanePos is virtual; wire the subclass override so
        // ShapeHandler's Parsed-POI lane/pos path does not hit the pure-virtual base.
        base.get_lane_pos_override = Some(get_lane_pos_virtual);
        PCLoaderXML {
            base,
            myTypeMap: Ptr::from_mut(tm),
            myOptions: Ptr::from_mut(oc),
        }
    }

    /// C++: `void PCLoaderXML::~PCLoaderXML() noexcept` [virtual, protected]
    /// Destructor
    // PCLoaderXML::~PCLoaderXML() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty; the Rust fields are dropped automatically
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
        if element != SUMO_TAG_POI && element != SUMO_TAG_POLY {
            return;
        }
        let mut ok = true;
        // get the id, report an error if not given or empty...
        let id: String = attrs.get::<String>(SUMO_ATTR_ID, "", &mut ok, true);
        let options = self.myOptions;
        let ty: String = attrs.get_opt::<String>(SUMO_ATTR_TYPE, &id, &mut ok, or_panic(options.get_string("type")), true);
        if !ok {
            return;
        }
        // patch the values
        let mut discard: bool = or_panic(options.get_bool("discard"));
        let tm = self.myTypeMap;
        if tm.m().has(&ty) {
            let def = tm.m().get(&ty);
            discard = def.discard;
            self.set_defaults(&def.prefix, &def.color, &def.icon, def.layer, def.allowFill != PCTypeMap_Filltype::NOFILL);
        } else {
            let prefix = or_panic(options.get_string("prefix"));
            let color = or_panic(RGBColor::parse_color(&or_panic(options.get_string("color"))));
            let icon = or_panic(options.get_string("icon"));
            let layer = or_panic(options.get_float("layer"));
            let fill = or_panic(options.get_bool("fill"));
            self.set_defaults(&prefix, &color, &icon, layer, fill);
        }
        if !discard {
            if element == SUMO_TAG_POI {
                let keep = or_panic(options.is_in_string_vector("prune.keep-list", &id));
                self.add_poi(attrs, keep, true);
            }
            if element == SUMO_TAG_POLY {
                let keep = or_panic(options.is_in_string_vector("prune.keep-list", &id));
                self.add_poly(attrs, keep, true);
            }
        }
    }

}

impl SaxHandler for PCLoaderXML {
    fn handler(&mut self) -> &mut utils::xml::generic_sax_handler::GenericSAXHandler {
        &mut self.base.base
    }
    fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) -> Result<(), ProcessError> {
        PCLoaderXML::my_start_element(self, element, attrs);
        Ok(())
    }
    fn my_end_element(&mut self, element: i32) -> Result<(), ProcessError> {
        ShapeHandler::my_end_element(&mut self.base, element);
        Ok(())
    }
}

/// SysUtils::getCurrentMillis()
fn current_millis() -> i64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as i64).unwrap_or(0)
}

fn or_panic<T, E: std::fmt::Display>(r: Result<T, E>) -> T {
    r.unwrap_or_else(|e| panic!("{e}"))
}

