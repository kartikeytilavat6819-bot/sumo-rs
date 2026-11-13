//! Port of microsim/output/MSVTKExport.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// Produce a VTK output to use with Tools like ParaView
/// C++ class `MSVTKExport` (microsim/output/MSVTKExport.h)
pub struct MSVTKExport {
}

impl MSVTKExport {
    /// C++: `void MSVTKExport::write(OutputDevice &, SUMOTime)` [static]
    /// Produce a VTK output to use with Tools like ParaView
    // void
    // MSVTKExport::write(OutputDevice& of, SUMOTime /* timestep */) {
    //
    //     std::vector<double> speed = getSpeed();
    //     std::vector<double> points = getPositions();
    //
    //     of << "<?xml version=\"1.0\" encoding=\"UTF-8\" ?>\n";
    //     of << "<VTKFile type=\"PolyData\" version=\"0.1\" order=\"LittleEndian\">\n";
    //     of << "<PolyData>\n";
    //     of << " <Piece NumberOfPoints=\"" << speed.size() << "\" NumberOfVerts=\"1\" NumberOfLines=\"0\" NumberOfStrips=\"0\" NumberOfPolys=\"0\">\n";
    //     of << "<PointData>\n";
    //     of << " <DataArray type=\"Float64\" Name=\"speed\" format=\"ascii\">" << List2String(getSpeed()) << "</DataArray>\n";
    //     of << "</PointData>\n";
    //     of << "<CellData/>\n";
    //     of << "<Points>\n";
    //     of << " <DataArray type=\"Float64\" Name=\"Points\" NumberOfComponents=\"3\" format=\"ascii\">" << List2String(getPositions()) << "</DataArray>\n";
    //     of << "</Points>\n";
    //     of << "<Verts>\n";
    //     of << " <DataArray type=\"Int64\" Name=\"connectivity\" format=\"ascii\">" <<  getOffset((int) speed.size()) << "</DataArray>\n";
    //     of << " <DataArray type=\"Int64\" Name=\"offsets\" format=\"ascii\">" << speed.size() << "</DataArray>\n";
    //     of << "</Verts>\n";
    //     of << "<Lines>\n";
    //     of << " <DataArray type=\"Int64\" Name=\"connectivity\" format=\"ascii\"/>\n";
    //     of << " <DataArray type=\"Int64\" Name=\"offsets\" format=\"ascii\"/>\n";
    //     of << "</Lines>\n";
    //     of << "<Stripes>\n";
    //     of << " <DataArray type=\"Int64\" Name=\"connectivity\" format=\"ascii\"/>\n";
    //     of << " <DataArray type=\"Int64\" Name=\"offsets\" format=\"ascii\"/>\n";
    //     of << "</Stripes>\n";
    //     of << "<Polys>\n";
    //     of << " <DataArray type=\"Int64\" Name=\"connectivity\" format=\"ascii\"/>\n";
    //     of << " <DataArray type=\"Int64\" Name=\"offsets\" format=\"ascii\"/>\n";
    //     of << "</Polys>\n";
    //     of << "</Piece>\n";
    //     of << "</PolyData>\n";
    //     of << "</VTKFile>";
    //
    // }
    #[allow(unused_variables)]
    pub fn write(of: &mut OutputDevice, timestep: SUMOTime) {
        let speed = Self::get_speed();
        let points = Self::get_positions();

        of.write("<?xml version=\"1.0\" encoding=\"UTF-8\" ?>\n");
        of.write("<VTKFile type=\"PolyData\" version=\"0.1\" order=\"LittleEndian\">\n");
        of.write("<PolyData>\n");
        of.write(" <Piece NumberOfPoints=\"").write(&speed.len()).write("\" NumberOfVerts=\"1\" NumberOfLines=\"0\" NumberOfStrips=\"0\" NumberOfPolys=\"0\">\n");
        of.write("<PointData>\n");
        of.write(" <DataArray type=\"Float64\" Name=\"speed\" format=\"ascii\">").write(Self::list2_string(Self::get_speed()).as_str()).write("</DataArray>\n");
        of.write("</PointData>\n");
        of.write("<CellData/>\n");
        of.write("<Points>\n");
        of.write(" <DataArray type=\"Float64\" Name=\"Points\" NumberOfComponents=\"3\" format=\"ascii\">").write(Self::list2_string(Self::get_positions()).as_str()).write("</DataArray>\n");
        of.write("</Points>\n");
        of.write("<Verts>\n");
        of.write(" <DataArray type=\"Int64\" Name=\"connectivity\" format=\"ascii\">").write(Self::get_offset(speed.len() as i32).as_str()).write("</DataArray>\n");
        of.write(" <DataArray type=\"Int64\" Name=\"offsets\" format=\"ascii\">").write(&speed.len()).write("</DataArray>\n");
        of.write("</Verts>\n");
        of.write("<Lines>\n");
        of.write(" <DataArray type=\"Int64\" Name=\"connectivity\" format=\"ascii\"/>\n");
        of.write(" <DataArray type=\"Int64\" Name=\"offsets\" format=\"ascii\"/>\n");
        of.write("</Lines>\n");
        of.write("<Stripes>\n");
        of.write(" <DataArray type=\"Int64\" Name=\"connectivity\" format=\"ascii\"/>\n");
        of.write(" <DataArray type=\"Int64\" Name=\"offsets\" format=\"ascii\"/>\n");
        of.write("</Stripes>\n");
        of.write("<Polys>\n");
        of.write(" <DataArray type=\"Int64\" Name=\"connectivity\" format=\"ascii\"/>\n");
        of.write(" <DataArray type=\"Int64\" Name=\"offsets\" format=\"ascii\"/>\n");
        of.write("</Polys>\n");
        of.write("</Piece>\n");
        of.write("</PolyData>\n");
        of.write("</VTKFile>");
    }

