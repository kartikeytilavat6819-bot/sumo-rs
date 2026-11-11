//! Port of utils/traction_wire/Element.{h,cpp}
//!
//! Representation of electric circuit elements: resistors, voltage and current sources.
//! The C++ `Node*` terminals are `NodeIdx` values into the owning `Circuit`; methods that need
//! a terminal's voltage take the circuit's node slice.

use crate::write_errorf;

use super::node::{Node, NodeIdx};

/// Index of an element in the owning `Circuit` (the C++ `Element*`).
pub type ElemIdx = usize;

/// An element is any component in the circuit (resistor, current source, voltage source).
/// Every element has two terminals pNode (positive terminal) and nNode (negative terminal);
/// value is the resistance in case of a resistor, current in case of a current source and
/// voltage in case of voltage source.
///
/// Conventions used:
///
///   1 - in case of a current source, "value" represents the current going from nNode to pNode,
///   2 - in case of a voltage source, "value" represents the voltage difference of pNode - nNode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElementType {
    Resistor,
    CurrentSource,
    VoltageSource,
    Error,
}

#[derive(Debug, Clone)]
pub struct Element {
    p_node: Option<NodeIdx>,
    n_node: Option<NodeIdx>,
    voltage: f64,
    current: f64,
    resistance: f64,
    power_wanted: f64,
    element_type: ElementType,
    // unique property, each object has distinctive and unique name
    name: String,
    // a sequential ID number, might be useful when making the equation
    id: i32,
    is_enabled: bool,
}

impl Element {
    // a constructor. same functionality as init functions in the last project
    pub fn new(name: &str, element_type: ElementType, value: f64) -> Self {
        let mut e = Self {
            p_node: None,
            n_node: None,
            voltage: 0.0,
            current: 0.0,
            resistance: 0.0,
            power_wanted: f64::NAN,
            element_type,
            name: name.to_string(),
            id: -2,
            is_enabled: true,
        };
        match element_type {
            ElementType::CurrentSource => e.current = value,
            ElementType::VoltageSource => e.voltage = value,
            ElementType::Resistor => e.resistance = value,
            ElementType::Error => write_errorf!("Undefined element type for '%'.", name),
        }
        e
    }

    // getters and setters

    // get the voltage across the element
    pub fn get_voltage(&self, nodes: &[Node]) -> f64 {
        if !self.is_enabled {
            return f64::MAX;
        }
        if self.element_type == ElementType::VoltageSource {
            return self.voltage;
        }
        let p = self.p_node.expect("element has no positive node");
        let n = self.n_node.expect("element has no negative node");
        nodes[p].voltage() - nodes[n].voltage()
    }

    // get the current running through the element
    pub fn get_current(&self, nodes: &[Node]) -> f64 {
        if !self.is_enabled {
            return f64::MAX;
        }
        match self.element_type {
            ElementType::Resistor => -1.0 * self.get_voltage(nodes) / self.resistance,
            ElementType::CurrentSource | ElementType::VoltageSource => self.current,
            ElementType::Error => 0.0,
        }
    }

    pub fn set_voltage(&mut self, voltage_in: f64) {
        self.voltage = voltage_in;
    }

    pub fn set_current(&mut self, current_in: f64) {
        self.current = current_in;
    }

    pub fn resistance(&self) -> f64 {
        self.resistance
    }

    // resistances below 1e-6 are raised to 1e-6 for numerical reasons
    pub fn set_resistance(&mut self, resistance_in: f64) {
        if resistance_in <= 1e-6 {
            self.resistance = 1e-6;
        } else {
            self.resistance = resistance_in;
        }
    }

    pub fn power_wanted(&self) -> f64 {
        self.power_wanted
    }

    pub fn set_power_wanted(&mut self, power_wanted_in: f64) {
        self.power_wanted = power_wanted_in;
    }

    pub fn get_power(&self, nodes: &[Node]) -> f64 {
        -1.0 * self.get_current(nodes) * self.get_voltage(nodes)
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn set_id(&mut self, new_id: i32) {
        self.id = new_id;
    }

    pub fn p_node(&self) -> Option<NodeIdx> {
        self.p_node
    }

    pub fn n_node(&self) -> Option<NodeIdx> {
        self.n_node
    }

    pub fn set_pos_node(&mut self, node: NodeIdx) {
        self.p_node = Some(node);
    }

    pub fn set_neg_node(&mut self, node: NodeIdx) {
        self.n_node = Some(node);
    }

    pub fn element_type(&self) -> ElementType {
        self.element_type
    }

    // sets the type of elements
    pub fn set_type(&mut self, et: ElementType) {
        self.element_type = et;
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn is_enabled(&self) -> bool {
        self.is_enabled
    }

    pub fn set_enabled(&mut self, new_is_enabled: bool) {
        self.is_enabled = new_is_enabled;
    }

    // if node == pNode, return nNode, else if node == nNode return pNode, else return None
    pub fn get_the_other_node(&self, node: NodeIdx) -> Option<NodeIdx> {
        if Some(node) == self.p_node {
            self.n_node
        } else if Some(node) == self.n_node {
            self.p_node
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resistance_setter_clamps_small_values() {
        let mut e = Element::new("r", ElementType::Resistor, 3.0);
        assert_eq!(e.resistance(), 3.0);
        e.set_resistance(1e-9);
        assert_eq!(e.resistance(), 1e-6);
    }

    #[test]
    fn other_node_of_terminal() {
        let mut e = Element::new("r", ElementType::Resistor, 1.0);
        e.set_pos_node(4);
        e.set_neg_node(7);
        assert_eq!(e.get_the_other_node(4), Some(7));
        assert_eq!(e.get_the_other_node(7), Some(4));
        assert_eq!(e.get_the_other_node(9), None);
    }

    #[test]
    fn disabled_element_reports_dbl_max() {
        let mut e = Element::new("v", ElementType::VoltageSource, 600.0);
        e.set_enabled(false);
        assert_eq!(e.get_voltage(&[]), f64::MAX);
        assert_eq!(e.get_current(&[]), f64::MAX);
    }
}
