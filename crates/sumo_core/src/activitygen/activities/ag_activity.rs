//! Port of activitygen/activities/AGActivity.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `AGActivity` (activitygen/activities/AGActivity.h)
pub struct AGActivity {
    pub myHousehold: Ptr<AGHousehold>,
    pub myStatData: Ptr<AGDataAndStatistics>,
    pub myPreviousTrips: Ptr<VecDeque<AGTrip>>,
    pub myPartialActivityTrips: VecDeque<AGTrip>,
    pub activityPriority: i32,
    pub genDone: bool,
    pub timePerKm: f64,
    /// rate of taking the car instead of the bus because of personal preference
    pub carPreference: f64,
}

impl AGActivity {
    /// C++: `void AGActivity::AGActivity(AGHousehold *, AGDataAndStatistics *, std::list<AGTrip> *, int)`
    // AGActivity(AGHousehold* hh, AGDataAndStatistics* das, std::list<AGTrip>* prevTrips, int prio) :
    //         myHousehold(hh),
    //         myStatData(das),
    //         myPreviousTrips(prevTrips),
    //         activityPriority(prio),
    //         genDone(false),
    //         timePerKm(das->speedTimePerKm),
    //         carPreference(das->carPreference) {}
    #[allow(unused_variables)]
    pub fn new(hh: Ptr<AGHousehold>, das: Ptr<AGDataAndStatistics>, prevTrips: Ptr<VecDeque<AGTrip>>, prio: i32) -> Self {
        Self {
            myHousehold: hh,
            myStatData: das,
            myPreviousTrips: prevTrips,
            myPartialActivityTrips: VecDeque::new(),
            activityPriority: prio,
            genDone: false,
            timePerKm: das.speedTimePerKm,
            carPreference: das.carPreference,
        }
    }

