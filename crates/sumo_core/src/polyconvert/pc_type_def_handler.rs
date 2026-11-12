//! Port of polyconvert/PCTypeDefHandler.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// A handler for loading polygon type maps
/// C++ class `PCTypeDefHandler` (polyconvert/PCTypeDefHandler.h)
#[repr(C)]
pub struct PCTypeDefHandler {
    /// C++ base class `SUMOSAXHandler`
    pub base: Unmapped /* SUMOSAXHandler */,
    /// The options (program settings)
    pub myOptions: Ptr<OptionsCont>,
    /// The type map to fill
    pub myContainer: Ptr<PCTypeMap>,
    /// whether to replace the loaded type with a custom value
    pub myOverwriteType: bool,
}

impl std::ops::Deref for PCTypeDefHandler {
    type Target = Unmapped /* SUMOSAXHandler */;
    fn deref(&self) -> &Unmapped /* SUMOSAXHandler */ {
        &self.base
    }
}
impl std::ops::DerefMut for PCTypeDefHandler {
    fn deref_mut(&mut self) -> &mut Unmapped /* SUMOSAXHandler */ {
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
        todo!("port PCTypeDefHandler::PCTypeDefHandler")
    }

    /// C++: `void PCTypeDefHandler::~PCTypeDefHandler() noexcept` [virtual]
    /// Destructor
    // PCTypeDefHandler::~PCTypeDefHandler() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port PCTypeDefHandler::~PCTypeDefHandler")
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
        todo!("port PCTypeDefHandler::myStartElement")
    }

    /// C++: `void PCTypeDefHandler::PCTypeDefHandler(const PCTypeDefHandler &)` [private]
    /// Invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(src: &PCTypeDefHandler) -> Self {
        todo!("port PCTypeDefHandler::PCTypeDefHandler")
    }

    /// C++: `PCTypeDefHandler & PCTypeDefHandler::operator=(const PCTypeDefHandler &)` [private]
    /// Invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, src: &PCTypeDefHandler) -> &mut PCTypeDefHandler {
        todo!("port PCTypeDefHandler::operator=")
    }

}

