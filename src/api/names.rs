/// Construct, parse, and display full qualified names
/// for Mondrian schema:
/// - drilldown
/// - measure
/// - cut
/// - property

// Structs for creating fully qualified names
// for query parameters
//
// Implement display for all of them so that they
// can be formatted to a string for joining to
// a url.
//
// Implement FromStr to be able to easily parse
// a small variety of names.
// - [Dimension].[Hierarchy].[Level]
// - Dimension.Hierarchy.Level
// - Dimension.Level 
// etc.

use failure::Error;

/// Fully qualified name of Dimension, Hierarchy, and Level
/// Basis for other names.
#[derive(Debug, Clone, PartialEq)]
pub struct LevelName {
    dimension: String,
    hierarchy: String,
    level: String,
}

impl LevelName {
    pub fn new<S: Into<String>>(dimension: S, hierarchy: S, level: S) -> Self {
        LevelName {
            dimension: dimension.into(),
            hierarchy: hierarchy.into(),
            level: level.into(),
        }
    }

    /// Names must have already been trimmed of [] delimiters.
    pub fn from_vec<S: Into<String> + Clone>(level_name: Vec<S>) -> Result<Self, Error> 
    {
        if level_name.len() == 3 {
            Ok(LevelName {
                dimension: level_name[0].clone().into(),
                hierarchy: level_name[1].clone().into(),
                level: level_name[2].clone().into(),
            })
        } else if level_name.len() == 2 {
            Ok(LevelName {
                dimension: level_name[0].clone().into(),
                hierarchy: level_name[0].clone().into(),
                level: level_name[1].clone().into(),
            })
        } else {
            bail!(
                "Dimension {:?} does not follow naming convention",
                level_name.into_iter().map(|s| s.into()).collect::<Vec<String>>()
            );
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Drilldown(LevelName);

impl Drilldown {
    pub fn new<S: Into<String>>(dimension: S, hierarchy: S, level: S) -> Self {
        Drilldown(
            LevelName::new(dimension, hierarchy, level)
        )
    }

    /// Names must have already been trimmed of [] delimiters.
    pub fn from_vec<S: Into<String> + Clone>(drilldown: Vec<S>) -> Result<Self, Error> 
    {
        LevelName::from_vec(drilldown).map(|x| Drilldown(x))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Measure(String);

impl Measure {
    pub fn new<S: Into<String>>(measure: S) -> Self {
        Measure(measure.into())
    }
}

// TODO change cut and property to LevelName
#[derive(Debug, Clone, PartialEq)]
pub struct Cut {
    level: LevelName,
    members: Vec<String>,
}

impl Cut {
    pub fn new<S: Into<String>>(
        dimension: S,
        hierarchy: S,
        level: S,
        members: Vec<S>,
        ) -> Self
    {
        Cut {
            level: LevelName::new(dimension, hierarchy, level),
            members: members.into_iter().map(|s| s.into()).collect(),
        }
    }

    /// Names must have already been trimmed of [] delimiters.
    pub fn from_vec<S: Into<String> + Clone>(cut_level: Vec<S>, members: Vec<S>) -> Result<Self, Error> 
    {
        // TODO get rid of clones
        Ok(LevelName::from_vec(cut_level.clone())
            .map(|level_name| {
                Cut {
                    level: level_name,
                    members: members.clone().into_iter().map(|s| s.into()).collect(),
                }
            })
            .map_err(|err| {
                err.context(format_err!(
                    "Dimension {:?}, {:?} does not follow naming convention",
                    cut_level.into_iter().map(|s| s.into()).collect::<Vec<String>>(),
                    members.into_iter().map(|s| s.into()).collect::<Vec<String>>(),
                ))
            })?)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Property {
    level_name: LevelName,
    property: String,
}

impl Property {
    pub fn new<S: Into<String>>(
        dimension: S,
        hierarchy: S,
        level: S,
        property: S,
        ) -> Self
    {
        Property {
            level_name: LevelName::new(dimension, hierarchy, level),
            property: property.into(),
        }
    }

    /// Names must have already been trimmed of [] delimiters.
    pub fn from_vec<S: Into<String> + Clone>(property: Vec<S>) -> Result<Self, Error> 
    {
        Ok(LevelName::from_vec(property[0..property.len()-1].to_vec())
            .map(|level_name| {
                Property {
                    level_name: level_name,
                    property: property[property.len()-1].clone().into(),
                }
            })
            .map_err(|err| {
                err.context(format_err!(
                    "Dimension {:?} does not follow naming convention",
                    property.into_iter().map(|s| s.into()).collect::<Vec<String>>()
                ))
            })?)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_level_name() {
        let drilldown = LevelName::new("Geography", "Geography", "County");
        let drill_from_vec_1 = LevelName::from_vec(vec!["Geography", "Geography", "County"]).unwrap();
        let drill_from_vec_2 = LevelName::from_vec(vec!["Geography", "County"]).unwrap();

        assert_eq!(drilldown, drill_from_vec_1);
        assert_eq!(drilldown, drill_from_vec_2);
    }

    #[test]
    #[should_panic]
    fn test_level_name_bad_1() {
        LevelName::from_vec(vec!["Geography", "Geography", "County", "County"]).unwrap();
    }
    #[test]
    #[should_panic]
    fn test_level_name_bad_2() {
        LevelName::from_vec(vec!["County"]).unwrap();
    }

    #[test]
    fn test_drilldown() {
        let drilldown = Drilldown::new("Geography", "Geography", "County");
        let drilldown_from_vec = Drilldown::from_vec(
            vec!["Geography", "County"],
            ).unwrap();

        assert_eq!(drilldown, drilldown_from_vec);
    }

    #[test]
    fn test_cut() {
        let cut = Cut::new("Geography", "Geography", "County", vec!["1", "2"]);
        let cut_from_vec = Cut::from_vec(
            vec!["Geography", "County"],
            vec!["1", "2"]
            ).unwrap();

        assert_eq!(cut, cut_from_vec);
    }

    #[test]
    fn test_property() {
        let property = Property::new("Geography", "Geography", "County", "name_en");
        let property_from_vec = Property::from_vec(
            vec!["Geography", "County", "name_en"],
            ).unwrap();

        assert_eq!(property, property_from_vec);
    }
}
