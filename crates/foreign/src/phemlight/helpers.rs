//! Port of foreign/PHEMlight/cpp/Helpers.{h,cpp}

use super::constants::*;

/// `(int)VEH.find(pattern) > 0` in C++: npos casts to -1, so only a hit past position 0 counts.
fn found_after_start(veh: &str, pattern: &str) -> bool {
    matches!(veh.find(pattern), Some(p) if p > 0)
}

#[derive(Debug, Default, Clone)]
pub struct Helpers {
    v_class: String,
    e_class: String,
    t_class: String,
    s_class: String,
    g_class: String,
    err_msg: String,
    comment_prefix: String,
    phem_data_v: String,
}

impl Helpers {
    pub fn get_v_class(&self) -> &str {
        &self.v_class
    }

    pub fn set_v_class(&mut self, value: &str) {
        self.v_class = value.to_string();
    }

    pub fn get_e_class(&self) -> &str {
        &self.e_class
    }

    pub fn set_e_class(&mut self, value: &str) {
        self.e_class = value.to_string();
    }

    pub fn get_t_class(&self) -> &str {
        &self.t_class
    }

    pub fn set_t_class(&mut self, value: &str) {
        self.t_class = value.to_string();
    }

    pub fn get_s_class(&self) -> &str {
        &self.s_class
    }

    pub fn set_s_class(&mut self, value: &str) {
        self.s_class = value.to_string();
    }

    pub fn get_g_class(&self) -> &str {
        &self.g_class
    }

    pub fn set_g_class(&mut self, value: &str) {
        self.g_class = value.to_string();
    }

    pub fn get_err_msg(&self) -> &str {
        &self.err_msg
    }

    pub fn set_err_msg(&mut self, value: &str) {
        self.err_msg = value.to_string();
    }

    pub fn get_comment_prefix(&self) -> &str {
        &self.comment_prefix
    }

    pub fn set_comment_prefix(&mut self, value: &str) {
        self.comment_prefix = value.to_string();
    }

    pub fn get_phem_data_v(&self) -> &str {
        &self.phem_data_v
    }

    pub fn set_phem_data_v(&mut self, value: &str) {
        self.phem_data_v = value.to_string();
    }

    /// Get vehicle class
    fn getvclass(&mut self, veh: &str) -> bool {
        // Set the drive train efficency
        set_drive_train_efficiency(DRIVE_TRAIN_EFFICIENCY_ALL);

        // Get the vehicle class
        if veh.contains(STR_PKW) {
            self.v_class = STR_PKW.to_string();
            return true;
        } else if veh.contains(STR_LNF) {
            self.v_class = STR_LNF.to_string();
            return true;
        } else if veh.contains(STR_LKW) {
            self.v_class = STR_LKW.to_string();
            return true;
        } else if veh.contains(STR_LSZ) {
            self.v_class = STR_LSZ.to_string();
            return true;
        } else if veh.contains(STR_RB) {
            self.v_class = STR_RB.to_string();
            return true;
        } else if veh.contains(STR_LB) {
            self.v_class = STR_LB.to_string();
            set_drive_train_efficiency(DRIVE_TRAIN_EFFICIENCY_CB);
            return true;
        } else if veh.contains(STR_MR2) {
            self.v_class = STR_MR2.to_string();
            return true;
        } else if veh.contains(STR_MR4) {
            self.v_class = STR_MR4.to_string();
            return true;
        } else if veh.contains(STR_KKR) {
            self.v_class = STR_KKR.to_string();
            return true;
        }
        // Should never happens
        self.err_msg = format!("Vehicle class not defined! ({})", veh);
        false
    }

    /// Get technologie Class
    fn gettclass(&mut self, veh: &str) -> bool {
        if found_after_start(veh, &format!("_{}", STR_DIESEL)) {
            if found_after_start(veh, &format!("_{}", STR_HYBRID)) {
                self.t_class = format!("{}_{}", STR_DIESEL, STR_HYBRID);
                return true;
            } else {
                self.t_class = STR_DIESEL.to_string();
                return true;
            }
        } else if found_after_start(veh, &format!("_{}", STR_GASOLINE)) {
            if found_after_start(veh, &format!("_{}", STR_HYBRID)) {
                self.t_class = format!("{}_{}", STR_GASOLINE, STR_HYBRID);
                return true;
            } else {
                self.t_class = STR_GASOLINE.to_string();
                return true;
            }
        } else if found_after_start(veh, &format!("_{}", STR_CNG)) {
            self.t_class = STR_CNG.to_string();
            return true;
        } else if found_after_start(veh, &format!("_{}", STR_BEV)) {
            self.t_class = STR_BEV.to_string();
            return true;
        }
        // Should never happens
        self.err_msg = format!("Fuel class not defined! ({})", veh);
        false
    }

