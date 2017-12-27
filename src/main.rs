#[macro_use]
extern crate failure;
extern crate reqwest;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

mod api;
mod config;

use api::{
    describe,
};
use config::Command;
use failure::Error;


fn main() {
    if let Err(err) = run() {
        for cause in err.causes() {
            println!("{}", cause);
        }
    }
}

fn run() -> Result<(), Error> {
    let config = config::get_config()
        .map_err(|err| {
            err.context("Mondrian Rest Cli")
        })?;
    match config.cmd {
        Command::Describe {cube_name} => {
            println!("{}", api::describe(config.base_url.unwrap(), cube_name)?);
        },
        Command::Test {..} => {
            ()
        },
        Command::Flush {..} => {
            ()
        },
        Command::Query {..} => {
            ()
        },
    }
    Ok(())
}
