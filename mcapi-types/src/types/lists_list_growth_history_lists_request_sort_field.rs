pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ListGrowthHistoryListsRequestSortField {
    #[serde(rename = "month")]
    Month,
}
impl fmt::Display for ListGrowthHistoryListsRequestSortField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Month => "month",
        };
        write!(f, "{}", s)
    }
}
