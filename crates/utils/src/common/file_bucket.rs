//! Port of utils/common/FileBucket.{h,cpp}
use crate::common::util_exceptions::ProcessError;

/// C++ `FileBucket::Type` (a bit mask)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FileBucketType(pub i32);

impl FileBucketType {
    /// Bucket for sumo configs (usually only one)
    pub const SUMO_CONFIG: FileBucketType = FileBucketType(1 << 0);
    /// Bucket for netedit configs (usually only one)
    pub const NETEDIT_CONFIG: FileBucketType = FileBucketType(1 << 1);
    /// Bucket for netconvert configs (usually only one)
    pub const NETCONVERT_CONFIG: FileBucketType = FileBucketType(1 << 2);
    /// Bucket for network elements (usually only one)
    pub const NETWORK: FileBucketType = FileBucketType(1 << 3);
    /// Bucket for edge type (usually only one)
    pub const EDGETYPE: FileBucketType = FileBucketType(1 << 4);
    /// Bucket for traffic lights (usually only one)
    pub const TLS: FileBucketType = FileBucketType(1 << 5);
    /// Bucket for demand elements
    pub const DEMAND: FileBucketType = FileBucketType(1 << 6);
    /// Bucket for meanData elements
    pub const MEANDATA: FileBucketType = FileBucketType(1 << 7);
    /// Bucket for additional elements (always after demand and meanData)
    pub const ADDITIONAL: FileBucketType = FileBucketType(1 << 8);
    /// Bucket for data elements
    pub const DATA: FileBucketType = FileBucketType(1 << 9);
    /// prefixes
    pub const NETEDIT_PREFIX: FileBucketType = FileBucketType(1 << 10);
    pub const SUMO_PREFIX: FileBucketType = FileBucketType(1 << 11);
    pub const NETCONVERT_PREFIX: FileBucketType = FileBucketType(1 << 12);
    /// Element is not saved in bucket
    pub const NOTHING: FileBucketType = FileBucketType(1 << 13);
}

/// C++ `constexpr FileBucket::Type operator|(FileBucket::Type, FileBucket::Type)`
impl std::ops::BitOr for FileBucketType {
    type Output = FileBucketType;
    fn bitor(self, b: FileBucketType) -> FileBucketType {
        FileBucketType(self.0 | b.0)
    }
}

/// C++ `constexpr bool operator&(FileBucket::Type, FileBucket::Type)`
impl std::ops::BitAnd for FileBucketType {
    type Output = bool;
    fn bitand(self, b: FileBucketType) -> bool {
        (self.0 & b.0) != 0
    }
}

/// C++ `FileBucket::types` (static member)
pub const TYPES: [FileBucketType; 10] = [
    FileBucketType::SUMO_CONFIG,
    FileBucketType::NETEDIT_CONFIG,
    FileBucketType::NETCONVERT_CONFIG,
    FileBucketType::NETWORK,
    FileBucketType::EDGETYPE,
    FileBucketType::TLS,
    FileBucketType::DEMAND,
    FileBucketType::MEANDATA,
    FileBucketType::ADDITIONAL,
    FileBucketType::DATA,
];

/// C++ `FileBucket::prefixes` (static member)
pub const PREFIXES: [FileBucketType; 3] = [FileBucketType::NETEDIT_PREFIX, FileBucketType::SUMO_PREFIX, FileBucketType::NETCONVERT_PREFIX];

/// class used for link Elements with their filenames
pub struct FileBucket {
    my_file_type: FileBucketType,
    my_filename: String,
    my_num_elements: i32,
    my_num_templates: i32,
    my_num_default_vtypes: i32,
    my_default_bucket: bool,
}

impl FileBucket {
    /// C++ `FileBucket::FileBucket(FileBucket::Type type)`
    pub fn new(type_: FileBucketType) -> Self {
        FileBucket { my_file_type: type_, my_filename: String::new(), my_num_elements: 0, my_num_templates: 0, my_num_default_vtypes: 0, my_default_bucket: true }
    }

