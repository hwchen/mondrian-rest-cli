/// Interface to mondrian rest api

mod names;

use failure::Error;
use reqwest::{self, Url};

pub use self::names::{Drilldown, Measure, Cut, Property};

#[derive(Debug, Clone, PartialEq)]
pub struct QueryBuilder {
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
    format: ResponseFormat,
}

impl Default for QueryBuilder {
    fn default () -> Self {
        QueryBuilder {
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
            format: ResponseFormat::Json,
        }
    }
}

/// Builder pattern
impl QueryBuilder {
    pub fn cube<S: Into<String>>(&mut self, cube_name: S) -> &mut Self {
        self.cube_name = Some(cube_name.into());
        self
    }

    pub fn drilldown(&mut self, drilldown: Drilldown) -> &mut Self {
        self.drilldowns.push(drilldown);
        self
    }

    pub fn drilldowns(&mut self, drilldowns: Vec<Drilldown>) -> &mut Self {
        self.drilldowns.extend_from_slice(&drilldowns);
        self
    }

    pub fn measure(&mut self, measure: Measure) -> &mut Self {
        self.measures.push(measure);
        self
    }

    pub fn measures(&mut self, measures: Vec<Measure>) -> &mut Self {
        self.measures.extend_from_slice(&measures);
        self
    }

    pub fn cut(&mut self, cut: Cut) -> &mut Self {
        self.cuts.push(cut);
        self
    }

    pub fn cuts(&mut self, cuts: Vec<Cut>) -> &mut Self {
        self.cuts.extend_from_slice(&cuts);
        self
    }

    pub fn property(&mut self, property: Property) -> &mut Self {
        self.properties.push(property);
        self
    }

    pub fn properties(&mut self, properties: Vec<Property>) -> &mut Self {
        self.properties.extend_from_slice(&properties);
        self
    }

    pub fn debug(&mut self, debug: bool) -> &mut Self {
        self.debug = debug;
        self
    }

    pub fn parents(&mut self, parents: bool) -> &mut Self {
        self.parents = parents;
        self
    }

    pub fn nonempty(&mut self, nonempty: bool) -> &mut Self {
        self.nonempty = nonempty;
        self
    }

    pub fn distinct(&mut self, distinct: bool) -> &mut Self {
        self.distinct = distinct;
        self
    }

    /// One finalizer for builder pattern
    /// Execute the call and return
    /// the body as unparsed string
    pub fn exec(&self) -> Result<String, Error> {
        let url = self.url()?;
        let mut resp = reqwest::get(url)?;

        // TODO return a good error
        ensure!(resp.status().is_success(), "error");

        Ok(resp.text()?)
    }

    /// The other finalizer
    /// return the url
    pub fn url(&self) -> Result<Url, Error> {
        let mut base_url = self.base_url.clone();
        add_trailing_slash(&mut base_url);

        let mut url = Url::parse(&self.base_url)?;
        url = url.join("cubes/")?;

        if let Some(ref cube_name) = self.cube_name {
            let mut cube_name = cube_name.clone();
            add_trailing_slash(&mut cube_name);
            url = url.join(&cube_name)?;
        }

        Ok(url)
    }

}

#[derive(Debug, Clone, PartialEq)]
pub enum ResponseFormat {
    Json,
    JsonRecords,
    Csv,
}

/// Initializer for the builder pattern
pub fn query(base_url: String) -> QueryBuilder {
    let mut builder = QueryBuilder::default();
    builder.base_url = base_url;
    builder
}

/// Other other finalizer for builder pattern
pub fn flush<S: Into<String>>(base_url: S, secret: S) -> Result<(), Error> {
    let mut base_url = base_url.into().clone();
    add_trailing_slash(&mut base_url);

    let mut url = Url::parse(&base_url)?;
    url = url.join("flush/")?;

    url.query_pairs_mut().append_pair("secret", &secret.into());
    println!("{}", url.as_str());

    let mut resp = reqwest::get(url)?;

    // TODO return a good error
    ensure!(resp.status().is_success(), "error");

    Ok(())
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

// util fn
fn add_trailing_slash(s: &mut String) {
    if let Some(last_char) = s.chars().last() {
        if last_char != '/' {
            s.push('/');
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add_trailing_slash() {
        let mut test = "test".to_owned();
        let mut test1 = "test1/".to_owned();

        add_trailing_slash(&mut test);
        add_trailing_slash(&mut test1);

        assert_eq!(test, "test/".to_owned());
        assert_eq!(test1, "test1/".to_owned());
    }
}
