//! Port of utils/router/GawronCalculator.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// Cost calculation with Gawron's method.
/// C++ class `GawronCalculator` (utils/router/GawronCalculator.h)
#[repr(C)]
pub struct GawronCalculator<R, E, V> {
    /// C++ base class `RouteCostCalculator<R, E, V>`
    pub base: RouteCostCalculator<R, E, V>,
    /// gawron beta - value
    pub myBeta: f64,
    /// gawron a - value
    pub myA: f64,
    _p: std::marker::PhantomData<(R, E, V,)>,
}

impl<R, E, V> std::ops::Deref for GawronCalculator<R, E, V> {
    type Target = RouteCostCalculator<R, E, V>;
    fn deref(&self) -> &RouteCostCalculator<R, E, V> {
        &self.base
    }
}
impl<R, E, V> std::ops::DerefMut for GawronCalculator<R, E, V> {
    fn deref_mut(&mut self) -> &mut RouteCostCalculator<R, E, V> {
        &mut self.base
    }
}

impl<R: RouteCostRoute<E>, E, V> GawronCalculator<R, E, V> {
    /// C++: `void GawronCalculator::GawronCalculator<R, E, V>(const double, const double)`
    /// Constructor
    // GawronCalculator(const double beta, const double a) : myBeta(beta), myA(a) {}
    #[allow(unused_variables)]
    pub fn new(beta: f64, a: f64) -> Self {
        let mut base = RouteCostCalculator::new();
        // C++ vtable: the base is the first field of this #[repr(C)] struct, so the cast is the C++ static_cast
        base.vt_set_costs = Some(|b, route, costs, isActive| unsafe { &*(b as *const RouteCostCalculator<R, E, V> as *const Self) }.set_costs(route, costs, isActive));
        base.vt_calculate_probabilities = Some(|b, alternatives, veh, time| unsafe { &mut *(b as *mut RouteCostCalculator<R, E, V> as *mut Self) }.calculate_probabilities(alternatives, veh, time));
        Self { base, myBeta: beta, myA: a, _p: std::marker::PhantomData }
    }

