//! Port of netimport/vissim/NIImporter_Vissim.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::string_utils::{convert_umlaute, ends_with, hex_to_int, to_double, to_int, to_lower_case};
use utils::common::msg_handler::{progress_begin_message, progress_done_message};
use utils::common::string_tokenizer::StringTokenizer;
use utils::common::util_exceptions::ProcessError;
use utils::xml::generic_sax_handler::{GenericSAXHandler, SaxHandler};
use utils::xml::xml_sub_sys::run_parser;

/// C++ enum `NIImporter_Vissim::VissimXMLAttr`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum NIImporter_Vissim_VissimXMLAttr {
    VISSIM_ATTR_NOTHING = 0,
    VISSIM_ATTR_NO = 1,
    VISSIM_ATTR_NAME = 2,
    VISSIM_ATTR_X = 3,
    VISSIM_ATTR_Y = 4,
    VISSIM_ATTR_ZOFFSET = 5,
    VISSIM_ATTR_ZUSCHLAG1 = 6,
    VISSIM_ATTR_ZUSCHLAG2 = 7,
    VISSIM_ATTR_WIDTH = 8,
    VISSIM_ATTR_LINKBEHAVETYPE = 9,
    VISSIM_ATTR_LANE = 10,
    VISSIM_ATTR_POS = 11,
    VISSIM_ATTR_LINK = 12,
    VISSIM_ATTR_INTLINK = 13,
    VISSIM_ATTR_PERCENTAGE = 14,
    VISSIM_ATTR_DISTRICT = 15,
    VISSIM_ATTR_COLOR = 16,
    VISSIM_ATTR_KEY = 17,
    VISSIM_ATTR_FX = 18,
    VISSIM_ATTR_DESTLINK = 19,
    VISSIM_ATTR_DESTPOS = 20,
    VISSIM_ATTR_LINK1 = 21,
    VISSIM_ATTR_LINK2 = 22,
    VISSIM_ATTR_STATUS = 23,
}

/// C++ enum `NIImporter_Vissim::VissimXMLTag`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum NIImporter_Vissim_VissimXMLTag {
    VISSIM_TAG_NOTHING = 0,
    VISSIM_TAG_NETWORK = 1,
    VISSIM_TAG_LANES = 2,
    VISSIM_TAG_LANE = 3,
    VISSIM_TAG_LINK = 4,
    VISSIM_TAG_LINKS = 5,
    VISSIM_TAG_POINTS3D = 6,
    VISSIM_TAG_POINT3D = 7,
    VISSIM_TAG_LINKPOLYPOINT = 8,
    VISSIM_TAG_LINKPOLYPTS = 9,
    VISSIM_TAG_FROM = 10,
    VISSIM_TAG_TO = 11,
    VISSIM_TAG_VEHICLE_INPUT = 12,
    VISSIM_TAG_PARKINGLOT = 13,
    VISSIM_TAG_VEHICLE_CLASS = 14,
    VISSIM_TAG_INTOBJECTREF = 15,
    VISSIM_TAG_SPEED_DECISION = 16,
    VISSIM_TAG_SPEED_DIST = 17,
    VISSIM_TAG_DATAPOINT = 18,
    VISSIM_TAG_DECISION_STATIC = 19,
    VISSIM_TAG_ROUTE_STATIC = 20,
    VISSIM_TAG_CA = 21,
}

/// C++ typedef `std::map<std::string, RGBColor> NIImporter_Vissim::ColorMap`
pub type NIImporter_Vissim_ColorMap = BTreeMap<String, RGBColor>;

/// C++ typedef `std::map<std::string, NIVissimElement> NIImporter_Vissim::ToElemIDMap`
pub type NIImporter_Vissim_ToElemIDMap = BTreeMap<String, NIVissimElement>;

/// C++ typedef `std::map<NIVissimElement, VissimSingleTypeParser *> NIImporter_Vissim::ToParserMap`
pub type NIImporter_Vissim_ToParserMap = BTreeMap<NIVissimElement, Ptr<NIImporter_Vissim_VissimSingleTypeParser>>;

/// C++ typedef `std::map<std::string, std::list<std::string>> NIImporter_Vissim::nodeMap`
pub type NIImporter_Vissim_nodeMap = BTreeMap<String, VecDeque<String>>;

/// Importer for networks stored in Vissim format
/// C++ class `NIImporter_Vissim` (netimport/vissim/NIImporter_Vissim.h)
pub struct NIImporter_Vissim {
    pub elementData: NIImporter_Vissim_nodeMap,
    /// Map from element names to their numerical representation
    pub myKnownElements: NIImporter_Vissim_ToElemIDMap,
    /// Parsers by element id
    pub myParsers: NIImporter_Vissim_ToParserMap,
    /// a map from color names to color definitions
    pub myColorMap: NIImporter_Vissim_ColorMap,
    pub myLastSecure: String,
    pub myNetBuilder: Ptr<NBNetBuilder>,
}

/// GenericSAXHandler base shared by all VISSIM handlers (C++ `GenericSAXHandler(vissimTags, ..., "vissim - file")`)
fn vissim_base_handler() -> GenericSAXHandler {
    GenericSAXHandler::new(
        VISSIM_TAGS,
        NIImporter_Vissim_VissimXMLTag::VISSIM_TAG_NOTHING as i32,
        VISSIM_ATTRS,
        NIImporter_Vissim_VissimXMLAttr::VISSIM_ATTR_NOTHING as i32,
        "vissim - file",
        "",
    )
}

/// C++ `myElemData[key].front()` (operator[] inserts the key when missing; front of an empty deque is UB, empty string here)
fn elem_front(elem: &Ptr<NIImporter_Vissim_nodeMap>, key: &str) -> String {
    elem.m().entry(key.to_string()).or_default().front().cloned().unwrap_or_default()
}

/// C++ `myElemData[key].push_back(value)`
fn elem_push(elem: &Ptr<NIImporter_Vissim_nodeMap>, key: &str, value: String) {
    elem.m().entry(key.to_string()).or_default().push_back(value);
}

/// C++ `myElemData[key].pop_front()` returning the removed value, None when empty
fn elem_pop_front(elem: &Ptr<NIImporter_Vissim_nodeMap>, key: &str) -> Option<String> {
    elem.m().entry(key.to_string()).or_default().pop_front()
}

/// C++ throws ProcessError from StringUtils::toInt/toDouble; the Rust handlers are void, so the error ends the run as a panic
fn vissim_parse<T>(r: Result<T, ProcessError>) -> T {
    r.unwrap_or_else(|e| panic!("{e}"))
}

/// C++ `strm.good()`: the stream is positioned before its end (the stream is the whole file in memory)
fn vissim_good(from: &std::io::Cursor<Vec<u8>>) -> bool {
    (from.position() as usize) < from.get_ref().len()
}

/// C++ `std::isspace` for the characters a `>>` extraction skips
fn vissim_is_space(c: u8) -> bool {
    matches!(c, b' ' | b'\t' | b'\n' | b'\r' | 0x0b | 0x0c)
}

/// C++ `from >> tmp` for a std::string: skips whitespace, reads one whitespace-delimited token ("" at end of stream)
fn vissim_read_token(from: &mut std::io::Cursor<Vec<u8>>) -> String {
    let data = from.get_ref();
    let mut i = from.position() as usize;
    while i < data.len() && vissim_is_space(data[i]) {
        i += 1;
    }
    let start = i;
    while i < data.len() && !vissim_is_space(data[i]) {
        i += 1;
    }
    let tok = String::from_utf8_lossy(&data[start..i]).into_owned();
    from.set_position(i as u64);
    tok
}

/// C++ `from >> int` / `from >> double` (libstdc++ num_get): skips whitespace, then consumes the longest
/// prefix that can be part of a number, so "12m" gives "12" and "m" stays in the stream. Returns the consumed text.
pub fn vissim_read_number(from: &mut std::io::Cursor<Vec<u8>>, is_float: bool) -> String {
    let data = from.get_ref();
    let mut i = from.position() as usize;
    while i < data.len() && vissim_is_space(data[i]) {
        i += 1;
    }
    let start = i;
    if i < data.len() && (data[i] == b'+' || data[i] == b'-') {
        i += 1;
    }
    while i < data.len() && data[i].is_ascii_digit() {
        i += 1;
    }
    if is_float {
        if i < data.len() && data[i] == b'.' {
            i += 1;
            while i < data.len() && data[i].is_ascii_digit() {
                i += 1;
            }
        }
        if i < data.len() && (data[i] == b'e' || data[i] == b'E') {
            i += 1;
            if i < data.len() && (data[i] == b'+' || data[i] == b'-') {
                i += 1;
            }
            while i < data.len() && data[i].is_ascii_digit() {
                i += 1;
            }
        }
    }
    let tok = String::from_utf8_lossy(&data[start..i]).into_owned();
    from.set_position(i as u64);
    tok
}

/// C++ `from >> double`: text that does not parse gives 0 (C++11 behaviour)
fn vissim_read_f64(from: &mut std::io::Cursor<Vec<u8>>) -> f64 {
    vissim_read_number(from, true).parse::<f64>().unwrap_or(0.0)
}

/// C++ `from >> int` (see vissim_read_f64)
fn vissim_read_i32(from: &mut std::io::Cursor<Vec<u8>>) -> i32 {
    vissim_read_number(from, false).parse::<i32>().unwrap_or(0)
}

#[cfg(test)]
mod vissim_read_number_tests {
    use super::*;

    fn cursor(s: &str) -> std::io::Cursor<Vec<u8>> {
        std::io::Cursor::new(s.as_bytes().to_vec())
    }

    #[test]
    fn int_stops_at_first_non_digit() {
        let mut c = cursor("  12m 7");
        assert_eq!(vissim_read_i32(&mut c), 12);
        assert_eq!(vissim_read_token(&mut c), "m");
        assert_eq!(vissim_read_i32(&mut c), 7);
    }

    #[test]
    fn float_takes_exponent_and_stops_at_letter() {
        let mut c = cursor("1.5e2x -3.25");
        assert_eq!(vissim_read_f64(&mut c), 150.0);
        assert_eq!(vissim_read_token(&mut c), "x");
        assert_eq!(vissim_read_f64(&mut c), -3.25);
    }

    #[test]
    fn bad_number_gives_zero_and_skips_only_its_prefix() {
        let mut c = cursor("abc");
        assert_eq!(vissim_read_i32(&mut c), 0);
        assert_eq!(vissim_read_token(&mut c), "abc");
    }
}

/// C++ `std::getline(strm, line)`: reads up to and including the newline, which is not returned
fn vissim_getline(from: &mut std::io::Cursor<Vec<u8>>) -> String {
    let data = from.get_ref();
    let start = from.position() as usize;
    if start >= data.len() {
        return String::new();
    }
    let mut i = start;
    while i < data.len() && data[i] != b'\n' {
        i += 1;
    }
    let line = String::from_utf8_lossy(&data[start..i]).into_owned();
    let next = if i < data.len() { i + 1 } else { i };
    from.set_position(next as u64);
    line
}

// C++ static members of NIImporter_Vissim (port as module-level statics, see AGENTS.md):
//   static SequentialStringBijection::Entry[] NIImporter_Vissim::vissimTags;
//   static SequentialStringBijection::Entry[] NIImporter_Vissim::vissimAttrs;
const VISSIM_TAGS: &[(&str, i32)] = &[
    ("network", NIImporter_Vissim_VissimXMLTag::VISSIM_TAG_NETWORK as i32),
    ("lanes", NIImporter_Vissim_VissimXMLTag::VISSIM_TAG_LANES as i32),
    ("lane", NIImporter_Vissim_VissimXMLTag::VISSIM_TAG_LANE as i32),
    ("link", NIImporter_Vissim_VissimXMLTag::VISSIM_TAG_LINK as i32),
    ("links", NIImporter_Vissim_VissimXMLTag::VISSIM_TAG_LINKS as i32),
    ("points3D", NIImporter_Vissim_VissimXMLTag::VISSIM_TAG_POINTS3D as i32),
    ("point3D", NIImporter_Vissim_VissimXMLTag::VISSIM_TAG_POINT3D as i32),
    ("linkPolyPoint", NIImporter_Vissim_VissimXMLTag::VISSIM_TAG_LINKPOLYPOINT as i32),
    ("linkPolyPts", NIImporter_Vissim_VissimXMLTag::VISSIM_TAG_LINKPOLYPTS as i32),
    ("fromLinkEndPt", NIImporter_Vissim_VissimXMLTag::VISSIM_TAG_FROM as i32),
    ("toLinkEndPt", NIImporter_Vissim_VissimXMLTag::VISSIM_TAG_TO as i32),
    ("vehicleInput", NIImporter_Vissim_VissimXMLTag::VISSIM_TAG_VEHICLE_INPUT as i32),
    ("parkingLot", NIImporter_Vissim_VissimXMLTag::VISSIM_TAG_PARKINGLOT as i32),
    ("vehicleClass", NIImporter_Vissim_VissimXMLTag::VISSIM_TAG_VEHICLE_CLASS as i32),
    ("intObjectRef", NIImporter_Vissim_VissimXMLTag::VISSIM_TAG_INTOBJECTREF as i32),
    ("desSpeedDecision", NIImporter_Vissim_VissimXMLTag::VISSIM_TAG_SPEED_DECISION as i32),
    ("desSpeedDistribution", NIImporter_Vissim_VissimXMLTag::VISSIM_TAG_SPEED_DIST as i32),
    ("speedDistributionDataPoint", NIImporter_Vissim_VissimXMLTag::VISSIM_TAG_DATAPOINT as i32),
    ("vehicleRoutingDecisionStatic", NIImporter_Vissim_VissimXMLTag::VISSIM_TAG_DECISION_STATIC as i32),
    ("vehicleRouteStatic", NIImporter_Vissim_VissimXMLTag::VISSIM_TAG_ROUTE_STATIC as i32),
    ("conflictArea", NIImporter_Vissim_VissimXMLTag::VISSIM_TAG_CA as i32),
    ("", NIImporter_Vissim_VissimXMLTag::VISSIM_TAG_NOTHING as i32),
];
const VISSIM_ATTRS: &[(&str, i32)] = &[
    ("no", NIImporter_Vissim_VissimXMLAttr::VISSIM_ATTR_NO as i32),
    ("name", NIImporter_Vissim_VissimXMLAttr::VISSIM_ATTR_NAME as i32),
    ("x", NIImporter_Vissim_VissimXMLAttr::VISSIM_ATTR_X as i32),
    ("y", NIImporter_Vissim_VissimXMLAttr::VISSIM_ATTR_Y as i32),
    ("zOffset", NIImporter_Vissim_VissimXMLAttr::VISSIM_ATTR_ZOFFSET as i32),
    ("surch1", NIImporter_Vissim_VissimXMLAttr::VISSIM_ATTR_ZUSCHLAG1 as i32),
    ("surch2", NIImporter_Vissim_VissimXMLAttr::VISSIM_ATTR_ZUSCHLAG2 as i32),
    ("width", NIImporter_Vissim_VissimXMLAttr::VISSIM_ATTR_WIDTH as i32),
    ("linkBehavType", NIImporter_Vissim_VissimXMLAttr::VISSIM_ATTR_LINKBEHAVETYPE as i32),
    ("lane", NIImporter_Vissim_VissimXMLAttr::VISSIM_ATTR_LANE as i32),
    ("pos", NIImporter_Vissim_VissimXMLAttr::VISSIM_ATTR_POS as i32),
    ("link", NIImporter_Vissim_VissimXMLAttr::VISSIM_ATTR_LINK as i32),
    ("intLink", NIImporter_Vissim_VissimXMLAttr::VISSIM_ATTR_INTLINK as i32),
    ("relFlow", NIImporter_Vissim_VissimXMLAttr::VISSIM_ATTR_PERCENTAGE as i32),
    ("zone", NIImporter_Vissim_VissimXMLAttr::VISSIM_ATTR_DISTRICT as i32),
    ("color", NIImporter_Vissim_VissimXMLAttr::VISSIM_ATTR_COLOR as i32),
    ("key", NIImporter_Vissim_VissimXMLAttr::VISSIM_ATTR_KEY as i32),
    ("fx", NIImporter_Vissim_VissimXMLAttr::VISSIM_ATTR_FX as i32),
    ("destLink", NIImporter_Vissim_VissimXMLAttr::VISSIM_ATTR_DESTLINK as i32),
    ("destPos", NIImporter_Vissim_VissimXMLAttr::VISSIM_ATTR_DESTPOS as i32),
    ("link1", NIImporter_Vissim_VissimXMLAttr::VISSIM_ATTR_LINK1 as i32),
    ("link2", NIImporter_Vissim_VissimXMLAttr::VISSIM_ATTR_LINK2 as i32),
    ("status", NIImporter_Vissim_VissimXMLAttr::VISSIM_ATTR_STATUS as i32),
    ("", NIImporter_Vissim_VissimXMLAttr::VISSIM_ATTR_NOTHING as i32),
];

impl NIImporter_Vissim {
    /// C++: `void NIImporter_Vissim::loadNetwork(const OptionsCont &, NBNetBuilder &)` [static]
    /// Loads network definition from the assigned option and stores it in the given network builder
    // void
    // NIImporter_Vissim::loadNetwork(const OptionsCont& oc, NBNetBuilder& nb) {
    //     if (!oc.isSet("vissim-file")) {
    //         return;
    //     }
    //     NIImporter_Vissim(nb).load(oc);
    // }
    #[allow(unused_variables)]
    pub fn load_network(oc: &OptionsCont, nb: &mut NBNetBuilder) {
        if !oc.is_set("vissim-file", true).unwrap_or(false) {
            return;
        }
        let mut importer = NIImporter_Vissim::new(nb);
        importer.load(oc);
    }

