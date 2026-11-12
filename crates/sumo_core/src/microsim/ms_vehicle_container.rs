//! Port of microsim/MSVehicleContainer.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `std::pair<SUMOTime, VehicleVector> MSVehicleContainer::VehicleDepartureVector`
pub type MSVehicleContainer_VehicleDepartureVector = (SUMOTime, MSVehicleContainer_VehicleVector);

/// C++ typedef `std::vector<VehicleDepartureVector> MSVehicleContainer::VehicleHeap`
pub type MSVehicleContainer_VehicleHeap = Vec<MSVehicleContainer_VehicleDepartureVector>;

/// C++ typedef `std::vector<SUMOVehicle *> MSVehicleContainer::VehicleVector`
pub type MSVehicleContainer_VehicleVector = Vec<Ptr<SUMOVehicle>>;

/// Searches for the VehicleDepartureVector with the wished depart
/// C++ class `MSVehicleContainer::DepartFinder` (microsim/MSVehicleContainer.h)
pub struct MSVehicleContainer_DepartFinder {
    /// the searched departure time
    pub myTime: SUMOTime,
}

impl MSVehicleContainer_DepartFinder {
    /// C++: `void DepartFinder::DepartFinder(SUMOTime)`
    /// constructor
    // MSVehicleContainer::DepartFinder::DepartFinder(SUMOTime time)
    //     : myTime(time) {}
    #[allow(unused_variables)]
    pub fn new(time: SUMOTime) -> Self {
        Self { myTime: time }
    }

    /// C++: `bool DepartFinder::operator()(const VehicleDepartureVector &) const` [const]
    /// comparison operator
    // bool
    // MSVehicleContainer::DepartFinder::operator()
    // (const VehicleDepartureVector& e) const {
    //     return myTime + DELTA_T > e.first && myTime <= e.first;
    // }
    #[allow(unused_variables)]
    pub fn call_op(&self, e: &MSVehicleContainer_VehicleDepartureVector) -> bool {
        self.myTime + delta_t() > e.0 && self.myTime <= e.0
    }

}

/// A storage for vehicles, mainly used by the vehicle/route loading structures and the insertion control. Stores vehicles in a heap of vehiclevector/departure- pairs.
/// C++ class `MSVehicleContainer` (microsim/MSVehicleContainer.h)
pub struct MSVehicleContainer {
    /// Number of elements in heap
    pub currentSize: i32,
    /// The vehicle vector heap
    pub array: MSVehicleContainer_VehicleHeap,
}

impl MSVehicleContainer {
    /// C++: `void MSVehicleContainer::MSVehicleContainer(int)`
    /// Constructor
    // MSVehicleContainer::MSVehicleContainer(int capacity)
    //     : currentSize(0), array(capacity + 1, VehicleDepartureVector()) {}
    #[allow(unused_variables)]
    pub fn new(capacity: i32) -> Self {
        Self {
            currentSize: 0,
            array: (0..capacity + 1).map(|_| (0, MSVehicleContainer_VehicleVector::new())).collect(),
        }
    }

