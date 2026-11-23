//! Port of utils/traction_wire/Circuit.{h,cpp}
//!
//! based on console-based C++ DC circuits simulator, https://github.com/rka97/Circuits-Solver by
//! Ahmad Khaled, Ahmad Essam, Omnia Zakaria, Mary Nader (MIT license).
//!
//! Representation of electric circuit of overhead wires.
//!
//! Differences from the C++ class:
//! - Nodes and elements live in arenas owned by the circuit and are referenced by index
//!   (`NodeIdx`, `ElemIdx`) instead of raw pointers. A removed node stays in the arena (so the
//!   indices of the others stay valid) but is no longer listed by the circuit.
//! - Eigen is replaced by `col_piv_householder_qr` (same algorithm, see that file). The Eigen
//!   path is the only solver in C++, so it is always compiled here.
//! - `lock()` / `unlock()` are not ported: `&mut self` already gives exclusive access.

use std::sync::atomic::{AtomicBool, Ordering};

use crate::common::msg_handler::{write_error, write_warning};
use crate::common::to_string::f64_to_string_prec;
use crate::{write_errorf, write_warningf};

use super::col_piv_householder_qr::{DenseMatrix, col_piv_householder_qr_solve, eigen_norm};
use super::element::{ElemIdx, Element, ElementType};
use super::node::{Node, NodeIdx};

/// whether per-substation overhead-wire current limits should be enforced
static MY_CURRENT_LIMITS: AtomicBool = AtomicBool::new(false);

/// Flag of alpha scaling parameter.
///
/// NotApplied => alpha should be 1
/// CurrentLimits => alpha is lower than one due to electric current limits of the substation
/// VoltageLimits => alpha is not one due to inability of network to transfer requested power due to overhead wire resistance
/// NotConverging => number of allowed iterations exceeded
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlphaFlag {
    /// The scaling alpha is not applied (is one)
    NotApplied = 0,
    /// The scaling alpha is applied (is not one) due to current limits
    CurrentLimits,
    /// The scaling alpha is applied (is not one) due to voltage limits
    VoltageLimits,
    /// The Newton-Rhapson method has reached maximum iterations and no solution of circuit has been found with actual value of alpha
    NotConverging,
}

/// All interactions will be through this class, the user will know nothing about the other classes,
/// and will interact only through the names of the elements/nodes.
#[derive(Debug, Clone)]
pub struct Circuit {
    node_arena: Vec<Node>,
    // boxed so that an element keeps its address while the arena grows (C++ Element* stays valid)
    element_arena: Vec<Box<Element>>,
    // nodes, elements and voltage sources that are part of the circuit, in insertion order
    nodes: Vec<NodeIdx>,
    elements: Vec<ElemIdx>,
    voltage_sources: Vec<ElemIdx>,

    last_id: i32,
    is_cleaned: bool,

    // The electric current limit of the voltage sources.
    circuit_current_limit: f64,

    // Best alpha scaling value.
    //
    // This parameter is used to scale down the power demands of current sources (vehicles
    // that draw power from the circuit) so that a solution of the system can be found.
    // Note: the system is nonlinear (quadratic), hence in some cases (typically too high
    // power demands) a solution cannot be found. In that moment we decrease all power
    // requirements by `alpha` and try to solve again, until we find alpha that ensures
    // stable solution. This is then reported as alphaBest.
    alpha_best: f64,

    alpha_reason: AlphaFlag,
}

impl Default for Circuit {
    fn default() -> Self {
        Self::new()
    }
}

impl Circuit {
    /// Enables or disables the per-substation overhead-wire current limits (static, all circuits).
    pub fn enforce_current_limits(val: bool) {
        MY_CURRENT_LIMITS.store(val, Ordering::Relaxed);
    }

    // a Constructor, same functionality as "init" functions
    pub fn new() -> Self {
        Self::with_current_limit(f64::INFINITY)
    }

    // Constructor with user-specified current limit parameter.
    pub fn with_current_limit(current_limit: f64) -> Self {
        Self {
            node_arena: Vec::new(),
            element_arena: Vec::new(),
            nodes: Vec::new(),
            elements: Vec::new(),
            voltage_sources: Vec::new(),
            last_id: 0,
            is_cleaned: true,
            circuit_current_limit: current_limit,
            alpha_best: 0.0,
            alpha_reason: AlphaFlag::NotApplied,
        }
    }

    // accessors for the arena entries behind the indices

    pub fn node(&self, idx: NodeIdx) -> &Node {
        &self.node_arena[idx]
    }

    pub fn node_mut(&mut self, idx: NodeIdx) -> &mut Node {
        &mut self.node_arena[idx]
    }

    pub fn element(&self, idx: ElemIdx) -> &Element {
        &self.element_arena[idx]
    }

    pub fn element_mut(&mut self, idx: ElemIdx) -> &mut Element {
        &mut self.element_arena[idx]
    }

    /// C++ `Element::getVoltage()` for the element at `idx`
    pub fn element_voltage(&self, idx: ElemIdx) -> f64 {
        self.element(idx).get_voltage(&self.node_arena)
    }

    /// C++ `Element::getCurrent()` for the element at `idx`
    pub fn element_current(&self, idx: ElemIdx) -> f64 {
        self.element(idx).get_current(&self.node_arena)
    }

