//! Port of activitygen/AGActivityGen.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// Central object handling City, Activities and Trips
/// C++ class `AGActivityGen` (activitygen/AGActivityGen.h)
pub struct AGActivityGen {
    pub inputFile: String,
    /// The generated routes
    pub outputFile: Ptr<OutputDevice>,
    pub net: Ptr<RONet>,
    pub city: AGCity,
    pub durationInDays: i32,
    pub beginTime: i32,
    pub endTime: i32,
}

impl AGActivityGen {
    /// C++: `void AGActivityGen::AGActivityGen(std::string, OutputDevice &, RONet *)`
    /// Constructor
    // AGActivityGen(std::string input, OutputDevice& output, RONet* net) :
    //         inputFile(input),
    //         outputFile(output),
    //         net(net),
    //         //activities(),
    //         city(net) {}
    #[allow(unused_variables)]
    pub fn new(input: String, output: &mut OutputDevice, net: Ptr<RONet>) -> Self {
        Self {
            inputFile: input,
            outputFile: Ptr::from_mut(output),
            net,
            city: AGCity::new(net),
            // C++ leaves these uninitialized; they are set by makeActivityTrips
            durationInDays: 0,
            beginTime: 0,
            endTime: 0,
        }
    }

    /// C++: `void AGActivityGen::importInfoCity()`
    /// build the internal city
    // void
    // AGActivityGen::importInfoCity() {
    //     AGActivityGenHandler handler(city, net);
    //     PROGRESS_BEGIN_MESSAGE(TL("Reading input"));
    //     if (!XMLSubSys::runParser(handler, inputFile)) {
    //         PROGRESS_FAILED_MESSAGE();
    //         throw ProcessError();
    //     } else {
    //         PROGRESS_DONE_MESSAGE();
    //     }
    //
    //     PROGRESS_BEGIN_MESSAGE(TL("Consolidating statistics"));
    //     city.statData.consolidateStat(); //some maps are still not
    //     PROGRESS_DONE_MESSAGE();
    //
    //     PROGRESS_BEGIN_MESSAGE(TL("Building street representation"));
    //     city.completeStreets();
    //     PROGRESS_DONE_MESSAGE();
    //
    //     PROGRESS_BEGIN_MESSAGE(TL("Generating work positions"));
    //     city.generateWorkPositions();
    //     PROGRESS_DONE_MESSAGE();
    //
    //     PROGRESS_BEGIN_MESSAGE(TL("Building bus lines"));
    //     city.completeBusLines();
    //     PROGRESS_DONE_MESSAGE();
    //
    //
    //     PROGRESS_BEGIN_MESSAGE(TL("Generating population"));
    //     city.generatePopulation();
    //     PROGRESS_DONE_MESSAGE();
    //
    //     PROGRESS_BEGIN_MESSAGE(TL("Allocating schools"));
    //     city.schoolAllocation();
    //     PROGRESS_DONE_MESSAGE();
    //
    //     PROGRESS_BEGIN_MESSAGE(TL("Allocating work places"));
    //     city.workAllocation();
    //     PROGRESS_DONE_MESSAGE();
    //
    //     PROGRESS_BEGIN_MESSAGE(TL("Allocating car places"));
    //     city.carAllocation();
    //     PROGRESS_DONE_MESSAGE();
    // }
    #[allow(unused_variables)]
    pub fn import_info_city(&mut self) {
        let mut handler = AGActivityGenHandler::new(&mut self.city, self.net);
        progress_begin_message("Reading input");
        // shortcut: AGActivityGenHandler does not implement SaxHandler yet, so XMLSubSys::runParser cannot run it, upgrade once ag_activity_gen_handler.rs is ported
        panic!("AGActivityGen::importInfoCity: XML parsing of '{}' needs AGActivityGenHandler as a SaxHandler", self.inputFile);

        progress_begin_message("Consolidating statistics");
        self.city.statData.m().consolidate_stat(); // some maps are still not
        progress_done_message();

        progress_begin_message("Building street representation");
        self.city.complete_streets();
        progress_done_message();

        progress_begin_message("Generating work positions");
        self.city.generate_work_positions();
        progress_done_message();

        progress_begin_message("Building bus lines");
        self.city.complete_bus_lines();
        progress_done_message();

        progress_begin_message("Generating population");
        self.city.generate_population();
        progress_done_message();

        progress_begin_message("Allocating schools");
        self.city.school_allocation();
        progress_done_message();

        progress_begin_message("Allocating work places");
        self.city.work_allocation();
        progress_done_message();

        progress_begin_message("Allocating car places");
        self.city.car_allocation();
        progress_done_message();
    }

