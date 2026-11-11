//! Port of activitygen/activities/AGFreeTime.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `AGFreeTime` (activitygen/activities/AGFreeTime.h)
pub struct AGFreeTime {
    /// C++ base class `AGActivity`
    pub base: AGActivity,
    /// frequency of going out or see family is assumed to be once a week (in mean)
    pub freqOut: f64,
    /// number of days for the simulation households are likely to go out some days but not others
    pub nbrDays: i32,
    /// time ready to do something else everybody is back home
    pub tReady: i32,
    /// possible type of trips for this household
    pub possibleType: i32,
}

impl std::ops::Deref for AGFreeTime {
    type Target = AGActivity;
    fn deref(&self) -> &AGActivity {
        &self.base
    }
}
impl std::ops::DerefMut for AGFreeTime {
    fn deref_mut(&mut self) -> &mut AGActivity {
        &mut self.base
    }
}

// C++ static members of AGFreeTime (port as module-level statics, see AGENTS.md):
//   static const int AGFreeTime::DAY;
//   static const int AGFreeTime::EVENING;
//   static const int AGFreeTime::NIGHT;
//   static const int AGFreeTime::TB_DAY;
//   static const int AGFreeTime::TE_DAY;
//   static const int AGFreeTime::TB_EVENING;
//   static const int AGFreeTime::TE_EVENING;
//   static const int AGFreeTime::TB_NIGHT;
//   static const int AGFreeTime::TE_NIGHT;

impl AGFreeTime {
    /// C++: `void AGFreeTime::AGFreeTime(AGHousehold *, AGDataAndStatistics *, std::list<AGTrip> *, int)`
    // AGFreeTime(AGHousehold* hh, AGDataAndStatistics* das, std::list<AGTrip>* prevTrips, int days = 1) :
    //         AGActivity(hh, das, prevTrips, 2),
    //         freqOut(das->freeTimeActivityRate),
    //         nbrDays(days) {}
    #[allow(unused_variables)]
    pub fn new(hh: Ptr<AGHousehold>, das: Ptr<AGDataAndStatistics>, prevTrips: Ptr<VecDeque<AGTrip>>, days: i32) -> Self {
        todo!("port AGFreeTime::AGFreeTime")
    }

    /// C++: `bool AGFreeTime::generateTrips()` [virtual]
    // bool
    // AGFreeTime::generateTrips() {
    //     tReady = whenBackHome();
    //     possibleType = possibleTypeOfTrip();
    //     int type;
    //
    //     for (int day = 1; day <= nbrDays; ++day) {
    //         type = decideTypeOfTrip();
    //         if (type == 0) {
    //             continue;
    //         } else if (type == DAY) {
    //             if (!typeFromHomeDay(day)) {
    //                 return false;
    //             }
    //         } else if (type == EVENING) {
    //             if (!typeFromHomeEvening(day)) {
    //                 return false;
    //             }
    //         } else if (type == NIGHT) {
    //             if (!typeFromHomeNight(day)) {
    //                 return false;
    //             }
    //         }
    //     }
    //     genDone = true;
    //     return genDone;
    // }
    #[allow(unused_variables)]
    pub fn generate_trips(&mut self) -> bool {
        todo!("port AGFreeTime::generateTrips")
    }

    /// C++: `int AGFreeTime::possibleTypeOfTrip()`
    /// returns the type of trip that will be done on this household there is also the case: no free time trip 0 = non 1 = during the day 2 = in the evening 4 = during the night (combinations using '+' are available for possibleTypeOfTrip())
    // int
    // AGFreeTime::possibleTypeOfTrip() {
    //     int val = 0;
    //     if (myHousehold->getAdults().front().getAge() >= myStatData->limitAgeRetirement && tReady == 0) {
    //         val += DAY + EVENING;
    //     } else {
    //         if (myHousehold->getPeopleNbr() > myHousehold->getAdultNbr()) {
    //             val += NIGHT;
    //         }
    //
    //         std::list<AGAdult>::const_iterator itA;
    //         bool noBodyWorks = true;
    //         for (itA = myHousehold->getAdults().begin(); itA != myHousehold->getAdults().end(); ++itA) {
    //             if (itA->isWorking()) {
    //                 noBodyWorks = false;
    //             }
    //         }
    //         if (noBodyWorks) {
    //             val += DAY;
    //         }
    //
    //         if (tReady < AGTime(0, 22, 0).getTime()) {
    //             val += EVENING;
    //         }
    //     }
    //     return val;
    // }
    #[allow(unused_variables)]
    pub fn possible_type_of_trip(&mut self) -> i32 {
        todo!("port AGFreeTime::possibleTypeOfTrip")
    }

