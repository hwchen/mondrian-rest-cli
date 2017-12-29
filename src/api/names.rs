/// Construct, parse, and display full qualified names
/// for Mondrian schema:
/// - drilldown
/// - measure
/// - cut
/// - property

#[derive(Debug, Clone, PartialEq)]
pub struct Drilldown {
    dimension: String,
    hierarchy: String,
    level: String,
}

impl Drilldown {
    pub fn new<S: Into<String>>(dimension: S, hierarchy: S, level: S) -> Self {
        Drilldown {
            dimension: dimension.into(),
            hierarchy: hierarchy.into(),
            level: level.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Measure(String);

impl Measure {
    pub fn new<S: Into<String>>(measure: S) -> Self {
        Measure(measure.into())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cut {
    dimension: String,
    hierarchy: String,
    level: String,
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
            dimension: dimension.into(),
            hierarchy: hierarchy.into(),
            level: level.into(),
            members: members.into_iter().map(|s| s.into()).collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Property {
    dimension: String,
    hierarchy: String,
    level: String,
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
            dimension: dimension.into(),
            hierarchy: hierarchy.into(),
            level: level.into(),
            property: property.into(),
        }
    }
}

