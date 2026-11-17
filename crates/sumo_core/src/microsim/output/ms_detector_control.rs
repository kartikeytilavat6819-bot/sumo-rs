//! Port of microsim/output/MSDetectorControl.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `std::pair<MSDetectorFileOutput *, OutputDevice *> MSDetectorControl::DetectorFilePair`
pub type MSDetectorControl_DetectorFilePair = (Ptr<MSDetectorFileOutput>, Ptr<OutputDevice>);

/// C++ typedef `std::vector<DetectorFilePair> MSDetectorControl::DetectorFileVec`
pub type MSDetectorControl_DetectorFileVec = Vec<MSDetectorControl_DetectorFilePair>;

/// C++ typedef `std::map<IntervalsKey, DetectorFileVec> MSDetectorControl::Intervals`
pub type MSDetectorControl_Intervals = BTreeMap<MSDetectorControl_IntervalsKey, MSDetectorControl_DetectorFileVec>;

/// C++ typedef `std::pair<SUMOTime, SUMOTime> MSDetectorControl::IntervalsKey`
pub type MSDetectorControl_IntervalsKey = (SUMOTime, SUMOTime);

/// Detectors container; responsible for string and output generation
/// C++ class `MSDetectorControl` (microsim/output/MSDetectorControl.h)
pub struct MSDetectorControl {
    /// The detectors map, first by detector type, then using NamedObjectCont (
    pub myDetectors: BTreeMap<SumoXMLTag, BTreeMap<String, Ptr<MSDetectorFileOutput>>>,
    /// Map that hold DetectorFileVec for given intervals
    pub myIntervals: MSDetectorControl_Intervals,
    /// The map that holds the last call for each sample interval
    pub myLastCalls: BTreeMap<MSDetectorControl_IntervalsKey, SUMOTime>,
    /// List of meanData detectors
    pub myMeanData: BTreeMap<String, Vec<Ptr<MSMeanData>>>,
    /// An empty container to return in getTypedDetectors() if no detectors of the asked type exist
    pub myEmptyContainer: BTreeMap<String, Ptr<MSDetectorFileOutput>>,
    pub myReminders: BTreeMap<String, Ptr<MSMoveReminder>>,
}

impl MSDetectorControl {
    /// C++: `void MSDetectorControl::MSDetectorControl()`
    /// Constructor
    // MSDetectorControl::MSDetectorControl() {
    // }
    #[allow(unused_variables)]
    pub fn new() -> Self {
        MSDetectorControl {
            myDetectors: BTreeMap::new(),
            myIntervals: BTreeMap::new(),
            myLastCalls: BTreeMap::new(),
            myMeanData: BTreeMap::new(),
            myEmptyContainer: BTreeMap::new(),
            myReminders: BTreeMap::new(),
        }
    }

