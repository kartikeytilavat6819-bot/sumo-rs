//! Port of utils/iodevices/OutputFormatter.h
//! Only PlainXMLFormatter exists, so OutputDevice holds it directly instead of a formatter trait object.
// shortcut: CSVFormatter and ParquetFormatter are not ported; add them (and a trait) when csv/parquet output is needed

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormatterType {
    XML,
    CSV,
}