    pub fn get_node(&self, name: &str) -> Option<NodeIdx> {
        self.nodes.iter().copied().find(|&n| self.node(n).name() == name)
    }

    pub fn get_node_by_id(&self, id: i32) -> Option<NodeIdx> {
        self.nodes.iter().copied().find(|&n| self.node(n).id() == id)
    }

    pub fn get_element(&self, name: &str) -> Option<ElemIdx> {
        self.elements
            .iter()
            .copied()
            .find(|&e| self.element(e).name() == name)
            .or_else(|| self.voltage_sources.iter().copied().find(|&e| self.element(e).name() == name))
    }

    fn get_element_by_id(&self, id: i32) -> Option<ElemIdx> {
        self.elements
            .iter()
            .copied()
            .find(|&e| self.element(e).id() == id)
            .or_else(|| self.get_voltage_source(id))
    }

    pub fn get_voltage_source(&self, id: i32) -> Option<ElemIdx> {
        self.voltage_sources.iter().copied().find(|&e| self.element(e).id() == id)
    }

    pub fn get_current(&self, name: &str) -> f64 {
        match self.get_element(name) {
            None => f64::MAX,
            Some(e) => self.element(e).get_current(&self.node_arena),
        }
    }

    // gets voltage across element or node "name"
    pub fn get_voltage(&self, name: &str) -> f64 {
        match self.get_element(name) {
            None => match self.get_node(name) {
                Some(n) => self.node(n).voltage(),
                None => f64::MAX,
            },
            Some(e) => self.element(e).get_voltage(&self.node_arena),
        }
    }

    // gets the resistance of an element.
    pub fn get_resistance(&self, name: &str) -> f64 {
        match self.get_element(name) {
            None => -1.0,
            Some(e) => self.element(e).resistance(),
        }
    }

    // gets the number of voltage sources in the circuit.
    pub fn get_num_voltage_sources(&self) -> usize {
        self.voltage_sources.len()
    }

    // The sum of voltage source powers in the circuit
    pub fn get_total_power_of_circuit_sources(&self) -> f64 {
        let mut power = 0.0;
        for &vs in &self.voltage_sources {
            power += self.element(vs).get_power(&self.node_arena);
        }
        power
    }

    // The sum of voltage source currents in the circuit
    pub fn get_total_current_of_circuit_sources(&self) -> f64 {
        let mut current = 0.0;
        for &vs in &self.voltage_sources {
            current += self.element(vs).get_current(&self.node_arena);
        }
        current
    }

    // List of currents of voltage sources as a string, separated by spaces
    pub fn get_currents_of_circuit_source(&self) -> String {
        let mut currents = String::new();
        for &vs in &self.voltage_sources {
            currents += &f64_to_string_prec(self.element(vs).get_current(&self.node_arena), 4);
            currents += " ";
        }
        if !currents.is_empty() {
            currents.pop();
        }
        currents
    }

    pub fn get_current_sources(&self) -> Vec<ElemIdx> {
        self.elements
            .iter()
            .copied()
            .filter(|&el| self.element(el).element_type() == ElementType::CurrentSource)
            .collect()
    }

    /// return alphaBest variable, the best alpha scaling value
    pub fn get_alpha_best(&self) -> f64 {
        self.alpha_best
    }

    /// return the reason why `alpha` scaling value has been used
    pub fn get_alpha_reason(&self) -> AlphaFlag {
        self.alpha_reason
    }

    // returns lastId
    pub fn get_last_id(&self) -> i32 {
        self.last_id
    }

    // decreases lastId by one
    pub fn decrease_last_id(&mut self) {
        self.last_id -= 1;
    }

    /// Set the electric current limit of this circuit.
    pub fn set_current_limit(&mut self, my_current_limit: f64) {
        self.circuit_current_limit = my_current_limit;
    }

    /// Get the electric current limit of this circuit.
    pub fn get_current_limit(&self) -> f64 {
        self.circuit_current_limit
    }

    // voltageSources->front()->getVoltage()
    fn front_voltage_source_voltage(&self) -> f64 {
        let vs = *self.voltage_sources.first().expect("circuit has no voltage source");
        self.element(vs).get_voltage(&self.node_arena)
    }

    // adds a node with name "name"
    pub fn add_node(&mut self, name: &str) -> Option<NodeIdx> {
        if self.get_node(name).is_some() {
            write_errorf!("The node: '%' already exists.", name);
            return None;
        }

        if self.nodes.is_empty() {
            self.last_id = -1;
        }
        let mut t_node = Node::new(name, self.last_id);
        if self.last_id == -1 {
            t_node.set_ground(true);
        }
        self.last_id += 1;
        self.node_arena.push(t_node);
        let idx = self.node_arena.len() - 1;
        self.nodes.push(idx);
        Some(idx)
    }

    // erases a node with name "name"
    pub fn erase_node(&mut self, node: NodeIdx) {
        self.nodes.retain(|&n| n != node);
    }

