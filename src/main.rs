// Worklist:
// x implement display and FromStr for qualified names
// x implement full url builder for querybuilder (including output formats)
// x implement serde json to allow for parsing of cube descriptions for testing
// x implement testing (flush, describe, query all dims of a cube)
// x implement better error reporting (runtime Java error, and NaN for json, etc.)
//
// Future:
// - use parser for Cut
// - implement state machine for builder, to better control pattern. Now that
//     members, flush, query, etc. are all possibilities.

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

use failure::Error;
use reqwest::{Client, Url};
use std::time::Duration;

use config::Command;
use api::names::{Drilldown, Measure, Property, LevelName};
use schema::{CubeDescription, CubeDescriptions};

fn main() {
    if let Err(err) = run() {
        for cause in err.causes() {
            println!("{}", cause);
        }
    }
}

fn run() -> Result<(), Error> {
    let config = config::get_config()?;

    let client = Client::builder()
        .timeout(Duration::from_secs(config.timeout as u64))
        .build()?;

    // TODO
    // Generate mondrianbuilder here
    // Then choose either exec or url in the last branch.
    // Or should I only generate url from api, then use reqwest.
    // How much deserializing should I do?
    let out = match config.cmd {
        Command::Describe {
            cube_name,
            members,
            raw,
            } =>
        {
            let mut req = api::query(config.base_url.unwrap());
            if let Some(ref cube) = cube_name {
                req.cube(cube.clone());

                if let Some(ref members) = members {
                    let lvl_name = members.parse::<LevelName>()?;
                    req.members(lvl_name);
                }
            }

            let url = req.url()?;
            if config.verbose {
                println!("{}", url);
            }

            let resp = exec_query(&client, url)?;

            if raw {
                resp
            } else {
                if let Some(cube) = cube_name {
                    let cube_or_members: String;

                    if let Some(members) = members {
                        let members: schema::Members = serde_json::from_str(&resp)?;
                        cube_or_members = members.to_string();
                    } else {
                        let cube: schema::CubeDescription = serde_json::from_str(&resp)?;
                        cube_or_members = cube.to_string();
                    }

                    cube_or_members
                } else  {
                    let cubes: CubeDescriptions = serde_json::from_str(&resp)?;
                    cubes.cubes.iter().map(|cube| cube.to_string()).collect::<Vec<_>>().join("\n")
                }
            }
        },
        Command::Test {cube_name} => {
            let mut req = api::query(config.base_url.clone().unwrap());

            if let Some(cube) = cube_name {
                req.cube(cube.clone());

                let url = req.url()?;
                let cube_description: schema::CubeDescription = serde_json::from_str(&exec_query(&client, url)?)?;
                test_cube(&client, &cube_description, &config.base_url.unwrap(), &cube, config.verbose)?
            } else {
                // for all cubes
                let url = req.url()?;
                let cube_descriptions: schema::CubeDescriptions = serde_json::from_str(&exec_query(&client, url)?)?;
                for cube_description in cube_descriptions.cubes {
                    test_cube(&client, &cube_description, config.base_url.as_ref().unwrap(), &cube_description.name, config.verbose)?
                }
            }
            "\nTest Complete".to_owned()
        },
        Command::Flush {secret} => {
            if config.verbose {
                println!("secret: {}", secret.as_ref().unwrap());
            }
            flush(&client, config.base_url.unwrap(), secret.unwrap())?;
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

            let url = req.url()?;
            if config.verbose {
                println!("{}", url);
            }
            exec_query(&client, url)?
        },
    };

    println!("{}", out);
    Ok(())
}

fn test_cube(client: &Client,
    cube_description: &CubeDescription,
    base_url: &str,
    cube_name: &str,
    verbose: bool,
    ) -> Result<(), Error>
{

    // Test strategy, to prevent all combinations being tested:
    // 2 runs
    // - all dims with one measure
    // - one dim with all measures

    let test_drill_mea_prop = cube_description.test_drill_mea_prop();
    if verbose {
        println!("{}", test_drill_mea_prop);
    }

    let drilldowns = test_drill_mea_prop.dims.iter()
        .map(|s| s.parse())
        .collect::<Result<Vec<Drilldown>, Error>>()?;
    let measures = test_drill_mea_prop.meas.iter()
        .map(|s| s.parse())
        .collect::<Result<Vec<Measure>, Error>>()?;
    let properties = test_drill_mea_prop.props.iter()
        .map(|s| s.parse())
        .collect::<Result<Vec<Property>, Error>>()?;

    for drilldown in drilldowns {
        let mut req = api::query(base_url.to_owned());
        req.cube(cube_name)
            .drilldown(drilldown)
            .measures(measures.clone());


        if verbose {
            println!("Test url:\n{}\n", req.url()?);
        }

        let url = req.url()?;
        client.get(url).send().map(|_|())?;
    }

    for property in properties {
        let mut req = api::query(base_url.to_owned());
        req.cube(cube_name)
            .drilldown(property.drill_level())
            .measures(measures.clone())
            .property(property);

        if verbose {
            println!("Test url:\n{}\n", req.url()?);
        }

        let url = req.url()?;
        client.get(url).send().map(|_|())?;
    }

    println!("{}: passed", cube_name);
    Ok(())
}

/// Execute the call and return
/// the body as unparsed string
pub fn exec_query(client: &Client, url: Url) -> Result<String, Error> {
    let mut resp = client.get(url).send()?;

    // TODO return a good error
    ensure!(resp.status().is_success(), format!("[{}]:\n{}", resp.status(), api::format_backtrace(resp.text()?)));

    Ok(resp.text()?)
}

pub fn flush<S: Into<String>>(client: &Client, base_url: S, secret: S) -> Result<(), Error> {
    let mut base_url = base_url.into().clone();
    api::add_trailing_slash(&mut base_url);

    let mut url = Url::parse(&base_url)?;
    url = url.join("flush")?;

    url.query_pairs_mut().append_pair("secret", &secret.into());
    //println!("{}", url.as_str());

    let mut resp = client.get(url).send()?;

    // TODO return a good error
    ensure!(resp.status().is_success(), format!("[{}]:\n{}", resp.status(), api::format_backtrace(resp.text()?)));

    Ok(())
}

