//! Port of netbuild/NBHeightMapper.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `std::vector<const Triangle *> NBHeightMapper::Triangles`
pub type NBHeightMapper_Triangles = Vec<Ptr<Triangle>>;

/// Set z-values for all network positions based on data from a height map
/// C++ class `NBHeightMapper` (netbuild/NBHeightMapper.h)
pub struct NBHeightMapper {
    pub myTriangles: NBHeightMapper_Triangles,
    /// The RTree for spatial queries
    pub myRTree: Unmapped /* RTree<NBHeightMapper::Triangle *, NBHeightMapper::Triangle, float, 2, NBHeightMapper::QueryResult>: use foreign::rtree::rtree::RTree with its 6 generic parameters */,
    /// raster height information in m for all loaded files
    pub myRasters: Vec<NBHeightMapper_RasterData>,
    /// dimensions of one pixel in raster data
    pub mySizeOfPixel: Position,
    /// convex boundary of all known triangles;
    pub myBoundary: Boundary,
}

// C++ static members of NBHeightMapper (port as module-level statics, see AGENTS.md):
//   static NBHeightMapper NBHeightMapper::myInstance;

impl NBHeightMapper {
    /// C++: `void NBHeightMapper::loadIfSet(OptionsCont &)` [static]
    /// loads height map data if any loading options are set
    // void
    // NBHeightMapper::loadIfSet(OptionsCont& oc) {
    //     if (oc.isSet("heightmap.geotiff")) {
    //         // parse file(s)
    //         std::vector<std::string> files = oc.getStringVector("heightmap.geotiff");
    //         for (std::vector<std::string>::const_iterator file = files.begin(); file != files.end(); ++file) {
    //             PROGRESS_BEGIN_MESSAGE("Parsing from GeoTIFF '" + *file + "'");
    //             int numFeatures = myInstance.loadTiff(*file);
    //             MsgHandler::getMessageInstance()->endProcessMsg(
    //                 " done (parsed " + toString(numFeatures) +
    //                 " features, Boundary: " + toString(myInstance.getBoundary()) + ").");
    //         }
    //     }
    //     if (oc.isSet("heightmap.shapefiles")) {
    //         // parse file(s)
    //         std::vector<std::string> files = oc.getStringVector("heightmap.shapefiles");
    //         for (std::vector<std::string>::const_iterator file = files.begin(); file != files.end(); ++file) {
    //             PROGRESS_BEGIN_MESSAGE("Parsing from shape-file '" + *file + "'");
    //             int numFeatures = myInstance.loadShapeFile(*file);
    //             MsgHandler::getMessageInstance()->endProcessMsg(
    //                 " done (parsed " + toString(numFeatures) +
    //                 " features, Boundary: " + toString(myInstance.getBoundary()) + ").");
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn load_if_set(oc: &mut OptionsCont) {
        todo!("port NBHeightMapper::loadIfSet")
    }

