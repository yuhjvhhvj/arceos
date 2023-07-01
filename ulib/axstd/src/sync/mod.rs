//! Useful synchronization primitives.

#[doc(no_inline)]
pub use core::sync::atomic;

#[cfg(feature = "alloc")]
#[doc(no_inline)]
pub use alloc::sync::{Arc, Weak};
