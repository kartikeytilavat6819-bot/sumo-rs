//! Port of foreign/rtree/RTree.h
//!
//! The templated C++ R-tree becomes `RTree<D, E, NUMDIMS, R, MAXNODES, MINNODES>`:
//! `D` is DATATYPE, `E` is ELEMTYPE (coordinates), `R` is ELEMTYPEREAL (volumes).
//! The algorithm is ported step by step (not built on the rstar crate) because
//! `search` must return entries in the same order as the C++ code.
//!
//! Differences from the C++ template that do not change results:
//! - No DATATYPENP / `Operation` member pointer: `search` takes the per-hit callback as an
//!   argument and calls `op(&data, context)` where C++ calls `(id->*myOperation)(c)`.
//! - The C++ union `Branch::{m_child, m_data}` is the `Payload` enum. Reinsertion of internal
//!   branches (C++ copies `m_data` of an internal branch) keeps the `Child` payload.
//! - Nodes own their children (`Box`), and a node's branches are a `Vec` whose length is `m_count`.
//! - The C++ reinsertion list is a stack (`ReInsert` pushes at the head), so it is a `Vec` used LIFO.
//! - RTREE_WANTS_IO (Save/Load) is #undef'd in RTree.h, so it is not ported.
//! - RTREE_DONT_USE_MEMPOOLS is defined, so plain allocation (Box/Vec) is the C++ `new`/`delete` path.
//! - Iterator stack is a `Vec`; the C++ fixed MAX_STACK=32 is only checked with debug_assert.

use std::marker::PhantomData;
use std::ops::{Add, Mul, Neg, Sub};

/// ELEMTYPE: coordinate type. `to_real` is the C++ cast `(ELEMTYPEREAL)x`.
pub trait Coord: Copy + PartialOrd + Sub<Output = Self> {
    fn zero() -> Self;
    fn to_real<R: Real>(self) -> R;
}

/// ELEMTYPEREAL: volume type (float or double).
pub trait Real: Copy + PartialOrd + Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Neg<Output = Self> {
    fn from_f64(v: f64) -> Self;
    fn to_f64(self) -> f64;
    fn sqrt(self) -> Self;
}

macro_rules! impl_float_coord {
    ($t:ty) => {
        impl Coord for $t {
            fn zero() -> Self {
                0.0
            }
            fn to_real<R: Real>(self) -> R {
                R::from_f64(f64::from(self))
            }
        }
        impl Real for $t {
            fn from_f64(v: f64) -> Self {
                v as $t
            }
            fn to_f64(self) -> f64 {
                f64::from(self)
            }
            fn sqrt(self) -> Self {
                <$t>::sqrt(self)
            }
        }
    };
}
impl_float_coord!(f32);
impl_float_coord!(f64);

/// Unit sphere volumes for dimensions 0..=20 (C++ UNIT_SPHERE_VOLUMES, float literals).
const UNIT_SPHERE_VOLUMES: [f32; 21] = [
    0.000000, 2.000000, 3.141593, // Dimension  0,1,2
    4.188790, 4.934802, 5.263789, // Dimension  3,4,5
    5.167713, 4.724766, 4.058712, // Dimension  6,7,8
    3.298509, 2.550164, 1.884104, // Dimension  9,10,11
    1.335263, 0.910629, 0.599265, // Dimension  12,13,14
    0.381443, 0.235331, 0.140981, // Dimension  15,16,17
    0.082146, 0.046622, 0.025807, // Dimension  18,19,20
];

const MAX_STACK: usize = 32;

/// Chooses the RectSphericalVolume of a tree. C++ specialises it for NamedRTree and
/// LANE_RTREE_QUAL (float, 2 dims); every other RTree uses the RTree.h template.
pub trait VolumeModel {
    /// true for the float 2D specialisation (`.78539816f * (extent0 * extent0 + extent1 * extent1)`)
    const FLOAT_2D_SPECIALISED: bool;
}

/// The RTree.h template RectSphericalVolume (default)
pub struct TemplateVolume;

impl VolumeModel for TemplateVolume {
    const FLOAT_2D_SPECIALISED: bool = false;
}

/// NamedRTree.h / MSLane.h RectSphericalVolume specialisation (float, 2 dims)
pub struct Float2DVolume;

impl VolumeModel for Float2DVolume {
    const FLOAT_2D_SPECIALISED: bool = true;
}

/// C++ rtree_min: `(a<b?a:b)`
fn rtree_min<T: PartialOrd + Copy>(a: T, b: T) -> T {
    if a < b { a } else { b }
}

/// C++ rtree_max: `(a>b?a:b)`
fn rtree_max<T: PartialOrd + Copy>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

/// Minimal bounding rectangle (n-dimensional).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rect<E, const N: usize> {
    pub min: [E; N],
    pub max: [E; N],
}

