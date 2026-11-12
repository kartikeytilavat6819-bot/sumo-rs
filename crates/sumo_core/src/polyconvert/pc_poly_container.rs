//! Port of polyconvert/PCPolyContainer.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// LanePos
/// C++ class `PCPolyContainer::LanePos` (polyconvert/PCPolyContainer.h)
pub struct PCPolyContainer_LanePos {
    /// laneID
    pub laneID: String,
    /// pos over lane
    pub pos: f64,
    /// friendly position
    pub friendlyPos: bool,
    /// pos lateral over lane
    pub posLat: f64,
}

impl PCPolyContainer_LanePos {
    /// C++: `void LanePos::LanePos()`
    /// default constructor
    // PCPolyContainer::LanePos::LanePos() :
    //     pos(0),
    //     friendlyPos(false),
    //     posLat(0) {
    // }
    #[allow(unused_variables)]
    pub fn new() -> Self {
        todo!("port LanePos::LanePos")
    }

    /// C++: `void LanePos::LanePos(const std::string &, double, bool, double)`
    /// parameter constructor
    // PCPolyContainer::LanePos::LanePos(const std::string& _laneID, double _pos, bool _friendlyPos, double _posLat) :
    //     laneID(_laneID),
    //     pos(_pos),
    //     friendlyPos(_friendlyPos),
    //     posLat(_posLat) {
    // }
    #[allow(unused_variables)]
    pub fn new_2(laneID: &str, pos: f64, friendlyPos: bool, posLat: f64) -> Self {
        todo!("port LanePos::LanePos")
    }

}

/// A storage for loaded polygons and pois
/// C++ class `PCPolyContainer` (polyconvert/PCPolyContainer.h)
#[repr(C)]
pub struct PCPolyContainer {
    /// C++ base class `ShapeContainer`
    pub base: ShapeContainer,
    /// An id to pos map for lane pos specs
    pub myLanePosPois: BTreeMap<String, PCPolyContainer_LanePos>,
    /// An id to int map for proper enumeration
    pub myIDEnums: BTreeMap<String, i32>,
    /// The boundary that described the rectangle within which an object must be in order to be kept
    pub myPruningBoundary: Boundary,
    /// Information whether the pruning boundary shall be used
    pub myDoPrune: bool,
    /// List of names of polygons/pois that shall be removed
    pub myRemoveByNames: Vec<String>,
}

impl std::ops::Deref for PCPolyContainer {
    type Target = ShapeContainer;
    fn deref(&self) -> &ShapeContainer {
        &self.base
    }
}
impl std::ops::DerefMut for PCPolyContainer {
    fn deref_mut(&mut self) -> &mut ShapeContainer {
        &mut self.base
    }
}

impl PCPolyContainer {
    /// C++: `void PCPolyContainer::PCPolyContainer(bool, const Boundary &, const std::vector<std::string> &)`
    /// Constructor
    // PCPolyContainer::PCPolyContainer(bool prune,
    //                                  const Boundary& pruningBoundary,
    //                                  const std::vector<std::string>& removeByNames)
    //     : myPruningBoundary(pruningBoundary), myDoPrune(prune),
    //       myRemoveByNames(removeByNames) {}
    #[allow(unused_variables)]
    pub fn new(prune: bool, pruningBoundary: &Boundary, removeByNames: &Vec<String>) -> Self {
        todo!("port PCPolyContainer::PCPolyContainer")
    }

