use clap::{App, Arg, ArgMatches};

pub mod compiler;

// todo: when must have args?
pub fn get_args<'a>() -> ArgMatches<'a> {
    get_app().get_matches()
}

fn get_app<'a, 'b>() -> App<'a, 'b> {
    App::new("Tiger Compiler")
        .version("0.0.1")
        .author("FusionBolt")
        .about("Modern Compiler Implementation in Rust")
        .arg(Arg::with_name("src")
            .short("s")
            .long("source")
            .takes_value(true)
            .help("set source code file"))
}

#[cfg(test)]
mod tests {
    use crate::driver::get_app;

    #[test]
    fn test_args_src_file() {
        let matches = get_app().get_matches_from(vec!["tiger", "-s=src_path"]);
        match matches.value_of("src") {
            Some(src) => { assert_eq!(src, "src_path")}
            None => { assert!(false) }
        }
    }
}