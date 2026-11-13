//! Port of polyconvert/PCLoaderVisum.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use self::PCLoaderVisum_VISUM_KEY as K;
use utils::common::file_helpers::is_readable;
use utils::common::msg_handler::{progress_begin_message, progress_done_message, write_warning};
use utils::common::rgb_color::RGBColor;
use utils::common::string_bijection::StringBijection;
use utils::common::string_tokenizer::StringTokenizer;
use utils::common::string_utils::{to_double, to_int, to_long};
use utils::geom::geo_conv_helper::GeoConvHelper;
use utils::importio::line_reader::LineReader;
use utils::importio::named_columns_parser::NamedColumnsParser;

/// C++ enum `PCLoaderVisum::VISUM_KEY`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum PCLoaderVisum_VISUM_KEY {
    VISUM_SYS = 0,
    VISUM_LINKTYPE = 1,
    VISUM_NODE = 2,
    VISUM_DISTRICT = 3,
    VISUM_POINT = 4,
    VISUM_LINK = 5,
    VISUM_V0 = 6,
    VISUM_TYPES = 7,
    VISUM_RANK = 8,
    VISUM_CAPACITY = 9,
    VISUM_XCOORD = 10,
    VISUM_YCOORD = 11,
    VISUM_FROMNODE = 12,
    VISUM_TONODE = 13,
    VISUM_TYPE = 14,
    VISUM_TYP = 15,
    VISUM_ID = 16,
    VISUM_CODE = 17,
    VISUM_DISTRICT_CONNECTION = 18,
    VISUM_SOURCE_DISTRICT = 19,
    VISUM_FROMNODENO = 20,
    VISUM_DIRECTION = 21,
    VISUM_SURFACEID = 22,
    VISUM_FACEID = 23,
    VISUM_FROMPOINTID = 24,
    VISUM_TOPOINTID = 25,
    VISUM_EDGE = 26,
    VISUM_VIANODENO = 27,
    VISUM_NUMLANES = 28,
    VISUM_TURN = 29,
    VISUM_INDEX = 30,
    VISUM_LINKPOLY = 31,
    VISUM_SURFACEITEM = 32,
    VISUM_FACEITEM = 33,
    VISUM_EDGEID = 34,
    VISUM_ORIGIN = 35,
    VISUM_DESTINATION = 36,
    VISUM_CATID = 37,
    VISUM_EDGEITEM = 38,
    VISUM_POICATEGORY = 39,
    VISUM_NETWORK = 40,
    VISUM_PROJECTIONDEFINITION = 41,
    VISUM_NO = 42,
}

/// A reader of pois and polygons stored in VISUM-format
/// C++ class `PCLoaderVisum` (polyconvert/PCLoaderVisum.h)
pub struct PCLoaderVisum {
}

// C++ static members of PCLoaderVisum (port as module-level statics, see AGENTS.md):
//   static StringBijection<VISUM_KEY>::Entry[] PCLoaderVisum::KEYS_DE;
//   static StringBijection<VISUM_KEY> PCLoaderVisum::KEYS;

/// C++ `PCLoaderVisum::KEYS_DE` (duplicates NIImporter_VISUM::KEYS_DE; the last entry is the terminator)
const KEYS_DE: &[(&str, PCLoaderVisum_VISUM_KEY)] = &[
    ("VSYS", K::VISUM_SYS),
    ("STRECKENTYP", K::VISUM_LINKTYPE),
    ("KNOTEN", K::VISUM_NODE),
    ("BEZIRK", K::VISUM_DISTRICT),
    ("PUNKT", K::VISUM_POINT),
    ("STRECKE", K::VISUM_LINK),
    ("V0IV", K::VISUM_V0),
    ("VSYSSET", K::VISUM_TYPES),
    ("RANG", K::VISUM_RANK),
    ("KAPIV", K::VISUM_CAPACITY),
    ("XKOORD", K::VISUM_XCOORD),
    ("YKOORD", K::VISUM_YCOORD),
    ("ID", K::VISUM_ID),
    ("CODE", K::VISUM_CODE),
    ("VONKNOTNR", K::VISUM_FROMNODE),
    ("NACHKNOTNR", K::VISUM_TONODE),
    ("TYPNR", K::VISUM_TYPE),
    ("TYP", K::VISUM_TYP),
    ("ANBINDUNG", K::VISUM_DISTRICT_CONNECTION),
    ("BEZNR", K::VISUM_SOURCE_DISTRICT),
    ("KNOTNR", K::VISUM_FROMNODENO),
    ("RICHTUNG", K::VISUM_DIRECTION),
    ("FLAECHEID", K::VISUM_SURFACEID),
    ("TFLAECHEID", K::VISUM_FACEID),
    ("VONPUNKTID", K::VISUM_FROMPOINTID),
    ("NACHPUNKTID", K::VISUM_TOPOINTID),
    ("KANTE", K::VISUM_EDGE),
    ("ABBIEGER", K::VISUM_TURN),
    ("UEBERKNOTNR", K::VISUM_VIANODENO),
    ("ANZFAHRSTREIFEN", K::VISUM_NUMLANES),
    ("INDEX", K::VISUM_INDEX),
    ("STRECKENPOLY", K::VISUM_LINKPOLY),
    ("FLAECHENELEMENT", K::VISUM_SURFACEITEM),
    ("TEILFLAECHENELEMENT", K::VISUM_FACEITEM),
    ("KANTEID", K::VISUM_EDGEID),
    ("Q", K::VISUM_ORIGIN),
    ("Z", K::VISUM_DESTINATION),
    ("KATNR", K::VISUM_CATID),
    ("ZWISCHENPUNKT", K::VISUM_EDGEITEM),
    ("POIKATEGORIE", K::VISUM_POICATEGORY),
    ("NETZ", K::VISUM_NETWORK),
    ("DEFKOORD", K::VISUM_PROJECTIONDEFINITION),
    ("NR", K::VISUM_NO), // must be the last one
];

