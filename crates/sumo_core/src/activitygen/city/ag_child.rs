//! Port of activitygen/city/AGChild.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `AGChild` (activitygen/city/AGChild.h)
pub struct AGChild {
    /// C++ base class `AGPerson`
    pub base: AGPerson,
    pub mySchool: Ptr<AGSchool>,
}

impl std::ops::Deref for AGChild {
    type Target = AGPerson;
    fn deref(&self) -> &AGPerson {
        &self.base
    }
}
impl std::ops::DerefMut for AGChild {
    fn deref_mut(&mut self) -> &mut AGPerson {
        &mut self.base
    }
}

impl AGChild {
    /// C++: `void AGChild::AGChild(int)`
    // AGChild(int age) :
    //         AGPerson(age),
    //         mySchool(nullptr) {}
    #[allow(unused_variables)]
    pub fn new(age: i32) -> Self {
        todo!("port AGChild::AGChild")
    }

    /// C++: `void AGChild::print() const` [const, virtual]
    // void
    // AGChild::print() const {
    //     std::cout << "- Child: Age=" << age << " School=" << mySchool << std::endl;
    // }
    #[allow(unused_variables)]
    pub fn print(&self) {
        todo!("port AGChild::print")
    }

    /// C++: `bool AGChild::setSchool(AGSchool *)`
    // bool
    // AGChild::setSchool(AGSchool* school) {
    //     if (school == nullptr) {
    //         return false;
    //     }
    //     bool enoughPlace = school->addNewChild();
    //     if (enoughPlace) {
    //         mySchool = school;
    //     }
    //     return enoughPlace;
    // }
    #[allow(unused_variables)]
    pub fn set_school(&mut self, school: Ptr<AGSchool>) -> bool {
        todo!("port AGChild::setSchool")
    }

    /// C++: `bool AGChild::allocateASchool(std::list<AGSchool> *, AGPosition)`
    /// Returns if a school was found corresponding to the child's age.
    // bool
    // AGChild::allocateASchool(std::list<AGSchool>* schools, AGPosition housePos) {
    //     double minDist = std::numeric_limits<double>::infinity();
    //     AGSchool* sch = nullptr;
    //     if (schools->size() == 0) {
    //         return false;
    //     }
    //     std::list<AGSchool>::iterator it;
    //
    //     for (it = schools->begin(); it != schools->end(); ++it) {
    //         if (it->acceptThisAge(age) && it->getPlaces() > 0 && housePos.distanceTo(it->getPosition()) < minDist) {
    //             minDist = housePos.distanceTo(it->getPosition());
    //             sch = &(*it);
    //         }
    //     }
    //     return setSchool(sch);
    // }
    #[allow(unused_variables)]
    pub fn allocate_a_school(&mut self, schools: Ptr<VecDeque<AGSchool>>, housePos: AGPosition) -> bool {
        todo!("port AGChild::allocateASchool")
    }

    /// C++: `bool AGChild::leaveSchool()`
    /// Returns if the child is now without any school
    // bool
    // AGChild::leaveSchool() {
    //     if (mySchool != nullptr)
    //         if (!mySchool->removeChild()) {
    //             return false;
    //         }
    //     mySchool = nullptr;
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn leave_school(&mut self) -> bool {
        todo!("port AGChild::leaveSchool")
    }

    /// C++: `bool AGChild::haveASchool() const` [const]
    // bool
    // AGChild::haveASchool() const {
    //     return (mySchool != nullptr);
    // }
    #[allow(unused_variables)]
    pub fn have_a_school(&self) -> bool {
        todo!("port AGChild::haveASchool")
    }

    /// C++: `AGPosition AGChild::getSchoolLocation() const` [const]
    // AGPosition
    // AGChild::getSchoolLocation() const {
    //     return mySchool->getPosition();
    // }
    #[allow(unused_variables)]
    pub fn get_school_location(&self) -> AGPosition {
        todo!("port AGChild::getSchoolLocation")
    }

    /// C++: `int AGChild::getSchoolOpening() const` [const]
    // int
    // AGChild::getSchoolOpening() const {
    //     return mySchool->getOpeningHour();
    // }
    #[allow(unused_variables)]
    pub fn get_school_opening(&self) -> i32 {
        todo!("port AGChild::getSchoolOpening")
    }

    /// C++: `int AGChild::getSchoolClosing() const` [const]
    // int
    // AGChild::getSchoolClosing() const {
    //     return mySchool->getClosingHour();
    // }
    #[allow(unused_variables)]
    pub fn get_school_closing(&self) -> i32 {
        todo!("port AGChild::getSchoolClosing")
    }

}

