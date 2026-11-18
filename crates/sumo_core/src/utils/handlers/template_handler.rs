//! Port of utils/handlers/TemplateHandler.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::msg_handler::{write_error, write_warning};
use utils::common::std_defs::{INVALID_DOUBLE, INVALID_INT};
use utils::common::string_utils::to_bool;
use utils::common::to_string::to_string;

/// A SAX-Handler for loading options
/// C++ class `TemplateHandler` (utils/handlers/TemplateHandler.h)
#[repr(C)]
pub struct TemplateHandler {
    /// C++ base class `xercesc_3_2::HandlerBase`
    pub base: Unmapped /* xercesc_3_2::HandlerBase */,
    /// The information whether an error occurred
    pub myError: bool,
    /// The nesting level of parsed elements
    pub myLevel: i32,
    /// The options to fill
    pub myOptions: Ptr<OptionsCont>,
    /// The name of the current option
    pub myOptionName: String,
    /// current subtopic
    pub mySubTopic: String,
}

impl std::ops::Deref for TemplateHandler {
    type Target = Unmapped /* xercesc_3_2::HandlerBase */;
    fn deref(&self) -> &Unmapped /* xercesc_3_2::HandlerBase */ {
        &self.base
    }
}
impl std::ops::DerefMut for TemplateHandler {
    fn deref_mut(&mut self) -> &mut Unmapped /* xercesc_3_2::HandlerBase */ {
        &mut self.base
    }
}

// C++ static members of TemplateHandler (port as module-level statics, see AGENTS.md):
//   static const std::string TemplateHandler::INVALID_INT_STR;
//   static const std::string TemplateHandler::INVALID_DOUBLE_STR;

impl TemplateHandler {
    /// C++: `void TemplateHandler::parseTemplate(OptionsCont &, const std::string &)` [static]
    /// run parser
    // void
    // TemplateHandler::parseTemplate(OptionsCont& options, const std::string& templateString) {
    //     // build parser
    //     XERCES_CPP_NAMESPACE::SAXParser parser;
    //     // disable validation
    //     parser.setValidationScheme(XERCES_CPP_NAMESPACE::SAXParser::Val_Never);
    //     parser.setDisableDefaultEntityResolution(true);
    //     // build TemplateHandler
    //     TemplateHandler handler(options);
    //     // start parsing
    //     try {
    //         parser.setDocumentHandler(&handler);
    //         parser.setErrorHandler(&handler);
    //         XERCES_CPP_NAMESPACE::MemBufInputSource memBufIS((const XMLByte*)templateString.c_str(), templateString.size(), "template");
    //         parser.parse(memBufIS);
    //         if (handler.myError) {
    //             throw ProcessError(TLF("Could not load template '%'.", templateString));
    //         }
    //     } catch (const XERCES_CPP_NAMESPACE::XMLException& e) {
    //         throw ProcessError("Could not load template '" + templateString + "':\n " + XMLSubSys::transcode(e.getMessage()));
    //     }
    //     // mark al loaded options as default
    //     options.resetDefault();
    // }
    #[allow(unused_variables)]
    pub fn parse_template(options: &mut OptionsCont, templateString: &str) {
        // shortcut: TemplateHandler::parseTemplate is called only from netedit (GUI), which is not ported; upgrade when netedit is ported (the SAX entry point utils::xml::SUMOSAXReader::parse_string exists)
        panic!("TemplateHandler::parseTemplate: cannot parse template '{}' yet", templateString);
    }

    /// C++: `void TemplateHandler::TemplateHandler(OptionsCont &)` [private]
    /// Constructor
    // TemplateHandler::TemplateHandler(OptionsCont& options) :
    //     myError(false),
    //     myOptions(options) {
    // }
    #[allow(unused_variables)]
    pub fn new(options: &mut OptionsCont) -> Self {
        TemplateHandler {
            base: (),
            myError: false,
            myLevel: 0,
            myOptions: Ptr::from_mut(options),
            myOptionName: String::new(),
            mySubTopic: String::new(),
        }
    }

