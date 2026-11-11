//! Port of utils/common/UtilExceptions.{h,cpp}
//! The C++ class hierarchy becomes one enum; `Display` yields the same `what()` text.
// shortcut: ProcessError::getTrace (backtrace) is not ported, port it if debugging needs it
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum ProcessError {
    /// ProcessError(msg)
    Process(String),
    InvalidArgument(String),
    EmptyData,
    /// FormatException(msg)
    Format(String),
    /// NumberFormatException(data)
    NumberFormat(String),
    /// TimeFormatException(data)
    TimeFormat(String),
    /// BoolFormatException(data)
    BoolFormat(String),
    OutOfBounds(String),
    UnknownElement(String),
    IOError(String),
}

impl ProcessError {
    /// NumberFormatException(data)
    pub fn number_format(data: &str) -> Self {
        ProcessError::NumberFormat(data.into())
    }
    /// BoolFormatException(data)
    pub fn bool_format(data: &str) -> Self {
        ProcessError::BoolFormat(data.into())
    }
    pub fn new(msg: impl Into<String>) -> Self {
        ProcessError::Process(msg.into())
    }
    /// ProcessError() default constructor
    pub fn default_msg() -> Self {
        ProcessError::Process("Process Error".into())
    }
    pub fn out_of_bounds() -> Self {
        ProcessError::OutOfBounds("Out Of Bounds".into())
    }
    pub fn unknown_element() -> Self {
        ProcessError::UnknownElement("Unknown Element".into())
    }
    /// true for FormatException and its subclasses (catch (FormatException&))
    pub fn is_format(&self) -> bool {
        matches!(
            self,
            ProcessError::Format(_) | ProcessError::NumberFormat(_) | ProcessError::TimeFormat(_) | ProcessError::BoolFormat(_)
        )
    }
}

impl fmt::Display for ProcessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProcessError::Process(m)
            | ProcessError::InvalidArgument(m)
            | ProcessError::Format(m)
            | ProcessError::OutOfBounds(m)
            | ProcessError::UnknownElement(m)
            | ProcessError::IOError(m) => f.write_str(m),
            ProcessError::EmptyData => f.write_str("Empty Data"),
            ProcessError::NumberFormat(d) => write!(f, "Invalid Number Format {d}"),
            ProcessError::TimeFormat(d) => write!(f, "Invalid Time Format {d}"),
            ProcessError::BoolFormat(d) => write!(f, "Invalid Bool Format {d}"),
        }
    }
}

impl std::error::Error for ProcessError {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn messages() {
        assert_eq!(ProcessError::NumberFormat("x".into()).to_string(), "Invalid Number Format x");
        assert_eq!(ProcessError::EmptyData.to_string(), "Empty Data");
        assert!(ProcessError::BoolFormat("x".into()).is_format());
    }
}
