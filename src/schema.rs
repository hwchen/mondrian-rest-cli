/// structs for deserializing description of cube schema

use serde::{de, Deserialize, Deserializer};
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

use api::names::LevelName;

#[derive(Debug, Deserialize)]
pub struct CubeDescriptions {
    pub cubes: Vec<CubeDescription>,
}

#[derive(Debug, Deserialize)]
pub struct CubeDescription {
    pub name: String,
    dimensions: Vec<Dimension>,
    measures: Vec<Measure>,
    named_sets: Vec<NamedSet>,
    annotations: HashMap<String,String>,
}

impl fmt::Display for CubeDescription {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut out = String::new();
        out.push_str("Cube: ");
        out.push_str(&self.name);
        out.push_str("\n");

        for (k, v) in &self.annotations {
            out.push_str(format!("  ({}: {})\n", k, v).as_str());
        }

        out.push_str("  Dimensions, Hierarchies, and Levels:\n");
        for dim in &self.dimensions {
            for (k, v) in &dim.annotations {
                out.push_str(format!("    ({}: {})\n", k, v).as_str());
            }
            for hier in &dim.hierarchies {
                // no hierarchy annotations for now
                for lvl in &hier.levels[1..] {
                    out.push_str("    ");
                    out.push_str(&lvl.full_name);
                    out.push_str("\n");

                    for prop in &lvl.properties {
                        out.push_str("      ");
                        out.push_str(prop);
                        out.push_str(" (property)\n");
                    }

                    for (k, v) in &lvl.annotations {
                        out.push_str(format!("      ({}: {})\n", k, v).as_str());
                    }
                }
            }
        }

        if !self.named_sets.is_empty() {
            out.push_str("  Named Sets:\n");
        }
        for named_set in &self.named_sets {
            for (k, v) in &named_set.annotations {
                out.push_str(format!("    ({}: {})\n", k, v).as_str());
            }

            let level_name = LevelName::new(
                named_set.dimension.to_owned(),
                named_set.hierarchy.to_owned(),
                named_set.level.to_owned());
            out.push_str("    ");
            out.push_str(&named_set.name);
            out.push_str(": ");
            out.push_str(level_name.to_string().as_str());
            out.push_str("\n");
        }

        out.push_str("  Measures:\n");
        for mea in &self.measures {
            for (k, v) in &mea.annotations {
                out.push_str(format!("    ({}: {})\n", k, v).as_str());
            }

            out.push_str("    ");
            out.push_str(&mea.name);
            if let Some(ref agg) = mea.aggregator {
                out.push_str(" | agg: ");
                out.push_str(&agg);
            }
            out.push_str("\n");
        }

        write!(f, "{}", out)
    }
}

impl CubeDescription {
    pub fn test_drill_mea_prop(&self) -> Test {
        let mut test_dims = Vec::new();

        for dim in &self.dimensions {
            let lvl = dim.hierarchies.first().and_then(|hier| hier.levels.first());
            if let Some(lvl) = lvl {
                test_dims.push(lvl.full_name.clone());
            }
        }

        let mut test_props = Vec::new();

        for dim in &self.dimensions {
            for hier in &dim.hierarchies {
                for level in &hier.levels {

                    if !level.properties.is_empty() {
                        let level_name = level.full_name.to_owned();
                        let properties = level.properties.iter()
                            .map(|prop| {
                                let prop = format!(".[{}]", prop);
                                let mut prop_full_name = level_name.clone();
                                prop_full_name.push_str(&prop);
                                prop_full_name
                            });

                        test_props.extend(properties);
                    }

                }
            }
        }

        Test {
            name: self.name.clone(),
            dims: test_dims,
            meas: self.measures.iter().map(|mea| mea.name.clone()).collect(),
            props: test_props,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Dimension {
    name: String,
    caption: String,
    annotations: HashMap<String,String>,
    hierarchies: Vec<Hierarchy>,
}

#[derive(Debug, Deserialize)]
pub struct Hierarchy {
    name: String,
    has_all: bool,
    all_member_name: String,
    levels: Vec<Level>,
}

#[derive(Debug, Deserialize)]
pub struct Level {
    name: String,
    full_name: String,
    depth: u32,
    caption: String,
    annotations: HashMap<String,String>,
    properties: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Measure {
    name: String,
    caption: String,
    annotations: HashMap<String,String>,
    full_name: String,
    aggregator: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct NamedSet {
    name: String,
    dimension: String,
    hierarchy: String,
    level: String,
    annotations: HashMap<String,String>,
}

#[derive(Debug)]
pub struct Test {
    pub name: String,
    pub dims: Vec<String>,
    pub meas: Vec<String>,
    pub props: Vec<String>,
}


impl fmt::Display for Test {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut out = String::new();
        out.push_str("Test Cube: ");
        out.push_str(&self.name.clone());
        out.push_str("\n");

        out.push_str("  Dimensions, Hierarchies, and Levels:\n");
        for dim in &self.dims {
            out.push_str("    ");
            out.push_str(&dim);
            out.push_str("\n");
        }

        out.push_str("  Measures:\n");
        for mea in &self.meas {
            out.push_str("    ");
            out.push_str(&mea);
            out.push_str("\n");
        }

        out.push_str("  Properties:\n");
        for prop in &self.props {
            out.push_str("    ");
            out.push_str(&prop);
            out.push_str("\n");
        }

        // TODO named sets

        write!(f, "{}", out)
    }
}

#[derive(Debug, Deserialize)]
pub struct Members {
    name: String,
    caption: String,
    members: Vec<Member>,
}

impl fmt::Display for Members {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut out = String::new();
        out.push_str(format!("Members of Level {}:\n", self.name).as_str());

        for member in &self.members {
            out.push_str(member.to_string().as_str());
        }

        write!(f, "{}", out)
    }
}


#[derive(Debug, Deserialize)]
pub struct Member {
    name: String,
    full_name: String,
    caption: String,
    key: Key, // always string because it is sometimes str sometimes num
    #[serde(rename = "all_member?")]
    is_all_member: bool,
    #[serde(rename = "drillable?")]
    is_drillable: bool,
    depth: u32,
    num_children: u32,
    parent_name: String,
    level_name: String,
    children: Vec<String>, // should be Vec of children, but don't need now
}

impl fmt::Display for Member {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut out = String::new();
        out.push_str(format!("&{}: {}\n", self.key, self.name).as_str());

        write!(f, "{}", out)
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Key {
    String(String),
    Int(i64),
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Key::String(ref s) => write!(f, "{}", s),
            Key::Int(ref i) => write!(f, "{}", i),
        }
    }
}
