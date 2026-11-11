//! Port of utils/iodevices/OutputDevice.{h,cpp}, OutputDevice_File, OutputDevice_String,
//! OutputDevice_COUT and OutputDevice_CERR: one OutputDevice type whose sink is the C++ subclass.
//! The device registry (getDevice/closeAll) is thread-local and hands out `OutputDeviceRef`s.
// shortcut: no CSV/Parquet formatters (csv/parquet names fall back to XML), no OutputDevice_Network,
// no binary output, no MsgHandler retriever bookkeeping (closeAll treats all devices alike),
// no StringUtils::substituteEnvironment on file names
use super::options_hook::options;
use super::output_formatter::OutputFormatterType;
use super::plain_xml_formatter::PlainXMLFormatter;
use crate::common::std_defs::g_precision;
use crate::common::time::SUMOTime;
use crate::common::to_string::f64_to_string_prec;
use crate::xml::stand_ins::{append_before_extension, escape_xml, is_socket, iso_time_string, prepend_to_last_path_component, write_error, write_warning, ProcessError};
use crate::xml::sumo_xml_definitions::{self as defs, Attrs, SumoXMLAttr, SumoXMLAttrMask, SumoXMLTag, Tags};
use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap};
use std::io::{self, BufWriter, Write};
use std::rc::Rc;

pub type OutputDeviceRef = Rc<RefCell<OutputDevice>>;

/// toString(val, precision) for values written with writeAttr / operator<<.
/// Implement it for further types (Position, RGBColor, ...) in their own modules.
pub trait OutputValue {
    fn to_output_string(&self, precision: i32) -> Cow<'_, str>;
}
impl OutputValue for str {
    fn to_output_string(&self, _: i32) -> Cow<'_, str> {
        Cow::Borrowed(self)
    }
}
impl OutputValue for String {
    fn to_output_string(&self, _: i32) -> Cow<'_, str> {
        Cow::Borrowed(self)
    }
}
impl<T: OutputValue + ?Sized> OutputValue for &T {
    fn to_output_string(&self, precision: i32) -> Cow<'_, str> {
        (**self).to_output_string(precision)
    }
}
impl OutputValue for f64 {
    fn to_output_string(&self, precision: i32) -> Cow<'_, str> {
        Cow::Owned(f64_to_string_prec(*self, precision))
    }
}
impl OutputValue for f32 {
    fn to_output_string(&self, precision: i32) -> Cow<'_, str> {
        Cow::Owned(f64_to_string_prec(*self as f64, precision))
    }
}
/// ostream << bool without boolalpha
impl OutputValue for bool {
    fn to_output_string(&self, _: i32) -> Cow<'_, str> {
        Cow::Borrowed(if *self { "1" } else { "0" })
    }
}
impl OutputValue for char {
    fn to_output_string(&self, _: i32) -> Cow<'_, str> {
        Cow::Owned(self.to_string())
    }
}
macro_rules! output_int {
    ($($t:ty),*) => {$(
        impl OutputValue for $t {
            fn to_output_string(&self, _: i32) -> Cow<'_, str> {
                Cow::Owned(self.to_string())
            }
        }
    )*};
}
output_int!(i8, i16, i32, i64, isize, u16, u32, u64, usize);
/// toString(std::vector) / joinToString(v, " ")
impl<T: OutputValue> OutputValue for [T] {
    fn to_output_string(&self, precision: i32) -> Cow<'_, str> {
        Cow::Owned(self.iter().map(|v| v.to_output_string(precision)).collect::<Vec<_>>().join(" "))
    }
}
impl<T: OutputValue> OutputValue for Vec<T> {
    fn to_output_string(&self, precision: i32) -> Cow<'_, str> {
        self.as_slice().to_output_string(precision)
    }
}
macro_rules! output_bijection {
    ($($t:ident => $bij:ident),*) => {$(
        impl OutputValue for defs::$t {
            fn to_output_string(&self, _: i32) -> Cow<'_, str> {
                Cow::Borrowed(defs::$bij.get_string(*self))
            }
        }
    )*};
}
// LinkState and LaneChangeAction are plain i32 here; convert them via LinkStates/LaneChangeActions
output_bijection!(SumoXMLNodeType => NodeTypes, SumoXMLEdgeFunc => EdgeFunctions, LaneSpreadFunction => LaneSpreadFunctions,
    ParkingType => ParkingTypes, RightOfWay => RightOfWayValues, FringeType => FringeTypeValues,
    RoundaboutType => RoundaboutTypeValues, PersonMode => PersonModeValues, LinkDirection => LinkDirections,
    TrafficLightType => TrafficLightTypes, TrafficLightLayout => TrafficLightLayouts, InsertionCheck => InsertionChecks,
    LaneChangeModel => LaneChangeModels);