    /// C++: `void NIImporter_Vissim::NIImporter_Vissim(NBNetBuilder &)` [protected]
    /// constructor
    // NIImporter_Vissim::NIImporter_Vissim(NBNetBuilder& nb) : myNetBuilder(nb) {
    //     insertKnownElements();
    //     buildParsers();
    //     myColorMap["blau"] = RGBColor(77, 77, 255, 255);
    //     myColorMap["gelb"] = RGBColor::YELLOW;
    //     myColorMap["grau"] = RGBColor::GREY;
    //     myColorMap["lila"] = RGBColor::MAGENTA;
    //     myColorMap["gruen"] = RGBColor::GREEN;
    //     myColorMap["rot"] = RGBColor::RED;
    //     myColorMap["schwarz"] = RGBColor::BLACK;
    //     myColorMap["tuerkis"] = RGBColor::CYAN;
    //     myColorMap["weiss"] = RGBColor::WHITE;
    //     myColorMap["keine"] = RGBColor::WHITE;
    // }
    #[allow(unused_variables)]
    pub fn new(nb: &mut NBNetBuilder) -> Box<Self> {
        // buildParsers hands `*this` to the parsers, so the importer must not move after construction: return the Box
        let mut b = Box::new(NIImporter_Vissim {
            elementData: BTreeMap::new(),
            myKnownElements: BTreeMap::new(),
            myParsers: BTreeMap::new(),
            myColorMap: BTreeMap::new(),
            myLastSecure: String::new(),
            myNetBuilder: Ptr::from_mut(nb),
        });
        b.insert_known_elements();
        b.build_parsers();
        let colors: [(&str, RGBColor); 10] = [
            ("blau", RGBColor::new(77, 77, 255, 255)),
            ("gelb", RGBColor::YELLOW),
            ("grau", RGBColor::GREY),
            ("lila", RGBColor::MAGENTA),
            ("gruen", RGBColor::GREEN),
            ("rot", RGBColor::RED),
            ("schwarz", RGBColor::BLACK),
            ("tuerkis", RGBColor::CYAN),
            ("weiss", RGBColor::WHITE),
            ("keine", RGBColor::WHITE),
        ];
        for (name, color) in colors {
            b.myColorMap.insert(name.to_string(), color);
        }
        b
    }

