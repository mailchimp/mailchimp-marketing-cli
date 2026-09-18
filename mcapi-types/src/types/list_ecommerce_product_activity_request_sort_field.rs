pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListEcommerceProductActivityRequestSortField {
    Title,
    TotalRevenue,
    TotalPurchased,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListEcommerceProductActivityRequestSortField {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Title => serializer.serialize_str("title"),
            Self::TotalRevenue => serializer.serialize_str("total_revenue"),
            Self::TotalPurchased => serializer.serialize_str("total_purchased"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListEcommerceProductActivityRequestSortField {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "title" => Ok(Self::Title),
            "total_revenue" => Ok(Self::TotalRevenue),
            "total_purchased" => Ok(Self::TotalPurchased),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListEcommerceProductActivityRequestSortField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Title => write!(f, "title"),
            Self::TotalRevenue => write!(f, "total_revenue"),
            Self::TotalPurchased => write!(f, "total_purchased"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
