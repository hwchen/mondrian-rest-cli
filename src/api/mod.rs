/// Interface to mondrian rest api

mod names;

use failure::Error;
use reqwest::{self, Url};
use serde_json;
use std::fmt;
use std::str::FromStr;

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

    pub fn format(&mut self, format: ResponseFormat) -> &mut Self {
        self.format = format;
        self
    }

    /// One finalizer for builder pattern
    /// Execute the call and return
    /// the body as unparsed string
    pub fn exec(&self) -> Result<String, Error> {
        let url = self.url()?;
        let mut resp = reqwest::get(url)?;

        // TODO return a good error
    ensure!(resp.status().is_success(), format!("[{}]:\n{}", resp.status(), format_backtrace(resp.text()?)));

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

            // At this point, only add aggregate etc if
            // there is drilldown and measure. Otherwise
            // pass through (it's just asking for a description
            // of one cube)
            if !self.drilldowns.is_empty() && !self.measures.is_empty() {
                url = url.join(
                    format!("aggregate.{}", self.format).as_str()
                )?;

                // add all query parameters

                for drilldown in &self.drilldowns {
                    url.query_pairs_mut()
                        .append_pair("drilldown[]", &drilldown.to_string());
                }
                for measure in &self.measures {
                    url.query_pairs_mut()
                        .append_pair("measures[]", &measure.to_string());
                }
                for cut in &self.cuts {
                    url.query_pairs_mut()
                        .append_pair("cut[]", &cut.to_string());
                }
                for property in &self.properties {
                    url.query_pairs_mut()
                        .append_pair("properties[]", &property.to_string());
                }

                url.query_pairs_mut()
                    .append_pair("debug", &self.debug.to_string());
                url.query_pairs_mut()
                    .append_pair("parents", &self.parents.to_string());
                url.query_pairs_mut()
                    .append_pair("nonempty", &self.nonempty.to_string());
                url.query_pairs_mut()
                    .append_pair("distinct", &self.distinct.to_string());
            }
        } else {
            // if there is no cube name and there's a query, return
            // error. Otherwise, just pass (just asking for
            // a description of all cubes)
            if !self.drilldowns.is_empty() ||
                !self.measures.is_empty() ||
                !self.cuts.is_empty() ||
                !self.properties.is_empty()
            {
                bail!("Cube name is required for query");
            }
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

impl FromStr for ResponseFormat {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use self::ResponseFormat::*;
        match s {
            "json" => Ok(Json),
            "jsonrecords" => Ok(JsonRecords),
            "csv" => Ok(Csv),
            _ => Err(format_err!("{:?} is not a valid response format", s))
        }
    }
}

impl fmt::Display for ResponseFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::ResponseFormat::*;
        match *self {
            Json => write!(f, "json"),
            JsonRecords => write!(f, "jsonrecords"),
            Csv => write!(f, "csv"),
        }
    }
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
    url = url.join("flush")?;

    url.query_pairs_mut().append_pair("secret", &secret.into());
    //println!("{}", url.as_str());

    let mut resp = reqwest::get(url)?;

    // TODO return a good error
    ensure!(resp.status().is_success(), format!("[{}]:\n{}", resp.status(), format_backtrace(resp.text()?)));

    Ok(())
}

// util fn
fn add_trailing_slash(s: &mut String) {
    if let Some(last_char) = s.chars().last() {
        if last_char != '/' {
            s.push('/');
        }
    }
}

fn format_backtrace(s: String) -> String {
    let try_de: Result<MonError, _> = serde_json::from_str(&s);

    // Need one more format for runtime errors that are caught.
    // Which appear to be {"error": "error description"}
    if let Ok(err) = try_de {
        // for query runtime errors that are uncaught
        err.error[0].clone()
    } else {
        // for flush runtime errors
        s.lines()
            .take(2)
            .collect::<Vec<_>>()
            .join("\n")
    }

}

#[derive(Debug, Deserialize)]
struct MonError {
    error: Vec<String>,
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
