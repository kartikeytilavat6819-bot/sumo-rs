//! Port of activitygen/city/AGDataAndStatistics.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `AGDataAndStatistics` (activitygen/city/AGDataAndStatistics.h)
pub struct AGDataAndStatistics {
    /// parameters
    pub speedTimePerKm: f64,
    pub carPreference: f64,
    /// from General element from XML file.
    pub inhabitants: i32,
    pub households: i32,
    pub limitAgeChildren: i32,
    pub limitAgeRetirement: i32,
    pub limitEndAge: i32,
    pub carRate: f64,
    pub unemployement: f64,
    pub laborDemand: f64,
    pub maxFootDistance: f64,
    pub incomingTraffic: i32,
    pub outgoingTraffic: i32,
    pub freeTimeActivityRate: f64,
    pub uniformRandomTrafficRate: f64,
    pub departureVariation: f64,
    pub beginWorkHours: BTreeMap<i32, f64>,
    pub endWorkHours: BTreeMap<i32, f64>,
    pub busStations: BTreeMap<i32, AGPosition>,
    pub population: BTreeMap<i32, f64>,
    pub ageSpan: BTreeMap<i32, i32>,
    /// number of incoming or outgoing people through the given city gates PROPORTION: it should be normalized
    pub incoming: BTreeMap<i32, f64>,
    pub outgoing: BTreeMap<i32, f64>,
    /// computed values used during processing
    pub workPositions: i32,
    pub factorInhabitants: f64,
    pub factorWorkPositions: f64,
    /// data used for household generation has to be computed before.
    pub oldAgeHhProb: f64,
    pub secondPersProb: f64,
    pub meanNbrChildren: f64,
    pub AdultNbr: i32,
    pub householdsNbr: i32,
    pub hhFarFromPT: i32,
}

