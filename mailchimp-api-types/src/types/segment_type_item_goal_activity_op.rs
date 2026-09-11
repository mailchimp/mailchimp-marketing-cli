pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Whether the website URL is/not exactly, contains/doesn't contain, starts with/ends with a string.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SegmentTypeItemGoalActivityOp {
    Is,
    GoalNot,
    Contains,
    GoalNotcontain,
    Starts,
    Ends,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SegmentTypeItemGoalActivityOp {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Is => serializer.serialize_str("is"),
            Self::GoalNot => serializer.serialize_str("goal_not"),
            Self::Contains => serializer.serialize_str("contains"),
            Self::GoalNotcontain => serializer.serialize_str("goal_notcontain"),
            Self::Starts => serializer.serialize_str("starts"),
            Self::Ends => serializer.serialize_str("ends"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SegmentTypeItemGoalActivityOp {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "is" => Ok(Self::Is),
            "goal_not" => Ok(Self::GoalNot),
            "contains" => Ok(Self::Contains),
            "goal_notcontain" => Ok(Self::GoalNotcontain),
            "starts" => Ok(Self::Starts),
            "ends" => Ok(Self::Ends),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SegmentTypeItemGoalActivityOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Is => write!(f, "is"),
            Self::GoalNot => write!(f, "goal_not"),
            Self::Contains => write!(f, "contains"),
            Self::GoalNotcontain => write!(f, "goal_notcontain"),
            Self::Starts => write!(f, "starts"),
            Self::Ends => write!(f, "ends"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
