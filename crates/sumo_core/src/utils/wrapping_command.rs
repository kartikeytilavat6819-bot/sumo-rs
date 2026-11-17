//! Port of utils/common/WrappingCommand.h

use crate::microsim::ms_event_control::Command;
use crate::prelude::*;

/// C++ class `WrappingCommand<T>` (utils/common/WrappingCommand.h): a Command that calls a method of a receiver
pub struct WrappingCommand<T> {
    /// C++ `T* myReceiver`
    pub myReceiver: Ptr<T>,
    /// C++ `Operation myOperation` (`SUMOTime (T::*)(SUMOTime)`)
    pub myOperation: fn(&mut T, SUMOTime) -> SUMOTime,
    /// C++ `bool myAmDescheduledByParent`
    pub myAmDescheduledByParent: bool,
    /// C++ `Command::priority`
    pub myPriority: i32,
}

impl<T> WrappingCommand<T> {
    /// C++: `WrappingCommand(T* receiver, Operation operation)`
    pub fn new(receiver: Ptr<T>, operation: fn(&mut T, SUMOTime) -> SUMOTime) -> Self {
        WrappingCommand {
            myReceiver: receiver,
            myOperation: operation,
            myAmDescheduledByParent: false,
            myPriority: 0,
        }
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

impl<T: 'static> Command for WrappingCommand<T> {
    /// C++: `SUMOTime execute(SUMOTime currentTime)`
    fn execute(&mut self, currentTime: SUMOTime) -> SUMOTime {
        // do not execute if the command was descheduled
        if self.myAmDescheduledByParent {
            return 0;
        }
        // execute if stil valid
        (self.myOperation)(self.myReceiver.m(), currentTime)
    }

    fn priority(&self) -> i32 {
        self.myPriority
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bump(v: &mut i64, t: SUMOTime) -> SUMOTime {
        *v += 1;
        t + 5
    }

    #[test]
    fn execute_calls_operation_until_descheduled() {
        let mut target: Box<i64> = Box::new(0);
        let mut cmd = WrappingCommand::new(Ptr::from_mut(&mut *target), bump);
        assert_eq!(cmd.execute(10), 15);
        cmd.deschedule();
        assert!(cmd.is_descheduled());
        assert_eq!(cmd.execute(20), 0);
        assert_eq!(*target, 1);
    }
}