    /// C++: `void MSVehicleContainer::~MSVehicleContainer()`
    /// Destructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // destructor: nothing to release explicitly, the Vec fields drop with the struct
    }

    /// C++: `void MSVehicleContainer::add(SUMOVehicle *)`
    /// Adds a single vehicle
    // void
    // MSVehicleContainer::add(SUMOVehicle* veh) {
    //     // check whether a new item shall be added or the vehicle may be
    //     //  added to an existing list
    //     VehicleHeap::iterator i =
    //         find_if(array.begin() + 1, array.begin() + currentSize + 1, DepartFinder(veh->getParameter().depart));
    //     if (currentSize == 0 || i == array.begin() + currentSize + 1) {
    //         // a new heap-item is necessary
    //         const SUMOTime delay = veh->getParameter().depart % DELTA_T;
    //         const SUMOTime depart = veh->getParameter().depart + (delay == 0 ? 0 : DELTA_T - delay);
    //         VehicleDepartureVector newElem(depart, VehicleVector());
    //         newElem.second.push_back(veh);
    //         addReplacing(newElem);
    //     } else {
    //         // add vehicle to an existing heap-item
    //         (*i).second.push_back(veh);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add(&mut self, veh: Ptr<SUMOVehicle>) {
        todo!("port MSVehicleContainer::add")
    }

    /// C++: `void MSVehicleContainer::remove(SUMOVehicle *)`
    /// Removes a single vehicle
    // void
    // MSVehicleContainer::remove(SUMOVehicle* veh) {
    //     // check whether a new item shall be added or the vehicle may be
    //     //  added to an existing list
    //     VehicleHeap::iterator i =
    //         find_if(array.begin() + 1, array.begin() + currentSize + 1, DepartFinder(veh->getParameter().depart));
    //     if (!(currentSize == 0 || i == array.begin() + currentSize + 1)) {
    //         // remove vehicle from an existing heap-item
    //         (*i).second.erase(std::remove((*i).second.begin(), (*i).second.end(), veh), (*i).second.end());
    //     }
    // }
    #[allow(unused_variables)]
    pub fn remove(&mut self, veh: Ptr<SUMOVehicle>) {
        todo!("port MSVehicleContainer::remove")
    }

    /// C++: `void MSVehicleContainer::add(SUMOTime, const VehicleVector &)`
    /// Adds a container with vehicles departing at the given time
    // void
    // MSVehicleContainer::add(SUMOTime time, const VehicleVector& cont) {
    //     VehicleHeap::iterator j =
    //         find_if(array.begin() + 1, array.begin() + currentSize + 1,
    //                 DepartFinder(time));
    //     if (currentSize == 0 || j == array.begin() + currentSize + 1) {
    //         VehicleDepartureVector newElem(time,
    //                                        VehicleVector(cont));
    //         addReplacing(newElem);
    //     } else {
    //         VehicleVector& stored = (*j).second;
    //         stored.reserve(stored.size() + cont.size());
    //         copy(cont.begin(), cont.end(), back_inserter(stored));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_2(&mut self, time: SUMOTime, cont: &MSVehicleContainer_VehicleVector) {
        let finder = MSVehicleContainer_DepartFinder::new(time);
        let j = if self.currentSize == 0 {
            None
        } else {
            self.array[1..=self.currentSize as usize].iter().position(|e| finder.call_op(e))
        };
        match j {
            None => self.add_replacing(&(time, cont.clone())),
            Some(k) => {
                let stored = &mut self.array[k + 1].1;
                stored.reserve(stored.len() + cont.len());
                stored.extend(cont.iter().cloned());
            }
        }
    }

    /// C++: `bool MSVehicleContainer::anyWaitingBefore(SUMOTime) const` [const]
    /// Returns the information whether any vehicles want to depart before the given time
    // bool
    // MSVehicleContainer::anyWaitingBefore(SUMOTime time) const {
    //     return !isEmpty() && topTime() <= time;
    // }
    #[allow(unused_variables)]
    pub fn any_waiting_before(&self, time: SUMOTime) -> bool {
        !self.is_empty() && self.top_time() <= time
    }

    /// C++: `const VehicleVector & MSVehicleContainer::top()`
    /// Returns the uppermost vehicle vector
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn top(&mut self) -> &MSVehicleContainer_VehicleVector {
        &self.array[1].1
    }

    /// C++: `SUMOTime MSVehicleContainer::topTime() const` [const]
    /// Returns the time the uppermost vehicle vector is assigned to
    // SUMOTime
    // MSVehicleContainer::topTime() const {
    //     if (isEmpty()) {
    //         throw 1;    //!!!Underflow( );
    //     }
    //     assert(array.size() > 1);
    //     return array[ 1 ].first;
    // }
    #[allow(unused_variables)]
    pub fn top_time(&self) -> SUMOTime {
        if self.is_empty() {
            panic!("MSVehicleContainer::topTime: underflow"); // C++: throw 1
        }
        assert!(self.array.len() > 1);
        self.array[1].0
    }

    /// C++: `void MSVehicleContainer::pop()`
    /// Removes the uppermost vehicle vector
    // void
    // MSVehicleContainer::pop()
    //
    // {
    //     if (isEmpty()) {
    //         throw 1;    //!!!Underflow( );
    //     }
    //
    //     assert(array.size() > 1);
    //     array[ 1 ] = array[ currentSize-- ];
    //     percolateDown(1);
    // }
    #[allow(unused_variables)]
    pub fn pop(&mut self) {
        if self.is_empty() {
            panic!("MSVehicleContainer::pop: underflow"); // C++: throw 1
        }
        assert!(self.array.len() > 1);
        self.array[1] = self.array[self.currentSize as usize].clone();
        self.currentSize -= 1;
        self.percolate_down(1);
    }

    /// C++: `bool MSVehicleContainer::isEmpty() const` [const]
    /// Returns the information whether the container is empty
    // bool
    // MSVehicleContainer::isEmpty() const {
    //     return currentSize == 0;
    // }
    #[allow(unused_variables)]
    pub fn is_empty(&self) -> bool {
        self.currentSize == 0
    }

    /// C++: `int MSVehicleContainer::size() const` [const]
    /// Returns the size of the container
    // int
    // MSVehicleContainer::size() const {
    //     return currentSize;
    // }
    #[allow(unused_variables)]
    pub fn size(&self) -> i32 {
        self.currentSize
    }

    /// C++: `void MSVehicleContainer::showArray() const` [const]
    /// Prints the container (the departure times)
    // void
    // MSVehicleContainer::showArray() const {
    //     for (VehicleHeap::const_iterator i = array.begin() + 1; i != array.begin() + currentSize + 1; ++i) {
    //         if (i != array.begin() + 1) {
    //             std::cout << ", ";
    //         }
    //         std::cout << (*i).first;
    //     }
    //     std::cout << std::endl << "-------------------------" << std::endl;
    // }
    #[allow(unused_variables)]
    pub fn show_array(&self) {
        for i in 1..=self.currentSize as usize {
            if i != 1 {
                print!(", ");
            }
            print!("{}", self.array[i].0);
        }
        println!();
        println!("-------------------------");
    }

    /// C++: `void MSVehicleContainer::clearState()`
    /// Remove all vehicles before quick-loading state
    // void
    // MSVehicleContainer::clearState() {
    //     for (VehicleDepartureVector& vdv : array) {
    //         vdv.first = 0;
    //         vdv.second.clear();
    //     }
    //     currentSize = 0;
    // }
    #[allow(unused_variables)]
    pub fn clear_state(&mut self) {
        for vdv in self.array.iter_mut() {
            vdv.0 = 0;
            vdv.1.clear();
        }
        self.currentSize = 0;
    }

    /// C++: `void MSVehicleContainer::addReplacing(const VehicleDepartureVector &)` [private]
    /// Replaces the existing single departure time vector by the one given
    // void
    // MSVehicleContainer::addReplacing(const VehicleDepartureVector& x) {
    //     if (isFull()) {
    //         std::vector<VehicleDepartureVector> array2((array.size() - 1) * 2 + 1, VehicleDepartureVector());
    //         for (int i = (int)array.size(); i-- > 0;) {
    //             assert(i < (int)array2.size());
    //             array2[i] = array[i];
    //         }
    //         array = array2;
    //     }
    //
    //     // Percolate up
    //     int hole = ++currentSize;
    //     for (; hole > 1 && (x.first < array[ hole / 2 ].first); hole /= 2) {
    //         assert((int)array.size() > hole);
    //         array[hole] = array[ hole / 2 ];
    //     }
    //     assert((int)array.size() > hole);
    //     array[hole] = x;
    // }
    #[allow(unused_variables)]
    pub fn add_replacing(&mut self, cont: &MSVehicleContainer_VehicleDepartureVector) {
        if self.is_full() {
            let mut array2: MSVehicleContainer_VehicleHeap = (0..(self.array.len() - 1) * 2 + 1).map(|_| (0, MSVehicleContainer_VehicleVector::new())).collect();
            for i in (0..self.array.len()).rev() {
                array2[i] = self.array[i].clone();
            }
            self.array = array2;
        }

        // Percolate up
        self.currentSize += 1;
        let mut hole = self.currentSize;
        while hole > 1 && cont.0 < self.array[(hole / 2) as usize].0 {
            self.array[hole as usize] = self.array[(hole / 2) as usize].clone();
            hole /= 2;
        }
        self.array[hole as usize] = cont.clone();
    }

    /// C++: `bool MSVehicleContainer::isFull() const` [const, private]
    /// Returns the information whether the container must be extended
    // bool
    // MSVehicleContainer::isFull() const {
    //     return currentSize >= ((int) array.size()) - 1;
    // }
    #[allow(unused_variables)]
    pub fn is_full(&self) -> bool {
        self.currentSize >= (self.array.len() as i32) - 1
    }

    /// C++: `void MSVehicleContainer::percolateDown(int)` [private]
    /// Moves the elements down
    // void
    // MSVehicleContainer::percolateDown(int hole) {
    //     int child;
    //     assert((int)array.size() > hole);
    //     VehicleDepartureVector tmp = array[ hole ];
    //
    //     for (; hole * 2 <= currentSize; hole = child) {
    //         child = hole * 2;
    //         if (child != currentSize && (array[child + 1].first < array[child].first)) {
    //             child++;
    //         }
    //         if ((array[ child ].first < tmp.first)) {
    //             assert((int)array.size() > hole);
    //             array[hole] = array[child];
    //         } else {
    //             break;
    //         }
    //     }
    //     assert((int)array.size() > hole);
    //     array[hole] = tmp;
    // }
    #[allow(unused_variables)]
    pub fn percolate_down(&mut self, hole: i32) {
        let mut hole = hole;
        let tmp = self.array[hole as usize].clone();

        while hole * 2 <= self.currentSize {
            let mut child = hole * 2;
            if child != self.currentSize && self.array[(child + 1) as usize].0 < self.array[child as usize].0 {
                child += 1;
            }
            if self.array[child as usize].0 < tmp.0 {
                self.array[hole as usize] = self.array[child as usize].clone();
            } else {
                break;
            }
            hole = child;
        }
        self.array[hole as usize] = tmp;
    }

}

