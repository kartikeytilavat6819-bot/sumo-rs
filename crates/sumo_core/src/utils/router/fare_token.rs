//! Port of utils/router/FareToken.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ enum `FareToken`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum FareToken {
    None = 0,
    Free = 1,
    H = 2,
    L = 3,
    T1 = 4,
    T2 = 5,
    T3 = 6,
    Z = 7,
    M = 8,
    U = 9,
    KL = 10,
    KH = 11,
    K = 12,
    KHU = 13,
    KLU = 14,
    KHZ = 15,
    KLZ = 16,
    ZU = 17,
    START = 18,
}

/// C++: `FareToken (std::string) FareUtil::stringToToken`
// inline FareToken stringToToken(std::string str) {
//     if (str == "H") {
//         return FareToken::H;
//     }
//     if (str == "L") {
//         return FareToken::L;
//     }
//     if (str == "T1") {
//         return FareToken::T1;
//     }
//     if (str == "T2") {
//         return FareToken::T2;
//     }
//     if (str == "T3") {
//         return FareToken::T3;
//     }
//     if (str == "1") {
//         return FareToken::T1;
//     }
//     if (str == "2") {
//         return FareToken::T2;
//     }
//     if (str == "3") {
//         return FareToken::T3;
//     }
//     if (str == "U") {
//         return FareToken::U;
//     }
//     if (str == "Z") {
//         return FareToken::Z;
//     }
//     if (str == "M") {
//         return FareToken::M;
//     }
//     if (str == "K") {
//         return FareToken::K;
//     }
//     if (str == "KL") {
//         return FareToken::KL;
//     }
//     if (str == "KH") {
//         return FareToken::KH;
//     }
//     if (str == "ZU") {
//         return FareToken::ZU;
//     }
//     if (str == "None") {
//         return FareToken::None;
//     }
//     if (str == "Free") {
//         return FareToken::Free;
//     }
//     if (str == "KHU") {
//         return FareToken::KHU;
//     }
//     if (str == "KLU") {
//         return FareToken::KLU;
//     }
//     if (str == "KHZ") {
//         return FareToken::KHZ;
//     }
//     if (str == "KLZ") {
//         return FareToken::KLZ;
//     }
//     if (str == "NOTFOUND") {
//         return FareToken::None;
//     }
//     assert(false);
//     return FareToken::None;
// }
#[allow(unused_variables)]
pub fn string_to_token(str: String) -> FareToken {
        if str == "H" {
        return FareToken::H;
    }
    if str == "L" {
        return FareToken::L;
    }
    if str == "T1" {
        return FareToken::T1;
    }
    if str == "T2" {
        return FareToken::T2;
    }
    if str == "T3" {
        return FareToken::T3;
    }
    if str == "1" {
        return FareToken::T1;
    }
    if str == "2" {
        return FareToken::T2;
    }
    if str == "3" {
        return FareToken::T3;
    }
    if str == "U" {
        return FareToken::U;
    }
    if str == "Z" {
        return FareToken::Z;
    }
    if str == "M" {
        return FareToken::M;
    }
    if str == "K" {
        return FareToken::K;
    }
    if str == "KL" {
        return FareToken::KL;
    }
    if str == "KH" {
        return FareToken::KH;
    }
    if str == "ZU" {
        return FareToken::ZU;
    }
    if str == "None" {
        return FareToken::None;
    }
    if str == "Free" {
        return FareToken::Free;
    }
    if str == "KHU" {
        return FareToken::KHU;
    }
    if str == "KLU" {
        return FareToken::KLU;
    }
    if str == "KHZ" {
        return FareToken::KHZ;
    }
    if str == "KLZ" {
        return FareToken::KLZ;
    }
    if str == "NOTFOUND" {
        return FareToken::None;
    }
    assert!(false);
    FareToken::None
}

/// C++: `FareToken (std::string) FareUtil::stringToToken`
// inline FareToken stringToToken(std::string str) {
//     if (str == "H") {
//         return FareToken::H;
//     }
//     if (str == "L") {
//         return FareToken::L;
//     }
//     if (str == "T1") {
//         return FareToken::T1;
//     }
//     if (str == "T2") {
//         return FareToken::T2;
//     }
//     if (str == "T3") {
//         return FareToken::T3;
//     }
//     if (str == "1") {
//         return FareToken::T1;
//     }
//     if (str == "2") {
//         return FareToken::T2;
//     }
//     if (str == "3") {
//         return FareToken::T3;
//     }
//     if (str == "U") {
//         return FareToken::U;
//     }
//     if (str == "Z") {
//         return FareToken::Z;
//     }
//     if (str == "M") {
//         return FareToken::M;
//     }
//     if (str == "K") {
//         return FareToken::K;
//     }
//     if (str == "KL") {
//         return FareToken::KL;
//     }
//     if (str == "KH") {
//         return FareToken::KH;
//     }
//     if (str == "ZU") {
//         return FareToken::ZU;
//     }
//     if (str == "None") {
//         return FareToken::None;
//     }
//     if (str == "Free") {
//         return FareToken::Free;
//     }
//     if (str == "KHU") {
//         return FareToken::KHU;
//     }
//     if (str == "KLU") {
//         return FareToken::KLU;
//     }
//     if (str == "KHZ") {
//         return FareToken::KHZ;
//     }
//     if (str == "KLZ") {
//         return FareToken::KLZ;
//     }
//     if (str == "NOTFOUND") {
//         return FareToken::None;
//     }
//     assert(false);
//     return FareToken::None;
// }
#[allow(unused_variables)]
pub fn string_to_token_2(str: String) -> FareToken {
    string_to_token(str)
}

