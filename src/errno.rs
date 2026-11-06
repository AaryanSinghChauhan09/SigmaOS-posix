//! POSIX errno constants and descriptions.

use std::fmt;

/// POSIX errno — stable numeric values per POSIX.1-2017 / Linux asm-generic.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum Errno {
    Perm = 1,
    NoEnt = 2,
    Srch = 3,
    Intr = 4,
    Io = 5,
    Nxio = 6,
    Again = 11,
    Access = 13,
    Fault = 14,
    Busy = 16,
    Exist = 17,
    Xdev = 18,
    Nodev = 19,
    Notdir = 20,
    Isdir = 21,
    Inval = 22,
    Fbigi = 27,
    Nospc = 28,
    Pipe = 32,
    Range = 34,
    Nametoolong = 36,
    Loop = 40,
    Notempty = 39,
    Mfile = 24,
    Nfile = 23,
    Overflow = 75,
    Unknown = -1,
}

impl Errno {
    pub fn from_raw(n: i32) -> Self {
        // SAFETY-free: match on known values, else Unknown.
        match n {
            1 => Errno::Perm, 2 => Errno::NoEnt, 3 => Errno::Srch, 4 => Errno::Intr,
            5 => Errno::Io, 6 => Errno::Nxio, 11 => Errno::Again, 13 => Errno::Access,
            14 => Errno::Fault, 16 => Errno::Busy, 17 => Errno::Exist, 18 => Errno::Xdev,
            19 => Errno::Nodev, 20 => Errno::Notdir, 21 => Errno::Isdir, 22 => Errno::Inval,
            23 => Errno::Nfile, 24 => Errno::Mfile, 27 => Errno::Fbigi, 28 => Errno::Nospc,
            32 => Errno::Pipe, 34 => Errno::Range, 36 => Errno::Nametoolong,
            39 => Errno::Notempty, 40 => Errno::Loop, 75 => Errno::Overflow,
            _ => Errno::Unknown,
        }
    }
    pub fn raw(self) -> i32 {
        self as i32
    }
    pub fn description(self) -> &'static str {
        match self {
            Errno::Perm => "Operation not permitted",
            Errno::NoEnt => "No such file or directory",
            Errno::Srch => "No such process",
            Errno::Intr => "Interrupted system call",
            Errno::Io => "I/O error",
            Errno::Nxio => "No such device or address",
            Errno::Again => "Resource temporarily unavailable",
            Errno::Access => "Permission denied",
            Errno::Fault => "Bad address",
            Errno::Busy => "Device or resource busy",
            Errno::Exist => "File exists",
            Errno::Xdev => "Cross-device link",
            Errno::Nodev => "No such device",
            Errno::Notdir => "Not a directory",
            Errno::Isdir => "Is a directory",
            Errno::Inval => "Invalid argument",
            Errno::Fbigi => "File too large",
            Errno::Nospc => "No space left on device",
            Errno::Pipe => "Broken pipe",
            Errno::Range => "Numerical result out of range",
            Errno::Nametoolong => "File name too long",
            Errno::Loop => "Too many levels of symbolic links",
            Errno::Notempty => "Directory not empty",
            Errno::Mfile => "Too many open files",
            Errno::Nfile => "Too many open files in system",
            Errno::Overflow => "Value too large for defined data type",
            Errno::Unknown => "Unknown error",
        }
    }
}

impl fmt::Display for Errno {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.description(), self.raw())
    }
}
impl std::error::Error for Errno {}