    /// C++: `void NIImporter_Vissim::~NIImporter_Vissim()` [protected]
    /// destructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // shortcut: the parsers are leaked (C++ deletes them here); freeing needs the concrete subclass type, so they are not freed
        NIVissimAbstractEdge::clear_dict();
        NIVissimClosures::clear_dict();
        NIVissimDistrictConnection::clear_dict();
        NIVissimDisturbance::clear_dict();
        NIVissimNodeCluster::clear_dict();
        NIVissimNodeDef::clear_dict();
        NIVissimSource::clear_dict();
        NIVissimTL::clear_dict();
        NIVissimTrafficDescription::clear_dict();
        NIVissimVehTypeClass::clear_dict();
        NIVissimVehicleType::clear_dict();
        NIVissimConnectionCluster::clear_dict();
        NIVissimConflictArea::clear_dict();
    }

    /// C++: `void NIImporter_Vissim::load(const OptionsCont &)` [protected]
    /// loads the vissim file
    // void
    // NIImporter_Vissim::load(const OptionsCont& options) {
    //     const std::string file = options.getString("vissim-file");
    //     // try to open the file
    //     std::ifstream strm(file.c_str());
    //     if (!strm.good()) {
    //         WRITE_ERRORF(TL("Could not open vissim-file '%'."), file);
    //         return;
    //     }
    //     std::string token;
    //     strm >> token;
    //     if (StringUtils::endsWith(file, ".inpx") || StringUtils::endsWith(token, "<?xml") || StringUtils::endsWith(token, "<network")) {
    //         // Create NIVissimXMLHandlers
    //         NIVissimXMLHandler_Streckendefinition XMLHandler_Streckendefinition(elementData);
    //         NIVissimXMLHandler_Zuflussdefinition XMLHandler_Zuflussdefinition;
    //         //NIVissimXMLHandler_Parkplatzdefinition XMLHandler_Parkplatzdefinition;
    //         NIVissimXMLHandler_Fahrzeugklassendefinition XMLHandler_Fahrzeugklassendefinition(elementData);
    //         NIVissimXMLHandler_Geschwindigkeitsverteilungsdefinition XMLHandler_Geschwindigkeitsverteilung(elementData);
    //         NIVissimXMLHandler_ConflictArea XMLHandler_ConflictAreas;
    //
    //         // Strecken + Verbinder
    //         XMLHandler_Streckendefinition.setFileName(file);
    //         PROGRESS_BEGIN_MESSAGE("Parsing strecken+verbinder from vissim-file '" + file + "'");
    //         if (!XMLSubSys::runParser(XMLHandler_Streckendefinition, file)) {
    //             return;
    //         }
    //         PROGRESS_DONE_MESSAGE();
    //
    //         // Zuflüsse
    //         XMLHandler_Zuflussdefinition.setFileName(file);
    //         PROGRESS_BEGIN_MESSAGE("Parsing zuflüsse from vissim-file '" + file + "'");
    //         if (!XMLSubSys::runParser(XMLHandler_Zuflussdefinition, file)) {
    //             return;
    //         }
    //         PROGRESS_DONE_MESSAGE();
    //
    //         //Geschwindigkeitsverteilungen
    //         XMLHandler_Geschwindigkeitsverteilung.setFileName(file);
    //         PROGRESS_BEGIN_MESSAGE("Parsing parkplätze from vissim-file '" + file + "'");
    //         if (!XMLSubSys::runParser(XMLHandler_Geschwindigkeitsverteilung, file)) {
    //             return;
    //         }
    //         PROGRESS_DONE_MESSAGE();
    //
    //
    //         //Fahrzeugklassen
    //         XMLHandler_Fahrzeugklassendefinition.setFileName(file);
    //         PROGRESS_BEGIN_MESSAGE("Parsing parkplätze from vissim-file '" + file + "'");
    //         if (!XMLSubSys::runParser(XMLHandler_Fahrzeugklassendefinition, file)) {
    //             return;
    //         }
    //         PROGRESS_DONE_MESSAGE();
    //
    //         //Parkplätze
    //         /*XMLHandler_Parkplatzdefinition.setFileName(file);
    //         PROGRESS_BEGIN_MESSAGE("Parsing parkplätze from vissim-file '" + file + "'");
    //         if (!XMLSubSys::runParser(XMLHandler_Parkplatzdefinition, file)) {
    //         return;
    //         }
    //         PROGRESS_DONE_MESSAGE();*/
    //
    //
    //         //Konfliktflächen
    //         XMLHandler_ConflictAreas.setFileName(file);
    //         PROGRESS_BEGIN_MESSAGE("Parsing conflict areas from vissim-file '" + file + "'");
    //         if (!XMLSubSys::runParser(XMLHandler_ConflictAreas, file)) {
    //             return;
    //         }
    //         PROGRESS_DONE_MESSAGE();
    //     } else {
    //         strm.seekg(strm.beg);
    //         if (!readContents(strm)) {
    //             return;
    //         }
    //     }
    //     postLoadBuild(options.getFloat("vissim.join-distance"));
    // }
    #[allow(unused_variables)]
    pub fn load(&mut self, options: &OptionsCont) {
        let file = vissim_parse(options.get_string("vissim-file"));
        // try to open the file
        let bytes = match std::fs::read(&file) {
            Ok(b) => b,
            Err(_) => {
                write_errorf!("Could not open vissim-file '%'.", file);
                return;
            }
        };
        let mut strm = std::io::Cursor::new(bytes);
        let token = vissim_read_token(&mut strm);
        if ends_with(&file, ".inpx") || ends_with(&token, "<?xml") || ends_with(&token, "<network") {
            // Create NIVissimXMLHandlers
            let mut XMLHandler_Streckendefinition = NIImporter_Vissim_NIVissimXMLHandler_Streckendefinition::new(&mut self.elementData);
            let mut XMLHandler_Zuflussdefinition = NIImporter_Vissim_NIVissimXMLHandler_Zuflussdefinition::new();
            //NIVissimXMLHandler_Parkplatzdefinition XMLHandler_Parkplatzdefinition;
            let mut XMLHandler_Fahrzeugklassendefinition = NIImporter_Vissim_NIVissimXMLHandler_Fahrzeugklassendefinition::new(&mut self.elementData);
            let mut XMLHandler_Geschwindigkeitsverteilung = NIImporter_Vissim_NIVissimXMLHandler_Geschwindigkeitsverteilungsdefinition::new(&mut self.elementData);
            let mut XMLHandler_ConflictAreas = NIImporter_Vissim_NIVissimXMLHandler_ConflictArea::new();

            // Strecken + Verbinder
            XMLHandler_Streckendefinition.set_file_name(&file);
            progress_begin_message(&format!("Parsing strecken+verbinder from vissim-file '{}'", file));
            if !run_parser(&mut XMLHandler_Streckendefinition, &file, false, false, false, true).unwrap_or(false) {
                return;
            }
            progress_done_message();

            // Zuflüsse
            XMLHandler_Zuflussdefinition.set_file_name(&file);
            progress_begin_message(&format!("Parsing zuflüsse from vissim-file '{}'", file));
            if !run_parser(&mut XMLHandler_Zuflussdefinition, &file, false, false, false, true).unwrap_or(false) {
                return;
            }
            progress_done_message();

            //Geschwindigkeitsverteilungen
            XMLHandler_Geschwindigkeitsverteilung.set_file_name(&file);
            progress_begin_message(&format!("Parsing parkplätze from vissim-file '{}'", file));
            if !run_parser(&mut XMLHandler_Geschwindigkeitsverteilung, &file, false, false, false, true).unwrap_or(false) {
                return;
            }
            progress_done_message();

            //Fahrzeugklassen
            XMLHandler_Fahrzeugklassendefinition.set_file_name(&file);
            progress_begin_message(&format!("Parsing parkplätze from vissim-file '{}'", file));
            if !run_parser(&mut XMLHandler_Fahrzeugklassendefinition, &file, false, false, false, true).unwrap_or(false) {
                return;
            }
            progress_done_message();

            //Parkplätze
            /*XMLHandler_Parkplatzdefinition.setFileName(file);
            PROGRESS_BEGIN_MESSAGE("Parsing parkplätze from vissim-file '" + file + "'");
            if (!XMLSubSys::runParser(XMLHandler_Parkplatzdefinition, file)) {
            return;
            }
            PROGRESS_DONE_MESSAGE();*/

            //Konfliktflächen
            XMLHandler_ConflictAreas.set_file_name(&file);
            progress_begin_message(&format!("Parsing conflict areas from vissim-file '{}'", file));
            if !run_parser(&mut XMLHandler_ConflictAreas, &file, false, false, false, true).unwrap_or(false) {
                return;
            }
            progress_done_message();
        } else {
            strm.set_position(0);
            if !self.read_contents(&mut strm) {
                return;
            }
        }
        self.post_load_build(vissim_parse(options.get_float("vissim.join-distance")));
    }

    /// C++: `bool NIImporter_Vissim::admitContinue(const std::string &)` [protected]
    // bool
    // NIImporter_Vissim::admitContinue(const std::string& tag) {
    //     ToElemIDMap::const_iterator i = myKnownElements.find(tag);
    //     if (i == myKnownElements.end()) {
    //         return true;
    //     }
    //     myLastSecure = tag;
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn admit_continue(&mut self, tag: &str) -> bool {
        if !self.myKnownElements.contains_key(tag) {
            return true;
        }
        self.myLastSecure = tag.to_string();
        false
    }

    /// C++: `bool NIImporter_Vissim::readContents(std::istream &)` [private]
    // bool
    // NIImporter_Vissim::readContents(std::istream& strm) {
    //     // read contents
    //     bool ok = true;
    //     while (strm.good() && ok) {
    //         std::string tag;
    //         if (myLastSecure != "") {
    //             tag = myLastSecure;
    //         } else {
    //             strm >> tag;
    //         }
    //         myLastSecure = "";
    //         bool parsed = false;
    //         while (!parsed && strm.good() && ok) {
    //             ToElemIDMap::iterator i = myKnownElements.find(StringUtils::to_lower_case(tag));
    //             if (i != myKnownElements.end()) {
    //                 ToParserMap::iterator j = myParsers.find((*i).second);
    //                 if (j != myParsers.end()) {
    //                     VissimSingleTypeParser* parser = (*j).second;
    //                     ok = parser->parse(strm);
    //                     parsed = true;
    //                 }
    //             }
    //             if (!parsed) {
    //                 std::string line;
    //                 std::streamoff pos;
    //                 do {
    //                     pos = strm.tellg();
    //                     getline(strm, line);
    //                 } while (strm.good() && (line == "" || line[0] == ' ' || line[0] == '-'));
    //                 if (!strm.good()) {
    //                     return true;
    //                 }
    //                 strm.seekg(pos);
    //                 strm >> tag;
    //             }
    //         }
    //     }
    //     return ok;
    // }
    #[allow(unused_variables)]
    pub fn read_contents(&mut self, strm: &mut std::io::Cursor<Vec<u8>> /* std::istream */) -> bool {
        // read contents
        let mut ok = true;
        while vissim_good(strm) && ok {
            let mut tag: String;
            if self.myLastSecure != "" {
                tag = self.myLastSecure.clone();
            } else {
                tag = vissim_read_token(strm);
            }
            self.myLastSecure = String::new();
            let mut parsed = false;
            while !parsed && vissim_good(strm) && ok {
                let elem = self.myKnownElements.get(&to_lower_case(&tag)).cloned();
                if let Some(elem) = elem {
                    if let Some(parser) = self.myParsers.get(&elem).cloned() {
                        // shortcut: myParsers stores the base VissimSingleTypeParser, so the subclass parse() is not dispatched (needs a trait object in myParsers), and the base parse() is pure virtual
                        ok = parser.m().parse(strm);
                        parsed = true;
                    }
                }
                if !parsed {
                    let mut line: String;
                    let mut pos: u64;
                    loop {
                        pos = strm.position();
                        line = vissim_getline(strm);
                        if !(vissim_good(strm) && (line == "" || line.starts_with(' ') || line.starts_with('-'))) {
                            break;
                        }
                    }
                    if !vissim_good(strm) {
                        return true;
                    }
                    strm.set_position(pos);
                    tag = vissim_read_token(strm);
                }
            }
        }
        ok
    }

    /// C++: `void NIImporter_Vissim::postLoadBuild(double)` [private]
    // void
    // NIImporter_Vissim::postLoadBuild(double offset) {
    //     // close the loading process
    //     NIVissimBoundedClusterObject::closeLoading();
    //     NIVissimConnection::dict_assignToEdges();
    //     NIVissimDisturbance::dict_SetDisturbances();
    //     // build district->connections map
    //     NIVissimDistrictConnection::dict_BuildDistrictConnections();
    //     // build clusters around nodes
    // //    NIVissimNodeDef::buildNodeClusters();
    //     // build node clusters around traffic lights
    // //    NIVissimTL::buildNodeClusters();
    //
    //     // when connections or disturbances are left, build nodes around them
    //
    //     // try to assign connection clusters to nodes
    //     //  only left connections will be processed in
    //     //   buildConnectionClusters & join
    // //30.4. brauchen wir noch!    NIVissimNodeDef::dict_assignConnectionsToNodes();
    //
    //     // build clusters of connections with the same direction and a similar position along the streets
    //     NIVissimEdge::buildConnectionClusters();
    //     // check whether further nodes (connection clusters by now) must be added
    //     NIVissimDistrictConnection::dict_CheckEdgeEnds();
    //
    //     // join clusters when overlapping (different streets are possible)
    //     NIVissimEdge::dict_checkEdges2Join();
    //     NIVissimConnectionCluster::joinBySameEdges(offset);
    // //    NIVissimConnectionCluster::joinByDisturbances(offset);
    //
    // //    NIVissimConnectionCluster::addTLs(offset);
    //
    //     // build nodes from clusters
    //     NIVissimNodeCluster::setCurrentVirtID(NIVissimNodeDef::getMaxID());
    //     NIVissimConnectionCluster::buildNodeClusters();
    //
    // //    NIVissimNodeCluster::dict_recheckEdgeChanges();
    //     NIVissimNodeCluster::buildNBNodes(myNetBuilder.getNodeCont());
    //     NIVissimDistrictConnection::dict_BuildDistrictNodes(
    //         myNetBuilder.getDistrictCont(), myNetBuilder.getNodeCont());
    //     NIVissimEdge::dict_propagateSpeeds();
    //     NIVissimEdge::dict_buildNBEdges(myNetBuilder.getDistrictCont(), myNetBuilder.getNodeCont(), myNetBuilder.getEdgeCont(), offset);
    //     if (OptionsCont::getOptions().getBool("vissim.report-unset-speeds")) {
    //         NIVissimEdge::reportUnsetSpeeds();
    //     }
    //     NIVissimDistrictConnection::dict_BuildDistricts(myNetBuilder.getDistrictCont(), myNetBuilder.getEdgeCont(), myNetBuilder.getNodeCont());
    //     NIVissimConnection::dict_buildNBEdgeConnections(myNetBuilder.getEdgeCont());
    //     NIVissimNodeCluster::dict_addDisturbances(myNetBuilder.getDistrictCont(), myNetBuilder.getNodeCont(), myNetBuilder.getEdgeCont());
    //     NIVissimConflictArea::setPriorityRegulation(myNetBuilder.getEdgeCont());
    //     NIVissimTL::dict_SetSignals(myNetBuilder.getTLLogicCont(), myNetBuilder.getEdgeCont());
    // }
    #[allow(unused_variables)]
    pub fn post_load_build(&mut self, offset: f64) {
        // close the loading process
        NIVissimBoundedClusterObject::close_loading();
        NIVissimConnection::dict_assign_to_edges();
        NIVissimDisturbance::dict_set_disturbances();
        // build district->connections map
        NIVissimDistrictConnection::dict_build_district_connections();
        // build clusters of connections with the same direction and a similar position along the streets
        NIVissimEdge::build_connection_clusters();
        // check whether further nodes (connection clusters by now) must be added
        NIVissimDistrictConnection::dict_check_edge_ends();
        // join clusters when overlapping (different streets are possible)
        NIVissimEdge::dict_check_edges2_join();
        NIVissimConnectionCluster::join_by_same_edges(offset);
        // build nodes from clusters
        NIVissimNodeCluster::set_current_virt_id(NIVissimNodeDef::get_max_id());
        NIVissimConnectionCluster::build_node_clusters();
        // the C++ passes several containers of the same builder into one call; they are disjoint fields
        let nbp = self.myNetBuilder.as_raw();
        macro_rules! nbc {
            ($f:ident) => {
                // SAFETY: the builder outlives the importer and the containers are disjoint fields
                unsafe { (*nbp).$f() }
            };
        }
        NIVissimNodeCluster::build_nb_nodes(nbc!(get_node_cont));
        NIVissimDistrictConnection::dict_build_district_nodes(nbc!(get_district_cont), nbc!(get_node_cont));
        NIVissimEdge::dict_propagate_speeds();
        NIVissimEdge::dict_build_nb_edges(nbc!(get_district_cont), nbc!(get_node_cont), nbc!(get_edge_cont), offset);
        if utils::options::options_cont::get_bool("vissim.report-unset-speeds") {
            NIVissimEdge::report_unset_speeds();
        }
        NIVissimDistrictConnection::dict_build_districts(nbc!(get_district_cont), nbc!(get_edge_cont), nbc!(get_node_cont));
        NIVissimConnection::dict_build_nb_edge_connections(nbc!(get_edge_cont));
        NIVissimNodeCluster::dict_add_disturbances(nbc!(get_district_cont), nbc!(get_node_cont), nbc!(get_edge_cont));
        NIVissimConflictArea::set_priority_regulation(nbc!(get_edge_cont));
        NIVissimTL::dict_set_signals(nbc!(get_tl_logic_cont), nbc!(get_edge_cont));
    }

    /// C++: `void NIImporter_Vissim::insertKnownElements()` [private]
    /// adds name-to-id - relationships of known elements into myKnownElements
    // void
    // NIImporter_Vissim::insertKnownElements() {
    //     myKnownElements["kennung"] = VE_Kennungszeile;
    //     myKnownElements["zufallszahl"] = VE_Startzufallszahl;
    //     myKnownElements["simulationsdauer"] = VE_Simdauer;
    //     myKnownElements["startuhrzeit"] = VE_Startuhrzeit;
    //     myKnownElements["simulationsrate"] = VE_SimRate;
    //     myKnownElements["zeitschritt"] = VE_Zeitschrittfaktor;
    //     myKnownElements["linksverkehr"] = VE_Linksverkehr;
    //     myKnownElements["dynuml"] = VE_DynUml;
    //     myKnownElements["stau"] = VE_Stauparameterdefinition;
    //     myKnownElements["gelbverhalten"] = VE_Gelbverhaltendefinition;
    //     myKnownElements["strecke"] = VE_Streckendefinition;
    //     myKnownElements["verbindung"] = VE_Verbindungsdefinition;
    //     myKnownElements["richtungsentscheidung"] = VE_Richtungsentscheidungsdefinition;
    //     myKnownElements["routenentscheidung"] = VE_Routenentscheidungsdefinition;
    //     myKnownElements["vwunschentscheidung"] = VE_VWunschentscheidungsdefinition;
    //     myKnownElements["langsamfahrbereich"] = VE_Langsamfahrbereichdefinition;
    //     myKnownElements["zufluss"] = VE_Zuflussdefinition;
    //     myKnownElements["fahrzeugtyp"] = VE_Fahrzeugtypdefinition;
    //     myKnownElements["fahrzeugklasse"] = VE_Fahrzeugklassendefinition;
    //     myKnownElements["zusammensetzung"] = VE_Verkehrszusammensetzungsdefinition;
    //     myKnownElements["vwunsch"] = VE_Geschwindigkeitsverteilungsdefinition;
    //     myKnownElements["laengen"] = VE_Laengenverteilungsdefinition;
    //     myKnownElements["zeiten"] = VE_Zeitenverteilungsdefinition;
    //     myKnownElements["baujahre"] = VE_Baujahrverteilungsdefinition;
    //     myKnownElements["leistungen"] = VE_Laufleistungsverteilungsdefinition;
    //     myKnownElements["massen"] = VE_Massenverteilungsdefinition;
    //     myKnownElements["leistungen"] = VE_Leistungsverteilungsdefinition;
    //     myKnownElements["maxbeschleunigung"] = VE_Maxbeschleunigungskurvedefinition;
    //     myKnownElements["wunschbeschleunigung"] = VE_Wunschbeschleunigungskurvedefinition;
    //     myKnownElements["maxverzoegerung"] = VE_Maxverzoegerungskurvedefinition;
    //     myKnownElements["wunschverzoegerung"] = VE_Wunschverzoegerungskurvedefinition;
    //     myKnownElements["querverkehrsstoerung"] = VE_Querverkehrsstoerungsdefinition;
    //     myKnownElements["lsa"] = VE_Lichtsignalanlagendefinition;
    //     myKnownElements["signalgruppe"] = VE_Signalgruppendefinition;
    //     myKnownElements["signalgeber"] = VE_Signalgeberdefinition;
    //     myKnownElements["lsakopplung"] = VE_LSAKopplungdefinition;
    //     myKnownElements["detektor"] = VE_Detektorendefinition;
    //     myKnownElements["haltestelle"] = VE_Haltestellendefinition;
    //     myKnownElements["linie"] = VE_Liniendefinition;
    //     myKnownElements["stopschild"] = VE_Stopschilddefinition;
    //     myKnownElements["messung"] = VE_Messungsdefinition;
    //     myKnownElements["reisezeit"] = VE_Reisezeitmessungsdefinition;
    //     myKnownElements["verlustzeit"] = VE_Verlustzeitmessungsdefinition;
    //     myKnownElements["querschnittsmessung"] = VE_Querschnittsmessungsdefinition;
    //     myKnownElements["stauzaehler"] = VE_Stauzaehlerdefinition;
    //     myKnownElements["auswertung"] = VE_Auswertungsdefinition;
    //     myKnownElements["fenster"] = VE_Fensterdefinition;
    //     myKnownElements["motiv"] = VE_Gefahrenwarnsystemdefinition;
    //     myKnownElements["parkplatz"] = VE_Parkplatzdefinition;
    //     myKnownElements["knoten"] = VE_Knotendefinition;
    //     myKnownElements["teapac"] = VE_TEAPACdefinition;
    //     myKnownElements["netzobjekt"] = VE_Netzobjektdefinition;
    //     myKnownElements["richtungspfeil"] = VE_Richtungspfeildefinition;
    //     myKnownElements["raute"] = VE_Rautedefinition;
    //     myKnownElements["fahrverhalten"] = VE_Fahrverhaltendefinition;
    //     myKnownElements["fahrtverlaufdateien"] = VE_Fahrtverlaufdateien;
    //     myKnownElements["emission"] = VE_Emission;
    //     myKnownElements["einheit"] = VE_Einheitendefinition;
    //     myKnownElements["streckentyp"] = VE_Streckentypdefinition;
    //     myKnownElements["kantensperrung"] = VE_Kantensperrung;
    //     myKnownElements["kante"] = VE_Kantensperrung;
    //
    //
    //     myKnownElements["advance"] = VE_DUMMY;
    //     myKnownElements["temperatur"] = VE_DUMMY;
    //
    // }
    #[allow(unused_variables)]
    pub fn insert_known_elements(&mut self) {
        let m = &mut self.myKnownElements;
        m.insert("kennung".to_string(), NIVissimElement::VE_Kennungszeile);
        m.insert("zufallszahl".to_string(), NIVissimElement::VE_Startzufallszahl);
        m.insert("simulationsdauer".to_string(), NIVissimElement::VE_Simdauer);
        m.insert("startuhrzeit".to_string(), NIVissimElement::VE_Startuhrzeit);
        m.insert("simulationsrate".to_string(), NIVissimElement::VE_SimRate);
        m.insert("zeitschritt".to_string(), NIVissimElement::VE_Zeitschrittfaktor);
        m.insert("linksverkehr".to_string(), NIVissimElement::VE_Linksverkehr);
        m.insert("dynuml".to_string(), NIVissimElement::VE_DynUml);
        m.insert("stau".to_string(), NIVissimElement::VE_Stauparameterdefinition);
        m.insert("gelbverhalten".to_string(), NIVissimElement::VE_Gelbverhaltendefinition);
        m.insert("strecke".to_string(), NIVissimElement::VE_Streckendefinition);
        m.insert("verbindung".to_string(), NIVissimElement::VE_Verbindungsdefinition);
        m.insert("richtungsentscheidung".to_string(), NIVissimElement::VE_Richtungsentscheidungsdefinition);
        m.insert("routenentscheidung".to_string(), NIVissimElement::VE_Routenentscheidungsdefinition);
        m.insert("vwunschentscheidung".to_string(), NIVissimElement::VE_VWunschentscheidungsdefinition);
        m.insert("langsamfahrbereich".to_string(), NIVissimElement::VE_Langsamfahrbereichdefinition);
        m.insert("zufluss".to_string(), NIVissimElement::VE_Zuflussdefinition);
        m.insert("fahrzeugtyp".to_string(), NIVissimElement::VE_Fahrzeugtypdefinition);
        m.insert("fahrzeugklasse".to_string(), NIVissimElement::VE_Fahrzeugklassendefinition);
        m.insert("zusammensetzung".to_string(), NIVissimElement::VE_Verkehrszusammensetzungsdefinition);
        m.insert("vwunsch".to_string(), NIVissimElement::VE_Geschwindigkeitsverteilungsdefinition);
        m.insert("laengen".to_string(), NIVissimElement::VE_Laengenverteilungsdefinition);
        m.insert("zeiten".to_string(), NIVissimElement::VE_Zeitenverteilungsdefinition);
        m.insert("baujahre".to_string(), NIVissimElement::VE_Baujahrverteilungsdefinition);
        m.insert("leistungen".to_string(), NIVissimElement::VE_Laufleistungsverteilungsdefinition);
        m.insert("massen".to_string(), NIVissimElement::VE_Massenverteilungsdefinition);
        m.insert("leistungen".to_string(), NIVissimElement::VE_Leistungsverteilungsdefinition);
        m.insert("maxbeschleunigung".to_string(), NIVissimElement::VE_Maxbeschleunigungskurvedefinition);
        m.insert("wunschbeschleunigung".to_string(), NIVissimElement::VE_Wunschbeschleunigungskurvedefinition);
        m.insert("maxverzoegerung".to_string(), NIVissimElement::VE_Maxverzoegerungskurvedefinition);
        m.insert("wunschverzoegerung".to_string(), NIVissimElement::VE_Wunschverzoegerungskurvedefinition);
        m.insert("querverkehrsstoerung".to_string(), NIVissimElement::VE_Querverkehrsstoerungsdefinition);
        m.insert("lsa".to_string(), NIVissimElement::VE_Lichtsignalanlagendefinition);
        m.insert("signalgruppe".to_string(), NIVissimElement::VE_Signalgruppendefinition);
        m.insert("signalgeber".to_string(), NIVissimElement::VE_Signalgeberdefinition);
        m.insert("lsakopplung".to_string(), NIVissimElement::VE_LSAKopplungdefinition);
        m.insert("detektor".to_string(), NIVissimElement::VE_Detektorendefinition);
        m.insert("haltestelle".to_string(), NIVissimElement::VE_Haltestellendefinition);
        m.insert("linie".to_string(), NIVissimElement::VE_Liniendefinition);
        m.insert("stopschild".to_string(), NIVissimElement::VE_Stopschilddefinition);
        m.insert("messung".to_string(), NIVissimElement::VE_Messungsdefinition);
        m.insert("reisezeit".to_string(), NIVissimElement::VE_Reisezeitmessungsdefinition);
        m.insert("verlustzeit".to_string(), NIVissimElement::VE_Verlustzeitmessungsdefinition);
        m.insert("querschnittsmessung".to_string(), NIVissimElement::VE_Querschnittsmessungsdefinition);
        m.insert("stauzaehler".to_string(), NIVissimElement::VE_Stauzaehlerdefinition);
        m.insert("auswertung".to_string(), NIVissimElement::VE_Auswertungsdefinition);
        m.insert("fenster".to_string(), NIVissimElement::VE_Fensterdefinition);
        m.insert("motiv".to_string(), NIVissimElement::VE_Gefahrenwarnsystemdefinition);
        m.insert("parkplatz".to_string(), NIVissimElement::VE_Parkplatzdefinition);
        m.insert("knoten".to_string(), NIVissimElement::VE_Knotendefinition);
        m.insert("teapac".to_string(), NIVissimElement::VE_TEAPACdefinition);
        m.insert("netzobjekt".to_string(), NIVissimElement::VE_Netzobjektdefinition);
        m.insert("richtungspfeil".to_string(), NIVissimElement::VE_Richtungspfeildefinition);
        m.insert("raute".to_string(), NIVissimElement::VE_Rautedefinition);
        m.insert("fahrverhalten".to_string(), NIVissimElement::VE_Fahrverhaltendefinition);
        m.insert("fahrtverlaufdateien".to_string(), NIVissimElement::VE_Fahrtverlaufdateien);
        m.insert("emission".to_string(), NIVissimElement::VE_Emission);
        m.insert("einheit".to_string(), NIVissimElement::VE_Einheitendefinition);
        m.insert("streckentyp".to_string(), NIVissimElement::VE_Streckentypdefinition);
        m.insert("kantensperrung".to_string(), NIVissimElement::VE_Kantensperrung);
        m.insert("kante".to_string(), NIVissimElement::VE_Kantensperrung);
        m.insert("advance".to_string(), NIVissimElement::VE_DUMMY);
        m.insert("temperatur".to_string(), NIVissimElement::VE_DUMMY);
    }

    /// C++: `void NIImporter_Vissim::buildParsers()` [private]
    /// adds id-to-parser - relationships of elements to parse into myParsers
    // void
    // NIImporter_Vissim::buildParsers() {
    //     myParsers[VE_Simdauer] =
    //         new NIVissimSingleTypeParser_Simdauer(*this);
    //     myParsers[VE_Startuhrzeit] =
    //         new NIVissimSingleTypeParser_Startuhrzeit(*this);
    //     myParsers[VE_DynUml] =
    //         new NIVissimSingleTypeParser_DynUml(*this);
    //     myParsers[VE_Streckendefinition] =
    //         new NIVissimSingleTypeParser_Streckendefinition(*this);
    //     myParsers[VE_Verbindungsdefinition] =
    //         new NIVissimSingleTypeParser_Verbindungsdefinition(*this);
    //     myParsers[VE_Richtungsentscheidungsdefinition] =
    //         new NIVissimSingleTypeParser_Richtungsentscheidungsdefinition(*this);
    //     myParsers[VE_Routenentscheidungsdefinition] =
    //         new NIVissimSingleTypeParser_Routenentscheidungsdefinition(*this);
    //     myParsers[VE_VWunschentscheidungsdefinition] =
    //         new NIVissimSingleTypeParser_VWunschentscheidungsdefinition(*this);
    //     myParsers[VE_Langsamfahrbereichdefinition] =
    //         new NIVissimSingleTypeParser_Langsamfahrbereichdefinition(*this);
    //     myParsers[VE_Zuflussdefinition] =
    //         new NIVissimSingleTypeParser_Zuflussdefinition(*this);
    //     myParsers[VE_Fahrzeugtypdefinition] =
    //         new NIVissimSingleTypeParser_Fahrzeugtypdefinition(*this, myColorMap);
    //     myParsers[VE_Fahrzeugklassendefinition] =
    //         new NIVissimSingleTypeParser_Fahrzeugklassendefinition(*this, myColorMap);
    //     myParsers[VE_Geschwindigkeitsverteilungsdefinition] =
    //         new NIVissimSingleTypeParser_Geschwindigkeitsverteilungsdefinition(*this);
    //     myParsers[VE_Laengenverteilungsdefinition] =
    //         new NIVissimSingleTypeParser_Laengenverteilungsdefinition(*this);
    //     myParsers[VE_Zeitenverteilungsdefinition] =
    //         new NIVissimSingleTypeParser_Zeitenverteilungsdefinition(*this);
    //     myParsers[VE_Querverkehrsstoerungsdefinition] =
    //         new NIVissimSingleTypeParser_Querverkehrsstoerungsdefinition(*this);
    //     myParsers[VE_Lichtsignalanlagendefinition] =
    //         new NIVissimSingleTypeParser_Lichtsignalanlagendefinition(*this);
    //     myParsers[VE_Signalgruppendefinition] =
    //         new NIVissimSingleTypeParser_Signalgruppendefinition(*this);
    //     myParsers[VE_Stopschilddefinition] =
    //         new NIVissimSingleTypeParser_Stopschilddefinition(*this);
    //     myParsers[VE_Knotendefinition] =
    //         new NIVissimSingleTypeParser_Knotendefinition(*this);
    //     myParsers[VE_Signalgeberdefinition] =
    //         new NIVissimSingleTypeParser_Signalgeberdefinition(*this);
    //     myParsers[VE_Detektorendefinition] =
    //         new NIVissimSingleTypeParser_Detektordefinition(*this);
    //     myParsers[VE_Haltestellendefinition] =
    //         new NIVissimSingleTypeParser_Haltestellendefinition(*this);
    //     myParsers[VE_Liniendefinition] =
    //         new NIVissimSingleTypeParser_Liniendefinition(*this);
    //     myParsers[VE_Reisezeitmessungsdefinition] =
    //         new NIVissimSingleTypeParser_Reisezeitmessungsdefinition(*this);
    //     myParsers[VE_Querschnittsmessungsdefinition] =
    //         new NIVissimSingleTypeParser_Querschnittsmessungsdefinition(*this);
    //     myParsers[VE_Messungsdefinition] =
    //         new NIVissimSingleTypeParser_Messungsdefinition(*this);
    //     myParsers[VE_Verlustzeitmessungsdefinition] =
    //         new NIVissimSingleTypeParser_Verlustzeitmessungsdefinition(*this);
    //     myParsers[VE_Stauzaehlerdefinition] =
    //         new NIVissimSingleTypeParser_Stauzaehlerdefinition(*this);
    //     myParsers[VE_Rautedefinition] =
    //         new NIVissimSingleTypeParser_Rautedefinition(*this);
    //     myParsers[VE_Richtungspfeildefinition] =
    //         new NIVissimSingleTypeParser_Richtungspfeildefinition(*this);
    //     myParsers[VE_Parkplatzdefinition] =
    //         new NIVissimSingleTypeParser_Parkplatzdefinition(*this);
    //     myParsers[VE_Fahrverhaltendefinition] =
    //         new NIVissimSingleTypeParser_Fahrverhaltendefinition(*this);
    //     myParsers[VE_Streckentypdefinition] =
    //         new NIVissimSingleTypeParser_Streckentypdefinition(*this);
    //     myParsers[VE_Kennungszeile] =
    //         new NIVissimSingleTypeParser_Kennungszeile(*this);
    //     myParsers[VE_Fensterdefinition] =
    //         new NIVissimSingleTypeParser_Fensterdefinition(*this);
    //     myParsers[VE_Auswertungsdefinition] =
    //         new NIVissimSingleTypeParser_Auswertungsdefinition(*this);
    //     myParsers[VE_Verkehrszusammensetzungsdefinition] =
    //         new NIVissimSingleTypeParser_Zusammensetzungsdefinition(*this);
    //     myParsers[VE_Kantensperrung] =
    //         new NIVissimSingleTypeParser_Kantensperrung(*this);
    //
    //     myParsers[VE_Startzufallszahl] =
    //         new NIVissimSingleTypeParser_Startzufallszahl(*this);
    //     myParsers[VE_SimRate] =
    //         new NIVissimSingleTypeParser_SimRate(*this);
    //     myParsers[VE_Zeitschrittfaktor] =
    //         new NIVissimSingleTypeParser_Zeitschrittfaktor(*this);
    //     myParsers[VE_Linksverkehr] =
    //         new NIVissimSingleTypeParser_Linksverkehr(*this);
    //     myParsers[VE_Stauparameterdefinition] =
    //         new NIVissimSingleTypeParser_Stauparameterdefinition(*this);
    //     myParsers[VE_Gelbverhaltendefinition] =
    //         new NIVissimSingleTypeParser_Gelbverhaltendefinition(*this);
    //     myParsers[VE_LSAKopplungdefinition] =
    //         new NIVissimSingleTypeParser_LSAKopplungsdefinition(*this);
    //     myParsers[VE_Gefahrenwarnsystemdefinition] =
    //         new NIVissimSingleTypeParser_Gefahrwarnungsdefinition(*this);
    //     myParsers[VE_TEAPACdefinition] =
    //         new NIVissimSingleTypeParser_TEAPACDefinition(*this);
    //     myParsers[VE_Netzobjektdefinition] =
    //         new NIVissimSingleTypeParser_Netzobjektdefinition(*this);
    //     myParsers[VE_Fahrtverlaufdateien] =
    //         new NIVissimSingleTypeParser_Fahrtverlaufdateien(*this);
    //     myParsers[VE_Emission] =
    //         new NIVissimSingleTypeParser_Emission(*this);
    //     myParsers[VE_Einheitendefinition] =
    //         new NIVissimSingleTypeParser_Einheitendefinition(*this);
    //     myParsers[VE_Baujahrverteilungsdefinition] =
    //         new NIVissimSingleTypeParser__XVerteilungsdefinition(*this);
    //     myParsers[VE_Laufleistungsverteilungsdefinition] =
    //         new NIVissimSingleTypeParser__XVerteilungsdefinition(*this);
    //     myParsers[VE_Massenverteilungsdefinition] =
    //         new NIVissimSingleTypeParser__XVerteilungsdefinition(*this);
    //     myParsers[VE_Leistungsverteilungsdefinition] =
    //         new NIVissimSingleTypeParser__XVerteilungsdefinition(*this);
    //     myParsers[VE_Maxbeschleunigungskurvedefinition] =
    //         new NIVissimSingleTypeParser__XKurvedefinition(*this);
    //     myParsers[VE_Wunschbeschleunigungskurvedefinition] =
    //         new NIVissimSingleTypeParser__XKurvedefinition(*this);
    //     myParsers[VE_Maxverzoegerungskurvedefinition] =
    //         new NIVissimSingleTypeParser__XKurvedefinition(*this);
    //     myParsers[VE_Wunschverzoegerungskurvedefinition] =
    //         new NIVissimSingleTypeParser__XKurvedefinition(*this);
    //
    // }
    #[allow(unused_variables)]
    pub fn build_parsers(&mut self) {
        // shortcut: parsers are leaked for the importer's lifetime (C++ deletes them in the destructor)
        fn leak_parser<T>(p: T) -> Ptr<NIImporter_Vissim_VissimSingleTypeParser> {
            Ptr::from_mut(Box::leak(Box::new(p))).cast()
        }
        // the parsers keep a reference to myColorMap, which lives inside *this, exactly as the C++ reference does
        let colorMap: *mut NIImporter_Vissim_ColorMap = &mut self.myColorMap;
        // SAFETY: the parser only dereferences the color map through its Ptr while the importer is alive
        let p = leak_parser(NIVissimSingleTypeParser_Simdauer::new(self));
        self.myParsers.insert(NIVissimElement::VE_Simdauer, p);
        let p = leak_parser(NIVissimSingleTypeParser_Startuhrzeit::new(self));
        self.myParsers.insert(NIVissimElement::VE_Startuhrzeit, p);
        let p = leak_parser(NIVissimSingleTypeParser_DynUml::new(self));
        self.myParsers.insert(NIVissimElement::VE_DynUml, p);
        let p = leak_parser(NIVissimSingleTypeParser_Streckendefinition::new(self));
        self.myParsers.insert(NIVissimElement::VE_Streckendefinition, p);
        let p = leak_parser(NIVissimSingleTypeParser_Verbindungsdefinition::new(self));
        self.myParsers.insert(NIVissimElement::VE_Verbindungsdefinition, p);
        let p = leak_parser(NIVissimSingleTypeParser_Richtungsentscheidungsdefinition::new(self));
        self.myParsers.insert(NIVissimElement::VE_Richtungsentscheidungsdefinition, p);
        let p = leak_parser(NIVissimSingleTypeParser_Routenentscheidungsdefinition::new(self));
        self.myParsers.insert(NIVissimElement::VE_Routenentscheidungsdefinition, p);
        let p = leak_parser(NIVissimSingleTypeParser_VWunschentscheidungsdefinition::new(self));
        self.myParsers.insert(NIVissimElement::VE_VWunschentscheidungsdefinition, p);
        let p = leak_parser(NIVissimSingleTypeParser_Langsamfahrbereichdefinition::new(self));
        self.myParsers.insert(NIVissimElement::VE_Langsamfahrbereichdefinition, p);
        let p = leak_parser(NIVissimSingleTypeParser_Zuflussdefinition::new(self));
        self.myParsers.insert(NIVissimElement::VE_Zuflussdefinition, p);
        let p = leak_parser(NIVissimSingleTypeParser_Fahrzeugtypdefinition::new(self, unsafe { &mut *colorMap }));
        self.myParsers.insert(NIVissimElement::VE_Fahrzeugtypdefinition, p);
        let p = leak_parser(NIVissimSingleTypeParser_Fahrzeugklassendefinition::new(self, unsafe { &mut *colorMap }));
        self.myParsers.insert(NIVissimElement::VE_Fahrzeugklassendefinition, p);
        let p = leak_parser(NIVissimSingleTypeParser_Geschwindigkeitsverteilungsdefinition::new(self));
        self.myParsers.insert(NIVissimElement::VE_Geschwindigkeitsverteilungsdefinition, p);
        let p = leak_parser(NIVissimSingleTypeParser_Laengenverteilungsdefinition::new(self));
        self.myParsers.insert(NIVissimElement::VE_Laengenverteilungsdefinition, p);
        let p = leak_parser(NIVissimSingleTypeParser_Zeitenverteilungsdefinition::new(self));
        self.myParsers.insert(NIVissimElement::VE_Zeitenverteilungsdefinition, p);
        let p = leak_parser(NIVissimSingleTypeParser_Querverkehrsstoerungsdefinition::new(self));
        self.myParsers.insert(NIVissimElement::VE_Querverkehrsstoerungsdefinition, p);
        let p = leak_parser(NIVissimSingleTypeParser_Lichtsignalanlagendefinition::new(self));
        self.myParsers.insert(NIVissimElement::VE_Lichtsignalanlagendefinition, p);
        let p = leak_parser(NIVissimSingleTypeParser_Signalgruppendefinition::new(self));
        self.myParsers.insert(NIVissimElement::VE_Signalgruppendefinition, p);
        let p = leak_parser(NIVissimSingleTypeParser_Stopschilddefinition::new(self));
        self.myParsers.insert(NIVissimElement::VE_Stopschilddefinition, p);
        let p = leak_parser(NIVissimSingleTypeParser_Knotendefinition::new(self));
        self.myParsers.insert(NIVissimElement::VE_Knotendefinition, p);
        let p = leak_parser(NIVissimSingleTypeParser_Signalgeberdefinition::new(self));
        self.myParsers.insert(NIVissimElement::VE_Signalgeberdefinition, p);
        let p = leak_parser(NIVissimSingleTypeParser_Detektordefinition::new(self));
        self.myParsers.insert(NIVissimElement::VE_Detektorendefinition, p);
        let p = leak_parser(NIVissimSingleTypeParser_Haltestellendefinition::new(self));
        self.myParsers.insert(NIVissimElement::VE_Haltestellendefinition, p);
        let p = leak_parser(NIVissimSingleTypeParser_Liniendefinition::new(self));
        self.myParsers.insert(NIVissimElement::VE_Liniendefinition, p);
        let p = leak_parser(NIVissimSingleTypeParser_Reisezeitmessungsdefinition::new(self));
        self.myParsers.insert(NIVissimElement::VE_Reisezeitmessungsdefinition, p);
        let p = leak_parser(NIVissimSingleTypeParser_Querschnittsmessungsdefinition::new(self));
        self.myParsers.insert(NIVissimElement::VE_Querschnittsmessungsdefinition, p);
        let p = leak_parser(NIVissimSingleTypeParser_Messungsdefinition::new(self));
        self.myParsers.insert(NIVissimElement::VE_Messungsdefinition, p);
        let p = leak_parser(NIVissimSingleTypeParser_Verlustzeitmessungsdefinition::new(self));
        self.myParsers.insert(NIVissimElement::VE_Verlustzeitmessungsdefinition, p);
        let p = leak_parser(NIVissimSingleTypeParser_Stauzaehlerdefinition::new(self));
        self.myParsers.insert(NIVissimElement::VE_Stauzaehlerdefinition, p);
        let p = leak_parser(NIVissimSingleTypeParser_Rautedefinition::new(self));
        self.myParsers.insert(NIVissimElement::VE_Rautedefinition, p);
        let p = leak_parser(NIVissimSingleTypeParser_Richtungspfeildefinition::new(self));
        self.myParsers.insert(NIVissimElement::VE_Richtungspfeildefinition, p);
        let p = leak_parser(NIVissimSingleTypeParser_Parkplatzdefinition::new(self));
        self.myParsers.insert(NIVissimElement::VE_Parkplatzdefinition, p);
        let p = leak_parser(NIVissimSingleTypeParser_Fahrverhaltendefinition::new(self));
        self.myParsers.insert(NIVissimElement::VE_Fahrverhaltendefinition, p);
        let p = leak_parser(NIVissimSingleTypeParser_Streckentypdefinition::new(self));
        self.myParsers.insert(NIVissimElement::VE_Streckentypdefinition, p);
        let p = leak_parser(NIVissimSingleTypeParser_Kennungszeile::new(self));
        self.myParsers.insert(NIVissimElement::VE_Kennungszeile, p);
        let p = leak_parser(NIVissimSingleTypeParser_Fensterdefinition::new(self));
        self.myParsers.insert(NIVissimElement::VE_Fensterdefinition, p);
        let p = leak_parser(NIVissimSingleTypeParser_Auswertungsdefinition::new(self));
        self.myParsers.insert(NIVissimElement::VE_Auswertungsdefinition, p);
        let p = leak_parser(NIVissimSingleTypeParser_Zusammensetzungsdefinition::new(self));
        self.myParsers.insert(NIVissimElement::VE_Verkehrszusammensetzungsdefinition, p);
        let p = leak_parser(NIVissimSingleTypeParser_Kantensperrung::new(self));
        self.myParsers.insert(NIVissimElement::VE_Kantensperrung, p);
        let p = leak_parser(NIVissimSingleTypeParser_Startzufallszahl::new(self));
        self.myParsers.insert(NIVissimElement::VE_Startzufallszahl, p);
    }

    /// C++: `void NIImporter_Vissim::NIImporter_Vissim(const NIImporter_Vissim &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &NIImporter_Vissim) -> Self {
        panic!("NIImporter_Vissim copy constructor is private and invalidated in C++")
    }

    /// C++: `NIImporter_Vissim & NIImporter_Vissim::operator=(const NIImporter_Vissim &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &NIImporter_Vissim) -> &mut NIImporter_Vissim {
        panic!("NIImporter_Vissim assignment operator is private and invalidated in C++")
    }

}

