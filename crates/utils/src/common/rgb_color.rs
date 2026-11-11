//! Port of utils/common/RGBColor.{h,cpp}
use super::msg_handler::write_error;
use super::rand_helper::SumoRNG;
use super::string_tokenizer::StringTokenizer;
use super::string_utils;
use super::to_string::ToStringPrec;
use super::util_exceptions::ProcessError;
use std::fmt;
use std::sync::{LazyLock, Mutex};

#[derive(Debug, Clone, Copy, Eq)]
pub struct RGBColor {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8,
    valid: bool,
    random: bool,
}

/// random colors do not affect the simulation. No initialization is necessary
static COLOR_RNG: LazyLock<Mutex<SumoRNG>> = LazyLock::new(|| Mutex::new(SumoRNG::new("color")));

impl Default for RGBColor {
    fn default() -> Self {
        Self::new_valid(true)
    }
}


impl RGBColor {
    pub const RED: RGBColor = RGBColor::new(255, 0, 0, 255);
    pub const GREEN: RGBColor = RGBColor::new(0, 255, 0, 255);
    pub const BLUE: RGBColor = RGBColor::new(0, 0, 255, 255);
    pub const YELLOW: RGBColor = RGBColor::new(255, 255, 0, 255);
    pub const CYAN: RGBColor = RGBColor::new(0, 255, 255, 255);
    pub const MAGENTA: RGBColor = RGBColor::new(255, 0, 255, 255);
    pub const ORANGE: RGBColor = RGBColor::new(255, 128, 0, 255);
    pub const WHITE: RGBColor = RGBColor::new(255, 255, 255, 255);
    pub const BLACK: RGBColor = RGBColor::new(0, 0, 0, 255);
    pub const GREY: RGBColor = RGBColor::new(128, 128, 128, 255);
    pub const INVISIBLE: RGBColor = RGBColor::new(0, 0, 0, 0);
    pub const DEFAULT_COLOR: RGBColor = RGBColor::YELLOW;
    pub const DEFAULT_COLOR_STRING: &'static str = "yellow";

    /// RGBColor(bool valid = true)
    pub const fn new_valid(valid: bool) -> Self {
        RGBColor { red: 0, green: 0, blue: 0, alpha: 0, valid, random: false }
    }