    // adds an element with name "name", type "type" and value "value" to positive node "pNode" and negative node "nNode"
    pub fn add_element(
        &mut self,
        name: &str,
        mut value: f64,
        p_node: NodeIdx,
        n_node: NodeIdx,
        et: ElementType,
    ) -> Option<ElemIdx> {
        // RICE_CHECK: This seems to be a bit of work in progress, is it final?
        if et == ElementType::Resistor && value <= 1e-6 {
            // due to numeric problems
            if value > -1e-6 {
                value = 1e-6;
                write_warning("Trying to add resistor element into the overhead wire circuit with resistance < 1e-6. ");
            } else {
                write_error("Trying to add resistor element into the overhead wire circuit with resistance < 0. ");
                return None;
            }
        }

        if self.get_element(name).is_some() {
            print!("The element: '{name}' already exists.");
            return None;
        }

        let mut e = Element::new(name, et, value);
        let idx = self.element_arena.len();
        if e.element_type() == ElementType::VoltageSource {
            e.set_id(self.last_id);
            self.last_id += 1;
            self.voltage_sources.push(idx);
        } else {
            self.elements.push(idx);
        }
        e.set_pos_node(p_node);
        e.set_neg_node(n_node);
        self.element_arena.push(Box::new(e));

        self.node_mut(p_node).add_element(idx);
        self.node_mut(n_node).add_element(idx);
        Some(idx)
    }

    pub fn erase_element(&mut self, element: ElemIdx) {
        let p = self.element(element).p_node().expect("element has no positive node");
        let n = self.element(element).n_node().expect("element has no negative node");
        self.node_mut(p).erase_element(element);
        self.node_mut(n).erase_element(element);
        self.elements.retain(|&e| e != element);
    }

    // replaces unusedNode with newNode everywhere in the circuit, modifies the ids of other nodes and elements, decreases the id by one and deletes unusedNode
    pub fn replace_and_delete_node(&mut self, unused_node: NodeIdx, new_node: NodeIdx) {
        // replace element node if it is unusedNode
        for k in 0..self.voltage_sources.len() {
            let vs = self.voltage_sources[k];
            self.redirect_terminals(vs, unused_node, new_node);
        }
        for k in 0..self.elements.len() {
            let el = self.elements[k];
            self.redirect_terminals(el, unused_node, new_node);
        }

        // erase unusedNode from nodes vector
        self.erase_node(unused_node);

        // modify id of other elements and nodes
        let mod_last_id = self.get_last_id() - 1;
        if self.node(unused_node).id() != mod_last_id {
            if let Some(node_last) = self.get_node_by_id(mod_last_id) {
                let unused_id = self.node(unused_node).id();
                self.node_mut(node_last).set_id(unused_id);
            } else if let Some(elem_last) = self.get_voltage_source(mod_last_id) {
                let unused_id = self.node(unused_node).id();
                self.element_mut(elem_last).set_id(unused_id);
            } else {
                write_error("The element or node with the last Id was not found in the circuit!");
            }
        }

        self.decrease_last_id();
        // the C++ `delete unusedNode` is the arena slot staying unlisted
    }

    // moves the terminals of `el` that sit on `old` over to `new`, keeping the node's element lists in step
    fn redirect_terminals(&mut self, el: ElemIdx, old: NodeIdx, new: NodeIdx) {
        if self.element(el).n_node() == Some(old) {
            self.element_mut(el).set_neg_node(new);
            self.node_mut(new).erase_element(el);
            self.node_mut(new).add_element(el);
        }
        if self.element(el).p_node() == Some(old) {
            self.element_mut(el).set_pos_node(new);
            self.node_mut(new).erase_element(el);
            self.node_mut(new).add_element(el);
        }
    }

    pub fn clean_up_sp(&mut self) {
        for k in 0..self.elements.len() {
            let el = self.elements[k];
            if self.element(el).element_type() != ElementType::Resistor {
                self.element_mut(el).set_enabled(true);
            }
        }
        for k in 0..self.voltage_sources.len() {
            let vs = self.voltage_sources[k];
            self.element_mut(vs).set_enabled(true);
        }
        self.is_cleaned = true;
    }

    // checks if the circuit's connections are correct.
    pub fn check_circuit(&self, substation_id: &str) -> bool {
        // check empty nodes
        for &n in &self.nodes {
            if self.node(n).num_of_elements() < 1 {
                return false;
            }
        }
        // check voltage sources
        for &vs in &self.voltage_sources {
            let el = self.element(vs);
            if el.p_node().is_none() || el.n_node().is_none() {
                write_errorf!(
                    "Circuit Voltage Source '%' is connected to less than two nodes, please adjust the definition of the section (with substation '%').",
                    el.name(),
                    substation_id
                );
                return false;
            }
        }
        // check other elements
        for &el_idx in &self.elements {
            let el = self.element(el_idx);
            if el.p_node().is_none() || el.n_node().is_none() {
                write_errorf!(
                    "Circuit Element '%' is connected to less than two nodes, please adjust the definition of the section (with substation '%').",
                    el.name(),
                    substation_id
                );
                return false;
            }
        }

        // check connectivity
        let num = self.nodes.len() + self.get_num_voltage_sources() - 1;
        let mut nodes_visited = vec![false; num];
        let node_minus_one = self.get_node_by_id(-1).expect("circuit has no node with id -1");
        if !self.node(node_minus_one).is_ground() {
            write_errorf!(
                "Circuit Node with id '-1' is not the grounded, please adjust the definition of the section (with substation '%').",
                substation_id
            );
        }
        // start with (voltageSources->front()->getPosNode())
        let first_vs = *self.voltage_sources.first().expect("circuit has no voltage source");
        nodes_visited[self.element(first_vs).id() as usize] = true;
        let mut queue: Vec<NodeIdx> = vec![self.element(first_vs).p_node().expect("element has no positive node")];

        while let Some(node) = queue.pop() {
            let node_id = self.node(node).id() as usize;
            if !nodes_visited[node_id] {
                nodes_visited[node_id] = true;
                for &el in self.node(node).elements() {
                    let neighboring = self.element(el).get_the_other_node(node).expect("element not connected to node");
                    if !self.node(neighboring).is_ground() {
                        queue.push(neighboring);
                    } else if self.element(el).element_type() == ElementType::VoltageSource {
                        // there used to be == 1 which was probably a typo ... check!
                        nodes_visited[self.element(el).id() as usize] = true;
                    } else if self.element(el).element_type() == ElementType::Resistor {
                        write_errorf!(
                            "A Circuit Resistor Element connects the ground, please adjust the definition of the section (with substation '%').",
                            substation_id
                        );
                    }
                }
            }
        }

        for (i, visited) in nodes_visited.iter().enumerate() {
            if !visited {
                write_warningf!(
                    "Circuit Node or Voltage Source with internal id '%' has been not visited during checking of the circuit. The circuit is disconnected, please adjust the definition of the section (with substation '%').",
                    i.to_string(),
                    substation_id
                );
            }
        }

        true
    }