/// C++: `std::string (const FareToken &) FareUtil::tokenToString`
// inline std::string tokenToString(FareToken const& token) {
//     switch (token) {
//         case FareToken::H:
//             return "H";
//         case FareToken::L:
//             return "L";
//         case FareToken::T1:
//             return "T1";
//         case FareToken::T2:
//             return "T2";
//         case FareToken::T3:
//             return "T3";
//         case FareToken::U:
//             return "U";
//         case FareToken::Z:
//             return "Z";
//         case FareToken::M:
//             return "M";
//         case FareToken::K:
//             return "K";
//         case FareToken::KL:
//             return "KL";
//         case FareToken::KH:
//             return "KH";
//         case FareToken::ZU:
//             return "ZU";
//         case FareToken::None:
//             return "None";
//         case FareToken::Free:
//             return "Free";
//         case FareToken::KHU:
//             return "KHU";
//         case FareToken::KLU:
//             return "KLU";
//         case FareToken ::KHZ:
//             return "KHZ";
//         case FareToken ::KLZ:
//             return  "KLZ";
//         default:
//             return "";
//     }
// }
#[allow(unused_variables)]
pub fn token_to_string(token: &FareToken) -> String {
        match token {
        FareToken::H => "H".to_string(),
        FareToken::L => "L".to_string(),
        FareToken::T1 => "T1".to_string(),
        FareToken::T2 => "T2".to_string(),
        FareToken::T3 => "T3".to_string(),
        FareToken::U => "U".to_string(),
        FareToken::Z => "Z".to_string(),
        FareToken::M => "M".to_string(),
        FareToken::K => "K".to_string(),
        FareToken::KL => "KL".to_string(),
        FareToken::KH => "KH".to_string(),
        FareToken::ZU => "ZU".to_string(),
        FareToken::None => "None".to_string(),
        FareToken::Free => "Free".to_string(),
        FareToken::KHU => "KHU".to_string(),
        FareToken::KLU => "KLU".to_string(),
        FareToken::KHZ => "KHZ".to_string(),
        FareToken::KLZ => "KLZ".to_string(),
        _ => String::new(),
    }
}

/// C++: `std::string (const FareToken &) FareUtil::tokenToTicket`
// inline std::string tokenToTicket(FareToken const& token) {
//     switch (token) {
//         case FareToken::H:
//             return "Einzelticket Halle";
//         case FareToken::L:
//             return "Einzelticket Leipzig";
//         case FareToken::T1:
//             return "Einzelticket Stadtverkehr 1";
//         case FareToken::T2:
//             return "Einzelticket Stadtverkehr 2";
//         case FareToken::T3:
//             return "Einzelticket Stadtverkehr 3";
//         case FareToken::U:
//             return "Einzelticket";
//         case FareToken::Z:
//             return "Einzelticket";
//         case FareToken::M:
//             return "Einzelticket Verbundpreis";
//         case FareToken::K:
//             return "Kurzstreckenticket";
//         case FareToken::KL:
//             return "Kurzstreckenticket Leipzig";
//         case FareToken::KH:
//             return "Kurzstreckenticket Halle";
//         case FareToken::ZU:
//             return "None";
//         case FareToken::None:
//             return "None";
//         case FareToken::Free:
//             return "Free";
//         case FareToken::KHU:
//             return "Kurzstreckenticket Halle";
//         case FareToken::KLU:
//             return "Kurzstreckenticket Leipzig";
//         case FareToken ::KHZ:
//             return "Kurzstreckenticket Halle";
//         case FareToken ::KLZ:
//             return  "Kurzstreckenticket Leipzig";
//         case FareToken ::START:
//             return "forbidden START";
//     }
//     return ""; //surpress compiler warning
// }
#[allow(unused_variables)]
pub fn token_to_ticket(token: &FareToken) -> String {
        match token {
        FareToken::H => "Einzelticket Halle".to_string(),
        FareToken::L => "Einzelticket Leipzig".to_string(),
        FareToken::T1 => "Einzelticket Stadtverkehr 1".to_string(),
        FareToken::T2 => "Einzelticket Stadtverkehr 2".to_string(),
        FareToken::T3 => "Einzelticket Stadtverkehr 3".to_string(),
        FareToken::U => "Einzelticket".to_string(),
        FareToken::Z => "Einzelticket".to_string(),
        FareToken::M => "Einzelticket Verbundpreis".to_string(),
        FareToken::K => "Kurzstreckenticket".to_string(),
        FareToken::KL => "Kurzstreckenticket Leipzig".to_string(),
        FareToken::KH => "Kurzstreckenticket Halle".to_string(),
        FareToken::ZU => "None".to_string(),
        FareToken::None => "None".to_string(),
        FareToken::Free => "Free".to_string(),
        FareToken::KHU => "Kurzstreckenticket Halle".to_string(),
        FareToken::KLU => "Kurzstreckenticket Leipzig".to_string(),
        FareToken::KHZ => "Kurzstreckenticket Halle".to_string(),
        FareToken::KLZ => "Kurzstreckenticket Leipzig".to_string(),
        FareToken::START => "forbidden START".to_string(),
        _ => String::new(),
    }
}

