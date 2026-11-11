//! Port of netimport/vissim/tempstructs/NIVissimTrafficDescription.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `std::map<int, NIVissimTrafficDescription *> NIVissimTrafficDescription::DictType`
pub type NIVissimTrafficDescription_DictType = BTreeMap<i32, Ptr<NIVissimTrafficDescription>>;

/// C++ class `NIVissimTrafficDescription` (netimport/vissim/tempstructs/NIVissimTrafficDescription.h)
pub struct NIVissimTrafficDescription {
    pub myName: String,
    pub myVehicleTypes: NIVissimVehicleClassVector,
}

// C++ static members of NIVissimTrafficDescription (port as module-level statics, see AGENTS.md):
//   static DictType NIVissimTrafficDescription::myDict;
thread_local! {
    static MY_DICT: std::cell::RefCell<NIVissimTrafficDescription_DictType> = std::cell::RefCell::new(BTreeMap::new());
}

impl NIVissimTrafficDescription {
    /// C++: `void NIVissimTrafficDescription::NIVissimTrafficDescription(const std::string &, const NIVissimVehicleClassVector &)`
    // NIVissimTrafficDescription::NIVissimTrafficDescription(
    //     const std::string& name,
    //     const NIVissimVehicleClassVector& vehicleTypes)
    //     : myName(name), myVehicleTypes(vehicleTypes) {}
    #[allow(unused_variables)]
    pub fn new(name: &str, vehicleTypes: &NIVissimVehicleClassVector) -> Self {
        Self {
            myName: name.to_string(),
            myVehicleTypes: vehicleTypes.clone(),
        }
    }

    /// C++: `void NIVissimTrafficDescription::~NIVissimTrafficDescription()`
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor is implicit; nothing to release.
    }

    /// C++: `bool NIVissimTrafficDescription::dictionary(int, const std::string &, const NIVissimVehicleClassVector &)` [static]
    // bool
    // NIVissimTrafficDescription::dictionary(int id,
    //                                        const std::string& name,
    //                                        const NIVissimVehicleClassVector& vehicleTypes) {
    //     NIVissimTrafficDescription* o = new NIVissimTrafficDescription(name, vehicleTypes);
    //     if (!dictionary(id, o)) {
    //         delete o;
    //         return false;
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn dictionary(id: i32, name: &str, vehicleTypes: &NIVissimVehicleClassVector) -> bool {
        let o = Ptr::from_raw(Box::into_raw(Box::new(NIVissimTrafficDescription::new(name, vehicleTypes))));
        if !NIVissimTrafficDescription::dictionary_2(id, o) {
            // delete o;
            drop(unsafe { Box::from_raw(o.as_raw()) });
            return false;
        }
        true
    }

    /// C++: `bool NIVissimTrafficDescription::dictionary(int, NIVissimTrafficDescription *)` [static]
    // bool
    // NIVissimTrafficDescription::dictionary(int id, NIVissimTrafficDescription* o) {
    //     DictType::iterator i = myDict.find(id);
    //     if (i == myDict.end()) {
    //         myDict[id] = o;
    //         return true;
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn dictionary_2(id: i32, o: Ptr<NIVissimTrafficDescription>) -> bool {
        MY_DICT.with(|d| {
            let mut d = d.borrow_mut();
            if !d.contains_key(&id) {
                d.insert(id, o);
                return true;
            }
            false
        })
    }

    /// C++: `NIVissimTrafficDescription * NIVissimTrafficDescription::dictionary(int)` [static]
    // NIVissimTrafficDescription*
    // NIVissimTrafficDescription::dictionary(int id) {
    //     DictType::iterator i = myDict.find(id);
    //     if (i == myDict.end()) {
    //         return nullptr;
    //     }
    //     return (*i).second;
    // }
    #[allow(unused_variables)]
    pub fn dictionary_3(id: i32) -> Ptr<NIVissimTrafficDescription> {
        MY_DICT.with(|d| d.borrow().get(&id).copied().unwrap_or_else(Ptr::null))
    }

    /// C++: `void NIVissimTrafficDescription::clearDict()` [static]
    // void
    // NIVissimTrafficDescription::clearDict() {
    //     for (DictType::iterator i = myDict.begin(); i != myDict.end(); i++) {
    //         delete (*i).second;
    //     }
    //     myDict.clear();
    // }
    #[allow(unused_variables)]
    pub fn clear_dict() {
        MY_DICT.with(|d| {
            let mut d = d.borrow_mut();
            for (_, o) in d.iter() {
                // delete (*i).second;
                drop(unsafe { Box::from_raw(o.as_raw()) });
            }
            d.clear();
        })
    }

    /// C++: `double NIVissimTrafficDescription::meanSpeed(int)` [static]
    // double
    // NIVissimTrafficDescription::meanSpeed(int id) {
    //     NIVissimTrafficDescription* i = dictionary(id);
    //     assert(i != 0);
    //     return i->meanSpeed();
    // }
    #[allow(unused_variables)]
    pub fn mean_speed(id: i32) -> f64 {
        let i = NIVissimTrafficDescription::dictionary_3(id);
        assert!(!i.is_null());
        i.mean_speed_2()
    }

    /// C++: `double NIVissimTrafficDescription::meanSpeed() const` [const]
    // double
    // NIVissimTrafficDescription::meanSpeed() const {
    //     double speed = 0;
    //     for (NIVissimVehicleClassVector::const_iterator i = myVehicleTypes.begin(); i != myVehicleTypes.end(); i++) {
    //         speed += (*i)->getSpeed();
    //     }
    //     return speed / (double) myVehicleTypes.size();
    // }
    #[allow(unused_variables)]
    pub fn mean_speed_2(&self) -> f64 {
        let mut speed = 0.0;
        for i in self.myVehicleTypes.iter() {
            speed += i.get_speed() as f64;
        }
        speed / self.myVehicleTypes.len() as f64
    }

}

