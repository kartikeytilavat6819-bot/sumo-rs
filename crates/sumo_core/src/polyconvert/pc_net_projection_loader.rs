//! Port of polyconvert/PCNetProjectionLoader.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// A reader for a SUMO network's projection description
/// C++ class `PCNetProjectionLoader` (polyconvert/PCNetProjectionLoader.h)
pub struct PCNetProjectionLoader {
    /// C++ base class `SUMOSAXHandler`
    pub base: Unmapped /* SUMOSAXHandler */,
    /// Information whether the parameter was read
    pub myFoundLocation: bool,
    /// scaling of input coordinates (not given in the location element)
    pub myScale: f64,
}

impl std::ops::Deref for PCNetProjectionLoader {
    type Target = Unmapped /* SUMOSAXHandler */;
    fn deref(&self) -> &Unmapped /* SUMOSAXHandler */ {
        &self.base
    }
}
impl std::ops::DerefMut for PCNetProjectionLoader {
    fn deref_mut(&mut self) -> &mut Unmapped /* SUMOSAXHandler */ {
        &mut self.base
    }
}

impl PCNetProjectionLoader {
    /// C++: `void PCNetProjectionLoader::load(const std::string &, double)` [static]
    /// Loads network projection if wished
    // void
    // PCNetProjectionLoader::load(const std::string& file, double scale) {
    //     if (!FileHelpers::isReadable(file)) {
    //         throw ProcessError(TLF("Could not open net-file '%'.", file));
    //     }
    //     // build handler and parser
    //     PCNetProjectionLoader handler(scale);
    //     handler.setFileName(file);
    //     SUMOSAXReader* parser = XMLSubSys::getSAXReader(handler, true);
    //     const long before = PROGRESS_BEGIN_TIME_MESSAGE("Parsing network projection from '" + file + "'");
    //     if (!parser->parseFirst(file)) {
    //         delete parser;
    //         throw ProcessError(TLF("Can not read XML-file '%'.", handler.getFileName()));
    //     }
    //     // parse
    //     while (parser->parseNext() && !handler.hasReadAll());
    //     // clean up
    //     PROGRESS_TIME_MESSAGE(before);
    //     if (!handler.hasReadAll()) {
    //         throw ProcessError(TL("Could not find projection parameter in net."));
    //     }
    //     delete parser;
    // }
    #[allow(unused_variables)]
    pub fn load(file: &str, scale: f64) {
        todo!("port PCNetProjectionLoader::load")
    }

    /// C++: `void PCNetProjectionLoader::PCNetProjectionLoader(double)` [protected]
    /// Constructor
    // PCNetProjectionLoader::PCNetProjectionLoader(double scale) :
    //     SUMOSAXHandler("sumo-network"),
    //     myFoundLocation(false),
    //     myScale(scale) {
    // }
    #[allow(unused_variables)]
    pub fn new(scale: f64) -> Self {
        todo!("port PCNetProjectionLoader::PCNetProjectionLoader")
    }

    /// C++: `void PCNetProjectionLoader::~PCNetProjectionLoader() noexcept` [virtual, protected]
    /// Destructor
    // PCNetProjectionLoader::~PCNetProjectionLoader() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port PCNetProjectionLoader::~PCNetProjectionLoader")
    }

    /// C++: `bool PCNetProjectionLoader::hasReadAll() const` [const, protected]
    /// Returns whether all needed values were read
    // bool
    // PCNetProjectionLoader::hasReadAll() const {
    //     return myFoundLocation;
    // }
    #[allow(unused_variables)]
    pub fn has_read_all(&self) -> bool {
        todo!("port PCNetProjectionLoader::hasReadAll")
    }

    /// C++: `void PCNetProjectionLoader::myStartElement(int, const SUMOSAXAttributes &)` [virtual, protected]
    /// Called on the opening of a tag;
    // void
    // PCNetProjectionLoader::myStartElement(int element,
    //                                       const SUMOSAXAttributes& attrs) {
    //     if (element != SUMO_TAG_LOCATION) {
    //         return;
    //     }
    //     // @todo refactor parsing of location since its duplicated in NLHandler and PCNetProjectionLoader
    //     myFoundLocation = true;
    //     PositionVector s = attrs.get<PositionVector>(SUMO_ATTR_NET_OFFSET, nullptr, myFoundLocation);
    //     Boundary convBoundary = attrs.get<Boundary>(SUMO_ATTR_CONV_BOUNDARY, nullptr, myFoundLocation);
    //     Boundary origBoundary = attrs.get<Boundary>(SUMO_ATTR_ORIG_BOUNDARY, nullptr, myFoundLocation);
    //     std::string proj = attrs.get<std::string>(SUMO_ATTR_ORIG_PROJ, nullptr, myFoundLocation);
    //     if (myFoundLocation) {
    //         OptionsCont& oc = OptionsCont::getOptions();
    //         Position networkOffset = s[0] + Position(oc.getFloat("offset.x"), oc.getFloat("offset.y"));
    //         GeoConvHelper::init(proj, networkOffset, origBoundary, convBoundary, myScale);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) {
        todo!("port PCNetProjectionLoader::myStartElement")
    }

}

