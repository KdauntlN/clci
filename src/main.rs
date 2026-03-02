//! CLCI — Command Line Config Interchanger
//! Copyright (c) 2026 Henry Knight
//! Licensed under CC BY-NC 4.0: https://creativecommons.org/licenses/by-nc/4.0/

use std::{
    fs::File,
    io::Read,
    process,
};

use clap::Parser;

fn main() {
    let args = Args::parse();

    let mut buf = String::new();

    let mut file = File::open(args.source).unwrap_or_else(|e| {
        eprintln!("Error opening provided file: {e}");
        process::exit(1);
    });

    file.read_to_string(&mut buf).unwrap_or_else(|e| {
        eprintln!("Error reading provided file: {e}");
        process::exit(1);
    });

    println!("File: {}", buf);
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    source: String,
}