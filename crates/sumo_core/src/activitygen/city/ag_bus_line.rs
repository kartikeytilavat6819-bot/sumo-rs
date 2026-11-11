//! Port of activitygen/city/AGBusLine.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `AGBusLine` (activitygen/city/AGBusLine.h)
pub struct AGBusLine {
    pub stations: VecDeque<AGPosition>,
    pub revStations: VecDeque<AGPosition>,
    pub buses: VecDeque<AGBus>,
    pub revBuses: VecDeque<AGBus>,
    pub lineNumber: String,
    pub maxTripTime: i32,
    pub busNbr: i32,
}

impl AGBusLine {
    /// C++: `void AGBusLine::AGBusLine(std::string)`
    // AGBusLine(std::string lineNr) :
    //         lineNumber(lineNr) {}
    #[allow(unused_variables)]
    pub fn new(lineNr: String) -> Self {
        todo!("port AGBusLine::AGBusLine")
    }

    /// C++: `void AGBusLine::setMaxTripTime(int)`
    // void
    // AGBusLine::setMaxTripTime(int time) {
    //     this->maxTripTime = time;
    // }
    #[allow(unused_variables)]
    pub fn set_max_trip_time(&mut self, time: i32) {
        todo!("port AGBusLine::setMaxTripTime")
    }

    /// C++: `void AGBusLine::setBusNames()`
    // void
    // AGBusLine::setBusNames() {
    //     busNbr = 0;
    //     std::list<AGBus>::iterator it1 = buses.begin(); //iterator on buses in the first direction
    //     std::list<AGBus>::iterator it2 = revBuses.begin(); //iterator on buses in the second direction
    //
    //     std::list<std::pair<int, std::string> > drivingBuses1, drivingBuses2; //buses on the road or in the parking of the corresponding end: int: the time of availability
    //
    //     while (it1 != buses.end() && it2 != revBuses.end()) {
    //         if (it1->getDeparture() > it2->getDeparture()) {
    //             if (drivingBuses2.size() == 0) {
    //                 drivingBuses2.push_front(make_pair(it2->getDeparture(), createName()));
    //             } else if (drivingBuses2.front().first > it2->getDeparture()) {
    //                 drivingBuses2.push_front(make_pair(it2->getDeparture(), createName()));
    //             }
    //             //here the first in drivingBuses2 is available for the trip
    //             it2->setName(drivingBuses2.front().second);
    //             drivingBuses2.pop_front();
    //             //the same bus will be available for the main direction after some time (see function getReady):
    //             drivingBuses1.push_back(make_pair(getReady(it2->getDeparture()), it2->getName()));
    //             it2++;
    //         } else {
    //             if (drivingBuses1.size() == 0) {
    //                 drivingBuses1.push_front(make_pair(it1->getDeparture(), createName()));
    //             } else if (drivingBuses1.front().first > it1->getDeparture()) {
    //                 drivingBuses1.push_front(make_pair(it1->getDeparture(), createName()));
    //             }
    //             //here the first in drivingBuses1 is available for the trip
    //             it1->setName(drivingBuses1.front().second);
    //             drivingBuses1.pop_front();
    //             //the same bus will be available for the return way after some time (see function getReady):
    //             drivingBuses2.push_back(make_pair(getReady(it1->getDeparture()), it1->getName()));
    //             it1++;
    //         }
    //     }
    //     if (it1 != buses.end()) {
    //         if (drivingBuses1.size() == 0) {
    //             it1->setName(createName());
    //         } else if (drivingBuses1.front().first > it1->getDeparture()) {
    //             it1->setName(createName());
    //         } else {
    //             it1->setName(drivingBuses1.front().second);
    //             drivingBuses1.pop_front();
    //         }
    //         it1++;
    //     }
    //     if (it2 != revBuses.end()) {
    //         if (drivingBuses2.size() == 0) {
    //             it2->setName(createName());
    //         } else if (drivingBuses2.front().first > it2->getDeparture()) {
    //             it2->setName(createName());
    //         } else {
    //             it2->setName(drivingBuses2.front().second);
    //             drivingBuses2.pop_front();
    //         }
    //         it2++;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_bus_names(&mut self) {
        todo!("port AGBusLine::setBusNames")
    }

    /// C++: `int AGBusLine::nbrBuses()`
    // int
    // AGBusLine::nbrBuses() {
    //     return static_cast<int>(buses.size());
    // }
    #[allow(unused_variables)]
    pub fn nbr_buses(&mut self) -> i32 {
        todo!("port AGBusLine::nbrBuses")
    }

    /// C++: `void AGBusLine::locateStation(AGPosition)`
    // void
    // AGBusLine::locateStation(AGPosition pos) {
    //     stations.push_back(pos);
    // }
    #[allow(unused_variables)]
    pub fn locate_station(&mut self, pos: AGPosition) {
        todo!("port AGBusLine::locateStation")
    }

    /// C++: `void AGBusLine::locateRevStation(AGPosition)`
    // void
    // AGBusLine::locateRevStation(AGPosition pos) {
    //     revStations.push_back(pos);
    // }
    #[allow(unused_variables)]
    pub fn locate_rev_station(&mut self, pos: AGPosition) {
        todo!("port AGBusLine::locateRevStation")
    }

    /// C++: `void AGBusLine::generateBuses(int, int, int)`
    // void
    // AGBusLine::generateBuses(int start, int stop, int rate) {
    //     int t = start;
    //     while (t < stop) {
    //         buses.push_back(AGBus(t)); //one direction
    //         revBuses.push_back(AGBus(t)); //return direction
    //         t += rate;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn generate_buses(&mut self, start: i32, stop: i32, rate: i32) {
        todo!("port AGBusLine::generateBuses")
    }

    /// C++: `void AGBusLine::printBuses()`
    // void
    // AGBusLine::printBuses() {
    //     std::list<AGBus>::iterator it;
    //     std::cout << "\n ----------- BUS LINE " << lineNumber << " PRINTING -------------\n" << std::endl;
    //     std::cout << "\n -------------------------- First way ---------------------------\n" << std::endl;
    //     for (it = buses.begin(); it != buses.end(); ++it) {
    //         it->print();
    //     }
    //     std::cout << "\n -------------------------- Second way --------------------------\n" << std::endl;
    //     for (it = revBuses.begin(); it != revBuses.end(); ++it) {
    //         it->print();
    //     }
    //     std::cout << "\n ----------------------------------------------------------------\n" << std::endl;
    // }
    #[allow(unused_variables)]
    pub fn print_buses(&mut self) {
        todo!("port AGBusLine::printBuses")
    }

    /// C++: `std::string AGBusLine::createName()` [private]
    /// Returns : a name for a new Bus. unique for the city.
    // std::string
    // AGBusLine::createName() {
    //     ++busNbr; //initialized in setBusNames()
    //     std::ostringstream os;
    //     os << busNbr;
    //     return "bl" + lineNumber + "b" + os.str();
    // }
    #[allow(unused_variables)]
    pub fn create_name(&mut self) -> String {
        todo!("port AGBusLine::createName")
    }

    /// C++: `int AGBusLine::getReady(int)` [private]
    /// Returns : time at which it will be ready for going in the opposite direction
    // int
    // AGBusLine::getReady(int time) {
    //     AGTime current(time);
    //     current.addSeconds(maxTripTime);
    //     current.addMinutes(PAUSE_TIME);
    //     return current.getTime();
    // }
    #[allow(unused_variables)]
    pub fn get_ready(&mut self, time: i32) -> i32 {
        todo!("port AGBusLine::getReady")
    }

}