    /// C++: `void GawronCalculator::~GawronCalculator<R, E, V>()` [virtual]
    /// Destructor
    // virtual ~GawronCalculator() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty; nothing to release
    }

    /// C++: `void GawronCalculator::setCosts(std::shared_ptr<R>, const double, const bool) const` [const]
    // void setCosts(std::shared_ptr<R> route, const double costs, const bool isActive = false) const {
    //         if (isActive) {
    //             route->setCosts(costs);
    //         } else {
    //             route->setCosts(myBeta * costs + ((double) 1.0 - myBeta) * route->getCosts());
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn set_costs(&self, route: Rc<R>, costs: f64, isActive: bool) {
        if isActive {
            route.set_costs(costs);
        } else {
            route.set_costs(self.myBeta * costs + (1.0 - self.myBeta) * route.get_costs());
        }
    }

    /// C++: `void GawronCalculator::calculateProbabilities(const std::vector<std::shared_ptr<R>> &, const V *const, const SUMOTime)`
    /// calculate the probabilities
    // void calculateProbabilities(const std::vector<std::shared_ptr<R> >& alternatives, const V* const /* veh */, const SUMOTime /* time */) {
    //         for (typename std::vector<std::shared_ptr<R> >::const_iterator i = alternatives.begin(); i != alternatives.end() - 1; i++) {
    //             std::shared_ptr<R> pR = *i;
    //             for (typename std::vector<std::shared_ptr<R> >::const_iterator j = i + 1; j != alternatives.end(); j++) {
    //                 std::shared_ptr<R> pS = *j;
    //                 // see [Gawron, 1998] (4.2)
    //                 const double delta =
    //                     (pS->getCosts() - pR->getCosts()) /
    //                     (pS->getCosts() + pR->getCosts());
    //                 // see [Gawron, 1998] (4.3a, 4.3b)
    //                 double newPR = gawronF(pR->getProbability(), pS->getProbability(), delta);
    //                 double newPS = pR->getProbability() + pS->getProbability() - newPR;
    //                 if (std::isnan(newPR) || std::isnan(newPS)) {
    //                     newPR = pS->getCosts() > pR->getCosts()
    //                             ? (double) 1. : 0;
    //                     newPS = pS->getCosts() > pR->getCosts()
    //                             ? 0 : (double) 1.;
    //                 }
    //                 newPR = MIN2((double) MAX2(newPR, (double) 0), (double) 1);
    //                 newPS = MIN2((double) MAX2(newPS, (double) 0), (double) 1);
    //                 pR->setProbability(newPR);
    //                 pS->setProbability(newPS);
    //             }
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn calculate_probabilities(&mut self, alternatives: &Vec<Rc<R>>, arg1: Ptr<V>, arg2: SUMOTime) {
        for i in 0..alternatives.len() - 1 {
            let pR = &alternatives[i];
            for j in i + 1..alternatives.len() {
                let pS = &alternatives[j];
                // see [Gawron, 1998] (4.2)
                let delta = (pS.get_costs() - pR.get_costs()) / (pS.get_costs() + pR.get_costs());
                // see [Gawron, 1998] (4.3a, 4.3b)
                let mut newPR = self.gawron_f(pR.get_probability(), pS.get_probability(), delta);
                let mut newPS = pR.get_probability() + pS.get_probability() - newPR;
                if newPR.is_nan() || newPS.is_nan() {
                    newPR = if pS.get_costs() > pR.get_costs() { 1.0 } else { 0.0 };
                    newPS = if pS.get_costs() > pR.get_costs() { 0.0 } else { 1.0 };
                }
                // MIN2(MAX2(x, 0), 1)
                newPR = min2(max2(newPR, 0.0), 1.0);
                newPS = min2(max2(newPS, 0.0), 1.0);
                pR.set_probability(newPR);
                pS.set_probability(newPS);
            }
        }
    }

    /// C++: `double GawronCalculator::gawronF(const double, const double, const double) const` [const, private]
    /// Performs the gawron - f() function From "Dynamic User Equilibria..."
    // double gawronF(const double pdr, const double pds, const double x) const {
    //         if (pdr * gawronG(myA, x) + pds == 0) {
    //             return std::numeric_limits<double>::max();
    //         }
    //         return (pdr * (pdr + pds) * gawronG(myA, x)) /
    //                (pdr * gawronG(myA, x) + pds);
    //     }
    #[allow(unused_variables)]
    pub fn gawron_f(&self, pdr: f64, pds: f64, x: f64) -> f64 {
        let g = self.gawron_g(self.myA, x);
        if pdr * g + pds == 0. {
            return f64::MAX;
        }
        (pdr * (pdr + pds) * g) / (pdr * g + pds)
    }

    /// C++: `double GawronCalculator::gawronG(const double, const double) const` [const, private]
    /// Performs the gawron - g() function From "Dynamic User Equilibria..."
    // double gawronG(const double a, const double x) const {
    //         if (((1.0 - (x * x)) == 0)) {
    //             return std::numeric_limits<double>::max();
    //         }
    //         return (double) exp((a * x) / (1.0 - (x * x)));
    //     }
    #[allow(unused_variables)]
    pub fn gawron_g(&self, a: f64, x: f64) -> f64 {
        if (1.0 - (x * x)) == 0. {
            return f64::MAX;
        }
        ((a * x) / (1.0 - (x * x))).exp()
    }

    /// C++: `GawronCalculator<R, E, V> & GawronCalculator::operator=(const GawronCalculator<R, E, V> &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &GawronCalculator<R, E, V>) -> &mut GawronCalculator<R, E, V> {
        panic!("GawronCalculator assignment operator is private and invalidated in C++")
    }

}

/// C++ MIN2 (utils/common/StdDefs.h)
fn min2(a: f64, b: f64) -> f64 {
    if a < b { a } else { b }
}

/// C++ MAX2 (utils/common/StdDefs.h)
fn max2(a: f64, b: f64) -> f64 {
    if a > b { a } else { b }
}
