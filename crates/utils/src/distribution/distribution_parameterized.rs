//! Port of utils/distribution/Distribution_Parameterized.{h,cpp}
use crate::common::msg_handler::write_error;
use crate::common::named::{HasId, Named};
use crate::common::rand_helper::{with_default, SumoRNG};
use crate::common::string_tokenizer::StringTokenizer;
use crate::common::string_utils::to_double_allow_time;
use crate::common::to_string::{join_to_string_prec, to_string, to_string_prec};
use crate::common::util_exceptions::ProcessError;
use crate::geom::NUMERICAL_EPS;
use crate::tlf;

use super::distribution::Distribution;

const INVALID_FORMAT: &str = "Invalid format of distribution parameterized";

/// A distribution described by the mean value and a standard deviation,
/// optionally with cutoff bounds (parameters: mean, deviation[, min[, max]]).
/// An empty parameter list is the "dummy" distribution.
#[derive(Debug, Clone, Default)]
pub struct DistributionParameterized {
    named: Named,
    /// The distribution's parameters
    my_parameter: Vec<f64>,
}

impl HasId for DistributionParameterized {
    fn get_id(&self) -> &str {
        self.named.get_id()
    }
}

impl DistributionParameterized {
    /// Constructor for any temporary distribution parsed directly from the description
    pub fn from_description(description: &str) -> Result<Self, ProcessError> {
        let mut d = DistributionParameterized { named: Named::new(""), my_parameter: vec![0., 0.] };
        d.parse(description, true)?;
        Ok(d)
    }

    /// Constructor for standard normal distribution
    pub fn with_mean_deviation(id: &str, mean: f64, deviation: f64) -> Self {
        DistributionParameterized { named: Named::new(id), my_parameter: vec![mean, deviation] }
    }

    /// Constructor for normal distribution with cutoff
    pub fn with_bounds(id: &str, mean: f64, deviation: f64, min: f64, max: f64) -> Self {
        DistributionParameterized { named: Named::new(id), my_parameter: vec![mean, deviation, min, max] }
    }

    /// Overwrite by parsable distribution description
    pub fn parse(&mut self, description: &str, hard_fail: bool) -> Result<(), ProcessError> {
        match self.try_parse(description) {
            Some(()) => Ok(()),
            None => {
                // set default distribution parameterized
                self.my_parameter = vec![0., 0.];
                if hard_fail {
                    Err(ProcessError::new(INVALID_FORMAT))
                } else {
                    write_error(INVALID_FORMAT);
                    Ok(())
                }
            }
        }
    }

    /// The body of parse; None stands for the C++ exception path
    fn try_parse(&mut self, description: &str) -> Option<()> {
        let dist_name = match description.find('(') {
            Some(i) => &description[..i],
            None => description,
        };
        if dist_name == "norm" || dist_name == "normc" {
            // substr(size, count): C++ unsigned arithmetic makes count wrap, which substr clamps to the rest
            let len = description.len();
            let start = dist_name.len() + 1;
            if start > len {
                return None;
            }
            let count = len.checked_sub(dist_name.len() + 2).unwrap_or(len - start);
            let sub = description.get(start..start + count.min(len - start))?;
            let params = StringTokenizer::with_special(sub, ',' as i32).get_vector();
            let values = params.iter().map(|p| to_double_allow_time(p)).collect::<Result<Vec<f64>, _>>().ok()?;
            self.my_parameter = values;
            self.named.set_id(dist_name);
        } else {
            let v = to_double_allow_time(description).ok()?;
            // C++ writes element 0 of a possibly empty vector here (undefined behaviour); append instead
            if self.my_parameter.is_empty() {
                self.my_parameter.push(v);
            } else {
                self.my_parameter[0] = v;
            }
        }
        if self.my_parameter.len() == 1 {
            self.my_parameter.push(0.);
        }
        Some(())
    }

    /// Returns the minimum value of this distribution
    pub fn get_min(&self) -> f64 {
        if self.my_parameter.is_empty() {
            return f64::NEG_INFINITY;
        }
        if self.my_parameter[1] <= 0. {
            return self.my_parameter[0];
        }
        if self.my_parameter.len() > 2 {
            self.my_parameter[2]
        } else {
            f64::NEG_INFINITY
        }
    }

    /// Returns the nth parameter of this distribution
    pub fn get_parameter(&self, index: usize) -> f64 {
        self.my_parameter[index]
    }

    /// Set a parameter of this distribution
    pub fn set_parameter(&mut self, index: usize, value: f64) -> Result<(), ProcessError> {
        if index >= self.my_parameter.len() {
            return Err(ProcessError::new(tlf!(
                "Attempted to set parameter index % of distribut '%' with % parameters",
                index,
                self.get_id(),
                self.my_parameter.len()
            )));
        }
        self.my_parameter[index] = value;
        Ok(())
    }

    /// check whether the distribution is valid; returns the error text or an empty string
    pub fn is_valid(&self) -> String {
        let p = &self.my_parameter;
        if p.is_empty() {
            return "Dummy Distribution (invalid)".to_string();
        }
        if p.len() >= 2 && p[1] > 0. {
            if self.get_min() > self.get_max() {
                return tlf!("minimum value % larger than maximum %", self.get_min(), self.get_max());
            }
            if self.get_min() > p[0] + 3. * p[1] {
                return tlf!("minimum value % too large for distribution with mean % and deviation %", p[2], p[0], p[1]);
            }
            if self.get_max() < p[0] - 3. * p[1] {
                return tlf!("maximum value % too small for distribution with mean % and deviation %", p[3], p[0], p[1]);
            }
            if p.len() > 3 && p[3] - p[2] < NUMERICAL_EPS * p[1] {
                return tlf!(
                    "maximum value % and minimum value % too close for distribution with mean % and deviation %",
                    p[3],
                    p[2],
                    p[0],
                    p[1]
                );
            }
        }
        String::new()
    }

