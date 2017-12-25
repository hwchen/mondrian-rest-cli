#[macro_use]
extern crate failure;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

mod config;

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
    Ok(())
}