    /// C++: `void AGActivityGen::makeActivityTrips(int, int, int)`
    /// build activities and trips of the population and generate routes
    // void
    // AGActivityGen::makeActivityTrips(int days, int beginSec, int endSec) {
    //     durationInDays = days;
    //     beginTime = beginSec;
    //     endTime = endSec;
    //     /**
    //      * making the activity trips
    //      */
    //     AGActivities acts(&city, durationInDays + 1);
    //     acts.generateActivityTrips();
    //
    //     /**
    //      * validating the trips with the simulation's time limits
    //      */
    //     //list<Trip>* trips = &(acts.trips);
    //     std::list<AGTrip> expTrips;
    //     std::map<std::string, int> carUsed;
    //     std::list<AGTrip>::iterator it;
    //     //multiplication of days
    //     for (it = acts.trips.begin(); it != acts.trips.end(); ++it) {
    //         if (it->isDaily()) {
    //             for (int currday = 1; currday < durationInDays + 2; ++currday) {
    //                 AGTrip tr(it->getDep(), it->getArr(), it->getVehicleName(), it->getTime(), currday);
    //                 tr.setType(it->getType());
    //                 if (carUsed.find(tr.getVehicleName()) != carUsed.end()) {
    //                     ++carUsed.find(tr.getVehicleName())->second;
    //                 } else {
    //                     carUsed[tr.getVehicleName()] = 1;
    //                 }
    //                 std::ostringstream os;
    //                 os << tr.getVehicleName() << ":" << carUsed.find(tr.getVehicleName())->second;
    //                 tr.setVehicleName(os.str());
    //                 tr.addLayOverWithoutDestination(*it); //intermediate destinations are taken in account too
    //                 varDepTime(tr); //slight variation on each "default" car
    //                 if (timeTripValidation(tr)) {
    //                     expTrips.push_back(tr);
    //                 }
    //                 //else
    //                 //std::cout << "trop tard 1 pour " << tr.getVehicleName() << " " << tr.getTime() << " day: " << tr.getDay() << std::endl;
    //             }
    //         } else {
    //             AGTrip tr(it->getDep(), it->getArr(), it->getVehicleName(), it->getTime(), it->getDay());
    //             tr.setType(it->getType());
    //             if (carUsed.find(tr.getVehicleName()) != carUsed.end()) {
    //                 ++carUsed.find(tr.getVehicleName())->second;
    //             } else {
    //                 carUsed[tr.getVehicleName()] = 1;
    //             }
    //             std::ostringstream os;
    //             os << tr.getVehicleName() << ":" << carUsed.find(tr.getVehicleName())->second;
    //             tr.setVehicleName(os.str());
    //             tr.addLayOverWithoutDestination(*it); //intermediate destinations are taken in account too
    //             varDepTime(tr); //slight variation on each "default" car
    //             if (timeTripValidation(tr)) {
    //                 expTrips.push_back(tr);
    //             }
    //             //else
    //             //std::cout << "trop tard 2 pour " << tr.getVehicleName() << " " << tr.getTime() << " day: " << tr.getDay() << std::endl;
    //         }
    //     }
    //
    //     std::cout << "total trips generated: " << acts.trips.size() << std::endl;
    //     std::cout << "total trips finally taken: " << expTrips.size() << std::endl;
    //
    //     /**
    //      * re-ordering of trips: SUMO needs routes ordered by departure time.
    //      */
    //     expTrips.sort(); //natural order of trips
    //     std::cout << "...sorted by departure time.\n" << std::endl;
    //
    //     /**
    //      * trip file generation
    //      */
    //     generateOutputFile(expTrips);
    // }
    #[allow(unused_variables)]
    pub fn make_activity_trips(&mut self, days: i32, beginTime: i32, endTime: i32) {
        self.durationInDays = days;
        self.beginTime = beginTime;
        self.endTime = endTime;
        // making the activity trips
        let mut acts = AGActivities::new(Ptr::from_mut(&mut self.city), self.durationInDays + 1);
        acts.generate_activity_trips();

        // validating the trips with the simulation's time limits
        let mut expTrips: VecDeque<AGTrip> = VecDeque::new();
        let mut carUsed: BTreeMap<String, i32> = BTreeMap::new();
        // multiplication of days
        for it in acts.trips.iter_mut() {
            if it.is_daily() {
                for currday in 1..self.durationInDays + 2 {
                    let mut tr = AGTrip::new_5(it.get_dep(), it.get_arr(), it.get_vehicle_name().to_string(), it.get_time(), currday);
                    tr.set_type(it.get_type().to_string());
                    let name = tr.get_vehicle_name().to_string();
                    let count = *carUsed.entry(name.clone()).and_modify(|c| *c += 1).or_insert(1);
                    tr.set_vehicle_name(format!("{}:{}", name, count));
                    tr.add_lay_over_without_destination(it); // intermediate destinations are taken in account too
                    self.var_dep_time(&mut tr); // slight variation on each "default" car
                    if self.time_trip_validation(&tr) {
                        expTrips.push_back(tr);
                    }
                }
            } else {
                let mut tr = AGTrip::new_5(it.get_dep(), it.get_arr(), it.get_vehicle_name().to_string(), it.get_time(), it.get_day());
                tr.set_type(it.get_type().to_string());
                let name = tr.get_vehicle_name().to_string();
                let count = *carUsed.entry(name.clone()).and_modify(|c| *c += 1).or_insert(1);
                tr.set_vehicle_name(format!("{}:{}", name, count));
                tr.add_lay_over_without_destination(it); // intermediate destinations are taken in account too
                self.var_dep_time(&mut tr); // slight variation on each "default" car
                if self.time_trip_validation(&tr) {
                    expTrips.push_back(tr);
                }
            }
        }

        println!("total trips generated: {}", acts.trips.len());
        println!("total trips finally taken: {}", expTrips.len());

        // re-ordering of trips: SUMO needs routes ordered by departure time.
        // std::list::sort is stable; sort_by is stable too, with operator< as lt_op
        expTrips.make_contiguous().sort_by(|a, b| {
            if a.lt_op(b) {
                std::cmp::Ordering::Less
            } else if b.lt_op(a) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        });
        println!("...sorted by departure time.\n");

        // trip file generation
        self.generate_output_file(&mut expTrips);
    }