    pub const fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self::new_random(red, green, blue, alpha, false)
    }

    pub const fn new_random(red: u8, green: u8, blue: u8, alpha: u8, random: bool) -> Self {
        RGBColor { red, green, blue, alpha, valid: true, random }
    }

    pub fn red(&self) -> u8 {
        self.red
    }
    pub fn green(&self) -> u8 {
        self.green
    }
    pub fn blue(&self) -> u8 {
        self.blue
    }
    pub fn alpha(&self) -> u8 {
        self.alpha
    }

    pub fn set(&mut self, r: u8, g: u8, b: u8, a: u8) {
        self.red = r;
        self.green = g;
        self.blue = b;
        self.alpha = a;
        self.valid = true;
    }

    pub fn set_alpha(&mut self, alpha: u8) {
        self.alpha = alpha;
    }

    pub fn set_valid(&mut self, value: bool) {
        self.valid = value;
    }

    pub fn is_valid(&self) -> bool {
        self.valid
    }

    pub fn is_random(&self) -> bool {
        self.random
    }

    fn same(&self, c: &RGBColor) -> bool {
        self.red == c.red && self.green == c.green && self.blue == c.blue && self.alpha == c.alpha && self.valid == c.valid
    }

    pub fn inverted_color(&self) -> RGBColor {
        RGBColor::new(255 - self.red, 255 - self.green, 255 - self.blue, self.alpha)
    }

    pub fn get_brightness(&self) -> i32 {
        (299 * self.red as i32 + 587 * self.green as i32 + 114 * self.blue as i32) / 1000
    }

    pub fn get_contrasting_color(&self) -> RGBColor {
        if self.get_brightness() > 180 {
            let mut result = self.changed_brightness(-150, 3);
            result.set_alpha(255);
            result
        } else {
            RGBColor::WHITE
        }
    }

    pub fn with_color_rng<R>(f: impl FnOnce(&mut SumoRNG) -> R) -> R {
        f(&mut COLOR_RNG.lock().unwrap_or_else(|e| e.into_inner()))
    }

    /// changedBrightness(change, toChange = 3)
    pub fn changed_brightness(&self, change: i32, to_change: i32) -> RGBColor {
        let clamp = |v: u8| (v as i32 + change).clamp(0, 255);
        let (red, blue, green) = (clamp(self.red), clamp(self.blue), clamp(self.green));
        let changed = (red - self.red as i32) + (blue - self.blue as i32) + (green - self.green as i32);
        let result = RGBColor::new(red as u8, green as u8, blue as u8, self.alpha);
        if changed == to_change * change || changed == 0 {
            return result;
        }
        let maxed_colors = (red != self.red as i32 + change) as i32
            + (blue != self.blue as i32 + change) as i32
            + (green != self.green as i32 + change) as i32;
        if maxed_colors == 3 {
            result
        } else {
            let to_change_next = 3 - maxed_colors;
            result.changed_brightness((to_change * change - changed) / to_change_next, to_change_next)
        }
    }

    pub fn changed_alpha(&self, change: i32) -> RGBColor {
        let alpha = (self.alpha as i32 + change).clamp(0, 255);
        RGBColor::new(self.red, self.green, self.blue, alpha as u8)
    }

    pub fn multiply(&self, factor: f64) -> RGBColor {
        let m = |v: u8| ((v as f64 * factor).clamp(0.0, 255.0) + 0.5).floor() as u8;
        RGBColor::new(m(self.red), m(self.green), m(self.blue), self.alpha)
    }

    pub fn parse_color(coldef: &str) -> Result<RGBColor, ProcessError> {
        let coldef = string_utils::to_lower_case(coldef);
        match coldef.as_str() {
            "red" => return Ok(Self::RED),
            "green" => return Ok(Self::GREEN),
            "blue" => return Ok(Self::BLUE),
            "yellow" => return Ok(Self::YELLOW),
            "cyan" => return Ok(Self::CYAN),
            "magenta" => return Ok(Self::MAGENTA),
            "orange" => return Ok(Self::ORANGE),
            "white" => return Ok(Self::WHITE),
            "black" => return Ok(Self::BLACK),
            "grey" | "gray" => return Ok(Self::GREY),
            "invisible" => return Ok(Self::INVISIBLE),
            "random" => {
                return Ok(Self::with_color_rng(|rng| {
                    let h = rng.rand_int(360) as f64;
                    // prefer more saturated colors
                    let s = rng.rand().powf(0.3);
                    // prefer brighter colors
                    let v = rng.rand().powf(0.3);
                    Self::from_hsv(h, s, v, true)
                }));
            }
            _ => {}
        }
        let (r, g, b): (u8, u8, u8);
        let mut a = 255u8;
        if coldef.starts_with('#') {
            let coldesc = string_utils::hex_to_int(&coldef)? as u32;
            if coldef.len() == 7 {
                r = ((coldesc & 0xFF0000) >> 16) as u8;
                g = ((coldesc & 0x00FF00) >> 8) as u8;
                b = (coldesc & 0xFF) as u8;
            } else if coldef.len() == 9 {
                r = ((coldesc & 0xFF000000) >> 24) as u8;
                g = ((coldesc & 0x00FF0000) >> 16) as u8;
                b = ((coldesc & 0x0000FF00) >> 8) as u8;
                a = (coldesc & 0xFF) as u8;
            } else {
                return Err(ProcessError::EmptyData);
            }
        } else {
            let st = StringTokenizer::with_token(&coldef, ",", false).get_vector();
            if st.len() == 3 || st.len() == 4 {
                let ints = || -> Result<(u8, u8, u8, u8), ProcessError> {
                    let r = string_utils::to_int(&st[0])? as u8;
                    let g = string_utils::to_int(&st[1])? as u8;
                    let b = string_utils::to_int(&st[2])? as u8;
                    let a = if st.len() == 4 { string_utils::to_int(&st[3])? as u8 } else { 255 };
                    if r <= 1 && g <= 1 && b <= 1 && (st.len() == 3 || a <= 1) {
                        return Err(ProcessError::NumberFormat(format!("(color component) {coldef}")));
                    }
                    Ok((r, g, b, a))
                };
                match ints() {
                    Ok(c) => (r, g, b, a) = c,
                    Err(ProcessError::NumberFormat(_)) => {
                        let d = |s: &str| -> Result<u8, ProcessError> { Ok((string_utils::to_double(s)? * 255. + 0.5) as u8) };
                        r = d(&st[0])?;
                        g = d(&st[1])?;
                        b = d(&st[2])?;
                        if st.len() == 4 {
                            a = d(&st[3])?;
                        }
                    }
                    Err(e) => return Err(e),
                }
            } else {
                return Err(ProcessError::Format(format!("Invalid color definition '{coldef}'")));
            }
        }
        Ok(RGBColor::new(r, g, b, a))
    }

    pub fn is_color(coldef: &str) -> bool {
        // check if is defined using a previous defined color
        let coldef = string_utils::to_lower_case(coldef);
        if matches!(
            coldef.as_str(),
            "red" | "green" | "blue" | "yellow" | "cyan" | "magenta" | "orange" | "white" | "black" | "grey" | "gray" | "invisible" | "random"
        ) {
            return true;
        }
        // check if is defined using an hexadecimal value
        if coldef.starts_with('#') {
            return string_utils::is_hex(&coldef) && (coldef.len() == 7 || coldef.len() == 9);
        }
        // Check definition by tuple of rgb or rgba
        let st = StringTokenizer::with_token(&coldef, ",", false).get_vector();
        (st.len() == 3 || st.len() == 4) && st.iter().all(|s| string_utils::is_double(s))
    }

    /// Err only for errors other than number format / empty data (they set ok = false and write an error)
    pub fn parse_color_reporting(
        coldef: &str,
        objecttype: &str,
        objectid: Option<&str>,
        _report: bool,
        ok: &mut bool,
    ) -> Result<RGBColor, ProcessError> {
        match Self::parse_color(coldef) {
            Ok(c) => return Ok(c),
            Err(ProcessError::NumberFormat(_) | ProcessError::EmptyData) => {}
            Err(e) => return Err(e),
        }
        *ok = false;
        let mut oss = String::from("Attribute 'color' in definition of ");
        if objectid.is_none() {
            oss += "a ";
        }
        oss += objecttype;
        if let Some(id) = objectid {
            oss += &format!(" '{id}'");
        }
        oss += " is not a valid color.";
        write_error(&oss);
        Ok(RGBColor::default())
    }

    pub fn interpolate(min_color: &RGBColor, max_color: &RGBColor, weight: f64) -> RGBColor {
        let weight = weight.clamp(0., 1.);
        let i = |a: u8, b: u8| (a as i32 as f64 + ((b as i32 - a as i32) as f64 * weight)) as u8;
        RGBColor::new(
            i(min_color.red, max_color.red),
            i(min_color.green, max_color.green),
            i(min_color.blue, max_color.blue),
            i(min_color.alpha, max_color.alpha),
        )
    }

    pub fn from_hsv(h: f64, s: f64, v: f64, random: bool) -> RGBColor {
        let mut h = h.clamp(0., 360.);
        let s = s.clamp(0., 1.);
        let v = v.clamp(0., 1.);
        h /= 60.;
        let i = h.floor() as i32;
        let mut f = h - i as f64;
        if i % 2 == 0 {
            f = 1. - f;
        }
        let m = (v * (1. - s) * 255. + 0.5) as u8;
        let n = (v * (1. - s * f) * 255. + 0.5) as u8;
        let vv = (v * 255. + 0.5) as u8;
        match i {
            0 | 6 => RGBColor::new_random(vv, n, m, 255, random),
            1 => RGBColor::new_random(n, vv, m, 255, random),
            2 => RGBColor::new_random(m, vv, n, 255, random),
            3 => RGBColor::new_random(m, n, vv, 255, random),
            4 => RGBColor::new_random(n, m, vv, 255, random),
            5 => RGBColor::new_random(vv, m, n, 255, random),
            _ => RGBColor::new_random(255, 255, 255, 255, random),
        }
    }

    /// randomHue(s = 1, v = 1, random = false)
    pub fn random_hue(s: f64, v: f64, random: bool) -> RGBColor {
        let h = Self::with_color_rng(|rng| rng.rand_int(360)) as f64;
        Self::from_hsv(h, s, v, random)
    }
}

