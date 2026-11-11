//! Port of foreign/RoutingKit/include/routingkit/id_mapper.h and src/id_mapper.cpp.
//! Only LocalIDMapper is ported (IDMapper::to_global is unused by SUMO).
//! The C++ mapper points into the caller's bit vector; this one owns a copy of the
//! bits. The CCH code never mutates a bit vector after mapping it, so results match.

use super::bit_vector::BitVector;

#[derive(Clone, Debug, Default)]
pub struct LocalIDMapper {
    bits: Vec<u64>,
    bit_count: u64,
    rank: Vec<u64>,
}

impl LocalIDMapper {
    /// LocalIDMapper(bit_vector)
    pub fn new(v: &BitVector) -> Self {
        Self::from_bits(v.size(), v.data())
    }

    /// LocalIDMapper(bit_count, bits)
    pub fn from_bits(bit_count: u64, bits: &[u64]) -> Self {
        let block_count = (bit_count + 511) / 512;
        let mut b = bits.to_vec();
        b.resize((block_count * 8) as usize, 0);
        let mut rank = vec![0u64; (block_count + 1) as usize];
        let mut s = 0u64;
        for j in 0..block_count as usize {
            rank[j] = s;
            s += b[j * 8..j * 8 + 8].iter().map(|w| w.count_ones() as u64).sum::<u64>();
        }
        rank[block_count as usize] = s;
        LocalIDMapper { bits: b, bit_count, rank }
    }

    pub fn global_id_count(&self) -> u64 {
        self.bit_count
    }

    pub fn local_id_count(&self) -> u64 {
        self.rank.last().copied().unwrap_or(0)
    }

    /// to_local(global_id): the global id must be mapped
    pub fn to_local(&self, global_id: u64) -> u64 {
        debug_assert!(global_id < self.global_id_count(), "global id is out of bounds");
        let uint64_offset = global_id % 64;
        let uint64_index = global_id / 64;
        debug_assert!(self.bits[uint64_index as usize] & (1u64 << uint64_offset) != 0, "global id is not mapped");
        let uint512_index = global_id / 512;
        let mut local_id = self.rank[uint512_index as usize];
        for i in (uint512_index * 8)..uint64_index {
            local_id += self.bits[i as usize].count_ones() as u64;
        }
        local_id += (self.bits[uint64_index as usize] & ((1u64 << uint64_offset) - 1)).count_ones() as u64;
        debug_assert!(local_id < self.local_id_count());
        local_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_local_counts_set_bits_across_block_boundaries() {
        let n = 1500u64;
        let mut v = BitVector::new(n, false);
        let set: Vec<u64> = (0..n).filter(|x| x % 7 == 0 || x % 512 == 511).collect();
        for &x in &set {
            v.set(x);
        }
        let m = LocalIDMapper::new(&v);
        assert_eq!(m.local_id_count(), set.len() as u64);
        for (local, &global) in set.iter().enumerate() {
            assert_eq!(m.to_local(global), local as u64);
        }
    }
}
