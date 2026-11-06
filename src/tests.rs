use super::*;


#[test]
fn errno_roundtrip() {
    for e in [Errno::Perm, Errno::NoEnt, Errno::Inval, Errno::Nospc] {
        assert_eq!(Errno::from_raw(e.raw()), e);
    }
    assert_eq!(Errno::from_raw(9999), Errno::Unknown);
}

#[test]
fn errno_display() {
    assert!(Errno::NoEnt.to_string().contains("No such file"));
}

#[test]
fn syscall_numbers_stable() {
    assert_eq!(Syscall::Read.number(), 0);
    assert_eq!(Syscall::Write.number(), 1);
    assert_eq!(Syscall::Openat.number(), 257);
    assert_eq!(Syscall::Read.name(), "read");
}

#[test]
fn path_normalization() {
    assert_eq!(normalize_path("/a/b/../c").unwrap(), "/a/c");
    assert_eq!(normalize_path("a//b/./c").unwrap(), "a/b/c");
    assert_eq!(normalize_path("/../x").unwrap(), "/x");
    assert_eq!(normalize_path("..").unwrap(), "..");
    assert_eq!(normalize_path("").unwrap_err(), Errno::Inval);
}

#[test]
fn stat_file_type_and_mode() {
    let dir = Stat {
        dev: 0, ino: 1, mode: 0o040755, nlink: 2, uid: 0, gid: 0,
        size: 4096, atime: 0, mtime: 0, ctime: 0,
    };
    assert!(dir.is_dir());
    assert_eq!(dir.permissions(), 0o755);
    let reg = Stat { mode: 0o100644, ..dir };
    assert_eq!(reg.file_type(), Some(FileType::Reg));
}
