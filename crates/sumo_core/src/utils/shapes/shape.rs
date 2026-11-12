//! Port of utils/shapes/Shape.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// A 2D- or 3D-Shape
/// C++ class `Shape` (utils/shapes/Shape.h)
pub struct Shape {
    /// C++ base class `Named`
    pub base: Named,
    /// The type of the Shape
    pub myType: String,
    /// The color of the Shape
    pub myColor: RGBColor,
    /// The layer of the Shape
    pub myLayer: f64,
    /// The angle of the Shape
    pub myNaviDegreeAngle: f64,
    /// The img file (include path)
    pub myImgFile: String,
    /// shape name
    pub myName: String,
}

impl std::ops::Deref for Shape {
    type Target = Named;
    fn deref(&self) -> &Named {
        &self.base
    }
}
impl std::ops::DerefMut for Shape {
    fn deref_mut(&mut self) -> &mut Named {
        &mut self.base
    }
}

// C++ static members of Shape (port as module-level statics, see AGENTS.md):
//   static const std::string Shape::DEFAULT_TYPE;
//   static const double Shape::DEFAULT_LAYER;
//   static const double Shape::DEFAULT_LINEWIDTH;
//   static const double Shape::DEFAULT_LAYER_POI;
//   static const double Shape::DEFAULT_ANGLE;
//   static const std::string Shape::DEFAULT_IMG_FILE;
//   static const double Shape::DEFAULT_IMG_WIDTH;
//   static const double Shape::DEFAULT_IMG_HEIGHT;
//   static const double Shape::DEFAULT_HEIGHT;
//   static const std::string Shape::DEFAULT_NAME;
//   static const Parameterised::Map Shape::DEFAULT_PARAMETERS;

impl Shape {
    /// C++: `void Shape::Shape(const std::string &)`
    /// default consructor
    // Shape::Shape(const std::string& id) :
    //     Named(id),
    //     myName(DEFAULT_NAME) {
    // }
    #[allow(unused_variables)]
    pub fn new(id: &str) -> Self {
        todo!("port Shape::Shape")
    }

    /// C++: `void Shape::Shape(const std::string &, const std::string &, const RGBColor &, double, double, const std::string &, const std::string &)`
    /// Constructor
    // Shape::Shape(const std::string& id, const std::string& type, const RGBColor& color, double layer,
    //              double angle, const std::string& imgFile, const std::string& name) :
    //     Named(id),
    //     myType(type),
    //     myColor(color),
    //     myLayer(layer),
    //     myNaviDegreeAngle(angle),
    //     myImgFile(imgFile),
    //     myName(name) {
    // }
    #[allow(unused_variables)]
    pub fn new_2(id: &str, type_: &str, color: &RGBColor, layer: f64, angle: f64, imgFile: &str, name: &str) -> Self {
        todo!("port Shape::Shape")
    }

