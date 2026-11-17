//! Port of utils/iodevices
pub mod options_hook;
pub mod output_device;
pub mod output_formatter;
pub mod plain_xml_formatter;
pub mod zstr;

#[cfg(test)]
mod tests {
    use super::options_hook::{set_options_hook, OptionsHook};
    use super::output_device::OutputDevice;
    use crate::xml::sumo_xml_definitions::*;
    use std::collections::{BTreeMap, HashMap};
    use std::io::{Read, Write};
    use std::sync::Arc;

    #[test]
    fn plain_xml_formatter() {
        let mut dev = OutputDevice::new_string(0);
        dev.open_tag("a").write_attr("x", &1.5).write_attr("n", &3).write_attr("b", &true);
        dev.open_tag_id(SUMO_TAG_EDGE).write_attr(SUMO_ATTR_ID, "e").write_attr(SUMO_ATTR_SPEED, &13.888);
        assert!(dev.close_tag(""));
        dev.open_tag("b");
        dev.close_tag(" <!-- c -->");
        dev.open_tag("c").write_time(SUMO_ATTR_BEGIN, 1500).write_attr_escaped(SUMO_ATTR_NAME, "<&\">", true);
        dev.write_preformatted_tag("        <pre/>\n");
        dev.close_tag("");
        dev.set_precision(4);
        dev.open_tag("d").write_attr("v", &vec![1.0, 2.5]).write_optional_attr(SUMO_ATTR_X, &1.0, true).write_non_empty_attr(SUMO_ATTR_TYPE, "default");
        assert!(dev.close_tag(""));
        assert!(dev.close_tag(""));
        assert!(!dev.close_tag(""));
        assert_eq!(
            dev.get_string(),
            "<a x=\"1.50\" n=\"3\" b=\"1\">\n    <edge id=\"e\" speed=\"13.89\"/>\n    <b/> <!-- c -->\n    <c begin=\"1.50\" name=\"&lt;&amp;&quot;&gt;\">\n        <pre/>\n    </c>\n    <d v=\"1.0000 2.5000\"/>\n</a>\n"
        );
        let mut dev = OutputDevice::new_string(1);
        dev.open_tag("a").close_tag("");
        assert_eq!(dev.get_string(), "    <a/>\n");
    }

    struct FakeOptions;
    impl OptionsHook for FakeOptions {
        fn exists(&self, _: &str) -> bool {
            false
        }
        fn is_set(&self, _: &str) -> bool {
            false
        }
        fn get_string(&self, _: &str) -> String {
            String::new()
        }
        fn get_bool(&self, _: &str) -> bool {
            false
        }
        fn get_full_name(&self) -> String {
            "Eclipse SUMO sumo Version 1.0".into()
        }
        fn write_configuration(&self, into: &mut dyn Write, indent: &str) {
            let _ = writeln!(into, "{indent}<configuration/>");
        }
    }

    #[test]
    fn xml_header_and_file_device() {
        set_options_hook(Arc::new(FakeOptions));
        let path = std::env::temp_dir().join(format!("sumo_rs_out_{}.xml.gz", std::process::id()));
        let name = path.to_str().unwrap();
        let dev = OutputDevice::get_device(name, true).unwrap();
        assert!(std::rc::Rc::ptr_eq(&dev, &OutputDevice::get_device(name, true).unwrap()));
        assert!(dev.borrow_mut().write_xml_header("routes", "routes_file.xsd", BTreeMap::new(), true));
        assert!(!dev.borrow_mut().write_xml_header("routes", "", BTreeMap::new(), true));
        dev.borrow_mut().open_tag("vehicle").write_attr(SUMO_ATTR_DEPART, &0.0);
        OutputDevice::close_all(false);
        let mut text = String::new();
        flate2::read::GzDecoder::new(std::fs::File::open(&path).unwrap()).read_to_string(&mut text).unwrap();
        let (head, body) = text.split_once(" by ").unwrap();
        assert!(head.starts_with("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\n<!-- generated on 2"), "{head}");
        assert_eq!(
            body,
            "Eclipse SUMO sumo Version 1.0\n<configuration/>\n-->\n\n<routes xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" xsi:noNamespaceSchemaLocation=\"http://sumo.dlr.de/xsd/routes_file.xsd\">\n    <vehicle depart=\"0.00\"/>\n</routes>\n"
        );
        let e = OutputDevice::new_file("does/not/exist/x.xml").err().unwrap();
        assert_eq!(e.to_string(), "Could not build output file 'does/not/exist/x.xml' (No such file or directory).");
    }

    #[test]
    fn written_attributes() {
        let m = OutputDevice::parse_written_attributes(&["speed".to_string(), "waitingTime".to_string()], "test", &HashMap::new());
        assert!(m.test(SUMO_ATTR_SPEED) && !m.test(SUMO_ATTR_DENSITY) && !m.none());
        let mut dev = OutputDevice::new_string(0);
        dev.open_tag("e").write_optional_attr_mask(SUMO_ATTR_SPEED, &1.0, m, false).write_optional_attr_mask(SUMO_ATTR_DENSITY, &1.0, m, false);
        dev.write_func_attr(SUMO_ATTR_SPEED, || 2, m, false).close_tag("");
        assert_eq!(dev.get_string(), "<e speed=\"1.00\" speed=\"2\"/>\n");
    }
}
