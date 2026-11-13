//! Port of utils/geom/GeoConvHelper.{h,cpp}
//!
//! shortcut: no PROJ library binding. Proj-based conversions (UTM, DHDN, DHDN_UTM and explicit
//! proj strings) keep the projection string / zone logic but convert coordinates as if the projection
//! were the identity; bind PROJ (e.g. the `proj` crate) when geo-input/output tests need it.
//! The "!" (none) and "-" (simple) methods plus offset/scale/rotation are complete.
use std::cell::RefCell;
use std::collections::BTreeMap;

use super::geom_helper::deg2rad;
use super::{Boundary, Position};
use crate::common::msg_handler::{write_error, write_warning};
use crate::common::to_string::f64_to_string;
use crate::common::util_exceptions::ProcessError;
use crate::options::option::SumoOption;
use crate::options::options_cont::OptionsCont;

/// projection method
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ProjectionMethod {
    None,
    Simple,
    Utm,
    Dhdn,
    DhdnUtm,
    Proj,
}

/// coordinate conversion for the current net
#[derive(Clone, Debug)]
pub struct GeoConvHelper {
    /// A proj options string describing the proj.4-projection to use
    proj_string: String,
    /// stands in for `myProjection != nullptr`
    has_projection: bool,
    offset: Position,
    geo_scale: f64,
    sin: f64,
    cos: f64,
    projection_method: ProjectionMethod,
    use_inverse_projection: bool,
    flatten: bool,
    orig_boundary: Boundary,
    conv_boundary: Boundary,
}

impl PartialEq for GeoConvHelper {
    fn eq(&self, o: &Self) -> bool {
        self.proj_string == o.proj_string
            && self.offset == o.offset
            && self.projection_method == o.projection_method
            && self.orig_boundary == o.orig_boundary
            && self.conv_boundary == o.conv_boundary
            && self.geo_scale == o.geo_scale
            && self.cos == o.cos
            && self.sin == o.sin
            && self.use_inverse_projection == o.use_inverse_projection
            && self.flatten == o.flatten
    }
}

struct Statics {
    processing: GeoConvHelper,
    loaded: GeoConvHelper,
    final_: GeoConvHelper,
    num_loaded: i32,
    loaded_plain: BTreeMap<String, (String, Position)>,
}

fn none_helper() -> GeoConvHelper {
    GeoConvHelper::new("!", Position::default(), Boundary::new(), Boundary::new(), 1.0, 0.0, false, false)
}

thread_local! {
    static STATICS: RefCell<Statics> = RefCell::new(Statics {
        processing: none_helper(),
        loaded: none_helper(),
        final_: none_helper(),
        num_loaded: 0,
        loaded_plain: BTreeMap::new(),
    });
}

/// StringUtils::replace (all occurrences, no-op for empty pattern)
fn replace(s: &str, what: &str, by: &str) -> String {
    if what.is_empty() { s.to_string() } else { s.replace(what, by) }
}