impl AGDataAndStatistics {
    /// C++: `AGDataAndStatistics & AGDataAndStatistics::getDataAndStatistics()` [static]
    // AGDataAndStatistics&
    // AGDataAndStatistics::getDataAndStatistics() {
    //     static AGDataAndStatistics ds;
    //     return ds;
    // }
    #[allow(unused_variables)]
    pub fn get_data_and_statistics() -> &'static mut AGDataAndStatistics {
        todo!("port AGDataAndStatistics::getDataAndStatistics")
    }

    /// C++: `int AGDataAndStatistics::getRandom(int, int)`
    /// function returning a random number between the two given numbers: [n;m[ (m cannot occur) it returns 0 if m < n
    // int
    // AGDataAndStatistics::getRandom(int n, int m) {
    //     if (m < n) {
    //         return 0;
    //     }
    //     int num = RandHelper::rand(m - n);
    //     num += n;
    //     return num;
    // }
    #[allow(unused_variables)]
    pub fn get_random(&mut self, n: i32, m: i32) -> i32 {
        todo!("port AGDataAndStatistics::getRandom")
    }

    /// C++: `int AGDataAndStatistics::getRandomPopDistributed(int, int)`
    /// function returning a random age between the two numbers satisfying the previous constrains this number is in relation to the distribution of the population through the brackets (population's list) if the given numbers are both greater than limitEndAge, it returns 0 if m is greater than limitEndAge, m=limitEndAge returns -1 if conditions are not satisfied
    // int
    // AGDataAndStatistics::getRandomPopDistributed(int n, int m) {
    //     if (m < n || n >= limitEndAge) {
    //         return -1;
    //     }
    //     if (m > limitEndAge) {
    //         m = limitEndAge;
    //     }
    //     const double alea = RandHelper::rand(getPropYoungerThan(n), getPropYoungerThan(m));
    //     for (int a = n; a < m; ++a) {
    //         if (alea < getPropYoungerThan(a + 1)) {
    //             return a;
    //         }
    //     }
    //     return -1;
    // }
    #[allow(unused_variables)]
    pub fn get_random_pop_distributed(&mut self, n: i32, m: i32) -> i32 {
        todo!("port AGDataAndStatistics::getRandomPopDistributed")
    }

    /// C++: `int AGDataAndStatistics::getPoissonsNumberOfChildren(double)`
    /// function evaluating the POISSON's lay (probability lay) it returns the number of children with a probability of POISSON in a household is given the mean of the distribution
    // int
    // AGDataAndStatistics::getPoissonsNumberOfChildren(double mean) {
    //     double alea = RandHelper::rand();
    //     double cumul = 0;
    //     for (int nbr = 0; nbr < LIMIT_CHILDREN_NUMBER; ++nbr) {
    //         cumul += poisson(mean, nbr);
    //         if (cumul > alea) {
    //             return nbr;
    //         }
    //     }
    //     return LIMIT_CHILDREN_NUMBER;
    // }
    #[allow(unused_variables)]
    pub fn get_poissons_number_of_children(&mut self, mean: f64) -> i32 {
        todo!("port AGDataAndStatistics::getPoissonsNumberOfChildren")
    }

    /// C++: `int AGDataAndStatistics::getPeopleOlderThan(int)`
    /// these functions return the number of people having more (or less) than the given age (inclusive for getPeopleOlderThan; exclusive for getPeopleYoungerThan) getPeopleOlderThan(n) + getPeopleYoungerThan(n) = inhabitants these first two function are based on the third one.
    // int
    // AGDataAndStatistics::getPeopleOlderThan(int age) {
    //     return (inhabitants - getPeopleYoungerThan(age));
    // }
    #[allow(unused_variables)]
    pub fn get_people_older_than(&mut self, age: i32) -> i32 {
        todo!("port AGDataAndStatistics::getPeopleOlderThan")
    }

    /// C++: `int AGDataAndStatistics::getPeopleYoungerThan(int)`
    // int
    // AGDataAndStatistics::getPeopleYoungerThan(int age) {
    //     return (int)((double)inhabitants * getPropYoungerThan(age) + .5);
    // }
    #[allow(unused_variables)]
    pub fn get_people_younger_than(&mut self, age: i32) -> i32 {
        todo!("port AGDataAndStatistics::getPeopleYoungerThan")
    }

    /// C++: `double AGDataAndStatistics::getPropYoungerThan(int)`
    // double
    // AGDataAndStatistics::getPropYoungerThan(int age) {
    //     std::map<int, double>::iterator it;
    //     double sum = 0;
    //     int previousAge = 0;
    //     double prop = 0;
    //
    //     for (it = population.begin(); it != population.end(); ++it) {
    //         if (it->first < age) {
    //             sum += it->second;
    //         } else if (it->first >= age && previousAge < age) {
    //             prop = ((double)(age - previousAge) / (double)(it->first - previousAge));
    //             sum += prop * it->second;
    //             break;
    //         }
    //         previousAge = it->first;
    //     }
    //     return sum;
    // }
    #[allow(unused_variables)]
    pub fn get_prop_younger_than(&mut self, age: i32) -> f64 {
        todo!("port AGDataAndStatistics::getPropYoungerThan")
    }

    /// C++: `void AGDataAndStatistics::consolidateStat()`
    /// function consolidating statistics: normalizes the maps with probabilities completes data which have to be computed before use
    // void
    // AGDataAndStatistics::consolidateStat() {
    //     normalizeMapProb(&beginWorkHours);
    //     normalizeMapProb(&endWorkHours);
    //     normalizeMapProb(&population);
    //     normalizeMapProb(&incoming);
    //     normalizeMapProb(&outgoing);
    //     limitEndAge = population.rbegin()->first;
    //
    //     oldAgeHhProb = (double)getPeopleOlderThan(limitAgeRetirement) / (double)getPeopleOlderThan(limitAgeChildren);
    //     secondPersProb = (double)(getPeopleOlderThan(limitAgeChildren) - households) / (double)households;
    //     meanNbrChildren = (double)getPeopleYoungerThan(limitAgeChildren) / ((1 - oldAgeHhProb) * (double)households);
    //     //cout << " --> oldAgeHhProb = " << setprecision(3) << oldAgeHhProb << "  - retAge? " << getPeopleOlderThan(limitAgeRetirement) << " adAge? " << getPeopleOlderThan(limitAgeChildren) << endl;
    //     //cout << " --> secondPersProb = " << setprecision(3) << secondPersProb << "  - adAge? " << getPeopleOlderThan(limitAgeChildren) << " hh?" << households << endl;
    //     //cout << " --> meanNbrChildren = " << setprecision(3) << meanNbrChildren << "  - chAge? " << getPeopleYoungerThan(limitAgeChildren) << endl;
    // }
    #[allow(unused_variables)]
    pub fn consolidate_stat(&mut self) {
        todo!("port AGDataAndStatistics::consolidateStat")
    }

    /// C++: `double AGDataAndStatistics::getInverseExpRandomValue(double, double)`
    /// function returning a random value corresponding to this distribution: -mean is given -max variation of the mean (|possible value - mean| <= maxVar -the mean is the most probable -the probability distribution function is a "scaled exponential" distribution from mean-maxVar to mean and from mean to mean+maxVar
    // double
    // AGDataAndStatistics::getInverseExpRandomValue(double mean, double maxVar) {
    //     if (maxVar <= 0) {
    //         return mean;
    //     }
    //     double p = RandHelper::rand(static_cast<double>(0.0001), static_cast<double>(1));
    //     //we have to scale the distribution because maxVar is different from INF
    //     double scale = exp((-1) * maxVar);
    //     //new p: scaled
    //     p = p * (1 - scale) + scale; // p = [scale; 1) ==> (1-p) = (0; 1-scale]
    //
    //     double variation = (-1) * log(p);
    //     //decide the side of the mean value
    //     if (RandHelper::rand(1000) < 500) {
    //         return mean + variation;
    //     } else {
    //         return mean - variation;
    //     }
    //
    // }
    #[allow(unused_variables)]
    pub fn get_inverse_exp_random_value(&mut self, mean: f64, maxVar: f64) -> f64 {
        todo!("port AGDataAndStatistics::getInverseExpRandomValue")
    }

    /// C++: `int AGDataAndStatistics::getRandomCityGateByIncoming()`
    /// function returning a random city gate corresponding to the distribution of the incoming/outgoing traffic In fact, it returns the position of a city gate in the citygate vector.
    // int
    // AGDataAndStatistics::getRandomCityGateByIncoming() {
    //     double alea = RandHelper::rand();
    //     double total = 0;
    //     std::map<int, double>::iterator it;
    //     for (it = incoming.begin(); it != incoming.end(); ++it) {
    //         total += it->second;
    //         if (alea < total) {
    //             return it->first;
    //         }
    //     }
    //     std::cout << "ERROR: incoming at city gates not normalized" << std::endl;
    //     return 0;
    // }
    #[allow(unused_variables)]
    pub fn get_random_city_gate_by_incoming(&mut self) -> i32 {
        todo!("port AGDataAndStatistics::getRandomCityGateByIncoming")
    }

    /// C++: `int AGDataAndStatistics::getRandomCityGateByOutgoing()`
    // int
    // AGDataAndStatistics::getRandomCityGateByOutgoing() {
    //     double alea = RandHelper::rand();
    //     double total = 0;
    //     std::map<int, double>::iterator it;
    //     for (it = outgoing.begin(); it != outgoing.end(); ++it) {
    //         total += it->second;
    //         if (alea < total) {
    //             return it->first;
    //         }
    //     }
    //     std::cout << "ERROR: outgoing at city gates not normalized" << std::endl;
    //     return 0;
    // }
    #[allow(unused_variables)]
    pub fn get_random_city_gate_by_outgoing(&mut self) -> i32 {
        todo!("port AGDataAndStatistics::getRandomCityGateByOutgoing")
    }

    /// C++: `void AGDataAndStatistics::AGDataAndStatistics()` [private]
    // AGDataAndStatistics() {}
    #[allow(unused_variables)]
    pub fn new() -> Self {
        todo!("port AGDataAndStatistics::AGDataAndStatistics")
    }

    /// C++: `double AGDataAndStatistics::poisson(double, int)` [private]
    /// returns the POISSON's probability (exp(-m)*m^k/k!)
    // double
    // AGDataAndStatistics::poisson(double mean, int occ) {
    //     return exp(-mean) * pow(mean, occ) / (double)factorial(occ);
    // }
    #[allow(unused_variables)]
    pub fn poisson(&mut self, mean: f64, occ: i32) -> f64 {
        todo!("port AGDataAndStatistics::poisson")
    }

    /// C++: `int AGDataAndStatistics::factorial(int)` [private]
    /// recursive mathematical function returning the factorial of n: n!
    // int
    // AGDataAndStatistics::factorial(int fact) {
    //     if (fact > 0) {
    //         return fact * factorial(fact - 1);
    //     }
    //     return 1;
    // }
    #[allow(unused_variables)]
    pub fn factorial(&mut self, n: i32) -> i32 {
        todo!("port AGDataAndStatistics::factorial")
    }

    /// C++: `void AGDataAndStatistics::normalizeMapProb(std::map<int, double> *)` [private]
    // void
    // AGDataAndStatistics::normalizeMapProb(std::map<int, double>* myMap) {
    //     double sum = 0;
    //     std::map<int, double>::iterator it;
    //     for (it = myMap->begin(); it != myMap->end(); ++it) {
    //         sum += it->second;
    //     }
    //     if (sum == 0) {
    //         return;
    //     }
    //     for (it = myMap->begin(); it != myMap->end(); ++it) {
    //         it->second = it->second / sum;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn normalize_map_prob(&mut self, myMap: Ptr<BTreeMap<i32, f64>>) {
        todo!("port AGDataAndStatistics::normalizeMapProb")
    }

}