/// An attribute given by its id (SumoXMLAttr) or by name.
pub trait AttrName {
    fn attr_name(&self) -> &str;
}
impl AttrName for SumoXMLAttr {
    fn attr_name(&self) -> &str {
        Attrs.get_string(*self)
    }
}
impl AttrName for &str {
    fn attr_name(&self) -> &str {
        self
    }
}
impl AttrName for String {
    fn attr_name(&self) -> &str {
        self
    }
}

enum Sink {
    /// OutputDevice_File (plain or gzip); None once closed
    File(Option<Box<dyn Write>>),
    String(Vec<u8>),
    Cout,
    Cerr,
}

/// the std::ostream: remembers write failures for ok()
struct Stream {
    sink: Sink,
    good: bool,
}

impl Write for Stream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let r = match &mut self.sink {
            Sink::File(Some(f)) => f.write_all(buf),
            Sink::File(None) => Err(io::Error::other("closed")),
            Sink::String(s) => s.write_all(buf),
            Sink::Cout => io::stdout().write_all(buf),
            Sink::Cerr => io::stderr().write_all(buf),
        };
        if r.is_err() {
            self.good = false;
        }
        r.map(|_| buf.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        match &mut self.sink {
            Sink::File(Some(f)) => f.flush(),
            Sink::Cout => io::stdout().flush(),
            Sink::Cerr => io::stderr().flush(),
            _ => Ok(()),
        }
    }
}

pub struct OutputDevice {
    my_filename: String,
    my_write_metadata: bool,
    my_formatter: PlainXMLFormatter,
    my_precision: i32,
    my_am_null: bool,
    my_stream: Stream,
}

thread_local! {
    static MY_OUTPUT_DEVICES: RefCell<BTreeMap<String, OutputDeviceRef>> = const { RefCell::new(BTreeMap::new()) };
}

/// C strerror texts for the common cases
fn strerror(e: &io::Error) -> String {
    match e.kind() {
        io::ErrorKind::NotFound => "No such file or directory".into(),
        io::ErrorKind::PermissionDenied => "Permission denied".into(),
        _ => e.to_string(),
    }
}

/// prefix/suffix "TIME" replacement: strftime("%Y-%m-%d-%H-%M-%S") of the load time
// shortcut: uses the current UTC time instead of OptionsIO::getLoadTime() in local time
fn time_stamp() -> String {
    iso_time_string()[..19].replace(['T', ':'], "-")
}

impl OutputDevice {
    fn with_sink(default_indentation: i32, filename: &str, sink: Sink) -> Self {
        OutputDevice {
            my_filename: filename.to_string(),
            my_write_metadata: false,
            my_formatter: PlainXMLFormatter::new(default_indentation),
            my_precision: g_precision(),
            my_am_null: false,
            my_stream: Stream { sink, good: true },
        }
    }

    /// OutputDevice_String
    pub fn new_string(default_indentation: i32) -> Self {
        Self::with_sink(default_indentation, "", Sink::String(Vec::new()))
    }

