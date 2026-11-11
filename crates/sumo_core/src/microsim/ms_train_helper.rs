//! Port of microsim/MSTrainHelper.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

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
        todo!("port MSTrainHelper::MSTrainHelper")
    }

    /// C++: `void MSTrainHelper::~MSTrainHelper() noexcept`
    // ~MSTrainHelper() {
    //         for (const Carriage* carriage : myCarriages) {
    //             delete carriage;
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MSTrainHelper::~MSTrainHelper")
    }

    /// C++: `double MSTrainHelper::getUpscaleLength() const` [const]
    // inline double getUpscaleLength() const {
    //         return myUpscaleLength;
    //     }
    #[allow(unused_variables)]
    pub fn get_upscale_length(&self) -> f64 {
        todo!("port MSTrainHelper::getUpscaleLength")
    }

    /// C++: `double MSTrainHelper::getHalfWidth() const` [const]
    // inline double getHalfWidth() const {
    //         return myHalfWidth;
    //     }
    #[allow(unused_variables)]
    pub fn get_half_width(&self) -> f64 {
        todo!("port MSTrainHelper::getHalfWidth")
    }

    /// C++: `int MSTrainHelper::getNumCarriages() const` [const]
    // inline int getNumCarriages() const {
    //         return myNumCarriages;
    //     }
    #[allow(unused_variables)]
    pub fn get_num_carriages(&self) -> i32 {
        todo!("port MSTrainHelper::getNumCarriages")
    }

    /// C++: `double MSTrainHelper::getCarriageLength() const` [const]
    // inline double getCarriageLength() const {
    //         return myCarriageLength;
    //     }
    #[allow(unused_variables)]
    pub fn get_carriage_length(&self) -> f64 {
        todo!("port MSTrainHelper::getCarriageLength")
    }

    /// C++: `double MSTrainHelper::getFirstCarriageLength() const` [const]
    // inline double getFirstCarriageLength() const {
    //         return myFirstCarriageLength;
    //     }
    #[allow(unused_variables)]
    pub fn get_first_carriage_length(&self) -> f64 {
        todo!("port MSTrainHelper::getFirstCarriageLength")
    }

    /// C++: `int MSTrainHelper::getCarriageDoors() const` [const]
    // inline int getCarriageDoors() const {
    //         return myCarriageDoors;
    //     }
    #[allow(unused_variables)]
    pub fn get_carriage_doors(&self) -> i32 {
        todo!("port MSTrainHelper::getCarriageDoors")
    }

    /// C++: `int MSTrainHelper::getFirstCarriageNo() const` [const]
    // inline int getFirstCarriageNo() const {
    //         return myFirstCarriageNo;
    //     }
    #[allow(unused_variables)]
    pub fn get_first_carriage_no(&self) -> i32 {
        todo!("port MSTrainHelper::getFirstCarriageNo")
    }

    /// C++: `int MSTrainHelper::getFirstPassengerCarriage() const` [const]
    // inline int getFirstPassengerCarriage() const {
    //         return myFirstPassengerCarriage;
    //     }
    #[allow(unused_variables)]
    pub fn get_first_passenger_carriage(&self) -> i32 {
        todo!("port MSTrainHelper::getFirstPassengerCarriage")
    }

    /// C++: `bool MSTrainHelper::isReversed() const` [const]
    // inline bool isReversed() const {
    //         return myIsReversed;
    //     }
    #[allow(unused_variables)]
    pub fn is_reversed(&self) -> bool {
        todo!("port MSTrainHelper::isReversed")
    }

    /// C++: `const std::vector<Carriage *> & MSTrainHelper::getCarriages() const` [const]
    // inline const std::vector<Carriage*>& getCarriages() const {
    //         return myCarriages;
    //     }
    #[allow(unused_variables)]
    pub fn get_carriages(&self) -> &Vec<Ptr<MSTrainHelper_Carriage>> {
        todo!("port MSTrainHelper::getCarriages")
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
        todo!("port MSTrainHelper::computeDoorPositions")
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
        todo!("port MSTrainHelper::computeUnboardingPositions")
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
        todo!("port MSTrainHelper::getUpscaleLength")
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
        todo!("port MSTrainHelper::computeTrainDimensions")
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
    //                                   ? (myTrain->getLane()->getWidth() * (MSGlobals::gLefthand ? -1 : 1))
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
        todo!("port MSTrainHelper::computeCarriages")
    }

}

