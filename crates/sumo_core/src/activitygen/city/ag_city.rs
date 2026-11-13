//! Port of activitygen/city/AGCity.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `AGCity` (activitygen/city/AGCity.h)
pub struct AGCity {
    pub statData: Ptr<AGDataAndStatistics>,
    pub streets: Vec<Ptr<AGStreet>>,
    pub passengerStreets: Vec<Ptr<AGStreet>>,
    pub workPositions: Vec<AGWorkPosition>,
    pub schools: VecDeque<AGSchool>,
    pub busLines: VecDeque<AGBusLine>,
    pub households: VecDeque<AGHousehold>,
    pub cityGates: Vec<AGPosition>,
    pub peopleIncoming: VecDeque<AGAdult>,
    pub net: Ptr<RONet>,
    /// false until the function completeStreets is called this function completes streets and turn this parameter to true
    pub streetsCompleted: bool,
    pub nbrCars: i32,
}

impl AGCity {
    /// C++: `void AGCity::AGCity(RONet *)`
    // AGCity(RONet* net) :
    //         statData(AGDataAndStatistics::getDataAndStatistics()),
    //         net(net),
    //         streetsCompleted(false) {}
    #[allow(unused_variables)]
    pub fn new(net: Ptr<RONet>) -> Self {
        Self {
            statData: Ptr::from_mut(AGDataAndStatistics::get_data_and_statistics()),
            streets: Vec::new(),
            passengerStreets: Vec::new(),
            workPositions: Vec::new(),
            schools: VecDeque::new(),
            busLines: VecDeque::new(),
            households: VecDeque::new(),
            cityGates: Vec::new(),
            peopleIncoming: VecDeque::new(),
            net,
            streetsCompleted: false,
            nbrCars: 0,
        }
    }

    /// C++: `void AGCity::completeStreets()`
    /// generates streets: complete the "streets" vector using the DataAndStat's map edges.
    // void
    // AGCity::completeStreets() {
    //     if (streetsCompleted) {
    //         return;
    //     } else {
    //         streetsCompleted = true;
    //     }
    //
    //     double pop = 0, work = 0;
    //     std::vector<AGStreet*>::iterator it;
    //
    //     for (it = streets.begin(); it != streets.end(); ++it) {
    //         pop += (*it)->getPopulation();
    //         work += (*it)->getWorkplaceNumber();
    //         if (((*it)->getPermissions() & SVC_PASSENGER) == SVC_PASSENGER) {
    //             passengerStreets.push_back(*it);
    //         }
    //     }
    //     statData.factorInhabitants = (double)statData.inhabitants / pop;
    //     //can be improved with other input data
    //     double neededWorkPositionsInCity = (1.0 - statData.unemployement)
    //                                        * ((double)statData.getPeopleYoungerThan(statData.limitAgeRetirement)
    //                                           - (double)statData.getPeopleYoungerThan(statData.limitAgeChildren))
    //                                        + (double)statData.incomingTraffic;
    //     // by default laborDemand = 1.05. We generate 5% more work positions that really needed to avoid any expensive research of random work positions
    //     neededWorkPositionsInCity *= statData.laborDemand;
    //     statData.workPositions = (int)neededWorkPositionsInCity;
    //     statData.factorWorkPositions = neededWorkPositionsInCity / (double) work;
    //
    //     for (it = streets.begin(); it != streets.end(); ++it) {
    //         (*it)->setPopulation((*it)->getPopulation() * statData.factorInhabitants);
    //         (*it)->setWorkplaceNumber((*it)->getWorkplaceNumber() * statData.factorWorkPositions);
    //     }
    //
    //     //completing streets from edges of the network not handled/present in STAT file (no population no work position)
    //     for (const auto& itE : net->getEdgeMap()) {
    //         std::vector<AGStreet*>::iterator itS;
    //         for (itS = streets.begin(); itS != streets.end(); ++itS) {
    //             if (*itS == itE.second) {
    //                 break;
    //             }
    //         }
    //         //if this edge isn't represented by a street
    //         if (itS == streets.end() && !itE.second->isInternal()) {
    //             streets.push_back(static_cast<AGStreet*>(itE.second));
    //             if (((itE.second)->getPermissions() & SVC_PASSENGER) == SVC_PASSENGER) {
    //                 passengerStreets.push_back(static_cast<AGStreet*>(itE.second));
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn complete_streets(&mut self) {
        if self.streetsCompleted {
            return;
        } else {
            self.streetsCompleted = true;
        }

        let mut pop = 0.0;
        let mut work = 0.0;

        for it in self.streets.iter() {
            pop += it.get_population();
            work += it.get_workplace_number();
            if it.allows(SVC_PASSENGER) {
                self.passengerStreets.push(*it);
            }
        }
        let sd = self.statData;
        sd.m().factorInhabitants = sd.m().inhabitants as f64 / pop;
        //can be improved with other input data
        let mut neededWorkPositionsInCity = (1.0 - sd.m().unemployement)
            * ((sd.m().get_people_younger_than(sd.m().limitAgeRetirement) as f64)
                - (sd.m().get_people_younger_than(sd.m().limitAgeChildren) as f64))
            + (sd.m().incomingTraffic as f64);
        // by default laborDemand = 1.05. We generate 5% more work positions that really needed to avoid any expensive research of random work positions
        neededWorkPositionsInCity *= sd.m().laborDemand;
        sd.m().workPositions = neededWorkPositionsInCity as i32;
        sd.m().factorWorkPositions = neededWorkPositionsInCity / work;

        for it in self.streets.iter() {
            let f = sd.m().factorInhabitants;
            let p = it.get_population() * f;
            it.m().set_population(p);
            let w = it.get_workplace_number() * sd.m().factorWorkPositions;
            it.m().set_workplace_number(w);
        }

        //completing streets from edges of the network not handled/present in STAT file (no population no work position)
        let net = self.net;
        for (_, itE) in net.get_edge_map().iter() {
            let found = self.streets.iter().any(|s| *s == itE.cast::<AGStreet>());
            //if this edge isn't represented by a street
            if !found && !itE.is_internal() {
                let street = itE.cast::<AGStreet>();
                self.streets.push(street);
                if (itE.get_permissions() & SVC_PASSENGER) == SVC_PASSENGER {
                    self.passengerStreets.push(street);
                }
            }
        }
    }

