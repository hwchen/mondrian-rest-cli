/// Interface to mondrian rest api

use failure::Error;
use reqwest;
use std::io::Read;

// construct url
fn construct_url(
    base_url: String,
    cube_name: String,
    dimensions: Option<Vec<String>>,
    measures: Option<Vec<String>>,
    cuts: Option<Vec<String>>,
    properties: Option<Vec<String>>,
    debug: bool,
    parents: bool,
    nonempty: bool,
    distinct: bool,
    ) -> String
{
    // query is constructed only if Some(dimensions).
    "".to_owned()
}

pub fn describe(base_url: String, cube_name: Option<String>) -> Result<String, Error> {
    let mut url = base_url;
    url.push_str("/cubes");
    if let Some(cube) = cube_name {
        url.push_str(&cube);
    }
    println!("{}", url);
    let mut resp = reqwest::get(&url)?;
    ensure!(resp.status().is_success(), "error");

    Ok(resp.text()?)
}