    /// OutputDevice_File; names ending in ".gz" are gzip-compressed
    pub fn new_file(full_name: &str) -> Result<Self, ProcessError> {
        let path = if full_name == "/dev/null" && cfg!(windows) { "NUL" } else { full_name };
        let file = std::fs::File::create(path).map_err(|e| {
            if full_name == "/dev/null" {
                ProcessError::IOError(format!("Could not redirect to NUL device ({}).", strerror(&e)))
            } else {
                ProcessError::IOError(format!("Could not build output file '{full_name}' ({}).", strerror(&e)))
            }
        })?;
        let w: Box<dyn Write> = if full_name.len() > 3 && full_name.ends_with(".gz") {
            Box::new(flate2::write::GzEncoder::new(BufWriter::new(file), flate2::Compression::default()))
        } else {
            Box::new(BufWriter::new(file))
        };
        let mut dev = Self::with_sink(0, full_name, Sink::File(Some(w)));
        dev.my_am_null = full_name == "/dev/null";
        Ok(dev)
    }

    pub fn get_device(name: &str, use_prefix: bool) -> Result<OutputDeviceRef, ProcessError> {
        // check whether the device has already been aqcuired
        if let Some(d) = MY_OUTPUT_DEVICES.with(|m| m.borrow().get(name).cloned()) {
            return Ok(d);
        }
        // build the device
        let oc = options();
        let format = |f: &str| oc.exists("output.format") && oc.get_string("output.format") == f;
        let len = name.len();
        let mut is_csv = format("csv") || (len > 4 && name.ends_with(".csv")) || (len > 7 && name.ends_with(".csv.gz"));
        let is_parquet = format("parquet") || (len > 8 && name.ends_with(".parquet"));
        if is_parquet {
            write_warning("Compiled without Parquet support, falling back to XML.");
        }
        let mut dev = if name == "stdout" {
            is_csv = false;
            Self::with_sink(0, "COUT", Sink::Cout)
        } else if name == "stderr" {
            is_csv = false;
            Self::with_sink(0, "CERR", Sink::Cerr)
        } else if is_socket(name) {
            // shortcut: OutputDevice_Network (socket output) is not ported
            return Err(ProcessError::IOError(format!("Network output '{name}' is not supported.")));
        } else {
            let mut name2 = if name == "nul" || name == "NUL" { "/dev/null".to_string() } else { name.to_string() };
            if use_prefix && oc.is_set("output-prefix") && name2 != "/dev/null" {
                let prefix = oc.get_string("output-prefix").replacen("TIME", &time_stamp(), 1);
                name2 = prepend_to_last_path_component(&prefix, name);
            }
            if use_prefix && oc.is_set("output-suffix") && name2 != "/dev/null" {
                let suffix = oc.get_string("output-suffix").replacen("TIME", &time_stamp(), 1);
                name2 = append_before_extension(&name2, &suffix);
            }
            Self::new_file(&name2)?
        };
        if is_csv {
            // shortcut: no CSVFormatter, csv output is written as XML
        }
        dev.set_precision(g_precision());
        dev.my_write_metadata = oc.exists("write-metadata") && oc.get_bool("write-metadata");
        let dev = Rc::new(RefCell::new(dev));
        MY_OUTPUT_DEVICES.with(|m| m.borrow_mut().insert(name.to_string(), dev.clone()));
        Ok(dev)
    }

    pub fn create_device_by_option(option_name: &str, root_element: &str, schema_file: &str, maximum_depth: i32) -> Result<bool, ProcessError> {
        let oc = options();
        if !oc.is_set(option_name) {
            return Ok(false);
        }
        let dev = Self::get_device(&oc.get_string(option_name), true)?;
        let mut dev = dev.borrow_mut();
        dev.set_expected_attributes(SumoXMLAttrMask::default(), maximum_depth);
        if !root_element.is_empty() {
            dev.write_xml_header(root_element, schema_file, BTreeMap::new(), true);
        }
        Ok(true)
    }

    pub fn get_device_by_option(option_name: &str) -> Result<OutputDeviceRef, ProcessError> {
        let dev_name = options().get_string(option_name);
        if !MY_OUTPUT_DEVICES.with(|m| m.borrow().contains_key(&dev_name)) {
            return Err(ProcessError::InvalidArgument(format!("Output device '{dev_name}' for option '{option_name}' has not been created.")));
        }
        Self::get_device(&dev_name, true)
    }

