/// structs for deserializing description of cube schema

use std::fmt;

#[derive(Debug, Deserialize)]
pub struct CubeDescriptions {
    pub cubes: Vec<CubeDescription>,
}

#[derive(Debug, Deserialize)]
pub struct CubeDescription {
    pub name: String,
    dimensions: Vec<Dim>,
    measures: Vec<Mea>,
}

impl fmt::Display for CubeDescription {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut out = String::new();
        out.push_str("Cube: ");
        out.push_str(&self.name);
        out.push_str("\n");

        out.push_str("  Dimensions, Hierarchies, and Levels:\n");
        for dim in &self.dimensions {
            for hier in &dim.hierarchies {
                for lvl in &hier.levels[1..] {
                    out.push_str("    ");
                    out.push_str(&lvl.full_name);
                    out.push_str("\n");
                }
            }
        }

        out.push_str("  Measures:\n");
        for mea in &self.measures {
            out.push_str("    ");
            out.push_str(&mea.name);
            out.push_str("\n");
        }

        // TODO add properties, named sets

        write!(f, "{}", out)
    }
}

impl CubeDescription {
    pub fn test_dim_mea(&self) -> Test {
        let mut test_dims = Vec::new();
        for dim in &self.dimensions {
            let lvl = dim.hierarchies.first().and_then(|hier| hier.levels.first());
            if let Some(lvl) = lvl {
                test_dims.push(lvl.full_name.clone());
            }
        }

        Test {
            name: self.name.clone(),
            dims: test_dims,
            meas: self.measures.iter().map(|mea| mea.name.clone()).collect(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Dim {
    hierarchies: Vec<Hier>,
}

#[derive(Debug, Deserialize)]
pub struct Hier {
    levels: Vec<Lvl>,
}

#[derive(Debug, Deserialize)]
pub struct Lvl {
    full_name: String,
}

#[derive(Debug, Deserialize)]
pub struct Mea {
    name: String,
}

#[derive(Debug)]
pub struct Test {
    pub name: String,
    pub dims: Vec<String>,
    pub meas: Vec<String>,
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

        // TODO add properties, named sets

        write!(f, "{}", out)
    }
}