/// A class which extracts VISSIM-ConflictAreas from a parsed VISSIM-file
/// C++ class `NIImporter_Vissim::NIVissimXMLHandler_ConflictArea` (netimport/vissim/NIImporter_Vissim.h)
#[repr(C)]
pub struct NIImporter_Vissim_NIVissimXMLHandler_ConflictArea {
    /// C++ base class `GenericSAXHandler`
    pub base: GenericSAXHandler,
}

impl std::ops::Deref for NIImporter_Vissim_NIVissimXMLHandler_ConflictArea {
    type Target = GenericSAXHandler;
    fn deref(&self) -> &GenericSAXHandler {
        &self.base
    }
}
impl std::ops::DerefMut for NIImporter_Vissim_NIVissimXMLHandler_ConflictArea {
    fn deref_mut(&mut self) -> &mut GenericSAXHandler {
        &mut self.base
    }
}

impl NIImporter_Vissim_NIVissimXMLHandler_ConflictArea {
    /// C++: `void NIVissimXMLHandler_ConflictArea::NIVissimXMLHandler_ConflictArea()`
    /// Constructor
    // NIImporter_Vissim::NIVissimXMLHandler_ConflictArea::NIVissimXMLHandler_ConflictArea()
    //     : GenericSAXHandler(vissimTags, VISSIM_TAG_NOTHING,
    //                         vissimAttrs, VISSIM_ATTR_NOTHING,
    //                         "vissim - file") {}
    #[allow(unused_variables)]
    pub fn new() -> Self {
        Self { base: vissim_base_handler() }
    }

    /// C++: `void NIVissimXMLHandler_ConflictArea::~NIVissimXMLHandler_ConflictArea() noexcept` [virtual]
    /// Destructor
    // NIImporter_Vissim::NIVissimXMLHandler_ConflictArea::~NIVissimXMLHandler_ConflictArea() { }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
    }

    /// C++: `void NIVissimXMLHandler_ConflictArea::myStartElement(int, const SUMOSAXAttributes &)` [virtual, protected]
    /// Called on the opening of a tag;
    // void
    // NIImporter_Vissim::NIVissimXMLHandler_ConflictArea::myStartElement(int element, const SUMOSAXAttributes& attrs) {
    //     // finding an actual flow
    //     if (element == VISSIM_TAG_CA) {
    //         //parse all flows
    //         bool ok = true;
    //         std::string status = attrs.get<std::string>(VISSIM_ATTR_STATUS, nullptr, ok);
    //         //get only the conflict areas which were set in VISSIM
    //         if (status != "PASSIVE") {
    //             NIVissimConflictArea::dictionary(attrs.get<int>(VISSIM_ATTR_NO, nullptr, ok),
    //                                              attrs.get<std::string>(VISSIM_ATTR_LINK1, nullptr, ok),
    //                                              attrs.get<std::string>(VISSIM_ATTR_LINK2, nullptr, ok),
    //                                              status);
    //         }
    //
    //     }
    // }
    #[allow(unused_variables)]
    pub fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) {
        // finding an actual flow
        if element == NIImporter_Vissim_VissimXMLTag::VISSIM_TAG_CA as i32 {
            //parse all flows
            let mut ok = true;
            let status: String = attrs.get(NIImporter_Vissim_VissimXMLAttr::VISSIM_ATTR_STATUS as i32, "", &mut ok, true);
            //get only the conflict areas which were set in VISSIM
            if status != "PASSIVE" {
                let no: i32 = attrs.get(NIImporter_Vissim_VissimXMLAttr::VISSIM_ATTR_NO as i32, "", &mut ok, true);
                let link1: String = attrs.get(NIImporter_Vissim_VissimXMLAttr::VISSIM_ATTR_LINK1 as i32, "", &mut ok, true);
                let link2: String = attrs.get(NIImporter_Vissim_VissimXMLAttr::VISSIM_ATTR_LINK2 as i32, "", &mut ok, true);
                NIVissimConflictArea::dictionary(no, &link1, &link2, &status);
            }
        }
    }

    /// C++: `void NIVissimXMLHandler_ConflictArea::NIVissimXMLHandler_ConflictArea(const NIVissimXMLHandler_ConflictArea &)` [private]
    /// invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(c: &NIImporter_Vissim_NIVissimXMLHandler_ConflictArea) -> Self {
        panic!("NIVissimXMLHandler_ConflictArea copy constructor is private and invalidated in C++")
    }

    /// C++: `NIVissimXMLHandler_ConflictArea & NIVissimXMLHandler_ConflictArea::operator=(const NIVissimXMLHandler_ConflictArea &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, c: &NIImporter_Vissim_NIVissimXMLHandler_ConflictArea) -> &mut NIImporter_Vissim_NIVissimXMLHandler_ConflictArea {
        panic!("NIVissimXMLHandler_ConflictArea assignment operator is private and invalidated in C++")
    }

}

/// A class which extracts VISSIM-Fahrzeugklassen from a parsed VISSIM-file
/// C++ class `NIImporter_Vissim::NIVissimXMLHandler_Fahrzeugklassendefinition` (netimport/vissim/NIImporter_Vissim.h)
#[repr(C)]
pub struct NIImporter_Vissim_NIVissimXMLHandler_Fahrzeugklassendefinition {
    /// C++ base class `GenericSAXHandler`
    pub base: GenericSAXHandler,
    pub myElemData: Ptr<NIImporter_Vissim_nodeMap>,
    /// The current hierarchy level
    pub myHierarchyLevel: i32,
    /// ID of the currently parsed node, for reporting mainly
    pub myLastNodeID: i32,
}

impl std::ops::Deref for NIImporter_Vissim_NIVissimXMLHandler_Fahrzeugklassendefinition {
    type Target = GenericSAXHandler;
    fn deref(&self) -> &GenericSAXHandler {
        &self.base
    }
}
impl std::ops::DerefMut for NIImporter_Vissim_NIVissimXMLHandler_Fahrzeugklassendefinition {
    fn deref_mut(&mut self) -> &mut GenericSAXHandler {
        &mut self.base
    }
}

impl NIImporter_Vissim_NIVissimXMLHandler_Fahrzeugklassendefinition {
    /// C++: `void NIVissimXMLHandler_Fahrzeugklassendefinition::NIVissimXMLHandler_Fahrzeugklassendefinition(nodeMap &)`
    /// Constructor
    // NIImporter_Vissim::NIVissimXMLHandler_Fahrzeugklassendefinition::NIVissimXMLHandler_Fahrzeugklassendefinition(nodeMap& elemData)
    //     : GenericSAXHandler(vissimTags, VISSIM_TAG_NOTHING,
    //                         vissimAttrs, VISSIM_ATTR_NOTHING,
    //                         "vissim - file"),
    //       myElemData(elemData),
    //       myHierarchyLevel(0) {
    //     myElemData.clear();
    // }
    #[allow(unused_variables)]
    pub fn new(elemData: &mut NIImporter_Vissim_nodeMap) -> Self {
        elemData.clear();
        Self { base: vissim_base_handler(), myElemData: Ptr::from_mut(elemData), myHierarchyLevel: 0, myLastNodeID: 0 }
    }

