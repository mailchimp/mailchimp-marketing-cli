pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The Mailchimp tracking code for the order. Uses the 'mc_tc' parameter in E-Commerce tracking URLs.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ECommerceOrderTrackingCode {
    #[serde(rename = "prec")]
    Prec,
}
impl fmt::Display for ECommerceOrderTrackingCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Prec => "prec",
        };
        write!(f, "{}", s)
    }
}
