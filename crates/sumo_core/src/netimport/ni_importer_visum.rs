//! Port of netimport/NIImporter_VISUM.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use self::NIImporter_VISUM_VISUM_KEY as K;
use utils::common::msg_handler::{progress_begin_message, progress_done_message, write_error, write_message, write_warning};
use utils::common::rgb_color::RGBColor;
use utils::common::string_bijection::StringBijection;
use utils::common::string_tokenizer::StringTokenizer;
use utils::common::string_utils::{ends_with, latin1_to_utf8, to_bool, to_double, to_int, to_long, to_lower_case};
use utils::common::sumo_vehicle_class::{get_vehicle_class_names, is_railway, parse_vehicle_classes};
use utils::common::to_string::f64_to_string;
use utils::common::util_exceptions::ProcessError;
use utils::geom::geo_conv_helper::GeoConvHelper;
use utils::geom::POSITION_EPS;
use utils::options::options_cont::{get_bool as opt_bool, get_float as opt_float, get_int as opt_int, get_string as opt_string};

/// C++ `throw`: raised as a panic carrying the ProcessError until a top-level catch exists
fn throw<T>(r: Result<T, ProcessError>) -> T {
    r.unwrap_or_else(|e| std::panic::panic_any(e))
}

/// C++ `try { ... } catch (ProcessError&)`: runs f and returns the ProcessError it throws
// shortcut: the default panic hook still prints each caught ProcessError to stderr, upgrade with a hook that silences them
fn catch_process_error<T>(f: impl FnOnce() -> T) -> Result<T, ProcessError> {
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)) {
        Ok(v) => Ok(v),
        Err(payload) => match payload.downcast::<ProcessError>() {
            Ok(e) => Err(*e),
            Err(other) => std::panic::resume_unwind(other),
        },
    }
}

/// NBHelpers::normalIDRepresentation
fn normal_id(s: &str) -> String {
    NBHelpers::normal_id_representation(s)
}

/// the part of an edge id before its first '_' (whole id if there is none)
fn before_underscore(s: &str) -> &str {
    match s.find('_') {
        Some(p) => &s[..p],
        None => s,
    }
}

const TSYSPREFIX: &str = "@";

const KEYS_DE: &[(&str, NIImporter_VISUM_VISUM_KEY)] = &[
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
    ("HALTEPUNKT", K::VISUM_STOPPOINT),
    ("NAME", K::VISUM_NAME),
    ("STRNR", K::VISUM_LINKNO),
    ("RELPOS", K::VISUM_RELPOS),
    ("KATNR", K::VISUM_CATID),
    ("ZWISCHENPUNKT", K::VISUM_EDGEITEM),
    ("POIKATEGORIE", K::VISUM_POICATEGORY),
    ("NETZ", K::VISUM_NETWORK),
    ("DEFKOORD", K::VISUM_PROJECTIONDEFINITION),
    ("IV", K::VISUM_PRT),
    ("NR", K::VISUM_NO), // must be the last one
];

thread_local! {
    // C++ NIImporter_VISUM::KEYS; StringBijection inserts the terminator entry too, as the C++ constructor does
    static KEYS: std::cell::RefCell<StringBijection<NIImporter_VISUM_VISUM_KEY>> =
        std::cell::RefCell::new(StringBijection::new(KEYS_DE, true).unwrap_or_else(|e| panic!("{e}")));
}

/// C++ KEYS.getString(key)
fn keys_string(key: NIImporter_VISUM_VISUM_KEY) -> String {
    KEYS.with(|k| throw(k.borrow().get_string(key).map(|s| s.to_string())))
}

/// C++ enum `NIImporter_VISUM::VISUM_KEY`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum NIImporter_VISUM_VISUM_KEY {
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
    VISUM_STOPPOINT = 37,
    VISUM_NAME = 38,
    VISUM_LINKNO = 39,
    VISUM_RELPOS = 40,
    VISUM_CATID = 41,
    VISUM_EDGEITEM = 42,
    VISUM_POICATEGORY = 43,
    VISUM_NETWORK = 44,
    VISUM_PROJECTIONDEFINITION = 45,
    VISUM_PRT = 46,
    VISUM_NO = 47,
}

/// C++ typedef `std::map<std::string, NIVisumTL *> NIImporter_VISUM::NIVisumTL_Map`
pub type NIImporter_VISUM_NIVisumTL_Map = BTreeMap<String, Ptr<NIVisumTL>>;

/// C++ typedef `std::vector<TypeParser> NIImporter_VISUM::ParserVector`
pub type NIImporter_VISUM_ParserVector = Vec<NIImporter_VISUM_TypeParser>;

/// C++ typedef `void (NIImporter_VISUM::*)() NIImporter_VISUM::ParsingFunction`
pub type NIImporter_VISUM_ParsingFunction = fn(&mut NIImporter_VISUM);

/// C++ typedef `std::map<std::string, VSysType> NIImporter_VISUM::VSysTypeNames`
pub type NIImporter_VISUM_VSysTypeNames = BTreeMap<String, NIImporter_VISUM_VSysType>;

/// A VISUM network importer
/// C++ class `NIImporter_VISUM` (netimport/NIImporter_VISUM.h)
pub struct NIImporter_VISUM {
    /// The network builder to fill with loaded values
    pub myNetBuilder: Ptr<NBNetBuilder>,
    /// The name of the parsed file, for error reporting
    pub myFileName: String,
    /// The line reader to use to read from the file
    pub myLineReader: LineReader,
    /// the parser to parse the information from the data lines
    pub myLineParser: NamedColumnsParser,
    /// The converter to compute the lane number of edges from their capacity
    pub myCapacity2Lanes: NBCapacity2Lanes,
    /// The used vsystypes
    pub myVSysTypes: NIImporter_VISUM_VSysTypeNames,
    /// List of known parsers
    pub mySingleDataParsers: NIImporter_VISUM_ParserVector,
    /// List of visum traffic lights
    pub myTLS: NIImporter_VISUM_NIVisumTL_Map,
    /// Already read edges
    pub myTouchedEdges: Vec<String>,
    /// Information whether VISUM priority information shall be used
    pub myUseVisumPrio: bool,
    /// The name of the currently parsed item used for error reporting
    pub myCurrentID: String,
    /// A map of point ids to positions
    pub myPoints: BTreeMap<i64, Position>,
    /// A map of edge (not road, but "edge" in this case) ids to from/to-points
    pub myEdges: BTreeMap<i64, (i64, i64)>, // C++: std::map<long long, std::pair<long long, long long>>
    /// A map from district shape definition name to the district
    pub myShapeDistrictMap: BTreeMap<i64, Ptr<NBDistrict>>,
    /// A map from area parts to area ids
    pub mySubPartsAreas: BTreeMap<i64, Vec<i64>>,
    /// A temporary storage for district shapes as they are filled incrementally
    pub myDistrictShapes: BTreeMap<Ptr<NBDistrict>, PositionVector>,
    pub myDefaultPermissions: SVCPermissions,
}

// C++ static members of NIImporter_VISUM (port as module-level statics, see AGENTS.md):
//   static StringBijection<VISUM_KEY>::Entry[] NIImporter_VISUM::KEYS_DE;
//   static StringBijection<VISUM_KEY> NIImporter_VISUM::KEYS;

impl NIImporter_VISUM {
    /// C++ myLineParser.get(name): throws when the column is missing or the line is too short
    fn field(&self, name: &str) -> String {
        throw(self.myLineParser.get(name, false))
    }

    /// C++: `void NIImporter_VISUM::loadNetwork(const OptionsCont &, NBNetBuilder &)` [static]
    /// Loads network definition from the assigned option and stores it in the given network builder
    // void
    // NIImporter_VISUM::loadNetwork(const OptionsCont& oc, NBNetBuilder& nb) {
    //     // check whether the option is set (properly)
    //     if (!oc.isSet("visum-file")) {
    //         return;
    //     }
    //     // build the handler
    //     NIImporter_VISUM loader(nb, oc.getString("visum-file"),
    //                             NBCapacity2Lanes(oc.getFloat("lanes-from-capacity.norm")),
    //                             oc.getBool("visum.use-type-priority"),
    //                             oc.getString("visum.language-file"));
    //     // rename loaded types (tsys code interpretations) so they never clash with edge types
    //     std::vector<std::string> codes;
    //     for (auto it = nb.getTypeCont().begin(); it != nb.getTypeCont().end(); it++) {
    //         codes.push_back(it->first);
    //     }
    //     for (const std::string& code : codes) {
    //         nb.getTypeCont().updateEdgeTypeID(code, TSYSPREFIX + code);
    //     }
    //     loader.load();
    // }
    #[allow(unused_variables)]
    pub fn load_network(oc: &OptionsCont, nb: &mut NBNetBuilder) {
        // check whether the option is set (properly)
        if !throw(oc.is_set("visum-file", false)) {
            return;
        }
        // build the handler
        let mut loader = NIImporter_VISUM::new(
            nb,
            &throw(oc.get_string("visum-file")),
            NBCapacity2Lanes::new(throw(oc.get_float("lanes-from-capacity.norm"))),
            throw(oc.get_bool("visum.use-type-priority")),
            &throw(oc.get_string("visum.language-file")),
        );
        // rename loaded types (tsys code interpretations) so they never clash with edge types
        let codes: Vec<String> = nb.get_type_cont().begin().map(|(k, _)| k.clone()).collect();
        for code in &codes {
            nb.get_type_cont().update_edge_type_id(code, &format!("{TSYSPREFIX}{code}"));
        }
        loader.load();
        loader.drop_();
    }

    /// C++: `void NIImporter_VISUM::NIImporter_VISUM(NBNetBuilder &, const std::string &, NBCapacity2Lanes, bool, const std::string &)` [protected]
    /// constructor
    // NIImporter_VISUM::NIImporter_VISUM(NBNetBuilder& nb,
    //                                    const std::string& file,
    //                                    NBCapacity2Lanes capacity2Lanes,
    //                                    bool useVisumPrio,
    //                                    const std::string& languageFile) :
    //     myNetBuilder(nb), myFileName(file),
    //     myCapacity2Lanes(capacity2Lanes), myUseVisumPrio(useVisumPrio) {
    //     if (languageFile != "") {
    //         loadLanguage(languageFile);
    //     }
    //     myDefaultPermissions = parseVehicleClasses(OptionsCont::getOptions().getString("default.allow"));
    //
    //     // the order of process is important!
    //     // set1
    //     addParser(KEYS.getString(VISUM_NETWORK), &NIImporter_VISUM::parse_Network);
    //     addParser(KEYS.getString(VISUM_SYS), &NIImporter_VISUM::parse_VSysTypes);
    //     addParser(KEYS.getString(VISUM_LINKTYPE), &NIImporter_VISUM::parse_Types);
    //     addParser(KEYS.getString(VISUM_NODE), &NIImporter_VISUM::parse_Nodes);
    //     addParser(KEYS.getString(VISUM_DISTRICT), &NIImporter_VISUM::parse_Districts);
    //     addParser(KEYS.getString(VISUM_POINT), &NIImporter_VISUM::parse_Point);
    //
    //     // set2
    //     // two types of "strecke"
    //     addParser(KEYS.getString(VISUM_LINK), &NIImporter_VISUM::parse_Edges);
    //     addParser(KEYS.getString(VISUM_EDGE), &NIImporter_VISUM::parse_Kante);
    //
    //     // set3
    //     if (OptionsCont::getOptions().getBool("visum.no-connectors")) {
    //         addParser(KEYS.getString(VISUM_DISTRICT_CONNECTION), &NIImporter_VISUM::parse_Connectors);
    //     } else {
    //         addParser(KEYS.getString(VISUM_DISTRICT_CONNECTION), &NIImporter_VISUM::parse_Connectors_legacy);
    //     }
    //     // two types of "abbieger"
    //     addParser("ABBIEGEBEZIEHUNG", &NIImporter_VISUM::parse_Turns);
    //     addParser(KEYS.getString(VISUM_TURN), &NIImporter_VISUM::parse_Turns);
    //
    //     addParser(KEYS.getString(VISUM_LINKPOLY), &NIImporter_VISUM::parse_EdgePolys);
    //     addParser("FAHRSTREIFEN", &NIImporter_VISUM::parse_Lanes);
    //     addParser(KEYS.getString(VISUM_SURFACEITEM), &NIImporter_VISUM::parse_PartOfArea);
    //
    //
    //     // set4
    //     // two types of lsa
    //     addParser("LSA", &NIImporter_VISUM::parse_TrafficLights);
    //     addParser("SIGNALANLAGE", &NIImporter_VISUM::parse_TrafficLights);
    //     // two types of knotenzulsa
    //     addParser("KNOTENZULSA", &NIImporter_VISUM::parse_NodesToTrafficLights);
    //     addParser("LSAZUKNOTEN", &NIImporter_VISUM::parse_NodesToTrafficLights);
    //     addParser("SIGNALANLAGEZUKNOTEN", &NIImporter_VISUM::parse_NodesToTrafficLights);
    //     // two types of signalgruppe
    //     addParser("LSASIGNALGRUPPE", &NIImporter_VISUM::parse_SignalGroups);
    //     addParser("SIGNALGRUPPE", &NIImporter_VISUM::parse_SignalGroups);
    //     // three types of ABBZULSASIGNALGRUPPE
    //     addParser("ABBZULSASIGNALGRUPPE", &NIImporter_VISUM::parse_TurnsToSignalGroups);
    //     addParser("SIGNALGRUPPEZUABBIEGER", &NIImporter_VISUM::parse_TurnsToSignalGroups);
    //     addParser("SIGNALGRUPPEZUFSABBIEGER", &NIImporter_VISUM::parse_TurnsToSignalGroups);
    //
    //     addParser(KEYS.getString(VISUM_FACEITEM), &NIImporter_VISUM::parse_AreaSubPartElement);
    //
    //     // two types of LSAPHASE
    //     addParser("LSAPHASE", &NIImporter_VISUM::parse_Phases);
    //     addParser("PHASE", &NIImporter_VISUM::parse_Phases);
    //
    //     addParser("LSASIGNALGRUPPEZULSAPHASE", &NIImporter_VISUM::parse_SignalGroupsToPhases);
    //     addParser("FAHRSTREIFENABBIEGER", &NIImporter_VISUM::parse_LanesConnections);
    //
    //     addParser(KEYS.getString(VISUM_STOPPOINT), &NIImporter_VISUM::parse_stopPoints);
    // }
    #[allow(unused_variables)]
    pub fn new(nb: &mut NBNetBuilder, file: &str, capacity2Lanes: NBCapacity2Lanes, useVisumPrio: bool, languageFile: &str) -> Self {
        let mut s = NIImporter_VISUM {
            myNetBuilder: Ptr::from_mut(nb),
            myFileName: file.to_string(),
            myLineReader: LineReader::new(),
            myLineParser: NamedColumnsParser::new(),
            myCapacity2Lanes: capacity2Lanes,
            myVSysTypes: BTreeMap::new(),
            mySingleDataParsers: Vec::new(),
            myTLS: BTreeMap::new(),
            myTouchedEdges: Vec::new(),
            myUseVisumPrio: useVisumPrio,
            myCurrentID: String::new(),
            myPoints: BTreeMap::new(),
            myEdges: BTreeMap::new(),
            myShapeDistrictMap: BTreeMap::new(),
            mySubPartsAreas: BTreeMap::new(),
            myDistrictShapes: BTreeMap::new(),
            myDefaultPermissions: 0,
        };
        if languageFile != "" {
            s.load_language(languageFile);
        }
        s.myDefaultPermissions = parse_vehicle_classes(&opt_string("default.allow"));

        // the order of process is important!
        // set1
        s.add_parser(&keys_string(K::VISUM_NETWORK), NIImporter_VISUM::parse_network);
        s.add_parser(&keys_string(K::VISUM_SYS), NIImporter_VISUM::parse_v_sys_types);
        s.add_parser(&keys_string(K::VISUM_LINKTYPE), NIImporter_VISUM::parse_types);
        s.add_parser(&keys_string(K::VISUM_NODE), NIImporter_VISUM::parse_nodes);
        s.add_parser(&keys_string(K::VISUM_DISTRICT), NIImporter_VISUM::parse_districts);
        s.add_parser(&keys_string(K::VISUM_POINT), NIImporter_VISUM::parse_point);

        // set2
        // two types of "strecke"
        s.add_parser(&keys_string(K::VISUM_LINK), NIImporter_VISUM::parse_edges);
        s.add_parser(&keys_string(K::VISUM_EDGE), NIImporter_VISUM::parse_kante);

        // set3
        if opt_bool("visum.no-connectors") {
            s.add_parser(&keys_string(K::VISUM_DISTRICT_CONNECTION), NIImporter_VISUM::parse_connectors);
        } else {
            s.add_parser(&keys_string(K::VISUM_DISTRICT_CONNECTION), NIImporter_VISUM::parse_connectors_legacy);
        }
        // two types of "abbieger"
        s.add_parser("ABBIEGEBEZIEHUNG", NIImporter_VISUM::parse_turns);
        s.add_parser(&keys_string(K::VISUM_TURN), NIImporter_VISUM::parse_turns);

        s.add_parser(&keys_string(K::VISUM_LINKPOLY), NIImporter_VISUM::parse_edge_polys);
        s.add_parser("FAHRSTREIFEN", NIImporter_VISUM::parse_lanes);
        s.add_parser(&keys_string(K::VISUM_SURFACEITEM), NIImporter_VISUM::parse_part_of_area);

        // set4
        // two types of lsa
        s.add_parser("LSA", NIImporter_VISUM::parse_traffic_lights);
        s.add_parser("SIGNALANLAGE", NIImporter_VISUM::parse_traffic_lights);
        // two types of knotenzulsa
        s.add_parser("KNOTENZULSA", NIImporter_VISUM::parse_nodes_to_traffic_lights);
        s.add_parser("LSAZUKNOTEN", NIImporter_VISUM::parse_nodes_to_traffic_lights);
        s.add_parser("SIGNALANLAGEZUKNOTEN", NIImporter_VISUM::parse_nodes_to_traffic_lights);
        // two types of signalgruppe
        s.add_parser("LSASIGNALGRUPPE", NIImporter_VISUM::parse_signal_groups);
        s.add_parser("SIGNALGRUPPE", NIImporter_VISUM::parse_signal_groups);
        // three types of ABBZULSASIGNALGRUPPE
        s.add_parser("ABBZULSASIGNALGRUPPE", NIImporter_VISUM::parse_turns_to_signal_groups);
        s.add_parser("SIGNALGRUPPEZUABBIEGER", NIImporter_VISUM::parse_turns_to_signal_groups);
        s.add_parser("SIGNALGRUPPEZUFSABBIEGER", NIImporter_VISUM::parse_turns_to_signal_groups);

        s.add_parser(&keys_string(K::VISUM_FACEITEM), NIImporter_VISUM::parse_area_sub_part_element);

        // two types of LSAPHASE
        s.add_parser("LSAPHASE", NIImporter_VISUM::parse_phases);
        s.add_parser("PHASE", NIImporter_VISUM::parse_phases);

        s.add_parser("LSASIGNALGRUPPEZULSAPHASE", NIImporter_VISUM::parse_signal_groups_to_phases);
        s.add_parser("FAHRSTREIFENABBIEGER", NIImporter_VISUM::parse_lanes_connections);

        s.add_parser(&keys_string(K::VISUM_STOPPOINT), NIImporter_VISUM::parse_stop_points);
        s
    }

