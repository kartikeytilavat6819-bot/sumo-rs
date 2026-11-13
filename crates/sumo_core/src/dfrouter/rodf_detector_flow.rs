//! Port of dfrouter/RODFDetectorFlow.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// Definition of the traffic during a certain time containing the flows and speeds
/// C++ class `FlowDef` (dfrouter/RODFDetectorFlow.h)
#[derive(Clone, Copy, Default)]
pub struct FlowDef {
    pub qPKW: f64,
    pub qLKW: f64,
    pub vPKW: f64,
    pub vLKW: f64,
    pub fLKW: f64,
    pub isLKW: f64,
    pub firstSet: bool,
}

impl FlowDef {
}

/// A container for flows
/// C++ class `RODFDetectorFlows` (dfrouter/RODFDetectorFlow.h)
#[derive(Clone)]
pub struct RODFDetectorFlows {
    pub myFastAccessFlows: BTreeMap<String, Vec<FlowDef>>,
    pub myBeginTime: SUMOTime,
    pub myEndTime: SUMOTime,
    pub myStepOffset: SUMOTime,
    /// `mutable` in C++: cached by the const getMaxDetectorFlow()
    pub myMaxDetectorFlow: std::cell::Cell<f64>,
}

impl RODFDetectorFlows {
    /// C++: `void RODFDetectorFlows::RODFDetectorFlows(SUMOTime, SUMOTime, SUMOTime)`
    // RODFDetectorFlows::RODFDetectorFlows(SUMOTime startTime, SUMOTime endTime,
    //                                      SUMOTime stepOffset)
    //     : myBeginTime(startTime), myEndTime(endTime), myStepOffset(stepOffset),
    //       myMaxDetectorFlow(-1) {}
    #[allow(unused_variables)]
    pub fn new(startTime: SUMOTime, endTime: SUMOTime, stepOffset: SUMOTime) -> Self {
        RODFDetectorFlows {
            myFastAccessFlows: BTreeMap::new(),
            myBeginTime: startTime,
            myEndTime: endTime,
            myStepOffset: stepOffset,
            myMaxDetectorFlow: std::cell::Cell::new(-1.0),
        }
    }

