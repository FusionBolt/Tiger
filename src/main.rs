#![feature(box_patterns)]

use std::{env, process};
use crate::driver::compiler::{CompilerOptions, Compiler};
use crate::driver::get_args;

mod parser;
mod ir;
mod driver;
mod symbol_table;

fn main() {
    // todo:driver and args file position is bad
    let args = get_args();
    let options = CompilerOptions::new(args).unwrap_or_else(|err| {
        panic!("{}", err);
    });

    let compiler = Compiler::new();
    if let Err(err) = compiler.compile(&options) {
        panic!("{}", err);
    }
}