impl GeoConvHelper {
    /// C++ defaults scale = 1.0, rot = 0.0, inverse = false, flatten = false
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        proj: &str,
        offset: Position,
        orig: Boundary,
        conv: Boundary,
        scale: f64,
        rot: f64,
        inverse: bool,
        flatten: bool,
    ) -> Self {
        // older PROJ libraries fail to construct inverse projection if this string is present
        let proj_string = replace(proj, "+type=crs", "");
        let projection_method = match proj {
            "!" => ProjectionMethod::None,
            "-" => ProjectionMethod::Simple,
            "UTM" => ProjectionMethod::Utm,
            "DHDN" => ProjectionMethod::Dhdn,
            "DHDN_UTM" => ProjectionMethod::DhdnUtm,
            _ => ProjectionMethod::Proj,
        };
        GeoConvHelper {
            proj_string,
            // shortcut: initProj always "succeeds" (identity) for explicit proj strings
            has_projection: projection_method == ProjectionMethod::Proj,
            offset,
            geo_scale: scale,
            sin: deg2rad(-rot).sin(), // rotate clockwise
            cos: deg2rad(-rot).cos(),
            projection_method,
            use_inverse_projection: inverse,
            flatten,
            orig_boundary: orig,
            conv_boundary: conv,
        }
    }

    /// GeoConvHelper::init(OptionsCont&)
    // shortcut: PROJ data search path (SUMO_HOME/data/proj) is not set, no PROJ binding yet
    pub fn init_from_options(oc: &OptionsCont) -> Result<bool, ProcessError> {
        let mut proj: String = "!".into(); // the default
        let scale = oc.get_float("proj.scale")?;
        let rot = oc.get_float("proj.rotate")?;
        let offset = Position::new3(oc.get_float("offset.x")?, oc.get_float("offset.y")?, oc.get_float("offset.z")?);
        let inverse = oc.exists("proj.inverse") && oc.get_bool("proj.inverse")?;
        let flatten = oc.exists("flatten") && oc.get_bool("flatten")?;

        if oc.get_bool("simple-projection")? {
            proj = "-".into();
        }

        if oc.get_bool("proj.inverse")? && oc.get_string("proj")? == "!" {
            write_error("Inverse projection works only with explicit proj parameters.");
            return Ok(false);
        }
        let num_projections = u32::from(oc.get_bool("simple-projection")?)
            + u32::from(oc.get_bool("proj.utm")?)
            + u32::from(oc.get_bool("proj.dhdn")?)
            + u32::from(oc.get_bool("proj.dhdnutm")?)
            + u32::from(oc.get_string("proj")?.len() > 1);
        if num_projections > 1 {
            write_error("The projection method needs to be uniquely defined.");
            return Ok(false);
        }

        if oc.get_bool("proj.utm")? {
            proj = "UTM".into();
        } else if oc.get_bool("proj.dhdn")? {
            proj = "DHDN".into();
        } else if oc.get_bool("proj.dhdnutm")? {
            proj = "DHDN_UTM".into();
        } else if !oc.is_default("proj") {
            proj = oc.get_string("proj")?;
        }

        let h = GeoConvHelper::new(&proj, offset, Boundary::new(), Boundary::new(), scale, rot, inverse, flatten);
        STATICS.with(|s| {
            let mut s = s.borrow_mut();
            s.final_ = h.clone();
            s.processing = h;
        });
        Ok(true)
    }

    /// Initialises the processing and the final instance using the given proj.4-definition and complete network parameter
    pub fn init(proj: &str, offset: Position, orig: Boundary, conv: Boundary, scale: f64) {
        let mut h = GeoConvHelper::new(proj, offset, orig, conv, scale, 0.0, false, false);
        h.resolve_abstract_projection();
        STATICS.with(|s| {
            let mut s = s.borrow_mut();
            s.final_ = h.clone();
            s.processing = h;
        });
    }

    /// GeoConvHelper::addProjectionOptions(OptionsCont&)
    pub fn add_projection_options(oc: &mut OptionsCont) {
        oc.add_option_sub_topic("Projection");

        oc.do_register("simple-projection", SumoOption::bool(false));
        oc.add_synonyme("simple-projection", "proj.simple", true);
        oc.add_description("simple-projection", "Projection", "Uses a simple method for projection");

        oc.do_register("proj.scale", SumoOption::float(1.0));
        oc.add_description("proj.scale", "Projection", "Scaling factor for input coordinates");

        oc.do_register("proj.rotate", SumoOption::float(0.0));
        oc.add_description("proj.rotate", "Projection", "Rotation (clockwise degrees) for input coordinates");

        oc.do_register("proj.utm", SumoOption::bool(false));
        oc.add_description("proj.utm", "Projection", "Determine the UTM zone (for a universal transversal mercator projection based on the WGS84 ellipsoid)");

        oc.do_register("proj.dhdn", SumoOption::bool(false));
        oc.add_description("proj.dhdn", "Projection", "Determine the DHDN zone (for a transversal mercator projection based on the bessel ellipsoid, \"Gauss-Krueger\")");

        oc.do_register("proj", SumoOption::string_with("!"));
        oc.add_description("proj", "Projection", "Uses STR as proj.4 definition for projection");

        oc.do_register("proj.inverse", SumoOption::bool(false));
        oc.add_description("proj.inverse", "Projection", "Inverses projection");

        oc.do_register("proj.dhdnutm", SumoOption::bool(false));
        oc.add_description("proj.dhdnutm", "Projection", "Convert from Gauss-Krueger to UTM");
    }

    /// the coordinate transformation to use for input conversion and processing
    pub fn with_processing<R>(f: impl FnOnce(&mut GeoConvHelper) -> R) -> R {
        STATICS.with(|s| f(&mut s.borrow_mut().processing))
    }
    /// the coordinate transformation that was loaded from an input file
    pub fn with_loaded<R>(f: impl FnOnce(&mut GeoConvHelper) -> R) -> R {
        STATICS.with(|s| f(&mut s.borrow_mut().loaded))
    }
    /// the coordinate transformation for writing the location element
    pub fn with_final<R>(f: impl FnOnce(&GeoConvHelper) -> R) -> R {
        STATICS.with(|s| f(&s.borrow().final_))
    }
    pub fn get_num_loaded() -> i32 {
        STATICS.with(|s| s.borrow().num_loaded)
    }

    /// compute the location attributes which will be used for output
    pub fn compute_final(lefthand: bool) {
        STATICS.with(|s| {
            let s = &mut *s.borrow_mut();
            if s.num_loaded == 0 {
                s.final_ = s.processing.clone();
                if lefthand {
                    s.final_.offset.mul_xy(1., -1.);
                }
            } else {
                if lefthand {
                    s.processing.offset.mul_xy(1., -1.);
                }
                s.final_ = GeoConvHelper::new(
                    // prefer options over loaded location
                    if s.processing.using_geo_projection() { &s.processing.proj_string } else { &s.loaded.proj_string },
                    // let offset and boundary lead back to the original coords of the loaded data
                    s.processing.get_offset() + s.loaded.get_offset(),
                    s.loaded.orig_boundary,
                    // the new boundary (updated during loading)
                    s.processing.conv_boundary,
                    1.0,
                    0.0,
                    false,
                    false,
                );
            }
            if lefthand {
                s.final_.conv_boundary.flip_y();
            }
        });
    }

    /// sets the coordinate transformation loaded from a location element
    pub fn set_loaded(loaded: &GeoConvHelper) {
        STATICS.with(|s| {
            let mut s = s.borrow_mut();
            s.num_loaded += 1;
            if s.num_loaded > 1 {
                write_warning(&format!(
                    "Ignoring loaded location attribute nr. {} for tracking of original location",
                    s.num_loaded
                ));
            } else {
                s.loaded = loaded.clone();
            }
        });
    }

    /// registers the coordinate transformation as having been loaded from the given file
    pub fn set_loaded_plain(nod_file: &str, loaded: &GeoConvHelper) {
        STATICS.with(|s| {
            s.borrow_mut().loaded_plain.insert(nod_file.to_string(), (loaded.proj_string.clone(), loaded.offset));
        });
    }

    /// C++ default suffix = ".edg.xml"
    pub fn get_loaded_plain(plain_file: &str, suffix: &str) -> Option<GeoConvHelper> {
        let nod_file = replace(plain_file, suffix, ".nod.xml");
        STATICS.with(|s| {
            s.borrow().loaded_plain.get(&nod_file).map(|(proj, offset)| {
                GeoConvHelper::new(proj, *offset, Boundary::new(), Boundary::new(), 1.0, 0.0, false, false)
            })
        })
    }

    /// resets loaded location elements
    pub fn reset_loaded() {
        STATICS.with(|s| {
            let mut s = s.borrow_mut();
            s.num_loaded = 0;
            s.loaded_plain.clear();
        });
    }

    /// Converts the given cartesian (shifted) position to its geo (lat/long) representation
    pub fn cartesian2geo(&self, cartesian: &mut Position) {
        cartesian.sub(&self.get_offset_base());
        if self.projection_method == ProjectionMethod::None {
            return;
        }
        if self.projection_method == ProjectionMethod::Simple {
            let y = cartesian.y() / 111136.;
            let x = cartesian.x() / 111320. / deg2rad(y).cos();
            cartesian.set(x, y);
        }
        // shortcut: inverse PROJ projection is the identity here
    }

    /// Converts the given coordinate into a cartesian and optionally update the conv boundary
    /// (C++ default include_in_boundary = true)
    pub fn x2cartesian(&mut self, from: &mut Position, include_in_boundary: bool) -> bool {
        if include_in_boundary {
            self.orig_boundary.add_pos(from);
        }
        // init projection parameter on first use
        if !self.has_projection {
            let mut x = from.x() * self.geo_scale;
            let mut utm = false;
            match self.projection_method {
                ProjectionMethod::DhdnUtm => {
                    let zone = ((x - 500000.) / 1000000.) as i32;
                    if !(1..=5).contains(&zone) {
                        write_warning(&format!("Attempt to initialize DHDN_UTM-projection on invalid longitude {}", f64_to_string(x)));
                        return false;
                    }
                    self.proj_string = dhdn_proj_string(zone);
                    // shortcut: inverse DHDN projection to WGS84 is not applied
                    x = ((x - 500000.) / 1000000.) * 3.; // continues with UTM
                    utm = true;
                }
                ProjectionMethod::Utm => utm = true,
                ProjectionMethod::Dhdn => {
                    let zone = (x / 3.) as i32;
                    if !(1..=5).contains(&zone) {
                        write_warning(&format!("Attempt to initialize DHDN-projection on invalid longitude {}", f64_to_string(x)));
                        return false;
                    }
                    self.proj_string = dhdn_proj_string(zone);
                    self.has_projection = true;
                }
                _ => {}
            }
            if utm {
                let zone = (x + 180.) as i32 / 6 + 1;
                self.proj_string = format!("+proj=utm +zone={zone} +ellps=WGS84 +datum=WGS84 +units=m +no_defs");
                self.has_projection = true;
            }
        }
        // perform conversion
        let ok = self.x2cartesian_const(from);
        if ok && include_in_boundary {
            self.conv_boundary.add_pos(from);
        }
        ok
    }

    /// Converts the given coordinate into a cartesian using the previous initialisation
    pub fn x2cartesian_const(&self, from: &mut Position) -> bool {
        let x2 = from.x() * self.geo_scale;
        let y2 = from.y() * self.geo_scale;
        let mut x = x2 * self.cos - y2 * self.sin;
        let mut y = x2 * self.sin + y2 * self.cos;
        if self.projection_method == ProjectionMethod::None {
            // do nothing
        } else if self.use_inverse_projection {
            self.cartesian2geo(from);
        } else {
            if x > 180.1 || x < -180.1 {
                write_warning(&format!("Invalid longitude {}", f64_to_string(x)));
                return false;
            }
            if y > 90.1 || y < -90.1 {
                write_warning(&format!("Invalid latitude {}", f64_to_string(y)));
                return false;
            }
            // shortcut: forward PROJ projection is the identity here
            if self.projection_method == ProjectionMethod::Simple {
                // Sinusoidal projection
                x *= 111320. * deg2rad(y).cos();
                y *= 111136.;
            }
        }
        if x > f64::MAX || y > f64::MAX {
            return false;
        }
        from.set(x, y);
        from.add(&self.offset);
        if self.flatten {
            from.setz(0.);
        }
        true
    }

    /// init projString such as 'UTM' in loaded projection
    pub fn resolve_abstract_projection(&mut self) {
        if !self.has_projection
            && self.projection_method != ProjectionMethod::None
            && self.projection_method != ProjectionMethod::Simple
        {
            let orig_proj = self.proj_string.clone();
            // try to initialized projection based on origBoundary
            let mut tmp = self.orig_boundary.get_center();
            self.x2cartesian(&mut tmp, false);
            if !self.has_projection {
                write_warning(&format!(
                    "Failed to intialized projection '{}' based on origBoundary centered on '{}'",
                    orig_proj,
                    self.orig_boundary.get_center()
                ));
                self.projection_method = ProjectionMethod::None;
            }
        }
    }

    /// Returns whether a transformation from geo to metric coordinates will be performed
    pub fn using_geo_projection(&self) -> bool {
        self.projection_method != ProjectionMethod::None
    }
    pub fn using_inverse_geo_projection(&self) -> bool {
        self.use_inverse_projection
    }
    /// Shifts the converted boundary by the given amounts
    pub fn move_converted_by(&mut self, x: f64, y: f64) {
        self.offset.add_xy(x, y);
        self.conv_boundary.moveby(x, y, 0.);
    }
    pub fn get_orig_boundary(&self) -> &Boundary {
        &self.orig_boundary
    }
    pub fn get_conv_boundary(&self) -> &Boundary {
        &self.conv_boundary
    }
    pub fn set_conv_boundary(&mut self, boundary: Boundary) {
        self.conv_boundary = boundary;
    }
    pub fn get_offset(&self) -> Position {
        self.offset
    }
    pub fn get_offset_base(&self) -> Position {
        self.offset
    }
    pub fn get_proj_string(&self) -> &str {
        &self.proj_string
    }
    // shortcut: writeLocation needs OutputDevice; write netOffset/convBoundary/origBoundary
    // (gPrecisionGeo when using_geo_projection)/projParameter from with_final once iodevices is ported
}

