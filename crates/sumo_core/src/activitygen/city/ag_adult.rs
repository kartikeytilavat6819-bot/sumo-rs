//! Port of activitygen/city/AGAdult.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// An adult person who can have a job.
/// C++ class `AGAdult` (activitygen/city/AGAdult.h)
pub struct AGAdult {
    /// C++ base class `AGPerson`
    pub base: AGPerson,
    /// The work position of this adult.
    pub work: Ptr<AGWorkPosition>,
}

impl std::ops::Deref for AGAdult {
    type Target = AGPerson;
    fn deref(&self) -> &AGPerson {
        &self.base
    }
}
impl std::ops::DerefMut for AGAdult {
    fn deref_mut(&mut self) -> &mut AGPerson {
        &mut self.base
    }
}

impl AGAdult {
    /// C++: `void AGAdult::AGAdult(int)`
    /// Initialises the base class and the own attributes.
    // AGAdult::AGAdult(int age)
    //     : AGPerson(age), work(nullptr) {}
    #[allow(unused_variables)]
    pub fn new(age: i32) -> Self {
        todo!("port AGAdult::AGAdult")
    }

    /// C++: `void AGAdult::print() const` [const, virtual]
    /// Puts out a summary of the attributes.
    // void
    // AGAdult::print() const {
    //     std::cout << "- AGAdult: Age=" << age << " Work=" << work << std::endl;
    // }
    #[allow(unused_variables)]
    pub fn print(&self) {
        todo!("port AGAdult::print")
    }

    /// C++: `bool AGAdult::isWorking() const` [const]
    /// States whether this person occupies a work position at present.
    // bool
    // AGAdult::isWorking() const {
    //     return (work != nullptr);
    // }
    #[allow(unused_variables)]
    pub fn is_working(&self) -> bool {
        todo!("port AGAdult::isWorking")
    }

    /// C++: `void AGAdult::tryToWork(double, std::vector<AGWorkPosition> *)`
    /// Tries to get a new work position.
    // void
    // AGAdult::tryToWork(double rate, std::vector<AGWorkPosition>* wps) {
    //     if (decide(rate)) {
    //         // Select the new work position before giving up the current one.
    //         // This avoids that the current one is the same as the new one.
    //         AGWorkPosition* newWork = randomFreeWorkPosition(wps);
    //
    //         if (work != nullptr) {
    //             work->let();
    //         }
    //         work = newWork;
    //         if (work != nullptr) {
    //             work->take(this);
    //         }
    //     } else {
    //         if (work != nullptr) {
    //             // Also sets work = 0 with the call back lostWorkPosition
    //             work->let();
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn try_to_work(&mut self, employmentRate: f64, wps: Ptr<Vec<AGWorkPosition>>) {
        todo!("port AGAdult::tryToWork")
    }

    /// C++: `void AGAdult::lostWorkPosition()`
    /// Called when the adult has lost her job.
    // void
    // AGAdult::lostWorkPosition() {
    //     work = nullptr;
    // }
    #[allow(unused_variables)]
    pub fn lost_work_position(&mut self) {
        todo!("port AGAdult::lostWorkPosition")
    }

    /// C++: `void AGAdult::resignFromWorkPosition()`
    /// Called when the adult should resign her job.
    // void
    // AGAdult::resignFromWorkPosition() {
    //     if (work != nullptr) {
    //         work->let();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn resign_from_work_position(&mut self) {
        todo!("port AGAdult::resignFromWorkPosition")
    }

    /// C++: `const AGWorkPosition & AGAdult::getWorkPosition() const` [const]
    /// Provides the work position of the adult.
    // const AGWorkPosition&
    // AGAdult::getWorkPosition() const {
    //     if (work != nullptr) {
    //         return *work;
    //     }
    //     throw ProcessError("AGAdult::getWorkPosition: Adult is unemployed.");
    // }
    #[allow(unused_variables)]
    pub fn get_work_position(&self) -> &AGWorkPosition {
        todo!("port AGAdult::getWorkPosition")
    }

    /// C++: `AGWorkPosition * AGAdult::randomFreeWorkPosition(std::vector<AGWorkPosition> *)` [static, private]
    /// Randomly selects a free work position from the list.
    // AGWorkPosition*
    // AGAdult::randomFreeWorkPosition(std::vector<AGWorkPosition>* wps) {
    //     std::vector<AGWorkPosition*> freePos;
    //     for (std::vector<AGWorkPosition>::iterator i = wps->begin(); i != wps->end(); ++i) {
    //         if (!i->isTaken()) {
    //             freePos.push_back(&*i);
    //         }
    //     }
    //     if (freePos.empty()) {
    //         return nullptr;
    //     }
    //     return RandHelper::getRandomFrom(freePos);
    // }
    #[allow(unused_variables)]
    pub fn random_free_work_position(wps: Ptr<Vec<AGWorkPosition>>) -> Ptr<AGWorkPosition> {
        todo!("port AGAdult::randomFreeWorkPosition")
    }

}

