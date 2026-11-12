//! Port of utils/shapes/SUMOPolygon.{h,cpp}
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use super::shape::{DEFAULT_ANGLE, DEFAULT_HEIGHT, DEFAULT_IMG_FILE, DEFAULT_LINEWIDTH};

/// A 2D- or 3D-polygon
/// C++ class `SUMOPolygon` (utils/shapes/SUMOPolygon.h)
pub struct SUMOPolygon {
    /// C++ base class `Shape`
    pub base: Shape,
    /// C++ base class `Parameterised`
    pub base2: Parameterised,
    /// The positions of the polygon
    pub myShape: PositionVector,
    /// The collection of the holes of the polygon, each given by a sequence of coodinates.
    pub myHoles: Vec<PositionVector>,
    /// specify if shape is handled as GEO coordinate (Main used in netedit)
    pub myGEO: bool,
    /// Information whether the polygon has to be filled
    pub myFill: bool,
    /// The line width for drawing an unfilled polygon
    pub myLineWidth: f64,
    /// The line width for drawing an unfilled polygon
    pub myHeight: f64,
}

impl std::ops::Deref for SUMOPolygon {
    type Target = Shape;
    fn deref(&self) -> &Shape {
        &self.base
    }
}
impl std::ops::DerefMut for SUMOPolygon {
    fn deref_mut(&mut self) -> &mut Shape {
        &mut self.base
    }
}

impl SUMOPolygon {
    /// C++: `void SUMOPolygon::SUMOPolygon(const std::string &, const std::string &, const RGBColor &, const PositionVector &, const bool, const bool, const double, const double, const double, const std::string &, const std::string &, const double, const Parameterised::Map &)`
    /// Constructor
    // SUMOPolygon::SUMOPolygon(const std::string& id, const std::string& type, const RGBColor& color,
    //                          const PositionVector& shape, const bool geo, const bool fill, const double lineWidth,
    //                          const double layer, const double angle, const std::string& imgFile, const std::string& name,
    //                          const double height, const Parameterised::Map& parameters) :
    //     Shape(id, type, color, layer, angle, imgFile, name),
    //     Parameterised(parameters),
    //     myShape(shape),
    //     myGEO(geo),
    //     myFill(fill),
    //     myLineWidth(lineWidth),
    //     myHeight(height) {
    // }
    #[allow(unused_variables)]
    pub fn new(id: &str, type_: &str, color: &RGBColor, shape: &PositionVector, geo: bool, fill: bool, lineWidth: f64, layer: f64, angle: f64, imgFile: &str, name: &str, height: f64, parameters: &BTreeMap<String, String>) -> Self {
        SUMOPolygon {
            base: Shape::new_2(id, type_, color, layer, angle, imgFile, name),
            base2: Parameterised::from_map(parameters),
            myShape: shape.clone(),
            myHoles: Vec::new(),
            myGEO: geo,
            myFill: fill,
            myLineWidth: lineWidth,
            myHeight: height,
        }
    }

