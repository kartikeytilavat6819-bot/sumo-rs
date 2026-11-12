//! Port of activitygen/city/AGHousehold.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `AGHousehold` (activitygen/city/AGHousehold.h)
pub struct AGHousehold {
    pub myCity: Ptr<AGCity>,
    pub myLocation: AGPosition,
    pub myId: i32,
    pub myAdults: VecDeque<AGAdult>,
    pub myChildren: VecDeque<AGChild>,
    pub myCars: VecDeque<AGCar>,
}

impl AGHousehold {
    /// C++: `void AGHousehold::AGHousehold(AGPosition, AGCity *, int)`
    // AGHousehold(AGPosition pos, AGCity* city, int idHouseholds) :
    //         myCity(city),
    //         myLocation(pos),
    //         myId(idHouseholds) {}
    #[allow(unused_variables)]
    pub fn new(pos: AGPosition, city: Ptr<AGCity>, idHouseholds: i32) -> Self {
        Self {
            myCity: city,
            myLocation: pos,
            myId: idHouseholds,
            myAdults: VecDeque::new(),
            myChildren: VecDeque::new(),
            myCars: VecDeque::new(),
        }
    }

    /// C++: `void AGHousehold::AGHousehold(AGStreet *, AGCity *, int)`
    // AGHousehold(AGStreet* str, AGCity* city, int idHouseholds) :
    //         myCity(city),
    //         myLocation(*str),
    //         myId(idHouseholds) {}
    #[allow(unused_variables)]
    pub fn new_2(str: Ptr<AGStreet>, city: Ptr<AGCity>, idHouseholds: i32) -> Self {
        Self {
            myCity: city,
            myLocation: AGPosition::new_2(&*str),
            myId: idHouseholds,
            myAdults: VecDeque::new(),
            myChildren: VecDeque::new(),
            myCars: VecDeque::new(),
        }
    }

