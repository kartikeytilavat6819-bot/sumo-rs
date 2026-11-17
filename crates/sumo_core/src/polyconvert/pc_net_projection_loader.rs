//! Port of polyconvert/PCNetProjectionLoader.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use utils::xml::generic_sax_handler::GenericSAXHandler;
use crate::prelude::*;

/// A reader for a SUMO network's projection description
/// C++ class `PCNetProjectionLoader` (polyconvert/PCNetProjectionLoader.h)
#[repr(C)]
pub struct PCNetProjectionLoader {
    /// C++ base class `SUMOSAXHandler`
    pub base: GenericSAXHandler,
    /// Information whether the parameter was read
    pub myFoundLocation: bool,
    /// scaling of input coordinates (not given in the location element)
    pub myScale: f64,
}

impl std::ops::Deref for PCNetProjectionLoader {
    type Target = GenericSAXHandler;
    fn deref(&self) -> &GenericSAXHandler {
        &self.base
    }
}
impl std::ops::DerefMut for PCNetProjectionLoader {
    fn deref_mut(&mut self) -> &mut GenericSAXHandler {
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
        if !utils::common::file_helpers::is_readable(file) {
            panic!("{}", utils::tlf!("Could not open net-file '%'.", file));
        }
        // build handler and parser
        let mut handler = PCNetProjectionLoader::new(scale);
        handler.set_file_name(file);
        let mut parser = utils::xml::xml_sub_sys::get_sax_reader(true, false);
        let before = current_millis();
        utils::common::msg_handler::progress_begin_message(&format!("Parsing network projection from '{}'", file));
        if !or_panic(parser.parse_first(&mut handler, file)) {
            panic!("{}", utils::tlf!("Can not read XML-file '%'.", handler.get_file_name()));
        }
        // parse
        while or_panic(parser.parse_next(&mut handler)) && !handler.has_read_all() {}
        // clean up
        utils::common::msg_handler::progress_time_message(current_millis() - before);
        if !handler.has_read_all() {
            panic!("{}", "Could not find projection parameter in net.");
        }
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
        PCNetProjectionLoader {
            base: utils::xml::sumo_sax_handler::new("sumo-network", ""),
            myFoundLocation: false,
            myScale: scale,
        }
    }

    /// C++: `void PCNetProjectionLoader::~PCNetProjectionLoader() noexcept` [virtual, protected]
    /// Destructor
    // PCNetProjectionLoader::~PCNetProjectionLoader() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty; the Rust fields are dropped automatically
    }

    /// C++: `bool PCNetProjectionLoader::hasReadAll() const` [const, protected]
    /// Returns whether all needed values were read
    // bool
    // PCNetProjectionLoader::hasReadAll() const {
    //     return myFoundLocation;
    // }
    #[allow(unused_variables)]
    pub fn has_read_all(&self) -> bool {
        self.myFoundLocation
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
        if element != SUMO_TAG_LOCATION {
            return;
        }
        // @todo refactor parsing of location since its duplicated in NLHandler and PCNetProjectionLoader
        self.myFoundLocation = true;
        let s: PositionVector = attrs.get::<PositionVector>(SUMO_ATTR_NET_OFFSET, "", &mut self.myFoundLocation, true);
        let convBoundary: Boundary = attrs.get::<Boundary>(SUMO_ATTR_CONV_BOUNDARY, "", &mut self.myFoundLocation, true);
        let origBoundary: Boundary = attrs.get::<Boundary>(SUMO_ATTR_ORIG_BOUNDARY, "", &mut self.myFoundLocation, true);
        let proj: String = attrs.get::<String>(SUMO_ATTR_ORIG_PROJ, "", &mut self.myFoundLocation, true);
        if self.myFoundLocation {
            let networkOffset = s[0] + Position::new(utils::options::options_cont::get_float("offset.x"), utils::options::options_cont::get_float("offset.y"));
            GeoConvHelper::init(&proj, networkOffset, origBoundary, convBoundary, self.myScale);
        }
    }

}

impl SaxHandler for PCNetProjectionLoader {
    fn handler(&mut self) -> &mut GenericSAXHandler {
        &mut self.base
    }
    fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) -> Result<(), ProcessError> {
        PCNetProjectionLoader::my_start_element(self, element, attrs);
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

