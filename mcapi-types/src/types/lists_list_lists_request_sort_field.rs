pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ListListsRequestSortField {
    #[serde(rename = "date_created")]
    DateCreated,
}
impl fmt::Display for ListListsRequestSortField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::DateCreated => "date_created",
        };
        write!(f, "{}", s)
    }
}
