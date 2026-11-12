//! Port of activitygen/city/AGBusLine.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `AGBusLine` (activitygen/city/AGBusLine.h)
#[derive(Clone)]
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
        AGBusLine {
            stations: VecDeque::new(),
            revStations: VecDeque::new(),
            buses: VecDeque::new(),
            revBuses: VecDeque::new(),
            lineNumber: lineNr,
            maxTripTime: 0,
            busNbr: 0,
        }
    }

    /// C++: `void AGBusLine::setMaxTripTime(int)`
    // void
    // AGBusLine::setMaxTripTime(int time) {
    //     this->maxTripTime = time;
    // }
    #[allow(unused_variables)]
    pub fn set_max_trip_time(&mut self, time: i32) {
        self.maxTripTime = time;
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
        self.busNbr = 0;
        let mut it1: usize = 0; //iterator on buses in the first direction
        let mut it2: usize = 0; //iterator on buses in the second direction

        // buses on the road or in the parking of the corresponding end: int: the time of availability
        let mut drivingBuses1: VecDeque<(i32, String)> = VecDeque::new();
        let mut drivingBuses2: VecDeque<(i32, String)> = VecDeque::new();

        while it1 != self.buses.len() && it2 != self.revBuses.len() {
            if self.buses[it1].get_departure() > self.revBuses[it2].get_departure() {
                if drivingBuses2.is_empty() {
                    let d = self.revBuses[it2].get_departure();
                    let n = self.create_name();
                    drivingBuses2.push_front((d, n));
                } else if drivingBuses2.front().unwrap().0 > self.revBuses[it2].get_departure() {
                    let d = self.revBuses[it2].get_departure();
                    let n = self.create_name();
                    drivingBuses2.push_front((d, n));
                }
                //here the first in drivingBuses2 is available for the trip
                let name = drivingBuses2.front().unwrap().1.clone();
                self.revBuses[it2].set_name(name);
                drivingBuses2.pop_front();
                //the same bus will be available for the main direction after some time (see function getReady):
                let dep = self.revBuses[it2].get_departure();
                let ready = self.get_ready(dep);
                let nm = self.revBuses[it2].get_name();
                drivingBuses1.push_back((ready, nm));
                it2 += 1;
            } else {
                if drivingBuses1.is_empty() {
                    let d = self.buses[it1].get_departure();
                    let n = self.create_name();
                    drivingBuses1.push_front((d, n));
                } else if drivingBuses1.front().unwrap().0 > self.buses[it1].get_departure() {
                    let d = self.buses[it1].get_departure();
                    let n = self.create_name();
                    drivingBuses1.push_front((d, n));
                }
                //here the first in drivingBuses1 is available for the trip
                let name = drivingBuses1.front().unwrap().1.clone();
                self.buses[it1].set_name(name);
                drivingBuses1.pop_front();
                //the same bus will be available for the return way after some time (see function getReady):
                let dep = self.buses[it1].get_departure();
                let ready = self.get_ready(dep);
                let nm = self.buses[it1].get_name();
                drivingBuses2.push_back((ready, nm));
                it1 += 1;
            }
        }
        if it1 != self.buses.len() {
            if drivingBuses1.is_empty() {
                let n = self.create_name();
                self.buses[it1].set_name(n);
            } else if drivingBuses1.front().unwrap().0 > self.buses[it1].get_departure() {
                let n = self.create_name();
                self.buses[it1].set_name(n);
            } else {
                let name = drivingBuses1.front().unwrap().1.clone();
                self.buses[it1].set_name(name);
                drivingBuses1.pop_front();
            }
            it1 += 1;
        }
        if it2 != self.revBuses.len() {
            if drivingBuses2.is_empty() {
                let n = self.create_name();
                self.revBuses[it2].set_name(n);
            } else if drivingBuses2.front().unwrap().0 > self.revBuses[it2].get_departure() {
                let n = self.create_name();
                self.revBuses[it2].set_name(n);
            } else {
                let name = drivingBuses2.front().unwrap().1.clone();
                self.revBuses[it2].set_name(name);
                drivingBuses2.pop_front();
            }
            it2 += 1;
        }
    }

    /// C++: `int AGBusLine::nbrBuses()`
    // int
    // AGBusLine::nbrBuses() {
    //     return static_cast<int>(buses.size());
    // }
    #[allow(unused_variables)]
    pub fn nbr_buses(&mut self) -> i32 {
        self.buses.len() as i32
    }

    /// C++: `void AGBusLine::locateStation(AGPosition)`
    // void
    // AGBusLine::locateStation(AGPosition pos) {
    //     stations.push_back(pos);
    // }
    #[allow(unused_variables)]
    pub fn locate_station(&mut self, pos: AGPosition) {
        self.stations.push_back(pos);
    }

    /// C++: `void AGBusLine::locateRevStation(AGPosition)`
    // void
    // AGBusLine::locateRevStation(AGPosition pos) {
    //     revStations.push_back(pos);
    // }
    #[allow(unused_variables)]
    pub fn locate_rev_station(&mut self, pos: AGPosition) {
        self.revStations.push_back(pos);
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
        let mut t = start;
        while t < stop {
            self.buses.push_back(AGBus::new_2(t)); //one direction
            self.revBuses.push_back(AGBus::new_2(t)); //return direction
            t += rate;
        }
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
        println!("\n ----------- BUS LINE {} PRINTING -------------\n", self.lineNumber);
        println!("\n -------------------------- First way ---------------------------\n");
        for it in self.buses.iter() {
            it.print();
        }
        println!("\n -------------------------- Second way --------------------------\n");
        for it in self.revBuses.iter() {
            it.print();
        }
        println!("\n ----------------------------------------------------------------\n");
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
        self.busNbr += 1; //initialized in setBusNames()
        format!("bl{}b{}", self.lineNumber, self.busNbr)
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
        let mut current = AGTime::new(time);
        current.add_seconds(self.maxTripTime);
        current.add_minutes(15); // PAUSE_TIME: time (in minutes) a bus waits before going in the opposite direction
        current.get_time()
    }

}

