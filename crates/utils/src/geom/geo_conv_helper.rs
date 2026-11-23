//! Port of utils/geom/GeoConvHelper.{h,cpp}
//!
//! UTM, DHDN and DHDN_UTM use an in-crate port of PROJ's default `etmerc` (see `Etmerc`), matching PROJ
//! to well below 1 mm. The "!" (none) and "-" (simple) methods plus offset/scale/rotation are complete.
//! shortcut: other explicit proj strings are projected only when they are +proj=utm or +proj=tmerc with lat_0=0.
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::f64::consts::PI;

use super::geom_helper::{deg2rad, rad2deg};
use super::{Boundary, Position};
use crate::common::msg_handler::{write_error, write_warning};
use crate::common::std_defs::{g_precision, g_precision_geo};
use crate::common::string_utils::escape_xml;
use crate::common::to_string::f64_to_string;
use crate::common::util_exceptions::ProcessError;
use crate::iodevices::output_device::OutputDevice;
use crate::options::option::SumoOption;
use crate::options::options_cont::OptionsCont;
use crate::xml::sumo_xml_definitions::{SUMO_ATTR_CONV_BOUNDARY, SUMO_ATTR_NET_OFFSET, SUMO_ATTR_ORIG_BOUNDARY, SUMO_ATTR_ORIG_PROJ, SUMO_TAG_LOCATION};

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
    offset: Position,
    geo_scale: f64,
    sin: f64,
    cos: f64,
    projection_method: ProjectionMethod,
    use_inverse_projection: bool,
    flatten: bool,
    orig_boundary: Boundary,
    conv_boundary: Boundary,
    /// C++ `myProjection`: set once the projection is known (`None` until initialised)
    projection: Option<Projection>,
    /// C++ `myInverseProjection`: DHDN Gauss-Krueger projection used by DHDN_UTM to turn the input
    /// easting/northing into lon/lat before the WGS84 UTM forward projection is applied
    dhdn_inverse: Option<Etmerc>,
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
        let projection = if projection_method == ProjectionMethod::Proj { Projection::from_proj(&proj_string) } else { None };
        GeoConvHelper {
            proj_string,
            projection,
            dhdn_inverse: None,
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
            return;
        }
        if let Some(p) = self.projection {
            let (lon, lat) = p.inv(cartesian.x(), cartesian.y());
            cartesian.set(rad2deg(lon), rad2deg(lat));
        }
    }

    /// Converts the given coordinate into a cartesian and optionally update the conv boundary
    /// (C++ default include_in_boundary = true)
    pub fn x2cartesian(&mut self, from: &mut Position, include_in_boundary: bool) -> bool {
        if include_in_boundary {
            self.orig_boundary.add_pos(from);
        }
        // init projection parameter on first use
        if self.projection.is_none() {
            let x = from.x() * self.geo_scale;
            match self.projection_method {
                ProjectionMethod::DhdnUtm => {
                    let zone = ((x - 500000.) / 1000000.) as i32;
                    if !(1..=5).contains(&zone) {
                        write_warning(&format!("Attempt to initialize DHDN_UTM-projection on invalid longitude {}", f64_to_string(x)));
                        return false;
                    }
                    // C++: myInverseProjection = proj_create(dhdn Gauss-Krueger string)
                    self.dhdn_inverse = Etmerc::from_proj(&dhdn_proj_string(zone));
                    // C++ then falls through into the UTM case with x = ((x-500000)/1e6)*3
                    let lon = ((x - 500000.) / 1000000.) * 3.;
                    let utm_zone = (lon + 180.) as i32 / 6 + 1;
                    self.proj_string = format!("+proj=utm +zone={utm_zone} +ellps=WGS84 +datum=WGS84 +units=m +no_defs");
                    self.projection = Projection::from_proj(&self.proj_string);
                }
                ProjectionMethod::Utm => {
                    let zone = (x + 180.) as i32 / 6 + 1;
                    self.proj_string = format!("+proj=utm +zone={zone} +ellps=WGS84 +datum=WGS84 +units=m +no_defs");
                    self.projection = Projection::from_proj(&self.proj_string);
                }
                ProjectionMethod::Dhdn => {
                    let zone = (x / 3.) as i32;
                    if !(1..=5).contains(&zone) {
                        write_warning(&format!("Attempt to initialize DHDN-projection on invalid longitude {}", f64_to_string(x)));
                        return false;
                    }
                    self.proj_string = dhdn_proj_string(zone);
                    self.projection = Projection::from_proj(&self.proj_string);
                }
                _ => {}
            }
            if matches!(self.projection_method, ProjectionMethod::Utm | ProjectionMethod::Dhdn | ProjectionMethod::DhdnUtm)
                && (self.projection.is_none() || (self.projection_method == ProjectionMethod::DhdnUtm && self.dhdn_inverse.is_none()))
            {
                write_error(&format!("Could not build projection '{}'", self.proj_string));
                return false;
            }
        }
        // C++ applies myInverseProjection on every call, not just while initialising: DHDN_UTM input
        // is a Gauss-Krueger easting/northing and must become lon/lat before the UTM forward step.
        if let Some(inv) = &self.dhdn_inverse {
            let (lon, lat) = inv.inv(from.x(), from.y());
            from.set(rad2deg(lon), rad2deg(lat));
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
            if let Some(p) = self.projection {
                (x, y) = p.fwd(deg2rad(x), deg2rad(y));
            } else if self.projection_method == ProjectionMethod::Proj {
                write_error(&format!("Projection '{}' is not supported by this build (only +proj=utm and +proj=tmerc with lat_0=0)", self.proj_string));
                return false;
            }
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
        if self.projection.is_none()
            && self.projection_method != ProjectionMethod::None
            && self.projection_method != ProjectionMethod::Simple
        {
            let orig_proj = self.proj_string.clone();
            // try to initialized projection based on origBoundary
            let mut tmp = self.orig_boundary.get_center();
            self.x2cartesian(&mut tmp, false);
            if self.projection.is_none() {
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
    /// writeLocation: the <location> element of the net and additional files
    pub fn write_location(&self, into: &mut OutputDevice) {
        into.open_tag_id(SUMO_TAG_LOCATION);
        into.write_attr(SUMO_ATTR_NET_OFFSET, &self.get_offset_base());
        into.write_attr(SUMO_ATTR_CONV_BOUNDARY, self.get_conv_boundary());
        if self.using_geo_projection() {
            into.set_precision(g_precision_geo());
        }
        into.write_attr(SUMO_ATTR_ORIG_BOUNDARY, self.get_orig_boundary());
        if self.using_geo_projection() {
            into.set_precision(g_precision());
        }
        into.write_attr(SUMO_ATTR_ORIG_PROJ, escape_xml(self.get_proj_string(), false).as_str());
        into.close_tag("");
        into.lf();
    }
}

/// Transverse Mercator (`+proj=utm` / `+proj=tmerc`) using PROJ's default Evenden/Snyder
/// algorithm. The pristine SUMO binary links a PROJ build whose default transverse Mercator
/// is `evenden_snyder` (see its proj.ini). The Poder/Engsager (`etmerc`) series round-trips
/// lon/lat a few ULPs differently, which flips the 6-digit geo output at exact `.5` boundaries.
#[derive(Clone, Copy, Debug)]
struct Etmerc {
    a: f64,
    es: f64,
    esp: f64,
    /// `en[0]` is the quarter-meridian scale, `en[1..7]` convert phi->mu, `en[7..13]` mu->phi.
    en: [f64; 13],
    ml0: f64,
    k0: f64,
    lon0: f64,
    x0: f64,
    y0: f64,
}

/// PROJ `adjlon`: wraps a longitude difference into [-pi, pi]
fn adjlon(lon: f64) -> f64 {
    if lon.abs() <= PI { lon } else { (lon + PI).rem_euclid(2. * PI) - PI }
}

/// PROJ `polyval`: Horner evaluation of a polynomial with `p[0]` the constant term.
fn polyval(x: f64, p: &[f64]) -> f64 {
    let mut y = *p.last().unwrap();
    for &c in p[..p.len() - 1].iter().rev() {
        y = y * x + c;
    }
    y
}

/// PROJ `clenshaw`: `sum(c[k] * sin((2*k+2) * zeta))` from `sin(zeta)`/`cos(zeta)`.
fn clenshaw(sz: f64, cz: f64, c: &[f64]) -> f64 {
    let mut u0 = 0.;
    let mut u1 = 0.;
    let x = 2. * (cz - sz) * (cz + sz);
    for &coef in c.iter().rev() {
        let t = x * u0 - u1 + coef;
        u1 = u0;
        u0 = t;
    }
    2. * sz * cz * u0
}

/// PROJ `pj_enfn`: meridian-distance Fourier coefficients in the third flattening n.
fn meridian_coeffs(n: f64) -> [f64; 13] {
    const MU_PHI: [f64; 12] = [
        -3. / 2., 9. / 16., -3. / 32., 15. / 16., -15. / 32., 135. / 2048.,
        -35. / 48., 105. / 256., 315. / 512., -189. / 512., -693. / 1280., 1001. / 2048.,
    ];
    const PHI_MU: [f64; 12] = [
        3. / 2., -27. / 32., 269. / 512., 21. / 16., -55. / 32., 6759. / 4096.,
        151. / 96., -417. / 128., 1097. / 512., -15543. / 2560., 8011. / 2560., 293393. / 61440.,
    ];
    let n2 = n * n;
    let mut en = [0.; 13];
    en[0] = polyval(n2, &[1., 1. / 4., 1. / 64., 1. / 256.]) / (1. + n);
    let mut d = n;
    let mut o = 0usize;
    for l in 0..6 {
        let m = (5 - l) / 2;
        en[l + 1] = d * polyval(n2, &MU_PHI[o..o + m + 1]);
        en[l + 7] = d * polyval(n2, &PHI_MU[o..o + m + 1]);
        d *= n;
        o += m + 1;
    }
    en
}

impl Etmerc {
    fn new(a: f64, rf: f64, k0: f64, lon0: f64, x0: f64, y0: f64) -> Etmerc {
        let f = 1. / rf;
        let n = f / (2. - f);
        let es = f * (2. - f);
        let en = meridian_coeffs(n);
        // lat_0 = 0 for every projection this port accepts, so ml0 = en[0] * clenshaw(0, 1, ..) = 0.
        let ml0 = 0.;
        Etmerc { a, es, esp: es / (1. - es), en, ml0, k0, lon0: deg2rad(lon0), x0, y0 }
    }

    /// Parses the `+proj=utm` and `+proj=tmerc` (lat_0=0) strings this port projects; None for anything else
    fn from_proj(proj: &str) -> Option<Etmerc> {
        let mut kind = None;
        let mut zone = None;
        let mut south = false;
        let mut ellps = None;
        let mut datum = None;
        let mut lon0 = 0.;
        let mut lat0 = 0.;
        let mut k0 = 1.;
        let mut x0 = 0.;
        let mut y0 = 0.;
        for tok in proj.split_whitespace() {
            let body = tok.strip_prefix('+')?;
            let (key, val) = body.split_once('=').unwrap_or((body, ""));
            match key {
                "proj" => kind = Some(val),
                "zone" => zone = Some(val.parse::<i32>().ok()?),
                "south" => south = true,
                "ellps" => ellps = Some(val),
                "datum" => datum = Some(val),
                "lon_0" => lon0 = val.parse::<f64>().ok()?,
                "lat_0" => lat0 = val.parse::<f64>().ok()?,
                "k" | "k_0" => k0 = val.parse::<f64>().ok()?,
                "x_0" => x0 = val.parse::<f64>().ok()?,
                "y_0" => y0 = val.parse::<f64>().ok()?,
                _ => {}
            }
        }
        let (a, rf) = match (ellps, datum) {
            (Some("WGS84"), _) | (None, Some("WGS84")) => (6378137., 298.257223563),
            (Some("GRS80"), _) => (6378137., 298.257222101),
            (Some("bessel"), _) | (None, Some("potsdam")) => (6377397.155, 299.1528128),
            _ => return None,
        };
        match kind? {
            "utm" => {
                let zone: i32 = zone?;
                if !(1..=60).contains(&zone) {
                    return None;
                }
                Some(Etmerc::new(a, rf, 0.9996, (zone * 6 - 183) as f64, 500000., if south { 10000000. } else { 0. }))
            }
            "tmerc" if lat0 == 0. => Some(Etmerc::new(a, rf, k0, lon0, x0, y0)),
            _ => None,
        }
    }

    /// forward: lon/lat in radians to easting/northing (PROJ `approx_e_fwd` + `fwd_finalize`)
    fn fwd(&self, lon: f64, lat: f64) -> (f64, f64) {
        const FC3: f64 = 0.16666666666666666666;
        const FC4: f64 = 0.08333333333333333333;
        const FC5: f64 = 0.05;
        const FC6: f64 = 0.03333333333333333333;
        const FC7: f64 = 0.02380952380952380952;
        const FC8: f64 = 0.01785714285714285714;
        let lam = adjlon(lon - self.lon0);
        let sphi = lat.sin();
        let cphi = lat.cos();
        let mut t = if cphi.abs() > 1e-10 { sphi / cphi } else { 0. };
        t *= t;
        let mut al = cphi * lam;
        let als = al * al;
        al /= (1. - self.es * sphi * sphi).sqrt();
        let n = self.esp * cphi * cphi;
        let x = self.k0 * al * (1. + FC3 * als * (1. - t + n
            + FC5 * als * (5. + t * (t - 18.) + n * (14. - 58. * t)
                + FC7 * als * (61. + t * (t * (179. - t) - 479.)))));
        let ml = self.en[0] * (lat + clenshaw(sphi, cphi, &self.en[1..7]));
        let y = self.k0 * (ml - self.ml0 + sphi * al * lam * 0.5 * (1.
            + FC4 * als * (5. - t + n * (9. + 4. * n)
                + FC6 * als * (61. + t * (t - 58.) + n * (270. - 330. * t)
                    + FC8 * als * (1385. + t * (t * (543. - t) - 3111.))))));
        // fwd_finalize: classic proj coordinates are in units of the semi-major axis,
        // then false easting/northing are added (fr_meter = 1 for metres).
        (self.a * x + self.x0, self.a * y + self.y0)
    }

    /// inverse: easting/northing to lon/lat in radians (PROJ `inv_prepare` + `approx_e_inv` + `inv_finalize`)
    fn inv(&self, x: f64, y: f64) -> (f64, f64) {
        const FC3: f64 = 0.16666666666666666666;
        const FC4: f64 = 0.08333333333333333333;
        const FC5: f64 = 0.05;
        const FC6: f64 = 0.03333333333333333333;
        const FC7: f64 = 0.02380952380952380952;
        const FC8: f64 = 0.01785714285714285714;
        // inv_prepare: de-offset, de-scale, then classic units are 1/a of a radian.
        let x = (x - self.x0) / self.a;
        let y = (y - self.y0) / self.a;
        // pj_inv_mlfn
        let mu = (self.ml0 + y / self.k0) / self.en[0];
        let mut phi = mu + clenshaw(mu.sin(), mu.cos(), &self.en[7..13]);
        if phi.abs() >= 0.5 * PI {
            return (self.lon0, if y < 0. { -0.5 * PI } else { 0.5 * PI });
        }
        let sphi = phi.sin();
        let cphi = phi.cos();
        let mut t = if cphi.abs() > 1e-10 { sphi / cphi } else { 0. };
        let n = self.esp * cphi * cphi;
        let mut con = 1. - self.es * sphi * sphi;
        let d = x * con.sqrt() / self.k0;
        con *= t;
        t *= t;
        let ds = d * d;
        phi -= (con * ds / (1. - self.es)) * 0.5 * (1.
            - ds * FC4 * (5. + t * (3. - 9. * n) + n * (1. - 4. * n)
                - ds * FC6 * (61. + t * (90. - 252. * n + 45. * t) + 46. * n
                    - ds * FC8 * (1385. + t * (3633. + t * (4095. + 1575. * t))))));
        let lam = d * (1. - ds * FC3 * (1. + 2. * t + n
            - ds * FC5 * (5. + t * (28. + 24. * t + 8. * n) + 6. * n
                - ds * FC7 * (61. + t * (662. + t * (1320. + 720. * t)))))) / cphi;
        // inv_finalize: add the central meridian back and wrap.
        (adjlon(self.lon0 + lam), phi)
    }
}

/// PROJ projections supported by this port. `+proj=merc` (Web Mercator) is handled as the spherical
/// case; all other supported strings use the `etmerc` port above.
#[derive(Clone, Copy, Debug)]
enum Projection {
    Etmerc(Etmerc),
    Merc(Merc),
}

impl Projection {
    /// Mirrors the proj strings recognised by `GeoConvHelper::initProj`; None makes
    /// `x2cartesian_const` report the projection as unsupported, as C++ does when PROJ is absent.
    fn from_proj(proj: &str) -> Option<Self> {
        if let Some(merc) = Merc::from_proj(proj) {
            return Some(Projection::Merc(merc));
        }
        Etmerc::from_proj(proj).map(Projection::Etmerc)
    }

    fn fwd(&self, lon: f64, lat: f64) -> (f64, f64) {
        match self {
            Projection::Etmerc(p) => p.fwd(lon, lat),
            Projection::Merc(p) => p.fwd(lon, lat),
        }
    }

    fn inv(&self, x: f64, y: f64) -> (f64, f64) {
        match self {
            Projection::Etmerc(p) => p.inv(x, y),
            Projection::Merc(p) => p.inv(x, y),
        }
    }
}

/// Spherical Mercator (`+proj=merc`), the projection used by Web Mercator / EPSG:3857 nets.
/// PROJ's `merc` forward is `x = a * (lon - lon_0) * k + x_0`,
/// `y = a * ln(tan(pi/4 + lat/2)) * k + y_0` (also written `a * asinh(tan(lat))`).
#[derive(Clone, Copy, Debug)]
struct Merc {
    a: f64,
    lon0: f64,
    x0: f64,
    y0: f64,
    k: f64,
}

impl Merc {
    fn from_proj(proj: &str) -> Option<Merc> {
        let mut kind = None;
        let mut a = None;
        let mut b = None;
        let mut lon0 = 0.;
        let mut x0 = 0.;
        let mut y0 = 0.;
        let mut k = 1.;
        let mut lat_ts = 0.;
        for tok in proj.split_whitespace() {
            let body = tok.strip_prefix('+')?;
            let (key, val) = body.split_once('=').unwrap_or((body, ""));
            match key {
                "proj" => kind = Some(val),
                "a" => a = Some(val.parse::<f64>().ok()?),
                "b" => b = Some(val.parse::<f64>().ok()?),
                "lon_0" => lon0 = val.parse::<f64>().ok()?,
                "lat_ts" => lat_ts = val.parse::<f64>().ok()?,
                "k" | "k_0" => k = val.parse::<f64>().ok()?,
                "x_0" => x0 = val.parse::<f64>().ok()?,
                "y_0" => y0 = val.parse::<f64>().ok()?,
                _ => {}
            }
        }
        if kind? != "merc" || lat_ts != 0. {
            // ellipsoidal merc / non-equatorial standard parallel: not covered by this port
            return None;
        }
        let a = a?;
        if b.is_some_and(|b| b != a) {
            return None;
        }
        Some(Merc { a, lon0: deg2rad(lon0), x0, y0, k })
    }

    fn fwd(&self, lon: f64, lat: f64) -> (f64, f64) {
        let x = self.a * (lon - self.lon0);
        let y = self.a * (0.5 * (0.5 * PI + lat)).tan().ln();
        (x * self.k + self.x0, y * self.k + self.y0)
    }

    fn inv(&self, x: f64, y: f64) -> (f64, f64) {
        let lon = (x - self.x0) / (self.a * self.k) + self.lon0;
        let lat = 2. * ((y - self.y0) / (self.a * self.k)).exp().atan() - 0.5 * PI;
        (lon, lat)
    }
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

    /// The net of tt_basic_person_person_inserted_departPos: Web Mercator of its origBoundary corner.
    #[test]
    fn merc_matches_proj() {
        let proj = "+proj=merc +a=6378137 +b=6378137 +lat_ts=0.0 +lon_0=0.0 +x_0=0.0 +y_0=0 +k=1.0 +units=m +nadgrids=@null +wktext +no_defs";
        let mut h = GeoConvHelper::new(proj, Position::default(), Boundary::new(), Boundary::new(), 1.0, 0.0, false, false);
        let mut p = Position::new(15.566128, 58.386126);
        assert!(h.x2cartesian_const(&mut p));
        // PROJ's spherical merc forward: x = a * lon, y = a * ln(tan(pi/4 + lat/2))
        assert!((p.x() - 6378137. * 15.566128 * PI / 180.).abs() < 1e-6);
        assert!((p.y() - 8048871.742765347).abs() < 1e-5);
        h.cartesian2geo(&mut p);
        assert!((p.x() - 15.566128).abs() < 1e-9);
        assert!((p.y() - 58.386126).abs() < 1e-9);
    }

    fn utm_proj(zone: i32) -> String {
        format!("+proj=utm +zone={zone} +ellps=WGS84 +datum=WGS84 +units=m +no_defs")
    }

    /// PROJ 9.4 reference: `echo "lon lat" | cs2cs +proj=longlat +datum=WGS84 +to +proj=utm +zone=Z +ellps=WGS84 +datum=WGS84 +units=m +no_defs -f %.9f`
    #[test]
    fn utm_matches_proj() {
        let cases: [(f64, f64, i32, f64, f64); 19] = [
            (-75.5, 40.2, 18, 457444.837151998, 4450075.498325491),
            (9.1, 48.7, 32, 507358.079288402, 5394111.900843232),
            (15.5, -33.9, 33, 546228.176321921, -3751180.769750892),
            (-177.3, 64.8, 1, 485747.357523810, 7186199.042602601),
            (177.4, -17.2, 60, 542532.163212333, -1901724.154100325),
            (2.35, 48.85, 31, 452314.891233666, 5410984.887604439),
            (-3.7, 40.4, 30, 440598.078851791, 4472390.031128573),
            (116.4, 39.9, 50, 448709.380192469, 4416830.562171828),
            (151.2, -33.9, 56, 333568.941011552, -3752526.663155980),
            (-123.1, 49.3, 10, 492729.777625124, 5460811.038490374),
            (-46.6, -23.5, 23, 336625.131857350, -2599781.139823964),
            (39.3, -6.8, 37, 533149.363859589, -751649.927888654),
            (-99.1, 19.4, 14, 489500.410979988, 2145090.574145707),
            (0.5, 0.1, 31, 221724.104715127, 11063.605805439),
            (10.0, 80.0, 32, 519384.803295973, 8881752.414807441),
            (10.0, -80.0, 32, 519384.803295973, -8881752.414807441),
            (-0.5, -0.001, 31, 110293.549385932, -110.737988137),
            (179.9, -45.0, 1, 255672.433467196, -4987626.456524279),
            (-179.9, 45.0, 60, 744327.566532804, 4987626.456524279),
        ];
        for (lon, lat, zone, e, n) in cases {
            let mut h = GeoConvHelper::new(&utm_proj(zone), Position::default(), Boundary::new(), Boundary::new(), 1.0, 0.0, false, false);
            let mut p = Position::new(lon, lat);
            assert!(h.x2cartesian(&mut p, false));
            assert!((p.x() - e).abs() < 1e-6, "{lon} {lat} zone {zone}: x {} vs {e}", p.x());
            assert!((p.y() - n).abs() < 1e-6, "{lon} {lat} zone {zone}: y {} vs {n}", p.y());
            h.cartesian2geo(&mut p);
            assert!((p.x() - lon).abs() < 1e-9 && (p.y() - lat).abs() < 1e-9, "inverse {lon} {lat}: {p:?}");
        }
    }

    /// Official SUMO 1.28.0 writes 52.428498 for the OSM input 52.4284985 after the
    /// UTM forward/inverse round-trip at precision 6; the Poder/Engsager series wrote
    /// 52.428499 because its round-trip lands just above the exact .5 boundary.
    #[test]
    fn utm_geo_roundtrip_halfway_boundary() {
        let h = GeoConvHelper::new(&utm_proj(33), Position::default(), Boundary::new(), Boundary::new(), 1.0, 0.0, false, false);
        let mut p = Position::new(13.5275234, 52.4284985);
        assert!(h.x2cartesian_const(&mut p));
        h.cartesian2geo(&mut p);
        assert!(p.y() < 52.4284985, "round-trip must stay below the .5 boundary: {}", p.y());
        assert!((p.y() - 52.428498).abs() < 1e-6);
        assert!((p.x() - 13.527523).abs() < 1e-6);
    }

    #[test]
    fn utm_zone_from_first_point() {
        let mut h = GeoConvHelper::new("UTM", Position::default(), Boundary::new(), Boundary::new(), 1.0, 0.0, false, false);
        let mut p = Position::new(9.1, 48.7);
        assert!(h.x2cartesian(&mut p, false));
        assert_eq!(h.get_proj_string(), utm_proj(32));
        assert!((p.x() - 507358.079288402).abs() < 1e-6 && (p.y() - 5394111.900843232).abs() < 1e-6);
    }

    #[test]
    fn utm_south_and_dhdn_match_proj() {
        let s = GeoConvHelper::new("+proj=utm +zone=33 +south +ellps=WGS84 +datum=WGS84 +units=m +no_defs", Position::default(), Boundary::new(), Boundary::new(), 1.0, 0.0, false, false);
        let mut p = Position::new(15.5, -33.9);
        assert!(s.x2cartesian_const(&mut p));
        assert!((p.y() - 6248819.230249108).abs() < 1e-6);

        // PROJ 9.4: cs2cs +proj=longlat +ellps=bessel +to +proj=tmerc +lat_0=0 +lon_0=12 +k=1 +x_0=4500000 +y_0=0 +ellps=bessel +units=m +no_defs
        let d = GeoConvHelper::new(&dhdn_proj_string(4), Position::default(), Boundary::new(), Boundary::new(), 1.0, 0.0, false, false);
        let mut q = Position::new(12.5, 52.3);
        assert!(d.x2cartesian_const(&mut q));
        assert!((q.x() - 4534104.708578545).abs() < 1e-6 && (q.y() - 5796245.650255103).abs() < 1e-6);
    }

    #[test]
    fn dhdn_utm_inverse_then_utm() {
        // The DHDN reference above is lon/lat (12.5, 52.3) in Gauss-Krueger zone 4. C++ DHDN_UTM
        // first inverts that easting/northing to lon/lat, then applies the WGS84 UTM forward
        // projection of the zone selected by ((x-500000)/1e6)*3 (here: 33).
        let mut h = GeoConvHelper::new("DHDN_UTM", Position::default(), Boundary::new(), Boundary::new(), 1.0, 0.0, false, false);
        let mut p = Position::new(4534104.708578545, 5796245.650255103);
        assert!(h.x2cartesian(&mut p, false));
        assert_eq!(h.get_proj_string(), utm_proj(33));

        let mut u = GeoConvHelper::new(&utm_proj(33), Position::default(), Boundary::new(), Boundary::new(), 1.0, 0.0, false, false);
        let mut expected = Position::new(12.5, 52.3);
        assert!(u.x2cartesian(&mut expected, false));
        assert!((p.x() - expected.x()).abs() < 1e-3, "{} vs {}", p.x(), expected.x());
        assert!((p.y() - expected.y()).abs() < 1e-3, "{} vs {}", p.y(), expected.y());

        // invalid DHDN zone: warning + false, as in GeoConvHelper.cpp
        let mut bad = GeoConvHelper::new("DHDN_UTM", Position::default(), Boundary::new(), Boundary::new(), 1.0, 0.0, false, false);
        assert!(!bad.x2cartesian(&mut Position::new(100000., 5000000.), false));
    }
}
