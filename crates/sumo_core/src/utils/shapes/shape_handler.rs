//! Port of utils/shapes/ShapeHandler.{h,cpp}
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::msg_handler::{write_error, write_warning};
use utils::common::to_string::to_string;
use super::shape::{gch_using_geo_projection, gch_x2cartesian, DEFAULT_ANGLE, DEFAULT_IMG_FILE, DEFAULT_IMG_HEIGHT, DEFAULT_IMG_WIDTH, DEFAULT_LAYER, DEFAULT_LAYER_POI, DEFAULT_LINEWIDTH, DEFAULT_NAME, DEFAULT_TYPE};

/// The XML-Handler for network loading
/// C++ class `ShapeHandler` (utils/shapes/ShapeHandler.h)
pub struct ShapeHandler {
    /// C++ base class `SUMOSAXHandler`
    pub base: Unmapped /* SUMOSAXHandler */, // shortcut: SUMOSAXHandler not ported yet, placeholder base
    /// reference to shape container in which all Shares are being added
    pub myShapeContainer: Ptr<ShapeContainer>,
    /// The prefix to use
    pub myPrefix: String,
    /// The default color to use
    pub myDefaultColor: RGBColor,
    /// The default icon to use
    pub myDefaultIcon: String,
    /// The default layer to use
    pub myDefaultLayer: f64,
    /// Information whether polygons should be filled
    pub myDefaultFill: bool,
    /// element to receive parameters
    pub myLastParameterised: Ptr<Parameterised>,
    /// geo-conversion to use during loading
    pub myGeoConvHelper: Ptr<GeoConvHelper>,
}

impl std::ops::Deref for ShapeHandler {
    type Target = Unmapped /* SUMOSAXHandler */;
    fn deref(&self) -> &Unmapped /* SUMOSAXHandler */ {
        &self.base
    }
}
impl std::ops::DerefMut for ShapeHandler {
    fn deref_mut(&mut self) -> &mut Unmapped /* SUMOSAXHandler */ {
        &mut self.base
    }
}

impl ShapeHandler {
    /// C++: `void ShapeHandler::ShapeHandler(const std::string &, ShapeContainer &, const GeoConvHelper *)`
    /// Constructor
    // ShapeHandler::ShapeHandler(const std::string& file, ShapeContainer& sc, const GeoConvHelper* geoConvHelper) :
    //     SUMOSAXHandler(file),
    //     myShapeContainer(sc),
    //     myPrefix(""),
    //     myDefaultColor(RGBColor::RED),
    //     myDefaultIcon(SUMOXMLDefinitions::POIIcons.getString(POIIcon::NONE)),
    //     myDefaultLayer(0),
    //     myDefaultFill(false),
    //     myLastParameterised(nullptr),
    //     myGeoConvHelper(geoConvHelper) {
    // }
    #[allow(unused_variables)]
    pub fn new(file: &str, sc: &mut ShapeContainer, geoConvHelper: Ptr<GeoConvHelper>) -> Self {
        ShapeHandler {
            base: (),
            myShapeContainer: Ptr::from_mut(sc),
            myPrefix: String::new(),
            myDefaultColor: RGBColor::RED,
            myDefaultIcon: POIIcons.get_string(POIIcon::NONE).to_string(),
            myDefaultLayer: 0.0,
            myDefaultFill: false,
            myLastParameterised: Ptr::null(),
            myGeoConvHelper: geoConvHelper,
        }
    }

