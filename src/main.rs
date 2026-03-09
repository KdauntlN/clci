//! CLCI — Command Line Config Interchanger
//! Copyright (c) 2026 Henry Knight
//! Licensed under CC BY-NC 4.0: https://creativecommons.org/licenses/by-nc/4.0/

use clci::{
    cli::Args,
    parsing::{Interchange},
};

use std::process;

use clap::Parser;

fn main() {
    let args = Args::parse();

    let source = clci::open_file(args.source).unwrap_or_else(|e| {
        eprintln!("Program encountered an error: {e}");
        process::exit(1);
    });

    let _ir = source.parse().unwrap_or_else(|e| {
        eprintln!("Program encountered an error: {e}");
        process::exit(1);
    });
}