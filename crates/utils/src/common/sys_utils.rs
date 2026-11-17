//! Port of utils/common/SysUtils.{h,cpp}
//!
//! Only `getCurrentMillis` is ported (the other helpers are not needed by the simulation core).
//! The WIN32 branch (QueryPerformanceCounter) is not ported: the reference runs are Linux.

/// C++: `long SysUtils::getCurrentMillis()`
// long
// SysUtils::getCurrentMillis() {
//     timeval current;
//     gettimeofday(&current, 0);
//     long nanosecs =
//         (long) current.tv_sec * 1000L + (long) current.tv_usec / 1000L;
//     return nanosecs;
// }
pub fn get_current_millis() -> i64 {
    let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default();
    now.as_secs() as i64 * 1000 + (now.subsec_micros() / 1000) as i64
}
