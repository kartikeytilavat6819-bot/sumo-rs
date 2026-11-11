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
        todo!("port DepartFinder::DepartFinder")
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
        todo!("port DepartFinder::operator()")
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
        todo!("port MSVehicleContainer::MSVehicleContainer")
    }

    /// C++: `void MSVehicleContainer::~MSVehicleContainer()`
    /// Destructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MSVehicleContainer::~MSVehicleContainer")
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
        todo!("port MSVehicleContainer::add")
    }

    /// C++: `bool MSVehicleContainer::anyWaitingBefore(SUMOTime) const` [const]
    /// Returns the information whether any vehicles want to depart before the given time
    // bool
    // MSVehicleContainer::anyWaitingBefore(SUMOTime time) const {
    //     return !isEmpty() && topTime() <= time;
    // }
    #[allow(unused_variables)]
    pub fn any_waiting_before(&self, time: SUMOTime) -> bool {
        todo!("port MSVehicleContainer::anyWaitingBefore")
    }

    /// C++: `const VehicleVector & MSVehicleContainer::top()`
    /// Returns the uppermost vehicle vector
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn top(&mut self) -> &MSVehicleContainer_VehicleVector {
        todo!("port MSVehicleContainer::top")
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
        todo!("port MSVehicleContainer::topTime")
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
        todo!("port MSVehicleContainer::pop")
    }

    /// C++: `bool MSVehicleContainer::isEmpty() const` [const]
    /// Returns the information whether the container is empty
    // bool
    // MSVehicleContainer::isEmpty() const {
    //     return currentSize == 0;
    // }
    #[allow(unused_variables)]
    pub fn is_empty(&self) -> bool {
        todo!("port MSVehicleContainer::isEmpty")
    }

    /// C++: `int MSVehicleContainer::size() const` [const]
    /// Returns the size of the container
    // int
    // MSVehicleContainer::size() const {
    //     return currentSize;
    // }
    #[allow(unused_variables)]
    pub fn size(&self) -> i32 {
        todo!("port MSVehicleContainer::size")
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
        todo!("port MSVehicleContainer::showArray")
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
        todo!("port MSVehicleContainer::clearState")
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
        todo!("port MSVehicleContainer::addReplacing")
    }

    /// C++: `bool MSVehicleContainer::isFull() const` [const, private]
    /// Returns the information whether the container must be extended
    // bool
    // MSVehicleContainer::isFull() const {
    //     return currentSize >= ((int) array.size()) - 1;
    // }
    #[allow(unused_variables)]
    pub fn is_full(&self) -> bool {
        todo!("port MSVehicleContainer::isFull")
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
        todo!("port MSVehicleContainer::percolateDown")
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
        todo!("port VehicleDepartureVectorSortCrit::operator()")
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

