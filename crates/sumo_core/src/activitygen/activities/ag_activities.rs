//! Port of activitygen/activities/AGActivities.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ `#define REBUILD_ITERATION_LIMIT 2` (activities/AGActivities.cpp)
const REBUILD_ITERATION_LIMIT: i32 = 2;

/// C++ class `AGActivities` (activitygen/activities/AGActivities.h)
pub struct AGActivities {
    /// trips contains trips as well for one day as for every day, these trips will be regenerated with small variations by ActivityGen at the end of the simulation before generating the trip file
    pub trips: VecDeque<AGTrip>,
    pub myCity: Ptr<AGCity>,
    pub nbrDays: i32,
}

impl AGActivities {
    /// C++: `void AGActivities::AGActivities(AGCity *, int)`
    // AGActivities(AGCity* city, int days) :
    //         myCity(city),
    //         nbrDays(days) {}
    #[allow(unused_variables)]
    pub fn new(city: Ptr<AGCity>, days: i32) -> Self {
        Self { trips: VecDeque::new(), myCity: city, nbrDays: days }
    }

    /// C++: `void AGActivities::addTrip(AGTrip, std::list<AGTrip> *)`
    // void
    // AGActivities::addTrip(AGTrip t, std::list<AGTrip>* tripSet) {
    //     tripSet->push_back(t);
    // }
    #[allow(unused_variables)]
    pub fn add_trip(&mut self, t: AGTrip, tripSet: Ptr<VecDeque<AGTrip>>) {
        self.trips.push_back(t);
    }

