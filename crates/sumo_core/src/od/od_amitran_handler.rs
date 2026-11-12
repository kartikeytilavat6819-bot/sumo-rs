//! Port of od/ODAmitranHandler.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use utils::xml::generic_sax_handler::GenericSAXHandler;
use crate::prelude::*;

/// An XML-Handler for districts
/// C++ class `ODAmitranHandler` (od/ODAmitranHandler.h)
#[repr(C)]
pub struct ODAmitranHandler {
    /// C++ base class `SUMOSAXHandler`
    pub base: GenericSAXHandler,
    /// The matrix to add demand to
    pub myMatrix: Ptr<ODMatrix>,
    pub myVehicleType: String,
    pub myBegin: SUMOTime,
    pub myEnd: SUMOTime,
}

impl std::ops::Deref for ODAmitranHandler {
    type Target = GenericSAXHandler;
    fn deref(&self) -> &GenericSAXHandler {
        &self.base
    }
}
impl std::ops::DerefMut for ODAmitranHandler {
    fn deref_mut(&mut self) -> &mut GenericSAXHandler {
        &mut self.base
    }
}

impl ODAmitranHandler {
    /// C++: `void ODAmitranHandler::ODAmitranHandler(ODMatrix &, const std::string &)`
    /// Constructor
    // ODAmitranHandler::ODAmitranHandler(ODMatrix& matrix, const std::string& file)
    //     : SUMOSAXHandler(file), myMatrix(matrix) {}
    #[allow(unused_variables)]
    pub fn new(matrix: &mut ODMatrix, file: &str) -> Self {
        todo!("port ODAmitranHandler::ODAmitranHandler")
    }

    /// C++: `void ODAmitranHandler::~ODAmitranHandler() noexcept` [virtual]
    /// Destructor
    // ODAmitranHandler::~ODAmitranHandler() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port ODAmitranHandler::~ODAmitranHandler")
    }

    /// C++: `void ODAmitranHandler::myStartElement(int, const SUMOSAXAttributes &)` [virtual, protected]
    /// Called when an opening-tag occurs
    // void
    // ODAmitranHandler::myStartElement(int element, const SUMOSAXAttributes& attrs) {
    //     bool ok = true;
    //     switch (element) {
    //         case SUMO_TAG_ACTORCONFIG:
    //             myVehicleType = attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
    //             break;
    //         case SUMO_TAG_TIMESLICE:
    //             myBegin = attrs.get<int>(SUMO_ATTR_STARTTIME, myVehicleType.c_str(), ok);
    //             myEnd = myBegin + attrs.get<int>(SUMO_ATTR_DURATION, myVehicleType.c_str(), ok);
    //             if (myBegin >= myEnd) {
    //                 WRITE_ERRORF(TL("Invalid duration for timeSlice starting %."), toString(myBegin));
    //             }
    //             break;
    //         case SUMO_TAG_OD_PAIR:
    //             myMatrix.add(attrs.get<double>(SUMO_ATTR_AMOUNT, myVehicleType.c_str(), ok),
    //                          std::make_pair(myBegin, myEnd), attrs.get<std::string>(SUMO_ATTR_ORIGIN, myVehicleType.c_str(), ok),
    //                          attrs.get<std::string>(SUMO_ATTR_DESTINATION, myVehicleType.c_str(), ok), myVehicleType);
    //             break;
    //         default:
    //             break;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) {
        todo!("port ODAmitranHandler::myStartElement")
    }

    /// C++: `void ODAmitranHandler::ODAmitranHandler(const ODAmitranHandler &)` [private]
    /// invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(s: &ODAmitranHandler) -> Self {
        todo!("port ODAmitranHandler::ODAmitranHandler")
    }

    /// C++: `ODAmitranHandler & ODAmitranHandler::operator=(const ODAmitranHandler &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &ODAmitranHandler) -> &mut ODAmitranHandler {
        todo!("port ODAmitranHandler::operator=")
    }

}