    pub fn flush_all() {
        for d in MY_OUTPUT_DEVICES.with(|m| m.borrow().values().cloned().collect::<Vec<_>>()) {
            d.borrow_mut().flush();
        }
    }

    pub fn close_all(_keep_error_retrievers: bool) {
        for d in MY_OUTPUT_DEVICES.with(|m| m.borrow().values().cloned().collect::<Vec<_>>()) {
            if let Err(e) = d.borrow_mut().close() {
                write_error("Error on closing output devices.");
                write_error(&e.to_string());
            }
        }
    }

    pub fn ok(&self) -> bool {
        self.my_stream.good
    }
    pub fn is_null(&self) -> bool {
        self.my_am_null
    }
    pub fn get_filename(&self) -> &str {
        &self.my_filename
    }

    /// Closes all open tags, finishes the file and removes the device from the registry.
    pub fn close(&mut self) -> Result<(), ProcessError> {
        while self.close_tag("") {}
        let me = self as *const OutputDevice;
        MY_OUTPUT_DEVICES.with(|m| {
            let mut m = m.borrow_mut();
            if let Some(k) = m.iter().find(|(_, d)| std::ptr::eq(d.as_ptr(), me)).map(|(k, _)| k.clone()) {
                m.remove(&k);
            }
        });
        let r = self.my_stream.flush();
        if let Sink::File(f) = &mut self.my_stream.sink {
            drop(f.take()); // GzEncoder writes its trailer on drop
        }
        r.map_err(|e| ProcessError::IOError(e.to_string()))
    }

    pub fn is_xml(&self) -> bool {
        self.get_formatter_type() == OutputFormatterType::XML
    }
    fn get_formatter_type(&self) -> OutputFormatterType {
        OutputFormatterType::XML
    }

    /// setPrecision(precision = gPrecision)
    pub fn set_precision(&mut self, precision: i32) {
        self.my_precision = precision;
    }
    pub fn get_precision(&self) -> i32 {
        self.my_precision
    }

    pub fn write_xml_header(&mut self, root_element: &str, schema_file: &str, mut attrs: BTreeMap<SumoXMLAttr, String>, include_config: bool) -> bool {
        if !schema_file.is_empty() {
            attrs.insert(defs::SUMO_ATTR_XMLNS, "http://www.w3.org/2001/XMLSchema-instance".into());
            attrs.insert(defs::SUMO_ATTR_SCHEMA_LOCATION, format!("http://sumo.dlr.de/xsd/{schema_file}"));
        }
        self.my_formatter.write_xml_header(&mut self.my_stream, root_element, &attrs, self.my_write_metadata, include_config)
    }

    pub fn open_tag(&mut self, xml_element: &str) -> &mut Self {
        self.my_formatter.open_tag(&mut self.my_stream, xml_element);
        self
    }
    /// openTag(const SumoXMLTag&)
    pub fn open_tag_id(&mut self, xml_element: SumoXMLTag) -> &mut Self {
        self.my_formatter.open_tag(&mut self.my_stream, Tags.get_string(xml_element));
        self
    }

    pub fn close_tag(&mut self, comment: &str) -> bool {
        if self.my_formatter.close_tag(&mut self.my_stream, comment) {
            self.post_write_hook();
            return true;
        }
        false
    }

    pub fn lf(&mut self) {
        let _ = self.my_stream.write_all(b"\n");
    }

    pub fn write_attr<A: AttrName, T: OutputValue + ?Sized>(&mut self, attr: A, val: &T) -> &mut Self {
        PlainXMLFormatter::write_attr(&mut self.my_stream, attr.attr_name(), &val.to_output_string(self.my_precision));
        self
    }
    /// writeAttr(attr, std::string, isNull = false, escape)
    pub fn write_attr_escaped<A: AttrName>(&mut self, attr: A, val: &str, escape: bool) -> &mut Self {
        let v = if escape { Cow::Owned(escape_xml(val)) } else { Cow::Borrowed(val) };
        PlainXMLFormatter::write_attr(&mut self.my_stream, attr.attr_name(), &v);
        self
    }

