//! Port of foreign/RoutingKit/include/routingkit/customizable_contraction_hierarchy.h and
//! foreign/RoutingKit/src/customizable_contraction_hierarchy.cpp.
//!
//! Ported: CCH construction from a node order, serial metric customization, partial
//! customization, and the query (distance and node path), i.e. what SUMO uses.
//! Not ported (unused by SUMO): CustomizableContractionHierarchyParallelization, the
//! one-to-many / many-to-one pinned queries, get_arc_path with original-arc unpacking,
//! build_contraction_hierarchy_using_perfect_witness_search, the filter_always_inf_arcs
//! option (SUMO uses the default, false), and the log_message callbacks.
//! // shortcut: CCH build logging is dropped, add a log callback when a caller needs it.
//! The C++ query state machine only feeds assertions, so it is not kept.
//!
//! determinism: all sorts that define the CCH arc numbering are stable (see sort.rs), so
//! arc and node numbering match the C++ build. Path unpacking iterates in the same order.
//! Distances use wrapping u32 addition, as the C++ unsigned arithmetic does.

use utils::common::util_exceptions::ProcessError;

use super::bit_vector::BitVector;
use super::constants::{INF_WEIGHT, INVALID_ID};
use super::filter::inplace_keep_element_of_vector_if;
use super::graph_util::{
    compute_inverse_sort_permutation_first_by_tail_then_by_head_and_apply_sort_to_tail,
    compute_sort_permutation_first_by_tail_then_by_head_and_apply_sort_to_tail,
    find_arc_given_sorted_head,
};
use super::id_mapper::LocalIDMapper;
use super::id_set_queue::IDSetMinQueue;
use super::inverse_vector::invert_vector;
use super::min_max::min_to;
use super::permutation::{
    apply_inverse_permutation, apply_permutation, apply_permutation_to_elements_of, invert_permutation,
};
use super::sort::compute_inverse_stable_sort_permutation_using_key;

/// The metric-independent CCH topology over a directed graph.
#[derive(Clone, Debug, Default)]
pub struct CustomizableContractionHierarchy {
    pub order: Vec<u32>,
    pub rank: Vec<u32>,

    pub elimination_tree_parent: Vec<u32>,

    pub up_first_out: Vec<u32>,
    pub up_head: Vec<u32>,
    pub up_tail: Vec<u32>,

    pub down_first_out: Vec<u32>,
    pub down_head: Vec<u32>,
    pub down_to_up: Vec<u32>,

    pub input_arc_to_cch_arc: Vec<u32>,
    pub is_input_arc_upward: BitVector,

    pub does_cch_arc_have_input_arc: BitVector,
    pub does_cch_arc_have_input_arc_mapper: LocalIDMapper,

    pub forward_input_arc_of_cch: Vec<u32>,
    pub backward_input_arc_of_cch: Vec<u32>,

    pub does_cch_arc_have_extra_input_arc: BitVector,
    pub does_cch_arc_have_extra_input_arc_mapper: LocalIDMapper,

    pub first_extra_forward_input_arc_of_cch: Vec<u32>,
    pub first_extra_backward_input_arc_of_cch: Vec<u32>,

    pub extra_forward_input_arc_of_cch: Vec<u32>,
    pub extra_backward_input_arc_of_cch: Vec<u32>,
}