    /// C++: `void AGActivity::~AGActivity() noexcept` [virtual]
    /// empty destructor
    // virtual ~AGActivity() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        {
            // No-op destructor in Rust for simplicity
        }
    }

    /// C++: `bool AGActivity::isGenerated()`
    /// returns whether the generation could have been well done
    // bool
    // AGActivity::isGenerated() {
    //     return genDone;
    // }
    #[allow(unused_variables)]
    pub fn is_generated(&mut self) -> bool {
        self.genDone
    }

    /// C++: `bool AGActivity::generateTrips()` [virtual, pure virtual]
    /// main function called for trip generation this function is overwritten in every child-class (every activity)
    // bool
    // AGActivity::generateTrips() {
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn generate_trips(&mut self) -> bool {
        true
    }

    /// C++: `int AGActivity::possibleTranspMean(AGPosition)`
    /// determine the possible transportation means, what would be chosen: 1 = foot 2 = bus 4 = car any combination is possible by simply addition of these values (ex. 7 means: 4+2+1 <=> foot, bus and car possible / 5 means: 4+1 <=> only foot and car are possible)
    // int
    // AGActivity::possibleTranspMean(AGPosition destination) {
    //     int FOOT = 1;
    //     int BUS = 2;
    //     int CAR = 4;
    //
    //     int transp = 0;
    //
    //     if (destination.distanceTo(myHousehold->getPosition()) <= myStatData->maxFootDistance) {
    //         transp = FOOT;
    //         if (myHousehold->getCarNbr() != 0) {
    //             transp += CAR;
    //         }
    //         if (destination.minDistanceTo(myStatData->busStations) <= myStatData->maxFootDistance
    //                 && myHousehold->getPosition().minDistanceTo(myStatData->busStations) <= myStatData->maxFootDistance) {
    //             transp += BUS;
    //         }
    //     } else if (myHousehold->getCarNbr() == 0) {
    //         double d1 = destination.distanceTo(myHousehold->getPosition());
    //         double d2 = destination.minDistanceTo(myStatData->busStations) + myHousehold->getPosition().minDistanceTo(myStatData->busStations);
    //
    //         if (d1 > d2) {
    //             transp = BUS;
    //         } else {
    //             transp = FOOT;
    //         }
    //     } else if (myHousehold->getCarNbr() != 0) { //all other cases
    //         if (destination.minDistanceTo(myStatData->busStations) > myStatData->maxFootDistance
    //                 || myHousehold->getPosition().minDistanceTo(myStatData->busStations) > myStatData->maxFootDistance) {
    //             transp = CAR;
    //         } else {
    //             transp = CAR + BUS;
    //         }
    //     }
    //     return transp;
    // }
    #[allow(unused_variables)]
    pub fn possible_transp_mean(&mut self, destination: AGPosition) -> i32 {
        let FOOT = 1;
        let BUS = 2;
        let CAR = 4;

        let mut transp = 0;
        let home = self.myHousehold.m().get_position();

        if destination.distance_to(&home) <= self.myStatData.maxFootDistance {
            transp = FOOT;
            if self.myHousehold.m().get_car_nbr() != 0 {
                transp += CAR;
            }
            if destination.min_distance_to_2(&self.myStatData.busStations) <= self.myStatData.maxFootDistance
                && home.min_distance_to_2(&self.myStatData.busStations) <= self.myStatData.maxFootDistance
            {
                transp += BUS;
            }
        } else if self.myHousehold.m().get_car_nbr() == 0 {
            let d1 = destination.distance_to(&home);
            let d2 = destination.min_distance_to_2(&self.myStatData.busStations) + home.min_distance_to_2(&self.myStatData.busStations);

            if d1 > d2 {
                transp = BUS;
            } else {
                transp = FOOT;
            }
        } else if self.myHousehold.m().get_car_nbr() != 0 { // all other cases
            if destination.min_distance_to_2(&self.myStatData.busStations) > self.myStatData.maxFootDistance
                || home.min_distance_to_2(&self.myStatData.busStations) > self.myStatData.maxFootDistance
            {
                transp = CAR;
            } else {
                transp = CAR + BUS;
            }
        }
        transp
    }

    /// C++: `int AGActivity::availableTranspMeans(AGPosition, AGPosition)`
    /// determine the possible means for a trip from one position to a destination. whether CAR is necessary or not, BUS available or not... 1 = by foot possible. 2 = by bus possible. 0 = by bus or foot NOT possible => only by car. : 4 is useless because it is always possible : 3 = 2 + 1 = means bus and foot possible.
    // int
    // AGActivity::availableTranspMeans(AGPosition from, AGPosition to) {
    //     int FOOT = 1;
    //     int BUS = 2;
    //
    //     int available = 0;
    //
    //     if (from.distanceTo(to) <= myStatData->maxFootDistance) {
    //         available += FOOT;
    //     }
    //     if (from.minDistanceTo(myStatData->busStations) <= myStatData->maxFootDistance
    //             && to.minDistanceTo(myStatData->busStations) <= myStatData->maxFootDistance) {
    //         available += BUS;
    //     }
    //     return available;
    // }
    #[allow(unused_variables)]
    pub fn available_transp_means(&mut self, from: AGPosition, to: AGPosition) -> i32 {
        let FOOT = 1;
        let BUS = 2;

        let mut available = 0;

        if from.distance_to(&to) <= self.myStatData.maxFootDistance {
            available += FOOT;
        }
        if from.min_distance_to_2(&self.myStatData.busStations) <= self.myStatData.maxFootDistance
            && to.min_distance_to_2(&self.myStatData.busStations) <= self.myStatData.maxFootDistance
        {
            available += BUS;
        }
        available
    }

    /// C++: `int AGActivity::timeToDrive(AGPosition, AGPosition)`
    /// evaluation of the needed time for going from one point to an other using the car
    // int
    // AGActivity::timeToDrive(AGPosition from, AGPosition to) {
    //     double dist = from.distanceTo(to);
    //     return (int)(timePerKm * dist / 1000.0);
    // }
    #[allow(unused_variables)]
    pub fn time_to_drive(&mut self, from: AGPosition, to: AGPosition) -> i32 {
        let dist = from.distance_to(&to);
        (self.timePerKm * dist / 1000.0) as i32
    }

    /// C++: `int AGActivity::depHour(AGPosition, AGPosition, int)`
    /// estimates the departure/arrival time given the departure location the arrival location and the wished arrival/departure time
    // int
    // AGActivity::depHour(AGPosition from, AGPosition to, int arrival) {
    //     // ?? departure.addDays(1); // in case of negative time: arrival < timeToDrive
    //     //departure.setDay(0); // days are set to 0 because we want the time in the current day
    //     return (arrival - timeToDrive(from, to));
    // }
    #[allow(unused_variables)]
    pub fn dep_hour(&mut self, from: AGPosition, to: AGPosition, arrival: i32) -> i32 {
        // C++ does not wrap negative times here (the day-wrap line is commented out in AGActivity.cpp)
        arrival - self.time_to_drive(from, to)
    }

    /// C++: `int AGActivity::arrHour(AGPosition, AGPosition, int)`
    // int
    // AGActivity::arrHour(AGPosition from, AGPosition to, int departure) {
    //     return (departure + timeToDrive(from, to));
    // }
    #[allow(unused_variables)]
    pub fn arr_hour(&mut self, from: AGPosition, to: AGPosition, departure: i32) -> i32 {
        departure + self.time_to_drive(from, to)
    }

    /// C++: `int AGActivity::randomTimeBetween(int, int)`
    /// evaluates a random time between the given two time instants
    // int
    // AGActivity::randomTimeBetween(int begin, int end) {
    //     if (0 > begin || begin > end) {
    //         return -1;
    //     }
    //     if (begin == end) {
    //         return begin;
    //     }
    //     int tAlea = RandHelper::rand(end - begin);
    //     return (begin + tAlea);
    // }
    #[allow(unused_variables)]
    pub fn random_time_between(&mut self, begin: i32, end: i32) -> i32 {
        if 0 > begin || begin > end {
            return -1;
        }
        if begin == end {
            return begin;
        }
        let tAlea = utils::common::rand_helper::with_default(|r| r.rand_int(end - begin));
        begin + tAlea
    }

    /// C++: `std::list<AGTrip> & AGActivity::getPartialActivityTrips()`
    // std::list<AGTrip>&
    // AGActivity::getPartialActivityTrips() {
    //     return myPartialActivityTrips;
    // }
    #[allow(unused_variables)]
    pub fn get_partial_activity_trips(&mut self) -> &mut VecDeque<AGTrip> {
        &mut self.myPartialActivityTrips
    }

}


#[cfg(test)]
mod dep_hour_tests {
    use super::*;

    fn pos(x: f64, y: f64) -> AGPosition {
        AGPosition { street: Ptr::null(), position: 0., pos2d: utils::geom::position::Position::new(x, y) }
    }

    // C++ AGActivity.cpp:110-116 (depHour): returns arrival - timeToDrive(from, to); the day-wrap is commented out, so no +24
    #[test]
    fn dep_hour_does_not_wrap_negative_times() {
        let mut das = AGDataAndStatistics::new();
        das.speedTimePerKm = 360.0;
        let mut act = AGActivity::new(Ptr::null(), Ptr::from_mut(&mut das), Ptr::null(), 0);
        // same place: timeToDrive is 0, so the departure equals a negative arrival unchanged
        assert_eq!(act.dep_hour(pos(0., 0.), pos(0., 0.), -100), -100);
        // 1000 m at 360 s/km: timeToDrive = (int)(360 * 1000 / 1000) = 360
        assert_eq!(act.dep_hour(pos(0., 0.), pos(1000., 0.), 100), -260);
    }
}