    pub fn parse_written_attributes(attr_list: &[String], desc: &str, special: &HashMap<String, SumoXMLAttrMask>) -> SumoXMLAttrMask {
        let mut result = SumoXMLAttrMask::default();
        for attr_name in attr_list {
            if attr_name == "all" {
                result.set_all();
            } else if let Some(m) = special.get(attr_name) {
                result.0 |= m.0;
            } else if let Ok(attr_nr) = Attrs.get(attr_name) {
                if attr_nr < SumoXMLAttrMask::SIZE {
                    result.set(attr_nr);
                } else {
                    write_error(&format!("Attribute '{attr_name}' is not supported for filtering written attributes in {desc}."));
                }
            } else {
                write_error(&format!("Unknown attribute '{attr_name}' to write in {desc}."));
            }
        }
        result
    }

    pub fn write_optional_attr<T: OutputValue + ?Sized>(&mut self, attr: SumoXMLAttr, val: &T, is_null: bool) -> &mut Self {
        if is_null {
            return self;
        }
        self.write_attr(attr, val)
    }

    pub fn write_optional_attr_mask<T: OutputValue + ?Sized>(&mut self, attr: SumoXMLAttr, val: &T, attribute_mask: SumoXMLAttrMask, is_null: bool) -> &mut Self {
        debug_assert!(attribute_mask.none() || attr <= SumoXMLAttrMask::SIZE);
        if attribute_mask.none() || attribute_mask.test(attr) {
            return self.write_optional_attr(attr, val, is_null);
        }
        self
    }

    pub fn write_func_attr<T: OutputValue, F: FnOnce() -> T>(&mut self, attr: SumoXMLAttr, val_func: F, attribute_mask: SumoXMLAttrMask, is_null: bool) -> &mut Self {
        debug_assert!(attr <= SumoXMLAttrMask::SIZE);
        if (attribute_mask.none() || attribute_mask.test(attr)) && !is_null {
            self.write_attr(attr, &val_func());
        }
        self
    }

    pub fn write_non_empty_attr(&mut self, attr: SumoXMLAttr, val: &str) -> &mut Self {
        if !val.is_empty() && val != "default" {
            self.write_attr(attr, val);
        }
        self
    }

    pub fn write_time(&mut self, attr: SumoXMLAttr, val: SUMOTime) -> &mut Self {
        self.my_formatter.write_time(&mut self.my_stream, attr, val);
        self
    }

    pub fn write_preformatted_tag(&mut self, val: &str) -> &mut Self {
        self.my_formatter.write_preformatted_tag(&mut self.my_stream, val);
        self
    }

    pub fn write_padding(&mut self, val: &str) -> &mut Self {
        self.my_formatter.write_padding(&mut self.my_stream, val);
        self
    }

    pub fn inform(&mut self, msg: &str, progress: bool) {
        let _ = self.my_stream.write_all(msg.as_bytes());
        if !progress {
            let _ = self.my_stream.write_all(b"\n");
        }
        self.post_write_hook();
    }

    /// operator<<
    pub fn write<T: OutputValue + ?Sized>(&mut self, t: &T) -> &mut Self {
        let _ = self.my_stream.write_all(t.to_output_string(self.my_precision).as_bytes());
        self.post_write_hook();
        self
    }

    pub fn flush(&mut self) {
        let _ = self.my_stream.flush();
    }

    pub fn wrote_header(&self) -> bool {
        self.my_formatter.wrote_header()
    }

    /// only used by the CSV formatter
    pub fn set_expected_attributes(&mut self, _expected: SumoXMLAttrMask, _depth: i32) {}

    /// OutputDevice_String::getString
    pub fn get_string(&self) -> String {
        match &self.my_stream.sink {
            Sink::String(s) => String::from_utf8_lossy(s).into_owned(),
            _ => String::new(),
        }
    }

    /// OutputDevice_COUT/CERR flush after every write
    fn post_write_hook(&mut self) {
        if matches!(self.my_stream.sink, Sink::Cout | Sink::Cerr) {
            let _ = self.my_stream.flush();
        }
    }
}