    /// C++: `void AGActivities::addTrips(std::list<AGTrip>, std::list<AGTrip> *)`
    // void
    // AGActivities::addTrips(std::list<AGTrip> t, std::list<AGTrip>* tripSet) {
    //     std::list<AGTrip>::iterator it;
    //     for (it = t.begin(); it != t.end(); ++it) {
    //         tripSet->push_back(*it);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_trips(&mut self, t: VecDeque<AGTrip>, tripSet: Ptr<VecDeque<AGTrip>>) {
        let ts = tripSet.m();
        for trip in t {
            ts.push_back(trip);
        }
    }

    /// C++: `void AGActivities::generateActivityTrips()`
    // void
    // AGActivities::generateActivityTrips() {
    //     int numbErr;
    //     /**
    //      * trips due to public transportation
    //      */
    //     numbErr = 0;
    //     std::list<AGBusLine>::iterator itBL;
    //     for (itBL = myCity->busLines.begin(); itBL != myCity->busLines.end(); ++itBL) {
    //         if (! generateBusTraffic(*itBL)) {
    //             ++numbErr;
    //         }
    //     }
    //     if (numbErr != 0) {
    //         std::cerr << "ERROR: " << numbErr << " bus lines couldn't been completely generated ( " << (float)numbErr * 100.0 / (float)myCity->busLines.size() << "% )..." << std::endl;
    //     } else {
    //         std::cout << "no problem during bus line trip generation..." << std::endl;
    //     }
    //
    //     std::cout << "after public transportation: " << trips.size() << std::endl;
    //     /**
    //      * trips due to activities in the city
    //      * @NOTICE: includes people working in work positions out of the city
    //      */
    //     numbErr = 0;
    //     std::list<AGHousehold>::iterator itHH;
    //     for (itHH = myCity->households.begin(); itHH != myCity->households.end(); ++itHH) {
    //         if (! generateTrips(*itHH)) {
    //             ++numbErr;
    //         }
    //     }
    //     if (numbErr != 0) {
    //         std::cout << "WARNING: " << numbErr << " ( " << (float)numbErr * 100.0 / (float)myCity->households.size() << "% ) households' trips haven't been generated: would probably need more iterations for rebuilding..." << std::endl;
    //     } else {
    //         std::cout << "no problem during households' trips generation..." << std::endl;
    //     }
    //
    //     std::cout << "after household activities: " << trips.size() << std::endl;
    //     /**
    //      * trips due to incoming and outgoing traffic
    //      * @WARNING: the outgoing traffic is already done: households in which someone works on a work position that is out of the city.
    //      */
    //     if (! generateInOutTraffic()) {
    //         std::cerr << "ERROR while generating in/Out traffic..." << std::endl;
    //     } else {
    //         std::cout << "no problem during in/out traffic generation..." << std::endl;
    //     }
    //
    //     std::cout << "after incoming/outgoing traffic: " << trips.size() << std::endl;
    //     /**
    //      * random traffic trips
    //      * @NOTICE: this includes uniform and proportional random traffic
    //      */
    //     if (! generateRandomTraffic()) {
    //         std::cerr << "ERROR while generating random traffic..." << std::endl;
    //     } else {
    //         std::cout << "no problem during random traffic generation..." << std::endl;
    //     }
    //
    //     std::cout << "after random traffic: " << trips.size() << std::endl;
    // }
    #[allow(unused_variables)]
    pub fn generate_activity_trips(&mut self) {
        let city = self.myCity; // C++ myCity pointer, copied so the loops below can call &mut self methods
        // trips due to public transportation
        let mut numbErr = 0;
        for bl in city.m().busLines.iter() {
            if !self.generate_bus_traffic(bl.clone()) {
                numbErr += 1;
            }
        }
        if numbErr != 0 {
            let pct = (numbErr as f32 as f64) * 100.0 / (city.m().busLines.len() as f32 as f64);
            eprintln!("ERROR: {} bus lines couldn't been completely generated ( {}% )...", numbErr, utils::common::to_string::f64_to_string_default(pct));
        } else {
            println!("no problem during bus line trip generation...");
        }

        println!("after public transportation: {}", self.trips.len());
        // trips due to activities in the city
        // @NOTICE: includes people working in work positions out of the city
        let mut numbErr = 0;
        for hh in city.m().households.iter_mut() {
            if !self.generate_trips(hh) {
                numbErr += 1;
            }
        }
        if numbErr != 0 {
            let pct = (numbErr as f32 as f64) * 100.0 / (city.m().households.len() as f32 as f64);
            println!("WARNING: {} ( {}% ) households' trips haven't been generated: would probably need more iterations for rebuilding...", numbErr, utils::common::to_string::f64_to_string_default(pct));
        } else {
            println!("no problem during households' trips generation...");
        }

        println!("after household activities: {}", self.trips.len());
        // trips due to incoming and outgoing traffic
        // @WARNING: the outgoing traffic is already done: households in which someone works on a work position that is out of the city.
        if !self.generate_in_out_traffic() {
            eprintln!("ERROR while generating in/Out traffic...");
        } else {
            println!("no problem during in/out traffic generation...");
        }

        println!("after incoming/outgoing traffic: {}", self.trips.len());
        // random traffic trips
        // @NOTICE: this includes uniform and proportional random traffic
        if !self.generate_random_traffic() {
            eprintln!("ERROR while generating random traffic...");
        } else {
            println!("no problem during random traffic generation...");
        }

        println!("after random traffic: {}", self.trips.len());
    }

    /// C++: `bool AGActivities::generateTrips(AGHousehold &)` [private]
    // bool
    // AGActivities::generateTrips(AGHousehold& hh) {
    //     int iteration = 0;
    //     bool generated = false;
    //     std::list<AGTrip> temporaTrips;
    //     while (!generated && iteration < REBUILD_ITERATION_LIMIT) {
    //         if (!temporaTrips.empty()) {
    //             temporaTrips.clear();
    //         }
    //         // Work and school activities
    //         AGWorkAndSchool ws(&hh, &(myCity->statData), &temporaTrips);
    //         generated = ws.generateTrips();
    //         if (!generated) {
    //             hh.regenerate();
    //             ++iteration;
    //             continue;
    //         }
    //         addTrips(ws.getPartialActivityTrips(), &temporaTrips);
    //
    //         // free time activities
    //         AGFreeTime ft(&hh, &(myCity->statData), &temporaTrips, nbrDays);
    //         generated = ft.generateTrips();
    //         if (!generated) {
    //             hh.regenerate();
    //             ++iteration;
    //             continue;
    //         }
    //         addTrips(ft.getPartialActivityTrips(), &temporaTrips);
    //         //cout << "after this hh: " << temporaTrips.size() << " we have: " << trips.size() << endl;
    //         //trips of all activities generated:
    //         addTrips(temporaTrips, &trips);
    //     }
    //     return generated;
    // }
    #[allow(unused_variables)]
    pub fn generate_trips(&mut self, hh: &mut AGHousehold) -> bool {
        let mut iteration = 0;
        let mut generated = false;
        let mut temporaTrips: VecDeque<AGTrip> = VecDeque::new();
        let city = self.myCity;
        while !generated && iteration < REBUILD_ITERATION_LIMIT {
            if !temporaTrips.is_empty() {
                temporaTrips.clear();
            }
            // Work and school activities
            let mut ws = AGWorkAndSchool::new(Ptr::from_mut(&mut *hh), city.m().statData, Ptr::from_mut(&mut temporaTrips));
            generated = ws.generate_trips();
            if !generated {
                hh.regenerate();
                iteration += 1;
                continue;
            }
            let tripSet = Ptr::from_mut(&mut temporaTrips);
            self.add_trips(std::mem::take(ws.get_partial_activity_trips()), tripSet);

            // free time activities
            let mut ft = AGFreeTime::new(Ptr::from_mut(&mut *hh), city.m().statData, Ptr::from_mut(&mut temporaTrips), self.nbrDays);
            generated = ft.generate_trips();
            if !generated {
                hh.regenerate();
                iteration += 1;
                continue;
            }
            let tripSet = Ptr::from_mut(&mut temporaTrips);
            self.add_trips(std::mem::take(ft.get_partial_activity_trips()), tripSet);
            // trips of all activities generated:
            let allTrips = Ptr::from_mut(&mut self.trips);
            self.add_trips(std::mem::take(&mut temporaTrips), allTrips);
        }
        generated
    }

    /// C++: `bool AGActivities::generateBusTraffic(AGBusLine)` [private]
    // bool
    // AGActivities::generateBusTraffic(AGBusLine bl) {
    //     std::list<AGBus>::iterator itB;
    //     std::list<AGPosition>::iterator itS;
    //     /**
    //      * Buses in the first direction
    //      */
    //     for (itB = bl.buses.begin(); itB != bl.buses.end(); ++itB) {
    //         if (bl.stations.size() < 1) {
    //             return false;
    //         }
    //         AGTrip t(bl.stations.front(), bl.stations.back(), *itB, itB->getDeparture());
    //         for (itS = bl.stations.begin(); itS != bl.stations.end(); ++itS) {
    //             if (*itS == t.getDep() || *itS == t.getArr()) {
    //                 continue;
    //             }
    //             t.addLayOver(*itS);
    //         }
    //         trips.push_back(t);
    //     }
    //     /**
    //      * Buses in the return direction
    //      */
    //     //verify that buses return back to the beginning
    //     if (bl.revStations.empty()) {
    //         return true;    //in this case, no return way: everything is ok.
    //     }
    //     for (itB = bl.revBuses.begin(); itB != bl.revBuses.end(); ++itB) {
    //         if (bl.revStations.size() < 1) {
    //             return false;
    //         }
    //         AGTrip t(bl.revStations.front(), bl.revStations.back(), *itB, itB->getDeparture());
    //         for (itS = bl.revStations.begin(); itS != bl.revStations.end(); ++itS) {
    //             if (*itS == t.getDep() || *itS == t.getArr()) {
    //                 continue;
    //             }
    //             t.addLayOver(*itS);
    //         }
    //         trips.push_back(t);
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn generate_bus_traffic(&mut self, bl: AGBusLine) -> bool {
        // buses in the first direction
        for itB in bl.buses.iter() {
            if bl.stations.len() < 1 {
                return false;
            }
            let mut t = AGTrip::new_3(*bl.stations.front().unwrap(), *bl.stations.back().unwrap(), itB.clone(), itB.clone().get_departure());
            for itS in bl.stations.iter() {
                if *itS == t.get_dep() || *itS == t.get_arr() {
                    continue;
                }
                t.add_lay_over(*itS);
            }
            self.trips.push_back(t);
        }
        // buses in the return direction
        // verify that buses return back to the beginning
        if bl.revStations.is_empty() {
            return true; // in this case, no return way: everything is ok.
        }
        for itB in bl.revBuses.iter() {
            if bl.revStations.len() < 1 {
                return false;
            }
            let mut t = AGTrip::new_3(*bl.revStations.front().unwrap(), *bl.revStations.back().unwrap(), itB.clone(), itB.clone().get_departure());
            for itS in bl.revStations.iter() {
                if *itS == t.get_dep() || *itS == t.get_arr() {
                    continue;
                }
                t.add_lay_over(*itS);
            }
            self.trips.push_back(t);
        }
        true
    }

    /// C++: `bool AGActivities::generateInOutTraffic()` [private]
    // bool
    // AGActivities::generateInOutTraffic() {
    //     /**
    //      * outgoing traffic already done by generateTrips():
    //      * people who work out of the city.
    //      * Here are people from outside the city coming to work.
    //      */
    //     if (myCity->peopleIncoming.empty()) {
    //         return true;
    //     }
    //     if (myCity->cityGates.empty()) {
    //         return false;
    //     }
    //     int num = 1;
    //     std::list<AGAdult>::iterator itA;
    //
    //     for (itA = myCity->peopleIncoming.begin(); itA != myCity->peopleIncoming.end(); ++itA) {
    //         int posi = myCity->statData.getRandomCityGateByIncoming();
    //         std::string nom(generateName(num, "carIn"));
    //         AGTrip wayTrip(myCity->cityGates[posi], itA->getWorkPosition().getPosition(), nom, itA->getWorkPosition().getOpening());
    //         //now we put the estimated time of entrance in the city.
    //         wayTrip.setDepTime(wayTrip.estimateDepTime(wayTrip.getTime(), myCity->statData.speedTimePerKm));
    //         AGTrip retTrip(itA->getWorkPosition().getPosition(), myCity->cityGates[posi], nom, itA->getWorkPosition().getClosing());
    //         trips.push_back(wayTrip);
    //         trips.push_back(retTrip);
    //         ++num;
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn generate_in_out_traffic(&mut self) -> bool {
        // outgoing traffic already done by generateTrips(): people who work out of the city.
        // Here are people from outside the city coming to work.
        let city = self.myCity;
        if city.m().peopleIncoming.is_empty() {
            return true;
        }
        if city.m().cityGates.is_empty() {
            return false;
        }
        let mut num = 1;

        for itA in city.m().peopleIncoming.iter() {
            let posi = city.m().statData.m().get_random_city_gate_by_incoming();
            let nom = self.generate_name(num, "carIn".to_string());
            let work = itA.get_work_position();
            let gate = city.m().cityGates[posi as usize];
            let mut wayTrip = AGTrip::new_4(gate, work.get_position(), nom.clone(), work.get_opening());
            // now we put the estimated time of entrance in the city.
            let wayTime = wayTrip.get_time();
            let wayDep = wayTrip.estimate_dep_time(wayTime, city.m().statData.speedTimePerKm);
            wayTrip.set_dep_time(wayDep);
            let retTrip = AGTrip::new_4(work.get_position(), gate, nom, work.get_closing());
            self.trips.push_back(wayTrip);
            self.trips.push_back(retTrip);
            num += 1;
        }
        true
    }

    /// C++: `bool AGActivities::generateRandomTraffic()` [private]
    // bool
    // AGActivities::generateRandomTraffic() {
    //     //total number of trips during the whole simulation
    //     int totalTrips = 0, ttOneDayTrips = 0, ttDailyTrips = 0;
    //     std::list<AGTrip>::iterator it;
    //     for (it = trips.begin(); it != trips.end(); ++it) {
    //         if (it->isDaily()) {
    //             ++ttDailyTrips;
    //         } else {
    //             ++ttOneDayTrips;
    //         }
    //     }
    //     totalTrips = ttOneDayTrips + ttDailyTrips * nbrDays;
    //     //TESTS
    //     std::cout << "Before Random traffic generation (days are still entire):" << std::endl;
    //     std::cout << "- Total number of trips: " << totalTrips << std::endl;
    //     std::cout << "- Total daily trips: " << ttDailyTrips << std::endl;
    //     std::cout << "- Total one-day trips: " << ttOneDayTrips << std::endl;
    //     //END OF TESTS
    //
    //     //random uniform distribution:
    //     int nbrRandUni = (int)((float)totalTrips * myCity->statData.uniformRandomTrafficRate / (1.0f - myCity->statData.uniformRandomTrafficRate));
    //     //TESTS
    //     std::cout << "added uniform random trips: " << nbrRandUni << std::endl;
    //     //END OF TESTS
    //     for (int i = 0; i < nbrRandUni; ++i) {
    //         AGPosition dep(myCity->getRandomStreet());
    //         AGPosition arr(myCity->getRandomStreet());
    //         AGTime depTime(RandHelper::rand(nbrDays * 86400));
    //         AGTrip rdtr(dep, arr, generateName(i, "randUni"), depTime.getTime() % 86400, depTime.getDay() + 1);
    //         rdtr.setType("random");
    //         trips.push_back(rdtr);
    //     }
    //
    //     //random proportional distribution:
    //     //float proportionalPercentage = 0.05f;
    //     //TODO generate a proportionally distributed random traffic
    //
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn generate_random_traffic(&mut self) -> bool {
        // total number of trips during the whole simulation
        let mut totalTrips = 0;
        let mut ttOneDayTrips = 0;
        let mut ttDailyTrips = 0;
        for it in self.trips.iter() {
            if it.is_daily() {
                ttDailyTrips += 1;
            } else {
                ttOneDayTrips += 1;
            }
        }
        totalTrips = ttOneDayTrips + ttDailyTrips * self.nbrDays;
        // TESTS
        println!("Before Random traffic generation (days are still entire):");
        println!("- Total number of trips: {}", totalTrips);
        println!("- Total daily trips: {}", ttDailyTrips);
        println!("- Total one-day trips: {}", ttOneDayTrips);
        // END OF TESTS

        let city = self.myCity;
        // random uniform distribution:
        let rate = city.m().statData.uniformRandomTrafficRate;
        let nbrRandUni = ((totalTrips as f32) as f64 * rate / (1.0f32 as f64 - rate)) as i32;
        // TESTS
        println!("added uniform random trips: {}", nbrRandUni);
        // END OF TESTS
        for i in 0..nbrRandUni {
            let dep = AGPosition::new_2(city.m().get_random_street());
            let arr = AGPosition::new_2(city.m().get_random_street());
            let mut depTime = AGTime::new(utils::common::rand_helper::with_default(|r| r.rand_int(self.nbrDays * 86400)));
            let depSec = depTime.get_time() % 86400;
            let depDay = depTime.get_day() + 1;
            let name = self.generate_name(i, "randUni".to_string());
            let mut rdtr = AGTrip::new_5(dep, arr, name, depSec, depDay);
            rdtr.set_type("random".to_string());
            self.trips.push_back(rdtr);
        }

        // random proportional distribution:
        // float proportionalPercentage = 0.05f;
        // TODO generate a proportionally distributed random traffic

        true
    }

    /// C++: `std::string AGActivities::generateName(int, std::string)` [private]
    /// generates car names, given the unique (number, prefix)
    // std::string
    // AGActivities::generateName(int i, std::string prefix) {
    //     std::ostringstream os;
    //     os << i;
    //     return prefix + os.str();
    // }
    #[allow(unused_variables)]
    pub fn generate_name(&mut self, i: i32, prefix: String) -> String {
        let mut os = String::new();
        os.push_str(&i.to_string());
        prefix + &os
    }

}