    // solves the circuit and deploys the results
    pub fn solve(&mut self) -> bool {
        if !self.is_cleaned {
            self.clean_up_sp();
        }
        self._solve_nr_method()
    }

    fn _solve_nr_method(&mut self) -> bool {
        let mut removable_ids: Vec<i32> = Vec::new();

        self.detect_removable_nodes(&mut removable_ids);
        let (eqn, mut vals) = self.create_equations_nr_method(&mut removable_ids);
        if !self.solve_equations_nr_method(&eqn, &mut vals, &removable_ids) {
            return false;
        }
        // vals are now the solution x of the circuit
        self.deploy_results(&vals, &removable_ids);
        true
    }

    /// Select removable nodes, i.e. nodes that are NOT the ground of the circuit
    /// and that have exactly two resistor elements connected. Ids of those
    /// removable nodes are added into `removable_ids`, which is sorted afterwards.
    fn detect_removable_nodes(&mut self, removable_ids: &mut Vec<i32>) {
        for k in 0..self.nodes.len() {
            let n = self.nodes[k];
            // if the node is connected to two elements and is not the ground
            if self.node(n).num_of_elements() == 2 && !self.node(n).is_ground() {
                // such node is removable only if both of its elements are resistors
                let all_resistors = self
                    .node(n)
                    .elements()
                    .iter()
                    .all(|&e| self.element(e).element_type() == ElementType::Resistor);
                self.node_mut(n).set_removability(all_resistors);
                if all_resistors {
                    // if the node is removable add its id into the vector of removable nodes
                    removable_ids.push(self.node(n).id());
                }
            } else {
                self.node_mut(n).set_removability(false);
            }
        }
        // sort the vector of removable ids
        removable_ids.sort_unstable();
    }

    /// creates all of the equations that represent the circuit
    /// in the form Ax = B(1/x) where A and B are matrices.
    /// Returns (eqs, vals): A as a row-major n-wide buffer and B.
    fn create_equations_nr_method(&mut self, removable_ids: &mut Vec<i32>) -> (Vec<f64>, Vec<f64>) {
        // removable_ids does not include nodes with voltage source yet

        // number of voltage sources + nodes without the ground node
        let n = self.voltage_sources.len() + self.nodes.len() - 1;
        // number of equations
        // assumption: each voltage source has different positive node and common ground node,
        //             i.e. any node excluding the ground node is connected to 0 or 1 voltage source
        let m = n - (removable_ids.len() + self.voltage_sources.len());

        // allocate and initialize zero matrix eqs and vector vals
        let mut eqs = vec![0.0; m * n];
        let mut vals = vec![0.0; m];

        // loop over all nodes
        let mut i = 0usize;
        for k in 0..self.nodes.len() {
            let node = self.nodes[k];
            if self.node(node).is_ground() || self.node(node).is_removable() {
                // if the node is grounded or is removable set the corresponding number of row in matrix to -1 (no equation in eqs)
                self.node_mut(node).set_num_matrix_row(-1);
                continue;
            }
            assert!(i < m);
            // constitute the equation corresponding to node it, add all passed voltage source elements into removable_ids
            let no_voltage_source = self.create_equation_nr_method(
                node,
                &mut eqs[n * i..n * (i + 1)],
                &mut vals[i],
                removable_ids,
            );
            // if the node it has element of type "voltage source" we do not use the equation, because some value of current throw the voltage source can be always find
            if no_voltage_source {
                self.node_mut(node).set_num_matrix_row(i as i32);
                i += 1;
            } else {
                self.node_mut(node).set_num_matrix_row(-2);
                vals[i] = 0.0;
                for j in 0..n {
                    eqs[n * i + j] = 0.0;
                }
            }
        }

        // removable_ids includes nodes with voltage source already
        removable_ids.sort_unstable();

        for k in 0..self.voltage_sources.len() {
            let vs = self.voltage_sources[k];
            assert!(i < m);
            self.create_equation(vs, &mut eqs[n * i..n * (i + 1)], &mut vals[i]);
            i += 1;
        }

        (eqs, vals)
    }

