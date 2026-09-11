pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Segment by whether someone has purchased anything.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SegmentTypeItemEcommPurchasedField {
    #[serde(rename = "ecomm_purchased")]
    EcommPurchased,
}
impl fmt::Display for SegmentTypeItemEcommPurchasedField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::EcommPurchased => "ecomm_purchased",
        };
        write!(f, "{}", s)
    }
}