/// C++ Branch union: a child node in internal nodes, the data id in leaves.
enum Payload<D, E, const N: usize> {
    Child(Box<Node<D, E, N>>),
    Data(D),
}

struct Branch<D, E, const N: usize> {
    rect: Rect<E, N>,
    payload: Payload<D, E, N>,
}

impl<D, E, const N: usize> Branch<D, E, N> {
    fn child(&self) -> &Node<D, E, N> {
        match &self.payload {
            Payload::Child(child) => child,
            Payload::Data(_) => unreachable!("internal branch must hold a child"),
        }
    }

    fn data(&self) -> &D {
        match &self.payload {
            Payload::Data(data) => data,
            Payload::Child(_) => unreachable!("leaf branch must hold data"),
        }
    }
}

/// C++ Node: `m_level` is 0 for leaves, `m_count` is `branches.len()`.
struct Node<D, E, const N: usize> {
    level: i32,
    branches: Vec<Branch<D, E, N>>,
}

impl<D, E, const N: usize> Node<D, E, N> {
    fn is_internal(&self) -> bool {
        self.level > 0
    }

    fn is_leaf(&self) -> bool {
        self.level == 0
    }
}

/// Variables for finding a split partition (C++ PartitionVars).
struct PartitionVars<D, E, const N: usize, R> {
    partition: Vec<i32>,
    total: usize,
    min_fill: usize,
    taken: Vec<bool>,
    count: [usize; 2],
    cover: [Rect<E, N>; 2],
    area: [R; 2],
    branch_buf: Vec<Branch<D, E, N>>,
    cover_split: Rect<E, N>,
    cover_split_area: R,
}

/// C++ RTree<...>. Use `RTree::<D, E, NUMDIMS, R, MAXNODES, MINNODES>` with the template arguments in C++ order.
pub struct RTree<D, E, const NUMDIMS: usize, R, const MAXNODES: usize, const MINNODES: usize, V: VolumeModel = TemplateVolume> {
    root: Box<Node<D, E, NUMDIMS>>,
    _real: PhantomData<(R, V)>,
}

impl<D, E, const NUMDIMS: usize, R, const MAXNODES: usize, const MINNODES: usize, V: VolumeModel> Default
    for RTree<D, E, NUMDIMS, R, MAXNODES, MINNODES, V>
where
    D: Clone + PartialEq,
    E: Coord,
    R: Real,
{
    fn default() -> Self {
        Self::new()
    }
}

/// C++ RTree::Iterator. Not remove safe.
pub struct RTreeIter<'a, D, E, const N: usize> {
    stack: Vec<(&'a Node<D, E, N>, usize)>,
}

impl<'a, D, E, const N: usize> RTreeIter<'a, D, E, N> {
    fn new() -> Self {
        RTreeIter { stack: Vec::new() }
    }

    fn push(&mut self, node: &'a Node<D, E, N>, branch_index: usize) {
        self.stack.push((node, branch_index));
        debug_assert!(self.stack.len() <= MAX_STACK);
    }

    /// C++ IsNull: true when the stack is empty.
    pub fn is_null(&self) -> bool {
        self.stack.is_empty()
    }

    /// C++ IsNotNull.
    pub fn is_not_null(&self) -> bool {
        !self.stack.is_empty()
    }

    /// C++ operator*: the current data element. Caller must check `is_not_null` first.
    pub fn get(&self) -> &D {
        let (node, index) = *self.stack.last().expect("RTree iterator is null");
        node.branches[index].data()
    }

    /// C++ operator++ (FindNextData).
    pub fn advance(&mut self) -> bool {
        self.find_next_data()
    }

    /// C++ FindNextData.
    fn find_next_data(&mut self) -> bool {
        loop {
            let Some((node, branch_index)) = self.stack.pop() else {
                return false;
            };
            if node.is_leaf() {
                // Keep walking through data while we can
                if branch_index + 1 < node.branches.len() {
                    self.push(node, branch_index + 1);
                    return true;
                }
                // No more data, so it falls back to the previous level
            } else {
                if branch_index + 1 < node.branches.len() {
                    // Sibling is the 'fall back' node when we finish with the current level
                    self.push(node, branch_index + 1);
                }
                // Push first branch of the next level to get deeper into the tree
                let next_level = node.branches[branch_index].child();
                self.push(next_level, 0);

                // If we pushed on a new leaf, the data is ready at TOS
                if next_level.is_leaf() {
                    return true;
                }
            }
        }
    }
}

impl<D, E, const NUMDIMS: usize, R, const MAXNODES: usize, const MINNODES: usize, V: VolumeModel>
    RTree<D, E, NUMDIMS, R, MAXNODES, MINNODES, V>
