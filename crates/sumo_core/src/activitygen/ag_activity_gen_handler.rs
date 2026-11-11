//! Port of activitygen/AGActivityGenHandler.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `AGActivityGenHandler` (activitygen/AGActivityGenHandler.h)
pub struct AGActivityGenHandler {
    /// C++ base class `SUMOSAXHandler`
    pub base: Unmapped /* SUMOSAXHandler */,
    /// The city to store the information into
    pub myCity: Ptr<AGCity>,
    /// The name of the object that is currently processed
    pub myCurrentObject: String,
    pub currentBusLine: Ptr<AGBusLine>,
    /// indicator whether the current station (in bus line context) is a reverse station or not.
    pub isRevStation: bool,
    /// The loaded network
    pub net: Ptr<RONet>,
    /// An indicator whether the next edge shall be read (internal edges are not read by now)
    pub myProcess: bool,
}

impl std::ops::Deref for AGActivityGenHandler {
    type Target = Unmapped /* SUMOSAXHandler */;
    fn deref(&self) -> &Unmapped /* SUMOSAXHandler */ {
        &self.base
    }
}
impl std::ops::DerefMut for AGActivityGenHandler {
    fn deref_mut(&mut self) -> &mut Unmapped /* SUMOSAXHandler */ {
        &mut self.base
    }
}

impl AGActivityGenHandler {
    /// C++: `void AGActivityGenHandler::AGActivityGenHandler(AGCity &, RONet *)`
    /// Constructor
    // AGActivityGenHandler::AGActivityGenHandler(AGCity& city, RONet* net)
    //     : SUMOSAXHandler("sumo-stat"),
    //       myCity(city), net(net) {}
    #[allow(unused_variables)]
    pub fn new(city: &mut AGCity, net: Ptr<RONet>) -> Self {
        todo!("port AGActivityGenHandler::AGActivityGenHandler")
    }

