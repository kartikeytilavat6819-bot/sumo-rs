//! Port of foreign/RoutingKit/include/routingkit/nested_dissection.h and src/nested_dissection.cpp.
//! Ported: the inertial-flow path SUMO uses (compute_nested_node_dissection_order_using_inertial_flow)
//! and everything it depends on. Not ported (unused by SUMO): mark_first_n_elements,
//! BlockingFlow::get_source_cut / get_target_cut, and the log_message callbacks
//! (// shortcut: progress logging is dropped, add it when a caller needs the timings).

use super::bit_vector::{make_bit_vector, BitVector};
use super::constants::INVALID_ID;
use super::filter::inplace_keep_element_of_vector_if;
use super::graph_util::*;
use super::id_mapper::LocalIDMapper;
use super::inverse_vector::invert_vector;
use super::permutation::{
    apply_inverse_permutation, identity_permutation, inplace_apply_permutation_to_elements_of,
};
use super::sort::std_nth_element;

#[derive(Clone, Debug, Default)]
pub struct GraphFragment {
    pub global_node_id: Vec<u32>,
    pub first_out: Vec<u32>,
    pub tail: Vec<u32>,
    pub head: Vec<u32>,
    pub back_arc: Vec<u32>,
}

impl GraphFragment {
    pub fn node_count(&self) -> u32 {
        self.global_node_id.len() as u32
    }

    pub fn arc_count(&self) -> u32 {
        self.tail.len() as u32
    }
}

/// make_graph_fragment(node_count, tail, head): undirected fragment with both arc directions
/// (self loops dropped), arcs sorted by tail then head.
pub fn make_graph_fragment(node_count: u32, tail: &[u32], head: &[u32]) -> GraphFragment {
    let arc_count = head.len();
    let mut fragment = GraphFragment::default();

    let non_loop_arc_count = (0..arc_count).filter(|&i| tail[i] != head[i]).count();
    fragment.tail = vec![0; 2 * non_loop_arc_count];
    fragment.head = vec![0; 2 * non_loop_arc_count];
    fragment.back_arc = vec![0; 2 * non_loop_arc_count];

    {
        let mut j = 0usize;
        for i in 0..arc_count {
            if tail[i] != head[i] {
                fragment.tail[j] = tail[i];
                fragment.tail[j + non_loop_arc_count] = head[i];
                fragment.head[j] = head[i];
                fragment.head[j + non_loop_arc_count] = tail[i];
                fragment.back_arc[j] = (j + non_loop_arc_count) as u32;
                fragment.back_arc[j + non_loop_arc_count] = j as u32;
                j += 1;
            }
        }
    }

    {
        let p = compute_inverse_sort_permutation_first_by_tail_then_by_head_and_apply_sort_to_tail(
            node_count,
            &mut fragment.tail,
            &fragment.head,
        );
        fragment.head = apply_inverse_permutation(&p, &fragment.head);
        fragment.back_arc = apply_inverse_permutation(&p, &fragment.back_arc);
        inplace_apply_permutation_to_elements_of(&p, &mut fragment.back_arc);
        fragment.first_out = invert_vector(&fragment.tail, node_count);
    }

    fragment.global_node_id = identity_permutation(node_count);
    fragment
}

#[derive(Clone, Debug, Default)]
pub struct CutSide {
    pub node_on_side_count: u32,
    pub cut_size: u32,
    pub is_node_on_side: BitVector,
}

/// pick_smaller_side(cut)
pub fn pick_smaller_side(cut: &mut CutSide) {
    let node_count = cut.is_node_on_side.size() as u32;
    if cut.node_on_side_count >= (node_count + 1) / 2 {
        cut.node_on_side_count = node_count - cut.node_on_side_count;
        cut.is_node_on_side.inplace_not();
    }
}

/// BlockingFlow: unit-capacity blocking flow between a source and a target set.
pub struct BlockingFlow<'a> {
    fragment: &'a GraphFragment,
    is_source: BitVector,
    is_target: BitVector,
    flow_intensity: u32,
    is_arc_saturated: BitVector,
    is_arc_blocked: BitVector,
    is_finished_flag: bool,
}