    /// C++: `void AGCity::generateWorkPositions()`
    // void
    // AGCity::generateWorkPositions() {
    //     std::vector<AGStreet*>::iterator it;
    //     int workPositionCounter = 0;
    //     try {
    //         for (it = streets.begin(); it != streets.end(); ++it) {
    //             for (int i = 0; i < (*it)->getWorkplaceNumber(); ++i) {
    //                 workPositions.push_back(AGWorkPosition(&statData, **it));
    //                 ++workPositionCounter;
    //             }
    //         }
    //     } catch (const std::bad_alloc&) {
    //         std::cout << "Number of work positions at bad_alloc exception: " << workPositionCounter << std::endl;
    //         throw;
    //     }
    //     //std::cout << "Inner work positions done. " << workPositionCounter << " generated." << std::endl;
    //
    //     // Work positions outside the city
    //     generateOutgoingWP();
    //     std::cout << "--> work position:" << std::endl;
    //     std::cout << "  |-> in city: " << workPositionCounter << std::endl;
    //     std::cout << "  |-> out city: " << statData.workPositions - workPositionCounter << std::endl;
    //     std::cout << "  |-> in+out city: " << statData.workPositions << std::endl;
    // }
    #[allow(unused_variables)]
    pub fn generate_work_positions(&mut self) {
        let mut workPositionCounter = 0;
        // shortcut: C++ try/catch(std::bad_alloc) that prints the counter and rethrows is not mirrored; Rust aborts on OOM
        for s in 0..self.streets.len() {
            let street = self.streets[s];
            let mut i: i32 = 0;
            while (i as f64) < street.get_workplace_number() {
                self.workPositions.push(AGWorkPosition::new(self.statData, &*street));
                workPositionCounter += 1;
                i += 1;
            }
        }
        //std::cout << "Inner work positions done. " << workPositionCounter << " generated." << std::endl;

        // Work positions outside the city
        self.generate_outgoing_wp();
        let sd = self.statData;
        println!("--> work position:");
        println!("  |-> in city: {}", workPositionCounter);
        println!("  |-> out city: {}", sd.m().workPositions - workPositionCounter);
        println!("  |-> in+out city: {}", sd.m().workPositions);
    }

