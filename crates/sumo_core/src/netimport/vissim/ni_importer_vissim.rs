//! Port of netimport/vissim/NIImporter_Vissim.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

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

// C++ static members of NIImporter_Vissim (port as module-level statics, see AGENTS.md):
//   static SequentialStringBijection::Entry[] NIImporter_Vissim::vissimTags;
//   static SequentialStringBijection::Entry[] NIImporter_Vissim::vissimAttrs;

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
        todo!("port NIImporter_Vissim::loadNetwork")
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
    pub fn new(nb: &mut NBNetBuilder) -> Self {
        todo!("port NIImporter_Vissim::NIImporter_Vissim")
    }

    /// C++: `void NIImporter_Vissim::~NIImporter_Vissim()` [protected]
    /// destructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port NIImporter_Vissim::~NIImporter_Vissim")
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
        todo!("port NIImporter_Vissim::load")
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
        todo!("port NIImporter_Vissim::admitContinue")
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
    pub fn read_contents(&mut self, strm: &mut Unmapped /* std::istream */) -> bool {
        todo!("port NIImporter_Vissim::readContents")
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
        todo!("port NIImporter_Vissim::postLoadBuild")
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
        todo!("port NIImporter_Vissim::insertKnownElements")
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
        todo!("port NIImporter_Vissim::buildParsers")
    }

    /// C++: `void NIImporter_Vissim::NIImporter_Vissim(const NIImporter_Vissim &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &NIImporter_Vissim) -> Self {
        todo!("port NIImporter_Vissim::NIImporter_Vissim")
    }

    /// C++: `NIImporter_Vissim & NIImporter_Vissim::operator=(const NIImporter_Vissim &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &NIImporter_Vissim) -> &mut NIImporter_Vissim {
        todo!("port NIImporter_Vissim::operator=")
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
        todo!("port NIVissimXMLHandler_ConflictArea::NIVissimXMLHandler_ConflictArea")
    }

    /// C++: `void NIVissimXMLHandler_ConflictArea::~NIVissimXMLHandler_ConflictArea() noexcept` [virtual]
    /// Destructor
    // NIImporter_Vissim::NIVissimXMLHandler_ConflictArea::~NIVissimXMLHandler_ConflictArea() { }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port NIVissimXMLHandler_ConflictArea::~NIVissimXMLHandler_ConflictArea")
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
        todo!("port NIVissimXMLHandler_ConflictArea::myStartElement")
    }

    /// C++: `void NIVissimXMLHandler_ConflictArea::NIVissimXMLHandler_ConflictArea(const NIVissimXMLHandler_ConflictArea &)` [private]
    /// invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(c: &NIImporter_Vissim_NIVissimXMLHandler_ConflictArea) -> Self {
        todo!("port NIVissimXMLHandler_ConflictArea::NIVissimXMLHandler_ConflictArea")
    }

    /// C++: `NIVissimXMLHandler_ConflictArea & NIVissimXMLHandler_ConflictArea::operator=(const NIVissimXMLHandler_ConflictArea &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, c: &NIImporter_Vissim_NIVissimXMLHandler_ConflictArea) -> &mut NIImporter_Vissim_NIVissimXMLHandler_ConflictArea {
        todo!("port NIVissimXMLHandler_ConflictArea::operator=")
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
        todo!("port NIVissimXMLHandler_Fahrzeugklassendefinition::NIVissimXMLHandler_Fahrzeugklassendefinition")
    }

    /// C++: `void NIVissimXMLHandler_Fahrzeugklassendefinition::~NIVissimXMLHandler_Fahrzeugklassendefinition() noexcept` [virtual]
    /// Destructor
    // NIImporter_Vissim::NIVissimXMLHandler_Fahrzeugklassendefinition::~NIVissimXMLHandler_Fahrzeugklassendefinition() { }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port NIVissimXMLHandler_Fahrzeugklassendefinition::~NIVissimXMLHandler_Fahrzeugklassendefinition")
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
        todo!("port NIVissimXMLHandler_Fahrzeugklassendefinition::myStartElement")
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
        todo!("port NIVissimXMLHandler_Fahrzeugklassendefinition::myEndElement")
    }

    /// C++: `void NIVissimXMLHandler_Fahrzeugklassendefinition::NIVissimXMLHandler_Fahrzeugklassendefinition(const NIVissimXMLHandler_Fahrzeugklassendefinition &)` [private]
    /// invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(f: &NIImporter_Vissim_NIVissimXMLHandler_Fahrzeugklassendefinition) -> Self {
        todo!("port NIVissimXMLHandler_Fahrzeugklassendefinition::NIVissimXMLHandler_Fahrzeugklassendefinition")
    }

    /// C++: `NIVissimXMLHandler_Fahrzeugklassendefinition & NIVissimXMLHandler_Fahrzeugklassendefinition::operator=(const NIVissimXMLHandler_Fahrzeugklassendefinition &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, f: &NIImporter_Vissim_NIVissimXMLHandler_Fahrzeugklassendefinition) -> &mut NIImporter_Vissim_NIVissimXMLHandler_Fahrzeugklassendefinition {
        todo!("port NIVissimXMLHandler_Fahrzeugklassendefinition::operator=")
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
        todo!("port NIVissimXMLHandler_Geschwindigkeitsverteilungsdefinition::NIVissimXMLHandler_Geschwindigkeitsverteilungsdefinition")
    }

    /// C++: `void NIVissimXMLHandler_Geschwindigkeitsverteilungsdefinition::~NIVissimXMLHandler_Geschwindigkeitsverteilungsdefinition() noexcept` [virtual]
    /// Destructor
    // NIImporter_Vissim::NIVissimXMLHandler_Geschwindigkeitsverteilungsdefinition::~NIVissimXMLHandler_Geschwindigkeitsverteilungsdefinition() { }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port NIVissimXMLHandler_Geschwindigkeitsverteilungsdefinition::~NIVissimXMLHandler_Geschwindigkeitsverteilungsdefinition")
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
        todo!("port NIVissimXMLHandler_Geschwindigkeitsverteilungsdefinition::myStartElement")
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
        todo!("port NIVissimXMLHandler_Geschwindigkeitsverteilungsdefinition::myEndElement")
    }

    /// C++: `void NIVissimXMLHandler_Geschwindigkeitsverteilungsdefinition::NIVissimXMLHandler_Geschwindigkeitsverteilungsdefinition(const NIVissimXMLHandler_Geschwindigkeitsverteilungsdefinition &)` [private]
    /// invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(vW: &NIImporter_Vissim_NIVissimXMLHandler_Geschwindigkeitsverteilungsdefinition) -> Self {
        todo!("port NIVissimXMLHandler_Geschwindigkeitsverteilungsdefinition::NIVissimXMLHandler_Geschwindigkeitsverteilungsdefinition")
    }

    /// C++: `NIVissimXMLHandler_Geschwindigkeitsverteilungsdefinition & NIVissimXMLHandler_Geschwindigkeitsverteilungsdefinition::operator=(const NIVissimXMLHandler_Geschwindigkeitsverteilungsdefinition &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, vW: &NIImporter_Vissim_NIVissimXMLHandler_Geschwindigkeitsverteilungsdefinition) -> &mut NIImporter_Vissim_NIVissimXMLHandler_Geschwindigkeitsverteilungsdefinition {
        todo!("port NIVissimXMLHandler_Geschwindigkeitsverteilungsdefinition::operator=")
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
        todo!("port NIVissimXMLHandler_Parkplatzdefinition::NIVissimXMLHandler_Parkplatzdefinition")
    }

    /// C++: `void NIVissimXMLHandler_Parkplatzdefinition::~NIVissimXMLHandler_Parkplatzdefinition() noexcept` [virtual]
    /// Destructor
    // NIImporter_Vissim::NIVissimXMLHandler_Parkplatzdefinition::~NIVissimXMLHandler_Parkplatzdefinition() { }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port NIVissimXMLHandler_Parkplatzdefinition::~NIVissimXMLHandler_Parkplatzdefinition")
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
        todo!("port NIVissimXMLHandler_Parkplatzdefinition::myStartElement")
    }

    /// C++: `void NIVissimXMLHandler_Parkplatzdefinition::NIVissimXMLHandler_Parkplatzdefinition(const NIVissimXMLHandler_Parkplatzdefinition &)` [private]
    /// invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(z: &NIImporter_Vissim_NIVissimXMLHandler_Parkplatzdefinition) -> Self {
        todo!("port NIVissimXMLHandler_Parkplatzdefinition::NIVissimXMLHandler_Parkplatzdefinition")
    }

    /// C++: `NIVissimXMLHandler_Parkplatzdefinition & NIVissimXMLHandler_Parkplatzdefinition::operator=(const NIVissimXMLHandler_Parkplatzdefinition &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, z: &NIImporter_Vissim_NIVissimXMLHandler_Parkplatzdefinition) -> &mut NIImporter_Vissim_NIVissimXMLHandler_Parkplatzdefinition {
        todo!("port NIVissimXMLHandler_Parkplatzdefinition::operator=")
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
        todo!("port NIVissimXMLHandler_Routenentscheidungsdefinition::NIVissimXMLHandler_Routenentscheidungsdefinition")
    }

    /// C++: `void NIVissimXMLHandler_Routenentscheidungsdefinition::~NIVissimXMLHandler_Routenentscheidungsdefinition() noexcept` [virtual]
    /// Destructor
    // NIImporter_Vissim::NIVissimXMLHandler_Routenentscheidungsdefinition::~NIVissimXMLHandler_Routenentscheidungsdefinition() { }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port NIVissimXMLHandler_Routenentscheidungsdefinition::~NIVissimXMLHandler_Routenentscheidungsdefinition")
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
        todo!("port NIVissimXMLHandler_Routenentscheidungsdefinition::myStartElement")
    }

    /// C++: `void NIVissimXMLHandler_Routenentscheidungsdefinition::myEndElement(int)` [virtual, protected]
    // void
    // NIImporter_Vissim::NIVissimXMLHandler_Routenentscheidungsdefinition::myEndElement(int /* element */) {
    //     --myHierarchyLevel;
    // }
    #[allow(unused_variables)]
    pub fn my_end_element(&mut self, element: i32) {
        todo!("port NIVissimXMLHandler_Routenentscheidungsdefinition::myEndElement")
    }

    /// C++: `void NIVissimXMLHandler_Routenentscheidungsdefinition::NIVissimXMLHandler_Routenentscheidungsdefinition(const NIVissimXMLHandler_Routenentscheidungsdefinition &)` [private]
    /// invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(r: &NIImporter_Vissim_NIVissimXMLHandler_Routenentscheidungsdefinition) -> Self {
        todo!("port NIVissimXMLHandler_Routenentscheidungsdefinition::NIVissimXMLHandler_Routenentscheidungsdefinition")
    }

    /// C++: `NIVissimXMLHandler_Routenentscheidungsdefinition & NIVissimXMLHandler_Routenentscheidungsdefinition::operator=(const NIVissimXMLHandler_Routenentscheidungsdefinition &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, r: &NIImporter_Vissim_NIVissimXMLHandler_Routenentscheidungsdefinition) -> &mut NIImporter_Vissim_NIVissimXMLHandler_Routenentscheidungsdefinition {
        todo!("port NIVissimXMLHandler_Routenentscheidungsdefinition::operator=")
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
        todo!("port NIVissimXMLHandler_Streckendefinition::NIVissimXMLHandler_Streckendefinition")
    }

    /// C++: `void NIVissimXMLHandler_Streckendefinition::~NIVissimXMLHandler_Streckendefinition() noexcept` [virtual]
    /// Destructor
    // NIImporter_Vissim::NIVissimXMLHandler_Streckendefinition::~NIVissimXMLHandler_Streckendefinition() { }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port NIVissimXMLHandler_Streckendefinition::~NIVissimXMLHandler_Streckendefinition")
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
        todo!("port NIVissimXMLHandler_Streckendefinition::myStartElement")
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
        todo!("port NIVissimXMLHandler_Streckendefinition::myEndElement")
    }

    /// C++: `void NIVissimXMLHandler_Streckendefinition::NIVissimXMLHandler_Streckendefinition(const NIVissimXMLHandler_Streckendefinition &)` [private]
    /// invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(s: &NIImporter_Vissim_NIVissimXMLHandler_Streckendefinition) -> Self {
        todo!("port NIVissimXMLHandler_Streckendefinition::NIVissimXMLHandler_Streckendefinition")
    }

    /// C++: `NIVissimXMLHandler_Streckendefinition & NIVissimXMLHandler_Streckendefinition::operator=(const NIVissimXMLHandler_Streckendefinition &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &NIImporter_Vissim_NIVissimXMLHandler_Streckendefinition) -> &mut NIImporter_Vissim_NIVissimXMLHandler_Streckendefinition {
        todo!("port NIVissimXMLHandler_Streckendefinition::operator=")
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
        todo!("port NIVissimXMLHandler_VWunschentscheidungsdefinition::NIVissimXMLHandler_VWunschentscheidungsdefinition")
    }

    /// C++: `void NIVissimXMLHandler_VWunschentscheidungsdefinition::~NIVissimXMLHandler_VWunschentscheidungsdefinition() noexcept` [virtual]
    /// Destructor
    // NIImporter_Vissim::NIVissimXMLHandler_VWunschentscheidungsdefinition::~NIVissimXMLHandler_VWunschentscheidungsdefinition() { }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port NIVissimXMLHandler_VWunschentscheidungsdefinition::~NIVissimXMLHandler_VWunschentscheidungsdefinition")
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
        todo!("port NIVissimXMLHandler_VWunschentscheidungsdefinition::myStartElement")
    }

    /// C++: `void NIVissimXMLHandler_VWunschentscheidungsdefinition::myEndElement(int)` [virtual, protected]
    // void
    // NIImporter_Vissim::NIVissimXMLHandler_VWunschentscheidungsdefinition::myEndElement(int /* element */) {
    //     --myHierarchyLevel;
    // }
    #[allow(unused_variables)]
    pub fn my_end_element(&mut self, element: i32) {
        todo!("port NIVissimXMLHandler_VWunschentscheidungsdefinition::myEndElement")
    }

    /// C++: `void NIVissimXMLHandler_VWunschentscheidungsdefinition::NIVissimXMLHandler_VWunschentscheidungsdefinition(const NIVissimXMLHandler_VWunschentscheidungsdefinition &)` [private]
    /// invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(vW: &NIImporter_Vissim_NIVissimXMLHandler_VWunschentscheidungsdefinition) -> Self {
        todo!("port NIVissimXMLHandler_VWunschentscheidungsdefinition::NIVissimXMLHandler_VWunschentscheidungsdefinition")
    }

    /// C++: `NIVissimXMLHandler_VWunschentscheidungsdefinition & NIVissimXMLHandler_VWunschentscheidungsdefinition::operator=(const NIVissimXMLHandler_VWunschentscheidungsdefinition &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, vW: &NIImporter_Vissim_NIVissimXMLHandler_VWunschentscheidungsdefinition) -> &mut NIImporter_Vissim_NIVissimXMLHandler_VWunschentscheidungsdefinition {
        todo!("port NIVissimXMLHandler_VWunschentscheidungsdefinition::operator=")
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
        todo!("port NIVissimXMLHandler_Zuflussdefinition::NIVissimXMLHandler_Zuflussdefinition")
    }

    /// C++: `void NIVissimXMLHandler_Zuflussdefinition::~NIVissimXMLHandler_Zuflussdefinition() noexcept` [virtual]
    /// Destructor
    // NIImporter_Vissim::NIVissimXMLHandler_Zuflussdefinition::~NIVissimXMLHandler_Zuflussdefinition() { }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port NIVissimXMLHandler_Zuflussdefinition::~NIVissimXMLHandler_Zuflussdefinition")
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
        todo!("port NIVissimXMLHandler_Zuflussdefinition::myStartElement")
    }

    /// C++: `void NIVissimXMLHandler_Zuflussdefinition::NIVissimXMLHandler_Zuflussdefinition(const NIVissimXMLHandler_Zuflussdefinition &)` [private]
    /// invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(z: &NIImporter_Vissim_NIVissimXMLHandler_Zuflussdefinition) -> Self {
        todo!("port NIVissimXMLHandler_Zuflussdefinition::NIVissimXMLHandler_Zuflussdefinition")
    }

    /// C++: `NIVissimXMLHandler_Zuflussdefinition & NIVissimXMLHandler_Zuflussdefinition::operator=(const NIVissimXMLHandler_Zuflussdefinition &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, z: &NIImporter_Vissim_NIVissimXMLHandler_Zuflussdefinition) -> &mut NIImporter_Vissim_NIVissimXMLHandler_Zuflussdefinition {
        todo!("port NIVissimXMLHandler_Zuflussdefinition::operator=")
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
        todo!("port VissimSingleTypeParser::VissimSingleTypeParser")
    }

    /// C++: `void VissimSingleTypeParser::~VissimSingleTypeParser()` [virtual]
    /// Destructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port VissimSingleTypeParser::~VissimSingleTypeParser")
    }

    /// C++: `bool VissimSingleTypeParser::parse(std::istream &)` [virtual, pure virtual]
    /// Parses a single data type. Returns whether no error occurred
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn parse(&mut self, from: &mut Unmapped /* std::istream */) -> bool {
        todo!("port VissimSingleTypeParser::parse")
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
    pub fn my_read(&mut self, from: &mut Unmapped /* std::istream */) -> String {
        todo!("port VissimSingleTypeParser::myRead")
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
    pub fn read_end_secure(&mut self, from: &mut Unmapped /* std::istream */, excl: &str) -> String {
        todo!("port VissimSingleTypeParser::readEndSecure")
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
    pub fn read_end_secure_2(&mut self, from: &mut Unmapped /* std::istream */, excl: &Vec<String>) -> String {
        todo!("port VissimSingleTypeParser::readEndSecure")
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
    pub fn override_optional_label(&mut self, from: &mut Unmapped /* std::istream */, tag: &str) -> String {
        todo!("port VissimSingleTypeParser::overrideOptionalLabel")
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
    pub fn get_position(&mut self, from: &mut Unmapped /* std::istream */) -> Position {
        todo!("port VissimSingleTypeParser::getPosition")
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
    pub fn parse_assigned_vehicle_types(&mut self, from: &mut Unmapped /* std::istream */, next: &str) -> Vec<i32> {
        todo!("port VissimSingleTypeParser::parseAssignedVehicleTypes")
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
    pub fn read_ext_edge_point_def(&mut self, from: &mut Unmapped /* std::istream */) -> NIVissimExtendedEdgePoint {
        todo!("port VissimSingleTypeParser::readExtEdgePointDef")
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
    pub fn read_name(&mut self, from: &mut Unmapped /* std::istream */) -> String {
        todo!("port VissimSingleTypeParser::readName")
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
    pub fn skip_overreading(&mut self, from: &mut Unmapped /* std::istream */, name: &str) -> bool {
        todo!("port VissimSingleTypeParser::skipOverreading")
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
    pub fn read_until(&mut self, from: &mut Unmapped /* std::istream */, name: &str) {
        todo!("port VissimSingleTypeParser::readUntil")
    }

    /// C++: `VissimSingleTypeParser & VissimSingleTypeParser::operator=(const VissimSingleTypeParser &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &NIImporter_Vissim_VissimSingleTypeParser) -> &mut NIImporter_Vissim_VissimSingleTypeParser {
        todo!("port VissimSingleTypeParser::operator=")
    }

}

