use std::error::Error;
use std::path::{Path, PathBuf};
use clap::{ArgMatches};
use std::{fs, io};
use crate::parser::parser::parse_source;

pub struct Compiler {

}


impl Compiler {
    pub fn new() -> Compiler {
        Compiler{ }
    }

    pub fn compile(&self, options: &CompilerOptions) -> Result<(), Box<dyn Error>> {
        let src = self.read_source(options)?;
        let module = parse_source(&src)?;
        println!("{:#?}", module);
        println!("compile successful");
        Ok(())
    }

    fn read_source(&self, options: &CompilerOptions) -> io::Result<String> {
        fs::read_to_string(&options.path)
    }
}

pub struct CompilerOptions {
    path: PathBuf,
}

impl CompilerOptions {
    // todo:error process
    pub fn new(args: ArgMatches) -> Result<CompilerOptions, &'static str> {
        let path = match args.value_of("src") {
            Some(src) => {
                PathBuf::from(src)
            }
            None => {
                return Err("no source file specified");
            }
        };
        Ok(CompilerOptions{ path })
    }
}