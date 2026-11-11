//! Port of utils/options/OptionsCont.{h,cpp}
//!
//! The global container (OptionsCont::getOptions()) is reached through `with_options(|oc| ...)`;
//! the free functions `get_string`, `get_float`, `get_int`, `get_bool`, `get_string_vector`,
//! `get_int_vector`, `is_set`, `is_default` and `exists` read single values from it.
//! Methods that throw ProcessError in C++ return `Result`; registration errors (programming
//! errors) panic with the C++ message.
use super::option::{IntVector, StringVector, SumoOption};
use super::options_io;
use crate::common::file_helpers;
use crate::common::msg_handler::{write_error, write_warning};
use crate::common::std_defs::g_locale_initialized;
use crate::common::string_tokenizer::StringTokenizer;
use crate::common::string_utils;
use crate::common::to_string::{join_to_string, to_string};
use crate::common::util_exceptions::ProcessError;
use crate::{tlf, write_errorf, write_messagef, write_warningf};
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::fmt;
use std::sync::{LazyLock, Mutex};

// shortcut: the build feature list of config.h (HAVE_ENABLED) is empty until optional features exist
pub const HAVE_ENABLED: &str = "";
/// SUMOSAXAttributes::ENCODING
const ENCODING: &str = " encoding=\"UTF-8\"";

#[derive(Debug, Clone)]
pub struct OptionsCont {
    am_localized: bool,
    /// all options, synonymes share the index
    options: Vec<SumoOption>,
    /// (first name, option index) in registration order, without synonymes
    addresses: Vec<(String, usize)>,
    values: BTreeMap<String, usize>,
    app_name: String,
    full_name: String,
    app_description: String,
    additional_message: String,
    call_examples: Vec<(String, String)>,
    sub_topics: Vec<String>,
    copyright_notices: Vec<String>,
    sub_topic_entries: BTreeMap<String, Vec<String>>,
    xml_defaults: BTreeMap<String, String>,
    deprecated_synonymes: RefCell<BTreeMap<String, bool>>,
}

static OPTIONS: LazyLock<Mutex<OptionsCont>> = LazyLock::new(|| Mutex::new(OptionsCont::new()));

/// OptionsCont::getOptions(). Not reentrant: do not call `with_options` (or the getters
/// below) from inside `f`.
pub fn with_options<R>(f: impl FnOnce(&mut OptionsCont) -> R) -> R {
    let mut g = OPTIONS.lock().unwrap_or_else(|e| e.into_inner());
    f(&mut g)
}

fn or_panic<T>(r: Result<T, ProcessError>) -> T {
    r.unwrap_or_else(|e| panic!("{e}"))
}

/// OptionsCont::getOptions().getString(name); panics for unknown options
pub fn get_string(name: &str) -> String {
    with_options(|oc| or_panic(oc.get_string(name)))
}
pub fn get_float(name: &str) -> f64 {
    with_options(|oc| or_panic(oc.get_float(name)))
}
pub fn get_int(name: &str) -> i32 {
    with_options(|oc| or_panic(oc.get_int(name)))
}
pub fn get_bool(name: &str) -> bool {
    with_options(|oc| or_panic(oc.get_bool(name)))
}
pub fn get_string_vector(name: &str) -> StringVector {
    with_options(|oc| or_panic(oc.get_string_vector(name)).clone())
}
pub fn get_int_vector(name: &str) -> IntVector {
    with_options(|oc| or_panic(oc.get_int_vector(name)).clone())
}
/// isSet(name) (fails on unknown options)
pub fn is_set(name: &str) -> bool {
    with_options(|oc| or_panic(oc.is_set(name, true)))
}
pub fn is_default(name: &str) -> bool {
    with_options(|oc| oc.is_default(name))
}
pub fn exists(name: &str) -> bool {
    with_options(|oc| oc.exists(name))
}

impl Default for OptionsCont {
    fn default() -> Self {
        Self::new()
    }
}

impl OptionsCont {
    pub fn new() -> Self {
        OptionsCont {
            am_localized: false,
            options: Vec::new(),
            addresses: Vec::new(),
            values: BTreeMap::new(),
            app_name: String::new(),
            full_name: String::new(),
            app_description: String::new(),
            additional_message: String::new(),
            call_examples: Vec::new(),
            sub_topics: Vec::new(),
            copyright_notices: vec!["Copyright (C) 2001-2026 German Aerospace Center (DLR) and others; https://sumo.dlr.de".into()],
            sub_topic_entries: BTreeMap::new(),
            xml_defaults: BTreeMap::new(),
            deprecated_synonymes: RefCell::new(BTreeMap::new()),
        }
    }

    pub fn do_register(&mut self, name: &str, o: SumoOption) {
        if self.values.contains_key(name) {
            panic!("{name} is an already used option name.");
        }
        self.options.push(o);
        self.register_index(name, self.options.len() - 1);
    }

    /// doRegister(name, Option*) for an option that is already known (synonym)
    fn register_index(&mut self, name: &str, idx: usize) {
        if self.values.contains_key(name) {
            panic!("{name} is an already used option name.");
        }
        if !self.addresses.iter().any(|(_, i)| *i == idx) {
            self.addresses.push((name.to_string(), idx));
        }
        self.values.insert(name.to_string(), idx);
    }

    /// doRegister(name, abbr, o)
    pub fn do_register_abbr(&mut self, name1: &str, abbr: char, o: SumoOption) {
        self.do_register(name1, o);
        let idx = self.values[name1];
        self.register_index(&self.convert_char(abbr), idx);
    }

