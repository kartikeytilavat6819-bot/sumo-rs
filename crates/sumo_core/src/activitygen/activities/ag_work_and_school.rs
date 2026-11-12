//! Port of activitygen/activities/AGWorkAndSchool.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::sumo_vehicle_class::SVC_PASSENGER;

/// C++ class `AGWorkAndSchool` (activitygen/activities/AGWorkAndSchool.h)
#[repr(C)]
pub struct AGWorkAndSchool {
    /// C++ base class `AGActivity`
    pub base: AGActivity,
    pub childrenNeedingCarAccompaniment: VecDeque<AGChild>,
    pub adultNeedingCarAccompaniment: VecDeque<AGAdult>,
    pub workingPeoplePossCar: VecDeque<AGAdult>,
    pub personsDrivingCars: VecDeque<AGAdult>,
    /// adults of the household who are not in need of car for work but are able to accompany someone. used only in case of need.
    pub notNeedingDrivers: VecDeque<AGAdult>,
    /// list of temporary trips: contains accompaniment trips without any return to home.
    pub tempAccTrip: VecDeque<AGTrip>,
    pub tempTrip: VecDeque<AGTrip>,
}

impl std::ops::Deref for AGWorkAndSchool {
    type Target = AGActivity;
    fn deref(&self) -> &AGActivity {
        &self.base
    }
}
impl std::ops::DerefMut for AGWorkAndSchool {
    fn deref_mut(&mut self) -> &mut AGActivity {
        &mut self.base
    }
}

impl AGWorkAndSchool {
    /// C++: `void AGWorkAndSchool::AGWorkAndSchool(AGHousehold *, AGDataAndStatistics *, std::list<AGTrip> *)`
    // AGWorkAndSchool(AGHousehold* hh, AGDataAndStatistics* das, std::list<AGTrip>* prevTrips) :
    //         AGActivity(hh, das, prevTrips, 1) {}
    #[allow(unused_variables)]
    pub fn new(hh: Ptr<AGHousehold>, das: Ptr<AGDataAndStatistics>, prevTrips: Ptr<VecDeque<AGTrip>>) -> Self {
        Self {
            base: AGActivity::new(hh, das, prevTrips, 1),
            childrenNeedingCarAccompaniment: VecDeque::new(),
            adultNeedingCarAccompaniment: VecDeque::new(),
            workingPeoplePossCar: VecDeque::new(),
            personsDrivingCars: VecDeque::new(),
            notNeedingDrivers: VecDeque::new(),
            tempAccTrip: VecDeque::new(),
            tempTrip: VecDeque::new(),
        }
    }

    /// C++: `bool AGWorkAndSchool::generateTrips()` [virtual]
    // bool
    // AGWorkAndSchool::generateTrips() {
    //     //buildDestinations();
    //     // generation of the waiting list for the accompaniment
    //     buildChildrenAccompaniment();
    //
    //     buildWorkDestinations();
    //
    //     if (myHousehold->getCarNbr() < (int)personsDrivingCars.size()) {
    //         return false;    //to rebuild the household
    //     }
    //     if (childrenNeedingCarAccompaniment.size() != 0 && myHousehold->getCarNbr() == 0) {
    //         return false;    //to rebuild the household
    //     }
    //     if (adultNeedingCarAccompaniment.size() != 0 && myHousehold->getCarNbr() == 0) {
    //         return false;
    //     }
    //
    //     carAllocation();
    //
    //     if (personsDrivingCars.empty() && notNeedingDrivers.empty()) {
    //         genDone = true;
    //         return true; // no trip to generate
    //     }
    //
    //     if (! carsToTrips()) {
    //         return false;
    //     }
    //
    //     genDone = true;
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn generate_trips(&mut self) -> bool {
        //buildDestinations();
        // generation of the waiting list for the accompaniment
        self.build_children_accompaniment();

        self.build_work_destinations();

        if self.base.myHousehold.m().get_car_nbr() < self.personsDrivingCars.len() as i32 {
            return false; //to rebuild the household
        }
        if !self.childrenNeedingCarAccompaniment.is_empty() && self.base.myHousehold.m().get_car_nbr() == 0 {
            return false; //to rebuild the household
        }
        if !self.adultNeedingCarAccompaniment.is_empty() && self.base.myHousehold.m().get_car_nbr() == 0 {
            return false;
        }

        self.car_allocation();

        if self.personsDrivingCars.is_empty() && self.notNeedingDrivers.is_empty() {
            self.base.genDone = true;
            return true; // no trip to generate
        }

        if !self.cars_to_trips() {
            return false;
        }

        self.base.genDone = true;
        true
    }

