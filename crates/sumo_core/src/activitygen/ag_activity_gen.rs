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
        todo!("port AGActivityGen::AGActivityGen")
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
        todo!("port AGActivityGen::importInfoCity")
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
        todo!("port AGActivityGen::makeActivityTrips")
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
        todo!("port AGActivityGen::timeTripValidation")
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
        todo!("port AGActivityGen::generateOutputFile")
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
        todo!("port AGActivityGen::varDepTime")
    }

    /// C++: `AGActivityGen & AGActivityGen::operator=(const AGActivityGen &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &AGActivityGen) -> &mut AGActivityGen {
        todo!("port AGActivityGen::operator=")
    }

}