/// equality ignores the random flag like the C++ operator==
impl PartialEq for RGBColor {
    fn eq(&self, c: &RGBColor) -> bool {
        self.same(c)
    }
}

impl fmt::Display for RGBColor {
    fn fmt(&self, os: &mut fmt::Formatter<'_>) -> fmt::Result {
        let named = [
            (Self::RED, "red"),
            (Self::GREEN, "green"),
            (Self::BLUE, "blue"),
            (Self::YELLOW, "yellow"),
            (Self::CYAN, "cyan"),
            (Self::MAGENTA, "magenta"),
            (Self::ORANGE, "orange"),
            (Self::WHITE, "white"),
            (Self::BLACK, "black"),
            (Self::GREY, "grey"),
            (Self::INVISIBLE, "invisible"),
        ];
        if let Some((_, n)) = named.iter().find(|(c, _)| c == self) {
            return os.write_str(n);
        }
        write!(os, "{},{},{}", self.red, self.green, self.blue)?;
        if self.alpha < 255 {
            write!(os, ",{}", self.alpha)?;
        }
        Ok(())
    }
}

impl ToStringPrec for RGBColor {
    fn to_string_prec(&self, _accuracy: i32) -> String {
        self.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_and_print() {
        assert_eq!(RGBColor::parse_color("Red").unwrap(), RGBColor::RED);
        assert_eq!(RGBColor::parse_color("1,0,0").unwrap(), RGBColor::RED);
        assert_eq!(RGBColor::parse_color("255,0,0").unwrap(), RGBColor::RED);
        assert_eq!(RGBColor::parse_color("0.2,0.4,0.6").unwrap().to_string(), "51,102,153");
        assert_eq!(RGBColor::parse_color("0.5,0.5,0.5").unwrap(), RGBColor::GREY);
        assert_eq!(RGBColor::parse_color("#ff8000").unwrap(), RGBColor::ORANGE);
        assert_eq!(RGBColor::parse_color("10,20,30,40").unwrap().to_string(), "10,20,30,40");
        assert_eq!(RGBColor::parse_color("1,2").unwrap_err().to_string(), "Invalid color definition '1,2'");
        assert_eq!(RGBColor::parse_color("#1234").unwrap_err(), ProcessError::EmptyData);
        assert!(RGBColor::is_color("#00ff00") && RGBColor::is_color("1,1,1") && !RGBColor::is_color("x"));
        assert_eq!(RGBColor::DEFAULT_COLOR.to_string(), RGBColor::DEFAULT_COLOR_STRING);
        let mut ok = true;
        RGBColor::parse_color_reporting("a,b,c", "vehicle", Some("v0"), true, &mut ok).unwrap();
        assert!(!ok);
    }

    #[test]
    fn manipulation() {
        assert_eq!(RGBColor::new(250, 10, 10, 255).changed_brightness(10, 3), RGBColor::new(255, 22, 22, 255));
        assert_eq!(RGBColor::BLACK.inverted_color(), RGBColor::WHITE);
        assert_eq!(RGBColor::WHITE.multiply(0.5), RGBColor::new(128, 128, 128, 255));
        assert_eq!(RGBColor::interpolate(&RGBColor::BLACK, &RGBColor::WHITE, 0.5), RGBColor::new(127, 127, 127, 255));
        assert_eq!(RGBColor::from_hsv(120., 1., 1., false), RGBColor::GREEN);
        assert!(RGBColor::parse_color("random").unwrap().is_random());
    }
}
