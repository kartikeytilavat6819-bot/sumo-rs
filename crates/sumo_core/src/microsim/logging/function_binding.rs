//! Port of microsim/logging/FunctionBinding.h
#![allow(non_snake_case, non_camel_case_types)]
use crate::microsim::logging::casting_function_binding::CastingFunctionBinding;
use crate::utils::value_source::{CastFrom, ValueSource};
use utils::common::ptr::Ptr;

/// C++ typedef `R (T::*)() const FunctionBinding::Operation`
pub type FunctionBinding_Operation<T, R> = fn(&T) -> R;

/// C++ typedef `std::string (T::*)() const FunctionBindingString::Operation`
pub type FunctionBindingString_Operation<T> = fn(&T) -> String;

/// C++ class `FunctionBinding<T, R>` (microsim/logging/FunctionBinding.h): a ValueSource<R> calling a getter of T
pub struct FunctionBinding<T, R> {
    /// The object the action is directed to.
    pub mySource: Ptr<T>,
    /// The object's operation to perform.
    pub myOperation: FunctionBinding_Operation<T, R>,
    /// The scale to apply.
    pub myScale: R,
}

impl<T, R> FunctionBinding<T, R> {
    /// C++: `FunctionBinding(T* const source, Operation operation, const R scale = 1)`
    pub fn new(source: Ptr<T>, operation: FunctionBinding_Operation<T, R>, scale: R) -> Self {
        FunctionBinding { mySource: source, myOperation: operation, myScale: scale }
    }
}

impl<T: 'static, R: Copy + std::ops::Mul<Output = R> + 'static> ValueSource<R> for FunctionBinding<T, R>
where
    f64: CastFrom<R>,
{
    /// C++: `return myScale * (mySource->*myOperation)();`
    fn get_value(&self) -> R {
        self.myScale * (self.myOperation)(&*self.mySource)
    }

    /// C++: `return new FunctionBinding<T, R>(mySource, myOperation, myScale);`
    fn copy(&self) -> Box<dyn ValueSource<R>> {
        Box::new(FunctionBinding::new(self.mySource, self.myOperation, self.myScale))
    }

    /// C++: `return new CastingFunctionBinding<T, double, R>(mySource, myOperation, myScale);`
    fn makedouble_returning_copy(&self) -> Option<Box<dyn ValueSource<f64>>> {
        Some(Box::new(CastingFunctionBinding::<T, f64, R>::new(self.mySource, self.myOperation, f64::cast_from(self.myScale))))
    }
}

/// C++ class `FunctionBindingString<T>` (microsim/logging/FunctionBinding.h)
pub struct FunctionBindingString<T> {
    /// The object the action is directed to.
    pub mySource: Ptr<T>,
    /// The object's operation to perform.
    pub myOperation: FunctionBindingString_Operation<T>,
}

impl<T> FunctionBindingString<T> {
    /// C++: `FunctionBindingString(T* const source, Operation operation)`
    pub fn new(source: Ptr<T>, operation: FunctionBindingString_Operation<T>) -> Self {
        FunctionBindingString { mySource: source, myOperation: operation }
    }
}

impl<T: 'static> ValueSource<String> for FunctionBindingString<T> {
    /// C++: `return (mySource->*myOperation)();`
    fn get_value(&self) -> String {
        (self.myOperation)(&*self.mySource)
    }

    /// C++: `return new FunctionBindingString<T>(mySource, myOperation);`
    fn copy(&self) -> Box<dyn ValueSource<String>> {
        Box::new(FunctionBindingString::new(self.mySource, self.myOperation))
    }

    /// C++: `return nullptr;`
    fn makedouble_returning_copy(&self) -> Option<Box<dyn ValueSource<f64>>> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Src {
        v: i32,
        name: String,
    }
    fn get_v(s: &Src) -> i32 {
        s.v
    }
    fn get_name(s: &Src) -> String {
        s.name.clone()
    }

    #[test]
    fn scaled_value_copy_and_double_copy() {
        let mut src = Box::new(Src { v: 7, name: "a".into() });
        let b = FunctionBinding::new(Ptr::from_mut(&mut *src), get_v, 3);
        assert_eq!(b.get_value(), 21);
        assert_eq!(b.copy().get_value(), 21);
        src.v = 2;
        // the double copy casts the getter result and the scale
        assert_eq!(b.makedouble_returning_copy().unwrap().get_value(), 6.0);
        let s = FunctionBindingString::new(Ptr::from_mut(&mut *src), get_name);
        assert_eq!(s.copy().get_value(), "a");
        assert!(s.makedouble_returning_copy().is_none());
    }
}
