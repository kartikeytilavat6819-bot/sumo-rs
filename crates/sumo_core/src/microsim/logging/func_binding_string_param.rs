//! Port of microsim/logging/FuncBinding_StringParam.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `R (T::*)(const int &) const FuncBinding_StringParam::Operation`
pub type FuncBinding_StringParam_Operation = Unmapped /* R (T::*)(const int &) */;

/// C++ class `FuncBinding_StringParam` (microsim/logging/FuncBinding_StringParam.h)
pub struct FuncBinding_StringParam<T, R> {
    /// C++ base class `ValueSource<R>`
    pub base: Unmapped /* ValueSource<R> */,
    /// The object the action is directed to.
    pub mySource: Ptr<T>,
    /// The object's operation to perform.
    pub myOperation: SUMOAbstractRouter_Operation,
    pub myParam: i32,
    _p: std::marker::PhantomData<(T, R,)>,
}

impl<T, R> std::ops::Deref for FuncBinding_StringParam<T, R> {
    type Target = Unmapped /* ValueSource<R> */;
    fn deref(&self) -> &Unmapped /* ValueSource<R> */ {
        &self.base
    }
}
impl<T, R> std::ops::DerefMut for FuncBinding_StringParam<T, R> {
    fn deref_mut(&mut self) -> &mut Unmapped /* ValueSource<R> */ {
        &mut self.base
    }
}

impl<T, R> FuncBinding_StringParam<T, R> {
    /// C++: `void FuncBinding_StringParam::FuncBinding_StringParam<T, R>(T *, Operation, const int &)`
    // FuncBinding_StringParam(T* source, Operation operation,
    //                             const std::string& param)
    //         :
    //         mySource(source),
    //         myOperation(operation),
    //         myParam(param) {}
    #[allow(unused_variables)]
    pub fn new(source: Ptr<T>, operation: SUMOAbstractRouter_Operation, param: &i32) -> Self {
        todo!("port FuncBinding_StringParam::FuncBinding_StringParam<T, R>")
    }

    /// C++: `void FuncBinding_StringParam::~FuncBinding_StringParam<T, R>()`
    /// Destructor.
    // ~FuncBinding_StringParam() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port FuncBinding_StringParam::~FuncBinding_StringParam<T, R>")
    }

    /// C++: `R FuncBinding_StringParam::getValue() const` [const]
    // R getValue() const {
    //         return (mySource->*myOperation)(myParam);
    //     }
    #[allow(unused_variables)]
    pub fn get_value(&self) -> R {
        todo!("port FuncBinding_StringParam::getValue")
    }

    /// C++: `ValueSource<R> * FuncBinding_StringParam::copy() const` [const]
    // ValueSource<R>* copy() const {
    //         return new FuncBinding_StringParam<T, R>(
    //                    mySource, myOperation, myParam);
    //     }
    #[allow(unused_variables)]
    pub fn copy(&self) -> Ptr<Unmapped /* ValueSource<R> */> {
        todo!("port FuncBinding_StringParam::copy")
    }

    /// C++: `ValueSource<double> * FuncBinding_StringParam::makedoubleReturningCopy() const` [const]
    // ValueSource<double>* makedoubleReturningCopy() const {
    //         throw 1;
    //     }
    #[allow(unused_variables)]
    pub fn makedouble_returning_copy(&self) -> Ptr<Unmapped /* ValueSource<double> */> {
        todo!("port FuncBinding_StringParam::makedoubleReturningCopy")
    }

}