    /// C++: `int AGFreeTime::decideTypeOfTrip()`
    // int
    // AGFreeTime::decideTypeOfTrip() {
    //     if (myHousehold->getAdults().front().decide(freqOut)) {
    //         int num_poss = 0; //(possibleType % 2) + (possibleType / 4) + ((possibleType / 2) % 2);
    //         if (possibleType & DAY) {
    //             ++num_poss;
    //         }
    //         if (possibleType & EVENING) {
    //             ++num_poss;
    //         }
    //         if (possibleType & NIGHT) {
    //             ++num_poss;
    //         }
    //
    //         if (num_poss == 0) {
    //             return 0;
    //         }
    //         double alea = RandHelper::rand(); //(float)(rand() % 1000) / 1000.0;
    //         int decision = (int)floor(alea * (double)num_poss);
    //
    //         if (possibleType & DAY) {
    //             if (decision == 0) {
    //                 return DAY;
    //             } else {
    //                 --decision;
    //             }
    //         }
    //         if (possibleType & EVENING) {
    //             if (decision == 0) {
    //                 return EVENING;
    //             } else {
    //                 --decision;
    //             }
    //         }
    //         if (possibleType & NIGHT) {
    //             if (decision == 0) {
    //                 return NIGHT;
    //             }
    //         }
    //     }
    //     return 0;
    // }
    #[allow(unused_variables)]
    pub fn decide_type_of_trip(&mut self) -> i32 {
        todo!("port AGFreeTime::decideTypeOfTrip")
    }

    /// C++: `bool AGFreeTime::typeFromHomeDay(int)`
    /// The different type of trips that are available: one function is called among all the following
    // bool
    // AGFreeTime::typeFromHomeDay(int day) {
    //     int backHome = whenBackHomeThisDay(day);
    //     if (myHousehold->getCars().empty()) {
    //         return true;
    //     }
    //     AGPosition destination(myHousehold->getTheCity()->getRandomStreet());
    //     int depTime = randomTimeBetween(MAX2(backHome, TB_DAY), (TB_DAY + TE_DAY) / 2);
    //     int arrTime = this->arrHour(myHousehold->getPosition(), destination, depTime);
    //     int retTime = randomTimeBetween(arrTime, TE_DAY);
    //     if (depTime < 0 || retTime < 0) {
    //         return true;    // not enough time during the day
    //     }
    //     AGTrip depTrip(myHousehold->getPosition(), destination, myHousehold->getCars().front().getName(), depTime, day);
    //     AGTrip retTrip(destination, myHousehold->getPosition(), myHousehold->getCars().front().getName(), retTime, day);
    //
    //     myPartialActivityTrips.push_back(depTrip);
    //     myPartialActivityTrips.push_back(retTrip);
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn type_from_home_day(&mut self, day: i32) -> bool {
        todo!("port AGFreeTime::typeFromHomeDay")
    }

    /// C++: `bool AGFreeTime::typeFromHomeEvening(int)`
    // bool
    // AGFreeTime::typeFromHomeEvening(int day) {
    //     int backHome = whenBackHomeThisDay(day);
    //     if (myHousehold->getCars().empty()) {
    //         return true;
    //     }
    //     AGPosition destination(myHousehold->getTheCity()->getRandomStreet());
    //     int depTime = randomTimeBetween(MAX2(backHome, TB_EVENING), TE_EVENING);
    //     int arrTime = this->arrHour(myHousehold->getPosition(), destination, depTime);
    //     int retTime = randomTimeBetween(arrTime, TE_EVENING);
    //     if (depTime < 0 || retTime < 0) {
    //         return true;    // not enough time during the day
    //     }
    //     AGTrip depTrip(myHousehold->getPosition(), destination, myHousehold->getCars().front().getName(), depTime, day);
    //     AGTrip retTrip(destination, myHousehold->getPosition(), myHousehold->getCars().front().getName(), retTime, day);
    //
    //     myPartialActivityTrips.push_back(depTrip);
    //     myPartialActivityTrips.push_back(retTrip);
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn type_from_home_evening(&mut self, day: i32) -> bool {
        todo!("port AGFreeTime::typeFromHomeEvening")
    }