where
    D: Clone + PartialEq,
    E: Coord,
    R: Real,
{
    /// C++ ASSERTs in the constructor, plus the unit sphere table bound (NUMDIMS <= 20).
    const PARAMS_OK: () = assert!(MAXNODES > MINNODES && MINNODES > 0 && NUMDIMS <= 20);

    /// C++ RTree(Operation). The operation is passed to `search` instead.
    pub fn new() -> Self {
        let () = Self::PARAMS_OK;
        RTree {
            root: Self::new_node(0),
            _real: PhantomData,
        }
    }

    /// C++ AllocNode + InitNode (level -1, count 0), with the level set by the caller when needed.
    fn new_node(level: i32) -> Box<Node<D, E, NUMDIMS>> {
        Box::new(Node {
            level,
            branches: Vec::with_capacity(MAXNODES + 1),
        })
    }

    /// C++ Insert: entry with the bounding rect `[min, max]` and data id.
    pub fn insert(&mut self, a_min: &[E; NUMDIMS], a_max: &[E; NUMDIMS], data_id: &D) {
        let rect = Rect { min: *a_min, max: *a_max };
        Self::insert_rect(&rect, Payload::Data(data_id.clone()), &mut self.root, 0);
    }

    /// C++ Remove: entry is identified by its rect and data id.
    pub fn remove(&mut self, a_min: &[E; NUMDIMS], a_max: &[E; NUMDIMS], data_id: &D) {
        let rect = Rect { min: *a_min, max: *a_max };
        Self::remove_rect(&rect, data_id, &mut self.root);
    }

    /// C++ Search: calls `op(data, context)` for every entry overlapping the query rect, in tree
    /// order, and returns the number of entries found.
    pub fn search<C, F>(&self, a_min: &[E; NUMDIMS], a_max: &[E; NUMDIMS], context: &C, mut op: F) -> i32
    where
        F: FnMut(&D, &C),
    {
        let rect = Rect { min: *a_min, max: *a_max };
        let mut found_count = 0;
        Self::search_rec(&self.root, &rect, &mut found_count, context, &mut op);
        found_count
    }

    /// C++ RemoveAll: drops every node and starts an empty leaf root.
    pub fn remove_all(&mut self) {
        self.root = Self::new_node(0);
    }

    /// C++ Count: number of data elements (walks the tree, no counter is kept).
    pub fn count(&self) -> i32 {
        let mut count = 0;
        Self::count_rec(&self.root, &mut count);
        count
    }

    /// C++ GetFirst.
    pub fn get_first(&self) -> RTreeIter<'_, D, E, NUMDIMS> {
        let mut it = RTreeIter::new();
        if !self.root.branches.is_empty() {
            it.push(&self.root, 0);
            // NOTE: faithful to RTree.h: for a leaf root this skips entry 0, because FindNextData
            // treats the pushed (root, 0) as the already-returned current element.
            it.find_next_data();
        }
        it
    }

    fn count_rec(node: &Node<D, E, NUMDIMS>, count: &mut i32) {
        if node.is_internal() {
            for branch in &node.branches {
                Self::count_rec(branch.child(), count);
            }
        } else {
            *count += node.branches.len() as i32;
        }
    }

    fn zero_rect() -> Rect<E, NUMDIMS> {
        Rect {
            min: [E::zero(); NUMDIMS],
            max: [E::zero(); NUMDIMS],
        }
    }

    /// C++ CalcRectVolume -> RectSphericalVolume (RTREE_USE_SPHERICAL_VOLUME is defined).
    fn calc_rect_volume(rect: &Rect<E, NUMDIMS>) -> R {
        Self::rect_spherical_volume(rect)
    }

    /// C++ RectVolume (only used when RTREE_USE_SPHERICAL_VOLUME is not defined).
    #[allow(dead_code)]
    fn rect_volume(rect: &Rect<E, NUMDIMS>) -> R {
        let mut volume = R::from_f64(1.0);
        for index in 0..NUMDIMS {
            volume = volume * (rect.max[index] - rect.min[index]).to_real::<R>();
        }
        volume
    }

    /// C++ UNIT_SPHERE_VOLUMES[NUMDIMS] stored as m_unitSphereVolume.
    fn unit_sphere_volume() -> R {
        R::from_f64(f64::from(UNIT_SPHERE_VOLUMES[NUMDIMS]))
    }

    /// C++ RectSphericalVolume.
    fn rect_spherical_volume(rect: &Rect<E, NUMDIMS>) -> R {
        if V::FLOAT_2D_SPECIALISED && NUMDIMS == 2 {
            // C++ NAMED_RTREE_QUAL / LANE_RTREE_QUAL::RectSphericalVolume (float literal, float arithmetic)
            let extent0 = (rect.max[0] - rect.min[0]).to_real::<R>();
            let extent1 = (rect.max[1] - rect.min[1]).to_real::<R>();
            return R::from_f64(f64::from(0.785_398_16_f32)) * (extent0 * extent0 + extent1 * extent1);
        }
        let mut sum_of_squares = R::from_f64(0.0);
        for index in 0..NUMDIMS {
            let half_extent = (rect.max[index].to_real::<R>() - rect.min[index].to_real::<R>()) * R::from_f64(0.5);
            sum_of_squares = sum_of_squares + half_extent * half_extent;
        }

        let radius = sum_of_squares.sqrt();
        let unit = Self::unit_sphere_volume();

        // C++ pow(radius, NUMDIMS) is a double, so the product with the unit volume is in double too.
        if NUMDIMS == 3 {
            radius * radius * radius * unit
        } else if NUMDIMS == 2 {
            radius * radius * unit
        } else {
            let pow = radius.to_f64().powf(NUMDIMS as f64);
            R::from_f64(pow * unit.to_f64())
        }
    }

    /// C++ CombineRect: smallest rect containing both.
    fn combine_rect(rect_a: &Rect<E, NUMDIMS>, rect_b: &Rect<E, NUMDIMS>) -> Rect<E, NUMDIMS> {
        let mut new_rect = Self::zero_rect();
        for index in 0..NUMDIMS {
            new_rect.min[index] = rtree_min(rect_a.min[index], rect_b.min[index]);
            new_rect.max[index] = rtree_max(rect_a.max[index], rect_b.max[index]);
        }
        new_rect
    }

    /// C++ NodeCover: smallest rect covering all branches of a node (zero rect when empty).
    fn node_cover(node: &Node<D, E, NUMDIMS>) -> Rect<E, NUMDIMS> {
        let mut rect = Self::zero_rect();
        for (index, branch) in node.branches.iter().enumerate() {
            if index == 0 {
                rect = branch.rect;
            } else {
                rect = Self::combine_rect(&rect, &branch.rect);
            }
        }
        rect
    }

    /// C++ Overlap: inclusive on both ends.
    fn overlap(rect_a: &Rect<E, NUMDIMS>, rect_b: &Rect<E, NUMDIMS>) -> bool {
        for index in 0..NUMDIMS {
            if rect_a.min[index] > rect_b.max[index] || rect_b.min[index] > rect_a.max[index] {
                return false;
            }
        }
        true
    }

    /// C++ PickBranch: least area increase, ties broken by smaller area.
    fn pick_branch(rect: &Rect<E, NUMDIMS>, node: &Node<D, E, NUMDIMS>) -> usize {
        let mut first_time = true;
        let mut best_incr = R::from_f64(-1.0);
        let mut best_area = R::from_f64(0.0);
        let mut best = 0;

        for index in 0..node.branches.len() {
            let cur_rect = &node.branches[index].rect;
            let area = Self::calc_rect_volume(cur_rect);
            let temp_rect = Self::combine_rect(rect, cur_rect);
            let increase = Self::calc_rect_volume(&temp_rect) - area;
            if increase < best_incr || first_time {
                best = index;
                best_area = area;
                best_incr = increase;
                first_time = false;
            } else if increase == best_incr && area < best_area {
                best = index;
                best_area = area;
                best_incr = increase;
            }
        }
        best
    }

    /// C++ AddBranch: pushes the branch, or splits the node and returns the new node.
    fn add_branch(branch: Branch<D, E, NUMDIMS>, node: &mut Node<D, E, NUMDIMS>) -> Option<Box<Node<D, E, NUMDIMS>>> {
        if node.branches.len() < MAXNODES {
            node.branches.push(branch);
            None
        } else {
            Some(Self::split_node(node, branch))
        }
    }

    /// C++ InsertRectRec. Returns the new node when `node` was split.
    fn insert_rect_rec(
        rect: &Rect<E, NUMDIMS>,
        payload: Payload<D, E, NUMDIMS>,
        node: &mut Node<D, E, NUMDIMS>,
        level: i32,
    ) -> Option<Box<Node<D, E, NUMDIMS>>> {
        if node.level > level {
            // Still above level for insertion, go down the tree recursively
            let index = Self::pick_branch(rect, node);
            let split = match &mut node.branches[index].payload {
                Payload::Child(child) => Self::insert_rect_rec(rect, payload, child, level),
                Payload::Data(_) => unreachable!("internal branch must hold a child"),
            };
            match split {
                None => {
                    // Child was not split
                    let combined = Self::combine_rect(rect, &node.branches[index].rect);
                    node.branches[index].rect = combined;
                    None
                }
                Some(other) => {
                    // Child was split
                    let cover = Self::node_cover(node.branches[index].child());
                    node.branches[index].rect = cover;
                    let branch = Branch {
                        rect: Self::node_cover(&other),
                        payload: Payload::Child(other),
                    };
                    Self::add_branch(branch, node)
                }
            }
        } else if node.level == level {
            // Reached level for insertion. Add rect, split if necessary
            Self::add_branch(Branch { rect: *rect, payload }, node)
        } else {
            debug_assert!(false, "InsertRectRec: insertion level above node level");
            None
        }
    }

    /// C++ InsertRect: inserts at `level` (0 = leaf) and grows the root when it splits.
    fn insert_rect(
        rect: &Rect<E, NUMDIMS>,
        payload: Payload<D, E, NUMDIMS>,
        root: &mut Box<Node<D, E, NUMDIMS>>,
        level: i32,
    ) {
        debug_assert!(level >= 0 && level <= root.level);
        if let Some(new_node) = Self::insert_rect_rec(rect, payload, root, level) {
            // Root split: grow tree taller with a new root
            let mut new_root = Self::new_node(-1);
            new_root.level = root.level + 1;
            let old_cover = Self::node_cover(root);
            let new_cover = Self::node_cover(&new_node);
            let old_root = std::mem::replace(root, new_root);
            root.branches.push(Branch {
                rect: old_cover,
                payload: Payload::Child(old_root),
            });
            root.branches.push(Branch {
                rect: new_cover,
                payload: Payload::Child(new_node),
            });
        }
    }

    /// C++ SplitNode: divides the node's branches and the extra one between two nodes.
    fn split_node(node: &mut Node<D, E, NUMDIMS>, branch: Branch<D, E, NUMDIMS>) -> Box<Node<D, E, NUMDIMS>> {
        let level = node.level;
        let mut par_vars = PartitionVars {
            partition: Vec::new(),
            total: 0,
            min_fill: 0,
            taken: Vec::new(),
            count: [0, 0],
            cover: [Self::zero_rect(), Self::zero_rect()],
            area: [R::from_f64(0.0), R::from_f64(0.0)],
            branch_buf: Vec::new(),
            cover_split: Self::zero_rect(),
            cover_split_area: R::from_f64(0.0),
        };

        // Load all the branches into a buffer, initialize old node
        Self::get_branches(node, branch, &mut par_vars);

        // Find partition
        Self::choose_partition(&mut par_vars, MINNODES);

        // Put branches from buffer into 2 nodes according to chosen partition
        let mut new_node = Self::new_node(level);
        node.level = level;
        Self::load_nodes(node, &mut new_node, &mut par_vars);

        debug_assert!(node.branches.len() + new_node.branches.len() == par_vars.total);
        new_node
    }

    /// C++ GetBranches: moves the full node's branches plus the extra one into the buffer.
    fn get_branches(
        node: &mut Node<D, E, NUMDIMS>,
        branch: Branch<D, E, NUMDIMS>,
        par_vars: &mut PartitionVars<D, E, NUMDIMS, R>,
    ) {
        debug_assert!(node.branches.len() == MAXNODES);

        // C++ copies the branches and then InitNode() empties the node; here the branches are moved.
        let mut buffer = std::mem::take(&mut node.branches);
        buffer.push(branch);
        par_vars.branch_buf = buffer;

        // Calculate rect containing all in the set
        par_vars.cover_split = par_vars.branch_buf[0].rect;
        for index in 1..MAXNODES + 1 {
            par_vars.cover_split = Self::combine_rect(&par_vars.cover_split, &par_vars.branch_buf[index].rect);
        }
        par_vars.cover_split_area = Self::calc_rect_volume(&par_vars.cover_split);

        node.level = -1; // InitNode
    }

    /// C++ ChoosePartition (method #0). Groups fill up to `min_fill` at minimum.
    fn choose_partition(par_vars: &mut PartitionVars<D, E, NUMDIMS, R>, min_fill: usize) {
        let mut chosen = 0;
        let mut better_group = 0;

        Self::init_par_vars(par_vars, par_vars.branch_buf.len(), min_fill);
        Self::pick_seeds(par_vars);

        while (par_vars.count[0] + par_vars.count[1] < par_vars.total)
            && (par_vars.count[0] < par_vars.total - par_vars.min_fill)
            && (par_vars.count[1] < par_vars.total - par_vars.min_fill)
        {
            let mut biggest_diff = R::from_f64(-1.0);
            for index in 0..par_vars.total {
                if !par_vars.taken[index] {
                    let cur_rect = par_vars.branch_buf[index].rect;
                    let rect0 = Self::combine_rect(&cur_rect, &par_vars.cover[0]);
                    let rect1 = Self::combine_rect(&cur_rect, &par_vars.cover[1]);
                    let growth0 = Self::calc_rect_volume(&rect0) - par_vars.area[0];
                    let growth1 = Self::calc_rect_volume(&rect1) - par_vars.area[1];
                    let mut diff = growth1 - growth0;
                    let group = if diff >= R::from_f64(0.0) {
                        0
                    } else {
                        diff = -diff;
                        1
                    };

                    if diff > biggest_diff {
                        biggest_diff = diff;
                        chosen = index;
                        better_group = group;
                    } else if diff == biggest_diff && par_vars.count[group] < par_vars.count[better_group] {
                        chosen = index;
                        better_group = group;
                    }
                }
            }
            Self::classify(chosen, better_group, par_vars);
        }

        // If one group too full, put remaining rects in the other
        if par_vars.count[0] + par_vars.count[1] < par_vars.total {
            let group = if par_vars.count[0] >= par_vars.total - par_vars.min_fill { 1 } else { 0 };
            for index in 0..par_vars.total {
                if !par_vars.taken[index] {
                    Self::classify(index, group, par_vars);
                }
            }
        }

        debug_assert!(par_vars.count[0] + par_vars.count[1] == par_vars.total);
        debug_assert!(par_vars.count[0] >= par_vars.min_fill && par_vars.count[1] >= par_vars.min_fill);
    }

    /// C++ LoadNodes: copies branches from the buffer into the two nodes by partition.
    fn load_nodes(node_a: &mut Node<D, E, NUMDIMS>, node_b: &mut Node<D, E, NUMDIMS>, par_vars: &mut PartitionVars<D, E, NUMDIMS, R>) {
        let buffer = std::mem::take(&mut par_vars.branch_buf);
        for (index, branch) in buffer.into_iter().enumerate() {
            debug_assert!(par_vars.partition[index] == 0 || par_vars.partition[index] == 1);
            let split = if par_vars.partition[index] == 0 {
                Self::add_branch(branch, node_a)
            } else if par_vars.partition[index] == 1 {
                Self::add_branch(branch, node_b)
            } else {
                None
            };
            debug_assert!(split.is_none(), "LoadNodes: group cannot exceed MAXNODES");
        }
    }

    /// C++ InitParVars.
    fn init_par_vars(par_vars: &mut PartitionVars<D, E, NUMDIMS, R>, max_rects: usize, min_fill: usize) {
        par_vars.count = [0, 0];
        par_vars.area = [R::from_f64(0.0), R::from_f64(0.0)];
        par_vars.total = max_rects;
        par_vars.min_fill = min_fill;
        par_vars.taken = vec![false; max_rects];
        par_vars.partition = vec![-1; max_rects];
    }

    /// C++ PickSeeds: the two rects that would waste the most area together.
    fn pick_seeds(par_vars: &mut PartitionVars<D, E, NUMDIMS, R>) {
        let mut seed0 = 0;
        let mut seed1 = 1;
        let mut area = vec![R::from_f64(0.0); par_vars.total];

        for index in 0..par_vars.total {
            area[index] = Self::calc_rect_volume(&par_vars.branch_buf[index].rect);
        }

        let mut worst = -par_vars.cover_split_area - R::from_f64(1.0);
        for index_a in 0..par_vars.total - 1 {
            for index_b in index_a + 1..par_vars.total {
                let one_rect = Self::combine_rect(&par_vars.branch_buf[index_a].rect, &par_vars.branch_buf[index_b].rect);
                let waste = Self::calc_rect_volume(&one_rect) - area[index_a] - area[index_b];
                if waste > worst {
                    worst = waste;
                    seed0 = index_a;
                    seed1 = index_b;
                }
            }
        }
        Self::classify(seed0, 0, par_vars);
        Self::classify(seed1, 1, par_vars);
    }

    /// C++ Classify: puts a branch in one of the groups.
    fn classify(index: usize, group: usize, par_vars: &mut PartitionVars<D, E, NUMDIMS, R>) {
        debug_assert!(!par_vars.taken[index]);
        par_vars.partition[index] = group as i32;
        par_vars.taken[index] = true;

        if par_vars.count[group] == 0 {
            par_vars.cover[group] = par_vars.branch_buf[index].rect;
        } else {
            par_vars.cover[group] = Self::combine_rect(&par_vars.branch_buf[index].rect, &par_vars.cover[group]);
        }
        par_vars.area[group] = Self::calc_rect_volume(&par_vars.cover[group]);
        par_vars.count[group] += 1;
    }

    /// C++ RemoveRect. Returns true when the entry was not found.
    fn remove_rect(rect: &Rect<E, NUMDIMS>, data_id: &D, root: &mut Box<Node<D, E, NUMDIMS>>) -> bool {
        // C++ ListNode list is a stack (ReInsert pushes at the head), so it is used LIFO.
        let mut reinsert_list: Vec<Box<Node<D, E, NUMDIMS>>> = Vec::new();

        if !Self::remove_rect_rec(rect, data_id, root, &mut reinsert_list) {
            // Found and deleted a data item. Reinsert any branches from eliminated nodes
            while let Some(temp_node) = reinsert_list.pop() {
                let level = temp_node.level;
                for branch in temp_node.branches {
                    Self::insert_rect(&branch.rect, branch.payload, root, level);
                }
            }

            // Check for redundant root (not leaf, 1 child) and eliminate
            if root.branches.len() == 1 && root.is_internal() {
                if let Some(Branch {
                    payload: Payload::Child(only_child),
                    ..
                }) = root.branches.pop()
                {
                    *root = only_child;
                }
            }
            false
        } else {
            true
        }
    }

    /// C++ RemoveRectRec. Returns true when the entry was not found.
    fn remove_rect_rec(
        rect: &Rect<E, NUMDIMS>,
        data_id: &D,
        node: &mut Node<D, E, NUMDIMS>,
        reinsert_list: &mut Vec<Box<Node<D, E, NUMDIMS>>>,
    ) -> bool {
        if node.is_internal() {
            for index in 0..node.branches.len() {
                if Self::overlap(rect, &node.branches[index].rect) {
                    let not_found = match &mut node.branches[index].payload {
                        Payload::Child(child) => Self::remove_rect_rec(rect, data_id, child, reinsert_list),
                        Payload::Data(_) => unreachable!("internal branch must hold a child"),
                    };
                    if !not_found {
                        if node.branches[index].child().branches.len() >= MINNODES {
                            // child removed, just resize parent rect
                            let cover = Self::node_cover(node.branches[index].child());
                            node.branches[index].rect = cover;
                        } else {
                            // child removed, not enough entries in node, eliminate node
                            let removed = node.branches.swap_remove(index);
                            if let Payload::Child(child) = removed.payload {
                                reinsert_list.push(child);
                            }
                        }
                        return false;
                    }
                }
            }
            true
        } else {
            for index in 0..node.branches.len() {
                if matches!(&node.branches[index].payload, Payload::Data(id) if id == data_id) {
                    node.branches.swap_remove(index); // C++ DisconnectBranch
                    return false;
                }
            }
            true
        }
    }

    /// C++ Search (recursive part). Returns false to stop the search (never happens in RTree.h).
    fn search_rec<C, F>(
        node: &Node<D, E, NUMDIMS>,
        rect: &Rect<E, NUMDIMS>,
        found_count: &mut i32,
        context: &C,
        op: &mut F,
    ) -> bool
    where
        F: FnMut(&D, &C),
    {
        if node.is_internal() {
            for branch in &node.branches {
                if Self::overlap(rect, &branch.rect) && !Self::search_rec(branch.child(), rect, found_count, context, op) {
                    return false;
                }
            }
        } else {
            for branch in &node.branches {
                if Self::overlap(rect, &branch.rect) {
                    *found_count += 1;
                    op(branch.data(), context);
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Tree = RTree<u32, f32, 2, f32, 8, 4>;

    fn full_query() -> ([f32; 2], [f32; 2]) {
        ([-1000.0, -1000.0], [1000.0, 1000.0])
    }

    fn search_ids(t: &Tree, min: &[f32; 2], max: &[f32; 2]) -> (i32, Vec<u32>) {
        let mut hits = Vec::new();
        let n = t.search(min, max, &(), |id, _| hits.push(*id));
        (n, hits)
    }

    #[test]
    fn leaf_search_keeps_insertion_order() {
        let mut t = Tree::new();
        for i in 0..3u32 {
            let x = i as f32;
            t.insert(&[x, 0.0], &[x + 1.0, 1.0], &i);
        }
        let (min, max) = full_query();
        assert_eq!(search_ids(&t, &min, &max), (3, vec![0, 1, 2]));
    }

    #[test]
    fn remove_swaps_last_branch_into_gap() {
        // C++ DisconnectBranch copies the last branch into the removed slot: [0,1,2] minus 0 -> [2,1]
        let mut t = Tree::new();
        for i in 0..3u32 {
            let x = i as f32;
            t.insert(&[x, 0.0], &[x + 1.0, 1.0], &i);
        }
        t.remove(&[0.0, 0.0], &[1.0, 1.0], &0);
        let (min, max) = full_query();
        assert_eq!(search_ids(&t, &min, &max), (2, vec![2, 1]));
        assert_eq!(t.count(), 2);
    }

    #[test]
    fn split_keeps_all_entries_and_fill_bounds() {
        let mut t = Tree::new();
        for i in 0..9u32 {
            let x = i as f32;
            t.insert(&[x, 0.0], &[x + 1.0, 1.0], &i);
        }
        assert_eq!(t.count(), 9);
        // 9 entries with MAXNODES 8 must split the root: two children, each at least MINNODES 4
        assert_eq!(t.root.level, 1);
        assert_eq!(t.root.branches.len(), 2);
        let sizes: Vec<usize> = t.root.branches.iter().map(|b| b.child().branches.len()).collect();
        assert_eq!(sizes.iter().sum::<usize>(), 9);
        assert!(sizes.iter().all(|&s| s >= 4));

        // Window [2.5, 4.5] x [0, 1] overlaps rects i in {2, 3, 4}
        let (n, mut hits) = search_ids(&t, &[2.5, 0.0], &[4.5, 1.0]);
        hits.sort();
        assert_eq!(n, 3);
        assert_eq!(hits, vec![2, 3, 4]);
    }

    #[test]
    fn iterator_visits_every_entry_of_split_tree() {
        let mut t = Tree::new();
        for i in 0..9u32 {
            let x = i as f32;
            t.insert(&[x, 0.0], &[x + 1.0, 1.0], &i);
        }
        let mut seen = Vec::new();
        let mut it = t.get_first();
        while it.is_not_null() {
            seen.push(*it.get());
            it.advance();
        }
        seen.sort();
        assert_eq!(seen, (0..9).collect::<Vec<u32>>());
    }

    #[test]
    fn iterator_on_leaf_root_skips_first_entry_like_cpp() {
        // Faithful to RTree.h GetFirst/FindNextData: a leaf root yields entries 1..n.
        let mut t = Tree::new();
        for i in 0..3u32 {
            let x = i as f32;
            t.insert(&[x, 0.0], &[x + 1.0, 1.0], &i);
        }
        let mut seen = Vec::new();
        let mut it = t.get_first();
        while it.is_not_null() {
            seen.push(*it.get());
            it.advance();
        }
        assert_eq!(seen, vec![1, 2]);
    }

    #[test]
    fn random_insert_remove_matches_brute_force() {
        // Exercises splits, reinsertion after underflow and root collapse (debug asserts are on in tests)
        let mut seed: u64 = 12345;
        let mut next = move || {
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            (seed >> 33) as u32
        };
        let mut t = Tree::new();
        let mut live: Vec<(u32, [f32; 2], [f32; 2])> = Vec::new();
        for id in 0..400u32 {
            let x = (next() % 1000) as f32;
            let y = (next() % 1000) as f32;
            let w = (next() % 50) as f32;
            let h = (next() % 50) as f32;
            let (min, max) = ([x, y], [x + w, y + h]);
            t.insert(&min, &max, &id);
            live.push((id, min, max));
            if id % 3 == 2 {
                let k = next() as usize % live.len();
                let (rid, rmin, rmax) = live.swap_remove(k);
                t.remove(&rmin, &rmax, &rid);
            }
        }
        assert_eq!(t.count() as usize, live.len());

        for _ in 0..50 {
            let qx = (next() % 1000) as f32;
            let qy = (next() % 1000) as f32;
            let qmin = [qx, qy];
            let qmax = [qx + 100.0, qy + 100.0];
            let mut expected: Vec<u32> = live
                .iter()
                .filter(|(_, min, max)| (0..2).all(|d| !(min[d] > qmax[d] || qmin[d] > max[d])))
                .map(|(id, _, _)| *id)
                .collect();
            let (n, mut got) = search_ids(&t, &qmin, &qmax);
            assert_eq!(n as usize, got.len());
            expected.sort();
            got.sort();
            assert_eq!(got, expected);
        }
    }

    #[test]
    fn spherical_volume_matches_cpp_formula() {
        // 2D square side 2: radius sqrt(2), volume = r*r*pi = 2*pi
        let rect = Rect { min: [0.0f32, 0.0], max: [2.0, 2.0] };
        let v = Tree::calc_rect_volume(&rect);
        assert!((v - 2.0 * std::f32::consts::PI).abs() < 1e-4, "{v}");

        // 3D cube side 2: radius sqrt(3), volume = r^3 * 4.188790 (table value)
        let cube = RTree::<u32, f32, 3, f32, 8, 4, TemplateVolume>::calc_rect_volume(&Rect {
            min: [0.0f32, 0.0, 0.0],
            max: [2.0, 2.0, 2.0],
        });
        let expected = 3.0f32.sqrt().powi(3) * 4.188790;
        assert!((cube - expected).abs() < 1e-3, "{cube} vs {expected}");
    }

    #[test]
    fn float_2d_volume_matches_named_rtree_override() {
        // C++ NamedRTree: .78539816f * (extent0 * extent0 + extent1 * extent1), exact float ops
        type NamedTree = RTree<u32, f32, 2, f32, 8, 4, Float2DVolume>;
        let square = Rect { min: [0.0f32, 0.0], max: [2.0, 2.0] };
        assert_eq!(NamedTree::calc_rect_volume(&square), 0.785_398_16_f32 * (2.0f32 * 2.0 + 2.0 * 2.0));
        let wide = Rect { min: [1.0f32, -1.0], max: [4.0, 0.0] };
        assert_eq!(NamedTree::calc_rect_volume(&wide), 0.785_398_16_f32 * (3.0f32 * 3.0 + 1.0 * 1.0));
    }
}
