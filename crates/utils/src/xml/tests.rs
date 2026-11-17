//! Tests of the SAX driver (GenericSAXHandler + SUMOSAXReader + XMLSubSys)
use super::generic_sax_handler::{GenericSAXHandler, SaxHandler};
use super::stand_ins::ProcessError;
use super::sumo_sax_attributes::SUMOSAXAttributes;
use super::sumo_sax_handler;
use super::sumo_sax_reader::SUMOSAXReader;
use super::sumo_xml_definitions::*;
use super::xml_sub_sys;
use std::cell::RefCell;
use std::io::Write;
use std::rc::Rc;

struct Recorder {
    base: GenericSAXHandler,
    log: Vec<String>,
}
impl Recorder {
    fn new() -> Self {
        Recorder { base: sumo_sax_handler::new("", ""), log: Vec::new() }
    }
}
impl SaxHandler for Recorder {
    fn handler(&mut self) -> &mut GenericSAXHandler {
        &mut self.base
    }
    fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) -> Result<(), ProcessError> {
        self.log.push(format!("<{}{}", Tags.get_string(element), attrs));
        Ok(())
    }
    fn my_characters(&mut self, element: i32, chars: &str) -> Result<(), ProcessError> {
        self.log.push(format!("{}:{chars:?}", Tags.get_string(element)));
        Ok(())
    }
    fn my_end_element(&mut self, element: i32) -> Result<(), ProcessError> {
        self.log.push(format!("/{}", Tags.get_string(element)));
        Ok(())
    }
}

