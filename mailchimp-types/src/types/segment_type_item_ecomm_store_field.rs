pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Segment by purchases from a specific store.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SegmentTypeItemEcommStoreField {
    #[serde(rename = "ecomm_store")]
    EcommStore,
}
impl fmt::Display for SegmentTypeItemEcommStoreField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::EcommStore => "ecomm_store",
        };
        write!(f, "{}", s)
    }
}
