//! Port of foreign/RoutingKit/include/routingkit/filter.h (inplace_keep_element_of_vector_if only).

use super::bit_vector::BitVector;

pub fn inplace_keep_element_of_vector_if<T: Copy>(keep_filter: &BitVector, vec: &mut Vec<T>) {
    debug_assert_eq!(keep_filter.size(), vec.len() as u64);
    let mut out = 0usize;
    for i in 0..vec.len() {
        if keep_filter.is_set(i as u64) {
            vec[out] = vec[i];
            out += 1;
        }
    }
    vec.truncate(out);
}
