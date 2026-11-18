//! Port of polyconvert/PCTypeDefHandler.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use utils::xml::generic_sax_handler::GenericSAXHandler;
use crate::prelude::*;

/// A handler for loading polygon type maps
/// C++ class `PCTypeDefHandler` (polyconvert/PCTypeDefHandler.h)
#[repr(C)]
pub struct PCTypeDefHandler {
    /// C++ base class `SUMOSAXHandler`
    pub base: GenericSAXHandler,
    /// The options (program settings)
    pub myOptions: Ptr<OptionsCont>,
    /// The type map to fill
    pub myContainer: Ptr<PCTypeMap>,
    /// whether to replace the loaded type with a custom value
    pub myOverwriteType: bool,
}

impl std::ops::Deref for PCTypeDefHandler {
    type Target = GenericSAXHandler;
    fn deref(&self) -> &GenericSAXHandler {
        &self.base
    }
}
impl std::ops::DerefMut for PCTypeDefHandler {
    fn deref_mut(&mut self) -> &mut GenericSAXHandler {
        &mut self.base
    }
}

impl PCTypeDefHandler {
    /// C++: `void PCTypeDefHandler::PCTypeDefHandler(OptionsCont &, PCTypeMap &)`
    /// Constructor
    // PCTypeDefHandler::PCTypeDefHandler(OptionsCont& oc, PCTypeMap& con) :
    //     SUMOSAXHandler(),
    //     myOptions(oc),
    //     myContainer(con),
    //     myOverwriteType(!oc.isDefault("type"))
    // {}
    #[allow(unused_variables)]
    pub fn new(oc: &mut OptionsCont, con: &mut PCTypeMap) -> Self {
        let overwrite = !oc.is_default("type");
        PCTypeDefHandler {
            base: utils::xml::sumo_sax_handler::new("", ""),
            myOptions: Ptr::from_mut(oc),
            myContainer: Ptr::from_mut(con),
            myOverwriteType: overwrite,
        }
    }