    /// C++ `FileBucket::FileBucket(FileBucket::Type type, const std::string filename)`
    pub fn new_with_filename(type_: FileBucketType, filename: &str) -> Self {
        FileBucket { my_file_type: type_, my_filename: filename.to_string(), my_num_elements: 0, my_num_templates: 0, my_num_default_vtypes: 0, my_default_bucket: false }
    }

    /// C++ `FileBucket::getType`
    pub fn get_type(&self) -> FileBucketType {
        self.my_file_type
    }

    /// C++ `FileBucket::getFilename`
    pub fn get_filename(&self) -> &str {
        &self.my_filename
    }

    /// C++ `FileBucket::setFilename`
    pub fn set_filename(&mut self, filename: &str) {
        self.my_filename = filename.to_string();
    }

    /// C++ `FileBucket::isDefaultBucket`
    pub fn is_default_bucket(&self) -> bool {
        self.my_default_bucket
    }

    /// C++ `FileBucket::addElement`
    pub fn add_element(&mut self, is_template: bool) {
        if is_template {
            self.my_num_templates += 1;
        } else {
            self.my_num_elements += 1;
        }
    }

    /// C++ `FileBucket::removeElement` (the two error texts are kept as in the C++ source)
    pub fn remove_element(&mut self, is_template: bool) -> Result<(), ProcessError> {
        if is_template {
            self.my_num_templates -= 1;
            // check that number of elementes are not negative
            if self.my_num_templates < 0 {
                return Err(ProcessError::new("Number of element negative"));
            }
        } else {
            self.my_num_elements -= 1;
            // check that number of elementes are not negative
            if self.my_num_elements < 0 {
                return Err(ProcessError::new("Number of templates negative"));
            }
        }
        Ok(())
    }

    /// C++ `FileBucket::addDefaultVType`
    pub fn add_default_vtype(&mut self) {
        self.my_num_default_vtypes += 1;
    }

    /// C++ `FileBucket::removeDefaultVType` (the check reads myNumElements, as in the C++ source)
    pub fn remove_default_vtype(&mut self) -> Result<(), ProcessError> {
        self.my_num_default_vtypes -= 1;
        // check that number of elementes are not negative
        if self.my_num_elements < 0 {
            return Err(ProcessError::new("Number of defaultVTypes negative"));
        }
        Ok(())
    }

    /// C++ `FileBucket::getNumElements`
    pub fn get_num_elements(&self) -> i32 {
        self.my_num_elements
    }

    /// C++ `FileBucket::isEmpty`
    pub fn is_empty(&self) -> bool {
        (self.my_num_elements + self.my_num_templates) == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn element_counts_and_negative_checks() {
        let mut b = FileBucket::new_with_filename(FileBucketType::NETWORK, "net.xml");
        assert!(!b.is_default_bucket());
        assert_eq!(b.get_filename(), "net.xml");
        assert!(b.is_empty());
        b.add_element(false);
        assert!(!b.is_empty());
        assert_eq!(b.get_num_elements(), 1);
        assert!(b.remove_element(false).is_ok());
        assert!(b.is_empty());
        match b.remove_element(true) {
            Err(e) => assert_eq!(e.to_string(), "Number of element negative"),
            Ok(_) => panic!("expected an error for a negative template count"),
        }
        match b.remove_element(false) {
            Err(e) => assert_eq!(e.to_string(), "Number of templates negative"),
            Ok(_) => panic!("expected an error for a negative element count"),
        }
    }

    #[test]
    fn type_mask_operators() {
        assert!((FileBucketType::SUMO_CONFIG | FileBucketType::TLS) & FileBucketType::TLS);
        assert!(!(FileBucketType::NETWORK & FileBucketType::TLS));
        assert_eq!((FileBucketType::SUMO_CONFIG | FileBucketType::TLS).0, 33);
        assert!(FileBucket::new(FileBucketType::DATA).is_default_bucket());
        assert_eq!(TYPES.len(), 10);
        assert_eq!(PREFIXES.len(), 3);
    }
}
