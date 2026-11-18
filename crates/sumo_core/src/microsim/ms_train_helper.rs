//! Port of microsim/MSTrainHelper.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::std_defs::INVALID_DOUBLE;

// C++ MSTrainHelper.cpp file-local/class statics; they stay in this module as in the C++ file
const PEDESTRIAN_RADIUS_EXTRA_TOLERANCE: f64 = 0.01;
const MIN_SCALED_CARRIAGE_LENGTH: f64 = 5.0;

/// C++ class `MSTrainHelper::Carriage` (microsim/MSTrainHelper.h)
pub struct MSTrainHelper_Carriage {
    pub front: Position,
    pub back: Position,
    pub doorPositions: Vec<Position>,
    pub unboardingPositions: Vec<Position>,
}

impl MSTrainHelper_Carriage {
}

/// A class that helps computing positions of a train's carriages and additional structures.
/// C++ class `MSTrainHelper` (microsim/MSTrainHelper.h)
pub struct MSTrainHelper {
    pub myTrain: Ptr<MSVehicle>,
    pub myUpscaleLength: f64,
    pub myLocomotiveLength: f64,
    pub myDefaultLength: f64,
    pub myCarriageGap: f64,
    pub myLength: f64,
    pub myUnscale: bool,
    pub myHalfWidth: f64,
    pub myNumCarriages: i32,
    pub myCarriageLengthWithGap: f64,
    pub myCarriageLength: f64,
    pub myFirstCarriageLength: f64,
    pub myCarriageDoors: i32,
    pub myFirstCarriageNo: i32,
    pub myFirstPassengerCarriage: i32,
    pub myIsReversed: bool,
    pub myCarriages: Vec<Ptr<MSTrainHelper_Carriage>>,
}

// C++ static members of MSTrainHelper (port as module-level statics, see AGENTS.md):
//   static const double MSTrainHelper::PEDESTRIAN_RADIUS_EXTRA_TOLERANCE;

impl MSTrainHelper {
    /// C++: `void MSTrainHelper::MSTrainHelper(const MSVehicle *, double, bool, bool, double, int)`
    // MSTrainHelper(const MSVehicle* vehicle, double scaledLength = -1, bool reversed = false,
    //                   bool secondaryShape = false, double exaggeration = 1.0, int vehicleQuality = 3)
    //         : myTrain(vehicle) {
    //         computeTrainDimensions(exaggeration, secondaryShape, scaledLength < 0 ? myTrain->getLength() : scaledLength, vehicleQuality);
    //         computeCarriages(reversed, secondaryShape);
    //     }
    #[allow(unused_variables)]
    pub fn new(vehicle: Ptr<MSVehicle>, scaledLength: f64, reversed: bool, secondaryShape: bool, exaggeration: f64, vehicleQuality: i32) -> Self {
        let mut th = MSTrainHelper {
            myTrain: vehicle,
            myUpscaleLength: 0.0,
            myLocomotiveLength: 0.0,
            myDefaultLength: 0.0,
            myCarriageGap: 0.0,
            myLength: 0.0,
            myUnscale: false,
            myHalfWidth: 0.0,
            myNumCarriages: 0,
            myCarriageLengthWithGap: 0.0,
            myCarriageLength: 0.0,
            myFirstCarriageLength: 0.0,
            myCarriageDoors: 0,
            myFirstCarriageNo: 0,
            myFirstPassengerCarriage: 0,
            myIsReversed: false,
            myCarriages: Vec::new(),
        };
        th.compute_train_dimensions(
            exaggeration,
            secondaryShape,
            if scaledLength < 0.0 { vehicle.get_length() } else { scaledLength },
            vehicleQuality,
        );
        th.compute_carriages(reversed, secondaryShape);
        th
    }

