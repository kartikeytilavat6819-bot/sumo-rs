//! Port of activitygen/city/AGWorkPosition.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `AGWorkPosition` (activitygen/city/AGWorkPosition.h)
pub struct AGWorkPosition {
    pub myStatData: Ptr<AGDataAndStatistics>,
    pub myLocation: AGPosition,
    pub myAdult: Ptr<AGAdult>,
    pub myOpeningTime: i32,
    pub myClosingTime: i32,
}

impl AGWorkPosition {
    /// C++: `void AGWorkPosition::AGWorkPosition(AGDataAndStatistics *, const AGStreet &)`
    // AGWorkPosition::AGWorkPosition(AGDataAndStatistics* ds, const AGStreet& inStreet) :
    //     myStatData(ds),
    //     myLocation(inStreet),
    //     myAdult(nullptr),
    //     myOpeningTime(generateOpeningTime(*ds)),
    //     myClosingTime(generateClosingTime(*ds)) {
    //     ds->workPositions++;
    // }
    #[allow(unused_variables)]
    pub fn new(ds: Ptr<AGDataAndStatistics>, inStreet: &AGStreet) -> Self {
        let wp = AGWorkPosition {
        myStatData: ds,
        myLocation: AGPosition::new_2(inStreet),
        myAdult: Ptr::null(),
        myOpeningTime: Self::generate_opening_time(&*ds),
        myClosingTime: Self::generate_closing_time(&*ds),
    };
    ds.m().workPositions += 1;
    wp
    }

    /// C++: `void AGWorkPosition::AGWorkPosition(AGDataAndStatistics *, const AGStreet &, double)`
    // AGWorkPosition::AGWorkPosition(AGDataAndStatistics* ds, const AGStreet& inStreet, double pos) :
    //     myStatData(ds),
    //     myLocation(inStreet, pos),
    //     myAdult(nullptr),
    //     myOpeningTime(generateOpeningTime(*ds)),
    //     myClosingTime(generateClosingTime(*ds)) {
    //     ds->workPositions++;
    // }
    #[allow(unused_variables)]
    pub fn new_2(ds: Ptr<AGDataAndStatistics>, inStreet: &AGStreet, pos: f64) -> Self {
        let wp = AGWorkPosition {
        myStatData: ds,
        myLocation: AGPosition::new(inStreet, pos),
        myAdult: Ptr::null(),
        myOpeningTime: Self::generate_opening_time(&*ds),
        myClosingTime: Self::generate_closing_time(&*ds),
    };
    ds.m().workPositions += 1;
    wp
    }