fn dhdn_proj_string(zone: i32) -> String {
    format!(
        "+proj=tmerc +lat_0=0 +lon_0={} +k=1 +x_0={} +y_0=0 +ellps=bessel +datum=potsdam +units=m +no_defs",
        3 * zone,
        zone * 1000000 + 500000
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn offset_and_simple() {
        let mut h = GeoConvHelper::new("!", Position::new(100., -50.), Boundary::new(), Boundary::new(), 1.0, 0.0, false, false);
        let mut p = Position::new(1., 2.);
        assert!(h.x2cartesian(&mut p, true));
        assert_eq!(p, Position::new(101., -48.));
        assert_eq!(h.get_conv_boundary().get_center(), Position::new(101., -48.));
        h.cartesian2geo(&mut p);
        assert_eq!(p, Position::new(1., 2.));

        let s = GeoConvHelper::new("-", Position::default(), Boundary::new(), Boundary::new(), 1.0, 0.0, false, false);
        let mut g = Position::new(0., 1.);
        assert!(s.x2cartesian_const(&mut g));
        assert_eq!(g, Position::new(0., 111136.));
        s.cartesian2geo(&mut g);
        assert_eq!(g, Position::new(0., 1.));
        assert!(!s.x2cartesian_const(&mut Position::new(200., 0.)));

        GeoConvHelper::init("!", Position::new(5., 5.), Boundary::new(), Boundary::new(), 1.0);
        GeoConvHelper::compute_final(false);
        assert_eq!(GeoConvHelper::with_final(|f| f.get_offset()), Position::new(5., 5.));
    }

    #[test]
    fn simple_projection_round_trip() {
        let s = GeoConvHelper::new("-", Position::default(), Boundary::new(), Boundary::new(), 1.0, 0.0, false, false);
        let mut p = Position::new(8.5, 49.5);
        assert!(s.x2cartesian_const(&mut p));
        s.cartesian2geo(&mut p);
        assert!((p.x() - 8.5).abs() < 1e-6);
        assert!((p.y() - 49.5).abs() < 1e-6);
    }

    #[test]
    fn init_from_options_projection_choice() {
        let mut oc = OptionsCont::new();
        GeoConvHelper::add_projection_options(&mut oc);
        for name in ["offset.x", "offset.y", "offset.z"] {
            oc.do_register(name, SumoOption::float(0.0));
        }
        assert!(GeoConvHelper::init_from_options(&oc).unwrap());
        assert!(!GeoConvHelper::with_final(|f| f.using_geo_projection()));

        oc.set("simple-projection", "true", false).unwrap();
        assert!(GeoConvHelper::init_from_options(&oc).unwrap());
        assert!(GeoConvHelper::with_final(|f| f.using_geo_projection()));

        oc.set("proj.utm", "true", false).unwrap();
        assert!(!GeoConvHelper::init_from_options(&oc).unwrap());
    }
}