impl CustomizableContractionHierarchy {
    /// CustomizableContractionHierarchy(order, tail, head)
    pub fn new(order: Vec<u32>, mut input_tail: Vec<u32>, mut input_head: Vec<u32>) -> Result<Self, ProcessError> {
        let mut c = CustomizableContractionHierarchy { order, ..Default::default() };
        let node_count = c.order.len() as u32;
        let input_arc_count = input_tail.len();

        c.rank = invert_permutation(&c.order);

        input_tail = apply_permutation_to_elements_of(&c.rank, &input_tail);
        input_head = apply_permutation_to_elements_of(&c.rank, &input_head);

        let mut input_arc_id = compute_sort_permutation_first_by_tail_then_by_head_and_apply_sort_to_tail(
            node_count,
            &mut input_tail,
            &input_head,
        );
        input_head = apply_permutation(&input_arc_id, &input_head);

        // Compute up graph: symmetric, duplicate-free, then the chordal supergraph
        {
            let mut symmetric_tail = [input_tail.clone(), input_head.clone()].concat();
            let mut symmetric_head = [input_head.clone(), input_tail.clone()].concat();

            {
                let p = compute_inverse_sort_permutation_first_by_tail_then_by_head_and_apply_sort_to_tail(
                    node_count,
                    &mut symmetric_tail,
                    &symmetric_head,
                );
                symmetric_head = apply_inverse_permutation(&p, &symmetric_head);
            }

            let total = 2 * input_arc_count;
            let mut filter = BitVector::uninitialized(total as u64);
            if input_arc_count != 0 {
                filter.set_to(0, symmetric_tail[0] != symmetric_head[0]);
            }
            for i in 1..total {
                filter.set_to(
                    i as u64,
                    (symmetric_head[i] != symmetric_head[i - 1] || symmetric_tail[i] != symmetric_tail[i - 1])
                        && symmetric_tail[i] != symmetric_head[i],
                );
            }

            inplace_keep_element_of_vector_if(&filter, &mut symmetric_tail);
            inplace_keep_element_of_vector_if(&filter, &mut symmetric_head);

            let mut up_tail = Vec::new();
            let mut up_head = Vec::new();
            compute_chordal_supergraph(node_count, &symmetric_tail, &symmetric_head, &mut |x, y| {
                if up_tail.len() == INVALID_ID as usize {
                    return Err(ProcessError::new("CCH must contain at most 2^32-1 arcs"));
                }
                up_tail.push(x);
                up_head.push(y);
                Ok(())
            })?;

            {
                let p = compute_inverse_sort_permutation_first_by_tail_then_by_head_and_apply_sort_to_tail(
                    node_count,
                    &mut up_tail,
                    &up_head,
                );
                up_head = apply_inverse_permutation(&p, &up_head);
            }

            c.up_first_out = invert_vector(&up_tail, node_count);
            c.up_tail = up_tail;
            c.up_head = up_head;
        }

        let cch_arc_count = c.up_tail.len();

        // Compute input -> ch mapping
        if cch_arc_count == 0 {
            c.input_arc_to_cch_arc = vec![INVALID_ID; input_arc_count];
            c.is_input_arc_upward = BitVector::new(input_arc_count as u64, false);
        } else {
            c.input_arc_to_cch_arc = vec![0; input_arc_count];
            c.is_input_arc_upward = BitVector::new(input_arc_count as u64, false);

            {
                let mut cch_up_arc = 0usize;
                for input_arc in 0..input_arc_count {
                    if input_tail[input_arc] < input_head[input_arc] {
                        while input_tail[input_arc] != c.up_tail[cch_up_arc]
                            || input_head[input_arc] != c.up_head[cch_up_arc]
                        {
                            cch_up_arc += 1;
                        }
                        c.input_arc_to_cch_arc[input_arc_id[input_arc] as usize] = cch_up_arc as u32;
                        c.is_input_arc_upward.set(input_arc_id[input_arc] as u64);
                    } else if input_tail[input_arc] == input_head[input_arc] {
                        c.input_arc_to_cch_arc[input_arc_id[input_arc] as usize] = INVALID_ID; // input arc is a loop
                    }
                }
            }

            {
                std::mem::swap(&mut input_tail, &mut input_head);
                let p = compute_inverse_sort_permutation_first_by_tail_then_by_head_and_apply_sort_to_tail(
                    node_count,
                    &mut input_tail,
                    &input_head,
                );
                input_head = apply_inverse_permutation(&p, &input_head);
                input_arc_id = apply_inverse_permutation(&p, &input_arc_id);
            }

            {
                let mut cch_up_arc = 0usize;
                for input_arc in 0..input_arc_count {
                    if input_tail[input_arc] < input_head[input_arc] {
                        while input_tail[input_arc] != c.up_tail[cch_up_arc]
                            || input_head[input_arc] != c.up_head[cch_up_arc]
                        {
                            cch_up_arc += 1;
                        }
                        c.input_arc_to_cch_arc[input_arc_id[input_arc] as usize] = cch_up_arc as u32;
                    }
                }
            }
        }

        // Compute elimination tree
        c.elimination_tree_parent = vec![INVALID_ID; node_count as usize];
        for x in 0..node_count as usize {
            if c.up_first_out[x] != c.up_first_out[x + 1] {
                c.elimination_tree_parent[x] = c.up_head[c.up_first_out[x] as usize];
            }
        }

        // Compute down graph
        {
            let mut down_tail = c.up_head.clone();
            c.down_head = c.up_tail.clone();
            c.down_to_up = compute_sort_permutation_first_by_tail_then_by_head_and_apply_sort_to_tail(
                node_count,
                &mut down_tail,
                &c.down_head,
            );
            c.down_head = apply_permutation(&c.down_to_up, &c.down_head);
            c.down_first_out = invert_vector(&down_tail, node_count);
        }

        // Compute ch mapping -> input
        {
            c.does_cch_arc_have_input_arc = BitVector::new(cch_arc_count as u64, false);
            for i in 0..input_arc_count {
                if c.input_arc_to_cch_arc[i] != INVALID_ID {
                    c.does_cch_arc_have_input_arc.set(c.input_arc_to_cch_arc[i] as u64);
                }
            }

            c.does_cch_arc_have_input_arc_mapper = LocalIDMapper::new(&c.does_cch_arc_have_input_arc);
            let local_count = c.does_cch_arc_have_input_arc_mapper.local_id_count() as usize;
            c.forward_input_arc_of_cch = vec![INVALID_ID; local_count];
            c.backward_input_arc_of_cch = vec![INVALID_ID; local_count];

            c.does_cch_arc_have_extra_input_arc = BitVector::new(cch_arc_count as u64, false);

            for input_arc in 0..input_arc_count {
                let cch_arc = c.input_arc_to_cch_arc[input_arc];
                if cch_arc != INVALID_ID {
                    let i = c.does_cch_arc_have_input_arc_mapper.to_local(cch_arc as u64) as usize;
                    if c.is_input_arc_upward.is_set(input_arc as u64) {
                        if c.forward_input_arc_of_cch[i] == INVALID_ID {
                            c.forward_input_arc_of_cch[i] = input_arc as u32;
                        } else {
                            c.does_cch_arc_have_extra_input_arc.set(cch_arc as u64);
                            c.first_extra_forward_input_arc_of_cch.push(cch_arc);
                            c.extra_forward_input_arc_of_cch.push(input_arc as u32);
                        }
                    } else if c.backward_input_arc_of_cch[i] == INVALID_ID {
                        c.backward_input_arc_of_cch[i] = input_arc as u32;
                    } else {
                        c.does_cch_arc_have_extra_input_arc.set(cch_arc as u64);
                        c.first_extra_backward_input_arc_of_cch.push(cch_arc);
                        c.extra_backward_input_arc_of_cch.push(input_arc as u32);
                    }
                }
            }

            c.does_cch_arc_have_extra_input_arc_mapper = LocalIDMapper::new(&c.does_cch_arc_have_extra_input_arc);
            let extra_count = c.does_cch_arc_have_extra_input_arc_mapper.local_id_count() as u32;

            for x in c.first_extra_forward_input_arc_of_cch.iter_mut() {
                *x = c.does_cch_arc_have_extra_input_arc_mapper.to_local(*x as u64) as u32;
            }
            for x in c.first_extra_backward_input_arc_of_cch.iter_mut() {
                *x = c.does_cch_arc_have_extra_input_arc_mapper.to_local(*x as u64) as u32;
            }

            {
                let p = compute_inverse_stable_sort_permutation_using_key(
                    &c.first_extra_forward_input_arc_of_cch,
                    extra_count,
                    &|x: u32| x,
                );
                c.first_extra_forward_input_arc_of_cch = invert_vector(
                    &apply_inverse_permutation(&p, &c.first_extra_forward_input_arc_of_cch),
                    extra_count,
                );
                c.extra_forward_input_arc_of_cch = apply_inverse_permutation(&p, &c.extra_forward_input_arc_of_cch);
            }

            {
                let p = compute_inverse_stable_sort_permutation_using_key(
                    &c.first_extra_backward_input_arc_of_cch,
                    extra_count,
                    &|x: u32| x,
                );
                c.first_extra_backward_input_arc_of_cch = invert_vector(
                    &apply_inverse_permutation(&p, &c.first_extra_backward_input_arc_of_cch),
                    extra_count,
                );
                c.extra_backward_input_arc_of_cch = apply_inverse_permutation(&p, &c.extra_backward_input_arc_of_cch);
            }
        }

        Ok(c)
    }