    /// C++: `void NIVissimXMLHandler_Fahrzeugklassendefinition::~NIVissimXMLHandler_Fahrzeugklassendefinition() noexcept` [virtual]
    /// Destructor
    // NIImporter_Vissim::NIVissimXMLHandler_Fahrzeugklassendefinition::~NIVissimXMLHandler_Fahrzeugklassendefinition() { }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
    }

    /// C++: `void NIVissimXMLHandler_Fahrzeugklassendefinition::myStartElement(int, const SUMOSAXAttributes &)` [virtual, protected]
    /// Called on the opening of a tag;
    // void
    // NIImporter_Vissim::NIVissimXMLHandler_Fahrzeugklassendefinition::myStartElement(int element, const SUMOSAXAttributes& attrs) {
    //     myHierarchyLevel++;
    //
    //     if (element == VISSIM_TAG_VEHICLE_CLASS) {
    //         bool ok = true;
    //         myElemData["id"].push_back(attrs.get<std::string>(VISSIM_ATTR_NO, nullptr, ok));
    //         myElemData["name"].push_back(attrs.get<std::string>(VISSIM_ATTR_NAME, nullptr, ok, false));
    //         std::string colorStr(attrs.get<std::string>(VISSIM_ATTR_COLOR, nullptr, ok));
    //         for (int pos = (int)colorStr.size() - 2; pos > 0; pos -= 2) {
    //             colorStr.insert(pos, " ");
    //         }
    //         myElemData["color"].push_back(colorStr);
    //     }
    //     if (element == VISSIM_TAG_INTOBJECTREF) {
    //         bool ok = true;
    //         myElemData["types"].push_back(attrs.get<std::string>(VISSIM_ATTR_KEY, nullptr, ok));
    //
    //
    //     }
    // }
    #[allow(unused_variables)]
    pub fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) {
        self.myHierarchyLevel += 1;

        if element == NIImporter_Vissim_VissimXMLTag::VISSIM_TAG_VEHICLE_CLASS as i32 {
            let mut ok = true;
            let id: String = attrs.get(NIImporter_Vissim_VissimXMLAttr::VISSIM_ATTR_NO as i32, "", &mut ok, true);
            self.myElemData.m().entry("id".to_string()).or_default().push_back(id);
            let name: String = attrs.get(NIImporter_Vissim_VissimXMLAttr::VISSIM_ATTR_NAME as i32, "", &mut ok, false);
            self.myElemData.m().entry("name".to_string()).or_default().push_back(name);
            let mut colorStr: String = attrs.get(NIImporter_Vissim_VissimXMLAttr::VISSIM_ATTR_COLOR as i32, "", &mut ok, true);
            let mut pos = colorStr.len() as i64 - 2;
            while pos > 0 {
                colorStr.insert_str(pos as usize, " ");
                pos -= 2;
            }
            self.myElemData.m().entry("color".to_string()).or_default().push_back(colorStr);
        }
        if element == NIImporter_Vissim_VissimXMLTag::VISSIM_TAG_INTOBJECTREF as i32 {
            let mut ok = true;
            let key: String = attrs.get(NIImporter_Vissim_VissimXMLAttr::VISSIM_ATTR_KEY as i32, "", &mut ok, true);
            self.myElemData.m().entry("types".to_string()).or_default().push_back(key);
        }
    }

    /// C++: `void NIVissimXMLHandler_Fahrzeugklassendefinition::myEndElement(int)` [virtual, protected]
    // void
    // NIImporter_Vissim::NIVissimXMLHandler_Fahrzeugklassendefinition::myEndElement(int element) {
    //     if (element == VISSIM_TAG_VEHICLE_CLASS && myHierarchyLevel == 3) {
    //         RGBColor color;
    //         std::istringstream iss(myElemData["color"].front());
    //         std::vector<std::string> sCol_v(StringTokenizer(
    //                                             myElemData["color"].front(), " ").getVector());
    //         std::vector<int> myColorVector(sCol_v.size());
    //         std::transform(sCol_v.begin(), sCol_v.end(), myColorVector.begin(), StringUtils::hexToInt);
    //
    //         color = RGBColor((unsigned char)myColorVector[0],
    //                          (unsigned char)myColorVector[1],
    //                          (unsigned char)myColorVector[2],
    //                          (unsigned char)myColorVector[3]);
    //         std::vector<int> types;
    //         while (!myElemData["types"].empty()) {
    //             types.push_back(StringUtils::toInt(myElemData["types"].front()));
    //             myElemData["types"].pop_front();
    //         }
    //
    //         NIVissimVehTypeClass::dictionary(StringUtils::toInt(myElemData["id"].front()),
    //                                          myElemData["name"].front(),
    //                                          color,
    //                                          types);
    //         myElemData.clear();
    //     }
    //     --myHierarchyLevel;
    // }
    #[allow(unused_variables)]
    pub fn my_end_element(&mut self, element: i32) {
        if element == NIImporter_Vissim_VissimXMLTag::VISSIM_TAG_VEHICLE_CLASS as i32 && self.myHierarchyLevel == 3 {
            let colorFront: String = self.myElemData.m().entry("color".to_string()).or_default().front().cloned().unwrap_or_default();
            let sCol_v: Vec<String> = StringTokenizer::with_token(&colorFront, " ", false).get_vector();
            let myColorVector: Vec<i32> = sCol_v.iter().map(|s| hex_to_int(s).unwrap_or_else(|e| panic!("{e}"))).collect();

            let color = RGBColor::new(myColorVector[0] as u8, myColorVector[1] as u8, myColorVector[2] as u8, myColorVector[3] as u8);
            let mut types: Vec<i32> = Vec::new();
            loop {
                let front = match self.myElemData.m().entry("types".to_string()).or_default().pop_front() {
                    Some(f) => f,
                    None => break,
                };
                types.push(to_int(&front).unwrap_or_else(|e| panic!("{e}")));
            }

            let idFront: String = self.myElemData.m().entry("id".to_string()).or_default().front().cloned().unwrap_or_default();
            let nameFront: String = self.myElemData.m().entry("name".to_string()).or_default().front().cloned().unwrap_or_default();
            NIVissimVehTypeClass::dictionary(to_int(&idFront).unwrap_or_else(|e| panic!("{e}")), &nameFront, &color, &mut types);
            self.myElemData.m().clear();
        }
        self.myHierarchyLevel -= 1;
    }

    /// C++: `void NIVissimXMLHandler_Fahrzeugklassendefinition::NIVissimXMLHandler_Fahrzeugklassendefinition(const NIVissimXMLHandler_Fahrzeugklassendefinition &)` [private]
    /// invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(f: &NIImporter_Vissim_NIVissimXMLHandler_Fahrzeugklassendefinition) -> Self {
        panic!("NIVissimXMLHandler_Fahrzeugklassendefinition copy constructor is private and invalidated in C++")
    }

    /// C++: `NIVissimXMLHandler_Fahrzeugklassendefinition & NIVissimXMLHandler_Fahrzeugklassendefinition::operator=(const NIVissimXMLHandler_Fahrzeugklassendefinition &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, f: &NIImporter_Vissim_NIVissimXMLHandler_Fahrzeugklassendefinition) -> &mut NIImporter_Vissim_NIVissimXMLHandler_Fahrzeugklassendefinition {
        panic!("NIVissimXMLHandler_Fahrzeugklassendefinition assignment operator is private and invalidated in C++")
    }

}

/// A class which extracts VISSIM-Geschwindigkeitsverteilung from a parsed VISSIM-file
/// C++ class `NIImporter_Vissim::NIVissimXMLHandler_Geschwindigkeitsverteilungsdefinition` (netimport/vissim/NIImporter_Vissim.h)
#[repr(C)]
pub struct NIImporter_Vissim_NIVissimXMLHandler_Geschwindigkeitsverteilungsdefinition {
    /// C++ base class `GenericSAXHandler`
    pub base: GenericSAXHandler,
    pub myElemData: Ptr<NIImporter_Vissim_nodeMap>,
    /// The current hierarchy level
    pub myHierarchyLevel: i32,
    /// ID of the currently parsed node, for reporting mainly
    pub myLastNodeID: i32,
}

impl std::ops::Deref for NIImporter_Vissim_NIVissimXMLHandler_Geschwindigkeitsverteilungsdefinition {
    type Target = GenericSAXHandler;
    fn deref(&self) -> &GenericSAXHandler {
        &self.base
    }
}
impl std::ops::DerefMut for NIImporter_Vissim_NIVissimXMLHandler_Geschwindigkeitsverteilungsdefinition {
    fn deref_mut(&mut self) -> &mut GenericSAXHandler {
        &mut self.base
    }
}

impl NIImporter_Vissim_NIVissimXMLHandler_Geschwindigkeitsverteilungsdefinition {
    /// C++: `void NIVissimXMLHandler_Geschwindigkeitsverteilungsdefinition::NIVissimXMLHandler_Geschwindigkeitsverteilungsdefinition(nodeMap &)`
    /// Constructor
    // NIImporter_Vissim::NIVissimXMLHandler_Geschwindigkeitsverteilungsdefinition::NIVissimXMLHandler_Geschwindigkeitsverteilungsdefinition(nodeMap& elemData)
    //     : GenericSAXHandler(vissimTags, VISSIM_TAG_NOTHING,
    //                         vissimAttrs, VISSIM_ATTR_NOTHING,
    //                         "vissim - file"),
    //       myElemData(elemData),
    //       myHierarchyLevel(0) {
    //     myElemData.clear();
    // }
    #[allow(unused_variables)]
    pub fn new(elemData: &mut NIImporter_Vissim_nodeMap) -> Self {
        elemData.clear();
        Self { base: vissim_base_handler(), myElemData: Ptr::from_mut(elemData), myHierarchyLevel: 0, myLastNodeID: 0 }
    }

    /// C++: `void NIVissimXMLHandler_Geschwindigkeitsverteilungsdefinition::~NIVissimXMLHandler_Geschwindigkeitsverteilungsdefinition() noexcept` [virtual]
    /// Destructor
    // NIImporter_Vissim::NIVissimXMLHandler_Geschwindigkeitsverteilungsdefinition::~NIVissimXMLHandler_Geschwindigkeitsverteilungsdefinition() { }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
    }

    /// C++: `void NIVissimXMLHandler_Geschwindigkeitsverteilungsdefinition::myStartElement(int, const SUMOSAXAttributes &)` [virtual, protected]
    /// Called on the opening of a tag;
    // void
    // NIImporter_Vissim::NIVissimXMLHandler_Geschwindigkeitsverteilungsdefinition::myStartElement(int element, const SUMOSAXAttributes& attrs) {
    //     myHierarchyLevel++;
    //     if (element == VISSIM_TAG_SPEED_DIST) {
    //         bool ok = true;
    //         myElemData["id"].push_back(attrs.get<std::string>(VISSIM_ATTR_NO, nullptr, ok));
    //     }
    //
    //     if (element == VISSIM_TAG_DATAPOINT) {
    //         bool ok = true;
    //         std::string sep(" ");
    //         std::string posS(attrs.get<std::string>(VISSIM_ATTR_X, nullptr, ok));
    //         posS += sep;
    //         posS.append(attrs.get<std::string>(VISSIM_ATTR_FX, nullptr, ok));
    //         myElemData["points"].push_back(posS);
    //
    //     }
    //
    // }
    #[allow(unused_variables)]
    pub fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) {
        self.myHierarchyLevel += 1;
        if element == NIImporter_Vissim_VissimXMLTag::VISSIM_TAG_SPEED_DIST as i32 {
            let mut ok = true;
            let id: String = attrs.get(NIImporter_Vissim_VissimXMLAttr::VISSIM_ATTR_NO as i32, "", &mut ok, true);
            self.myElemData.m().entry("id".to_string()).or_default().push_back(id);
        }

        if element == NIImporter_Vissim_VissimXMLTag::VISSIM_TAG_DATAPOINT as i32 {
            let mut ok = true;
            let sep = " ";
            let mut posS: String = attrs.get(NIImporter_Vissim_VissimXMLAttr::VISSIM_ATTR_X as i32, "", &mut ok, true);
            posS += sep;
            let fx: String = attrs.get(NIImporter_Vissim_VissimXMLAttr::VISSIM_ATTR_FX as i32, "", &mut ok, true);
            posS.push_str(&fx);
            self.myElemData.m().entry("points".to_string()).or_default().push_back(posS);
        }
    }

    /// C++: `void NIVissimXMLHandler_Geschwindigkeitsverteilungsdefinition::myEndElement(int)` [virtual, protected]
    // void
    // NIImporter_Vissim::NIVissimXMLHandler_Geschwindigkeitsverteilungsdefinition::myEndElement(int element) {
    //     if (element == VISSIM_TAG_SPEED_DIST && myHierarchyLevel == 3) {
    //         Distribution_Points* points = new Distribution_Points(myElemData["id"].front());
    //         while (!myElemData["points"].empty()) {
    //             std::vector<std::string> sPos_v(StringTokenizer(
    //                                                 myElemData["points"].front(), " ").getVector());
    //             myElemData["points"].pop_front();
    //             points->add(StringUtils::toDouble(sPos_v[0]), StringUtils::toDouble(sPos_v[1]));
    //         }
    //         DistributionCont::dictionary("speed", myElemData["id"].front(), points);
    //         myElemData.clear();
    //     }
    //     --myHierarchyLevel;
    // }
    #[allow(unused_variables)]
    pub fn my_end_element(&mut self, element: i32) {
        if element == NIImporter_Vissim_VissimXMLTag::VISSIM_TAG_SPEED_DIST as i32 && self.myHierarchyLevel == 3 {
            let idFront: String = self.myElemData.m().entry("id".to_string()).or_default().front().cloned().unwrap_or_default();
            let mut points = DistributionPoints::new(&idFront);
            loop {
                let front = match self.myElemData.m().entry("points".to_string()).or_default().pop_front() {
                    Some(f) => f,
                    None => break,
                };
                let sPos_v: Vec<String> = StringTokenizer::with_token(&front, " ", false).get_vector();
                let x = to_double(&sPos_v[0]).unwrap_or_else(|e| panic!("{e}"));
                let y = to_double(&sPos_v[1]).unwrap_or_else(|e| panic!("{e}"));
                points.add(x, y, true);
            }
            let idFront: String = self.myElemData.m().entry("id".to_string()).or_default().front().cloned().unwrap_or_default();
            utils::distribution::distribution_cont::dictionary_add("speed", &idFront, std::sync::Arc::new(points));
            self.myElemData.m().clear();
        }
        self.myHierarchyLevel -= 1;
    }

    /// C++: `void NIVissimXMLHandler_Geschwindigkeitsverteilungsdefinition::NIVissimXMLHandler_Geschwindigkeitsverteilungsdefinition(const NIVissimXMLHandler_Geschwindigkeitsverteilungsdefinition &)` [private]
    /// invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(vW: &NIImporter_Vissim_NIVissimXMLHandler_Geschwindigkeitsverteilungsdefinition) -> Self {
        panic!("NIVissimXMLHandler_Geschwindigkeitsverteilungsdefinition copy constructor is private and invalidated in C++")
    }

    /// C++: `NIVissimXMLHandler_Geschwindigkeitsverteilungsdefinition & NIVissimXMLHandler_Geschwindigkeitsverteilungsdefinition::operator=(const NIVissimXMLHandler_Geschwindigkeitsverteilungsdefinition &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, vW: &NIImporter_Vissim_NIVissimXMLHandler_Geschwindigkeitsverteilungsdefinition) -> &mut NIImporter_Vissim_NIVissimXMLHandler_Geschwindigkeitsverteilungsdefinition {
        panic!("NIVissimXMLHandler_Geschwindigkeitsverteilungsdefinition assignment operator is private and invalidated in C++")
    }

}

/// A class which extracts VISSIM-Parkplätze from a parsed VISSIM-file
/// C++ class `NIImporter_Vissim::NIVissimXMLHandler_Parkplatzdefinition` (netimport/vissim/NIImporter_Vissim.h)
#[repr(C)]
pub struct NIImporter_Vissim_NIVissimXMLHandler_Parkplatzdefinition {
    /// C++ base class `GenericSAXHandler`
    pub base: GenericSAXHandler,
}

impl std::ops::Deref for NIImporter_Vissim_NIVissimXMLHandler_Parkplatzdefinition {
    type Target = GenericSAXHandler;
    fn deref(&self) -> &GenericSAXHandler {
        &self.base
    }
}
impl std::ops::DerefMut for NIImporter_Vissim_NIVissimXMLHandler_Parkplatzdefinition {
    fn deref_mut(&mut self) -> &mut GenericSAXHandler {
        &mut self.base
    }
}

impl NIImporter_Vissim_NIVissimXMLHandler_Parkplatzdefinition {
    /// C++: `void NIVissimXMLHandler_Parkplatzdefinition::NIVissimXMLHandler_Parkplatzdefinition()`
    /// Constructor
    // NIImporter_Vissim::NIVissimXMLHandler_Parkplatzdefinition::NIVissimXMLHandler_Parkplatzdefinition()
    //     : GenericSAXHandler(vissimTags, VISSIM_TAG_NOTHING,
    //                         vissimAttrs, VISSIM_ATTR_NOTHING,
    //                         "vissim - file") {
    // }
    #[allow(unused_variables)]
    pub fn new() -> Self {
        Self { base: vissim_base_handler() }
    }

