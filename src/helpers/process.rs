// NOTE: Several functions here were borrowed from Nix crate
// See https://docs.rs/nix/0.18.0/src/nix/unistd.rs.html

/// Get the pid of this processes' parent (see
/// [getpid(2)](http://pubs.opengroup.org/onlinepubs/9699919799/functions/getppid.html)).
///
/// There is always a parent pid to return, so there is no error case that needs
/// to be handled according to man page: "These functions are always successful."
pub fn getppid() -> libc::pid_t {
    unsafe { libc::getppid() }
}