    /// C++: `void AGActivityGenHandler::~AGActivityGenHandler() noexcept` [virtual]
    /// Destructor
    // AGActivityGenHandler::~AGActivityGenHandler() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port AGActivityGenHandler::~AGActivityGenHandler")
    }

    /// C++: `void AGActivityGenHandler::myStartElement(int, const SUMOSAXAttributes &)` [virtual, protected]
    /// Called on the opening of a tag;
    // void
    // AGActivityGenHandler::myStartElement(int element, const SUMOSAXAttributes& attrs) {
    //     try {
    //         switch (element) {
    //             case AGEN_TAG_GENERAL:
    //                 parseGeneralCityInfo(attrs);
    //                 break;
    //             case AGEN_TAG_STREET:
    //                 parseStreets(attrs);
    //                 break;
    //             case AGEN_TAG_WORKHOURS:
    //                 parseWorkHours();
    //                 break;
    //             case AGEN_TAG_OPENING:
    //                 parseOpeningHour(attrs);
    //                 break;
    //             case AGEN_TAG_CLOSING:
    //                 parseClosingHour(attrs);
    //                 break;
    //             case AGEN_TAG_SCHOOLS:
    //                 parseSchools();
    //                 break;
    //             case AGEN_TAG_SCHOOL:
    //                 parseSchool(attrs);
    //                 break;
    //             case AGEN_TAG_BUSSTATION:
    //                 parseBusStation(attrs);
    //                 break;
    //             case AGEN_TAG_BUSLINE:
    //                 parseBusLine(attrs);
    //                 break;
    //             case AGEN_TAG_STATIONS:
    //                 parseStations();
    //                 break;
    //             case AGEN_TAG_REV_STATIONS:
    //                 parseRevStations();
    //                 break;
    //             case AGEN_TAG_STATION:
    //                 parseStation(attrs);
    //                 break;
    //             case AGEN_TAG_FREQUENCY:
    //                 parseFrequency(attrs);
    //                 break;
    //             case AGEN_TAG_POPULATION:
    //                 parsePopulation();
    //                 break;
    //             /*case AGEN_TAG_CHILD_ACOMP:
    //                 parseChildrenAccompaniment();
    //                 break;*/
    //             case AGEN_TAG_BRACKET:
    //                 parseBracket(attrs);
    //                 break;
    //             case AGEN_TAG_PARAM:
    //                 parseParameters(attrs);
    //                 break;
    //             case AGEN_TAG_ENTRANCE:
    //                 parseCityGates(attrs);
    //                 break;
    //             default:
    //                 break;
    //         }
    //     } catch (const std::exception& e) {
    //         throw ProcessError(e.what());
    //     }
    // }
    #[allow(unused_variables)]
    pub fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) {
        todo!("port AGActivityGenHandler::myStartElement")
    }

    /// C++: `void AGActivityGenHandler::parseGeneralCityInfo(const SUMOSAXAttributes &)` [protected]
    /// Parses attributes from an "general"-element in city file.
    // void
    // AGActivityGenHandler::parseGeneralCityInfo(const SUMOSAXAttributes& attrs) {
    //     try {
    //         bool ok;
    //         myCity.statData.inhabitants = attrs.getInt(AGEN_ATTR_INHABITANTS);
    //         myCity.statData.households = attrs.getInt(AGEN_ATTR_HOUSEHOLDS);
    //         myCity.statData.limitAgeChildren = attrs.getOpt<int>(AGEN_ATTR_CHILDREN, nullptr, ok, 18);
    //         myCity.statData.limitAgeRetirement = attrs.getOpt<int>(AGEN_ATTR_RETIREMENT, nullptr, ok, 63);
    //         myCity.statData.carRate = attrs.getOpt<double>(AGEN_ATTR_CARS, nullptr, ok, 0.58);
    //         myCity.statData.unemployement = attrs.getOpt<double>(AGEN_ATTR_UNEMPLOYEMENT, nullptr, ok, 0.06);
    //         myCity.statData.laborDemand = attrs.getOpt<double>(AGEN_ATTR_LABORDEMAND, nullptr, ok, 1.05);
    //         myCity.statData.maxFootDistance = attrs.getOpt<double>(AGEN_ATTR_MAX_FOOT_DIST, nullptr, ok, 300.0);
    //         myCity.statData.incomingTraffic = attrs.getOpt<int>(AGEN_ATTR_IN_TRAFFIC, nullptr, ok, 0);
    //         myCity.statData.outgoingTraffic = attrs.getOpt<int>(AGEN_ATTR_OUT_TRAFFIC, nullptr, ok, 0);
    //     } catch (const std::exception& e) {
    //         WRITE_ERROR("Error while parsing the element " +
    //                     SUMOXMLDefinitions::Tags.getString(AGEN_TAG_GENERAL) + ": " +
    //                     e.what());
    //         throw ProcessError();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_general_city_info(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port AGActivityGenHandler::parseGeneralCityInfo")
    }

    /// C++: `void AGActivityGenHandler::parseParameters(const SUMOSAXAttributes &)` [protected]
    /// parse parameters, all are optional, default values are given.
    // void
    // AGActivityGenHandler::parseParameters(const SUMOSAXAttributes& attrs) {
    //     try {
    //         bool ok;
    //         myCity.statData.carPreference = attrs.getOpt<double>(AGEN_ATTR_CARPREF, nullptr, ok, 0.0);
    //         myCity.statData.speedTimePerKm = attrs.getOpt<double>(AGEN_ATTR_CITYSPEED, nullptr, ok, 360.0);
    //         myCity.statData.freeTimeActivityRate = attrs.getOpt<double>(AGEN_ATTR_FREETIMERATE, nullptr, ok, 0.15);
    //         myCity.statData.uniformRandomTrafficRate = attrs.getOpt<double>(AGEN_ATTR_UNI_RAND_TRAFFIC, nullptr, ok, 0.0);
    //         myCity.statData.departureVariation = attrs.getOpt<double>(AGEN_ATTR_DEP_VARIATION, nullptr, ok, 0.0);
    //     } catch (const std::exception& e) {
    //         WRITE_ERROR("Error while parsing the element " +
    //                     SUMOXMLDefinitions::Tags.getString(AGEN_TAG_PARAM) + ": " +
    //                     e.what());
    //         throw ProcessError();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_parameters(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port AGActivityGenHandler::parseParameters")
    }

    /// C++: `void AGActivityGenHandler::parseStreets(const SUMOSAXAttributes &)` [protected]
    /// streets are extracted: it fills the streets list of the City object an additional computation is needed to normalize the number of inhabitants (and workpositions) in each street
    // void
    // AGActivityGenHandler::parseStreets(const SUMOSAXAttributes& attrs) {
    //     try {
    //         double pop = 0;
    //         double work = 0;
    //
    //         std::string eid = attrs.getString(SUMO_ATTR_EDGE);
    //         if (attrs.hasAttribute(AGEN_ATTR_POPULATION)) {
    //             pop = attrs.getFloat(AGEN_ATTR_POPULATION);
    //             if (std::isnan(pop)) {
    //                 pop = 0;
    //                 WRITE_WARNINGF(TL("Invalid % value of edge % is treated as zero."), SUMOXMLDefinitions::Attrs.getString(AGEN_ATTR_POPULATION), eid);
    //             }
    //         }
    //         if (attrs.hasAttribute(AGEN_ATTR_OUT_WORKPOSITION)) {
    //             work = attrs.getFloat(AGEN_ATTR_OUT_WORKPOSITION);
    //             if (std::isnan(work)) {
    //                 work = 0;
    //                 WRITE_WARNINGF(TL("Invalid % value of edge % is treated as zero."), SUMOXMLDefinitions::Attrs.getString(AGEN_ATTR_OUT_WORKPOSITION), eid);
    //             }
    //         }
    //         AGStreet* street = dynamic_cast<AGStreet*>(net->getEdge(eid));
    //         if (street == nullptr) {
    //             WRITE_ERRORF(TL("Edge '%' is not known."), eid);
    //             return;
    //         }
    //         street->setPopulation(pop * street->getLength());
    //         street->setWorkplaceNumber(work * street->getLength());
    //         myCity.streets.push_back(street);
    //     } catch (const std::exception& e) {
    //         WRITE_ERROR("Error while parsing the element " +
    //                     SUMOXMLDefinitions::Tags.getString(AGEN_TAG_STREET) + ": " +
    //                     e.what());
    //         throw ProcessError();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_streets(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port AGActivityGenHandler::parseStreets")
    }

    /// C++: `void AGActivityGenHandler::parseCityGates(const SUMOSAXAttributes &)` [protected]
    /// parse city entrances: the gates of the city, used for incoming and outgoing traffic
    // void
    // AGActivityGenHandler::parseCityGates(const SUMOSAXAttributes& attrs) {
    //     try {
    //         std::string edge = attrs.getString(SUMO_ATTR_EDGE);
    //         double positionOnEdge = attrs.getFloat(SUMO_ATTR_POSITION);
    //         AGPosition posi(myCity.getStreet(edge), positionOnEdge);
    //         myCity.statData.incoming[(int)myCity.cityGates.size()] = attrs.getFloat(AGEN_ATTR_INCOMING);
    //         myCity.statData.outgoing[(int)myCity.cityGates.size()] = attrs.getFloat(AGEN_ATTR_OUTGOING);
    //         myCity.cityGates.push_back(posi);
    //
    //     } catch (const std::exception& e) {
    //         WRITE_ERROR("Error while parsing the element " +
    //                     SUMOXMLDefinitions::Tags.getString(AGEN_TAG_CITYGATES) + ": " +
    //                     e.what());
    //         throw ProcessError();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_city_gates(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port AGActivityGenHandler::parseCityGates")
    }

    /// C++: `void AGActivityGenHandler::parseWorkHours()` [protected]
    /// extraction of work opening and closing hours
    // void
    // AGActivityGenHandler::parseWorkHours() {
    //     myCurrentObject = "workHours";
    // }
    #[allow(unused_variables)]
    pub fn parse_work_hours(&mut self) {
        todo!("port AGActivityGenHandler::parseWorkHours")
    }

    /// C++: `void AGActivityGenHandler::parseOpeningHour(const SUMOSAXAttributes &)` [protected]
    // void
    // AGActivityGenHandler::parseOpeningHour(const SUMOSAXAttributes& attrs) {
    //     if (myCurrentObject == "workHours") {
    //         try {
    //             myCity.statData.beginWorkHours[attrs.getInt(AGEN_ATTR_HOUR)] = attrs.getFloat(AGEN_ATTR_PROP);
    //
    //         } catch (const std::exception& e) {
    //             WRITE_ERROR("Error while parsing the element " +
    //                         SUMOXMLDefinitions::Tags.getString(AGEN_TAG_OPENING) + ": "
    //                         + e.what());
    //             throw ProcessError();
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_opening_hour(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port AGActivityGenHandler::parseOpeningHour")
    }

    /// C++: `void AGActivityGenHandler::parseClosingHour(const SUMOSAXAttributes &)` [protected]
    // void
    // AGActivityGenHandler::parseClosingHour(const SUMOSAXAttributes& attrs) {
    //     if (myCurrentObject == "workHours") {
    //         try {
    //             myCity.statData.endWorkHours[attrs.getInt(AGEN_ATTR_HOUR)] = attrs.getFloat(AGEN_ATTR_PROP);
    //
    //         } catch (const std::exception& e) {
    //             WRITE_ERROR("Error while parsing the element " +
    //                         SUMOXMLDefinitions::Tags.getString(AGEN_TAG_CLOSING) + ": "
    //                         + e.what());
    //             throw ProcessError();
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_closing_hour(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port AGActivityGenHandler::parseClosingHour")
    }

    /// C++: `void AGActivityGenHandler::parseSchools()` [protected]
    /// School extraction
    // void
    // AGActivityGenHandler::parseSchools() {
    //     myCurrentObject = "schools";
    // }
    #[allow(unused_variables)]
    pub fn parse_schools(&mut self) {
        todo!("port AGActivityGenHandler::parseSchools")
    }

    /// C++: `void AGActivityGenHandler::parseSchool(const SUMOSAXAttributes &)` [protected]
    // void
    // AGActivityGenHandler::parseSchool(const SUMOSAXAttributes& attrs) {
    //     try {
    //         bool ok = true;
    //         std::string edge = attrs.getString(SUMO_ATTR_EDGE, &ok);
    //         if (!ok) {
    //             throw ProcessError("Cannot parse edge attribute");
    //         }
    //         double positionOnEdge = attrs.hasAttribute(SUMO_ATTR_POSITION) ? attrs.getFloat(SUMO_ATTR_POSITION) : 0.;
    //         AGPosition posi(myCity.getStreet(edge), positionOnEdge);
    //         int beginAge = attrs.getInt(AGEN_ATTR_BEGINAGE);
    //         int endAge = attrs.getInt(AGEN_ATTR_ENDAGE);
    //         int capacity = attrs.getInt(AGEN_ATTR_CAPACITY);
    //         int openingHour = attrs.getInt(AGEN_ATTR_OPENING);
    //         int closingHour = attrs.getInt(AGEN_ATTR_CLOSING);
    //         AGSchool sch(capacity, posi, beginAge, endAge, openingHour, closingHour);
    //         myCity.schools.push_back(sch);
    //
    //     } catch (const std::exception& e) {
    //         WRITE_ERROR("Error while parsing the element " +
    //                     SUMOXMLDefinitions::Tags.getString(AGEN_TAG_SCHOOL) + ": " +
    //                     e.what());
    //         throw ProcessError();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_school(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port AGActivityGenHandler::parseSchool")
    }

    /// C++: `void AGActivityGenHandler::parseBusStation(const SUMOSAXAttributes &)` [protected]
    /// Bus stations and Bus extraction an additional computation have to be done to create buses' names and buses in the return way
    // void
    // AGActivityGenHandler::parseBusStation(const SUMOSAXAttributes& attrs) {
    //     try {
    //         bool ok = true;
    //         std::string edge = attrs.getString(SUMO_ATTR_EDGE, &ok);
    //         if (!ok) {
    //             throw ProcessError("Cannot parse edge attribute");
    //         };
    //         double positionOnEdge = attrs.hasAttribute(SUMO_ATTR_POSITION) ? attrs.getFloat(SUMO_ATTR_POSITION) : 0.;
    //         int id = attrs.getInt(SUMO_ATTR_ID);
    //         AGPosition posi(myCity.getStreet(edge), positionOnEdge);
    //         myCity.statData.busStations.insert(std::pair<int, AGPosition>(id, posi));
    //
    //     } catch (const std::exception& e) {
    //         WRITE_ERROR("Error while parsing the element " +
    //                     SUMOXMLDefinitions::Tags.getString(AGEN_TAG_BUSSTATION) + ": " +
    //                     e.what());
    //         throw ProcessError();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_bus_station(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port AGActivityGenHandler::parseBusStation")
    }

    /// C++: `void AGActivityGenHandler::parseBusLine(const SUMOSAXAttributes &)` [protected]
    // void
    // AGActivityGenHandler::parseBusLine(const SUMOSAXAttributes& attrs) {
    //     try {
    //         myCurrentObject = "busLine";
    //         AGBusLine busL(attrs.getString(SUMO_ATTR_ID));
    //         busL.setMaxTripTime(attrs.getInt(AGEN_ATTR_MAX_TRIP_DURATION));
    //         myCity.busLines.push_front(busL);
    //         currentBusLine = &*myCity.busLines.begin();
    //
    //     } catch (const std::exception& e) {
    //         WRITE_ERROR("Error while parsing the element " +
    //                     SUMOXMLDefinitions::Tags.getString(AGEN_TAG_BUSLINE) + ": " +
    //                     e.what());
    //         throw ProcessError();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_bus_line(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port AGActivityGenHandler::parseBusLine")
    }

    /// C++: `void AGActivityGenHandler::parseStations()` [protected]
    // void
    // AGActivityGenHandler::parseStations() {
    //     isRevStation = false;
    // }
    #[allow(unused_variables)]
    pub fn parse_stations(&mut self) {
        todo!("port AGActivityGenHandler::parseStations")
    }

    /// C++: `void AGActivityGenHandler::parseRevStations()` [protected]
    // void
    // AGActivityGenHandler::parseRevStations() {
    //     isRevStation = true;
    // }
    #[allow(unused_variables)]
    pub fn parse_rev_stations(&mut self) {
        todo!("port AGActivityGenHandler::parseRevStations")
    }

    /// C++: `void AGActivityGenHandler::parseStation(const SUMOSAXAttributes &)` [protected]
    // void
    // AGActivityGenHandler::parseStation(const SUMOSAXAttributes& attrs) {
    //     if (myCurrentObject != "busLine") {
    //         return;
    //     }
    //
    //     try {
    //         bool ok = true;
    //         int refID = attrs.get<int>(SUMO_ATTR_REFID, myCurrentObject.c_str(), ok);
    //         if (!ok) {
    //             throw ProcessError();
    //         }
    //         if (myCity.statData.busStations.count(refID) == 0) {
    //             throw ProcessError(TLF("Unknown bus station '%'.", refID));
    //         }
    //         if (!isRevStation) {
    //             currentBusLine->locateStation(myCity.statData.busStations.find(refID)->second);
    //         } else {
    //             currentBusLine->locateRevStation(myCity.statData.busStations.find(refID)->second);
    //         }
    //
    //     } catch (const std::exception& e) {
    //         WRITE_ERROR("Error while parsing the element " +
    //                     SUMOXMLDefinitions::Tags.getString(AGEN_TAG_STATION) + ": " +
    //                     e.what());
    //         throw ProcessError();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_station(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port AGActivityGenHandler::parseStation")
    }

    /// C++: `void AGActivityGenHandler::parseFrequency(const SUMOSAXAttributes &)` [protected]
    // void
    // AGActivityGenHandler::parseFrequency(const SUMOSAXAttributes& attrs) {
    //     if (myCurrentObject != "busLine") {
    //         return;
    //     }
    //
    //     try {
    //         int beginB = attrs.getInt(SUMO_ATTR_BEGIN);
    //         int endB = attrs.getInt(SUMO_ATTR_END);
    //         int rateB = attrs.getInt(AGEN_ATTR_RATE);
    //         currentBusLine->generateBuses(beginB, endB, rateB);
    //
    //     } catch (const std::exception& e) {
    //         WRITE_ERROR("Error while parsing the element " +
    //                     SUMOXMLDefinitions::Tags.getString(AGEN_TAG_FREQUENCY) + ": " +
    //                     e.what());
    //         throw ProcessError();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_frequency(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port AGActivityGenHandler::parseFrequency")
    }

    /// C++: `void AGActivityGenHandler::parsePopulation()` [protected]
    /// population and children accompaniment bracket extraction then households and all population data will be computable
    // void
    // AGActivityGenHandler::parsePopulation() {
    //     myCurrentObject = "population";
    // }
    #[allow(unused_variables)]
    pub fn parse_population(&mut self) {
        todo!("port AGActivityGenHandler::parsePopulation")
    }

    /// C++: `void AGActivityGenHandler::parseBracket(const SUMOSAXAttributes &)` [protected]
    // void
    // AGActivityGenHandler::parseBracket(const SUMOSAXAttributes& attrs) {
    //     try {
    //
    //         int beginAge = attrs.getInt(AGEN_ATTR_BEGINAGE); //included in the bracket
    //         int endAge = attrs.getInt(AGEN_ATTR_ENDAGE); //NOT included in the bracket
    //         bool overlapping = false;
    //         // evaluate age
    //         if (beginAge < endAge) {
    //             for (auto it = myCity.statData.ageSpan.begin(); it != myCity.statData.ageSpan.end(); ++it) {
    //                 if (!(beginAge >= it->second || endAge <= it->first)) {
    //                     overlapping = true;
    //                 }
    //             }
    //             if (!overlapping) {
    //                 myCity.statData.ageSpan[beginAge] = endAge;
    //
    //                 if (myCurrentObject == "population") {
    //                     myCity.statData.population[endAge] = attrs.getInt(AGEN_ATTR_PEOPLENBR);
    //                 }
    //             } else {
    //                 WRITE_ERROR("Error while parsing the element " +
    //                             SUMOXMLDefinitions::Tags.getString(AGEN_TAG_BRACKET) + ": begin and end age of the population is overlapping");
    //                 throw ProcessError();
    //             }
    //         } else {
    //             WRITE_ERROR("Error while parsing the element " +
    //                         SUMOXMLDefinitions::Tags.getString(AGEN_TAG_BRACKET) + ": begin and end age of the population are not properly set");
    //             throw ProcessError();
    //         }
    //
    //     } catch (const std::exception& e) {
    //         WRITE_ERROR("Error while parsing the element " +
    //                     SUMOXMLDefinitions::Tags.getString(AGEN_TAG_BRACKET) + ": " +
    //                     e.what());
    //         throw ProcessError();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_bracket(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port AGActivityGenHandler::parseBracket")
    }

    /// C++: `void AGActivityGenHandler::AGActivityGenHandler(const AGActivityGenHandler &)` [private]
    /// Invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(src: &AGActivityGenHandler) -> Self {
        todo!("port AGActivityGenHandler::AGActivityGenHandler")
    }

    /// C++: `AGActivityGenHandler & AGActivityGenHandler::operator=(const AGActivityGenHandler &)` [private]
    /// Invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, src: &AGActivityGenHandler) -> &mut AGActivityGenHandler {
        todo!("port AGActivityGenHandler::operator=")
    }

}

