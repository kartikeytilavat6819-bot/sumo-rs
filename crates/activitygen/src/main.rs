//! Port of activitygen/activitygen_main.cpp (the logic lives in sumo_core::activitygen::activitygen_main)
fn main() -> std::process::ExitCode {
    let argv: Vec<String> = std::env::args().collect();
    let ret = sumo_core::activitygen::activitygen_main::main(argv.len() as i32, &argv);
    std::process::ExitCode::from(ret as u8)
}
