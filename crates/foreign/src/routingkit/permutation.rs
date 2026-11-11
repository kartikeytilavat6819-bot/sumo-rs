//! Port of foreign/RoutingKit/include/routingkit/permutation.h (the functions SUMO's CCH uses).

use super::constants::INVALID_ID;

/// apply_permutation(p, v): {v[p[0]], v[p[1]], ...}
pub fn apply_permutation<T: Copy>(p: &[u32], v: &[T]) -> Vec<T> {
    debug_assert_eq!(p.len(), v.len());
    (0..v.len()).map(|i| v[p[i] as usize]).collect()
}

/// apply_inverse_permutation(p, v): r[p[i]] = v[i]
pub fn apply_inverse_permutation<T: Copy>(p: &[u32], v: &[T]) -> Vec<T> {
    debug_assert_eq!(p.len(), v.len());
    let mut r = v.to_vec();
    for i in 0..v.len() {
        r[p[i] as usize] = v[i];
    }
    r
}

/// apply_permutation_to_elements_of(p, v): {p[v[0]], p[v[1]], ...}
pub fn apply_permutation_to_elements_of(p: &[u32], v: &[u32]) -> Vec<u32> {
    let mut r = v.to_vec();
    inplace_apply_permutation_to_elements_of(p, &mut r);
    r
}

pub fn inplace_apply_permutation_to_elements_of(p: &[u32], v: &mut [u32]) {
    for x in v.iter_mut() {
        debug_assert!(*x < p.len() as u32 || *x == INVALID_ID);
        *x = p[*x as usize];
    }
}

pub fn invert_permutation(p: &[u32]) -> Vec<u32> {
    let mut inv_p = vec![0u32; p.len()];
    for i in 0..p.len() {
        inv_p[p[i] as usize] = i as u32;
    }
    inv_p
}

pub fn identity_permutation(n: u32) -> Vec<u32> {
    (0..n).collect()
}

/// chain_permutation_first_left_then_right(p, q): r[i] = p[q[i]]
pub fn chain_permutation_first_left_then_right(p: &[u32], q: &[u32]) -> Vec<u32> {
    q.iter().map(|&qi| p[qi as usize]).collect()
}
