//! Port of foreign/RoutingKit/include/routingkit/inverse_vector.h (invert_vector only).

/// invert_vector(v, element_count): for sorted v, index[i]..index[i+1] is the range of
/// positions holding value i.
pub fn invert_vector(v: &[u32], element_count: u32) -> Vec<u32> {
    let mut index = vec![0u32; element_count as usize + 1];
    if !v.is_empty() {
        debug_assert!(v.windows(2).all(|w| w[0] <= w[1]), "v must be sorted");
        let mut pos = 0usize;
        for i in 0..element_count as usize {
            while pos < v.len() && (v[pos] as usize) < i {
                pos += 1;
            }
            index[i] = pos as u32;
        }
        index[element_count as usize] = v.len() as u32;
    }
    index
}