    /// C++: `void AGWorkAndSchool::buildChildrenAccompaniment()`
    // void
    // AGWorkAndSchool::buildChildrenAccompaniment() {
    //     std::list<AGChild>::const_iterator itC;
    //     for (itC = myHousehold->getChildren().begin(); itC != myHousehold->getChildren().end(); ++itC) {
    //         if (itC->haveASchool()) {
    //             if (this->availableTranspMeans(myHousehold->getPosition(), itC->getSchoolLocation()) == 0) {
    //                 //in this case the school is far from home and bus stations too
    //                 this->childrenNeedingCarAccompaniment.push_back(*itC);
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn build_children_accompaniment(&mut self) {
        let hh = self.base.myHousehold;
        for itC in hh.get_children().iter() {
            if itC.have_a_school() {
                if self.available_transp_means(hh.m().get_position(), itC.get_school_location()) == 0 {
                    //in this case the school is far from home and bus stations too
                    self.childrenNeedingCarAccompaniment.push_back(copy_child(itC));
                }
            }
        }
    }

    /// C++: `void AGWorkAndSchool::buildWorkDestinations()`
    // void
    // AGWorkAndSchool::buildWorkDestinations() {
    //     std::list<AGAdult>::const_iterator itA;
    //     for (itA = myHousehold->getAdults().begin(); itA != myHousehold->getAdults().end(); ++itA) {
    //         if (itA->isWorking()) {
    //             if (this->possibleTranspMean(itA->getWorkPosition().getPosition()) % 2 == 0) {
    //                 //not too close, to not being able to go by foot
    //                 if (this->possibleTranspMean(itA->getWorkPosition().getPosition()) > 4) {
    //                     //too far from home ==> Car or Bus AND Car and bus are possible
    //                     workingPeoplePossCar.push_back(*itA);
    //                 } else if (this->possibleTranspMean(itA->getWorkPosition().getPosition()) == 4) {
    //                     //only the car is possible (and there is one (use of possibleTranspMean))
    //                     if (myHousehold->getCarNbr() > (int)personsDrivingCars.size()) {
    //                         personsDrivingCars.push_back(*itA);
    //                     } else {
    //                         adultNeedingCarAccompaniment.push_back(*itA);
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //
    //     // sometimes, people still have choice: when vehicles are available and their car take a bus.
    //     std::list<AGAdult>::iterator it;
    //     for (it = workingPeoplePossCar.begin(); it != workingPeoplePossCar.end(); ++it) {
    //         if (possibleTranspMean(it->getWorkPosition().getPosition()) == 6 && myHousehold->getCarNbr() > (int)personsDrivingCars.size()) {
    //             //car or bus (always because of workDestinations' construction) AND at least one car not used
    //             if (myHousehold->getAdults().front().decide(this->carPreference)) {
    //                 personsDrivingCars.push_back(*it);
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn build_work_destinations(&mut self) {
        let hh = self.base.myHousehold;
        for itA in hh.get_adults().iter() {
            if itA.is_working() {
                if self.possible_transp_mean(itA.get_work_position().get_position()) % 2 == 0 {
                    //not too close, to not being able to go by foot
                    if self.possible_transp_mean(itA.get_work_position().get_position()) > 4 {
                        //too far from home ==> Car or Bus AND Car and bus are possible
                        self.workingPeoplePossCar.push_back(copy_adult(itA));
                    } else if self.possible_transp_mean(itA.get_work_position().get_position()) == 4 {
                        //only the car is possible (and there is one (use of possibleTranspMean))
                        if hh.m().get_car_nbr() > self.personsDrivingCars.len() as i32 {
                            self.personsDrivingCars.push_back(copy_adult(itA));
                        } else {
                            self.adultNeedingCarAccompaniment.push_back(copy_adult(itA));
                        }
                    }
                }
            }
        }

        // sometimes, people still have choice: when vehicles are available and their car take a bus.
        for idx in 0..self.workingPeoplePossCar.len() {
            let pos = self.workingPeoplePossCar[idx].get_work_position().get_position();
            if self.possible_transp_mean(pos) == 6 && hh.m().get_car_nbr() > self.personsDrivingCars.len() as i32 {
                //car or bus (always because of workDestinations' construction) AND at least one car not used
                if hh.get_adults().front().unwrap().decide(self.base.carPreference) {
                    let a = copy_adult(&self.workingPeoplePossCar[idx]);
                    self.personsDrivingCars.push_back(a);
                }
            }
        }
    }

