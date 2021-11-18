use clap::{App, Arg, ArgMatches};

pub mod compiler;

// todo: when must have args?
pub fn get_args() -> ArgMatches {
    App::new("Tiger Compiler")
        .version("0.0.1")
        .author("FusionBolt")
        .about("Modern Compiler Implementation in Rust")
        .arg(Arg::with_name("SourceFile")
            .short("s")
            .long("source")
            .takes_value(true)
            .help("set source code file"))
        .get_matches()
}