    /// C++: `void MSTrainHelper::~MSTrainHelper() noexcept`
    // ~MSTrainHelper() {
    //         for (const Carriage* carriage : myCarriages) {
    //             delete carriage;
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        for carriage in self.myCarriages.drain(..) {
            // delete carriage
            unsafe { drop(Box::from_raw(carriage.as_raw())) };
        }
    }

    /// C++: `double MSTrainHelper::getUpscaleLength() const` [const]
    // inline double getUpscaleLength() const {
    //         return myUpscaleLength;
    //     }
    #[allow(unused_variables)]
    pub fn get_upscale_length(&self) -> f64 {
        self.myUpscaleLength
    }

    /// C++: `double MSTrainHelper::getHalfWidth() const` [const]
    // inline double getHalfWidth() const {
    //         return myHalfWidth;
    //     }
    #[allow(unused_variables)]
    pub fn get_half_width(&self) -> f64 {
        self.myHalfWidth
    }

    /// C++: `int MSTrainHelper::getNumCarriages() const` [const]
    // inline int getNumCarriages() const {
    //         return myNumCarriages;
    //     }
    #[allow(unused_variables)]
    pub fn get_num_carriages(&self) -> i32 {
        self.myNumCarriages
    }

    /// C++: `double MSTrainHelper::getCarriageLength() const` [const]
    // inline double getCarriageLength() const {
    //         return myCarriageLength;
    //     }
    #[allow(unused_variables)]
    pub fn get_carriage_length(&self) -> f64 {
        self.myCarriageLength
    }

    /// C++: `double MSTrainHelper::getFirstCarriageLength() const` [const]
    // inline double getFirstCarriageLength() const {
    //         return myFirstCarriageLength;
    //     }
    #[allow(unused_variables)]
    pub fn get_first_carriage_length(&self) -> f64 {
        self.myFirstCarriageLength
    }

    /// C++: `int MSTrainHelper::getCarriageDoors() const` [const]
    // inline int getCarriageDoors() const {
    //         return myCarriageDoors;
    //     }
    #[allow(unused_variables)]
    pub fn get_carriage_doors(&self) -> i32 {
        self.myCarriageDoors
    }

    /// C++: `int MSTrainHelper::getFirstCarriageNo() const` [const]
    // inline int getFirstCarriageNo() const {
    //         return myFirstCarriageNo;
    //     }
    #[allow(unused_variables)]
    pub fn get_first_carriage_no(&self) -> i32 {
        self.myFirstCarriageNo
    }

    /// C++: `int MSTrainHelper::getFirstPassengerCarriage() const` [const]
    // inline int getFirstPassengerCarriage() const {
    //         return myFirstPassengerCarriage;
    //     }
    #[allow(unused_variables)]
    pub fn get_first_passenger_carriage(&self) -> i32 {
        self.myFirstPassengerCarriage
    }

    /// C++: `bool MSTrainHelper::isReversed() const` [const]
    // inline bool isReversed() const {
    //         return myIsReversed;
    //     }
    #[allow(unused_variables)]
    pub fn is_reversed(&self) -> bool {
        self.myIsReversed
    }

    /// C++: `const std::vector<Carriage *> & MSTrainHelper::getCarriages() const` [const]
    // inline const std::vector<Carriage*>& getCarriages() const {
    //         return myCarriages;
    //     }
    #[allow(unused_variables)]
    pub fn get_carriages(&self) -> &Vec<Ptr<MSTrainHelper_Carriage>> {
        &self.myCarriages
    }

    /// C++: `void MSTrainHelper::computeDoorPositions()`
    /// compute door positions on demand and fills the carriage structures
    // void
    // MSTrainHelper::computeDoorPositions() {
    //     for (Carriage* carriage : myCarriages) {
    //         Position dir = carriage->front - carriage->back;
    //         const double carriageLength = dir.length2D();
    //         if (carriageLength > 0.0) {
    //             dir.norm2D();
    //             for (int j = 1; j <= myCarriageDoors; j++) {
    //                 const double doorOffset = j * carriageLength / (myCarriageDoors + 1);
    //                 carriage->doorPositions.push_back(carriage->front - dir * doorOffset);
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn compute_door_positions(&mut self) {
        for carriage in self.myCarriages.iter() {
            let carriage = carriage.m();
            let mut dir = carriage.front - carriage.back;
            let carriageLength = dir.length2d();
            if carriageLength > 0.0 {
                dir.norm2d();
                for j in 1..=self.myCarriageDoors {
                    let doorOffset = j as f64 * carriageLength / (self.myCarriageDoors + 1) as f64;
                    carriage.doorPositions.push(carriage.front - dir * doorOffset);
                }
            }
        }
    }

    /// C++: `void MSTrainHelper::computeUnboardingPositions(double, std::vector<Position> &)`
    /// compute unboarding positions on demand and fills the carriage structures
    // void
    // MSTrainHelper::computeUnboardingPositions(double passengerRadius, std::vector<Position>& unboardingPositions) {
    //     passengerRadius += PEDESTRIAN_RADIUS_EXTRA_TOLERANCE;
    //     for (Carriage* carriage : myCarriages) {
    //         Position dir = carriage->front - carriage->back;
    //         const double carriageLength = dir.length2D();
    //         if (carriageLength > 0.0) {
    //             dir.norm2D();
    //             const Position perp = Position(-dir.y(), dir.x());
    //             double nbrLongitudinalCells, longitudinalOffset;
    //             longitudinalOffset = std::modf((carriageLength - 2.0 * passengerRadius) / (2.0 * passengerRadius), &nbrLongitudinalCells);
    //             double nbrLateralCells, lateralOffset;
    //             lateralOffset = std::modf((myHalfWidth * 2.0 - 2.0 * passengerRadius) / (2.0 * passengerRadius), &nbrLateralCells);
    //             const Position gridOrigin = carriage->back + dir * (passengerRadius + 0.5 * longitudinalOffset) - perp * (myHalfWidth - passengerRadius - 0.5 * lateralOffset);
    //             for (unsigned int i = 0; i <= (unsigned int)nbrLongitudinalCells; i++) {
    //                 for (unsigned int j = 0; j <= (unsigned int)nbrLateralCells; j++) {
    //                     carriage->unboardingPositions.push_back(gridOrigin + dir * i * 2.0 * passengerRadius + perp * j * 2.0 * passengerRadius);
    //                 }
    //             }
    //         }
    //         std::copy(carriage->unboardingPositions.begin(), carriage->unboardingPositions.end(), std::back_inserter(unboardingPositions));
    //     }
    //     // Shuffle the positions upstream so that we don't have to sample later on, just pop the last element.
    //     RandHelper::shuffle(unboardingPositions);
    // }
    #[allow(unused_variables)]
    pub fn compute_unboarding_positions(&mut self, passengerRadius: f64, unboardingPositions: &mut Vec<Position>) {
        let passengerRadius = passengerRadius + PEDESTRIAN_RADIUS_EXTRA_TOLERANCE;
        for carriage in self.myCarriages.iter() {
            let carriage = carriage.m();
            let mut dir = carriage.front - carriage.back;
            let carriageLength = dir.length2d();
            if carriageLength > 0.0 {
                dir.norm2d();
                let perp = Position::new(-dir.y(), dir.x());
                // std::modf: integral part and fractional part
                let longitudinalRatio = (carriageLength - 2.0 * passengerRadius) / (2.0 * passengerRadius);
                let nbrLongitudinalCells = longitudinalRatio.trunc();
                let longitudinalOffset = longitudinalRatio.fract();
                let lateralRatio = (self.myHalfWidth * 2.0 - 2.0 * passengerRadius) / (2.0 * passengerRadius);
                let nbrLateralCells = lateralRatio.trunc();
                let lateralOffset = lateralRatio.fract();
                let gridOrigin = carriage.back + dir * (passengerRadius + 0.5 * longitudinalOffset)
                    - perp * (self.myHalfWidth - passengerRadius - 0.5 * lateralOffset);
                for i in 0..=(nbrLongitudinalCells as u32) {
                    for j in 0..=(nbrLateralCells as u32) {
                        carriage.unboardingPositions.push(
                            gridOrigin + dir * i as f64 * 2.0 * passengerRadius + perp * j as f64 * 2.0 * passengerRadius,
                        );
                    }
                }
            }
            unboardingPositions.extend(carriage.unboardingPositions.iter().copied());
        }
        // Shuffle the positions upstream so that we don't have to sample later on, just pop the last element.
        utils::common::rand_helper::with_default(|rng| rng.shuffle(unboardingPositions));
    }

    /// C++: `double MSTrainHelper::getUpscaleLength(double, double, double, int)` [static]
    /// return length exaggeration factor (special for long vehicles)
    // double
    // MSTrainHelper::getUpscaleLength(double upscale, double length, double width, int vehicleQuality) {
    //     if (upscale > 1 && length > 5 && width < 5 && vehicleQuality != 4) {
    //         return MAX2(1.0, upscale * 5 / length);
    //     } else {
    //         return upscale;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_upscale_length_2(upscale: f64, length: f64, width: f64, vehicleQuality: i32) -> f64 {
        if upscale > 1.0 && length > 5.0 && width < 5.0 && vehicleQuality != 4 {
            f64::max(1.0, upscale * 5.0 / length)
        } else {
            upscale
        }
    }

    /// C++: `void MSTrainHelper::computeTrainDimensions(double, bool, double, int)` [private]
    // void
    // MSTrainHelper::computeTrainDimensions(double exaggeration, bool secondaryShape, double scaledLength, int vehicleQuality) {
    //     const MSVehicleType& vtype = myTrain->getVehicleType();
    //     const double laneFactor = (myTrain->getLane() != nullptr
    //                                ? myTrain->getLane()->getLengthGeometryFactor(secondaryShape)
    //                                : (myTrain->getEdge()->getLanes().size() > 0 ? myTrain->getEdge()->getLanes()[0]->getLengthGeometryFactor(secondaryShape) : 1));
    //     double totalLength = vtype.getLength();
    //     const double geometryScale = scaledLength / totalLength;
    //     myUpscaleLength = getUpscaleLength(exaggeration, totalLength, vtype.getWidth(), vehicleQuality);
    //     myLocomotiveLength = vtype.getParameter().locomotiveLength * myUpscaleLength;
    //     myDefaultLength = vtype.getParameter().carriageLength * myUpscaleLength;
    //     if (myLocomotiveLength == 0) {
    //         myLocomotiveLength = myDefaultLength;
    //     }
    //     const double minLength = MIN2(myLocomotiveLength, myDefaultLength);
    //     const double minScaledLength = MIN2(MIN_SCALED_CARRIAGE_LENGTH, minLength);
    //     myUnscale = geometryScale == 1 && laneFactor != 1;
    //     if (geometryScale < 1 && minLength * geometryScale < minScaledLength) {
    //         const double rescaleSmall = minScaledLength / (minLength * geometryScale);
    //         myLocomotiveLength *= rescaleSmall;
    //         myDefaultLength *= rescaleSmall;
    //     }
    //     myCarriageGap = vtype.getParameter().carriageGap * myUpscaleLength;
    //     myLength = totalLength * myUpscaleLength;
    //     myHalfWidth = 0.5 * vtype.getWidth() * exaggeration;
    //     myNumCarriages = MAX2(1, 1 + (int)((myLength - myLocomotiveLength) / (myDefaultLength + myCarriageGap) + 0.5)); // Round to closest integer.
    //     if (myUpscaleLength > 1 && vehicleQuality != 4) {
    //         // at high zoom, it doesn't help to draw many carriages)
    //         myNumCarriages = MIN2(myNumCarriages, 2);
    //         myLocomotiveLength = myLength / 2;
    //     }
    //     assert(myNumCarriages > 0);
    //     if (myNumCarriages == 1) {
    //         myCarriageGap = 0;
    //     }
    //     myCarriageLengthWithGap = myLength / myNumCarriages;
    //     myCarriageLength = myCarriageLengthWithGap - myCarriageGap;
    //     myFirstCarriageLength = myCarriageLength;
    //     if (myDefaultLength != myLocomotiveLength && myNumCarriages > 1) {
    //         myFirstCarriageLength = myLocomotiveLength;
    //         myCarriageLengthWithGap = (myLength - myLocomotiveLength) / (myNumCarriages - 1);
    //         myCarriageLength = myCarriageLengthWithGap - myCarriageGap;
    //     }
    //     myCarriageDoors = vtype.getParameter().carriageDoors;
    // }
    #[allow(unused_variables)]
    pub fn compute_train_dimensions(&mut self, exaggeration: f64, secondaryShape: bool, scaledLength: f64, vehicleQuality: i32) {
        let vtype = self.myTrain.get_vehicle_type();
        let laneFactor = if !self.myTrain.get_lane().is_null() {
            self.myTrain.get_lane().get_length_geometry_factor(secondaryShape)
        } else if self.myTrain.get_edge().get_lanes().len() > 0 {
            self.myTrain.get_edge().get_lanes()[0].get_length_geometry_factor(secondaryShape)
        } else {
            1.0
        };
        let totalLength = vtype.get_length();
        let geometryScale = scaledLength / totalLength;
        self.myUpscaleLength = Self::get_upscale_length_2(exaggeration, totalLength, vtype.get_width(), vehicleQuality);
        self.myLocomotiveLength = vtype.get_parameter().locomotiveLength * self.myUpscaleLength;
        self.myDefaultLength = vtype.get_parameter().carriageLength * self.myUpscaleLength;
        if self.myLocomotiveLength == 0.0 {
            self.myLocomotiveLength = self.myDefaultLength;
        }
        let minLength = f64::min(self.myLocomotiveLength, self.myDefaultLength);
        let minScaledLength = f64::min(MIN_SCALED_CARRIAGE_LENGTH, minLength);
        self.myUnscale = geometryScale == 1.0 && laneFactor != 1.0;
        if geometryScale < 1.0 && minLength * geometryScale < minScaledLength {
            let rescaleSmall = minScaledLength / (minLength * geometryScale);
            self.myLocomotiveLength *= rescaleSmall;
            self.myDefaultLength *= rescaleSmall;
        }
        self.myCarriageGap = vtype.get_parameter().carriageGap * self.myUpscaleLength;
        self.myLength = totalLength * self.myUpscaleLength;
        self.myHalfWidth = 0.5 * vtype.get_width() * exaggeration;
        // Round to closest integer.
        self.myNumCarriages = std::cmp::max(
            1,
            1 + ((self.myLength - self.myLocomotiveLength) / (self.myDefaultLength + self.myCarriageGap) + 0.5) as i32,
        );
        if self.myUpscaleLength > 1.0 && vehicleQuality != 4 {
            // at high zoom, it doesn't help to draw many carriages)
            self.myNumCarriages = std::cmp::min(self.myNumCarriages, 2);
            self.myLocomotiveLength = self.myLength / 2.0;
        }
        debug_assert!(self.myNumCarriages > 0);
        if self.myNumCarriages == 1 {
            self.myCarriageGap = 0.0;
        }
        self.myCarriageLengthWithGap = self.myLength / self.myNumCarriages as f64;
        self.myCarriageLength = self.myCarriageLengthWithGap - self.myCarriageGap;
        self.myFirstCarriageLength = self.myCarriageLength;
        if self.myDefaultLength != self.myLocomotiveLength && self.myNumCarriages > 1 {
            self.myFirstCarriageLength = self.myLocomotiveLength;
            self.myCarriageLengthWithGap = (self.myLength - self.myLocomotiveLength) / (self.myNumCarriages - 1) as f64;
            self.myCarriageLength = self.myCarriageLengthWithGap - self.myCarriageGap;
        }
        self.myCarriageDoors = vtype.get_parameter().carriageDoors;
    }

    /// C++: `void MSTrainHelper::computeCarriages(bool, bool)` [private]
    // void
    // MSTrainHelper::computeCarriages(bool reversed, bool secondaryShape) {
    //     myCarriages.clear();
    //
    //     const MSLane* lane = myTrain->getLane(); // Lane on which the carriage's front is situated.
    //     int furtherIndex = 0;
    //     const MSLane* backLane = lane; // Lane on which the carriage's back is situated.
    //     int backFurtherIndex = furtherIndex;
    //     // Offsets of front and back parts of a carriage.
    //     double carriageOffset = myTrain->getPositionOnLane();
    //     if (myTrain->getLaneChangeModel().isOpposite()) {
    //         // This still produces some artifacts when not fully on the current lane.
    //         carriageOffset = MIN2(carriageOffset + myTrain->getLength(), lane->getLength());
    //     }
    //     const double unscale = myUnscale ? 1. / lane->getLengthGeometryFactor() : 1.;
    //     double carriageBackOffset = carriageOffset - myFirstCarriageLength * unscale;
    //
    //     myFirstCarriageNo = 0;  // default case - we're going forwards
    //     myIsReversed = (myTrain->isReversed() && reversed) || myTrain->getLaneChangeModel().isOpposite();
    //     if (myIsReversed) {
    //         myFirstCarriageNo = myNumCarriages - 1;
    //         if (myNumCarriages > 1) {
    //             carriageBackOffset = carriageOffset - myCarriageLength;
    //         }
    //     }
    //     if (myTrain->getVehicleType().getParameter().locomotiveLength == 0) {
    //         myFirstCarriageNo = -1; // don't draw locomotive
    //     }
    //
    //     myFirstPassengerCarriage = myDefaultLength == myLocomotiveLength || myNumCarriages == 1
    //                                || (myTrain->getVClass() & (SVC_RAIL_ELECTRIC | SVC_RAIL_FAST | SVC_RAIL)) == 0 ? 0 : 1;
    //
    //     const double lateralOffset = (myTrain->isParking() && myTrain->getNextStopParameter()->posLat == INVALID_DOUBLE
    //                                   ? (myTrain->getLane()->getWidth() * (lefthand() ? -1 : 1))
    //                                   : -myTrain->getLateralPositionOnLane());
    //
    //     for (int i = 0; i < myNumCarriages; ++i) {
    //         Carriage* carriage = new Carriage();
    //         if (i == myFirstCarriageNo) {
    //             if (myFirstCarriageNo > 0) {
    //                 // Previous loop iteration has adjusted backpos for a normal carriage so have to correct
    //                 carriageBackOffset += myCarriageLengthWithGap;
    //                 carriageBackOffset -= myFirstCarriageLength + myCarriageGap;
    //             }
    //         }
    //         while (carriageOffset < 0) {
    //             const MSLane* prev = myTrain->getPreviousLane(lane, furtherIndex);
    //             if (prev != lane) {
    //                 carriageOffset += prev->getLength();
    //             } else {
    //                 break;
    //             }
    //             lane = prev;
    //         }
    //         while (carriageBackOffset < 0) {
    //             const MSLane* prev = myTrain->getPreviousLane(backLane, backFurtherIndex);
    //             if (prev != backLane) {
    //                 if (myUnscale) {
    //                     carriageBackOffset *= backLane->getLengthGeometryFactor() / prev->getLengthGeometryFactor();
    //                 }
    //                 carriageBackOffset += prev->getLength();
    //             } else {
    //                 break;
    //             }
    //             backLane = prev;
    //         }
    //         carriage->front = lane->getShape(secondaryShape).positionAtOffset2D(carriageOffset * lane->getLengthGeometryFactor(secondaryShape), lateralOffset, true);
    //         carriage->back = backLane->getShape(secondaryShape).positionAtOffset2D(carriageBackOffset * backLane->getLengthGeometryFactor(secondaryShape), lateralOffset, true);
    //         myCarriages.push_back(carriage);
    //         lane = backLane;
    //         furtherIndex = backFurtherIndex;
    //         carriageOffset = carriageBackOffset - myCarriageGap;
    //         carriageBackOffset -= myCarriageLengthWithGap * unscale;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn compute_carriages(&mut self, reversed: bool, secondaryShape: bool) {
        self.myCarriages.clear();

        let mut lane = self.myTrain.get_lane(); // Lane on which the carriage's front is situated.
        let mut furtherIndex: i32 = 0;
        let mut backLane = lane; // Lane on which the carriage's back is situated.
        let mut backFurtherIndex = furtherIndex;
        // Offsets of front and back parts of a carriage.
        let mut carriageOffset = self.myTrain.get_position_on_lane();
        if self.myTrain.get_lane_change_model_2().is_opposite() {
            // This still produces some artifacts when not fully on the current lane.
            carriageOffset = f64::min(carriageOffset + self.myTrain.get_length(), lane.get_length());
        }
        let unscale = if self.myUnscale { 1.0 / lane.get_length_geometry_factor_2() } else { 1.0 };
        let mut carriageBackOffset = carriageOffset - self.myFirstCarriageLength * unscale;

        self.myFirstCarriageNo = 0; // default case - we're going forwards
        self.myIsReversed = (self.myTrain.is_reversed() && reversed) || self.myTrain.get_lane_change_model_2().is_opposite();
        if self.myIsReversed {
            self.myFirstCarriageNo = self.myNumCarriages - 1;
            if self.myNumCarriages > 1 {
                carriageBackOffset = carriageOffset - self.myCarriageLength;
            }
        }
        if self.myTrain.get_vehicle_type().get_parameter().locomotiveLength == 0.0 {
            self.myFirstCarriageNo = -1; // don't draw locomotive
        }

        self.myFirstPassengerCarriage = if self.myDefaultLength == self.myLocomotiveLength
            || self.myNumCarriages == 1
            || (self.myTrain.get_v_class() & (SVC_RAIL_ELECTRIC | SVC_RAIL_FAST | SVC_RAIL)) == 0
        {
            0
        } else {
            1
        };

        let lateralOffset = if self.myTrain.is_parking() && self.myTrain.get_next_stop_parameter().posLat == INVALID_DOUBLE {
            lane.get_width() * (if lefthand() { -1.0 } else { 1.0 })
        } else {
            -self.myTrain.get_lateral_position_on_lane()
        };

        for i in 0..self.myNumCarriages {
            if i == self.myFirstCarriageNo {
                if self.myFirstCarriageNo > 0 {
                    // Previous loop iteration has adjusted backpos for a normal carriage so have to correct
                    carriageBackOffset += self.myCarriageLengthWithGap;
                    carriageBackOffset -= self.myFirstCarriageLength + self.myCarriageGap;
                }
            }
            while carriageOffset < 0.0 {
                let prev = self.myTrain.get_previous_lane(lane, &mut furtherIndex);
                if prev != lane {
                    carriageOffset += prev.get_length();
                } else {
                    break;
                }
                lane = prev;
            }
            while carriageBackOffset < 0.0 {
                let prev = self.myTrain.get_previous_lane(backLane, &mut backFurtherIndex);
                if prev != backLane {
                    if self.myUnscale {
                        carriageBackOffset *= backLane.get_length_geometry_factor_2() / prev.get_length_geometry_factor_2();
                    }
                    carriageBackOffset += prev.get_length();
                } else {
                    break;
                }
                backLane = prev;
            }
            let front = lane.get_shape(secondaryShape).position_at_offset2d(
                carriageOffset * lane.get_length_geometry_factor(secondaryShape),
                lateralOffset,
                true,
            );
            let back = backLane.get_shape(secondaryShape).position_at_offset2d(
                carriageBackOffset * backLane.get_length_geometry_factor(secondaryShape),
                lateralOffset,
                true,
            );
            self.myCarriages.push(Ptr::from_raw(Box::into_raw(Box::new(MSTrainHelper_Carriage {
                front,
                back,
                doorPositions: Vec::new(),
                unboardingPositions: Vec::new(),
            }))));
            lane = backLane;
            furtherIndex = backFurtherIndex;
            carriageOffset = carriageBackOffset - self.myCarriageGap;
            carriageBackOffset -= self.myCarriageLengthWithGap * unscale;
        }
    }

}