    /// C++: `bool AGFreeTime::typeFromHomeNight(int)`
    // bool
    // AGFreeTime::typeFromHomeNight(int day) {
    //     int backHome = whenBackHomeThisDay(day);
    //     int ActivitiesNextDay = whenBeginActivityNextDay(day); // is equal to 2 days if there is nothing the next day
    //     int nextDay = 0;
    //     if (myHousehold->getCars().empty()) {
    //         return true;
    //     }
    //     AGPosition destination(myHousehold->getTheCity()->getRandomStreet());
    //
    //     int depTime = randomTimeBetween(MAX2(backHome, TB_NIGHT), TE_NIGHT);
    //     int arrTime = this->arrHour(myHousehold->getPosition(), destination, depTime);
    //     //we have to go back home before the beginning of next day activities.
    //     int lastRetTime = this->depHour(destination, myHousehold->getPosition(), MIN2(TE_NIGHT, ActivitiesNextDay));
    //     int retTime = randomTimeBetween(arrTime, lastRetTime);
    //     if (depTime < 0 || retTime < 0) {
    //         return true;    // not enough time during the day
    //     }
    //
    //     AGTime departureTime(depTime);
    //     nextDay = departureTime.getDay();
    //     departureTime.setDay(0);
    //     AGTrip depTrip(myHousehold->getPosition(), destination, myHousehold->getCars().front().getName(), departureTime.getTime(), day + nextDay);
    //
    //     AGTime returnTime(depTime);
    //     nextDay = returnTime.getDay();
    //     returnTime.setDay(0);
    //     AGTrip retTrip(destination, myHousehold->getPosition(), myHousehold->getCars().front().getName(), returnTime.getTime(), day + nextDay);
    //
    //     myPartialActivityTrips.push_back(depTrip);
    //     myPartialActivityTrips.push_back(retTrip);
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn type_from_home_night(&mut self, day: i32) -> bool {
        todo!("port AGFreeTime::typeFromHomeNight")
    }

    /// C++: `int AGFreeTime::whenBackHome()`
    /// returns the moment when everybody is back home and ready to do some thing else using everyday trips
    // int
    // AGFreeTime::whenBackHome() {
    //     int timeBack = 0;
    //     for (std::list<AGTrip>::iterator itT = myPreviousTrips->begin(); itT != myPreviousTrips->end(); ++itT) {
    //         if (timeBack < itT->getArrTime(this->timePerKm) && itT->isDaily()) {
    //             timeBack = itT->getArrTime(this->timePerKm);
    //         }
    //     }
    //     return timeBack;
    // }
    #[allow(unused_variables)]
    pub fn when_back_home(&mut self) -> i32 {
        todo!("port AGFreeTime::whenBackHome")
    }

    /// C++: `int AGFreeTime::whenBackHomeThisDay(int)`
    /// moment when everybody is back home using all trips applicable for the given day if no car is used: return 0
    // int
    // AGFreeTime::whenBackHomeThisDay(int day) {
    //     int timeBack = 0;
    //     for (std::list<AGTrip>::iterator itT = myPreviousTrips->begin(); itT != myPreviousTrips->end(); ++itT) {
    //         if (timeBack < itT->getArrTime(this->timePerKm) && (itT->getDay() == day || itT->isDaily())) {
    //             timeBack = itT->getArrTime(this->timePerKm);
    //         }
    //     }
    //     return timeBack;
    // }
    #[allow(unused_variables)]
    pub fn when_back_home_this_day(&mut self, day: i32) -> i32 {
        todo!("port AGFreeTime::whenBackHomeThisDay")
    }

    /// C++: `int AGFreeTime::whenBeginActivityNextDay(int)`
    /// moment when the first person takes the car to go somewhere id no car is used: return 2400 (midnight at the end of the given day)
    // int
    // AGFreeTime::whenBeginActivityNextDay(int day) {
    //     AGTime timeBack(1, 0, 0);
    //     for (std::list<AGTrip>::iterator itT = myPreviousTrips->begin(); itT != myPreviousTrips->end(); ++itT) {
    //         if (timeBack.getTime() > itT->getTime() && (itT->getDay() == (day + 1) || itT->isDaily())) {
    //             timeBack.setTime(itT->getTime());
    //         }
    //     }
    //     timeBack.addDays(1); // this the beginning of activities of the next day
    //     return timeBack.getTime();
    // }
    #[allow(unused_variables)]
    pub fn when_begin_activity_next_day(&mut self, day: i32) -> i32 {
        todo!("port AGFreeTime::whenBeginActivityNextDay")
    }

}