impl<'a> BlockingFlow<'a> {
    pub fn new(fragment: &'a GraphFragment, is_source: BitVector, is_target: BitVector) -> Self {
        let arc_count = fragment.arc_count() as u64;
        BlockingFlow {
            fragment,
            is_source,
            is_target,
            flow_intensity: 0,
            is_arc_saturated: BitVector::new(arc_count, false),
            is_arc_blocked: BitVector::uninitialized(arc_count),
            is_finished_flag: false,
        }
    }

    pub fn get_current_flow_intensity(&self) -> u32 {
        self.flow_intensity
    }

    pub fn is_finished(&self) -> bool {
        self.is_finished_flag
    }

    pub fn advance(&mut self) {
        if !self.is_finished_flag
            && compute_blocking_flow(
                self.fragment,
                &self.is_source,
                &self.is_target,
                &self.is_arc_saturated,
                &mut self.is_arc_blocked,
            )
        {
            self.flow_intensity += augment_all_non_blocked_path(
                self.fragment,
                &self.is_source,
                &self.is_target,
                &mut self.is_arc_saturated,
                &mut self.is_arc_blocked,
            );
            self.is_finished_flag = false;
        } else {
            self.is_finished_flag = true;
        }
    }

    /// get_balanced_cut(): the side of the cut reached from the source side or the target side,
    /// whichever keeps the sides balanced.
    pub fn get_balanced_cut(&self) -> CutSide {
        let f = self.fragment;
        let node_count = f.node_count() as usize;
        let arc_count = f.arc_count() as usize;

        let mut is_source_reachable = self.is_source.clone();
        let mut is_target_reachable = self.is_target.clone();
        let mut source_reachable_count = 0u32;
        let mut target_reachable_count = 0u32;

        let mut stack = vec![0u32; node_count];
        let mut stack_end = 0usize;

        // arc_count is no typo, nodes may be multiple times in the vector
        let mut potential_source_piercing_node = vec![0u32; arc_count];
        let mut potential_source_piercing_node_end = 0usize;
        let mut potential_target_piercing_node = vec![0u32; arc_count];
        let mut potential_target_piercing_node_end = 0usize;

        for x in 0..node_count {
            if self.is_source.is_set(x as u64) {
                stack[stack_end] = x as u32;
                stack_end += 1;
            }
        }
        enlarge_source_side(
            f,
            &self.is_arc_saturated,
            &mut is_source_reachable,
            &mut stack,
            &mut stack_end,
            &mut source_reachable_count,
            &mut potential_source_piercing_node,
            &mut potential_source_piercing_node_end,
        );

        for x in 0..node_count {
            if self.is_target.is_set(x as u64) {
                stack[stack_end] = x as u32;
                stack_end += 1;
            }
        }
        enlarge_target_side(
            f,
            &self.is_arc_saturated,
            &mut is_target_reachable,
            &mut stack,
            &mut stack_end,
            &mut target_reachable_count,
            &mut potential_target_piercing_node,
            &mut potential_target_piercing_node_end,
        );

        loop {
            if source_reachable_count <= target_reachable_count {
                let mut pierce_node = INVALID_ID;
                while pierce_node == INVALID_ID {
                    if potential_source_piercing_node_end == 0 {
                        return CutSide {
                            node_on_side_count: source_reachable_count,
                            cut_size: self.flow_intensity,
                            is_node_on_side: is_source_reachable,
                        };
                    }
                    potential_source_piercing_node_end -= 1;
                    let y = potential_source_piercing_node[potential_source_piercing_node_end];
                    if !is_source_reachable.is_set(y as u64) && !is_target_reachable.is_set(y as u64) {
                        pierce_node = y;
                    }
                }
                is_source_reachable.set(pierce_node as u64);
                stack[stack_end] = pierce_node;
                stack_end += 1;
                enlarge_source_side(
                    f,
                    &self.is_arc_saturated,
                    &mut is_source_reachable,
                    &mut stack,
                    &mut stack_end,
                    &mut source_reachable_count,
                    &mut potential_source_piercing_node,
                    &mut potential_source_piercing_node_end,
                );
            } else {
                let mut pierce_node = INVALID_ID;
                while pierce_node == INVALID_ID {
                    if potential_target_piercing_node_end == 0 {
                        return CutSide {
                            node_on_side_count: target_reachable_count,
                            cut_size: self.flow_intensity,
                            is_node_on_side: is_target_reachable,
                        };
                    }
                    potential_target_piercing_node_end -= 1;
                    let y = potential_target_piercing_node[potential_target_piercing_node_end];
                    if !is_source_reachable.is_set(y as u64) && !is_target_reachable.is_set(y as u64) {
                        pierce_node = y;
                    }
                }
                is_target_reachable.set(pierce_node as u64);
                stack[stack_end] = pierce_node;
                stack_end += 1;
                enlarge_target_side(
                    f,
                    &self.is_arc_saturated,
                    &mut is_target_reachable,
                    &mut stack,
                    &mut stack_end,
                    &mut target_reachable_count,
                    &mut potential_target_piercing_node,
                    &mut potential_target_piercing_node_end,
                );
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn enlarge_source_side(
    f: &GraphFragment,
    is_arc_saturated: &BitVector,
    is_source_reachable: &mut BitVector,
    stack: &mut [u32],
    stack_end: &mut usize,
    source_reachable_count: &mut u32,
    potential_source_piercing_node: &mut [u32],
    potential_source_piercing_node_end: &mut usize,
) {
    while *stack_end != 0 {
        *source_reachable_count += 1;
        *stack_end -= 1;
        let x = stack[*stack_end] as usize;
        for xy in f.first_out[x]..f.first_out[x + 1] {
            let y = f.head[xy as usize];
            if is_arc_saturated.is_set(xy as u64) {
                potential_source_piercing_node[*potential_source_piercing_node_end] = y;
                *potential_source_piercing_node_end += 1;
            } else if !is_source_reachable.is_set(y as u64) {
                is_source_reachable.set(y as u64);
                stack[*stack_end] = y;
                *stack_end += 1;
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn enlarge_target_side(
    f: &GraphFragment,
    is_arc_saturated: &BitVector,
    is_target_reachable: &mut BitVector,
    stack: &mut [u32],
    stack_end: &mut usize,
    target_reachable_count: &mut u32,
    potential_target_piercing_node: &mut [u32],
    potential_target_piercing_node_end: &mut usize,
) {
    while *stack_end != 0 {
        *target_reachable_count += 1;
        *stack_end -= 1;
        let x = stack[*stack_end] as usize;
        for xy in f.first_out[x]..f.first_out[x + 1] {
            let y = f.head[xy as usize];
            if is_arc_saturated.is_set(f.back_arc[xy as usize] as u64) {
                potential_target_piercing_node[*potential_target_piercing_node_end] = y;
                *potential_target_piercing_node_end += 1;
            } else if !is_target_reachable.is_set(y as u64) {
                stack[*stack_end] = y;
                *stack_end += 1;
                is_target_reachable.set(y as u64);
            }
        }
    }
}

fn compute_blocking_flow(
    fragment: &GraphFragment,
    is_source: &BitVector,
    is_target: &BitVector,
    is_arc_saturated: &BitVector,
    is_arc_blocked: &mut BitVector,
) -> bool {
    let node_count = fragment.node_count() as usize;
    let mut is_on_same_level_or_lower = BitVector::new(node_count as u64, false);
    let mut was_node_pushed = BitVector::new(node_count as u64, false);

    is_arc_blocked.reset_all();

    let mut queue = vec![0u32; node_count];
    let mut queue_begin = 0usize;
    let mut queue_end = 0usize;

    for x in 0..node_count {
        if is_source.is_set(x as u64) {
            queue[queue_end] = x as u32;
            queue_end += 1;
        }
    }
    let mut queue_current_level_end = queue_end;

    let mut is_a_target_node_reachable = false;

    while queue_begin != queue_end {
        for i in queue_begin..queue_current_level_end {
            is_on_same_level_or_lower.set(queue[i] as u64);
        }

        for i in queue_begin..queue_current_level_end {
            let x = queue[i] as usize;
            for xy in fragment.first_out[x]..fragment.first_out[x + 1] {
                if is_arc_saturated.is_set(xy as u64) {
                    is_arc_blocked.set(xy as u64);
                } else {
                    let y = fragment.head[xy as usize];
                    if is_on_same_level_or_lower.is_set(y as u64) {
                        is_arc_blocked.set(xy as u64);
                    } else if is_target.is_set(y as u64) {
                        is_a_target_node_reachable = true;
                    } else if !was_node_pushed.is_set(y as u64) {
                        queue[queue_end] = y;
                        queue_end += 1;
                        was_node_pushed.set(y as u64);
                    }
                }
            }
        }

        queue_begin = queue_current_level_end;
        queue_current_level_end = queue_end;
    }
    is_a_target_node_reachable
}

fn find_first_non_block_outgoing_arc_of_node(fragment: &GraphFragment, is_arc_blocked: &BitVector, x: usize) -> u32 {
    for xy in fragment.first_out[x]..fragment.first_out[x + 1] {
        if !is_arc_blocked.is_set(xy as u64) {
            return xy;
        }
    }
    INVALID_ID
}

fn augment_all_non_blocked_path(
    fragment: &GraphFragment,
    is_source: &BitVector,
    is_target: &BitVector,
    is_arc_saturated: &mut BitVector,
    is_arc_blocked: &mut BitVector,
) -> u32 {
    let node_count = fragment.node_count() as usize;
    let mut current_path_node = vec![0u32; node_count];
    let mut current_path_arc = vec![0u32; node_count];

    let mut augmented_path_count = 0u32;

    for s in 0..node_count {
        if is_source.is_set(s as u64) {
            current_path_node[0] = s as u32;
            current_path_arc[0] = s as u32;
            let mut current_path_arc_count = 0usize;
            loop {
                let x = current_path_node[current_path_arc_count] as usize;
                let xy = find_first_non_block_outgoing_arc_of_node(fragment, is_arc_blocked, x);
                if xy == INVALID_ID {
                    if current_path_arc_count == 0 {
                        break;
                    }
                    current_path_arc_count -= 1;
                    is_arc_blocked.set(current_path_arc[current_path_arc_count] as u64);
                } else {
                    let y = fragment.head[xy as usize];
                    current_path_arc[current_path_arc_count] = xy;
                    current_path_arc_count += 1;
                    current_path_node[current_path_arc_count] = y;
                    if is_target.is_set(y as u64) {
                        for i in 0..current_path_arc_count {
                            let a = current_path_arc[i];
                            debug_assert!(!is_arc_saturated.is_set(a as u64));
                            is_arc_blocked.set(a as u64);

                            let b = fragment.back_arc[a as usize];
                            if is_arc_saturated.is_set(b as u64) {
                                is_arc_saturated.reset(b as u64);
                            } else {
                                is_arc_saturated.set(a as u64);
                            }
                        }
                        current_path_arc_count = 0;
                        augmented_path_count += 1;
                    }
                }
            }
        }
    }
    augmented_path_count
}

/// Source and target sets: the n nodes with the smallest keys and the n with the largest.
fn select_source_and_target<K: Fn(u32) -> f32>(n: u32, node_count: u32, sort_key: &K) -> (BitVector, BitVector) {
    let mut v = identity_permutation(node_count);
    let less = |l: u32, r: u32| sort_key(l) < sort_key(r);
    let source_end = n as usize;
    let target_begin = (node_count - n) as usize;
    std_nth_element(&mut v, 0, source_end, node_count as usize, &less);
    std_nth_element(&mut v, source_end, target_begin, node_count as usize, &less);

    let mut is_source = BitVector::uninitialized(node_count as u64);
    for &x in &v[..source_end] {
        is_source.set(x as u64);
    }
    let mut is_target = BitVector::uninitialized(node_count as u64);
    for &x in &v[target_begin..] {
        is_target.set(x as u64);
    }
    (is_source, is_target)
}

/// inertial_flow(fragment, min_balance, latitude, longitude)
pub fn inertial_flow_with_balance(g: &GraphFragment, min_balance: u32, latitude: &[f32], longitude: &[f32]) -> CutSide {
    let node_count = g.node_count();

    let mut side_size = node_count.wrapping_mul(min_balance) / 100;
    if side_size == 0 {
        side_size = 1;
    }

    let lat = |x: u32| latitude[g.global_node_id[x as usize] as usize];
    let lon = |x: u32| longitude[g.global_node_id[x as usize] as usize];

    let (hs, ht) = select_source_and_target(side_size, node_count, &lat);
    let (vs, vt) = select_source_and_target(side_size, node_count, &lon);
    let (ds, dt) = select_source_and_target(side_size, node_count, &|x: u32| lat(x) + lon(x));
    let (ns, nt) = select_source_and_target(side_size, node_count, &|x: u32| lat(x) - lon(x));

    let mut cutters = [
        BlockingFlow::new(g, hs, ht),
        BlockingFlow::new(g, vs, vt),
        BlockingFlow::new(g, ds, dt),
        BlockingFlow::new(g, ns, nt),
    ];

    loop {
        let i = get_next_cutter(&cutters);
        let c = &mut cutters[i];
        if !c.is_finished() {
            c.advance();
        } else {
            return c.get_balanced_cut();
        }
    }
}

fn get_next_cutter(cutters: &[BlockingFlow]) -> usize {
    let h = cutters[0].get_current_flow_intensity();
    let v = cutters[1].get_current_flow_intensity();
    let md = cutters[2].get_current_flow_intensity();
    let nd = cutters[3].get_current_flow_intensity();
    if h <= v && h <= md && h <= nd {
        0
    } else if v <= md && v <= nd {
        1
    } else if md <= nd {
        2
    } else {
        3
    }
}

/// inertial_flow(fragment, latitude, longitude): the best of balances 25%, 33% and 40%
pub fn inertial_flow(g: &GraphFragment, latitude: &[f32], longitude: &[f32]) -> CutSide {
    let c25 = inertial_flow_with_balance(g, 25, latitude, longitude);
    let c33 = inertial_flow_with_balance(g, 33, latitude, longitude);
    let c40 = inertial_flow_with_balance(g, 40, latitude, longitude);

    let cut25 = c25.cut_size as u64;
    let cut33 = c33.cut_size as u64;
    let cut40 = c40.cut_size as u64;
    let n25 = c25.node_on_side_count as u64;
    let n33 = c33.node_on_side_count as u64;
    let n40 = c40.node_on_side_count as u64;

    if cut25 * n33 < cut33 * n25 && cut25 * n40 < cut40 * n25 {
        c25
    } else if cut33 * n40 < cut40 * n33 {
        c33
    } else {
        c40
    }
}

/// derive_separator_from_cut(fragment, cut)
pub fn derive_separator_from_cut(fragment: &GraphFragment, cut: &BitVector) -> BitVector {
    let small_side = cut.population_count() <= (fragment.node_count() / 2) as u64;
    let mut is_separator_node = BitVector::new(fragment.node_count() as u64, false);

    for xy in 0..fragment.arc_count() as usize {
        let x = fragment.tail[xy];
        let y = fragment.head[xy];
        if cut.is_set(x as u64) == small_side && cut.is_set(y as u64) != small_side {
            is_separator_node.set(y as u64);
        }
    }
    is_separator_node
}

/// decompose_graph_fragment_into_connected_components(fragment)
pub fn decompose_graph_fragment_into_connected_components(mut fragment: GraphFragment) -> Vec<GraphFragment> {
    let node_count = fragment.node_count() as usize;
    let arc_count = fragment.arc_count() as usize;

    let mut component_count = 0u32;
    let mut component = vec![INVALID_ID; node_count];
    {
        let mut inv_pseudo_preorder = vec![0u32; node_count];
        let mut stack = vec![0u32; node_count];
        let mut pos = 0u32;
        for r in 0..node_count {
            if component[r] == INVALID_ID {
                component[r] = component_count;
                let mut stack_end = 1usize;
                stack[0] = r as u32;
                while stack_end != 0 {
                    stack_end -= 1;
                    let x = stack[stack_end] as usize;
                    inv_pseudo_preorder[x] = pos;
                    pos += 1;
                    for xy in fragment.first_out[x]..fragment.first_out[x + 1] {
                        let y = fragment.head[xy as usize];
                        if component[y as usize] == INVALID_ID {
                            stack[stack_end] = y;
                            stack_end += 1;
                            component[y as usize] = component_count;
                        }
                    }
                }
                component_count += 1;
            }
        }

        inplace_apply_permutation_to_elements_of(&inv_pseudo_preorder, &mut fragment.tail);
        inplace_apply_permutation_to_elements_of(&inv_pseudo_preorder, &mut fragment.head);
        fragment.global_node_id = apply_inverse_permutation(&inv_pseudo_preorder, &fragment.global_node_id);
        component = apply_inverse_permutation(&inv_pseudo_preorder, &component);
    }

    {
        let p = compute_inverse_sort_permutation_first_by_tail_then_by_head_and_apply_sort_to_tail(
            node_count as u32,
            &mut fragment.tail,
            &fragment.head,
        );
        fragment.head = apply_inverse_permutation(&p, &fragment.head);
        fragment.back_arc = apply_inverse_permutation(&p, &fragment.back_arc);
        inplace_apply_permutation_to_elements_of(&p, &mut fragment.back_arc);
        fragment.first_out = invert_vector(&fragment.tail, node_count as u32);
    }

    let mut part_list = Vec::new();

    let mut component_node_begin = 0usize;
    let mut component_arc_begin = 0usize;
    let mut component_node_end = 1usize;
    let mut component_arc_end = 0usize;

    let mut current_component = 0u32;
    while component_node_end < node_count {
        if component[component_node_end] != current_component {
            while component_arc_end != arc_count
                && component[fragment.tail[component_arc_end] as usize] == current_component
            {
                component_arc_end += 1;
            }
            part_list.push(generate_part(
                &fragment,
                component_node_begin,
                component_node_end,
                component_arc_begin,
                component_arc_end,
            ));
            component_node_begin = component_node_end;
            component_arc_begin = component_arc_end;
            current_component = component[component_node_end];
        }
        component_node_end += 1;
    }
    part_list.push(generate_part(
        &fragment,
        component_node_begin,
        component_node_end,
        component_arc_begin,
        arc_count,
    ));

    part_list
}

fn generate_part(
    fragment: &GraphFragment,
    component_node_begin: usize,
    component_node_end: usize,
    component_arc_begin: usize,
    component_arc_end: usize,
) -> GraphFragment {
    let part_node_count = (component_node_end - component_node_begin) as u32;
    let nb = component_node_begin as u32;
    let ab = component_arc_begin as u32;
    let tail: Vec<u32> = fragment.tail[component_arc_begin..component_arc_end].iter().map(|&x| x.wrapping_sub(nb)).collect();
    GraphFragment {
        first_out: invert_vector(&tail, part_node_count),
        tail,
        head: fragment.head[component_arc_begin..component_arc_end].iter().map(|&x| x.wrapping_sub(nb)).collect(),
        back_arc: fragment.back_arc[component_arc_begin..component_arc_end].iter().map(|&x| x.wrapping_sub(ab)).collect(),
        global_node_id: fragment.global_node_id[component_node_begin..component_node_end].to_vec(),
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SeparatorTreeNode {
    pub left_child: u32,
    pub right_sibling: u32,
    pub first_separator_vertex: u32,
    pub last_separator_vertex: u32,
}

#[derive(Clone, Debug, Default)]
pub struct SeparatorDecomposition {
    pub tree: Vec<SeparatorTreeNode>,
    pub order: Vec<u32>,
}

/// compute_separator_decomposition(fragment, compute_separator)
pub fn compute_separator_decomposition<F: Fn(&GraphFragment) -> BitVector>(
    fragment: GraphFragment,
    compute_separator: &F,
) -> SeparatorDecomposition {
    let node_count = fragment.node_count();
    let mut decomp = SeparatorDecomposition {
        tree: Vec::new(),
        order: vec![0; node_count as usize],
    };

    if node_count == 1 {
        decomp.tree.push(SeparatorTreeNode {
            left_child: 0,
            right_sibling: 0,
            first_separator_vertex: 0,
            last_separator_vertex: 1,
        });
        decomp.order = fragment.global_node_id;
    } else {
        let mut pred = 0usize;
        let mut order_begin = 0u32;
        let mut order_end = node_count;

        decomp.tree.push(SeparatorTreeNode {
            left_child: 0,
            right_sibling: 0,
            first_separator_vertex: 0,
            last_separator_vertex: order_end,
        });

        let part_list = decompose_graph_fragment_into_connected_components(fragment);

        for mut part in part_list {
            if part.node_count() == 1 {
                order_end -= 1;
                decomp.order[order_end as usize] = part.global_node_id[0];
            } else {
                let is_separator_node = compute_separator(&part);

                let f = make_bit_vector(part.arc_count() as u64, |a| {
                    !is_separator_node.is_set(part.tail[a as usize] as u64)
                        && !is_separator_node.is_set(part.head[a as usize] as u64)
                });

                inplace_keep_element_of_vector_if(&f, &mut part.tail);
                inplace_keep_element_of_vector_if(&f, &mut part.head);
                inplace_keep_element_of_vector_if(&f, &mut part.back_arc);

                {
                    let map = LocalIDMapper::new(&f);
                    for x in part.back_arc.iter_mut() {
                        *x = map.to_local(*x as u64) as u32;
                    }
                }

                part.first_out = invert_vector(&part.tail, part.node_count());

                let sub_decomp = compute_separator_decomposition(part, compute_separator);

                let offset = decomp.tree.len() as u32;
                let mut sub_tree = sub_decomp.tree;
                for node in sub_tree.iter_mut() {
                    if node.left_child != 0 {
                        node.left_child += offset;
                    }
                    if node.right_sibling != 0 {
                        node.right_sibling += offset;
                    }
                    node.first_separator_vertex += order_begin;
                    node.last_separator_vertex += order_begin;
                }
                if pred == 0 {
                    decomp.tree[pred].left_child = offset;
                } else {
                    decomp.tree[pred].right_sibling = offset;
                }
                pred = offset as usize;
                decomp.tree.extend(sub_tree);
                let sub_len = sub_decomp.order.len();
                decomp.order[order_begin as usize..order_begin as usize + sub_len].copy_from_slice(&sub_decomp.order);
                order_begin += sub_len as u32;
            }
        }
        decomp.tree[0].first_separator_vertex = order_begin;
    }

    decomp
}

/// compute_nested_node_dissection_order_using_inertial_flow(node_count, tail, head, latitude, longitude)
pub fn compute_nested_node_dissection_order_using_inertial_flow(
    node_count: u32,
    tail: &[u32],
    head: &[u32],
    latitude: &[f32],
    longitude: &[f32],
) -> Vec<u32> {
    let g = make_graph_fragment(node_count, tail, head);

    let compute_separator = |fragment: &GraphFragment| -> BitVector {
        let mut c = inertial_flow(fragment, latitude, longitude);
        pick_smaller_side(&mut c);
        let cut = c.is_node_on_side;
        derive_separator_from_cut(fragment, &cut)
    };

    compute_separator_decomposition(g, &compute_separator).order
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 4x4 grid, node id = row*4 + col, both directions as separate arcs
    fn grid(side: u32) -> (u32, Vec<u32>, Vec<u32>, Vec<f32>, Vec<f32>) {
        let mut tail = Vec::new();
        let mut head = Vec::new();
        let mut lat = Vec::new();
        let mut lon = Vec::new();
        for r in 0..side {
            for c in 0..side {
                lat.push(r as f32);
                lon.push(c as f32);
                let id = r * side + c;
                if c + 1 < side {
                    tail.push(id);
                    head.push(id + 1);
                }
                if r + 1 < side {
                    tail.push(id);
                    head.push(id + side);
                }
            }
        }
        (side * side, tail, head, lat, lon)
    }

    #[test]
    fn nested_dissection_order_is_a_permutation() {
        let (n, tail, head, lat, lon) = grid(6);
        let order = compute_nested_node_dissection_order_using_inertial_flow(n, &tail, &head, &lat, &lon);
        let mut sorted = order.clone();
        sorted.sort();
        assert_eq!(sorted, (0..n).collect::<Vec<u32>>());
    }

    #[test]
    fn make_graph_fragment_drops_loops_and_pairs_back_arcs() {
        let f = make_graph_fragment(3, &[0, 1, 2], &[1, 1, 2]);
        assert_eq!(f.arc_count(), 2);
        assert_eq!(f.first_out, vec![0, 1, 2, 2]);
        for a in 0..f.arc_count() as usize {
            let b = f.back_arc[a] as usize;
            assert_eq!(f.back_arc[b] as usize, a);
            assert_eq!(f.tail[a], f.head[b]);
        }
    }
}