    /// C++: `void AGWorkAndSchool::carAllocation()`
    /// allocation of cars to the convenient adult: the adult able to accompany the other (adult) and children in need.
    // void
    // AGWorkAndSchool::carAllocation() {
    //     // only two adults are possibles: no car, 1 car, 2 cars and more
    //     // the only choice case: 1 car / 2 adults needing this car (otherwise no choice problems)
    //     if (! personsDrivingCars.empty() && ! adultNeedingCarAccompaniment.empty()) {
    //         //in that case there is only one element in each list and only one car.
    //         if (adultNeedingCarAccompaniment.front().getWorkPosition().getOpening() >= personsDrivingCars.front().getWorkPosition().getOpening()) {
    //             //we will invert the driver and the accompanied
    //             personsDrivingCars.push_back(adultNeedingCarAccompaniment.front());
    //             adultNeedingCarAccompaniment.pop_front();
    //             adultNeedingCarAccompaniment.push_back(personsDrivingCars.front());
    //             personsDrivingCars.pop_front();
    //         }
    //     }
    //     if (personsDrivingCars.empty() && ! childrenNeedingCarAccompaniment.empty()) {
    //         //at least one adult exists because no household contains less than one adult
    //         if ((int)workingPeoplePossCar.size() != myHousehold->getAdultNbr()) { //personsDrivingCars.size() + adultNeedingCarAccompaniment.size() is equal to 0
    //             std::list<AGAdult>::const_iterator itUA;
    //             for (itUA = myHousehold->getAdults().begin(); itUA != myHousehold->getAdults().end(); ++itUA) {
    //                 if (! itUA->isWorking()) {
    //                     notNeedingDrivers.push_back(*itUA);
    //                     break;
    //                 }
    //             }
    //         } else {
    //             personsDrivingCars.push_back(workingPeoplePossCar.front());
    //             workingPeoplePossCar.pop_front();
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn car_allocation(&mut self) {
        // only two adults are possibles: no car, 1 car, 2 cars and more
        // the only choice case: 1 car / 2 adults needing this car (otherwise no choice problems)
        if !self.personsDrivingCars.is_empty() && !self.adultNeedingCarAccompaniment.is_empty() {
            //in that case there is only one element in each list and only one car.
            if self.adultNeedingCarAccompaniment.front().unwrap().get_work_position().get_opening()
                >= self.personsDrivingCars.front().unwrap().get_work_position().get_opening()
            {
                //we will invert the driver and the accompanied
                let a = copy_adult(self.adultNeedingCarAccompaniment.front().unwrap());
                self.personsDrivingCars.push_back(a);
                self.adultNeedingCarAccompaniment.pop_front();
                let d = copy_adult(self.personsDrivingCars.front().unwrap());
                self.adultNeedingCarAccompaniment.push_back(d);
                self.personsDrivingCars.pop_front();
            }
        }
        if self.personsDrivingCars.is_empty() && !self.childrenNeedingCarAccompaniment.is_empty() {
            //at least one adult exists because no household contains less than one adult
            let hh = self.base.myHousehold;
            if self.workingPeoplePossCar.len() as i32 != hh.m().get_adult_nbr() {
                for itUA in hh.get_adults().iter() {
                    if !itUA.is_working() {
                        self.notNeedingDrivers.push_back(copy_adult(itUA));
                        break;
                    }
                }
            } else {
                let w = copy_adult(self.workingPeoplePossCar.front().unwrap());
                self.personsDrivingCars.push_back(w);
                self.workingPeoplePossCar.pop_front();
            }
        }
    }

