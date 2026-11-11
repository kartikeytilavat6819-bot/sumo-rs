//! Port of utils/traction_wire/Node.{h,cpp}
//!
//! Elements are referenced by their index in the owning `Circuit` (`ElemIdx`) instead of by
//! pointer; the C++ `std::vector<Element*>*` becomes a plain `Vec<ElemIdx>`.

use super::element::ElemIdx;

/// Index of a node in the owning `Circuit` (the C++ `Node*`).
pub type NodeIdx = usize;

/// Representation of electric circuit nodes, i.e. wire junctions and connection points.
#[derive(Debug, Clone)]
pub struct Node {
    is_ground: bool,
    is_removable: bool,
    // unique property, each object has distinctive and unique name
    name: String,
    // a sequential ID number, might be useful when making the equation
    id: i32,
    // number of matrix row during solving the equations
    num_matrix_row: i32,
    // number of matrix column during solving the equations
    num_matrix_col: i32,
    voltage: f64,
    // each node is connected to one or more element, an element is a resistor or voltage/current source
    elements: Vec<ElemIdx>,
}

impl Node {
    // A constructor, same functionality as "init" functions
    pub fn new(name: &str, id: i32) -> Self {
        Self {
            is_ground: false,
            is_removable: false,
            name: name.to_string(),
            id,
            num_matrix_row: -1,
            num_matrix_col: -1,
            voltage: 0.0,
            elements: Vec::new(),
        }
    }

    // connects an element to the node
    pub fn add_element(&mut self, element: ElemIdx) {
        self.elements.push(element);
    }

    // disconnects an element from the node (all occurrences)
    pub fn erase_element(&mut self, element: ElemIdx) {
        self.elements.retain(|&e| e != element);
    }

    pub fn voltage(&self) -> f64 {
        self.voltage
    }

    pub fn set_voltage(&mut self, volt: f64) {
        self.voltage = volt;
    }

    pub fn num_of_elements(&self) -> usize {
        self.elements.len()
    }

    // returns the first element which is not equal to "element", None if there is none
    pub fn get_an_other_element(&self, element: ElemIdx) -> Option<ElemIdx> {
        self.elements.iter().copied().find(|&it| it != element)
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn is_ground(&self) -> bool {
        self.is_ground
    }

    pub fn set_ground(&mut self, new_is_ground: bool) {
        self.is_ground = new_is_ground;
    }

    pub fn is_removable(&self) -> bool {
        self.is_removable
    }

    pub fn set_removability(&mut self, new_is_removable: bool) {
        self.is_removable = new_is_removable;
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn set_id(&mut self, new_id: i32) {
        self.id = new_id;
    }

    pub fn num_matrix_row(&self) -> i32 {
        self.num_matrix_row
    }

    pub fn set_num_matrix_row(&mut self, num: i32) {
        self.num_matrix_row = num;
    }

    pub fn num_matrix_col(&self) -> i32 {
        self.num_matrix_col
    }

    pub fn set_num_matrix_col(&mut self, num: i32) {
        self.num_matrix_col = num;
    }

    pub fn elements(&self) -> &[ElemIdx] {
        &self.elements
    }
}
