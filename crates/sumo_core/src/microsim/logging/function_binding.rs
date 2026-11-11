//! Port of microsim/logging/FunctionBinding.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `R (T::*)() const FunctionBinding::Operation`
pub type FunctionBinding_Operation = Unmapped /* R (T::*)() */;

/// C++ typedef `int (T::*)() const FunctionBindingString::Operation`
pub type FunctionBindingString_Operation = Unmapped /* int (T::*)() */;

/// C++ class `FunctionBinding` (microsim/logging/FunctionBinding.h)
pub struct FunctionBinding<T, R> {
    /// C++ base class `ValueSource<R>`
    pub base: Unmapped /* ValueSource<R> */,
    /// The object the action is directed to.
    pub mySource: Ptr<T>,
    /// The object's operation to perform.
    pub myOperation: SUMOAbstractRouter_Operation,
    /// The scale to apply.
    pub myScale: R,
    _p: std::marker::PhantomData<(T, R,)>,
}

impl<T, R> std::ops::Deref for FunctionBinding<T, R> {
    type Target = Unmapped /* ValueSource<R> */;
    fn deref(&self) -> &Unmapped /* ValueSource<R> */ {
        &self.base
    }
}
impl<T, R> std::ops::DerefMut for FunctionBinding<T, R> {
    fn deref_mut(&mut self) -> &mut Unmapped /* ValueSource<R> */ {
        &mut self.base
    }
}

impl<T, R> FunctionBinding<T, R> {
    /// C++: `void FunctionBinding::FunctionBinding<T, R>(T *const, Operation, const R)`
    // FunctionBinding(T* const source, Operation operation, const R scale = 1) :
    //         mySource(source),
    //         myOperation(operation),
    //         myScale(scale) {}
    #[allow(unused_variables)]
    pub fn new(source: Ptr<T>, operation: SUMOAbstractRouter_Operation, scale: R) -> Self {
        todo!("port FunctionBinding::FunctionBinding<T, R>")
    }

    /// C++: `void FunctionBinding::~FunctionBinding<T, R>()`
    /// Destructor.
    // ~FunctionBinding() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port FunctionBinding::~FunctionBinding<T, R>")
    }

    /// C++: `R FunctionBinding::getValue() const` [const]
    // R getValue() const {
    //         return myScale * (mySource->*myOperation)();
    //     }
    #[allow(unused_variables)]
    pub fn get_value(&self) -> R {
        todo!("port FunctionBinding::getValue")
    }

    /// C++: `ValueSource<R> * FunctionBinding::copy() const` [const]
    // ValueSource<R>* copy() const {
    //         return new FunctionBinding<T, R>(mySource, myOperation, myScale);
    //     }
    #[allow(unused_variables)]
    pub fn copy(&self) -> Ptr<Unmapped /* ValueSource<R> */> {
        todo!("port FunctionBinding::copy")
    }

    /// C++: `ValueSource<double> * FunctionBinding::makedoubleReturningCopy() const` [const]
    // ValueSource<double>* makedoubleReturningCopy() const {
    //         return new CastingFunctionBinding<T, double, R>(mySource, myOperation, myScale);
    //     }
    #[allow(unused_variables)]
    pub fn makedouble_returning_copy(&self) -> Ptr<Unmapped /* ValueSource<double> */> {
        todo!("port FunctionBinding::makedoubleReturningCopy")
    }

    /// C++: `FunctionBinding<T, R> & FunctionBinding::operator=(const FunctionBinding<T, R> &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &FunctionBinding<T, R>) -> &mut FunctionBinding<T, R> {
        todo!("port FunctionBinding::operator=")
    }

}

/// C++ class `FunctionBindingString` (microsim/logging/FunctionBinding.h)
pub struct FunctionBindingString<T> {
    /// The object the action is directed to.
    pub mySource: Ptr<T>,
    /// The object's operation to perform.
    pub myOperation: SUMOAbstractRouter_Operation,
    _p: std::marker::PhantomData<(T,)>,
}

impl<T> FunctionBindingString<T> {
    /// C++: `void FunctionBindingString::FunctionBindingString<T>(T *const, Operation)`
    // FunctionBindingString(T* const source, Operation operation) :
    //         mySource(source),
    //         myOperation(operation)
    //     {}
    #[allow(unused_variables)]
    pub fn new(source: Ptr<T>, operation: SUMOAbstractRouter_Operation) -> Self {
        todo!("port FunctionBindingString::FunctionBindingString<T>")
    }

    /// C++: `void FunctionBindingString::~FunctionBindingString<T>()`
    /// Destructor.
    // ~FunctionBindingString() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port FunctionBindingString::~FunctionBindingString<T>")
    }

    /// C++: `int FunctionBindingString::getValue() const` [const]
    // std::string getValue() const {
    //         return (mySource->*myOperation)();
    //     }
    #[allow(unused_variables)]
    pub fn get_value(&self) -> i32 {
        todo!("port FunctionBindingString::getValue")
    }

    /// C++: `int * FunctionBindingString::copy() const` [const]
    // ValueSource<std::string>* copy() const {
    //         return new FunctionBindingString<T>(mySource, myOperation);
    //     }
    #[allow(unused_variables)]
    pub fn copy(&self) -> Ptr<i32> {
        todo!("port FunctionBindingString::copy")
    }

    /// C++: `ValueSource<double> * FunctionBindingString::makedoubleReturningCopy() const` [const]
    // ValueSource<double>* makedoubleReturningCopy() const {
    //         return nullptr;
    //     }
    #[allow(unused_variables)]
    pub fn makedouble_returning_copy(&self) -> Ptr<Unmapped /* ValueSource<double> */> {
        todo!("port FunctionBindingString::makedoubleReturningCopy")
    }

    /// C++: `FunctionBindingString<T> & FunctionBindingString::operator=(const FunctionBindingString<T> &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &FunctionBindingString<T>) -> &mut FunctionBindingString<T> {
        todo!("port FunctionBindingString::operator=")
    }

}