    /// C++: `const NBHeightMapper & NBHeightMapper::get()` [static]
    /// return the singleton instance (maybe 0)
    // const NBHeightMapper&
    // NBHeightMapper::get() {
    //     return myInstance;
    // }
    #[allow(unused_variables)]
    pub fn get() -> &'static NBHeightMapper {
        todo!("port NBHeightMapper::get")
    }

    /// C++: `bool NBHeightMapper::ready() const` [const]
    /// returns whether the NBHeightMapper has data
    // bool
    // NBHeightMapper::ready() const {
    //     return myRasters.size() > 0 || myTriangles.size() > 0;
    // }
    #[allow(unused_variables)]
    pub fn ready(&self) -> bool {
        todo!("port NBHeightMapper::ready")
    }

    /// C++: `const Boundary & NBHeightMapper::getBoundary()`
    /// returns the convex boundary of all known triangles
    // const Boundary& getBoundary() {
    //         return myInstance.myBoundary;
    //     }
    #[allow(unused_variables)]
    pub fn get_boundary(&mut self) -> &Boundary {
        todo!("port NBHeightMapper::getBoundary")
    }

    /// C++: `double NBHeightMapper::getZ(const Position &) const` [const]
    /// returns height for the given geo coordinate (WGS84)
    // double
    // NBHeightMapper::getZ(const Position& geo) const {
    //     if (!ready()) {
    //         WRITE_WARNING(TL("Cannot supply height since no height data was loaded"));
    //         return 0;
    //     }
    //     for (auto& item : myRasters) {
    //         const Boundary& boundary = item.boundary;
    //         float* raster = item.raster;
    //         double result = -1e6;
    //
    //         double x = geo.x();
    //         double y = geo.y();
    //
    // #ifdef HAVE_GDAL
    //         // Transform geo coordinates to the coordinate system of this
    //         // raster image for lookup in its raster, if applicable.
    //         if (item.transform != nullptr) {
    //             // Since the input coordinates are always WGS84 (they may be
    //             // transformed to it in NBNetBuilder::transformCoordinate), and
    //             // WGS84 uses latitude-longitude order (y-x), we have to swap the
    //             // input coordinates here.
    //             std::swap(x, y);
    //
    //             item.transform->Transform(1, &x, &y);
    //         }
    // #endif
    //
    //         if (boundary.around2D(x, y)) {
    //             const int xSize = item.xSize;
    //             const double normX = (x - boundary.xmin()) / mySizeOfPixel.x();
    //             const double normY = (y - boundary.ymax()) / mySizeOfPixel.y();
    //             PositionVector corners;
    //             corners.push_back(Position(floor(normX) + 0.5, floor(normY) + 0.5, raster[(int)normY * xSize + (int)normX]));
    //             if (normX - floor(normX) > 0.5) {
    //                 corners.push_back(Position(floor(normX) + 1.5, floor(normY) + 0.5, raster[(int)normY * xSize + (int)normX + 1]));
    //             } else {
    //                 corners.push_back(Position(floor(normX) - 0.5, floor(normY) + 0.5, raster[(int)normY * xSize + (int)normX - 1]));
    //             }
    //             if (normY - floor(normY) > 0.5 && ((int)normY + 1) < item.ySize) {
    //                 corners.push_back(Position(floor(normX) + 0.5, floor(normY) + 1.5, raster[((int)normY + 1) * xSize + (int)normX]));
    //             } else {
    //                 corners.push_back(Position(floor(normX) + 0.5, floor(normY) - 0.5, raster[((int)normY - 1) * xSize + (int)normX]));
    //             }
    //             result = Triangle(corners).getZ(Position(normX, normY));
    //         }
    //         if (result > -1e5 && result < 1e5) {
    //             return result;
    //         }
    //     }
    //     // coordinates in degrees hence a small search window
    //     float minB[2];
    //     float maxB[2];
    //     minB[0] = (float)geo.x() - 0.00001f;
    //     minB[1] = (float)geo.y() - 0.00001f;
    //     maxB[0] = (float)geo.x() + 0.00001f;
    //     maxB[1] = (float)geo.y() + 0.00001f;
    //     QueryResult queryResult;
    //     int hits = myRTree.Search(minB, maxB, queryResult);
    //     Triangles result = queryResult.triangles;
    //     assert(hits == (int)result.size());
    //     UNUSED_PARAMETER(hits); // only used for assertion
    //
    //     for (Triangles::iterator it = result.begin(); it != result.end(); it++) {
    //         const Triangle* triangle = *it;
    //         if (triangle->contains(geo)) {
    //             return triangle->getZ(geo);
    //         }
    //     }
    //     WRITE_WARNINGF(TL("Could not get height data for coordinate %"), toString(geo));
    //     return 0;
    // }
    #[allow(unused_variables)]
    pub fn get_z(&self, geo: &Position) -> f64 {
        todo!("port NBHeightMapper::getZ")
    }

    /// C++: `void NBHeightMapper::NBHeightMapper()` [private]
    /// private constructor and destructor (Singleton)
    // NBHeightMapper::NBHeightMapper():
    //     myRTree(&Triangle::addSelf) {
    // }
    #[allow(unused_variables)]
    pub fn new() -> Self {
        todo!("port NBHeightMapper::NBHeightMapper")
    }

    /// C++: `void NBHeightMapper::~NBHeightMapper() noexcept` [private]
    // NBHeightMapper::~NBHeightMapper() {
    //     clearData();
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port NBHeightMapper::~NBHeightMapper")
    }

    /// C++: `void NBHeightMapper::addTriangle(PositionVector)` [private]
    /// adds one triangles worth of height data
    // void
    // NBHeightMapper::addTriangle(PositionVector corners) {
    //     Triangle* triangle = new Triangle(corners);
    //     myTriangles.push_back(triangle);
    //     Boundary b = corners.getBoxBoundary();
    //     const float cmin[2] = {(float) b.xmin(), (float) b.ymin()};
    //     const float cmax[2] = {(float) b.xmax(), (float) b.ymax()};
    //     myRTree.Insert(cmin, cmax, triangle);
    // }
    #[allow(unused_variables)]
    pub fn add_triangle(&mut self, corners: PositionVector) {
        todo!("port NBHeightMapper::addTriangle")
    }

    /// C++: `int NBHeightMapper::loadShapeFile(const std::string &)` [private]
    /// load height data from Arcgis-shape file and returns the number of parsed features
    // int
    // NBHeightMapper::loadShapeFile(const std::string& file) {
    // #ifdef HAVE_GDAL
    // #if GDAL_VERSION_MAJOR < 2
    //     OGRRegisterAll();
    //     OGRDataSource* ds = OGRSFDriverRegistrar::Open(file.c_str(), FALSE);
    // #else
    //     GDALAllRegister();
    //     GDALDataset* ds = (GDALDataset*)GDALOpenEx(file.c_str(), GDAL_OF_VECTOR | GA_ReadOnly, nullptr, nullptr, nullptr);
    // #endif
    //     if (ds == nullptr) {
    //         throw ProcessError(TLF("Could not open shape file '%'.", file));
    //     }
    //
    //     // begin file parsing
    //     OGRLayer* layer = ds->GetLayer(0);
    //     layer->ResetReading();
    //
    //     // triangle coordinates are stored in WGS84 and later matched with network coordinates in WGS84
    //     // build coordinate transformation
    // #if GDAL_VERSION_MAJOR < 3
    //     OGRSpatialReference* sr_src = layer->GetSpatialRef();
    // #else
    //     const OGRSpatialReference* sr_src = layer->GetSpatialRef();
    // #endif
    //     OGRSpatialReference sr_dest;
    //     sr_dest.SetWellKnownGeogCS("WGS84");
    //     OGRCoordinateTransformation* toWGS84 = OGRCreateCoordinateTransformation(sr_src, &sr_dest);
    //     if (toWGS84 == nullptr) {
    //         WRITE_WARNING(TL("Could not create geocoordinates converter; check whether proj.4 is installed."));
    //     }
    //
    //     int numFeatures = 0;
    //     OGRFeature* feature;
    //     layer->ResetReading();
    //     while ((feature = layer->GetNextFeature()) != nullptr) {
    //         OGRGeometry* geom = feature->GetGeometryRef();
    //         assert(geom != 0);
    //
    //         OGRwkbGeometryType gtype = geom->getGeometryType();
    //         if (gtype == wkbPolygon) {
    //             assert(std::string(geom->getGeometryName()) == std::string("POLYGON"));
    //             // try transform to wgs84
    //             geom->transform(toWGS84);
    //             OGRLinearRing* cgeom = ((OGRPolygon*) geom)->getExteriorRing();
    //             // assume TIN with with 4 points and point0 == point3
    //             assert(cgeom->getNumPoints() == 4);
    //             PositionVector corners;
    //             for (int j = 0; j < 3; j++) {
    //                 Position pos((double) cgeom->getX(j), (double) cgeom->getY(j), (double) cgeom->getZ(j));
    //                 corners.push_back(pos);
    //                 myBoundary.add(pos);
    //             }
    //             addTriangle(corners);
    //             numFeatures++;
    //         } else {
    //             WRITE_WARNINGF(TL("Ignored heightmap feature type %"), geom->getGeometryName());
    //         }
    //
    //         /*
    //         switch (gtype) {
    //             case wkbPolygon: {
    //                 break;
    //             }
    //             case wkbPoint: {
    //                 WRITE_WARNING(TL("got wkbPoint"));
    //                 break;
    //             }
    //             case wkbLineString: {
    //                 WRITE_WARNING(TL("got wkbLineString"));
    //                 break;
    //             }
    //             case wkbMultiPoint: {
    //                 WRITE_WARNING(TL("got wkbMultiPoint"));
    //                 break;
    //             }
    //             case wkbMultiLineString: {
    //                 WRITE_WARNING(TL("got wkbMultiLineString"));
    //                 break;
    //             }
    //             case wkbMultiPolygon: {
    //                 WRITE_WARNING(TL("got wkbMultiPolygon"));
    //                 break;
    //             }
    //             default:
    //                 WRITE_WARNING(TL("Unsupported shape type occurred"));
    //             break;
    //         }
    //         */
    //         OGRFeature::DestroyFeature(feature);
    //     }
    // #if GDAL_VERSION_MAJOR < 2
    //     OGRDataSource::DestroyDataSource(ds);
    // #else
    //     GDALClose(ds);
    // #endif
    //     OCTDestroyCoordinateTransformation(reinterpret_cast<OGRCoordinateTransformationH>(toWGS84));
    //     OGRCleanupAll();
    //     return numFeatures;
    // #else
    //     UNUSED_PARAMETER(file);
    //     WRITE_ERROR(TL("Cannot load shape file since SUMO was compiled without GDAL support."));
    //     return 0;
    // #endif
    // }
    #[allow(unused_variables)]
    pub fn load_shape_file(&mut self, file: &str) -> i32 {
        todo!("port NBHeightMapper::loadShapeFile")
    }

    /// C++: `int NBHeightMapper::loadTiff(const std::string &)` [private]
    /// load height data from GeoTIFF file and returns the number of non void pixels
    // int
    // NBHeightMapper::loadTiff(const std::string& file) {
    // #ifdef HAVE_GDAL
    //     GDALAllRegister();
    //     GDALDataset* poDataset = (GDALDataset*)GDALOpen(file.c_str(), GA_ReadOnly);
    //     if (poDataset == 0) {
    //         WRITE_ERROR(TL("Cannot load GeoTIFF file."));
    //         return 0;
    //     }
    //     Boundary boundary;
    //     const int xSize = poDataset->GetRasterXSize();
    //     const int ySize = poDataset->GetRasterYSize();
    //     double adfGeoTransform[6];
    //     if (poDataset->GetGeoTransform(adfGeoTransform) == CE_None) {
    //         Position topLeft(adfGeoTransform[0], adfGeoTransform[3]);
    //         mySizeOfPixel.set(adfGeoTransform[1], adfGeoTransform[5]);
    //         const double horizontalSize = xSize * mySizeOfPixel.x();
    //         const double verticalSize = ySize * mySizeOfPixel.y();
    //         boundary.add(topLeft);
    //         boundary.add(topLeft.x() + horizontalSize, topLeft.y() + verticalSize);
    //     } else {
    //         WRITE_ERRORF(TL("Could not parse geo information from %."), file);
    //         return 0;
    //     }
    //     const int picSize = xSize * ySize;
    //     float* raster = (float*)CPLMalloc(sizeof(float) * picSize);
    //     bool ok = true;
    //     for (int i = 1; i <= poDataset->GetRasterCount(); i++) {
    //         GDALRasterBand* poBand = poDataset->GetRasterBand(i);
    //         if (poBand->GetColorInterpretation() != GCI_GrayIndex) {
    //             WRITE_ERRORF(TL("Unknown color band in %."), file);
    //             clearData();
    //             ok = false;
    //             break;
    //         }
    //         assert(xSize == poBand->GetXSize() && ySize == poBand->GetYSize());
    //         if (poBand->RasterIO(GF_Read, 0, 0, xSize, ySize, raster, xSize, ySize, GDT_Float32, 0, 0) == CE_Failure) {
    //             WRITE_ERRORF(TL("Failure in reading %."), file);
    //             clearData();
    //             ok = false;
    //             break;
    //         }
    //     }
    //     double min = std::numeric_limits<double>::max();
    //     double max = -std::numeric_limits<double>::max();
    //     for (int i = 0; i < picSize; i++) {
    //         min = MIN2(min, (double)raster[i]);
    //         max = MAX2(max, (double)raster[i]);
    //     }
    //
    //     // Make a copy, GDALClose will destroy the original
    // #if GDAL_VERSION_MAJOR < 3
    //     OGRSpatialReference spatialRef;
    //     char* wkt = const_cast<char*>(poDataset->GetProjectionRef());
    //     spatialRef.importFromWkt(&wkt);
    // #else
    //     OGRSpatialReference spatialRef(*poDataset->GetSpatialRef());
    // #endif
    //     GDALClose(poDataset);
    //     if (ok) {
    //         WRITE_MESSAGE("Read geotiff heightmap with size " + toString(xSize) + "," + toString(ySize)
    //                       + " for geo boundary [" + toString(boundary)
    //                       + "] with elevation range [" + toString(min) + "," + toString(max) + "].");
    //         OGRSpatialReference wgs;
    //         wgs.SetWellKnownGeogCS("WGS84");
    //         myRasters.push_back(RasterData{raster, boundary, xSize, ySize, OGRCreateCoordinateTransformation(&wgs, &spatialRef)});
    //         return picSize;
    //     }
    //     return 0;
    // #else
    //     UNUSED_PARAMETER(file);
    //     WRITE_ERROR(TL("Cannot load GeoTIFF file since SUMO was compiled without GDAL support."));
    //     return 0;
    // #endif
    // }
    #[allow(unused_variables)]
    pub fn load_tiff(&mut self, file: &str) -> i32 {
        todo!("port NBHeightMapper::loadTiff")
    }

    /// C++: `void NBHeightMapper::clearData()` [private]
    /// clears loaded data
    // void
    // NBHeightMapper::clearData() {
    //     for (Triangles::iterator it = myTriangles.begin(); it != myTriangles.end(); it++) {
    //         delete *it;
    //     }
    //     myTriangles.clear();
    // #ifdef HAVE_GDAL
    //     for (auto& item : myRasters) {
    //         CPLFree(item.raster);
    //         if (item.transform != nullptr) {
    //             delete item.transform;
    //         }
    //     }
    //     myRasters.clear();
    // #endif
    //     myBoundary.reset();
    // }
    #[allow(unused_variables)]
    pub fn clear_data(&mut self) {
        todo!("port NBHeightMapper::clearData")
    }

    /// C++: `void NBHeightMapper::NBHeightMapper(const NBHeightMapper &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &NBHeightMapper) -> Self {
        todo!("port NBHeightMapper::NBHeightMapper")
    }

    /// C++: `NBHeightMapper & NBHeightMapper::operator=(const NBHeightMapper &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &NBHeightMapper) -> &mut NBHeightMapper {
        todo!("port NBHeightMapper::operator=")
    }

}