    pub fn node_count(&self) -> u32 {
        self.rank.len() as u32
    }

    pub fn input_arc_count(&self) -> u32 {
        self.input_arc_to_cch_arc.len() as u32
    }

    pub fn cch_arc_count(&self) -> u32 {
        self.up_head.len() as u32
    }
}

/// compute_chordal_supergraph: calls on_new_arc(x, y) for every upward arc of the chordal
/// supergraph, grouped by x in ascending order.
fn compute_chordal_supergraph<F: FnMut(u32, u32) -> Result<(), ProcessError>>(
    node_count: u32,
    tail: &[u32],
    head: &[u32],
    on_new_arc: &mut F,
) -> Result<(), ProcessError> {
    let n = node_count as usize;
    let mut nodes: Vec<Vec<u32>> = vec![Vec::new(); n];
    for i in 0..tail.len() {
        if tail[i] < head[i] {
            nodes[tail[i] as usize].push(head[i]);
        }
    }
    for list in nodes.iter_mut() {
        list.sort_unstable();
        list.dedup();
    }

    for x in 0..n {
        if nodes[x].is_empty() {
            continue;
        }
        let lowest_neighbor = nodes[x][0] as usize;
        let merged = merge_unique(&nodes[x][1..], &nodes[lowest_neighbor]);
        nodes[lowest_neighbor] = merged;

        for k in 0..nodes[x].len() {
            let neighbor = nodes[x][k];
            on_new_arc(x as u32, neighbor)?;
        }
    }
    Ok(())
}

/// std::merge followed by std::unique on two sorted lists
fn merge_unique(a: &[u32], b: &[u32]) -> Vec<u32> {
    let mut out: Vec<u32> = Vec::with_capacity(a.len() + b.len());
    let (mut i, mut j) = (0usize, 0usize);
    while i < a.len() || j < b.len() {
        let next = if j == b.len() || (i < a.len() && a[i] <= b[j]) {
            i += 1;
            a[i - 1]
        } else {
            j += 1;
            b[j - 1]
        };
        if out.last() != Some(&next) {
            out.push(next);
        }
    }
    out
}

// Triangle enumeration. A triangle {x,y,z} with ranks x<y<z has bottom arc xy, mid arc xz and
// top arc yz. Callbacks get (bottom_arc, mid_arc, top_arc, bottom_node, mid_node, top_node)
// and return false to abort the enumeration.

fn forall_upper_triangles_of_arc_xy<F: FnMut(u32, u32, u32, u32, u32, u32) -> bool>(
    cch: &CustomizableContractionHierarchy,
    x: u32,
    y: u32,
    xy: u32,
    f: &mut F,
) -> bool {
    let mut x_up_arc = xy + 1;
    let x_up_arc_end = cch.up_first_out[x as usize + 1];

    let mut y_up_arc = cch.up_first_out[y as usize];
    let y_up_arc_end = cch.up_first_out[y as usize + 1];

    while x_up_arc != x_up_arc_end && y_up_arc != y_up_arc_end {
        let xh = cch.up_head[x_up_arc as usize];
        let yh = cch.up_head[y_up_arc as usize];
        if xh < yh {
            x_up_arc += 1;
        } else if xh > yh {
            y_up_arc += 1;
        } else {
            let z = xh;
            if !f(xy, x_up_arc, y_up_arc, x, y, z) {
                return false;
            }
            x_up_arc += 1;
            y_up_arc += 1;
        }
    }
    true
}

fn forall_upper_triangles_of_arc<F: FnMut(u32, u32, u32, u32, u32, u32) -> bool>(
    cch: &CustomizableContractionHierarchy,
    xy: u32,
    f: &mut F,
) -> bool {
    forall_upper_triangles_of_arc_xy(cch, cch.up_tail[xy as usize], cch.up_head[xy as usize], xy, f)
}