    /// C++: `bool AGWorkAndSchool::carsToTrips()`
    /// trip generation for adults and children
    // bool
    // AGWorkAndSchool::carsToTrips() {
    //     // check if the starting edge allows cars
    //     if (!myHousehold->getPosition().getStreet().allows(SVC_PASSENGER)) {
    //         return false;
    //     }
    //     std::list<AGAdult>::const_iterator itDriA;
    //     std::list<AGCar>::const_iterator itCar = myHousehold->getCars().begin();
    //     for (itDriA = personsDrivingCars.begin(); itDriA != personsDrivingCars.end(); ++itDriA) {
    //         //check if the number of cars is lower than the number of drivers
    //         if (itCar == myHousehold->getCars().end()) {
    //             return false;
    //         }
    //         // check if the destination edge allows cars
    //         if (!itDriA->getWorkPosition().getPosition().getStreet().allows(SVC_PASSENGER)) {
    //             return false;
    //         }
    //         AGTrip trip(myHousehold->getPosition(), itDriA->getWorkPosition().getPosition(), *itCar, depHour(myHousehold->getPosition(), itDriA->getWorkPosition().getPosition(), itDriA->getWorkPosition().getOpening()));
    //         ++itCar;
    //         tempTrip.push_back(trip);
    //     }
    //
    //     std::list<AGAdult>::iterator itAccA;
    //     for (itAccA = adultNeedingCarAccompaniment.begin(); itAccA != adultNeedingCarAccompaniment.end(); ++itAccA) {
    //         AGTrip trip(myHousehold->getPosition(), itAccA->getWorkPosition().getPosition(), depHour(myHousehold->getPosition(), itAccA->getWorkPosition().getPosition(), itAccA->getWorkPosition().getOpening()));
    //         tempAccTrip.push_back(trip);
    //     }
    //
    //     std::list<AGChild>::iterator itAccC;
    //     for (itAccC = childrenNeedingCarAccompaniment.begin(); itAccC != childrenNeedingCarAccompaniment.end(); ++itAccC) {
    //         AGTrip trip(myHousehold->getPosition(), itAccC->getSchoolLocation(), depHour(myHousehold->getPosition(), itAccC->getSchoolLocation(), itAccC->getSchoolOpening()));
    //         tempAccTrip.push_back(trip);
    //     }
    //
    //     checkAndBuildTripConsistancy();
    //     if (isThereUnusedCar() && ! checkDriversScheduleMatching()) {
    //         makePossibleDriversDrive();
    //     }
    //
    //     generateListTrips();
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn cars_to_trips(&mut self) -> bool {
        let hh = self.base.myHousehold;
        // check if the starting edge allows cars
        if !hh.m().get_position().get_street().allows(SVC_PASSENGER) {
            return false;
        }
        let mut itCar: usize = 0;
        for idx in 0..self.personsDrivingCars.len() {
            //check if the number of cars is lower than the number of drivers
            if itCar == hh.get_cars().len() {
                return false;
            }
            let work = self.personsDrivingCars[idx].get_work_position().get_position();
            // check if the destination edge allows cars
            if !work.get_street().allows(SVC_PASSENGER) {
                return false;
            }
            let car = copy_car(&hh.get_cars()[itCar]);
            let start = hh.m().get_position();
            let opening = self.personsDrivingCars[idx].get_work_position().get_opening();
            let dep = self.base.dep_hour(start, work, opening);
            let trip = AGTrip::new_2(start, work, car, dep);
            itCar += 1;
            self.tempTrip.push_back(trip);
        }

        for idx in 0..self.adultNeedingCarAccompaniment.len() {
            let work = self.adultNeedingCarAccompaniment[idx].get_work_position().get_position();
            let start = hh.m().get_position();
            let opening = self.adultNeedingCarAccompaniment[idx].get_work_position().get_opening();
            let dep = self.base.dep_hour(start, work, opening);
            let trip = AGTrip::new(start, work, dep);
            self.tempAccTrip.push_back(trip);
        }

        for idx in 0..self.childrenNeedingCarAccompaniment.len() {
            let school = self.childrenNeedingCarAccompaniment[idx].get_school_location();
            let start = hh.m().get_position();
            let opening = self.childrenNeedingCarAccompaniment[idx].get_school_opening();
            let dep = self.base.dep_hour(start, school, opening);
            let trip = AGTrip::new(start, school, dep);
            self.tempAccTrip.push_back(trip);
        }

        self.check_and_build_trip_consistancy();
        if self.is_there_unused_car() && !self.check_drivers_schedule_matching() {
            self.make_possible_drivers_drive();
        }

        self.generate_list_trips();
        true
    }

    /// C++: `bool AGWorkAndSchool::isThereUnusedCar()`
    /// returns true if there is an unused car that can be driven by an adult
    // bool
    // AGWorkAndSchool::isThereUnusedCar() {
    //     return (myHousehold->getCarNbr() > static_cast<int>(notNeedingDrivers.size() + personsDrivingCars.size()));
    // }
    #[allow(unused_variables)]
    pub fn is_there_unused_car(&mut self) -> bool {
        self.base.myHousehold.m().get_car_nbr() > (self.notNeedingDrivers.len() + self.personsDrivingCars.len()) as i32
    }