/// class for cirumventing the const-restriction of RTree::Search-context
/// C++ class `NBHeightMapper::QueryResult` (netbuild/NBHeightMapper.h)
pub struct NBHeightMapper_QueryResult {
    pub triangles: NBHeightMapper_Triangles,
}

impl NBHeightMapper_QueryResult {
    /// C++: `void QueryResult::QueryResult()`
    // QueryResult() {}
    #[allow(unused_variables)]
    pub fn new() -> Self {
        todo!("port QueryResult::QueryResult")
    }

    /// C++: `void QueryResult::~QueryResult() noexcept`
    // ~QueryResult() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port QueryResult::~QueryResult")
    }

    /// C++: `void QueryResult::add(Triangle *) const` [const]
    // void add(Triangle* triangle) const {
    //             triangles.push_back(triangle);
    //         }
    #[allow(unused_variables)]
    pub fn add(&self, triangle: Ptr<Triangle>) {
        todo!("port QueryResult::add")
    }

}

/// C++ class `NBHeightMapper::RasterData` (netbuild/NBHeightMapper.h)
pub struct NBHeightMapper_RasterData {
    pub raster: Ptr<f32>,
    pub boundary: Boundary,
    pub xSize: i32,
    pub ySize: i32,
}

impl NBHeightMapper_RasterData {
}

