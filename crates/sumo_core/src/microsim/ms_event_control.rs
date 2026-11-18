//! Port of microsim/MSEventControl.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ base class `Command` (utils/common/Command.h). The base has no port yet, so its virtual
/// methods are a trait here. shortcut: move to the Command port once it exists, then drop this trait.
pub trait Command {
    /// C++ `virtual SUMOTime execute(SUMOTime currentTime) = 0`
    fn execute(&mut self, currentTime: SUMOTime) -> SUMOTime;
    /// C++ `virtual SUMOTime shiftTime(SUMOTime, SUMOTime, SUMOTime)`, default returns -1
    fn shift_time(&mut self, currentTime: SUMOTime, execTime: SUMOTime, newTime: SUMOTime) -> SUMOTime {
        -1
    }
    /// C++ field `int priority = 0`
    fn priority(&self) -> i32 {
        0
    }
}

/// C++ typedef `std::pair<Command *, SUMOTime> MSEventControl::Event`
pub type MSEventControl_Event = (Ptr<dyn Command>, SUMOTime);

/// C++ `std::push_heap` (libstdc++ __push_heap) on a max-heap ordered by `eventCompare`.
/// Same algorithm as libstdc++ so that ties come out in the same order as C++.
// determinism: heap tie order must match libstdc++, so the sift logic is copied, not replaced by BinaryHeap.
fn heap_push_hole(v: &mut [MSEventControl_Event], mut hole: isize, top: isize, value: MSEventControl_Event) {
    let mut parent = (hole - 1) / 2;
    while hole > top && MSEventControl::event_compare(&v[parent as usize], &value) {
        v[hole as usize] = v[parent as usize];
        hole = parent;
        parent = (hole - 1) / 2;
    }
    v[hole as usize] = value;
}

/// libstdc++ __adjust_heap
fn heap_adjust(v: &mut [MSEventControl_Event], mut hole: isize, len: isize, value: MSEventControl_Event) {
    let top = hole;
    let mut second = hole;
    while second < (len - 1) / 2 {
        second = 2 * (second + 1);
        if MSEventControl::event_compare(&v[second as usize], &v[(second - 1) as usize]) {
            second -= 1;
        }
        v[hole as usize] = v[second as usize];
        hole = second;
    }
    if (len & 1) == 0 && second == (len - 2) / 2 {
        second = 2 * (second + 1);
        v[hole as usize] = v[(second - 1) as usize];
        hole = second - 1;
    }
    heap_push_hole(v, hole, top, value);
}

/// C++ `std::push_heap(begin, end, eventCompare)` after `emplace_back`
fn heap_push(v: &mut [MSEventControl_Event]) {
    let n = v.len() as isize;
    let value = v[(n - 1) as usize];
    heap_push_hole(v, n - 1, 0, value);
}

/// C++ `std::pop_heap(begin, end, eventCompare)` followed by `pop_back()`
fn heap_pop(v: &mut Vec<MSEventControl_Event>) {
    let n = v.len();
    let value = v[n - 1];
    v[n - 1] = v[0];
    heap_adjust(v, 0, (n - 1) as isize, value);
    v.pop();
}

/// C++ `std::make_heap(begin, end, eventCompare)` (libstdc++ algorithm)
fn heap_make(v: &mut [MSEventControl_Event]) {
    let len = v.len() as isize;
    if len < 2 {
        return;
    }
    let mut parent = (len - 2) / 2;
    loop {
        let value = v[parent as usize];
        heap_adjust(v, parent, len, value);
        if parent == 0 {
            return;
        }
        parent -= 1;
    }
}

/// C++ `delete command`: the event queue owns the commands it holds.
fn delete_command(cmd: Ptr<dyn Command>) {
    drop(unsafe { Box::from_raw(cmd.as_raw()) });
}

/// Stores time-dependant events and executes them at the proper time
/// C++ class `MSEventControl` (microsim/MSEventControl.h)
pub struct MSEventControl {
    /// Event-container, holds executable events.
    pub myEvents: Vec<MSEventControl_Event>,
}

impl Drop for MSEventControl {
    fn drop(&mut self) {
        self.drop_();
    }
}

impl MSEventControl {
    /// C++: `void MSEventControl::MSEventControl()`
    /// Default constructor.
    // MSEventControl::MSEventControl() :
    //     myEvents() {}
    #[allow(unused_variables)]
    pub fn new() -> Self {
        MSEventControl { myEvents: Vec::new() }
    }