    /// C++: `void NIImporter_VISUM::~NIImporter_VISUM() throw()` [protected]
    /// destructor
    // NIImporter_VISUM::~NIImporter_VISUM() {
    //     for (NIVisumTL_Map::iterator j = myTLS.begin(); j != myTLS.end(); j++) {
    //         delete j->second;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        for (_, tl) in std::mem::take(&mut self.myTLS) {
            // SAFETY: each NIVisumTL was leaked from a Box in parse_traffic_lights and is owned by this importer only
            drop(unsafe { Box::from_raw(tl.as_raw()) });
        }
    }

    /// C++: `void NIImporter_VISUM::load()` [protected]
    /// Parses the VISUM-network file storing the parsed structures within myNetBuilder
    // void
    // NIImporter_VISUM::load() {
    //     // open the file
    //     if (!myLineReader.setFile(myFileName)) {
    //         throw ProcessError(TLF("Can not open visum-file '%'.", myFileName));
    //     }
    //     // scan the file for data positions
    //     while (myLineReader.hasMore()) {
    //         std::string line = myLineReader.readLine();
    //         if (line.length() > 0 && line[0] == '$') {
    //             ParserVector::iterator i;
    //             for (i = mySingleDataParsers.begin(); i != mySingleDataParsers.end(); i++) {
    //                 std::string dataName = "$" + (*i).name + ":";
    //                 if (line.substr(0, dataName.length()) == dataName) {
    //                     (*i).position = myLineReader.getPosition();
    //                     (*i).pattern = line.substr(dataName.length());
    //                     WRITE_MESSAGE("Found: " + dataName + " at line " + toString<int>(myLineReader.getLineNumber()));
    //                 }
    //             }
    //         }
    //     }
    //     // go through the parsers and process all entries
    //     for (ParserVector::iterator i = mySingleDataParsers.begin(); i != mySingleDataParsers.end(); i++) {
    //         if ((*i).position < 0) {
    //             // do not process using parsers for which no information was found
    //             continue;
    //         }
    //         // ok, the according information is stored in the file
    //         PROGRESS_BEGIN_MESSAGE("Parsing " + (*i).name);
    //         // reset the line reader and let it point to the begin of the according data field
    //         myLineReader.reinit();
    //         myLineReader.setPos((*i).position);
    //         // prepare the line parser
    //         myLineParser.reinit((*i).pattern);
    //         // read
    //         bool singleDataEndFound = false;
    //         while (myLineReader.hasMore() && !singleDataEndFound) {
    //             std::string line = myLineReader.readLine();
    //             if (line.length() == 0 || line[0] == '*' || line[0] == '$') {
    //                 singleDataEndFound = true;
    //             } else {
    //                 myLineParser.parseLine(line);
    //                 try {
    //                     myCurrentID = "<unknown>";
    //                     (this->*(*i).function)();
    //                 } catch (OutOfBoundsException&) {
    //                     WRITE_ERRORF(TL("Too short value line in % occurred."), (*i).name);
    //                 } catch (NumberFormatException&) {
    //                     WRITE_ERRORF(TL("A value in % should be numeric but is not (id='%')."), (*i).name, myCurrentID);
    //                 } catch (UnknownElement& e) {
    //                     WRITE_ERRORF(TL("One of the needed values ('%') is missing in %."), std::string(e.what()), (*i).name);
    //                 }
    //             }
    //         }
    //         // close single reader processing
    //         PROGRESS_DONE_MESSAGE();
    //     }
    //     myNetBuilder.getEdgeCont().reduceGeometries(POSITION_EPS);
    //
    //     // build traffic lights
    //     for (NIVisumTL_Map::iterator j = myTLS.begin(); j != myTLS.end(); j++) {
    //         j->second->build(myNetBuilder.getEdgeCont(), myNetBuilder.getTLLogicCont());
    //     }
    //     // build district shapes
    //     for (std::map<NBDistrict*, PositionVector>::const_iterator k = myDistrictShapes.begin(); k != myDistrictShapes.end(); ++k) {
    //         (*k).first->addShape((*k).second);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn load(&mut self) {
        // open the file
        if !self.myLineReader.set_file(&self.myFileName) {
            std::panic::panic_any(ProcessError::new(utils::tlf!("Can not open visum-file '%'.", self.myFileName)));
        }
        // scan the file for data positions
        while self.myLineReader.has_more() {
            let line = self.myLineReader.read_line();
            if line.starts_with('$') {
                for i in 0..self.mySingleDataParsers.len() {
                    let dataName = format!("${}:", self.mySingleDataParsers[i].name);
                    if line.starts_with(&dataName) {
                        self.mySingleDataParsers[i].position = self.myLineReader.get_position() as i64;
                        self.mySingleDataParsers[i].pattern = line[dataName.len()..].to_string();
                        write_message(&format!("Found: {} at line {}", dataName, self.myLineReader.get_line_number()));
                    }
                }
            }
        }
        // go through the parsers and process all entries
        for i in 0..self.mySingleDataParsers.len() {
            if self.mySingleDataParsers[i].position < 0 {
                // do not process using parsers for which no information was found
                continue;
            }
            // ok, the according information is stored in the file
            let name = self.mySingleDataParsers[i].name.clone();
            progress_begin_message(&format!("Parsing {}", name));
            // reset the line reader and let it point to the begin of the according data field
            self.myLineReader.reinit();
            self.myLineReader.set_pos(self.mySingleDataParsers[i].position as u64);
            // prepare the line parser
            let pattern = self.mySingleDataParsers[i].pattern.clone();
            self.myLineParser.reinit(&pattern, ";", ";", false, true);
            // read
            let mut singleDataEndFound = false;
            while self.myLineReader.has_more() && !singleDataEndFound {
                let line = self.myLineReader.read_line();
                if line.len() == 0 || line.starts_with('*') || line.starts_with('$') {
                    singleDataEndFound = true;
                } else {
                    self.myLineParser.parse_line(&line);
                    let function = self.mySingleDataParsers[i].function;
                    let res = catch_process_error(|| {
                        self.myCurrentID = "<unknown>".to_string();
                        function(self);
                    });
                    match res {
                        Ok(()) => {}
                        Err(ProcessError::OutOfBounds(_)) => {
                            write_errorf!("Too short value line in % occurred.", name.clone());
                        }
                        Err(ProcessError::NumberFormat(_)) => {
                            write_errorf!("A value in % should be numeric but is not (id='%').", name.clone(), self.myCurrentID.clone());
                        }
                        Err(ProcessError::UnknownElement(m)) => {
                            write_errorf!("One of the needed values ('%') is missing in %.", m, name.clone());
                        }
                        Err(e) => std::panic::panic_any(e),
                    }
                }
            }
            // close single reader processing
            progress_done_message();
        }
        self.myNetBuilder.m().get_edge_cont().reduce_geometries(POSITION_EPS);

        // build traffic lights
        let tls: Vec<Ptr<NIVisumTL>> = self.myTLS.values().copied().collect();
        for tl in tls {
            let nb = self.myNetBuilder.m();
            let ec = Ptr::from_mut(nb.get_edge_cont());
            let tlc = Ptr::from_mut(nb.get_tl_logic_cont());
            tl.m().build(ec.m(), tlc.m());
        }
        // build district shapes
        let shapes: Vec<(Ptr<NBDistrict>, PositionVector)> = self.myDistrictShapes.iter().map(|(d, s)| (*d, s.clone())).collect();
        for (d, shape) in shapes {
            d.m().add_shape(&shape);
        }
    }

    /// C++: `double NIImporter_VISUM::getNamedFloat(const std::string &)` [private]
    /// Returns the value from the named column as a float
    // double
    // NIImporter_VISUM::getNamedFloat(const std::string& fieldName) {
    //     std::string value = myLineParser.get(fieldName);
    //     if (StringUtils::endsWith(myLineParser.get(fieldName), "km/h")) {
    //         value = value.substr(0, value.length() - 4);
    //     }
    //     return StringUtils::toDouble(value);
    // }
    #[allow(unused_variables)]
    pub fn get_named_float(&mut self, fieldName: &str) -> f64 {
        let mut value = self.field(fieldName);
        if ends_with(&self.field(fieldName), "km/h") {
            value.truncate(value.len() - 4);
        }
        throw(to_double(&value))
    }

    /// C++: `double NIImporter_VISUM::getNamedFloat(const std::string &, const std::string &)` [private]
    /// The same, but two different names for the field are allowed
    // double
    // NIImporter_VISUM::getNamedFloat(const std::string& fieldName1, const std::string& fieldName2) {
    //     if (myLineParser.know(fieldName1)) {
    //         return getNamedFloat(fieldName1);
    //     } else {
    //         return getNamedFloat(fieldName2);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_named_float_2(&mut self, fieldName1: &str, fieldName2: &str) -> f64 {
        if self.myLineParser.know(fieldName1) {
            self.get_named_float(fieldName1)
        } else {
            self.get_named_float(fieldName2)
        }
    }

    /// C++: `double NIImporter_VISUM::getNamedFloat(const std::string &, double)` [private]
    /// Returns the value from the named column as a float or the default value if an error occurs
    // double
    // NIImporter_VISUM::getNamedFloat(const std::string& fieldName, double defaultValue) {
    //     try {
    //         return StringUtils::toDouble(myLineParser.get(fieldName));
    //     } catch (...) {
    //         return defaultValue;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_named_float_3(&mut self, fieldName: &str, defaultValue: f64) -> f64 {
        match self.myLineParser.get(fieldName, false).and_then(|v| to_double(&v)) {
            Ok(v) => v,
            Err(_) => defaultValue,
        }
    }

    /// C++: `double NIImporter_VISUM::getNamedFloat(const std::string &, const std::string &, double)` [private]
    /// The same, but two different names for the field are allowed
    // double
    // NIImporter_VISUM::getNamedFloat(const std::string& fieldName1, const std::string& fieldName2,
    //                                 double defaultValue) {
    //     if (myLineParser.know(fieldName1)) {
    //         return getNamedFloat(fieldName1, defaultValue);
    //     } else {
    //         return getNamedFloat(fieldName2, defaultValue);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_named_float_4(&mut self, fieldName1: &str, fieldName2: &str, defaultValue: f64) -> f64 {
        if self.myLineParser.know(fieldName1) {
            self.get_named_float_3(fieldName1, defaultValue)
        } else {
            self.get_named_float_3(fieldName2, defaultValue)
        }
    }

    /// C++: `std::string NIImporter_VISUM::getNamedString(const std::string &)` [private]
    /// Returns the value from the named column as a normalised string
    // std::string
    // NIImporter_VISUM::getNamedString(const std::string& fieldName) {
    //     return NBHelpers::normalIDRepresentation(myLineParser.get(fieldName));
    // }
    #[allow(unused_variables)]
    pub fn get_named_string(&mut self, fieldName: &str) -> String {
        normal_id(&self.field(fieldName))
    }

    /// C++: `std::string NIImporter_VISUM::getNamedString(const std::string &, const std::string &)` [private]
    /// The same, but two different names for the field are allowed
    // std::string
    // NIImporter_VISUM::getNamedString(const std::string& fieldName1,
    //                                  const std::string& fieldName2) {
    //     if (myLineParser.know(fieldName1)) {
    //         return getNamedString(fieldName1);
    //     } else {
    //         return getNamedString(fieldName2);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_named_string_2(&mut self, fieldName1: &str, fieldName2: &str) -> String {
        if self.myLineParser.know(fieldName1) {
            self.get_named_string(fieldName1)
        } else {
            self.get_named_string(fieldName2)
        }
    }

    /// C++: `double NIImporter_VISUM::getWeightedFloat(const std::string &, const std::string &)` [private]
    /// tries to get a double which is possibly assigned to a certain modality
    // double
    // NIImporter_VISUM::getWeightedFloat(const std::string& name, const std::string& suffix) {
    //     try {
    //         std::string val = myLineParser.get(name);
    //         if (val.find(suffix) != std::string::npos) {
    //             val = val.substr(0, val.find(suffix));
    //         }
    //         return StringUtils::toDouble(val);
    //     } catch (...) {}
    //     return -1;
    // }
    #[allow(unused_variables)]
    pub fn get_weighted_float(&mut self, name: &str, suffix: &str) -> f64 {
        let r = self.myLineParser.get(name, false).and_then(|mut val| {
            if let Some(p) = val.find(suffix) {
                val.truncate(p);
            }
            to_double(&val)
        });
        match r {
            Ok(v) => v,
            Err(_) => -1.0,
        }
    }

    /// C++: `double NIImporter_VISUM::getWeightedFloat2(const std::string &, const std::string &, const std::string &)` [private]
    /// as above but with two alternative names
    // double
    // NIImporter_VISUM::getWeightedFloat2(const std::string& name, const std::string& name2, const std::string& suffix) {
    //     double result = getWeightedFloat(name, suffix);
    //     if (result != -1) {
    //         return result;
    //     } else {
    //         return getWeightedFloat(name2, suffix);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_weighted_float2(&mut self, name: &str, name2: &str, suffix: &str) -> f64 {
        let result = self.get_weighted_float(name, suffix);
        if result != -1.0 {
            result
        } else {
            self.get_weighted_float(name2, suffix)
        }
    }

    /// C++: `SVCPermissions NIImporter_VISUM::getPermissions(const std::string &, const std::string &, SVCPermissions)` [private]
    /// parse permissions
    // SVCPermissions
    // NIImporter_VISUM::getPermissions(const std::string& name, const std::string& edgeType, SVCPermissions unknown) {
    //     SVCPermissions result = 0;
    //     NBTypeCont& tc = myNetBuilder.getTypeCont();
    //     for (std::string v : StringTokenizer(myLineParser.get(name), ",").getVector()) {
    //         const std::string v2 = TSYSPREFIX + v;
    //         const std::string v3 = StringUtils::to_lower_case(v);
    //         const std::string v4 = TSYSPREFIX + v3;
    //         if (tc.knows(v2)) {
    //             result |= tc.getEdgeTypePermissions(v2);
    //         } else if (tc.knows(v4)) {
    //             result |= tc.getEdgeTypePermissions(v4);
    //         } else {
    //             SVCPermissions guessed = SVC_IGNORING;
    //             std::string desc;
    //             auto it = myVSysTypes.find(v);
    //             if (it != myVSysTypes.end()) {
    //                 desc = it->second.name;
    //                 if (desc.find("taxi") != std::string::npos) {
    //                     guessed = SVC_TAXI;
    //                 } else if (desc.find("bus") != std::string::npos) {
    //                     guessed = SVC_BUS;
    //                 } else if (desc.find("seilbahn") != std::string::npos || desc.find("funiculaire") != std::string::npos || desc.find("cable") != std::string::npos || desc.find("funicular") != std::string::npos) {
    //                     guessed = SVC_CABLE_CAR;
    //                 } else if (desc.find("subway") != std::string::npos || desc.find("ubahn") != std::string::npos || desc.find("u-bahn") != std::string::npos) {
    //                     guessed = SVC_SUBWAY;
    //                 } else if (desc.find("train") != std::string::npos || desc.find("schiene") != std::string::npos || desc.find("rail") != std::string::npos || desc.find("bahn") != std::string::npos || desc.find("zug") != std::string::npos) {
    //                     guessed = SVC_RAIL;
    //                 } else if (desc.find("tram") != std::string::npos || desc.find("strab") != std::string::npos) {
    //                     guessed = SVC_TRAM;
    //                 } else if (desc.find("moto") != std::string::npos) {
    //                     guessed = SVC_MOTORCYCLE;
    //                 } else if (desc.find("bike") != std::string::npos || desc.find("velo") != std::string::npos || desc.find("bicycle") != std::string::npos || desc.find("rad") != std::string::npos) {
    //                     guessed = SVC_BICYCLE;
    //                 } else if (desc.find("foot") != std::string::npos || desc.find("ped") != std::string::npos || desc.find("fu\xdf") != std::string::npos
    //                         || desc.find("fuss") != std::string::npos || desc.find("walk") != std::string::npos || desc.find("pied") != std::string::npos) {
    //                     guessed = SVC_PEDESTRIAN;
    //                 } else if (desc.find("lkw") != std::string::npos || desc.find("truck") != std::string::npos || desc.find("camion") != std::string::npos) {
    //                     guessed = SVC_TRUCK;
    //                 } else if (desc.find("pkw") != std::string::npos || desc.find("car") != std::string::npos || desc.find("auto") != std::string::npos) {
    //                     guessed = SVC_PASSENGER;
    //                 }
    //             }
    //             if (guessed == SVC_IGNORING) {
    //                 WRITE_WARNINGF("Encountered unknown vehicle category '%' in type '%' (description: '%')", v3, edgeType, desc);
    //                 guessed = unknown;
    //             } else {
    //                 WRITE_WARNINGF("Encountered unknown vehicle category '%' in type '%' (guessed '%' based on description '%')", v3, edgeType, getVehicleClassNames(guessed), desc);
    //             }
    //             tc.insertEdgeType(v4, 1, 1, -1, guessed, LaneSpreadFunction::RIGHT, NBEdge::UNSPECIFIED_WIDTH, false, NBEdge::UNSPECIFIED_WIDTH, NBEdge::UNSPECIFIED_WIDTH, 0, 0, 0);
    //             result |= guessed;
    //         }
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_permissions(&mut self, name: &str, edgeType: &str, unknown: SVCPermissions) -> SVCPermissions {
        let mut result: SVCPermissions = 0;
        let tc = self.myNetBuilder.m().get_type_cont();
        for v in StringTokenizer::with_token(&self.field(name), ",", false).get_vector() {
            let v2 = format!("{TSYSPREFIX}{v}");
            let v3 = to_lower_case(&v);
            let v4 = format!("{TSYSPREFIX}{v3}");
            if tc.knows(&v2) {
                result |= tc.get_edge_type_permissions(&v2);
            } else if tc.knows(&v4) {
                result |= tc.get_edge_type_permissions(&v4);
            } else {
                let mut guessed: SVCPermissions = SVC_IGNORING;
                let mut desc = String::new();
                if let Some(t) = self.myVSysTypes.get(&v) {
                    desc = t.name.clone();
                    if desc.contains("taxi") {
                        guessed = SVC_TAXI;
                    } else if desc.contains("bus") {
                        guessed = SVC_BUS;
                    } else if desc.contains("seilbahn") || desc.contains("funiculaire") || desc.contains("cable") || desc.contains("funicular") {
                        guessed = SVC_CABLE_CAR;
                    } else if desc.contains("subway") || desc.contains("ubahn") || desc.contains("u-bahn") {
                        guessed = SVC_SUBWAY;
                    } else if desc.contains("train") || desc.contains("schiene") || desc.contains("rail") || desc.contains("bahn") || desc.contains("zug") {
                        guessed = SVC_RAIL;
                    } else if desc.contains("tram") || desc.contains("strab") {
                        guessed = SVC_TRAM;
                    } else if desc.contains("moto") {
                        guessed = SVC_MOTORCYCLE;
                    } else if desc.contains("bike") || desc.contains("velo") || desc.contains("bicycle") || desc.contains("rad") {
                        guessed = SVC_BICYCLE;
                    } else if desc.contains("foot") || desc.contains("ped") || desc.contains("fu\u{df}") || desc.contains("fuss") || desc.contains("walk") || desc.contains("pied") {
                        guessed = SVC_PEDESTRIAN;
                    } else if desc.contains("lkw") || desc.contains("truck") || desc.contains("camion") {
                        guessed = SVC_TRUCK;
                    } else if desc.contains("pkw") || desc.contains("car") || desc.contains("auto") {
                        guessed = SVC_PASSENGER;
                    }
                }
                if guessed == SVC_IGNORING {
                    write_warningf!("Encountered unknown vehicle category '%' in type '%' (description: '%')", v3.clone(), edgeType.to_string(), desc.clone());
                    guessed = unknown;
                } else {
                    write_warningf!("Encountered unknown vehicle category '%' in type '%' (guessed '%' based on description '%')", v3.clone(), edgeType.to_string(), get_vehicle_class_names(guessed, false), desc.clone());
                }
                tc.insert_edge_type(&v4, 1, 1.0, -1, guessed, LaneSpreadFunction::RIGHT, UNSPECIFIED_WIDTH, false, UNSPECIFIED_WIDTH, UNSPECIFIED_WIDTH, 0.0, 0.0, 0.0);
                result |= guessed;
            }
        }
        result
    }

    /// C++: `bool NIImporter_VISUM::getWeightedBool(const std::string &)` [private]
    /// tries to get a bool which is possibly assigned to a certain modality
    // bool
    // NIImporter_VISUM::getWeightedBool(const std::string& name) {
    //     try {
    //         return StringUtils::toBool(myLineParser.get(name));
    //     } catch (...) {}
    //     try {
    //         return StringUtils::toBool(myLineParser.get((name + "(IV)")));
    //     } catch (...) {}
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn get_weighted_bool(&mut self, name: &str) -> bool {
        if let Ok(v) = self.myLineParser.get(name, false).and_then(|v| to_bool(&v)) {
            return v;
        }
        if let Ok(v) = self.myLineParser.get(&format!("{name}(IV)"), false).and_then(|v| to_bool(&v)) {
            return v;
        }
        false
    }

    /// C++: `NBNode * NIImporter_VISUM::getNamedNode(const std::string &)` [private]
    /// Tries to get the node which name is stored in the given field
    // NBNode*
    // NIImporter_VISUM::getNamedNode(const std::string& fieldName) {
    //     std::string nodeS = NBHelpers::normalIDRepresentation(myLineParser.get(fieldName));
    //     NBNode* node = myNetBuilder.getNodeCont().retrieve(nodeS);
    //     if (node == nullptr) {
    //         WRITE_ERRORF(TL("The node '%' is not known."), nodeS);
    //     }
    //     return node;
    // }
    #[allow(unused_variables)]
    pub fn get_named_node(&mut self, fieldName: &str) -> Ptr<NBNode> {
        let nodeS = normal_id(&self.field(fieldName));
        let node = self.myNetBuilder.m().get_node_cont().retrieve(&nodeS);
        if node.is_null() {
            write_errorf!("The node '%' is not known.", nodeS);
        }
        node
    }

    /// C++: `NBNode * NIImporter_VISUM::getNamedNodeSecure(const std::string &, NBNode *)` [private]
    // NBNode*
    // NIImporter_VISUM::getNamedNodeSecure(const std::string& fieldName, NBNode* fallback) {
    //     std::string nodeS = NBHelpers::normalIDRepresentation(myLineParser.get(fieldName));
    //     NBNode* node = myNetBuilder.getNodeCont().retrieve(nodeS);
    //     if (node == nullptr) {
    //         return fallback;
    //     }
    //     return node;
    // }
    #[allow(unused_variables)]
    pub fn get_named_node_secure(&mut self, fieldName: &str, fallback: Ptr<NBNode>) -> Ptr<NBNode> {
        let nodeS = normal_id(&self.field(fieldName));
        let node = self.myNetBuilder.m().get_node_cont().retrieve(&nodeS);
        if node.is_null() {
            return fallback;
        }
        node
    }

    /// C++: `NBNode * NIImporter_VISUM::getNamedNode(const std::string &, const std::string &)` [private]
    /// The same, but two different names for the field are allowed
    // NBNode*
    // NIImporter_VISUM::getNamedNode(const std::string& fieldName1, const std::string& fieldName2) {
    //     if (myLineParser.know(fieldName1)) {
    //         return getNamedNode(fieldName1);
    //     } else {
    //         return getNamedNode(fieldName2);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_named_node_2(&mut self, fieldName1: &str, fieldName2: &str) -> Ptr<NBNode> {
        if self.myLineParser.know(fieldName1) {
            self.get_named_node(fieldName1)
        } else {
            self.get_named_node(fieldName2)
        }
    }

    /// C++: `NBEdge * NIImporter_VISUM::getNamedEdge(const std::string &)` [private]
    /// Tries to get the edge which name is stored in the given field
    // NBEdge*
    // NIImporter_VISUM::getNamedEdge(const std::string& fieldName) {
    //     std::string edgeS = NBHelpers::normalIDRepresentation(myLineParser.get(fieldName));
    //     NBEdge* edge = myNetBuilder.getEdgeCont().retrieve(edgeS);
    //     if (edge == nullptr) {
    //         WRITE_ERRORF(TL("The edge '%' is not known."), edgeS);
    //     }
    //     return edge;
    // }
    #[allow(unused_variables)]
    pub fn get_named_edge(&mut self, fieldName: &str) -> Ptr<NBEdge> {
        let edgeS = normal_id(&self.field(fieldName));
        let edge = self.myNetBuilder.m().get_edge_cont().retrieve(&edgeS, false);
        if edge.is_null() {
            write_errorf!("The edge '%' is not known.", edgeS);
        }
        edge
    }

    /// C++: `NBEdge * NIImporter_VISUM::getNamedEdge(const std::string &, const std::string &)` [private]
    /// The same, but two different names for the field are allowed
    // NBEdge*
    // NIImporter_VISUM::getNamedEdge(const std::string& fieldName1, const std::string& fieldName2) {
    //     if (myLineParser.know(fieldName1)) {
    //         return getNamedEdge(fieldName1);
    //     } else {
    //         return getNamedEdge(fieldName2);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_named_edge_2(&mut self, fieldName1: &str, fieldName2: &str) -> Ptr<NBEdge> {
        if self.myLineParser.know(fieldName1) {
            self.get_named_edge(fieldName1)
        } else {
            self.get_named_edge(fieldName2)
        }
    }

    /// C++: `NBEdge * NIImporter_VISUM::getNamedEdgeContinuating(const std::string &, NBNode *)` [private]
    /// Tries to get the edge which name is stored in the given field continuating the search for a subedge that ends at the given node
    // NBEdge*
    // NIImporter_VISUM::getNamedEdgeContinuating(const std::string& fieldName, NBNode* node) {
    //     std::string edgeS = NBHelpers::normalIDRepresentation(myLineParser.get(fieldName));
    //     NBEdge* edge = myNetBuilder.getEdgeCont().retrieve(edgeS);
    //     if (edge == nullptr) {
    //         WRITE_ERRORF(TL("The edge '%' is not known."), edgeS);
    //     }
    //     return getNamedEdgeContinuating(edge, node);
    // }
    #[allow(unused_variables)]
    pub fn get_named_edge_continuating(&mut self, fieldName: &str, node: Ptr<NBNode>) -> Ptr<NBEdge> {
        let edgeS = normal_id(&self.field(fieldName));
        let edge = self.myNetBuilder.m().get_edge_cont().retrieve(&edgeS, false);
        if edge.is_null() {
            write_errorf!("The edge '%' is not known.", edgeS);
        }
        self.get_named_edge_continuating_3(edge, node)
    }

    /// C++: `NBEdge * NIImporter_VISUM::getNamedEdgeContinuating(const std::string &, const std::string &, NBNode *)` [private]
    /// The same, but two different names for the field are allowed
    // NBEdge*
    // NIImporter_VISUM::getNamedEdgeContinuating(const std::string& fieldName1, const std::string& fieldName2,
    //         NBNode* node) {
    //     if (myLineParser.know(fieldName1)) {
    //         return getNamedEdgeContinuating(fieldName1, node);
    //     } else {
    //         return getNamedEdgeContinuating(fieldName2, node);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_named_edge_continuating_2(&mut self, fieldName1: &str, fieldName2: &str, node: Ptr<NBNode>) -> Ptr<NBEdge> {
        if self.myLineParser.know(fieldName1) {
            self.get_named_edge_continuating(fieldName1, node)
        } else {
            self.get_named_edge_continuating(fieldName2, node)
        }
    }

    /// C++: `NBEdge * NIImporter_VISUM::getNamedEdgeContinuating(NBEdge *, NBNode *)` [private]
    /// The same, but for an already given edge
    // NBEdge*
    // NIImporter_VISUM::getNamedEdgeContinuating(NBEdge* begin, NBNode* node) {
    //     if (begin == nullptr) {
    //         return nullptr;
    //     }
    //     NBEdge* ret = begin;
    //     std::string edgeID = ret->getID();
    //     // hangle forward
    //     while (ret != nullptr) {
    //         // ok, this is the edge we are looking for
    //         if (ret->getToNode() == node) {
    //             return ret;
    //         }
    //         const EdgeVector& nedges = ret->getToNode()->getOutgoingEdges();
    //         if (nedges.size() != 1) {
    //             // too many edges follow
    //             ret = nullptr;
    //             continue;
    //         }
    //         NBEdge* next = nedges[0];
    //         if (ret->getID().substr(0, edgeID.length()) != next->getID().substr(0, edgeID.length())) {
    //             // ok, another edge is next...
    //             ret = nullptr;
    //             continue;
    //         }
    //         if (next->getID().substr(next->getID().length() - node->getID().length()) != node->getID()) {
    //             ret = nullptr;
    //             continue;
    //         }
    //         ret = next;
    //     }
    //
    //     ret = begin;
    //     // hangle backward
    //     while (ret != nullptr) {
    //         // ok, this is the edge we are looking for
    //         if (ret->getFromNode() == node) {
    //             return ret;
    //         }
    //         const EdgeVector& nedges = ret->getFromNode()->getIncomingEdges();
    //         if (nedges.size() != 1) {
    //             // too many edges follow
    //             ret = nullptr;
    //             continue;
    //         }
    //         NBEdge* next = nedges[0];
    //         if (ret->getID().substr(0, edgeID.length()) != next->getID().substr(0, edgeID.length())) {
    //             // ok, another edge is next...
    //             ret = nullptr;
    //             continue;
    //         }
    //         if (next->getID().substr(next->getID().length() - node->getID().length()) != node->getID()) {
    //             ret = nullptr;
    //             continue;
    //         }
    //         ret = next;
    //     }
    //     return nullptr;
    // }
    #[allow(unused_variables)]
    pub fn get_named_edge_continuating_3(&mut self, begin: Ptr<NBEdge>, node: Ptr<NBNode>) -> Ptr<NBEdge> {
        if begin.is_null() {
            return Ptr::null();
        }
        let mut ret = begin;
        let edgeID = ret.get_id().to_string();
        let nodeID = node.get_id().to_string();
        // C++ substr(0, n) and substr(size - n) on the edge ids
        let prefix = |s: &str| -> Vec<u8> { s.as_bytes()[..edgeID.len().min(s.len())].to_vec() };
        let suffix = |s: &str| -> Vec<u8> { s.as_bytes()[s.len().saturating_sub(nodeID.len())..].to_vec() };
        // hangle forward
        while !ret.is_null() {
            // ok, this is the edge we are looking for
            if ret.get_to_node() == node {
                return ret;
            }
            let nedges = ret.get_to_node().m().get_outgoing_edges().clone();
            if nedges.len() != 1 {
                // too many edges follow
                ret = Ptr::null();
                continue;
            }
            let next = nedges[0];
            if prefix(ret.get_id()) != prefix(next.get_id()) {
                // ok, another edge is next...
                ret = Ptr::null();
                continue;
            }
            if suffix(next.get_id()) != nodeID.as_bytes() {
                ret = Ptr::null();
                continue;
            }
            ret = next;
        }

        ret = begin;
        // hangle backward
        while !ret.is_null() {
            // ok, this is the edge we are looking for
            if ret.get_from_node() == node {
                return ret;
            }
            let nedges = ret.get_from_node().m().get_incoming_edges().clone();
            if nedges.len() != 1 {
                // too many edges follow
                ret = Ptr::null();
                continue;
            }
            let next = nedges[0];
            if prefix(ret.get_id()) != prefix(next.get_id()) {
                // ok, another edge is next...
                ret = Ptr::null();
                continue;
            }
            if suffix(next.get_id()) != nodeID.as_bytes() {
                ret = Ptr::null();
                continue;
            }
            ret = next;
        }
        Ptr::null()
    }

    /// C++: `NBEdge * NIImporter_VISUM::getEdge(NBNode *, NBNode *)` [private]
    /// Returns the edge that connects both nodes
    // NBEdge*
    // NIImporter_VISUM::getEdge(NBNode* FromNode, NBNode* ToNode) {
    //     EdgeVector::const_iterator i;
    //     for (i = FromNode->getOutgoingEdges().begin(); i != FromNode->getOutgoingEdges().end(); i++) {
    //         if (ToNode == (*i)->getToNode()) {
    //             return (*i);
    //         }
    //     }
    //     //!!!
    //     return nullptr;
    // }
    #[allow(unused_variables)]
    pub fn get_edge(&mut self, FromNode: Ptr<NBNode>, ToNode: Ptr<NBNode>) -> Ptr<NBEdge> {
        let edges = FromNode.get_outgoing_edges().clone();
        for e in edges {
            if ToNode == e.get_to_node() {
                return e;
            }
        }
        //!!!
        Ptr::null()
    }

    /// C++: `NBEdge * NIImporter_VISUM::getReversedContinuating(NBEdge *, NBNode *)` [private]
    /// Returns the opposite direction of the given edge
    // NBEdge*
    // NIImporter_VISUM::getReversedContinuating(NBEdge* edge, NBNode* node) {
    //     std::string sid;
    //     if (edge->getID()[0] == '-') {
    //         sid = edge->getID().substr(1);
    //     } else {
    //         sid = "-" + edge->getID();
    //     }
    //     if (sid.find('_') != std::string::npos) {
    //         sid = sid.substr(0, sid.find('_'));
    //     }
    //     return getNamedEdgeContinuating(myNetBuilder.getEdgeCont().retrieve(sid),  node);
    // }
    #[allow(unused_variables)]
    pub fn get_reversed_continuating(&mut self, edge: Ptr<NBEdge>, node: Ptr<NBNode>) -> Ptr<NBEdge> {
        let mut sid: String = if edge.get_id().starts_with('-') {
            edge.get_id()[1..].to_string()
        } else {
            format!("-{}", edge.get_id())
        };
        if let Some(p) = sid.find('_') {
            sid.truncate(p);
        }
        let e = self.myNetBuilder.m().get_edge_cont().retrieve(&sid, false);
        self.get_named_edge_continuating_3(e, node)
    }

    /// C++: `NBNode * NIImporter_VISUM::buildDistrictNode(const std::string &, NBNode *, bool)` [private]
    /// Builds a node for the given district and returns it
    // NBNode*
    // NIImporter_VISUM::buildDistrictNode(const std::string& id, NBNode* dest,
    //                                     bool isSource) {
    //     // get the district
    //     NBDistrict* dist = myNetBuilder.getDistrictCont().retrieve(id);
    //     if (dist == nullptr) {
    //         return nullptr;
    //     }
    //     // build the id
    //     std::string nid;
    //     nid = id + "-" + dest->getID();
    //     if (!isSource) {
    //         nid = "-" + nid;
    //     }
    //     // insert the node
    //     if (!myNetBuilder.getNodeCont().insert(nid, dist->getPosition())) {
    //         WRITE_ERRORF(TL("Could not build connector node '%'."), nid);
    //     }
    //     // return the node
    //     return myNetBuilder.getNodeCont().retrieve(nid);
    // }
    #[allow(unused_variables)]
    pub fn build_district_node(&mut self, id: &str, dest: Ptr<NBNode>, isSource: bool) -> Ptr<NBNode> {
        // get the district
        let dist = self.myNetBuilder.m().get_district_cont().retrieve(id);
        if dist.is_null() {
            return Ptr::null();
        }
        // build the id
        let mut nid = format!("{}-{}", id, dest.get_id());
        if !isSource {
            nid = format!("-{nid}");
        }
        // insert the node
        if !self.myNetBuilder.m().get_node_cont().insert(&nid, dist.get_position(), Ptr::null()) {
            write_errorf!("Could not build connector node '%'.", nid.clone());
        }
        // return the node
        self.myNetBuilder.m().get_node_cont().retrieve(&nid)
    }

    /// C++: `bool NIImporter_VISUM::checkNodes(NBNode *, NBNode *)` [private]
    /// Returns whether both nodes are a valid combination of from/to-nodes
    // bool
    // NIImporter_VISUM::checkNodes(NBNode* from, NBNode* to)  {
    //     if (from == nullptr) {
    //         WRITE_ERROR(TL(" The from-node was not found within the net"));
    //     }
    //     if (to == nullptr) {
    //         WRITE_ERROR(TL(" The to-node was not found within the net"));
    //     }
    //     if (from == to) {
    //         WRITE_ERROR(TL(" Both nodes are the same"));
    //     }
    //     return from != nullptr && to != nullptr && from != to;
    // }
    #[allow(unused_variables)]
    pub fn check_nodes(&mut self, from: Ptr<NBNode>, to: Ptr<NBNode>) -> bool {
        if from.is_null() {
            write_error(" The from-node was not found within the net");
        }
        if to.is_null() {
            write_error(" The to-node was not found within the net");
        }
        if from == to {
            write_error(" Both nodes are the same");
        }
        !from.is_null() && !to.is_null() && from != to
    }

    /// C++: `void NIImporter_VISUM::parse_Network()` [private]
    /// Parses meta data (i.e. projection)
    // void
    // NIImporter_VISUM::parse_Network() {
    //     const std::string key = KEYS.getString(VISUM_PROJECTIONDEFINITION);
    //     if (myLineParser.know(key)) {
    //         std::string proj = myLineParser.get(key);
    //         // visum network contains projected geometry, we only need the projection info computing lon, lat outputs
    //         GeoConvHelper* result = new GeoConvHelper(proj, Position(0, 0), Boundary(0, 0, 1, 1), Boundary(0, 0, 1, 1));
    //         GeoConvHelper::setLoaded(*result);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_network(&mut self) {
        let key = keys_string(K::VISUM_PROJECTIONDEFINITION);
        if self.myLineParser.know(&key) {
            let proj = self.field(&key);
            // visum network contains projected geometry, we only need the projection info computing lon, lat outputs
            let result = GeoConvHelper::new(&proj, Position::new(0.0, 0.0), Boundary::from_xy(0.0, 0.0, 1.0, 1.0), Boundary::from_xy(0.0, 0.0, 1.0, 1.0), 1.0, 0.0, false, false);
            GeoConvHelper::set_loaded(&result);
        }
    }

    /// C++: `void NIImporter_VISUM::parse_VSysTypes()` [private]
    /// Parses VSYS
    // void
    // NIImporter_VISUM::parse_VSysTypes() {
    //     std::string code = myLineParser.know("VSysCode") ? myLineParser.get("VSysCode").c_str() : myLineParser.get(KEYS.getString(VISUM_CODE));
    //     std::string name = myLineParser.get(KEYS.getString(VISUM_NAME)).c_str();
    //     std::string type = myLineParser.know("VSysMode") ? myLineParser.get("VSysMode").c_str() : myLineParser.get(KEYS.getString(VISUM_TYP));
    //     myVSysTypes.emplace(code, VSysType(type, StringUtils::to_lower_case(name)));
    // }
    #[allow(unused_variables)]
    pub fn parse_v_sys_types(&mut self) {
        let code = if self.myLineParser.know("VSysCode") { self.field("VSysCode") } else { self.field(&keys_string(K::VISUM_CODE)) };
        let name = self.field(&keys_string(K::VISUM_NAME));
        let type_ = if self.myLineParser.know("VSysMode") { self.field("VSysMode") } else { self.field(&keys_string(K::VISUM_TYP)) };
        self.myVSysTypes.entry(code).or_insert_with(|| NIImporter_VISUM_VSysType::new(&type_, &to_lower_case(&name)));
    }

    /// C++: `void NIImporter_VISUM::parse_Types()` [private]
    /// Parses STRECKENTYP
    // void
    // NIImporter_VISUM::parse_Types() {
    //     // get the id
    //     myCurrentID = NBHelpers::normalIDRepresentation(myLineParser.get(KEYS.getString(VISUM_NO)));
    //     // get the maximum speed
    //     double speed = getWeightedFloat2("v0-IV", KEYS.getString(VISUM_V0), "km/h");
    //     if (speed == 0) {
    //         // unlimited speed
    //         speed = 3600;
    //     } else if (speed < 0) {
    //         WRITE_ERROR("Type '" + myCurrentID + "' has speed " + toString(speed));
    //     }
    //     // get the permissions
    //     SVCPermissions permissions = getPermissions(KEYS.getString(VISUM_TYPES), myCurrentID, myDefaultPermissions);
    //     // get the priority
    //     const int priority = 1000 - StringUtils::toInt(myLineParser.get(KEYS.getString(VISUM_RANK)));
    //     // try to retrieve the number of lanes
    //     const int numLanes = myCapacity2Lanes.get(getNamedFloat("Kap-IV", KEYS.getString(VISUM_CAPACITY)));
    //     // insert the type
    //     myNetBuilder.getTypeCont().insertEdgeType(myCurrentID, numLanes, speed / (double) 3.6, priority, permissions, LaneSpreadFunction::RIGHT,
    //             NBEdge::UNSPECIFIED_WIDTH, false, NBEdge::UNSPECIFIED_WIDTH, NBEdge::UNSPECIFIED_WIDTH, 0, 0, 0);
    //     myNetBuilder.getTypeCont().markEdgeTypeAsSet(myCurrentID, SUMO_ATTR_NUMLANES);
    //     myNetBuilder.getTypeCont().markEdgeTypeAsSet(myCurrentID, SUMO_ATTR_SPEED);
    //     myNetBuilder.getTypeCont().markEdgeTypeAsSet(myCurrentID, SUMO_ATTR_PRIORITY);
    //     myNetBuilder.getTypeCont().markEdgeTypeAsSet(myCurrentID, SUMO_ATTR_ONEWAY);
    //     myNetBuilder.getTypeCont().markEdgeTypeAsSet(myCurrentID, SUMO_ATTR_ALLOW);
    //     myNetBuilder.getTypeCont().markEdgeTypeAsSet(myCurrentID, SUMO_ATTR_SPREADTYPE);
    // }
    #[allow(unused_variables)]
    pub fn parse_types(&mut self) {
        // get the id
        self.myCurrentID = normal_id(&self.field(&keys_string(K::VISUM_NO)));
        // get the maximum speed
        let mut speed = self.get_weighted_float2("v0-IV", &keys_string(K::VISUM_V0), "km/h");
        if speed == 0.0 {
            // unlimited speed
            speed = 3600.0;
        } else if speed < 0.0 {
            write_error(&format!("Type '{}' has speed {}", self.myCurrentID, f64_to_string(speed)));
        }
        // get the permissions
        let key = keys_string(K::VISUM_TYPES);
        let cur = self.myCurrentID.clone();
        let dp = self.myDefaultPermissions;
        let permissions = self.get_permissions(&key, &cur, dp);
        // get the priority
        let priority = 1000 - throw(to_int(&self.field(&keys_string(K::VISUM_RANK))));
        // try to retrieve the number of lanes
        let cap = self.get_named_float_2("Kap-IV", &keys_string(K::VISUM_CAPACITY));
        let numLanes = self.myCapacity2Lanes.get(cap);
        // insert the type
        let tc = self.myNetBuilder.m().get_type_cont();
        tc.insert_edge_type(&self.myCurrentID, numLanes, speed / 3.6, priority, permissions, LaneSpreadFunction::RIGHT, UNSPECIFIED_WIDTH, false, UNSPECIFIED_WIDTH, UNSPECIFIED_WIDTH, 0.0, 0.0, 0.0);
        tc.mark_edge_type_as_set(&self.myCurrentID, SUMO_ATTR_NUMLANES);
        tc.mark_edge_type_as_set(&self.myCurrentID, SUMO_ATTR_SPEED);
        tc.mark_edge_type_as_set(&self.myCurrentID, SUMO_ATTR_PRIORITY);
        tc.mark_edge_type_as_set(&self.myCurrentID, SUMO_ATTR_ONEWAY);
        tc.mark_edge_type_as_set(&self.myCurrentID, SUMO_ATTR_ALLOW);
        tc.mark_edge_type_as_set(&self.myCurrentID, SUMO_ATTR_SPREADTYPE);
    }

    /// C++: `void NIImporter_VISUM::parse_Nodes()` [private]
    /// Parses KNOTEN
    // void
    // NIImporter_VISUM::parse_Nodes() {
    //     // get the id
    //     myCurrentID = NBHelpers::normalIDRepresentation(myLineParser.get(KEYS.getString(VISUM_NO)));
    //     // get the position
    //     double x = getNamedFloat(KEYS.getString(VISUM_XCOORD));
    //     double y = getNamedFloat(KEYS.getString(VISUM_YCOORD));
    //     Position pos(x, y);
    //     if (!NBNetBuilder::transformCoordinate(pos)) {
    //         WRITE_ERRORF(TL("Unable to project coordinates for node %."), myCurrentID);
    //         return;
    //     }
    //     // add to the list
    //     if (!myNetBuilder.getNodeCont().insert(myCurrentID, pos)) {
    //         WRITE_ERRORF(TL("Duplicate node occurred ('%')."), myCurrentID);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_nodes(&mut self) {
        // get the id
        self.myCurrentID = normal_id(&self.field(&keys_string(K::VISUM_NO)));
        // get the position
        let x = self.get_named_float(&keys_string(K::VISUM_XCOORD));
        let y = self.get_named_float(&keys_string(K::VISUM_YCOORD));
        let mut pos = Position::new(x, y);
        if !NBNetBuilder::transform_coordinate(&mut pos, true, Ptr::null()) {
            write_errorf!("Unable to project coordinates for node %.", self.myCurrentID.clone());
            return;
        }
        // add to the list
        if !self.myNetBuilder.m().get_node_cont().insert(&self.myCurrentID, &pos, Ptr::null()) {
            write_errorf!("Duplicate node occurred ('%').", self.myCurrentID.clone());
        }
    }

    /// C++: `void NIImporter_VISUM::parse_Districts()` [private]
    /// Parses BEZIRK
    // void
    // NIImporter_VISUM::parse_Districts() {
    //     // get the id
    //     myCurrentID = NBHelpers::normalIDRepresentation(myLineParser.get(KEYS.getString(VISUM_NO)));
    //     // get the information whether the source and the destination
    //     //  connections are weighted
    //     //bool sourcesWeighted = getWeightedBool("Proz_Q");
    //     //bool destWeighted = getWeightedBool("Proz_Z");
    //     // get the node information
    //     double x = getNamedFloat(KEYS.getString(VISUM_XCOORD));
    //     double y = getNamedFloat(KEYS.getString(VISUM_YCOORD));
    //     Position pos(x, y);
    //     if (!NBNetBuilder::transformCoordinate(pos, false)) {
    //         WRITE_ERRORF(TL("Unable to project coordinates for district %."), myCurrentID);
    //         return;
    //     }
    //     // build the district
    //     NBDistrict* district = new NBDistrict(myCurrentID, pos);
    //     if (!myNetBuilder.getDistrictCont().insert(district)) {
    //         WRITE_ERRORF(TL("Duplicate district occurred ('%')."), myCurrentID);
    //         delete district;
    //         return;
    //     }
    //     if (myLineParser.know(KEYS.getString(VISUM_SURFACEID))) {
    //         long long int flaecheID;
    //         try {
    //             flaecheID = StringUtils::toLong(myLineParser.get(KEYS.getString(VISUM_SURFACEID)));
    //         } catch (EmptyData&) {
    //             flaecheID = StringUtils::toLong(myLineParser.get(KEYS.getString(VISUM_NO)));
    //         }
    //         myShapeDistrictMap[flaecheID] = district;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_districts(&mut self) {
        // get the id
        self.myCurrentID = normal_id(&self.field(&keys_string(K::VISUM_NO)));
        // get the node information
        let x = self.get_named_float(&keys_string(K::VISUM_XCOORD));
        let y = self.get_named_float(&keys_string(K::VISUM_YCOORD));
        let mut pos = Position::new(x, y);
        if !NBNetBuilder::transform_coordinate(&mut pos, false, Ptr::null()) {
            write_errorf!("Unable to project coordinates for district %.", self.myCurrentID.clone());
            return;
        }
        // build the district
        let district = Box::into_raw(Box::new(NBDistrict::new(&self.myCurrentID, &pos)));
        if !self.myNetBuilder.m().get_district_cont().insert(Ptr::from_raw(district)) {
            write_errorf!("Duplicate district occurred ('%').", self.myCurrentID.clone());
            // SAFETY: the district was not inserted, so this is the only owner
            drop(unsafe { Box::from_raw(district) });
            return;
        }
        if self.myLineParser.know(&keys_string(K::VISUM_SURFACEID)) {
            let flaecheID: i64 = match to_long(&self.field(&keys_string(K::VISUM_SURFACEID))) {
                Ok(v) => v,
                Err(ProcessError::EmptyData) => throw(to_long(&self.field(&keys_string(K::VISUM_NO)))),
                Err(e) => std::panic::panic_any(e),
            };
            self.myShapeDistrictMap.insert(flaecheID, Ptr::from_raw(district));
        }
    }

    /// C++: `void NIImporter_VISUM::parse_Point()` [private]
    /// Parses PUNKT
    // void
    // NIImporter_VISUM::parse_Point() {
    //     long long int id = StringUtils::toLong(myLineParser.get(KEYS.getString(VISUM_ID)));
    //     double x = StringUtils::toDouble(myLineParser.get(KEYS.getString(VISUM_XCOORD)));
    //     double y = StringUtils::toDouble(myLineParser.get(KEYS.getString(VISUM_YCOORD)));
    //     Position pos(x, y);
    //     if (!NBNetBuilder::transformCoordinate(pos, false)) {
    //         WRITE_ERRORF(TL("Unable to project coordinates for point %."), toString(id));
    //         return;
    //     }
    //     myPoints[id] = pos;
    // }
    #[allow(unused_variables)]
    pub fn parse_point(&mut self) {
        let id = throw(to_long(&self.field(&keys_string(K::VISUM_ID))));
        let x = throw(to_double(&self.field(&keys_string(K::VISUM_XCOORD))));
        let y = throw(to_double(&self.field(&keys_string(K::VISUM_YCOORD))));
        let mut pos = Position::new(x, y);
        if !NBNetBuilder::transform_coordinate(&mut pos, false, Ptr::null()) {
            write_errorf!("Unable to project coordinates for point %.", id);
            return;
        }
        self.myPoints.insert(id, pos);
    }

    /// C++: `void NIImporter_VISUM::parse_Edges()` [private]
    /// Parses STRECKE/STRECKEN
    // void
    // NIImporter_VISUM::parse_Edges() {
    //     if (myLineParser.know(KEYS.getString(VISUM_TYPES)) && myLineParser.get(KEYS.getString(VISUM_TYPES)) == "") {
    //         // no vehicle allowed; don't add
    //         return;
    //     }
    //     // get the id
    //     myCurrentID = NBHelpers::normalIDRepresentation(myLineParser.get(KEYS.getString(VISUM_NO)));
    //     // get the from- & to-node and validate them
    //     NBNode* from = getNamedNode("VonKnot", KEYS.getString(VISUM_FROMNODE));
    //     NBNode* to = getNamedNode("NachKnot", KEYS.getString(VISUM_TONODE));
    //     if (!checkNodes(from, to)) {
    //         return;
    //     }
    //     // get the type
    //     std::string type = myLineParser.know(KEYS.getString(VISUM_TYP)) ? myLineParser.get(KEYS.getString(VISUM_TYP)) : myLineParser.get(KEYS.getString(VISUM_TYPE));
    //     // get the speed
    //     double speed = myNetBuilder.getTypeCont().getEdgeTypeSpeed(type);
    //     if (!OptionsCont::getOptions().getBool("visum.use-type-speed")) {
    //         try {
    //             std::string speedS = myLineParser.know("v0-IV") ? myLineParser.get("v0-IV") : myLineParser.get(KEYS.getString(VISUM_V0));
    //             if (speedS.find("km/h") != std::string::npos) {
    //                 speedS = speedS.substr(0, speedS.find("km/h"));
    //             }
    //             speed = StringUtils::toDouble(speedS) / 3.6;
    //         } catch (OutOfBoundsException&) {}
    //     }
    //     if (speed <= 0) {
    //         speed = myNetBuilder.getTypeCont().getEdgeTypeSpeed(type);
    //     }
    //
    //     // get the number of lanes
    //     int nolanes = myNetBuilder.getTypeCont().getEdgeTypeNumLanes(type);
    //     if (!OptionsCont::getOptions().getBool("visum.recompute-lane-number")) {
    //         if (!OptionsCont::getOptions().getBool("visum.use-type-laneno")) {
    //             if (myLineParser.know("Fahrstreifen")) {
    //                 nolanes = StringUtils::toInt(myLineParser.get("Fahrstreifen"));
    //             } else if (myLineParser.know(KEYS.getString(VISUM_NUMLANES))) {
    //                 nolanes = StringUtils::toInt(myLineParser.get(KEYS.getString(VISUM_NUMLANES)));
    //             }
    //         }
    //     } else {
    //         if (myLineParser.know(KEYS.getString(VISUM_CAPACITY))) {
    //             nolanes = myCapacity2Lanes.get(StringUtils::toDouble(myLineParser.get(KEYS.getString(VISUM_CAPACITY))));
    //         } else if (myLineParser.know("KAP-IV")) {
    //             nolanes = myCapacity2Lanes.get(StringUtils::toDouble(myLineParser.get("KAP-IV")));
    //         }
    //     }
    //     // check whether the id is already used
    //     //  (should be the opposite direction)
    //     bool oneway_checked = true;
    //     NBEdge* previous = myNetBuilder.getEdgeCont().retrieve(myCurrentID);
    //     if (previous != nullptr) {
    //         myCurrentID = '-' + myCurrentID;
    //         previous->setLaneSpreadFunction(LaneSpreadFunction::RIGHT);
    //         oneway_checked = false;
    //     }
    //     if (find(myTouchedEdges.begin(), myTouchedEdges.end(), myCurrentID) != myTouchedEdges.end()) {
    //         oneway_checked = false;
    //     }
    //     std::string tmpid = '-' + myCurrentID;
    //     if (find(myTouchedEdges.begin(), myTouchedEdges.end(), tmpid) != myTouchedEdges.end()) {
    //         previous = myNetBuilder.getEdgeCont().retrieve(tmpid);
    //         if (previous != nullptr) {
    //             previous->setLaneSpreadFunction(LaneSpreadFunction::RIGHT);
    //         }
    //         oneway_checked = false;
    //     }
    //     std::string name = StringUtils::latin1_to_utf8(myLineParser.get(KEYS.getString(VISUM_NAME)));
    //     // add the edge
    //     const SVCPermissions permissions = getPermissions(KEYS.getString(VISUM_TYPES), type, myNetBuilder.getTypeCont().getEdgeTypePermissions(type));
    //     int prio = myUseVisumPrio ? myNetBuilder.getTypeCont().getEdgeTypePriority(type) : -1;
    //     if (nolanes != 0 && speed != 0) {
    //         LaneSpreadFunction lsf = oneway_checked ? LaneSpreadFunction::CENTER : LaneSpreadFunction::RIGHT;
    //         NBEdge* e = new NBEdge(myCurrentID, from, to, type, speed, NBEdge::UNSPECIFIED_FRICTION, nolanes, prio,
    //                                NBEdge::UNSPECIFIED_WIDTH, NBEdge::UNSPECIFIED_OFFSET, lsf, name);
    //         e->setPermissions(permissions);
    //         if (!myNetBuilder.getEdgeCont().insert(e)) {
    //             delete e;
    //             WRITE_ERRORF(TL("Duplicate edge occurred ('%')."), myCurrentID);
    //         }
    //     }
    //     myTouchedEdges.push_back(myCurrentID);
    // }
    #[allow(unused_variables)]
    pub fn parse_edges(&mut self) {
        if self.myLineParser.know(&keys_string(K::VISUM_TYPES)) && self.field(&keys_string(K::VISUM_TYPES)) == "" {
            // no vehicle allowed; don't add
            return;
        }
        // get the id
        self.myCurrentID = normal_id(&self.field(&keys_string(K::VISUM_NO)));
        // get the from- & to-node and validate them
        let from = self.get_named_node_2("VonKnot", &keys_string(K::VISUM_FROMNODE));
        let to = self.get_named_node_2("NachKnot", &keys_string(K::VISUM_TONODE));
        if !self.check_nodes(from, to) {
            return;
        }
        // get the type
        let type_ = if self.myLineParser.know(&keys_string(K::VISUM_TYP)) { self.field(&keys_string(K::VISUM_TYP)) } else { self.field(&keys_string(K::VISUM_TYPE)) };
        // get the speed
        let mut speed = self.myNetBuilder.m().get_type_cont().get_edge_type_speed(&type_);
        if !opt_bool("visum.use-type-speed") {
            let speed_s = if self.myLineParser.know("v0-IV") { self.myLineParser.get("v0-IV", false) } else { self.myLineParser.get(&keys_string(K::VISUM_V0), false) };
            match speed_s {
                Ok(mut speedS) => {
                    if let Some(p) = speedS.find("km/h") {
                        speedS.truncate(p);
                    }
                    speed = throw(to_double(&speedS)) / 3.6;
                }
                Err(ProcessError::OutOfBounds(_)) => {}
                Err(e) => std::panic::panic_any(e),
            }
        }
        if speed <= 0.0 {
            speed = self.myNetBuilder.m().get_type_cont().get_edge_type_speed(&type_);
        }

        // get the number of lanes
        let mut nolanes = self.myNetBuilder.m().get_type_cont().get_edge_type_num_lanes(&type_);
        if !opt_bool("visum.recompute-lane-number") {
            if !opt_bool("visum.use-type-laneno") {
                if self.myLineParser.know("Fahrstreifen") {
                    nolanes = throw(to_int(&self.field("Fahrstreifen")));
                } else if self.myLineParser.know(&keys_string(K::VISUM_NUMLANES)) {
                    nolanes = throw(to_int(&self.field(&keys_string(K::VISUM_NUMLANES))));
                }
            }
        } else if self.myLineParser.know(&keys_string(K::VISUM_CAPACITY)) {
            nolanes = self.myCapacity2Lanes.get(throw(to_double(&self.field(&keys_string(K::VISUM_CAPACITY)))));
        } else if self.myLineParser.know("KAP-IV") {
            nolanes = self.myCapacity2Lanes.get(throw(to_double(&self.field("KAP-IV"))));
        }
        // check whether the id is already used
        //  (should be the opposite direction)
        let mut oneway_checked = true;
        let mut previous = self.myNetBuilder.m().get_edge_cont().retrieve(&self.myCurrentID, false);
        if !previous.is_null() {
            self.myCurrentID = format!("-{}", self.myCurrentID);
            previous.m().set_lane_spread_function(LaneSpreadFunction::RIGHT);
            oneway_checked = false;
        }
        if self.myTouchedEdges.contains(&self.myCurrentID) {
            oneway_checked = false;
        }
        let tmpid = format!("-{}", self.myCurrentID);
        if self.myTouchedEdges.contains(&tmpid) {
            previous = self.myNetBuilder.m().get_edge_cont().retrieve(&tmpid, false);
            if !previous.is_null() {
                previous.m().set_lane_spread_function(LaneSpreadFunction::RIGHT);
            }
            oneway_checked = false;
        }
        let name = latin1_to_utf8(self.field(&keys_string(K::VISUM_NAME)).as_bytes());

        // add the edge
        let key = keys_string(K::VISUM_TYPES);
        let dp = self.myNetBuilder.m().get_type_cont().get_edge_type_permissions(&type_);
        let permissions = self.get_permissions(&key, &type_, dp);
        let prio = if self.myUseVisumPrio { self.myNetBuilder.m().get_type_cont().get_edge_type_priority(&type_) } else { -1 };
        if nolanes != 0 && speed != 0.0 {
            let lsf = if oneway_checked { LaneSpreadFunction::CENTER } else { LaneSpreadFunction::RIGHT };
            let mut e = NBEdge::new(&self.myCurrentID, from, to, type_.clone(), speed, UNSPECIFIED_FRICTION, nolanes, prio, UNSPECIFIED_WIDTH, UNSPECIFIED_OFFSET, lsf, &name);
            e.set_permissions(permissions, -1);
            let eptr = Ptr::from_raw(Box::into_raw(e));
            if !self.myNetBuilder.m().get_edge_cont().insert(eptr, false) {
                // SAFETY: the edge was not inserted, so this is the only owner
                drop(unsafe { Box::from_raw(eptr.as_raw()) });
                write_errorf!("Duplicate edge occurred ('%').", self.myCurrentID.clone());
            }
        }
        self.myTouchedEdges.push(self.myCurrentID.clone());
    }

    /// C++: `void NIImporter_VISUM::parse_PartOfArea()` [private]
    /// Parses FLAECHENELEMENT
    // void
    // NIImporter_VISUM::parse_PartOfArea() {
    //     long long int flaecheID = StringUtils::toLong(myLineParser.get(KEYS.getString(VISUM_SURFACEID)));
    //     long long int flaechePartID = StringUtils::toLong(myLineParser.get(KEYS.getString(VISUM_FACEID)));
    //     if (mySubPartsAreas.find(flaechePartID) == mySubPartsAreas.end()) {
    //         mySubPartsAreas[flaechePartID] = std::vector<long long int>();
    //     }
    //     mySubPartsAreas[flaechePartID].push_back(flaecheID);
    // }
    #[allow(unused_variables)]
    pub fn parse_part_of_area(&mut self) {
        let flaecheID = throw(to_long(&self.field(&keys_string(K::VISUM_SURFACEID))));
        let flaechePartID = throw(to_long(&self.field(&keys_string(K::VISUM_FACEID))));
        self.mySubPartsAreas.entry(flaechePartID).or_default().push(flaecheID);
    }

    /// C++: `void NIImporter_VISUM::parse_Kante()` [private]
    /// Parses FLAECHENELEMENT
    // void
    // NIImporter_VISUM::parse_Kante() {
    //     long long int id = StringUtils::toLong(myLineParser.get(KEYS.getString(VISUM_ID)));
    //     long long int from = StringUtils::toLong(myLineParser.get(KEYS.getString(VISUM_FROMPOINTID)));
    //     long long int to = StringUtils::toLong(myLineParser.get(KEYS.getString(VISUM_TOPOINTID)));
    //     myEdges[id] = std::make_pair(from, to);
    // }
    #[allow(unused_variables)]
    pub fn parse_kante(&mut self) {
        let id = throw(to_long(&self.field(&keys_string(K::VISUM_ID))));
        let from = throw(to_long(&self.field(&keys_string(K::VISUM_FROMPOINTID))));
        let to = throw(to_long(&self.field(&keys_string(K::VISUM_TOPOINTID))));
        self.myEdges.insert(id, (from, to));
    }

    /// C++: `void NIImporter_VISUM::parse_Connectors()` [private]
    /// Parses ANBINDUNG
    // void
    // NIImporter_VISUM::parse_Connectors() {
    //     // get the source district
    //     std::string bez = NBHelpers::normalIDRepresentation(myLineParser.get(KEYS.getString(VISUM_SOURCE_DISTRICT)));
    //     // get the destination node
    //     NBNode* dest = getNamedNode(KEYS.getString(VISUM_FROMNODENO));
    //     if (dest == nullptr) {
    //         return;
    //     }
    //     // get the weight of the connection
    //     double proz = 1;
    //     if (myLineParser.know("Proz") || myLineParser.know("Proz(IV)")) {
    //         proz = getNamedFloat("Proz", "Proz(IV)") / 100;
    //     }
    //     // get the information whether this is a sink or a source
    //     std::string dir = myLineParser.get(KEYS.getString(VISUM_DIRECTION));
    //     if (dir.length() == 0) {
    //         dir = KEYS.getString(VISUM_ORIGIN) + KEYS.getString(VISUM_DESTINATION);
    //     }
    //     // build the source when needed
    //     if (dir.find(KEYS.getString(VISUM_ORIGIN)) != std::string::npos) {
    //         for (NBEdge* edge : dest->getOutgoingEdges()) {
    //             myNetBuilder.getDistrictCont().addSource(bez, edge, proz);
    //         }
    //     }
    //     // build the sink when needed
    //     if (dir.find(KEYS.getString(VISUM_DESTINATION)) != std::string::npos) {
    //         for (NBEdge* edge : dest->getIncomingEdges()) {
    //             myNetBuilder.getDistrictCont().addSink(bez, edge, proz);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_connectors(&mut self) {
        // get the source district
        let bez = normal_id(&self.field(&keys_string(K::VISUM_SOURCE_DISTRICT)));
        // get the destination node
        let dest = self.get_named_node(&keys_string(K::VISUM_FROMNODENO));
        if dest.is_null() {
            return;
        }
        // get the weight of the connection
        let mut proz = 1.0;
        if self.myLineParser.know("Proz") || self.myLineParser.know("Proz(IV)") {
            proz = self.get_named_float_2("Proz", "Proz(IV)") / 100.0;
        }
        // get the information whether this is a sink or a source
        let mut dir = self.field(&keys_string(K::VISUM_DIRECTION));
        if dir.is_empty() {
            dir = keys_string(K::VISUM_ORIGIN) + &keys_string(K::VISUM_DESTINATION);
        }
        // build the source when needed
        if dir.contains(&keys_string(K::VISUM_ORIGIN)) {
            for edge in dest.get_outgoing_edges().clone() {
                self.myNetBuilder.m().get_district_cont().add_source(&bez, edge, proz);
            }
        }
        // build the sink when needed
        if dir.contains(&keys_string(K::VISUM_DESTINATION)) {
            for edge in dest.get_incoming_edges().clone() {
                self.myNetBuilder.m().get_district_cont().add_sink(&bez, edge, proz);
            }
        }
    }

    /// C++: `void NIImporter_VISUM::parse_Connectors_legacy()` [private]
    // void
    // NIImporter_VISUM::parse_Connectors_legacy() {
    //     // get the source district
    //     std::string bez = NBHelpers::normalIDRepresentation(myLineParser.get(KEYS.getString(VISUM_SOURCE_DISTRICT)));
    //     // get the destination node
    //     NBNode* dest = getNamedNode(KEYS.getString(VISUM_FROMNODENO));
    //     if (dest == nullptr) {
    //         return;
    //     }
    //     // get the weight of the connection
    //     double proz = 1;
    //     if (myLineParser.know("Proz") || myLineParser.know("Proz(IV)")) {
    //         proz = getNamedFloat("Proz", "Proz(IV)") / 100;
    //     }
    //     // get the duration to wait (unused)
    // //     double retard = -1;
    // //     if (myLineParser.know("t0-IV")) {
    // //         retard = getNamedFloat("t0-IV", -1);
    // //     }
    //     // get the type;
    //     //  use a standard type with a large speed when a type is not given
    //
    //     std::string type = myLineParser.know(KEYS.getString(VISUM_TYP))
    //                        ? NBHelpers::normalIDRepresentation(myLineParser.get(KEYS.getString(VISUM_TYP)))
    //                        : "";
    //     // add the connectors as an edge
    //     std::string id = bez + "-" + dest->getID();
    //     // get the information whether this is a sink or a source
    //     std::string dir = myLineParser.get(KEYS.getString(VISUM_DIRECTION));
    //     if (dir.length() == 0) {
    //         dir = KEYS.getString(VISUM_ORIGIN) + KEYS.getString(VISUM_DESTINATION);
    //     }
    //     // build the source when needed
    //     if (dir.find(KEYS.getString(VISUM_ORIGIN)) != std::string::npos) {
    //         const EdgeVector& edges = dest->getOutgoingEdges();
    //         bool hasContinuation = false;
    //         for (EdgeVector::const_iterator i = edges.begin(); i != edges.end(); ++i) {
    //             if (!(*i)->isMacroscopicConnector()) {
    //                 hasContinuation = true;
    //             }
    //         }
    //         if (!hasContinuation) {
    //             // obviously, there is no continuation on the net
    //             WRITE_WARNINGF(TL("Incoming connector '%' will not be build - would be not connected to network."), id);
    //         } else {
    //             NBNode* src = buildDistrictNode(bez, dest, true);
    //             if (src == nullptr) {
    //                 WRITE_ERRORF(TL("The district '%' could not be built."), bez);
    //                 return;
    //             }
    //             NBEdge* edge = new NBEdge(id, src, dest, "VisumConnector",
    //                                       OptionsCont::getOptions().getFloat("visum.connector-speeds"), NBEdge::UNSPECIFIED_FRICTION,
    //                                       OptionsCont::getOptions().getInt("visum.connectors-lane-number"),
    //                                       -1, NBEdge::UNSPECIFIED_WIDTH, NBEdge::UNSPECIFIED_OFFSET,
    //                                       LaneSpreadFunction::RIGHT, "");
    //             edge->setAsMacroscopicConnector();
    //             if (!myNetBuilder.getEdgeCont().insert(edge)) {
    //                 WRITE_ERRORF(TL("A duplicate edge id occurred (ID='%')."), id);
    //                 return;
    //             }
    //             edge = myNetBuilder.getEdgeCont().retrieve(id);
    //             if (edge != nullptr) {
    //                 myNetBuilder.getDistrictCont().addSource(bez, edge, proz);
    //             }
    //         }
    //     }
    //     // build the sink when needed
    //     if (dir.find(KEYS.getString(VISUM_DESTINATION)) != std::string::npos) {
    //         const EdgeVector& edges = dest->getIncomingEdges();
    //         bool hasPredeccessor = false;
    //         for (EdgeVector::const_iterator i = edges.begin(); i != edges.end(); ++i) {
    //             if (!(*i)->isMacroscopicConnector()) {
    //                 hasPredeccessor = true;
    //             }
    //         }
    //         if (!hasPredeccessor) {
    //             // obviously, the network is not connected to this node
    //             WRITE_WARNINGF(TL("Outgoing connector '%' will not be build - would be not connected to network."), id);
    //         } else {
    //             NBNode* src = buildDistrictNode(bez, dest, false);
    //             if (src == nullptr) {
    //                 WRITE_ERRORF(TL("The district '%' could not be built."), bez);
    //                 return;
    //             }
    //             id = "-" + id;
    //             NBEdge* edge = new NBEdge(id, dest, src, "VisumConnector",
    //                                       OptionsCont::getOptions().getFloat("visum.connector-speeds"), NBEdge::UNSPECIFIED_FRICTION,
    //                                       OptionsCont::getOptions().getInt("visum.connectors-lane-number"),
    //                                       -1, NBEdge::UNSPECIFIED_WIDTH, NBEdge::UNSPECIFIED_OFFSET,
    //                                       LaneSpreadFunction::RIGHT, "");
    //             edge->setAsMacroscopicConnector();
    //             if (!myNetBuilder.getEdgeCont().insert(edge)) {
    //                 WRITE_ERRORF(TL("A duplicate edge id occurred (ID='%')."), id);
    //                 return;
    //             }
    //             edge = myNetBuilder.getEdgeCont().retrieve(id);
    //             if (edge != nullptr) {
    //                 myNetBuilder.getDistrictCont().addSink(bez, edge, proz);
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_connectors_legacy(&mut self) {
        // get the source district
        let bez = normal_id(&self.field(&keys_string(K::VISUM_SOURCE_DISTRICT)));
        // get the destination node
        let dest = self.get_named_node(&keys_string(K::VISUM_FROMNODENO));
        if dest.is_null() {
            return;
        }
        // get the weight of the connection
        let mut proz = 1.0;
        if self.myLineParser.know("Proz") || self.myLineParser.know("Proz(IV)") {
            proz = self.get_named_float_2("Proz", "Proz(IV)") / 100.0;
        }
        // get the type;
        //  use a standard type with a large speed when a type is not given
        let typ = if self.myLineParser.know(&keys_string(K::VISUM_TYP)) { normal_id(&self.field(&keys_string(K::VISUM_TYP))) } else { String::new() };
        // add the connectors as an edge
        let mut id = format!("{}-{}", bez, dest.get_id());
        // get the information whether this is a sink or a source
        let mut dir = self.field(&keys_string(K::VISUM_DIRECTION));
        if dir.is_empty() {
            dir = keys_string(K::VISUM_ORIGIN) + &keys_string(K::VISUM_DESTINATION);
        }
        // build the source when needed
        if dir.contains(&keys_string(K::VISUM_ORIGIN)) {
            let edges = dest.get_outgoing_edges().clone();
            let mut hasContinuation = false;
            for e in &edges {
                if !e.is_macroscopic_connector() {
                    hasContinuation = true;
                }
            }
            if !hasContinuation {
                // obviously, there is no continuation on the net
                write_warningf!("Incoming connector '%' will not be build - would be not connected to network.", id.clone());
            } else {
                let src = self.build_district_node(&bez, dest, true);
                if src.is_null() {
                    write_errorf!("The district '%' could not be built.", bez.clone());
                    return;
                }
                let mut edge = NBEdge::new(&id, src, dest, "VisumConnector".to_string(), opt_float("visum.connector-speeds"), UNSPECIFIED_FRICTION, opt_int("visum.connectors-lane-number"), -1, UNSPECIFIED_WIDTH, UNSPECIFIED_OFFSET, LaneSpreadFunction::RIGHT, "");
                edge.set_as_macroscopic_connector();
                let eptr = Ptr::from_raw(Box::into_raw(edge));
                if !self.myNetBuilder.m().get_edge_cont().insert(eptr, false) {
                    // SAFETY: the edge was not inserted, so this is the only owner
                    drop(unsafe { Box::from_raw(eptr.as_raw()) });
                    write_errorf!("A duplicate edge id occurred (ID='%').", id.clone());
                    return;
                }
                let edge = self.myNetBuilder.m().get_edge_cont().retrieve(&id, false);
                if !edge.is_null() {
                    self.myNetBuilder.m().get_district_cont().add_source(&bez, edge, proz);
                }
            }
        }
        // build the sink when needed
        if dir.contains(&keys_string(K::VISUM_DESTINATION)) {
            let edges = dest.get_incoming_edges().clone();
            let mut hasPredeccessor = false;
            for e in &edges {
                if !e.is_macroscopic_connector() {
                    hasPredeccessor = true;
                }
            }
            if !hasPredeccessor {
                // obviously, the network is not connected to this node
                write_warningf!("Outgoing connector '%' will not be build - would be not connected to network.", id.clone());
            } else {
                let src = self.build_district_node(&bez, dest, false);
                if src.is_null() {
                    write_errorf!("The district '%' could not be built.", bez.clone());
                    return;
                }
                id = format!("-{id}");
                let mut edge = NBEdge::new(&id, dest, src, "VisumConnector".to_string(), opt_float("visum.connector-speeds"), UNSPECIFIED_FRICTION, opt_int("visum.connectors-lane-number"), -1, UNSPECIFIED_WIDTH, UNSPECIFIED_OFFSET, LaneSpreadFunction::RIGHT, "");
                edge.set_as_macroscopic_connector();
                let eptr = Ptr::from_raw(Box::into_raw(edge));
                if !self.myNetBuilder.m().get_edge_cont().insert(eptr, false) {
                    // SAFETY: the edge was not inserted, so this is the only owner
                    drop(unsafe { Box::from_raw(eptr.as_raw()) });
                    write_errorf!("A duplicate edge id occurred (ID='%').", id.clone());
                    return;
                }
                let edge = self.myNetBuilder.m().get_edge_cont().retrieve(&id, false);
                if !edge.is_null() {
                    self.myNetBuilder.m().get_district_cont().add_sink(&bez, edge, proz);
                }
            }
        }
        let _ = typ;
    }

    /// C++: `void NIImporter_VISUM::parse_Turns()` [private]
    /// Parses ABBIEGEBEZIEHUNG/ABBIEGER
    // void
    // NIImporter_VISUM::parse_Turns() {
    //     if (myLineParser.know(KEYS.getString(VISUM_TYPES)) && myLineParser.get(KEYS.getString(VISUM_TYPES)) == "") {
    //         // no vehicle allowed; don't add
    //         return;
    //     }
    //     // retrieve the nodes
    //     NBNode* from = getNamedNode("VonKnot", KEYS.getString(VISUM_FROMNODE));
    //     NBNode* via = getNamedNode("UeberKnot", KEYS.getString(VISUM_VIANODENO));
    //     NBNode* to = getNamedNode("NachKnot", KEYS.getString(VISUM_TONODE));
    //     if (from == nullptr || via == nullptr || to == nullptr) {
    //         return;
    //     }
    //     // all nodes are known
    //     std::string type = myLineParser.know("VSysCode")
    //                        ? myLineParser.get("VSysCode")
    //                        : myLineParser.get(KEYS.getString(VISUM_TYPES));
    //     if (myVSysTypes.find(type) != myVSysTypes.end() &&
    //             myVSysTypes.find(type)->second.type == KEYS.getString(VISUM_PRT)) {
    //         // try to set the turning definition
    //         NBEdge* src = from->getConnectionTo(via);
    //         NBEdge* dest = via->getConnectionTo(to);
    //         // check both
    //         if (src == nullptr) {
    //             if (OptionsCont::getOptions().getBool("visum.verbose-warnings")) {
    //                 WRITE_WARNINGF(TL("There is no edge from node '%' to node '%'."), from->getID(), via->getID());
    //             }
    //             return;
    //         }
    //         if (dest == nullptr) {
    //             if (OptionsCont::getOptions().getBool("visum.verbose-warnings")) {
    //                 WRITE_WARNINGF(TL("There is no edge from node '%' to node '%'."), via->getID(), to->getID());
    //             }
    //             return;
    //         }
    //         // both edges found
    //         //  set them into the edge
    //         src->addEdge2EdgeConnection(dest);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_turns(&mut self) {
        if self.myLineParser.know(&keys_string(K::VISUM_TYPES)) && self.field(&keys_string(K::VISUM_TYPES)) == "" {
            // no vehicle allowed; don't add
            return;
        }
        // retrieve the nodes
        let from = self.get_named_node_2("VonKnot", &keys_string(K::VISUM_FROMNODE));
        let via = self.get_named_node_2("UeberKnot", &keys_string(K::VISUM_VIANODENO));
        let to = self.get_named_node_2("NachKnot", &keys_string(K::VISUM_TONODE));
        if from.is_null() || via.is_null() || to.is_null() {
            return;
        }
        // all nodes are known
        let typ = if self.myLineParser.know("VSysCode") { self.field("VSysCode") } else { self.field(&keys_string(K::VISUM_TYPES)) };
        let is_prt = self.myVSysTypes.get(&typ).is_some_and(|t| t.type_ == keys_string(K::VISUM_PRT));
        if is_prt {
            // try to set the turning definition
            let src = from.get_connection_to(via);
            let dest = via.get_connection_to(to);
            // check both
            if src.is_null() {
                if opt_bool("visum.verbose-warnings") {
                    write_warningf!("There is no edge from node '%' to node '%'.", from.get_id().to_string(), via.get_id().to_string());
                }
                return;
            }
            if dest.is_null() {
                if opt_bool("visum.verbose-warnings") {
                    write_warningf!("There is no edge from node '%' to node '%'.", via.get_id().to_string(), to.get_id().to_string());
                }
                return;
            }
            // both edges found
            //  set them into the edge
            src.m().add_edge2_edge_connection(dest, false, SVC_UNSPECIFIED);
        }
    }

    /// C++: `void NIImporter_VISUM::parse_EdgePolys()` [private]
    /// Parses STRECKENPOLY
    // void
    // NIImporter_VISUM::parse_EdgePolys() {
    //     // get the from- & to-node and validate them
    //     NBNode* from = getNamedNode("VonKnot", KEYS.getString(VISUM_FROMNODE));
    //     NBNode* to = getNamedNode("NachKnot", KEYS.getString(VISUM_TONODE));
    //     if (!checkNodes(from, to)) {
    //         return;
    //     }
    //     bool failed = false;
    //     int index;
    //     double x, y;
    //     try {
    //         index = StringUtils::toInt(myLineParser.get(KEYS.getString(VISUM_INDEX)));
    //         x = getNamedFloat(KEYS.getString(VISUM_XCOORD));
    //         y = getNamedFloat(KEYS.getString(VISUM_YCOORD));
    //     } catch (NumberFormatException&) {
    //         WRITE_ERRORF(TL("Error in geometry description from node '%' to node '%'."), from->getID(), to->getID());
    //         return;
    //     }
    //     Position pos(x, y);
    //     if (!NBNetBuilder::transformCoordinate(pos)) {
    //         WRITE_ERRORF(TL("Unable to project coordinates for node '%'."), from->getID());
    //         return;
    //     }
    //     NBEdge* e = from->getConnectionTo(to);
    //     if (e != nullptr) {
    //         e->addGeometryPoint(index, pos);
    //     } else {
    //         failed = true;
    //     }
    //     e = to->getConnectionTo(from);
    //     if (e != nullptr) {
    //         e->addGeometryPoint(-index, pos);
    //         failed = false;
    //     }
    //     // check whether the operation has failed
    //     if (failed) {
    //         if (OptionsCont::getOptions().getBool("visum.verbose-warnings")) {
    //             WRITE_WARNINGF(TL("There is no edge from node '%' to node '%'."), from->getID(), to->getID());
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_edge_polys(&mut self) {
        // get the from- & to-node and validate them
        let from = self.get_named_node_2("VonKnot", &keys_string(K::VISUM_FROMNODE));
        let to = self.get_named_node_2("NachKnot", &keys_string(K::VISUM_TONODE));
        if !self.check_nodes(from, to) {
            return;
        }
        let mut failed = false;
        let parsed = catch_process_error(|| {
            let index = throw(to_int(&self.field(&keys_string(K::VISUM_INDEX))));
            let x = self.get_named_float(&keys_string(K::VISUM_XCOORD));
            let y = self.get_named_float(&keys_string(K::VISUM_YCOORD));
            (index, x, y)
        });
        let (index, x, y) = match parsed {
            Ok(v) => v,
            Err(ProcessError::NumberFormat(_)) => {
                write_errorf!("Error in geometry description from node '%' to node '%'.", from.get_id().to_string(), to.get_id().to_string());
                return;
            }
            Err(e) => std::panic::panic_any(e),
        };
        let mut pos = Position::new(x, y);
        if !NBNetBuilder::transform_coordinate(&mut pos, true, Ptr::null()) {
            write_errorf!("Unable to project coordinates for node '%'.", from.get_id().to_string());
            return;
        }
        let mut e = from.get_connection_to(to);
        if !e.is_null() {
            e.m().add_geometry_point(index, &pos);
        } else {
            failed = true;
        }
        e = to.get_connection_to(from);
        if !e.is_null() {
            e.m().add_geometry_point(-index, &pos);
            failed = false;
        }
        // check whether the operation has failed
        if failed && opt_bool("visum.verbose-warnings") {
            write_warningf!("There is no edge from node '%' to node '%'.", from.get_id().to_string(), to.get_id().to_string());
        }
    }

    /// C++: `void NIImporter_VISUM::parse_Lanes()` [private]
    /// Parses FAHRSTREIFEN
    // void
    // NIImporter_VISUM::parse_Lanes() {
    //     // The base number of lanes for the edge was already defined in STRECKE
    //     // this refines lane specific attribute (width) and optionally introduces splits for additional lanes
    //     // It is permitted for KNOTNR to be 0
    //     //
    //     // get the edge
    //     NBEdge* baseEdge = getNamedEdge(KEYS.getString(VISUM_LINKNO));
    //     if (baseEdge == nullptr) {
    //         return;
    //     }
    //     NBEdge* edge = baseEdge;
    //     // get the node
    //     NBNode* node = getNamedNodeSecure("KNOTNR");
    //     if (node == nullptr) {
    //         node = edge->getToNode();
    //     } else {
    //         edge = getNamedEdgeContinuating(KEYS.getString(VISUM_LINKNO), node);
    //     }
    //     // check
    //     if (edge == nullptr) {
    //         return;
    //     }
    //     // get the lane
    //     std::string laneS = myLineParser.know("FSNR")
    //                         ? NBHelpers::normalIDRepresentation(myLineParser.get("FSNR"))
    //                         : NBHelpers::normalIDRepresentation(myLineParser.get("NR"));
    //     int lane = -1;
    //     try {
    //         lane = StringUtils::toInt(laneS);
    //     } catch (NumberFormatException&) {
    //         WRITE_ERRORF(TL("A lane number for edge '%' is not numeric (%)."), edge->getID(), laneS);
    //         return;
    //     }
    //     lane -= 1;
    //     if (lane < 0) {
    //         WRITE_ERRORF(TL("A lane number for edge '%' is not positive (%)."), edge->getID(), laneS);
    //         return;
    //     }
    //     // get the direction
    //     std::string dirS = NBHelpers::normalIDRepresentation(myLineParser.get("RICHTTYP"));
    //     int prevLaneNo = baseEdge->getNumLanes();
    //     if ((dirS == "1" && !(node->hasIncoming(edge))) || (dirS == "0" && !(node->hasOutgoing(edge)))) {
    //         // get the last part of the turnaround direction
    //         NBEdge* cand = getReversedContinuating(edge, node);
    //         if (cand) {
    //             edge = cand;
    //         }
    //     }
    //     // get the length
    //     std::string lengthS = NBHelpers::normalIDRepresentation(myLineParser.get("LAENGE"));
    //     double length = -1;
    //     try {
    //         length = StringUtils::toDouble(lengthS);
    //     } catch (NumberFormatException&) {
    //         WRITE_ERRORF(TL("A lane length for edge '%' is not numeric (%)."), edge->getID(), lengthS);
    //         return;
    //     }
    //     if (length < 0) {
    //         WRITE_ERRORF(TL("A lane length for edge '%' is not positive (%)."), edge->getID(), lengthS);
    //         return;
    //     }
    //     //
    //     if (dirS == "1") {
    //         lane -= prevLaneNo;
    //     }
    //     //
    //     if (length == 0) {
    //         if ((int) edge->getNumLanes() > lane) {
    //             // ok, we know this already...
    //             return;
    //         }
    //         // increment by one
    //         edge->incLaneNo(1);
    //     } else {
    //         // check whether this edge already has been created
    //         if (isSplitEdge(edge, node)) {
    //             if (edge->getID().substr(edge->getID().find('_')) == "_" + toString(length) + "_" + node->getID()) {
    //                 if ((int) edge->getNumLanes() > lane) {
    //                     // ok, we know this already...
    //                     return;
    //                 }
    //                 // increment by one
    //                 edge->incLaneNo(1);
    //                 return;
    //             }
    //         }
    //         // nope, we have to split the edge...
    //         //  maybe it is not the proper edge to split - VISUM seems not to sort the splits...
    //         bool mustRecheck = true;
    //         double seenLength = 0;
    //         while (mustRecheck) {
    //             if (isSplitEdge(edge, node)) {
    //                 // ok, we have a previously created edge here
    //                 std::string sub = edge->getID();
    //                 sub = sub.substr(sub.rfind('_', sub.rfind('_') - 1));
    //                 sub = sub.substr(1, sub.find('_', 1) - 1);
    //                 double dist = StringUtils::toDouble(sub);
    //                 if (dist < length) {
    //                     seenLength += edge->getLength();
    //                     if (dirS == "1") {
    //                         // incoming -> move back
    //                         edge = edge->getFromNode()->getIncomingEdges()[0];
    //                     } else {
    //                         // outgoing -> move forward
    //                         edge = edge->getToNode()->getOutgoingEdges()[0];
    //                     }
    //                 } else {
    //                     mustRecheck = false;
    //                 }
    //             } else {
    //                 // we have the center edge - do not continue...
    //                 mustRecheck = false;
    //             }
    //         }
    //         // compute position
    //         Position p;
    //         double useLength = length - seenLength;
    //         useLength = edge->getLength() - useLength;
    //         if (useLength < 0 || useLength > edge->getLength()) {
    //             WRITE_WARNINGF(TL("Could not find split position for edge '%'."), edge->getID());
    //             return;
    //         }
    //         std::string edgeID = edge->getID();
    //         p = edge->getGeometry().positionAtOffset(useLength);
    //         if (isSplitEdge(edge, node)) {
    //             edgeID = edgeID.substr(0, edgeID.find('_'));
    //         }
    //         NBNode* rn = new NBNode(edgeID + "_" +  toString((int) length) + "_" + node->getID(), p);
    //         if (!myNetBuilder.getNodeCont().insert(rn)) {
    //             throw ProcessError(TL("Ups - could not insert node!"));
    //         }
    //         std::string nid = edgeID + "_" +  toString((int) length) + "_" + node->getID();
    //         myNetBuilder.getEdgeCont().splitAt(myNetBuilder.getDistrictCont(), edge, useLength, rn,
    //                                            edge->getID(), nid, edge->getNumLanes() + 0, edge->getNumLanes() + 1);
    //         // old edge is deleted and a new edge with the same name created
    //         edge = myNetBuilder.getEdgeCont().retrieve(edgeID);
    //         NBEdge* nedge = myNetBuilder.getEdgeCont().retrieve(nid);
    //         nedge = nedge->getToNode()->getOutgoingEdges()[0];
    //         while (isSplitEdge(edge, node)) {
    //             assert(nedge->getToNode()->getOutgoingEdges().size() > 0);
    //             nedge->incLaneNo(1);
    //             nedge = nedge->getToNode()->getOutgoingEdges()[0];
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_lanes(&mut self) {
        // The base number of lanes for the edge was already defined in STRECKE
        // this refines lane specific attribute (width) and optionally introduces splits for additional lanes
        // It is permitted for KNOTNR to be 0
        //
        // get the edge
        let baseEdge = self.get_named_edge(&keys_string(K::VISUM_LINKNO));
        if baseEdge.is_null() {
            return;
        }
        let mut edge = baseEdge;
        // get the node
        let mut node = self.get_named_node_secure("KNOTNR", Ptr::null());
        if node.is_null() {
            node = edge.get_to_node();
        } else {
            edge = self.get_named_edge_continuating(&keys_string(K::VISUM_LINKNO), node);
        }
        // check
        if edge.is_null() {
            return;
        }
        // get the lane
        let laneS = if self.myLineParser.know("FSNR") { normal_id(&self.field("FSNR")) } else { normal_id(&self.field("NR")) };
        let lane = match to_int(&laneS) {
            Ok(v) => v,
            Err(ProcessError::NumberFormat(_)) => {
                write_errorf!("A lane number for edge '%' is not numeric (%).", edge.get_id().to_string(), laneS.clone());
                return;
            }
            Err(e) => std::panic::panic_any(e),
        };
        let mut lane = lane - 1;
        if lane < 0 {
            write_errorf!("A lane number for edge '%' is not positive (%).", edge.get_id().to_string(), laneS.clone());
            return;
        }
        // get the direction
        let dirS = normal_id(&self.field("RICHTTYP"));
        let prevLaneNo = baseEdge.get_num_lanes();
        if (dirS == "1" && !node.has_incoming(edge)) || (dirS == "0" && !node.has_outgoing(edge)) {
            // get the last part of the turnaround direction
            let cand = self.get_reversed_continuating(edge, node);
            if !cand.is_null() {
                edge = cand;
            }
        }
        // get the length
        let lengthS = normal_id(&self.field("LAENGE"));
        let length: f64 = match to_double(&lengthS) {
            Ok(v) => v,
            Err(ProcessError::NumberFormat(_)) => {
                write_errorf!("A lane length for edge '%' is not numeric (%).", edge.get_id().to_string(), lengthS.clone());
                return;
            }
            Err(e) => std::panic::panic_any(e),
        };
        if length < 0.0 {
            write_errorf!("A lane length for edge '%' is not positive (%).", edge.get_id().to_string(), lengthS.clone());
            return;
        }
        //
        if dirS == "1" {
            lane -= prevLaneNo;
        }
        //
        if length == 0.0 {
            if edge.get_num_lanes() > lane {
                // ok, we know this already...
                return;
            }
            // increment by one
            edge.m().inc_lane_no(1);
        } else {
            // check whether this edge already has been created
            if Self::is_split_edge(edge, node) {
                let eid = edge.get_id().to_string();
                let suffix = &eid[eid.find('_').unwrap_or(0)..];
                if suffix == format!("_{}_{}", f64_to_string(length), node.get_id()) {
                    if edge.get_num_lanes() > lane {
                        // ok, we know this already...
                        return;
                    }
                    // increment by one
                    edge.m().inc_lane_no(1);
                    return;
                }
            }
            // nope, we have to split the edge...
            //  maybe it is not the proper edge to split - VISUM seems not to sort the splits...
            let mut mustRecheck = true;
            let mut seenLength = 0.0;
            while mustRecheck {
                if Self::is_split_edge(edge, node) {
                    // ok, we have a previously created edge here
                    let mut sub = edge.get_id().to_string();
                    let p1 = sub.rfind('_').unwrap_or(0);
                    let p2 = sub[..p1].rfind('_').unwrap_or(0);
                    sub = sub[p2..].to_string();
                    let q = sub[1..].find('_').map(|x| x + 1).unwrap_or(sub.len());
                    let sub = sub[1..q].to_string();
                    let dist = throw(to_double(&sub));
                    if dist < length {
                        seenLength += edge.get_length();
                        if dirS == "1" {
                            // incoming -> move back
                            edge = edge.get_from_node().m().get_incoming_edges()[0];
                        } else {
                            // outgoing -> move forward
                            edge = edge.get_to_node().m().get_outgoing_edges()[0];
                        }
                    } else {
                        mustRecheck = false;
                    }
                } else {
                    // we have the center edge - do not continue...
                    mustRecheck = false;
                }
            }
            // compute position
            let mut useLength = length - seenLength;
            useLength = edge.get_length() - useLength;
            if useLength < 0.0 || useLength > edge.get_length() {
                write_warningf!("Could not find split position for edge '%'.", edge.get_id().to_string());
                return;
            }
            let mut edgeID = edge.get_id().to_string();
            let p = edge.get_geometry().position_at_offset(useLength, 0.0);
            if Self::is_split_edge(edge, node) {
                edgeID = before_underscore(&edgeID).to_string();
            }
            let nid = format!("{}_{}_{}", edgeID, (length as i32).to_string(), node.get_id());
            let rn = Ptr::from_raw(Box::into_raw(Box::new(NBNode::new_2(&nid, &p, Ptr::null()))));
            if !self.myNetBuilder.m().get_node_cont().insert_2(rn) {
                std::panic::panic_any(ProcessError::new("Ups - could not insert node!"));
            }
            let dc = Ptr::from_mut(self.myNetBuilder.m().get_district_cont());
            let ec = Ptr::from_mut(self.myNetBuilder.m().get_edge_cont());
            let numLanes = edge.get_num_lanes();
            ec.m().split_at_3(dc.m(), edge, useLength, rn, &edge.get_id().to_string(), &nid, numLanes, numLanes + 1, -1.0, 1.0, 0);
            // old edge is deleted and a new edge with the same name created
            edge = self.myNetBuilder.m().get_edge_cont().retrieve(&edgeID, false);
            let mut nedge = self.myNetBuilder.m().get_edge_cont().retrieve(&nid, false);
            nedge = nedge.get_to_node().m().get_outgoing_edges()[0];
            while Self::is_split_edge(edge, node) {
                debug_assert!(!nedge.get_to_node().get_outgoing_edges().is_empty());
                nedge.m().inc_lane_no(1);
                nedge = nedge.get_to_node().m().get_outgoing_edges()[0];
            }
        }
    }

    /// C++: `void NIImporter_VISUM::parse_TrafficLights()` [private]
    /// Parses LSA/SIGNALANLAGE
    // void
    // NIImporter_VISUM::parse_TrafficLights() {
    //     myCurrentID = NBHelpers::normalIDRepresentation(myLineParser.get(KEYS.getString(VISUM_NO)));
    //     const SUMOTime cycleTime = TIME2STEPS(getWeightedFloat2("Umlaufzeit", "UMLZEIT", "s"));
    //     const SUMOTime intermediateTime = TIME2STEPS(getWeightedFloat2("StdZwischenzeit", "STDZWZEIT", "s"));
    //     bool phaseBased = myLineParser.know("PhasenBasiert")
    //                       ? StringUtils::toBool(myLineParser.get("PhasenBasiert"))
    //                       : false;
    //     const SUMOTime offset = myLineParser.know("ZEITVERSATZ") ? TIME2STEPS(getWeightedFloat("ZEITVERSATZ", "s")) : 0;
    //     // add to the list
    //     myTLS[myCurrentID] = new NIVisumTL(myCurrentID, cycleTime, offset, intermediateTime, phaseBased);
    // }
    #[allow(unused_variables)]
    pub fn parse_traffic_lights(&mut self) {
        self.myCurrentID = normal_id(&self.field(&keys_string(K::VISUM_NO)));
        let cycleTime = time2steps(self.get_weighted_float2("Umlaufzeit", "UMLZEIT", "s"));
        let intermediateTime = time2steps(self.get_weighted_float2("StdZwischenzeit", "STDZWZEIT", "s"));
        let phaseBased = if self.myLineParser.know("PhasenBasiert") { throw(to_bool(&self.field("PhasenBasiert"))) } else { false };
        let offset = if self.myLineParser.know("ZEITVERSATZ") { time2steps(self.get_weighted_float("ZEITVERSATZ", "s")) } else { 0 };
        // add to the list
        let tl = Ptr::from_raw(Box::into_raw(Box::new(NIVisumTL::new(&self.myCurrentID, cycleTime, offset, intermediateTime, phaseBased))));
        self.myTLS.insert(self.myCurrentID.clone(), tl);
    }

    /// C++: `void NIImporter_VISUM::parse_NodesToTrafficLights()` [private]
    /// Parses KNOTENZULSA/SIGNALANLAGEZUKNOTEN
    // void
    // NIImporter_VISUM::parse_NodesToTrafficLights() {
    //     std::string node = myLineParser.get("KnotNr").c_str();
    //     if (node == "0") {
    //         // this is a dummy value which cannot be assigned to
    //         return;
    //     }
    //     std::string trafficLight = myLineParser.get("LsaNr").c_str();
    //     // add to the list
    //     NBNode* n = myNetBuilder.getNodeCont().retrieve(node);
    //     auto tlIt = myTLS.find(trafficLight);
    //     if (n != nullptr && tlIt != myTLS.end()) {
    //         tlIt->second->addNode(n);
    //     } else {
    //         WRITE_ERROR("Could not assign" + std::string(n == nullptr ? " missing" : "") + " node '" + node
    //                     + "' to" + std::string(tlIt == myTLS.end() ? " missing" : "") + " traffic light '" + trafficLight + "'");
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_nodes_to_traffic_lights(&mut self) {
        let node = self.field("KnotNr");
        if node == "0" {
            // this is a dummy value which cannot be assigned to
            return;
        }
        let trafficLight = self.field("LsaNr");
        // add to the list
        let n = self.myNetBuilder.m().get_node_cont().retrieve(&node);
        let tlIt = self.myTLS.get(&trafficLight).copied();
        match tlIt {
            Some(tl) if !n.is_null() => tl.m().add_node(n),
            _ => write_error(&format!(
                "Could not assign{} node '{}' to{} traffic light '{}'",
                if n.is_null() { " missing" } else { "" },
                node,
                if tlIt.is_none() { " missing" } else { "" },
                trafficLight
            )),
        }
    }

    /// C++: `void NIImporter_VISUM::parse_SignalGroups()` [private]
    /// Parses LSASIGNALGRUPPE/SIGNALGRUPPE
    // void
    // NIImporter_VISUM::parse_SignalGroups() {
    //     myCurrentID = NBHelpers::normalIDRepresentation(myLineParser.get(KEYS.getString(VISUM_NO)));
    //     std::string LSAid = NBHelpers::normalIDRepresentation(myLineParser.get("LsaNr"));
    //     const SUMOTime startTime = TIME2STEPS(getNamedFloat("GzStart", "GRUENANF"));
    //     const SUMOTime endTime = TIME2STEPS(getNamedFloat("GzEnd", "GRUENENDE"));
    //     const SUMOTime yellowTime = myLineParser.know("GELB") ? TIME2STEPS(getNamedFloat("GELB")) : -1;
    //     // add to the list
    //     if (myTLS.find(LSAid) == myTLS.end()) {
    //         WRITE_ERRORF(TL("Could not find TLS '%' for setting the signal group."), LSAid);
    //         return;
    //     }
    //     myTLS.find(LSAid)->second->addSignalGroup(myCurrentID, startTime, endTime, yellowTime);
    // }
    #[allow(unused_variables)]
    pub fn parse_signal_groups(&mut self) {
        self.myCurrentID = normal_id(&self.field(&keys_string(K::VISUM_NO)));
        let LSAid = normal_id(&self.field("LsaNr"));
        let startTime = time2steps(self.get_named_float_2("GzStart", "GRUENANF"));
        let endTime = time2steps(self.get_named_float_2("GzEnd", "GRUENENDE"));
        let yellowTime = if self.myLineParser.know("GELB") { time2steps(self.get_named_float("GELB")) } else { -1 };
        // add to the list
        let Some(tl) = self.myTLS.get(&LSAid).copied() else {
            write_errorf!("Could not find TLS '%' for setting the signal group.", LSAid.clone());
            return;
        };
        tl.m().add_signal_group(&self.myCurrentID, startTime, endTime, yellowTime);
    }

    /// C++: `void NIImporter_VISUM::parse_TurnsToSignalGroups()` [private]
    /// Parses ABBZULSASIGNALGRUPPE/SIGNALGRUPPEZUABBIEGER
    // void
    // NIImporter_VISUM::parse_TurnsToSignalGroups() {
    //     // get the id
    //     std::string SGid = getNamedString("SGNR", "SIGNALGRUPPENNR");
    //     if (!myLineParser.know("LsaNr")) {
    //         /// XXX could be retrieved from context
    //         WRITE_WARNING(TL("Ignoring SIGNALGRUPPEZUFSABBIEGER because LsaNr is not known"));
    //         return;
    //     }
    //     std::string LSAid = getNamedString("LsaNr");
    //     // nodes
    //     NBNode* from = myLineParser.know("VonKnot") ? getNamedNode("VonKnot") : nullptr;
    //     NBNode* via = myLineParser.know("KNOTNR")
    //                   ? getNamedNode("KNOTNR")
    //                   : getNamedNode("UeberKnot", "UeberKnotNr");
    //     NBNode* to = myLineParser.know("NachKnot") ? getNamedNode("NachKnot") : nullptr;
    //     // edges
    //     NBEdge* edg1 = nullptr;
    //     NBEdge* edg2 = nullptr;
    //     if (from == nullptr && to == nullptr) {
    //         edg1 = getNamedEdgeContinuating("VONSTRNR", via);
    //         edg2 = getNamedEdgeContinuating("NACHSTRNR", via);
    //     } else {
    //         edg1 = getEdge(from, via);
    //         edg2 = getEdge(via, to);
    //     }
    //     // add to the list
    //     NIVisumTL::SignalGroup& SG = myTLS.find(LSAid)->second->getSignalGroup(SGid);
    //     if (edg1 != nullptr && edg2 != nullptr) {
    //         if (!via->hasIncoming(edg1)) {
    //             std::string sid;
    //             if (edg1->getID()[0] == '-') {
    //                 sid = edg1->getID().substr(1);
    //             } else {
    //                 sid = "-" + edg1->getID();
    //             }
    //             if (sid.find('_') != std::string::npos) {
    //                 sid = sid.substr(0, sid.find('_'));
    //             }
    //             edg1 = getNamedEdgeContinuating(myNetBuilder.getEdgeCont().retrieve(sid),  via);
    //         }
    //         if (!via->hasOutgoing(edg2)) {
    //             std::string sid;
    //             if (edg2->getID()[0] == '-') {
    //                 sid = edg2->getID().substr(1);
    //             } else {
    //                 sid = "-" + edg2->getID();
    //             }
    //             if (sid.find('_') != std::string::npos) {
    //                 sid = sid.substr(0, sid.find('_'));
    //             }
    //             edg2 = getNamedEdgeContinuating(myNetBuilder.getEdgeCont().retrieve(sid),  via);
    //         }
    //         SG.connections().push_back(NBConnection(edg1, edg2));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_turns_to_signal_groups(&mut self) {
        // get the id
        let SGid = self.get_named_string_2("SGNR", "SIGNALGRUPPENNR");
        if !self.myLineParser.know("LsaNr") {
            // XXX could be retrieved from context
            write_warning("Ignoring SIGNALGRUPPEZUFSABBIEGER because LsaNr is not known");
            return;
        }
        let LSAid = self.get_named_string("LsaNr");
        // nodes
        let from = if self.myLineParser.know("VonKnot") { self.get_named_node("VonKnot") } else { Ptr::null() };
        let via = if self.myLineParser.know("KNOTNR") { self.get_named_node("KNOTNR") } else { self.get_named_node_2("UeberKnot", "UeberKnotNr") };
        let to = if self.myLineParser.know("NachKnot") { self.get_named_node("NachKnot") } else { Ptr::null() };
        // edges
        let mut edg1: Ptr<NBEdge> = Ptr::null();
        let mut edg2: Ptr<NBEdge> = Ptr::null();
        if from.is_null() && to.is_null() {
            edg1 = self.get_named_edge_continuating("VONSTRNR", via);
            edg2 = self.get_named_edge_continuating("NACHSTRNR", via);
        } else {
            edg1 = self.get_edge(from, via);
            edg2 = self.get_edge(via, to);
        }
        // add to the list
        let tl = self.myTLS.get(&LSAid).copied().expect("unknown traffic light");
        let SG = tl.m().get_signal_group(&SGid);
        if !edg1.is_null() && !edg2.is_null() {
            if !via.has_incoming(edg1) {
                let mut sid = if edg1.get_id().starts_with('-') { edg1.get_id()[1..].to_string() } else { format!("-{}", edg1.get_id()) };
                if let Some(p) = sid.find('_') {
                    sid.truncate(p);
                }
                let e = self.myNetBuilder.m().get_edge_cont().retrieve(&sid, false);
                edg1 = self.get_named_edge_continuating_3(e, via);
            }
            if !via.has_outgoing(edg2) {
                let mut sid = if edg2.get_id().starts_with('-') { edg2.get_id()[1..].to_string() } else { format!("-{}", edg2.get_id()) };
                if let Some(p) = sid.find('_') {
                    sid.truncate(p);
                }
                let e = self.myNetBuilder.m().get_edge_cont().retrieve(&sid, false);
                edg2 = self.get_named_edge_continuating_3(e, via);
            }
            SG.connections().push(NBConnection::new(edg1, edg2));
        }
    }

    /// C++: `void NIImporter_VISUM::parse_AreaSubPartElement()` [private]
    /// Parses ABBZULSASIGNALGRUPPE/SIGNALGRUPPEZUABBIEGER
    // void
    // NIImporter_VISUM::parse_AreaSubPartElement() {
    //     long long int id = StringUtils::toLong(myLineParser.get(KEYS.getString(VISUM_FACEID)));
    //     long long int edgeid = StringUtils::toLong(myLineParser.get(KEYS.getString(VISUM_EDGEID)));
    //     if (myEdges.find(edgeid) == myEdges.end()) {
    //         WRITE_ERROR(TL("Unknown edge in TEILFLAECHENELEMENT"));
    //         return;
    //     }
    //     std::string dir = myLineParser.get(KEYS.getString(VISUM_DIRECTION));
    // // get index (unused)
    // //     std::string indexS = NBHelpers::normalIDRepresentation(myLineParser.get("INDEX"));
    // //     int index = -1;
    // //     try {
    // //         index = StringUtils::toInt(indexS) - 1;
    // //     } catch (NumberFormatException&) {
    // //         WRITE_ERRORF(TL("An index for a TEILFLAECHENELEMENT is not numeric (id='%')."), toString(id));
    // //         return;
    // //     }
    //     PositionVector shape;
    //     shape.push_back(myPoints[myEdges[edgeid].first]);
    //     shape.push_back(myPoints[myEdges[edgeid].second]);
    //     if (dir.length() > 0 && dir[0] == '1') {
    //         shape = shape.reverse();
    //     }
    //     if (mySubPartsAreas.find(id) == mySubPartsAreas.end()) {
    //         WRITE_ERRORF(TL("Unknown are for area part '%'."), myCurrentID);
    //         return;
    //     }
    //
    //     const std::vector<long long int>& areas = mySubPartsAreas.find(id)->second;
    //     for (std::vector<long long int>::const_iterator i = areas.begin(); i != areas.end(); ++i) {
    //         NBDistrict* d = myShapeDistrictMap[*i];
    //         if (d == nullptr) {
    //             continue;
    //         }
    //         if (myDistrictShapes.find(d) == myDistrictShapes.end()) {
    //             myDistrictShapes[d] = PositionVector();
    //         }
    //         if (dir.length() > 0 && dir[0] == '1') {
    //             myDistrictShapes[d].push_back(myPoints[myEdges[edgeid].second]);
    //             myDistrictShapes[d].push_back(myPoints[myEdges[edgeid].first]);
    //         } else {
    //             myDistrictShapes[d].push_back(myPoints[myEdges[edgeid].first]);
    //             myDistrictShapes[d].push_back(myPoints[myEdges[edgeid].second]);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_area_sub_part_element(&mut self) {
        let id = throw(to_long(&self.field(&keys_string(K::VISUM_FACEID))));
        let edgeid = throw(to_long(&self.field(&keys_string(K::VISUM_EDGEID))));
        if !self.myEdges.contains_key(&edgeid) {
            write_error("Unknown edge in TEILFLAECHENELEMENT");
            return;
        }
        let dir = self.field(&keys_string(K::VISUM_DIRECTION));
        // get index (unused)
        let (efrom, eto) = self.myEdges[&edgeid];
        let origin = Position::new(0.0, 0.0);
        let mut shape = PositionVector::default();
        shape.0.push(*self.myPoints.entry(efrom).or_insert(origin));
        shape.0.push(*self.myPoints.entry(eto).or_insert(origin));
        if dir.starts_with('1') {
            shape = shape.reverse();
        }
        if !self.mySubPartsAreas.contains_key(&id) {
            write_errorf!("Unknown are for area part '%'.", self.myCurrentID.clone());
            return;
        }

        let areas = self.mySubPartsAreas[&id].clone();
        for a in areas {
            let d = *self.myShapeDistrictMap.entry(a).or_insert(Ptr::null());
            if d.is_null() {
                continue;
            }
            let pfrom = *self.myPoints.entry(efrom).or_insert(origin);
            let pto = *self.myPoints.entry(eto).or_insert(origin);
            let ds = self.myDistrictShapes.entry(d).or_default();
            if dir.starts_with('1') {
                ds.0.push(pto);
                ds.0.push(pfrom);
            } else {
                ds.0.push(pfrom);
                ds.0.push(pto);
            }
        }
    }

    /// C++: `void NIImporter_VISUM::parse_Phases()` [private]
    /// Parses LSAPHASE/PHASE
    // void
    // NIImporter_VISUM::parse_Phases() {
    //     // get the id
    //     const std::string phaseid = NBHelpers::normalIDRepresentation(myLineParser.get(KEYS.getString(VISUM_NO)));
    //     const std::string LSAid = NBHelpers::normalIDRepresentation(myLineParser.get("LsaNr"));
    //     const SUMOTime startTime = TIME2STEPS(getNamedFloat("GzStart", "GRUENANF"));
    //     const SUMOTime endTime = TIME2STEPS(getNamedFloat("GzEnd", "GRUENENDE"));
    //     const SUMOTime yellowTime = myLineParser.know("GELB") ? TIME2STEPS(getNamedFloat("GELB")) : -1;
    //     myTLS.find(LSAid)->second->addPhase(phaseid, startTime, endTime, yellowTime);
    // }
    #[allow(unused_variables)]
    pub fn parse_phases(&mut self) {
        // get the id
        let phaseid = normal_id(&self.field(&keys_string(K::VISUM_NO)));
        let LSAid = normal_id(&self.field("LsaNr"));
        let startTime = time2steps(self.get_named_float_2("GzStart", "GRUENANF"));
        let endTime = time2steps(self.get_named_float_2("GzEnd", "GRUENENDE"));
        let yellowTime = if self.myLineParser.know("GELB") { time2steps(self.get_named_float("GELB")) } else { -1 };
        let tl = self.myTLS.get(&LSAid).copied().expect("unknown traffic light");
        tl.m().add_phase(&phaseid, startTime, endTime, yellowTime);
    }

    /// C++: `void NIImporter_VISUM::parse_SignalGroupsToPhases()` [private]
    /// Parses LSASIGNALGRUPPEZULSAPHASE
    // void NIImporter_VISUM::parse_SignalGroupsToPhases() {
    //     // get the id
    //     std::string Phaseid = NBHelpers::normalIDRepresentation(myLineParser.get("PsNr"));
    //     std::string LSAid = NBHelpers::normalIDRepresentation(myLineParser.get("LsaNr"));
    //     std::string SGid = NBHelpers::normalIDRepresentation(myLineParser.get("SGNR"));
    //     // insert
    //     NIVisumTL* LSA = myTLS.find(LSAid)->second;
    //     NIVisumTL::SignalGroup& SG = LSA->getSignalGroup(SGid);
    //     NIVisumTL::Phase* PH = LSA->getPhases().find(Phaseid)->second;
    //     SG.phases()[Phaseid] = PH;
    // }
    #[allow(unused_variables)]
    pub fn parse_signal_groups_to_phases(&mut self) {
        // get the id
        let Phaseid = normal_id(&self.field("PsNr"));
        let LSAid = normal_id(&self.field("LsaNr"));
        let SGid = normal_id(&self.field("SGNR"));
        // insert
        let LSA = self.myTLS.get(&LSAid).copied().expect("unknown traffic light");
        let SG = LSA.m().get_signal_group(&SGid);
        let PH = *LSA.m().get_phases().get(&Phaseid).expect("unknown phase");
        SG.phases().insert(Phaseid, PH);
    }

    /// C++: `void NIImporter_VISUM::parse_LanesConnections()` [private]
    /// Parses FAHRSTREIFENABBIEGER
    // void NIImporter_VISUM::parse_LanesConnections() {
    //     NBNode* node = nullptr;
    //     NBEdge* fromEdge = nullptr;
    //     NBEdge* toEdge = nullptr;
    //     // get the node and edges depending on network format
    //     const std::string nodeID = getNamedString("KNOTNR", "KNOT");
    //     if (nodeID == "0") {
    //         fromEdge = getNamedEdge("VONSTRNR", "VONSTR");
    //         toEdge = getNamedEdge("NACHSTRNR", "NACHSTR");
    //         if (fromEdge == nullptr) {
    //             return;
    //         }
    //         node = fromEdge->getToNode();
    //         WRITE_WARNING(TL("Ignoring lane-to-lane connection (not yet implemented for this format version)"));
    //         return;
    //     } else {
    //         node = getNamedNode("KNOTNR", "KNOT");
    //         if (node  == nullptr) {
    //             return;
    //         }
    //         fromEdge = getNamedEdgeContinuating("VONSTRNR", "VONSTR", node);
    //         toEdge = getNamedEdgeContinuating("NACHSTRNR", "NACHSTR", node);
    //     }
    //     if (fromEdge == nullptr || toEdge == nullptr) {
    //         return;
    //     }
    //
    //     int fromLaneOffset = 0;
    //     if (!node->hasIncoming(fromEdge)) {
    //         fromLaneOffset = fromEdge->getNumLanes();
    //         fromEdge = getReversedContinuating(fromEdge, node);
    //     } else {
    //         fromEdge = getReversedContinuating(fromEdge, node);
    //         NBEdge* tmp = myNetBuilder.getEdgeCont().retrieve(fromEdge->getID().substr(0, fromEdge->getID().find('_')));
    //         fromLaneOffset = tmp->getNumLanes();
    //     }
    //
    //     int toLaneOffset = 0;
    //     if (!node->hasOutgoing(toEdge)) {
    //         toLaneOffset = toEdge->getNumLanes();
    //         toEdge = getReversedContinuating(toEdge, node);
    //     } else {
    //         NBEdge* tmp = myNetBuilder.getEdgeCont().retrieve(toEdge->getID().substr(0, toEdge->getID().find('_')));
    //         toLaneOffset = tmp->getNumLanes();
    //     }
    //     // get the from-lane
    //     std::string fromLaneS = NBHelpers::normalIDRepresentation(myLineParser.get("VONFSNR"));
    //     int fromLane = -1;
    //     try {
    //         fromLane = StringUtils::toInt(fromLaneS);
    //     } catch (NumberFormatException&) {
    //         WRITE_ERRORF(TL("A from-lane number for edge '%' is not numeric (%)."), fromEdge->getID(), fromLaneS);
    //         return;
    //     }
    //     fromLane -= 1;
    //     if (fromLane < 0) {
    //         WRITE_ERRORF(TL("A from-lane number for edge '%' is not positive (%)."), fromEdge->getID(), fromLaneS);
    //         return;
    //     }
    //     // get the from-lane
    //     std::string toLaneS = NBHelpers::normalIDRepresentation(myLineParser.get("NACHFSNR"));
    //     int toLane = -1;
    //     try {
    //         toLane = StringUtils::toInt(toLaneS);
    //     } catch (NumberFormatException&) {
    //         WRITE_ERRORF(TL("A to-lane number for edge '%' is not numeric (%)."), toEdge->getID(), toLaneS);
    //         return;
    //     }
    //     toLane -= 1;
    //     if (toLane < 0) {
    //         WRITE_ERRORF(TL("A to-lane number for edge '%' is not positive (%)."), toEdge->getID(), toLaneS);
    //         return;
    //     }
    //     // !!! the next is probably a hack
    //     if (fromLane - fromLaneOffset < 0) {
    //         //fromLaneOffset = 0;
    //     } else {
    //         fromLane = (int)fromEdge->getNumLanes() - (fromLane - fromLaneOffset) - 1;
    //     }
    //     if (toLane - toLaneOffset < 0) {
    //         //toLaneOffset = 0;
    //     } else {
    //         toLane = (int)toEdge->getNumLanes() - (toLane - toLaneOffset) - 1;
    //     }
    //     //
    //     if ((int) fromEdge->getNumLanes() <= fromLane) {
    //         WRITE_ERRORF(TL("A from-lane number for edge '%' is larger than the edge's lane number (%)."), fromEdge->getID(), fromLaneS);
    //         return;
    //     }
    //     if ((int) toEdge->getNumLanes() <= toLane) {
    //         WRITE_ERRORF(TL("A to-lane number for edge '%' is larger than the edge's lane number (%)."), toEdge->getID(), toLaneS);
    //         return;
    //     }
    //     //
    //     fromEdge->addLane2LaneConnection(fromLane, toEdge, toLane, NBEdge::Lane2LaneInfoType::VALIDATED);
    // }
    #[allow(unused_variables)]
    pub fn parse_lanes_connections(&mut self) {
        // get the node and edges depending on network format
        let nodeID = self.get_named_string_2("KNOTNR", "KNOT");
        let mut node: Ptr<NBNode>;
        let mut fromEdge: Ptr<NBEdge>;
        let mut toEdge: Ptr<NBEdge>;
        if nodeID == "0" {
            fromEdge = self.get_named_edge_2("VONSTRNR", "VONSTR");
            toEdge = self.get_named_edge_2("NACHSTRNR", "NACHSTR");
            if fromEdge.is_null() {
                return;
            }
            node = fromEdge.get_to_node();
            write_warning("Ignoring lane-to-lane connection (not yet implemented for this format version)");
            return;
        } else {
            node = self.get_named_node_2("KNOTNR", "KNOT");
            if node.is_null() {
                return;
            }
            fromEdge = self.get_named_edge_continuating_2("VONSTRNR", "VONSTR", node);
            toEdge = self.get_named_edge_continuating_2("NACHSTRNR", "NACHSTR", node);
        }
        if fromEdge.is_null() || toEdge.is_null() {
            return;
        }

        let mut fromLaneOffset: i32 = 0;
        if !node.has_incoming(fromEdge) {
            fromLaneOffset = fromEdge.get_num_lanes();
            fromEdge = self.get_reversed_continuating(fromEdge, node);
        } else {
            fromEdge = self.get_reversed_continuating(fromEdge, node);
            let tmp = self.myNetBuilder.m().get_edge_cont().retrieve(before_underscore(&fromEdge.get_id().to_string()), false);
            fromLaneOffset = tmp.get_num_lanes();
        }

        let mut toLaneOffset: i32 = 0;
        if !node.has_outgoing(toEdge) {
            toLaneOffset = toEdge.get_num_lanes();
            toEdge = self.get_reversed_continuating(toEdge, node);
        } else {
            let tmp = self.myNetBuilder.m().get_edge_cont().retrieve(before_underscore(&toEdge.get_id().to_string()), false);
            toLaneOffset = tmp.get_num_lanes();
        }
        // get the from-lane
        let fromLaneS = normal_id(&self.field("VONFSNR"));
        let mut fromLane = match to_int(&fromLaneS) {
            Ok(v) => v,
            Err(ProcessError::NumberFormat(_)) => {
                write_errorf!("A from-lane number for edge '%' is not numeric (%).", fromEdge.get_id().to_string(), fromLaneS.clone());
                return;
            }
            Err(e) => std::panic::panic_any(e),
        };
        fromLane -= 1;
        if fromLane < 0 {
            write_errorf!("A from-lane number for edge '%' is not positive (%).", fromEdge.get_id().to_string(), fromLaneS.clone());
            return;
        }
        // get the to-lane
        let toLaneS = normal_id(&self.field("NACHFSNR"));
        let mut toLane = match to_int(&toLaneS) {
            Ok(v) => v,
            Err(ProcessError::NumberFormat(_)) => {
                write_errorf!("A to-lane number for edge '%' is not numeric (%).", toEdge.get_id().to_string(), toLaneS.clone());
                return;
            }
            Err(e) => std::panic::panic_any(e),
        };
        toLane -= 1;
        if toLane < 0 {
            write_errorf!("A to-lane number for edge '%' is not positive (%).", toEdge.get_id().to_string(), toLaneS.clone());
            return;
        }
        // !!! the next is probably a hack
        if fromLane - fromLaneOffset >= 0 {
            // C++ keeps fromLaneOffset unchanged in the other branch
            fromLane = fromEdge.get_num_lanes() - (fromLane - fromLaneOffset) - 1;
        }
        if toLane - toLaneOffset >= 0 {
            toLane = toEdge.get_num_lanes() - (toLane - toLaneOffset) - 1;
        }
        //
        if fromEdge.get_num_lanes() <= fromLane {
            write_errorf!("A from-lane number for edge '%' is larger than the edge's lane number (%).", fromEdge.get_id().to_string(), fromLaneS.clone());
            return;
        }
        if toEdge.get_num_lanes() <= toLane {
            write_errorf!("A to-lane number for edge '%' is larger than the edge's lane number (%).", toEdge.get_id().to_string(), toLaneS.clone());
            return;
        }
        //
        // shortcut: C++ passes NBEdge::myDefaultConnectionLength as the connection length; the static is private to nb_edge.rs so UNSPECIFIED_LOADED_LENGTH (-1) is used, upgrade when my_default_connection_length is pub
        fromEdge.m().add_lane2_lane_connection(fromLane, toEdge, toLane, NBEdge_Lane2LaneInfoType::VALIDATED, false, false, KeepClear::KEEPCLEAR_UNSPECIFIED, UNSPECIFIED_CONTPOS, UNSPECIFIED_VISIBILITY_DISTANCE, UNSPECIFIED_SPEED, UNSPECIFIED_FRICTION, UNSPECIFIED_LOADED_LENGTH, &PositionVector::EMPTY, UNSPECIFIED_CONNECTION_UNCONTROLLED, SVC_UNSPECIFIED, false, "", SVC_UNSPECIFIED, SVC_UNSPECIFIED, false);
    }

    /// C++: `void NIImporter_VISUM::parse_stopPoints()` [private]
    /// Parses HALTEPUNKT (public transport stop locations)
    // void NIImporter_VISUM::parse_stopPoints() {
    //     std::string id = NBHelpers::normalIDRepresentation(myLineParser.get(KEYS.getString(VISUM_NO)));
    //     std::string name = StringUtils::latin1_to_utf8(myLineParser.get(KEYS.getString(VISUM_NAME)));
    //     SVCPermissions permissions = getPermissions(KEYS.getString(VISUM_TYPES), id, myDefaultPermissions);
    //     NBNode* from = getNamedNodeSecure(KEYS.getString(VISUM_FROMNODE));
    //     NBNode* to = getNamedNodeSecure(KEYS.getString(VISUM_FROMNODENO));
    //     const std::string edgeID = myLineParser.get(KEYS.getString(VISUM_LINKNO));
    //     if (edgeID == "") {
    //         WRITE_WARNINGF(TL("Ignoring stopping place '%' without edge id"), id);
    //     } else if (from == nullptr && to == nullptr) {
    //         WRITE_WARNINGF(TL("Ignoring stopping place '%' without node information"), id);
    //     } else {
    //         NBEdge* edge = getNamedEdge(KEYS.getString(VISUM_LINKNO));
    //         if (edge == nullptr) {
    //             WRITE_WARNINGF(TL("Ignoring stopping place '%' with invalid edge reference '%'"), id, edgeID);
    //             return;
    //         } else if (from != nullptr) {
    //             if (edge->getToNode() == from) {
    //                 NBEdge* edge2 = myNetBuilder.getEdgeCont().retrieve("-" + edge->getID());
    //                 if (edge2 == nullptr) {
    //                     WRITE_WARNINGF(TL("Could not find edge with from-node '%' and base id '%' for stopping place '%'"), from->getID(), edge->getID(), id);
    //                 } else {
    //                     edge = edge2;
    //                 }
    //             } else if (edge->getFromNode() != from) {
    //                 WRITE_WARNINGF(TL("Unexpected from-node '%' for edge '%' of stopping place '%'"), from->getID(), edge->getID(), id);
    //             }
    //         } else {
    //             if (edge->getFromNode() == to) {
    //                 NBEdge* edge2 = myNetBuilder.getEdgeCont().retrieve("-" + edge->getID());
    //                 if (edge2 == nullptr) {
    //                     WRITE_WARNINGF(TL("Could not find edge with to-node '%' and base id '%' for stopping place '%'"), to->getID(), edge->getID(), id);
    //                 } else {
    //                     edge = edge2;
    //                 }
    //             } else if (edge->getToNode() != to) {
    //                 WRITE_WARNINGF(TL("Unexpected to-node '%' for edge '%' of stopping place '%'"), to->getID(), edge->getID(), id);
    //             }
    //         }
    //         double relPos = StringUtils::toDouble(myLineParser.get(KEYS.getString(VISUM_RELPOS)));
    //         /// @note could also retrieve Xkoord, ykoord from $HALTESTELLE
    //         Position pos = edge->getGeometry().positionAtOffset(edge->getLength() * relPos);
    //
    //         const double length = OptionsCont::getOptions().getFloat("osm.stop-output.length");
    //         SumoXMLTag element = isRailway(permissions) ? SUMO_TAG_TRAIN_STOP : SUMO_TAG_BUS_STOP;
    //         std::shared_ptr<NBPTStop> ptStop = std::make_shared<NBPTStop>(element, id, pos, edge->getID(), edge->getID(), length, name, permissions);
    //         myNetBuilder.getPTStopCont().insert(ptStop);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_stop_points(&mut self) {
        let id = normal_id(&self.field(&keys_string(K::VISUM_NO)));
        let name = latin1_to_utf8(self.field(&keys_string(K::VISUM_NAME)).as_bytes());
        let key = keys_string(K::VISUM_TYPES);
        let dp = self.myDefaultPermissions;
        let permissions = self.get_permissions(&key, &id, dp);
        let from = self.get_named_node_secure(&keys_string(K::VISUM_FROMNODE), Ptr::null());
        let to = self.get_named_node_secure(&keys_string(K::VISUM_FROMNODENO), Ptr::null());
        let edgeID = self.field(&keys_string(K::VISUM_LINKNO));
        if edgeID.is_empty() {
            write_warningf!("Ignoring stopping place '%' without edge id", id.clone());
        } else if from.is_null() && to.is_null() {
            write_warningf!("Ignoring stopping place '%' without node information", id.clone());
        } else {
            let mut edge = self.get_named_edge(&keys_string(K::VISUM_LINKNO));
            if edge.is_null() {
                write_warningf!("Ignoring stopping place '%' with invalid edge reference '%'", id.clone(), edgeID.clone());
                return;
            } else if !from.is_null() {
                if edge.get_to_node() == from {
                    let edge2 = self.myNetBuilder.m().get_edge_cont().retrieve(&format!("-{}", edge.get_id()), false);
                    if edge2.is_null() {
                        write_warningf!("Could not find edge with from-node '%' and base id '%' for stopping place '%'", from.get_id().to_string(), edge.get_id().to_string(), id.clone());
                    } else {
                        edge = edge2;
                    }
                } else if edge.get_from_node() != from {
                    write_warningf!("Unexpected from-node '%' for edge '%' of stopping place '%'", from.get_id().to_string(), edge.get_id().to_string(), id.clone());
                }
            } else if edge.get_from_node() == to {
                let edge2 = self.myNetBuilder.m().get_edge_cont().retrieve(&format!("-{}", edge.get_id()), false);
                if edge2.is_null() {
                    write_warningf!("Could not find edge with to-node '%' and base id '%' for stopping place '%'", to.get_id().to_string(), edge.get_id().to_string(), id.clone());
                } else {
                    edge = edge2;
                }
            } else if edge.get_to_node() != to {
                write_warningf!("Unexpected to-node '%' for edge '%' of stopping place '%'", to.get_id().to_string(), edge.get_id().to_string(), id.clone());
            }
            let relPos = throw(to_double(&self.field(&keys_string(K::VISUM_RELPOS))));
            // @note could also retrieve Xkoord, ykoord from $HALTESTELLE
            let pos = edge.get_geometry().position_at_offset(edge.get_length() * relPos, 0.0);

            let length = opt_float("osm.stop-output.length");
            let element = if is_railway(permissions) { SUMO_TAG_TRAIN_STOP } else { SUMO_TAG_BUS_STOP };
            let ptStop = std::rc::Rc::new(NBPTStop::new(element, id.clone(), pos, edge.get_id().to_string(), edge.get_id().to_string(), length, name.clone(), permissions, 0.0, RGBColor::new_valid(false), -1.0));
            self.myNetBuilder.m().get_pt_stop_cont().insert(ptStop, false);
        }
    }

    /// C++: `void NIImporter_VISUM::addParser(const std::string &, ParsingFunction)` [private]
    /// Adds a parser into the sorted list of parsers to use
    // void
    // NIImporter_VISUM::addParser(const std::string& name, ParsingFunction function) {
    //     TypeParser p;
    //     p.name = name;
    //     p.function = function;
    //     p.position = -1;
    //     mySingleDataParsers.push_back(p);
    // }
    #[allow(unused_variables)]
    pub fn add_parser(&mut self, name: &str, function: NIImporter_VISUM_ParsingFunction) {
        self.mySingleDataParsers.push(NIImporter_VISUM_TypeParser {
            name: name.to_string(),
            function,
            position: -1,
            pattern: String::new(),
        });
    }

    /// C++: `bool NIImporter_VISUM::isSplitEdge(NBEdge *, NBNode *)` [static, private]
    /// whether the edge id ends with _nodeID
    // bool
    // NIImporter_VISUM::isSplitEdge(NBEdge* edge, NBNode* node) {
    //     return (edge->getID().length() > node->getID().length() + 1
    //             && (edge->getID().substr(edge->getID().length() - node->getID().length() - 1) == "_" + node->getID()));
    // }
    #[allow(unused_variables)]
    pub fn is_split_edge(edge: Ptr<NBEdge>, node: Ptr<NBNode>) -> bool {
        let eid = edge.get_id();
        let nid = node.get_id();
        eid.len() > nid.len() + 1 && eid.ends_with(format!("_{}", nid).as_str())
    }

    /// C++: `void NIImporter_VISUM::loadLanguage(const std::string &)` [protected]
    // void
    // NIImporter_VISUM::loadLanguage(const std::string& file) {
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
    //             WRITE_WARNINGF(TL("Unknown entry '%' in VISUM language map"), keyDE);
    //         }
    //     }
    //
    // }
    #[allow(unused_variables)]
    pub fn load_language(&mut self, file: &str) {
        let content = match std::fs::read_to_string(file) {
            Ok(c) => c,
            Err(_) => std::panic::panic_any(ProcessError::new(utils::tlf!("Could not load VISUM language map from '%'.", file))),
        };
        // strm >> keyDE; strm >> keyNew; (whitespace separated tokens)
        let mut toks = content.split_whitespace();
        while let Some(keyDE) = toks.next() {
            let keyNew = toks.next().unwrap_or("");
            let known = KEYS.with(|k| k.borrow().has_string(keyDE));
            if known {
                KEYS.with(|k| {
                    let mut k = k.borrow_mut();
                    let key = throw(k.get(keyDE));
                    k.remove(keyDE, key);
                    throw(k.insert(keyNew, key, true));
                });
            } else {
                write_warningf!("Unknown entry '%' in VISUM language map", keyDE.to_string());
            }
        }
    }

}

/// A complete call description for parsing a single db.
/// C++ class `NIImporter_VISUM::TypeParser` (netimport/NIImporter_VISUM.h)
pub struct NIImporter_VISUM_TypeParser {
    /// The name of the db
    pub name: String,
    /// Pointer to the function used for parsing
    pub function: NIImporter_VISUM_ParsingFunction,
    /// Position of the according db within the file
    pub position: i64,
    /// The column names
    pub pattern: String,
}

impl NIImporter_VISUM_TypeParser {
}

/// Definition of a storage for vsystypes
/// C++ class `NIImporter_VISUM::VSysType` (netimport/NIImporter_VISUM.h)
pub struct NIImporter_VISUM_VSysType {
    pub type_: String,
    pub name: String,
}

impl NIImporter_VISUM_VSysType {
    /// C++: `void VSysType::VSysType(const std::string &, const std::string &)`
    // VSysType(const std::string& _type, const std::string& _name) : type(_type), name(_name) {}
    #[allow(unused_variables)]
    pub fn new(_type: &str, _name: &str) -> Self {
        NIImporter_VISUM_VSysType { type_: _type.to_string(), name: _name.to_string() }
    }

}

