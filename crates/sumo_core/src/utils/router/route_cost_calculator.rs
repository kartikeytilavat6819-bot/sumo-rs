//! Port of utils/router/RouteCostCalculator.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use crate::utils::router::router_edge::RouterEdge;

/// Abstract base class providing static factory method.
/// C++ class `RouteCostCalculator` (utils/router/RouteCostCalculator.h)
pub struct RouteCostCalculator<R, E, V> {
    /// The maximum route alternatives number
    pub myMaxRouteNumber: i32,
    /// Information whether all routes should be saved
    pub myKeepRoutes: bool,
    /// Information whether new routes shall be computed
    pub mySkipNewRoutes: bool,
    /// Information whether the old route shall be kept
    pub myKeepRouteProb: f64,
    /// C++ vtable slot `setCosts`, filled by the subclass constructor (GawronCalculator / LogitCalculator)
    pub vt_set_costs: Option<fn(&RouteCostCalculator<R, E, V>, Rc<R>, f64, bool)>,
    /// C++ vtable slot `calculateProbabilities`, filled by the subclass constructor
    pub vt_calculate_probabilities: Option<fn(&mut RouteCostCalculator<R, E, V>, &Vec<Rc<R>>, Ptr<V>, SUMOTime)>,
    _p: std::marker::PhantomData<(R, E, V,)>,
}

// C++ static members of RouteCostCalculator (port as module-level statics, see AGENTS.md):
//   static RouteCostCalculator<R, E, V> * RouteCostCalculator::myInstance;
// One myInstance per template instantiation, keyed by its TypeId: (key, owning box of the subclass, pointer to its base)
thread_local! {
    static MY_INSTANCE: std::cell::RefCell<Vec<(std::any::TypeId, Box<dyn std::any::Any>, *mut ())>> = std::cell::RefCell::new(Vec::new());
}

/// The route operations the C++ templates call on R (RORoute is the only instantiation)
pub trait RouteCostRoute<E> {
    fn get_costs(&self) -> f64;
    fn set_costs(&self, costs: f64);
    fn get_probability(&self) -> f64;
    fn set_probability(&self, prob: f64);
    fn get_edge_vector(&self) -> &Vec<Ptr<E>>;
}

impl RouteCostRoute<ROEdge> for RORoute {
    fn get_costs(&self) -> f64 {
        RORoute::get_costs(self)
    }
    fn set_costs(&self, costs: f64) {
        RORoute::set_costs(self, costs)
    }
    fn get_probability(&self) -> f64 {
        RORoute::get_probability(self)
    }
    fn set_probability(&self, prob: f64) {
        RORoute::set_probability(self, prob)
    }
    fn get_edge_vector(&self) -> &Vec<Ptr<ROEdge>> {
        RORoute::get_edge_vector(self)
    }
}

impl<R, E, V> RouteCostCalculator<R, E, V> {
    /// C++: `RouteCostCalculator<R, E, V> & RouteCostCalculator::getCalculator()` [static]
    // template<class R, class E, class V>
    // RouteCostCalculator<R, E, V>& RouteCostCalculator<R, E, V>::getCalculator() {
    //     if (myInstance == 0) {
    //         OptionsCont& oc = OptionsCont::getOptions();
    //         if (oc.getString("route-choice-method") == "logit") {
    //             myInstance = new LogitCalculator<R, E, V>(oc.getFloat("logit.beta"), oc.getFloat("logit.gamma"), oc.getFloat("logit.theta"));
    //         } else if (oc.getString("route-choice-method") == "gawron") {
    //             myInstance = new GawronCalculator<R, E, V>(oc.getFloat("gawron.beta"), oc.getFloat("gawron.a"));
    //         }
    //     }
    //     return *myInstance;
    // }
    #[allow(unused_variables)]
    pub fn get_calculator() -> &'static mut RouteCostCalculator<R, E, V>
    where
        R: RouteCostRoute<E> + 'static,
        E: RouterEdge<V> + 'static,
        V: 'static,
    {
        let key = std::any::TypeId::of::<RouteCostCalculator<R, E, V>>();
        let base = MY_INSTANCE.with(|m| {
            let mut m = m.borrow_mut();
            if !m.iter().any(|(k, _, _)| *k == key) {
                use utils::options::options_cont::{get_float, get_string};
                let method = get_string("route-choice-method");
                if method == "logit" {
                    let mut c = Box::new(LogitCalculator::<R, E, V>::new(get_float("logit.beta"), get_float("logit.gamma"), get_float("logit.theta")));
                    let p = &mut c.base as *mut RouteCostCalculator<R, E, V> as *mut ();
                    m.push((key, c, p));
                } else if method == "gawron" {
                    let mut c = Box::new(GawronCalculator::<R, E, V>::new(get_float("gawron.beta"), get_float("gawron.a")));
                    let p = &mut c.base as *mut RouteCostCalculator<R, E, V> as *mut ();
                    m.push((key, c, p));
                }
            }
            m.iter().find(|(k, _, _)| *k == key).map(|(_, _, p)| *p)
        });
        // C++ dereferences the null myInstance for any other method (RODUAFrame::checkOptions rejects them)
        let base = base.expect("RouteCostCalculator::getCalculator: route-choice-method is neither 'logit' nor 'gawron'");
        unsafe { &mut *(base as *mut RouteCostCalculator<R, E, V>) }
    }

