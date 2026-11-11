//! Port of microsim/logging/CastingFunctionBinding_Param.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `O (T::*)(P) const CastingFunctionBinding_Param::Operation`
pub type CastingFunctionBinding_Param_Operation = Unmapped /* O (T::*)(P) */;

/// C++ class `CastingFunctionBinding_Param` (microsim/logging/CastingFunctionBinding_Param.h)
pub struct CastingFunctionBinding_Param<T, R, O, P> {
    /// C++ base class `ValueSource<R>`
    pub base: Unmapped /* ValueSource<R> */,
    /// The object the action is directed to.
    pub mySource: Ptr<T>,
    /// The object's operation to perform.
    pub myOperation: SUMOAbstractRouter_Operation,
    /// The scale to apply.
    pub myScale: R,
    /// The parameter to apply to the operation.
    pub myParam: P,
    _p: std::marker::PhantomData<(T, R, O, P,)>,
}

impl<T, R, O, P> std::ops::Deref for CastingFunctionBinding_Param<T, R, O, P> {
    type Target = Unmapped /* ValueSource<R> */;
    fn deref(&self) -> &Unmapped /* ValueSource<R> */ {
        &self.base
    }
}
impl<T, R, O, P> std::ops::DerefMut for CastingFunctionBinding_Param<T, R, O, P> {
    fn deref_mut(&mut self) -> &mut Unmapped /* ValueSource<R> */ {
        &mut self.base
    }
}

impl<T, R, O, P> CastingFunctionBinding_Param<T, R, O, P> {
    /// C++: `void CastingFunctionBinding_Param::CastingFunctionBinding_Param<T, R, O, P>(T *, Operation, P, const R)`
    // CastingFunctionBinding_Param(T* source, Operation operation, P param, const R scale = 1.0) :
    //         mySource(source),
    //         myOperation(operation),
    //         myScale(scale),
    //         myParam(param)
    //     {}
    #[allow(unused_variables)]
    pub fn new(source: Ptr<T>, operation: SUMOAbstractRouter_Operation, param: P, scale: R) -> Self {
        todo!("port CastingFunctionBinding_Param::CastingFunctionBinding_Param<T, R, O, P>")
    }

    /// C++: `void CastingFunctionBinding_Param::~CastingFunctionBinding_Param<T, R, O, P>()`
    /// Destructor.
    // ~CastingFunctionBinding_Param() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port CastingFunctionBinding_Param::~CastingFunctionBinding_Param<T, R, O, P>")
    }

    /// C++: `R CastingFunctionBinding_Param::getValue() const` [const]
    // R getValue() const {
    //         return myScale * (R)(mySource->*myOperation)(myParam);
    //     }
    #[allow(unused_variables)]
    pub fn get_value(&self) -> R {
        todo!("port CastingFunctionBinding_Param::getValue")
    }

    /// C++: `ValueSource<R> * CastingFunctionBinding_Param::copy() const` [const]
    // ValueSource<R>* copy() const {
    //         return new CastingFunctionBinding_Param<T, R, O, P>(mySource, myOperation, myParam, myScale);
    //     }
    #[allow(unused_variables)]
    pub fn copy(&self) -> Ptr<Unmapped /* ValueSource<R> */> {
        todo!("port CastingFunctionBinding_Param::copy")
    }

    /// C++: `ValueSource<double> * CastingFunctionBinding_Param::makedoubleReturningCopy() const` [const]
    // ValueSource<double>* makedoubleReturningCopy() const {
    //         return new CastingFunctionBinding_Param<T, double, O, P>(mySource, myOperation, myParam, myScale);
    //     }
    #[allow(unused_variables)]
    pub fn makedouble_returning_copy(&self) -> Ptr<Unmapped /* ValueSource<double> */> {
        todo!("port CastingFunctionBinding_Param::makedoubleReturningCopy")
    }

    /// C++: `CastingFunctionBinding_Param<T, R, O, P> & CastingFunctionBinding_Param::operator=(const CastingFunctionBinding_Param<T, R, O, P> &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &CastingFunctionBinding_Param<T, R, O, P>) -> &mut CastingFunctionBinding_Param<T, R, O, P> {
        todo!("port CastingFunctionBinding_Param::operator=")
    }

}

