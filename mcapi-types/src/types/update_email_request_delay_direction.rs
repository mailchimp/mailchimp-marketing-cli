pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Whether the delay settings describe before or after the delay action of an automation email.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum UpdateEmailRequestDelayDirection {
    #[serde(rename = "after")]
    After,
}
impl fmt::Display for UpdateEmailRequestDelayDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::After => "after",
        };
        write!(f, "{}", s)
    }
}