    /// C++: `void ShapeHandler::~ShapeHandler()` [virtual]
    /// Destructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor: implicit, nothing to do
    }

    /// C++: `bool ShapeHandler::loadFiles(const int &, ShapeHandler &)` [static]
    /// loads all of the given files
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn load_files(files: &Vec<String>, sh: &mut ShapeHandler) -> bool {
        for fileIt in files.iter() {
            // shortcut: XMLSubSys::runParser needs ShapeHandler as a SaxHandler (SUMOSAXHandler base is unported), upgrade when wired
            let parsed: bool = panic!("ShapeHandler::loadFiles: XMLSubSys::runParser not wired");
            if !parsed {
                write_messagef!("Loading of shapes from % failed.", fileIt);
                return false;
            }
        }
        true
    }

    /// C++: `void ShapeHandler::myStartElement(int, const SUMOSAXAttributes &)` [virtual, protected]
    /// Called on the opening of a tag;
    // void
    // ShapeHandler::myStartElement(int element, const SUMOSAXAttributes& attrs) {
    //     try {
    //         switch (element) {
    //             case SUMO_TAG_POLY:
    //                 // default layer is different depending if we're parsing a Poly or a POI, therefore it has to be here defined
    //                 myDefaultLayer = Shape::DEFAULT_LAYER;
    //                 addPoly(attrs, false, false);
    //                 break;
    //             case SUMO_TAG_POI:
    //                 // default layer is different depending if we're parsing a Poly or a POI, therefore it has to be here defined
    //                 myDefaultLayer = Shape::DEFAULT_LAYER_POI;
    //                 addPOI(attrs, false, false);
    //                 break;
    //             case SUMO_TAG_PARAM:
    //                 if (myLastParameterised != nullptr) {
    //                     bool ok = true;
    //                     const std::string key = attrs.get<std::string>(SUMO_ATTR_KEY, nullptr, ok);
    //                     // continue if key was successfully loaded
    //                     if (ok) {
    //                         // circumventing empty string value
    //                         const std::string val = attrs.hasAttribute(SUMO_ATTR_VALUE) ? attrs.getString(SUMO_ATTR_VALUE) : "";
    //                         // show warnings if values are invalid
    //                         if (key.empty()) {
    //                             WRITE_WARNING(TL("Error parsing key from shape generic parameter. Key cannot be empty"));
    //                         } else if (!SUMOXMLDefinitions::isValidParameterKey(key)) {
    //                             WRITE_WARNING(TL("Error parsing key from shape generic parameter. Key contains invalid characters"));
    //                         } else {
    //                             myLastParameterised->setParameter(key, val);
    //                         }
    //                     }
    //                 }
    //                 break;
    //             default:
    //                 break;
    //         }
    //     } catch (InvalidArgument& e) {
    //         WRITE_ERROR(e.what());
    //     }
    // }
    #[allow(unused_variables)]
    pub fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) {
        match element {
            SUMO_TAG_POLY => {
                // default layer is different depending if we're parsing a Poly or a POI, therefore it has to be here defined
                self.myDefaultLayer = DEFAULT_LAYER;
                self.add_poly(attrs, false, false);
            }
            SUMO_TAG_POI => {
                // default layer is different depending if we're parsing a Poly or a POI, therefore it has to be here defined
                self.myDefaultLayer = DEFAULT_LAYER_POI;
                self.add_poi(attrs, false, false);
            }
            SUMO_TAG_PARAM => {
                if !self.myLastParameterised.is_null() {
                    let mut ok = true;
                    let key: String = attrs.get::<String>(SUMO_ATTR_KEY, "", &mut ok, true);
                    // continue if key was successfully loaded
                    if ok {
                        // circumventing empty string value
                        let val: String = if attrs.has_attribute(SUMO_ATTR_VALUE) { attrs.get_string(SUMO_ATTR_VALUE) } else { String::new() };
                        // show warnings if values are invalid
                        if key.is_empty() {
                            write_warning("Error parsing key from shape generic parameter. Key cannot be empty");
                        } else if !utils::xml::sumo_xml_definitions::is_valid_parameter_key(&key) {
                            write_warning("Error parsing key from shape generic parameter. Key contains invalid characters");
                        } else {
                            self.myLastParameterised.m().set_parameter(&key, &val);
                        }
                    }
                }
            }
            _ => {}
        }
        // shortcut: C++ catches InvalidArgument from attrs and calls WRITE_ERROR(e.what()); SUMOSAXAttributes does not return Result yet
    }

    /// C++: `void ShapeHandler::myEndElement(int)` [virtual, protected]
    /// Called when a closing tag occurs
    // void
    // ShapeHandler::myEndElement(int element) {
    //     if (element != SUMO_TAG_PARAM) {
    //         myLastParameterised = nullptr;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn my_end_element(&mut self, element: i32) {
        if element != SUMO_TAG_PARAM {
            self.myLastParameterised = Ptr::null();
        }
    }

    /// C++: `Position ShapeHandler::getLanePos(const std::string &, const std::string &, double, bool, double)` [virtual, pure virtual, protected]
    /// get position for a given laneID (Has to be implemented in all child)
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_lane_pos(&mut self, poiID: &str, laneID: &str, lanePos: f64, friendlyPos: bool, lanePosLat: f64) -> Position {
        panic!("ShapeHandler::getLanePos is pure virtual in C++; subclasses must override it")
    }

    /// C++: `bool ShapeHandler::addLanePosParams()` [virtual, protected]
    /// Whether some input attributes shall be automatically added as params (Can be implemented in all child)
    // bool
    // ShapeHandler::addLanePosParams() {
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn add_lane_pos_params(&mut self) -> bool {
        false
    }

    /// C++: `void ShapeHandler::setDefaults(const std::string &, const RGBColor &, const std::string &, const double, const bool)` [protected]
    /// set default values
    // void
    // ShapeHandler::setDefaults(const std::string& prefix, const RGBColor& color, const std::string& icon, const double layer, const bool fill) {
    //     myPrefix = prefix;
    //     myDefaultColor = color;
    //     myDefaultIcon = icon;
    //     myDefaultLayer = layer;
    //     myDefaultFill = fill;
    // }
    #[allow(unused_variables)]
    pub fn set_defaults(&mut self, prefix: &str, color: &RGBColor, icon: &str, layer: f64, fill: bool) {
        self.myPrefix = prefix.to_string();
        self.myDefaultColor = color.clone();
        self.myDefaultIcon = icon.to_string();
        self.myDefaultLayer = layer;
        self.myDefaultFill = fill;
    }

    /// C++: `void ShapeHandler::addPOI(const SUMOSAXAttributes &, const bool, const bool)` [protected]
    /// adds a POI
    // void
    // ShapeHandler::addPOI(const SUMOSAXAttributes& attrs, const bool ignorePruning, const bool useProcessing) {
    //     bool ok = true;
    //     const double INVALID_POSITION(-1000000);
    //     const std::string id = myPrefix + attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
    //     double x = attrs.getOpt<double>(SUMO_ATTR_X, id.c_str(), ok, INVALID_POSITION);
    //     const double y = attrs.getOpt<double>(SUMO_ATTR_Y, id.c_str(), ok, INVALID_POSITION);
    //     const double z = attrs.getOpt<double>(SUMO_ATTR_Z, id.c_str(), ok, INVALID_POSITION);
    //     double lon = attrs.getOpt<double>(SUMO_ATTR_LON, id.c_str(), ok, INVALID_POSITION);
    //     double lat = attrs.getOpt<double>(SUMO_ATTR_LAT, id.c_str(), ok, INVALID_POSITION);
    //     const double lanePos = attrs.getOpt<double>(SUMO_ATTR_POSITION, id.c_str(), ok, 0);
    //     const bool friendlyPos = attrs.getOpt<bool>(SUMO_ATTR_FRIENDLY_POS, id.c_str(), ok, false);
    //     const double lanePosLat = attrs.getOpt<double>(SUMO_ATTR_POSITION_LAT, id.c_str(), ok, 0);
    //     std::string icon = attrs.getOpt<std::string>(SUMO_ATTR_ICON, id.c_str(), ok, myDefaultIcon);
    //     // check icon
    //     if (!SUMOXMLDefinitions::POIIcons.hasString(icon)) {
    //         WRITE_WARNING(TLF("Invalid icon % for POI '%', using default", icon, id));
    //         icon = "none";
    //     }
    //     const double layer = attrs.getOpt<double>(SUMO_ATTR_LAYER, id.c_str(), ok, myDefaultLayer);
    //     const std::string type = attrs.getOpt<std::string>(SUMO_ATTR_TYPE, id.c_str(), ok, "");
    //     const std::string laneID = attrs.getOpt<std::string>(SUMO_ATTR_LANE, id.c_str(), ok, "");
    //     const double angle = attrs.getOpt<double>(SUMO_ATTR_ANGLE, id.c_str(), ok, Shape::DEFAULT_ANGLE);
    //     std::string imgFile = attrs.getOpt<std::string>(SUMO_ATTR_IMGFILE, id.c_str(), ok, Shape::DEFAULT_IMG_FILE);
    //     const RGBColor color = attrs.hasAttribute(SUMO_ATTR_COLOR) ? attrs.get<RGBColor>(SUMO_ATTR_COLOR, id.c_str(), ok) : (imgFile != "" ? RGBColor::WHITE : myDefaultColor);
    //     // If the image file is set, change the default POI color to white.
    //     if (imgFile != "" && !FileHelpers::isAbsolute(imgFile)) {
    //         imgFile = FileHelpers::getConfigurationRelative(getFileName(), imgFile);
    //     }
    //     const double width = attrs.getOpt<double>(SUMO_ATTR_WIDTH, id.c_str(), ok, Shape::DEFAULT_IMG_WIDTH);
    //     const double height = attrs.getOpt<double>(SUMO_ATTR_HEIGHT, id.c_str(), ok, Shape::DEFAULT_IMG_HEIGHT);
    //     // check if ID is valid
    //     if (!SUMOXMLDefinitions::isValidTypeID(id)) {
    //         WRITE_WARNINGF(TL("Invalid characters for PoI ID '%'"), id);
    //         ok = false;
    //     }
    //     // continue
    //     if (ok) {
    //         const GeoConvHelper* gch;
    //         // set GEOConverter
    //         if (myGeoConvHelper != nullptr) {
    //             gch = myGeoConvHelper;
    //         } else if (useProcessing) {
    //             gch = &GeoConvHelper::getProcessing();
    //         } else {
    //             gch = &GeoConvHelper::getFinal();
    //         }
    //         // check if GEOProjection has to be used
    //         if (useProcessing && gch->usingGeoProjection()) {
    //             if ((lat == INVALID_POSITION) || (lon == INVALID_POSITION)) {
    //                 lon = x;
    //                 lat = y;
    //                 x = INVALID_POSITION;
    //             }
    //         }
    //         Position pos(x, y);
    //         bool useGeo = false;
    //         if ((x == INVALID_POSITION) || (y == INVALID_POSITION)) {
    //             // try computing x,y from lane,pos
    //             if (laneID != "") {
    //                 pos = getLanePos(id, laneID, lanePos, friendlyPos, lanePosLat);
    //             } else {
    //                 // try computing x,y from lon,lat
    //                 if ((lat == INVALID_POSITION) || (lon == INVALID_POSITION)) {
    //                     WRITE_ERRORF(TL("Either (x, y), (lon, lat) or (lane, pos) must be specified for PoI '%'."), id);
    //                     return;
    //                 } else if (!gch->usingGeoProjection()) {
    //                     WRITE_ERRORF(TL("(lon, lat) is specified for PoI '%' but no geo-conversion is specified for the network."), id);
    //                     return;
    //                 }
    //                 pos.set(lon, lat);
    //                 useGeo = true;
    //                 bool success = true;
    //                 if (useProcessing) {
    //                     success = GeoConvHelper::getProcessing().x2cartesian(pos);
    //                 } else {
    //                     success = gch->x2cartesian_const(pos);
    //                 }
    //                 if (!success) {
    //                     WRITE_ERRORF(TL("Unable to project coordinates for POI '%'."), id);
    //                     return;
    //                 }
    //             }
    //         }
    //         if (z != INVALID_POSITION) {
    //             pos.setz(z);
    //         }
    //         if (!myShapeContainer.addPOI(id, type, color, pos, useGeo, laneID, lanePos, friendlyPos, lanePosLat, icon,
    //                                      layer, angle, imgFile, width, height, ignorePruning)) {
    //             WRITE_ERRORF(TL("PoI '%' already exists."), id);
    //         }
    //         myLastParameterised = myShapeContainer.getPOIs().get(id);
    //         if ((laneID != "") && addLanePosParams()) {
    //             myLastParameterised->setParameter(toString(SUMO_ATTR_LANE), laneID);
    //             myLastParameterised->setParameter(toString(SUMO_ATTR_POSITION), toString(lanePos));
    //             myLastParameterised->setParameter(toString(SUMO_ATTR_POSITION_LAT), toString(lanePosLat));
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_poi(&mut self, attrs: &SUMOSAXAttributes, ignorePruning: bool, useProcessing: bool) {
        let mut ok = true;
        let INVALID_POSITION: f64 = -1000000.0;
        let id: String = self.myPrefix.clone() + &attrs.get::<String>(SUMO_ATTR_ID, "", &mut ok, true);
        let mut x: f64 = attrs.get_opt::<f64>(SUMO_ATTR_X, id.as_str(), &mut ok, INVALID_POSITION, true);
        let y: f64 = attrs.get_opt::<f64>(SUMO_ATTR_Y, id.as_str(), &mut ok, INVALID_POSITION, true);
        let z: f64 = attrs.get_opt::<f64>(SUMO_ATTR_Z, id.as_str(), &mut ok, INVALID_POSITION, true);
        let mut lon: f64 = attrs.get_opt::<f64>(SUMO_ATTR_LON, id.as_str(), &mut ok, INVALID_POSITION, true);
        let mut lat: f64 = attrs.get_opt::<f64>(SUMO_ATTR_LAT, id.as_str(), &mut ok, INVALID_POSITION, true);
        let lanePos: f64 = attrs.get_opt::<f64>(SUMO_ATTR_POSITION, id.as_str(), &mut ok, 0.0, true);
        let friendlyPos: bool = attrs.get_opt::<bool>(SUMO_ATTR_FRIENDLY_POS, id.as_str(), &mut ok, false, true);
        let lanePosLat: f64 = attrs.get_opt::<f64>(SUMO_ATTR_POSITION_LAT, id.as_str(), &mut ok, 0.0, true);
        let mut icon: String = attrs.get_opt::<String>(SUMO_ATTR_ICON, id.as_str(), &mut ok, self.myDefaultIcon.clone(), true);
        // check icon
        if !POIIcons.has_string(&icon) {
            write_warningf!("Invalid icon % for POI '%', using default", icon, id);
            icon = "none".to_string();
        }
        let layer: f64 = attrs.get_opt::<f64>(SUMO_ATTR_LAYER, id.as_str(), &mut ok, self.myDefaultLayer, true);
        let type_: String = attrs.get_opt::<String>(SUMO_ATTR_TYPE, id.as_str(), &mut ok, String::new(), true);
        let laneID: String = attrs.get_opt::<String>(SUMO_ATTR_LANE, id.as_str(), &mut ok, String::new(), true);
        let angle: f64 = attrs.get_opt::<f64>(SUMO_ATTR_ANGLE, id.as_str(), &mut ok, DEFAULT_ANGLE, true);
        let mut imgFile: String = attrs.get_opt::<String>(SUMO_ATTR_IMGFILE, id.as_str(), &mut ok, DEFAULT_IMG_FILE.to_string(), true);
        let color: RGBColor = if attrs.has_attribute(SUMO_ATTR_COLOR) {
            attrs.get::<RGBColor>(SUMO_ATTR_COLOR, id.as_str(), &mut ok, true)
        } else if imgFile != "" {
            RGBColor::WHITE
        } else {
            self.myDefaultColor.clone()
        };
        // If the image file is set, change the default POI color to white.
        if imgFile != "" && !utils::common::file_helpers::is_absolute(&imgFile) {
            imgFile = utils::common::file_helpers::get_configuration_relative({ panic!("shortcut: ShapeHandler base SUMOSAXHandler is unported, file name unavailable; upgrade when SAX base is wired") }, &imgFile);
        }
        let width: f64 = attrs.get_opt::<f64>(SUMO_ATTR_WIDTH, id.as_str(), &mut ok, DEFAULT_IMG_WIDTH, true);
        let height: f64 = attrs.get_opt::<f64>(SUMO_ATTR_HEIGHT, id.as_str(), &mut ok, DEFAULT_IMG_HEIGHT, true);
        // check if ID is valid
        if !utils::xml::sumo_xml_definitions::is_valid_type_id(&id) {
            write_warningf!("Invalid characters for PoI ID '%'", id);
            ok = false;
        }
        // continue
        if ok {
            // set GEOConverter
            let gch: Option<Ptr<GeoConvHelper>> = if !self.myGeoConvHelper.is_null() { Some(self.myGeoConvHelper.clone()) } else { None };
            // check if GEOProjection has to be used
            if useProcessing && gch_using_geo_projection(gch, useProcessing) {
                if (lat == INVALID_POSITION) || (lon == INVALID_POSITION) {
                    lon = x;
                    lat = y;
                    x = INVALID_POSITION;
                }
            }
            let mut pos: Position = Position::new(x, y);
            let mut useGeo = false;
            if (x == INVALID_POSITION) || (y == INVALID_POSITION) {
                // try computing x,y from lane,pos
                if laneID != "" {
                    pos = self.get_lane_pos(&id, &laneID, lanePos, friendlyPos, lanePosLat);
                } else {
                    // try computing x,y from lon,lat
                    if (lat == INVALID_POSITION) || (lon == INVALID_POSITION) {
                        write_errorf!("Either (x, y), (lon, lat) or (lane, pos) must be specified for PoI '%'.", id);
                        return;
                    } else if !gch_using_geo_projection(gch, useProcessing) {
                        write_errorf!("(lon, lat) is specified for PoI '%' but no geo-conversion is specified for the network.", id);
                        return;
                    }
                    pos.set(lon, lat);
                    useGeo = true;
                    let success: bool = gch_x2cartesian(gch, useProcessing, &mut pos);
                    if !success {
                        write_errorf!("Unable to project coordinates for POI '%'.", id);
                        return;
                    }
                }
            }
            if z != INVALID_POSITION {
                pos.setz(z);
            }
            if !self.myShapeContainer.m().add_poi(&id, &type_, &color, &pos, useGeo, &laneID, lanePos, friendlyPos, lanePosLat, &icon,
                                                 layer, angle, &imgFile, width, height, ignorePruning) {
                write_errorf!("PoI '%' already exists.", id);
            }
            let poi: Option<Ptr<PointOfInterest>> = self.myShapeContainer.get_po_is().get(&id).cloned();
            self.myLastParameterised = match poi {
                Some(p) => Ptr::from_mut(&mut p.m().base3),
                None => Ptr::null(),
            };
            if (laneID != "") && self.add_lane_pos_params() {
                let lp = self.myLastParameterised.m();
                lp.set_parameter(utils::xml::sumo_xml_definitions::Attrs.get_string(SUMO_ATTR_LANE), &laneID);
                lp.set_parameter(utils::xml::sumo_xml_definitions::Attrs.get_string(SUMO_ATTR_POSITION), &to_string(&lanePos));
                lp.set_parameter(utils::xml::sumo_xml_definitions::Attrs.get_string(SUMO_ATTR_POSITION_LAT), &to_string(&lanePosLat));
            }
        }
    }

    /// C++: `void ShapeHandler::addPoly(const SUMOSAXAttributes &, const bool, const bool)` [protected]
    /// adds a polygon
    // void
    // ShapeHandler::addPoly(const SUMOSAXAttributes& attrs, const bool ignorePruning, const bool useProcessing) {
    //     bool ok = true;
    //     const std::string id = myPrefix + attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
    //     // check if ID is valid
    //     if (!SUMOXMLDefinitions::isValidTypeID(id)) {
    //         WRITE_WARNINGF(TL("Invalid characters for Poly ID '%'"), id);
    //         ok = false;
    //     }
    //     // get the id, report an error if not given or empty...
    //     if (ok) {
    //         // continue loading parameters
    //         const double layer = attrs.getOpt<double>(SUMO_ATTR_LAYER, id.c_str(), ok, myDefaultLayer);
    //         const bool fill = attrs.getOpt<bool>(SUMO_ATTR_FILL, id.c_str(), ok, myDefaultFill);
    //         const double lineWidth = attrs.getOpt<double>(SUMO_ATTR_LINEWIDTH, id.c_str(), ok, Shape::DEFAULT_LINEWIDTH);
    //         const std::string type = attrs.getOpt<std::string>(SUMO_ATTR_TYPE, id.c_str(), ok, Shape::DEFAULT_TYPE);
    //         const RGBColor color = attrs.hasAttribute(SUMO_ATTR_COLOR) ? attrs.get<RGBColor>(SUMO_ATTR_COLOR, id.c_str(), ok) : myDefaultColor;
    //         PositionVector shape = attrs.get<PositionVector>(SUMO_ATTR_SHAPE, id.c_str(), ok);
    //         const bool geo = attrs.getOpt<bool>(SUMO_ATTR_GEO, id.c_str(), ok, false);
    //         // set geo converter
    //         const GeoConvHelper* gch;
    //         if (myGeoConvHelper != nullptr) {
    //             gch = myGeoConvHelper;
    //         } else {
    //             gch = &GeoConvHelper::getFinal();
    //         }
    //         // check if poly use geo coordinates
    //         if (geo || useProcessing) {
    //             bool success = true;
    //             for (int i = 0; i < (int)shape.len(); i++) {
    //                 if (useProcessing) {
    //                     success &= GeoConvHelper::getProcessing().x2cartesian(shape[i]);
    //                 } else {
    //                     success &= gch->x2cartesian_const(shape[i]);
    //                 }
    //             }
    //             if (!success) {
    //                 WRITE_WARNINGF(TL("Unable to project coordinates for polygon '%'."), id);
    //                 return;
    //             }
    //         }
    //         const double angle = attrs.getOpt<double>(SUMO_ATTR_ANGLE, id.c_str(), ok, Shape::DEFAULT_ANGLE);
    //         std::string imgFile = attrs.getOpt<std::string>(SUMO_ATTR_IMGFILE, id.c_str(), ok, Shape::DEFAULT_IMG_FILE);
    //         if (imgFile != "" && !FileHelpers::isAbsolute(imgFile)) {
    //             imgFile = FileHelpers::getConfigurationRelative(getFileName(), imgFile);
    //         }
    //         // check that shape's size is valid
    //         if (shape.len() == 0) {
    //             WRITE_ERROR(TL("Polygon's shape cannot be empty."));
    //             return;
    //         }
    //         // check that lineWidth is positive
    //         if (lineWidth <= 0) {
    //             WRITE_ERROR(TL("Polygon's lineWidth must be greater than 0."));
    //             return;
    //         }
    //         // create polygon, or show an error if polygon already exists
    //         if (!myShapeContainer.addPolygon(id, type, color, layer, angle, imgFile, shape, geo, fill, lineWidth, ignorePruning)) {
    //             WRITE_ERRORF(TL("Polygon '%' already exists."), id);
    //         }
    //         myLastParameterised = myShapeContainer.getPolygons().get(id);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_poly(&mut self, attrs: &SUMOSAXAttributes, ignorePruning: bool, useProcessing: bool) {
        let mut ok = true;
        let id: String = self.myPrefix.clone() + &attrs.get::<String>(SUMO_ATTR_ID, "", &mut ok, true);
        // check if ID is valid
        if !utils::xml::sumo_xml_definitions::is_valid_type_id(&id) {
            write_warningf!("Invalid characters for Poly ID '%'", id);
            ok = false;
        }
        // get the id, report an error if not given or empty...
        if ok {
            // continue loading parameters
            let layer: f64 = attrs.get_opt::<f64>(SUMO_ATTR_LAYER, id.as_str(), &mut ok, self.myDefaultLayer, true);
            let fill: bool = attrs.get_opt::<bool>(SUMO_ATTR_FILL, id.as_str(), &mut ok, self.myDefaultFill, true);
            let lineWidth: f64 = attrs.get_opt::<f64>(SUMO_ATTR_LINEWIDTH, id.as_str(), &mut ok, DEFAULT_LINEWIDTH, true);
            let type_: String = attrs.get_opt::<String>(SUMO_ATTR_TYPE, id.as_str(), &mut ok, DEFAULT_TYPE.to_string(), true);
            let color: RGBColor = if attrs.has_attribute(SUMO_ATTR_COLOR) {
                attrs.get::<RGBColor>(SUMO_ATTR_COLOR, id.as_str(), &mut ok, true)
            } else {
                self.myDefaultColor.clone()
            };
            let mut shape: PositionVector = attrs.get::<PositionVector>(SUMO_ATTR_SHAPE, id.as_str(), &mut ok, true);
            let geo: bool = attrs.get_opt::<bool>(SUMO_ATTR_GEO, id.as_str(), &mut ok, false, true);
            // set geo converter
            let gch: Option<Ptr<GeoConvHelper>> = if !self.myGeoConvHelper.is_null() { Some(self.myGeoConvHelper.clone()) } else { None };
            // check if poly use geo coordinates
            if geo || useProcessing {
                let mut success = true;
                for i in 0..shape.len() {
                    success &= gch_x2cartesian(gch, useProcessing, &mut shape[i]);
                }
                if !success {
                    write_warningf!("Unable to project coordinates for polygon '%'.", id);
                    return;
                }
            }
            let angle: f64 = attrs.get_opt::<f64>(SUMO_ATTR_ANGLE, id.as_str(), &mut ok, DEFAULT_ANGLE, true);
            let mut imgFile: String = attrs.get_opt::<String>(SUMO_ATTR_IMGFILE, id.as_str(), &mut ok, DEFAULT_IMG_FILE.to_string(), true);
            if imgFile != "" && !utils::common::file_helpers::is_absolute(&imgFile) {
                imgFile = utils::common::file_helpers::get_configuration_relative({ panic!("shortcut: ShapeHandler base SUMOSAXHandler is unported, file name unavailable; upgrade when SAX base is wired") }, &imgFile);
            }
            // check that shape's size is valid
            if shape.len() == 0 {
                write_error("Polygon's shape cannot be empty.");
                return;
            }
            // check that lineWidth is positive
            if lineWidth <= 0.0 {
                write_error("Polygon's lineWidth must be greater than 0.");
                return;
            }
            // create polygon, or show an error if polygon already exists
            if !self.myShapeContainer.m().add_polygon(&id, &type_, &color, layer, angle, &imgFile, &shape, geo, fill, lineWidth, ignorePruning, DEFAULT_NAME) {
                write_errorf!("Polygon '%' already exists.", id);
            }
            let poly: Option<Ptr<SUMOPolygon>> = self.myShapeContainer.get_polygons().get(&id).cloned();
            self.myLastParameterised = match poly {
                Some(p) => Ptr::from_mut(&mut p.m().base2),
                None => Ptr::null(),
            };
        }
    }

    /// C++: `Parameterised * ShapeHandler::getLastParameterised() const` [const, protected]
    /// get last parameterised object
    // Parameterised*
    // ShapeHandler::getLastParameterised() const {
    //     return myLastParameterised;
    // }
    #[allow(unused_variables)]
    pub fn get_last_parameterised(&self) -> Ptr<Parameterised> {
        self.myLastParameterised.clone()
    }

    /// C++: `void ShapeHandler::ShapeHandler(const ShapeHandler &)` [protected]
    /// invalidate copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(s: &ShapeHandler) -> Self {
        // C++ copy constructor is deleted
        panic!("ShapeHandler copy constructor is deleted in C++")
    }

    /// C++: `ShapeHandler & ShapeHandler::operator=(const ShapeHandler &)` [protected]
    /// invalidate assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &ShapeHandler) -> &mut ShapeHandler {
        // C++ operator= is deleted
        panic!("ShapeHandler::operator= is deleted in C++")
    }

}

