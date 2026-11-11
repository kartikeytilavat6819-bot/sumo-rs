//! Port of utils/importio/LineHandler.h
/// The interface of a class which retrieves lines from a LineReader
pub trait LineHandler {
    /// Method that obtains a line read by the LineReader.
    /// Returns whether the caller shall continue with reading.
    fn report(&mut self, result: &str) -> bool;
}
