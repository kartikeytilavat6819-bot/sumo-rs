//! Vehicles must stop for a red signal: the queued v2 (queue scenario, C++ reference arrival 94 s)
//! brakes for the red at t=87-89 and arrives at 94 s. Before the switch-command wiring fix it ran the
//! light and arrived at 92 s.

use std::process::Command;

#[test]
fn queued_vehicle_obeys_red_light() {
    let case = concat!(env!("CARGO_MANIFEST_DIR"), "/../../tools/oracle/cases/signal");
    let out = std::env::temp_dir().join("sumo_signal_red_tripinfo.xml");
    let status = Command::new(env!("CARGO_BIN_EXE_sumo"))
        .args(["-n", &format!("{case}/ref.net.xml"), "-r", &format!("{case}/routes_queue.rou")])
        .args(["--end", "900", "--no-step-log", "true", "--tripinfo-output"])
        .arg(&out)
        .status()
        .expect("spawn sumo");
    assert!(status.success());
    let xml = std::fs::read_to_string(&out).expect("tripinfo written");
    let v2 = xml.lines().find(|l| l.contains(r#"id="v2""#)).expect("v2 tripinfo");
    assert!(v2.contains(r#"arrival="94.00""#), "v2 arrival changed: {v2}");
}
