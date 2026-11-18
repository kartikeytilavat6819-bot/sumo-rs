//! Port of microsim/logging/CastingFunctionBinding_Param.h
#![allow(non_snake_case, non_camel_case_types)]
use crate::utils::value_source::{CastFrom, ValueSource};
use utils::common::ptr::Ptr;

/// C++ typedef `O (T::*)(P) const CastingFunctionBinding_Param::Operation`
pub type CastingFunctionBinding_Param_Operation<T, O, P> = fn(&T, P) -> O;

/// C++ class `CastingFunctionBinding_Param<T, R, O, P>` (microsim/logging/CastingFunctionBinding_Param.h)
pub struct CastingFunctionBinding_Param<T, R, O, P> {
    /// The object the action is directed to.
    pub mySource: Ptr<T>,
    /// The object's operation to perform.
    pub myOperation: CastingFunctionBinding_Param_Operation<T, O, P>,
    /// The scale to apply.
    pub myScale: R,
    /// The parameter passed to the operation.
    pub myParam: P,
}

impl<T, R, O, P> CastingFunctionBinding_Param<T, R, O, P> {
    /// C++: `CastingFunctionBinding_Param(T* source, Operation operation, P param, const R scale = 1.0)`
    pub fn new(source: Ptr<T>, operation: CastingFunctionBinding_Param_Operation<T, O, P>, param: P, scale: R) -> Self {
        CastingFunctionBinding_Param { mySource: source, myOperation: operation, myScale: scale, myParam: param }
    }
}

impl<T: 'static, R: Copy + std::ops::Mul<Output = R> + CastFrom<O> + 'static, O: 'static, P: Clone + 'static> ValueSource<R> for CastingFunctionBinding_Param<T, R, O, P>
where
    f64: CastFrom<O> + CastFrom<R>,
{
    /// C++: `return myScale * (R)(mySource->*myOperation)(myParam);`
    fn get_value(&self) -> R {
        self.myScale * R::cast_from((self.myOperation)(&*self.mySource, self.myParam.clone()))
    }

    /// C++: `return new CastingFunctionBinding_Param<T, R, O, P>(mySource, myOperation, myParam, myScale);`
    fn copy(&self) -> Box<dyn ValueSource<R>> {
        Box::new(CastingFunctionBinding_Param::<T, R, O, P>::new(self.mySource, self.myOperation, self.myParam.clone(), self.myScale))
    }

    /// C++: `return new CastingFunctionBinding_Param<T, double, O, P>(mySource, myOperation, myParam, myScale);`
    fn makedouble_returning_copy(&self) -> Option<Box<dyn ValueSource<f64>>> {
        Some(Box::new(CastingFunctionBinding_Param::<T, f64, O, P>::new(self.mySource, self.myOperation, self.myParam.clone(), f64::cast_from(self.myScale))))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Src(Vec<i32>);
    fn at(s: &Src, i: usize) -> i32 {
        s.0[i]
    }

    #[test]
    fn passes_param_and_scales() {
        let mut src = Box::new(Src(vec![4, 9]));
        let b = CastingFunctionBinding_Param::<Src, f64, i32, usize>::new(Ptr::from_mut(&mut *src), at, 1, 0.5);
        assert_eq!(b.get_value(), 4.5);
        assert_eq!(b.copy().get_value(), 4.5);
        assert_eq!(b.makedouble_returning_copy().unwrap().get_value(), 4.5);
    }
}