    /// C++: `void NIVissimXMLHandler_Parkplatzdefinition::~NIVissimXMLHandler_Parkplatzdefinition() noexcept` [virtual]
    /// Destructor
    // NIImporter_Vissim::NIVissimXMLHandler_Parkplatzdefinition::~NIVissimXMLHandler_Parkplatzdefinition() { }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
    }

    /// C++: `void NIVissimXMLHandler_Parkplatzdefinition::myStartElement(int, const SUMOSAXAttributes &)` [virtual, protected]
    /// Called on the opening of a tag;
    // void
    // NIImporter_Vissim::NIVissimXMLHandler_Parkplatzdefinition::myStartElement(int element, const SUMOSAXAttributes& attrs) {
    //     // finding an actual parkinglot
    //     if (element == VISSIM_TAG_PARKINGLOT) {
    //         //parse all parkinglots
    //         bool ok = true;
    //         int id = attrs.get<int>(VISSIM_ATTR_NO, nullptr, ok);
    //         int edgeid = attrs.get<int>(VISSIM_ATTR_INTLINK, nullptr, ok);
    //         std::string name = attrs.get<std::string>(VISSIM_ATTR_NAME, nullptr, ok, false);
    //         double position = attrs.get<double>(VISSIM_ATTR_POS, nullptr, ok);
    //         std::vector<std::pair<int, int> > assignedVehicles; // (vclass, vwunsch)
    //         //FIXME: vWunsch + Fahzeugklassen einlesen
    //         // There can be s
    //         std::vector<int> districts;
    //         //FIXME: Parkplatzdefinition für mehrere Zonen implementieren
    //         std::vector<double> percentages;
    //         districts.push_back(attrs.get<int>(VISSIM_ATTR_DISTRICT, nullptr, ok));
    //         percentages.push_back(attrs.get<double>(VISSIM_ATTR_PERCENTAGE, nullptr, ok));
    //
    //         NIVissimDistrictConnection::dictionary(id,
    //                                                name,
    //                                                districts,
    //                                                percentages,
    //                                                edgeid,
    //                                                position,
    //                                                assignedVehicles);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) {
        // finding an actual parkinglot
        if element == NIImporter_Vissim_VissimXMLTag::VISSIM_TAG_PARKINGLOT as i32 {
            //parse all parkinglots
            let mut ok = true;
            let id: i32 = attrs.get(NIImporter_Vissim_VissimXMLAttr::VISSIM_ATTR_NO as i32, "", &mut ok, true);
            let edgeid: i32 = attrs.get(NIImporter_Vissim_VissimXMLAttr::VISSIM_ATTR_INTLINK as i32, "", &mut ok, true);
            let name: String = attrs.get(NIImporter_Vissim_VissimXMLAttr::VISSIM_ATTR_NAME as i32, "", &mut ok, false);
            let position: f64 = attrs.get(NIImporter_Vissim_VissimXMLAttr::VISSIM_ATTR_POS as i32, "", &mut ok, true);
            let assignedVehicles: Vec<(i32, i32)> = Vec::new(); // (vclass, vwunsch)
            //FIXME: vWunsch + Fahzeugklassen einlesen
            // There can be s
            let mut districts: Vec<i32> = Vec::new();
            //FIXME: Parkplatzdefinition für mehrere Zonen implementieren
            let mut percentages: Vec<f64> = Vec::new();
            districts.push(attrs.get(NIImporter_Vissim_VissimXMLAttr::VISSIM_ATTR_DISTRICT as i32, "", &mut ok, true));
            percentages.push(attrs.get(NIImporter_Vissim_VissimXMLAttr::VISSIM_ATTR_PERCENTAGE as i32, "", &mut ok, true));

            NIVissimDistrictConnection::dictionary(id, &name, &districts, &percentages, edgeid, position, &assignedVehicles);
        }
    }

    /// C++: `void NIVissimXMLHandler_Parkplatzdefinition::NIVissimXMLHandler_Parkplatzdefinition(const NIVissimXMLHandler_Parkplatzdefinition &)` [private]
    /// invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(z: &NIImporter_Vissim_NIVissimXMLHandler_Parkplatzdefinition) -> Self {
        panic!("NIVissimXMLHandler_Parkplatzdefinition copy constructor is private and invalidated in C++")
    }

    /// C++: `NIVissimXMLHandler_Parkplatzdefinition & NIVissimXMLHandler_Parkplatzdefinition::operator=(const NIVissimXMLHandler_Parkplatzdefinition &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, z: &NIImporter_Vissim_NIVissimXMLHandler_Parkplatzdefinition) -> &mut NIImporter_Vissim_NIVissimXMLHandler_Parkplatzdefinition {
        panic!("NIVissimXMLHandler_Parkplatzdefinition assignment operator is private and invalidated in C++")
    }

}

/// A class which extracts VISSIM-Routes from a parsed VISSIM-file
/// C++ class `NIImporter_Vissim::NIVissimXMLHandler_Routenentscheidungsdefinition` (netimport/vissim/NIImporter_Vissim.h)
#[repr(C)]
pub struct NIImporter_Vissim_NIVissimXMLHandler_Routenentscheidungsdefinition {
    /// C++ base class `GenericSAXHandler`
    pub base: GenericSAXHandler,
    pub myElemData: Ptr<NIImporter_Vissim_nodeMap>,
    /// The current hierarchy level
    pub myHierarchyLevel: i32,
    /// ID of the currently parsed node, for reporting mainly
    pub myLastNodeID: i32,
}

impl std::ops::Deref for NIImporter_Vissim_NIVissimXMLHandler_Routenentscheidungsdefinition {
    type Target = GenericSAXHandler;
    fn deref(&self) -> &GenericSAXHandler {
        &self.base
    }
}
impl std::ops::DerefMut for NIImporter_Vissim_NIVissimXMLHandler_Routenentscheidungsdefinition {
    fn deref_mut(&mut self) -> &mut GenericSAXHandler {
        &mut self.base
    }
}

impl NIImporter_Vissim_NIVissimXMLHandler_Routenentscheidungsdefinition {
    /// C++: `void NIVissimXMLHandler_Routenentscheidungsdefinition::NIVissimXMLHandler_Routenentscheidungsdefinition(nodeMap &)`
    /// Constructor
    // NIImporter_Vissim::NIVissimXMLHandler_Routenentscheidungsdefinition::NIVissimXMLHandler_Routenentscheidungsdefinition(nodeMap& elemData)
    //     : GenericSAXHandler(vissimTags, VISSIM_TAG_NOTHING,
    //                         vissimAttrs, VISSIM_ATTR_NOTHING,
    //                         "vissim - file"),
    //       myElemData(elemData),
    //       myHierarchyLevel(0) {
    //     myElemData.clear();
    // }
    #[allow(unused_variables)]
    pub fn new(elemData: &mut NIImporter_Vissim_nodeMap) -> Self {
        elemData.clear();
        Self { base: vissim_base_handler(), myElemData: Ptr::from_mut(elemData), myHierarchyLevel: 0, myLastNodeID: 0 }
    }

    /// C++: `void NIVissimXMLHandler_Routenentscheidungsdefinition::~NIVissimXMLHandler_Routenentscheidungsdefinition() noexcept` [virtual]
    /// Destructor
    // NIImporter_Vissim::NIVissimXMLHandler_Routenentscheidungsdefinition::~NIVissimXMLHandler_Routenentscheidungsdefinition() { }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
    }

    /// C++: `void NIVissimXMLHandler_Routenentscheidungsdefinition::myStartElement(int, const SUMOSAXAttributes &)` [virtual, protected]
    /// Called on the opening of a tag;
    // void
    // NIImporter_Vissim::NIVissimXMLHandler_Routenentscheidungsdefinition::myStartElement(int element, const SUMOSAXAttributes& attrs) {
    //     myHierarchyLevel++;
    //     if (element == VISSIM_TAG_DECISION_STATIC) {
    //         bool ok = true;
    //         myElemData["startLink"].push_back(attrs.get<std::string>(VISSIM_ATTR_LINK, nullptr, ok));
    //         myElemData["startPos"].push_back(attrs.get<std::string>(VISSIM_ATTR_POS, nullptr, ok));
    //     }
    //     if (element == VISSIM_TAG_ROUTE_STATIC) {
    //         bool ok = true;
    //         myElemData["destLink"].push_back(attrs.get<std::string>(VISSIM_ATTR_DESTLINK, nullptr, ok));
    //         myElemData["destPos"].push_back(attrs.get<std::string>(VISSIM_ATTR_DESTPOS, nullptr, ok));
    //         myElemData["id"].push_back(attrs.get<std::string>(VISSIM_ATTR_NO, nullptr, ok));
    //     }
    //     if (element == VISSIM_TAG_INTOBJECTREF) {
    //         // bool ok = true;
    //     }
    //
    // }
    #[allow(unused_variables)]
    pub fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) {
        self.myHierarchyLevel += 1;
        if element == NIImporter_Vissim_VissimXMLTag::VISSIM_TAG_DECISION_STATIC as i32 {
            let mut ok = true;
            let startLink: String = attrs.get(NIImporter_Vissim_VissimXMLAttr::VISSIM_ATTR_LINK as i32, "", &mut ok, true);
            self.myElemData.m().entry("startLink".to_string()).or_default().push_back(startLink);
            let startPos: String = attrs.get(NIImporter_Vissim_VissimXMLAttr::VISSIM_ATTR_POS as i32, "", &mut ok, true);
            self.myElemData.m().entry("startPos".to_string()).or_default().push_back(startPos);
        }
        if element == NIImporter_Vissim_VissimXMLTag::VISSIM_TAG_ROUTE_STATIC as i32 {
            let mut ok = true;
            let destLink: String = attrs.get(NIImporter_Vissim_VissimXMLAttr::VISSIM_ATTR_DESTLINK as i32, "", &mut ok, true);
            self.myElemData.m().entry("destLink".to_string()).or_default().push_back(destLink);
            let destPos: String = attrs.get(NIImporter_Vissim_VissimXMLAttr::VISSIM_ATTR_DESTPOS as i32, "", &mut ok, true);
            self.myElemData.m().entry("destPos".to_string()).or_default().push_back(destPos);
            let id: String = attrs.get(NIImporter_Vissim_VissimXMLAttr::VISSIM_ATTR_NO as i32, "", &mut ok, true);
            self.myElemData.m().entry("id".to_string()).or_default().push_back(id);
        }
        if element == NIImporter_Vissim_VissimXMLTag::VISSIM_TAG_INTOBJECTREF as i32 {
            // bool ok = true;
        }
    }

    /// C++: `void NIVissimXMLHandler_Routenentscheidungsdefinition::myEndElement(int)` [virtual, protected]
    // void
    // NIImporter_Vissim::NIVissimXMLHandler_Routenentscheidungsdefinition::myEndElement(int /* element */) {
    //     --myHierarchyLevel;
    // }
    #[allow(unused_variables)]
    pub fn my_end_element(&mut self, element: i32) {
        self.myHierarchyLevel -= 1;
    }

    /// C++: `void NIVissimXMLHandler_Routenentscheidungsdefinition::NIVissimXMLHandler_Routenentscheidungsdefinition(const NIVissimXMLHandler_Routenentscheidungsdefinition &)` [private]
    /// invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(r: &NIImporter_Vissim_NIVissimXMLHandler_Routenentscheidungsdefinition) -> Self {
        panic!("NIVissimXMLHandler_Routenentscheidungsdefinition copy constructor is private and invalidated in C++")
    }

    /// C++: `NIVissimXMLHandler_Routenentscheidungsdefinition & NIVissimXMLHandler_Routenentscheidungsdefinition::operator=(const NIVissimXMLHandler_Routenentscheidungsdefinition &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, r: &NIImporter_Vissim_NIVissimXMLHandler_Routenentscheidungsdefinition) -> &mut NIImporter_Vissim_NIVissimXMLHandler_Routenentscheidungsdefinition {
        panic!("NIVissimXMLHandler_Routenentscheidungsdefinition assignment operator is private and invalidated in C++")
    }

}

/// A class which extracts VISSIM-Strecken from a parsed VISSIM-file
/// C++ class `NIImporter_Vissim::NIVissimXMLHandler_Streckendefinition` (netimport/vissim/NIImporter_Vissim.h)
#[repr(C)]
pub struct NIImporter_Vissim_NIVissimXMLHandler_Streckendefinition {
    /// C++ base class `GenericSAXHandler`
    pub base: GenericSAXHandler,
    pub myElemData: Ptr<NIImporter_Vissim_nodeMap>,
    /// The current hierarchy level
    pub myHierarchyLevel: i32,
    /// check if the link is a connector
    pub isConnector: bool,
    /// ID of the currently parsed node, for reporting mainly
    pub myLastNodeID: i32,
}

impl std::ops::Deref for NIImporter_Vissim_NIVissimXMLHandler_Streckendefinition {
    type Target = GenericSAXHandler;
    fn deref(&self) -> &GenericSAXHandler {
        &self.base
    }
}
impl std::ops::DerefMut for NIImporter_Vissim_NIVissimXMLHandler_Streckendefinition {
    fn deref_mut(&mut self) -> &mut GenericSAXHandler {
        &mut self.base
    }
}

impl NIImporter_Vissim_NIVissimXMLHandler_Streckendefinition {
    /// C++: `void NIVissimXMLHandler_Streckendefinition::NIVissimXMLHandler_Streckendefinition(nodeMap &)`
    // NIImporter_Vissim::NIVissimXMLHandler_Streckendefinition::NIVissimXMLHandler_Streckendefinition(
    //     //std::map<int, VissimXMLEdge>& toFill)
    //     nodeMap& elemData)
    //     : GenericSAXHandler(vissimTags, VISSIM_TAG_NOTHING,
    //                         vissimAttrs, VISSIM_ATTR_NOTHING,
    //                         "vissim - file"),
    //       myElemData(elemData),
    //       myHierarchyLevel(0),
    //       isConnector(false) {
    //     myElemData.clear();
    // }
    #[allow(unused_variables)]
    pub fn new(elemData: &mut NIImporter_Vissim_nodeMap) -> Self {
        // C++ leaves myLastNodeID uninitialized; 0 is used here
        Self { base: vissim_base_handler(), myElemData: Ptr::from_mut(elemData), myHierarchyLevel: 0, isConnector: false, myLastNodeID: 0 }
    }

