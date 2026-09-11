pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ListOpenDetailsReportsRequestSortField {
    #[serde(rename = "opens_count")]
    OpensCount,
}
impl fmt::Display for ListOpenDetailsReportsRequestSortField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::OpensCount => "opens_count",
        };
        write!(f, "{}", s)
    }
}
