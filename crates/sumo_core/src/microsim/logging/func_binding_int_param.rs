//! Port of microsim/logging/FuncBinding_IntParam.h
#![allow(non_snake_case, non_camel_case_types)]
use crate::utils::value_source::{CastFrom, ValueSource};
use utils::common::ptr::Ptr;

/// C++ typedef `R (T::*)(int) const FuncBinding_IntParam::Operation`
pub type FuncBinding_IntParam_Operation<T, R> = fn(&T, i32) -> R;

/// C++ class `FuncBinding_IntParam<T, R>` (microsim/logging/FuncBinding_IntParam.h)
pub struct FuncBinding_IntParam<T, R> {
    /// The object the action is directed to.
    pub mySource: Ptr<T>,
    /// The object's operation to perform.
    pub myOperation: FuncBinding_IntParam_Operation<T, R>,
    /// The parameter passed to the operation.
    pub myParam: i32,
}

impl<T, R> FuncBinding_IntParam<T, R> {
    /// C++: `FuncBinding_IntParam(T* source, Operation operation, int param)`
    pub fn new(source: Ptr<T>, operation: FuncBinding_IntParam_Operation<T, R>, param: i32) -> Self {
        FuncBinding_IntParam { mySource: source, myOperation: operation, myParam: param }
    }
}

impl<T: 'static, R: 'static> ValueSource<R> for FuncBinding_IntParam<T, R>
where
    f64: CastFrom<R>,
{
    /// C++: `double getValue() const { return (mySource->*myOperation)(myParam); }` (declared double: only R = double compiles in C++)
    fn get_value(&self) -> R {
        (self.myOperation)(&*self.mySource, self.myParam)
    }

    /// C++: `return new FuncBinding_IntParam<T, R>(mySource, myOperation, myParam);`
    fn copy(&self) -> Box<dyn ValueSource<R>> {
        Box::new(FuncBinding_IntParam::new(self.mySource, self.myOperation, self.myParam))
    }

    /// C++: `return new FuncBinding_IntParam<T, double>(mySource, myOperation, myParam);` (the R result converts to double)
    fn makedouble_returning_copy(&self) -> Option<Box<dyn ValueSource<f64>>> {
        Some(Box::new(DoubleCopy { inner: FuncBinding_IntParam::new(self.mySource, self.myOperation, self.myParam) }))
    }
}

/// the `FuncBinding_IntParam<T, double>` copy over an operation returning R (C++ converts R to double implicitly)
struct DoubleCopy<T, R> {
    inner: FuncBinding_IntParam<T, R>,
}

impl<T: 'static, R: 'static> ValueSource<f64> for DoubleCopy<T, R>
where
    f64: CastFrom<R>,
{
    fn get_value(&self) -> f64 {
        f64::cast_from((self.inner.myOperation)(&*self.inner.mySource, self.inner.myParam))
    }
    fn copy(&self) -> Box<dyn ValueSource<f64>> {
        Box::new(DoubleCopy { inner: FuncBinding_IntParam::new(self.inner.mySource, self.inner.myOperation, self.inner.myParam) })
    }
    fn makedouble_returning_copy(&self) -> Option<Box<dyn ValueSource<f64>>> {
        Some(self.copy())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Src;
    fn twice(_: &Src, i: i32) -> f64 {
        2.0 * i as f64
    }

    #[test]
    fn calls_with_int_param() {
        let mut src = Box::new(Src);
        let b = FuncBinding_IntParam::new(Ptr::from_mut(&mut *src), twice, 21);
        assert_eq!(b.get_value(), 42.0);
        assert_eq!(b.copy().get_value(), 42.0);
        assert_eq!(b.makedouble_returning_copy().unwrap().get_value(), 42.0);
    }
}