    /// C++: `bool AGWorkAndSchool::checkAndBuildTripConsistancy()`
    /// main function of Accompaniment trip management: simple trips are analyzed whether they can be grouped together or not. return true if every thing worked well
    // bool
    // AGWorkAndSchool::checkAndBuildTripConsistancy() {
    //     bool finish = false;
    //     int diff1, diff2;
    //     int arrTime;
    //     std::list<AGTrip>::iterator it1, it2;
    //
    //     while (!finish) {
    //         finish = true;
    //         for (it1 = tempAccTrip.begin(); it1 != tempAccTrip.end(); ++it1) {
    //             for (it2 = tempAccTrip.begin(); it2 != tempAccTrip.end(); ++it2) {
    //                 if (it1 == it2) {
    //                     continue;
    //                 }
    //                 diff1 = it2->getTime() - it1->getRideBackArrTime(this->timePerKm);
    //                 diff2 = it1->getTime() - it2->getRideBackArrTime(this->timePerKm);
    //
    //                 if (diff1 < 0 || diff2 < 0) {
    //                     if (diff2 < diff1) {
    //                         arrTime = it2->getArrTime(this->timePerKm);
    //                         it2->addLayOver(*it1);
    //                         it2->setDepTime(it2->estimateDepTime(arrTime, this->timePerKm));
    //                         tempAccTrip.erase(it1);
    //                     } else {
    //                         arrTime = it1->getArrTime(this->timePerKm);
    //                         it1->addLayOver(*it2);
    //                         it1->setDepTime(it1->estimateDepTime(arrTime, this->timePerKm));
    //                         tempAccTrip.erase(it2);
    //                     }
    //                     finish = false;
    //                     break;
    //                 }
    //             }
    //             if (!finish) {
    //                 break; // return to while
    //             }
    //         }
    //     }
    //     return finish;
    // }
    #[allow(unused_variables)]
    pub fn check_and_build_trip_consistancy(&mut self) -> bool {
        let mut finish = false;
        while !finish {
            finish = true;
            for i1 in 0..self.tempAccTrip.len() {
                for i2 in 0..self.tempAccTrip.len() {
                    if i1 == i2 {
                        continue;
                    }
                    let diff1 = self.tempAccTrip[i2].get_time() - self.tempAccTrip[i1].get_ride_back_arr_time(self.base.timePerKm);
                    let diff2 = self.tempAccTrip[i1].get_time() - self.tempAccTrip[i2].get_ride_back_arr_time(self.base.timePerKm);

                    if diff1 < 0 || diff2 < 0 {
                        if diff2 < diff1 {
                            let arrTime = self.tempAccTrip[i2].get_arr_time(self.base.timePerKm);
                            // copy: add_lay_over_2 takes &mut, the C++ argument is only read
                            let mut it1 = copy_trip(&self.tempAccTrip[i1]);
                            self.tempAccTrip[i2].add_lay_over_2(&mut it1);
                            let dep = self.tempAccTrip[i2].estimate_dep_time(arrTime, self.base.timePerKm);
                            self.tempAccTrip[i2].set_dep_time(dep);
                            self.tempAccTrip.remove(i1);
                        } else {
                            let arrTime = self.tempAccTrip[i1].get_arr_time(self.base.timePerKm);
                            let mut it2 = copy_trip(&self.tempAccTrip[i2]);
                            self.tempAccTrip[i1].add_lay_over_2(&mut it2);
                            let dep = self.tempAccTrip[i1].estimate_dep_time(arrTime, self.base.timePerKm);
                            self.tempAccTrip[i1].set_dep_time(dep);
                            self.tempAccTrip.remove(i2);
                        }
                        finish = false;
                        break;
                    }
                }
                if !finish {
                    break; // return to while
                }
            }
        }
        finish
    }

    /// C++: `bool AGWorkAndSchool::checkDriversScheduleMatching()`
    /// checks whether the driver is able to accompany all people or if the others have to go too late to work for him
    // bool
    // AGWorkAndSchool::checkDriversScheduleMatching() {
    //     bool check = false;
    //     std::list<AGTrip>::iterator itAccT;
    //     std::list<AGTrip>::iterator itDriT;
    //     std::list<AGAdult>::iterator itA;
    //     for (itAccT = tempAccTrip.begin(); itAccT != tempAccTrip.end(); ++itAccT) {
    //         for (itDriT = tempTrip.begin(); itDriT != tempTrip.end(); ++itDriT) {
    //             if (itAccT->getArrTime(this->timePerKm) < itDriT->getArrTime(this->timePerKm)) {
    //                 check = true;
    //             }
    //         }
    //         for (itA = notNeedingDrivers.begin(); itA != notNeedingDrivers.end(); ++itA) {
    //             if (!itA->isWorking()) {
    //                 check = true;
    //             } else if (itAccT->getRideBackArrTime(this->timePerKm) < itA->getWorkPosition().getOpening()) {
    //                 check = true;
    //             }
    //         }
    //         if (!check) { //at least one trip is not performed by the existing drivers because it is to late for them
    //             return false;
    //         }
    //         check = false;
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn check_drivers_schedule_matching(&mut self) -> bool {
        let mut check = false;
        for itAccT in self.tempAccTrip.iter() {
            for itDriT in self.tempTrip.iter() {
                if itAccT.get_arr_time(self.base.timePerKm) < itDriT.get_arr_time(self.base.timePerKm) {
                    check = true;
                }
            }
            for itA in self.notNeedingDrivers.iter() {
                if !itA.is_working() {
                    check = true;
                } else if itAccT.get_ride_back_arr_time(self.base.timePerKm) < itA.get_work_position().get_opening() {
                    check = true;
                }
            }
            if !check {
                //at least one trip is not performed by the existing drivers because it is to late for them
                return false;
            }
            check = false;
        }
        true
    }

