pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ListsAbuseReportsMergeFieldsValue {
        ListsAbuseReportsMergeFieldsValueAddr1(ListsAbuseReportsMergeFieldsValueAddr1),

        String(String),

        Double(f64),
}

impl ListsAbuseReportsMergeFieldsValue {
    pub fn is_lists_abuse_reports_merge_fields_value_addr1(&self) -> bool {
        matches!(self, Self::ListsAbuseReportsMergeFieldsValueAddr1(_))
    }

    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }


    pub fn as_lists_abuse_reports_merge_fields_value_addr1(&self) -> Option<&ListsAbuseReportsMergeFieldsValueAddr1> {
        match self {
                    Self::ListsAbuseReportsMergeFieldsValueAddr1(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_lists_abuse_reports_merge_fields_value_addr1(self) -> Option<ListsAbuseReportsMergeFieldsValueAddr1> {
        match self {
                    Self::ListsAbuseReportsMergeFieldsValueAddr1(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_string(&self) -> Option<&str> {
        match self {
                    Self::String(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_string(self) -> Option<String> {
        match self {
                    Self::String(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_double(&self) -> Option<&f64> {
        match self {
                    Self::Double(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_double(self) -> Option<f64> {
        match self {
                    Self::Double(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for ListsAbuseReportsMergeFieldsValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ListsAbuseReportsMergeFieldsValueAddr1(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::String(value) => write!(f, "{}", value),
            Self::Double(value) => write!(f, "{}", value),
        }
    }
}
