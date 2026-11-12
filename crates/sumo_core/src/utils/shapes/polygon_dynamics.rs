//! Port of utils/shapes/PolygonDynamics.{h,cpp}
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `PolygonDynamics` (utils/shapes/PolygonDynamics.h)
pub struct PolygonDynamics {
    /// The polygon this dynamics acts upon.
    pub myPolygon: Ptr<SUMOPolygon>,
    /// Current time
    pub myCurrentTime: f64,
    /// The last time the animation has been updated
    pub myLastUpdateTime: f64,
    /// Whether this polygon is animated, i.e., whether timelines should be used to control properties.
    pub animated: bool,
    /// Whether animation should be looped.
    pub looped: bool,
    /// Whether this polygon tracks an object
    pub tracking: bool,
    /// Whether this polygon should be rotated with the tracked object
    pub rotate: bool,
    /// An object tracked by the shape, deletion by caller
    pub myTrackedObject: Ptr<SUMOTrafficObject>,
    pub myTrackedObjectID: String,
    /// Initial position of the tracked object
    pub myTrackedObjectsInitialPositon: Option<Box<Position>>, // None == nullptr
    /// Initial angle of the tracked object
    pub myTrackedObjectsInitialAngle: f64,
    /// the original shape of the polygon (in case of tracking another object, this is converted to relative coords wrt to the other objects initial position as origin)
    pub myOriginalShape: Box<PositionVector>,
    /// Time points corresponding to the anchor values of the dynamic properties
    pub myTimeSpan: Box<Vec<f64>>,
    /// Pointer to the next time points in timeSpan
    pub myPrevTime: usize, // index into myTimeSpan (C++ iterator)
    pub myNextTime: usize, // index into myTimeSpan (C++ iterator)
    /// Alpha values corresponding to
    pub myAlphaSpan: Box<Vec<f64>>,
    /// Pointer to the next alpha points in alphaSpan
    pub myPrevAlpha: usize, // index into myAlphaSpan (C++ iterator)
    pub myNextAlpha: usize, // index into myAlphaSpan (C++ iterator)
    /// RTree will be supplied in case of GUI simulation to be updated on move
    pub myVis: Ptr<Unmapped /* SUMORTree */>,
}

