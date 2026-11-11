//! Port of foreign/RoutingKit/src/graph_util.cpp (the functions SUMO's CCH uses).

use super::constants::INVALID_ID;
use super::permutation::{apply_inverse_permutation, apply_permutation, chain_permutation_first_left_then_right};
use super::sort::{compute_inverse_stable_sort_permutation_using_key, compute_stable_sort_permutation_using_key};

/// find_arc_given_sorted_head(first_out, head, x, y)
pub fn find_arc_given_sorted_head(first_out: &[u32], head: &[u32], x: u32, y: u32) -> u32 {
    let ret = find_arc_or_return_invalid_given_sorted_head(first_out, head, x, y);
    debug_assert!(ret != INVALID_ID, "arc does not exist");
    ret
}

/// find_arc_or_return_invalid_given_sorted_head(first_out, head, x, y)
pub fn find_arc_or_return_invalid_given_sorted_head(first_out: &[u32], head: &[u32], x: u32, y: u32) -> u32 {
    let begin = first_out[x as usize] as usize;
    let end = first_out[x as usize + 1] as usize;
    let slice = &head[begin..end];
    debug_assert!(slice.windows(2).all(|w| w[0] <= w[1]), "heads are not sorted");
    let pos = slice.partition_point(|&h| h < y);
    if pos == slice.len() || slice[pos] != y {
        return INVALID_ID;
    }
    (begin + pos) as u32
}

/// compute_sort_permutation_first_by_left_then_by_right_and_apply_sort_to_left
pub fn compute_sort_permutation_first_by_left_then_by_right_and_apply_sort_to_left(
    a_count: u32,
    a: &mut Vec<u32>,
    b_count: u32,
    b: &[u32],
) -> Vec<u32> {
    let p = compute_stable_sort_permutation_using_key(b, b_count, &|x: u32| x);
    *a = apply_permutation(&p, &a[..]);
    let q = compute_stable_sort_permutation_using_key(&a[..], a_count, &|x: u32| x);
    *a = apply_permutation(&q, &a[..]);
    chain_permutation_first_left_then_right(&p, &q)
}

/// compute_inverse_sort_permutation_first_by_left_then_by_right_and_apply_sort_to_left
pub fn compute_inverse_sort_permutation_first_by_left_then_by_right_and_apply_sort_to_left(
    a_count: u32,
    a: &mut Vec<u32>,
    b_count: u32,
    b: &[u32],
) -> Vec<u32> {
    let p = compute_inverse_stable_sort_permutation_using_key(b, b_count, &|x: u32| x);
    *a = apply_inverse_permutation(&p, &a[..]);
    let q = compute_inverse_stable_sort_permutation_using_key(&a[..], a_count, &|x: u32| x);
    *a = apply_inverse_permutation(&q, &a[..]);
    chain_permutation_first_left_then_right(&q, &p)
}

/// compute_inverse_sort_permutation_first_by_tail_then_by_head_and_apply_sort_to_tail
pub fn compute_inverse_sort_permutation_first_by_tail_then_by_head_and_apply_sort_to_tail(
    node_count: u32,
    tail: &mut Vec<u32>,
    head: &[u32],
) -> Vec<u32> {
    compute_inverse_sort_permutation_first_by_left_then_by_right_and_apply_sort_to_left(node_count, tail, node_count, head)
}

/// compute_sort_permutation_first_by_tail_then_by_head_and_apply_sort_to_tail
pub fn compute_sort_permutation_first_by_tail_then_by_head_and_apply_sort_to_tail(
    node_count: u32,
    tail: &mut Vec<u32>,
    head: &[u32],
) -> Vec<u32> {
    compute_sort_permutation_first_by_left_then_by_right_and_apply_sort_to_left(node_count, tail, node_count, head)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts_arcs_by_tail_then_head_stably() {
        // arcs: 0->2, 1->0, 0->1, 1->0 (duplicate), 2->1
        let mut tail = vec![0u32, 1, 0, 1, 2];
        let head = vec![2u32, 0, 1, 0, 1];
        let p = compute_inverse_sort_permutation_first_by_tail_then_by_head_and_apply_sort_to_tail(3, &mut tail, &head);
        assert_eq!(tail, vec![0, 0, 1, 1, 2]);
        // head after applying the inverse permutation
        let sorted_head = apply_inverse_permutation(&p, &head);
        assert_eq!(sorted_head, vec![1, 2, 0, 0, 1]);
        assert_eq!(find_arc_given_sorted_head(&[0, 2, 4, 5], &sorted_head, 0, 2), 1);
        assert_eq!(find_arc_or_return_invalid_given_sorted_head(&[0, 2, 4, 5], &sorted_head, 0, 0), INVALID_ID);
    }
}