    pub fn add_synonyme(&mut self, name1: &str, name2: &str, is_deprecated: bool) {
        let i1 = self.values.get(name1).copied();
        let i2 = self.values.get(name2).copied();
        match (i1, i2) {
            (None, None) => panic!("Neither the option '{name1}' nor the option '{name2}' is known yet"),
            (Some(a), Some(b)) => {
                if a != b {
                    panic!("Both options '{name1}' and '{name2}' do exist and differ.");
                }
            }
            (None, Some(b)) => {
                self.register_index(name1, b);
                if is_deprecated {
                    self.deprecated_synonymes.borrow_mut().insert(name1.to_string(), false);
                }
            }
            (Some(a), None) => {
                self.register_index(name2, a);
                if is_deprecated {
                    self.deprecated_synonymes.borrow_mut().insert(name2.to_string(), false);
                }
            }
        }
    }

    pub fn add_xml_default(&mut self, name: &str, xml_root: &str) {
        self.xml_defaults.insert(xml_root.to_string(), name.to_string());
    }

    pub fn exists(&self, name: &str) -> bool {
        self.values.contains_key(name)
    }

    pub fn is_set(&self, name: &str, fail_on_non_existant: bool) -> Result<bool, ProcessError> {
        match self.values.get(name) {
            None if fail_on_non_existant => Err(ProcessError::new(tlf!("Internal request for unknown option '%'!", name))),
            None => Ok(false),
            Some(&i) => Ok(self.options[i].is_set()),
        }
    }

    pub fn is_default(&self, name: &str) -> bool {
        self.values.get(name).is_some_and(|&i| self.options[i].is_default())
    }

