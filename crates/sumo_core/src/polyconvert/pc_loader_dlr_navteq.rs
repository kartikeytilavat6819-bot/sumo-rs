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
        if or_panic(oc.is_set("dlr-navteq-poly-files", true)) {
            Self::load_poly_files(oc, toFill, tm);
        }
        if or_panic(oc.is_set("dlr-navteq-poi-files", true)) {
            Self::load_poi_files(oc, toFill, tm);
        }
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
        let files: Vec<String> = or_panic(oc.get_string_vector("dlr-navteq-poi-files")).clone();
        for file in files.iter() {
            if !utils::common::file_helpers::is_readable(file) {
                panic!("{}", utils::tlf!("Could not open dlr-navteq-poi-file '%'.", file));
            }
            utils::common::msg_handler::progress_begin_message(&format!("Parsing pois from dlr-navteq-poi-file '{}'", file));
            Self::load_poi_file(file, oc, toFill, tm);
            utils::common::msg_handler::progress_done_message();
        }
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
        let files: Vec<String> = or_panic(oc.get_string_vector("dlr-navteq-poly-files")).clone();
        for file in files.iter() {
            if !utils::common::file_helpers::is_readable(file) {
                panic!("{}", utils::tlf!("Could not open dlr-navteq-poly-file '%'.", file));
            }
            utils::common::msg_handler::progress_begin_message(&format!("Parsing pois from dlr-navteq-poly-file '{}'", file));
            Self::load_poly_file(file, oc, toFill, tm);
            utils::common::msg_handler::progress_done_message();
        }
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
        // get the defaults
        let c: RGBColor = or_panic(RGBColor::parse_color(&or_panic(oc.get_string("color"))));
        // parse
        let mut l: i32 = 0;
        let mut lr = utils::importio::line_reader::LineReader::with_file(file);
        while lr.has_more() {
            let line: String = lr.read_line();
            l += 1;
            // skip invalid/empty lines
            if line.len() == 0 || line.contains('#') {
                continue;
            }
            if utils::common::string_utils::prune(&line) == "" {
                continue;
            }
            // parse the poi
            // attributes of the poi (std::istringstream emulated by a cursor)
            let mut cur: usize = 0;
            let (mut name, ok1) = getline_delim(&line, &mut cur, '\t');
            let (_skip, ok2) = getline_delim(&line, &mut cur, '\t');
            let (mut ty, ok3) = getline_delim(&line, &mut cur, '\t');
            let (_desc, ok4) = getline_delim(&line, &mut cur, '\t');
            if !(ok1 && ok2 && ok3 && ok4) {
                panic!("Invalid dlr-navteq-poi in line {}:\n{}", l, line);
            }
            let Some(x) = extract_double(&line, &mut cur) else {
                panic!("{}", utils::tlf!("Invalid x coordinate for POI '%'.", name));
            };
            let Some(y) = extract_double(&line, &mut cur) else {
                panic!("{}", utils::tlf!("Invalid y coordinate for POI '%'.", name));
            };
            let mut pos = Position::new(x, y);
            // check the poi
            if name == "" {
                panic!("The name of a POI is missing.");
            }
            if !GeoConvHelper::with_processing(|g| g.x2cartesian(&mut pos, true)) {
                panic!("{}", utils::tlf!("Unable to project coordinates for POI '%'.", name));
            }

            // patch the values
            let mut discard: bool = or_panic(oc.get_bool("discard"));
            let mut icon: String = or_panic(oc.get_string("icon"));
            let mut layer: f64 = or_panic(oc.get_float("layer"));
            let mut color: RGBColor;
            if tm.has(&ty) {
                let def = tm.get(&ty);
                name = def.prefix.clone() + &name;
                ty = def.id.clone();
                color = def.color;
                discard = def.discard;
                icon = def.icon.clone();
                layer = def.layer;
            } else {
                name = or_panic(oc.get_string("prefix")) + &name;
                ty = or_panic(oc.get_string("type"));
                color = c;
            }
            if !discard {
                let poi: Ptr<PointOfInterest> = Ptr::from_raw(Box::into_raw(Box::new(PointOfInterest::new(&name, &ty, &color, &pos, false, "", 0.0, false, 0.0, &icon, layer, DEFAULT_ANGLE, DEFAULT_IMG_FILE, DEFAULT_IMG_WIDTH, DEFAULT_IMG_HEIGHT, DEFAULT_NAME, &BTreeMap::new()))));
                let keep = or_panic(oc.is_in_string_vector("prune.keep-list", &name));
                toFill.add_2(poi, keep);
            }
        }
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
        // get the defaults
        let c: RGBColor = or_panic(RGBColor::parse_color(&or_panic(oc.get_string("color"))));
        // attributes of the poly
        // parse
        let mut lr = utils::importio::line_reader::LineReader::with_file(file);
        while lr.has_more() {
            let line: String = lr.read_line();
            // skip invalid/empty lines
            if line.len() == 0 || line.contains('#') {
                continue;
            }
            if utils::common::string_utils::prune(&line) == "" {
                continue;
            }
            // parse the poi
            let mut st = utils::common::string_tokenizer::StringTokenizer::with_token(&line, "\t", false);
            let values: Vec<String> = st.get_vector();
            if values.len() < 6 || values.len() % 2 != 0 {
                panic!("{}", utils::tlf!("Invalid dlr-navteq-polygon - line: '%'.", line));
            }
            let id: String = values[0].clone();
            let _ort: String = values[1].clone();
            let mut ty: String = values[2].clone();
            let mut name: String = values[3].clone();
            let mut vec = PositionVector::new();
            let mut index: usize = 4;
            // now collect the positions
            while values.len() > index {
                let xpos: String = values[index].clone();
                let ypos: String = values[index + 1].clone();
                index += 2;
                let x: f64 = or_panic(utils::common::string_utils::to_double(&xpos));
                let y: f64 = or_panic(utils::common::string_utils::to_double(&ypos));
                let mut pos = Position::new(x, y);
                if !GeoConvHelper::with_processing(|g| g.x2cartesian(&mut pos, true)) {
                    utils::common::msg_handler::write_warning(&utils::tlf!("Unable to project coordinates for polygon '%'.", id));
                }
                vec.push(pos);
            }

            name = utils::common::string_utils::convert_umlaute(&name);
            if name == "noname" || toFill.base.get_polygons().contains_key(&name) {
                name = name.clone() + "#" + &utils::common::to_string::to_string(&toFill.get_enum_id_for(&name));
            }

            // check the polygon
            if vec.len() == 0 {
                utils::common::msg_handler::write_warning(&utils::tlf!("The polygon '%' is empty.", id));
                continue;
            }
            if id == "" {
                utils::common::msg_handler::write_warning("The name of a polygon is missing; it will be discarded.");
                continue;
            }

            // patch the values
            let mut fill: bool = vec.front() == vec.back();
            let mut discard: bool = or_panic(oc.get_bool("discard"));
            let mut icon: String = or_panic(oc.get_string("icon"));
            let mut layer: f64 = or_panic(oc.get_float("layer"));
            let mut color: RGBColor;
            if tm.has(&ty) {
                let def = tm.get(&ty);
                name = def.prefix.clone() + &name;
                ty = def.id.clone();
                color = def.color;
                fill = fill && def.allowFill != PCTypeMap_Filltype::NOFILL;
                discard = def.discard;
                icon = def.icon.clone();
                layer = def.layer;
            } else {
                name = or_panic(oc.get_string("prefix")) + &name;
                ty = or_panic(oc.get_string("type"));
                color = c;
            }
            if !discard {
                let poly: Ptr<SUMOPolygon> = Ptr::from_raw(Box::into_raw(Box::new(SUMOPolygon::new(&name, &ty, &color, &vec, false, fill, 1.0, layer, DEFAULT_ANGLE, DEFAULT_IMG_FILE, DEFAULT_NAME, DEFAULT_HEIGHT, &BTreeMap::new()))));
                toFill.add(poly, false);
            }
            vec.clear();
        }
    }

}