    /// C++: `void RouteCostCalculator::cleanup()` [static]
    // static void cleanup() {
    //         delete myInstance;
    //         myInstance = 0;
    //     }
    #[allow(unused_variables)]
    pub fn cleanup()
    where
        R: 'static,
        E: 'static,
        V: 'static,
    {
        let key = std::any::TypeId::of::<RouteCostCalculator<R, E, V>>();
        MY_INSTANCE.with(|m| m.borrow_mut().retain(|(k, _, _)| *k != key));
    }

    /// C++: `void RouteCostCalculator::setCosts(std::shared_ptr<R>, const double, const bool) const` [const, virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn set_costs(&self, route: Rc<R>, costs: f64, isActive: bool) {
        match self.vt_set_costs {
            Some(f) => f(self, route, costs, isActive),
            None => panic!("pure virtual RouteCostCalculator::setCosts called"),
        }
    }

    /// C++: `void RouteCostCalculator::calculateProbabilities(const std::vector<std::shared_ptr<R>> &, const V *const, const SUMOTime)` [virtual, pure virtual]
    /// calculate the probabilities in the logit model
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn calculate_probabilities(&mut self, alternatives: &Vec<Rc<R>>, veh: Ptr<V>, time: SUMOTime) {
        match self.vt_calculate_probabilities {
            Some(f) => f(self, alternatives, veh, time),
            None => panic!("pure virtual RouteCostCalculator::calculateProbabilities called"),
        }
    }

    /// C++: `int RouteCostCalculator::getMaxRouteNumber() const` [const]
    // int getMaxRouteNumber() const {
    //         return myMaxRouteNumber;
    //     }
    #[allow(unused_variables)]
    pub fn get_max_route_number(&self) -> i32 {
        self.myMaxRouteNumber
    }

    /// C++: `bool RouteCostCalculator::keepAllRoutes() const` [const]
    // bool keepAllRoutes() const {
    //         return myKeepRoutes;
    //     }
    #[allow(unused_variables)]
    pub fn keep_all_routes(&self) -> bool {
        self.myKeepRoutes
    }

    /// C++: `bool RouteCostCalculator::skipRouteCalculation() const` [const]
    // bool skipRouteCalculation() const {
    //         return mySkipNewRoutes;
    //     }
    #[allow(unused_variables)]
    pub fn skip_route_calculation(&self) -> bool {
        self.mySkipNewRoutes
    }

    /// C++: `bool RouteCostCalculator::keepRoute() const` [const]
    // bool keepRoute() const {
    //         if (myKeepRouteProb == 1) {
    //             return true;
    //         } else if (myKeepRouteProb == 0) {
    //             return false;
    //         } else {
    //             return RandHelper::rand() < myKeepRouteProb;
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn keep_route(&self) -> bool {
        if self.myKeepRouteProb == 1.0 {
            true
        } else if self.myKeepRouteProb == 0.0 {
            false
        } else {
            utils::common::rand_helper::rand() < self.myKeepRouteProb
        }
    }

    /// C++: `void RouteCostCalculator::RouteCostCalculator<R, E, V>()` [protected]
    /// Constructor
    // RouteCostCalculator() {
    //         OptionsCont& oc = OptionsCont::getOptions();
    //         myMaxRouteNumber = oc.getInt("max-alternatives");
    //         myKeepRoutes = oc.getBool("keep-all-routes");
    //         mySkipNewRoutes = oc.getBool("skip-new-routes");
    //         myKeepRouteProb = oc.exists("keep-route-probability") ? oc.getFloat("keep-route-probability") : 0;
    //     }
    #[allow(unused_variables)]
    pub fn new() -> Self {
        use utils::options::options_cont::{exists, get_bool, get_float, get_int};
        RouteCostCalculator {
            myMaxRouteNumber: get_int("max-alternatives"),
            myKeepRoutes: get_bool("keep-all-routes"),
            mySkipNewRoutes: get_bool("skip-new-routes"),
            myKeepRouteProb: if exists("keep-route-probability") { get_float("keep-route-probability") } else { 0.0 },
            vt_set_costs: None,
            vt_calculate_probabilities: None,
            _p: std::marker::PhantomData,
        }
    }

    /// the constructor without the options lookup (unit tests)
    #[cfg(test)]
    pub(crate) fn with_params(maxRouteNumber: i32, keepRoutes: bool, skipNewRoutes: bool, keepRouteProb: f64) -> Self {
        RouteCostCalculator {
            myMaxRouteNumber: maxRouteNumber,
            myKeepRoutes: keepRoutes,
            mySkipNewRoutes: skipNewRoutes,
            myKeepRouteProb: keepRouteProb,
            vt_set_costs: None,
            vt_calculate_probabilities: None,
            _p: std::marker::PhantomData,
        }
    }

    /// C++: `void RouteCostCalculator::~RouteCostCalculator<R, E, V>()` [virtual, protected]
    /// Destructor
    // virtual ~RouteCostCalculator() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // empty destructor (C++ `{}`)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    // RouteCostCalculator.h:69-77 — the two deterministic keepRoute branches.
    #[test]
    fn keep_route_boundaries() {
        let c = RouteCostCalculator::<(), (), ()>::with_params(5, false, false, 1.0);
        assert!(c.keep_route());
        let c = RouteCostCalculator::<(), (), ()>::with_params(5, false, false, 0.0);
        assert!(!c.keep_route());
    }
}

