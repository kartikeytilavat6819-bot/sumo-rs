//! Port of microsim/logging/FuncBinding_IntParam.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `R (T::*)(int) const FuncBinding_IntParam::Operation`
pub type FuncBinding_IntParam_Operation = Unmapped /* R (T::*)(int) */;

/// C++ class `FuncBinding_IntParam` (microsim/logging/FuncBinding_IntParam.h)
pub struct FuncBinding_IntParam<T, R> {
    /// C++ base class `ValueSource<R>`
    pub base: Unmapped /* ValueSource<R> */,
    /// The object the action is directed to.
    pub mySource: Ptr<T>,
    /// The object's operation to perform.
    pub myOperation: SUMOAbstractRouter_Operation,
    pub myParam: i32,
    _p: std::marker::PhantomData<(T, R,)>,
}

impl<T, R> std::ops::Deref for FuncBinding_IntParam<T, R> {
    type Target = Unmapped /* ValueSource<R> */;
    fn deref(&self) -> &Unmapped /* ValueSource<R> */ {
        &self.base
    }
}
impl<T, R> std::ops::DerefMut for FuncBinding_IntParam<T, R> {
    fn deref_mut(&mut self) -> &mut Unmapped /* ValueSource<R> */ {
        &mut self.base
    }
}

impl<T, R> FuncBinding_IntParam<T, R> {
    /// C++: `void FuncBinding_IntParam::FuncBinding_IntParam<T, R>(T *, Operation, int)`
    // FuncBinding_IntParam(T* source, Operation operation,
    //                          int param)
    //         :
    //         mySource(source),
    //         myOperation(operation),
    //         myParam(param) {}
    #[allow(unused_variables)]
    pub fn new(source: Ptr<T>, operation: SUMOAbstractRouter_Operation, param: i32) -> Self {
        todo!("port FuncBinding_IntParam::FuncBinding_IntParam<T, R>")
    }

    /// C++: `void FuncBinding_IntParam::~FuncBinding_IntParam<T, R>()`
    /// Destructor.
    // ~FuncBinding_IntParam() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port FuncBinding_IntParam::~FuncBinding_IntParam<T, R>")
    }

    /// C++: `double FuncBinding_IntParam::getValue() const` [const]
    // double getValue() const {
    //         return (mySource->*myOperation)(myParam);
    //     }
    #[allow(unused_variables)]
    pub fn get_value(&self) -> f64 {
        todo!("port FuncBinding_IntParam::getValue")
    }

    /// C++: `ValueSource<R> * FuncBinding_IntParam::copy() const` [const]
    // ValueSource<R>* copy() const {
    //         return new FuncBinding_IntParam<T, R>(
    //                    mySource, myOperation, myParam);
    //     }
    #[allow(unused_variables)]
    pub fn copy(&self) -> Ptr<Unmapped /* ValueSource<R> */> {
        todo!("port FuncBinding_IntParam::copy")
    }

    /// C++: `ValueSource<double> * FuncBinding_IntParam::makedoubleReturningCopy() const` [const]
    // ValueSource<double>* makedoubleReturningCopy() const {
    //         return new FuncBinding_IntParam<T, double>(mySource, myOperation, myParam);
    //     }
    #[allow(unused_variables)]
    pub fn makedouble_returning_copy(&self) -> Ptr<Unmapped /* ValueSource<double> */> {
        todo!("port FuncBinding_IntParam::makedoubleReturningCopy")
    }

}