fn forall_intermediate_triangles_of_arc_xy<F: FnMut(u32, u32, u32, u32, u32, u32) -> bool>(
    cch: &CustomizableContractionHierarchy,
    x: u32,
    y: u32,
    xy: u32,
    f: &mut F,
) -> bool {
    let mut x_up_arc = cch.up_first_out[x as usize];
    let x_up_arc_end = xy;

    let mut y_down_arc = cch.down_first_out[y as usize];
    let y_down_arc_end = cch.down_first_out[y as usize + 1];

    while x_up_arc != x_up_arc_end && y_down_arc != y_down_arc_end {
        let xh = cch.up_head[x_up_arc as usize];
        let yh = cch.down_head[y_down_arc as usize];
        if xh < yh {
            x_up_arc += 1;
        } else if xh > yh {
            y_down_arc += 1;
        } else {
            let z = xh;
            if !f(x_up_arc, xy, cch.down_to_up[y_down_arc as usize], x, z, y) {
                return false;
            }
            x_up_arc += 1;
            y_down_arc += 1;
        }
    }
    true
}

fn forall_intermediate_triangles_of_arc<F: FnMut(u32, u32, u32, u32, u32, u32) -> bool>(
    cch: &CustomizableContractionHierarchy,
    xy: u32,
    f: &mut F,
) -> bool {
    forall_intermediate_triangles_of_arc_xy(cch, cch.up_tail[xy as usize], cch.up_head[xy as usize], xy, f)
}

fn forall_lower_triangles_of_arc_xy<F: FnMut(u32, u32, u32, u32, u32, u32) -> bool>(
    cch: &CustomizableContractionHierarchy,
    x: u32,
    y: u32,
    xy: u32,
    f: &mut F,
) -> bool {
    let mut x_down_arc = cch.down_first_out[x as usize];
    let x_down_arc_end = cch.down_first_out[x as usize + 1];

    let mut y_down_arc = cch.down_first_out[y as usize];
    let y_down_arc_end = cch.down_first_out[y as usize + 1];

    while x_down_arc != x_down_arc_end && y_down_arc != y_down_arc_end {
        let xh = cch.down_head[x_down_arc as usize];
        let yh = cch.down_head[y_down_arc as usize];
        if xh < yh {
            x_down_arc += 1;
        } else if xh > yh {
            y_down_arc += 1;
        } else {
            let z = xh;
            if !f(
                cch.down_to_up[x_down_arc as usize],
                cch.down_to_up[y_down_arc as usize],
                xy,
                z,
                x,
                y,
            ) {
                return false;
            }
            x_down_arc += 1;
            y_down_arc += 1;
        }
    }
    true
}

fn forall_lower_triangles_of_arc<F: FnMut(u32, u32, u32, u32, u32, u32) -> bool>(
    cch: &CustomizableContractionHierarchy,
    xy: u32,
    f: &mut F,
) -> bool {
    forall_lower_triangles_of_arc_xy(cch, cch.up_tail[xy as usize], cch.up_head[xy as usize], xy, f)
}

/// A metric: forward and backward weight per CCH arc, derived from input weights.
#[derive(Clone, Debug, Default)]
pub struct CustomizableContractionHierarchyMetric<'a> {
    pub forward: Vec<u32>,
    pub backward: Vec<u32>,
    cch: Option<&'a CustomizableContractionHierarchy>,
    input_weight: &'a [u32],
}

impl<'a> CustomizableContractionHierarchyMetric<'a> {
    /// CustomizableContractionHierarchyMetric(cch, input_weight). The weights are borrowed,
    /// as the C++ class keeps a pointer to them.
    pub fn new(cch: &'a CustomizableContractionHierarchy, input_weight: &'a [u32]) -> Self {
        let n = cch.cch_arc_count() as usize;
        CustomizableContractionHierarchyMetric {
            forward: vec![0; n],
            backward: vec![0; n],
            cch: Some(cch),
            input_weight,
        }
    }

    fn attached_cch(&self) -> &'a CustomizableContractionHierarchy {
        self.cch.expect("Metric must be attached to a CCH")
    }

    /// customize(): serial customization of all arcs
    pub fn customize(&mut self) -> &mut Self {
        let cch = self.attached_cch();

        for cch_arc in 0..cch.cch_arc_count() {
            extract_initial_metric_of_cch_arc(cch, self, cch_arc);
        }

        let node_count = cch.node_count() as usize;
        let mut arc_id_cache = vec![0u32; node_count];

        for x in 0..node_count {
            let xz_up_end = cch.up_first_out[x + 1];
            for xz_up in cch.up_first_out[x]..xz_up_end {
                arc_id_cache[cch.up_head[xz_up as usize] as usize] = xz_up;
            }

            let xy_down_end = cch.down_first_out[x + 1];
            for xy_down in cch.down_first_out[x]..xy_down_end {
                let yx_up = cch.down_to_up[xy_down as usize];
                let y = cch.down_head[xy_down as usize] as usize;
                let yz_up_end_reversed = cch.up_first_out[y];
                let mut yz_up_reversed = cch.up_first_out[y + 1];
                while yz_up_reversed > yz_up_end_reversed {
                    let yz_up = yz_up_reversed - 1;
                    let z = cch.up_head[yz_up as usize];
                    if z <= x as u32 {
                        break;
                    }
                    lower_triangle_relax(self, yx_up, yz_up, arc_id_cache[z as usize]);
                    yz_up_reversed -= 1;
                }
            }
        }
        self
    }
}

