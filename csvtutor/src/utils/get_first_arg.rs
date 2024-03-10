use std::{
    error::Error,
    ffi::OsString,
    env,
};

// Get the first argument from the command line 
// (which is index at 1; the argument at index 0 is the executable name)
/// example: "./target/debug/csvtutor" -> 0 uspop.csv -> 1 (first arg)
// so if one doesn't exist, then "get_first_arg()" returns an error.
pub fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}