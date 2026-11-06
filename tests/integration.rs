use sigmaos_posix::*;

#[test]
fn posix_surface_smoke() {
    let e = Errno::Access;
    assert_eq!(e.raw(), 13);
    assert_eq!(Syscall::Execve.name(), "execve");
    assert_eq!(normalize_path("/usr//lib/../bin").unwrap(), "/usr/bin");
}