fn extract_initial_metric_of_cch_arc(
    cch: &CustomizableContractionHierarchy,
    metric: &mut CustomizableContractionHierarchyMetric,
    cch_arc: u32,
) {
    let a = cch_arc as usize;
    if !cch.does_cch_arc_have_input_arc.is_set(cch_arc as u64) {
        metric.forward[a] = INF_WEIGHT;
        metric.backward[a] = INF_WEIGHT;
    } else {
        {
            let i = cch.does_cch_arc_have_input_arc_mapper.to_local(cch_arc as u64) as usize;
            metric.forward[a] = if cch.forward_input_arc_of_cch[i] != INVALID_ID {
                metric.input_weight[cch.forward_input_arc_of_cch[i] as usize]
            } else {
                INF_WEIGHT
            };
            metric.backward[a] = if cch.backward_input_arc_of_cch[i] != INVALID_ID {
                metric.input_weight[cch.backward_input_arc_of_cch[i] as usize]
            } else {
                INF_WEIGHT
            };
        }
        if cch.does_cch_arc_have_extra_input_arc.is_set(cch_arc as u64) {
            let i = cch.does_cch_arc_have_extra_input_arc_mapper.to_local(cch_arc as u64) as usize;
            for j in cch.first_extra_forward_input_arc_of_cch[i]..cch.first_extra_forward_input_arc_of_cch[i + 1] {
                let w = metric.input_weight[cch.extra_forward_input_arc_of_cch[j as usize] as usize];
                min_to(&mut metric.forward[a], w);
            }
            for j in cch.first_extra_backward_input_arc_of_cch[i]..cch.first_extra_backward_input_arc_of_cch[i + 1] {
                let w = metric.input_weight[cch.extra_backward_input_arc_of_cch[j as usize] as usize];
                min_to(&mut metric.backward[a], w);
            }
        }
    }
}

/// LowerTriangleRelaxer: relax the top arc of a lower triangle
fn lower_triangle_relax(metric: &mut CustomizableContractionHierarchyMetric, bottom_arc: u32, mid_arc: u32, top_arc: u32) {
    let (b, m, t) = (bottom_arc as usize, mid_arc as usize, top_arc as usize);
    let v = metric.backward[b].wrapping_add(metric.forward[m]);
    min_to(&mut metric.forward[t], v);
    let w = metric.forward[b].wrapping_add(metric.backward[m]);
    min_to(&mut metric.backward[t], w);
}

/// Partial customization: re-customizes only the arcs affected by changed input weights.
pub struct CustomizableContractionHierarchyPartialCustomization<'a> {
    q: IDSetMinQueue,
    cch: &'a CustomizableContractionHierarchy,
}

impl<'a> CustomizableContractionHierarchyPartialCustomization<'a> {
    pub fn new(cch: &'a CustomizableContractionHierarchy) -> Self {
        CustomizableContractionHierarchyPartialCustomization {
            q: IDSetMinQueue::new(cch.cch_arc_count()),
            cch,
        }
    }

    /// reset(cch)
    pub fn reset(&mut self, cch: &'a CustomizableContractionHierarchy) -> &mut Self {
        if cch.cch_arc_count() != self.q.id_count() {
            *self = Self::new(cch);
        } else {
            self.q.clear();
            self.cch = cch;
        }
        self
    }

    /// update_arc(xy): xy is an input arc id
    pub fn update_arc(&mut self, xy: u32) -> &mut Self {
        let cch_arc = self.cch.input_arc_to_cch_arc[xy as usize];
        if cch_arc != INVALID_ID {
            self.q.push(cch_arc);
        }
        self
    }

    /// customize(metric)
    pub fn customize(&mut self, metric: &mut CustomizableContractionHierarchyMetric) -> &mut Self {
        let cch = self.cch;

        while !self.q.empty() {
            let xy = self.q.pop();

            let old_forward = metric.forward[xy as usize];
            let old_backward = metric.backward[xy as usize];

            metric.forward[xy as usize] = INF_WEIGHT;
            metric.backward[xy as usize] = INF_WEIGHT;

            extract_initial_metric_of_cch_arc(cch, metric, xy);

            forall_lower_triangles_of_arc(cch, xy, &mut |bottom_arc, mid_arc, top_arc, _, _, _| {
                lower_triangle_relax(metric, bottom_arc, mid_arc, top_arc);
                true
            });

            let new_forward = metric.forward[xy as usize];
            let new_backward = metric.backward[xy as usize];

            if old_forward != new_forward || old_backward != new_backward {
                let q = &mut self.q;
                let m = &*metric;
                forall_intermediate_triangles_of_arc(cch, xy, &mut |bottom_arc, _mid_arc, top_arc, _, _, _| {
                    let (b, t) = (bottom_arc as usize, top_arc as usize);
                    if m.backward[b].wrapping_add(old_forward) == m.forward[t]
                        || m.forward[b].wrapping_add(old_backward) == m.backward[t]
                        || m.backward[b].wrapping_add(new_forward) < m.forward[t]
                        || m.forward[b].wrapping_add(new_backward) < m.backward[t]
                    {
                        q.push(top_arc);
                    }
                    true
                });
                forall_upper_triangles_of_arc(cch, xy, &mut |_bottom_arc, mid_arc, top_arc, _, _, _| {
                    let (mi, t) = (mid_arc as usize, top_arc as usize);
                    if m.forward[mi].wrapping_add(old_backward) == m.forward[t]
                        || m.backward[mi].wrapping_add(old_forward) == m.backward[t]
                        || m.forward[mi].wrapping_add(new_backward) < m.forward[t]
                        || m.backward[mi].wrapping_add(new_forward) < m.backward[t]
                    {
                        q.push(top_arc);
                    }
                    true
                });
            }
        }
        self
    }
}

