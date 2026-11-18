//! Port of utils/router/LogitCalculator.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use crate::utils::router::router_edge::RouterEdge;

/// Cost calculation with c-logit or logit method.
/// C++ class `LogitCalculator` (utils/router/LogitCalculator.h)
#[repr(C)]
pub struct LogitCalculator<R, E, V> {
    /// C++ base class `RouteCostCalculator<R, E, V>`
    pub base: RouteCostCalculator<R, E, V>,
    /// logit beta - value
    pub myBeta: f64,
    /// logit gamma - value
    pub myGamma: f64,
    /// logit theta - value
    pub myTheta: f64,
    /// The route commonality factors for c-logit
    pub myCommonalities: BTreeMap<Ptr<R>, f64>,
    _p: std::marker::PhantomData<(R, E, V,)>,
}

impl<R, E, V> std::ops::Deref for LogitCalculator<R, E, V> {
    type Target = RouteCostCalculator<R, E, V>;
    fn deref(&self) -> &RouteCostCalculator<R, E, V> {
        &self.base
    }
}
impl<R, E, V> std::ops::DerefMut for LogitCalculator<R, E, V> {
    fn deref_mut(&mut self) -> &mut RouteCostCalculator<R, E, V> {
        &mut self.base
    }
}

impl<R: RouteCostRoute<E>, E: RouterEdge<V>, V> LogitCalculator<R, E, V> {
    /// C++: `void LogitCalculator::LogitCalculator<R, E, V>(const double, const double, const double)`
    /// Constructor
    // LogitCalculator(const double beta, const double gamma,
    //                     const double theta) : myBeta(beta), myGamma(gamma), myTheta(theta) {}
    #[allow(unused_variables)]
    pub fn new(beta: f64, gamma: f64, theta: f64) -> Self {
        let mut base = RouteCostCalculator::new();
        // C++ vtable: the base is the first field of this #[repr(C)] struct, so the cast is the C++ static_cast
        base.vt_set_costs = Some(|b, route, costs, isActive| unsafe { &*(b as *const RouteCostCalculator<R, E, V> as *const Self) }.set_costs(route, costs, isActive));
        base.vt_calculate_probabilities = Some(|b, alternatives, veh, time| unsafe { &mut *(b as *mut RouteCostCalculator<R, E, V> as *mut Self) }.calculate_probabilities(alternatives, veh, time));
        Self { base, myBeta: beta, myGamma: gamma, myTheta: theta, myCommonalities: BTreeMap::new(), _p: std::marker::PhantomData }
    }