    /// C++: `void AGWorkAndSchool::generateListTrips()`
    /// function generating Trip objects using the local objects
    // void
    // AGWorkAndSchool::generateListTrips() {
    //     int arrTime;
    //     std::list<AGTrip>::iterator itAccT;
    //     std::list<AGTrip>::iterator itDriT;
    //     std::list<AGAdult>::iterator itA;
    //     bool alreadyDone;
    //
    //     /**
    //      * 1 / 3 : Accompaniment
    //      */
    //     for (itAccT = tempAccTrip.begin(); itAccT != tempAccTrip.end(); ++itAccT) {
    //         alreadyDone = false;
    //         for (itDriT = tempTrip.begin(); itDriT != tempTrip.end(); ++itDriT) {
    //             if (!alreadyDone) {
    //                 if (itAccT->getArrTime(this->timePerKm) < itDriT->getArrTime(this->timePerKm) && !alreadyDone) {
    //                     //Add the accompaniment trip to the driver's trip OR new trip
    //                     if (itAccT->getRideBackArrTime(this->timePerKm) < itDriT->getTime()) {
    //                         //there is enough time to accompany people and go back home before going to work
    //                         itAccT->setVehicleName(itDriT->getVehicleName());
    //                         itAccT->addLayOver(itAccT->getArr());//final destination is the last accompaniment stop: not the destination of the course
    //                         itAccT->setArr(myHousehold->getPosition());//final destination of the whole trip: home
    //                         myPartialActivityTrips.push_back(*itAccT);
    //                         alreadyDone = true;
    //                     } else {
    //                         //the driver drives people to their working place or school and goes directly to work after that
    //                         arrTime = itDriT->getArrTime(this->timePerKm);
    //                         itDriT->addLayOver(*itAccT);
    //                         itDriT->setDepTime(itDriT->estimateDepTime(arrTime, this->timePerKm));
    //                         //tempAccTrip.erase(itAccT);
    //                         //--itAccT; //because of erasure
    //                         alreadyDone = true;
    //                     }
    //                 }
    //             }
    //         }
    //
    //         for (itA = notNeedingDrivers.begin(); itA != notNeedingDrivers.end(); ++itA) {
    //             if (!itA->isWorking() && !alreadyDone) {
    //                 std::string nameC = getUnusedCar();
    //                 if (nameC.size() != 0) {
    //                     itAccT->setVehicleName(getUnusedCar());
    //                     itAccT->addLayOver(itAccT->getArr());
    //                     itAccT->setArr(myHousehold->getPosition());
    //                     myPartialActivityTrips.push_back(*itAccT);
    //                     alreadyDone = true;
    //                 }
    //             } else if (itAccT->getRideBackArrTime(this->timePerKm) < itA->getWorkPosition().getOpening() && !alreadyDone) {
    //                 std::string nameC = getUnusedCar();
    //                 if (nameC.size() != 0) {
    //                     itAccT->setVehicleName(getUnusedCar());
    //                     itAccT->addLayOver(itAccT->getArr());
    //                     itAccT->setArr(myHousehold->getPosition());
    //                     myPartialActivityTrips.push_back(*itAccT);
    //                     alreadyDone = true;
    //                 }
    //             }
    //         }
    //     }
    //
    //     /**
    //      * 2/3 : drivers way
    //      */
    //     for (itDriT = tempTrip.begin(); itDriT != tempTrip.end(); ++itDriT) {
    //         myPartialActivityTrips.push_back(*itDriT);
    //     }
    //
    //     /**
    //      * 3/3: way return
    //      */
    //     for (itA = personsDrivingCars.begin(); itA != personsDrivingCars.end(); ++itA) {
    //         for (itDriT = tempTrip.begin(); itDriT != tempTrip.end(); ++itDriT) {
    //             if (itA->getWorkPosition().getPosition() == itDriT->getArr()) {
    //                 AGTrip trip(itA->getWorkPosition().getPosition(), myHousehold->getPosition(), itDriT->getVehicleName(), itA->getWorkPosition().getClosing());
    //                 myPartialActivityTrips.push_back(trip);
    //                 tempTrip.erase(itDriT);
    //                 break;
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn generate_list_trips(&mut self) {
        let hh = self.base.myHousehold;
        let timePerKm = self.base.timePerKm;

        /**
         * 1 / 3 : Accompaniment
         */
        for i in 0..self.tempAccTrip.len() {
            let mut alreadyDone = false;
            for j in 0..self.tempTrip.len() {
                if !alreadyDone {
                    if self.tempAccTrip[i].get_arr_time(timePerKm) < self.tempTrip[j].get_arr_time(timePerKm) && !alreadyDone {
                        //Add the accompaniment trip to the driver's trip OR new trip
                        if self.tempAccTrip[i].get_ride_back_arr_time(timePerKm) < self.tempTrip[j].get_time() {
                            //there is enough time to accompany people and go back home before going to work
                            let vn = self.tempTrip[j].get_vehicle_name().to_string();
                            self.tempAccTrip[i].set_vehicle_name(vn);
                            let arr = self.tempAccTrip[i].get_arr();
                            self.tempAccTrip[i].add_lay_over(arr); //final destination is the last accompaniment stop: not the destination of the course
                            let home = hh.m().get_position();
                            self.tempAccTrip[i].set_arr(home); //final destination of the whole trip: home
                            let t = copy_trip(&self.tempAccTrip[i]);
                            self.base.myPartialActivityTrips.push_back(t);
                            alreadyDone = true;
                        } else {
                            //the driver drives people to their working place or school and goes directly to work after that
                            let arrTime = self.tempTrip[j].get_arr_time(timePerKm);
                            // copy: add_lay_over_2 takes &mut, the C++ argument is only read
                            let mut acc = copy_trip(&self.tempAccTrip[i]);
                            self.tempTrip[j].add_lay_over_2(&mut acc);
                            let dep = self.tempTrip[j].estimate_dep_time(arrTime, timePerKm);
                            self.tempTrip[j].set_dep_time(dep);
                            alreadyDone = true;
                        }
                    }
                }
            }

            for k in 0..self.notNeedingDrivers.len() {
                if !self.notNeedingDrivers[k].is_working() && !alreadyDone {
                    let nameC = self.get_unused_car();
                    if !nameC.is_empty() {
                        let vn = self.get_unused_car();
                        self.tempAccTrip[i].set_vehicle_name(vn);
                        let arr = self.tempAccTrip[i].get_arr();
                        self.tempAccTrip[i].add_lay_over(arr);
                        let home = hh.m().get_position();
                        self.tempAccTrip[i].set_arr(home);
                        let t = copy_trip(&self.tempAccTrip[i]);
                        self.base.myPartialActivityTrips.push_back(t);
                        alreadyDone = true;
                    }
                } else if self.tempAccTrip[i].get_ride_back_arr_time(timePerKm) < self.notNeedingDrivers[k].get_work_position().get_opening() && !alreadyDone {
                    let nameC = self.get_unused_car();
                    if !nameC.is_empty() {
                        let vn = self.get_unused_car();
                        self.tempAccTrip[i].set_vehicle_name(vn);
                        let arr = self.tempAccTrip[i].get_arr();
                        self.tempAccTrip[i].add_lay_over(arr);
                        let home = hh.m().get_position();
                        self.tempAccTrip[i].set_arr(home);
                        let t = copy_trip(&self.tempAccTrip[i]);
                        self.base.myPartialActivityTrips.push_back(t);
                        alreadyDone = true;
                    }
                }
            }
        }

        /**
         * 2/3 : drivers way
         */
        for t in self.tempTrip.iter() {
            self.base.myPartialActivityTrips.push_back(copy_trip(t));
        }

        /**
         * 3/3: way return
         */
        for k in 0..self.personsDrivingCars.len() {
            let mut j = 0;
            while j < self.tempTrip.len() {
                let work = self.personsDrivingCars[k].get_work_position();
                if work.get_position() == self.tempTrip[j].get_arr() {
                    let trip = AGTrip::new_4(
                        work.get_position(),
                        hh.m().get_position(),
                        self.tempTrip[j].get_vehicle_name().to_string(),
                        work.get_closing(),
                    );
                    self.base.myPartialActivityTrips.push_back(trip);
                    self.tempTrip.remove(j);
                    break;
                }
                j += 1;
            }
        }
    }

