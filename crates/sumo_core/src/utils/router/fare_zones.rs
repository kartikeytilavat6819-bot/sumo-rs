//! Port of utils/router/FareZones.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++: `int (int) getOverlayZone`
// inline int getOverlayZone(int zoneNumber) {
//     if (zoneNumber < 400) {
//         return zoneNumber;    //real "zone" numbers, no city zones
//     }
//
//     switch (zoneNumber) {
//         case 511:
//             return 165;
//         case 512:
//             return 166;
//         case 513:
//             return 167;
//         case 514:
//             return 142;
//         case 515:
//             return 123;
//         case 516:
//             return 127;
//         case 518:
//             return 145;
//         case 519:
//             return 144;
//         case 521:
//             return 153;
//         case 551:
//             return 231;
//         case 552:
//             return 232;
//         case 553:
//             return 233;
//         case 554:
//             return 259;
//         case 555:
//             return 241;
//         case 556:
//             return 255;
//         case 571:
//             return 322;
//         case 572:
//             return 324;
//         default:
//             return zoneNumber;
//     }
// }
#[allow(unused_variables)]
pub fn get_overlay_zone(zoneNumber: i32) -> i32 {
    todo!("port getOverlayZone")
}