    /// C++: `void LogitCalculator::~LogitCalculator<R, E, V>()` [virtual]
    /// Destructor
    // virtual ~LogitCalculator() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty; nothing to release
    }

    /// C++: `void LogitCalculator::setCosts(std::shared_ptr<R>, const double, const bool) const` [const]
    // void setCosts(std::shared_ptr<R> route, const double costs, const bool /* isActive */) const {
    //         route->setCosts(costs);
    //     }
    #[allow(unused_variables)]
    pub fn set_costs(&self, route: Rc<R>, costs: f64, arg2: bool) {
        route.set_costs(costs);
    }

    /// C++: `void LogitCalculator::calculateProbabilities(const std::vector<std::shared_ptr<R>> &, const V *const, const SUMOTime)`
    /// calculate the probabilities in the logit model
    // void calculateProbabilities(const std::vector<std::shared_ptr<R> >& alternatives, const V* const veh, const SUMOTime time) {
    //         const double theta = myTheta >= 0 ? myTheta : getThetaForCLogit(alternatives);
    //         const double beta = myBeta >= 0 ? myBeta : getBetaForCLogit(alternatives);
    //         const double t = STEPS2TIME(time);
    //         if (beta > 0) {
    //             // calculate commonalities
    //             for (const std::shared_ptr<R>& pR : alternatives) {
    //                 double lengthR = 0;
    //                 const std::vector<const E*>& edgesR = pR->getEdgeVector();
    //                 for (const E* const edge : edgesR) {
    //                     //@todo we should use costs here
    //                     lengthR += edge->getTravelTime(veh, t);
    //                 }
    //                 double overlapSum = 0;
    //                 for (const std::shared_ptr<R>& pS : alternatives) {
    //                     double overlapLength = 0.;
    //                     double lengthS = 0;
    //                     for (const E* const edge : pS->getEdgeVector()) {
    //                         lengthS += edge->getTravelTime(veh, t);
    //                         if (std::find(edgesR.begin(), edgesR.end(), edge) != edgesR.end()) {
    //                             overlapLength += edge->getTravelTime(veh, t);
    //                         }
    //                     }
    //                     overlapSum += pow(overlapLength / sqrt(lengthR * lengthS), myGamma);
    //                 }
    //                 myCommonalities[pR.get()] = beta * log(overlapSum);
    //             }
    //         }
    //         for (const std::shared_ptr<R>& pR : alternatives) {
    //             double weightedSum = 0;
    //             for (const std::shared_ptr<R>& pS : alternatives) {
    //                 weightedSum += exp(theta * (pR->getCosts() - pS->getCosts() + myCommonalities[pR.get()] - myCommonalities[pS.get()]));
    //             }
    //             pR->setProbability(1. / weightedSum);
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn calculate_probabilities(&mut self, alternatives: &Vec<Rc<R>>, veh: Ptr<V>, time: SUMOTime) {
        let theta = if self.myTheta >= 0. { self.myTheta } else { self.get_theta_for_c_logit(alternatives.clone()) };
        let beta = if self.myBeta >= 0. { self.myBeta } else { self.get_beta_for_c_logit(alternatives.clone()) };
        let t = utils::common::time::steps2time(time);
        if beta > 0. {
            // calculate commonalities
            for pR in alternatives.iter() {
                let mut lengthR: f64 = 0.;
                let edgesR = pR.get_edge_vector();
                for edge in edgesR.iter() {
                    //@todo we should use costs here
                    lengthR += edge.travel_time(veh, t);
                }
                let mut overlapSum: f64 = 0.;
                for pS in alternatives.iter() {
                    let mut overlapLength: f64 = 0.;
                    let mut lengthS: f64 = 0.;
                    for edge in pS.get_edge_vector().iter() {
                        lengthS += edge.travel_time(veh, t);
                        if edgesR.contains(edge) {
                            overlapLength += edge.travel_time(veh, t);
                        }
                    }
                    overlapSum += (overlapLength / (lengthR * lengthS).sqrt()).powf(self.myGamma);
                }
                self.myCommonalities.insert(Ptr::new(&**pR), beta * overlapSum.ln());
            }
        }
        for pR in alternatives.iter() {
            let mut weightedSum: f64 = 0.;
            for pS in alternatives.iter() {
                // C++ std::map::operator[] default-inserts 0
                let cR = *self.myCommonalities.entry(Ptr::new(&**pR)).or_insert(0.);
                let cS = *self.myCommonalities.entry(Ptr::new(&**pS)).or_insert(0.);
                weightedSum += (theta * (pR.get_costs() - pS.get_costs() + cR - cS)).exp();
            }
            pR.set_probability(1. / weightedSum);
        }
    }

    /// C++: `double LogitCalculator::getBetaForCLogit(const std::vector<std::shared_ptr<R>>) const` [const, private]
    /// calculate the scaling factor in the logit model
    // double getBetaForCLogit(const std::vector<std::shared_ptr<R> > alternatives) const {
    //         double min = std::numeric_limits<double>::max();
    //         for (const std::shared_ptr<R>& pR : alternatives) {
    //             const double cost = pR->getCosts() / 3600.;
    //             if (cost < min) {
    //                 min = cost;
    //             }
    //         }
    //         return min;
    //     }
    #[allow(unused_variables)]
    pub fn get_beta_for_c_logit(&self, alternatives: Vec<Rc<R>>) -> f64 {
        let mut min = f64::MAX;
        for pR in alternatives.iter() {
            let cost = pR.get_costs() / 3600.;
            if cost < min {
                min = cost;
            }
        }
        min
    }

    /// C++: `double LogitCalculator::getThetaForCLogit(const std::vector<std::shared_ptr<R>>) const` [const, private]
    /// calculate the scaling factor in the logit model
    // double getThetaForCLogit(const std::vector<std::shared_ptr<R> > alternatives) const {
    //         // @todo this calculation works for travel times only
    //         double sum = 0.;
    //         double diff = 0.;
    //         double min = std::numeric_limits<double>::max();
    //         for (const std::shared_ptr<R>& pR : alternatives) {
    //             const double cost = pR->getCosts() / 3600.;
    //             sum += cost;
    //             if (cost < min) {
    //                 min = cost;
    //             }
    //         }
    //         const double meanCost = sum / double(alternatives.size());
    //         for (const std::shared_ptr<R>& pR : alternatives) {
    //             diff += pow(pR->getCosts() / 3600. - meanCost, 2);
    //         }
    //         const double cvCost = sqrt(diff / double(alternatives.size())) / meanCost;
    //         // @todo re-evaluate function
    //         if (cvCost > 0) { // all Magic numbers from Lohse book, original says this should be cvCost > 0.04
    //             return 3.1415926535897932384626433832795 / (sqrt(6.) * cvCost * (min + 1.1)) / 3600.;
    //         }
    //         return 1. / 3600.;
    //     }
    #[allow(unused_variables)]
    pub fn get_theta_for_c_logit(&self, alternatives: Vec<Rc<R>>) -> f64 {
        // @todo this calculation works for travel times only
        let mut sum: f64 = 0.;
        let mut diff: f64 = 0.;
        let mut min = f64::MAX;
        for pR in alternatives.iter() {
            let cost = pR.get_costs() / 3600.;
            sum += cost;
            if cost < min {
                min = cost;
            }
        }
        let meanCost = sum / (alternatives.len() as f64);
        for pR in alternatives.iter() {
            diff += (pR.get_costs() / 3600. - meanCost).powf(2.);
        }
        let cvCost = (diff / (alternatives.len() as f64)).sqrt() / meanCost;
        // @todo re-evaluate function
        if cvCost > 0. {
            // all Magic numbers from Lohse book, original says this should be cvCost > 0.04
            return 3.1415926535897932384626433832795 / ((6.0f64).sqrt() * cvCost * (min + 1.1)) / 3600.;
        }
        1. / 3600.
    }

    /// C++: `LogitCalculator<R, E, V> & LogitCalculator::operator=(const LogitCalculator<R, E, V> &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &LogitCalculator<R, E, V>) -> &mut LogitCalculator<R, E, V> {
        panic!("LogitCalculator assignment operator is private and invalidated in C++")
    }

}

