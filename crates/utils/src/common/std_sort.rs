//! libstdc++ `std::sort` (bits/stl_algo.h: introsort, median-of-3 pivot, insertion sort below 16,
//! heapsort fallback at depth limit), reproduced step by step. std::sort is unstable, so equal
//! elements end up in the same order as with C++ only when the same algorithm is used.

const THRESHOLD: isize = 16;

/// C++ `std::sort(first, last, less)` on a slice
pub fn std_sort<T: Copy>(v: &mut [T], less: impl Fn(T, T) -> bool) {
    let n = v.len() as isize;
    if n == 0 {
        return;
    }
    let depth = 2 * (63 - (n as u64).leading_zeros() as isize); // 2 * __lg(n)
    introsort_loop(v, 0, n, depth, &less);
    final_insertion_sort(v, 0, n, &less);
}

fn introsort_loop<T: Copy>(v: &mut [T], mut first: isize, mut last: isize, mut depth: isize, less: &impl Fn(T, T) -> bool) {
    while last - first > THRESHOLD {
        if depth == 0 {
            partial_sort(v, first, last, last, less);
            return;
        }
        depth -= 1;
        let cut = unguarded_partition_pivot(v, first, last, less);
        introsort_loop(v, cut, last, depth, less);
        last = cut;
    }
}

fn unguarded_partition_pivot<T: Copy>(v: &mut [T], first: isize, last: isize, less: &impl Fn(T, T) -> bool) -> isize {
    let mid = first + (last - first) / 2;
    move_median_to_first(v, first, first + 1, mid, last - 1, less);
    unguarded_partition(v, first + 1, last, first, less)
}

fn move_median_to_first<T: Copy>(v: &mut [T], result: isize, a: isize, b: isize, c: isize, less: &impl Fn(T, T) -> bool) {
    let (va, vb, vc) = (v[a as usize], v[b as usize], v[c as usize]);
    let pick = if less(va, vb) {
        if less(vb, vc) {
            b
        } else if less(va, vc) {
            c
        } else {
            a
        }
    } else if less(va, vc) {
        a
    } else if less(vb, vc) {
        c
    } else {
        b
    };
    v.swap(result as usize, pick as usize);
}

fn unguarded_partition<T: Copy>(v: &mut [T], mut first: isize, mut last: isize, pivot: isize, less: &impl Fn(T, T) -> bool) -> isize {
    loop {
        while less(v[first as usize], v[pivot as usize]) {
            first += 1;
        }
        last -= 1;
        while less(v[pivot as usize], v[last as usize]) {
            last -= 1;
        }
        if first >= last {
            return first;
        }
        v.swap(first as usize, last as usize);
        first += 1;
    }
}

fn final_insertion_sort<T: Copy>(v: &mut [T], first: isize, last: isize, less: &impl Fn(T, T) -> bool) {
    if last - first > THRESHOLD {
        insertion_sort(v, first, first + THRESHOLD, less);
        unguarded_insertion_sort(v, first + THRESHOLD, last, less);
    } else {
        insertion_sort(v, first, last, less);
    }
}

fn insertion_sort<T: Copy>(v: &mut [T], first: isize, last: isize, less: &impl Fn(T, T) -> bool) {
    if first == last {
        return;
    }
    for i in first + 1..last {
        if less(v[i as usize], v[first as usize]) {
            let val = v[i as usize];
            let mut k = i;
            while k > first {
                v[k as usize] = v[(k - 1) as usize];
                k -= 1;
            }
            v[first as usize] = val;
        } else {
            unguarded_linear_insert(v, i, less);
        }
    }
}

fn unguarded_insertion_sort<T: Copy>(v: &mut [T], first: isize, last: isize, less: &impl Fn(T, T) -> bool) {
    for i in first..last {
        unguarded_linear_insert(v, i, less);
    }
}

fn unguarded_linear_insert<T: Copy>(v: &mut [T], mut last: isize, less: &impl Fn(T, T) -> bool) {
    let val = v[last as usize];
    let mut next = last - 1;
    while less(val, v[next as usize]) {
        v[last as usize] = v[next as usize];
        last = next;
        next -= 1;
    }
    v[last as usize] = val;
}

fn partial_sort<T: Copy>(v: &mut [T], first: isize, middle: isize, last: isize, less: &impl Fn(T, T) -> bool) {
    // __heap_select
    make_heap(v, first, middle, less);
    for i in middle..last {
        if less(v[i as usize], v[first as usize]) {
            pop_heap(v, first, middle, i, less);
        }
    }
    // __sort_heap
    let mut end = middle;
    while end - first > 1 {
        end -= 1;
        pop_heap(v, first, end, end, less);
    }
}

fn make_heap<T: Copy>(v: &mut [T], first: isize, last: isize, less: &impl Fn(T, T) -> bool) {
    let len = last - first;
    if len < 2 {
        return;
    }
    let mut parent = (len - 2) / 2;
    loop {
        let value = v[(first + parent) as usize];
        adjust_heap(v, first, parent, len, value, less);
        if parent == 0 {
            return;
        }
        parent -= 1;
    }
}

fn pop_heap<T: Copy>(v: &mut [T], first: isize, last: isize, result: isize, less: &impl Fn(T, T) -> bool) {
    let value = v[result as usize];
    v[result as usize] = v[first as usize];
    adjust_heap(v, first, 0, last - first, value, less);
}

fn adjust_heap<T: Copy>(v: &mut [T], first: isize, mut hole: isize, len: isize, value: T, less: &impl Fn(T, T) -> bool) {
    let top = hole;
    let mut second = hole;
    while second < (len - 1) / 2 {
        second = 2 * (second + 1);
        if less(v[(first + second) as usize], v[(first + second - 1) as usize]) {
            second -= 1;
        }
        v[(first + hole) as usize] = v[(first + second) as usize];
        hole = second;
    }
    if len & 1 == 0 && second == (len - 2) / 2 {
        second = 2 * (second + 1);
        v[(first + hole) as usize] = v[(first + second - 1) as usize];
        hole = second - 1;
    }
    // __push_heap
    let mut parent = (hole - 1) / 2;
    while hole > top && less(v[(first + parent) as usize], value) {
        v[(first + hole) as usize] = v[(first + parent) as usize];
        hole = parent;
        parent = (hole - 1) / 2;
    }
    v[(first + hole) as usize] = value;
}

#[cfg(test)]
mod tests {
    use super::std_sort;

    #[test]
    fn sorts_like_a_total_order() {
        let mut v: Vec<i64> = (0..200).map(|i| (i * 7919) % 101).collect();
        std_sort(&mut v, |a, b| a < b);
        assert!(v.windows(2).all(|w| w[0] <= w[1]));
        let mut small = vec![3, 1, 2];
        std_sort(&mut small, |a, b| a < b);
        assert_eq!(small, vec![1, 2, 3]);
    }
}
