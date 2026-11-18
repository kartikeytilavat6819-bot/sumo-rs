//! Port of utils/common/FileBucket.{h,cpp}

use super::util_exceptions::ProcessError;

/// C++ `FileBucket::Type` (enum class Type : int, used as a bit set)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FileBucketType(pub i32);

impl FileBucketType {
    pub const SUMO_CONFIG: FileBucketType = FileBucketType(1 << 0);
    pub const NETEDIT_CONFIG: FileBucketType = FileBucketType(1 << 1);
    pub const NETCONVERT_CONFIG: FileBucketType = FileBucketType(1 << 2);
    pub const NETWORK: FileBucketType = FileBucketType(1 << 3);
    pub const EDGETYPE: FileBucketType = FileBucketType(1 << 4);
    pub const TLS: FileBucketType = FileBucketType(1 << 5);
    pub const DEMAND: FileBucketType = FileBucketType(1 << 6);
    pub const MEANDATA: FileBucketType = FileBucketType(1 << 7);
    pub const ADDITIONAL: FileBucketType = FileBucketType(1 << 8);
    pub const DATA: FileBucketType = FileBucketType(1 << 9);
    pub const NETEDIT_PREFIX: FileBucketType = FileBucketType(1 << 10);
    pub const SUMO_PREFIX: FileBucketType = FileBucketType(1 << 11);
    pub const NETCONVERT_PREFIX: FileBucketType = FileBucketType(1 << 12);
    pub const NOTHING: FileBucketType = FileBucketType(1 << 13);

    /// C++ `operator|`
    pub fn or(self, other: FileBucketType) -> FileBucketType {
        FileBucketType(self.0 | other.0)
    }

    /// C++ `operator&` (returns bool)
    pub fn and(self, other: FileBucketType) -> bool {
        (self.0 & other.0) != 0
    }
}

/// C++ `FileBucket::types`
pub const FILE_BUCKET_TYPES: [FileBucketType; 10] = [
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

/// C++ `FileBucket::prefixes`
pub const FILE_BUCKET_PREFIXES: [FileBucketType; 3] =
    [FileBucketType::NETEDIT_PREFIX, FileBucketType::SUMO_PREFIX, FileBucketType::NETCONVERT_PREFIX];

/// C++ class `FileBucket`: links elements with their filenames
pub struct FileBucket {
    my_file_type: FileBucketType,
    my_filename: String,
    my_num_elements: i32,
    my_num_templates: i32,
    my_num_default_v_types: i32,
    my_default_bucket: bool,
}

impl FileBucket {
    /// C++ `FileBucket::FileBucket(Type)`: default bucket (cannot be removed)
    pub fn new(file_type: FileBucketType) -> Self {
        FileBucket {
            my_file_type: file_type,
            my_filename: String::new(),
            my_num_elements: 0,
            my_num_templates: 0,
            my_num_default_v_types: 0,
            my_default_bucket: true,
        }
    }

    /// C++ `FileBucket::FileBucket(Type, const std::string)`
    pub fn new_with_filename(file_type: FileBucketType, filename: &str) -> Self {
        FileBucket {
            my_file_type: file_type,
            my_filename: filename.to_string(),
            my_num_elements: 0,
            my_num_templates: 0,
            my_num_default_v_types: 0,
            my_default_bucket: false,
        }
    }

    /// C++ `getType()`
    pub fn get_type(&self) -> FileBucketType {
        self.my_file_type
    }

    /// C++ `getFilename()`
    pub fn get_filename(&self) -> &str {
        &self.my_filename
    }

    /// C++ `setFilename()`
    pub fn set_filename(&mut self, filename: &str) {
        self.my_filename = filename.to_string();
    }

    /// C++ `isDefaultBucket()`
    pub fn is_default_bucket(&self) -> bool {
        self.my_default_bucket
    }

    /// C++ `addElement(isTemplate)`
    pub fn add_element(&mut self, is_template: bool) {
        if is_template {
            self.my_num_templates += 1;
        } else {
            self.my_num_elements += 1;
        }
    }

    /// C++ `removeElement(isTemplate)`; the messages are swapped in C++ and kept as they are
    pub fn remove_element(&mut self, is_template: bool) -> Result<(), ProcessError> {
        if is_template {
            self.my_num_templates -= 1;
            if self.my_num_templates < 0 {
                return Err(ProcessError::new("Number of element negative"));
            }
        } else {
            self.my_num_elements -= 1;
            if self.my_num_elements < 0 {
                return Err(ProcessError::new("Number of templates negative"));
            }
        }
        Ok(())
    }

    /// C++ `addDefaultVType()`
    pub fn add_default_v_type(&mut self) {
        self.my_num_default_v_types += 1;
    }

    /// C++ `removeDefaultVType()`; C++ checks myNumElements here, kept as it is
    pub fn remove_default_v_type(&mut self) -> Result<(), ProcessError> {
        self.my_num_default_v_types -= 1;
        if self.my_num_elements < 0 {
            return Err(ProcessError::new("Number of defaultVTypes negative"));
        }
        Ok(())
    }

    /// C++ `getNumElements()`
    pub fn get_num_elements(&self) -> i32 {
        self.my_num_elements
    }

    /// C++ `isEmpty()`
    pub fn is_empty(&self) -> bool {
        (self.my_num_elements + self.my_num_templates) == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bucket_counts_follow_cpp() {
        let mut b = FileBucket::new_with_filename(FileBucketType::ADDITIONAL, "a.add.xml");
        assert_eq!(b.get_filename(), "a.add.xml");
        assert!(!b.is_default_bucket());
        assert!(b.is_empty());
        b.add_element(false);
        b.add_element(true);
        assert_eq!(b.get_num_elements(), 1);
        assert!(!b.is_empty());
        b.remove_element(true).unwrap();
        b.remove_element(false).unwrap();
        assert!(b.is_empty());
        // C++ removeElement(false) on a zero count throws "Number of templates negative"
        let err = b.remove_element(false).unwrap_err();
        assert_eq!(err.to_string(), "Number of templates negative");
    }

    #[test]
    fn type_bits() {
        let t = FileBucketType::NETWORK.or(FileBucketType::TLS);
        assert!(t.and(FileBucketType::TLS));
        assert!(!t.and(FileBucketType::DATA));
        assert!(FileBucket::new(FileBucketType::DATA).is_default_bucket());
    }
}
