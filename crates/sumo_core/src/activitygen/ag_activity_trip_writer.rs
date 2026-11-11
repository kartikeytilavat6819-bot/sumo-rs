//! Port of activitygen/AGActivityTripWriter.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `AGActivityTripWriter` (activitygen/AGActivityTripWriter.h)
pub struct AGActivityTripWriter {
    pub myTripOutput: Ptr<OutputDevice>,
}

impl AGActivityTripWriter {
    /// C++: `void AGActivityTripWriter::AGActivityTripWriter(OutputDevice &)`
    // AGActivityTripWriter::AGActivityTripWriter(OutputDevice& file) : myTripOutput(file) {
    //     myTripOutput.openTag(SUMO_TAG_VTYPE)
    //     .writeAttr(SUMO_ATTR_ID, "default")
    //     .writeAttr(SUMO_ATTR_VCLASS, "passenger")
    //     .writeAttr(SUMO_ATTR_COLOR, RGBColor::RED).closeTag();
    //     myTripOutput.openTag(SUMO_TAG_VTYPE)
    //     .writeAttr(SUMO_ATTR_ID, "random")
    //     .writeAttr(SUMO_ATTR_VCLASS, "passenger")
    //     .writeAttr(SUMO_ATTR_COLOR, RGBColor::BLUE).closeTag();
    //     myTripOutput.openTag(SUMO_TAG_VTYPE)
    //     .writeAttr(SUMO_ATTR_ID, "bus")
    //     .writeAttr(SUMO_ATTR_VCLASS, "bus")
    //     .writeAttr(SUMO_ATTR_COLOR, RGBColor::GREEN).closeTag();
    //     myTripOutput.lf();
    // }
    #[allow(unused_variables)]
    pub fn new(file: &mut OutputDevice) -> Self {
        todo!("port AGActivityTripWriter::AGActivityTripWriter")
    }

    /// C++: `void AGActivityTripWriter::addTrip(const AGTrip &)`
    // void
    // AGActivityTripWriter::addTrip(const AGTrip& trip) {
    //     int time = (trip.getDay() - 1) * 86400 + trip.getTime();
    //
    //     myTripOutput.openTag(SUMO_TAG_TRIP)
    //     .writeAttr(SUMO_ATTR_ID, trip.getVehicleName())
    //     .writeAttr(SUMO_ATTR_TYPE, trip.getType())
    //     .writeAttr(SUMO_ATTR_DEPART, time2string(TIME2STEPS(time)))
    //     .writeAttr(SUMO_ATTR_DEPARTPOS, trip.getDep().getPosition())
    //     .writeAttr(SUMO_ATTR_ARRIVALPOS, trip.getArr().getPosition())
    //     .writeAttr(SUMO_ATTR_ARRIVALSPEED, 0.)
    //     .writeAttr(SUMO_ATTR_FROM, trip.getDep().getStreet().getID());
    //
    //     if (!trip.getPassed()->empty()) {
    //         std::ostringstream oss;
    //         for (std::list<AGPosition>::const_iterator it = trip.getPassed()->begin(); it != trip.getPassed()->end(); ++it) {
    //             if (it != trip.getPassed()->begin()) {
    //                 oss << " ";
    //             }
    //             oss << it->getStreet().getID();
    //         }
    //         myTripOutput.writeAttr(SUMO_ATTR_VIA, oss.str());
    //     }
    //     myTripOutput.writeAttr(SUMO_ATTR_TO, trip.getArr().getStreet().getID());
    //     myTripOutput.closeTag();
    // }
    #[allow(unused_variables)]
    pub fn add_trip(&mut self, trip: &AGTrip) {
        todo!("port AGActivityTripWriter::addTrip")
    }

    /// C++: `void AGActivityTripWriter::AGActivityTripWriter(const AGActivityTripWriter &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &AGActivityTripWriter) -> Self {
        todo!("port AGActivityTripWriter::AGActivityTripWriter")
    }

    /// C++: `AGActivityTripWriter & AGActivityTripWriter::operator=(const AGActivityTripWriter &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &AGActivityTripWriter) -> &mut AGActivityTripWriter {
        todo!("port AGActivityTripWriter::operator=")
    }

}

