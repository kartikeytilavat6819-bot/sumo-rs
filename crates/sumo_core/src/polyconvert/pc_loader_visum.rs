//! Port of polyconvert/PCLoaderVisum.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

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
        todo!("port PCLoaderVisum::loadIfSet")
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
        todo!("port PCLoaderVisum::load")
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
        todo!("port PCLoaderVisum::loadLanguage")
    }

}