impl PolygonDynamics {
    /// C++: `void PolygonDynamics::PolygonDynamics(double, SUMOPolygon *, SUMOTrafficObject *, const std::vector<double> &, const std::vector<double> &, bool, bool)`
    /// Constructor that takes a SUMOPolygon and adds timelines for the properties to be modified dynamically.
    // PolygonDynamics::PolygonDynamics(double creationTime,
    //                                  SUMOPolygon* p,
    //                                  SUMOTrafficObject* trackedObject,
    //                                  const std::vector<double>& timeSpan,
    //                                  const std::vector<double>& alphaSpan,
    //                                  bool looped,
    //                                  bool rotate) :
    //     myPolygon(p),
    //     myCurrentTime(0),
    //     myLastUpdateTime(creationTime),
    //     animated(!timeSpan.empty()),
    //     looped(looped),
    //     tracking(trackedObject != nullptr),
    //     rotate(rotate),
    //     myTrackedObject(trackedObject),
    //     myTrackedObjectID(""),
    //     myTrackedObjectsInitialPositon(nullptr),
    //     myTrackedObjectsInitialAngle(-1),
    //     myOriginalShape(nullptr),
    //     myTimeSpan(nullptr),
    //     myAlphaSpan(nullptr),
    //     myVis(nullptr) {
    //     // Check for consistency
    //     if (animated) {
    //         myTimeSpan = std::unique_ptr<std::vector<double> >(new std::vector<double>(timeSpan));
    //         assert(myTimeSpan->size() >= 2);
    //         assert((*myTimeSpan)[0] == 0.0);
    //         assert(myAlphaSpan == nullptr || myAlphaSpan->size() >= 2);
    // #ifdef DEBUG_DYNAMIC_SHAPES
    //         if (myTimeSpan->size() >= 2) {
    //             for (unsigned int i = 1; i < myTimeSpan->size(); ++i) {
    //                 assert((*myTimeSpan)[i - 1] <= (*myTimeSpan)[i]);
    //             }
    //         }
    // #endif
    //         myPrevTime = myTimeSpan->begin();
    //         myNextTime = ++myTimeSpan->begin();
    //     }
    // #ifdef DEBUG_DYNAMIC_SHAPES
    //     else  {
    //         assert(myAlphaSpan == nullptr);
    //     }
    // #endif
    //
    //     myOriginalShape = std::unique_ptr<PositionVector>(new PositionVector(p->getShape()));
    //
    //     if (tracking) {
    //         // Try initializing the tracked position (depends on whether object is already on the road)
    //         initTrackedPosition();
    //         myTrackedObjectID = myTrackedObject->getID();
    //     }
    //
    //     if (!alphaSpan.empty()) {
    //         myAlphaSpan = std::unique_ptr<std::vector<double> >(new std::vector<double>(alphaSpan));
    //         assert(myAlphaSpan->size() >= 2);
    //         assert(myAlphaSpan->size() == myTimeSpan->size());
    //         myPrevAlpha = myAlphaSpan->begin();
    //         myNextAlpha = ++myAlphaSpan->begin();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn new(creationTime: f64, p: Ptr<SUMOPolygon>, trackedObject: Ptr<SUMOTrafficObject>, timeSpan: &Vec<f64>, alphaSpan: &Vec<f64>, looped: bool, rotate: bool) -> Self {
        let animated = !timeSpan.is_empty();
        let tracking = !trackedObject.is_null();
        let mut this = PolygonDynamics {
            myPolygon: p.clone(),
            myCurrentTime: 0.0,
            myLastUpdateTime: creationTime,
            animated,
            looped,
            tracking,
            rotate,
            myTrackedObject: trackedObject.clone(),
            myTrackedObjectID: String::new(),
            myTrackedObjectsInitialPositon: None,
            myTrackedObjectsInitialAngle: -1.0,
            myOriginalShape: Box::new(PositionVector::default()),
            myTimeSpan: Box::new(Vec::new()),
            myPrevTime: 0,
            myNextTime: 0,
            myAlphaSpan: Box::new(Vec::new()),
            myPrevAlpha: 0,
            myNextAlpha: 0,
            myVis: Ptr::null(),
        };
        // Check for consistency (C++ asserts; debug only)
        if animated {
            this.myTimeSpan = Box::new(timeSpan.clone());
            debug_assert!(this.myTimeSpan.len() >= 2);
            debug_assert!(this.myTimeSpan[0] == 0.0);
            debug_assert!(this.myAlphaSpan.is_empty() || this.myAlphaSpan.len() >= 2);
            this.myPrevTime = 0;
            this.myNextTime = 1;
        }
        this.myOriginalShape = Box::new(p.get_shape().clone());
        if this.tracking {
            // Try initializing the tracked position (depends on whether object is already on the road)
            this.init_tracked_position();
            this.myTrackedObjectID = this.myTrackedObject.get_id().to_string();
        }
        if !alphaSpan.is_empty() {
            this.myAlphaSpan = Box::new(alphaSpan.clone());
            debug_assert!(this.myAlphaSpan.len() >= 2);
            debug_assert!(this.myAlphaSpan.len() == this.myTimeSpan.len());
            this.myPrevAlpha = 0;
            this.myNextAlpha = 1;
        }
        this
    }

    /// C++: `void PolygonDynamics::~PolygonDynamics()` [virtual]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
    }

    /// C++: `const std::string & PolygonDynamics::getPolygonID() const` [const]
    // const std::string& getPolygonID() const {
    //         return myPolygon->getID();
    //     }
    #[allow(unused_variables)]
    pub fn get_polygon_id(&self) -> &str {
        self.myPolygon.get_id()
    }

    /// C++: `SUMOPolygon * PolygonDynamics::getPolygon() const` [const]
    // SUMOPolygon* getPolygon() const {
    //         return myPolygon;
    //     }
    #[allow(unused_variables)]
    pub fn get_polygon(&self) -> Ptr<SUMOPolygon> {
        self.myPolygon.clone()
    }

    /// C++: `const std::string & PolygonDynamics::getTrackedObjectID() const` [const]
    // inline const std::string& getTrackedObjectID() const {
    //         return myTrackedObjectID;
    //     }
    #[allow(unused_variables)]
    pub fn get_tracked_object_id(&self) -> &str {
        &self.myTrackedObjectID
    }

    /// C++: `SUMOTime PolygonDynamics::update(SUMOTime)`
    /// Updates the polygon according to its timeSpan and follows the tracked object
    // SUMOTime
    // PolygonDynamics::update(SUMOTime t) {
    // #ifdef DEBUG_DYNAMIC_SHAPES
    //     std::cout << t << " PolygonDynamics::update() for polygon '" << myPolygon->getID() << "'" << std::endl;
    // #endif
    //     const double simtime = STEPS2TIME(t);
    //     const double dt = simtime - myLastUpdateTime;
    //     myLastUpdateTime = simtime;
    //
    //     SUMOTime ret = utils::common::time::delta_t();
    //
    //     if (tracking) {
    //         if (myTrackedObjectsInitialPositon == nullptr) {
    //             // Tracked object hasn't entered the network, until now.
    //             // Continuously try to obtain its initial position
    //             initTrackedPosition();
    //         }
    //         if (myTrackedObjectsInitialPositon != nullptr) {
    //             // Initial position was initialized, relative tracking is possible
    //             const Position& objPos = myTrackedObject->getPosition();
    //             const bool onRoad = objPos != Position::INVALID;
    //             if (onRoad) {
    // #ifdef DEBUG_DYNAMIC_SHAPES
    //                 std::cout << " Tracked object '" << myTrackedObject->getID() << "' is on the road. Tracked position=" << objPos << std::endl;
    // #endif
    //                 // Update polygon's shape
    //                 PositionVector newShape(*myOriginalShape);
    //                 if (rotate) {
    //                     const double relRotation = myTrackedObject->getAngle() - myTrackedObjectsInitialAngle;
    //                     newShape.rotate2D(relRotation);
    // #ifdef DEBUG_DYNAMIC_SHAPES
    //                     std::cout << " Relative rotation wrt original rotation: " << relRotation << std::endl;
    // #endif
    //                 }
    //                 newShape.add(objPos);
    //                 myPolygon->setShape(newShape);
    //             }
    // #ifdef DEBUG_DYNAMIC_SHAPES
    //             else {
    //                 // tracked object is off road
    //                 std::cout << " Tracked object '" << myTrackedObject->getID() << "' is off road." << std::endl;
    //             }
    // #endif
    //         }
    // #ifdef DEBUG_DYNAMIC_SHAPES
    //         else {
    //             // Initial position was not initialized, yet
    //             std::cout << " Tracked object '" << myTrackedObject->getID() << "' hasn't entered the network since tracking was started." << std::endl;
    //         }
    // #endif
    //     }
    //
    //     if (animated) {
    //         // Continue animation
    //         myCurrentTime += dt;
    //         while (myCurrentTime >= *myNextTime) {
    //             // step forward along time lines to appropriate anchor points
    //             ++myPrevTime;
    //             ++myNextTime;
    //             if (myNextTime == myTimeSpan->end()) {
    //                 // Set iterators back to point to valid positions
    //                 --myPrevTime;
    //                 --myNextTime;
    //                 break;
    //             } else {
    //                 // Forward corresponding iterators for property time lines
    //                 if (myAlphaSpan != nullptr) {
    //                     ++myPrevAlpha;
    //                     ++myNextAlpha;
    //                 }
    //             }
    //         }
    //
    //         // Linear interpolation factor between previous and next time
    //         double theta = 1.0;
    // #ifdef DEBUG_DYNAMIC_SHAPES
    //         std::cout << " animation: dt=" << dt
    //                   << ", current animation time: " << myCurrentTime
    //                   << ", previous anchor time: " << *myPrevTime
    //                   << ", next anchor time: " << *myNextTime;
    // #endif
    //         if (looped) {
    //             const bool resetAnimation = myCurrentTime >= *myNextTime;
    // #ifdef DEBUG_DYNAMIC_SHAPES
    //             if (resetAnimation) {
    //                 std::cout << " (resetting animation!)";
    //             }
    // #endif
    //             if (resetAnimation) {
    //                 // Reset animation time line to start, if appropriate
    //                 while (myCurrentTime >= *myNextTime) {
    //                     myCurrentTime -= *myNextTime;
    //                 }
    //                 myCurrentTime = MAX2(myCurrentTime, 0.0);
    //                 myPrevTime = myTimeSpan->begin();
    //                 myNextTime = ++myTimeSpan->begin();
    //                 if (myAlphaSpan != nullptr) {
    //                     myPrevAlpha = myAlphaSpan->begin();
    //                     myNextAlpha = ++myAlphaSpan->begin();
    //                 }
    //             }
    //         }
    //         if (myCurrentTime >= *myNextTime) {
    //             assert(!looped);
    //             // Reached the end of the dynamics, indicate expiration by returning zero
    //             // and set all properties to the final state (theta remains one)
    //             ret = 0;
    // #ifdef DEBUG_DYNAMIC_SHAPES
    //             std::cout << " (animation elapsed!)";
    // #endif
    //         } else {
    //             // Animation is still going on, schedule next update
    //             if (*myNextTime - *myPrevTime != 0) {
    //                 theta = (myCurrentTime - *myPrevTime) / (*myNextTime - *myPrevTime);
    //             }
    //         }
    //         if (myAlphaSpan != nullptr) {
    //             // Interpolate values of properties
    //             setAlpha(*myPrevAlpha + theta * (*myNextAlpha - *myPrevAlpha));
    // #ifdef DEBUG_DYNAMIC_SHAPES
    //             std::cout << ", previous anchor alpha: " << *myPrevAlpha
    //                       << ", next anchor alpha: " << *myNextAlpha;
    // #endif
    //         }
    // #ifdef DEBUG_DYNAMIC_SHAPES
    //         std::cout << ", theta=" << theta << std::endl;
    // #endif
    //     }
    //     return ret;
    // }
    #[allow(unused_variables)]
    pub fn update(&mut self, t: SUMOTime) -> SUMOTime {
        let simtime = steps2time(t);
        let dt = simtime - self.myLastUpdateTime;
        self.myLastUpdateTime = simtime;

        let mut ret: SUMOTime = utils::common::time::delta_t();

        if self.tracking {
            if self.myTrackedObjectsInitialPositon.is_none() {
                // Tracked object hasn't entered the network, until now.
                // Continuously try to obtain its initial position
                self.init_tracked_position();
            }
            if self.myTrackedObjectsInitialPositon.is_some() {
                // Initial position was initialized, relative tracking is possible
                let objPos: Position = self.myTrackedObject.get_position(0.0).clone();
                let onRoad = objPos != Position::INVALID;
                if onRoad {
                    // Update polygon's shape
                    let mut newShape: PositionVector = (*self.myOriginalShape).clone();
                    if self.rotate {
                        let relRotation = self.myTrackedObject.get_angle() - self.myTrackedObjectsInitialAngle;
                        newShape.rotate2d(relRotation);
                    }
                    newShape.add(&objPos);
                    self.myPolygon.m().set_shape(&newShape);
                }
            }
        }

        if self.animated {
            // Continue animation
            self.myCurrentTime += dt;
            while self.myCurrentTime >= self.myTimeSpan[self.myNextTime] {
                // step forward along time lines to appropriate anchor points
                self.myPrevTime += 1;
                self.myNextTime += 1;
                if self.myNextTime == self.myTimeSpan.len() {
                    // Set indices back to point to valid positions
                    self.myPrevTime -= 1;
                    self.myNextTime -= 1;
                    break;
                } else {
                    // Forward corresponding indices for property time lines
                    if !self.myAlphaSpan.is_empty() {
                        self.myPrevAlpha += 1;
                        self.myNextAlpha += 1;
                    }
                }
            }

            // Linear interpolation factor between previous and next time
            let mut theta = 1.0;
            if self.looped {
                let resetAnimation = self.myCurrentTime >= self.myTimeSpan[self.myNextTime];
                if resetAnimation {
                    // Reset animation time line to start, if appropriate
                    while self.myCurrentTime >= self.myTimeSpan[self.myNextTime] {
                        self.myCurrentTime -= self.myTimeSpan[self.myNextTime];
                    }
                    // MAX2(myCurrentTime, 0.0)
                    self.myCurrentTime = if self.myCurrentTime > 0.0 { self.myCurrentTime } else { 0.0 };
                    self.myPrevTime = 0;
                    self.myNextTime = 1;
                    if !self.myAlphaSpan.is_empty() {
                        self.myPrevAlpha = 0;
                        self.myNextAlpha = 1;
                    }
                }
            }
            if self.myCurrentTime >= self.myTimeSpan[self.myNextTime] {
                debug_assert!(!self.looped);
                // Reached the end of the dynamics, indicate expiration by returning zero
                // and set all properties to the final state (theta remains one)
                ret = 0;
            } else {
                // Animation is still going on, schedule next update
                if self.myTimeSpan[self.myNextTime] - self.myTimeSpan[self.myPrevTime] != 0.0 {
                    theta = (self.myCurrentTime - self.myTimeSpan[self.myPrevTime]) / (self.myTimeSpan[self.myNextTime] - self.myTimeSpan[self.myPrevTime]);
                }
            }
            if !self.myAlphaSpan.is_empty() {
                // Interpolate values of properties
                let alpha = self.myAlphaSpan[self.myPrevAlpha] + theta * (self.myAlphaSpan[self.myNextAlpha] - self.myAlphaSpan[self.myPrevAlpha]);
                self.set_alpha(alpha);
            }
        }
        ret
    }

    /// C++: `void PolygonDynamics::setRTree(SUMORTree *)`
    /// Set the RTree
    // void setRTree(SUMORTree* rtree) {
    //         myVis = rtree;
    //     }
    #[allow(unused_variables)]
    pub fn set_r_tree(&mut self, rtree: Ptr<Unmapped /* SUMORTree */>) {
        self.myVis = rtree;
    }

    /// C++: `void PolygonDynamics::setAlpha(double)` [private]
    /// Sets the alpha value for the shape's color
    // void
    // PolygonDynamics::setAlpha(double alpha) {
    //     int a = (int) alpha;
    //     myPolygon->setShapeAlpha((unsigned char) a);
    // #ifdef DEBUG_DYNAMIC_SHAPES
    //     std::cout << "\n   DynamicPolygon::setAlpha() Converted alpha=" << alpha << " into myAlpha=" << a << std::endl;
    // #endif
    // }
    #[allow(unused_variables)]
    pub fn set_alpha(&mut self, alpha: f64) {
        let a = alpha as i32; // (int) cast
        self.myPolygon.m().set_shape_alpha(a as u8); // (unsigned char) cast
    }

    /// C++: `void PolygonDynamics::initTrackedPosition()` [private]
    /// Initialize the object's position
    // void
    // PolygonDynamics::initTrackedPosition() {
    //     const Position& objPos = myTrackedObject->getPosition();
    //     if (objPos != Position::INVALID) {
    //         // Initialize Position of tracked object
    //         myTrackedObjectsInitialPositon = std::unique_ptr<Position>(new Position(objPos));
    //         myTrackedObjectsInitialAngle = myTrackedObject->getAngle();
    //         // Store original polygon shape relative to the tracked object's original position
    //         myOriginalShape->sub(*myTrackedObjectsInitialPositon);
    // #ifdef DEBUG_DYNAMIC_SHAPES
    //         std::cout << " Tracking object '" << myTrackedObject->getID() << "' at initial positon: " << *myTrackedObjectsInitialPositon << std::endl;
    // #endif
    //     }
    // }
    #[allow(unused_variables)]
    pub fn init_tracked_position(&mut self) {
        let objPos: Position = self.myTrackedObject.get_position(0.0).clone();
        if objPos != Position::INVALID {
            // Initialize Position of tracked object
            self.myTrackedObjectsInitialPositon = Some(Box::new(objPos.clone()));
            self.myTrackedObjectsInitialAngle = self.myTrackedObject.get_angle();
            // Store original polygon shape relative to the tracked object's original position
            self.myOriginalShape.sub(&objPos);
        }
    }

}