    /// creates the equation V2 - V1 = E of the voltage source, a row of A and a value of B
    fn create_equation(&self, vsource: ElemIdx, eqn: &mut [f64], val: &mut f64) -> bool {
        let p = self.element(vsource).p_node().expect("element has no positive node");
        let n = self.element(vsource).n_node().expect("element has no negative node");
        if !self.node(p).is_ground() {
            eqn[self.node(p).id() as usize] = 1.0;
        }
        if !self.node(n).is_ground() {
            eqn[self.node(n).id() as usize] = -1.0;
        }
        if self.element(vsource).is_enabled() {
            *val = self.element(vsource).get_voltage(&self.node_arena);
        } else {
            *val = 0.0;
        }
        true
    }

    /// creates the nodal equation of the node 'node' GV = I in the form Ax = B(1/x),
    /// where A is a matrix with one row. Returns false if the node has a voltage source.
    fn create_equation_nr_method(
        &self,
        node: NodeIdx,
        eqn: &mut [f64],
        val: &mut f64,
        removable_ids: &mut Vec<i32>,
    ) -> bool {
        // loop over all elements connected to the node
        for &el in self.node(node).elements() {
            let element = self.element(el);
            match element.element_type() {
                ElementType::Resistor => {
                    if element.is_enabled() {
                        let mut x = element.resistance();
                        // go through all neighboring removable nodes and sum resistance of resistors in the serial branch
                        let mut next_non_removable_node =
                            element.get_the_other_node(node).expect("element not connected to node");
                        let mut next_serial_resistor = el;
                        while self.node(next_non_removable_node).is_removable() {
                            next_serial_resistor = self
                                .node(next_non_removable_node)
                                .get_an_other_element(next_serial_resistor)
                                .expect("removable node without a second element");
                            x += self.element(next_serial_resistor).resistance();
                            next_non_removable_node = self
                                .element(next_serial_resistor)
                                .get_the_other_node(next_non_removable_node)
                                .expect("element not connected to node");
                        }
                        // compute inverse value and place/add this value at proper places in eqn
                        x = 1.0 / x;
                        eqn[self.node(node).id() as usize] += x;

                        if !self.node(next_non_removable_node).is_ground() {
                            eqn[self.node(next_non_removable_node).id() as usize] -= x;
                        }
                    }
                }
                ElementType::CurrentSource => {
                    let x = if element.is_enabled() {
                        // initialize current in current source
                        if element.p_node() == Some(node) {
                            -element.power_wanted() / self.front_voltage_source_voltage()
                        } else {
                            element.power_wanted() / self.front_voltage_source_voltage()
                        }
                    } else {
                        0.0
                    };
                    *val += x;
                }
                ElementType::VoltageSource => {
                    let x = if element.p_node() == Some(node) { -1.0 } else { 1.0 };
                    eqn[element.id() as usize] += x;
                    // equations with voltage source can be ignored, because some value of current throw the voltage source can be always find
                    removable_ids.push(element.id());
                    return false;
                }
                ElementType::Error => return false,
            }
        }
        true
    }

