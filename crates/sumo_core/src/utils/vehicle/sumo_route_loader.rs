//! Port of utils/vehicle/SUMORouteLoader.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `SUMORouteLoader` (utils/vehicle/SUMORouteLoader.h)
pub struct SUMORouteLoader {
    /// the used SAXReader
    pub myParser: Ptr<SUMOSAXReader>,
    /// flag with information whether more vehicles should be available
    pub myMoreAvailable: bool,
    /// the used Handler
    pub myHandler: Ptr<SUMORouteHandler>,
}

impl SUMORouteLoader {
    /// C++: `void SUMORouteLoader::SUMORouteLoader(SUMORouteHandler *)`
    /// constructor
    // SUMORouteLoader::SUMORouteLoader(SUMORouteHandler* handler)
    //     : myParser(nullptr), myMoreAvailable(true), myHandler(handler) {
    //     myParser = XMLSubSys::getSAXReader(*myHandler, false, true);
    //     if (!myParser->parseFirst(myHandler->getFileName())) {
    //         throw ProcessError(TLF("Can not read XML-file '%'.", myHandler->getFileName()));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn new(handler: Ptr<SUMORouteHandler>) -> Self {
        // shortcut: parseFirst(handler) needs SUMORouteHandler as &mut dyn SaxHandler, but it has no SaxHandler impl yet (base is Unmapped SUMOSAXHandler), upgrade when sumo_route_handler.rs is ported
        let _ = handler;
        panic!("SUMORouteLoader: SUMORouteHandler does not implement SaxHandler yet")
    }

    /// C++: `void SUMORouteLoader::~SUMORouteLoader()`
    /// destructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // implicit C++ destructor: members are raw pointers and a bool, nothing to free
    }

    /// C++: `SUMOTime SUMORouteLoader::loadUntil(SUMOTime)`
    /// loads vehicles until a vehicle is read that starts after the specified time
    // SUMOTime
    // SUMORouteLoader::loadUntil(SUMOTime time) {
    //     // read only when further data is available, no error occurred
    //     //  and vehicles may be found in the between the departure time of
    //     //  the last read vehicle and the time to read until
    //     if (!myMoreAvailable) {
    //         return SUMOTime_MAX;
    //     }
    //     // read vehicles until specified time or the period to read vehicles
    //     //  until is reached
    //     while (myHandler->getLastDepart() <= time) {
    //         if (!myParser->parseNext()) {
    //             // no data available anymore
    //             myMoreAvailable = false;
    //             return SUMOTime_MAX;
    //         }
    //     }
    //     return myHandler->getLastDepart();
    // }
    #[allow(unused_variables)]
    pub fn load_until(&mut self, time: SUMOTime) -> SUMOTime {
        if !self.myMoreAvailable {
            return SUMOTIME_MAX;
        }
        while self.myHandler.get_last_depart() <= time {
            // shortcut: parseNext(myHandler) needs SUMORouteHandler as &mut dyn SaxHandler, not implemented yet, upgrade when sumo_route_handler.rs is ported
            panic!("SUMORouteLoader::loadUntil: SUMORouteHandler does not implement SaxHandler yet");
        }
        self.myHandler.get_last_depart()
    }

    /// C++: `bool SUMORouteLoader::moreAvailable() const` [const]
    /// returns the information whether new data is available
    // bool
    // SUMORouteLoader::moreAvailable() const {
    //     return myMoreAvailable;
    // }
    #[allow(unused_variables)]
    pub fn more_available(&self) -> bool {
        self.myMoreAvailable
    }

    /// C++: `SUMOTime SUMORouteLoader::getFirstDepart() const` [const]
    /// returns the first departure time that was ever read
    // SUMOTime
    // SUMORouteLoader::getFirstDepart() const {
    //     return myHandler->getFirstDepart();
    // }
    #[allow(unused_variables)]
    pub fn get_first_depart(&self) -> SUMOTime {
        self.myHandler.get_first_depart()
    }

    /// C++: `SUMORouteHandler * SUMORouteLoader::getRouteHandler()`
    // SUMORouteHandler* getRouteHandler() {
    //         return myHandler;
    //     }
    #[allow(unused_variables)]
    pub fn get_route_handler(&mut self) -> Ptr<SUMORouteHandler> {
        self.myHandler
    }

}

