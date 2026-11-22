//! Port of utils/options/OptionsParser.{h,cpp}
//! `parse` works on the global OptionsCont like the C++ code; `parse_with` takes an explicit one.
use super::options_cont::{with_options, OptionsCont};
use crate::common::msg_handler::write_error;
use crate::common::util_exceptions::ProcessError;
use crate::write_errorf;

/// std::string::operator[] semantics: the terminating '\0' at size()
fn at(s: &str, i: usize) -> u8 {
    s.as_bytes().get(i).copied().unwrap_or(0)
}

/// Parses the given command line arguments (args[0] is the program name) into the global options.
pub fn parse(args: &[String], ignore_appenders: bool) -> bool {
    with_options(|oc| parse_with(oc, args, ignore_appenders))
}

pub fn parse_with(oc: &mut OptionsCont, args: &[String], ignore_appenders: bool) -> bool {
    let mut ok = true;
    let argc = args.len();
    let mut i = 1;
    while i < argc {
        let arg2 = if i < argc - 1 { Some(args[i + 1].as_str()) } else { None };
        match check(oc, &args[i], arg2, &mut ok, ignore_appenders) {
            Ok(add) => i += add,
            Err(e) => {
                write_error(&format!("On processing option '{}':\n {}", args[i], e));
                i += 1;
                ok = false;
            }
        }
    }
    ok
}

fn check(oc: &mut OptionsCont, arg1: &str, arg2: Option<&str>, ok: &mut bool, ignore_appenders: bool) -> Result<usize, ProcessError> {
    // the first argument should be an option
    // (only the second may be a free string)
    if !check_parameter(arg1) {
        *ok = false;
        return Ok(1);
    }
    let append = at(arg1, 0) == b'+';
    // process not abbreviated switches
    if append || at(arg1, 1) == b'-' {
        let tmp = &arg1[if append { 1 } else { 2 }..];
        let idx1 = tmp.find('=');
        if append && ignore_appenders {
            return Ok(if idx1.is_none() { 2 } else { 1 });
        }
        // check whether a parameter was submitted
        if let Some(idx1) = idx1 {
            *ok &= oc.set(&tmp[..idx1], &tmp[idx1 + 1..], append)?;
        } else if arg2.is_none() || (oc.is_bool(tmp)? && at(arg2.unwrap(), 0) == b'-') {
            *ok &= oc.set(tmp, "true", false)?;
        } else {
            *ok &= oc.set(tmp, arg2.unwrap(), append)?;
            return Ok(2);
        }
        return Ok(1);
    }
    // go through the abbreviated switches
    let len = arg1.len();
    for (i, c) in arg1.char_indices().skip(1) {
        // set boolean switches
        let abbr = c.to_string();
        let last = i + c.len_utf8() == len;
        if oc.is_bool(&abbr)? {
            if arg2.is_none() || at(arg2.unwrap(), 0) == b'-' || !last {
                *ok &= oc.set(&abbr, "true", false)?;
            } else {
                *ok &= oc.set(&abbr, arg2.unwrap(), false)?;
                return Ok(2);
            }
            // set non-boolean switches
        } else {
            // check whether the parameter comes directly after the switch
            //  and process if so
            if arg2.is_none() || !last {
                *ok &= process_non_boolean_single_switch(oc, &arg1[i..], append)?;
                return Ok(1);
                // process parameter following after a space
            } else {
                *ok &= oc.set(&abbr, arg2.unwrap(), append)?;
                // option name and attribute were in two arguments
                return Ok(2);
            }
        }
    }
    // all switches within the current argument were boolean switches
    Ok(1)
}

fn process_non_boolean_single_switch(oc: &mut OptionsCont, arg: &str, append: bool) -> Result<bool, ProcessError> {
    let first_len = arg.chars().next().map_or(0, char::len_utf8);
    let name = &arg[..first_len];
    if at(arg, first_len) == b'=' {
        if arg.len() < first_len + 2 {
            write_errorf!("Missing value for parameter '%'.", name);
            Ok(false)
        } else {
            oc.set(name, &arg[first_len + 1..], append)
        }
    } else if arg.len() < first_len + 1 {
        write_errorf!("Missing value for parameter '%'.", arg);
        Ok(false)
    } else {
        oc.set(name, &arg[first_len..], append)
    }
}