    /// Solves the non-linear system with the Newton-Raphson method, scaling the power of the
    /// current sources by alpha when needed. Overwrites `vals` with the solution x_best.
    fn solve_equations_nr_method(&mut self, eqn: &[f64], vals: &mut [f64], removable_ids: &[i32]) -> bool {
        // removable_ids includes nodes with voltage source already
        let numofcolumn = self.voltage_sources.len() + self.nodes.len() - 1;
        let numofeqs = numofcolumn - removable_ids.len();

        // map equations into matrix A
        let mut a = DenseMatrix::from_row_major(numofeqs, numofcolumn, eqn[..numofeqs * numofcolumn].to_vec());

        // remove removable columns of matrix A, i.e. remove equations corresponding to nodes with two resistors connected in series
        for &id in removable_ids.iter().rev() {
            let id = if id >= 0 { id } else { -id };
            a.remove_column(id as usize);
        }

        // detect number of column for each node
        // in other words: detect elements of x to certain node
        // in other words: assign number of column to the proper non removable node
        let mut j = 0usize;
        for i in 0..numofcolumn {
            if let Some(node) = self.get_node_by_id(i as i32) {
                if self.node(node).is_removable() {
                    self.node_mut(node).set_num_matrix_col(-1);
                    continue;
                } else {
                    if j > numofeqs {
                        write_error("Index of renumbered node exceeded the reduced number of equations.");
                        break;
                    }
                    self.node_mut(node).set_num_matrix_col(j as i32);
                    j += 1;
                    continue;
                }
            } else if self.get_element_by_id(i as i32).is_some() {
                if j > numofeqs {
                    write_error("Index of renumbered element exceeded the reduced number of equations.");
                    break;
                }
                continue;
            }
            // tNode == nullptr && tElem == nullptr
            write_error("Structural error in reduced circuit matrix.");
        }

        // map 'vals' into vector b and initialize solution x
        let mut x = col_piv_householder_qr_solve(&a, &vals[..numofeqs]);

        // initialize progressively increasing maximal number of Newton-Rhapson iterations
        let mut max_iter_of_nr = 10;
        // value of scaling parameter alpha
        let mut alpha = 1.0_f64;
        // the best (maximum) value of alpha that guarantees the existence of solution
        self.alpha_best = 0.0;
        // reason why is alpha not 1
        self.alpha_reason = AlphaFlag::NotApplied;
        // vector of alphas for that no solution has been found
        let mut alpha_not_solution: Vec<f64> = Vec::new();
        // initialize progressively decreasing tolerance for alpha
        let mut alpha_res = 1e-2;

        // solution x corresponding to the alphaBest
        // (the C++ x_best_exist flag is always true, so it is not kept)
        let mut x_best = x.clone();

        if x_max(&x) > 10e6 || x_min(&x) < -10e6 {
            write_error("Initial solution x used during solving DC circuit is out of bounds.\n");
        }

        let num_current_sources = numofeqs - self.voltage_sources.len();
        let nodes = self.nodes.clone();

        // Search for the suitable scaling value alpha
        loop {
            let mut iter_nr = 0;
            // run Newton-Raphson methods
            loop {
                // update right-hand side vector vals and Jacobian matrix J
                // node's right-hand side set to zero
                for v in vals.iter_mut().take(num_current_sources) {
                    *v = 0.0;
                }
                // Jacobian J = A, updated by the current-source terms below
                let mut jac = a.clone();

                let mut i = 0usize;
                for &node in &nodes {
                    if self.node(node).is_ground()
                        || self.node(node).is_removable()
                        || self.node(node).num_matrix_row() == -2
                    {
                        continue;
                    }
                    if self.node(node).num_matrix_row() != i as i32 {
                        write_error("wrongly assigned row of matrix A during solving the circuit");
                    }
                    // loop over all node's elements
                    for el in self.node(node).elements().to_vec() {
                        let element = self.element(el);
                        if element.element_type() != ElementType::CurrentSource || !element.is_enabled() {
                            continue;
                        }
                        let power = element.power_wanted();
                        let pos = element.p_node().expect("element has no positive node");
                        let neg = element.n_node().expect("element has no negative node");
                        let pos_col = self.node(pos).num_matrix_col();
                        let neg_col = self.node(neg).num_matrix_col();
                        // compute voltage on current source
                        let diff_voltage = if pos_col == -1 {
                            // if the positive node is the ground => U = 0 - phi(NegNode)
                            -x[neg_col as usize]
                        } else if neg_col == -1 {
                            // if the negative node is the ground => U = phi(PosNode) - 0
                            x[pos_col as usize]
                        } else {
                            // U = phi(PosNode) - phi(NegNode)
                            x[pos_col as usize] - x[neg_col as usize]
                        };

                        if pos == node {
                            // the positive current (the element is consuming energy if powerWanted > 0) is flowing from the positive node (sign minus)
                            vals[i] -= alpha * power / diff_voltage;
                            self.element_mut(el).set_current(-alpha * power / diff_voltage);
                            if pos_col != -1 {
                                // -1* d_b/d_phiPos = -1* d(-alpha*P/(phiPos-phiNeg) )/d_phiPos = -1* (--alpha*P/(phiPos-phiNeg)^2 )
                                *jac.get_mut(i, pos_col as usize) -= alpha * power / diff_voltage / diff_voltage;
                            }
                            if neg_col != -1 {
                                // -1* d_b/d_phiNeg = -1* d(-alpha*P/(phiPos-phiNeg) )/d_phiNeg = -1* (---alpha*P/(phiPos-phiNeg)^2 )
                                *jac.get_mut(i, neg_col as usize) += alpha * power / diff_voltage / diff_voltage;
                            }
                        } else {
                            // the positive current (the element is consuming energy if powerWanted > 0) is flowing to the negative node (sign plus)
                            vals[i] += alpha * power / diff_voltage;
                            // Note: we should never reach this part of code since the authors assumes the negative node of current source as the ground node
                            write_warning("The negative node of current source is not the ground.");
                            if pos_col != -1 {
                                // -1* d_b/d_phiPos = -1* d(alpha*P/(phiPos-phiNeg) )/d_phiPos = -1* (-alpha*P/(phiPos-phiNeg)^2 )
                                *jac.get_mut(i, pos_col as usize) += alpha * power / diff_voltage / diff_voltage;
                            }
                            if neg_col != -1 {
                                // -1* d_b/d_phiNeg = -1* d(alpha*P/(phiPos-phiNeg) )/d_phiNeg = -1* (--alpha*P/(phiPos-phiNeg)^2 )
                                *jac.get_mut(i, neg_col as usize) -= alpha * power / diff_voltage / diff_voltage;
                            }
                        }
                    }
                    i += 1;
                }

                // Sum of currents going through the all voltage sources
                // the sum is over all nodes, but the nonzero nodes are only those neighboring with current sources,
                // so the sum is negative sum of currents through/from current sources representing trolleybuses
                let mut current_sum_actual = 0.0;
                for v in vals.iter().take(num_current_sources) {
                    current_sum_actual -= v;
                }

                // residual A * x - b
                let ax = a.mul_vec(&x);
                let residual: Vec<f64> = (0..numofeqs).map(|r| ax[r] - vals[r]).collect();
                let residual_norm = eigen_norm(&residual); // Circuit.cpp: (A * x - b).norm()

                if residual_norm < 1e-6 {
                    // current limits
                    if current_sum_actual > self.get_current_limit() && MY_CURRENT_LIMITS.load(Ordering::Relaxed) {
                        self.alpha_reason = AlphaFlag::CurrentLimits;
                        alpha_not_solution.push(alpha);
                        x = x_best.clone();
                        break;
                    }
                    // voltage limits 70% - 120% of nominal voltage
                    let nominal = self.front_voltage_source_voltage();
                    if x_max(&x) > nominal * 1.2 || x_min(&x) < nominal * 0.7 {
                        self.alpha_reason = AlphaFlag::VoltageLimits;
                        alpha_not_solution.push(alpha);
                        x = x_best.clone();
                        break;
                    }

                    self.alpha_best = alpha;
                    x_best = x.clone();
                    break;
                } else if iter_nr == max_iter_of_nr {
                    self.alpha_reason = AlphaFlag::NotConverging;
                    alpha_not_solution.push(alpha);
                    x = x_best.clone();
                    break;
                }

                // Newton=Rhapson iteration
                let dx: Vec<f64> = col_piv_householder_qr_solve(&jac, &residual)
                    .into_iter()
                    .map(|d| -d)
                    .collect();
                for (xk, dk) in x.iter_mut().zip(&dx) {
                    *xk += dk;
                }
                iter_nr += 1;
            }

            if alpha_not_solution.is_empty() {
                // no alpha without solution is in the alpha_notSolution, so the solving procedure is terminating
                break;
            }

            let last = *alpha_not_solution.last().expect("checked non-empty");
            if (last - self.alpha_best) < alpha_res {
                max_iter_of_nr *= 2;
                // it follows Sevcik, Jakub, et al. "Solvability of the Power Flow Problem in DC Overhead Wire Circuit Modeling." Applications of Mathematics (2021): 1-19.
                // see Alg 2 (progressive decrease of optimality tolerance)
                alpha_res /= 10.0;
                if alpha_res < 5e-5 {
                    break;
                }
                alpha = last;
                alpha_not_solution.pop();
                continue;
            }

            alpha = self.alpha_best + 0.5 * (last - self.alpha_best);
        }

        // vals is pointer to memory and we use it now for saving solution x_best instead of right-hand side b
        vals[..numofeqs].copy_from_slice(&x_best[..numofeqs]);

        // we take x_best and alphaBest and update current values in current sources in order to be in agreement with the solution
        let mut i = 0usize;
        for &node in &nodes {
            if self.node(node).is_ground()
                || self.node(node).is_removable()
                || self.node(node).num_matrix_row() == -2
            {
                continue;
            }
            if self.node(node).num_matrix_row() != i as i32 {
                write_error("wrongly assigned row of matrix A during solving the circuit");
            }
            for el in self.node(node).elements().to_vec() {
                let element = self.element(el);
                if element.element_type() != ElementType::CurrentSource || !element.is_enabled() {
                    continue;
                }
                let pos = element.p_node().expect("element has no positive node");
                let neg = element.n_node().expect("element has no negative node");
                let pos_col = self.node(pos).num_matrix_col();
                let neg_col = self.node(neg).num_matrix_col();
                let diff_voltage = if pos_col == -1 {
                    -x_best[neg_col as usize]
                } else if neg_col == -1 {
                    x_best[pos_col as usize]
                } else {
                    x_best[pos_col as usize] - x_best[neg_col as usize]
                };

                if pos == node {
                    let current = -self.alpha_best * element.power_wanted() / diff_voltage;
                    self.element_mut(el).set_current(current);
                } else {
                    // Note: we should never reach this part of code since the authors assumes the negative node of current source as the ground node
                    write_warning("The negative node of current source is not the ground.");
                }
            }
            i += 1;
        }

        true
    }

