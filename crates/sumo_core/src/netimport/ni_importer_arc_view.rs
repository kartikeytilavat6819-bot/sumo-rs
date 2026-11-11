//! Port of netimport/NIImporter_ArcView.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::file_helpers::is_readable;
use utils::common::msg_handler::{MsgType, with_instance, write_error};
use utils::common::string_utils::starts_with;
use utils::tlf;

/// OptionsCont lookups return Result (C++ throws ProcessError); the options used here are registered
/// in fillOptions, so a failure is a programming error and is reported as a panic.
fn must<T>(r: Result<T, ProcessError>) -> T {
    r.unwrap_or_else(|e| panic!("{e}"))
}

/// Importer for networks stored in ArcView-shape format
/// C++ class `NIImporter_ArcView` (netimport/NIImporter_ArcView.h)
pub struct NIImporter_ArcView {
    /// The options to use
    pub myOptions: Ptr<OptionsCont>,
    /// The name of the shape file
    pub mySHPName: String,
    /// A running number to assure unique edge ids
    pub myNameAddition: i32,
    /// The container to add nodes to
    pub myNodeCont: Ptr<NBNodeCont>,
    /// The container to add edges to
    pub myEdgeCont: Ptr<NBEdgeCont>,
    /// The container to get the types from
    pub myTypeCont: Ptr<NBTypeCont>,
    /// Whether the speed is given in km/h
    pub mySpeedInKMH: bool,
    /// A running number to assure unique ids (as fallback)
    pub myRunningEdgeID: i32,
    pub myRunningNodeID: i32,
}

impl NIImporter_ArcView {
    /// C++: `void NIImporter_ArcView::loadNetwork(const OptionsCont &, NBNetBuilder &)` [static]
    /// Loads content of the optionally given ArcView Shape files
    // void
    // NIImporter_ArcView::loadNetwork(const OptionsCont& oc, NBNetBuilder& nb) {
    //     if (!oc.isSet("shapefile-prefix")) {
    //         return;
    //     }
    //     // check whether the correct set of entries is given
    //     //  and compute all file names
    //     const std::string dbf_file = oc.getString("shapefile-prefix") + ".dbf";
    //     const std::string shp_file = oc.getString("shapefile-prefix") + ".shp";
    //     const std::string shx_file = oc.getString("shapefile-prefix") + ".shx";
    //     if (!StringUtils::startsWith(shp_file, "/vsi")) {
    //         // if we are not using a virtual file system, check whether the files do exist
    //         if (!FileHelpers::isReadable(dbf_file)) {
    //             WRITE_ERROR("File not accessible: " + dbf_file);
    //         }
    //         if (!FileHelpers::isReadable(shp_file)) {
    //             WRITE_ERROR("File not accessible: " + shp_file);
    //         }
    //         if (!FileHelpers::isReadable(shx_file)) {
    //             WRITE_ERROR("File not accessible: " + shx_file);
    //         }
    //     }
    //     if (MsgHandler::getErrorInstance()->wasInformed()) {
    //         return;
    //     }
    //     // load the arcview files
    //     NIImporter_ArcView loader(oc, nb.getNodeCont(), nb.getEdgeCont(), nb.getTypeCont(),
    //                               shp_file, oc.getBool("speed-in-kmh"));
    //     loader.load();
    // }
    #[allow(unused_variables)]
    pub fn load_network(oc: &OptionsCont, nb: &mut NBNetBuilder) {
        if !must(oc.is_set("shapefile-prefix", true)) {
            return;
        }
        // check whether the correct set of entries is given
        //  and compute all file names
        let dbf_file = must(oc.get_string("shapefile-prefix")) + ".dbf";
        let shp_file = must(oc.get_string("shapefile-prefix")) + ".shp";
        let shx_file = must(oc.get_string("shapefile-prefix")) + ".shx";
        if !starts_with(&shp_file, "/vsi") {
            // if we are not using a virtual file system, check whether the files do exist
            if !is_readable(&dbf_file) {
                write_error(&format!("File not accessible: {dbf_file}"));
            }
            if !is_readable(&shp_file) {
                write_error(&format!("File not accessible: {shp_file}"));
            }
            if !is_readable(&shx_file) {
                write_error(&format!("File not accessible: {shx_file}"));
            }
        }
        if with_instance(MsgType::Error, |h| h.was_informed()) {
            return;
        }
        // load the arcview files
        let nc = Ptr::from_mut(nb.get_node_cont());
        let ec = Ptr::from_mut(nb.get_edge_cont());
        let tc = Ptr::from_mut(nb.get_type_cont());
        let mut loader = NIImporter_ArcView::new(oc, nc.m(), ec.m(), tc.m(), &shp_file, must(oc.get_bool("speed-in-kmh")));
        loader.load();
    }