fn or_panic<T, E: std::fmt::Display>(r: Result<T, E>) -> T {
    r.unwrap_or_else(|e| panic!("{e}"))
}

/// std::getline(stream, s, delim) on a string cursor; the flag is false when nothing could be extracted (failbit)
fn getline_delim(s: &str, cur: &mut usize, delim: char) -> (String, bool) {
    if *cur >= s.len() {
        return (String::new(), false);
    }
    let rest = &s[*cur..];
    match rest.find(delim) {
        Some(i) => {
            *cur += i + delim.len_utf8();
            (rest[..i].to_string(), true)
        }
        None => {
            *cur = s.len();
            (rest.to_string(), true)
        }
    }
}

/// `stream >> double` on a string cursor: skips whitespace, then reads sign, digits, fraction and exponent
fn extract_double(s: &str, cur: &mut usize) -> Option<f64> {
    let b = s.as_bytes();
    let mut i = *cur;
    while i < b.len() && matches!(b[i], b' ' | b'\t' | b'\n' | 0x0b | 0x0c | b'\r') {
        i += 1;
    }
    let start = i;
    if i < b.len() && (b[i] == b'+' || b[i] == b'-') {
        i += 1;
    }
    let mut digits = 0;
    while i < b.len() && b[i].is_ascii_digit() {
        i += 1;
        digits += 1;
    }
    if i < b.len() && b[i] == b'.' {
        i += 1;
        while i < b.len() && b[i].is_ascii_digit() {
            i += 1;
            digits += 1;
        }
    }
    if digits == 0 {
        return None;
    }
    if i < b.len() && (b[i] == b'e' || b[i] == b'E') {
        i += 1;
        if i < b.len() && (b[i] == b'+' || b[i] == b'-') {
            i += 1;
        }
        let exp_start = i;
        while i < b.len() && b[i].is_ascii_digit() {
            i += 1;
        }
        if i == exp_start {
            return None;
        }
    }
    *cur = i;
    s[start..i].parse::<f64>().ok()
}