fn forall_ancestors_reset(parent: &[u32], start: u32, stop_at: u32, mut f: impl FnMut(u32)) {
    let mut x = start;
    while x != stop_at {
        f(x);
        x = parent[x as usize];
    }
}

/// reset_source_list: clears the search space marked by the sources (or targets)
fn reset_source_list(
    elimination_tree_parent: &[u32],
    source_node: &mut Vec<u32>,
    source_elimination_tree_end: &mut Vec<u32>,
    in_search_space: &mut [bool],
    tentative_distance: &mut [u32],
) {
    for i in 0..source_node.len() {
        forall_ancestors_reset(elimination_tree_parent, source_node[i], source_elimination_tree_end[i], |x| {
            in_search_space[x as usize] = false;
            tentative_distance[x as usize] = INF_WEIGHT;
        });
    }
    source_node.clear();
    source_elimination_tree_end.clear();
}

/// internal_add_source: shared by add_source and add_target
#[allow(clippy::too_many_arguments)]
fn internal_add_source(
    external_s: u32,
    dist_to_s: u32,
    rank: &[u32],
    elimination_tree_parent: &[u32],
    tentative_distance: &mut [u32],
    predecessor_node: &mut [u32],
    in_search_space: &mut [bool],
    source_node: &mut Vec<u32>,
    source_elimination_tree_end: &mut Vec<u32>,
) {
    let s = rank[external_s as usize];

    if tentative_distance[s as usize] == INF_WEIGHT {
        source_node.push(s);
        tentative_distance[s as usize] = dist_to_s;
        predecessor_node[s as usize] = INVALID_ID;

        // walk up the elimination tree until a node already in the search space is found
        let mut x = s;
        while x != INVALID_ID {
            if !in_search_space[x as usize] {
                in_search_space[x as usize] = true;
                x = elimination_tree_parent[x as usize];
            } else {
                source_elimination_tree_end.push(x);
                break;
            }
        }
        if source_elimination_tree_end.len() != source_node.len() {
            source_elimination_tree_end.push(INVALID_ID);
        }
    } else {
        min_to(&mut tentative_distance[s as usize], dist_to_s);
    }
}

/// relax_outgoing_arcs: relaxes the up arcs of x with the given weights
fn relax_outgoing_arcs(
    first_out: &[u32],
    head: &[u32],
    weight: &[u32],
    tentative_distance: &mut [u32],
    predecessor_node: &mut [u32],
    x: u32,
) {
    for xy in first_out[x as usize]..first_out[x as usize + 1] {
        let y = head[xy as usize];
        let candidate = tentative_distance[x as usize].wrapping_add(weight[xy as usize]);
        if candidate < tentative_distance[y as usize] {
            tentative_distance[y as usize] = candidate;
            predecessor_node[y as usize] = x;
        }
    }
}

/// Query on a customized metric: bidirectional upward search over the elimination tree.
pub struct CustomizableContractionHierarchyQuery<'a> {
    forward_tentative_distance: Vec<u32>,
    backward_tentative_distance: Vec<u32>,
    source_node: Vec<u32>,
    source_elimination_tree_end: Vec<u32>,
    target_node: Vec<u32>,
    target_elimination_tree_end: Vec<u32>,

    forward_predecessor_node: Vec<u32>,
    backward_predecessor_node: Vec<u32>,

    in_forward_search_space: Vec<bool>,
    in_backward_search_space: Vec<bool>,

    shortest_path_meeting_node: u32,

    cch: &'a CustomizableContractionHierarchy,
    metric: &'a CustomizableContractionHierarchyMetric<'a>,
}

impl<'a> CustomizableContractionHierarchyQuery<'a> {
    /// CustomizableContractionHierarchyQuery(metric)
    pub fn new(metric: &'a CustomizableContractionHierarchyMetric<'a>) -> Self {
        let cch = metric.attached_cch();
        let n = cch.node_count() as usize;
        CustomizableContractionHierarchyQuery {
            forward_tentative_distance: vec![INF_WEIGHT; n],
            backward_tentative_distance: vec![INF_WEIGHT; n],
            source_node: Vec::new(),
            source_elimination_tree_end: Vec::new(),
            target_node: Vec::new(),
            target_elimination_tree_end: Vec::new(),
            forward_predecessor_node: vec![0; n],
            backward_predecessor_node: vec![0; n],
            in_forward_search_space: vec![false; n],
            in_backward_search_space: vec![false; n],
            shortest_path_meeting_node: INVALID_ID,
            cch,
            metric,
        }
    }

    /// reset()
    pub fn reset(&mut self) -> &mut Self {
        let cch = self.cch;
        reset_source_list(
            &cch.elimination_tree_parent,
            &mut self.source_node,
            &mut self.source_elimination_tree_end,
            &mut self.in_forward_search_space,
            &mut self.forward_tentative_distance,
        );
        reset_source_list(
            &cch.elimination_tree_parent,
            &mut self.target_node,
            &mut self.target_elimination_tree_end,
            &mut self.in_backward_search_space,
            &mut self.backward_tentative_distance,
        );
        self
    }

    /// reset(metric)
    pub fn reset_with(&mut self, metric: &'a CustomizableContractionHierarchyMetric<'a>) -> &mut Self {
        if std::ptr::eq(self.cch, metric.attached_cch()) {
            self.metric = metric;
            self.reset();
        } else {
            *self = Self::new(metric);
        }
        self
    }

    pub fn add_source(&mut self, external_s: u32, dist_to_s: u32) -> &mut Self {
        let cch = self.cch;
        internal_add_source(
            external_s,
            dist_to_s,
            &cch.rank,
            &cch.elimination_tree_parent,
            &mut self.forward_tentative_distance,
            &mut self.forward_predecessor_node,
            &mut self.in_forward_search_space,
            &mut self.source_node,
            &mut self.source_elimination_tree_end,
        );
        self
    }