    /// C++: `void AGWorkPosition::~AGWorkPosition()`
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // nothing to release: all members are owned values or non-owning pointers
    }

    /// C++: `void AGWorkPosition::take(AGAdult *)`
    // void
    // AGWorkPosition::take(AGAdult* worker) {
    //     if (myAdult == nullptr) {
    //         myStatData->workPositions--;
    //         myAdult = worker;
    //     } else {
    //         throw ProcessError(TL("Work position already occupied. Cannot give it to another adult."));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn take(&mut self, ad: Ptr<AGAdult>) {
        if self.myAdult.is_null() {
            self.myStatData.m().workPositions -= 1;
            self.myAdult = ad;
        } else {
            // shortcut: C++ throws ProcessError here; panics until callers can take a Result, upgrade with the callers
            panic!("{}", ProcessError::new("Work position already occupied. Cannot give it to another adult."));
        }
    }

    /// C++: `void AGWorkPosition::let()`
    // void
    // AGWorkPosition::let() {
    //     if (myAdult != nullptr) {
    //         myStatData->workPositions++;
    //         myAdult->lostWorkPosition();
    //         myAdult = nullptr;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn let_(&mut self) {
        if !self.myAdult.is_null() {
            self.myStatData.m().workPositions += 1;
            self.myAdult.m().lost_work_position();
            self.myAdult = Ptr::null();
        }
    }

    /// C++: `bool AGWorkPosition::isTaken() const` [const]
    // bool
    // AGWorkPosition::isTaken() const {
    //     return (myAdult != nullptr);
    // }
    #[allow(unused_variables)]
    pub fn is_taken(&self) -> bool {
        !self.myAdult.is_null()
    }

    /// C++: `AGPosition AGWorkPosition::getPosition() const` [const]
    // AGPosition
    // AGWorkPosition::getPosition() const {
    //     return myLocation;
    // }
    #[allow(unused_variables)]
    pub fn get_position(&self) -> AGPosition {
        AGPosition { street: self.myLocation.street, position: self.myLocation.position, pos2d: self.myLocation.pos2d }
    }

    /// C++: `int AGWorkPosition::getOpening() const` [const]
    // int
    // AGWorkPosition::getOpening() const {
    //     return myOpeningTime;
    // }
    #[allow(unused_variables)]
    pub fn get_opening(&self) -> i32 {
        self.myOpeningTime
    }

    /// C++: `int AGWorkPosition::getClosing() const` [const]
    // int
    // AGWorkPosition::getClosing() const {
    //     return myClosingTime;
    // }
    #[allow(unused_variables)]
    pub fn get_closing(&self) -> i32 {
        self.myClosingTime
    }

    /// C++: `void AGWorkPosition::print() const` [const]
    // void
    // AGWorkPosition::print() const {
    //     std::cout << "- AGWorkPosition: open=" << myOpeningTime << " closingTime=" << myClosingTime << " taken=" << isTaken() << std::endl;
    //     std::cout << "\t";
    //     myLocation.print();
    // }
    #[allow(unused_variables)]
    pub fn print(&self) {
        println!("- AGWorkPosition: open={} closingTime={} taken={}", self.myOpeningTime, self.myClosingTime, self.is_taken() as i32);
        print!("\t");
        self.myLocation.print();
    }

    /// C++: `int AGWorkPosition::generateOpeningTime(const AGDataAndStatistics &)` [static, private]
    // int
    // AGWorkPosition::generateOpeningTime(const AGDataAndStatistics& ds) {
    //     double choice = RandHelper::rand();
    //     double cumul = 0;
    //
    //     for (std::map<int, double>::const_iterator it = ds.beginWorkHours.begin();
    //             it != ds.beginWorkHours.end(); ++it) {
    //         cumul += it->second;
    //         if (cumul >= choice) {
    //             return it->first;
    //         }
    //     }
    //     std::cout << "-- WARNING: work time distribution not complete (Sum(proportions) != 1): AUTODEFINED at 9.00am --" << std::endl;
    //     return 32400;
    // }
    #[allow(unused_variables)]
    pub fn generate_opening_time(ds: &AGDataAndStatistics) -> i32 {
        let choice = utils::common::rand_helper::rand();
    let mut cumul = 0.0;

    for (k, v) in ds.beginWorkHours.iter() {
        cumul += v;
        if cumul >= choice {
            return *k;
        }
    }
    println!("-- WARNING: work time distribution not complete (Sum(proportions) != 1): AUTODEFINED at 9.00am --");
    32400
    }

    /// C++: `int AGWorkPosition::generateClosingTime(const AGDataAndStatistics &)` [static, private]
    // int
    // AGWorkPosition::generateClosingTime(const AGDataAndStatistics& ds) {
    //     double choice = RandHelper::rand();
    //     double cumul = 0;
    //     for (std::map<int, double>::const_iterator it = ds.endWorkHours.begin();
    //             it != ds.endWorkHours.end(); ++it) {
    //         cumul += it->second;
    //         if (cumul >= choice) {
    //             return it->first;
    //         }
    //     }
    //     std::cout << "-- WARNING: work time distribution not complete (Sum(proportions) != 1): AUTODEFINED at 5.00pm --" << std::endl;
    //     return 61200;
    // }
    #[allow(unused_variables)]
    pub fn generate_closing_time(ds: &AGDataAndStatistics) -> i32 {
        let choice = utils::common::rand_helper::rand();
    let mut cumul = 0.0;
    for (k, v) in ds.endWorkHours.iter() {
        cumul += v;
        if cumul >= choice {
            return *k;
        }
    }
    println!("-- WARNING: work time distribution not complete (Sum(proportions) != 1): AUTODEFINED at 5.00pm --");
    61200
    }

}

