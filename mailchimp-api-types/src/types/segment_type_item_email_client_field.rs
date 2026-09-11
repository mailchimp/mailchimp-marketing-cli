pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Segment by use of a particular email client.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SegmentTypeItemEmailClientField {
    #[serde(rename = "email_client")]
    EmailClient,
}
impl fmt::Display for SegmentTypeItemEmailClientField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::EmailClient => "email_client",
        };
        write!(f, "{}", s)
    }
}
