//! `OwnedRefCell` is similar to `RefCell`, but with a different borrowing mechanism. Unlike
//! `RefCell` which provides references tied to the lifetimes of the borrow scopes, `OwnedRefCell`
//! returns special owned references. These references keep the borrowed state until they are explicitly
//! dropped, offering a more flexible way to manage lifetimes especially in complex or dynamic
//! application structures such as event-driven systems or in scenarios where lifetime management
//! is more nuanced.
//!
//! The main class in this module, `OwnedRefCell<T>`, provides an interface similar to `RefCell<T>`,
//! allowing both mutable and immutable borrows, tracked at runtime to ensure that there are no value races.
//! `OwnedRefCell<T>` should be used when you need temporary mutable access to value inside a value
//! structure that does not itself provide intrinsic mutable access.
//!
//! # Differences from `RefCell`
//!
//! - `OwnedRefCell` provides `OwnedRef` and `OwnedRefMut`, which own their borrow status and thus
//!   do not require lifetime annotations.
//! - Borrowing rules are enforced at runtime as with `RefCell`, but `OwnedRefCell` uses owned types
//!   to manage the borrow state instead of lifetimes.
//! - While `RefCell` reacts at runtime with panics when a borrowing rule is violated,
//!   `OwnedRefCell` also offers methods (`try_borrow` and `try_borrow_mut`) that return `None` when
//!   a borrow would violate the rules, allowing the caller to react without forcing a panic.
//!
//! # Safety
//!
//! Unlike `RefCell<T>`, `OwnedRefCell<T>` uses `Rc<T>` to track the borrowing state, and thus it is not
//! thread-safe. It is meant for use only in single-threaded scenarios. Attempting to use `OwnedRefCell<T>`
//! in a multithreaded context may lead to value races and is not supported.
//!
//! # Examples
//!
//! Basic usage:
//!
//! ```
//! use owned_ref_cell::OwnedRefCell;
//! let cell = OwnedRefCell::new(42);
//!
//! {
//!     let value = cell.borrow();
//!     assert_eq!(*value, 42);
//!     // Cannot borrow mutably when already borrowed immutably
//!     assert!(cell.try_borrow_mut().is_none());
//! }
//!
//! {
//!     let mut value = cell.borrow_mut();
//!     *value = 45;
//! }
//!
//! {
//!     let value = cell.borrow();
//!     assert_eq!(*value, 45);
//! }
//! ```
//!
//! This module also provides:
//!
//! - `OwnedRef<T>`: an owned, immutable reference to the value inside an `OwnedRefCell<T>`.
//! - `OwnedRefMut<T>`: an owned, mutable reference to the value inside an `OwnedRefCell<T>`.

use std::cell::{RefCell, UnsafeCell};
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

/// Provides mutable or immutable access to encapsulated value with owned references.
pub struct OwnedRefCell<T> {
    value: UnsafeCell<T>,
    state: Rc<RefCell<BorrowState>>,
}

/// Internal state to keep track of the borrowing status.
struct BorrowState {
    is_writing: bool,
    reading_count: usize,
}

/// An immutable reference to the value within `OwnedRefCell`.
pub struct OwnedRef<T> {
    value: *const T,
    state: Rc<RefCell<BorrowState>>,
}

/// A mutable reference to the value within `OwnedRefCell`.
pub struct OwnedRefMut<T> {
    value: *mut T,
    state: Rc<RefCell<BorrowState>>,
}

impl<T> OwnedRefCell<T> {
    /// Constructs a new `OwnedRefCell` with the specified value.
    pub fn new(value: T) -> Self {
        OwnedRefCell {
            value: UnsafeCell::new(value),
            state: Rc::new(RefCell::new(BorrowState {
                is_writing: false,
                reading_count: 0,
            })),
        }
    }

    /// Borrows the cell immutably.
    /// Panics if the cell is already borrowed mutably.
    pub fn borrow(&self) -> OwnedRef<T> {
        self.try_borrow()
            .expect("Failed to borrow: already mutably borrowed")
    }

    /// Borrows the cell mutably.
    /// Panics if the cell is already borrowed immutably or mutably.
    pub fn borrow_mut(&self) -> OwnedRefMut<T> {
        self.try_borrow_mut()
            .expect("Failed to borrow mutably: already borrowed")
    }

    /// Tries to immutably borrow the cell.
    /// Returns `None` if the cell is already borrowed mutably.
    pub fn try_borrow(&self) -> Option<OwnedRef<T>> {
        let mut state = self.state.borrow_mut();
        if state.is_writing {
            None
        } else {
            state.reading_count += 1;
            Some(OwnedRef {
                value: self.value.get(),
                state: Rc::clone(&self.state),
            })
        }
    }