    /// C++: `void MSVTKExport::MSVTKExport(const MSVTKExport &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new(arg0: &MSVTKExport) -> Self {
        MSVTKExport {}
    }

    /// C++: `MSVTKExport & MSVTKExport::operator=(const MSVTKExport &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSVTKExport) -> &mut MSVTKExport {
        self
    }

    /// C++: `std::string MSVTKExport::trim(std::string)` [static, private]
    /// Deletes the whitespaces at the end of a String
    // std::string
    // MSVTKExport::trim(std::string istring) {
    //     bool trimmed = false;
    //
    //     if (ctype_space(istring[istring.length() - 1])) {
    //         istring.erase(istring.length() - 1);
    //         trimmed = true;
    //     }
    //
    //     if (ctype_space(istring[0])) {
    //         istring.erase(0, 1);
    //         trimmed = true;
    //     }
    //
    //     if (!trimmed) {
    //         return istring;
    //     } else {
    //         return trim(istring);
    //     }
    //
    // }
    #[allow(unused_variables)]
    pub fn trim(mut istring: String) -> String {
        // shortcut: C++ reads istring[length - 1] on empty input (undefined behaviour); empty input is returned unchanged, upgrade only if that UB must be reproduced
        if istring.is_empty() {
            return istring;
        }
        let mut trimmed = false;

        if Self::ctype_space(istring.as_bytes()[istring.len() - 1] as i8) {
            istring.pop();
            trimmed = true;
        }

        if !istring.is_empty() && Self::ctype_space(istring.as_bytes()[0] as i8) {
            istring.remove(0);
            trimmed = true;
        }

        if !trimmed {
            istring
        } else {
            Self::trim(istring)
        }
    }

    /// C++: `bool MSVTKExport::ctype_space(const char)` [static, private]
    /// Checks if there is a whitespace
    // bool
    // MSVTKExport::ctype_space(const char c) {
    //     if (c == ' ' || c == '\t' || c == '\r' || c == '\n' || c == 11) {
    //         return true;
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn ctype_space(c: i8) -> bool {
        let c = c as u8;
        if c == b' ' || c == b'\t' || c == b'\r' || c == b'\n' || c == 11 {
            return true;
        }
        false
    }

    /// C++: `std::string MSVTKExport::List2String(std::vector<double>)` [static, private]
    /// Get a comma separated String from a Vector
    // std::string
    // MSVTKExport::List2String(std::vector<double> input) {
    //
    //     std::string output = "";
    //     for (int i = 0; i < (int)input.size(); i++) {
    //
    //         std::stringstream ss;
    //
    //         //for a high precision
    //         //ss.precision(::std::numeric_limits<double>::digits10);
    //         //ss.unsetf(::std::ios::dec);
    //         //ss.setf(::std::ios::scientific);
    //
    //         ss << input[i] << " ";
    //         output += ss.str();
    //     }
    //
    //     return trim(output);
    // }
    #[allow(unused_variables)]
    pub fn list2_string(input: Vec<f64>) -> String {
        let mut output = String::new();
        for i in 0..input.len() {
            output += &(utils::common::to_string::f64_to_string_default(input[i]) + " ");
        }
        Self::trim(output)
    }

    /// C++: `std::vector<double> MSVTKExport::getSpeed()` [static, private]
    /// Get a Vector with the speed values of each vehicle in the actual timestep
    // std::vector<double>
    // MSVTKExport::getSpeed() {
    //
    //     std::vector<double> output;
    //
    //     MSVehicleControl& vc = MSNet::getInstance()->getVehicleControl();
    //     MSVehicleControl::constVehIt it = vc.loadedVehBegin();
    //     MSVehicleControl::constVehIt end = vc.loadedVehEnd();
    //
    //
    //     for (; it != end; ++it) {
    //         const MSVehicle* veh = static_cast<const MSVehicle*>((*it).second);
    //
    //         if (veh->isOnRoad()) {
    //
    //             //Position pos = veh->getLane()->getShape().positionAtOffset(veh->getPositionOnLane());
    //             output.push_back(veh->getSpeed());
    //         }
    //
    //     }
    //
    //     return output;
    // }
    #[allow(unused_variables)]
    pub fn get_speed() -> Vec<f64> {
        let mut output = Vec::new();
        let msnet = MSNet::get_instance();
        let vc = msnet.m().get_vehicle_control();
        for (_, p) in vc.myVehicleDict.iter() {
            let veh = p.cast::<MSVehicle>();
            if veh.is_on_road() {
                output.push(veh.get_speed());
            }
        }
        output
    }

    /// C++: `std::vector<double> MSVTKExport::getPositions()` [static, private]
    /// Get a Vector of the Positions (x,y,z) of each vehicle in the actual timestep
    // std::vector<double>
    // MSVTKExport::getPositions() {
    //
    //     std::vector<double> output;
    //
    //     MSVehicleControl& vc = MSNet::getInstance()->getVehicleControl();
    //     MSVehicleControl::constVehIt it = vc.loadedVehBegin();
    //     MSVehicleControl::constVehIt end = vc.loadedVehEnd();
    //
    //
    //     for (; it != end; ++it) {
    //         const MSVehicle* veh = static_cast<const MSVehicle*>((*it).second);
    //
    //         if (veh->isOnRoad()) {
    //
    //             output.push_back(veh->getPosition().x());
    //             output.push_back(veh->getPosition().y());
    //             output.push_back(veh->getPosition().z());
    //
    //         }
    //
    //     }
    //
    //     return output;
    // }
    #[allow(unused_variables)]
    pub fn get_positions() -> Vec<f64> {
        let mut output = Vec::new();
        let msnet = MSNet::get_instance();
        let vc = msnet.m().get_vehicle_control();
        for (_, p) in vc.myVehicleDict.iter() {
            let veh = p.cast::<MSVehicle>();
            if veh.is_on_road() {
                let pos = veh.get_position(0.0);
                output.push(pos.x());
                output.push(pos.y());
                output.push(pos.z());
            }
        }
        output
    }

    /// C++: `std::string MSVTKExport::getOffset(int)` [static, private]
    /// Get a String with the indexes of all vehicles (needed in the VTk File)
    // std::string
    // MSVTKExport::getOffset(int nr) {
    //
    //     std::string output = "";
    //     for (int i = 0; i < nr; i++) {
    //
    //         std::stringstream ss;
    //         ss << i << " ";
    //         output += ss.str();
    //     }
    //
    //     return trim(output);
    // }
    #[allow(unused_variables)]
    pub fn get_offset(nr: i32) -> String {
        let mut output = String::new();
        for i in 0..nr {
            output += &format!("{} ", i);
        }
        Self::trim(output)
    }

}

