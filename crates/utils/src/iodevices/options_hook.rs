//! What iodevices needs from OptionsCont::getOptions(), plus the port of OptionsCont::writeXMLHeader.
// Production uses the OptionsCont singleton (RealOptions); `set_options_hook` is only a test seam
// that lets the unit tests replace it with fixed values.
use crate::xml::stand_ins::iso_time_string;
use crate::xml::sumo_sax_attributes::ENCODING;
use std::io::Write;
use std::sync::{Arc, RwLock};

pub trait OptionsHook: Send + Sync {
    fn exists(&self, name: &str) -> bool;
    fn is_set(&self, name: &str) -> bool;
    fn get_string(&self, name: &str) -> String;
    fn get_bool(&self, name: &str) -> bool;
    fn get_full_name(&self) -> String;
    /// OptionsCont::writeConfiguration(os, true, false, false, "", false, true, indent)
    fn write_configuration(&self, into: &mut dyn Write, indent: &str);
}

/// OptionsCont::getOptions() (the global options singleton)
struct RealOptions;
impl OptionsHook for RealOptions {
    fn exists(&self, name: &str) -> bool {
        crate::options::options_cont::exists(name)
    }
    fn is_set(&self, name: &str) -> bool {
        crate::options::options_cont::with_options(|oc| oc.is_set(name, false).unwrap_or(false))
    }
    fn get_string(&self, name: &str) -> String {
        crate::options::options_cont::get_string(name)
    }
    fn get_bool(&self, name: &str) -> bool {
        crate::options::options_cont::get_bool(name)
    }
    fn get_full_name(&self) -> String {
        crate::options::options_cont::with_options(|oc| oc.get_full_name().to_string())
    }
    fn write_configuration(&self, into: &mut dyn Write, indent: &str) {
        let mut s = String::new();
        crate::options::options_cont::with_options(|oc| oc.write_configuration(&mut s, true, false, false, "", false, true, indent))
            .unwrap_or_else(|e| panic!("{e}"));
        let _ = into.write_all(s.as_bytes());
    }
}

static OPTIONS: RwLock<Option<Arc<dyn OptionsHook>>> = RwLock::new(None);

pub fn set_options_hook(hook: Arc<dyn OptionsHook>) {
    *OPTIONS.write().unwrap_or_else(|e| e.into_inner()) = Some(hook);
}

pub fn options() -> Arc<dyn OptionsHook> {
    OPTIONS.read().unwrap_or_else(|e| e.into_inner()).clone().unwrap_or_else(|| Arc::new(RealOptions))
}

/// OptionsCont::writeXMLHeader
pub fn write_xml_header(os: &mut dyn Write, include_config: bool) -> std::io::Result<()> {
    let oc = options();
    write!(os, "<?xml version=\"1.0\"{ENCODING}?>\n\n")?;
    write!(os, "<!-- ")?;
    if !oc.get_bool("write-metadata") {
        writeln!(os, "generated on {} by {}", iso_time_string(), oc.get_full_name())?;
    }
    if oc.get_bool("write-license") {
        os.write_all(
            b"This data file and the accompanying materials\n\
are made available under the terms of the Eclipse Public License v2.0\n\
which accompanies this distribution, and is available at\n\
http://www.eclipse.org/legal/epl-v20.html\n\
This file may also be made available under the following Secondary\n\
Licenses when the conditions for such availability set forth in the Eclipse\n\
Public License 2.0 are satisfied: GNU General Public License, version 2\n\
or later which is available at\n\
https://www.gnu.org/licenses/old-licenses/gpl-2.0-standalone.html\n\
SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later\n",
        )?;
    }
    if include_config && !oc.get_bool("write-metadata") {
        oc.write_configuration(os, "");
    }
    write!(os, "-->\n\n")
}
