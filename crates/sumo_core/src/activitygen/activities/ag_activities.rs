//! Port of activitygen/activities/AGActivities.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

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
        todo!("port AGActivities::AGActivities")
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
        todo!("port AGActivities::addTrips")
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
        todo!("port AGActivities::generateActivityTrips")
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
        todo!("port AGActivities::generateTrips")
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
        todo!("port AGActivities::generateBusTraffic")
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
        todo!("port AGActivities::generateInOutTraffic")
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
        todo!("port AGActivities::generateRandomTraffic")
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

