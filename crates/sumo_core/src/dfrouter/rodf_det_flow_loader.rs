//! Port of dfrouter/RODFDetFlowLoader.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// A loader for detector flows
/// C++ class `RODFDetFlowLoader` (dfrouter/RODFDetFlowLoader.h)
pub struct RODFDetFlowLoader {
    /// The container for read detector values
    pub myStorage: Ptr<RODFDetectorFlows>,
    /// The time offset to apply to read time values
    pub myTimeOffset: SUMOTime,
    /// The time scale to apply to read time values
    pub myTimeScale: SUMOTime,
    /// The first and the last time step to read
    pub myStartTime: SUMOTime,
    /// The first and the last time step to read
    pub myEndTime: SUMOTime,
    /// The value extractor
    pub myLineHandler: NamedColumnsParser,
    /// Container holding known detectors
    pub myDetectorContainer: Ptr<RODFDetectorCon>,
    /// Whether a warning about overriding boundaries was already written
    pub myHaveWarnedAboutOverridingBoundaries: bool,
    /// Whether a warning about partial definitions was already written
    pub myHaveWarnedAboutPartialDefs: bool,
}

impl RODFDetFlowLoader {
    /// C++: `void RODFDetFlowLoader::RODFDetFlowLoader(const RODFDetectorCon &, RODFDetectorFlows &, SUMOTime, SUMOTime, SUMOTime, SUMOTime)`
    /// Constructor
    // RODFDetFlowLoader::RODFDetFlowLoader(const RODFDetectorCon& dets,
    //                                      RODFDetectorFlows& into,
    //                                      SUMOTime startTime, SUMOTime endTime,
    //                                      SUMOTime timeOffset, SUMOTime timeScale)
    //     : myStorage(into), myTimeOffset(timeOffset), myTimeScale(timeScale),
    //       myStartTime(startTime), myEndTime(endTime), myDetectorContainer(dets),
    //       myHaveWarnedAboutOverridingBoundaries(false), myHaveWarnedAboutPartialDefs(false) {}
    #[allow(unused_variables)]
    pub fn new(dets: &RODFDetectorCon, into: &mut RODFDetectorFlows, startTime: SUMOTime, endTime: SUMOTime, timeOffset: SUMOTime, timeScale: SUMOTime) -> Self {
        todo!("port RODFDetFlowLoader::RODFDetFlowLoader")
    }

    /// C++: `void RODFDetFlowLoader::~RODFDetFlowLoader() noexcept`
    /// Destructor
    // RODFDetFlowLoader::~RODFDetFlowLoader() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port RODFDetFlowLoader::~RODFDetFlowLoader")
    }

    /// C++: `void RODFDetFlowLoader::read(const std::string &)`
    /// Reads the given file assuming it contains detector values
    // void
    // RODFDetFlowLoader::read(const std::string& file) {
    //     LineReader lr(file);
    //     // parse first line
    //     myLineHandler.reinit(lr.readLine(), ";", ";", true, true);
    //     // parse values
    //     while (lr.hasMore()) {
    //         std::string line = lr.readLine();
    //         if (line.find(';') == std::string::npos) {
    //             continue;
    //         }
    //         myLineHandler.parseLine(line);
    //         try {
    //             std::string detName = myLineHandler.get("detector");
    //             if (!myDetectorContainer.knows(detName)) {
    //                 continue;
    //             }
    //             const SUMOTime time = (SUMOTime)(StringUtils::toDouble(myLineHandler.get("time")) * (double)myTimeScale + .5) - myTimeOffset;
    //             // parsing as float to handle values which would cause int overflow
    //             if (time < myStartTime || time >= myEndTime) {
    //                 if (!myHaveWarnedAboutOverridingBoundaries) {
    //                     myHaveWarnedAboutOverridingBoundaries = true;
    //                     WRITE_WARNING(TL("At least one value lies beyond given time boundaries."));
    //                 }
    //                 continue;
    //             }
    //             FlowDef fd;
    //             fd.isLKW = 0;
    //             fd.qPKW = StringUtils::toDouble(myLineHandler.get("qpkw"));
    //             fd.vPKW = 0;
    //             if (myLineHandler.know("vPKW")) {
    //                 fd.vPKW = StringUtils::toDouble(myLineHandler.get("vpkw"));
    //             }
    //             fd.qLKW = 0;
    //             if (myLineHandler.know("qLKW")) {
    //                 fd.qLKW = StringUtils::toDouble(myLineHandler.get("qlkw"));
    //             }
    //             fd.vLKW = 0;
    //             if (myLineHandler.know("vLKW")) {
    //                 fd.vLKW = StringUtils::toDouble(myLineHandler.get("vlkw"));
    //             }
    //             if (fd.qLKW < 0) {
    //                 fd.qLKW = 0;
    //             }
    //             if (fd.qPKW < 0) {
    //                 fd.qPKW = 0;
    //             }
    //             myStorage.addFlow(detName, time, fd);
    //             if (!myHaveWarnedAboutPartialDefs && !myLineHandler.hasFullDefinition()) {
    //                 myHaveWarnedAboutPartialDefs = true;
    //                 WRITE_WARNING(TL("At least one line does not contain the correct number of columns."));
    //             }
    //             continue;
    //         } catch (ProcessError& e) {
    //             throw ProcessError(toString(e.what()) + " in '" + lr.getFileName() + "', line " + toString(lr.getLineNumber()) + ";\n"
    //                                + " The following values must be supplied : 'Detector', 'Time', 'qPKW'\n"
    //                                + " The according column names must be given in the first line of the file.");
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn read(&mut self, file: &str) {
        todo!("port RODFDetFlowLoader::read")
    }

    /// C++: `void RODFDetFlowLoader::RODFDetFlowLoader(const RODFDetFlowLoader &)` [private]
    /// Invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(src: &RODFDetFlowLoader) -> Self {
        {
            Self {
                myStorage: Ptr::null(),
                myTimeOffset: 0,
                myTimeScale: 1,
                myStartTime: 0,
                myEndTime: 0,
                myLineHandler: NamedColumnsParser::new(),
                myDetectorContainer: Ptr::null(),
                myHaveWarnedAboutOverridingBoundaries: false,
                myHaveWarnedAboutPartialDefs: false,
            }
        }
    }

    /// C++: `RODFDetFlowLoader & RODFDetFlowLoader::operator=(const RODFDetFlowLoader &)` [private]
    /// Invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, src: &RODFDetFlowLoader) -> &mut RODFDetFlowLoader {
        todo!("port RODFDetFlowLoader::operator=")
    }

}