    /// C++: `void NIVissimXMLHandler_Streckendefinition::~NIVissimXMLHandler_Streckendefinition() noexcept` [virtual]
    /// Destructor
    // NIImporter_Vissim::NIVissimXMLHandler_Streckendefinition::~NIVissimXMLHandler_Streckendefinition() { }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
    }

    /// C++: `void NIVissimXMLHandler_Streckendefinition::myStartElement(int, const SUMOSAXAttributes &)` [virtual, protected]
    /// Called on the opening of a tag;
    // void
    // NIImporter_Vissim::NIVissimXMLHandler_Streckendefinition::myStartElement(int element, const SUMOSAXAttributes& attrs) {
    //     myHierarchyLevel++;
    //
    //     // finding an actual LINK
    //     if (element == VISSIM_TAG_LINK) {
    //         //parse all links
    //         bool ok = true;
    //         int id = attrs.get<int>(VISSIM_ATTR_NO, nullptr, ok);
    //         myLastNodeID = id;
    //
    //         // !!! assuming empty myElemData
    //         myElemData["id"].push_back(attrs.get<std::string>(VISSIM_ATTR_NO, nullptr, ok));
    //         // error ignored if name is empty
    //         myElemData["name"].push_back(attrs.get<std::string>(VISSIM_ATTR_NAME, nullptr, ok, false));
    //         myElemData["type"].push_back(attrs.get<std::string>(VISSIM_ATTR_LINKBEHAVETYPE, nullptr, ok));
    //         myElemData["zuschlag1"].push_back(attrs.get<std::string>(VISSIM_ATTR_ZUSCHLAG1, nullptr, ok));
    //         myElemData["zuschlag2"].push_back(attrs.get<std::string>(VISSIM_ATTR_ZUSCHLAG2, nullptr, ok));
    //     }
    //
    //     if (element == VISSIM_TAG_LANE) {
    //         bool ok = true;
    //         // appends empty element if no width found
    //         // error ignored if name is empty
    //         myElemData["width"].push_back(attrs.get<std::string>(VISSIM_ATTR_WIDTH, nullptr, ok, false));
    //     }
    //
    //     if (element == VISSIM_TAG_FROM) {
    //         if (isConnector != true) {
    //             isConnector = true;
    //         }
    //         bool ok = true;
    //         std::vector<std::string> from(StringTokenizer(attrs.get<std::string>(
    //                                           VISSIM_ATTR_LANE, nullptr, ok), " ").getVector());
    //         myElemData["from_pos"].push_back(attrs.get<std::string>(VISSIM_ATTR_POS, nullptr, ok));
    //         myElemData["from_id"].push_back(from[0]);
    //         myElemData["from_lane"].push_back(from[1]);
    //     }
    //
    //     if (element == VISSIM_TAG_TO) {
    //         bool ok = true;
    //         std::vector<std::string> to(StringTokenizer(attrs.get<std::string>(
    //                                         VISSIM_ATTR_LANE, nullptr, ok), " ").getVector());
    //         myElemData["to_pos"].push_back(attrs.get<std::string>(VISSIM_ATTR_POS, nullptr, ok));
    //         myElemData["to_id"].push_back(to[0]);
    //         myElemData["to_lane"].push_back(to[1]);
    //     }
    //
    //     if (element == VISSIM_TAG_POINT3D || element == VISSIM_TAG_LINKPOLYPOINT) {
    //         bool ok = true;
    //         // create a <sep> separated string of coordinate data
    //         std::string sep(" ");
    //
    //         std::string posS(attrs.get<std::string>(VISSIM_ATTR_X, nullptr, ok));
    //         posS += sep;
    //         posS.append(attrs.get<std::string>(VISSIM_ATTR_Y, nullptr, ok));
    //         // allow for no Z
    //         std::string z(attrs.get<std::string>(VISSIM_ATTR_ZOFFSET, nullptr, ok, false));
    //         if (z.length() > 0) {
    //             posS += sep;
    //             posS.append(z);
    //         }
    //         myElemData["pos"].push_back(posS);
    //     }
    //
    //
    // }
    #[allow(unused_variables)]
    pub fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) {
        self.myHierarchyLevel += 1;

        // finding an actual LINK
        if element == NIImporter_Vissim_VissimXMLTag::VISSIM_TAG_LINK as i32 {
            //parse all links
            let mut ok = true;
            let id: i32 = attrs.get(NIImporter_Vissim_VissimXMLAttr::VISSIM_ATTR_NO as i32, "", &mut ok, true);
            self.myLastNodeID = id;

            // !!! assuming empty myElemData
            elem_push(&self.myElemData, "id", attrs.get(NIImporter_Vissim_VissimXMLAttr::VISSIM_ATTR_NO as i32, "", &mut ok, true));
            // error ignored if name is empty
            elem_push(&self.myElemData, "name", attrs.get(NIImporter_Vissim_VissimXMLAttr::VISSIM_ATTR_NAME as i32, "", &mut ok, false));
            elem_push(&self.myElemData, "type", attrs.get(NIImporter_Vissim_VissimXMLAttr::VISSIM_ATTR_LINKBEHAVETYPE as i32, "", &mut ok, true));
            elem_push(&self.myElemData, "zuschlag1", attrs.get(NIImporter_Vissim_VissimXMLAttr::VISSIM_ATTR_ZUSCHLAG1 as i32, "", &mut ok, true));
            elem_push(&self.myElemData, "zuschlag2", attrs.get(NIImporter_Vissim_VissimXMLAttr::VISSIM_ATTR_ZUSCHLAG2 as i32, "", &mut ok, true));
        }

        if element == NIImporter_Vissim_VissimXMLTag::VISSIM_TAG_LANE as i32 {
            let mut ok = true;
            // appends empty element if no width found
            // error ignored if name is empty
            elem_push(&self.myElemData, "width", attrs.get(NIImporter_Vissim_VissimXMLAttr::VISSIM_ATTR_WIDTH as i32, "", &mut ok, false));
        }

        if element == NIImporter_Vissim_VissimXMLTag::VISSIM_TAG_FROM as i32 {
            if self.isConnector != true {
                self.isConnector = true;
            }
            let mut ok = true;
            let laneStr: String = attrs.get(NIImporter_Vissim_VissimXMLAttr::VISSIM_ATTR_LANE as i32, "", &mut ok, true);
            let from: Vec<String> = StringTokenizer::with_token(&laneStr, " ", false).get_vector();
            elem_push(&self.myElemData, "from_pos", attrs.get(NIImporter_Vissim_VissimXMLAttr::VISSIM_ATTR_POS as i32, "", &mut ok, true));
            elem_push(&self.myElemData, "from_id", from[0].clone());
            elem_push(&self.myElemData, "from_lane", from[1].clone());
        }

        if element == NIImporter_Vissim_VissimXMLTag::VISSIM_TAG_TO as i32 {
            let mut ok = true;
            let laneStr: String = attrs.get(NIImporter_Vissim_VissimXMLAttr::VISSIM_ATTR_LANE as i32, "", &mut ok, true);
            let to: Vec<String> = StringTokenizer::with_token(&laneStr, " ", false).get_vector();
            elem_push(&self.myElemData, "to_pos", attrs.get(NIImporter_Vissim_VissimXMLAttr::VISSIM_ATTR_POS as i32, "", &mut ok, true));
            elem_push(&self.myElemData, "to_id", to[0].clone());
            elem_push(&self.myElemData, "to_lane", to[1].clone());
        }

        if element == NIImporter_Vissim_VissimXMLTag::VISSIM_TAG_POINT3D as i32 || element == NIImporter_Vissim_VissimXMLTag::VISSIM_TAG_LINKPOLYPOINT as i32 {
            let mut ok = true;
            // create a <sep> separated string of coordinate data
            let sep = " ";

            let mut posS: String = attrs.get(NIImporter_Vissim_VissimXMLAttr::VISSIM_ATTR_X as i32, "", &mut ok, true);
            posS += sep;
            let y: String = attrs.get(NIImporter_Vissim_VissimXMLAttr::VISSIM_ATTR_Y as i32, "", &mut ok, true);
            posS.push_str(&y);
            // allow for no Z
            let z: String = attrs.get(NIImporter_Vissim_VissimXMLAttr::VISSIM_ATTR_ZOFFSET as i32, "", &mut ok, false);
            if z.len() > 0 {
                posS += sep;
                posS.push_str(&z);
            }
            elem_push(&self.myElemData, "pos", posS);
        }
    }

    /// C++: `void NIVissimXMLHandler_Streckendefinition::myEndElement(int)` [virtual, protected]
    // void
    // NIImporter_Vissim::NIVissimXMLHandler_Streckendefinition::myEndElement(int element) {
    //     if (element == VISSIM_TAG_LINK && myHierarchyLevel == 3) {
    //         //std::cout << "elemData len:" << myElemData.size() << std::endl;
    //
    //         NIVissimClosedLanesVector clv;          //FIXME -> clv einlesen
    //         std::vector<int> assignedVehicles;      //FIXME -> assignedVehicles einlesen
    //         int id(StringUtils::toInt(myElemData["id"].front()));
    //
    //         PositionVector geom;
    //         // convert all position coordinate strings to PositionVectors
    //         while (!myElemData["pos"].empty()) {
    //             std::vector<std::string> sPos_v(StringTokenizer(
    //                                                 myElemData["pos"].front(), " ").getVector());
    //             myElemData["pos"].pop_front();
    //             std::vector<double> pos_v(3);
    //
    //             // doing a transform with explicit hint on function signature
    //             std::transform(sPos_v.begin(), sPos_v.end(), pos_v.begin(),
    //                            StringUtils::toDouble);
    //             geom.push_back_noDoublePos(Position(pos_v[0], pos_v[1], pos_v[2]));
    //         }
    //         // FIXME: a length = 0 PosVec seems fatal -> segfault
    //         double length(geom.length());
    //
    //         if (!isConnector) {
    //             // Add Edge
    //             std::vector<double> laneWidths;
    //             for (std::string& w : myElemData["width"]) {
    //                 laneWidths.push_back(StringUtils::toDouble(w));
    //             }
    //             NIVissimEdge* edge = new NIVissimEdge(id,
    //                                                   myElemData["name"].front(),
    //                                                   myElemData["type"].front(),
    //                                                   laneWidths,
    //                                                   StringUtils::toDouble(myElemData["zuschlag1"].front()),
    //                                                   StringUtils::toDouble(myElemData["zuschlag2"].front()),
    //                                                   length, geom, clv);
    //             NIVissimEdge::dictionary(id, edge);
    //         } else {
    //             int numLanes = (int)myElemData["width"].size();
    //             std::vector<int> laneVec(numLanes);
    //             // Add Connector
    //
    //             //NOTE: there should be only 1 lane number in XML
    //             // subtraction of 1 as in readExtEdgePointDef()
    //             laneVec[0] = StringUtils::toInt(myElemData["from_lane"].front()) - 1;
    //             // then count up, building lane number vector
    //             for (std::vector<int>::iterator each = ++laneVec.begin(); each != laneVec.end(); ++each) {
    //                 *each = *(each - 1) + 1;
    //             }
    //
    //             NIVissimExtendedEdgePoint from_def(
    //                 StringUtils::toInt(myElemData["from_id"].front()),
    //                 laneVec,
    //                 StringUtils::toDouble(myElemData["from_pos"].front()),
    //                 assignedVehicles);
    //
    //             //NOTE: there should be only 1 lane number in XML
    //             // subtraction of 1 as in readExtEdgePointDef()
    //             laneVec[0] = StringUtils::toInt(myElemData["to_lane"].front()) - 1;
    //             // then count up, building lane number vector
    //             for (std::vector<int>::iterator each = ++laneVec.begin(); each != laneVec.end(); ++each) {
    //                 *each = *(each - 1) + 1;
    //             }
    //
    //             NIVissimExtendedEdgePoint to_def(
    //                 StringUtils::toInt(myElemData["to_id"].front()),
    //                 laneVec,
    //                 StringUtils::toDouble(myElemData["to_pos"].front()),
    //                 assignedVehicles);
    //
    //             NIVissimConnection* connector = new
    //             NIVissimConnection(id,
    //                                myElemData["name"].front(),
    //                                from_def, to_def,
    //                                geom, assignedVehicles, clv);
    //
    //             NIVissimConnection::dictionary(id, connector);
    //         }
    //         // clear the element data
    //         myElemData.clear();
    //         isConnector = false;
    //         //std::cout << "elemData len (clear):" << myElemData.size() << std::endl;
    //         //std::cout.flush();
    //
    //     }
    //     --myHierarchyLevel;
    // }
    #[allow(unused_variables)]
    pub fn my_end_element(&mut self, element: i32) {
        if element == NIImporter_Vissim_VissimXMLTag::VISSIM_TAG_LINK as i32 && self.myHierarchyLevel == 3 {
            //std::cout << "elemData len:" << myElemData.size() << std::endl;

            let clv: NIVissimClosedLanesVector = Vec::new(); //FIXME -> clv einlesen
            let assignedVehicles: Vec<i32> = Vec::new(); //FIXME -> assignedVehicles einlesen
            let id: i32 = vissim_parse(to_int(&elem_front(&self.myElemData, "id")));

            let mut geom = PositionVector::default();
            // convert all position coordinate strings to PositionVectors
            loop {
                let posFront = match elem_pop_front(&self.myElemData, "pos") {
                    Some(p) => p,
                    None => break,
                };
                let sPos_v: Vec<String> = StringTokenizer::with_token(&posFront, " ", false).get_vector();
                // std::transform into pos_v(3): only the first three values fit
                let mut pos_v = vec![0.0f64; 3];
                for (i, sPos) in sPos_v.iter().enumerate() {
                    let v = vissim_parse(to_double(sPos));
                    if i < 3 {
                        pos_v[i] = v;
                    }
                }
                geom.push_back_no_double_pos(&Position::new3(pos_v[0], pos_v[1], pos_v[2]));
            }
            // FIXME: a length = 0 PosVec seems fatal -> segfault
            let length: f64 = geom.length();

            if !self.isConnector {
                // Add Edge
                let mut laneWidths: Vec<f64> = Vec::new();
                let widths: Vec<String> = self.myElemData.m().entry("width".to_string()).or_default().iter().cloned().collect();
                for w in widths.iter() {
                    laneWidths.push(vissim_parse(to_double(w)));
                }
                let name = elem_front(&self.myElemData, "name");
                let type_ = elem_front(&self.myElemData, "type");
                let zuschlag1 = vissim_parse(to_double(&elem_front(&self.myElemData, "zuschlag1")));
                let zuschlag2 = vissim_parse(to_double(&elem_front(&self.myElemData, "zuschlag2")));
                let mut edge = NIVissimEdge::new(id, &name, &type_, laneWidths, zuschlag1, zuschlag2, length, &geom, &clv);
                NIVissimEdge::dictionary_2(id, Ptr::from_mut(&mut *edge));
                // shortcut: the dictionary keeps the edge for the rest of the run (C++ never frees it here), so leak the Box
                Box::leak(edge);
            } else {
                let numLanes = self.myElemData.m().entry("width".to_string()).or_default().len() as i32;
                let mut laneVec: Vec<i32> = vec![0; numLanes as usize];
                // Add Connector

                //NOTE: there should be only 1 lane number in XML
                // subtraction of 1 as in readExtEdgePointDef()
                let fromLane = vissim_parse(to_int(&elem_front(&self.myElemData, "from_lane"))) - 1;
                if !laneVec.is_empty() {
                    laneVec[0] = fromLane;
                }
                // then count up, building lane number vector
                for each in 1..laneVec.len() {
                    laneVec[each] = laneVec[each - 1] + 1;
                }

                let from_def = NIVissimExtendedEdgePoint::new(
                    vissim_parse(to_int(&elem_front(&self.myElemData, "from_id"))),
                    &laneVec,
                    vissim_parse(to_double(&elem_front(&self.myElemData, "from_pos"))),
                    &assignedVehicles);

                //NOTE: there should be only 1 lane number in XML
                // subtraction of 1 as in readExtEdgePointDef()
                let toLane = vissim_parse(to_int(&elem_front(&self.myElemData, "to_lane"))) - 1;
                if !laneVec.is_empty() {
                    laneVec[0] = toLane;
                }
                // then count up, building lane number vector
                for each in 1..laneVec.len() {
                    laneVec[each] = laneVec[each - 1] + 1;
                }

                let to_def = NIVissimExtendedEdgePoint::new(
                    vissim_parse(to_int(&elem_front(&self.myElemData, "to_id"))),
                    &laneVec,
                    vissim_parse(to_double(&elem_front(&self.myElemData, "to_pos"))),
                    &assignedVehicles);

                let name = elem_front(&self.myElemData, "name");
                let connector = Box::new(NIVissimConnection::new(id, &name, &from_def, &to_def, &geom, &assignedVehicles, &clv));
                NIVissimConnection::dictionary(id, Ptr::from_box(&connector));
                // shortcut: the dictionary keeps the connector for the rest of the run (C++ never frees it here), so leak the Box
                Box::leak(connector);
            }
            // clear the element data
            self.myElemData.m().clear();
            self.isConnector = false;
            //std::cout << "elemData len (clear):" << myElemData.size() << std::endl;
            //std::cout.flush();

        }
        self.myHierarchyLevel -= 1;
    }

    /// C++: `void NIVissimXMLHandler_Streckendefinition::NIVissimXMLHandler_Streckendefinition(const NIVissimXMLHandler_Streckendefinition &)` [private]
    /// invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(s: &NIImporter_Vissim_NIVissimXMLHandler_Streckendefinition) -> Self {
        panic!("NIVissimXMLHandler_Streckendefinition copy constructor is private and invalidated in C++")
    }

    /// C++: `NIVissimXMLHandler_Streckendefinition & NIVissimXMLHandler_Streckendefinition::operator=(const NIVissimXMLHandler_Streckendefinition &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &NIImporter_Vissim_NIVissimXMLHandler_Streckendefinition) -> &mut NIImporter_Vissim_NIVissimXMLHandler_Streckendefinition {
        panic!("NIVissimXMLHandler_Streckendefinition assignment operator is private and invalidated in C++")
    }

}

/// A class which extracts VISSIM-VWunschentscheidungen from a parsed VISSIM-file
/// C++ class `NIImporter_Vissim::NIVissimXMLHandler_VWunschentscheidungsdefinition` (netimport/vissim/NIImporter_Vissim.h)
#[repr(C)]
pub struct NIImporter_Vissim_NIVissimXMLHandler_VWunschentscheidungsdefinition {
    /// C++ base class `GenericSAXHandler`
    pub base: GenericSAXHandler,
    pub myElemData: Ptr<NIImporter_Vissim_nodeMap>,
    /// The current hierarchy level
    pub myHierarchyLevel: i32,
    /// ID of the currently parsed node, for reporting mainly
    pub myLastNodeID: i32,
}

impl std::ops::Deref for NIImporter_Vissim_NIVissimXMLHandler_VWunschentscheidungsdefinition {
    type Target = GenericSAXHandler;
    fn deref(&self) -> &GenericSAXHandler {
        &self.base
    }
}
impl std::ops::DerefMut for NIImporter_Vissim_NIVissimXMLHandler_VWunschentscheidungsdefinition {
    fn deref_mut(&mut self) -> &mut GenericSAXHandler {
        &mut self.base
    }
}

impl NIImporter_Vissim_NIVissimXMLHandler_VWunschentscheidungsdefinition {
    /// C++: `void NIVissimXMLHandler_VWunschentscheidungsdefinition::NIVissimXMLHandler_VWunschentscheidungsdefinition(nodeMap &)`
    /// Constructor
    // NIImporter_Vissim::NIVissimXMLHandler_VWunschentscheidungsdefinition::NIVissimXMLHandler_VWunschentscheidungsdefinition(nodeMap& elemData)
    //     : GenericSAXHandler(vissimTags, VISSIM_TAG_NOTHING,
    //                         vissimAttrs, VISSIM_ATTR_NOTHING,
    //                         "vissim - file"),
    //       myElemData(elemData),
    //       myHierarchyLevel(0) {
    //     myElemData.clear();
    // }
    #[allow(unused_variables)]
    pub fn new(elemData: &mut NIImporter_Vissim_nodeMap) -> Self {
        elemData.clear();
        Self { base: vissim_base_handler(), myElemData: Ptr::from_mut(elemData), myHierarchyLevel: 0, myLastNodeID: 0 }
    }

    /// C++: `void NIVissimXMLHandler_VWunschentscheidungsdefinition::~NIVissimXMLHandler_VWunschentscheidungsdefinition() noexcept` [virtual]
    /// Destructor
    // NIImporter_Vissim::NIVissimXMLHandler_VWunschentscheidungsdefinition::~NIVissimXMLHandler_VWunschentscheidungsdefinition() { }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
    }

    /// C++: `void NIVissimXMLHandler_VWunschentscheidungsdefinition::myStartElement(int, const SUMOSAXAttributes &)` [virtual, protected]
    /// Called on the opening of a tag;
    // void
    // NIImporter_Vissim::NIVissimXMLHandler_VWunschentscheidungsdefinition::myStartElement(int element, const SUMOSAXAttributes& attrs) {
    //     myHierarchyLevel++;
    //     if (element == VISSIM_TAG_SPEED_DECISION) {
    //         bool ok = true;
    //         myElemData["name"].push_back(attrs.get<std::string>(VISSIM_ATTR_NAME, nullptr, ok, false));
    //         //FIXME: 2 vWunsch in the xml file, but only 1 of them is set???
    //     }
    //
    // }
    #[allow(unused_variables)]
    pub fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) {
        self.myHierarchyLevel += 1;
        if element == NIImporter_Vissim_VissimXMLTag::VISSIM_TAG_SPEED_DECISION as i32 {
            let mut ok = true;
            elem_push(&self.myElemData, "name", attrs.get(NIImporter_Vissim_VissimXMLAttr::VISSIM_ATTR_NAME as i32, "", &mut ok, false));
            //FIXME: 2 vWunsch in the xml file, but only 1 of them is set???
        }
    }

    /// C++: `void NIVissimXMLHandler_VWunschentscheidungsdefinition::myEndElement(int)` [virtual, protected]
    // void
    // NIImporter_Vissim::NIVissimXMLHandler_VWunschentscheidungsdefinition::myEndElement(int /* element */) {
    //     --myHierarchyLevel;
    // }
    #[allow(unused_variables)]
    pub fn my_end_element(&mut self, element: i32) {
        self.myHierarchyLevel -= 1;
    }

    /// C++: `void NIVissimXMLHandler_VWunschentscheidungsdefinition::NIVissimXMLHandler_VWunschentscheidungsdefinition(const NIVissimXMLHandler_VWunschentscheidungsdefinition &)` [private]
    /// invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(vW: &NIImporter_Vissim_NIVissimXMLHandler_VWunschentscheidungsdefinition) -> Self {
        panic!("NIVissimXMLHandler_VWunschentscheidungsdefinition copy constructor is private and invalidated in C++")
    }

    /// C++: `NIVissimXMLHandler_VWunschentscheidungsdefinition & NIVissimXMLHandler_VWunschentscheidungsdefinition::operator=(const NIVissimXMLHandler_VWunschentscheidungsdefinition &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, vW: &NIImporter_Vissim_NIVissimXMLHandler_VWunschentscheidungsdefinition) -> &mut NIImporter_Vissim_NIVissimXMLHandler_VWunschentscheidungsdefinition {
        panic!("NIVissimXMLHandler_VWunschentscheidungsdefinition assignment operator is private and invalidated in C++")
    }

}

/// A class which extracts VISSIM-Zuflüsse from a parsed VISSIM-file
/// C++ class `NIImporter_Vissim::NIVissimXMLHandler_Zuflussdefinition` (netimport/vissim/NIImporter_Vissim.h)
#[repr(C)]
pub struct NIImporter_Vissim_NIVissimXMLHandler_Zuflussdefinition {
    /// C++ base class `GenericSAXHandler`
    pub base: GenericSAXHandler,
}

impl std::ops::Deref for NIImporter_Vissim_NIVissimXMLHandler_Zuflussdefinition {
    type Target = GenericSAXHandler;
    fn deref(&self) -> &GenericSAXHandler {
        &self.base
    }
}
impl std::ops::DerefMut for NIImporter_Vissim_NIVissimXMLHandler_Zuflussdefinition {
    fn deref_mut(&mut self) -> &mut GenericSAXHandler {
        &mut self.base
    }
}

impl NIImporter_Vissim_NIVissimXMLHandler_Zuflussdefinition {
    /// C++: `void NIVissimXMLHandler_Zuflussdefinition::NIVissimXMLHandler_Zuflussdefinition()`
    /// Constructor
    // NIImporter_Vissim::NIVissimXMLHandler_Zuflussdefinition::NIVissimXMLHandler_Zuflussdefinition()
    //     : GenericSAXHandler(vissimTags, VISSIM_TAG_NOTHING,
    //                         vissimAttrs, VISSIM_ATTR_NOTHING,
    //                         "vissim - file") {
    // }
    #[allow(unused_variables)]
    pub fn new() -> Self {
        Self { base: vissim_base_handler() }
    }