    /// C++: `void AGHousehold::generatePeople(int, int, bool)`
    /// function generating the given number of adults (1 or 2) and possibly children
    // void
    // AGHousehold::generatePeople(int numAdults, int numChilds, bool firstRetired) {
    //     AGDataAndStatistics* ds = &(myCity->statData);
    //     //the first adult
    //     AGAdult pers(ds->getRandomPopDistributed(ds->limitAgeChildren, ds->limitEndAge));
    //     if (firstRetired) {
    //         pers = AGAdult(ds->getRandomPopDistributed(ds->limitAgeRetirement, ds->limitEndAge));
    //     }
    //     myAdults.push_back(pers);
    //     //further adults
    //     while (static_cast<int>(myAdults.size()) < numAdults) {
    //         if (firstRetired) {
    //             AGAdult pers2(ds->getRandomPopDistributed(ds->limitAgeRetirement, ds->limitEndAge));
    //             myAdults.push_back(pers2);
    //         } else {
    //             AGAdult pers2(ds->getRandomPopDistributed(ds->limitAgeChildren, ds->limitAgeRetirement));
    //             myAdults.push_back(pers2);
    //         }
    //     }
    //     //Children
    //     while (static_cast<int>(myChildren.size()) < numChilds) {
    //         AGChild chl(ds->getRandomPopDistributed(0, ds->limitAgeChildren));
    //         myChildren.push_back(chl);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn generate_people(&mut self, numAdults: i32, numChilds: i32, firstRetired: bool) {
        let ds = self.myCity.statData.m();
        //the first adult
        let mut pers = AGAdult::new(ds.get_random_pop_distributed(ds.limitAgeChildren, ds.limitEndAge));
        if firstRetired {
            pers = AGAdult::new(ds.get_random_pop_distributed(ds.limitAgeRetirement, ds.limitEndAge));
        }
        self.myAdults.push_back(pers);
        //further adults
        while (self.myAdults.len() as i32) < numAdults {
            if firstRetired {
                let pers2 = AGAdult::new(ds.get_random_pop_distributed(ds.limitAgeRetirement, ds.limitEndAge));
                self.myAdults.push_back(pers2);
            } else {
                let pers2 = AGAdult::new(ds.get_random_pop_distributed(ds.limitAgeChildren, ds.limitAgeRetirement));
                self.myAdults.push_back(pers2);
            }
        }
        //Children
        while (self.myChildren.len() as i32) < numChilds {
            let chl = AGChild::new(ds.get_random_pop_distributed(0, ds.limitAgeChildren));
            self.myChildren.push_back(chl);
        }
    }

    /// C++: `int AGHousehold::getPeopleNbr()`
    // int
    // AGHousehold::getPeopleNbr() {
    //     return static_cast<int>(myAdults.size() + myChildren.size());
    // }
    #[allow(unused_variables)]
    pub fn get_people_nbr(&mut self) -> i32 {
        (self.myAdults.len() + self.myChildren.len()) as i32
    }

    /// C++: `int AGHousehold::getAdultNbr()`
    // int
    // AGHousehold::getAdultNbr() {
    //     return static_cast<int>(myAdults.size());
    // }
    #[allow(unused_variables)]
    pub fn get_adult_nbr(&mut self) -> i32 {
        self.myAdults.len() as i32
    }

    /// C++: `const std::list<AGAdult> & AGHousehold::getAdults() const` [const]
    // const std::list<AGAdult>&
    // AGHousehold::getAdults() const {
    //     return myAdults;
    // }
    #[allow(unused_variables)]
    pub fn get_adults(&self) -> &VecDeque<AGAdult> {
        &self.myAdults
    }

    /// C++: `const std::list<AGChild> & AGHousehold::getChildren() const` [const]
    // const std::list<AGChild>&
    // AGHousehold::getChildren() const {
    //     return myChildren;
    // }
    #[allow(unused_variables)]
    pub fn get_children(&self) -> &VecDeque<AGChild> {
        &self.myChildren
    }

    /// C++: `const std::list<AGCar> & AGHousehold::getCars() const` [const]
    // const std::list<AGCar>&
    // AGHousehold::getCars() const {
    //     return myCars;
    // }
    #[allow(unused_variables)]
    pub fn get_cars(&self) -> &VecDeque<AGCar> {
        &self.myCars
    }

    /// C++: `bool AGHousehold::isCloseFromPubTransport(std::list<AGPosition> *)`
    /// function returning true if the household is close to the given stations stations
    // bool
    // AGHousehold::isCloseFromPubTransport(std::list<AGPosition>* pubTransport) {
    //     double distToPT = myLocation.minDistanceTo(*pubTransport);
    //     if (distToPT > myCity->statData.maxFootDistance) {
    //         return false;
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn is_close_from_pub_transport(&mut self, pubTransport: Ptr<VecDeque<AGPosition>>) -> bool {
        let distToPT = self.myLocation.min_distance_to(&*pubTransport);
        if distToPT > self.myCity.statData.maxFootDistance {
            return false;
        }
        true
    }

    /// C++: `bool AGHousehold::isCloseFromPubTransport(std::map<int, AGPosition> *)`
    // bool
    // AGHousehold::isCloseFromPubTransport(std::map<int, AGPosition>* pubTransport) {
    //     double distToPT = myLocation.minDistanceTo(*pubTransport);
    //     if (distToPT > myCity->statData.maxFootDistance) {
    //         return false;
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn is_close_from_pub_transport_2(&mut self, pubTransport: Ptr<BTreeMap<i32, AGPosition>>) -> bool {
        let distToPT = self.myLocation.min_distance_to_2(&*pubTransport);
        if distToPT > self.myCity.statData.maxFootDistance {
            return false;
        }
        true
    }

    /// C++: `void AGHousehold::regenerate()`
    /// function regenerating the household: --> work positions and schools are resigned --> cars and people are deleted --> number of people are MAINTAINED --> work positions, schools and cars are reallocated
    // void
    // AGHousehold::regenerate() {
    //     //only allocation of work or school to people will change
    //     std::list<AGChild>::iterator itC;
    //     std::list<AGAdult>::iterator itA;
    //     for (itC = myChildren.begin(); itC != myChildren.end(); ++itC) {
    //         if (itC->haveASchool()) {
    //             if (itC->leaveSchool()) {
    //                 itC->allocateASchool(&(myCity->schools), getPosition());
    //             }
    //         } else {
    //             itC->allocateASchool(&(myCity->schools), getPosition());
    //         }
    //     }
    //     for (itA = myAdults.begin(); itA != myAdults.end(); ++itA) {
    //         if (itA->isWorking()) {
    //             itA->resignFromWorkPosition();
    //         }
    //
    //         if (myCity->statData.workPositions > 0) {
    //             itA->tryToWork(1 - myCity->statData.unemployement, &(myCity->workPositions));
    //
    //         } else {
    //             std::cout << "Not enough work positions in AGHousehold::regenerate. Should not happen!" << std::endl;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn regenerate(&mut self) {
        //only allocation of work or school to people will change
        let city = self.myCity;
        let schools = Ptr::from_mut(&mut city.m().schools);
        let pos = self.myLocation;
        for itC in self.myChildren.iter_mut() {
            if itC.have_a_school() {
                if itC.leave_school() {
                    itC.allocate_a_school(schools, pos);
                }
            } else {
                itC.allocate_a_school(schools, pos);
            }
        }
        for itA in self.myAdults.iter_mut() {
            if itA.is_working() {
                itA.resign_from_work_position();
            }

            if city.statData.workPositions > 0 {
                itA.try_to_work(1.0 - city.statData.unemployement, Ptr::from_mut(&mut city.m().workPositions));

            } else {
                println!("Not enough work positions in AGHousehold::regenerate. Should not happen!");
            }
        }
    }

    /// C++: `bool AGHousehold::allocateChildrenSchool()`
    /// associates a school to each children. return false if not done (not enough place at school in the city...
    // bool
    // AGHousehold::allocateChildrenSchool() {
    //     std::list<AGChild>::iterator it;
    //     bool oneRemainsAtHome = false;
    //
    //     for (it = myChildren.begin(); it != myChildren.end(); ++it) {
    //         if (!it->allocateASchool(&(myCity->schools), myLocation)) {
    //             oneRemainsAtHome = true;
    //         }
    //     }
    //     return !oneRemainsAtHome;
    // }
    #[allow(unused_variables)]
    pub fn allocate_children_school(&mut self) -> bool {
        let mut oneRemainsAtHome = false;
        let city = self.myCity;
        let schools = Ptr::from_mut(&mut city.m().schools);
        let pos = self.myLocation;
        for it in self.myChildren.iter_mut() {
            if !it.allocate_a_school(schools, pos) {
                oneRemainsAtHome = true;
            }
        }
        !oneRemainsAtHome
    }

    /// C++: `bool AGHousehold::allocateAdultsWork()`
    /// associates a work position to every working adult is taken in account the unemployment and the number of work positions
    // bool
    // AGHousehold::allocateAdultsWork() {
    //     std::list<AGAdult>::iterator it;
    //     for (it = myAdults.begin(); it != myAdults.end(); ++it) {
    //         if (myCity->statData.workPositions <= 0) {
    //             std::cout << "Not enough free work positions in AGHousehold::allocateAdultsWork. Should not happen." << std::endl;
    //             return false;
    //
    //         } else {
    //             it->tryToWork(1 - myCity->statData.unemployement, &(myCity->workPositions));
    //         }
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn allocate_adults_work(&mut self) -> bool {
        let city = self.myCity;
        let wps = Ptr::from_mut(&mut city.m().workPositions);
        for it in self.myAdults.iter_mut() {
            if city.statData.workPositions <= 0 {
                println!("Not enough free work positions in AGHousehold::allocateAdultsWork. Should not happen.");
                return false;

            } else {
                it.try_to_work(1.0 - city.statData.unemployement, wps);
            }
        }
        true
    }

    /// C++: `void AGHousehold::generateCars(double)`
    /// function allocating cars to this household in relation to the given rate for each adult
    // void
    // AGHousehold::generateCars(double rate) {
    //     int peopleInNeed = static_cast<int>(myAdults.size()) - static_cast<int>(myCars.size());
    //     while (peopleInNeed > 0) {
    //         if (RandHelper::rand() < rate) {
    //             addACar();
    //         }
    //         --peopleInNeed;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn generate_cars(&mut self, rate: f64) {
        let mut peopleInNeed = self.myAdults.len() as i32 - self.myCars.len() as i32;
        while peopleInNeed > 0 {
            if utils::common::rand_helper::rand() < rate {
                self.add_a_car();
            }
            peopleInNeed -= 1;
        }
    }

    /// C++: `int AGHousehold::getCarNbr()`
    // int
    // AGHousehold::getCarNbr() {
    //     return static_cast<int>(myCars.size());
    // }
    #[allow(unused_variables)]
    pub fn get_car_nbr(&mut self) -> i32 {
        self.myCars.len() as i32
    }

    /// C++: `void AGHousehold::addACar()`
    /// generates one (more) car in this household
    // void
    // AGHousehold::addACar() {
    //     int numCar = static_cast<int>(myCars.size() + 1);
    //     myCars.push_back(AGCar(myId, numCar));
    // }
    #[allow(unused_variables)]
    pub fn add_a_car(&mut self) {
        let numCar = (self.myCars.len() + 1) as i32;
        self.myCars.push_back(AGCar::new_2(self.myId, numCar));
    }

    /// C++: `AGCity * AGHousehold::getTheCity()`
    /// returns the city pointer in which the household is.
    // AGCity*
    // AGHousehold::getTheCity() {
    //     return myCity;
    // }
    #[allow(unused_variables)]
    pub fn get_the_city(&mut self) -> Ptr<AGCity> {
        self.myCity
    }

    /// C++: `bool AGHousehold::retiredHouseholders()`
    /// returns if adults are retired or in working age
    // bool
    // AGHousehold::retiredHouseholders() {
    //     return (myAdults.front().getAge() >= myCity->statData.limitAgeRetirement);
    // }
    #[allow(unused_variables)]
    pub fn retired_householders(&mut self) -> bool {
        self.myAdults.front().unwrap().get_age() >= self.myCity.statData.limitAgeRetirement
    }

    /// C++: `AGPosition AGHousehold::getPosition()`
    /// returns the position of the household and other private entities
    // AGPosition
    // AGHousehold::getPosition() {
    //     return myLocation;
    // }
    #[allow(unused_variables)]
    pub fn get_position(&mut self) -> AGPosition {
        self.myLocation
    }

}