    /// C++: `void AGCity::completeBusLines()`
    // void
    // AGCity::completeBusLines() {
    //     std::list<AGBusLine>::iterator it;
    //     for (it = busLines.begin(); it != busLines.end(); ++it) {
    //         //it->generateOpositDirection();
    //         it->setBusNames();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn complete_bus_lines(&mut self) {
        for it in self.busLines.iter_mut() {
            //it->generateOpositDirection();
            it.set_bus_names();
        }
    }

    /// C++: `void AGCity::generatePopulation()`
    // void
    // AGCity::generatePopulation() {
    //     std::vector<AGStreet*>::iterator it;
    //     double people = 0;
    //     nbrCars = 0;
    //     int idHouseholds = 0;
    //     std::vector<int> numAdults(statData.households);
    //     std::vector<int> numChilds(statData.households);
    //     int totalChildrenLeft = statData.inhabitants - statData.getPeopleOlderThan(statData.limitAgeChildren);
    //     const double retiredProb = statData.getPeopleOlderThan(statData.limitAgeRetirement) / statData.getPeopleOlderThan(statData.limitAgeChildren);
    //     for (int i = 0; i < statData.households; i++) {
    //         numAdults[i] = 1;
    //         numChilds[i] = 0;
    //         if (RandHelper::rand() < retiredProb) {
    //             numAdults[i] = -numAdults[i];
    //         } else if (totalChildrenLeft > 0) {
    //             numChilds[i] = statData.getPoissonsNumberOfChildren(statData.meanNbrChildren);
    //             totalChildrenLeft -= numChilds[i];
    //         }
    //     }
    //     //compensate with adults for too many / missing children
    //     const int numSecondPers = statData.getPeopleOlderThan(statData.limitAgeChildren) - statData.households + totalChildrenLeft;
    //     for (int i = 0; i < numSecondPers; i++) {
    //         int index = i % numAdults.size();
    //         if (numAdults[index] >= 0) {
    //             numAdults[index] += 1;
    //         } else {
    //             numAdults[index] -= 1;
    //         }
    //     }
    //     for (it = streets.begin(); it != streets.end(); ++it) {
    //         people += (*it)->getPopulation();
    //         while (people > 0 && idHouseholds < (int)numAdults.size()) {
    //             int i = RandHelper::rand((int)numAdults.size() - idHouseholds);
    //             ++idHouseholds;
    //             households.push_back(AGHousehold(*it, this, idHouseholds));
    //             households.back().generatePeople(abs(numAdults[i]), numChilds[i], numAdults[i] < 0); //&statData
    //             //households.back().generateCars(statData.carRate);
    //             people -= households.back().getPeopleNbr();
    //             numAdults[i] = numAdults[numAdults.size() - idHouseholds];
    //             numChilds[i] = numChilds[numAdults.size() - idHouseholds];
    //         }
    //     }
    //
    //     //people from outside of the city generation:
    //     generateIncomingPopulation();
    //
    //     //TEST
    //     int nbrSingle = 0;
    //     int nbrCouple = 0;
    //     int nbrChild = 0;
    //     int nbrHH = 0;
    //     std::list<AGHousehold>::iterator itt;
    //     for (itt = households.begin(); itt != households.end(); ++itt) {
    //         if (itt->getAdultNbr() == 1) {
    //             nbrSingle++;
    //         }
    //         if (itt->getAdultNbr() == 2) {
    //             nbrCouple += 2;
    //         }
    //         nbrChild += itt->getPeopleNbr() - itt->getAdultNbr();
    //         nbrHH++;
    //     }
    //     //cout << "number hh: " << nbrHH << std::endl;
    //     //cout << "number single: " << nbrSingle << std::endl;
    //     //cout << "number couple: " << nbrCouple << std::endl;
    //     //cout << "number 3 or more: " << nbr3More << std::endl;
    //     //cout << "number adults: " << nbrSingle + nbrCouple + nbr3More << std::endl;
    //     //cout << "number children: " << nbrChild << std::endl;
    //     //cout << "number people: " << nbrSingle + nbrCouple + nbr3More + nbrChild << std::endl;
    //     //END TEST
    //
    //     std::cout << "--> population:" << std::endl;
    //     std::cout << "  |-> city households: " << nbrHH << std::endl;
    //     std::cout << "  |-> city people: " << nbrSingle + nbrCouple + nbrChild << std::endl;
    //     std::cout << "    |-> city single: " << nbrSingle << " / (in) couple: " << nbrCouple << std::endl;
    //     std::cout << "    |-> city adults: " << nbrSingle + nbrCouple << std::endl;
    //     std::cout << "      |-> estimation: " << statData.getPeopleOlderThan(statData.limitAgeChildren) << std::endl;
    //     std::cout << "      |-> retired: " << statData.getPeopleOlderThan(statData.limitAgeRetirement) << std::endl;
    //     std::cout << "    |-> city children: " << nbrChild << std::endl;
    //     std::cout << "      |-> estimation: " << statData.getPeopleYoungerThan(statData.limitAgeChildren) << std::endl;
    //
    // }
    #[allow(unused_variables)]
    pub fn generate_population(&mut self) {
        self.nbrCars = 0;
        let mut idHouseholds: i32 = 0;
        let sd = self.statData;
        let hhCount = sd.m().households;
        let mut numAdults: Vec<i32> = vec![0; hhCount as usize];
        let mut numChilds: Vec<i32> = vec![0; hhCount as usize];
        let mut totalChildrenLeft = sd.m().inhabitants - sd.m().get_people_older_than(sd.m().limitAgeChildren);
        // C++ divides two ints here (integer division, then converted to double): kept as is
        let retiredProb = (sd.m().get_people_older_than(sd.m().limitAgeRetirement) / sd.m().get_people_older_than(sd.m().limitAgeChildren)) as f64;
        for i in 0..hhCount {
            let i = i as usize;
            numAdults[i] = 1;
            numChilds[i] = 0;
            if utils::common::rand_helper::rand() < retiredProb {
                numAdults[i] = -numAdults[i];
            } else if totalChildrenLeft > 0 {
                let mean = sd.m().meanNbrChildren;
                numChilds[i] = sd.m().get_poissons_number_of_children(mean);
                totalChildrenLeft -= numChilds[i];
            }
        }
        //compensate with adults for too many / missing children
        let numSecondPers = sd.m().get_people_older_than(sd.m().limitAgeChildren) - sd.m().households + totalChildrenLeft;
        for i in 0..numSecondPers {
            let index = (i as usize) % numAdults.len();
            if numAdults[index] >= 0 {
                numAdults[index] += 1;
            } else {
                numAdults[index] -= 1;
            }
        }
        let mut people = 0.0;
        for s in 0..self.streets.len() {
            let street = self.streets[s];
            people += street.get_population();
            while people > 0.0 && idHouseholds < numAdults.len() as i32 {
                let i = utils::common::rand_helper::with_default(|r| r.rand_int(numAdults.len() as i32 - idHouseholds)) as usize;
                idHouseholds += 1;
                let city = Ptr::from_mut(self);
                self.households.push_back(AGHousehold::new_2(street, city, idHouseholds));
                let adults = numAdults[i].abs();
                let children = numChilds[i];
                let firstRetired = numAdults[i] < 0;
                self.households.back_mut().unwrap().generate_people(adults, children, firstRetired);
                people -= self.households.back_mut().unwrap().get_people_nbr() as f64;
                numAdults[i] = numAdults[numAdults.len() - (idHouseholds as usize)];
                numChilds[i] = numChilds[numAdults.len() - (idHouseholds as usize)];
            }
        }

        //people from outside of the city generation:
        self.generate_incoming_population();

        //TEST
        let mut nbrSingle = 0;
        let mut nbrCouple = 0;
        let mut nbrChild = 0;
        let mut nbrHH = 0;
        for itt in self.households.iter_mut() {
            if itt.get_adult_nbr() == 1 {
                nbrSingle += 1;
            }
            if itt.get_adult_nbr() == 2 {
                nbrCouple += 2;
            }
            nbrChild += itt.get_people_nbr() - itt.get_adult_nbr();
            nbrHH += 1;
        }
        //END TEST

        println!("--> population:");
        println!("  |-> city households: {}", nbrHH);
        println!("  |-> city people: {}", nbrSingle + nbrCouple + nbrChild);
        println!("    |-> city single: {} / (in) couple: {}", nbrSingle, nbrCouple);
        println!("    |-> city adults: {}", nbrSingle + nbrCouple);
        println!("      |-> estimation: {}", sd.m().get_people_older_than(sd.m().limitAgeChildren));
        println!("      |-> retired: {}", sd.m().get_people_older_than(sd.m().limitAgeRetirement));
        println!("    |-> city children: {}", nbrChild);
        println!("      |-> estimation: {}", sd.m().get_people_younger_than(sd.m().limitAgeChildren));
    }

    /// C++: `void AGCity::schoolAllocation()`
    // void
    // AGCity::schoolAllocation() {
    //     std::list<AGHousehold>::iterator it;
    //     bool shortage;
    //     for (it = households.begin(); it != households.end(); ++it) {
    //         shortage = !it->allocateChildrenSchool();
    //         if (shortage) {
    //             /*ofstream fichier("test.txt", ios::app);  // ouverture en écriture avec effacement du fichier ouvert
    //             if(fichier)
    //             {
    //               fichier << "===> WARNING: Not enough school places in the city for all children..." << std::endl;
    //               fichier.close();
    //             }
    //             else
    //               cerr << "Impossible d'ouvrir le fichier !" << std::endl;*/
    //
    //             //std::cout << "===> WARNING: Not enough school places in the city for all children..." << std::endl;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn school_allocation(&mut self) {
        for it in self.households.iter_mut() {
            // C++ shortage warning is commented out upstream, so the result is unused
            let _shortage = !it.allocate_children_school();
        }
    }

    /// C++: `void AGCity::workAllocation()`
    // void
    // AGCity::workAllocation() {
    //     const bool debug = OptionsCont::getOptions().getBool("debug");
    //     statData.AdultNbr = 0;
    //     //end tests
    //     /**
    //      * people from the city
    //      */
    //     std::list<AGHousehold>::iterator it;
    //     bool shortage;
    //
    //     if (debug) {
    //         std::cout << "\n";
    //     }
    //
    //     for (it = households.begin(); it != households.end(); ++it) {
    //         if (it->retiredHouseholders()) {
    //             continue;
    //         }
    //         shortage = !it->allocateAdultsWork();
    //         if (shortage) {
    //             std::cout << "===> ERROR: Not enough work positions in the city for all working people..." << std::endl;
    //         }
    //         statData.AdultNbr += it->getAdultNbr(); //TESTING
    //         if (debug) {
    //             std::cout << " processed " << statData.AdultNbr << " adults\r";
    //         }
    //     }
    //
    //     /**
    //      * people from outside
    //      */
    //     std::list<AGAdult>::iterator itA;
    //     for (itA = peopleIncoming.begin(); itA != peopleIncoming.end(); ++itA) {
    //         if (statData.workPositions > 0) {
    //             itA->tryToWork(1, &workPositions);
    //         } else {
    //             //shouldn't happen
    //             std::cout << "not enough work for incoming people..." << std::endl;
    //         }
    //     }
    //
    //     //BEGIN TESTS
    //     int workingP = 0;
    //     std::list<AGHousehold>::iterator itt;
    //     for (itt = households.begin(); itt != households.end(); ++itt) {
    //         if (itt->getAdultNbr() == 1) {
    //             if (itt->getAdults().front().isWorking()) {
    //                 workingP++;
    //             }
    //         }
    //         if (itt->getAdultNbr() == 2) {
    //             if (itt->getAdults().front().isWorking()) {
    //                 workingP++;
    //             }
    //             if (itt->getAdults().back().isWorking()) {
    //                 workingP++;
    //             }
    //         }
    //     }
    //     std::cout << "  |-> working people: " << peopleIncoming.size() + workingP << std::endl;
    //     std::cout << "    |-> working people in city: " << workingP << std::endl;
    //     std::cout << "    |-> working people from outside: " << peopleIncoming.size() << std::endl;
    //     //END TESTS
    // }
    #[allow(unused_variables)]
    pub fn work_allocation(&mut self) {
        let debug = utils::options::options_cont::get_bool("debug");
        let sd = self.statData;
        sd.m().AdultNbr = 0;
        //end tests
        /**
         * people from the city
         */
        if debug {
            println!();
        }

        for it in self.households.iter_mut() {
            if it.retired_householders() {
                continue;
            }
            let shortage = !it.allocate_adults_work();
            if shortage {
                println!("===> ERROR: Not enough work positions in the city for all working people...");
            }
            sd.m().AdultNbr += it.get_adult_nbr(); //TESTING
            if debug {
                print!(" processed {} adults\r", sd.m().AdultNbr);
            }
        }

        /**
         * people from outside
         */
        for itA in self.peopleIncoming.iter_mut() {
            if sd.m().workPositions > 0 {
                itA.try_to_work(1.0, Ptr::from_mut(&mut self.workPositions));
            } else {
                //shouldn't happen
                println!("not enough work for incoming people...");
            }
        }

        //BEGIN TESTS
        let mut workingP = 0;
        for itt in self.households.iter_mut() {
            if itt.get_adult_nbr() == 1 {
                if itt.get_adults().front().unwrap().is_working() {
                    workingP += 1;
                }
            }
            if itt.get_adult_nbr() == 2 {
                if itt.get_adults().front().unwrap().is_working() {
                    workingP += 1;
                }
                if itt.get_adults().back().unwrap().is_working() {
                    workingP += 1;
                }
            }
        }
        println!("  |-> working people: {}", self.peopleIncoming.len() as i32 + workingP);
        println!("    |-> working people in city: {}", workingP);
        println!("    |-> working people from outside: {}", self.peopleIncoming.len());
        //END TESTS
    }

    /// C++: `void AGCity::carAllocation()`
    // void
    // AGCity::carAllocation() {
    //     statData.hhFarFromPT = 0;
    //     nbrCars = 0;
    //     std::list<AGHousehold>::iterator it;
    //     for (it = households.begin(); it != households.end(); ++it) {
    //         if (!it->isCloseFromPubTransport(&(statData.busStations))) {
    //             statData.hhFarFromPT++;
    //             nbrCars++;
    //             it->addACar();
    //         }
    //         statData.householdsNbr++;
    //     }
    //     // new rate: the rate on the people that have'nt any car yet:
    //     // nR = (R * Drivers - AlreadyCars) / (Drivers - AlreadyCars)
    //     double newRate = statData.carRate * statData.getPeopleOlderThan(statData.limitAgeChildren) - statData.hhFarFromPT;
    //     if (statData.getPeopleOlderThan(statData.limitAgeChildren) == statData.hhFarFromPT) {
    //         newRate = 0.;
    //     } else {
    //         newRate /= statData.getPeopleOlderThan(statData.limitAgeChildren) - statData.hhFarFromPT;
    //     }
    //     //std::cout << " - " << newRate << std::endl;
    //     if (newRate < 0 || newRate >= 1) {
    //         newRate = 0;
    //     }
    //
    //     nbrCars = 0;
    //     //int nbrAdults = 0;
    //     for (it = households.begin(); it != households.end(); ++it) {
    //         it->generateCars(newRate);
    //         nbrCars += it->getCarNbr();
    //         //nbrAdults += it->getAdultNbr();
    //     }
    //     //TEST RESULTS
    //     //std::cout << "number of cars: " << nbrCars << std::endl;
    //     //std::cout << "number of adults: " << statData.getPeopleOlderThan(statData.limitAgeChildren) << std::endl;
    //     //std::cout << "real number of adults: " << nbrAdults << std::endl;
    //     //std::cout << "number of people far from public transport: " << statData.hhFarFromPT << std::endl;
    //     //std::cout << "original rate: " << setprecision(4) << statData.carRate << std::endl;
    //     //std::cout << "new rate: " << setprecision(4) << newRate << std::endl;
    //     //std::cout << "real rate: " << setprecision(4) << (double)nbrCars / (double)statData.getPeopleOlderThan(statData.limitAgeChildren) << std::endl;
    //     //END TEST RESULTS
    // }
    #[allow(unused_variables)]
    pub fn car_allocation(&mut self) {
        let sd = self.statData;
        sd.m().hhFarFromPT = 0;
        self.nbrCars = 0;
        // shortcut: the std::map overload of isCloseFromPubTransport is not ported; the bus stations are copied in map order and the list overload is used
        let mut busStations: VecDeque<AGPosition> = sd.m().busStations.values().copied().collect();
        let busStationsPtr = Ptr::from_mut(&mut busStations);
        for it in self.households.iter_mut() {
            if !it.is_close_from_pub_transport(busStationsPtr) {
                sd.m().hhFarFromPT += 1;
                self.nbrCars += 1;
                it.add_a_car();
            }
            sd.m().householdsNbr += 1;
        }
        // new rate: the rate on the people that have'nt any car yet:
        // nR = (R * Drivers - AlreadyCars) / (Drivers - AlreadyCars)
        let older = sd.m().get_people_older_than(sd.m().limitAgeChildren);
        let mut newRate = sd.m().carRate * (older as f64) - (sd.m().hhFarFromPT as f64);
        if older == sd.m().hhFarFromPT {
            newRate = 0.0;
        } else {
            newRate /= (older - sd.m().hhFarFromPT) as f64;
        }
        if newRate < 0.0 || newRate >= 1.0 {
            newRate = 0.0;
        }

        self.nbrCars = 0;
        for it in self.households.iter_mut() {
            it.generate_cars(newRate);
            self.nbrCars += it.get_car_nbr();
        }
    }

    /// C++: `const AGStreet & AGCity::getStreet(const std::string &)`
    /// manipulation functions
    // const AGStreet&
    // AGCity::getStreet(const std::string& edge) {
    //     /**
    //      * verify if it is the first time this function is called
    //      * in this case, we have to complete the streets with the
    //      * network edges this means that streets are completely
    //      * loaded (no any more to be read from stat-file)
    //      */
    //     if (!streetsCompleted) {
    //         statData.consolidateStat();
    //         completeStreets();
    //         std::cout << "first completed in getStreet() of City: Consolidation of data not needed in ActivityGen any more" << std::endl;
    //     }
    //     //rest of the function
    //     std::vector<AGStreet*>::iterator it = streets.begin();
    //     while (it != streets.end()) {
    //         if ((*it)->getID() == edge) {
    //             return **it;
    //         }
    //         ++it;
    //     }
    //     std::cout << "===> ERROR: WRONG STREET EDGE (" << edge << ") given and not found in street set." << std::endl;
    //     throw ProcessError("Street not found with edge id " + edge);
    // }
    #[allow(unused_variables)]
    pub fn get_street(&mut self, edge: &str) -> &AGStreet {
        /**
         * verify if it is the first time this function is called
         * in this case, we have to complete the streets with the
         * network edges this means that streets are completely
         * loaded (no any more to be read from stat-file)
         */
        if !self.streetsCompleted {
            self.statData.m().consolidate_stat();
            self.complete_streets();
            println!("first completed in getStreet() of City: Consolidation of data not needed in ActivityGen any more");
        }
        //rest of the function
        for s in self.streets.iter() {
            if s.get_id() == edge {
                return &**s;
            }
        }
        println!("===> ERROR: WRONG STREET EDGE ({}) given and not found in street set.", edge);
        panic!("{}", ProcessError::new(format!("Street not found with edge id {}", edge)));
    }

    /// C++: `const AGStreet & AGCity::getRandomStreet()`
    /// returns a random street
    // const AGStreet&
    // AGCity::getRandomStreet() {
    //     if (passengerStreets.empty()) {
    //         throw ProcessError(TL("No street that allows passenger vehicles found in this city."));
    //     }
    //     return *RandHelper::getRandomFrom(passengerStreets);
    // }
    #[allow(unused_variables)]
    pub fn get_random_street(&mut self) -> &AGStreet {
        if self.passengerStreets.is_empty() {
            panic!("{}", ProcessError::new("No street that allows passenger vehicles found in this city."));
        }
        let v = &self.passengerStreets;
        let s = utils::common::rand_helper::with_default(|r| r.get_random_from(v));
        &**s
    }

    /// C++: `AGSchool AGCity::closestSchoolTo(AGPosition)` [private]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn closest_school_to(&mut self, pos: AGPosition) -> AGSchool {
        // (declared in AGCity.h but never defined in C++: calling it is a link error)
        panic!("AGCity::closestSchoolTo is declared but not defined in C++")
    }

    /// C++: `void AGCity::generateOutgoingWP()` [private]
    /// generates workpositions on the city's gates (entrances) for the outgoing work traffic.
    // void
    // AGCity::generateOutgoingWP() {
    //     // work positions outside the city
    //     double nbrWorkers = static_cast<double>(statData.getPeopleYoungerThan(statData.limitAgeRetirement) - statData.getPeopleYoungerThan(statData.limitAgeChildren));
    //     if (nbrWorkers <= 0) {
    //         return;
    //     }
    //     nbrWorkers *= (1.0 - statData.unemployement);
    //     /**
    //      * N_out = N_in * (ProportionOut / (1 - ProportionOut)) = N_out = N_in * (Noutworkers / (Nworkers - Noutworkers))
    //      */
    //     int nbrOutWorkPositions = (int)((double)workPositions.size() * (double)statData.outgoingTraffic / (nbrWorkers - (double)statData.outgoingTraffic));
    //
    //     if (cityGates.empty()) {
    //         statData.workPositions = static_cast<int>(workPositions.size());
    //         return;
    //     }
    //
    //     for (int i = 0; i < nbrOutWorkPositions; ++i) {
    //         int posi = statData.getRandomCityGateByOutgoing();
    //         workPositions.push_back(AGWorkPosition(&statData, cityGates[posi].getStreet(), cityGates[posi].getPosition()));
    //     }
    //     //cout << "outgoing traffic: " << statData.outgoingTraffic << std::endl;
    //     //cout << "total number of workers in the city: " << nbrWorkers << std::endl;
    //     //cout << "work positions out side the city: " << nbrOutWorkPositions << std::endl;
    //     //cout << "work positions in and out of the city: " << workPositions.size() << std::endl;
    //     statData.workPositions = static_cast<int>(workPositions.size());
    // }
    #[allow(unused_variables)]
    pub fn generate_outgoing_wp(&mut self) {
        // work positions outside the city
        let sd = self.statData;
        let mut nbrWorkers = (sd.m().get_people_younger_than(sd.m().limitAgeRetirement) - sd.m().get_people_younger_than(sd.m().limitAgeChildren)) as f64;
        if nbrWorkers <= 0.0 {
            return;
        }
        nbrWorkers *= 1.0 - sd.m().unemployement;
        /**
         * N_out = N_in * (ProportionOut / (1 - ProportionOut)) = N_out = N_in * (Noutworkers / (Nworkers - Noutworkers))
         */
        let outgoing = sd.m().outgoingTraffic as f64;
        let nbrOutWorkPositions = (self.workPositions.len() as f64 * outgoing / (nbrWorkers - outgoing)) as i32;

        if self.cityGates.is_empty() {
            sd.m().workPositions = self.workPositions.len() as i32;
            return;
        }

        for _i in 0..nbrOutWorkPositions {
            let posi = sd.m().get_random_city_gate_by_outgoing() as usize;
            let gate = self.cityGates[posi];
            self.workPositions.push(AGWorkPosition::new_2(sd, gate.get_street(), gate.get_position()));
        }
        //cout << "outgoing traffic: " << statData.outgoingTraffic << std::endl;
        sd.m().workPositions = self.workPositions.len() as i32;
    }

    /// C++: `void AGCity::generateIncomingPopulation()` [private]
    /// generates people from outside the city for incoming traffic generation
    // void
    // AGCity::generateIncomingPopulation() {
    //     for (int i = 0; i < statData.incomingTraffic; ++i) {
    //         AGAdult ad(statData.getRandomPopDistributed(statData.limitAgeChildren, statData.limitAgeRetirement));
    //         peopleIncoming.push_back(ad);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn generate_incoming_population(&mut self) {
        let sd = self.statData;
        for _i in 0..sd.m().incomingTraffic {
            let age = sd.m().get_random_pop_distributed(sd.m().limitAgeChildren, sd.m().limitAgeRetirement);
            self.peopleIncoming.push_back(AGAdult::new(age));
        }
    }

    /// C++: `AGCity & AGCity::operator=(const AGCity &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &AGCity) -> &mut AGCity {
        // (declared private in AGCity.h and never defined in C++: calling it is a link error)
        panic!("AGCity::operator= is declared but not defined in C++")
    }

}