    /// C++: `void AGWorkAndSchool::makePossibleDriversDrive()`
    /// function adding new drivers when a not used car is needed + when the person prefer the car to the bus (choice variable)
    // void
    // AGWorkAndSchool::makePossibleDriversDrive() {
    //     //give to a non working adult the ability to drive children or someone else.
    //     if ((int)(workingPeoplePossCar.size() + personsDrivingCars.size() + adultNeedingCarAccompaniment.size()) != myHousehold->getAdultNbr()) {
    //         std::list<AGAdult>::const_iterator itUA;
    //         for (itUA = myHousehold->getAdults().begin(); itUA != myHousehold->getAdults().end(); ++itUA) {
    //             if (! itUA->isWorking()) {
    //                 notNeedingDrivers.push_back(*itUA);
    //                 break;
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn make_possible_drivers_drive(&mut self) {
        //give to a non working adult the ability to drive children or someone else.
        let hh = self.base.myHousehold;
        if (self.workingPeoplePossCar.len() + self.personsDrivingCars.len() + self.adultNeedingCarAccompaniment.len()) as i32 != hh.m().get_adult_nbr() {
            for itUA in hh.get_adults().iter() {
                if !itUA.is_working() {
                    self.notNeedingDrivers.push_back(copy_adult(itUA));
                    break;
                }
            }
        }
    }