    /// C++: `void NIImporter_ArcView::NIImporter_ArcView(const OptionsCont &, NBNodeCont &, NBEdgeCont &, NBTypeCont &, const std::string &, bool)` [protected]
    /// Constructor
    // NIImporter_ArcView::NIImporter_ArcView(const OptionsCont& oc,
    //                                        NBNodeCont& nc,
    //                                        NBEdgeCont& ec,
    //                                        NBTypeCont& tc,
    //                                        const std::string& shp_name,
    //                                        bool speedInKMH)
    //     : myOptions(oc), mySHPName(shp_name),
    //       myNameAddition(0),
    //       myNodeCont(nc), myEdgeCont(ec), myTypeCont(tc),
    //       mySpeedInKMH(speedInKMH),
    //       myRunningEdgeID(0),
    //       myRunningNodeID(0) {
    // }
    #[allow(unused_variables)]
    pub fn new(oc: &OptionsCont, nc: &mut NBNodeCont, ec: &mut NBEdgeCont, tc: &mut NBTypeCont, shp_name: &str, speedInKMH: bool) -> Self {
        NIImporter_ArcView {
            myOptions: Ptr::new(oc),
            mySHPName: shp_name.to_string(),
            myNameAddition: 0,
            myNodeCont: Ptr::from_mut(nc),
            myEdgeCont: Ptr::from_mut(ec),
            myTypeCont: Ptr::from_mut(tc),
            mySpeedInKMH: speedInKMH,
            myRunningEdgeID: 0,
            myRunningNodeID: 0,
        }
    }

