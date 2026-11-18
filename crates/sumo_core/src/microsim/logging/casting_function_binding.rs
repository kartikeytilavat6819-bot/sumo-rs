//! Port of microsim/logging/CastingFunctionBinding.h
#![allow(non_snake_case, non_camel_case_types)]
use crate::utils::value_source::{CastFrom, ValueSource};
use utils::common::ptr::Ptr;

/// C++ typedef `O (T::*)() const CastingFunctionBinding::Operation`
pub type CastingFunctionBinding_Operation<T, O> = fn(&T) -> O;

/// C++ class `CastingFunctionBinding<T, R, O>` (microsim/logging/CastingFunctionBinding.h): calls a getter returning O and casts it to R
pub struct CastingFunctionBinding<T, R, O> {
    /// The object the action is directed to.
    pub mySource: Ptr<T>,
    /// The object's operation to perform.
    pub myOperation: CastingFunctionBinding_Operation<T, O>,
    /// The scale to apply.
    pub myScale: R,
}

impl<T, R, O> CastingFunctionBinding<T, R, O> {
    /// C++: `CastingFunctionBinding(T* source, Operation operation, const R scale = 1)`
    pub fn new(source: Ptr<T>, operation: CastingFunctionBinding_Operation<T, O>, scale: R) -> Self {
        CastingFunctionBinding { mySource: source, myOperation: operation, myScale: scale }
    }
}

impl<T: 'static, R: Copy + std::ops::Mul<Output = R> + CastFrom<O> + 'static, O: 'static> ValueSource<R> for CastingFunctionBinding<T, R, O>
where
    f64: CastFrom<O> + CastFrom<R>,
{
    /// C++: `return myScale * (R)(mySource->*myOperation)();`
    fn get_value(&self) -> R {
        self.myScale * R::cast_from((self.myOperation)(&*self.mySource))
    }

    /// C++: `return new CastingFunctionBinding<T, R, O>(mySource, myOperation, myScale);`
    fn copy(&self) -> Box<dyn ValueSource<R>> {
        Box::new(CastingFunctionBinding::<T, R, O>::new(self.mySource, self.myOperation, self.myScale))
    }

    /// C++: `return new CastingFunctionBinding<T, double, O>(mySource, myOperation, myScale);`
    fn makedouble_returning_copy(&self) -> Option<Box<dyn ValueSource<f64>>> {
        Some(Box::new(CastingFunctionBinding::<T, f64, O>::new(self.mySource, self.myOperation, f64::cast_from(self.myScale))))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Src(f64);
    fn get(s: &Src) -> f64 {
        s.0
    }

    #[test]
    fn casts_before_scaling() {
        let mut src = Box::new(Src(2.75));
        // (int)2.75 == 2, times scale 3
        let b = CastingFunctionBinding::<Src, i32, f64>::new(Ptr::from_mut(&mut *src), get, 3);
        assert_eq!(b.get_value(), 6);
        assert_eq!(b.copy().get_value(), 6);
        assert_eq!(b.makedouble_returning_copy().unwrap().get_value(), 8.25);
    }
}
