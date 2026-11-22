//! oc93 measurement probe (temporary, removed before the final commit).
//! Active only when the environment variable OC93_COUNT=1; prints to stderr.
use std::sync::OnceLock;
use std::sync::atomic::{AtomicU64, Ordering::Relaxed};

pub const NAMES: [&str; 8] = [
    "prohibit",
    "get_router_tt",
    "notify_enter",
    "enter_lane",
    "lane_insert",
    "edge_insert",
    "try_insert",
    "emit",
];

static COUNTS: [AtomicU64; 8] = [const { AtomicU64::new(0) }; 8];
static ON: OnceLock<bool> = OnceLock::new();

pub fn on() -> bool {
    *ON.get_or_init(|| std::env::var("OC93_COUNT").map(|v| v == "1").unwrap_or(false))
}

/// Increments counter `i` and returns the new value (0 when the probe is off).
pub fn hit(i: usize) -> u64 {
    if !on() {
        return 0;
    }
    COUNTS[i].fetch_add(1, Relaxed) + 1
}

/// Prints all counters with the current simulation time.
pub fn line(tag: &str) {
    if !on() {
        return;
    }
    let mut s = format!("OC93 {} t={}", tag, crate::microsim::ms_net::MSNet::get_instance().m().get_current_time_step());
    for (i, name) in NAMES.iter().enumerate() {
        s.push_str(&format!(" {}={}", name, COUNTS[i].load(Relaxed)));
    }
    eprintln!("{}", s);
}
