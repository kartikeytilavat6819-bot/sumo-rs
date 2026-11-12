//! Port of activitygen/activities/AGTrip.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `AGTrip` (activitygen/activities/AGTrip.h)
pub struct AGTrip {
    pub myFrom: AGPosition,
    pub myTo: AGPosition,
    pub myDepTime: i32,
    pub myVehicle: String,
    /// indicates if it is a bus or a car (or any type) "bus", "default" or "random" (which is a kind of default)
    pub myType: String,
    /// if everyday : 0 else : number of the day ( != 0 )
    pub myDay: i32,
    pub myPassBy: VecDeque<AGPosition>,
}

impl AGTrip {
    /// C++: `void AGTrip::AGTrip(AGPosition, AGPosition, int)`
    // AGTrip(AGPosition from, AGPosition to, int at) : //vehicle not specified
    //         myFrom(from),
    //         myTo(to),
    //         myDepTime(at),
    //         myType("default"),
    //         myDay(0) {}
    #[allow(unused_variables)]
    pub fn new(from: AGPosition, to: AGPosition, at: i32) -> Self {
        AGTrip { myFrom: from, myTo: to, myDepTime: at, myVehicle: String::new(), myType: "default".to_string(), myDay: 0, myPassBy: VecDeque::new() }
    }

    /// C++: `void AGTrip::AGTrip(AGPosition, AGPosition, AGCar, int)`
    // AGTrip(AGPosition from, AGPosition to, AGCar c, int at) :
    //         myFrom(from),
    //         myTo(to),
    //         myDepTime(at),
    //         myVehicle(c.getName()),
    //         myType("default"),
    //         myDay(0) {}
    #[allow(unused_variables)]
    pub fn new_2(from: AGPosition, to: AGPosition, c: AGCar, at: i32) -> Self {
        AGTrip { myFrom: from, myTo: to, myDepTime: at, myVehicle: c.get_name(), myType: "default".to_string(), myDay: 0, myPassBy: VecDeque::new() }
    }

    /// C++: `void AGTrip::AGTrip(AGPosition, AGPosition, AGBus, int)`
    // AGTrip(AGPosition from, AGPosition to, AGBus b, int at) :
    //         myFrom(from),
    //         myTo(to),
    //         myDepTime(at),
    //         myVehicle(b.getName()),
    //         myType("bus"),
    //         myDay(0) {}
    #[allow(unused_variables)]
    pub fn new_3(from: AGPosition, to: AGPosition, b: AGBus, at: i32) -> Self {
        let mut b = b;
        AGTrip { myFrom: from, myTo: to, myDepTime: at, myVehicle: b.get_name(), myType: "bus".to_string(), myDay: 0, myPassBy: VecDeque::new() }
    }

    /// C++: `void AGTrip::AGTrip(AGPosition, AGPosition, std::string, int)`
    // AGTrip(AGPosition from, AGPosition to, std::string v, int at) :
    //         myFrom(from),
    //         myTo(to),
    //         myDepTime(at),
    //         myVehicle(v),
    //         myType("default"),
    //         myDay(0) {}
    #[allow(unused_variables)]
    pub fn new_4(from: AGPosition, to: AGPosition, v: String, at: i32) -> Self {
        AGTrip { myFrom: from, myTo: to, myDepTime: at, myVehicle: v, myType: "default".to_string(), myDay: 0, myPassBy: VecDeque::new() }
    }

    /// C++: `void AGTrip::AGTrip(AGPosition, AGPosition, std::string, int, int)`
    // AGTrip(AGPosition from, AGPosition to, std::string v, int at, int day) :
    //         myFrom(from),
    //         myTo(to),
    //         myDepTime(at),
    //         myVehicle(v),
    //         myType("default"),
    //         myDay(day) {}
    #[allow(unused_variables)]
    pub fn new_5(from: AGPosition, to: AGPosition, v: String, at: i32, day: i32) -> Self {
        AGTrip { myFrom: from, myTo: to, myDepTime: at, myVehicle: v, myType: "default".to_string(), myDay: day, myPassBy: VecDeque::new() }
    }

    /// C++: `void AGTrip::print() const` [const]
    // void
    // AGTrip::print() const {
    //     std::cout << "Trip: " << std::endl;
    //     std::cout << "\t-From= ";
    //     myFrom.print();
    //     std::cout << "\t-To= ";
    //     myTo.print();
    //     std::cout << "\t-At= " << myDepTime << " -Day= " << myDay << std::endl;
    //     std::cout << "\t-Vehicle= " << myVehicle << std::endl;
    //     std::cout << "\t-type= " << myType << std::endl;
    // }
    #[allow(unused_variables)]
    pub fn print(&self) {
        println!("Trip: ");
        print!("\t-From= ");
        self.myFrom.print();
        print!("\t-To= ");
        self.myTo.print();
        println!("\t-At= {} -Day= {}", self.myDepTime, self.myDay);
        println!("\t-Vehicle= {}", self.myVehicle);
        println!("\t-type= {}", self.myType);
    }

