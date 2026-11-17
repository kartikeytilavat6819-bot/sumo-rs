//! Port of utils/common/ValueSource.h

/// C++ class `ValueSource<T>` (utils/common/ValueSource.h): a source of values of type T
pub trait ValueSource<T> {
    /// C++: `virtual T getValue() const = 0`
    fn get_value(&self) -> T;
    /// C++: `virtual ValueSource* copy() const = 0`
    fn copy(&self) -> Box<dyn ValueSource<T>>;
    /// C++: `virtual ValueSource<double>* makedoubleReturningCopy() const = 0` (nullptr is None)
    fn makedouble_returning_copy(&self) -> Option<Box<dyn ValueSource<f64>>>;
}

/// C++ value cast `(R)x` between the arithmetic types the bindings use
pub trait CastFrom<O> {
    fn cast_from(o: O) -> Self;
}

macro_rules! cast_from_all {
    ($r:ty: $($o:ty),*) => { $(impl CastFrom<$o> for $r { fn cast_from(o: $o) -> $r { o as $r } })* };
}
cast_from_all!(f64: f64, f32, i32, i64, u32, usize);
cast_from_all!(f32: f64, f32, i32, i64, u32, usize);
cast_from_all!(i32: f64, f32, i32, i64, u32, usize);
cast_from_all!(i64: f64, f32, i32, i64, u32, usize);
cast_from_all!(u32: f64, f32, i32, i64, u32, usize);
cast_from_all!(usize: f64, f32, i32, i64, u32, usize);
