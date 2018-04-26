/// Interface to mondrian rest api

pub mod names;

use failure::Error;
use reqwest::{self, Url};
use serde_json;
use std::fmt;
use std::str::FromStr;

pub use self::names::{LevelName, Drilldown, Measure, Cut, Property};

#[derive(Debug, Clone, PartialEq)]
pub struct QueryBuilder {
    base_url: String,
    cube_name: Option<String>,
    members: Option< LevelName>,
    drilldowns: Vec<Drilldown>,
    measures: Vec<Measure>,
    cuts: Vec<Cut>,
    properties: Vec<Property>,
    debug: bool,
    parents: bool,
    nonempty: bool,
    distinct: bool,
    sparse: bool,
    format: ResponseFormat,
}

impl Default for QueryBuilder {
    fn default () -> Self {
        QueryBuilder {
            base_url: "".to_owned(),
            cube_name: None,
            members: None,
            drilldowns: Vec::new(),
            measures: Vec::new(),
            cuts: Vec::new(),
            properties: Vec::new(),
            debug: false,
            parents: false,
            nonempty: false,
            distinct: false,
            sparse: false,
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

    /// Returns &Self so that no more changes can be made
    /// but exec and url can still be called
    pub fn members(&mut self, level_name: LevelName) -> &Self {
        self.members = Some(level_name);
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

    pub fn sparse(&mut self, sparse: bool) -> &mut Self {
        self.sparse = sparse;
        self
    }

    pub fn format(&mut self, format: ResponseFormat) -> &mut Self {
        self.format = format;
        self
    }

    /// The other finalizer
    /// return the url
    pub fn url(&self) -> Result<Url, Error> {
        // Check that members has cube_name (also checked in cli,
        // but good to check in lib too.)
        //
        // Also check that there aren't other builders called; could just
        // ignore, since members has to be the last builder called,
        // but provides more clarity as to how the api should be used.

        // Then do members first

        if let Some(ref members) = self.members {
            if self.cube_name.is_none() {
                bail!("Members call requires a cube name");
            }

            if !self.drilldowns.is_empty() ||
                !self.measures.is_empty() ||
                !self.cuts.is_empty() ||
                !self.properties.is_empty()
            {
                bail!("Members call should not include query parameters");
            }

            let mut base_url = self.base_url.clone();
            add_trailing_slash(&mut base_url);

            let mut url = Url::parse(&self.base_url)?;
            url = url.join("cubes/")?;

            // cube_name must be Some, from above early return
            let mut cube_name = self.cube_name.as_ref().unwrap().clone();
            add_trailing_slash(&mut cube_name);
            url = url.join(&cube_name)?;

            url = url.join("dimensions/")?;

            let mut dim_name = members.dimension().to_owned();
            add_trailing_slash(&mut dim_name);
            url = url.join(&dim_name)?;

            url = url.join("hierarchies/")?;

            let mut hier_name = members.hierarchy().to_owned();
            add_trailing_slash(&mut hier_name);
            url = url.join(&hier_name)?;

            url = url.join("levels/")?;

            let mut lvl_name = members.level().to_owned();
            add_trailing_slash(&mut lvl_name);
            url = url.join(&lvl_name)?;

            url = url.join("members")?;

            return Ok(url)
        }

        // Special case of members done,
        // Now move onto general query
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
                url.query_pairs_mut()
                    .append_pair("sparse", &self.sparse.to_string());
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

// util fn
pub(crate) fn add_trailing_slash(s: &mut String) {
    if let Some(last_char) = s.chars().last() {
        if last_char != '/' {
            s.push('/');
        }
    }
}

pub(crate) fn format_backtrace(s: String) -> String {
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
