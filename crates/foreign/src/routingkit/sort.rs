//! Port of the parts of foreign/RoutingKit/include/routingkit/sort.h that SUMO's CCH uses,
//! plus a libstdc++-compatible std::nth_element (used by nested dissection).
//!
//! determinism: key-based sorts here are stable (bucket sort or stable sort), so their
//! result is fully defined. std::nth_element is not stable: which of several equal keys
//! lands on which side of the cut depends on the algorithm, so nth_element below follows
//! libstdc++ step by step (introselect, median-of-3 partition, heap-select fallback).

use super::permutation::{identity_permutation, invert_permutation};

const BUCKET_SORT_MIN_KEY_TO_ELEMENT_RATIO: u32 = 16;

/// compute_stable_sort_permutation_using_comparator(v, is_less)
pub fn compute_stable_sort_permutation_using_comparator<T: Copy, F: Fn(T, T) -> bool>(v: &[T], is_less: &F) -> Vec<u32> {
    let mut p = identity_permutation(v.len() as u32);
    p.sort_by(|&l, &r| {
        let (a, b) = (v[l as usize], v[r as usize]);
        if is_less(a, b) {
            std::cmp::Ordering::Less
        } else if is_less(b, a) {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    });
    p
}

/// compute_inverse_stable_sort_permutation_using_comparator(v, is_less)
pub fn compute_inverse_stable_sort_permutation_using_comparator<T: Copy, F: Fn(T, T) -> bool>(v: &[T], is_less: &F) -> Vec<u32> {
    invert_permutation(&compute_stable_sort_permutation_using_comparator(v, is_less))
}

fn compute_key_pos<T: Copy, K: Fn(T) -> u32>(v: &[T], key_count: u32, get_key: &K) -> Vec<u32> {
    let mut key_pos = vec![0u32; key_count as usize];
    for &x in v {
        key_pos[get_key(x) as usize] += 1;
    }
    let mut sum = 0u32;
    for k in key_pos.iter_mut() {
        let tmp = sum + *k;
        *k = sum;
        sum = tmp;
    }
    key_pos
}

/// compute_stable_sort_permutation_using_key(v, key_count, get_key)
pub fn compute_stable_sort_permutation_using_key<T: Copy, K: Fn(T) -> u32>(v: &[T], key_count: u32, get_key: &K) -> Vec<u32> {
    if v.len() >= (key_count / BUCKET_SORT_MIN_KEY_TO_ELEMENT_RATIO) as usize {
        let mut key_pos = compute_key_pos(v, key_count, get_key);
        let mut p = vec![0u32; v.len()];
        for (i, &x) in v.iter().enumerate() {
            let k = get_key(x) as usize;
            p[key_pos[k] as usize] = i as u32;
            key_pos[k] += 1;
        }
        p
    } else {
        compute_stable_sort_permutation_using_comparator(v, &|l: T, r: T| get_key(l) < get_key(r))
    }
}

/// compute_inverse_stable_sort_permutation_using_key(v, key_count, get_key)
pub fn compute_inverse_stable_sort_permutation_using_key<T: Copy, K: Fn(T) -> u32>(v: &[T], key_count: u32, get_key: &K) -> Vec<u32> {
    if v.len() >= (key_count / BUCKET_SORT_MIN_KEY_TO_ELEMENT_RATIO) as usize {
        let mut key_pos = compute_key_pos(v, key_count, get_key);
        let mut p = vec![0u32; v.len()];
        for (i, &x) in v.iter().enumerate() {
            let k = get_key(x) as usize;
            p[i] = key_pos[k];
            key_pos[k] += 1;
        }
        p
    } else {
        compute_inverse_stable_sort_permutation_using_comparator(v, &|l: T, r: T| get_key(l) < get_key(r))
    }
}

// ---------------------------------------------------------------------------
// libstdc++ std::nth_element (bits/stl_algo.h), reproduced step by step so that
// the order of equal elements matches a GCC/Clang build of the C++ code.
// ---------------------------------------------------------------------------

/// std::nth_element(v[first..last], v[nth]) with comparator `less`.
pub fn std_nth_element<T: Copy, F: Fn(T, T) -> bool>(v: &mut [T], first: usize, nth: usize, last: usize, less: &F) {
    if first == last || nth == last {
        return;
    }
    let depth = 2 * lg(last - first);
    introselect(v, first, nth, last, depth, less);
}

/// floor(log2(n)) for n >= 1, as std::__lg
fn lg(n: usize) -> usize {
    (usize::BITS - 1 - n.leading_zeros()) as usize
}

fn introselect<T: Copy, F: Fn(T, T) -> bool>(v: &mut [T], mut first: usize, nth: usize, mut last: usize, mut depth: usize, less: &F) {
    while last - first > 3 {
        if depth == 0 {
            heap_select(v, first, nth + 1, last, less);
            v.swap(first, nth);
            return;
        }
        depth -= 1;
        let cut = unguarded_partition_pivot(v, first, last, less);
        if cut <= nth {
            first = cut;
        } else {
            last = cut;
        }
    }
    insertion_sort(v, first, last, less);
}

fn unguarded_partition_pivot<T: Copy, F: Fn(T, T) -> bool>(v: &mut [T], first: usize, last: usize, less: &F) -> usize {
    let mid = first + (last - first) / 2;
    move_median_to_first(v, first, first + 1, mid, last - 1, less);
    unguarded_partition(v, first + 1, last, first, less)
}

fn move_median_to_first<T: Copy, F: Fn(T, T) -> bool>(v: &mut [T], result: usize, a: usize, b: usize, c: usize, less: &F) {
    if less(v[a], v[b]) {
        if less(v[b], v[c]) {
            v.swap(result, b);
        } else if less(v[a], v[c]) {
            v.swap(result, c);
        } else {
            v.swap(result, a);
        }
    } else if less(v[a], v[c]) {
        v.swap(result, a);
    } else if less(v[b], v[c]) {
        v.swap(result, c);
    } else {
        v.swap(result, b);
    }
}

fn unguarded_partition<T: Copy, F: Fn(T, T) -> bool>(v: &mut [T], mut first: usize, mut last: usize, pivot: usize, less: &F) -> usize {
    loop {
        while less(v[first], v[pivot]) {
            first += 1;
        }
        last -= 1;
        while less(v[pivot], v[last]) {
            last -= 1;
        }
        if first >= last {
            return first;
        }
        v.swap(first, last);
        first += 1;
    }
}

fn insertion_sort<T: Copy, F: Fn(T, T) -> bool>(v: &mut [T], first: usize, last: usize, less: &F) {
    if first == last {
        return;
    }
    for i in first + 1..last {
        if less(v[i], v[first]) {
            let val = v[i];
            let mut j = i;
            while j > first {
                v[j] = v[j - 1];
                j -= 1;
            }
            v[first] = val;
        } else {
            unguarded_linear_insert(v, i, less);
        }
    }
}

fn unguarded_linear_insert<T: Copy, F: Fn(T, T) -> bool>(v: &mut [T], mut last: usize, less: &F) {
    let val = v[last];
    let mut next = last - 1;
    while less(val, v[next]) {
        v[last] = v[next];
        last = next;
        next -= 1;
    }
    v[last] = val;
}

fn heap_select<T: Copy, F: Fn(T, T) -> bool>(v: &mut [T], first: usize, middle: usize, last: usize, less: &F) {
    make_heap(v, first, middle, less);
    for i in middle..last {
        if less(v[i], v[first]) {
            pop_heap(v, first, middle, i, less);
        }
    }
}

fn make_heap<T: Copy, F: Fn(T, T) -> bool>(v: &mut [T], first: usize, last: usize, less: &F) {
    let len = (last - first) as isize;
    if len < 2 {
        return;
    }
    let mut parent = (len - 2) / 2;
    loop {
        let value = v[first + parent as usize];
        adjust_heap(v, first, parent, len, value, less);
        if parent == 0 {
            break;
        }
        parent -= 1;
    }
}

fn pop_heap<T: Copy, F: Fn(T, T) -> bool>(v: &mut [T], first: usize, last: usize, result: usize, less: &F) {
    let value = v[result];
    v[result] = v[first];
    adjust_heap(v, first, 0, (last - first) as isize, value, less);
}

fn adjust_heap<T: Copy, F: Fn(T, T) -> bool>(v: &mut [T], first: usize, mut hole: isize, len: isize, value: T, less: &F) {
    let top = hole;
    let mut second = hole;
    while second < (len - 1) / 2 {
        second = 2 * (second + 1);
        if less(v[first + second as usize], v[first + (second - 1) as usize]) {
            second -= 1;
        }
        v[first + hole as usize] = v[first + second as usize];
        hole = second;
    }
    if len % 2 == 0 && second == (len - 2) / 2 {
        second = 2 * (second + 1);
        v[first + hole as usize] = v[first + (second - 1) as usize];
        hole = second - 1;
    }
    push_heap(v, first, hole, top, value, less);
}

fn push_heap<T: Copy, F: Fn(T, T) -> bool>(v: &mut [T], first: usize, mut hole: isize, top: isize, value: T, less: &F) {
    let mut parent = (hole - 1) / 2;
    while hole > top && less(v[first + parent as usize], value) {
        v[first + hole as usize] = v[first + parent as usize];
        hole = parent;
        parent = (hole - 1) / 2;
    }
    v[first + hole as usize] = value;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nth_element_places_nth_and_partitions() {
        let keys = [5u32, 1, 4, 1, 3, 9, 2, 6, 5, 3, 5, 8];
        let mut v: Vec<u32> = (0..keys.len() as u32).collect();
        let less = |l: u32, r: u32| keys[l as usize] < keys[r as usize];
        std_nth_element(&mut v, 0, 4, keys.len(), &less);
        let pivot = keys[v[4] as usize];
        for &x in &v[..4] {
            assert!(keys[x as usize] <= pivot);
        }
        for &x in &v[5..] {
            assert!(keys[x as usize] >= pivot);
        }
    }

    #[test]
    fn stable_key_permutation_matches_definition() {
        let v = [2u32, 0, 2, 1, 0];
        let p = compute_stable_sort_permutation_using_key(&v, 3, &|x: u32| x);
        assert_eq!(p, vec![1, 4, 3, 0, 2]);
        let inv = compute_inverse_stable_sort_permutation_using_key(&v, 3, &|x: u32| x);
        assert_eq!(inv, invert_permutation(&p));
    }
}