    /// C++: `void MSDetectorControl::~MSDetectorControl()`
    /// Destructor
    // MSDetectorControl::~MSDetectorControl() {
    //     for (auto i = myDetectors.begin(); i != myDetectors.end(); ++i) {
    //         (*i).second.clear();
    //     }
    //     for (auto item : myMeanData) {
    //         for (MSMeanData* md : item.second) {
    //             delete md;
    //         }
    //     }
    //     myMeanData.clear();
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        for (_, dets) in self.myDetectors.iter_mut() {
            dets.clear();
        }
        // shortcut: C++ `delete md` omitted, Ptr<MSMeanData> is non-owning; upgrade when the port settles on an ownership model
        self.myMeanData.clear();
    }

    /// C++: `void MSDetectorControl::close(SUMOTime)`
    /// Closes the detector outputs
    // void
    // MSDetectorControl::close(SUMOTime step) {
    //     // flush the last values
    //     writeOutput(step, true);
    //     // [...] files are closed on another place [...]
    //     myIntervals.clear();
    // }
    #[allow(unused_variables)]
    pub fn close(&mut self, step: SUMOTime) {
        // flush the last values
        self.write_output(step, true);
        // [...] files are closed on another place [...]
        self.myIntervals.clear();
    }

    /// C++: `void MSDetectorControl::add(SumoXMLTag, MSDetectorFileOutput *, const std::string &, SUMOTime, SUMOTime)`
    /// Adds a detector/output combination into the containers
    // void
    // MSDetectorControl::add(SumoXMLTag type, MSDetectorFileOutput* d, const std::string& device, SUMOTime interval, SUMOTime begin) {
    //     if (!myDetectors[type].add(d->getID(), d)) {
    //         const std::string id = d->getID();
    //         delete d;
    //         throw ProcessError(toString(type) + " detector '" + id + "' could not be built (declared twice?).");
    //     }
    //     addDetectorAndInterval(d, &OutputDevice::getDevice(device), interval, begin);
    // }
    #[allow(unused_variables)]
    pub fn add(&mut self, type_: SumoXMLTag, d: Ptr<MSDetectorFileOutput>, device: &str, interval: SUMOTime, begin: SUMOTime) -> Result<(), ProcessError> {
        self.add_2(type_, d)?;
        let dev = OutputDevice::get_device(device, true)?;
        self.add_detector_and_interval(d, Ptr::from_raw(dev.as_ptr()), interval, begin)
    }

    /// C++: `void MSDetectorControl::add(SumoXMLTag, MSDetectorFileOutput *)`
    /// Adds only the detector into the containers
    // void
    // MSDetectorControl::add(SumoXMLTag type, MSDetectorFileOutput* d) {
    //     if (!myDetectors[type].add(d->getID(), d)) {
    //         const std::string id = d->getID();
    //         delete d;
    //         throw ProcessError(toString(type) + " detector '" + id + "' could not be built (declared twice?).");
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_2(&mut self, type_: SumoXMLTag, d: Ptr<MSDetectorFileOutput>) -> Result<(), ProcessError> {
        let id = d.get_id().to_string();
        let dets = self.myDetectors.entry(type_).or_default();
        if dets.contains_key(&id) {
            // shortcut: C++ `delete d` omitted, Ptr<MSDetectorFileOutput> is non-owning
            return Err(ProcessError::new(format!("{} detector '{}' could not be built (declared twice?).", Tags.get_string(type_), id)));
        }
        dets.insert(id, d);
        Ok(())
    }

    /// C++: `void MSDetectorControl::add(MSMeanData *, const std::string &, SUMOTime, SUMOTime)`
    /// Adds a mean data object
    // void
    // MSDetectorControl::add(MSMeanData* md, const std::string& device,
    //                        SUMOTime frequency, SUMOTime begin) {
    //     myMeanData[md->getID()].push_back(md);
    //     addDetectorAndInterval(md, &OutputDevice::getDevice(device), frequency, begin);
    //     if (begin <= string2time(OptionsCont::getOptions().getString("begin"))) {
    //         md->init();
    //     }
    //     MSGlobals::gHaveEmissions |= typeid(*md) == typeid(MSMeanData_Emissions);
    // }
    #[allow(unused_variables)]
    pub fn add_3(&mut self, mn: Ptr<MSMeanData>, device: &str, frequency: SUMOTime, begin: SUMOTime) -> Result<(), ProcessError> {
        self.myMeanData.entry(mn.get_id().to_string()).or_default().push(mn);
        let dev = OutputDevice::get_device(device, true)?;
        self.add_detector_and_interval(Ptr::from_raw(&mut mn.m().base as *mut MSDetectorFileOutput), Ptr::from_raw(dev.as_ptr()), frequency, begin)?;
        if begin <= utils::common::time::string2time(&utils::options::options_cont::get_string("begin"))? {
            mn.m().init();
        }
        // shortcut: MSGlobals::gHaveEmissions |= typeid(*md) == typeid(MSMeanData_Emissions) needs RTTI and MSGlobals statics are not ported, set it once both exist
        Ok(())
    }

    /// C++: `void MSDetectorControl::addDetectorAndInterval(MSDetectorFileOutput *, OutputDevice *, SUMOTime, SUMOTime)`
    /// Adds one of the detectors as a new MSDetectorFileOutput
    // void
    // MSDetectorControl::addDetectorAndInterval(MSDetectorFileOutput* det,
    //         OutputDevice* device,
    //         SUMOTime interval,
    //         SUMOTime begin) {
    //     const SUMOTime simBegin = string2time(OptionsCont::getOptions().getString("begin"));
    //     if (begin == -1) {
    //         begin = simBegin;
    //     }
    //     IntervalsKey key = std::make_pair(interval, begin);
    //     Intervals::iterator it = myIntervals.find(key);
    //     // Add command for given key only once to MSEventControl...
    //     if (it == myIntervals.end()) {
    //         DetectorFileVec detAndFileVec;
    //         detAndFileVec.push_back(std::make_pair(det, device));
    //         myIntervals.insert(std::make_pair(key, detAndFileVec));
    //         SUMOTime lastCall = begin;
    //         if (begin < simBegin) {
    //             SUMOTime divRest = (simBegin - begin) % interval;
    //             lastCall = simBegin - divRest;
    //         }
    //         myLastCalls[key] = lastCall;
    //     } else {
    //         DetectorFileVec& detAndFileVec = it->second;
    //         if (find_if(detAndFileVec.begin(), detAndFileVec.end(), [&](const DetectorFilePair & pair) {
    //         return pair.first == det;
    //     }) == detAndFileVec.end()) {
    //             detAndFileVec.push_back(std::make_pair(det, device));
    //         } else {
    //             // detector already in container. Don't add several times
    //             WRITE_WARNING("MSDetectorControl::addDetectorAndInterval: detector already in container. Ignoring.");
    //             return;
    //         }
    //     }
    //     det->writeXMLDetectorProlog(*device);
    // }
    #[allow(unused_variables)]
    pub fn add_detector_and_interval(&mut self, det: Ptr<MSDetectorFileOutput>, device: Ptr<OutputDevice>, interval: SUMOTime, mut begin: SUMOTime) -> Result<(), ProcessError> {
        let simBegin = utils::common::time::string2time(&utils::options::options_cont::get_string("begin"))?;
        if begin == -1 {
            begin = simBegin;
        }
        let key = (interval, begin);
        // Add command for given key only once to MSEventControl...
        if let Some(detAndFileVec) = self.myIntervals.get_mut(&key) {
            if !detAndFileVec.iter().any(|pair| pair.0 == det) {
                detAndFileVec.push((det, device));
            } else {
                // detector already in container. Don't add several times
                utils::common::msg_handler::write_warning("MSDetectorControl::addDetectorAndInterval: detector already in container. Ignoring.");
                return Ok(());
            }
        } else {
            self.myIntervals.insert(key, vec![(det, device)]);
            let mut lastCall = begin;
            if begin < simBegin {
                let divRest = (simBegin - begin) % interval;
                lastCall = simBegin - divRest;
            }
            self.myLastCalls.insert(key, lastCall);
        }
        det.m().write_xml_detector_prolog(device.m());
        Ok(())
    }

    /// C++: `const std::vector<SumoXMLTag> MSDetectorControl::getAvailableTypes() const` [const]
    /// Returns the list of available detector types
    // const std::vector<SumoXMLTag>
    // MSDetectorControl::getAvailableTypes() const {
    //     std::vector<SumoXMLTag> result;
    //     for (std::map<SumoXMLTag, NamedObjectCont<MSDetectorFileOutput*> >::const_iterator i = myDetectors.begin(); i != myDetectors.end(); ++i) {
    //         result.push_back(i->first);
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_available_types(&self) -> Vec<SumoXMLTag> {
        let mut result = Vec::new();
        for i in self.myDetectors.keys() {
            result.push(*i);
        }
        result
    }

    /// C++: `const NamedObjectCont<MSDetectorFileOutput *> & MSDetectorControl::getTypedDetectors(SumoXMLTag) const` [const]
    /// Returns the list of detectors of the given type
    // const NamedObjectCont<MSDetectorFileOutput*>&
    // MSDetectorControl::getTypedDetectors(SumoXMLTag type) const {
    //     if (myDetectors.find(type) == myDetectors.end()) {
    //         return myEmptyContainer;
    //     }
    //     return myDetectors.find(type)->second;
    // }
    #[allow(unused_variables)]
    pub fn get_typed_detectors(&self, type_: SumoXMLTag) -> &BTreeMap<String, Ptr<MSDetectorFileOutput>> {
        match self.myDetectors.get(&type_) {
            None => &self.myEmptyContainer,
            Some(dets) => dets,
        }
    }

    /// C++: `const std::map<std::string, std::vector<MSMeanData *>> & MSDetectorControl::getMeanData() const` [const]
    // const std::map<std::string, std::vector<MSMeanData*> >& getMeanData() const {
    //         return myMeanData;
    //     }
    #[allow(unused_variables)]
    pub fn get_mean_data(&self) -> &BTreeMap<String, Vec<Ptr<MSMeanData>>> {
        &self.myMeanData
    }

    /// C++: `void MSDetectorControl::updateDetectors(const SUMOTime)`
    /// Computes detector values
    // void
    // MSDetectorControl::updateDetectors(const SUMOTime step) {
    //     for (const auto& i : myDetectors) {
    //         for (const auto& j : getTypedDetectors(i.first)) {
    //             j.second->detectorUpdate(step);
    //         }
    //     }
    //     for (auto item : myMeanData) {
    //         for (MSMeanData* md : item.second) {
    //             md->detectorUpdate(step);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn update_detectors(&mut self, step: SUMOTime) {
        for type_ in self.get_available_types() {
            for j in self.get_typed_detectors(type_).values() {
                j.m().detector_update(step);
            }
        }
        for (_, item) in self.myMeanData.iter() {
            for md in item.iter() {
                md.m().detector_update(step);
            }
        }
    }

    /// C++: `void MSDetectorControl::writeOutput(SUMOTime, bool)`
    /// Writes the output to be generated within the given time step
    // void
    // MSDetectorControl::writeOutput(SUMOTime step, bool closing) {
    //     for (Intervals::iterator i = myIntervals.begin(); i != myIntervals.end(); ++i) {
    //         IntervalsKey interval = (*i).first;
    //         if (myLastCalls[interval] + interval.first <= step || (closing && myLastCalls[interval] < step)) {
    //             DetectorFileVec dfVec = (*i).second;
    //             SUMOTime startTime = myLastCalls[interval];
    //             // check whether at the end the output was already generated
    //             for (DetectorFileVec::iterator it = dfVec.begin(); it != dfVec.end(); ++it) {
    //                 MSDetectorFileOutput* det = it->first;
    //                 det->writeXMLOutput(*(it->second), startTime, step);
    //             }
    //             myLastCalls[interval] = step;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn write_output(&mut self, step: SUMOTime, closing: bool) {
        for (interval, dfVec) in self.myIntervals.iter() {
            let last = *self.myLastCalls.entry(*interval).or_insert(0);
            if last + interval.0 <= step || (closing && last < step) {
                let startTime = last;
                // check whether at the end the output was already generated
                for (det, dev) in dfVec.iter() {
                    det.m().write_xml_output(dev.m(), startTime, step);
                }
                self.myLastCalls.insert(*interval, step);
            }
        }
    }

    /// C++: `void MSDetectorControl::clearState(SUMOTime)`
    /// Remove all vehicles before quick-loading state
    // void
    // MSDetectorControl::clearState(SUMOTime step) {
    //     for (const auto& i : myDetectors) {
    //         for (const auto& j : getTypedDetectors(i.first)) {
    //             j.second->clearState(step);
    //         }
    //     }
    //     myReminders.clear();
    // }
    #[allow(unused_variables)]
    pub fn clear_state(&mut self, step: SUMOTime) {
        for type_ in self.get_available_types() {
            for j in self.get_typed_detectors(type_).values() {
                j.m().clear_state(step);
            }
        }
        self.myReminders.clear();
    }

    /// C++: `const std::map<std::string, MSMoveReminder *> & MSDetectorControl::getAllReminders()`
    /// retrieve reminders of all detectors (for loading state)
    // const std::map<std::string, MSMoveReminder*>&
    // MSDetectorControl::getAllReminders() {
    //     if (myReminders.empty()) {
    //         for (auto item : myMeanData) {
    //             for (MSMeanData* md : item.second) {
    //                 for (MSMoveReminder* rem : md->getReminders()) {
    //                     myReminders[rem->getDescription()] = rem;
    //                 }
    //             }
    //         }
    //         if (myReminders.empty()) {
    //             // ensure non-empty map
    //             myReminders[""] = nullptr;
    //         }
    //     }
    //     return myReminders;
    // }
    #[allow(unused_variables)]
    pub fn get_all_reminders(&mut self) -> &BTreeMap<String, Ptr<MSMoveReminder>> {
        if self.myReminders.is_empty() {
            for (_, item) in self.myMeanData.iter() {
                for md in item.iter() {
                    for rem in md.get_reminders() {
                        self.myReminders.insert(rem.get_description().to_string(), rem);
                    }
                }
            }
            if self.myReminders.is_empty() {
                // ensure non-empty map
                self.myReminders.insert(String::new(), Ptr::default());
            }
        }
        &self.myReminders
    }

    /// C++: `void MSDetectorControl::MSDetectorControl(const MSDetectorControl &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSDetectorControl) -> Self {
        panic!("MSDetectorControl copy constructor is private and invalidated in C++")
    }

    /// C++: `MSDetectorControl & MSDetectorControl::operator=(const MSDetectorControl &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSDetectorControl) -> &mut MSDetectorControl {
        panic!("MSDetectorControl assignment operator is private and invalidated in C++")
    }

}

