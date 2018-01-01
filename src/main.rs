// Worklist:
// x implement display and FromStr for qualified names
// - implement full url builder for querybuilder (including output formats)
// - implement serde json to allow for parsing of cube descriptions for testing
// - implement testing (flush, describe, query all dims of a cube)
// - implement better error reporting (runtime Java error, and NaN for json, etc.)
//
// Future:
// - use parser for Cut

#[macro_use]
extern crate failure;
extern crate reqwest;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

mod api;
mod config;

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
    // TODO
    // Generate mondrianbuilder here
    // Then choose either exec or url in the last branch.
    // Or should I only generate url from api, then use reqwest.
    // How much deserializing should I do?
    let out = match config.cmd {
        Command::Describe {cube_name} => {
            let mut res = api::query(config.base_url.unwrap());
            if let Some(cube) = cube_name {
                res.cube(cube);
            }
            println!("{}", res.url().unwrap());
            res.exec()?
        },
        Command::Test {..} => {
            "".to_owned()
        },
        Command::Flush {secret} => {
            api::flush(config.base_url.unwrap(), secret.unwrap())?;
            "Flush complete".to_owned()
        },
        Command::Query {
            cube_name,
            drilldowns,
            measures,
            cuts,
            properties,
            debug,
            parents,
            nonempty,
            distinct,
            format,
            } =>
        {
            let drilldowns = drilldowns.iter()
                .map(|s| s.parse())
                .collect::<Result<Vec<_>, Error>>()?;
            let measures = measures.iter()
                .map(|s| s.parse())
                .collect::<Result<Vec<_>, Error>>()?;
            let cuts = cuts.iter()
                .map(|s| s.parse())
                .collect::<Result<Vec<_>, Error>>()?;
            let properties = properties.iter()
                .map(|s| s.parse())
                .collect::<Result<Vec<_>, Error>>()?;

            let mut res = api::query(config.base_url.unwrap());
            res.cube(cube_name)
                .drilldowns(drilldowns)
                .measures(measures)
                .cuts(cuts)
                .properties(properties)
                .debug(debug)
                .parents(parents)
                .nonempty(nonempty)
                .distinct(distinct)
                .format(format);

            println!("{}", res.url().unwrap());
            res.exec()?
        },
    };

    println!("{}", out);
    Ok(())
}