fn tmp(name: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("sumo_rs_xml_{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    dir.join(name)
}

const ROUTES: &str = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<!-- c -->\n<routes>\n    <vehicle id=\"v&amp;0\" depart=\"0\"\n route=\"r\">\n        <stop lane=\"a_0\"/>\n    </vehicle>\n    <foo x=\"&#65;\"/>\n</routes>\n";

#[test]
fn nested_elements_and_attributes() {
    let mut h = Recorder::new();
    SUMOSAXReader::new("never").parse_string(&mut h, ROUTES).unwrap();
    assert_eq!(h.log, vec!["<routes", "<vehicle id=\"v&0\" depart=\"0\" route=\"r\"", "<stop lane=\"a_0\"", "/stop", "/vehicle", "< x=\"A\"", "/", "/routes"]);
}

#[test]
fn characters() {
    let mut h = Recorder::new();
    h.base.needs_character_data(true);
    SUMOSAXReader::new("never").parse_string(&mut h, "<net>a &lt; b<edge/><![CDATA[x<y]]></net>").unwrap();
    // the vector is cleared by <edge>, so </net> sees only the trailing text
    assert_eq!(h.log, vec!["<net", "<edge", "/edge", "net:\"x<y\"", "/net"]);
}

#[test]
fn gz_and_include() {
    let inner = tmp("inner.xml");
    std::fs::write(&inner, "<net><busStop id=\"b\"/></net>").unwrap();
    let outer = tmp("outer.xml.gz");
    let mut enc = flate2::write::GzEncoder::new(std::fs::File::create(&outer).unwrap(), flate2::Compression::default());
    enc.write_all(b"<routes><include href=\"inner.xml\"/><vType id=\"t\"/></routes>").unwrap();
    enc.finish().unwrap();
    let mut h = Recorder::new();
    assert!(xml_sub_sys::run_parser(&mut h, outer.to_str().unwrap(), false, false, false, false).unwrap());
    assert_eq!(h.log, vec!["<routes", "<net", "<busStop id=\"b\"", "/busStop", "/net", "<vType id=\"t\"", "/vType", "/routes"]);
    assert_eq!(h.base.get_file_name(), "");
}

#[test]
fn errors() {
    let mut h = Recorder::new();
    let e = SUMOSAXReader::new("never").parse_string(&mut h, "<a>\n<b></a>").unwrap_err().to_string();
    assert!(e.starts_with("expected end of tag 'b'\n"), "{e}");
    assert!(e.ends_with(" In file ''\n At line/column 3/6.\n"), "{e}");
    let e = SUMOSAXReader::new("never").parse_string(&mut h, "<a>").unwrap_err();
    assert!(matches!(e, ProcessError::Process(_)));
    let e = SUMOSAXReader::new("never").parse(&mut h, "does/not/exist.xml").unwrap_err();
    assert_eq!(e, ProcessError::IOError("Cannot read file 'does/not/exist.xml'!".into()));
    let e = xml_sub_sys::run_parser(&mut h, "does/not/exist.xml", false, false, false, true).unwrap();
    assert!(!e);
}

#[test]
fn progressive_and_sections() {
    let f = tmp("sections.xml");
    std::fs::write(&f, "<net><node id=\"1\"/><node id=\"2\"/><way id=\"3\"><nd/></way><relation id=\"4\"><nd/></relation></net>").unwrap();
    let mut h = Recorder::new();
    let mut r = SUMOSAXReader::new("never");
    assert!(r.parse_first(&mut h, f.to_str().unwrap()).unwrap());
    assert!(h.log.is_empty());
    assert!(r.parse_section(&mut h, SUMO_TAG_NODE).unwrap());
    assert_eq!(h.log, vec!["<net", "<node id=\"1\"", "/node", "<node id=\"2\"", "/node"]);
    h.log.clear();
    assert!(r.parse_section(&mut h, SUMO_TAG_WAY).unwrap());
    assert_eq!(h.log, vec!["<way id=\"3\"", "<nd", "/nd", "/way"]);
    h.log.clear();
    assert!(!r.parse_section(&mut h, SUMO_TAG_RELATION).unwrap());
    assert_eq!(h.log, vec!["<relation id=\"4\"", "<nd", "/nd", "/relation", "/net"]);
    let mut r2 = SUMOSAXReader::new("never");
    let mut h2 = Recorder::new();
    r2.parse_first(&mut h2, f.to_str().unwrap()).unwrap();
    assert!(r2.parse_next(&mut h2).unwrap());
    assert_eq!(h2.log, vec!["<net"]);
    while r2.parse_next(&mut h2).unwrap() {}
    assert_eq!(h2.log.len(), 14);
}

/// a parent handing <calibrator> over to a child, which calls back the parent end (MSCalibrator)
struct Parent {
    base: GenericSAXHandler,
    child: Rc<RefCell<Recorder>>,
    log: Vec<String>,
}
impl SaxHandler for Parent {
    fn handler(&mut self) -> &mut GenericSAXHandler {
        &mut self.base
    }
    fn my_start_element(&mut self, element: i32, _attrs: &SUMOSAXAttributes) -> Result<(), ProcessError> {
        self.log.push(format!("<{}", Tags.get_string(element)));
        if element == SUMO_TAG_CALIBRATOR {
            self.base.register_child(SUMO_TAG_CALIBRATOR, self.child.clone());
        }
        Ok(())
    }
    fn my_end_element(&mut self, element: i32) -> Result<(), ProcessError> {
        self.log.push(format!("/{}", Tags.get_string(element)));
        Ok(())
    }
}
struct Child(Recorder);
impl SaxHandler for Child {
    fn handler(&mut self) -> &mut GenericSAXHandler {
        &mut self.0.base
    }
    fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) -> Result<(), ProcessError> {
        self.0.my_start_element(element, attrs)
    }
    fn my_end_element(&mut self, element: i32) -> Result<(), ProcessError> {
        self.0.my_end_element(element)?;
        self.0.base.call_parent_end(element);
        Ok(())
    }
}

#[test]
fn register_child() {
    let child = Rc::new(RefCell::new(Child(Recorder::new())));
    let rec = Rc::new(RefCell::new(Recorder::new()));
    let mut p = Parent { base: sumo_sax_handler::new("", ""), child: rec.clone(), log: Vec::new() };
    SUMOSAXReader::new("never").parse_string(&mut p, "<net><calibrator id=\"c\"><flow/></calibrator><e1Detector/></net>").unwrap();
    assert_eq!(p.log, vec!["<net", "<calibrator", "<e1Detector", "/e1Detector", "/net"]);
    assert_eq!(rec.borrow().log, vec!["<flow", "/flow", "/calibrator"]);
    // with callParentEnd
    let mut p2 = Parent { base: sumo_sax_handler::new("", ""), child: rec, log: Vec::new() };
    p2.base.register_child(SUMO_TAG_CALIBRATOR, child.clone());
    SUMOSAXReader::new("never").parse_string(&mut p2, "<calibrator/>").unwrap();
    assert_eq!(child.borrow().0.log, vec!["<calibrator", "/calibrator"]);
    assert_eq!(p2.log, vec!["/calibrator"]);
}
