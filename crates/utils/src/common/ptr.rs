//! `Ptr<T>`: the Rust stand-in for a C++ `T*` / `const T*` in ported code.
//!
//! SUMO's object graph (vehicles <-> lanes <-> edges <-> links ...) is built from raw pointers that
//! are freely shared and mutated. Porting it faithfully means keeping pointer semantics:
//! nullable, copyable, compared/ordered/hashed by address (so `std::set<T*>` order is preserved).
//! Objects are owned elsewhere (usually `Box<T>` in a container that lives for the whole simulation),
//! exactly like the C++ code. Dereferencing a dangling `Ptr` is undefined behaviour, as in C++.
// shortcut: unsafe raw pointers mirror C++ ownership; replace with safe ownership per module once its output matches SUMO.
use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut};

pub struct Ptr<T: ?Sized>(*mut T);

impl<T> Ptr<T> {
    pub const fn null() -> Self {
        Ptr(std::ptr::null_mut())
    }
}
impl<T: ?Sized> Ptr<T> {
    pub fn new(r: &T) -> Self {
        Ptr(r as *const T as *mut T)
    }
    pub fn from_mut(r: &mut T) -> Self {
        Ptr(r as *mut T)
    }
    pub fn from_box(b: &Box<T>) -> Self {
        Ptr(&**b as *const T as *mut T)
    }
    pub fn from_raw(p: *mut T) -> Self {
        Ptr(p)
    }
    pub fn as_raw(self) -> *mut T {
        self.0
    }
    pub fn is_null(self) -> bool {
        self.0.is_null()
    }
    /// C++ `p != nullptr ? p : ...` helper
    pub fn get(self) -> Option<&'static T> {
        unsafe { self.0.as_ref() }
    }
    pub fn get_mut(self) -> Option<&'static mut T> {
        unsafe { self.0.as_mut() }
    }
    /// Mutable access through a shared pointer, like calling a non-const method via `T*`.
    #[allow(clippy::mut_from_ref)]
    pub fn m(&self) -> &mut T {
        debug_assert!(!self.0.is_null(), "null Ptr dereference");
        unsafe { &mut *self.0 }
    }
    fn addr(self) -> usize {
        self.0 as *mut () as usize
    }
}
impl<T: ?Sized> Clone for Ptr<T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<T: ?Sized> Copy for Ptr<T> {}
impl<T> Default for Ptr<T> {
    fn default() -> Self {
        Ptr::null()
    }
}
impl<T: ?Sized> Deref for Ptr<T> {
    type Target = T;
    fn deref(&self) -> &T {
        debug_assert!(!self.0.is_null(), "null Ptr dereference");
        unsafe { &*self.0 }
    }
}
impl<T: ?Sized> DerefMut for Ptr<T> {
    fn deref_mut(&mut self) -> &mut T {
        debug_assert!(!self.0.is_null(), "null Ptr dereference");
        unsafe { &mut *self.0 }
    }
}
impl<T: ?Sized> PartialEq for Ptr<T> {
    fn eq(&self, o: &Self) -> bool {
        self.addr() == o.addr()
    }
}
impl<T: ?Sized> Eq for Ptr<T> {}
impl<T: ?Sized> PartialOrd for Ptr<T> {
    fn partial_cmp(&self, o: &Self) -> Option<Ordering> {
        Some(self.cmp(o))
    }
}
impl<T: ?Sized> Ord for Ptr<T> {
    // determinism: C++ orders std::set<T*> by address too; output that depends on it is
    // nondeterministic in SUMO itself, so tests don't rely on it.
    fn cmp(&self, o: &Self) -> Ordering {
        self.addr().cmp(&o.addr())
    }
}
impl<T: ?Sized> Hash for Ptr<T> {
    fn hash<H: Hasher>(&self, h: &mut H) {
        self.addr().hash(h)
    }
}
impl<T: ?Sized> fmt::Debug for Ptr<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Ptr({:#x})", self.addr())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pointer_semantics() {
        let mut b = Box::new(5);
        let p = Ptr::from_mut(&mut *b);
        let q = p;
        *q.m() += 1;
        assert_eq!(*p, 6);
        assert_eq!(p, q);
        assert!(Ptr::<i32>::null().is_null());
        assert!(Ptr::<i32>::null().get().is_none());
    }
}