    /// C++: `void TemplateHandler::~TemplateHandler() noexcept` [virtual, private]
    /// destructor
    // TemplateHandler::~TemplateHandler() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // destructor: nothing to release explicitly, the fields drop with the struct
    }

    /// C++: `void TemplateHandler::startElement(const XMLCh *const, xercesc_3_2::AttributeList &)` [virtual, private]
    /// Called on the occurrence of the beginning of a tag
    // void
    // TemplateHandler::startElement(const XMLCh* const name, XERCES_CPP_NAMESPACE::AttributeList& attributes) {
    //     // get current topic
    //     myOptionName = XMLSubSys::transcode(name);
    //     if (myLevel++ == 0) {
    //         // skip root elemnt
    //         return;
    //     }
    //     // check if this is a subtopic
    //     if (attributes.getLength() == 0) {
    //         mySubTopic = myOptionName;
    //         myOptions.addOptionSubTopic(mySubTopic);
    //     } else {
    //         // declare options parameters (by default all empty)
    //         std::string value;
    //         std::string synonymes;
    //         std::string type;
    //         std::string help;
    //         bool required = false;
    //         bool positional = false;
    //         std::string listSep = "";
    //         // iterate over attributes
    //         for (int i = 0; i < (int)attributes.getLength(); i++) {
    //             const std::string attributeName = XMLSubSys::transcode(attributes.getName(i));
    //             const std::string attributeValue = XMLSubSys::transcode(attributes.getValue(i));
    //             // check attribute name
    //             if (attributeName == "value") {
    //                 value = attributeValue;
    //             } else if (attributeName == "synonymes") {
    //                 synonymes = attributeValue;
    //             } else if (attributeName == "type") {
    //                 type = attributeValue;
    //             } else if (attributeName == "help") {
    //                 help = attributeValue;
    //             } else if (attributeName == "category") {
    //                 // tool templates have subtopic as attribute category
    //                 mySubTopic = attributeValue;
    //                 const auto& topics = myOptions.getSubTopics();
    //                 if (std::find(topics.begin(), topics.end(), attributeValue) == topics.end()) {
    //                     myOptions.addOptionSubTopic(attributeValue);
    //                 }
    //             } else if (attributeName == "required") {
    //                 required = StringUtils::toBool(attributeValue);
    //             } else if (attributeName == "positional") {
    //                 positional = StringUtils::toBool(attributeValue);
    //             } else if (attributeName == "listSeparator") {
    //                 listSep = attributeValue;
    //             }
    //         }
    //         // add option
    //         addOption(value, synonymes, type, help, required, positional, listSep);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn start_element(&mut self, name: &str, attributes: &[(String, String)]) {
        // get current topic
        self.myOptionName = name.to_string();
        let level = self.myLevel;
        self.myLevel += 1;
        if level == 0 {
            // skip root elemnt
            return;
        }
        // check if this is a subtopic
        if attributes.is_empty() {
            self.mySubTopic = self.myOptionName.clone();
            self.myOptions.m().add_option_sub_topic(&self.mySubTopic);
        } else {
            // declare options parameters (by default all empty)
            let mut value = String::new();
            let mut synonymes = String::new();
            let mut type_ = String::new();
            let mut help = String::new();
            let mut required = false;
            let mut positional = false;
            let mut listSep = String::new();
            // iterate over attributes
            for (attributeName, attributeValue) in attributes {
                // check attribute name
                if attributeName == "value" {
                    value = attributeValue.clone();
                } else if attributeName == "synonymes" {
                    synonymes = attributeValue.clone();
                } else if attributeName == "type" {
                    type_ = attributeValue.clone();
                } else if attributeName == "help" {
                    help = attributeValue.clone();
                } else if attributeName == "category" {
                    // tool templates have subtopic as attribute category
                    self.mySubTopic = attributeValue.clone();
                    if !self.myOptions.get_sub_topics().contains(attributeValue) {
                        self.myOptions.m().add_option_sub_topic(attributeValue);
                    }
                } else if attributeName == "required" {
                    required = to_bool(attributeValue).unwrap_or_else(|e| panic!("{}", e));
                } else if attributeName == "positional" {
                    positional = to_bool(attributeValue).unwrap_or_else(|e| panic!("{}", e));
                } else if attributeName == "listSeparator" {
                    listSep = attributeValue.clone();
                }
            }
            // add option
            self.add_option(value, &synonymes, &type_, &help, required, positional, &listSep);
        }
    }

    /// C++: `bool TemplateHandler::addOption(std::string, const std::string &, const std::string &, const std::string &, bool, bool, const std::string &) const` [const, private]
    /// add option
    // bool
    // TemplateHandler::addOption(std::string value, const std::string& synonymes, const std::string& type,
    //                            const std::string& help, bool required, bool positional, const std::string& listSep) const {
    //     if (myOptions.exists(myOptionName)) {
    //         WRITE_WARNING(myOptionName + " already exists");
    //         return false;
    //     } else {
    //         // declare option
    //         Option* option = nullptr;
    //         // handle "None" as empty
    //         if (value == "None") {
    //             value.clear();
    //         }
    //         // create register depending of type
    //         if ((type == "STR") || (type == "string")) {
    //             option = new Option_String(value);
    //         } else if ((type == "TIME") || (type == "time")) {
    //             option = new Option_String(value, "TIME");
    //         } else if ((type == "INT") || (type == "int")) {
    //             option = new Option_Integer(0);
    //             if (value.empty()) {
    //                 option->set(INVALID_INT_STR, "", true);
    //             } else {
    //                 option->set(value, value, true);
    //             }
    //         } else if ((type == "FLOAT") || (type == "float")) {
    //             option = new Option_Float(0);
    //             if (value.empty()) {
    //                 option->set(INVALID_DOUBLE_STR, "", true);
    //             } else {
    //                 option->set(value, value, true);
    //             }
    //         } else if ((type == "BOOL") || (type == "bool")) {
    //             option = new Option_Bool(false);
    //             if (value.empty()) {
    //                 option->set("false", "", true);
    //             } else {
    //                 option->set(value, value, true);
    //             }
    //         } else if (type == "INT[]") {
    //             option = new Option_IntVector();
    //         } else if (type == "STR[]") {
    //             option = new Option_StringVector();
    //         } else if ((type == "FILE") || (type == "file")) {
    //             option = new Option_FileName();
    //         } else if ((type == "NETWORK") || (type == "net_file")) {
    //             option = new Option_Network(value);
    //         } else if ((type == "ADDITIONAL") || (type == "additional_file")) {
    //             option = new Option_Additional(value);
    //         } else if ((type == "ROUTE") || (type == "route_file")) {
    //             option = new Option_Route(value);
    //         } else if ((type == "DATA") || (type == "data_file") || (type == "edgedata_file")) {
    //             option = new Option_Data(value);
    //         } else if ((type == "SUMOCONFIG") || (type == "sumoconfig_file")) {
    //             option = new Option_SumoConfig(value);
    //         } else if ((type == "EDGE") || (type == "edge")) {
    //             if (listSep.empty()) {
    //                 option = new Option_Edge(value);
    //             } else {
    //                 option = new Option_EdgeVector(value);
    //             }
    //         } else if (type.size() > 0) {
    //             WRITE_WARNING(type + " is an invalid type");
    //         }
    //         // check if option was created
    //         if (option) {
    //             // set value
    //             if (!option->isSet()) {
    //                 option->set(value, "", false);
    //             }
    //             myOptions.doRegister(myOptionName, option);
    //             // check if add synonyme
    //             if (synonymes.size() > 0) {
    //                 myOptions.addSynonyme(myOptionName, synonymes);
    //             }
    //             // check if add help
    //             if (help.size() > 0) {
    //                 myOptions.addDescription(myOptionName, mySubTopic, help);
    //             }
    //             myOptions.setFurtherAttributes(myOptionName, mySubTopic, required, positional, listSep);
    //             return true;
    //         } else {
    //             return false;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_option(&self, value: String, synonymes: &str, type_: &str, help: &str, required: bool, positional: bool, listSep: &str) -> bool {
        if self.myOptions.exists(&self.myOptionName) {
            write_warning(&format!("{} already exists", self.myOptionName));
            return false;
        }
        // declare option
        let mut value = value;
        let mut option: Option<SumoOption> = None;
        // handle "None" as empty
        if value == "None" {
            value.clear();
        }
        // create register depending of type
        if type_ == "STR" || type_ == "string" {
            option = Some(SumoOption::string_with(&value));
        } else if type_ == "TIME" || type_ == "time" {
            option = Some(SumoOption::string_typed(&value, "TIME"));
        } else if type_ == "INT" || type_ == "int" {
            let mut o = SumoOption::integer(0);
            if value.is_empty() {
                o.set(&to_string(&INVALID_INT), "", true).unwrap_or_else(|e| panic!("{}", e));
            } else {
                o.set(&value, &value, true).unwrap_or_else(|e| panic!("{}", e));
            }
            option = Some(o);
        } else if type_ == "FLOAT" || type_ == "float" {
            let mut o = SumoOption::float(0.);
            if value.is_empty() {
                o.set(&to_string(&INVALID_DOUBLE), "", true).unwrap_or_else(|e| panic!("{}", e));
            } else {
                o.set(&value, &value, true).unwrap_or_else(|e| panic!("{}", e));
            }
            option = Some(o);
        } else if type_ == "BOOL" || type_ == "bool" {
            let mut o = SumoOption::bool(false);
            if value.is_empty() {
                o.set("false", "", true).unwrap_or_else(|e| panic!("{}", e));
            } else {
                o.set(&value, &value, true).unwrap_or_else(|e| panic!("{}", e));
            }
            option = Some(o);
        } else if type_ == "INT[]" {
            option = Some(SumoOption::int_vector());
        } else if type_ == "STR[]" {
            option = Some(SumoOption::string_vector());
        } else if type_ == "FILE" || type_ == "file" {
            option = Some(SumoOption::file_name());
        } else if type_ == "NETWORK" || type_ == "net_file" {
            option = Some(SumoOption::network(&value));
        } else if type_ == "ADDITIONAL" || type_ == "additional_file" {
            option = Some(SumoOption::additional(&value));
        } else if type_ == "ROUTE" || type_ == "route_file" {
            option = Some(SumoOption::route(&value));
        } else if type_ == "DATA" || type_ == "data_file" || type_ == "edgedata_file" {
            option = Some(SumoOption::data(&value));
        } else if type_ == "SUMOCONFIG" || type_ == "sumoconfig_file" {
            option = Some(SumoOption::sumo_config(&value));
        } else if type_ == "EDGE" || type_ == "edge" {
            if listSep.is_empty() {
                option = Some(SumoOption::edge(&value));
            } else {
                option = Some(SumoOption::edge_vector(&value));
            }
        } else if !type_.is_empty() {
            write_warning(&format!("{} is an invalid type", type_));
        }
        // check if option was created
        if let Some(mut option) = option {
            // set value
            if !option.is_set() {
                option.set(&value, "", false).unwrap_or_else(|e| panic!("{}", e));
            }
            self.myOptions.m().do_register(&self.myOptionName, option);
            // check if add synonyme
            if !synonymes.is_empty() {
                self.myOptions.m().add_synonyme(&self.myOptionName, synonymes, false);
            }
            // check if add help
            if !help.is_empty() {
                self.myOptions.m().add_description(&self.myOptionName, &self.mySubTopic, help);
            }
            self.myOptions.m().set_further_attributes(&self.myOptionName, &self.mySubTopic, required, positional, listSep);
            true
        } else {
            false
        }
    }

    /// C++: `void TemplateHandler::endElement(const XMLCh *const)` [virtual, private]
    /// Called on the end of an element
    // void
    // TemplateHandler::endElement(const XMLCh* const /*name*/) {
    //     myLevel--;
    //     if (myOptionName.length() == 0) {
    //         return;
    //     }
    //     myOptionName = "";
    // }
    #[allow(unused_variables)]
    pub fn end_element(&mut self, name: &str) {
        self.myLevel -= 1;
        if self.myOptionName.is_empty() {
            return;
        }
        self.myOptionName = String::new();
    }

    /// C++: `void TemplateHandler::warning(const xercesc_3_2::SAXParseException &)` [virtual, private]
    /// Called on an XML-warning
    // void
    // TemplateHandler::warning(const XERCES_CPP_NAMESPACE::SAXParseException& exception) {
    //     WRITE_WARNING(XMLSubSys::transcode(exception.getMessage()));
    //     WRITE_WARNING(" (At line/column " \
    //                   + toString(exception.getLineNumber() + 1) + '/' \
    //                   + toString(exception.getColumnNumber()) + ").");
    //     myError = true;
    // }
    #[allow(unused_variables)]
    pub fn warning(&mut self, msg: &str, line: u64, column: u64) {
        write_warning(msg);
        write_warning(&format!(" (At line/column {}/{}).", line + 1, column));
        self.myError = true;
    }

    /// C++: `void TemplateHandler::error(const xercesc_3_2::SAXParseException &)` [virtual, private]
    /// Called on an XML-error
    // void
    // TemplateHandler::error(const XERCES_CPP_NAMESPACE::SAXParseException& exception) {
    //     WRITE_ERROR(XMLSubSys::transcode(exception.getMessage()));
    //     WRITE_ERROR(" (At line/column "
    //                 + toString(exception.getLineNumber() + 1) + '/'
    //                 + toString(exception.getColumnNumber()) + ").");
    //     myError = true;
    // }
    #[allow(unused_variables)]
    pub fn error(&mut self, msg: &str, line: u64, column: u64) {
        write_error(msg);
        write_error(&format!(" (At line/column {}/{}).", line + 1, column));
        self.myError = true;
    }

    /// C++: `void TemplateHandler::fatalError(const xercesc_3_2::SAXParseException &)` [virtual, private]
    /// Called on an XML-fatal error
    // void
    // TemplateHandler::fatalError(const XERCES_CPP_NAMESPACE::SAXParseException& exception) {
    //     WRITE_ERROR(XMLSubSys::transcode(exception.getMessage()));
    //     WRITE_ERROR(" (At line/column "
    //                 + toString(exception.getLineNumber() + 1) + '/'
    //                 + toString(exception.getColumnNumber()) + ").");
    //     myError = true;
    // }
    #[allow(unused_variables)]
    pub fn fatal_error(&mut self, msg: &str, line: u64, column: u64) {
        write_error(msg);
        write_error(&format!(" (At line/column {}/{}).", line + 1, column));
        self.myError = true;
    }

    /// C++: `void TemplateHandler::TemplateHandler()` [private]
    /// invalidate default onstructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2() -> Self {
        panic!("TemplateHandler default constructor is invalidated")
    }

    /// C++: `void TemplateHandler::TemplateHandler(const TemplateHandler &)` [private]
    /// invalid copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_3(s: &TemplateHandler) -> Self {
        panic!("TemplateHandler copy constructor is invalidated")
    }

    /// C++: `TemplateHandler & TemplateHandler::operator=(const TemplateHandler &)` [private]
    /// invalid assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &TemplateHandler) -> &mut TemplateHandler {
        panic!("TemplateHandler assignment operator is invalidated")
    }

}

