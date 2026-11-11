//! Port of utils/distribution/Distribution.h
//! The virtual interface of Distribution; the subclasses are the structs in
//! distribution_parameterized.rs and distribution_points.rs.
use crate::common::named::HasId;
use crate::common::rand_helper::SumoRNG;
use crate::common::util_exceptions::ProcessError;

/// The base class for distribution descriptions (Named is provided via HasId)
pub trait Distribution: HasId + Send + Sync {
    /// Draw a sample of the distribution; `None` uses the default RNG
    fn sample(&self, which: Option<&mut SumoRNG>) -> Result<f64, ProcessError>;

    /// Returns the maximum value of this distribution
    fn get_max(&self) -> f64;

    /// Returns the string representation of this distribution
    fn to_str(&self, accuracy: i32) -> String;
}
