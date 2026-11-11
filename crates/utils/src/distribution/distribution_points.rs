//! Port of utils/distribution/Distribution_Points.{h,cpp}
use std::ops::{Deref, DerefMut};

use crate::common::named::{HasId, Named};
use crate::common::rand_helper::SumoRNG;
use crate::common::to_string::format_g;
use crate::common::util_exceptions::ProcessError;

use super::distribution::Distribution;
use super::random_distributor::RandomDistributor;

/// A description of a distribution by a set of points: each x-value with its probability.
/// The C++ class inherits RandomDistributor<double>; here it is held as a field and
/// reachable through Deref, so add/remove/getVals/... keep working on it.
#[derive(Debug, Clone)]
pub struct DistributionPoints {
    named: Named,
    dist: RandomDistributor<f64>,
}

impl DistributionPoints {
    pub fn new(id: &str) -> Self {
        DistributionPoints { named: Named::new(id), dist: RandomDistributor::default() }
    }
}

impl Deref for DistributionPoints {
    type Target = RandomDistributor<f64>;
    fn deref(&self) -> &Self::Target {
        &self.dist
    }
}

impl DerefMut for DistributionPoints {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.dist
    }
}

impl HasId for DistributionPoints {
    fn get_id(&self) -> &str {
        self.named.get_id()
    }
}

impl Distribution for DistributionPoints {
    fn sample(&self, which: Option<&mut SumoRNG>) -> Result<f64, ProcessError> {
        self.dist.get(which)
    }

    fn get_max(&self) -> f64 {
        // C++ asserts a non-empty distribution, then returns getVals().back()
        *self.dist.get_vals().last().expect("getMax on an empty points distribution")
    }

    fn to_str(&self, accuracy: i32) -> String {
        // oss << setprecision(accuracy) prints doubles with %g; a negative precision means 6 in libstdc++
        let prec = if accuracy < 0 { 6 } else { accuracy as usize };
        let vals = self.dist.get_vals();
        let probs = self.dist.get_probs();
        let mut oss = String::new();
        for i in 0..vals.len() {
            if i > 0 {
                oss += ",";
            }
            oss += &format_g(vals[i], prec);
            oss += ":";
            oss += &format_g(probs[i], prec);
        }
        format!("points({oss})")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn points_string_and_max() {
        let mut d = DistributionPoints::new("pts");
        d.add(1.5, 0.5, true);
        d.add(2., 0.5, true);
        assert_eq!(d.to_str(3), "points(1.5:0.5,2:0.5)");
        assert_eq!(d.get_max(), 2.);
        assert_eq!(d.get_id(), "pts");
        assert!(d.remove(&2.));
        assert_eq!(d.get_vals(), &[1.5]);
    }

    #[test]
    fn sample_single_point() {
        let mut d = DistributionPoints::new("pts");
        d.add(4., 1., true);
        assert_eq!(d.sample(None).unwrap(), 4.);
    }

    #[test]
    fn sample_empty_fails() {
        let d = DistributionPoints::new("pts");
        assert!(d.sample(None).is_err());
    }
}
