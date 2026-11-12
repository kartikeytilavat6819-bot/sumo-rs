//! Port of microsim/logging/CastingFunctionBinding.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `O (T::*)() const CastingFunctionBinding::Operation`
pub type CastingFunctionBinding_Operation = Unmapped /* O (T::*)() */;

/// C++ class `CastingFunctionBinding` (microsim/logging/CastingFunctionBinding.h)
#[repr(C)]
pub struct CastingFunctionBinding<T, R, O> {
    /// C++ base class `ValueSource<R>`
    pub base: Unmapped /* ValueSource<R> */,
    /// The object the action is directed to.
    pub mySource: Ptr<T>,
    /// The object's operation to perform.
    pub myOperation: SUMOAbstractRouter_Operation,
    /// The scale to apply.
    pub myScale: R,
    _p: std::marker::PhantomData<(T, R, O,)>,
}

impl<T, R, O> std::ops::Deref for CastingFunctionBinding<T, R, O> {
    type Target = Unmapped /* ValueSource<R> */;
    fn deref(&self) -> &Unmapped /* ValueSource<R> */ {
        &self.base
    }
}
impl<T, R, O> std::ops::DerefMut for CastingFunctionBinding<T, R, O> {
    fn deref_mut(&mut self) -> &mut Unmapped /* ValueSource<R> */ {
        &mut self.base
    }
}

impl<T, R, O> CastingFunctionBinding<T, R, O> {
    /// C++: `void CastingFunctionBinding::CastingFunctionBinding<T, R, O>(T *, Operation, const R)`
    // CastingFunctionBinding(T* source, Operation operation, const R scale = 1) :
    //         mySource(source),
    //         myOperation(operation),
    //         myScale(scale) {}
    #[allow(unused_variables)]
    pub fn new(source: Ptr<T>, operation: SUMOAbstractRouter_Operation, scale: R) -> Self {
        Self {
            base: (),
            mySource: source,
            myOperation: operation,
            myScale: scale,
            _p: std::marker::PhantomData,
        }
    }

    /// C++: `void CastingFunctionBinding::~CastingFunctionBinding<T, R, O>()`
    /// Destructor.
    // ~CastingFunctionBinding() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
    }

    /// C++: `R CastingFunctionBinding::getValue() const` [const]
    // R getValue() const {
    //         return myScale * (R)(mySource->*myOperation)();
    //     }
    #[allow(unused_variables)]
    pub fn get_value(&self) -> R {
        // shortcut: needs the pointer-to-member Operation (Unmapped) and ValueSource<R>, upgrade when SUMOAbstractRouter_Operation is mapped to a Rust type
        panic!("CastingFunctionBinding::getValue is not ported yet")
    }

    /// C++: `ValueSource<R> * CastingFunctionBinding::copy() const` [const]
    // ValueSource<R>* copy() const {
    //         return new CastingFunctionBinding<T, R, O>(mySource, myOperation, myScale);
    //     }
    #[allow(unused_variables)]
    pub fn copy(&self) -> Ptr<Unmapped /* ValueSource<R> */> {
        // shortcut: ValueSource<R> is not ported, so the heap copy cannot be typed; upgrade when ValueSource is ported
        panic!("CastingFunctionBinding::copy is not ported yet")
    }

    /// C++: `ValueSource<double> * CastingFunctionBinding::makedoubleReturningCopy() const` [const]
    // ValueSource<double>* makedoubleReturningCopy() const {
    //         return new CastingFunctionBinding<T, double, O>(mySource, myOperation, myScale);
    //     }
    #[allow(unused_variables)]
    pub fn makedouble_returning_copy(&self) -> Ptr<Unmapped /* ValueSource<double> */> {
        // shortcut: ValueSource<double> is not ported, so the heap copy cannot be typed; upgrade when ValueSource is ported
        panic!("CastingFunctionBinding::makedoubleReturningCopy is not ported yet")
    }

    /// C++: `CastingFunctionBinding<T, R, O> & CastingFunctionBinding::operator=(const CastingFunctionBinding<T, R, O> &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &CastingFunctionBinding<T, R, O>) -> &mut CastingFunctionBinding<T, R, O> {
        panic!("CastingFunctionBinding assignment operator is private and invalidated in C++")
    }

}

