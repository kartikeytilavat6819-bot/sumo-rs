//! Port of microsim/logging/FuncBinding_StringParam.h
#![allow(non_snake_case, non_camel_case_types)]
use crate::utils::value_source::ValueSource;
use utils::common::ptr::Ptr;

/// C++ typedef `R (T::*)(const std::string&) const FuncBinding_StringParam::Operation`
pub type FuncBinding_StringParam_Operation<T, R> = fn(&T, &str) -> R;

/// C++ class `FuncBinding_StringParam<T, R>` (microsim/logging/FuncBinding_StringParam.h)
pub struct FuncBinding_StringParam<T, R> {
    /// The object the action is directed to.
    pub mySource: Ptr<T>,
    /// The object's operation to perform.
    pub myOperation: FuncBinding_StringParam_Operation<T, R>,
    /// The parameter passed to the operation.
    pub myParam: String,
}

impl<T, R> FuncBinding_StringParam<T, R> {
    /// C++: `FuncBinding_StringParam(T* source, Operation operation, const std::string& param)`
    pub fn new(source: Ptr<T>, operation: FuncBinding_StringParam_Operation<T, R>, param: &str) -> Self {
        FuncBinding_StringParam { mySource: source, myOperation: operation, myParam: param.to_string() }
    }
}

impl<T: 'static, R: 'static> ValueSource<R> for FuncBinding_StringParam<T, R> {
    /// C++: `return (mySource->*myOperation)(myParam);`
    fn get_value(&self) -> R {
        (self.myOperation)(&*self.mySource, &self.myParam)
    }

    /// C++: `return new FuncBinding_StringParam<T, R>(mySource, myOperation, myParam);`
    fn copy(&self) -> Box<dyn ValueSource<R>> {
        Box::new(FuncBinding_StringParam::new(self.mySource, self.myOperation, &self.myParam))
    }

    /// C++: `throw 1;`
    fn makedouble_returning_copy(&self) -> Option<Box<dyn ValueSource<f64>>> {
        std::panic::panic_any(1i32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Src;
    fn len(_: &Src, s: &str) -> usize {
        s.len()
    }

    #[test]
    fn calls_with_string_param() {
        let mut src = Box::new(Src);
        let b = FuncBinding_StringParam::new(Ptr::from_mut(&mut *src), len, "abc");
        assert_eq!(b.get_value(), 3);
        assert_eq!(b.copy().get_value(), 3);
    }
}
