//! # The ArceOS Standard Library
//!
//! The [ArceOS] Standard Library is a mini-std library, with an interface similar
//! to rust [std], but calling the functions directly in ArceOS modules, instead
//! of using libc and system calls.
//!
//! ## Cargo Features
//!
//! These features are exactly the same as those in [axfeat], they are used to
//! provide users with the selection of features in axfeat, without import
//! [axfeat] additionally:
//!
//! - CPU
//!     - `smp`: Enable SMP (symmetric multiprocessing) support.
//!     - `fp_simd`: Enable floating point and SIMD support.
//! - Interrupts:
//!     - `irq`: Enable interrupt handling support.
//! - Memory
//!     - `alloc`: Enable dynamic memory allocation.
//!     - `paging`: Enable page table manipulation.
//! - Task management
//!     - `multitask`: Enable multi-threading support.
//!     - `sched_fifo`: Use the FIFO cooperative scheduler.
//!     - `sched_rr`: Use the Round-robin preemptive scheduler.
//!     - `sched_cfs`: Use the Completely Fair Scheduler (CFS) preemptive scheduler.
//! - Device and upperlayer stack
//!     - `fs`: Enable file system support.
//!     - `use-ramdisk`: Use the RAM disk to emulate the block device.
//!     - `net`: Enable networking support.
//!     - `display`: Enable graphics support.
//!     - `bus-mmio`: Use device tree to probe all MMIO devices.
//!     - `bus-pci`: Use PCI bus to probe all PCI devices.
//! - Logging
//!     - `log-level-off`: Disable all logging.
//!     - `log-level-error`, `log-level-warn`, `log-level-info`, `log-level-debug`,
//!       `log-level-trace`: Keep logging only at the specified level or higher.
//! - Platform
//!     - `platform-pc-x86`: Specify for use on the corresponding platform.
//!     - `platform-qemu-virt-riscv`: Specify for use on the corresponding platform.
//!     - `platform-qemu-virt-aarch64`: Specify for use on the corresponding platform.
//!     - `platform-raspi4-aarch64`: Specify for use on the corresponding platform.
//!
//! [ArceOS]: https://github.com/rcore-os/arceos

#![cfg_attr(all(not(test), not(doc)), no_std)]
#![feature(doc_cfg)]
#![feature(doc_auto_cfg)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
#[doc(no_inline)]
pub use alloc::{boxed, collections, format, string, vec};

#[doc(no_inline)]
pub use core::{arch, cell, cmp, hint, marker, mem, ops, ptr, slice, str};

pub mod io;
pub mod time;
