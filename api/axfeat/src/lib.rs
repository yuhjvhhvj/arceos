//! Top-level feature selection for [ArceOS].
//!
//! # Cargo Features
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

#![no_std]