    /// C++: `void RODFDetectorFlows::~RODFDetectorFlows()`
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
    }

    /// C++: `void RODFDetectorFlows::addFlow(const std::string &, SUMOTime, const FlowDef &)`
    // void
    // RODFDetectorFlows::addFlow(const std::string& id, SUMOTime t, const FlowDef& fd) {
    //     if (myFastAccessFlows.find(id) == myFastAccessFlows.end()) {
    //         int noItems = (int)((myEndTime - myBeginTime) / myStepOffset);
    //         myFastAccessFlows[id] = std::vector<FlowDef>(noItems);
    //         std::vector<FlowDef>& cflows = myFastAccessFlows[id];
    //         // initialise
    //         for (std::vector<FlowDef>::iterator i = cflows.begin(); i < cflows.end(); ++i) {
    //             (*i).qPKW = 0;
    //             (*i).qLKW = 0;
    //             (*i).vPKW = 0;
    //             (*i).vLKW = 0;
    //             (*i).fLKW = 0;
    //             (*i).isLKW = 0;
    //             (*i).firstSet = true;
    //         }
    //     }
    //     const int index = (int)((t - myBeginTime) / myStepOffset);
    //     assert(index < (int) myFastAccessFlows[id].size());
    //     FlowDef& ofd = myFastAccessFlows[id][index];
    //     if (ofd.firstSet) {
    //         ofd = fd;
    //         ofd.firstSet = false;
    //     } else {
    //         ofd.qLKW = ofd.qLKW + fd.qLKW;
    //         ofd.qPKW = ofd.qPKW + fd.qPKW;
    //         ofd.vLKW = ofd.vLKW + fd.vLKW; //!!! mean value?
    //         ofd.vPKW = ofd.vPKW + fd.vPKW; //!!! mean value?
    //     }
    //     if (ofd.qPKW != 0) {
    //         ofd.fLKW = ofd.qLKW / (ofd.qLKW + ofd.qPKW);
    //     } else {
    //         ofd.fLKW = 1;
    //         ofd.isLKW = 1;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_flow(&mut self, detector_id: &str, timestamp: SUMOTime, fd: &FlowDef) {
        if !self.myFastAccessFlows.contains_key(detector_id) {
            let noItems = ((self.myEndTime - self.myBeginTime) / self.myStepOffset) as usize;
            let mut cflows = vec![FlowDef::default(); noItems];
            for i in cflows.iter_mut() {
                i.qPKW = 0.0;
                i.qLKW = 0.0;
                i.vPKW = 0.0;
                i.vLKW = 0.0;
                i.fLKW = 0.0;
                i.isLKW = 0.0;
                i.firstSet = true;
            }
            self.myFastAccessFlows.insert(detector_id.to_string(), cflows);
        }
        let index = ((timestamp - self.myBeginTime) / self.myStepOffset) as usize;
        let flows = self.myFastAccessFlows.get_mut(detector_id).unwrap();
        assert!(index < flows.len());
        let ofd = &mut flows[index];
        if ofd.firstSet {
            *ofd = *fd;
            ofd.firstSet = false;
        } else {
            ofd.qLKW = ofd.qLKW + fd.qLKW;
            ofd.qPKW = ofd.qPKW + fd.qPKW;
            ofd.vLKW = ofd.vLKW + fd.vLKW; //!!! mean value?
            ofd.vPKW = ofd.vPKW + fd.vPKW; //!!! mean value?
        }
        if ofd.qPKW != 0.0 {
            ofd.fLKW = ofd.qLKW / (ofd.qLKW + ofd.qPKW);
        } else {
            ofd.fLKW = 1.0;
            ofd.isLKW = 1.0;
        }
    }

    /// C++: `void RODFDetectorFlows::removeFlow(const std::string &)`
    // void
    // RODFDetectorFlows::removeFlow(const std::string& detector_id) {
    //     myFastAccessFlows.erase(detector_id);
    // }
    #[allow(unused_variables)]
    pub fn remove_flow(&mut self, detector_id: &str) {
        self.myFastAccessFlows.remove(detector_id);
    }

    /// C++: `void RODFDetectorFlows::setFlows(const std::string &, std::vector<FlowDef> &)`
    // void
    // RODFDetectorFlows::setFlows(const std::string& detector_id,
    //                             std::vector<FlowDef>& flows) {
    //     for (std::vector<FlowDef>::iterator i = flows.begin(); i < flows.end(); ++i) {
    //         FlowDef& ofd = *i;
    //         if (ofd.qLKW != 0 && ofd.qPKW != 0) {
    //             ofd.fLKW = ofd.qLKW / ofd.qPKW;
    //         } else {
    //             ofd.fLKW = 0;
    //         }
    //     }
    //     myFastAccessFlows[detector_id] = flows;
    // }
    #[allow(unused_variables)]
    pub fn set_flows(&mut self, detector_id: &str, arg1: &mut Vec<FlowDef>) {
        for ofd in arg1.iter_mut() {
            if ofd.qLKW != 0.0 && ofd.qPKW != 0.0 {
                ofd.fLKW = ofd.qLKW / ofd.qPKW;
            } else {
                ofd.fLKW = 0.0;
            }
        }
        self.myFastAccessFlows.insert(detector_id.to_string(), arg1.clone());
    }

    /// C++: `const std::vector<FlowDef> & RODFDetectorFlows::getFlowDefs(const std::string &) const` [const]
    // const std::vector<FlowDef>&
    // RODFDetectorFlows::getFlowDefs(const std::string& id) const {
    //     assert(myFastAccessFlows.find(id) != myFastAccessFlows.end());
    //     assert(myFastAccessFlows.find(id)->second.size() != 0);
    //     return myFastAccessFlows.find(id)->second;
    // }
    #[allow(unused_variables)]
    pub fn get_flow_defs(&self, id: &str) -> &Vec<FlowDef> {
        assert!(self.myFastAccessFlows.contains_key(id));
        assert!(self.myFastAccessFlows[id].len() != 0);
        &self.myFastAccessFlows[id]
    }

    /// C++: `bool RODFDetectorFlows::knows(const std::string &) const` [const]
    // bool
    // RODFDetectorFlows::knows(const std::string& det_id) const {
    //     return myFastAccessFlows.find(det_id) != myFastAccessFlows.end();
    // }
    #[allow(unused_variables)]
    pub fn knows(&self, det_id: &str) -> bool {
        self.myFastAccessFlows.contains_key(det_id)
    }

    /// C++: `double RODFDetectorFlows::getFlowSumSecure(const std::string &) const` [const]
    // double
    // RODFDetectorFlows::getFlowSumSecure(const std::string& id) const {
    //     double ret = 0;
    //     if (knows(id)) {
    //         const std::vector<FlowDef>& flows = getFlowDefs(id);
    //         for (std::vector<FlowDef>::const_iterator i = flows.begin(); i != flows.end(); ++i) {
    //             ret += (*i).qPKW;
    //             ret += (*i).qLKW;
    //         }
    //     }
    //     return ret;
    // }
    #[allow(unused_variables)]
    pub fn get_flow_sum_secure(&self, id: &str) -> f64 {
        let mut ret = 0.0;
        if self.knows(id) {
            let flows = self.get_flow_defs(id);
            for i in flows.iter() {
                ret += i.qPKW;
                ret += i.qLKW;
            }
        }
        ret
    }

    /// C++: `double RODFDetectorFlows::getMaxDetectorFlow() const` [const]
    // double
    // RODFDetectorFlows::getMaxDetectorFlow() const {
    //     if (myMaxDetectorFlow < 0) {
    //         double max = 0;
    //         std::map<std::string, std::vector<FlowDef> >::const_iterator j;
    //         for (j = myFastAccessFlows.begin(); j != myFastAccessFlows.end(); ++j) {
    //             double curr = 0;
    //             const std::vector<FlowDef>& flows = (*j).second;
    //             for (std::vector<FlowDef>::const_iterator i = flows.begin(); i != flows.end(); ++i) {
    //                 curr += (*i).qPKW;
    //                 curr += (*i).qLKW;
    //             }
    //             if (max < curr) {
    //                 max = curr;
    //             }
    //         }
    //         myMaxDetectorFlow = max;
    //     }
    //     return myMaxDetectorFlow;
    // }
    #[allow(unused_variables)]
    pub fn get_max_detector_flow(&self) -> f64 {
        if self.myMaxDetectorFlow.get() < 0.0 {
            let mut max = 0.0;
            for (_, flows) in self.myFastAccessFlows.iter() {
                let mut curr = 0.0;
                for i in flows.iter() {
                    curr += i.qPKW;
                    curr += i.qLKW;
                }
                if max < curr {
                    max = curr;
                }
            }
            self.myMaxDetectorFlow.set(max);
        }
        self.myMaxDetectorFlow.get()
    }

    /// C++: `void RODFDetectorFlows::printAbsolute() const` [const]
    // void
    // RODFDetectorFlows::printAbsolute() const {
    //     for (std::map<std::string, std::vector<FlowDef> >::const_iterator i = myFastAccessFlows.begin(); i != myFastAccessFlows.end(); ++i) {
    //         std::cout << (*i).first << ":";
    //         const std::vector<FlowDef>& flows = (*i).second;
    //         double qPKW = 0;
    //         double qLKW = 0;
    //         for (std::vector<FlowDef>::const_iterator j = flows.begin(); j != flows.end(); ++j) {
    //             qPKW += (*j).qPKW;
    //             qLKW += (*j).qLKW;
    //         }
    //         std::cout << qPKW << "/" << qLKW << std::endl;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn print_absolute(&self) {
        for (id, flows) in self.myFastAccessFlows.iter() {
            print!("{}:", id);
            let mut qPKW = 0.0;
            let mut qLKW = 0.0;
            for j in flows.iter() {
                qPKW += j.qPKW;
                qLKW += j.qLKW;
            }
            println!(
                "{}/{}",
                utils::common::to_string::f64_to_string_default(qPKW),
                utils::common::to_string::f64_to_string_default(qLKW)
            );
        }
    }

    /// C++: `void RODFDetectorFlows::mesoJoin(const std::string &, const std::vector<std::string> &)`
    // void
    // RODFDetectorFlows::mesoJoin(const std::string& nid,
    //                             const std::vector<std::string>& oldids) {
    //     for (std::vector<std::string>::const_iterator i = oldids.begin(); i != oldids.end(); ++i) {
    //         if (!knows(*i)) {
    //             continue;
    //         }
    //         std::vector<FlowDef>& flows = myFastAccessFlows[*i];
    //         int index = 0;
    //         for (SUMOTime t = myBeginTime; t != myEndTime; t += myStepOffset) {
    //             addFlow(nid, t, flows[index++]); // !!!
    //         }
    //         myFastAccessFlows.erase(*i);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn meso_join(&mut self, nid: &str, oldids: &Vec<String>) {
        for i in oldids.iter() {
            if !self.knows(i) {
                continue;
            }
            let mut index = 0;
            let mut t = self.myBeginTime;
            while t != self.myEndTime {
                let fd = self.myFastAccessFlows[i.as_str()][index];
                index += 1;
                self.add_flow(nid, t, &fd);
                t += self.myStepOffset;
            }
            self.myFastAccessFlows.remove(i.as_str());
        }
    }

    /// C++: `void RODFDetectorFlows::RODFDetectorFlows(const RODFDetectorFlows &)` [private]
    /// Invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(src: &RODFDetectorFlows) -> Self {
        src.clone()
    }

    /// C++: `RODFDetectorFlows & RODFDetectorFlows::operator=(const RODFDetectorFlows &)` [private]
    /// Invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, src: &RODFDetectorFlows) -> &mut RODFDetectorFlows {
        *self = src.clone();
        self
    }

}

