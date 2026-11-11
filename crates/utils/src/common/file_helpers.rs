//! Port of utils/common/FileHelpers.{h,cpp}
// shortcut: the binary stream writers (writeInt, writeFloat, ...) are not ported, add them with OutputDevice_Binary
use super::string_tokenizer::StringTokenizer;
use super::to_string::join_to_string;
use super::util_exceptions::ProcessError;
use crate::tlf;

pub fn is_readable(path: &str) -> bool {
    let path = path.trim_end_matches(['/', '\\']);
    if path.is_empty() {
        return false;
    }
    // access(path, R_OK): directories count as readable, too
    std::fs::metadata(path).is_ok_and(|m| m.is_dir() || std::fs::File::open(path).is_ok())
}

pub fn is_directory(path: &str) -> Result<bool, ProcessError> {
    std::fs::metadata(path)
        .map(|m| m.is_dir())
        .map_err(|_| ProcessError::new(tlf!("Cannot get file attributes for file '%'!", path)))
}

pub fn get_file_path(path: &str) -> String {
    match path.rfind(['\\', '/']) {
        Some(beg) => path[..beg + 1].to_string(),
        None => String::new(),
    }
}

pub fn get_file_from_path(path: &str, remove_extension: bool) -> String {
    let mut path = path;
    if remove_extension {
        if let Some(beg) = path.rfind('.') {
            path = &path[..beg];
        }
    }
    match path.rfind(['\\', '/']) {
        Some(beg) => path[beg + 1..].to_string(),
        None => path.to_string(),
    }
}

pub fn add_extension(path: &str, extension: &str) -> String {
    if path.is_empty() {
        String::new()
    } else if extension.is_empty() {
        path.to_string()
    } else if path == extension {
        String::new()
    } else if path.len() < extension.len() || !path.ends_with(extension) {
        format!("{path}{extension}")
    } else {
        path.to_string()
    }
}

pub fn get_configuration_relative(config_path: &str, path: &str) -> String {
    get_file_path(config_path) + path
}

pub fn is_socket(name: &str) -> bool {
    match name.find(':') {
        Some(colon_pos) => colon_pos > 1 || name.starts_with('['),
        None => false,
    }
}

pub fn is_absolute(path: &str) -> bool {
    if is_socket(path) {
        return true;
    }
    let b = path.as_bytes();
    if b.first() == Some(&b'/') || b.first() == Some(&b'\\') {
        return true;
    }
    if b.len() > 1 && b[1] == b':' {
        return true;
    }
    path == "nul" || path == "NUL"
}

pub fn check_for_relativity(filename: &str, base_path: &str) -> String {
    if filename == "stdout" || filename == "STDOUT" || filename == "-" {
        return "stdout".into();
    }
    if filename == "stderr" || filename == "STDERR" {
        return "stderr".into();
    }
    if filename == "nul" || filename == "NUL" {
        return "/dev/null".into();
    }
    if !is_absolute(filename) {
        return get_configuration_relative(base_path, filename);
    }
    filename.to_string()
}

pub fn get_current_dir() -> String {
    std::env::current_dir().map(|p| p.to_string_lossy().into_owned()).unwrap_or_default()
}

pub fn split_dirs(filename: &str) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for d in StringTokenizer::with_token(filename, "\\/", true).get_vector() {
        if d == ".." && result.last().is_some_and(|b| b != "..") {
            result.pop();
        } else if (d.is_empty() && result.is_empty()) || (!d.is_empty() && d != ".") {
            result.push(d);
        }
    }
    result
}

pub fn fix_relative(filename: &str, base_path: &str, force: bool, cur_dir: &str) -> String {
    if filename == "stdout" || filename == "STDOUT" || filename == "-" {
        return "stdout".into();
    }
    if filename == "stderr" || filename == "STDERR" {
        return "stderr".into();
    }
    if filename == "nul" || filename == "NUL" || filename == "/dev/null" {
        return "/dev/null".into();
    }
    if is_socket(filename) || (is_absolute(filename) && !force) {
        return filename.to_string();
    }
    let mut file_path_split = split_dirs(filename);
    let mut base_path_split = split_dirs(base_path);
    if is_absolute(filename) || is_absolute(base_path) || base_path_split.first().is_some_and(|b| b == "..") {
        // if at least one is absolute we need to make the other absolute too
        // the same is true if the basePath refers to a parent dir
        let cur_dir = if cur_dir.is_empty() { get_current_dir() } else { cur_dir.to_string() };
        if !is_absolute(filename) {
            file_path_split = split_dirs(&format!("{cur_dir}/{filename}"));
        }
        if !is_absolute(base_path) {
            base_path_split = split_dirs(&format!("{cur_dir}/{base_path}"));
        }
        if file_path_split.first() != base_path_split.first() {
            // don't try to make something relative on different windows disks
            return join_to_string(&file_path_split, "/");
        }
    }
    while !file_path_split.is_empty() && !base_path_split.is_empty() && file_path_split[0] == base_path_split[0] {
        file_path_split.remove(0);
        base_path_split.remove(0);
    }
    for _ in 1..base_path_split.len() {
        file_path_split.insert(0, "..".into());
    }
    join_to_string(&file_path_split, "/")
}

pub fn prepend_to_last_path_component(prefix: &str, path: &str) -> String {
    match path.rfind(['\\', '/']) {
        None => format!("{prefix}{path}"),
        Some(i) => format!("{}{prefix}{}", &path[..i + 1], &path[i + 1..]),
    }
}

pub fn append_before_extension(path: &str, suffix: &str, check_sep: bool) -> String {
    if check_sep {
        return match path.rfind(['\\', '/']) {
            None => append_before_extension(path, suffix, false),
            Some(i) => format!("{}{}", &path[..i + 1], append_before_extension(&path[i + 1..], suffix, false)),
        };
    }
    let mut components = StringTokenizer::with_token(path, ".", false).get_vector();
    for i in (0..components.len()).rev() {
        // assume anything after a dot with less then 5 letters is part of the extension
        if i == 0 || components[i].len() > 4 {
            components[i] += suffix;
            break;
        }
    }
    join_to_string(&components, ".")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn paths() {
        assert_eq!(get_file_path("a/b/c.xml"), "a/b/");
        assert_eq!(get_file_from_path("a/b/c.xml", true), "c");
        assert_eq!(check_for_relativity("x.xml", "cfg/a.sumocfg"), "cfg/x.xml");
        assert_eq!(check_for_relativity("/x.xml", "cfg/a.sumocfg"), "/x.xml");
        assert_eq!(check_for_relativity("-", "cfg/a.sumocfg"), "stdout");
        assert_eq!(fix_relative("cfg/x.xml", "cfg/a.sumocfg", false, ""), "x.xml");
        assert_eq!(fix_relative("x.xml", "cfg/a.sumocfg", false, ""), "../x.xml");
        assert_eq!(fix_relative("/r/x.xml", "/r/s/a.cfg", true, ""), "../x.xml");
        assert_eq!(split_dirs("a/./b/../c"), ["a", "c"]);
        assert_eq!(add_extension("a", ".xml"), "a.xml");
        assert_eq!(add_extension("a.xml", ".xml"), "a.xml");
        assert_eq!(append_before_extension("dir.d/out.xml.gz", "_1", true), "dir.d/out_1.xml.gz");
        assert_eq!(prepend_to_last_path_component("p_", "a/b"), "a/p_b");
        assert!(is_absolute("C:\\x") && is_socket("localhost:80") && !is_absolute("x"));
    }
}
