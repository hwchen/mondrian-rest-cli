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
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

mod api;
mod config;
mod schema;

use config::Command;
use failure::Error;

use api::names::{Drilldown, Measure};
use schema::{CubeDescription};

fn main() {
    if let Err(err) = run() {
        for cause in err.causes() {
            println!("{}", cause);
        }
    }
}

fn run() -> Result<(), Error> {
    let config = config::get_config()?;

    // TODO
    // Generate mondrianbuilder here
    // Then choose either exec or url in the last branch.
    // Or should I only generate url from api, then use reqwest.
    // How much deserializing should I do?
    let out = match config.cmd {
        Command::Describe {cube_name} => {
            let mut req = api::query(config.base_url.unwrap());
            if let Some(cube) = cube_name {
                req.cube(cube);
            }
            if config.verbose {
                println!("{}", req.url().unwrap());
            }
            req.exec()?
        },
        Command::Test {cube_name} => {
            let mut req = api::query(config.base_url.clone().unwrap());

            if let Some(cube) = cube_name {
                req.cube(cube.clone());

                let cube_description: schema::CubeDescription = serde_json::from_str(&req.exec()?)?;
                test_cube(&cube_description, &config.base_url.unwrap(), &cube, config.verbose)?
            } else {
                // Do it again for all cubes
                let cube_descriptions: schema::CubeDescriptions = serde_json::from_str(&req.exec()?)?;
                for cube_description in cube_descriptions.cubes {
                    test_cube(&cube_description, config.base_url.as_ref().unwrap(), &cube_description.name, config.verbose)?
                }
            }
            "\nTest Complete".to_owned()
        },
        Command::Flush {secret} => {
            if config.verbose {
                println!("secret: {}", secret.as_ref().unwrap());
            }
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
            sparse,
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

            let mut req = api::query(config.base_url.unwrap());
            req.cube(cube_name)
                .drilldowns(drilldowns)
                .measures(measures)
                .cuts(cuts)
                .properties(properties)
                .debug(debug)
                .parents(parents)
                .nonempty(nonempty)
                .distinct(distinct)
                .sparse(sparse)
                .format(format);

            if config.verbose {
                println!("{}", req.url().unwrap());
            }
            req.exec()?
        },
    };

    println!("{}", out);
    Ok(())
}

fn test_cube(cube_description: &CubeDescription, base_url: &str, cube_name: &str, verbose: bool) -> Result<(), Error> {
    // Test strategy, to prevent all combinations being tested:
    // 2 runs
    // - all dims with one measure
    // - one dim with all measures

    let test_dim_mea = cube_description.test_dim_mea();
    if verbose {
        println!("{}", test_dim_mea);
    }

    let drilldowns = test_dim_mea.dims.iter()
        .map(|s| s.parse())
        .collect::<Result<Vec<Drilldown>, Error>>()?;
    let measures = test_dim_mea.meas.iter()
        .map(|s| s.parse())
        .collect::<Result<Vec<Measure>, Error>>()?;

    for drilldown in drilldowns {
        let mut req = api::query(base_url.to_owned());
        req.cube(cube_name)
            .drilldown(drilldown)
            .measures(measures.clone());


        if verbose {
            println!("Test url:\n{}\n", req.url().unwrap());
        }

        req.exec().map(|_|())?;
    }

    println!("{}: passed", cube_name);
    Ok(())
}
