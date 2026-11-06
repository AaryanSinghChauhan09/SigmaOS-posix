//! SigmaOS-posix — POSIX.1 compatibility layer.
//!
//! Provides the stable ABI surface userland (coreutils, shells, ports of
//! Arch/Debian tools) programs against:
//! - [`errno::Errno`]: POSIX errno constants with stable numeric values
//! - [`syscall::Syscall`]: syscall number registry (Linux x86_64 ABI)
//! - [`path::normalize_path`]: POSIX pathname resolution
//! - [`stat::Stat`]/[`stat::FileType`]: stat(2) result structures
//!
//! Mechanism only — policy (sandboxing, capability checks) lives above.

pub mod errno;
pub mod path;
pub mod stat;
pub mod syscall;

pub use errno::Errno;
pub use path::normalize_path;
pub use stat::{FileType, Stat};
pub use syscall::Syscall;

#[cfg(test)]
mod tests;
