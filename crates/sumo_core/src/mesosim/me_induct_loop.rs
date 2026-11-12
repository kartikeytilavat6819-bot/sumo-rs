//! Port of mesosim/MEInductLoop.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// An induction loop for mesoscopic simulation
/// C++ class `MEInductLoop` (mesosim/MEInductLoop.h)
#[repr(C)]
pub struct MEInductLoop {
    /// C++ base class `MSDetectorFileOutput`
    pub base: MSDetectorFileOutput,
    /// name
    pub myName: String,
    /// mesoscopic edge segment the loop lies on
    pub mySegment: Ptr<MESegment>,
    /// position from the start of the edge / lane
    pub myPosition: f64,
    /// data collector for the loop
    pub myMeanData: MSMeanData_Net_MSLaneMeanDataValues,
}

impl std::ops::Deref for MEInductLoop {
    type Target = MSDetectorFileOutput;
    fn deref(&self) -> &MSDetectorFileOutput {
        &self.base
    }
}
impl std::ops::DerefMut for MEInductLoop {
    fn deref_mut(&mut self) -> &mut MSDetectorFileOutput {
        &mut self.base
    }
}

impl MEInductLoop {
    /// C++: `void MEInductLoop::MEInductLoop(const std::string &, MESegment *, double, const std::string, const std::string &, const std::string &, int)`
    // MEInductLoop::MEInductLoop(const std::string& id,
    //                            MESegment* s,
    //                            double positionInMeters,
    //                            const std::string name, const std::string& vTypes,
    //                            const std::string& nextEdges,
    //                            int detectPersons) :
    //     MSDetectorFileOutput(id, vTypes, nextEdges, detectPersons),
    //     myName(name),
    //     mySegment(s),
    //     myPosition(positionInMeters),
    //     myMeanData(nullptr, mySegment->getLength(), false, nullptr) {
    //     myMeanData.setDescription("inductionLoop_" + id);
    //     s->addDetector(&myMeanData);
    // }
    // shortcut-free: returns Box<Self> because the body registers &myMeanData with the segment (stores `this`)
    #[allow(unused_variables)]
    pub fn new(id: &str, s: Ptr<MESegment>, positionInMeters: f64, name: String, vTypes: &str, nextEdges: &str, detectPersons: i32) -> Box<Self> {
        let base = MSDetectorFileOutput::new(id, vTypes, nextEdges, detectPersons).unwrap_or_else(|e| std::panic::panic_any(e));
        let mut b = Box::new(MEInductLoop {
            base,
            myName: name,
            mySegment: s,
            myPosition: positionInMeters,
            myMeanData: MSMeanData_Net_MSLaneMeanDataValues::new(Ptr::null(), s.get_length(), false, Ptr::null()),
        });
        b.myMeanData.base.base.set_description(&format!("inductionLoop_{}", id));
        s.m().add_detector(Ptr::from_mut(&mut b.myMeanData.base.base), -1);
        b
    }

    /// C++: `void MEInductLoop::~MEInductLoop() noexcept` [virtual]
    // MEInductLoop::~MEInductLoop() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty; the Rust fields are dropped automatically
    }

    /// C++: `void MEInductLoop::writeXMLDetectorProlog(OutputDevice &) const` [const, virtual]
    /// Opens the XML-output using "detector" as root element
    // void writeXMLDetectorProlog(OutputDevice& dev) const {
    //         dev.writeXMLHeader("detector", "det_e1meso_file.xsd");
    //     }
    #[allow(unused_variables)]
    pub fn write_xml_detector_prolog(&self, dev: &mut OutputDevice) {
        dev.write_xml_header("detector", "det_e1meso_file.xsd", BTreeMap::new(), true);
    }

    /// C++: `void MEInductLoop::writeXMLOutput(OutputDevice &, SUMOTime, SUMOTime)` [virtual]
    /// Get the XML-formatted output of all the get*-methods except getTimeSinceLastDetection.
    // void
    // MEInductLoop::writeXMLOutput(OutputDevice& dev,
    //                              SUMOTime startTime, SUMOTime stopTime) {
    //     mySegment->prepareDetectorForWriting(myMeanData);
    //     dev.openTag(SUMO_TAG_INTERVAL).writeAttr(SUMO_ATTR_BEGIN, time2string(startTime)).writeAttr(SUMO_ATTR_END, time2string(stopTime));
    //     dev.writeAttr(SUMO_ATTR_ID, StringUtils::escapeXML(myID)).writeAttr("sampledSeconds", myMeanData.getSamples());
    //     myMeanData.write(dev, 0, stopTime - startTime, (int)mySegment->getEdge().getLanes().size(), mySegment->getEdge().getSpeedLimit(), -1.0);
    //     myMeanData.reset();
    // }
    #[allow(unused_variables)]
    pub fn write_xml_output(&mut self, dev: &mut OutputDevice, startTime: SUMOTime, stopTime: SUMOTime) {
        self.mySegment.m().prepare_detector_for_writing(&mut self.myMeanData.base.base, -1);
        dev.open_tag_id(SUMO_TAG_INTERVAL)
            .write_attr(SUMO_ATTR_BEGIN, &time2string(startTime))
            .write_attr(SUMO_ATTR_END, &time2string(stopTime));
        dev.write_attr(SUMO_ATTR_ID, &string_utils::escape_xml(self.get_id(), false))
            .write_attr("sampledSeconds", &self.myMeanData.get_samples());
        self.myMeanData.write(
            dev,
            &SumoXMLAttrMask(0),
            stopTime - startTime,
            self.mySegment.get_edge().get_lanes().len() as i32,
            self.mySegment.get_edge().get_speed_limit(),
            -1.0,
            -1,
        );
        self.myMeanData.reset(false);
    }

    /// C++: `const MSMeanData_Net::MSLaneMeanDataValues & MEInductLoop::getMeanData() const` [const]
    // const MSMeanData_Net::MSLaneMeanDataValues& getMeanData() const {
    //         return myMeanData;
    //     }
    #[allow(unused_variables)]
    pub fn get_mean_data(&self) -> &MSMeanData_Net_MSLaneMeanDataValues {
        &self.myMeanData
    }

    /// C++: `const MSEdge & MEInductLoop::getEdge() const` [const]
    // const MSEdge&
    // MEInductLoop::getEdge() const {
    //     return mySegment->getEdge();
    // }
    #[allow(unused_variables)]
    pub fn get_edge(&self) -> &MSEdge {
        self.mySegment.get_edge()
    }

    /// C++: `void MEInductLoop::MEInductLoop()` [private]
    /// Hidden default constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2() -> Self {
        panic!("MEInductLoop default constructor is private and undefined in C++; it cannot be called")
    }

    /// C++: `void MEInductLoop::MEInductLoop(const MEInductLoop &)` [private]
    /// Hidden copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_3(arg0: &MEInductLoop) -> Self {
        panic!("MEInductLoop copy constructor is private and undefined in C++; it cannot be called")
    }

    /// C++: `MEInductLoop & MEInductLoop::operator=(const MEInductLoop &)` [private]
    /// Hidden assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MEInductLoop) -> &mut MEInductLoop {
        panic!("MEInductLoop assignment operator is private and undefined in C++; it cannot be called")
    }

}