    /// C++: `bool AGActivityGen::timeTripValidation(const AGTrip &) const` [const, protected]
    /// validation: compatibility of the given trip
    // bool
    // AGActivityGen::timeTripValidation(const AGTrip& trip) const {
    //     if (trip.getDay() > durationInDays + 1) {
    //         return false;
    //     }
    //     if (trip.getDay() == 1) { //first day
    //         if (trip.getTime() < beginTime) {
    //             return false;
    //         }
    //         if (durationInDays == 0 && trip.getTime() > endTime) {
    //             return false;
    //         }
    //     }
    //     if (trip.getDay() == durationInDays + 1) { //last day
    //         if (trip.getTime() > endTime) {
    //             return false;
    //         }
    //         if (durationInDays == 0 && trip.getTime() < beginTime) {
    //             return false;
    //         }
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn time_trip_validation(&self, trip: &AGTrip) -> bool {
        if trip.get_day() > self.durationInDays + 1 {
            return false;
        }
        if trip.get_day() == 1 { // first day
            if trip.get_time() < self.beginTime {
                return false;
            }
            if self.durationInDays == 0 && trip.get_time() > self.endTime {
                return false;
            }
        }
        if trip.get_day() == self.durationInDays + 1 { // last day
            if trip.get_time() > self.endTime {
                return false;
            }
            if self.durationInDays == 0 && trip.get_time() < self.beginTime {
                return false;
            }
        }
        true
    }