fn check_parameter(arg1: &str) -> bool {
    let (c0, c1) = (at(arg1, 0), at(arg1, 1));
    if c0 != b'-' && c0 != b'+' {
        write_errorf!("The parameter '%' is not allowed in this context.\n Switch or parameter name expected.", arg1);
        return false;
    }
    if (c0 == b'-' && c1 == b'+') || (c0 == b'+' && c1 == b'-') {
        write_errorf!("Mixed parameter syntax in '%'.", arg1);
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::options::options_cont::tests::sample;

    fn args(a: &[&str]) -> Vec<String> {
        std::iter::once("sumo").chain(a.iter().copied()).map(String::from).collect()
    }

    #[test]
    fn long_forms() {
        let mut oc = sample();
        assert!(parse_with(&mut oc, &args(&["--net-file=a.xml", "--begin", "10", "--verbose", "--scale", "2"]), false));
        assert_eq!(oc.get_string("net-file").unwrap(), "a.xml");
        assert_eq!(oc.get_string("begin").unwrap(), "10");
        assert!(oc.get_bool("verbose").unwrap());
        assert_eq!(oc.get_float("scale").unwrap(), 2.);
    }

    #[test]
    fn bool_followed_by_option() {
        let mut oc = sample();
        assert!(parse_with(&mut oc, &args(&["--verbose", "--begin", "3"]), false));
        assert!(oc.get_bool("verbose").unwrap());
        assert_eq!(oc.get_string("begin").unwrap(), "3");
        let mut oc = sample();
        assert!(parse_with(&mut oc, &args(&["--verbose", "false"]), false));
        assert!(!oc.get_bool("verbose").unwrap());
    }

    #[test]
    fn abbreviations() {
        let mut oc = sample();
        assert!(parse_with(&mut oc, &args(&["-n", "a.xml", "-v", "-rb.xml"]), false));
        assert_eq!(oc.get_string("net-file").unwrap(), "a.xml");
        assert!(oc.get_bool("v").unwrap());
        assert_eq!(oc.get_string("route-files").unwrap(), "b.xml");
        let mut oc = sample();
        // combined bool switch and value switch
        assert!(parse_with(&mut oc, &args(&["-vn", "x.xml"]), false));
        assert!(oc.get_bool("verbose").unwrap());
        assert_eq!(oc.get_string("net-file").unwrap(), "x.xml");
        let mut oc = sample();
        assert!(parse_with(&mut oc, &args(&["-n=y.xml"]), false));
        assert_eq!(oc.get_string("net-file").unwrap(), "y.xml");
        let mut oc = sample();
        assert!(!parse_with(&mut oc, &args(&["-n="]), false));
    }

    #[test]
    fn synonymes_and_appenders() {
        let mut oc = sample();
        assert!(parse_with(&mut oc, &args(&["--net", "a.xml", "--routes", "r1.xml"]), false));
        assert_eq!(oc.get_string("net-file").unwrap(), "a.xml");
        oc.reset_writable();
        assert!(parse_with(&mut oc, &args(&["+route-files", "r2.xml"]), false));
        assert_eq!(oc.get_string_vector("route-files").unwrap(), &vec!["r1.xml", "r2.xml"]);
        // appenders are skipped (with their value) when ignored
        let mut oc = sample();
        assert!(parse_with(&mut oc, &args(&["+route-files", "r2.xml", "-v"]), true));
        assert!(!oc.is_set("route-files", true).unwrap());
        assert!(oc.get_bool("verbose").unwrap());
    }

    #[test]
    fn meta_option_forms() {
        let setup = |oc: &mut OptionsCont| {
            oc.do_register_abbr("additional-files", 'a', crate::options::option::SumoOption::file_name());
            oc.do_register_abbr("save-configuration", 'C', crate::options::option::SumoOption::file_name());
        };
        // meta/option_list_equal_sign: -a=FILE (short option with '=')
        let mut oc = sample();
        setup(&mut oc);
        assert!(parse_with(&mut oc, &args(&["-a=a1.xml,a2.xml"]), false));
        assert_eq!(oc.get_string("additional-files").unwrap(), "a1.xml,a2.xml");
        // meta/option_list_append: +a appends to an existing list
        let mut oc = sample();
        setup(&mut oc);
        assert!(parse_with(&mut oc, &args(&["-a", "a1.xml"]), false));
        oc.reset_writable(); // the real parser resets between config and command line
        assert!(parse_with(&mut oc, &args(&["+a", "a2.xml"]), false));
        assert_eq!(oc.get_string("additional-files").unwrap(), "a1.xml,a2.xml");
        // meta config-writing forms: --save-configuration=FILE, -C FILE (alias)
        let mut oc = sample();
        setup(&mut oc);
        assert!(parse_with(&mut oc, &args(&["--save-configuration=cfg.xml"]), false));
        assert_eq!(oc.get_string("save-configuration").unwrap(), "cfg.xml");
        let mut oc = sample();
        setup(&mut oc);
        assert!(parse_with(&mut oc, &args(&["-C", "cfg.xml"]), false));
        assert_eq!(oc.get_string("save-configuration").unwrap(), "cfg.xml");
    }

    #[test]
    fn errors() {
        let mut oc = sample();
        assert!(!parse_with(&mut oc, &args(&["free"]), false));
        assert!(!parse_with(&mut oc, &args(&["-+x"]), false));
        assert!(!parse_with(&mut oc, &args(&["--unknown", "1"]), false));
        assert!(!parse_with(&mut oc, &args(&["--threads", "x"]), false));
        // setting twice is reported
        assert!(parse_with(&mut oc, &args(&["--begin", "1"]), false));
        assert!(!parse_with(&mut oc, &args(&["--begin", "2"]), false));
    }
}