    pub fn is_dummy(&self) -> bool {
        self.my_parameter.is_empty()
    }

    pub fn is_variable(&self) -> bool {
        !self.my_parameter.is_empty() && self.get_min() != self.get_max()
    }

    /// validate input description; on failure `error` holds the reason
    pub fn is_valid_description(description: &str, error: &mut String) -> bool {
        match Self::from_description(description) {
            Ok(dummy) => {
                *error = dummy.is_valid();
                if error.is_empty() {
                    return true;
                }
            }
            Err(_) => *error = INVALID_FORMAT.to_string(),
        }
        false
    }
}

impl Distribution for DistributionParameterized {
    fn sample(&self, which: Option<&mut SumoRNG>) -> Result<f64, ProcessError> {
        let p = &self.my_parameter;
        if p.is_empty() {
            return Err(ProcessError::new("Attempted to sample from dummy distribution"));
        }
        if p[1] <= 0. {
            return Ok(p[0]);
        }
        let mut which = which;
        let mut val = rand_norm(p[0], p[1], which.as_deref_mut());
        if p.len() > 2 {
            let min = p[2];
            let max = self.get_max();
            while val < min || val > max {
                val = rand_norm(p[0], p[1], which.as_deref_mut());
            }
        }
        Ok(val)
    }

    fn get_max(&self) -> f64 {
        if self.my_parameter.is_empty() {
            return f64::INFINITY;
        }
        if self.my_parameter[1] <= 0. {
            return self.my_parameter[0];
        }
        if self.my_parameter.len() > 3 {
            self.my_parameter[3]
        } else {
            f64::INFINITY
        }
    }

    fn to_str(&self, accuracy: i32) -> String {
        let p = &self.my_parameter;
        let id = self.get_id();
        if p.is_empty() {
            String::new()
        } else if p[1] < 0. {
            // only write simple speedFactor
            to_string(&p[0])
        } else if p[1] == 0. {
            format!("{}({},{})", id, to_string_prec(&p[0], accuracy), to_string_prec(&p[1], accuracy))
        } else {
            format!("{}({})", id, join_to_string_prec(p.iter(), ",", accuracy))
        }
    }
}

/// RandHelper::randNorm(mean, variance, which)
fn rand_norm(mean: f64, variance: f64, which: Option<&mut SumoRNG>) -> f64 {
    match which {
        Some(r) => r.rand_norm(mean, variance),
        None => with_default(|r| r.rand_norm(mean, variance)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_normal_with_bounds() {
        let d = DistributionParameterized::from_description("norm(1,2,0,3)").unwrap();
        assert_eq!(d.get_id(), "norm");
        assert_eq!(d.get_parameter(3), 3.);
        assert_eq!(d.get_min(), 0.);
        assert_eq!(d.get_max(), 3.);
        assert!(d.is_variable());
        assert_eq!(d.is_valid(), "");
        assert_eq!(d.to_str(2), "norm(1.00,2.00,0.00,3.00)");
    }

    #[test]
    fn parse_plain_number() {
        let d = DistributionParameterized::from_description("5").unwrap();
        assert_eq!(d.get_id(), "");
        assert_eq!(d.to_str(2), "(5.00,0.00)");
        assert!(!d.is_variable());
        assert_eq!(d.sample(None).unwrap(), 5.);
    }

    #[test]
    fn invalid_descriptions() {
        let err = DistributionParameterized::from_description("norm(a,b)").unwrap_err();
        assert_eq!(err.to_string(), INVALID_FORMAT);
        let mut error = String::new();
        assert!(!DistributionParameterized::is_valid_description("norm(a,b)", &mut error));
        assert_eq!(error, INVALID_FORMAT);
        assert!(DistributionParameterized::is_valid_description("norm(1,2)", &mut error));
    }

    #[test]
    fn validity_messages() {
        let d = DistributionParameterized::with_bounds("norm", 1., 2., 10., 20.);
        assert_eq!(d.is_valid(), "minimum value 10.00 too large for distribution with mean 1.00 and deviation 2.00");
        let d = DistributionParameterized::with_bounds("norm", 1., 2., 3., 2.);
        assert_eq!(d.is_valid(), "minimum value 3.00 larger than maximum 2.00");
    }

    #[test]
    fn sample_without_deviation_is_mean() {
        let d = DistributionParameterized::with_mean_deviation("norm", 3., 0.);
        assert_eq!(d.sample(None).unwrap(), 3.);
        assert_eq!(d.get_max(), 3.);
    }

    #[test]
    fn dummy_cannot_sample() {
        let d = DistributionParameterized::default();
        assert!(d.is_dummy());
        assert_eq!(d.sample(None).unwrap_err().to_string(), "Attempted to sample from dummy distribution");
    }

    #[test]
    fn set_parameter_out_of_range() {
        let mut d = DistributionParameterized::with_mean_deviation("norm", 3., 1.);
        assert!(d.set_parameter(1, 2.).is_ok());
        assert_eq!(
            d.set_parameter(2, 2.).unwrap_err().to_string(),
            "Attempted to set parameter index 2 of distribut 'norm' with 2 parameters"
        );
    }
}