    /// C++: `std::string AGWorkAndSchool::getUnusedCar()`
    /// return the name of a car that is not used by another adult driver of the household in fact there is only 1 or 2 adults, so if the number of cars is greater that 1 at least one car isn't used
    // std::string
    // AGWorkAndSchool::getUnusedCar() {
    //     std::string nameCar = "";
    //     std::string nameCarUsed = "";
    //     //only two cars can be used in the household, so: the first one or the last one is not used.
    //     if (!tempTrip.empty()) {
    //         nameCarUsed = tempTrip.front().getVehicleName();
    //     } else if (!myPartialActivityTrips.empty()) {
    //         nameCarUsed = myPartialActivityTrips.front().getVehicleName();
    //     }
    //
    //     if (nameCarUsed.size() != 0) {
    //         if (myHousehold->getCars().front().getName() == nameCarUsed) {
    //             nameCar = myHousehold->getCars().back().getName();
    //         } else {
    //             nameCar = myHousehold->getCars().front().getName();
    //         }
    //     }
    //     return nameCar;
    // }
    #[allow(unused_variables)]
    pub fn get_unused_car(&mut self) -> String {
        let mut nameCar = String::new();
        let mut nameCarUsed = String::new();
        //only two cars can be used in the household, so: the first one or the last one is not used.
        if !self.tempTrip.is_empty() {
            nameCarUsed = self.tempTrip.front().unwrap().get_vehicle_name().to_string();
        } else if !self.base.myPartialActivityTrips.is_empty() {
            nameCarUsed = self.base.myPartialActivityTrips.front().unwrap().get_vehicle_name().to_string();
        }

        if !nameCarUsed.is_empty() {
            let hh = self.base.myHousehold;
            if hh.get_cars().front().unwrap().get_name() == nameCarUsed {
                nameCar = hh.get_cars().back().unwrap().get_name();
            } else {
                nameCar = hh.get_cars().front().unwrap().get_name();
            }
        }
        nameCar
    }

}

// Value copies of the C++ by-value AGAdult / AGChild / AGTrip / AGCar (these Rust structs do not derive Clone).
fn copy_adult(a: &AGAdult) -> AGAdult {
    AGAdult { base: AGPerson { age: a.base.age }, work: a.work }
}

fn copy_child(c: &AGChild) -> AGChild {
    AGChild { base: AGPerson { age: c.base.age }, mySchool: c.mySchool }
}

fn copy_car(c: &AGCar) -> AGCar {
    AGCar { idName: c.idName.clone(), currentUser: c.currentUser }
}

fn copy_trip(t: &AGTrip) -> AGTrip {
    AGTrip {
        myFrom: t.myFrom,
        myTo: t.myTo,
        myDepTime: t.myDepTime,
        myVehicle: t.myVehicle.clone(),
        myType: t.myType.clone(),
        myDay: t.myDay,
        myPassBy: t.myPassBy.clone(),
    }
}
