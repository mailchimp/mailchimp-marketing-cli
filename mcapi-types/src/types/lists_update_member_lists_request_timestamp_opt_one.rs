pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum UpdateMemberListsRequestTimestampOptOne {
    #[serde(rename = "")]
    Empty,
}
impl fmt::Display for UpdateMemberListsRequestTimestampOptOne {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Empty => "",
        };
        write!(f, "{}", s)
    }
}
