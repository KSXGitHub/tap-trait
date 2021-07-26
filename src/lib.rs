//! Inspect and mutate values without leaving the method chain.
//!
//! **Example:**
//!
//! ```
//! use tap_trait::Tap;
//! use pipe_trait::Pipe;
//! let result = 2i32
//!     .tap(|x| assert_eq!(x, 2))
//!     .tap_mut(|x| *x += 1)
//!     .tap(|x| assert_eq!(x, 3))
//!     .tap_mut(|x| *x *= 3)
//!     .tap(|x| assert_eq!(x, 9))
//!     .pipe(|x| -x)
//!     .tap(|x| assert_eq!(x, -9))
//!     .pipe_ref(ToString::to_string)
//!     .tap_ref(|x| assert_eq!(x, "-9"))
//!     .tap_mut(|x| *x += ".0")
//!     .tap_ref(|x| assert_eq!(x, "-9.0"));
//! assert_eq!(result, "-9.0");
//! ```

#![no_std]
use core::{
    borrow::{Borrow, BorrowMut},
    ops::{Deref, DerefMut},
};

/// All sized types implement this trait.
pub trait Tap: Sized {
    #[inline]
    fn tap<Function>(self, f: Function) -> Self
    where
        Self: Copy,
        Function: FnOnce(Self),
    {
        f(self);
        self
    }

    #[inline]
    fn tap_ref<Function>(self, f: Function) -> Self
    where
        Function: FnOnce(&Self),
    {
        f(&self);
        self
    }

    #[inline]
    fn tap_mut<Function>(mut self, f: Function) -> Self
    where
        Function: FnOnce(&mut Self),
    {
        f(&mut self);
        self
    }

    #[inline]
    fn tap_as_ref<Param, Function>(self, f: Function) -> Self
    where
        Self: AsRef<Param>,
        Param: ?Sized,
        Function: FnOnce(&Param),
    {
        f(self.as_ref());
        self
    }

    #[inline]
    fn tap_as_mut<Param, Function>(mut self, f: Function) -> Self
    where
        Self: AsMut<Param>,
        Param: ?Sized,
        Function: FnOnce(&mut Param),
    {
        f(self.as_mut());
        self
    }

    #[inline]
    fn tap_deref<Param, Function>(self, f: Function) -> Self
    where
        Self: Deref<Target = Param>,
        Param: ?Sized,
        Function: FnOnce(&Param),
    {
        f(&self);
        self
    }

    #[inline]
    fn tap_deref_mut<Param, Function>(mut self, f: Function) -> Self
    where
        Self: DerefMut<Target = Param>,
        Param: ?Sized,
        Function: FnOnce(&mut Param),
    {
        f(&mut self);
        self
    }

    #[inline]
    fn tap_borrow<Param, Function>(self, f: Function) -> Self
    where
        Self: Borrow<Param>,
        Param: ?Sized,
        Function: FnOnce(&Param),
    {
        f(self.borrow());
        self
    }

    #[inline]
    fn tap_borrow_mut<Param, Function>(mut self, f: Function) -> Self
    where
        Self: BorrowMut<Param>,
        Param: ?Sized,
        Function: FnOnce(&Param),
    {
        f(self.borrow_mut());
        self
    }
}

impl<X> Tap for X {}