thread_local! {
    // C++ PCLoaderVisum::KEYS (the terminator entry is inserted too, as the C++ constructor does)
    static KEYS: std::cell::RefCell<StringBijection<PCLoaderVisum_VISUM_KEY>> =
        std::cell::RefCell::new(StringBijection::new(KEYS_DE, true).unwrap_or_else(|e| panic!("{e}")));
}

/// C++ KEYS.getString(key)
fn keys_string(key: PCLoaderVisum_VISUM_KEY) -> String {
    KEYS.with(|k| or_panic(k.borrow().get_string(key).map(|s| s.to_string())))
}

/// C++ `lineParser.get(KEYS.getString(key))`
fn parser_get(p: &NamedColumnsParser, key: PCLoaderVisum_VISUM_KEY) -> String {
    or_panic(p.get(&keys_string(key), false))
}

/// C++ `atof`: the longest leading number, 0 when there is none
fn atof(s: &str) -> f64 {
    let t = s.trim_start();
    (0..=t.len())
        .rev()
        .filter(|&n| t.is_char_boundary(n))
        .find_map(|n| t[..n].parse::<f64>().ok())
        .unwrap_or(0.0)
}

impl PCLoaderVisum {
    /// C++: `void PCLoaderVisum::loadIfSet(OptionsCont &, PCPolyContainer &, PCTypeMap &)` [static]
    /// Loads pois/polygons assumed to be stored using VISUM-format
    // void
    // PCLoaderVisum::loadIfSet(OptionsCont& oc, PCPolyContainer& toFill,
    //                          PCTypeMap& tm) {
    //     if (!oc.isSet("visum-files")) {
    //         return;
    //     }
    //     const std::string languageFile = oc.getString("visum.language-file");
    //     if (languageFile != "") {
    //         loadLanguage(languageFile);
    //     }
    //     // parse file(s)
    //     std::vector<std::string> files = oc.getStringVector("visum-files");
    //     for (std::vector<std::string>::const_iterator file = files.begin(); file != files.end(); ++file) {
    //         if (!FileHelpers::isReadable(*file)) {
    //             throw ProcessError(TLF("Could not open visum-file '%'.", *file));
    //         }
    //         PROGRESS_BEGIN_MESSAGE("Parsing from visum-file '" + *file + "'");
    //         load(*file, oc, toFill, tm);
    //         PROGRESS_DONE_MESSAGE();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn load_if_set(oc: &mut OptionsCont, toFill: &mut PCPolyContainer, tm: &mut PCTypeMap) {
        if !or_panic(oc.is_set("visum-files", true)) {
            return;
        }
        let languageFile = or_panic(oc.get_string("visum.language-file"));
        if languageFile != "" {
            Self::load_language(&languageFile);
        }
        // parse file(s)
        let files = or_panic(oc.get_string_vector("visum-files")).clone();
        for file in files.iter() {
            if !is_readable(file) {
                panic!("{}", utils::tlf!("Could not open visum-file '%'.", file));
            }
            progress_begin_message(&format!("Parsing from visum-file '{}'", file));
            Self::load(file, oc, toFill, tm);
            progress_done_message();
        }
    }

    /// C++: `void PCLoaderVisum::load(const std::string &, OptionsCont &, PCPolyContainer &, PCTypeMap &)` [static, protected]
    /// Parses pois/polys stored within the given file
    // void
    // PCLoaderVisum::load(const std::string& file, OptionsCont& oc, PCPolyContainer& toFill,
    //                     PCTypeMap& tm) {
    //     GeoConvHelper& geoConvHelper = GeoConvHelper::getProcessing();
    //     std::string what;
    //     std::map<long long int, Position> punkte;
    //     std::map<long long int, PositionVector> kanten;
    //     std::map<long long int, PositionVector> teilflaechen;
    //     std::map<long long int, long long int> flaechenelemente;
    //     NamedColumnsParser lineParser;
    //     LineReader lr(file);
    //     while (lr.hasMore()) {
    //         std::string line = lr.readLine();
    //         // reset if current is over
    //         if (line.length() == 0 || line[0] == '*' || line[0] == '$') {
    //             what = "";
    //         }
    //         // read items
    //         if (what == "$" + KEYS.getString(VISUM_POINT)) {
    //             lineParser.parseLine(line);
    //             long long int id = StringUtils::toLong(lineParser.get(KEYS.getString(VISUM_ID)));
    //             double x = StringUtils::toDouble(lineParser.get(KEYS.getString(VISUM_XCOORD)));
    //             double y = StringUtils::toDouble(lineParser.get(KEYS.getString(VISUM_YCOORD)));
    //             Position pos(x, y);
    //             if (!geoConvHelper.x2cartesian(pos)) {
    //                 WRITE_WARNINGF(TL("Unable to project coordinates for point '%'."), toString(id));
    //             }
    //             punkte[id] = pos;
    //             continue;
    //         } else if (what == "$" + KEYS.getString(VISUM_EDGE)) {
    //             lineParser.parseLine(line);
    //             long long int id = StringUtils::toLong(lineParser.get(KEYS.getString(VISUM_ID)));
    //             long long int fromID = StringUtils::toLong(lineParser.get(KEYS.getString(VISUM_FROMPOINTID)));
    //             long long int toID = StringUtils::toLong(lineParser.get(KEYS.getString(VISUM_TOPOINTID)));
    //             PositionVector vec;
    //             vec.push_back(punkte[fromID]);
    //             vec.push_back(punkte[toID]);
    //             kanten[id] = vec;
    //             continue;
    //         } else if (what == "$" + KEYS.getString(VISUM_EDGEITEM)) {
    //             lineParser.parseLine(line);
    //             long long int id = StringUtils::toLong(lineParser.get(KEYS.getString(VISUM_EDGEID)));
    //             int index = StringUtils::toInt(lineParser.get(KEYS.getString(VISUM_INDEX)));
    //             double x = StringUtils::toDouble(lineParser.get(KEYS.getString(VISUM_XCOORD)));
    //             double y = StringUtils::toDouble(lineParser.get(KEYS.getString(VISUM_YCOORD)));
    //             Position pos(x, y);
    //             if (!geoConvHelper.x2cartesian(pos)) {
    //                 WRITE_WARNINGF(TL("Unable to project coordinates for edge '%'."), toString(id));
    //             }
    //             kanten[id].insert(kanten[id].begin() + index, pos);
    //             continue;
    //         } else if (what == "$" + KEYS.getString(VISUM_FACEITEM)) {
    //             lineParser.parseLine(line);
    //             long long int id = StringUtils::toLong(lineParser.get(KEYS.getString(VISUM_FACEID)));
    //             //int index = StringUtils::toInt(lineParser.get("INDEX"));
    //             //index = 0; /// hmmmm - assume it's sorted...
    //             long long int kid = StringUtils::toLong(lineParser.get(KEYS.getString(VISUM_EDGEID)));
    //             int dir = StringUtils::toInt(lineParser.get(KEYS.getString(VISUM_DIRECTION)));
    //             if (teilflaechen.find(id) == teilflaechen.end()) {
    //                 teilflaechen[id] = PositionVector();
    //             }
    //             if (dir == 0) {
    //                 for (int i = 0; i < (int) kanten[kid].size(); ++i) {
    //                     teilflaechen[id].push_back_noDoublePos(kanten[kid][i]);
    //                 }
    //             } else {
    //                 for (int i = (int) kanten[kid].size() - 1; i >= 0; --i) {
    //                     teilflaechen[id].push_back_noDoublePos(kanten[kid][i]);
    //                 }
    //             }
    //             continue;
    //         } else if (what == "$" + KEYS.getString(VISUM_SURFACEITEM)) {
    //             lineParser.parseLine(line);
    //             long long int id = StringUtils::toLong(lineParser.get(KEYS.getString(VISUM_SURFACEID)));
    //             long long int tid = StringUtils::toLong(lineParser.get(KEYS.getString(VISUM_FACEID)));
    //             flaechenelemente[id] = tid;
    //             continue;
    //         }
    //         // set if read
    //         if (line[0] == '$') {
    //             what = "";
    //             if (line.find("$" + KEYS.getString(VISUM_POINT) + ":") == 0) {
    //                 what = "$" + KEYS.getString(VISUM_POINT);
    //             } else if (line.find("$" + KEYS.getString(VISUM_EDGE) + ":") == 0) {
    //                 what = "$" + KEYS.getString(VISUM_EDGE);
    //             } else if (line.find("$" + KEYS.getString(VISUM_EDGEITEM) + ":") == 0) {
    //                 what = "$" + KEYS.getString(VISUM_EDGEITEM);
    //             } else if (line.find("$" + KEYS.getString(VISUM_FACEITEM) + ":") == 0) {
    //                 what = "$" + KEYS.getString(VISUM_FACEITEM);
    //             } else if (line.find("$" + KEYS.getString(VISUM_SURFACEITEM) + ":") == 0) {
    //                 what = "$" + KEYS.getString(VISUM_SURFACEITEM);
    //             }
    //             if (what != "") {
    //                 lineParser.reinit(line.substr(what.length() + 1));
    //             }
    //         }
    //     }
    //
    //     // do some more sane job...
    //     RGBColor c = RGBColor::parseColor(oc.getString("color"));
    //     std::map<std::string, std::string> typemap;
    //     // load the pois/polys
    //     lr.reinit();
    //     bool parsingCategories = false;
    //     bool parsingPOIs = false;
    //     bool parsingDistrictsDirectly = false;
    //     PositionVector vec;
    //     std::string polyType, lastID;
    //     bool first = true;
    //     while (lr.hasMore()) {
    //         std::string line = lr.readLine();
    //         // do not parse empty lines
    //         if (line.length() == 0) {
    //             continue;
    //         }
    //         // do not parse comment lines
    //         if (line[0] == '*') {
    //             continue;
    //         }
    //
    //         if (line[0] == '$') {
    //             // reset parsing on new entry type
    //             parsingCategories = false;
    //             parsingPOIs = false;
    //             parsingDistrictsDirectly = false;
    //             polyType = "";
    //         }
    //
    //         if (parsingCategories) {
    //             // parse the category
    //             StringTokenizer st(line, ";");
    //             std::string catid = st.next();
    //             std::string catname = st.next();
    //             typemap[catid] = catname;
    //         }
    //         if (parsingPOIs) {
    //             // parse the poi
    //             // $POI:Nr;CATID;CODE;NAME;Kommentar;XKoord;YKoord;
    //             lineParser.parseLine(line);
    //             long long int idL = StringUtils::toLong(lineParser.get(KEYS.getString(VISUM_NO)));
    //             std::string id = toString(idL);
    //             std::string catid = lineParser.get(KEYS.getString(VISUM_CATID));
    //             // process read values
    //             double x = StringUtils::toDouble(lineParser.get(KEYS.getString(VISUM_XCOORD)));
    //             double y = StringUtils::toDouble(lineParser.get(KEYS.getString(VISUM_YCOORD)));
    //             Position pos(x, y);
    //             if (!geoConvHelper.x2cartesian(pos)) {
    //                 WRITE_WARNINGF(TL("Unable to project coordinates for POI '%'."), id);
    //             }
    //             std::string type = typemap[catid];
    //             // patch the values
    //             bool discard = oc.getBool("discard");
    //             std::string icon = oc.getString("icon");
    //             double layer = oc.getFloat("layer");
    //             RGBColor color;
    //             if (tm.has(type)) {
    //                 const PCTypeMap::TypeDef& def = tm.get(type);
    //                 id = def.prefix + id;
    //                 type = def.id;
    //                 color = def.color;
    //                 discard = def.discard;
    //                 icon = def.icon;
    //                 layer = def.layer;
    //             } else {
    //                 id = oc.getString("prefix") + id;
    //                 color = c;
    //             }
    //             if (!discard) {
    //                 const std::string origId = id;
    //                 int index = 1;
    //                 while (toFill.getPOIs().get(id) != nullptr) {
    //                     id = origId + "#" + toString(index++);
    //                 }
    //                 PointOfInterest* poi = new PointOfInterest(id, type, color, pos, false, "", 0, false, 0, icon, layer);
    //                 toFill.add(poi);
    //             }
    //         }
    //
    //         // poly
    //         if (polyType != "") {
    //             StringTokenizer st(line, ";");
    //             std::string id = st.next();
    //             std::string type;
    //             if (!first && lastID != id) {
    //                 // we have parsed a polygon completely
    //                 RGBColor color;
    //                 double layer = oc.getFloat("layer");
    //                 bool discard = oc.getBool("discard");
    //                 if (tm.has(polyType)) {
    //                     const PCTypeMap::TypeDef& def = tm.get(polyType);
    //                     id = def.prefix + id;
    //                     type = def.id;
    //                     color = def.color;
    //                     discard = def.discard;
    //                     layer = def.layer;
    //                 } else {
    //                     id = oc.getString("prefix") + id;
    //                     type = oc.getString("type");
    //                     color = c;
    //                 }
    //                 if (!discard) {
    //                     const std::string origId = id;
    //                     int index = 1;
    //                     while (toFill.getPolygons().get(id) != nullptr) {
    //                         id = origId + "#" + toString(index++);
    //                     }
    //                     SUMOPolygon* poly = new SUMOPolygon(id, type, color, vec, false, false, 1, layer);
    //                     toFill.add(poly);
    //                 }
    //                 vec.clear();
    //             }
    //             lastID = id;
    //             first = false;
    //             // parse current poly
    //             std::string index = st.next();
    //             std::string xpos = st.next();
    //             std::string ypos = st.next();
    //             Position pos2D((double) atof(xpos.c_str()), (double) atof(ypos.c_str()));
    //             if (!geoConvHelper.x2cartesian(pos2D)) {
    //                 WRITE_WARNINGF(TL("Unable to project coordinates for polygon '%'."), id);
    //             }
    //             vec.push_back(pos2D);
    //         }
    //
    //         // district refering a shape
    //         if (parsingDistrictsDirectly) {
    //             //$BEZIRK:NR	CODE	NAME	TYPNR	XKOORD	YKOORD	FLAECHEID	BEZART	IVANTEIL_Q	IVANTEIL_Z	OEVANTEIL	METHODEANBANTEILE	ZWERT1	ZWERT2	ZWERT3	ISTINAUSWAHL	OBEZNR	NOM_COM	COD_COM
    //             lineParser.parseLine(line);
    //             long long int idL = StringUtils::toLong(lineParser.get(KEYS.getString(VISUM_NO)));
    //             std::string id = toString(idL);
    //             long long int area = StringUtils::toLong(lineParser.get(KEYS.getString(VISUM_SURFACEID)));
    //             double x = StringUtils::toDouble(lineParser.get(KEYS.getString(VISUM_XCOORD)));
    //             double y = StringUtils::toDouble(lineParser.get(KEYS.getString(VISUM_YCOORD)));
    //             // patch the values
    //             std::string type = "district";
    //             bool discard = oc.getBool("discard");
    //             std::string icon = oc.getString("icon");
    //             double layer = oc.getFloat("layer");
    //             RGBColor color;
    //             if (tm.has(type)) {
    //                 const PCTypeMap::TypeDef& def = tm.get(type);
    //                 id = def.prefix + id;
    //                 type = def.id;
    //                 color = def.color;
    //                 discard = def.discard;
    //                 icon = def.icon;
    //                 layer = def.layer;
    //             } else {
    //                 id = oc.getString("prefix") + id;
    //                 type = oc.getString("type");
    //                 color = c;
    //             }
    //             if (!discard) {
    //                 if (teilflaechen[flaechenelemente[area]].size() > 0) {
    //                     const std::string origId = id;
    //                     int index = 1;
    //                     while (toFill.getPolygons().get(id) != nullptr) {
    //                         id = origId + "#" + toString(index++);
    //                     }
    //                     const auto shape = teilflaechen[flaechenelemente[area]];
    //                     SUMOPolygon* poly = new SUMOPolygon(id, type, color, shape, false, false, 1, layer);
    //                     toFill.add(poly);
    //                 } else {
    //                     Position pos(x, y);
    //                     if (!geoConvHelper.x2cartesian(pos)) {
    //                         WRITE_WARNINGF(TL("Unable to project coordinates for POI '%'."), id);
    //                     }
    //                     const std::string origId = id;
    //                     int index = 1;
    //                     while (toFill.getPOIs().get(id) != nullptr) {
    //                         id = origId + "#" + toString(index++);
    //                     }
    //                     PointOfInterest* poi = new PointOfInterest(id, type, color, pos, false, "", 0, false, 0, icon, layer);
    //                     toFill.add(poi);
    //                 }
    //             }
    //         }
    //
    //
    //         if (line.find("$POIKATEGORIEDEF:") == 0 || line.find("$" + KEYS.getString(VISUM_POICATEGORY) + ":") == 0) {
    //             // ok, got categories, begin parsing from next line
    //             parsingCategories = true;
    //             lineParser.reinit(line.substr(line.find(":") + 1));
    //         }
    //         if ((line.find("$POI:") == 0) || line.find("$POIOFCAT") != std::string::npos) {
    //             // ok, got pois, begin parsing from next line
    //             parsingPOIs = true;
    //             lineParser.reinit(line.substr(line.find(":") + 1));
    //         }
    //         if (line.find("$" + KEYS.getString(VISUM_DISTRICT)) == 0 && line.find(KEYS.getString(VISUM_SURFACEID)) != std::string::npos) {
    //             // ok, have a district header, and it seems like districts would reference shapes...
    //             parsingDistrictsDirectly = true;
    //             lineParser.reinit(line.substr(line.find(":") + 1));
    //         }
    //
    //
    //         if (line.find("$BEZIRKPOLY") != std::string::npos) {
    //             polyType = "district";
    //         }
    //         if (line.find("$GEBIETPOLY") != std::string::npos) {
    //             polyType = "area";
    //         }
    //
    //     }
    // }
    #[allow(unused_variables)]
    pub fn load(file: &str, oc: &mut OptionsCont, toFill: &mut PCPolyContainer, tm: &mut PCTypeMap) {
        let mut what = String::new();
        let mut punkte: BTreeMap<i64, Position> = BTreeMap::new();
        let mut kanten: BTreeMap<i64, PositionVector> = BTreeMap::new();
        let mut teilflaechen: BTreeMap<i64, PositionVector> = BTreeMap::new();
        let mut flaechenelemente: BTreeMap<i64, i64> = BTreeMap::new();
        let mut lineParser = NamedColumnsParser::new();
        let mut lr = LineReader::with_file(file);
        while lr.has_more() {
            let line = lr.read_line();
            // reset if current is over
            if line.is_empty() || line.starts_with('*') || line.starts_with('$') {
                what = String::new();
            }
            // read items
            if what == format!("${}", keys_string(K::VISUM_POINT)) {
                lineParser.parse_line(&line);
                let id = or_panic(to_long(&parser_get(&lineParser, K::VISUM_ID)));
                let x = or_panic(to_double(&parser_get(&lineParser, K::VISUM_XCOORD)));
                let y = or_panic(to_double(&parser_get(&lineParser, K::VISUM_YCOORD)));
                let mut pos = Position::new(x, y);
                if !GeoConvHelper::with_processing(|g| g.x2cartesian(&mut pos, true)) {
                    write_warning(&format!("Unable to project coordinates for point '{}'.", id));
                }
                punkte.insert(id, pos);
                continue;
            } else if what == format!("${}", keys_string(K::VISUM_EDGE)) {
                lineParser.parse_line(&line);
                let id = or_panic(to_long(&parser_get(&lineParser, K::VISUM_ID)));
                let fromID = or_panic(to_long(&parser_get(&lineParser, K::VISUM_FROMPOINTID)));
                let toID = or_panic(to_long(&parser_get(&lineParser, K::VISUM_TOPOINTID)));
                let mut vec = PositionVector::default();
                vec.0.push(*punkte.entry(fromID).or_default());
                vec.0.push(*punkte.entry(toID).or_default());
                kanten.insert(id, vec);
                continue;
            } else if what == format!("${}", keys_string(K::VISUM_EDGEITEM)) {
                lineParser.parse_line(&line);
                let id = or_panic(to_long(&parser_get(&lineParser, K::VISUM_EDGEID)));
                let index = or_panic(to_int(&parser_get(&lineParser, K::VISUM_INDEX)));
                let x = or_panic(to_double(&parser_get(&lineParser, K::VISUM_XCOORD)));
                let y = or_panic(to_double(&parser_get(&lineParser, K::VISUM_YCOORD)));
                let mut pos = Position::new(x, y);
                if !GeoConvHelper::with_processing(|g| g.x2cartesian(&mut pos, true)) {
                    write_warning(&format!("Unable to project coordinates for edge '{}'.", id));
                }
                kanten.entry(id).or_default().0.insert(index as usize, pos);
                continue;
            } else if what == format!("${}", keys_string(K::VISUM_FACEITEM)) {
                lineParser.parse_line(&line);
                let id = or_panic(to_long(&parser_get(&lineParser, K::VISUM_FACEID)));
                let kid = or_panic(to_long(&parser_get(&lineParser, K::VISUM_EDGEID)));
                let dir = or_panic(to_int(&parser_get(&lineParser, K::VISUM_DIRECTION)));
                teilflaechen.entry(id).or_default();
                let kv: Vec<Position> = kanten.entry(kid).or_default().0.clone();
                let tf = teilflaechen.entry(id).or_default();
                if dir == 0 {
                    for p in kv.iter() {
                        tf.push_back_no_double_pos(p);
                    }
                } else {
                    for p in kv.iter().rev() {
                        tf.push_back_no_double_pos(p);
                    }
                }
                continue;
            } else if what == format!("${}", keys_string(K::VISUM_SURFACEITEM)) {
                lineParser.parse_line(&line);
                let id = or_panic(to_long(&parser_get(&lineParser, K::VISUM_SURFACEID)));
                let tid = or_panic(to_long(&parser_get(&lineParser, K::VISUM_FACEID)));
                flaechenelemente.insert(id, tid);
                continue;
            }
            // set if read
            if line.starts_with('$') {
                what = String::new();
                if line.starts_with(&format!("${}:", keys_string(K::VISUM_POINT))) {
                    what = format!("${}", keys_string(K::VISUM_POINT));
                } else if line.starts_with(&format!("${}:", keys_string(K::VISUM_EDGE))) {
                    what = format!("${}", keys_string(K::VISUM_EDGE));
                } else if line.starts_with(&format!("${}:", keys_string(K::VISUM_EDGEITEM))) {
                    what = format!("${}", keys_string(K::VISUM_EDGEITEM));
                } else if line.starts_with(&format!("${}:", keys_string(K::VISUM_FACEITEM))) {
                    what = format!("${}", keys_string(K::VISUM_FACEITEM));
                } else if line.starts_with(&format!("${}:", keys_string(K::VISUM_SURFACEITEM))) {
                    what = format!("${}", keys_string(K::VISUM_SURFACEITEM));
                }
                if what != "" {
                    lineParser.reinit(&line[what.len() + 1..], ";", ";", false, true);
                }
            }
        }

        // do some more sane job...
        let c: RGBColor = or_panic(RGBColor::parse_color(&or_panic(oc.get_string("color"))));
        let mut typemap: BTreeMap<String, String> = BTreeMap::new();
        // load the pois/polys
        lr.reinit();
        let mut parsingCategories = false;
        let mut parsingPOIs = false;
        let mut parsingDistrictsDirectly = false;
        let mut vec = PositionVector::default();
        let mut polyType = String::new();
        let mut lastID = String::new();
        let mut first = true;
        while lr.has_more() {
            let line = lr.read_line();
            // do not parse empty lines
            if line.is_empty() {
                continue;
            }
            // do not parse comment lines
            if line.starts_with('*') {
                continue;
            }

            if line.starts_with('$') {
                // reset parsing on new entry type
                parsingCategories = false;
                parsingPOIs = false;
                parsingDistrictsDirectly = false;
                polyType = String::new();
            }

            if parsingCategories {
                // parse the category
                let mut st = StringTokenizer::with_token(&line, ";", false);
                let catid = or_panic(st.next());
                let catname = or_panic(st.next());
                typemap.insert(catid, catname);
            }
            if parsingPOIs {
                // parse the poi
                // $POI:Nr;CATID;CODE;NAME;Kommentar;XKoord;YKoord;
                lineParser.parse_line(&line);
                let idL = or_panic(to_long(&parser_get(&lineParser, K::VISUM_NO)));
                let mut id = idL.to_string();
                let catid = parser_get(&lineParser, K::VISUM_CATID);
                // process read values
                let x = or_panic(to_double(&parser_get(&lineParser, K::VISUM_XCOORD)));
                let y = or_panic(to_double(&parser_get(&lineParser, K::VISUM_YCOORD)));
                let mut pos = Position::new(x, y);
                if !GeoConvHelper::with_processing(|g| g.x2cartesian(&mut pos, true)) {
                    write_warning(&format!("Unable to project coordinates for POI '{}'.", id));
                }
                let mut ty = typemap.get(&catid).cloned().unwrap_or_default();
                // patch the values
                let mut discard = or_panic(oc.get_bool("discard"));
                let mut icon = or_panic(oc.get_string("icon"));
                let mut layer = or_panic(oc.get_float("layer"));
                let mut color: RGBColor = RGBColor::default();
                if tm.has(&ty) {
                    let def = tm.get(&ty);
                    id = def.prefix.clone() + &id;
                    ty = def.id.clone();
                    color = def.color;
                    discard = def.discard;
                    icon = def.icon.clone();
                    layer = def.layer;
                } else {
                    id = or_panic(oc.get_string("prefix")) + &id;
                    color = c;
                }
                if !discard {
                    let origId = id.clone();
                    let mut index = 1;
                    while toFill.base.get_po_is().contains_key(&id) {
                        id = origId.clone() + "#" + &index.to_string();
                        index += 1;
                    }
                    let poi: Ptr<PointOfInterest> = Ptr::from_raw(Box::into_raw(Box::new(PointOfInterest::new(&id, &ty, &color, &pos, false, "", 0.0, false, 0.0, &icon, layer, DEFAULT_ANGLE, DEFAULT_IMG_FILE, DEFAULT_IMG_WIDTH, DEFAULT_IMG_HEIGHT, DEFAULT_NAME, &BTreeMap::new()))));
                    toFill.add_2(poi, false);
                }
            }

            // poly
            if polyType != "" {
                let mut st = StringTokenizer::with_token(&line, ";", false);
                let mut id = or_panic(st.next());
                let mut ty = String::new();
                if !first && lastID != id {
                    // we have parsed a polygon completely
                    let mut color: RGBColor = RGBColor::default();
                    let mut layer = or_panic(oc.get_float("layer"));
                    let mut discard = or_panic(oc.get_bool("discard"));
                    if tm.has(&polyType) {
                        let def = tm.get(&polyType);
                        id = def.prefix.clone() + &id;
                        ty = def.id.clone();
                        color = def.color;
                        discard = def.discard;
                        layer = def.layer;
                    } else {
                        id = or_panic(oc.get_string("prefix")) + &id;
                        ty = or_panic(oc.get_string("type"));
                        color = c;
                    }
                    if !discard {
                        let origId = id.clone();
                        let mut index = 1;
                        while toFill.base.get_polygons().contains_key(&id) {
                            id = origId.clone() + "#" + &index.to_string();
                            index += 1;
                        }
                        let poly: Ptr<SUMOPolygon> = Ptr::from_raw(Box::into_raw(Box::new(SUMOPolygon::new(&id, &ty, &color, &vec, false, false, 1.0, layer, DEFAULT_ANGLE, DEFAULT_IMG_FILE, DEFAULT_NAME, DEFAULT_HEIGHT, &BTreeMap::new()))));
                        toFill.add(poly, false);
                    }
                    vec.0.clear();
                }
                lastID = id.clone();
                first = false;
                // parse current poly
                let _index = or_panic(st.next());
                let xpos = or_panic(st.next());
                let ypos = or_panic(st.next());
                let mut pos2D = Position::new(atof(&xpos), atof(&ypos));
                if !GeoConvHelper::with_processing(|g| g.x2cartesian(&mut pos2D, true)) {
                    write_warning(&format!("Unable to project coordinates for polygon '{}'.", id));
                }
                vec.0.push(pos2D);
            }

            // district refering a shape
            if parsingDistrictsDirectly {
                //$BEZIRK:NR	CODE	NAME	TYPNR	XKOORD	YKOORD	FLAECHEID	BEZART	IVANTEIL_Q	IVANTEIL_Z	OEVANTEIL	METHODEANBANTEILE	ZWERT1	ZWERT2	ZWERT3	ISTINAUSWAHL	OBEZNR	NOM_COM	COD_COM
                lineParser.parse_line(&line);
                let idL = or_panic(to_long(&parser_get(&lineParser, K::VISUM_NO)));
                let mut id = idL.to_string();
                let area = or_panic(to_long(&parser_get(&lineParser, K::VISUM_SURFACEID)));
                let x = or_panic(to_double(&parser_get(&lineParser, K::VISUM_XCOORD)));
                let y = or_panic(to_double(&parser_get(&lineParser, K::VISUM_YCOORD)));
                // patch the values
                let mut ty = "district".to_string();
                let mut discard = or_panic(oc.get_bool("discard"));
                let mut icon = or_panic(oc.get_string("icon"));
                let mut layer = or_panic(oc.get_float("layer"));
                let mut color: RGBColor = RGBColor::default();
                if tm.has(&ty) {
                    let def = tm.get(&ty);
                    id = def.prefix.clone() + &id;
                    ty = def.id.clone();
                    color = def.color;
                    discard = def.discard;
                    icon = def.icon.clone();
                    layer = def.layer;
                } else {
                    id = or_panic(oc.get_string("prefix")) + &id;
                    ty = or_panic(oc.get_string("type"));
                    color = c;
                }
                if !discard {
                    let area_key = *flaechenelemente.entry(area).or_default();
                    if teilflaechen.entry(area_key).or_default().0.len() > 0 {
                        let origId = id.clone();
                        let mut index = 1;
                        while toFill.base.get_polygons().contains_key(&id) {
                            id = origId.clone() + "#" + &index.to_string();
                            index += 1;
                        }
                        let shape = teilflaechen.entry(area_key).or_default().clone();
                        let poly: Ptr<SUMOPolygon> = Ptr::from_raw(Box::into_raw(Box::new(SUMOPolygon::new(&id, &ty, &color, &shape, false, false, 1.0, layer, DEFAULT_ANGLE, DEFAULT_IMG_FILE, DEFAULT_NAME, DEFAULT_HEIGHT, &BTreeMap::new()))));
                        toFill.add(poly, false);
                    } else {
                        let mut pos = Position::new(x, y);
                        if !GeoConvHelper::with_processing(|g| g.x2cartesian(&mut pos, true)) {
                            write_warning(&format!("Unable to project coordinates for POI '{}'.", id));
                        }
                        let origId = id.clone();
                        let mut index = 1;
                        while toFill.base.get_po_is().contains_key(&id) {
                            id = origId.clone() + "#" + &index.to_string();
                            index += 1;
                        }
                        let poi: Ptr<PointOfInterest> = Ptr::from_raw(Box::into_raw(Box::new(PointOfInterest::new(&id, &ty, &color, &pos, false, "", 0.0, false, 0.0, &icon, layer, DEFAULT_ANGLE, DEFAULT_IMG_FILE, DEFAULT_IMG_WIDTH, DEFAULT_IMG_HEIGHT, DEFAULT_NAME, &BTreeMap::new()))));
                        toFill.add_2(poi, false);
                    }
                }
            }

            if line.starts_with("$POIKATEGORIEDEF:") || line.starts_with(&format!("${}:", keys_string(K::VISUM_POICATEGORY))) {
                // ok, got categories, begin parsing from next line
                parsingCategories = true;
                lineParser.reinit(&line[line.find(':').map_or(0, |i| i + 1)..], ";", ";", false, true);
            }
            if line.starts_with("$POI:") || line.contains("$POIOFCAT") {
                // ok, got pois, begin parsing from next line
                parsingPOIs = true;
                lineParser.reinit(&line[line.find(':').map_or(0, |i| i + 1)..], ";", ";", false, true);
            }
            if line.starts_with(&format!("${}", keys_string(K::VISUM_DISTRICT))) && line.contains(&keys_string(K::VISUM_SURFACEID)) {
                // ok, have a district header, and it seems like districts would reference shapes...
                parsingDistrictsDirectly = true;
                lineParser.reinit(&line[line.find(':').map_or(0, |i| i + 1)..], ";", ";", false, true);
            }

            if line.contains("$BEZIRKPOLY") {
                polyType = "district".to_string();
            }
            if line.contains("$GEBIETPOLY") {
                polyType = "area".to_string();
            }
        }
    }

    /// C++: `void PCLoaderVisum::loadLanguage(const std::string &)` [static, protected]
    // void
    // PCLoaderVisum::loadLanguage(const std::string& file) {
    //     std::ifstream strm(file.c_str());
    //     if (!strm.good()) {
    //         throw ProcessError(TLF("Could not load VISUM language map from '%'.", file));
    //     }
    //     while (strm.good()) {
    //         std::string keyDE;
    //         std::string keyNew;
    //         strm >> keyDE;
    //         strm >> keyNew;
    //         if (KEYS.hasString(keyDE)) {
    //             VISUM_KEY key = KEYS.get(keyDE);
    //             KEYS.remove(keyDE, key);
    //             KEYS.insert(keyNew, key);
    //         } else if (keyDE != "") {
    //             // do not warn about network-related keys (NIImporter_VISUM)
    //             //WRITE_WARNINGF(TL("Unknown entry '%' in VISUM language map"), keyDE);
    //         }
    //     }
    //
    // }
    #[allow(unused_variables)]
    pub fn load_language(file: &str) {
        let text = match std::fs::read(file) {
            Ok(bytes) => String::from_utf8_lossy(&bytes).into_owned(),
            Err(_) => panic!("{}", utils::tlf!("Could not load VISUM language map from '%'.", file)),
        };
        // C++ `strm >> keyDE; strm >> keyNew;` in a loop; a token that is missing reads as ""
        let tokens: Vec<&str> = text.split_whitespace().collect();
        let mut i = 0;
        while i < tokens.len() {
            let keyDE = tokens[i];
            let keyNew = tokens.get(i + 1).copied().unwrap_or("");
            i += 2;
            let has = KEYS.with(|k| k.borrow().has_string(keyDE));
            if has {
                KEYS.with(|k| {
                    let mut k = k.borrow_mut();
                    let key = or_panic(k.get(keyDE));
                    k.remove(keyDE, key);
                    or_panic(k.insert(keyNew, key, true));
                });
            }
            // do not warn about network-related keys (NIImporter_VISUM)
        }
    }

}