    /// C++: `bool AGTrip::operator<(const AGTrip &) const` [const]
    // bool
    // AGTrip::operator <(const AGTrip& trip) const {
    //     if (getDay() < trip.getDay()) {
    //         return true;
    //     }
    //     if (getDay() == trip.getDay())
    //         if (getTime() < trip.getTime()) {
    //             return true;
    //         }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn lt_op(&self, trip: &AGTrip) -> bool {
        if self.get_day() < trip.get_day() {
            return true;
        }
        if self.get_day() == trip.get_day() {
            if self.get_time() < trip.get_time() {
                return true;
            }
        }
        false
    }

    /// C++: `void AGTrip::addLayOver(AGPosition)`
    // void
    // AGTrip::addLayOver(AGPosition by) {
    //     myPassBy.push_back(by);
    // }
    #[allow(unused_variables)]
    pub fn add_lay_over(&mut self, by: AGPosition) {
        self.myPassBy.push_back(by);
    }

    /// C++: `void AGTrip::addLayOver(AGTrip &)`
    // void
    // AGTrip::addLayOver(AGTrip& trip) {
    //     std::list<AGPosition>::iterator it;
    //     for (it = trip.myPassBy.begin(); it != trip.myPassBy.end(); ++it) {
    //         myPassBy.push_back(*it);
    //     }
    //     myPassBy.push_back(trip.myTo);
    // }
    #[allow(unused_variables)]
    pub fn add_lay_over_2(&mut self, trip: &mut AGTrip) {
        for it in trip.myPassBy.iter() {
            self.myPassBy.push_back(*it);
        }
        self.myPassBy.push_back(trip.myTo);
    }

    /// C++: `void AGTrip::addLayOverWithoutDestination(AGTrip &)`
    // void
    // AGTrip::addLayOverWithoutDestination(AGTrip& trip) {
    //     std::list<AGPosition>::iterator it;
    //     for (it = trip.myPassBy.begin(); it != trip.myPassBy.end(); ++it) {
    //         myPassBy.push_back(*it);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_lay_over_without_destination(&mut self, trip: &mut AGTrip) {
        for it in trip.myPassBy.iter() {
            self.myPassBy.push_back(*it);
        }
    }

    /// C++: `AGPosition AGTrip::getDep() const` [const]
    // AGPosition
    // AGTrip::getDep() const {
    //     return myFrom;
    // }
    #[allow(unused_variables)]
    pub fn get_dep(&self) -> AGPosition {
        self.myFrom
    }

    /// C++: `AGPosition AGTrip::getArr() const` [const]
    // AGPosition
    // AGTrip::getArr() const {
    //     return myTo;
    // }
    #[allow(unused_variables)]
    pub fn get_arr(&self) -> AGPosition {
        self.myTo
    }

    /// C++: `int AGTrip::getTime() const` [const]
    // int
    // AGTrip::getTime() const {
    //     return myDepTime;
    // }
    #[allow(unused_variables)]
    pub fn get_time(&self) -> i32 {
        self.myDepTime
    }

    /// C++: `void AGTrip::setDepTime(int)`
    // void
    // AGTrip::setDepTime(int time) {
    //     myDepTime = time;
    // }
    #[allow(unused_variables)]
    pub fn set_dep_time(&mut self, time: i32) {
        self.myDepTime = time;
    }

    /// C++: `const std::string & AGTrip::getVehicleName() const` [const]
    // const std::string&
    // AGTrip::getVehicleName() const {
    //     return myVehicle;
    // }
    #[allow(unused_variables)]
    pub fn get_vehicle_name(&self) -> &str {
        &self.myVehicle
    }

    /// C++: `void AGTrip::setVehicleName(std::string)`
    // void
    // AGTrip::setVehicleName(std::string name) {
    //     myVehicle = name;
    // }
    #[allow(unused_variables)]
    pub fn set_vehicle_name(&mut self, name: String) {
        self.myVehicle = name;
    }

    /// C++: `void AGTrip::setArr(AGPosition)`
    // void
    // AGTrip::setArr(AGPosition arrival) {
    //     myTo = AGPosition(arrival.getStreet(), arrival.getPosition());
    // }
    #[allow(unused_variables)]
    pub fn set_arr(&mut self, arrival: AGPosition) {
        self.myTo = AGPosition::new(arrival.get_street(), arrival.get_position());
    }

    /// C++: `void AGTrip::setDep(AGPosition)`
    // void
    // AGTrip::setDep(AGPosition departure) {
    //     myFrom = AGPosition(departure.getStreet(), departure.getPosition());
    // }
    #[allow(unused_variables)]
    pub fn set_dep(&mut self, departure: AGPosition) {
        self.myFrom = AGPosition::new(departure.get_street(), departure.get_position());
    }

    /// C++: `int AGTrip::getDay() const` [const]
    // int
    // AGTrip::getDay() const {
    //     return myDay;
    // }
    #[allow(unused_variables)]
    pub fn get_day(&self) -> i32 {
        self.myDay
    }

    /// C++: `void AGTrip::setDay(int)`
    // void
    // AGTrip::setDay(int d) {
    //     myDay = d;
    // }
    #[allow(unused_variables)]
    pub fn set_day(&mut self, day: i32) {
        self.myDay = day;
    }

    /// C++: `const std::string & AGTrip::getType() const` [const]
    // const std::string&
    // AGTrip::getType() const {
    //     return myType;
    // }
    #[allow(unused_variables)]
    pub fn get_type(&self) -> &str {
        &self.myType
    }

    /// C++: `void AGTrip::setType(std::string)`
    // void
    // AGTrip::setType(std::string type) {
    //     myType = type;
    // }
    #[allow(unused_variables)]
    pub fn set_type(&mut self, type_: String) {
        self.myType = type_;
    }

    /// C++: `const std::list<AGPosition> * AGTrip::getPassed() const` [const]
    // const std::list<AGPosition>*
    // AGTrip::getPassed() const {
    //     return &myPassBy;
    // }
    #[allow(unused_variables)]
    pub fn get_passed(&self) -> Ptr<VecDeque<AGPosition>> {
        Ptr::new(&self.myPassBy)
    }

    /// C++: `int AGTrip::getRideBackArrTime(double) const` [const]
    /// returns the time regarding the departure time going through the different points and coming back to the initial position given the time to make one kilometer
    // int
    // AGTrip::getRideBackArrTime(double secPerKm) const {
    //     return getArrTime(secPerKm) + (int)(secPerKm * myTo.distanceTo(myFrom) / 1000.0);
    // }
    #[allow(unused_variables)]
    pub fn get_ride_back_arr_time(&self, secPerKm: f64) -> i32 {
        self.get_arr_time(secPerKm) + (secPerKm * self.myTo.distance_to(&self.myFrom) / 1000.0) as i32
    }

    /// C++: `int AGTrip::getArrTime(double) const` [const]
    /// returns the estimated arrival time given the time to make one kilometer
    // int
    // AGTrip::getArrTime(double secPerKm) const {
    //     return myDepTime + getTimeTrip(secPerKm);
    // }
    #[allow(unused_variables)]
    pub fn get_arr_time(&self, secPerKm: f64) -> i32 {
        self.myDepTime + self.get_time_trip(secPerKm)
    }

    /// C++: `int AGTrip::getTimeTrip(double) const` [const]
    /// gives the time in seconds for the trip given a speed in seconds per kilometer (in city, not car speed but time needed to make a distance in the city)
    // int
    // AGTrip::getTimeTrip(double secPerKm) const {
    //     double dist = 0;
    //     std::list<AGPosition> positions;
    //     positions.push_back(myFrom);
    //     std::list<AGPosition>::const_iterator it;
    //     for (it = myPassBy.begin(); it != myPassBy.end(); ++it) {
    //         positions.push_back(*it);
    //     }
    //     positions.push_back(myTo);
    //
    //     const AGPosition* temp = &positions.front();
    //     for (it = positions.begin(), ++it; it != positions.end(); ++it) {
    //         dist += temp->distanceTo(*it);
    //         temp = &*it;
    //     }
    //     return (int)(secPerKm * (dist / 1000.0));
    // }
    #[allow(unused_variables)]
    pub fn get_time_trip(&self, secPerKm: f64) -> i32 {
        let mut dist: f64 = 0.0;
        let mut positions: Vec<AGPosition> = Vec::new();
        positions.push(self.myFrom);
        for it in self.myPassBy.iter() {
            positions.push(*it);
        }
        positions.push(self.myTo);
        let mut temp: &AGPosition = &positions[0];
        for it in positions.iter().skip(1) {
            dist += temp.distance_to(it);
            temp = it;
        }
        (secPerKm * (dist / 1000.0)) as i32
    }

    /// C++: `int AGTrip::estimateDepTime(int, double) const` [const]
    /// estimate the departure time needed for a given arrival time and a speed in seconds per kilometer
    // int
    // AGTrip::estimateDepTime(int arrTime, double secPerKm) const {
    //     return arrTime - getTimeTrip(secPerKm);
    // }
    #[allow(unused_variables)]
    pub fn estimate_dep_time(&self, arrTime: i32, secPerKm: f64) -> i32 {
        arrTime - self.get_time_trip(secPerKm)
    }

    /// C++: `bool AGTrip::isDaily() const` [const]
    /// returns whether this is a daily trip or a one day trip
    // bool
    // AGTrip::isDaily() const {
    //     return (myDay == 0);
    // }
    #[allow(unused_variables)]
    pub fn is_daily(&self) -> bool {
        self.myDay == 0
    }

}