    /// C++: `void SUMOPolygon::~SUMOPolygon() noexcept` [virtual]
    /// Destructor
    // SUMOPolygon::~SUMOPolygon() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
    }

    /// C++: `const PositionVector & SUMOPolygon::getShape() const` [const]
    /// Returns the shape of the polygon
    // const PositionVector&
    // SUMOPolygon::getShape() const {
    //     return myShape;
    // }
    #[allow(unused_variables)]
    pub fn get_shape(&self) -> &PositionVector {
        &self.myShape
    }

    /// C++: `PositionVector & SUMOPolygon::getShapeRef()`
    /// Return the exterior shape of the polygon.
    // PositionVector&
    // SUMOPolygon::getShapeRef() {
    //     return myShape;
    // }
    #[allow(unused_variables)]
    pub fn get_shape_ref(&mut self) -> &mut PositionVector {
        &mut self.myShape
    }

    /// C++: `const std::vector<PositionVector> & SUMOPolygon::getHoles() const` [const]
    /// Returns the holers of the polygon
    // const std::vector<PositionVector>&
    // SUMOPolygon::getHoles() const {
    //     return myHoles;
    // }
    #[allow(unused_variables)]
    pub fn get_holes(&self) -> &Vec<PositionVector> {
        &self.myHoles
    }

    /// C++: `bool SUMOPolygon::getFill() const` [const]
    /// Returns whether the polygon is filled
    // bool
    // SUMOPolygon::getFill() const {
    //     return myFill;
    // }
    #[allow(unused_variables)]
    pub fn get_fill(&self) -> bool {
        self.myFill
    }

    /// C++: `double SUMOPolygon::getLineWidth() const` [const]
    /// Returns the line width
    // double
    // SUMOPolygon::getLineWidth() const {
    //     return myLineWidth;
    // }
    #[allow(unused_variables)]
    pub fn get_line_width(&self) -> f64 {
        self.myLineWidth
    }

    /// C++: `double SUMOPolygon::getHeight() const` [const]
    /// Returns the polygon height
    // double
    // SUMOPolygon::getHeight() const {
    //     return myHeight;
    // }
    #[allow(unused_variables)]
    pub fn get_height(&self) -> f64 {
        self.myHeight
    }

    /// C++: `void SUMOPolygon::setFill(bool)`
    /// Sets whether the polygon shall be filled
    // void
    // SUMOPolygon::setFill(bool fill) {
    //     myFill = fill;
    // }
    #[allow(unused_variables)]
    pub fn set_fill(&mut self, fill: bool) {
        self.myFill = fill;
    }

    /// C++: `void SUMOPolygon::setLineWidth(double)`
    /// set line width
    // void
    // SUMOPolygon::setLineWidth(double lineWidth) {
    //     myLineWidth = lineWidth;
    // }
    #[allow(unused_variables)]
    pub fn set_line_width(&mut self, lineWidth: f64) {
        self.myLineWidth = lineWidth;
    }

    /// C++: `void SUMOPolygon::setHeight(double)`
    /// set height
    // void
    // SUMOPolygon::setHeight(double height) {
    //     myHeight = height;
    // }
    #[allow(unused_variables)]
    pub fn set_height(&mut self, height: f64) {
        self.myHeight = height;
    }

    /// C++: `void SUMOPolygon::setShape(const PositionVector &)` [virtual]
    /// Sets the shape of the polygon
    // void
    // SUMOPolygon::setShape(const PositionVector& shape) {
    //     myShape = shape;
    // }
    #[allow(unused_variables)]
    pub fn set_shape(&mut self, shape: &PositionVector) {
        self.myShape = shape.clone();
    }

    /// C++: `void SUMOPolygon::setHoles(const std::vector<PositionVector> &)` [virtual]
    /// Sets the holes of the polygon
    // void
    // SUMOPolygon::setHoles(const std::vector<PositionVector>& holes) {
    //     myHoles = holes;
    // }
    #[allow(unused_variables)]
    pub fn set_holes(&mut self, holes: &Vec<PositionVector>) {
        self.myHoles = holes.clone();
    }

    /// C++: `void SUMOPolygon::writeXML(OutputDevice &, bool) const` [const]
    // void
    // SUMOPolygon::writeXML(OutputDevice& out, bool geo) const {
    //     out.openTag(SUMO_TAG_POLY);
    //     out.writeAttr(SUMO_ATTR_ID, StringUtils::escapeXML(getID()));
    //     if (getShapeType().size() > 0) {
    //         out.writeAttr(SUMO_ATTR_TYPE, StringUtils::escapeXML(getShapeType()));
    //     }
    //     out.writeAttr(SUMO_ATTR_COLOR, getShapeColor());
    //     out.writeAttr(SUMO_ATTR_FILL,  getFill());
    //     if (getLineWidth() != Shape::DEFAULT_LINEWIDTH) {
    //         out.writeAttr(SUMO_ATTR_LINEWIDTH, getLineWidth());
    //     }
    //     if (getHeight() != Shape::DEFAULT_HEIGHT) {
    //         out.writeAttr(SUMO_ATTR_HEIGHT, getHeight());
    //     }
    //     out.writeAttr(SUMO_ATTR_LAYER, getShapeLayer());
    //     if (!getShapeName().empty()) {
    //         out.writeAttr(SUMO_ATTR_NAME, getShapeName());
    //     }
    //     PositionVector shape = getShape();
    //     if (geo) {
    //         out.writeAttr(SUMO_ATTR_GEO, true);
    //         for (int i = 0; i < (int) shape.size(); i++) {
    //             GeoConvHelper::getFinal().cartesian2geo(shape[i]);
    //         }
    //     }
    //     out.setPrecision(gPrecisionGeo);
    //     out.writeAttr(SUMO_ATTR_SHAPE, shape);
    //     out.setPrecision();
    //     if (getShapeNaviDegree() != Shape::DEFAULT_ANGLE) {
    //         out.writeAttr(SUMO_ATTR_ANGLE, getShapeNaviDegree());
    //     }
    //     if (getShapeImgFile() != Shape::DEFAULT_IMG_FILE) {
    //         out.writeAttr(SUMO_ATTR_IMGFILE, getShapeImgFile());
    //     }
    //     writeParams(out);
    //     out.closeTag();
    // }
    #[allow(unused_variables)]
    pub fn write_xml(&self, out: &mut OutputDevice, geo: bool) {
        out.open_tag_id(SUMO_TAG_POLY);
        out.write_attr(SUMO_ATTR_ID, &StringUtils::escape_xml(self.get_id()));
        if !self.get_shape_type().is_empty() {
            out.write_attr(SUMO_ATTR_TYPE, &StringUtils::escape_xml(self.get_shape_type()));
        }
        out.write_attr(SUMO_ATTR_COLOR, self.get_shape_color());
        out.write_attr(SUMO_ATTR_FILL, &self.get_fill());
        if self.get_line_width() != DEFAULT_LINEWIDTH {
            out.write_attr(SUMO_ATTR_LINEWIDTH, &self.get_line_width());
        }
        if self.get_height() != DEFAULT_HEIGHT {
            out.write_attr(SUMO_ATTR_HEIGHT, &self.get_height());
        }
        out.write_attr(SUMO_ATTR_LAYER, &self.get_shape_layer());
        if !self.get_shape_name().is_empty() {
            out.write_attr(SUMO_ATTR_NAME, self.get_shape_name());
        }
        let mut shape: PositionVector = self.get_shape().clone();
        if geo {
            out.write_attr(SUMO_ATTR_GEO, &true);
            for i in 0..shape.size() {
                GeoConvHelper::get_final().cartesian2geo(&mut shape[i]);
            }
        }
        out.set_precision(Some(gPrecisionGeo));
        out.write_attr(SUMO_ATTR_SHAPE, &shape);
        out.set_precision(None); // C++ setPrecision() without argument
        if self.get_shape_navi_degree() != DEFAULT_ANGLE {
            out.write_attr(SUMO_ATTR_ANGLE, &self.get_shape_navi_degree());
        }
        if self.get_shape_img_file() != DEFAULT_IMG_FILE {
            out.write_attr(SUMO_ATTR_IMGFILE, self.get_shape_img_file());
        }
        self.base2.write_params(out);
        out.close_tag("");
    }

}

