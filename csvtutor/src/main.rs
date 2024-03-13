use std::{
    process,
    env::args_os,
    io::stdin
};
mod lib;
mod utils;
use crate::lib::*;

static _CSV_READERS: [(&str, fn() -> Result<(), Box<dyn std::error::Error>>); 4] = [
    ("read_from_stdin", read_from_stdin),
    ("read_from_path (Default reader)", read_from_path),
    ("read_with_transcode", read_with_transcode),
    ("read_custom_config", read_custom_config)
];

fn run(function: fn() -> Result<(), Box<dyn std::error::Error>> ) {
    if let Err(err) = function() {
        println!("{}", err);
        process::exit(1);
    }
}

fn main() {
    if args_os().count() == 2 {
        println!("Digite o número de uma das opções:");
        let mut valid_inputs: Vec<u8> = Vec::new();
        for (i, (name, _)) in _CSV_READERS.iter().enumerate() {
            if i == 0 {
                continue;
            }
            valid_inputs.push(i as u8);
            println!("{}: {}", i, name);
        }
        println!();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let input = input.trim().parse::<u8>().unwrap();
        if valid_inputs.contains(&input) {
            let function = _CSV_READERS[input as usize].1;
            run(function);
        }
    } else {
        let function = _CSV_READERS[0].1;
        run(function);
    }
}