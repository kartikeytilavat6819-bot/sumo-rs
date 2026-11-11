//! Port of polyconvert/PCLoaderDlrNavteq.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// A reader of pois and polygons stored in DLR-Navteq (Elmar)-format
/// C++ class `PCLoaderDlrNavteq` (polyconvert/PCLoaderDlrNavteq.h)
pub struct PCLoaderDlrNavteq {
}

impl PCLoaderDlrNavteq {
    /// C++: `void PCLoaderDlrNavteq::loadIfSet(OptionsCont &, PCPolyContainer &, PCTypeMap &)` [static]
    /// Loads pois/polygons assumed to be stored as according DLR-Navteq (Elmar)-files
    // void
    // PCLoaderDlrNavteq::loadIfSet(OptionsCont& oc, PCPolyContainer& toFill,
    //                              PCTypeMap& tm) {
    //     if (oc.isSet("dlr-navteq-poly-files")) {
    //         loadPolyFiles(oc, toFill, tm);
    //     }
    //     if (oc.isSet("dlr-navteq-poi-files")) {
    //         loadPOIFiles(oc, toFill, tm);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn load_if_set(oc: &mut OptionsCont, toFill: &mut PCPolyContainer, tm: &mut PCTypeMap) {
        todo!("port PCLoaderDlrNavteq::loadIfSet")
    }

    /// C++: `void PCLoaderDlrNavteq::loadPOIFiles(OptionsCont &, PCPolyContainer &, PCTypeMap &)` [static, protected]
    /// Loads pois assumed to be stored as according DLR-Navteq (Elmar)-files
    // void
    // PCLoaderDlrNavteq::loadPOIFiles(OptionsCont& oc, PCPolyContainer& toFill,
    //                                 PCTypeMap& tm) {
    //     std::vector<std::string> files = oc.getStringVector("dlr-navteq-poi-files");
    //     for (std::vector<std::string>::const_iterator file = files.begin(); file != files.end(); ++file) {
    //         if (!FileHelpers::isReadable(*file)) {
    //             throw ProcessError(TLF("Could not open dlr-navteq-poi-file '%'.", *file));
    //         }
    //         PROGRESS_BEGIN_MESSAGE("Parsing pois from dlr-navteq-poi-file '" + *file + "'");
    //         loadPOIFile(*file, oc, toFill, tm);
    //         PROGRESS_DONE_MESSAGE();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn load_poi_files(oc: &mut OptionsCont, toFill: &mut PCPolyContainer, tm: &mut PCTypeMap) {
        todo!("port PCLoaderDlrNavteq::loadPOIFiles")
    }

    /// C++: `void PCLoaderDlrNavteq::loadPolyFiles(OptionsCont &, PCPolyContainer &, PCTypeMap &)` [static, protected]
    /// Loads polygons assumed to be stored as according DLR-Navteq (Elmar)-files
    // void
    // PCLoaderDlrNavteq::loadPolyFiles(OptionsCont& oc, PCPolyContainer& toFill,
    //                                  PCTypeMap& tm) {
    //     std::vector<std::string> files = oc.getStringVector("dlr-navteq-poly-files");
    //     for (std::vector<std::string>::const_iterator file = files.begin(); file != files.end(); ++file) {
    //         if (!FileHelpers::isReadable(*file)) {
    //             throw ProcessError(TLF("Could not open dlr-navteq-poly-file '%'.", *file));
    //         }
    //         PROGRESS_BEGIN_MESSAGE("Parsing pois from dlr-navteq-poly-file '" + *file + "'");
    //         loadPolyFile(*file, oc, toFill, tm);
    //         PROGRESS_DONE_MESSAGE();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn load_poly_files(oc: &mut OptionsCont, toFill: &mut PCPolyContainer, tm: &mut PCTypeMap) {
        todo!("port PCLoaderDlrNavteq::loadPolyFiles")
    }

    /// C++: `void PCLoaderDlrNavteq::loadPOIFile(const std::string &, OptionsCont &, PCPolyContainer &, PCTypeMap &)` [static, protected]
    /// Loads DLR-Navteq (Elmar)-pois from the given file
    // void
    // PCLoaderDlrNavteq::loadPOIFile(const std::string& file,
    //                                OptionsCont& oc, PCPolyContainer& toFill,
    //                                PCTypeMap& tm) {
    //     // get the defaults
    //     RGBColor c = RGBColor::parseColor(oc.getString("color"));
    //     // parse
    //     int l = 0;
    //     LineReader lr(file);
    //     while (lr.hasMore()) {
    //         std::string line = lr.readLine();
    //         ++l;
    //         // skip invalid/empty lines
    //         if (line.length() == 0 || line.find("#") != std::string::npos) {
    //             continue;
    //         }
    //         if (StringUtils::prune(line) == "") {
    //             continue;
    //         }
    //         // parse the poi
    //         std::istringstream stream(line);
    //         // attributes of the poi
    //         std::string name, skip, type, desc;
    //         std::getline(stream, name, '\t');
    //         std::getline(stream, skip, '\t');
    //         std::getline(stream, type, '\t');
    //         std::getline(stream, desc, '\t');
    //         if (stream.fail()) {
    //             throw ProcessError("Invalid dlr-navteq-poi in line " + toString(l) + ":\n" + line);
    //         }
    //         double x, y;
    //         stream >> x;
    //         if (stream.fail()) {
    //             throw ProcessError(TLF("Invalid x coordinate for POI '%'.", name));
    //         }
    //         stream >> y;
    //         if (stream.fail()) {
    //             throw ProcessError(TLF("Invalid y coordinate for POI '%'.", name));
    //         }
    //         Position pos(x, y);
    //         // check the poi
    //         if (name == "") {
    //             throw ProcessError(TL("The name of a POI is missing."));
    //         }
    //         if (!GeoConvHelper::getProcessing().x2cartesian(pos, true)) {
    //             throw ProcessError(TLF("Unable to project coordinates for POI '%'.", name));
    //         }
    //
    //         // patch the values
    //         bool discard = oc.getBool("discard");
    //         std::string icon = oc.getString("icon");
    //         double layer = oc.getFloat("layer");
    //         RGBColor color;
    //         if (tm.has(type)) {
    //             const PCTypeMap::TypeDef& def = tm.get(type);
    //             name = def.prefix + name;
    //             type = def.id;
    //             color = def.color;
    //             discard = def.discard;
    //             icon = def.icon;
    //             layer = def.layer;
    //         } else {
    //             name = oc.getString("prefix") + name;
    //             type = oc.getString("type");
    //             color = c;
    //         }
    //         if (!discard) {
    //             PointOfInterest* poi = new PointOfInterest(name, type, color, pos, false, "", 0, false, 0, icon, layer);
    //             toFill.add(poi, OptionsCont::getOptions().isInStringVector("prune.keep-list", name));
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn load_poi_file(file: &str, oc: &mut OptionsCont, toFill: &mut PCPolyContainer, tm: &mut PCTypeMap) {
        todo!("port PCLoaderDlrNavteq::loadPOIFile")
    }

    /// C++: `void PCLoaderDlrNavteq::loadPolyFile(const std::string &, OptionsCont &, PCPolyContainer &, PCTypeMap &)` [static, protected]
    /// Loads DLR-Navteq (Elmar)-polygons from the given file
    // void
    // PCLoaderDlrNavteq::loadPolyFile(const std::string& file,
    //                                 OptionsCont& oc, PCPolyContainer& toFill,
    //                                 PCTypeMap& tm) {
    //     // get the defaults
    //     RGBColor c = RGBColor::parseColor(oc.getString("color"));
    //     // attributes of the poly
    //     // parse
    //     LineReader lr(file);
    //     while (lr.hasMore()) {
    //         std::string line = lr.readLine();
    //         // skip invalid/empty lines
    //         if (line.length() == 0 || line.find("#") != std::string::npos) {
    //             continue;
    //         }
    //         if (StringUtils::prune(line) == "") {
    //             continue;
    //         }
    //         // parse the poi
    //         StringTokenizer st(line, "\t");
    //         std::vector<std::string> values = st.getVector();
    //         if (values.size() < 6 || values.size() % 2 != 0) {
    //             throw ProcessError(TLF("Invalid dlr-navteq-polygon - line: '%'.", line));
    //         }
    //         std::string id = values[0];
    //         std::string ort = values[1];
    //         std::string type = values[2];
    //         std::string name = values[3];
    //         PositionVector vec;
    //         int index = 4;
    //         // now collect the positions
    //         while ((int)values.size() > index) {
    //             std::string xpos = values[index];
    //             std::string ypos = values[index + 1];
    //             index += 2;
    //             double x = StringUtils::toDouble(xpos);
    //             double y = StringUtils::toDouble(ypos);
    //             Position pos(x, y);
    //             if (!GeoConvHelper::getProcessing().x2cartesian(pos)) {
    //                 WRITE_WARNINGF(TL("Unable to project coordinates for polygon '%'."), id);
    //             }
    //             vec.push_back(pos);
    //         }
    //
    //         name = StringUtils::convertUmlaute(name);
    //         if (name == "noname" || toFill.getPolygons().get(name) != 0) {
    //             name = name + "#" + toString(toFill.getEnumIDFor(name));
    //         }
    //
    //         // check the polygon
    //         if (vec.size() == 0) {
    //             WRITE_WARNINGF(TL("The polygon '%' is empty."), id);
    //             continue;
    //         }
    //         if (id == "") {
    //             WRITE_WARNING(TL("The name of a polygon is missing; it will be discarded."));
    //             continue;
    //         }
    //
    //         // patch the values
    //         bool fill = vec.front() == vec.back();
    //         bool discard = oc.getBool("discard");
    //         std::string icon = oc.getString("icon");
    //         double layer = oc.getFloat("layer");
    //         RGBColor color;
    //         if (tm.has(type)) {
    //             const PCTypeMap::TypeDef& def = tm.get(type);
    //             name = def.prefix + name;
    //             type = def.id;
    //             color = def.color;
    //             fill = fill && def.allowFill != PCTypeMap::Filltype::NOFILL;
    //             discard = def.discard;
    //             icon = def.icon;
    //             layer = def.layer;
    //         } else {
    //             name = oc.getString("prefix") + name;
    //             type = oc.getString("type");
    //             color = c;
    //         }
    //         if (!discard) {
    //             SUMOPolygon* poly = new SUMOPolygon(name, type, color, vec, false, fill, 1, layer);
    //             toFill.add(poly);
    //         }
    //         vec.clear();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn load_poly_file(file: &str, oc: &mut OptionsCont, toFill: &mut PCPolyContainer, tm: &mut PCTypeMap) {
        todo!("port PCLoaderDlrNavteq::loadPolyFile")
    }

}

