//! Port of activitygen/city/AGTime.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `AGTime` (activitygen/city/AGTime.h)
pub struct AGTime {
    pub mySeconds: i32,
}

impl AGTime {
    /// C++: `void AGTime::AGTime(int)`
    // AGTime(int seconds) :
    //         mySeconds(seconds) {}
    #[allow(unused_variables)]
    pub fn new(seconds: i32) -> Self {
        todo!("port AGTime::AGTime")
    }

    /// C++: `void AGTime::AGTime(int, int)`
    // AGTime(int hour, int minutes) :
    //         mySeconds(convert(0, hour, minutes, 0)) {}
    #[allow(unused_variables)]
    pub fn new_2(hour: i32, minutes: i32) -> Self {
        todo!("port AGTime::AGTime")
    }

    /// C++: `void AGTime::AGTime(int, int, int)`
    // AGTime(int day, int hour, int min) :
    //         mySeconds(convert(day, hour, min, 0)) {}
    #[allow(unused_variables)]
    pub fn new_3(day: i32, hour: i32, min: i32) -> Self {
        todo!("port AGTime::AGTime")
    }

    /// C++: `void AGTime::AGTime(int, int, int, int)`
    // AGTime(int day, int hour, int min, int sec) :
    //         mySeconds(convert(day, hour, min, sec)) {}
    #[allow(unused_variables)]
    pub fn new_4(day: i32, hour: i32, min: i32, sec: i32) -> Self {
        todo!("port AGTime::AGTime")
    }

    /// C++: `void AGTime::AGTime(const AGTime &)`
    // AGTime::AGTime(const AGTime& time) {
    //     mySeconds = time.mySeconds;
    // }
    #[allow(unused_variables)]
    pub fn new_5(time: &AGTime) -> Self {
        todo!("port AGTime::AGTime")
    }

