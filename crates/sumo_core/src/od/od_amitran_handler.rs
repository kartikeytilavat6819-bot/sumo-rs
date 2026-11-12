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
        ODAmitranHandler {
            base: GenericSAXHandler::new(TAGS, SUMO_TAG_NOTHING, ATTRS, SUMO_ATTR_NOTHING, file, ""),
            myMatrix: Ptr::from_mut(matrix),
            myVehicleType: String::new(),
            myBegin: 0,
            myEnd: 0,
        }
    }

    /// C++: `void ODAmitranHandler::~ODAmitranHandler() noexcept` [virtual]
    /// Destructor
    // ODAmitranHandler::~ODAmitranHandler() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
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
        let mut ok = true;
        match element {
            SUMO_TAG_ACTORCONFIG => {
                self.myVehicleType = attrs.get::<String>(SUMO_ATTR_ID, "", &mut ok, true);
            }
            SUMO_TAG_TIMESLICE => {
                self.myBegin = attrs.get::<i32>(SUMO_ATTR_STARTTIME, &self.myVehicleType, &mut ok, true) as SUMOTime;
                self.myEnd = self.myBegin + attrs.get::<i32>(SUMO_ATTR_DURATION, &self.myVehicleType, &mut ok, true) as SUMOTime;
                if self.myBegin >= self.myEnd {
                    write_errorf!("Invalid duration for timeSlice starting %.", utils::common::to_string::to_string(&self.myBegin));
                }
            }
            SUMO_TAG_OD_PAIR => {
                // determinism: C++ argument evaluation order is unspecified; errors are reported in textual order here
                let amount = attrs.get::<f64>(SUMO_ATTR_AMOUNT, &self.myVehicleType, &mut ok, true);
                let begin_end = (self.myBegin, self.myEnd);
                let origin = attrs.get::<String>(SUMO_ATTR_ORIGIN, &self.myVehicleType, &mut ok, true);
                let destination = attrs.get::<String>(SUMO_ATTR_DESTINATION, &self.myVehicleType, &mut ok, true);
                let vtype = self.myVehicleType.clone();
                self.myMatrix.add(amount, &begin_end, &origin, &destination, &vtype, false, false, false);
            }
            _ => {}
        }
    }

    /// C++: `void ODAmitranHandler::ODAmitranHandler(const ODAmitranHandler &)` [private]
    /// invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(s: &ODAmitranHandler) -> Self {
        panic!("ODAmitranHandler copy constructor is private and undefined in C++")
    }

    /// C++: `ODAmitranHandler & ODAmitranHandler::operator=(const ODAmitranHandler &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &ODAmitranHandler) -> &mut ODAmitranHandler {
        panic!("ODAmitranHandler assignment operator is private and undefined in C++")
    }

}

const TAGS: &[(&str, i32)] = &[
    ("timeSlice", SUMO_TAG_TIMESLICE),
    ("actorConfig", SUMO_TAG_ACTORCONFIG),
    ("odPair", SUMO_TAG_OD_PAIR),
    ("", SUMO_TAG_NOTHING),
];
const ATTRS: &[(&str, i32)] = &[
    ("id", SUMO_ATTR_ID),
    ("startTime", SUMO_ATTR_STARTTIME),
    ("duration", SUMO_ATTR_DURATION),
    ("amount", SUMO_ATTR_AMOUNT),
    ("origin", SUMO_ATTR_ORIGIN),
    ("destination", SUMO_ATTR_DESTINATION),
    ("", SUMO_ATTR_NOTHING),
];

impl SaxHandler for ODAmitranHandler {
    fn handler(&mut self) -> &mut GenericSAXHandler {
        &mut self.base
    }
    fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) -> Result<(), ProcessError> {
        ODAmitranHandler::my_start_element(self, element, attrs);
        Ok(())
    }
}

