//! What iodevices needs from OptionsCont::getOptions(), plus the port of OptionsCont::writeXMLHeader.
// shortcut: replace with utils::options::OptionsCont once merged; until then the options port
// registers itself with `set_options_hook`
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

/// Stands for an OptionsCont without any registered option.
struct NoOptions;
impl OptionsHook for NoOptions {
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
        String::new()
    }
    fn write_configuration(&self, _: &mut dyn Write, _: &str) {}
}

static OPTIONS: RwLock<Option<Arc<dyn OptionsHook>>> = RwLock::new(None);

pub fn set_options_hook(hook: Arc<dyn OptionsHook>) {
    *OPTIONS.write().unwrap_or_else(|e| e.into_inner()) = Some(hook);
}

pub fn options() -> Arc<dyn OptionsHook> {
    OPTIONS.read().unwrap_or_else(|e| e.into_inner()).clone().unwrap_or_else(|| Arc::new(NoOptions))
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