    /// C++: `void AGActivityGen::generateOutputFile(std::list<AGTrip> &)` [protected]
    /// generate the output file (trips or routes) using a trip list
    // void
    // AGActivityGen::generateOutputFile(std::list<AGTrip>& trips) {
    //     AGActivityTripWriter atw(outputFile);
    //     if (trips.size() != 0) {
    //         std::list<AGTrip>::iterator it;
    //         //variables for TESTS:
    //         int firstTrip = trips.front().getTime() + trips.front().getDay() * 86400;
    //         int lastTrip = trips.front().getTime() + trips.front().getDay() * 86400;
    //         std::map<int, int> histogram;
    //         for (int i = 0; i < 100; ++i) {
    //             histogram[i] = 0;
    //         }
    //         //END var TESTS
    //         for (it = trips.begin(); it != trips.end(); ++it) {
    //             atw.addTrip(*it);
    //             //TEST
    //             if (it->getTime() + 86400 * it->getDay() > lastTrip) {
    //                 lastTrip = it->getTime() + 86400 * it->getDay();
    //             }
    //             if (it->getTime() + 86400 * it->getDay() < firstTrip) {
    //                 firstTrip = it->getTime() + 86400 * it->getDay();
    //             }
    //             //++histogram[((it->getDay()-1)*86400 + it->getTime())/3600];
    //             ++histogram[(it->getTime()) / 3600];
    //             //END TEST
    //         }
    //         //PRINT TEST
    //         AGTime first(firstTrip);
    //         AGTime last(lastTrip);
    //         std::cout << "first real trip: " << first.getDay() << ", " << first.getHour() << ":" << first.getMinute() << ":" << first.getSecond() << std::endl;
    //         std::cout << "last real trip: " << last.getDay() << ", " << last.getHour() << ":" << last.getMinute() << ":" << last.getSecond() << std::endl;
    //         for (int i = 0; i < 100; ++i) {
    //             if (histogram[i] > 0) {
    //                 std::cout << "histogram[ hour " << i << " ] = " << histogram[i] << std::endl;
    //             }
    //         }
    //     } else {
    //         std::cout << "No real trips were generated" << std::endl;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn generate_output_file(&mut self, trips: &mut VecDeque<AGTrip>) {
        let mut atw = AGActivityTripWriter::new(self.outputFile.m());
        if trips.len() != 0 {
            // variables for TESTS:
            let mut firstTrip = trips.front().unwrap().get_time() + trips.front().unwrap().get_day() * 86400;
            let mut lastTrip = trips.front().unwrap().get_time() + trips.front().unwrap().get_day() * 86400;
            let mut histogram: BTreeMap<i32, i32> = BTreeMap::new();
            for i in 0..100 {
                histogram.insert(i, 0);
            }
            // END var TESTS
            for it in trips.iter() {
                atw.add_trip(it);
                // TEST
                if it.get_time() + 86400 * it.get_day() > lastTrip {
                    lastTrip = it.get_time() + 86400 * it.get_day();
                }
                if it.get_time() + 86400 * it.get_day() < firstTrip {
                    firstTrip = it.get_time() + 86400 * it.get_day();
                }
                *histogram.entry(it.get_time() / 3600).or_insert(0) += 1;
                // END TEST
            }
            // PRINT TEST
            let mut first = AGTime::new(firstTrip);
            let mut last = AGTime::new(lastTrip);
            println!("first real trip: {}, {}:{}:{}", first.get_day(), first.get_hour(), first.get_minute(), first.get_second());
            println!("last real trip: {}, {}:{}:{}", last.get_day(), last.get_hour(), last.get_minute(), last.get_second());
            for i in 0..100 {
                let count = histogram.get(&i).copied().unwrap_or(0);
                if count > 0 {
                    println!("histogram[ hour {} ] = {}", i, count);
                }
            }
        } else {
            println!("No real trips were generated");
        }
    }

    /// C++: `void AGActivityGen::varDepTime(AGTrip &) const` [const, protected]
    /// introduce a slight variation into the departure time of "default" vehicles
    // void
    // AGActivityGen::varDepTime(AGTrip& trip) const {
    //     if (trip.getType() != "default") {
    //         return;
    //     }
    //     //buses are on time and random are already spread
    //     int variation = (int)RandHelper::randNorm(0, city.statData.departureVariation);
    //     AGTime depTime(trip.getDay(), 0, 0, trip.getTime());
    //     depTime += variation;
    //     if (depTime.getDay() > 0) {
    //         trip.setDay(depTime.getDay());
    //         trip.setDepTime(depTime.getSecondsInCurrentDay());
    //     } else {
    //         trip.setDay(1);
    //         trip.setDepTime(0);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn var_dep_time(&self, trip: &mut AGTrip) {
        if trip.get_type() != "default" {
            return;
        }
        // buses are on time and random are already spread
        let variation = utils::common::rand_helper::with_default(|r| r.rand_norm(0., self.city.statData.departureVariation)) as i32;
        let mut depTime = AGTime::new_4(trip.get_day(), 0, 0, trip.get_time());
        depTime.add_assign_op_2(variation);
        let depDay = depTime.get_day();
        if depDay > 0 {
            trip.set_day(depDay);
            trip.set_dep_time(depTime.get_seconds_in_current_day());
        } else {
            trip.set_day(1);
            trip.set_dep_time(0);
        }
    }

    /// C++: `AGActivityGen & AGActivityGen::operator=(const AGActivityGen &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &AGActivityGen) -> &mut AGActivityGen {
        panic!("AGActivityGen assignment operator is private and never defined in C++")
    }

}