/// Sort-criterion for vehicle departure lists
/// C++ class `MSVehicleContainer::VehicleDepartureVectorSortCrit` (microsim/MSVehicleContainer.h)
pub struct MSVehicleContainer_VehicleDepartureVectorSortCrit {
}

impl MSVehicleContainer_VehicleDepartureVectorSortCrit {
    /// C++: `bool VehicleDepartureVectorSortCrit::operator()(const VehicleDepartureVector &, const VehicleDepartureVector &) const` [const]
    /// comparison operator
    // bool
    // MSVehicleContainer::VehicleDepartureVectorSortCrit::operator()
    // (const VehicleDepartureVector& e1, const VehicleDepartureVector& e2) const {
    //     return e1.first < e2.first;
    // }
    #[allow(unused_variables)]
    pub fn call_op(&self, e1: &MSVehicleContainer_VehicleDepartureVector, e2: &MSVehicleContainer_VehicleDepartureVector) -> bool {
        e1.0 < e2.0
    }

}

/// C++: `std::ostream &(std::ostream &, MSVehicleContainer &) operator<<`
// std::ostream& operator << (std::ostream& strm, MSVehicleContainer& cont) {
//     strm << "------------------------------------" << std::endl;
//     while (!cont.isEmpty()) {
//         const MSVehicleContainer::VehicleVector& v = cont.top();
//         for (MSVehicleContainer::VehicleVector::const_iterator i = v.begin(); i != v.end(); ++i) {
//             strm << (*i)->getParameter().depart << std::endl;
//         }
//         cont.pop();
//     }
//     return strm;
// }
#[allow(unused_variables)]
pub fn shl_op(strm: &mut Unmapped /* std::ostream */, cont: &mut MSVehicleContainer) -> &'static mut Unmapped /* std::ostream */ {
    todo!("port operator<<")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heap_pops_in_departure_order_across_growth() {
        // capacity 2 forces addReplacing to grow the heap array twice
        let mut c = MSVehicleContainer::new(2);
        for t in [30, 10, 20, 40, 5] {
            c.add_replacing(&(t, MSVehicleContainer_VehicleVector::new()));
        }
        assert_eq!(c.size(), 5);
        let mut out = Vec::new();
        while !c.is_empty() {
            out.push(c.top_time());
            c.pop();
        }
        assert_eq!(out, vec![5, 10, 20, 30, 40]);
    }
}