    pub fn add_target(&mut self, external_t: u32, dist_to_t: u32) -> &mut Self {
        let cch = self.cch;
        internal_add_source(
            external_t,
            dist_to_t,
            &cch.rank,
            &cch.elimination_tree_parent,
            &mut self.backward_tentative_distance,
            &mut self.backward_predecessor_node,
            &mut self.in_backward_search_space,
            &mut self.target_node,
            &mut self.target_elimination_tree_end,
        );
        self
    }

    /// run()
    pub fn run(&mut self) -> &mut Self {
        let cch = self.cch;
        let metric = self.metric;

        for i in (0..self.source_node.len()).rev() {
            let mut x = self.source_node[i];
            let stop = self.source_elimination_tree_end[i];
            while x != stop {
                relax_outgoing_arcs(
                    &cch.up_first_out,
                    &cch.up_head,
                    &metric.forward,
                    &mut self.forward_tentative_distance,
                    &mut self.forward_predecessor_node,
                    x,
                );
                x = cch.elimination_tree_parent[x as usize];
            }
        }

        self.shortest_path_meeting_node = INVALID_ID;
        let mut shortest_path_length = INF_WEIGHT;

        for i in (0..self.target_node.len()).rev() {
            let mut x = self.target_node[i];
            let stop = self.target_elimination_tree_end[i];
            while x != stop {
                relax_outgoing_arcs(
                    &cch.up_first_out,
                    &cch.up_head,
                    &metric.backward,
                    &mut self.backward_tentative_distance,
                    &mut self.backward_predecessor_node,
                    x,
                );
                if self.in_forward_search_space[x as usize] {
                    let l = self.forward_tentative_distance[x as usize]
                        .wrapping_add(self.backward_tentative_distance[x as usize]);
                    if l < shortest_path_length {
                        shortest_path_length = l;
                        self.shortest_path_meeting_node = x;
                    }
                }
                x = cch.elimination_tree_parent[x as usize];
            }
        }
        self
    }

    /// get_distance(): INF_WEIGHT if unreachable
    pub fn get_distance(&self) -> u32 {
        if self.shortest_path_meeting_node == INVALID_ID {
            INF_WEIGHT
        } else {
            let m = self.shortest_path_meeting_node as usize;
            self.forward_tentative_distance[m].wrapping_add(self.backward_tentative_distance[m])
        }
    }

    /// get_node_path(): external node ids from source to target
    pub fn get_node_path(&self) -> Vec<u32> {
        let cch = self.cch;
        let mut path = Vec::new();
        let last = unpack_shortest_path(cch, self.metric, self, &mut |cch_node: u32, _cch_arc: u32, _forward: bool| {
            path.push(cch.order[cch_node as usize]);
        });
        if last != INVALID_ID {
            path.push(cch.order[last as usize]);
        }
        path
    }
}

/// unpack_arc: expands one CCH arc into its input segments (or reports it as a segment)
fn unpack_arc<F: FnMut(u32, u32, bool)>(
    cch: &CustomizableContractionHierarchy,
    metric: &CustomizableContractionHierarchyMetric,
    is_forward: bool,
    x: u32,
    y: u32,
    xy: u32,
    on_new_segment: &mut F,
) {
    let mut bottom_node = INVALID_ID;
    let mut bottom_arc = 0u32;
    let mut mid_arc = 0u32;

    forall_lower_triangles_of_arc_xy(cch, x, y, xy, &mut |bottom_arc_, mid_arc_, top_arc_, bottom_node_, _mid, _top| {
        let (b, m, t) = (bottom_arc_ as usize, mid_arc_ as usize, top_arc_ as usize);
        let fits = if is_forward {
            metric.forward[t] == metric.backward[b].wrapping_add(metric.forward[m])
        } else {
            metric.backward[t] == metric.forward[b].wrapping_add(metric.backward[m])
        };
        if fits {
            bottom_node = bottom_node_;
            bottom_arc = bottom_arc_;
            mid_arc = mid_arc_;
            false
        } else {
            true
        }
    });

    if bottom_node == INVALID_ID {
        if is_forward {
            on_new_segment(x, xy, true);
        } else {
            on_new_segment(y, xy, false);
        }
    } else if is_forward {
        unpack_arc(cch, metric, false, bottom_node, x, bottom_arc, on_new_segment);
        unpack_arc(cch, metric, true, bottom_node, y, mid_arc, on_new_segment);
    } else {
        unpack_arc(cch, metric, false, bottom_node, y, mid_arc, on_new_segment);
        unpack_arc(cch, metric, true, bottom_node, x, bottom_arc, on_new_segment);
    }
}

