/// Interface to mondrian rest api

use failure::Error;
use reqwest;
use std::io::Read;

#[derive(Debug, Clone, PartialEq)]
pub struct MondrianBuilder {
    base_url: String,
    cube_name: Option<String>,
    drilldowns: Vec<Drilldown>,
    measures: Vec<Measure>,
    cuts: Vec<Cut>,
    properties: Vec<Property>,
    debug: bool,
    parents: bool,
    nonempty: bool,
    distinct: bool,
}

impl Default for MondrianBuilder {
    fn default () -> Self {
        MondrianBuilder {
            base_url: "".to_owned(),
            cube_name: None,
            drilldowns: Vec::new(),
            measures: Vec::new(),
            cuts: Vec::new(),
            properties: Vec::new(),
            debug: false,
            parents: false,
            nonempty: false,
            distinct: false,
        }
    }
}

/// Builder pattern
impl MondrianBuilder {
    pub fn cube<S: Into<String>>(&mut self, cube_name: S) {
        self.cube_name = Some(cube_name.into());
    }

    pub fn drilldown(&mut self, drilldown: Drilldown) {
        self.drilldowns.push(drilldown);
    }

    pub fn drilldowns(&mut self, drilldowns: Vec<Drilldown>) {
        self.drilldowns.extend_from_slice(&drilldowns);
    }

    pub fn measure(&mut self, measure: Measure) {
        self.measures.push(measure);
    }

    pub fn measures(&mut self, measures: Vec<Measure>) {
        self.measures.extend_from_slice(&measures);
    }

    pub fn cut(&mut self, cut: Cut) {
        self.cuts.push(cut);
    }

    pub fn cuts(&mut self, cuts: Vec<Cut>) {
        self.cuts.extend_from_slice(&cuts);
    }

    pub fn property(&mut self, property: Property) {
        self.properties.push(property);
    }

    pub fn properties(&mut self, properties: Vec<Property>) {
        self.properties.extend_from_slice(&properties);
    }

    pub fn debug(&mut self, debug: bool) {
        self.debug = debug;
    }

    pub fn parents(&mut self, parents: bool) {
        self.parents = parents;
    }

    pub fn nonempty(&mut self, nonempty: bool) {
        self.nonempty = nonempty;
    }

    pub fn distinct(&mut self, distinct: bool) {
        self.distinct = distinct;
    }

    /// One finalizer for builder pattern
    /// Execute the call and return
    /// the body as unparsed string
    pub fn exec(&self) -> Result<String, Error> {
        Ok("".to_owned())
    }

    /// The other finalizer
    /// return the url
    pub fn url(&self) -> String {
        "".to_owned()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Drilldown {
    dimension: String,
    hierarchy: String,
    level: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Measure(String);

#[derive(Debug, Clone, PartialEq)]
pub struct Cut {
    dimension: String,
    hierarchy: String,
    level: String,
    members: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Property {
    dimension: String,
    hierarchy: String,
    level: String,
    property: String,
}

/// Initializer for the builder pattern
pub fn call(base_url: String) -> MondrianBuilder {
    let mut builder = MondrianBuilder::default();
    builder.base_url = base_url;
    builder
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

