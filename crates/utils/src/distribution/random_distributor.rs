//! Port of utils/distribution/RandomDistributor.h
use crate::common::rand_helper::{with_default, SumoRNG};
use crate::common::util_exceptions::ProcessError;

/// Represents a generic random distribution: values with non-negative probabilities
#[derive(Debug, Clone)]
pub struct RandomDistributor<T> {
    /// the total probability
    my_prob: f64,
    /// the members
    my_vals: Vec<T>,
    /// the corresponding probabilities
    my_probs: Vec<f64>,
}

impl<T> Default for RandomDistributor<T> {
    fn default() -> Self {
        RandomDistributor { my_prob: 0., my_vals: Vec::new(), my_probs: Vec::new() }
    }
}

impl<T: PartialEq + Clone> RandomDistributor<T> {
    /// Adds a value with an assigned probability. If the value is already a member and
    /// `check_duplicates` is set, the probability is added to the existing entry.
    /// Returns true if a new value was added.
    pub fn add(&mut self, val: T, prob: f64, check_duplicates: bool) -> bool {
        self.my_prob += prob;
        debug_assert!(self.my_prob >= 0.);
        if check_duplicates {
            for i in 0..self.my_vals.len() {
                if val == self.my_vals[i] {
                    self.my_probs[i] += prob;
                    debug_assert!(self.my_probs[i] >= 0.);
                    return false;
                }
            }
        } else {
            debug_assert!(prob >= 0.);
        }
        self.my_vals.push(val);
        self.my_probs.push(prob);
        true
    }

    /// Removes the first occurrence of a value. Returns true if it was found.
    pub fn remove(&mut self, val: &T) -> bool {
        for i in 0..self.my_vals.len() {
            if self.my_vals[i] == *val {
                self.my_prob -= self.my_probs[i];
                self.my_probs.remove(i);
                self.my_vals.remove(i);
                return true;
            }
        }
        false
    }

    /// Draw a sample; `None` uses the default RNG
    pub fn get(&self, which: Option<&mut SumoRNG>) -> Result<T, ProcessError> {
        if self.my_prob == 0. {
            return Err(ProcessError::out_of_bounds());
        }
        // RandHelper::rand(myProb, which)
        let mut prob = match which {
            Some(r) => r.rand_max(self.my_prob),
            None => with_default(|r| r.rand_max(self.my_prob)),
        };
        for i in 0..self.my_vals.len() {
            if prob < self.my_probs[i] {
                return Ok(self.my_vals[i].clone());
            }
            prob -= self.my_probs[i];
        }
        // C++ returns myVals.back() here, which is undefined on an empty distribution
        self.my_vals.last().cloned().ok_or_else(ProcessError::out_of_bounds)
    }

    /// Return the sum of the probabilities assigned to the members
    pub fn get_overall_prob(&self) -> f64 {
        self.my_prob
    }

    /// Clears the distribution
    pub fn clear(&mut self) {
        self.my_prob = 0.;
        self.my_vals.clear();
        self.my_probs.clear();
    }

    /// The members of the distribution (see get_probs)
    pub fn get_vals(&self) -> &[T] {
        &self.my_vals
    }

    /// The probabilities assigned to the members (see get_vals)
    pub fn get_probs(&self) -> &[f64] {
        &self.my_probs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_remove_and_duplicates() {
        let mut d = RandomDistributor::<f64>::default();
        assert!(d.add(1.0, 0.5, true));
        assert!(!d.add(1.0, 0.25, true));
        assert!(d.add(2.0, 1.0, true));
        assert_eq!(d.get_vals(), &[1.0, 2.0]);
        assert_eq!(d.get_probs(), &[0.75, 1.0]);
        assert!((d.get_overall_prob() - 1.75).abs() < 1e-12);
        assert!(d.remove(&1.0));
        assert!(!d.remove(&1.0));
        assert_eq!(d.get_vals(), &[2.0]);
        assert!((d.get_overall_prob() - 1.0).abs() < 1e-12);
    }

    #[test]
    fn get_from_empty_fails() {
        let d = RandomDistributor::<f64>::default();
        assert_eq!(d.get(None).unwrap_err().to_string(), "Out Of Bounds");
    }

    #[test]
    fn get_single_member_always_returns_it() {
        let mut d = RandomDistributor::<i32>::default();
        d.add(7, 2.0, true);
        let mut rng = SumoRNG::new("t");
        for _ in 0..10 {
            assert_eq!(d.get(Some(&mut rng)).unwrap(), 7);
        }
    }
}