    /// C++: `void NIVissimXMLHandler_Zuflussdefinition::~NIVissimXMLHandler_Zuflussdefinition() noexcept` [virtual]
    /// Destructor
    // NIImporter_Vissim::NIVissimXMLHandler_Zuflussdefinition::~NIVissimXMLHandler_Zuflussdefinition() { }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
    }

    /// C++: `void NIVissimXMLHandler_Zuflussdefinition::myStartElement(int, const SUMOSAXAttributes &)` [virtual, protected]
    /// Called on the opening of a tag;
    // void
    // NIImporter_Vissim::NIVissimXMLHandler_Zuflussdefinition::myStartElement(int element, const SUMOSAXAttributes& attrs) {
    //     // finding an actual flow
    //     if (element == VISSIM_TAG_VEHICLE_INPUT) {
    //         //parse all flows
    //         bool ok = true;
    //         std::string id = attrs.get<std::string>(VISSIM_ATTR_NO, nullptr, ok);
    //         std::string edgeid = attrs.get<std::string>(VISSIM_ATTR_LINK, nullptr, ok);
    //         std::string name = attrs.get<std::string>(VISSIM_ATTR_NAME, nullptr, ok, false);
    //
    //         NIVissimSource::dictionary(id,
    //                                    name,
    //                                    edgeid);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) {
        // finding an actual flow
        if element == NIImporter_Vissim_VissimXMLTag::VISSIM_TAG_VEHICLE_INPUT as i32 {
            //parse all flows
            let mut ok = true;
            let id: String = attrs.get(NIImporter_Vissim_VissimXMLAttr::VISSIM_ATTR_NO as i32, "", &mut ok, true);
            let edgeid: String = attrs.get(NIImporter_Vissim_VissimXMLAttr::VISSIM_ATTR_LINK as i32, "", &mut ok, true);
            let name: String = attrs.get(NIImporter_Vissim_VissimXMLAttr::VISSIM_ATTR_NAME as i32, "", &mut ok, false);

            NIVissimSource::dictionary(&id, &name, &edgeid);
        }
    }

    /// C++: `void NIVissimXMLHandler_Zuflussdefinition::NIVissimXMLHandler_Zuflussdefinition(const NIVissimXMLHandler_Zuflussdefinition &)` [private]
    /// invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(z: &NIImporter_Vissim_NIVissimXMLHandler_Zuflussdefinition) -> Self {
        panic!("NIVissimXMLHandler_Zuflussdefinition copy constructor is private and invalidated in C++")
    }

    /// C++: `NIVissimXMLHandler_Zuflussdefinition & NIVissimXMLHandler_Zuflussdefinition::operator=(const NIVissimXMLHandler_Zuflussdefinition &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, z: &NIImporter_Vissim_NIVissimXMLHandler_Zuflussdefinition) -> &mut NIImporter_Vissim_NIVissimXMLHandler_Zuflussdefinition {
        panic!("NIVissimXMLHandler_Zuflussdefinition assignment operator is private and invalidated in C++")
    }

}

// The handlers that load() runs through XMLSubSys::runParser implement the SaxHandler trait; the
// trait methods forward to the inherent myStartElement/myEndElement above.
impl SaxHandler for NIImporter_Vissim_NIVissimXMLHandler_ConflictArea {
    fn handler(&mut self) -> &mut GenericSAXHandler {
        &mut self.base
    }
    fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) -> Result<(), ProcessError> {
        NIImporter_Vissim_NIVissimXMLHandler_ConflictArea::my_start_element(self, element, attrs);
        Ok(())
    }
}

impl SaxHandler for NIImporter_Vissim_NIVissimXMLHandler_Fahrzeugklassendefinition {
    fn handler(&mut self) -> &mut GenericSAXHandler {
        &mut self.base
    }
    fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) -> Result<(), ProcessError> {
        NIImporter_Vissim_NIVissimXMLHandler_Fahrzeugklassendefinition::my_start_element(self, element, attrs);
        Ok(())
    }
    fn my_end_element(&mut self, element: i32) -> Result<(), ProcessError> {
        NIImporter_Vissim_NIVissimXMLHandler_Fahrzeugklassendefinition::my_end_element(self, element);
        Ok(())
    }
}

impl SaxHandler for NIImporter_Vissim_NIVissimXMLHandler_Geschwindigkeitsverteilungsdefinition {
    fn handler(&mut self) -> &mut GenericSAXHandler {
        &mut self.base
    }
    fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) -> Result<(), ProcessError> {
        NIImporter_Vissim_NIVissimXMLHandler_Geschwindigkeitsverteilungsdefinition::my_start_element(self, element, attrs);
        Ok(())
    }
    fn my_end_element(&mut self, element: i32) -> Result<(), ProcessError> {
        NIImporter_Vissim_NIVissimXMLHandler_Geschwindigkeitsverteilungsdefinition::my_end_element(self, element);
        Ok(())
    }
}

impl SaxHandler for NIImporter_Vissim_NIVissimXMLHandler_Streckendefinition {
    fn handler(&mut self) -> &mut GenericSAXHandler {
        &mut self.base
    }
    fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) -> Result<(), ProcessError> {
        NIImporter_Vissim_NIVissimXMLHandler_Streckendefinition::my_start_element(self, element, attrs);
        Ok(())
    }
    fn my_end_element(&mut self, element: i32) -> Result<(), ProcessError> {
        NIImporter_Vissim_NIVissimXMLHandler_Streckendefinition::my_end_element(self, element);
        Ok(())
    }
}

impl SaxHandler for NIImporter_Vissim_NIVissimXMLHandler_Zuflussdefinition {
    fn handler(&mut self) -> &mut GenericSAXHandler {
        &mut self.base
    }
    fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) -> Result<(), ProcessError> {
        NIImporter_Vissim_NIVissimXMLHandler_Zuflussdefinition::my_start_element(self, element, attrs);
        Ok(())
    }
}

/// C++ class `NIImporter_Vissim::VissimSingleTypeParser` (netimport/vissim/NIImporter_Vissim.h)
pub struct NIImporter_Vissim_VissimSingleTypeParser {
    pub myVissimParent: Ptr<NIImporter_Vissim>,
}

impl NIImporter_Vissim_VissimSingleTypeParser {
    /// C++: `void VissimSingleTypeParser::VissimSingleTypeParser(NIImporter_Vissim &)`
    /// Constructor
    // NIImporter_Vissim::VissimSingleTypeParser::VissimSingleTypeParser(NIImporter_Vissim& parent)
    //     : myVissimParent(parent) {}
    #[allow(unused_variables)]
    pub fn new(parent: &mut NIImporter_Vissim) -> Self {
        Self { myVissimParent: Ptr::from_mut(parent) }
    }

    /// C++: `void VissimSingleTypeParser::~VissimSingleTypeParser()` [virtual]
    /// Destructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
    }

    /// C++: `bool VissimSingleTypeParser::parse(std::istream &)` [virtual, pure virtual]
    /// Parses a single data type. Returns whether no error occurred
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn parse(&mut self, from: &mut std::io::Cursor<Vec<u8>> /* std::istream */) -> bool {
        panic!("VissimSingleTypeParser::parse is pure virtual; only the subclasses parse")
    }

    /// C++: `std::string VissimSingleTypeParser::myRead(std::istream &)` [protected]
    /// reads from the stream and returns the lower case version of the read value
    // std::string
    // NIImporter_Vissim::VissimSingleTypeParser::myRead(std::istream& from) {
    //     std::string tmp;
    //     from >> tmp;
    //     return StringUtils::to_lower_case(tmp);
    // }
    #[allow(unused_variables)]
    pub fn my_read(&mut self, from: &mut std::io::Cursor<Vec<u8>> /* std::istream */) -> String {
        let tmp = vissim_read_token(from);
        to_lower_case(&tmp)
    }

    /// C++: `std::string VissimSingleTypeParser::readEndSecure(std::istream &, const std::string &)` [protected]
    /// as myRead, but returns "DATAEND" when the current field has ended
    // std::string
    // NIImporter_Vissim::VissimSingleTypeParser::readEndSecure(std::istream& from,
    //         const std::string& excl) {
    //     std::string myExcl = StringUtils::to_lower_case(excl);
    //     std::string tmp = myRead(from);
    //     if (tmp == "") {
    //         return "DATAEND";
    //     }
    //     if (tmp != myExcl
    //             &&
    //             (tmp.substr(0, 2) == "--" || !myVissimParent.admitContinue(tmp))
    //        ) {
    //         return "DATAEND";
    //     }
    //     return StringUtils::to_lower_case(tmp);
    // }
    #[allow(unused_variables)]
    pub fn read_end_secure(&mut self, from: &mut std::io::Cursor<Vec<u8>> /* std::istream */, excl: &str) -> String {
        let myExcl = to_lower_case(excl);
        let tmp = self.my_read(from);
        if tmp == "" {
            return "DATAEND".to_string();
        }
        if tmp != myExcl && (tmp.starts_with("--") || !self.myVissimParent.m().admit_continue(&tmp)) {
            return "DATAEND".to_string();
        }
        to_lower_case(&tmp)
    }

    /// C++: `std::string VissimSingleTypeParser::readEndSecure(std::istream &, const std::vector<std::string> &)` [protected]
    // std::string
    // NIImporter_Vissim::VissimSingleTypeParser::readEndSecure(std::istream& from,
    //         const std::vector<std::string>& excl) {
    //     std::vector<std::string> myExcl;
    //     std::vector<std::string>::const_iterator i;
    //     for (i = excl.begin(); i != excl.end(); i++) {
    //         std::string mes = StringUtils::to_lower_case(*i);
    //         myExcl.push_back(mes);
    //     }
    //     std::string tmp = myRead(from);
    //     if (tmp == "") {
    //         return "DATAEND";
    //     }
    //
    //     bool equals = false;
    //     for (i = myExcl.begin(); i != myExcl.end() && !equals; i++) {
    //         if ((*i) == tmp) {
    //             equals = true;
    //         }
    //     }
    //     if (!equals
    //             &&
    //             (tmp.substr(0, 2) == "--" || !myVissimParent.admitContinue(tmp))
    //        ) {
    //         return "DATAEND";
    //     }
    //     return StringUtils::to_lower_case(tmp);
    // }
    #[allow(unused_variables)]
    pub fn read_end_secure_2(&mut self, from: &mut std::io::Cursor<Vec<u8>> /* std::istream */, excl: &Vec<String>) -> String {
        let myExcl: Vec<String> = excl.iter().map(|e| to_lower_case(e)).collect();
        let tmp = self.my_read(from);
        if tmp == "" {
            return "DATAEND".to_string();
        }

        let equals = myExcl.iter().any(|e| *e == tmp);
        if !equals && (tmp.starts_with("--") || !self.myVissimParent.m().admit_continue(&tmp)) {
            return "DATAEND".to_string();
        }
        to_lower_case(&tmp)
    }

    /// C++: `std::string VissimSingleTypeParser::overrideOptionalLabel(std::istream &, const std::string &)` [protected]
    /// overrides the optional label definition; returns the next tag as done by readEndSecure
    // std::string
    // NIImporter_Vissim::VissimSingleTypeParser::overrideOptionalLabel(std::istream& from,
    //         const std::string& tag) {
    //     std::string tmp;
    //     if (tag == "") {
    //         tmp = myRead(from);
    //     } else {
    //         tmp = tag;
    //     }
    //     if (tmp == "beschriftung") {
    //         tmp = myRead(from);
    //         if (tmp == "keine") {
    //             from >> tmp;
    //         }
    //         tmp = myRead(from);
    //         tmp = myRead(from);
    //     }
    //     return tmp;
    // }
    #[allow(unused_variables)]
    pub fn override_optional_label(&mut self, from: &mut std::io::Cursor<Vec<u8>> /* std::istream */, tag: &str) -> String {
        let mut tmp: String;
        if tag == "" {
            tmp = self.my_read(from);
        } else {
            tmp = tag.to_string();
        }
        if tmp == "beschriftung" {
            tmp = self.my_read(from);
            if tmp == "keine" {
                tmp = vissim_read_token(from);
            }
            tmp = self.my_read(from);
            tmp = self.my_read(from);
        }
        tmp
    }

    /// C++: `Position VissimSingleTypeParser::getPosition(std::istream &)` [protected]
    /// returns the 2d-position saved as next within the stream
    // Position
    // NIImporter_Vissim::VissimSingleTypeParser::getPosition(std::istream& from) {
    //     double x, y;
    //     from >> x; // type-checking is missing!
    //     from >> y; // type-checking is missing!
    //     return Position(x, y);
    // }
    #[allow(unused_variables)]
    pub fn get_position(&mut self, from: &mut std::io::Cursor<Vec<u8>> /* std::istream */) -> Position {
        let x = vissim_read_f64(from); // type-checking is missing!
        let y = vissim_read_f64(from); // type-checking is missing!
        Position::new(x, y)
    }

    /// C++: `std::vector<int> VissimSingleTypeParser::parseAssignedVehicleTypes(std::istream &, const std::string &)` [protected]
    /// parses a listof vehicle types assigned to the current data field One should remeber, that -1 means "all" vehicle types
    // std::vector<int>
    // NIImporter_Vissim::VissimSingleTypeParser::parseAssignedVehicleTypes(
    //     std::istream& from, const std::string& next) {
    //     std::string tmp = readEndSecure(from);
    //     std::vector<int> ret;
    //     if (tmp == "alle") {
    //         ret.push_back(-1);
    //         return ret;
    //     }
    //     while (tmp != "DATAEND" && tmp != next) {
    //         ret.push_back(StringUtils::toInt(tmp));
    //         tmp = readEndSecure(from);
    //     }
    //     return ret;
    // }
    #[allow(unused_variables)]
    pub fn parse_assigned_vehicle_types(&mut self, from: &mut std::io::Cursor<Vec<u8>> /* std::istream */, next: &str) -> Vec<i32> {
        let mut tmp = self.read_end_secure(from, "");
        let mut ret: Vec<i32> = Vec::new();
        if tmp == "alle" {
            ret.push(-1);
            return ret;
        }
        while tmp != "DATAEND" && tmp != next {
            ret.push(vissim_parse(to_int(&tmp)));
            tmp = self.read_end_secure(from, "");
        }
        ret
    }

    /// C++: `NIVissimExtendedEdgePoint VissimSingleTypeParser::readExtEdgePointDef(std::istream &)` [protected]
    // NIVissimExtendedEdgePoint
    // NIImporter_Vissim::VissimSingleTypeParser::readExtEdgePointDef(
    //     std::istream& from) {
    //     std::string tag;
    //     from >> tag; // "Strecke"
    //     int edgeid;
    //     from >> edgeid; // type-checking is missing!
    //     from >> tag; // "Spuren"
    //     std::vector<int> lanes;
    //     while (tag != "bei") {
    //         tag = readEndSecure(from);
    //         if (tag != "bei") {
    //             int lane = StringUtils::toInt(tag);
    //             lanes.push_back(lane - 1);
    //         }
    //     }
    //     double position;
    //     from >> position;
    //     std::vector<int> dummy;
    //     return NIVissimExtendedEdgePoint(edgeid, lanes, position, dummy);
    // }
    #[allow(unused_variables)]
    pub fn read_ext_edge_point_def(&mut self, from: &mut std::io::Cursor<Vec<u8>> /* std::istream */) -> NIVissimExtendedEdgePoint {
        let mut tag = vissim_read_token(from); // "Strecke"
        let edgeid = vissim_read_i32(from); // type-checking is missing!
        tag = vissim_read_token(from); // "Spuren"
        let mut lanes: Vec<i32> = Vec::new();
        while tag != "bei" {
            tag = self.read_end_secure(from, "");
            if tag != "bei" {
                let lane = vissim_parse(to_int(&tag));
                lanes.push(lane - 1);
            }
        }
        let position = vissim_read_f64(from);
        let dummy: Vec<i32> = Vec::new();
        NIVissimExtendedEdgePoint::new(edgeid, &lanes, position, &dummy)
    }

    /// C++: `std::string VissimSingleTypeParser::readName(std::istream &)` [protected]
    /// Reads the structures name We cannot use the "<<" operator, as names may contain more than one word which are joined using '"'.
    // std::string
    // NIImporter_Vissim::VissimSingleTypeParser::readName(std::istream& from) {
    //     std::string name;
    //     from >> name;
    //     if (name[0] == '"') {
    //         while (name[name.length() - 1] != '"') {
    //             std::string tmp;
    //             from >> tmp;
    //             name = name + " " + tmp;
    //         }
    //         name = name.substr(1, name.length() - 2);
    //     }
    //     return StringUtils::convertUmlaute(name);
    // }
    #[allow(unused_variables)]
    pub fn read_name(&mut self, from: &mut std::io::Cursor<Vec<u8>> /* std::istream */) -> String {
        let mut name = vissim_read_token(from);
        if name.starts_with('"') {
            while !name.ends_with('"') {
                // shortcut: C++ loops forever on an unterminated quoted name at end of file, here the loop stops at end of file
                if !vissim_good(from) {
                    break;
                }
                let tmp = vissim_read_token(from);
                name = format!("{} {}", name, tmp);
            }
            name = if name.len() >= 2 { name[1..name.len() - 1].to_string() } else { String::new() };
        }
        convert_umlaute(&name)
    }

    /// C++: `bool VissimSingleTypeParser::skipOverreading(std::istream &, const std::string &)` [protected]
    /// Overreads the named parameter (if) given and skips the rest until "DATAEND"
    // bool
    // NIImporter_Vissim::VissimSingleTypeParser::skipOverreading(std::istream& from,
    //         const std::string& name) {
    //     std::string tag;
    //     while (tag != name) {
    //         tag = myRead(from);
    //     }
    //     while (tag != "DATAEND") {
    //         tag = readEndSecure(from);
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn skip_overreading(&mut self, from: &mut std::io::Cursor<Vec<u8>> /* std::istream */, name: &str) -> bool {
        let mut tag = String::new();
        while tag != name {
            tag = self.my_read(from);
            // shortcut: C++ loops forever when the name never comes, here the loop stops at end of file
            if tag != name && !vissim_good(from) {
                break;
            }
        }
        while tag != "DATAEND" {
            tag = self.read_end_secure(from, "");
        }
        true
    }

    /// C++: `void VissimSingleTypeParser::readUntil(std::istream &, const std::string &)` [protected]
    /// Reads from the stream until the keywor occurs
    // void
    // NIImporter_Vissim::VissimSingleTypeParser::readUntil(std::istream& from,
    //         const std::string& name) {
    //     std::string tag;
    //     while (tag != name) {
    //         tag = myRead(from);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn read_until(&mut self, from: &mut std::io::Cursor<Vec<u8>> /* std::istream */, name: &str) {
        let mut tag = String::new();
        while tag != name {
            tag = self.my_read(from);
            // shortcut: C++ loops forever when the name never comes, here the loop stops at end of file
            if tag != name && !vissim_good(from) {
                break;
            }
        }
    }

    /// C++: `VissimSingleTypeParser & VissimSingleTypeParser::operator=(const VissimSingleTypeParser &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &NIImporter_Vissim_VissimSingleTypeParser) -> &mut NIImporter_Vissim_VissimSingleTypeParser {
        panic!("VissimSingleTypeParser assignment operator is private and invalidated in C++")
    }

}