    /// C++: `void MSEventControl::~MSEventControl()` [virtual]
    /// Destructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ `delete e.first` for every queued event
        for e in self.myEvents.drain(..) {
            delete_command(e.0);
        }
    }

    /// C++: `void MSEventControl::addEvent(Command *, SUMOTime)` [virtual]
    /// Adds an Event.
    // void
    // MSEventControl::addEvent(Command* operation, SUMOTime execTimeStep) {
    //     myEvents.emplace_back(Event(operation, execTimeStep));
    //     std::push_heap(myEvents.begin(), myEvents.end(), MSEventControl::eventCompare);
    // }
    #[allow(unused_variables)]
    pub fn add_event(&mut self, operation: Ptr<dyn Command>, execTimeStep: SUMOTime) {
        self.myEvents.push((operation, execTimeStep));
        heap_push(&mut self.myEvents);
    }

    /// C++: `void MSEventControl::execute(SUMOTime)` [virtual]
    /// Executes time-dependant commands
    // void
    // MSEventControl::execute(SUMOTime execTime) {
    //     // Execute all events that are scheduled for execTime.
    //     while (!myEvents.empty()) {
    //         Event currEvent = myEvents.front();
    //         if (currEvent.second < 0) {
    //             currEvent.second = execTime;
    //         }
    //         if (currEvent.second < execTime + DELTA_T) {
    //             Command* command = currEvent.first;
    //             std::pop_heap(myEvents.begin(), myEvents.end(), eventCompare);
    //             myEvents.pop_back();
    //             SUMOTime time = 0;
    //             try {
    //                 time = command->execute(execTime);
    //             } catch (...) {
    //                 delete command;
    //                 throw;
    //             }
    //
    //             // Delete nonrecurring events, reinsert recurring ones
    //             // with new execution time = execTime + returned offset.
    //             if (time <= 0) {
    //                 if (time < 0) {
    //                     WRITE_WARNING("Command returned negative repeat number; will be deleted.");
    //                 }
    //                 delete currEvent.first;
    //             } else {
    //                 addEvent(currEvent.first, currEvent.second + time);
    //             }
    //         } else {
    //             break;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn execute(&mut self, execTime: SUMOTime) {
        // Execute all events that are scheduled for execTime.
        while !self.myEvents.is_empty() {
            let mut currEvent = self.myEvents[0];
            if currEvent.1 < 0 {
                currEvent.1 = execTime;
            }
            if currEvent.1 < execTime + utils::common::time::delta_t() {
                let command = currEvent.0;
                heap_pop(&mut self.myEvents);
                // shortcut: C++ wraps this in try/catch to delete the command and rethrow a ProcessError;
                // Command::execute has no error channel yet, so errors are not rethrown here.
                let time = command.m().execute(execTime);

                // Delete nonrecurring events, reinsert recurring ones
                // with new execution time = execTime + returned offset.
                if time <= 0 {
                    if time < 0 {
                        utils::common::msg_handler::write_warning("Command returned negative repeat number; will be deleted.");
                    }
                    delete_command(currEvent.0);
                } else {
                    self.add_event(currEvent.0, currEvent.1 + time);
                }
            } else {
                break;
            }
        }
    }

    /// C++: `bool MSEventControl::isEmpty()`
    /// Returns whether events are in the que.
    // bool
    // MSEventControl::isEmpty() {
    //     return myEvents.empty();
    // }
    #[allow(unused_variables)]
    pub fn is_empty(&mut self) -> bool {
        self.myEvents.is_empty()
    }

    /// C++: `void MSEventControl::clearState(SUMOTime, SUMOTime)`
    /// Remove all events before quick-loading state
    // void
    // MSEventControl::clearState(SUMOTime currentTime, SUMOTime newTime) {
    //     for (auto eventIt = myEvents.begin(); eventIt != myEvents.end();) {
    //         eventIt->second = eventIt->first->shiftTime(currentTime, eventIt->second, newTime);
    //         if (eventIt->second >= 0) {
    //             ++eventIt;
    //         } else {
    //             delete eventIt->first;
    //             eventIt = myEvents.erase(eventIt);
    //         }
    //     }
    //     std::make_heap(myEvents.begin(), myEvents.end(), eventCompare);
    // }
    #[allow(unused_variables)]
    pub fn clear_state(&mut self, currentTime: SUMOTime, newTime: SUMOTime) {
        let mut i = 0;
        while i < self.myEvents.len() {
            let t = self.myEvents[i].0.m().shift_time(currentTime, self.myEvents[i].1, newTime);
            self.myEvents[i].1 = t;
            if t >= 0 {
                i += 1;
            } else {
                delete_command(self.myEvents[i].0);
                self.myEvents.remove(i);
            }
        }
        heap_make(&mut self.myEvents);
    }

    /// C++: `SUMOTime MSEventControl::getEventTime(Command *) const` [const]
    /// get the next scheduled event time for the given command, -2 if it is not scheduled
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_event_time(&self, cmd: Ptr<dyn Command>) -> SUMOTime {
        // C++ declares getEventTime but never defines it; implemented from its doc comment:
        // first queued event for cmd gives its time, otherwise -2.
        for e in &self.myEvents {
            if e.0.as_raw() as *const () == cmd.as_raw() as *const () {
                return e.1;
            }
        }
        -2
    }

    /// C++: `bool MSEventControl::eventCompare(const Event &, const Event &)` [static, protected]
    /// compares two events
    // bool
    // MSEventControl::eventCompare(const Event& e1, const Event& e2) {
    //     return e1.second == e2.second ? e1.first->priority < e2.first->priority : e1.second > e2.second;
    // }
    #[allow(unused_variables)]
    pub fn event_compare(e1: &MSEventControl_Event, e2: &MSEventControl_Event) -> bool {
        if e1.1 == e2.1 {
            e1.0.priority() < e2.0.priority()
        } else {
            e1.1 > e2.1
        }
    }

    /// C++: `void MSEventControl::MSEventControl(const MSEventControl &)` [private]
    /// invalid copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSEventControl) -> Self {
        panic!("MSEventControl copy constructor is private and undefined in C++; it cannot be called")
    }

    /// C++: `MSEventControl & MSEventControl::operator=(const MSEventControl &)` [private]
    /// invalid assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSEventControl) -> &mut MSEventControl {
        panic!("MSEventControl assignment operator is private and undefined in C++; it cannot be called")
    }

}