    /// C++: `void PCPolyContainer::~PCPolyContainer() noexcept` [virtual]
    /// Destructor
    // PCPolyContainer::~PCPolyContainer() {
    //     myPolygons.clear();
    //     myPOIs.clear();
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port PCPolyContainer::~PCPolyContainer")
    }

    /// C++: `bool PCPolyContainer::add(SUMOPolygon *, bool)` [virtual]
    /// Adds a polygon to the storage
    // bool
    // PCPolyContainer::add(SUMOPolygon* poly, bool ignorePruning) {
    //     // check whether the polygon lies within the wished area
    //     //  - if such an area was given
    //     if (myDoPrune && !ignorePruning) {
    //         Boundary b = poly->getShape().getBoxBoundary();
    //         if (!b.partialWithin(myPruningBoundary)) {
    //             delete poly;
    //             return false;
    //         }
    //     }
    //     // check whether the polygon was named to be a removed one
    //     if (find(myRemoveByNames.begin(), myRemoveByNames.end(), poly->getID()) != myRemoveByNames.end()) {
    //         delete poly;
    //         return false;
    //     }
    //     return ShapeContainer::add(poly);
    // }
    #[allow(unused_variables)]
    pub fn add(&mut self, poly: Ptr<SUMOPolygon>, ignorePruning: bool) -> bool {
        todo!("port PCPolyContainer::add")
    }

    /// C++: `bool PCPolyContainer::add(PointOfInterest *, bool)` [virtual]
    /// Adds a poi to the storage
    // bool
    // PCPolyContainer::add(PointOfInterest* poi, bool ignorePruning) {
    //     // check whether the poi lies within the wished area
    //     //  - if such an area was given
    //     if (myDoPrune && !ignorePruning) {
    //         if (!myPruningBoundary.around(*poi)) {
    //             delete poi;
    //             return false;
    //         }
    //     }
    //     // check whether the polygon was named to be a removed one
    //     if (find(myRemoveByNames.begin(), myRemoveByNames.end(), poi->getID()) != myRemoveByNames.end()) {
    //         delete poi;
    //         return false;
    //     }
    //     return ShapeContainer::add(poi);
    // }
    #[allow(unused_variables)]
    pub fn add_2(&mut self, poi: Ptr<PointOfInterest>, ignorePruning: bool) -> bool {
        todo!("port PCPolyContainer::add")
    }

    /// C++: `void PCPolyContainer::addLanePos(const std::string &, const std::string &, const double, const bool, const double)`
    /// add lane pos
    // void
    // PCPolyContainer::addLanePos(const std::string& poiID, const std::string& laneID, const double lanePos, const bool friendlyPos, const double lanePosLat) {
    //     myLanePosPois[poiID] = LanePos(laneID, lanePos, friendlyPos, lanePosLat);
    // }
    #[allow(unused_variables)]
    pub fn add_lane_pos(&mut self, poiID: &str, laneID: &str, lanePos: f64, friendlyPos: bool, lanePosLat: f64) {
        todo!("port PCPolyContainer::addLanePos")
    }

    /// C++: `void PCPolyContainer::save(const std::string &, bool)`
    /// Saves the stored polygons and pois into the given file
    // void
    // PCPolyContainer::save(const std::string& file, bool useGeo) {
    //     const GeoConvHelper& gch = GeoConvHelper::getFinal();
    //     if (useGeo && !gch.usingGeoProjection()) {
    //         WRITE_WARNING(TL("Ignoring option \"proj.plain-geo\" because no geo-conversion has been defined"));
    //         useGeo = false;
    //     }
    //     OutputDevice& out = OutputDevice::getDevice(file);
    //     out.writeXMLHeader("additional", "additional_file.xsd");
    //     if (useGeo) {
    //         out.setPrecision(gPrecisionGeo);
    //     } else if (gch.usingGeoProjection()) {
    //         GeoConvHelper::writeLocation(out);
    //     }
    //     // write polygons
    //     for (auto i : myPolygons) {
    //         i.second->writeXML(out, useGeo);
    //     }
    //     // write pois
    //     const double zOffset = OptionsCont::getOptions().getFloat("poi-layer-offset");
    //     for (const auto& POI : myPOIs) {
    //         std::map<std::string, LanePos>::const_iterator it = myLanePosPois.find(POI.first);
    //         if (it == myLanePosPois.end()) {
    //             POI.second->writeXML(out, useGeo, zOffset);
    //         } else {
    //             POI.second->writeXML(out, useGeo, zOffset, it->second.laneID, it->second.pos, it->second.friendlyPos, it->second.posLat);
    //         }
    //     }
    //     out.close();
    // }
    #[allow(unused_variables)]
    pub fn save(&mut self, file: &str, useGeo: bool) {
        todo!("port PCPolyContainer::save")
    }

    /// C++: `void PCPolyContainer::saveDlrTDP(const std::string &)`
    /// Saves the stored polygons and pois into the given file in dlrTDP format
    // void
    // PCPolyContainer::saveDlrTDP(const std::string& prefix) {
    //     const OptionsCont& oc = OptionsCont::getOptions();
    //     const GeoConvHelper& gch = GeoConvHelper::getFinal();
    //     const bool haveGeo = gch.usingGeoProjection();
    //     const double geoScale = pow(10.0f, haveGeo ? 5 : 2); // see NIImporter_DlrNavteq::GEO_SCALE
    //     // write pois
    //     OutputDevice& out = OutputDevice::getDevice(prefix + "_points_of_interest.txt");
    //     out.setPrecision(0);
    //     writeDlrTDPHeader(out, oc);
    //     // write format specifier
    //     out << "# ID\tCITY\tTYPE\tNAME\tgeo_x\tgeo_y\n";
    //     int id = 0;
    //     for (const auto& i : myPOIs) {
    //         Position pos(*i.second);
    //         gch.cartesian2geo(pos);
    //         pos.mul(geoScale);
    //         out << id << "\t";
    //         out << "" << "\t";
    //         out << i.second->getShapeType() << "\t";
    //         out << i.first << "\t";
    //         out << pos.x() << "\t";
    //         out << pos.y() << "\t";
    //         id++;
    //     }
    //     out.close();
    //     // write polygons
    //     OutputDevice& out2 = OutputDevice::getDevice(prefix + "_polygons.txt");
    //     out2.setPrecision(0);
    //     writeDlrTDPHeader(out2, oc);
    //     // write format specifier
    //     out2 << "# ID\tCITY\tTYPE\tNAME\tgeo_x1\tgeo_y1\t[geo_x2 geo_y2 ...]\n";
    //     id = 0;
    //     for (const auto& i : myPolygons) {
    //         out2 << id << "\t";
    //         out2 << "" << "\t";
    //         out2 << i.second->getShapeType() << "\t";
    //         out2 << i.first << "\t";
    //
    //         for (Position pos : i.second->getShape()) {
    //             gch.cartesian2geo(pos);
    //             pos.mul(geoScale);
    //             out2 << pos.x() << "\t";
    //             out2 << pos.y() << "\t";
    //         }
    //         id++;
    //     }
    //     out2.close();
    // }
    #[allow(unused_variables)]
    pub fn save_dlr_tdp(&mut self, prefix: &str) {
        todo!("port PCPolyContainer::saveDlrTDP")
    }

    /// C++: `int PCPolyContainer::getEnumIDFor(const std::string &)`
    /// Retuns a unique id for a given name
    // int
    // PCPolyContainer::getEnumIDFor(const std::string& key) {
    //     return myIDEnums[key]++;
    // }
    #[allow(unused_variables)]
    pub fn get_enum_id_for(&mut self, key: &str) -> i32 {
        todo!("port PCPolyContainer::getEnumIDFor")
    }

    /// C++: `void PCPolyContainer::writeDlrTDPHeader(OutputDevice &, const OptionsCont &)` [static, private]
    /// write DLR TDP Header
    // void PCPolyContainer::writeDlrTDPHeader(OutputDevice& device, const OptionsCont& oc) {
    //     // XXX duplicate of NWWriter_DlrNavteq::writeHeader()
    //     device << "# Format matches Extraction version: V6.5 \n";
    //     std::stringstream tmp;
    //     oc.writeConfiguration(tmp, true, false, false);
    //     tmp.seekg(std::ios_base::beg);
    //     std::string line;
    //     while (!tmp.eof()) {
    //         std::getline(tmp, line);
    //         device << "# " << line << "\n";
    //     }
    //     device << "#\n";
    // }
    #[allow(unused_variables)]
    pub fn write_dlr_tdp_header(device: &mut OutputDevice, oc: &OptionsCont) {
        todo!("port PCPolyContainer::writeDlrTDPHeader")
    }

    /// C++: `void PCPolyContainer::PCPolyContainer(const PCPolyContainer &)` [private]
    /// Invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(s: &PCPolyContainer) -> Self {
        todo!("port PCPolyContainer::PCPolyContainer")
    }

    /// C++: `PCPolyContainer & PCPolyContainer::operator=(const PCPolyContainer &)` [private]
    /// Invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &PCPolyContainer) -> &mut PCPolyContainer {
        todo!("port PCPolyContainer::operator=")
    }

}

