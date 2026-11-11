//! Port of foreign/RoutingKit/include/routingkit/bit_vector.h and src/bit_vector.cpp.
//! Only the operations used by the CCH and nested dissection code are ported.
//! Storage is padded to whole 512-bit blocks as in C++, so LocalIDMapper can read
//! 8-word blocks without bounds checks; padding bits are always zero.

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct BitVector {
    data: Vec<u64>,
    size: u64,
}

fn word_count(size: u64) -> usize {
    (((size + 511) / 512) * 8) as usize
}

impl BitVector {
    /// BitVector(size, init_value)
    pub fn new(size: u64, init_value: bool) -> Self {
        let mut v = Self::uninitialized(size);
        if init_value {
            v.set_all();
        }
        v
    }

    /// BitVector(size, BitVector::uninitialized). Zeroed, which is a valid
    /// "uninitialized" state for callers that set or reset every bit.
    pub fn uninitialized(size: u64) -> Self {
        BitVector { data: vec![0; word_count(size)], size }
    }

    pub fn size(&self) -> u64 {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn data(&self) -> &[u64] {
        &self.data
    }

    pub fn is_set(&self, x: u64) -> bool {
        debug_assert!(x < self.size, "argument out of bounds");
        (self.data[(x / 64) as usize] >> (x % 64)) & 1 != 0
    }

    pub fn set(&mut self, x: u64) {
        debug_assert!(x < self.size, "argument out of bounds");
        self.data[(x / 64) as usize] |= 1u64 << (x % 64);
    }

    /// BitVector::set(x, value)
    pub fn set_to(&mut self, x: u64, value: bool) {
        debug_assert!(x < self.size, "argument out of bounds");
        let w = &mut self.data[(x / 64) as usize];
        let b = 1u64 << (x % 64);
        if value {
            *w |= b;
        } else {
            *w &= !b;
        }
    }

    pub fn reset(&mut self, x: u64) {
        debug_assert!(x < self.size, "argument out of bounds");
        self.data[(x / 64) as usize] &= !(1u64 << (x % 64));
    }

    pub fn reset_all(&mut self) {
        self.data.iter_mut().for_each(|w| *w = 0);
    }

    pub fn set_all(&mut self) {
        self.data.iter_mut().for_each(|w| *w = !0);
        self.reset_all_padding_bits();
    }

    pub fn population_count(&self) -> u64 {
        self.data.iter().map(|w| w.count_ones() as u64).sum()
    }

    /// BitVector::operator|=
    pub fn or_assign(&mut self, other: &BitVector) {
        debug_assert_eq!(self.size, other.size);
        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a |= *b;
        }
    }

    pub fn inplace_not(&mut self) {
        self.data.iter_mut().for_each(|w| *w = !*w);
        self.reset_all_padding_bits();
    }

    /// Resizes to new_size bits. Existing bits are kept, new bits take init_value.
    pub fn resize(&mut self, new_size: u64, init_value: bool) {
        let mut o = BitVector::new(new_size, init_value);
        for x in 0..self.size.min(new_size) {
            o.set_to(x, self.is_set(x));
        }
        *self = o;
    }

    fn reset_all_padding_bits(&mut self) {
        let x = self.size % 512;
        if x != 0 {
            let base = ((self.size / 512) * 8) as usize;
            for i in 0..8u64 {
                let lo = i * 64;
                let mask = if x >= lo + 64 {
                    !0u64
                } else if x > lo {
                    (1u64 << (x - lo)) - 1
                } else {
                    0
                };
                self.data[base + i as usize] &= mask;
            }
        }
    }
}

/// make_bit_vector(size, f): bit x is f(x)
pub fn make_bit_vector(size: u64, f: impl Fn(u64) -> bool) -> BitVector {
    let mut v = BitVector::uninitialized(size);
    for x in 0..size {
        v.set_to(x, f(x));
    }
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inplace_not_keeps_padding_zero() {
        let mut v = BitVector::new(515, false);
        v.set(514);
        v.inplace_not();
        assert_eq!(v.population_count(), 514);
        assert!(!v.is_set(514));
        // word 8 holds bits 512..515; only 512 and 513 are set, bits 515.. are padding
        assert_eq!(v.data()[8], 0b011, "bits past size must stay zero");
    }

    #[test]
    fn make_bit_vector_and_or_assign() {
        let a = make_bit_vector(10, |x| x % 3 == 0);
        let mut b = make_bit_vector(10, |x| x % 2 == 0);
        b.or_assign(&a);
        assert_eq!(b.population_count(), 7); // 0,2,3,4,6,8,9
    }
}