    fn get_secure_index(&self, name: &str) -> Result<usize, ProcessError> {
        let Some(&idx) = self.values.get(name) else {
            return Err(ProcessError::new(tlf!("No option with the name '%' exists.", name)));
        };
        let warn = self.deprecated_synonymes.borrow().get(name) == Some(&false);
        if warn {
            let mut default_name = String::new();
            'outer: for entries in self.sub_topic_entries.values() {
                for value in entries {
                    if self.values.get(value) == Some(&idx) {
                        default_name = value.clone();
                        break 'outer;
                    }
                }
            }
            write_warningf!("Please note that '%' is deprecated.\n Use '%' instead.", name, default_name);
            self.deprecated_synonymes.borrow_mut().insert(name.to_string(), true);
        }
        Ok(idx)
    }

    fn get_secure(&self, name: &str) -> Result<&SumoOption, ProcessError> {
        Ok(&self.options[self.get_secure_index(name)?])
    }

    fn get_secure_mut(&mut self, name: &str) -> Result<&mut SumoOption, ProcessError> {
        let i = self.get_secure_index(name)?;
        Ok(&mut self.options[i])
    }

    pub fn get_value_string(&self, name: &str) -> Result<String, ProcessError> {
        Ok(self.get_secure(name)?.get_value_string().to_string())
    }

    pub fn get_string(&self, name: &str) -> Result<String, ProcessError> {
        self.get_secure(name)?.get_string()
    }

    pub fn get_float(&self, name: &str) -> Result<f64, ProcessError> {
        self.get_secure(name)?.get_float()
    }

    pub fn get_int(&self, name: &str) -> Result<i32, ProcessError> {
        self.get_secure(name)?.get_int()
    }

    pub fn get_bool(&self, name: &str) -> Result<bool, ProcessError> {
        self.get_secure(name)?.get_bool()
    }

    pub fn get_int_vector(&self, name: &str) -> Result<&IntVector, ProcessError> {
        self.get_secure(name)?.get_int_vector()
    }

    pub fn get_string_vector(&self, name: &str) -> Result<&StringVector, ProcessError> {
        self.get_secure(name)?.get_string_vector()
    }

    /// Ok(false) when the option was not writable or the value was invalid (error written)
    pub fn set(&mut self, name: &str, value: &str, append: bool) -> Result<bool, ProcessError> {
        let i = self.get_secure_index(name)?;
        if !self.options[i].is_writeable() {
            self.report_double_setting(name);
            return Ok(false);
        }
        // Substitute environment variables defined by ${NAME} with their value
        let v = string_utils::substitute_environment(value, Some(options_io::get_load_time()));
        match self.options[i].set(&v, value, append) {
            Ok(true) => Ok(true),
            Ok(false) => Ok(false),
            Err(e) => {
                write_error(&format!("While processing option '{name}':\n {e}"));
                Ok(false)
            }
        }
    }

    pub fn set_default(&mut self, name: &str, value: &str) -> Result<bool, ProcessError> {
        let i = self.get_secure_index(name)?;
        if self.options[i].is_writeable() && self.set(name, value, false)? {
            self.options[i].reset_default();
            return Ok(true);
        }
        Ok(false)
    }

    pub fn set_by_root_element(&mut self, root: &str, value: &str) -> Result<bool, ProcessError> {
        if let Some(name) = self.xml_defaults.get(root).cloned() {
            return self.set(&name, value, false);
        }
        if let Some(name) = self.xml_defaults.get("").cloned() {
            return self.set(&name, value, false);
        }
        Ok(false)
    }

    pub fn get_synonymes(&self, name: &str) -> Result<Vec<String>, ProcessError> {
        let o = self.get_secure_index(name)?;
        Ok(self.values.iter().filter(|(k, v)| **v == o && k.as_str() != name).map(|(k, _)| k.clone()).collect())
    }

    pub fn get_description(&self, name: &str) -> Result<&str, ProcessError> {
        Ok(self.get_secure(name)?.get_description())
    }

    pub fn get_sub_topic(&self, name: &str) -> Result<&str, ProcessError> {
        Ok(self.get_secure(name)?.get_sub_topic())
    }

    pub fn relocate_files(&mut self, configuration: &str) {
        for a in 0..self.addresses.len() {
            let (name, idx) = self.addresses[a].clone();
            let o = &self.options[idx];
            if o.is_file_name() && o.is_set() {
                let mut file_list = o.get_string_vector().unwrap().clone();
                for file in file_list.iter_mut() {
                    if name != "configuration-file" {
                        *file = file_helpers::check_for_relativity(file, configuration);
                    }
                    match string_utils::url_decode(file) {
                        Ok(f) => *file = f,
                        Err(e) => write_warning(&format!("{e} when trying to decode filename '{file}'.")),
                    }
                }
                let mut raw_list = StringTokenizer::with_token(o.get_value_string(), ",", false).get_vector();
                for file in raw_list.iter_mut() {
                    *file = file_helpers::check_for_relativity(file, configuration);
                }
                let conv = join_to_string(&file_list, ",");
                if conv != join_to_string(o.get_string_vector().unwrap(), ",") {
                    let had_default = o.is_default();
                    let o = &mut self.options[idx];
                    let _ = o.set(&conv, &join_to_string(&raw_list, ","), false);
                    if had_default {
                        o.reset_default();
                    }
                }
            }
        }
    }

    pub fn is_usable_file_list(&self, name: &str) -> Result<bool, ProcessError> {
        if !self.get_secure(name)?.is_set() {
            return Ok(false);
        }
        // check whether the list of files is valid
        let mut ok = true;
        let files = self.get_string_vector(name)?;
        if files.is_empty() {
            write_errorf!("The file list for '%' is empty.", name);
            ok = false;
        }
        for file in files {
            if !file_helpers::is_readable(file) {
                if !file.is_empty() {
                    let err = match std::fs::File::open(file) {
                        Err(e) if e.kind() == std::io::ErrorKind::NotFound => "No such file or directory".to_string(),
                        Err(e) => e.to_string(),
                        Ok(_) => String::new(),
                    };
                    write_errorf!("File '%' is not accessible (%).", file, err);
                    ok = false;
                } else {
                    write_warning("Empty file name given; ignoring.");
                }
            }
        }
        Ok(ok)
    }

    pub fn check_depending_suboptions(&self, name: &str, prefix: &str) -> Result<bool, ProcessError> {
        if self.get_secure(name)?.is_set() {
            return Ok(true);
        }
        let mut ok = true;
        let mut seen_synonymes: Vec<String> = Vec::new();
        for (key, &idx) in &self.values {
            if seen_synonymes.contains(key) {
                continue;
            }
            let o = &self.options[idx];
            if o.is_set() && !o.is_default() && key.starts_with(prefix) {
                write_errorf!("Option '%' needs option '%'.", key, name);
                seen_synonymes.extend(self.get_synonymes(key)?);
                ok = false;
            }
        }
        Ok(ok)
    }

    fn report_double_setting(&self, arg: &str) {
        let synonymes = self.get_synonymes(arg).unwrap_or_default();
        let s = tlf!("A value for the option '%' was already set.\n Possible synonymes: ", arg) + &synonymes.join(", ");
        write_error(&s);
    }

    fn convert_char(&self, abbr: char) -> String {
        abbr.to_string()
    }

    pub fn is_bool(&self, name: &str) -> Result<bool, ProcessError> {
        Ok(self.get_secure(name)?.is_bool())
    }

    pub fn reset_writable(&mut self) {
        for (_, i) in &self.addresses {
            self.options[*i].reset_writable();
        }
    }

    pub fn reset_writable_name(&mut self, name: &str) -> Result<(), ProcessError> {
        self.get_secure_mut(name)?.reset_writable();
        Ok(())
    }

    pub fn reset_default(&mut self) {
        for (_, i) in &self.addresses {
            self.options[*i].reset_default();
        }
    }

    pub fn reset_default_name(&mut self, name: &str) -> Result<(), ProcessError> {
        self.get_secure_mut(name)?.reset_default();
        Ok(())
    }

    pub fn is_writeable(&self, name: &str) -> Result<bool, ProcessError> {
        Ok(self.get_secure(name)?.is_writeable())
    }

    pub fn is_editable(&self, name: &str) -> Result<bool, ProcessError> {
        Ok(self.get_secure(name)?.is_editable())
    }

    pub fn clear(&mut self) {
        self.options.clear();
        self.addresses.clear();
        self.values.clear();
        self.sub_topics.clear();
        self.sub_topic_entries.clear();
    }

    fn check_sub_topic(&self, name: &str, subtopic: &str) -> usize {
        let i = or_panic(self.get_secure_index(name));
        if !self.sub_topics.iter().any(|t| t == subtopic) {
            panic!("SubTopic '{subtopic}' doesn't exist");
        }
        i
    }

    pub fn add_description(&mut self, name: &str, subtopic: &str, description: &str) {
        let i = self.check_sub_topic(name, subtopic);
        self.options[i].set_description(description);
        self.options[i].set_subtopic(subtopic);
        self.sub_topic_entries.entry(subtopic.to_string()).or_default().push(name.to_string());
    }

    pub fn set_further_attributes(&mut self, name: &str, subtopic: &str, required: bool, positional: bool, list_sep: &str) {
        let i = self.check_sub_topic(name, subtopic);
        let o = &mut self.options[i];
        if required {
            o.set_required();
        }
        if positional {
            o.set_positional();
        }
        o.set_list_separator(list_sep);
    }

    pub fn set_option_editable(&mut self, name: &str, value: bool) -> Result<(), ProcessError> {
        self.get_secure_mut(name)?.set_editable(value);
        Ok(())
    }

    pub fn set_application_name(&mut self, app_name: &str, full_name: &str) {
        self.app_name = app_name.to_string();
        self.full_name = full_name.to_string();
    }

    pub fn set_application_description(&mut self, app_desc: &str) {
        self.app_description = app_desc.to_string();
    }

    pub fn add_call_example(&mut self, example: &str, desc: &str) {
        self.call_examples.push((example.to_string(), desc.to_string()));
    }

    pub fn set_additional_help_message(&mut self, add: &str) {
        self.additional_message = add.to_string();
    }

    pub fn add_copyright_notice(&mut self, copyright_line: &str) {
        self.copyright_notices.push(copyright_line.to_string());
    }

    pub fn clear_copyright_notices(&mut self) {
        self.copyright_notices.clear();
    }

    pub fn add_option_sub_topic(&mut self, topic: &str) {
        self.sub_topics.push(topic.to_string());
        self.sub_topic_entries.insert(topic.to_string(), Vec::new());
    }

    pub fn remove_sub_topic(&mut self, topic: &str) {
        if let Some(i) = self.sub_topics.iter().position(|t| t == topic) {
            self.sub_topics.remove(i);
        }
        self.sub_topic_entries.remove(topic);
    }

    pub fn remove_option(&mut self, name: &str) {
        for entry in self.sub_topic_entries.values_mut() {
            if let Some(i) = entry.iter().position(|n| n == name) {
                entry.remove(i);
            }
        }
    }

    fn split_lines(os: &mut String, what: &str, mut offset: i32, next_offset: i32) {
        // byte based like std::string; split points are ASCII so the pieces stay valid UTF-8
        let mut what = what.as_bytes();
        let rfind = |w: &[u8], c: u8, pos: i32| -> Option<usize> {
            // a negative pos converts to npos, i.e. search everything
            let end = if pos < 0 { w.len() } else { (pos as usize + 1).min(w.len()) };
            w[..end].iter().rposition(|&b| b == c)
        };
        while !what.is_empty() {
            if what.len() as i32 > 79 - offset {
                let split_pos = match rfind(what, b';', 79 - offset) {
                    Some(p) => Some(p + 1),
                    None => rfind(what, b' ', 79 - offset),
                };
                if let Some(sp) = split_pos {
                    *os += &String::from_utf8_lossy(&what[..sp]);
                    *os += "\n";
                    what = &what[(sp + 1).min(what.len())..];
                    for _ in 0..next_offset + 1 {
                        os.push(' ');
                    }
                } else {
                    *os += &String::from_utf8_lossy(what);
                    what = &[];
                }
                offset = next_offset;
            } else {
                *os += &String::from_utf8_lossy(what);
                what = &[];
            }
        }
        *os += "\n";
    }

    /// Prints version / help / configuration as requested; returns true if the application
    /// should stop. Output goes to stdout like std::cout.
    pub fn process_meta_options(&mut self, missing_options: bool) -> Result<bool, ProcessError> {
        crate::common::msg_handler::MsgHandler::setup_i18n(&self.get_string("language")?);
        self.localize_descriptions();
        let mut out = String::new();
        if missing_options {
            // no options are given
            out += &format!("{}\n", self.full_name);
            out += &format!(" Build features: {HAVE_ENABLED}\n");
            for c in &self.copyright_notices {
                out += &format!(" {c}\n");
            }
            out += " License EPL-2.0: Eclipse Public License Version 2 <https://eclipse.org/legal/epl-v20.html>\n";
            out += " Use --help to get the list of options.\n";
            print!("{out}");
            return Ok(true);
        }
        // check whether the help shall be printed
        if self.get_bool("help")? {
            out += &format!("{}\n", self.full_name);
            for c in &self.copyright_notices {
                out += &format!(" {c}\n");
            }
            self.print_help(&mut out)?;
            print!("{out}");
            return Ok(true);
        }
        // check whether the version shall be printed
        if self.get_bool("version")? {
            out += &format!("{}\n", self.full_name);
            out += &format!(" Build features: {HAVE_ENABLED}\n");
            for c in &self.copyright_notices {
                out += &format!(" {c}\n");
            }
            out += &format!("\n{} is part of SUMO.\n", self.full_name);
            out += "This program and the accompanying materials\n";
            out += "are made available under the terms of the Eclipse Public License v2.0\n";
            out += "which accompanies this distribution, and is available at\n";
            out += "http://www.eclipse.org/legal/epl-v20.html\n";
            out += "This program may also be made available under the following Secondary\n";
            out += "Licenses when the conditions for such availability set forth in the Eclipse\n";
            out += "Public License 2.0 are satisfied: GNU General Public License, version 2\n";
            out += "or later which is available at\n";
            out += "https://www.gnu.org/licenses/old-licenses/gpl-2.0-standalone.html\n";
            out += "SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later\n";
            print!("{out}");
            return Ok(true);
        }
        // check whether the settings shall be printed
        if self.get_bool("print-options")? {
            print!("{self}");
        }
        // whether the current options shall be saved
        if self.is_set("save-configuration", true)? {
            let config_path = self.get_string("save-configuration")?;
            if config_path == "-" || config_path == "stdout" {
                self.write_configuration(&mut out, true, false, self.get_bool("save-commented")?, "", false, false, "")?;
                print!("{out}");
                return Ok(true);
            }
            self.write_configuration(&mut out, true, false, self.get_bool("save-commented")?, &config_path, false, false, "")?;
            if std::fs::write(&config_path, &out).is_err() {
                return Err(ProcessError::new(tlf!("Could not save configuration to '%'", config_path)));
            }
            if self.get_bool("verbose")? {
                write_messagef!("Written configuration to '%'", config_path);
            }
            return Ok(true);
        }
        // whether the template shall be saved
        if self.is_set("save-template", true)? {
            let path = self.get_string("save-template")?;
            self.write_configuration(&mut out, false, true, self.get_bool("save-commented")?, "", false, false, "")?;
            if path == "-" || path == "stdout" {
                print!("{out}");
                return Ok(true);
            }
            if std::fs::write(&path, &out).is_err() {
                return Err(ProcessError::new(tlf!("Could not save template to '%'", path)));
            }
            if self.get_bool("verbose")? {
                write_messagef!("Written template to '%'", path);
            }
            return Ok(true);
        }
        if self.is_set("save-schema", true)? {
            let path = self.get_string("save-schema")?;
            self.write_schema(&mut out)?;
            if path == "-" || path == "stdout" {
                print!("{out}");
                return Ok(true);
            }
            if std::fs::write(&path, &out).is_err() {
                return Err(ProcessError::new(tlf!("Could not save schema to '%'", path)));
            }
            if self.get_bool("verbose")? {
                write_messagef!("Written schema to '%'", path);
            }
            return Ok(true);
        }
        Ok(false)
    }

    /// TL() is the identity, so only the flag changes
    pub fn localize_descriptions(&mut self) {
        if !self.am_localized && g_locale_initialized() {
            self.am_localized = true;
        }
    }

    pub fn get_sub_topics(&self) -> &[String] {
        &self.sub_topics
    }

    pub fn get_sub_topics_entries(&self, subtopic: &str) -> Vec<String> {
        self.sub_topic_entries.get(subtopic).cloned().unwrap_or_default()
    }

    pub fn get_type_name(&self, name: &str) -> Result<String, ProcessError> {
        Ok(self.get_secure(name)?.get_type_name().to_string())
    }

    pub fn get_full_name(&self) -> &str {
        &self.full_name
    }

    pub fn is_empty(&self) -> bool {
        self.addresses.is_empty()
    }

    /// begin()/end(): the options in registration order (without synonymes)
    pub fn iter(&self) -> impl Iterator<Item = (&str, &SumoOption)> {
        self.addresses.iter().map(|(n, i)| (n.as_str(), &self.options[*i]))
    }

    /// clone(): a copy whose options are all writable again
    pub fn clone_options(&self) -> OptionsCont {
        let mut oc = self.clone();
        oc.reset_writable();
        oc
    }

    fn has_abbreviation(&self, entry: &str) -> Result<Option<String>, ProcessError> {
        let deprecated = self.deprecated_synonymes.borrow();
        Ok(self.get_synonymes(entry)?.into_iter().find(|s| s.len() == 1 && !deprecated.contains_key(s)))
    }

    pub fn print_help(&self, os: &mut String) -> Result<(), ProcessError> {
        // print application description
        Self::split_lines(os, &self.app_description, 0, 0);
        *os += "\n";
        // check option sizes first
        //  we want to know how large the largest not-too-large-entry will be
        let too_large = 40;
        let mut max_size = 0;
        let empty = Vec::new();
        for sub_topic in &self.sub_topics {
            for entry in self.sub_topic_entries.get(sub_topic).unwrap_or(&empty) {
                let o = self.get_secure(entry)?;
                // name, two leading spaces and "--"
                let mut csize = entry.len() as i32 + 2 + 4;
                // abbreviation length ("-X, "->4chars) if any
                if self.has_abbreviation(entry)?.is_some() {
                    csize += 4;
                }
                // the type name
                if !o.is_bool() {
                    csize += 1 + o.get_type_name().len() as i32;
                }
                // divider
                csize += 2;
                if csize < too_large && max_size < csize {
                    max_size = csize;
                }
            }
        }
        let help_topic = string_utils::to_lower_case(self.get_secure("help")?.get_value_string());
        if !help_topic.is_empty() {
            let mut found_topic = false;
            for topic in &self.sub_topics {
                if string_utils::to_lower_case(topic).contains(&help_topic) {
                    found_topic = true;
                    self.print_help_on_topic(topic, too_large, max_size, os)?;
                }
            }
            if !found_topic {
                // print topic list
                *os += "Help Topics:\n";
                for t in &self.sub_topics {
                    *os += &format!("    {t}\n");
                }
            }
            return Ok(());
        }
        // print usage BNF
        *os += &format!("Usage: {} [OPTION]*\n", self.app_name);
        // print additional text if any
        if !self.additional_message.is_empty() {
            *os += &format!("{}\n\n", self.additional_message);
        }
        // print the options
        for sub_topic in &self.sub_topics {
            self.print_help_on_topic(sub_topic, too_large, max_size, os)?;
        }
        *os += "\n";
        // print usage examples, calc size first
        if !self.call_examples.is_empty() {
            *os += "Examples:\n";
            for (example, desc) in &self.call_examples {
                *os += &format!("  {} {}\n", self.app_name, example);
                *os += &format!("    {desc}\n");
            }
        }
        *os += "\n";
        *os += &tlf!("Report bugs at %.", "<https://github.com/eclipse-sumo/sumo/issues>");
        *os += "\n";
        *os += &tlf!("Get in contact via %.", "<sumo@dlr.de>");
        *os += "\n";
        Ok(())
    }

    pub fn print_help_on_topic(&self, topic: &str, too_large: i32, max_size: i32, os: &mut String) -> Result<(), ProcessError> {
        *os += &tlf!("% Options:", topic);
        *os += "\n";
        for entry in self.sub_topic_entries.get(topic).map(|v| v.as_slice()).unwrap_or(&[]) {
            // start length computation
            let mut csize = entry.len() as i32 + 2;
            let o = self.get_secure(entry)?;
            *os += "  ";
            // write abbreviation if given
            if let Some(abbr) = self.has_abbreviation(entry)? {
                *os += &format!("-{abbr}, ");
                csize += 4;
            }
            // write leading '-'/"--"
            *os += "--";
            csize += 2;
            // write the name
            *os += entry;
            // write the type if not a bool option
            if !o.is_bool() {
                *os += &format!(" {}", o.get_type_name());
                csize += 1 + o.get_type_name().len() as i32;
            }
            csize += 2;
            // write the description formatting it
            *os += "  ";
            for _ in csize..max_size {
                os.push(' ');
            }
            let offset = if csize > too_large { csize } else { max_size };
            Self::split_lines(os, o.get_description(), offset, max_size);
        }
        *os += "\n";
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn write_configuration(
        &self,
        os: &mut String,
        filled: bool,
        complete: bool,
        add_comments: bool,
        relative_to: &str,
        force_relative: bool,
        in_comment: bool,
        indent: &str,
    ) -> Result<(), ProcessError> {
        if !in_comment {
            self.write_xml_header(os, false)?;
        }
        let app = if self.app_name == "sumo-gui" { "sumo" } else { self.app_name.as_str() };
        *os += &format!(
            "{indent}<{app}Configuration xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" xsi:noNamespaceSchemaLocation=\"http://sumo.dlr.de/xsd/{app}Configuration.xsd\">\n\n"
        );
        for subtopic in &self.sub_topics {
            if subtopic == "Configuration" && !complete {
                continue;
            }
            let entries = &self.sub_topic_entries[subtopic];
            let subtopic = string_utils::to_lower_case(&subtopic.replace(' ', "_"));
            let mut had_one = false;
            for name in entries {
                let o = self.get_secure(name)?;
                let write = complete || (filled && !o.is_default());
                if !write {
                    continue;
                }
                if name == "registry-viewport" && !complete {
                    continue;
                }
                if !had_one {
                    *os += &format!("{indent}    <{subtopic}>\n");
                }
                // add the comment if wished
                if add_comments {
                    *os += &format!("{indent}        <!-- {} -->\n", string_utils::escape_xml(o.get_description(), in_comment));
                }
                // write the option and the value (if given)
                *os += &format!("{indent}        <{name} value=\"");
                if o.is_set() && (filled || o.is_default()) {
                    if o.is_file_name() && !relative_to.is_empty() {
                        let mut file_list = StringTokenizer::with_token(o.get_value_string(), ",", false).get_vector();
                        for file in file_list.iter_mut() {
                            if file.starts_with("${") {
                                // there is an environment variable up front, assume it points to an absolute path
                                // not even forcing relativity makes sense here
                                *file = string_utils::url_encode(file, " ;%");
                            } else {
                                *file = file_helpers::fix_relative(
                                    &string_utils::url_encode(file, " ;%"),
                                    &string_utils::url_encode(relative_to, " ;%"),
                                    force_relative || self.get_bool("save-configuration.relative")?,
                                    "",
                                );
                            }
                        }
                        *os += &string_utils::escape_xml(&join_to_string(&file_list, ","), in_comment);
                    } else {
                        *os += &string_utils::escape_xml(o.get_value_string(), in_comment);
                    }
                }
                if complete {
                    let synonymes = self.get_synonymes(name)?;
                    if !synonymes.is_empty() {
                        *os += &format!("\" synonymes=\"{}", to_string(&synonymes));
                    }
                    let mut deprecated = String::new();
                    for synonym in &synonymes {
                        if self.deprecated_synonymes.borrow().contains_key(synonym) {
                            deprecated += &format!(" {synonym}");
                        }
                    }
                    if !deprecated.is_empty() {
                        *os += &format!("\" deprecated=\"{}", &deprecated[1..]);
                    }
                    *os += &format!("\" type=\"{}", o.get_type_name());
                    if !add_comments {
                        *os += &format!("\" help=\"{}", string_utils::escape_xml(o.get_description(), false));
                    }
                }
                *os += "\"/>\n";
                // append an endline if a comment was printed
                if add_comments {
                    *os += "\n";
                }
                had_one = true;
            }
            if had_one {
                *os += &format!("{indent}    </{subtopic}>\n\n");
            }
        }
        *os += &format!("{indent}</{app}Configuration>\n");
        Ok(())
    }

    pub fn write_schema(&self, os: &mut String) -> Result<(), ProcessError> {
        let app = if self.app_name == "sumo-gui" { "sumo" } else { self.app_name.as_str() };
        self.write_xml_header(os, false)?;
        *os += "<xsd:schema elementFormDefault=\"qualified\" xmlns:xsd=\"http://www.w3.org/2001/XMLSchema\">\n";
        *os += &format!("    <xsd:complexType name=\"{app}ConfigurationType\">\n");
        *os += "        <xsd:all>\n";
        let norm = |s: &str| string_utils::to_lower_case(&s.replace(' ', "_"));
        for subtopic in &self.sub_topics {
            if subtopic == "Configuration" {
                continue;
            }
            let subtopic = norm(subtopic);
            *os += &format!("            <xsd:element name=\"{subtopic}\" type=\"{app}{subtopic}TopicType\" minOccurs=\"0\"/>\n");
        }
        *os += "        </xsd:all>\n";
        *os += "    </xsd:complexType>\n\n";
        for subtopic in &self.sub_topics {
            if subtopic == "Configuration" {
                continue;
            }
            let entries = &self.sub_topic_entries[subtopic];
            let subtopic = norm(subtopic);
            *os += &format!("    <xsd:complexType name=\"{app}{subtopic}TopicType\">\n");
            *os += "        <xsd:all>\n";
            for entry in entries {
                let o = &self.options[self.values[entry]];
                let mut ty = string_utils::to_lower_case(o.get_type_name());
                if ty == "int[]" {
                    ty = "intArray".into();
                }
                if ty == "str[]" {
                    ty = "strArray".into();
                }
                *os += &format!("            <xsd:element name=\"{entry}\" type=\"{ty}OptionType\" minOccurs=\"0\"/>\n");
            }
            *os += "        </xsd:all>\n";
            *os += "    </xsd:complexType>\n\n";
        }
        *os += "</xsd:schema>\n";
        Ok(())
    }

    pub fn write_xml_header(&self, os: &mut String, include_config: bool) -> Result<(), ProcessError> {
        *os += &format!("<?xml version=\"1.0\"{ENCODING}?>\n\n");
        *os += "<!-- ";
        if !self.get_bool("write-metadata")? {
            *os += &format!("generated on {} by {}\n", string_utils::iso_time_string(None), self.full_name);
        }
        if self.get_bool("write-license")? {
            *os += "This data file and the accompanying materials\n\
are made available under the terms of the Eclipse Public License v2.0\n\
which accompanies this distribution, and is available at\n\
http://www.eclipse.org/legal/epl-v20.html\n\
This file may also be made available under the following Secondary\n\
Licenses when the conditions for such availability set forth in the Eclipse\n\
Public License 2.0 are satisfied: GNU General Public License, version 2\n\
or later which is available at\n\
https://www.gnu.org/licenses/old-licenses/gpl-2.0-standalone.html\n\
SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later\n";
        }
        if include_config && !self.get_bool("write-metadata")? {
            self.write_configuration(os, true, false, false, "", false, true, "")?;
        }
        *os += "-->\n\n";
        Ok(())
    }

    pub fn is_in_string_vector(&self, option_name: &str, item_name: &str) -> Result<bool, ProcessError> {
        if self.is_set(option_name, true)? {
            return Ok(self.get_string_vector(option_name)?.iter().any(|v| v == item_name));
        }
        Ok(false)
    }
}

