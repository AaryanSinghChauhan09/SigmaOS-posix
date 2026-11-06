# SigmaOS-posix

POSIX.1 compatibility layer for SigmaOS: stable errno values, the Linux
x86_64 syscall number table, POSIX pathname normalization, and stat(2)
file-type/mode decoding. Mechanism only — sandboxing and capability
policy live in higher layers.

## Usage

```rust
use sigmaos_posix::{Errno, Syscall, normalize_path, Stat, FileType};

let e = Errno::NoEnt;
assert_eq!(e.raw(), 2);
assert_eq!(Syscall::Openat.number(), 257);
assert_eq!(normalize_path("/a/b/../c").unwrap(), "/a/c");
```

## Testing

`cargo test` — deterministic unit + integration tests.