    /// C++: `void NIImporter_ArcView::~NIImporter_ArcView() noexcept` [protected]
    /// Destructor
    // NIImporter_ArcView::~NIImporter_ArcView() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty

    }

    /// C++: `void NIImporter_ArcView::load()` [protected]
    /// Loads the shape files
    // void
    // NIImporter_ArcView::load() {
    // #ifdef HAVE_GDAL
    //     PROGRESS_BEGIN_MESSAGE("Loading data from '" + mySHPName + "'");
    // #if GDAL_VERSION_MAJOR < 2
    //     OGRRegisterAll();
    //     OGRDataSource* poDS = OGRSFDriverRegistrar::Open(mySHPName.c_str(), FALSE);
    // #else
    //     GDALAllRegister();
    //     GDALDataset* poDS = (GDALDataset*)GDALOpenEx(mySHPName.c_str(), GDAL_OF_VECTOR | GA_ReadOnly, NULL, NULL, NULL);
    // #endif
    //     if (poDS == NULL) {
    //         WRITE_ERRORF(TL("Could not open shape description '%'."), mySHPName);
    //         return;
    //     }
    //
    //     // begin file parsing
    //     OGRLayer* poLayer = poDS->GetLayer(0);
    //     poLayer->ResetReading();
    //
    //     // build coordinate transformation
    // #if GDAL_VERSION_MAJOR < 3
    //     OGRSpatialReference* origTransf = poLayer->GetSpatialRef();
    // #else
    //     const OGRSpatialReference* origTransf = poLayer->GetSpatialRef();
    // #endif
    //     OGRSpatialReference destTransf;
    //     // use wgs84 as destination
    //     destTransf.SetWellKnownGeogCS("WGS84");
    // #if GDAL_VERSION_MAJOR > 2
    //     if (myOptions.getBool("shapefile.traditional-axis-mapping") || origTransf != nullptr) {
    //         destTransf.SetAxisMappingStrategy(OAMS_TRADITIONAL_GIS_ORDER);
    //     }
    // #endif
    //     OGRCoordinateTransformation* poCT = origTransf == nullptr ? nullptr : OGRCreateCoordinateTransformation(origTransf, &destTransf);
    //     if (poCT == nullptr) {
    //         if (myOptions.getBool("shapefile.guess-projection")) {
    //             OGRSpatialReference origTransf2;
    //             origTransf2.SetWellKnownGeogCS("WGS84");
    //             poCT = OGRCreateCoordinateTransformation(&origTransf2, &destTransf);
    //         }
    //     }
    //
    //     const bool saveOrigIDs = OptionsCont::getOptions().getBool("output.original-names");
    //     OGRFeature* poFeature;
    //     poLayer->ResetReading();
    //
    //     const double nodeJoinDist = myOptions.getFloat("shapefile.node-join-dist");
    //     const std::vector<std::string> params = myOptions.getStringVector("shapefile.add-params");
    //
    //     int featureIndex = 0;
    //     bool warnNotUnique = true;
    //     bool warnMissingProjection = true;
    //     std::string idPrefix = ""; // prefix for non-unique street-id values
    //     std::map<std::string, int> idIndex; // running index to make street-id unique
    //     while ((poFeature = poLayer->GetNextFeature()) != NULL) {
    //         // read in edge attributes
    //         if (featureIndex == 0) {
    //             WRITE_MESSAGE("Available fields: " + toString(getFieldNames(poFeature)));
    //         }
    //         std::string id;
    //         std::string name;
    //         std::string from_node;
    //         std::string to_node;
    //         if (!getStringEntry(poFeature, "shapefile.street-id", "LINK_ID", true, id)) {
    //             WRITE_ERRORF(TL("Needed field '%' (street-id) is missing."), id);
    //             id = "";
    //         }
    //         if (id == "") {
    //             id = toString(myRunningEdgeID++);
    //         }
    //
    //         getStringEntry(poFeature, "shapefile.name", "ST_NAME", true, name);
    //         name = StringUtils::replace(name, "&", "&amp;");
    //
    //         if (!getStringEntry(poFeature, "shapefile.from-id", "REF_IN_ID", true, from_node)) {
    //             WRITE_ERRORF(TL("Needed field '%' (from node id) is missing."), from_node);
    //             from_node = "";
    //         }
    //         if (!getStringEntry(poFeature, "shapefile.to-id", "NREF_IN_ID", true, to_node)) {
    //             WRITE_ERRORF(TL("Needed field '%' (to node id) is missing."), to_node);
    //             to_node = "";
    //         }
    //
    //         if (from_node == "" || to_node == "") {
    //             from_node = toString(myRunningNodeID++);
    //             to_node = toString(myRunningNodeID++);
    //         }
    //
    //         std::string type;
    //         if (myOptions.isSet("shapefile.type-id") && poFeature->GetFieldIndex(myOptions.getString("shapefile.type-id").c_str()) >= 0) {
    //             type = poFeature->GetFieldAsString(myOptions.getString("shapefile.type-id").c_str());
    //         } else if (poFeature->GetFieldIndex("ST_TYP_AFT") >= 0) {
    //             type = poFeature->GetFieldAsString("ST_TYP_AFT");
    //         }
    //         if ((type != "" || myOptions.isSet("shapefile.type-id")) && !myTypeCont.knows(type)) {
    //             WRITE_WARNINGF(TL("Unknown type '%' for edge '%'"), type, id);
    //         }
    //         bool oneway = myTypeCont.knows(type) ? myTypeCont.getEdgeTypeIsOneWay(type) : false;
    //         double speed = getSpeed(*poFeature, id);
    //         int nolanes = getLaneNo(*poFeature, id, speed);
    //         int priority = getPriority(*poFeature, id);
    //         double width = getLaneWidth(*poFeature, id, nolanes);
    //         double length = getLength(*poFeature, id);
    //         if (nolanes <= 0 || speed <= 0) {
    //             if (myOptions.getBool("shapefile.use-defaults-on-failure")) {
    //                 nolanes = nolanes <= 0 ? myTypeCont.getEdgeTypeNumLanes(type) : nolanes;
    //                 speed = speed <= 0 ? myTypeCont.getEdgeTypeSpeed(type) : speed;
    //             } else {
    //                 const std::string lanesField = myOptions.isSet("shapefile.laneNumber") ? myOptions.getString("shapefile.laneNumber") : "nolanes";
    //                 const std::string speedField = myOptions.isSet("shapefile.speed") ? myOptions.getString("shapefile.speed") : "speed";
    //                 WRITE_ERRORF(TL("Required field '%' or '%' is missing (add fields or set option --shapefile.use-defaults-on-failure)."), lanesField, speedField);
    //                 WRITE_ERROR("Available fields: " + toString(getFieldNames(poFeature)));
    //                 OGRFeature::DestroyFeature(poFeature);
    //                 return;
    //             }
    //         }
    //         if (mySpeedInKMH) {
    //             speed /= 3.6;
    //         }
    //
    //
    //         // read in the geometry
    //         OGRGeometry* poGeometry = poFeature->GetGeometryRef();
    //         OGRwkbGeometryType gtype = poGeometry->getGeometryType();
    //         if (gtype != wkbLineString && gtype != wkbLineString25D) {
    //             OGRFeature::DestroyFeature(poFeature);
    //             WRITE_ERRORF(TL("Road geometry must be of type 'linestring' or 'linestring25D' (found '%')"), toString(gtype));
    //             return;
    //         }
    //         OGRLineString* cgeom = (OGRLineString*) poGeometry;
    //         if (poCT == nullptr && warnMissingProjection) {
    //             int outOfRange = 0;
    //             for (int j = 0; j < cgeom->getNumPoints(); j++) {
    //                 if (fabs(cgeom->getX(j)) > 180 || fabs(cgeom->getY(j)) > 90) {
    //                     outOfRange++;
    //                 }
    //             }
    //             if (2 * outOfRange > cgeom->getNumPoints()) {
    //                 WRITE_WARNING(TL("No coordinate system found and coordinates look already projected."));
    //                 GeoConvHelper::init("!", GeoConvHelper::getProcessing().getOffset(), GeoConvHelper::getProcessing().getOrigBoundary(), GeoConvHelper::getProcessing().getConvBoundary());
    //             } else {
    //                 WRITE_WARNING(TL("Could not find geo coordinate system, assuming WGS84."));
    //             }
    //             warnMissingProjection = false;
    //         }
    //         if (poCT != nullptr) {
    //             // try transform to wgs84
    //             cgeom->transform(poCT);
    //         }
    //
    //         PositionVector shape;
    //         for (int j = 0; j < cgeom->getNumPoints(); j++) {
    //             Position pos(cgeom->getX(j), cgeom->getY(j), cgeom->getZ(j));
    //             if (!NBNetBuilder::transformCoordinate(pos)) {
    //                 WRITE_WARNINGF(TL("Unable to project coordinates for edge '%'."), id);
    //             }
    //             shape.push_back_noDoublePos(pos);
    //         }
    //
    //         // build from-node
    //         NBNode* from = myNodeCont.retrieve(from_node);
    //         if (from == nullptr) {
    //             Position from_pos = shape[0];
    //             std::vector<NBNode*> cands = myNodeCont.retrieveByPos(from_pos, nodeJoinDist);
    //             if (!cands.empty()) {
    //                 from = cands.front();
    //             }
    //             if (from == nullptr) {
    //                 from = new NBNode(from_node, from_pos);
    //                 if (!myNodeCont.insert(from)) {
    //                     WRITE_ERRORF(TL("Node '%' could not be added"), from_node);
    //                     delete from;
    //                     continue;
    //                 }
    //             }
    //         }
    //         // build to-node
    //         NBNode* to = myNodeCont.retrieve(to_node);
    //         if (to == nullptr) {
    //             Position to_pos = shape[-1];
    //             std::vector<NBNode*> cands = myNodeCont.retrieveByPos(to_pos, nodeJoinDist);
    //             if (!cands.empty()) {
    //                 to = cands.front();
    //             }
    //             if (to == nullptr) {
    //                 to = new NBNode(to_node, to_pos);
    //                 if (!myNodeCont.insert(to)) {
    //                     WRITE_ERRORF(TL("Node '%' could not be added"), to_node);
    //                     delete to;
    //                     continue;
    //                 }
    //             }
    //         }
    //
    //         if (from == to) {
    //             WRITE_WARNINGF(TL("Edge '%' connects identical nodes, skipping."), id);
    //             continue;
    //         }
    //
    //         // retrieve the information whether the street is bi-directional
    //         std::string dir;
    //         int index = poFeature->GetDefnRef()->GetFieldIndex("DIR_TRAVEL");
    //         if (index >= 0 && poFeature->IsFieldSet(index)) {
    //             dir = poFeature->GetFieldAsString(index);
    //         }
    //         const std::string origID = saveOrigIDs ? id : "";
    //         // check for duplicate ids
    //         NBEdge* const existing = myEdgeCont.retrieve(id);
    //         NBEdge* const existingReverse = myEdgeCont.retrieve("-" + id);
    //         if (existing != nullptr || existingReverse != nullptr) {
    //             if ((existing != nullptr && existing->getGeometry() == shape)
    //                     || (existingReverse != nullptr && existingReverse->getGeometry() == shape.reverse())) {
    //                 WRITE_ERRORF(TL("Edge '%' is not unique."), (existing != nullptr ? id : existingReverse->getID()));
    //             } else {
    //                 if (idIndex.count(id) == 0) {
    //                     idIndex[id] = 0;
    //                 }
    //                 idIndex[id]++;
    //                 idPrefix = id;
    //                 id += "#" + toString(idIndex[id]);
    //                 if (warnNotUnique) {
    //                     WRITE_WARNINGF(TL("Edge '%' is not unique. Renaming subsequent edge to '%'."), idPrefix, id);
    //                     warnNotUnique = false;
    //                 }
    //             }
    //         }
    //         // add positive direction if wanted
    //         if (dir == "B" || dir == "F" || dir == "" || myOptions.getBool("shapefile.all-bidirectional")) {
    //             if (myEdgeCont.retrieve(id) == 0) {
    //                 LaneSpreadFunction spread = dir == "B" || dir == "FALSE" ? LaneSpreadFunction::RIGHT : LaneSpreadFunction::CENTER;
    //                 if (spread == LaneSpreadFunction::RIGHT && OptionsCont::getOptions().getString("default.spreadtype") == toString(LaneSpreadFunction::ROADCENTER)) {
    //                     spread = LaneSpreadFunction::ROADCENTER;
    //                 }
    //                 NBEdge* edge = new NBEdge(id, from, to, type, speed, NBEdge::UNSPECIFIED_FRICTION, nolanes, priority, width, NBEdge::UNSPECIFIED_OFFSET, shape, spread, name, origID);
    //                 edge->setPermissions(myTypeCont.getEdgeTypePermissions(type));
    //                 edge->setLoadedLength(length);
    //                 myEdgeCont.insert(edge);
    //                 checkSpread(edge);
    //                 addParams(edge, poFeature, params);
    //             } else {
    //                 WRITE_ERRORF(TL("Could not create edge '%'. An edge with the same id already exists."), id);
    //             }
    //         }
    //         // add negative direction if wanted
    //         if ((dir == "B" || dir == "T" || myOptions.getBool("shapefile.all-bidirectional")) && !oneway) {
    //             if (myEdgeCont.retrieve("-" + id) == 0) {
    //                 LaneSpreadFunction spread = dir == "B" || dir == "FALSE" ? LaneSpreadFunction::RIGHT : LaneSpreadFunction::CENTER;
    //                 if (spread == LaneSpreadFunction::RIGHT && OptionsCont::getOptions().getString("default.spreadtype") == toString(LaneSpreadFunction::ROADCENTER)) {
    //                     spread = LaneSpreadFunction::ROADCENTER;
    //                 }
    //                 NBEdge* edge = new NBEdge("-" + id, to, from, type, speed, NBEdge::UNSPECIFIED_FRICTION, nolanes, priority, width, NBEdge::UNSPECIFIED_OFFSET, shape.reverse(), spread, name, origID);
    //                 edge->setPermissions(myTypeCont.getEdgeTypePermissions(type));
    //                 edge->setLoadedLength(length);
    //                 myEdgeCont.insert(edge);
    //                 checkSpread(edge);
    //                 addParams(edge, poFeature, params);
    //             } else {
    //                 WRITE_ERRORF(TL("Could not create edge '-%'. An edge with the same id already exists."), id);
    //             }
    //         }
    //         //
    //         OGRFeature::DestroyFeature(poFeature);
    //         featureIndex++;
    //     }
    // #if GDAL_VERSION_MAJOR < 2
    //     OGRDataSource::DestroyDataSource(poDS);
    // #else
    //     GDALClose(poDS);
    // #endif
    //     PROGRESS_DONE_MESSAGE();
    // #else
    //     WRITE_ERROR(TL("SUMO was compiled without GDAL support."));
    // #endif
    // }
    #[allow(unused_variables)]
    pub fn load(&mut self) {
        // shortcut: GDAL is not bound in this port, so only the non-GDAL branch (#else of HAVE_GDAL) is ported; add the OGR/GDAL shapefile reader when GDAL bindings exist
        write_error(&tlf!("SUMO was compiled without GDAL support."));
    }

    /// C++: `void NIImporter_ArcView::NIImporter_ArcView(const NIImporter_ArcView &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &NIImporter_ArcView) -> Self {
        panic!("NIImporter_ArcView copy constructor is invalidated (private and never defined in C++)")
    }

    /// C++: `NIImporter_ArcView & NIImporter_ArcView::operator=(const NIImporter_ArcView &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &NIImporter_ArcView) -> &mut NIImporter_ArcView {
        panic!("NIImporter_ArcView assignment operator is invalidated (private and never defined in C++)")
    }

}

