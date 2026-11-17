//! Port of utils/vehicle/SUMORouteLoaderControl.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// SUMORouteLoaderControl This controls is initialised with the list of route loaders and uses them to load routes step wise. The parameter myInAdvanceStepNo holds the number of time steps to read the routes in forward. If it is 0 (default), all routes will be read at once.
/// C++ class `SUMORouteLoaderControl` (utils/vehicle/SUMORouteLoaderControl.h)
pub struct SUMORouteLoaderControl {
    /// the first time step for which vehicles were loaded
    pub myFirstLoadTime: SUMOTime,
    /// the time step up to which vehicles were loaded
    pub myCurrentLoadTime: SUMOTime,
    /// the number of routes to read in forward
    pub myInAdvanceStepNo: SUMOTime,
    /// the list of route loaders
    pub myRouteLoaders: Vec<Ptr<SUMORouteLoader>>,
    /// information whether all routes shall be loaded and whether they were loaded
    pub myLoadAll: bool,
    /// information whether all routes shall be loaded and whether they were loaded
    pub myAllLoaded: bool,
}

impl SUMORouteLoaderControl {
    /// C++: `void SUMORouteLoaderControl::SUMORouteLoaderControl(SUMOTime)`
    /// constructor
    // SUMORouteLoaderControl::SUMORouteLoaderControl(SUMOTime inAdvanceStepNo):
    //     myFirstLoadTime(SUMOTime_MAX),
    //     myCurrentLoadTime(-SUMOTime_MAX),
    //     myInAdvanceStepNo(inAdvanceStepNo),
    //     myRouteLoaders(),
    //     myLoadAll(inAdvanceStepNo <= 0),
    //     myAllLoaded(false) {
    // }
    #[allow(unused_variables)]
    pub fn new(inAdvanceStepNo: SUMOTime) -> Self {
        Self {
            myFirstLoadTime: SUMOTIME_MAX,
            myCurrentLoadTime: -SUMOTIME_MAX,
            myInAdvanceStepNo: inAdvanceStepNo,
            myRouteLoaders: Vec::new(),
            myLoadAll: inAdvanceStepNo <= 0,
            myAllLoaded: false,
        }
    }