    /// C++: `bool AGTime::operator==(const AGTime &)`
    // bool
    // AGTime::operator==(const AGTime& time) {
    //     if (this->mySeconds == time.mySeconds) {
    //         return true;
    //     } else {
    //         return false;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn eq_op(&mut self, time: &AGTime) -> bool {
        todo!("port AGTime::operator==")
    }

    /// C++: `bool AGTime::operator<(const AGTime &)`
    // bool
    // AGTime::operator<(const AGTime& time) {
    //     if (this->mySeconds < time.mySeconds) {
    //         return true;
    //     } else {
    //         return false;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn lt_op(&mut self, time: &AGTime) -> bool {
        todo!("port AGTime::operator<")
    }

    /// C++: `bool AGTime::operator<=(const AGTime &)`
    // bool
    // AGTime::operator<=(const AGTime& time) {
    //     if (this->mySeconds <= time.mySeconds) {
    //         return true;
    //     } else {
    //         return false;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn le_op(&mut self, time: &AGTime) -> bool {
        todo!("port AGTime::operator<=")
    }

    /// C++: `void AGTime::operator+=(const AGTime &)`
    // void
    // AGTime::operator+=(const AGTime& time) {
    //     this->mySeconds += time.mySeconds;
    // }
    #[allow(unused_variables)]
    pub fn add_assign_op(&mut self, time: &AGTime) {
        todo!("port AGTime::operator+=")
    }

    /// C++: `void AGTime::operator+=(int)`
    // void
    // AGTime::operator+=(int seconds) {
    //     this->mySeconds += seconds;
    // }
    #[allow(unused_variables)]
    pub fn add_assign_op_2(&mut self, seconds: i32) {
        todo!("port AGTime::operator+=")
    }

    /// C++: `void AGTime::operator-=(const AGTime &)`
    // void
    // AGTime::operator-=(const AGTime& time) {
    //     this->mySeconds -= time.mySeconds;
    // }
    #[allow(unused_variables)]
    pub fn sub_assign_op(&mut self, time: &AGTime) {
        todo!("port AGTime::operator-=")
    }

    /// C++: `AGTime AGTime::operator+(const AGTime &)`
    // AGTime
    // AGTime::operator+(const AGTime& time) {
    //     AGTime newtime(time.mySeconds + this->mySeconds);
    //     return newtime;
    // }
    #[allow(unused_variables)]
    pub fn add_op(&mut self, time: &AGTime) -> AGTime {
        todo!("port AGTime::operator+")
    }

    /// C++: `int AGTime::getDay()`
    /// ****************** In/Out functions * ******************
    // int
    // AGTime::getDay() {
    //     return (mySeconds / 86400);
    // }
    #[allow(unused_variables)]
    pub fn get_day(&mut self) -> i32 {
        todo!("port AGTime::getDay")
    }

    /// C++: `int AGTime::getHour()`
    // int
    // AGTime::getHour() {
    //     return ((mySeconds / 3600) % 24);
    // }
    #[allow(unused_variables)]
    pub fn get_hour(&mut self) -> i32 {
        todo!("port AGTime::getHour")
    }

    /// C++: `int AGTime::getMinute()`
    // int
    // AGTime::getMinute() {
    //     return ((mySeconds / 60) % 60);
    // }
    #[allow(unused_variables)]
    pub fn get_minute(&mut self) -> i32 {
        todo!("port AGTime::getMinute")
    }

    /// C++: `int AGTime::getSecond()`
    // int
    // AGTime::getSecond() {
    //     return (mySeconds % 60);
    // }
    #[allow(unused_variables)]
    pub fn get_second(&mut self) -> i32 {
        todo!("port AGTime::getSecond")
    }

    /// C++: `int AGTime::getSecondsInCurrentDay()`
    // int
    // AGTime::getSecondsInCurrentDay() {
    //     return (mySeconds % 86400);
    // }
    #[allow(unused_variables)]
    pub fn get_seconds_in_current_day(&mut self) -> i32 {
        todo!("port AGTime::getSecondsInCurrentDay")
    }

    /// C++: `int AGTime::getTime()`
    /// : returns the number of seconds from the beginning of the first day of simulation this includes
    // int
    // AGTime::getTime() {
    //     return this->mySeconds;
    // }
    #[allow(unused_variables)]
    pub fn get_time(&mut self) -> i32 {
        todo!("port AGTime::getTime")
    }

    /// C++: `void AGTime::setDay(int)`
    // void
    // AGTime::setDay(int d) {
    //     if (0 <= d) {
    //         mySeconds -= 86400 * getDay();
    //         mySeconds += 86400 * d;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_day(&mut self, d: i32) {
        todo!("port AGTime::setDay")
    }

    /// C++: `void AGTime::setHour(int)`
    // void
    // AGTime::setHour(int h) {
    //     if (0 <= h && h < 24) {
    //         mySeconds -= 3600 * getHour();
    //         mySeconds += 3600 * h;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_hour(&mut self, h: i32) {
        todo!("port AGTime::setHour")
    }

    /// C++: `void AGTime::setMinute(int)`
    // void
    // AGTime::setMinute(int m) {
    //     if (0 <= m && m < 60) {
    //         mySeconds -= 60 * getMinute();
    //         mySeconds += 60 * m;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_minute(&mut self, m: i32) {
        todo!("port AGTime::setMinute")
    }

    /// C++: `void AGTime::setSecond(int)`
    // void
    // AGTime::setSecond(int s) {
    //     if (0 <= s && s < 60) {
    //         mySeconds -= getSecond();
    //         mySeconds += s;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_second(&mut self, s: i32) {
        todo!("port AGTime::setSecond")
    }

    /// C++: `void AGTime::setTime(int)`
    /// : sets the time from the beginning of the first day of simulation in seconds
    // void
    // AGTime::setTime(int seconds) {
    //     mySeconds = seconds;
    // }
    #[allow(unused_variables)]
    pub fn set_time(&mut self, sec: i32) {
        todo!("port AGTime::setTime")
    }

    /// C++: `void AGTime::addSeconds(int)`
    /// addition of seconds to the current moment
    // void
    // AGTime::addSeconds(int s) {
    //     mySeconds += s;
    // }
    #[allow(unused_variables)]
    pub fn add_seconds(&mut self, sec: i32) {
        todo!("port AGTime::addSeconds")
    }

    /// C++: `void AGTime::addMinutes(int)`
    /// addition of minutes to the current moment
    // void
    // AGTime::addMinutes(int m) {
    //     mySeconds += 60 * m;
    // }
    #[allow(unused_variables)]
    pub fn add_minutes(&mut self, min: i32) {
        todo!("port AGTime::addMinutes")
    }

    /// C++: `void AGTime::addHours(int)`
    /// addition of hours to the current moment
    // void
    // AGTime::addHours(int h) {
    //     mySeconds += 3600 * h;
    // }
    #[allow(unused_variables)]
    pub fn add_hours(&mut self, hours: i32) {
        todo!("port AGTime::addHours")
    }

    /// C++: `void AGTime::addDays(int)`
    /// addition of days to the current moment
    // void
    // AGTime::addDays(int d) {
    //     mySeconds += 86400 * d;
    // }
    #[allow(unused_variables)]
    pub fn add_days(&mut self, days: i32) {
        todo!("port AGTime::addDays")
    }

    /// C++: `int AGTime::getSecondsOf(double)`
    /// computes the number of seconds in the given minutes
    // int
    // AGTime::getSecondsOf(double minutes) {
    //     return static_cast<int>(60.0 * minutes);
    // }
    #[allow(unused_variables)]
    pub fn get_seconds_of(&mut self, minutes: f64) -> i32 {
        todo!("port AGTime::getSecondsOf")
    }

    /// C++: `int AGTime::convert(int, int, int, int)` [private]
    /// converts days, hours and minutes to seconds
    // int
    // AGTime::convert(int days, int hours, int minutes, int seconds) {
    //     mySeconds = seconds + 60 * (minutes + 60 * (hours + 24 * (days)));
    //     return mySeconds;
    // }
    #[allow(unused_variables)]
    pub fn convert(&mut self, days: i32, hours: i32, minutes: i32, seconds: i32) -> i32 {
        todo!("port AGTime::convert")
    }

}

