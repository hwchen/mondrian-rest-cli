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
    let out = match config.cmd {
        Command::Describe {cube_name} => {
            let mut res = api::call(config.base_url.unwrap());
            if let Some(cube) = cube_name {
                res.cube(cube);
            }
            println!("{}", res.url().unwrap());
            res.exec()?
        },
        Command::Test {..} => {
            "".to_owned()
        },
        Command::Flush {..} => {
            "".to_owned()
        },
        Command::Query {..} => {
            "".to_owned()
        },
    };

    println!("{}", out);
    Ok(())
}