    /// C++: `void SUMORouteLoaderControl::~SUMORouteLoaderControl()`
    /// destructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // destructor: nothing to release explicitly, the Vec fields drop with the struct
    }

    /// C++: `void SUMORouteLoaderControl::add(SUMORouteLoader *)`
    /// add another loader
    // void
    // SUMORouteLoaderControl::add(SUMORouteLoader* loader) {
    //     myRouteLoaders.push_back(loader);
    // }
    #[allow(unused_variables)]
    pub fn add(&mut self, loader: Ptr<SUMORouteLoader>) {
        self.myRouteLoaders.push(loader);
    }

    /// C++: `void SUMORouteLoaderControl::loadNext(SUMOTime)`
    /// loads the next routes up to and including the given time step
    // void
    // SUMORouteLoaderControl::loadNext(SUMOTime step) {
    //     // check whether new vehicles shall be loaded
    //     //  return if not
    //     if (myAllLoaded) {
    //         return;
    //     }
    //     if (myCurrentLoadTime > step) {
    //         return;
    //     }
    //     const SUMOTime loadMaxTime = myLoadAll ? SUMOTime_MAX : MAX2(myCurrentLoadTime + myInAdvanceStepNo, step);
    //     myCurrentLoadTime = SUMOTime_MAX;
    //     // load all routes for the specified time period
    //     bool furtherAvailable = false;
    //     for (SUMORouteLoader* loader : myRouteLoaders) {
    //         myCurrentLoadTime = MIN2(myCurrentLoadTime, loader->loadUntil(loadMaxTime));
    //         if (loader->getFirstDepart() != -1) {
    //             myFirstLoadTime = MIN2(myFirstLoadTime, loader->getFirstDepart());
    //         }
    //         furtherAvailable |= loader->moreAvailable();
    //     }
    //     if (myFirstLoadTime == SUMOTime_MAX) {
    //         myFirstLoadTime = 0;
    //     }
    //     myAllLoaded = !furtherAvailable;
    // }
    #[allow(unused_variables)]
    pub fn load_next(&mut self, step: SUMOTime) {
        // check whether new vehicles shall be loaded
        //  return if not
        if self.myAllLoaded {
            return;
        }
        if self.myCurrentLoadTime > step {
            return;
        }
        let loadMaxTime = if self.myLoadAll { SUMOTIME_MAX } else { std::cmp::max(self.myCurrentLoadTime + self.myInAdvanceStepNo, step) };
        self.myCurrentLoadTime = SUMOTIME_MAX;
        // load all routes for the specified time period
        let mut furtherAvailable = false;
        for &loader in self.myRouteLoaders.iter() {
            self.myCurrentLoadTime = std::cmp::min(self.myCurrentLoadTime, loader.m().load_until(loadMaxTime));
            if loader.get_first_depart() != -1 {
                self.myFirstLoadTime = std::cmp::min(self.myFirstLoadTime, loader.get_first_depart());
            }
            furtherAvailable |= loader.more_available();
        }
        if self.myFirstLoadTime == SUMOTIME_MAX {
            self.myFirstLoadTime = 0;
        }
        self.myAllLoaded = !furtherAvailable;
    }

    /// C++: `SUMOTime SUMORouteLoaderControl::getFirstLoadTime() const` [const]
    /// returns the timestamp of the first loaded vehicle or flow
    // SUMOTime getFirstLoadTime() const {
    //         return myFirstLoadTime;
    //     }
    #[allow(unused_variables)]
    pub fn get_first_load_time(&self) -> SUMOTime {
        self.myFirstLoadTime
    }

    /// C++: `bool SUMORouteLoaderControl::haveAllLoaded() const` [const]
    /// returns whether loading is completed
    // bool haveAllLoaded() const {
    //         return myAllLoaded;
    //     }
    #[allow(unused_variables)]
    pub fn have_all_loaded(&self) -> bool {
        self.myAllLoaded
    }

    /// C++: `SUMORouteLoader * SUMORouteLoaderControl::getFirstLoader() const` [const]
    /// return a route loader
    // SUMORouteLoader*
    // SUMORouteLoaderControl::getFirstLoader() const {
    //     if (myRouteLoaders.size() > 0) {
    //         return myRouteLoaders.front();
    //     }
    //     return nullptr;
    // }
    #[allow(unused_variables)]
    pub fn get_first_loader(&self) -> Ptr<SUMORouteLoader> {
        if self.myRouteLoaders.len() > 0 {
            return self.myRouteLoaders[0];
        }
        Ptr::null()
    }

    /// C++: `SUMOTime SUMORouteLoaderControl::getCurrentLoadTime() const` [const]
    // SUMOTime getCurrentLoadTime() const {
    //         return myCurrentLoadTime;
    //     }
    #[allow(unused_variables)]
    pub fn get_current_load_time(&self) -> SUMOTime {
        self.myCurrentLoadTime
    }

    /// C++: `void SUMORouteLoaderControl::setCurrentLoadTime(SUMOTime)`
    // void setCurrentLoadTime(SUMOTime time) {
    //         myCurrentLoadTime = time;
    //     }
    #[allow(unused_variables)]
    pub fn set_current_load_time(&mut self, time: SUMOTime) {
        self.myCurrentLoadTime = time;
    }

    /// C++: `void SUMORouteLoaderControl::SUMORouteLoaderControl(const SUMORouteLoaderControl &)` [private]
    /// Invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(src: &SUMORouteLoaderControl) -> Self {
        panic!("SUMORouteLoaderControl copy constructor is invalidated")
    }

    /// C++: `SUMORouteLoaderControl & SUMORouteLoaderControl::operator=(const SUMORouteLoaderControl &)` [private]
    /// Invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, src: &SUMORouteLoaderControl) -> &mut SUMORouteLoaderControl {
        panic!("SUMORouteLoaderControl assignment operator is invalidated")
    }

}

