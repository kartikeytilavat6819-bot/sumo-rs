//! Port of netbuild/NBHeightMapper.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::msg_handler::{progress_begin_message, with_instance, write_error, write_warning, MsgType};
use utils::common::std_defs::g_precision;
use utils::common::util_exceptions::ProcessError;
use utils::{write_errorf, write_warningf};
thread_local! {
    /// C++ `static NBHeightMapper NBHeightMapper::myInstance` (leaked once per thread, like a static)
    static MY_INSTANCE: Ptr<NBHeightMapper> = Ptr::from_raw(Box::into_raw(Box::new(NBHeightMapper::new())));
}

/// C++ typedef `std::vector<const Triangle *> NBHeightMapper::Triangles`
pub type NBHeightMapper_Triangles = Vec<Ptr<NBHeightMapper_Triangle>>;

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
        if oc.is_set("heightmap.geotiff", true).unwrap() {
            // parse file(s)
            let files = oc.get_string_vector("heightmap.geotiff").unwrap().clone();
            for file in files.iter() {
                progress_begin_message(&format!("Parsing from GeoTIFF '{}'", file));
                let num_features = MY_INSTANCE.with(|p| p.m().load_tiff(file));
                let boundary = MY_INSTANCE.with(|p| p.m().get_boundary().to_string_prec(g_precision()));
                with_instance(MsgType::Message, |h| h.end_process_msg(&format!(" done (parsed {} features, Boundary: {}).", num_features, boundary)));
            }
        }
        if oc.is_set("heightmap.shapefiles", true).unwrap() {
            // parse file(s)
            let files = oc.get_string_vector("heightmap.shapefiles").unwrap().clone();
            for file in files.iter() {
                progress_begin_message(&format!("Parsing from shape-file '{}'", file));
                let num_features = MY_INSTANCE.with(|p| p.m().load_shape_file(file));
                let boundary = MY_INSTANCE.with(|p| p.m().get_boundary().to_string_prec(g_precision()));
                with_instance(MsgType::Message, |h| h.end_process_msg(&format!(" done (parsed {} features, Boundary: {}).", num_features, boundary)));
            }
        }
    }

    /// C++: `const NBHeightMapper & NBHeightMapper::get()` [static]
    /// return the singleton instance (maybe 0)
    // const NBHeightMapper&
    // NBHeightMapper::get() {
    //     return myInstance;
    // }
    #[allow(unused_variables)]
    pub fn get() -> &'static NBHeightMapper {
        MY_INSTANCE.with(|p| p.get().expect("NBHeightMapper instance is never null"))
    }

    /// C++: `bool NBHeightMapper::ready() const` [const]
    /// returns whether the NBHeightMapper has data
    // bool
    // NBHeightMapper::ready() const {
    //     return myRasters.size() > 0 || myTriangles.size() > 0;
    // }
    #[allow(unused_variables)]
    pub fn ready(&self) -> bool {
        !self.myRasters.is_empty() || !self.myTriangles.is_empty()
    }

    /// C++: `const Boundary & NBHeightMapper::getBoundary()`
    /// returns the convex boundary of all known triangles
    // const Boundary& getBoundary() {
    //         return myInstance.myBoundary;
    //     }
    #[allow(unused_variables)]
    pub fn get_boundary(&mut self) -> &Boundary {
        &self.myBoundary
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
        if !self.ready() {
            write_warning("Cannot supply height since no height data was loaded");
            return 0.;
        }
        for item in self.myRasters.iter() {
            let boundary = &item.boundary;
            let raster = item.raster;
            let mut result = -1e6;
            // shortcut: the HAVE_GDAL coordinate transform is skipped; rasters only come from loadTiff, which needs GDAL (not ported)
            let x = geo.x();
            let y = geo.y();
            if boundary.around2d_xy(x, y) {
                let x_size = item.xSize;
                let norm_x = (x - boundary.xmin()) / self.mySizeOfPixel.x();
                let norm_y = (y - boundary.ymax()) / self.mySizeOfPixel.y();
                let at = |i: i32| unsafe { *raster.as_raw().offset(i as isize) };
                let ix = norm_x as i32;
                let iy = norm_y as i32;
                let mut corners = PositionVector::new();
                corners.0.push(Position::new3(norm_x.floor() + 0.5, norm_y.floor() + 0.5, at(iy * x_size + ix) as f64));
                if norm_x - norm_x.floor() > 0.5 {
                    corners.0.push(Position::new3(norm_x.floor() + 1.5, norm_y.floor() + 0.5, at(iy * x_size + ix + 1) as f64));
                } else {
                    corners.0.push(Position::new3(norm_x.floor() - 0.5, norm_y.floor() + 0.5, at(iy * x_size + ix - 1) as f64));
                }
                if norm_y - norm_y.floor() > 0.5 && (iy + 1) < item.ySize {
                    corners.0.push(Position::new3(norm_x.floor() + 0.5, norm_y.floor() + 1.5, at((iy + 1) * x_size + ix) as f64));
                } else {
                    corners.0.push(Position::new3(norm_x.floor() + 0.5, norm_y.floor() - 0.5, at((iy - 1) * x_size + ix) as f64));
                }
                result = NBHeightMapper_Triangle::new(&corners).get_z(&Position::new(norm_x, norm_y));
            }
            if result > -1e5 && result < 1e5 {
                return result;
            }
        }
        // coordinates in degrees hence a small search window
        // shortcut: C++ queries myRTree with a 1e-5 window; the RTree is not wired (triangles only come from GDAL shapefiles), so all triangles are scanned in insertion order
        for triangle in self.myTriangles.iter() {
            if triangle.contains(geo) {
                return triangle.get_z(geo);
            }
        }
        write_warningf!("Could not get height data for coordinate %", geo.to_string_prec(g_precision()));
        0.
    }

    /// C++: `void NBHeightMapper::NBHeightMapper()` [private]
    /// private constructor and destructor (Singleton)
    // NBHeightMapper::NBHeightMapper():
    //     myRTree(&Triangle::addSelf) {
    // }
    #[allow(unused_variables)]
    pub fn new() -> Self {
        NBHeightMapper {
            myTriangles: Vec::new(),
            myRTree: (), // shortcut: RTree not ported, see addTriangle
            myRasters: Vec::new(),
            mySizeOfPixel: Position::default(),
            myBoundary: Boundary::default(),
        }
    }

    /// C++: `void NBHeightMapper::~NBHeightMapper() noexcept` [private]
    // NBHeightMapper::~NBHeightMapper() {
    //     clearData();
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        self.clear_data();
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
        // shortcut: C++ also inserts the triangle's box into myRTree; the RTree is not wired because triangles only come from GDAL shapefiles (not ported), getZ scans myTriangles instead
        self.myTriangles.push(Ptr::from_raw(Box::into_raw(Box::new(NBHeightMapper_Triangle::new(&corners)))));
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
        // shortcut: GDAL shapefile loading is not ported; report like a build compiled without GDAL, upgrade when a shapefile reader lands
        write_error("Cannot load shape file since SUMO was compiled without GDAL support.");
        0
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
        // shortcut: GDAL GeoTIFF loading is not ported; report like a build compiled without GDAL, upgrade when a GeoTIFF reader lands
        write_error("Cannot load GeoTIFF file since SUMO was compiled without GDAL support.");
        0
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
        for t in self.myTriangles.drain(..) {
            drop(unsafe { Box::from_raw(t.as_raw()) });
        }
        // myRasters is only filled with GDAL, so there is nothing to free here
        self.myBoundary.reset();
    }

    /// C++: `void NBHeightMapper::NBHeightMapper(const NBHeightMapper &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &NBHeightMapper) -> Self {
        unreachable!("NBHeightMapper copy constructor is private and invalidated in C++")
    }

    /// C++: `NBHeightMapper & NBHeightMapper::operator=(const NBHeightMapper &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &NBHeightMapper) -> &mut NBHeightMapper {
        unreachable!("NBHeightMapper assignment operator is private and invalidated in C++")
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
        NBHeightMapper_QueryResult { triangles: Vec::new() }
    }

    /// C++: `void QueryResult::~QueryResult() noexcept`
    // ~QueryResult() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor is empty; the Vec owns no triangles
    }

    /// C++: `void QueryResult::add(Triangle *) const` [const]
    // void add(Triangle* triangle) const {
    //             triangles.push_back(triangle);
    //         }
    #[allow(unused_variables)]
    pub fn add(&mut self, triangle: Ptr<NBHeightMapper_Triangle>) {
        self.triangles.push(triangle);
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
        debug_assert_eq!(corners.len(), 3);
        NBHeightMapper_Triangle { myCorners: corners.clone() }
    }

    /// C++: `void Triangle::~Triangle() noexcept`
    // ~Triangle() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor is empty
    }

    /// C++: `void Triangle::addSelf(const QueryResult &) const` [const]
    /// callback for RTree search
    // void
    // NBHeightMapper::Triangle::addSelf(const QueryResult& queryResult) const {
    //     queryResult.triangles.push_back(this);
    // }
    #[allow(unused_variables)]
    pub fn add_self(&self, query_result: &mut NBHeightMapper_QueryResult) {
        query_result.add(Ptr::new(self));
    }

    /// C++: `bool Triangle::contains(const Position &) const` [const]
    /// checks whether pos lies within triangle (only checks x,y)
    // bool
    // NBHeightMapper::Triangle::contains(const Position& pos) const {
    //     return myCorners.around(pos);
    // }
    #[allow(unused_variables)]
    pub fn contains(&self, pos: &Position) -> bool {
        self.myCorners.around(pos, 0.)
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
        // en.wikipedia.org/wiki/Line-plane_intersection
        let mut p0 = *self.myCorners.front();
        let line = Position::new3(0., 0., 1.);
        p0.sub(geo); // p0 - l0
        let normal = self.normal_vector();
        p0.dot_product(&normal) / line.dot_product(&normal)
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
        // @todo maybe cache result to avoid multiple computations?
        let side1 = self.myCorners[1] - self.myCorners[0];
        let side2 = self.myCorners[2] - self.myCorners[0];
        side1.cross_product(&side2)
    }

}

