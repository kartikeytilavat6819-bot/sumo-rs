//! Port of utils/common/SystemFrame.{h,cpp}
use super::msg_handler::{MsgHandler, MsgType, with_instance};
use crate::iodevices::output_device::OutputDevice;
use crate::options::option::SumoOption;
use crate::options::options_cont::{OptionsCont, with_options};
use crate::xml::xml_sub_sys;

// shortcut: gLanguage is not a ported global (only the GUI changes it), so the C default "C" is used, upgrade when the language is settable
const G_LANGUAGE: &str = "C";

/// C++: `void SystemFrame::addConfigurationOptions(OptionsCont& oc)`
pub fn add_configuration_options(oc: &mut OptionsCont) {
    oc.add_option_sub_topic("Configuration");

    oc.do_register_abbr("configuration-file", 'c', SumoOption::file_name());
    oc.add_synonyme("configuration-file", "configuration", false);
    oc.add_description("configuration-file", "Configuration", "Loads the named config on startup");
    oc.add_xml_default("configuration-file", "");

    oc.do_register_abbr("save-configuration", 'C', SumoOption::file_name());
    oc.add_synonyme("save-config", "save-configuration", false);
    oc.add_description("save-configuration", "Configuration", "Saves current configuration into FILE");

    oc.do_register("save-configuration.relative", SumoOption::bool(false));
    oc.add_synonyme("save-config.relative", "save-configuration.relative", false);
    oc.add_description("save-configuration.relative", "Configuration", "Enforce relative paths when saving the configuration");

    oc.do_register("save-template", SumoOption::file_name());
    oc.add_description("save-template", "Configuration", "Saves a configuration template (empty) into FILE");

    oc.do_register("save-schema", SumoOption::file_name());
    oc.add_description("save-schema", "Configuration", "Saves the configuration schema into FILE");

    oc.do_register("save-commented", SumoOption::bool(false));
    oc.add_synonyme("save-commented", "save-template.commented", false);
    oc.add_description("save-commented", "Configuration", "Adds comments to saved template, configuration, or schema");
}

/// C++: `void SystemFrame::addReportOptions(OptionsCont& oc)`
pub fn add_report_options(oc: &mut OptionsCont) {
    oc.add_option_sub_topic("Report");

    oc.do_register_abbr("verbose", 'v', SumoOption::bool(false));
    oc.add_description("verbose", "Report", "Switches to verbose output");

    oc.do_register("print-options", SumoOption::bool(false));
    oc.add_description("print-options", "Report", "Prints option values before processing");

    oc.do_register_abbr("help", '?', SumoOption::bool_extended(false));
    oc.add_description("help", "Report", "Prints this screen or selected topics");

    oc.do_register_abbr("version", 'V', SumoOption::bool(false));
    oc.add_description("version", "Report", "Prints the current version");

    oc.do_register_abbr("xml-validation", 'X', SumoOption::string_with("local"));
    oc.add_description("xml-validation", "Report", "Set schema validation scheme of XML inputs (\"never\", \"local\", \"auto\" or \"always\")");

    if oc.exists("net-file") {
        oc.do_register("xml-validation.net", SumoOption::string_with("never"));
        oc.add_description("xml-validation.net", "Report", "Set schema validation scheme of SUMO network inputs (\"never\", \"local\", \"auto\" or \"always\")");
    }

    if oc.exists("route-files") {
        oc.do_register("xml-validation.routes", SumoOption::string_with("local"));
        oc.add_description("xml-validation.routes", "Report", "Set schema validation scheme of SUMO route inputs (\"never\", \"local\", \"auto\" or \"always\")");
    }

    oc.do_register_abbr("no-warnings", 'W', SumoOption::bool(false));
    oc.add_synonyme("no-warnings", "suppress-warnings", true);
    oc.add_description("no-warnings", "Report", "Disables output of warnings");

    oc.do_register("aggregate-warnings", SumoOption::integer(-1));
    oc.add_description("aggregate-warnings", "Report", "Aggregate warnings of the same type whenever more than INT occur");

    oc.do_register_abbr("log", 'l', SumoOption::file_name());
    oc.add_synonyme("log", "log-file", false);
    oc.add_description("log", "Report", "Writes all messages to FILE (implies verbose)");

    oc.do_register("message-log", SumoOption::file_name());
    oc.add_description("message-log", "Report", "Writes all non-error messages to FILE (implies verbose)");

    oc.do_register("error-log", SumoOption::file_name());
    oc.add_description("error-log", "Report", "Writes all warnings and errors to FILE");

    oc.do_register("log.timestamps", SumoOption::bool(false));
    oc.add_description("log.timestamps", "Report", "Writes timestamps in front of all messages");

    oc.do_register("log.processid", SumoOption::bool(false));
    oc.add_description("log.processid", "Report", "Writes process ID in front of all messages");

    oc.do_register("language", SumoOption::string_with(G_LANGUAGE));
    oc.add_description("language", "Report", "Language to use in messages");

    oc.do_register("write-license", SumoOption::bool(false));
    oc.add_description("write-license", "Output", "Include license info into every output file");

    oc.do_register("write-metadata", SumoOption::bool(false));
    oc.add_description("write-metadata", "Output", "Write parsable metadata (configuration etc.) instead of comments");

    oc.do_register("output-prefix", SumoOption::string());
    oc.add_description("output-prefix", "Output", "Prefix which is applied to all output files. The special string 'TIME' is replaced by the current time.");

    oc.do_register("output-suffix", SumoOption::string());
    oc.add_description("output-suffix", "Output", "Suffix which is applied to all output files. The special string 'TIME' is replaced by the current time.");

    oc.do_register("precision", SumoOption::integer(2));
    oc.add_description("precision", "Output", "Defines the number of digits after the comma for floating point output");

    oc.do_register("precision.geo", SumoOption::integer(6));
    oc.add_description("precision.geo", "Output", "Defines the number of digits after the comma for lon,lat output");

    oc.do_register("output.compression", SumoOption::string());
    oc.add_description("output.compression", "Output", "Defines the standard compression algorithm (currently only for parquet output)");

    oc.do_register("output.format", SumoOption::string_with("xml"));
    oc.add_description("output.format", "Output", "Defines the standard output format if not derivable from the file name ('xml', 'csv', 'parquet')");

    oc.do_register("output.column-header", SumoOption::string_with("auto"));
    oc.add_description("output.column-header", "Output", "How to derive column headers from attribute names ('none', 'tag', 'auto', 'plain', 'full')");

    oc.do_register("output.column-separator", SumoOption::string_with(";"));
    oc.add_description("output.column-separator", "Output", "Separator in CSV output");

    oc.do_register_abbr("human-readable-time", 'H', SumoOption::bool(false));
    oc.add_description("human-readable-time", "Output", "Write time values as hour:minute:second or day:hour:minute:second rather than seconds");
}

/// C++: `void SystemFrame::close()`
pub fn close() {
    // flush aggregated warnings
    with_instance(MsgType::Warning, |h| h.clear(true));
    // close all output devices
    OutputDevice::close_all(false);
    // close the xml-subsystem
    xml_sub_sys::close();
    // delete build program options
    with_options(|oc| oc.clear());
    // delete messages
    MsgHandler::cleanup_on_end();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn report_options_register_under_report_topic() {
        let mut oc = OptionsCont::new();
        oc.add_option_sub_topic("Output");
        add_report_options(&mut oc);
        assert!(oc.exists("verbose"));
        assert_eq!(oc.get_int("precision").unwrap(), 2);
        assert_eq!(oc.get_int("precision.geo").unwrap(), 6);
    }
}
