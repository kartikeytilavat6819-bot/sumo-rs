//! Port of microsim/output/MSVTypeProbe.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// Writes positions of vehicles that have a certain (named) type
/// C++ class `MSVTypeProbe` (microsim/output/MSVTypeProbe.h)
#[repr(C)]
pub struct MSVTypeProbe {
    /// C++ base class `Named`
    pub base: Named,
    /// C++ base class `Command`
    pub base2: Unmapped /* Command */,
    /// The id of the vehicle type vehicles must have to be reported
    pub myVType: String,
    /// The device to write into
    pub myOutputDevice: Ptr<OutputDevice>,
    /// The frequency of reporting
    pub myFrequency: SUMOTime,
}

impl std::ops::Deref for MSVTypeProbe {
    type Target = Named;
    fn deref(&self) -> &Named {
        &self.base
    }
}
impl std::ops::DerefMut for MSVTypeProbe {
    fn deref_mut(&mut self) -> &mut Named {
        &mut self.base
    }
}

impl MSVTypeProbe {
    /// C++: `void MSVTypeProbe::MSVTypeProbe(const std::string &, const std::string &, OutputDevice &, SUMOTime)`
    /// Constructor
    // MSVTypeProbe::MSVTypeProbe(const std::string& id,
    //                            const std::string& vType,
    //                            OutputDevice& od, SUMOTime frequency)
    //     : Named(id), myVType(vType), myOutputDevice(od), myFrequency(frequency) {
    //     MSNet::getInstance()->getEndOfTimestepEvents()->addEvent(this);
    //     myOutputDevice.writeXMLHeader("vehicle-type-probes", "vtypeprobe_file.xsd");
    // }
    #[allow(unused_variables)]
    pub fn new(id: &str, vType: &str, od: &mut OutputDevice, frequency: SUMOTime) -> Self {
        todo!("port MSVTypeProbe::MSVTypeProbe")
    }

    /// C++: `void MSVTypeProbe::~MSVTypeProbe() noexcept` [virtual]
    /// Destructor
    // MSVTypeProbe::~MSVTypeProbe() {
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MSVTypeProbe::~MSVTypeProbe")
    }

    /// C++: `SUMOTime MSVTypeProbe::execute(SUMOTime)` [virtual]
    /// Writes values into the given stream
    // SUMOTime
    // MSVTypeProbe::execute(SUMOTime currentTime) {
    //     myOutputDevice.openTag(SUMO_TAG_TIMESTEP);
    //     myOutputDevice.writeAttr(SUMO_ATTR_TIME, time2string(currentTime));
    //     myOutputDevice.writeAttr(SUMO_ATTR_ID, getID());
    //     myOutputDevice.writeAttr("vType", myVType);
    //     MSVehicleControl& vc = MSNet::getInstance()->getVehicleControl();
    //     for (MSVehicleControl::constVehIt it = vc.loadedVehBegin(); it != vc.loadedVehEnd(); ++it) {
    //         const SUMOVehicle* veh = it->second;
    //         const MSVehicle* microVeh = dynamic_cast<const MSVehicle*>(veh);
    //         if (myVType == "" || myVType == veh->getVehicleType().getID()) {
    //             if (veh->isOnRoad()) {
    //                 Position pos = veh->getPosition();
    //                 myOutputDevice.openTag(SUMO_TAG_VEHICLE);
    //                 myOutputDevice.writeAttr(SUMO_ATTR_ID, veh->getID());
    //                 if (microVeh != nullptr) {
    //                     myOutputDevice.writeAttr(SUMO_ATTR_LANE, microVeh->getLane()->getID());
    //                 }
    //                 myOutputDevice.writeAttr(SUMO_ATTR_POSITION, veh->getPositionOnLane());
    //                 myOutputDevice.writeAttr(SUMO_ATTR_X, pos.x());
    //                 myOutputDevice.writeAttr(SUMO_ATTR_Y, pos.y());
    //                 if (MSNet::getInstance()->hasElevation()) {
    //                     myOutputDevice.writeAttr(SUMO_ATTR_Z, pos.z());
    //                 }
    //                 if (GeoConvHelper::getFinal().usingGeoProjection()) {
    //                     GeoConvHelper::getFinal().cartesian2geo(pos);
    //                     myOutputDevice.setPrecision(gPrecisionGeo);
    //                     myOutputDevice.writeAttr(SUMO_ATTR_LAT, pos.y());
    //                     myOutputDevice.writeAttr(SUMO_ATTR_LON, pos.x());
    //                     myOutputDevice.setPrecision();
    //                 }
    //                 myOutputDevice.writeAttr(SUMO_ATTR_SPEED, veh->getSpeed());
    //                 myOutputDevice.closeTag();
    //             }
    //         }
    //     }
    //     myOutputDevice.closeTag();
    //     return myFrequency;
    // }
    #[allow(unused_variables)]
    pub fn execute(&mut self, currentTime: SUMOTime) -> SUMOTime {
        todo!("port MSVTypeProbe::execute")
    }

    /// C++: `void MSVTypeProbe::MSVTypeProbe(const MSVTypeProbe &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSVTypeProbe) -> Self {
        todo!("port MSVTypeProbe::MSVTypeProbe")
    }

    /// C++: `MSVTypeProbe & MSVTypeProbe::operator=(const MSVTypeProbe &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSVTypeProbe) -> &mut MSVTypeProbe {
        todo!("port MSVTypeProbe::operator=")
    }

}