    fn deploy_results(&mut self, vals: &[f64], removable_ids: &[i32]) {
        // vals are the solution x

        let numofcolumn = self.voltage_sources.len() + self.nodes.len() - 1;
        let numofeqs = numofcolumn - removable_ids.len();

        // loop over non-removable nodes: we assign the computed voltage to the non-removables nodes
        let mut j = 0usize;
        for i in 0..numofcolumn {
            if let Some(node) = self.get_node_by_id(i as i32) {
                if self.node(node).is_removable() {
                    continue;
                } else {
                    if j > numofeqs {
                        write_error("Results deployment during circuit evaluation was unsuccessful.");
                        break;
                    }
                    self.node_mut(node).set_voltage(vals[j]);
                    j += 1;
                    continue;
                }
            } else if self.get_element_by_id(i as i32).is_some() {
                if j > numofeqs {
                    write_error("Results deployment during circuit evaluation was unsuccessful.");
                    break;
                }
                // tElem should be voltage source - the current through voltage source is computed in a loop below
                // if tElem is current source, the current is already assign at the end of solveEquationsNRmethod method
                continue;
            }
            write_error("Results deployment during circuit evaluation was unsuccessful.");
        }

        // interpolate result of voltage to removable nodes
        for k in 0..self.nodes.len() {
            let node = self.nodes[k];
            if !self.node(node).is_removable() {
                continue;
            }
            if self.node(node).num_of_elements() != 2 {
                continue;
            }

            let elems = self.node(node).elements();
            let mut el1 = elems[0];
            let mut el2 = elems[elems.len() - 1];
            let mut next_non_removable_node1 =
                self.element(el1).get_the_other_node(node).expect("element not connected to node");
            let mut next_non_removable_node2 =
                self.element(el2).get_the_other_node(node).expect("element not connected to node");
            let mut x = self.element(el1).resistance();
            let mut y = self.element(el2).resistance();

            while self.node(next_non_removable_node1).is_removable() {
                el1 = self
                    .node(next_non_removable_node1)
                    .get_an_other_element(el1)
                    .expect("removable node without a second element");
                x += self.element(el1).resistance();
                next_non_removable_node1 = self
                    .element(el1)
                    .get_the_other_node(next_non_removable_node1)
                    .expect("element not connected to node");
            }

            while self.node(next_non_removable_node2).is_removable() {
                el2 = self
                    .node(next_non_removable_node2)
                    .get_an_other_element(el2)
                    .expect("removable node without a second element");
                y += self.element(el2).resistance();
                next_non_removable_node2 = self
                    .element(el2)
                    .get_the_other_node(next_non_removable_node2)
                    .expect("element not connected to node");
            }

            x = x / (x + y);
            let v1 = self.node(next_non_removable_node1).voltage();
            let v2 = self.node(next_non_removable_node2).voltage();
            self.node_mut(node).set_voltage(((1.0 - x) * v1) + (x * v2));
            self.node_mut(node).set_removability(false);
        }

        // Update the electric currents for voltage sources (based on Kirchhof's law: current out = current in)
        for k in 0..self.voltage_sources.len() {
            let vs = self.voltage_sources[k];
            let pos = self.element(vs).p_node().expect("element has no positive node");
            let mut current_sum = 0.0;
            for el in self.node(pos).elements().to_vec() {
                // loop over all elements on PosNode excluding the actual voltage source it
                if el != vs {
                    let other = self
                        .element(el)
                        .get_the_other_node(pos)
                        .expect("element not connected to node");
                    current_sum += (self.node(pos).voltage() - self.node(other).voltage()) / self.element(el).resistance();
                    if self.element(el).element_type() == ElementType::VoltageSource {
                        write_warning("Cannot assign unambigous electric current value to two voltage sources connected in parallel at the same node.");
                    }
                }
            }
            self.element_mut(vs).set_current(current_sum);
        }
    }
}