    /// Tries to mutably borrow the cell.
    /// Returns `None` if the cell is already borrowed immutably or mutably.
    pub fn try_borrow_mut(&self) -> Option<OwnedRefMut<T>> {
        let mut state = self.state.borrow_mut();
        if state.is_writing || state.reading_count > 0 {
            None
        } else {
            state.is_writing = true;
            Some(OwnedRefMut {
                value: self.value.get(),
                state: Rc::clone(&self.state),
            })
        }
    }
}

/// Implements `Deref` for `OwnedRef` to allow dereferencing the owned reference.
impl<T> Deref for OwnedRef<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.value }
    }
}

/// Implements `Deref` for `OwnedRefMut` to allow dereferencing the owned mutable reference.
impl<T> Deref for OwnedRefMut<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.value }
    }
}

/// Implements `DerefMut` for `OwnedRefMut` to allow dereferencing the owned mutable reference.
impl<T> DerefMut for OwnedRefMut<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.value }
    }
}

/// Implements `Drop` for `OwnedRef` and `OwnedRefMut` to update the borrowing state when the
/// references are dropped.
impl<T> Drop for OwnedRef<T> {
    fn drop(&mut self) {
        let mut state = self.state.borrow_mut();
        state.reading_count -= 1;
    }
}

/// Implements `Drop` for `OwnedRefMut` to update the borrowing state when the reference is dropped.
impl<T> Drop for OwnedRefMut<T> {
    fn drop(&mut self) {
        let mut state = self.state.borrow_mut();
        state.is_writing = false;
    }
}

#[cfg(test)]
mod tests {
    use std::panic::{self, AssertUnwindSafe};

    use super::*;

    #[test]
    fn borrow_mut_modify_and_borrow_after_drop() {
        let cell = OwnedRefCell::new(10);
        {
            let mut b = cell.borrow_mut();
            *b = 20;
        }
        let b = cell.borrow();
        assert_eq!(*b, 20);
    }

    #[test]
    fn cannot_borrow_mut_while_immutably_borrowed() {
        let cell = OwnedRefCell::new(10);
        let _b = cell.borrow();
        assert!(cell.try_borrow_mut().is_none());
    }

    #[test]
    fn cannot_borrow_while_mutably_borrowed() {
        let cell = OwnedRefCell::new(10);
        let _b = cell.borrow_mut();
        assert!(cell.try_borrow().is_none());
    }

    #[test]
    fn cannot_borrow_mut_while_mutably_borrowed() {
        let cell = OwnedRefCell::new(10);
        let _b = cell.borrow_mut();
        assert!(cell.try_borrow_mut().is_none());
    }

    #[test]
    fn multiple_immutable_borrows() {
        let cell = OwnedRefCell::new(10);
        let b1 = cell.try_borrow().unwrap();
        let b2 = cell.try_borrow().unwrap();
        assert_eq!(*b1, 10);
        assert_eq!(*b2, 10);
    }

    #[test]
    fn multiple_immutable_borrows_after_borrow_mut() {
        let cell = OwnedRefCell::new(10);
        {
            let _b1 = cell.borrow_mut();
        }
        let b2 = cell.borrow();
        let b3 = cell.borrow();
        assert_eq!(*b2, 10);
        assert_eq!(*b3, 10);
    }

    #[test]
    fn borrow_mut_again_after_drop() {
        let cell = OwnedRefCell::new(10);
        {
            let mut b = cell.borrow_mut();
            *b = 20;
        }
        {
            let mut b = cell.borrow_mut();
            *b = 30;
        }
        let b = cell.borrow();
        assert_eq!(*b, 30);
    }

    #[test]
    fn panic_on_borrow_when_already_borrowed_mutably() {
        let cell = OwnedRefCell::new(50);
        let _b1 = cell.borrow_mut();
        let cell_ref = AssertUnwindSafe(&cell);
        let result = panic::catch_unwind(move || {
            cell_ref.borrow();
        });
        assert!(result.is_err());
    }

    #[test]
    fn panic_on_borrow_mut_when_already_borrowed() {
        let cell = OwnedRefCell::new(50);
        let _b1 = cell.borrow();
        let cell_ref = AssertUnwindSafe(&cell);
        let result = panic::catch_unwind(move || {
            cell_ref.borrow_mut();
        });
        assert!(result.is_err());
    }
}
