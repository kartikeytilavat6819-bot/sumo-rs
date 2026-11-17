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
    if zoneNumber < 400 {
        return zoneNumber; // real "zone" numbers, no city zones
    }
    match zoneNumber {
        511 => 165,
        512 => 166,
        513 => 167,
        514 => 142,
        515 => 123,
        516 => 127,
        518 => 145,
        519 => 144,
        521 => 153,
        551 => 231,
        552 => 232,
        553 => 233,
        554 => 259,
        555 => 241,
        556 => 255,
        571 => 322,
        572 => 324,
        _ => zoneNumber,
    }
}

/// C++ `static std::unordered_map<int, long long int> fareZoneToRep` (utils/router/FareZones.h)
// looked up by linear scan (lookup only, no iteration order involved); C++ operator[] returns 0 for absent keys
static FARE_ZONE_TO_REP: [(i32, i64); 62] = [
    (110, 1),
    (121, 2),
    (122, 4),
    (123, 8),
    (124, 16),
    (125, 32),
    (126, 64),
    (127, 128),
    (128, 256),
    (129, 512),
    (131, 1024),
    (132, 2048),
    (133, 4096),
    (134, 8192),
    (141, 16384),
    (142, 32768),
    (143, 65536),
    (144, 131072),
    (145, 262144),
    (146, 524288),
    (147, 1048576),
    (151, 2097152),
    (152, 4194304),
    (153, 8388608),
    (154, 16777216),
    (155, 33554432),
    (156, 67108864),
    (162, 134217728),
    (163, 268435456),
    (164, 536870912),
    (165, 1073741824),
    (166, 2147483648),
    (167, 4294967296),
    (168, 8589934592),
    (210, 17179869184),
    (221, 34359738368),
    (222, 68719476736),
    (223, 137438953472),
    (224, 274877906944),
    (225, 549755813888),
    (231, 1099511627776),
    (232, 2199023255552),
    (233, 4398046511104),
    (234, 8796093022208),
    (241, 17592186044416),
    (242, 35184372088832),
    (243, 70368744177664),
    (251, 140737488355328),
    (252, 281474976710656),
    (253, 562949953421312),
    (254, 1125899906842624),
    (255, 2251799813685248),
    (256, 4503599627370496),
    (257, 9007199254740992),
    (258, 18014398509481984),
    (259, 36028797018963968),
    (261, 72057594037927936),
    (299, 144115188075855872),
    (321, 288230376151711744),
    (322, 576460752303423488),
    (323, 1152921504606846976),
    (324, 2305843009213693952),
];

/// C++ `fareZoneToRep[zone]`: unordered_map::operator[] yields 0 for a missing key
pub fn fare_zone_to_rep(zone: i32) -> i64 {
    FARE_ZONE_TO_REP.iter().find(|(z, _)| *z == zone).map_or(0, |(_, r)| *r)
}