/// unpack_shortest_path: returns the last node reached on the backward chain (or INVALID_ID)
fn unpack_shortest_path<F: FnMut(u32, u32, bool)>(
    cch: &CustomizableContractionHierarchy,
    metric: &CustomizableContractionHierarchyMetric,
    query: &CustomizableContractionHierarchyQuery,
    on_new_segment: &mut F,
) -> u32 {
    if query.shortest_path_meeting_node == INVALID_ID {
        return INVALID_ID;
    }

    let meeting = query.shortest_path_meeting_node;

    let mut up_path = vec![meeting];
    let mut x = meeting;
    while query.forward_predecessor_node[x as usize] != INVALID_ID {
        x = query.forward_predecessor_node[x as usize];
        up_path.push(x);
    }

    for i in (1..up_path.len()).rev() {
        let arc = find_arc_given_sorted_head(&cch.up_first_out, &cch.up_head, up_path[i], up_path[i - 1]);
        unpack_arc(cch, metric, true, up_path[i], up_path[i - 1], arc, on_new_segment);
    }

    let mut x = meeting;
    let mut y = query.backward_predecessor_node[x as usize];
    while y != INVALID_ID {
        let arc = find_arc_given_sorted_head(&cch.up_first_out, &cch.up_head, y, x);
        unpack_arc(cch, metric, false, y, x, arc, on_new_segment);
        x = y;
        y = query.backward_predecessor_node[y as usize];
    }
    x
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::routingkit::nested_dissection::compute_nested_node_dissection_order_using_inertial_flow;

    /// Deterministic LCG for test weights
    struct Lcg(u64);
    impl Lcg {
        fn next(&mut self) -> u32 {
            self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            (self.0 >> 33) as u32
        }
    }

    /// Directed grid with distinct weights per direction; returns (n, tail, head, weight, lat, lon)
    fn directed_grid(side: u32, seed: u64) -> (u32, Vec<u32>, Vec<u32>, Vec<u32>, Vec<f32>, Vec<f32>) {
        let mut rng = Lcg(seed);
        let (mut tail, mut head, mut weight) = (Vec::new(), Vec::new(), Vec::new());
        let mut lat = Vec::new();
        let mut lon = Vec::new();
        for r in 0..side {
            for c in 0..side {
                lat.push(r as f32);
                lon.push(c as f32);
                let id = r * side + c;
                let mut link = |a: u32, b: u32| {
                    tail.push(a);
                    head.push(b);
                    weight.push(1 + rng.next() % 100);
                    tail.push(b);
                    head.push(a);
                    weight.push(1 + rng.next() % 100);
                };
                if c + 1 < side {
                    link(id, id + 1);
                }
                if r + 1 < side {
                    link(id, id + side);
                }
            }
        }
        (side * side, tail, head, weight, lat, lon)
    }

    /// plain Dijkstra over the arc list
    fn dijkstra(n: u32, tail: &[u32], head: &[u32], weight: &[u32], s: u32) -> Vec<u64> {
        let mut dist = vec![u64::MAX; n as usize];
        dist[s as usize] = 0;
        let mut done = vec![false; n as usize];
        for _ in 0..n {
            let mut best = None;
            for v in 0..n as usize {
                if !done[v] && dist[v] != u64::MAX && best.map_or(true, |b: usize| dist[v] < dist[b]) {
                    best = Some(v);
                }
            }
            let Some(u) = best else { break };
            done[u] = true;
            for a in 0..tail.len() {
                if tail[a] as usize == u {
                    let nd = dist[u] + weight[a] as u64;
                    if nd < dist[head[a] as usize] {
                        dist[head[a] as usize] = nd;
                    }
                }
            }
        }
        dist
    }

    #[test]
    fn cch_queries_match_dijkstra_and_paths_are_valid() {
        let (n, tail, head, weight, lat, lon) = directed_grid(5, 7);
        let order = compute_nested_node_dissection_order_using_inertial_flow(n, &tail, &head, &lat, &lon);
        let cch = CustomizableContractionHierarchy::new(order, tail.clone(), head.clone()).unwrap();
        let metric = {
            let mut m = CustomizableContractionHierarchyMetric::new(&cch, &weight);
            m.customize();
            m
        };
        for s in (0..n).step_by(3) {
            let dist = dijkstra(n, &tail, &head, &weight, s);
            for t in 0..n {
                let mut q = CustomizableContractionHierarchyQuery::new(&metric);
                q.add_source(s, 0).add_target(t, 0).run();
                assert_eq!(q.get_distance() as u64, dist[t as usize], "s={s} t={t}");
                if s != t {
                    let path = q.get_node_path();
                    assert_eq!(*path.first().unwrap(), s);
                    assert_eq!(*path.last().unwrap(), t);
                    let mut len = 0u64;
                    for w in path.windows(2) {
                        let a = (0..tail.len())
                            .find(|&a| tail[a] == w[0] && head[a] == w[1])
                            .expect("path uses a non-existent arc");
                        len += weight[a] as u64;
                    }
                    assert_eq!(len, dist[t as usize], "path length s={s} t={t}");
                }
            }
        }
    }

    #[test]
    fn partial_customization_matches_full_customization() {
        let (n, tail, head, mut weight, lat, lon) = directed_grid(4, 11);
        let order = compute_nested_node_dissection_order_using_inertial_flow(n, &tail, &head, &lat, &lon);
        let cch = CustomizableContractionHierarchy::new(order, tail.clone(), head.clone()).unwrap();
        // fully customized metric for the original weights
        let (old_forward, old_backward) = {
            let mut metric = CustomizableContractionHierarchyMetric::new(&cch, &weight);
            metric.customize();
            (metric.forward.clone(), metric.backward.clone())
        };

        // change a few input weights, then update only those arcs
        let changed = [0u32, 5, 9];
        for &a in &changed {
            weight[a as usize] = 200 + a;
        }
        let mut updated_metric = CustomizableContractionHierarchyMetric::new(&cch, &weight);
        updated_metric.forward = old_forward;
        updated_metric.backward = old_backward;
        let mut partial = CustomizableContractionHierarchyPartialCustomization::new(&cch);
        for &a in &changed {
            partial.update_arc(a);
        }
        partial.customize(&mut updated_metric);

        let mut full = CustomizableContractionHierarchyMetric::new(&cch, &weight);
        full.customize();
        assert_eq!(updated_metric.forward, full.forward);
        assert_eq!(updated_metric.backward, full.backward);
    }
}
