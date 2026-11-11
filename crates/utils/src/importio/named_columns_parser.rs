//! Port of utils/importio/NamedColumnsParser.{h,cpp}
use std::collections::BTreeMap;

use crate::common::string_tokenizer::StringTokenizer;
use crate::common::string_utils::to_lower_case;
use crate::common::util_exceptions::ProcessError;

/// A parser to retrieve information from a table with known columns.
///
/// Initialise with e.g. "Name;PositionX;PositionY", parse each line with `parse_line`
/// (e.g. "Dummy;0;0"), and get values by name with `get`.
#[derive(Debug, Default)]
pub struct NamedColumnsParser {
    /// The map of column item names to their positions within the table
    my_definitions_map: BTreeMap<String, i32>,
    /// The delimiter to split the column items on
    my_line_delimiter: String,
    /// The contents of the current line
    my_line_parser: StringTokenizer,
    /// Whether case insensitive match shall be done
    my_am_case_insensitive: bool,
}

impl NamedColumnsParser {
    /// Constructor that does nothing; a later call to reinit is necessary
    pub fn new() -> Self {
        Self::default()
    }

    /// Constructor that initialises the parser (see reinit)
    pub fn with_definition(def: &str, def_delim: &str, line_delim: &str, prune: bool, ignore_case: bool) -> Self {
        let mut p = NamedColumnsParser {
            my_line_delimiter: line_delim.to_string(),
            my_am_case_insensitive: ignore_case,
            ..Default::default()
        };
        p.reinit_map(def, def_delim, prune);
        p
    }

    /// Reinitialises the parser
    pub fn reinit(&mut self, def: &str, def_delim: &str, line_delim: &str, prune: bool, ignore_case: bool) {
        self.my_am_case_insensitive = ignore_case;
        self.reinit_map(def, def_delim, prune);
        self.my_line_delimiter = line_delim.to_string();
    }

    /// Parses the contents of the line using the line delimiter
    pub fn parse_line(&mut self, line: &str) {
        self.my_line_parser = StringTokenizer::with_token(line, &self.my_line_delimiter, false);
    }

    /// Returns the named information of the parsed line.
    /// Errors: UnknownElement if the name was not defined, OutOfBounds if the line is too short.
    pub fn get(&self, name: &str, prune: bool) -> Result<String, ProcessError> {
        let mut pos = self.my_definitions_map.get(name).copied();
        if pos.is_none() && self.my_am_case_insensitive {
            pos = self.my_definitions_map.get(&to_lower_case(name)).copied();
        }
        let Some(pos) = pos else {
            return Err(ProcessError::UnknownElement(format!("Element '{name}' is missing")));
        };
        if self.my_line_parser.size() <= pos as usize {
            return Err(ProcessError::out_of_bounds());
        }
        let mut ret = self.my_line_parser.get(pos as usize)?;
        check_prune(&mut ret, prune);
        Ok(ret)
    }

    /// Returns whether the named column is known and present in the parsed line
    pub fn know(&self, name: &str) -> bool {
        let mut pos = self.my_definitions_map.get(name).copied();
        if pos.is_none() && self.my_am_case_insensitive {
            pos = self.my_definitions_map.get(&to_lower_case(name)).copied();
        }
        match pos {
            None => false,
            Some(pos) => self.my_line_parser.size() > pos as usize,
        }
    }

    /// Returns whether the number of named columns matches the actual number
    pub fn has_full_definition(&self) -> bool {
        self.my_definitions_map.len() == self.my_line_parser.size()
    }

    /// Rebuilds the map of attribute names to their positions in a table
    fn reinit_map(&mut self, def: &str, delim: &str, prune: bool) {
        let s = if self.my_am_case_insensitive { to_lower_case(def) } else { def.to_string() };
        self.my_definitions_map.clear();
        let mut pos = 0;
        for mut next in StringTokenizer::with_token(&s, delim, false).get_vector() {
            check_prune(&mut next, prune);
            // std::map::insert keeps the first position of a duplicate name
            self.my_definitions_map.entry(next).or_insert(pos);
            pos += 1;
        }
    }
}

/// Removes leading and trailing spaces. Like upstream, a string of only spaces is left unchanged.
fn check_prune(s: &mut String, prune: bool) {
    if !prune || s.chars().all(|c| c == ' ') {
        return;
    }
    let front = s.len() - s.trim_start_matches(' ').len();
    if front > 0 {
        s.drain(..front);
    }
    let back = s.trim_end_matches(' ').len();
    s.truncate(back);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn named_lookup_is_case_insensitive() {
        let mut p = NamedColumnsParser::with_definition("Name;PositionX;PositionY", ";", ";", false, true);
        p.parse_line("Dummy;0;0");
        assert_eq!(p.get("PositionX", false).unwrap(), "0");
        assert_eq!(p.get("name", false).unwrap(), "Dummy");
        assert!(p.know("PositionY"));
        assert!(!p.know("Unknown"));
        assert!(p.has_full_definition());
        assert_eq!(p.get("Nope", false).unwrap_err().to_string(), "Element 'Nope' is missing");
    }

    #[test]
    fn short_line_is_out_of_bounds() {
        let mut p = NamedColumnsParser::with_definition("a;b;c", ";", ";", false, false);
        p.parse_line("1;2");
        assert_eq!(p.get("b", false).unwrap(), "2");
        assert!(!p.know("c"));
        assert_eq!(p.get("c", false).unwrap_err().to_string(), "Out Of Bounds");
        assert!(!p.has_full_definition());
    }

    #[test]
    fn prune_and_duplicates() {
        let mut p = NamedColumnsParser::with_definition(" a ; a ;b", ";", ",", true, false);
        p.parse_line(" x ,y, z ");
        assert_eq!(p.get("a", false).unwrap(), " x ");
        assert_eq!(p.get("a", true).unwrap(), "x");
        assert_eq!(p.get("b", true).unwrap(), "z");
        assert_eq!(p.get("b", false).unwrap(), " z ");
    }

    #[test]
    fn check_prune_keeps_all_space_strings() {
        let mut s = "   ".to_string();
        check_prune(&mut s, true);
        assert_eq!(s, "   ");
        let mut s = "  a b  ".to_string();
        check_prune(&mut s, true);
        assert_eq!(s, "a b");
    }
}