/// operator<<(std::ostream&, const OptionsCont&)
impl fmt::Display for OptionsCont {
    fn fmt(&self, os: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut done: Vec<String> = Vec::new();
        writeln!(os, "Options set:")?;
        for (name, &idx) in &self.values {
            if done.contains(name) {
                continue;
            }
            let synonymes = self.get_synonymes(name).unwrap_or_default();
            if !synonymes.is_empty() {
                write!(os, "{name} ({})", synonymes.join(", "))?;
            } else {
                write!(os, "{name}")?;
            }
            let o = &self.options[idx];
            if o.is_set() {
                writeln!(os, ": {}", o.get_value_string())?;
            } else {
                writeln!(os, ": <INVALID>")?;
            }
            done.push(name.clone());
            done.extend(synonymes);
        }
        Ok(())
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    /// a small container like SystemFrame builds
    pub(crate) fn sample() -> OptionsCont {
        let mut oc = OptionsCont::new();
        oc.set_application_name("sumo", "Eclipse SUMO sumo Version test");
        oc.set_application_description("A microscopic, multi-modal traffic simulation.");
        oc.add_option_sub_topic("Configuration");
        oc.add_option_sub_topic("Input");
        oc.add_option_sub_topic("Report");
        oc.do_register_abbr("configuration-file", 'c', SumoOption::file_name());
        oc.add_synonyme("configuration-file", "configuration", false);
        oc.add_description("configuration-file", "Configuration", "Loads the named config on startup");
        oc.add_xml_default("configuration-file", "");
        oc.do_register_abbr("net-file", 'n', SumoOption::file_name());
        oc.add_synonyme("net-file", "net", false);
        oc.add_description("net-file", "Input", "Load road network description from FILE");
        oc.add_xml_default("net-file", "net");
        oc.do_register_abbr("route-files", 'r', SumoOption::file_name());
        oc.add_synonyme("route-files", "routes", true);
        oc.add_description("route-files", "Input", "Load routes descriptions from FILE(s)");
        oc.do_register("begin", SumoOption::string_with("0"));
        oc.add_description("begin", "Input", "Defines the begin time");
        oc.do_register_abbr("verbose", 'v', SumoOption::bool(false));
        oc.add_description("verbose", "Report", "Switches to verbose output");
        oc.do_register_abbr("help", '?', SumoOption::bool_extended(false));
        oc.add_description("help", "Report", "Prints this screen or selected topics");
        oc.do_register("scale", SumoOption::float(1.));
        oc.add_description("scale", "Input", "Scale demand by the given factor (by discarding or duplicating vehicles)");
        oc.do_register("threads", SumoOption::integer(1));
        oc.add_description("threads", "Report", "Number of threads");
        oc.do_register("write-metadata", SumoOption::bool(true));
        oc.do_register("write-license", SumoOption::bool(false));
        oc.do_register("save-configuration.relative", SumoOption::bool(false));
        oc
    }

    #[test]
    fn register_and_get() {
        let mut oc = sample();
        assert!(oc.exists("n") && oc.exists("net"));
        assert_eq!(oc.get_synonymes("net-file").unwrap(), ["n", "net"]);
        assert!(!oc.is_set("net-file", true).unwrap());
        assert_eq!(oc.is_set("nope", true).unwrap_err().to_string(), "Internal request for unknown option 'nope'!");
        assert_eq!(oc.get_string("nope").unwrap_err().to_string(), "No option with the name 'nope' exists.");
        assert!(oc.set("net", "a.net.xml", false).unwrap());
        assert_eq!(oc.get_string("net-file").unwrap(), "a.net.xml");
        // second set of a synonym is rejected
        assert!(!oc.set("n", "b.net.xml", false).unwrap());
        assert!(oc.set("scale", "2.5", false).unwrap());
        assert_eq!(oc.get_float("scale").unwrap(), 2.5);
        assert!(!oc.set("threads", "x", false).unwrap());
        assert_eq!(oc.get_int("threads").unwrap(), 1);
        assert!(oc.is_default("threads"));
        assert!(oc.set_by_root_element("net", "x.net.xml").is_ok());
        assert!(!oc.check_depending_suboptions("route-files", "net").unwrap());
    }

    #[test]
    fn display() {
        let mut oc = sample();
        oc.set("begin", "5", false).unwrap();
        let s = oc.to_string();
        assert!(s.starts_with("Options set:\n? (help): false\nbegin: 5\nc (configuration, configuration-file): <INVALID>\n"), "{s}");
        assert!(s.contains("\nr (route-files, routes): <INVALID>\nsave-configuration.relative: false\nscale: 1\n"), "{s}");
    }

    #[test]
    fn help() {
        let mut oc = sample();
        oc.set("help", "true", false).unwrap();
        let mut s = String::new();
        oc.print_help(&mut s).unwrap();
        let expected = concat!(
            "A microscopic, multi-modal traffic simulation.\n\n",
            "Usage: sumo [OPTION]*\n",
            "Configuration Options:\n",
            "  -c, --configuration-file FILE    Loads the named config on startup\n\n",
            "Input Options:\n",
            "  -n, --net-file FILE              Load road network description from FILE\n",
            "  -r, --route-files FILE           Load routes descriptions from FILE(s)\n",
            "  --begin STR                      Defines the begin time\n",
            "  --scale FLOAT                    Scale demand by the given factor (by\n",
            "                                    discarding or duplicating vehicles)\n\n",
            "Report Options:\n",
            "  -v, --verbose                    Switches to verbose output\n",
            "  -?, --help                       Prints this screen or selected topics\n",
            "  --threads INT                    Number of threads\n\n\n\n",
            "Report bugs at <https://github.com/eclipse-sumo/sumo/issues>.\n",
            "Get in contact via <sumo@dlr.de>.\n"
        );
        assert_eq!(s, expected);
        // a topic given as help value
        let mut oc = sample();
        oc.set("help", "rep", false).unwrap();
        let mut s = String::new();
        oc.print_help(&mut s).unwrap();
        assert!(s.ends_with("\n\nReport Options:\n  -v, --verbose                    Switches to verbose output\n  -?, --help                       Prints this screen or selected topics\n  --threads INT                    Number of threads\n\n"), "{s}");
    }

    #[test]
    fn configuration() {
        let mut oc = sample();
        oc.set("net-file", "a.net.xml", false).unwrap();
        oc.set("verbose", "true", false).unwrap();
        let mut s = String::new();
        oc.write_configuration(&mut s, true, false, false, "", false, false, "").unwrap();
        let expected = concat!(
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\n<!-- -->\n\n",
            "<sumoConfiguration xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" xsi:noNamespaceSchemaLocation=\"http://sumo.dlr.de/xsd/sumoConfiguration.xsd\">\n\n",
            "    <input>\n        <net-file value=\"a.net.xml\"/>\n    </input>\n\n",
            "    <report>\n        <verbose value=\"true\"/>\n    </report>\n\n",
            "</sumoConfiguration>\n"
        );
        assert_eq!(s, expected);
        let mut s = String::new();
        oc.write_configuration(&mut s, false, true, false, "", false, true, "").unwrap();
        assert!(s.contains("<route-files value=\"\" synonymes=\"r routes\" deprecated=\"routes\" type=\"FILE\" help=\"Load routes descriptions from FILE(s)\"/>"), "{s}");
    }

    #[test]
    fn relocate() {
        let mut oc = sample();
        oc.set("net-file", "a.net.xml,/abs/b.net.xml", false).unwrap();
        oc.relocate_files("cfg/x.sumocfg");
        assert_eq!(oc.get_string_vector("net-file").unwrap(), &vec!["cfg/a.net.xml", "/abs/b.net.xml"]);
    }
}
