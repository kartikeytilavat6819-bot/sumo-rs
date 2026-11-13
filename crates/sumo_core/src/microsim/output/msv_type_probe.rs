//! Port of microsim/output/MSVTypeProbe.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::std_defs::{g_precision, g_precision_geo};
use utils::common::time::time2string;

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

impl Command for MSVTypeProbe {
    /// C++ `MSVTypeProbe::execute` (the virtual override)
    fn execute(&mut self, currentTime: SUMOTime) -> SUMOTime {
        MSVTypeProbe::execute(self, currentTime)
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
    // Returns Box<Self>: the constructor registers `this` with the end-of-timestep events.
    #[allow(unused_variables)]
    pub fn new(id: &str, vType: &str, od: &mut OutputDevice, frequency: SUMOTime) -> Box<Self> {
        let mut b = Box::new(MSVTypeProbe {
            base: Named::new(id),
            base2: (),
            myVType: vType.to_string(),
            myOutputDevice: Ptr::from_mut(od),
            myFrequency: frequency,
        });
        let cmd: *mut dyn Command = &mut *b;
        let net = MSNet::get_instance();
        let events = net.m().get_end_of_timestep_events();
        events.m().add_event(Ptr::from_raw(cmd), -1);
        b.myOutputDevice.m().write_xml_header("vehicle-type-probes", "vtypeprobe_file.xsd", BTreeMap::new(), true);
        b
    }

    /// C++: `void MSVTypeProbe::~MSVTypeProbe() noexcept` [virtual]
    /// Destructor
    // MSVTypeProbe::~MSVTypeProbe() {
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
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
        self.myOutputDevice.m().open_tag_id(SUMO_TAG_TIMESTEP);
        self.myOutputDevice.m().write_attr(SUMO_ATTR_TIME, &time2string(currentTime));
        self.myOutputDevice.m().write_attr(SUMO_ATTR_ID, self.get_id());
        self.myOutputDevice.m().write_attr("vType", self.myVType.as_str());
        let net = MSNet::get_instance();
        let vc = net.m().get_vehicle_control();
        for v in vc.myVehicleDict.values() {
            let veh: Ptr<SUMOVehicle> = *v;
            // shortcut: dynamic_cast<const MSVehicle*> has no runtime check; every entry of myVehicleDict is an MSVehicle, upgrade when other SUMOVehicle kinds are stored there
            let microVeh: Ptr<MSVehicle> = veh.cast::<MSVehicle>();
            if self.myVType.is_empty() || self.myVType == microVeh.get_vehicle_type().get_id() {
                if microVeh.is_on_road() {
                    let mut pos: Position = microVeh.get_position(0.0);
                    self.myOutputDevice.m().open_tag_id(SUMO_TAG_VEHICLE);
                    self.myOutputDevice.m().write_attr(SUMO_ATTR_ID, microVeh.get_id());
                    if !microVeh.is_null() {
                        self.myOutputDevice.m().write_attr(SUMO_ATTR_LANE, microVeh.get_lane().get_id());
                    }
                    self.myOutputDevice.m().write_attr(SUMO_ATTR_POSITION, &microVeh.get_position_on_lane());
                    self.myOutputDevice.m().write_attr(SUMO_ATTR_X, &pos.x());
                    self.myOutputDevice.m().write_attr(SUMO_ATTR_Y, &pos.y());
                    if net.m().has_elevation() {
                        self.myOutputDevice.m().write_attr(SUMO_ATTR_Z, &pos.z());
                    }
                    if GeoConvHelper::with_final(|g| g.using_geo_projection()) {
                        GeoConvHelper::with_final(|g| g.cartesian2geo(&mut pos));
                        self.myOutputDevice.m().set_precision(g_precision_geo());
                        self.myOutputDevice.m().write_attr(SUMO_ATTR_LAT, &pos.y());
                        self.myOutputDevice.m().write_attr(SUMO_ATTR_LON, &pos.x());
                        self.myOutputDevice.m().set_precision(g_precision());
                    }
                    self.myOutputDevice.m().write_attr(SUMO_ATTR_SPEED, &microVeh.get_speed());
                    self.myOutputDevice.m().close_tag("");
                }
            }
        }
        self.myOutputDevice.m().close_tag("");
        self.myFrequency
    }

    /// C++: `void MSVTypeProbe::MSVTypeProbe(const MSVTypeProbe &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSVTypeProbe) -> Self {
        panic!("MSVTypeProbe copy constructor is private and invalidated in C++")
    }

    /// C++: `MSVTypeProbe & MSVTypeProbe::operator=(const MSVTypeProbe &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSVTypeProbe) -> &mut MSVTypeProbe {
        panic!("MSVTypeProbe assignment operator is private and invalidated in C++")
    }

}

