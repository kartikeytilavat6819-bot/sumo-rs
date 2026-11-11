//! Port of dfrouter/RODFDetectorFlow.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// Definition of the traffic during a certain time containing the flows and speeds
/// C++ class `FlowDef` (dfrouter/RODFDetectorFlow.h)
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
pub struct RODFDetectorFlows {
    pub myFastAccessFlows: BTreeMap<String, Vec<FlowDef>>,
    pub myBeginTime: SUMOTime,
    pub myEndTime: SUMOTime,
    pub myStepOffset: SUMOTime,
    pub myMaxDetectorFlow: f64,
}

impl RODFDetectorFlows {
    /// C++: `void RODFDetectorFlows::RODFDetectorFlows(SUMOTime, SUMOTime, SUMOTime)`
    // RODFDetectorFlows::RODFDetectorFlows(SUMOTime startTime, SUMOTime endTime,
    //                                      SUMOTime stepOffset)
    //     : myBeginTime(startTime), myEndTime(endTime), myStepOffset(stepOffset),
    //       myMaxDetectorFlow(-1) {}
    #[allow(unused_variables)]
    pub fn new(startTime: SUMOTime, endTime: SUMOTime, stepOffset: SUMOTime) -> Self {
        todo!("port RODFDetectorFlows::RODFDetectorFlows")
    }

    /// C++: `void RODFDetectorFlows::~RODFDetectorFlows()`
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port RODFDetectorFlows::~RODFDetectorFlows")
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
        todo!("port RODFDetectorFlows::addFlow")
    }

    /// C++: `void RODFDetectorFlows::removeFlow(const std::string &)`
    // void
    // RODFDetectorFlows::removeFlow(const std::string& detector_id) {
    //     myFastAccessFlows.erase(detector_id);
    // }
    #[allow(unused_variables)]
    pub fn remove_flow(&mut self, detector_id: &str) {
        todo!("port RODFDetectorFlows::removeFlow")
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
        todo!("port RODFDetectorFlows::setFlows")
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
        todo!("port RODFDetectorFlows::getFlowDefs")
    }

    /// C++: `bool RODFDetectorFlows::knows(const std::string &) const` [const]
    // bool
    // RODFDetectorFlows::knows(const std::string& det_id) const {
    //     return myFastAccessFlows.find(det_id) != myFastAccessFlows.end();
    // }
    #[allow(unused_variables)]
    pub fn knows(&self, det_id: &str) -> bool {
        todo!("port RODFDetectorFlows::knows")
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
        todo!("port RODFDetectorFlows::getFlowSumSecure")
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
        todo!("port RODFDetectorFlows::getMaxDetectorFlow")
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
        todo!("port RODFDetectorFlows::printAbsolute")
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
        todo!("port RODFDetectorFlows::mesoJoin")
    }

    /// C++: `void RODFDetectorFlows::RODFDetectorFlows(const RODFDetectorFlows &)` [private]
    /// Invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(src: &RODFDetectorFlows) -> Self {
        todo!("port RODFDetectorFlows::RODFDetectorFlows")
    }

    /// C++: `RODFDetectorFlows & RODFDetectorFlows::operator=(const RODFDetectorFlows &)` [private]
    /// Invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, src: &RODFDetectorFlows) -> &mut RODFDetectorFlows {
        todo!("port RODFDetectorFlows::operator=")
    }

}