    /// C++: `void Shape::~Shape() noexcept` [virtual]
    /// Destructor
    // Shape::~Shape() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port Shape::~Shape")
    }

    /// C++: `void Shape::writeShapeAttributes(OutputDevice &, const RGBColor &, const double) const` [const]
    /// write shape attributes in a xml file
    // void
    // Shape::writeShapeAttributes(OutputDevice& device, const RGBColor& defaultColor, const double defaultLayer) const {
    //     // name
    //     if (myName != DEFAULT_NAME) {
    //         device.writeAttr(SUMO_ATTR_NAME, myName);
    //     }
    //     // type
    //     if (myType != DEFAULT_TYPE) {
    //         device.writeAttr(SUMO_ATTR_TYPE, StringUtils::escapeXML(myType));
    //     }
    //     // color
    //     if (myColor != defaultColor) {
    //         device.writeAttr(SUMO_ATTR_COLOR, myColor);
    //     }
    //     // layer
    //     if (myLayer != defaultLayer) {
    //         device.writeAttr(SUMO_ATTR_LAYER, myLayer);
    //     }
    //     // angle
    //     if (myNaviDegreeAngle != Shape::DEFAULT_ANGLE) {
    //         device.writeAttr(SUMO_ATTR_ANGLE, myNaviDegreeAngle);
    //     }
    //     // img file
    //     if (myImgFile != Shape::DEFAULT_IMG_FILE) {
    //         device.writeAttr(SUMO_ATTR_IMGFILE, myImgFile);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn write_shape_attributes(&self, device: &mut OutputDevice, defaultColor: &RGBColor, defaultLayer: f64) {
        todo!("port Shape::writeShapeAttributes")
    }

    /// C++: `const std::string & Shape::getShapeType() const` [const]
    /// Returns the (abstract) type of the Shape
    // const std::string&
    // Shape::getShapeType() const {
    //     return myType;
    // }
    #[allow(unused_variables)]
    pub fn get_shape_type(&self) -> &str {
        todo!("port Shape::getShapeType")
    }

    /// C++: `const RGBColor & Shape::getShapeColor() const` [const]
    /// Returns the color of the Shape
    // const RGBColor&
    // Shape::getShapeColor() const {
    //     return myColor;
    // }
    #[allow(unused_variables)]
    pub fn get_shape_color(&self) -> &RGBColor {
        todo!("port Shape::getShapeColor")
    }

    /// C++: `double Shape::getShapeLayer() const` [const]
    /// Returns the layer of the Shape
    // double
    // Shape::getShapeLayer() const {
    //     return myLayer;
    // }
    #[allow(unused_variables)]
    pub fn get_shape_layer(&self) -> f64 {
        todo!("port Shape::getShapeLayer")
    }

    /// C++: `double Shape::getShapeNaviDegree() const` [const]
    /// Returns the angle of the Shape in navigational degrees
    // double
    // Shape::getShapeNaviDegree() const {
    //     return myNaviDegreeAngle;
    // }
    #[allow(unused_variables)]
    pub fn get_shape_navi_degree(&self) -> f64 {
        todo!("port Shape::getShapeNaviDegree")
    }

    /// C++: `const std::string & Shape::getShapeImgFile() const` [const]
    /// Returns the imgFile of the Shape
    // const std::string&
    // Shape::getShapeImgFile() const {
    //     return myImgFile;
    // }
    #[allow(unused_variables)]
    pub fn get_shape_img_file(&self) -> &str {
        todo!("port Shape::getShapeImgFile")
    }

    /// C++: `const std::string & Shape::getShapeName() const` [const]
    /// Returns the name of the Shape
    // const std::string&
    // Shape::getShapeName() const {
    //     return myName;
    // }
    #[allow(unused_variables)]
    pub fn get_shape_name(&self) -> &str {
        todo!("port Shape::getShapeName")
    }

    /// C++: `void Shape::setShapeType(const std::string &)`
    /// Sets a new type
    // void
    // Shape::setShapeType(const std::string& type) {
    //     myType = type;
    // }
    #[allow(unused_variables)]
    pub fn set_shape_type(&mut self, type_: &str) {
        todo!("port Shape::setShapeType")
    }

    /// C++: `void Shape::setShapeColor(const RGBColor &)`
    /// Sets a new color
    // void
    // Shape::setShapeColor(const RGBColor& col) {
    //     myColor = col;
    // }
    #[allow(unused_variables)]
    pub fn set_shape_color(&mut self, col: &RGBColor) {
        todo!("port Shape::setShapeColor")
    }

    /// C++: `void Shape::setShapeAlpha(unsigned char)`
    /// Sets a new alpha value
    // void
    // Shape::setShapeAlpha(unsigned char alpha) {
    //     myColor.setAlpha(alpha);
    // }
    #[allow(unused_variables)]
    pub fn set_shape_alpha(&mut self, alpha: u8) {
        todo!("port Shape::setShapeAlpha")
    }

    /// C++: `void Shape::setShapeLayer(const double)`
    /// Sets a new layer
    // void
    // Shape::setShapeLayer(const double layer) {
    //     myLayer = layer;
    // }
    #[allow(unused_variables)]
    pub fn set_shape_layer(&mut self, layer: f64) {
        todo!("port Shape::setShapeLayer")
    }

    /// C++: `void Shape::setShapeNaviDegree(const double)` [virtual]
    /// Sets a new angle in navigational degrees
    // void
    // Shape::setShapeNaviDegree(const double angle) {
    //     myNaviDegreeAngle = angle;
    // }
    #[allow(unused_variables)]
    pub fn set_shape_navi_degree(&mut self, angle: f64) {
        todo!("port Shape::setShapeNaviDegree")
    }

    /// C++: `void Shape::setShapeImgFile(const std::string &)`
    /// Sets a new imgFile
    // void
    // Shape::setShapeImgFile(const std::string& imgFile) {
    //     myImgFile = imgFile;
    // }
    #[allow(unused_variables)]
    pub fn set_shape_img_file(&mut self, imgFile: &str) {
        todo!("port Shape::setShapeImgFile")
    }

    /// C++: `void Shape::setShapeName(const std::string &)`
    /// Sets a new shape name
    // void
    // Shape::setShapeName(const std::string& name) {
    //     myName = name;
    // }
    #[allow(unused_variables)]
    pub fn set_shape_name(&mut self, name: &str) {
        todo!("port Shape::setShapeName")
    }

}

