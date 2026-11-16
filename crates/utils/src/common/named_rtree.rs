//! Port of utils/common/NamedRTree.h
//!
//! C++ `NamedRTree` is `RTree<Named*, Named, float, 2, Named::StoringVisitor>` with the float 2D
//! RectSphericalVolume override, so it is `RTree<Ptr<Named>, f32, 2, f32, 8, 4, Float2DVolume>`
//! (TMAXNODES = 8, TMINNODES = TMAXNODES / 2 are the RTree.h defaults).
//!
//! determinism: the tree shape (and so the order of Search hits) depends on the insertion order
//! and on the volumes; both are kept identical to C++ (callers insert in the same container order).
//! Search hits are stored in a std::set ordered by address, so their output order does not depend on it.
use super::named::Named;
use super::ptr::Ptr;
use foreign::rtree::rtree::{Float2DVolume, RTree};

/// C++ `class NamedRTree`
pub type NamedRTree = RTree<Ptr<Named>, f32, 2, f32, 8, 4, Float2DVolume>;

/// C++ `new NamedRTree()`: the tree lives until `delete_named_rtree` (C++ statics live until cleanup)
pub fn new_named_rtree() -> Ptr<NamedRTree> {
    Ptr::from_raw(Box::into_raw(Box::new(NamedRTree::new())))
}

/// C++ `delete tree` for a tree created by `new_named_rtree` (a null tree is a no-op, as in C++)
pub fn delete_named_rtree(tree: Ptr<NamedRTree>) {
    if !tree.is_null() {
        drop(unsafe { Box::from_raw(tree.as_raw()) });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::named::NamedStoringVisitor;
    use std::collections::BTreeSet;

    #[test]
    fn search_matches_brute_force() {
        // Exercises splits and both insert and search through the Named visitor
        let mut seed: u64 = 987654321;
        let mut next = move || {
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            (seed >> 33) as u32
        };
        let objects: Vec<Box<Named>> = (0..300).map(|i| Box::new(Named::new(&format!("n{i}")))).collect();
        let mut tree = NamedRTree::new();
        let mut boxes: Vec<([f32; 2], [f32; 2])> = Vec::new();
        for obj in &objects {
            let x = (next() % 1000) as f32;
            let y = (next() % 1000) as f32;
            let w = (next() % 40) as f32;
            let h = (next() % 40) as f32;
            let (min, max) = ([x, y], [x + w, y + h]);
            tree.insert(&min, &max, &Ptr::from_box(obj));
            boxes.push((min, max));
        }
        assert_eq!(tree.count() as usize, objects.len());

        for _ in 0..50 {
            let qx = (next() % 1000) as f32;
            let qy = (next() % 1000) as f32;
            let qmin = [qx, qy];
            let qmax = [qx + 150.0, qy + 150.0];
            let mut found: BTreeSet<Ptr<Named>> = BTreeSet::new();
            let n = tree.search(&qmin, &qmax, &NamedStoringVisitor::new(&mut found), |d, c| d.add_to(c));
            let expected: BTreeSet<Ptr<Named>> = objects
                .iter()
                .zip(boxes.iter())
                .filter(|(_, (min, max))| (0..2).all(|d| !(min[d] > qmax[d] || qmin[d] > max[d])))
                .map(|(obj, _)| Ptr::from_box(obj))
                .collect();
            assert_eq!(n as usize, expected.len());
            assert_eq!(found, expected);
        }
    }

    #[test]
    fn remove_then_search_misses_the_entry() {
        let a = Box::new(Named::new("a"));
        let b = Box::new(Named::new("b"));
        let mut tree = new_named_rtree();
        tree.m().insert(&[0.0, 0.0], &[1.0, 1.0], &Ptr::from_box(&a));
        tree.m().insert(&[0.5, 0.5], &[2.0, 2.0], &Ptr::from_box(&b));
        tree.m().remove(&[0.0, 0.0], &[1.0, 1.0], &Ptr::from_box(&a));
        let mut found: BTreeSet<Ptr<Named>> = BTreeSet::new();
        tree.search(&[0.0, 0.0], &[3.0, 3.0], &NamedStoringVisitor::new(&mut found), |d, c| d.add_to(c));
        assert_eq!(found, BTreeSet::from([Ptr::from_box(&b)]));
        delete_named_rtree(tree);
    }
}