fn x_max(x: &[f64]) -> f64 {
    x.iter().copied().fold(f64::NEG_INFINITY, f64::max)
}

fn x_min(x: &[f64]) -> f64 {
    x.iter().copied().fold(f64::INFINITY, f64::min)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Voltage divider: 10 V source at A, R1 = 1 ohm (A-B), R2 = 1 ohm (B-ground).
    // Hand-traced through the C++ algorithm: B is removable, the solution is A = 10, B = 5, source current 5.
    #[test]
    fn voltage_divider_with_serial_resistors() {
        let mut c = Circuit::new();
        let g = c.add_node("ground").unwrap();
        let a = c.add_node("A").unwrap();
        let b = c.add_node("B").unwrap();
        c.add_element("vs", 10.0, a, g, ElementType::VoltageSource).unwrap();
        c.add_element("r1", 1.0, a, b, ElementType::Resistor).unwrap();
        c.add_element("r2", 1.0, b, g, ElementType::Resistor).unwrap();

        assert!(c.check_circuit(""));
        assert!(c.solve());
        assert!((c.get_voltage("A") - 10.0).abs() < 1e-9);
        assert!((c.get_voltage("B") - 5.0).abs() < 1e-9);
        assert!((c.get_current("vs") - 5.0).abs() < 1e-9);
        assert_eq!(c.get_alpha_best(), 1.0);
        assert_eq!(c.get_alpha_reason(), AlphaFlag::NotApplied);
    }

    // Constant-power load (10 kW) at B behind a 0.5 ohm line from a 600 V source.
    // KCL: 2 (600 - U) = 10000 / U  =>  U^2 - 600 U + 5000 = 0  =>  U = 300 + sqrt(85000) (upper root).
    #[test]
    fn constant_power_load_newton_raphson() {
        let mut c = Circuit::new();
        let g = c.add_node("ground").unwrap();
        let a = c.add_node("A").unwrap();
        let b = c.add_node("B").unwrap();
        c.add_element("vs", 600.0, a, g, ElementType::VoltageSource).unwrap();
        c.add_element("line", 0.5, a, b, ElementType::Resistor).unwrap();
        let load = c.add_element("load", 0.0, b, g, ElementType::CurrentSource).unwrap();
        c.element_mut(load).set_power_wanted(10000.0);

        assert!(c.solve());
        let expected = 300.0 + 85000.0_f64.sqrt();
        assert!((c.get_voltage("B") - expected).abs() < 1e-6, "U_B = {}", c.get_voltage("B"));
        assert!((c.get_voltage("A") - 600.0).abs() < 1e-9);
        assert_eq!(c.get_alpha_best(), 1.0);
        assert_eq!(c.get_alpha_reason(), AlphaFlag::NotApplied);
    }

    #[test]
    fn resistor_below_minimum_is_clamped_and_negative_is_rejected() {
        let mut c = Circuit::new();
        let g = c.add_node("ground").unwrap();
        let a = c.add_node("A").unwrap();
        c.add_element("tiny", 0.0, a, g, ElementType::Resistor).unwrap();
        assert_eq!(c.get_resistance("tiny"), 1e-6);
        assert!(c.add_element("neg", -1.0, a, g, ElementType::Resistor).is_none());
    }
}