/// C++ class `NBHeightMapper::Triangle` (netbuild/NBHeightMapper.h)
pub struct NBHeightMapper_Triangle {
    /// the corners of the triangle
    pub myCorners: PositionVector,
}

impl NBHeightMapper_Triangle {
    /// C++: `void Triangle::Triangle(const PositionVector &)`
    // NBHeightMapper::Triangle::Triangle(const PositionVector& corners):
    //     myCorners(corners) {
    //     assert(myCorners.size() == 3);
    //     // @todo assert non-colinearity
    // }
    #[allow(unused_variables)]
    pub fn new(corners: &PositionVector) -> Self {
        todo!("port Triangle::Triangle")
    }

    /// C++: `void Triangle::~Triangle() noexcept`
    // ~Triangle() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port Triangle::~Triangle")
    }

    /// C++: `void Triangle::addSelf(const QueryResult &) const` [const]
    /// callback for RTree search
    // void
    // NBHeightMapper::Triangle::addSelf(const QueryResult& queryResult) const {
    //     queryResult.triangles.push_back(this);
    // }
    #[allow(unused_variables)]
    pub fn add_self(&self, queryResult: &NBHeightMapper_QueryResult) {
        todo!("port Triangle::addSelf")
    }

    /// C++: `bool Triangle::contains(const Position &) const` [const]
    /// checks whether pos lies within triangle (only checks x,y)
    // bool
    // NBHeightMapper::Triangle::contains(const Position& pos) const {
    //     return myCorners.around(pos);
    // }
    #[allow(unused_variables)]
    pub fn contains(&self, pos: &Position) -> bool {
        todo!("port Triangle::contains")
    }

    /// C++: `double Triangle::getZ(const Position &) const` [const]
    /// returns the projection of the give geoCoordinate (WGS84) onto triangle plane
    // double
    // NBHeightMapper::Triangle::getZ(const Position& geo) const {
    //     // en.wikipedia.org/wiki/Line-plane_intersection
    //     Position p0 = myCorners.front();
    //     Position line(0, 0, 1);
    //     p0.sub(geo); // p0 - l0
    //     Position normal = normalVector();
    //     return p0.dotProduct(normal) / line.dotProduct(normal);
    // }
    #[allow(unused_variables)]
    pub fn get_z(&self, geo: &Position) -> f64 {
        todo!("port Triangle::getZ")
    }

    /// C++: `Position Triangle::normalVector() const` [const]
    /// returns the normal vector for this triangles plane
    // Position
    // NBHeightMapper::Triangle::normalVector() const {
    //     // @todo maybe cache result to avoid multiple computations?
    //     Position side1 = myCorners[1] - myCorners[0];
    //     Position side2 = myCorners[2] - myCorners[0];
    //     return side1.crossProduct(side2);
    // }
    #[allow(unused_variables)]
    pub fn normal_vector(&self) -> Position {
        todo!("port Triangle::normalVector")
    }

}

