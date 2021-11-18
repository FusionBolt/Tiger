#![feature(box_patterns)]

use std::{env, process};
use crate::driver::compiler::{CompilerOptions, Compiler};
use crate::driver::get_args;

mod parser;
mod ir;
mod driver;

fn main() {
    let args = get_args();
    let options = CompilerOptions::new(args).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(-1);
    });

    let compiler = Compiler::new();
    if let Err(err) = compiler.compile(&options) {
        println!("{}", err);
        process::exit(-1);
    }
}