    /// Get size class
    fn getsclass(&mut self, veh: &str) -> bool {
        if veh.contains(STR_LKW) {
            if found_after_start(veh, &format!("_{}", STR_SII)) {
                self.s_class = STR_SII.to_string();
                true
            } else if found_after_start(veh, &format!("_{}", STR_SI)) {
                self.s_class = STR_SI.to_string();
                true
            } else {
                // Should never happen
                self.err_msg = format!("Size class not defined! ({})", veh);
                false
            }
        } else if veh.contains(STR_LNF) {
            if found_after_start(veh, &format!("_{}", STR_SIII)) {
                self.s_class = STR_SIII.to_string();
                true
            } else if found_after_start(veh, &format!("_{}", STR_SII)) {
                self.s_class = STR_SII.to_string();
                true
            } else if found_after_start(veh, &format!("_{}", STR_SI)) {
                self.s_class = STR_SI.to_string();
                true
            } else {
                // C++ calls substr(npos, ...) and throws std::out_of_range when there is no
                // backslash; shortcut: fall back to the whole string instead of throwing.
                let tail = match veh.rfind('\\') {
                    Some(p) => &veh[p..],
                    None => veh,
                };
                self.err_msg = format!("Size class not defined! ({})", tail);
                false
            }
        } else {
            self.s_class = String::new();
            true
        }
    }

    /// Get euro class
    fn geteclass(&mut self, veh: &str) -> bool {
        // Euro class: text after "_EU" up to the next '_', else up to the next '.', else the end.
        // "_EU" is ASCII, so the byte offsets below are char boundaries.
        if let Some(eu) = veh.find("_EU").filter(|&p| p > 0) {
            let start = eu + 3;
            let rest = &veh[eu + 1..];
            if let Some(q) = rest.find('_') {
                self.e_class = format!("{}{}", STR_EU, &veh[start..eu + 1 + q]);
                true
            } else if let Some(q) = rest.find('.') {
                self.e_class = format!("{}{}", STR_EU, &veh[start..eu + 1 + q]);
                true
            } else {
                self.e_class = format!("{}{}", STR_EU, &veh[start..]);
                true
            }
        } else if found_after_start(veh, &format!("_{}", STR_BEV)) {
            self.e_class = String::new();
            true
        } else {
            // Should never happens
            self.err_msg = format!("Euro class not defined! ({})", veh);
            false
        }
    }

    /// Set complete class string
    pub fn setclass(&mut self, veh: &str) -> bool {
        if self.getvclass(veh) {
            self.g_class = self.v_class.clone();
        } else {
            return false;
        }
        if self.getsclass(veh) {
            if !self.s_class.is_empty() {
                self.g_class = format!("{}_{}", self.g_class, self.s_class);
            }
        } else {
            return false;
        }
        if self.gettclass(veh) {
            self.g_class = format!("{}_{}", self.g_class, self.t_class);
        } else {
            return false;
        }
        if self.geteclass(veh) {
            if !self.e_class.is_empty() {
                self.g_class = format!("{}_{}", self.g_class, self.e_class);
            }
        } else {
            return false;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn setclass_builds_full_class_string() {
        let _guard = EFFICIENCY_TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let mut h = Helpers::default();
        assert!(h.setclass("HDV_RT_I_D_EU5_x.veh"));
        // HDV_RT -> size I (from "_I"), fuel D, euro "5" (up to '_' after "_EU")
        assert_eq!(h.get_g_class(), "HDV_RT_I_D_EU5");
        assert_eq!(h.get_v_class(), "HDV_RT");
    }

    #[test]
    fn setclass_bev_has_no_euro_class_and_drive_efficiency_cb_for_lb() {
        let _guard = EFFICIENCY_TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let mut h = Helpers::default();
        assert!(h.setclass("PC_BEV"));
        assert_eq!(h.get_g_class(), "PC_BEV");
        assert!(h.setclass("HDV_CB_D_EU6"));
        assert_eq!(get_drive_train_efficiency(), DRIVE_TRAIN_EFFICIENCY_CB);
        assert!(h.setclass("PC_D_EU6"));
        assert_eq!(get_drive_train_efficiency(), DRIVE_TRAIN_EFFICIENCY_ALL);
    }

    #[test]
    fn setclass_unknown_vehicle_sets_error() {
        let _guard = EFFICIENCY_TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let mut h = Helpers::default();
        assert!(!h.setclass("BUS_D"));
        assert_eq!(h.get_err_msg(), "Vehicle class not defined! (BUS_D)");
    }
}
