extern crate ron;
extern crate anyhow;
extern crate serde;
extern crate clap;

use std::fs::File;
use std::io::{self, Read};
use anyhow::{anyhow, Result};
use clap::{Arg, App, crate_version, crate_authors};

fn get_app<'a, 'b>() -> App<'a, 'b> {
    App::new("borscht")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Simple linting of RON files for errors")
        .arg(Arg::with_name("INPUT")
                    .help("Sets the input file to use")
                    .required(true)
                    .index(1))
}

fn run_app() -> Result<()> {
    let matches = get_app().get_matches();

    let input = matches.value_of("INPUT").unwrap_or("-".into());
    let rdr: Box<dyn Read> = if input == "-" {
        Box::new(io::stdin())
    } else {
        Box::new(File::open(input)?)
    };

    let name = if input == "-" {
        "<stdin>".into()
    } else {
        input
    };

    let result: ron::error::Result<ron::Value> = ron::de::from_reader(rdr);
    match result {
        Ok(_) => Ok(()),
        Err(e) => {
            println!("{}:{}", name, e);
            Ok(())
        }
    }
}

fn main() -> ! {
    let result = run_app();
    let exit_code = match result {
        Ok(_) => 0,
        Err(_) => 1
    };

    std::process::exit(exit_code);
}
