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
        PCPolyContainer_LanePos {
            laneID: String::new(),
            pos: 0.0,
            friendlyPos: false,
            posLat: 0.0,
        }
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
        PCPolyContainer_LanePos {
            laneID: laneID.to_string(),
            pos,
            friendlyPos,
            posLat,
        }
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
        PCPolyContainer {
            base: ShapeContainer::new(),
            myLanePosPois: BTreeMap::new(),
            myIDEnums: BTreeMap::new(),
            myPruningBoundary: *pruningBoundary,
            myDoPrune: prune,
            myRemoveByNames: removeByNames.clone(),
        }
    }

    /// C++: `void PCPolyContainer::~PCPolyContainer() noexcept` [virtual]
    /// Destructor
    // PCPolyContainer::~PCPolyContainer() {
    //     myPolygons.clear();
    //     myPOIs.clear();
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        self.base.myPolygons.clear();
        self.base.myPOIs.clear();
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
        // check whether the polygon lies within the wished area
        //  - if such an area was given
        if self.myDoPrune && !ignorePruning {
            let b = poly.get_shape().get_box_boundary();
            if !b.partial_within(&self.myPruningBoundary, 0.0) {
                // delete poly
                drop(unsafe { Box::from_raw(poly.as_raw()) });
                return false;
            }
        }
        // check whether the polygon was named to be a removed one
        if self.myRemoveByNames.iter().any(|n| n.as_str() == poly.get_id()) {
            // delete poly
            drop(unsafe { Box::from_raw(poly.as_raw()) });
            return false;
        }
        self.base.add(poly, false)
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
        // check whether the poi lies within the wished area
        //  - if such an area was given
        if self.myDoPrune && !ignorePruning {
            if !self.myPruningBoundary.around(&poi.base2, 0.0) {
                // delete poi
                drop(unsafe { Box::from_raw(poi.as_raw()) });
                return false;
            }
        }
        // check whether the polygon was named to be a removed one
        if self.myRemoveByNames.iter().any(|n| n.as_str() == poi.get_id()) {
            // delete poi
            drop(unsafe { Box::from_raw(poi.as_raw()) });
            return false;
        }
        self.base.add_2(poi, false)
    }

    /// C++: `void PCPolyContainer::addLanePos(const std::string &, const std::string &, const double, const bool, const double)`
    /// add lane pos
    // void
    // PCPolyContainer::addLanePos(const std::string& poiID, const std::string& laneID, const double lanePos, const bool friendlyPos, const double lanePosLat) {
    //     myLanePosPois[poiID] = LanePos(laneID, lanePos, friendlyPos, lanePosLat);
    // }
    #[allow(unused_variables)]
    pub fn add_lane_pos(&mut self, poiID: &str, laneID: &str, lanePos: f64, friendlyPos: bool, lanePosLat: f64) {
        self.myLanePosPois.insert(poiID.to_string(), PCPolyContainer_LanePos::new_2(laneID, lanePos, friendlyPos, lanePosLat));
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
        let gch = GeoConvHelper::with_final(|g| g.clone());
        let mut useGeo = useGeo;
        if useGeo && !gch.using_geo_projection() {
            write_warning("Ignoring option \"proj.plain-geo\" because no geo-conversion has been defined");
            useGeo = false;
        }
        let out: Ptr<OutputDevice> = Ptr::from_raw(OutputDevice::get_device(file, true).unwrap_or_else(|e| panic!("{e}")).as_ptr());
        out.m().write_xml_header("additional", "additional_file.xsd", BTreeMap::new(), true);
        if useGeo {
            out.m().set_precision(g_precision_geo());
        } else if gch.using_geo_projection() {
            write_location(out.m());
        }
        // write polygons
        for (_, i) in self.base.myPolygons.iter() {
            i.write_xml(out.m(), useGeo);
        }
        // write pois
        let zOffset: f64 = utils::options::options_cont::get_float("poi-layer-offset");
        for (poiID, poi) in self.base.myPOIs.iter() {
            match self.myLanePosPois.get(poiID) {
                None => poi.write_xml(out.m(), useGeo, zOffset, String::new(), 0.0, false, 0.0),
                Some(it) => poi.write_xml(out.m(), useGeo, zOffset, it.laneID.clone(), it.pos, it.friendlyPos, it.posLat),
            }
        }
        or_panic(out.m().close());
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
        let gch = GeoConvHelper::with_final(|g| g.clone());
        let haveGeo = gch.using_geo_projection();
        let geoScale: f64 = 10.0f64.powf(if haveGeo { 5.0 } else { 2.0 }); // see NIImporter_DlrNavteq::GEO_SCALE
        // write pois
        let out: Ptr<OutputDevice> = Ptr::from_raw(OutputDevice::get_device(&format!("{}_points_of_interest.txt", prefix), true).unwrap_or_else(|e| panic!("{e}")).as_ptr());
        out.m().set_precision(0);
        with_options(|oc| Self::write_dlr_tdp_header(out.m(), oc));
        // write format specifier
        out.m().write("# ID\tCITY\tTYPE\tNAME\tgeo_x\tgeo_y\n");
        let mut id: i32 = 0;
        for (name, poi) in self.base.myPOIs.iter() {
            let mut pos: Position = poi.base2;
            gch.cartesian2geo(&mut pos);
            pos.mul(geoScale);
            out.m().write(&id);
            out.m().write("\t");
            out.m().write("");
            out.m().write("\t");
            out.m().write(poi.get_shape_type());
            out.m().write("\t");
            out.m().write(name);
            out.m().write("\t");
            out.m().write(&pos.x());
            out.m().write("\t");
            out.m().write(&pos.y());
            out.m().write("\t");
            id += 1;
        }
        or_panic(out.m().close());
        // write polygons
        let out2: Ptr<OutputDevice> = Ptr::from_raw(OutputDevice::get_device(&format!("{}_polygons.txt", prefix), true).unwrap_or_else(|e| panic!("{e}")).as_ptr());
        out2.m().set_precision(0);
        with_options(|oc| Self::write_dlr_tdp_header(out2.m(), oc));
        // write format specifier
        out2.m().write("# ID\tCITY\tTYPE\tNAME\tgeo_x1\tgeo_y1\t[geo_x2 geo_y2 ...]\n");
        id = 0;
        for (name, poly) in self.base.myPolygons.iter() {
            out2.m().write(&id);
            out2.m().write("\t");
            out2.m().write("");
            out2.m().write("\t");
            out2.m().write(poly.get_shape_type());
            out2.m().write("\t");
            out2.m().write(name);
            out2.m().write("\t");

            for pos in poly.get_shape().iter() {
                let mut pos: Position = *pos;
                gch.cartesian2geo(&mut pos);
                pos.mul(geoScale);
                out2.m().write(&pos.x());
                out2.m().write("\t");
                out2.m().write(&pos.y());
                out2.m().write("\t");
            }
            id += 1;
        }
        or_panic(out2.m().close());
    }

    /// C++: `int PCPolyContainer::getEnumIDFor(const std::string &)`
    /// Retuns a unique id for a given name
    // int
    // PCPolyContainer::getEnumIDFor(const std::string& key) {
    //     return myIDEnums[key]++;
    // }
    #[allow(unused_variables)]
    pub fn get_enum_id_for(&mut self, key: &str) -> i32 {
        let e = self.myIDEnums.entry(key.to_string()).or_insert(0);
        let old = *e;
        *e += 1;
        old
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
        // XXX duplicate of NWWriter_DlrNavteq::writeHeader()
        device.write("# Format matches Extraction version: V6.5 \n");
        let mut tmp = String::new();
        or_panic(oc.write_configuration(&mut tmp, true, false, false, "", false, false, ""));
        // emulates `while (!tmp.eof()) { std::getline(tmp, line); ... }` on a string cursor
        let mut cur: usize = 0;
        let mut eof = false;
        while !eof {
            let line: &str;
            if cur >= tmp.len() {
                eof = true;
                line = "";
            } else {
                match tmp[cur..].find('\n') {
                    Some(i) => {
                        line = &tmp[cur..cur + i];
                        cur += i + 1;
                    }
                    None => {
                        line = &tmp[cur..];
                        cur = tmp.len();
                        eof = true;
                    }
                }
            }
            device.write("# ");
            device.write(line);
            device.write("\n");
        }
        device.write("#\n");
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

