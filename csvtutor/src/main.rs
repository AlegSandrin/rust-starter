use std::process;
mod lib;
mod utils;
use crate::lib::*;

fn main() {
    if let Err(err) = read_custom_config() {
        println!("{}", err);
        process::exit(1);
    }
}