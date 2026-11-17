//! Port of utils/common/ParametrisedWrappingCommand.h

use crate::microsim::ms_event_control::Command;
use crate::prelude::*;

/// C++ class `ParametrisedWrappingCommand<T, S>` (utils/common/ParametrisedWrappingCommand.h):
/// a Command that calls a method of a receiver together with a stored parameter
pub struct ParametrisedWrappingCommand<T, S> {
    /// C++ `T* myReceiver`
    pub myReceiver: Ptr<T>,
    /// C++ `S myParameter`
    pub myParameter: S,
    /// C++ `Operation myOperation` (`SUMOTime (T::*)(SUMOTime, S)`)
    pub myOperation: fn(&mut T, SUMOTime, S) -> SUMOTime,
    /// C++ `bool myAmDescheduledByParent`
    pub myAmDescheduledByParent: bool,
}

impl<T, S> ParametrisedWrappingCommand<T, S> {
    /// C++: `ParametrisedWrappingCommand(T* receiver, const S& param, Operation operation)`
    pub fn new(receiver: Ptr<T>, param: S, operation: fn(&mut T, SUMOTime, S) -> SUMOTime) -> Self {
        ParametrisedWrappingCommand { myReceiver: receiver, myParameter: param, myOperation: operation, myAmDescheduledByParent: false }
    }

    /// C++: `void deschedule()`
    pub fn deschedule(&mut self) {
        self.myAmDescheduledByParent = true;
    }

    /// C++: `bool isDescheduled()`
    pub fn is_descheduled(&self) -> bool {
        self.myAmDescheduledByParent
    }
}

impl<T: 'static, S: Clone + 'static> Command for ParametrisedWrappingCommand<T, S> {
    /// C++: `SUMOTime execute(SUMOTime currentTime)`
    fn execute(&mut self, currentTime: SUMOTime) -> SUMOTime {
        // do not execute if the command was descheduled
        if self.myAmDescheduledByParent {
            return 0;
        }
        // execute if stil valid
        (self.myOperation)(self.myReceiver.m(), currentTime, self.myParameter.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn add(v: &mut i64, t: SUMOTime, p: i64) -> SUMOTime {
        *v += p;
        t + 5
    }

    #[test]
    fn execute_passes_parameter_until_descheduled() {
        let mut target: Box<i64> = Box::new(0);
        let mut cmd = ParametrisedWrappingCommand::new(Ptr::from_mut(&mut *target), 3, add);
        assert_eq!(cmd.execute(10), 15);
        assert_eq!(*target, 3);
        cmd.deschedule();
        assert!(cmd.is_descheduled());
        assert_eq!(cmd.execute(20), 0);
        assert_eq!(*target, 3);
    }
}
