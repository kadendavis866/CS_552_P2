use std::{env, io};

/// Returns the current working directory.
///
/// # Returns
/// An `io::Result` containing the current working directory as a `std::path::PathBuf`.

pub(crate) fn pwd() -> io::Result<std::path::PathBuf> {
    env::current_dir()
}

/// Changes the current working directory to the specified directory.
/// If the directory is `"~"`, it changes to the user's home directory.
///
/// # Arguments
/// * `dir` - A string pointer to the target directory.
///
/// # Returns
///
/// An `io::Result` indicating success or failure.
pub(crate) fn cd(dir: &str) -> io::Result<()>{
    if dir == "~" {
        let home =
            env::var("HOME").unwrap_or_else(|_| unsafe {
                // get the home directory from the passwd file if HOME is not set
                let uid = libc::getuid();
                let passwd = libc::getpwuid(uid);
                std::ffi::CStr::from_ptr((*passwd).pw_dir).to_string_lossy().into_owned()
            });
        return env::set_current_dir(home);
    }
    env::set_current_dir(dir)
}

/// Exits the current process with a status code of 0.
pub(crate) fn exit(){
    std::process::exit(0);
}