    /// C++: `void PCTypeDefHandler::~PCTypeDefHandler() noexcept` [virtual]
    /// Destructor
    // PCTypeDefHandler::~PCTypeDefHandler() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty; the Rust fields are dropped automatically
    }

    /// C++: `void PCTypeDefHandler::myStartElement(int, const SUMOSAXAttributes &)` [virtual, protected]
    /// Called on the opening of a tag;
    // void
    // PCTypeDefHandler::myStartElement(int element,
    //                                  const SUMOSAXAttributes& attrs) {
    //     if (element == SUMO_TAG_POLYTYPE) {
    //         bool ok = true;
    //         // get the id, report an error if not given or empty...
    //         std::string id = attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
    //         if (!ok) {
    //             return;
    //         }
    //         const std::string icon = attrs.getOpt<std::string>(SUMO_ATTR_ICON, id.c_str(), ok, myOptions.getString("icon"));
    //         const double layer = attrs.getOpt<double>(SUMO_ATTR_LAYER, id.c_str(), ok, myOptions.getFloat("layer"));
    //         const bool discard = attrs.getOpt<bool>(SUMO_ATTR_DISCARD, id.c_str(), ok, false);
    //         PCTypeMap::Filltype allowFill = myOptions.getBool("fill") ? PCTypeMap::Filltype::FILL : PCTypeMap::Filltype::NOFILL;
    //         if (attrs.hasAttribute(SUMO_ATTR_FILL)) {
    //             const std::string allowFillS = attrs.get<std::string>(SUMO_ATTR_FILL, id.c_str(), ok);
    //             if (allowFillS == "force") {
    //                 allowFill = PCTypeMap::Filltype::FORCE;
    //             } else {
    //                 allowFill = StringUtils::toBool(allowFillS) ? PCTypeMap::Filltype::FILL : PCTypeMap::Filltype::NOFILL;
    //             }
    //         }
    //         const std::string type = attrs.getOpt<std::string>(SUMO_ATTR_NAME, id.c_str(), ok, myOverwriteType ? myOptions.getString("type") : id);
    //         const std::string prefix = attrs.getOpt<std::string>(SUMO_ATTR_PREFIX, id.c_str(), ok, myOptions.getString("prefix"));
    //         const std::string color = attrs.getOpt<std::string>(SUMO_ATTR_COLOR, id.c_str(), ok, myOptions.getString("color"));
    //         const double angle = attrs.getOpt<double>(SUMO_ATTR_ANGLE, id.c_str(), ok, Shape::DEFAULT_ANGLE);
    //         const std::string imgFile = attrs.getOpt<std::string>(SUMO_ATTR_IMGFILE, id.c_str(), ok, Shape::DEFAULT_IMG_FILE);
    //         // !!! what about error handling?
    //         if (!myContainer.add(id, type, color, prefix, icon, layer, angle, imgFile, discard, allowFill)) {
    //             WRITE_ERRORF(TL("Could not add polygon type '%' (probably the id is already used)."), id);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) {
        if element == SUMO_TAG_POLYTYPE {
            let mut ok = true;
            // get the id, report an error if not given or empty...
            let id: String = attrs.get::<String>(SUMO_ATTR_ID, "", &mut ok, true);
            if !ok {
                return;
            }
            let options = self.myOptions;
            let icon: String = attrs.get_opt::<String>(SUMO_ATTR_ICON, &id, &mut ok, or_panic(options.get_string("icon")), true);
            let layer: f64 = attrs.get_opt::<f64>(SUMO_ATTR_LAYER, &id, &mut ok, or_panic(options.get_float("layer")), true);
            let discard: bool = attrs.get_opt::<bool>(SUMO_ATTR_DISCARD, &id, &mut ok, false, true);
            let mut allowFill = if or_panic(options.get_bool("fill")) { PCTypeMap_Filltype::FILL } else { PCTypeMap_Filltype::NOFILL };
            if attrs.has_attribute(SUMO_ATTR_FILL) {
                let allowFillS: String = attrs.get::<String>(SUMO_ATTR_FILL, &id, &mut ok, true);
                if allowFillS == "force" {
                    allowFill = PCTypeMap_Filltype::FORCE;
                } else {
                    allowFill = if or_panic(utils::common::string_utils::to_bool(&allowFillS)) { PCTypeMap_Filltype::FILL } else { PCTypeMap_Filltype::NOFILL };
                }
            }
            let ty: String = attrs.get_opt::<String>(SUMO_ATTR_NAME, &id, &mut ok, if self.myOverwriteType { or_panic(options.get_string("type")) } else { id.clone() }, true);
            let prefix: String = attrs.get_opt::<String>(SUMO_ATTR_PREFIX, &id, &mut ok, or_panic(options.get_string("prefix")), true);
            let color: String = attrs.get_opt::<String>(SUMO_ATTR_COLOR, &id, &mut ok, or_panic(options.get_string("color")), true);
            let angle: f64 = attrs.get_opt::<f64>(SUMO_ATTR_ANGLE, &id, &mut ok, DEFAULT_ANGLE, true);
            let imgFile: String = attrs.get_opt::<String>(SUMO_ATTR_IMGFILE, &id, &mut ok, DEFAULT_IMG_FILE.to_string(), true);
            // !!! what about error handling?
            if !self.myContainer.m().add(&id, &ty, &color, &prefix, &icon, layer, angle, &imgFile, discard, allowFill) {
                write_errorf!("Could not add polygon type '%' (probably the id is already used).", id);
            }
        }
    }

    /// C++: `void PCTypeDefHandler::PCTypeDefHandler(const PCTypeDefHandler &)` [private]
    /// Invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(src: &PCTypeDefHandler) -> Self {
        panic!("PCTypeDefHandler copy constructor is private and never defined in C++; copying is not allowed")
    }

    /// C++: `PCTypeDefHandler & PCTypeDefHandler::operator=(const PCTypeDefHandler &)` [private]
    /// Invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, src: &PCTypeDefHandler) -> &mut PCTypeDefHandler {
        panic!("PCTypeDefHandler::operator= is private and never defined in C++; assignment is not allowed")
    }

}

// C++ TL()/WRITE_ERRORF argument evaluation: a failing options lookup throws, so it panics here
fn or_panic<T, E: std::fmt::Display>(r: Result<T, E>) -> T {
    r.unwrap_or_else(|e| panic!("{e}"))
}


impl utils::xml::generic_sax_handler::SaxHandler for PCTypeDefHandler {
    fn handler(&mut self) -> &mut GenericSAXHandler {
        &mut self.base
    }
    fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) -> Result<(), ProcessError> {
        PCTypeDefHandler::my_start_element(self, element, attrs);
        Ok(())
    }
}
